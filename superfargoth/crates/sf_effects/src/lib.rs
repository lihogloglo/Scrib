//! Magic effects, Bevy-free (`docs/specs/effects.md`): content defines effects ([`EffectTable`]
//! of [`EffectDef`]s, each an [`Archetype`] with data), every character holds an [`ActiveSet`] of
//! active sources, and the pure [`apply`] / [`tick`] step turns them into `sf_stats` ops with one
//! [`sf_stats::SourceId`] per effect ([`InstanceId::source_id`]).
//!
//! Canonical technique: spec-then-instance with aggregated modifiers (UE GAS gameplay effects,
//! Creation `MagicEffect`), grouped by source as `OpenMW` `ActiveSpellParams`. The step reads no
//! sheet and writes none: it returns [`EffectOut`] for the host to run through `sf_stats::apply`
//! in the same tick. No clock and no RNG inside (net rule 4): rolls come from the op's
//! [`RollSeed`], the tick and its length from [`EffectCtx`]. No game's effect names live here.
//!
//! Slice E1 (core): the stat archetypes, inflict / end / skip, stacking, events, [`TestRules`].
//! Slice E2 (rules seam): [`EffectRules`] with [`CastModel`], [`LandModel`], [`ItemModel`], and the
//! modern defaults [`StandardEffectRules`] as pure `sf.effects.*` rule functions.
//! Slice E3 (book + casting): [`SpellBook`] and [`SourceTable`], [`apply_book`] with `Cast`,
//! `Learn` / `Forget` / `Select`, cures, powers recharging on [`GameMinutes`], cooldowns, and the
//! `Request` archetype ([`EffectRequest`]). Items and the Bevy shell come later.

mod active;
mod book;
mod def;
mod op;
mod rules;
mod standard;
mod step;

pub use active::{
    ActiveEffect, ActiveSet, ActiveSource, ActorRef, EffectIdAlloc, InstanceFlags, InstanceId, ItemKey,
    MAX_EFFECTS_PER_SOURCE, Phase, SourceClass, SourceRef, StackKey,
};
pub use def::{
    ArgKind, ArgRef, Archetype, CureFilter, EffectDef, EffectFlags, EffectIdx, EffectKey, EffectTable, EffectTableError,
    LayerRule,
    PoolMode, StackPolicy, StatTarget, Tag,
};
pub use book::{BookRejection, GameMinutes, SourceDef, SourceFlags, SourceTable, SourceTableError, SpellBook};
pub use op::{
    Delivery, EffectCtx, EffectEntry, EffectEvent, EffectList, EffectOp, EffectOut, EffectRequest, EndReason, Range,
    Rejection, RequestPhase, RollSeed,
};
pub use rules::{
    CastFailReason, CastModel, CastPlan, CastQuery, EffectRules, ItemModel, ItemQuery, LandModel, ProtectQuery, Protection,
    ResistQuery, RollQuery, TestRules, roll,
};
pub use standard::{
    CastIn, CostConsts, CostEntryIn, ItemCharge, ItemIn, MagnitudeIn, ProtectIn, ResistIn, StandardEffectRules, cast_rule,
    entry_cost, item_charge_rule, magnitude_rule, protect_rule, resist_rule,
};
pub use step::{EffectEnv, apply, apply_book, tick};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_rules;
#[cfg(test)]
mod tests_book;
