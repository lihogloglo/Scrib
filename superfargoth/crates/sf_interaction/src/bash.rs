//! Bashing (`superfargoth/docs/specs/interaction-destruction.md` "Bashing (IN37)", row IN37a): one blow on a
//! lock or a door, by strength, with a weapon or fists.
//!
//! - [`bash`]: what one [`Blow`] does under the adapter's [`BashRules`] (BG3-style: a damage threshold by the
//!   target's material, under it nothing, over it the full `floor(force x kind factor)`), plus the blow's cost
//!   to the basher (fists: self damage; armed: weapon wear) and the risk to fragile contents.
//! - [`Bashed`]: one landed blow, the event the shell sends; [`bash_seed`] its seeded stream (contents rolls),
//!   the same on host and clients.
//!
//! Crime owns the lock (its hp, broken, the offences): `lock_damage` is only a number here. The core never
//! names a lock, a stat or a game; the adapter sets the force, the kinds and the bare-handed kind.

use std::collections::BTreeMap;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_world::PlacementId;

use crate::claims::{Tick, UserKey};
use crate::def::TagId;
use crate::destroy::splitmix64;

/// The bash verb (a `VerbDef` whose `requires` the adapter sets: the core names no lock).
pub const VERB_BASH: TagId = TagId::of("bash");

/// One blow, as the adapter computes it at the press.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blow {
    /// The adapter's force (strength x weapon); non-finite or negative = a zero result.
    pub force: f32,
    /// The damage kind (blunt, blade, the game's hand kind ...).
    pub kind: TagId,
    /// Fists: the basher takes self damage instead of weapon wear.
    pub bare_handed: bool,
}

/// The adapter's bash record. The default is neutral: kind factor 1, threshold 0, no costs or risks.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BashRules {
    /// The kind a bare-handed blow uses (`None` = never bare-handed); the shell sets [`Blow::bare_handed`]
    /// from it.
    pub bare_kind: Option<TagId>,
    /// Damage factor by blow kind; an unknown kind = 1.
    pub kind_factor: BTreeMap<TagId, f32>,
    /// Damage threshold by the target def's material; an unknown or missing material = 0.
    pub threshold: BTreeMap<TagId, f32>,
    /// Self damage per unit force, bare-handed only.
    pub self_per_force: f32,
    /// Weapon wear per unit force, armed only.
    pub wear_per_force: f32,
    /// Chance per unit force that one fragile item inside breaks.
    pub contents_risk_per_force: f32,
    /// The cap of that chance.
    pub contents_risk_max: f32,
}

impl BashRules {
    fn factor(&self, kind: TagId) -> f32 {
        self.kind_factor.get(&kind).copied().unwrap_or(1.0)
    }

    fn threshold_of(&self, material: Option<TagId>) -> f32 {
        material.and_then(|m| self.threshold.get(&m).copied()).unwrap_or(0.0)
    }
}

/// What one blow did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BashResult {
    /// Lock hit points taken (crime applies them): 0 under the threshold, else `floor(force x factor)`.
    pub lock_damage: u32,
    /// Health the basher loses (bare-handed only).
    pub self_damage: f32,
    /// Weapon condition lost (armed only; the game rounds and floors it).
    pub weapon_wear: f32,
    /// Chance in `0 ..= contents_risk_max` that each fragile item inside breaks.
    pub contents_risk: f32,
}

/// One blow under `rules` on a target whose def's material is `material`. Pure.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn bash(rules: &BashRules, blow: &Blow, material: Option<TagId>) -> BashResult {
    let force = blow.force;
    if !force.is_finite() || force <= 0.0 {
        return BashResult::default();
    }
    let f = force * rules.factor(blow.kind);
    // `as u32` saturates (and maps NaN to 0): a non-finite factor cannot wrap.
    let lock_damage = if f.is_finite() && f >= rules.threshold_of(material) {
        f.floor().max(0.0) as u32
    } else {
        0
    };
    let (self_damage, weapon_wear) = if blow.bare_handed {
        (force * rules.self_per_force, 0.0)
    } else {
        (0.0, force * rules.wear_per_force)
    };
    BashResult {
        lock_damage,
        self_damage,
        weapon_wear,
        // The contents feel the raw blow, not the lock's share of it.
        contents_risk: (force * rules.contents_risk_per_force).min(rules.contents_risk_max),
    }
}

/// The seed of a blow: `h = 0; for w in [placement >> 64, placement as u64, user kind (0 actor, 1 player),
/// user key >> 64, user key as u64, tick] { h = splitmix64(h ^ w) }` (the [`crate::break_seed`] scheme). Pure,
/// so every client rolls the same contents for the same event.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn bash_seed(placement: PlacementId, user: UserKey, tick: Tick) -> u64 {
    let (who, key) = match user {
        UserKey::Actor(a) => (0u64, a.0),
        UserKey::Player(p) => (1u64, p.0),
    };
    let words = [(placement.0 >> 64) as u64, placement.0 as u64, who, (key >> 64) as u64, key as u64, tick.0];
    let mut h = 0u64;
    let mut i = 0;
    while i < words.len() {
        h = splitmix64(h ^ words[i]);
        i += 1;
    }
    h
}

/// One landed blow (the shell's `Bashed` message wraps it).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bashed {
    pub tick: Tick,
    pub placement: PlacementId,
    /// Who swung.
    pub culprit: UserKey,
    /// The blow's damage kind.
    pub kind: TagId,
    /// The blow's raw force.
    pub force: f32,
    pub result: BashResult,
    /// Where it landed, world metres (the door slot's doorway, else the object origin).
    pub point: Vec3,
    /// [`bash_seed`]`(placement, culprit, tick)`.
    pub seed: u64,
}
