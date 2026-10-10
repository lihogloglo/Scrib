use indexmap::IndexMap;
use smallvec::SmallVec;

use crate::report::{KindSummary, RecordConflict};
use crate::{RecordId, SourceId};

/// What a source did to a record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Define,
    Patch,
    Delete,
}

/// One source's contribution to a record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Touch {
    pub source: SourceId,
    pub op: Op,
}

/// A resolved record and the history of every source that touched it.
#[derive(Clone, Debug)]
pub struct Entry<T> {
    /// `None` when the last operation deleted the record.
    pub value: Option<T>,
    pub history: SmallVec<[Touch; 2]>,
}

impl<T> Entry<T> {
    /// The source that defined the record's current base value.
    pub fn defined_by(&self) -> Option<SourceId> {
        self.history.iter().rev().find(|t| t.op == Op::Define).map(|t| t.source)
    }

    /// The last source to touch the record (the "winner").
    pub fn last_touched_by(&self) -> SourceId {
        self.history.last().expect("entries always have history").source
    }

    fn touch(&mut self, source: SourceId, op: Op) {
        // Several operations from one source collapse into the strongest one.
        if let Some(last) = self.history.last_mut()
            && last.source == source
        {
            if op != Op::Patch {
                last.op = op;
            }
            return;
        }
        self.history.push(Touch { source, op });
    }
}

pub(crate) struct Store<T> {
    pub(crate) kind: String,
    pub(crate) entries: IndexMap<RecordId, Entry<T>>,
    pub(crate) live: usize,
}

impl<T> Store<T> {
    pub(crate) fn new(kind: String) -> Self {
        Self { kind, entries: IndexMap::new(), live: 0 }
    }

    pub(crate) fn get(&self, id: &RecordId) -> Option<&T> {
        self.entries.get(id)?.value.as_ref()
    }

    pub(crate) fn define(&mut self, source: SourceId, id: RecordId, value: T) {
        let entry = self.entries.entry(id).or_insert_with(|| Entry { value: None, history: SmallVec::new() });
        if entry.value.is_none() {
            self.live += 1;
        }
        entry.value = Some(value);
        entry.touch(source, Op::Define);
    }

    pub(crate) fn patch(&mut self, source: SourceId, id: &RecordId, patch: impl FnOnce(&mut T)) -> bool {
        let Some(entry) = self.entries.get_mut(id) else { return false };
        let Some(value) = entry.value.as_mut() else { return false };
        patch(value);
        entry.touch(source, Op::Patch);
        true
    }

    pub(crate) fn delete(&mut self, source: SourceId, id: RecordId) {
        let entry = self.entries.entry(id).or_insert_with(|| Entry { value: None, history: SmallVec::new() });
        if entry.value.take().is_some() {
            self.live -= 1;
        }
        entry.touch(source, Op::Delete);
    }

    pub(crate) fn summarize(&self, knows: &[Vec<bool>]) -> KindSummary {
        let mut summary = KindSummary { kind: self.kind.clone(), live: self.live, ..KindSummary::default() };
        for (id, entry) in &self.entries {
            summary.total += 1;
            if entry.history.len() > 1 {
                summary.overridden += 1;
            }
            // A conflict is a later source touching a record that an earlier
            // source it does not depend on also touched: the later author
            // never saw that change. Overriding your own masters is intended.
            let blind = entry.history.iter().enumerate().any(|(i, later)| {
                entry.history[..i].iter().any(|earlier| !knows[later.source.0 as usize][earlier.source.0 as usize])
            });
            if blind {
                summary.conflicts.push(RecordConflict {
                    id: id.clone(),
                    history: entry.history.iter().copied().collect(),
                });
            }
        }
        summary
    }
}
