//! Экспорт CFG в Graphviz DOT: кластер на функцию, сплошные рёбра — переходы,
//! пунктирные — вызовы (только к функциям, попавшим в вывод).

use std::{collections::BTreeSet, io};

use crate::{CallTarget, Function};

pub fn write_dot<'a>(
    functions: impl IntoIterator<Item = &'a Function>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    let functions: Vec<_> = functions.into_iter().collect();
    let entries: BTreeSet<_> = functions.iter().map(|f| f.entry).collect();
    let blocks: BTreeSet<_> = functions
        .iter()
        .flat_map(|f| &f.blocks)
        .map(|b| b.start)
        .collect();
    let mut external = BTreeSet::new();
    writeln!(w, "digraph cfg {{")?;
    writeln!(w, "  node [shape=plaintext, fontname=\"Courier New\"];")?;
    writeln!(w, "  compound=true;")?;
    let mut calls = BTreeSet::new();
    for function in functions {
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
                if let Some(CallTarget::Internal { pc }) = insn.call
                    && entries.contains(&pc)
                {
                    calls.insert((block.start, pc));
                }
            }
            writeln!(w, "</table>>];")?;
        }
        writeln!(w, "  }}")?;
        for block in &function.blocks {
            // У noreturn-блока единственное ребро — возврат, который по анализу невозможен.
            let style = if block.noreturn {
                " [style=dashed, color=red]"
            } else {
                ""
            };
            for succ in &block.successors {
                if !blocks.contains(succ) {
                    external.insert(*succ);
                }
                writeln!(w, "  lbb_{} -> lbb_{succ}{style};", block.start)?;
            }
        }
    }
    // Переходы в функции, не попавшие в вывод.
    for pc in external {
        writeln!(w, "  lbb_{pc} [label=\"→ lbb_{pc}\", shape=note];")?;
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
