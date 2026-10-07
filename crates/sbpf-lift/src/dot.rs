//! Экспорт CFG в Graphviz DOT: кластер на функцию, сплошные рёбра — переходы,
//! пунктирные — вызовы.

use std::{collections::BTreeSet, io};

use crate::{CallTarget, Cfg};

pub fn write_dot(cfg: &Cfg, w: &mut impl io::Write) -> io::Result<()> {
    writeln!(w, "digraph cfg {{")?;
    writeln!(w, "  node [shape=plaintext, fontname=\"Courier New\"];")?;
    writeln!(w, "  compound=true;")?;
    let mut calls = BTreeSet::new();
    for function in &cfg.functions {
        writeln!(w, "  subgraph cluster_{} {{", function.entry)?;
        writeln!(w, "    label=<{}>;", escape(&function.name))?;
        for block in &function.blocks {
            write!(
                w,
                "    lbb_{} [label=<<table border=\"1\" cellborder=\"0\" cellspacing=\"0\">\
                 <tr><td align=\"left\"><b>lbb_{}</b></td></tr>",
                block.start, block.start
            )?;
            for insn in &block.instructions {
                write!(w, "<tr><td align=\"left\">{}</td></tr>", escape(&insn.text))?;
                if let Some(CallTarget::Internal { pc }) = insn.call {
                    calls.insert((block.start, pc));
                }
            }
            writeln!(w, "</table>>];")?;
        }
        writeln!(w, "  }}")?;
        for block in &function.blocks {
            for succ in &block.successors {
                writeln!(w, "  lbb_{} -> lbb_{succ};", block.start)?;
            }
        }
    }
    for (from, to) in calls {
        writeln!(
            w,
            "  lbb_{from} -> lbb_{to} [style=dashed, color=gray, lhead=cluster_{to}];"
        )?;
    }
    writeln!(w, "}}")
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
