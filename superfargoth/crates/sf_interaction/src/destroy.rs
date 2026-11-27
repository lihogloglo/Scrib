//! Destruction core, tier 1 (interaction-destruction.md "Tier 1: stage swap", IN1a): the pure maths
//! of a hit on a [`DestructibleDef`], the per-client debris distance gate, the host's per-tick break
//! queue and the health + stage rest state. Bethesda `DEST` stages (a stage starts at a health
//! fraction; one hit may cross several) and UE Chaos debris removal by distance; no code ported.
//!
//! - [`apply_damage`]: health after a hit, the stages it crossed (in order, several per hit) and the
//!   overkill fraction the adapter's spill curve damages the contents by.
//! - [`debris_lod`]: whether a client spawns debris bodies for a break at a distance from its camera
//!   (beyond `interact.debris.lod_m`: the stage swap and the effect only).
//! - [`BreakQueue`]: breaks over `interact.debris.breaks_per_tick` wait for the next tick, in order.
//! - [`DestructState::rest_bytes`] / [`DestructState::from_rest_bytes`]: the state at rest, stored as the
//!   [`DESTRUCT_STATE_OWNER`] state blob of its placement.
//! - [`Destroyed`]: one released break (`IN1b`), the event the shell wraps; [`break_seed`] its debris seed,
//!   the same on host and clients (`splitmix64` over placement, hit tick and stage).
//!
//! The adapter's `DestructionRules` turns (source, damage type, the def's `InteractableDef::material`) into
//! the damage number; nothing here names a stat, a source or a game.

use std::collections::VecDeque;
use std::ops::Range;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_world::PlacementId;

use crate::claims::{Tick, UserKey};
use crate::def::{DestructibleDef, TagId};

/// The owner name of a destructible's rest state among a placement's state blobs.
pub const DESTRUCT_STATE_OWNER: &str = "destruct";

/// Version byte of a destructible's rest state; v1 = `[version, stage, health as f32 LE (4 bytes)]`.
pub const DESTRUCT_REST_VERSION: u8 = 1;

/// The damage kind of a script's hit: the only kind an `ignore_external` def takes (Bethesda's
/// "ignore external damage": only a quest breaks it).
pub const DAMAGE_SCRIPT: TagId = TagId::of("script");

/// The damage kind of a physics impact (interaction-destruction.md "Impacts (IN4)"): the adapter's
/// `(impact, material)` factor scales it (stone 0, glass high).
pub const DAMAGE_IMPACT: TagId = TagId::of("impact");

/// One break the host released (interaction-destruction.md "Shell and game"): the stage swap, the
/// spill, the debris and the effect all start from it. "Net" sends its first six fields.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destroyed {
    /// The tick of the hit (not of the release: the break queue may delay it).
    pub tick: Tick,
    pub placement: PlacementId,
    /// Index into the def's `stages` of the stage crossed.
    pub stage: u8,
    /// Where the hit landed, world metres.
    pub point: Vec3,
    /// The hit's direction, world.
    pub dir: Vec3,
    /// [`break_seed`]`(placement, tick, stage)`: the debris stream.
    pub seed: u64,
    /// Who hit it; `None` = a rule (fire) or a script.
    pub culprit: Option<UserKey>,
    /// The hit's overkill on the last stage it crossed, 0 on the others ([`Hit::overkill`]).
    pub overkill: f32,
    /// The stage removes the object (its `model` is `None`).
    pub removes: bool,
    /// The hit's damage kind (`PropDamage::kind`): a `WorldChange`'s cause, a melee break's reveal.
    pub kind: TagId,
}

/// One splitmix64 step (Steele, Lea, Flood 2014; the `SplittableRandom` finaliser): mixes `z`
/// into a well spread `u64`.
#[must_use]
pub const fn splitmix64(z: u64) -> u64 {
    let mut z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The debris seed of a break: `h = 0; for w in [placement >> 64, placement as u64, tick, stage] {
/// h = splitmix64(h ^ w) }`. Pure, so every client spawns the same pieces for the same event.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn break_seed(placement: PlacementId, tick: Tick, stage: u8) -> u64 {
    let words = [(placement.0 >> 64) as u64, placement.0 as u64, tick.0, stage as u64];
    let mut h = 0u64;
    let mut i = 0;
    while i < words.len() {
        h = splitmix64(h ^ words[i]);
        i += 1;
    }
    h
}

/// One destructible's state: its health and how many of its def's stages it has reached.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestructState {
    /// Health left, `0 ..= def.health`.
    pub health: f32,
    /// Stages reached: 0 intact, `k` = the def's `stages[k - 1]` is the current one.
    pub stage: u8,
}

/// What one hit did ([`apply_damage`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    /// The state after the hit.
    pub state: DestructState,
    /// Indices into the def's `stages` this hit crossed, in order (empty: no stage change).
    pub crossed: Range<u8>,
    /// Damage past zero health as a fraction of the def's full health (0 when health is left);
    /// the adapter's spill curve damages the contents by it (a vaporised crate leaves little).
    pub overkill: f32,
}

impl DestructState {
    /// A fresh, unhit object of `def`.
    #[must_use]
    pub fn intact(def: &DestructibleDef) -> Self {
        Self { health: def.health.max(0.0), stage: 0 }
    }

    /// The state at rest as `[DESTRUCT_REST_VERSION, stage, health f32 LE]`.
    #[must_use]
    pub fn rest_bytes(&self) -> [u8; 6] {
        let h = self.health.to_le_bytes();
        [DESTRUCT_REST_VERSION, self.stage, h[0], h[1], h[2], h[3]]
    }

    /// The state that [`Self::rest_bytes`] wrote; `None` for any other length or version, or a
    /// negative or non-finite health. A stage past the def's count is the shell's to clamp (the
    /// def may have changed under an old save).
    #[must_use]
    pub fn from_rest_bytes(bytes: &[u8]) -> Option<Self> {
        let &[DESTRUCT_REST_VERSION, stage, a, b, c, d] = bytes else { return None };
        let health = f32::from_le_bytes([a, b, c, d]);
        (health.is_finite() && health >= 0.0).then_some(Self { health, stage })
    }
}

/// Applies `damage` to an object of `def` in `state`. Health floors at zero; every stage from the
/// current one whose `at` the new health fraction has reached (`fraction <= at`) is crossed, in the
/// def's order (descending `at`, the cook's job), so one hard hit crosses several. A stage once
/// crossed is never crossed again. Zero, negative or non-finite damage changes nothing. A def with
/// no health (`<= 0`) breaks through every stage at the first hit.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn apply_damage(def: &DestructibleDef, state: DestructState, damage: f32) -> Hit {
    let from = state.stage;
    if !(damage.is_finite() && damage > 0.0) {
        return Hit { state, crossed: from..from, overkill: 0.0 };
    }
    let health = (state.health - damage).max(0.0);
    let fraction = if def.health > 0.0 { health / def.health } else { 0.0 };
    let overkill = if def.health > 0.0 { (damage - state.health).max(0.0) / def.health } else { 0.0 };
    // Stage counts fit a u8 (a def holds a handful; the codec stores one byte).
    let total = def.stages.len().min(usize::from(u8::MAX)) as u8;
    let mut to = from.min(total);
    while to < total && fraction <= def.stages[usize::from(to)].at {
        to += 1;
    }
    Hit { state: DestructState { health, stage: to }, crossed: from.min(to)..to, overkill }
}

/// Whether a client spawns debris bodies for a break `distance_m` from its camera: within `lod_m`
/// (inclusive) yes; beyond it, or a non-finite distance, only the stage swap and the effect.
#[must_use]
pub fn debris_lod(distance_m: f32, lod_m: f32) -> bool {
    distance_m.is_finite() && distance_m <= lod_m
}

/// The host's queue of pending breaks: at most `breaks_per_tick` are applied per tick, the rest
/// carry to the next tick in arrival order (the shell pushes in its sorted same-tick order, so the
/// result is deterministic).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BreakQueue<T> {
    pending: VecDeque<T>,
}

impl<T> BreakQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { pending: VecDeque::new() }
    }

    /// Queues one break behind those already waiting.
    pub fn push(&mut self, item: T) {
        self.pending.push_back(item);
    }

    /// This tick's breaks: the first `breaks_per_tick` waiting, oldest first.
    pub fn take_tick(&mut self, breaks_per_tick: u8) -> impl Iterator<Item = T> + '_ {
        let n = usize::from(breaks_per_tick).min(self.pending.len());
        self.pending.drain(..n)
    }

    /// Breaks still waiting.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
