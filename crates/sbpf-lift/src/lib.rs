//! Лифтер sBPF-байткода Solana в IR.

mod cfg;
mod dot;
pub mod ir;
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "lift() появится при сборке модуля")
)]
mod lift;
mod load;
mod syscalls;

pub use cfg::{
    Block, CallTarget, Cfg, FindFunctionError, Function, Instruction, NORETURN_SYSCALLS,
};
pub use dot::write_dot;
pub use lift::LiftError;
pub use load::{LoadError, Program};
pub use solana_sbpf;
pub use syscalls::{NoExec, SYSCALLS, syscall_name};
