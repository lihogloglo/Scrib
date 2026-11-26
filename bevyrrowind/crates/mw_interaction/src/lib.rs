//! Morrowind interaction adapter core (`bevyrrowind/docs/specs/interaction-adapter.md`), Bevy-free,
//! on `sf_interaction`:
//!
//! - [`furniture`]: census furniture kinds ([`Kind`], [`classify`]) from record ids and model names.
//! - [`defs`]: [`def_for`], a census kind plus its [`Measured`] shape to an `InteractableDef`
//!   (seats, benches, beds, tables, counters).
//! - [`activation`]: the per-record-class activation table ([`ROWS`]): verb, default action and
//!   Classic behaviour; today's prompts unchanged, unbuilt actions listed but never offered.
//! - [`tunables`]: `mw.rules.interaction.sit` and the `mw.interact.*` furniture lengths
//!   ([`FurnitureParams`]).
//!
//! The bake fills [`Measured`] from the census (`IN32b`); the game shell indexes the defs and
//! registers the verbs (`IN32c`).

pub mod activation;
pub mod defs;
pub mod furniture;
pub mod tunables;

pub use activation::{
    Availability, Classic, Condition, DefaultAction, ROWS, RecordClass, Row, Switch, offered, primary, rows,
};
pub use defs::{Facing, Measured, def_for};
pub use furniture::{Kind, classify};
pub use tunables::{FurnitureParams, TUNABLES};

#[cfg(test)]
mod tests_activation;
#[cfg(test)]
mod tests_defs;
