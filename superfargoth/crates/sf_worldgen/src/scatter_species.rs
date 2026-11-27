//! Scatter v2 species file (spec `worldgen-scatter.md` "Adapter data" and "Tunables"; slice WG-E2a).
//!
//! A TOML file with six parts:
//!
//! - `[params]`: the scatter-wide v2 values (stand mosaic, edge band, competition, light, caps), each a
//!   `{ v, unit, min, max, source }` entry of a name declared in [`declared`]; a name left out keeps its declared
//!   default, an unknown name is refused.
//! - `[columns]`: unit, range and source of every species column the rows use (the climate file's
//!   `[override_columns]` pattern). A column's unit is fixed ([`SPECIES_COLUMNS`]); the range of a numeric column
//!   must lie inside its declared one ([`species_declared`]); a hat column's range bounds all four hat values.
//! - `[classes."<class name>"]`: canopy stems per hectare of one biome class (`stems_per_ha`, a full entry); a
//!   class left out keeps its default ([`class_defaults`]).
//! - `[[species]]`: one row per species: `name`, `layer` (`canopy`, `understorey`, `ground`), optional `model`,
//!   `size_law` (`reverse_j`, `weibull`), optional `classes`, numeric column values and hats
//!   (`hat_<driver> = [e_min, i_min, i_max, e_max]`). A value is allowed only in a declared column and must lie in
//!   its range; a numeric value left out takes the column's declared default, which must then lie in the file's
//!   range when the column is declared. A hat left out is open (no constraint on that driver).
//!
//! - `[rocks.<context>]` (`talus`, `outcrop`, `field`; slice WG-E3a): the row of one rock context: `size_min_m`
//!   and `size_max_m` (longest axis, required, checked in the species columns of those names), optional
//!   `sink_small`, `sink_large`, `align` (resolved as species values are).
//! - `[rock_units."<unit>"]`: one rock unit (a geology unit name of the adapter): optional `field_stone_per_ha`,
//!   `rockfall`, and per context (`talus`, `outcrop`, `field`) a list of weighted entries,
//!   `{ kind = "class", class, weight }` (a library class by name) or
//!   `{ kind = "model", model, axis_m, scale_min, scale_max, weight }` (a pack model with its longest axis and
//!   allowed scale range). Weights are finite and positive; an entry is not repeated within its context.
//!
//! [`SpeciesFile::to_toml`] writes every value explicitly, so a written file parses back to the same
//! [`SpeciesFile`]. Nothing here names a game: species, models and classes are the adapter's data.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::filter::Declared;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_scatter_species.rs"]
mod tests;

/// The columns a file may declare, with their fixed units (species numeric columns, rock row and rock unit
/// columns, then the six hats).
// tunables: const (format: the column names and units of the species rows)
pub const SPECIES_COLUMNS: &[(&str, &str)] = &[
    ("parent_per_ha", "1/ha"),
    ("cluster_sigma_m", "m"),
    ("offspring_mean", "count"),
    ("background_fraction", "ratio"),
    ("size_shape", "ratio"),
    ("size_min_m", "m"),
    ("size_max_m", "m"),
    ("crown_a_m", "m"),
    ("crown_b", "ratio"),
    ("crown_k", "ratio"),
    ("shade_tolerance", "ratio"),
    ("footprint_ratio", "ratio"),
    ("align", "ratio"),
    ("tilt_jitter_deg", "deg"),
    ("offset_m", "m"),
    ("sink_small", "ratio"),
    ("sink_large", "ratio"),
    ("field_stone_per_ha", "1/ha"),
    ("rockfall", "ratio"),
    ("hat_temperature_c", "C"),
    ("hat_precipitation_mm", "mm/yr"),
    ("hat_moisture", "ratio"),
    ("hat_slope_deg", "deg"),
    ("hat_altitude_m", "m"),
    ("hat_water_distance_m", "m"),
];

/// The scatter-wide v2 values (spec "Tunables"; `tunables.md` "Definition": value, unit, range, source), then the
/// rock layer's `scatter.rock.*` values ([`crate::scatter_rocks::declared`]).
#[must_use]
pub fn declared() -> Vec<Declared> {
    let mut v = vegetation_declared();
    v.extend(crate::scatter_rocks::declared());
    v
}

/// The vegetation part of [`declared`].
fn vegetation_declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("scatter.stand_wavelength_m", 250.0, "m", 50.0, 1000.0, "Guess",
          "lattice spacing of the stand mosaic's value noise D(x)"),
        d("scatter.stand_clearing_threshold", 0.35, "ratio", 0.0, 1.0, "Guess", "D below this is a clearing"),
        d("scatter.stand_sparse_threshold", 0.5, "ratio", 0.0, 1.0, "Guess",
          "D below this (and above the clearing threshold) is a sparse stand"),
        d("scatter.stand_inner_threshold", 0.7, "ratio", 0.0, 1.0, "Guess",
          "D from this up is an inner stand; between sparse and inner is the edge"),
        d("scatter.edge_band_m", 25.0, "m", 5.0, 60.0, "Guess", "width of the regrowth fringe round every clearing"),
        d("scatter.edge_size_ratio", 0.6, "ratio", 0.2, 1.0, "Guess",
          "size quantile scale in the edge band (big trees in the core)"),
        d("scatter.edge_boost", 1.0, "ratio", 0.0, 4.0, "Guess",
          "lower layers' intensity x (1 + this) in the edge band"),
        d("scatter.field_step_m", 4.0, "m", 1.0, 16.0, "Guess",
          "grid step of the clearing distance field over a tile and its halo"),
        d("scatter.competition_rounds", 3.0, "count", 1.0, 6.0, "Guess",
          "fixed rounds of the competition; round 1 is Matern II"),
        d("scatter.shade_per_crown", 1.0, "ratio", 0.2, 3.0, "Guess",
          "optical depth a canopy crown adds over the ground below it (light = exp(-sum))"),
        d("scatter.canopy_stems_per_ha_max", 400.0, "1/ha", 50.0, 2000.0,
          "Reference(big-world-rendering.md Cost model, the stand case)",
          "cap on canopy stems per hectare of a tile, kept in rank order"),
        d("scatter.region_prior_max", 3.0, "ratio", 1.0, 10.0, "Guess",
          "upper bound of a learned region prior on parent intensity"),
        d("scatter.census_step_m", 16.0, "m", 4.0, 128.0, "Guess",
          "grid step of the class census (class land area and canopy weight mean)"),
        d("scatter.calib_area_ha", 200.0, "ha", 10.0, 2000.0, "Guess",
          "class land of the canopy calibration's pilot sample, tiles in one hashed world order"),
        d("scatter.calib_iters", 4.0, "count", 1.0, 8.0, "Guess",
          "most fixed-point steps of the canopy calibration per class"),
        d("scatter.calib_tol", 0.05, "ratio", 0.01, 0.25, "Guess",
          "calibration stops when winners per ha are this close to the class target"),
        d("scatter.calib_saturation", 0.25, "ratio", 0.0, 1.0, "Guess",
          "a raise of the competition scale by f that raises winners by less than this x f is the crown ceiling"),
        d("scatter.calib_z", 2.0, "ratio", 1.0, 3.0, "Reference(Cochran 1977 Sampling Techniques, t = 2)",
          "standard errors of the calibration's winners per ha that the sample size keeps within calib_tol"),
        d("scatter.calib_tiles_min", 30.0, "count", 10.0, 100.0,
          "Reference(Cochran 1977 Sampling Techniques, large-sample ratio variance)",
          "fewest parent tiles holding the class in the calibration's pilot sample"),
        d("scatter.calib_tiles_max", 400.0, "count", 50.0, 4000.0, "Guess",
          "most parent tiles of a class's sized calibration sample (cost bound)"),
        d("scatter.intensity_scale_max", 16.0, "ratio", 1.0, 40.0, "Guess",
          "bound of the canopy class multiplier g / m"),
    ]
}

/// The declared defaults of the numeric species columns (hats have none: a hat left out is open).
#[must_use]
pub fn species_declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("parent_per_ha", 4.0, "1/ha", 0.0, 500.0, "Guess", "cluster parents per hectare at full suitability"),
        d("cluster_sigma_m", 12.0, "m", 2.0, 40.0, "Guess", "Gaussian spread of a cluster's offspring (Thomas)"),
        d("offspring_mean", 12.0, "count", 1.0, 60.0, "Guess", "mean offspring per parent at full suitability"),
        d("background_fraction", 0.2, "ratio", 0.0, 0.5, "Guess", "share of candidates from a uniform background"),
        d("size_shape", 1.2, "ratio", 0.5, 6.0, "Guess", "shape of the size law (reverse-J rate or Weibull k)"),
        d("size_min_m", 1.0, "m", 0.2, 40.0, "Guess", "smallest size mark (height)"),
        d("size_max_m", 10.0, "m", 0.2, 40.0, "Guess", "largest size mark (height)"),
        d("crown_a_m", 0.75, "m", 0.0, 5.0, "Reference(Brack and Wood crown width form)",
          "crown radius at zero height: r = a + b h"),
        d("crown_b", 0.2, "ratio", 0.0, 0.5, "Reference(Brack and Wood crown width form)",
          "crown radius per metre of height: r = a + b h"),
        d("crown_k", 0.8, "ratio", 0.3, 1.2, "Guess", "conflict distance R_ij = k (r_i + r_j)"),
        d("shade_tolerance", 0.3, "ratio", 0.0, 1.0, "Industry(Makowski 2019 stol form)",
          "intensity kept in full shade: x lerp(stol, 1, light)"),
        d("footprint_ratio", 0.01, "ratio", 0.0, 0.5, "Guess",
          "stem footprint radius per metre of size mark: a hard obstacle for later layers"),
        d("align", 0.0, "ratio", 0.0, 1.0, "Guess", "up axis blended toward the ground normal (0 upright)"),
        d("tilt_jitter_deg", 0.0, "deg", 0.0, 30.0, "Guess", "random tilt of the up axis"),
        d("offset_m", 0.0, "m", -5.0, 5.0, "Guess", "height above the ground per unit scale"),
    ]
}

/// The declared defaults of the rock columns: sinks of a rock row, then a rock unit's values (spec "Tunables").
/// A rock row's `size_min_m` and `size_max_m` use the species columns of those names and are required.
#[must_use]
pub fn rock_declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("sink_small", 0.35, "ratio", 0.0, 0.6, "Guess",
          "share of the short axis sunk into the ground for the smallest rock of the row"),
        d("sink_large", 0.15, "ratio", 0.0, 0.6, "Guess",
          "share of the short axis sunk into the ground for the largest rock of the row"),
        d("field_stone_per_ha", 20.0, "1/ha", 0.0, 500.0, "Guess", "field stones per hectare of a rock unit"),
        d("rockfall", 1.0, "ratio", 0.0, 4.0, "Guess", "talus parent intensity factor of a rock unit"),
    ]
}

/// Default canopy stems per hectare per biome class (spec "Counts per biome class"; class names of the shipped
/// class table `climate_classes.toml`).
#[must_use]
pub fn class_defaults() -> Vec<(&'static str, f64)> {
    vec![
        ("desert", 0.0),
        ("cold desert", 0.0),
        ("tundra", 5.0),
        ("grassland", 5.0),
        ("shrubland", 10.0),
        ("savanna", 20.0),
        ("woodland", 60.0),
        ("taiga", 250.0),
        ("temperate forest", 150.0),
        ("seasonal forest", 150.0),
        ("temperate rainforest", 250.0),
        ("rainforest", 300.0),
    ]
}

/// Declared bounds of a class's stems per hectare.
fn class_declared(v: f64) -> Entry {
    // tunables: const (the declaration of `class.<name>.stems_per_ha`: unit 1/ha, range 0-2000, spec "Tunables")
    Entry { v, unit: "1/ha".into(), min: 0.0, max: 2000.0, source: "Guess".into(), help: String::new() }
}

/// One `{ v, unit, min, max, source }` entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub v: f64,
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub source: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub help: String,
}

/// Unit, range and source of one species column.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnDecl {
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub source: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub help: String,
}

/// Which layer of the passes a species belongs to (spec "Design": canopy, then understorey, then ground).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layer {
    Canopy,
    Understorey,
    Ground,
}

/// The size mark's law (spec "Design" 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeLaw {
    /// Truncated exponential in height (de Liocourt's reverse J).
    ReverseJ,
    /// Weibull of shape `size_shape`.
    Weibull,
}

/// A suitability hat `[e_min, i_min, i_max, e_max]`: 0 outside, 1 inside, linear between (spec "Design" 2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hat {
    pub e_min: f64,
    pub i_min: f64,
    pub i_max: f64,
    pub e_max: f64,
}

impl Hat {
    fn from_array(a: [f64; 4]) -> Self {
        Self { e_min: a[0], i_min: a[1], i_max: a[2], e_max: a[3] }
    }

    fn to_array(self) -> [f64; 4] {
        [self.e_min, self.i_min, self.i_max, self.e_max]
    }
}

/// The six hats of a species (`None`: open, the driver doesn't constrain it).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hats {
    pub temperature_c: Option<Hat>,
    pub precipitation_mm: Option<Hat>,
    pub moisture: Option<Hat>,
    pub slope_deg: Option<Hat>,
    pub altitude_m: Option<Hat>,
    pub water_distance_m: Option<Hat>,
}

/// One species row with every numeric value resolved (given or declared default).
#[derive(Debug, Clone, PartialEq)]
pub struct Species {
    pub name: String,
    pub layer: Layer,
    pub model: Option<String>,
    pub size_law: SizeLaw,
    /// Biome classes the species lives in; empty = any.
    pub classes: Vec<String>,
    pub parent_per_ha: f64,
    pub cluster_sigma_m: f64,
    pub offspring_mean: f64,
    pub background_fraction: f64,
    pub size_shape: f64,
    pub size_min_m: f64,
    pub size_max_m: f64,
    pub crown_a_m: f64,
    pub crown_b: f64,
    pub crown_k: f64,
    pub shade_tolerance: f64,
    /// Stem footprint radius per metre of size mark (a hard obstacle for later layers).
    pub footprint_ratio: f64,
    pub align: f64,
    pub tilt_jitter_deg: f64,
    pub offset_m: f64,
    pub hats: Hats,
}

/// The scatter-wide v2 values (see [`declared`] for units and meaning).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScatterValues {
    pub stand_wavelength_m: f64,
    pub stand_clearing_threshold: f64,
    pub stand_sparse_threshold: f64,
    pub stand_inner_threshold: f64,
    pub edge_band_m: f64,
    pub edge_size_ratio: f64,
    pub edge_boost: f64,
    pub field_step_m: f64,
    pub competition_rounds: u32,
    pub shade_per_crown: f64,
    pub canopy_stems_per_ha_max: f64,
    pub region_prior_max: f64,
    pub census_step_m: f64,
    pub calib_area_ha: f64,
    pub calib_iters: u32,
    pub calib_tol: f64,
    pub calib_saturation: f64,
    pub calib_z: f64,
    pub calib_tiles_min: u32,
    pub calib_tiles_max: u32,
    pub intensity_scale_max: f64,
}

impl ScatterValues {
    /// Builds the values from declared entries (ranges, the thresholds' order and the integer round count checked).
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("scatter species: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        let (c, s, i) = (
            get("scatter.stand_clearing_threshold")?,
            get("scatter.stand_sparse_threshold")?,
            get("scatter.stand_inner_threshold")?,
        );
        if !(c < s && s < i) {
            return Err(Error::Invalid(format!(
                "scatter species: stand thresholds clearing {c} < sparse {s} < inner {i} do not hold"
            )));
        }
        let whole = |name: &str| -> Result<f64> {
            let n = get(name)?;
            if n.fract() != 0.0 {
                return Err(Error::Invalid(format!("scatter species: {name} = {n} is not a whole count")));
            }
            Ok(n)
        };
        let rounds = whole("scatter.competition_rounds")?;
        let iters = whole("scatter.calib_iters")?;
        let (tiles_min, tiles_max) = (whole("scatter.calib_tiles_min")?, whole("scatter.calib_tiles_max")?);
        if tiles_min > tiles_max {
            return Err(Error::Invalid(format!(
                "scatter species: calib_tiles_min {tiles_min} above calib_tiles_max {tiles_max}"
            )));
        }
        Ok(Self {
            stand_wavelength_m: get("scatter.stand_wavelength_m")?,
            stand_clearing_threshold: c,
            stand_sparse_threshold: s,
            stand_inner_threshold: i,
            edge_band_m: get("scatter.edge_band_m")?,
            edge_size_ratio: get("scatter.edge_size_ratio")?,
            edge_boost: get("scatter.edge_boost")?,
            field_step_m: get("scatter.field_step_m")?,
            // Whole and inside 1..=6 (checked above).
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            competition_rounds: rounds as u32,
            shade_per_crown: get("scatter.shade_per_crown")?,
            canopy_stems_per_ha_max: get("scatter.canopy_stems_per_ha_max")?,
            region_prior_max: get("scatter.region_prior_max")?,
            census_step_m: get("scatter.census_step_m")?,
            calib_area_ha: get("scatter.calib_area_ha")?,
            // Whole and inside 1..=8 (checked above).
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            calib_iters: iters as u32,
            calib_tol: get("scatter.calib_tol")?,
            calib_saturation: get("scatter.calib_saturation")?,
            calib_z: get("scatter.calib_z")?,
            // Whole and inside 10..=100 / 50..=4000 (checked above).
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            calib_tiles_min: tiles_min as u32,
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            calib_tiles_max: tiles_max as u32,
            intensity_scale_max: get("scatter.intensity_scale_max")?,
        })
    }
}

impl Default for ScatterValues {
    fn default() -> Self {
        Self::from_declared(&declared()).expect("defaults are valid")
    }
}

/// A rock context: one candidate stream of the rock layer (spec "Rocks (WG-E3)", three contexts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RockContext {
    /// Blocks fallen from a scarp.
    Talus,
    /// A parent block and its debris.
    Outcrop,
    /// Small clumped stones on open ground.
    Field,
}

/// The row of one rock context: its size range (longest axis), sink and up blend, every value resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockRow {
    pub size_min_m: f64,
    pub size_max_m: f64,
    pub sink_small: f64,
    pub sink_large: f64,
    pub align: f64,
}

/// One weighted mesh entry of a rock unit's context (spec "Mesh pick").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RockEntry {
    /// A generated library class, by name (it fits any size).
    Class { class: String, weight: f64 },
    /// A pack's own model: eligible only when the size lies in `axis_m x [scale_min, scale_max]`.
    Model { model: String, axis_m: f64, scale_min: f64, scale_max: f64, weight: f64 },
}

impl RockEntry {
    /// The entry's weight.
    #[must_use]
    pub fn weight(&self) -> f64 {
        match self {
            Self::Class { weight, .. } | Self::Model { weight, .. } => *weight,
        }
    }

    /// The entry's kind and name.
    fn name(&self) -> (&'static str, &str) {
        match self {
            Self::Class { class, .. } => ("class", class),
            Self::Model { model, .. } => ("model", model),
        }
    }
}

/// One rock unit (a geology unit of the adapter): its values and the mesh entries of each context.
#[derive(Debug, Clone, PartialEq)]
pub struct RockUnit {
    pub field_stone_per_ha: f64,
    pub rockfall: f64,
    /// Entries per context; a context with none is left out.
    pub entries: BTreeMap<RockContext, Vec<RockEntry>>,
}

impl RockUnit {
    /// The entries of one context (empty when none are given).
    #[must_use]
    pub fn entries(&self, context: RockContext) -> &[RockEntry] {
        self.entries.get(&context).map_or(&[], Vec::as_slice)
    }
}

/// A parsed species file.
#[derive(Debug, Clone, PartialEq)]
pub struct SpeciesFile {
    pub values: ScatterValues,
    /// The scatter-wide values with the file's v, min, max and source (for a product's provenance).
    pub declared: Vec<Declared>,
    /// The file's `[columns]` table.
    pub columns: BTreeMap<String, ColumnDecl>,
    /// Canopy stems per hectare per class name (defaults, then the file's entries).
    pub classes: BTreeMap<String, Entry>,
    /// In file order; names unique.
    pub species: Vec<Species>,
    /// The `[rocks.<context>]` rows given (a context without a row has no rocks).
    pub rocks: BTreeMap<RockContext, RockRow>,
    /// The `[rock_units."<unit>"]` tables, by unit name.
    pub rock_units: BTreeMap<String, RockUnit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    #[serde(default)]
    params: BTreeMap<String, Entry>,
    #[serde(default)]
    columns: BTreeMap<String, ColumnDecl>,
    #[serde(default)]
    classes: BTreeMap<String, RawClass>,
    #[serde(default)]
    species: Vec<RawSpecies>,
    #[serde(default, skip_serializing_if = "RawRocks::is_empty")]
    rocks: RawRocks,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    rock_units: BTreeMap<String, RawRockUnit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRocks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    talus: Option<RawRockRow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    outcrop: Option<RawRockRow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    field: Option<RawRockRow>,
}

impl RawRocks {
    fn is_empty(&self) -> bool {
        self.talus.is_none() && self.outcrop.is_none() && self.field.is_none()
    }

    fn rows(&self) -> [(RockContext, Option<&RawRockRow>); 3] {
        [
            (RockContext::Talus, self.talus.as_ref()),
            (RockContext::Outcrop, self.outcrop.as_ref()),
            (RockContext::Field, self.field.as_ref()),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRockRow {
    size_min_m: f64,
    size_max_m: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sink_small: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sink_large: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    align: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRockUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    field_stone_per_ha: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rockfall: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    talus: Vec<RockEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    outcrop: Vec<RockEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    field: Vec<RockEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawClass {
    stems_per_ha: Entry,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSpecies {
    name: String,
    layer: Layer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    size_law: SizeLaw,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    classes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parent_per_ha: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster_sigma_m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    offspring_mean: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    background_fraction: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size_shape: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size_min_m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size_max_m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crown_a_m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crown_b: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crown_k: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shade_tolerance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    footprint_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    align: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tilt_jitter_deg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    offset_m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_temperature_c: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_precipitation_mm: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_moisture: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_slope_deg: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_altitude_m: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hat_water_distance_m: Option<[f64; 4]>,
}

impl RawSpecies {
    fn numeric(&self) -> [(&'static str, Option<f64>); 15] {
        [
            ("parent_per_ha", self.parent_per_ha),
            ("cluster_sigma_m", self.cluster_sigma_m),
            ("offspring_mean", self.offspring_mean),
            ("background_fraction", self.background_fraction),
            ("size_shape", self.size_shape),
            ("size_min_m", self.size_min_m),
            ("size_max_m", self.size_max_m),
            ("crown_a_m", self.crown_a_m),
            ("crown_b", self.crown_b),
            ("crown_k", self.crown_k),
            ("shade_tolerance", self.shade_tolerance),
            ("footprint_ratio", self.footprint_ratio),
            ("align", self.align),
            ("tilt_jitter_deg", self.tilt_jitter_deg),
            ("offset_m", self.offset_m),
        ]
    }

    fn hats(&self) -> [(&'static str, Option<[f64; 4]>); 6] {
        [
            ("hat_temperature_c", self.hat_temperature_c),
            ("hat_precipitation_mm", self.hat_precipitation_mm),
            ("hat_moisture", self.hat_moisture),
            ("hat_slope_deg", self.hat_slope_deg),
            ("hat_altitude_m", self.hat_altitude_m),
            ("hat_water_distance_m", self.hat_water_distance_m),
        ]
    }
}

/// Checks the `[columns]` table: known names, fixed units, valid sources, finite ordered ranges, and a numeric
/// column's range inside its declared one.
fn check_columns(columns: &BTreeMap<String, ColumnDecl>) -> Result<()> {
    let mut declared = species_declared();
    declared.extend(rock_declared());
    for (name, c) in columns {
        let unit = SPECIES_COLUMNS
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, u)| *u)
            .ok_or_else(|| Error::Invalid(format!("scatter species: unknown column {name}")))?;
        if c.unit != unit {
            return Err(Error::Invalid(format!(
                "scatter species: column {name} unit {:?}, expected {unit:?}",
                c.unit
            )));
        }
        crate::recipe::check_source(name, &c.source)?;
        if !c.min.is_finite() || !c.max.is_finite() || c.min > c.max {
            return Err(Error::Invalid(format!(
                "scatter species: column {name} range [{}, {}] is not finite and ordered",
                c.min, c.max
            )));
        }
        if let Some(d) = declared.iter().find(|d| d.name == name)
            && (c.min < d.min || c.max > d.max)
        {
            return Err(Error::Invalid(format!(
                "scatter species: column {name} range [{}, {}] outside its declared [{}, {}]",
                c.min, c.max, d.min, d.max
            )));
        }
    }
    Ok(())
}

/// Resolves one species row: values only in declared columns and in range, defaults in range, hats ordered.
fn species_row(raw: &RawSpecies, columns: &BTreeMap<String, ColumnDecl>) -> Result<Species> {
    let name = &raw.name;
    if name.is_empty() {
        return Err(Error::Invalid("scatter species: empty species name".into()));
    }
    let declared = species_declared();
    let mut values = BTreeMap::new();
    for (col, given) in raw.numeric() {
        let column = columns.get(col);
        if given.is_some() && column.is_none() {
            return Err(Error::Invalid(format!(
                "scatter species: {name:?} gives {col}, not declared in [columns]"
            )));
        }
        let d = declared.iter().find(|d| d.name == col).expect("every numeric column is declared");
        let v = given.unwrap_or(d.v);
        if let Some(c) = column {
            crate::recipe::check_range(&format!("scatter species: {name:?} {col}"), v, c.min, c.max)?;
        }
        values.insert(col, v);
    }
    if values["size_min_m"] > values["size_max_m"] {
        return Err(Error::Invalid(format!(
            "scatter species: {name:?} size_min_m {} above size_max_m {}",
            values["size_min_m"], values["size_max_m"]
        )));
    }
    let mut hats = BTreeMap::new();
    for (col, given) in raw.hats() {
        let Some(a) = given else { continue };
        let c = columns.get(col).ok_or_else(|| {
            Error::Invalid(format!("scatter species: {name:?} gives {col}, not declared in [columns]"))
        })?;
        for v in a {
            crate::recipe::check_range(&format!("scatter species: {name:?} {col}"), v, c.min, c.max)?;
        }
        if !(a[0] <= a[1] && a[1] <= a[2] && a[2] <= a[3]) {
            return Err(Error::Invalid(format!(
                "scatter species: {name:?} {col} {a:?} is not ordered e_min <= i_min <= i_max <= e_max"
            )));
        }
        hats.insert(col, Hat::from_array(a));
    }
    for (i, class) in raw.classes.iter().enumerate() {
        if class.is_empty() || raw.classes[..i].contains(class) {
            return Err(Error::Invalid(format!(
                "scatter species: {name:?} class {class:?} is empty or repeated"
            )));
        }
    }
    if raw.model.as_deref() == Some("") {
        return Err(Error::Invalid(format!("scatter species: {name:?} has an empty model")));
    }
    Ok(Species {
        name: name.clone(),
        layer: raw.layer,
        model: raw.model.clone(),
        size_law: raw.size_law,
        classes: raw.classes.clone(),
        parent_per_ha: values["parent_per_ha"],
        cluster_sigma_m: values["cluster_sigma_m"],
        offspring_mean: values["offspring_mean"],
        background_fraction: values["background_fraction"],
        size_shape: values["size_shape"],
        size_min_m: values["size_min_m"],
        size_max_m: values["size_max_m"],
        crown_a_m: values["crown_a_m"],
        crown_b: values["crown_b"],
        crown_k: values["crown_k"],
        shade_tolerance: values["shade_tolerance"],
        footprint_ratio: values["footprint_ratio"],
        align: values["align"],
        tilt_jitter_deg: values["tilt_jitter_deg"],
        offset_m: values["offset_m"],
        hats: Hats {
            temperature_c: hats.get("hat_temperature_c").copied(),
            precipitation_mm: hats.get("hat_precipitation_mm").copied(),
            moisture: hats.get("hat_moisture").copied(),
            slope_deg: hats.get("hat_slope_deg").copied(),
            altitude_m: hats.get("hat_altitude_m").copied(),
            water_distance_m: hats.get("hat_water_distance_m").copied(),
        },
    })
}

/// One numeric value of a rock row or unit: given only in a declared column, else the declared default; in the
/// file's column range when the column is declared.
fn rock_value(
    owner: &str,
    col: &str,
    given: Option<f64>,
    columns: &BTreeMap<String, ColumnDecl>,
    declared: &[Declared],
) -> Result<f64> {
    let column = columns.get(col);
    if given.is_some() && column.is_none() {
        return Err(Error::Invalid(format!("scatter species: {owner} gives {col}, not declared in [columns]")));
    }
    let v = match given {
        Some(v) => v,
        None => declared.iter().find(|d| d.name == col).expect("every rock column is declared").v,
    };
    if let Some(c) = column {
        crate::recipe::check_range(&format!("scatter species: {owner} {col}"), v, c.min, c.max)?;
    }
    Ok(v)
}

/// Resolves one rock row: sizes required, positive and ordered; every value in its column.
fn rock_row(context: RockContext, raw: &RawRockRow, columns: &BTreeMap<String, ColumnDecl>) -> Result<RockRow> {
    let owner = format!("rock row {context:?}");
    let mut declared = species_declared();
    declared.extend(rock_declared());
    let size_min_m = rock_value(&owner, "size_min_m", Some(raw.size_min_m), columns, &declared)?;
    let size_max_m = rock_value(&owner, "size_max_m", Some(raw.size_max_m), columns, &declared)?;
    if !(size_min_m > 0.0 && size_min_m <= size_max_m) {
        return Err(Error::Invalid(format!(
            "scatter species: {owner} sizes [{size_min_m}, {size_max_m}] are not positive and ordered"
        )));
    }
    Ok(RockRow {
        size_min_m,
        size_max_m,
        sink_small: rock_value(&owner, "sink_small", raw.sink_small, columns, &declared)?,
        sink_large: rock_value(&owner, "sink_large", raw.sink_large, columns, &declared)?,
        align: rock_value(&owner, "align", raw.align, columns, &declared)?,
    })
}

/// Checks one entry: a non-empty name, a finite positive weight, and for a pack model a finite positive axis
/// and a finite, positive, ordered scale range.
fn check_entry(owner: &str, e: &RockEntry) -> Result<()> {
    let (kind, name) = e.name();
    if name.is_empty() {
        return Err(Error::Invalid(format!("scatter species: {owner} has a {kind} entry with an empty name")));
    }
    let w = e.weight();
    if !(w.is_finite() && w > 0.0) {
        return Err(Error::Invalid(format!(
            "scatter species: {owner} {kind} {name:?} weight {w} is not finite and positive"
        )));
    }
    if let RockEntry::Model { axis_m, scale_min, scale_max, .. } = e {
        if !(axis_m.is_finite() && *axis_m > 0.0) {
            return Err(Error::Invalid(format!(
                "scatter species: {owner} model {name:?} axis_m {axis_m} is not finite and positive"
            )));
        }
        if !(scale_min.is_finite() && scale_max.is_finite() && *scale_min > 0.0 && scale_min <= scale_max) {
            return Err(Error::Invalid(format!(
                "scatter species: {owner} model {name:?} scale [{scale_min}, {scale_max}] is not positive and ordered"
            )));
        }
    }
    Ok(())
}

/// Resolves one rock unit: a non-empty name, its values in their columns, entries valid and not repeated.
fn rock_unit(name: &str, raw: &RawRockUnit, columns: &BTreeMap<String, ColumnDecl>) -> Result<RockUnit> {
    if name.is_empty() {
        return Err(Error::Invalid("scatter species: empty rock unit name".into()));
    }
    let owner = format!("rock unit {name:?}");
    let declared = rock_declared();
    let mut entries = BTreeMap::new();
    for (context, list) in [
        (RockContext::Talus, &raw.talus),
        (RockContext::Outcrop, &raw.outcrop),
        (RockContext::Field, &raw.field),
    ] {
        let owner = format!("{owner} {context:?}");
        for (i, e) in list.iter().enumerate() {
            check_entry(&owner, e)?;
            if list[..i].iter().any(|p| p.name() == e.name()) {
                let (kind, n) = e.name();
                return Err(Error::Invalid(format!("scatter species: {owner} {kind} {n:?} repeated")));
            }
        }
        if !list.is_empty() {
            entries.insert(context, list.clone());
        }
    }
    Ok(RockUnit {
        field_stone_per_ha: rock_value(&owner, "field_stone_per_ha", raw.field_stone_per_ha, columns, &declared)?,
        rockfall: rock_value(&owner, "rockfall", raw.rockfall, columns, &declared)?,
        entries,
    })
}

/// Checks one class entry: non-empty name, unit 1/ha, valid source, range inside the declared one, value in range.
fn check_class(name: &str, e: &Entry) -> Result<()> {
    let d = class_declared(0.0);
    if name.is_empty() {
        return Err(Error::Invalid("scatter species: empty class name".into()));
    }
    if e.unit != d.unit {
        return Err(Error::Invalid(format!(
            "scatter species: class {name:?} stems_per_ha unit {:?}, expected {:?}",
            e.unit, d.unit
        )));
    }
    crate::recipe::check_source(name, &e.source)?;
    if !(e.min.is_finite() && e.max.is_finite() && e.min <= e.max && e.min >= d.min && e.max <= d.max) {
        return Err(Error::Invalid(format!(
            "scatter species: class {name:?} range [{}, {}] not ordered inside [{}, {}]",
            e.min, e.max, d.min, d.max
        )));
    }
    crate::recipe::check_range(&format!("scatter species: class {name:?} stems_per_ha"), e.v, e.min, e.max)
}

impl SpeciesFile {
    /// The declared defaults, the default class table, no column and no species.
    #[must_use]
    pub fn defaults() -> Self {
        Self {
            values: ScatterValues::default(),
            declared: declared(),
            columns: BTreeMap::new(),
            classes: class_defaults().into_iter().map(|(n, v)| (n.to_owned(), class_declared(v))).collect(),
            species: Vec::new(),
            rocks: BTreeMap::new(),
            rock_units: BTreeMap::new(),
        }
    }

    /// Parses a species file (module docs): params declared with their unit and a valid source, in range, the
    /// stand thresholds ordered; columns known with their fixed unit, a valid source and a range inside the
    /// declared one; classes with unit 1/ha in 0..2000; species named uniquely, values only in declared columns
    /// and in their range, sizes ordered, hats ordered; no unknown key anywhere.
    pub fn parse_toml(text: &str) -> Result<Self> {
        let file: RawFile = toml::from_str(text)?;
        let mut declared = declared();
        for (name, p) in &file.params {
            let d = declared
                .iter_mut()
                .find(|d| d.name == name)
                .ok_or_else(|| Error::Invalid(format!("scatter species: unknown param {name}")))?;
            if p.unit != d.unit {
                return Err(Error::Invalid(format!(
                    "scatter species: {name} unit {:?}, expected {:?}",
                    p.unit, d.unit
                )));
            }
            crate::recipe::check_source(name, &p.source)?;
            if !p.min.is_finite() || !p.max.is_finite() || p.min > p.max {
                return Err(Error::Invalid(format!(
                    "scatter species: {name} range [{}, {}] is not finite and ordered",
                    p.min, p.max
                )));
            }
            d.v = p.v;
            d.min = p.min;
            d.max = p.max;
            d.source.clone_from(&p.source);
        }
        let values = ScatterValues::from_declared(&declared)?;
        check_columns(&file.columns)?;
        let mut classes: BTreeMap<String, Entry> =
            class_defaults().into_iter().map(|(n, v)| (n.to_owned(), class_declared(v))).collect();
        for (name, c) in file.classes {
            check_class(&name, &c.stems_per_ha)?;
            classes.insert(name, c.stems_per_ha);
        }
        let mut species: Vec<Species> = Vec::with_capacity(file.species.len());
        for raw in &file.species {
            if species.iter().any(|s| s.name == raw.name) {
                return Err(Error::Invalid(format!("scatter species: species {:?} repeated", raw.name)));
            }
            species.push(species_row(raw, &file.columns)?);
        }
        let mut rocks = BTreeMap::new();
        for (context, raw) in file.rocks.rows() {
            if let Some(raw) = raw {
                rocks.insert(context, rock_row(context, raw, &file.columns)?);
            }
        }
        let mut rock_units = BTreeMap::new();
        for (name, raw) in &file.rock_units {
            rock_units.insert(name.clone(), rock_unit(name, raw, &file.columns)?);
        }
        Ok(Self { values, declared, columns: file.columns, classes, species, rocks, rock_units })
    }

    /// Writes the file with every value explicit (params, columns, classes, each species' numeric values in its
    /// declared columns and its hats); [`Self::parse_toml`] reads it back to an equal file.
    pub fn to_toml(&self) -> Result<String> {
        let params = self
            .declared
            .iter()
            .map(|d| {
                let e = Entry {
                    v: d.v,
                    unit: d.unit.to_owned(),
                    min: d.min,
                    max: d.max,
                    source: d.source.clone(),
                    help: String::new(),
                };
                (d.name.to_owned(), e)
            })
            .collect();
        let classes = self
            .classes
            .iter()
            .map(|(n, e)| (n.clone(), RawClass { stems_per_ha: e.clone() }))
            .collect();
        let declared_col = |c: &str| self.columns.contains_key(c);
        let opt = |c: &str, v: f64| declared_col(c).then_some(v);
        let species = self
            .species
            .iter()
            .map(|s| RawSpecies {
                name: s.name.clone(),
                layer: s.layer,
                model: s.model.clone(),
                size_law: s.size_law,
                classes: s.classes.clone(),
                parent_per_ha: opt("parent_per_ha", s.parent_per_ha),
                cluster_sigma_m: opt("cluster_sigma_m", s.cluster_sigma_m),
                offspring_mean: opt("offspring_mean", s.offspring_mean),
                background_fraction: opt("background_fraction", s.background_fraction),
                size_shape: opt("size_shape", s.size_shape),
                size_min_m: opt("size_min_m", s.size_min_m),
                size_max_m: opt("size_max_m", s.size_max_m),
                crown_a_m: opt("crown_a_m", s.crown_a_m),
                crown_b: opt("crown_b", s.crown_b),
                crown_k: opt("crown_k", s.crown_k),
                shade_tolerance: opt("shade_tolerance", s.shade_tolerance),
                footprint_ratio: opt("footprint_ratio", s.footprint_ratio),
                align: opt("align", s.align),
                tilt_jitter_deg: opt("tilt_jitter_deg", s.tilt_jitter_deg),
                offset_m: opt("offset_m", s.offset_m),
                hat_temperature_c: s.hats.temperature_c.map(Hat::to_array),
                hat_precipitation_mm: s.hats.precipitation_mm.map(Hat::to_array),
                hat_moisture: s.hats.moisture.map(Hat::to_array),
                hat_slope_deg: s.hats.slope_deg.map(Hat::to_array),
                hat_altitude_m: s.hats.altitude_m.map(Hat::to_array),
                hat_water_distance_m: s.hats.water_distance_m.map(Hat::to_array),
            })
            .collect();
        let row = |c: RockContext| {
            self.rocks.get(&c).map(|r| RawRockRow {
                size_min_m: r.size_min_m,
                size_max_m: r.size_max_m,
                sink_small: opt("sink_small", r.sink_small),
                sink_large: opt("sink_large", r.sink_large),
                align: opt("align", r.align),
            })
        };
        let rocks = RawRocks {
            talus: row(RockContext::Talus),
            outcrop: row(RockContext::Outcrop),
            field: row(RockContext::Field),
        };
        let rock_units = self
            .rock_units
            .iter()
            .map(|(n, u)| {
                let raw = RawRockUnit {
                    field_stone_per_ha: opt("field_stone_per_ha", u.field_stone_per_ha),
                    rockfall: opt("rockfall", u.rockfall),
                    talus: u.entries(RockContext::Talus).to_vec(),
                    outcrop: u.entries(RockContext::Outcrop).to_vec(),
                    field: u.entries(RockContext::Field).to_vec(),
                };
                (n.clone(), raw)
            })
            .collect();
        let raw = RawFile { params, columns: self.columns.clone(), classes, species, rocks, rock_units };
        toml::to_string(&raw).map_err(|e| Error::Invalid(e.to_string()))
    }
}
