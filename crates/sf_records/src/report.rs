use std::fmt::Write as _;

use crate::{RecordDb, RecordId, Touch};

/// A record touched by a source that does not depend on an earlier source
/// that also touched it.
#[derive(Clone, Debug)]
pub struct RecordConflict {
    pub id: RecordId,
    pub history: Vec<Touch>,
}

/// Per-kind override and conflict statistics.
#[derive(Clone, Debug, Default)]
pub struct KindSummary {
    pub kind: String,
    /// Records ever seen, including deleted ones.
    pub total: usize,
    /// Records currently alive.
    pub live: usize,
    /// Records touched by more than one source.
    pub overridden: usize,
    /// Records with blind overrides (see [`RecordConflict`]).
    pub conflicts: Vec<RecordConflict>,
}

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

    /// Human-readable rendering, listing at most `max_conflicts` conflicts per kind.
    pub fn render(&self, db: &RecordDb, max_conflicts: usize) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{:<6} {:>8} {:>8} {:>10} {:>9}", "kind", "live", "total", "overridden", "conflicts");
        for k in &self.kinds {
            let _ = writeln!(
                out,
                "{:<6} {:>8} {:>8} {:>10} {:>9}",
                k.kind,
                k.live,
                k.total,
                k.overridden,
                k.conflicts.len()
            );
        }
        for k in self.kinds.iter().filter(|k| !k.conflicts.is_empty()) {
            let _ = writeln!(out, "\n{} conflicts:", k.kind);
            for c in k.conflicts.iter().take(max_conflicts) {
                let chain: Vec<String> =
                    c.history.iter().map(|t| format!("{} ({:?})", db.source(t.source).name, t.op)).collect();
                let _ = writeln!(out, "  {:?}: {}", c.id, chain.join(" -> "));
            }
            if k.conflicts.len() > max_conflicts {
                let _ = writeln!(out, "  ... and {} more", k.conflicts.len() - max_conflicts);
            }
        }
        out
    }
}
