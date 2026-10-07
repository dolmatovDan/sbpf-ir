//! Syscall'ы рантайма Solana и загрузчик, в котором они зарегистрированы.
//!
//! В байткоде syscall — это `call`, у которого в `imm` лежит murmur3-хеш имени
//! (`ebpf::hash_symbol_name`). Имя по хешу восстанавливается только через реестр
//! загрузчика, поэтому все известные имена регистрируются в нём с функцией-заглушкой.

use std::{
    collections::HashMap,
    ptr::NonNull,
    sync::{Arc, LazyLock},
};

use solana_sbpf::{
    ebpf,
    memory_region::MemoryMapping,
    program::{BuiltinProgram, JitCompiler, SBPFVersion},
    vm::{Config, ContextObject, EncryptedHostAddressToEbpfVm},
};

/// Имена syscall'ов, которые регистрирует валидатор.
///
/// Источник: agave v4.3.0, `syscalls/src/lib.rs`, `create_program_runtime_environment`,
/// включая syscall'ы за feature gate'ами.
pub const SYSCALLS: &[&str] = &[
    "abort",
    "sol_panic_",
    "sol_log_",
    "sol_log_64_",
    "sol_log_pubkey",
    "sol_log_compute_units_",
    "sol_create_program_address",
    "sol_try_find_program_address",
    "sol_sha256",
    "sol_keccak256",
    "sol_secp256k1_recover",
    "sol_blake3",
    "sol_sha512",
    "sol_curve_validate_point",
    "sol_curve_group_op",
    "sol_curve_multiscalar_mul",
    "sol_curve_decompress",
    "sol_curve_pairing_map",
    "sol_get_clock_sysvar",
    "sol_get_epoch_schedule_sysvar",
    "sol_get_fees_sysvar",
    "sol_get_rent_sysvar",
    "sol_get_last_restart_slot",
    "sol_get_epoch_rewards_sysvar",
    "sol_memcpy_",
    "sol_memmove_",
    "sol_memset_",
    "sol_memcmp_",
    "sol_get_processed_sibling_instruction",
    "sol_get_stack_height",
    "sol_set_return_data",
    "sol_get_return_data",
    "sol_invoke_signed_c",
    "sol_invoke_signed_rust",
    "sol_alloc_free_",
    "sol_alt_bn128_group_op",
    "sol_big_mod_exp",
    "sol_poseidon",
    "sol_remaining_compute_units",
    "sol_alt_bn128_compression",
    "sol_get_sysvar",
    "sol_get_epoch_stake",
    "sol_log_data",
];

/// Имя syscall'а по хешу из `imm` инструкции `call`.
pub fn syscall_name(hash: u32) -> Option<&'static str> {
    static BY_HASH: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
        SYSCALLS
            .iter()
            .map(|&name| (ebpf::hash_symbol_name(name.as_bytes()), name))
            .collect()
    });
    BY_HASH.get(&hash).copied()
}

/// Контекст-заглушка: программа не исполняется, только загружается и анализируется.
pub struct NoExec;

impl ContextObject for NoExec {
    fn consume(&mut self, _amount: u64) {}

    fn get_remaining(&self) -> u64 {
        0
    }

    fn active_mapping_ptr(&mut self) -> NonNull<MemoryMapping> {
        unreachable!("программа не исполняется")
    }
}

fn not_executable(
    _vm: EncryptedHostAddressToEbpfVm<NoExec>,
    _: u64,
    _: u64,
    _: u64,
    _: u64,
    _: u64,
) {
    unreachable!("программа не исполняется")
}

fn no_codegen(_jit: &mut JitCompiler<NoExec>) {}

/// В отличие от валидатора при деплое, `reject_broken_elfs` выключен: v0 с неизвестным
/// syscall'ом загружается. Метки символов включены ради имён функций.
pub(crate) fn loader() -> Arc<BuiltinProgram<NoExec>> {
    let config = Config {
        enabled_sbpf_versions: SBPFVersion::V0..=SBPFVersion::V3,
        enable_symbol_and_section_labels: true,
        ..Config::default()
    };
    let mut loader = BuiltinProgram::new_loader(config);
    for name in SYSCALLS {
        loader
            .register_function(name, (not_executable, no_codegen))
            .unwrap_or_else(|e| panic!("не удалось зарегистрировать {name}: {e:?}"));
    }
    Arc::new(loader)
}
