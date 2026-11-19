//! **Composable stratovolcano** (WG-VO): a dramatic volcano stamped onto an existing guide (the base terrain),
//! built from switchable features, plus the radial-profile measurement used to calibrate it on real DEMs.
//!
//! Everything here is a pure function of the world position, the base heights, the distance to the coast and
//! the declared parameters (integer-hash value noise in absolute coordinates, [`crate::perturb`]), except the
//! ravine stage, which runs the existing deterministic stream-power and talus stages. Results never depend on the
//! thread count. Sea samples are never touched and land stays at or above sea level (the coast mask is kept).
//!
//! # Technique
//!
//! **Profile, two parts.** Real stratovolcano flanks are concave: steep near the summit, flattening toward a
//! long low apron (Karátson, Favalli, Tarquini, Fornaciai, Wörner, "The regular shape of stratovolcanoes: a
//! DEM-based morphometrical approach", JVGR 193, 2010, who fit exponential profiles; concavity from erosion and
//! deposition balance, Lacey, Ockendon, Turcotte, "On the geomorphology of volcanoes", EPSL 54, 1981). Here:
//!
//! - the **apron** `A = A_max tanh(G d_c / A_max) w_r(r)`, rising with the distance to the coast `d_c` at the
//!   gradient `G` (so it is 0 on the shore and the coast never moves), faded out radially by `w_r` beyond
//!   `apron_inner_m .. apron_outer_m`. `d_c` is the exact Euclidean distance transform of the wet mask
//!   (Felzenszwalb & Huttenlocher, "Distance transforms of sampled functions", Theory of Computing 8, 2012);
//! - the **edifice** `E = H_e (1 − ρ)^p`, `ρ = r / R(θ)`: a power-law concave cone (slope `p H_e / R` at the top,
//!   0 at the foot, so it meets the apron C1). `p` and the height/radius ratio are calibrated on the measured
//!   radial profiles of real volcanoes ([`radial_profile`], [`fit_profile`]) and then exaggerated;
//! - the base under the edifice is replaced by a reference level (the median base height on the foot ring), and
//!   around it the base relief above that level is compressed by `apron_flatten` (soft knee, C1), so the old
//!   landscape fades into a flat apron and the volcano rises alone ("the vast long views").
//!
//! **Additive mode** (`mode = "additive"`, WG-X): for a base that already has its own mountain (a game's own
//! massif scaled up), the base is **kept** and the volcano is added on top, `h' = h + A(d_c, r) + f_c E(r)`:
//! no reference level, no flattening (`apron_flatten` is ignored), no smooth max or clamp of the top, so the base's
//! own crater, valleys and ridges ride on the new edifice. `E` is the same cone, measured from the rim of an
//! optional **flat top** (`top_flat_m`, a uniform lift that keeps the summit area's relief exactly, joined to the
//! flank by a C1 knee of half-width `top_round_m`); `H_e` is calibrated so the final maximum (base + apron +
//! edifice) is `summit_m`; `f_c` is the coast fade. Where the coast is nearer than the footprint ellipse (a mountain
//! close to the sea), the optional **coast-fitted footprint** (`coast_fit_norm` = n > 0) also measures the cone
//! between the summit and the nearest water, `ρ_c = r_c / (r_c + d_c)` (0 on the flat top, 1 on the shore), and uses
//! the smooth p-norm `ρ = (ρ_e^n + ρ_c^n)^(1/n)` (≥ both): the edifice reaches 0 at every shore with a
//! concave, flattening foot instead of a fade ramp, and its contours follow the coastline. Everything added is ≥ 0 and land is never lowered, sea never
//! touched, so the wet/dry classes cannot change.
//!
//! **Asymmetry.** The footprint `R(θ)` is an ellipse whose centre is offset from the summit (the summit is not over
//! the base centre), times `1 + a n(θ)` with angular value noise; the exponent `p(θ)` varies by flank (steeper and
//! gentler sectors); a low-frequency domain warp of the whole edifice breaks the circular contours (Ebert, Musgrave
//! et al., *Texturing and Modeling*, 3rd ed. 2003, ch. 16, domain distortion).
//!
//! **Features** (each switchable per arm):
//!
//! - *Summit crater*: inside `crater_radius_m` the surface is cut to the rim level (the edifice evaluated on the rim
//!   along the same ray) minus a bowl, so the rim is the summit.
//! - *Caldera* (Teide's Las Cañadas): an older collapse rim of radius `R_k` around an offset centre; inside it the old
//!   edifice is truncated to a floor `wall_m` below the rim, with a steep wall; a younger, steeper cone rises from the
//!   floor to the summit (smooth max, Quílez polynomial smin); one sector is open (buried by younger lavas), the
//!   wall fading out across `caldera_open_fade_deg`.
//! - *Secondary summit* (Pico Viejo): a second cone with its own crater, merged by smooth max.
//! - *Sub-peaks*: a few hashed, radially elongated knobs added to the upper flanks.
//! - *Flank collapse* (Etna's Valle del Bove): a horseshoe scar, a rounded head and two side walls of signed-distance
//!   shape (noise-warped wall line), depth constant to `collapse_depth_fade` of its length, then fading to 0 at the
//!   toe, so it opens downslope.
//! - *Rift zones*: 2-3 curved lines from the upper cone into the apron, each a low ridge with aligned parasitic
//!   cinder cones of log-uniform basal width `W`, height `0.18 W` and crater width `0.40 W` (Porter, "Distribution,
//!   morphology, and size frequency of cinder cones on Mauna Kea volcano, Hawaii", GSA Bulletin 83, 1972; Wood,
//!   "Morphometric evolution of cinder cones", JVGR 7, 1980).
//!
//! **Ravines (barrancos).** Light stream-power erosion of the edifice with the existing implicit solver
//! ([`crate::stream::erode`], Braun & Willett 2013; MFD drainage area, Freeman 1991 / Holmgren 1994) and no uplift,
//! with **heterogeneous erodibility**, which is what makes channel networks irregular rather than evenly spaced
//! (Perron, Dietrich, Kirchner, "Controls on the spacing of first-order valleys", JGR 113, 2008; Perron, Kirchner,
//! Dietrich, "Formation of evenly spaced ridges and valleys", Nature 460, 2009, for why uniform K gives regular
//! spacing): K varies by layered rock bands in elevation (alternating lava and pyroclastic beds), by cellular
//! patches (Worley, "A cellular texture basis function", SIGGRAPH 1996: lava-flow fields of different age) and by
//! flank (older flanks are dissected deeper, Tenerife's barrancos vs Teide's young cone). A few seeded **fracture
//! lines** (hashed, wandering, mostly downslope) are notched into the surface and given a higher K, so they capture
//! flow and grow into the deep barrancos; a two-wavelength seed noise lets small catchments organise into branching
//! networks. A talus relaxation ([`crate::thermal::relax`], Musgrave et al. 1989) then widens the cut channels into
//! steep V-shaped ravines.
//!
//! Every parameter is declared in the params file with its unit, range and source ([`Param`]); nothing here names
//! a game.

use std::collections::BTreeMap;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::guide::Guide;
use crate::perturb::{lattice, value_noise};
use crate::stream::{Flow, StreamParams, erode};
use crate::thermal::{ThermalParams, relax};
use crate::volcano::smootherstep;
use crate::{Audit, Error, Grid, Result, WetDry, hash_f32};

#[cfg(test)]
#[path = "tests_stratovolcano.rs"]
mod tests;

/// Hash stream ids (added to the seed; ids, not tunables).
// tunables: const (hash stream ids)
const S_WARP_X: u64 = 1;
const S_WARP_Y: u64 = 2;
const S_FOOT: u64 = 3;
const S_EXPONENT: u64 = 4;
const S_RIFT_CURVE: u64 = 5;
const S_CALDERA: u64 = 6;
const S_COLLAPSE: u64 = 7;
const S_SUBPEAK: u64 = 8;
const S_CONE: u64 = 9;
const S_FRACTURE: u64 = 10;
const S_FRACTURE_WANDER: u64 = 11;
const S_SEED_COARSE: u64 = 12;
const S_SEED_FINE: u64 = 13;
const S_BAND: u64 = 14;
const S_BAND_WARP: u64 = 15;
const S_CELL: u64 = 16;
const S_FLANK_K: u64 = 17;

/// Height stand-in for "no surface" in smooth max (maths: below any height).
const NONE: f64 = f64::NEG_INFINITY;
/// Distance stand-in for dry samples in the distance transform, samples² (maths: larger than any grid distance²).
const EDT_FAR: f64 = 1e20;
/// Summit calibration: secant steps and stop tolerance, metres (maths: converges in 3-5 steps).
const CALIB_STEPS: usize = 16;
const CALIB_TOL_M: f64 = 1e-3;
/// Summit calibration window, share of the mean foot radius (layout: the summit lies within it by construction).
const CALIB_WINDOW: f64 = 0.5;
/// Fracture polyline segments (layout: smooth enough at any declared length).
const FRACTURE_SEGMENTS: usize = 32;
/// Apron measurement: rays and shortest usable segment, metres (report layout).
const APRON_RAYS: usize = 64;
const APRON_MIN_LEN_M: f64 = 2000.0;
/// Footprint bearings sampled to bound the edifice (layout).
const FOOT_BEARINGS: usize = 720;
/// Profile fit resolution (measurement layout): power exponent grid, exponential decay grid.
const FIT_P: [f64; 3] = [0.3, 5.0, 0.01];
const FIT_R0_STEPS: usize = 200;
/// Channel-structure measurement on the edifice window (report layout, as the S4 recipe: channels from 0.1 km²,
/// 1 km coherence window, direction over 5 receivers).
const STRUCTURE_THRESHOLD_M2: f64 = 1e5;
const STRUCTURE_WINDOW_M: f64 = 1000.0;
const STRUCTURE_DIR_STEPS: usize = 5;
/// Flank-slope measurement rings around the final summit, metres (report layout: WG-X brief, 5 / 10 / 15 km).
// tunables: const (report layout)
const FLANK_RINGS_M: [f64; 3] = [5000.0, 10000.0, 15000.0];
/// Guide side files copied from the base into every arm (format: guide v1 optional files; same grid, same frame).
const GUIDE_SIDE_FILES: [&str; 3] = ["regions.png", "regions.json", "markers.json"];
/// Hillshade preview light and sea gray (preview layout, as the other worldgen previews).
const PREVIEW_LIGHT: crate::shade::Light = crate::shade::Light {
    azimuth_deg: 315.0,
    altitude_deg: 45.0,
    sea_gray: 40,
};

// ---------------------------------------------------------------------------------------------------------------
// Parameters

/// One declared value (spec `tunables.md` "Definition": value, unit, range, source).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub v: f64,
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub source: String,
    #[serde(default)]
    pub help: String,
}

/// Every parameter the params file must declare.
pub const PARAM_NAMES: &[&str] = &[
    "seed",
    "summit_m",
    "ref_quantile",
    "foot_major_m",
    "foot_minor_m",
    "foot_major_bearing_deg",
    "foot_offset_m",
    "foot_offset_bearing_deg",
    "foot_warp",
    "foot_warp_wavelength_m",
    "domain_warp_m",
    "domain_warp_wavelength_m",
    "cone_exponent",
    "cone_exponent_var",
    "cone_exponent_wavelength_m",
    "crater_radius_m",
    "crater_depth_m",
    "blend_in",
    "blend_out",
    "coast_fade_m",
    "apron_gradient",
    "apron_max_m",
    "apron_inner_m",
    "apron_outer_m",
    "apron_flatten",
    "flatten_knee_m",
    "smooth_max_m",
    "caldera_radius_m",
    "caldera_offset_m",
    "caldera_offset_bearing_deg",
    "caldera_wall_m",
    "caldera_wall_width_m",
    "caldera_warp",
    "caldera_warp_wavelength_m",
    "caldera_open_bearing_deg",
    "caldera_open_half_deg",
    "caldera_open_fade_deg",
    "young_radius_m",
    "young_exponent",
    "secondary_distance_m",
    "secondary_bearing_deg",
    "secondary_height_ratio",
    "secondary_radius_m",
    "secondary_exponent",
    "secondary_crater_radius_m",
    "secondary_crater_depth_m",
    "subpeak_count",
    "subpeak_r_min",
    "subpeak_r_max",
    "subpeak_h_min_m",
    "subpeak_h_max_m",
    "subpeak_radius_min_m",
    "subpeak_radius_max_m",
    "subpeak_aspect",
    "collapse_bearing_deg",
    "collapse_head",
    "collapse_toe",
    "collapse_width_head_m",
    "collapse_width_toe_m",
    "collapse_depth_m",
    "collapse_depth_fade",
    "collapse_wall_m",
    "collapse_wall_warp_m",
    "collapse_wall_warp_wavelength_m",
    "rift_count",
    "rift_1_bearing_deg",
    "rift_2_bearing_deg",
    "rift_3_bearing_deg",
    "rift_start",
    "rift_end",
    "rift_taper",
    "rift_curve_deg",
    "rift_curve_wavelength_m",
    "rift_ridge_m",
    "rift_ridge_width_m",
    "rift_cones",
    "rift_cone_lateral_m",
    "cone_width_min_m",
    "cone_width_max_m",
    "cone_height_ratio",
    "cone_crater_ratio",
    "cone_crater_depth_ratio",
    "cone_knee",
    "ravine_steps",
    "ravine_k_dt",
    "ravine_m",
    "ravine_flow_exponent",
    "ravine_seed_m",
    "ravine_seed_wavelength_m",
    "ravine_seed_fine_m",
    "ravine_seed_fine_wavelength_m",
    "flank_k_var",
    "flank_k_wavelength_m",
    "band_thickness_m",
    "band_edge",
    "band_k_var",
    "band_warp_m",
    "cell_size_m",
    "cell_k_var",
    "fracture_count",
    "fracture_start_min",
    "fracture_start_max",
    "fracture_len_min",
    "fracture_len_max",
    "fracture_angle_deg",
    "fracture_wander_wavelength_m",
    "fracture_depth_m",
    "fracture_width_m",
    "fracture_taper",
    "fracture_k_boost",
    "ravine_talus_deg",
    "ravine_thermal_steps",
    "ravine_thermal_rate",
    "detail_spacing_m",
    "detail_crop_m",
    "fit_bin_m",
    "fit_max_r_m",
];

/// Optional parameters: required in [`Mode::Additive`], allowed (and read, absent = 0) in [`Mode::Replace`].
pub const OPTIONAL_PARAM_NAMES: &[&str] = &["top_flat_m", "top_round_m", "coast_fit_norm"];

/// How the volcano meets the base.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// The edifice replaces the base inside `blend_in`, the old relief is flattened under the apron (WG-VO).
    #[default]
    Replace,
    /// The base is kept and the apron and edifice are added on top (WG-X: the base already has its mountain).
    Additive,
}

/// Feature switches of one arm.
#[allow(clippy::struct_excessive_bools)] // one switch per feature: the arms are defined by them
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Features {
    #[serde(default)]
    pub caldera: bool,
    #[serde(default)]
    pub secondary: bool,
    #[serde(default)]
    pub subpeaks: bool,
    #[serde(default)]
    pub collapse: bool,
    #[serde(default)]
    pub rifts: bool,
    #[serde(default)]
    pub ravines: bool,
}

/// One arm of the params file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmSpec {
    pub name: String,
    #[serde(default)]
    pub help: String,
    #[serde(default)]
    pub features: Features,
    /// Runtime-detail preview of another arm's output ([`crate::filter`]) instead of a stamp.
    pub detail_from: Option<String>,
    /// Per-arm values of declared parameters (each inside its declared range), e.g. `{ summit_m = 4500 }`.
    #[serde(default)]
    pub set: BTreeMap<String, f64>,
}

/// Where the summit goes: an explicit position, or the highest base sample inside a region of a region guide.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreSpec {
    /// Summit position, world metres (x east, y north).
    pub explicit_m: Option<[f64; 2]>,
    /// Guide folder holding `regions.png` + `regions.json`.
    pub region_guide: Option<PathBuf>,
    /// Legend name of the region.
    pub region: Option<String>,
    /// Region guide frame metres of its sample (0, 0) (format: the guide's frame).
    #[serde(default)]
    pub region_origin_m: [f64; 2],
    /// World metres per region-guide metre (format: the frame scale the base was made at).
    pub region_scale: Option<f64>,
}

/// Real volcano guides whose radial profiles are measured for the calibration table.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceSpec {
    #[serde(default)]
    pub guides: Vec<PathBuf>,
    /// Guides in the base frame whose channel structure is measured on the same edifice window (e.g. an earlier
    /// volcano arm), for comparison.
    #[serde(default)]
    pub structure_guides: Vec<PathBuf>,
}

/// A `stamp-volcano` params file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    /// Replace (default) or additive.
    #[serde(default)]
    pub mode: Mode,
    #[serde(default)]
    pub centre: CentreSpec,
    #[serde(default)]
    pub reference: ReferenceSpec,
    pub params: BTreeMap<String, Param>,
    #[serde(rename = "arm")]
    pub arms: Vec<ArmSpec>,
}

impl Spec {
    /// Parses and validates a params file.
    pub fn parse(text: &str) -> Result<Self> {
        let s: Self = toml::from_str(text)?;
        s.validate()?;
        Ok(s)
    }

    /// Checks names, sources, ranges and orderings.
    pub fn validate(&self) -> Result<()> {
        for name in PARAM_NAMES {
            if !self.params.contains_key(*name) {
                return Err(Error::Invalid(format!("param {name} missing")));
            }
        }
        if self.mode == Mode::Additive {
            for name in OPTIONAL_PARAM_NAMES {
                if !self.params.contains_key(*name) {
                    return Err(Error::Invalid(format!("param {name} missing (required in additive mode)")));
                }
            }
        }
        for (name, p) in &self.params {
            if !PARAM_NAMES.contains(&name.as_str()) && !OPTIONAL_PARAM_NAMES.contains(&name.as_str()) {
                return Err(Error::Invalid(format!("unknown param {name}")));
            }
            crate::recipe::check_source(name, &p.source)?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
        }
        let v = |n: &str| self.v(n);
        let ordered = [
            ("blend_in", "blend_out"),
            ("apron_inner_m", "apron_outer_m"),
            ("subpeak_r_min", "subpeak_r_max"),
            ("subpeak_h_min_m", "subpeak_h_max_m"),
            ("subpeak_radius_min_m", "subpeak_radius_max_m"),
            ("collapse_head", "collapse_toe"),
            ("rift_start", "rift_end"),
            ("cone_width_min_m", "cone_width_max_m"),
            ("fracture_start_min", "fracture_start_max"),
            ("fracture_len_min", "fracture_len_max"),
            ("crater_radius_m", "foot_minor_m"),
            ("young_radius_m", "foot_minor_m"),
        ];
        for (a, b) in ordered {
            if v(a) >= v(b) {
                return Err(Error::Invalid(format!("{a} < {b} required")));
            }
        }
        if v("foot_minor_m") > v("foot_major_m") || v("foot_offset_m") >= v("foot_minor_m") {
            return Err(Error::Invalid(
                "foot_minor_m <= foot_major_m and foot_offset_m < foot_minor_m required (summit inside)".into(),
            ));
        }
        for n in [
            "seed",
            "subpeak_count",
            "rift_count",
            "rift_cones",
            "fracture_count",
            "ravine_steps",
            "ravine_thermal_steps",
        ] {
            if v(n).fract() != 0.0 {
                return Err(Error::Invalid(format!("{n} must be an integer")));
            }
        }
        let mut names: Vec<&str> = self.arms.iter().map(|a| a.name.as_str()).collect();
        names.sort_unstable();
        if names.windows(2).any(|w| w[0] == w[1]) || names.is_empty() {
            return Err(Error::Invalid("arm names must be unique and non-empty".into()));
        }
        for a in &self.arms {
            for name in a.set.keys() {
                if !self.params.contains_key(name) {
                    return Err(Error::Invalid(format!("arm {}: set {name} is not a declared param", a.name)));
                }
            }
            if !a.set.is_empty() {
                self.for_arm(a)?;
            }
            if let Some(src) = &a.detail_from
                && !self.arms.iter().any(|b| &b.name == src && b.detail_from.is_none())
            {
                return Err(Error::Invalid(format!(
                    "arm {}: detail_from {src} is not a stamp arm",
                    a.name
                )));
            }
        }
        Ok(())
    }

    /// Value of a declared parameter (NaN when missing; [`Spec::validate`] rules that out).
    pub fn v(&self, name: &str) -> f64 {
        self.params.get(name).map_or(f64::NAN, |p| p.v)
    }

    /// Value of an optional parameter (0 when not declared, its neutral value).
    pub fn v_opt(&self, name: &str) -> f64 {
        self.params.get(name).map_or(0.0, |p| p.v)
    }

    /// The spec with `arm.set` applied (each value range-checked; orderings re-validated).
    pub fn for_arm(&self, arm: &ArmSpec) -> Result<Self> {
        if arm.set.is_empty() {
            return Ok(self.clone());
        }
        let mut s = self.clone();
        for (name, &v) in &arm.set {
            let p = s
                .params
                .get_mut(name)
                .ok_or_else(|| Error::Invalid(format!("arm {}: set {name} is not a declared param", arm.name)))?;
            crate::recipe::check_range(name, v, p.min, p.max)?;
            p.v = v;
        }
        // The overrides are applied: validate the orderings without recursing into the arms.
        s.arms.clear();
        s.arms.push(ArmSpec {
            set: BTreeMap::new(),
            detail_from: None,
            ..arm.clone()
        });
        s.validate()?;
        s.arms.clone_from(&self.arms);
        Ok(s)
    }

    /// Count of parameters whose source is `Guess`.
    pub fn guess_count(&self) -> usize {
        self.params.values().filter(|p| p.source == "Guess").count()
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Small maths

#[inline]
fn dir(bearing: f64) -> [f64; 2] {
    [bearing.sin(), bearing.cos()]
}

/// Compass bearing (0 = north, clockwise) of a vector.
#[inline]
fn bearing_of(d: [f64; 2]) -> f64 {
    d[0].atan2(d[1])
}

#[inline]
fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline]
fn dot(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

#[inline]
fn len(a: [f64; 2]) -> f64 {
    a[0].hypot(a[1])
}

/// Signed angle a − b wrapped to (−π, π].
#[inline]
fn ang_diff(a: f64, b: f64) -> f64 {
    let d = (a - b).rem_euclid(TAU);
    if d > PI { d - TAU } else { d }
}

/// Uniform [0, 1) from the lattice hash.
#[inline]
fn u01(i: i64, k: i64, seed: u64) -> f64 {
    f64::midpoint(lattice(i, k, seed), 1.0)
}

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Polynomial smooth maximum with blend width `k` (Quílez, "smooth minimum", 2013): C1, equal to `max` once the
/// two differ by more than `k`.
#[inline]
fn smax(a: f64, b: f64, k: f64) -> f64 {
    if k <= 0.0 || !a.is_finite() || !b.is_finite() {
        return a.max(b);
    }
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.max(b) + h * h * k * 0.25
}

/// C1 soft ramp: 0 below −k, x above k, a quadratic knee between.
#[inline]
fn soft_ramp(x: f64, k: f64) -> f64 {
    if x <= -k {
        0.0
    } else if x >= k {
        x
    } else {
        (x + k) * (x + k) / (4.0 * k)
    }
}

/// C1 clamp of x to [0, 1] with quadratic knees of half-width k at both ends.
#[inline]
fn knee_clamp01(x: f64, k: f64) -> f64 {
    if k <= 0.0 {
        return x.clamp(0.0, 1.0);
    }
    if x >= 0.5 {
        1.0 - soft_ramp(1.0 - x, k)
    } else {
        soft_ramp(x, k)
    }
}

/// Smooth bowl `(1 − t²)²` on t ∈ [0, 1], 0 beyond.
#[inline]
fn bowl(t: f64) -> f64 {
    if t >= 1.0 {
        0.0
    } else {
        let u = 1.0 - t * t;
        u * u
    }
}

/// Value noise on a circle of `radius` around `centre`, read at bearing `theta` (periodic in θ, absolute coords).
#[inline]
fn ang_noise(centre: [f64; 2], radius: f64, theta: f64, wavelength: f64, seed: u64) -> f64 {
    let d = dir(theta);
    value_noise(
        [centre[0] + radius * d[0], centre[1] + radius * d[1]],
        wavelength,
        seed,
    )
}

// ---------------------------------------------------------------------------------------------------------------
// Shape

/// The caldera feature, resolved.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Caldera {
    pub centre_m: [f64; 2],
    pub radius_m: f64,
    pub wall_m: f64,
    pub wall_width_m: f64,
    pub warp: f64,
    pub warp_wavelength_m: f64,
    pub open_bearing: f64,
    pub open_half: f64,
    pub open_fade: f64,
    pub young_radius_m: f64,
    pub young_exponent: f64,
}

/// The secondary summit, resolved (`peak_m` above the edifice's zero, i.e. above reference + apron).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Secondary {
    pub centre_m: [f64; 2],
    pub peak_m: f64,
    pub radius_m: f64,
    pub exponent: f64,
    pub crater_radius_m: f64,
    pub crater_depth_m: f64,
}

/// One sub-peak knob.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SubPeak {
    pub centre_m: [f64; 2],
    pub height_m: f64,
    /// Radial and tangential radii, metres.
    pub radius_m: [f64; 2],
    /// Unit radial direction.
    pub axis: [f64; 2],
}

/// One cinder cone.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Cinder {
    pub centre_m: [f64; 2],
    pub base_radius_m: f64,
    pub top_radius_m: f64,
    pub height_m: f64,
    pub crater_depth_m: f64,
}

/// One rift zone.
#[derive(Debug, Clone, Serialize)]
pub struct Rift {
    pub bearing: f64,
    pub index: i64,
    pub start_m: f64,
    pub end_m: f64,
    pub cones: Vec<Cinder>,
}

/// The flank collapse scar, resolved.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Collapse {
    pub dir: [f64; 2],
    pub head_m: f64,
    pub toe_m: f64,
    pub width_head_m: f64,
    pub width_toe_m: f64,
    pub depth_m: f64,
    pub depth_fade: f64,
    pub wall_m: f64,
    pub wall_warp_m: f64,
    pub wall_warp_wavelength_m: f64,
}

/// The resolved volcano: every feature at its place, the edifice height calibrated to the summit target.
#[derive(Debug, Clone, Serialize)]
pub struct Shape {
    pub summit_m: [f64; 2],
    pub sea_level_m: f64,
    /// Reference level the edifice stands on, metres.
    pub ref_m: f64,
    /// Edifice height `H_e` above reference + apron, metres (calibrated).
    pub edifice_m: f64,
    pub features: Features,
    pub foot_centre_m: [f64; 2],
    /// Semi-axes (major, minor), metres, and the unit major axis.
    pub foot_axes_m: [f64; 2],
    pub foot_axis: [f64; 2],
    /// Mean of the semi-axes, metres.
    pub r_mean_m: f64,
    /// Largest footprint radius times `blend_out`, plus the warp: nothing of the edifice lies beyond.
    pub edifice_support_m: f64,
    pub support_m: f64,
    pub mode: Mode,
    /// Flat-top radius and its knee half-width, metres (the cone is measured from `top_flat + top_round`).
    top_flat: f64,
    top_round: f64,
    /// p-norm exponent of the coast-fitted footprint (0 = off).
    coast_fit: f64,
    seed: u64,
    foot_warp: f64,
    foot_warp_wl: f64,
    warp_m: f64,
    warp_wl: f64,
    p0: f64,
    p_var: f64,
    p_wl: f64,
    crater_r: f64,
    crater_d: f64,
    blend: [f64; 2],
    coast_fade: f64,
    apron_g: f64,
    apron_max: f64,
    apron_r: [f64; 2],
    flatten: f64,
    knee: f64,
    smooth_k: f64,
    pub caldera: Caldera,
    pub secondary: Secondary,
    pub subpeaks: Vec<SubPeak>,
    pub rifts: Vec<Rift>,
    pub collapse: Collapse,
    rift_taper: f64,
    rift_curve: f64,
    rift_curve_wl: f64,
    rift_ridge_m: f64,
    rift_ridge_w: f64,
    cone_knee: f64,
}

impl Shape {
    /// Resolves the shape at `summit_m`. `under_summit_m` is reference + apron at the summit (it places the
    /// secondary summit's absolute height); `edifice_m` is the first guess of `H_e` (calibrated later).
    #[allow(clippy::too_many_lines)] // a flat mapping of the declared values onto the shape
    pub fn new(
        spec: &Spec,
        features: Features,
        summit_m: [f64; 2],
        sea_level_m: f64,
        ref_m: f64,
        under_summit_m: f64,
    ) -> Self {
        let v = |n: &str| spec.v(n);
        let deg = |n: &str| spec.v(n).to_radians();
        let seed = v("seed") as u64;
        let offset = dir(deg("foot_offset_bearing_deg"));
        let foot_centre_m = [
            summit_m[0] + v("foot_offset_m") * offset[0],
            summit_m[1] + v("foot_offset_m") * offset[1],
        ];
        let foot_axes_m = [v("foot_major_m"), v("foot_minor_m")];
        let r_mean = f64::midpoint(foot_axes_m[0], foot_axes_m[1]);
        let ck = dir(deg("caldera_offset_bearing_deg"));
        let caldera = Caldera {
            centre_m: [
                summit_m[0] + v("caldera_offset_m") * ck[0],
                summit_m[1] + v("caldera_offset_m") * ck[1],
            ],
            radius_m: v("caldera_radius_m"),
            wall_m: v("caldera_wall_m"),
            wall_width_m: v("caldera_wall_width_m"),
            warp: v("caldera_warp"),
            warp_wavelength_m: v("caldera_warp_wavelength_m"),
            open_bearing: deg("caldera_open_bearing_deg"),
            open_half: deg("caldera_open_half_deg"),
            open_fade: deg("caldera_open_fade_deg"),
            young_radius_m: v("young_radius_m"),
            young_exponent: v("young_exponent"),
        };
        let sd = dir(deg("secondary_bearing_deg"));
        let secondary = Secondary {
            centre_m: [
                summit_m[0] + v("secondary_distance_m") * sd[0],
                summit_m[1] + v("secondary_distance_m") * sd[1],
            ],
            peak_m: (v("secondary_height_ratio") * v("summit_m") - under_summit_m).max(0.0),
            radius_m: v("secondary_radius_m"),
            exponent: v("secondary_exponent"),
            crater_radius_m: v("secondary_crater_radius_m"),
            crater_depth_m: v("secondary_crater_depth_m"),
        };
        let cd = dir(deg("collapse_bearing_deg"));
        let collapse = Collapse {
            dir: cd,
            head_m: v("collapse_head") * r_mean,
            toe_m: v("collapse_toe") * r_mean,
            width_head_m: v("collapse_width_head_m"),
            width_toe_m: v("collapse_width_toe_m"),
            depth_m: v("collapse_depth_m"),
            depth_fade: v("collapse_depth_fade"),
            wall_m: v("collapse_wall_m"),
            wall_warp_m: v("collapse_wall_warp_m"),
            wall_warp_wavelength_m: v("collapse_wall_warp_wavelength_m"),
        };
        let mut s = Self {
            summit_m,
            sea_level_m,
            ref_m,
            edifice_m: (v("summit_m") - under_summit_m).max(1.0),
            features,
            foot_centre_m,
            foot_axes_m,
            foot_axis: dir(deg("foot_major_bearing_deg")),
            r_mean_m: r_mean,
            edifice_support_m: 0.0,
            support_m: 0.0,
            mode: spec.mode,
            top_flat: spec.v_opt("top_flat_m"),
            top_round: spec.v_opt("top_round_m"),
            coast_fit: spec.v_opt("coast_fit_norm"),
            seed,
            foot_warp: v("foot_warp"),
            foot_warp_wl: v("foot_warp_wavelength_m"),
            warp_m: v("domain_warp_m"),
            warp_wl: v("domain_warp_wavelength_m"),
            p0: v("cone_exponent"),
            p_var: v("cone_exponent_var"),
            p_wl: v("cone_exponent_wavelength_m"),
            crater_r: v("crater_radius_m"),
            crater_d: v("crater_depth_m"),
            blend: [v("blend_in"), v("blend_out")],
            coast_fade: v("coast_fade_m"),
            apron_g: v("apron_gradient"),
            apron_max: v("apron_max_m"),
            apron_r: [v("apron_inner_m"), v("apron_outer_m")],
            flatten: v("apron_flatten"),
            knee: v("flatten_knee_m"),
            smooth_k: v("smooth_max_m"),
            caldera,
            secondary,
            subpeaks: Vec::new(),
            rifts: Vec::new(),
            collapse,
            rift_taper: v("rift_taper"),
            rift_curve: deg("rift_curve_deg"),
            rift_curve_wl: v("rift_curve_wavelength_m"),
            rift_ridge_m: v("rift_ridge_m"),
            rift_ridge_w: v("rift_ridge_width_m"),
            cone_knee: v("cone_knee"),
        };
        s.subpeaks = s.make_subpeaks(spec);
        s.rifts = s.make_rifts(spec);
        let foot_max = (0..FOOT_BEARINGS)
            .map(|k| s.foot_radius(TAU * k as f64 / FOOT_BEARINGS as f64))
            .fold(0.0, f64::max);
        s.edifice_support_m = foot_max * s.blend[1] + 2.0 * s.warp_m + s.top_flat + 2.0 * s.top_round;
        let rift_reach = s
            .rifts
            .iter()
            .flat_map(|r| r.cones.iter())
            .map(|c| {
                len(sub(c.centre_m, s.summit_m)) + c.base_radius_m * (1.0 + s.cone_knee)
            })
            .chain(s.rifts.iter().map(|r| r.end_m + 3.0 * s.rift_ridge_w))
            .fold(0.0, f64::max);
        s.support_m = s
            .edifice_support_m
            .max(s.apron_r[1] + 2.0 * s.warp_m)
            .max(rift_reach + 2.0 * s.warp_m);
        s
    }

    fn make_subpeaks(&self, spec: &Spec) -> Vec<SubPeak> {
        let v = |n: &str| spec.v(n);
        let seed = self.seed + S_SUBPEAK;
        (0..v("subpeak_count") as i64)
            .map(|i| {
                let b = TAU * u01(i, 0, seed);
                let d = lerp(v("subpeak_r_min"), v("subpeak_r_max"), u01(i, 1, seed)) * self.r_mean_m;
                let axis = dir(b);
                let rad = lerp(
                    v("subpeak_radius_min_m"),
                    v("subpeak_radius_max_m"),
                    u01(i, 3, seed),
                );
                SubPeak {
                    centre_m: [self.summit_m[0] + d * axis[0], self.summit_m[1] + d * axis[1]],
                    height_m: lerp(v("subpeak_h_min_m"), v("subpeak_h_max_m"), u01(i, 2, seed)),
                    radius_m: [rad * v("subpeak_aspect"), rad],
                    axis,
                }
            })
            .collect()
    }

    fn make_rifts(&self, spec: &Spec) -> Vec<Rift> {
        let v = |n: &str| spec.v(n);
        let names = ["rift_1_bearing_deg", "rift_2_bearing_deg", "rift_3_bearing_deg"];
        let count = (v("rift_count") as usize).min(names.len());
        let seed = self.seed + S_CONE;
        let (start, end) = (v("rift_start") * self.r_mean_m, v("rift_end") * self.r_mean_m);
        let (wmin, wmax) = (v("cone_width_min_m").ln(), v("cone_width_max_m").ln());
        let n = v("rift_cones") as i64;
        names[..count]
            .iter()
            .enumerate()
            .map(|(j, name)| {
                let mut rift = Rift {
                    bearing: v(name).to_radians(),
                    index: j as i64,
                    start_m: start,
                    end_m: end,
                    cones: Vec::new(),
                };
                rift.cones = (0..n)
                    .map(|k| {
                        let key = j as i64 * 1000 + k;
                        let t = (k as f64 + 0.5 + 0.8 * (u01(key, 0, seed) - 0.5)) / n as f64;
                        let u = lerp(start, end, t);
                        let b = self.rift_bearing(&rift, u);
                        let (a, p) = (dir(b), dir(b + FRAC_PI_2));
                        let lat = v("rift_cone_lateral_m") * (2.0 * u01(key, 1, seed) - 1.0);
                        let w = lerp(wmin, wmax, u01(key, 2, seed)).exp();
                        let base_r = 0.5 * w;
                        let h = v("cone_height_ratio") * w;
                        Cinder {
                            centre_m: [
                                self.summit_m[0] + u * a[0] + lat * p[0],
                                self.summit_m[1] + u * a[1] + lat * p[1],
                            ],
                            base_radius_m: base_r,
                            top_radius_m: v("cone_crater_ratio") * base_r,
                            height_m: h,
                            crater_depth_m: v("cone_crater_depth_ratio") * h,
                        }
                    })
                    .collect();
                rift
            })
            .collect()
    }

    /// Domain-warped position (the edifice is evaluated there).
    #[inline]
    pub fn warp(&self, p: [f64; 2]) -> [f64; 2] {
        if self.warp_m <= 0.0 {
            return p;
        }
        [
            p[0] + self.warp_m * value_noise(p, self.warp_wl, self.seed + S_WARP_X),
            p[1] + self.warp_m * value_noise(p, self.warp_wl, self.seed + S_WARP_Y),
        ]
    }

    /// Footprint radius R(θ) from the summit at bearing θ: the offset ellipse, times angular noise.
    pub fn foot_radius(&self, theta: f64) -> f64 {
        let u = dir(theta);
        let (ax, ay) = (self.foot_axis, [-self.foot_axis[1], self.foot_axis[0]]);
        let [a, b] = self.foot_axes_m;
        let g = sub(self.summit_m, self.foot_centre_m);
        let (gu, uu) = ([dot(g, ax) / a, dot(g, ay) / b], [dot(u, ax) / a, dot(u, ay) / b]);
        let (qa, qb, qc) = (dot(uu, uu), dot(gu, uu), dot(gu, gu) - 1.0);
        let t = (-qb + (qb * qb - qa * qc).max(0.0).sqrt()) / qa;
        t * (1.0 + self.foot_warp * ang_noise(self.summit_m, self.r_mean_m, theta, self.foot_warp_wl, self.seed + S_FOOT))
    }

    /// Flank exponent p(θ), at least 1 (maths: a finite foot slope).
    pub fn exponent(&self, theta: f64) -> f64 {
        (self.p0
            * (1.0
                + self.p_var
                    * ang_noise(self.summit_m, self.r_mean_m, theta, self.p_wl, self.seed + S_EXPONENT)))
            .max(1.0)
    }

    /// Cone distance: 0 on the flat top (`r <= top_flat`), a C1 quadratic knee over `2 top_round`, then
    /// `r - top_flat - top_round` (with both 0 it is `r`).
    #[inline]
    fn cone_r(&self, r: f64) -> f64 {
        soft_ramp(r - self.top_flat - self.top_round, self.top_round)
    }

    /// Main cone + caldera + secondary summit at warped position `q` with coast distance `d_coast` (infinite =
    /// no coast fit); returns (E, ρ).
    fn core(&self, q: [f64; 2], d_coast: f64) -> (f64, f64) {
        let d = sub(q, self.summit_m);
        let r = len(d);
        let th = bearing_of(d);
        let rc = self.cone_r(r);
        let mut rho = rc / self.foot_radius(th);
        if self.coast_fit > 0.0 && d_coast.is_finite() && rc > 0.0 {
            let rho_c = rc / (rc + d_coast.max(0.0));
            let n = self.coast_fit;
            rho = (rho.powf(n) + rho_c.powf(n)).powf(n.recip());
        }
        let mut e = if rho < 1.0 {
            self.edifice_m * (1.0 - rho).powf(self.exponent(th))
        } else {
            0.0
        };
        if self.features.caldera {
            e = self.caldera_at(q, r, e);
        }
        if self.features.secondary {
            e = smax(e, self.secondary_at(q), self.smooth_k);
        }
        (e, rho)
    }

    fn caldera_at(&self, q: [f64; 2], r: f64, e: f64) -> f64 {
        let c = &self.caldera;
        let dk = sub(q, c.centre_m);
        let (rk, thk) = (len(dk), bearing_of(dk));
        let rad = c.radius_m
            * (1.0 + c.warp * ang_noise(c.centre_m, c.radius_m, thk, c.warp_wavelength_m, self.seed + S_CALDERA));
        let floor = self.edifice_m * (1.0 - c.radius_m / self.r_mean_m).max(0.0).powf(self.p0) - c.wall_m;
        let s = smootherstep((rk - (rad - c.wall_width_m)) / c.wall_width_m);
        let cut = e.min(floor + s * (e - floor));
        let young = if r < c.young_radius_m {
            floor + (self.edifice_m - floor) * (1.0 - r / c.young_radius_m).powf(c.young_exponent)
        } else {
            floor - (r - c.young_radius_m)
        };
        let inside = smax(cut, young, self.smooth_k);
        let closed = smootherstep((ang_diff(thk, c.open_bearing).abs() - c.open_half) / c.open_fade);
        e + closed * (inside - e)
    }

    fn secondary_at(&self, q: [f64; 2]) -> f64 {
        let s = &self.secondary;
        let r2 = len(sub(q, s.centre_m));
        if r2 >= s.radius_m {
            return NONE;
        }
        if r2 < s.crater_radius_m {
            let rim = s.peak_m * (1.0 - s.crater_radius_m / s.radius_m).powf(s.exponent);
            rim - s.crater_depth_m * bowl(r2 / s.crater_radius_m)
        } else {
            s.peak_m * (1.0 - r2 / s.radius_m).powf(s.exponent)
        }
    }

    fn rift_bearing(&self, rift: &Rift, u: f64) -> f64 {
        rift.bearing
            + self.rift_curve
                * value_noise(
                    [u, rift.index as f64 * 1e5],
                    self.rift_curve_wl,
                    self.seed + S_RIFT_CURVE,
                )
    }

    fn rifts_at(&self, q: [f64; 2]) -> f64 {
        let d = sub(q, self.summit_m);
        let (r, th) = (len(d), bearing_of(d));
        let mut add = 0.0;
        for rift in &self.rifts {
            let ramp = (self.rift_taper * (rift.end_m - rift.start_m)).max(f64::MIN_POSITIVE);
            if r > rift.start_m && r < rift.end_m {
                let ad = ang_diff(th, self.rift_bearing(rift, r));
                if ad.abs() < FRAC_PI_2 {
                    let dp = r * ad.sin() / self.rift_ridge_w;
                    let taper = smootherstep((r - rift.start_m) / ramp)
                        * (1.0 - smootherstep((r - (rift.end_m - ramp)) / ramp));
                    add += self.rift_ridge_m * (-dp * dp).exp() * taper;
                }
            }
            for c in &rift.cones {
                let rc = len(sub(q, c.centre_m));
                let span = c.base_radius_m - c.top_radius_m;
                if rc < c.base_radius_m + self.cone_knee * span {
                    let mut h = c.height_m * knee_clamp01((c.base_radius_m - rc) / span, self.cone_knee);
                    if rc < c.top_radius_m {
                        h -= c.crater_depth_m * bowl(rc / c.top_radius_m);
                    }
                    add += h;
                }
            }
        }
        add
    }

    fn subpeaks_at(&self, q: [f64; 2]) -> f64 {
        let mut add = 0.0;
        for s in &self.subpeaks {
            let d = sub(q, s.centre_m);
            let (u, v) = (
                dot(d, s.axis) / s.radius_m[0],
                (d[0] * s.axis[1] - d[1] * s.axis[0]) / s.radius_m[1],
            );
            let t = u.hypot(v);
            if t < 1.0 {
                add += s.height_m * (1.0 - t) * (1.0 - t);
            }
        }
        add
    }

    fn collapse_at(&self, q: [f64; 2], e: f64) -> f64 {
        let c = &self.collapse;
        let d = sub(q, self.summit_m);
        let u = dot(d, c.dir);
        let v = d[0] * c.dir[1] - d[1] * c.dir[0];
        let u0 = c.head_m + 0.5 * c.width_head_m;
        let mut sd = if u < u0 {
            (u - u0).hypot(v) - 0.5 * c.width_head_m
        } else {
            let t = ((u - u0) / (c.toe_m - u0).max(f64::MIN_POSITIVE)).clamp(0.0, 1.0);
            v.abs() - 0.5 * lerp(c.width_head_m, c.width_toe_m, t)
        };
        if sd > c.wall_m + c.wall_warp_m {
            return e;
        }
        sd += c.wall_warp_m * value_noise(q, c.wall_warp_wavelength_m, self.seed + S_COLLAPSE);
        let m = 1.0 - smootherstep((sd + 0.5 * c.wall_m) / c.wall_m);
        let t = ((u - c.head_m) / (c.toe_m - c.head_m)).clamp(0.0, 1.0);
        let depth = c.depth_m * (1.0 - smootherstep((t - c.depth_fade) / (1.0 - c.depth_fade)));
        (e - m * depth).max(0.0)
    }

    /// Edifice height E above reference + apron at warped position `q`, and ρ there (no coast fit).
    pub fn edifice(&self, q: [f64; 2]) -> (f64, f64) {
        self.edifice_at(q, f64::INFINITY)
    }

    /// [`Shape::edifice`] with the sample's coast distance (the coast-fitted footprint uses it).
    pub fn edifice_at(&self, q: [f64; 2], d_coast: f64) -> (f64, f64) {
        let (mut e, rho) = self.core(q, d_coast);
        let d = sub(q, self.summit_m);
        let r = len(d);
        if r < self.crater_r && self.crater_d > 0.0 {
            let u = if r > 0.0 { [d[0] / r, d[1] / r] } else { [0.0, 1.0] };
            let rim = [
                self.summit_m[0] + self.crater_r * u[0],
                self.summit_m[1] + self.crater_r * u[1],
            ];
            let level = self.core(rim, d_coast).0;
            e = e.min(level) - self.crater_d * bowl(r / self.crater_r);
        }
        if self.features.subpeaks {
            e += self.subpeaks_at(q);
        }
        if self.features.rifts {
            e += self.rifts_at(q);
        }
        if self.features.collapse {
            e = self.collapse_at(q, e);
        }
        (e.max(0.0), rho)
    }

    /// Apron height at a coast distance `d_coast` and radial apron weight `w_r`.
    #[inline]
    fn apron(&self, d_coast: f64, w_r: f64) -> f64 {
        if self.apron_max <= 0.0 || w_r <= 0.0 {
            return 0.0;
        }
        self.apron_max * (self.apron_g * d_coast / self.apron_max).tanh() * w_r
    }

    /// Radial apron weight at distance r from the summit.
    #[inline]
    fn apron_weight(&self, r: f64) -> f64 {
        1.0 - smootherstep((r - self.apron_r[0]) / (self.apron_r[1] - self.apron_r[0]))
    }

    /// Final height of a land sample at world position `p` over base height `base` with coast distance
    /// `d_coast`; returns (height, edifice blend weight). Far from the volcano it returns `base` exactly.
    pub fn surface(&self, p: [f64; 2], base: f64, d_coast: f64) -> (f64, f64) {
        let q = self.warp(p);
        let r = len(sub(q, self.summit_m));
        if r > self.support_m {
            return (base, 0.0);
        }
        let (e, rho) = if r <= self.edifice_support_m || self.features.rifts {
            self.edifice_at(q, d_coast)
        } else {
            (0.0, f64::INFINITY)
        };
        let w_e = 1.0 - smootherstep((rho - self.blend[0]) / (self.blend[1] - self.blend[0]));
        let w_r = self.apron_weight(r);
        let cf = if self.coast_fade > 0.0 {
            smootherstep(d_coast / self.coast_fade)
        } else {
            1.0
        };
        if self.mode == Mode::Additive {
            // The base is kept: everything added is >= 0, so land stays land (and stays at or above sea).
            return (base + self.apron(d_coast, w_r) + cf * e, w_e);
        }
        let base2 = if w_e <= 0.0 && w_r <= 0.0 {
            base
        } else {
            let x = base - self.ref_m;
            self.ref_m + (1.0 - w_e) * (x - self.flatten * w_r * soft_ramp(x - self.knee, self.knee))
        };
        let h = base2 + self.apron(d_coast, w_r) + cf * e;
        (h.max(self.sea_level_m), w_e)
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Grid stages

/// A height grid with its world frame (row 0 = north; sample (i, j) at `origin + (i, −j) spacing`).
#[derive(Debug, Clone)]
pub struct Field {
    pub grid: Grid,
    pub origin_m: [f64; 2],
    pub sea_level_m: f64,
    pub fill_m: Option<f64>,
}

impl Field {
    /// Reads a guide v1 folder (region grid not carried).
    pub fn read(dir: &Path) -> Result<Self> {
        let g = Guide::read(dir)?;
        Ok(Self {
            grid: Grid {
                w: g.w,
                h: g.h,
                spacing: g.spacing_m,
                data: g.heights.iter().map(|&v| f64::from(v)).collect(),
            },
            origin_m: g.origin_m,
            sea_level_m: g.sea_level_m,
            fill_m: g.fill_m,
        })
    }

    /// World position of sample (i, j).
    #[inline]
    pub fn world(&self, i: usize, j: usize) -> [f64; 2] {
        [
            self.origin_m[0] + i as f64 * self.grid.spacing,
            self.origin_m[1] - j as f64 * self.grid.spacing,
        ]
    }

    /// Bilinear height at world position `p` (edge-clamped).
    pub fn bilinear(&self, p: [f64; 2]) -> f64 {
        let x = (p[0] - self.origin_m[0]) / self.grid.spacing;
        let y = (self.origin_m[1] - p[1]) / self.grid.spacing;
        let (x0, y0) = (x.floor(), y.floor());
        let (tx, ty) = (x - x0, y - y0);
        let (xi, yi) = (x0 as isize, y0 as isize);
        let g = &self.grid;
        let a = lerp(g.at_clamped(xi, yi), g.at_clamped(xi + 1, yi), tx);
        let b = lerp(g.at_clamped(xi, yi + 1), g.at_clamped(xi + 1, yi + 1), tx);
        lerp(a, b, ty)
    }

    /// Sample index nearest to world position `p`, if inside.
    pub fn index_of(&self, p: [f64; 2]) -> Option<usize> {
        let x = ((p[0] - self.origin_m[0]) / self.grid.spacing).round();
        let y = ((self.origin_m[1] - p[1]) / self.grid.spacing).round();
        (x >= 0.0 && y >= 0.0 && (x as usize) < self.grid.w && (y as usize) < self.grid.h)
            .then(|| y as usize * self.grid.w + x as usize)
    }
}

fn edt_1d(f: &[f64], out: &mut [f64], v: &mut [usize], z: &mut [f64]) {
    let n = f.len();
    let sq = |q: usize| (q * q) as f64;
    let inter = |q: usize, p: usize| ((f[q] + sq(q)) - (f[p] + sq(p))) / (2.0 * (q as f64 - p as f64));
    let mut k = 0usize;
    v[0] = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        // f is finite, so s > z[0] = −∞ and k never goes below 0.
        let mut s = inter(q, v[k]);
        while s <= z[k] {
            k -= 1;
            s = inter(q, v[k]);
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, o) in out.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        let d = q as f64 - p as f64;
        *o = d * d + f[p];
    }
}

/// Euclidean distance (metres) from every sample to the nearest wet sample: the exact squared distance transform
/// of Felzenszwalb & Huttenlocher (2012), columns then rows; each 1D pass is a pure function of its line.
pub fn coast_distance(wet: &[bool], w: usize, h: usize, spacing: f64) -> Vec<f64> {
    let cols: Vec<Vec<f64>> = (0..w)
        .into_par_iter()
        .map(|x| {
            let f: Vec<f64> = (0..h).map(|y| if wet[y * w + x] { 0.0 } else { EDT_FAR }).collect();
            let mut out = vec![0.0; h];
            let (mut v, mut z) = (vec![0usize; h], vec![0.0; h + 1]);
            edt_1d(&f, &mut out, &mut v, &mut z);
            out
        })
        .collect();
    let mut d = vec![0.0; w * h];
    d.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let f: Vec<f64> = (0..w).map(|x| cols[x][y]).collect();
        let (mut v, mut z) = (vec![0usize; w], vec![0.0; w + 1]);
        edt_1d(&f, row, &mut v, &mut z);
        for o in row.iter_mut() {
            *o = o.min(EDT_FAR).sqrt() * spacing;
        }
    });
    d
}

/// Highest land sample of `field` whose position falls in region `name` of `spec.region_guide`.
pub fn region_peak(field: &Field, spec: &CentreSpec) -> Result<([f64; 2], String)> {
    if let Some(p) = spec.explicit_m {
        return Ok((p, format!("explicit ({:.0}, {:.0}) m", p[0], p[1])));
    }
    let (Some(dir), Some(name)) = (&spec.region_guide, &spec.region) else {
        return Err(Error::Invalid(
            "[centre] needs explicit_m, or region_guide + region".into(),
        ));
    };
    let g = Guide::read(dir)?;
    let reg = g
        .regions
        .ok_or_else(|| Error::Invalid(format!("{}: no regions.png", dir.display())))?;
    let idx = reg
        .find(name)
        .ok_or_else(|| Error::Invalid(format!("region {name} not in the legend")))?;
    let scale = spec.region_scale.unwrap_or(1.0);
    let [ox, oy] = spec.region_origin_m;
    let sea = field.sea_level_m;
    let best = (0..field.grid.len())
        .into_par_iter()
        .filter_map(|i| {
            let v = field.grid.data[i];
            if v < sea {
                return None;
            }
            let p = field.world(i % field.grid.w, i / field.grid.w);
            let (gx, gy) = (p[0] / scale, p[1] / scale);
            let px = ((gx - ox) / reg.cell_m).floor();
            let py = ((oy - gy) / reg.cell_m).floor();
            let inside = px >= 0.0 && py >= 0.0 && (px as usize) < reg.w && (py as usize) < reg.h;
            (inside && reg.index[py as usize * reg.w + px as usize] == idx).then_some((i, v))
        })
        .reduce_with(|a, b| match b.1.total_cmp(&a.1) {
            std::cmp::Ordering::Greater => b,
            std::cmp::Ordering::Equal if b.0 < a.0 => b,
            _ => a,
        })
        .ok_or_else(|| Error::Invalid(format!("region {name}: no land sample in the base")))?;
    let p = field.world(best.0 % field.grid.w, best.0 / field.grid.w);
    Ok((
        p,
        format!(
            "highest base sample of \"{name}\" ({:.1} m) at ({:.0}, {:.0}) m",
            best.1, p[0], p[1]
        ),
    ))
}

/// Quantile `q` of the land base heights on the ring of radius `r` around `c` (one sample wide); sea level when
/// the ring has no land.
fn ring_quantile(field: &Field, c: [f64; 2], r: f64, q: f64) -> f64 {
    let sp = field.grid.spacing;
    let mut ring: Vec<f64> = (0..field.grid.len())
        .filter(|&i| field.grid.data[i] >= field.sea_level_m)
        .filter(|&i| (len(sub(field.world(i % field.grid.w, i / field.grid.w), c)) - r).abs() <= sp)
        .map(|i| field.grid.data[i])
        .collect();
    if ring.is_empty() {
        return field.sea_level_m;
    }
    ring.sort_by(f64::total_cmp);
    ring[((ring.len() - 1) as f64 * q).round() as usize]
}

/// Stamps `shape` on every land sample of `base` (rows in parallel, each sample a pure function). Returns the
/// heights and the edifice blend weight.
pub fn stamp(shape: &Shape, base: &Field, d_coast: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let w = base.grid.w;
    let mut h = base.grid.data.clone();
    let mut we = vec![0.0; h.len()];
    h.par_chunks_mut(w)
        .zip(we.par_chunks_mut(w))
        .enumerate()
        .for_each(|(j, (row, wrow))| {
            for (i, (v, wv)) in row.iter_mut().zip(wrow.iter_mut()).enumerate() {
                if *v < base.sea_level_m {
                    continue;
                }
                let (nh, ww) = shape.surface(base.world(i, j), *v, d_coast[j * w + i]);
                *v = nh;
                *wv = ww;
            }
        });
    (h, we)
}

/// Largest stamped height in the calibration window around the summit.
fn window_max(shape: &Shape, base: &Field, d_coast: &[f64], window: &[usize]) -> f64 {
    window
        .par_iter()
        .map(|&i| {
            let p = base.world(i % base.grid.w, i / base.grid.w);
            shape.surface(p, base.grid.data[i], d_coast[i]).0
        })
        .reduce(|| f64::NEG_INFINITY, f64::max)
}

/// Sets `shape.edifice_m` so the stamped maximum near the summit equals `target_m` (secant on `H_e`; the maximum
/// grows monotonically with `H_e`). Returns the reached maximum.
pub fn calibrate(shape: &mut Shape, base: &Field, d_coast: &[f64], target_m: f64) -> f64 {
    let rw = CALIB_WINDOW * shape.r_mean_m;
    let window: Vec<usize> = (0..base.grid.len())
        .filter(|&i| {
            base.grid.data[i] >= base.sea_level_m
                && len(sub(base.world(i % base.grid.w, i / base.grid.w), shape.summit_m)) <= rw
        })
        .collect();
    if window.is_empty() {
        return f64::NAN;
    }
    let f = |s: &mut Shape, he: f64| {
        s.edifice_m = he;
        window_max(s, base, d_coast, &window) - target_m
    };
    let (mut x0, mut x1) = (shape.edifice_m, shape.edifice_m * 1.05);
    let (mut f0, mut f1) = (f(shape, x0), f(shape, x1));
    for _ in 0..CALIB_STEPS {
        if f1.abs() < CALIB_TOL_M || (f1 - f0).abs() < f64::EPSILON {
            break;
        }
        let x2 = (x1 - f1 * (x1 - x0) / (f1 - f0)).max(1.0);
        (x0, f0) = (x1, f1);
        x1 = x2;
        f1 = f(shape, x1);
    }
    shape.edifice_m = x1;
    f1 + target_m
}

// ---------------------------------------------------------------------------------------------------------------
// Ravines

/// Numbers of the ravine stage.
#[derive(Debug, Clone, Default, Serialize)]
pub struct RavineStats {
    pub crop: [usize; 4],
    pub steps: usize,
    pub thermal_steps: usize,
    pub thermal_moved_m3: f64,
    /// Incision (stamp − final) over samples with edifice weight > 0.5, metres.
    pub incision_mean_m: f64,
    pub incision_p90_m: f64,
    pub incision_p99_m: f64,
    pub incision_max_m: f64,
    pub fractures: usize,
    pub seconds_fields: f64,
    pub seconds_erosion: f64,
    pub seconds_thermal: f64,
}

/// A fracture polyline (world metres) with its cumulative length.
struct Fracture {
    pts: Vec<[f64; 2]>,
    cum: Vec<f64>,
    /// Bounding box (x0, y0, x1, y1), world metres.
    bbox: [f64; 4],
}

fn fractures(shape: &Shape, spec: &Spec) -> Vec<Fracture> {
    let v = |n: &str| spec.v(n);
    let seed = shape.seed + S_FRACTURE;
    let angle = v("fracture_angle_deg").to_radians();
    (0..v("fracture_count") as i64)
        .map(|f| {
            let b = TAU * u01(f, 0, seed);
            let r0 = lerp(v("fracture_start_min"), v("fracture_start_max"), u01(f, 1, seed)) * shape.r_mean_m;
            let length = lerp(v("fracture_len_min"), v("fracture_len_max"), u01(f, 2, seed)) * shape.r_mean_m;
            let heading = b + angle * (2.0 * u01(f, 3, seed) - 1.0);
            let seg = length / FRACTURE_SEGMENTS as f64;
            let s0 = dir(b);
            let mut p = [shape.summit_m[0] + r0 * s0[0], shape.summit_m[1] + r0 * s0[1]];
            let mut pts = vec![p];
            let mut cum = vec![0.0];
            for k in 0..FRACTURE_SEGMENTS {
                let t = k as f64 * seg;
                let hd = heading
                    + angle
                        * value_noise(
                            [t, f as f64 * 1e5],
                            v("fracture_wander_wavelength_m"),
                            shape.seed + S_FRACTURE_WANDER,
                        );
                let d = dir(hd);
                p = [p[0] + seg * d[0], p[1] + seg * d[1]];
                pts.push(p);
                cum.push(t + seg);
            }
            let bbox = pts.iter().fold(
                [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY],
                |b, p| [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])],
            );
            Fracture { pts, cum, bbox }
        })
        .collect()
}

/// Distance from `p` to the polyline and the arc-length share at the nearest point.
fn polyline_distance(fr: &Fracture, p: [f64; 2]) -> (f64, f64) {
    let total = *fr.cum.last().unwrap_or(&1.0);
    let mut best = (f64::INFINITY, 0.0);
    for k in 0..fr.pts.len() - 1 {
        let (a, b) = (fr.pts[k], fr.pts[k + 1]);
        let ab = sub(b, a);
        let l2 = dot(ab, ab);
        let t = if l2 > 0.0 { (dot(sub(p, a), ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
        let c = [a[0] + t * ab[0], a[1] + t * ab[1]];
        let d = len(sub(p, c));
        if d < best.0 {
            best = (d, (fr.cum[k] + t * (fr.cum[k + 1] - fr.cum[k])) / total);
        }
    }
    best
}

/// Relative erodibility of a sample: rock bands by height × cellular patches × flank.
fn erodibility(shape: &Shape, spec: &Spec, p: [f64; 2], h: f64) -> f64 {
    let v = |n: &str| spec.v(n);
    let s = shape.seed;
    let thick = v("band_thickness_m");
    let t = (h + v("band_warp_m") * value_noise(p, thick * 4.0, s + S_BAND_WARP)) / thick;
    let i = t.floor();
    let f = t - i;
    let edge = v("band_edge");
    let (b0, b1) = (lattice(i as i64, 0, s + S_BAND), lattice(i as i64 + 1, 0, s + S_BAND));
    let band = lerp(b0, b1, smootherstep((f - (1.0 - edge)) / edge));
    let cell = v("cell_size_m");
    let (cx, cy) = ((p[0] / cell).floor() as i64, (p[1] / cell).floor() as i64);
    let mut best = (f64::INFINITY, 0.0);
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (ix, iy) = (cx + dx, cy + dy);
            let jx = (ix as f64 + u01(ix, iy, s + S_CELL)) * cell;
            let jy = (iy as f64 + u01(iy, ix, s + S_CELL)) * cell;
            let d = (p[0] - jx).hypot(p[1] - jy);
            if d < best.0 {
                best = (d, lattice(ix, iy, s + S_CELL + 100));
            }
        }
    }
    let th = bearing_of(sub(p, shape.summit_m));
    let flank = ang_noise(shape.summit_m, shape.r_mean_m, th, v("flank_k_wavelength_m"), s + S_FLANK_K);
    ((1.0 + v("band_k_var") * band) * (1.0 + v("cell_k_var") * best.1) * (1.0 + v("flank_k_var") * flank))
        .max(0.0)
}

/// Erodes the edifice: heterogeneous-K stream power with seeded fractures, then talus relaxation, on a crop
/// around the edifice. `h` and `w_e` are full-grid fields of `base`'s frame.
#[allow(clippy::too_many_lines)] // one stage: fields, erosion, talus, stats, in order
pub fn ravines(
    shape: &Shape,
    spec: &Spec,
    base: &Field,
    h: &mut [f64],
    w_e: &[f64],
    d_coast: &[f64],
) -> Result<RavineStats> {
    let v = |n: &str| spec.v(n);
    let t0 = Instant::now();
    let (gw, gh, sp) = (base.grid.w, base.grid.h, base.grid.spacing);
    let reach = shape.edifice_support_m + 2.0 * sp;
    let c = shape.summit_m;
    let col = |x: f64| ((x - base.origin_m[0]) / sp).round().clamp(0.0, (gw - 1) as f64) as usize;
    let row = |y: f64| ((base.origin_m[1] - y) / sp).round().clamp(0.0, (gh - 1) as f64) as usize;
    let (x0, x1, y0, y1) = (col(c[0] - reach), col(c[0] + reach), row(c[1] + reach), row(c[1] - reach));
    let (cw, ch) = (x1 - x0 + 1, y1 - y0 + 1);
    let gi = |i: usize| (y0 + i / cw) * gw + x0 + i % cw;
    let mut g = Grid {
        w: cw,
        h: ch,
        spacing: sp,
        data: (0..cw * ch).map(|i| h[gi(i)]).collect(),
    };
    let fr = fractures(shape, spec);
    let (fd, fwid, ftaper) = (v("fracture_depth_m"), v("fracture_width_m"), v("fracture_taper"));
    let cf = |i: usize| {
        if v("coast_fade_m") > 0.0 {
            smootherstep(d_coast[gi(i)] / v("coast_fade_m"))
        } else {
            1.0
        }
    };
    // Per-sample fields: K, seed perturbation (+ fracture notch), talus.
    let fields: Vec<(f64, f64)> = (0..cw * ch)
        .into_par_iter()
        .map(|i| {
            let k = gi(i);
            let we = w_e[k] * cf(i);
            if we <= 0.0 || g.data[i] < base.sea_level_m {
                return (0.0, 0.0);
            }
            let p = base.world(k % gw, k / gw);
            let mut mask: f64 = 0.0;
            for f in &fr {
                let m = 3.0 * fwid;
                if p[0] < f.bbox[0] - m || p[0] > f.bbox[2] + m || p[1] < f.bbox[1] - m || p[1] > f.bbox[3] + m {
                    continue;
                }
                let (d, t) = polyline_distance(f, p);
                if d < 3.0 * fwid {
                    let taper = smootherstep(t / ftaper) * smootherstep((1.0 - t) / ftaper);
                    mask = mask.max((-(d / fwid) * (d / fwid)).exp() * taper);
                }
            }
            let kk = v("ravine_k_dt") * we * erodibility(shape, spec, p, g.data[i])
                * (1.0 + v("fracture_k_boost") * mask);
            let seed = v("ravine_seed_m") * value_noise(p, v("ravine_seed_wavelength_m"), shape.seed + S_SEED_COARSE)
                + v("ravine_seed_fine_m")
                    * value_noise(p, v("ravine_seed_fine_wavelength_m"), shape.seed + S_SEED_FINE);
            (kk, we * seed - fd * mask * we)
        })
        .collect();
    let k_dt: Vec<f64> = fields.iter().map(|f| f.0).collect();
    let perturb: Vec<f64> = fields.iter().map(|f| f.1).collect();
    drop(fields);
    let zeros = vec![0.0; g.len()];
    let before = g.data.clone();
    let t1 = Instant::now();
    let st = erode(
        &mut g,
        &before,
        &StreamParams {
            m: v("ravine_m"),
            iterations: v("ravine_steps") as usize,
            k_dt: &k_dt,
            restore: &zeros,
            sea_level: base.sea_level_m,
            flow: Flow::Mfd {
                exponent: v("ravine_flow_exponent"),
            },
            perturb: Some(&perturb),
            steady_dh_m: None,
        },
    )?;
    let t2 = Instant::now();
    let tan_t = v("ravine_talus_deg").to_radians().tan();
    let talus: Vec<f64> = (0..g.len())
        .map(|i| if w_e[gi(i)] > 0.0 { tan_t } else { f64::MAX })
        .collect();
    let thermal_steps = v("ravine_thermal_steps") as usize;
    let moved = if thermal_steps > 0 {
        relax(
            &mut g,
            &ThermalParams {
                iterations: thermal_steps,
                rate: v("ravine_thermal_rate"),
                tan_talus: &talus,
                sea_level: base.sea_level_m,
            },
        )?
    } else {
        0.0
    };
    let t3 = Instant::now();
    let mut inc: Vec<f64> = (0..g.len())
        .filter(|&i| w_e[gi(i)] > 0.5)
        .map(|i| (before[i] - g.data[i]).max(0.0))
        .collect();
    for (i, &val) in g.data.iter().enumerate() {
        h[gi(i)] = val;
    }
    let mut out = RavineStats {
        crop: [x0, y0, cw, ch],
        steps: st.iterations,
        thermal_steps,
        thermal_moved_m3: moved,
        fractures: fr.len(),
        seconds_fields: (t1 - t0).as_secs_f64(),
        seconds_erosion: (t2 - t1).as_secs_f64(),
        seconds_thermal: (t3 - t2).as_secs_f64(),
        ..RavineStats::default()
    };
    if !inc.is_empty() {
        inc.sort_by(f64::total_cmp);
        let q = |f: f64| inc[((inc.len() - 1) as f64 * f).round() as usize];
        out.incision_mean_m = inc.iter().sum::<f64>() / inc.len() as f64;
        out.incision_p90_m = q(0.9);
        out.incision_p99_m = q(0.99);
        out.incision_max_m = inc[inc.len() - 1];
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------
// Measurement

/// Azimuthal-median radial profile around a centre: median land height per ring of width `bin_m`.
#[derive(Debug, Clone, Serialize)]
pub struct RadialProfile {
    pub centre_m: [f64; 2],
    pub bin_m: f64,
    /// Median per bin (NaN where a bin has no land); bin k covers [k bin, (k + 1) bin).
    pub median_m: Vec<f64>,
}

/// Measures the radial profile of `field` around `centre_m` out to `max_r_m`.
pub fn radial_profile(field: &Field, centre_m: [f64; 2], bin_m: f64, max_r_m: f64) -> RadialProfile {
    let nb = (max_r_m / bin_m).floor().max(1.0) as usize;
    let mut bins: Vec<Vec<f64>> = vec![Vec::new(); nb];
    for i in 0..field.grid.len() {
        let v = field.grid.data[i];
        if v < field.sea_level_m {
            continue;
        }
        let r = len(sub(field.world(i % field.grid.w, i / field.grid.w), centre_m));
        let k = (r / bin_m).floor() as usize;
        if k < nb {
            bins[k].push(v);
        }
    }
    let median_m = bins
        .iter_mut()
        .map(|b| {
            if b.is_empty() {
                f64::NAN
            } else {
                b.sort_by(f64::total_cmp);
                b[(b.len() - 1) / 2]
            }
        })
        .collect();
    RadialProfile {
        centre_m,
        bin_m,
        median_m,
    }
}

/// Two-part fit of a radial profile: a power-law concave cone `z_b + (h_0 − z_b)(1 − r/r_b)^p` from the summit
/// bin to a break `r_b`, a straight apron beyond, the break chosen by least total squared error; plus an
/// exponential `a + B e^{−r/r₀}` fit of the same cone part (Karátson et al. 2010's form) for comparison.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct ProfileFit {
    pub summit_bin_m: f64,
    pub break_r_m: f64,
    pub break_h_m: f64,
    pub power_p: f64,
    pub power_rmse_m: f64,
    pub exp_r0_m: f64,
    pub exp_rmse_m: f64,
    /// Apron gradient beyond the break (rise toward the summit per metre) and its fit error.
    pub apron_gradient: f64,
    pub apron_rmse_m: f64,
    /// Cone relief over break radius, and slopes in degrees: mean summit-to-break, top (first quarter of the cone),
    /// steepest bin to bin.
    pub cone_relief_ratio: f64,
    pub cone_mean_deg: f64,
    pub cone_top_deg: f64,
    pub cone_steepest_deg: f64,
}

fn line_fit(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let a = my - b * mx;
    let sse = xs.iter().zip(ys).map(|(x, y)| (y - a - b * x).powi(2)).sum();
    (a, b, sse)
}

/// Best exponent p of `z_b + A (1 − r/r_b)^p` (`z_b` the last bin, A by least squares for each p) and its SSE.
fn power_fit(rs: &[f64], hs: &[f64], rb: f64) -> (f64, f64) {
    let zb = hs[hs.len() - 1];
    let mut best = (f64::NAN, f64::INFINITY);
    let mut p = FIT_P[0];
    while p <= FIT_P[1] {
        let xs: Vec<f64> = rs.iter().map(|r| (1.0 - r / rb).max(0.0).powf(p)).collect();
        let sxx: f64 = xs.iter().map(|x| x * x).sum();
        let amp = if sxx > 0.0 {
            xs.iter().zip(hs).map(|(x, h)| x * (h - zb)).sum::<f64>() / sxx
        } else {
            0.0
        };
        let sse: f64 = xs
            .iter()
            .zip(hs)
            .map(|(x, h)| (h - zb - amp * x).powi(2))
            .sum();
        if sse < best.1 {
            best = (p, sse);
        }
        p += FIT_P[2];
    }
    best
}

fn exp_fit(rs: &[f64], hs: &[f64], rb: f64) -> (f64, f64) {
    let mut best = (f64::NAN, f64::INFINITY);
    let (lo, hi) = ((0.05 * rb).ln(), (20.0 * rb).ln());
    for k in 0..=FIT_R0_STEPS {
        let r0 = lerp(lo, hi, k as f64 / FIT_R0_STEPS as f64).exp();
        let xs: Vec<f64> = rs.iter().map(|r| (-r / r0).exp()).collect();
        let sse = line_fit(&xs, hs).2;
        if sse < best.1 {
            best = (r0, sse);
        }
    }
    best
}

/// Fits a profile (bins with land only; at least 4 bins each side of the break).
pub fn fit_profile(p: &RadialProfile) -> Option<ProfileFit> {
    let pts: Vec<(f64, f64)> = p
        .median_m
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_finite())
        .map(|(k, &v)| ((k as f64 + 0.5) * p.bin_m, v))
        .collect();
    if pts.len() < 10 {
        return None;
    }
    let rs: Vec<f64> = pts.iter().map(|x| x.0).collect();
    let hs: Vec<f64> = pts.iter().map(|x| x.1).collect();
    let n = pts.len();
    let mut best: Option<(usize, f64, f64, f64, f64)> = None;
    for b in 4..n - 4 {
        let (pp, sse_u) = power_fit(&rs[..=b], &hs[..=b], rs[b]);
        let (_, slope, sse_l) = line_fit(&rs[b..], &hs[b..]);
        let total = sse_u + sse_l;
        if best.is_none_or(|x| total < x.1 + x.2) {
            best = Some((b, sse_u, sse_l, pp, slope));
        }
    }
    let (b, sse_u, sse_l, pp, slope) = best?;
    let (r0, sse_e) = exp_fit(&rs[..=b], &hs[..=b], rs[b]);
    let deg = |dh: f64, dr: f64| (dh / dr).atan().to_degrees();
    let q = (b / 4).max(1);
    let steepest = (0..b)
        .map(|k| deg(hs[k] - hs[k + 1], rs[k + 1] - rs[k]))
        .fold(f64::NEG_INFINITY, f64::max);
    Some(ProfileFit {
        summit_bin_m: hs[0],
        break_r_m: rs[b],
        break_h_m: hs[b],
        power_p: pp,
        power_rmse_m: (sse_u / (b + 1) as f64).sqrt(),
        exp_r0_m: r0,
        exp_rmse_m: (sse_e / (b + 1) as f64).sqrt(),
        apron_gradient: -slope,
        apron_rmse_m: (sse_l / (n - b) as f64).sqrt(),
        cone_relief_ratio: (hs[0] - hs[b]) / rs[b],
        cone_mean_deg: deg(hs[0] - hs[b], rs[b] - rs[0]),
        cone_top_deg: deg(hs[0] - hs[q], rs[q] - rs[0]),
        cone_steepest_deg: steepest,
    })
}

/// Median apron gradient (rise toward the volcano per metre) along rays from the edifice foot outward to the coast
/// or the apron's inner radius; with the number of usable rays.
pub fn apron_gradient(shape: &Shape, field: &Field) -> (f64, usize) {
    let sp = field.grid.spacing;
    let mut g: Vec<f64> = (0..APRON_RAYS)
        .filter_map(|k| {
            let th = TAU * k as f64 / APRON_RAYS as f64;
            let d = dir(th);
            let start = shape.foot_radius(th) * shape.blend[1] + 2.0 * shape.warp_m;
            let (mut xs, mut ys) = (Vec::new(), Vec::new());
            let mut s = start;
            while s <= shape.apron_r[0] {
                let p = [shape.summit_m[0] + s * d[0], shape.summit_m[1] + s * d[1]];
                let i = field.index_of(p)?;
                if field.grid.data[i] < field.sea_level_m {
                    break;
                }
                xs.push(s);
                ys.push(field.bilinear(p));
                s += sp;
            }
            (xs.len() >= 10 && s - start >= APRON_MIN_LEN_M).then(|| -line_fit(&xs, &ys).1)
        })
        .collect();
    if g.is_empty() {
        return (f64::NAN, 0);
    }
    g.sort_by(f64::total_cmp);
    (g[(g.len() - 1) / 2], g.len())
}

// ---------------------------------------------------------------------------------------------------------------
// Runner

/// Slopes on one ring around the final summit.
#[derive(Debug, Clone, Serialize)]
pub struct FlankRing {
    pub r_m: f64,
    /// Land samples on the ring (final field).
    pub samples: usize,
    /// Mean local slope (central differences) of the ring's land samples, degrees: base, final.
    pub base_slope_deg: f64,
    pub slope_deg: f64,
    /// Secant slope from the field's maximum down to the mean ring height, degrees: base, final.
    pub base_secant_deg: f64,
    pub secant_deg: f64,
}

/// Mean local slope and mean height of the land samples of `h` within `half` of radius `r` around `c`.
fn ring_slope(field: &Field, h: &[f64], c: [f64; 2], r: f64, half: f64) -> (f64, f64, usize) {
    let (w, gh, sp) = (field.grid.w, field.grid.h, field.grid.spacing);
    let sea = field.sea_level_m;
    let reach = r + half;
    let col = |x: f64| ((x - field.origin_m[0]) / sp).round().clamp(1.0, (w - 2) as f64) as usize;
    let row = |y: f64| ((field.origin_m[1] - y) / sp).round().clamp(1.0, (gh - 2) as f64) as usize;
    let (x0, x1) = (col(c[0] - reach), col(c[0] + reach));
    let (y0, y1) = (row(c[1] + reach), row(c[1] - reach));
    let (mut slope, mut height, mut n) = (0.0, 0.0, 0usize);
    for j in y0..=y1 {
        for i in x0..=x1 {
            let k = j * w + i;
            if h[k] < sea || (len(sub(field.world(i, j), c)) - r).abs() > half {
                continue;
            }
            let gx = (h[k + 1] - h[k - 1]) / (2.0 * sp);
            let gy = (h[k - w] - h[k + w]) / (2.0 * sp);
            slope += gx.hypot(gy).atan().to_degrees();
            height += h[k];
            n += 1;
        }
    }
    if n == 0 {
        return (f64::NAN, f64::NAN, 0);
    }
    (slope / n as f64, height / n as f64, n)
}

/// Flank slopes of the base and the final field on [`FLANK_RINGS_M`] around `c`; `max` = (base, final) maxima.
fn flank_rings(base: &Field, h: &[f64], c: [f64; 2], max: [f64; 2], half: f64) -> Vec<FlankRing> {
    FLANK_RINGS_M
        .iter()
        .map(|&r| {
            let (bs, bh, _) = ring_slope(base, &base.grid.data, c, r, half);
            let (s, hh, n) = ring_slope(base, h, c, r, half);
            FlankRing {
                r_m: r,
                samples: n,
                base_slope_deg: bs,
                slope_deg: s,
                base_secant_deg: ((max[0] - bh) / r).atan().to_degrees(),
                secant_deg: ((max[1] - hh) / r).atan().to_degrees(),
            }
        })
        .collect()
}

/// Uplift (final - base) on the land samples at coast distance `coast_fade_m` (half a sample either side) within
/// `apron_inner_m` of the summit: (mean, max, samples).
fn fade_uplift(shape: &Shape, base: &Field, h: &[f64], d_coast: &[f64]) -> (f64, f64, usize) {
    let half = 0.5 * base.grid.spacing;
    let (mut sum, mut max, mut n) = (0.0, 0.0_f64, 0usize);
    for (k, (&b, &v)) in base.grid.data.iter().zip(h).enumerate() {
        if b < base.sea_level_m || (d_coast[k] - shape.coast_fade).abs() > half {
            continue;
        }
        let p = base.world(k % base.grid.w, k / base.grid.w);
        if len(sub(p, shape.summit_m)) > shape.apron_r[0] {
            continue;
        }
        sum += v - b;
        max = max.max(v - b);
        n += 1;
    }
    if n == 0 {
        return (f64::NAN, f64::NAN, 0);
    }
    (sum / n as f64, max, n)
}

/// Numbers of one arm.
#[derive(Debug, Clone, Serialize)]
pub struct ArmReport {
    pub name: String,
    pub help: String,
    pub mode: Mode,
    /// Per-arm parameter values.
    pub set: BTreeMap<String, f64>,
    pub features: Features,
    pub detail_from: Option<String>,
    pub seconds_shape: f64,
    pub seconds_ravines: f64,
    pub seconds_write: f64,
    pub summit_target_m: f64,
    /// Calibrated maximum before ravines, and the final maximum with its position.
    pub summit_stamped_m: f64,
    pub max_m: f64,
    pub max_at_m: [f64; 2],
    pub edifice_m: f64,
    pub apron_gradient: f64,
    pub apron_rays: usize,
    /// Apron height at coast distance `coast_fade_m` (radial weight 1), metres.
    pub apron_at_fade_m: f64,
    /// Measured uplift (final - base) on the coast-fade contour within `apron_inner_m`: mean, max, samples.
    pub fade_uplift_mean_m: f64,
    pub fade_uplift_max_m: f64,
    pub fade_samples: usize,
    /// Largest uplift of any land sample, metres.
    pub uplift_max_m: f64,
    /// Flank slopes on rings around the final summit.
    pub flank: Vec<FlankRing>,
    pub fit: Option<ProfileFit>,
    pub ravines: Option<RavineStats>,
    pub detail: Option<crate::filter::DetailReport>,
    pub audit: Audit,
    pub wet_dry: WetDry,
    pub heights_blake3: String,
    /// Channel structure on the edifice window ([`crate::structure`]).
    pub structure: Option<crate::structure::Structure>,
}

/// What a `stamp-volcano` run produced.
#[derive(Debug, Clone, Serialize)]
pub struct VolcanoRun {
    pub base: String,
    pub centre_m: [f64; 2],
    pub centre_rule: String,
    pub ref_m: f64,
    pub coast_distance_at_summit_m: f64,
    pub seconds_setup: f64,
    pub references: Vec<(String, Option<ProfileFit>)>,
    /// Channel structure of `[reference] structure_guides` on the same window.
    pub reference_structure: Vec<(String, crate::structure::Structure)>,
    pub arms: Vec<ArmReport>,
    pub guess_count: usize,
    pub param_count: usize,
}

/// The shared inputs of every arm.
pub struct Prepared {
    pub base: Field,
    pub d_coast: Vec<f64>,
    pub centre_m: [f64; 2],
    pub centre_rule: String,
    pub ref_m: f64,
    pub under_summit_m: f64,
}

/// Reference level of the replace mode: quantile `ref_quantile` of the land base on the foot ring, at least sea.
fn ref_level(spec: &Spec, base: &Field, centre: [f64; 2]) -> f64 {
    let r_mean = f64::midpoint(spec.v("foot_major_m"), spec.v("foot_minor_m"));
    ring_quantile(base, centre, r_mean * spec.v("blend_out"), spec.v("ref_quantile")).max(base.sea_level_m)
}

/// What the edifice stands on at the summit: the reference level (replace) or the base (additive), plus the apron
/// there (its radial weight is 1 at the summit by the ordering `apron_inner_m` > 0).
fn under_summit(spec: &Spec, base: &Field, d_coast: &[f64], centre: [f64; 2], ref_m: f64) -> f64 {
    let probe = Shape::new(spec, Features::default(), centre, base.sea_level_m, ref_m, 0.0);
    let dc = base.index_of(centre).map_or(0.0, |i| d_coast[i]);
    let lift = probe.apron(dc, probe.apron_weight(0.0));
    match spec.mode {
        Mode::Replace => ref_m + lift,
        Mode::Additive => base.bilinear(centre).max(base.sea_level_m) + lift,
    }
}

/// Reads nothing: prepares a base field (coast distance from the base's own wet mask, centre, reference level).
pub fn prepare(spec: &Spec, base: Field) -> Result<Prepared> {
    let wet: Vec<bool> = base.grid.data.iter().map(|&v| v < base.sea_level_m).collect();
    let d_coast = coast_distance(&wet, base.grid.w, base.grid.h, base.grid.spacing);
    let (centre_m, centre_rule) = region_peak(&base, &spec.centre)?;
    let ref_m = ref_level(spec, &base, centre_m);
    let under_summit_m = under_summit(spec, &base, &d_coast, centre_m, ref_m);
    Ok(Prepared {
        base,
        d_coast,
        centre_m,
        centre_rule,
        ref_m,
        under_summit_m,
    })
}

/// Stamps one arm: shape, calibration, stamp, optional ravines. Returns the heights and the report (without the
/// write and detail parts).
pub fn stamp_arm(spec: &Spec, prep: &Prepared, arm: &ArmSpec) -> Result<(Vec<f64>, ArmReport)> {
    let t0 = Instant::now();
    let base = &prep.base;
    let spec = &spec.for_arm(arm)?;
    let (ref_m, under) = if arm.set.is_empty() {
        (prep.ref_m, prep.under_summit_m)
    } else {
        let r = ref_level(spec, base, prep.centre_m);
        (r, under_summit(spec, base, &prep.d_coast, prep.centre_m, r))
    };
    let target = spec.v("summit_m");
    let mut shape = Shape::new(spec, arm.features, prep.centre_m, base.sea_level_m, ref_m, under);
    let summit_stamped = calibrate(&mut shape, base, &prep.d_coast, target);
    let (mut h, w_e) = stamp(&shape, base, &prep.d_coast);
    let t1 = Instant::now();
    let rav = if arm.features.ravines {
        Some(ravines(&shape, spec, base, &mut h, &w_e, &prep.d_coast)?)
    } else {
        None
    };
    let t2 = Instant::now();
    let out = Field {
        grid: Grid {
            w: base.grid.w,
            h: base.grid.h,
            spacing: base.grid.spacing,
            data: h,
        },
        ..base.clone()
    };
    let (mi, max) = out
        .grid
        .data
        .iter()
        .enumerate()
        .fold((0, f64::NEG_INFINITY), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
    let max_at = out.world(mi % out.grid.w, mi / out.grid.w);
    let (ag, rays) = apron_gradient(&shape, &out);
    let fit = fit_profile(&radial_profile(&out, max_at, spec.v("fit_bin_m"), spec.v("fit_max_r_m")));
    let structure = Some(edifice_structure(&out, spec, prep.centre_m));
    let base_max = base.grid.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let flank = flank_rings(base, &out.grid.data, max_at, [base_max, max], 0.5 * spec.v("fit_bin_m"));
    let fade = fade_uplift(&shape, base, &out.grid.data, &prep.d_coast);
    let uplift_max_m = base
        .grid
        .data
        .iter()
        .zip(&out.grid.data)
        .filter(|(b, _)| **b >= base.sea_level_m)
        .map(|(b, n)| n - b)
        .fold(0.0, f64::max);
    let report = ArmReport {
        name: arm.name.clone(),
        help: arm.help.clone(),
        mode: spec.mode,
        set: arm.set.clone(),
        features: arm.features,
        detail_from: None,
        seconds_shape: (t1 - t0).as_secs_f64(),
        seconds_ravines: (t2 - t1).as_secs_f64(),
        seconds_write: 0.0,
        summit_target_m: target,
        summit_stamped_m: summit_stamped,
        max_m: max,
        max_at_m: max_at,
        edifice_m: shape.edifice_m,
        apron_gradient: ag,
        apron_rays: rays,
        apron_at_fade_m: shape.apron(shape.coast_fade, 1.0),
        fade_uplift_mean_m: fade.0,
        fade_uplift_max_m: fade.1,
        fade_samples: fade.2,
        uplift_max_m,
        flank,
        fit,
        ravines: rav,
        detail: None,
        audit: Audit::compare(&base.grid.data, &out.grid.data, base.sea_level_m),
        wet_dry: WetDry::count(&out.grid.data, base.sea_level_m),
        heights_blake3: hash_f32(&out.grid.data),
        structure,
    };
    Ok((out.grid.data, report))
}

/// Channel structure of `field` inside the square edifice window around `centre` (half-edge: major semi-axis +
/// offset, times `blend_out`).
pub fn edifice_structure(field: &Field, spec: &Spec, centre: [f64; 2]) -> crate::structure::Structure {
    let half = (spec.v("foot_major_m") + spec.v("foot_offset_m")) * spec.v("blend_out");
    let sp = field.grid.spacing;
    let (gw, gh) = (field.grid.w, field.grid.h);
    let col = |x: f64| ((x - field.origin_m[0]) / sp).round().clamp(0.0, (gw - 1) as f64) as usize;
    let row = |y: f64| ((field.origin_m[1] - y) / sp).round().clamp(0.0, (gh - 1) as f64) as usize;
    let (x0, x1) = (col(centre[0] - half), col(centre[0] + half));
    let (y0, y1) = (row(centre[1] + half), row(centre[1] - half));
    let (w, h) = (x1 - x0 + 1, y1 - y0 + 1);
    let g = Grid {
        w,
        h,
        spacing: sp,
        data: (0..w * h).map(|i| field.grid.data[(y0 + i / w) * gw + x0 + i % w]).collect(),
    };
    crate::structure::structure(
        &g,
        field.sea_level_m,
        STRUCTURE_THRESHOLD_M2,
        STRUCTURE_WINDOW_M,
        STRUCTURE_DIR_STEPS,
    )
}

/// Writes a guide v1 folder: heights, `meta.json` (the base's own keys, e.g. `cell_size_m`, kept under ours) and the
/// base's side files ([`GUIDE_SIDE_FILES`]: same grid and frame, so they stay valid).
fn write_field(
    dir: &Path,
    base: &Field,
    h: &[f64],
    meta_extra: serde_json::Value,
    base_dir: Option<&Path>,
) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut bytes = Vec::with_capacity(h.len() * 4);
    for &v in h {
        bytes.extend_from_slice(&(v as f32).to_le_bytes());
    }
    std::fs::write(dir.join("heights.f32"), bytes)?;
    let mut merged = serde_json::Map::new();
    if let Some(bd) = base_dir {
        if let Ok(text) = std::fs::read_to_string(bd.join("meta.json"))
            && let Ok(serde_json::Value::Object(m)) = serde_json::from_str::<serde_json::Value>(&text)
        {
            for (k, v) in m {
                let k = if k == "producer" { "base_producer".to_string() } else { k };
                merged.insert(k, v);
            }
        }
        for f in GUIDE_SIDE_FILES {
            if bd.join(f).exists() {
                std::fs::copy(bd.join(f), dir.join(f))?;
            }
        }
    }
    let meta = serde_json::json!({
        "format": 1,
        "dims": [base.grid.w, base.grid.h],
        "sample_spacing_m": base.grid.spacing,
        "sea_level_m": base.sea_level_m,
        "fill_m": base.fill_m,
        "origin_m": base.origin_m,
        "orientation": "row-major, row 0 = north edge, column 0 = west edge; sample (col, row) at \
                        x = origin_m.x + spacing col (east), y = origin_m.y - spacing row (north)",
        "producer": "sf_worldgen scrib-worldgen stamp-volcano",
        "stratovolcano": meta_extra,
    });
    if let serde_json::Value::Object(m) = meta {
        merged.extend(m);
    }
    std::fs::write(dir.join("meta.json"), serde_json::to_string_pretty(&merged)?)?;
    Ok(())
}

fn write_preview(dir: &Path, field: &Field, h: &[f64], centre: [f64; 2], half_m: f64) -> Result<()> {
    let sp = field.grid.spacing;
    let (gw, gh) = (field.grid.w, field.grid.h);
    let col = |x: f64| ((x - field.origin_m[0]) / sp).round().clamp(0.0, (gw - 1) as f64) as usize;
    let row = |y: f64| ((field.origin_m[1] - y) / sp).round().clamp(0.0, (gh - 1) as f64) as usize;
    let (x0, x1) = (col(centre[0] - half_m), col(centre[0] + half_m));
    let (y0, y1) = (row(centre[1] + half_m), row(centre[1] - half_m));
    let (w, hh) = (x1 - x0 + 1, y1 - y0 + 1);
    let g = Grid {
        w,
        h: hh,
        spacing: sp,
        data: (0..w * hh).map(|i| h[(y0 + i / w) * gw + x0 + i % w]).collect(),
    };
    crate::shade::write_png(
        &dir.join("edifice.png"),
        &crate::shade::hillshade(&g, field.sea_level_m, PREVIEW_LIGHT),
    )
}

/// One `scrib-worldgen stamp-volcano` run.
#[derive(Debug, Clone)]
pub struct StampRequest {
    pub base: PathBuf,
    pub params: PathBuf,
    pub out: PathBuf,
    /// Only these arms (a detail arm also needs its source arm in the list or on disk).
    pub arms: Option<Vec<String>>,
}

/// One arm's summary lines on stdout.
fn print_arm(rep: &ArmReport) {
    println!(
        "  {}: max {:.1} m at ({:.0}, {:.0}), apron {:.4} ({} rays), audit {}/{}/{}, shape {:.2} s, ravines {:.2} s",
        rep.name,
        rep.max_m,
        rep.max_at_m[0],
        rep.max_at_m[1],
        rep.apron_gradient,
        rep.apron_rays,
        rep.audit.new_wet,
        rep.audit.new_dry,
        rep.audit.sea_moved,
        rep.seconds_shape,
        rep.seconds_ravines
    );
    let rings: Vec<String> = rep
        .flank
        .iter()
        .map(|f| {
            format!(
                "{:.0} km {:.1} deg (base {:.1}), secant {:.1} (base {:.1})",
                f.r_m * 1e-3,
                f.slope_deg,
                f.base_slope_deg,
                f.secant_deg,
                f.base_secant_deg
            )
        })
        .collect();
    println!(
        "    {:?}, H_e {:.0} m, uplift max {:.0} m, apron at coast fade {:.1} m (measured uplift mean {:.1} / max {:.1} m, {} samples); flank {}",
        rep.mode,
        rep.edifice_m,
        rep.uplift_max_m,
        rep.apron_at_fade_m,
        rep.fade_uplift_mean_m,
        rep.fade_uplift_max_m,
        rep.fade_samples,
        rings.join("; ")
    );
}

/// Runs every arm of a params file and writes `<out>/<arm>/` guides and `<out>/README.md`.
pub fn run_stamp(req: &StampRequest) -> Result<VolcanoRun> {
    let t0 = Instant::now();
    let text = std::fs::read_to_string(&req.params)?;
    let spec = Spec::parse(&text)?;
    let prep = prepare(&spec, Field::read(&req.base)?)?;
    let dc_summit = prep.base.index_of(prep.centre_m).map_or(f64::NAN, |i| prep.d_coast[i]);
    let references = spec
        .reference
        .guides
        .iter()
        .map(|dir| {
            let f = Field::read(dir)?;
            let (mi, _) = f
                .grid
                .data
                .iter()
                .enumerate()
                .fold((0, f64::NEG_INFINITY), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
            let c = f.world(mi % f.grid.w, mi / f.grid.w);
            let fit = fit_profile(&radial_profile(&f, c, spec.v("fit_bin_m"), spec.v("fit_max_r_m")));
            Ok((dir.display().to_string(), fit))
        })
        .collect::<Result<Vec<_>>>()?;
    let reference_structure = spec
        .reference
        .structure_guides
        .iter()
        .map(|dir| {
            let f = Field::read(dir)?;
            Ok((dir.display().to_string(), edifice_structure(&f, &spec, prep.centre_m)))
        })
        .collect::<Result<Vec<_>>>()?;
    let seconds_setup = t0.elapsed().as_secs_f64();
    let mut arms = Vec::new();
    for arm in &spec.arms {
        if req.arms.as_ref().is_some_and(|a| !a.contains(&arm.name)) {
            continue;
        }
        let dir = req.out.join(&arm.name);
        if let Some(src) = &arm.detail_from {
            arms.push(detail_arm(&spec, &prep, arm, src, &req.out)?);
            continue;
        }
        let (h, mut rep) = stamp_arm(&spec, &prep, arm)?;
        let t = Instant::now();
        let aspec = spec.for_arm(arm)?;
        let extra = serde_json::json!({
            "base_guide": req.base.display().to_string(),
            "params_file": req.params.display().to_string(),
            "mode": aspec.mode,
            "params": aspec.params,
            "guess_count": aspec.guess_count(),
            "report": rep,
        });
        write_field(&dir, &prep.base, &h, extra, Some(&req.base))?;
        let half = spec.v("apron_inner_m");
        write_preview(&dir, &prep.base, &h, prep.centre_m, half)?;
        rep.seconds_write = t.elapsed().as_secs_f64();
        print_arm(&rep);
        arms.push(rep);
    }
    let run = VolcanoRun {
        base: req.base.display().to_string(),
        centre_m: prep.centre_m,
        centre_rule: prep.centre_rule.clone(),
        ref_m: prep.ref_m,
        coast_distance_at_summit_m: dc_summit,
        seconds_setup,
        references,
        reference_structure,
        arms,
        guess_count: spec.guess_count(),
        param_count: spec.params.len(),
    };
    std::fs::create_dir_all(&req.out)?;
    std::fs::write(req.out.join("README.md"), readme(&run, &spec, req))?;
    std::fs::write(req.out.join("run.json"), serde_json::to_string_pretty(&run)?)?;
    Ok(run)
}

fn detail_arm(spec: &Spec, prep: &Prepared, arm: &ArmSpec, src: &str, out: &Path) -> Result<ArmReport> {
    let input = out.join(src);
    let t = Instant::now();
    let src_field = Field::read(&input)?;
    let half = 0.5 * spec.v("detail_crop_m");
    let (mi, max) = src_field
        .grid
        .data
        .iter()
        .enumerate()
        .fold((0, f64::NEG_INFINITY), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
    let c = src_field.world(mi % src_field.grid.w, mi / src_field.grid.w);
    let dreq = crate::filter::DetailRequest {
        input,
        out: out.join(&arm.name),
        spacing_m: spec.v("detail_spacing_m"),
        crop_m: Some([c[0] - half, c[1] - half, c[0] + half, c[1] + half]),
        declared: crate::filter::declared_defaults(),
    };
    let d = crate::filter::run_detail(&dreq)?;
    let det = Field::read(&dreq.out)?;
    let (dmi, dmax) = det
        .grid
        .data
        .iter()
        .enumerate()
        .fold((0, f64::NEG_INFINITY), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
    println!(
        "  {}: detail of {src} at {} m, {}x{}, filter {:.2} s, max {:.1} m (source {:.1} m)",
        arm.name, d.spacing_m, d.dims[0], d.dims[1], d.seconds_filter, dmax, max
    );
    Ok(ArmReport {
        name: arm.name.clone(),
        help: arm.help.clone(),
        mode: spec.mode,
        set: arm.set.clone(),
        features: arm.features,
        detail_from: Some(src.to_string()),
        seconds_shape: 0.0,
        seconds_ravines: 0.0,
        seconds_write: t.elapsed().as_secs_f64(),
        summit_target_m: spec.v("summit_m"),
        summit_stamped_m: max,
        max_m: dmax,
        max_at_m: det.world(dmi % det.grid.w, dmi / det.grid.w),
        edifice_m: f64::NAN,
        apron_gradient: f64::NAN,
        apron_rays: 0,
        apron_at_fade_m: f64::NAN,
        fade_uplift_mean_m: f64::NAN,
        fade_uplift_max_m: f64::NAN,
        fade_samples: 0,
        uplift_max_m: f64::NAN,
        flank: Vec::new(),
        fit: None,
        ravines: None,
        audit: Audit {
            new_wet: d.stats.new_wet,
            new_dry: d.stats.new_dry,
            sea_moved: 0,
        },
        wet_dry: WetDry::count(&det.grid.data, prep.base.sea_level_m),
        heights_blake3: hash_f32(&det.grid.data),
        structure: None,
        detail: Some(d),
    })
}

fn fit_row(name: &str, f: Option<&ProfileFit>) -> String {
    match f {
        None => format!("| {name} | (no fit) | | | | | | | | |\n"),
        Some(f) => format!(
            "| {name} | {:.0} | {:.1} / {:.0} | {:.2} ({:.0}) | {:.1} ({:.0}) | {:.3} | {:.4} ({:.0}) | {:.1} | {:.1} | {:.1} |\n",
            f.summit_bin_m,
            f.break_r_m * 1e-3,
            f.break_h_m,
            f.power_p,
            f.power_rmse_m,
            f.exp_r0_m * 1e-3,
            f.exp_rmse_m,
            f.cone_relief_ratio,
            f.apron_gradient,
            f.apron_rmse_m,
            f.cone_mean_deg,
            f.cone_top_deg,
            f.cone_steepest_deg
        ),
    }
}

#[allow(clippy::too_many_lines)] // report layout: one table after another
fn readme(run: &VolcanoRun, spec: &Spec, req: &StampRequest) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(s, "# WG-VO stratovolcano arms\n");
    let _ = writeln!(
        s,
        "Made by `scrib-worldgen stamp-volcano --base {} --params {} --out {}` (`sf_worldgen::stratovolcano`).\n",
        req.base.display(),
        req.params.display(),
        req.out.display()
    );
    let _ = writeln!(
        s,
        "Summit: {} ; reference level {:.1} m (quantile {} of the base on the foot ring); coast distance at the summit \
         {:.0} m; setup (read, coast distance, centre, reference fits) {:.1} s. Params: {} declared, {} Guess.\n",
        run.centre_rule,
        run.ref_m,
        spec.v("ref_quantile"),
        run.coast_distance_at_summit_m,
        run.seconds_setup,
        run.param_count,
        run.guess_count
    );
    let _ = writeln!(
        s,
        "Apron gradient (arms table): median over rays of the least-squares slope of the final heights from the edifice \
         foot (R(theta) x blend_out + 2 x warp) to the coast or apron_inner_m; on a real base it includes the base relief \
         left after flattening. The profile-fit apron (fits table) is the azimuthal-median trend beyond the fitted break.\n"
    );
    let _ = writeln!(s, "## Arms\n");
    let _ = writeln!(
        s,
        "| Arm | Features | Shape s | Ravines s | Write s | Max m (target) | Max at m | H_e m | Apron gradient (rays) | Audit (new wet, new dry, sea moved) | blake3 |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|---|");
    for a in &run.arms {
        let f = &a.features;
        let feats: Vec<&str> = [
            (f.caldera, "caldera"),
            (f.secondary, "secondary"),
            (f.subpeaks, "sub-peaks"),
            (f.collapse, "collapse"),
            (f.rifts, "rifts"),
            (f.ravines, "ravines"),
        ]
        .iter()
        .filter(|x| x.0)
        .map(|x| x.1)
        .collect();
        let feats = match &a.detail_from {
            Some(src) => format!("detail filter of {src}"),
            None if feats.is_empty() => "profile + apron + asymmetry".into(),
            None => feats.join(", "),
        };
        let _ = writeln!(
            s,
            "| {} | {} | {:.2} | {:.2} | {:.2} | {:.1} ({:.0}) | ({:.0}, {:.0}) | {:.0} | {:.4} ({}) | {}, {}, {} | `{}` |",
            a.name,
            feats,
            a.seconds_shape,
            a.seconds_ravines,
            a.seconds_write,
            a.max_m,
            a.summit_target_m,
            a.max_at_m[0],
            a.max_at_m[1],
            a.edifice_m,
            a.apron_gradient,
            a.apron_rays,
            a.audit.new_wet,
            a.audit.new_dry,
            a.audit.sea_moved,
            &a.heights_blake3[..16]
        );
    }
    let _ = writeln!(s, "\n## Flanks and apron\n");
    let _ = writeln!(
        s,
        "Mode `{:?}`. Flank: mean local slope of the land samples on a ring (+- fit_bin_m / 2) around the final summit, \
         and the secant from the maximum to the ring's mean height; base values on the same rings in brackets. Apron: \
         analytic apron at coast distance coast_fade_m; uplift = final - base on that contour within apron_inner_m.\n",
        spec.mode
    );
    let _ = writeln!(
        s,
        "| Arm | Set | Uplift max m | Apron at fade m | Fade uplift mean / max m (samples) | Rings: slope deg (base), secant deg (base) |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|");
    for a in run.arms.iter().filter(|a| a.detail.is_none()) {
        let set: Vec<String> = a.set.iter().map(|(k, v)| format!("{k} = {v}")).collect();
        let rings: Vec<String> = a
            .flank
            .iter()
            .map(|f| {
                format!(
                    "{:.0} km: {:.1} ({:.1}), {:.1} ({:.1})",
                    f.r_m * 1e-3,
                    f.slope_deg,
                    f.base_slope_deg,
                    f.secant_deg,
                    f.base_secant_deg
                )
            })
            .collect();
        let _ = writeln!(
            s,
            "| {} | {} | {:.0} | {:.1} | {:.1} / {:.1} ({}) | {} |",
            a.name,
            set.join(", "),
            a.uplift_max_m,
            a.apron_at_fade_m,
            a.fade_uplift_mean_m,
            a.fade_uplift_max_m,
            a.fade_samples,
            rings.join("; ")
        );
    }
    let _ = writeln!(s, "\n## Ravines\n");
    let _ = writeln!(
        s,
        "| Arm | Crop (x0, y0, w, h) | Steps (erosion / talus) | Fractures | Incision mean / p90 / p99 / max m | Talus moved km3 | Fields / erosion / talus s |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|---|");
    for a in &run.arms {
        if let Some(r) = &a.ravines {
            let _ = writeln!(
                s,
                "| {} | {:?} | {} / {} | {} | {:.1} / {:.1} / {:.1} / {:.1} | {:.3} | {:.1} / {:.1} / {:.1} |",
                a.name,
                r.crop,
                r.steps,
                r.thermal_steps,
                r.fractures,
                r.incision_mean_m,
                r.incision_p90_m,
                r.incision_p99_m,
                r.incision_max_m,
                r.thermal_moved_m3 * 1e-9,
                r.seconds_fields,
                r.seconds_erosion,
                r.seconds_thermal
            );
        }
        if let Some(d) = &a.detail {
            let st = &d.stats;
            let _ = writeln!(
                s,
                "| {} (detail) | {}x{} @ {} m, origin ({:.0}, {:.0}) | filter {:.2} s | | dh mean abs {:.2} / p99 abs {:.2} / max abs {:.2} m, mean {:.2} m | | |",
                a.name,
                d.dims[0],
                d.dims[1],
                d.spacing_m,
                d.origin_m[0],
                d.origin_m[1],
                d.seconds_filter,
                st.mean_abs_m,
                st.p99_abs_m,
                st.max_abs_m,
                st.mean_m
            );
        }
    }
    let _ = writeln!(s, "\n## Radial profile fits (azimuthal median, bin {} m, to {} km)\n", spec.v("fit_bin_m"), spec.v("fit_max_r_m") * 1e-3);
    let _ = writeln!(
        s,
        "Two-part fit: power cone `z_b + (h0 - z_b)(1 - r/r_b)^p` to the break r_b, straight apron beyond (break by least total squared error); exponential `a + B exp(-r/r0)` on the same cone part for comparison. Slopes are of the azimuthal median, so local slopes are steeper.\n"
    );
    let _ = writeln!(
        s,
        "| Profile | Summit bin m | Break km / m | Power p (rmse m) | Exp r0 km (rmse m) | Cone relief / r_b | Apron gradient (rmse m) | Cone mean deg | Top quarter deg | Steepest bin deg |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|");
    for (name, f) in &run.references {
        s.push_str(&fit_row(name, f.as_ref()));
    }
    for a in &run.arms {
        if a.detail.is_none() {
            s.push_str(&fit_row(&a.name, a.fit.as_ref()));
        }
    }
    let _ = writeln!(
        s,
        "\n## Channel structure on the edifice window ({:.1} km2 channels, {} m window)\n",
        STRUCTURE_THRESHOLD_M2 * 1e-6,
        STRUCTURE_WINDOW_M
    );
    let _ = writeln!(
        s,
        "Coherence 1 = parallel channels (radial grooves score high), lower = branching (`sf_worldgen::structure`).\n"
    );
    let _ = writeln!(
        s,
        "| Field | Channel km | Drainage density km/km2 | Coherence mean / median / p90 | Junctions | Junction angle mean / median deg |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|");
    let srow = |s: &mut String, name: &str, t: &crate::structure::Structure| {
        let _ = writeln!(
            s,
            "| {name} | {:.0} | {:.3} | {:.3} / {:.3} / {:.3} | {} | {:.1} / {:.1} |",
            t.length_km,
            t.drainage_density,
            t.coherence_mean,
            t.coherence_median,
            t.coherence_p90,
            t.junctions,
            t.junction_mean_deg,
            t.junction_median_deg
        );
    };
    for (n, t) in &run.reference_structure {
        srow(&mut s, n, t);
    }
    for a in &run.arms {
        if let Some(t) = &a.structure {
            srow(&mut s, &a.name, t);
        }
    }
    let _ = writeln!(s, "\n## Parameters\n");
    let _ = writeln!(s, "| Name | Value | Unit | Range | Source | Help |");
    let _ = writeln!(s, "|---|---|---|---|---|---|");
    for (n, p) in &spec.params {
        let _ = writeln!(s, "| {n} | {} | {} | {} .. {} | {} | {} |", p.v, p.unit, p.min, p.max, p.source, p.help);
    }
    s
}
