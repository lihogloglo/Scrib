//! Fighters, vitals and the per-character fight state (combat.md "Fighter and vitals").

use serde::{Deserialize, Serialize};
use sf_action::{CharacterId, MovesetKey, RuleKey, Tick};
use smallvec::SmallVec;

use crate::hitstop::HitStop;
use crate::react::Tier;
use crate::rules::{PoolDelta, SwingPlan};
use crate::shape::{HurtShape, WeaponShape};

/// A bounded pool.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    pub cur: f32,
    pub max: f32,
}

impl Pool {
    pub fn full(max: f32) -> Self {
        Self { cur: max, max }
    }

    /// Adds `d`, clamped to `0..=max` (an overdraw stops at 0).
    pub fn add(&mut self, d: f32) {
        if d.is_finite() {
            self.cur = (self.cur + d).clamp(0.0, self.max);
        }
    }
}

/// The pools the framework knows; other pools stay content-side (`Outcome::events`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vitals {
    pub health: Pool,
    pub stamina: Pool,
    pub poise: Option<Pool>,
}

impl Vitals {
    pub fn apply(&mut self, d: PoolDelta) {
        self.health.add(d.health);
        self.stamina.add(d.stamina);
        if let Some(p) = &mut self.poise {
            p.add(d.poise);
        }
    }
}

/// Offence numbers the content fills (classic rules ignore them).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offense {
    pub attack: f32,
}

/// Defence numbers the content fills.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Defense {
    /// Share of damage absorbed, 0..1.
    pub absorb: f32,
    pub guard_absorb: f32,
    pub guard_stability: f32,
}

/// One combatant (the shell's `Combatant` component wraps it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fighter {
    pub id: CharacterId,
    pub moveset: MovesetKey,
    pub vitals: Vitals,
    pub weapon: WeaponShape,
    pub hurt: HurtShape,
    pub offense: Offense,
    pub defense: Defense,
    pub hostile: bool,
    /// The least an unguarded hit staggers (player `Small`: every hit staggers).
    pub min_tier: Tier,
}

impl Fighter {
    pub fn dead(&self) -> bool {
        self.vitals.health.cur <= 0.0
    }
}

/// The current swing of a combat move.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwingState {
    pub mov: RuleKey,
    pub start: Tick,
    pub plan: SwingPlan,
    /// `(hit row, defender)` pairs that landed: each lands once.
    pub landed: SmallVec<[(u8, CharacterId); 4]>,
    /// Hit rows whose window opened (their stamina is charged), a bit per row.
    pub charged: u32,
}

/// A character's combat state beside its `ActionState`: plain serde, replicated (host-only parts
/// in co-op: combat.md "Co-op").
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FightState {
    pub swing: Option<SwingState>,
    pub hit_stop: Option<HitStop>,
    /// Regeneration ran this tick.
    pub regen_open: bool,
    /// Stamina ran out: sprint is refused until it refills to [`FightState::SPRINT_RESUME`] of
    /// its maximum (hysteresis, so an empty pool doesn't flicker sprint on and off each tick).
    pub winded: bool,
}

impl FightState {
    /// Share of the stamina maximum at which a winded fighter may sprint again.
    pub const SPRINT_RESUME: f32 = 0.15;

    /// Updates [`Self::winded`] from the pool after this tick's vitals.
    pub fn update_winded(&mut self, stamina: Pool) {
        if stamina.cur <= 0.0 {
            self.winded = true;
        } else if stamina.cur >= stamina.max * Self::SPRINT_RESUME {
            self.winded = false;
        }
    }
}
