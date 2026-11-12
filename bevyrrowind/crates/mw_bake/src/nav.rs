//! The Morrowind navmesh adapter (`superfargoth/docs/specs/nav.md` "Bake", "Doors";
//! `docs/specs/ai-adapter.md` "Navigation"): [`MwNavSource`], the [`NavGeometrySource`] the
//! framework's `nav.*` steps (`sf_nav_bake`) bake from, and [`bake_nav`], the `bevyrrowind bake`
//! pass.
//!
//! - **Regions**: one per worldspace, origin the world origin (every Morrowind worldspace fits
//!   one 16 km region). The exterior covers the `LAND` grid's bounding box; an interior covers
//!   its placements' bounds plus [`INTERIOR_PAD`].
//! - **Geometry**: exactly what the mover collides with: each placement's `collision.cook`
//!   product (statics and items; [`is_solid`]) and the baked level-0 `terrain.tile` heights (the
//!   physics heightfield's source), read through the step's `RunCx` so they are recorded inputs.
//! - **Nav cells** ([`CELL_STEP`], [`NAV_CELL`] m squares): the host's world source puts every
//!   placement in each cell its world bounds touch, and names the terrain tiles under the cell.
//!   A tile reads the cells under its bounds, so a moved wall re-bakes only the tiles over the
//!   cells its bounds touch (nav.md "Re-bake granularity"). The cell product's bytes are its plan
//!   parameter: a changed list re-runs the cell and, through early cutoff, only its tiles.
//! - **Water**: a flat quad at the swim level, area [`WATER`] (swimmers only, by filter), as
//!   `OpenMW` does: `getSwimLevel` in `components/detournavigator/makenavmesh.cpp` (master,
//!   read 2026-11-11) puts it at `water - (fSwimHeightScale + 1) * half height`, so actors wade
//!   until deep water. The seabed stays, as in `OpenMW` (nav.md: drop it for non-swimmers only if
//!   the bake gate needs it). Exterior tiles with no terrain and no object get no water: open sea
//!   past the land's terrain tiles stores nothing.
//! - **Doors**: the leaf is left out of the geometry; each door is a [`DoorVolume`] (area
//!   [`DOOR`], `OpenMW`'s `AreaType_door`) over the leaf's footprint, widened to
//!   [`DOOR_HALF_DEPTH`] on both sides so the doorway and its threshold get their own polygons.
//!   [`NavDoorId`] is the door reference's `PlacementId` (a hash of content file + `FRMR`),
//!   folded to 64 bits: stable across runs and load orders that keep the reference.
//! - **Links**: door approach points, the nearest pathgrid node to each door within
//!   [`APPROACH_RADIUS`] (seamless research §4.11). Pathgrid edges as off-mesh links are NV9.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use glam::{Quat, Vec2, Vec3, Vec3Swizzles};
use mw_ai::pathgrid::{CellKey, PathGrids};
use mw_world::{MwWorldSource, SourceOptions, ids};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, PlanCx, ProductKey, RunCx, Step, StepStats};
use sf_controller::CollisionMesh;
use sf_controller::steps::COLLISION_COOK;
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, AreaVolume, BuildConfig, DoorApproach,
    MAX_CLASSES, NavDoorId, NavLinks, PolyFlags, TileInput,
};
use sf_nav_bake::{DoorVolume, NavBake, NavBakeConfig, NavGeometrySource, NavRegion, NavTileBounds, NavTileSource};
use sf_records::RecordDb;
use sf_terrain::TileCoord;
use sf_terrain::bake::{TILE_STEP, parse_tile_subject, tile_subject};
use sf_world::{Location, Placement, PlacementClass, PlacementFlags, PlacementId, WorldspaceId};
use tes3::esp::{Cell, GameSetting, GameSettingValue, Landscape};

/// Per-cell placement lists, subject `<x>|<z>|<worldspace>`.
pub const CELL_STEP: &str = "mw.nav.cell";
/// The worldspace's door approach points, subject `<worldspace>`.
pub const LINKS_STEP: &str = "mw.nav.links";
/// Edge of a nav cell, meters: a humanoid tile (19.2 m + border) reads 1-4 cells.
pub const NAV_CELL: f32 = 64.0;
/// Interior region margin around its placements, meters.
pub const INTERIOR_PAD: f32 = 2.0;
/// Door volumes reach this far on each side of the leaf, meters.
pub const DOOR_HALF_DEPTH: f32 = 0.75;
/// Floor slack under a door's lowest point, meters.
const DOOR_FLOOR_SLACK: f32 = 0.5;
/// Farthest pathgrid node that counts as a door's approach point, meters.
pub const APPROACH_RADIUS: f32 = 6.0;
/// `fSwimHeightScale` when the game settings don't have it (`Morrowind.esm`'s value).
const DEFAULT_SWIM_HEIGHT_SCALE: f32 = 0.9;
/// Level-0 terrain sample spacing, meters (`sf_terrain` `BASE_SPACING`).
const TERRAIN_SPACING: f32 = sf_terrain::BASE_SPACING;
/// Bump when this module's geometry rules change (part of every cell product).
const RULES_VERSION: u32 = 1;

pub const HUMANOID: AgentClassId = AgentClassId(0);
pub const LARGE: AgentClassId = AgentClassId(1);
pub const GROUND: AreaId = AreaId(1);
pub const DOOR: AreaId = AreaId(2);
pub const WATER: AreaId = AreaId(3);

/// The two agent classes (ai-adapter.md "Navigation"; NV0 calibration, nav.md "Tiles").
pub fn classes() -> Vec<BuildConfig> {
    vec![
        BuildConfig {
            class: AgentClass {
                id: HUMANOID,
                radius: 0.3,
                height: 1.8,
                max_climb: 0.4,
                max_slope_deg: 45.0,
                can_swim: true,
            },
            cell_size: 0.15,
            cell_height: 0.1,
            tile_cells: 128,
        },
        BuildConfig {
            class: AgentClass {
                id: LARGE,
                radius: 1.0,
                height: 3.0,
                max_climb: 0.6,
                max_slope_deg: 45.0,
                can_swim: false,
            },
            cell_size: 0.5,
            cell_height: 0.2,
            tile_cells: 64,
        },
    ]
}

/// The area table (ai-adapter.md "Navigation"): ground 1, door 2 (`OpenMW`'s door cost), water
/// 2 for swimmers (swimming is slower than walking).
pub fn areas() -> AreaTable {
    let def = |id, name: &str, cost, flags| AreaDef { id, name: name.into(), cost: [cost; MAX_CLASSES], flags };
    AreaTable::new(vec![
        def(GROUND, "ground", 1.0, PolyFlags::WALK),
        def(DOOR, "door", 2.0, PolyFlags::WALK | PolyFlags::DOOR),
        def(WATER, "water", 2.0, PolyFlags::SWIM),
    ])
    .expect("the Morrowind area table is valid")
}

/// Whether a placement is nav geometry: statics, items and doors (doors become volumes), not
/// actors, lights, markers, editor helpers or references that start disabled.
pub fn is_solid(p: &Placement) -> bool {
    matches!(p.class, PlacementClass::Static | PlacementClass::Dynamic | PlacementClass::Door)
        && !p.flags.intersects(PlacementFlags::EDITOR_ONLY | PlacementFlags::DISABLED)
}

/// A door's stable id: its reference's `PlacementId`, folded to 64 bits.
#[allow(clippy::cast_possible_truncation, reason = "folding a 128-bit hash")]
pub fn door_id(id: PlacementId) -> NavDoorId {
    NavDoorId((id.0 as u64) ^ ((id.0 >> 64) as u64))
}

/// A placed model as a nav cell lists it (region-local meters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavObject {
    pub id: u128,
    pub model: String,
    pub pos: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    /// Model-space bounds (`model.info`).
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl NavObject {
    fn of(p: &Placement, model: &str, (min, max): (Vec3, Vec3)) -> Self {
        let t = &p.transform;
        Self {
            id: p.id.0,
            model: model.to_owned(),
            pos: WorldPos::ORIGIN.delta_f32(t.pos).to_array(),
            rotation: t.rotation.to_array(),
            scale: t.scale.to_array(),
            min: min.to_array(),
            max: max.to_array(),
        }
    }

    fn apply(&self, v: Vec3) -> Vec3 {
        Vec3::from(self.pos) + Quat::from_array(self.rotation) * (v * Vec3::from(self.scale))
    }

    fn corners(&self) -> [Vec3; 8] {
        let (lo, hi) = (Vec3::from(self.min), Vec3::from(self.max));
        std::array::from_fn(|i| {
            let c = Vec3::new(
                if i & 1 == 0 { lo.x } else { hi.x },
                if i & 2 == 0 { lo.y } else { hi.y },
                if i & 4 == 0 { lo.z } else { hi.z },
            );
            self.apply(c)
        })
    }

    /// World XZ bounds of the scaled, rotated model bounds.
    fn xz_bounds(&self) -> (Vec2, Vec2) {
        self.corners().iter().fold((Vec2::MAX, Vec2::MIN), |(lo, hi), c| (lo.min(c.xz()), hi.max(c.xz())))
    }

    /// The door's volume: the leaf's footprint, its thin horizontal axis widened to
    /// [`DOOR_HALF_DEPTH`] on each side, from just under its lowest point to its top.
    pub fn door_volume(&self) -> DoorVolume {
        let (mut lo, mut hi) = (Vec3::from(self.min), Vec3::from(self.max));
        let scale = Vec3::from(self.scale).abs().max(Vec3::splat(1e-6));
        if (hi.x - lo.x) * scale.x <= (hi.z - lo.z) * scale.z {
            let half = DOOR_HALF_DEPTH / scale.x;
            let c = f32::midpoint(lo.x, hi.x);
            lo.x = lo.x.min(c - half);
            hi.x = hi.x.max(c + half);
        } else {
            let half = DOOR_HALF_DEPTH / scale.z;
            let c = f32::midpoint(lo.z, hi.z);
            lo.z = lo.z.min(c - half);
            hi.z = hi.z.max(c + half);
        }
        let footprint = [(lo.x, lo.z), (hi.x, lo.z), (hi.x, hi.z), (lo.x, hi.z)]
            .iter()
            .map(|&(x, z)| self.apply(Vec3::new(x, lo.y, z)).xz())
            .collect();
        let ys = self.corners().map(|c| c.y);
        let min_y = ys.iter().copied().fold(f32::INFINITY, f32::min);
        let max_y = ys.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        DoorVolume {
            door: door_id(PlacementId(self.id)),
            volume: AreaVolume { footprint, min_y: min_y - DOOR_FLOOR_SLACK, max_y, area: DOOR },
        }
    }
}

/// One nav cell: the `mw.nav.cell` product.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NavCell {
    /// Level-0 terrain tiles under the cell that the terrain bake made.
    pub terrain: Vec<(i32, i32)>,
    pub solids: Vec<NavObject>,
    pub doors: Vec<NavObject>,
}

/// What [`NavSpace::build`] needs about one worldspace.
pub struct SpaceInput<'a> {
    /// XZ extent (region-local); `None`: the placements' bounds plus [`INTERIOR_PAD`].
    pub extent: Option<(Vec2, Vec2)>,
    /// Water surface height, meters.
    pub water: Option<f32>,
    pub placements: Vec<&'a Placement>,
    /// Level-0 terrain tiles the terrain bake made (exterior only).
    pub terrain: &'a BTreeSet<(i32, i32)>,
    /// Pathgrid nodes (region-local).
    pub nodes: Vec<Vec3>,
}

/// One worldspace's nav data, built at plan time from the host's world source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NavSpace {
    pub min: Vec2,
    pub max: Vec2,
    pub water: Option<f32>,
    pub cells: BTreeMap<(i32, i32), NavCell>,
    pub links: NavLinks,
}

#[allow(clippy::cast_possible_truncation, reason = "cell indices of a ≤ 16 km region")]
fn cell_range(lo: f32, hi: f32, size: f32) -> std::ops::RangeInclusive<i32> {
    (lo / size).floor() as i32..=((hi / size).floor() as i32).max((lo / size).floor() as i32)
}

impl NavSpace {
    /// Cells, extent and door approaches. `bounds` gives a model's `model.info` bounds; models
    /// without bounds (not cooked) are left out. `None` when there is nothing to walk on.
    pub fn build(ws: &WorldspaceId, input: SpaceInput<'_>, bounds: &dyn Fn(&str) -> Option<(Vec3, Vec3)>) -> Option<Self> {
        let mut cells: BTreeMap<(i32, i32), NavCell> = BTreeMap::new();
        let (mut lo, mut hi) = (Vec2::MAX, Vec2::MIN);
        let mut doors = Vec::new();
        for p in input.placements.iter().filter(|p| is_solid(p)) {
            let Some(model) = p.model.as_ref() else { continue };
            let Some(b) = bounds(model.as_str()) else { continue };
            let o = NavObject::of(p, model.as_str(), b);
            let (blo, bhi) = o.xz_bounds();
            if !(blo.is_finite() && bhi.is_finite()) {
                continue;
            }
            lo = lo.min(blo);
            hi = hi.max(bhi);
            let door = p.class == PlacementClass::Door;
            for x in cell_range(blo.x, bhi.x, NAV_CELL) {
                for z in cell_range(blo.y, bhi.y, NAV_CELL) {
                    let c = cells.entry((x, z)).or_default();
                    if door { c.doors.push(o.clone()) } else { c.solids.push(o.clone()) }
                }
            }
            if door {
                doors.push(o);
            }
        }
        let (min, max) = match input.extent {
            Some(e) => e,
            None if lo.x <= hi.x => (lo - INTERIOR_PAD, hi + INTERIOR_PAD),
            None => return None,
        };
        #[allow(clippy::cast_precision_loss, reason = "512 quads")]
        let tile = sf_terrain::TILE_QUADS as f32 * TERRAIN_SPACING;
        for &(tx, tz) in input.terrain {
            #[allow(clippy::cast_precision_loss, reason = "tile indices are small")]
            let o = Vec2::new(tx as f32, tz as f32) * tile;
            for x in cell_range(o.x, o.x + tile - 1e-3, NAV_CELL) {
                for z in cell_range(o.y, o.y + tile - 1e-3, NAV_CELL) {
                    cells.entry((x, z)).or_default().terrain.push((tx, tz));
                }
            }
        }
        for c in cells.values_mut() {
            c.solids.sort_by_key(|o| o.id);
            c.doors.sort_by_key(|o| o.id);
            c.terrain.sort_unstable();
        }
        let links = NavLinks { links: Vec::new(), doors: approaches(ws, &doors, &input.nodes) };
        Some(Self { min, max, water: input.water, cells, links })
    }
}

/// Each door's nearest pathgrid node within [`APPROACH_RADIUS`] of the door volume's floor centre.
fn approaches(ws: &WorldspaceId, doors: &[NavObject], nodes: &[Vec3]) -> Vec<DoorApproach> {
    let mut grid: HashMap<(i32, i32), Vec<Vec3>> = HashMap::new();
    #[allow(clippy::cast_possible_truncation, reason = "bucket indices of a ≤ 16 km region")]
    let key = |v: Vec3| ((v.x / APPROACH_RADIUS).floor() as i32, (v.z / APPROACH_RADIUS).floor() as i32);
    for &n in nodes {
        grid.entry(key(n)).or_default().push(n);
    }
    let mut out = Vec::new();
    for d in doors {
        let v = d.door_volume();
        #[allow(clippy::cast_precision_loss, reason = "a few footprint corners")]
        let n = v.volume.footprint.len().max(1) as f32;
        let c = v.volume.footprint.iter().copied().sum::<Vec2>() / n;
        let at = Vec3::new(c.x, v.volume.min_y + DOOR_FLOOR_SLACK, c.y);
        let (kx, kz) = key(at);
        let best = (kx - 1..=kx + 1)
            .flat_map(|x| (kz - 1..=kz + 1).map(move |z| (x, z)))
            .filter_map(|k| grid.get(&k))
            .flatten()
            .map(|&p| (p.distance_squared(at), p))
            .filter(|(d2, _)| *d2 <= APPROACH_RADIUS * APPROACH_RADIUS)
            .min_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.to_array().partial_cmp(&b.1.to_array()).unwrap_or(std::cmp::Ordering::Equal)));
        if let Some((_, p)) = best {
            out.push(DoorApproach {
                door: v.door,
                at: Location { worldspace: ws.clone(), pos: WorldPos::from_f64(p.as_dvec3()) },
            });
        }
    }
    out.sort_by_key(|d| d.door);
    out
}

/// Every worldspace's nav data.
#[derive(Clone, Debug, Default)]
pub struct NavWorld {
    pub spaces: BTreeMap<WorldspaceId, NavSpace>,
    pub swim_height_scale: f32,
}

impl NavWorld {
    pub fn new(spaces: BTreeMap<WorldspaceId, NavSpace>, swim_height_scale: f32) -> Self {
        Self { spaces, swim_height_scale }
    }

    pub fn worldspaces(&self) -> Vec<WorldspaceId> {
        self.spaces.keys().cloned().collect()
    }

    fn cell_bytes(&self, ws: &WorldspaceId, cell: (i32, i32)) -> Vec<u8> {
        let empty = NavCell::default();
        let c = self.spaces.get(ws).and_then(|s| s.cells.get(&cell)).unwrap_or(&empty);
        postcard::to_stdvec(c).expect("nav cells serialize")
    }
}

pub fn cell_subject(ws: &WorldspaceId, (x, z): (i32, i32)) -> String {
    format!("{x}|{z}|{}", ws.as_str())
}

fn parse_cell_subject(s: &str) -> Option<(WorldspaceId, (i32, i32))> {
    let mut it = s.splitn(3, '|');
    let x = it.next()?.parse().ok()?;
    let z = it.next()?.parse().ok()?;
    Some((WorldspaceId::new(it.next()?), (x, z)))
}

fn step_err(step: &str, subject: &str, e: impl std::fmt::Display) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

/// `mw.nav.cell`: the cell's list, from the plan (its bytes are the plan parameter).
struct CellStep(Arc<NavWorld>);

impl Step for CellStep {
    fn id(&self) -> &'static str {
        CELL_STEP
    }
    fn version(&self) -> u32 {
        RULES_VERSION
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(CELL_STEP, subject, "bad subject"))?;
        cx.param("cell", self.0.cell_bytes(&ws, cell));
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, cell) = parse_cell_subject(subject).ok_or_else(|| step_err(CELL_STEP, subject, "bad subject"))?;
        Ok(self.0.cell_bytes(&ws, cell))
    }
}

/// `mw.nav.links`: the worldspace's door approach points, from the plan.
struct LinksStep(Arc<NavWorld>);

impl LinksStep {
    fn bytes(&self, subject: &str) -> Vec<u8> {
        let links = self.0.spaces.get(&WorldspaceId::new(subject)).map(|s| s.links.clone()).unwrap_or_default();
        postcard::to_stdvec(&links).expect("nav links serialize")
    }
}

impl Step for LinksStep {
    fn id(&self) -> &'static str {
        LINKS_STEP
    }
    fn version(&self) -> u32 {
        RULES_VERSION
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.param("links", self.bytes(subject));
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        Ok(self.bytes(subject))
    }
}

/// Byte budgets of the [`GeometryCache`] kinds. A humanoid tile front crosses ~20 height
/// tiles (256 m each) per scan line; the budgets hold several fronts (16 bake threads).
const HEIGHT_BUDGET: usize = 128 << 20;
const MESH_BUDGET: usize = 128 << 20;
const CELL_BUDGET: usize = 32 << 20;

/// Decoded products keyed by their digest, least recently used evicted past a byte budget.
struct Decoded<T> {
    budget: usize,
    inner: Mutex<DecodedInner<T>>,
}

struct DecodedInner<T> {
    map: HashMap<Digest, (Arc<T>, usize, u64)>,
    bytes: usize,
    clock: u64,
}

impl<T> Decoded<T> {
    fn new(budget: usize) -> Self {
        Self { budget, inner: Mutex::new(DecodedInner { map: HashMap::new(), bytes: 0, clock: 0 }) }
    }

    fn lock(&self) -> MutexGuard<'_, DecodedInner<T>> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn get(&self, d: &Digest) -> Option<Arc<T>> {
        let mut g = self.lock();
        g.clock += 1;
        let now = g.clock;
        g.map.get_mut(d).map(|(v, _, used)| {
            *used = now;
            v.clone()
        })
    }

    fn insert(&self, d: Digest, v: Arc<T>, size: usize) {
        let mut g = self.lock();
        g.clock += 1;
        let now = g.clock;
        if let Some((_, old, _)) = g.map.insert(d, (v, size, now)) {
            g.bytes -= old;
        }
        g.bytes += size;
        while g.bytes > self.budget {
            let lru = g.map.iter().filter(|(k, _)| **k != d).min_by_key(|(_, e)| e.2).map(|(k, _)| *k);
            let Some((_, old, _)) = lru.and_then(|k| g.map.remove(&k)) else { break };
            g.bytes -= old;
        }
    }

    #[cfg(test)]
    fn contains(&self, d: &Digest) -> bool {
        self.lock().map.contains_key(d)
    }

    fn clear(&self) {
        let mut g = self.lock();
        g.map = HashMap::new();
        g.bytes = 0;
    }
}

/// What the [`MwNavSource`]'s shared decode did, cumulative since the source was made.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GeometryStats {
    /// Product reads by tiles (each one a recorded dependency).
    pub reads: u64,
    /// Reads that decoded the product; the rest shared a decoded copy.
    pub decoded: u64,
    /// Time resolving the products read (`product_digest`; a nav cell's first read bakes it).
    pub resolve: Duration,
    /// Time reading product bytes from the store (decoded reads only).
    pub read: Duration,
    /// Time decoding (decoded reads only).
    pub decode: Duration,
}

impl std::ops::Sub for GeometryStats {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            reads: self.reads - o.reads,
            decoded: self.decoded - o.decoded,
            resolve: self.resolve.saturating_sub(o.resolve),
            read: self.read.saturating_sub(o.read),
            decode: self.decode.saturating_sub(o.decode),
        }
    }
}

#[derive(Default)]
struct StatCounters {
    reads: AtomicU64,
    decoded: AtomicU64,
    resolve: AtomicU64,
    read: AtomicU64,
    decode: AtomicU64,
}

fn add_nanos(a: &AtomicU64, d: Duration) {
    a.fetch_add(u64::try_from(d.as_nanos()).unwrap_or(u64::MAX), Ordering::Relaxed);
}

/// Decoded tile geometry shared by every tile of a bake (NV4d): nav cells, collision meshes and
/// level-0 height tiles, keyed by product digest, so a shared copy is exactly the product's
/// bytes decoded. Every tile still reads each product through its `RunCx` (`product_digest`,
/// a recorded dependency), so incremental re-bakes stay exact; only the read and decode are
/// skipped. Bounded per kind ([`HEIGHT_BUDGET`], [`MESH_BUDGET`], [`CELL_BUDGET`]).
struct GeometryCache {
    cells: Decoded<NavCell>,
    meshes: Decoded<Option<CollisionMesh>>,
    heights: Decoded<sf_terrain::HeightTile>,
    stats: StatCounters,
}

impl Default for GeometryCache {
    fn default() -> Self {
        Self {
            cells: Decoded::new(CELL_BUDGET),
            meshes: Decoded::new(MESH_BUDGET),
            heights: Decoded::new(HEIGHT_BUDGET),
            stats: StatCounters::default(),
        }
    }
}

impl GeometryCache {
    /// Product `step`/`subject` decoded, shared when its digest was decoded before.
    fn get<T>(
        &self,
        cx: &mut RunCx<'_>,
        kind: &Decoded<T>,
        (step, subject): (&str, &str),
        decode: impl FnOnce(&[u8]) -> Result<(T, usize), String>,
    ) -> Result<Arc<T>, BakeError> {
        let s = &self.stats;
        s.reads.fetch_add(1, Ordering::Relaxed);
        let t = Instant::now();
        let digest = cx.product_digest(step, subject)?;
        add_nanos(&s.resolve, t.elapsed());
        if let Some(v) = kind.get(&digest) {
            return Ok(v);
        }
        let t = Instant::now();
        let bytes = cx.product(step, subject)?;
        add_nanos(&s.read, t.elapsed());
        let t = Instant::now();
        let (v, size) = decode(&bytes).map_err(|e| step_err(step, subject, e))?;
        add_nanos(&s.decode, t.elapsed());
        s.decoded.fetch_add(1, Ordering::Relaxed);
        let v = Arc::new(v);
        kind.insert(digest, v.clone(), size);
        Ok(v)
    }

    fn cell(&self, cx: &mut RunCx<'_>, subject: &str) -> Result<Arc<NavCell>, BakeError> {
        self.get(cx, &self.cells, (CELL_STEP, subject), |b| {
            postcard::from_bytes(b).map(|c| (c, 2 * b.len())).map_err(|e| e.to_string())
        })
    }

    fn mesh(&self, cx: &mut RunCx<'_>, model: &str) -> Result<Arc<Option<CollisionMesh>>, BakeError> {
        self.get(cx, &self.meshes, (COLLISION_COOK, model), |b| {
            let mesh = CollisionMesh::from_bytes(b).map_err(|e| e.to_string())?;
            let size = 12 * (mesh.positions.len() + mesh.triangles.len()) + 2 * mesh.surfaces.len();
            Ok(((!mesh.is_empty()).then_some(mesh), size))
        })
    }

    fn height(&self, cx: &mut RunCx<'_>, subject: &str) -> Result<Arc<sf_terrain::HeightTile>, BakeError> {
        self.get(cx, &self.heights, (TILE_STEP, subject), |b| {
            let tile = sf_terrain::TileProduct::from_bytes(b).map_err(|e| e.to_string())?.height_tile();
            let size = 4 * tile.heights.len();
            Ok((tile, size))
        })
    }

    fn stats(&self) -> GeometryStats {
        let s = &self.stats;
        let d = |a: &AtomicU64| Duration::from_nanos(a.load(Ordering::Relaxed));
        GeometryStats {
            reads: s.reads.load(Ordering::Relaxed),
            decoded: s.decoded.load(Ordering::Relaxed),
            resolve: d(&s.resolve),
            read: d(&s.read),
            decode: d(&s.decode),
        }
    }

    fn clear(&self) {
        self.cells.clear();
        self.meshes.clear();
        self.heights.clear();
    }
}

/// The Morrowind [`NavGeometrySource`] (module docs).
pub struct MwNavSource {
    world: Arc<NavWorld>,
    cache: GeometryCache,
}

impl MwNavSource {
    pub fn new(world: Arc<NavWorld>) -> Self {
        Self { world, cache: GeometryCache::default() }
    }

    /// The shared decode's counters so far.
    pub fn geometry_stats(&self) -> GeometryStats {
        self.cache.stats()
    }

    /// Drops the shared decoded geometry. The source stays registered with the baker after a
    /// bake; later reads decode again.
    pub fn release_geometry(&self) {
        self.cache.clear();
    }

    /// Registers the adapter's own steps (`mw.nav.cell`, `mw.nav.links`).
    pub fn register(&self, baker: &mut Baker) {
        baker.register(CellStep(self.world.clone())).register(LinksStep(self.world.clone()));
    }
}

fn overlaps(b: &NavTileBounds, lo: Vec2, hi: Vec2) -> bool {
    b.overlaps(lo, hi)
}

/// Appends `mesh` placed by `o`, keeping only triangles whose XZ bounds meet `b`.
fn push_mesh(input: &mut TileInput, mesh: &CollisionMesh, o: &NavObject, b: &NavTileBounds) {
    let world: Vec<Vec3> = mesh.positions.iter().map(|v| o.apply(Vec3::from(*v))).collect();
    let mut remap = vec![u32::MAX; world.len()];
    for t in &mesh.triangles {
        let [a, bb, c] = t.map(|i| world.get(i as usize).copied());
        let (Some(a), Some(bb), Some(c)) = (a, bb, c) else { continue };
        let lo = a.xz().min(bb.xz()).min(c.xz());
        let hi = a.xz().max(bb.xz()).max(c.xz());
        if !overlaps(b, lo, hi) || !a.is_finite() || !bb.is_finite() || !c.is_finite() {
            continue;
        }
        let tri = t.map(|i| {
            let slot = &mut remap[i as usize];
            if *slot == u32::MAX {
                *slot = u32::try_from(input.verts.len()).expect("tile vertices fit u32");
                input.verts.push(world[i as usize]);
            }
            *slot
        });
        input.tris.push(tri);
        input.tri_areas.push(GROUND);
    }
}

/// Appends the terrain tile's triangles over `b` (canonical main diagonal, +Y up).
fn push_terrain(input: &mut TileInput, tile: &sf_terrain::HeightTile, b: &NavTileBounds) {
    let origin = WorldPos::ORIGIN.delta_f32(tile.coord.origin());
    let quads = sf_terrain::TILE_QUADS;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "clamped sample indices")]
    let range = |lo: f32, hi: f32, o: f32| {
        let i0 = ((lo - o) / TERRAIN_SPACING).floor().clamp(0.0, quads as f32) as usize;
        let i1 = ((hi - o) / TERRAIN_SPACING).ceil().clamp(0.0, quads as f32) as usize;
        (i0, i1)
    };
    let (i0, i1) = range(b.min.x, b.max.x, origin.x);
    let (j0, j1) = range(b.min.y, b.max.y, origin.z);
    if i0 >= i1 || j0 >= j1 {
        return;
    }
    let base = u32::try_from(input.verts.len()).expect("tile vertices fit u32");
    let row = u32::try_from(i1 - i0 + 1).expect("small");
    #[allow(clippy::cast_precision_loss, reason = "sample indices ≤ 512")]
    for j in j0..=j1 {
        for i in i0..=i1 {
            let h = tile.get(i, j);
            input.verts.push(Vec3::new(origin.x + i as f32 * TERRAIN_SPACING, h, origin.z + j as f32 * TERRAIN_SPACING));
        }
    }
    for j in 0..u32::try_from(j1 - j0).expect("small") {
        for i in 0..row - 1 {
            let a = base + j * row + i;
            let (b1, d, c) = (a + 1, a + row, a + row + 1);
            let finite = [a, b1, d, c].iter().all(|&k| input.verts[k as usize].y.is_finite());
            if !finite {
                continue;
            }
            input.tris.push([a, c, b1]);
            input.tris.push([a, d, c]);
            input.tri_areas.extend([GROUND, GROUND]);
        }
    }
}

fn push_water(input: &mut TileInput, b: &NavTileBounds, y: f32) {
    let base = u32::try_from(input.verts.len()).expect("tile vertices fit u32");
    input.verts.extend([
        Vec3::new(b.min.x, y, b.min.y),
        Vec3::new(b.max.x, y, b.min.y),
        Vec3::new(b.max.x, y, b.max.y),
        Vec3::new(b.min.x, y, b.max.y),
    ]);
    input.tris.extend([[base, base + 2, base + 1], [base, base + 3, base + 2]]);
    input.tri_areas.extend([WATER, WATER]);
}

impl NavGeometrySource for MwNavSource {
    fn regions(&self, ws: &WorldspaceId) -> Vec<NavRegion> {
        self.world
            .spaces
            .get(ws)
            .map(|s| vec![NavRegion { region: 0, origin: WorldPos::ORIGIN, min: s.min, max: s.max }])
            .unwrap_or_default()
    }

    fn tile_input(
        &self,
        cx: &mut RunCx<'_>,
        ws: &WorldspaceId,
        class: &AgentClass,
        bounds: &NavTileBounds,
    ) -> Result<NavTileSource, BakeError> {
        let mut src = NavTileSource::default();
        let Some(space) = self.world.spaces.get(ws) else { return Ok(src) };
        // Every cell under the bounds inside the region, empty or not, so content arriving in
        // an empty cell later re-bakes this tile.
        let (lo, hi) = (bounds.min.max(space.min), bounds.max.min(space.max));
        let mut cells = Vec::new();
        if lo.x <= hi.x && lo.y <= hi.y {
            for x in cell_range(lo.x, hi.x, NAV_CELL) {
                for z in cell_range(lo.y, hi.y, NAV_CELL) {
                    cells.push(self.cache.cell(cx, &cell_subject(ws, (x, z)))?);
                }
            }
        }
        let mut solids: BTreeMap<u128, &NavObject> = BTreeMap::new();
        let mut doors: BTreeMap<u128, &NavObject> = BTreeMap::new();
        let mut terrain: BTreeSet<(i32, i32)> = BTreeSet::new();
        for cell in &cells {
            terrain.extend(cell.terrain.iter().copied());
            solids.extend(cell.solids.iter().map(|o| (o.id, o)));
            doors.extend(cell.doors.iter().map(|o| (o.id, o)));
        }
        // One read per model per tile (each a recorded dependency of this tile).
        let mut meshes: HashMap<&str, Arc<Option<CollisionMesh>>> = HashMap::new();
        for o in solids.values() {
            let (blo, bhi) = o.xz_bounds();
            if !overlaps(bounds, blo, bhi) {
                continue;
            }
            let mesh = if let Some(m) = meshes.get(o.model.as_str()) {
                m.clone()
            } else {
                let m = self.cache.mesh(cx, &o.model)?;
                meshes.insert(&o.model, m.clone());
                m
            };
            if let Some(mesh) = &*mesh {
                push_mesh(&mut src.input, mesh, o, bounds);
            }
        }
        for &(x, z) in &terrain {
            let tile = self.cache.height(cx, &tile_subject(ws, TileCoord::new(0, x, z)))?;
            push_terrain(&mut src.input, &tile, bounds);
        }
        // Water only over something (terrain or an object): open sea stores nothing.
        if let Some(level) = space.water
            && !src.input.tris.is_empty()
        {
            let swim = level - 0.5 * class.height * (self.world.swim_height_scale + 1.0);
            push_water(&mut src.input, bounds, swim);
        }
        src.doors = doors
            .values()
            .map(|o| o.door_volume())
            .filter(|d| {
                let f = &d.volume.footprint;
                let lo = f.iter().copied().fold(Vec2::MAX, Vec2::min);
                let hi = f.iter().copied().fold(Vec2::MIN, Vec2::max);
                overlaps(bounds, lo, hi)
            })
            .collect();
        Ok(src)
    }

    fn links(&self, cx: &mut RunCx<'_>, ws: &WorldspaceId) -> Result<NavLinks, BakeError> {
        postcard::from_bytes(&cx.product(LINKS_STEP, ws.as_str())?).map_err(|e| step_err(LINKS_STEP, ws.as_str(), e))
    }
}

/// One class's share of the bake.
#[derive(Clone, Debug, Default)]
pub struct ClassReport {
    pub name: &'static str,
    pub tiles: usize,
    pub stats: StepStats,
    pub wall: Duration,
}

/// What [`bake_nav`] did.
#[derive(Debug, Default)]
pub struct NavReport {
    pub worldspaces: usize,
    pub doors: usize,
    pub approaches: usize,
    pub prepare: Duration,
    pub classes: Vec<ClassReport>,
    pub elapsed: Duration,
    pub failures: Vec<(ProductKey, Arc<BakeError>)>,
    /// What the shared geometry decode did.
    pub geometry: GeometryStats,
}

impl std::fmt::Display for NavReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::cast_precision_loss, reason = "display")]
        let parts: Vec<String> = self
            .classes
            .iter()
            .map(|c| {
                format!(
                    "{} {} tiles ({} built, {} hit) {:.1} s wall {:.1} CPU-s {:.1} MiB",
                    c.name,
                    c.tiles,
                    c.stats.built,
                    c.stats.hits,
                    c.wall.as_secs_f64(),
                    c.stats.run_time.as_secs_f64(),
                    c.stats.bytes_written as f64 / (1024.0 * 1024.0)
                )
            })
            .collect();
        writeln!(
            f,
            "nav: {} worldspaces, {} doors ({} with an approach point); {}; geometry {} decoded of {} reads; prepare {:.2?}, total {:.2?}, {} failed",
            self.worldspaces,
            self.doors,
            self.approaches,
            parts.join("; "),
            self.geometry.decoded,
            self.geometry.reads,
            self.prepare,
            self.elapsed,
            self.failures.len()
        )
    }
}

fn swim_height_scale(db: &RecordDb) -> f32 {
    match db.get::<GameSetting>(&sf_records::RecordId::new("fSwimHeightScale")).map(|g| &g.value) {
        Some(GameSettingValue::Float(v)) => *v,
        _ => DEFAULT_SWIM_HEIGHT_SCALE,
    }
}

/// The plan-time nav world from the records and this session's model and terrain products.
pub fn nav_world(baker: &Baker, db: &RecordDb, models: &[String]) -> NavWorld {
    let bounds = crate::world::model_bounds(baker, models);
    let lookup = |m: &str| bounds.get(m).copied();
    // Same options as `world::bake_exterior`, so positions are grounded identically.
    let source = MwWorldSource::build(db, &SourceOptions { model_bounds: Some(&lookup), ..SourceOptions::default() });
    let records = mw_world::WorldRecords::build(db);
    let ext = ids::exterior();
    let manifest = baker.manifest();
    let terrain: BTreeSet<(i32, i32)> = manifest
        .products
        .keys()
        .filter(|k| k.step == TILE_STEP)
        .filter_map(|k| parse_tile_subject(&k.subject))
        .filter(|(ws, c)| *ws == ext && c.level == 0)
        .map(|(_, c)| (c.x, c.z))
        .collect();
    let no_terrain = BTreeSet::new();
    // Exterior extent: the LAND grid's bounding box (world z = -Morrowind y).
    #[allow(clippy::cast_possible_truncation, reason = "a cell edge in meters")]
    let k = (mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64) as f32;
    let (glo, ghi) = db
        .iter::<Landscape>()
        .map(|(_, l)| l.grid)
        .fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)), |(lo, hi), g| ((lo.0.min(g.0), lo.1.min(g.1)), (hi.0.max(g.0), hi.1.max(g.1))));
    #[allow(clippy::cast_precision_loss, reason = "grid indices are small")]
    let ext_extent = (glo.0 <= ghi.0).then(|| {
        (Vec2::new(glo.0 as f32 * k, -(ghi.1 + 1) as f32 * k), Vec2::new((ghi.0 + 1) as f32 * k, -glo.1 as f32 * k))
    });
    // Pathgrid nodes per worldspace (interior keys are lower-cased cell names).
    let interior_ws: HashMap<String, WorldspaceId> = db
        .iter::<Cell>()
        .filter(|(_, c)| c.is_interior())
        .map(|(_, c)| (c.name.to_ascii_lowercase(), ids::interior(&c.name)))
        .collect();
    let mut nodes: HashMap<WorldspaceId, Vec<Vec3>> = HashMap::new();
    let grids = PathGrids::build(db);
    let mut keys: Vec<_> = grids.grids.keys().collect();
    keys.sort_by_key(|k| format!("{k:?}"));
    for key in keys {
        let ws = match key {
            CellKey::Exterior(..) => ext.clone(),
            CellKey::Interior(name) => match interior_ws.get(name) {
                Some(ws) => ws.clone(),
                None => continue,
            },
        };
        nodes.entry(ws).or_default().extend(grids.grids[key].positions().map(|p| WorldPos::ORIGIN.delta_f32(p)));
    }
    let mut spaces = BTreeMap::new();
    for (ws, record) in &records.worldspaces {
        let is_ext = *ws == ext;
        let input = SpaceInput {
            extent: if is_ext { ext_extent } else { None },
            water: record.sea_level,
            placements: source.all_placements(ws).collect(),
            terrain: if is_ext { &terrain } else { &no_terrain },
            nodes: nodes.remove(ws).unwrap_or_default(),
        };
        if is_ext && ext_extent.is_none() {
            continue;
        }
        if let Some(space) = NavSpace::build(ws, input, &|m| lookup(m).map(|(a, b)| (Vec3::from(a), Vec3::from(b)))) {
            spaces.insert(ws.clone(), space);
        }
    }
    NavWorld::new(spaces, swim_height_scale(db))
}

/// The bake's navmesh pass: after the collision cook and the terrain (their products are this
/// session's, so tile reads hit). Bakes links, then each class, timed per class.
pub fn bake_nav(baker: &mut Baker, db: &RecordDb, models: &[String]) -> NavReport {
    let started = Instant::now();
    let mut report = NavReport::default();
    let world = Arc::new(nav_world(baker, db, models));
    let worldspaces = world.worldspaces();
    report.worldspaces = worldspaces.len();
    report.doors = world.spaces.values().flat_map(|s| s.cells.values()).flat_map(|c| &c.doors).map(|o| o.id).collect::<BTreeSet<_>>().len();
    report.approaches = world.spaces.values().map(|s| s.links.doors.len()).sum();
    let source = Arc::new(MwNavSource::new(world));
    source.register(baker);
    let backend = match sf_nav_recast::RecastBackend::new(sf_nav_recast::RecastConfig::new(classes())) {
        Ok(b) => b,
        Err(e) => {
            report.failures.push((ProductKey::new(sf_nav_bake::TILES, ""), Arc::new(BakeError::Other(e.to_string()))));
            return report;
        }
    };
    let cfg = NavBakeConfig { classes: classes(), areas: areas(), backend_tag: sf_nav_recast::BAKE_TAG.into() };
    let bake = match NavBake::new(source.clone(), Arc::new(backend), cfg) {
        Ok(b) => b,
        Err(e) => {
            report.failures.push((ProductKey::new(sf_nav_bake::TILES, ""), Arc::new(BakeError::Other(e.to_string()))));
            return report;
        }
    };
    bake.register(baker);
    report.prepare = started.elapsed();
    let links: Vec<ProductKey> = worldspaces.iter().map(|ws| ProductKey::new(sf_nav_bake::LINKS, ws.as_str())).collect();
    report.failures.extend(baker.bake(&links).failures().map(|(k, e)| (k.clone(), e.clone())));
    for (class, name) in [(HUMANOID, "humanoid"), (LARGE, "large")] {
        let keys = bake.class_products(&worldspaces, class);
        let before = baker.step_stats().get(sf_nav_bake::TILES).copied().unwrap_or_default();
        let t = Instant::now();
        let r = baker.bake(&keys);
        let wall = t.elapsed();
        report.failures.extend(r.failures().map(|(k, e)| (k.clone(), e.clone())));
        let after = baker.step_stats().get(sf_nav_bake::TILES).copied().unwrap_or_default();
        let stats = StepStats {
            hits: after.hits - before.hits,
            built: after.built - before.built,
            overrides: after.overrides - before.overrides,
            failed: after.failed - before.failed,
            duplicates: after.duplicates - before.duplicates,
            run_time: after.run_time.saturating_sub(before.run_time),
            bytes_written: after.bytes_written - before.bytes_written,
        };
        let tiles = keys.iter().filter(|k| k.step == sf_nav_bake::TILES).count();
        report.classes.push(ClassReport { name, tiles, stats, wall });
    }
    report.geometry = source.geometry_stats();
    source.release_geometry();
    report.elapsed = started.elapsed();
    report
}
