//! Lock-on math (camera.md "Optional lock-on"). The locked target is gameplay state and lives in
//! `sf_action`; these are the pure functions it and the camera shell call over a candidate list
//! (the shell gathers candidates and their line of sight), plus the small timer struct the owner
//! keeps beside the target.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

use crate::{Look, wrap_angle};

/// A lockable character as seen this frame or tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Candidate<Id> {
    pub id: Id,
    /// Aim point (chest), absolute.
    pub pos: WorldPos,
    /// Unobstructed from the eye or pivot (the shell's cast).
    pub los: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LockParams {
    /// Acquire within this range and cone half-angle around the view yaw (radians).
    pub acquire_range: f32,
    pub acquire_cone: f32,
    /// Break beyond this range (longer than `acquire_range`: hysteresis).
    pub break_range: f32,
    /// Break after this long without line of sight, seconds.
    pub los_timeout: f32,
    /// After a traversal broke the lock, the same target is offered back for this long after
    /// touchdown, seconds.
    pub reacquire_window: f32,
    /// Score weight of the angle against the distance (0: nearest wins, 1: most centred wins).
    pub angle_weight: f32,
    /// Locked framing: pitch limits, a downward bias, and the ease half-life.
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub pitch_bias: f32,
    pub framing_halflife: f32,
}

impl Default for LockParams {
    fn default() -> Self {
        Self {
            acquire_range: 15.0,
            acquire_cone: 35f32.to_radians(),
            break_range: 20.0,
            los_timeout: 1.5,
            reacquire_window: 1.0,
            angle_weight: 0.6,
            pitch_min: -40f32.to_radians(),
            pitch_max: 25f32.to_radians(),
            pitch_bias: -8f32.to_radians(),
            framing_halflife: 0.12,
        }
    }
}

fn yaw_and_distance(from: WorldPos, to: WorldPos) -> (f32, f32) {
    let d = from.delta_f32(to);
    (d.x.atan2(d.z), Vec3::new(d.x, 0.0, d.z).length())
}

/// Acquisition score of a candidate, lower is better; `None` outside the acquire cone or range or
/// without line of sight.
pub fn score<Id>(origin: WorldPos, view_yaw: f32, c: &Candidate<Id>, p: &LockParams) -> Option<f32> {
    let (yaw, dist) = yaw_and_distance(origin, c.pos);
    let angle = wrap_angle(yaw - view_yaw).abs();
    (c.los && dist <= p.acquire_range && angle <= p.acquire_cone)
        .then(|| p.angle_weight * angle / p.acquire_cone.max(1e-4) + (1.0 - p.angle_weight) * dist / p.acquire_range.max(1e-4))
}

/// The best candidate to lock on a lock press, if any (none: the shell recentres instead).
pub fn acquire<Id: Copy>(origin: WorldPos, view_yaw: f32, candidates: &[Candidate<Id>], p: &LockParams) -> Option<Id> {
    candidates
        .iter()
        .filter_map(|c| score(origin, view_yaw, c, p).map(|s| (s, c.id)))
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id)| id)
}

/// Switch to the next target on one side of the current one (`right`: clockwise on screen, i.e.
/// toward the view's right), the smallest yaw step within `break_range` with line of sight.
pub fn switch_target<Id: Copy + PartialEq>(
    origin: WorldPos,
    current: Id,
    candidates: &[Candidate<Id>],
    right: bool,
    p: &LockParams,
) -> Option<Id> {
    let cur = candidates.iter().find(|c| c.id == current)?;
    let (cur_yaw, _) = yaw_and_distance(origin, cur.pos);
    candidates
        .iter()
        .filter(|c| c.id != current && c.los)
        .filter_map(|c| {
            let (yaw, dist) = yaw_and_distance(origin, c.pos);
            // Yaw grows counter-clockwise seen from above, so screen right is a negative step.
            let step = wrap_angle(yaw - cur_yaw) * if right { -1.0 } else { 1.0 };
            (dist <= p.break_range && step > 0.0).then_some((step, c.id))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id)| id)
}

/// Why a lock ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakReason {
    Range,
    LineOfSight,
    /// The target is gone (dead, despawned, not in the candidate list).
    Lost,
    /// A traversal or interaction started (action.md priority list).
    Traversal,
}

/// Timers kept with the locked target by its owner (`sf_action` state), advanced with that
/// owner's `dt`. Serde so it can ride in saved or replicated state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LockTimers {
    /// Seconds the locked target has been out of line of sight.
    pub no_los: f32,
    /// A target a traversal broke, offered back after touchdown.
    pub offered: Option<u32>,
    /// Seconds left of the re-acquire offer once landed; `None` while still traversing.
    pub offer_left: Option<f32>,
}

impl LockTimers {
    /// Keep or break the current lock this frame or tick. `target` is the locked one's current
    /// candidate entry, `None` if it is gone.
    pub fn check<Id>(&mut self, origin: WorldPos, target: Option<&Candidate<Id>>, p: &LockParams, dt: f32) -> Result<(), BreakReason> {
        let Some(t) = target else {
            self.no_los = 0.0;
            return Err(BreakReason::Lost);
        };
        let (_, dist) = yaw_and_distance(origin, t.pos);
        if dist > p.break_range {
            self.no_los = 0.0;
            return Err(BreakReason::Range);
        }
        self.no_los = if t.los { 0.0 } else { self.no_los + dt };
        if self.no_los > p.los_timeout {
            self.no_los = 0.0;
            return Err(BreakReason::LineOfSight);
        }
        Ok(())
    }

    /// A traversal or interaction started while locked on `target` (its stable id key): break,
    /// and remember it for [`Self::reacquire`].
    pub fn break_for_traversal(&mut self, target: u32) -> BreakReason {
        self.no_los = 0.0;
        self.offered = Some(target);
        self.offer_left = None;
        BreakReason::Traversal
    }

    /// Touchdown after the traversal: the re-acquire window starts.
    pub fn touchdown(&mut self, p: &LockParams) {
        if self.offered.is_some() && self.offer_left.is_none() {
            self.offer_left = Some(p.reacquire_window);
        }
    }

    /// While the offer runs, the remembered target comes back if it is still a candidate within
    /// `break_range` with line of sight (no cone: the player didn't look away on purpose).
    /// `key` maps a candidate id to the stable key passed to [`Self::break_for_traversal`].
    pub fn reacquire<Id: Copy>(
        &mut self,
        origin: WorldPos,
        candidates: &[Candidate<Id>],
        key: impl Fn(Id) -> u32,
        p: &LockParams,
        dt: f32,
    ) -> Option<Id> {
        let (Some(target), Some(left)) = (self.offered, self.offer_left) else { return None };
        let found = candidates
            .iter()
            .find(|c| key(c.id) == target && c.los && yaw_and_distance(origin, c.pos).1 <= p.break_range)
            .map(|c| c.id);
        let left = left - dt;
        if found.is_some() || left <= 0.0 {
            self.offered = None;
            self.offer_left = None;
        } else {
            self.offer_left = Some(left);
        }
        found
    }
}

/// Locked framing goal: the camera behind the character on the line to the target, pitch from
/// the height gap (plus a downward bias) within limits. Ease toward it with [`Look::eased`] and
/// `p.framing_halflife`.
pub fn locked_look(pivot: WorldPos, target: WorldPos, p: &LockParams) -> Look {
    let d = pivot.delta_f32(target);
    let flat = Vec3::new(d.x, 0.0, d.z).length().max(0.5);
    Look { yaw: d.x.atan2(d.z), pitch: (d.y.atan2(flat) + p.pitch_bias).clamp(p.pitch_min, p.pitch_max) }
}
