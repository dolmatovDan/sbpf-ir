//! Поддерживаются только sBPF v0 и v3: остальные версии отклоняются при загрузке.

mod common;

use sbpf_lift::{LoadError, Program, solana_sbpf::program::SBPFVersion};

/// Смещение поля `e_flags` в заголовке ELF64, где Solana хранит версию sBPF.
const E_FLAGS_OFFSET: usize = 48;

/// Читает тестовый контракт и подменяет версию sBPF в заголовке.
fn with_e_flags(file: &str, e_flags: u32) -> Vec<u8> {
    let mut bytes = std::fs::read(common::programs_dir().join("bin").join(file)).unwrap();
    bytes[E_FLAGS_OFFSET..E_FLAGS_OFFSET + 4].copy_from_slice(&e_flags.to_le_bytes());
    bytes
}

#[test]
fn v1_and_v2_are_rejected() {
    // v0 и v1/v2 разбираются одним и тем же (нестрогим) парсером ELF,
    // поэтому загрузка доходит до проверки версии.
    for (e_flags, version) in [(1, SBPFVersion::V1), (2, SBPFVersion::V2)] {
        match Program::load(&with_e_flags("native-basic.v0.so", e_flags)) {
            Err(LoadError::UnsupportedVersion(Some(v))) => assert_eq!(v, version),
            Err(e) => panic!("e_flags={e_flags}: неожиданная ошибка {e}"),
            Ok(_) => panic!("e_flags={e_flags}: программа загрузилась"),
        }
    }
}

#[test]
fn v4_is_rejected() {
    for e_flags in [4, 5] {
        match Program::load(&with_e_flags("native-basic.v3.so", e_flags)) {
            Err(LoadError::UnsupportedVersion(None)) => {}
            Err(e) => panic!("e_flags={e_flags}: неожиданная ошибка {e}"),
            Ok(_) => panic!("e_flags={e_flags}: программа загрузилась"),
        }
    }
}
