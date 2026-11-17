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
//! - **Shore foam gate** (WA-6): the band of shore foam follows the local wave energy, the trains'
//!   height over the `McCowan` limit squared ([`ShoreWave::breaking`], [`shore_foam_weight`]), as
//!   surf-zone foam follows breaking (Battjes & Janssen 1978's dissipation): none on calm or
//!   enclosed water, where the trains, scaled by openness, have nearly no height.
//! - **Breaker foam** (SF-1, [`surf_foam`], `--off surffoam` = the band above): foam born from
//!   the breaking ratio and trailing the main crest in phase (stateless roller wake), a swash edge
//!   from the swash phase, contact foam as a thin line plus lace, a look by age (research
//!   `surf-and-foam-2026-11-17.md` "Foam"; Crest's foam feather, bubbles and 3D lighting).
//!
//! Every period is a whole number of cycles per [`REPEAT_PERIOD`], so the GPU's `t mod T` clock
//! never pops. Mechanism prior art: the owner's own Godotwind shore swash (MIT,
//! `ocean_shore_common.gdshaderinc`, distance-to-shore-driven phase and envelopes); every constant
//! here is recalibrated from the physics above.
//!
//! **Surf shape** (SF-2, [`crate::SurfShape`], `--off surfskew` = the shape above unchanged): the
//! offshore height is raised by `surf_height_gain` and γ is tunable, and the breaking ratio before the
//! smooth min, `b`, sets a lifecycle per point ([`surf_life`]): shoaling (symmetric), steepening (the
//! phase is warped, `θ' = θ − s·sin θ`, for the height and the horizontal term alike, so the surface
//! stays a Lagrangian map with a steep shoreward face) and bore (more skew, a higher loop cap).
//! Outside the bore the loop budget counts the skew (`|∂θ'/∂θ| ≤ 1 + s`), and the skew a bed carries
//! is capped so its growth with shore distance stretches the map by ≤ [`SKEW_RATE_LIMIT`]: the map
//! never folds there; a world noise sampled at the point's shoreline foot
//! (constant along each wave ray, varying along the crest) masks the bore into patches.

// Numeric kernel: the physics symbols (d, o, h, k, t, ...) read as in the sources.
#![allow(clippy::many_single_char_names)]

use std::f32::consts::{PI, TAU};

use glam::{DVec2, Vec2, Vec3, Vec4};

use crate::shore::{ShoreSettings, SurfShape};
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
/// Seeds of the SF-2 break gap noise (two fields crossfaded over [`REPEAT_PERIOD`], as the sets).
pub const GAP_SEEDS: [u32; 2] = [4, 5];
/// The steepening's skew growth may stretch the horizontal map by at most this, `Q·Σ A·coth·|∂s/∂d|`
/// (SF-2): the skew a bed carries is capped to it ([`surf_life`]), as [`ORBIT_LIMIT`] caps `coth`.
/// With the phase term's [`STEEPNESS_LIMIT`] and the envelope's [`ORBIT_LIMIT`] (in quadrature, plus
/// Q's own response to the skew, also ≤ this): `√((0.75 + 0.1)² + (0.4 + 0.1)²) < 1`, no fold. A
/// bound, not a tunable.
pub const SKEW_RATE_LIMIT: f32 = 0.1; // tunables: const (a no-fold bound derived from the Jacobian, as ORBIT_LIMIT)

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
    /// The surf shape (SF-2) the trains are drawn with ([`ShoreSettings::surf_shape`]; its gain is
    /// already in [`Self::amplitude`]).
    pub surf: SurfShape,
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
    /// `cos` of the main train's phase: with [`Self::crest`], the phase the breaker foam trails
    /// (SF-1, [`surf_foam`]; crossfaded as a vector through a sea-state transition).
    pub crest_cos: f32,
    /// The main train's phase θ, rad (the dominant train set's through a transition): its whole
    /// cycles count the waves (the GPU varies the foam lace per wave).
    pub phase: f32,
    /// Swash (SF-1): (weight 0-1: openness × sets × the swash zone's fades, 0 without swash; the
    /// swash phase `g` rad: the water stands at `sin g` of its run-up, rising while `cos g < 0`).
    pub swash: Vec2,
    /// Σ Q·k·A·coth(kh) (≤ [`STEEPNESS_LIMIT`]).
    pub steepness: f32,
    /// How close the trains come to breaking here, 0-1: their shoaled amplitude over the
    /// depth-limited (`McCowan`) one at the local depth (held at [`MIN_DEPTH`] on the beach and in the
    /// swash), times the openness weight and the sets. ≈ 1 where an open sea's waves break, ≤ the
    /// openness on enclosed water, 0 without waves. The shore foam follows it ([`shore_foam_weight`]).
    pub breaking: f32,
    /// The surf lifecycle (SF-2, [`surf_life`]): (phase skew `s`, bore weight 0-1 with the break
    /// gaps applied). (0, 0) with `--off surfskew`.
    pub lifecycle: Vec2,
}

/// The SF-2 lifecycle at one point ([`surf_life`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfLife {
    /// Phase skew `s` (0 = symmetric; < 1).
    pub skew: f32,
    /// Bore weight 0-1, the break gaps applied (0 in a gap).
    pub bore: f32,
    /// `|∂s/∂d|` of the steepening part, 1/m (`orbit · skew_rate ≤` [`SKEW_RATE_LIMIT`]).
    pub skew_rate: f32,
}

/// The surf lifecycle (SF-2) at breaking ratio `b` (before the smooth min: shoaled over
/// depth-limited amplitude, × openness, band fade and sets), its shore-distance rate `db_dd` (1/m)
/// and the gap mask `open` (1 = breaks, 0 = a gap). Shoaling below `steepen_start` (no skew),
/// steepening to `bore_start`, the bore to `bore_full`. `orbit` (m) is the horizontal orbit the skew
/// warps, `Q₀·Σ A·coth` (Q₀ = the unskewed loop factor): the steepening's skew is capped so that its
/// fastest growth, `1.5·skew_steepen/span·|∂b/∂d|`, stretches the map by at most
/// [`SKEW_RATE_LIMIT`] (a steep bed packs the steepening into a few metres; the cap is the skew it
/// can carry without folding; 0 = no cap). The cap uses the steepening's peak rate, not the local
/// one, so it varies with the bed and the sea, not across the steepening (a Q reacting to the local
/// rate folded the map through its own gradient). The GPU twin is `surf_life` in `water_common.wesl`.
pub fn surf_life(s: &SurfShape, b: f32, db_dd: f32, open: f32, orbit: f32) -> SurfLife {
    let span = (s.bore_start - s.steepen_start).max(1e-3);
    let x = ((b - s.steepen_start) / span).clamp(0.0, 1.0);
    let steep_w = x * x * (3.0 - 2.0 * x);
    let bore_ramp = smoothstep(s.bore_start, s.bore_full.max(s.bore_start + 1e-3), b);
    let bore = bore_ramp * open.clamp(0.0, 1.0);
    let skew_steepen = s.skew_steepen.clamp(0.0, 0.9);
    let skew_bore = s.skew_bore.clamp(0.0, 0.9);
    let grow = orbit * skew_steepen * 1.5 / span * db_dd.abs();
    let lead = skew_steepen * (SKEW_RATE_LIMIT / grow.max(1e-12)).min(1.0);
    // The bore starts where the steepening ends (steep_w = 1): the steepening's skew lerped to the
    // bore's.
    let skew = lead * steep_w * (1.0 - bore) + skew_bore * bore;
    let skew_rate = lead * 6.0 * x * (1.0 - x) / span * db_dd.abs();
    SurfLife { skew, bore, skew_rate }
}

/// The envelope at the break line (SF-2's minimum visible crest): the smooth min `soft` of the
/// shoaled `shoal` and depth-limited `breaker` amplitudes (m), raised to the sharper min of
/// `break_sharpness` up to half `break_crest_min_m`. A max and min of smooth laws, so its growth with
/// shore distance is always one of theirs (the loop limits keep holding). The GPU twin is
/// `surf_envelope` in `water_common.wesl`.
pub fn surf_envelope(s: &SurfShape, shoal: f32, breaker: f32, soft: f32) -> f32 {
    let (lo, hi) = (shoal.min(breaker), shoal.max(breaker));
    let p = s.break_sharpness.clamp(1.0, 32.0);
    let sharp = lo / (1.0 + (lo / hi.max(1e-20)).max(1e-12).powf(p)).powf(1.0 / p);
    let raised = sharp.min(0.5 * s.break_crest_min_m.max(0.0));
    if raised > soft { raised } else { soft }
}

/// Share (0-1) of the shore foam band drawn at breaking ratio `breaking` ([`ShoreWave::breaking`];
/// with the shore waves off the GPU passes the openness × the body's swell scale): the band follows
/// the local wave energy (∝ H², so ∝ ratio²) and is full from `full`
/// ([`crate::ShoreSettings::foam_full_breaking`]). 0 where no waves reach the shore (WA-6: calm and
/// enclosed water). The GPU twin is `shore_foam_weight` in `water_common.wesl`.
pub fn shore_foam_weight(breaking: f32, full: f32) -> f32 {
    let r = (breaking / full.max(1e-3)).clamp(0.0, 1.0);
    if r.is_nan() { 0.0 } else { r * r }
}

/// One fragment's inputs to the breaker foam ([`surf_foam`]; SF-1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfFoamInput {
    /// [`ShoreWave::breaking`].
    pub breaking: f32,
    /// The main train's phase θ, rad ([`ShoreWave::phase`]; the GPU takes `atan2(crest, crest_cos)`).
    pub phase: f32,
    /// [`ShoreWave::swash`]: (weight, swash phase rad).
    pub swash: Vec2,
    /// Vertical water depth over the scene behind the surface, m (the snapshot depth's view path ×
    /// |V.y|); infinite where unknown.
    pub vertical: f32,
    /// The look's contact foam reach, m (0 = no contact foam).
    pub contact: f32,
    /// The cellular lace field `F2 − F1` here, cells (0 on cell borders).
    pub lace: f32,
    /// Value noise 0-1 varying the bore lace's width (per wave on the GPU).
    pub variation: f32,
    /// The look's shore-foam intensity 0-1 (scales the bore birth; 0 = none in this water).
    pub strength: f32,
    /// 1 = the lace resolved, 0 = its mean (the pixel's footprint covers its cells).
    pub detail: f32,
}

/// The breaker foam's terms at one fragment ([`surf_foam`]), each 0-1.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfFoamTerms {
    /// Bore foam: born where the trains break, trailing behind the main crest.
    pub bore: f32,
    /// Waterline foam: the contact line and swash lace.
    pub waterline: f32,
    /// Both, soft-combined (`1 − (1 − a)(1 − b)`).
    pub coverage: f32,
    /// Freshness 0 (aged: thin, translucent, body-tinted) to 1 (fresh: dense, bright).
    pub fresh: f32,
    /// The under-roller glow: the body's in-scatter is raised by this share.
    pub glow: f32,
    /// ∂coverage/∂lace (per cell of `F2 − F1`): with the lace field's gradient, the foam's relief
    /// slope for its normal.
    pub d_lace: f32,
}

/// Birth of bore foam at breaking ratio `breaking` (SF-1): 0 below `birth_start` (calm and enclosed
/// water stay clean, W-FOAM-GATE), full from `birth_full`.
pub fn surf_birth(breaking: f32, s: &crate::SurfFoam) -> f32 {
    let b = smoothstep(s.birth_start, s.birth_full.max(s.birth_start + 1e-3), breaking);
    if b.is_nan() { 0.0 } else { b }
}

/// The bore foam's trailing profile at phase `psi` (rad, 0-2π) measured from `front_rad` ahead of
/// the main crest: rises over the front feather, 1 on the crest (`psi = front_rad`), then
/// `exp(−Δφ / bore_decay_rad)` behind it.
pub fn bore_profile(psi: f32, s: &crate::SurfFoam) -> f32 {
    let front = s.front_rad.max(1e-3);
    smoothstep(0.0, front, psi) * (-(psi - front).max(0.0) / s.bore_decay_rad.max(1e-3)).exp()
}

/// Phase behind the main crest from the feather's start (rad, 0-2π) at the main train's phase `theta`.
pub fn behind_crest(theta: f32, s: &crate::SurfFoam) -> f32 {
    (0.5 * PI + s.front_rad.max(1e-3) - theta).rem_euclid(TAU)
}

/// (lace 0-1, ∂lace/∂e) of the cellular field `e` (`F2 − F1`, cells) for a lace `width` (cells)
/// and `feather`: foam where `e < width`, so a wide lace covers the cells and a narrow one leaves
/// thin lines on their borders and holes inside.
pub fn foam_lace(e: f32, width: f32, feather: f32) -> (f32, f32) {
    let f = feather.max(1e-3);
    let t = ((e - (width - f)) / (2.0 * f)).clamp(0.0, 1.0);
    (1.0 - t * t * (3.0 - 2.0 * t), -6.0 * t * (1.0 - t) / (2.0 * f))
}

/// The swash edge's strength 0-1 at swash `(weight, phase g)`: on through the uprush (age 0-π
/// from the lowest water, after the front feather), its residue fading through the backwash.
pub fn swash_edge(swash: Vec2, s: &crate::SurfFoam) -> f32 {
    let age = (1.5 * PI - swash.y).rem_euclid(TAU);
    let shape = if age < PI { smoothstep(0.0, s.front_rad.max(1e-3), age) } else { (-(age - PI) / s.swash_residue_rad.max(1e-3)).exp() };
    let e = swash.x.clamp(0.0, 1.0) * shape;
    if e.is_nan() { 0.0 } else { e }
}

/// The breaker foam (SF-1, [`crate::SurfFoam`]) at one fragment; `water_material.wesl` is the GPU
/// twin (same order of operations). Bore foam: `birth × profile × lace`, the lace widest on the
/// crest and narrowing behind it. Waterline foam (where the vertical depth is known and under the
/// contact reach): a thin bright line at the very edge plus a lace outward, raised from
/// `contact_calm` to full by the local turbulence (bore birth, swash edge), the reach widened by
/// the uprush. All terms 0 when off.
pub fn surf_foam(i: &SurfFoamInput, s: &crate::SurfFoam) -> SurfFoamTerms {
    if !s.enabled {
        return SurfFoamTerms::default();
    }
    let detail = i.detail.clamp(0.0, 1.0);
    let mix = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let birth = surf_birth(i.breaking, s) * i.strength.clamp(0.0, 1.0);
    let profile = bore_profile(behind_crest(i.phase, s), s);
    let var = 1.0 + s.lace_variation * (2.0 * i.variation.clamp(0.0, 1.0) - 1.0);
    let (bore_lace, bore_slope) = foam_lace(i.lace, mix(s.lace_aged, s.lace_fresh, profile) * var, s.feather);
    let bore = birth * profile * mix(profile, bore_lace, detail);
    let edge = swash_edge(i.swash, s);
    let (mut waterline, mut line_fresh, mut line_slope) = (0.0, 0.0, 0.0);
    if i.contact > 1e-3 && i.vertical.is_finite() {
        let tau = birth.max(edge);
        let amount = mix(s.contact_calm, 1.0, tau);
        let x = i.vertical.max(0.0) / (i.contact * (1.0 + s.swash_spread * edge));
        let line = 1.0 - smoothstep(0.0, s.contact_line.max(1e-3), x);
        let out = 1.0 - smoothstep(0.0, 1.0, x);
        let (edge_lace, edge_slope) = foam_lace(i.lace, mix(s.lace_aged, s.lace_fresh, out * tau), s.feather);
        let lace = out * mix(out, edge_lace, detail);
        waterline = amount * (1.0 - (1.0 - line) * (1.0 - lace));
        line_fresh = line + (1.0 - line) * tau * out;
        line_slope = amount * (1.0 - line) * out * detail * edge_slope;
    }
    let coverage = 1.0 - (1.0 - bore) * (1.0 - waterline);
    let fresh = (bore * profile + waterline * line_fresh) / (bore + waterline).max(1e-6);
    let terms = SurfFoamTerms {
        bore,
        waterline,
        coverage,
        fresh,
        glow: s.glow * birth * profile,
        d_lace: (1.0 - waterline) * birth * profile * detail * bore_slope + (1.0 - bore) * line_slope,
    };
    if [terms.bore, terms.waterline, terms.coverage, terms.fresh, terms.glow, terms.d_lace].iter().all(|v| v.is_finite()) { terms } else { SurfFoamTerms::default() }
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
        let physical = (2.0 * m0.max(0.0).sqrt()) as f32 * params.amplitude * strength;
        // SF-2: the surf gain raises the incident waves; the swash keeps the sea's own height.
        let amplitude = physical * settings.surf_shape.gain();
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
        let swash = 0.375 * beta * (2.0 * physical * l0).sqrt();
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
            surf: settings.surf_shape,
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
            crest_cos: from.crest_cos + (to.crest_cos - from.crest_cos) * w,
            phase: if w >= 0.5 { to.phase } else { from.phase },
            swash: Vec2::new(from.swash.x + (to.swash.x - from.swash.x) * w, if w >= 0.5 { to.swash.y } else { from.swash.y }),
            steepness: from.steepness + (to.steepness - from.steepness) * w,
            breaking: from.breaking + (to.breaking - from.breaking) * w,
            lifecycle: from.lifecycle.lerp(to.lifecycle, w),
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
            surf: to.surf,
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
        // γ: `BREAKER_INDEX` with `--off surfskew`, the tunable `breaker_gamma` otherwise (SF-2).
        let gamma = self.surf.gamma();
        let shoal = self.amplitude * (h_edge / h.max(MIN_DEPTH)).max(1.0).powf(0.25);
        let breaker = 0.5 * gamma * h;
        let env = shoal * breaker / (shoal.powi(4) + breaker.powi(4) + 1e-20).powf(0.25);
        // env / breaker, the breaker held at MIN_DEPTH so the beach and the swash keep the ratio.
        let breaker_r = 0.5 * gamma * h.max(MIN_DEPTH);
        let breaking = w * set * shoal / (shoal.powi(4) + breaker_r.powi(4) + 1e-20).powf(0.25);
        // The minimum visible crest at the break line (SF-2).
        let env = if self.surf.enabled { surf_envelope(&self.surf, shoal, breaker, env) } else { env };
        let common = env * w * set;
        let mut trains = [(0.0f32, 0.0f32, 0.0f32, 0.0f32); TRAINS];
        let mut steep = 0.0;
        let mut orbit = 0.0;
        // The orbit's envelope term (W-GERSTNER-DEPTH): the breaker-held amplitude grows as ½γβ per
        // metre, so its horizontal orbit A·coth grows as ½γβ·coth; capping coth holds that at
        // `ORBIT_LIMIT` on any bed (the 1:20 proxy never reached it).
        let coth_cap = ORBIT_LIMIT / (0.5 * gamma * beta);
        for (i, train) in trains.iter_mut().enumerate() {
            let (k, phi) = local_wave(omega[i], h, beta);
            let th = (k * h.max(MIN_DEPTH)).tanh();
            let a = (SHARES[i] * common).min(MICHE * th / k);
            let theta = -(phi + omega[i] * t + psi * PHASE_SPREAD[i]);
            let coth = (1.0 / th).min(coth_cap);
            steep += k * a * coth;
            orbit += a * coth;
            *train = (a, k, coth, theta);
        }
        // The lifecycle (SF-2), its skew capped by the orbit it warps (unskewed Q₀; outside the bore
        // Q ≤ Q₀, so the skew's growth stretches the map by ≤ SKEW_RATE_LIMIT).
        let life = if self.surf.enabled {
            let orbit = orbit * (STEEPNESS_LIMIT / steep.max(1e-6)).min(1.0);
            self.life(d, o, h, w, set, shoal, breaker_r, beta, dir, noise, fade, orbit)
        } else {
            SurfLife::default()
        };
        // The loop budget (SF-2): the skewed phase steepens the horizontal map by up to 1 + s (its
        // growth is held by the cap above); the bore raises the cap to `bore_loop`. All zero when
        // off: the old `STEEPNESS_LIMIT / steep` exactly.
        let s = life.skew;
        let limit = STEEPNESS_LIMIT + (self.surf.bore_loop - STEEPNESS_LIMIT) * life.bore;
        let budget = steep * (1.0 + s * (1.0 - life.bore));
        let q = (limit / budget.max(1e-6)).min(1.0);
        let (mut y, mut along, mut dy, mut dalong) = (0.0, 0.0, 0.0, 0.0);
        // The main train's skewed phase: the visible crest the foam follows.
        let main = trains[0].3 - s * trains[0].3.sin();
        for (a, k, coth, theta) in trains {
            // The skewed phase θ' = θ − s·sin θ (θ itself when s = 0), its derivative 1 − s·cos θ.
            let (s0, c0) = theta.sin_cos();
            let warped = theta - s * s0;
            let (sn, cs) = warped.sin_cos();
            let m = 1.0 - s * c0;
            y += a * sn;
            along += q * a * coth * cs;
            dy += a * k * cs * m;
            dalong -= q * a * coth * k * sn * m;
        }
        // Swash: the whole beach at the main train's shoreline phase. Stockdon's S ∝ β: `swash` is
        // at `beach_slope`, scaled to the local slope (the land margin, S/β, does not change).
        let (_, phi0) = local_wave(omega[0], 0.0, beta);
        let theta0 = -(phi0 + omega[0] * t + psi * PHASE_SPREAD[0]);
        let swash_w = o * set * (1.0 - smoothstep(0.0, SWASH_ZONE * self.band, d)) * smoothstep(-self.land_margin, -0.5 * self.land_margin, d);
        let swash = self.swash * (beta / self.beach_slope.max(1e-3)) * swash_w;
        let g = theta0 - SWASH_SKEW * theta0.sin();
        y += swash * g.sin();
        ShoreWave {
            offset: Vec3::new(dir.x * along, y, dir.y * along),
            slope: Vec4::new(dy * dir.x, dy * dir.y, dalong * dir.x * dir.x, dalong * dir.y * dir.y),
            crest: main.sin(),
            crest_cos: main.cos(),
            phase: main,
            swash: Vec2::new(if self.swash > 0.0 { swash_w } else { 0.0 }, g),
            steepness: q * steep,
            breaking,
            lifecycle: Vec2::new(life.skew, life.bore),
        }
    }

    /// The SF-2 lifecycle at shore distance `d` (the inputs of [`Self::eval_trains`]): the breaking
    /// ratio before the smooth min and its rate, the gap mask at the point's shoreline foot
    /// (`p + d·dir`: constant along the wave's ray, so a patch breaks all the way in, varying along
    /// the crest), the skew cap's `orbit` ([`surf_life`]).
    #[allow(clippy::too_many_arguments, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn life(&self, d: f32, o: f32, h: f32, w: f32, set: f32, shoal: f32, breaker_r: f32, beta: f32, dir: Vec2, noise: Vec2, fade: f32, orbit: f32) -> SurfLife {
        let ratio = shoal / breaker_r.max(1e-20);
        let b = w * set * ratio;
        // ∂b/∂d: shoal ∝ h^-¼ and breaker ∝ h above MIN_DEPTH (b ∝ d^-5/4), and the band fade
        // w = o·(1 − smoothstep(0, band, d)). Shoreward of MIN_DEPTH the depth part is held at its
        // value there (an over-estimate, but continuous: the skew cap must not jump).
        let x = (d / self.band).clamp(0.0, 1.0);
        let depth_rate = 1.25 * beta / h.max(MIN_DEPTH);
        let db_dd = b * depth_rate + o * set * ratio * 6.0 * x * (1.0 - x) / self.band;
        let period = NOISE_PERIOD as f32;
        let foot = Vec2::new((noise.x + d * dir.x / period).rem_euclid(1.0), (noise.y + d * dir.y / period).rem_euclid(1.0));
        let cells = self.surf.gap_cells() as u32;
        let g1 = value_noise(foot, cells, GAP_SEEDS[0]);
        let gaps = g1 + (value_noise(foot, cells, GAP_SEEDS[1]) - g1) * fade;
        let feather = self.surf.gap_feather.max(1e-3);
        let open = smoothstep(self.surf.gap_share - feather, self.surf.gap_share + feather, gaps);
        let open = 1.0 - self.variation * (1.0 - open);
        surf_life(&self.surf, b, db_dd, open, orbit)
    }
}
