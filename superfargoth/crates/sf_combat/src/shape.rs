//! Hit and hurt volumes (combat.md "Hit and hurt volumes"): the weapon is a segment and radius in
//! socket space, carried by the baked socket track; the hurt shape is a capsule from the mover's
//! capsule (radius x0.8) plus a head sphere. Sweeps test the weapon at [`SWEEP_STEPS`] sub-steps
//! between two ticks, segment against capsule axis.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Sub-steps per tick a weapon sweep tests (ends included).
pub const SWEEP_STEPS: usize = 5;

/// The weapon's blade: a segment `a`-`b` in socket space, m, swept with `radius`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponShape {
    pub a: Vec3,
    pub b: Vec3,
    pub radius: f32,
}

/// A character's hurt volume, in its root frame (feet at the origin, +Y up).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HurtShape {
    /// Body capsule axis, bottom and top (sphere centres).
    pub bottom: Vec3,
    pub top: Vec3,
    pub radius: f32,
    pub head: Vec3,
    pub head_radius: f32,
}

impl HurtShape {
    /// Body radius as a share of the mover capsule's (hit and hurt sizes are tuned as a pair).
    pub const RADIUS_SCALE: f32 = 0.8;
    pub const HEAD_RADIUS: f32 = 0.12;
    /// Head centre height as a share of the character height.
    pub const HEAD_HEIGHT: f32 = 0.93;

    /// From the mover capsule (`radius`, total `height`, feet at 0).
    pub fn from_capsule(radius: f32, height: f32) -> Self {
        let r = radius * Self::RADIUS_SCALE;
        Self {
            bottom: Vec3::new(0.0, r, 0.0),
            top: Vec3::new(0.0, (height - r).max(r), 0.0),
            radius: r,
            head: Vec3::new(0.0, height * Self::HEAD_HEIGHT, 0.0),
            head_radius: Self::HEAD_RADIUS,
        }
    }

    /// Where the blade `p`-`q` (world, swept by `blade_radius`) first touches this shape placed at
    /// `pos` facing `yaw`: the contact point, or `None`.
    pub fn touch(&self, pos: Vec3, yaw: f32, p: Vec3, q: Vec3, blade_radius: f32) -> Option<Vec3> {
        let rot = Quat::from_rotation_y(yaw);
        let (a, b) = (pos + rot * self.bottom, pos + rot * self.top);
        let (on_blade, on_body) = closest_points(p, q, a, b);
        if on_blade.distance(on_body) <= blade_radius + self.radius {
            return Some(on_body + (on_blade - on_body).normalize_or_zero() * self.radius);
        }
        let h = pos + rot * self.head;
        let on_blade = closest_on_segment(p, q, h);
        (on_blade.distance(h) <= blade_radius + self.head_radius).then(|| h + (on_blade - h).normalize_or_zero() * self.head_radius)
    }
}

/// The point of segment `a`-`b` closest to `p`.
pub fn closest_on_segment(a: Vec3, b: Vec3, p: Vec3) -> Vec3 {
    let d = b - a;
    let len2 = d.length_squared();
    if len2 <= f32::EPSILON {
        return a;
    }
    a + d * ((p - a).dot(d) / len2).clamp(0.0, 1.0)
}

/// Closest points between segments `p1`-`q1` and `p2`-`q2` (Ericson, Real-Time Collision
/// Detection 5.1.9), returned as (on the first, on the second).
pub fn closest_points(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> (Vec3, Vec3) {
    let (d1, d2, r) = (q1 - p1, q2 - p2, p1 - p2);
    let (a, e, f) = (d1.length_squared(), d2.length_squared(), d2.dot(r));
    let eps = f32::EPSILON;
    if a <= eps && e <= eps {
        return (p1, p2);
    }
    let (s, t) = if a <= eps {
        (0.0, (f / e).clamp(0.0, 1.0))
    } else {
        let c = d1.dot(r);
        if e <= eps {
            ((-c / a).clamp(0.0, 1.0), 0.0)
        } else {
            let b = d1.dot(d2);
            let denom = a * e - b * b;
            let mut s = if denom > eps { ((b * f - c * e) / denom).clamp(0.0, 1.0) } else { 0.0 };
            let mut t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            }
            (s, t)
        }
    };
    (p1 + d1 * s, p2 + d2 * t)
}

/// Segment-to-segment distance.
pub fn segment_distance(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> f32 {
    let (a, b) = closest_points(p1, q1, p2, q2);
    a.distance(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_distances() {
        let d = segment_distance(Vec3::new(-1.0, 1.0, 0.5), Vec3::new(1.0, 1.0, 0.5), Vec3::ZERO, Vec3::Y * 2.0);
        assert!((d - 0.5).abs() < 1e-6);
        // Parallel, offset.
        assert!((segment_distance(Vec3::ZERO, Vec3::Y, Vec3::X, Vec3::X + Vec3::Y) - 1.0).abs() < 1e-6);
        // Past the end.
        assert!((segment_distance(Vec3::new(0.0, 3.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Vec3::ZERO, Vec3::Y) - 2.0).abs() < 1e-6);
    }

    #[test]
    fn hurt_capsule_and_head() {
        let h = HurtShape::from_capsule(0.3, 1.8);
        assert!((h.radius - 0.24).abs() < 1e-6 && (h.head.y - 1.674).abs() < 1e-4);
        let blade = |z: f32| (Vec3::new(-1.0, 1.0, z), Vec3::new(1.0, 1.0, z));
        let (p, q) = blade(0.3);
        assert!(h.touch(Vec3::ZERO, 0.0, p, q, 0.08).is_some());
        let (p, q) = blade(0.33);
        assert!(h.touch(Vec3::ZERO, 0.0, p, q, 0.08).is_none());
        // Head height.
        let (p, q) = (Vec3::new(-1.0, 1.7, 0.15), Vec3::new(1.0, 1.7, 0.15));
        assert!(h.touch(Vec3::ZERO, 0.0, p, q, 0.08).is_some());
    }
}
