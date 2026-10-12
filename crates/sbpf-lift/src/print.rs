//! Текстовая запись IR для чтения человеком, как в примере из `docs/ir.md`.

use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    io,
};

use crate::ir::{Exit, Expr, Fault, Function, Module, Stmt, Width};

pub fn write_ir<'a>(
    module: &Module,
    functions: impl IntoIterator<Item = &'a Function>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    let names: BTreeMap<usize, &str> = module
        .functions
        .iter()
        .map(|f| (f.entry, f.name.as_str()))
        .collect();
    for (i, function) in functions.into_iter().enumerate() {
        if i > 0 {
            writeln!(w)?;
        }
        writeln!(w, "{}:", function.name)?;
        for block in &function.blocks {
            writeln!(w, "  bb_{}:", block.start)?;
            for stmt in &block.statements {
                writeln!(w, "    {}", StmtText(stmt))?;
            }
            let mark = if block.noreturn { " (noreturn)" } else { "" };
            writeln!(w, "    {}{mark}", ExitText(&block.exit, &names))?;
        }
    }
    Ok(())
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Reg(r) => write!(f, "r{r}"),
            Expr::Const { value, w } => {
                if *value <= 0xffff {
                    write!(f, "{value}")?;
                } else {
                    write!(f, "{value:#x}")?;
                }
                if *w != Width::W64 {
                    write!(f, ":{}", w.bits())?;
                }
                Ok(())
            }
            Expr::Bin(op, a, b) => write!(f, "{}({a}, {b})", op.name()),
            Expr::Cmp(op, a, b) => write!(f, "{}({a}, {b})", op.name()),
            Expr::Un(op, a) => write!(f, "{}({a})", op.name()),
            Expr::Ext(op, w, a) => write!(f, "{}{}({a})", op.name(), w.bits()),
        }
    }
}

impl Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

struct StmtText<'a>(&'a Stmt);

impl Display for StmtText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Stmt::Assign { pc, dst, value } => write!(f, "[{pc}] r{dst} := {value}"),
            Stmt::Load { pc, dst, w, addr } => {
                write!(f, "[{pc}] r{dst} := load.{} {addr}", w.bits())
            }
            Stmt::Store { pc, w, addr, value } => {
                write!(f, "[{pc}] store.{} {addr}, {value}", w.bits())
            }
            Stmt::Check { pc, cond, error } => write!(f, "[{pc}] check {cond} {error}"),
        }
    }
}

struct ExitText<'a>(&'a Exit, &'a BTreeMap<usize, &'a str>);

impl Display for ExitText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let next = |next: &Option<usize>| match next {
            Some(pc) => format!("bb_{pc}"),
            None => "—".into(),
        };
        match self.0 {
            Exit::Next { target, pc: None } => write!(f, "→ bb_{target}"),
            Exit::Next {
                target,
                pc: Some(pc),
            } => write!(f, "[{pc}] next → bb_{target}"),
            Exit::Branch {
                pc,
                cond,
                then,
                r#else,
            } => write!(f, "[{pc}] branch {cond} → bb_{then}, bb_{}", r#else),
            Exit::Call {
                pc,
                target,
                next: n,
            } => {
                let name = self.1.get(target).copied().unwrap_or("?");
                write!(f, "[{pc}] call {name} → {}", next(n))
            }
            Exit::Callx { pc, reg, next: n } => write!(f, "[{pc}] callx r{reg} → {}", next(n)),
            Exit::Syscall {
                pc,
                hash,
                name,
                next: n,
            } => match name {
                Some(name) => write!(f, "[{pc}] syscall {name} → {}", next(n)),
                None => write!(f, "[{pc}] syscall {hash:#010x} → {}", next(n)),
            },
            Exit::Return { pc } => write!(f, "[{pc}] return"),
            Exit::Trap { pc, error } => write!(f, "[{pc}] trap {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::{BinOp, Expr, Width};

    #[test]
    fn expr() {
        let cases = [
            (Expr::c64(0xffff), "65535"),
            (Expr::c64(0x10000), "0x10000"),
            (Expr::c64(u64::MAX), "0xffffffffffffffff"),
            (Expr::c32(23), "23:32"),
            (Expr::constant(0xff, Width::W8), "255:8"),
            (
                Expr::sext64(Expr::bin(
                    BinOp::Add,
                    Expr::lo32(Expr::reg(1)),
                    Expr::c32(1),
                )),
                "sext64(add(trunc32(r1), 1:32))",
            ),
        ];
        for (e, text) in cases {
            assert_eq!(e.to_string(), text);
        }
    }
}
