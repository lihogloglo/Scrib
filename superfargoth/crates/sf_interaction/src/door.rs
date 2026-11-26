//! Swing doors (interaction.md, IN33): a kinematic hinged leaf turned about its [`DoorDef`] hinge,
//! no physics joint. The state is the target `open` plus `t` (0 closed .. 1 open, linear in time);
//! the leaf angle eases in and out of `t` (smoothstep), the standard timeline-with-curve door of UE.
//! The verbs `open` / `close` are slotless and filter on the [`DOOR_OPEN`] state tag. Behaviour
//! reference (no code ported): `OpenMW` `apps/openmw/mwworld/worldimp.cpp` `rotateDoor`, where a toggle
//! mid-swing reverses from the current angle and a leaf that would hit an actor does not advance.
//! The shell sweeps the next pose ([`DoorState::stepped`]) for a blocker before it commits a step;
//! close-behind is a pure schedule here ([`DoorState::released`], [`DoorState::poll`]).

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_world::PlacementId;

use crate::claims::{Tick, UserKey};
use crate::def::{DoorDef, Frame, TagId};
use crate::index::compose;

/// State tag an open door carries (verbs `open` / `close` filter on it).
pub const DOOR_OPEN: TagId = TagId::of("door.open");

/// Float slack of the arrival test, as a fraction of one step: a step that leaves less than this
/// share of itself to go lands on the target exactly (rounding of summed steps, not a tunable).
// tunables: const (f32 rounding slack of summed steps, maths; far below one visible pose change)
const ARRIVE_SLACK: f32 = 1.0e-3;

/// One door's swing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorState {
    /// The target; this (with `sign`) is what a save keeps.
    pub open: bool,
    /// 0 closed .. 1 open, linear in time.
    pub t: f32,
    /// +1 or -1: which way it swings about the hinge axis; chosen at a flip from fully closed.
    pub sign: i8,
    /// The user whose toggle took it from closed to open.
    pub opener: Option<UserKey>,
    /// Close-behind schedule.
    pub close_at: Option<Tick>,
}

impl Default for DoorState {
    fn default() -> Self {
        Self::resting(false, 1)
    }
}

/// One toggle of one door, for the shell (sound, nav links, `DOOR_OPEN`) and the replay log.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorChanged {
    pub placement: PlacementId,
    /// The new target.
    pub open: bool,
    /// Who toggled it; `None` = close-behind or a script.
    pub user: Option<UserKey>,
    pub tick: Tick,
}

/// Which way a door swings for a user (all in the object's frame; `doorway` = the door slot's
/// `local.translation`, the centre of the closed leaf): `+1` unless the door swings both ways and a
/// user position is given, then away from the user. A positive turn about `axis` moves the leaf
/// centre along `n = axis × r` (`r` = hinge to doorway, off the axis); a user on the `n` side gets
/// `-1`. Degenerate geometry gives `+1`.
#[must_use]
pub fn swing_sign(def: &DoorDef, doorway: Vec3, user: Option<Vec3>) -> i8 {
    let Some(user) = user.filter(|_| def.both_ways) else { return 1 };
    let Some(axis) = def.axis.try_normalize() else { return 1 };
    let arm = doorway - def.hinge;
    let r = arm - axis * arm.dot(axis);
    let Some(n) = axis.cross(r).try_normalize() else { return 1 };
    if (user - doorway).dot(n) > 0.0 { -1 } else { 1 }
}

impl DoorState {
    /// At rest (`t` = 0 or 1), no opener, no schedule.
    #[must_use]
    pub fn resting(open: bool, sign: i8) -> Self {
        Self { open, t: if open { 1.0 } else { 0.0 }, sign, opener: None, close_at: None }
    }

    fn target(&self) -> f32 {
        if self.open { 1.0 } else { 0.0 }
    }

    /// Whether `t` has not reached the target yet.
    #[must_use]
    pub fn is_moving(&self) -> bool {
        self.t.to_bits() != self.target().to_bits()
    }

    /// Flips the target (mid-swing too: it then reverses from the current `t`). A flip from fully
    /// closed picks the swing side ([`swing_sign`]) and records the opener; closing clears the
    /// opener; every toggle cancels a close-behind schedule. `user` = (key, position in the
    /// object's frame).
    pub fn toggle(
        &mut self,
        placement: PlacementId,
        def: &DoorDef,
        doorway: Vec3,
        user: Option<(UserKey, Vec3)>,
        tick: Tick,
    ) -> DoorChanged {
        let key = user.map(|(k, _)| k);
        self.open = !self.open;
        if self.open {
            if self.t <= 0.0 {
                self.sign = swing_sign(def, doorway, user.map(|(_, at)| at));
                self.opener = key;
            }
        } else {
            self.opener = None;
        }
        self.close_at = None;
        DoorChanged { placement, open: self.open, user: key, tick }
    }

    /// The state after `dt_s` of unblocked swing (`swing_s` = closed to open), without committing:
    /// the shell sweeps this pose for a blocker first. Lands exactly on 0 / 1; `swing_s <= 0`
    /// arrives at once.
    #[must_use]
    pub fn stepped(&self, dt_s: f32, swing_s: f32) -> Self {
        let target = self.target();
        let remaining = (target - self.t).abs();
        if remaining <= 0.0 || dt_s <= 0.0 {
            return *self;
        }
        let mut next = *self;
        if swing_s <= 0.0 {
            next.t = target;
            return next;
        }
        let step = dt_s / swing_s;
        next.t = if remaining - step <= step * ARRIVE_SLACK {
            target
        } else if self.open {
            self.t + step
        } else {
            self.t - step
        };
        next
    }

    /// Moves `t` toward the target by `dt_s / swing_s`; `blocked` = a pause (no movement, the
    /// target is kept). Returns whether `t` changed.
    pub fn step(&mut self, dt_s: f32, swing_s: f32, blocked: bool) -> bool {
        if blocked {
            return false;
        }
        let next = self.stepped(dt_s, swing_s);
        let changed = next.t.to_bits() != self.t.to_bits();
        *self = next;
        changed
    }

    /// The leaf angle about the hinge axis, radians: `sign * smoothstep(t) * open_deg`.
    #[must_use]
    pub fn angle_rad(&self, def: &DoorDef) -> f32 {
        let t = self.t.clamp(0.0, 1.0);
        let eased = t * t * (3.0 - 2.0 * t);
        f32::from(self.sign) * eased * def.open_deg.to_radians()
    }

    /// The leaf's world frame: `object` turned by [`Self::angle_rad`] about `def.axis` through
    /// `def.hinge`. Closed (or a degenerate axis) returns `object` unchanged.
    #[must_use]
    pub fn leaf(&self, def: &DoorDef, object: &Frame) -> Frame {
        let angle = self.angle_rad(def);
        let Some(axis) = def.axis.try_normalize() else { return *object };
        if angle == 0.0 {
            return *object;
        }
        let q = Quat::from_axis_angle(axis, angle);
        compose(object, &Frame { translation: def.hinge - q * def.hinge, rotation: q })
    }

    /// A user leaves the doorway: when the door closes behind and this user opened it, schedule the
    /// close `close_behind_ticks` from `tick` and forget the opener; otherwise nothing.
    pub fn released(&mut self, user: UserKey, def: &DoorDef, tick: Tick, close_behind_ticks: u64) {
        if def.close_behind && self.open && self.opener == Some(user) {
            self.close_at = Some(tick.plus(close_behind_ticks));
            self.opener = None;
        }
    }

    /// Fires a due close-behind once: clears the schedule and, if still open, closes with no user.
    pub fn poll(&mut self, placement: PlacementId, def: &DoorDef, doorway: Vec3, tick: Tick) -> Option<DoorChanged> {
        let due = self.close_at.is_some_and(|at| at <= tick);
        if !due {
            return None;
        }
        self.close_at = None;
        self.open.then(|| self.toggle(placement, def, doorway, None, tick))
    }
}
