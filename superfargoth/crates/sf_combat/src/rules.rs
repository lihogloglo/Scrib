//! The rules seam (combat.md "The rules seam"): the framework owns timing and never computes
//! damage; a [`CombatRules`] implementor decides hit or miss, damage, costs and regeneration.
//! [`StandardRules`] is the modern default; classic rules live in the content pack (C5).
//!
//! Guard (C2b): the framework tests a raised guard and its arc before the rules
//! ([`DefenderState::guarded`]); the rules decide what a guarded hit costs and report
//! [`Outcome::guarded`] and [`Outcome::guard_break`], which pick the guard reaction instead of a hit
//! reaction. The classic seam (C5, not built): Morrowind has no held guard, so a classic
//! `resolve_contact` rolls the block itself (`OpenMW` `apps/openmw/mwmechanics/combat.cpp`
//! `blockMeleeAttack` at `ca5e2576`: a shield in the left hand, the attacker inside
//! `fCombatBlockLeftAngle..fCombatBlockRightAngle` of the facing from `Contact::direction`, a Block
//! skill roll with `fBlockStillBonus` when standing still) and sets `Outcome::guarded` on success:
//! the same guard reaction then plays (`Shield@block`), with no guard move or arc from here.

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
    /// A raised guard (past its raise time) with the attacker inside its arc (C2b).
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
    /// The hit row's stamina damage when guarded, before the rules scale it.
    pub guard_damage: f32,
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
    /// The hit was guarded: the guard reaction plays instead of a hit reaction (C2b).
    pub guarded: bool,
    /// A guarded hit broke the guard: the forced guard-break reaction.
    pub guard_break: bool,
    pub tier_hint: Option<Tier>,
    /// Content tags (other pools, effects).
    pub events: SmallVec<[TagId; 2]>,
}

impl Outcome {
    pub fn miss() -> Self {
        Self {
            landed: false,
            defender: PoolDelta::ZERO,
            attacker: PoolDelta::ZERO,
            guarded: false,
            guard_break: false,
            tier_hint: None,
            events: SmallVec::new(),
        }
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
/// stamina > 0 (the last action may overdraw), costs = base, regen 45/s (x0.5 guarding). Guarded
/// (C2b): damage also x `(1 - guard_absorb)`, stamina damage `guard_damage x guard_stamina /
/// guard_stability`, and the guard breaks when that empties the pool (at 0 exactly too).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardRules {
    pub regen_per_s: f32,
    /// Regeneration multiplier while guarding.
    pub guard_regen: f32,
    /// Share of a guarded hit's `guard_damage` the defender's stamina takes at stability 1.
    #[serde(default = "guard_stamina")]
    pub guard_stamina: f32,
}

fn guard_stamina() -> f32 {
    0.55
}

impl Default for StandardRules {
    fn default() -> Self {
        Self { regen_per_s: 45.0, guard_regen: 0.5, guard_stamina: guard_stamina() }
    }
}

impl StandardRules {
    /// Classes whose start costs stamina.
    pub const COSTED: [ActionClass; 4] = [ActionClass::Light, ActionClass::Heavy, ActionClass::Offhand, ActionClass::Dodge];
    /// The least `Defense::guard_stability` counts as (an unset 0 would divide by zero).
    pub const MIN_STABILITY: f32 = 0.1;

    /// A guarded hit's stamina damage on `defender` (positive).
    pub fn guard_stamina_damage(&self, guard_damage: f32, defender: &Fighter) -> f32 {
        (guard_damage.max(0.0) * self.guard_stamina / defender.defense.guard_stability.max(Self::MIN_STABILITY)).max(0.0)
    }
}

impl CombatRules for StandardRules {
    fn begin_swing(&self, _swing: &Swing<'_>) -> SwingPlan {
        SwingPlan::FULL
    }

    fn resolve_contact(&self, c: &Contact, attacker: &Fighter, defender: &Fighter) -> Outcome {
        let mut damage = attacker.offense.attack * c.mv * c.strength * (1.0 - defender.defense.absorb.clamp(0.0, 1.0));
        if !c.defender_state.guarded {
            return Outcome { landed: true, defender: PoolDelta { health: -damage.max(0.0), ..PoolDelta::ZERO }, ..Outcome::miss() };
        }
        damage *= 1.0 - defender.defense.guard_absorb.clamp(0.0, 1.0);
        let stamina = self.guard_stamina_damage(c.guard_damage, defender);
        let guard_break = defender.vitals.stamina.cur - stamina <= 0.0;
        Outcome {
            landed: true,
            defender: PoolDelta { health: -damage.max(0.0), stamina: -stamina, poise: 0.0 },
            guarded: true,
            guard_break,
            ..Outcome::miss()
        }
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
