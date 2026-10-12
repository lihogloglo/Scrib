//! Morrowind units and axes to `sf_coords`, converted once.
//!
//! Morrowind: Z up, X east, Y north, game units (64 per yard). Engine:
//! Y up, -Z forward (north), meters. `(x, y, z) -> (x, z, -y) · UNIT_METERS`
//! (the same basis change as `mw_nif::coords`). Positions convert in f64 and
//! become a `WorldPos` directly, so no f32 world coordinate is ever formed:
//! the only rounding is the source's own f32 and the final f32 cell offset
//! (≤ 7.6 µm in a 256 m cell).

use glam::{DVec3, Quat, Vec3};
use mw_nif::coords::reference_rotation;
use sf_coords::{WorldPos, WorldTransform};

/// Meters per Morrowind unit, in f64 (`mw_nif::coords::UNIT_METERS` is f32).
pub const UNIT_METERS_F64: f64 = 0.9144 / 64.0;

/// Edge of an exterior cell in units.
pub const CELL_UNITS: f64 = 8192.0;

/// A Morrowind position (units) in engine meters.
pub fn engine_f64(p: [f32; 3]) -> DVec3 {
    let [x, y, z] = p.map(f64::from);
    DVec3::new(x, z, -y) * UNIT_METERS_F64
}

/// A Morrowind position as a `WorldPos`.
pub fn world_pos(p: [f32; 3]) -> WorldPos {
    WorldPos::from_f64(engine_f64(p))
}

/// A Morrowind length (units) in meters.
#[allow(clippy::cast_possible_truncation)]
pub fn meters(units: f32) -> f32 {
    (f64::from(units) * UNIT_METERS_F64) as f32
}

/// A reference pose (position, per-axis angles, uniform scale) as a
/// `WorldTransform`.
pub fn world_transform(translation: [f32; 3], rotation: [f32; 3], scale: f32) -> WorldTransform {
    WorldTransform {
        pos: world_pos(translation),
        rotation: Quat::from_array(reference_rotation(rotation)),
        scale: Vec3::splat(scale),
    }
}

/// Absolute engine position of exterior grid corner `(x, y)` (its
/// south-west corner in Morrowind terms), in meters.
pub fn grid_corner(x: i32, y: i32) -> DVec3 {
    DVec3::new(f64::from(x) * CELL_UNITS, 0.0, -f64::from(y) * CELL_UNITS) * UNIT_METERS_F64
}

/// The exterior grid square containing a Morrowind position.
#[allow(clippy::cast_possible_truncation)]
pub fn grid_of(p: [f32; 3]) -> (i32, i32) {
    let cell = |v: f32| (f64::from(v) / CELL_UNITS).floor() as i32;
    (cell(p[0]), cell(p[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_convert_like_mw_nif() {
        let p = [1000.5f32, -2000.25, 300.0];
        let ours = world_pos(p).to_f64();
        let nif = mw_nif::coords::point(mw_nif::glam::Vec3::from_array(p));
        assert!((ours.x - f64::from(nif.x)).abs() < 1e-4);
        assert!((ours.y - f64::from(nif.y)).abs() < 1e-4);
        assert!((ours.z - f64::from(nif.z)).abs() < 1e-4);
    }

    #[test]
    fn far_positions_stay_exact() {
        // The far corner of Morrowind's map is ~40 cells out; go further.
        let p = [8192.0 * 300.0 + 0.125, -8192.0 * 300.0 - 0.125, 12.0];
        let pos = world_pos(p);
        let back = pos.to_f64();
        let want = engine_f64(p);
        assert!((back - want).abs().max_element() < 1e-5, "{back} vs {want}");
        assert!(pos.local.abs().max_element() < 128.0);
    }

    #[test]
    fn grid_squares_floor() {
        assert_eq!(grid_of([-1.0, 8192.0, 0.0]), (-1, 1));
        assert_eq!(grid_of([8191.9, -0.1, 0.0]), (0, -1));
        let c = grid_corner(1, 1);
        assert!((c.x - 117.0432).abs() < 1e-4 && (c.z + 117.0432).abs() < 1e-4);
    }
}
