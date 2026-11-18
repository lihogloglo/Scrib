//! Deterministic **symmetry breaking** for the macro stage: a small value-noise field from an integer hash of
//! absolute world coordinates.
//!
//! Why: on a smooth intent (a bicubic-upscaled guide) neighbouring samples see the same slope, so flow routing
//! has nothing to choose between them and channels come out parallel and grid-aligned. Landscape-evolution models
//! break that symmetry with a small random perturbation of the initial surface or of the forcing: Braun & Willett
//! (2013, Geomorphology 180-181) start their experiments from a surface with small random noise; Schott, Paris,
//! Fournier, Guérin, Galin ("Large-scale terrain authoring through interactive erosion simulation", ACM TOG 42(5),
//! 2023; code `H-Schott/StreamPowerErosion`, `spe_shader.glsl`) multiply the erodibility by a noise buffer;
//! Cordonnier et al. (CGF 35(2), 2016) drive the form by an uplift map. Here the noise perturbs the **uplift
//! target** (the intent the stream stage relaxes toward) and the initial height, so it acts at every step.
//!
//! The field is value noise (lattice values from a 64-bit integer hash, smootherstep interpolation; Perlin,
//! "Improving noise", SIGGRAPH 2002, for the quintic fade) at one wavelength, in [-1, 1]. It is a pure function of
//! the world position and the seed: never of the thread count, the tile or the grid origin (worldgen.md "Stage
//! rules": noise samples absolute world coordinates).

use rayon::prelude::*;

/// splitmix64 finaliser (Steele, Lea, Flood, "Fast splittable pseudorandom number generators", OOPSLA 2014).
#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Lattice value in [-1, 1] at integer lattice point (ix, iy) for `seed`.
#[inline]
pub fn lattice(ix: i64, iy: i64, seed: u64) -> f64 {
    let h = mix64(
        mix64(seed ^ ix.cast_unsigned().wrapping_mul(0x9e37_79b9_7f4a_7c15))
            ^ iy.cast_unsigned().wrapping_mul(0xc2b2_ae3d_27d4_eb4f),
    );
    // Top 53 bits → [0, 1), then → [-1, 1).
    (h >> 11) as f64 * (1.0 / (1u64 << 53) as f64) * 2.0 - 1.0
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Value noise in [-1, 1] at world position `p` (metres) with lattice spacing `wavelength_m`.
pub fn value_noise(p: [f64; 2], wavelength_m: f64, seed: u64) -> f64 {
    let (u, v) = (p[0] / wavelength_m, p[1] / wavelength_m);
    let (fx, fy) = (u.floor(), v.floor());
    let (ix, iy) = (fx as i64, fy as i64);
    let (tx, ty) = (fade(u - fx), fade(v - fy));
    let a = lattice(ix, iy, seed);
    let b = lattice(ix + 1, iy, seed);
    let c = lattice(ix, iy + 1, seed);
    let d = lattice(ix + 1, iy + 1, seed);
    let ab = a + (b - a) * tx;
    let cd = c + (d - c) * tx;
    ab + (cd - ab) * ty
}

/// Perturbation field of a `w` × `h` grid: `amp_m × value_noise(world(x, y))`, where `world` maps a sample to
/// world metres. Rows run in parallel; each value is a pure function of its world position.
pub fn field(
    w: usize,
    h: usize,
    world: impl Fn(usize, usize) -> [f64; 2] + Sync,
    amp_m: f64,
    wavelength_m: f64,
    seed: u64,
) -> Vec<f64> {
    let mut out = vec![0.0; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, v) in row.iter_mut().enumerate() {
            *v = amp_m * value_noise(world(x, y), wavelength_m, seed);
        }
    });
    out
}
