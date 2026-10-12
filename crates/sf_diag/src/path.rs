//! Camera paths for scripted fly-throughs.
//!
//! Positions follow a centripetal Catmull-Rom spline through the keyframes
//! (Yuksel, Schaefer, Keyser 2011: no cusps or self-intersections even with
//! uneven keyframe spacing), evaluated with the Barry-Goldman pyramid, and
//! re-parameterized by arc length so a camera moves at constant speed.
//! Coordinates are f64 so a path can span a whole world; converting to
//! engine space is the caller's job. Up is +Y.

use glam::{DMat3, DQuat, DVec3};
use serde::{Deserialize, Serialize};

use crate::DiagError;

/// Arc-length table resolution: samples per spline segment.
const SAMPLES_PER_SEGMENT: usize = 256;

/// Consecutive keyframes closer than this are rejected.
const MIN_KEY_SPACING: f64 = 1e-6;

/// A point the path passes through.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub position: DVec3,
    /// Camera orientation here (camera looks down its local -Z). When this
    /// and the next keyframe both have one, orientation slerps between
    /// them; otherwise the camera looks along the path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<DQuat>,
}

impl Keyframe {
    pub fn at(position: DVec3) -> Self {
        Self {
            position,
            rotation: None,
        }
    }
}

/// A camera position and orientation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub position: DVec3,
    pub rotation: DQuat,
}

/// A constant-speed spline through keyframes.
#[derive(Clone, Debug)]
pub struct CameraPath {
    keys: Vec<Keyframe>,
    /// Control points with a mirrored phantom point at each end.
    points: Vec<DVec3>,
    /// (arc length, spline parameter `segment + local t`), ascending.
    table: Vec<(f64, f64)>,
}

impl CameraPath {
    /// Builds a path through at least two keyframes, with no two
    /// consecutive keyframes at the same position.
    pub fn new(keys: Vec<Keyframe>) -> Result<Self, DiagError> {
        if keys.len() < 2 {
            return Err(DiagError::Path("needs at least two keyframes".into()));
        }
        for (i, pair) in keys.windows(2).enumerate() {
            if pair[0].position.distance(pair[1].position) < MIN_KEY_SPACING {
                return Err(DiagError::Path(format!(
                    "keyframes {i} and {} are at the same position",
                    i + 1
                )));
            }
            if !pair[0].position.is_finite() {
                return Err(DiagError::Path(format!("keyframe {i} is not finite")));
            }
        }
        if !keys[keys.len() - 1].position.is_finite() {
            return Err(DiagError::Path("last keyframe is not finite".into()));
        }
        let n = keys.len();
        let mut points = Vec::with_capacity(n + 2);
        points.push(2.0 * keys[0].position - keys[1].position);
        points.extend(keys.iter().map(|k| k.position));
        points.push(2.0 * keys[n - 1].position - keys[n - 2].position);

        let mut path = Self {
            keys,
            points,
            table: Vec::new(),
        };
        path.build_table();
        Ok(path)
    }

    pub fn keyframes(&self) -> &[Keyframe] {
        &self.keys
    }

    /// Total arc length.
    pub fn length(&self) -> f64 {
        self.table.last().map_or(0.0, |e| e.0)
    }

    /// Seconds to travel the path at `speed` (units per second).
    pub fn duration(&self, speed: f64) -> f64 {
        self.length() / speed
    }

    /// Pose after traveling `distance` along the path (clamped to the ends).
    pub fn pose_at_distance(&self, distance: f64) -> Pose {
        let u = self.param_at_distance(distance);
        Pose {
            position: self.position_at_param(u),
            rotation: self.rotation_at_param(u),
        }
    }

    /// Pose after `seconds` at `speed`.
    pub fn pose_at_time(&self, seconds: f64, speed: f64) -> Pose {
        self.pose_at_distance(seconds * speed)
    }

    fn segments(&self) -> usize {
        self.keys.len() - 1
    }

    fn build_table(&mut self) {
        let segments = self.segments();
        self.table.reserve(segments * SAMPLES_PER_SEGMENT + 1);
        self.table.push((0.0, 0.0));
        let mut length = 0.0;
        let mut prev = self.position_at_param(0.0);
        for seg in 0..segments {
            for step in 1..=SAMPLES_PER_SEGMENT {
                #[allow(clippy::cast_precision_loss)]
                let u = seg as f64 + step as f64 / SAMPLES_PER_SEGMENT as f64;
                let p = self.position_at_param(u);
                length += p.distance(prev);
                prev = p;
                self.table.push((length, u));
            }
        }
    }

    fn param_at_distance(&self, distance: f64) -> f64 {
        let s = distance.clamp(0.0, self.length());
        let i = self.table.partition_point(|e| e.0 < s);
        if i == 0 {
            return 0.0;
        }
        let (s1, u1) = self.table[i.min(self.table.len() - 1)];
        let (s0, u0) = self.table[i - 1];
        if s1 <= s0 {
            return u1;
        }
        u0 + (u1 - u0) * (s - s0) / (s1 - s0)
    }

    /// Splits a global parameter into (segment, local t in 0..=1).
    fn split(&self, u: f64) -> (usize, f64) {
        let max = self.segments();
        #[allow(clippy::cast_precision_loss)]
        let u = u.clamp(0.0, max as f64);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // u >= 0
        let seg = (u.floor() as usize).min(max - 1);
        #[allow(clippy::cast_precision_loss)]
        let t = u - seg as f64;
        (seg, t)
    }

    fn position_at_param(&self, u: f64) -> DVec3 {
        let (seg, t) = self.split(u);
        let p = &self.points[seg..seg + 4];
        centripetal_catmull_rom(p[0], p[1], p[2], p[3], t)
    }

    fn rotation_at_param(&self, u: f64) -> DQuat {
        let (seg, t) = self.split(u);
        if let (Some(a), Some(b)) = (self.keys[seg].rotation, self.keys[seg + 1].rotation) {
            return a.slerp(b, t).normalize();
        }
        // Central difference over a small parameter step.
        let du = 1e-3;
        #[allow(clippy::cast_precision_loss)]
        let max = self.segments() as f64;
        let a = self.position_at_param((u - du).max(0.0));
        let b = self.position_at_param((u + du).min(max));
        looking_to(b - a)
    }
}

/// Orientation whose -Z axis points along `direction`, with +Y up.
fn looking_to(direction: DVec3) -> DQuat {
    let Some(forward) = direction.try_normalize() else {
        return DQuat::IDENTITY;
    };
    let back = -forward;
    let up = if back.cross(DVec3::Y).length_squared() < 1e-12 {
        DVec3::Z
    } else {
        DVec3::Y
    };
    let right = up.cross(back).normalize();
    let up = back.cross(right);
    DQuat::from_mat3(&DMat3::from_cols(right, up, back)).normalize()
}

/// Point on the centripetal (alpha = 0.5) Catmull-Rom segment from `p1` to
/// `p2`, at local `t` in 0..=1 (Barry-Goldman pyramidal evaluation).
fn centripetal_catmull_rom(p0: DVec3, p1: DVec3, p2: DVec3, p3: DVec3, t: f64) -> DVec3 {
    let knot = |a: DVec3, b: DVec3| a.distance(b).sqrt().max(1e-9);
    let t0 = 0.0;
    let t1 = t0 + knot(p0, p1);
    let t2 = t1 + knot(p1, p2);
    let t3 = t2 + knot(p2, p3);
    let t = t1 + (t2 - t1) * t;
    let lerp = |a: DVec3, b: DVec3, ta: f64, tb: f64| {
        a * ((tb - t) / (tb - ta)) + b * ((t - ta) / (tb - ta))
    };
    let a1 = lerp(p0, p1, t0, t1);
    let a2 = lerp(p1, p2, t1, t2);
    let a3 = lerp(p2, p3, t2, t3);
    let b1 = lerp(a1, a2, t0, t2);
    let b2 = lerp(a2, a3, t1, t3);
    lerp(b1, b2, t1, t2)
}
