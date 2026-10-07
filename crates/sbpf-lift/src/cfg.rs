//! CFG программы: функции, базовые блоки, переходы и вызовы.
//!
//! Блоки режем сами: `static_analysis::Analysis` в v3 не видит функций (registry
//! программы там заполняется только из отладочных символов) и теряет ребро
//! проваливания у первого блока функции.
//!
//! Входы функций: точка входа, символы из registry и цели `call`. Функция без
//! символа, вызываемая только через `callx`, окажется внутри предыдущей.

use std::collections::{BTreeMap, BTreeSet};

use solana_sbpf::{
    disassembler::disassemble_instruction,
    ebpf::{self, Insn},
    program::SBPFVersion,
    static_analysis::CfgNode,
};

use crate::{Program, syscall_name};

#[derive(Debug)]
pub struct Cfg {
    pub version: SBPFVersion,
    pub entrypoint: usize,
    /// В порядке адресов; блоки функции идут подряд от её входа до входа следующей.
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub entry: usize,
    /// Имя символа (demangled) или `fn_<pc>`.
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub start: usize,
    pub instructions: Vec<Instruction>,
    /// Переходы внутри функции; у `call` — возврат в следующий блок.
    /// Пусто после `exit`, невозвращающегося или невалидного вызова.
    pub successors: Vec<usize>,
}

#[derive(Debug)]
pub struct Instruction {
    pub insn: Insn,
    /// Текст дизассемблера с именами syscall'ов и функций.
    pub text: String,
    pub call: Option<CallTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallTarget {
    Internal {
        pc: usize,
    },
    /// `name == None` — хеша нет в [`crate::SYSCALLS`]. В v0 сюда же попадает `call`,
    /// не найденный в registry программы: отличить его от нового syscall'а нельзя,
    /// в рантайме оба дают ошибку исполнения, если syscall неизвестен валидатору.
    Syscall {
        hash: u32,
        name: Option<&'static str>,
    },
    Indirect {
        reg: u8,
    },
    /// Цель не начало инструкции, либо в v3 `src` не 0 и не 1. Интерпретатор
    /// завершает программу ошибкой.
    Invalid,
}

impl Cfg {
    pub fn build(program: &Program) -> Self {
        let executable = program.executable();
        let version = program.version();
        let registry = executable.get_function_registry();
        let entrypoint = executable.get_entrypoint_instruction_offset();

        let insns = decode(executable.get_text_bytes().1);
        let starts: BTreeSet<usize> = insns.iter().map(|insn| insn.ptr).collect();
        let calls: Vec<_> = insns
            .iter()
            .map(|insn| {
                call_target(version, insn, |key| {
                    registry.lookup_by_key(key).map(|(_, pc)| pc)
                })
                .map(|call| match call {
                    CallTarget::Internal { pc } if !starts.contains(&pc) => CallTarget::Invalid,
                    call => call,
                })
            })
            .collect();

        let mut names: BTreeMap<usize, String> = BTreeMap::new();
        for (_, (name, pc)) in registry.iter() {
            // `function_<pc>` загрузчик придумывает сам для целей call в v0.
            if !name.is_empty() && name != format!("function_{pc}").as_bytes() {
                names.insert(
                    pc,
                    rustc_demangle::demangle(&String::from_utf8_lossy(name)).to_string(),
                );
            }
        }
        let mut entries: BTreeSet<usize> = names.keys().copied().collect();
        entries.insert(entrypoint);
        entries.extend(calls.iter().filter_map(|call| match call {
            Some(CallTarget::Internal { pc }) => Some(*pc),
            _ => None,
        }));
        names
            .entry(entrypoint)
            .or_insert_with(|| "entrypoint".into());

        let mut leaders = entries.clone();
        if let Some(first) = insns.first() {
            leaders.insert(first.ptr);
        }
        for (i, insn) in insns.iter().enumerate() {
            if let Some(target) = jump_target(version, insn) {
                leaders.insert(target);
            }
            if ends_block(version, insn)
                && let Some(next) = insns.get(i + 1)
            {
                leaders.insert(next.ptr);
            }
        }

        let labels: BTreeMap<usize, CfgNode> = leaders
            .iter()
            .map(|&pc| {
                let label = match names.get(&pc) {
                    Some(name) => name.clone(),
                    None if entries.contains(&pc) => format!("fn_{pc}"),
                    None => format!("lbb_{pc}"),
                };
                (
                    pc,
                    CfgNode {
                        label,
                        ..CfgNode::default()
                    },
                )
            })
            .collect();

        let mut functions: Vec<Function> = Vec::new();
        let mut i = 0;
        while i < insns.len() {
            let start = insns[i].ptr;
            let mut block = Vec::new();
            loop {
                let insn = &insns[i];
                block.push(Instruction {
                    insn: insn.clone(),
                    text: match &calls[i] {
                        Some(CallTarget::Internal { pc }) => format!("call {}", labels[pc].label),
                        _ => disassemble_instruction(
                            insn,
                            insn.ptr,
                            &labels,
                            registry,
                            executable.get_loader(),
                            version,
                        ),
                    },
                    call: calls[i].clone(),
                });
                i += 1;
                if i == insns.len() || leaders.contains(&insns[i].ptr) {
                    break;
                }
            }
            let last = block.last().unwrap();
            let next = insns.get(i).map(|insn| insn.ptr);
            let successors = successors(version, last, next, &entries);

            if entries.contains(&start) || functions.is_empty() {
                functions.push(Function {
                    entry: start,
                    name: labels[&start].label.clone(),
                    blocks: Vec::new(),
                });
            }
            functions.last_mut().unwrap().blocks.push(Block {
                start,
                instructions: block,
                successors,
            });
        }
        cut_noreturn_edges(&mut functions);

        Self {
            version,
            entrypoint,
            functions,
        }
    }

    pub fn function_at(&self, pc: usize) -> Option<&Function> {
        self.functions
            .binary_search_by_key(&pc, |f| f.entry)
            .ok()
            .map(|i| &self.functions[i])
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.functions.iter().flat_map(|f| &f.blocks)
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.blocks().flat_map(|b| &b.instructions)
    }

    /// (pc, хеш) вызовов syscall'ов с неизвестным именем.
    pub fn unknown_syscalls(&self) -> Vec<(usize, u32)> {
        self.instructions()
            .filter_map(|i| match i.call {
                Some(CallTarget::Syscall { hash, name: None }) => Some((i.insn.ptr, hash)),
                _ => None,
            })
            .collect()
    }
}

/// `lddw` занимает два слота и декодируется в одну инструкцию с 64-битным `imm`.
fn decode(text: &[u8]) -> Vec<Insn> {
    let slots = text.len() / ebpf::INSN_SIZE;
    let mut insns = Vec::with_capacity(slots);
    let mut pc = 0;
    while pc < slots {
        let mut insn = ebpf::get_insn_unchecked(text, pc);
        if insn.opc == ebpf::LD_DW_IMM {
            ebpf::augment_lddw_unchecked(text, &mut insn);
            pc += 1;
        }
        insns.push(insn);
        pc += 1;
    }
    insns
}

fn is_jump(version: SBPFVersion, insn: &Insn) -> bool {
    match insn.opc & ebpf::BPF_CLS_MASK {
        ebpf::BPF_JMP64 => !matches!(insn.opc, ebpf::CALL_IMM | ebpf::CALL_REG | ebpf::EXIT),
        ebpf::BPF_JMP32 => version.enable_jmp32(),
        _ => false,
    }
}

fn jump_target(version: SBPFVersion, insn: &Insn) -> Option<usize> {
    is_jump(version, insn).then(|| (insn.ptr as isize + insn.off as isize + 1) as usize)
}

fn ends_block(version: SBPFVersion, insn: &Insn) -> bool {
    is_jump(version, insn) || matches!(insn.opc, ebpf::EXIT | ebpf::CALL_IMM | ebpf::CALL_REG)
}

const NORETURN_SYSCALLS: [&str; 2] = ["abort", "sol_panic_"];

/// Убирает возврат после вызова функции без `exit` и после `abort`/`sol_panic_`.
fn cut_noreturn_edges(functions: &mut [Function]) {
    let noreturn: BTreeSet<usize> = functions
        .iter()
        .filter(|f| {
            f.blocks
                .iter()
                .flat_map(|b| &b.instructions)
                .all(|i| i.insn.opc != ebpf::EXIT)
        })
        .map(|f| f.entry)
        .collect();
    for block in functions.iter_mut().flat_map(|f| &mut f.blocks) {
        let cut = match &block.instructions.last().unwrap().call {
            Some(CallTarget::Internal { pc }) => noreturn.contains(pc),
            Some(CallTarget::Syscall {
                name: Some(name), ..
            }) => NORETURN_SYSCALLS.contains(name),
            _ => false,
        };
        if cut {
            block.successors.clear();
        }
    }
}

fn successors(
    version: SBPFVersion,
    last: &Instruction,
    next: Option<usize>,
    entries: &BTreeSet<usize>,
) -> Vec<usize> {
    if last.call == Some(CallTarget::Invalid) {
        return vec![];
    }
    let last = &last.insn;
    match last.opc {
        ebpf::EXIT => vec![],
        ebpf::JA => vec![jump_target(version, last).unwrap()],
        // После невозвращающегося вызова (panic) сразу может начинаться другая функция.
        ebpf::CALL_IMM | ebpf::CALL_REG => next
            .filter(|pc| !entries.contains(pc))
            .into_iter()
            .collect(),
        _ => {
            let mut succ: Vec<usize> = next.into_iter().collect();
            if let Some(target) = jump_target(version, last)
                && !succ.contains(&target)
            {
                succ.push(target);
            }
            succ
        }
    }
}

/// Повторяет разрешение `CALL_IMM` в интерпретаторе `solana-sbpf`: в v0 сначала
/// syscall по `imm`, затем функция из registry; в v3 `src == 0` — syscall,
/// `src == 1` — функция по относительному смещению.
fn call_target(
    version: SBPFVersion,
    insn: &Insn,
    lookup_function: impl Fn(u32) -> Option<usize>,
) -> Option<CallTarget> {
    match insn.opc {
        ebpf::CALL_IMM => {
            let hash = insn.imm as u32;
            let syscall = |name| CallTarget::Syscall { hash, name };
            if version.static_syscalls() {
                Some(match insn.src {
                    0 => syscall(syscall_name(hash)),
                    1 => CallTarget::Internal {
                        pc: version.calculate_call_imm_target_pc(insn.ptr, insn.imm) as usize,
                    },
                    _ => CallTarget::Invalid,
                })
            } else if let Some(name) = syscall_name(hash) {
                Some(syscall(Some(name)))
            } else {
                Some(match lookup_function(hash) {
                    Some(pc) => CallTarget::Internal { pc },
                    None => syscall(None),
                })
            }
        }
        ebpf::CALL_REG => Some(CallTarget::Indirect {
            reg: if version.callx_uses_dst_reg() {
                insn.dst
            } else {
                insn.imm as u8
            },
        }),
        _ => None,
    }
}
