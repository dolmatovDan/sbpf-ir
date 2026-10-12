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
            Expr::Reg(r) => map.serialize_entry("reg", r)?,
            Expr::Const { value, w } => {
                map.serialize_entry("const", &format!("{value:#x}"))?;
                map.serialize_entry("w", w)?;
            }
            Expr::Bin(op, a, b) => {
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("args", &[a, b])?;
            }
            Expr::Cmp(op, a, b) => {
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("args", &[a, b])?;
            }
            Expr::Un(op, a) => {
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("args", &[a])?;
            }
            Expr::Ext(op, w, a) => {
                map.serialize_entry("op", op.name())?;
                map.serialize_entry("w", w)?;
                map.serialize_entry("args", &[a])?;
            }
        }
        map.end()
    }
}

#[derive(Deserialize)]
struct RawExpr {
    reg: Option<u8>,
    #[serde(rename = "const", default, with = "opt_hex")]
    value: Option<u64>,
    w: Option<Width>,
    op: Option<String>,
    args: Option<Vec<Expr>>,
}

mod opt_hex {
    use super::*;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
        hex::deserialize(d).map(Some)
    }
}

impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = RawExpr::deserialize(d)?;
        match raw {
            RawExpr { reg: Some(r), .. } => Ok(Expr::Reg(r)),
            RawExpr {
                value: Some(value),
                w: Some(w),
                ..
            } => Ok(Expr::Const { value, w }),
            RawExpr {
                op: Some(op),
                args: Some(args),
                w,
                ..
            } => op_expr(&op, w, args).map_err(D::Error::custom),
            _ => Err(D::Error::custom("ожидалось reg, const+w или op+args")),
        }
    }
}

fn op_expr(name: &str, w: Option<Width>, args: Vec<Expr>) -> Result<Expr, String> {
    let n = args.len();
    let arity = |_| format!("{name}: неверное число аргументов {n}");
    if let Some(op) = by_name(&BinOp::ALL, name, BinOp::name) {
        let [a, b] = <[Expr; 2]>::try_from(args).map_err(arity)?;
        Ok(Expr::bin(op, a, b))
    } else if let Some(op) = by_name(&CmpOp::ALL, name, CmpOp::name) {
        let [a, b] = <[Expr; 2]>::try_from(args).map_err(arity)?;
        Ok(Expr::cmp(op, a, b))
    } else if let Some(op) = by_name(&UnOp::ALL, name, UnOp::name) {
        let [a] = <[Expr; 1]>::try_from(args).map_err(arity)?;
        Ok(Expr::un(op, a))
    } else if let Some(op) = by_name(&ExtOp::ALL, name, ExtOp::name) {
        let [a] = <[Expr; 1]>::try_from(args).map_err(arity)?;
        let w = w.ok_or_else(|| format!("{name}: нет поля w"))?;
        Ok(Expr::ext(op, w, a))
    } else {
        Err(format!("неизвестная операция {name:?}"))
    }
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
        roundtrip(&r(3), json!({"reg": 3}));
        roundtrip(
            &Expr::constant(u64::MAX, Width::W64),
            json!({"const": "0xffffffffffffffff", "w": 64}),
        );
        roundtrip(&Expr::c32(5), json!({"const": "0x5", "w": 32}));
        roundtrip(
            &Expr::bin(BinOp::Lshr, r(1), r(2)),
            json!({"op": "lshr", "args": [{"reg": 1}, {"reg": 2}]}),
        );
        roundtrip(
            &Expr::cmp(CmpOp::Sge, r(1), r(2)),
            json!({"op": "sge", "args": [{"reg": 1}, {"reg": 2}]}),
        );
        roundtrip(
            &Expr::un(UnOp::Bswap, r(1)),
            json!({"op": "bswap", "args": [{"reg": 1}]}),
        );
        roundtrip(
            &Expr::sext64(Expr::lo32(r(1))),
            json!({"op": "sext", "w": 64, "args": [
                {"op": "trunc", "w": 32, "args": [{"reg": 1}]}
            ]}),
        );
    }

    #[test]
    fn bad_expr() {
        for bad in [
            json!({}),
            json!({"const": "5", "w": 64}),
            json!({"const": "0x5", "w": 7}),
            json!({"const": "0x5"}),
            json!({"op": "add", "args": [{"reg": 1}]}),
            json!({"op": "zext", "args": [{"reg": 1}]}),
            json!({"op": "sdiv", "args": [{"reg": 1}, {"reg": 2}]}),
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
                {"op": "eq", "args": [{"reg": 3}, {"const": "0x0", "w": 64}]}}),
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
            { "pc": 85, "op": "check", "error": "DivideByZero",
              "cond": { "op": "ne", "args": [{ "reg": 3 }, { "const": "0x0", "w": 64 }] } },
            { "pc": 85, "op": "assign", "dst": 2,
              "value": { "op": "udiv", "args": [{ "reg": 2 }, { "reg": 3 }] } },
            { "pc": 86, "op": "store", "w": 64,
              "addr": { "op": "add", "args": [{ "reg": 1 }, { "const": "0x8", "w": 64 }] },
              "value": { "reg": 2 } },
            { "pc": 87, "op": "assign", "dst": 2, "value": { "const": "0x0", "w": 64 } }
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
