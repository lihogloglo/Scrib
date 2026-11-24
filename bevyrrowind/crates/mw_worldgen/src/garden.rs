//! Garden planting for `worldgen-town` (G1b): trees and shrubs on the `placements.json` features of kind `"garden"`.
//!
//! **Temporary scaffold (owner 2026-11-24).** Lots where no kit house fits, Magna Urbis' garden lots and slivers get
//! trees and shrubs "just to have something". It is isolated behind `worldgen-town --gardens <toml>` (absent or
//! `--no-gardens`: no plants, the mod byte for byte what it was without it) and this module. Removal path: Magna
//! Urbis' own land-use planting (gardens, yards, orchards) or flora garden zones, plus small kit structures on
//! slivers; then delete this module, `bevyrrowind/worldgen/town-gardens.toml` and the two flags.
//!
//! - Feature (contract): `{ "kind": "garden", "source": "empty_plot" | "mu_garden" | "sliver", "lot": "<lot id>",
//!   "polygon": [[x, y], ...] }`, guide frame (x east, y north, metres), counter-clockwise, no holes. Other kinds are
//!   ignored (counted).
//! - Scatter: seeded Poisson-disk sampling (Bridson 2007, `candidates` per active sample) inside the polygon inset by
//!   `edge_inset_m` plus the class clearance, deterministic per lot id (the seed hashes the file's seed, the lot id
//!   and the feature's index among that lot's gardens). Trees first at `tree_spacing_m`; then shrubs at
//!   `shrub_spacing_m` in the gaps, each at least half of both spacings from every tree disk. After its Bridson pass
//!   the region is swept cell by cell (cell = spacing / sqrt 2) for samples the front missed (disconnected parts,
//!   thin slivers). Each disk is planted with probability `1 - open_share`; an unplanted disk stays open (an open
//!   tree disk is a clearing no shrub fills). A polygon too thin for a tree gets shrubs only, or nothing.
//! - Species: weighted pick within the class (`[[species]]`); a species whose record does not resolve is dropped
//!   (listed in the report). Random yaw, unit scale.
//! - Placements: key `garden:<lot>:<n>` (n counts that lot's plants from 0), `GROUNDED`, height from the guide's
//!   ground at the pivot (the caller's `height`: `GuideTiles::height` at level 0, the ground the game draws, as the
//!   flora pass does; the game itself never moves a placement to the ground).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;
use sf_vfs::VfsPath;
use sf_world::{GroupPlacement, PlacementFlags, RecordRef};
use sf_worldgen::recipe::Param;

use crate::DumpError;
use crate::town::engine_transform;
use mw_world::source::Base;

/// The feature kind this module plants.
// tunables: const (format: the contract's feature kind)
pub const KIND: &str = "garden";
/// The key prefix of a garden placement.
// tunables: const (format: an id namespace)
pub const KEY_PREFIX: &str = "garden";

/// Size class of a species.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Class {
    Tree,
    Shrub,
}

/// One `[[species]]` entry.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Species {
    /// Base record id.
    pub id: String,
    pub class: Class,
    pub weight: Param,
}

/// The `[params]` of `town-gardens.toml`.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Params {
    pub seed: Param,
    pub tree_spacing_m: Param,
    pub shrub_spacing_m: Param,
    pub tree_clearance_m: Param,
    pub shrub_clearance_m: Param,
    pub edge_inset_m: Param,
    pub open_share: Param,
    pub candidates: Param,
}

/// `town-gardens.toml`.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GardenTable {
    pub params: Params,
    pub species: Vec<Species>,
}

impl GardenTable {
    /// Parses the table and checks every value against its declared range.
    ///
    /// # Errors
    /// Not TOML of this shape, a value out of its range, a spacing that is not positive.
    pub fn from_toml(text: &str) -> Result<Self, DumpError> {
        let t: Self = toml::from_str(text).map_err(|e| DumpError::Invalid(format!("town-gardens.toml: {e}")))?;
        let p = &t.params;
        let named = [
            ("seed", &p.seed),
            ("tree_spacing_m", &p.tree_spacing_m),
            ("shrub_spacing_m", &p.shrub_spacing_m),
            ("tree_clearance_m", &p.tree_clearance_m),
            ("shrub_clearance_m", &p.shrub_clearance_m),
            ("edge_inset_m", &p.edge_inset_m),
            ("open_share", &p.open_share),
            ("candidates", &p.candidates),
        ];
        let species = t.species.iter().map(|s| (s.id.as_str(), &s.weight));
        for (name, v) in named.into_iter().chain(species) {
            if !(v.min..=v.max).contains(&v.v) {
                return Err(DumpError::Invalid(format!("town-gardens.toml: {name} = {} not in {}..={}", v.v, v.min, v.max)));
            }
        }
        if p.tree_spacing_m.v <= 0.0 || p.shrub_spacing_m.v <= 0.0 || p.candidates.v < 1.0 {
            return Err(DumpError::Invalid("town-gardens.toml: spacings > 0 and candidates >= 1".into()));
        }
        Ok(t)
    }

    /// Reads and parses `path`.
    ///
    /// # Errors
    /// Unreadable, or [`Self::from_toml`].
    pub fn read(path: &Path) -> Result<Self, DumpError> {
        let text = std::fs::read_to_string(path).map_err(|e| DumpError::Invalid(format!("{}: {e}", path.display())))?;
        Self::from_toml(&text)
    }
}

/// One garden feature (contract).
#[derive(Clone, Debug, Deserialize)]
pub struct GardenFeature {
    pub kind: String,
    pub source: String,
    pub lot: String,
    pub polygon: Vec<[f64; 2]>,
}

/// One plant, guide frame.
#[derive(Clone, Debug, PartialEq)]
pub struct Plant {
    pub class: Class,
    /// Index into the table's species.
    pub species: usize,
    pub pos: [f64; 2],
    /// Counter-clockwise about +z, radians.
    pub yaw: f32,
}

/// Trees and shrubs planted from one source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourceCount {
    pub features: usize,
    pub trees: usize,
    pub shrubs: usize,
}

/// What the garden pass did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GardenCounts {
    /// Per feature `source`.
    pub by_source: BTreeMap<String, SourceCount>,
    /// Features of another kind (ignored).
    pub other_kinds: usize,
    /// Species whose record does not resolve (dropped).
    pub unresolved: Vec<String>,
}

/// A small deterministic generator (`SplitMix64`, Steele et al. 2014).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // tunables: const (maths: SplitMix64's constants)
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    #[allow(clippy::cast_precision_loss)] // 53 bits: exact in f64.
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// The seed of the `index`-th garden of `lot`.
fn lot_seed(seed: u64, lot: &str, index: usize) -> u64 {
    let mut h = blake3::Hasher::new();
    h.update(&seed.to_le_bytes());
    h.update(lot.as_bytes());
    h.update(&(index as u64).to_le_bytes());
    let b = h.finalize();
    u64::from_le_bytes(b.as_bytes()[..8].try_into().expect("8 bytes"))
}

/// Whether `p` is inside `poly` (even-odd).
pub fn inside(poly: &[[f64; 2]], p: [f64; 2]) -> bool {
    let mut odd = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + n - 1) % n]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            odd = !odd;
        }
    }
    odd
}

/// Distance from `p` to the boundary of `poly`.
#[allow(clippy::many_single_char_names)] // Geometry: the usual names.
pub fn edge_distance(poly: &[[f64; 2]], p: [f64; 2]) -> f64 {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let len2 = dx * dx + dy * dy;
            let t = if len2 > 0.0 { (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / len2).clamp(0.0, 1.0) } else { 0.0 };
            (p[0] - a[0] - t * dx).hypot(p[1] - a[1] - t * dy)
        })
        .fold(f64::INFINITY, f64::min)
}

/// Whether `p` lies in `poly` at least `inset` from its boundary.
pub fn inside_inset(poly: &[[f64; 2]], p: [f64; 2], inset: f64) -> bool {
    inside(poly, p) && edge_distance(poly, p) >= inset
}

fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)
}

/// Bridson's Poisson-disk samples at spacing `r` inside `poly` inset by `inset`, at least `apart` from each of
/// `blockers`, then a sweep of the bounding box (cell r / sqrt 2) for regions the front missed.
#[allow(clippy::many_single_char_names)] // Bridson's own names (r, k).
fn bridson(
    rng: &mut Rng,
    poly: &[[f64; 2]],
    inset: f64,
    r: f64,
    k: usize,
    blockers: &[[f64; 2]],
    apart: f64,
) -> Vec<[f64; 2]> {
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for p in poly {
        for a in 0..2 {
            lo[a] = lo[a].min(p[a]);
            hi[a] = hi[a].max(p[a]);
        }
    }
    let mut out: Vec<[f64; 2]> = Vec::new();
    if poly.len() < 3 || hi[0] - lo[0] < 2.0 * inset || hi[1] - lo[1] < 2.0 * inset {
        return out;
    }
    let ok = |out: &[[f64; 2]], p: [f64; 2]| {
        inside_inset(poly, p, inset)
            && out.iter().all(|q| dist2(*q, p) >= r * r)
            && blockers.iter().all(|q| dist2(*q, p) >= apart * apart)
    };
    let grow = |rng: &mut Rng, out: &mut Vec<[f64; 2]>, start: usize| {
        let mut active = vec![start];
        while !active.is_empty() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
            let i = ((rng.unit() * active.len() as f64) as usize).min(active.len() - 1);
            let c = out[active[i]];
            let mut found = false;
            for _ in 0..k {
                let a = rng.unit() * std::f64::consts::TAU;
                let d = r * (1.0 + rng.unit());
                let p = [c[0] + d * a.cos(), c[1] + d * a.sin()];
                if ok(out, p) {
                    out.push(p);
                    active.push(out.len() - 1);
                    found = true;
                    break;
                }
            }
            if !found {
                active.swap_remove(i);
            }
        }
    };
    let cell = r / std::f64::consts::SQRT_2;
    let mut y = lo[1];
    while y < hi[1] {
        let mut x = lo[0];
        while x < hi[0] {
            for _ in 0..k {
                let p = [x + rng.unit() * cell, y + rng.unit() * cell];
                if ok(&out, p) {
                    out.push(p);
                    let start = out.len() - 1;
                    grow(rng, &mut out, start);
                    break;
                }
            }
            x += cell;
        }
        y += cell;
    }
    out
}

/// The weighted pick table of `class` (species index, cumulative weight).
fn picks(table: &GardenTable, class: Class, usable: &[bool]) -> Vec<(usize, f64)> {
    let mut acc = 0.0;
    let mut v = Vec::new();
    for (i, s) in table.species.iter().enumerate() {
        if s.class == class && usable[i] && s.weight.v > 0.0 {
            acc += s.weight.v;
            v.push((i, acc));
        }
    }
    v
}

fn pick(rng: &mut Rng, list: &[(usize, f64)]) -> usize {
    let total = list.last().map_or(0.0, |l| l.1);
    let u = rng.unit() * total;
    list.iter().find(|(_, c)| u < *c).or(list.last()).map(|l| l.0).expect("a non-empty list")
}

/// The plants of one garden polygon (seeded by `seed`); `usable[i]`: species `i` resolves.
pub fn scatter(table: &GardenTable, polygon: &[[f64; 2]], seed: u64, usable: &[bool]) -> Vec<Plant> {
    let p = &table.params;
    let mut rng = Rng(seed);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // A small declared count.
    let k = p.candidates.v as usize;
    let keep = 1.0 - p.open_share.v;
    let (rt, rs) = (p.tree_spacing_m.v, p.shrub_spacing_m.v);
    let mut plants = Vec::new();
    let trees = picks(table, Class::Tree, usable);
    let shrubs = picks(table, Class::Shrub, usable);
    let tree_disks =
        if trees.is_empty() { Vec::new() } else { bridson(&mut rng, polygon, p.edge_inset_m.v + p.tree_clearance_m.v, rt, k, &[], 0.0) };
    for &pos in &tree_disks {
        if rng.unit() < keep {
            let species = pick(&mut rng, &trees);
            #[allow(clippy::cast_possible_truncation)]
            let yaw = (rng.unit() * std::f64::consts::TAU) as f32;
            plants.push(Plant { class: Class::Tree, species, pos, yaw });
        }
    }
    if !shrubs.is_empty() {
        let inset = p.edge_inset_m.v + p.shrub_clearance_m.v;
        for pos in bridson(&mut rng, polygon, inset, rs, k, &tree_disks, f64::midpoint(rt, rs)) {
            if rng.unit() < keep {
                let species = pick(&mut rng, &shrubs);
                #[allow(clippy::cast_possible_truncation)]
                let yaw = (rng.unit() * std::f64::consts::TAU) as f32;
                plants.push(Plant { class: Class::Shrub, species, pos, yaw });
            }
        }
    }
    plants
}

/// The garden placements of `features` (other kinds ignored), species resolved by `resolve`, heights from `height`
/// (guide x, y → z).
///
/// # Errors
/// A `"garden"` feature that does not parse.
pub fn plant(
    table: &GardenTable,
    features: &[serde_json::Value],
    resolve: impl Fn(&str) -> Option<Base>,
    height: &dyn Fn(f64, f64) -> f64,
) -> Result<(Vec<GroupPlacement>, GardenCounts), DumpError> {
    let mut counts = GardenCounts::default();
    let bases: Vec<Option<Base>> = table.species.iter().map(|s| resolve(&s.id)).collect();
    counts.unresolved = table.species.iter().zip(&bases).filter(|(_, b)| b.is_none()).map(|(s, _)| s.id.clone()).collect();
    let usable: Vec<bool> = bases.iter().map(Option::is_some).collect();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // A declared id in u32 range.
    let seed = table.params.seed.v as u64;
    let mut per_lot: BTreeMap<String, (usize, usize)> = BTreeMap::new(); // lot → (gardens, plants)
    let mut out = Vec::new();
    for f in features {
        if f.get("kind").and_then(serde_json::Value::as_str) != Some(KIND) {
            counts.other_kinds += 1;
            continue;
        }
        let g: GardenFeature = serde_json::from_value(f.clone())?;
        let slot = per_lot.entry(g.lot.clone()).or_default();
        let plants = scatter(table, &g.polygon, lot_seed(seed, &g.lot, slot.0), &usable);
        slot.0 += 1;
        let c = counts.by_source.entry(g.source.clone()).or_default();
        c.features += 1;
        for pl in plants {
            match pl.class {
                Class::Tree => c.trees += 1,
                Class::Shrub => c.shrubs += 1,
            }
            let s = &table.species[pl.species];
            let base = bases[pl.species].as_ref().expect("usable species only");
            let z = height(pl.pos[0], pl.pos[1]);
            out.push(GroupPlacement {
                key: format!("{KEY_PREFIX}:{}:{}", g.lot, slot.1),
                base: RecordRef::new(base.tag, s.id.as_str()),
                model: base.model.as_deref().and_then(|m| VfsPath::new(m).ok()),
                transform: engine_transform([pl.pos[0], pl.pos[1], z], pl.yaw),
                class: base.class,
                flags: PlacementFlags::GROUNDED,
            });
            slot.1 += 1;
        }
    }
    Ok((out, counts))
}

/// The report line.
pub fn report_line(c: &GardenCounts) -> String {
    let mut s = String::from("  gardens:");
    let (mut t, mut sh) = (0, 0);
    for (src, n) in &c.by_source {
        let _ = write!(s, " {src} {} features {} trees {} shrubs;", n.features, n.trees, n.shrubs);
        t += n.trees;
        sh += n.shrubs;
    }
    let _ = write!(s, " total {t} trees {sh} shrubs; {} other features ignored", c.other_kinds);
    if !c.unresolved.is_empty() {
        let _ = write!(s, "; unresolved species (dropped): {}", c.unresolved.join(", "));
    }
    s.push('\n');
    s
}
