//! Hands on the lip (action.md B2b): where a mantle's hands grip the ledge and how strongly hand
//! IK pulls them there over the move, fixed when the warp is planned ([`crate::WarpState::grip`]).
//! The host feeds the targets and weights to its animation graph's hand IK.
//!
//! From the slot clip's baked hand contacts ([`sf_anim::RootPath::contacts`]): each hand holds
//! from its contact's start to its end (clip seconds, the warp progress). The target sits on the
//! lip line at the hand's baked lateral offset (clamped to [`GRIP_LATERAL`]) along the edge,
//! [`GRIP_INSET`] in from the face and [`GRIP_RISE`] above the top (the wrist joint over the
//! top). A hand without a contact mirrors the other hand's window (a one-handed clip grips with
//! both). A hand whose baked contact, carried by the warped root, lands over [`GRIP_MAX_GAP`] from
//! its target is skipped (`HandsOnLip`): IK would drag the arm across the ledge.
//!
//! Weights ramp in (smoothstep) over [`GRIP_RAMP`] before the hold, stay 1 through it and ramp
//! out over [`GRIP_RAMP`] after the release.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::{ContactKind, RootPath, Side, WarpPlan};

/// Lateral offset of a hand from the lip point along the edge, m (min, max).
pub const GRIP_LATERAL: (f32, f32) = (0.18, 0.32);
/// How far in from the face the target sits, m.
pub const GRIP_INSET: f32 = 0.04;
/// How far above the top the target (the wrist joint) sits, m.
pub const GRIP_RISE: f32 = 0.03;
/// A hold whose baked contact lands further than this from its target is skipped, m.
pub const GRIP_MAX_GAP: f32 = 0.5;
/// Weight ramp before the hold and after the release, clip seconds.
pub const GRIP_RAMP: f32 = 0.10;

/// Index of the left hand in [`LedgeGrip`]'s arrays.
pub const LEFT: usize = 0;
/// Index of the right hand in [`LedgeGrip`]'s arrays.
pub const RIGHT: usize = 1;

/// One hand's hold on the lip.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandHold {
    /// The grip point relative to the warp anchor (the start-tick feet), world axes.
    pub target: Vec3,
    /// Clip seconds the hand holds (warp progress), start to release.
    pub start: f32,
    pub end: f32,
    /// Distance from the target to where the warped clip puts the hand at `start`, m.
    pub gap: f32,
    /// Mirrored from the other hand's contact.
    pub mirrored: bool,
}

impl HandHold {
    /// IK weight at clip time `t` ([`hold_weight`]).
    #[must_use]
    pub fn weight(&self, t: f32) -> f32 {
        hold_weight(self.start, self.end, t)
    }
}

/// Both hands' holds of a mantle, `[left, right]` ([`LEFT`], [`RIGHT`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LedgeGrip {
    /// The holds to feed; `None`: that hand stays animated.
    pub hands: [Option<HandHold>; 2],
    /// Holds refused by the gap check (the host logs them).
    pub skipped: [Option<HandHold>; 2],
}

impl LedgeGrip {
    /// The grip of `path` (the warped slot clip) under `plan`, on the lip at `lip` (on the top,
    /// from the anchor) of a face whose flat inward direction is `into` (unit). `None` when the
    /// path has no hand contact.
    #[must_use]
    pub fn plan(path: &(impl RootPath + ?Sized), plan: &WarpPlan, lip: Vec3, into: Vec3) -> Option<Self> {
        let first = |side| path.contacts().iter().find(|c| c.kind == ContactKind::Hand && c.side == side);
        let (left, right) = (first(Side::Left), first(Side::Right));
        if left.is_none() && right.is_none() {
            return None;
        }
        // Clip +X is the character's left; the edge runs along it, facing into the face.
        let edge = Quat::from_rotation_y(into.x.atan2(into.z)) * Vec3::X;
        let mut grip = Self::default();
        for (i, own, other) in [(LEFT, left, right), (RIGHT, right, left)] {
            let Some((c, mirrored)) = own.map(|c| (c, false)).or_else(|| other.map(|c| (c, true))) else { continue };
            let sign = if i == LEFT { 1.0 } else { -1.0 };
            let lateral = sign * c.local.x.abs().clamp(GRIP_LATERAL.0, GRIP_LATERAL.1);
            let target = lip + edge * lateral + into * GRIP_INSET + Vec3::Y * GRIP_RISE;
            // Where the clip plants the hand, carried by the warped root (mirrored: the other's, across).
            let local = if mirrored { c.local * Vec3::new(-1.0, 1.0, 1.0) } else { c.local };
            let root = plan.root_at(path, c.start);
            let baked = root.position + Quat::from_rotation_y(root.yaw) * local;
            let hold = HandHold { target, start: c.start, end: c.end.max(c.start), gap: target.distance(baked), mirrored };
            if hold.gap <= GRIP_MAX_GAP && target.is_finite() {
                grip.hands[i] = Some(hold);
            } else {
                grip.skipped[i] = Some(hold);
            }
        }
        Some(grip)
    }

    /// Each hand's target and IK weight at clip time `t`.
    #[must_use]
    pub fn at(&self, t: f32) -> [Option<(Vec3, f32)>; 2] {
        self.hands.map(|h| h.map(|h| (h.target, h.weight(t))))
    }
}

/// A hold's IK weight at clip time `t`: smoothstep in over [`GRIP_RAMP`] before `start`, 1 from
/// `start` to `end`, smoothstep out over [`GRIP_RAMP`] after `end`, else 0 (also for a
/// non-finite `t`).
#[must_use]
pub fn hold_weight(start: f32, end: f32, t: f32) -> f32 {
    let smooth = |x: f32| {
        let x = x.clamp(0.0, 1.0);
        x * x * (3.0 - 2.0 * x)
    };
    if !t.is_finite() {
        0.0
    } else if t < start {
        smooth(1.0 - (start - t) / GRIP_RAMP)
    } else if t > end {
        smooth(1.0 - (t - end) / GRIP_RAMP)
    } else {
        1.0
    }
}
