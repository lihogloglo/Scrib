//! Small math shared by the crate: [`Trs`] and quaternion helpers.

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

/// A local joint transform: translation, rotation, scale (applied scale
/// first, then rotation, then translation).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trs {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Trs {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Trs {
    pub const IDENTITY: Self = Self { translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE };

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { translation, rotation, scale }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self { translation, ..Self::IDENTITY }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self { rotation, ..Self::IDENTITY }
    }

    pub fn to_affine(self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Decomposes an affine transform (shear is lost).
    pub fn from_affine(m: &Affine3A) -> Self {
        let (scale, rotation, translation) = m.to_scale_rotation_translation();
        Self { translation, rotation: rotation.normalize(), scale }
    }

    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        self.translation + self.rotation * (self.scale * p)
    }
}

/// `q` or `-q`, whichever is in the same hemisphere as `reference`.
#[inline]
pub fn align_hemisphere(q: Quat, reference: Quat) -> Quat {
    if q.dot(reference) < 0.0 { -q } else { q }
}

/// Normalized linear interpolation along the shorter arc.
#[inline]
pub fn nlerp(a: Quat, b: Quat, t: f32) -> Quat {
    let b = align_hemisphere(b, a);
    (a * (1.0 - t) + b * t).normalize()
}

/// The rotation angle in radians between two rotations, in `[0, π]`.
///
/// Computed as `2·atan2(|v|, |w|)` of the relative rotation, not
/// `2·acos(|a·b|)`: in f32 the dot product of rotations less than about
/// 3e-4 rad apart rounds to 1, so the acos form reads them as identical,
/// and the cook's quantizer measures its error with this.
#[inline]
pub fn angle_between(a: Quat, b: Quat) -> f32 {
    let d = a.conjugate() * b;
    2.0 * Vec3::new(d.x, d.y, d.z).length().atan2(d.w.abs())
}

/// Angle of the twist of `q` about the unit `axis` (swing-twist
/// decomposition), in `(-π, π]`.
pub fn twist_angle(q: Quat, axis: Vec3) -> f32 {
    let v = Vec3::new(q.x, q.y, q.z);
    let p = axis * v.dot(axis);
    let twist = Quat::from_xyzw(p.x, p.y, p.z, q.w);
    let len = twist.length();
    if len < 1e-9 {
        return 0.0;
    }
    let twist = twist / len;
    let angle = 2.0 * twist.w.clamp(-1.0, 1.0).acos();
    let signed = if Vec3::new(twist.x, twist.y, twist.z).dot(axis) < 0.0 { -angle } else { angle };
    wrap_angle(signed)
}

/// Wraps an angle to `(-π, π]`.
pub fn wrap_angle(a: f32) -> f32 {
    use std::f32::consts::{PI, TAU};
    let w = (a + PI).rem_euclid(TAU) - PI;
    if w <= -PI { w + TAU } else { w }
}

/// Rotation of `m`'s linear part with scale removed (assumes no shear).
#[inline]
pub fn affine_rotation(m: &Affine3A) -> Quat {
    let x = m.matrix3.x_axis.normalize_or_zero();
    let y = m.matrix3.y_axis.normalize_or_zero();
    let z = m.matrix3.z_axis.normalize_or_zero();
    Quat::from_mat3a(&glam::Mat3A::from_cols(x, y, z)).normalize()
}

/// A unit vector perpendicular to `v` (any).
pub fn any_perpendicular(v: Vec3) -> Vec3 {
    let other = if v.x.abs() < 0.9 { Vec3::X } else { Vec3::Y };
    v.cross(other).normalize_or(Vec3::Z)
}

/// The rotation taking orthonormal frame (`a_dir`, `a_up`) to (`b_dir`,
/// `b_up`). `*_up` need only be non-parallel to `*_dir`.
pub fn rotation_between_frames(a_dir: Vec3, a_up: Vec3, b_dir: Vec3, b_up: Vec3) -> Quat {
    let frame = |dir: Vec3, up: Vec3| {
        let x = dir.normalize();
        let z = x.cross(up).normalize_or(any_perpendicular(x));
        let y = z.cross(x);
        glam::Mat3::from_cols(x, y, z)
    };
    let a = frame(a_dir, a_up);
    let b = frame(b_dir, b_up);
    Quat::from_mat3(&(b * a.transpose())).normalize()
}
