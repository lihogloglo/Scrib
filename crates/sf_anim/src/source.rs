//! Source animation data as importers produce it: keyframe curves per joint
//! (glTF's model: step, linear, or cubic Hermite), named float curves,
//! events and markers. `anim.cook` turns a [`SourceClip`] into a cooked
//! [`crate::Clip`]; nothing here runs at game time.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::{Event, Skeleton, SourceJoint, SyncMarker, Trs, math};

/// How a curve interpolates between keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interpolation {
    Step,
    #[default]
    Linear,
    /// Cubic Hermite. `values` holds `[in_tangent, value, out_tangent]` per
    /// key (glTF `CUBICSPLINE`), tangents in units per second.
    CubicSpline,
}

/// A value a curve can hold.
pub trait CurveValue: Copy {
    fn lerp_value(a: Self, b: Self, t: f32) -> Self;
    fn hermite(p0: Self, m0: Self, p1: Self, m1: Self, t: f32, dt: f32) -> Self;
    #[must_use]
    fn finish(self) -> Self {
        self
    }
}

impl CurveValue for f32 {
    fn lerp_value(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
    fn hermite(p0: Self, m0: Self, p1: Self, m1: Self, t: f32, dt: f32) -> Self {
        let (h00, h10, h01, h11) = hermite_basis(t);
        h00 * p0 + h10 * dt * m0 + h01 * p1 + h11 * dt * m1
    }
}

impl CurveValue for Vec3 {
    fn lerp_value(a: Self, b: Self, t: f32) -> Self {
        a.lerp(b, t)
    }
    fn hermite(p0: Self, m0: Self, p1: Self, m1: Self, t: f32, dt: f32) -> Self {
        let (h00, h10, h01, h11) = hermite_basis(t);
        h00 * p0 + h10 * dt * m0 + h01 * p1 + h11 * dt * m1
    }
}

impl CurveValue for Quat {
    /// glTF: linear rotation keys interpolate by slerp.
    fn lerp_value(a: Self, b: Self, t: f32) -> Self {
        a.slerp(math::align_hemisphere(b, a), t)
    }
    fn hermite(p0: Self, m0: Self, p1: Self, m1: Self, t: f32, dt: f32) -> Self {
        let (h00, h10, h01, h11) = hermite_basis(t);
        p0 * h00 + m0 * (h10 * dt) + p1 * h01 + m1 * (h11 * dt)
    }
    fn finish(self) -> Self {
        self.normalize()
    }
}

fn hermite_basis(t: f32) -> (f32, f32, f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    (2.0 * t3 - 3.0 * t2 + 1.0, t3 - 2.0 * t2 + t, -2.0 * t3 + 3.0 * t2, t3 - t2)
}

/// A keyframed curve. Times are seconds, strictly increasing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Curve<T> {
    pub times: Vec<f32>,
    pub values: Vec<T>,
    pub interpolation: Interpolation,
}

impl<T: CurveValue> Curve<T> {
    pub fn linear(times: Vec<f32>, values: Vec<T>) -> Self {
        Self { times, values, interpolation: Interpolation::Linear }
    }

    pub fn constant(value: T) -> Self {
        Self::linear(vec![0.0], vec![value])
    }

    /// Whether the value array matches the key count for the interpolation.
    pub fn is_valid(&self) -> bool {
        let per_key = if self.interpolation == Interpolation::CubicSpline { 3 } else { 1 };
        !self.times.is_empty() && self.values.len() == self.times.len() * per_key && self.times.windows(2).all(|w| w[0] < w[1])
    }

    fn key(&self, i: usize) -> T {
        match self.interpolation {
            Interpolation::CubicSpline => self.values[i * 3 + 1],
            _ => self.values[i],
        }
    }

    /// The value at `time`, clamped to the first and last keys.
    pub fn sample(&self, time: f32) -> T {
        let n = self.times.len();
        if n == 1 || time <= self.times[0] {
            return self.key(0).finish();
        }
        if time >= self.times[n - 1] {
            return self.key(n - 1).finish();
        }
        let i = self.times.partition_point(|&k| k <= time) - 1;
        let (t0, t1) = (self.times[i], self.times[i + 1]);
        let dt = t1 - t0;
        let u = (time - t0) / dt;
        match self.interpolation {
            Interpolation::Step => self.key(i),
            Interpolation::Linear => T::lerp_value(self.key(i), self.key(i + 1), u),
            Interpolation::CubicSpline => {
                let out_tangent = self.values[i * 3 + 2];
                let in_tangent = self.values[(i + 1) * 3];
                T::hermite(self.key(i), out_tangent, self.key(i + 1), in_tangent, u, dt)
            }
        }
        .finish()
    }

    pub fn end_time(&self) -> f32 {
        self.times.last().copied().unwrap_or(0.0)
    }
}

/// Keyframes of one joint. Missing channels keep the skeleton's rest value.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SourceTrack {
    pub joint: String,
    pub translation: Option<Curve<Vec3>>,
    pub rotation: Option<Curve<Quat>>,
    pub scale: Option<Curve<Vec3>>,
}

/// A named float curve (morph weights, authored gameplay curves).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceCurve {
    pub name: String,
    pub curve: Curve<f32>,
}

/// One animation as imported, before cooking. Times are seconds from the
/// clip start.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SourceClip {
    pub name: String,
    pub duration: f32,
    pub tracks: Vec<SourceTrack>,
    pub curves: Vec<SourceCurve>,
    pub events: Vec<Event>,
    pub markers: Vec<SyncMarker>,
}

/// What an animation importer returns.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AnimSource {
    /// The skin's joints, if the file has a skeleton.
    pub skeleton: Option<Vec<SourceJoint>>,
    pub clips: Vec<SourceClip>,
    /// The root-motion joint the importer detected
    /// ([`crate::detect_root_motion_joint`]), if any.
    pub root_motion: Option<String>,
}

/// A [`SourceClip`] bound to a skeleton: track `i` drives joint
/// `joints[i]`. Unknown joint names are reported, not fatal (clips are often
/// shared by rigs with a few extra or missing helper joints).
pub struct BoundSource<'a> {
    pub clip: &'a SourceClip,
    joints: Vec<Option<usize>>,
    pub unbound: Vec<String>,
}

impl<'a> BoundSource<'a> {
    pub fn new(clip: &'a SourceClip, skeleton: &Skeleton) -> Self {
        let mut unbound = Vec::new();
        let joints = clip
            .tracks
            .iter()
            .map(|t| {
                let j = skeleton.find(&t.joint);
                if j.is_none() {
                    unbound.push(t.joint.clone());
                }
                j
            })
            .collect();
        Self { clip, joints, unbound }
    }

    /// The local pose at `time` (seconds), on top of `rest`.
    pub fn sample(&self, time: f32, rest: &[Trs], out: &mut [Trs]) {
        out.copy_from_slice(rest);
        for (track, joint) in self.clip.tracks.iter().zip(&self.joints) {
            let Some(j) = *joint else { continue };
            if let Some(c) = &track.translation {
                out[j].translation = c.sample(time);
            }
            if let Some(c) = &track.rotation {
                out[j].rotation = c.sample(time);
            }
            if let Some(c) = &track.scale {
                out[j].scale = c.sample(time);
            }
        }
    }

    /// Whether any track drives joint `j`.
    pub fn drives(&self, joint: usize) -> bool {
        self.joints.contains(&Some(joint))
    }

    /// The smallest key interval across all tracks, if every key time is a
    /// multiple of it (a uniformly keyed source), in keys per second.
    pub fn native_rate(&self) -> Option<f32> {
        let mut times: Vec<f32> = Vec::new();
        for t in &self.clip.tracks {
            if let Some(c) = &t.translation {
                times.extend(&c.times);
            }
            if let Some(c) = &t.rotation {
                times.extend(&c.times);
            }
            if let Some(c) = &t.scale {
                times.extend(&c.times);
            }
        }
        times.sort_by(f32::total_cmp);
        times.dedup_by(|a, b| (*a - *b).abs() < 1e-5);
        let step = times.windows(2).map(|w| w[1] - w[0]).fold(f32::INFINITY, f32::min);
        if !step.is_finite() || step <= 1e-4 {
            return None;
        }
        let uniform = times.iter().all(|&t| {
            let k = (t - times[0]) / step;
            (k - k.round()).abs() < 1e-2
        });
        uniform.then(|| 1.0 / step)
    }
}
