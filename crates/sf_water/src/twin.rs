//! CPU twin of the long wave cascades (water.md "The water query", "Waves"): the same `h0` the GPU
//! uses, evolved in time and inverse-transformed with `rustfft` on a small grid, then sampled
//! with a Catmull-Rom cubic. Displaced height at an XZ inverts the horizontal displacement by fixed-point
//! iteration (HDRP and Crest do the same).
//!
//! Spatial field convention: `f(x) = Σ_k F(k)·e^{i k·x}` over the tile; heights
//! `h̃(k,t) = h0(k)·e^{iωt} + conj(h0(−k))·e^{−iωt}`; horizontal displacement `D̃ = λ·i·k̂·h̃`
//! (crests sharpen, as Gerstner waves). Grids are stored x-major: `grid[ix * n + iz]`.

// Numeric kernels: grid indices, frequencies and sizes convert between integer and float types
// throughout, always within ranges far below the lossy limits (grids ≤ 4096², tiles ≤ 2^16 m).
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names
)]

use std::f64::consts::TAU;
use std::sync::Arc;

use glam::{IVec3, Vec2, Vec3};
use rustfft::num_complex::Complex32;
use rustfft::{Fft, FftPlanner};
use sf_coords::{GRID_CELL, WorldPos};

use crate::spectrum::{H0, SpectrumParams, signed};

/// Fixed-point iterations of the displaced-height inversion.
pub const INVERSION_ITERATIONS: usize = 3;

/// Position inside a tile of edge `size` metres for a world position. A pure function of the
/// absolute position: the grid cell (an integer) contributes `(cell · 256) mod size` exactly, so
/// the render origin never enters and an origin shift moves no wave (water.md "Floating origin").
#[allow(clippy::cast_precision_loss)]
pub fn tile_coords(at: WorldPos, size: u32) -> Vec2 {
    let s = i64::from(size);
    let cell = i64::from(GRID_CELL as u32);
    let phase = |c: i32| (i64::from(c) * cell).rem_euclid(s) as f32;
    let sf = size as f32;
    Vec2::new(
        (phase(at.cell.x) + at.local.x).rem_euclid(sf),
        (phase(at.cell.z) + at.local.z).rem_euclid(sf),
    )
}

/// The per-cascade phase the GPU adds to camera-relative XZ: `(origin_cell · 256) mod size`,
/// exact (integers). The GPU samples at `fract((phase + rel) / size)`.
#[allow(clippy::cast_precision_loss)]
pub fn origin_phase(origin_cell: IVec3, size: u32) -> Vec2 {
    let s = i64::from(size);
    let cell = i64::from(GRID_CELL as u32);
    Vec2::new(
        (i64::from(origin_cell.x) * cell).rem_euclid(s) as f32,
        (i64::from(origin_cell.z) * cell).rem_euclid(s) as f32,
    )
}

/// One bin's constants.
#[derive(Clone, Copy)]
struct Bin {
    index: u32,
    omega: f64,
    k: Vec2,
    k_hat: Vec2,
    h0: Complex32,
    /// `conj(h0(−k))`.
    h0_mirror_conj: Complex32,
}

/// Wave state at one time: displacement, slopes and their rates on the grid.
#[derive(Clone, Debug, Default)]
pub struct WaveGrids {
    pub height: Vec<f32>,
    pub height_rate: Vec<f32>,
    pub dx: Vec<f32>,
    pub dz: Vec<f32>,
    pub dx_rate: Vec<f32>,
    pub dz_rate: Vec<f32>,
    pub slope_x: Vec<f32>,
    pub slope_z: Vec<f32>,
}

/// One cascade on the CPU.
pub struct CascadeTwin {
    n: usize,
    size: u32,
    choppiness: f32,
    bins: Vec<Bin>,
    fft: Arc<dyn Fft<f32>>,
    scratch: Vec<Complex32>,
    buffers: [Vec<Complex32>; 4],
    grids: WaveGrids,
    time: f64,
}

impl CascadeTwin {
    pub fn new(h0: &H0, params: &SpectrumParams) -> Self {
        let n = h0.n;
        let dk = std::f32::consts::TAU / h0.size as f32;
        let mut bins = Vec::new();
        for iz in 0..n {
            for ix in 0..n {
                let i = iz * n + ix;
                let m = ((n - iz) % n) * n + (n - ix) % n;
                if h0.h0[i] == Complex32::ZERO && h0.h0[m] == Complex32::ZERO {
                    continue;
                }
                #[allow(clippy::cast_precision_loss)]
                let k = Vec2::new(signed(ix, n) as f32, signed(iz, n) as f32) * dk;
                bins.push(Bin {
                    index: u32::try_from(i).expect("grid fits u32"),
                    omega: params.omega(f64::from(k.length())),
                    k,
                    k_hat: k.normalize_or_zero(),
                    h0: h0.h0[i],
                    h0_mirror_conj: h0.h0[m].conj(),
                });
            }
        }
        let fft = FftPlanner::new().plan_fft_inverse(n);
        let scratch = vec![Complex32::ZERO; fft.get_inplace_scratch_len()];
        let zero = vec![Complex32::ZERO; n * n];
        let grid = vec![0.0; n * n];
        Self {
            n,
            size: h0.size,
            choppiness: params.choppiness,
            bins,
            fft,
            scratch,
            buffers: [zero.clone(), zero.clone(), zero.clone(), zero],
            grids: WaveGrids {
                height: grid.clone(),
                height_rate: grid.clone(),
                dx: grid.clone(),
                dz: grid.clone(),
                dx_rate: grid.clone(),
                dz_rate: grid.clone(),
                slope_x: grid.clone(),
                slope_z: grid,
            },
            time: f64::NAN,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn resolution(&self) -> usize {
        self.n
    }

    /// Bins with energy (evolved per step).
    pub fn active_bins(&self) -> usize {
        self.bins.len()
    }

    pub fn grids(&self) -> &WaveGrids {
        &self.grids
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    /// Evaluates the cascade at time `t` seconds. A pure function of `(h0, t)`: phases are reduced
    /// in f64 before the f32 sine, so a long session keeps full precision.
    #[allow(clippy::cast_possible_truncation)]
    pub fn evaluate(&mut self, t: f64) {
        for b in &mut self.buffers {
            b.fill(Complex32::ZERO);
        }
        let i = Complex32::new(0.0, 1.0);
        let lambda = self.choppiness;
        for bin in &self.bins {
            let phase = (bin.omega * t).rem_euclid(TAU) as f32;
            let (s, c) = phase.sin_cos();
            let e = Complex32::new(c, s);
            let a = bin.h0 * e;
            let b = bin.h0_mirror_conj * e.conj();
            let h = a + b;
            let rate = i * (bin.omega as f32) * (a - b);
            let idx = bin.index as usize;
            // Two Hermitian spectra per complex transform: re = first, im = second.
            self.buffers[0][idx] = h + i * rate;
            let dx = i * bin.k_hat.x * lambda * h;
            let dz = i * bin.k_hat.y * lambda * h;
            self.buffers[1][idx] = dx + i * dz;
            let dxr = i * bin.k_hat.x * lambda * rate;
            let dzr = i * bin.k_hat.y * lambda * rate;
            self.buffers[2][idx] = dxr + i * dzr;
            let sx = i * bin.k.x * h;
            let sz = i * bin.k.y * h;
            self.buffers[3][idx] = sx + i * sz;
        }
        for b in 0..4 {
            inverse_2d(&*self.fft, &mut self.buffers[b], &mut self.scratch, self.n);
        }
        let g = &mut self.grids;
        for (j, c) in self.buffers[0].iter().enumerate() {
            g.height[j] = c.re;
            g.height_rate[j] = c.im;
        }
        for (j, c) in self.buffers[1].iter().enumerate() {
            g.dx[j] = c.re;
            g.dz[j] = c.im;
        }
        for (j, c) in self.buffers[2].iter().enumerate() {
            g.dx_rate[j] = c.re;
            g.dz_rate[j] = c.im;
        }
        for (j, c) in self.buffers[3].iter().enumerate() {
            g.slope_x[j] = c.re;
            g.slope_z[j] = c.im;
        }
        self.time = t;
    }

    /// Catmull-Rom stencil (periodic) at tile coordinates (metres). Cubic, not bilinear: at 64² and
    /// 4 m texels bilinear loses ~2 % of the peak waves' amplitude (4 cm at Hs 2 m); the cubic keeps
    /// the twin within millimetres of the exact field (test `twin_matches_the_reference_dft`).
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        clippy::cast_possible_wrap
    )]
    fn stencil(&self, at: Vec2) -> Stencil {
        let n = self.n as i64;
        let u = at * (self.n as f32 / self.size as f32);
        let (fx, fz) = (u.x.floor(), u.y.floor());
        let axis =
            |f: f32| std::array::from_fn(|i| (f as i64 - 1 + i as i64).rem_euclid(n) as usize);
        Stencil {
            x: axis(fx),
            z: axis(fz),
            wx: catmull_rom(u.x - fx),
            wz: catmull_rom(u.y - fz),
        }
    }

    fn fetch(&self, grid: &[f32], s: &Stencil) -> f32 {
        let n = self.n;
        let mut sum = 0.0;
        for (x, wx) in s.x.iter().zip(s.wx) {
            let row = &grid[x * n..x * n + n];
            sum += wx
                * (s.wz[0] * row[s.z[0]]
                    + s.wz[1] * row[s.z[1]]
                    + s.wz[2] * row[s.z[2]]
                    + s.wz[3] * row[s.z[3]]);
        }
        sum
    }

    /// Horizontal displacement at undisplaced tile coordinates.
    pub fn displacement(&self, at: Vec2) -> Vec2 {
        let s = self.stencil(at);
        Vec2::new(
            self.fetch(&self.grids.dx, &s),
            self.fetch(&self.grids.dz, &s),
        )
    }

    /// Everything at undisplaced tile coordinates `x0`: (displacement xyz, rate xyz, slope xz).
    pub fn lagrangian(&self, x0: Vec2) -> (Vec3, Vec3, Vec2) {
        let g = &self.grids;
        let s = self.stencil(x0);
        (
            Vec3::new(
                self.fetch(&g.dx, &s),
                self.fetch(&g.height, &s),
                self.fetch(&g.dz, &s),
            ),
            Vec3::new(
                self.fetch(&g.dx_rate, &s),
                self.fetch(&g.height_rate, &s),
                self.fetch(&g.dz_rate, &s),
            ),
            Vec2::new(self.fetch(&g.slope_x, &s), self.fetch(&g.slope_z, &s)),
        )
    }
}

/// Grid indices and weights of a 4×4 cubic fetch.
struct Stencil {
    x: [usize; 4],
    z: [usize; 4],
    wx: [f32; 4],
    wz: [f32; 4],
}

fn catmull_rom(t: f32) -> [f32; 4] {
    let (t2, t3) = (t * t, t * t * t);
    [
        t2 - 0.5 * t3 - 0.5 * t,
        1.5 * t3 - 2.5 * t2 + 1.0,
        2.0 * t2 + 0.5 * t - 1.5 * t3,
        0.5 * t3 - 0.5 * t2,
    ]
}

/// In-place 2D inverse DFT (unnormalised) of a z-major buffer; the result is x-major.
fn inverse_2d(fft: &dyn Fft<f32>, buf: &mut [Complex32], scratch: &mut [Complex32], n: usize) {
    fft.process_with_scratch(buf, scratch);
    transpose(buf, n);
    fft.process_with_scratch(buf, scratch);
}

fn transpose(buf: &mut [Complex32], n: usize) {
    for r in 0..n {
        for c in r + 1..n {
            buf.swap(r * n + c, c * n + r);
        }
    }
}

/// What the twin gives at a world XZ.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveSample {
    /// Surface height above the rest level, metres.
    pub height: f32,
    pub normal: Vec3,
    /// Orbital velocity of the surface point, m/s.
    pub velocity: Vec3,
}

/// The ocean's CPU twin: the cascades the query evaluates (cascade 0; cascade 1 too on High when the
/// sea is rough), summed.
pub struct WaveTwin {
    pub cascades: Vec<CascadeTwin>,
}

impl WaveTwin {
    pub fn evaluate(&mut self, t: f64) {
        for c in &mut self.cascades {
            c.evaluate(t);
        }
    }

    pub fn time(&self) -> f64 {
        self.cascades.first().map_or(f64::NAN, CascadeTwin::time)
    }

    /// Displaced surface at world XZ `at` (its Y is ignored): inverts the horizontal displacement
    /// with [`INVERSION_ITERATIONS`] fixed-point steps `x0 ← p − D(x0)`.
    pub fn sample(&self, at: WorldPos) -> WaveSample {
        self.sample_iterations(at, INVERSION_ITERATIONS)
    }

    pub fn sample_iterations(&self, at: WorldPos, iterations: usize) -> WaveSample {
        let mut base = [Vec2::ZERO; 4];
        for (b, c) in base.iter_mut().zip(&self.cascades) {
            *b = tile_coords(at, c.size);
        }
        let mut offset = Vec2::ZERO; // x0 − p, shared by every cascade
        for _ in 0..iterations {
            let mut d = Vec2::ZERO;
            for (b, c) in base.iter().zip(&self.cascades) {
                d += c.displacement(*b + offset);
            }
            offset = -d;
        }
        let mut disp = Vec3::ZERO;
        let mut rate = Vec3::ZERO;
        let mut slope = Vec2::ZERO;
        for (b, c) in base.iter().zip(&self.cascades) {
            let (d, r, s) = c.lagrangian(*b + offset);
            disp += d;
            rate += r;
            slope += s;
        }
        WaveSample {
            height: disp.y,
            normal: Vec3::new(-slope.x, 1.0, -slope.y).normalize(),
            velocity: rate,
        }
    }
}

/// Reference: the exact field (direct sum over bins, no grid) of one cascade at undisplaced tile
/// coordinates. Slow; for tests and goldens.
#[allow(clippy::cast_possible_truncation)]
pub fn reference_height(h0: &H0, params: &SpectrumParams, t: f64, x0: Vec2) -> (f32, Vec2) {
    let n = h0.n;
    let dk = TAU / f64::from(h0.size);
    let (mut h, mut dx, mut dz) = (0.0f64, 0.0f64, 0.0f64);
    for iz in 0..n {
        for ix in 0..n {
            let i = iz * n + ix;
            let m = ((n - iz) % n) * n + (n - ix) % n;
            if h0.h0[i] == Complex32::ZERO && h0.h0[m] == Complex32::ZERO {
                continue;
            }
            let k = glam::DVec2::new(f64::from(signed(ix, n)), f64::from(signed(iz, n))) * dk;
            let w = params.omega(k.length());
            let e = |p: f64| (p.cos(), p.sin());
            let (c, s) = e(w * t);
            let a = (f64::from(h0.h0[i].re), f64::from(h0.h0[i].im));
            let b = (f64::from(h0.h0[m].re), -f64::from(h0.h0[m].im));
            // h̃ = a·e^{iωt} + b·e^{−iωt}
            let hr = a.0 * c - a.1 * s + b.0 * c + b.1 * s;
            let hi = a.0 * s + a.1 * c - b.0 * s + b.1 * c;
            let (pc, ps) = e(k.dot(x0.as_dvec2()));
            let re = hr * pc - hi * ps; // Re(h̃·e^{ik·x})
            let im = hr * ps + hi * pc; // Im(h̃·e^{ik·x})
            h += re;
            let kh = k.normalize_or_zero();
            // Re(i·k̂·λ·h̃·e^{ikx}) = −k̂·λ·Im(...)
            dx -= kh.x * im;
            dz -= kh.y * im;
        }
    }
    let l = f64::from(params.choppiness);
    (h as f32, Vec2::new((dx * l) as f32, (dz * l) as f32))
}
