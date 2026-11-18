//! Recipe files (TOML) for the `scrib-worldgen` runner.
//!
//! Every scalar that trades quality, sets the look, or is a distance, count or threshold is a **parameter**:
//! `{ v, unit, min, max, source, help }`, with `source` in the tunables vocabulary (`Guess`, `Measured(..)`,
//! `Industry(..)`, `Reference(..)`, `Owner(..)`, `BevyDefault(..)`; spec `tunables.md` "Definition"). Per-region
//! values are plain numbers whose unit, range and source are declared once per column in `[region_params]`.
//! The runner validates every value against its range and counts the `Guess` sources.
//!
//! Sections: `[guide]` (folder, optional `origin_m`), `[output]` (folder), `[params]` (scalars, names fixed by
//! [`Params`]), `[place]` (`mask_to_sea`: region-name prefixes whose samples become sea), `[region_params]`
//! (column metadata), `[regions.default]` + `[regions."<legend name>"]` (per-region values; a missing value falls
//! back to `default`), `[[crop]]` (preview windows), `[[arm]]` (variants), `[report]` (optional: how to label
//! region-grid cells in the source's own coordinates).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{Error, Result};

/// One declared parameter value.
#[derive(Debug, Clone, Deserialize)]
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

/// Unit, range and source of one per-region column.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Column {
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub source: String,
    #[serde(default)]
    pub help: String,
}

/// The scalar parameters.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Params {
    /// Horizontal scale factor S.
    pub scale: Param,
    /// Macro grid spacing, metres (after scaling).
    pub macro_spacing_m: Param,
    /// Sea kept around the land bounding box, metres (after scaling).
    pub sea_margin_m: Param,
    /// Stream power drainage-area exponent m.
    pub stream_m: Param,
    /// `K dt` per step for erodibility 1.
    pub stream_k_dt: Param,
    /// Fraction of the gap to the intent restored per step for uplift scale 1 (`dt / τ`).
    pub stream_restore: Param,
    pub stream_iterations: Param,
    pub thermal_iterations: Param,
    /// Fraction of the talus excess moved per thermal step.
    pub thermal_rate: Param,
    pub light_azimuth_deg: Param,
    pub light_altitude_deg: Param,
    pub sea_gray: Param,
    /// Overview width cap, px.
    pub overview_max_px: Param,
    /// Crop edge, metres (after scaling).
    pub crop_size_m: Param,
    /// Crop edge, px.
    pub crop_px: Param,
}

/// Per-region values (all optional except in `default`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionValues {
    /// Stream power erodibility multiplier.
    pub erodibility: Option<f64>,
    /// Uplift (restore rate) multiplier.
    pub uplift: Option<f64>,
    /// Vertical factor applied to intent heights.
    pub relief: Option<f64>,
    /// Talus angle, degrees.
    pub talus_deg: Option<f64>,
    /// Profile boost: the region's highest intent sample is lifted to this height (metres after scaling).
    pub peak_m: Option<f64>,
    /// Profile boost starts above this intent height (metres before scaling).
    pub boost_base_m: Option<f64>,
    /// Profile boost exponent (> 1: concave profile, steeper toward the top).
    pub boost_power: Option<f64>,
}

/// Fully resolved values of one region.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resolved {
    pub erodibility: f64,
    pub uplift: f64,
    pub relief: f64,
    pub talus_deg: f64,
    /// (peak after scaling, base before scaling, power), when the region has a profile boost.
    pub boost: Option<(f64, f64, f64)>,
}

/// A preview window.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Crop {
    pub name: String,
    /// Centre on the highest guide sample of this legend name.
    pub peak_of: Option<String>,
    /// Centre on the crop-sized window of the region grid holding the most pixels of this legend name.
    pub region: Option<String>,
}

/// One variant of the run.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arm {
    pub name: String,
    #[serde(default)]
    pub help: String,
    /// Per-region values (else `default` everywhere).
    pub regional: bool,
    pub erosion: bool,
    pub thermal: bool,
}

/// Labels for region-grid cells in the source's own coordinates: pixel (px, py) is labelled
/// `cell_origin + (px × cell_step.0, py × cell_step.1)`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub cell_label: String,
    pub cell_origin: [i64; 2],
    pub cell_step: [i64; 2],
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideSection {
    pub dir: PathBuf,
    pub origin_m: Option<[f64; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputSection {
    pub dir: PathBuf,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceSection {
    #[serde(default)]
    pub mask_to_sea: Vec<String>,
}

/// A recipe file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    pub guide: GuideSection,
    pub output: OutputSection,
    pub params: Params,
    #[serde(default)]
    pub place: PlaceSection,
    pub region_params: BTreeMap<String, Column>,
    pub regions: BTreeMap<String, RegionValues>,
    #[serde(rename = "crop", default)]
    pub crops: Vec<Crop>,
    #[serde(rename = "arm")]
    pub arms: Vec<Arm>,
    pub report: Option<Report>,
}

const SOURCES: [&str; 6] = [
    "Guess",
    "Measured(",
    "Industry(",
    "Reference(",
    "Owner(",
    "BevyDefault(",
];

fn check_source(name: &str, s: &str) -> Result<()> {
    if s == "Guess"
        || SOURCES[1..]
            .iter()
            .any(|p| s.starts_with(p) && s.ends_with(')'))
    {
        Ok(())
    } else {
        Err(Error::Invalid(format!(
            "{name}: source {s:?} is not one of Guess, Measured(..), Industry(..), ..."
        )))
    }
}

fn check_range(name: &str, v: f64, min: f64, max: f64) -> Result<()> {
    if v.is_finite() && v >= min && v <= max {
        Ok(())
    } else {
        Err(Error::Invalid(format!(
            "{name} = {v} outside [{min}, {max}]"
        )))
    }
}

impl Params {
    fn all(&self) -> [(&'static str, &Param); 15] {
        [
            ("scale", &self.scale),
            ("macro_spacing_m", &self.macro_spacing_m),
            ("sea_margin_m", &self.sea_margin_m),
            ("stream_m", &self.stream_m),
            ("stream_k_dt", &self.stream_k_dt),
            ("stream_restore", &self.stream_restore),
            ("stream_iterations", &self.stream_iterations),
            ("thermal_iterations", &self.thermal_iterations),
            ("thermal_rate", &self.thermal_rate),
            ("light_azimuth_deg", &self.light_azimuth_deg),
            ("light_altitude_deg", &self.light_altitude_deg),
            ("sea_gray", &self.sea_gray),
            ("overview_max_px", &self.overview_max_px),
            ("crop_size_m", &self.crop_size_m),
            ("crop_px", &self.crop_px),
        ]
    }
}

/// Counts of declared values: (parameters + region values, of which `Guess`).
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Census {
    pub values: usize,
    pub guesses: usize,
}

impl Recipe {
    /// Reads and validates a recipe.
    pub fn read(path: &Path) -> Result<Self> {
        let r: Self = toml::from_str(&std::fs::read_to_string(path)?)?;
        r.validate()?;
        Ok(r)
    }

    /// Checks ranges, sources and the region table.
    pub fn validate(&self) -> Result<()> {
        for (name, p) in self.params.all() {
            check_source(name, &p.source)?;
            check_range(name, p.v, p.min, p.max)?;
        }
        if self.params.stream_m.v.to_bits() == 0.0f64.to_bits() {
            return Err(Error::Invalid("stream_m must be > 0".into()));
        }
        for (name, c) in &self.region_params {
            check_source(name, &c.source)?;
        }
        let default = self
            .regions
            .get("default")
            .ok_or_else(|| Error::Invalid("[regions.default] missing".into()))?;
        if default.erodibility.is_none()
            || default.uplift.is_none()
            || default.relief.is_none()
            || default.talus_deg.is_none()
        {
            return Err(Error::Invalid(
                "[regions.default] needs erodibility, uplift, relief, talus_deg".into(),
            ));
        }
        for (region, v) in &self.regions {
            for (col, val) in [
                ("erodibility", v.erodibility),
                ("uplift", v.uplift),
                ("relief", v.relief),
                ("talus_deg", v.talus_deg),
                ("peak_m", v.peak_m),
                ("boost_base_m", v.boost_base_m),
                ("boost_power", v.boost_power),
            ] {
                if let Some(x) = val {
                    let c = self
                        .region_params
                        .get(col)
                        .ok_or_else(|| Error::Invalid(format!("[region_params.{col}] missing")))?;
                    check_range(&format!("regions.{region:?}.{col}"), x, c.min, c.max)?;
                }
            }
            let boost = [
                v.peak_m.is_some(),
                v.boost_base_m.is_some(),
                v.boost_power.is_some(),
            ];
            if boost.iter().any(|b| *b) && !boost.iter().all(|b| *b) {
                return Err(Error::Invalid(format!(
                    "regions.{region:?}: peak_m, boost_base_m, boost_power go together"
                )));
            }
        }
        if self.arms.is_empty() {
            return Err(Error::Invalid("no [[arm]]".into()));
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
    pub fn resolve(&self, name: &str) -> Resolved {
        let d = &self.regions["default"];
        let r = self.regions.get(name).unwrap_or(d);
        let boost = match (r.peak_m, r.boost_base_m, r.boost_power) {
            (Some(p), Some(b), Some(e)) => Some((p, b, e)),
            _ => None,
        };
        Resolved {
            erodibility: r.erodibility.or(d.erodibility).unwrap_or(1.0),
            uplift: r.uplift.or(d.uplift).unwrap_or(1.0),
            relief: r.relief.or(d.relief).unwrap_or(1.0),
            talus_deg: r.talus_deg.or(d.talus_deg).unwrap_or(0.0),
            boost,
        }
    }

    /// Values and guesses declared in this recipe.
    pub fn census(&self) -> Census {
        let mut c = Census::default();
        for (_, p) in self.params.all() {
            c.values += 1;
            c.guesses += usize::from(p.source == "Guess");
        }
        for v in self.regions.values() {
            for (col, val) in [
                ("erodibility", v.erodibility),
                ("uplift", v.uplift),
                ("relief", v.relief),
                ("talus_deg", v.talus_deg),
                ("peak_m", v.peak_m),
                ("boost_base_m", v.boost_base_m),
                ("boost_power", v.boost_power),
            ] {
                if val.is_some() {
                    c.values += 1;
                    c.guesses += usize::from(
                        self.region_params
                            .get(col)
                            .is_some_and(|k| k.source == "Guess"),
                    );
                }
            }
        }
        c
    }
}
