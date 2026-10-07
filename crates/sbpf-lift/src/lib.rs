//! Лифтер sBPF-байткода Solana в IR.

mod load;
mod syscalls;

pub use load::{LoadError, Program};
pub use solana_sbpf;
pub use syscalls::{NoExec, SYSCALLS, syscall_name};
