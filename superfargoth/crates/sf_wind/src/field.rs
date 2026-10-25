//! CPU twin of `shaders/sf_wind.wesl`: the same operations in the same
//! order, so the two agree to float rounding (`tests/wgsl_twin.rs`, 1e-4).
//!
//! Gust field after the owner's Megalophobia `grass_wind.wgsl` (MIT OR
//! Apache-2.0, ARCHITECTURE §10 2026-10-12); see the shader header for what
//! changed.

use glam::{Vec2, Vec3};

use crate::frame::{WindParams, WindSourceGpu};

/// Below this distance (m) a radial or vortex source scales down linearly.
pub const SOURCE_CORE: f32 = 0.5;
pub(crate) const OCTAVE1_SEED: u32 = 0x9e37_79b9;

/// The gust field at one point (WGSL `WindGust`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GustSample {
    /// Local flow direction (unit, xz), at most `asin(swirl)` off the
    /// prevailing axis.
    pub dir: Vec2,
    /// The divergence-free swirl before it is folded into `dir` (length ≤ 1).
    pub curl: Vec2,
    /// Gust envelope: 0 in a lull, 1 under a gust front.
    pub gust: f32,
}

/// PCG hash (Jarzynski and Olano, "Hash Functions for GPU Rendering", 2020).
fn pcg(v: u32) -> u32 {
    let state = v.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277_803_737);
    (word >> 22) ^ word
}

#[expect(clippy::cast_sign_loss, reason = "bitcast, as in the WGSL")]
fn lattice(ix: i32, iy: i32, seed: u32) -> f32 {
    let h = pcg((ix as u32).wrapping_add(pcg((iy as u32).wrapping_add(seed))));
    #[expect(clippy::cast_precision_loss, reason = "24 bits: exact")]
    let v = (h >> 8) as f32;
    v * (1.0 / 16_777_216.0)
}

fn wrap(i: i32, n: i32) -> i32 {
    ((i % n) + n) % n
}

/// Value noise with its analytic gradient `(value, d/dx, d/dy)` on a lattice
/// repeating every `n` cells.
#[expect(clippy::cast_possible_truncation, reason = "floor()ed lattice cell, as WGSL i32()")]
#[expect(clippy::many_single_char_names, reason = "mirrors the WGSL")]
fn noise_d(p: Vec2, base: (i32, i32), n: i32, seed: u32) -> Vec3 {
    let cell = p.floor();
    let f = p - cell;
    let u = f * f * (3.0 - 2.0 * f);
    let du = 6.0 * f * (1.0 - f);
    let ix = cell.x as i32 + base.0;
    let iy = cell.y as i32 + base.1;
    let (x0, x1) = (wrap(ix, n), wrap(ix + 1, n));
    let (y0, y1) = (wrap(iy, n), wrap(iy + 1, n));
    let a = lattice(x0, y0, seed);
    let b = lattice(x1, y0, seed);
    let c = lattice(x0, y1, seed);
    let d = lattice(x1, y1, seed);
    let k1 = b - a;
    let k2 = c - a;
    let k3 = a - b - c + d;
    Vec3::new(
        a + k1 * u.x + k2 * u.y + k3 * u.x * u.y,
        du.x * (k1 + k3 * u.y),
        du.y * (k2 + k3 * u.x),
    )
}

/// WGSL `smoothstep`.
pub(crate) fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[expect(clippy::cast_possible_truncation, reason = "integer cell count stored as f32")]
pub(crate) fn gust(w: &WindParams, rel_xz: Vec2) -> GustSample {
    let axis = w.axis_speed.truncate().truncate();
    let p0 = rel_xz * w.octave0.z + Vec2::new(w.octave0.x, w.octave0.y);
    let p1 = rel_xz * w.octave1.z + Vec2::new(w.octave1.x, w.octave1.y);
    let n0 = noise_d(p0, (w.cells.x as i32, w.cells.y as i32), w.octave0.w as i32, 0);
    let n1 = noise_d(p1, (w.cells.z as i32, w.cells.w as i32), w.octave1.w as i32, OCTAVE1_SEED);
    let raw = Vec2::new(n0.z, -n0.y) + Vec2::new(n1.z, -n1.y) * 0.5;
    let curl = raw / raw.length().max(1.0);
    let turned = axis + curl * w.shape.x;
    let dir = turned / turned.length().max(1e-4);
    let potential = n0.x * 0.7 + n1.x * 0.3;
    let gust = smoothstep(w.shape.y, w.shape.y + w.shape.z, potential);
    GustSample { dir, curl, gust }
}

pub(crate) fn profile(w: &WindParams, above_ground: f32) -> f32 {
    let h = above_ground.max(0.0);
    ((h / w.profile.x + 1.0).ln() * w.profile.y).min(w.profile.z)
}

pub(crate) fn global(w: &WindParams, rel: Vec3, above_ground: f32) -> Vec3 {
    let speed = w.axis_speed.z;
    if speed <= 0.0 {
        return Vec3::ZERO;
    }
    let g = gust(w, Vec2::new(rel.x, rel.z));
    let s = speed * (1.0 + w.axis_speed.w * (2.0 * g.gust - 1.0)) * profile(w, above_ground);
    Vec3::new(g.dir.x * s, 0.0, g.dir.y * s)
}

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "kind tag stored as f32")]
#[expect(clippy::many_single_char_names, reason = "mirrors the WGSL")]
pub(crate) fn source(s: &WindSourceGpu, rel: Vec3) -> Vec3 {
    let r = rel - s.a.truncate();
    let d = r.length();
    let fall = 1.0 - smoothstep(s.c.x, s.c.y, d);
    let k = s.b.w * fall;
    match s.a.w as u32 {
        1 => r * (k / d.max(SOURCE_CORE)),
        2 => {
            let t = s.b.truncate().cross(r);
            t * (k / t.length().max(SOURCE_CORE))
        }
        _ => s.b.truncate() * k,
    }
}
