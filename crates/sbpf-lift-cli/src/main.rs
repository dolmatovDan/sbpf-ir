//! CLI лифтера: `sbpf-lift cfg <file.so> ...`, `sbpf-lift run <example.json>`.

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
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
    /// Исполнить контракт на примере через Mollusk (формат — docs/runner.md).
    Run {
        example: PathBuf,
        /// Добавить финальные регистры r0..r10 и pc.
        #[arg(long)]
        registers: bool,
        /// Добавить трассу регистров по каждой инструкции.
        #[arg(long)]
        trace: bool,
        /// Файл для вывода (по умолчанию stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    if std::env::var_os("RUST_LOG").is_none() {
        // Mollusk включает debug-логи рантайма; логи программы и так есть в выводе run.
        // SAFETY: других потоков ещё нет.
        unsafe { std::env::set_var("RUST_LOG", "error") };
    }
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
        Command::Run {
            example,
            registers,
            trace,
            output,
        } => {
            let options = sbpf_run::Options { registers, trace };
            let outcome = sbpf_run::run_file(&example, options).map_err(|e| e.to_string())?;
            write_output(output.as_deref(), |w| {
                serde_json::to_writer_pretty(&mut *w, &outcome)?;
                writeln!(w)
            })
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
