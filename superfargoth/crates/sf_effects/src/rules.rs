//! The rules seam (effects.md "The rules seam"): [`EffectRules`] bundles a [`CastModel`], a
//! [`LandModel`] and an [`ItemModel`]. Each model reads one query (sheets, defs, the source and
//! the uniform rolls the host drew), gathers its numbers into one rule input and calls a pure
//! `fn(&In) -> Out` (rule-hooks.md "Declaring a rule"; the modern defaults are in
//! [`crate::StandardEffectRules`]). No RNG and no clock inside: rolls and ticks come in.

use std::fmt;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_stats::{SheetRead, StatIdx};

use crate::active::{ActorRef, InstanceFlags, SourceClass};
use crate::def::{EffectDef, EffectKey, EffectTable};
use crate::op::{EffectEntry, RollSeed};

/// One entry's magnitude roll.
#[derive(Clone, Copy, Debug)]
pub struct RollQuery<'a> {
    pub def: &'a EffectDef,
    pub entry: &'a EffectEntry,
    /// Uniform in `[0, 1)`, drawn from the landing's seed.
    pub rolls: [f32; 2],
}

/// One entry's resistance.
#[derive(Clone, Copy)]
pub struct ResistQuery<'a> {
    pub def: &'a EffectDef,
    pub entry: &'a EffectEntry,
    pub caster: Option<ActorRef>,
    pub rolls: [f32; 2],
    /// The target's sheet (E2). `None` while the step reads no sheet (E1 step): no resistance
    /// from stats.
    pub target: Option<SheetRead<'a>>,
}

impl fmt::Debug for ResistQuery<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResistQuery")
            .field("def", &self.def.key)
            .field("entry", self.entry)
            .field("caster", &self.caster)
            .field("rolls", &self.rolls)
            .field("target", &self.target.is_some())
            .finish()
    }
}

/// What protection does to a landing source (pipeline step 3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Protection {
    /// The source lands normally.
    #[default]
    Applied,
    /// The source is re-inflicted on its caster with `from_reflect` (no second reflect).
    Reflected,
    /// The source is dropped; the target gains `gain` on its `Mana` role (a `PoolAdjust`).
    Absorbed { gain: f32 },
}

/// One source's protection check on one target. The host asks only for temporary sources from
/// another actor (effects.md pipeline step 3); the model reads the flags and the defs.
#[derive(Clone, Copy)]
pub struct ProtectQuery<'a> {
    pub defs: &'a EffectTable,
    pub source: &'a RecordId,
    pub class: SourceClass,
    pub entries: &'a [EffectEntry],
    pub caster: Option<ActorRef>,
    pub target: SheetRead<'a>,
    pub flags: InstanceFlags,
    /// What the source cost its caster (the cast plan's cost; 0 for a free source).
    pub cost: f32,
    /// Uniform in `[0, 1)`: `[absorb, reflect]` in the standard rules.
    pub rolls: [f32; 2],
}

/// How entries land. Implementors call pure rule functions; no RNG or clock (rolls come in).
pub trait LandModel: Send + Sync {
    /// The rolled magnitude, once per entry.
    fn magnitude(&self, q: &RollQuery<'_>) -> f32;
    /// A magnitude multiplier; 0 = fully resisted. A `no_magnitude` effect is resisted when
    /// `rolls[1] >=` the multiplier (all or nothing).
    fn resist(&self, q: &ResistQuery<'_>) -> f32;
    /// Reflect, absorb or apply one source (E2). The default applies everything, so an E1
    /// implementor keeps compiling.
    fn protect(&self, _q: &ProtectQuery<'_>) -> Protection {
        Protection::Applied
    }
}

/// Why a cast did not go off. `plan` returns the first three (nothing spent); the host's chance
/// roll gives `Fizzle` (cost spent).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CastFailReason {
    Silenced,
    /// The caster's `Mana` pool does not cover the cost.
    NoMana,
    /// The book's cooldown (or power recharge) has not run out.
    Recharging,
    Fizzle,
    /// The book does not hold the source (E3; the step refuses before the plan).
    NotKnown,
    /// No `SourceDef` for the record (E3).
    UnknownSource,
    /// Not a spell or power, or the step has no caster sheet to plan with (E3).
    NotCastable,
}

/// The cast model's answer (effects.md "Casting").
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CastPlan {
    /// Nothing is spent; the host emits `CastFailed { reason }`.
    Refused(CastFailReason),
    /// Spend `cost` from the caster's `Mana`, roll `rolls[0] < chance`, then start a cooldown of
    /// `cooldown` fixed ticks (0 = none).
    Ready { cost: f32, chance: f32, cooldown: u32 },
}

/// One cast of one source by one caster, at the release tick.
#[derive(Clone, Copy)]
pub struct CastQuery<'a> {
    pub defs: &'a EffectTable,
    pub source: &'a RecordId,
    pub class: SourceClass,
    pub entries: &'a [EffectEntry],
    /// The source's fixed cost (`SourceDef::cost`); `None` = computed from the entries.
    pub cost: Option<f32>,
    /// `SourceDef` `always_succeeds`: the chance is 1 whatever the rule says.
    pub always_succeeds: bool,
    /// The source's cooldown in fixed ticks (a `SourceDef` field, E3); 0 = none.
    pub cooldown: u32,
    /// The fixed tick the book says the source is usable again; `None` = ready.
    pub ready_at: Option<u32>,
    /// The fixed tick of the cast.
    pub tick: u32,
    pub caster: SheetRead<'a>,
    /// The skill the source's school uses, resolved by content; `None` = no skill.
    pub skill: Option<StatIdx>,
}

/// Casting: cost, chance, cooldown and refusals. Pure; the host spends and rolls.
pub trait CastModel: Send + Sync {
    /// Cost, chance (0..=1) and cooldown of casting `source` now, or why it is refused
    /// (silenced, no magicka, recharging).
    fn plan(&self, q: &CastQuery<'_>) -> CastPlan;
}

/// One enchanted item's charge numbers (item state lives in `sf_rpg`; E5 spends it).
#[derive(Clone, Copy)]
pub struct ItemQuery<'a> {
    pub defs: &'a EffectTable,
    pub source: &'a RecordId,
    pub entries: &'a [EffectEntry],
    /// The record's fixed charge per use; `None` = computed from the entries.
    pub cost: Option<f32>,
    /// The record's fixed maximum charge (content: soul size, record value); `None` = computed.
    pub capacity: Option<f32>,
    /// Who uses or strikes with the item, when it matters.
    pub user: Option<SheetRead<'a>>,
    /// The user's skill that lowers the charge cost, resolved by content.
    pub skill: Option<StatIdx>,
}

/// Item charge: per use or per strike, and the maximum.
pub trait ItemModel: Send + Sync {
    /// Charge spent per use / per strike.
    fn charge_cost(&self, q: &ItemQuery<'_>) -> f32;
    /// The item's maximum charge.
    fn max_charge(&self, q: &ItemQuery<'_>) -> f32;
}

/// The effect rules a game runs with: chosen per new game and stored with the world meta's
/// rule-set id, like `sf_stats::Rules`.
pub struct EffectRules {
    pub cast: Box<dyn CastModel>,
    pub land: Box<dyn LandModel>,
    pub items: Box<dyn ItemModel>,
}

impl EffectRules {
    /// The modern defaults ([`crate::StandardEffectRules`] with no content stats).
    pub fn standard() -> Self {
        Self::from_model(&crate::StandardEffectRules::default())
    }

    /// One model value behind all three seams.
    pub fn from_model<M>(model: &M) -> Self
    where
        M: CastModel + LandModel + ItemModel + Clone + 'static,
    {
        Self { cast: Box::new(model.clone()), land: Box::new(model.clone()), items: Box::new(model.clone()) }
    }
}

impl Default for EffectRules {
    fn default() -> Self {
        Self::standard()
    }
}

impl fmt::Debug for EffectRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EffectRules").finish_non_exhaustive()
    }
}

/// Fixed, readable rules for tests (this crate's, the shell's, adapters' smoke tests): the
/// magnitude is `min + (max - min) · rolls[0]`, the resist multiplier is looked up per effect
/// (1 when absent), protection is [`TestRules::protect`] for every source, a cast is `cast`
/// (or, when `None`, `Ready` at the fixed cost or 0, chance 1, the query's cooldown), an item
/// charges its fixed cost and holds its fixed capacity (0 when absent).
#[derive(Clone, Debug, Default)]
pub struct TestRules {
    pub resist: Vec<(EffectKey, f32)>,
    pub protect: Protection,
    pub cast: Option<CastPlan>,
}

impl LandModel for TestRules {
    fn magnitude(&self, q: &RollQuery<'_>) -> f32 {
        let (min, max) = q.entry.magnitude;
        min + (max - min) * q.rolls[0]
    }

    fn resist(&self, q: &ResistQuery<'_>) -> f32 {
        self.resist.iter().find(|(k, _)| *k == q.def.key).map_or(1.0, |&(_, m)| m)
    }

    fn protect(&self, _q: &ProtectQuery<'_>) -> Protection {
        self.protect
    }
}

impl CastModel for TestRules {
    fn plan(&self, q: &CastQuery<'_>) -> CastPlan {
        self.cast.unwrap_or(CastPlan::Ready { cost: q.cost.unwrap_or(0.0), chance: 1.0, cooldown: q.cooldown })
    }
}

impl ItemModel for TestRules {
    fn charge_cost(&self, q: &ItemQuery<'_>) -> f32 {
        q.cost.unwrap_or(0.0)
    }

    fn max_charge(&self, q: &ItemQuery<'_>) -> f32 {
        q.capacity.unwrap_or(0.0)
    }
}

/// splitmix64 (Steele, Lea, Flood 2014): a stateless, well-mixed hash of the input.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Roll `salt` of entry `index` from a landing's seed: uniform in `[0, 1)`, the same for the
/// same inputs on every machine.
#[allow(clippy::cast_precision_loss, reason = "a 24-bit integer is exact in f32")]
pub fn roll(seed: RollSeed, index: u8, salt: u8) -> f32 {
    let bits = mix(seed.0 ^ ((u64::from(index) << 8) | u64::from(salt))) >> 40;
    bits as f32 / 16_777_216.0
}
