//! The rules seam (combat.md "The rules seam"): the framework owns timing and never computes
//! damage; a [`CombatRules`] implementor decides hit or miss, damage, costs and regeneration.
//! [`StandardRules`] is the modern default; classic rules live in the content pack (C5).

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_action::{ActionClass, CharacterId, RuleKey, TICK_HZ, TagId, Tick};
use smallvec::SmallVec;

use crate::fighter::Fighter;
use crate::react::{Side, Tier};
use crate::sheet::HitMask;

/// Pool changes (added: negative spends).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PoolDelta {
    pub health: f32,
    pub stamina: f32,
    pub poise: f32,
}

impl PoolDelta {
    pub const ZERO: Self = Self { health: 0.0, stamina: 0.0, poise: 0.0 };

    pub fn stamina(d: f32) -> Self {
        Self { stamina: d, ..Self::ZERO }
    }
}

/// A combat move about to swing.
#[derive(Clone, Copy, Debug)]
pub struct Swing<'a> {
    pub attacker: CharacterId,
    pub move_key: RuleKey,
    pub strength: f32,
    /// A cone query for cone-resolved moves (classic, C5); empty for sweeps.
    pub candidates: &'a [CharacterId],
}

/// What the rules decided for a swing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwingPlan {
    pub connects: bool,
    /// 0..1, scales costs and damage.
    pub strength: f32,
    pub target: Option<CharacterId>,
}

impl SwingPlan {
    pub const FULL: Self = Self { connects: true, strength: 1.0, target: None };
}

/// The defender's state at contact.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct DefenderState {
    pub guarded: bool,
    pub down: bool,
    pub unaware: bool,
    pub in_reaction: bool,
}

/// A sweep met a defender, after the i-frame and guard tests.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub attacker: CharacterId,
    pub defender: CharacterId,
    pub move_key: RuleKey,
    pub hit: u8,
    pub strength: f32,
    pub kind: TagId,
    pub mv: f32,
    pub flags: HitMask,
    /// Contact point, in the frame the shell placed the bodies in.
    pub point: Vec3,
    /// Planar unit direction attacker to defender.
    pub direction: Vec3,
    pub side: Side,
    pub defender_state: DefenderState,
}

/// What a contact does.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub landed: bool,
    pub defender: PoolDelta,
    pub attacker: PoolDelta,
    pub guard_break: bool,
    pub tier_hint: Option<Tier>,
    /// Content tags (other pools, effects).
    pub events: SmallVec<[TagId; 2]>,
}

impl Outcome {
    pub fn miss() -> Self {
        Self { landed: false, defender: PoolDelta::ZERO, attacker: PoolDelta::ZERO, guard_break: false, tier_hint: None, events: SmallVec::new() }
    }
}

/// When regeneration runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegenState {
    Open,
    /// Guard raised (C2).
    Guarding,
}

pub trait CombatRules: Send + Sync {
    /// Before the move's first hit window (classic decides hit or miss here). Modern: connects,
    /// strength 1.
    fn begin_swing(&self, swing: &Swing<'_>) -> SwingPlan;
    /// A sweep (or cone) met a defender, after the i-frame and guard tests.
    fn resolve_contact(&self, contact: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome;
    /// May `who` start a move of `class` now? Modern: stamina > 0 for costed classes. Classic:
    /// always.
    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool;
    /// Pool cost of a move start or a hit window opening (`base` from the sheet, `strength` 0..1).
    fn cost(&self, who: &Fighter, base: f32, strength: f32) -> PoolDelta;
    /// Per-tick regeneration while regen is open.
    fn regen(&self, who: &Fighter, state: RegenState) -> PoolDelta;
}

/// The modern rules: no hit roll, damage `attack x mv x strength x (1 - absorb)`, starts need
/// stamina > 0 (the last action may overdraw), costs = base, regen 45/s (x0.5 guarding).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardRules {
    pub regen_per_s: f32,
    pub guard_regen: f32,
}

impl Default for StandardRules {
    fn default() -> Self {
        Self { regen_per_s: 45.0, guard_regen: 0.5 }
    }
}

impl StandardRules {
    /// Classes whose start costs stamina.
    pub const COSTED: [ActionClass; 4] = [ActionClass::Light, ActionClass::Heavy, ActionClass::Offhand, ActionClass::Dodge];
}

impl CombatRules for StandardRules {
    fn begin_swing(&self, _swing: &Swing<'_>) -> SwingPlan {
        SwingPlan::FULL
    }

    fn resolve_contact(&self, c: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome {
        let damage = attacker.offense.attack * c.mv * c.strength * (1.0 - defender.defense.absorb.clamp(0.0, 1.0));
        Outcome { landed: true, defender: PoolDelta { health: -damage.max(0.0), ..PoolDelta::ZERO }, ..Outcome::miss() }
    }

    fn can_start(&self, who: &Fighter, class: ActionClass) -> bool {
        !Self::COSTED.contains(&class) || who.vitals.stamina.cur > 0.0
    }

    fn cost(&self, _who: &Fighter, base: f32, _strength: f32) -> PoolDelta {
        PoolDelta::stamina(-base)
    }

    fn regen(&self, _who: &Fighter, state: RegenState) -> PoolDelta {
        #[allow(clippy::cast_precision_loss)]
        let per_tick = self.regen_per_s / TICK_HZ as f32;
        PoolDelta::stamina(match state {
            RegenState::Open => per_tick,
            RegenState::Guarding => per_tick * self.guard_regen,
        })
    }
}

/// A deterministic random pick: a hash of (seed, character, tick), so every peer rolls the same.
pub fn roll(seed: u64, character: CharacterId, tick: Tick) -> u32 {
    // splitmix64 over the three inputs.
    let mut x = seed ^ character.0.rotate_left(17) ^ (u64::from(tick.0) << 1);
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    // The high half: the better-mixed bits.
    #[allow(clippy::cast_possible_truncation)]
    let r = (x >> 32) as u32;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_is_stable_and_spreads() {
        let a = roll(1, CharacterId(5), Tick(10));
        assert_eq!(a, roll(1, CharacterId(5), Tick(10)));
        assert_ne!(a, roll(1, CharacterId(5), Tick(11)));
        assert_ne!(a, roll(2, CharacterId(5), Tick(10)));
        let hi = (0..1000).filter(|&t| roll(0, CharacterId(1), Tick(t)) > u32::MAX / 2).count();
        assert!((400..600).contains(&hi), "{hi}");
    }
}
