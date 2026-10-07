//! Лифтер sBPF-байткода Solana в IR.

mod cfg;
mod dot;
mod load;
mod syscalls;

pub use cfg::{Block, CallTarget, Cfg, Function, Instruction};
pub use dot::write_dot;
pub use load::{LoadError, Program};
pub use solana_sbpf;
pub use syscalls::{NoExec, SYSCALLS, syscall_name};
