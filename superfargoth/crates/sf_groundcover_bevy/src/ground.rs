//! What the game hands the renderer for the ground, and the per-tile ground cache.
//!
//! [`GroundSource`] is the core's [`GroundSampler`] plus `Send + Sync` (the scatter runs on a task) and a
//! readiness test: a tile is scattered only once its ground is there (streamed terrain), so an unloaded tile
//! never scatters empty. [`FnGround`] wraps a closure.
//!
//! **Ground cache** (GC-1 open point: one ground sample per candidate per species dominates the cost). Species
//! with different ids jitter their candidates differently, so samples can't be shared point for point. Instead
//! the job samples the tile once on a lattice ([`LatticeGround`]) and every species reads it bilinearly, when that
//! takes fewer samples than sampling each candidate directly ([`LatticeGround::worth_it`]). Still a pure function
//! of (tile, ground, spacing): every peer and run scatters the same field. The core's density twin
//! (`Scatterer::density_at`) samples directly, so it can differ from a cached scatter by the lattice's
//! interpolation error (presentation only).

use bevy_math::Vec3;
use sf_groundcover::{GroundSample, GroundSampler, Rect, TileKey};

/// The game's ground, as the scatter task reads it.
pub trait GroundSource: GroundSampler + Send + Sync + 'static {
    /// Whether the ground under `rect` (world XZ, metres) is loaded. A tile waits until it is.
    fn ready(&self, rect: Rect) -> bool {
        let _ = rect;
        true
    }
}

/// A closure as a [`GroundSource`] (always ready).
pub struct FnGround<F>(pub F);

impl<F: Fn(f64, f64, &mut GroundSample) + Send + Sync + 'static> GroundSampler for FnGround<F> {
    fn sample(&self, x: f64, z: f64, out: &mut GroundSample) {
        (self.0)(x, z, out);
    }
}

impl<F: Fn(f64, f64, &mut GroundSample) + Send + Sync + 'static> GroundSource for FnGround<F> {}

/// One tile's ground on an `(n + 1)²` lattice, read bilinearly.
pub struct LatticeGround {
    corner: [f64; 2],
    step: f64,
    n: usize,
    height: Vec<f32>,
    normal: Vec<Vec3>,
    water: Vec<f32>,
    valid: Vec<bool>,
    /// `channels[point * width + c]`.
    channels: Vec<f32>,
    width: usize,
}

impl LatticeGround {
    /// Cells per axis for `spacing_m` over a tile (at least 1).
    pub fn cells(tile_size_m: f32, spacing_m: f32) -> usize {
        // A small positive count.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = (tile_size_m / spacing_m).ceil().max(1.0) as usize;
        n
    }

    /// Whether a lattice of `spacing_m` takes fewer ground samples than `direct` candidate samples.
    pub fn worth_it(tile_size_m: f32, spacing_m: f32, direct: u64) -> bool {
        if spacing_m.is_nan() || spacing_m <= 0.0 {
            return false;
        }
        let n = Self::cells(tile_size_m, spacing_m) as u64 + 1;
        n * n < direct
    }

    /// Samples `ground` over `tile` every `spacing_m` (snapped to divide the tile).
    pub fn build<G: GroundSampler + ?Sized>(ground: &G, tile: TileKey, tile_size_m: f32, spacing_m: f32) -> Self {
        let n = Self::cells(tile_size_m, spacing_m);
        let corner = tile.min_corner(tile_size_m);
        // A small count.
        #[allow(clippy::cast_precision_loss)]
        let step = f64::from(tile_size_m) / n as f64;
        let points = (n + 1) * (n + 1);
        let mut out = Self {
            corner,
            step,
            n,
            height: Vec::with_capacity(points),
            normal: Vec::with_capacity(points),
            water: Vec::with_capacity(points),
            valid: Vec::with_capacity(points),
            channels: Vec::new(),
            width: 0,
        };
        let mut g = GroundSample::blank();
        for j in 0..=n {
            for i in 0..=n {
                // Lattice indices, exact in f64.
                #[allow(clippy::cast_precision_loss)]
                let (x, z) = (corner[0] + i as f64 * step, corner[1] + j as f64 * step);
                ground.sample(x, z, &mut g);
                if out.height.is_empty() {
                    out.width = g.channels.len();
                    out.channels.reserve(points * out.width);
                }
                out.height.push(g.height);
                out.normal.push(g.normal);
                out.water.push(g.water_distance);
                out.valid.push(g.valid);
                // A sampler must report the same channel count everywhere; pad or cut if it doesn't.
                out.channels.extend((0..out.width).map(|c| g.channels.get(c).copied().unwrap_or(0.0)));
            }
        }
        out
    }
}

impl GroundSampler for LatticeGround {
    #[allow(clippy::many_single_char_names, clippy::cast_precision_loss)] // lattice maths; n is small
    fn sample(&self, x: f64, z: f64, out: &mut GroundSample) {
        let n = self.n;
        let fx = ((x - self.corner[0]) / self.step).clamp(0.0, n as f64);
        let fz = ((z - self.corner[1]) / self.step).clamp(0.0, n as f64);
        // Clamped into [0, n].
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (i, j) = ((fx.floor() as usize).min(n - 1), (fz.floor() as usize).min(n - 1));
        // Fractions in [0, 1].
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let (tx, tz) = ((fx - i as f64) as f32, (fz - j as f64) as f32);
        let idx = [j * (n + 1) + i, j * (n + 1) + i + 1, (j + 1) * (n + 1) + i, (j + 1) * (n + 1) + i + 1];
        let w = [(1.0 - tx) * (1.0 - tz), tx * (1.0 - tz), (1.0 - tx) * tz, tx * tz];
        let lerp = |v: &[f32]| idx.iter().zip(w).map(|(&k, w)| v[k] * w).sum::<f32>();
        out.height = lerp(&self.height);
        out.water_distance = lerp(&self.water);
        out.normal = idx.iter().zip(w).map(|(&k, w)| self.normal[k] * w).sum::<Vec3>().normalize_or(Vec3::Y);
        out.valid = idx.iter().all(|&k| self.valid[k]);
        out.channels.clear();
        for c in 0..self.width {
            out.channels.push(idx.iter().zip(w).map(|(&k, w)| self.channels[k * self.width + c] * w).sum());
        }
    }
}
