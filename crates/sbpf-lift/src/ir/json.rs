//! JSON-представление IR: выражения вручную, 64-битные значения строками `"0x…"`.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::SerializeMap};

use super::{BinOp, CmpOp, Expr, ExtOp, UnOp, Width};

impl Serialize for Width {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for Width {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bits = u32::deserialize(d)?;
        Width::from_bits(bits).ok_or_else(|| D::Error::custom(format!("ширина {bits}")))
    }
}

pub mod hex {
    use super::*;

    pub fn serialize<S: Serializer>(value: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{value:#x}"))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        let text = String::deserialize(d)?;
        text.strip_prefix("0x")
            .filter(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_hexdigit()))
            .and_then(|digits| u64::from_str_radix(digits, 16).ok())
            .ok_or_else(|| D::Error::custom(format!("ожидалось 0x…, получено {text:?}")))
    }
}

pub mod base64 {
    use ::base64::{Engine, engine::general_purpose::STANDARD};

    use super::*;

    pub fn serialize<S: Serializer>(data: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&STANDARD.encode(data))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        STANDARD
            .decode(String::deserialize(d)?)
            .map_err(D::Error::custom)
    }
}

impl Serialize for Expr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;
        match self {
            Expr::Reg(r) => {
                map.serialize_entry("kind", "reg")?;
                map.serialize_entry("reg", r)?;
            }
            Expr::Const { value, w } => {
                map.serialize_entry("kind", "const")?;
                map.serialize_entry("value", &format!("{value:#x}"))?;
                map.serialize_entry("w", w)?;
            }
            Expr::Bin(op, a, b) => {
                map.serialize_entry("kind", "bin")?;
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("a", a)?;
                map.serialize_entry("b", b)?;
            }
            Expr::Cmp(op, a, b) => {
                map.serialize_entry("kind", "cmp")?;
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("a", a)?;
                map.serialize_entry("b", b)?;
            }
            Expr::Un(op, a) => {
                map.serialize_entry("kind", "un")?;
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("a", a)?;
            }
            Expr::Ext(op, w, a) => {
                map.serialize_entry("kind", "ext")?;
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("w", w)?;
                map.serialize_entry("a", a)?;
            }
        }
        map.end()
    }
}

/// `kind` задаёт поля узла, `op` — операцию внутри вида.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum RawExpr {
    Reg {
        reg: u8,
    },
    Const {
        #[serde(with = "hex")]
        value: u64,
        w: Width,
    },
    Bin {
        op: String,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Un {
        op: String,
        a: Box<Expr>,
    },
    Ext {
        op: String,
        w: Width,
        a: Box<Expr>,
    },
    Cmp {
        op: String,
        a: Box<Expr>,
        b: Box<Expr>,
    },
}

impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(match RawExpr::deserialize(d)? {
            RawExpr::Reg { reg } => Expr::Reg(reg),
            RawExpr::Const { value, w } => Expr::Const { value, w },
            RawExpr::Bin { op, a, b } => Expr::Bin(parse_op(BinOp::ALL, &op, BinOp::name)?, a, b),
            RawExpr::Un { op, a } => Expr::Un(parse_op(UnOp::ALL, &op, UnOp::name)?, a),
            RawExpr::Ext { op, w, a } => Expr::Ext(parse_op(ExtOp::ALL, &op, ExtOp::name)?, w, a),
            RawExpr::Cmp { op, a, b } => Expr::Cmp(parse_op(CmpOp::ALL, &op, CmpOp::name)?, a, b),
        })
    }
}

/// Имя ищется только среди операций своего вида: `{"kind": "bin", "op": "eq"}` — ошибка.
fn parse_op<T: Copy, E: serde::de::Error>(
    all: &[T],
    name: &str,
    op_name: fn(T) -> &'static str,
) -> Result<T, E> {
    by_name(all, name, op_name).ok_or_else(|| E::custom(format!("неизвестная операция {name:?}")))
}

fn by_name<T: Copy>(all: &[T], name: &str, op_name: fn(T) -> &'static str) -> Option<T> {
    all.iter().copied().find(|&op| op_name(op) == name)
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::super::*;

    fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        value: &T,
        expected: Value,
    ) {
        assert_eq!(serde_json::to_value(value).unwrap(), expected);
        assert_eq!(&serde_json::from_value::<T>(expected).unwrap(), value);
    }

    #[test]
    fn expr() {
        let r = Expr::reg;
        roundtrip(&r(3), json!({"kind": "reg", "reg": 3}));
        roundtrip(
            &Expr::constant(u64::MAX, Width::W64),
            json!({"kind": "const", "value": "0xffffffffffffffff", "w": 64}),
        );
        roundtrip(
            &Expr::c32(5),
            json!({"kind": "const", "value": "0x5", "w": 32}),
        );
        roundtrip(
            &Expr::bin(BinOp::Lshr, r(1), r(2)),
            json!({"kind": "bin", "op": "lshr",
                "a": {"kind": "reg", "reg": 1}, "b": {"kind": "reg", "reg": 2}}),
        );
        roundtrip(
            &Expr::cmp(CmpOp::Sge, r(1), r(2)),
            json!({"kind": "cmp", "op": "sge",
                "a": {"kind": "reg", "reg": 1}, "b": {"kind": "reg", "reg": 2}}),
        );
        roundtrip(
            &Expr::un(UnOp::Bswap, r(1)),
            json!({"kind": "un", "op": "bswap", "a": {"kind": "reg", "reg": 1}}),
        );
        roundtrip(
            &Expr::sext64(Expr::lo32(r(1))),
            json!({"kind": "ext", "op": "sext", "w": 64, "a":
                {"kind": "ext", "op": "trunc", "w": 32, "a": {"kind": "reg", "reg": 1}}
            }),
        );
    }

    #[test]
    fn every_op() {
        let names: Vec<&str> = (BinOp::ALL.iter().map(|op| op.name()))
            .chain(CmpOp::ALL.iter().map(|op| op.name()))
            .chain(UnOp::ALL.iter().map(|op| op.name()))
            .chain(ExtOp::ALL.iter().map(|op| op.name()))
            .collect();
        let unique: std::collections::BTreeSet<_> = names.iter().collect();
        assert_eq!(unique.len(), names.len());

        let (a, b) = (|| Expr::reg(1), || Expr::reg(2));
        let exprs = (BinOp::ALL.iter().map(|&op| Expr::bin(op, a(), b())))
            .chain(CmpOp::ALL.iter().map(|&op| Expr::cmp(op, a(), b())))
            .chain(UnOp::ALL.iter().map(|&op| Expr::un(op, a())))
            .chain(ExtOp::ALL.iter().map(|&op| Expr::ext(op, Width::W64, a())));
        for e in exprs {
            let json = serde_json::to_value(&e).unwrap();
            assert_eq!(serde_json::from_value::<Expr>(json).unwrap(), e);
        }
    }

    #[test]
    fn bad_expr() {
        for bad in [
            json!({}),
            json!({"reg": 1}),
            json!({"kind": "vec", "reg": 1}),
            json!({"kind": "reg"}),
            json!({"kind": "const", "value": "5", "w": 64}),
            json!({"kind": "const", "value": "0x5", "w": 7}),
            json!({"kind": "const", "value": "0x5"}),
            json!({"kind": "const", "value": "0x+5", "w": 64}),
            json!({"kind": "const", "value": "0x", "w": 64}),
            json!({"kind": "bin", "op": "add", "a": {"kind": "reg", "reg": 1}}),
            json!({"kind": "ext", "op": "zext", "a": {"kind": "reg", "reg": 1}}),
            json!({"kind": "bin", "op": "sdiv",
                "a": {"kind": "reg", "reg": 1}, "b": {"kind": "reg", "reg": 2}}),
            // Операция чужого вида.
            json!({"kind": "bin", "op": "eq",
                "a": {"kind": "reg", "reg": 1}, "b": {"kind": "reg", "reg": 2}}),
            json!({"kind": "un", "op": "zext", "a": {"kind": "reg", "reg": 1}}),
        ] {
            assert!(
                serde_json::from_value::<Expr>(bad.clone()).is_err(),
                "{bad}"
            );
        }
    }

    #[test]
    fn exits() {
        roundtrip(
            &Exit::Next {
                target: 88,
                pc: None,
            },
            json!({"kind": "next", "target": 88}),
        );
        roundtrip(
            &Exit::Next {
                target: 88,
                pc: Some(92),
            },
            json!({"kind": "next", "target": 88, "pc": 92}),
        );
        roundtrip(
            &Exit::Branch {
                pc: 84,
                cond: Expr::cmp(CmpOp::Eq, Expr::reg(3), Expr::c64(0)),
                then: 90,
                r#else: 85,
            },
            json!({"kind": "branch", "pc": 84, "then": 90, "else": 85, "cond":
                {"kind": "cmp", "op": "eq", "a": {"kind": "reg", "reg": 3},
                    "b": {"kind": "const", "value": "0x0", "w": 64}}}),
        );
        roundtrip(
            &Exit::Call {
                pc: 7,
                target: 84,
                next: None,
            },
            json!({"kind": "call", "pc": 7, "target": 84, "next": null}),
        );
        roundtrip(
            &Exit::Syscall {
                pc: 7,
                hash: 0x207559bd,
                name: Some("sol_log_".into()),
                next: Some(8),
            },
            json!({"kind": "syscall", "pc": 7, "hash": 0x207559bd, "name": "sol_log_", "next": 8}),
        );
        roundtrip(
            &Exit::Trap {
                pc: 9,
                error: Fault::ExecutionOverrun,
            },
            json!({"kind": "trap", "pc": 9, "error": "ExecutionOverrun"}),
        );
    }

    /// Блок `bb_85` из примера в `docs/ir.md`.
    #[test]
    fn doc_example_block() {
        let doc = json!({
          "start": 85,
          "noreturn": false,
          "statements": [
            { "kind": "check", "pc": 85, "error": "DivideByZero",
              "cond": { "kind": "cmp", "op": "ne",
                "a": { "kind": "reg", "reg": 3 }, "b": { "kind": "const", "value": "0x0", "w": 64 } } },
            { "kind": "assign", "pc": 85, "dst": 2,
              "value": { "kind": "bin", "op": "udiv",
                "a": { "kind": "reg", "reg": 2 }, "b": { "kind": "reg", "reg": 3 } } },
            { "kind": "store", "pc": 86, "w": 64,
              "addr": { "kind": "bin", "op": "add",
                "a": { "kind": "reg", "reg": 1 }, "b": { "kind": "const", "value": "0x8", "w": 64 } },
              "value": { "kind": "reg", "reg": 2 } },
            { "kind": "assign", "pc": 87, "dst": 2, "value": { "kind": "const", "value": "0x0", "w": 64 } }
          ],
          "exit": { "kind": "next", "target": 88 }
        });
        let block: Block = serde_json::from_value(doc.clone()).unwrap();
        assert_eq!(serde_json::to_value(&block).unwrap(), doc);
    }

    #[test]
    fn module_header() {
        let module = Module {
            format_version: FORMAT_VERSION,
            sbpf_version: 3,
            entrypoint: 233,
            text_vaddr: 0x1_0000_0000,
            ro_region: RoRegion {
                vaddr: 0,
                data: b"sbpf".to_vec(),
            },
            address_taken: vec![76, 1891],
            functions: vec![],
        };
        roundtrip(
            &module,
            json!({
                "format_version": 1,
                "sbpf_version": 3,
                "entrypoint": 233,
                "text_vaddr": "0x100000000",
                "ro_region": { "vaddr": "0x0", "data": "c2JwZg==" },
                "address_taken": [76, 1891],
                "functions": []
            }),
        );
    }
}
