//! Смоук-тест: все тестовые контракты загружаются через `solana-sbpf`
//! и определяются с ожидаемой версией sBPF.

use std::{fs, path::Path, ptr::NonNull, sync::Arc};

use solana_sbpf::{
    elf::Executable,
    memory_region::MemoryMapping,
    program::{BuiltinProgram, SBPFVersion},
    vm::{Config, ContextObject},
};

/// Контекст-заглушка: на этапе загрузки программа не исполняется.
struct NoExec;

impl ContextObject for NoExec {
    fn consume(&mut self, _amount: u64) {}

    fn get_remaining(&self) -> u64 {
        0
    }

    fn active_mapping_ptr(&mut self) -> NonNull<MemoryMapping> {
        unreachable!("программа не исполняется")
    }
}

fn load(path: &Path) -> Executable<NoExec> {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let loader = Arc::new(BuiltinProgram::new_loader(Config::default()));
    Executable::from_elf(&bytes, loader).unwrap_or_else(|e| panic!("{}: {e:?}", path.display()))
}

fn programs_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/programs"))
}

#[test]
fn own_programs_load_with_expected_version() {
    let mut count = 0;
    for entry in fs::read_dir(programs_dir().join("bin")).unwrap() {
        let path = entry.unwrap().path();
        let expected = match path.file_name().unwrap().to_str().unwrap() {
            name if name.ends_with(".v0.so") => SBPFVersion::V0,
            name if name.ends_with(".v3.so") => SBPFVersion::V3,
            _ => continue,
        };
        assert_eq!(
            load(&path).get_sbpf_version(),
            expected,
            "{}",
            path.display()
        );
        count += 1;
    }
    assert_eq!(count, 6, "ожидалось 3 программы × (v0, v3)");
}

#[test]
fn mainnet_programs_load() {
    for name in ["p-token", "ata", "marinade"] {
        let path = programs_dir().join("mainnet").join(format!("{name}.so"));
        assert_eq!(load(&path).get_sbpf_version(), SBPFVersion::V0, "{name}");
    }
}
