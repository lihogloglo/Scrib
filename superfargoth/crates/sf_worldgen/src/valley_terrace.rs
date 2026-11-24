//! **Site preparation: river-valley terraces** (`valley_terrace`, spec `superfargoth/docs/specs/site-prep.md`, slice
//! SP-1a): reshapes the dry banks of a channel into a terraced valley (a quay at the water, treads stepping up to the
//! bank top, risers between them) and writes the result as one height stamp (format v1, [`crate::stamps`]) plus a
//! `valley.json` report. Generic: a guide sampler, a sea level and a town extent in; nothing here names a game.
//!
//! Everything derives from the channel's distance field (exact EDT on the analysis grid, wet = `h <= sea`), with the
//! nearest wet sample of each dry sample (its **foot**, a feature transform) carrying the bank top and the count:
//!
//! 1. **Bank top `P`** per foot: mean guide height of the dry samples with `d` in `[ref_lo_m, ref_hi_m]` inside a disc
//!    of `ref_radius_m` around a probe `(ref_lo_m + ref_hi_m) / 2` inland of the foot along the edge normal (the
//!    normal is the gradient of the signed distance field at the foot), capped at `sea + bank_cap_m`.
//! 2. **Treads:** quay level `q = sea + quay_freeboard_m`; the continuous count `c = (P - q) / riser_h` (riser height
//!    modulated by noise) is reduced per **along-valley cell** (a `cell_len_m` lattice on the foot positions, per
//!    bank) to `n = clamp(ceil(max c), 1, treads_max)`; neighbouring cells cross-fade over a quarter cell. `ceil` (not
//!    the spec's `round`) keeps every rise at or under the local riser height, so tread levels rise by at most
//!    `riser_h_m (1 + vary_amp)`; the rise is also capped there where a neighbouring cell's count is lower.
//! 3. **Variation:** riser height (value noise at the foot, through the count); tread width (`vary_amp`) and the
//!    first tread's width by `meander_m` (value noise at the sample itself, so the wavelengths hold in world space on
//!    both sides of a bend), so every riser line above the first bends like a cut bank.
//! 4. **Risers** rise over `rise / slope` along the perpendicular: kind **wall** where the riser's foot line lies
//!    inside `core_radius_m`, kind **bank** outside. The slope is `riser_slope` inside the core and eases to
//!    `bank_slope` over the last quarter cell (`cell_len_m / 4`) before the core edge (smootherstep), so a riser line
//!    crossing the edge never shifts the risers above it in one step. The quay edge is kind **quay**.
//! 5. **Beyond the top tread** the target meets the guide by a talus bank (`clamp(g, top - t, top + t)`, `t = talus x
//!    run`). **Cut cap:** ground standing more than `cut_cap_m` above the top tread is never cut deeper than
//!    `cut_cap_m`: such samples hold `g - cut_cap_m`, and a talus cone grown from them (a two-pass 5x5 chamfer,
//!    weights 1, sqrt 2, sqrt 5, confined to the sample's bank) is the natural bank that replaces the stair below
//!    them. The stair's own excavation (below the top tread, down to the quay) is the design and is not capped.
//! 6. **Weight** 1 on every touched sample, 0 on wet samples, smootherstep to 0 over `blend_m` beyond the touched area
//!    and over [`ValleyParams::edge_blend_m`] outside the extent (`reach_m`): the stair runs along the river through
//!    the extent edge, so that fade undoes the stair's own excavation (up to `bank_cap_m - quay_freeboard_m`, more
//!    than `cut_cap_m`) and the spec's rule `blend >= cut / talus` is applied to that depth.
//!
//! **Ramps** (SP-1c, `ramps` on by default): before the weights, switchback columns of ramps every `ramp_spacing_m`
//! are cut into the risers, each rising at `ramp_grade` along its riser from the lower tread to the upper one
//! ([`cut_ramps`]); they only cut, inside the touched area, so the weights are those without ramps.
//!
//! **Banks** are the 4-connected dry components of the analysis grid (deterministic: labelled in scan order); the
//! report gives cut and fill per bank and each bank's centroid. Polylines (quay edge, riser foot lines) follow the
//! chain of feet of each bank. [`merge_over`] composes this stamp with a later one on the same lattice exactly.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::filter::Declared;
use crate::measure::distance_transform;
use crate::perturb::value_noise;
use crate::stamps::{Stamp, StampSet};
use crate::volcano::smootherstep;
use crate::{Error, Result};

/// Most treads a stair can hold (the declared range's maximum of `treads_max`).
// tunables: const (capacity: upper bound of the declared `treads_max` range)
const MAXN: usize = 8;

/// Share of a cell over which neighbouring cells' counts cross-fade.
// tunables: const (spec definition: "cross-faded over a quarter cell")
const FADE_SHARE: f64 = 0.25;

/// Half-width of the central-difference stencil of the edge normal and the distance gradient, samples.
// tunables: const (geometry: gradient stencil of the signed distance field)
const NORMAL_STENCIL: usize = 4;

/// Polyline vertex spacing along the edge, metres.
// tunables: const (geometry resolution of the report polylines)
const LINE_STEP_M: f64 = 2.0;

/// Two profile heights closer than this are the same surface, metres.
// tunables: const (numerics: f64 rounding of equal profiles)
const SAME_EPS_M: f64 = 1e-6;

/// Noise streams (xor into the seed) of tread width, riser height and meander.
// tunables: const (format: noise stream ids)
const STREAM_TREAD: u64 = 0x7472_6561_6400_0001;
// tunables: const (format: noise stream ids)
const STREAM_RISER: u64 = 0x7269_7365_7200_0002;
// tunables: const (format: noise stream ids)
const STREAM_MEANDER: u64 = 0x6d65_616e_6400_0003;

/// Finite-difference step of the noise gradients, metres.
// tunables: const (numerics: central-difference step, far under the shortest noise wavelength)
const FD_M: f64 = 0.5;

/// Fixed-point iterations that find a riser line along a ray.
// tunables: const (numerics: iterations of a contraction, converged far under a sample)
const RAY_ITERS: usize = 8;

/// Iterations of a ramp's length fit (length against the feet it touches); a ramp still growing after them does not fit.
// tunables: const (numerics: a fixed point reached in two or three steps)
const RAMP_FIT_ITERS: usize = 16;

/// Side (metres) of the buckets that index the ramps' bounding boxes for the cut.
// tunables: const (indexing: a lookup grid's cell, any size gives the same cut)
const RAMP_BUCKET_M: f64 = 32.0;

/// Half-window (feet) of the moving average that smooths the ramp centreline points along a chain.
// tunables: const (numerics: averages the digitised edge's jitter, far under the bend radii and ramp lengths)
const SMOOTH_FEET: usize = 8;

/// `valley.json` format version.
// tunables: const (format version)
const REPORT_FORMAT: u32 = 1;

/// Declared values of the operator (spec `site-prep.md` "Tunables", `Lifetime::Bake`).
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
    vec![
        d("quay_freeboard_m", 2.5, "m", 1.0, 4.0, "Guess (no water above sea, no flood model)",
            "quay level above the sea"),
        d("quay_w_m", 8.0, "m", 4.0, 16.0, "Guess", "width of the quay tread at the water"),
        d("riser_h_m", 5.0, "m", 3.0, 7.0, "Reference(Amalfi dry-stone terrace walls 3-7 m)",
            "riser height before noise; the tread count follows from it"),
        d("treads_max", 5.0, "count", 2.0, 8.0, "Guess", "most treads above the quay"),
        d("tread_w_m", 28.0, "m", 16.0, 50.0, "Reference(one row of 8-16 m building depths plus a street)",
            "tread width before noise"),
        d("tread_fall", 0.02, "m/m", 0.01, 0.04, "Guess (far under the 0.12 street grade)",
            "cross-fall of a tread toward the water"),
        d("riser_slope", 1.0, "m/m", 0.7, 2.0,
            "Reference(wall-backed bank; a 0.5 m height sample cannot hold a vertical face)",
            "slope of a riser inside the town core (a retaining wall stands in front)"),
        d("bank_slope", 0.6, "m/m", 0.4, 0.8, "Guess (grass holds)", "slope of an earth-bank riser outside the core"),
        d("talus_slope", 0.6, "m/m", 0.4, 0.8, "Industry(thermal talus relaxation, Musgrave et al. 1989)",
            "slope of natural banks meeting the guide"),
        d("cut_cap_m", 12.0, "m", 6.0, 25.0, "Guess", "deepest cut into ground above the top tread"),
        d("blend_m", 40.0, "m", 20.0, 120.0, "Industry(stamp blend margin >= cut / tan(talus))",
            "falloff of the weight beyond the touched area and the extent"),
        d("ref_radius_m", 90.0, "m", 50.0, 150.0, "Guess (under half the channel plus banks, banks never mix)",
            "disc radius of the bank-top mean"),
        d("ref_lo_m", 90.0, "m", 30.0, 200.0, "Measured(the plain starts 30-110 m from the water, site-prep research)",
            "near edge of the bank-top band, distance from the water"),
        d("ref_hi_m", 160.0, "m", 60.0, 400.0, "Measured(the plain starts 30-110 m from the water, site-prep research)",
            "far edge of the bank-top band, distance from the water"),
        d("bank_cap_m", 40.0, "m", 20.0, 80.0, "Guess", "highest bank top above the sea"),
        d("vary_wavelength_m", 250.0, "m", 100.0, 600.0, "Guess (unpaired terraces)",
            "wavelength of the tread-width and riser-height noise"),
        d("vary_amp", 0.25, "ratio", 0.0, 0.5, "Guess", "relative amplitude of the tread-width and riser-height noise"),
        d("meander_m", 6.0, "m", 0.0, 15.0, "Guess", "offset amplitude of the riser lines"),
        d("meander_wavelength_m", 70.0, "m", 30.0, 200.0, "Guess", "wavelength of the riser-line offset"),
        d("cell_len_m", 120.0, "m", 60.0, 300.0, "Guess", "along-valley cell over which the tread count holds"),
        d("spacing_m", 1.0, "m", 0.5, 2.0, "Reference(the town stamp's grid, so merges are exact)",
            "stamp sample spacing"),
        d("ramps", 1.0, "switch", 0.0, 1.0, "Guess (owner 2026-11-24: ramps cut into the steps)",
            "1 cuts ramps into the risers (SP-1c), 0 leaves the stair without them"),
        d("ramp_spacing_m", 100.0, "m", 40.0, 400.0, "Guess", "along-valley spacing of the ramp columns"),
        d("ramp_w_m", 6.0, "m", 3.0, 12.0, "Guess (one street of the settlement planner)",
            "width of a ramp's floor on the upper tread"),
        d("ramp_grade", 0.10, "m/m", 0.04, 0.12, "Guess (under the planner's 0.12 street grade)",
            "grade of a ramp along the riser"),
    ]
}

/// The operator's values (see [`declared_defaults`]).
#[derive(Debug, Clone, PartialEq)]
pub struct ValleyParams {
    pub quay_freeboard_m: f64,
    pub quay_w_m: f64,
    pub riser_h_m: f64,
    pub treads_max: usize,
    pub tread_w_m: f64,
    pub tread_fall: f64,
    pub riser_slope: f64,
    pub bank_slope: f64,
    pub talus_slope: f64,
    pub cut_cap_m: f64,
    pub blend_m: f64,
    pub ref_radius_m: f64,
    pub ref_lo_m: f64,
    pub ref_hi_m: f64,
    pub bank_cap_m: f64,
    pub vary_wavelength_m: f64,
    pub vary_amp: f64,
    pub meander_m: f64,
    pub meander_wavelength_m: f64,
    pub cell_len_m: f64,
    pub spacing_m: f64,
    pub ramps: bool,
    pub ramp_spacing_m: f64,
    pub ramp_w_m: f64,
    pub ramp_grade: f64,
}

impl ValleyParams {
    /// The declared defaults overridden by `values` (by declared name); every value range-checked, then the
    /// cross-value rules ([`Self::check`]).
    pub fn from_values(values: &BTreeMap<String, f64>) -> Result<Self> {
        let decl = declared_defaults();
        if let Some(k) = values.keys().find(|k| !decl.iter().any(|d| d.name == k.as_str())) {
            return Err(Error::Invalid(format!("valley_terrace: unknown value {k:?}")));
        }
        let get = |name: &str| -> Result<f64> {
            let d = decl.iter().find(|d| d.name == name).ok_or_else(|| Error::Invalid(format!("undeclared {name}")))?;
            let v = values.get(name).copied().unwrap_or(d.v);
            if !(v.is_finite() && (d.min..=d.max).contains(&v)) {
                return Err(Error::Invalid(format!("valley_terrace: {name} = {v} outside {}..{}", d.min, d.max)));
            }
            Ok(v)
        };
        let p = Self {
            quay_freeboard_m: get("quay_freeboard_m")?,
            quay_w_m: get("quay_w_m")?,
            riser_h_m: get("riser_h_m")?,
            treads_max: get("treads_max")?.round() as usize,
            tread_w_m: get("tread_w_m")?,
            tread_fall: get("tread_fall")?,
            riser_slope: get("riser_slope")?,
            bank_slope: get("bank_slope")?,
            talus_slope: get("talus_slope")?,
            cut_cap_m: get("cut_cap_m")?,
            blend_m: get("blend_m")?,
            ref_radius_m: get("ref_radius_m")?,
            ref_lo_m: get("ref_lo_m")?,
            ref_hi_m: get("ref_hi_m")?,
            bank_cap_m: get("bank_cap_m")?,
            vary_wavelength_m: get("vary_wavelength_m")?,
            vary_amp: get("vary_amp")?,
            meander_m: get("meander_m")?,
            meander_wavelength_m: get("meander_wavelength_m")?,
            cell_len_m: get("cell_len_m")?,
            spacing_m: get("spacing_m")?,
            ramps: get("ramps")? >= 0.5,
            ramp_spacing_m: get("ramp_spacing_m")?,
            ramp_w_m: get("ramp_w_m")?,
            ramp_grade: get("ramp_grade")?,
        };
        p.check()?;
        Ok(p)
    }

    /// The declared defaults.
    ///
    /// # Panics
    /// Never: the declared defaults pass [`Self::check`] (tested).
    pub fn defaults() -> Self {
        Self::from_values(&BTreeMap::new()).expect("declared defaults are valid")
    }

    /// Fade of the weight outside the extent, metres: `max(blend_m, (bank_cap_m - quay_freeboard_m) / talus_slope)`
    /// (the deepest stair cut over the talus slope, the spec's blend rule applied to the stair).
    pub fn edge_blend_m(&self) -> f64 {
        self.blend_m.max((self.bank_cap_m - self.quay_freeboard_m) / self.talus_slope)
    }

    /// Cross-value rules: `blend_m >= cut_cap_m / talus_slope` (spec step 6), `ref_lo_m < ref_hi_m`, the narrowest
    /// first tread stays positive (`tread_w_m (1 - vary_amp) > meander_m`), `treads_max <= 8`, and a ramp's floor
    /// fits on the narrowest tread (`ramp_w_m < tread_w_m (1 - vary_amp) - meander_m`).
    pub fn check(&self) -> Result<()> {
        let bad = |m: String| Err(Error::Invalid(format!("valley_terrace: {m}")));
        if self.blend_m < self.cut_cap_m / self.talus_slope {
            return bad(format!(
                "blend_m {} < cut_cap_m / talus_slope = {}",
                self.blend_m,
                self.cut_cap_m / self.talus_slope
            ));
        }
        if self.ref_lo_m >= self.ref_hi_m {
            return bad("ref_lo_m must be under ref_hi_m".into());
        }
        if self.tread_w_m * (1.0 - self.vary_amp) <= self.meander_m {
            return bad("tread_w_m (1 - vary_amp) must exceed meander_m".into());
        }
        if self.ramp_w_m >= self.tread_w_m * (1.0 - self.vary_amp) - self.meander_m {
            return bad("ramp_w_m must fit on the narrowest tread".into());
        }
        if !(1..=MAXN).contains(&self.treads_max) {
            return bad(format!("treads_max {} outside 1..={MAXN}", self.treads_max));
        }
        Ok(())
    }
}

/// What the operator runs on. The guide sampler returns the guide height (metres) at a guide-frame point (x east,
/// y north, metres), as the runtime samples it.
pub struct ValleyInput<'a> {
    /// Name of the written stamp.
    pub name: String,
    pub guide: &'a (dyn Fn([f64; 2]) -> f64 + Sync),
    pub sea_level_m: f64,
    /// Centre of the town extent, guide frame.
    pub centre_m: [f64; 2],
    /// Radius of the town extent, metres.
    pub reach_m: f64,
    /// Radius of the town core (walls inside, earth banks outside), metres.
    pub core_radius_m: f64,
    pub seed: u64,
    /// Report metric: the street grade the "share under the grade" counts against (m/m).
    pub street_grade: f64,
    /// Report metric: dry ground counted for that share lies within this distance of the water, metres.
    pub share_within_m: f64,
}

/// Per-sample class of the stamp grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// Dry, not touched (weight from the blend only; the stamp holds the guide).
    Untouched,
    /// `h <= sea`: weight 0, never touched.
    Wet,
    Quay,
    Tread,
    /// Riser inside the core (a wall will stand in front).
    Wall,
    /// Earth-bank riser outside the core.
    Bank,
    /// Where neighbouring cells' stairs (different counts) cross-fade.
    Fade,
    /// Talus bank meeting the guide, or ground held by the cut cap.
    Natural,
    /// Cut by a ramp between two treads (its floor or its cut faces).
    Ramp,
}

/// Kind of a report polyline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineKind {
    Wall,
    Bank,
    Quay,
    /// A ramp's centreline, from its lower tread to its upper one (`riser` = the riser it climbs).
    Ramp,
}

/// One polyline of `valley.json`: the quay edge (`riser` 0), a riser's foot line (`riser` 1..), or a ramp's
/// centreline (bottom first).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub kind: LineKind,
    pub riser: u32,
    /// Guide-frame metres.
    pub points: Vec<[f64; 2]>,
}

/// Cut and fill of one bank (a dry component), on the final surface `mix(guide, stamp, w)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankReport {
    pub id: u32,
    /// Mean position of the bank's touched samples, guide frame.
    pub centroid_m: [f64; 2],
    pub cut_m3: f64,
    pub fill_m3: f64,
    /// Touched area, m2.
    pub area_m2: f64,
}

/// `valley.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValleyReport {
    pub format: u32,
    pub name: String,
    pub sea_level_m: f64,
    pub centre_m: [f64; 2],
    pub reach_m: f64,
    pub core_radius_m: f64,
    pub seed: u64,
    pub spacing_m: f64,
    pub cut_m3: f64,
    pub fill_m3: f64,
    pub banks: Vec<BankReport>,
    /// Share of dry ground inside the extent that is quay or tread.
    pub tread_share: f64,
    pub street_grade: f64,
    pub share_within_m: f64,
    /// Share of dry ground inside the extent and within `share_within_m` of the water under `street_grade`,
    /// on the guide and on the final surface.
    pub grade_share_before: f64,
    pub grade_share_after: f64,
    pub lines: Vec<Line>,
}

/// Per-sample diagnostics on the stamp grid (row 0 north), for tests and later slices.
#[derive(Debug, Clone, PartialEq)]
pub struct Diag {
    pub kind: Vec<Kind>,
    /// Guide height as sampled, metres (f32 like the stamp).
    pub guide_m: Vec<f32>,
    /// Top tread level of the sample's stair, metres; NaN without a stair.
    pub top_m: Vec<f32>,
    /// Distance from the wet edge, metres (NaN on wet samples).
    pub d_m: Vec<f32>,
    /// Dry component (`u32::MAX` on wet samples).
    pub bank: Vec<u32>,
}

/// The operator's result.
#[derive(Debug, Clone, PartialEq)]
pub struct ValleyTerrace {
    pub stamp: Stamp,
    pub report: ValleyReport,
    pub diag: Diag,
}

impl ValleyTerrace {
    /// Writes `stamps.json` + `stamps.f32` (this one stamp) and `valley.json` into `dir`.
    pub fn write(&self, dir: &Path) -> Result<()> {
        StampSet { stamps: vec![self.stamp.clone()] }.write(dir)?;
        std::fs::write(dir.join("valley.json"), serde_json::to_string_pretty(&self.report)?)?;
        Ok(())
    }
}

/// Merges `top` over `base` on their common lattice (same spacing, origins a whole number of samples apart), over
/// the union of both rectangles: `w_m = 1 - (1 - w_t)(1 - w_s)`, `h_m = (w_t h_t + (1 - w_t) w_s h_s) / w_m` (0 where
/// `w_m = 0`), so `mix(h, h_m, w_m) == mix(mix(h, h_s, w_s), h_t, w_t)` and the runtime detail scales by `1 - w_m`.
/// The merged stamp keeps `base`'s name.
pub fn merge_over(base: &Stamp, top: &Stamp) -> Result<Stamp> {
    let s = base.spacing_m;
    if top.spacing_m.to_bits() != s.to_bits() {
        return Err(Error::Invalid("merge_over: spacings differ".into()));
    }
    let off = |a: f64, b: f64| -> Result<i64> {
        let k = (a - b) / s;
        if (k - k.round()).abs() > 1e-6 {
            return Err(Error::Invalid("merge_over: origins are not on one lattice".into()));
        }
        Ok(k.round() as i64)
    };
    // Lattice column/row of each stamp's sample (0, 0) relative to base's.
    let (tc, tr) = (off(top.origin_m[0], base.origin_m[0])?, off(base.origin_m[1], top.origin_m[1])?);
    let c0 = tc.min(0);
    let r0 = tr.min(0);
    let c1 = (base.dims[0] as i64).max(tc + top.dims[0] as i64);
    let r1 = (base.dims[1] as i64).max(tr + top.dims[1] as i64);
    let (w, h) = ((c1 - c0) as usize, (r1 - r0) as usize);
    let at = |st: &Stamp, c: i64, r: i64| -> (f64, f64) {
        if c < 0 || r < 0 || c >= st.dims[0] as i64 || r >= st.dims[1] as i64 {
            return (0.0, 0.0);
        }
        let k = r as usize * st.dims[0] + c as usize;
        (f64::from(st.heights[k]), f64::from(st.weights[k]))
    };
    let mut heights = Vec::with_capacity(w * h);
    let mut weights = Vec::with_capacity(w * h);
    for r in r0..r1 {
        for c in c0..c1 {
            let (hs, ws) = at(base, c, r);
            let (ht, wt) = at(top, c - tc, r - tr);
            let wm = 1.0 - (1.0 - wt) * (1.0 - ws);
            let hm = if wm > 0.0 { (wt * ht + (1.0 - wt) * ws * hs) / wm } else { 0.0 };
            heights.push(hm as f32);
            weights.push((wm as f32).clamp(0.0, 1.0));
        }
    }
    Ok(Stamp {
        name: base.name.clone(),
        origin_m: [base.origin_m[0] + c0 as f64 * s, base.origin_m[1] - r0 as f64 * s],
        dims: [w, h],
        spacing_m: s,
        heights,
        weights,
    })
}

// ------------------------------------------------------------------------------------------------ stair

/// One stair along a perpendicular: positions `s` (metres from the wet edge) of each riser and tread.
#[derive(Debug, Clone, Copy)]
struct Stair {
    n: usize,
    q: f64,
    rise: f64,
    top: f64,
    qw: f64,
    qfall: f64,
    /// Riser `i` (0-based, rising to tread `i + 1`): start, run, height, kind.
    a: [f64; MAXN],
    run: [f64; MAXN],
    hr: [f64; MAXN],
    wall: [bool; MAXN],
    /// Tread `i + 1`: landward end and fall.
    e: [f64; MAXN],
    fall: [f64; MAXN],
    /// Landward end of the top tread, and its skew (the talus ramp beyond keeps its slope across the line).
    end: f64,
    end_skew: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Seg {
    Quay,
    Tread,
    Riser(bool),
    Beyond,
}

impl Stair {
    /// Target height at distance `s` (guide `g` there), and the segment.
    fn eval(&self, s: f64, g: f64, talus: f64) -> (f64, Seg) {
        if s < self.qw {
            return (self.q - self.qfall * (self.qw - s), Seg::Quay);
        }
        for i in 0..self.n {
            if s < self.a[i] + self.run[i] {
                let z_prev = self.q + i as f64 * self.rise;
                return (z_prev + self.hr[i] * (s - self.a[i]) / self.run[i], Seg::Riser(self.wall[i]));
            }
            if s < self.e[i] {
                let z = self.q + (i + 1) as f64 * self.rise;
                return (z - self.fall[i] * (self.e[i] - s), Seg::Tread);
            }
        }
        let t = talus * (s - self.end) / self.end_skew;
        (g.clamp(self.top - t, self.top + t), Seg::Beyond)
    }
}

/// Along-valley data of one foot (a wet sample nearest to some dry sample).
#[derive(Debug, Clone)]
struct Foot {
    /// The wet edge point: the foot's centre moved half a sample along the normal.
    edge: [f64; 2],
    /// Unit normal into dry ground.
    nrm: [f64; 2],
    /// Continuous tread count `(P - q) / riser height`.
    c: f64,
    /// Bank top `P` (capped), at least the quay level.
    p_eff: f64,
}

fn dist_m(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

/// The stair of `n` treads seen from a sample at `pos`, distance `s_here` from the edge along the unit direction
/// `dir` (the distance field's gradient there). Tread width and meander are value noise at the sample's edge point
/// `pos - dir s_here`, which is the same for every sample of one perpendicular (so slopes along it are exact) and
/// moves smoothly between perpendiculars. Where a riser or tread line is oblique to the perpendicular (the noise
/// varies along the edge), its run lengthens and its fall shortens by `hypot(1, d(line)/d(edge))`, so the slope
/// across the line keeps its value. Each riser's kind and slope come from its foot point `edge + dir a_i`.
#[allow(clippy::too_many_arguments)]
fn build_stair(p: &ValleyParams, inp: &ValleyInput, pos: [f64; 2], dir: [f64; 2], s_here: f64, p_eff: f64, n: usize) -> Stair {
    let q = inp.sea_level_m + p.quay_freeboard_m;
    let edge = [pos[0] - dir[0] * s_here, pos[1] - dir[1] * s_here];
    let tangent = [-dir[1], dir[0]];
    let noise = |at: [f64; 2], wl: f64, stream: u64| value_noise(at, wl, inp.seed ^ stream);
    let tw = p.tread_w_m * (1.0 + p.vary_amp * noise(edge, p.vary_wavelength_m, STREAM_TREAD));
    let m = p.meander_m * noise(edge, p.meander_wavelength_m, STREAM_MEANDER);
    // Derivatives (m/m) of the tread width and the meander along the edge.
    let along = |wl: f64, stream: u64, amp: f64| {
        let at = |t: f64| noise([edge[0] + tangent[0] * t, edge[1] + tangent[1] * t], wl, stream);
        amp * (at(FD_M) - at(-FD_M)) / (2.0 * FD_M)
    };
    let d_tw = along(p.vary_wavelength_m, STREAM_TREAD, p.tread_w_m * p.vary_amp);
    let d_m = along(p.meander_wavelength_m, STREAM_MEANDER, p.meander_m);
    let rise = ((p_eff - q) / n as f64).min(p.riser_h_m * (1.0 + p.vary_amp)).max(0.0);
    let mut st = Stair {
        n,
        q,
        rise,
        top: q + n as f64 * rise,
        qw: p.quay_w_m,
        qfall: p.tread_fall,
        a: [0.0; MAXN],
        run: [0.0; MAXN],
        hr: [0.0; MAXN],
        wall: [false; MAXN],
        e: [0.0; MAXN],
        fall: [0.0; MAXN],
        end: 0.0,
        end_skew: 1.0,
    };
    let mut prev = p.quay_w_m;
    for i in 0..n {
        let wi = if i == 0 { tw + m } else { tw };
        // Line positions: a_i = qw + sum_{k<i} (run_k + w_k), w_0 = tw + m; e_i = a_i + run_i + w_i (the runs'
        // own variation is second order: they hold only through the tiny fall term).
        let mi = if i == 0 { 0.0 } else { 1.0 };
        let skew_a = (mi * d_m + i as f64 * d_tw).hypot(1.0);
        let skew_e = (d_m + (i + 1) as f64 * d_tw).hypot(1.0);
        let fall = p.tread_fall.min(rise / wi) / skew_e;
        st.end_skew = skew_e;
        let hr = (rise - fall * wi).max(0.0);
        let start = [edge[0] + dir[0] * prev, edge[1] + dir[1] * prev];
        let r = dist_m(start, inp.centre_m);
        let wall = r <= inp.core_radius_m;
        // The slope eases from the wall's to the bank's over the last quarter cell inside the core, so a riser
        // crossing the core edge never shifts the risers above it in one step.
        let band = p.cell_len_m * FADE_SHARE;
        let slope = p.riser_slope + (p.bank_slope - p.riser_slope) * smootherstep((r - inp.core_radius_m + band) / band);
        let run = hr / slope * skew_a;
        st.a[i] = prev;
        st.run[i] = run;
        st.hr[i] = hr;
        st.wall[i] = wall;
        st.fall[i] = fall;
        st.e[i] = prev + run + wi;
        prev = st.e[i];
    }
    st.end = prev;
    st
}

/// Up to four (count, weight) components of a foot's stair, largest weight first.
type Comps = ([(usize, f64); 4], usize);

fn comps(foot: &Foot, bank: u32, counts: &BTreeMap<(u32, i64, i64), usize>, cell: f64) -> Comps {
    let own = |v: f64| (v / cell).floor() as i64;
    let own_n = counts[&(bank, own(foot.edge[0]), own(foot.edge[1]))];
    let axis = |v: f64| {
        let u = v / cell - 0.5;
        let i0 = u.floor();
        let t = smootherstep((u - i0 - (0.5 - FADE_SHARE / 2.0)) / FADE_SHARE);
        (i0 as i64, t)
    };
    let (ix, tx) = axis(foot.edge[0]);
    let (iy, ty) = axis(foot.edge[1]);
    let mut out: Comps = ([(0, 0.0); 4], 0);
    for (dx, wx) in [(0, 1.0 - tx), (1, tx)] {
        for (dy, wy) in [(0, 1.0 - ty), (1, ty)] {
            let wt = wx * wy;
            if wt <= 0.0 {
                continue;
            }
            let n = counts.get(&(bank, ix + dx, iy + dy)).copied().unwrap_or(own_n);
            if let Some(slot) = out.0[..out.1].iter_mut().find(|c| c.0 == n) {
                slot.1 += wt;
            } else {
                out.0[out.1] = (n, wt);
                out.1 += 1;
            }
        }
    }
    // Largest weight first (stable: ties keep lattice order).
    out.0[..out.1].sort_by(|a, b| b.1.total_cmp(&a.1));
    out
}

// ------------------------------------------------------------------------------------------------ fields

/// Exact squared-distance feature transform (Felzenszwalb & Huttenlocher 2012, as [`distance_transform`], keeping
/// the argmin): distance in samples to the nearest seed and that seed's index (`u32::MAX` without seeds).
fn feature_transform(seed: &[bool], w: usize, h: usize) -> (Vec<f64>, Vec<u32>) {
    // tunables: const (maths: "infinite" squared distance seed of the transform)
    const BIG: f64 = 1e20;
    let mut f = vec![BIG; w * h];
    let mut col = vec![u32::MAX; w * h];
    f.par_chunks_mut(w).zip(col.par_chunks_mut(w)).enumerate().for_each(|(j, (fr, cr))| {
        let row = &seed[j * w..(j + 1) * w];
        let mut last: Option<usize> = None;
        for i in 0..w {
            if row[i] {
                last = Some(i);
            }
            if let Some(l) = last {
                fr[i] = ((i - l) * (i - l)) as f64;
                cr[i] = l as u32;
            }
        }
        last = None;
        for i in (0..w).rev() {
            if row[i] {
                last = Some(i);
            }
            if let Some(l) = last {
                let d = ((l - i) * (l - i)) as f64;
                if d < fr[i] {
                    fr[i] = d;
                    cr[i] = l as u32;
                }
            }
        }
    });
    let cols: Vec<(Vec<f64>, Vec<usize>)> = (0..w)
        .into_par_iter()
        .map(|i| {
            let line: Vec<f64> = (0..h).map(|j| f[j * w + i]).collect();
            lower_envelope(&line)
        })
        .collect();
    let mut dist = vec![f64::INFINITY; w * h];
    let mut idx = vec![u32::MAX; w * h];
    for (i, (d2, arg)) in cols.iter().enumerate() {
        for j in 0..h {
            let r = arg[j];
            let c = col[r * w + i];
            if c != u32::MAX && d2[j] < BIG {
                dist[j * w + i] = d2[j].sqrt();
                idx[j * w + i] = (r * w + c as usize) as u32;
            }
        }
    }
    (dist, idx)
}

/// 1-D lower envelope of parabolas: squared distance and the argmin.
fn lower_envelope(src: &[f64]) -> (Vec<f64>, Vec<usize>) {
    let n = src.len();
    let mut v = vec![0usize; n];
    let mut z = vec![0.0f64; n + 1];
    let mut k = 0usize;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        loop {
            let p = v[k];
            let s = ((src[q] + (q * q) as f64) - (src[p] + (p * p) as f64)) / (2.0 * (q - p) as f64);
            if s <= z[k] && k > 0 {
                k -= 1;
            } else {
                k += 1;
                v[k] = q;
                z[k] = s;
                z[k + 1] = f64::INFINITY;
                break;
            }
        }
    }
    let mut out = vec![0.0; n];
    let mut arg = vec![0usize; n];
    k = 0;
    for q in 0..n {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        let dq = q as f64 - p as f64;
        out[q] = dq * dq + src[p];
        arg[q] = p;
    }
    (out, arg)
}

/// 4-connected components of `dry`, labelled in scan order (`u32::MAX` elsewhere).
fn components(dry: &[bool], w: usize, h: usize) -> Vec<u32> {
    let mut lab = vec![u32::MAX; w * h];
    let mut next = 0u32;
    let mut stack = Vec::new();
    for k0 in 0..w * h {
        if !dry[k0] || lab[k0] != u32::MAX {
            continue;
        }
        lab[k0] = next;
        stack.push(k0);
        while let Some(k) = stack.pop() {
            let (i, j) = (k % w, k / w);
            let mut visit = |kk: usize| {
                if dry[kk] && lab[kk] == u32::MAX {
                    lab[kk] = next;
                    stack.push(kk);
                }
            };
            if i > 0 {
                visit(k - 1);
            }
            if i + 1 < w {
                visit(k + 1);
            }
            if j > 0 {
                visit(k - w);
            }
            if j + 1 < h {
                visit(k + w);
            }
        }
        next += 1;
    }
    lab
}

/// Central-difference slope magnitude (m/m) of a grid; NaN on the border.
fn slope_grid(z: &[f64], w: usize, h: usize, s: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        if j == 0 || j + 1 >= h {
            return;
        }
        for i in 1..w.saturating_sub(1) {
            let gx = (z[j * w + i + 1] - z[j * w + i - 1]) / (2.0 * s);
            let gy = (z[(j + 1) * w + i] - z[(j - 1) * w + i]) / (2.0 * s);
            row[i] = gx.hypot(gy);
        }
    });
    out
}

// ------------------------------------------------------------------------------------------------ operator

/// Runs the operator (module docs). Deterministic: same input, same stamp bits.
#[allow(clippy::too_many_lines)]
pub fn valley_terrace(inp: &ValleyInput, p: &ValleyParams) -> Result<ValleyTerrace> {
    p.check()?;
    let finite = [inp.sea_level_m, inp.centre_m[0], inp.centre_m[1], inp.reach_m, inp.core_radius_m];
    if finite.iter().any(|v| !v.is_finite()) || inp.reach_m <= 0.0 || inp.core_radius_m < 0.0 {
        return Err(Error::Invalid("valley_terrace: extent must be finite, reach_m > 0, core_radius_m >= 0".into()));
    }
    if !(inp.street_grade > 0.0 && inp.share_within_m > 0.0) {
        return Err(Error::Invalid("valley_terrace: street_grade and share_within_m must be > 0".into()));
    }
    let s = p.spacing_m;
    let sea = inp.sea_level_m;
    let q = sea + p.quay_freeboard_m;
    // Stamp grid: extent plus blend, on the world lattice of `spacing_m` (so stamps on it merge exactly).
    let half = inp.reach_m + p.edge_blend_m();
    let x0 = ((inp.centre_m[0] - half) / s).floor() * s;
    let y0 = ((inp.centre_m[1] + half) / s).ceil() * s;
    let x1 = ((inp.centre_m[0] + half) / s).ceil() * s;
    let y1 = ((inp.centre_m[1] - half) / s).floor() * s;
    let w = ((x1 - x0) / s).round() as usize + 1;
    let h = ((y0 - y1) / s).round() as usize + 1;
    // Analysis grid: padded so the bank-top band of every stamp sample's foot lies inside.
    let pad = ((p.ref_hi_m + p.ref_radius_m) / s).ceil() as usize;
    let (aw, ah) = (w + 2 * pad, h + 2 * pad);
    if aw.checked_mul(ah).is_none_or(|n| n >= 1 << 28) {
        return Err(Error::Invalid(format!("valley_terrace: analysis grid {aw}x{ah} too large")));
    }
    let (ax0, ay0) = (x0 - pad as f64 * s, y0 + pad as f64 * s);
    let apos = |k: usize| [ax0 + (k % aw) as f64 * s, ay0 - (k / aw) as f64 * s];

    // 1. Guide and wet/dry.
    let mut g = vec![0.0f64; aw * ah];
    g.par_chunks_mut(aw).enumerate().for_each(|(j, row)| {
        for (i, v) in row.iter_mut().enumerate() {
            *v = (inp.guide)([ax0 + i as f64 * s, ay0 - j as f64 * s]);
        }
    });
    if g.iter().any(|v| !v.is_finite()) {
        return Err(Error::Invalid("valley_terrace: guide returned a non-finite height".into()));
    }
    let wet: Vec<bool> = g.iter().map(|&v| v <= sea).collect();
    let dry: Vec<bool> = wet.iter().map(|&v| !v).collect();

    // 2. Distance (metres from the wet edge, the edge half a sample outside the last wet sample) and feet.
    let (dd, foot_of) = feature_transform(&wet, aw, ah);
    let dw = distance_transform(&dry, aw, ah);
    let sd: Vec<f64> = dd.iter().zip(&dw).map(|(a, b)| a - b).collect();
    let d_m: Vec<f64> = dd.iter().map(|v| (v - 0.5) * s).collect();
    let bank = components(&dry, aw, ah);

    // 3. Bank-top band: row prefix sums of guide height and count.
    let in_band = |k: usize| dry[k] && (p.ref_lo_m..=p.ref_hi_m).contains(&d_m[k]);
    let mut ps = vec![0.0f64; (aw + 1) * ah];
    let mut pc = vec![0u32; (aw + 1) * ah];
    ps.par_chunks_mut(aw + 1).zip(pc.par_chunks_mut(aw + 1)).enumerate().for_each(|(j, (sr, cr))| {
        for i in 0..aw {
            let k = j * aw + i;
            let b = in_band(k);
            sr[i + 1] = sr[i] + if b { g[k] } else { 0.0 };
            cr[i + 1] = cr[i] + u32::from(b);
        }
    });
    let disc_mean = |ci: i64, cj: i64| -> Option<f64> {
        let r = (p.ref_radius_m / s).floor() as i64;
        let (mut sum, mut cnt) = (0.0, 0u64);
        for dy in -r..=r {
            let j = cj + dy;
            if j < 0 || j >= ah as i64 {
                continue;
            }
            let hw = ((r * r - dy * dy) as f64).sqrt().floor() as i64;
            let lo = (ci - hw).clamp(0, aw as i64) as usize;
            let hi = (ci + hw + 1).clamp(0, aw as i64) as usize;
            if lo >= hi {
                continue;
            }
            let base = j as usize * (aw + 1);
            sum += ps[base + hi] - ps[base + lo];
            cnt += u64::from(pc[base + hi] - pc[base + lo]);
        }
        (cnt > 0).then(|| sum / cnt as f64)
    };

    // 4. Feet of the stamp's dry samples, each with its stairs.
    let sidx = |i: usize, j: usize| (j + pad) * aw + i + pad;
    let mut feet_ids: Vec<u32> = (0..h)
        .flat_map(|j| (0..w).map(move |i| (i, j)))
        .filter_map(|(i, j)| {
            let k = sidx(i, j);
            (dry[k] && foot_of[k] != u32::MAX).then_some(foot_of[k])
        })
        .collect();
    feet_ids.sort_unstable();
    feet_ids.dedup();
    let feet: Vec<Option<Foot>> = feet_ids
        .par_iter()
        .map(|&f| {
            let f = f as usize;
            let (fi, fj) = ((f % aw) as i64, (f / aw) as i64);
            let at = |i: i64, j: i64| sd[(j.clamp(0, ah as i64 - 1) as usize) * aw + i.clamp(0, aw as i64 - 1) as usize];
            let k = NORMAL_STENCIL as i64;
            let (gx, gy) = (at(fi + k, fj) - at(fi - k, fj), at(fi, fj - k) - at(fi, fj + k));
            let len = gx.hypot(gy);
            if !(len.is_finite() && len > 0.0) {
                return None;
            }
            let nrm = [gx / len, gy / len];
            let c0 = apos(f);
            let edge = [c0[0] + nrm[0] * 0.5 * s, c0[1] + nrm[1] * 0.5 * s];
            let mid = f64::midpoint(p.ref_lo_m, p.ref_hi_m);
            let probe = [edge[0] + nrm[0] * mid, edge[1] + nrm[1] * mid];
            let pi = ((probe[0] - ax0) / s).round() as i64;
            let pj = ((ay0 - probe[1]) / s).round() as i64;
            let bank_top = disc_mean(pi, pj)?.min(sea + p.bank_cap_m);
            let p_eff = bank_top.max(q);
            let rh = p.riser_h_m * (1.0 + p.vary_amp * value_noise(edge, p.vary_wavelength_m, inp.seed ^ STREAM_RISER));
            Some(Foot { edge, nrm, c: (p_eff - q) / rh, p_eff })
        })
        .collect();
    let foot = |k: usize| -> Option<&Foot> {
        let f = foot_of[k];
        if !dry[k] || f == u32::MAX {
            return None;
        }
        feet_ids.binary_search(&f).ok().and_then(|x| feet[x].as_ref())
    };

    // 5. Tread count per (bank, cell): ceil of the largest continuous count among the cell's feet.
    let cell = p.cell_len_m;
    let mut cmax: BTreeMap<(u32, i64, i64), f64> = BTreeMap::new();
    for j in 0..h {
        for i in 0..w {
            let k = sidx(i, j);
            if let Some(ft) = foot(k) {
                let key = (bank[k], (ft.edge[0] / cell).floor() as i64, (ft.edge[1] / cell).floor() as i64);
                let e = cmax.entry(key).or_insert(f64::NEG_INFINITY);
                *e = e.max(ft.c);
            }
        }
    }
    let counts: BTreeMap<(u32, i64, i64), usize> =
        cmax.into_iter().map(|(k, c)| (k, (c.ceil().max(1.0) as usize).min(p.treads_max))).collect();

    // Direction of an analysis sample's perpendicular: the distance field's gradient (central differences over the
    // normal stencil, which smooths a digitised edge; the direction to its foot where it vanishes).
    let sample_dir = |k: usize| -> [f64; 2] {
        let (here, fc) = (apos(k), apos(foot_of[k] as usize));
        let (ai, aj) = (k % aw, k / aw);
        let dd_at = |i: usize, j: usize| dd[j.min(ah - 1) * aw + i.min(aw - 1)];
        let st = NORMAL_STENCIL;
        let gx = dd_at(ai + st, aj) - dd_at(ai.saturating_sub(st), aj);
        let gy = dd_at(ai, aj.saturating_sub(st)) - dd_at(ai, aj + st);
        let (vx, vy) = if gx.hypot(gy).is_finite() && gx.hypot(gy) > 0.0 { (gx, gy) } else { (here[0] - fc[0], here[1] - fc[1]) };
        let vl = vx.hypot(vy);
        [vx / vl, vy / vl]
    };

    // 6. Stair heights per stamp sample.
    let n = w * h;
    let mut kind = vec![Kind::Untouched; n];
    let mut hp = vec![0.0f64; n];
    let mut top = vec![f64::NAN; n];
    let mut gs = vec![0.0f64; n];
    let mut sb = vec![u32::MAX; n];
    let mut sdm = vec![f64::NAN; n];
    let talus = p.talus_slope;
    kind.par_chunks_mut(w)
        .zip(hp.par_chunks_mut(w))
        .zip(top.par_chunks_mut(w))
        .zip(gs.par_chunks_mut(w))
        .zip(sb.par_chunks_mut(w).zip(sdm.par_chunks_mut(w)))
        .enumerate()
        .for_each(|(j, ((((kr, hr), tr), gr), (br, dr)))| {
            for i in 0..w {
                let k = sidx(i, j);
                gr[i] = g[k];
                hr[i] = g[k];
                if wet[k] {
                    kr[i] = Kind::Wet;
                    continue;
                }
                br[i] = bank[k];
                dr[i] = d_m[k];
                let Some(ft) = foot(k) else { continue };
                let (cs, len) = comps(ft, bank[k], &counts, cell);
                let (here, dir) = (apos(k), sample_dir(k));
                let (mut hh, mut tt, mut lo, mut hi) = (0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY);
                let (mut seg, mut z0) = (Seg::Beyond, 0.0);
                for (x, &(cn, wt)) in cs[..len].iter().enumerate() {
                    let st = build_stair(p, inp, here, dir, d_m[k], ft.p_eff, cn);
                    let (z, sg) = st.eval(d_m[k], g[k], talus);
                    if x == 0 {
                        (seg, z0) = (sg, z);
                    }
                    hh += wt * z;
                    tt += wt * st.top;
                    lo = lo.min(z);
                    hi = hi.max(z);
                }
                // One surface (all components agree): its exact value, so untouched ground stays bit-exact.
                if hi - lo <= SAME_EPS_M {
                    hh = z0;
                }
                hr[i] = hh;
                tr[i] = tt;
                kr[i] = if hi - lo > SAME_EPS_M {
                    Kind::Fade
                } else {
                    match seg {
                        Seg::Quay => Kind::Quay,
                        Seg::Tread => Kind::Tread,
                        Seg::Riser(true) => Kind::Wall,
                        Seg::Riser(false) => Kind::Bank,
                        Seg::Beyond if hh.to_bits() == g[k].to_bits() => Kind::Untouched,
                        Seg::Beyond => Kind::Natural,
                    }
                };
                if kr[i] == Kind::Untouched {
                    hr[i] = g[k];
                }
            }
        });

    // 7. Cut cap: ground more than `cut_cap_m` above the top tread holds `g - cut_cap_m`; a talus cone grown from
    //    it inside its bank (two-pass 5x5 chamfer) is the natural bank below it.
    let mut cone: Vec<f64> = (0..n)
        .map(|k| if top[k].is_finite() && gs[k] > top[k] + p.cut_cap_m { gs[k] - p.cut_cap_m } else { f64::NEG_INFINITY })
        .collect();
    let (r2, r5) = (2f64.sqrt(), 5f64.sqrt());
    let fwd: [(i64, i64, f64); 8] =
        [(-1, 0, 1.0), (-1, -1, r2), (0, -1, 1.0), (1, -1, r2), (-2, -1, r5), (-1, -2, r5), (1, -2, r5), (2, -1, r5)];
    for pass in 0..2 {
        let sign = if pass == 0 { 1 } else { -1 };
        for t in 0..n {
            let k = if pass == 0 { t } else { n - 1 - t };
            if sb[k] == u32::MAX {
                continue;
            }
            let (i, j) = ((k % w) as i64, (k / w) as i64);
            for &(dx, dy, len) in &fwd {
                let (ni, nj) = (i + sign * dx, j + sign * dy);
                if ni < 0 || nj < 0 || ni >= w as i64 || nj >= h as i64 {
                    continue;
                }
                let nk = nj as usize * w + ni as usize;
                if sb[nk] != sb[k] {
                    continue;
                }
                let c = cone[nk] - talus * len * s;
                if c > cone[k] {
                    cone[k] = c;
                }
            }
        }
    }
    for k in 0..n {
        // The natural bank never stands above the guide: it only stops the stair cutting, it fills nothing.
        let nat = cone[k].min(gs[k]);
        if top[k].is_finite() && nat > hp[k] {
            hp[k] = nat;
            kind[k] = Kind::Natural;
        }
    }

    // 7b. Ramps (SP-1c), cut into the stair: never raise it, never change what is touched (so the weights hold).
    let ramp_lines = if p.ramps {
        let foot_by_id = |f: u32| feet_ids.binary_search(&f).ok().and_then(|x| feet[x].as_ref());
        let stamp_ak = |k: usize| sidx(k % w, k / w);
        let (lines, cuts) = cut_ramps(&RampCtx {
            inp,
            p,
            aw,
            counts: &counts,
            kind: &kind,
            bank: &sb,
            foot_of: &|k| foot_of[stamp_ak(k)],
            foot_by_id: &foot_by_id,
            sample: &|k| {
                let ak = stamp_ak(k);
                (apos(ak), sample_dir(ak), d_m[ak])
            },
            at: &|pt| {
                let (fi, fj) = (((pt[0] - x0) / s).round(), ((y0 - pt[1]) / s).round());
                (fi >= 0.0 && fj >= 0.0 && (fi as usize) < w && (fj as usize) < h).then(|| fj as usize * w + fi as usize)
            },
        }, &hp);
        for (k, z) in cuts {
            hp[k] = z;
            kind[k] = Kind::Ramp;
        }
        lines
    } else {
        Vec::new()
    };

    // 8. Weights.
    let touched: Vec<bool> = kind.iter().map(|k| !matches!(k, Kind::Untouched | Kind::Wet)).collect();
    let dist_t = distance_transform(&touched, w, h);
    let mut heights = vec![0.0f32; n];
    let mut weights = vec![0.0f32; n];
    for k in 0..n {
        let pos = [x0 + (k % w) as f64 * s, y0 - (k / w) as f64 * s];
        heights[k] = (if touched[k] { hp[k] } else { gs[k] }) as f32;
        if kind[k] == Kind::Wet {
            continue;
        }
        let w_t = 1.0 - smootherstep(dist_t[k] * s / p.blend_m);
        let w_e = 1.0 - smootherstep((dist_m(pos, inp.centre_m) - inp.reach_m) / p.edge_blend_m());
        weights[k] = (w_t * w_e) as f32;
    }
    let stamp = Stamp { name: inp.name.clone(), origin_m: [x0, y0], dims: [w, h], spacing_m: s, heights, weights };

    // 9. Report.
    let guide32: Vec<f32> = gs.iter().map(|&v| v as f32).collect();
    let fin = final_surface(&stamp, &guide32);
    let g64: Vec<f64> = guide32.iter().map(|&v| f64::from(v)).collect();
    let mut per_bank: BTreeMap<u32, (f64, f64, f64, [f64; 2], u64)> = BTreeMap::new();
    let area = s * s;
    for k in 0..n {
        if sb[k] == u32::MAX {
            continue;
        }
        let e = per_bank.entry(sb[k]).or_insert((0.0, 0.0, 0.0, [0.0; 2], 0));
        let dz = fin[k] - g64[k];
        e.0 += (-dz).max(0.0) * area;
        e.1 += dz.max(0.0) * area;
        if touched[k] {
            e.2 += area;
            e.3[0] += x0 + (k % w) as f64 * s;
            e.3[1] += y0 - (k / w) as f64 * s;
            e.4 += 1;
        }
    }
    let banks: Vec<BankReport> = per_bank
        .into_iter()
        .filter(|(_, e)| e.4 > 0)
        .map(|(id, e)| BankReport {
            id,
            centroid_m: [e.3[0] / e.4 as f64, e.3[1] / e.4 as f64],
            cut_m3: e.0,
            fill_m3: e.1,
            area_m2: e.2,
        })
        .collect();
    let (sl0, sl1) = (slope_grid(&g64, w, h, s), slope_grid(&fin, w, h, s));
    let (mut dry_in, mut treads, mut cnt, mut ok0, mut ok1) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for k in 0..n {
        let pos = [x0 + (k % w) as f64 * s, y0 - (k / w) as f64 * s];
        if sb[k] == u32::MAX || dist_m(pos, inp.centre_m) > inp.reach_m {
            continue;
        }
        dry_in += 1;
        treads += u64::from(matches!(kind[k], Kind::Quay | Kind::Tread));
        if sdm[k] <= inp.share_within_m && sl0[k].is_finite() {
            cnt += 1;
            ok0 += u64::from(sl0[k] <= inp.street_grade);
            ok1 += u64::from(sl1[k] <= inp.street_grade);
        }
    }
    let share = |a: u64, b: u64| if b == 0 { 0.0 } else { a as f64 / b as f64 };
    let grid = Grid0 { x0, y0, w, h, s };
    let mut lines =
        trace_lines(inp, p, &grid, &kind, &sb, &touched, &|i, j| foot(sidx(i, j)), &|i, j| foot_of[sidx(i, j)], aw, &counts);
    lines.extend(ramp_lines);
    let report = ValleyReport {
        format: REPORT_FORMAT,
        name: inp.name.clone(),
        sea_level_m: sea,
        centre_m: inp.centre_m,
        reach_m: inp.reach_m,
        core_radius_m: inp.core_radius_m,
        seed: inp.seed,
        spacing_m: s,
        cut_m3: banks.iter().map(|b| b.cut_m3).sum(),
        fill_m3: banks.iter().map(|b| b.fill_m3).sum(),
        banks,
        tread_share: share(treads, dry_in),
        street_grade: inp.street_grade,
        share_within_m: inp.share_within_m,
        grade_share_before: share(ok0, cnt),
        grade_share_after: share(ok1, cnt),
        lines,
    };
    let diag = Diag {
        kind,
        guide_m: guide32,
        top_m: top.iter().map(|&v| v as f32).collect(),
        d_m: sdm.iter().map(|&v| v as f32).collect(),
        bank: sb,
    };
    Ok(ValleyTerrace { stamp, report, diag })
}

/// The runtime's blend at the stamp's grid points, `g + (h_s - g) w` in f64 from the f32 values (guide `g` per
/// sample, same layout).
pub fn final_surface(stamp: &Stamp, guide: &[f32]) -> Vec<f64> {
    stamp
        .heights
        .iter()
        .zip(&stamp.weights)
        .zip(guide)
        .map(|((&hs, &ws), &g)| {
            let g = f64::from(g);
            g + (f64::from(hs) - g) * f64::from(ws)
        })
        .collect()
}

/// The stamp grid's placement.
struct Grid0 {
    x0: f64,
    y0: f64,
    w: usize,
    h: usize,
    s: f64,
}

impl Grid0 {
    fn nearest(&self, pt: [f64; 2]) -> Option<usize> {
        let i = ((pt[0] - self.x0) / self.s).round();
        let j = ((self.y0 - pt[1]) / self.s).round();
        (i >= 0.0 && j >= 0.0 && (i as usize) < self.w && (j as usize) < self.h).then(|| j as usize * self.w + i as usize)
    }
}

/// Polylines of each bank: the feet of its touched samples chained along the edge (8-neighbours, 4-neighbours
/// first), every [`LINE_STEP_M`], offset along the normal to the quay edge and each riser's foot line; a line breaks
/// where its kind changes, the riser is absent or overridden (natural bank, fade) or the point leaves the extent.
#[allow(clippy::too_many_arguments)]
fn trace_lines<'f>(
    inp: &ValleyInput,
    p: &ValleyParams,
    grid: &Grid0,
    kind: &[Kind],
    bank: &[u32],
    touched: &[bool],
    foot: &dyn Fn(usize, usize) -> Option<&'f Foot>,
    foot_id: &dyn Fn(usize, usize) -> u32,
    aw: usize,
    counts: &BTreeMap<(u32, i64, i64), usize>,
) -> Vec<Line> {
    // Feet per bank, with one stamp sample each to look the foot up by.
    let mut sets: BTreeMap<u32, BTreeMap<u32, (usize, usize)>> = BTreeMap::new();
    for j in 0..grid.h {
        for i in 0..grid.w {
            let k = j * grid.w + i;
            if touched[k] && foot(i, j).is_some() {
                sets.entry(bank[k]).or_default().entry(foot_id(i, j)).or_insert((i, j));
            }
        }
    }
    let step = (LINE_STEP_M / grid.s).round().max(1.0) as usize;
    let mut out = Vec::new();
    for (b, set) in sets {
        for chain in foot_chains(set.keys().copied(), aw) {
            if chain.len() < 2 {
                continue;
            }
            let mut picks: Vec<u32> = chain.iter().step_by(step).copied().collect();
            if chain.len() % step != 1 {
                picks.push(chain[chain.len() - 1]);
            }
            let mut open: Vec<Option<Line>> = vec![None; MAXN + 1];
            for fid in picks {
                let (si, sj) = set[&fid];
                let Some(ft) = foot(si, sj) else { continue };
                let (cs, _) = comps(ft, b, counts, p.cell_len_m);
                let cn = cs[0].0;
                let along = |t: f64| [ft.edge[0] + ft.nrm[0] * t, ft.edge[1] + ft.nrm[1] * t];
                let on_ray = |t: f64| build_stair(p, inp, along(t), ft.nrm, t, ft.p_eff, cn);
                let st = on_ray(p.quay_w_m);
                let kind_at = |pt: [f64; 2]| grid.nearest(pt).map(|k| kind[k]);
                let inside = |pt: [f64; 2]| dist_m(pt, inp.centre_m) <= inp.reach_m;
                for (slot, line) in open.iter_mut().enumerate() {
                    let here = if slot == 0 {
                        let pt = along(0.0);
                        (inside(pt) && kind_at(along(0.5 * st.qw)) == Some(Kind::Quay)).then_some((LineKind::Quay, pt))
                    } else if slot <= st.n {
                        // The riser line: the fixed point `t = a_i(along(t))` (|da/dt| < 1, so it converges).
                        let mut t = st.a[slot - 1];
                        for _ in 0..RAY_ITERS {
                            t = on_ray(t).a[slot - 1];
                        }
                        let sr = on_ray(t);
                        let (a0, run, is_wall) = (sr.a[slot - 1], sr.run[slot - 1], sr.wall[slot - 1]);
                        let (lk, want) = if is_wall { (LineKind::Wall, Kind::Wall) } else { (LineKind::Bank, Kind::Bank) };
                        let pt = along(a0);
                        (run > 0.0 && inside(pt) && kind_at(along(a0 + 0.5 * run)) == Some(want)).then_some((lk, pt))
                    } else {
                        None
                    };
                    let same = matches!((&here, line.as_ref()), (Some((lk, _)), Some(l)) if l.kind == *lk);
                    if same {
                        if let (Some((_, pt)), Some(l)) = (here, line.as_mut()) {
                            l.points.push(pt);
                        }
                        continue;
                    }
                    if let Some(l) = line.take().filter(|l| l.points.len() >= 2) {
                        out.push(l);
                    }
                    if let Some((lk, pt)) = here {
                        *line = Some(Line { kind: lk, riser: slot as u32, points: vec![pt] });
                    }
                }
            }
            out.extend(open.into_iter().flatten().filter(|l| l.points.len() >= 2));
        }
    }
    out
}

/// Feet of one bank chained along the edge (8-neighbours on the analysis grid, 4-neighbours first): from the smallest
/// id left, walked forward then backward, until every foot is in a chain.
fn foot_chains(ids: impl IntoIterator<Item = u32>, aw: usize) -> Vec<Vec<u32>> {
    let mut left: BTreeSet<u32> = ids.into_iter().collect();
    let mut out = Vec::new();
    while let Some(&start) = left.iter().next() {
        left.remove(&start);
        let walk = |left: &mut BTreeSet<u32>, from: u32| {
            let mut chain = Vec::new();
            let mut cur = from as usize;
            loop {
                let (ci, cj) = ((cur % aw) as i64, (cur / aw) as i64);
                let next = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)]
                    .iter()
                    .filter(|&&(dx, dy)| ci + dx >= 0 && ci + dx < aw as i64 && cj + dy >= 0)
                    .map(|&(dx, dy)| ((cj + dy) * aw as i64 + ci + dx) as u32)
                    .find(|nb| left.contains(nb));
                let Some(nb) = next else { break };
                left.remove(&nb);
                chain.push(nb);
                cur = nb as usize;
            }
            chain
        };
        let fwd = walk(&mut left, start);
        let rev = walk(&mut left, start);
        out.push(rev.into_iter().rev().chain(std::iter::once(start)).chain(fwd).collect());
    }
    out
}

// ------------------------------------------------------------------------------------------------ ramps

/// What the ramp pass reads (stamp-grid indices `k`; the closures map them to the analysis grid).
struct RampCtx<'a> {
    inp: &'a ValleyInput<'a>,
    p: &'a ValleyParams,
    aw: usize,
    counts: &'a BTreeMap<(u32, i64, i64), usize>,
    kind: &'a [Kind],
    bank: &'a [u32],
    foot_of: &'a (dyn Fn(usize) -> u32 + Sync),
    foot_by_id: &'a (dyn Fn(u32) -> Option<&'a Foot> + Sync),
    /// Position, perpendicular direction and distance from the edge of a stamp sample.
    sample: &'a (dyn Fn(usize) -> ([f64; 2], [f64; 2], f64) + Sync),
    /// The stamp sample nearest a position, if inside the stamp.
    at: &'a (dyn Fn([f64; 2]) -> Option<usize> + Sync),
}

/// One foot of a chain, seen by the ramps: per riser `i`, the ramp centreline point (the middle of the floor strip
/// on the upper tread, `a_i + run_i + ramp_w_m / 2` along the foot's normal), its arc length `v` along that riser's
/// line (all smoothed along the chain) and the along direction there: the feet's edge tangents (normal turned a
/// quarter), oriented with the chain and smoothed (the cell bounds of [`centreline_v`]).
#[derive(Clone, Copy)]
struct ChainFoot {
    n: usize,
    /// Per riser: a ramp may run here (one count, no fade, the riser rises, the point inside the extent and on the
    /// built stair).
    ok: [bool; MAXN],
    rise: f64,
    pt: [[f64; 2]; MAXN],
    v: [f64; MAXN],
    tan: [[f64; 2]; MAXN],
}

/// One ramp: riser `i` (0-based, from tread `i` to `i + 1`) of a stair of `n` treads on chain `chain`, rising from
/// arc length `vb` along the riser line in direction `dir` (+1 / -1) over `len` metres; it touches chain feet
/// `lo..=hi` (its centreline polyline, end faces included).
struct Ramp {
    chain: usize,
    i: usize,
    n: usize,
    vb: f64,
    dir: f64,
    len: f64,
    lo: usize,
    hi: usize,
    /// Bounding box (min, max) of the samples the ramp may cut: its centreline, padded by its corridor and faces.
    bbox: [[f64; 2]; 2],
}

/// A fitted ramp before it is stored: riser, bottom and top chain index, touched chain range, length, direction.
type Fit = (usize, usize, usize, usize, usize, f64, f64);

/// Ramps cut into the risers (spec `site-prep.md` "Ramps (SP-1c)"): per bank, the chain of feet carries an arc length
/// along each riser line; **columns** of ramps every `ramp_spacing_m` along riser 0's line, each a switchback: the
/// ramp of riser 0 climbs forward from its bottom over `rise / ramp_grade`, the ramp of riser 1 starts where it
/// arrived and climbs back, and so on up the stair (successive ramps never line up into one straight cut). A
/// column's start slides forward by [`LINE_STEP_M`] until its first ramp fits (no fade, one count, inside the
/// extent, end faces included); a column stops at the first ramp that does not fit.
///
/// A ramp's surface at a sample of its stair: the floor `z_i + rise clamp(u / len, 0, 1)` (`u` the arc length from
/// the bottom, `z_i` the lower tread's level, so it joins both treads without a step) across the corridor from the
/// riser's foot `a_i` to `a_i + run_i + ramp_w_m` (the riser band plus a strip of the upper tread), rising outside
/// the corridor at the riser's own slope (cut faces at the ends and on the landward side). The target becomes
/// `min(stair, ramp)`: a ramp only cuts. Returns the ramp centrelines and the cut samples with their new heights.
#[allow(clippy::too_many_lines)]
fn cut_ramps(c: &RampCtx, hp: &[f64]) -> (Vec<Line>, Vec<(usize, f64)>) {
    let (p, inp) = (c.p, c.inp);
    let wr = p.ramp_w_m;
    // Chains of the touched samples' feet per bank (the report polylines' chains).
    let mut sets: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for k in 0..c.kind.len() {
        if !matches!(c.kind[k], Kind::Untouched | Kind::Wet) && c.bank[k] != u32::MAX {
            let f = (c.foot_of)(k);
            if f != u32::MAX && (c.foot_by_id)(f).is_some() {
                sets.entry(c.bank[k]).or_default().insert(f);
            }
        }
    }
    let mut chains: Vec<(u32, Vec<ChainFoot>)> = Vec::new();
    for (&b, set) in &sets {
        for ids in foot_chains(set.iter().copied(), c.aw) {
            chains.push((b, chain_feet(c, b, &ids)));
        }
    }
    // Placement: switchback columns.
    let mut ramps: Vec<Ramp> = Vec::new();
    let mut lines = Vec::new();
    let step = (LINE_STEP_M / p.spacing_m).round().max(1.0) as usize;
    let mut buckets: BTreeMap<(u32, i64, i64), Vec<usize>> = BTreeMap::new();
    for (ci, (b, cf)) in chains.iter().enumerate() {
        let m = cf.len();
        if m < 2 {
            continue;
        }
        let mut vb = cf[0].v[0];
        let mut j0 = 0;
        loop {
            while j0 < m && cf[j0].v[0] < vb {
                j0 += 1;
            }
            if j0 >= m {
                break;
            }
            let mut column: Vec<Fit> = Vec::new();
            let (mut jb, mut dir) = (j0, 1.0);
            for i in 0..MAXN {
                let Some((jt, lo, hi, len)) = fit_ramp(p, cf, i, jb, dir) else { break };
                column.push((i, jb, jt, lo, hi, len, dir));
                (jb, dir) = (jt, -dir);
            }
            if column.is_empty() {
                vb += LINE_STEP_M;
                continue;
            }
            for (i, jb, jt, lo, hi, len, dir) in column {
                let r = ramps.len();
                // Beyond the corridor (at most `run + ramp_w_m` wide, under `rise / bank_slope + ramp_w_m`) the
                // faces rise above the stair within the run (`rise / riser slope`, under `rise / bank_slope`).
                let pad = 2.0 * (len * p.ramp_grade / p.bank_slope + wr);
                let mut bbox = [[f64::INFINITY; 2], [f64::NEG_INFINITY; 2]];
                for f in &cf[lo..=hi] {
                    for (d, &x) in f.pt[i].iter().enumerate() {
                        bbox[0][d] = bbox[0][d].min(x - pad);
                        bbox[1][d] = bbox[1][d].max(x + pad);
                    }
                }
                let cell = |x: f64| (x / RAMP_BUCKET_M).floor() as i64;
                for by in cell(bbox[0][1])..=cell(bbox[1][1]) {
                    for bx in cell(bbox[0][0])..=cell(bbox[1][0]) {
                        buckets.entry((*b, bx, by)).or_default().push(r);
                    }
                }
                ramps.push(Ramp { chain: ci, i, n: cf[jb].n, vb: cf[jb].v[i], dir, len, lo, hi, bbox });
                let path: Vec<usize> = if jt >= jb { (jb..=jt).collect() } else { (jt..=jb).rev().collect() };
                let mut pts: Vec<[f64; 2]> = path.iter().step_by(step).map(|&x| cf[x].pt[i]).collect();
                if !(path.len() - 1).is_multiple_of(step) {
                    pts.push(cf[jt].pt[i]);
                }
                lines.push(Line { kind: LineKind::Ramp, riser: i as u32 + 1, points: pts });
            }
            vb = cf[j0].v[0] + p.ramp_spacing_m;
        }
    }
    // Cut: every stair sample of a bank inside one of its ramps' boxes, on the sample's own stair (as step 6 builds
    // it). The domain is geometric, not the feet a ramp touches: on a curve, neighbouring samples' feet (nearest
    // edge points) jump along the digitised edge, and a foot-based domain would leave uncut cliffs at its border.
    let cuts: Vec<(usize, f64)> = (0..c.kind.len())
        .into_par_iter()
        .filter_map(|k| {
            if !matches!(c.kind[k], Kind::Tread | Kind::Wall | Kind::Bank) {
                return None;
            }
            let b = c.bank[k];
            let (here, dir, t) = (c.sample)(k);
            let list = buckets.get(&(b, (here[0] / RAMP_BUCKET_M).floor() as i64, (here[1] / RAMP_BUCKET_M).floor() as i64))?;
            let inside = |r: &Ramp| (0..2).all(|d| here[d] >= r.bbox[0][d] && here[d] <= r.bbox[1][d]);
            if !list.iter().any(|&r| inside(&ramps[r])) {
                return None;
            }
            let ft = (c.foot_by_id)((c.foot_of)(k))?;
            let (cs, len) = comps(ft, b, c.counts, p.cell_len_m);
            if len > 1 {
                return None;
            }
            let st = build_stair(p, inp, here, dir, t, ft.p_eff, cs[0].0);
            let mut z = hp[k];
            for &r in list {
                let rp = &ramps[r];
                let i = rp.i;
                if !inside(rp) || rp.n != st.n || i >= st.n || st.run[i] <= 0.0 {
                    continue;
                }
                let cf = &chains[rp.chain].1;
                // Arc length of the sample: its own position projected on the ramp's smoothed centreline (a
                // foot's arc length plus an offset would jump between neighbouring samples of different feet on a
                // curve).
                let v = centreline_v(cf, i, rp.lo, rp.hi, here);
                let u = (v - rp.vb) * rp.dir;
                let du = (-u).max(u - rp.len).max(0.0);
                let floor = st.q + i as f64 * st.rise + st.rise * (u / rp.len).clamp(0.0, 1.0);
                let strip_end = st.a[i] + st.run[i] + wr;
                let dt = (t - strip_end).max(st.a[i] - t).max(0.0);
                let side = st.hr[i] / st.run[i];
                z = z.min(floor + side * du.hypot(dt));
            }
            (z < hp[k] - SAME_EPS_M).then_some((k, z))
        })
        .collect();
    (lines, cuts)
}

/// The ramp view of a chain of feet of bank `b` (see [`ChainFoot`]).
fn chain_feet(c: &RampCtx, b: u32, ids: &[u32]) -> Vec<ChainFoot> {
    let p = c.p;
    let mut cf: Vec<ChainFoot> = ids
        .iter()
        .map(|&f| {
            let ft = (c.foot_by_id)(f).expect("chained feet exist");
            let (cs, len) = comps(ft, b, c.counts, p.cell_len_m);
            let n = cs[0].0;
            let at = [ft.edge[0] + ft.nrm[0] * p.quay_w_m, ft.edge[1] + ft.nrm[1] * p.quay_w_m];
            let st = build_stair(p, c.inp, at, ft.nrm, p.quay_w_m, ft.p_eff, n);
            let mut out =
                ChainFoot { n, ok: [false; MAXN], rise: st.rise, pt: [ft.edge; MAXN], v: [0.0; MAXN], tan: [[-ft.nrm[1], ft.nrm[0]]; MAXN] };
            for i in 0..n {
                let t = st.a[i] + st.run[i] + 0.5 * p.ramp_w_m;
                out.pt[i] = [ft.edge[0] + ft.nrm[0] * t, ft.edge[1] + ft.nrm[1] * t];
                // ... and the stair stands there (not ground held by the cut cap, not a fade).
                let built = (c.at)(out.pt[i]).is_some_and(|k| matches!(c.kind[k], Kind::Tread | Kind::Wall | Kind::Bank));
                out.ok[i] = len == 1 && st.hr[i] > 0.0 && dist_m(out.pt[i], c.inp.centre_m) <= c.inp.reach_m && built;
            }
            out
        })
        .collect();
    // Arc length and tangent per riser line (a foot without riser `i` carries the previous foot's point).
    // The points are smoothed along the chain first: a foot's normal carries the digitised edge's jitter out to the
    // riser line (about a metre at 100 m), which would inflate the arc length and so steepen the ramps.
    for i in 0..MAXN {
        for j in 1..cf.len() {
            if i >= cf[j].n {
                cf[j].pt[i] = cf[j - 1].pt[i];
            }
        }
        let raw: Vec<[f64; 2]> = cf.iter().map(|f| f.pt[i]).collect();
        for j in 0..cf.len() {
            let (a, z) = (j.saturating_sub(SMOOTH_FEET), (j + SMOOTH_FEET).min(cf.len() - 1));
            let m = (z - a + 1) as f64;
            let sum = raw[a..=z].iter().fold([0.0, 0.0], |s, q| [s[0] + q[0], s[1] + q[1]]);
            cf[j].pt[i] = [sum[0] / m, sum[1] / m];
        }
        // Along direction: the feet's edge tangents (normal turned a quarter) oriented with the chain, then smoothed
        // like the points, so the cell bounds of [`centreline_v`] stay on the feet's own perpendiculars on a straight
        // edge (a sample's floor level is constant across the corridor) and turn smoothly on a curve.
        for j in 0..cf.len() {
            let (a, z) = (cf[j.saturating_sub(1)].pt[i], cf[(j + 1).min(cf.len() - 1)].pt[i]);
            let e = cf[j].tan[i];
            if e[0] * (z[0] - a[0]) + e[1] * (z[1] - a[1]) < 0.0 {
                cf[j].tan[i] = [-e[0], -e[1]];
            }
        }
        let raw: Vec<[f64; 2]> = cf.iter().map(|f| f.tan[i]).collect();
        for j in 0..cf.len() {
            let (a, z) = (j.saturating_sub(SMOOTH_FEET), (j + SMOOTH_FEET).min(cf.len() - 1));
            let sum = raw[a..=z].iter().fold([0.0, 0.0], |s, q| [s[0] + q[0], s[1] + q[1]]);
            let l = sum[0].hypot(sum[1]);
            if l > 0.0 {
                cf[j].tan[i] = [sum[0] / l, sum[1] / l];
            }
        }
        // Arc length: each step's advance along the mean of its two along directions. A greedy chain of 8-neighbour
        // feet zigzags across the edge, and the smoothed points keep a little of it; a polyline length would count
        // that zigzag as length (a ramp up to ~6 % short and steep), the advance does not.
        for j in 1..cf.len() {
            let (a, z) = (cf[j - 1].pt[i], cf[j].pt[i]);
            let t = [cf[j - 1].tan[i][0] + cf[j].tan[i][0], cf[j - 1].tan[i][1] + cf[j].tan[i][1]];
            let l = t[0].hypot(t[1]);
            let adv = if l > 0.0 { ((z[0] - a[0]) * t[0] + (z[1] - a[1]) * t[1]) / l } else { dist_m(a, z) };
            cf[j].v[i] = cf[j - 1].v[i] + adv;
        }
    }
    cf
}

/// Arc length of point `q` along the smoothed centreline of riser `i` over chain feet `lo..=hi`, continuous in `q`:
/// each segment's cell is bounded by the lines through its two vertices across their tangents, and inside it `q`
/// sits at the fraction `da / (da - db)` of the segment (`da`, `db` its signed distances along those tangents), so
/// both cells meeting at a vertex give that vertex's arc length on their shared bound. Among several cells (the
/// inside of a curve, far from it) the nearest segment wins; beyond the ends the end tangents extend the line; in no
/// cell at all (bounds crossing), the nearest vertex's tangent does.
fn centreline_v(cf: &[ChainFoot], i: usize, lo: usize, hi: usize, q: [f64; 2]) -> f64 {
    let along = |x: usize| {
        let (a, t) = (cf[x].pt[i], cf[x].tan[i]);
        (q[0] - a[0]) * t[0] + (q[1] - a[1]) * t[1]
    };
    let d_lo = along(lo);
    if lo == hi || d_lo < 0.0 {
        return cf[lo].v[i] + d_lo;
    }
    let d_hi = along(hi);
    if d_hi > 0.0 {
        return cf[hi].v[i] + d_hi;
    }
    let x = (lo..=hi).min_by(|&a, &b| dist_m(q, cf[a].pt[i]).total_cmp(&dist_m(q, cf[b].pt[i]))).unwrap_or(lo);
    let mut best = (f64::INFINITY, cf[x].v[i] + along(x));
    let mut da = d_lo;
    for x in lo..hi {
        let db = along(x + 1);
        if da >= 0.0 && db <= 0.0 {
            let s = if da - db > 0.0 { da / (da - db) } else { 0.0 };
            let (a, b) = (cf[x].pt[i], cf[x + 1].pt[i]);
            let on = [a[0] + s * (b[0] - a[0]), a[1] + s * (b[1] - a[1])];
            let d = dist_m(q, on);
            if d < best.0 {
                best = (d, cf[x].v[i] + s * (cf[x + 1].v[i] - cf[x].v[i]));
            }
        }
        da = db;
    }
    best.1
}

/// The ramp of riser `i` from chain foot `jb` climbing in direction `dir`: (top foot, first and last touched chain
/// index, length), or `None` when it does not fit (off the chain, or a touched foot of another count, in a fade or
/// outside the extent).
fn fit_ramp(p: &ValleyParams, cf: &[ChainFoot], i: usize, jb: usize, dir: f64) -> Option<(usize, usize, usize, f64)> {
    let (m, n) = (cf.len(), cf[jb].n);
    if i >= n || !cf[jb].ok[i] {
        return None;
    }
    // The rise varies along the edge (each foot's own top level), and with it the lower tread's level: a foot's
    // floor is `q + rise (i + u / len)`. The length grows until that floor climbs no steeper than the grade between
    // the touched feet (and at least the largest rise over the grade); the touched range grows with the length, so
    // the two are iterated to a fixed point.
    let floor = |x: usize, len: f64| cf[x].rise * (i as f64 + ((cf[x].v[i] - cf[jb].v[i]) * dir / len).clamp(0.0, 1.0));
    let mut len = cf[jb].rise / p.ramp_grade;
    for _ in 0..RAMP_FIT_ITERS {
        let rise = len * p.ramp_grade;
        // The end faces reach `rise / slope` beyond the run (the bank slope is the gentlest), plus the floor's width
        // (samples off the centreline reach it through the tangent offset).
        let margin = rise / p.bank_slope + p.ramp_w_m;
        let target = cf[jb].v[i] + dir * len;
        let mut jt = jb;
        while (cf[jt].v[i] - target) * dir < 0.0 {
            jt = if dir > 0.0 { (jt + 1 < m).then_some(jt + 1)? } else { jt.checked_sub(1)? };
        }
        let (va, vz) = (cf[jb].v[i].min(cf[jt].v[i]) - margin, cf[jb].v[i].max(cf[jt].v[i]) + margin);
        let mut lo = jb.min(jt);
        while cf[lo].v[i] > va {
            lo = lo.checked_sub(1)?;
        }
        let mut hi = jb.max(jt);
        while cf[hi].v[i] < vz {
            hi = (hi + 1 < m).then_some(hi + 1)?;
        }
        if !(lo..=hi).all(|x| cf[x].n == n && cf[x].ok[i]) {
            return None;
        }
        let mut need = cf[lo..=hi].iter().map(|f| f.rise).fold(rise, f64::max) / p.ramp_grade;
        for x in lo..hi {
            let dv = (cf[x + 1].v[i] - cf[x].v[i]).abs();
            let g = (floor(x + 1, len) - floor(x, len)).abs() / dv;
            if dv > 0.0 && g > p.ramp_grade {
                need = need.max(len * g / p.ramp_grade);
            }
        }
        if need <= len + SAME_EPS_M {
            return Some((jt, lo, hi, len));
        }
        len = need;
    }
    None
}
