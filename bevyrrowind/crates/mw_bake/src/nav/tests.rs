//! `NV4b` adapter tests on a synthetic room: a 20 m floor, a wall and one door, a pathgrid node
//! in front of it. Exact: door ids are stable, the leaf is not nav geometry, the door has its
//! volume, polygons and approach point, and a second session is a full cache hit.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use glam::{Quat, Vec2, Vec3};
use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, RunCx, Step, Store};
use sf_controller::{CollisionMesh, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};
use sf_nav::{BakedTile, NavLinks};
use sf_vfs::VfsPath;
use sf_world::{Placement, PlacementId, RecordRef};

use super::*;

struct NoInputs;

impl Inputs for NoInputs {
    fn file_fingerprint(&self, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Err(BakeError::Other(format!("no file {path}")))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

const FLOOR: &str = "meshes/floor.nif";
const WALL: &str = "meshes/wall.nif";
const DOOR_MODEL: &str = "meshes/door.nif";
/// The door leaf's top: no nav geometry may reach it.
const LEAF_TOP: f32 = 2.2;

/// A box mesh `[lo, hi]` (12 triangles).
fn cuboid(lo: Vec3, hi: Vec3) -> CollisionMesh {
    let c = |i: u32| {
        [
            if i & 1 == 0 { lo.x } else { hi.x },
            if i & 2 == 0 { lo.y } else { hi.y },
            if i & 4 == 0 { lo.z } else { hi.z },
        ]
    };
    let positions = (0..8).map(c).collect();
    let triangles = vec![
        [0, 1, 3], [0, 3, 2], [4, 6, 7], [4, 7, 5], [0, 4, 5], [0, 5, 1],
        [2, 3, 7], [2, 7, 6], [0, 2, 6], [0, 6, 4], [1, 5, 7], [1, 7, 3],
    ];
    CollisionMesh { positions, triangles, surfaces: vec![SurfaceId::default(); 12] }
}

/// Model-space shapes: a 20 x 20 m slab, a 0.3 m thick wall with a 1.2 m doorway at x 9.4..10.6
/// (two boxes), and the door leaf filling the doorway.
fn meshes() -> BTreeMap<&'static str, CollisionMesh> {
    let floor = cuboid(Vec3::new(0.0, -0.2, 0.0), Vec3::new(20.0, 0.0, 20.0));
    let mut wall = cuboid(Vec3::new(0.0, 0.0, 9.85), Vec3::new(9.4, 3.0, 10.15));
    let right = cuboid(Vec3::new(10.6, 0.0, 9.85), Vec3::new(20.0, 3.0, 10.15));
    let base = u32::try_from(wall.positions.len()).unwrap();
    wall.positions.extend(right.positions);
    wall.triangles.extend(right.triangles.iter().map(|t| t.map(|i| i + base)));
    wall.surfaces.extend(right.surfaces);
    let door = cuboid(Vec3::new(-0.6, 0.0, -0.05), Vec3::new(0.6, LEAF_TOP, 0.05));
    BTreeMap::from([(FLOOR, floor), (WALL, wall), (DOOR_MODEL, door)])
}

fn bounds_of(m: &str) -> Option<(Vec3, Vec3)> {
    let mesh = meshes().remove(m)?;
    let lo = mesh.positions.iter().fold(Vec3::MAX, |a, p| a.min(Vec3::from(*p)));
    let hi = mesh.positions.iter().fold(Vec3::MIN, |a, p| a.max(Vec3::from(*p)));
    Some((lo, hi))
}

/// A fake `collision.cook`: the shapes above.
struct FakeCollision;

impl Step for FakeCollision {
    fn id(&self) -> &'static str {
        COLLISION_COOK
    }
    fn version(&self) -> u32 {
        1
    }
    fn run(&self, subject: &str, _: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        meshes().remove(subject).map(|m| m.to_bytes()).ok_or_else(|| BakeError::Other(format!("no mesh {subject}")))
    }
}

fn placed(id: u128, model: &str, class: PlacementClass, at: Vec3) -> Placement {
    Placement {
        id: PlacementId(id),
        base: RecordRef::new("STAT", "x"),
        model: Some(VfsPath::new(model).unwrap()),
        transform: WorldTransform { pos: WorldPos::from_f64(at.as_dvec3()), rotation: Quat::IDENTITY, scale: Vec3::ONE },
        class,
        flags: PlacementFlags::empty(),
    }
}

const DOOR_REF: u128 = 0x1234_5678_9abc_def0_0fed_cba9_8765_4321;
/// In front of the door, on the floor.
const NODE: Vec3 = Vec3::new(10.0, 0.0, 8.5);

fn room() -> Vec<Placement> {
    vec![
        placed(1, FLOOR, PlacementClass::Static, Vec3::ZERO),
        placed(2, WALL, PlacementClass::Static, Vec3::ZERO),
        placed(DOOR_REF, DOOR_MODEL, PlacementClass::Door, Vec3::new(10.0, 0.0, 10.0)),
    ]
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("Test Room")
}

fn world() -> NavWorld {
    let placements = room();
    let terrain = BTreeSet::new();
    let input = SpaceInput {
        extent: None,
        water: None,
        placements: placements.iter().collect(),
        terrain: &terrain,
        nodes: vec![NODE, Vec3::new(1.0, 0.0, 1.0)],
    };
    let space = NavSpace::build(&ws(), input, &bounds_of).unwrap();
    NavWorld::new(BTreeMap::from([(ws(), space)]), 0.9)
}

#[test]
fn door_ids_are_stable_and_follow_the_reference() {
    let a = door_id(PlacementId(DOOR_REF));
    assert_eq!(a, door_id(PlacementId(DOOR_REF)));
    assert_eq!(a.0, 0x1234_5678_9abc_def0 ^ 0x0fed_cba9_8765_4321);
    assert_ne!(a, door_id(PlacementId(DOOR_REF + 1)));
    // The whole plan-time world is deterministic.
    let (w1, w2) = (world(), world());
    assert_eq!(w1.spaces, w2.spaces);
}

#[test]
fn cells_list_every_placement_their_bounds_touch() {
    let w = world();
    let s = &w.spaces[&ws()];
    // Placements span 0..20 m: one 64 m cell; extent = bounds + pad.
    assert_eq!(s.cells.keys().copied().collect::<Vec<_>>(), vec![(0, 0)]);
    let c = &s.cells[&(0, 0)];
    assert_eq!(c.solids.iter().map(|o| o.id).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(c.doors.iter().map(|o| o.id).collect::<Vec<_>>(), vec![DOOR_REF]);
    assert_eq!(s.min, Vec2::splat(-INTERIOR_PAD));
    assert_eq!(s.max, Vec2::splat(20.0 + INTERIOR_PAD));
}

#[test]
fn door_volume_covers_the_doorway_and_its_threshold() {
    let w = world();
    let door = &w.spaces[&ws()].cells[&(0, 0)].doors[0];
    let v = door.door_volume();
    assert_eq!(v.door, door_id(PlacementId(DOOR_REF)));
    assert_eq!(v.volume.area, DOOR);
    let lo = v.volume.footprint.iter().copied().fold(Vec2::MAX, Vec2::min);
    let hi = v.volume.footprint.iter().copied().fold(Vec2::MIN, Vec2::max);
    // Width of the leaf (x 9.4..10.6), depth widened to 0.75 m each side of z 10.
    assert!((lo - Vec2::new(9.4, 10.0 - DOOR_HALF_DEPTH)).abs().max_element() < 1e-5, "{lo}");
    assert!((hi - Vec2::new(10.6, 10.0 + DOOR_HALF_DEPTH)).abs().max_element() < 1e-5, "{hi}");
    assert!((v.volume.min_y + DOOR_FLOOR_SLACK).abs() < 1e-5);
    assert!((v.volume.max_y - LEAF_TOP).abs() < 1e-5);
}

#[test]
fn approach_point_is_the_nearest_pathgrid_node() {
    let w = world();
    let links = &w.spaces[&ws()].links;
    assert!(links.links.is_empty());
    assert_eq!(links.doors.len(), 1);
    assert_eq!(links.doors[0].door, door_id(PlacementId(DOOR_REF)));
    assert_eq!(links.doors[0].at.worldspace, ws());
    assert!((WorldPos::ORIGIN.delta_f32(links.doors[0].at.pos) - NODE).length() < 1e-4);
    // No node within the radius: no approach point.
    let placements = room();
    let terrain = BTreeSet::new();
    let input = SpaceInput {
        extent: None,
        water: None,
        placements: placements.iter().collect(),
        terrain: &terrain,
        nodes: vec![Vec3::new(10.0, 0.0, 10.0 - APPROACH_RADIUS - 1.0)],
    };
    assert!(NavSpace::build(&ws(), input, &bounds_of).unwrap().links.doors.is_empty());
}

/// What the source hands the bake for tile (0, 0): (top vertex height, wall-top vertices,
/// leaf-top vertices, door ids, triangles), postcard.
struct Probe(Arc<MwNavSource>);

impl Step for Probe {
    fn id(&self) -> &'static str {
        "test.probe"
    }
    fn version(&self) -> u32 {
        1
    }
    fn run(&self, _: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let class = classes()[0].class.clone();
        // Tile (0, 0) with a 1 m border: the floor, both wall parts and the doorway.
        let b = NavTileBounds { region: 0, tile_x: 0, tile_z: 0, min: Vec2::splat(-1.0), max: Vec2::splat(20.2) };
        let src = self.0.tile_input(cx, &ws(), &class, &b)?;
        let top = src.input.verts.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max);
        let walls = src.input.verts.iter().filter(|v| (v.y - 3.0).abs() < 1e-4).count();
        let leaf = src.input.verts.iter().filter(|v| (v.y - LEAF_TOP).abs() < 1e-4).count();
        let doors: Vec<u64> = src.doors.iter().map(|d| d.door.0).collect();
        Ok(postcard::to_stdvec(&(top, walls, leaf, doors, src.input.tris.len())).unwrap())
    }
}

fn session(dir: &std::path::Path, world: &Arc<NavWorld>) -> (Baker, NavBake) {
    let (baker, bake, _) = session_with(dir, world, FakeCollision);
    (baker, bake)
}

fn session_with(dir: &std::path::Path, world: &Arc<NavWorld>, collision: impl Step) -> (Baker, NavBake, Arc<MwNavSource>) {
    let mut baker = Baker::new(Store::open(dir).unwrap(), Arc::new(NoInputs));
    baker.register(collision);
    let source = Arc::new(MwNavSource::new(world.clone()));
    source.register(&mut baker);
    baker.register(Probe(source.clone()));
    let backend = sf_nav_recast::RecastBackend::new(sf_nav_recast::RecastConfig::new(classes())).unwrap();
    let cfg = NavBakeConfig { classes: classes(), areas: areas(), backend_tag: sf_nav_recast::BAKE_TAG.into() };
    let bake = NavBake::new(source.clone(), Arc::new(backend), cfg).unwrap();
    bake.register(&mut baker);
    (baker, bake, source)
}

/// The fake cook with the wall raised to [`RAISED_WALL`] m (version 2, so it re-runs).
struct RaisedWallCollision;

const RAISED_WALL: f32 = 4.0;

impl Step for RaisedWallCollision {
    fn id(&self) -> &'static str {
        COLLISION_COOK
    }
    fn version(&self) -> u32 {
        2
    }
    fn run(&self, subject: &str, _: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let mut mesh = meshes().remove(subject).ok_or_else(|| BakeError::Other(format!("no mesh {subject}")))?;
        if subject == WALL {
            for p in &mut mesh.positions {
                if p[1] > 1.0 {
                    p[1] = RAISED_WALL;
                }
            }
        }
        Ok(mesh.to_bytes())
    }
}

#[test]
fn tiles_share_decoded_geometry_and_still_rebake_on_a_changed_mesh() {
    let dir = tempfile::tempdir().unwrap();
    let world = Arc::new(world());
    let (baker, bake, source) = session_with(dir.path(), &world, FakeCollision);
    // The probe decodes the room first; the tiles then read the same products, shared.
    let probe = ProductKey::new("test.probe", "room");
    let mut keys = vec![probe.clone()];
    keys.extend(bake.class_products(&[ws()], HUMANOID));
    for key in &keys {
        baker.product(key).unwrap();
    }
    let g = source.geometry_stats();
    // Distinct products: the floor and wall meshes (the door leaf is not read), the listed cells
    // and one empty cell.
    assert!(g.decoded < g.reads, "{g:?}");
    let cells = u64::try_from(world.spaces[&ws()].cells.len()).unwrap();
    assert!(g.decoded <= 3 + cells, "each product decoded once: {g:?}");

    // A new session with a taller wall: the tiles over the wall re-bake (their reads were
    // recorded even when the decode was shared) and see the new wall.
    let (again, bake, _) = session_with(dir.path(), &world, RaisedWallCollision);
    let report = again.bake(&bake.class_products(&[ws()], HUMANOID));
    assert_eq!(report.failures().count(), 0);
    assert!(again.step_stats()[sf_nav_bake::TILES].built > 0, "tiles over the wall re-bake");
    let (top, ..): (f32, usize, usize, Vec<u64>, usize) = postcard::from_bytes(&bytes(&again, &probe)).unwrap();
    assert!((top - RAISED_WALL).abs() < 1e-5, "{top}");
}

#[test]
fn decoded_cache_evicts_least_recently_used_past_its_budget() {
    let d = |i: u8| Digest([i; 32]);
    let cache: Decoded<u8> = Decoded::new(10);
    cache.insert(d(1), Arc::new(1), 4);
    cache.insert(d(2), Arc::new(2), 4);
    assert_eq!(cache.get(&d(1)).as_deref(), Some(&1)); // 1 is now the most recent
    cache.insert(d(3), Arc::new(3), 4); // 12 > 10: evicts 2
    assert!(cache.contains(&d(1)) && !cache.contains(&d(2)) && cache.contains(&d(3)));
    cache.insert(d(4), Arc::new(4), 20); // over budget alone: keeps only itself
    assert!(!cache.contains(&d(1)) && !cache.contains(&d(3)) && cache.contains(&d(4)));
    cache.clear();
    assert!(cache.get(&d(4)).is_none());
}

fn bytes(baker: &Baker, key: &ProductKey) -> Vec<u8> {
    baker.store().get(&baker.product(key).unwrap()).unwrap()
}

#[test]
fn the_door_leaf_is_not_geometry_but_the_wall_is() {
    let dir = tempfile::tempdir().unwrap();
    let (baker, _) = session(dir.path(), &Arc::new(world()));
    let key = ProductKey::new("test.probe", "room");
    let report = baker.bake(std::slice::from_ref(&key));
    assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
    let (top, walls, leaf, doors, tris): (f32, usize, usize, Vec<u64>, usize) = postcard::from_bytes(&bytes(&baker, &key)).unwrap();
    // Floor 12 + wall 24 triangles, no leaf: the wall (3 m) is the top, nothing at the leaf's 2.2 m.
    assert_eq!(tris, 36);
    assert!((top - 3.0).abs() < 1e-5, "{top}");
    assert_eq!(walls, 8);
    assert_eq!(leaf, 0);
    assert_eq!(doors, vec![door_id(PlacementId(DOOR_REF)).0]);
}

#[test]
fn bake_gives_the_door_polygons_and_an_approach_point_then_hits() {
    let dir = tempfile::tempdir().unwrap();
    let world = Arc::new(world());
    let (baker, bake) = session(dir.path(), &world);
    let keys = bake.products(&[ws()]);
    let report = baker.bake(&keys);
    assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
    let door = door_id(PlacementId(DOOR_REF));
    let links: NavLinks = postcard::from_bytes(&bytes(&baker, &ProductKey::new(sf_nav_bake::LINKS, ws().as_str()))).unwrap();
    assert_eq!(links.doors.iter().map(|d| d.door).collect::<Vec<_>>(), vec![door]);
    let mut door_tiles = 0;
    for k in bake.tile_keys(&ws(), HUMANOID) {
        let b = bytes(&baker, &ProductKey::new(sf_nav_bake::TILES, sf_nav_bake::tile_subject(&k)));
        if let Some(t) = BakedTile::decode(&b).unwrap() {
            door_tiles += usize::from(t.doors.iter().any(|d| d.door == door && !d.polys.is_empty()));
        }
    }
    assert!(door_tiles > 0, "some humanoid tile lists the doorway's polygons");

    // A new session over the same store: every nav product is a hit.
    let (again, bake) = session(dir.path(), &world);
    let report = again.bake(&bake.products(&[ws()]));
    assert_eq!(report.failures().count(), 0);
    for (step, s) in again.step_stats() {
        assert_eq!(s.built, 0, "{step} rebuilt on an unchanged world");
    }
}

// NV4m: where the real navmesh bake spends its time (measurement only).

/// One class over one sample, summed over its tiles: the real `tile_input` (gather; its parts
/// from the source's [`GeometryStats`]) + Recast build + encode + store.
#[derive(Default)]
struct Cost {
    tiles: usize,
    polys: usize,
    with_input: usize,
    content: usize,
    content_time: Duration,
    empty_time: Duration,
    geometry: GeometryStats,
    gather: Duration,
    build: Duration,
    encode: Duration,
    store: Duration,
    tris: Vec<usize>,
    underwater: usize,
    underwater_time: Duration,
    bytes: usize,
}

/// Whether any nav cell under the tile has terrain, solids or doors (plan-time data only: the
/// whole-world extrapolation splits tiles by this).
fn has_content(space: &NavSpace, b: &NavTileBounds) -> bool {
    let (lo, hi) = (b.min.max(space.min), b.max.min(space.max));
    if lo.x > hi.x || lo.y > hi.y {
        return false;
    }
    cell_range(lo.x, hi.x, NAV_CELL).any(|x| {
        cell_range(lo.y, hi.y, NAV_CELL).any(|z| {
            space.cells.get(&(x, z)).is_some_and(|c| !(c.terrain.is_empty() && c.solids.is_empty() && c.doors.is_empty()))
        })
    })
}

/// Measured costs: sample label, class name, cost.
type Results = Arc<Mutex<Vec<(String, &'static str, Cost)>>>;

/// A sample: a label and its areas (worldspace + XZ rectangle, region-local meters).
type Sample = (String, Vec<(WorldspaceId, Vec2, Vec2)>);

struct CostProbe {
    world: Arc<NavWorld>,
    source: Arc<MwNavSource>,
    backend: Arc<sf_nav_recast::RecastBackend>,
    scratch: Store,
    samples: Vec<Sample>,
    out: Results,
}

impl CostProbe {
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, reason = "tile indices")]
    fn measure(&self, cx: &mut RunCx<'_>, cfg: &BuildConfig, areas: &[(WorldspaceId, Vec2, Vec2)]) -> Result<Cost, BakeError> {
        use sf_nav::NavBackend;
        let table = super::areas();
        let other = |e: &dyn std::fmt::Display| BakeError::Other(e.to_string());
        let mut c = Cost::default();
        let before = self.source.geometry_stats();
        let (size, border) = (cfg.tile_size(), cfg.border());
        for (ws, alo, ahi) in areas {
            let space = &self.world.spaces[ws];
            for x in (alo.x / size).floor() as i32..=((ahi.x - 1e-3) / size).floor() as i32 {
                for z in (alo.y / size).floor() as i32..=((ahi.y - 1e-3) / size).floor() as i32 {
                    let lo = Vec2::new(x as f32 * size, z as f32 * size);
                    let bounds = NavTileBounds { region: 0, tile_x: x, tile_z: z, min: lo - border, max: lo + size + border };
                    let content = has_content(space, &bounds);
                    let t = Instant::now();
                    let real = self.source.tile_input(cx, ws, &cfg.class, &bounds)?;
                    let gather = t.elapsed();
                    c.gather += gather;
                    // The water quad sits below the surface, so this is "all geometry under it".
                    let top = real.input.verts.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max);
                    let underwater = !real.input.tris.is_empty() && space.water.is_some_and(|w| top < w);
                    c.underwater += usize::from(underwater);
                    let NavTileSource { mut input, mut doors } = real;
                    if !input.tris.is_empty() {
                        c.with_input += 1;
                        c.tris.push(input.tris.len());
                    }
                    input.tile_x = x;
                    input.tile_z = z;
                    doors.sort_by_key(|d| d.door);
                    input.volumes.extend(doors.iter().map(|d| d.volume.clone()));
                    let t = Instant::now();
                    let blob = self.backend.build_tile(&input, cfg, &table).map_err(|e| other(&e))?;
                    let build = t.elapsed();
                    c.build += build;
                    let (mut encode, mut store) = (Duration::ZERO, Duration::ZERO);
                    if !blob.is_empty() {
                        c.polys += 1;
                        let t = Instant::now();
                        self.backend.read_tile_blob(&blob).map_err(|e| other(&e))?;
                        let bytes = BakedTile { blob, doors: Vec::new() }.encode().map_err(|e| other(&e))?;
                        encode = t.elapsed();
                        let t = Instant::now();
                        self.scratch.put(&bytes, true).map_err(|e| other(&e))?;
                        store = t.elapsed();
                        c.bytes += bytes.len();
                    }
                    c.encode += encode;
                    c.store += store;
                    let tile = gather + build + encode + store;
                    c.tiles += 1;
                    if content {
                        c.content += 1;
                        c.content_time += tile;
                    } else {
                        c.empty_time += tile;
                    }
                    if underwater {
                        c.underwater_time += tile;
                    }
                }
            }
        }
        c.geometry = self.source.geometry_stats() - before;
        Ok(c)
    }
}

impl Step for CostProbe {
    fn id(&self) -> &'static str {
        "test.nv4m.cost"
    }
    fn version(&self) -> u32 {
        1
    }
    fn run(&self, _: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        for (label, areas) in &self.samples {
            for (cfg, name) in classes().iter().zip(["humanoid", "large"]) {
                let c = self.measure(cx, cfg, areas)?;
                self.out.lock().unwrap().push((label.clone(), name, c));
            }
        }
        Ok(Vec::new())
    }
}

fn ms(d: Duration, n: usize) -> f64 {
    #[allow(clippy::cast_precision_loss, reason = "display")]
    let n = n.max(1) as f64;
    d.as_secs_f64() * 1000.0 / n
}

#[test]
#[ignore = "measurement: nav bake cost"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
fn nav_bake_cost_over_real_cells() {
    let started = Instant::now();
    let explicit = std::env::var_os("MORROWIND_DIR").map(std::path::PathBuf::from);
    let (install, _) = mw_install::MorrowindInstall::detect(explicit.as_deref()).expect("Morrowind install");
    let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).expect("vfs").vfs;
    let content = mw_records::load(&vfs, &install.game_files()).expect("records");
    let root = std::env::var_os("SF_NAV_CACHE").map_or_else(|| crate::default_cache_root().expect("cache root"), std::path::PathBuf::from);
    let mut models = crate::referenced_models(&content.db);
    let importers = crate::importers();
    models.retain(|p, _| importers.for_path(p).is_some());
    let paths: Vec<String> = models.keys().cloned().collect();
    let store = Store::open(&root).expect("store");
    let db = Arc::new(content.db);
    let vfs = Arc::new(vfs);
    let terrain = Arc::new(mw_world::MwTerrainSource::new(db.clone()));
    let layers = Arc::new(crate::world::MwLayers::new(&db, &vfs));
    let extra: Vec<_> = crate::world::source_tile_fingerprints(&terrain).into_iter().chain(layers.fingerprints()).collect();
    let inputs = Arc::new(
        crate::VfsInputs::new(vfs.clone(), &install, root.join("filehash.memo")).with_records(db.clone()).with_extra_records(extra),
    );
    println!("nv4m: content loaded in {:.1?}", started.elapsed());
    let mut baker = crate::model_baker(store, inputs.clone());
    let t = Instant::now();
    let mut keys = crate::model_keys(&paths);
    keys.extend(crate::collision::collision_keys(&paths));
    let r = baker.bake(&keys);
    println!("nv4m: models + collision in {:.1?} ({} failed)", t.elapsed(), r.failures().count());
    let t = Instant::now();
    crate::world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
    inputs.save().expect("memo");
    println!("nv4m: exterior + terrain in {:.1?}", t.elapsed());
    let t = Instant::now();
    let world = Arc::new(nav_world(&baker, &db, &paths));
    println!("nv4m: nav_world in {:.1?}, {} worldspaces", t.elapsed(), world.spaces.len());

    let source = Arc::new(MwNavSource::new(world.clone()));
    source.register(&mut baker);
    let backend = Arc::new(sf_nav_recast::RecastBackend::new(sf_nav_recast::RecastConfig::new(classes())).unwrap());
    let cfg = NavBakeConfig { classes: classes(), areas: areas(), backend_tag: sf_nav_recast::BAKE_TAG.into() };
    let bake = NavBake::new(source.clone(), backend.clone(), cfg).unwrap();

    // 3 x 3 exterior cells around a grid cell (world z = -Morrowind y).
    let k = (mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64) as f32;
    let ext = ids::exterior();
    let around = |gx: i32, gy: i32| {
        vec![(ext.clone(), Vec2::new((gx - 1) as f32 * k, -(gy + 2) as f32 * k), Vec2::new((gx + 2) as f32 * k, -(gy - 1) as f32 * k))]
    };
    let interiors: Vec<_> = world
        .spaces
        .iter()
        .filter(|(ws, _)| **ws != ext && ws.as_str().to_ascii_lowercase().starts_with("balmora"))
        .map(|(ws, s)| (ws.clone(), s.min, s.max))
        .collect();
    let n_int = interiors.len();
    let samples: Vec<Sample> =
        vec![("balmora".into(), around(-3, -2)), ("seyda_neen".into(), around(-2, -9)), (format!("balmora_interiors({n_int})"), interiors)];
    let out = Arc::new(Mutex::new(Vec::new()));
    let scratch_dir = tempfile::tempdir().unwrap();
    baker.register(CostProbe {
        world: world.clone(),
        source: source.clone(),
        backend,
        scratch: Store::open(scratch_dir.path()).unwrap(),
        samples,
        out: out.clone(),
    });
    let nonce = format!("{:?}", std::time::SystemTime::now());
    let t = Instant::now();
    baker.product(&ProductKey::new("test.nv4m.cost", nonce)).expect("measurement ran");
    println!("nv4m: measured in {:.1?} (one thread, shared geometry decode)", t.elapsed());

    let out = out.lock().unwrap();
    for (label, class, c) in out.iter() {
        let n = c.tiles;
        let mean_tris = c.tris.iter().sum::<usize>() as f64 / c.tris.len().max(1) as f64;
        let max_tris = c.tris.iter().copied().max().unwrap_or(0);
        let g = c.geometry;
        println!(
            "nv4m {label} {class}: tiles {n} (polys {} / empty {}; input {}; content cells {}) | per tile ms: gather {:.2} build {:.2} encode {:.3} store {:.3} = {:.2}",
            c.polys,
            n - c.polys,
            c.with_input,
            c.content,
            ms(c.gather, n),
            ms(c.build, n),
            ms(c.encode, n),
            ms(c.store, n),
            ms(c.gather + c.build + c.encode + c.store, n)
        );
        println!(
            "nv4m {label} {class}:   gather parts ms/tile: resolve {:.3} read {:.3} decode {:.3} rest (lists, push) {:.3}; {} reads, {} decoded ({:.2} ms per decoded read, {:.2} ms decode)",
            ms(g.resolve, n),
            ms(g.read, n),
            ms(g.decode, n),
            ms(c.gather.saturating_sub(g.resolve + g.read + g.decode), n),
            g.reads,
            g.decoded,
            ms(g.read, g.decoded as usize),
            ms(g.decode, g.decoded as usize)
        );
        println!(
            "nv4m {label} {class}:   tris/tile (with input) mean {mean_tris:.0} max {max_tris}; underwater {} tiles ({:.2} ms each); {:.1} KiB stored; content tile {:.2} ms, empty tile {:.3} ms",
            c.underwater,
            ms(c.underwater_time, c.underwater),
            c.bytes as f64 / 1024.0,
            ms(c.content_time, c.content),
            ms(c.empty_time, n - c.content)
        );
    }

    // First reads: store objects this process has not read yet (collision products the samples
    // never touched), read twice. The first read pays the OS file open + cold cache.
    let unread: Vec<Digest> = baker
        .manifest()
        .products
        .iter()
        .filter(|(k, d)| k.step == COLLISION_COOK && !source.cache.meshes.contains(d))
        .map(|(_, d)| *d)
        .take(400)
        .collect();
    let t = Instant::now();
    let size: usize = unread.iter().map(|d| baker.store().get(d).map_or(0, |b| b.len())).sum();
    let first = t.elapsed();
    let t = Instant::now();
    for d in &unread {
        let _ = baker.store().get(d);
    }
    let second = t.elapsed();
    println!(
        "nv4m first reads: {} unread collision objects ({:.1} KiB mean): first read {:.3} ms, second {:.3} ms each",
        unread.len(),
        size as f64 / 1024.0 / unread.len().max(1) as f64,
        ms(first, unread.len()),
        ms(second, unread.len())
    );

    // Whole world: tiles per class, split exterior / interior and content / empty cells.
    let mut total_cpu = 0.0;
    for (i, cfg) in classes().iter().enumerate() {
        let name = ["humanoid", "large"][i];
        let (size, border) = (cfg.tile_size(), cfg.border());
        let (mut ec, mut ee, mut ic, mut ie) = (0usize, 0usize, 0usize, 0usize);
        for (ws, space) in &world.spaces {
            for key in bake.tile_keys(ws, cfg.class.id) {
                let lo = Vec2::new(key.x as f32 * size, key.z as f32 * size);
                let b = NavTileBounds { region: 0, tile_x: key.x, tile_z: key.z, min: lo - border, max: lo + size + border };
                let content = has_content(space, &b);
                match (*ws == ext, content) {
                    (true, true) => ec += 1,
                    (true, false) => ee += 1,
                    (false, true) => ic += 1,
                    (false, false) => ie += 1,
                }
            }
        }
        let pick = |interior: bool| {
            let (mut ct, mut cn, mut et, mut en) = (Duration::ZERO, 0, Duration::ZERO, 0);
            for (label, class, c) in out.iter() {
                if *class == name && label.starts_with("balmora_interiors") == interior {
                    ct += c.content_time;
                    cn += c.content;
                    et += c.empty_time;
                    en += c.tiles - c.content;
                }
            }
            (ms(ct, cn), ms(et, en))
        };
        let ((ext_c, ext_e), (int_c, int_e)) = (pick(false), pick(true));
        let cpu = (ec as f64 * ext_c + ee as f64 * ext_e + ic as f64 * int_c + ie as f64 * int_e) / 1000.0;
        total_cpu += cpu;
        println!(
            "nv4m world {name}: exterior {ec} content + {ee} empty tiles ({ext_c:.2} / {ext_e:.3} ms), interiors {ic} content + {ie} empty ({int_c:.2} / {int_e:.3} ms) -> {cpu:.0} CPU-s, {:.0} s on 16 threads",
            cpu / 16.0
        );
    }
    println!("nv4m world total: {total_cpu:.0} CPU-s, {:.0} s on 16 threads (ideal scaling)", total_cpu / 16.0);
}
