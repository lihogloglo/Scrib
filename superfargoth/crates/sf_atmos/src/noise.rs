//! Tiling noise for the clouds, generated on the CPU (deterministic per seed).
//!
//! Ported from Megalophobia (`src/clouds.rs`, `src/volumetrics.rs`; see the
//! crate docs for provenance): lattice value noise, tiling Worley, and the
//! Perlin-Worley shape volume. Every octave period divides its texture, so
//! all three textures wrap seamlessly under a repeat sampler.

#![allow(clippy::many_single_char_names)] // noise math: x, y, z, p, c, f, w

use glam::{Vec2, Vec3};
use rayon::prelude::*;

use crate::density::remap;

/// Shape volume edge (texels): R Perlin-Worley, G Worley FBM.
pub const SHAPE_SIZE: u32 = 128;
/// Detail volume edge (texels): Worley FBM, erodes edges.
pub const DETAIL_SIZE: u32 = 32;
/// Weather-noise map edge (texels): R coverage field, G type field, B storm cells.
pub const WEATHER_SIZE: u32 = 512;

/// Shape and detail lattice periods, coarse to fine.
pub const SHAPE_OCTAVES: [i64; 3] = [4, 8, 16];
pub const DETAIL_OCTAVES: [i64; 3] = [4, 8, 16];
const COVER_OCTAVES: [i64; 4] = [4, 8, 16, 32];
const TYPE_OCTAVES: [i64; 3] = [2, 4, 8];
// Storm cells (cumulonimbus) are a few km wide: 3 cells per weather tile.
const STORM_PERIOD: i64 = 3;
const CIRRUS_OCTAVES: [i64; 3] = [4, 8, 16];
const CIRRUS_WARP_OCTAVES: [i64; 2] = [2, 4];
/// Cirrus streaks run along X: the field is squeezed this many times across them.
const CIRRUS_STRETCH: f32 = 6.0;

/// The three textures, as raw texel bytes.
#[derive(Clone, Debug)]
pub struct NoiseVolumes {
    /// `SHAPE_SIZE³` × RG8 (x fastest, then y, then z).
    pub shape: Vec<u8>,
    /// `DETAIL_SIZE³` × R8.
    pub detail: Vec<u8>,
    /// `WEATHER_SIZE²` × RGBA8.
    pub weather: Vec<u8>,
}

impl NoiseVolumes {
    /// Generates all three (parallel; about a second for the 128³ shape volume).
    pub fn generate(seed: u64) -> Self {
        Self { shape: shape_volume(seed ^ 0xC10D), detail: detail_volume(seed ^ 0xDE7A), weather: weather_noise(seed ^ 0x1EA7) }
    }
}

/// `SplitMix64`, with a per-lattice-point stream (independent of call order).
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn at(seed: u64, ix: i64, iy: i64, iz: i64) -> Self {
        let mut h = seed ^ 0x9E37_79B9_7F4A_7C15;
        for v in [ix, iy, iz] {
            h = (h ^ v.cast_unsigned()).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            h ^= h >> 27;
        }
        Self(h)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    #[allow(clippy::cast_precision_loss)] // 24 bits, exact
    pub fn f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)] // small lattice indices
fn cell_coords(i: i64, period: i64) -> (i64, i64, i64) {
    (i % period, (i / period) % period, i / (period * period))
}

/// Tiling value-noise lattice with smoothstep interpolation.
pub struct Lattice {
    period: i64,
    v: Vec<f32>,
}

impl Lattice {
    pub fn new(period: i64, seed: u64) -> Self {
        let v = (0..period * period * period)
            .map(|i| {
                let (x, y, z) = cell_coords(i, period);
                Rng::at(seed, x, y, z).f32()
            })
            .collect();
        Self { period, v }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)] // rem_euclid >= 0
    fn corner(&self, x: i64, y: i64, z: i64) -> f32 {
        let p = self.period;
        let (x, y, z) = (x.rem_euclid(p), y.rem_euclid(p), z.rem_euclid(p));
        self.v[(z * p * p + y * p + x) as usize]
    }

    /// `p` runs 0..1 across the tile. Wraps.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn sample(&self, p: Vec3) -> f32 {
        let g = p * self.period as f32;
        let c = g.floor();
        let f = g - c;
        let w = f * f * (Vec3::splat(3.0) - 2.0 * f);
        let (x, y, z) = (c.x as i64, c.y as i64, c.z as i64);
        let mix = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let row = |y, z| mix(self.corner(x, y, z), self.corner(x + 1, y, z), w.x);
        let plane = |z| mix(row(y, z), row(y + 1, z), w.y);
        mix(plane(z), plane(z + 1), w.z)
    }
}

/// Coarse octaves make the banks, fine ones the edges.
pub fn fbm(octaves: &[Lattice], p: Vec3) -> f32 {
    let (mut sum, mut amp, mut norm) = (0.0, 1.0, 0.0);
    for l in octaves {
        sum += amp * l.sample(p);
        norm += amp;
        amp *= 0.5;
    }
    sum / norm
}

/// Tiling Worley (cellular) noise: one feature point per cell.
pub struct Cells {
    period: i64,
    points: Vec<Vec3>,
}

impl Cells {
    #[allow(clippy::cast_precision_loss)]
    pub fn new(period: i64, seed: u64) -> Self {
        let points = (0..period * period * period)
            .map(|i| {
                let (x, y, z) = cell_coords(i, period);
                let mut rng = Rng::at(seed, x, y, z);
                Vec3::new(x as f32, y as f32, z as f32) + Vec3::new(rng.f32(), rng.f32(), rng.f32())
            })
            .collect();
        Self { period, points }
    }

    /// The point owned by `cell`, placed at the *unwrapped* cell: stored
    /// wrapped, placed unwrapped, which is what makes the field tile.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
    fn point(&self, cell: Vec3) -> Vec3 {
        let p = self.period;
        let w = |c: f32| (c as i64).rem_euclid(p);
        let (x, y, z) = (w(cell.x), w(cell.y), w(cell.z));
        let stored = self.points[(z * p * p + y * p + x) as usize];
        cell + (stored - Vec3::new(x as f32, y as f32, z as f32))
    }

    /// Nearest feature distance in cells (clamped to 1), `p` 0..1 across the tile.
    #[allow(clippy::cast_precision_loss)]
    pub fn distance(&self, p: Vec3) -> f32 {
        let g = p * self.period as f32;
        let c = g.floor();
        let mut best = f32::MAX;
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let cell = c + Vec3::new(dx as f32, dy as f32, dz as f32);
                    best = best.min(g.distance_squared(self.point(cell)));
                }
            }
        }
        best.sqrt().min(1.0)
    }
}

/// Inverted Worley FBM: high means "inside a billow".
pub fn worley_fbm(octaves: &[Cells], p: Vec3) -> f32 {
    let (mut sum, mut amp, mut norm) = (0.0, 1.0, 0.0);
    for c in octaves {
        sum += amp * (1.0 - c.distance(p));
        norm += amp;
        amp *= 0.5;
    }
    sum / norm
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn unorm(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

#[allow(clippy::cast_precision_loss)]
fn texel_center(i: usize, size: u32) -> f32 {
    (i as f32 + 0.5) / size as f32
}

fn seeds(seed: u64, mul: u64, periods: &[i64]) -> impl Iterator<Item = (i64, u64)> + '_ {
    periods.iter().enumerate().map(move |(i, p)| (*p, seed ^ (i as u64).wrapping_mul(mul)))
}

/// R: Perlin-Worley (value FBM dilated by low-frequency Worley), the base
/// shape. G: Worley FBM, which the shader uses to dilate it again.
pub fn shape_volume(seed: u64) -> Vec<u8> {
    let perlin: Vec<Lattice> = seeds(seed, 0x9E37_79B9, &SHAPE_OCTAVES).map(|(p, s)| Lattice::new(p, s)).collect();
    let worley: Vec<Cells> = seeds(seed, 0xC2B2_AE35, &SHAPE_OCTAVES).map(|(p, s)| Cells::new(p, s)).collect();
    let n = SHAPE_SIZE as usize;
    let mut out = vec![0u8; n * n * n * 2];
    out.par_chunks_mut(n * n * 2).enumerate().for_each(|(iz, slice)| {
        for iy in 0..n {
            for ix in 0..n {
                let p = Vec3::new(texel_center(ix, SHAPE_SIZE), texel_center(iy, SHAPE_SIZE), texel_center(iz, SHAPE_SIZE));
                let pf = fbm(&perlin, p);
                let wf = worley_fbm(&worley, p);
                let pw = remap(pf, -worley[0].distance(p), 1.0, 0.0, 1.0);
                let i = (iy * n + ix) * 2;
                slice[i] = unorm(pw);
                slice[i + 1] = unorm(wf);
            }
        }
    });
    out
}

/// Worley FBM, the erosion octave.
pub fn detail_volume(seed: u64) -> Vec<u8> {
    let worley: Vec<Cells> = seeds(seed, 0x85EB_CA6B, &DETAIL_OCTAVES).map(|(p, s)| Cells::new(p, s)).collect();
    let n = DETAIL_SIZE as usize;
    let mut out = vec![0u8; n * n * n];
    out.par_chunks_mut(n * n).enumerate().for_each(|(iz, slice)| {
        for iy in 0..n {
            for ix in 0..n {
                let p = Vec3::new(texel_center(ix, DETAIL_SIZE), texel_center(iy, DETAIL_SIZE), texel_center(iz, DETAIL_SIZE));
                slice[iy * n + ix] = unorm(worley_fbm(&worley, p));
            }
        }
    });
    out
}

/// The weather-noise map: R coverage field (contrast-stretched so a
/// coverage threshold opens real holes), G type field, B storm cells,
/// A cirrus streaks.
pub fn weather_noise(seed: u64) -> Vec<u8> {
    let cover: Vec<Lattice> = seeds(seed, 0x27D4_EB2F, &COVER_OCTAVES).map(|(p, s)| Lattice::new(p, s)).collect();
    let kind: Vec<Lattice> = seeds(seed ^ 0xABCD, 0x1656_67B1, &TYPE_OCTAVES).map(|(p, s)| Lattice::new(p, s)).collect();
    let storm = Cells::new(STORM_PERIOD, seed ^ 0x5707);
    let streak: Vec<Lattice> = seeds(seed ^ 0xC1C1, 0x2545_F491, &CIRRUS_OCTAVES).map(|(p, s)| Lattice::new(p, s)).collect();
    let warp: Vec<Lattice> = seeds(seed ^ 0x3A3A, 0x9E37_79B9, &CIRRUS_WARP_OCTAVES).map(|(p, s)| Lattice::new(p, s)).collect();
    let n = WEATHER_SIZE as usize;
    let mut out = vec![0u8; n * n * 4];
    out.par_chunks_mut(n * 4).enumerate().for_each(|(iy, row)| {
        for ix in 0..n {
            let p = Vec2::new(texel_center(ix, WEATHER_SIZE), texel_center(iy, WEATHER_SIZE));
            // A 2-D slice (z = 0) of the 3-D tiling noises.
            let q = Vec3::new(p.x, p.y, 0.0);
            let c = remap(fbm(&cover, q), 0.3, 0.72, 0.0, 1.0);
            let k = fbm(&kind, q);
            let s = 1.0 - storm.distance(q);
            // Cirrus fibres along X, bent by a periodic warp (the squeeze is a
            // whole number, so the field still tiles).
            let bend = fbm(&warp, Vec3::new(p.x, p.y, 0.5)) - 0.5;
            let fibre = Vec3::new(p.x, (p.y + bend * 0.3) * CIRRUS_STRETCH, 0.25);
            let ci = remap(fbm(&streak, fibre), 0.35, 0.72, 0.0, 1.0);
            let i = ix * 4;
            row[i] = unorm(c);
            row[i + 1] = unorm(k);
            row[i + 2] = unorm(s * s);
            row[i + 3] = unorm(ci);
        }
    });
    out
}
