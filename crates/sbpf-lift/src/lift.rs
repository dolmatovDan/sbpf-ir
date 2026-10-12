//! Лифтер: инструкции sBPF → IR по таблицам «Перевод инструкций» из `docs/ir.md`.

use solana_sbpf::ebpf::{self, Insn};

use crate::ir::{BinOp, CmpOp, Expr, Fault, Stmt, UnOp, Width};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LiftError {
    #[error("pc {pc}: неподдерживаемый опкод {opc:#04x}")]
    UnsupportedOpcode { pc: usize, opc: u8 },
}

/// Statement'ы инструкции, которая не передаёт управление.
fn lift_stmts(insn: &Insn) -> Result<Vec<Stmt>, LiftError> {
    match insn.opc & ebpf::BPF_CLS_MASK {
        _ if matches!(insn.opc, ebpf::LE | ebpf::BE) => byte_order(insn),
        ebpf::BPF_ALU64_STORE => alu(insn, Width::W64),
        ebpf::BPF_ALU32_LOAD => alu(insn, Width::W32),
        ebpf::BPF_LD | ebpf::BPF_LDX | ebpf::BPF_ST | ebpf::BPF_STX => memory(insn),
        _ => Err(unsupported(insn)),
    }
}

fn unsupported(insn: &Insn) -> LiftError {
    LiftError::UnsupportedOpcode {
        pc: insn.ptr,
        opc: insn.opc,
    }
}

fn memory(insn: &Insn) -> Result<Vec<Stmt>, LiftError> {
    let pc = insn.ptr;
    if insn.opc == ebpf::LD_DW_IMM {
        return Ok(vec![Stmt::Assign {
            pc,
            dst: insn.dst,
            value: Expr::c64(insn.imm as u64),
        }]);
    }
    let addr = |base| {
        Expr::bin(
            BinOp::Add,
            Expr::reg(base),
            Expr::c64(insn.off as i64 as u64),
        )
    };
    let stx = |w| Stmt::Store {
        pc,
        w,
        addr: addr(insn.dst),
        value: if w == Width::W64 {
            Expr::reg(insn.src)
        } else {
            Expr::trunc(w, Expr::reg(insn.src))
        },
    };
    let st = |w| Stmt::Store {
        pc,
        w,
        addr: addr(insn.dst),
        value: Expr::constant(insn.imm as u64, w),
    };
    let ldx = |w| Stmt::Load {
        pc,
        dst: insn.dst,
        w,
        addr: addr(insn.src),
    };
    let stmt = match insn.opc {
        ebpf::LD_B_REG => ldx(Width::W8),
        ebpf::LD_H_REG => ldx(Width::W16),
        ebpf::LD_W_REG => ldx(Width::W32),
        ebpf::LD_DW_REG => ldx(Width::W64),
        ebpf::ST_B_IMM => st(Width::W8),
        ebpf::ST_H_IMM => st(Width::W16),
        ebpf::ST_W_IMM => st(Width::W32),
        ebpf::ST_DW_IMM => st(Width::W64),
        ebpf::ST_B_REG => stx(Width::W8),
        ebpf::ST_H_REG => stx(Width::W16),
        ebpf::ST_W_REG => stx(Width::W32),
        ebpf::ST_DW_REG => stx(Width::W64),
        _ => return Err(unsupported(insn)),
    };
    Ok(vec![stmt])
}

fn byte_order(insn: &Insn) -> Result<Vec<Stmt>, LiftError> {
    let d = || Expr::reg(insn.dst);
    let swap = insn.opc == ebpf::BE;
    let value = match (insn.imm, swap) {
        (16, false) => Expr::zext64(Expr::trunc(Width::W16, d())),
        (32, false) => Expr::zext64(Expr::lo32(d())),
        (64, false) => d(),
        (16, true) => Expr::zext64(Expr::un(UnOp::Bswap, Expr::trunc(Width::W16, d()))),
        (32, true) => Expr::zext64(Expr::un(UnOp::Bswap, Expr::lo32(d()))),
        (64, true) => Expr::un(UnOp::Bswap, d()),
        _ => return Err(unsupported(insn)),
    };
    Ok(vec![Stmt::Assign {
        pc: insn.ptr,
        dst: insn.dst,
        value,
    }])
}

fn alu(insn: &Insn, w: Width) -> Result<Vec<Stmt>, LiftError> {
    let pc = insn.ptr;
    let wide = w == Width::W64;
    let narrow = |e| if wide { e } else { Expr::lo32(e) };
    let dst = narrow(Expr::reg(insn.dst));
    let reg_src = insn.opc & ebpf::BPF_X != 0;
    let mut src = if reg_src {
        narrow(Expr::reg(insn.src))
    } else {
        // `imm` уже знаково расширен до i64 при декодировании.
        Expr::constant(insn.imm as u64, w)
    };

    let mut stmts = Vec::new();
    let result = match insn.opc & ebpf::BPF_ALU_OP_MASK {
        ebpf::BPF_NEG if !reg_src => Expr::un(UnOp::Neg, dst),
        ebpf::BPF_MOV => src,
        op => {
            let op = match op {
                ebpf::BPF_ADD => BinOp::Add,
                ebpf::BPF_SUB => BinOp::Sub,
                ebpf::BPF_MUL => BinOp::Mul,
                ebpf::BPF_OR => BinOp::Or,
                ebpf::BPF_AND => BinOp::And,
                ebpf::BPF_XOR => BinOp::Xor,
                ebpf::BPF_DIV => BinOp::Udiv,
                ebpf::BPF_MOD => BinOp::Urem,
                ebpf::BPF_LSH => BinOp::Shl,
                ebpf::BPF_RSH => BinOp::Lshr,
                ebpf::BPF_ARSH => BinOp::Ashr,
                _ => return Err(unsupported(insn)),
            };
            if reg_src && matches!(op, BinOp::Udiv | BinOp::Urem) {
                stmts.push(Stmt::Check {
                    pc,
                    cond: Expr::cmp(CmpOp::Ne, src.clone(), Expr::constant(0, w)),
                    error: Fault::DivideByZero,
                });
            }
            // Константу сдвига меньше `w` гарантирует верификатор, регистр маскирует VM.
            if reg_src && matches!(op, BinOp::Shl | BinOp::Lshr | BinOp::Ashr) {
                src = Expr::bin(BinOp::And, src, Expr::constant((w.bits() - 1).into(), w));
            }
            Expr::bin(op, dst, src)
        }
    };
    // В v0/v3 результат add32/sub32/mul32 расширяется знаком, остальных — нулями.
    let sign_extend = matches!(
        insn.opc & ebpf::BPF_ALU_OP_MASK,
        ebpf::BPF_ADD | ebpf::BPF_SUB | ebpf::BPF_MUL
    );
    let value = match (wide, sign_extend) {
        (true, _) => result,
        (false, true) => Expr::sext64(result),
        (false, false) => Expr::zext64(result),
    };
    stmts.push(Stmt::Assign {
        pc,
        dst: insn.dst,
        value,
    });
    Ok(stmts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn insn(opc: u8, dst: u8, src: u8, off: i16, imm: i64) -> Insn {
        Insn {
            ptr: 7,
            opc,
            dst,
            src,
            off,
            imm,
        }
    }

    fn r(n: u8) -> Expr {
        Expr::reg(n)
    }

    fn assign(value: Expr) -> Vec<Stmt> {
        vec![Stmt::Assign {
            pc: 7,
            dst: 1,
            value,
        }]
    }

    fn check(insn: Insn, expected: Vec<Stmt>) {
        assert_eq!(lift_stmts(&insn), Ok(expected), "{insn:?}");
    }

    #[test]
    fn alu64() {
        use BinOp::*;
        check(
            insn(ebpf::ADD64_IMM, 1, 0, 0, -1),
            assign(Expr::bin(Add, r(1), Expr::c64(u64::MAX))),
        );
        check(
            insn(ebpf::SUB64_IMM, 1, 0, 0, 5),
            assign(Expr::bin(Sub, r(1), Expr::c64(5))),
        );
        check(
            insn(ebpf::SUB64_REG, 1, 2, 0, 0),
            assign(Expr::bin(Sub, r(1), r(2))),
        );
        check(
            insn(ebpf::AND64_IMM, 1, 0, 0, -16),
            assign(Expr::bin(And, r(1), Expr::c64(-16i64 as u64))),
        );
        check(
            insn(ebpf::MOV64_IMM, 1, 0, 0, -2),
            assign(Expr::c64(-2i64 as u64)),
        );
        check(insn(ebpf::MOV64_REG, 1, 2, 0, 0), assign(r(2)));
        check(
            insn(ebpf::NEG64, 1, 0, 0, 0),
            assign(Expr::un(UnOp::Neg, r(1))),
        );
        check(
            insn(ebpf::LSH64_IMM, 1, 0, 0, 3),
            assign(Expr::bin(Shl, r(1), Expr::c64(3))),
        );
        check(
            insn(ebpf::ARSH64_REG, 1, 2, 0, 0),
            assign(Expr::bin(Ashr, r(1), Expr::bin(And, r(2), Expr::c64(63)))),
        );
    }

    #[test]
    fn alu32() {
        use BinOp::*;
        check(
            insn(ebpf::ADD32_IMM, 1, 0, 0, -1),
            assign(Expr::sext64(Expr::bin(
                Add,
                Expr::lo32(r(1)),
                Expr::c32(u32::MAX),
            ))),
        );
        check(
            insn(ebpf::SUB32_IMM, 1, 0, 0, 5),
            assign(Expr::sext64(Expr::bin(Sub, Expr::lo32(r(1)), Expr::c32(5)))),
        );
        check(
            insn(ebpf::MUL32_REG, 1, 2, 0, 0),
            assign(Expr::sext64(Expr::bin(
                Mul,
                Expr::lo32(r(1)),
                Expr::lo32(r(2)),
            ))),
        );
        check(
            insn(ebpf::XOR32_IMM, 1, 0, 0, -1),
            assign(Expr::zext64(Expr::bin(
                Xor,
                Expr::lo32(r(1)),
                Expr::c32(u32::MAX),
            ))),
        );
        check(
            insn(ebpf::MOV32_IMM, 1, 0, 0, -1),
            assign(Expr::zext64(Expr::c32(u32::MAX))),
        );
        check(
            insn(ebpf::MOV32_REG, 1, 2, 0, 0),
            assign(Expr::zext64(Expr::lo32(r(2)))),
        );
        check(
            insn(ebpf::NEG32, 1, 0, 0, 0),
            assign(Expr::zext64(Expr::un(UnOp::Neg, Expr::lo32(r(1))))),
        );
        check(
            insn(ebpf::RSH32_IMM, 1, 0, 0, 31),
            assign(Expr::zext64(Expr::bin(
                Lshr,
                Expr::lo32(r(1)),
                Expr::c32(31),
            ))),
        );
        check(
            insn(ebpf::LSH32_REG, 1, 2, 0, 0),
            assign(Expr::zext64(Expr::bin(
                Shl,
                Expr::lo32(r(1)),
                Expr::bin(And, Expr::lo32(r(2)), Expr::c32(31)),
            ))),
        );
    }

    #[test]
    fn division() {
        use BinOp::*;
        check(
            insn(ebpf::DIV64_IMM, 1, 0, 0, -3),
            assign(Expr::bin(Udiv, r(1), Expr::c64(-3i64 as u64))),
        );
        check(
            insn(ebpf::MOD32_IMM, 1, 0, 0, 10),
            assign(Expr::zext64(Expr::bin(
                Urem,
                Expr::lo32(r(1)),
                Expr::c32(10),
            ))),
        );
        let check64 = Stmt::Check {
            pc: 7,
            cond: Expr::cmp(CmpOp::Ne, r(2), Expr::c64(0)),
            error: Fault::DivideByZero,
        };
        let mut expected = vec![check64];
        expected.extend(assign(Expr::bin(Udiv, r(1), r(2))));
        check(insn(ebpf::DIV64_REG, 1, 2, 0, 0), expected);
        let check32 = Stmt::Check {
            pc: 7,
            cond: Expr::cmp(CmpOp::Ne, Expr::lo32(r(2)), Expr::c32(0)),
            error: Fault::DivideByZero,
        };
        let mut expected = vec![check32];
        expected.extend(assign(Expr::zext64(Expr::bin(
            Urem,
            Expr::lo32(r(1)),
            Expr::lo32(r(2)),
        ))));
        check(insn(ebpf::MOD32_REG, 1, 2, 0, 0), expected);
    }

    #[test]
    fn byte_order() {
        let t16 = || Expr::trunc(Width::W16, r(1));
        check(insn(ebpf::LE, 1, 0, 0, 16), assign(Expr::zext64(t16())));
        check(
            insn(ebpf::LE, 1, 0, 0, 32),
            assign(Expr::zext64(Expr::lo32(r(1)))),
        );
        check(insn(ebpf::LE, 1, 0, 0, 64), assign(r(1)));
        check(
            insn(ebpf::BE, 1, 0, 0, 16),
            assign(Expr::zext64(Expr::un(UnOp::Bswap, t16()))),
        );
        check(
            insn(ebpf::BE, 1, 0, 0, 32),
            assign(Expr::zext64(Expr::un(UnOp::Bswap, Expr::lo32(r(1))))),
        );
        check(
            insn(ebpf::BE, 1, 0, 0, 64),
            assign(Expr::un(UnOp::Bswap, r(1))),
        );
    }

    #[test]
    fn memory() {
        let addr = |base, off: i64| Expr::bin(BinOp::Add, r(base), Expr::c64(off as u64));
        check(
            insn(ebpf::LD_DW_IMM, 1, 0, 0, 0x1234_5678_9abc_def0),
            assign(Expr::c64(0x1234_5678_9abc_def0)),
        );
        check(
            insn(ebpf::LD_B_REG, 1, 2, -8, 0),
            vec![Stmt::Load {
                pc: 7,
                dst: 1,
                w: Width::W8,
                addr: addr(2, -8),
            }],
        );
        check(
            insn(ebpf::LD_DW_REG, 1, 2, 16, 0),
            vec![Stmt::Load {
                pc: 7,
                dst: 1,
                w: Width::W64,
                addr: addr(2, 16),
            }],
        );
        check(
            insn(ebpf::ST_H_IMM, 1, 0, 2, -1),
            vec![Stmt::Store {
                pc: 7,
                w: Width::W16,
                addr: addr(1, 2),
                value: Expr::constant(0xffff, Width::W16),
            }],
        );
        check(
            insn(ebpf::ST_DW_IMM, 1, 0, 0, -1),
            vec![Stmt::Store {
                pc: 7,
                w: Width::W64,
                addr: addr(1, 0),
                value: Expr::c64(u64::MAX),
            }],
        );
        check(
            insn(ebpf::ST_W_REG, 1, 2, -4, 0),
            vec![Stmt::Store {
                pc: 7,
                w: Width::W32,
                addr: addr(1, -4),
                value: Expr::lo32(r(2)),
            }],
        );
        check(
            insn(ebpf::ST_DW_REG, 10, 2, -8, 0),
            vec![Stmt::Store {
                pc: 7,
                w: Width::W64,
                addr: addr(10, -8),
                value: r(2),
            }],
        );
    }

    #[test]
    fn well_typed_and_rejects_v2() {
        for opc in 0..=u8::MAX {
            let cls = opc & ebpf::BPF_CLS_MASK;
            if cls == ebpf::BPF_JMP64 || cls == ebpf::BPF_JMP32 {
                continue;
            }
            for imm in [16, 32, 64] {
                if let Ok(stmts) = lift_stmts(&insn(opc, 1, 2, 0, imm)) {
                    for stmt in stmts {
                        let exprs = match &stmt {
                            Stmt::Assign { value, .. } => vec![value],
                            Stmt::Check { cond, .. } => vec![cond],
                            Stmt::Load { addr, .. } => vec![addr],
                            Stmt::Store { addr, value, .. } => vec![addr, value],
                        };
                        for e in exprs {
                            e.ty().unwrap_or_else(|err| panic!("{opc:#04x}: {err}"));
                        }
                    }
                }
            }
        }
        for opc in [
            ebpf::HOR64_IMM,
            ebpf::UHMUL64_IMM,
            ebpf::SDIV64_REG,
            ebpf::BPF_LD | ebpf::BPF_ABS | ebpf::BPF_W,
            ebpf::BPF_LD | ebpf::BPF_IND | ebpf::BPF_B,
            ebpf::BPF_LD | ebpf::BPF_MEM | ebpf::BPF_W,
            ebpf::BPF_LD | ebpf::BPF_IMM | ebpf::BPF_W,
        ] {
            assert!(lift_stmts(&insn(opc, 1, 2, 0, 0)).is_err(), "{opc:#04x}");
        }
        assert!(matches!(
            lift_stmts(&insn(ebpf::LE, 1, 0, 0, 8)),
            Err(LiftError::UnsupportedOpcode { pc: 7, .. })
        ));
    }
}
