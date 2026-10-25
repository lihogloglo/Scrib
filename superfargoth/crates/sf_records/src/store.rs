use indexmap::IndexMap;
use smallvec::SmallVec;

use crate::patch::paths_overlap;
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Touch {
    pub source: SourceId,
    pub op: Op,
    /// Field paths a patch wrote (sorted); `None` when the whole record was
    /// written: a define, a delete, or a patch whose fields aren't known
    /// (a closure patch, legacy formats).
    pub fields: Option<Box<[Box<str>]>>,
}

impl Touch {
    /// What two touches both wrote: `None` when disjoint, an empty list when
    /// both wrote the whole record, else the contested paths (the broader of
    /// each overlapping pair).
    pub fn overlap(&self, other: &Touch) -> Option<Vec<Box<str>>> {
        match (&self.fields, &other.fields) {
            (None, None) => Some(Vec::new()),
            (None, Some(fields)) | (Some(fields), None) => Some(fields.to_vec()),
            (Some(a), Some(b)) => {
                let mut out: Vec<Box<str>> = Vec::new();
                for x in a {
                    for y in b.iter().filter(|y| paths_overlap(x, y)) {
                        let broader = if x.len() <= y.len() { x } else { y };
                        if !out.contains(broader) {
                            out.push(broader.clone());
                        }
                    }
                }
                (!out.is_empty()).then_some(out)
            }
        }
    }
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

    fn touch(&mut self, source: SourceId, op: Op, fields: Option<Vec<String>>) {
        // Several operations from one source collapse into the strongest one;
        // field lists of patches merge.
        if let Some(last) = self.history.last_mut()
            && last.source == source
        {
            match (op, &last.fields, fields) {
                (Op::Patch, Some(old), Some(new)) => {
                    let mut all: Vec<Box<str>> = old.to_vec();
                    all.extend(new.into_iter().map(String::into_boxed_str));
                    all.sort();
                    all.dedup();
                    last.fields = Some(all.into_boxed_slice());
                }
                (Op::Patch, Some(_), None) => last.fields = None,
                (Op::Patch, None, _) => {}
                (_, _, _) => {
                    last.op = op;
                    last.fields = None;
                }
            }
            return;
        }
        let fields = fields.map(|f| f.into_iter().map(String::into_boxed_str).collect());
        self.history.push(Touch { source, op, fields });
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
        entry.touch(source, Op::Define, None);
    }

    /// Patches a live record; `fields` are the paths written (`None`: unknown).
    pub(crate) fn patch(
        &mut self,
        source: SourceId,
        id: &RecordId,
        patch: impl FnOnce(&mut T),
        fields: Option<Vec<String>>,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(id) else { return false };
        let Some(value) = entry.value.as_mut() else { return false };
        patch(value);
        entry.touch(source, Op::Patch, fields);
        true
    }

    pub(crate) fn delete(&mut self, source: SourceId, id: RecordId) {
        let entry = self.entries.entry(id).or_insert_with(|| Entry { value: None, history: SmallVec::new() });
        if entry.value.take().is_some() {
            self.live -= 1;
        }
        entry.touch(source, Op::Delete, None);
    }

    pub(crate) fn summarize(&self, knows: &[Vec<bool>]) -> KindSummary {
        let mut summary = KindSummary { kind: self.kind.clone(), live: self.live, ..KindSummary::default() };
        for (id, entry) in &self.entries {
            summary.total += 1;
            if entry.history.len() > 1 {
                summary.overridden += 1;
            }
            // A later source touching a record that an earlier source it does
            // not depend on also touched never saw that change (overriding your
            // own masters is intended). If both wrote a common field (or the
            // whole record), that is a conflict; disjoint fields merge.
            let mut contested: Option<Vec<Box<str>>> = None;
            let mut blind = false;
            for (i, later) in entry.history.iter().enumerate() {
                for earlier in &entry.history[..i] {
                    if knows[later.source.0 as usize][earlier.source.0 as usize] {
                        continue;
                    }
                    blind = true;
                    if let Some(fields) = later.overlap(earlier) {
                        let all = contested.get_or_insert_with(Vec::new);
                        if fields.is_empty() {
                            all.clear();
                            all.push("*".into());
                        } else if all.first().is_none_or(|f| &**f != "*") {
                            all.extend(fields.into_iter().filter(|f| !all.contains(f)).collect::<Vec<_>>());
                        }
                    }
                }
            }
            let record = |fields: Vec<Box<str>>| RecordConflict {
                id: id.clone(),
                history: entry.history.iter().cloned().collect(),
                fields: fields.into_iter().filter(|f| &**f != "*").collect(),
            };
            match contested {
                Some(fields) => summary.conflicts.push(record(fields)),
                None if blind => summary.merges.push(record(Vec::new())),
                None => {}
            }
        }
        summary
    }
}
