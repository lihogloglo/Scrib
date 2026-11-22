//! **Fast steerable macro sharp landforms** (Phase 3b; research `docs/research/terrain-sharp-features-2026-11-19.md`):
//! turns smooth/blobby heightfields into heavy-tailed geological relief (asymmetric fault-block scarps / cuestas,
//! macro stepped rock shelves, slope-selective cliff oversteepening, rocky sea-cliff headlands, explicit TOML
//! escarpment strokes, and hardness-aware thermal talus relaxation).
//!
//! Designed for fast iteration (~2-5 s on a 10 M-sample 50 x 50 km island): every pass is parallel and deterministic
//! (pure per-sample integer-hashed maps + Jacobi [`crate::thermal::relax`]), sea cells are fixed base level
//! ([`crate::Audit::is_clean`]), and every parameter is a declared value with unit, range and source.
//! Nothing here names a game.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::filter::Declared;
use crate::guide::{Guide, RegionMap};
use crate::lithology::{RockMap, RockType, UnitWeight};
use crate::perturb::{lattice, value_noise};
use crate::thermal::{ThermalParams, relax};
use crate::{Audit, Error, Grid, Result, hash_f32};

/// Minimum land elevation above sea level kept when sharpening modifies a dry sample, metres.
// tunables: const (invariant: keeps modified dry samples strictly above sea level so Audit stays clean)
const LAND_FLOOR_M: f64 = 0.05;

/// Declared defaults for the macro cliff pass.
pub fn declared_defaults() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared {
        name,
        v,
        unit,
        min,
        max,
        source: source.into(),
        help,
    };
    vec![
        d("seed", 77.0, "id", 0.0, 4_294_967_295.0, "Guess", "hash seed for rock units, scarps and headlands"),
        d("coast_fade_m", 4.0, "m", 0.1, 200.0, "Guess", "elevation above sea over which inland cliff ops fade in"),
        d("unit_cell_m", 1400.0, "m", 100.0, 20_000.0, "Guess", "geological fault-block / lithology unit spacing"),
        d("unit_warp_m", 320.0, "m", 0.0, 5000.0, "Guess", "domain warp amplitude of rock-unit boundaries"),
        d("unit_blend_m", 260.0, "m", 0.0, 10_000.0, "Guess", "width over which scarp bearing and rock strength blend across rock-unit borders (capped at half a unit cell)"),
        d("scarp_amp_m", 55.0, "m", 0.0, 400.0, "Guess", "asymmetric fault-block / cuesta scarp height"),
        d("scarp_wavelength_m", 620.0, "m", 40.0, 10_000.0, "Guess", "spacing between successive cuesta/fault scarps"),
        d("scarp_steepness", 0.84, "ratio", 0.55, 0.96, "Guess", "share of wavelength that is gentle dip slope (rest is steep cliff face)"),
        d("scarp_warp_m", 180.0, "m", 0.0, 3000.0, "Guess", "domain warp amplitude breaking up scarp trace linearity"),
        d("shelf_amp_m", 32.0, "m", 0.0, 300.0, "Guess", "macro stepped rock-shelf / strata bench amplitude on mountain flanks"),
        d("shelf_step_m", 58.0, "m", 5.0, 500.0, "Guess", "vertical elevation interval between macro rock shelves"),
        d("shelf_sharpness", 0.82, "ratio", 0.0, 1.0, "Guess", "riser sharpness of macro rock shelves (1 = near-vertical wall)"),
        d("steepen_gain", 1.35, "ratio", 0.0, 6.0, "Guess", "unsharp relief contrast gain on slopes above steepen_slope_deg"),
        d("steepen_slope_deg", 14.0, "deg", 1.0, 60.0, "Guess", "slope above which existing ridges/walls/foyadas oversteepen into cliffs"),
        d("steepen_cap_m", 65.0, "m", 0.0, 400.0, "Guess", "maximum displacement of the slope-selective oversteepening pass"),
        d("mask_boost_m", 45.0, "m", 0.0, 300.0, "Guess", "extra rocky outcrop & scarp lift where an external cliff mask is 1"),
        d("sea_cliff_amp_m", 38.0, "m", 0.0, 200.0, "Guess", "coastal headland cliff lift where terrain rises from sea"),
        d("sea_cliff_band_m", 45.0, "m", 2.0, 300.0, "Guess", "elevation band above sea where coastal headlands sharpen into sea cliffs"),
        d("hard_talus_deg", 68.0, "deg", 20.0, 85.0, "Industry(hard basalt/cap-rock cliff face repose 60-75 deg)", "thermal repose angle of hard cap-rock bands"),
        d("soft_talus_deg", 34.0, "deg", 15.0, 60.0, "Industry(scree/talus angle of repose 32-36 deg)", "thermal repose angle of soft scree/talus bands"),
        d("thermal_steps", 16.0, "count", 0.0, 200.0, "Guess", "hardness-aware Jacobi thermal iterations building talus aprons under hard cliffs"),
        d("thermal_rate", 0.30, "ratio", 0.01, 0.5, "Industry(Olsen 2004: Jacobi stability <= 0.5)", "thermal relaxation rate per step"),
        d("lava_flow_count", 28.0, "count", 0.0, 200.0, "Guess", "downhill-routed volcanic aa-lava flow lobes seeded in lava-enabled regions"),
        d("lava_thickness_m", 24.0, "m", 0.0, 120.0, "Industry(Etna/Hawaii aa-lava flow fronts 5-30 m)", "nominal thickness of routed lava lobes"),
        d("lava_width_m", 240.0, "m", 20.0, 2500.0, "Guess", "nominal lava lobe width (widens slightly toward toe)"),
        d("lava_length_m", 4800.0, "m", 100.0, 25_000.0, "Guess", "maximum downhill length of a routed lava flow"),
        d("lava_levee_ratio", 0.28, "ratio", 0.0, 0.8, "Guess", "raised lateral levee ridge height as a fraction of lava thickness"),
    ]
}

/// Per-region geological multipliers (TOML `[region."<name>"]`), smoothly blurred across region borders.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct RegionProfile {
    #[serde(default = "one")]
    pub scarp_scale: f64,
    #[serde(default = "one")]
    pub shelf_scale: f64,
    #[serde(default = "one")]
    pub steepen_scale: f64,
    #[serde(default = "one")]
    pub sea_cliff_scale: f64,
    #[serde(default)]
    pub lava_scale: f64,
}

fn one() -> f64 {
    1.0
}

impl Default for RegionProfile {
    fn default() -> Self {
        Self {
            // tunables: const (identity multipliers when a region has no TOML override)
            scarp_scale: 1.0,
            // tunables: const (identity multiplier)
            shelf_scale: 1.0,
            // tunables: const (identity multiplier)
            steepen_scale: 1.0,
            // tunables: const (identity multiplier)
            sea_cliff_scale: 1.0,
            // tunables: const (identity: lava flows only run in regions that opt in via TOML)
            lava_scale: 0.0,
        }
    }
}

impl RegionProfile {
    fn to_array(self) -> [f64; 5] {
        [
            self.scarp_scale,
            self.shelf_scale,
            self.steepen_scale,
            self.sea_cliff_scale,
            self.lava_scale,
        ]
    }

    fn from_array(a: [f64; 5]) -> Self {
        Self {
            scarp_scale: a[0],
            shelf_scale: a[1],
            steepen_scale: a[2],
            sea_cliff_scale: a[3],
            lava_scale: a[4],
        }
    }
}

/// Smoothly blurred cell-resolution field of [`RegionProfile`] over a guide's [`RegionMap`].
struct RegionField {
    w: usize,
    h: usize,
    cell_m: f64,
    origin_m: [f64; 2],
    cells: Vec<[f64; 5]>,
}

impl RegionField {
    fn build(rm: &RegionMap, origin_m: [f64; 2], profiles: &BTreeMap<String, RegionProfile>) -> Self {
        let lookup: Vec<[f64; 5]> = rm
            .names
            .iter()
            .map(|name| {
                profiles
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(name))
                    .map_or(RegionProfile::default(), |(_, p)| *p)
                    .to_array()
            })
            .collect();
        let mut cells: Vec<[f64; 5]> = rm
            .index
            .iter()
            .map(|&idx| lookup.get(usize::from(idx)).copied().unwrap_or(RegionProfile::default().to_array()))
            .collect();

        // Two separable [1, 2, 1] / 4 passes blur region geology smoothly across ~1-2 km borders.
        let (w, h) = (rm.w, rm.h);
        let mut tmp = vec![[0.0; 5]; w * h];
        for _ in 0..2 {
            for y in 0..h {
                for x in 0..w {
                    let a = cells[y * w + x.saturating_sub(1)];
                    let b = cells[y * w + x];
                    let c = cells[y * w + (x + 1).min(w - 1)];
                    for k in 0..5 {
                        tmp[y * w + x][k] = 0.25 * a[k] + 0.5 * b[k] + 0.25 * c[k];
                    }
                }
            }
            for y in 0..h {
                let y0 = y.saturating_sub(1);
                let y1 = (y + 1).min(h - 1);
                for x in 0..w {
                    let a = tmp[y0 * w + x];
                    let b = tmp[y * w + x];
                    let c = tmp[y1 * w + x];
                    for k in 0..5 {
                        cells[y * w + x][k] = 0.25 * a[k] + 0.5 * b[k] + 0.25 * c[k];
                    }
                }
            }
        }
        Self { w, h, cell_m: rm.cell_m, origin_m, cells }
    }

    fn sample(&self, wx: f64, wy: f64) -> RegionProfile {
        let u = ((wx - self.origin_m[0]) / self.cell_m - 0.5).clamp(0.0, (self.w - 1) as f64);
        let v = ((self.origin_m[1] - wy) / self.cell_m - 0.5).clamp(0.0, (self.h - 1) as f64);
        let x0 = (u.floor() as usize).min(self.w - 1);
        let y0 = (v.floor() as usize).min(self.h - 1);
        let x1 = (x0 + 1).min(self.w - 1);
        let y1 = (y0 + 1).min(self.h - 1);
        let tx = u - x0 as f64;
        let ty = v - y0 as f64;
        let c00 = self.cells[y0 * self.w + x0];
        let c10 = self.cells[y0 * self.w + x1];
        let c01 = self.cells[y1 * self.w + x0];
        let c11 = self.cells[y1 * self.w + x1];
        let mut out = [0.0; 5];
        for k in 0..5 {
            let top = c00[k] + (c10[k] - c00[k]) * tx;
            let bot = c01[k] + (c11[k] - c01[k]) * tx;
            out[k] = top + (bot - top) * ty;
        }
        RegionProfile::from_array(out)
    }
}

/// An explicit steerable escarpment stroke (TOML `[[stroke]]`): a warped linear cliff trace.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CliffStroke {
    pub name: String,
    /// Start point `(x east, y north)` in guide metres.
    pub from_m: [f64; 2],
    /// End point `(x east, y north)` in guide metres.
    pub to_m: [f64; 2],
    /// Total vertical drop across the cliff face, metres (positive raises the left side of `from -> to`).
    pub throw_m: f64,
    /// Width of the steep cliff face, metres.
    pub wall_width_m: f64,
    /// Distance over which the uplifted/dropped shoulders blend back into the surrounding land, metres.
    pub apron_width_m: f64,
    /// Lateral domain warp amplitude along the stroke trace, metres.
    #[serde(default = "default_stroke_warp_m")]
    pub warp_m: f64,
    /// Lateral domain warp wavelength, metres.
    #[serde(default = "default_stroke_warp_wl")]
    pub warp_wavelength_m: f64,
}

fn default_stroke_warp_m() -> f64 {
    120.0
}

fn default_stroke_warp_wl() -> f64 {
    600.0
}

/// Typed parameters for [`apply_cliffs`].
#[derive(Debug, Clone, PartialEq)]
pub struct CliffParams {
    pub seed: u64,
    pub coast_fade_m: f64,
    pub unit_cell_m: f64,
    pub unit_warp_m: f64,
    pub unit_blend_m: f64,
    pub scarp_amp_m: f64,
    pub scarp_wavelength_m: f64,
    pub scarp_steepness: f64,
    pub scarp_warp_m: f64,
    pub shelf_amp_m: f64,
    pub shelf_step_m: f64,
    pub shelf_sharpness: f64,
    pub steepen_gain: f64,
    pub steepen_slope_deg: f64,
    pub steepen_cap_m: f64,
    pub mask_boost_m: f64,
    pub sea_cliff_amp_m: f64,
    pub sea_cliff_band_m: f64,
    pub hard_talus_deg: f64,
    pub soft_talus_deg: f64,
    pub thermal_steps: usize,
    pub thermal_rate: f64,
    pub lava_flow_count: usize,
    pub lava_thickness_m: f64,
    pub lava_width_m: f64,
    pub lava_length_m: f64,
    pub lava_levee_ratio: f64,
    pub strokes: Vec<CliffStroke>,
    pub regions: BTreeMap<String, RegionProfile>,
}

impl Default for CliffParams {
    fn default() -> Self {
        Self::from_declared(&declared_defaults(), Vec::new(), BTreeMap::new()).expect("defaults valid")
    }
}

impl CliffParams {
    pub fn from_declared(
        d: &[Declared],
        strokes: Vec<CliffStroke>,
        regions: BTreeMap<String, RegionProfile>,
    ) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("cliff value {name} not declared")))?;
            if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
                Ok(p.v)
            } else {
                Err(Error::Invalid(format!("{name} = {} outside [{}, {}]", p.v, p.min, p.max)))
            }
        };
        Ok(Self {
            seed: get("seed")? as u64,
            coast_fade_m: get("coast_fade_m")?,
            unit_cell_m: get("unit_cell_m")?,
            unit_warp_m: get("unit_warp_m")?,
            unit_blend_m: get("unit_blend_m")?,
            scarp_amp_m: get("scarp_amp_m")?,
            scarp_wavelength_m: get("scarp_wavelength_m")?,
            scarp_steepness: get("scarp_steepness")?,
            scarp_warp_m: get("scarp_warp_m")?,
            shelf_amp_m: get("shelf_amp_m")?,
            shelf_step_m: get("shelf_step_m")?,
            shelf_sharpness: get("shelf_sharpness")?,
            steepen_gain: get("steepen_gain")?,
            steepen_slope_deg: get("steepen_slope_deg")?,
            steepen_cap_m: get("steepen_cap_m")?,
            mask_boost_m: get("mask_boost_m")?,
            sea_cliff_amp_m: get("sea_cliff_amp_m")?,
            sea_cliff_band_m: get("sea_cliff_band_m")?,
            hard_talus_deg: get("hard_talus_deg")?,
            soft_talus_deg: get("soft_talus_deg")?,
            thermal_steps: get("thermal_steps")?.round() as usize,
            thermal_rate: get("thermal_rate")?,
            lava_flow_count: get("lava_flow_count")?.round() as usize,
            lava_thickness_m: get("lava_thickness_m")?,
            lava_width_m: get("lava_width_m")?,
            lava_length_m: get("lava_length_m")?,
            lava_levee_ratio: get("lava_levee_ratio")?,
            strokes,
            regions,
        })
    }
}

/// Slope-distribution sharpness metrics on dry land (research §5: heavy-tailed slope distribution).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct SlopeStats {
    pub dry_samples: usize,
    pub pct_over_25_deg: f64,
    pub pct_over_35_deg: f64,
    pub pct_over_50_deg: f64,
    pub p95_deg: f64,
    pub p99_deg: f64,
    pub max_deg: f64,
}

impl SlopeStats {
    pub fn measure(g: &Grid, sea_level: f64) -> Self {
        let mut slopes = Vec::with_capacity(g.len() / 2);
        let inv2 = 0.5 / g.spacing;
        for y in 1..g.h.saturating_sub(1) {
            for x in 1..g.w.saturating_sub(1) {
                let i = y * g.w + x;
                if g.data[i] < sea_level {
                    continue;
                }
                let dx = (g.data[i + 1] - g.data[i - 1]) * inv2;
                let dy = (g.data[i + g.w] - g.data[i - g.w]) * inv2;
                let deg = dx.hypot(dy).atan().to_degrees();
                slopes.push(deg);
            }
        }
        if slopes.is_empty() {
            return Self {
                dry_samples: 0,
                pct_over_25_deg: 0.0,
                pct_over_35_deg: 0.0,
                pct_over_50_deg: 0.0,
                p95_deg: 0.0,
                p99_deg: 0.0,
                max_deg: 0.0,
            };
        }
        slopes.sort_unstable_by(f64::total_cmp);
        let n = slopes.len() as f64;
        let pct = |deg: f64| 100.0 * slopes.iter().filter(|&&s| s >= deg).count() as f64 / n;
        let q = |p: f64| slopes[((slopes.len() - 1) as f64 * p).round() as usize];
        Self {
            dry_samples: slopes.len(),
            pct_over_25_deg: pct(25.0),
            pct_over_35_deg: pct(35.0),
            pct_over_50_deg: pct(50.0),
            p95_deg: q(0.95),
            p99_deg: q(0.99),
            max_deg: *slopes.last().unwrap_or(&0.0),
        }
    }
}

#[inline]
fn smoothstep(lo: f64, hi: f64, x: f64) -> f64 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Scarp / shelf strength of a rock-unit type (0 hard cliff-former, 1 medium, 2 soft).
#[inline]
fn unit_strength_of(kind: usize) -> f64 {
    match kind {
        0 => 1.25,
        1 => 0.85,
        _ => 0.45,
    }
}

/// Talus hardness of a rock-unit type (0 hard cliff-former, 1 medium, 2 soft).
#[inline]
fn unit_hard_of(kind: usize) -> f64 {
    match kind {
        0 => 0.85,
        1 => 0.55,
        _ => 0.25,
    }
}

/// Asymmetric cuesta / fault-scarp wave in `[-0.5, 0.5]`: gentle ramp over `[0, split]`, steep drop over `[split, 1]`.
#[inline]
fn cuesta_wave(phase: f64, split: f64) -> f64 {
    let f = phase - phase.floor();
    let v = if f < split {
        f / split
    } else {
        let t = (f - split) / (1.0 - split);
        1.0 - t * t * (3.0 - 2.0 * t)
    };
    v - 0.5
}

/// Evaluates explicit TOML escarpment strokes at world position `(wx, wy)`.
fn stroke_delta(wx: f64, wy: f64, strokes: &[CliffStroke], seed: u64) -> f64 {
    let mut sum = 0.0;
    for (idx, st) in strokes.iter().enumerate() {
        let vx = st.to_m[0] - st.from_m[0];
        let vy = st.to_m[1] - st.from_m[1];
        let len = vx.hypot(vy);
        if len < 1.0 || st.throw_m == 0.0 {
            continue;
        }
        let ux = vx / len;
        let uy = vy / len;
        let dx = wx - st.from_m[0];
        let dy = wy - st.from_m[1];
        let along = dx * ux + dy * uy;
        let taper_m = (0.15 * len).clamp(100.0, 1500.0);
        if along < -taper_m || along > len + taper_m {
            continue;
        }
        let perp_raw = -dx * uy + dy * ux;
        let reach = st.apron_width_m + st.wall_width_m + st.warp_m;
        if perp_raw.abs() > reach {
            continue;
        }
        let s = seed ^ ((idx as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let warp = if st.warp_m > 0.0 && st.warp_wavelength_m > 1.0 {
            st.warp_m * value_noise([wx, wy], st.warp_wavelength_m, s)
        } else {
            0.0
        };
        let perp = perp_raw + warp;
        let end_fade = smoothstep(-taper_m, taper_m, along) * (1.0 - smoothstep(len - taper_m, len + taper_m, along));
        let hw = (0.5 * st.wall_width_m).max(1.0);
        // Step from -0.5 on right (perp < 0) to +0.5 on left (perp > 0), fading back to 0 across apron_width_m.
        let wall = smoothstep(-hw, hw, perp) - 0.5;
        let apron = 1.0 - smoothstep(hw, hw + st.apron_width_m.max(1.0), perp.abs());
        sum += st.throw_m * wall * apron * end_fade;
    }
    sum
}

/// Traces deterministic downhill volcanic aa-lava flow lobes with steep fronts and raised channel
/// levees inside regions where `lava_scale > 0.2`. Returns per-sample height delta (metres, >= 0).
#[allow(clippy::too_many_lines)]
fn route_lava_lobes(
    guide: &Guide,
    base: &[f64],
    region_field: Option<&RegionField>,
    params: &CliffParams,
) -> Vec<f64> {
    let n = guide.w * guide.h;
    let mut lava = vec![0.0f64; n];
    if params.lava_flow_count == 0 || params.lava_thickness_m <= 0.0 {
        return lava;
    }
    let Some(rf) = region_field else {
        return lava;
    };
    let (w, h, sp, sea) = (guide.w, guide.h, guide.spacing_m, guide.sea_level_m);
    let stride = (400.0 / sp).round().max(4.0) as usize;
    let mut candidates: Vec<(f64, usize, usize, f64)> = Vec::new();
    for y in (stride..h.saturating_sub(stride)).step_by(stride) {
        let wy = guide.origin_m[1] - y as f64 * sp;
        for x in (stride..w.saturating_sub(stride)).step_by(stride) {
            let h0 = base[y * w + x];
            if h0 < sea + 80.0 {
                continue;
            }
            let wx = guide.origin_m[0] + x as f64 * sp;
            let rp = rf.sample(wx, wy);
            if rp.lava_scale < 0.25 {
                continue;
            }
            let score = lattice(x as i64, y as i64, params.seed ^ 0x7a51_e923) + 0.35 * (h0 / 3000.0).min(1.0);
            candidates.push((score, x, y, rp.lava_scale));
        }
    }
    candidates.sort_by(|a, b| b.0.total_cmp(&a.0));

    let min_vent_sep_m = (params.lava_width_m * 3.0).max(650.0);
    let mut vents: Vec<([f64; 2], f64)> = Vec::new();
    for (_, cx, cy, lscale) in candidates {
        if vents.len() >= params.lava_flow_count {
            break;
        }
        let wx = guide.origin_m[0] + cx as f64 * sp;
        let wy = guide.origin_m[1] - cy as f64 * sp;
        if vents.iter().any(|(v, _)| (v[0] - wx).hypot(v[1] - wy) < min_vent_sep_m) {
            continue;
        }
        vents.push(([wx, wy], lscale));
    }

    let step_m = (1.5 * sp).max(20.0);
    let max_steps = (params.lava_length_m / step_m).round().max(4.0) as usize;
    let inv2 = 0.5 / sp;

    for (v_idx, (start, vent_scale)) in vents.iter().enumerate() {
        let fseed = params.seed ^ ((v_idx as u64 + 1).wrapping_mul(0x517c_c1b7_2722_0a95));
        let mut pts: Vec<[f64; 2]> = Vec::with_capacity(max_steps + 1);
        let mut p = *start;
        pts.push(p);
        let mut dir = [0.0f64; 2];

        for _ in 0..max_steps {
            let fx = (p[0] - guide.origin_m[0]) / sp;
            let fy = (guide.origin_m[1] - p[1]) / sp;
            let ix = fx.round() as isize;
            let iy = fy.round() as isize;
            if ix < 2 || iy < 2 || ix >= (w as isize - 2) || iy >= (h as isize - 2) {
                break;
            }
            let (ux, uy) = (ix as usize, iy as usize);
            let hc = base[uy * w + ux];
            if hc < sea + 12.0 {
                break;
            }
            // Steepest descent in guide frame (+x east, +y north -> row increases south).
            let gx = (base[uy * w + ux + 1] - base[uy * w + ux - 1]) * inv2;
            let gy = (base[(uy - 1) * w + ux] - base[(uy + 1) * w + ux]) * inv2;
            let gmag = gx.hypot(gy);
            if gmag < 0.015 {
                break;
            }
            let mut dx = -gx / gmag;
            let mut dy = -gy / gmag;
            let wander = 0.28 * value_noise(p, 520.0, fseed);
            let (cw, sw) = (wander.cos(), wander.sin());
            let (rx, ry) = (dx * cw - dy * sw, dx * sw + dy * cw);
            if dir[0] == 0.0 && dir[1] == 0.0 {
                dx = rx;
                dy = ry;
            } else {
                dx = 0.65 * rx + 0.35 * dir[0];
                dy = 0.65 * ry + 0.35 * dir[1];
                let dlen = dx.hypot(dy).max(1e-6);
                dx /= dlen;
                dy /= dlen;
            }
            dir = [dx, dy];
            p = [p[0] + dx * step_m, p[1] + dy * step_m];
            pts.push(p);
        }
        if pts.len() < 4 {
            continue;
        }

        // Rasterize each segment into its tight bounding box with steep aa-lava fronts + lateral levees.
        let total_segs = (pts.len() - 1) as f64;
        let flow_thick = params.lava_thickness_m * vent_scale * (0.75 + 0.5 * lattice(v_idx as i64, 17, fseed).abs());
        let base_half_w = 0.5 * params.lava_width_m * (0.7 + 0.6 * lattice(v_idx as i64, 53, fseed).abs());

        for s_idx in 0..pts.len() - 1 {
            let frac = s_idx as f64 / total_segs;
            let a = pts[s_idx];
            let b = pts[s_idx + 1];
            let abx = b[0] - a[0];
            let aby = b[1] - a[1];
            let ab2 = (abx * abx + aby * aby).max(1e-6);
            // Widen gently toward the cooling toe lobe; taper height cleanly at vent and toe.
            let half_w = base_half_w * (0.55 + 0.65 * frac);
            let end_taper = smoothstep(0.0, 0.08, frac) * (1.0 - smoothstep(0.90, 1.0, frac));
            let pad = half_w + sp;
            let x_min = a[0].min(b[0]) - pad;
            let x_max = a[0].max(b[0]) + pad;
            let y_min = a[1].min(b[1]) - pad;
            let y_max = a[1].max(b[1]) + pad;
            let c0 = ((x_min - guide.origin_m[0]) / sp).floor().max(0.0) as usize;
            let c1 = ((x_max - guide.origin_m[0]) / sp).ceil().min((w - 1) as f64) as usize;
            let r0 = ((guide.origin_m[1] - y_max) / sp).floor().max(0.0) as usize;
            let r1 = ((guide.origin_m[1] - y_min) / sp).ceil().min((h - 1) as f64) as usize;
            for r in r0..=r1 {
                let wy = guide.origin_m[1] - r as f64 * sp;
                for c in c0..=c1 {
                    let i = r * w + c;
                    if base[i] < sea + 2.0 {
                        continue;
                    }
                    let wx = guide.origin_m[0] + c as f64 * sp;
                    let u = (((wx - a[0]) * abx + (wy - a[1]) * aby) / ab2).clamp(0.0, 1.0);
                    let px = a[0] + u * abx;
                    let py = a[1] + u * aby;
                    let d = (wx - px).hypot(wy - py);
                    let t = d / half_w;
                    if t >= 1.0 {
                        continue;
                    }
                    // Steep aa-lava margin (flat lobe top, crisp drop over t in 0.72..1.0) + raised lateral levees.
                    let front = 1.0 - smoothstep(0.72, 1.0, t);
                    let levee_d = (t - 0.64) / 0.15;
                    let levee = (-levee_d * levee_d).exp() * params.lava_levee_ratio;
                    let rough = 0.88 + 0.24 * value_noise([wx, wy], 75.0, fseed ^ 0x3333);
                    let dh = flow_thick * end_taper * (front + levee) * rough;
                    if dh > lava[i] {
                        lava[i] = dh;
                    }
                }
            }
        }
    }
    lava
}

/// Output summary of [`apply_cliffs`].
#[derive(Debug, Clone, Serialize)]
pub struct CliffReport {
    pub before: SlopeStats,
    pub after: SlopeStats,
    pub audit: Audit,
    pub thermal_moved_m3: f64,
    pub wall_s: f64,
    pub blake3: String,
}

/// Applies the fast steerable macro cliff pass to `guide` in place (returns the modified heights and report).
/// Optional `cliff_mask` (same length as `guide.heights`, values in `[0, 1]`) boosts outcrops/oversteepening at
/// authored cliff locations (e.g. harvested `terrain_rock_*` footprints).
#[allow(clippy::too_many_lines)]
pub fn apply_cliffs(
    guide: &Guide,
    params: &CliffParams,
    cliff_mask: Option<&[f32]>,
) -> Result<(Vec<f32>, CliffReport)> {
    if let Some(m) = cliff_mask
        && m.len() != guide.heights.len()
    {
        return Err(Error::Invalid(format!(
            "cliff mask len {} does not match guide {}",
            m.len(),
            guide.heights.len()
        )));
    }
    let started = Instant::now();
    let (w, h) = (guide.w, guide.h);
    let sea = guide.sea_level_m;
    let base_f64: Vec<f64> = guide.heights.iter().map(|&v| f64::from(v)).collect();
    let base_grid = Grid { w, h, spacing: guide.spacing_m, data: base_f64.clone() };
    let before = SlopeStats::measure(&base_grid, sea);

    // Precompute local slope and 2-ring local mean on the base heightfield (for unsharp oversteepening).
    let mut base_slope = vec![0.0f64; w * h];
    let mut local_mean = vec![0.0f64; w * h];
    let inv2 = 0.5 / guide.spacing_m;
    base_slope
        .par_chunks_mut(w)
        .zip(local_mean.par_chunks_mut(w))
        .enumerate()
        .for_each(|(y, (sr, mr))| {
            let y0 = y.saturating_sub(1);
            let y1 = (y + 1).min(h - 1);
            let y2a = y.saturating_sub(3);
            let y2b = (y + 3).min(h - 1);
            for x in 0..w {
                let x0 = x.saturating_sub(1);
                let x1 = (x + 1).min(w - 1);
                let x2a = x.saturating_sub(3);
                let x2b = (x + 3).min(w - 1);
                let gx = (base_f64[y * w + x1] - base_f64[y * w + x0]) * inv2;
                let gy = (base_f64[y1 * w + x] - base_f64[y0 * w + x]) * inv2;
                sr[x] = gx.hypot(gy);
                let m = 0.25
                    * (base_f64[y * w + x2a]
                        + base_f64[y * w + x2b]
                        + base_f64[y2a * w + x]
                        + base_f64[y2b * w + x]);
                mr[x] = m;
            }
        });

    // Geological rock units (reuses `lithology::RockMap` for domain-warped Voronoi fault blocks).
    let rock_types = [
        RockType { k: 0.45, share: 0.28 }, // hard cliff-forming basalt / granite
        RockType { k: 1.0, share: 0.42 },  // medium bedded rock
        RockType { k: 1.7, share: 0.30 },  // softer fractured/tuff unit
    ];
    let rock_cum = crate::lithology::cumulative(&rock_types);
    let rock_map = RockMap {
        cell_m: params.unit_cell_m,
        jitter: 0.85,
        warp_m: params.unit_warp_m,
        warp_wavelength_m: params.unit_cell_m * 0.9,
        edge_band_m: params.unit_cell_m * 0.08,
        edge_k: 1.6,
        seed: params.seed ^ 0x3a2f_1d09,
    };

    let steepen_tan = params.steepen_slope_deg.to_radians().tan();
    let hard_tan = params.hard_talus_deg.to_radians().tan();
    let soft_tan = params.soft_talus_deg.to_radians().tan();

    let region_field = guide
        .regions
        .as_ref()
        .filter(|_| !params.regions.is_empty())
        .map(|rm| RegionField::build(rm, guide.origin_m, &params.regions));
    let lava_dh = route_lava_lobes(guide, &base_f64, region_field.as_ref(), params);

    let mut work = vec![0.0f64; w * h];
    let mut tan_talus = vec![soft_tan; w * h];

    work.par_chunks_mut(w)
        .zip(tan_talus.par_chunks_mut(w))
        .enumerate()
        .for_each(|(y, (wr, tr))| {
            let wy = guide.origin_m[1] - y as f64 * guide.spacing_m;
            for x in 0..w {
                let i = y * w + x;
                let h0 = base_f64[i];
                if h0 < sea {
                    wr[x] = h0;
                    tr[x] = soft_tan;
                    continue;
                }
                let wx = guide.origin_m[0] + x as f64 * guide.spacing_m;
                let above = h0 - sea;
                let shore = smoothstep(0.0, params.coast_fade_m, above);
                let sl = base_slope[i];
                let reg = region_field.as_ref().map_or_else(RegionProfile::default, |rf| rf.sample(wx, wy));

                // Geological units (the RockMap's domain-warped Voronoi cells) with weights blending
                // continuously across unit borders; each unit has its own dip bearing for asymmetric scarps.
                let mut units_buf = [UnitWeight::default(); 25];
                let n_units = rock_map.unit_weights([wx, wy], &rock_cum, params.unit_blend_m, &mut units_buf);
                let units = &units_buf[..n_units];
                let unit_strength: f64 = units.iter().map(|u| u.w * unit_strength_of(u.kind)).sum();

                let mut dh = 0.0;

                // 1. Asymmetric fault-block / cuesta scarps.
                if params.scarp_amp_m > 0.0 && params.scarp_wavelength_m > 1.0 && reg.scarp_scale > 0.0 {
                    let warp_wl = params.scarp_wavelength_m * 0.75;
                    let warped_x = wx + params.scarp_warp_m * value_noise([wx, wy], warp_wl, params.seed ^ 0x1111);
                    let warped_y = wy + params.scarp_warp_m * value_noise([wx, wy], warp_wl, params.seed ^ 0x2222);
                    // Blend each unit's strength x wave (never the bearings: angles wrap).
                    let wave: f64 = units
                        .iter()
                        .map(|u| {
                            let unit_ang = std::f64::consts::PI * lattice(u.ix, u.iy, params.seed ^ 0x91e1_0da5);
                            let proj = (warped_x * unit_ang.cos() + warped_y * unit_ang.sin()) / params.scarp_wavelength_m;
                            u.w * unit_strength_of(u.kind) * cuesta_wave(proj, params.scarp_steepness)
                        })
                        .sum();
                    // Active on hills & slopes, gentler in low flats.
                    let mask = shore * smoothstep(2.0, 28.0, above) * (0.25 + 0.75 * smoothstep(0.04, 0.25, sl));
                    dh += params.scarp_amp_m * reg.scarp_scale * mask * wave;
                }

                // 2. Macro stepped rock shelves on mountain flanks.
                if params.shelf_amp_m > 0.0 && params.shelf_step_m > 1.0 && reg.shelf_scale > 0.0 {
                    let s_mask = shore * smoothstep(0.10, 0.32, sl);
                    if s_mask > 0.0 {
                        let warp = value_noise([wx, wy], 420.0, params.seed ^ 0x4444) * 0.45 * params.shelf_step_m;
                        let u = (h0 + warp) / params.shelf_step_m;
                        let f = u - u.floor();
                        let r = 0.44 - 0.38 * params.shelf_sharpness.clamp(0.0, 1.0);
                        let step_dev = smoothstep(0.5 - r, 0.5 + r, f) - f;
                        dh += params.shelf_amp_m * reg.shelf_scale * unit_strength * s_mask * step_dev;
                    }
                }

                // 3. Slope-selective unsharp oversteepening (sharpens existing foyadas, ridges, caldera walls).
                if params.steepen_gain > 0.0 && params.steepen_cap_m > 0.0 && reg.steepen_scale > 0.0 {
                    let over = smoothstep(steepen_tan * 0.7, steepen_tan * 1.6, sl);
                    if over > 0.0 {
                        let contrast = (h0 - local_mean[i]) * params.steepen_gain * reg.steepen_scale * over;
                        dh += contrast.clamp(-params.steepen_cap_m, params.steepen_cap_m) * shore;
                    }
                }

                // 4. Rocky headland sea cliffs (gated by headland noise so beaches stay sandy/gentle).
                if params.sea_cliff_amp_m > 0.0 && reg.sea_cliff_scale > 0.0 && above < params.sea_cliff_band_m * 2.5 {
                    let headland = smoothstep(0.15, 0.55, value_noise([wx, wy], 900.0, params.seed ^ 0x6666));
                    let coast_rise = smoothstep(0.05, 0.22, sl);
                    let band = smoothstep(0.5, params.sea_cliff_band_m * 0.4, above)
                        * (1.0 - smoothstep(params.sea_cliff_band_m, params.sea_cliff_band_m * 2.4, above));
                    dh += params.sea_cliff_amp_m * reg.sea_cliff_scale * headland * coast_rise * band;
                }

                // 5. Routed volcanic aa-lava flow lobes (steep fronts & lateral levees).
                if lava_dh[i] > 0.0 {
                    dh += shore * lava_dh[i];
                }

                // 6. External cliff/outcrop steering mask (e.g. harvested terrain_rock_* footprints).
                if let Some(mask) = cliff_mask {
                    let mv = f64::from(mask[i]).clamp(0.0, 1.0);
                    if mv > 0.0 && params.mask_boost_m > 0.0 {
                        let crag = 1.0 - value_noise([wx, wy], 90.0, params.seed ^ 0x8888).abs();
                        dh += params.mask_boost_m * shore * mv * (0.4 + 0.6 * crag * crag);
                    }
                }

                // 7. Explicit TOML escarpment strokes.
                if !params.strokes.is_empty() {
                    dh += shore * stroke_delta(wx, wy, &params.strokes, params.seed ^ 0xaaaa);
                }

                let h_new = (h0 + dh).max(sea + LAND_FLOOR_M);
                wr[x] = h_new;

                // Per-sample hardness talus angle: alternating cap-rock bands + hard rock units stay steep;
                // softer bands slump into 34 deg talus aprons at cliff feet.
                let band_p = ((h_new / params.shelf_step_m.max(10.0)) + 0.35 * value_noise([wx, wy], 380.0, params.seed ^ 0x5555)).rem_euclid(1.0);
                let cap_rock = smoothstep(0.35, 0.65, band_p);
                let unit_hard: f64 = units.iter().map(|u| u.w * unit_hard_of(u.kind)).sum();
                let hard = (0.55 * cap_rock + 0.45 * unit_hard).clamp(0.0, 1.0);
                tr[x] = soft_tan + (hard_tan - soft_tan) * hard;
            }
        });

    let mut grid = Grid { w, h, spacing: guide.spacing_m, data: work };
    let moved = if params.thermal_steps > 0 {
        relax(
            &mut grid,
            &ThermalParams {
                iterations: params.thermal_steps,
                rate: params.thermal_rate,
                tan_talus: &tan_talus,
                sea_level: sea,
            },
        )?
    } else {
        0.0
    };

    let audit = Audit::compare(&base_f64, &grid.data, sea);
    let after = SlopeStats::measure(&grid, sea);
    let blake3 = hash_f32(&grid.data);
    let out_f32: Vec<f32> = grid.data.iter().map(|&v| v as f32).collect();
    let report = CliffReport {
        before,
        after,
        audit,
        thermal_moved_m3: moved,
        wall_s: started.elapsed().as_secs_f64(),
        blake3,
    };
    Ok((out_f32, report))
}

/// Recipe TOML file for `scrib-worldgen cliffs`.
#[derive(Debug, Clone, Deserialize)]
pub struct CliffRecipe {
    #[serde(default)]
    pub params: toml::Table,
    #[serde(default)]
    pub stroke: Vec<CliffStroke>,
    #[serde(default)]
    pub region: BTreeMap<String, RegionProfile>,
    #[serde(default)]
    pub arm: Vec<CliffArm>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CliffArm {
    pub name: String,
    #[serde(default)]
    pub help: String,
    #[serde(default)]
    pub set: toml::Table,
}

/// Request for [`run_cliffs`].
pub struct CliffRequest {
    pub base: PathBuf,
    pub params_path: PathBuf,
    pub out: PathBuf,
    pub mask_path: Option<PathBuf>,
    pub arms: Option<Vec<String>>,
}

/// Runs all (or selected) arms of a cliff recipe and writes guide v1 folders + `README.md`.
pub fn run_cliffs(req: &CliffRequest) -> Result<String> {
    let guide = Guide::read(&req.base)?;
    let text = std::fs::read_to_string(&req.params_path)?;
    let recipe: CliffRecipe = toml::from_str(&text)?;
    let mask = if let Some(mp) = &req.mask_path {
        let bytes = std::fs::read(mp)?;
        if bytes.len() != guide.heights.len() * 4 {
            return Err(Error::Invalid(format!(
                "cliff mask {} has {} bytes, expected {}",
                mp.display(),
                bytes.len(),
                guide.heights.len() * 4
            )));
        }
        Some(
            bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|c| f32::from_le_bytes(*c))
                .collect::<Vec<f32>>(),
        )
    } else {
        None
    };

    let mut base_decl = declared_defaults();
    let base_toml = toml::to_string(&recipe.params).map_err(|e| Error::Invalid(e.to_string()))?;
    crate::filter::apply_overrides_toml(&mut base_decl, &base_toml)?;

    let arms: Vec<CliffArm> = if recipe.arm.is_empty() {
        vec![CliffArm { name: "default".into(), help: "default parameters".into(), set: toml::Table::new() }]
    } else if let Some(filter) = &req.arms {
        recipe.arm.into_iter().filter(|a| filter.contains(&a.name)).collect()
    } else {
        recipe.arm
    };
    if arms.is_empty() {
        return Err(Error::Invalid("no matching cliff arms".into()));
    }

    std::fs::create_dir_all(&req.out)?;
    let mut summary = String::from("# Macro cliff pass (`scrib-worldgen cliffs`)\n\n");
    let _ = writeln!(summary, "Base: `{}` ({}x{} @ {} m)\n", req.base.display(), guide.w, guide.h, guide.spacing_m);
    summary.push_str("| Arm | >25° % | >35° % | >50° % | p95° | p99° | max° | wet/dry | time s | hash |\n");
    summary.push_str("|---|---:|---:|---:|---:|---:|---:|---|---:|---|\n");

    let mut printed_base = false;
    for arm in &arms {
        let mut d = base_decl.clone();
        if !arm.set.is_empty() {
            let s = toml::to_string(&arm.set).map_err(|e| Error::Invalid(e.to_string()))?;
            crate::filter::apply_overrides_toml(&mut d, &s)?;
        }
        let prm = CliffParams::from_declared(&d, recipe.stroke.clone(), recipe.region.clone())?;
        let (heights, rep) = apply_cliffs(&guide, &prm, mask.as_deref())?;
        if !rep.audit.is_clean() {
            return Err(Error::Invalid(format!("arm {} broke wet/dry audit: {:?}", arm.name, rep.audit)));
        }
        if !printed_base {
            let b = rep.before;
            let _ = writeln!(
                summary,
                "| *(base)* | {:.2} | {:.2} | {:.2} | {:.1} | {:.1} | {:.1} | 0/0/0 | - | - |",
                b.pct_over_25_deg, b.pct_over_35_deg, b.pct_over_50_deg, b.p95_deg, b.p99_deg, b.max_deg
            );
            printed_base = true;
        }
        let a = rep.after;
        let _ = writeln!(
            summary,
            "| {} | {:.2} | {:.2} | {:.2} | {:.1} | {:.1} | {:.1} | {}/{}/{} | {:.2} | `{}` |",
            arm.name,
            a.pct_over_25_deg,
            a.pct_over_35_deg,
            a.pct_over_50_deg,
            a.p95_deg,
            a.p99_deg,
            a.max_deg,
            rep.audit.new_wet,
            rep.audit.new_dry,
            rep.audit.sea_moved,
            rep.wall_s,
            &rep.blake3[..12],
        );

        let arm_dir = if arms.len() == 1 && arm.name == "default" {
            req.out.clone()
        } else {
            req.out.join(&arm.name)
        };
        write_guide_copy(&req.base, &arm_dir, &guide, &heights, &rep, &arm.name)?;
    }

    std::fs::write(req.out.join("README.md"), &summary)?;
    Ok(summary)
}

fn write_guide_copy(
    base_dir: &Path,
    out_dir: &Path,
    guide: &Guide,
    heights: &[f32],
    rep: &CliffReport,
    arm: &str,
) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let mut raw = Vec::with_capacity(heights.len() * 4);
    for &h in heights {
        raw.extend_from_slice(&h.to_le_bytes());
    }
    std::fs::write(out_dir.join("heights.f32"), raw)?;

    let base_meta: Option<serde_json::Value> = std::fs::read_to_string(base_dir.join("meta.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok());
    let mut meta = serde_json::json!({
        "format": 1,
        "dims": [guide.w, guide.h],
        "sample_spacing_m": guide.spacing_m,
        "sea_level_m": guide.sea_level_m,
        "origin_m": guide.origin_m,
        "producer": format!("scrib-worldgen cliffs ({arm})"),
        "heights_blake3": rep.blake3,
        "cliffs": rep,
    });
    if let (Some(obj), Some(src)) = (meta.as_object_mut(), base_meta.as_ref().and_then(serde_json::Value::as_object)) {
        for key in ["cell_size_m", "fill_m"] {
            if let Some(v) = src.get(key) {
                obj.insert(key.to_owned(), v.clone());
            }
        }
    }
    std::fs::write(out_dir.join("meta.json"), serde_json::to_string_pretty(&meta)?)?;

    for sidecar in ["regions.png", "regions.json", "markers.json"] {
        let src = base_dir.join(sidecar);
        if src.exists() {
            std::fs::copy(&src, out_dir.join(sidecar))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hill_guide() -> Guide {
        let (w, h, s) = (96, 96, 20.0);
        let mut heights = vec![0.0f32; w * h];
        for r in 0..h {
            for c in 0..w {
                let dx = (c as f64 - 48.0) * s;
                let dy = (r as f64 - 48.0) * s;
                let dist = dx.hypot(dy);
                let elev = (280.0 * (1.0 - dist / 820.0)).max(-25.0);
                heights[r * w + c] = elev as f32;
            }
        }
        Guide {
            w,
            h,
            spacing_m: s,
            sea_level_m: 0.0,
            fill_m: None,
            origin_m: [-960.0, 960.0],
            heights,
            regions: None,
        }
    }

    #[test]
    fn cliffs_sharpen_slopes_keep_wet_dry_clean_and_stay_deterministic() {
        let g = hill_guide();
        let mut p = CliffParams::default();
        p.strokes.push(CliffStroke {
            name: "test_scarp".into(),
            from_m: [-400.0, 200.0],
            to_m: [400.0, -200.0],
            throw_m: 80.0,
            wall_width_m: 35.0,
            apron_width_m: 240.0,
            warp_m: 40.0,
            warp_wavelength_m: 200.0,
        });
        let (h1, r1) = apply_cliffs(&g, &p, None).unwrap();
        let (h2, r2) = apply_cliffs(&g, &p, None).unwrap();
        assert!(r1.audit.is_clean(), "wet/dry audit failed: {:?}", r1.audit);
        assert_eq!(r1.blake3, r2.blake3, "must be deterministic across runs");
        assert_eq!(h1, h2);
        assert!(
            r1.after.pct_over_35_deg > r1.before.pct_over_35_deg + 1.0,
            "cliffs should increase steep slope share: before={:.2}%, after={:.2}%",
            r1.before.pct_over_35_deg,
            r1.after.pct_over_35_deg
        );
        assert!(r1.after.p99_deg > r1.before.p99_deg + 5.0);
    }

    /// Tilted plane (8.5 deg) on a 4 km x 4 km grid at 20 m, scarps only (other ops off, no thermal).
    fn tilted_plane_guide() -> Guide {
        let (w, h, s) = (200, 200, 20.0);
        let origin = [-1990.0, 1990.0];
        let mut heights = vec![0.0f32; w * h];
        for r in 0..h {
            for c in 0..w {
                let wx = origin[0] + c as f64 * s;
                heights[r * w + c] = (400.0 + 0.15 * wx) as f32;
            }
        }
        Guide { w, h, spacing_m: s, sea_level_m: 0.0, fill_m: None, origin_m: origin, heights, regions: None }
    }

    /// Max neighbour |dh| on pairs straddling the square `cell_m` lattice lines vs on all other pairs.
    fn seam_vs_rest(g: &Guide, out: &[f32], cell_m: f64) -> (f64, f64) {
        let (mut on, mut off) = (0.0f64, 0.0f64);
        let cell = |v: f64| (v / cell_m).floor() as i64;
        for r in 0..g.h {
            let wy = g.origin_m[1] - r as f64 * g.spacing_m;
            for c in 0..g.w {
                let wx = g.origin_m[0] + c as f64 * g.spacing_m;
                let i = r * g.w + c;
                if c + 1 < g.w {
                    let d = (f64::from(out[i + 1]) - f64::from(out[i])).abs();
                    if cell(wx) == cell(wx + g.spacing_m) { off = off.max(d) } else { on = on.max(d) }
                }
                if r + 1 < g.h {
                    let d = (f64::from(out[i + g.w]) - f64::from(out[i])).abs();
                    if cell(wy) == cell(wy - g.spacing_m) { off = off.max(d) } else { on = on.max(d) }
                }
            }
        }
        (on, off)
    }

    #[test]
    fn scarps_have_no_square_grid_seams() {
        let g = tilted_plane_guide();
        let p = CliffParams {
            unit_cell_m: 400.0,
            scarp_wavelength_m: 2000.0,
            scarp_steepness: 0.55,
            shelf_amp_m: 0.0,
            steepen_gain: 0.0,
            sea_cliff_amp_m: 0.0,
            lava_flow_count: 0,
            thermal_steps: 0,
            ..CliffParams::default()
        };
        let (h1, r1) = apply_cliffs(&g, &p, None).unwrap();
        let (_, r2) = apply_cliffs(&g, &p, None).unwrap();
        assert_eq!(r1.blake3, r2.blake3, "must be deterministic across runs");
        let (on, off) = seam_vs_rest(&g, &h1, p.unit_cell_m);
        assert!(on <= off * 1.5, "square-grid seam: max |dh| on lattice lines {on:.2} m vs elsewhere {off:.2} m");
    }

    #[test]
    fn region_profiles_differentiate_geology_and_route_lava_lobes() {
        let mut g = hill_guide();
        // West half = "Swamp", East half = "Volcano" on a 4x4 region map.
        let mut idx = vec![1u8; 16];
        for r in 0..4 {
            for c in 2..4 {
                idx[r * 4 + c] = 2;
            }
        }
        g.regions = Some(RegionMap {
            w: 4,
            h: 4,
            cell_m: 480.0,
            index: idx,
            names: vec![String::new(), "Swamp".into(), "Volcano".into()],
        });
        let mut p = CliffParams {
            lava_flow_count: 4,
            lava_width_m: 120.0,
            lava_length_m: 600.0,
            ..CliffParams::default()
        };
        p.regions.insert(
            "Swamp".into(),
            RegionProfile { scarp_scale: 0.05, shelf_scale: 0.05, steepen_scale: 0.05, sea_cliff_scale: 0.0, lava_scale: 0.0 },
        );
        p.regions.insert(
            "Volcano".into(),
            RegionProfile { scarp_scale: 1.5, shelf_scale: 1.4, steepen_scale: 1.5, sea_cliff_scale: 1.2, lava_scale: 1.5 },
        );
        let (h, rep) = apply_cliffs(&g, &p, None).unwrap();
        assert!(rep.audit.is_clean(), "wet/dry audit failed: {:?}", rep.audit);
        // Compare mean |dh| on west swamp half vs east volcanic half (dry land).
        let (mut w_sum, mut w_n, mut e_sum, mut e_n) = (0.0f64, 0usize, 0.0f64, 0usize);
        for r in 0..g.h {
            for c in 0..g.w {
                let i = r * g.w + c;
                if g.heights[i] <= 5.0 {
                    continue;
                }
                let diff = (f64::from(h[i]) - f64::from(g.heights[i])).abs();
                if c < g.w / 3 {
                    w_sum += diff;
                    w_n += 1;
                } else if c > 2 * g.w / 3 {
                    e_sum += diff;
                    e_n += 1;
                }
            }
        }
        let w_mean = w_sum / w_n.max(1) as f64;
        let e_mean = e_sum / e_n.max(1) as f64;
        assert!(
            e_mean > w_mean * 2.0,
            "expected volcanic half to sculpt far more than swamp half: east={e_mean:.2} m, west={w_mean:.2} m"
        );
    }
}
