//! Octahedral view mapping for impostors (Brucks, *Octahedral Impostors*,
//! 2018): a sphere (or the upper hemisphere) of view directions folded onto
//! a square, sampled by an N×N grid of frames whose corners lie on the
//! square's edges (so horizon views are captured exactly).
//!
//! Directions point from the object toward the viewer. Engine space is
//! Y up. The runtime shader (`sf_lod_bevy`, `impostor.wesl`) mirrors these
//! functions exactly; `tests` pin both against each other's constants.

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// Which directions an atlas covers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OctaMode {
    /// Upper hemisphere (ground foliage seen from walking height or above).
    #[default]
    Hemi,
    /// The whole sphere (flight, objects seen from below).
    Full,
}

/// Unit direction → octahedral square `[0, 1]²`.
pub fn encode(mode: OctaMode, d: Vec3) -> Vec2 {
    let d = d.normalize_or(Vec3::Y);
    let p = d / (d.x.abs() + d.y.abs() + d.z.abs());
    let e = match mode {
        OctaMode::Hemi => {
            // Lower directions clamp to the horizon.
            let p = Vec3::new(p.x, p.y.max(0.0), p.z);
            let p = p / (p.x.abs() + p.y + p.z.abs()).max(f32::MIN_POSITIVE);
            Vec2::new(p.x + p.z, p.x - p.z)
        }
        OctaMode::Full => {
            let xz = Vec2::new(p.x, p.z);
            if p.y >= 0.0 { xz } else { (Vec2::ONE - Vec2::new(xz.y.abs(), xz.x.abs())) * sign2(xz) }
        }
    };
    (e * 0.5 + 0.5).clamp(Vec2::ZERO, Vec2::ONE)
}

/// Octahedral square `[0, 1]²` → unit direction.
pub fn decode(mode: OctaMode, uv: Vec2) -> Vec3 {
    let e = uv * 2.0 - 1.0;
    let d = match mode {
        OctaMode::Hemi => {
            let x = f32::midpoint(e.x, e.y);
            let z = f32::midpoint(e.x, -e.y);
            Vec3::new(x, 1.0 - x.abs() - z.abs(), z)
        }
        OctaMode::Full => {
            let y = 1.0 - e.x.abs() - e.y.abs();
            let t = (-y).max(0.0);
            let xz = e - sign2(e) * t;
            Vec3::new(xz.x, y, xz.y)
        }
    };
    d.normalize()
}

fn sign2(v: Vec2) -> Vec2 {
    Vec2::new(if v.x >= 0.0 { 1.0 } else { -1.0 }, if v.y >= 0.0 { 1.0 } else { -1.0 })
}

/// The view direction of frame `(i, j)` in an `n`×`n` grid.
pub fn frame_dir(mode: OctaMode, n: u32, i: u32, j: u32) -> Vec3 {
    #[allow(clippy::cast_precision_loss)]
    let s = (n.max(2) - 1) as f32;
    #[allow(clippy::cast_precision_loss)]
    decode(mode, Vec2::new(i as f32 / s, j as f32 / s))
}

/// The orthographic basis a frame is captured with: `(right, up, toward
/// viewer)`. Right-handed, `up` as close to +Y as the direction allows.
pub fn frame_basis(dir: Vec3) -> (Vec3, Vec3, Vec3) {
    let f = dir.normalize();
    let world_up = if f.y.abs() > 0.999 { Vec3::NEG_Z } else { Vec3::Y };
    let right = world_up.cross(f).normalize();
    let up = f.cross(right);
    (right, up, f)
}

/// The three frames to blend for a view direction, with weights summing
/// to 1: the corners of the grid triangle containing the direction.
#[allow(clippy::many_single_char_names)]
pub fn blend_frames(mode: OctaMode, n: u32, d: Vec3) -> [([u32; 2], f32); 3] {
    #[allow(clippy::cast_precision_loss)]
    let s = (n.max(2) - 1) as f32;
    let g = encode(mode, d) * s;
    let base = g.floor().min(Vec2::splat(s - 1.0)).max(Vec2::ZERO);
    let f = g - base;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (i, j) = (base.x as u32, base.y as u32);
    // Split each grid cell along its (0,0)-(1,1) diagonal.
    if f.x >= f.y {
        [([i, j], 1.0 - f.x), ([i + 1, j], f.x - f.y), ([i + 1, j + 1], f.y)]
    } else {
        [([i, j], 1.0 - f.y), ([i, j + 1], f.y - f.x), ([i + 1, j + 1], f.x)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dirs() -> Vec<Vec3> {
        let mut out = Vec::new();
        for a in 0..24 {
            for b in 0..13 {
                #[allow(clippy::cast_precision_loss)]
                let (yaw, pitch) = (a as f32 * 0.2617, -1.5 + b as f32 * 0.25);
                out.push(Vec3::new(pitch.cos() * yaw.cos(), pitch.sin(), pitch.cos() * yaw.sin()));
            }
        }
        out
    }

    #[test]
    fn full_round_trips() {
        for d in dirs() {
            let back = decode(OctaMode::Full, encode(OctaMode::Full, d));
            assert!(back.distance(d.normalize()) < 1e-4, "{d} -> {back}");
        }
    }

    #[test]
    fn hemi_round_trips_upper_and_clamps_lower() {
        for d in dirs() {
            let back = decode(OctaMode::Hemi, encode(OctaMode::Hemi, d));
            if d.y >= 0.0 {
                assert!(back.distance(d.normalize()) < 1e-4, "{d} -> {back}");
            } else {
                assert!(back.y.abs() < 1e-4, "below the horizon clamps to it: {d} -> {back}");
            }
        }
    }

    #[test]
    fn frames_blend_to_one_and_hit_exact_frames() {
        for mode in [OctaMode::Hemi, OctaMode::Full] {
            for d in dirs() {
                let w = blend_frames(mode, 8, d);
                let sum: f32 = w.iter().map(|(_, w)| w).sum();
                assert!((sum - 1.0).abs() < 1e-5);
                assert!(w.iter().all(|(f, w)| *w >= -1e-6 && f[0] < 8 && f[1] < 8));
            }
            for i in 0..8 {
                for j in 0..8 {
                    let d = frame_dir(mode, 8, i, j);
                    let w = blend_frames(mode, 8, d);
                    let best = w.iter().max_by(|a, b| a.1.total_cmp(&b.1)).expect("three");
                    // The full octahedron's border frames come in mirrored
                    // pairs (same direction), so compare directions.
                    let got = frame_dir(mode, 8, best.0[0], best.0[1]);
                    assert!(got.distance(d) < 1e-4, "{mode:?} frame {i},{j}: picked {:?}", best.0);
                    assert!(best.1 > 0.999);
                }
            }
        }
    }

    #[test]
    fn bases_are_orthonormal() {
        for d in dirs().into_iter().chain([Vec3::Y, Vec3::NEG_Y]) {
            let (r, u, f) = frame_basis(d);
            assert!((r.length() - 1.0).abs() < 1e-5 && (u.length() - 1.0).abs() < 1e-5);
            assert!(r.dot(u).abs() < 1e-5 && r.dot(f).abs() < 1e-5 && u.dot(f).abs() < 1e-5);
            assert!((r.cross(u) - f).length() < 1e-4, "right-handed");
        }
    }
}
