//! **Landform-first macro runner** (WG-S4): layout → uplift-driven steady-state landscape → optional volcano.
//!
//! A recipe with `pipeline = "landform"` ([`LandformRecipe`]) runs here instead of [`crate::run`]'s S2 pipeline:
//!
//! 1. **Layout** ([`crate::layout`]): the masked source is sampled at `S × guide position` on two grids (the
//!    macro grid and the coarser landscape grid) and reduced to its land/sea mask, a Gaussian low-passed relief
//!    (cutoff `layout_cutoff_m`, world metres) and a low-passed sea floor. The region map is the third guide. The
//!    source surface never feeds the final heights.
//! 2. **Landscape** ([`crate::landscape`]): uplift `U(x) = c_r · (floor + (1 − floor) · L(x)/L_ref)` from the
//!    low-passed relief L (`L_ref` = its 99th percentile over land), `c_r` per region blended by a cubic B-spline
//!    over the region grid; started from a low surface (`start_fraction` of the target-scaled layout + hash
//!    noise, sea fixed); run to steady state on the landscape grid. **Calibration:** with n = 1 the steady state of
//!    stream power scales with U/K (`S = (U/K) A^{−m}`, so heights along a channel are `(U/K) χ`, χ = ∫ A^{−m} dl;
//!    Perron & Royden, "An integral approach to bedrock river profile analysis", ESPL 38, 2013; Cordonnier et al.
//!    2016 use the same steady-state relation to set heights from uplift). The talus clamp breaks the exact
//!    linearity, so `c_r` is found by iterative rescaling: run to steady, measure each region's p95 height, scale
//!    `c_r` (and warm-start the heights) by target / measured, repeat `calib_rounds` times. Region target: p95 =
//!    `relief × p95 of L` over the region (the layout's relative relief, scaled). The calibrated field is then
//!    bicubic-upsampled to the macro grid (mask re-imposed) and refined there to steady state (`refine_max_iterations`).
//! 3. **Volcano** ([`crate::volcano`]): parametric stratovolcano or exemplar stamp centred on the volcano region's
//!    highest layout sample, based on the median landscape height of the blend-end ring, blended C1; the
//!    parametric cone is then lightly eroded (the S2 stream stage with no uplift, erodibility × blend weight,
//!    MFD with a concentrating exponent, hash-noise seed × blend weight).
//!
//! WG-S5 arms choose the landscape per arm ([`LandscapeKey`]): `solver = "lem"` runs [`crate::lem`] (the Veloren
//! port) instead of [`crate::landscape`]; `rock` multiplies erodibility by the [`crate::lithology`] rock map,
//! `uplift_noise` modulates uplift, `rough_start` adds start noise, and `finite` replaces the macro refine by a
//! finite-time run: forcing = calibrated uplift / f, its steady p95 measured (warm start), then a run from the start
//! surface at `dt × finite_dt_ratio` until the land p95 reaches f × that, plus `finite_refine_steps` on the macro
//! grid. Each distinct key runs once; arms with equal keys share it.
//!
//! Outputs, per arm, as [`crate::run`]: guide v1 heights + meta, overview, crops, sheets, README with the numbers
//! (wall time, steps, residual, max height, relief per region, land/sea + wet/dry audit, channel structure
//! ([`crate::structure`]) and the S2b channel metrics ([`crate::metrics`])), for every arm and `[[compare]]` folder.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use serde::Deserialize;

use crate::guide::{Guide, RegionMap};
use crate::landscape::{LandscapeParams, LandscapeStats, evolve};
use crate::layout::{Layout, layout, sigma_for_cutoff};
use crate::lem::{self, LemParams, MfdLaw, Stop};
use crate::lithology::{RockMap, RockType, uplift_factor};
use crate::metrics::{ChannelMetrics, channel_metrics};
use crate::perturb;
use crate::recipe::{
    Column, Compare, Crop, GuideSection, OutputSection, Param, PlaceSection, Report, check_range,
    check_source,
};
use crate::resample::bicubic;
use crate::run::{
    CropAt, Frame, Overrides, SHEET_GAP_GRAY, SHEET_GAP_PX, frame_for, masked_source,
    region_weights, render_crop, resolve_crops, write_heights,
};
use crate::shade::{Gray, Light, contact_sheet, downsample, hillshade, write_png};
use crate::stream::{Flow, StreamParams, erode};
use crate::structure::{Structure, structure};
use crate::volcano::{Strato, blend_weight};
use crate::{Audit, Error, Grid, Result, WetDry, hash_f32};

/// Scalar parameters of a landform recipe.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandformParams {
    pub scale: Param,
    pub macro_spacing_m: Param,
    pub sea_margin_m: Param,
    pub layout_cutoff_m: Param,
    pub landscape_spacing_m: Param,
    pub stream_m: Param,
    pub land_k_dt: Param,
    pub flow_exponent: Param,
    pub uplift_floor: Param,
    pub rise_steps: Param,
    pub start_fraction: Param,
    pub start_noise_m: Param,
    pub start_noise_wavelength_m: Param,
    pub noise_seed: Param,
    pub steady_rel: Param,
    pub steady_max_iterations: Param,
    pub calib_rounds: Param,
    pub calib_round_iterations: Param,
    pub refine_max_iterations: Param,
    pub volcano_summit_m: Param,
    pub volcano_radius_m: Param,
    pub volcano_decay_m: Param,
    pub volcano_crater_radius_m: Param,
    pub volcano_crater_depth_m: Param,
    pub volcano_asym_amp: Param,
    pub volcano_asym_wavelength_m: Param,
    pub volcano_blend_start: Param,
    pub volcano_blend_end: Param,
    pub volcano_k_dt: Param,
    pub volcano_erosion_steps: Param,
    pub volcano_flow_exponent: Param,
    pub volcano_seed_amp_m: Param,
    pub volcano_seed_wavelength_m: Param,
    pub exemplar_base_quantile: Param,
    pub channel_area_km2: Param,
    pub coherence_window_m: Param,
    pub coherence_dir_steps: Param,
    pub light_azimuth_deg: Param,
    pub light_altitude_deg: Param,
    pub sea_gray: Param,
    pub overview_max_px: Param,
    pub crop_size_m: Param,
    pub crop_px: Param,
    // WG-S5 (optional; required by the arms that use them, see [`LandformRecipe::validate`]).
    /// LEM: slope exponent n.
    #[serde(default)]
    pub stream_n: Option<Param>,
    /// LEM: multiple-receiver exponent `p = base + slope_coef × mean slope`.
    #[serde(default)]
    pub mfd_exp_base: Option<Param>,
    #[serde(default)]
    pub mfd_exp_slope: Option<Param>,
    /// LEM: cap of the nonlinear diffusivity boost, × base D.
    #[serde(default)]
    pub nonlinear_max_ratio: Option<Param>,
    /// Rock-type map.
    #[serde(default)]
    pub rock_cell_m: Option<Param>,
    #[serde(default)]
    pub rock_jitter: Option<Param>,
    #[serde(default)]
    pub rock_warp_m: Option<Param>,
    #[serde(default)]
    pub rock_warp_wavelength_m: Option<Param>,
    #[serde(default)]
    pub rock_edge_band_m: Option<Param>,
    #[serde(default)]
    pub rock_edge_k: Option<Param>,
    /// Uplift noise.
    #[serde(default)]
    pub uplift_noise_amp: Option<Param>,
    #[serde(default)]
    pub uplift_noise_wavelength_m: Option<Param>,
    /// Rough (noisy) start, on top of `start_noise_m`.
    #[serde(default)]
    pub rough_start_m: Option<Param>,
    #[serde(default)]
    pub rough_start_wavelength_m: Option<Param>,
    /// Finite time.
    #[serde(default)]
    pub finite_fraction: Option<Param>,
    #[serde(default)]
    pub finite_dt_ratio: Option<Param>,
    #[serde(default)]
    pub finite_max_steps: Option<Param>,
    #[serde(default)]
    pub finite_refine_steps: Option<Param>,
}

impl LandformParams {
    /// The WG-S5 optional values, by name.
    fn optional(&self) -> [(&'static str, Option<&Param>); 18] {
        [
            ("stream_n", self.stream_n.as_ref()),
            ("mfd_exp_base", self.mfd_exp_base.as_ref()),
            ("mfd_exp_slope", self.mfd_exp_slope.as_ref()),
            ("nonlinear_max_ratio", self.nonlinear_max_ratio.as_ref()),
            ("rock_cell_m", self.rock_cell_m.as_ref()),
            ("rock_jitter", self.rock_jitter.as_ref()),
            ("rock_warp_m", self.rock_warp_m.as_ref()),
            ("rock_warp_wavelength_m", self.rock_warp_wavelength_m.as_ref()),
            ("rock_edge_band_m", self.rock_edge_band_m.as_ref()),
            ("rock_edge_k", self.rock_edge_k.as_ref()),
            ("uplift_noise_amp", self.uplift_noise_amp.as_ref()),
            ("uplift_noise_wavelength_m", self.uplift_noise_wavelength_m.as_ref()),
            ("rough_start_m", self.rough_start_m.as_ref()),
            ("rough_start_wavelength_m", self.rough_start_wavelength_m.as_ref()),
            ("finite_fraction", self.finite_fraction.as_ref()),
            ("finite_dt_ratio", self.finite_dt_ratio.as_ref()),
            ("finite_max_steps", self.finite_max_steps.as_ref()),
            ("finite_refine_steps", self.finite_refine_steps.as_ref()),
        ]
    }

    /// Value of an optional parameter (validated present before use).
    fn opt(&self, name: &str) -> f64 {
        self.optional()
            .iter()
            .find(|(n, _)| *n == name)
            .and_then(|(_, p)| p.map(|p| p.v))
            .unwrap_or(0.0)
    }

    fn all(&self) -> Vec<(&'static str, &Param)> {
        let mut v = self.required();
        v.extend(self.optional().into_iter().filter_map(|(n, p)| p.map(|p| (n, p))));
        v
    }

    fn required(&self) -> Vec<(&'static str, &Param)> {
        vec![
            ("scale", &self.scale),
            ("macro_spacing_m", &self.macro_spacing_m),
            ("sea_margin_m", &self.sea_margin_m),
            ("layout_cutoff_m", &self.layout_cutoff_m),
            ("landscape_spacing_m", &self.landscape_spacing_m),
            ("stream_m", &self.stream_m),
            ("land_k_dt", &self.land_k_dt),
            ("flow_exponent", &self.flow_exponent),
            ("uplift_floor", &self.uplift_floor),
            ("rise_steps", &self.rise_steps),
            ("start_fraction", &self.start_fraction),
            ("start_noise_m", &self.start_noise_m),
            ("start_noise_wavelength_m", &self.start_noise_wavelength_m),
            ("noise_seed", &self.noise_seed),
            ("steady_rel", &self.steady_rel),
            ("steady_max_iterations", &self.steady_max_iterations),
            ("calib_rounds", &self.calib_rounds),
            ("calib_round_iterations", &self.calib_round_iterations),
            ("refine_max_iterations", &self.refine_max_iterations),
            ("volcano_summit_m", &self.volcano_summit_m),
            ("volcano_radius_m", &self.volcano_radius_m),
            ("volcano_decay_m", &self.volcano_decay_m),
            ("volcano_crater_radius_m", &self.volcano_crater_radius_m),
            ("volcano_crater_depth_m", &self.volcano_crater_depth_m),
            ("volcano_asym_amp", &self.volcano_asym_amp),
            ("volcano_asym_wavelength_m", &self.volcano_asym_wavelength_m),
            ("volcano_blend_start", &self.volcano_blend_start),
            ("volcano_blend_end", &self.volcano_blend_end),
            ("volcano_k_dt", &self.volcano_k_dt),
            ("volcano_erosion_steps", &self.volcano_erosion_steps),
            ("volcano_flow_exponent", &self.volcano_flow_exponent),
            ("volcano_seed_amp_m", &self.volcano_seed_amp_m),
            ("volcano_seed_wavelength_m", &self.volcano_seed_wavelength_m),
            ("exemplar_base_quantile", &self.exemplar_base_quantile),
            ("channel_area_km2", &self.channel_area_km2),
            ("coherence_window_m", &self.coherence_window_m),
            ("coherence_dir_steps", &self.coherence_dir_steps),
            ("light_azimuth_deg", &self.light_azimuth_deg),
            ("light_altitude_deg", &self.light_altitude_deg),
            ("sea_gray", &self.sea_gray),
            ("overview_max_px", &self.overview_max_px),
            ("crop_size_m", &self.crop_size_m),
            ("crop_px", &self.crop_px),
        ]
    }
}

/// Per-region values of a landform recipe (all optional except in `default`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandformRegion {
    /// Target p95 height = relief × p95 of the low-passed source relief over the region.
    pub relief: Option<f64>,
    /// K multiplier.
    pub erodibility: Option<f64>,
    /// Critical hillslope angle, degrees.
    pub talus_deg: Option<f64>,
    /// D / U, metres: steady-state hillslope curvature length (`h'' = −U/D`).
    pub hillslope_m: Option<f64>,
}

const REGION_COLUMNS: [&str; 4] = ["relief", "erodibility", "talus_deg", "hillslope_m"];

impl LandformRegion {
    fn values(&self) -> [Option<f64>; 4] {
        [
            self.relief,
            self.erodibility,
            self.talus_deg,
            self.hillslope_m,
        ]
    }
}

/// The volcano region and the optional exemplar heightfield.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolcanoSection {
    /// Legend name of the volcano region.
    pub region: String,
    /// Guide v1 folder of the exemplar (arms with `volcano = "exemplar"`).
    pub exemplar_dir: Option<PathBuf>,
}

/// Volcano of an arm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VolcanoKind {
    #[default]
    None,
    Parametric,
    Exemplar,
}

/// One arm of a landform recipe (the flags are independent recipe switches).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct LandformArm {
    pub name: String,
    #[serde(default)]
    pub help: String,
    #[serde(default)]
    pub volcano: VolcanoKind,
    /// Landscape solver (WG-S4 `evolve` or the WG-S5 Veloren port).
    #[serde(default)]
    pub solver: Solver,
    /// Erodibility × the rock-type map.
    #[serde(default)]
    pub rock: bool,
    /// Uplift × (1 + low-frequency noise).
    #[serde(default)]
    pub uplift_noise: bool,
    /// Start surface + `rough_start_m` noise.
    #[serde(default)]
    pub rough_start: bool,
    /// Stop at a fraction of the steady-state relief (LEM only).
    #[serde(default)]
    pub finite: bool,
}

/// Landscape solver of an arm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Solver {
    #[default]
    S4,
    Lem,
}

/// What makes two arms' landscapes differ (arms with equal keys share one landscape run).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct LandscapeKey {
    pub solver: Solver,
    pub rock: bool,
    pub uplift_noise: bool,
    pub rough_start: bool,
    pub finite: bool,
}

impl LandscapeKey {
    fn of(a: &LandformArm) -> Self {
        Self {
            solver: a.solver,
            rock: a.rock,
            uplift_noise: a.uplift_noise,
            rough_start: a.rough_start,
            finite: a.finite,
        }
    }

    fn label(self) -> String {
        let mut s = match self.solver {
            Solver::S4 => "S4 solver".to_string(),
            Solver::Lem => "LEM (Veloren port)".to_string(),
        };
        for (on, t) in [
            (self.rock, "rock K"),
            (self.uplift_noise, "U noise"),
            (self.rough_start, "rough start"),
            (self.finite, "finite time"),
        ] {
            if on {
                s.push_str(" + ");
                s.push_str(t);
            }
        }
        s
    }
}

/// One rock type of the recipe (`[rocks.<name>]`).
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RockEntry {
    /// K multiplier.
    pub k: f64,
    /// Share of the map.
    pub share: f64,
}

/// A landform recipe file (`pipeline = "landform"`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandformRecipe {
    pub pipeline: String,
    pub guide: GuideSection,
    pub output: OutputSection,
    pub params: LandformParams,
    #[serde(default)]
    pub place: PlaceSection,
    pub volcano: Option<VolcanoSection>,
    pub region_params: BTreeMap<String, Column>,
    pub regions: BTreeMap<String, LandformRegion>,
    /// Unit, range and source of the rock columns (`k`, `share`), for arms with `rock = true`.
    #[serde(default)]
    pub rock_params: BTreeMap<String, Column>,
    #[serde(default)]
    pub rocks: BTreeMap<String, RockEntry>,
    #[serde(rename = "crop", default)]
    pub crops: Vec<Crop>,
    #[serde(rename = "arm")]
    pub arms: Vec<LandformArm>,
    pub report: Option<Report>,
    #[serde(rename = "compare", default)]
    pub compares: Vec<Compare>,
}

/// Resolved per-region values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegionResolved {
    pub relief: f64,
    pub erodibility: f64,
    pub tan_talus: f64,
    pub hillslope_m: f64,
}

impl LandformRecipe {
    /// Parses and validates a landform recipe.
    pub fn parse(text: &str) -> Result<Self> {
        let r: Self = toml::from_str(text)?;
        r.validate()?;
        Ok(r)
    }

    /// Checks ranges, sources, the region table and the arms.
    #[allow(clippy::too_many_lines)]
    pub fn validate(&self) -> Result<()> {
        if self.pipeline != "landform" {
            return Err(Error::Invalid("pipeline must be \"landform\"".into()));
        }
        for (name, p) in self.params.all() {
            check_source(name, &p.source)?;
            check_range(name, p.v, p.min, p.max)?;
        }
        let p = &self.params;
        if p.stream_m.v <= 0.0 || p.layout_cutoff_m.v < 0.0 {
            return Err(Error::Invalid(
                "stream_m must be > 0, layout_cutoff_m >= 0".into(),
            ));
        }
        if p.volcano_blend_end.v < p.volcano_blend_start.v
            || p.volcano_crater_radius_m.v >= p.volcano_radius_m.v
            || p.volcano_decay_m.v <= 0.0
            || p.start_noise_wavelength_m.v <= 0.0
            || p.volcano_asym_wavelength_m.v <= 0.0
            || p.volcano_seed_wavelength_m.v <= 0.0
        {
            return Err(Error::Invalid(
                "volcano: blend_end >= blend_start, crater_radius < radius, positive lengths".into(),
            ));
        }
        for col in REGION_COLUMNS {
            let c = self
                .region_params
                .get(col)
                .ok_or_else(|| Error::Invalid(format!("[region_params.{col}] missing")))?;
            check_source(col, &c.source)?;
        }
        let default = self
            .regions
            .get("default")
            .ok_or_else(|| Error::Invalid("[regions.default] missing".into()))?;
        if default.values().iter().any(Option::is_none) {
            return Err(Error::Invalid(format!(
                "[regions.default] needs {}",
                REGION_COLUMNS.join(", ")
            )));
        }
        for (region, v) in &self.regions {
            for (col, val) in REGION_COLUMNS.iter().zip(v.values()) {
                if let Some(x) = val {
                    let c = &self.region_params[*col];
                    check_range(&format!("regions.{region:?}.{col}"), x, c.min, c.max)?;
                }
            }
        }
        if self.arms.is_empty() {
            return Err(Error::Invalid("no [[arm]]".into()));
        }
        for a in &self.arms {
            if a.volcano != VolcanoKind::None && self.volcano.is_none() {
                return Err(Error::Invalid(format!(
                    "arm {}: needs a [volcano] section",
                    a.name
                )));
            }
            if a.volcano == VolcanoKind::Exemplar
                && self.volcano.as_ref().is_some_and(|v| v.exemplar_dir.is_none())
            {
                return Err(Error::Invalid(format!(
                    "arm {}: needs [volcano] exemplar_dir",
                    a.name
                )));
            }
            if a.finite && a.solver != Solver::Lem {
                return Err(Error::Invalid(format!(
                    "arm {}: finite = true needs solver = \"lem\"",
                    a.name
                )));
            }
            let mut need: Vec<&str> = Vec::new();
            if a.solver == Solver::Lem {
                need.extend(["stream_n", "mfd_exp_base", "mfd_exp_slope", "nonlinear_max_ratio"]);
            }
            if a.rock {
                need.extend([
                    "rock_cell_m",
                    "rock_jitter",
                    "rock_warp_m",
                    "rock_warp_wavelength_m",
                    "rock_edge_band_m",
                    "rock_edge_k",
                ]);
            }
            if a.uplift_noise {
                need.extend(["uplift_noise_amp", "uplift_noise_wavelength_m"]);
            }
            if a.rough_start {
                need.extend(["rough_start_m", "rough_start_wavelength_m"]);
            }
            if a.finite {
                need.extend([
                    "finite_fraction",
                    "finite_dt_ratio",
                    "finite_max_steps",
                    "finite_refine_steps",
                ]);
            }
            let opt = p.optional();
            for n in need {
                if opt.iter().any(|(k, v)| *k == n && v.is_none()) {
                    return Err(Error::Invalid(format!("arm {}: needs [params] {n}", a.name)));
                }
            }
            if a.rock {
                for col in ["k", "share"] {
                    let c = self.rock_params.get(col).ok_or_else(|| {
                        Error::Invalid(format!("arm {}: needs [rock_params.{col}]", a.name))
                    })?;
                    check_source(col, &c.source)?;
                }
                if self.rocks.is_empty() || self.rocks.values().all(|r| r.share <= 0.0) {
                    return Err(Error::Invalid(format!(
                        "arm {}: needs [rocks.*] with a positive share",
                        a.name
                    )));
                }
                for (name, r) in &self.rocks {
                    for (col, x) in [("k", r.k), ("share", r.share)] {
                        let c = &self.rock_params[col];
                        check_range(&format!("rocks.{name:?}.{col}"), x, c.min, c.max)?;
                    }
                }
            }
        }
        if p.stream_n.as_ref().is_some_and(|n| n.v < 1.0) {
            return Err(Error::Invalid("stream_n must be >= 1 (local Newton)".into()));
        }
        for c in &self.crops {
            if c.peak_of.is_some() == c.region.is_some() {
                return Err(Error::Invalid(format!(
                    "crop {}: exactly one of peak_of, region",
                    c.name
                )));
            }
        }
        Ok(())
    }

    /// Resolved values of a legend name (falls back to `default` per value).
    pub fn resolve(&self, name: &str) -> RegionResolved {
        let d = &self.regions["default"];
        let r = self.regions.get(name).unwrap_or(d);
        let pick = |a: Option<f64>, b: Option<f64>| a.or(b).unwrap_or(0.0);
        RegionResolved {
            relief: pick(r.relief, d.relief),
            erodibility: pick(r.erodibility, d.erodibility),
            tan_talus: pick(r.talus_deg, d.talus_deg).to_radians().tan(),
            hillslope_m: pick(r.hillslope_m, d.hillslope_m),
        }
    }

    /// (declared values, of which `Guess`).
    pub fn census(&self) -> (usize, usize) {
        let (mut v, mut g) = (0, 0);
        for (_, p) in self.params.all() {
            v += 1;
            g += usize::from(p.source == "Guess");
        }
        for r in self.regions.values() {
            for (col, val) in REGION_COLUMNS.iter().zip(r.values()) {
                if val.is_some() {
                    v += 1;
                    g += usize::from(self.region_params[*col].source == "Guess");
                }
            }
        }
        for col in ["k", "share"] {
            if let Some(c) = self.rock_params.get(col) {
                v += self.rocks.len();
                g += self.rocks.len() * usize::from(c.source == "Guess");
            }
        }
        (v, g)
    }
}

/// Guide coordinates (fractional samples) of a world position.
fn guide_pos(guide: &Guide, src_spacing: f64, s: f64, p: [f64; 2]) -> [f64; 2] {
    [
        (p[0] / s - guide.origin_m[0]) / src_spacing,
        (guide.origin_m[1] - p[1] / s) / src_spacing,
    ]
}

/// Context shared by the stages: guide, source, region map, resolved values.
struct Ctx<'a> {
    guide: &'a Guide,
    src: &'a Grid,
    reg: &'a RegionMap,
    s: f64,
    resolved: Vec<RegionResolved>,
}

impl Ctx<'_> {
    /// Bicubic sample of the masked source on `frame`.
    fn sample(&self, frame: &Frame) -> Grid {
        let mut data = vec![0.0; frame.w * frame.h];
        data.par_chunks_mut(frame.w).enumerate().for_each(|(j, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let [c, r] = guide_pos(
                    self.guide,
                    self.src.spacing,
                    self.s,
                    frame.world(x as f64, j as f64),
                );
                *v = bicubic(self.src, c, r);
            }
        });
        Grid {
            w: frame.w,
            h: frame.h,
            spacing: frame.spacing_m,
            data,
        }
    }

    /// Legend index of the region pixel holding each sample of `frame`.
    fn dominant(&self, frame: &Frame) -> Vec<u8> {
        let mut out = vec![0u8; frame.w * frame.h];
        out.par_chunks_mut(frame.w).enumerate().for_each(|(j, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let [c, r] = guide_pos(
                    self.guide,
                    self.src.spacing,
                    self.s,
                    frame.world(x as f64, j as f64),
                );
                let px = ((c + 0.5) * self.src.spacing / self.reg.cell_m).floor() as isize;
                let py = ((r + 0.5) * self.src.spacing / self.reg.cell_m).floor() as isize;
                *v = self.reg.at_clamped(px, py);
            }
        });
        out
    }

    /// Per-sample blend (cubic B-spline over the region grid) of a per-legend-index value.
    fn blend(&self, frame: &Frame, value: impl Fn(usize) -> f64 + Sync) -> Vec<f64> {
        let mut out = vec![0.0; frame.w * frame.h];
        out.par_chunks_mut(frame.w).enumerate().for_each(|(j, row)| {
            let mut wts = Vec::with_capacity(16);
            for (x, v) in row.iter_mut().enumerate() {
                let [c, r] = guide_pos(
                    self.guide,
                    self.src.spacing,
                    self.s,
                    frame.world(x as f64, j as f64),
                );
                region_weights(self.reg, self.src.spacing, c, r, &mut wts);
                *v = wts
                    .iter()
                    .map(|&(idx, w)| w * value(usize::from(idx)))
                    .sum();
            }
        });
        out
    }
}

/// p95 of `values` over the samples of each legend index (land only); 0 where a region has no sample.
fn p95_by_region(values: &[f64], dom: &[u8], land: &[bool]) -> Vec<(usize, f64)> {
    let mut by: Vec<Vec<f64>> = vec![Vec::new(); 256];
    for ((&v, &d), &l) in values.iter().zip(dom).zip(land) {
        if l {
            by[usize::from(d)].push(v);
        }
    }
    by.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return (0, 0.0);
            }
            v.sort_by(f64::total_cmp);
            (v.len(), v[((v.len() - 1) as f64 * 0.95).round() as usize])
        })
        .collect()
}

/// One calibration round.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Round {
    pub stats: LandscapeStats,
    /// Largest |measured / target − 1| over regions with samples, after this round.
    pub max_rel_error: f64,
}

/// Per-region calibration row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CalibRow {
    pub name: String,
    pub samples: usize,
    pub layout_p95_m: f64,
    pub target_p95_m: f64,
    /// Final uplift scale `c_r`, metres per step at `L = L_ref`.
    pub uplift_dt_m: f64,
    /// Measured p95 after the last round (landscape grid).
    pub measured_p95_m: f64,
    /// Finite-time landscapes: p95 of the boosted forcing's steady state and at the end of the finite run.
    pub steady_boost_p95_m: f64,
    pub finite_p95_m: f64,
}

/// Relief of one region in one field.
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Relief {
    pub samples: usize,
    pub min: f64,
    pub median: f64,
    pub p95: f64,
    pub max: f64,
}

fn relief_by_region(h: &[f64], dom: &[u8], sea: f64) -> Vec<Relief> {
    let mut by: Vec<Vec<f64>> = vec![Vec::new(); 256];
    for (&v, &d) in h.iter().zip(dom) {
        if v >= sea {
            by[usize::from(d)].push(v);
        }
    }
    by.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return Relief::default();
            }
            v.sort_by(f64::total_cmp);
            let q = |f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
            Relief {
                samples: v.len(),
                min: v[0],
                median: q(0.5),
                p95: q(0.95),
                max: v[v.len() - 1],
            }
        })
        .collect()
}

/// Numbers of one field (an arm or a compare folder).
#[derive(Debug, Clone, serde::Serialize)]
pub struct FieldReport {
    pub name: String,
    pub help: String,
    /// Wall seconds: layout (shared), landscape (shared), volcano, metrics, render.
    pub layout_s: f64,
    pub landscape_s: f64,
    pub volcano_s: f64,
    pub metrics_s: f64,
    pub render_s: f64,
    /// Landscape steps (all rounds on the landscape grid, refine on the macro grid) and volcano erosion steps.
    pub landscape_steps: usize,
    pub refine_steps: usize,
    pub volcano_steps: usize,
    /// Final residual of the refine run (or the last landscape round): relative and p99 metres per step.
    pub residual_rel: f64,
    pub residual_p99_m: f64,
    pub converged: bool,
    /// Landscape the arm uses ([`LandscapeKey::label`]).
    pub landscape: String,
    /// Finite-time landscapes: land p95 at the end / the boosted steady-state p95.
    pub finite_fraction: Option<f64>,
    pub max_height_m: f64,
    pub wet_dry: WetDry,
    /// Class and sea changes against the layout (mask + low-passed sea floor).
    pub audit: Audit,
    pub heights_blake3: String,
    #[serde(skip)]
    pub relief: Vec<Relief>,
    pub channels: ChannelMetrics,
    pub structure: Structure,
    pub volcano_note: String,
}

/// The finite-time run of one landscape.
#[derive(Debug, Clone, serde::Serialize)]
pub struct FiniteReport {
    /// Recipe stop fraction f and time-step ratio.
    pub fraction: f64,
    pub dt_ratio: f64,
    /// Steady state of the boosted forcing (calibrated uplift / f).
    pub steady_steps: usize,
    pub steady_p95_m: f64,
    /// Stop rule: land p95 ≥ f × steady p95.
    pub target_p95_m: f64,
    pub steps: usize,
    pub stopped: bool,
    pub stop_p95_m: f64,
    pub fraction_at_stop: f64,
    /// Macro-grid refine steps (same dt) and the land p95 / fraction at the end.
    pub refine_steps: usize,
    pub final_p95_m: f64,
    pub fraction_final: f64,
    pub steady: LandscapeStats,
    pub run: LandscapeStats,
}

/// One landscape (shared by the arms with its key).
pub struct LandscapeOut {
    pub key: LandscapeKey,
    /// Heights on the macro grid.
    pub h1: Grid,
    pub rounds: Vec<Round>,
    pub refine: Option<LandscapeStats>,
    pub calib: Vec<CalibRow>,
    pub last: LandscapeStats,
    pub landscape_steps: usize,
    pub seconds: f64,
    pub finite: Option<FiniteReport>,
    /// Erodibility multiplier over land (landscape grid): min, mean, max.
    pub k_stats: [f64; 3],
    /// Land samples per rock type (landscape grid).
    pub rock_shares: Vec<usize>,
    /// Uplift factor over land: min, max.
    pub u_stats: [f64; 2],
}

/// The volcano stamp's numbers.
#[derive(Debug, Clone, Default, serde::Serialize)]
struct VolcanoInfo {
    centre_m: [f64; 2],
    base_at_centre_m: f64,
    strato: Option<Strato>,
    radius_m: f64,
}

/// Runs a landform recipe (`text` is the file's content, already read).
#[allow(clippy::too_many_lines)]
pub fn run(recipe_path: &Path, text: &str, ov: &Overrides) -> Result<()> {
    let recipe = LandformRecipe::parse(text)?;
    let p = &recipe.params;
    let mut guide = Guide::read(&recipe.guide.dir)?;
    if let Some(o) = recipe.guide.origin_m {
        guide.origin_m = o;
    }
    let reg = guide
        .regions
        .clone()
        .ok_or_else(|| Error::Invalid("a landform recipe needs a region grid".into()))?;
    for name in recipe
        .regions
        .keys()
        .filter(|k| *k != "default")
        .chain(recipe.volcano.as_ref().map(|v| &v.region))
    {
        if reg.find(name).is_none() {
            return Err(Error::Invalid(format!(
                "region {name:?}: no such name in the guide's legend"
            )));
        }
    }
    let sea = guide.sea_level_m;
    let s = p.scale.v;
    let out_dir = ov.out.clone().unwrap_or_else(|| recipe.output.dir.clone());
    std::fs::create_dir_all(&out_dir)?;
    let (src, masked) = masked_source(&guide, &recipe.place.mask_to_sea);
    let spacing = ov.macro_spacing_m.unwrap_or(p.macro_spacing_m.v);
    let lspacing = p.landscape_spacing_m.v.max(spacing);
    let frame = frame_for(&guide, s, p.sea_margin_m.v, &src, spacing)?;
    let lframe = frame_for(&guide, s, p.sea_margin_m.v, &src, lspacing)?;
    let crops = resolve_crops(&guide, &recipe.crops, p.crop_size_m.v, s, &src)?;
    let default = recipe.resolve("default");
    let resolved: Vec<RegionResolved> = (0..256)
        .map(|i| {
            if reg.names[i].is_empty() {
                default
            } else {
                recipe.resolve(&reg.names[i])
            }
        })
        .collect();
    let ctx = Ctx {
        guide: &guide,
        src: &src,
        reg: &reg,
        s,
        resolved,
    };
    let light = Light {
        azimuth_deg: p.light_azimuth_deg.v,
        altitude_deg: p.light_altitude_deg.v,
        sea_gray: p.sea_gray.v as u8,
    };
    println!(
        "scrib-worldgen landform: guide {}x{} @ {} m, masked {masked}; macro {}x{} @ {spacing} m ({:.1} M), landscape {}x{} @ {lspacing} m ({:.1} M), S = {s}",
        guide.w,
        guide.h,
        guide.spacing_m,
        frame.w,
        frame.h,
        (frame.w * frame.h) as f64 * 1e-6,
        lframe.w,
        lframe.h,
        (lframe.w * lframe.h) as f64 * 1e-6
    );

    // 1. Layout.
    let t = Instant::now();
    let sigma = sigma_for_cutoff(p.layout_cutoff_m.v);
    let lay = layout(&ctx.sample(&frame), sea, sigma);
    let lay_l = if lspacing.to_bits() == spacing.to_bits() {
        lay.clone()
    } else {
        layout(&ctx.sample(&lframe), sea, sigma)
    };
    let dom = ctx.dominant(&frame);
    let dom_l = ctx.dominant(&lframe);
    let layout_s = t.elapsed().as_secs_f64();
    let layout_field: Vec<f64> = lay
        .land
        .iter()
        .zip(&lay.sea_floor)
        .map(|(&l, &f)| if l { sea } else { f })
        .collect();

    // 2. Landscapes on the landscape grid, calibrated: one per distinct arm key (arms with equal keys share it).
    let mut land_l: Vec<f64> = lay_l
        .relief
        .data
        .iter()
        .zip(&lay_l.land)
        .filter(|(_, l)| **l)
        .map(|(v, _)| *v)
        .collect();
    land_l.sort_by(f64::total_cmp);
    let l_ref = if land_l.is_empty() {
        1.0
    } else {
        land_l[((land_l.len() - 1) as f64 * 0.99).round() as usize].max(f64::MIN_POSITIVE)
    };
    let layout_p95 = p95_by_region(&lay_l.relief.data, &dom_l, &lay_l.land);
    let target: Vec<f64> = (0..256)
        .map(|i| ctx.resolved[i].relief * layout_p95[i].1)
        .collect();
    let floor = p.uplift_floor.v;
    let seed = p.noise_seed.v as u64;
    let rock_types: Vec<RockType> = recipe
        .rocks
        .values()
        .map(|r| RockType { k: r.k, share: r.share })
        .collect();
    let rock_map = RockMap {
        cell_m: p.opt("rock_cell_m"),
        jitter: p.opt("rock_jitter"),
        warp_m: p.opt("rock_warp_m"),
        warp_wavelength_m: p.opt("rock_warp_wavelength_m"),
        edge_band_m: p.opt("rock_edge_band_m"),
        edge_k: p.opt("rock_edge_k"),
        seed: seed + 3,
    };
    let compute = |key: LandscapeKey| -> Result<LandscapeOut> {
        let t = Instant::now();
        let world = world_of;
        let ufac = |fr: &Frame| {
            if key.uplift_noise {
                uplift_factor(
                    fr.w,
                    fr.h,
                    world(fr),
                    p.opt("uplift_noise_amp"),
                    p.opt("uplift_noise_wavelength_m"),
                    seed + 4,
                )
            } else {
                vec![1.0; fr.w * fr.h]
            }
        };
        let kmul = |fr: &Frame| {
            if key.rock {
                rock_map.field(fr.w, fr.h, world(fr), &rock_types)
            } else {
                (vec![1.0; fr.w * fr.h], vec![0; fr.w * fr.h])
            }
        };
        let shape =
            |l: &Layout, i: usize| floor + (1.0 - floor) * (l.relief.data[i] / l_ref).clamp(0.0, 1.0);
        let fields = |fr: &Frame, l: &Layout, c: &[f64], uf: &[f64], scale: f64| {
            let cb = ctx.blend(fr, |i| c[i]);
            let u: Vec<f64> = (0..cb.len())
                .map(|i| {
                    if l.land[i] {
                        cb[i] * shape(l, i) * uf[i] * scale
                    } else {
                        0.0
                    }
                })
                .collect();
            let hill = ctx.blend(fr, |i| ctx.resolved[i].hillslope_m);
            let d: Vec<f64> = u.iter().zip(&hill).map(|(u, h)| u * h).collect();
            (u, d)
        };
        let kfield = |fr: &Frame, km: &[f64], scale: f64| -> Vec<f64> {
            ctx.blend(fr, |i| ctx.resolved[i].erodibility)
                .iter()
                .zip(km)
                .map(|(e, m)| e * p.land_k_dt.v * m * scale)
                .collect()
        };
        let start = |fr: &Frame, l: &Layout| {
            let relief = ctx.blend(fr, |i| ctx.resolved[i].relief);
            let noise = perturb::field(
                fr.w,
                fr.h,
                world(fr),
                p.start_noise_m.v,
                p.start_noise_wavelength_m.v,
                seed,
            );
            let rough = if key.rough_start {
                perturb::field(
                    fr.w,
                    fr.h,
                    world(fr),
                    p.opt("rough_start_m"),
                    p.opt("rough_start_wavelength_m"),
                    seed + 5,
                )
            } else {
                Vec::new()
            };
            let data: Vec<f64> = (0..fr.w * fr.h)
                .map(|i| {
                    if l.land[i] {
                        let mut v = sea + p.start_fraction.v * relief[i] * l.relief.data[i] + noise[i];
                        if key.rough_start {
                            // |noise|: the rough start only adds height, so no land starts in the sea.
                            v += rough[i].abs();
                        }
                        v.max(sea)
                    } else {
                        l.sea_floor[i]
                    }
                })
                .collect();
            Grid {
                w: fr.w,
                h: fr.h,
                spacing: fr.spacing_m,
                data,
            }
        };
        let run_one = |g: &mut Grid,
                       k: &[f64],
                       u: &[f64],
                       d: &[f64],
                       talus: &[f64],
                       cap: usize,
                       stop: Stop|
         -> Result<LandscapeStats> {
            match key.solver {
                Solver::S4 => evolve(
                    g,
                    &LandscapeParams {
                        m: p.stream_m.v,
                        k_dt: k,
                        uplift_dt: u,
                        diffusion_dt: d,
                        tan_talus: talus,
                        flow_exponent: p.flow_exponent.v,
                        sea_level: sea,
                        max_iterations: cap,
                        steady_rel: p.steady_rel.v,
                    },
                ),
                Solver::Lem => lem::evolve(
                    g,
                    &LemParams {
                        m: p.stream_m.v,
                        n: p.opt("stream_n"),
                        k_dt: k,
                        uplift_dt: u,
                        diffusion_dt: d,
                        tan_talus: talus,
                        fixed: None,
                        mfd: MfdLaw {
                            base: p.opt("mfd_exp_base"),
                            slope_coef: p.opt("mfd_exp_slope"),
                        },
                        nonlinear_max_ratio: p.opt("nonlinear_max_ratio"),
                        sea_level: sea,
                        max_iterations: cap,
                        stop,
                    },
                ),
            }
        };
        let uf_l = ufac(&lframe);
        let (kmul_l, rock_kinds_l) = kmul(&lframe);
        let k_l = kfield(&lframe, &kmul_l, 1.0);
        let talus_l = ctx.blend(&lframe, |i| ctx.resolved[i].tan_talus);
        let steady = Stop::Steady {
            rel: p.steady_rel.v,
        };
        let mut c: Vec<f64> = target.iter().map(|t| t / p.rise_steps.v).collect();
        let mut hl = start(&lframe, &lay_l);
        let cap = ov
            .iterations
            .unwrap_or(p.steady_max_iterations.v as usize);
        let early_cap = ov
            .iterations
            .unwrap_or(p.calib_round_iterations.v as usize);
        let rounds_n = (p.calib_rounds.v as usize).max(1);
        let mut rounds = Vec::new();
        let mut measured = vec![(0usize, 0.0f64); 256];
        let label = key.label();
        for round in 0..rounds_n {
            let (u, d) = fields(&lframe, &lay_l, &c, &uf_l, 1.0);
            let st = run_one(
                &mut hl,
                &k_l,
                &u,
                &d,
                &talus_l,
                if round + 1 < rounds_n { early_cap } else { cap },
                steady,
            )?;
            let above: Vec<f64> = hl.data.iter().map(|v| v - sea).collect();
            measured = p95_by_region(&above, &dom_l, &lay_l.land);
            let ratio: Vec<f64> = (0..256)
                .map(|i| {
                    if measured[i].0 > 0 && measured[i].1 > 0.0 && target[i] > 0.0 {
                        target[i] / measured[i].1
                    } else {
                        1.0
                    }
                })
                .collect();
            let max_rel_error = (0..256)
                .filter(|&i| measured[i].0 > 0 && target[i] > 0.0)
                .map(|i| (measured[i].1 / target[i] - 1.0).abs())
                .fold(0.0, f64::max);
            println!(
                "  [{label}] landscape round {}: {} steps, converged {}, residual rel {:.4} (p99 {:.3} m/step, mean uplift {:.3} m/step, D dt/dx2 max {:.2}); max |p95/target - 1| {:.3}; lakes max {}",
                round + 1,
                st.iterations,
                st.converged,
                st.residual_rel,
                st.residual_p99_m,
                st.mean_uplift_dt_m,
                st.diffusion_ratio_max,
                max_rel_error,
                st.lakes_max
            );
            rounds.push(Round {
                stats: st,
                max_rel_error,
            });
            if round + 1 < rounds_n {
                for (ci, r) in c.iter_mut().zip(&ratio) {
                    *ci *= r;
                }
                let rb = ctx.blend(&lframe, |i| ratio[i]);
                for ((v, &l), &r) in hl.data.iter_mut().zip(&lay_l.land).zip(&rb) {
                    if l {
                        *v = sea + (*v - sea) * r;
                    }
                }
            }
        }
        // Bicubic upsample of a landscape-grid field to the macro grid (mask re-imposed).
        let upsample = |src: &Grid| -> Grid {
            let mut data = vec![0.0; frame.w * frame.h];
            data.par_chunks_mut(frame.w).enumerate().for_each(|(j, row)| {
                for (x, v) in row.iter_mut().enumerate() {
                    let i = j * frame.w + x;
                    *v = if lay.land[i] {
                        let [sx, sy] = lframe.sample(frame.world(x as f64, j as f64));
                        bicubic(src, sx, sy).max(sea)
                    } else {
                        lay.sea_floor[i]
                    };
                }
            });
            Grid {
                w: frame.w,
                h: frame.h,
                spacing,
                data,
            }
        };
        let same_grid = lspacing.to_bits() == spacing.to_bits();
        let macro_fields = |c: &[f64], scale: f64| {
            let uf = ufac(&frame);
            let (km, _) = kmul(&frame);
            let (u, d) = fields(&frame, &lay, c, &uf, scale);
            let k = kfield(&frame, &km, scale);
            let talus = ctx.blend(&frame, |i| ctx.resolved[i].tan_talus);
            (k, u, d, talus)
        };
        let mut refine = None;
        let mut finite = None;
        let mut final_regions = vec![(0usize, 0.0f64); 256];
        let mut boost_regions = vec![(0usize, 0.0f64); 256];
        let mut h1 = if key.finite {
            let f = p.opt("finite_fraction");
            let r = p.opt("finite_dt_ratio");
            // Steady state of the boosted forcing c / f, warm-started from the calibrated field × 1 / f.
            let c_boost: Vec<f64> = c.iter().map(|x| x / f).collect();
            let mut hb = hl.clone();
            for (v, &l) in hb.data.iter_mut().zip(&lay_l.land) {
                if l {
                    *v = sea + (*v - sea) / f;
                }
            }
            let (ub, db) = fields(&lframe, &lay_l, &c_boost, &uf_l, 1.0);
            let sb = run_one(&mut hb, &k_l, &ub, &db, &talus_l, cap, steady)?;
            let steady_p95 = sb.land_p95_m;
            let above: Vec<f64> = hb.data.iter().map(|v| v - sea).collect();
            boost_regions = p95_by_region(&above, &dom_l, &lay_l.land);
            // Finite run from the start at dt × r until the land p95 reaches f × the boosted steady p95.
            let mut hf = start(&lframe, &lay_l);
            let (ur, dr) = fields(&lframe, &lay_l, &c_boost, &uf_l, r);
            let kr = kfield(&lframe, &kmul_l, r);
            let target_m = f * steady_p95;
            let sf = run_one(
                &mut hf,
                &kr,
                &ur,
                &dr,
                &talus_l,
                p.opt("finite_max_steps") as usize,
                Stop::LandP95 { target_m },
            )?;
            println!(
                "  [{label}] boosted steady: {} steps, land p95 {:.1} m; finite run (dt x {r}): {} steps, stopped {}, land p95 {:.1} m = {:.3} of steady (target {f})",
                sb.iterations, steady_p95, sf.iterations, sf.converged, sf.land_p95_m,
                sf.land_p95_m / steady_p95
            );
            let above: Vec<f64> = hf.data.iter().map(|v| v - sea).collect();
            final_regions = p95_by_region(&above, &dom_l, &lay_l.land);
            let (g, rst) = if same_grid {
                (hf.clone(), None)
            } else {
                let mut g = upsample(&hf);
                let (k, u, d, talus) = macro_fields(&c_boost, r);
                let st = run_one(
                    &mut g,
                    &k,
                    &u,
                    &d,
                    &talus,
                    p.opt("finite_refine_steps") as usize,
                    Stop::Steps,
                )?;
                println!(
                    "  [{label}] finite refine @ {spacing} m: {} steps, land p95 {:.1} m",
                    st.iterations, st.land_p95_m
                );
                (g, Some(st))
            };
            let final_p95 = rst.as_ref().map_or(sf.land_p95_m, |s| s.land_p95_m);
            finite = Some(FiniteReport {
                fraction: f,
                dt_ratio: r,
                steady_steps: sb.iterations,
                steady_p95_m: steady_p95,
                target_p95_m: target_m,
                steps: sf.iterations,
                stopped: sf.converged,
                stop_p95_m: sf.land_p95_m,
                fraction_at_stop: sf.land_p95_m / steady_p95,
                refine_steps: rst.as_ref().map_or(0, |s| s.iterations),
                final_p95_m: final_p95,
                fraction_final: final_p95 / steady_p95,
                steady: sb,
                run: sf,
            });
            refine = rst;
            g
        } else if same_grid {
            hl.clone()
        } else {
            let mut g = upsample(&hl);
            let (k, u, d, talus) = macro_fields(&c, 1.0);
            let st = run_one(
                &mut g,
                &k,
                &u,
                &d,
                &talus,
                p.refine_max_iterations.v as usize,
                steady,
            )?;
            println!(
                "  [{label}] refine @ {spacing} m: {} steps, converged {}, residual rel {:.4} (p99 {:.3} m/step), D dt/dx2 max {:.2}",
                st.iterations, st.converged, st.residual_rel, st.residual_p99_m, st.diffusion_ratio_max
            );
            refine = Some(st);
            g
        };
        for (v, &l) in h1.data.iter_mut().zip(&lay.land) {
            if l {
                *v = v.max(sea);
            }
        }
        let calib: Vec<CalibRow> = (0..256)
            .filter(|&i| layout_p95[i].0 > 0)
            .map(|i| CalibRow {
                name: region_name(&reg, i),
                samples: layout_p95[i].0,
                layout_p95_m: layout_p95[i].1,
                target_p95_m: target[i],
                uplift_dt_m: c[i],
                measured_p95_m: measured[i].1,
                steady_boost_p95_m: boost_regions[i].1,
                finite_p95_m: final_regions[i].1,
            })
            .collect();
        let last = refine
            .clone()
            .unwrap_or_else(|| rounds.last().map(|r| r.stats.clone()).unwrap_or_default());
        let landscape_steps: usize = rounds.iter().map(|r| r.stats.iterations).sum::<usize>()
            + finite.as_ref().map_or(0, |f| f.steady_steps + f.steps);
        // Erodibility multiplier over land (landscape grid) and the share of each rock type.
        let land_km: Vec<f64> = kmul_l
            .iter()
            .zip(&lay_l.land)
            .filter(|(_, l)| **l)
            .map(|(k, _)| *k)
            .collect();
        let mut shares = vec![0usize; rock_types.len().max(1)];
        for (&t, &l) in rock_kinds_l.iter().zip(&lay_l.land) {
            if l {
                let last = shares.len() - 1;
                shares[usize::from(t).min(last)] += 1;
            }
        }
        let k_stats = if land_km.is_empty() {
            [0.0; 3]
        } else {
            [
                land_km.iter().copied().fold(f64::INFINITY, f64::min),
                land_km.iter().sum::<f64>() / land_km.len() as f64,
                land_km.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            ]
        };
        let ul: Vec<f64> = uf_l
            .iter()
            .zip(&lay_l.land)
            .filter(|(_, l)| **l)
            .map(|(u, _)| *u)
            .collect();
        let u_stats = if ul.is_empty() {
            [0.0; 2]
        } else {
            [
                ul.iter().copied().fold(f64::INFINITY, f64::min),
                ul.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            ]
        };
        Ok(LandscapeOut {
            key,
            h1,
            rounds,
            refine,
            calib,
            last,
            landscape_steps,
            seconds: t.elapsed().as_secs_f64(),
            finite,
            k_stats,
            rock_shares: shares,
            u_stats,
        })
    };
    let mut landscapes: Vec<LandscapeOut> = Vec::new();

    // 3. Arms.
    let vreg = recipe
        .volcano
        .as_ref()
        .and_then(|v| reg.find(&v.region));
    let threshold = p.channel_area_km2.v * 1e6;
    let mut reports: Vec<FieldReport> = Vec::new();
    let mut overviews: Vec<Gray> = Vec::new();
    let mut crop_imgs: Vec<Vec<Gray>> = vec![Vec::new(); crops.len()];
    let mut volcano_info = VolcanoInfo::default();
    let mut skipped = Vec::new();
    for arm in &recipe.arms {
        if ov.arms.as_ref().is_some_and(|a| !a.contains(&arm.name)) {
            continue;
        }
        let key = LandscapeKey::of(arm);
        let li = if let Some(i) = landscapes.iter().position(|o| o.key == key) {
            i
        } else {
            landscapes.push(compute(key)?);
            landscapes.len() - 1
        };
        let lo = &landscapes[li];
        let h1 = &lo.h1;
        let t = Instant::now();
        let mut g = h1.clone();
        let mut volcano_steps = 0;
        let mut note = String::new();
        if arm.volcano != VolcanoKind::None {
            let vi = vreg.ok_or_else(|| Error::Invalid("volcano region not in legend".into()))?;
            let (ci, _) = (0..g.len())
                .filter(|&i| lay.land[i] && dom[i] == vi)
                .map(|i| (i, lay.relief.data[i]))
                .fold((usize::MAX, f64::NEG_INFINITY), |b, x| {
                    if x.1 > b.1 { x } else { b }
                });
            if ci == usize::MAX {
                return Err(Error::Invalid("volcano region has no land sample".into()));
            }
            let centre = frame.world((ci % frame.w) as f64, (ci / frame.w) as f64);
            // Base level: the median landscape height on the ring where the blend ends (the surrounding
            // landscape), so the cone rises from its surroundings, not from the region's own uplifted relief.
            let (b0, b1) = (p.volcano_blend_start.v, p.volcano_blend_end.v);
            let ring_r = b1 * p.volcano_radius_m.v;
            let mut ring: Vec<f64> = (0..h1.len())
                .filter(|&i| lay.land[i])
                .filter(|&i| {
                    let pw = frame.world((i % frame.w) as f64, (i / frame.w) as f64);
                    ((pw[0] - centre[0]).hypot(pw[1] - centre[1]) - ring_r).abs() <= spacing
                })
                .map(|i| h1.data[i])
                .collect();
            ring.sort_by(f64::total_cmp);
            let base_c = if ring.is_empty() {
                sea
            } else {
                ring[(ring.len() - 1) / 2]
            };
            let base = Grid::filled(frame.w, frame.h, spacing, base_c);
            volcano_info.centre_m = centre;
            volcano_info.base_at_centre_m = base_c;
            match arm.volcano {
                VolcanoKind::Parametric => {
                    let strato = Strato {
                        height_m: p.volcano_summit_m.v - base_c,
                        radius_m: p.volcano_radius_m.v,
                        decay_m: p.volcano_decay_m.v,
                        crater_radius_m: p.volcano_crater_radius_m.v,
                        crater_depth_m: p.volcano_crater_depth_m.v,
                    };
                    volcano_info.strato = Some(strato);
                    volcano_info.radius_m = strato.radius_m;
                    let (amp, awl) = (p.volcano_asym_amp.v, p.volcano_asym_wavelength_m.v);
                    let mut wfield = vec![0.0; g.len()];
                    g.data
                        .par_chunks_mut(frame.w)
                        .zip(wfield.par_chunks_mut(frame.w))
                        .enumerate()
                        .for_each(|(j, (row, wrow))| {
                            for (x, (v, wv)) in row.iter_mut().zip(wrow.iter_mut()).enumerate() {
                                let i = j * frame.w + x;
                                if !lay.land[i] {
                                    continue;
                                }
                                let pw = frame.world(x as f64, j as f64);
                                let r = (pw[0] - centre[0]).hypot(pw[1] - centre[1])
                                    * (1.0 + amp * perturb::value_noise(pw, awl, seed + 1));
                                let w = blend_weight(r / strato.radius_m, b0, b1);
                                if w > 0.0 {
                                    let cone = base.data[i] + strato.profile(r);
                                    *v = (*v + w * (cone - *v)).max(sea);
                                }
                                *wv = w;
                            }
                        });
                    // Light erosion: gullies on a young cone.
                    let intent = g.data.clone();
                    let k: Vec<f64> = wfield.iter().map(|w| w * p.volcano_k_dt.v).collect();
                    let zeros = vec![0.0; g.len()];
                    let seedf: Vec<f64> = perturb::field(
                        frame.w,
                        frame.h,
                        |x, y| frame.world(x as f64, y as f64),
                        p.volcano_seed_amp_m.v,
                        p.volcano_seed_wavelength_m.v,
                        seed + 2,
                    )
                    .iter()
                    .zip(&wfield)
                    .map(|(n, w)| n * w)
                    .collect();
                    let st = erode(
                        &mut g,
                        &intent,
                        &StreamParams {
                            m: p.stream_m.v,
                            iterations: p.volcano_erosion_steps.v as usize,
                            k_dt: &k,
                            restore: &zeros,
                            sea_level: sea,
                            flow: Flow::Mfd {
                                exponent: p.volcano_flow_exponent.v,
                            },
                            perturb: Some(&seedf),
                            steady_dh_m: None,
                        },
                    )?;
                    volcano_steps = st.iterations;
                    note = format!(
                        "parametric: rim {:.0} m above base {:.0} m, R {:.0} m, r0 {:.0} m, crater {:.0} m / {:.0} m deep; erosion {} steps, incision mean {:.2} / p99 {:.1} / max {:.1} m",
                        strato.height_m,
                        base_c,
                        strato.radius_m,
                        strato.decay_m,
                        strato.crater_radius_m,
                        strato.crater_depth_m,
                        st.iterations,
                        st.mean_incision_m,
                        st.p99_incision_m,
                        st.max_incision_m
                    );
                }
                VolcanoKind::Exemplar => {
                    let dir = recipe
                        .volcano
                        .as_ref()
                        .and_then(|v| v.exemplar_dir.clone())
                        .unwrap_or_default();
                    if !dir.join("meta.json").exists() {
                        let msg = format!(
                            "{}: skipped, exemplar `{}` not found",
                            arm.name,
                            dir.display()
                        );
                        println!("  {msg}");
                        skipped.push(msg);
                        continue;
                    }
                    let eg = Guide::read(&dir)?;
                    note = stamp_exemplar(
                        &mut g,
                        &frame,
                        &lay,
                        &base,
                        &eg,
                        &dir,
                        centre,
                        p,
                        sea,
                        &mut volcano_info,
                    );
                }
                VolcanoKind::None => {}
            }
        }
        let volcano_s = t.elapsed().as_secs_f64();
        let t = Instant::now();
        let written = Grid {
            data: g.data.iter().map(|&v| f64::from(v as f32)).collect(),
            ..g.clone()
        };
        let channels = channel_metrics(&written, sea, threshold);
        let st = structure(
            &written,
            sea,
            threshold,
            p.coherence_window_m.v,
            p.coherence_dir_steps.v as usize,
        );
        let relief = relief_by_region(&g.data, &dom, sea);
        let metrics_s = t.elapsed().as_secs_f64();
        let mut rep = FieldReport {
            name: arm.name.clone(),
            help: arm.help.clone(),
            layout_s,
            landscape_s: lo.seconds,
            volcano_s,
            metrics_s,
            render_s: 0.0,
            landscape_steps: lo.landscape_steps,
            refine_steps: lo.refine.as_ref().map_or(0, |r| r.iterations),
            volcano_steps,
            residual_rel: lo.last.residual_rel,
            residual_p99_m: lo.last.residual_p99_m,
            converged: lo.last.converged,
            landscape: lo.key.label(),
            finite_fraction: lo.finite.as_ref().map(|f| f.fraction_final),
            max_height_m: g.data.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            wet_dry: WetDry::count(&g.data, sea),
            audit: Audit::compare(&layout_field, &g.data, sea),
            heights_blake3: hash_f32(&g.data),
            relief,
            channels,
            structure: st,
            volcano_note: note,
        };
        let dir = out_dir.join(&arm.name);
        std::fs::create_dir_all(&dir)?;
        write_heights(&dir, &g, &frame, sea, &serde_json::to_value(&rep)?)?;
        if !ov.no_images {
            let t = Instant::now();
            let full = hillshade(&g, sea, light);
            let factor = full.w.div_ceil(p.overview_max_px.v as usize);
            let ovw = downsample(&full, factor);
            write_png(&dir.join("overview.png"), &ovw)?;
            overviews.push(ovw);
            for (ci, crop) in crops.iter().enumerate() {
                let img = render_crop(
                    &g,
                    &frame,
                    crop,
                    p.crop_px.v as usize,
                    p.crop_size_m.v,
                    sea,
                    light,
                );
                write_png(&dir.join(format!("crop-{}.png", crop.name)), &img)?;
                crop_imgs[ci].push(img);
            }
            rep.render_s = t.elapsed().as_secs_f64();
        }
        println!(
            "  {}: volcano {:.1} s, metrics {:.1} s, render {:.1} s; max {:.1} m; land {}, sea {}; audit {:?}; coherence {:.3} / {:.3} / {:.3}; density {:.3} km/km2; junction {:.1} deg ({})",
            rep.name,
            rep.volcano_s,
            rep.metrics_s,
            rep.render_s,
            rep.max_height_m,
            rep.wet_dry.land,
            rep.wet_dry.sea,
            rep.audit,
            rep.structure.coherence_mean,
            rep.structure.coherence_median,
            rep.structure.coherence_p90,
            rep.structure.drainage_density,
            rep.structure.junction_mean_deg,
            rep.structure.junctions
        );
        reports.push(rep);
    }
    // Compares.
    let mut compares = Vec::new();
    for cmp in &recipe.compares {
        let cg = Guide::read(&cmp.dir)?;
        let g = Grid {
            w: cg.w,
            h: cg.h,
            spacing: cg.spacing_m,
            data: cg.heights.iter().map(|&v| f64::from(v)).collect(),
        };
        let cframe = Frame {
            scale: s,
            spacing_m: cg.spacing_m,
            origin_m: cg.origin_m,
            w: cg.w,
            h: cg.h,
        };
        let t = Instant::now();
        let cdom = ctx.dominant(&cframe);
        let rep = FieldReport {
            name: cmp.name.clone(),
            help: format!("`{}`", cmp.dir.display()),
            layout_s: 0.0,
            landscape_s: 0.0,
            volcano_s: 0.0,
            metrics_s: 0.0,
            render_s: 0.0,
            landscape_steps: 0,
            refine_steps: 0,
            volcano_steps: 0,
            residual_rel: 0.0,
            residual_p99_m: 0.0,
            converged: false,
            landscape: String::new(),
            finite_fraction: None,
            max_height_m: g.data.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            wet_dry: WetDry::count(&g.data, cg.sea_level_m),
            audit: Audit::default(),
            heights_blake3: hash_f32(&g.data),
            relief: relief_by_region(&g.data, &cdom, cg.sea_level_m),
            channels: channel_metrics(&g, cg.sea_level_m, threshold),
            structure: structure(
                &g,
                cg.sea_level_m,
                threshold,
                p.coherence_window_m.v,
                p.coherence_dir_steps.v as usize,
            ),
            volcano_note: String::new(),
        };
        println!(
            "  compare {}: {:.1} s; max {:.1} m; coherence {:.3} / {:.3} / {:.3}; density {:.3}; junction {:.1} deg",
            rep.name,
            t.elapsed().as_secs_f64(),
            rep.max_height_m,
            rep.structure.coherence_mean,
            rep.structure.coherence_median,
            rep.structure.coherence_p90,
            rep.structure.drainage_density,
            rep.structure.junction_mean_deg
        );
        compares.push(rep);
    }
    if !ov.no_images && !overviews.is_empty() {
        let refs: Vec<&Gray> = overviews.iter().collect();
        write_png(
            &out_dir.join("sheet-overview.png"),
            &contact_sheet(&refs, SHEET_GAP_PX, SHEET_GAP_GRAY),
        )?;
        for (ci, crop) in crops.iter().enumerate() {
            let refs: Vec<&Gray> = crop_imgs[ci].iter().collect();
            write_png(
                &out_dir.join(format!("sheet-{}.png", crop.name)),
                &contact_sheet(&refs, SHEET_GAP_PX, SHEET_GAP_GRAY),
            )?;
        }
    }
    let readme = Readme {
        recipe: &recipe,
        recipe_path,
        guide: &guide,
        reg: &reg,
        frame: &frame,
        lframe: &lframe,
        masked,
        sigma,
        l_ref,
        crops: &crops,
        landscapes: &landscapes,
        rock_names: &recipe.rocks.keys().cloned().collect::<Vec<_>>(),
        reports: &reports,
        compares: &compares,
        overview: overviews.first(),
        volcano: &volcano_info,
        skipped: &skipped,
    };
    std::fs::write(out_dir.join("README.md"), readme.render())?;
    Ok(())
}

fn region_name(reg: &RegionMap, i: usize) -> String {
    if reg.names[i].is_empty() {
        format!("(index {i}, no name)")
    } else {
        reg.names[i].clone()
    }
}

/// Stamps an exemplar heightfield at `centre`; returns the note for the README.
#[allow(clippy::too_many_arguments)]
fn stamp_exemplar(
    g: &mut Grid,
    frame: &Frame,
    lay: &Layout,
    base: &Grid,
    eg: &Guide,
    dir: &Path,
    centre: [f64; 2],
    p: &LandformParams,
    sea: f64,
    info: &mut VolcanoInfo,
) -> String {
    let e = Grid {
        w: eg.w,
        h: eg.h,
        spacing: eg.spacing_m,
        data: eg
            .heights
            .iter()
            .map(|&v| f64::from(v).max(eg.sea_level_m))
            .collect(),
    };
    let (top, e_max) = e
        .data
        .iter()
        .enumerate()
        .fold((0, f64::NEG_INFINITY), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
    let (tx, ty) = ((top % e.w) as f64, (top / e.w) as f64);
    let edge = tx
        .min(ty)
        .min((e.w - 1) as f64 - tx)
        .min((e.h - 1) as f64 - ty)
        * e.spacing;
    let radius = edge.min(p.volcano_radius_m.v);
    let rs = radius / e.spacing;
    let mut disc: Vec<f64> = (0..e.len())
        .filter(|&i| {
            let (x, y) = ((i % e.w) as f64 - tx, (i / e.w) as f64 - ty);
            x.hypot(y) <= rs
        })
        .map(|i| e.data[i])
        .collect();
    disc.sort_by(f64::total_cmp);
    let e_base = if disc.is_empty() {
        eg.sea_level_m
    } else {
        disc[((disc.len() - 1) as f64 * p.exemplar_base_quantile.v).round() as usize]
    };
    let ci = {
        let [sx, sy] = frame.sample(centre);
        (sy.round() as usize) * frame.w + sx.round() as usize
    };
    let height = p.volcano_summit_m.v - base.data[ci];
    let k = if e_max > e_base {
        height / (e_max - e_base)
    } else {
        0.0
    };
    info.radius_m = radius;
    let (b0, b1) = (p.volcano_blend_start.v, p.volcano_blend_end.v);
    g.data.par_chunks_mut(frame.w).enumerate().for_each(|(j, row)| {
        for (x, v) in row.iter_mut().enumerate() {
            let i = j * frame.w + x;
            if !lay.land[i] {
                continue;
            }
            let pw = frame.world(x as f64, j as f64);
            let (dx, dy) = (pw[0] - centre[0], pw[1] - centre[1]);
            let w = blend_weight(dx.hypot(dy) / radius, b0, b1);
            if w > 0.0 {
                let ev = bicubic(&e, tx + dx / e.spacing, ty - dy / e.spacing);
                let cone = base.data[i] + (ev - e_base).max(0.0) * k;
                *v = (*v + w * (cone - *v)).max(sea);
            }
        }
    });
    format!(
        "exemplar `{}` ({}x{} @ {} m): top {:.0} m, base quantile {} = {:.0} m, vertical factor {:.3}, blend radius {:.0} m",
        dir.display(),
        e.w,
        e.h,
        e.spacing,
        e_max,
        p.exemplar_base_quantile.v,
        e_base,
        k,
        radius
    )
}

struct Readme<'a> {
    recipe: &'a LandformRecipe,
    recipe_path: &'a Path,
    guide: &'a Guide,
    reg: &'a RegionMap,
    frame: &'a Frame,
    lframe: &'a Frame,
    masked: usize,
    sigma: f64,
    l_ref: f64,
    crops: &'a [CropAt],
    landscapes: &'a [LandscapeOut],
    rock_names: &'a [String],
    reports: &'a [FieldReport],
    compares: &'a [FieldReport],
    overview: Option<&'a Gray>,
    volcano: &'a VolcanoInfo,
    skipped: &'a [String],
}

impl Readme<'_> {
    #[allow(clippy::too_many_lines)]
    fn render(&self) -> String {
        let r = self.recipe;
        let p = &r.params;
        let (values, guesses) = r.census();
        let mut s = String::new();
        let _ = writeln!(s, "# scrib-worldgen run (landform pipeline, WG-S4/S5)\n");
        let _ = writeln!(
            s,
            "Recipe `{}` ({values} declared values, {guesses} `Guess`).",
            self.recipe_path.display()
        );
        let _ = writeln!(
            s,
            "Guide `{}`: {}x{} samples at {} m; {} samples masked to sea (`mask_to_sea` = {:?}).",
            r.guide.dir.display(),
            self.guide.w,
            self.guide.h,
            self.guide.spacing_m,
            self.masked,
            r.place.mask_to_sea
        );
        for (label, f) in [("Macro grid", self.frame), ("Landscape grid", self.lframe)] {
            let _ = writeln!(
                s,
                "{label}: {}x{} samples ({:.2} M) at {} m, S = {}, origin (NW sample) ({:.1}, {:.1}) m world, {:.1} x {:.1} km.",
                f.w,
                f.h,
                (f.w * f.h) as f64 * 1e-6,
                f.spacing_m,
                f.scale,
                f.origin_m[0],
                f.origin_m[1],
                (f.w - 1) as f64 * f.spacing_m / 1000.0,
                (f.h - 1) as f64 * f.spacing_m / 1000.0
            );
        }
        let _ = writeln!(
            s,
            "Layout: Gaussian low-pass, cutoff {} m (amplitude 1/2) = sigma {:.1} m world; L_ref (p99 of low-passed relief over land) {:.2} m.",
            p.layout_cutoff_m.v, self.sigma, self.l_ref
        );
        let _ = writeln!(
            s,
            "Landscape: m = {}, n = 1, K dt = {} (x erodibility), MFD p = {}, uplift floor {}, start fraction {} + noise {} m @ {} m, steady when p99 |dh| / mean uplift per step < {}, cap {} steps in the last of {} rounds ({} before); refine cap {}.",
            p.stream_m.v,
            p.land_k_dt.v,
            p.flow_exponent.v,
            p.uplift_floor.v,
            p.start_fraction.v,
            p.start_noise_m.v,
            p.start_noise_wavelength_m.v,
            p.steady_rel.v,
            p.steady_max_iterations.v,
            p.calib_rounds.v,
            p.calib_round_iterations.v,
            p.refine_max_iterations.v
        );
        let set: Vec<String> = p
            .optional()
            .iter()
            .filter_map(|(n, v)| v.map(|v| format!("{n} {} {}", v.v, v.unit)))
            .collect();
        if !set.is_empty() {
            let _ = writeln!(s, "WG-S5 values: {}.", set.join(", "));
        }
        if !r.rocks.is_empty() {
            let rk: Vec<String> = r
                .rocks
                .iter()
                .map(|(n, x)| format!("{n} (K x {}, share {})", x.k, x.share))
                .collect();
            let _ = writeln!(s, "Rock types: {}.", rk.join(", "));
        }
        let _ = writeln!(
            s,
            "Light: azimuth {} deg, altitude {} deg, true vertical scale; sea drawn at gray {}.\n",
            p.light_azimuth_deg.v, p.light_altitude_deg.v, p.sea_gray.v
        );
        let _ = writeln!(s, "## Arms (left to right on every sheet)\n");
        for (i, a) in self.reports.iter().enumerate() {
            let _ = writeln!(s, "{}. **{}**: {}", i + 1, a.name, a.help);
        }
        for k in self.skipped {
            let _ = writeln!(s, "\n**Skipped:** {k}.");
        }
        if let Some(o) = self.overview {
            let _ = writeln!(
                s,
                "\nSheets: `sheet-overview.png` (each panel {}x{} px), one `sheet-<crop>.png` per crop ({} px per panel), {} px gaps.",
                o.w, o.h, p.crop_px.v, SHEET_GAP_PX
            );
        }
        let _ = writeln!(
            s,
            "\n## Crops ({:.0} x {:.0} km, {} px)\n",
            p.crop_size_m.v / 1000.0,
            p.crop_size_m.v / 1000.0,
            p.crop_px.v
        );
        let label = r
            .report
            .as_ref()
            .map_or("region-grid pixel", |x| x.cell_label.as_str());
        let _ = writeln!(s, "| Crop | Centre, world m (x, y) | Centre, guide m | {label} | Rule |");
        let _ = writeln!(s, "|---|---|---|---|---|");
        for c in self.crops {
            let cell = r.report.as_ref().map_or(c.cell_px, |x| {
                [
                    x.cell_origin[0] + c.cell_px[0] * x.cell_step[0],
                    x.cell_origin[1] + c.cell_px[1] * x.cell_step[1],
                ]
            });
            let _ = writeln!(
                s,
                "| {} | ({:.0}, {:.0}) | ({:.1}, {:.1}) | ({}, {}) | {} |",
                c.name,
                c.centre_m[0],
                c.centre_m[1],
                c.centre_guide_m[0],
                c.centre_guide_m[1],
                cell[0],
                cell[1],
                c.rule
            );
        }
        let _ = writeln!(s, "\n## Numbers per arm\n");
        let _ = writeln!(
            s,
            "| Arm | Landscape | Wall s (layout / landscape / volcano / metrics / render) | Steps (landscape / refine / volcano) | Residual (rel / p99 m per step), converged | Finite fraction | Max height m | Land / sea | Audit vs layout (new wet, new dry, sea moved) | heights blake3 |"
        );
        let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|");
        for a in self.reports {
            let _ = writeln!(
                s,
                "| {} | {} | {:.1} / {:.1} / {:.1} / {:.1} / {:.1} | {} / {} / {} | {:.4} / {:.3}, {} | {} | {:.1} | {} / {} | {}, {}, {} | `{}` |",
                a.name,
                a.landscape,
                a.layout_s,
                a.landscape_s,
                a.volcano_s,
                a.metrics_s,
                a.render_s,
                a.landscape_steps,
                a.refine_steps,
                a.volcano_steps,
                a.residual_rel,
                a.residual_p99_m,
                a.converged,
                a.finite_fraction.map_or("-".into(), |f| format!("{f:.3}")),
                a.max_height_m,
                a.wet_dry.land,
                a.wet_dry.sea,
                a.audit.new_wet,
                a.audit.new_dry,
                a.audit.sea_moved,
                &a.heights_blake3[..16]
            );
        }
        let _ = writeln!(
            s,
            "\nThe layout is computed once; each distinct landscape (solver and fields) once, shared by the arms that name it (volcano arms start from their landscape's heights). Landscape steps count every run on the landscape grid (calibration rounds, and for finite time the boosted steady run and the finite run). Audit: class changes against the layout's mask, and sea samples whose height differs from the layout's low-passed sea floor."
        );
        for lo in self.landscapes {
            let _ = writeln!(
                s,
                "\n## Calibration: {} (landscape grid, {:.1} s)\n",
                lo.key.label(),
                lo.seconds
            );
            if lo.key.rock {
                let tot: usize = lo.rock_shares.iter().sum::<usize>().max(1);
                let sh: Vec<String> = self
                    .rock_names
                    .iter()
                    .zip(&lo.rock_shares)
                    .map(|(n, c)| format!("{n} {:.1} %", *c as f64 * 100.0 / tot as f64))
                    .collect();
                let _ = writeln!(
                    s,
                    "Rock map over land: {}; K multiplier (edge bands included) min / mean / max {:.3} / {:.3} / {:.3}.",
                    sh.join(", "),
                    lo.k_stats[0],
                    lo.k_stats[1],
                    lo.k_stats[2]
                );
            }
            if lo.key.uplift_noise {
                let _ = writeln!(
                    s,
                    "Uplift factor over land min / max {:.3} / {:.3}.",
                    lo.u_stats[0], lo.u_stats[1]
                );
            }
            let _ = writeln!(
                s,
                "\n| Round | Steps | Converged | Residual rel | Residual p99 m/step | Mean uplift m/step | Max D dt / dx² | Max abs(p95 / target - 1) | Lakes (max per step) | Flooded nodes (max) |"
            );
            let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|");
            let row = |s: &mut String, label: &str, st: &LandscapeStats, err: String| {
                let _ = writeln!(
                    s,
                    "| {label} | {} | {} | {:.4} | {:.3} | {:.3} | {:.2} | {err} | {} | {} |",
                    st.iterations,
                    st.converged,
                    st.residual_rel,
                    st.residual_p99_m,
                    st.mean_uplift_dt_m,
                    st.diffusion_ratio_max,
                    st.lakes_max,
                    st.lake_nodes_max
                );
            };
            for (i, rd) in lo.rounds.iter().enumerate() {
                row(&mut s, &(i + 1).to_string(), &rd.stats, format!("{:.3}", rd.max_rel_error));
            }
            if let Some(f) = &lo.finite {
                row(&mut s, "boosted steady", &f.steady, "-".into());
                row(&mut s, "finite run", &f.run, "-".into());
            }
            if let Some(st) = &lo.refine {
                row(
                    &mut s,
                    &format!("refine @ {} m", self.frame.spacing_m),
                    st,
                    "-".into(),
                );
            }
            for (i, rd) in lo.rounds.iter().enumerate() {
                let tr: Vec<String> = rd
                    .stats
                    .residual_trace
                    .iter()
                    .map(|(k, v)| format!("{k}: {v:.4}"))
                    .collect();
                let _ = writeln!(s, "\nResidual (rel) trace round {}: {}.", i + 1, tr.join(", "));
            }
            if let Some(f) = &lo.finite {
                let _ = writeln!(
                    s,
                    "\n**Finite time:** forcing = calibrated uplift / f (f = {}), so its steady state is ~1/f of the calibrated relief; the boosted steady state (warm start, {} steps) has land p95 {:.1} m. The finite run starts from the start surface at dt x {} (K dt, U dt, D dt all scaled, steady state unchanged) and stops when land p95 >= f x that = {:.1} m: {} steps, stopped {}, land p95 {:.1} m = **{:.3}** of steady at the stop; after {} macro-grid steps {:.1} m = **{:.3}**.",
                    f.fraction,
                    f.steady_steps,
                    f.steady_p95_m,
                    f.dt_ratio,
                    f.target_p95_m,
                    f.steps,
                    f.stopped,
                    f.stop_p95_m,
                    f.fraction_at_stop,
                    f.refine_steps,
                    f.final_p95_m,
                    f.fraction_final
                );
            }
            let fin = lo.finite.is_some();
            let _ = writeln!(
                s,
                "\n| Region | Samples (landscape grid) | Layout p95 m | Target p95 m | Uplift c_r m/step | Measured p95 m (last round) |{}",
                if fin { " Boosted steady p95 m | Finite p95 m (fraction) |" } else { "" }
            );
            let _ = writeln!(s, "|---|---|---|---|---|---|{}", if fin { "---|---|" } else { "" });
            for c in &lo.calib {
                let extra = if fin {
                    format!(
                        " {:.1} | {:.1} ({:.2}) |",
                        c.steady_boost_p95_m,
                        c.finite_p95_m,
                        if c.steady_boost_p95_m > 0.0 { c.finite_p95_m / c.steady_boost_p95_m } else { 0.0 }
                    )
                } else {
                    String::new()
                };
                let _ = writeln!(
                    s,
                    "| {} | {} | {:.1} | {:.1} | {:.3} | {:.1} |{extra}",
                    c.name, c.samples, c.layout_p95_m, c.target_p95_m, c.uplift_dt_m, c.measured_p95_m
                );
            }
        }
        let v = self.volcano;
        if v.radius_m > 0.0 {
            let _ = writeln!(
                s,
                "\n## Volcano\n\nCentre (highest low-passed layout sample of {:?}): ({:.0}, {:.0}) m world; base level (median landscape height on the blend-end ring) {:.1} m; summit target {} m.",
                r.volcano.as_ref().map_or("", |x| x.region.as_str()),
                v.centre_m[0],
                v.centre_m[1],
                v.base_at_centre_m,
                p.volcano_summit_m.v
            );
            for a in self.reports.iter().filter(|a| !a.volcano_note.is_empty()) {
                let _ = writeln!(s, "\n- {}: {}.", a.name, a.volcano_note);
            }
        }
        let all: Vec<&FieldReport> = self.reports.iter().chain(self.compares).collect();
        let _ = writeln!(s, "\n## Relief per region (metres, dry samples; min / median / p95 / max)\n");
        let _ = writeln!(
            s,
            "| Region | {} |",
            all.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(" | ")
        );
        let _ = writeln!(s, "|---|{}", "---|".repeat(all.len()));
        for i in 0..256 {
            if all.iter().all(|a| a.relief[i].samples == 0) {
                continue;
            }
            let cells: Vec<String> = all
                .iter()
                .map(|a| {
                    let x = a.relief[i];
                    if x.samples == 0 {
                        "-".into()
                    } else {
                        format!("{:.0} / {:.0} / {:.0} / {:.0}", x.min, x.median, x.p95, x.max)
                    }
                })
                .collect();
            let _ = writeln!(s, "| {} | {} |", region_name(self.reg, i), cells.join(" | "));
        }
        let _ = writeln!(s, "\n## Channel structure\n");
        let st0 = &all[0].structure;
        let _ = writeln!(
            s,
            "Channels: D8 area on the heights as written (f32) over the Priority-Flood+ε surface, area >= {:.3} km². Coherence: |mean of (cos 2θ, sin 2θ)| of channel directions ({} receivers downstream) in a {:.0} m square window around each channel sample (1 = all parallel). Junction angle: between the two largest tributaries, {} samples up each. Sinuosity and max direction share as WG-S2b.\n",
            st0.threshold_km2, st0.dir_steps, st0.window_m, st0.dir_steps
        );
        let _ = writeln!(
            s,
            "| Field | Channel samples | Length km | Land km² | Drainage density km/km² | Coherence mean / median / p90 | Junctions | Junction angle mean / median deg | Sinuosity | Max direction share % | Max height m | Land / sea |"
        );
        let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|---|---|");
        for a in &all {
            let t = &a.structure;
            let c = &a.channels;
            let _ = writeln!(
                s,
                "| {} | {} | {:.0} | {:.0} | {:.3} | {:.3} / {:.3} / {:.3} | {} | {:.1} / {:.1} | {:.3} | {:.1} | {:.1} | {} / {} |",
                a.name,
                t.channel_samples,
                t.length_km,
                t.land_km2,
                t.drainage_density,
                t.coherence_mean,
                t.coherence_median,
                t.coherence_p90,
                t.junctions,
                t.junction_mean_deg,
                t.junction_median_deg,
                c.mean_sinuosity,
                c.direction_share.iter().copied().fold(0.0, f64::max) * 100.0,
                a.max_height_m,
                a.wet_dry.land,
                a.wet_dry.sea
            );
        }
        for c in self.compares {
            let _ = writeln!(s, "\nCompare `{}`: {}.", c.name, c.help);
        }
        s
    }
}

/// World position (metres) of sample (x, y) of `fr`, as a per-sample closure.
fn world_of(fr: &Frame) -> impl Fn(usize, usize) -> [f64; 2] + Sync + '_ {
    move |x, y| fr.world(x as f64, y as f64)
}
