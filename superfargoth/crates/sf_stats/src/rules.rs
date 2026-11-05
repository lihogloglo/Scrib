//! The rules seam (stats.md "The rules seam"): the content implements derivation, regeneration,
//! skill use and levelling; the core owns order, clamping and events.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::ops::StatOp;
use crate::sheet::SheetRead;
use crate::table::StatIdx;

/// External numbers the content's models need, supplied by the caller (no clock or RNG inside
/// `sf_stats`, net rule 4). What `extra` holds is the content's convention.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Inputs {
    /// For models that need randomness: a hash of (owner, tick) from the host.
    pub seed: u64,
    pub is_player: bool,
    pub in_combat: bool,
    /// Combat owns this character's stamina regen (stats.md "Consumers").
    pub combatant: bool,
    /// Carried weight in the content's units.
    pub carried: f32,
    pub extra: SmallVec<[f32; 4]>,
}

impl Inputs {
    /// Whether two inputs derive the same caches (the seed doesn't feed derivation).
    pub(crate) fn same_derivation(&self, other: &Self) -> bool {
        self.is_player == other.is_player
            && self.in_combat == other.in_combat
            && self.combatant == other.combatant
            && self.carried.to_bits() == other.carried.to_bits()
            && self.extra.len() == other.extra.len()
            && self.extra.iter().zip(&other.extra).all(|(a, b)| a.to_bits() == b.to_bits())
    }

    pub(crate) fn for_derivation(&self) -> Self {
        Self { seed: 0, ..self.clone() }
    }
}

/// Where a skill level came from (or went to: `Penalty` lowers it).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncreaseSource {
    Usage,
    Training,
    Book,
    Script,
    /// A loss imposed by a game system (time served in jail, a curse).
    Penalty,
    Debug,
}

/// What a level-up screen offers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LevelOffer {
    /// How many stats the player picks.
    pub picks: u8,
    /// The gain each pickable stat would get.
    pub gains: Vec<(StatIdx, f32)>,
}

/// Why a level-up was refused.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum Refused {
    #[error("no level-up is due")]
    NotReady,
    #[error("expected {expected} picks, got {got}")]
    PickCount { expected: u8, got: u8 },
    #[error("stat {0:?} can't be picked")]
    BadPick(StatIdx),
}

/// Derived maxima, derived stats, regeneration and rest.
pub trait DerivedModel: Send + Sync {
    /// A pool's base (max before modifiers) derived from other stats; `None` = the stored base
    /// (changed only by ops).
    fn pool_base(&self, s: &SheetRead<'_>, pool: StatIdx, inputs: &Inputs) -> Option<f32>;
    /// A `Derived` stat's value (carry capacity, encumbrance, a stamina term). Derived stats are
    /// computed in table order after values and pools.
    fn derived(&self, s: &SheetRead<'_>, stat: StatIdx, inputs: &Inputs) -> f32;
    /// Per fixed tick regeneration (`dt` = tick length in seconds): `PoolAdjust` / `PoolSetCurrent`
    /// ops only (others are ignored). Called only when some pool is below its max.
    fn regen(&self, s: &SheetRead<'_>, dt: f32, inputs: &Inputs, out: &mut SmallVec<[StatOp; 3]>);
    /// Rest or wait for `hours` (`sleep` = in a bed or an allowed rest).
    fn rest(&self, s: &SheetRead<'_>, hours: f32, sleep: bool, inputs: &Inputs, out: &mut SmallVec<[StatOp; 4]>);
}

/// Skill progression by use (`OpenMW`'s `I.SkillProgression` handler seam).
pub trait SkillUseModel: Send + Sync {
    /// One use: returns the gain (reported in `SkillUsed`) and pushes its ops (the progress
    /// counter; a `RaiseSkill` when a level is due).
    fn use_skill(
        &self,
        s: &SheetRead<'_>,
        skill: StatIdx,
        use_kind: u8,
        scale: f32,
        inputs: &Inputs,
        out: &mut SmallVec<[StatOp; 2]>,
    ) -> f32;
    /// Ops for one skill level gained or lost from `source` (base, level progress, tallies);
    /// pushes nothing to refuse (a cap).
    fn raise(&self, s: &SheetRead<'_>, skill: StatIdx, source: IncreaseSource, out: &mut SmallVec<[StatOp; 6]>);
}

/// Character levelling.
pub trait LevelingModel: Send + Sync {
    /// A level-up is due.
    fn ready(&self, s: &SheetRead<'_>) -> bool;
    /// What the level-up screen offers.
    fn offer(&self, s: &SheetRead<'_>) -> LevelOffer;
    /// The ops of one level-up with these picks, or why not.
    fn level_up(&self, s: &SheetRead<'_>, picks: &[StatIdx], out: &mut SmallVec<[StatOp; 8]>) -> Result<(), Refused>;
}

/// One rule set (chosen per new game, stored in the save's meta as a rule-set id).
pub struct Rules {
    pub derived: Box<dyn DerivedModel>,
    pub skill_use: Box<dyn SkillUseModel>,
    pub leveling: Box<dyn LevelingModel>,
}
