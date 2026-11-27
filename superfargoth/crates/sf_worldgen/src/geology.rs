//! **Geology map** (LF-1, `docs/specs/landforms.md` "Data model", "Pipeline" step 2): which rock unit lies under
//! every guide sample, how hard it is, and where its beds run, so placed landforms, runtime detail, texture banding
//! and rockgen's strata all read the same rock.
//!
//! - **Units** ([`RockUnit`], adapter data in a TOML [`GeologyTable`]): hardness, erodibility, bedding (thickness
//!   range, share of soft beds, dip and how it dips), repose angles, outcrop flag and material layer ids.
//! - **Unit map:** the seamless smooth-Voronoi blend of [`RockMap::unit_weights`] (jittered lattice of feature points
//!   every `unit_cell_m`, domain-warped; Worley, SIGGRAPH 1996). Each Voronoi cell draws its unit by its hash from the
//!   cumulative shares of the region holding the cell's centre (the guide's `regions.png`; regions without shares
//!   use the table's default shares), so a unit is a whole cell, never cut by a region border. `unit.u8` is the
//!   nearest cell's unit; `hardness.u8` and `datum.f32` blend over the cells within `unit_blend_m`, so both are
//!   continuous across unit contacts.
//! - **Datum** (a bed boundary sits at `z - datum = sum of beds`): per cell `n` with centre `c_n`, `L` the
//!   `surface_lowpass_m` low-pass of the base field (Gaussian-weighted local linear fit, [`lowpass_local_linear`],
//!   so planes and cone flanks pass exactly):
//!   - `Surface`: `L(p)`: beds parallel to the smoothed ground; strata show only where something cuts across them.
//!   - `Fixed(azimuth)`: `L(c_n) - tan(dip) (p - c_n) . d`, `d` the dip direction (compass azimuth): a tilted stack
//!     anchored at its cell, so each cell is a fault block with its own offset.
//!   - `Radial(centre)`: `L(c_n) - tan(dip) (|p - centre| - |c_n - centre|)`: beds dip away from a centre.
//! - **Beds** ([`bed`], [`GeologyMap::bed_at`]): with `s = z - datum`, mean thickness `t = (min + max) / 2` and
//!   `a = (max - min) / (max + min)`, boundary `i` sits at `b_i = t (i + a (u_i - 1/2))`, `u_i` the bed hash
//!   `pcg3d(unit, i, seed)` (Jarzynski & Olano, JCGT 9(3), 2020). `a < 1` keeps the boundaries ordered and every
//!   bed in `[min, max]`, and the bed holding `s` is found in O(1) (`floor(s / t)` plus at most one step), so the
//!   WGSL twin costs a few hashes. A bed is soft (hardness `soft_bed_ratio x` the unit's) with probability
//!   `soft_bed_share`, by a second hash word.
//!
//! **Products** (`<out>/geology/`, one value per guide sample, little-endian, row-major, row 0 north): `unit.u8`,
//! `hardness.u8` (/255), `datum.f32`, `cliff.u8`, `talus.u8`, `human.u8`, `lip.f16`, `feature.u16`, plus
//! `geology.json` (units, declared values) and `meta.json` (dims, frame, per-file and total blake3). LF-1 fills
//! unit, hardness and datum; cliff, talus, human and feature are 0 and lip is `+lip_band_m` (no feature) until the
//! placement slices write them. `datum` is f32, not the spec's f16: at 2-4 km heights f16 steps by 2 m, which would
//! step every bed boundary by 2 m.
//!
//! Deterministic: integer hashes only, rows map in parallel as pure functions of position.

use std::collections::BTreeMap;
use std::path::Path;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::filter::{Declared, pcg3d};
use crate::guide::RegionMap;
use crate::lithology::{RockMap, UnitWeight};
use crate::measure::{lowpass_local_linear, sigma_for_wavelength};
use crate::perturb::lattice;
use crate::{Error, Result};

/// Products format written to `geology/meta.json`.
// tunables: const (format version)
pub const FORMAT: u32 = 1;

/// Declared values of the geology map (TOML `[values]`, `scrib-worldgen geology --set`).
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
        d("seed", 7.0, "id", 0.0, 4_294_967_295.0, "Guess", "hash seed of the unit cells and the beds"),
        d("unit_cell_m", 1400.0, "m", 100.0, 20_000.0, "Guess", "rock-unit Voronoi cell spacing (as cliffs unit_cell_m)"),
        d("unit_jitter", 0.85, "ratio", 0.0, 1.0, "Guess", "feature-point jitter inside its lattice cell (as cliffs)"),
        d("unit_warp_m", 320.0, "m", 0.0, 5000.0, "Guess", "domain warp amplitude of unit borders (as cliffs unit_warp_m)"),
        d("unit_warp_wavelength_m", 1260.0, "m", 50.0, 50_000.0, "Guess", "domain warp wavelength (cliffs: 0.9 unit cell)"),
        d("unit_blend_m", 260.0, "m", 0.0, 10_000.0, "Guess", "width over which hardness and datum blend across unit contacts (capped at half a cell)"),
        d("surface_lowpass_m", 2000.0, "m", 200.0, 20_000.0, "Guess", "half-power wavelength of the low-passed surface the datum follows (landforms.md Pipeline 2)"),
        d("soft_bed_ratio", 0.5, "ratio", 0.0, 0.5, "Guess", "hardness of a soft bed as a share of its unit's hardness (soft beds stay below 0.5)"),
        d("lip_band_m", 120.0, "m", 10.0, 1000.0, "Guess", "clamp of the signed lip distance (landforms.md Tunables)"),
    ]
}

/// Checked values of [`declared_defaults`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeologyParams {
    pub seed: u32,
    pub unit_cell_m: f64,
    pub unit_jitter: f64,
    pub unit_warp_m: f64,
    pub unit_warp_wavelength_m: f64,
    pub unit_blend_m: f64,
    pub surface_lowpass_m: f64,
    pub soft_bed_ratio: f64,
    pub lip_band_m: f64,
}

impl GeologyParams {
    /// Reads every value of `d` (ranges checked).
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |n: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == n)
                .ok_or_else(|| Error::Invalid(format!("geology: missing value {n}")))?;
            if !(p.min..=p.max).contains(&p.v) {
                return Err(Error::Invalid(format!("geology: {n} = {} outside [{}, {}]", p.v, p.min, p.max)));
            }
            Ok(p.v)
        };
        Ok(Self {
            seed: get("seed")? as u32,
            unit_cell_m: get("unit_cell_m")?,
            unit_jitter: get("unit_jitter")?,
            unit_warp_m: get("unit_warp_m")?,
            unit_warp_wavelength_m: get("unit_warp_wavelength_m")?,
            unit_blend_m: get("unit_blend_m")?,
            surface_lowpass_m: get("surface_lowpass_m")?,
            soft_bed_ratio: get("soft_bed_ratio")?,
            lip_band_m: get("lip_band_m")?,
        })
    }
}

/// How a unit's beds dip.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DipMode {
    /// Dip toward a compass azimuth (degrees clockwise from north).
    Fixed { azimuth_deg: f64 },
    /// Dip away from a guide-frame point (volcano flanks).
    Radial { centre_m: [f64; 2] },
    /// Beds parallel to the low-passed surface (`dip_deg` unused).
    Surface,
}

/// Material layer ids of a unit (indices into the adapter's terrain layer table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnitLayers {
    pub top: u16,
    pub face: u16,
    pub scree: u16,
}

/// One rock unit (adapter data).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RockUnit {
    pub name: String,
    /// [0, 1].
    pub hardness: f64,
    /// Erodibility multiplier (as [`crate::lithology::RockType::k`]).
    #[serde(default = "one")]
    pub k: f64,
    #[serde(default)]
    pub layered: bool,
    /// Bed thickness range `[min, max]`, metres (`0 < min <= max`).
    #[serde(default = "default_bed")]
    pub bed_m: [f64; 2],
    /// Share of beds that are soft (hardness below 0.5).
    #[serde(default)]
    pub soft_bed_share: f64,
    #[serde(default)]
    pub dip_deg: f64,
    pub dip_mode: DipMode,
    #[serde(default = "default_repose_hard")]
    pub repose_hard_deg: f64,
    #[serde(default = "default_repose_soft")]
    pub repose_soft_deg: f64,
    /// Blocks (fractured outcrop detail) allowed.
    #[serde(default)]
    pub outcrop: bool,
    #[serde(default)]
    pub layers: UnitLayers,
}

fn one() -> f64 {
    1.0
}
fn default_bed() -> [f64; 2] {
    // tunables: const (data default of a unit that omits bed_m; the adapter's table sets real values)
    [4.0, 12.0]
}
fn default_repose_hard() -> f64 {
    // tunables: const (data default: cap-rock face repose, landforms.md Tunables face_deg 70)
    70.0
}
fn default_repose_soft() -> f64 {
    // tunables: const (data default: scree repose, landforms.md Tunables talus_deg 33)
    33.0
}

/// Unit shares of one region.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegionShares {
    /// Unit name -> share (normalised).
    pub shares: BTreeMap<String, f64>,
}

/// The adapter's geology table (TOML: `[[unit]]`, `[shares]`, `[region."<legend name>"] shares`, `[values]`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeologyTable {
    #[serde(rename = "unit")]
    pub units: Vec<RockUnit>,
    /// Default unit shares (regions without their own).
    pub shares: BTreeMap<String, f64>,
    #[serde(default)]
    pub region: BTreeMap<String, RegionShares>,
    /// Overrides of [`declared_defaults`].
    #[serde(default)]
    pub values: BTreeMap<String, f64>,
}

impl GeologyTable {
    /// Parses and checks a TOML table.
    pub fn from_toml(text: &str) -> Result<Self> {
        let t: Self = toml::from_str(text)?;
        t.check()?;
        Ok(t)
    }

    /// Unit names unique, at most 255 units, valid beds and shares naming known units.
    pub fn check(&self) -> Result<()> {
        let bad = |m: String| Err(Error::Invalid(format!("geology: {m}")));
        if self.units.is_empty() || self.units.len() > 255 {
            return bad(format!("{} units (1-255)", self.units.len()));
        }
        for (i, u) in self.units.iter().enumerate() {
            if self.units[..i].iter().any(|o| o.name == u.name) {
                return bad(format!("unit {} twice", u.name));
            }
            if !(0.0..=1.0).contains(&u.hardness) || !(0.0..=1.0).contains(&u.soft_bed_share) {
                return bad(format!("unit {}: hardness and soft_bed_share are in [0, 1]", u.name));
            }
            if u.layered && !(u.bed_m[0] > 0.0 && u.bed_m[1] >= u.bed_m[0]) {
                return bad(format!("unit {}: bed_m needs 0 < min <= max", u.name));
            }
            if !(0.0..90.0).contains(&u.dip_deg) {
                return bad(format!("unit {}: dip_deg in [0, 90)", u.name));
            }
        }
        let shares = std::iter::once(("default", &self.shares))
            .chain(self.region.iter().map(|(n, r)| (n.as_str(), &r.shares)));
        for (n, s) in shares {
            if let Some(u) = s.keys().find(|k| !self.units.iter().any(|u| &u.name == *k)) {
                return bad(format!("shares {n}: unknown unit {u}"));
            }
            if s.values().any(|&v| v < 0.0) || s.values().sum::<f64>() <= 0.0 {
                return bad(format!("shares {n}: need a positive total and no negative share"));
            }
        }
        Ok(())
    }

    /// [`declared_defaults`] with the table's `[values]` applied (source `Recipe`).
    pub fn declared(&self) -> Result<Vec<Declared>> {
        let mut d = declared_defaults();
        for (k, &v) in &self.values {
            let p = d
                .iter_mut()
                .find(|p| p.name == k)
                .ok_or_else(|| Error::Invalid(format!("geology: unknown value {k}")))?;
            p.v = v;
            p.source = "Recipe".into();
        }
        Ok(d)
    }

    /// Cumulative shares per legend index of `regions` for the regions the table names (`None`: the default).
    fn region_cumulative(&self, regions: Option<&RegionMap>) -> Vec<Option<Vec<f64>>> {
        let mut out = vec![None; 256];
        if let Some(rm) = regions {
            for (name, rs) in &self.region {
                if let Some(v) = rm.find(name) {
                    out[usize::from(v)] = Some(self.cumulative(&rs.shares));
                }
            }
        }
        out
    }

    /// Cumulative unit shares (in unit order) of one share map.
    fn cumulative(&self, shares: &BTreeMap<String, f64>) -> Vec<f64> {
        let mut acc = 0.0;
        self.units
            .iter()
            .map(|u| {
                acc += shares.get(&u.name).copied().unwrap_or(0.0);
                acc
            })
            .collect()
    }
}

/// One bed at a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bed {
    /// Bed number counted from the datum (0 = the bed just above it); 0 for an unlayered unit.
    pub index: i64,
    /// [0, 1].
    pub hardness: f64,
    /// Position inside the bed, 0 at its base, toward 1 at its top.
    pub frac: f64,
}

/// u32 -> [0, 1) with 24 bits (as the runtime filter's hash).
#[inline]
fn unit01(v: u32) -> f64 {
    f64::from(v >> 8) * (1.0 / 16_777_216.0)
}

/// The bed of `unit` (table index `unit_index`) at stratigraphic height `s = z - datum` (module docs, "Beds").
pub fn bed(unit: &RockUnit, unit_index: u8, s: f64, seed: u32, soft_bed_ratio: f64) -> Bed {
    if !unit.layered {
        return Bed {
            index: 0,
            hardness: unit.hardness,
            frac: 0.0,
        };
    }
    let [lo, hi] = unit.bed_m;
    let t = f64::midpoint(lo, hi);
    let a = (hi - lo) / (hi + lo);
    let hash = |i: i64| pcg3d(u32::from(unit_index), i as u32, seed);
    let b = |i: i64| t * (i as f64 + a * (unit01(hash(i)[0]) - 0.5));
    let mut i = (s / t).floor() as i64;
    if s < b(i) {
        i -= 1;
    } else if s >= b(i + 1) {
        i += 1;
    }
    let (b0, b1) = (b(i), b(i + 1));
    let soft = unit01(hash(i)[1]) < unit.soft_bed_share;
    Bed {
        index: i,
        hardness: if soft { unit.hardness * soft_bed_ratio } else { unit.hardness },
        frac: ((s - b0) / (b1 - b0)).clamp(0.0, 1.0),
    }
}

/// IEEE 754 binary16 bits of `v`, round to nearest even (`lip.f16`).
pub fn f16_bits(v: f32) -> u16 {
    let x = v.to_bits();
    let sign = ((x >> 16) & 0x8000) as u16;
    let exp = ((x >> 23) & 0xff) as i32;
    let man = x & 0x7f_ffff;
    if exp == 0xff {
        return sign | 0x7c00 | if man != 0 { 0x200 } else { 0 };
    }
    let e = exp - 127 + 15;
    if e >= 0x1f {
        return sign | 0x7c00;
    }
    let round = |r: u32, rem: u32, half: u32| if rem > half || (rem == half && r & 1 == 1) { r + 1 } else { r };
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let m = man | 0x80_0000;
        let shift = (14 - e) as u32;
        let r = round(m >> shift, m & ((1 << shift) - 1), 1 << (shift - 1));
        return sign | r as u16;
    }
    // A carry out of the mantissa rounds into the exponent (and to infinity past the largest finite value).
    sign | round(((e as u32) << 10) | (man >> 13), man & 0x1fff, 0x1000) as u16
}

/// Value of binary16 bits `b`.
pub fn f16_value(b: u16) -> f32 {
    let s = if b & 0x8000 != 0 { -1.0 } else { 1.0 };
    let e = i32::from((b >> 10) & 0x1f);
    let m = f32::from(b & 0x3ff);
    match e {
        0 => s * m * 2f32.powi(-24),
        0x1f if m == 0.0 => s * f32::INFINITY,
        0x1f => f32::NAN,
        _ => s * (1.0 + m / 1024.0) * 2f32.powi(e - 15),
    }
}

/// The geology map of one guide (module docs).
#[derive(Debug, Clone, PartialEq)]
pub struct GeologyMap {
    pub w: usize,
    pub h: usize,
    pub spacing_m: f64,
    /// Guide-frame metres of sample (0, 0) (x east, y north, row 0 north).
    pub origin_m: [f64; 2],
    pub params: GeologyParams,
    pub units: Vec<RockUnit>,
    /// Nearest cell's unit (index into `units`).
    pub unit: Vec<u8>,
    /// Blended unit hardness x 255.
    pub hardness: Vec<u8>,
    /// Strata datum, metres.
    pub datum: Vec<f32>,
    pub cliff: Vec<u8>,
    pub talus: Vec<u8>,
    pub human: Vec<u8>,
    /// Signed distance to the nearest feature lip, binary16 metres.
    pub lip: Vec<u16>,
    /// 0 = none, else feature index + 1.
    pub feature: Vec<u16>,
}

/// Guide-frame point of sample (col, row).
#[inline]
fn sample_m(origin: [f64; 2], spacing: f64, col: f64, row: f64) -> [f64; 2] {
    [origin[0] + col * spacing, origin[1] - row * spacing]
}

/// Bilinear value of `v` (a `w` x `h` grid) at fractional sample (col, row), clamped to the grid.
fn bilinear(v: &[f64], w: usize, h: usize, col: f64, row: f64) -> f64 {
    let c = col.clamp(0.0, (w - 1) as f64);
    let r = row.clamp(0.0, (h - 1) as f64);
    let (c0, r0) = (c.floor() as usize, r.floor() as usize);
    let (c1, r1) = ((c0 + 1).min(w - 1), (r0 + 1).min(h - 1));
    let (tx, ty) = (c - c0 as f64, r - r0 as f64);
    let top = v[r0 * w + c0] + (v[r0 * w + c1] - v[r0 * w + c0]) * tx;
    let bot = v[r1 * w + c0] + (v[r1 * w + c1] - v[r1 * w + c0]) * tx;
    top + (bot - top) * ty
}

/// The base field a map is built on (the guide's heights and frame).
#[derive(Debug, Clone, Copy)]
pub struct BaseGrid<'a> {
    pub heights: &'a [f64],
    pub w: usize,
    pub h: usize,
    pub spacing_m: f64,
    pub origin_m: [f64; 2],
    pub regions: Option<&'a RegionMap>,
}

/// Datum of unit `u` of the cell centred at `c` at point `p` (module docs, "Datum"): `low_p` is the low-passed
/// surface at `p`, `low_c` gives it at `c`.
fn unit_datum(u: &RockUnit, p: [f64; 2], c: [f64; 2], low_p: f64, low_c: impl Fn() -> f64) -> f64 {
    let tan = u.dip_deg.to_radians().tan();
    match u.dip_mode {
        DipMode::Surface => low_p,
        DipMode::Fixed { azimuth_deg } => {
            let (sa, ca) = azimuth_deg.to_radians().sin_cos();
            low_c() - tan * ((p[0] - c[0]) * sa + (p[1] - c[1]) * ca)
        }
        DipMode::Radial { centre_m } => {
            let r = (p[0] - centre_m[0]).hypot(p[1] - centre_m[1]);
            let rc = (c[0] - centre_m[0]).hypot(c[1] - centre_m[1]);
            low_c() - tan * (r - rc)
        }
    }
}

/// Builds the geology map of `base` from `table` (module docs; products other than unit, hardness and datum
/// start empty).
pub fn build(base: &BaseGrid<'_>, table: &GeologyTable, params: &GeologyParams) -> Result<GeologyMap> {
    table.check()?;
    let BaseGrid {
        heights,
        w,
        h,
        spacing_m: dx,
        origin_m: origin,
        regions,
    } = *base;
    if heights.len() != w * h || w < 2 || h < 2 {
        return Err(Error::Invalid("geology: base grid size".into()));
    }
    let default_cum = table.cumulative(&table.shares);
    let region_cum = table.region_cumulative(regions);
    let cum_at = |q: [f64; 2]| -> &[f64] {
        let Some(rm) = regions else { return &default_cum };
        let px = ((q[0] - origin[0] + 0.5 * dx) / rm.cell_m).floor() as isize;
        let py = ((origin[1] - q[1] + 0.5 * dx) / rm.cell_m).floor() as isize;
        region_cum[usize::from(rm.at_clamped(px, py))]
            .as_deref()
            .unwrap_or(&default_cum)
    };
    let seed = params.seed;
    let cell = params.unit_cell_m;
    let unit_of = |ix: i64, iy: i64| -> u8 {
        let c = [(ix as f64 + 0.5) * cell, (iy as f64 + 0.5) * cell];
        let cum = cum_at(c);
        let total = cum.last().copied().unwrap_or(0.0);
        let u = f64::midpoint(lattice(ix, iy, u64::from(seed) ^ 0x6e17_a5c3), 1.0) * total;
        cum.iter().position(|&v| u < v).unwrap_or(cum.len() - 1) as u8
    };
    let low = lowpass_local_linear(
        heights,
        &vec![true; w * h],
        w,
        h,
        sigma_for_wavelength(params.surface_lowpass_m) / dx,
    );
    let low_at = |q: [f64; 2]| bilinear(&low, w, h, (q[0] - origin[0]) / dx, (origin[1] - q[1]) / dx);
    let map = RockMap {
        cell_m: cell,
        jitter: params.unit_jitter,
        warp_m: params.unit_warp_m,
        warp_wavelength_m: params.unit_warp_wavelength_m,
        edge_band_m: 0.0,
        edge_k: 1.0,
        seed: u64::from(seed) ^ 0x3a2f_1d09,
    };
    let units = &table.units;
    let n = w * h;
    let mut unit = vec![0u8; n];
    let mut hardness = vec![0u8; n];
    let mut datum = vec![0f32; n];
    unit.par_chunks_mut(w)
        .zip(hardness.par_chunks_mut(w))
        .zip(datum.par_chunks_mut(w))
        .enumerate()
        .for_each(|(row, ((ur, hr), dr))| {
            let mut buf = [UnitWeight::default(); 25];
            for col in 0..w {
                let p = sample_m(origin, dx, col as f64, row as f64);
                let count = map.unit_weights(p, &[1.0], params.unit_blend_m, &mut buf);
                let (mut hs, mut ds) = (0.0, 0.0);
                for (k, uw) in buf[..count].iter().enumerate() {
                    let ui = unit_of(uw.ix, uw.iy);
                    let u = &units[usize::from(ui)];
                    if k == 0 {
                        ur[col] = ui;
                    }
                    let c = [(uw.ix as f64 + 0.5) * cell, (uw.iy as f64 + 0.5) * cell];
                    let d = unit_datum(u, p, c, low[row * w + col], || low_at(c));
                    hs += uw.w * u.hardness;
                    ds += uw.w * d;
                }
                hr[col] = (hs * 255.0).round().clamp(0.0, 255.0) as u8;
                dr[col] = ds as f32;
            }
        });
    Ok(GeologyMap {
        w,
        h,
        spacing_m: dx,
        origin_m: origin,
        params: *params,
        units: units.clone(),
        unit,
        hardness,
        datum,
        cliff: vec![0; n],
        talus: vec![0; n],
        human: vec![0; n],
        lip: vec![f16_bits(params.lip_band_m as f32); n],
        feature: vec![0; n],
    })
}

/// One product file: name and little-endian bytes.
type Layer = (&'static str, Vec<u8>);

impl GeologyMap {
    /// The bed at guide-frame point (x, y) and height `z`: the nearest sample's unit, the bilinear datum.
    pub fn bed_at(&self, x: f64, y: f64, z: f64) -> Bed {
        let col = (x - self.origin_m[0]) / self.spacing_m;
        let row = (self.origin_m[1] - y) / self.spacing_m;
        let c = (col.round().max(0.0) as usize).min(self.w - 1);
        let r = (row.round().max(0.0) as usize).min(self.h - 1);
        let ui = self.unit[r * self.w + c];
        let cc = col.clamp(0.0, (self.w - 1) as f64);
        let rr = row.clamp(0.0, (self.h - 1) as f64);
        let (c0, r0) = (cc.floor() as usize, rr.floor() as usize);
        let (c1, r1) = ((c0 + 1).min(self.w - 1), (r0 + 1).min(self.h - 1));
        let (tx, ty) = (cc - c0 as f64, rr - r0 as f64);
        let d = |c: usize, r: usize| f64::from(self.datum[r * self.w + c]);
        let top = d(c0, r0) + (d(c1, r0) - d(c0, r0)) * tx;
        let bot = d(c0, r1) + (d(c1, r1) - d(c0, r1)) * tx;
        let datum = top + (bot - top) * ty;
        bed(
            &self.units[usize::from(ui)],
            ui,
            z - datum,
            self.params.seed,
            self.params.soft_bed_ratio,
        )
    }

    /// The product files in their fixed order.
    fn layers(&self) -> Vec<Layer> {
        let bytes16 = |v: &[u16]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        vec![
            ("unit.u8", self.unit.clone()),
            ("hardness.u8", self.hardness.clone()),
            ("datum.f32", self.datum.iter().flat_map(|x| x.to_le_bytes()).collect()),
            ("cliff.u8", self.cliff.clone()),
            ("talus.u8", self.talus.clone()),
            ("human.u8", self.human.clone()),
            ("lip.f16", bytes16(&self.lip)),
            ("feature.u16", bytes16(&self.feature)),
        ]
    }

    /// blake3 over every product file in order (the map's product hash).
    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        for (_, b) in self.layers() {
            hasher.update(&b);
        }
        hasher.finalize().to_hex().to_string()
    }

    /// Writes `dir/geology/` (module docs) and returns the product hash.
    pub fn write(&self, dir: &Path, declared: &[Declared]) -> Result<String> {
        let out = dir.join("geology");
        std::fs::create_dir_all(&out)?;
        let mut files = serde_json::Map::new();
        let mut total = blake3::Hasher::new();
        for (name, b) in self.layers() {
            std::fs::write(out.join(name), &b)?;
            total.update(&b);
            files.insert(name.into(), blake3::hash(&b).to_hex().to_string().into());
        }
        let hash = total.finalize().to_hex().to_string();
        let values: serde_json::Map<String, serde_json::Value> = declared
            .iter()
            .map(|d| {
                (
                    d.name.to_string(),
                    serde_json::json!({ "v": d.v, "unit": d.unit, "source": d.source }),
                )
            })
            .collect();
        std::fs::write(
            out.join("geology.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "format": FORMAT,
                "units": self.units,
                "values": values,
            }))?,
        )?;
        std::fs::write(
            out.join("meta.json"),
            serde_json::to_string_pretty(&serde_json::json!({
                "format": FORMAT,
                "producer": "sf_worldgen geology (LF-1)",
                "dims": [self.w, self.h],
                "sample_spacing_m": self.spacing_m,
                "origin_m": self.origin_m,
                "files": files,
                "blake3": hash,
            }))?,
        )?;
        Ok(hash)
    }

    /// Share of samples per unit (in unit order).
    pub fn unit_shares(&self) -> Vec<f64> {
        let mut c = vec![0usize; self.units.len()];
        for &u in &self.unit {
            c[usize::from(u)] += 1;
        }
        c.iter().map(|&v| v as f64 / self.unit.len().max(1) as f64).collect()
    }
}

/// The unit and cliff layers of a `geology/` product, read back for the scatter's rock ground (worldgen-scatter.md
/// "Rocks (WG-E3)" "Inputs"; WG-E3f.3): the frame, the unit names and `unit.u8` / `cliff.u8`, nothing else.
#[derive(Debug, Clone, PartialEq)]
pub struct GeologyGrid {
    pub w: usize,
    pub h: usize,
    /// `meta.json`'s spacing and origin (guide-frame metres of sample (0, 0), row 0 north), as written.
    pub spacing_m: f64,
    pub origin_m: [f64; 2],
    /// Unit names in index order (`geology.json`'s `units`).
    pub units: Vec<String>,
    /// Unit index per sample (row-major, row 0 north).
    pub unit: Vec<u8>,
    /// Cliff flag per sample (non-zero = a mapped cliff).
    pub cliff: Vec<u8>,
}

impl GeologyGrid {
    /// Reads `dir/geology/` (written by [`GeologyMap::write`]): the format of `meta.json` and `geology.json`, the size
    /// and blake3 of `unit.u8` and `cliff.u8` against `meta.json`, every unit index against the unit names.
    ///
    /// # Errors
    /// A missing or unreadable file, another format, a size or hash mismatch, an out-of-range unit index.
    pub fn read(dir: &Path) -> Result<Self> {
        #[derive(Deserialize)]
        struct Meta {
            format: u32,
            dims: [usize; 2],
            sample_spacing_m: f64,
            origin_m: [f64; 2],
            files: BTreeMap<String, String>,
        }
        #[derive(Deserialize)]
        struct Unit {
            name: String,
        }
        #[derive(Deserialize)]
        struct Info {
            format: u32,
            units: Vec<Unit>,
        }
        let src = dir.join("geology");
        let bad = |what: String| Error::Invalid(format!("geology product {}: {what}", src.display()));
        let meta: Meta = serde_json::from_str(&std::fs::read_to_string(src.join("meta.json"))?)?;
        let info: Info = serde_json::from_str(&std::fs::read_to_string(src.join("geology.json"))?)?;
        if meta.format != FORMAT || info.format != FORMAT {
            return Err(bad(format!(
                "format {} / {} (meta / geology.json), this build reads {FORMAT}",
                meta.format, info.format
            )));
        }
        let [w, h] = meta.dims;
        let layer = |name: &str| -> Result<Vec<u8>> {
            let want = meta.files.get(name).ok_or_else(|| bad(format!("meta.json has no blake3 for {name}")))?;
            let bytes = std::fs::read(src.join(name))?;
            if bytes.len() != w * h {
                return Err(bad(format!("{name} is {} bytes, {w} x {h} needs {}", bytes.len(), w * h)));
            }
            let got = blake3::hash(&bytes).to_hex().to_string();
            if &got != want {
                return Err(bad(format!("{name} blake3 {got} differs from meta.json's {want}")));
            }
            Ok(bytes)
        };
        let unit = layer("unit.u8")?;
        let cliff = layer("cliff.u8")?;
        let units: Vec<String> = info.units.into_iter().map(|u| u.name).collect();
        if let Some(&u) = unit.iter().find(|&&u| usize::from(u) >= units.len()) {
            return Err(bad(format!("unit index {u}, geology.json names {} units", units.len())));
        }
        Ok(Self {
            w,
            h,
            spacing_m: meta.sample_spacing_m,
            origin_m: meta.origin_m,
            units,
            unit,
            cliff,
        })
    }

    /// The nearest sample's index at guide-frame point (x, y); `None` outside the grid by more than half a sample.
    #[must_use]
    pub fn sample(&self, x: f64, y: f64) -> Option<usize> {
        let col = ((x - self.origin_m[0]) / self.spacing_m).round();
        let row = ((self.origin_m[1] - y) / self.spacing_m).round();
        let inside = |v: f64, n: usize| v >= 0.0 && v <= (n - 1) as f64;
        (inside(col, self.w) && inside(row, self.h)).then(|| row as usize * self.w + col as usize)
    }

    /// The unit name at (x, y) (nearest sample).
    #[must_use]
    pub fn unit_at(&self, x: f64, y: f64) -> Option<&str> {
        self.sample(x, y).map(|i| self.units[usize::from(self.unit[i])].as_str())
    }

    /// A mapped cliff at (x, y) (nearest sample; `false` outside the grid).
    #[must_use]
    pub fn cliff_at(&self, x: f64, y: f64) -> bool {
        self.sample(x, y).is_some_and(|i| self.cliff[i] != 0)
    }
}

/// One `scrib-worldgen geology` invocation.
#[derive(Debug, Clone)]
pub struct GeologyRequest {
    /// Base guide folder (v1).
    pub base: std::path::PathBuf,
    /// Geology table TOML.
    pub table: std::path::PathBuf,
    /// `name=value` overrides of the declared values (after the table's `[values]`).
    pub sets: Vec<(String, f64)>,
    /// Output folder (`geology/` is written inside it).
    pub out: std::path::PathBuf,
}

/// Reads the guide and the table, builds the map, writes it and prints the build time, unit shares and hash.
pub fn run_geology(req: &GeologyRequest) -> Result<GeologyMap> {
    let gd = crate::guide::Guide::read(&req.base)?;
    let table = GeologyTable::from_toml(&std::fs::read_to_string(&req.table)?)?;
    let mut declared = table.declared()?;
    for (k, v) in &req.sets {
        let p = declared
            .iter_mut()
            .find(|p| p.name == k)
            .ok_or_else(|| Error::Invalid(format!("geology: unknown value {k}")))?;
        p.v = *v;
        p.source = "Input(--set)".into();
    }
    let params = GeologyParams::from_declared(&declared)?;
    let heights: Vec<f64> = gd.heights.iter().map(|&v| f64::from(v)).collect();
    let t = std::time::Instant::now();
    let map = build(
        &BaseGrid {
            heights: &heights,
            w: gd.w,
            h: gd.h,
            spacing_m: gd.spacing_m,
            origin_m: gd.origin_m,
            regions: gd.regions.as_ref(),
        },
        &table,
        &params,
    )?;
    let built = t.elapsed().as_secs_f64();
    let t = std::time::Instant::now();
    let hash = map.write(&req.out, &declared)?;
    let written = t.elapsed().as_secs_f64();
    let shares: Vec<String> = map
        .units
        .iter()
        .zip(map.unit_shares())
        .map(|(u, s)| format!("{} {:.1} %", u.name, 100.0 * s))
        .collect();
    println!(
        "geology: {}x{} ({:.1} M samples), {} units ({}); build {built:.2} s, write {written:.2} s; blake3 {}",
        map.w,
        map.h,
        (map.w * map.h) as f64 * 1e-6,
        map.units.len(),
        shares.join(", "),
        &hash[..12]
    );
    Ok(map)
}
