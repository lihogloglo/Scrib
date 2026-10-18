//! Keyed list models (ui.md "View-models": stable keys, so sorting and
//! filtering never rebuild rows).
//!
//! Rows keep insertion order and a per-row version. The *view* is the keys
//! that pass the filter, in sort order; it is recomputed lazily when rows,
//! sort or filter change. A virtualized list binds a window of the view to
//! pooled row widgets and re-reads only rows whose version moved.

use std::hash::Hash;

use indexmap::IndexMap;

use crate::view_model::{UiValue, ViewModel};

/// Sort by a view-model field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortSpec {
    pub field: String,
    pub descending: bool,
}

/// Row filter: a case-insensitive search over some fields, plus exact field matches
/// (category tabs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListFilter {
    /// Search text; empty matches everything.
    pub search: String,
    /// Fields the search looks in.
    pub search_fields: Vec<String>,
    /// Every `(field, value)` must match exactly.
    pub equals: Vec<(String, UiValue)>,
}

impl ListFilter {
    fn matches<T: ViewModel>(&self, row: &T, needle: &str) -> bool {
        let found = needle.is_empty()
            || self.search_fields.iter().any(|f| {
                row.field(f)
                    .is_some_and(|v| v.to_string().to_lowercase().contains(needle))
            });
        found
            && self
                .equals
                .iter()
                .all(|(f, want)| row.field(f).as_ref() == Some(want))
    }
}

struct Row<T> {
    value: T,
    version: u64,
}

/// A keyed list of view-models with sort, filter and change versions.
pub struct ListModel<K, T> {
    rows: IndexMap<K, Row<T>>,
    /// Bumped by every change to rows, sort or filter.
    version: u64,
    sort: Option<SortSpec>,
    filter: ListFilter,
    view: Vec<K>,
    /// `version` the view was computed at.
    view_version: Option<u64>,
}

impl<K, T> Default for ListModel<K, T> {
    fn default() -> Self {
        Self {
            rows: IndexMap::new(),
            version: 0,
            sort: None,
            filter: ListFilter::default(),
            view: Vec::new(),
            view_version: None,
        }
    }
}

impl<K, T> ListModel<K, T>
where
    K: Clone + Eq + Hash,
    T: ViewModel + PartialEq,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or updates a row. Returns `false` (and changes nothing) when
    /// the row already held an equal value.
    pub fn upsert(&mut self, key: K, value: T) -> bool {
        if let Some(row) = self.rows.get_mut(&key) {
            if row.value == value {
                return false;
            }
            self.version += 1;
            *row = Row {
                value,
                version: self.version,
            };
        } else {
            self.version += 1;
            self.rows.insert(
                key,
                Row {
                    value,
                    version: self.version,
                },
            );
        }
        true
    }

    pub fn remove(&mut self, key: &K) -> Option<T> {
        let row = self.rows.shift_remove(key)?;
        self.version += 1;
        Some(row.value)
    }

    pub fn clear(&mut self) {
        if !self.rows.is_empty() {
            self.rows.clear();
            self.version += 1;
        }
    }

    pub fn get(&self, key: &K) -> Option<&T> {
        self.rows.get(key).map(|r| &r.value)
    }

    /// Version of a row's last change; compare with a bound widget's copy.
    pub fn row_version(&self, key: &K) -> Option<u64> {
        self.rows.get(key).map(|r| r.version)
    }

    /// Model version: changes whenever rows, sort or filter change.
    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Rows changed after `version` (inserted or updated), in insertion order.
    pub fn changed_since(&self, version: u64) -> impl Iterator<Item = &K> {
        self.rows
            .iter()
            .filter(move |(_, r)| r.version > version)
            .map(|(k, _)| k)
    }

    pub fn sort(&self) -> Option<&SortSpec> {
        self.sort.as_ref()
    }

    pub fn set_sort(&mut self, sort: Option<SortSpec>) {
        if self.sort != sort {
            self.sort = sort;
            self.version += 1;
        }
    }

    pub fn filter(&self) -> &ListFilter {
        &self.filter
    }

    pub fn set_filter(&mut self, filter: ListFilter) {
        if self.filter != filter {
            self.filter = filter;
            self.version += 1;
        }
    }

    /// Keys passing the filter, in sort order (ties keep insertion order).
    pub fn view(&mut self) -> &[K] {
        if self.view_version != Some(self.version) {
            self.rebuild_view();
        }
        &self.view
    }

    /// A window of the view for a virtualized list: `count` keys from `first`, clamped.
    pub fn window(&mut self, first: usize, count: usize) -> &[K] {
        let view = self.view();
        let start = first.min(view.len());
        &view[start..(start + count).min(view.len())]
    }

    fn rebuild_view(&mut self) {
        let needle = self.filter.search.to_lowercase();
        let mut keyed: Vec<(UiValue, &K)> = self
            .rows
            .iter()
            .filter(|(_, r)| self.filter.matches(&r.value, &needle))
            .map(|(k, r)| {
                let sort_value = self
                    .sort
                    .as_ref()
                    .and_then(|s| r.value.field(&s.field))
                    .unwrap_or(UiValue::None);
                (sort_value, k)
            })
            .collect();
        if let Some(sort) = &self.sort {
            // Stable: equal values keep insertion order in either direction.
            if sort.descending {
                keyed.sort_by(|a, b| match (&a.0, &b.0) {
                    (UiValue::None, UiValue::None) => std::cmp::Ordering::Equal,
                    (UiValue::None, _) => std::cmp::Ordering::Greater,
                    (_, UiValue::None) => std::cmp::Ordering::Less,
                    (x, y) => y.sort_cmp(x),
                });
            } else {
                keyed.sort_by(|a, b| a.0.sort_cmp(&b.0));
            }
        }
        self.view = keyed.into_iter().map(|(_, k)| k.clone()).collect();
        self.view_version = Some(self.version);
    }
}
