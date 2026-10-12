//! Morrowind space to engine space.
//!
//! Morrowind: right-handed, Z up, X east, Y north, in game units.
//! Engine (Bevy): right-handed, Y up, -Z forward, in meters.
//!
//! `(x, y, z)_mw -> (x, z, -y)_engine * UNIT_METERS`. The basis change is a
//! proper rotation (determinant +1), so triangle winding is preserved.

use tes3::nif::glam::{Affine3A, Mat3, Quat, Vec3};

/// Meters per Morrowind unit. Bethesda's convention is 64 units per yard
/// (0.9144 m), about 1.43 cm per unit. A cell is 8192 units, about 117 m.
pub const UNIT_METERS: f32 = 0.9144 / 64.0;

/// The basis change as a matrix (columns are the images of MW X, Y, Z).
const BASIS: Mat3 = Mat3::from_cols(Vec3::X, Vec3::NEG_Z, Vec3::Y);

/// A Morrowind-space position (game units) in engine space (meters).
pub fn point(p: Vec3) -> Vec3 {
    BASIS * p * UNIT_METERS
}

/// A Morrowind-space direction (normals, axes) in engine space.
pub fn direction(d: Vec3) -> Vec3 {
    BASIS * d
}

/// A Morrowind-space affine transform in engine space.
///
/// The linear part is conjugated by the basis change (`B M Bᵀ`); the
/// translation is rotated and scaled to meters. Uniform scale inside the
/// transform is preserved.
pub fn affine(a: Affine3A) -> Affine3A {
    let m = Mat3::from(a.matrix3);
    let linear = BASIS * m * BASIS.transpose();
    Affine3A::from_mat3_translation(linear, point(Vec3::from(a.translation)))
}

/// A Morrowind-space rotation in engine space (conjugated by the basis).
pub fn rotation(q: Quat) -> Quat {
    Quat::from_mat3(&(BASIS * Mat3::from_quat(q) * BASIS.transpose())).normalize()
}

/// Engine-space TRS, as plain arrays so callers need not share our glam.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trs {
    pub translation: [f32; 3],
    /// Quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
    /// Uniform scale. NIF transforms only carry uniform scale.
    pub scale: f32,
}

impl Default for Trs {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Trs {
    pub const IDENTITY: Self = Self { translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 };

    /// Decomposes an engine-space affine transform.
    pub fn from_affine(a: Affine3A) -> Self {
        let (scale, rotation, translation) = a.to_scale_rotation_translation();
        // NIF scale is uniform; average absorbs float noise.
        let scale = (scale.x + scale.y + scale.z) / 3.0;
        Self { translation: translation.to_array(), rotation: rotation.normalize().to_array(), scale }
    }

    pub fn to_affine(self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(
            Vec3::splat(self.scale),
            Quat::from_array(self.rotation),
            Vec3::from_array(self.translation),
        )
    }
}

/// A placed reference's rotation (Morrowind's per-axis angles, radians) as
/// an engine-space quaternion `[x, y, z, w]`.
///
/// Morrowind applies the angles as rotations about X, then Y, then Z, with
/// clockwise-positive angles. Conjugating into engine space maps those
/// axes onto engine X, -Z, Y. TO VERIFY visually with real data (M1); a
/// wrong convention shows up immediately as mis-rotated furniture.
pub fn reference_rotation(angles: [f32; 3]) -> [f32; 4] {
    let [x, y, z] = angles;
    let mw = Quat::from_rotation_z(-z) * Quat::from_rotation_y(-y) * Quat::from_rotation_x(-x);
    let engine = Mat3::from_quat(mw);
    Quat::from_mat3(&(BASIS * engine * BASIS.transpose())).normalize().to_array()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-5
    }

    #[test]
    fn up_is_up_and_north_is_forward() {
        assert!(close(direction(Vec3::Z), Vec3::Y));
        // North (MW +Y) is engine forward (-Z).
        assert!(close(direction(Vec3::Y), Vec3::NEG_Z));
        assert!(close(direction(Vec3::X), Vec3::X));
        assert!((BASIS.determinant() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn a_cell_is_about_117_meters() {
        let p = point(Vec3::new(8192.0, 0.0, 0.0));
        assert!((p.x - 117.04).abs() < 0.01);
    }

    #[test]
    fn affine_conversion_commutes_with_points() {
        let mw = Affine3A::from_scale_rotation_translation(
            Vec3::splat(2.0),
            Quat::from_rotation_z(0.7) * Quat::from_rotation_x(0.3),
            Vec3::new(100.0, -50.0, 25.0),
        );
        let p = Vec3::new(3.0, 4.0, 5.0);
        // Transform-then-convert equals convert-then-transform.
        assert!(close(point(mw.transform_point3(p)), affine(mw).transform_point3(point(p))));
        let trs = Trs::from_affine(affine(mw));
        assert!((trs.scale - 2.0).abs() < 1e-5);
        assert!(close(trs.to_affine().transform_point3(Vec3::ONE), affine(mw).transform_point3(Vec3::ONE)));
    }

    #[test]
    fn reference_yaw_turns_about_engine_up() {
        let q = Quat::from_array(reference_rotation([0.0, 0.0, std::f32::consts::FRAC_PI_2]));
        // Clockwise-positive yaw of 90° seen from above: north turns to east.
        assert!(close(q * Vec3::NEG_Z, Vec3::X));
    }
}
