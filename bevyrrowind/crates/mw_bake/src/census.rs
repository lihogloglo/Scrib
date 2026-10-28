//! Furniture and race census (`docs/specs/motion-generation.md`, "Furniture
//! and race census (MG4)"): the seat, table, counter, bed and shelf heights
//! of the user's install, weighted by how often the world places them, so
//! the motion-generation planner targets the heights players see.
//!
//! Per furniture record (`STAT`, `ACTI`, `CONT` whose id or model matches
//! [`classify`]): its static collision ([`collision_of`], the bake's rules)
//! as horizontal surfaces above the model's floor (its lowest point). The
//! measured height is the highest surface with a footprint of at least
//! [`MIN_EXTENT`] in both directions, between 0.1 m (floor plates, box
//! bottoms) and the kind's cap ([`Kind::cap`]; seats: 0.8 m). Models whose collision has no such surface (a chair
//! collided as one tall box) fall back to their render geometry. Every
//! placement (`CELL` references, `XSCL` applied) is one sample; samples
//! cluster per kind into at most [`MAX_BANDS`] bands ([`bands`]).

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use glam::{Affine3A, Vec3};
use mw_nif::NodeKind;
use rayon::prelude::*;
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Activator, Cell, Container, Race, RaceFlags, Static};

use crate::collision::collision_of;

/// Smallest side of a surface that counts (a seat a pelvis fits on), meters.
pub const MIN_EXTENT: f32 = 0.25;
/// Most bands per kind.
pub const MAX_BANDS: usize = 4;
/// Histogram bin of the samples, meters.
pub const BIN: f32 = 0.05;
/// Bands closer than this merge, meters.
const MERGE_GAP: f32 = 0.10;
/// ... unless the merged band would span more than this, meters.
const MAX_SPAN: f32 = 0.15;
/// Least area over the surface's bounding rectangle: two armrests in one
/// height bin span a seat but don't fill it.
const MIN_FILL: f32 = 0.3;
/// Surfaces lower than this are the floor plate or a box bottom, meters.
const MIN_HEIGHT: f32 = 0.1;
/// Bands with a smaller share of a kind's placements merge into a neighbor.
const MIN_SHARE: f32 = 0.03;
/// A triangle is horizontal when `|normal.y|` is at least this (~25 degrees).
const FLAT: f32 = 0.9;
/// Surface height bin before merging neighbors, meters.
const SURFACE_BIN: f32 = 0.02;

/// What a piece of furniture is used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Seat,
    Table,
    Counter,
    Bed,
    Shelf,
}

impl Kind {
    pub const ALL: [Self; 5] = [
        Self::Seat,
        Self::Table,
        Self::Counter,
        Self::Bed,
        Self::Shelf,
    ];

    /// The key in `bands.json` (`tools/motiongen/default_bands.json`).
    pub fn key(self) -> &'static str {
        match self {
            Self::Seat => "seat",
            Self::Table => "table",
            Self::Counter => "counter",
            Self::Bed => "bed",
            Self::Shelf => "shelf",
        }
    }

    /// Highest surface this kind measures (above it: backrests, canopies,
    /// hutches), meters above the floor.
    pub fn cap(self) -> f32 {
        match self {
            Self::Seat => 0.8,
            Self::Table | Self::Bed => 1.2,
            Self::Counter => 1.5,
            Self::Shelf => 2.5,
        }
    }
}

/// The kind of a furniture record from its id or, failing that, its model's
/// file name: `furn_*` / `active_*` ids (and `furn_*` models) whose
/// `_`-separated words (trailing digits dropped) name a bed, counter,
/// table, seat or shelf, in that order (`furn_de_bar_01` is a counter,
/// `furn_de_r_bench_01` a seat, `active_de_bedroll` a bed). Words, not
/// substrings: `barrel` is not a `bar`.
pub fn classify(id: &str, model: Option<&str>) -> Option<Kind> {
    fn words(name: &str) -> Option<Vec<String>> {
        let lower = name.to_ascii_lowercase();
        let words: Vec<String> = lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .map(|w| w.trim_end_matches(|c: char| c.is_ascii_digit()).to_owned())
            .filter(|w| !w.is_empty())
            .collect();
        matches!(words.first().map(String::as_str), Some("furn" | "active")).then_some(words)
    }
    fn kind(words: &[String]) -> Option<Kind> {
        let any = |names: &[&str]| words.iter().any(|w| names.contains(&w.as_str()));
        if any(&["door"]) {
            return None;
        }
        if any(&["bed", "beds", "bedroll", "hammock", "cot", "bunk"]) {
            Some(Kind::Bed)
        } else if any(&["counter", "bar", "bartop"]) {
            Some(Kind::Counter)
        } else if any(&["table", "desk"]) {
            Some(Kind::Table)
        } else if any(&["chair", "stool", "bench", "seat", "throne", "pew"]) {
            Some(Kind::Seat)
        } else if any(&["shelf", "shelves", "bookshelf", "bookcase"]) {
            Some(Kind::Shelf)
        } else {
            None
        }
    }
    let stem = |m: &str| {
        m.rsplit(['/', '\\'])
            .next()
            .unwrap_or(m)
            .split('.')
            .next()
            .unwrap_or(m)
            .to_owned()
    };
    words(id).and_then(|w| kind(&w)).or_else(|| {
        model
            .map(stem)
            .and_then(|s| words(&s))
            .and_then(|w| kind(&w))
    })
}

/// A horizontal surface of a model, above its floor.
#[derive(Clone, Copy, Debug, Default)]
pub struct Surface {
    /// Height above the model's floor, meters.
    pub height: f32,
    /// Top-facing (or bottom-facing) area, square meters.
    pub area: f32,
    /// Extent along engine x and z, meters.
    pub extent: [f32; 2],
}

/// A model's measured shape.
#[derive(Clone, Debug, Default)]
pub struct Shape {
    /// `"collision"` or `"render"` (the fallback).
    pub source: &'static str,
    /// Overall height (top minus floor), meters.
    pub height: f32,
    /// Overall extent along x and z, meters.
    pub footprint: [f32; 2],
    /// Low to high.
    pub surfaces: Vec<Surface>,
}

impl Shape {
    /// The highest usable surface at or below `cap`.
    pub fn top_below(&self, cap: f32) -> Option<f32> {
        self.surfaces
            .iter()
            .filter(|s| {
                (MIN_HEIGHT..=cap).contains(&s.height)
                    && s.extent[0].min(s.extent[1]) >= MIN_EXTENT
                    && s.area >= MIN_EXTENT * MIN_EXTENT * 0.5
                    && s.area >= MIN_FILL * s.extent[0] * s.extent[1]
            })
            .map(|s| s.height)
            .reduce(f32::max)
    }
}

/// Horizontal surfaces of a triangle soup (engine space, y up). `None`
/// without triangles.
pub fn shape(triangles: &[[Vec3; 3]], source: &'static str) -> Option<Shape> {
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    for p in triangles.iter().flatten() {
        lo = lo.min(*p);
        hi = hi.max(*p);
    }
    if triangles.is_empty() || !lo.is_finite() {
        return None;
    }
    // Per height bin: area, x/z bounds.
    let mut bins: BTreeMap<i32, (f32, f32, [f32; 4])> = BTreeMap::new();
    for [a, b, c] in triangles {
        let cross = (*b - *a).cross(*c - *a);
        let area = cross.length() * 0.5;
        if area < 1e-6 || (cross.y.abs() / (2.0 * area)) < FLAT {
            continue;
        }
        let y = (a.y + b.y + c.y) / 3.0 - lo.y;
        #[allow(
            clippy::cast_possible_truncation,
            reason = "a model is far below i32::MAX bins tall"
        )]
        let key = (y / SURFACE_BIN).round() as i32;
        let entry = bins.entry(key).or_insert((
            0.0,
            0.0,
            [
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ],
        ));
        entry.0 += area;
        entry.1 += area * y;
        for p in [a, b, c] {
            entry.2 = [
                entry.2[0].min(p.x),
                entry.2[1].max(p.x),
                entry.2[2].min(p.z),
                entry.2[3].max(p.z),
            ];
        }
    }
    // Runs of neighboring bins are one surface (sloped cushions, welds).
    let mut surfaces: Vec<Surface> = Vec::new();
    let mut run: Option<(i32, f32, f32, [f32; 4])> = None;
    let flush = |run: &mut Option<(i32, f32, f32, [f32; 4])>, out: &mut Vec<Surface>| {
        if let Some((_, area, weighted, b)) = run.take() {
            out.push(Surface {
                height: weighted / area,
                area,
                extent: [b[1] - b[0], b[3] - b[2]],
            });
        }
    };
    for (key, (area, weighted, b)) in bins {
        match &mut run {
            Some((last, a, w, rb)) if key - *last <= 1 => {
                *last = key;
                *a += area;
                *w += weighted;
                *rb = [
                    rb[0].min(b[0]),
                    rb[1].max(b[1]),
                    rb[2].min(b[2]),
                    rb[3].max(b[3]),
                ];
            }
            _ => {
                flush(&mut run, &mut surfaces);
                run = Some((key, area, weighted, b));
            }
        }
    }
    flush(&mut run, &mut surfaces);
    Some(Shape {
        source,
        height: hi.y - lo.y,
        footprint: [hi.x - lo.x, hi.z - lo.z],
        surfaces,
    })
}

/// A model's collision shape and its render-geometry shape (visible nodes
/// outside collision subtrees).
pub fn model_shapes(nif: &mw_nif::Model) -> Shapes {
    let (_, mesh) = collision_of(nif, crate::collision::MwCollisionRules::default().weld);
    let collision: Vec<[Vec3; 3]> = mesh
        .triangles
        .iter()
        .map(|t| t.map(|i| Vec3::from_array(mesh.positions[i as usize])))
        .collect();
    let world = mw_nif::world_transforms(nif);
    let mut hidden: Vec<bool> = Vec::with_capacity(nif.nodes.len());
    for node in &nif.nodes {
        let parent = node
            .parent
            .and_then(|p| hidden.get(p).copied())
            .unwrap_or(false);
        hidden.push(
            parent
                || !node.visible
                || matches!(
                    node.kind,
                    NodeKind::Collision | NodeKind::Particles | NodeKind::Billboard
                ),
        );
    }
    let mut render = Vec::new();
    for (i, node) in nif.nodes.iter().enumerate() {
        let Some(mesh) = node.mesh.and_then(|m| nif.meshes.get(m)) else {
            continue;
        };
        if hidden[i] {
            continue;
        }
        let transform = Affine3A::from_cols_array(&world[i].to_cols_array());
        let point =
            |i: u32| transform.transform_point3(Vec3::from_array(mesh.positions[i as usize]));
        render.extend(
            mesh.indices
                .as_chunks::<3>()
                .0
                .iter()
                .map(|t| [point(t[0]), point(t[1]), point(t[2])]),
        );
    }
    (shape(&collision, "collision"), shape(&render, "render"))
}

/// A model's collision and render shapes ([`model_shapes`]).
pub type Shapes = (Option<Shape>, Option<Shape>);

/// How a record is placed in the world.
#[derive(Clone, Debug, Default)]
pub struct Placements {
    pub interior: u32,
    pub exterior: u32,
    /// `XSCL` in hundredths (100 = unscaled) -> placements.
    pub scales: BTreeMap<i32, u32>,
}

impl Placements {
    pub fn total(&self) -> u32 {
        self.interior + self.exterior
    }
}

/// One furniture record.
#[derive(Clone, Debug)]
pub struct Piece {
    pub id: String,
    /// `"STAT"`, `"ACTI"` or `"CONT"`.
    pub record: &'static str,
    pub kind: Kind,
    pub model: String,
    /// The shape measured (collision, else render).
    pub shape: Option<Shape>,
    /// Seat or top height at scale 1, meters; `None` when nothing qualifies.
    pub height: Option<f32>,
    pub placements: Placements,
}

/// A race's height factors (`RADT`).
#[derive(Clone, Debug)]
pub struct RaceHeight {
    pub id: String,
    pub name: String,
    pub playable: bool,
    pub beast: bool,
    pub male: f32,
    pub female: f32,
}

/// A cluster of one kind's placements.
#[derive(Clone, Debug)]
pub struct Band {
    pub kind: Kind,
    /// Placement-weighted mean height, meters.
    pub center: f32,
    pub min: f32,
    pub max: f32,
    /// Distinct records.
    pub members: usize,
    pub refs: u32,
    /// The most placed members, up to three.
    pub examples: Vec<String>,
}

/// The whole census.
#[derive(Clone, Debug, Default)]
pub struct Census {
    pub pieces: Vec<Piece>,
    pub races: Vec<RaceHeight>,
    pub bands: Vec<Band>,
    /// Model path -> load error.
    pub errors: Vec<(String, String)>,
}

/// Every reference in every cell, by lowercase base id.
fn placements(db: &RecordDb) -> HashMap<String, Placements> {
    let mut out: HashMap<String, Placements> = HashMap::new();
    for (_, cell) in db.iter::<Cell>() {
        for r in cell.references.values().filter(|r| r.deleted != Some(true)) {
            let entry = out.entry(r.id.to_ascii_lowercase()).or_default();
            if cell.is_interior() {
                entry.interior += 1;
            } else {
                entry.exterior += 1;
            }
            #[allow(clippy::cast_possible_truncation, reason = "XSCL is 0.5..2")]
            let scale = (r.scale.unwrap_or(1.0) * 100.0).round() as i32;
            *entry.scales.entry(scale).or_default() += 1;
        }
    }
    out
}

/// Runs the census over loaded records and their VFS.
pub fn run(db: &RecordDb, vfs: &Vfs) -> Census {
    let mut found: Vec<(String, &'static str, Kind, String)> = Vec::new();
    let mut add = |id: &str, record: &'static str, mesh: &str| {
        let Some(model) = mw_world::model_path(mesh) else {
            return;
        };
        if let Some(kind) = classify(id, Some(&model)) {
            found.push((id.to_ascii_lowercase(), record, kind, model));
        }
    };
    for (id, r) in db.iter::<Static>() {
        add(id.as_str(), "STAT", &r.mesh);
    }
    for (id, r) in db.iter::<Activator>() {
        add(id.as_str(), "ACTI", &r.mesh);
    }
    for (id, r) in db.iter::<Container>() {
        add(id.as_str(), "CONT", &r.mesh);
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));

    let models: Vec<String> = {
        let mut m: Vec<String> = found.iter().map(|f| f.3.to_ascii_lowercase()).collect();
        m.sort();
        m.dedup();
        m
    };
    let measured: Vec<(String, Result<Shapes, String>)> = models
        .into_par_iter()
        .map(|path| {
            let shapes = VfsPath::new(&path)
                .map_err(|e| e.to_string())
                .and_then(|p| vfs.read(&p).map_err(|e| e.to_string()))
                .and_then(|bytes| mw_nif::load(&bytes).map_err(|e| e.to_string()))
                .map(|nif| model_shapes(&nif));
            (path, shapes)
        })
        .collect();
    let mut census = Census::default();
    let mut shapes: HashMap<String, Shapes> = HashMap::new();
    for (path, result) in measured {
        match result {
            Ok(s) => {
                shapes.insert(path, s);
            }
            Err(e) => census.errors.push((path, e)),
        }
    }

    let mut refs = placements(db);
    for (id, record, kind, model) in found {
        let (collision, render) = shapes
            .get(&model.to_ascii_lowercase())
            .cloned()
            .unwrap_or_default();
        let from = |s: Option<Shape>| s.and_then(|s| s.top_below(kind.cap()).map(|h| (s, h)));
        let (shape, height) = match from(collision.clone()).or_else(|| from(render)) {
            Some((s, h)) => (Some(s), Some(h)),
            None => (collision, None),
        };
        let placements = refs.remove(&id).unwrap_or_default();
        census.pieces.push(Piece {
            id,
            record,
            kind,
            model,
            shape,
            height,
            placements,
        });
    }

    for (_, race) in db.iter::<Race>() {
        census.races.push(RaceHeight {
            id: race.id.clone(),
            name: race.name.clone(),
            playable: race.data.flags.contains(RaceFlags::PLAYABLE),
            beast: race.data.flags.contains(RaceFlags::BEAST_RACE),
            male: race.data.height[0],
            female: race.data.height[1],
        });
    }
    census.races.sort_by(|a, b| a.id.cmp(&b.id));
    census.bands = Kind::ALL
        .into_iter()
        .flat_map(|k| bands(&census.pieces, k))
        .collect();
    census
}

/// One kind's placements clustered: 5 cm bins, then neighbors merged
/// greedily (closest centers first) while under 10 cm apart and the merged
/// band spans at most 15 cm; then bands under 3 % of the placements fold
/// into their nearest neighbor, and the closest merge while more than
/// [`MAX_BANDS`] remain. Unplaced records join with no weight.
pub fn bands(pieces: &[Piece], kind: Kind) -> Vec<Band> {
    // (piece index, height, weight)
    let mut samples: Vec<(usize, f32, u32)> = Vec::new();
    for (i, p) in pieces.iter().enumerate().filter(|(_, p)| p.kind == kind) {
        let Some(h) = p.height else { continue };
        if p.placements.scales.is_empty() {
            samples.push((i, h, 0));
        }
        #[allow(clippy::cast_precision_loss, reason = "hundredths of a scale")]
        samples.extend(
            p.placements
                .scales
                .iter()
                .map(|(&s, &n)| (i, h * s as f32 / 100.0, n)),
        );
    }
    let mut groups: BTreeMap<i32, Vec<(usize, f32, u32)>> = BTreeMap::new();
    for s in samples {
        #[allow(clippy::cast_possible_truncation, reason = "heights are meters")]
        groups
            .entry((s.1 / BIN).floor() as i32)
            .or_default()
            .push(s);
    }
    let mut groups: Vec<Vec<(usize, f32, u32)>> = groups.into_values().collect();
    let weight = |g: &[(usize, f32, u32)]| g.iter().map(|s| s.2).sum::<u32>();
    let center = |g: &[(usize, f32, u32)]| {
        let w = weight(g);
        #[allow(clippy::cast_precision_loss, reason = "placement counts")]
        if w == 0 {
            g.iter().map(|s| s.1).sum::<f32>() / g.len() as f32
        } else {
            g.iter().map(|s| s.1 * s.2 as f32).sum::<f32>() / w as f32
        }
    };
    #[allow(clippy::cast_precision_loss, reason = "placement counts")]
    let total = groups.iter().map(|g| weight(g)).sum::<u32>() as f32;
    while groups.len() > 1 {
        let centers: Vec<f32> = groups.iter().map(|g| center(g)).collect();
        let closest = centers
            .windows(2)
            .map(|w| w[1] - w[0])
            .enumerate()
            .fold(
                (0, f32::INFINITY),
                |a, (i, g)| if g < a.1 { (i, g) } else { a },
            )
            .0;
        let lo = |g: &[(usize, f32, u32)]| g.iter().map(|s| s.1).fold(f32::INFINITY, f32::min);
        let hi = |g: &[(usize, f32, u32)]| g.iter().map(|s| s.1).fold(f32::NEG_INFINITY, f32::max);
        let tight = (0..groups.len() - 1)
            .filter(|&i| {
                centers[i + 1] - centers[i] < MERGE_GAP
                    && hi(&groups[i + 1]) - lo(&groups[i]) <= MAX_SPAN
            })
            .min_by(|&a, &b| {
                (centers[a + 1] - centers[a]).total_cmp(&(centers[b + 1] - centers[b]))
            });
        #[allow(clippy::cast_precision_loss, reason = "placement counts")]
        let small = (0..groups.len())
            .filter(|&i| total > 0.0 && (weight(&groups[i]) as f32) < MIN_SHARE * total)
            .min_by_key(|&i| weight(&groups[i]))
            .map(|i| {
                let left = if i > 0 {
                    centers[i] - centers[i - 1]
                } else {
                    f32::INFINITY
                };
                let right = centers.get(i + 1).map_or(f32::INFINITY, |c| c - centers[i]);
                if left <= right { i - 1 } else { i }
            });
        let merge = tight
            .or(small)
            .or_else(|| (groups.len() > MAX_BANDS).then_some(closest));
        let Some(i) = merge else { break };
        let next = groups.remove(i + 1);
        groups[i].extend(next);
    }
    groups
        .iter()
        .map(|g| {
            let mut by_record: BTreeMap<usize, u32> = BTreeMap::new();
            for s in g {
                *by_record.entry(s.0).or_default() += s.2;
            }
            let mut ranked: Vec<(usize, u32)> = by_record.into_iter().collect();
            ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            Band {
                kind,
                center: center(g),
                min: g.iter().map(|s| s.1).fold(f32::INFINITY, f32::min),
                max: g.iter().map(|s| s.1).fold(f32::NEG_INFINITY, f32::max),
                members: ranked.len(),
                refs: weight(g),
                examples: ranked
                    .iter()
                    .take(3)
                    .map(|r| pieces[r.0].id.clone())
                    .collect(),
            }
        })
        .collect()
}

impl Census {
    /// `bands.json` for the planner (`tools/motiongen/recipe.py`,
    /// `load_bands`): the keys of `default_bands.json`, a `ground` band at
    /// 0, race heights in the note.
    pub fn bands_json(&self) -> String {
        let mut points = serde_json::Map::new();
        for kind in Kind::ALL {
            let rows: Vec<serde_json::Value> = self
                .bands
                .iter()
                .filter(|b| b.kind == kind)
                .map(|b| {
                    let cm = (b.center * 100.0).round();
                    serde_json::json!({ "id": format!("{}{cm}", kind.key()), "height": f64::from(cm) / 100.0, "refs": b.refs })
                })
                .collect();
            if !rows.is_empty() {
                points.insert(kind.key().to_owned(), rows.into());
            }
        }
        points.insert(
            "ground".into(),
            serde_json::json!([{ "id": "ground0", "height": 0.0, "refs": 0 }]),
        );
        let races: Vec<String> = self
            .races
            .iter()
            .filter(|r| r.playable)
            .map(|r| format!("{} {:.2}/{:.2}", r.name, r.male, r.female))
            .collect();
        let note = format!(
            "Measured by `bevyrrowind furniture-census`: meters above the floor, one sample per placement (XSCL applied). \
             Race height factors (male/female, x the base skeleton): {}.",
            races.join(", ")
        );
        let json = serde_json::json!({ "placeholder": false, "note": note, "reference_height_m": null, "points": points });
        serde_json::to_string_pretty(&json).unwrap_or_default() + "\n"
    }
}

impl fmt::Display for Census {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Races (height factor male / female)")?;
        for r in &self.races {
            let tags = match (r.playable, r.beast) {
                (true, true) => "playable, beast",
                (true, false) => "playable",
                (false, true) => "beast",
                (false, false) => "",
            };
            writeln!(
                f,
                "  {:<24} {:<20} {:.2} / {:.2}  {tags}",
                r.id, r.name, r.male, r.female
            )?;
        }
        for kind in Kind::ALL {
            self.kind_table(f, kind)?;
        }
        let mut scales: BTreeMap<i32, u32> = BTreeMap::new();
        for p in &self.pieces {
            for (&s, &n) in &p.placements.scales {
                *scales.entry(s.div_euclid(5) * 5).or_default() += n;
            }
        }
        writeln!(f, "\nXSCL over furniture placements (5 % bins)")?;
        for (s, n) in &scales {
            writeln!(
                f,
                "  {:.2}-{:.2}  {n}",
                f64::from(*s) / 100.0,
                f64::from(*s + 5) / 100.0
            )?;
        }
        writeln!(f, "\nBands (placement-weighted)")?;
        writeln!(
            f,
            "  {:<8} center  range          members  refs  examples",
            "kind"
        )?;
        for b in &self.bands {
            writeln!(
                f,
                "  {:<8} {:6.3}  {:5.3}-{:5.3}  {:>7}  {:>5}  {}",
                b.kind.key(),
                b.center,
                b.min,
                b.max,
                b.members,
                b.refs,
                b.examples.join(", ")
            )?;
        }
        for (path, e) in self.errors.iter().take(5) {
            writeln!(f, "error: {path}: {e}")?;
        }
        Ok(())
    }
}

impl Census {
    /// One kind's records, most placed first.
    fn kind_table(&self, f: &mut fmt::Formatter<'_>, kind: Kind) -> fmt::Result {
        let mut of: Vec<&Piece> = self.pieces.iter().filter(|p| p.kind == kind).collect();
        of.sort_by(|a, b| {
            b.placements
                .total()
                .cmp(&a.placements.total())
                .then(a.id.cmp(&b.id))
        });
        let refs: u32 = of.iter().map(|p| p.placements.total()).sum();
        let interior: u32 = of.iter().map(|p| p.placements.interior).sum();
        let unmeasured: Vec<&str> = of
            .iter()
            .filter(|p| p.height.is_none())
            .map(|p| p.id.as_str())
            .collect();
        writeln!(
            f,
            "\n{} : {} records, {refs} placements ({interior} interior), {} unmeasured",
            kind.key(),
            of.len(),
            unmeasured.len()
        )?;
        writeln!(
            f,
            "  {:<32} rec  src        height  overall  footprint    refs (int)",
            "id"
        )?;
        for p in of.iter().take(15) {
            let (src, overall, fp) = p.shape.as_ref().map_or(("-", 0.0, [0.0, 0.0]), |s| {
                (s.source, s.height, s.footprint)
            });
            let h = p
                .height
                .map_or_else(|| "   -  ".to_owned(), |h| format!("{h:6.3}"));
            writeln!(
                f,
                "  {:<32} {} {src:<10} {h}  {overall:6.3}  {:4.2} x {:4.2}  {:>5} ({})",
                p.id,
                p.record,
                fp[0],
                fp[1],
                p.placements.total(),
                p.placements.interior
            )?;
        }
        if of.len() > 15 {
            writeln!(f, "  ... {} more", of.len() - 15)?;
        }
        if !unmeasured.is_empty() {
            writeln!(
                f,
                "  unmeasured: {}",
                unmeasured
                    .iter()
                    .take(8)
                    .copied()
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_by_words() {
        assert_eq!(classify("furn_de_p_chair_01", None), Some(Kind::Seat));
        assert_eq!(classify("furn_com_rm_table_03", None), Some(Kind::Table));
        assert_eq!(classify("active_de_bed_29", None), Some(Kind::Bed));
        assert_eq!(classify("furn_de_bar_01", None), Some(Kind::Counter));
        assert_eq!(classify("furn_de_r_bench_01", None), Some(Kind::Seat));
        assert_eq!(classify("furn_barrel_01", None), None);
        assert_eq!(classify("in_de_bar_01", None), None);
        assert_eq!(classify("active_de_bar_door", None), None);
        assert_eq!(
            classify("xyz", Some("meshes/f/furn_com_chair01.nif")),
            Some(Kind::Seat)
        );
    }

    fn quad(y: f32, x0: f32, x1: f32) -> [[Vec3; 3]; 2] {
        let corners = [
            Vec3::new(x0, y, 0.0),
            Vec3::new(x1, y, 0.0),
            Vec3::new(x1, y, -0.4),
            Vec3::new(x0, y, -0.4),
        ];
        [
            [corners[0], corners[1], corners[2]],
            [corners[0], corners[2], corners[3]],
        ]
    }

    #[test]
    fn seat_is_the_wide_surface_under_the_cap() {
        // Floor at 0.1 (model origin below the floor), seat 0.45 above it,
        // a narrow backrest top, a wide canopy at 1.1 m.
        let mut tris: Vec<[Vec3; 3]> = Vec::new();
        tris.extend(quad(0.1, 0.0, 0.05));
        tris.extend(quad(0.55, 0.0, 0.4));
        tris.extend(quad(1.0, 0.0, 0.05));
        tris.extend(quad(1.2, 0.0, 0.4));
        let s = shape(&tris, "collision").unwrap();
        assert!((s.top_below(Kind::Seat.cap()).unwrap() - 0.45).abs() < 1e-4);
        assert!((s.top_below(Kind::Bed.cap()).unwrap() - 1.1).abs() < 1e-4);
    }
}
