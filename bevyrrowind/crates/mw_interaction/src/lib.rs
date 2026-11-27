//! Morrowind interaction adapter core (`bevyrrowind/docs/specs/interaction-adapter.md`), Bevy-free,
//! on `sf_interaction`:
//!
//! - [`furniture`]: census furniture kinds ([`Kind`], [`classify`]) from record ids and model names.
//! - [`defs`]: [`def_for`], a census kind plus its [`Measured`] shape to an `InteractableDef`
//!   (seats, benches, beds, tables, counters); [`door_def`], a swing door's def.
//! - [`measure`](mod@measure): [`measure()`], a census model's triangles and top to its [`Measured`]
//!   shape (seat front from the backrest, bed head from the boards; the bake calls it, `IN32b2`).
//! - [`activation`]: the per-record-class activation table ([`ROWS`]): verb, default action and
//!   Classic behaviour; today's prompts unchanged, unbuilt actions listed but never offered;
//!   [`runs_default`], whether a default handler acts on an approved use's verb.
//! - [`tunables`]: `mw.rules.interaction.sit`, the `mw.interact.*` furniture lengths
//!   ([`FurnitureParams`]) and the `mw.interact.measure.*` bake thresholds ([`MeasureParams`]).
//!
//! The bake fills [`Measured`] from the census through [`measure()`] and stores the defs as
//! `mw.interact.defs` (`mw_bake::interact`, `IN32b2`); the game shell indexes the defs and registers the verbs
//! (`IN32c`).

pub mod activation;
pub mod defs;
pub mod furniture;
pub mod measure;
pub mod tunables;

pub use activation::{
    Availability, Classic, Condition, DefaultAction, ROWS, RecordClass, Row, Switch, offered, primary, rows,
    runs_default,
};
pub use defs::{
    Basis, CLASS_VERB_PRIORITY, Facing, Measured, POSE_VERB_PRIORITY, USER_NPC, bed_verb, class_verb, def_for, door_def,
};
pub use furniture::{Kind, classify};
pub use measure::measure;
/// The def types [`def_for`] fills (re-exported so the bake needs no `sf_interaction` dependency).
pub use sf_interaction::{InteractableDef, Side, SlotDef, TagId};
pub use tunables::{FurnitureParams, MeasureParams, TUNABLES};

#[cfg(test)]
mod tests_activation;
#[cfg(test)]
mod tests_defs;
#[cfg(test)]
mod tests_measure;
