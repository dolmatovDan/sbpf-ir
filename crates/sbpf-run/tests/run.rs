use std::path::{Path, PathBuf};

use sbpf_run::{Example, Options, Outcome, RunError, Status, run, run_file};

fn examples_dir() -> PathBuf {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/examples")).to_owned()
}

fn example(name: &str) -> PathBuf {
    examples_dir().join(format!("{name}.json"))
}

fn load(path: &Path) -> Example {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn outcome(name: &str) -> Outcome {
    run_file(&example(name), Options::default()).unwrap()
}

fn all_examples() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for dir in std::fs::read_dir(examples_dir()).unwrap() {
        let dir = dir.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        for file in std::fs::read_dir(dir).unwrap() {
            let path = file.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "json") {
                paths.push(path);
            }
        }
    }
    assert!(paths.len() >= 24);
    paths.sort();
    paths
}

fn program_logs(outcome: &Outcome) -> Vec<&str> {
    outcome
        .logs
        .iter()
        .filter_map(|l| l.strip_prefix("Program log: "))
        .collect()
}

fn assert_success_log(name: &str, log: &str) {
    let o = outcome(name);
    assert_eq!(o.status, Status::Success, "{name}: {:?}", o.error);
    assert_eq!(program_logs(&o), [log], "{name}");
}

fn assert_failure(name: &str, error: &str) {
    let o = outcome(name);
    assert_eq!(o.status, Status::Failure, "{name}");
    assert_eq!(o.error.as_deref(), Some(error), "{name}");
}

#[test]
fn native_basic() {
    assert_success_log("native-basic/alu64", "op 0 -> 2688");
    assert_success_log("native-basic/alu32", "op 1 -> 4294966847");
    assert_success_log("native-basic/div", "op 2 -> 14");
    assert_success_log("native-basic/signed-cmp", "op 3 -> 18446744073709551615");
    assert_success_log("native-basic/sum-loop", "op 4 -> 285");
    assert_success_log("native-basic/rem32", "op 5 -> 18446744073709551615");
    assert_failure("native-basic/div-by-zero", "ArithmeticOverflow");
    assert_failure("native-basic/bad-op", "InvalidInstructionData");
    assert_failure("native-basic/short-data", "InvalidInstructionData");
}

#[test]
fn native_cpi() {
    let lamports = |o: &Outcome| o.accounts.iter().map(|a| a.lamports).collect::<Vec<_>>();

    let o = outcome("native-cpi/transfer");
    assert_eq!(o.status, Status::Success, "{:?}", o.error);
    assert_eq!(lamports(&o), [1_900_000, 1_100_000, 1]);
    assert!(
        o.logs
            .contains(&"Program 11111111111111111111111111111111 invoke [2]".to_owned())
    );

    let o = outcome("native-cpi/vault-transfer");
    assert_eq!(o.status, Status::Success, "{:?}", o.error);
    assert_eq!(lamports(&o), [1_700_000, 1_300_000, 1]);
    assert_eq!(program_logs(&o), ["vault transfer 300000"]);

    let o = outcome("native-cpi/transfer-below-rent");
    assert_eq!(o.status, Status::Failure);
    assert_eq!(o.error.as_deref(), Some("InsufficientFundsForRent"));
    assert_eq!(lamports(&o), [2_000_000, 0, 1]);

    assert_failure("native-cpi/transfer-unsigned", "MissingRequiredSignature");
    assert_failure("native-cpi/vault-wrong-seeds", "InvalidSeeds");
}

#[test]
fn anchor_basic() {
    let o = outcome("anchor-basic/increment");
    assert_eq!(o.status, Status::Success, "{:?}", o.error);
    assert_eq!(program_logs(&o), ["Instruction: Increment", "count = 8"]);
    let counter = &o.accounts[0].data;
    assert_eq!(&counter[80..], "0800000000000000");

    assert_failure("anchor-basic/increment-overflow", "Custom(6000)");
    assert_failure("anchor-basic/increment-wrong-authority", "Custom(2001)");
}

/// Итог asm-программ в r1–r9 на v0 и v3; ожидания посчитаны по семантике `docs/ir.md`.
#[test]
fn asm() {
    let options = Options {
        registers: true,
        trace: false,
    };
    let cases: [(&str, [u64; 9]); 7] = [
        (
            "mov",
            [
                42,
                u64::MAX,
                u64::MAX,
                0xffffffff,
                0x8000000000000001,
                1,
                0xdeadbeefcafebabe,
                0xdeadbeefcafebabe,
                0xcafebabe,
            ],
        ),
        (
            "alu64",
            [
                3,
                0xfffffffffffffff9,
                0xffffffffffffffd6,
                0x200000001,
                0x200000f01,
                0xff00ff02ff00ff0e,
                0xf00,
                0xffffffffffffedcb,
                0xfffffffffffffffb,
            ],
        ),
        (
            "shift64",
            [
                0x8000000000000000,
                0xf800000000000000,
                0x0800000000000000,
                70,
                192,
                0xfffffffffffffffc,
                0x03fffffffffffffc,
                64,
                0x1234,
            ],
        ),
        (
            "alu32",
            [
                0xffffffff80000000,
                0xfffffffffffffffe,
                1,
                0xffffffff80008000,
                0xfffffffffffff830,
                0xff0,
                0x80000001,
                0x7cf,
                0xfffffffb,
            ],
        ),
        (
            "shift32",
            [
                0x80000000, 0xf8000000, 0x08000000, 36, 48, 0xfffffff0, 0x0ffffff0, 32, 0x1234,
            ],
        ),
        (
            "divmod",
            [
                14,
                2,
                0x5555555555555555,
                9,
                71,
                0x55555555,
                2,
                0x12492491,
                2,
            ],
        ),
        (
            "endian",
            [
                0x7788,
                0x55667788,
                0x1122334455667788,
                0x8877,
                0x88776655,
                0x8877665544332211,
                0x0201,
                0xffffffff,
                0x3412000000000000,
            ],
        ),
    ];
    for (name, expected) in cases {
        let path = example(&format!("asm/{name}"));
        let v0 = load(&path);
        let mut v3 = v0.clone();
        v3.program = v0.program.replace(".v0.so", ".v3.so");
        for e in [v0, v3] {
            let o = run(&e, path.parent().unwrap(), options).unwrap();
            assert_eq!(o.status, Status::Success, "{}: {:?}", e.program, o.error);
            let registers = o.registers.unwrap().unwrap();
            assert_eq!(registers[0], 0, "{}", e.program);
            assert_eq!(registers[1..10], expected, "{}", e.program);
        }
    }
}

#[test]
fn input_accounts_unchanged_on_failure() {
    let path = example("native-cpi/transfer-unsigned");
    let input = load(&path);
    let o = run_file(&path, Options::default()).unwrap();
    for (before, after) in input.accounts.iter().zip(&o.accounts) {
        assert_eq!(before.key, after.key);
        assert_eq!(before.lamports, after.lamports);
    }
}

/// v0 и v3 одного контракта дают одинаковый итог; различаются только CU.
#[test]
fn v0_and_v3_agree() {
    for path in all_examples() {
        let v0 = load(&path);
        assert!(v0.program.ends_with(".v0.so"), "{}", path.display());
        let mut v3 = v0.clone();
        v3.program = v0.program.replace(".v0.so", ".v3.so");
        let dir = path.parent().unwrap();
        let normalize = |mut o: Outcome| {
            o.compute_units = 0;
            o.logs.retain(|l| !l.contains(" consumed "));
            o
        };
        let a = normalize(run(&v0, dir, Options::default()).unwrap());
        let b = normalize(run(&v3, dir, Options::default()).unwrap());
        assert_eq!(a, b, "{}", path.display());
    }
}

#[test]
fn registers_and_trace() {
    let options = Options {
        registers: true,
        trace: true,
    };
    for path in all_examples() {
        let o = run_file(&path, options).unwrap();
        let registers = o.registers.unwrap().unwrap();
        let trace = o.trace.unwrap();
        assert_eq!(trace.last(), Some(&registers), "{}", path.display());
        assert!(trace.len() as u64 <= o.compute_units, "{}", path.display());
        if o.status == Status::Success {
            assert_eq!(registers[0], 0, "{}", path.display());
        }
    }

    let o = outcome("native-basic/alu64");
    assert_eq!((o.registers, o.trace), (None, None));
}

#[test]
fn extra_programs_load() {
    let path = example("native-basic/alu64");
    let mut e = load(&path);
    e.programs.push(sbpf_run::ExtraProgram {
        program_id: "Cpi1111111111111111111111111111111111111111".into(),
        program: "../../programs/bin/native-cpi.v3.so".into(),
    });
    let o = run(&e, path.parent().unwrap(), Options::default()).unwrap();
    assert_eq!(o.status, Status::Success);
}

#[test]
fn bad_input() {
    let path = example("native-basic/alu64");
    let base = load(&path);
    let dir = path.parent().unwrap();

    let mut e = base.clone();
    e.program_id = "не ключ".into();
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::Pubkey { .. })
    ));

    let mut e = base.clone();
    e.instruction_data = "0g".into();
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::Hex { .. })
    ));

    let mut e = base.clone();
    e.program = "nope.so".into();
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::Io { .. })
    ));

    let mut e = base;
    e.program = "../../programs/native-basic/Cargo.toml".into();
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::NotSo(_))
    ));
}

#[test]
fn compute_unit_limit() {
    let o = outcome("native-basic/alu64");
    assert!(
        o.logs
            .iter()
            .any(|l| l.ends_with(" of 200000 compute units"))
    );

    let path = example("native-basic/alu64");
    let mut e = load(&path);
    e.compute_unit_limit = Some(500);
    let o = run(&e, path.parent().unwrap(), Options::default()).unwrap();
    assert_eq!(o.status, Status::Failure);
    assert_eq!(o.error.as_deref(), Some("ProgramFailedToComplete"));
    assert!(
        o.logs
            .last()
            .unwrap()
            .ends_with("exceeded CUs meter at BPF instruction")
    );
    assert_eq!(o.compute_units, 500);
}

#[test]
fn duplicate_accounts() {
    let path = example("native-cpi/transfer");
    let base = load(&path);
    let dir = path.parent().unwrap();

    let mut e = base.clone();
    let mut alias = e.accounts[0].clone();
    alias.is_signer = false;
    e.accounts.push(alias);
    let o = run(&e, dir, Options::default()).unwrap();
    assert_eq!(o.status, Status::Success, "{:?}", o.error);
    assert_eq!(o.accounts.len(), 3);

    let mut e = base;
    let mut alias = e.accounts[0].clone();
    alias.lamports += 1;
    e.accounts.push(alias);
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::ConflictingAccount(_))
    ));
}

#[test]
fn shadowed_programs_rejected() {
    let path = example("native-basic/alu64");
    let base = load(&path);
    let dir = path.parent().unwrap();
    for id in [
        base.program_id.as_str(),
        "11111111111111111111111111111111",
        "BPFLoaderUpgradeab1e11111111111111111111111",
    ] {
        let mut e = base.clone();
        e.programs.push(sbpf_run::ExtraProgram {
            program_id: id.into(),
            program: "../../programs/bin/native-cpi.v0.so".into(),
        });
        assert!(
            matches!(
                run(&e, dir, Options::default()),
                Err(RunError::ShadowedProgram { index: 0, .. })
            ),
            "{id}"
        );
    }

    let mut e = base;
    let extra = sbpf_run::ExtraProgram {
        program_id: "Cpi1111111111111111111111111111111111111111".into(),
        program: "../../programs/bin/native-cpi.v0.so".into(),
    };
    e.programs = vec![extra.clone(), extra];
    assert!(matches!(
        run(&e, dir, Options::default()),
        Err(RunError::ShadowedProgram { index: 1, .. })
    ));
}

#[test]
fn requested_registers_always_serialized() {
    let mut o = outcome("native-basic/alu64");
    let json = serde_json::to_value(&o).unwrap();
    assert!(json.get("registers").is_none() && json.get("trace").is_none());

    o.registers = Some(None);
    o.trace = Some(Vec::new());
    let json = serde_json::to_value(&o).unwrap();
    assert_eq!(json["registers"], serde_json::Value::Null);
    assert_eq!(json["trace"], serde_json::json!([]));
}
