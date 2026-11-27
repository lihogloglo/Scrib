//! The RPG layer in the streamed view: dialogue and journal glue (`Rpg`), item pickup and containers,
//! the inventory screen (app-split.md slice 9), a container window's open and close as a
//! `DoorChanged` (`container_door`), item use (potions) and the breakables' stage hook
//! (`breakable`, `IN38b2`).

pub mod breakable;
pub mod cast;
pub mod container_door;
pub mod dialogue;
pub mod effects;
pub mod inventory;
pub mod inventory_ui;
pub mod item_use;
pub mod stats;

#[cfg(test)]
mod tests_breakable;
