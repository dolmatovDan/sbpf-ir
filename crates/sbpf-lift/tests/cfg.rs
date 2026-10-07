//! Инварианты CFG на всех тестовых контрактах.

mod common;

use std::{collections::BTreeSet, path::Path};

use sbpf_lift::{
    Block, CallTarget, Cfg, NORETURN_SYSCALLS, Program,
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

/// Необходимые условия для `noreturn`, независимые от неподвижной точки.
fn check_noreturn(cfg: &Cfg, block: &Block, ctx: &str) {
    match &block.instructions.last().unwrap().call {
        Some(CallTarget::Syscall {
            name: Some(name), ..
        }) => {
            assert_eq!(block.noreturn, NORETURN_SYSCALLS.contains(name), "{ctx}")
        }
        Some(CallTarget::Internal { pc }) => {
            let callee = cfg.function_at(*pc).unwrap();
            if !has_exit(callee) {
                assert!(block.noreturn, "{ctx}: {} без exit", callee.name);
            }
            if !block.noreturn {
                assert!(has_exit(callee), "{ctx}");
            }
        }
        Some(CallTarget::Invalid) => assert!(block.noreturn, "{ctx}"),
        _ => assert!(!block.noreturn, "{ctx}"),
    }
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
            let returns = last.call != Some(CallTarget::Invalid)
                && next.is_some_and(|pc| cfg.function_at(pc).is_none());
            let expected: Vec<_> = next.filter(|_| returns).into_iter().collect();
            assert_eq!(succ, &expected, "{ctx}");
            check_noreturn(cfg, block, &ctx);
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
        let calls = edges.iter().filter(|l| l.contains("color=gray")).count();
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
/// и невозвращающийся посреди функции (ребро возврата при этом сохранено).
#[test]
fn call_kinds() {
    let cfg = build(&common::programs_dir().join("bin/anchor-basic.v0.so"));
    let (mut returning, mut before_function, mut mid_function) = (0, 0, 0);
    for block in cfg.blocks() {
        if !matches!(
            block.instructions.last().unwrap().call,
            Some(CallTarget::Internal { .. })
        ) {
            continue;
        }
        match (block.noreturn, block.successors.is_empty()) {
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

/// Сверено по дизассемблеру: `custom_panic` в этих сборках кончается `sol_panic_`.
#[test]
fn custom_panic_is_noreturn() {
    for file in ["anchor-basic.v0.so", "native-cpi.v0.so"] {
        let cfg = build(&common::programs_dir().join("bin").join(file));
        let entry = cfg
            .functions
            .iter()
            .find(|f| f.name == "custom_panic")
            .unwrap()
            .entry;
        let calls: Vec<_> = cfg
            .blocks()
            .filter(|b| {
                b.instructions.last().unwrap().call == Some(CallTarget::Internal { pc: entry })
            })
            .collect();
        assert!(!calls.is_empty(), "{file}");
        assert!(calls.iter().all(|b| b.noreturn), "{file}");
    }
}

/// Переход в другую функцию (хвостовой переход, .cold-часть) не роняет построение CFG.
#[test]
fn jump_into_other_function() {
    let path = common::programs_dir().join("bin/native-basic.v3.so");
    let mut bytes = std::fs::read(&path).unwrap();
    let program = Program::load(&bytes).unwrap();
    let cfg = Cfg::build(&program);
    let text = program.executable().get_text_bytes().1;
    let unique_pos = |ptr: usize| {
        let insn = &text[ptr * ebpf::INSN_SIZE..(ptr + 1) * ebpf::INSN_SIZE];
        let mut found = (0..=bytes.len() - 8).filter(|&p| &bytes[p..p + 8] == insn);
        let pos = found.next()?;
        found.next().is_none().then_some(pos)
    };
    let (ptr, entry, pos) =
        cfg.instructions()
            .filter(|i| i.insn.opc == ebpf::JA)
            .find_map(|i| {
                let ptr = i.insn.ptr;
                let own = cfg.functions.iter().rfind(|f| f.entry <= ptr)?.entry;
                let entry = cfg.functions.iter().map(|f| f.entry).find(|&e| {
                    e != own && (e as isize - ptr as isize - 1).abs() < i16::MAX as isize
                })?;
                Some((ptr, entry, unique_pos(ptr)?))
            })
            .expect("нет подходящего ja");
    let off = (entry as isize - ptr as isize - 1) as i16;
    bytes[pos + 2..pos + 4].copy_from_slice(&off.to_le_bytes());

    let cfg = Cfg::build(&Program::load(&bytes).unwrap());
    let block = cfg
        .blocks()
        .find(|b| b.instructions.last().unwrap().insn.ptr == ptr)
        .unwrap();
    assert_eq!(block.successors, [entry]);
}
