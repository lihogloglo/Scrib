//! `worldgen-kit` (S1, scrib-worldgen `docs/research/settlements-m2-placement-2026-11-24.md` section 5a): the
//! **kit catalogue** of one or more named settlement anchors in one art style, `kit.json` format 1, for the settlement
//! tool (files only).
//!
//! One entry per distinct model of the anchors' members that is not a door (doors are separate `DOOR` references
//! against a building's wall; they become the buildings' `doors`) and that the table's `[style]` keeps (K1,
//! [`KitStyle`]: a `keep` pattern and no `drop` pattern). Several anchors (K1) pool their references, a reference in
//! two anchors once, so every rule below runs over all of them; the kit's `name` is `A + B`. Local frame of an entry: the model's own frame at
//! unit scale, x = model x (east at yaw 0), y = model north (engine -z), z up; pivot at the model origin. The frame
//! is not turned so that +y is the door side: `doors[].facing_deg` says where each door looks.
//!
//! - `id`: the model's VFS path; `tags`: `base:<record id>` of each base record using it.
//! - `kind`: the first rule of `bevyrrowind/data/worldgen-kit.toml` ([`KitTable`]) whose pattern is a substring of
//!   the lower-case id; `prop` when none matches.
//! - `footprint`: convex hull (counter-clockwise) of the model's visible vertices projected on the ground;
//!   `obb`: the minimum-area rectangle of the hull (rotating calipers over its edges; `yaw_deg` in `[0, 90)`).
//! - `height_m` = highest minus lowest vertex z; `base_offset_m` = lowest vertex z (the NIF bound is centred, so
//!   vertices are used, not the bound).
//! - `weight`: the model's reference count over the anchors; `blake3`: of the model file's bytes.
//! - `doors`: each `DOOR` member is given to the nearest building or tower reference (dropped models included, so a
//!   door never goes to the wrong building) whose world hull is within
//!   [`KitParams::door_radius_m`] (ties: lower reference id). The door mesh's normal is its thinnest axis (in its
//!   own frame); a door whose normal is vertical is a trapdoor and is left out (counted in the report). In the
//!   building's frame: the door's edge is the hull edge nearest its pivot on the ground, except near a corner (the
//!   two nearest edges both within [`KitParams::door_corner_m`]), where it is the one of the two whose outward
//!   normal best agrees with the mesh normal (either sign); `at` = the pivot projected on that edge; `facing_deg` =
//!   the edge's outward normal, 0 = +x, counter-clockwise; `width_m` = the door mesh's extent across its own normal;
//!   `sill_z_m` = its lowest vertex z. A model keeps the doors of its reference with the most doors (ties: lowest
//!   reference id), street door first (by `sill_z_m`, then `at`); the report counts how many of its other
//!   references agree ([`KitParams::door_agree_m`], [`KitParams::door_agree_deg`]).
//! - `ground_z_m` (S1c): where the vanilla terrain stands in the model's frame, so a layout can seat the model on new
//!   ground the way the authors did. Per reference, the **pad point** is the model's street door (its first door)
//!   moved out along `facing_deg` by [`KitParams::pad_probe_m`], or the hull centroid for a doorless model; the
//!   source terrain there, minus the reference's z, over its scale, is that reference's ground. `ground_z_m` is the
//!   median over the references (null without terrain); `foundation_depth_m` = `ground_z_m - base_offset_m` (the
//!   mesh below the pad). The report prints each model's spread over its references and the lowest and highest
//!   terrain under its hull (sampled every [`KitParams::hull_probe_m`]).
//! - Not filled yet: `roles` (`[]`), `floors` (null), `yaw_step_deg` (0 = any yaw).
//!
//! Values are rounded to the millimetre and the hundredth of a degree, so the file is deterministic.

// Model sizes and counts stay far below 2^31 and within a few km.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use glam::{DQuat, DVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::WorldTransform;
use sf_records::RecordDb;
use sf_world::PlacementClass;
use sf_worldgen::filter::Declared;

use crate::DumpError;
use crate::site::{self, SiteParams, convex_hull};
use crate::x10::Bounds;

/// The kit format this module writes and reads.
// tunables: const (format version of the contract)
pub const FORMAT: u32 = 1;
/// File name of the catalogue (contract).
pub const KIT_FILE: &str = "kit.json";
/// The shipped kind table, `bevyrrowind/data/worldgen-kit.toml`.
pub const BUILTIN: &str = include_str!("../../../data/worldgen-kit.toml");
/// The `frame` the file states (module docs).
pub const FRAME: &str = "local x right (model x), y model forward (model north at yaw 0; not turned to the door side, \
                         see doors[].facing_deg), z up; pivot at model origin; unit scale";
/// Output precision: steps per metre and per degree.
// tunables: const (format: written precision)
const PER_M: f64 = 1000.0;
// tunables: const (format: written precision)
const PER_DEG: f64 = 100.0;

/// A model's visible vertices (model space at unit scale, engine axes, metres) and the blake3 (hex) of its file.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KitMesh {
    pub points: Vec<[f32; 3]>,
    pub blake3: String,
}

/// blake3 (hex) of a model file's bytes, as [`KitMesh::blake3`] holds it.
pub fn hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// Model lookup: VFS path -> its mesh.
pub type Meshes<'a> = &'a dyn Fn(&str) -> Option<KitMesh>;

/// The source terrain: height (metres) at `[x east, y north]` (source metres), `None` where there is none.
pub type Terrain<'a> = &'a dyn Fn([f64; 2]) -> Option<f64>;

/// The declared kit values (spec `tunables.md` "Definition").
pub fn declared() -> Vec<Declared> {
    vec![
        Declared {
            name: "kit.door_radius_m",
            v: 1.5,
            unit: "m",
            min: 0.0,
            max: 20.0,
            source: "Guess".into(),
            help: "how far outside a building's ground hull a door pivot may stand and still be that building's door \
                   (load doors stand against the wall, inside or just outside the hull)",
        },
        Declared {
            name: "kit.door_corner_m",
            v: 1.0,
            unit: "m",
            min: 0.0,
            max: 10.0,
            source: "Guess".into(),
            help: "a door pivot whose two nearest hull edges are both within this distance stands at a corner: its \
                   edge is the one whose outward normal agrees with the door mesh's normal, not simply the nearest",
        },
        Declared {
            name: "kit.door_agree_m",
            v: 0.25,
            unit: "m",
            min: 0.0,
            max: 10.0,
            source: "Guess".into(),
            help: "largest door position (local frame) and sill height difference at which two references of a model \
                   have the same door",
        },
        Declared {
            name: "kit.door_agree_deg",
            v: 5.0,
            unit: "deg",
            min: 0.0,
            max: 180.0,
            source: "Guess".into(),
            help: "largest door facing difference at which two references of a model have the same door",
        },
        Declared {
            name: "kit.bin_m",
            v: 2.0,
            unit: "m",
            min: 0.1,
            max: 100.0,
            source: "Guess".into(),
            help: "bin width of the printed front and depth histograms",
        },
        Declared {
            name: "kit.pad_probe_m",
            v: 1.0,
            unit: "m",
            min: 0.0,
            max: 10.0,
            source: "Guess".into(),
            help: "how far out of the street door (along its facing) the vanilla terrain is sampled for a model's ground \
                   height: the pad the player steps on, clear of the door's own sill",
        },
        Declared {
            name: "kit.hull_probe_m",
            v: 1.0,
            unit: "m",
            min: 0.1,
            max: 50.0,
            source: "Guess".into(),
            help: "spacing of the terrain samples under a reference's hull for the printed lowest and highest ground \
                   (the hull's vertices are always sampled)",
        },
        Declared {
            name: "kit.report_spread_m",
            v: 0.5,
            unit: "m",
            min: 0.0,
            max: 100.0,
            source: "Guess".into(),
            help: "the report lists models whose ground height differs by more than this between their references",
        },
        Declared {
            name: "kit.report_sill_m",
            v: 0.3,
            unit: "m",
            min: 0.0,
            max: 10.0,
            source: "Guess".into(),
            help: "the report counts buildings whose ground height is within this of their street door's sill",
        },
    ]
}

/// The kit values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KitParams {
    pub door_radius_m: f64,
    pub door_corner_m: f64,
    pub door_agree_m: f64,
    pub door_agree_deg: f64,
    pub bin_m: f64,
    pub pad_probe_m: f64,
    pub hull_probe_m: f64,
    pub report_spread_m: f64,
    pub report_sill_m: f64,
}

impl Default for KitParams {
    fn default() -> Self {
        let v = |name: &str| declared().into_iter().find(|d| d.name == name).map_or(f64::NAN, |d| d.v);
        Self {
            door_radius_m: v("kit.door_radius_m"),
            door_corner_m: v("kit.door_corner_m"),
            door_agree_m: v("kit.door_agree_m"),
            door_agree_deg: v("kit.door_agree_deg"),
            bin_m: v("kit.bin_m"),
            pad_probe_m: v("kit.pad_probe_m"),
            hull_probe_m: v("kit.hull_probe_m"),
            report_spread_m: v("kit.report_spread_m"),
            report_sill_m: v("kit.report_sill_m"),
        }
    }
}

/// A kit entry's kind (contract).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KitKind {
    Building,
    /// A tall landmark of the style (Hlaalu: `ex_hlaalu_bal_NN`); takes doors like a building.
    Tower,
    Wall,
    Arch,
    Stairs,
    Bridge,
    Canal,
    Lamp,
    Prop,
}

impl KitKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Building => "building",
            Self::Tower => "tower",
            Self::Wall => "wall",
            Self::Arch => "arch",
            Self::Stairs => "stairs",
            Self::Bridge => "bridge",
            Self::Canal => "canal",
            Self::Lamp => "lamp",
            Self::Prop => "prop",
        }
    }

    /// Whether door references are given to this kind's references (buildings and towers).
    pub fn takes_doors(self) -> bool {
        matches!(self, Self::Building | Self::Tower)
    }
}

/// One rule of `worldgen-kit.toml`: models whose lower-case id contains one of `patterns` are `kind`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct KindRule {
    pub kind: KitKind,
    pub patterns: Vec<String>,
}

/// The `[style]` of `worldgen-kit.toml`: a model is kept when one of `keep` and none of `drop` is a substring of its
/// lower-case id (drop wins).
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct KitStyle {
    pub name: String,
    pub keep: Vec<String>,
    #[serde(default)]
    pub drop: Vec<String>,
}

impl KitStyle {
    /// Whether model `id` belongs to the style (ASCII case ignored).
    pub fn keeps(&self, id: &str) -> bool {
        let id = id.to_ascii_lowercase();
        let any = |v: &[String]| v.iter().any(|p| id.contains(&p.to_ascii_lowercase()));
        any(&self.keep) && !any(&self.drop)
    }
}

/// `worldgen-kit.toml`: the kind rules in order (the first match wins) and the art style (none keeps every model).
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct KitTable {
    #[serde(default)]
    pub style: Option<KitStyle>,
    #[serde(default)]
    pub kinds: Vec<KindRule>,
}

impl KitTable {
    pub fn from_toml(text: &str) -> Result<Self, DumpError> {
        toml::from_str(text).map_err(|e| DumpError::Invalid(format!("worldgen-kit.toml: {e}")))
    }

    /// The shipped table ([`BUILTIN`]).
    pub fn builtin() -> Result<Self, DumpError> {
        Self::from_toml(BUILTIN)
    }

    /// The kind of model `id` (ASCII case ignored); [`KitKind::Prop`] when no rule matches.
    pub fn kind_of(&self, id: &str) -> KitKind {
        let id = id.to_ascii_lowercase();
        self.kinds
            .iter()
            .find(|r| r.patterns.iter().any(|p| id.contains(&p.to_ascii_lowercase())))
            .map_or(KitKind::Prop, |r| r.kind)
    }

    /// Whether model `id` goes in the kit: the style keeps it, or there is no style.
    pub fn keeps(&self, id: &str) -> bool {
        self.style.as_ref().is_none_or(|s| s.keeps(id))
    }
}

/// Oriented rectangle of a footprint (local metres).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Obb {
    pub centre: [f64; 2],
    pub half: [f64; 2],
    pub yaw_deg: f64,
}

/// One door of a kit entry (local frame).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KitDoor {
    pub at: [f64; 2],
    pub facing_deg: f64,
    pub width_m: f64,
    pub sill_z_m: f64,
}

/// One kit entry (contract).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KitModel {
    pub id: String,
    pub kind: KitKind,
    pub tags: Vec<String>,
    pub roles: Vec<String>,
    pub footprint: Vec<[f64; 2]>,
    #[serde(default)]
    pub obb: Option<Obb>,
    pub doors: Vec<KitDoor>,
    pub height_m: f64,
    pub floors: Option<u32>,
    pub base_offset_m: f64,
    /// The vanilla terrain height in the model's frame (module docs); null without terrain.
    #[serde(default)]
    pub ground_z_m: Option<f64>,
    pub foundation_depth_m: Option<f64>,
    pub yaw_step_deg: f64,
    pub weight: f64,
    pub blake3: String,
}

/// `kit.json` (contract, format 1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KitJson {
    pub format: u32,
    pub name: String,
    pub producer: String,
    pub units: String,
    pub frame: String,
    pub models: Vec<KitModel>,
}

/// Writes `kit` as `dir/kit.json` (`dir` created).
pub fn write(kit: &KitJson, dir: &Path) -> Result<(), DumpError> {
    std::fs::create_dir_all(dir)?;
    std::fs::write(dir.join(KIT_FILE), serde_json::to_vec_pretty(kit)?)?;
    Ok(())
}

/// Reads `dir/kit.json`, checking the format.
pub fn read(dir: &Path) -> Result<KitJson, DumpError> {
    let kit: KitJson = serde_json::from_slice(&std::fs::read(dir.join(KIT_FILE))?)?;
    if kit.format != FORMAT {
        return Err(DumpError::Invalid(format!("kit format {} (expected {FORMAT})", kit.format)));
    }
    Ok(kit)
}

/// One member reference of the anchor.
#[derive(Clone, Debug, PartialEq)]
pub struct KitRef {
    /// The reference key (placement id).
    pub key: String,
    /// Base record id.
    pub base: String,
    pub model: Option<String>,
    pub transform: WorldTransform,
    pub door: bool,
}

/// Door agreement of one building model over its references.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorAgreement {
    pub id: String,
    /// Doors per reference, in reference id order.
    pub doors_per_ref: Vec<usize>,
    /// The doors of each reference, same order (each sorted as the kit writes them).
    pub doors_by_ref: Vec<Vec<KitDoor>>,
    /// Index (same order) of the reference whose doors the model keeps: the one with the most doors, the first on
    /// a tie.
    pub kept: usize,
    /// Other references whose doors match the kept one's.
    pub agreeing: usize,
}

/// The vanilla ground of one model (model frame, unit scale), over its references with terrain, in reference id
/// order.
#[derive(Clone, Debug, PartialEq)]
pub struct GroundStat {
    pub id: String,
    pub kind: KitKind,
    /// The pad point is the street door's outside point (else the hull centroid).
    pub at_door: bool,
    /// Terrain at the pad point, per reference.
    pub pad: Vec<f64>,
    /// Lowest and highest terrain under the hull, per reference.
    pub hull_min: Vec<f64>,
    pub hull_max: Vec<f64>,
}

impl GroundStat {
    /// Highest minus lowest pad ground over the references (0 for fewer than two).
    pub fn spread(&self) -> f64 {
        let lo = self.pad.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = self.pad.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        if self.pad.len() < 2 { 0.0 } else { hi - lo }
    }
}

/// What [`build`] found besides the catalogue.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KitStats {
    /// Door references, and how many were given to a building.
    pub door_refs: usize,
    pub doors_assigned: usize,
    /// Models of the door references near a building left out as trapdoors (vertical normal), one per reference,
    /// in door reference id order.
    pub trapdoors: Vec<String>,
    /// Models skipped: no mesh or no hull (ids).
    pub no_vertices: Vec<String>,
    /// References without a model.
    pub no_model: usize,
    /// References of a non-unit scale.
    pub scaled_refs: usize,
    /// Building models (by id order).
    pub agreement: Vec<DoorAgreement>,
    /// Every model's vanilla ground (by id order).
    pub ground: Vec<GroundStat>,
    /// Per anchor, in the order given.
    pub anchors: Vec<AnchorStat>,
    /// Models the style left out, with their reference counts over all anchors (by id order).
    pub dropped: Vec<(String, usize)>,
}

/// One anchor's share of a kit: its distinct non-door models the style kept and dropped, and its references.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnchorStat {
    pub name: String,
    pub kept: usize,
    pub dropped: usize,
    /// Member references, doors included.
    pub refs: usize,
}

/// The catalogue and its stats.
#[derive(Clone, Debug, PartialEq)]
pub struct Built {
    pub kit: KitJson,
    pub stats: KitStats,
}

fn round_m(v: f64) -> f64 {
    (v * PER_M).round() / PER_M + 0.0
}

fn round_deg(v: f64) -> f64 {
    let d = (v * PER_DEG).round() / PER_DEG + 0.0;
    if d >= 360.0 { d - 360.0 } else { d }
}

fn xy(p: [f64; 2]) -> [f64; 2] {
    [round_m(p[0]), round_m(p[1])]
}

/// Engine model axes -> kit local `[x, y, z]` (x, north = -z, up = y).
fn kit_local(v: DVec3) -> [f64; 3] {
    [v.x, -v.z, v.y]
}

/// The minimum-area rectangle of a counter-clockwise hull (the first edge wins a tie), `yaw_deg` in `[0, 90)`.
pub fn min_area_rect(hull: &[[f64; 2]]) -> Option<Obb> {
    let count = hull.len();
    let mut best: Option<(f64, Obb)> = None;
    for edge in 0..count {
        let (from, to) = (hull[edge], hull[(edge + 1) % count]);
        let (ex, ey) = (to[0] - from[0], to[1] - from[1]);
        let len = ex.hypot(ey);
        if len <= 0.0 {
            continue;
        }
        let (along, across) = ([ex / len, ey / len], [-ey / len, ex / len]);
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for p in hull {
            for (axis, dir) in [along, across].into_iter().enumerate() {
                let dist = p[0] * dir[0] + p[1] * dir[1];
                lo[axis] = lo[axis].min(dist);
                hi[axis] = hi[axis].max(dist);
            }
        }
        let area = (hi[0] - lo[0]) * (hi[1] - lo[1]);
        if best.as_ref().is_none_or(|(least, _)| area < *least) {
            let (mid_along, mid_across) = (f64::midpoint(lo[0], hi[0]), f64::midpoint(lo[1], hi[1]));
            let centre =
                [along[0] * mid_along + across[0] * mid_across, along[1] * mid_along + across[1] * mid_across];
            let mut half = [0.5 * (hi[0] - lo[0]), 0.5 * (hi[1] - lo[1])];
            let yaw = ey.atan2(ex).to_degrees();
            let quarters = (yaw / 90.0).floor();
            if (quarters as i64).rem_euclid(2) == 1 {
                half.swap(0, 1);
            }
            best = Some((area, Obb { centre, half, yaw_deg: yaw - 90.0 * quarters }));
        }
    }
    best.map(|(_, mut o)| {
        o.centre = xy(o.centre);
        o.half = xy(o.half);
        o.yaw_deg = (o.yaw_deg * PER_DEG).round() / PER_DEG + 0.0;
        if o.yaw_deg >= 90.0 {
            o.yaw_deg -= 90.0;
            o.half.swap(0, 1);
        }
        o
    })
}

/// Whether `p` is inside the counter-clockwise convex polygon `poly` (edges included).
fn inside(poly: &[[f64; 2]], p: [f64; 2]) -> bool {
    let n = poly.len();
    (0..n).all(|i| {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) >= 0.0
    })
}

/// The point of segment `a..b` nearest `p`.
fn nearest_on(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> [f64; 2] {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let len2 = dx * dx + dy * dy;
    let t = if len2 > 0.0 { (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / len2).clamp(0.0, 1.0) } else { 0.0 };
    [a[0] + t * dx, a[1] + t * dy]
}

/// The nearest edge of `poly` to `p`: its index, the nearest point on it, and the distance.
fn nearest_edge(poly: &[[f64; 2]], p: [f64; 2]) -> (usize, [f64; 2], f64) {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let q = nearest_on(poly[i], poly[(i + 1) % n], p);
            (i, q, (q[0] - p[0]).hypot(q[1] - p[1]))
        })
        .fold((0, poly[0], f64::INFINITY), |best, e| if e.2 < best.2 { e } else { best })
}

/// Distance of `p` to the convex polygon `poly` (0 inside).
fn distance_to(poly: &[[f64; 2]], p: [f64; 2]) -> f64 {
    if inside(poly, p) { 0.0 } else { nearest_edge(poly, p).2 }
}

/// A placement's frame in f64.
struct Frame {
    pos: DVec3,
    rot: DQuat,
    scale: DVec3,
}

impl Frame {
    fn of(tf: &WorldTransform) -> Self {
        Self { pos: tf.pos.to_f64(), rot: tf.rotation.as_dquat(), scale: tf.scale.as_dvec3() }
    }
    fn to_world(&self, v: DVec3) -> DVec3 {
        self.pos + self.rot * (self.scale * v)
    }
    fn to_local(&self, w: DVec3) -> DVec3 {
        (self.rot.inverse() * (w - self.pos)) / self.scale
    }
}

/// One door in a building reference's frame (unrounded) and its key.
struct LocalDoor {
    key: String,
    door: KitDoor,
}

/// The outward normal of edge `i` of the counter-clockwise polygon `poly` (zero for a degenerate edge).
fn outward(poly: &[[f64; 2]], i: usize) -> [f64; 2] {
    let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
    let len = (b[0] - a[0]).hypot(b[1] - a[1]).max(f64::MIN_POSITIVE);
    [(b[1] - a[1]) / len, -(b[0] - a[0]) / len]
}

/// `door` (with mesh `mesh`) in the frame of `building`, whose local hull is `hull` (module docs); `None` for a
/// trapdoor.
fn door_in(building: &WorldTransform, hull: &[[f64; 2]], door: &KitRef, mesh: Option<&KitMesh>, params: &KitParams) -> Option<KitDoor> {
    let (fb, fd) = (Frame::of(building), Frame::of(&door.transform));
    let pivot = kit_local(fb.to_local(fd.pos));
    let points: Vec<DVec3> = mesh.map(|m| m.points.iter().map(|p| Vec3::from_array(*p).as_dvec3()).collect()).unwrap_or_default();
    // The mesh normal: its thinnest axis in its own (scaled) frame, then in the building's kit frame.
    let mesh_normal = if points.is_empty() {
        None
    } else {
        let (mut lo, mut hi) = (DVec3::INFINITY, DVec3::NEG_INFINITY);
        for p in &points {
            lo = lo.min(fd.scale * *p);
            hi = hi.max(fd.scale * *p);
        }
        let e = hi - lo;
        let axis = if e.x <= e.y && e.x <= e.z {
            DVec3::X
        } else if e.z <= e.y {
            DVec3::Z
        } else {
            DVec3::Y
        };
        let n = kit_local(fb.rot.inverse() * (fd.rot * axis));
        let flat = n[0].hypot(n[1]);
        // A vertical normal: a trapdoor.
        if n[2].abs() > flat {
            return None;
        }
        Some([n[0] / flat, n[1] / flat])
    };
    // The door's edge: the nearest; at a corner, the nearer of the two in the mesh normal's direction.
    let p = [pivot[0], pivot[1]];
    let count = hull.len();
    let mut edges: Vec<(usize, [f64; 2], f64)> = (0..count)
        .map(|i| {
            let q = nearest_on(hull[i], hull[(i + 1) % count], p);
            (i, q, (q[0] - p[0]).hypot(q[1] - p[1]))
        })
        .collect();
    edges.sort_by(|a, b| a.2.total_cmp(&b.2).then(a.0.cmp(&b.0)));
    let mut edge = edges[0];
    if let (Some(n), Some(&second)) = (mesh_normal, edges.get(1))
        && edge.2 <= params.door_corner_m
        && second.2 <= params.door_corner_m
    {
        let agree = |i: usize| {
            let o = outward(hull, i);
            (o[0] * n[0] + o[1] * n[1]).abs()
        };
        if agree(second.0) > agree(edge.0) {
            edge = second;
        }
    }
    let (at, facing) = (edge.1, outward(hull, edge.0));
    let in_building: Vec<[f64; 3]> = points.iter().map(|p| kit_local(fb.to_local(fd.to_world(*p)))).collect();
    // Width across the door's own normal, in its plane.
    let normal = mesh_normal.unwrap_or(facing);
    let across = [-normal[1], normal[0]];
    let width = if in_building.is_empty() {
        0.0
    } else {
        let d = in_building.iter().map(|p| p[0] * across[0] + p[1] * across[1]);
        d.clone().fold(f64::NEG_INFINITY, f64::max) - d.fold(f64::INFINITY, f64::min)
    };
    let sill = in_building.iter().map(|p| p[2]).fold(f64::INFINITY, f64::min);
    Some(KitDoor {
        at,
        facing_deg: facing[1].atan2(facing[0]).to_degrees().rem_euclid(360.0),
        width_m: width,
        sill_z_m: if sill.is_finite() { sill } else { pivot[2] },
    })
}

fn rounded(d: &KitDoor) -> KitDoor {
    KitDoor { at: xy(d.at), facing_deg: round_deg(d.facing_deg), width_m: round_m(d.width_m), sill_z_m: round_m(d.sill_z_m) }
}

fn same_doors(a: &[KitDoor], b: &[KitDoor], p: &KitParams) -> bool {
    a.len() == b.len()
        && b.iter().all(|d| {
            a.iter().any(|e| {
                let turn = (d.facing_deg - e.facing_deg).rem_euclid(360.0);
                (d.at[0] - e.at[0]).hypot(d.at[1] - e.at[1]) <= p.door_agree_m
                    && (d.sill_z_m - e.sill_z_m).abs() <= p.door_agree_m
                    && turn.min(360.0 - turn) <= p.door_agree_deg
            })
        })
}

/// Each door to its nearest building reference within the radius (ties: lower key), in that building's frame; by
/// building key. Also the models of the trapdoors left out.
fn assign_doors<'r, 'm>(
    doors: &[&KitRef],
    others: &[&'r KitRef],
    hulls: &BTreeMap<&str, Vec<[f64; 2]>>,
    mesh_of: &dyn Fn(&str) -> Option<&'m KitMesh>,
    table: &KitTable,
    params: &KitParams,
) -> (BTreeMap<&'r str, Vec<LocalDoor>>, Vec<String>) {
    // Building references (by key) with their world hulls.
    let buildings: Vec<(&'r KitRef, Vec<[f64; 2]>)> = others
        .iter()
        .filter_map(|r| {
            let id = r.model.as_deref()?;
            if !table.kind_of(id).takes_doors() || !hulls.contains_key(id) {
                return None;
            }
            let m = mesh_of(id)?;
            let world: Vec<[f64; 2]> = m.points.iter().map(|p| site::to_ground(&r.transform, Vec3::from_array(*p))).collect();
            Some((*r, convex_hull(&world)?))
        })
        .collect();
    let mut doors_of: BTreeMap<&str, Vec<LocalDoor>> = BTreeMap::new();
    let mut trapdoors = Vec::new();
    for d in doors {
        let p = d.transform.pos.to_f64();
        let at = [p.x, -p.z];
        let best = buildings
            .iter()
            .map(|(b, h)| (*b, distance_to(h, at)))
            .filter(|(_, dist)| *dist <= params.door_radius_m)
            .fold(None, |best: Option<(&KitRef, f64)>, c| if best.is_none_or(|b| c.1 < b.1) { Some(c) } else { best });
        if let Some((b, _)) = best {
            let id = b.model.as_deref().unwrap_or_default();
            match door_in(&b.transform, &hulls[id], d, d.model.as_deref().and_then(mesh_of), params) {
                Some(door) => doors_of.entry(b.key.as_str()).or_default().push(LocalDoor { key: d.key.clone(), door }),
                None => trapdoors.push(d.model.clone().unwrap_or_default()),
            }
        }
    }
    (doors_of, trapdoors)
}

/// A reference's doors rounded, street door first: by sill, then position, facing and door key.
fn street_first(doors: Option<&Vec<LocalDoor>>) -> Vec<KitDoor> {
    let mut v: Vec<(KitDoor, &str)> = doors.map(|v| v.iter().map(|d| (rounded(&d.door), d.key.as_str())).collect()).unwrap_or_default();
    v.sort_by(|(a, ka), (b, kb)| {
        a.sill_z_m
            .total_cmp(&b.sill_z_m)
            .then(a.at[0].total_cmp(&b.at[0]))
            .then(a.at[1].total_cmp(&b.at[1]))
            .then(a.facing_deg.total_cmp(&b.facing_deg))
            .then(ka.cmp(kb))
    });
    v.into_iter().map(|(d, _)| d).collect()
}

/// The middle value (the mean of the two middle ones for an even count).
fn median(values: &[f64]) -> Option<f64> {
    let mut s = values.to_vec();
    s.sort_by(f64::total_cmp);
    let n = s.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(s[n / 2]),
        _ => Some(f64::midpoint(s[n / 2 - 1], s[n / 2])),
    }
}

/// Area centroid of a counter-clockwise polygon (the vertex mean when it has no area).
fn centroid(poly: &[[f64; 2]]) -> [f64; 2] {
    let count = poly.len();
    let (mut area2, mut sum) = (0.0, [0.0, 0.0]);
    for i in 0..count {
        let (p, q) = (poly[i], poly[(i + 1) % count]);
        let cross = p[0] * q[1] - q[0] * p[1];
        area2 += cross;
        sum = [sum[0] + (p[0] + q[0]) * cross, sum[1] + (p[1] + q[1]) * cross];
    }
    if area2.abs() > f64::EPSILON {
        [sum[0] / (3.0 * area2), sum[1] / (3.0 * area2)]
    } else {
        let k = count.max(1) as f64;
        [poly.iter().map(|p| p[0]).sum::<f64>() / k, poly.iter().map(|p| p[1]).sum::<f64>() / k]
    }
}

/// The ground samples under a local hull: its vertices, then the points of a grid of about `spacing` m over its
/// bounds that are inside it.
fn hull_probes(hull: &[[f64; 2]], spacing: f64) -> Vec<[f64; 2]> {
    let mut out = hull.to_vec();
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for p in hull {
        lo = [lo[0].min(p[0]), lo[1].min(p[1])];
        hi = [hi[0].max(p[0]), hi[1].max(p[1])];
    }
    let steps = |a: f64, b: f64| ((b - a) / spacing.max(f64::MIN_POSITIVE)).ceil().max(1.0) as usize;
    let (nx, ny) = (steps(lo[0], hi[0]), steps(lo[1], hi[1]));
    for j in 0..=ny {
        for i in 0..=nx {
            let p = [lo[0] + (hi[0] - lo[0]) * i as f64 / nx as f64, lo[1] + (hi[1] - lo[1]) * j as f64 / ny as f64];
            if inside(hull, p) {
                out.push(p);
            }
        }
    }
    out
}

/// The ground of reference `r` in its model's frame (terrain minus its z, over its scale): at the local `pad`
/// point, and the lowest and highest at `probes`; `None` without terrain at the pad.
fn ground_of(r: &KitRef, pad: [f64; 2], probes: &[[f64; 2]], terrain: Terrain<'_>) -> Option<(f64, f64, f64)> {
    let z = r.transform.pos.to_f64().y;
    let scale = f64::from(r.transform.scale.y);
    let at = |p: [f64; 2]| terrain(site::to_ground(&r.transform, Vec3::new(p[0] as f32, 0.0, -p[1] as f32))).map(|h| (h - z) / scale);
    let g = at(pad)?;
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for h in probes.iter().filter_map(|p| at(*p)) {
        lo = lo.min(h);
        hi = hi.max(h);
    }
    Some(if lo.is_finite() { (g, lo, hi) } else { (g, g, g) })
}

/// The ground of model `id` over its references `list`, from its local `hull` and kit `doors`: the pad out of the
/// street door, or the hull centroid.
fn model_ground(
    (id, kind): (&str, KitKind),
    list: &[&KitRef],
    hull: &[[f64; 2]],
    doors: &[KitDoor],
    terrain: Terrain<'_>,
    params: &KitParams,
) -> GroundStat {
    let mut g = GroundStat { id: id.to_owned(), kind, at_door: false, pad: Vec::new(), hull_min: Vec::new(), hull_max: Vec::new() };
    let pad = doors.first().map_or_else(
        || centroid(hull),
        |d| {
            let f = d.facing_deg.to_radians();
            [d.at[0] + params.pad_probe_m * f.cos(), d.at[1] + params.pad_probe_m * f.sin()]
        },
    );
    g.at_door = !doors.is_empty();
    let probes = hull_probes(hull, params.hull_probe_m);
    for (p, lo, hi) in list.iter().filter_map(|r| ground_of(r, pad, &probes, terrain)) {
        g.pad.push(p);
        g.hull_min.push(lo);
        g.hull_max.push(hi);
    }
    g
}

/// The kit of the anchor `name` from its member references (module docs), its ground from `terrain`;
/// deterministic for any `refs` order.
pub fn build(
    name: &str,
    producer: &str,
    refs: &[KitRef],
    meshes: Meshes<'_>,
    terrain: Terrain<'_>,
    table: &KitTable,
    params: &KitParams,
) -> Built {
    build_anchors(producer, &[(name, refs)], meshes, terrain, table, params)
}

/// The anchors' shares (kept and dropped models by `table`'s style) and their references pooled by key (the first
/// anchor's copy of a shared one), in key order.
fn pool<'r>(anchors: &[(&str, &'r [KitRef])], table: &KitTable) -> (Vec<AnchorStat>, Vec<&'r KitRef>) {
    let mut stats = Vec::with_capacity(anchors.len());
    let mut pooled: BTreeMap<&str, &KitRef> = BTreeMap::new();
    for &(name, list) in anchors {
        let models: BTreeSet<&str> = list.iter().filter(|r| !r.door).filter_map(|r| r.model.as_deref()).collect();
        let kept = models.iter().filter(|id| table.keeps(id)).count();
        stats.push(AnchorStat { name: name.to_owned(), kept, dropped: models.len() - kept, refs: list.len() });
        for r in list {
            pooled.entry(r.key.as_str()).or_insert(r);
        }
    }
    (stats, pooled.into_values().collect())
}

/// One kit from several anchors `(name, member references)`: their references pooled (a reference shared by two
/// anchors counts once), so `weight` sums the anchors' reference counts and the ground and doors come from every
/// reference by the single-anchor rules; the kit is named `A + B`. Models the table's style leaves out are not
/// entries (they still take doors, so none goes to a wrong building); deterministic for any `refs` order.
pub fn build_anchors(
    producer: &str,
    anchors: &[(&str, &[KitRef])],
    meshes: Meshes<'_>,
    terrain: Terrain<'_>,
    table: &KitTable,
    params: &KitParams,
) -> Built {
    let (anchor_stats, refs) = pool(anchors, table);
    let mut stats = KitStats { anchors: anchor_stats, ..KitStats::default() };
    let name = anchors.iter().map(|a| a.0).collect::<Vec<_>>().join(" + ");
    let name = name.as_str();
    let (doors, others): (Vec<&KitRef>, Vec<&KitRef>) = refs.into_iter().partition(|r| r.door);
    stats.door_refs = doors.len();
    // Model id -> its references (by key).
    let mut by_model: BTreeMap<&str, Vec<&KitRef>> = BTreeMap::new();
    for r in &others {
        match &r.model {
            Some(m) => by_model.entry(m.as_str()).or_default().push(r),
            None => stats.no_model += 1,
        }
        stats.scaled_refs += usize::from(r.transform.scale != Vec3::ONE);
    }
    let mut mesh_cache: BTreeMap<&str, Option<KitMesh>> = BTreeMap::new();
    for r in doors.iter().chain(&others) {
        if let Some(m) = &r.model {
            mesh_cache.entry(m.as_str()).or_insert_with(|| meshes(m));
        }
    }
    let mesh_of = |id: &str| mesh_cache.get(id).and_then(Option::as_ref);
    let ground = |m: &KitMesh| -> Vec<[f64; 2]> {
        m.points.iter().map(|p| {
            let k = kit_local(Vec3::from_array(*p).as_dvec3());
            [k[0], k[1]]
        }).collect()
    };
    // Local hulls of the usable models.
    let mut hulls: BTreeMap<&str, Vec<[f64; 2]>> = BTreeMap::new();
    for &id in by_model.keys() {
        match mesh_of(id).and_then(|m| convex_hull(&ground(m))) {
            Some(h) => {
                hulls.insert(id, h);
            }
            None => stats.no_vertices.push(id.to_owned()),
        }
    }
    let (doors_of, trapdoors) = assign_doors(&doors, &others, &hulls, &mesh_of, table, params);
    stats.doors_assigned = doors_of.values().map(Vec::len).sum();
    stats.trapdoors = trapdoors;
    let mut models = Vec::with_capacity(hulls.len());
    for (&id, list) in &by_model {
        if !table.keeps(id) {
            stats.dropped.push((id.to_owned(), list.len()));
            continue;
        }
        let (Some(hull), Some(mesh)) = (hulls.get(id), mesh_of(id)) else { continue };
        let kind = table.kind_of(id);
        let zs = mesh.points.iter().map(|p| f64::from(p[1]));
        let (zmin, zmax) = (zs.clone().fold(f64::INFINITY, f64::min), zs.fold(f64::NEG_INFINITY, f64::max));
        let per_ref: Vec<Vec<KitDoor>> = list.iter().map(|r| street_first(doors_of.get(r.key.as_str()))).collect();
        // The reference with the most doors (the first on a tie).
        let kept = (0..per_ref.len()).fold(0, |k, i| if per_ref[i].len() > per_ref[k].len() { i } else { k });
        if kind.takes_doors() {
            stats.agreement.push(DoorAgreement {
                id: id.to_owned(),
                doors_per_ref: per_ref.iter().map(Vec::len).collect(),
                agreeing: per_ref.iter().enumerate().filter(|&(i, d)| i != kept && same_doors(&per_ref[kept], d, params)).count(),
                kept,
                doors_by_ref: per_ref.clone(),
            });
        }
        let doors = per_ref.into_iter().nth(kept).unwrap_or_default();
        let g = model_ground((id, kind), list, hull, &doors, terrain, params);
        let ground_z = median(&g.pad);
        stats.ground.push(g);
        let tags: BTreeSet<String> = list.iter().map(|r| format!("base:{}", r.base)).collect();
        models.push(KitModel {
            id: id.to_owned(),
            kind,
            tags: tags.into_iter().collect(),
            roles: Vec::new(),
            footprint: hull.iter().map(|p| xy(*p)).collect(),
            obb: min_area_rect(hull),
            doors,
            height_m: round_m(zmax - zmin),
            floors: None,
            base_offset_m: round_m(zmin),
            ground_z_m: ground_z.map(round_m),
            foundation_depth_m: ground_z.map(|g| round_m(g - zmin)),
            yaw_step_deg: 0.0,
            weight: list.len() as f64,
            blake3: mesh.blake3.clone(),
        });
    }
    let kit = KitJson {
        format: FORMAT,
        name: name.to_owned(),
        producer: producer.to_owned(),
        units: "m".into(),
        frame: FRAME.into(),
        models,
    };
    Built { kit, stats }
}

/// Front and depth of a kit entry: the rectangle side the first door's edge runs along, and across it; without a
/// door the longer side and the shorter.
pub fn front_depth(m: &KitModel) -> Option<(f64, f64)> {
    let o = m.obb.as_ref()?;
    let (w, d) = (2.0 * o.half[0], 2.0 * o.half[1]);
    Some(match m.doors.first() {
        Some(door) => {
            let rel = (door.facing_deg - o.yaw_deg).to_radians();
            // Facing along the rectangle's first axis: the door is on a side of length `d`.
            if rel.cos().abs() >= rel.sin().abs() { (d, w) } else { (w, d) }
        }
        None => (w.max(d), w.min(d)),
    })
}

fn histogram(values: &[f64], bin_m: f64) -> String {
    let mut bins: BTreeMap<i64, usize> = BTreeMap::new();
    for v in values {
        *bins.entry((v / bin_m).floor() as i64).or_default() += 1;
    }
    bins.iter().map(|(b, n)| format!("{}-{}:{n}", *b as f64 * bin_m, (*b + 1) as f64 * bin_m)).collect::<Vec<_>>().join(" ")
}

/// The printed report of `built`.
pub fn report(built: &Built, params: &KitParams) -> String {
    let (kit, s) = (&built.kit, &built.stats);
    let mut out = String::new();
    let refs: f64 = kit.models.iter().map(|m| m.weight).sum();
    let _ = writeln!(out, "worldgen-kit: {} ({} models, {refs} references)", kit.name, kit.models.len());
    for a in &s.anchors {
        let _ = writeln!(out, "  anchor {}: {} references, models kept {} dropped {}", a.name, a.refs, a.kept, a.dropped);
    }
    let dropped: Vec<String> = s.dropped.iter().map(|(id, n)| format!("{} x{n}", id.rsplit('/').next().unwrap_or(id))).collect();
    let _ = writeln!(out, "  style dropped {} models: {}", dropped.len(), dropped.join(", "));
    let mut kinds: BTreeMap<KitKind, Vec<&KitModel>> = BTreeMap::new();
    for m in &kit.models {
        kinds.entry(m.kind).or_default().push(m);
    }
    let _ = writeln!(out, "  fronts and depths ({} m bins, models):", params.bin_m);
    for (kind, list) in &kinds {
        let fd: Vec<(f64, f64)> = list.iter().filter_map(|m| front_depth(m)).collect();
        let w: f64 = list.iter().map(|m| m.weight).sum();
        let _ = writeln!(out, "    {:<8} {} models, {w} refs", kind.name(), list.len());
        let _ = writeln!(out, "      fronts  {}", histogram(&fd.iter().map(|p| p.0).collect::<Vec<_>>(), params.bin_m));
        let _ = writeln!(out, "      depths  {}", histogram(&fd.iter().map(|p| p.1).collect::<Vec<_>>(), params.bin_m));
    }
    let with_doors = kit.models.iter().filter(|m| m.kind.takes_doors() && !m.doors.is_empty()).count();
    let refs_with_doors: usize = s.agreement.iter().map(|a| a.doors_per_ref.iter().filter(|&&n| n > 0).count()).sum();
    let building_refs: usize = s.agreement.iter().map(|a| a.doors_per_ref.len()).sum();
    let mut trap: BTreeMap<&str, usize> = BTreeMap::new();
    for m in &s.trapdoors {
        *trap.entry(m.rsplit('/').next().unwrap_or(m)).or_default() += 1;
    }
    let _ = writeln!(
        out,
        "  doors: {} door references, {} given to a building or tower (within {} m), {} trapdoors left out {trap:?}; building \
         and tower models with doors {with_doors} of {}, their references with doors {refs_with_doors} of {building_refs}",
        s.door_refs,
        s.doors_assigned,
        params.door_radius_m,
        s.trapdoors.len(),
        s.agreement.len()
    );
    let multi: Vec<String> = s
        .agreement
        .iter()
        .filter(|a| a.doors_per_ref.len() > 1)
        .map(|a| {
            let stem = a.id.rsplit('/').next().unwrap_or(&a.id);
            format!("{stem} {:?} agree {}/{}", a.doors_per_ref, a.agreeing, a.doors_per_ref.len() - 1)
        })
        .collect();
    let _ = writeln!(out, "  door agreement of models used more than once (doors per ref, agreeing others): {}", multi.join("; "));
    // The disagreeing ones in full (at, facing per door; references in id order, the kept one marked `*`).
    for a in s.agreement.iter().filter(|a| a.agreeing + 1 < a.doors_per_ref.len()) {
        let per_ref: Vec<String> = a
            .doors_by_ref
            .iter()
            .enumerate()
            .map(|(i, doors)| {
                let d: Vec<String> =
                    doors.iter().map(|d| format!("[{:.2}, {:.2}] z {:.2} {:.0} deg", d.at[0], d.at[1], d.sill_z_m, d.facing_deg)).collect();
                format!("{}({})", if i == a.kept { "*" } else { "" }, d.join(", "))
            })
            .collect();
        let _ = writeln!(out, "    {}: {}", a.id.rsplit('/').next().unwrap_or(&a.id), per_ref.join(" | "));
    }
    let _ = writeln!(
        out,
        "  skipped: {} models without vertices {:?}, {} references without a model; {} references of a non-unit scale",
        s.no_vertices.len(),
        s.no_vertices,
        s.no_model,
        s.scaled_refs
    );
    report_ground(&mut out, built, params);
    let _ = writeln!(out, "  models by kind:");
    for (kind, list) in &kinds {
        let names: Vec<String> = list
            .iter()
            .map(|m| format!("{} x{}", m.id.rsplit('/').next().unwrap_or(&m.id), m.weight))
            .collect();
        let _ = writeln!(out, "    {:<8} {}", kind.name(), names.join(", "));
    }
    out
}

/// The vanilla ground part of the report: the buildings' range, the models over the spread, the buildings seated at
/// their street door's sill, then one line per model.
fn report_ground(out: &mut String, built: &Built, params: &KitParams) {
    let (kit, s) = (&built.kit, &built.stats);
    let _ = writeln!(
        out,
        "  vanilla ground (model frame; median over references of the terrain {} m out of the street door, else at the \
         hull centroid):",
        params.pad_probe_m
    );
    let buildings: Vec<&KitModel> = kit.models.iter().filter(|m| m.kind == KitKind::Building).collect();
    let zs: Vec<f64> = buildings.iter().filter_map(|m| m.ground_z_m).collect();
    let (lo, hi) = (zs.iter().copied().fold(f64::INFINITY, f64::min), zs.iter().copied().fold(f64::NEG_INFINITY, f64::max));
    let seated: Vec<(f64, f64)> =
        buildings.iter().filter_map(|m| m.ground_z_m.zip(m.doors.first().map(|d| d.sill_z_m))).collect();
    let near = seated.iter().filter(|(g, sill)| (g - sill).abs() <= params.report_sill_m).count();
    let _ = writeln!(
        out,
        "    buildings: ground_z_m {lo:.2}..{hi:.2} over {} of {}; street door sill within {} m of ground_z_m: {near} of {}",
        zs.len(),
        buildings.len(),
        params.report_sill_m,
        seated.len()
    );
    let stem = |id: &str| id.rsplit('/').next().unwrap_or(id).to_owned();
    let wide: Vec<String> =
        s.ground.iter().filter(|g| g.spread() > params.report_spread_m).map(|g| format!("{} {:.2}", stem(&g.id), g.spread())).collect();
    let _ = writeln!(out, "    spread over {} m between references: {} models: {}", params.report_spread_m, wide.len(), wide.join(", "));
    for (m, g) in kit.models.iter().zip(&s.ground) {
        let (Some(z), Some(f)) = (m.ground_z_m, m.foundation_depth_m) else { continue };
        let hmin = g.hull_min.iter().copied().fold(f64::INFINITY, f64::min);
        let hmax = g.hull_max.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let sill = m.doors.first().map_or_else(String::new, |d| format!(" sill {:.2}", d.sill_z_m));
        let _ = writeln!(
            out,
            "      {:<8} {} x{}: ground {z:.2} ({}) spread {:.2} hull {hmin:.2}..{hmax:.2} base {:.2} foundation {f:.2}{sill}",
            m.kind.name(),
            stem(&m.id),
            g.pad.len(),
            if g.at_door { "door" } else { "centroid" },
            g.spread(),
            m.base_offset_m
        );
    }
}

/// The command's arguments.
#[derive(Clone, Debug)]
pub struct KitArgs {
    /// Anchor names as `anchors-census.md` gives them (ASCII case ignored), at least one; one merged kit.
    pub anchors: Vec<String>,
    pub out: PathBuf,
    /// The anchor lookup's values (`site.anchor_scale`, the anchor params).
    pub site: SiteParams,
    pub params: KitParams,
    pub table: KitTable,
}

/// The `worldgen-kit` command body; returns the printed report.
pub fn run(db: &RecordDb, bounds: Bounds<'_>, meshes: Meshes<'_>, args: &KitArgs) -> Result<String, DumpError> {
    let started = Instant::now();
    let (first, rest) = args.anchors.split_first().ok_or_else(|| DumpError::Invalid("worldgen-kit: no --anchor".into()))?;
    let site::Anchored { dumped, source, members, found: first_found } = site::anchored(db, bounds, &args.site, first)?;
    let src = site::source_field(&dumped);
    let mut found = vec![first_found];
    for q in rest {
        found.push(site::find_anchor(&dumped, &src, &members, &args.site, q)?);
    }
    let all: Vec<KitRef> = source
        .all_placements(&mw_world::ids::exterior())
        .map(|pl| KitRef {
            key: pl.id.to_string(),
            base: pl.base.id.as_str().to_owned(),
            model: pl.model.as_ref().map(|m| m.as_str().to_owned()),
            transform: pl.transform,
            door: pl.class == PlacementClass::Door,
        })
        .collect();
    let per_anchor: Vec<Vec<KitRef>> = found
        .iter()
        .map(|f| {
            let keys: BTreeSet<&str> = f.keys.iter().map(String::as_str).collect();
            all.iter().filter(|r| keys.contains(r.key.as_str())).cloned().collect()
        })
        .collect();
    let anchors: Vec<(&str, &[KitRef])> = found.iter().zip(&per_anchor).map(|(f, r)| (f.name.as_str(), r.as_slice())).collect();
    let style = args.table.style.as_ref().map_or_else(String::new, |s| format!(", style {}", s.name));
    let producer =
        format!("bevyrrowind worldgen-kit (mw_worldgen::kit, S1){style}, exterior of {}", dumped.plugins.join(" + "));
    let terrain = |p: [f64; 2]| Some(src.bilinear(p[0], p[1]));
    let built = build_anchors(&producer, &anchors, meshes, &terrain, &args.table, &args.params);
    write(&built.kit, &args.out)?;
    let mut out = report(&built, &args.params);
    let _ = writeln!(out, "wrote {} in {:.1} s", args.out.join(KIT_FILE).display(), started.elapsed().as_secs_f64());
    Ok(out)
}
