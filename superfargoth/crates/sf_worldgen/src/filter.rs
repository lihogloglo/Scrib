//! **Runtime detail** (worldgen.md "Pipeline" stage 7): a stateless, analytic, gradient-aligned erosion filter that
//! adds gully and ridge detail below the baked grid. Every output point is a pure function of its world position,
//! the base height field around it and the declared parameters, so it can run in a shader with this module as the
//! CPU twin (height queries, physics, nav). Nothing is stored: a bake writes guide v1 grids only for previews.
//!
//! # Technique
//!
//! Sources (written descriptions and papers only; worldgen.md "Reuse check": the MPL-2.0 reference code is not
//! read or copied, this is a reimplementation):
//!
//! - Rune Skovbo Johansen ("runevision"), *Advanced Terrain Erosion Filter* (2025): write-up, blog post and
//!   video. The idea taken: a filter, not a simulation. Stripe noise whose stripes run **down the slope** of the
//!   input height, layered in octaves where each octave is oriented by the gradient of the input **plus the
//!   octaves above it** (the cascade), so small gullies branch off the walls of larger ones; strength fades where
//!   the slope gives no direction (flat ground) and around ridge and valley lines, so those stay clean. It extends
//!   the earlier "erosion noise" lineage (gradient-noise fBm whose octaves use the analytic derivative of the sum
//!   so far, as in Quilez's derivative-aware noise write-ups, and shader "erosion" filters built on it).
//! - Grenier, Guérin, Galin et al., "Real-time terrain enhancement with controlled procedural patterns",
//!   Computer Graphics Forum 43, 2024: per-point evaluation of oriented procedural patterns (phasor noise) aligned
//!   with the terrain gradient, with a periodic profile shaping crests and channels and amplitude driven by local
//!   terrain attributes.
//! - Tricard, Efremov, Zanni, Neyret, Martínez, Lefebvre, "Procedural phasor noise", ACM TOG 38(4), SIGGRAPH 2019:
//!   the stripe primitive. Kernels sum as complex phasors, the phase of the sum goes through a profile, so the
//!   contrast stays constant (no washed-out patches where kernels interfere, the classic Gabor-noise flaw).
//! - Jarzynski & Olano, "Hash functions for GPU rendering", JCGT 9(3), 2020: `pcg3d`, a 32-bit integer hash with
//!   identical CPU and WGSL results.
//!
//! Our formulation (all names are ours):
//!
//! 1. **Base.** Height `h₀` and gradient `∇h₀` from the Catmull-Rom bicubic of the guide ([`crate::resample`]),
//!    the same surface a plain resample gives (C1, so the gradient is continuous). Plan curvature `κ` (curvature
//!    of the contour lines, 1/m) from the cubic B-spline of the same taps: the B-spline is C2, so `κ` and every
//!    fade built on it are continuous, where Catmull-Rom second derivatives jump at cell edges.
//! 2. **Amplitude.** Local relief over one base wavelength is `|∇h₀| λ₀`; the detail amplitude is
//!    `A = min(relief_fraction · |∇h₀| · λ₀, max_amplitude_m, shore_ratio · max(h₀ − sea, 0))`. The shore term
//!    keeps every land point above sea level and every sea point unchanged, so the filter can never create or drain
//!    water (worldgen.md "Stage rules").
//! 3. **Octaves** `k = 0..octaves`: wavelength `λₖ = λ₀ / lacunarityᵏ`, weight `wₖ = gainᵏ / Σ gainʲ` (Σ wₖ = 1).
//!    Direction: `g` = gradient of the base plus the octaves so far (the cascade); the stripes vary across the
//!    slope, along `d = perp(g / |g|)`, so the crests and channels run downhill.
//! 4. **Phasor noise** in cells of size `λₖ`, one jittered kernel per cell, compact bump kernel
//!    `(1 − r²/R²)³` with `R` = [`KERNEL_RADIUS_CELLS`], over the 5 × 5 cells around the point (enough: every kernel
//!    reaching the point is there). Each kernel carries the plane wave `exp(i (2π d·(x − xᵢ)/λₖ + φᵢ))` with a random
//!    phase `φᵢ`. The phase of the sum is `θ`; its magnitude ratio `m = |Σ| / Σ aᵢ` (1 = coherent) fades the octave
//!    out near the phase singularities (`smoothstep(0, phase_fade, m)`), which keeps the output continuous there.
//! 5. **Profile** `p(θ) = (mix(cos θ, 1 − 2|sin(θ/2)|, crest_sharpness) − carve) / (1 + carve)`, in [-1, 1]: the
//!    sharp term gives knife-edge crests between rounded channel floors; `carve` biases the detail downward
//!    (erosion removes material; `carve` = 1 only lowers).
//! 6. **Fades** per octave: `smoothstep(slope_fade_lo, slope_fade_hi, |g|)` (no direction on flat ground) times
//!    `1 − smoothstep(curvature_fade_lo, curvature_fade_hi, |κ| λₖ)` (contours bending within one wavelength:
//!    ridge and valley lines).
//! 7. **Sum** `Δh = Σ A wₖ fadeₖ mₖ p(θₖ)`, so `|Δh| ≤ A ≤ max_amplitude_m`. The cascade adds
//!    `A wₖ fadeₖ mₖ p′(θₖ) (2π / λₖ) d` to `g` (the plane-wave part of the phase gradient; the envelope part is
//!    dropped, as the derivative-noise lineage does).
//!
//! Precision: world positions stay f64 only to split them into an integer cell and an f32 offset; everything
//! after is f32 in a fixed summation order, so results never depend on threads or tiles (runtime detail never feeds
//! saves or nav bakes; the GPU twin will receive the same cell + offset split).

use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

use crate::resample::{bicubic_with, bspline_weights, catmull_rom_weights};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_filter.rs"]
mod tests;

/// Kernel radius in cells. Layout constant: one jittered kernel per cell lies within √2 cells of any point of its
/// cell, so a radius ≥ √2 always covers the point (the weight sum is never 0), and a radius ≤ 1.5 means kernels
/// beyond the 5 × 5 neighbourhood never reach it.
// tunables: const (layout: coverage needs ≥ √2 cells, the 5 × 5 search allows ≤ 1.5)
pub const KERNEL_RADIUS_CELLS: f32 = 1.5;

/// Output size above which `detail` warns (samples). Not a limit: a reminder that a whole-guide run at metre
/// spacing is gigabytes.
// tunables: const (report layout: a warning threshold, ~800 MB of f32 heights)
pub const WARN_SAMPLES: usize = 200_000_000;

const TAU: f32 = std::f32::consts::TAU;

/// One declared filter value (spec `tunables.md` "Definition": value, unit, range, source).
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Declared {
    pub name: &'static str,
    pub v: f64,
    pub unit: &'static str,
    pub min: f64,
    pub max: f64,
    pub source: String,
    pub help: &'static str,
}

fn decl(
    name: &'static str,
    v: f64,
    unit: &'static str,
    min: f64,
    max: f64,
    source: &str,
    help: &'static str,
) -> Declared {
    Declared {
        name,
        v,
        unit,
        min,
        max,
        source: source.into(),
        help,
    }
}

/// The declared filter values with their defaults.
pub fn declared_defaults() -> Vec<Declared> {
    vec![
        decl("octaves", 4.0, "count", 0.0, 8.0, "Guess",
            "stripe octaves; 160 m to 20 m at the defaults, the smallest still 4 samples wide at 5 m"),
        decl("wavelength_m", 160.0, "m", 4.0, 4000.0, "Guess",
            "wavelength of the largest gullies: a few 20 m macro cells, so it details rather than reshapes the form"),
        decl("lacunarity", 2.0, "ratio", 1.25, 4.0, "Industry(fBm lacunarity 2, Musgrave et al. 1989)",
            "wavelength ratio between octaves"),
        decl("gain", 0.5, "ratio", 0.05, 0.95, "Guess",
            "amplitude ratio between octaves; 0.5 with lacunarity 2 keeps the gully wall slope equal per octave"),
        decl("relief_fraction", 0.25, "ratio", 0.0, 2.0, "Guess",
            "amplitude as a fraction of the local relief |grad h| x wavelength_m"),
        decl("max_amplitude_m", 20.0, "m", 0.0, 500.0, "Guess",
            "cap on the amplitude, and so on |dh| anywhere"),
        decl("slope_fade_lo", 0.03, "tan", 0.0, 5.0, "Guess",
            "slope (rise over run) below which an octave is off: flat ground has no downhill direction"),
        decl("slope_fade_hi", 0.15, "tan", 0.0, 5.0, "Guess",
            "slope above which an octave is at full strength"),
        decl("curvature_fade_lo", 0.5, "ratio", 0.0, 100.0, "Guess",
            "contour curvature x octave wavelength above which the octave starts fading (ridge and valley lines)"),
        decl("curvature_fade_hi", 2.0, "ratio", 0.0, 100.0, "Guess",
            "contour curvature x octave wavelength at which the octave is off"),
        decl("crest_sharpness", 0.5, "ratio", 0.0, 1.0, "Guess",
            "0 = sinusoidal stripes, 1 = knife-edge crests between rounded channel floors"),
        decl("carve", 0.5, "ratio", 0.0, 1.0, "Guess",
            "downward bias: 0 = symmetric, 1 = the filter only removes material"),
        decl("phase_fade", 0.3, "ratio", 0.001, 1.0, "Guess",
            "phasor coherence below which an octave fades (continuity at phase singularities)"),
        decl("shore_ratio", 0.5, "ratio", 0.0, 0.99, "Guess",
            "amplitude cap as a fraction of the height above sea level: keeps the wet/dry class"),
        decl("cascade", 1.0, "switch", 0.0, 1.0, "Reference(runevision 2025, octaves follow the accumulated gradient)",
            "1 = each octave follows the gradient of the base plus the octaves above it (branching); 0 = base gradient"),
        decl("seed", 1.0, "id", 0.0, 4_294_967_295.0, "Guess", "hash seed (u32)"),
    ]
}

/// Filter parameters, built from the declared values ([`FilterParams::from_declared`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilterParams {
    pub octaves: u32,
    pub wavelength_m: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub relief_fraction: f32,
    pub max_amplitude_m: f32,
    pub slope_fade: [f32; 2],
    pub curvature_fade: [f32; 2],
    pub crest_sharpness: f32,
    pub carve: f32,
    pub phase_fade: f32,
    pub shore_ratio: f32,
    pub cascade: bool,
    pub seed: u32,
}

impl Default for FilterParams {
    fn default() -> Self {
        Self::from_declared(&declared_defaults()).expect("defaults are in range")
    }
}

impl FilterParams {
    /// Builds the parameters, checking every value against its declared range and the fade pairs' order.
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("filter value {name} not declared")))?;
            if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
                Ok(p.v)
            } else {
                Err(Error::Invalid(format!(
                    "{name} = {} outside [{}, {}]",
                    p.v, p.min, p.max
                )))
            }
        };
        let integer = |name: &str| -> Result<f64> {
            let v = get(name)?;
            if v.fract() == 0.0 {
                Ok(v)
            } else {
                Err(Error::Invalid(format!("{name} = {v} must be an integer")))
            }
        };
        let f = |name: &str| get(name).map(|v| v as f32);
        let p = Self {
            octaves: integer("octaves")? as u32,
            wavelength_m: f("wavelength_m")?,
            lacunarity: f("lacunarity")?,
            gain: f("gain")?,
            relief_fraction: f("relief_fraction")?,
            max_amplitude_m: f("max_amplitude_m")?,
            slope_fade: [f("slope_fade_lo")?, f("slope_fade_hi")?],
            curvature_fade: [f("curvature_fade_lo")?, f("curvature_fade_hi")?],
            crest_sharpness: f("crest_sharpness")?,
            carve: f("carve")?,
            phase_fade: f("phase_fade")?,
            shore_ratio: f("shore_ratio")?,
            cascade: integer("cascade")? != 0.0,
            seed: integer("seed")? as u32,
        };
        if p.slope_fade[0] >= p.slope_fade[1] || p.curvature_fade[0] >= p.curvature_fade[1] {
            return Err(Error::Invalid(
                "slope_fade_lo < slope_fade_hi and curvature_fade_lo < curvature_fade_hi required".into(),
            ));
        }
        Ok(p)
    }
}

/// Applies overrides from a small TOML file: `name = number` or `name = { v = number, source = "..." }`. A plain
/// number takes the source `Guess`.
pub fn apply_overrides_toml(d: &mut [Declared], text: &str) -> Result<()> {
    let table: toml::Table = toml::from_str(text)?;
    for (name, value) in &table {
        let (v, source) = match value {
            toml::Value::Integer(i) => (*i as f64, None),
            toml::Value::Float(x) => (*x, None),
            toml::Value::Table(t) => {
                let v = match t.get("v") {
                    Some(toml::Value::Integer(i)) => *i as f64,
                    Some(toml::Value::Float(x)) => *x,
                    _ => return Err(Error::Invalid(format!("{name}: table needs a numeric v"))),
                };
                let source = t.get("source").and_then(toml::Value::as_str).map(str::to_owned);
                if let Some(s) = &source {
                    crate::recipe::check_source(name, s)?;
                }
                (v, source)
            }
            _ => return Err(Error::Invalid(format!("{name}: number or {{ v, source }} expected"))),
        };
        set(d, name, v, source)?;
    }
    Ok(())
}

/// Sets one declared value (`source` defaults to `Guess`).
pub fn set(d: &mut [Declared], name: &str, v: f64, source: Option<String>) -> Result<()> {
    let p = d
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| Error::Invalid(format!("unknown filter value {name}")))?;
    p.v = v;
    p.source = source.unwrap_or_else(|| "Guess".into());
    Ok(())
}

/// The base surface at one point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaseSample {
    /// Catmull-Rom height, metres.
    pub h: f64,
    /// Catmull-Rom gradient (∂h/∂x east, ∂h/∂y north), dimensionless.
    pub grad: [f32; 2],
    /// Plan (contour) curvature of the cubic B-spline, 1/m; 0 where the B-spline is flat.
    pub plan_curvature: f32,
}

/// A guide height grid seen as a continuous surface. Row 0 is the north edge; sample (col, row) sits at world
/// `(origin.x + col·spacing, origin.y − row·spacing)`.
#[derive(Debug, Clone, Copy)]
pub struct BaseField<'a> {
    pub w: usize,
    pub h: usize,
    pub spacing_m: f64,
    pub origin_m: [f64; 2],
    pub heights: &'a [f32],
}

impl BaseField<'_> {
    #[inline]
    fn at(&self, c: isize, r: isize) -> f64 {
        let x = c.clamp(0, self.w as isize - 1).cast_unsigned();
        let y = r.clamp(0, self.h as isize - 1).cast_unsigned();
        f64::from(self.heights[y * self.w + x])
    }

    /// Catmull-Rom height at fractional sample position (col, row): exactly [`bicubic_with`] on the grid.
    pub fn height(&self, col: f64, row: f64) -> f64 {
        bicubic_with(col, row, |c, r| self.at(c, r))
    }

    /// The base at fractional sample position (col, row), edges extended.
    pub fn sample(&self, col: f64, row: f64) -> BaseSample {
        let h = self.height(col, row);
        let (cf, rf) = (col.floor(), row.floor());
        let (c0, r0) = (cf as isize - 1, rf as isize - 1);
        let (tx, ty) = (col - cf, row - rf);
        let mut taps = [[0.0f64; 4]; 4];
        for (j, row_taps) in taps.iter_mut().enumerate() {
            for (i, t) in row_taps.iter_mut().enumerate() {
                *t = self.at(c0 + i as isize, r0 + j as isize);
            }
        }
        // Catmull-Rom gradient in sample units.
        let (cx, cy) = (catmull_rom_weights(tx), catmull_rom_weights(ty));
        let (dcx, dcy) = (catmull_rom_derivative(tx), catmull_rom_derivative(ty));
        let (mut d_col, mut d_row) = (0.0, 0.0);
        // B-spline value derivatives (first and second) in sample units.
        let (bx, by) = (bspline_weights(tx), bspline_weights(ty));
        let (b1x, b1y) = (bspline_derivative(tx), bspline_derivative(ty));
        let (b2x, b2y) = (bspline_second(tx), bspline_second(ty));
        let (mut sc, mut sr, mut scc, mut scr, mut srr) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for j in 0..4 {
            for i in 0..4 {
                let t = taps[j][i];
                d_col += dcx[i] * cy[j] * t;
                d_row += cx[i] * dcy[j] * t;
                sc += b1x[i] * by[j] * t;
                sr += bx[i] * b1y[j] * t;
                scc += b2x[i] * by[j] * t;
                scr += b1x[i] * b1y[j] * t;
                srr += bx[i] * b2y[j] * t;
            }
        }
        let s = self.spacing_m;
        // World axes: x = +col, y = −row.
        let grad = [(d_col / s) as f32, (-d_row / s) as f32];
        let (hx, hy) = (sc / s, -sr / s);
        let (hxx, hxy, hyy) = (scc / (s * s), -scr / (s * s), srr / (s * s));
        let g2 = hx * hx + hy * hy;
        let plan_curvature = if g2 > 1e-18 {
            let k = (hxx * hy * hy - 2.0 * hxy * hx * hy + hyy * hx * hx) / (g2 * g2.sqrt());
            k.clamp(-1e6, 1e6) as f32
        } else {
            0.0
        };
        BaseSample {
            h,
            grad,
            plan_curvature,
        }
    }
}

/// Derivatives of [`catmull_rom_weights`] with respect to `t`.
#[inline]
pub fn catmull_rom_derivative(t: f64) -> [f64; 4] {
    let t2 = t * t;
    [
        -1.5 * t2 + 2.0 * t - 0.5,
        4.5 * t2 - 5.0 * t,
        -4.5 * t2 + 4.0 * t + 0.5,
        1.5 * t2 - t,
    ]
}

/// First derivatives of [`bspline_weights`] with respect to `t`.
#[inline]
fn bspline_derivative(t: f64) -> [f64; 4] {
    let u = 1.0 - t;
    let t2 = t * t;
    [
        -0.5 * u * u,
        1.5 * t2 - 2.0 * t,
        -1.5 * t2 + t + 0.5,
        0.5 * t2,
    ]
}

/// Second derivatives of [`bspline_weights`] with respect to `t`.
#[inline]
fn bspline_second(t: f64) -> [f64; 4] {
    [1.0 - t, 3.0 * t - 2.0, 1.0 - 3.0 * t, t]
}

#[inline]
fn smoothstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// `pcg3d` (Jarzynski & Olano 2020): three well-mixed u32 from three u32.
#[inline]
fn pcg3d(mut x: u32, mut y: u32, mut z: u32) -> [u32; 3] {
    x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    y = y.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    z = z.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    x = x.wrapping_add(y.wrapping_mul(z));
    y = y.wrapping_add(z.wrapping_mul(x));
    z = z.wrapping_add(x.wrapping_mul(y));
    x ^= x >> 16;
    y ^= y >> 16;
    z ^= z >> 16;
    x = x.wrapping_add(y.wrapping_mul(z));
    y = y.wrapping_add(z.wrapping_mul(x));
    z = z.wrapping_add(x.wrapping_mul(y));
    [x, y, z]
}

/// u32 → [0, 1) with 24 bits.
#[inline]
fn unit(v: u32) -> f32 {
    (v >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// Phasor noise of wavelength `lambda_m` at world point `p`, stripes varying along the unit vector `dir`.
/// Returns (phase in (-π, π], coherence in [0, 1]).
fn phasor(p: [f64; 2], lambda_m: f32, dir: [f32; 2], seed: u32) -> (f32, f32) {
    let l = f64::from(lambda_m);
    let (ux, uy) = (p[0] / l, p[1] / l);
    let (fx0, fy0) = (ux.floor(), uy.floor());
    let (ix, iy) = (fx0 as i64, fy0 as i64);
    let (fx, fy) = ((ux - fx0) as f32, (uy - fy0) as f32);
    let inv_r2 = 1.0 / (KERNEL_RADIUS_CELLS * KERNEL_RADIUS_CELLS);
    let (mut re, mut im, mut wsum) = (0.0f32, 0.0f32, 0.0f32);
    for dy in -2i32..=2 {
        for dx in -2i32..=2 {
            let cx = (ix + i64::from(dx)) as u32;
            let cy = (iy + i64::from(dy)) as u32;
            let [hx, hy, hp] = pcg3d(cx, cy, seed);
            let ox = fx - (dx as f32 + unit(hx));
            let oy = fy - (dy as f32 + unit(hy));
            let r2 = (ox * ox + oy * oy) * inv_r2;
            if r2 >= 1.0 {
                continue;
            }
            let b = 1.0 - r2;
            let a = b * b * b;
            let th = TAU * (ox * dir[0] + oy * dir[1] + unit(hp));
            re += a * th.cos();
            im += a * th.sin();
            wsum += a;
        }
    }
    let mag = (re * re + im * im).sqrt();
    let coherence = if wsum > 0.0 { (mag / wsum).min(1.0) } else { 0.0 };
    (im.atan2(re), coherence)
}

/// Profile value in [-1, 1] and its derivative with respect to the phase.
#[inline]
fn profile(th: f32, sharp: f32, carve: f32) -> (f32, f32) {
    let (s2, c2) = (0.5 * th).sin_cos();
    let smooth = th.cos();
    let d_smooth = -th.sin();
    let edge = 1.0 - 2.0 * s2.abs();
    let d_edge = -s2.signum() * c2;
    let k = 1.0 / (1.0 + carve);
    (
        (smooth + (edge - smooth) * sharp - carve) * k,
        (d_smooth + (d_edge - d_smooth) * sharp) * k,
    )
}

/// The detail offset Δh (metres) at world point `p` (x east, y north) over `base`. `|Δh| ≤ max_amplitude_m`,
/// 0 on flat ground and at or below `sea_level_m`.
pub fn detail_offset(p: [f64; 2], base: &BaseSample, sea_level_m: f64, prm: &FilterParams) -> f32 {
    let g0 = base.grad;
    let slope0 = (g0[0] * g0[0] + g0[1] * g0[1]).sqrt();
    let shore = ((base.h - sea_level_m).max(0.0) as f32) * prm.shore_ratio;
    let amp = (prm.relief_fraction * slope0 * prm.wavelength_m)
        .min(prm.max_amplitude_m)
        .min(shore);
    if prm.octaves == 0 || amp.is_nan() || amp <= 0.0 {
        return 0.0;
    }
    let mut wsum = 0.0f32;
    let mut wk = 1.0f32;
    for _ in 0..prm.octaves {
        wsum += wk;
        wk *= prm.gain;
    }
    let curv = base.plan_curvature.abs();
    let mut g = g0;
    let mut dh = 0.0f32;
    let mut lambda = prm.wavelength_m;
    let mut w = 1.0 / wsum;
    for k in 0..prm.octaves {
        let s = (g[0] * g[0] + g[1] * g[1]).sqrt();
        let fade = smoothstep(prm.slope_fade[0], prm.slope_fade[1], s)
            * (1.0 - smoothstep(prm.curvature_fade[0], prm.curvature_fade[1], curv * lambda));
        if fade > 0.0 && s > 0.0 {
            let dir = [-g[1] / s, g[0] / s];
            let seed = prm.seed ^ k.wrapping_mul(0x9e37_79b9);
            let (th, coherence) = phasor(p, lambda, dir, seed);
            let a = amp * w * fade * smoothstep(0.0, prm.phase_fade, coherence);
            let (v, dv) = profile(th, prm.crest_sharpness, prm.carve);
            dh += a * v;
            if prm.cascade {
                let slope = a * dv * TAU / lambda;
                g = [g[0] + slope * dir[0], g[1] + slope * dir[1]];
            }
        }
        lambda /= prm.lacunarity;
        w *= prm.gain;
    }
    dh
}

/// An output lattice: samples at `spacing_m`, anchored at the base's sample (0, 0), so a sample's world position
/// (and its base position) depends only on its global indices: tiles cut from one lattice share edges bitwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lattice {
    /// Global column and row of the first output sample.
    pub col0: usize,
    pub row0: usize,
    pub w: usize,
    pub h: usize,
}

impl Lattice {
    /// The lattice over the base's whole extent, or over `crop_m = [x0, y0, x1, y1]` (world metres, x east,
    /// y north), clamped to the base.
    pub fn new(base: &BaseField<'_>, spacing_m: f64, crop_m: Option<[f64; 4]>) -> Result<Self> {
        if !(spacing_m.is_finite() && spacing_m > 0.0) {
            return Err(Error::Invalid("spacing must be > 0".into()));
        }
        let max_c = (((base.w - 1) as f64) * base.spacing_m / spacing_m).floor() as i64;
        let max_r = (((base.h - 1) as f64) * base.spacing_m / spacing_m).floor() as i64;
        let (c0, c1, r0, r1) = match crop_m {
            None => (0, max_c, 0, max_r),
            Some([x0, y0, x1, y1]) => {
                if !(x0 < x1 && y0 < y1) {
                    return Err(Error::Invalid("crop needs x0 < x1 and y0 < y1".into()));
                }
                let [ox, oy] = base.origin_m;
                (
                    (((x0 - ox) / spacing_m).ceil() as i64).max(0),
                    (((x1 - ox) / spacing_m).floor() as i64).min(max_c),
                    (((oy - y1) / spacing_m).ceil() as i64).max(0),
                    (((oy - y0) / spacing_m).floor() as i64).min(max_r),
                )
            }
        };
        if c1 < c0 || r1 < r0 {
            return Err(Error::Invalid("crop misses the guide".into()));
        }
        Ok(Self {
            col0: c0 as usize,
            row0: r0 as usize,
            w: (c1 - c0 + 1) as usize,
            h: (r1 - r0 + 1) as usize,
        })
    }

    /// Sample count.
    pub fn len(&self) -> usize {
        self.w * self.h
    }

    /// True when empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Base heights and filtered heights of a lattice (f32, row-major, row 0 = north). Rows run in parallel; each
/// sample is a pure function of its global indices.
pub fn evaluate(
    base: &BaseField<'_>,
    spacing_m: f64,
    lat: &Lattice,
    sea_level_m: f64,
    prm: &FilterParams,
) -> (Vec<f32>, Vec<f32>) {
    let mut base_out = vec![0.0f32; lat.len()];
    let mut out = vec![0.0f32; lat.len()];
    base_out
        .par_chunks_mut(lat.w)
        .zip(out.par_chunks_mut(lat.w))
        .enumerate()
        .for_each(|(j, (brow, orow))| {
            let r = (lat.row0 + j) as f64 * spacing_m;
            for (i, (b, o)) in brow.iter_mut().zip(orow.iter_mut()).enumerate() {
                let c = (lat.col0 + i) as f64 * spacing_m;
                let s = base.sample(c / base.spacing_m, r / base.spacing_m);
                let p = [base.origin_m[0] + c, base.origin_m[1] - r];
                let hb = s.h as f32;
                *b = hb;
                *o = hb + detail_offset(p, &s, sea_level_m, prm);
            }
        });
    (base_out, out)
}

/// Height-change statistics of one run.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct DeltaStats {
    pub mean_abs_m: f64,
    pub p99_abs_m: f64,
    pub max_abs_m: f64,
    /// Mean signed change, metres (negative: the filter removed material on average).
    pub mean_m: f64,
    /// Share of samples changed at all.
    pub changed_share: f64,
    /// Samples whose wet/dry class (below sea level = wet) differs from the base.
    pub new_wet: usize,
    pub new_dry: usize,
}

/// Statistics of `out − base`.
pub fn delta_stats(base: &[f32], out: &[f32], sea_level_m: f64) -> DeltaStats {
    let sea = sea_level_m as f32;
    let mut abs: Vec<f32> = base.iter().zip(out).map(|(b, o)| (o - b).abs()).collect();
    let n = abs.len().max(1) as f64;
    let (mut sum_abs, mut sum, mut changed, mut new_wet, mut new_dry) = (0.0f64, 0.0f64, 0usize, 0, 0);
    for (b, o) in base.iter().zip(out) {
        let d = f64::from(o - b);
        sum_abs += d.abs();
        sum += d;
        changed += usize::from(o.to_bits() != b.to_bits());
        new_wet += usize::from(*b >= sea && *o < sea);
        new_dry += usize::from(*b < sea && *o >= sea);
    }
    let max_abs = abs.iter().copied().fold(0.0f32, f32::max);
    let p99 = if abs.is_empty() {
        0.0
    } else {
        let k = ((abs.len() - 1) as f64 * 0.99).round() as usize;
        *abs.select_nth_unstable_by(k, f32::total_cmp).1
    };
    DeltaStats {
        mean_abs_m: sum_abs / n,
        p99_abs_m: f64::from(p99),
        max_abs_m: f64::from(max_abs),
        mean_m: sum / n,
        changed_share: changed as f64 / n,
        new_wet,
        new_dry,
    }
}

/// One `scrib-worldgen detail` run.
#[derive(Debug, Clone)]
pub struct DetailRequest {
    pub input: PathBuf,
    pub out: PathBuf,
    pub spacing_m: f64,
    pub crop_m: Option<[f64; 4]>,
    pub declared: Vec<Declared>,
}

/// What a run produced.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetailReport {
    pub dims: [usize; 2],
    pub origin_m: [f64; 2],
    pub spacing_m: f64,
    pub samples: usize,
    pub seconds_read: f64,
    pub seconds_filter: f64,
    pub seconds_write: f64,
    pub stats: DeltaStats,
}

/// Reads the base guide, resamples it to the lattice, applies the filter and writes a guide v1 folder
/// (`heights.f32` + `meta.json`; the region grid is not carried).
pub fn run_detail(req: &DetailRequest) -> Result<DetailReport> {
    let prm = FilterParams::from_declared(&req.declared)?;
    let t0 = Instant::now();
    let guide = crate::guide::Guide::read(&req.input)?;
    let base = BaseField {
        w: guide.w,
        h: guide.h,
        spacing_m: guide.spacing_m,
        origin_m: guide.origin_m,
        heights: &guide.heights,
    };
    let lat = Lattice::new(&base, req.spacing_m, req.crop_m)?;
    if lat.len() > WARN_SAMPLES {
        eprintln!(
            "scrib-worldgen detail: warning: {}x{} = {:.0} M samples (> {} M); consider --crop",
            lat.w,
            lat.h,
            lat.len() as f64 * 1e-6,
            WARN_SAMPLES / 1_000_000
        );
    }
    let t1 = Instant::now();
    let (base_h, out) = evaluate(&base, req.spacing_m, &lat, guide.sea_level_m, &prm);
    let stats = delta_stats(&base_h, &out, guide.sea_level_m);
    let t2 = Instant::now();
    let origin_m = [
        guide.origin_m[0] + lat.col0 as f64 * req.spacing_m,
        guide.origin_m[1] - lat.row0 as f64 * req.spacing_m,
    ];
    let mut report = DetailReport {
        dims: [lat.w, lat.h],
        origin_m,
        spacing_m: req.spacing_m,
        samples: lat.len(),
        seconds_read: (t1 - t0).as_secs_f64(),
        seconds_filter: (t2 - t1).as_secs_f64(),
        seconds_write: 0.0,
        stats,
    };
    write_guide(&req.out, &out, &guide, req, &report)?;
    report.seconds_write = t2.elapsed().as_secs_f64();
    Ok(report)
}

fn write_guide(
    dir: &Path,
    heights: &[f32],
    src: &crate::guide::Guide,
    req: &DetailRequest,
    rep: &DetailReport,
) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut bytes = Vec::with_capacity(heights.len() * 4);
    for v in heights {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(dir.join("heights.f32"), bytes)?;
    let meta = serde_json::json!({
        "format": 1,
        "dims": rep.dims,
        "sample_spacing_m": rep.spacing_m,
        "sea_level_m": src.sea_level_m,
        "fill_m": src.fill_m,
        "origin_m": rep.origin_m,
        "orientation": "row-major, row 0 = north edge, column 0 = west edge; sample (col, row) at \
                        x = origin_m.x + spacing col (east), y = origin_m.y - spacing row (north)",
        "producer": "sf_worldgen scrib-worldgen detail (runtime-detail erosion filter preview)",
        "detail": {
            "base_guide": req.input.display().to_string(),
            "base_spacing_m": src.spacing_m,
            "crop_m": req.crop_m,
            "params": req.declared,
            "guess_count": req.declared.iter().filter(|d| d.source == "Guess").count(),
            "stats": rep.stats,
            "seconds_filter": rep.seconds_filter,
        },
    });
    std::fs::write(dir.join("meta.json"), serde_json::to_string_pretty(&meta)?)?;
    Ok(())
}
