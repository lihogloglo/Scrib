//! Character stats, Bevy-free (`docs/specs/stats.md`): the content defines which stats exist
//! ([`StatTable`]), each character holds a [`Sheet`], every change is a [`StatOp`] run by the pure
//! [`apply`] / [`tick`] step, and the content's rules models ([`DerivedModel`], [`SkillUseModel`],
//! [`LevelingModel`]) derive maxima, regenerate, grow skills and level up.
//!
//! Canonical technique: actor values with layered, source-tracked modifiers (Creation Engine actor
//! values, UE GAS attribute aggregators). Modifiers are aggregated, never mutate-and-undo: removing
//! a source restores the exact prior sheet. Pool currents follow the semantics of `OpenMW`'s `DynamicStat::setCurrent`
//! flags ([`set_current`]). No clock and no RNG inside (net rule 4): models get a seed in [`Inputs`].
//! No game's stat names live here: content supplies them, and [`Role`]s name the few stats
//! framework consumers must find.

mod ops;
mod rules;
mod save;
mod sheet;
mod table;
mod test_model;

pub use ops::{ApplyCtx, Cause, CauseKind, StatEvent, StatOp, StatOwner, TickCtx, apply, set_current, tick};
pub use rules::{DerivedModel, IncreaseSource, Inputs, LevelOffer, LevelingModel, Refused, Rules, SkillUseModel};
pub use save::{CounterSave, LoadReport, PoolSave, SheetSave, ValueSave};
pub use sheet::{Layer, Membership, ModOp, Modifier, PoolStat, Settle, Sheet, SheetRead, SourceId, SourceKind, ValueStat};
pub use sf_ids::{ActorKey, PlayerKey};
pub use table::{Rebase, Role, RoleMap, StatDef, StatIdx, StatKey, StatKind, StatTable, TableError};
pub use test_model::TestModel;

#[cfg(test)]
mod tests;
