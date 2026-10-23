//! The icon grid (ui.md "As built (U5)" listed it as not done): the same rows as the list,
//! laid out `columns` to a grid row and virtualized by grid rows.
//!
//! Unreal's `UTileView` is a `UListView` whose entries are laid out in tiles; the same reuse
//! here: the visible stacks (tab, search and sort already applied) are chunked into grid rows,
//! held as a keyed `sf_ui::ListModel` (key = grid row index), and `sf_ui::VirtualList` windows,
//! scrolls and pools those rows exactly as it does list rows. Focus stays the inventory's keyed
//! focus ([`InventoryView::focus`]), so it survives sorting, filtering and a change of column
//! count; left/right move it by one stack, up/down by one grid row.

use sf_rpg::StackId;
use sf_ui::{UiValue, ViewModel, VirtualList};

use crate::view::InventoryView;

/// One grid row: the stacks in its cells, left to right (the last row may be short).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GridRow {
    pub stacks: Vec<StackId>,
}

impl ViewModel for GridRow {
    const KIND: &'static str = "inventory.grid_row";

    fn fields() -> &'static [&'static str] {
        &["cells"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        (name == "cells")
            .then(|| UiValue::from(i64::try_from(self.stacks.len()).unwrap_or(i64::MAX)))
    }
}

/// A visible grid row this frame.
#[derive(Clone, Debug, PartialEq)]
pub struct GridRowBinding {
    /// Pool slot (row widget), stable while the row stays in the window.
    pub slot: usize,
    /// Grid row index.
    pub row: usize,
    /// Top relative to the viewport top.
    pub offset: f32,
    /// Its cells hold other stacks than last frame (cells also rebind on their own row
    /// versions, see [`InventoryView::rows`]).
    pub rebind: bool,
    pub stacks: Vec<StackId>,
}

/// What the grid shows this frame.
#[derive(Clone, Debug, PartialEq)]
pub struct GridFrame {
    pub rows: Vec<GridRowBinding>,
    /// Row slots that hold nothing now (hide them).
    pub released: Vec<usize>,
    pub columns: usize,
    /// Visible stacks (cells), not grid rows.
    pub len: usize,
    pub content_extent: f32,
    pub scroll: f32,
    /// The focused stack and its view index.
    pub focus: Option<(usize, StackId)>,
    /// Scrollbar thumb as `(start, length)` fractions; `None` when everything fits.
    pub thumb: Option<(f32, f32)>,
}

/// Grid layout and virtualization over an [`InventoryView`].
pub struct InventoryGrid {
    columns: usize,
    rows: sf_ui::ListModel<usize, GridRow>,
    nav: VirtualList<usize>,
    /// (list version, columns) the rows were chunked for.
    built: Option<(u64, usize)>,
    row_count: usize,
    /// Focus seen last frame (stack, view index): the window follows it only when it moves.
    last_focus: Option<(StackId, usize)>,
}

impl InventoryGrid {
    /// `cell_extent` (grid row height) and `viewport` in layout units.
    pub fn new(columns: usize, cell_extent: f32, viewport: f32) -> Self {
        Self {
            columns: columns.max(1),
            rows: sf_ui::ListModel::new(),
            nav: VirtualList::new(cell_extent, viewport),
            built: None,
            row_count: 0,
            last_focus: None,
        }
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Cells per grid row (the viewport width over the cell width); the focused stack stays
    /// in view.
    pub fn set_columns(&mut self, columns: usize) {
        let columns = columns.max(1);
        if columns != self.columns {
            self.columns = columns;
            self.last_focus = None;
        }
    }

    pub fn cell_extent(&self) -> f32 {
        self.nav.row_extent()
    }

    pub fn set_cell_extent(&mut self, extent: f32) {
        self.nav.set_row_extent(extent);
    }

    pub fn viewport(&self) -> f32 {
        self.nav.viewport()
    }

    pub fn set_viewport(&mut self, extent: f32) {
        self.nav.set_viewport(extent);
    }

    pub fn scroll(&self) -> f32 {
        self.nav.scroll()
    }

    /// Mouse wheel, scrollbar: focus doesn't move.
    pub fn scroll_by(&mut self, delta: f32) {
        self.nav.scroll_by(delta);
    }

    pub fn scroll_to(&mut self, offset: f32) {
        self.nav.scroll_to(offset);
    }

    /// Moves focus `dx` cells and `dy` grid rows (d-pad, arrows), clamped to the visible
    /// stacks. A move off the end of a row continues on the next one (reading order).
    pub fn move_focus(&mut self, view: &mut InventoryView, dx: isize, dy: isize) {
        let columns = isize::try_from(self.columns).unwrap_or(1);
        view.move_focus(dx.saturating_add(dy.saturating_mul(columns)));
    }

    /// Page up/down by `pages` viewports of grid rows.
    pub fn page(&mut self, view: &mut InventoryView, pages: isize) {
        let rows = isize::try_from(self.nav.page_rows().saturating_sub(1).max(1)).unwrap_or(1);
        self.move_focus(view, 0, pages.saturating_mul(rows));
    }

    /// The view index under a point relative to the viewport's top left, for cells of
    /// `cell_width` (hover, click, drop).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // checked non-negative
    pub fn index_at(&self, x: f32, y: f32, cell_width: f32, len: usize) -> Option<usize> {
        if x < 0.0 || cell_width <= 0.0 {
            return None;
        }
        let column = (x / cell_width).floor() as usize;
        if column >= self.columns {
            return None;
        }
        let row = self.nav.index_at(y)?;
        let index = row * self.columns + column;
        (index < len).then_some(index)
    }

    /// The rows to show this frame: re-chunks after any change to the list (rows, tab,
    /// search, sort) or to the column count, then windows and pools the grid rows.
    pub fn frame(&mut self, view: &mut InventoryView) -> GridFrame {
        // A focused stack the tab or search hid hands focus to the stack now at its index.
        if let Some(focus) = view.focus()
            && !view.visible().contains(&focus)
        {
            view.move_focus(0);
        }
        let version = view.rows().version();
        let columns = self.columns;
        if self.built != Some((version, columns)) {
            self.built = Some((version, columns));
            let visible = view.visible();
            let count = visible.len().div_ceil(columns);
            for (row, chunk) in visible.chunks(columns).enumerate() {
                self.rows.upsert(
                    row,
                    GridRow {
                        stacks: chunk.to_vec(),
                    },
                );
            }
            for row in count..self.row_count {
                self.rows.remove(&row);
            }
            self.row_count = count;
        }
        let focused = view.focus();
        let visible = view.visible();
        let len = visible.len();
        let focus = focused.and_then(|f| visible.iter().position(|k| *k == f).map(|i| (i, f)));
        let moved = focus.map(|(i, f)| (f, i));
        if moved != self.last_focus {
            self.last_focus = moved;
            // Re-focusing the grid row scrolls it into view.
            self.nav.set_focus(focus.map(|(i, _)| i / columns));
        }
        let frame = self.nav.update(&mut self.rows);
        let rows = frame
            .rows
            .into_iter()
            .map(|b| GridRowBinding {
                slot: b.slot,
                row: b.key,
                offset: b.offset,
                rebind: b.rebind,
                stacks: self
                    .rows
                    .get(&b.key)
                    .map(|r| r.stacks.clone())
                    .unwrap_or_default(),
            })
            .collect();
        GridFrame {
            rows,
            released: frame.released,
            columns,
            len,
            content_extent: frame.content_extent,
            scroll: frame.scroll,
            focus,
            thumb: frame.thumb,
        }
    }
}
