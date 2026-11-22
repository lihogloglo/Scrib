//! **Placed landforms** (LF-2, `docs/specs/landforms.md` "Pipeline" steps 2-7): discrete landforms put where the
//! geology says, on an otherwise untouched base field. LF-2 carries the placement core, `fault_scarp` and `fan`
//! (talus) ([`scarp`]), the thermal settle and the walkability repair ramps ([`routes`]).
//!
//! Bake order ([`place`]):
//!
//! 1. **Geology** ([`crate::geology::build`]) on the base field.
//! 2. **Analysis** (no heights change): slope, its box mean over `calm_window_m`, D8 drainage area on the
//!    Priority-Flood surface ([`crate::stream::route`], as the census), distance to the sea and to frozen (anchor
//!    footprint) samples.
//! 3. **Traces:** unit boundaries ([`scarp::traces`]), broken at the sea, the coast guard and anchor bands.
//! 4. **Placement:** a boundary is a fault with probability `fault_share` (hash); each fault trace holds candidates
//!    of length `L` from the truncated power law `N(>L) ~ L^-b` on `[size_min_m, min(size_max_m, trace)]`, throw
//!    `throw_ratio L` (displacement-length scaling, Cowie & Scholz 1992), upthrown on the harder unit's side (a
//!    fault-line scarp faces the softer rock). Score = region weight x (`contrast_floor` + hardness contrast) x calm
//!    (mean of `1 - slope / calm_slope_deg` along the trace, so faces land on calm ground where they read) x hash.
//!    Candidates are accepted best first while (a) the region's face budget is left (estimated face area = the
//!    face's horizontal width plus the prefilter footprint, integrated along strike), (b) no accepted fault's
//!    centre lies within `spacing_ratio x (L + L') / 2`, (c) the trace avoids anchor bands (step 3), (d) its mean
//!    water-gap factor is at least `gap_reject` (a boundary that follows a river is no scarp). Each fault is
//!    split into en-echelon segments with relay ramps and water gaps where channels over `gap_area_m2` cross
//!    ([`scarp`] docs). The delta fades to 0 toward the sea (`coast_guard_m`) and anchors (`anchor_band_m`; 0 inside
//!    a footprint, so anchors stay bit-exact).
//! 5. **Settle:** [`crate::thermal::relax`] with per-sample repose from the unit and bed (hard beds
//!    `repose_hard_deg`, soft `repose_soft_deg`; a sample whose drop spans more than a mean bed uses the unit's
//!    bed-averaged hardness), only within `settle.margin_m` of a changed sample; frozen samples restored.
//! 6. **Routes** ([`routes`]), last, so the repair sees the final field.
//! 7. **Masks** from the final field into the geology map: `cliff` (face zone, slope >= `cliff.slope_deg`),
//!    `talus` (talus zone), `lip` (signed distance to the lip, + outward/down, clamped), `feature` (index + 1).
//!
//! Every value is declared ([`declared_defaults`]; recipe `[values]`, `--set`); region weights and face budgets come
//! from the recipe's `[region."<name>"]` tables (adapter data). Nothing here names a game.

pub mod facets;
pub mod routes;
pub mod scarp;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::filter::{Declared, pcg3d};
use crate::geology::{BaseGrid, GeologyMap, GeologyParams, GeologyTable, f16_bits};
use crate::guide::{Guide, RegionMap};
use crate::measure::{UNTOUCHED_M, declared_walk, distance_transform, walk_slope_deg};
use crate::thermal::{ThermalParams, relax};
use crate::{Audit, Error, Grid, Result, hash_f32};
use routes::{Ramp, RegionRoute, RouteParams, repair};
use facets::Wobble;
use scarp::{Segment, Shape, Zone, arc_len, eval_fault, ramp01};

/// Polyline vertex spacing of traces, samples.
// tunables: const (geometry resolution: a vertex every 2 samples, the guide footprint of the prefilter)
const TRACE_STEP_SAMPLES: f64 = 2.0;

/// Repose of a sample that thermal must never move (outside the settle margin): `tan` of ~90 deg.
// tunables: const (sentinel: an unreachable talus slope)
const NEVER_TAN: f64 = 1e9;

/// Hardness below which a bed is soft (`geology` "Beds": soft beds sit below 0.5).
// tunables: const (definition shared with geology.rs: soft = hardness below 0.5)
const SOFT_BELOW: f64 = 0.5;

/// Declared placement values (`landforms.md` "Tunables"; recipe `[values]`, `--set`).
#[allow(clippy::too_many_lines)]
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
    let walk = declared_walk().remove(0);
    vec![
        d("seed", 11.0, "id", 0.0, 4_294_967_295.0, "Guess", "hash seed of fault picks, sizes and ranks"),
        d("face_budget", 0.04, "ratio", 0.0, 0.3, "Reference(Etna 1.8 %, Teide 6.2 % of land >= 35 deg)", "placed face area per region, share of its land (recipe overrides per region)"),
        d("size_b", 1.3, "ratio", 0.5, 3.0, "Industry(S: fault-length power laws b 1-2)", "exponent of the truncated power law N(>L) ~ L^-b"),
        d("size_min_m", 800.0, "m", 50.0, 50_000.0, "Guess", "shortest fault"),
        d("size_max_m", 6000.0, "m", 100.0, 100_000.0, "Guess", "longest fault (also capped by its trace)"),
        d("spacing_ratio", 1.5, "ratio", 0.0, 10.0, "Guess", "accepted fault centres stay this many mean lengths apart"),
        d("face_soft_deg", 45.0, "deg", 20.0, 85.0, "Industry(V: Wallace 1978, fresh scarps 50-90 deg; owner 2026-11-22 dramatic)", "face angle of a fault whose upthrown unit has hardness 0 (linear in hardness)"),
        d("face_hard_deg", 75.0, "deg", 20.0, 85.0, "Industry(cap-rock faces 60-75 deg)", "face angle of a fault whose upthrown unit has hardness 1"),
        d("talus_deg", 33.0, "deg", 20.0, 40.0, "Industry(repose of scree 32-36 deg)", "talus (fan) angle"),
        d("fan_share", 0.35, "ratio", 0.0, 0.8, "Guess", "eroded 0: share of the throw buried under the talus fan; eroded 1: debris-cone apex of the largest chute, share of the throw"),
        d("eroded", 1.0, "switch", 0.0, 1.0, "Owner(2026-11-22: scarps read as eroded fault scarps, LF-2b)", "1 = faces dissected into facets and chutes with debris cones; 0 = the LF-2 straight face + uniform talus"),
        d("throw_noise", 0.35, "ratio", 0.0, 0.9, "Guess(S: displacement profiles of real faults are irregular)", "along-strike throw modulation, +- share of the throw"),
        d("throw_noise_m", 1200.0, "m", 50.0, 50_000.0, "Guess", "wavelength of the along-strike throw modulation (second octave at half)"),
        d("segment_throw_var", 0.3, "ratio", 0.0, 0.9, "Guess", "segment-to-segment throw variation, +- share"),
        d("tip_ratio", 0.2, "ratio", 0.0, 0.5, "Industry(S: Cowie & Scholz 1992, throw tapers to 0 at fault tips)", "taper of a fault's outer tips, share of its length"),
        d("chute_min_m", 50.0, "m", 5.0, 5000.0, "Owner(2026-11-22 brief: chute spacing 50-300 m)", "smallest chute spacing"),
        d("chute_max_m", 300.0, "m", 5.0, 5000.0, "Owner(2026-11-22 brief: chute spacing 50-300 m)", "largest chute spacing"),
        d("chute_b", 1.0, "ratio", 0.3, 3.0, "Guess(S: drainage spacing power laws)", "exponent of the chute spacing power law"),
        d("chute_width_share", 0.9, "ratio", 0.1, 1.5, "Guess", "chute half-width at the lip over half its spacing (1 = neighbouring chutes meet at the lip: facet apexes on the lip)"),
        d("chute_wall_deg", 40.0, "deg", 15.0, 80.0, "Guess(ravine side walls in rock 35-45 deg)", "chute side-wall slope"),
        d("chute_depth_max", 0.85, "ratio", 0.0, 0.98, "Guess", "deepest chute at the lip, share of the face height above its mouth"),
        d("cone_deg", 12.0, "deg", 2.0, 30.0, "Industry(S: Blair & McPherson 1994, fans 1.5-25 deg, debris-flow fans steep)", "debris-cone slope at chute mouths"),
        d("facet_talus_share", 0.1, "ratio", 0.0, 0.5, "Guess", "eroded 1: thin talus at a facet's foot, share of the throw"),
        d("prefilter_taps", 5.0, "count", 1.0, 16.0, "Guess", "eroded 1: box-filter taps per axis (midpoint rule)"),
        d("back_deg", 6.0, "deg", 0.5, 10.0, "Guess(tilted fault blocks dip a few degrees away from the scarp)", "footwall back-tilt: the top falls back to the base this steeply"),
        d("throw_ratio", 0.02, "ratio", 0.005, 0.1, "Industry(S: Cowie & Scholz 1992 D/L 0.01-0.03)", "maximum throw over fault length"),
        d("fault_share", 0.5, "ratio", 0.0, 1.0, "Guess", "share of unit boundaries that are faults"),
        d("segment_m", 1500.0, "m", 100.0, 100_000.0, "Guess", "en-echelon segment length (a fault of length L has round(L / this) segments)"),
        d("relay_overlap", 0.2, "ratio", 0.0, 0.6, "Guess", "overlap of consecutive segments, share of a segment (the relay ramp length)"),
        d("relay_aspect", 4.0, "ratio", 1.0, 20.0, "Industry(S: Soliva & Benedicto 2004, relay overlap / separation ~ 2-5)", "relay overlap over the segments' separation"),
        d("tip_min_m", 150.0, "m", 0.0, 5000.0, "Guess", "shortest taper of a segment's throw at its ends"),
        d("trace_smooth_m", 250.0, "m", 0.0, 5000.0, "Guess", "moving-average window of a traced boundary"),
        d("calm_slope_deg", 35.0, "deg", 5.0, 80.0, "Guess", "mean ground slope at which a trace's calm score reaches 0"),
        d("calm_window_m", 200.0, "m", 0.0, 5000.0, "Guess", "half-width of the box the ground slope is averaged over"),
        d("contrast_floor", 0.2, "ratio", 0.0, 1.0, "Guess", "score of a boundary between units of equal hardness"),
        d("gap_area_m2", 5.0e6, "m2", 0.0, 1.0e12, "Guess", "drainage area at which a crossing channel cuts a water gap (throw 0; ramp over x0.5-x2)"),
        d("gap_reject", 0.5, "ratio", 0.0, 1.0, "Guess", "a candidate whose mean water-gap factor is below this is dropped (a trace along a river)"),
        d("gap_width_m", 120.0, "m", 0.0, 2000.0, "Guess", "half-width of a water gap along strike"),
        d("coast_guard_m", 250.0, "m", 0.0, 5000.0, "Guess", "deltas fade to 0 over this distance to the sea; traces stop there"),
        d("anchor_band_m", 150.0, "m", 0.0, 5000.0, "Guess", "deltas fade to 0 over this distance to anchor footprints; traces stop there"),
        d("prefilter_samples", 2.0, "count", 0.0, 8.0, "Guess(spec: profiles box-filtered over 2 x spacing)", "box-filter width across the strike, samples"),
        d("route.min_largest", 0.95, "ratio", 0.0, 1.0, "Reference(Teide 98.7 % largest walkable)", "repair until the region's largest walkable component holds this share of its land"),
        d("route.ramp_deg", 30.0, "deg", 5.0, 45.0, "Guess(<= 46 with margin)", "steepest ramp grade"),
        d("route.ramp_width_m", 24.0, "m", 1.0, 500.0, "Guess", "ramp core width (at least 3 samples)"),
        d("route.max_ramps", 400.0, "count", 0.0, 100_000.0, "Guess", "ramps per region"),
        d("route.min_component_m2", 50_000.0, "m2", 0.0, 1.0e9, "Guess", "smallest walkable component a ramp is cut to"),
        d("route.max_cross_m", 300.0, "m", 0.0, 10_000.0, "Guess", "longest unwalkable crossing a ramp replaces"),
        d("route.max_ramp_m", 800.0, "m", 10.0, 20_000.0, "Guess", "longest ramp"),
        d("route.max_cut_m", 25.0, "m", 0.0, 500.0, "Guess(graded road cuts and fills ~5-25 m)", "largest cut or fill of a ramp's centre line (no causeways over ravines)"),
        d("route.shoulder_m", 30.0, "m", 0.0, 500.0, "Guess", "blend of a ramp into the ground at its sides and ends"),
        d("route.rounds", 6.0, "count", 0.0, 100.0, "Guess", "relabel-and-repair rounds"),
        d("settle.margin_m", 60.0, "m", 0.0, 5000.0, "Guess(spec 300; smaller keeps the base untouched)", "thermal settle only this close to a changed sample"),
        d("settle.iterations", 40.0, "count", 0.0, 10_000.0, "Guess", "thermal settle steps"),
        d("settle.rate", 0.25, "ratio", 0.01, 0.5, "Guess", "share of the excess moved per thermal step"),
        d("cliff.slope_deg", 45.0, "deg", 20.0, 89.0, "Reference(rockgen cliff.slope_deg)", "a face sample at least this steep is cliff in the mask"),
        Declared { name: "max_walk_deg", ..walk },
    ]
}

/// Checked values of [`declared_defaults`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaceParams {
    pub seed: u32,
    pub face_budget: f64,
    pub size_b: f64,
    pub size_min_m: f64,
    pub size_max_m: f64,
    pub spacing_ratio: f64,
    pub face_deg: [f64; 2],
    pub talus_deg: f64,
    pub fan_share: f64,
    pub eroded: bool,
    pub throw_noise: f64,
    pub throw_noise_m: f64,
    pub segment_throw_var: f64,
    pub tip_ratio: f64,
    pub chute: facets::ChuteParams,
    pub chute_wall_deg: f64,
    pub cone_deg: f64,
    pub prefilter_taps: usize,
    pub back_deg: f64,
    pub throw_ratio: f64,
    pub fault_share: f64,
    pub segment_m: f64,
    pub relay_overlap: f64,
    pub relay_aspect: f64,
    pub tip_min_m: f64,
    pub trace_smooth_m: f64,
    pub calm_slope_deg: f64,
    pub calm_window_m: f64,
    pub contrast_floor: f64,
    pub gap_area_m2: f64,
    pub gap_reject: f64,
    pub gap_width_m: f64,
    pub coast_guard_m: f64,
    pub anchor_band_m: f64,
    pub prefilter_samples: f64,
    pub route: RouteParams,
    pub settle_margin_m: f64,
    pub settle_iterations: usize,
    pub settle_rate: f64,
    pub cliff_slope_deg: f64,
}

impl PlaceParams {
    /// Reads every value of `d` (ranges checked).
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |n: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == n)
                .ok_or_else(|| Error::Invalid(format!("place: missing value {n}")))?;
            if !(p.min..=p.max).contains(&p.v) {
                return Err(Error::Invalid(format!("place: {n} = {} outside [{}, {}]", p.v, p.min, p.max)));
            }
            Ok(p.v)
        };
        let p = Self {
            seed: get("seed")? as u32,
            face_budget: get("face_budget")?,
            size_b: get("size_b")?,
            size_min_m: get("size_min_m")?,
            size_max_m: get("size_max_m")?,
            spacing_ratio: get("spacing_ratio")?,
            face_deg: [get("face_soft_deg")?, get("face_hard_deg")?],
            talus_deg: get("talus_deg")?,
            fan_share: get("fan_share")?,
            eroded: get("eroded")? != 0.0,
            throw_noise: get("throw_noise")?,
            throw_noise_m: get("throw_noise_m")?,
            segment_throw_var: get("segment_throw_var")?,
            tip_ratio: get("tip_ratio")?,
            chute: facets::ChuteParams {
                min_m: get("chute_min_m")?,
                max_m: get("chute_max_m")?,
                b: get("chute_b")?,
                width_share: get("chute_width_share")?,
                depth_max: get("chute_depth_max")?,
                fan_share: get("fan_share")?,
                facet_talus: get("facet_talus_share")?,
            },
            chute_wall_deg: get("chute_wall_deg")?,
            cone_deg: get("cone_deg")?,
            prefilter_taps: get("prefilter_taps")? as usize,
            back_deg: get("back_deg")?,
            throw_ratio: get("throw_ratio")?,
            fault_share: get("fault_share")?,
            segment_m: get("segment_m")?,
            relay_overlap: get("relay_overlap")?,
            relay_aspect: get("relay_aspect")?,
            tip_min_m: get("tip_min_m")?,
            trace_smooth_m: get("trace_smooth_m")?,
            calm_slope_deg: get("calm_slope_deg")?,
            calm_window_m: get("calm_window_m")?,
            contrast_floor: get("contrast_floor")?,
            gap_area_m2: get("gap_area_m2")?,
            gap_reject: get("gap_reject")?,
            gap_width_m: get("gap_width_m")?,
            coast_guard_m: get("coast_guard_m")?,
            anchor_band_m: get("anchor_band_m")?,
            prefilter_samples: get("prefilter_samples")?,
            route: RouteParams {
                max_walk_deg: get("max_walk_deg")?,
                min_largest: get("route.min_largest")?,
                ramp_deg: get("route.ramp_deg")?,
                ramp_width_m: get("route.ramp_width_m")?,
                max_ramps: get("route.max_ramps")? as usize,
                min_component_m2: get("route.min_component_m2")?,
                max_cross_m: get("route.max_cross_m")?,
                max_ramp_m: get("route.max_ramp_m")?,
                shoulder_m: get("route.shoulder_m")?,
                max_cut_m: get("route.max_cut_m")?,
                rounds: get("route.rounds")? as usize,
            },
            settle_margin_m: get("settle.margin_m")?,
            settle_iterations: get("settle.iterations")? as usize,
            settle_rate: get("settle.rate")?,
            cliff_slope_deg: get("cliff.slope_deg")?,
        };
        if p.size_min_m > p.size_max_m || p.chute.min_m > p.chute.max_m {
            return Err(Error::Invalid("place: size_min_m > size_max_m or chute_min_m > chute_max_m".into()));
        }
        Ok(p)
    }

    /// Face angle of a fault whose upthrown unit has hardness `hardness` (linear between the soft and hard angles).
    pub fn face_deg_at(&self, hardness: f64) -> f64 {
        let t = hardness.clamp(0.0, 1.0);
        self.face_deg[0] + (self.face_deg[1] - self.face_deg[0]) * t
    }

    fn shape(&self, spacing: f64) -> Shape {
        Shape {
            tan_face: self.face_deg_at(1.0).to_radians().tan(),
            tan_talus: self.talus_deg.to_radians().tan(),
            fan: if self.eroded { self.chute.facet_talus } else { self.fan_share },
            tan_back: self.back_deg.to_radians().tan(),
            filter_r: self.prefilter_samples * spacing * 0.5,
            tan_cone: self.cone_deg.to_radians().tan(),
            tan_wall: self.chute_wall_deg.to_radians().tan(),
            taps: self.prefilter_taps.max(1),
        }
    }
}

/// Per-region placement knobs (recipe `[region."<name>"]`, adapter data).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionKnobs {
    /// Multiplies every candidate's score in the region (0 = no faults).
    #[serde(default = "one")]
    pub weight: f64,
    /// Overrides `face_budget`.
    #[serde(default)]
    pub face_budget: Option<f64>,
    /// Overrides `throw_ratio` (LF-2b: taller faults where the region allows).
    #[serde(default)]
    pub throw_ratio: Option<f64>,
    /// Overrides `size_min_m`.
    #[serde(default)]
    pub size_min_m: Option<f64>,
    /// Overrides `size_max_m`.
    #[serde(default)]
    pub size_max_m: Option<f64>,
}

fn one() -> f64 {
    1.0
}

/// The landforms recipe (TOML): `[values]` (declared overrides) and `[region."<name>"]` knobs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlaceRecipe {
    #[serde(default)]
    pub values: BTreeMap<String, f64>,
    #[serde(default)]
    pub region: BTreeMap<String, RegionKnobs>,
}

impl PlaceRecipe {
    pub fn from_toml(text: &str) -> Result<Self> {
        toml::from_str(text).map_err(|e| Error::Invalid(format!("landforms recipe: {e}")))
    }

    /// [`declared_defaults`] with `[values]` (source `Recipe`) then `sets` (source `Input(--set)`).
    pub fn declared(&self, sets: &[(String, f64)]) -> Result<Vec<Declared>> {
        let mut d = declared_defaults();
        let mut put = |k: &str, v: f64, src: &str| -> Result<()> {
            let p = d
                .iter_mut()
                .find(|p| p.name == k)
                .ok_or_else(|| Error::Invalid(format!("place: unknown value {k}")))?;
            p.v = v;
            p.source = src.into();
            Ok(())
        };
        for (k, &v) in &self.values {
            put(k, v, "Recipe")?;
        }
        for (k, v) in sets {
            put(k, *v, "Input(--set)")?;
        }
        Ok(d)
    }
}

/// The field a placement runs on.
#[derive(Debug, Clone, Copy)]
pub struct PlaceInput<'a> {
    pub heights: &'a [f64],
    pub w: usize,
    pub h: usize,
    pub spacing_m: f64,
    pub origin_m: [f64; 2],
    pub sea_level_m: f64,
    pub regions: Option<&'a RegionMap>,
    /// Samples no step may write (anchor footprints).
    pub frozen: &'a [bool],
    /// Score weight per region legend index.
    pub weight: &'a [f64; 256],
    /// Face budget per region legend index (share of land).
    pub budget: &'a [f64; 256],
    /// Fault length law `[size_min_m, size_max_m]` per region legend index.
    pub size_m: &'a [[f64; 2]; 256],
    /// `throw_ratio` per region legend index.
    pub throw_ratio: &'a [f64; 256],
}

/// Per-region arrays of a [`PlaceInput`] from the declared values and the recipe's region knobs (by legend index).
#[derive(Debug, Clone)]
pub struct RegionArrays {
    pub weight: [f64; 256],
    pub budget: [f64; 256],
    pub size_m: [[f64; 2]; 256],
    pub throw_ratio: [f64; 256],
}

impl RegionArrays {
    /// Every region at the declared values.
    pub fn uniform(p: &PlaceParams) -> Self {
        Self {
            weight: [1.0; 256],
            budget: [p.face_budget; 256],
            size_m: [[p.size_min_m, p.size_max_m]; 256],
            throw_ratio: [p.throw_ratio; 256],
        }
    }

    /// Applies one region's knobs (ranges checked as the declared values).
    pub fn set(&mut self, v: u8, k: &RegionKnobs) -> Result<()> {
        let v = usize::from(v);
        self.weight[v] = k.weight;
        if let Some(b) = k.face_budget {
            self.budget[v] = b;
        }
        if let Some(t) = k.throw_ratio {
            self.throw_ratio[v] = t;
        }
        if let Some(a) = k.size_min_m {
            self.size_m[v][0] = a;
        }
        if let Some(b) = k.size_max_m {
            self.size_m[v][1] = b;
        }
        let [a, b] = self.size_m[v];
        let t = self.throw_ratio[v];
        if !(a > 0.0 && a <= b && (0.0..=0.2).contains(&t) && k.weight >= 0.0) {
            return Err(Error::Invalid(format!("landforms recipe: region #{v} knobs out of range")));
        }
        Ok(())
    }
}

/// One placed feature (`features.json`, format 1).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlacedFeature {
    pub id: usize,
    pub kind: &'static str,
    pub seed: u32,
    pub region: String,
    /// Lip polyline per segment, guide frame (x east, y north).
    pub lip: Vec<Vec<[f64; 2]>>,
    pub length_m: f64,
    pub throw_m: f64,
    pub top_m: f64,
    pub base_m: f64,
    pub face_deg: f64,
    pub talus_deg: f64,
    pub face_width_m: f64,
    /// Talus apron width, or the largest debris-cone radius of an eroded face.
    pub apron_m: f64,
    /// Chutes (ravines) of an eroded face (LF-2b; 0 = straight face).
    pub chutes: usize,
    /// `[top, face, base]` unit names.
    pub units: [String; 3],
    /// Tips and relay-ramp centres, guide frame.
    pub routes: Vec<[f64; 2]>,
    /// Unit vector (guide frame) the face looks along, from the lip toward the downthrown side, at the middle of
    /// the first segment.
    pub facing: [f64; 2],
    pub face_area_m2: f64,
    pub bbox: [f64; 4],
}

/// Per-region numbers of one placement.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct RegionPlace {
    pub region: String,
    pub land_km2: f64,
    pub budget_km2: f64,
    /// Estimated placed face area (budget accounting).
    pub face_km2: f64,
    pub faults: usize,
    /// Realised fault throws p50 / p90 / max, metres.
    pub throw_m: [f64; 3],
    /// Cliff-mask area from the final field.
    pub cliff_km2: f64,
    pub talus_km2: f64,
    pub route: RegionRoute,
}

/// What a placement did (printed, never limits).
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PlaceReport {
    pub steps_s: Vec<(String, f64)>,
    pub traces: usize,
    /// Trace lengths p50 / p90 / max, metres.
    pub trace_len_m: [f64; 3],
    /// Realised fault throws p50 / p90 / max, metres.
    pub throw_m: [f64; 3],
    pub candidates: usize,
    pub placed: usize,
    pub segments: usize,
    pub rejected_budget: usize,
    pub rejected_spacing: usize,
    pub rejected_gap: usize,
    pub regions: Vec<RegionPlace>,
    pub new_wet: usize,
    pub new_dry: usize,
    pub sea_moved: usize,
    pub frozen_moved: usize,
    pub settle_moved_m3: f64,
    pub ramps: usize,
    pub hash: String,
}

/// A placement's products.
#[derive(Debug, Clone)]
pub struct Placed {
    pub heights: Vec<f64>,
    pub geology: GeologyMap,
    pub features: Vec<PlacedFeature>,
    /// Ramps, guide frame.
    pub ramps: Vec<Ramp>,
    pub report: PlaceReport,
}

/// p50 / p90 / max of `v` (0 when empty).
pub fn quantiles(v: &[f64]) -> [f64; 3] {
    let mut l = v.to_vec();
    l.sort_by(f64::total_cmp);
    let q = |f: f64| l.get(((l.len().max(1) - 1) as f64 * f) as usize).copied().unwrap_or(0.0);
    [q(0.5), q(0.9), q(1.0)]
}

/// Largest throw along a fault's segments, at their vertices, metres.
pub fn realised_throw(segs: &[Segment]) -> f64 {
    segs.iter()
        .flat_map(|sg| sg.cum.iter().map(move |&c| sg.throw_at(c)))
        .fold(0.0, f64::max)
}

/// u32 -> [0, 1) with 24 bits.
#[inline]
fn u01(v: u32) -> f64 {
    f64::from(v >> 8) / 16_777_216.0
}

/// `L` from the truncated power law `N(>L) ~ L^-b` on `[lo, hi]` at quantile `u`.
pub fn power_law_length(u: f64, lo: f64, hi: f64, b: f64) -> f64 {
    if hi <= lo {
        return lo;
    }
    let (a, z) = (lo.powf(-b), hi.powf(-b));
    (a - u * (a - z)).powf(-1.0 / b)
}

/// Box mean of `v` over finite samples within a `(2r + 1)^2` square (summed-area table).
fn box_mean(v: &[f64], w: usize, h: usize, r: usize) -> Vec<f64> {
    let (w1, h1) = (w + 1, h + 1);
    let mut sum = vec![0.0f64; w1 * h1];
    let mut cnt = vec![0u32; w1 * h1];
    for y in 0..h {
        for x in 0..w {
            let val = v[y * w + x];
            let (a, c) = if val.is_finite() { (val, 1) } else { (0.0, 0) };
            let k = (y + 1) * w1 + x + 1;
            sum[k] = a + sum[k - 1] + sum[k - w1] - sum[k - w1 - 1];
            cnt[k] = c + cnt[k - 1] + cnt[k - w1] - cnt[k - w1 - 1];
        }
    }
    let mut out = vec![f64::NAN; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let (y0, y1) = (y.saturating_sub(r), (y + r + 1).min(h));
        for (x, o) in row.iter_mut().enumerate() {
            let (x0, x1) = (x.saturating_sub(r), (x + r + 1).min(w));
            let s = sum[y1 * w1 + x1] - sum[y0 * w1 + x1] - sum[y1 * w1 + x0] + sum[y0 * w1 + x0];
            let c = cnt[y1 * w1 + x1] + cnt[y0 * w1 + x0] - cnt[y0 * w1 + x1] - cnt[y1 * w1 + x0];
            if c > 0 {
                *o = s / f64::from(c);
            }
        }
    });
    out
}

/// One candidate fault.
#[derive(Debug, Clone)]
struct Candidate {
    trace: usize,
    k: u32,
    s0: f64,
    len: f64,
    side: f64,
    score: f64,
    region: u8,
    centre: [f64; 2],
    seed: u32,
}

/// One accepted fault.
#[derive(Debug, Clone)]
struct Fault {
    cand: Candidate,
    segs: Vec<Segment>,
    face_m2: f64,
    up: u8,
    down: u8,
}

/// Runs the placement (module docs).
#[allow(clippy::too_many_lines)]
pub fn place(inp: &PlaceInput<'_>, table: &GeologyTable, gp: &GeologyParams, p: &PlaceParams) -> Result<Placed> {
    let (w, h, s) = (inp.w, inp.h, inp.spacing_m);
    let n = w * h;
    if inp.heights.len() != n || inp.frozen.len() != n || w < 3 || h < 3 {
        return Err(Error::Invalid("place: grid size".into()));
    }
    let sea = inp.sea_level_m;
    let base = inp.heights;
    let mut steps: Vec<(String, f64)> = Vec::new();
    let mut tick = Instant::now();
    let mut lap = |name: &str, steps: &mut Vec<(String, f64)>| {
        steps.push((name.into(), tick.elapsed().as_secs_f64()));
        tick = Instant::now();
    };
    let wet: Vec<bool> = base.iter().map(|&v| v < sea).collect();
    let labels: Vec<u8> = match inp.regions {
        Some(rm) => (0..n).map(|i| rm.at_sample(i % w, i / w, s)).collect(),
        None => vec![0; n],
    };
    let region_name = |v: u8| -> String {
        inp.regions
            .and_then(|rm| rm.names.get(usize::from(v)).filter(|s| !s.is_empty()).cloned())
            .unwrap_or_else(|| format!("#{v}"))
    };

    // 1. Geology.
    let mut geo = crate::geology::build(
        &BaseGrid {
            heights: base,
            w,
            h,
            spacing_m: s,
            origin_m: inp.origin_m,
            regions: inp.regions,
        },
        table,
        gp,
    )?;
    lap("geology", &mut steps);

    // 2. Analysis.
    let g0 = Grid {
        w,
        h,
        spacing: s,
        data: base.to_vec(),
    };
    let slope0 = walk_slope_deg(&g0, sea);
    let calm = box_mean(&slope0, w, h, (p.calm_window_m / s).round() as usize);
    let mut zr = Vec::new();
    let area = crate::stream::route(&g0, &wet, sea, &mut zr).area;
    let coast_m: Vec<f64> = distance_transform(&wet, w, h).iter().map(|d| d * s).collect();
    let frozen_m: Vec<f64> = distance_transform(inp.frozen, w, h).iter().map(|d| d * s).collect();
    lap("analysis", &mut steps);

    // 3. Traces (as short as the shortest fault any region allows).
    let size_floor = inp.size_m.iter().map(|v| v[0]).fold(f64::INFINITY, f64::min);
    let blocked: Vec<bool> = (0..n)
        .map(|i| wet[i] || coast_m[i] < p.coast_guard_m || frozen_m[i] < p.anchor_band_m)
        .collect();
    let unit_hardness: Vec<f64> = geo.units.iter().map(|u| u.hardness).collect();
    let traces = scarp::traces(
        &geo.unit,
        &unit_hardness,
        &blocked,
        w,
        h,
        p.trace_smooth_m / s,
        TRACE_STEP_SAMPLES,
        size_floor / s,
    );
    let traces_m: Vec<Vec<[f64; 2]>> = traces
        .iter()
        .map(|t| t.pts.iter().map(|q| [q[0] * s, q[1] * s]).collect())
        .collect();
    lap("traces", &mut steps);

    // 4. Candidates.
    let sample_of = |q: [f64; 2]| -> usize {
        let c = ((q[0] / s).round().max(0.0) as usize).min(w - 1);
        let r = ((q[1] / s).round().max(0.0) as usize).min(h - 1);
        r * w + c
    };
    let units = &geo.units;
    let mut cands: Vec<Candidate> = Vec::new();
    for (ti, (tr, pts)) in traces.iter().zip(&traces_m).enumerate() {
        let total = arc_len(pts);
        // The length law of the region holding the trace middle.
        let [size_min, size_max] = inp.size_m[usize::from(labels[sample_of(pts[pts.len() / 2])])];
        if total < size_min {
            continue;
        }
        let key = (u32::from(tr.a) << 8) | u32::from(tr.b);
        let first = sample_of(pts[0]) as u32;
        let hf = pcg3d(p.seed ^ 0x51ab_3e07, key, first);
        if u01(hf[0]) >= p.fault_share {
            continue;
        }
        let (ha, hb) = (units[usize::from(tr.a)].hardness, units[usize::from(tr.b)].hardness);
        let contrast = (ha - hb).abs();
        let tmpl = Segment::new(pts.clone(), 1.0, 0.0, 0.0);
        let count = (total / size_min).ceil() as u32;
        for k in 0..count {
            let hk = pcg3d(p.seed ^ 0x2c9e_77d1, first, k + 1);
            let len = power_law_length(u01(hk[0]), size_min, size_max.min(total), p.size_b);
            let s0 = u01(hk[1]) * (total - len).max(0.0);
            // Vertices of the candidate's arc range.
            let idx: Vec<usize> = (0..pts.len())
                .filter(|&v| tmpl.cum[v] >= s0 && tmpl.cum[v] <= s0 + len)
                .collect();
            if idx.len() < 2 {
                continue;
            }
            // Which normal side holds unit a (majority over the vertices, 3 samples out).
            let mut votes = 0i64;
            let mut calm_sum = 0.0;
            for &v in &idx {
                let (pa, pb) = (pts[v.saturating_sub(1)], pts[(v + 1).min(pts.len() - 1)]);
                let l = (pb[0] - pa[0]).hypot(pb[1] - pa[1]).max(1e-9);
                let nrm = [-(pb[1] - pa[1]) / l, (pb[0] - pa[0]) / l];
                let q = [pts[v][0] + nrm[0] * 3.0 * s, pts[v][1] + nrm[1] * 3.0 * s];
                let qn = [pts[v][0] - nrm[0] * 3.0 * s, pts[v][1] - nrm[1] * 3.0 * s];
                votes += i64::from(geo.unit[sample_of(q)] == tr.a) - i64::from(geo.unit[sample_of(qn)] == tr.a);
                let c = calm[sample_of(pts[v])];
                if c.is_finite() {
                    calm_sum += (1.0 - c / p.calm_slope_deg).clamp(0.0, 1.0);
                }
            }
            let a_left = votes >= 0;
            // The trace's own unit is the harder one: the upthrown side.
            let side = if a_left { 1.0 } else { -1.0 };
            let mid = pts[idx[idx.len() / 2]];
            let region = labels[sample_of(mid)];
            let calm_score = calm_sum / idx.len() as f64;
            let score = inp.weight[usize::from(region)] * (p.contrast_floor + contrast) * calm_score * u01(hk[2]);
            cands.push(Candidate {
                trace: ti,
                k,
                s0,
                len,
                side,
                score,
                region,
                centre: mid,
                seed: hk[2],
            });
        }
    }
    let n_cands = cands.len();
    cands.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.trace.cmp(&b.trace)).then(a.k.cmp(&b.k)));

    // Accept.
    let shape = p.shape(s);
    let mut land_n = [0usize; 256];
    for i in 0..n {
        if !wet[i] {
            land_n[usize::from(labels[i])] += 1;
        }
    }
    let budget_m2: Vec<f64> = (0..256).map(|v| inp.budget[v] * land_n[v] as f64 * s * s).collect();
    let mut used_m2 = [0.0f64; 256];
    let mut faults: Vec<Fault> = Vec::new();
    let (mut rej_budget, mut rej_spacing, mut rej_gap) = (0usize, 0usize, 0usize);
    let gap_r = (p.gap_width_m / s).ceil() as isize;
    for c in cands {
        if c.score <= 0.0 {
            continue;
        }
        let v = usize::from(c.region);
        if faults.iter().any(|f| {
            let d = (f.cand.centre[0] - c.centre[0]).hypot(f.cand.centre[1] - c.centre[1]);
            d < p.spacing_ratio * 0.5 * (f.cand.len + c.len)
        }) {
            rej_spacing += 1;
            continue;
        }
        let tr = &traces[c.trace];
        let h_max = inp.throw_ratio[v] * c.len;
        let mut segs = scarp::segments(
            &traces_m[c.trace],
            c.s0,
            c.len,
            c.side,
            h_max,
            p.segment_m,
            p.relay_overlap,
            p.relay_aspect,
            p.tip_min_m,
        );
        // LF-2b: face angle from the upthrown unit, segment-to-segment throw, outer tips, wobble, chutes.
        let tan_face = p.face_deg_at(units[usize::from(tr.a)].hardness).to_radians().tan();
        let tip = (p.tip_ratio * c.len).max(p.tip_min_m);
        let nseg = segs.len();
        for (k, sg) in segs.iter_mut().enumerate() {
            let hk = pcg3d(p.seed ^ 0x7a3c_51e9, c.seed, k as u32);
            sg.h_max *= 1.0 + p.segment_throw_var * (2.0 * u01(hk[0]) - 1.0);
            sg.tan_face = Some(tan_face);
            let half = 0.5 * sg.len();
            if k == 0 {
                sg.taper[0] = sg.taper[0].max(tip).min(half);
            }
            if k + 1 == nseg {
                sg.taper[1] = sg.taper[1].max(tip).min(half);
            }
            sg.wobble = Wobble {
                amp: p.throw_noise,
                wavelength_m: p.throw_noise_m,
                seed: hk[1],
            };
            if p.eroded {
                sg.chutes = facets::chutes(sg.len(), sg.h_max, shape.tan_wall, &p.chute, hk[2]);
                sg.chute_reach = facets::chute_reach(sg, &shape);
            }
        }
        // Water gaps.
        for sg in &mut segs {
            let raw: Vec<f64> = sg
                .pts
                .iter()
                .map(|q| {
                    let (cx, cy) = ((q[0] / s).round() as isize, (q[1] / s).round() as isize);
                    let mut amax = 0.0f64;
                    for yy in (cy - gap_r).max(0)..=(cy + gap_r).min(h as isize - 1) {
                        for xx in (cx - gap_r).max(0)..=(cx + gap_r).min(w as isize - 1) {
                            amax = amax.max(area[yy as usize * w + xx as usize]);
                        }
                    }
                    let lo = 0.5 * p.gap_area_m2;
                    1.0 - ramp01(amax - lo, 2.0 * p.gap_area_m2 - lo)
                })
                .collect();
            let half = (p.gap_width_m / (TRACE_STEP_SAMPLES * s)).round() as usize;
            sg.gap = (0..raw.len())
                .map(|i| {
                    let (a, b) = (i.saturating_sub(half), (i + half + 1).min(raw.len()));
                    raw[a..b].iter().sum::<f64>() / (b - a) as f64
                })
                .collect();
        }
        let (gap_sum, gap_n) = segs
            .iter()
            .flat_map(|sg| sg.gap.iter())
            .fold((0.0, 0usize), |(a, k), &v| (a + v, k + 1));
        if gap_n == 0 || gap_sum / (gap_n as f64) < p.gap_reject {
            rej_gap += 1;
            continue;
        }
        let face_m2: f64 = segs
            .iter()
            .map(|sg| {
                (1..sg.pts.len())
                    .map(|k| {
                        let ds = sg.cum[k] - sg.cum[k - 1];
                        let th = sg.throw_at(f64::midpoint(sg.cum[k], sg.cum[k - 1]));
                        if th > 0.0 {
                            ds * (th / tan_face + 2.0 * shape.filter_r)
                        } else {
                            0.0
                        }
                    })
                    .sum::<f64>()
            })
            .sum();
        if used_m2[v] + face_m2 > budget_m2[v] {
            rej_budget += 1;
            continue;
        }
        used_m2[v] += face_m2;
        let (up, down) = (tr.a, tr.b);
        faults.push(Fault {
            cand: c,
            segs,
            face_m2,
            up,
            down,
        });
    }
    lap("placement", &mut steps);

    // Rasterize: per fault in parallel, merged in fault order (largest delta wins).
    let hits: Vec<Vec<(u32, f64, Zone, f64)>> = faults
        .par_iter()
        .map(|f| {
            let reach = f.segs.iter().map(|sg| sg.reach(&shape)).fold(0.0, f64::max);
            let mut bb = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
            for sg in &f.segs {
                let b = sg.bounds();
                bb = [bb[0].min(b[0]), bb[1].min(b[1]), bb[2].max(b[2]), bb[3].max(b[3])];
            }
            let c0 = ((bb[0] - reach) / s).floor().max(0.0) as usize;
            let r0 = ((bb[1] - reach) / s).floor().max(0.0) as usize;
            let c1 = (((bb[2] + reach) / s).ceil().max(0.0) as usize).min(w - 1);
            let r1 = (((bb[3] + reach) / s).ceil().max(0.0) as usize).min(h - 1);
            let mut out = Vec::new();
            for r in r0..=r1 {
                for c in c0..=c1 {
                    let i = r * w + c;
                    if wet[i] {
                        continue;
                    }
                    if let Some(hit) = eval_fault(&f.segs, &shape, [c as f64 * s, r as f64 * s]) {
                        let fade = ramp01(coast_m[i], p.coast_guard_m) * ramp01(frozen_m[i], p.anchor_band_m);
                        let dv = hit.delta * fade;
                        if dv > 0.0 && !inp.frozen[i] {
                            out.push((i as u32, dv, hit.zone, hit.d));
                        }
                    }
                }
            }
            out
        })
        .collect();
    let mut delta = vec![0.0f64; n];
    let mut owner = vec![u32::MAX; n];
    let mut zone = vec![Zone::Plain; n];
    let mut dist_d = vec![0.0f64; n];
    for (fi, list) in hits.iter().enumerate() {
        for &(i, dv, z, d) in list {
            let i = i as usize;
            if dv > delta[i] {
                delta[i] = dv;
                owner[i] = fi as u32;
                zone[i] = z;
                dist_d[i] = d;
            }
        }
    }
    drop(hits);
    let mut g = Grid {
        w,
        h,
        spacing: s,
        data: base.iter().zip(&delta).map(|(&b, &d)| if d > 0.0 { b + d } else { b }).collect(),
    };
    lap("raster", &mut steps);

    // 5. Settle (thermal, near changed samples only).
    let changed: Vec<bool> = delta.iter().map(|&d| d > 0.0).collect();
    let near: Vec<bool> = if changed.iter().any(|&c| c) {
        distance_transform(&changed, w, h).iter().map(|&d| d * s <= p.settle_margin_m).collect()
    } else {
        vec![false; n]
    };
    let mut settle_moved = 0.0;
    if near.iter().any(|&v| v) && p.settle_iterations > 0 {
        let tan: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|i| {
                if !near[i] || wet[i] || inp.frozen[i] {
                    return NEVER_TAN;
                }
                let u = &geo.units[usize::from(geo.unit[i])];
                let (c, r) = (i % w, i / w);
                let z = g.data[i];
                let mut drop = 0.0f64;
                for k in 0..8 {
                    if let Some(j) = g.neighbour(i, k) {
                        drop = drop.max(z - g.data[j]);
                    }
                }
                let mean_bed = f64::midpoint(u.bed_m[0], u.bed_m[1]);
                let hard = if u.layered && drop > mean_bed {
                    (1.0 - u.soft_bed_share) * u.hardness + u.soft_bed_share * gp.soft_bed_ratio * u.hardness
                } else {
                    let x = inp.origin_m[0] + c as f64 * s;
                    let y = inp.origin_m[1] - r as f64 * s;
                    geo.bed_at(x, y, z).hardness
                };
                let deg = if hard >= SOFT_BELOW { u.repose_hard_deg } else { u.repose_soft_deg };
                deg.to_radians().tan()
            })
            .collect();
        settle_moved = relax(
            &mut g,
            &ThermalParams {
                iterations: p.settle_iterations,
                rate: p.settle_rate,
                tan_talus: &tan,
                sea_level: sea,
            },
        )?;
        for ((v, &b), &f) in g.data.iter_mut().zip(base).zip(inp.frozen) {
            if f {
                *v = b;
            }
        }
    }
    lap("settle", &mut steps);

    // 6. Routes.
    let (ramps_local, route_rows) = repair(&mut g, sea, &labels, inp.frozen, &p.route);
    lap("routes", &mut steps);

    // 7. Masks and features.
    let slope_f = walk_slope_deg(&g, sea);
    let lip_band = gp.lip_band_m;
    let mut cliff_n = [0usize; 256];
    let mut talus_n = [0usize; 256];
    for i in 0..n {
        if owner[i] == u32::MAX {
            continue;
        }
        if delta[i] >= UNTOUCHED_M {
            geo.feature[i] = (owner[i] + 1).min(u32::from(u16::MAX)) as u16;
        }
        let v = usize::from(labels[i]);
        match zone[i] {
            Zone::Face if slope_f[i] >= p.cliff_slope_deg => {
                geo.cliff[i] = 255;
                cliff_n[v] += 1;
            }
            Zone::Talus if delta[i] >= UNTOUCHED_M => {
                geo.talus[i] = 255;
                talus_n[v] += 1;
            }
            _ => {}
        }
        geo.lip[i] = f16_bits((-dist_d[i]).clamp(-lip_band, lip_band) as f32);
    }
    let to_guide = |q: [f64; 2]| [inp.origin_m[0] + q[0], inp.origin_m[1] - q[1]];
    let height_at = |q: [f64; 2]| -> f64 { g.data[sample_of(q)] };
    let features: Vec<PlacedFeature> = faults
        .iter()
        .enumerate()
        .map(|(fi, f)| {
            let mut tops = Vec::new();
            let mut bases = Vec::new();
            let mut bb = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
            for sg in &f.segs {
                for (k, q) in sg.pts.iter().enumerate() {
                    let th = sg.throw_at(sg.cum[k]);
                    if th <= 0.5 * sg.h_max {
                        continue;
                    }
                    let (pa, pb) = (sg.pts[k.saturating_sub(1)], sg.pts[(k + 1).min(sg.pts.len() - 1)]);
                    let l = (pb[0] - pa[0]).hypot(pb[1] - pa[1]).max(1e-9);
                    let nrm = [-(pb[1] - pa[1]) / l * sg.side, (pb[0] - pa[0]) / l * sg.side];
                    let din = shape.filter_r + s;
                    let dout = -shape.profile(th).knots()[0] + shape.filter_r + s;
                    tops.push(height_at([q[0] + nrm[0] * din, q[1] + nrm[1] * din]));
                    bases.push(height_at([q[0] - nrm[0] * dout, q[1] - nrm[1] * dout]));
                }
                let b = sg.bounds();
                let (g0, g1) = (to_guide([b[0], b[3]]), to_guide([b[2], b[1]]));
                bb = [bb[0].min(g0[0]), bb[1].min(g0[1]), bb[2].max(g1[0]), bb[3].max(g1[1])];
            }
            let mean = |v: &[f64]| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 };
            let mut routes = Vec::new();
            if let (Some(a), Some(b)) = (f.segs.first(), f.segs.last()) {
                routes.push(to_guide(a.pts[0]));
                routes.push(to_guide(*b.pts.last().unwrap_or(&b.pts[0])));
            }
            for pair in f.segs.windows(2) {
                let (a, b) = (*pair[0].pts.last().unwrap_or(&pair[0].pts[0]), pair[1].pts[0]);
                routes.push(to_guide([f64::midpoint(a[0], b[0]), f64::midpoint(a[1], b[1])]));
            }
            // Realised throw: the largest along strike (tapers, gaps and wobble included).
            let h_max = realised_throw(&f.segs);
            let tan_face = f.segs.first().and_then(|sg| sg.tan_face).unwrap_or(shape.tan_face);
            let fan = if p.eroded { p.chute.facet_talus } else { p.fan_share };
            let cone_r = f
                .segs
                .iter()
                .flat_map(|sg| sg.chutes.iter())
                .map(|c| c.apex * h_max / shape.tan_cone)
                .fold(0.0, f64::max);
            let facing = f.segs.first().map_or([0.0, 0.0], |sg| {
                let k = sg.pts.len() / 2;
                let (pa, pb) = (sg.pts[k.saturating_sub(1)], sg.pts[(k + 1).min(sg.pts.len() - 1)]);
                let l = (pb[0] - pa[0]).hypot(pb[1] - pa[1]).max(1e-9);
                // Upthrown = left normal x side (local frame, y = south); the face looks the other way.
                let up = [-(pb[1] - pa[1]) / l * sg.side, (pb[0] - pa[0]) / l * sg.side];
                [-up[0], up[1]]
            });
            PlacedFeature {
                id: fi,
                kind: "fault_scarp",
                seed: f.cand.seed,
                region: region_name(f.cand.region),
                lip: f.segs.iter().map(|sg| sg.pts.iter().map(|&q| to_guide(q)).collect()).collect(),
                length_m: f.cand.len,
                throw_m: h_max,
                top_m: mean(&tops),
                base_m: mean(&bases),
                face_deg: tan_face.atan().to_degrees(),
                talus_deg: p.talus_deg,
                face_width_m: (1.0 - fan) * h_max / tan_face,
                apron_m: (fan * h_max / shape.tan_talus).max(cone_r),
                chutes: f.segs.iter().map(|sg| sg.chutes.len()).sum(),
                units: [
                    units[usize::from(f.up)].name.clone(),
                    units[usize::from(f.up)].name.clone(),
                    units[usize::from(f.down)].name.clone(),
                ],
                routes,
                facing,
                face_area_m2: f.face_m2,
                bbox: bb,
            }
        })
        .collect();
    let ramps: Vec<Ramp> = ramps_local
        .iter()
        .map(|r| Ramp {
            from: to_guide(r.from),
            to: to_guide(r.to),
            ..*r
        })
        .collect();
    let audit = Audit::compare(base, &g.data, sea);
    let frozen_moved = (0..n)
        .filter(|&i| inp.frozen[i] && base[i].to_bits() != g.data[i].to_bits())
        .count();
    let mut faults_in = [0usize; 256];
    let mut throws_in: Vec<Vec<f64>> = vec![Vec::new(); 256];
    for f in &faults {
        faults_in[usize::from(f.cand.region)] += 1;
        throws_in[usize::from(f.cand.region)].push(realised_throw(&f.segs));
    }
    let all_throws: Vec<f64> = throws_in.iter().flatten().copied().collect();
    let a2 = s * s * 1e-6;
    let regions: Vec<RegionPlace> = route_rows
        .iter()
        .map(|rr| {
            let v = usize::from(rr.label);
            RegionPlace {
                region: region_name(rr.label),
                land_km2: land_n[v] as f64 * a2,
                budget_km2: budget_m2[v] * 1e-6,
                face_km2: used_m2[v] * 1e-6,
                faults: faults_in[v],
                throw_m: quantiles(&throws_in[v]),
                cliff_km2: cliff_n[v] as f64 * a2,
                talus_km2: talus_n[v] as f64 * a2,
                route: rr.clone(),
            }
        })
        .collect();
    lap("masks", &mut steps);
    let report = PlaceReport {
        steps_s: steps,
        traces: traces.len(),
        trace_len_m: {
            let mut l: Vec<f64> = traces_m.iter().map(|t| arc_len(t)).collect();
            l.sort_by(f64::total_cmp);
            let q = |f: f64| l.get(((l.len().max(1) - 1) as f64 * f) as usize).copied().unwrap_or(0.0);
            [q(0.5), q(0.9), q(1.0)]
        },
        throw_m: quantiles(&all_throws),
        candidates: n_cands,
        placed: faults.len(),
        segments: faults.iter().map(|f| f.segs.len()).sum(),
        rejected_budget: rej_budget,
        rejected_spacing: rej_spacing,
        rejected_gap: rej_gap,
        regions,
        new_wet: audit.new_wet,
        new_dry: audit.new_dry,
        sea_moved: audit.sea_moved,
        frozen_moved,
        settle_moved_m3: settle_moved,
        ramps: ramps.len(),
        hash: hash_f32(&g.data),
    };
    Ok(Placed {
        heights: g.data,
        geology: geo,
        features,
        ramps,
        report,
    })
}

/// One `scrib-worldgen place` invocation.
#[derive(Debug, Clone)]
pub struct PlaceRequest {
    /// Base guide folder (v1).
    pub base: PathBuf,
    /// Geology table TOML.
    pub geology: PathBuf,
    /// Landforms recipe TOML (optional).
    pub recipe: Option<PathBuf>,
    /// `name=value` overrides (after the recipe's `[values]`).
    pub sets: Vec<(String, f64)>,
    /// Output guide folder.
    pub out: PathBuf,
    /// Per-world filter file copied into the output as `filter.toml` (LF-2b steep-face gully fade; checked by
    /// [`crate::tiles::world_filter`]).
    pub world_filter: Option<PathBuf>,
}

/// Frozen samples of a guide folder: stamp weight > 0 (the anchors' footprints).
pub fn frozen_mask(dir: &Path, gd: &Guide) -> Result<Vec<bool>> {
    let Some(set) = crate::stamps::StampSet::read(dir)? else {
        return Ok(vec![false; gd.w * gd.h]);
    };
    let idx = crate::stamps::StampIndex::new(set);
    let mut out = vec![false; gd.w * gd.h];
    out.par_chunks_mut(gd.w).enumerate().for_each(|(r, row)| {
        for (c, o) in row.iter_mut().enumerate() {
            let x = gd.origin_m[0] + c as f64 * gd.spacing_m;
            let y = gd.origin_m[1] - r as f64 * gd.spacing_m;
            *o = idx.sample(x, y).is_some_and(|(_, wt)| wt > 0.0);
        }
    });
    Ok(out)
}

/// Reads the guide, table and recipe, runs [`place`], writes the full guide folder (every base file copied but the
/// heights and meta), `geology/`, `features.json` and `README.md`; returns the report and the bake time.
#[allow(clippy::too_many_lines)]
pub fn run_place(req: &PlaceRequest) -> Result<(PlaceReport, f64)> {
    let gd = Guide::read(&req.base)?;
    let table = GeologyTable::from_toml(&std::fs::read_to_string(&req.geology)?)?;
    let gdecl = table.declared()?;
    let gp = GeologyParams::from_declared(&gdecl)?;
    let recipe = match &req.recipe {
        Some(path) => PlaceRecipe::from_toml(&std::fs::read_to_string(path)?)?,
        None => PlaceRecipe::default(),
    };
    let decl = recipe.declared(&req.sets)?;
    let params = PlaceParams::from_declared(&decl)?;
    let mut arrays = RegionArrays::uniform(&params);
    for (name, k) in &recipe.region {
        let v = gd
            .regions
            .as_ref()
            .and_then(|rm| rm.find(name))
            .ok_or_else(|| Error::Invalid(format!("landforms recipe: unknown region {name}")))?;
        arrays.set(v, k).map_err(|e| Error::Invalid(format!("{e} ({name})")))?;
    }
    let frozen = frozen_mask(&req.base, &gd)?;
    let heights: Vec<f64> = gd.heights.iter().map(|&v| f64::from(v)).collect();
    let t = Instant::now();
    let placed = place(
        &PlaceInput {
            heights: &heights,
            w: gd.w,
            h: gd.h,
            spacing_m: gd.spacing_m,
            origin_m: gd.origin_m,
            sea_level_m: gd.sea_level_m,
            regions: gd.regions.as_ref(),
            frozen: &frozen,
            weight: &arrays.weight,
            budget: &arrays.budget,
            size_m: &arrays.size_m,
            throw_ratio: &arrays.throw_ratio,
        },
        &table,
        &gp,
        &params,
    )?;
    let bake_s = t.elapsed().as_secs_f64();
    let rep = &placed.report;
    if rep.new_wet + rep.new_dry + rep.sea_moved + rep.frozen_moved > 0 {
        return Err(Error::Invalid(format!(
            "place broke an invariant: new wet {}, new dry {}, sea moved {}, frozen moved {}",
            rep.new_wet, rep.new_dry, rep.sea_moved, rep.frozen_moved
        )));
    }

    // Guide folder: copy the base's files, then the new heights and meta.
    std::fs::create_dir_all(&req.out)?;
    for entry in std::fs::read_dir(&req.base)? {
        let entry = entry?;
        let name = entry.file_name();
        let skip = ["heights.f32", "meta.json", "README.md", "features.json"];
        if entry.file_type()?.is_file() && !skip.iter().any(|s| name == *s) {
            std::fs::copy(entry.path(), req.out.join(&name))?;
        }
    }
    let mut raw = Vec::with_capacity(placed.heights.len() * 4);
    for &v in &placed.heights {
        raw.extend_from_slice(&(v as f32).to_le_bytes());
    }
    std::fs::write(req.out.join("heights.f32"), &raw)?;
    let mut meta: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(req.base.join("meta.json"))?)?;
    if let Some(obj) = meta.as_object_mut() {
        obj.insert("producer".into(), "scrib-worldgen place (sf_worldgen::placed, LF-2b)".into());
        obj.insert("heights_blake3".into(), rep.hash.clone().into());
        obj.insert("heights_from".into(), req.base.display().to_string().into());
        obj.insert(
            "placed".into(),
            serde_json::json!({
                "geology_table": req.geology.display().to_string(),
                "recipe": req.recipe.as_ref().map(|p| p.display().to_string()),
                "bake_s": bake_s,
                "faults": rep.placed,
                "segments": rep.segments,
                "ramps": rep.ramps,
            }),
        );
    }
    std::fs::write(req.out.join("meta.json"), serde_json::to_string_pretty(&meta)?)?;
    let geo_hash = placed.geology.write(&req.out, &gdecl)?;
    let features = serde_json::json!({
        "format": 1,
        "frame": { "origin_m": gd.origin_m, "spacing_m": gd.spacing_m },
        "features": placed.features,
        "ramps": placed.ramps,
        "declared": decl,
    });
    std::fs::write(req.out.join("features.json"), serde_json::to_string_pretty(&features)?)?;
    if let Some(wf) = &req.world_filter {
        std::fs::copy(wf, req.out.join(crate::tiles::WORLD_FILTER_FILE))?;
        crate::tiles::world_filter(&req.out)?;
    }
    std::fs::write(req.out.join("README.md"), markdown(req, rep, bake_s, &geo_hash))?;
    Ok((placed.report, bake_s))
}

/// The run's README.
pub fn markdown(req: &PlaceRequest, rep: &PlaceReport, bake_s: f64, geo_hash: &str) -> String {
    let mut s = String::from("# Placed landforms (`scrib-worldgen place`, LF-2)\n\n");
    let _ = writeln!(s, "Base `{}`, geology `{}`.\n", req.base.display(), req.geology.display());
    let steps: Vec<String> = rep.steps_s.iter().map(|(k, v)| format!("{k} {v:.2}")).collect();
    let _ = writeln!(s, "Bake {bake_s:.2} s ({}).\n", steps.join(", "));
    let _ = writeln!(
        s,
        "Traces {} (length p50 / p90 / max {:.0} / {:.0} / {:.0} m), candidates {}, placed {} faults ({} segments), rejected: budget {}, spacing {}, water gap {}; ramps {}; \
         settle moved {:.0} m3; wet/dry {}/{}/{}, frozen moved {}; heights blake3 `{}`, geology `{}`.\n",
        rep.traces,
        rep.trace_len_m[0],
        rep.trace_len_m[1],
        rep.trace_len_m[2],
        rep.candidates,
        rep.placed,
        rep.segments,
        rep.rejected_budget,
        rep.rejected_spacing,
        rep.rejected_gap,
        rep.ramps,
        rep.settle_moved_m3,
        rep.new_wet,
        rep.new_dry,
        rep.sea_moved,
        rep.frozen_moved,
        &rep.hash[..12],
        &geo_hash[..12.min(geo_hash.len())]
    );
    s.push_str(
        "| Region | Land km2 | Budget km2 | Face km2 (est.) | Faults | Throw p50 / p90 / max m | Cliff km2 | Talus km2 | Largest walkable before % | after % | Ramps | Failed |\n",
    );
    s.push_str("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for r in &rep.regions {
        let _ = writeln!(
            s,
            "| {} | {:.1} | {:.2} | {:.2} | {} | {:.0} / {:.0} / {:.0} | {:.2} | {:.2} | {:.1} | {:.1} | {} | {} |",
            r.region,
            r.land_km2,
            r.budget_km2,
            r.face_km2,
            r.faults,
            r.throw_m[0],
            r.throw_m[1],
            r.throw_m[2],
            r.cliff_km2,
            r.talus_km2,
            100.0 * r.route.largest_before,
            100.0 * r.route.largest_after,
            r.route.ramps,
            r.route.failed
        );
    }
    s
}
