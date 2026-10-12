use std::path::{Path, PathBuf};

use sbpf_run::{Example, Options, Outcome, RunError, Status, run, run_file};

fn examples_dir() -> PathBuf {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/examples")).to_owned()
}

fn example(name: &str) -> PathBuf {
    examples_dir().join(format!("{name}.json"))
}

fn outcome(name: &str) -> Outcome {
    run_file(&example(name), Options::default()).unwrap()
}

fn all_examples() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for dir in std::fs::read_dir(examples_dir()).unwrap() {
        for file in std::fs::read_dir(dir.unwrap().path()).unwrap() {
            paths.push(file.unwrap().path());
        }
    }
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

#[test]
fn input_accounts_unchanged_on_failure() {
    let path = example("native-cpi/transfer-unsigned");
    let input: Example = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
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
        let text = std::fs::read_to_string(&path).unwrap();
        let v0: Example = serde_json::from_str(&text).unwrap();
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
        let registers = o.registers.unwrap();
        let trace = o.trace.unwrap();
        assert_eq!(trace.last(), Some(&registers), "{}", path.display());
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
    let mut e: Example = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
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
    let base: Example = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
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
