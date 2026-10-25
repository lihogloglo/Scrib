//! The inventory screen's state: rows synced from a [`Loadout`], tabs, search, sort,
//! totals and the virtualized list.

use std::collections::{HashMap, HashSet};

use sf_rpg::{Holder, Loadout, RecordId, RpgEvent, SlotTable, StackId};
use sf_ui::{
    ListFilter, ListFrame, ListModel, SortSpec, UiValue, VirtualList,
    records::{ColumnDef, SortKind, ValueSource},
};

use crate::catalog::{InventoryTab, ItemCatalog};
use crate::row::ItemRow;

/// The list id columns target (`ui.column` records with `list = "inventory.items"`).
pub const INVENTORY_LIST: &str = "inventory.items";

/// Carried weight against the carry limit.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Encumbrance {
    pub carried: f64,
    /// The carry limit (the adapter derives it from stats); `None` for no limit.
    pub capacity: Option<f64>,
}

impl Encumbrance {
    /// `carried / capacity` (1.0 = at the limit); `None` without a limit.
    pub fn ratio(&self) -> Option<f64> {
        self.capacity.map(|c| {
            if c > 0.0 {
                self.carried / c
            } else {
                f64::INFINITY
            }
        })
    }

    /// Over the limit (the game decides what that does: slow, can't move).
    pub fn is_over(&self) -> bool {
        self.capacity.is_some_and(|c| self.carried > c)
    }

    /// Capacity left (negative when over); `None` without a limit.
    pub fn remaining(&self) -> Option<f64> {
        self.capacity.map(|c| c - self.carried)
    }

    /// The same limit after carrying `delta` more (a take-all or barter preview).
    #[must_use]
    pub fn after(&self, delta: f64) -> Self {
        Self {
            carried: self.carried + delta,
            ..*self
        }
    }
}

/// Totals over every stack (not just the current tab or search).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Totals {
    pub encumbrance: Encumbrance,
    /// Sum of `count × value`.
    pub value: i64,
    pub stacks: usize,
    pub pieces: u64,
}

/// What a [`InventoryView::sync`] changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SyncReport {
    /// Rows added, or whose stack changed so `describe` ran and gave new data.
    pub described: usize,
    /// Rows whose equipment or carried weight changed without new display data.
    pub updated: usize,
    pub removed: usize,
}

/// The current sort: one column, plus the fixed tie-breaks (name, then inventory order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventorySort {
    pub field: String,
    pub descending: bool,
}

/// State of one inventory list (the player's, or a container's or merchant's side of a
/// barter screen).
pub struct InventoryView {
    holder: Holder,
    list: ListModel<StackId, ItemRow>,
    nav: VirtualList<StackId>,
    tabs: Vec<InventoryTab>,
    tab: usize,
    search: String,
    search_fields: Vec<String>,
    sort: Option<InventorySort>,
    pin_equipped: bool,
    totals: Totals,
}

impl InventoryView {
    /// A view of `holder`'s items with the given tabs (an "all" tab when empty) and list
    /// geometry (row extent and viewport in layout units).
    pub fn new(holder: Holder, tabs: Vec<InventoryTab>, row_extent: f32, viewport: f32) -> Self {
        let tabs = if tabs.is_empty() {
            vec![InventoryTab::all()]
        } else {
            tabs
        };
        let mut view = Self {
            holder,
            list: ListModel::new(),
            nav: VirtualList::new(row_extent, viewport),
            tabs,
            tab: 0,
            search: String::new(),
            search_fields: vec!["name".into()],
            sort: None,
            pin_equipped: false,
            totals: Totals::default(),
        };
        view.apply_sort();
        view.apply_filter();
        view
    }

    pub fn holder(&self) -> Holder {
        self.holder
    }

    /// The row model (columns and bindings read it).
    pub fn rows(&self) -> &ListModel<StackId, ItemRow> {
        &self.list
    }

    pub fn row(&self, stack: StackId) -> Option<&ItemRow> {
        self.list.get(&stack)
    }

    /// Stack ids passing the tab and search, in sort order.
    pub fn visible(&mut self) -> &[StackId] {
        self.list.view()
    }

    pub fn totals(&self) -> Totals {
        self.totals
    }

    pub fn encumbrance(&self) -> Encumbrance {
        self.totals.encumbrance
    }

    /// Sets the carry limit (the adapter recomputes it when stats change).
    pub fn set_capacity(&mut self, capacity: Option<f64>) {
        self.totals.encumbrance.capacity = capacity;
    }

    // ------------------------------------------------------------------ sync

    /// Brings the rows in line with the loadout. Unchanged stacks keep their row (no
    /// `describe`, no rebind); call it when the holder's items or equipment changed
    /// (`ItemsChanged`, stack events, equip).
    pub fn sync(
        &mut self,
        loadout: &Loadout,
        table: &SlotTable,
        catalog: &dyn ItemCatalog,
    ) -> SyncReport {
        self.sync_inner(loadout, table, catalog, false)
    }

    /// As [`InventoryView::sync`], describing every stack again (the catalog changed: a
    /// mod reloaded, the language switched).
    pub fn refresh(
        &mut self,
        loadout: &Loadout,
        table: &SlotTable,
        catalog: &dyn ItemCatalog,
    ) -> SyncReport {
        self.sync_inner(loadout, table, catalog, true)
    }

    fn sync_inner(
        &mut self,
        loadout: &Loadout,
        table: &SlotTable,
        catalog: &dyn ItemCatalog,
        force: bool,
    ) -> SyncReport {
        let mut slots: HashMap<StackId, Vec<RecordId>> = HashMap::new();
        for (slot, stack) in loadout.equipped() {
            if let Some(name) = table.name(slot) {
                slots.entry(stack).or_default().push(name.clone());
            }
        }
        let mut report = SyncReport::default();
        let stacks = loadout.inventory().stacks();
        let mut totals = Totals {
            encumbrance: Encumbrance {
                carried: 0.0,
                capacity: self.totals.encumbrance.capacity,
            },
            stacks: stacks.len(),
            ..Totals::default()
        };
        for stack in stacks {
            let slots = slots.remove(&stack.id).unwrap_or_default();
            let equipped = !slots.is_empty();
            let new_row = match self.list.get(&stack.id) {
                Some(old) if !force && old.stack == *stack => (old.slots != slots).then(|| {
                    report.updated += 1;
                    ItemRow {
                        carried: catalog.carried_weight(stack, &old.info, equipped),
                        slots,
                        ..old.clone()
                    }
                }),
                old => {
                    let info = catalog.describe(stack);
                    let row = ItemRow {
                        stack: stack.clone(),
                        carried: catalog.carried_weight(stack, &info, equipped),
                        info,
                        slots,
                    };
                    (old != Some(&row)).then(|| {
                        report.described += 1;
                        row
                    })
                }
            };
            let row = match new_row {
                Some(row) => {
                    totals.encumbrance.carried += row.carried;
                    totals.value = totals.value.saturating_add(row.total_value());
                    self.list.upsert(stack.id, row);
                    None
                }
                None => self.list.get(&stack.id),
            };
            if let Some(row) = row {
                totals.encumbrance.carried += row.carried;
                totals.value = totals.value.saturating_add(row.total_value());
            }
            totals.pieces += u64::from(stack.count);
        }
        // Every stack has a row now, so extra rows are exactly the stacks that left.
        if self.list.len() > stacks.len() {
            let present: HashSet<StackId> = stacks.iter().map(|s| s.id).collect();
            let gone: Vec<StackId> = self
                .list
                .iter()
                .map(|(k, _)| *k)
                .filter(|k| !present.contains(k))
                .collect();
            for k in gone {
                self.list.remove(&k);
                report.removed += 1;
            }
        }
        // Rows follow inventory order (a split stack sits after its source); moving rows
        // doesn't rebind them.
        let order: Vec<StackId> = stacks.iter().map(|s| s.id).collect();
        self.list.reorder(&order);
        self.totals = totals;
        report
    }

    // ------------------------------------------------------------------ tabs and search

    pub fn tabs(&self) -> &[InventoryTab] {
        &self.tabs
    }

    pub fn tab(&self) -> usize {
        self.tab
    }

    /// Selects a tab (clamped).
    pub fn set_tab(&mut self, index: usize) {
        let index = index.min(self.tabs.len() - 1);
        if index != self.tab {
            self.tab = index;
            self.apply_filter();
        }
    }

    /// Next (`+1`) or previous (`-1`) tab, wrapping (LB/RB, Q/E).
    pub fn cycle_tab(&mut self, step: isize) {
        let n = self.tabs.len().cast_signed();
        let next = (self.tab.cast_signed() + step).rem_euclid(n);
        self.set_tab(next.cast_unsigned());
    }

    /// Rows each tab would show, ignoring the search (badges; the controller preset dims
    /// empty tabs).
    pub fn tab_counts(&self) -> Vec<usize> {
        let filters: Vec<ListFilter> = self.tabs.iter().map(tab_filter).collect();
        let mut counts = vec![0; filters.len()];
        for (_, row) in self.list.iter() {
            for (count, filter) in counts.iter_mut().zip(&filters) {
                if filter.matches(row, &[]) {
                    *count += 1;
                }
            }
        }
        counts
    }

    pub fn search(&self) -> &str {
        &self.search
    }

    /// Search text: every word must appear in the name (case-insensitive).
    pub fn set_search(&mut self, text: &str) {
        if self.search != text {
            text.clone_into(&mut self.search);
            self.apply_filter();
        }
    }

    /// Fields the search looks in (default `name`; a mod may add `category`).
    pub fn set_search_fields(&mut self, fields: Vec<String>) {
        self.search_fields = fields;
        self.apply_filter();
    }

    fn apply_filter(&mut self) {
        let mut filter = tab_filter(&self.tabs[self.tab]);
        filter.search.clone_from(&self.search);
        filter.search_fields.clone_from(&self.search_fields);
        self.list.set_filter(filter);
        self.nav.reveal_focus();
    }

    // ------------------------------------------------------------------ sort

    pub fn sort(&self) -> Option<&InventorySort> {
        self.sort.as_ref()
    }

    /// Sorts by a row field (`None` = inventory order). Ties go by name, then inventory order.
    pub fn set_sort(&mut self, sort: Option<InventorySort>) {
        self.sort = sort;
        self.apply_sort();
    }

    /// Header click / sort button: a new column sorts in its default direction (text A→Z,
    /// numbers high→low: heaviest, most valuable first); the same column again flips it.
    /// Returns `false` for columns that don't sort (`SortKind::None`, Lua values until the
    /// Lua UI lands).
    pub fn sort_by_column(&mut self, column: &ColumnDef) -> bool {
        let ValueSource::Field(field) = &column.value else {
            return false;
        };
        let descending = match column.sort {
            SortKind::None => return false,
            SortKind::Text => false,
            SortKind::Number => true,
        };
        let sort = match &self.sort {
            Some(s) if s.field == *field => InventorySort {
                field: field.clone(),
                descending: !s.descending,
            },
            _ => InventorySort {
                field: field.clone(),
                descending,
            },
        };
        self.set_sort(Some(sort));
        true
    }

    /// Equipped rows first, whatever the sort (off by default).
    pub fn set_pin_equipped(&mut self, pin: bool) {
        self.pin_equipped = pin;
        self.apply_sort();
    }

    fn apply_sort(&mut self) {
        let spec = |field: &str, descending| SortSpec {
            field: field.into(),
            descending,
        };
        let mut keys = Vec::new();
        if self.pin_equipped {
            keys.push(spec("equipped", true));
        }
        if let Some(s) = &self.sort {
            keys.push(spec(&s.field, s.descending));
            if s.field != "name" {
                keys.push(spec("name", false));
            }
        }
        // Remaining ties keep inventory order (the list's insertion order).
        self.list.set_sort_keys(keys);
        self.nav.reveal_focus();
    }

    // ------------------------------------------------------------------ list navigation

    /// The virtualized list's scroll and pool state (viewport, wheel, scrollbar).
    pub fn nav(&mut self) -> &mut VirtualList<StackId> {
        &mut self.nav
    }

    pub fn focus(&self) -> Option<StackId> {
        self.nav.focus().copied()
    }

    pub fn set_focus(&mut self, stack: Option<StackId>) {
        self.nav.set_focus(stack);
    }

    /// D-pad / arrows: move focus by rows.
    pub fn move_focus(&mut self, delta: isize) {
        self.nav.move_focus(&mut self.list, delta);
    }

    /// Page up/down by `pages`.
    pub fn page(&mut self, pages: isize) {
        self.nav.page(&mut self.list, pages);
    }

    /// The rows to show this frame.
    pub fn frame(&mut self) -> ListFrame<StackId> {
        self.nav.update(&mut self.list)
    }

    /// Follows a host event for this holder: focus moves to the stack a split created or a
    /// merge kept. Returns whether the rows need a [`InventoryView::sync`].
    pub fn on_event(&mut self, event: &RpgEvent) -> bool {
        match event {
            RpgEvent::ItemsChanged { holder, .. } => *holder == self.holder,
            RpgEvent::StackSplit { holder, new, .. } if *holder == self.holder => {
                self.set_focus(Some(*new));
                true
            }
            RpgEvent::StacksMerged {
                holder, from, into, ..
            } if *holder == self.holder => {
                if self.focus() == Some(*from) {
                    self.set_focus(Some(*into));
                }
                true
            }
            _ => false,
        }
    }
}

fn tab_filter(tab: &InventoryTab) -> ListFilter {
    let mut filter = ListFilter::default();
    if !tab.categories.is_empty() {
        let values = tab
            .categories
            .iter()
            .map(|c| UiValue::Text(c.as_str().to_owned()))
            .collect();
        filter.one_of.push(("category".into(), values));
    }
    if tab.equipped_only {
        filter.equals.push(("equipped".into(), UiValue::Bool(true)));
    }
    filter
}
