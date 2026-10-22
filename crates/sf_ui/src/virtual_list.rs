//! Virtualized list state (ui.md "Performance": a 1,000-item inventory scrolls at full
//! frame rate with ~40 live rows).
//!
//! The industry pattern (Unity UI Toolkit `ListView` with `FixedHeight` virtualization and
//! `makeItem`/`bindItem` recycling; Unreal's `UListView` entry pooling): a small pool of
//! row widgets covers the viewport plus a few rows of overscan, and scrolling rebinds only
//! the rows that enter the window. [`VirtualList`] is that bookkeeping without widgets:
//! scroll offset, keyed focus that survives re-sorting and filtering, and a stable
//! pool-slot assignment per key, so the Bevy side spawns `pool_len()` row entities once and
//! re-binds only rows flagged [`RowBinding::rebind`].
//!
//! Rows have one fixed extent (in the layout's units, e.g. px after UI scale). Variable
//! heights are not supported: dense game lists use fixed rows, and fixed rows make the
//! window an O(1) computation.

use std::collections::HashMap;
use std::hash::Hash;

use crate::list::ListModel;
use crate::view_model::ViewModel;

/// One visible row: which pool slot shows which key, and where.
#[derive(Clone, Debug, PartialEq)]
pub struct RowBinding<K> {
    /// Pool slot (row widget) index, stable while the key stays in the window.
    pub slot: usize,
    /// Position in the list view.
    pub index: usize,
    pub key: K,
    /// Top of the row relative to the viewport top (negative when partly scrolled out).
    pub offset: f32,
    /// The slot got a new key or the row's data changed: re-read its view-model.
    pub rebind: bool,
}

/// What to show this frame.
#[derive(Clone, Debug, PartialEq)]
pub struct ListFrame<K> {
    /// Rows in the window (visible plus overscan), in view order.
    pub rows: Vec<RowBinding<K>>,
    /// Pool slots that hold no row now (hide them).
    pub released: Vec<usize>,
    /// Number of rows in the view.
    pub len: usize,
    /// Height of all rows together.
    pub content_extent: f32,
    /// Scroll offset after clamping.
    pub scroll: f32,
    /// The focused row: view index and key.
    pub focus: Option<(usize, K)>,
    /// Scrollbar thumb as `(start, length)` fractions of the track; `None` when everything fits.
    pub thumb: Option<(f32, f32)>,
}

#[derive(Clone, Debug)]
struct Bound<K> {
    key: K,
    row_version: u64,
}

/// Scroll, focus and row-pool state of one virtualized list over a [`ListModel`].
#[derive(Clone, Debug)]
pub struct VirtualList<K> {
    row_extent: f32,
    viewport: f32,
    overscan: usize,
    scroll: f32,
    focus: Option<K>,
    /// Last known view index of `focus`: where focus lands when its row disappears.
    focus_index: usize,
    /// `focus` must be looked up again (view or focus changed).
    focus_dirty: bool,
    /// Scroll so the focused row is visible on the next update.
    follow_focus: bool,
    slots: Vec<Option<Bound<K>>>,
    /// Model version the focus index was resolved at.
    seen_version: Option<u64>,
    /// View length at the last update (for clamping and hit tests between updates);
    /// `None` before the first update, when scrolling isn't clamped yet.
    len: Option<usize>,
}

impl<K> VirtualList<K>
where
    K: Clone + Eq + Hash,
{
    /// Overscan rows on each side of the viewport by default.
    pub const DEFAULT_OVERSCAN: usize = 2;

    /// `row_extent` and `viewport` in the same units (px after UI scale).
    pub fn new(row_extent: f32, viewport: f32) -> Self {
        Self {
            row_extent: row_extent.max(f32::EPSILON),
            viewport: viewport.max(0.0),
            overscan: Self::DEFAULT_OVERSCAN,
            scroll: 0.0,
            focus: None,
            focus_index: 0,
            focus_dirty: false,
            follow_focus: false,
            slots: Vec::new(),
            seen_version: None,
            len: None,
        }
    }

    /// Rows kept bound above and below the viewport (smooth wheel scrolling, key repeat).
    #[must_use]
    pub fn with_overscan(mut self, rows: usize) -> Self {
        self.overscan = rows;
        self
    }

    pub fn row_extent(&self) -> f32 {
        self.row_extent
    }

    /// A new row extent (UI or text scale changed). The scroll offset scales with it, so
    /// the same rows stay in view.
    pub fn set_row_extent(&mut self, extent: f32) {
        let extent = extent.max(f32::EPSILON);
        self.scroll *= extent / self.row_extent;
        self.row_extent = extent;
    }

    pub fn viewport(&self) -> f32 {
        self.viewport
    }

    pub fn set_viewport(&mut self, extent: f32) {
        self.viewport = extent.max(0.0);
    }

    /// Rows that fit the viewport entirely (at least 1): the page size.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // non-negative, small
    pub fn page_rows(&self) -> usize {
        ((self.viewport / self.row_extent).floor() as usize).max(1)
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    /// Scrolls to an offset (mouse wheel, scrollbar drag), clamped to the content of the
    /// last update. Focus doesn't move.
    pub fn scroll_to(&mut self, offset: f32) {
        self.scroll = self.clamp_scroll(offset);
        self.follow_focus = false;
    }

    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_to(self.scroll + delta);
    }

    /// Scrolls by whole rows (wheel notches in rows).
    #[allow(clippy::cast_precision_loss)] // row counts are far below 2^24
    pub fn scroll_rows(&mut self, rows: isize) {
        self.scroll_by(rows as f32 * self.row_extent);
    }

    /// The view index under a point `y` from the viewport top (mouse hover, click).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // checked non-negative
    pub fn index_at(&self, y: f32) -> Option<usize> {
        if y < 0.0 || y >= self.viewport {
            return None;
        }
        let index = ((self.scroll + y) / self.row_extent).floor() as usize;
        (index < self.len.unwrap_or(0)).then_some(index)
    }

    pub fn focus(&self) -> Option<&K> {
        self.focus.as_ref()
    }

    /// Focuses a key (click, or the item a command just created) and scrolls to it.
    pub fn set_focus(&mut self, key: Option<K>) {
        self.focus = key;
        self.focus_dirty = true;
        self.follow_focus = true;
    }

    /// Scrolls the focused row into view on the next update. Call it after the user
    /// re-sorts or filters (a row changing underneath doesn't move the scroll).
    pub fn reveal_focus(&mut self) {
        self.follow_focus = true;
    }

    /// Focuses the row at a view index (clamped) and scrolls to it.
    pub fn focus_index<T>(&mut self, model: &mut ListModel<K, T>, index: usize)
    where
        T: ViewModel + PartialEq,
    {
        let view = model.view();
        if view.is_empty() {
            self.focus = None;
            return;
        }
        let index = index.min(view.len() - 1);
        self.focus = Some(view[index].clone());
        self.focus_index = index;
        self.focus_dirty = false;
        self.follow_focus = true;
        self.seen_version = Some(model.version());
    }

    /// Moves focus by `delta` rows, clamped at the ends (d-pad, arrows, key repeat). With
    /// nothing focused it starts at the first row.
    pub fn move_focus<T>(&mut self, model: &mut ListModel<K, T>, delta: isize)
    where
        T: ViewModel + PartialEq,
    {
        let start = match self.resolve_focus(model) {
            Some(i) => i.saturating_add_signed(delta),
            None => 0,
        };
        self.focus_index(model, start);
    }

    /// Moves focus by a page (`pages` = ±1 for page down/up, shoulder buttons).
    pub fn page<T>(&mut self, model: &mut ListModel<K, T>, pages: isize)
    where
        T: ViewModel + PartialEq,
    {
        let rows = isize::try_from(self.page_rows().saturating_sub(1).max(1)).unwrap_or(1);
        self.move_focus(model, pages.saturating_mul(rows));
    }

    /// Number of pool slots (row widgets) in use or released so far.
    pub fn pool_len(&self) -> usize {
        self.slots.len()
    }

    /// Forgets every slot binding, so the next update rebinds all rows (row template
    /// changed, theme swapped).
    pub fn invalidate(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }

    /// Current view index of the focused key, re-resolved when the view changed. A focused
    /// row that left the view hands focus to the row now at its old index.
    fn resolve_focus<T>(&mut self, model: &mut ListModel<K, T>) -> Option<usize>
    where
        T: ViewModel + PartialEq,
    {
        let version = model.version();
        let key = self.focus.clone()?;
        if self.focus_dirty || self.seen_version != Some(version) {
            self.seen_version = Some(version);
            self.focus_dirty = false;
            let view = model.view();
            if let Some(i) = view.iter().position(|k| *k == key) {
                self.focus_index = i;
            } else if view.is_empty() {
                self.focus = None;
                return None;
            } else {
                self.focus_index = self.focus_index.min(view.len() - 1);
                self.focus = Some(view[self.focus_index].clone());
            }
        }
        Some(self.focus_index)
    }

    #[allow(clippy::cast_precision_loss)] // row counts are far below 2^24
    fn clamp_scroll(&self, offset: f32) -> f32 {
        let Some(len) = self.len else {
            return offset.max(0.0);
        };
        let max = (len as f32 * self.row_extent - self.viewport).max(0.0);
        offset.clamp(0.0, max)
    }

    /// Resolves focus, scroll and the window, and assigns pool slots.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )] // row counts are far below 2^24; offsets are clamped non-negative
    pub fn update<T>(&mut self, model: &mut ListModel<K, T>) -> ListFrame<K>
    where
        T: ViewModel + PartialEq,
    {
        let focus = self.resolve_focus(model);
        let len = model.view().len();
        self.len = Some(len);
        let row = self.row_extent;
        if let (Some(i), true) = (focus, self.follow_focus) {
            let top = i as f32 * row;
            if top < self.scroll {
                self.scroll = top;
            } else if top + row > self.scroll + self.viewport {
                self.scroll = top + row - self.viewport;
            }
        }
        self.follow_focus = false;
        self.scroll = self.clamp_scroll(self.scroll);

        let first = ((self.scroll / row).floor() as usize).saturating_sub(self.overscan);
        let last = (((self.scroll + self.viewport) / row).ceil() as usize)
            .saturating_add(self.overscan)
            .min(len);
        let first = first.min(last);
        let window: Vec<K> = model.view()[first..last].to_vec();

        // Keys that stay in the window keep their slot; the rest are released.
        let in_window: HashMap<&K, usize> = window
            .iter()
            .enumerate()
            .map(|(i, k)| (k, first + i))
            .collect();
        let mut released = Vec::new();
        let mut kept: HashMap<K, usize> = HashMap::new();
        for (slot, bound) in self.slots.iter_mut().enumerate() {
            match bound {
                Some(b) if in_window.contains_key(&b.key) => {
                    kept.insert(b.key.clone(), slot);
                }
                Some(_) => {
                    *bound = None;
                    released.push(slot);
                }
                None => {}
            }
        }
        // Free slots, lowest last so `pop` reuses the lowest first; the pool grows only when
        // none is free.
        let mut free: Vec<usize> = (0..self.slots.len())
            .rev()
            .filter(|s| self.slots[*s].is_none())
            .collect();

        let mut rows = Vec::with_capacity(window.len());
        for (i, key) in window.into_iter().enumerate() {
            let index = first + i;
            let version = model.row_version(&key).unwrap_or(0);
            let (slot, rebind) = if let Some(&slot) = kept.get(&key) {
                let bound = self.slots[slot].as_mut().expect("kept slot is bound");
                let rebind = bound.row_version != version;
                bound.row_version = version;
                (slot, rebind)
            } else {
                let slot = free.pop().unwrap_or_else(|| {
                    self.slots.push(None);
                    self.slots.len() - 1
                });
                self.slots[slot] = Some(Bound {
                    key: key.clone(),
                    row_version: version,
                });
                (slot, true)
            };
            rows.push(RowBinding {
                slot,
                index,
                offset: index as f32 * row - self.scroll,
                key,
                rebind,
            });
        }
        released.retain(|s| self.slots[*s].is_none());

        let content_extent = len as f32 * row;
        let thumb = (content_extent > self.viewport && content_extent > 0.0)
            .then(|| (self.scroll / content_extent, self.viewport / content_extent));
        ListFrame {
            rows,
            released,
            len,
            content_extent,
            scroll: self.scroll,
            focus: focus.and_then(|i| self.focus.clone().map(|k| (i, k))),
            thumb,
        }
    }
}
