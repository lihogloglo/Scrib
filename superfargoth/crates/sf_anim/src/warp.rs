//! Chain warping (action.md "Motion owners in detail" → Warp; animation-layers.md "Decided shape
//! (R11b)" item 2): a move's clip chain is bent onto a target fixed at the move's start, as a
//! closed-form function of the action clock, so gameplay (`sf_action`, fixed tick) and
//! presentation (any frame rate) compute the same root from the same progress.
//!
//! At start, a [`WarpPlan`] fixes the start root, the target, the chain's baked root motion and
//! the correction (target minus where the baked chain ends). At chain time `t`:
//!
//! `root(t) = start ⊕ baked(t) + correction × profile(u(t))`
//!
//! where `u` runs 0 → 1 across the warp window (the Layer 1 `warp` event window of the clip that
//! carries the correction, else that clip's whole span) and the profile is [`WarpProfile::Linear`]
//! or [`WarpProfile::Late`] (cubic: the clip's own rise first, over the lip late; AC1, R3 §5).
//! Yaw is corrected the same way. A zero correction reproduces the baked root exactly; the window's
//! end lands exactly on the target. The plan is refused when the correction is too large a share
//! of the chain's own displacement ([`WarpPlan::ratio`]).
//!
//! Layer 1's per-update skew warp (`sf_anim_graph`, `Drive` players) stays for moving targets
//! (attack tracking); this is the fixed-target, progress-driven form for `Progress` moves.

use std::sync::Arc;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::math::wrap_angle;
use crate::{Clip, Contact};

/// A root position and heading (radians about +Y, 0 faces +Z).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RootPose {
    pub position: Vec3,
    pub yaw: f32,
}

impl RootPose {
    pub const IDENTITY: Self = Self { position: Vec3::ZERO, yaw: 0.0 };

    pub const fn new(position: Vec3, yaw: f32) -> Self {
        Self { position, yaw }
    }

    /// `local` (in this pose's frame) carried into this pose's space.
    #[must_use]
    pub fn then(self, local: Self) -> Self {
        Self { position: self.position + Quat::from_rotation_y(self.yaw) * local.position, yaw: self.yaw + local.yaw }
    }
}

/// How the correction is spread over the warp window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarpProfile {
    /// In proportion to window time.
    #[default]
    Linear,
    /// Cubic (`u³`): small early, most of it at the end (pull-ups: rise first, over the lip late).
    Late,
}

impl WarpProfile {
    /// Share of the correction applied at window fraction `u` (clamped to 0–1).
    pub fn weight(self, u: f32) -> f32 {
        let u = u.clamp(0.0, 1.0);
        match self {
            Self::Linear => u,
            Self::Late => u * u * u,
        }
    }
}

/// Baked root motion over time: one clip or a chain of them. Times are seconds from the path's
/// start; roots are relative to the path's start (its first frame is the identity).
pub trait RootPath {
    /// Seconds.
    fn duration(&self) -> f32;
    /// Root position and heading at `t` (clamped to the path), in the path's start frame.
    fn root_at(&self, t: f32) -> RootPose;
    /// The contact-bounded window the correction may be applied in, path seconds (`None`: the
    /// whole path).
    fn warp_window(&self) -> Option<(f32, f32)> {
        None
    }
    /// Baked hand and foot contacts ([`crate::ClipMeta::contacts`]), path seconds (empty: none
    /// known; a [`RootChain`] reports none, its parts' times being their own).
    fn contacts(&self) -> &[Contact] {
        &[]
    }
}

impl RootPath for Clip {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn root_at(&self, t: f32) -> RootPose {
        let (p, yaw) = Clip::root_at(self, t);
        RootPose::new(p, yaw)
    }

    /// The first `warp` event window (animation-layers.md Layer 1 windows).
    fn warp_window(&self) -> Option<(f32, f32)> {
        self.events.iter().find(|e| e.name == "warp").and_then(|e| Some((e.time, e.end?)))
    }

    fn contacts(&self) -> &[Contact] {
        &self.meta.contacts
    }
}

impl<T: RootPath + ?Sized> RootPath for &T {
    fn duration(&self) -> f32 {
        (**self).duration()
    }

    fn root_at(&self, t: f32) -> RootPose {
        (**self).root_at(t)
    }

    fn warp_window(&self) -> Option<(f32, f32)> {
        (**self).warp_window()
    }

    fn contacts(&self) -> &[Contact] {
        (**self).contacts()
    }
}

impl<T: RootPath + ?Sized> RootPath for Arc<T> {
    fn duration(&self) -> f32 {
        (**self).duration()
    }

    fn root_at(&self, t: f32) -> RootPose {
        (**self).root_at(t)
    }

    fn warp_window(&self) -> Option<(f32, f32)> {
        (**self).warp_window()
    }

    fn contacts(&self) -> &[Contact] {
        (**self).contacts()
    }
}

/// Clips played back to back (a slot chain: enter, loop, exit), each from its start to its end;
/// each part's root motion continues from where the previous part's ended.
#[derive(Clone, Debug)]
pub struct RootChain<P> {
    parts: Vec<P>,
    /// Per part: chain time it starts at and its start root in the chain's frame.
    starts: Vec<(f32, RootPose)>,
    duration: f32,
}

impl<P: RootPath> RootChain<P> {
    pub fn new(parts: Vec<P>) -> Self {
        let mut starts = Vec::with_capacity(parts.len());
        let (mut t, mut at) = (0.0, RootPose::IDENTITY);
        for p in &parts {
            starts.push((t, at));
            let d = p.duration().max(0.0);
            at = at.then(p.root_at(d));
            t += d;
        }
        Self { parts, starts, duration: t }
    }

    pub fn parts(&self) -> &[P] {
        &self.parts
    }

    /// Chain time where part `i` starts.
    pub fn part_start(&self, i: usize) -> f32 {
        self.starts.get(i).map_or(self.duration, |s| s.0)
    }

    /// The part playing at chain time `t` and the time inside it (a boundary belongs to the next
    /// part; past the end, the last part at its end).
    pub fn locate(&self, t: f32) -> Option<(usize, f32)> {
        let i = self.starts.iter().rposition(|s| t >= s.0).unwrap_or(0);
        let part = self.parts.get(i)?;
        Some((i, (t - self.starts[i].0).clamp(0.0, part.duration().max(0.0))))
    }

    /// The part that carries the correction: among parts with a warp window (else all), the one
    /// whose root moves furthest (R3 §5).
    pub fn carrier(&self) -> Option<usize> {
        let windowed = self.parts.iter().any(|p| p.warp_window().is_some());
        let reach = |p: &P| p.root_at(p.duration()).position.length();
        self.parts
            .iter()
            .enumerate()
            .filter(|(_, p)| !windowed || p.warp_window().is_some())
            .max_by(|a, b| reach(a.1).total_cmp(&reach(b.1)))
            .map(|(i, _)| i)
    }
}

impl<P: RootPath> RootPath for RootChain<P> {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn root_at(&self, t: f32) -> RootPose {
        match self.locate(t) {
            Some((i, local)) => self.starts[i].1.then(self.parts[i].root_at(local)),
            None => RootPose::IDENTITY,
        }
    }

    /// The carrier's window (or its whole span) in chain time.
    fn warp_window(&self) -> Option<(f32, f32)> {
        let i = self.carrier()?;
        let (start, d) = (self.starts[i].0, self.parts[i].duration().max(0.0));
        let (a, b) = self.parts[i].warp_window().unwrap_or((0.0, d));
        Some((start + a.clamp(0.0, d), start + b.clamp(0.0, d)))
    }
}

/// A plan refused: the correction is too large a share of the chain's own displacement.
#[derive(Clone, Copy, Debug, PartialEq, thiserror::Error)]
#[error("warp correction {correction:.3} m is {ratio:.2} of the root distance (refuse above {refuse_ratio:.2})")]
pub struct WarpRefused {
    pub correction: f32,
    pub ratio: f32,
    pub refuse_ratio: f32,
}

/// A chain warp fixed at the move's start. Plain data (serde): gameplay can keep it in its saved,
/// replicated state; the clips stay with the caller and are passed back in each call.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarpPlan {
    /// Root at progress 0.
    pub start: RootPose,
    pub target: RootPose,
    pub profile: WarpProfile,
    /// Path seconds the correction is spread over.
    pub window: (f32, f32),
    /// The path's baked root at its end, in the start's frame.
    pub baked_end: RootPose,
    /// Target minus the baked end, world (start's parent) space.
    pub correction: Vec3,
    /// Radians, wrapped to `(-π, π]`.
    pub yaw_correction: f32,
}

impl WarpPlan {
    /// Plans `path` from `start` onto `target` (both in the same space). The window is the path's
    /// [`RootPath::warp_window`], else the whole path.
    pub fn new(path: &(impl RootPath + ?Sized), start: RootPose, target: RootPose, profile: WarpProfile) -> Self {
        let d = path.duration().max(0.0);
        let window = path.warp_window().map_or((0.0, d), |(a, b)| (a.clamp(0.0, d), b.clamp(a.clamp(0.0, d), d)));
        let baked_end = path.root_at(d);
        let end = start.then(baked_end);
        Self {
            start,
            target,
            profile,
            window,
            baked_end,
            correction: target.position - end.position,
            yaw_correction: wrap_angle(target.yaw - end.yaw),
        }
    }

    /// [`WarpPlan::new`], refused above `refuse_ratio` ([`WarpPlan::ratio`]).
    ///
    /// # Errors
    /// The correction exceeds `refuse_ratio` of the root distance.
    pub fn try_new(
        path: &(impl RootPath + ?Sized),
        start: RootPose,
        target: RootPose,
        profile: WarpProfile,
        refuse_ratio: f32,
    ) -> Result<Self, WarpRefused> {
        let plan = Self::new(path, start, target, profile);
        plan.check(refuse_ratio)?;
        Ok(plan)
    }

    /// Correction length over the chain's own root displacement (start to end, 3D). A chain that
    /// doesn't move has ratio 0 when no correction is needed (under 1 mm), else infinity.
    pub fn ratio(&self) -> f32 {
        let (c, d) = (self.correction.length(), self.baked_end.position.length());
        if d > 1e-4 {
            c / d
        } else if c <= 1e-3 {
            0.0
        } else {
            f32::INFINITY
        }
    }

    /// # Errors
    /// [`WarpPlan::ratio`] is above `refuse_ratio` (equal passes).
    pub fn check(&self, refuse_ratio: f32) -> Result<(), WarpRefused> {
        let ratio = self.ratio();
        if ratio > refuse_ratio {
            return Err(WarpRefused { correction: self.correction.length(), ratio, refuse_ratio });
        }
        Ok(())
    }

    /// Share of the correction applied at path time `t`: 0 before the window, 1 after it.
    pub fn weight(&self, t: f32) -> f32 {
        let (a, b) = self.window;
        let u = if b - a > 1e-6 {
            (t - a) / (b - a)
        } else if t >= b {
            1.0
        } else {
            0.0
        };
        self.profile.weight(u)
    }

    /// The gameplay root at path time `t` (the action clock's progress, seconds): `start ⊕
    /// baked(t) + correction × profile(t)`, yaw alike. `path` must be the one planned.
    pub fn root_at(&self, path: &(impl RootPath + ?Sized), t: f32) -> RootPose {
        let k = self.weight(t);
        let baked = self.start.then(path.root_at(t));
        RootPose::new(baked.position + self.correction * k, baked.yaw + self.yaw_correction * k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A straight path to `to` over `d` s, turning `turn` rad.
    struct Line {
        d: f32,
        to: Vec3,
        turn: f32,
        window: Option<(f32, f32)>,
    }

    impl RootPath for Line {
        fn duration(&self) -> f32 {
            self.d
        }
        fn root_at(&self, t: f32) -> RootPose {
            let u = (t / self.d).clamp(0.0, 1.0);
            RootPose::new(self.to * u, self.turn * u)
        }
        fn warp_window(&self) -> Option<(f32, f32)> {
            self.window
        }
    }

    fn chain() -> RootChain<Line> {
        RootChain::new(vec![
            Line { d: 0.5, to: Vec3::new(0.0, 0.4, 0.3), turn: 0.3, window: None },
            Line { d: 0.8, to: Vec3::new(0.0, 0.8, 0.9), turn: 0.0, window: Some((0.1, 0.6)) },
        ])
    }

    #[test]
    fn chains_compose_root_motion_and_pick_the_windowed_carrier() {
        let c = chain();
        assert!((c.duration() - 1.3).abs() < 1e-6);
        assert_eq!(c.carrier(), Some(1));
        assert_eq!(c.warp_window(), Some((0.6, 1.1)));
        // The second part continues from the first's end, turned by its 0.3 rad.
        let end = c.root_at(1.3);
        let expect = Vec3::new(0.0, 0.4, 0.3) + Quat::from_rotation_y(0.3) * Vec3::new(0.0, 0.8, 0.9);
        assert!((end.position - expect).length() < 1e-5 && (end.yaw - 0.3).abs() < 1e-6);
        assert_eq!(c.locate(0.5).map(|(i, _)| i), Some(1));
        assert_eq!(c.locate(2.0), Some((1, 0.8)));
    }

    #[test]
    fn profiles_land_on_the_target_and_late_stays_small_early() {
        let c = chain();
        let start = RootPose::new(Vec3::new(10.0, 2.0, -4.0), 1.2);
        let baked = start.then(c.root_at(1.3));
        let target = RootPose::new(baked.position + Vec3::new(0.15, 0.2, -0.1), baked.yaw + 0.2);
        for profile in [WarpProfile::Linear, WarpProfile::Late] {
            let plan = WarpPlan::new(&c, start, target, profile);
            let end = plan.root_at(&c, c.duration());
            assert!((end.position - target.position).length() < 1e-4, "{profile:?}");
            assert!(wrap_angle(end.yaw - target.yaw).abs() < 1e-5, "{profile:?}");
            // Nothing before the window, everything after it.
            assert!(plan.weight(0.59).abs() < f32::EPSILON && (plan.weight(1.11) - 1.0).abs() < f32::EPSILON);
        }
        let mid = 0.85;
        let (lin, late) = (WarpProfile::Linear, WarpProfile::Late);
        assert!((WarpPlan::new(&c, start, target, lin).weight(mid) - 0.5).abs() < 1e-5);
        assert!((WarpPlan::new(&c, start, target, late).weight(mid) - 0.125).abs() < 1e-5);
    }

    #[test]
    fn refusal_compares_the_correction_with_the_root_distance() {
        let c = chain();
        let start = RootPose::IDENTITY;
        let baked = c.root_at(1.3);
        let dist = baked.position.length();
        let off = |share: f32| RootPose::new(baked.position + Vec3::X * dist * share, baked.yaw);
        assert!(WarpPlan::try_new(&c, start, off(0.30), WarpProfile::Late, 0.35).is_ok());
        let err = WarpPlan::try_new(&c, start, off(0.40), WarpProfile::Late, 0.35).unwrap_err();
        assert!((err.ratio - 0.40).abs() < 1e-4);
        // A still chain: no correction passes, any correction refuses.
        let still = Line { d: 1.0, to: Vec3::ZERO, turn: 0.0, window: None };
        assert!(WarpPlan::try_new(&still, start, start, WarpProfile::Linear, 0.1).is_ok());
        assert!(WarpPlan::try_new(&still, start, RootPose::new(Vec3::X * 0.05, 0.0), WarpProfile::Linear, 0.1).is_err());
    }
}
