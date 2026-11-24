//! No groundcover under placed statics (slice GC-M4; spec `superfargoth/docs/specs/groundcover.md` "Overrides"):
//! the exterior's grounded statics (rocks, buildings, docks, stairs, doors, containers, activators, standing
//! lights) become `sf_groundcover` exclusion regions, so neither the mesh scatter nor the blades plant inside them.
//!
//! Industry: UE removes landscape grass under actors through exclusion volumes / PCG "Difference" with actor bounds,
//! Horizon paints placement and exclusion maps; a footprint exclusion is the standard (Bevy has nothing native,
//! `bevy_grass` only reads density maps, which `sf_groundcover` writes from these overrides).
//!
//! **Footprint rule.** A static's model AABB, scaled and turned by its yaw (tilt ignored, as the terrain bake's
//! `PlacementFootprints::footprint`), projected to XZ and grown by [`MARGIN_M`] on every side: an exact oriented
//! box (`sf_groundcover::Shape::OrientedBox`), not the AABB of the turned box. It counts only when the object
//! reaches the ground: its world bottom at most [`REACH_M`] above the lowest terrain sample under the footprint
//! (centre and four corners) and its top above that ground (not buried). Bridges, hanging lanterns and signs above
//! the ground exclude nothing.
//!
//! **Not footprints:** actors and loose items (they move), editor markers and disabled references, models without
//! baked bounds, and Morrowind's flora folder (`meshes/f/`: trees, bushes, kelp), whose AABB is the canopy, not the
//! trunk.
//!
//! **Representation (measured, `per_sample_cost` in the tests):** one oriented-box region per static, no mask
//! memory. `sf_groundcover::Overrides` keeps them in an R-tree, so a density sample costs the regions near it, not
//! every static of the worldspace (the linear scan cost ~400 µs per sample at 40 000 statics in a debug build).
//!
//! Inputs, read once at `Startup` ([`build`]): the placements of every page of the worldspace's index
//! (`WorldIndexSource`), their models' baked bounds (`model.info`), and the baked level-0 terrain tiles under them.

use std::collections::{HashMap, HashSet};
use std::num::NonZero;
use std::sync::Arc;

use bevy::math::DVec3;
use sf_bake::{Manifest, Store};
use sf_coords::WorldPos;
use sf_groundcover::Region;
use sf_terrain_bevy::sf_terrain::bake::{BakedTerrain, PlacementFootprints};
use sf_terrain_bevy::sf_terrain::{ResidentTile, TerrainHeights, TileCoord};
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Source, Unit, def};
use sf_vfs::VfsPath;
use sf_world::{
    CellDirectory, IndexPage, IndexReader, ModelInfo, Placement, PlacementClass, PlacementFlags, WorldspaceId,
    placement_bounds,
};

/// `groundcover.statics.reach_m`: how far above the ground a static's bottom may sit and still clear groundcover.
pub const REACH_M: Def<f32> = def!(float "groundcover.statics.reach_m" = 0.5)
    .range(0.0, 5.0)
    .unit(Unit::Meters)
    .help("A static clears groundcover under it only when its bottom is at most this far above the ground (bridges and hanging lights don't)")
    .source(Source::Guess)
    .lifetime(Lifetime::Restart);

/// `groundcover.statics.margin_m`: how far past a static's footprint groundcover is cleared.
pub const MARGIN_M: Def<f32> = def!(float "groundcover.statics.margin_m" = 0.25)
    .range(0.0, 4.0)
    .unit(Unit::Meters)
    .help("Groundcover is cleared this far past a grounded static's footprint on every side")
    .source(Source::Guess)
    .lifetime(Lifetime::Restart);

pub const TUNABLES: &[&dyn AnyDef] = &[&REACH_M, &MARGIN_M];

// tunables: const (content convention: Morrowind's flora meshes live in `meshes/f/`)
const FLORA_PREFIX: &str = "meshes/f/";

/// The footprint tunables' values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FootprintSettings {
    pub reach_m: f32,
    pub margin_m: f32,
}

impl Default for FootprintSettings {
    fn default() -> Self {
        Self { reach_m: REACH_M.default_value(), margin_m: MARGIN_M.default_value() }
    }
}

/// One static's ground box: an XZ rectangle turned by `yaw` about +Y, and its world Y extent. World metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaticBox {
    pub center: [f64; 2],
    /// Half size along the box's own X and Z.
    pub half: [f64; 2],
    /// Radians about +Y, right-handed (`Quat::from_rotation_y`).
    pub yaw: f64,
    pub bottom_y: f64,
    pub top_y: f64,
}

impl StaticBox {
    /// The box's own X and Z axes in world XZ.
    fn axes(&self) -> ([f64; 2], [f64; 2]) {
        let (s, c) = self.yaw.sin_cos();
        // from_rotation_y(yaw): X → (cos, 0, -sin), Z → (sin, 0, cos).
        ([c, -s], [s, c])
    }

    /// Whether (x, z) lies inside the box grown by `grow` on every side.
    pub fn contains(&self, x: f64, z: f64, grow: f64) -> bool {
        let (ax, az) = self.axes();
        let (dx, dz) = (x - self.center[0], z - self.center[1]);
        (dx * ax[0] + dz * ax[1]).abs() <= self.half[0] + grow && (dx * az[0] + dz * az[1]).abs() <= self.half[1] + grow
    }

    /// The four corners in world XZ.
    fn corners(&self) -> [[f64; 2]; 4] {
        let (ax, az) = self.axes();
        let [hx, hz] = self.half;
        let at = |sx: f64, sz: f64| {
            [self.center[0] + sx * hx * ax[0] + sz * hz * az[0], self.center[1] + sx * hx * ax[1] + sz * hz * az[1]]
        };
        [at(-1.0, -1.0), at(1.0, -1.0), at(1.0, 1.0), at(-1.0, 1.0)]
    }

    /// The centre and the four corners, where [`exclusions`] samples the ground.
    fn ground_points(&self) -> [[f64; 2]; 5] {
        let [a, b, c, d] = self.corners();
        [self.center, a, b, c, d]
    }
}

/// Whether a placement can clear groundcover (module docs, "Not footprints").
pub fn is_footprint_class(p: &Placement) -> bool {
    let class = !matches!(p.class, PlacementClass::Actor | PlacementClass::Dynamic);
    let flags = !p.flags.intersects(PlacementFlags::EDITOR_ONLY | PlacementFlags::DISABLED);
    let flora = p.model.as_ref().is_some_and(|m| m.as_str().starts_with(FLORA_PREFIX));
    class && flags && !flora && p.model.is_some()
}

/// The ground box of `p` with model bounds `info`.
pub fn static_box(p: &Placement, info: &ModelInfo) -> StaticBox {
    let f = PlacementFootprints::footprint(p, (info.min, info.max));
    let (aabb, _) = placement_bounds(p, Some(info));
    let pivot = p.transform.pos.to_f64();
    let center = f.center.to_f64();
    StaticBox {
        center: [center.x, center.z],
        half: [f64::from(f.half_extents.x), f64::from(f.half_extents.y)],
        yaw: f64::from(f.yaw),
        bottom_y: pivot.y + f64::from(aabb.min.y),
        top_y: pivot.y + f64::from(aabb.max.y),
    }
}

/// What [`build`] and [`exclusions`] did, for the log line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FootprintReport {
    /// Placements read from the index (every class).
    pub placements: usize,
    /// Footprint-class placements with baked model bounds: the boxes tested.
    pub statics: usize,
    /// Boxes that became exclusions.
    pub excluded: usize,
    /// Bottom above the reach (bridges, hanging lights) or buried.
    pub floating: usize,
    /// No terrain under the footprint (outside the baked terrain).
    pub no_ground: usize,
    /// Level-0 terrain tiles decoded for the ground test.
    pub tiles: usize,
}

/// Exclusion regions of the boxes that reach the ground (module docs). `ground(x, z)` is the terrain height there,
/// `None` where there is no terrain.
pub fn exclusions(
    boxes: &[StaticBox],
    ground: &dyn Fn(f64, f64) -> Option<f32>,
    settings: FootprintSettings,
) -> (Vec<Region>, FootprintReport) {
    let mut report = FootprintReport { statics: boxes.len(), ..FootprintReport::default() };
    let mut out = Vec::new();
    let reach = f64::from(settings.reach_m.max(0.0));
    let margin = f64::from(settings.margin_m.max(0.0));
    for b in boxes {
        let lowest = b.ground_points().into_iter().try_fold(f64::INFINITY, |m, [x, z]| ground(x, z).map(|h| m.min(f64::from(h))));
        let Some(floor) = lowest else {
            report.no_ground += 1;
            continue;
        };
        if b.bottom_y > floor + reach || b.top_y < floor {
            report.floating += 1;
            continue;
        }
        report.excluded += 1;
        out.push(Region::oriented_box(b.center, [b.half[0] + margin, b.half[1] + margin], b.yaw));
    }
    (out, report)
}

/// Every placement in `ws`'s index (all pages), and the footprint-class ones among them.
pub fn read_placements(reader: &dyn IndexReader, ws: &WorldspaceId) -> Result<(usize, Vec<Placement>), String> {
    let Some(dir) = reader.directory(ws)? else { return Ok((0, Vec::new())) };
    let dir = CellDirectory::from_bytes(&dir).map_err(|e| e.to_string())?;
    let (mut all, mut out) = (0, Vec::new());
    for summary in &dir.pages {
        let Some(bytes) = reader.page(ws, summary.page)? else { continue };
        let page = IndexPage::from_bytes(&bytes).map_err(|e| e.to_string())?;
        for cell in page.cells {
            all += cell.placements.len();
            out.extend(cell.placements.into_iter().map(|p| p.placement).filter(is_footprint_class));
        }
    }
    Ok((all, out))
}

/// The baked level-0 tiles under every box's ground points, decoded on all cores, and how many there were.
fn terrain_under(terrain: &BakedTerrain, boxes: &[StaticBox]) -> (TerrainHeights, usize) {
    let to_pos = |[x, z]: [f64; 2]| WorldPos::from_f64(DVec3::new(x, 0.0, z));
    let coords: HashSet<TileCoord> =
        boxes.iter().flat_map(StaticBox::ground_points).map(|p| TileCoord::containing(to_pos(p), 0)).collect();
    let coords: Vec<TileCoord> = coords.into_iter().filter(|&c| terrain.has_tile(c)).collect();
    let threads = std::thread::available_parallelism().map_or(1, NonZero::get);
    let chunk = coords.len().div_ceil(threads).max(1);
    let decode = |cs: &[TileCoord]| -> Vec<Arc<ResidentTile>> {
        cs.iter()
            .filter_map(|&c| match terrain.tile(c)? {
                Ok(t) => Some(Arc::new(ResidentTile::new(t.height_tile()))),
                Err(e) => {
                    bevy::log::warn!("groundcover statics: terrain tile {c:?}: {e}");
                    None
                }
            })
            .collect()
    };
    let tiles: Vec<Arc<ResidentTile>> = std::thread::scope(|s| {
        let handles: Vec<_> = coords.chunks(chunk).map(|cs| s.spawn(move || decode(cs))).collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    });
    let count = tiles.len();
    let mut heights = TerrainHeights::new();
    for t in tiles {
        heights.insert(t);
    }
    (heights, count)
}

/// The exclusion regions of `ws`'s grounded statics (module docs), from the index, the baked model bounds and the
/// baked terrain.
pub fn build(
    reader: &dyn IndexReader,
    store: &Store,
    manifest: &Manifest,
    ws: &WorldspaceId,
    settings: FootprintSettings,
) -> Result<(Vec<Region>, FootprintReport), String> {
    let (placements, statics) = read_placements(reader, ws)?;
    let models: Vec<&VfsPath> = statics.iter().filter_map(|p| p.model.as_ref()).collect();
    let infos: HashMap<VfsPath, ModelInfo> = crate::worldgen::model_infos(store, manifest, models);
    let boxes: Vec<StaticBox> =
        statics.iter().filter_map(|p| infos.get(p.model.as_ref()?).map(|info| static_box(p, info))).collect();
    let terrain = BakedTerrain::new(store.clone(), manifest, ws.clone());
    let (heights, tiles) = terrain_under(&terrain, &boxes);
    let ground = |x: f64, z: f64| heights.height(WorldPos::from_f64(DVec3::new(x, 0.0, z)));
    let (regions, mut report) = exclusions(&boxes, &ground, settings);
    report.placements = placements;
    report.tiles = tiles;
    Ok((regions, report))
}

#[cfg(test)]
#[path = "groundcover_statics_tests.rs"]
mod tests;
