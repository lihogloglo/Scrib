//! Self-documentation: every registered tunable as rows (for `--json` through serde) or a
//! Markdown table (`docs/tunables-index.md`). Sorted by name, so the output is stable whatever
//! the registration order. Spec: `docs/specs/tunables.md` "Self-documentation".

use std::fmt::Write as _;

use serde::Serialize;

use crate::meta::{Lifetime, Net, Scope, Unit};
use crate::store::{SetBy, Tunables};

/// One tunable, everything as text or plain data.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Row {
    pub name: String,
    /// The kind as the console shows it (`float 0..=500`, `on|off`, `low|high`).
    pub kind: String,
    pub default: String,
    /// The effective value.
    pub value: String,
    /// The layer it comes from (`default`, `mod`, `preset`, `config`, ...).
    pub set_by: &'static str,
    /// The note on that layer's value, if any.
    pub note: Option<String>,
    /// Every layer that holds a value above the default: `(layer, value)`, weakest first.
    pub layers: Vec<(&'static str, String)>,
    pub unit: Unit,
    /// `industry`, `measured`, `guess`, ...
    pub source: &'static str,
    pub source_detail: &'static str,
    /// The preset group driver (`sg.shadows`).
    pub group: Option<&'static str>,
    pub scope: Scope,
    pub net: Net,
    pub lifetime: Lifetime,
    pub help: String,
    /// `path/to/file.rs:line`, `/` separators.
    pub location: Option<String>,
}

impl Tunables {
    /// Every tunable as a [`Row`], sorted by name.
    pub fn rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = self
            .iter()
            .map(|(_, t)| {
                let meta = t.meta();
                Row {
                    name: t.name().to_owned(),
                    kind: t.kind().describe(),
                    default: t.default().to_string(),
                    value: t.value().to_string(),
                    set_by: t.set_by().describe(),
                    note: t.note(t.set_by()).map(str::to_owned),
                    layers: t
                        .layers()
                        .filter(|(l, _)| *l != SetBy::Default)
                        .map(|(l, v)| (l.describe(), v.to_string()))
                        .collect(),
                    unit: meta.unit,
                    source: meta.source.kind(),
                    source_detail: meta.source.detail(),
                    group: meta.group.map(crate::Group::driver),
                    scope: meta.scope,
                    net: meta.net,
                    lifetime: meta.lifetime,
                    help: t.help().to_owned(),
                    location: meta
                        .location
                        .map(|(file, line)| format!("{}:{line}", file.replace('\\', "/"))),
                }
            })
            .collect();
        rows.sort_by(|a, b| a.name.cmp(&b.name));
        rows
    }

    /// How many tunables still have [`crate::Source::Guess`] as their source.
    pub fn guesses(&self) -> usize {
        self.iter()
            .filter(|(_, t)| t.meta().source == crate::Source::Guess)
            .count()
    }

    /// A Markdown table of [`Tunables::rows`], one line per tunable, plus a count line.
    pub fn dump_markdown(&self) -> String {
        let cell = |s: &str| s.replace('|', "\\|").replace('\n', " ");
        let mut out = format!(
            "{} tunable(s), {} guess(es)\n\n\
             | name | value | default | kind | unit | set by | source | lifetime | net | scope | group | where | help |\n\
             |---|---|---|---|---|---|---|---|---|---|---|---|---|\n",
            self.len(),
            self.guesses()
        );
        for r in self.rows() {
            let source = match r.source_detail {
                "" => r.source.to_owned(),
                d => format!("{}: {d}", r.source),
            };
            let set_by = match &r.note {
                Some(n) => format!("{} ({n})", r.set_by),
                None => r.set_by.to_owned(),
            };
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                r.name,
                cell(&r.value),
                cell(&r.default),
                cell(&r.kind),
                r.unit.symbol(),
                cell(&set_by),
                cell(&source),
                r.lifetime.name(),
                match r.net {
                    Net::Presentation => "presentation",
                    Net::Gameplay => "gameplay",
                },
                match r.scope {
                    Scope::Dev => "dev",
                    Scope::Player => "player",
                },
                r.group.unwrap_or(""),
                r.location.as_deref().unwrap_or(""),
                cell(&r.help),
            );
        }
        out
    }
}
