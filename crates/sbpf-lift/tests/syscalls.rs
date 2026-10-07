//! Syscall'ы разрешаются в имена и в v0 (релокации), и в v3 (статические).

mod common;

use std::collections::BTreeSet;

use sbpf_lift::{CallTarget, Cfg, Program, solana_sbpf::ebpf, syscall_name};

fn syscalls(path: &std::path::Path) -> BTreeSet<&'static str> {
    let program = Program::load_file(path).unwrap();
    let cfg = Cfg::build(&program);
    cfg.instructions()
        .filter_map(|i| match i.call {
            Some(CallTarget::Syscall { name, .. }) => name,
            _ => None,
        })
        .collect()
}

#[test]
fn syscall_name_by_hash() {
    assert_eq!(
        syscall_name(ebpf::hash_symbol_name(b"sol_log_")),
        Some("sol_log_")
    );
    assert_eq!(
        syscall_name(ebpf::hash_symbol_name(b"no_such_syscall")),
        None
    );
}

#[test]
fn own_programs_syscalls() {
    let bin = common::programs_dir().join("bin");
    for version in ["v0", "v3"] {
        for (program, expected) in [
            ("native-basic", "sol_log_"),
            ("native-cpi", "sol_invoke_signed_rust"),
            ("anchor-basic", "sol_log_"),
        ] {
            let found = syscalls(&bin.join(format!("{program}.{version}.so")));
            assert!(
                found.contains(expected),
                "{program}.{version}: нет {expected}, есть {found:?}"
            );
        }
    }
}

#[test]
fn mainnet_programs_syscalls() {
    for name in common::MAINNET {
        let found = syscalls(&common::mainnet_program(name));
        assert!(!found.is_empty(), "{name}: syscall'ы не найдены");
    }
}

#[test]
fn disassembler_shows_syscall_names() {
    for version in ["v0", "v3"] {
        let path = common::programs_dir().join(format!("bin/native-cpi.{version}.so"));
        let cfg = Cfg::build(&Program::load_file(path).unwrap());
        assert!(
            cfg.instructions()
                .any(|i| i.text == "syscall sol_invoke_signed_rust"),
            "{version}: дизассемблер не показывает имя syscall'а"
        );
    }
}

/// v3: `call` с `src` не 0/1 интерпретатор не исполняет.
#[test]
fn v3_invalid_call() {
    let mut call = [ebpf::CALL_IMM, 0, 0, 0, 0, 0, 0, 0];
    call[4..].copy_from_slice(&ebpf::hash_symbol_name(b"sol_log_").to_le_bytes());
    let mut patched = call;
    patched[1] = 2 << 4;
    let mut bytes = read("native-basic.v3.so");
    replace(&mut bytes, &call, &patched);

    let cfg = Cfg::build(&Program::load(&bytes).unwrap());
    let block = cfg
        .blocks()
        .find(|b| {
            b.instructions
                .iter()
                .any(|i| i.call == Some(CallTarget::Invalid))
        })
        .expect("нет невалидного вызова");
    assert!(block.successors.is_empty());
}

fn read(file: &str) -> Vec<u8> {
    std::fs::read(common::programs_dir().join("bin").join(file)).unwrap()
}

fn replace(bytes: &mut [u8], from: &[u8], to: &[u8]) {
    let pos = bytes
        .windows(from.len())
        .position(|w| w == from)
        .expect("образец не найден");
    bytes[pos..pos + to.len()].copy_from_slice(to);
}

fn assert_reported(bytes: &[u8], name: &[u8]) {
    let hash = ebpf::hash_symbol_name(name);
    let cfg = Cfg::build(&Program::load(bytes).unwrap());
    assert!(cfg.unknown_syscalls().iter().any(|&(_, h)| h == hash));
}

/// Валидатор (`RequisiteVerifier`) хеши syscall'ов не проверяет, поэтому
/// неизвестный syscall не мешает загрузке ни в v0, ни в v3.
#[test]
fn unknown_syscall_is_reported() {
    // v0: хеш вписывает загрузчик по имени символа из релокации.
    let mut bytes = read("native-basic.v0.so");
    replace(&mut bytes, b"sol_log_\0", b"sol_xog_\0");
    assert_reported(&bytes, b"sol_xog_");

    // v3: хеш уже записан в инструкции call (src=0).
    let mut call = [ebpf::CALL_IMM, 0, 0, 0, 0, 0, 0, 0];
    call[4..].copy_from_slice(&ebpf::hash_symbol_name(b"sol_log_").to_le_bytes());
    let mut patched = call;
    patched[4..].copy_from_slice(&ebpf::hash_symbol_name(b"sol_xog_").to_le_bytes());
    let mut bytes = read("native-basic.v3.so");
    replace(&mut bytes, &call, &patched);
    assert_reported(&bytes, b"sol_xog_");
}
