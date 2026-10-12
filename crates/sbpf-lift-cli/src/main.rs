//! CLI лифтера: `sbpf-lift {cfg | lift | ir} <file.so> ...`.

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Parser, Subcommand};
use sbpf_lift::{Cfg, Function, Program, ir, lift, solana_sbpf::ebpf, write_dot, write_ir};

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
    /// IR программы в JSON (формат — docs/ir.md).
    Lift {
        file: PathBuf,
        /// С отступами.
        #[arg(long)]
        pretty: bool,
        /// Файл для вывода (по умолчанию stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// IR программы в текстовом виде.
    Ir {
        file: PathBuf,
        /// Только одна функция: имя или pc входа.
        #[arg(short, long)]
        function: Option<String>,
        /// Файл для вывода (по умолчанию stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Cfg {
            file,
            dot,
            function,
            output,
        } => {
            let (_, cfg) = load(&file)?;
            let functions = select(&cfg, function.as_deref())?;
            write_output(output.as_deref(), |w| {
                if dot {
                    write_dot(functions, w)
                } else {
                    print_cfg(&cfg, &functions, w)
                }
            })
        }
        Command::Lift {
            file,
            pretty,
            output,
        } => {
            let (program, cfg) = load(&file)?;
            let module = lift_module(&file, &program, &cfg)?;
            write_output(output.as_deref(), |w| {
                if pretty {
                    serde_json::to_writer_pretty(&mut *w, &module)?;
                } else {
                    serde_json::to_writer(&mut *w, &module)?;
                }
                writeln!(w)
            })
        }
        Command::Ir {
            file,
            function,
            output,
        } => {
            let (program, cfg) = load(&file)?;
            let module = lift_module(&file, &program, &cfg)?;
            let entries: Vec<usize> = select(&cfg, function.as_deref())?
                .iter()
                .map(|f| f.entry)
                .collect();
            let functions = module
                .functions
                .iter()
                .filter(|f| entries.binary_search(&f.entry).is_ok());
            write_output(output.as_deref(), |w| write_ir(&module, functions, w))
        }
    }
}

fn load(file: &Path) -> Result<(Program, Cfg), String> {
    let program = Program::load_file(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let cfg = Cfg::build(&program);
    for (pc, hash) in cfg.unknown_syscalls() {
        eprintln!("предупреждение: pc {pc}: неизвестный syscall {hash:#010x}");
    }
    Ok((program, cfg))
}

fn lift_module(file: &Path, program: &Program, cfg: &Cfg) -> Result<ir::Module, String> {
    lift(program, cfg).map_err(|e| format!("{}: {e}", file.display()))
}

fn select<'a>(cfg: &'a Cfg, function: Option<&str>) -> Result<Vec<&'a Function>, String> {
    match function {
        None => Ok(cfg.functions.iter().collect()),
        Some(query) => cfg
            .find_function(query)
            .map(|f| vec![f])
            .map_err(|e| e.to_string()),
    }
}

fn write_output(
    output: Option<&Path>,
    write: impl FnOnce(&mut Box<dyn Write>) -> io::Result<()>,
) -> Result<(), String> {
    let mut out: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(
            File::create(path).map_err(|e| format!("{}: {e}", path.display()))?,
        )),
        None => Box::new(BufWriter::new(io::stdout().lock())),
    };
    match write(&mut out).and_then(|()| out.flush()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn print_cfg(cfg: &Cfg, functions: &[&Function], w: &mut impl Write) -> io::Result<()> {
    let entry = cfg
        .function_at(cfg.entrypoint)
        .map_or("?", |f| f.name.as_str());
    writeln!(
        w,
        "sBPF {:?}, всего в программе функций: {}, блоков: {}, точка входа: {entry} (pc {})",
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
