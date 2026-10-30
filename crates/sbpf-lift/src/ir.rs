//! Типы IR. Формат и семантика — `docs/ir.md`.

mod json;

use serde::{Deserialize, Serialize};

pub const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Width {
    W8,
    W16,
    W32,
    W64,
}

impl Width {
    pub fn bits(self) -> u32 {
        match self {
            Width::W8 => 8,
            Width::W16 => 16,
            Width::W32 => 32,
            Width::W64 => 64,
        }
    }

    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            8 => Some(Width::W8),
            16 => Some(Width::W16),
            32 => Some(Width::W32),
            64 => Some(Width::W64),
            _ => None,
        }
    }

    pub fn mask(self) -> u64 {
        u64::MAX >> (64 - self.bits())
    }
}

macro_rules! ops {
    ($($ty:ident { $($variant:ident = $name:literal),+ $(,)? })+) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $ty {
            $($variant),+
        }

        impl $ty {
            pub const ALL: &[Self] = &[$(Self::$variant),+];

            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name),+
                }
            }
        }
    )+};
}

ops! {
    BinOp {
        Add = "add",
        Sub = "sub",
        Mul = "mul",
        And = "and",
        Or = "or",
        Xor = "xor",
        Udiv = "udiv",
        Urem = "urem",
        Shl = "shl",
        Lshr = "lshr",
        Ashr = "ashr",
    }
    UnOp {
        Neg = "neg",
        Bswap = "bswap",
    }
    ExtOp {
        Zext = "zext",
        Sext = "sext",
        Trunc = "trunc",
    }
    CmpOp {
        Eq = "eq",
        Ne = "ne",
        Ult = "ult",
        Ule = "ule",
        Ugt = "ugt",
        Uge = "uge",
        Slt = "slt",
        Sle = "sle",
        Sgt = "sgt",
        Sge = "sge",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Reg(u8),
    Const { value: u64, w: Width },
    Bin(BinOp, Box<Expr>, Box<Expr>),
    Un(UnOp, Box<Expr>),
    Ext(ExtOp, Width, Box<Expr>),
    Cmp(CmpOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    Bv(Width),
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TypeError {
    #[error("ожидался тип {expected:?}, получено {got:?}")]
    Unexpected { expected: Ty, got: Ty },
    #[error("нет регистра r{0}")]
    BadReg(u8),
    #[error("константа {value:#x} не помещается в {} бит", w.bits())]
    ConstOverflow { value: u64, w: Width },
    #[error("{op}: ожидался битвектор, получено {got:?}")]
    ExpectedBv { op: &'static str, got: Ty },
    #[error("{op}: разные ширины {left} и {right}")]
    WidthMismatch {
        op: &'static str,
        left: u32,
        right: u32,
    },
    #[error("{op}: недопустимая ширина {w}")]
    BadOperandWidth { op: &'static str, w: u32 },
    #[error("{op}: недопустимая ширина {from} → {to}")]
    BadWidth {
        op: &'static str,
        from: u32,
        to: u32,
    },
}

impl Expr {
    pub fn reg(r: u8) -> Self {
        Expr::Reg(r)
    }

    /// Старшие биты сверх `w` отбрасываются.
    pub fn constant(value: u64, w: Width) -> Self {
        Expr::Const {
            value: value & w.mask(),
            w,
        }
    }

    pub fn c64(value: u64) -> Self {
        Self::constant(value, Width::W64)
    }

    pub fn c32(value: u32) -> Self {
        Self::constant(value.into(), Width::W32)
    }

    pub fn bin(op: BinOp, a: Expr, b: Expr) -> Self {
        Expr::Bin(op, Box::new(a), Box::new(b))
    }

    pub fn un(op: UnOp, a: Expr) -> Self {
        Expr::Un(op, Box::new(a))
    }

    pub fn ext(op: ExtOp, w: Width, a: Expr) -> Self {
        Expr::Ext(op, w, Box::new(a))
    }

    pub fn cmp(op: CmpOp, a: Expr, b: Expr) -> Self {
        Expr::Cmp(op, Box::new(a), Box::new(b))
    }

    pub fn zext64(a: Expr) -> Self {
        Self::ext(ExtOp::Zext, Width::W64, a)
    }

    pub fn sext64(a: Expr) -> Self {
        Self::ext(ExtOp::Sext, Width::W64, a)
    }

    pub fn trunc(w: Width, a: Expr) -> Self {
        Self::ext(ExtOp::Trunc, w, a)
    }

    pub fn lo32(a: Expr) -> Self {
        Self::trunc(Width::W32, a)
    }

    pub fn ty(&self) -> Result<Ty, TypeError> {
        match self {
            Expr::Reg(r) if *r <= 10 => Ok(Ty::Bv(Width::W64)),
            Expr::Reg(r) => Err(TypeError::BadReg(*r)),
            Expr::Const { value, w } if value & !w.mask() == 0 => Ok(Ty::Bv(*w)),
            &Expr::Const { value, w } => Err(TypeError::ConstOverflow { value, w }),
            Expr::Bin(op, a, b) => same_width(op.name(), a, b).map(Ty::Bv),
            Expr::Un(op, a) => {
                let w = bv(op.name(), a)?;
                if *op == UnOp::Bswap && w == Width::W8 {
                    return Err(TypeError::BadOperandWidth {
                        op: op.name(),
                        w: 8,
                    });
                }
                Ok(Ty::Bv(w))
            }
            Expr::Ext(op, to, a) => {
                let from = bv(op.name(), a)?;
                let ok = match op {
                    ExtOp::Zext | ExtOp::Sext => *to >= from,
                    ExtOp::Trunc => *to <= from,
                };
                if !ok {
                    return Err(TypeError::BadWidth {
                        op: op.name(),
                        from: from.bits(),
                        to: to.bits(),
                    });
                }
                Ok(Ty::Bv(*to))
            }
            Expr::Cmp(op, a, b) => same_width(op.name(), a, b).map(|_| Ty::Bool),
        }
    }
}

fn bv(op: &'static str, e: &Expr) -> Result<Width, TypeError> {
    match e.ty()? {
        Ty::Bv(w) => Ok(w),
        got => Err(TypeError::ExpectedBv { op, got }),
    }
}

fn same_width(op: &'static str, a: &Expr, b: &Expr) -> Result<Width, TypeError> {
    let (left, right) = (bv(op, a)?, bv(op, b)?);
    if left != right {
        return Err(TypeError::WidthMismatch {
            op,
            left: left.bits(),
            right: right.bits(),
        });
    }
    Ok(left)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fault {
    DivideByZero,
    UnsupportedInstruction,
    ExecutionOverrun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Stmt {
    Assign {
        pc: usize,
        dst: u8,
        value: Expr,
    },
    Load {
        pc: usize,
        dst: u8,
        w: Width,
        addr: Expr,
    },
    Store {
        pc: usize,
        w: Width,
        addr: Expr,
        value: Expr,
    },
    Check {
        pc: usize,
        cond: Expr,
        error: Fault,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Exit {
    /// Без `pc` — продолжение в следующий блок, с `pc` — инструкция `ja`.
    Next {
        target: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pc: Option<usize>,
    },
    Branch {
        pc: usize,
        cond: Expr,
        then: usize,
        r#else: usize,
    },
    /// `next == None` — вызов последний в программе.
    Call {
        pc: usize,
        target: usize,
        next: Option<usize>,
    },
    Callx {
        pc: usize,
        reg: u8,
        next: Option<usize>,
    },
    Syscall {
        pc: usize,
        hash: u32,
        name: Option<String>,
        next: Option<usize>,
    },
    Return {
        pc: usize,
    },
    Trap {
        pc: usize,
        error: Fault,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub start: usize,
    pub noreturn: bool,
    pub statements: Vec<Stmt>,
    pub exit: Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub entry: usize,
    pub name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoRegion {
    #[serde(with = "json::hex")]
    pub vaddr: u64,
    #[serde(with = "json::base64")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub format_version: u32,
    pub sbpf_version: u8,
    pub entrypoint: usize,
    #[serde(with = "json::hex")]
    pub text_vaddr: u64,
    pub ro_region: RoRegion,
    pub address_taken: Vec<usize>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ValidationError(String);

impl Module {
    /// Структурные инварианты из `docs/ir.md`: типы выражений, уникальные блоки,
    /// существующие цели переходов и вызовов.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let err = |msg: String| Err(ValidationError(msg));
        if self.format_version != FORMAT_VERSION {
            return err(format!("format_version {}", self.format_version));
        }
        if !matches!(self.sbpf_version, 0 | 3) {
            return err(format!("sbpf_version {}", self.sbpf_version));
        }
        let mut starts = std::collections::BTreeSet::new();
        for block in self.functions.iter().flat_map(|f| &f.blocks) {
            if !starts.insert(block.start) {
                return err(format!("блок {} повторяется", block.start));
            }
        }
        let entries: std::collections::BTreeSet<_> =
            self.functions.iter().map(|f| f.entry).collect();
        if !entries.contains(&self.entrypoint) {
            return err(format!("точка входа {} не вход функции", self.entrypoint));
        }
        if let Some(pc) = self.address_taken.iter().find(|pc| !entries.contains(pc)) {
            return err(format!("address_taken: {pc} не вход функции"));
        }
        for function in &self.functions {
            if function.blocks.first().map(|b| b.start) != Some(function.entry) {
                return err(format!("функция {} начинается не с entry", function.entry));
            }
            for block in &function.blocks {
                let at = |e: String| ValidationError(format!("блок {}: {e}", block.start));
                for stmt in &block.statements {
                    stmt.validate().map_err(at)?;
                }
                let targets: Vec<usize> = match &block.exit {
                    Exit::Next { target, .. } => vec![*target],
                    Exit::Branch {
                        cond, then, r#else, ..
                    } => {
                        expect_ty(cond, Ty::Bool).map_err(|e| at(e.to_string()))?;
                        vec![*then, *r#else]
                    }
                    Exit::Call { target, next, .. } => {
                        if !entries.contains(target) {
                            return err(format!("блок {}: call не в функцию", block.start));
                        }
                        next.iter().copied().collect()
                    }
                    Exit::Callx { reg, next, .. } => {
                        if *reg > 9 {
                            return err(format!("блок {}: callx через r{reg}", block.start));
                        }
                        next.iter().copied().collect()
                    }
                    Exit::Syscall { next, .. } => next.iter().copied().collect(),
                    Exit::Return { .. } | Exit::Trap { .. } => vec![],
                };
                if let Some(pc) = targets.iter().find(|pc| !starts.contains(pc)) {
                    return err(format!(
                        "блок {}: переход в несуществующий блок {pc}",
                        block.start
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Stmt {
    fn validate(&self) -> Result<(), String> {
        let dst_ok = |dst: u8| {
            if dst <= 9 {
                Ok(())
            } else {
                Err(format!("запись в r{dst}"))
            }
        };
        let ty = |e: &Expr, ty| expect_ty(e, ty).map_err(|e| e.to_string());
        match self {
            Stmt::Assign { dst, value, .. } => {
                dst_ok(*dst)?;
                ty(value, Ty::Bv(Width::W64))
            }
            Stmt::Load { dst, addr, .. } => {
                dst_ok(*dst)?;
                ty(addr, Ty::Bv(Width::W64))
            }
            Stmt::Store { w, addr, value, .. } => {
                ty(addr, Ty::Bv(Width::W64))?;
                ty(value, Ty::Bv(*w))
            }
            Stmt::Check { cond, .. } => ty(cond, Ty::Bool),
        }
    }
}

fn expect_ty(e: &Expr, expected: Ty) -> Result<(), TypeError> {
    match e.ty()? {
        got if got == expected => Ok(()),
        got => Err(TypeError::Unexpected { expected, got }),
    }
}

#[cfg(test)]
mod tests {
    use super::{BinOp::*, CmpOp::*, ExtOp::*, Width::*, *};

    fn r(n: u8) -> Expr {
        Expr::reg(n)
    }

    #[test]
    fn well_typed() {
        let cases = [
            (r(10), Ty::Bv(W64)),
            (Expr::c32(5), Ty::Bv(W32)),
            (Expr::bin(Add, r(1), Expr::c64(u64::MAX)), Ty::Bv(W64)),
            (
                Expr::sext64(Expr::bin(Add, Expr::lo32(r(1)), Expr::c32(1))),
                Ty::Bv(W64),
            ),
            (Expr::ext(Zext, W64, r(1)), Ty::Bv(W64)),
            (Expr::trunc(W64, r(1)), Ty::Bv(W64)),
            (Expr::un(UnOp::Bswap, Expr::trunc(W16, r(1))), Ty::Bv(W16)),
            (Expr::cmp(Ne, r(3), Expr::c64(0)), Ty::Bool),
        ];
        for (e, ty) in cases {
            assert_eq!(e.ty(), Ok(ty), "{e:?}");
        }
    }

    #[test]
    fn ill_typed() {
        let cases = [
            r(11),
            Expr::Const {
                value: 0x100,
                w: W8,
            },
            Expr::bin(Add, r(1), Expr::c32(1)),
            Expr::bin(And, Expr::cmp(Eq, r(1), r(2)), r(1)),
            Expr::cmp(Ult, Expr::lo32(r(1)), r(2)),
            Expr::ext(Zext, W32, r(1)),
            Expr::ext(Sext, W32, r(1)),
            Expr::trunc(W64, Expr::lo32(r(1))),
            Expr::un(UnOp::Bswap, Expr::trunc(W8, r(1))),
            Expr::zext64(Expr::cmp(Eq, r(1), r(2))),
        ];
        for e in cases {
            assert!(e.ty().is_err(), "{e:?}");
        }
    }

    fn module(blocks: Vec<Block>) -> Module {
        Module {
            format_version: FORMAT_VERSION,
            sbpf_version: 3,
            entrypoint: 0,
            text_vaddr: 0,
            ro_region: RoRegion {
                vaddr: 0,
                data: vec![],
            },
            address_taken: vec![],
            functions: vec![Function {
                entry: 0,
                name: "f".into(),
                blocks,
            }],
        }
    }

    fn block(start: usize, statements: Vec<Stmt>, exit: Exit) -> Block {
        Block {
            start,
            noreturn: false,
            statements,
            exit,
        }
    }

    #[test]
    fn validate() {
        let ret = |pc| Exit::Return { pc };
        let ok = module(vec![
            block(
                0,
                vec![Stmt::Check {
                    pc: 0,
                    cond: Expr::cmp(Ne, r(1), Expr::c64(0)),
                    error: Fault::DivideByZero,
                }],
                Exit::Next {
                    target: 1,
                    pc: None,
                },
            ),
            block(1, vec![], ret(1)),
        ]);
        assert_eq!(ok.validate(), Ok(()));

        let bad = [
            module(vec![block(0, vec![], ret(0)), block(0, vec![], ret(0))]),
            module(vec![block(
                0,
                vec![],
                Exit::Next {
                    target: 5,
                    pc: None,
                },
            )]),
            module(vec![block(
                0,
                vec![],
                Exit::Branch {
                    pc: 0,
                    cond: r(1),
                    then: 0,
                    r#else: 0,
                },
            )]),
            module(vec![block(
                0,
                vec![Stmt::Assign {
                    pc: 0,
                    dst: 10,
                    value: r(1),
                }],
                ret(0),
            )]),
            module(vec![block(
                0,
                vec![Stmt::Store {
                    pc: 0,
                    w: W32,
                    addr: r(1),
                    value: r(2),
                }],
                ret(0),
            )]),
            module(vec![block(
                0,
                vec![],
                Exit::Call {
                    pc: 0,
                    target: 7,
                    next: None,
                },
            )]),
            Module {
                address_taken: vec![3],
                ..module(vec![block(0, vec![], ret(0))])
            },
            Module {
                sbpf_version: 2,
                ..module(vec![block(0, vec![], ret(0))])
            },
            Module {
                format_version: 2,
                ..module(vec![block(0, vec![], ret(0))])
            },
            module(vec![block(
                0,
                vec![],
                Exit::Callx {
                    pc: 0,
                    reg: 10,
                    next: None,
                },
            )]),
        ];
        for m in bad {
            assert!(m.validate().is_err(), "{m:?}");
        }
    }

    #[test]
    fn constant_masks_to_width() {
        assert_eq!(
            Expr::constant(0xffff_ffff_ffff_fff0, W8),
            Expr::Const { value: 0xf0, w: W8 }
        );
    }
}
