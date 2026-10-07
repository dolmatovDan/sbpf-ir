//! Поддерживаются только sBPF v0 и v3.

mod common;

use sbpf_lift::{LoadError, Program, solana_sbpf::program::SBPFVersion};

const E_FLAGS_OFFSET: usize = 48;

fn with_e_flags(file: &str, e_flags: u32) -> Vec<u8> {
    let mut bytes = std::fs::read(common::programs_dir().join("bin").join(file)).unwrap();
    bytes[E_FLAGS_OFFSET..E_FLAGS_OFFSET + 4].copy_from_slice(&e_flags.to_le_bytes());
    bytes
}

#[test]
fn other_versions_are_rejected() {
    for (e_flags, version) in [
        (1, SBPFVersion::V1),
        (2, SBPFVersion::V2),
        (4, SBPFVersion::V4),
        (5, SBPFVersion::Reserved),
    ] {
        match Program::load(&with_e_flags("native-basic.v3.so", e_flags)) {
            Err(LoadError::UnsupportedVersion(v)) => assert_eq!(v, version),
            Err(e) => panic!("e_flags={e_flags}: неожиданная ошибка {e}"),
            Ok(_) => panic!("e_flags={e_flags}: программа загрузилась"),
        }
    }
}
