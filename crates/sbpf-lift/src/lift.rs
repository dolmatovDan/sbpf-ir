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
        ebpf::BPF_ALU64_STORE => alu(insn, Width::W64),
        ebpf::BPF_ALU32_LOAD => alu(insn, Width::W32),
        _ => Err(unsupported(insn)),
    }
}

fn unsupported(insn: &Insn) -> LiftError {
    LiftError::UnsupportedOpcode {
        pc: insn.ptr,
        opc: insn.opc,
    }
}

fn alu(insn: &Insn, w: Width) -> Result<Vec<Stmt>, LiftError> {
    let pc = insn.ptr;
    let d = || Expr::reg(insn.dst);
    let value = match (insn.opc, insn.imm) {
        (ebpf::LE, 16) => Expr::zext64(Expr::trunc(Width::W16, d())),
        (ebpf::LE, 32) => Expr::zext64(Expr::lo32(d())),
        (ebpf::LE, 64) => d(),
        (ebpf::BE, 16) => Expr::zext64(Expr::un(UnOp::Bswap, Expr::trunc(Width::W16, d()))),
        (ebpf::BE, 32) => Expr::zext64(Expr::un(UnOp::Bswap, Expr::lo32(d()))),
        (ebpf::BE, 64) => Expr::un(UnOp::Bswap, d()),
        (ebpf::LE | ebpf::BE, _) => return Err(unsupported(insn)),
        _ => return arith(insn, w),
    };
    Ok(vec![Stmt::Assign {
        pc,
        dst: insn.dst,
        value,
    }])
}

fn arith(insn: &Insn, w: Width) -> Result<Vec<Stmt>, LiftError> {
    let pc = insn.ptr;
    let wide = w == Width::W64;
    let narrow = |e| if wide { e } else { Expr::lo32(e) };
    let dst = narrow(Expr::reg(insn.dst));
    let reg_src = insn.opc & ebpf::BPF_X != 0;
    let src = if reg_src {
        narrow(Expr::reg(insn.src))
    } else {
        // `imm` уже знаково расширен до i64 при декодировании.
        Expr::constant(insn.imm as u64, w)
    };

    let mut stmts = Vec::new();
    let mut sign_extend = false;
    let result = match insn.opc & ebpf::BPF_ALU_OP_MASK {
        ebpf::BPF_NEG if !reg_src => Expr::un(UnOp::Neg, dst),
        ebpf::BPF_MOV => src,
        op => {
            let (op, src) = match op {
                ebpf::BPF_ADD | ebpf::BPF_SUB | ebpf::BPF_MUL => {
                    // В v0/v3 результат add32/sub32/mul32 расширяется знаком.
                    sign_extend = true;
                    let op = match op {
                        ebpf::BPF_ADD => BinOp::Add,
                        ebpf::BPF_SUB => BinOp::Sub,
                        _ => BinOp::Mul,
                    };
                    (op, src)
                }
                ebpf::BPF_OR => (BinOp::Or, src),
                ebpf::BPF_AND => (BinOp::And, src),
                ebpf::BPF_XOR => (BinOp::Xor, src),
                ebpf::BPF_DIV | ebpf::BPF_MOD => {
                    if reg_src {
                        stmts.push(Stmt::Check {
                            pc,
                            cond: Expr::cmp(CmpOp::Ne, src.clone(), Expr::constant(0, w)),
                            error: Fault::DivideByZero,
                        });
                    }
                    let op = if op == ebpf::BPF_DIV {
                        BinOp::Udiv
                    } else {
                        BinOp::Urem
                    };
                    (op, src)
                }
                ebpf::BPF_LSH | ebpf::BPF_RSH | ebpf::BPF_ARSH => {
                    let op = match op {
                        ebpf::BPF_LSH => BinOp::Shl,
                        ebpf::BPF_RSH => BinOp::Lshr,
                        _ => BinOp::Ashr,
                    };
                    // Константу сдвига меньше `w` гарантирует верификатор, регистр маскирует VM.
                    let src = if reg_src {
                        let mask = Expr::constant((w.bits() - 1).into(), w);
                        Expr::bin(BinOp::And, src, mask)
                    } else {
                        src
                    };
                    (op, src)
                }
                _ => return Err(unsupported(insn)),
            };
            Expr::bin(op, dst, src)
        }
    };
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
    fn well_typed_and_rejects_v2() {
        for opc in 0..=u8::MAX {
            let cls = opc & ebpf::BPF_CLS_MASK;
            if cls != ebpf::BPF_ALU64_STORE && cls != ebpf::BPF_ALU32_LOAD {
                continue;
            }
            for imm in [16, 32, 64] {
                if let Ok(stmts) = lift_stmts(&insn(opc, 1, 2, 0, imm)) {
                    for stmt in stmts {
                        let (Stmt::Assign { value: e, .. } | Stmt::Check { cond: e, .. }) = &stmt
                        else {
                            panic!("{stmt:?}");
                        };
                        e.ty().unwrap_or_else(|err| panic!("{opc:#04x}: {err}"));
                    }
                }
            }
        }
        for opc in [ebpf::HOR64_IMM, ebpf::UHMUL64_IMM, ebpf::SDIV64_REG] {
            assert!(lift_stmts(&insn(opc, 1, 2, 0, 0)).is_err(), "{opc:#04x}");
        }
        assert!(matches!(
            lift_stmts(&insn(ebpf::LE, 1, 0, 0, 8)),
            Err(LiftError::UnsupportedOpcode { pc: 7, .. })
        ));
    }
}
