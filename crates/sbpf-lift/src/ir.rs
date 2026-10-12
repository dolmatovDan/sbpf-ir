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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Udiv,
    Urem,
    Shl,
    Lshr,
    Ashr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Bswap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtOp {
    Zext,
    Sext,
    Trunc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Ult,
    Ule,
    Ugt,
    Uge,
    Slt,
    Sle,
    Sgt,
    Sge,
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

impl BinOp {
    pub const ALL: [Self; 11] = [
        BinOp::Add,
        BinOp::Sub,
        BinOp::Mul,
        BinOp::And,
        BinOp::Or,
        BinOp::Xor,
        BinOp::Udiv,
        BinOp::Urem,
        BinOp::Shl,
        BinOp::Lshr,
        BinOp::Ashr,
    ];

    pub fn name(self) -> &'static str {
        match self {
            BinOp::Add => "add",
            BinOp::Sub => "sub",
            BinOp::Mul => "mul",
            BinOp::And => "and",
            BinOp::Or => "or",
            BinOp::Xor => "xor",
            BinOp::Udiv => "udiv",
            BinOp::Urem => "urem",
            BinOp::Shl => "shl",
            BinOp::Lshr => "lshr",
            BinOp::Ashr => "ashr",
        }
    }
}

impl UnOp {
    pub const ALL: [Self; 2] = [UnOp::Neg, UnOp::Bswap];

    pub fn name(self) -> &'static str {
        match self {
            UnOp::Neg => "neg",
            UnOp::Bswap => "bswap",
        }
    }
}

impl ExtOp {
    pub const ALL: [Self; 3] = [ExtOp::Zext, ExtOp::Sext, ExtOp::Trunc];

    pub fn name(self) -> &'static str {
        match self {
            ExtOp::Zext => "zext",
            ExtOp::Sext => "sext",
            ExtOp::Trunc => "trunc",
        }
    }
}

impl CmpOp {
    pub const ALL: [Self; 10] = [
        CmpOp::Eq,
        CmpOp::Ne,
        CmpOp::Ult,
        CmpOp::Ule,
        CmpOp::Ugt,
        CmpOp::Uge,
        CmpOp::Slt,
        CmpOp::Sle,
        CmpOp::Sgt,
        CmpOp::Sge,
    ];

    pub fn name(self) -> &'static str {
        match self {
            CmpOp::Eq => "eq",
            CmpOp::Ne => "ne",
            CmpOp::Ult => "ult",
            CmpOp::Ule => "ule",
            CmpOp::Ugt => "ugt",
            CmpOp::Uge => "uge",
            CmpOp::Slt => "slt",
            CmpOp::Sle => "sle",
            CmpOp::Sgt => "sgt",
            CmpOp::Sge => "sge",
        }
    }
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
#[serde(tag = "op", rename_all = "snake_case")]
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

    #[test]
    fn constant_masks_to_width() {
        assert_eq!(
            Expr::constant(0xffff_ffff_ffff_fff0, W8),
            Expr::Const { value: 0xf0, w: W8 }
        );
    }
}
