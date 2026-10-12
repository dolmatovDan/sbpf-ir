//! Инварианты IR на всех тестовых контрактах.

mod common;

use std::collections::BTreeMap;

use sbpf_lift::{
    Cfg, Program,
    ir::{Exit, Fault, Module, Stmt},
    lift,
};

fn lift_file(path: &std::path::Path) -> (Cfg, Module) {
    let program = Program::load_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let cfg = Cfg::build(&program);
    let module = lift(&program, &cfg).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    (cfg, module)
}

fn exit_pc(exit: &Exit) -> Option<usize> {
    match *exit {
        Exit::Next { pc, .. } => pc,
        Exit::Branch { pc, .. }
        | Exit::Call { pc, .. }
        | Exit::Callx { pc, .. }
        | Exit::Syscall { pc, .. }
        | Exit::Return { pc }
        | Exit::Trap { pc, .. } => Some(pc),
    }
}

#[test]
fn invariants() {
    for path in common::all_programs() {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let (cfg, module) = lift_file(&path);
        module.validate().unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(module.entrypoint, cfg.entrypoint, "{name}");
        assert_eq!(module.address_taken, cfg.address_taken, "{name}");
        let text_len = program_text_len(&path);

        // Одна инструкция — ровно один statement (кроме check) или выход с pc.
        let mut seen: BTreeMap<usize, usize> = BTreeMap::new();
        for block in module.functions.iter().flat_map(|f| &f.blocks) {
            for stmt in &block.statements {
                let pc = match *stmt {
                    Stmt::Check { .. } => continue,
                    Stmt::Assign { pc, .. } | Stmt::Load { pc, .. } | Stmt::Store { pc, .. } => pc,
                };
                *seen.entry(pc).or_default() += 1;
            }
            match block.exit {
                Exit::Trap {
                    pc,
                    error: Fault::ExecutionOverrun,
                } => assert_eq!(pc * 8, text_len, "{name}"),
                ref exit => {
                    if let Some(pc) = exit_pc(exit) {
                        *seen.entry(pc).or_default() += 1;
                    }
                }
            }
        }
        let expected: BTreeMap<usize, usize> =
            cfg.instructions().map(|i| (i.insn.ptr, 1)).collect();
        assert_eq!(seen, expected, "{name}");

        // Блоки и noreturn — те же, что в CFG.
        let blocks: Vec<_> = cfg.blocks().map(|b| (b.start, b.noreturn)).collect();
        let lifted: Vec<_> = module
            .functions
            .iter()
            .flat_map(|f| &f.blocks)
            .filter(|b| b.start * 8 != text_len)
            .map(|b| (b.start, b.noreturn))
            .collect();
        assert_eq!(lifted, blocks, "{name}");

        let json = serde_json::to_string(&module).unwrap();
        let back: Module = serde_json::from_str(&json).unwrap();
        assert_eq!(back, module, "{name}");
    }
}

fn program_text_len(path: &std::path::Path) -> usize {
    let program = Program::load_file(path).unwrap();
    program.executable().get_text_bytes().1.len()
}

#[test]
fn header() {
    let (_, v0) = lift_file(&common::programs_dir().join("bin/native-basic.v0.so"));
    let (_, v3) = lift_file(&common::programs_dir().join("bin/native-basic.v3.so"));
    assert_eq!((v0.format_version, v0.sbpf_version), (1, 0));
    assert_eq!((v3.format_version, v3.sbpf_version), (1, 3));
    // В v0 код входит в read-only регион, в v3 нет.
    let contains_text = |m: &Module| {
        (m.ro_region.vaddr..m.ro_region.vaddr + m.ro_region.data.len() as u64)
            .contains(&m.text_vaddr)
    };
    assert!(contains_text(&v0));
    assert!(!contains_text(&v3));
}
