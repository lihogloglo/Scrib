//! The reaction controller (research 12 "R2"; physics.md "Ragdolls"): a
//! Bevy-free state machine `Tracking → Stagger → Falling → Down → GettingUp`
//! (plus terminal `Dead`) that turns a [`ReactionRequest`] into per-region
//! strength and physics-weight timelines read from a RON [`ReactionSet`], and
//! reports [`ReactionEvent`]s (`Fell`, `Settled`, `GettingUp`, `GotUp`,
//! `Died`).
//!
//! The shape is the industry one (UE physical-animation profiles and hit
//! reactions, `PuppetMaster`'s Puppet/Unpinned/GetUp states, Euphoria's
//! game-driven messages): the clip stays the target, gameplay decides the
//! outcome class (`sf_action`'s Forced moves), and the controller only moves
//! strength and weight along authored curves. It never picks clips.
//!
//! Outputs are pure functions of the elapsed time in the current state, so
//! they do not depend on the step size; only the instants of the sensed
//! transitions (settling) are quantized to the step. `seed` adds a small,
//! deterministic variation (time scale and dip depth) like
//! `sf_anim_graph::variety`.

use std::collections::BTreeMap;
use std::sync::Arc;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

#[cfg(test)]
mod tests;

/// Body regions the curves address (the adapter maps its rig's regions to
/// these by [`Region::of_name`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Region {
    Legs,
    Spine,
    Arms,
    Head,
}

impl Region {
    pub const ALL: [Region; 4] = [Region::Legs, Region::Spine, Region::Arms, Region::Head];

    pub const fn index(self) -> usize {
        self as usize
    }

    /// The controller region of a ragdoll region name (`"core"`, `"head"`,
    /// limb and chain names); anything unrecognized follows the spine.
    pub fn of_name(name: &str) -> Self {
        let has = |parts: &[&str]| parts.iter().any(|p| name.contains(p));
        if has(&["head", "neck"]) {
            Self::Head
        } else if has(&["arm", "hand", "wing", "shoulder"]) {
            Self::Arms
        } else if has(&["leg", "foot", "paw", "hoof", "thigh"]) {
            Self::Legs
        } else {
            Self::Spine
        }
    }
}

/// How the hit is classed (decided by gameplay, never by the controller).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReactionClass {
    /// Dip and recover; escalates to a fall above [`ReactionSet::fall_impulse`]
    /// or on lost balance.
    Stagger,
    /// Powered fall, then down, then get up.
    Knockdown,
    /// Crumple; terminal.
    Death,
}

/// A hit's reaction (from `sf_action`'s Forced moves; replicated to clients
/// with its tick, which runs the same controller for presentation).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReactionRequest {
    pub class: ReactionClass,
    /// The hit body (index into the ragdoll's bodies).
    pub body: u16,
    pub point: WorldPos,
    /// N·s, world.
    pub impulse: Vec3,
    /// The simulation tick of the hit.
    pub tick: u64,
    /// Variation seed (placement id folded with the tick, say).
    pub seed: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionState {
    Tracking,
    Stagger,
    Falling,
    Down,
    GettingUp,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactionEvent {
    /// Left tracking for a fall (knockdown, or a stagger upgraded by impulse or balance).
    Fell,
    /// Came to rest.
    Settled,
    /// The get-up began (the host plays its get-up clip).
    GettingUp,
    /// Back to tracking.
    GotUp,
    Died,
}

/// A piecewise-linear curve of `(seconds, value)` keys, ascending in time;
/// held at both ends.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Curve(pub Vec<(f32, f32)>);

impl Curve {
    pub fn constant(value: f32) -> Self {
        Self(vec![(0.0, value)])
    }

    pub fn eval(&self, t: f32) -> f32 {
        let keys = &self.0;
        let Some(&(t0, v0)) = keys.first() else { return 0.0 };
        if t <= t0 {
            return v0;
        }
        match keys.iter().position(|&(kt, _)| kt > t) {
            None => keys[keys.len() - 1].1,
            Some(i) => {
                let (ta, va) = keys[i - 1];
                let (tb, vb) = keys[i];
                va + (vb - va) * ((t - ta) / (tb - ta))
            }
        }
    }

    /// The time of the last key.
    pub fn end(&self) -> f32 {
        self.0.last().map_or(0.0, |k| k.0)
    }

    fn first(&self) -> f32 {
        self.0.first().map_or(0.0, |k| k.1)
    }

    fn last(&self) -> f32 {
        self.0.last().map_or(0.0, |k| k.1)
    }

    /// The time of the lowest key.
    fn argmin(&self) -> f32 {
        self.0.iter().copied().min_by(|a, b| a.1.total_cmp(&b.1)).map_or(0.0, |k| k.0)
    }
}

/// One curve for every region, with per-region overrides.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionCurves {
    pub all: Curve,
    #[serde(default)]
    pub overrides: BTreeMap<Region, Curve>,
}

impl RegionCurves {
    pub fn curve(&self, region: Region) -> &Curve {
        self.overrides.get(&region).unwrap_or(&self.all)
    }

    fn curves(&self) -> impl Iterator<Item = (Region, &Curve)> {
        Region::ALL.into_iter().map(|r| (r, self.curve(r)))
    }

    fn end(&self) -> f32 {
        self.curves().map(|(_, c)| c.end()).fold(0.0, f32::max)
    }
}

/// Strength and physics-weight curves of one phase (seconds from its start).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    /// 0 = limp, 1 = full tracking strength.
    pub strength: RegionCurves,
    /// 0 = the clip, 1 = the bodies.
    pub physics_weight: RegionCurves,
    /// ±share of time scale and dip depth drawn from the request's seed.
    #[serde(default)]
    pub variation: f32,
}

impl Timeline {
    fn end(&self) -> f32 {
        self.strength.end().max(self.physics_weight.end())
    }
}

/// The reaction data of a game (RON): curves per phase and the thresholds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactionSet {
    pub stagger: Timeline,
    /// The powered fall (also after a stagger upgrade); its last values hold while down.
    pub knockdown: Timeline,
    pub death: Timeline,
    pub get_up: Timeline,
    /// A stagger of at least this impulse (N·s) becomes a fall (0: never).
    pub fall_impulse: f32,
    /// A staggering character whose upright (body up · world up) drops below
    /// this falls (host only).
    pub fall_upright: f32,
    /// Settled: the pelvis slower than this (m/s) for `rest_time` s, grounded.
    pub rest_speed: f32,
    pub rest_time: f32,
    /// Falling longer than this (s) settles anyway, at any pelvis speed: a
    /// body that never comes to rest (lying on a rolling ball, jitter, a slow
    /// slide) still gets up instead of staying down for good (AN-4). 0: never.
    #[serde(default = "default_settle_timeout")]
    pub settle_timeout: f32,
}

/// The default [`ReactionSet`] (`data/reactions.ron`).
pub const DEFAULT_RON: &str = include_str!("../data/reactions.ron");

fn default_settle_timeout() -> f32 {
    6.0
}

impl Default for ReactionSet {
    fn default() -> Self {
        Self::from_ron(DEFAULT_RON).expect("the bundled reaction set parses")
    }
}

impl ReactionSet {
    pub fn from_ron(text: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(text)
    }

    pub fn to_ron(&self) -> String {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).expect("in-memory serialization")
    }

    /// Checks the curves: ascending keys, a stagger and a get-up that end at
    /// full strength and weight 0 (so the return to tracking is continuous),
    /// and a get-up that starts where the knockdown ends.
    pub fn validate(&self) -> Result<(), String> {
        const EPS: f32 = 1e-4;
        let phases = [("stagger", &self.stagger), ("knockdown", &self.knockdown), ("death", &self.death), ("get_up", &self.get_up)];
        for (name, tl) in phases {
            for (kind, rc) in [("strength", &tl.strength), ("physics_weight", &tl.physics_weight)] {
                for (region, c) in rc.curves() {
                    if c.0.is_empty() || c.0.windows(2).any(|w| w[1].0 <= w[0].0) {
                        return Err(format!("{name}.{kind}.{region:?}: keys must be non-empty and strictly ascending"));
                    }
                }
            }
        }
        for (name, tl) in [("stagger", &self.stagger), ("get_up", &self.get_up)] {
            for r in Region::ALL {
                if (tl.strength.curve(r).last() - 1.0).abs() > EPS || tl.physics_weight.curve(r).last().abs() > EPS {
                    return Err(format!("{name}.{r:?} must end at strength 1 and weight 0"));
                }
            }
        }
        for r in Region::ALL {
            let down = (self.knockdown.strength.curve(r).last(), self.knockdown.physics_weight.curve(r).last());
            let up = (self.get_up.strength.curve(r).first(), self.get_up.physics_weight.curve(r).first());
            if (down.0 - up.0).abs() > EPS || (down.1 - up.1).abs() > EPS {
                return Err(format!("get_up.{r:?} must start where knockdown ends"));
            }
        }
        Ok(())
    }
}

/// What the physics reports each step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sensed {
    /// The pelvis's speed, m/s.
    pub pelvis_speed: f32,
    /// The pelvis's up axis · world up (1 upright, 0 sideways, −1 inverted).
    pub upright: f32,
    pub grounded: bool,
    /// Gameplay allows the get-up (false: knocked out, held down).
    pub can_get_up: bool,
    /// This peer decides falls from lost balance (the host; clients follow
    /// its requests: physics.md "co-op").
    pub host: bool,
}

impl Default for Sensed {
    fn default() -> Self {
        Self { pelvis_speed: 0.0, upright: 1.0, grounded: true, can_get_up: true, host: true }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReactionOutput {
    pub state: ReactionState,
    /// Per [`Region::index`].
    pub region_strength: [f32; 4],
    pub region_physics_weight: [f32; 4],
    /// Raised by this step (and by requests since the last one).
    pub events: Vec<ReactionEvent>,
}

impl ReactionOutput {
    pub fn strength(&self, region: Region) -> f32 {
        self.region_strength[region.index()]
    }

    pub fn physics_weight(&self, region: Region) -> f32 {
        self.region_physics_weight[region.index()]
    }

    pub fn max_strength(&self) -> f32 {
        self.region_strength.iter().copied().fold(0.0, f32::max)
    }
}

/// One character's reaction state.
#[derive(Clone, Debug)]
pub struct ReactionController {
    set: Arc<ReactionSet>,
    state: ReactionState,
    /// Seconds in the current state.
    elapsed: f32,
    time_scale: f32,
    depth: f32,
    /// Seconds the pelvis has been slower than the rest speed (while falling).
    slow: f32,
    last: Option<ReactionRequest>,
    pending: Vec<ReactionEvent>,
}

/// The splitmix64 finalizer (as `sf_anim_graph::variety::mix64`).
fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Two uniform values in [−1, 1) from a seed.
#[allow(clippy::cast_precision_loss)]
fn signed_pair(seed: u64) -> (f32, f32) {
    let m = mix64(seed);
    let unit = |bits: u64| (bits & 0xFF_FFFF) as f32 / 16_777_216.0 * 2.0 - 1.0;
    (unit(m), unit(m >> 32))
}

impl ReactionController {
    pub fn new(set: Arc<ReactionSet>) -> Self {
        Self { set, state: ReactionState::Tracking, elapsed: 0.0, time_scale: 1.0, depth: 1.0, slow: 0.0, last: None, pending: Vec::new() }
    }

    pub fn state(&self) -> ReactionState {
        self.state
    }

    /// The request that started the current reaction.
    pub fn last_request(&self) -> Option<&ReactionRequest> {
        self.last.as_ref()
    }

    pub fn set(&self) -> &ReactionSet {
        &self.set
    }

    /// Seconds in the current state.
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Starts a reaction; `false` if the state ignores it (death is terminal;
    /// a fall or get-up ignores a stagger; a fall in progress ignores a
    /// knockdown, whose impulse the caller still applies to the bodies).
    pub fn request(&mut self, request: ReactionRequest) -> bool {
        use ReactionClass as C;
        use ReactionState as S;
        let state = self.state;
        match (request.class, state) {
            (_, S::Dead) | (C::Stagger, S::Falling | S::Down | S::GettingUp) | (C::Knockdown, S::Falling | S::Down) => return false,
            (C::Death, _) => {
                self.begin(S::Dead, &request);
                self.pending.push(ReactionEvent::Died);
            }
            (C::Knockdown, _) => self.fall(&request),
            (C::Stagger, S::Tracking | S::Stagger) => {
                let limit = self.set.fall_impulse;
                if limit > 0.0 && request.impulse.length() >= limit {
                    self.fall(&request);
                } else if state == S::Stagger {
                    // A re-hit deepens the dip: rewind to its bottom, keeping the scale.
                    let bottom = self.set.stagger.strength.all.argmin() * self.time_scale;
                    self.elapsed = self.elapsed.min(bottom);
                    self.last = Some(request);
                } else {
                    self.begin(S::Stagger, &request);
                }
            }
        }
        true
    }

    fn begin(&mut self, state: ReactionState, request: &ReactionRequest) {
        let variation = match state {
            ReactionState::Stagger => self.set.stagger.variation,
            ReactionState::Falling => self.set.knockdown.variation,
            ReactionState::Dead => self.set.death.variation,
            _ => 0.0,
        };
        let (a, b) = signed_pair(request.seed);
        self.time_scale = (1.0 + variation * a).max(0.1);
        self.depth = (1.0 + variation * b).max(0.0);
        self.state = state;
        self.elapsed = 0.0;
        self.slow = 0.0;
        self.last = Some(*request);
    }

    fn fall(&mut self, request: &ReactionRequest) {
        self.begin(ReactionState::Falling, request);
        self.pending.push(ReactionEvent::Fell);
    }

    fn timeline(&self) -> Option<&Timeline> {
        match self.state {
            ReactionState::Tracking => None,
            ReactionState::Stagger => Some(&self.set.stagger),
            ReactionState::Falling | ReactionState::Down => Some(&self.set.knockdown),
            ReactionState::GettingUp => Some(&self.set.get_up),
            ReactionState::Dead => Some(&self.set.death),
        }
    }

    /// Advances `dt` seconds and returns the strengths and weights.
    pub fn step(&mut self, dt: f32, sensed: &Sensed) -> ReactionOutput {
        use ReactionState as S;
        self.elapsed += dt;
        if self.state == S::Falling {
            self.slow = if sensed.pelvis_speed < self.set.rest_speed { self.slow + dt } else { 0.0 };
        }
        // A step can pass several states (settle, then straight into the get-up).
        for _ in 0..4 {
            let scaled_end = self.timeline().map_or(0.0, Timeline::end) * self.time_scale;
            match self.state {
                S::Tracking | S::Dead => {}
                S::Stagger => {
                    if sensed.host && sensed.upright < self.set.fall_upright {
                        let request = self.last.expect("a stagger has a request");
                        self.fall(&request);
                        continue;
                    }
                    if self.elapsed >= scaled_end {
                        self.state = S::Tracking;
                        self.elapsed = 0.0;
                    }
                }
                S::Falling => {
                    let timed_out = self.set.settle_timeout > 0.0 && self.elapsed >= self.set.settle_timeout;
                    if self.elapsed >= scaled_end && (self.slow >= self.set.rest_time || timed_out) && sensed.grounded {
                        self.state = S::Down;
                        self.elapsed = 0.0;
                        self.pending.push(ReactionEvent::Settled);
                        continue;
                    }
                }
                S::Down => {
                    if sensed.can_get_up {
                        self.state = S::GettingUp;
                        self.elapsed = 0.0;
                        self.time_scale = 1.0;
                        self.depth = 1.0;
                        self.pending.push(ReactionEvent::GettingUp);
                        continue;
                    }
                }
                S::GettingUp => {
                    if self.elapsed >= scaled_end {
                        self.state = S::Tracking;
                        self.elapsed = 0.0;
                        self.pending.push(ReactionEvent::GotUp);
                    }
                }
            }
            break;
        }
        self.output()
    }

    /// The current strengths and weights, draining the raised events.
    pub fn output(&mut self) -> ReactionOutput {
        let mut out = ReactionOutput { state: self.state, region_strength: [1.0; 4], region_physics_weight: [0.0; 4], events: std::mem::take(&mut self.pending) };
        let Some(tl) = self.timeline() else { return out };
        // Down holds the fall's last keys.
        let t = if self.state == ReactionState::Down { f32::MAX } else { self.elapsed / self.time_scale };
        for r in Region::ALL {
            let s = tl.strength.curve(r).eval(t);
            out.region_strength[r.index()] = (1.0 - (1.0 - s) * self.depth).clamp(0.0, 1.0);
            out.region_physics_weight[r.index()] = tl.physics_weight.curve(r).eval(t).clamp(0.0, 1.0);
        }
        out
    }
}
