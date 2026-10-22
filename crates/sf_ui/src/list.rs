//! Keyed list models (ui.md "View-models": stable keys, so sorting and
//! filtering never rebuild rows).
//!
//! Rows keep insertion order and a per-row version. The *view* is the keys
//! that pass the filter, in sort order; it is recomputed lazily when rows,
//! sort or filter change. Sort keys and lowercased search text are cached per
//! row until the row, the sort or the searched fields change, so a rebuild
//! after one changed row re-reads one row and re-sorts cached keys. A
//! virtualized list ([`VirtualList`](crate::VirtualList)) binds a window of the
//! view to pooled row widgets and re-reads only rows whose version moved.

use std::cmp::Ordering;
use std::collections::HashMap;
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
    /// Search text; empty matches everything. Split on whitespace, every word must
    /// appear (in any searched field, in any order): "iron sw" finds "Iron Longsword".
    pub search: String,
    /// Fields the search looks in.
    pub search_fields: Vec<String>,
    /// Every `(field, value)` must match exactly.
    pub equals: Vec<(String, UiValue)>,
    /// Every field's value must be one of its values (a tab spanning several categories).
    pub one_of: Vec<(String, Vec<UiValue>)>,
}

impl ListFilter {
    /// Whether `row` passes; `words` is the search text lowercased and split (`search_words`).
    pub fn matches<T: ViewModel>(&self, row: &T, words: &[String]) -> bool {
        self.fields_match(row) && (words.is_empty() || words_found(words, &self.haystack(row)))
    }

    /// The `equals` and `one_of` conditions.
    fn fields_match<T: ViewModel>(&self, row: &T) -> bool {
        self.equals
            .iter()
            .all(|(f, want)| row.field(f).as_ref() == Some(want))
            && self
                .one_of
                .iter()
                .all(|(f, set)| row.field(f).is_some_and(|v| set.contains(&v)))
    }

    /// The searched fields of a row, lowercased.
    fn haystack<T: ViewModel>(&self, row: &T) -> Vec<String> {
        self.search_fields
            .iter()
            .filter_map(|f| row.field(f))
            .map(|v| v.to_string().to_lowercase())
            .collect()
    }

    /// The search text as lowercase words.
    pub fn search_words(&self) -> Vec<String> {
        self.search
            .to_lowercase()
            .split_whitespace()
            .map(str::to_owned)
            .collect()
    }
}

fn words_found(words: &[String], haystack: &[String]) -> bool {
    words
        .iter()
        .all(|w| haystack.iter().any(|h| h.contains(w.as_str())))
}

/// A row's sort value, computed once per row and sort (text lowercased once, not per
/// comparison) and cached until the row or the sort changes.
enum SortKey {
    Number(f64),
    Text { folded: String, raw: String },
    None,
}

impl SortKey {
    fn of(value: Option<UiValue>) -> Self {
        match value {
            None | Some(UiValue::None) => SortKey::None,
            Some(UiValue::Text(raw)) => SortKey::Text {
                folded: raw.to_lowercase(),
                raw,
            },
            Some(v) => v.as_f64().map_or(SortKey::None, SortKey::Number),
        }
    }

    /// [`UiValue::sort_cmp`] order on precomputed keys; `None` last in both directions.
    fn cmp(&self, other: &Self, descending: bool) -> Ordering {
        let ord = match (self, other) {
            (SortKey::None, SortKey::None) => return Ordering::Equal,
            (SortKey::None, _) => return Ordering::Greater,
            (_, SortKey::None) => return Ordering::Less,
            (SortKey::Number(a), SortKey::Number(b)) => a.total_cmp(b),
            (SortKey::Text { folded: a, raw: ra }, SortKey::Text { folded: b, raw: rb }) => {
                a.cmp(b).then_with(|| ra.cmp(rb))
            }
            // Mixed text and numbers: numbers first.
            (SortKey::Number(_), SortKey::Text { .. }) => Ordering::Less,
            (SortKey::Text { .. }, SortKey::Number(_)) => Ordering::Greater,
        };
        if descending { ord.reverse() } else { ord }
    }
}

struct Row<T> {
    value: T,
    version: u64,
    cache: RowCache,
}

impl<T> Row<T> {
    fn new(value: T, version: u64) -> Self {
        Self {
            value,
            version,
            cache: RowCache::default(),
        }
    }
}

/// Derived per-row data for view rebuilds, valid while the row and the epoch it was
/// computed at are unchanged (a new value replaces the whole row and its cache).
#[derive(Default)]
struct RowCache {
    sort_epoch: u64,
    sort: Vec<SortKey>,
    search_epoch: u64,
    search: Vec<String>,
}

/// A keyed list of view-models with sort, filter and change versions.
pub struct ListModel<K, T> {
    rows: IndexMap<K, Row<T>>,
    /// Bumped by every change to rows, sort or filter.
    version: u64,
    /// Sort keys, most significant first; empty keeps insertion order.
    sort: Vec<SortSpec>,
    filter: ListFilter,
    view: Vec<K>,
    /// `version` the view was computed at.
    view_version: Option<u64>,
    /// Bumped when the sort keys change: cached row sort keys from older epochs are stale.
    sort_epoch: u64,
    /// Bumped when the searched fields change.
    search_epoch: u64,
}

impl<K, T> Default for ListModel<K, T> {
    fn default() -> Self {
        Self {
            rows: IndexMap::new(),
            version: 0,
            sort: Vec::new(),
            filter: ListFilter::default(),
            view: Vec::new(),
            view_version: None,
            sort_epoch: 1,
            search_epoch: 1,
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
            *row = Row::new(value, self.version);
        } else {
            self.version += 1;
            self.rows.insert(key, Row::new(value, self.version));
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

    /// Every row in insertion order, ignoring sort and filter.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &T)> {
        self.rows.iter().map(|(k, r)| (k, &r.value))
    }

    /// Rows changed after `version` (inserted or updated), in insertion order.
    pub fn changed_since(&self, version: u64) -> impl Iterator<Item = &K> {
        self.rows
            .iter()
            .filter(move |(_, r)| r.version > version)
            .map(|(k, _)| k)
    }

    /// The primary sort key.
    pub fn sort(&self) -> Option<&SortSpec> {
        self.sort.first()
    }

    /// Sorts on one field (or not at all).
    pub fn set_sort(&mut self, sort: Option<SortSpec>) {
        self.set_sort_keys(sort.into_iter().collect());
    }

    /// Every sort key, most significant first.
    pub fn sort_keys(&self) -> &[SortSpec] {
        &self.sort
    }

    /// Sorts on several fields: later keys break ties of earlier ones (sort by weight,
    /// then name); remaining ties keep insertion order.
    pub fn set_sort_keys(&mut self, keys: Vec<SortSpec>) {
        if self.sort != keys {
            self.sort = keys;
            self.version += 1;
            self.sort_epoch += 1;
        }
    }

    pub fn filter(&self) -> &ListFilter {
        &self.filter
    }

    pub fn set_filter(&mut self, filter: ListFilter) {
        if self.filter != filter {
            if self.filter.search_fields != filter.search_fields {
                self.search_epoch += 1;
            }
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

    /// Puts rows in the order of `keys` (the source list was reordered: a stack inserted
    /// mid-list). Rows not named keep their relative order after the named ones. Row
    /// versions don't move, so bound rows don't rebind; the view is recomputed. Returns
    /// whether anything moved.
    pub fn reorder(&mut self, keys: &[K]) -> bool {
        if self.rows.len() == keys.len() && self.rows.keys().eq(keys) {
            return false;
        }
        let rank: HashMap<&K, usize> = keys.iter().enumerate().map(|(i, k)| (k, i)).collect();
        let rank_of = |k: &K| rank.get(k).copied().unwrap_or(usize::MAX);
        let mut last = 0;
        let sorted = self.rows.keys().all(|k| {
            let r = rank_of(k);
            let ok = r >= last;
            last = r;
            ok
        });
        if sorted {
            return false;
        }
        self.rows.sort_by_cached_key(|k, _| rank_of(k));
        self.version += 1;
        true
    }

    fn rebuild_view(&mut self) {
        let words = self.filter.search_words();
        let mut passing = Vec::new();
        for (i, row) in self.rows.values_mut().enumerate() {
            if !self.filter.fields_match(&row.value) {
                continue;
            }
            if !words.is_empty() {
                if row.cache.search_epoch != self.search_epoch {
                    row.cache.search = self.filter.haystack(&row.value);
                    row.cache.search_epoch = self.search_epoch;
                }
                if !words_found(&words, &row.cache.search) {
                    continue;
                }
            }
            if !self.sort.is_empty() && row.cache.sort_epoch != self.sort_epoch {
                row.cache.sort = self
                    .sort
                    .iter()
                    .map(|s| SortKey::of(row.value.field(&s.field)))
                    .collect();
                row.cache.sort_epoch = self.sort_epoch;
            }
            passing.push(i);
        }
        if !self.sort.is_empty() {
            let rows = &self.rows;
            let keys = |i: usize| &rows.get_index(i).expect("row index").1.cache.sort;
            // Stable: equal values keep insertion order in either direction.
            passing.sort_by(|&a, &b| {
                self.sort
                    .iter()
                    .zip(keys(a).iter().zip(keys(b)))
                    .map(|(spec, (x, y))| x.cmp(y, spec.descending))
                    .find(|o| o.is_ne())
                    .unwrap_or(Ordering::Equal)
            });
        }
        self.view = passing
            .into_iter()
            .map(|i| self.rows.get_index(i).expect("row index").0.clone())
            .collect();
        self.view_version = Some(self.version);
    }

    /// Position of `key` in the view.
    pub fn position(&mut self, key: &K) -> Option<usize> {
        self.view().iter().position(|k| k == key)
    }
}
