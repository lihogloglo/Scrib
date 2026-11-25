//! `AI8h1` band reactions core (`docs/specs/ai.md` "Perception": **Curious** looks toward,
//! **Suspicious** turns and steps toward the last known position). Bevy-free; `sf_ai_bevy`
//! (`AI8h2`) runs the goals through the existing `MoveTo` / `Face` executors, as it does
//! [`SearchGoal`](crate::SearchGoal)s.
//!
//! **Industry standard** (Skyrim's detection stages, Assassin's Creed's and Hitman's suspicion
//! meters, MGS V's "?" stage): the first stage stops the NPC and turns it toward the stimulus (head
//! first, the body once the stimulus is outside a cone); the second walks it a short way toward the
//! last known position (LKP), where it looks; it then returns to its routine as awareness decays.
//! Head look-at is an animation feature (anim lane, later) layered on top of this core: here the
//! output is the facing goal, and the body turn is the existing `Face` operator.
//!
//! **Rule**, from the actor's top [`Awareness`] entry, its position and the clock:
//! - Unaware, Searching ([`SearchState`](crate::SearchState) owns it) and Alerted (drives own it,
//!   later): [`ReactGoal::None`], and the state resets.
//! - Curious: [`ReactGoal::Face`] toward the LKP.
//! - Suspicious: [`ReactGoal::Investigate`]: walk to the point [`ReactConfig::step_m`] along the
//!   planar (XZ) line from the actor toward the LKP (height interpolated on the same line), never
//!   past it (the LKP itself when closer than `step_m`), then face the LKP.
//! - **Double take:** a reaction starts [`ReactConfig::face_delay_s`] after the actor first entered
//!   Curious or Suspicious from idle; until then the goal is `None`. Escalating Curious →
//!   Suspicious or switching target keeps reacting without a new delay.
//! - **Hold:** a goal is anchored on the LKP it was computed from. A band change or another target
//!   re-targets at once; an LKP moved more than [`ReactConfig::retarget_m`] from the anchor
//!   re-targets at once; a smaller move re-targets only once [`ReactConfig::hold_s`] has passed
//!   since the goal was issued. An unmoved LKP keeps the goal, so a Suspicious actor walks one step,
//!   not a creep all the way to the LKP.
//! - **Spent stimulus:** once the actor escalates past Suspicious (Searching or Alerted) on a
//!   placement, decaying back down through Suspicious and Curious with that same placement (same
//!   target, same `LastSeen::tick`) gives `None`: the search already looked, so the actor returns to
//!   its routine. A newer placement re-arms the reaction.
//!
//! **Determinism, net:** no RNG; pure data in, goal out, on the logical tick ([`PerceptClock`]).
//! [`ReactState`] is serde so a host snapshot can carry it; saves don't need it (the meter is saved,
//! and the reaction is recomputed from it). The target's `LastSeen::velocity` is not extrapolated
//! here (a fleeing target is the search's outward look-around).

use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::exec::{ExecGait, FaceTarget, OperatorKind};
use crate::perception::{Awareness, AwarenessBand, LastSeen, PerceptClock, PerceptTarget};
use crate::sight::seconds_to_ticks;

/// Below this planar distance, squared, the actor stands on the LKP (m²; the exec crate's planar
/// epsilon).
// tunables: const (maths: planar coincidence epsilon, as exec's PLANAR_EPS_SQ)
const PLANAR_EPS_SQ: f32 = 1e-8;

/// Band reaction tunables (`ai.perception.react_*`, bound in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactConfig {
    /// How far a Suspicious actor steps toward the LKP, meters (range 0-10).
    pub step_m: f32,
    /// An LKP moved farther than this from the goal's anchor re-targets at once, meters (range 0-10).
    pub retarget_m: f32,
    /// A smaller LKP move re-targets only after this long since the goal was issued, seconds (range 0-10).
    pub hold_s: f32,
    /// The "double take": delay from first noticing to the first reaction, seconds (range 0-2).
    pub face_delay_s: f32,
}

impl Default for ReactConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.react_step_m in sf_ai_bevy; Guess: a few strides, AC/Hitman suspicious walk)
            step_m: 3.0,
            // tunables: const (bound from ai.perception.react_retarget_m in sf_ai_bevy; Guess: about one stride of target motion)
            retarget_m: 1.5,
            // tunables: const (bound from ai.perception.react_hold_s in sf_ai_bevy; Guess: keeps the goal from flickering)
            hold_s: 2.0,
            // tunables: const (bound from ai.perception.react_face_delay_s in sf_ai_bevy; Guess: human reaction ~0.2-0.3 s)
            face_delay_s: 0.3,
        }
    }
}

/// A finite, non-negative config value (else 0, as `light`'s bad-input rule).
fn non_neg(v: f32) -> f32 {
    if v.is_finite() { v.max(0.0) } else { 0.0 }
}

/// Whole ticks in `seconds` (0 for a zero or bad duration, else at least one tick).
fn ticks(seconds: f32, tick_s: f32) -> u64 {
    let s = non_neg(seconds);
    if s <= 0.0 { 0 } else { u64::from(seconds_to_ticks(s, tick_s)) }
}

/// What a Curious or Suspicious actor does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ReactGoal {
    /// No reaction: idle, still in the double take, a spent stimulus, or a band owned elsewhere.
    #[default]
    None,
    /// Curious: turn toward this point (the LKP).
    Face { toward: WorldPos },
    /// Suspicious: walk to `to`, then face `then_face` (the LKP).
    Investigate { to: WorldPos, then_face: WorldPos },
}

impl ReactGoal {
    /// The existing executors' operators for this goal, run in order: `Face` is one `Face`,
    /// `Investigate` a `MoveTo` then a `Face`, `None` none.
    pub fn operators(self, worldspace: &WorldspaceId, gait: ExecGait, speed: f32, tolerance_rad: f32) -> [Option<OperatorKind>; 2] {
        let face = |p| OperatorKind::Face { target: FaceTarget::Point(p), tolerance_rad };
        match self {
            Self::None => [None, None],
            Self::Face { toward } => [Some(face(toward)), None],
            Self::Investigate { to, then_face } => {
                [Some(OperatorKind::MoveTo { goal: Location::new(worldspace.clone(), to), gait, speed }), Some(face(then_face))]
            }
        }
    }
}

/// The point `step_m` along the planar line from `from` toward `lkp` (height on the same line),
/// or `lkp` itself when it is no farther than `step_m`.
pub fn investigate_point(from: WorldPos, lkp: WorldPos, step_m: f32) -> WorldPos {
    let d = from.delta_f32(lkp);
    let planar_sq = d.x * d.x + d.z * d.z;
    let step = non_neg(step_m);
    if planar_sq <= PLANAR_EPS_SQ || planar_sq <= step * step {
        return lkp;
    }
    from.offset(d * (step / planar_sq.sqrt()))
}

/// What the reaction reads from the actor's top [`Awareness`] entry (pure data, so a client or a
/// test can build it).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactInput {
    pub target: PerceptTarget,
    pub band: AwarenessBand,
    pub last_seen: LastSeen,
}

impl From<&Awareness> for ReactInput {
    fn from(a: &Awareness) -> Self {
        Self { target: a.target, band: a.band, last_seen: a.last_seen }
    }
}

/// The goal being held and what it was computed from.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Held {
    target: PerceptTarget,
    band: AwarenessBand,
    /// The LKP the goal was computed from.
    anchor: WorldPos,
    /// Tick the goal was issued.
    issued: u64,
    goal: ReactGoal,
}

/// One actor's band reaction: the double take, the held goal and the spent placement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactState {
    /// Tick the actor first entered Curious or Suspicious from idle.
    noticed: Option<u64>,
    held: Option<Held>,
    /// `(target, LastSeen::tick)` of the placement last escalated past Suspicious.
    spent: Option<(PerceptTarget, u64)>,
    /// Bumped on every re-target (a new goal for the executors).
    revision: u32,
}

impl ReactState {
    pub fn new() -> Self {
        Self::default()
    }

    /// The goal currently held (`None` when idle or in the double take).
    pub fn goal(&self) -> ReactGoal {
        self.held.map_or(ReactGoal::None, |h| h.goal)
    }

    /// Bumped on every re-target: the executors restart when it changes.
    pub fn revision(&self) -> u32 {
        self.revision
    }

    /// Advances the reaction to `now` from the actor's top awareness entry (`None`: no entry; see
    /// [`AwarenessMeter::strongest`](crate::AwarenessMeter::strongest)) and its own position, and
    /// returns the goal to run (the module's rule).
    pub fn update(&mut self, cfg: &ReactConfig, top: Option<ReactInput>, actor_pos: WorldPos, now: PerceptClock) -> ReactGoal {
        let Some(top) = top else {
            self.idle();
            return ReactGoal::None;
        };
        let placement = (top.target, top.last_seen.tick);
        match top.band {
            AwarenessBand::Unaware => {
                self.idle();
                return ReactGoal::None;
            }
            AwarenessBand::Searching | AwarenessBand::Alerted => {
                self.idle();
                self.spent = Some(placement);
                return ReactGoal::None;
            }
            AwarenessBand::Curious | AwarenessBand::Suspicious => {}
        }
        if self.spent == Some(placement) {
            self.idle();
            return ReactGoal::None;
        }
        let noticed = *self.noticed.get_or_insert(now.tick);
        if now.tick.saturating_sub(noticed) < ticks(cfg.face_delay_s, now.tick_s) {
            return ReactGoal::None;
        }
        let lkp = top.last_seen.pos;
        let retarget = match self.held {
            None => true,
            Some(h) if h.target != top.target || h.band != top.band => true,
            Some(h) => {
                let moved = h.anchor.delta_f32(lkp).length();
                moved > non_neg(cfg.retarget_m)
                    || (h.anchor != lkp && now.tick.saturating_sub(h.issued) >= ticks(cfg.hold_s, now.tick_s))
            }
        };
        if retarget {
            let goal = if top.band == AwarenessBand::Curious {
                ReactGoal::Face { toward: lkp }
            } else {
                ReactGoal::Investigate { to: investigate_point(actor_pos, lkp, cfg.step_m), then_face: lkp }
            };
            self.held = Some(Held { target: top.target, band: top.band, anchor: lkp, issued: now.tick, goal });
            self.revision = self.revision.wrapping_add(1);
        }
        self.goal()
    }

    /// Drops the held goal and the double take (the spent placement stays).
    fn idle(&mut self) {
        self.noticed = None;
        self.held = None;
    }
}
