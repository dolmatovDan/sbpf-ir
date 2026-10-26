//! Инварианты IR на всех тестовых контрактах.

mod common;

use std::collections::BTreeMap;

use sbpf_lift::{
    Cfg, Program,
    ir::{Exit, Expr, Fault, Module, Stmt},
    lift,
    solana_sbpf::ebpf,
};

/// Третье значение — число слотов кода по длине `.text`, независимо от декодирования в CFG.
fn lift_file(path: &std::path::Path) -> (Cfg, Module, usize) {
    let program = Program::load_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let cfg = Cfg::build(&program);
    let module = lift(&program, &cfg).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let slots = program.executable().get_text_bytes().1.len() / ebpf::INSN_SIZE;
    (cfg, module, slots)
}

fn exit_pc(exit: &Exit) -> Option<usize> {
    match *exit {
        Exit::Next { pc, .. } => pc,
        Exit::Branch { pc, .. }
        | Exit::Call { pc, .. }
        | Exit::Callx { pc, .. }
        | Exit::Syscall { pc, .. }
        | Exit::Return { pc }
        | Exit::Trap { pc, .. } => Some(pc),
    }
}

#[test]
fn invariants() {
    for path in common::all_programs() {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let (cfg, module, end) = lift_file(&path);
        module.validate().unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(module.entrypoint, cfg.entrypoint, "{name}");
        assert_eq!(module.address_taken, cfg.address_taken, "{name}");

        // Одна инструкция — ровно один statement (кроме check) или выход с pc.
        let mut seen: BTreeMap<usize, usize> = BTreeMap::new();
        for block in module.functions.iter().flat_map(|f| &f.blocks) {
            for stmt in &block.statements {
                let pc = match *stmt {
                    Stmt::Check { .. } => continue,
                    Stmt::Assign { pc, .. } | Stmt::Load { pc, .. } | Stmt::Store { pc, .. } => pc,
                };
                *seen.entry(pc).or_default() += 1;
            }
            match block.exit {
                Exit::Trap {
                    pc,
                    error: Fault::ExecutionOverrun,
                } => assert_eq!(pc, end, "{name}"),
                ref exit => {
                    if let Some(pc) = exit_pc(exit) {
                        *seen.entry(pc).or_default() += 1;
                    }
                }
            }
        }
        let expected: BTreeMap<usize, usize> =
            cfg.instructions().map(|i| (i.insn.ptr, 1)).collect();
        assert_eq!(seen, expected, "{name}");

        // Блоки и noreturn — те же, что в CFG.
        let blocks: Vec<_> = cfg.blocks().map(|b| (b.start, b.noreturn)).collect();
        let lifted: Vec<_> = module
            .functions
            .iter()
            .flat_map(|f| &f.blocks)
            .filter(|b| b.start != end)
            .map(|b| (b.start, b.noreturn))
            .collect();
        assert_eq!(lifted, blocks, "{name}");

        let json = serde_json::to_string(&module).unwrap();
        let back: Module = serde_json::from_str(&json).unwrap();
        assert_eq!(back, module, "{name}");
    }
}

/// Asm-примеры для первой версии машины: один блок без памяти и вызовов,
/// регистр читается только после записи, в итоге записаны все r1–r9.
#[test]
fn asm_programs_are_straight_line() {
    let mut count = 0;
    for path in common::own_programs() {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.starts_with("asm-") {
            continue;
        }
        count += 1;
        let (_, module, _) = lift_file(&path);
        let [function] = module.functions.as_slice() else {
            panic!("{name}: функций {}", module.functions.len());
        };
        let [block] = function.blocks.as_slice() else {
            panic!("{name}: блоков {}", function.blocks.len());
        };
        assert!(matches!(block.exit, Exit::Return { .. }), "{name}");
        let mut written = [false; 11];
        for stmt in &block.statements {
            match stmt {
                Stmt::Assign { dst, value, .. } => {
                    assert_reads_written(value, &written, &name);
                    written[*dst as usize] = true;
                }
                Stmt::Check {
                    cond,
                    error: Fault::DivideByZero,
                    ..
                } if name.starts_with("asm-divmod.") => assert_reads_written(cond, &written, &name),
                other => panic!("{name}: {other:?}"),
            }
        }
        assert!(
            written[1..10].iter().all(|&w| w),
            "{name}: записаны не все r1–r9"
        );
    }
    assert_eq!(count, 14, "ожидалось 7 asm-программ × (v0, v3)");
}

fn assert_reads_written(expr: &Expr, written: &[bool; 11], name: &str) {
    match expr {
        Expr::Reg(r) => assert!(written[*r as usize], "{name}: r{r} читается до записи"),
        Expr::Const { .. } => {}
        Expr::Un(_, a) | Expr::Ext(_, _, a) => assert_reads_written(a, written, name),
        Expr::Bin(_, a, b) | Expr::Cmp(_, a, b) => {
            assert_reads_written(a, written, name);
            assert_reads_written(b, written, name);
        }
    }
}

#[test]
fn header() {
    let (_, v0, _) = lift_file(&common::programs_dir().join("bin/native-basic.v0.so"));
    let (_, v3, _) = lift_file(&common::programs_dir().join("bin/native-basic.v3.so"));
    assert_eq!((v0.format_version, v0.sbpf_version), (1, 0));
    assert_eq!((v3.format_version, v3.sbpf_version), (1, 3));
    // В v0 код входит в read-only регион, в v3 нет.
    let contains_text = |m: &Module| {
        (m.ro_region.vaddr..m.ro_region.vaddr + m.ro_region.data.len() as u64)
            .contains(&m.text_vaddr)
    };
    assert!(contains_text(&v0));
    assert!(!contains_text(&v3));
}

/// Пример `checked_div` из `docs/ir.md` (native-basic, v3); имя функции в сборке
/// без символов — `fn_<pc>`, поэтому её ищем по коду, а не по имени.
#[test]
fn doc_example_text() {
    let (_, module, _) = lift_file(&common::programs_dir().join("bin/native-basic.v3.so"));
    let expected = "  bb_84:
    [84] branch eq(r3, 0) → bb_90, bb_85
  bb_85:
    [85] check ne(r3, 0) DivideByZero
    [85] r2 := udiv(r2, r3)
    [86] store.64 add(r1, 8), r2
    [87] r2 := 0
    → bb_88
  bb_88:
    [88] store.32 add(r1, 0), trunc32(r2)
    [89] return
  bb_90:
    [90] store.32 add(r1, 4), 23:32
    [91] r2 := 1
    [92] next → bb_88
";
    let mut found = 0;
    for function in &module.functions {
        let mut out = Vec::new();
        sbpf_lift::write_ir(&module, [function], &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        let body = text.split_once('\n').unwrap().1;
        if renumber(body, function.entry) == renumber(expected, 84) {
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

/// Сдвигает все pc так, чтобы функция начиналась с 0: пример не зависит от сборки.
/// pc меньше `base` (переход в более раннюю функцию) превращается в `?`.
fn renumber(text: &str, base: usize) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(i) = rest.find(|c: char| c.is_ascii_digit()) {
        let (before, tail) = rest.split_at(i);
        let len = tail
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(tail.len());
        let (digits, after) = tail.split_at(len);
        out.push_str(before);
        let is_pc = before.ends_with('[') || before.ends_with("bb_");
        match digits.parse::<usize>() {
            Ok(pc) if is_pc => match pc.checked_sub(base) {
                Some(pc) => out.push_str(&pc.to_string()),
                None => out.push('?'),
            },
            _ => out.push_str(digits),
        }
        rest = after;
    }
    out.push_str(rest);
    out
}
