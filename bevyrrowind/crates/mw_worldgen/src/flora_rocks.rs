//! The rock layer's adapter half (spec `worldgen-scatter.md` "Rocks (WG-E3)", "Adapter data"; slices WG-E3f.*): the
//! source's learned rock models as pack-model entries of the rock units ([`rock_entries`]) and the field-stone region
//! prior ([`rock_prior`]).
//!
//! **Model entries** (owner 2026-11-26: the source's rocks stay beside the library). Every learned rock model
//! ([`Learned::rocks`]: one per rock base record, `terrain_rock*` and rocks of 14 m or more) becomes one
//! `{ kind = "model" }` entry in every context a rock unit already lists (a unit without an outcrop context gets no
//! outcrop model): `model` = its base record id (unique per learned model, where two records may share a mesh; the
//! game resolves it to the record's mesh), `axis_m` = the longest edge of its bounds, `[scale_min, scale_max]` = the
//! scale range of its references, and as weight its learned count in the unit's regions: `w(u) = sum over regions r
//! of count(r) x share(r, u)`, `share` the region's `geology.toml` unit share (its `[region."<name>"]` row, else the
//! table's default `[shares]`, normalised). The scatter's entries are per unit, not per region, so the region's
//! count reaches the unit through the shares the geology map itself draws its units from. Model entries are added to
//! the unit's own lists only: a regime override list (RG-9j) replaces them, as it replaces the classes.
//!
//! **Ground** ([`Ground`], the scatter's [`RockGround`], spec "Rocks (WG-E3)" "Inputs"): unit and cliff from the
//! guide's `geology/` product ([`GeologyGrid`], nearest sample) when the guide has one, else no cliff and per region
//! its largest [`unit_shares`] unit (ties: the first name); slope and aspect from the guide heights (central
//! differences one sample apart, [`Grid::gradient`]); height the final ground ([`World::height`]), which the rocks
//! stand on.
//!
//! **Wiring** (WG-E3f.4, [`Rocks`], [`place_rock`]): with a [`RockSource`] the passes run the rock layer first
//! (`RockLayer` in the candidate rules; the field-stone prior per region where the source learned any rock density,
//! else none: the authored `field_stone_per_ha`), the budget cuts rocks as its group 0, and each kept rock takes its
//! mesh and pose at load (`scatter_rocks::pick`). A pack model becomes a [`Placement`] copying its learned reference
//! (group `rocks` of the content's extra passes), seated by its learned origin offset (`offset_m x scale`, as the
//! plants: a pack mesh's origin is its own, so a share of its short axis would not seat it); a library class stays a
//! [`RockPlacement`] in the run (class, variant hash, size mark, transform, sink share), which the streamer reads
//! once RG-P lands.

use std::collections::BTreeMap;
use std::path::Path;

use glam::{DVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_world::{Placement, PlacementId};
use sf_worldgen::geology::{GeologyGrid, GeologyTable};
use sf_worldgen::scatter::Params;
use sf_worldgen::scatter_rocks::{RockCandidate, RockGround, RockKey, RockValues, pick};
use sf_worldgen::scatter_species::{RockContext, RockEntry, SpeciesFile};

use super::{FloraRun, Grid, Learned, World};
use crate::{DumpError, RegionMap};

/// The shipped geology table (`bevyrrowind/worldgen/geology.toml`); a guide folder's own copy wins
/// ([`RockSource::read`]).
pub const SHIPPED_GEOLOGY: &str = include_str!("../../../worldgen/geology.toml");

/// Namespace of the scattered rocks' placement ids (never a content key).
// tunables: const (format: an id namespace)
const ROCK_ID_NAMESPACE: &str = "mw_worldgen.flora.rocks";

/// Per legend index of `regions`: the unit shares of its `[region."<name>"]` row (by the legend's `name`), else the
/// table's default `[shares]`, normalised to sum 1 (no share at all when they sum to 0).
#[must_use]
pub fn unit_shares(regions: &RegionMap, geology: &GeologyTable) -> Vec<BTreeMap<String, f64>> {
    regions
        .legend
        .iter()
        .map(|e| {
            let raw = geology.region.get(&e.name).map_or(&geology.shares, |r| &r.shares);
            let sum: f64 = raw.values().filter(|v| v.is_finite() && **v > 0.0).sum();
            if sum <= 0.0 {
                return BTreeMap::new();
            }
            raw.iter().filter(|(_, v)| v.is_finite() && **v > 0.0).map(|(u, v)| (u.clone(), v / sum)).collect()
        })
        .collect()
}

/// Adds the learned rock models of `learned` to its file's rock units as pack-model entries ([module docs](self)),
/// in base-record order; returns the entries added. A model whose bounds or scales are not positive, or whose weight
/// in a unit is 0, is left out of it.
pub fn rock_entries(learned: &mut Learned, regions: &RegionMap, geology: &GeologyTable) -> usize {
    let shares = unit_shares(regions, geology);
    let mut order: Vec<usize> = (0..learned.rocks.len()).collect();
    order.sort_by(|&a, &b| learned.reps[learned.rocks[a].rep].key.cmp(&learned.reps[learned.rocks[b].rep].key));
    let mut added = 0;
    for i in order {
        let s = &learned.rocks[i];
        let ok = |v: f32| v.is_finite() && v > 0.0;
        if !(ok(s.extent_m) && ok(s.scale[0]) && ok(s.scale[1])) {
            continue;
        }
        let model = learned.reps[s.rep].key.clone();
        for (name, unit) in &mut learned.file.rock_units {
            let weight: f64 = s
                .per_region
                .iter()
                .enumerate()
                .map(|(r, &n)| n as f64 * shares.get(r).and_then(|m| m.get(name)).copied().unwrap_or(0.0))
                .sum();
            if !(weight.is_finite() && weight > 0.0) {
                continue;
            }
            for context in [RockContext::Talus, RockContext::Outcrop, RockContext::Field] {
                let Some(list) = unit.entries.get_mut(&context).filter(|l| !l.is_empty()) else { continue };
                list.push(RockEntry::Model {
                    model: model.clone(),
                    axis_m: f64::from(s.extent_m),
                    scale_min: f64::from(s.scale[0]),
                    scale_max: f64::from(s.scale[1]),
                    weight,
                });
                added += 1;
            }
        }
    }
    added
}

/// The field-stone region prior (spec "Adapter data": learned from the source's rock references): per region,
/// its rock density `D(r)` ([`Learned::rock_density`]) over the land-weighted mean `M = sum D(r) a(r) / sum a(r)`,
/// `land[r]` = `a(r)` the guide land samples of the region (0 everywhere when `M` is 0). Its land-weighted mean is
/// 1, so a unit keeps its authored `field_stone_per_ha` on average; the scatter clamps it to
/// `scatter.region_prior_max` as the plants' prior.
#[must_use]
pub fn rock_prior(density: &[f64], land: &[u64]) -> Vec<f64> {
    let (mut num, mut den) = (0.0, 0.0);
    for (r, &a) in land.iter().enumerate() {
        num += density.get(r).copied().unwrap_or(0.0) * a as f64;
        den += a as f64;
    }
    let mean = if den > 0.0 { num / den } else { 0.0 };
    (0..land.len()).map(|r| if mean > 0.0 { density.get(r).copied().unwrap_or(0.0) / mean } else { 0.0 }).collect()
}

/// The geology product of a guide folder (`dir/geology/`), when it has one.
///
/// # Errors
/// A product that is present (its `meta.json` exists) but unreadable.
pub fn guide_geology(dir: &Path) -> Result<Option<GeologyGrid>, DumpError> {
    if !dir.join("geology").join("meta.json").exists() {
        return Ok(None);
    }
    GeologyGrid::read(dir).map(Some).map_err(|e| DumpError::Invalid(format!("{}: {e}", dir.join("geology").display())))
}

/// The game's rock ground ([module docs](self)).
pub struct Ground<'a> {
    world: &'a World<'a>,
    geology: Option<&'a GeologyGrid>,
    /// Per region legend index, its default unit (used without a geology product).
    defaults: Vec<Option<String>>,
}

impl<'a> Ground<'a> {
    /// The ground of `world`, on `geology` when given (its frame must be the guide's), else on `table`'s region
    /// shares.
    ///
    /// # Errors
    /// A geology product whose dims, spacing or origin are not the guide's.
    pub fn new(world: &'a World<'a>, geology: Option<&'a GeologyGrid>, table: &GeologyTable) -> Result<Self, DumpError> {
        if let Some(g) = geology {
            let w = &world.guide;
            let close = |a: f64, b: f64| (a - b).abs() <= 1e-6 * b.abs().max(1.0);
            if [g.w, g.h] != [w.w, w.h]
                || !close(g.spacing_m, w.spacing)
                || !close(g.origin_m[0], w.origin[0])
                || !close(g.origin_m[1], w.origin[1])
            {
                return Err(DumpError::Invalid(format!(
                    "rocks: the geology product ({}x{} @ {} m, origin {:?}) is not the guide ({}x{} @ {} m, origin {:?})",
                    g.w, g.h, g.spacing_m, g.origin_m, w.w, w.h, w.spacing, w.origin
                )));
            }
        }
        let defaults = unit_shares(world.regions, table)
            .into_iter()
            .map(|m| {
                let mut best: Option<(&String, f64)> = None;
                for (u, &v) in &m {
                    if best.is_none_or(|(_, b)| v > b) {
                        best = Some((u, v));
                    }
                }
                best.map(|(u, _)| u.clone())
            })
            .collect();
        Ok(Self { world, geology, defaults })
    }

    fn guide(&self) -> &Grid<'a> {
        &self.world.guide
    }
}

impl RockGround for Ground<'_> {
    fn height_m(&self, x: f64, y: f64) -> f64 {
        self.world.height(x, y)
    }

    fn slope_aspect_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let (c, r) = self.guide().colrow(x, y);
        let (gx, gy) = self.guide().gradient(c, r);
        let slope = gx.hypot(gy).atan().to_degrees();
        // Downslope is -(gx, gy); its compass azimuth, clockwise from north.
        let aspect = if gx == 0.0 && gy == 0.0 { 0.0 } else { (-gx).atan2(-gy).to_degrees().rem_euclid(360.0) };
        (slope, aspect)
    }

    fn cliff(&self, x: f64, y: f64) -> bool {
        self.geology.is_some_and(|g| g.cliff_at(x, y))
    }

    fn unit(&self, x: f64, y: f64) -> Option<&str> {
        match self.geology {
            Some(g) => g.unit_at(x, y),
            None => self.defaults.get(usize::from(self.world.region(x, y)?))?.as_deref(),
        }
    }
}

/// What the rock layer of a guide reads besides the world: the geology table (unit shares) and the guide's
/// `geology/` product, when it has one.
#[derive(Debug, Clone)]
pub struct RockSource {
    pub table: GeologyTable,
    pub grid: Option<GeologyGrid>,
}

impl RockSource {
    /// The rock source of a guide folder: its own `geology.toml` when it has one, else the shipped table; its
    /// `geology/` product ([`guide_geology`]).
    ///
    /// # Errors
    /// An unreadable or invalid table, or a present but unreadable product.
    pub fn read(dir: &Path) -> Result<Self, DumpError> {
        let own = dir.join(crate::geology::TABLE_FILE);
        let (text, what) = if own.exists() {
            (std::fs::read_to_string(&own)?, own.display().to_string())
        } else {
            (SHIPPED_GEOLOGY.to_owned(), format!("shipped {}", crate::geology::TABLE_FILE))
        };
        let table =
            crate::geology::AdapterGeology::from_toml(&text).map_err(|e| DumpError::Invalid(format!("{what}: {e}")))?.table;
        Ok(Self { table, grid: guide_geology(dir)? })
    }
}

/// The rock layer's inputs of a pass ([module docs](self) "Wiring").
pub struct Rocks<'a> {
    pub ground: &'a Ground<'a>,
    /// The file's `scatter.rock.*` values ([`values_of`]).
    pub values: RockValues,
    /// The field-stone prior per region legend index ([`rock_prior`]); `None`: 1 everywhere.
    pub prior: Option<&'a [f64]>,
    /// The geology table (hashed into the product inputs).
    pub table: &'a GeologyTable,
}

/// The `scatter.rock.*` values of `file`, checked.
///
/// # Errors
/// A value missing or out of range.
pub fn values_of(file: &SpeciesFile) -> Result<RockValues, DumpError> {
    RockValues::from_declared(&file.declared).map_err(|e| DumpError::Invalid(e.to_string()))
}

/// The field-stone prior a pass uses: [`rock_prior`] over the guide land per region, `None` (1 everywhere, the
/// authored density) when the source learned no rock density at all.
#[must_use]
pub fn pass_prior(learned: &Learned, area: &[Vec<u64>]) -> Option<Vec<f64>> {
    let land: Vec<u64> = area.iter().map(|a| a.iter().sum()).collect();
    let p = rock_prior(&learned.rock_density, &land);
    p.iter().any(|&v| v > 0.0).then_some(p)
}

/// The mesh of a placed rock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RockMesh {
    /// A learned pack model: its index in [`Learned::rocks`].
    Model(u32),
    /// A library class and its variant hash (the reader takes it modulo the class's variant count).
    Class { class: String, variant: u64 },
}

/// One placed rock, in the guide frame (x east, y north, z up, m).
#[derive(Clone, Debug, PartialEq)]
pub struct Rock {
    pub key: RockKey,
    pub mesh: RockMesh,
    /// Size mark (longest axis), m.
    pub size_m: f64,
    /// A model: the ground plus its learned origin offset; a class: the ground (the reader sinks it by `sink`).
    pub pos: [f64; 3],
    pub yaw: f32,
    /// Up axis (unit).
    pub up: [f32; 3],
    /// A model: its uniform scale; a class: the size mark, m (the reader divides by the class size).
    pub scale: f32,
    /// Share of a class mesh's short axis sunk below the ground (unused by a model).
    pub sink: f32,
    /// The weathering regime id at the parent (RG-9j).
    pub regime: Option<u8>,
}

/// The learned rock model index of each pack-model entry name (its base record id).
// Learned rock models: a few hundred.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn model_index(learned: &Learned) -> BTreeMap<&str, u32> {
    learned.rocks.iter().enumerate().map(|(i, s)| (learned.reps[s.rep].key.as_str(), i as u32)).collect()
}

/// The mesh and pose of the kept rock `c` at load (`scatter_rocks::pick` on `learned`'s file, `models` its
/// [`model_index`]): `None` when its unit has no fitting entry, or a pack model that is not one of `learned`'s.
// Poses are f32 in the placement.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn place_rock(learned: &Learned, models: &BTreeMap<&str, u32>, rocks: &Rocks<'_>, prm: &Params, c: &RockCandidate) -> Option<Rock> {
    let p = pick(&learned.file, prm, rocks.ground, c)?;
    let ground = rocks.ground.height_m(c.pos[0], c.pos[1]);
    let (mesh, z) = match p.entry {
        RockEntry::Model { model, .. } => {
            let i = *models.get(model.as_str())?;
            (RockMesh::Model(i), ground + f64::from(learned.rocks[i as usize].offset_m) * p.scale)
        }
        RockEntry::Class { class, .. } => (RockMesh::Class { class: class.clone(), variant: p.variant }, ground),
    };
    Some(Rock {
        key: c.key,
        mesh,
        size_m: c.size_m,
        pos: [c.pos[0], c.pos[1], z],
        yaw: p.yaw_rad as f32,
        up: p.up.map(|v| v as f32),
        scale: p.scale as f32,
        sink: p.sink as f32,
        regime: c.regime,
    })
}

/// A rock's id key bytes (context, tile, parent, offspring).
fn key_bytes(k: &RockKey) -> Vec<u8> {
    let mut key = Vec::with_capacity(17);
    key.push(k.context as u8);
    for v in [k.tile.0.to_le_bytes(), k.tile.1.to_le_bytes(), k.parent.to_le_bytes(), k.offspring.to_le_bytes()] {
        key.extend_from_slice(&v);
    }
    key
}

/// A rock's transform in the engine frame (Y up, -Z north).
fn transform(r: &Rock) -> WorldTransform {
    let up = Vec3::new(r.up[0], r.up[2], -r.up[1]);
    WorldTransform {
        pos: WorldPos::from_f64(DVec3::new(r.pos[0], r.pos[2], -r.pos[1])),
        rotation: Quat::from_rotation_arc(Vec3::Y, up) * Quat::from_rotation_y(r.yaw),
        scale: Vec3::splat(r.scale),
    }
}

/// The run's pack-model rocks as placements in the engine frame, each a copy of its model's source reference (base,
/// model, class, flags); the id is the rock key's.
#[must_use]
pub fn rock_placements(run: &FloraRun) -> Vec<Placement> {
    run.rocks
        .iter()
        .filter_map(|r| {
            let RockMesh::Model(i) = r.mesh else { return None };
            let rep = &run.learned.reps[run.learned.rocks[i as usize].rep];
            Some(Placement {
                id: PlacementId::from_key(ROCK_ID_NAMESPACE, &key_bytes(&r.key)),
                base: rep.base.clone(),
                model: rep.model.clone(),
                transform: transform(r),
                class: rep.class,
                flags: rep.flags,
            })
        })
        .collect()
}

/// One library rock for the streamer (spec `rockgen.md` "Placement in the game (RG-P)", once it lands).
#[derive(Clone, Debug, PartialEq)]
pub struct RockPlacement {
    pub id: PlacementId,
    /// The library class name.
    pub class: String,
    /// The variant hash (modulo the class's variant count).
    pub variant: u64,
    /// Engine frame; its uniform scale is the size mark, m (the reader divides by the class size); its position on
    /// the ground.
    pub transform: WorldTransform,
    /// Share of the mesh's short axis sunk below the ground.
    pub sink: f32,
    pub regime: Option<u8>,
}

/// The run's library rocks in the engine frame (no consumer until RG-P).
#[must_use]
pub fn library_rocks(run: &FloraRun) -> Vec<RockPlacement> {
    run.rocks
        .iter()
        .filter_map(|r| {
            let RockMesh::Class { class, variant } = &r.mesh else { return None };
            Some(RockPlacement {
                id: PlacementId::from_key(ROCK_ID_NAMESPACE, &key_bytes(&r.key)),
                class: class.clone(),
                variant: *variant,
                transform: transform(r),
                sink: r.sink,
                regime: r.regime,
            })
        })
        .collect()
}

/// (pack-model rocks, library rocks) of a run.
#[must_use]
pub fn rock_kinds(run: &FloraRun) -> [usize; 2] {
    let models = run.rocks.iter().filter(|r| matches!(r.mesh, RockMesh::Model(_))).count();
    [models, run.rocks.len() - models]
}
