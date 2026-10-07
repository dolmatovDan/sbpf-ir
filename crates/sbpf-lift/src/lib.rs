//! Лифтер sBPF-байткода Solana в IR.

mod cfg;
mod load;
mod syscalls;

pub use cfg::{Block, CallTarget, Cfg, Function, Instruction};
pub use load::{LoadError, Program};
pub use solana_sbpf;
pub use syscalls::{NoExec, SYSCALLS, syscall_name};
