//! Directional wave spectrum (water.md "Ocean surface"): JONSWAP with fetch, TMA shallow-water
//! correction, Donelan-Banner directional spreading (Horvath 2015), split into band-limited cascades
//! with power-of-two tile sizes, and the deterministic initial amplitudes `h0` of Tessendorf (2001).
//!
//! Portions (the JONSWAP peak/alpha relations, the integer hash and the Box-Muller pair) are ported
//! from bevy-aqua 0.1.3 (`crates/bevy-aqua-fft/src/lib.rs`, commit 98d755f),
//! Copyright (c) bevy-aqua contributors, MIT OR Apache-2.0. See `THIRD_PARTY_NOTICES.md`.
//! Changes: physical amplitude (no fixed-RMS normalisation), noise keyed by wave vector instead of
//! texel index (so a 64² CPU twin and a 256² GPU grid share every coefficient), TMA, Donelan-Banner
//! spreading, finite-depth dispersion.

// Numeric kernels: grid indices, frequencies and sizes convert between integer and float types
// throughout, always within ranges far below the lossy limits (grids ≤ 4096², tiles ≤ 2^16 m).
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names
)]

use std::f64::consts::{PI as PI64, TAU as TAU64};

use glam::Vec2;
use rustfft::num_complex::Complex32;
use serde::{Deserialize, Serialize};

/// Gravity in m/s².
pub const GRAVITY: f32 = 9.81;

/// Wind input for the spectrum. Scaffold until `sf_wind::WindState` lands (ROADMAP M6 WD1): the
/// spectrum only needs the 10 m wind speed and the direction the wind blows toward.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindInput {
    /// Wind speed 10 m above the surface, m/s.
    pub speed: f32,
    /// Unit XZ direction the wind blows toward.
    pub toward: Vec2,
}

/// Inputs of one sea state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectrumParams {
    pub wind: WindInput,
    /// Fetch: open water upwind, metres (JONSWAP).
    pub fetch: f32,
    /// Water depth for TMA and finite-depth dispersion, metres. `None` = deep water.
    pub depth: Option<f32>,
    /// Amplitude scale; 1 is the physical spectrum.
    pub amplitude: f32,
    /// Horizontal displacement scale (Tessendorf's lambda). 0 = no chop.
    pub choppiness: f32,
}

impl Default for SpectrumParams {
    fn default() -> Self {
        Self {
            wind: WindInput {
                speed: 8.0,
                toward: Vec2::X,
            },
            fetch: 100_000.0,
            depth: None,
            amplitude: 1.0,
            choppiness: 0.8,
        }
    }
}

impl SpectrumParams {
    /// JONSWAP peak angular frequency (rad/s).
    pub fn omega_peak(&self) -> f64 {
        let (u, f, g) = (
            f64::from(self.wind.speed.max(0.1)),
            f64::from(self.fetch.max(1.0)),
            f64::from(GRAVITY),
        );
        22.0 * (g * g / (u * f)).powf(1.0 / 3.0)
    }

    /// Dispersion: angular frequency of wave number `k` (rad/m).
    pub fn omega(&self, k: f64) -> f64 {
        let g = f64::from(GRAVITY);
        match self.depth {
            Some(d) => (g * k * (k * f64::from(d)).tanh()).sqrt(),
            None => (g * k).sqrt(),
        }
    }

    /// dω/dk.
    fn group(&self, k: f64, omega: f64) -> f64 {
        let g = f64::from(GRAVITY);
        match self.depth {
            Some(d) => {
                let kh = k * f64::from(d);
                let sech = 1.0 / kh.cosh();
                g * (kh.tanh() + kh * sech * sech) / (2.0 * omega.max(1e-9))
            }
            None => 0.5 * (g / k).sqrt(),
        }
    }

    /// JONSWAP frequency spectrum with the TMA depth factor, m²·s.
    pub fn frequency_spectrum(&self, omega: f64) -> f64 {
        if omega <= 0.0 {
            return 0.0;
        }
        let (u, f, g) = (
            f64::from(self.wind.speed.max(0.1)),
            f64::from(self.fetch.max(1.0)),
            f64::from(GRAVITY),
        );
        let alpha = 0.076 * (u * u / (f * g)).powf(0.22);
        let wp = self.omega_peak();
        let sigma = if omega <= wp { 0.07 } else { 0.09 };
        let r = (-(omega - wp).powi(2) / (2.0 * sigma * sigma * wp * wp)).exp();
        let s =
            alpha * g * g / omega.powi(5) * (-1.25 * (wp / omega).powi(4)).exp() * 3.3_f64.powf(r);
        s * self.tma(omega)
    }

    /// Kitaigorodskii depth attenuation (TMA), approximated as in Horvath 2015.
    fn tma(&self, omega: f64) -> f64 {
        let Some(d) = self.depth else { return 1.0 };
        let wh = omega * (f64::from(d) / f64::from(GRAVITY)).sqrt();
        if wh <= 1.0 {
            0.5 * wh * wh
        } else if wh < 2.0 {
            1.0 - 0.5 * (2.0 - wh).powi(2)
        } else {
            1.0
        }
    }

    /// Donelan-Banner directional spreading, normalised over θ in (-π, π].
    pub fn spreading(&self, omega: f64, theta: f64) -> f64 {
        let ratio = omega / self.omega_peak();
        let beta = if ratio < 0.95 {
            2.61 * ratio.powf(1.3)
        } else if ratio < 1.6 {
            2.28 * ratio.powf(-0.65)
        } else {
            let eps = -0.4 + 0.8393 * (-0.567 * (ratio * ratio).ln()).exp();
            10f64.powf(eps)
        };
        let sech = 1.0 / (beta * theta).cosh();
        beta / (2.0 * (beta * PI64).tanh()) * sech * sech
    }

    /// Variance density in wave-vector space, m⁴ (integrate over dkx·dkz for m²).
    pub fn density(&self, k: glam::DVec2) -> f64 {
        let kl = k.length();
        if kl <= 0.0 {
            return 0.0;
        }
        let omega = self.omega(kl);
        let wind = self
            .wind
            .toward
            .as_dvec2()
            .try_normalize()
            .unwrap_or(glam::DVec2::X);
        let theta = wind.angle_to(k);
        self.frequency_spectrum(omega) * self.group(kl, omega) / kl * self.spreading(omega, theta)
    }

    /// ∫ S(ω) dω over the wave numbers `[k_lo, k_hi)`: the analytic height variance of that band.
    pub fn band_variance(&self, k_lo: f64, k_hi: f64) -> f64 {
        let (w0, w1) = (self.omega(k_lo.max(1e-6)), self.omega(k_hi));
        // Simpson in log ω.
        let steps = 4000;
        let (l0, l1) = (w0.ln(), w1.ln());
        let hstep = (l1 - l0) / f64::from(steps);
        let f = |l: f64| {
            let w = l.exp();
            self.frequency_spectrum(w) * w
        };
        let mut sum = f(l0) + f(l1);
        for i in 1..steps {
            sum += f(l0 + hstep * f64::from(i)) * if i % 2 == 1 { 4.0 } else { 2.0 };
        }
        sum * hstep / 3.0
    }
}

/// The cascade tiles: power-of-two sizes in metres, largest first, each band-limited so that no
/// wave is counted twice. Cascade `i` keeps wave numbers in `[k_lo(i), k_hi(i))`; the boundary
/// between `i` and `i + 1` is the wavelength `size(i + 1) / split` (default 2: the longest wave of
/// cascade `i + 1` repeats twice per tile, and cascade `i` keeps 8+ texels per wave on the
/// 64² CPU grid).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cascades {
    sizes: Vec<u32>,
    /// Band boundary divisor (power of two).
    #[serde(default = "default_split")]
    split: u32,
}

/// Default tile sizes (water.md "Ocean surface"): 256, 64, 16, 4 m.
pub const DEFAULT_CASCADES: [u32; 4] = [256, 64, 16, 4];

/// Default band boundary divisor.
pub const DEFAULT_SPLIT: u32 = 2;

fn default_split() -> u32 {
    DEFAULT_SPLIT
}

impl Default for Cascades {
    fn default() -> Self {
        Self {
            sizes: DEFAULT_CASCADES.to_vec(),
            split: DEFAULT_SPLIT,
        }
    }
}

impl Cascades {
    /// `sizes` must be powers of two, strictly decreasing, each at least 4 m.
    pub fn new(sizes: &[u32]) -> Option<Self> {
        let ok = !sizes.is_empty()
            && sizes.iter().all(|s| s.is_power_of_two() && *s >= 4)
            && sizes.windows(2).all(|w| w[0] > w[1]);
        ok.then(|| Self {
            sizes: sizes.to_vec(),
            split: DEFAULT_SPLIT,
        })
    }

    /// Same tiles, band boundary at `size(i + 1) / split` (a power of two).
    #[must_use]
    pub fn with_split(mut self, split: u32) -> Self {
        assert!(split.is_power_of_two());
        self.split = split;
        self
    }

    pub fn len(&self) -> usize {
        self.sizes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sizes.is_empty()
    }

    pub fn size(&self, i: usize) -> u32 {
        self.sizes[i]
    }

    /// Wave-number band of cascade `i`, rad/m (`k_hi` is infinite for the last cascade: its grid's
    /// Nyquist limit applies).
    pub fn band(&self, i: usize) -> (f64, f64) {
        let boundary = |j: usize| TAU64 / (f64::from(self.sizes[j + 1]) / f64::from(self.split));
        let lo = if i == 0 { 0.0 } else { boundary(i - 1) };
        let hi = if i + 1 < self.sizes.len() {
            boundary(i)
        } else {
            f64::INFINITY
        };
        (lo, hi)
    }

    /// Smallest grid that holds cascade `i`'s whole band below its Nyquist row.
    pub fn min_resolution(&self, i: usize) -> usize {
        let (_, hi) = self.band(i);
        if hi.is_infinite() {
            return 0;
        }
        let max_index = (hi * f64::from(self.sizes[i]) / TAU64).ceil() as usize;
        (2 * max_index + 2).next_power_of_two()
    }
}

/// One cascade's initial amplitudes on an `n × n` grid. Index `iz * n + ix`, `ix` = signed
/// frequency mod `n`. The Nyquist row and column are zero.
#[derive(Clone, Debug, PartialEq)]
pub struct H0 {
    pub n: usize,
    pub size: u32,
    pub cascade: usize,
    pub h0: Vec<Complex32>,
    /// Expected height variance of the band on this grid, Σ E(k)·Δk², m².
    pub variance: f64,
}

/// Signed frequency of grid index `i` on an `n` grid (Nyquist maps to `n / 2`).
pub fn signed(i: usize, n: usize) -> i32 {
    if i <= n / 2 {
        i as i32
    } else {
        i as i32 - n as i32
    }
}

impl H0 {
    /// Deterministic in `(seed, cascade, params)`; identical for every bin two grid sizes share.
    pub fn generate(
        params: &SpectrumParams,
        cascades: &Cascades,
        cascade: usize,
        n: usize,
        seed: u32,
    ) -> Self {
        assert!(n.is_power_of_two() && n >= 4, "grid must be a power of two");
        let size = cascades.size(cascade);
        let (lo, hi) = cascades.band(cascade);
        let dk = TAU64 / f64::from(size);
        let mut h0 = vec![Complex32::new(0.0, 0.0); n * n];
        let mut variance = 0.0;
        for iz in 0..n {
            for ix in 0..n {
                let (sx, sz) = (signed(ix, n), signed(iz, n));
                if sx.unsigned_abs() as usize == n / 2 || sz.unsigned_abs() as usize == n / 2 {
                    continue;
                }
                let k = glam::DVec2::new(f64::from(sx), f64::from(sz)) * dk;
                let kl = k.length();
                if kl <= 0.0 || kl < lo || kl >= hi {
                    continue;
                }
                let e = params.density(k) * dk * dk * f64::from(params.amplitude).powi(2);
                variance += e;
                let g = gaussian_pair(hash(seed ^ hash(((cascade as u32) << 24) ^ key(sx, sz))));
                h0[iz * n + ix] = Complex32::new(g.x, g.y) * (0.5 * e.sqrt()) as f32;
            }
        }
        Self {
            n,
            size,
            cascade,
            h0,
            variance,
        }
    }

    /// `mix(a, b, w)`: a weather transition. The FFT is linear, so blending `h0` blends the sea
    /// states without a pop (water.md "Weather changes"). Both must come from the same seed and grid.
    pub fn blend(a: &Self, b: &Self, w: f32) -> Self {
        assert!(a.n == b.n && a.size == b.size && a.cascade == b.cascade);
        let h0 =
            a.h0.iter()
                .zip(&b.h0)
                .map(|(x, y)| *x * (1.0 - w) + *y * w)
                .collect();
        let v = a.variance * f64::from(1.0 - w).powi(2) + b.variance * f64::from(w).powi(2);
        Self {
            n: a.n,
            size: a.size,
            cascade: a.cascade,
            h0,
            variance: v,
        }
    }

    /// Height variance this realization produces (Parseval): 2·Σ|h0|².
    pub fn realized_variance(&self) -> f64 {
        2.0 * self.h0.iter().map(|c| f64::from(c.norm_sqr())).sum::<f64>()
    }
}

fn key(sx: i32, sz: i32) -> u32 {
    // 12 bits per signed component (grids up to 4096²).
    ((sx & 0xFFF) as u32) | (((sz & 0xFFF) as u32) << 12)
}

/// Integer hash (bevy-aqua; MIT OR Apache-2.0).
pub(crate) fn hash(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^ (value >> 16)
}

/// Two standard normal deviates (Box-Muller; bevy-aqua, MIT OR Apache-2.0).
pub(crate) fn gaussian_pair(seed: u32) -> Vec2 {
    let first = hash(seed).max(1);
    let second = hash(seed ^ 0x68BC_21EB);
    let u1 = (first as f32 + 0.5) / (u32::MAX as f32 + 1.0);
    let u2 = (second as f32 + 0.5) / (u32::MAX as f32 + 1.0);
    let radius = (-2.0 * u1.ln()).sqrt();
    let angle = std::f32::consts::TAU * u2;
    radius * Vec2::new(angle.cos(), angle.sin())
}
