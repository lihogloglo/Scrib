//! Animation data from NIF controllers, and its evaluation.
//!
//! Every NIF key type reduces to one of two forms at load:
//! - **Hermite curves** ([`Curve`]) for floats, positions, and colors.
//!   Linear keys become Hermite keys whose tangents are the segment deltas
//!   (which interpolates exactly linearly), Bezier keys already store
//!   Hermite tangents, and TCB keys get Kochanek–Bartels tangents.
//! - **Rotation tracks** ([`RotationTrack`]): quaternion keys (slerp) or
//!   three Euler angle curves.
//!
//! Values are converted to engine space (see [`crate::coords`]) when the
//! model is built, so sampling needs no further conversion.

use std::ops::{Add, Mul, Sub};

use tes3::nif::glam::{Quat, Vec3, Vec4};
use tes3::nif::{AxisOrder, CycleType, NiFloatKey, NiPosKey, NiRotKey, NiTimeController};

use crate::coords;

/// What happens outside a controller's `[start, stop]` range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cycle {
    Loop,
    /// Plays forward, then backward.
    PingPong,
    /// Holds the end values.
    Clamp,
}

/// Maps application time to a controller's key time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timing {
    pub frequency: f32,
    pub phase: f32,
    pub start: f32,
    pub stop: f32,
    pub cycle: Cycle,
}

impl Default for Timing {
    fn default() -> Self {
        Self { frequency: 1.0, phase: 0.0, start: 0.0, stop: 0.0, cycle: Cycle::Loop }
    }
}

impl Timing {
    pub(crate) fn from_controller(controller: &NiTimeController) -> Self {
        Self {
            frequency: controller.frequency,
            phase: controller.phase,
            start: controller.start_time,
            stop: controller.stop_time,
            cycle: match controller.cycle_type() {
                CycleType::Cycle => Cycle::Loop,
                CycleType::Reverse => Cycle::PingPong,
                CycleType::Clamp => Cycle::Clamp,
            },
        }
    }

    /// Key time for application time `time` (seconds).
    pub fn key_time(&self, time: f32) -> f32 {
        let t = time * self.frequency + self.phase;
        let length = self.stop - self.start;
        if length <= 0.0 {
            return self.start;
        }
        match self.cycle {
            Cycle::Loop => self.start + (t - self.start).rem_euclid(length),
            Cycle::PingPong => {
                let u = (t - self.start).rem_euclid(2.0 * length);
                self.start + if u > length { 2.0 * length - u } else { u }
            }
            Cycle::Clamp => t.clamp(self.start, self.stop),
        }
    }
}

/// Values a [`Curve`] can interpolate.
pub trait Interpolate: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self> {}
impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>> Interpolate for T {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Key<T> {
    time: f32,
    value: T,
    /// Tangent arriving at this key (for the segment before it).
    in_tan: T,
    /// Tangent leaving this key (for the segment after it).
    out_tan: T,
}

/// A cubic Hermite curve; holds the end values outside its keys.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve<T> {
    keys: Vec<Key<T>>,
}

impl<T> Default for Curve<T> {
    fn default() -> Self {
        Self { keys: Vec::new() }
    }
}

impl<T: Interpolate> Curve<T> {
    /// Linear interpolation through `(time, value)` keys.
    pub fn linear(keys: impl IntoIterator<Item = (f32, T)>) -> Self {
        let points: Vec<(f32, T)> = keys.into_iter().collect();
        let keys = (0..points.len())
            .map(|i| {
                let value = points[i].1;
                let before = if i > 0 { value - points[i - 1].1 } else { value * 0.0 };
                let after = points.get(i + 1).map_or(value * 0.0, |next| next.1 - value);
                Key { time: points[i].0, value, in_tan: before, out_tan: after }
            })
            .collect();
        Self { keys }
    }

    /// Keys with explicit Hermite tangents `(time, value, in, out)`.
    pub fn hermite(keys: impl IntoIterator<Item = (f32, T, T, T)>) -> Self {
        Self { keys: keys.into_iter().map(|(time, value, in_tan, out_tan)| Key { time, value, in_tan, out_tan }).collect() }
    }

    /// Kochanek–Bartels keys `(time, value, tension, continuity, bias)`.
    ///
    /// Tangents are per segment, so an interior key's tangents are scaled
    /// to the duration of the segment they serve: the incoming one by
    /// `2 dt_prev / (dt_prev + dt_next)`, the outgoing one by
    /// `2 dt_next / (dt_prev + dt_next)` (Kochanek & Bartels 1984, "speed
    /// control"; `Gamebryo` and `OpenMW` do the same). Without it, a short
    /// segment next to a long one gets the long one's tangent and swings
    /// far past its keys.
    pub fn tcb(keys: impl IntoIterator<Item = (f32, T, f32, f32, f32)>) -> Self {
        let points: Vec<(f32, T, f32, f32, f32)> = keys.into_iter().collect();
        let keys = (0..points.len())
            .map(|i| {
                let (time, value, t, c, b) = points[i];
                // End keys reuse their one neighbor on both sides.
                let prev = if i > 0 { points[i - 1].1 } else { value };
                let next = points.get(i + 1).map_or(value, |n| n.1);
                let (incoming, outgoing) = (value - prev, next - value);
                let ((incoming, outgoing), (in_scale, out_scale)) = match (i > 0, i + 1 < points.len()) {
                    (false, true) => ((outgoing, outgoing), (1.0, 1.0)),
                    (true, false) => ((incoming, incoming), (1.0, 1.0)),
                    (true, true) => {
                        let (before, after) = (time - points[i - 1].0, points[i + 1].0 - time);
                        let span = before + after;
                        let scales = if span > 0.0 { (2.0 * before / span, 2.0 * after / span) } else { (1.0, 1.0) };
                        ((incoming, outgoing), scales)
                    }
                    (false, false) => ((incoming, outgoing), (1.0, 1.0)),
                };
                let in_tan = (incoming * ((1.0 - t) * (1.0 - c) * (1.0 + b) / 2.0)
                    + outgoing * ((1.0 - t) * (1.0 + c) * (1.0 - b) / 2.0))
                    * in_scale;
                let out_tan = (incoming * ((1.0 - t) * (1.0 + c) * (1.0 + b) / 2.0)
                    + outgoing * ((1.0 - t) * (1.0 - c) * (1.0 - b) / 2.0))
                    * out_scale;
                Key { time, value, in_tan, out_tan }
            })
            .collect();
        Self { keys }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Keys as `(time, value, in_tangent, out_tangent)`. Tangents are per
    /// segment (Hermite on `x` in `[0, 1]`): divide by the adjacent
    /// segment's duration for units per second.
    pub fn keys(&self) -> impl ExactSizeIterator<Item = (f32, T, T, T)> + '_ {
        self.keys.iter().map(|k| (k.time, k.value, k.in_tan, k.out_tan))
    }

    /// Value at key time `t`.
    pub fn sample(&self, t: f32) -> Option<T> {
        let (first, last) = (self.keys.first()?, self.keys.last()?);
        if t <= first.time {
            return Some(first.value);
        }
        if t >= last.time {
            return Some(last.value);
        }
        let i = self.keys.partition_point(|k| k.time <= t);
        let (from, to) = (&self.keys[i - 1], &self.keys[i]);
        let span = to.time - from.time;
        if span <= 0.0 {
            return Some(to.value);
        }
        let x = (t - from.time) / span;
        let (x2, x3) = (x * x, x * x * x);
        Some(
            from.value * (2.0 * x3 - 3.0 * x2 + 1.0)
                + to.value * (3.0 * x2 - 2.0 * x3)
                + from.out_tan * (x3 - 2.0 * x2 + x)
                + to.in_tan * (x3 - x2),
        )
    }

    /// Applies a *linear* map to values and tangents (unit or basis changes).
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Curve<U> {
        Curve {
            keys: self
                .keys
                .into_iter()
                .map(|k| Key { time: k.time, value: f(k.value), in_tan: f(k.in_tan), out_tan: f(k.out_tan) })
                .collect(),
        }
    }

    /// First and last key times.
    pub fn time_range(&self) -> Option<(f32, f32)> {
        Some((self.keys.first()?.time, self.keys.last()?.time))
    }
}

pub(crate) fn float_curve(keys: &NiFloatKey) -> Curve<f32> {
    match keys {
        NiFloatKey::LinKey(keys) => Curve::linear(keys.iter().map(|k| (k.time, k.value))),
        NiFloatKey::BezKey(keys) => Curve::hermite(keys.iter().map(|k| (k.time, k.value, k.in_tan, k.out_tan))),
        NiFloatKey::TCBKey(keys) => {
            Curve::tcb(keys.iter().map(|k| (k.time, k.value, k.tension, k.continuity, k.bias)))
        }
    }
}

/// Position-like keys, as stored (no coordinate conversion).
pub(crate) fn vec3_curve(keys: &NiPosKey) -> Curve<Vec3> {
    match keys {
        NiPosKey::LinKey(keys) => Curve::linear(keys.iter().map(|k| (k.time, k.value))),
        NiPosKey::BezKey(keys) => Curve::hermite(keys.iter().map(|k| (k.time, k.value, k.in_tan, k.out_tan))),
        NiPosKey::TCBKey(keys) => Curve::tcb(keys.iter().map(|k| (k.time, k.value, k.tension, k.continuity, k.bias))),
    }
}

/// Step keys: the value of the last key at or before `t`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Steps<T> {
    keys: Vec<(f32, T)>,
}

impl<T: Copy> Steps<T> {
    pub fn new(keys: impl IntoIterator<Item = (f32, T)>) -> Self {
        Self { keys: keys.into_iter().collect() }
    }

    /// `(time, value)` in time order.
    pub fn keys(&self) -> &[(f32, T)] {
        &self.keys
    }

    pub fn sample(&self, t: f32) -> Option<T> {
        let i = self.keys.partition_point(|k| k.0 <= t);
        self.keys.get(i.saturating_sub(1)).map(|k| k.1)
    }

    pub fn time_range(&self) -> Option<(f32, f32)> {
        Some((self.keys.first()?.0, self.keys.last()?.0))
    }
}

/// Rotation keys, in engine space.
#[derive(Clone, Debug, PartialEq)]
pub enum RotationTrack {
    /// Spherical interpolation between key orientations.
    Quats(Vec<(f32, Quat)>),
    /// Three angle curves (radians, Morrowind axes) applied in `order`,
    /// first axis first. Converted to engine space after composing.
    Euler { order: [Vec3; 3], axes: [Curve<f32>; 3] },
}

impl RotationTrack {
    pub(crate) fn from_keys(keys: &NiRotKey) -> Option<Self> {
        let quats: Vec<(f32, Quat)> = match keys {
            NiRotKey::LinKey(keys) => keys.iter().map(|k| (k.time, k.value)).collect(),
            NiRotKey::BezKey(keys) => keys.iter().map(|k| (k.time, k.value)).collect(),
            // TCB rotations (1828 of ~11k rotation channels in the base game,
            // 1096 with non-zero parameters) are slerped through their keys, as
            // OpenMW does; Gamebryo fits a squad spline. Measured and kept (squad
            // departs a few degrees): nif-runtime.md "Keyframe conventions".
            NiRotKey::TCBKey(keys) => keys.iter().map(|k| (k.time, k.value)).collect(),
            NiRotKey::EulerKey(euler) => {
                let order = euler_axes(euler.axis_order);
                let axes = [0, 1, 2].map(|i| float_curve(&euler.axes[i].keys));
                if axes.iter().all(Curve::is_empty) {
                    return None;
                }
                return Some(Self::Euler { order, axes });
            }
        };
        if quats.is_empty() {
            return None;
        }
        Some(Self::Quats(quats.into_iter().map(|(t, q)| (t, coords::rotation(q))).collect()))
    }

    /// Engine-space rotation at key time `t`.
    pub fn sample(&self, t: f32) -> Option<Quat> {
        match self {
            Self::Quats(keys) => {
                let (first, last) = (keys.first()?, keys.last()?);
                if t <= first.0 {
                    return Some(first.1);
                }
                if t >= last.0 {
                    return Some(last.1);
                }
                let i = keys.partition_point(|k| k.0 <= t);
                let ((t0, from), (t1, to)) = (keys[i - 1], keys[i]);
                let x = if t1 > t0 { (t - t0) / (t1 - t0) } else { 1.0 };
                Some(from.slerp(to, x).normalize())
            }
            Self::Euler { order, axes } => {
                let mut q = Quat::IDENTITY;
                for (axis, curve) in order.iter().zip(axes) {
                    let angle = curve.sample(t).unwrap_or(0.0);
                    // Later axes apply after earlier ones.
                    q = Quat::from_axis_angle(*axis, angle) * q;
                }
                Some(coords::rotation(q))
            }
        }
    }

    pub fn time_range(&self) -> Option<(f32, f32)> {
        match self {
            Self::Quats(keys) => Some((keys.first()?.0, keys.last()?.0)),
            Self::Euler { axes, .. } => axes.iter().filter_map(Curve::time_range).reduce(|a, b| (a.0.min(b.0), a.1.max(b.1))),
        }
    }
}

fn euler_axes(order: AxisOrder) -> [Vec3; 3] {
    let (x, y, z) = (Vec3::X, Vec3::Y, Vec3::Z);
    match order {
        AxisOrder::XYZ => [x, y, z],
        AxisOrder::XZY => [x, z, y],
        AxisOrder::YZX => [y, z, x],
        AxisOrder::YXZ => [y, x, z],
        AxisOrder::ZXY => [z, x, y],
        AxisOrder::ZYX => [z, y, x],
        AxisOrder::XYX => [x, y, x],
        AxisOrder::YZY => [y, z, y],
        AxisOrder::ZXZ => [z, x, z],
    }
}

/// A node's animated transform; tracks without keys leave that part alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransformTrack {
    /// Engine-space translation (meters).
    pub translation: Option<Curve<Vec3>>,
    pub rotation: Option<RotationTrack>,
    pub scale: Option<Curve<f32>>,
}

/// Which material color a [`Target::MaterialColor`] drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSlot {
    Ambient,
    Diffuse,
    Specular,
    Emissive,
}

/// Texture coordinate animation (offset and tiling per axis).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UvTrack {
    pub offset_u: Curve<f32>,
    pub offset_v: Curve<f32>,
    pub tiling_u: Curve<f32>,
    pub tiling_v: Curve<f32>,
}

impl UvTrack {
    /// `(offset, tiling)` at key time `t`; missing curves give 0 and 1.
    pub fn sample(&self, t: f32) -> ([f32; 2], [f32; 2]) {
        (
            [self.offset_u.sample(t).unwrap_or(0.0), self.offset_v.sample(t).unwrap_or(0.0)],
            [self.tiling_u.sample(t).unwrap_or(1.0), self.tiling_v.sample(t).unwrap_or(1.0)],
        )
    }
}

/// What a controller animates.
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    /// Index into [`crate::Model::nodes`].
    Transform { node: usize, track: TransformTrack },
    Visibility { node: usize, keys: Steps<bool> },
    /// Index into [`crate::Model::materials`]; drives `alpha`.
    Alpha { material: usize, curve: Curve<f32> },
    MaterialColor { material: usize, slot: ColorSlot, curve: Curve<Vec3> },
    Uv { material: usize, track: UvTrack },
}

/// One animation channel of a model.
#[derive(Clone, Debug, PartialEq)]
pub struct Controller {
    pub timing: Timing,
    /// Plays by itself (looping ambient animation). Otherwise it waits for
    /// gameplay to drive it (actor animation, scripted effects).
    pub auto_play: bool,
    /// Instances may start at a random point of the cycle.
    pub random_start: bool,
    pub target: Target,
}

impl Controller {
    /// Key range covered by the data, for timings that leave it empty.
    pub(crate) fn key_range(&self) -> Option<(f32, f32)> {
        match &self.target {
            Target::Transform { track, .. } => [
                track.translation.as_ref().and_then(Curve::time_range),
                track.rotation.as_ref().and_then(RotationTrack::time_range),
                track.scale.as_ref().and_then(Curve::time_range),
            ]
            .into_iter()
            .flatten()
            .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1))),
            Target::Visibility { keys, .. } => keys.time_range(),
            Target::Alpha { curve, .. } => curve.time_range(),
            Target::MaterialColor { curve, .. } => curve.time_range(),
            Target::Uv { track, .. } => [&track.offset_u, &track.offset_v, &track.tiling_u, &track.tiling_v]
                .into_iter()
                .filter_map(Curve::time_range)
                .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1))),
        }
    }
}

/// Engine-space translation curve from Morrowind-space keys.
pub(crate) fn translation_curve(keys: &NiPosKey) -> Curve<Vec3> {
    vec3_curve(keys).map(coords::point)
}

/// Linear interpolation of RGBA colors.
pub(crate) fn color_curve(keys: impl IntoIterator<Item = (f32, Vec4)>) -> Curve<Vec4> {
    Curve::linear(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn linear_keys_interpolate_linearly() {
        let curve = Curve::linear([(0.0, 0.0), (1.0, 10.0), (3.0, 10.0)]);
        assert!(close(curve.sample(0.25).unwrap(), 2.5));
        assert!(close(curve.sample(0.5).unwrap(), 5.0));
        assert!(close(curve.sample(2.0).unwrap(), 10.0));
        assert!(close(curve.sample(-1.0).unwrap(), 0.0), "holds before the first key");
        assert!(close(curve.sample(9.0).unwrap(), 10.0), "holds after the last key");
    }

    #[test]
    fn hermite_hits_keys_and_follows_tangents() {
        let curve = Curve::hermite([(0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 0.0, 0.0)]);
        assert!(close(curve.sample(0.5).unwrap(), 0.5), "flat tangents: smoothstep midpoint");
        assert!(curve.sample(0.1).unwrap() < 0.1, "eases in");
        let tcb = Curve::tcb([(0.0, 0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 0.0, 0.0, 0.0), (2.0, 0.0, 0.0, 0.0, 0.0)]);
        assert!(close(tcb.sample(1.0).unwrap(), 1.0));
        assert!(tcb.sample(0.5).unwrap() > 0.5, "Catmull-Rom overshoots the chord toward the peak");
    }

    /// `OpenMW`'s `generateTCBTangents` (`components/nif/nifkey.hpp`), written
    /// out independently as the reference: `(in, out)` per key.
    fn reference_tcb(keys: &[(f32, f32, f32, f32, f32)]) -> Vec<(f32, f32)> {
        let coefficients = |&(_, _, t, c, b): &(f32, f32, f32, f32, f32)| {
            (
                (1.0 - t) * (1.0 - c) * (1.0 + b),
                (1.0 - t) * (1.0 + c) * (1.0 - b),
                (1.0 - t) * (1.0 + c) * (1.0 + b),
                (1.0 - t) * (1.0 - c) * (1.0 - b),
            )
        };
        let last = keys.len() - 1;
        (0..=last)
            .map(|i| {
                let (ka, kb, kc, kd) = coefficients(&keys[i]);
                if i == 0 || i == last {
                    let delta = if i == 0 { keys[1].1 - keys[0].1 } else { keys[last].1 - keys[last - 1].1 };
                    return (delta * (ka + kb) * 0.5, delta * (kc + kd) * 0.5);
                }
                let (prev, key, next) = (keys[i - 1], keys[i], keys[i + 1]);
                let span = next.0 - prev.0;
                let (before, after) = (key.1 - prev.1, next.1 - key.1);
                ((before * ka + after * kb) * ((key.0 - prev.0) / span), (before * kc + after * kd) * ((next.0 - key.0) / span))
            })
            .collect()
    }

    #[test]
    fn tcb_tangents_match_the_reference_convention() {
        let keys = [
            (0.0, 1.0, 0.0, 0.0, 0.0),
            (0.4, 3.0, 0.3, -0.2, 0.1),
            (0.5, -2.0, -0.5, 0.4, -0.3),
            (2.0, 4.0, 0.0, -1.0, 0.0),
            (2.1, 0.5, 0.2, 0.2, 0.9),
        ];
        let curve = Curve::tcb(keys);
        for ((_, _, in_tan, out_tan), (want_in, want_out)) in curve.keys().zip(reference_tcb(&keys)) {
            assert!(close(in_tan, want_in) && close(out_tan, want_out), "({in_tan}, {out_tan}) vs ({want_in}, {want_out})");
        }
    }

    #[test]
    fn tcb_unevenly_spaced_collinear_keys_stay_on_the_line() {
        // v = 10 t, keyed at 0, 1 and 1.1 s: a long segment then a short one.
        let curve = Curve::tcb([(0.0, 0.0, 0.0, 0.0, 0.0), (1.0, 10.0, 0.0, 0.0, 0.0), (1.1, 11.0, 0.0, 0.0, 0.0)]);
        for t in [0.5, 1.0, 1.02, 1.05, 1.08] {
            assert!((curve.sample(t).unwrap() - 10.0 * t).abs() < 1e-4, "t {t}: {}", curve.sample(t).unwrap());
        }
        // Even spacing is plain Catmull-Rom: the tangent is half the two-segment chord.
        let even = Curve::tcb([(0.0, 0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 0.0, 0.0, 0.0), (2.0, 5.0, 0.0, 0.0, 0.0)]);
        let (_, _, in_tan, out_tan) = even.keys().nth(1).unwrap();
        assert!(close(in_tan, 2.5) && close(out_tan, 2.5));
    }

    #[test]
    fn quadratic_tangents_are_per_segment() {
        // Hermite on [0, 1] per segment: tangent = velocity x segment duration.
        // v = 3 t keyed at 0, 2 and 2.5 s: tangents 6 and 1.5 at the middle key.
        let curve = Curve::hermite([(0.0, 0.0, 6.0, 6.0), (2.0, 6.0, 6.0, 1.5), (2.5, 7.5, 1.5, 1.5)]);
        for t in [0.7, 2.0, 2.2, 2.4] {
            assert!((curve.sample(t).unwrap() - 3.0 * t).abs() < 1e-4, "t {t}");
        }
    }

    #[test]
    fn timing_cycles() {
        let timing = Timing { start: 1.0, stop: 3.0, ..Timing::default() };
        assert!(close(timing.key_time(3.5), 1.5));
        let pingpong = Timing { cycle: Cycle::PingPong, ..timing };
        assert!(close(pingpong.key_time(3.5), 2.5));
        let clamp = Timing { cycle: Cycle::Clamp, ..timing };
        assert!(close(clamp.key_time(10.0), 3.0));
        let fast = Timing { frequency: 2.0, phase: 0.5, ..timing };
        assert!(close(fast.key_time(1.0), 2.5));
    }

    #[test]
    fn steps_hold_values() {
        let steps = Steps::new([(0.0, true), (1.0, false), (2.0, true)]);
        assert_eq!(steps.sample(0.5), Some(true));
        assert_eq!(steps.sample(1.0), Some(false));
        assert_eq!(steps.sample(5.0), Some(true));
        assert_eq!(steps.sample(-1.0), Some(true));
    }

    #[test]
    fn euler_yaw_matches_quaternion_yaw() {
        let yaw = Curve::linear([(0.0, 0.0), (1.0, std::f32::consts::FRAC_PI_2)]);
        let euler = RotationTrack::Euler { order: euler_axes(AxisOrder::XYZ), axes: [Curve::default(), Curve::default(), yaw] };
        let quats = RotationTrack::Quats(vec![
            (0.0, coords::rotation(Quat::IDENTITY)),
            (1.0, coords::rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))),
        ]);
        let (a, b) = (euler.sample(0.5).unwrap(), quats.sample(0.5).unwrap());
        assert!(a.dot(b).abs() > 0.9999, "{a} vs {b}");
        // MW yaw is about MW Z, which is engine Y.
        assert!((a * Vec3::X).y.abs() < 1e-5);
    }
}
