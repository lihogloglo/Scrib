//! `AI8f1` search core (`docs/specs/ai.md` "Perception": the Searching band runs the HTN `search`
//! at the last known position: look, widen, give up on ttl). Bevy-free; `sf_ai_bevy` (`AI8f2`) runs
//! the goals through the existing `MoveTo` / `Face` executors and filters points by nav.
//!
//! **Pattern** (the industry's searched-area model: TLOU2's search at the last known position,
//! Splinter Cell's widening search around the LKP, The Dark Mod's `search` state that picks random
//! points in a radius growing with time and gives up after the AI's search diligence):
//! 1. [`SearchGoal::GoTo`] the LKP (skipped when `reachable` rejects it), then a
//!    [`SearchGoal::LookAround`] there.
//! 2. Then rings: a ring is [`SearchPatternConfig::ring_points`] points evenly spaced in yaw around
//!    the LKP (a seeded rotation and jitter), each at a seeded distance in
//!    `[inner_frac, 1] × r(t)`, `t` the seconds since the search started when the ring is drawn.
//!    Each reachable point is a `GoTo` followed by a `LookAround` facing away from the LKP (where a
//!    fleeing target went). An exhausted ring draws the next at the then-current (larger) radius.
//! 3. [`SearchGoal::Done`] on the first call at or after the ttl; the awareness meter then decays
//!    the actor out of Searching (`AI8a`).
//!
//! **Radius rule:** `r(t) = (error_m + widen_mps × t) × (1 + (1 − c) × (uncertain_mul − 1))`, `c`
//! the LKP's confidence (0..1): a sighting (`c = 1`) searches the profile's own radius, a vague
//! noise (`c = 0`) `uncertain_mul` times it. Radii only grow, so every point lies within `r(now)`.
//!
//! **Determinism:** every draw is a splitmix64 hash of [`search_seed`] (`(actor, start tick)`, as
//! the sight stagger) and the ring and slot, never call order or a global RNG.
//!
//! **HTN shape:** one primitive operator [`OP_SEARCH`] (content: a `search` method with one
//! primitive task on it). The search is an open-ended loop whose next step depends on reachability
//! and time, which a static method list of `MoveTo` / `Face` tasks can't express without a replan per
//! point; so `AI8f2`'s `search` executor owns a [`SearchState`] and steps the existing executors
//! through [`SearchGoal::operators`], succeeding on `Done`.
//!
//! **Saves:** a running Search directive is saved (ai.md "Saves", LKP in its params), so
//! [`SearchState`] is serde; on load the actor starts at the Searching floor.

use std::f32::consts::TAU;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::{Location, WorldspaceId};
use thiserror::Error;

use crate::exec::{ExecGait, FaceTarget, OperatorKind, wrap_angle, yaw_toward};
use crate::htn::Operators;
use crate::ids::OperatorId;
use crate::perception::PerceptClock;
use crate::profile::SearchParams;
use crate::sight::{actor_hash, mix64, seconds_to_ticks};

/// Built-in operator name for the search executor (`AI8f2`).
pub const OP_SEARCH: &str = "search";

/// Registers [`OP_SEARCH`] in `ops` and returns its id.
pub fn register_search_operator(ops: &mut Operators) -> OperatorId {
    ops.add(OP_SEARCH)
}

/// The search pattern's tunables (`ai.perception.*`, bound in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchPatternConfig {
    /// Points per ring (range 1-16).
    pub ring_points: u8,
    /// Half-arc of a look-around, degrees (range 0-180).
    pub look_half_deg: f32,
    /// Radius multiplier at confidence 0 (range 1-4).
    pub uncertain_mul: f32,
    /// A ring point's nearest distance as a fraction of the ring radius (range 0-1).
    pub inner_frac: f32,
}

impl Default for SearchPatternConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.search_ring_points in sf_ai_bevy; Guess)
            ring_points: 6,
            // tunables: const (bound from ai.perception.search_look_half_deg in sf_ai_bevy; Guess)
            look_half_deg: 60.0,
            // tunables: const (bound from ai.perception.search_uncertain_mul in sf_ai_bevy; Guess)
            uncertain_mul: 2.0,
            // tunables: const (bound from ai.perception.search_inner_frac in sf_ai_bevy; Guess)
            inner_frac: 0.5,
        }
    }
}

/// A rejected [`SearchState::start`] input: every parameter must be finite and ≥ 0 (confidence:
/// finite, then clamped to 0..1).
#[derive(Clone, Copy, Debug, PartialEq, Error)]
#[error("search: {field} = {value} (must be finite and >= 0)")]
pub struct SearchParamError {
    pub field: &'static str,
    pub value: f32,
}

/// What the searcher does next.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SearchGoal {
    /// Walk to this point.
    GoTo(WorldPos),
    /// Turn from `yaw_from` to `yaw_to` (radians about +Y, 0 facing +Z, each in `[-π, π)`).
    LookAround { yaw_from: f32, yaw_to: f32 },
    /// The ttl ran out: give up.
    Done,
}

impl SearchGoal {
    /// The existing executors' operators for this goal, run in order: `GoTo` is one `MoveTo`,
    /// `LookAround` a `Face` to each end, `Done` none.
    pub fn operators(self, worldspace: &WorldspaceId, gait: ExecGait, speed: f32, tolerance_rad: f32) -> [Option<OperatorKind>; 2] {
        let face = |yaw| OperatorKind::Face { target: FaceTarget::Yaw(yaw), tolerance_rad };
        match self {
            Self::GoTo(pos) => [Some(OperatorKind::MoveTo { goal: Location::new(worldspace.clone(), pos), gait, speed }), None],
            Self::LookAround { yaw_from, yaw_to } => [Some(face(yaw_from)), Some(face(yaw_to))],
            Self::Done => [None, None],
        }
    }
}

/// The seed of a search started by `actor` at `tick`.
pub fn search_seed(actor: ActorKey, tick: u64) -> u64 {
    mix64(actor_hash(actor) ^ mix64(tick))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Phase {
    /// Nothing issued yet.
    Start,
    /// A `GoTo` was issued: look around next.
    Moved,
    /// A look-around was issued: the next point next.
    Looked,
    Done,
}

/// One actor's running search.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchState {
    lkp: WorldPos,
    /// 0..1.
    confidence: f32,
    params: SearchParams,
    pattern: SearchPatternConfig,
    seed: u64,
    start_tick: u64,
    phase: Phase,
    /// The last point walked to (the LKP before any).
    at: WorldPos,
    /// Rings drawn so far (0: none).
    ring: u32,
    ring_radius_m: f32,
    /// The current ring's next slot.
    slot: u8,
    /// Look-arounds that had no outward direction (seeds their yaw).
    free_looks: u32,
}

impl SearchState {
    /// Starts a search at `lkp` on `tick`.
    ///
    /// # Errors
    /// A NaN, infinite or negative `params` field or pattern float, or a non-finite `confidence`.
    pub fn start(
        lkp: WorldPos,
        confidence: f32,
        params: SearchParams,
        pattern: &SearchPatternConfig,
        seed: u64,
        tick: u64,
    ) -> Result<Self, SearchParamError> {
        let check = |field: &'static str, value: f32| {
            if value.is_finite() && value >= 0.0 { Ok(()) } else { Err(SearchParamError { field, value }) }
        };
        check("ttl_s", params.ttl_s)?;
        check("error_m", params.error_m)?;
        check("widen_mps", params.widen_mps)?;
        check("look_half_deg", pattern.look_half_deg)?;
        check("uncertain_mul", pattern.uncertain_mul)?;
        check("inner_frac", pattern.inner_frac)?;
        if !confidence.is_finite() {
            return Err(SearchParamError { field: "confidence", value: confidence });
        }
        let pattern = SearchPatternConfig {
            ring_points: pattern.ring_points.max(1),
            look_half_deg: pattern.look_half_deg.min(180.0),
            uncertain_mul: pattern.uncertain_mul.max(1.0),
            inner_frac: pattern.inner_frac.min(1.0),
        };
        Ok(Self {
            lkp,
            confidence: confidence.clamp(0.0, 1.0),
            params,
            pattern,
            seed,
            start_tick: tick,
            phase: Phase::Start,
            at: lkp,
            ring: 0,
            ring_radius_m: 0.0,
            slot: 0,
            free_looks: 0,
        })
    }

    pub fn lkp(&self) -> WorldPos {
        self.lkp
    }

    pub fn is_done(&self) -> bool {
        self.phase == Phase::Done
    }

    /// The tick count after `start` at which the search gives up: `ttl_s` rounded to the nearest
    /// tick (0 for a zero ttl).
    pub fn ttl_ticks(&self, tick_s: f32) -> u64 {
        if self.params.ttl_s <= 0.0 { 0 } else { u64::from(seconds_to_ticks(self.params.ttl_s, tick_s)) }
    }

    /// The searched radius around the LKP `elapsed_s` seconds in (the module's radius rule).
    pub fn radius_at(&self, elapsed_s: f32) -> f32 {
        let conf_mul = 1.0 + (1.0 - self.confidence) * (self.pattern.uncertain_mul - 1.0);
        (self.params.error_m + self.params.widen_mps * elapsed_s.max(0.0)) * conf_mul
    }

    /// The next goal, called when the previous one has finished (succeeded or failed). `reachable`
    /// filters candidate points (nav in `AI8f2`); at most `2 × ring_points + 1` calls per step.
    #[allow(clippy::cast_precision_loss)] // tick deltas are small; f32 seconds suffice
    pub fn next_goal(&mut self, now: PerceptClock, reachable: &mut dyn FnMut(WorldPos) -> bool) -> SearchGoal {
        if self.phase == Phase::Done {
            return SearchGoal::Done;
        }
        let elapsed = now.tick.saturating_sub(self.start_tick);
        if elapsed >= self.ttl_ticks(now.tick_s) {
            self.phase = Phase::Done;
            return SearchGoal::Done;
        }
        let elapsed_s = elapsed as f32 * now.tick_s;
        match self.phase {
            Phase::Start if reachable(self.lkp) => {
                self.phase = Phase::Moved;
                SearchGoal::GoTo(self.lkp)
            }
            Phase::Moved => {
                self.phase = Phase::Looked;
                let outward = (self.at != self.lkp).then(|| yaw_toward(self.lkp, self.at)).flatten();
                let center = outward.unwrap_or_else(|| self.free_look_yaw());
                self.look(center)
            }
            Phase::Start | Phase::Looked => {
                if let Some(p) = self.next_point(elapsed_s, reachable) {
                    self.at = p;
                    self.phase = Phase::Moved;
                    SearchGoal::GoTo(p)
                } else {
                    self.phase = Phase::Looked;
                    let center = self.free_look_yaw();
                    self.look(center)
                }
            }
            Phase::Done => SearchGoal::Done,
        }
    }

    fn look(&self, center: f32) -> SearchGoal {
        let half = self.pattern.look_half_deg.to_radians();
        SearchGoal::LookAround { yaw_from: wrap_angle(center - half), yaw_to: wrap_angle(center + half) }
    }

    fn free_look_yaw(&mut self) -> f32 {
        // tunables: const (format: hash salt separating look draws from ring draws)
        const LOOK_SALT: u64 = 0x4c4f_4f4b;
        self.free_looks += 1;
        let h = self.hash(LOOK_SALT, u64::from(self.free_looks));
        wrap_angle(unit(h) * TAU)
    }

    /// The rest of the current ring, then (if needed) one fresh ring at `r(elapsed_s)`.
    fn next_point(&mut self, elapsed_s: f32, reachable: &mut dyn FnMut(WorldPos) -> bool) -> Option<WorldPos> {
        let n = self.pattern.ring_points;
        let mut fresh = false;
        loop {
            if self.ring == 0 || self.slot >= n {
                if fresh {
                    return None;
                }
                fresh = true;
                self.ring += 1;
                self.ring_radius_m = self.radius_at(elapsed_s);
                self.slot = 0;
            }
            let p = self.ring_point(self.ring, self.slot);
            self.slot += 1;
            if reachable(p) {
                return Some(p);
            }
        }
    }

    /// Slot `slot` of ring `ring`: yaw `rotation + (slot + jitter) × τ/n`, `jitter` in ±¼ sector,
    /// distance in `[inner_frac, 1] × ring radius`.
    fn ring_point(&self, ring: u32, slot: u8) -> WorldPos {
        // tunables: const (format: hash salts separating the ring rotation from slot draws)
        const ROT_SALT: u64 = 0x524f_5441;
        // tunables: const (format: hash salts separating the ring rotation from slot draws)
        const SLOT_SALT: u64 = 0x534c_4f54;
        let n = f32::from(self.pattern.ring_points);
        let rotation = unit(self.hash(ROT_SALT, u64::from(ring))) * TAU;
        let key = (u64::from(ring) << 8) | u64::from(slot);
        let h = self.hash(SLOT_SALT, key);
        let jitter = (unit(h) - 0.5) * 0.5;
        let yaw = rotation + (f32::from(slot) + jitter) * TAU / n;
        let inner = self.pattern.inner_frac;
        let dist = self.ring_radius_m * (inner + (1.0 - inner) * unit(mix64(h)));
        self.lkp.offset(Vec3::new(dist * yaw.sin(), 0.0, dist * yaw.cos()))
    }

    fn hash(&self, salt: u64, key: u64) -> u64 {
        mix64(self.seed ^ mix64(salt ^ mix64(key)))
    }
}

/// A hash's top 24 bits as a float in `[0, 1)`.
#[allow(clippy::cast_precision_loss)] // 24 bits are exact in f32
fn unit(h: u64) -> f32 {
    // tunables: const (maths: f32 mantissa width)
    const BITS: u32 = 24;
    (h >> (64 - BITS)) as f32 / (1u64 << BITS) as f32
}
