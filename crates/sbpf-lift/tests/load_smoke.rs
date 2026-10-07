//! Смоук-тест: все тестовые контракты загружаются, проходят верификацию
//! и определяются с ожидаемой версией sBPF.

mod common;

use sbpf_lift::{Program, solana_sbpf::program::SBPFVersion};

fn load(path: &std::path::Path) -> Program {
    Program::load_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn own_programs_load_with_expected_version() {
    let mut count = 0;
    for path in common::own_programs() {
        let expected = match path.file_name().unwrap().to_str().unwrap() {
            name if name.ends_with(".v0.so") => SBPFVersion::V0,
            name if name.ends_with(".v3.so") => SBPFVersion::V3,
            _ => continue,
        };
        assert_eq!(load(&path).version(), expected, "{}", path.display());
        count += 1;
    }
    assert_eq!(count, 6, "ожидалось 3 программы × (v0, v3)");
}

#[test]
fn mainnet_programs_load() {
    for name in common::MAINNET {
        assert_eq!(
            load(&common::mainnet_program(name)).version(),
            SBPFVersion::V0,
            "{name}"
        );
    }
}
