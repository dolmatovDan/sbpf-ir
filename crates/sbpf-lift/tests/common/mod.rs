//! Shared test helpers: paths to test programs.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

pub fn programs_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/programs"))
}

/// Our own test programs: `bin/<name>.<v0|v3>.so`.
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

/// All test programs: our own and mainnet ones.
pub fn all_programs() -> Vec<PathBuf> {
    let mut paths = own_programs();
    paths.extend(MAINNET.map(mainnet_program));
    paths
}
