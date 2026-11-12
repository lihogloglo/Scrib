//! Analytic shore waves (water.md "Shoreline and depth colour", W-GERSTNER): where the FFT swell
//! fades out over the shore band, three Gerstner (trochoidal) wave trains take over, travelling
//! shoreward along the baked shore direction with their crests on the iso-lines of the shore
//! distance, plus a swash term that lifts and drops the water on the beach at the main train's
//! period. `water_common.wesl` `shore_waves` is the GPU twin: same constants, same order of
//! operations, same noise hash.
//!
//! Physics (each choice and its source):
//! - **Depth** `h = β · d`, β the baked local bed slope (W-GERSTNER-DEPTH,
//!   `ShoreField::sample_slope`: the bed's mean slope from the shoreline, blurred), or the uniform
//!   `ShoreSettings::beach_slope` (1:20) without bathymetry (`--off shoredepth`).
//! - **Dispersion** `ω² = g·k·tanh(k·h)` through Eckart's explicit form `k = k0 / √tanh(k0·h)`,
//!   `k0 = ω²/g` (Eckart 1951; ≤ 5 % from the exact root). Over a linear slope its phase integral
//!   is closed-form: `Φ(d) = ∫k dd = (atanh v + atan v) / β`, `v = √tanh(k0·h)`, so crests slow
//!   and bunch up toward the shore exactly as the local wavelength shortens.
//! - **Amplitude**: the swell's significant amplitude `Hs/2 = 2√m0` (`m0` = the FFT's swell-band
//!   variance, so wind and sea state drive both), shoaled by Green's law `(h_edge/h)^¼`, capped by
//!   the depth-limited breaker `H ≤ γ·h`, γ = 0.78 (`McCowan` 1894), and by Miche's steepness limit
//!   `kA ≤ 0.142·π·tanh(kh)`; smooth min of the two so there is no kink at the break point.
//! - **Horizontal motion**: the linear-theory surface orbit `A·coth(kh)` (Dean & Dalrymple 2002),
//!   so shallow crests lean shoreward and sharpen; the sum `Σ Q·k·A·coth(kh)` is held at ≤ 0.75
//!   (the Gerstner Jacobian `1 − Σ Q·k·A·coth` stays ≥ 0.25 before the phase noise, > 0 with it:
//!   no loops; GPU Gems 1 ch. 1, Finch 2004), and `coth` is capped so the orbit's growth with
//!   distance, `½γβ·coth`, stays ≤ [`ORBIT_LIMIT`] (steep real beds; W-GERSTNER-DEPTH).
//! - **Blend with the FFT**: weight `o·(1 − smoothstep(0, swell_ramp, d))`, the exact complement of
//!   the FFT swell's `o·smoothstep(0, swell_ramp, d)`: zero at the band's edge, no pop.
//! - **Swash**: the incident swash of Stockdon et al. 2006, `S = 0.75·β·√(H0·L0)`, as a vertical
//!   oscillation of amplitude `S/2` over the beach at the main train's shoreline phase, fast uprush
//!   and slow backwash (`sin(θ − 0.5·sin θ)`), fading seaward over 30 % of the band and landward at
//!   the land margin (≈ 2× the run-up on the assumed slope).
//! - **Variation** (no conveyor belt): two close periods (1, 1.12× the peak) beat into sets every
//!   ~8 waves; a short wind-sea train (1.8× the peak) on top; value noise in world space bends the
//!   crests (± 0.8 rad over 64 m) and varies the sets along the shore (× 0.6 … 1 over 32 m, cross-
//!   faded over the 256 s repeat period).
//!
//! Every period is a whole number of cycles per [`REPEAT_PERIOD`], so the GPU's `t mod T` clock
//! never pops. Mechanism prior art: the owner's own Godotwind shore swash (MIT,
//! `ocean_shore_common.gdshaderinc`, distance-to-shore-driven phase and envelopes); every constant
//! here is recalibrated from the physics above.

// Numeric kernel: the physics symbols (d, o, h, k, t, ...) read as in the sources.
#![allow(clippy::many_single_char_names)]

use std::f32::consts::{PI, TAU};

use glam::{DVec2, Vec2, Vec3, Vec4};

use crate::shore::ShoreSettings;
use crate::spectrum::{Cascades, GRAVITY, REPEAT_PERIOD, SpectrumParams};

/// Wave trains.
pub const TRAINS: usize = 3;
/// Share of the envelope amplitude per train (sum 1: the crests can at most add up to it).
pub const SHARES: [f32; TRAINS] = [0.5, 0.3, 0.2];
/// Angular frequency per train relative to the spectrum's peak.
pub const FREQUENCIES: [f32; TRAINS] = [1.0, 1.12, 1.8];
/// How much of the phase noise each train takes (different bends per train).
const PHASE_SPREAD: [f32; TRAINS] = [1.0, -0.7, 1.3];
/// Depth-limited breaker index H/h (`McCowan` 1894).
pub const BREAKER_INDEX: f32 = 0.78;
/// Miche's limit kA ≤ 0.142·π·tanh(kh).
const MICHE: f32 = 0.142 * PI;
/// Σ Q·k·A·coth(kh) never exceeds this.
pub const STEEPNESS_LIMIT: f32 = 0.75;
/// The horizontal orbit's growth with shore distance, `∂(A·coth kh)/∂d`, never exceeds this
/// (W-GERSTNER-DEPTH: with the breaker-held phase term ≤ ½γ at the shoreline the Gerstner Jacobian
/// stays > 0.2 there on beds up to 0.3).
pub const ORBIT_LIMIT: f32 = 0.4;
/// Depth below which the waves stop slowing (m): the inner swash zone, so wavelengths stay ≥ ~3 m.
pub const MIN_DEPTH: f32 = 0.1;
/// World period of the noise, metres (the GPU gets the floating origin modulo this).
pub const NOISE_PERIOD: f64 = 1024.0;
const PHASE_CELLS: u32 = 16;
const SET_CELLS: u32 = 32;
const PHASE_NOISE: f32 = 0.8;
const SET_FLOOR: f32 = 0.6;
const SWASH_SKEW: f32 = 0.5;
/// The swash fades seaward over this share of the band.
const SWASH_ZONE: f32 = 0.3;
/// Above this `k0·h` Eckart's phase integral is linear (1/√tanh − 1 < 4e-4).
const ECKART_LINEAR: f32 = 4.0;

/// The shore waves of one sea state on one shore field.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShoreWaves {
    /// Angular frequency per train, rad/s, whole cycles per [`REPEAT_PERIOD`].
    pub omega: [f32; TRAINS],
    /// Offshore amplitude at the band's edge, m (the swell's Hs/2 × strength).
    pub amplitude: f32,
    /// Swash amplitude on the beach, m.
    pub swash: f32,
    pub beach_slope: f32,
    /// The band (m): the FFT swell's ramp.
    pub band: f32,
    /// Land side (m) over which the swash may lift the water.
    pub land_margin: f32,
    /// 1 = noise-varied sets and crests, 0 = uniform (tests).
    pub variation: f32,
    /// A sea-state transition (W-WIND, [`Self::blend`]): the trains of the sea it comes from, and the
    /// weight of [`Self::omega`] against them (1 = no transition). The amplitudes above are already
    /// blended; the two train sets are crossfaded (their phases differ, so they cannot be lerped).
    pub omega_from: [f32; TRAINS],
    pub mix: f32,
}

/// One evaluation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShoreWave {
    /// Displacement (x, y, z), m.
    pub offset: Vec3,
    /// (∂y/∂x, ∂y/∂z, ∂Dx/∂x, ∂Dz/∂z): added to the FFT's slopes for the shading normal.
    pub slope: Vec4,
    /// `sin` of the main train's phase (1 on its crests; the shore foam follows it).
    pub crest: f32,
    /// Σ Q·k·A·coth(kh) (≤ [`STEEPNESS_LIMIT`]).
    pub steepness: f32,
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Integer hash to [0, 1) (the GPU's is the same u32 arithmetic).
fn hash(x: u32, y: u32, seed: u32) -> f32 {
    let mut h = x.wrapping_mul(0x8da6_b343) ^ y.wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^= h >> 16;
    #[allow(clippy::cast_precision_loss)]
    let v = (h >> 8) as f32;
    v / 16_777_216.0
}

/// Tiling value noise in [0, 1] at `uv` ∈ [0, 1)² on a `cells`² lattice.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn value_noise(uv: Vec2, cells: u32, seed: u32) -> f32 {
    let g = uv * cells as f32;
    let i = g.floor();
    let f = g - i;
    let (x0, y0) = ((i.x as u32) % cells, (i.y as u32) % cells);
    let (x1, y1) = ((x0 + 1) % cells, (y0 + 1) % cells);
    let s = f * f * (Vec2::splat(3.0) - 2.0 * f);
    let top = hash(x0, y0, seed) + (hash(x1, y0, seed) - hash(x0, y0, seed)) * s.x;
    let bottom = hash(x0, y1, seed) + (hash(x1, y1, seed) - hash(x0, y1, seed)) * s.x;
    top + (bottom - top) * s.y
}

/// `∫ du / √tanh u` from 0 (Eckart's phase over a slope, in units of `k0·h`).
fn eckart_phase(u: f32) -> f32 {
    let closed = |u: f32| {
        let v = u.tanh().sqrt();
        v.atanh() + v.atan()
    };
    if u < ECKART_LINEAR { closed(u) } else { closed(ECKART_LINEAR) + u - ECKART_LINEAR }
}

/// Local wave number (rad/m) and phase `Φ(d)` (rad, 0 at the shoreline before the `MIN_DEPTH`
/// clamp) of angular frequency `omega` at depth `h` on slope `beta`.
pub fn local_wave(omega: f32, h: f32, beta: f32) -> (f32, f32) {
    let k0 = omega * omega / GRAVITY;
    let hc = h.max(MIN_DEPTH);
    let k = k0 / (k0 * hc).tanh().sqrt();
    let phi = if h >= MIN_DEPTH {
        eckart_phase(k0 * h) / beta
    } else {
        (eckart_phase(k0 * MIN_DEPTH) - k * (MIN_DEPTH - h)) / beta
    };
    (k, phi)
}

/// `omega` rounded to whole cycles per [`REPEAT_PERIOD`] (at least `min_cycles`).
#[allow(clippy::cast_possible_truncation)]
fn repeat_omega(omega: f64, min_cycles: f64) -> (f32, f64) {
    let cycles = (omega * REPEAT_PERIOD / std::f64::consts::TAU).round().max(min_cycles);
    ((cycles * std::f64::consts::TAU / REPEAT_PERIOD) as f32, cycles)
}

impl ShoreWaves {
    /// The shore waves of sea state `params` (the FFT's spectrum and cascades) on a field baked
    /// with `settings`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(params: &SpectrumParams, cascades: &Cascades, settings: &ShoreSettings) -> Self {
        // The swell cascades (0-1, as the GPU's `shore_scales`): their band's height variance.
        let swell = cascades.len().min(2);
        let k_hi = if swell == 0 {
            0.0
        } else {
            let (_, hi) = cascades.band(swell - 1);
            if hi.is_finite() { hi } else { std::f64::consts::TAU / 0.5 }
        };
        let m0 = if k_hi > 0.0 { params.band_variance(1e-6, k_hi) } else { 0.0 };
        let strength = settings.shore_waves.max(0.0);
        let amplitude = (2.0 * m0.max(0.0).sqrt()) as f32 * params.amplitude * strength;
        let peak = params.omega_peak();
        let mut omega = [0.0; TRAINS];
        let mut last = 0.0;
        for (i, f) in FREQUENCIES.iter().enumerate() {
            let (w, cycles) = repeat_omega(peak * f64::from(*f), last + 1.0);
            omega[i] = w;
            last = cycles;
        }
        let beta = settings.beach_slope.max(1e-3);
        // Stockdon et al. 2006, incident swash S = 0.75·β·√(H0·L0), half of it as an amplitude.
        let l0 = TAU * GRAVITY / (omega[0] * omega[0]);
        let swash = 0.375 * beta * (2.0 * amplitude * l0).sqrt();
        Self {
            omega,
            amplitude,
            swash,
            beach_slope: beta,
            band: settings.swell_ramp.max(1e-3),
            land_margin: (2.0 * swash / beta).clamp(2.0, 8.0),
            variation: 1.0,
            omega_from: omega,
            mix: 1.0,
        }
    }

    /// Noise coordinates of world XZ `p` (the GPU: the floating origin modulo [`NOISE_PERIOD`]
    /// plus render-space XZ).
    #[allow(clippy::cast_possible_truncation)]
    pub fn noise_coords(p: DVec2) -> Vec2 {
        let f = |x: f64| (x / NOISE_PERIOD).rem_euclid(1.0) as f32;
        Vec2::new(f(p.x), f(p.y))
    }

    /// The shore waves at shore distance `d` (m, + in water), openness `o`, shoreward direction
    /// `dir` ([`crate::ShoreField::sample_direction`]), noise coordinates `noise`
    /// ([`Self::noise_coords`]) and time `t` (s, any multiple of [`REPEAT_PERIOD`] apart is the same).
    /// Zero outside `(−land_margin, band)` and when off.
    /// Over the uniform [`Self::beach_slope`]; [`Self::eval_on`] takes the baked bed's.
    pub fn eval(&self, d: f32, o: f32, dir: Vec2, noise: Vec2, t: f32) -> ShoreWave {
        self.eval_on(d, o, self.beach_slope, dir, noise, t)
    }

    /// [`Self::eval`] over the local bed slope `beta` (W-GERSTNER-DEPTH,
    /// [`crate::ShoreField::sample_slope`]): the depth is `beta · d` and the closed-form Eckart
    /// phase assumes that slope from the shoreline to here (exact on a plane beach; on a real bed the
    /// local depth is exact and the phase a smooth approximation, since β is blurred).
    #[allow(clippy::float_cmp)] // the same trains exactly: no crossfade
    pub fn eval_on(&self, d: f32, o: f32, beta: f32, dir: Vec2, noise: Vec2, t: f32) -> ShoreWave {
        let beta = if beta.is_finite() { beta.max(1e-3) } else { self.beach_slope };
        let to = self.eval_trains(&self.omega, d, o, beta, dir, noise, t);
        if self.mix >= 1.0 || self.omega_from == self.omega {
            return to;
        }
        let from = self.eval_trains(&self.omega_from, d, o, beta, dir, noise, t);
        let w = self.mix.max(0.0);
        ShoreWave {
            offset: from.offset.lerp(to.offset, w),
            slope: from.slope.lerp(to.slope, w),
            crest: from.crest + (to.crest - from.crest) * w,
            steepness: from.steepness + (to.steepness - from.steepness) * w,
        }
    }

    /// `pair` blended at `w` (W-WIND): amplitudes, swash and land margin lerped, `from`'s trains
    /// crossfaded into `to`'s. `from` must not itself be mid-transition (one transition at a time).
    #[must_use]
    pub fn blend(from: &Self, to: &Self, w: f32) -> Self {
        let w = if w.is_finite() { w.clamp(0.0, 1.0) } else { 1.0 };
        let lerp = |a: f32, b: f32| a + (b - a) * w;
        Self {
            omega: to.omega,
            amplitude: lerp(from.amplitude, to.amplitude),
            swash: lerp(from.swash, to.swash),
            beach_slope: to.beach_slope,
            band: lerp(from.band, to.band),
            land_margin: lerp(from.land_margin, to.land_margin),
            variation: to.variation,
            omega_from: from.omega,
            mix: w,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_trains(&self, omega: &[f32; TRAINS], d: f32, o: f32, beta: f32, dir: Vec2, noise: Vec2, t: f32) -> ShoreWave {
        // (`!(a && b)` also rejects a NaN distance.)
        if !(d < self.band && d > -self.land_margin) || (self.amplitude <= 0.0 && self.swash <= 0.0) {
            return ShoreWave::default();
        }
        let h = beta * d.max(0.0);
        let h_edge = beta * self.band;
        let w = o * (1.0 - smoothstep(0.0, self.band, d));
        #[allow(clippy::cast_possible_truncation)]
        let fade = 0.5 - 0.5 * (TAU * t / REPEAT_PERIOD as f32).cos();
        let sets = value_noise(noise, SET_CELLS, 1) + (value_noise(noise, SET_CELLS, 2) - value_noise(noise, SET_CELLS, 1)) * fade;
        let set = 1.0 - self.variation * (1.0 - SET_FLOOR) * (1.0 - sets);
        let psi = self.variation * PHASE_NOISE * (2.0 * value_noise(noise, PHASE_CELLS, 3) - 1.0);
        // Green's-law shoaling, smooth-min'd with the depth-limited breaker.
        let shoal = self.amplitude * (h_edge / h.max(MIN_DEPTH)).max(1.0).powf(0.25);
        let breaker = 0.5 * BREAKER_INDEX * h;
        let env = shoal * breaker / (shoal.powi(4) + breaker.powi(4) + 1e-20).powf(0.25);
        let common = env * w * set;
        let mut trains = [(0.0f32, 0.0f32, 0.0f32, 0.0f32); TRAINS];
        let mut steep = 0.0;
        // The orbit's envelope term (W-GERSTNER-DEPTH): the breaker-held amplitude grows as ½γβ per
        // metre, so its horizontal orbit A·coth grows as ½γβ·coth; capping coth holds that at
        // `ORBIT_LIMIT` on any bed (the 1:20 proxy never reached it).
        let coth_cap = ORBIT_LIMIT / (0.5 * BREAKER_INDEX * beta);
        for (i, train) in trains.iter_mut().enumerate() {
            let (k, phi) = local_wave(omega[i], h, beta);
            let th = (k * h.max(MIN_DEPTH)).tanh();
            let a = (SHARES[i] * common).min(MICHE * th / k);
            let theta = -(phi + omega[i] * t + psi * PHASE_SPREAD[i]);
            let coth = (1.0 / th).min(coth_cap);
            steep += k * a * coth;
            *train = (a, k, coth, theta);
        }
        let q = (STEEPNESS_LIMIT / steep.max(1e-6)).min(1.0);
        let (mut y, mut along, mut dy, mut dalong) = (0.0, 0.0, 0.0, 0.0);
        for (a, k, coth, theta) in trains {
            let (s, c) = theta.sin_cos();
            y += a * s;
            along += q * a * coth * c;
            dy += a * k * c;
            dalong -= q * a * coth * k * s;
        }
        // Swash: the whole beach at the main train's shoreline phase. Stockdon's S ∝ β: `swash` is
        // at `beach_slope`, scaled to the local slope (the land margin, S/β, does not change).
        let (_, phi0) = local_wave(omega[0], 0.0, beta);
        let theta0 = -(phi0 + omega[0] * t + psi * PHASE_SPREAD[0]);
        let swash = self.swash
            * (beta / self.beach_slope.max(1e-3))
            * o
            * set
            * (1.0 - smoothstep(0.0, SWASH_ZONE * self.band, d))
            * smoothstep(-self.land_margin, -0.5 * self.land_margin, d);
        y += swash * (theta0 - SWASH_SKEW * theta0.sin()).sin();
        ShoreWave {
            offset: Vec3::new(dir.x * along, y, dir.y * along),
            slope: Vec4::new(dy * dir.x, dy * dir.y, dalong * dir.x * dir.x, dalong * dir.y * dir.y),
            crest: trains[0].3.sin(),
            steepness: q * steep,
        }
    }
}
