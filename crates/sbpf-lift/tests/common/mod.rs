//! Общие помощники тестов: пути к тестовым контрактам.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

pub fn programs_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/programs"))
}

/// Собственные тестовые контракты: `bin/<имя>.<v0|v3>.so`.
pub fn own_programs() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(programs_dir().join("bin"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "so"))
        .collect();
    paths.sort();
    paths
}

pub const MAINNET: [&str; 3] = ["p-token", "ata", "marinade"];

pub fn mainnet_program(name: &str) -> PathBuf {
    programs_dir().join("mainnet").join(format!("{name}.so"))
}

/// Все тестовые контракты: собственные и из мейннета.
pub fn all_programs() -> Vec<PathBuf> {
    let mut paths = own_programs();
    paths.extend(MAINNET.map(mainnet_program));
    paths
}
