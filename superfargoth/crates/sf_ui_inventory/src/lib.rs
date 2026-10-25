//! Bevy-free inventory screen logic (ui.md "As built (U5)"), no rendering.
//!
//! An [`InventoryView`] mirrors one holder's `sf_rpg` [`Loadout`](sf_rpg::Loadout) as keyed
//! `sf_ui` list rows ([`ItemRow`], view-model kind `inventory.item`) and adds what an
//! inventory, container or barter list needs on top:
//!
//! - category [tabs](InventoryTab), word search, column sort with fixed tie-breaks
//!   (name, then inventory order) and optional equipped-first pinning;
//! - equipped markers (the slots a stack is worn in) and [totals](Totals): carried
//!   weight against the carry limit ([`Encumbrance`]), value, pieces;
//! - [compare-with-equipped](Comparison): per-stat deltas against what equipping would
//!   displace;
//! - stack split and merge as `sf_rpg` commands, with a [`QuantityPicker`];
//! - the virtualized list (`sf_ui::VirtualList`): a few dozen pooled rows for thousands
//!   of stacks, keyed focus that survives sorting and filtering;
//! - the icon [grid](InventoryGrid): the same rows `columns` to a grid row, virtualized by
//!   grid rows with the same pool.
//!
//! Game data comes through the [`ItemCatalog`] adapter hook (names, categories, weights,
//! values, stats) and `sf_rpg`'s `EquipRules`; nothing here knows a game.

mod catalog;
mod command;
mod compare;
mod grid;
mod row;
mod view;

#[cfg(test)]
mod tests;

pub use catalog::{Better, InventoryTab, ItemCatalog, ItemInfo, StatDef};
pub use command::{QuantityPicker, StackActions};
pub use compare::{Comparison, StatDelta, Verdict};
pub use grid::{GridFrame, GridRow, GridRowBinding, InventoryGrid};
pub use row::{ItemRow, ROW_FIELDS, STAT_PREFIX};
pub use view::{Encumbrance, INVENTORY_LIST, InventorySort, InventoryView, SyncReport, Totals};
