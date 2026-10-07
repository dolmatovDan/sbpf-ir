//! Инварианты CFG на всех тестовых контрактах.

mod common;

use std::{collections::BTreeSet, path::Path};

use sbpf_lift::{CallTarget, Cfg, Program};

fn build(path: &Path) -> Cfg {
    let program = Program::load_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    Cfg::build(&program)
}

#[test]
fn cfg_invariants() {
    for path in common::all_programs() {
        let name = path.display();
        let cfg = build(&path);
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
                next_pc = block.instructions.last().unwrap().insn.ptr
                    + if block.instructions.last().unwrap().insn.opc
                        == sbpf_lift::solana_sbpf::ebpf::LD_DW_IMM
                    {
                        2
                    } else {
                        1
                    };
            }
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
