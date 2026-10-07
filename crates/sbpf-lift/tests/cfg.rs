//! Инварианты CFG на всех тестовых контрактах.

mod common;

use std::{collections::BTreeSet, path::Path};

use sbpf_lift::{
    Block, CallTarget, Cfg, Program,
    solana_sbpf::{ebpf, program::SBPFVersion},
};

fn load(path: &Path) -> Program {
    Program::load_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn build(path: &Path) -> Cfg {
    Cfg::build(&load(path))
}

fn slots(insn: &ebpf::Insn) -> usize {
    if insn.opc == ebpf::LD_DW_IMM { 2 } else { 1 }
}

fn is_jump(version: SBPFVersion, opc: u8) -> bool {
    match opc & ebpf::BPF_CLS_MASK {
        ebpf::BPF_JMP64 => !matches!(opc, ebpf::CALL_IMM | ebpf::CALL_REG | ebpf::EXIT),
        ebpf::BPF_JMP32 => version.enable_jmp32(),
        _ => false,
    }
}

fn has_exit(function: &sbpf_lift::Function) -> bool {
    function
        .blocks
        .iter()
        .flat_map(|b| &b.instructions)
        .any(|i| i.insn.opc == ebpf::EXIT)
}

/// Преемники блока согласованы с его последней инструкцией.
fn check_terminator(cfg: &Cfg, block: &Block, next: Option<usize>, name: &str) {
    let last = block.instructions.last().unwrap();
    let insn = &last.insn;
    let target = (insn.ptr as isize + insn.off as isize + 1) as usize;
    let succ = &block.successors;
    let ctx = format!("{name}: lbb_{} `{}` -> {succ:?}", block.start, last.text);
    match insn.opc {
        ebpf::EXIT => assert!(succ.is_empty(), "{ctx}"),
        ebpf::JA => assert_eq!(succ, &[target], "{ctx}"),
        opc if is_jump(cfg.version, opc) => {
            let next = next.unwrap_or_else(|| panic!("{ctx}: условный переход в конце программы"));
            let mut expected = vec![next];
            if target != next {
                expected.push(target);
            }
            assert_eq!(succ, &expected, "{ctx}");
        }
        ebpf::CALL_IMM | ebpf::CALL_REG => {
            let callee_returns = match &last.call {
                Some(CallTarget::Internal { pc }) => has_exit(cfg.function_at(*pc).unwrap()),
                Some(CallTarget::Syscall { name, .. }) => {
                    !matches!(name, Some("abort" | "sol_panic_"))
                }
                Some(CallTarget::Indirect { .. }) => true,
                Some(CallTarget::Invalid) | None => false,
            };
            let returns = callee_returns && next.is_some_and(|pc| cfg.function_at(pc).is_none());
            let expected: Vec<_> = next.filter(|_| returns).into_iter().collect();
            assert_eq!(succ, &expected, "{ctx}");
        }
        _ => assert_eq!(succ, &next.into_iter().collect::<Vec<_>>(), "{ctx}"),
    }
}

#[test]
fn cfg_invariants() {
    for path in common::all_programs() {
        let name = path.display();
        let program = load(&path);
        let cfg = Cfg::build(&program);
        let text_slots = program.executable().get_text_bytes().1.len() / ebpf::INSN_SIZE;
        assert!(!cfg.functions.is_empty(), "{name}: нет функций");

        let mut next_pc = 0;
        for function in &cfg.functions {
            assert!(
                !function.blocks.is_empty(),
                "{name}: пустая функция {}",
                function.name
            );
            assert_eq!(
                function.blocks[0].start, function.entry,
                "{name}: {}",
                function.name
            );
            for block in &function.blocks {
                assert!(
                    !block.instructions.is_empty(),
                    "{name}: пустой блок {}",
                    block.start
                );
                assert_eq!(
                    block.start, next_pc,
                    "{name}: разрыв перед блоком {}",
                    block.start
                );
                assert_eq!(block.instructions[0].insn.ptr, block.start);
                for pair in block.instructions.windows(2) {
                    assert_eq!(pair[0].insn.ptr + slots(&pair[0].insn), pair[1].insn.ptr);
                }
                let last = &block.instructions.last().unwrap().insn;
                next_pc = last.ptr + slots(last);
            }
        }
        assert_eq!(next_pc, text_slots, "{name}: блоки не покрывают .text");

        let blocks: Vec<_> = cfg.blocks().collect();
        for (i, block) in blocks.iter().enumerate() {
            let next = blocks.get(i + 1).map(|b| b.start);
            check_terminator(&cfg, block, next, &name.to_string());
        }

        for function in &cfg.functions {
            let starts: BTreeSet<_> = function.blocks.iter().map(|b| b.start).collect();
            for block in &function.blocks {
                for succ in &block.successors {
                    assert!(
                        starts.contains(succ),
                        "{name}: {} lbb_{} -> lbb_{succ} вне функции",
                        function.name,
                        block.start
                    );
                }
            }
        }

        assert!(
            cfg.function_at(cfg.entrypoint).is_some(),
            "{name}: точка входа"
        );
        for insn in cfg.instructions() {
            if let Some(CallTarget::Internal { pc }) = insn.call {
                assert!(
                    cfg.function_at(pc).is_some(),
                    "{name}: call из pc {} в {pc} — не вход функции",
                    insn.insn.ptr
                );
            }
        }

        assert_eq!(cfg.unknown_syscalls(), [], "{name}: неизвестные syscall'ы");
    }
}

#[test]
fn dot_export() {
    for version in ["v0", "v3"] {
        let cfg = build(&common::programs_dir().join(format!("bin/native-cpi.{version}.so")));
        let mut out = Vec::new();
        sbpf_lift::write_dot(&cfg, &mut out).unwrap();
        let dot = String::from_utf8(out).unwrap();

        assert!(dot.starts_with("digraph cfg {") && dot.trim_end().ends_with('}'));
        assert_eq!(
            dot.matches("subgraph cluster_").count(),
            cfg.functions.len()
        );
        let edges: Vec<_> = dot
            .lines()
            .filter(|l| l.starts_with("  lbb_") && l.contains(" -> lbb_"))
            .collect();
        let calls = edges.iter().filter(|l| l.contains("style=dashed")).count();
        let call_pairs: BTreeSet<_> = cfg
            .blocks()
            .flat_map(|b| {
                b.instructions.iter().filter_map(move |i| match i.call {
                    Some(CallTarget::Internal { pc }) => Some((b.start, pc)),
                    _ => None,
                })
            })
            .collect();
        assert_eq!(calls, call_pairs.len(), "{version}");
        assert_eq!(
            edges.len() - calls,
            cfg.blocks().map(|b| b.successors.len()).sum::<usize>(),
            "{version}"
        );
        assert!(dot.contains("syscall sol_invoke_signed_rust"), "{version}");
    }
}

/// Встречаются все виды `call`: с возвратом, перед входом другой функции
/// и невозвращающийся посреди функции.
#[test]
fn call_successors_kinds() {
    let cfg = build(&common::programs_dir().join("bin/anchor-basic.v0.so"));
    let blocks: Vec<_> = cfg.blocks().collect();
    let (mut returning, mut before_function, mut mid_function) = (0, 0, 0);
    for pair in blocks.windows(2) {
        let last = pair[0].instructions.last().unwrap();
        if !matches!(last.call, Some(CallTarget::Internal { .. })) {
            continue;
        }
        match (
            pair[0].successors.is_empty(),
            cfg.function_at(pair[1].start).is_some(),
        ) {
            (false, _) => returning += 1,
            (true, true) => before_function += 1,
            (true, false) => mid_function += 1,
        }
    }
    assert!(
        returning > 0 && before_function > 0 && mid_function > 0,
        "{returning} {before_function} {mid_function}"
    );
}
