//! CLI лифтера: `sbpf-lift cfg <file.so> [--function F] [--dot] [-o FILE]`.

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use sbpf_lift::{Cfg, Function, Program, solana_sbpf::ebpf, write_dot};

#[derive(Parser)]
#[command(name = "sbpf-lift", about = "Лифтер sBPF-байткода Solana в IR")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Функции, базовые блоки и переходы программы.
    Cfg {
        file: PathBuf,
        /// Вывести в формате Graphviz DOT.
        #[arg(long)]
        dot: bool,
        /// Только одна функция: имя или pc входа.
        #[arg(short, long)]
        function: Option<String>,
        /// Файл для вывода (по умолчанию stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let Command::Cfg {
        file,
        dot,
        function,
        output,
    } = Cli::parse().command;
    let program = match Program::load_file(&file) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{}: {e}", file.display());
            return ExitCode::FAILURE;
        }
    };
    let cfg = Cfg::build(&program);
    for (pc, hash) in cfg.unknown_syscalls() {
        eprintln!("предупреждение: pc {pc}: неизвестный syscall {hash:#010x}");
    }
    let functions: Vec<&Function> = match &function {
        None => cfg.functions.iter().collect(),
        Some(query) => match find_function(&cfg, query) {
            Ok(f) => vec![f],
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        },
    };

    let mut out: Box<dyn Write> = match output {
        Some(path) => match File::create(&path) {
            Ok(f) => Box::new(BufWriter::new(f)),
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                return ExitCode::FAILURE;
            }
        },
        None => Box::new(BufWriter::new(io::stdout().lock())),
    };
    let result = if dot {
        write_dot(functions, &mut out)
    } else {
        print_cfg(&cfg, &functions, &mut out)
    }
    .and_then(|()| out.flush());
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn find_function<'a>(cfg: &'a Cfg, query: &str) -> Result<&'a Function, String> {
    if let Ok(pc) = query.parse() {
        return cfg
            .function_at(pc)
            .ok_or_else(|| format!("нет функции с входом в pc {pc}"));
    }
    let found: Vec<_> = cfg.functions.iter().filter(|f| f.name == query).collect();
    match found.as_slice() {
        [f] => Ok(f),
        [] => Err(format!("нет функции {query}")),
        _ => Err(format!(
            "несколько функций {query}, укажите pc: {}",
            found
                .iter()
                .map(|f| f.entry.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn print_cfg(cfg: &Cfg, functions: &[&Function], w: &mut impl Write) -> io::Result<()> {
    let entry = cfg
        .function_at(cfg.entrypoint)
        .map_or("?", |f| f.name.as_str());
    writeln!(
        w,
        "sBPF {:?}, функций: {}, блоков: {}, точка входа: {entry} (pc {})",
        cfg.version,
        cfg.functions.len(),
        cfg.blocks().count(),
        cfg.entrypoint
    )?;
    for function in functions {
        writeln!(w, "\nfn {} @ {}", function.name, function.entry)?;
        for block in &function.blocks {
            let last = &block.instructions.last().unwrap().insn;
            let end = last.ptr + usize::from(last.opc == ebpf::LD_DW_IMM);
            let succ: Vec<_> = block
                .successors
                .iter()
                .map(|s| format!("lbb_{s}"))
                .collect();
            writeln!(
                w,
                "  lbb_{} [{}..={end}] -> {}",
                block.start,
                block.start,
                if succ.is_empty() {
                    "—".into()
                } else {
                    succ.join(", ")
                }
            )?;
            for insn in block.instructions.iter().filter(|i| i.call.is_some()) {
                let mark = if block.noreturn { " (noreturn)" } else { "" };
                writeln!(w, "    pc {}: {}{mark}", insn.insn.ptr, insn.text)?;
            }
        }
    }
    Ok(())
}
