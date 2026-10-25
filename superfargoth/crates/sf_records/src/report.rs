use std::fmt::Write as _;

use crate::{Op, RecordDb, RecordId, Touch};

/// A record touched by a source that does not depend on an earlier source
/// that also touched it (a "blind" override).
///
/// In [`KindSummary::conflicts`] the two wrote a common field (or the whole
/// record); in [`KindSummary::merges`] their fields were disjoint and both
/// changes apply.
#[derive(Clone, Debug)]
pub struct RecordConflict {
    pub id: RecordId,
    /// Every source's touch, in apply order, with the fields each wrote.
    pub history: Vec<Touch>,
    /// Contested field paths; empty when the whole record is contested (or for merges).
    pub fields: Vec<Box<str>>,
}

impl RecordConflict {
    /// One line: `id: a (define) -> b (patch radius) -> c (patch color) [radius]`.
    pub fn render(&self, db: &RecordDb) -> String {
        let chain: Vec<String> = self.history.iter().map(|t| format!("{} ({})", db.source(t.source).name, describe(t))).collect();
        let mut line = format!("{:?}: {}", self.id, chain.join(" -> "));
        if !self.fields.is_empty() {
            let _ = write!(line, " [contested: {}]", self.fields.join(", "));
        }
        line
    }
}

/// `define`, `delete`, `patch` or `patch a, b`.
pub fn describe(touch: &Touch) -> String {
    match (touch.op, &touch.fields) {
        (Op::Define, _) => "define".into(),
        (Op::Delete, _) => "delete".into(),
        (Op::Patch, None) => "patch".into(),
        (Op::Patch, Some(fields)) => format!("patch {}", fields.join(", ")),
    }
}

/// Per-kind override, conflict and merge statistics.
#[derive(Clone, Debug, Default)]
pub struct KindSummary {
    pub kind: String,
    /// Records ever seen, including deleted ones.
    pub total: usize,
    /// Records currently alive.
    pub live: usize,
    /// Records touched by more than one source.
    pub overridden: usize,
    /// Blind overrides that wrote a common field (see [`RecordConflict`]).
    pub conflicts: Vec<RecordConflict>,
    /// Blind overrides of disjoint fields, merged automatically.
    pub merges: Vec<RecordConflict>,
}

/// Picks one list of a kind summary.
type ListOf = fn(&KindSummary) -> &Vec<RecordConflict>;

#[derive(Clone, Debug, Default)]
pub struct ConflictReport {
    pub kinds: Vec<KindSummary>,
}

impl ConflictReport {
    pub fn total_live(&self) -> usize {
        self.kinds.iter().map(|k| k.live).sum()
    }

    pub fn total_conflicts(&self) -> usize {
        self.kinds.iter().map(|k| k.conflicts.len()).sum()
    }

    pub fn total_merges(&self) -> usize {
        self.kinds.iter().map(|k| k.merges.len()).sum()
    }

    /// Human-readable rendering, listing at most `max_conflicts` conflicts
    /// (and merges) per kind.
    pub fn render(&self, db: &RecordDb, max_conflicts: usize) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "{:<6} {:>8} {:>8} {:>10} {:>9} {:>7}",
            "kind", "live", "total", "overridden", "conflicts", "merges"
        );
        for k in &self.kinds {
            let _ = writeln!(
                out,
                "{:<6} {:>8} {:>8} {:>10} {:>9} {:>7}",
                k.kind,
                k.live,
                k.total,
                k.overridden,
                k.conflicts.len(),
                k.merges.len()
            );
        }
        let lists: [(&str, ListOf); 2] =
            [("conflicts", |k| &k.conflicts), ("merges", |k| &k.merges)];
        for (label, list) in lists {
            for k in self.kinds.iter().filter(|k| !list(k).is_empty()) {
                let list = list(k);
                let _ = writeln!(out, "\n{} {label}:", k.kind);
                for c in list.iter().take(max_conflicts) {
                    let _ = writeln!(out, "  {}", c.render(db));
                }
                if list.len() > max_conflicts {
                    let _ = writeln!(out, "  ... and {} more", list.len() - max_conflicts);
                }
            }
        }
        out
    }
}
