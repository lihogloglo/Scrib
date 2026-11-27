//! Headless residency tests (B3a, B3b-1, B3b-2; brief §6.2): real `scene.cells` tables and
//! `scene.chains` products off a bake store, through real streaming.
//!
//! The fixture is one streaming cell of plates, every placement at
//! `Level::Near` (`far_min_radius = f32::MAX`: at the default radius the
//! plates are far-eligible and the near step, the only level the scene draws,
//! is empty — `sf_world_bevy`'s `streaming_app_radius` note). Its table names
//! up to two chains of one model, `plate.mdl` parts 0 and 1, both stored as the
//! same cooked plate. No render app: [`ChainSlabTable`] resolves only what a
//! test injects by hand, and asserts its length first.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::asset::{AssetPlugin, AssetServer, LoadState, UntypedAssetId, UntypedHandle};
use bevy::mesh::MeshPlugin;
use bevy::pbr::gpu_instance_batch::{GpuBatchedMesh3d, GpuBatchedMeshRange, validate_ranges};
use bevy::prelude::*;
use bevy::render::sync_world::MainEntity;
use big_space::prelude::FloatingOrigin;
use glam::{Affine3A, DVec3};
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::Counters;
use sf_model::{CookSettings, Cooked, Material as ModelMaterial, MeshData, Model, Node, cook};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_scene::{
    Aabb, BatchId, CELLS_STEP, CHAINS_STEP, CellPlan, InstanceFlags, LodChain, LodLevel, MaterialClass,
    RangeSpec, Sphere, SpaceConfig, StaticCellTable, StaticInstance, cell_subject, chain_subject,
};
use sf_vfs::VfsPath;
use sf_world::{
    IndexParams, IndexedPlacement, MemoryIndex, ModelInfo, Partition, Placement, PlacementClass, PlacementFlags,
    PlacementId, RecordRef, WorldspaceId,
};
use sf_world_bevy::{
    ActiveWorldspace, CellScene, PlacementSpawner, PlacementSpawners, SceneCellCx, SceneCells, SpawnCx, Streaming,
    StreamingAnchor, WorldIndexSource, WorldStreamingPlugin, WorldspaceRegistry, to_grid,
};

use super::*;

const PLACEMENTS: usize = 9;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

/// A flat 8 × 8 m grid, fine enough for a LOD chain (`sf_world_bevy`'s test plate).
fn plate() -> Model {
    let n = 24u32;
    let mut mesh = MeshData { normals: Some(Vec::new()), ..MeshData::default() };
    for z in 0..=n {
        for x in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            let (fx, fz) = (x as f32 / n as f32, z as f32 / n as f32);
            mesh.positions.push([fx * 8.0 - 4.0, 0.3 * (fx * 9.0).sin() * (fz * 7.0).cos(), fz * 8.0 - 4.0]);
            mesh.normals.as_mut().unwrap().push([0.0, 1.0, 0.0]);
        }
    }
    let row = n + 1;
    for z in 0..n {
        for x in 0..n {
            let i = z * row + x;
            mesh.indices.extend([i, i + row, i + 1, i + 1, i + row, i + row + 1]);
        }
    }
    let mut node = Node::new("plate");
    node.mesh = Some(0);
    Model { nodes: vec![node], meshes: vec![mesh], materials: vec![ModelMaterial::default()] }
}

/// `PLACEMENTS` static plates, 3 m apart, all in one streaming cell.
fn placements() -> Vec<Placement> {
    (0..PLACEMENTS)
        .map(|i| {
            let key = format!("p{i}");
            #[allow(clippy::cast_precision_loss)]
            let x = 1.0 + i as f64 * 3.0;
            Placement {
                id: PlacementId::from_key("test", key.as_bytes()),
                base: RecordRef::new("thing", key.as_str()),
                model: Some(VfsPath::new("plate.mdl").unwrap()),
                transform: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(x, 0.0, 2.0))),
                class: PlacementClass::Static,
                flags: PlacementFlags::empty(),
            }
        })
        .collect()
}

/// What the cell's `scene.cells` product is.
#[derive(Clone, Copy)]
enum TableKind {
    /// A valid table; `parts(k)` lists the chains (0 or 1) instance `k` draws.
    Valid(fn(usize) -> Vec<u32>),
    /// No product at all (not baked).
    Missing,
    /// Bytes the loader rejects.
    Corrupt,
}

fn chain_a(_: usize) -> Vec<u32> {
    vec![0]
}

fn chain_a_and_b(k: usize) -> Vec<u32> {
    if k % 2 == 1 { vec![0, 1] } else { vec![0] }
}

/// Placements 0-2 draw chain 0 only, placement 3 both chains, the rest chain 1
/// only: with capacity 4 and one batch per (unresolved) chain, chain 0's
/// bucket (4 instances) fits and chain 1's (6) overflows.
fn overflow_split(k: usize) -> Vec<u32> {
    match k {
        0..=2 => vec![0],
        3 => vec![0, 1],
        _ => vec![1],
    }
}

struct Fixture {
    app: App,
    cell: IVec3,
    /// The cell's placements in index order: a table's cell-local id indexes this.
    indexed: Vec<PlacementId>,
    /// Instances per local placement.
    parts: Vec<Vec<u32>>,
    counters: Arc<Counters>,
    spawned: Arc<Mutex<Vec<PlacementId>>>,
}

impl Fixture {
    fn ids(&self, locals: impl IntoIterator<Item = usize>) -> HashSet<PlacementId> {
        locals.into_iter().map(|k| self.indexed[k]).collect()
    }

    fn all(&self) -> HashSet<PlacementId> {
        self.indexed.iter().copied().collect()
    }

    fn spawned(&self) -> HashSet<PlacementId> {
        let s = self.spawned.lock().unwrap();
        let set: HashSet<PlacementId> = s.iter().copied().collect();
        assert_eq!(set.len(), s.len(), "a placement spawned twice: {s:?}");
        set
    }

    /// Every counter's total since the start (nothing else drains them here). Drains: call once.
    fn counters(&self) -> HashMap<String, f64> {
        let mut out = HashMap::new();
        self.counters.drain(|_, def, value| {
            if let Some(v) = value {
                out.insert(def.name.clone(), v);
            }
        });
        out
    }

    fn state(&self) -> &SpaceState {
        &self.app.world().resource::<SceneSpaces>().0[&ws()]
    }
}

/// Records the static placements it is asked to spawn; needs their model, so
/// a placement the scene does not cover has its model gathered and loaded.
struct Recorder(Arc<Mutex<Vec<PlacementId>>>);

impl PlacementSpawner for Recorder {
    fn spawn(&self, _world: &mut World, _cx: &SpawnCx<'_>, p: &IndexedPlacement) -> u32 {
        self.0.lock().unwrap().push(p.placement.id);
        0
    }
    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        true
    }
}

/// The cooked plate, and its LOD level count (a chain level per cooked level).
fn cooked_plate() -> (Vec<u8>, usize) {
    cooked_plate_with(ModelMaterial::default())
}

/// The plate cooked with `material` as its only material.
fn cooked_plate_with(material: ModelMaterial) -> (Vec<u8>, usize) {
    let mut model = plate();
    model.materials = vec![material];
    let bytes = cook(&model, &CookSettings::default()).unwrap().to_bytes();
    let Cooked::Static(model) = Cooked::from_bytes(&bytes).unwrap() else { panic!("the plate cooks static") };
    (bytes, model.lods.len())
}

/// Chain 0's material key; chain 1's is `b_material`.
const A_MATERIAL: u64 = 0xA5A5;

fn table_bytes(levels: usize, parts: &[Vec<u32>], b_material: u64) -> Vec<u8> {
    let levels: Vec<LodLevel> = (0..levels)
        .map(|l| {
            #[allow(clippy::cast_precision_loss)]
            let error = l as f32 * 0.01;
            LodLevel { first_index: 0, index_count: 600, error }
        })
        .collect();
    let mut instances = Vec::new();
    for (k, chains) in parts.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let center = Vec3::new(k as f32 * 3.0, 0.0, 0.0);
        for &chain in chains {
            instances.push(StaticInstance {
                placement: u32::try_from(k).unwrap(),
                transform: StaticInstance::rows(&Affine3A::from_translation(center)),
                sphere: Sphere { center, radius: 6.0 },
                chain,
                material_override: 0,
                flags: InstanceFlags::CASTS_SHADOW,
            });
        }
    }
    let chain = |mesh, part| LodChain {
        mesh,
        material: if part == 0 { A_MATERIAL } else { b_material },
        material_class: MaterialClass::Opaque,
        model: 0,
        part,
        levels: levels.clone(),
    };
    StaticCellTable {
        origin_cell: IVec3::ZERO,
        aabb: Aabb { min: Vec3::splat(-64.0), max: Vec3::splat(64.0) },
        land_min: -1.0,
        layers: 1,
        instances,
        // A table names only the chains its instances use (`TableError::UnusedChain`).
        chains: if parts.iter().flatten().any(|&c| c == 1) { vec![chain(0x10, 0), chain(0x20, 1)] } else { vec![chain(0x10, 0)] },
        models: vec!["plate.mdl".into()],
    }
    .write()
    .unwrap()
}

/// The store, the index and the streaming app with the scene enabled.
/// `chain_b`: whether chain 1's product is in the store.
fn fixture(dir: &std::path::Path, kind: TableKind, chain_b: bool, capacity: u32) -> Fixture {
    fixture_with(dir, kind, chain_b, capacity, None)
}

/// [`fixture`]; with `b_material`, chain 1's product is the plate cooked with that material and
/// the table gives chain 1 its own material key (`B_MATERIAL`).
fn fixture_with(
    dir: &std::path::Path,
    kind: TableKind,
    chain_b: bool,
    capacity: u32,
    b_material: Option<ModelMaterial>,
) -> Fixture {
    const B_MATERIAL: u64 = 0xB6B6;
    let store = Store::open(dir).unwrap();
    let (cooked, levels) = cooked_plate();
    let b_key = if b_material.is_some() { B_MATERIAL } else { A_MATERIAL };
    let cooked_b = b_material.map_or_else(|| cooked.clone(), |m| cooked_plate_with(m).0);
    let mut manifest = Manifest::default();
    let put = |m: &mut Manifest, step: &str, subject: String, bytes: &[u8]| {
        m.products.insert(ProductKey::new(step, subject), store.put(bytes, true).unwrap());
    };
    put(&mut manifest, "mesh.cook", "plate.mdl".into(), &cooked);
    put(&mut manifest, CHAINS_STEP, chain_subject("plate.mdl", 0), &cooked);
    if chain_b {
        put(&mut manifest, CHAINS_STEP, chain_subject("plate.mdl", 1), &cooked_b);
    }

    let models = |_: &VfsPath| {
        Some(ModelInfo { min: Vec3::new(-4.0, -0.3, -4.0), max: Vec3::new(4.0, 0.3, 4.0), bytes: 50_000, lod_distance: 0.0 })
    };
    let params = IndexParams { far_min_radius: f32::MAX, ..IndexParams::default() };
    let all = placements();
    let cell = Partition::STREAMING.cell_of(all[0].transform.pos);
    assert!(all.iter().all(|p| Partition::STREAMING.cell_of(p.transform.pos) == cell), "the fixture spans cells");
    let (pages, _) = sf_world::build_pages(&params, all, &models);
    let indexed: Vec<PlacementId> =
        pages.iter().find_map(|p| p.cell(cell)).expect("the cell is indexed").placements.iter().map(|p| p.placement.id).collect();
    assert_eq!(indexed.len(), PLACEMENTS);

    let mut parts = Vec::new();
    let subject = cell_subject(ws().as_str(), cell);
    match kind {
        TableKind::Valid(f) => {
            parts = (0..PLACEMENTS).map(f).collect();
            put(&mut manifest, CELLS_STEP, subject, &table_bytes(levels, &parts, b_key));
        }
        TableKind::Missing => {}
        TableKind::Corrupt => {
            let mut bytes = table_bytes(levels, &[vec![0]], b_key);
            bytes.truncate(bytes.len() - 8);
            put(&mut manifest, CELLS_STEP, subject, &bytes);
        }
    }

    let directory = sf_world::build_directory(ws(), &params, &pages);
    let mut index = MemoryIndex::new();
    index.insert(&sf_world::WorldIndex { directory, pages, stats: sf_world::IndexStats::default() });

    let mut extensions = BTreeMap::from([("mesh.cook".to_owned(), "sfmesh".to_owned())]);
    extensions.extend(baked_extensions());
    let mut app = App::new();
    app.register_asset_source(BAKED_SOURCE, baked_source(store, BakedManifest::new(manifest), extensions));
    let config = SceneConfig {
        enabled: true,
        space: SpaceConfig::default().with_uniform_capacity(capacity),
        ..SceneConfig::default()
    };
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .add_plugins(WorldStreamingPlugin::default())
        .add_plugins(ScenePlugin { config })
        .insert_resource(WorldIndexSource(Arc::new(index)))
        .insert_resource(ActiveWorldspace(Some(ws())));
    app.finish();
    app.cleanup();
    let counters = Arc::new(Counters::new());
    app.insert_resource(DiagCounters(counters.clone()));
    let spawned = Arc::new(Mutex::new(Vec::new()));
    let mut spawners = PlacementSpawners::default();
    spawners.set(PlacementClass::Static, Arc::new(Recorder(spawned.clone())));
    app.insert_resource(spawners);
    Fixture { app, cell, indexed, parts, counters, spawned }
}

/// Puts the anchors at the origin and starts a burst.
fn anchor(app: &mut App) -> Entity {
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(WorldPos::from_f64(DVec3::new(0.0, 2.0, 0.0)));
    let camera = app.world_mut().spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root))).id();
    app.world_mut().spawn((to_grid(WorldPos::ORIGIN).0, Transform::IDENTITY, StreamingAnchor::gameplay(), ChildOf(root)));
    app.world_mut().resource_mut::<Streaming>().begin_burst();
    camera
}

/// Runs frames until streaming is idle.
fn settle(app: &mut App) {
    for frame in 0..5000 {
        app.update();
        if frame > 3 && app.world().resource::<Streaming>().is_idle() {
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("streaming never went idle");
}

/// Runs frames until `done`.
fn pump(app: &mut App, what: &str, mut done: impl FnMut(&mut App) -> bool) {
    for _ in 0..5000 {
        app.update();
        if done(app) {
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("never: {what}");
}

/// What one `covered` call saw.
struct Seen {
    cell: IVec3,
    /// The cell's table had resolved (loaded or failed).
    table_resolved: bool,
    /// The cell's plan, if it was planned.
    plan: Option<CellPlan>,
    decided: bool,
    answer: Option<Arc<HashSet<PlacementId>>>,
}

/// [`SceneHook`] with a log; `bypass_ready` answers `ready` true always, so
/// only `acquire`'s handle can hold the fetch (gate 1 alone).
#[derive(Default)]
struct Probe {
    bypass_ready: bool,
    seen: Mutex<Vec<Seen>>,
    acquired: Mutex<Vec<IVec3>>,
}

impl CellScene for Probe {
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle> {
        self.acquired.lock().unwrap().push(cell);
        SceneHook.acquire(world, worldspace, cell)
    }
    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool {
        self.bypass_ready || SceneHook.ready(world, worldspace, cell)
    }
    fn covered(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3, indexed: &[PlacementId]) -> Option<Arc<HashSet<PlacementId>>> {
        let asset_server = world.resource::<AssetServer>();
        let load = world.resource::<SceneSpaces>().0.get(worldspace).and_then(|s| s.loading.get(&cell));
        let table_resolved = load.is_some_and(|l| {
            matches!(asset_server.get_load_state(l.table.id()), Some(LoadState::Loaded | LoadState::Failed(_)))
        });
        let plan = load.and_then(|l| match &l.phase {
            LoadPhase::Planned { plan } => Some((**plan).clone()),
            _ => None,
        });
        let decided = load.is_some_and(|l| l.phase.decided());
        let answer = SceneHook.covered(world, worldspace, cell, indexed);
        self.seen.lock().unwrap().push(Seen { cell, table_resolved, plan, decided, answer: answer.clone() });
        answer
    }
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> u64 {
        SceneHook.upload(world, cx)
    }
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3) {
        SceneHook.release(world, worldspace, cell);
    }
}

/// Anchors at the origin, waits for the cell's near load to start (streaming
/// reads idle before its directory is in), then settles.
fn stream_in(f: &mut Fixture, probe: &Probe) {
    anchor(&mut f.app);
    let cell = f.cell;
    pump(&mut f.app, "the cell's near load started", |_| probe.acquired.lock().unwrap().contains(&cell));
    settle(&mut f.app);
}

fn install(f: &mut Fixture, probe: Probe) -> Arc<Probe> {
    let probe = Arc::new(probe);
    f.app.insert_resource(SceneCells(Some(probe.clone())));
    probe
}

/// The scene answers `covered` for exactly this cell, once.
fn the_covered_call(probe: &Probe, cell: IVec3) -> Seen {
    let mut seen = probe.seen.lock().unwrap();
    assert_eq!(seen.iter().filter(|s| s.cell == cell).count(), 1, "covered asked once for the cell");
    let i = seen.iter().position(|s| s.cell == cell).unwrap();
    seen.swap_remove(i)
}

/// Gate 1 alone: with `ready` bypassed, only the table handle `acquire`
/// returned can hold the fetch, and `covered` still never sees a table that is
/// still arriving.
#[test]
fn acquires_table_handle_holds_the_fetch_open() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let probe = install(&mut f, Probe { bypass_ready: true, ..Probe::default() });
    stream_in(&mut f, &probe);
    assert!(probe.acquired.lock().unwrap().contains(&f.cell), "the cell's near load acquired its table");
    let seen = the_covered_call(&probe, f.cell);
    assert!(seen.table_resolved, "covered was asked before the cell's table resolved");
}

/// The ruling: the fetch stays open until the cell is planned. Held at gate 3
/// (a render world that has not allocated the chain meshes yet), `covered` is
/// not asked; once the slabs arrive the cell plans with them (one shared
/// batch, slab-keyed) and `covered` is exactly the placed set.
#[test]
fn the_fetch_waits_until_the_cell_is_planned() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let probe = install(&mut f, Probe::default());
    f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    anchor(&mut f.app);
    let cell = f.cell;
    let phase_is_slabs = |app: &mut App| {
        app.world().resource::<SceneSpaces>().0.get(&ws()).and_then(|s| s.loading.get(&cell)).is_some_and(|l| {
            matches!(l.phase, LoadPhase::Slabs { .. })
        })
    };
    pump(&mut f.app, "the cell waits for its slabs", phase_is_slabs);
    for _ in 0..30 {
        f.app.update();
    }
    assert!(phase_is_slabs(&mut f.app), "still waiting for slabs");
    assert!(probe.seen.lock().unwrap().iter().all(|s| s.cell != cell), "covered was asked before the cell was planned");

    // The render world's answer, by hand: every mesh and material the scene asked about.
    let (_, levels) = cooked_plate();
    let slabs = f.app.world().resource::<ChainSlabTable>();
    assert_eq!(slabs.wanted.len(), 2 * levels, "two chains' level meshes asked about");
    assert_eq!(slabs.wanted_materials.len(), 1, "one material key, one material asked about");
    answer_slabs(&mut f, |_| 0);
    settle(&mut f.app);

    let seen = the_covered_call(&probe, cell);
    assert!(seen.decided && seen.table_resolved);
    let plan = seen.plan.expect("covered is asked on a planned cell");
    assert_eq!(plan.batches.len(), 1, "both chains share the slab-keyed batch: {:?}", plan.batches);
    assert!(plan.batches[0].1.chain.is_none());
    assert!(plan.overflowed.is_empty());
    assert_eq!(seen.answer.as_deref(), Some(&f.all()), "covered is every placement the plan placed");
    assert!(f.spawned().is_empty(), "nothing the scene placed spawns");
    // Committed by the upload stage: resident, its table kept for promotion.
    let state = f.state();
    assert!(state.space.resident(cell).is_some() && state.loading.is_empty() && state.tables.contains_key(&cell));
    assert_eq!(f.app.world().resource::<Streaming>().stats().scene_covered, PLACEMENTS as u64);
}

/// Overflow, the hot path (§1.3): a batch too small for its bucket. Those
/// placements are not in `covered`, their models are gathered, and they reach
/// the spawner — including placement 3, whose other part did get a slot and is
/// written inactive.
#[test]
fn overflowed_placements_are_not_covered_and_spawn_as_entities() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(overflow_split), true, 4);
    let probe = install(&mut f, Probe::default());
    stream_in(&mut f, &probe);

    let seen = the_covered_call(&probe, f.cell);
    let plan = seen.plan.expect("planned");
    assert_eq!(plan.overflowed.len(), 1, "chain 1's batch overflowed");
    assert_eq!(plan.covered, vec![0, 1, 2]);
    // Placement 3's chain-0 part is resident but drawn by nobody: it spawns whole.
    let index_of_3 = f.parts[..3].iter().map(Vec::len).sum::<usize>();
    let write = plan.writes.iter().find(|w| w.index as usize == index_of_3).expect("placement 3's chain-0 part has a slot");
    assert!(!write.active);
    assert!(plan.writes.iter().filter(|w| w.active).all(|w| (w.index as usize) < index_of_3));

    assert_eq!(seen.answer.as_deref(), Some(&f.ids(0..3)));
    assert_eq!(f.spawned(), f.ids(3..PLACEMENTS), "exactly the overflowed placements spawn");
    assert_eq!(f.app.world().resource::<Streaming>().stats().models_loaded, 1, "their model was gathered");
    let counters = f.counters();
    assert_eq!(counters.get("scene/slots/overflow"), Some(&6.0));
    assert_eq!(counters.get("scene/overflow/cells"), Some(&1.0));
    assert_eq!(f.state().space.stats().overflow_cells, 1);
}

/// A failed chain product fails the whole cell: `covered` is `None`, every
/// placement spawns, nothing of the cell stays in the space.
#[test]
fn a_failed_chain_falls_back_the_whole_cell() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), false, 4096);
    let probe = install(&mut f, Probe::default());
    stream_in(&mut f, &probe);

    let seen = the_covered_call(&probe, f.cell);
    assert!(seen.decided && seen.plan.is_none() && seen.answer.is_none());
    assert_eq!(f.spawned(), f.all());
    assert_eq!(f.counters().get("scene/chains/failed"), Some(&1.0));
    let state = f.state();
    assert!(matches!(state.loading.get(&f.cell).map(|l| &l.phase), Some(LoadPhase::Fallback)));
    assert_eq!(state.space.stats().cells, 0);
    assert_eq!(state.space.chains().stats().live, 0, "no chain ref was taken");
    assert!(state.chains.iter().all(Option::is_none));
}

/// §4.4 row 1: no `scene.cells` product.
#[test]
fn a_missing_table_falls_back_to_entities() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Missing, true, 4096);
    let probe = install(&mut f, Probe::default());
    stream_in(&mut f, &probe);
    let seen = the_covered_call(&probe, f.cell);
    assert!(seen.table_resolved && seen.decided && seen.answer.is_none());
    assert_eq!(f.spawned(), f.all());
    let counters = f.counters();
    assert_eq!(counters.get("scene/cells/missing"), Some(&1.0));
    assert_eq!(counters.get("scene/cells/failed"), None);
    assert_eq!(f.state().space.stats().cells, 0);
}

/// §4.4 row 2: a table the loader rejects.
#[test]
fn a_corrupt_table_falls_back_to_entities() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Corrupt, true, 4096);
    let probe = install(&mut f, Probe::default());
    stream_in(&mut f, &probe);
    let seen = the_covered_call(&probe, f.cell);
    assert!(seen.decided && seen.answer.is_none());
    assert_eq!(f.spawned(), f.all());
    let counters = f.counters();
    assert_eq!(counters.get("scene/cells/failed"), Some(&1.0));
    assert_eq!(counters.get("scene/cells/missing"), None);
}

/// Drives the hook by hand, without streaming: `release` twice in each state.
#[test]
fn release_is_idempotent_when_planned_committed_or_fallen_back() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let (cell, indexed) = (f.cell, f.indexed.clone());
    let hook = SceneHook;
    let ready = |app: &mut App, cell: IVec3| SceneHook.ready(app.world(), &ws(), cell);
    let clean = |f: &Fixture| {
        let state = f.state();
        assert!(state.loading.is_empty() && state.tables.is_empty());
        assert_eq!((state.space.stats().cells, state.space.chains().stats().live), (0, 0));
        assert!(state.chains.iter().all(Option::is_none), "chain assets dropped");
    };

    // Planned, never committed.
    assert!(hook.acquire(f.app.world(), &ws(), cell).is_some());
    pump(&mut f.app, "planned", |app| ready(app, cell));
    assert_eq!(hook.covered(f.app.world(), &ws(), cell, &indexed).as_deref(), Some(&f.all()));
    assert!(f.state().chains.iter().flatten().count() == 2, "the plan stored both chains' assets");
    for _ in 0..2 {
        hook.release(f.app.world_mut(), &ws(), cell);
        clean(&f);
    }

    // Committed.
    assert!(hook.acquire(f.app.world(), &ws(), cell).is_some());
    pump(&mut f.app, "planned again", |app| ready(app, cell));
    let cx = SceneCellCx { worldspace: &ws(), cell, cell_origin: WorldPos::ORIGIN, level: sf_world::Level::Near, indexed: &f.indexed };
    assert!(hook.upload(f.app.world_mut(), &cx) > 0);
    assert!(f.state().space.resident(cell).is_some() && f.state().tables.contains_key(&cell));
    assert!(ready(&mut f.app, cell), "a committed cell is ready");
    for _ in 0..2 {
        hook.release(f.app.world_mut(), &ws(), cell);
        clean(&f);
    }

    // Fallen back: a cell with no table.
    let empty = cell + IVec3::X;
    assert!(hook.acquire(f.app.world(), &ws(), empty).is_some());
    assert!(!ready(&mut f.app, empty), "not ready before its table resolved");
    pump(&mut f.app, "fallen back", |app| ready(app, empty));
    assert!(hook.covered(f.app.world(), &ws(), empty, &indexed).is_none());
    for _ in 0..2 {
        hook.release(f.app.world_mut(), &ws(), empty);
        clean(&f);
    }
    // Released before `poll_cell_loads` even saw it.
    assert!(hook.acquire(f.app.world(), &ws(), cell).is_some());
    hook.release(f.app.world_mut(), &ws(), cell);
    f.app.update();
    clean(&f);
}

/// A chain two cells share keeps its assets while either is resident.
#[test]
fn chain_assets_outlive_the_first_cell_and_drop_with_the_last() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let cell = f.cell;
    // A second cell naming the same chain (its table is added below, by hand).
    let other = cell + IVec3::Z;
    let hook = SceneHook;
    let commit = |f: &mut Fixture, c: IVec3| {
        let cx = SceneCellCx { worldspace: &ws(), cell: c, cell_origin: WorldPos::ORIGIN, level: sf_world::Level::Near, indexed: &f.indexed };
        hook.upload(f.app.world_mut(), &cx)
    };
    assert!(hook.acquire(f.app.world(), &ws(), cell).is_some());
    pump(&mut f.app, "planned", |app| SceneHook.ready(app.world(), &ws(), cell));
    assert!(commit(&mut f, cell) > 0);
    let slot = f.state().space.chains().slot(0x10).expect("chain 0 resident");
    // The second cell's table is the first one's, re-planned under another cell id.
    let bytes = f.app.world().resource::<Assets<SceneTableAsset>>().get(&f.state().tables[&cell]).unwrap().0.clone();
    let handle = f.app.world_mut().resource_mut::<Assets<SceneTableAsset>>().add(SceneTableAsset(bytes));
    f.app.world().resource::<SceneInbox>().lock().push((ws(), other, handle));
    pump(&mut f.app, "the second cell planned", |app| SceneHook.ready(app.world(), &ws(), other));
    assert!(commit(&mut f, other) > 0);
    assert_eq!(f.state().space.chains().refs(0x10), Some(2));

    hook.release(f.app.world_mut(), &ws(), cell);
    assert!(f.state().chain(slot).is_some(), "the other cell still holds the chain");
    hook.release(f.app.world_mut(), &ws(), other);
    assert!(f.state().chain(slot).is_none(), "dropped with the last cell");
}

// ---- B3b-1: batch entities and their ranges (B3 brief §4.3 steps 5, 8, 10) ----

/// A batch's range list as plain values (`GpuBatchedMeshRange` has no `PartialEq`).
type RangeRow = (AssetId<Mesh>, u32, u32, u32, u32, Option<UntypedAssetId>);

fn rows(ranges: &[GpuBatchedMeshRange]) -> Vec<RangeRow> {
    ranges.iter().map(|r| (r.mesh.id(), r.first_index, r.index_count, r.first_slot, r.slot_count, r.material)).collect()
}

/// `SceneSpace::batch_ranges` mapped per step 8: a range is one whole chain level mesh, and its
/// material is `None` when its chain's material is `own`, the batch's.
fn expected_rows(state: &SpaceState, batch: BatchId, own: AssetId<StandardMaterial>) -> Vec<RangeRow> {
    let chain = |r: &RangeSpec| state.chain(r.chain).expect("a range names a resident chain");
    let row = |r: &RangeSpec| {
        let c = chain(r);
        let material = c.material_id().expect("a chain material").untyped();
        (c.levels[usize::from(r.level)].id(), 0, 0, r.first_slot, r.slot_count, (material != own.untyped()).then_some(material))
    };
    state.space.batch_ranges(batch).iter().map(row).collect()
}

/// Asserts the batch's entity carries exactly its `batch_ranges`, non-empty and valid for the
/// port, with `mesh` the first range's mesh, `MeshMaterial3d` that range's chain's material and
/// no `Mesh3d`; returns the entity.
fn check_batch(f: &Fixture, batch: BatchId) -> Entity {
    let state = f.state();
    let entity = state.batches[batch as usize].expect("the batch has its entity").entity;
    let world = f.app.world();
    let mesh = world.get::<GpuBatchedMesh3d>(entity).expect("a GpuBatchedMesh3d");
    let own = world.get::<MeshMaterial3d<StandardMaterial>>(entity).expect("a MeshMaterial3d").id();
    let first = state.space.batch_ranges(batch).first().map(|r| state.chain(r.chain).unwrap().material_id());
    assert_eq!(first, Some(Some(own)), "batch {batch}'s material is its first range's chain's");
    let expected = expected_rows(state, batch, own);
    assert!(!expected.is_empty(), "batch {batch} holds a block");
    assert_eq!(rows(&mesh.ranges), expected, "batch {batch}'s ranges");
    assert_eq!(mesh.mesh.id(), expected[0].0, "mesh is the first range's mesh");
    let key = state.space.batch_keys().nth(batch as usize).expect("the batch exists").1;
    assert_eq!(mesh.max_capacity, state.space.config().capacity_of_key(&key), "batch {batch}'s own class capacity");
    assert_eq!(mesh.max_capacity, state.space.batch_capacity(batch));
    assert_eq!(validate_ranges(&mesh.ranges, mesh.max_capacity), Ok(()));
    assert!(world.get::<Mesh3d>(entity).is_none());
    assert_eq!(world.get::<SceneBatch>(entity).map(|b| b.batch), Some(batch));
    entity
}

fn batch_entities(f: &mut Fixture) -> HashSet<Entity> {
    let world = f.app.world_mut();
    world.query_filtered::<Entity, With<GpuBatchedMesh3d>>().iter(world).collect()
}

fn zero_fills(f: &Fixture) -> Vec<BatchZeroFill> {
    f.app.world().resource::<SceneUploads>().zero_fills.clone()
}

fn scene_frame(f: &Fixture) -> u64 {
    f.app.world().resource::<SceneFrame>().0
}

/// Pumps `cell` (acquired, or posted to the inbox by hand) to planned and returns its plan.
fn planned(f: &mut Fixture, cell: IVec3) -> CellPlan {
    pump(&mut f.app, "planned", |app| SceneHook.ready(app.world(), &ws(), cell));
    match &f.state().loading[&cell].phase {
        LoadPhase::Planned { plan } => (**plan).clone(),
        _ => panic!("cell {cell} fell back"),
    }
}

fn commit(f: &mut Fixture, cell: IVec3) -> u64 {
    let cx = SceneCellCx { worldspace: &ws(), cell, cell_origin: WorldPos::ORIGIN, level: sf_world::Level::Near, indexed: &f.indexed };
    SceneHook.upload(f.app.world_mut(), &cx)
}

/// Acquires `cell`, plans it and commits it; returns the plan.
fn stream_cell_in(f: &mut Fixture, cell: IVec3) -> CellPlan {
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
    let plan = planned(f, cell);
    assert!(commit(f, cell) > 0);
    plan
}

/// [`stream_cell_in`] with the render world's slab answer injected by hand (needs
/// `ChainSlabTable::tracking`): the batches are then slab-keyed, the same whatever chain slot
/// the cell's chains get, as in the game. Without slabs a batch is keyed by its chain slot, and
/// a cell back within the frames in flight gets fresh slots, so fresh batches.
fn stream_cell_in_with_slabs(f: &mut Fixture, cell: IVec3) -> CellPlan {
    stream_cell_in_with_groups(f, cell, |_| 0)
}

/// The render world's answer, by hand: slabs `(1, 2)` for every mesh the scene asked about and
/// `group(material)` as the bind group of every material it asked about.
fn answer_slabs(f: &mut Fixture, group: impl Fn(&StandardMaterial) -> u32) {
    let world = f.app.world_mut();
    let wanted_materials: Vec<_> = world.resource::<ChainSlabTable>().wanted_materials.iter().copied().collect();
    let groups: Vec<(AssetId<StandardMaterial>, u32)> = {
        let materials = world.resource::<Assets<StandardMaterial>>();
        wanted_materials.into_iter().map(|id| (id, group(materials.get(id).expect("a made material")))).collect()
    };
    let mut slabs = world.resource_mut::<ChainSlabTable>();
    let wanted: Vec<_> = slabs.wanted.iter().copied().collect();
    for id in wanted {
        slabs.slabs.insert(id, MeshSlabIds { vertex_slab: 1, index_slab: 2 });
    }
    slabs.groups.extend(groups);
}

fn waits_for_slabs(f: &mut Fixture, cell: IVec3) {
    let waiting = |app: &mut App| {
        app.world().resource::<SceneSpaces>().0.get(&ws()).and_then(|s| s.loading.get(&cell)).is_some_and(|l| {
            matches!(l.phase, LoadPhase::Slabs { .. })
        })
    };
    pump(&mut f.app, "the cell waits for its slabs", waiting);
}

/// [`stream_cell_in_with_slabs`] with each material's bind group from `group`.
fn stream_cell_in_with_groups(f: &mut Fixture, cell: IVec3, group: impl Fn(&StandardMaterial) -> u32) -> CellPlan {
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
    waits_for_slabs(f, cell);
    answer_slabs(f, group);
    let plan = planned(f, cell);
    assert!(plan.batches.iter().all(|(_, key)| key.chain.is_none()), "slab-keyed batches: {:?}", plan.batches);
    assert!(commit(f, cell) > 0);
    plan
}

/// Posts a second cell whose table is `cell`'s, so it shares `cell`'s chains and batches.
fn post_twin(f: &mut Fixture, cell: IVec3, twin: IVec3) {
    let bytes = f.app.world().resource::<Assets<SceneTableAsset>>().get(&f.state().tables[&cell]).unwrap().0.clone();
    let handle = f.app.world_mut().resource_mut::<Assets<SceneTableAsset>>().add(SceneTableAsset(bytes));
    f.app.world().resource::<SceneInbox>().lock().push((ws(), twin, handle));
}

/// Steps 5 and 8: every batch a committed plan names is one entity with its ranges, and one
/// zero-fill of its whole capacity is queued per new batch.
#[test]
fn a_committed_cell_spawns_its_batches_with_their_ranges() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert!(!plan.batches.is_empty());
    let entities = batch_entities(&mut f);
    assert_eq!(entities.len(), plan.batches.len());
    let fills = zero_fills(&f);
    assert_eq!(fills.len(), plan.batches.len(), "one zero-fill per new batch");
    for &(batch, _) in &plan.batches {
        let entity = check_batch(&f, batch);
        assert!(entities.contains(&entity));
        let mine: Vec<(Entity, u32)> = fills.iter().filter(|z| z.batch == batch).map(|z| (z.entity, z.slots)).collect();
        assert_eq!(mine, vec![(entity, 4096)], "batch {batch}'s zero-fill covers its whole capacity");
    }
}

/// B5c (plan §7, which replaces §11's `promotion_is_safe_without_the_shader_fix`: no doubling can
/// happen, the risk is the opposite). Without GPU culling both batching callers skip instance
/// batches, so scene statics would go missing; the gate `extract_scene_support` reads is
/// `GpuPreprocessingSupport::is_culling_supported` (`vendor/bevy_render/src/batching/
/// gpu_preprocessing.rs:151-154`; the pipeline specialisation behind it is in
/// `vendor/bevy_pbr/src/render/gpu_preprocess.rs:2163-2190`). `Unsupported` is `--off scene`:
/// nothing acquired, no batch, `covered` is `None`, every placement spawns as an entity.
#[test]
fn an_adapter_without_gpu_culling_turns_the_scene_off() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    f.app.insert_resource(SceneSupport::Unsupported);
    let probe = install(&mut f, Probe::default());
    stream_in(&mut f, &probe);
    let cell = f.cell;
    let world = f.app.world();
    assert!(SceneHook.acquire(world, &ws(), cell).is_none(), "acquire gives nothing");
    assert!(SceneHook.ready(world, &ws(), cell), "nothing to wait for");
    assert!(SceneHook.covered(world, &ws(), cell, &f.indexed).is_none(), "covered is None");
    assert!(probe.seen.lock().unwrap().iter().all(|s| s.answer.is_none()), "streaming never saw a covered set");
    assert_eq!(world.resource::<Streaming>().stats().scene_covered, 0);
    assert!(world.resource::<SceneSpaces>().0.get(&ws()).is_none_or(|s| s.space.stats().cells == 0), "no cell committed");
    assert!(batch_entities(&mut f).is_empty(), "no batch is created");
    assert!(zero_fills(&f).is_empty());
    assert_eq!(f.spawned(), f.all(), "every placement spawns as an entity");
}

/// B5c: while the render world has not answered (`SceneSupport::Unknown`), `acquire` proceeds but
/// `ready` holds the cell; once the answer is `Supported` it plans and commits as usual.
#[test]
fn an_unanswered_adapter_holds_the_cell_until_supported() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    f.app.insert_resource(SceneSupport::Unknown);
    let cell = f.cell;
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some(), "Unknown lets acquire proceed");
    let is_planned = |app: &mut App| {
        app.world().resource::<SceneSpaces>().0.get(&ws()).and_then(|s| s.loading.get(&cell)).is_some_and(|l| {
            matches!(l.phase, LoadPhase::Planned { .. })
        })
    };
    pump(&mut f.app, "the cell planned behind the gate", is_planned);
    for _ in 0..10 {
        f.app.update();
        assert!(!SceneHook.ready(f.app.world(), &ws(), cell), "ready waits for the adapter's answer");
    }
    f.app.insert_resource(SceneSupport::Supported);
    let plan = planned(&mut f, cell);
    assert!(commit(&mut f, cell) > 0);
    assert_eq!(batch_entities(&mut f).len(), plan.batches.len());
}

/// B5c: the extract system's answer.
#[test]
fn scene_support_resolves_from_culling() {
    assert_eq!(SceneSupport::resolve(true), SceneSupport::Supported);
    assert_eq!(SceneSupport::resolve(false), SceneSupport::Unsupported);
}

/// B6 (§9): the occupancy gauges follow one cell in and out, set every frame.
#[test]
#[allow(clippy::cast_precision_loss)] // a few slots
fn scene_gauges_follow_a_cell_in_and_out() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert!(plan.overflowed.is_empty(), "the fixture fits its batches");
    f.app.update();
    let gauges = f.counters();
    let writes = plan.writes.len() as f64;
    assert_eq!(gauges.get("scene/cells/resident"), Some(&1.0));
    assert_eq!(gauges.get("scene/batches/count"), Some(&(plan.batches.len() as f64)));
    assert_eq!(gauges.get("scene/slots/batch_live"), Some(&writes));
    assert_eq!(gauges.get("scene/slots/batch_capacity"), Some(&(plan.batches.len() as f64 * 4096.0)));
    assert_eq!(
        gauges.get("scene/slots/batch_capacity_bytes"),
        Some(&(plan.batches.len() as f64 * 4096.0 * upload::SLOT_BYTES as f64))
    );
    assert_eq!(gauges.get("scene/overflow/instances"), Some(&0.0));

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    f.app.update();
    let gauges = f.counters();
    assert_eq!(gauges.get("scene/cells/resident"), Some(&0.0), "set every frame, zeros included");
    assert_eq!(gauges.get("scene/slots/batch_live"), Some(&0.0));
}

/// Fix 3: a batch's output buffer is its own class's capacity (or the single-chain one), not
/// one uniform size, and `scene/batch/<group>/*` reports it per group.
#[test]
#[allow(clippy::cast_precision_loss, clippy::float_cmp)] // a few slots: small integers, exact in f64
fn each_batch_takes_its_class_capacity_and_is_gauged_per_class() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    // Before the space exists: it is created from this config at first stream-in.
    {
        let mut config = f.app.world_mut().resource_mut::<SceneConfig>();
        config.space.class_capacity[MaterialClass::Opaque as usize] = 3000;
        config.space.chain_capacity = 2000;
    }
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert!(plan.overflowed.is_empty());
    f.app.update();
    let counters = f.counters();
    let state = f.state();
    let mut capacity: HashMap<&str, f64> = HashMap::new();
    for (batch, key) in state.space.batch_keys() {
        let expected = if key.chain.is_some() { 2000 } else { 3000 };
        assert_eq!(key.class, MaterialClass::Opaque, "the fixture's chains are opaque");
        let entity = state.batches[batch as usize].expect("the batch has its entity").entity;
        let mesh = f.app.world().get::<GpuBatchedMesh3d>(entity).expect("a GpuBatchedMesh3d");
        assert_eq!(mesh.max_capacity, expected, "batch {batch} ({key:?})");
        *capacity.entry(gauges::group_name(&key)).or_default() += f64::from(expected);
    }
    assert!(!capacity.is_empty());
    for (group, total) in &capacity {
        assert_eq!(counters.get(&format!("scene/batch/{group}/capacity")), Some(total), "{group}");
        assert_eq!(counters.get(&format!("scene/batch/{group}/overflow")), Some(&0.0), "{group}");
    }
    let live: f64 = capacity.keys().map(|g| counters[&format!("scene/batch/{g}/live")]).sum();
    assert_eq!(live, plan.writes.len() as f64, "the groups' live slots are the batches' live slots");
    // Every group is set, zeros included.
    assert_eq!(counters.get("scene/batch/masked_wind_two_sided/count"), Some(&0.0));
}

/// Step 10: a batch two cells share keeps its entity when one streams out, and its ranges
/// follow `batch_ranges`.
#[test]
fn a_shared_batch_keeps_its_entity_when_one_cell_streams_out() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let cell = f.cell;
    let twin = cell + IVec3::Z;
    let first = stream_cell_in(&mut f, cell);
    post_twin(&mut f, cell, twin);
    let second = planned(&mut f, twin);
    assert!(commit(&mut f, twin) > 0);
    let shared: Vec<BatchId> =
        second.batches.iter().map(|&(b, _)| b).filter(|b| first.batches.iter().any(|(a, _)| a == b)).collect();
    assert!(!shared.is_empty(), "the twin shares its batches");
    assert_eq!(zero_fills(&f).len(), first.batches.len(), "a shared batch is zero-filled once");
    let before: Vec<Entity> = shared.iter().map(|&b| check_batch(&f, b)).collect();
    for &b in &shared {
        assert_eq!(f.state().space.batch_ranges(b).len(), 2, "one range per cell's block");
    }

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    let after: Vec<Entity> = shared.iter().map(|&b| check_batch(&f, b)).collect();
    assert_eq!(after, before, "the shared batch keeps its entity");
    for &b in &shared {
        assert_eq!(f.state().space.batch_ranges(b).len(), 1);
    }
}

/// Step 10: an emptied batch outlives the frames in flight, refills in place before then
/// (same entity, no second zero-fill), and despawns once empty for longer.
#[test]
fn an_empty_batch_despawns_after_the_frames_in_flight() {
    // Wide enough for a re-plan to fit inside it; the space reads it at its first poll.
    const WINDOW: u32 = 300;
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    f.app.world_mut().resource_mut::<SceneConfig>().space.frames_in_flight = WINDOW;
    f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    let cell = f.cell;
    let plan = stream_cell_in_with_slabs(&mut f, cell);
    assert_eq!(f.state().space.config().frames_in_flight, WINDOW);
    let batches: Vec<(BatchId, Entity)> = plan.batches.iter().map(|&(b, _)| (b, check_batch(&f, b))).collect();
    let empty = |f: &Fixture, e: Entity| f.app.world().get::<GpuBatchedMesh3d>(e).is_some_and(|m| m.ranges.is_empty());

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    for &(b, e) in &batches {
        assert!(empty(&f, e), "batch {b} emptied");
        assert!(f.state().batches[b as usize].is_some_and(|s| s.entity == e && s.empty_since.is_some()));
    }

    // Back in before the window ends: the same entities, refilled, no second zero-fill.
    let start = scene_frame(&f);
    let again = stream_cell_in_with_slabs(&mut f, cell);
    assert!(scene_frame(&f) - start < u64::from(WINDOW), "the re-plan fit in the window");
    assert_eq!(again.batches, plan.batches);
    for &(b, e) in &batches {
        assert_eq!(check_batch(&f, b), e, "batch {b} kept its entity");
        assert_eq!(f.state().batches[b as usize].unwrap().empty_since, None);
    }
    assert_eq!(zero_fills(&f).len(), plan.batches.len());

    // Out for good: alive for the window, gone the frame after.
    SceneHook.release(f.app.world_mut(), &ws(), cell);
    for _ in 0..WINDOW {
        f.app.update();
    }
    for &(b, e) in &batches {
        assert!(empty(&f, e), "batch {b} survives the frames in flight");
    }
    f.app.update();
    for &(b, e) in &batches {
        assert!(f.app.world().get_entity(e).is_err(), "batch {b}'s entity despawned");
        assert!(f.state().batches[b as usize].is_none());
    }
    assert!(batch_entities(&mut f).is_empty());
}

// ---- B3b-2: chain materials and their bind groups (B3 brief §4.3 steps 6, 8) ----

fn red() -> ModelMaterial {
    ModelMaterial { base_color: [1.0, 0.0, 0.0, 1.0], ..ModelMaterial::default() }
}

fn is_red(m: &StandardMaterial) -> bool {
    m.base_color == Color::linear_rgba(1.0, 0.0, 0.0, 1.0)
}

/// The materials of the resident chains, in slot order.
fn chain_materials(f: &Fixture) -> Vec<AssetId<StandardMaterial>> {
    f.state().chains.iter().flatten().map(|a| a.material_id().expect("a chain material")).collect()
}

/// Step 6: one material per material key. Two chains with one key share a handle; a chain
/// with its own key and description gets its own, made from its product's material.
#[test]
fn chains_share_a_material_per_material_key() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    stream_cell_in(&mut f, cell);
    let shared = chain_materials(&f);
    assert_eq!(shared.len(), 2);
    assert_eq!(shared[0], shared[1], "one material key, one material");
    assert_eq!(f.app.world().resource::<ChainMaterials>().0.len(), 1);

    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture_with(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096, Some(red()));
    stream_cell_in(&mut f, cell);
    let own = chain_materials(&f);
    assert_eq!(own.len(), 2);
    assert_ne!(own[0], own[1], "two material keys, two materials");
    assert_eq!(f.app.world().resource::<ChainMaterials>().0.len(), 2);
    let materials = f.app.world().resource::<Assets<StandardMaterial>>();
    let reds: Vec<bool> = own.iter().map(|&m| is_red(materials.get(m).unwrap())).collect();
    assert_eq!(reds, vec![false, true], "each chain's material is its own product's");
}

/// Steps 6 and 8: the bind group splits batches. Two chains in different groups land in
/// different batches; two in one group with different materials share a batch, whose
/// `MeshMaterial3d` is its first range's chain's material and whose other chain's ranges
/// name their own material.
#[test]
fn bind_groups_split_batches_and_name_range_materials() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture_with(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096, Some(red()));
    f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    let cell = f.cell;
    let plan = stream_cell_in_with_groups(&mut f, cell, |m| u32::from(is_red(m)));
    let mut groups: Vec<u32> = plan.batches.iter().map(|(_, k)| k.bind_group).collect();
    groups.sort_unstable();
    assert_eq!(groups, vec![0, 1], "one batch per bind group: {:?}", plan.batches);
    for &(batch, _) in &plan.batches {
        let entity = check_batch(&f, batch);
        let ranges = &f.app.world().get::<GpuBatchedMesh3d>(entity).unwrap().ranges;
        assert!(ranges.iter().all(|r| r.material.is_none()), "one chain per batch, all its own material");
    }

    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture_with(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096, Some(red()));
    f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    let plan = stream_cell_in_with_groups(&mut f, cell, |_| 7);
    assert_eq!(plan.batches.len(), 1, "one bind group, one batch: {:?}", plan.batches);
    assert_eq!(plan.batches[0].1.bind_group, 7);
    let entity = check_batch(&f, plan.batches[0].0);
    let world = f.app.world();
    let own = world.get::<MeshMaterial3d<StandardMaterial>>(entity).unwrap().id();
    let other = chain_materials(&f).into_iter().find(|&m| m != own).expect("the other chain's material");
    let named: HashSet<Option<UntypedAssetId>> =
        world.get::<GpuBatchedMesh3d>(entity).unwrap().ranges.iter().map(|r| r.material).collect();
    assert_eq!(named, HashSet::from([None, Some(other.untyped())]), "the non-batch chain's ranges name its material");
}

/// The Slabs gate also waits for the material's bind group, and a binding that never arrives
/// lets the cell plan after `SLAB_WAIT_FRAMES`, its chains batching alone.
#[test]
fn a_material_binding_that_never_arrives_times_out() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
    let cell = f.cell;
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
    waits_for_slabs(&mut f, cell);
    let since = scene_frame(&f);
    // Mesh slabs only: no material is ever prepared.
    answer_slabs(&mut f, |_| 0);
    f.app.world_mut().resource_mut::<ChainSlabTable>().groups.clear();
    for _ in 0..30 {
        f.app.update();
    }
    assert!(!SceneHook.ready(f.app.world(), &ws(), cell), "the gate waits for the material binding");
    let plan = planned(&mut f, cell);
    assert!(scene_frame(&f) - since >= SLAB_WAIT_FRAMES, "planned only after the wait");
    assert!(!plan.batches.is_empty());
    assert!(plan.batches.iter().all(|(_, k)| k.chain.is_some()), "unresolved chains batch alone: {:?}", plan.batches);
    assert!(commit(&mut f, cell) > 0);
}

/// B4a test 3: a committed cell queues exactly one resolved write per `plan.writes`, in plan
/// order, with its batch's entity, its slot, its `active` (an overflowed placement's other
/// parts written inactive) and its table row placed by the floating origin.
#[test]
fn a_committed_cell_queues_one_resolved_write_per_plan_write() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(overflow_split), true, 4);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert!(!plan.overflowed.is_empty(), "the fixture overflows chain 1's batch");
    let world = f.app.world();
    let writes = &world.resource::<SceneUploads>().writes;
    assert_eq!(writes.len(), plan.writes.len());
    assert!(plan.writes.iter().any(|w| !w.active), "placement 3's chain-0 part is written inactive");

    let state = f.state();
    let view = world.resource::<Assets<SceneTableAsset>>().get(&state.tables[&cell]).unwrap().view().unwrap();
    let frame = world.get_resource::<sf_world_bevy::WorldOrigin>().map_or(IVec3::ZERO, |o| o.cell);
    let offset = sf_scene::cell_offset(view.origin_cell(), WorldPos::new(frame, Vec3::ZERO));
    for (got, want) in writes.iter().zip(&plan.writes) {
        let entity = state.batches[want.batch as usize].expect("a committed batch has its entity").entity;
        assert_eq!(got.entity, MainEntity::from(entity));
        assert_eq!((got.batch, got.slot, got.active()), (want.batch, want.slot, want.active));
        let row = view.instance(want.index as usize).unwrap().transform;
        assert_eq!(
            instance_bytes(&[got.instance]),
            instance_bytes(&[gpu_instance(row, offset, want.active)])
        );
    }
}

/// O1: an origin shift re-queues one write per resident slot in the new frame, a promoted
/// placement still inactive; no shift, no write.
#[test]
fn an_origin_shift_rewrites_every_resident_slot_in_the_new_frame() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert_eq!(promote(&mut f, cell, 1).len(), 2, "placement 1 draws both chains");
    f.app.world_mut().resource_mut::<SceneUploads>().writes.clear();

    let old = f.app.world().get_resource::<sf_world_bevy::WorldOrigin>().map_or(IVec3::ZERO, |o| o.cell);
    let frame = old + IVec3::X;
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), frame));
    f.app.update();

    let world = f.app.world();
    let writes = &world.resource::<SceneUploads>().writes;
    assert_eq!(writes.len(), plan.writes.len(), "one write per resident slot");
    let state = f.state();
    let view = world.resource::<Assets<SceneTableAsset>>().get(&state.tables[&cell]).unwrap().view().unwrap();
    let offset = sf_scene::cell_offset(view.origin_cell(), WorldPos::new(frame, Vec3::ZERO));
    for (got, want) in writes.iter().zip(&plan.writes) {
        let row = view.instance(want.index as usize).unwrap();
        let active = want.active && row.placement != 1;
        let entity = state.batches[want.batch as usize].expect("a committed batch has its entity").entity;
        assert_eq!(got.entity, MainEntity::from(entity));
        assert_eq!((got.batch, got.slot, got.active()), (want.batch, want.slot, active));
        assert_eq!(instance_bytes(&[got.instance]), instance_bytes(&[gpu_instance(row.transform, offset, active)]));
    }
    assert_eq!(writes.iter().filter(|w| !w.active()).count(), 2, "the promoted placement stays inactive");
    let counters = f.counters();
    assert_eq!(counters.get("scene/shift/rewrites"), Some(&1.0));
    #[allow(clippy::cast_precision_loss)] // a few slots
    let bytes = (plan.writes.len() as u64 * upload::SLOT_BYTES) as f64;
    assert_eq!(counters.get("scene/shift/rewrite_bytes"), Some(&bytes));

    // No change, no rewrite.
    f.app.world_mut().resource_mut::<SceneUploads>().writes.clear();
    f.app.update();
    assert!(f.app.world().resource::<SceneUploads>().writes.is_empty());
}

// -- promotion (B5a) ------------------------------------------------------------

/// Sends one [`PromotePlacement`] (by its global id) for local placement `k` of `cell`, runs a
/// frame and returns the writes it queued.
fn promote(f: &mut Fixture, cell: IVec3, k: usize) -> Vec<BatchSlotWrite> {
    let before = f.app.world().resource::<SceneUploads>().writes.len();
    f.app.world_mut().write_message(PromotePlacement { worldspace: ws(), cell, placement: f.indexed[k] });
    f.app.update();
    f.app.world().resource::<SceneUploads>().writes[before..].to_vec()
}

/// B5a test 1: promoting a two-part placement queues exactly its two slots, inactive, at the
/// batch, slot and table row its commit wrote them to.
#[test]
fn promotion_queues_one_inactive_write_per_slot() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert_eq!(f.parts[1].len(), 2, "placement 1 draws both chains");
    assert!(plan.covered.contains(&1));
    let got = promote(&mut f, cell, 1);

    let world = f.app.world();
    let state = f.state();
    let view = world.resource::<Assets<SceneTableAsset>>().get(&state.tables[&cell]).unwrap().view().unwrap();
    let want: Vec<_> = plan.writes.iter().filter(|w| view.instance(w.index as usize).unwrap().placement == 1).collect();
    assert_eq!(want.len(), 2);
    assert_eq!(got.len(), 2);
    let frame = world.get_resource::<sf_world_bevy::WorldOrigin>().map_or(IVec3::ZERO, |o| o.cell);
    let offset = sf_scene::cell_offset(view.origin_cell(), WorldPos::new(frame, Vec3::ZERO));
    for (got, want) in got.iter().zip(&want) {
        let entity = state.batches[want.batch as usize].expect("a committed batch has its entity").entity;
        assert_eq!(got.entity, MainEntity::from(entity));
        assert_eq!((got.batch, got.slot), (want.batch, want.slot));
        assert!(!got.active());
        let row = view.instance(want.index as usize).unwrap().transform;
        assert_eq!(instance_bytes(&[got.instance]), instance_bytes(&[gpu_instance(row, offset, false)]));
    }
}

/// B5a test 2: a second promotion of the same placement queues nothing, and neither does one
/// of a placement the scene does not draw (one of its parts overflowed: the entity path has it).
#[test]
fn promotion_is_one_way_and_skips_what_the_scene_does_not_draw() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(overflow_split), true, 4);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert!(!plan.overflowed.is_empty(), "the fixture overflows chain 1's batch");
    assert!(plan.covered.contains(&0), "placement 0 draws chain 0 only, which fits");
    let uncovered: Vec<usize> =
        (0..PLACEMENTS).filter(|&k| !plan.covered.contains(&u32::try_from(k).unwrap())).collect();
    assert!(uncovered.contains(&3), "placement 3's chain-1 part overflowed: {uncovered:?}");

    assert_eq!(promote(&mut f, cell, 0).len(), 1);
    assert!(promote(&mut f, cell, 0).is_empty(), "promotion is one way");
    for k in uncovered {
        assert!(promote(&mut f, cell, k).is_empty(), "placement {k} is not the scene's");
    }
    assert_eq!(f.state().space.stats().promoted, 1);
}

/// B5a test 3: a promotion of a cell still loading, of a released cell or of an unknown
/// worldspace is dropped; promotions die with the cell, so streaming it back in draws all of it.
#[test]
fn promotion_of_a_cell_that_is_not_resident_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
    let _ = planned(&mut f, cell);
    assert!(promote(&mut f, cell, 1).is_empty(), "planned, not committed");
    assert!(commit(&mut f, cell) > 0);
    assert_eq!(promote(&mut f, cell, 1).len(), 2);
    assert_eq!(f.state().space.stats().promoted, 1);

    let before = f.app.world().resource::<SceneUploads>().writes.len();
    let elsewhere = PromotePlacement { worldspace: WorldspaceId::new("nowhere"), cell, placement: f.indexed[3] };
    f.app.world_mut().write_message(elsewhere);
    f.app.update();
    assert_eq!(f.app.world().resource::<SceneUploads>().writes.len(), before, "unknown worldspace");

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    assert!(!f.state().covered.contains_key(&cell), "the map drops with the table");
    assert!(promote(&mut f, cell, 1).is_empty(), "released");
    assert!(promote(&mut f, cell, 3).is_empty(), "released");
    assert_eq!(f.state().space.stats().promoted, 0);

    let before = f.app.world().resource::<SceneUploads>().writes.len();
    let plan = stream_cell_in(&mut f, cell);
    let writes = &f.app.world().resource::<SceneUploads>().writes[before..];
    assert_eq!(writes.len(), plan.writes.len());
    assert!(plan.writes.iter().all(|w| w.active), "{:?}", plan.writes);
    assert!(writes.iter().all(BatchSlotWrite::active), "placement 1 is drawn again");
}

/// B5a-2: streaming tells the hook after it spawned a promoted placement's entity, and the hook
/// queues exactly one [`PromotePlacement`] for it, with its fields as given.
#[test]
fn promoted_queues_one_promote_placement() {
    let mut world = World::new();
    world.init_resource::<Messages<PromotePlacement>>();
    let (cell, placement) = (IVec3::new(2, 0, -1), PlacementId::from_key("test", b"promoted"));
    SceneHook.promoted(&mut world, &ws(), cell, placement);
    let messages = world.resource::<Messages<PromotePlacement>>();
    let got: Vec<&PromotePlacement> = messages.iter_current_update_messages().collect();
    assert_eq!(got, [&PromotePlacement { worldspace: ws(), cell, placement }]);
}

/// `IN38c1`: the hook's `hidden` (`Streaming::hide_covered`) queues one inactive write per
/// instance of the placement, like a promotion, and a second one nothing; once the overlay removed
/// it, releasing and streaming the cell back in leaves it undrawn.
#[test]
fn hidden_queues_one_inactive_write_per_instance_and_stays_gone_with_the_overlay() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    assert_eq!(f.parts[1].len(), 2, "placement 1 draws both chains");
    assert!(plan.covered.contains(&1));

    let hide = |f: &mut Fixture| {
        let before = f.app.world().resource::<SceneUploads>().writes.len();
        let id = f.indexed[1];
        SceneHook.hidden(f.app.world_mut(), &ws(), cell, id);
        f.app.update();
        f.app.world().resource::<SceneUploads>().writes[before..].to_vec()
    };
    let got = hide(&mut f);
    assert_eq!(got.len(), 2, "one write per instance");
    assert!(got.iter().all(|w| !w.active()));
    assert!(hide(&mut f).is_empty(), "already hidden");

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    let (plan, covered) = stream_in_overlaid(&mut f, |f, o| o.remove(f.indexed[1]));
    assert_left_the_scene(&f, &plan, &covered, 1);
}

// -- the world overlay (B5b) ----------------------------------------------------------

/// Edits the fixture's `WorldOverlay` with `edit` before the cell streams in, then acquires,
/// plans and commits it; returns the plan and `covered`'s answer (asked between the two).
fn stream_in_overlaid(
    f: &mut Fixture,
    edit: impl FnOnce(&Fixture, &mut sf_world::PlacementOverlay),
) -> (CellPlan, Arc<HashSet<PlacementId>>) {
    let cell = f.cell;
    let overlay = f.app.world().resource::<sf_world_bevy::WorldOverlay>().0.clone();
    edit(f, &mut overlay.write().unwrap());
    assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
    let plan = planned(f, cell);
    let covered = SceneHook.covered(f.app.world(), &ws(), cell, &f.indexed).expect("a planned cell answers");
    assert!(commit(f, cell) > 0);
    (plan, covered)
}

/// The three facts of a placement `k` the overlay touched: `covered` lacks it, each of its
/// planned slots ends inactive in `SceneUploads::writes`, and the promotion map lacks it.
fn assert_left_the_scene(f: &Fixture, plan: &CellPlan, covered: &HashSet<PlacementId>, k: usize) {
    let local = u32::try_from(k).unwrap();
    assert!(plan.covered.contains(&local), "the plan itself draws placement {k}");
    let id = f.indexed[k];
    assert!(!covered.contains(&id), "covered keeps placement {k}");
    assert_eq!(covered.len(), plan.covered.len() - 1, "covered drops only placement {k}");

    let world = f.app.world();
    let state = f.state();
    let view = world.resource::<Assets<SceneTableAsset>>().get(&state.tables[&f.cell]).unwrap().view().unwrap();
    let slots: Vec<(BatchId, u32)> = plan
        .writes
        .iter()
        .filter(|w| view.instance(w.index as usize).unwrap().placement == local)
        .map(|w| (w.batch, w.slot))
        .collect();
    assert_eq!(slots.len(), f.parts[k].len(), "one planned slot per part of placement {k}");
    let writes = &world.resource::<SceneUploads>().writes;
    for (batch, slot) in slots {
        let last = writes.iter().rev().find(|w| (w.batch, w.slot) == (batch, slot)).expect("the slot is written");
        assert!(!last.active(), "slot {slot} of batch {batch:?} ends active");
    }
    let map = &state.covered[&f.cell];
    assert!(!map.contains_key(&id), "the promotion map keeps placement {k}");
    assert_eq!(map.len(), plan.covered.len() - 1);
}

/// B5b test 1: a placement the overlay removed before its cell streamed in is not drawn.
#[test]
fn an_overlay_removed_placement_is_not_drawn() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    assert_eq!(f.parts[1].len(), 2, "placement 1 draws both chains");
    let (plan, covered) = stream_in_overlaid(&mut f, |f, o| o.remove(f.indexed[1]));
    assert_left_the_scene(&f, &plan, &covered, 1);
}

/// B5b test 2: a placement the overlay moved leaves the scene (streaming spawns it where it went).
#[test]
fn an_overlay_moved_placement_leaves_the_scene() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let (plan, covered) = stream_in_overlaid(&mut f, |f, o| {
        let mut moved = placements().into_iter().find(|p| p.id == f.indexed[2]).expect("placement 2 is the fixture's");
        moved.transform = WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(5.5, 1.0, 6.0)));
        o.place(ws(), moved);
    });
    assert_left_the_scene(&f, &plan, &covered, 2);
}

/// B5b test 3: an overlay that touches only another cell's placement (and stores a state blob
/// for one of this cell's, which moves nothing) leaves the cell drawn exactly as planned.
#[test]
fn an_untouched_cell_is_drawn_as_planned() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let (plan, covered) = stream_in_overlaid(&mut f, |f, o| {
        o.remove(PlacementId::from_key("test", b"another cell's"));
        o.set_state(f.indexed[4], "lock", vec![1]);
    });
    let full: HashSet<PlacementId> = plan.covered.iter().map(|&l| f.indexed[l as usize]).collect();
    assert_eq!(*covered, full);
    assert_eq!(f.state().covered[&f.cell].keys().copied().collect::<HashSet<_>>(), full);
    let got: Vec<_> =
        f.app.world().resource::<SceneUploads>().writes.iter().map(|w| (w.batch, w.slot, w.active())).collect();
    let want: Vec<_> = plan.writes.iter().map(|w| (w.batch, w.slot, w.active)).collect();
    assert_eq!(got, want);
}

/// `stream.scene.load_ms`: once a frame's budget is spent the remaining cells wait for the next
/// frame, and at least one cell steps even with no budget at all, so loads always progress.
#[test]
fn cell_loads_are_time_sliced_but_always_progress() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let cell = f.cell;
    stream_cell_in(&mut f, cell);
    f.app.update();
    let _ = f.counters();

    f.app.insert_resource(SceneLoadBudget { ms: 0.0 });
    let (a, b) = (cell + IVec3::Z, cell + IVec3::X);
    post_twin(&mut f, cell, a);
    post_twin(&mut f, cell, b);
    f.app.update();
    assert_eq!(f.counters().get("scene/load/deferred"), Some(&1.0), "one of the two new cells waited");

    // Still one cell a frame with no budget: both get planned.
    planned(&mut f, a);
    planned(&mut f, b);
}

// -- world bugs repro (D2: interior rocks, RO-18 floating statics) -------------------------------

/// The second worldspace of the world-bugs repros (an interior entered from the fixture's).
fn interior() -> WorldspaceId {
    WorldspaceId::new("interior")
}

/// The offset a frame at grid cell `frame` puts a cell origin at `origin_cell` (cell edge
/// [`sf_coords::GRID_CELL`]), computed here, independently of `sf_scene::cell_offset`.
fn frame_offset(origin_cell: IVec3, frame: IVec3) -> Vec3 {
    (origin_cell - frame).as_vec3() * sf_coords::GRID_CELL
}

/// The offset `write` placed its table `row` at: its translation lanes minus the row's.
fn placed_offset(row: [[f32; 4]; 3], write: &BatchSlotWrite) -> Vec3 {
    let t = write.instance.world_from_local;
    Vec3::new(t[0].w - row[0][3], t[1].w - row[1][3], t[2].w - row[2][3])
}

/// The cell's table origin and its plan writes' table rows, in plan order.
fn plan_rows(f: &Fixture, cell: IVec3, plan: &CellPlan) -> (IVec3, Vec<[[f32; 4]; 3]>) {
    let world = f.app.world();
    let view = world.resource::<Assets<SceneTableAsset>>().get(&f.state().tables[&cell]).unwrap().view().unwrap();
    let rows = plan.writes.iter().map(|w| view.instance(w.index as usize).unwrap().transform).collect();
    (view.origin_cell(), rows)
}

/// Whether `entity` is hidden by its own `Visibility` or an inherited one (walks `ChildOf`).
fn hidden_in_tree(world: &World, mut entity: Entity) -> bool {
    loop {
        match world.get::<Visibility>(entity) {
            Some(Visibility::Hidden) => return true,
            Some(Visibility::Visible) => return false,
            _ => {}
        }
        match world.get::<ChildOf>(entity) {
            Some(parent) => entity = parent.parent(),
            None => return false,
        }
    }
}

/// Every `SceneBatch` entity with its render layers (none = layer 0, Bevy's default).
fn scene_batch_layers(f: &mut Fixture) -> Vec<(Entity, bevy::camera::visibility::RenderLayers)> {
    let world = f.app.world_mut();
    world
        .query_filtered::<(Entity, Option<&bevy::camera::visibility::RenderLayers>), With<SceneBatch>>()
        .iter(world)
        .map(|(e, l)| (e, l.cloned().unwrap_or_default()))
        .collect()
}

/// Repro of bug 1 (entering an interior from Balmora showed exterior rocks that are not part of
/// the cell): a committed cell of worldspace A, then worldspace B active. No `SceneBatch` of A
/// may be drawn by a camera of B (`WorldspaceLayers::camera`, `[0, B's world layer]`): it carries
/// layers that miss B's camera, or it is hidden. Batches are spawned with no `RenderLayers`
/// (layer 0, which every worldspace camera sees) and not under A's root, so parking A
/// (`sf_world_bevy` `park.rs`, which hides the root's subtree) cannot hide them. Back in A, they
/// are drawn by A's camera again.
#[test]
fn scene_batches_of_a_worldspace_are_not_drawn_by_another_worldspaces_camera() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    stream_cell_in(&mut f, cell);
    let a = WorldspaceRegistry::ensure_in(f.app.world_mut(), &ws());
    let b = WorldspaceRegistry::ensure_in(f.app.world_mut(), &interior());
    f.app.insert_resource(ActiveWorldspace(Some(interior())));
    for _ in 0..3 {
        f.app.update();
    }
    let batches = scene_batch_layers(&mut f);
    assert!(!batches.is_empty(), "A's committed cell has batch entities");
    let b_camera = b.layers().camera();
    for (entity, layers) in &batches {
        let hidden = hidden_in_tree(f.app.world(), *entity);
        assert!(
            hidden || !layers.intersects(&b_camera),
            "A's batch {entity:?} is drawn by B's camera: layers {layers:?}, B's camera {b_camera:?}, A's content {:?}, hidden {hidden}",
            a.layers().content()
        );
    }

    f.app.insert_resource(ActiveWorldspace(Some(ws())));
    for _ in 0..3 {
        f.app.update();
    }
    let a_camera = a.layers().camera();
    for (entity, layers) in scene_batch_layers(&mut f) {
        assert!(
            !hidden_in_tree(f.app.world(), entity) && layers.intersects(&a_camera),
            "back in A, A's batch {entity:?} is drawn by A's camera: layers {layers:?}, A's camera {a_camera:?}"
        );
    }
}

/// A worldspace's layer pair belongs to its loaded lifetime (`WorldspaceRegistry::release_in`
/// recycles it, first in first out): once A's root is released, A's batches keep no layer, so the
/// worldspace that takes A's pair next never draws them; loaded again, A's batches take A's new pair.
#[test]
fn scene_batches_never_keep_a_released_worldspaces_layers() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    stream_cell_in(&mut f, cell);
    let a = WorldspaceRegistry::ensure_in(f.app.world_mut(), &ws());
    f.app.update();
    let batches = scene_batch_layers(&mut f);
    assert!(!batches.is_empty(), "A's committed cell has batch entities");
    assert!(batches.iter().all(|(_, l)| *l == a.layers().content()), "A's batches on A's content layer: {batches:?}");

    assert!(WorldspaceRegistry::release_in(f.app.world_mut(), &ws()), "nothing under A's root");
    let b = WorldspaceRegistry::ensure_in(f.app.world_mut(), &interior());
    assert_eq!(b.layer, a.layer, "B takes A's released pair");
    f.app.insert_resource(ActiveWorldspace(Some(interior())));
    f.app.update();
    let now = f.app.world().resource::<WorldspaceRegistry>().get(&ws());
    for (entity, layers) in scene_batch_layers(&mut f) {
        assert!(!layers.intersects(&b.layers().camera()), "A's batch {entity:?} drawn by B on A's old pair: {layers:?}");
        let want = now.map_or_else(bevy::camera::visibility::RenderLayers::none, |e| e.layers().content());
        assert_eq!(layers, want, "A's batch {entity:?} follows A's current registry entry {now:?}");
    }

    let a2 = WorldspaceRegistry::ensure_in(f.app.world_mut(), &ws());
    f.app.insert_resource(ActiveWorldspace(Some(ws())));
    f.app.update();
    for (entity, layers) in scene_batch_layers(&mut f) {
        assert_eq!(layers, a2.layers().content(), "A's batch {entity:?} on A's new pair");
    }
}

/// Repro of the cross-worldspace half of RO-18 (and bug 1's placement): worldspace B is active
/// with its origin at `(5, 0, -3)`, A was last drawn with its origin at `(2, 1, 0)`, and a cell of
/// A streams in. Its slots must be in A's frame, `(2, 1, 0)`.
///
/// Why A's last origin: `shift.rs` `rewrite_on_origin_shift` rewrites only the origin's
/// worldspace, so while B is active A's resident slots stay in the frame A was last drawn in;
/// on the return to A, `update_world_origin` sets `WorldOrigin` to A's grid's floating-origin
/// cell and the worldspace change rewrites every resident slot of A there. A's last origin is
/// that cell while A's camera has not moved, so it is the one frame every slot of A agrees on.
/// Commit writes in the space's own frame (`SpaceState::frame`, started from
/// `shift::OriginFrames`), never in the active worldspace's origin. The returns to A are checked
/// too: at A's last origin nothing is rewritten, one cell further every slot is.
#[test]
fn a_cell_planned_while_another_worldspace_is_active_is_written_in_its_own_frame() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let (a_frame, b_frame) = (IVec3::new(2, 1, 0), IVec3::new(5, 0, -3));
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), a_frame));
    f.app.update();
    f.app.insert_resource(ActiveWorldspace(Some(interior())));
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(interior()), b_frame));
    f.app.update();
    f.app.world_mut().resource_mut::<SceneUploads>().writes.clear();

    let plan = stream_cell_in(&mut f, cell);
    let (origin_cell, rows) = plan_rows(&f, cell, &plan);
    let writes = f.app.world().resource::<SceneUploads>().writes.clone();
    assert_eq!(writes.len(), plan.writes.len(), "one write per plan write");
    let want = frame_offset(origin_cell, a_frame);
    for (got, row) in writes.iter().zip(&rows) {
        assert_eq!(
            placed_offset(*row, got),
            want,
            "slot {} of batch {} placed in B's frame {b_frame} (offset {}), not A's {a_frame} (offset {want})",
            got.slot,
            got.batch,
            frame_offset(origin_cell, b_frame)
        );
    }

    // Back in A at its last origin: the slots are already in A's frame, nothing is rewritten.
    f.app.world_mut().resource_mut::<SceneUploads>().writes.clear();
    f.app.insert_resource(ActiveWorldspace(Some(ws())));
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), a_frame));
    f.app.update();
    assert!(f.app.world().resource::<SceneUploads>().writes.is_empty(), "the return to A's own frame rewrites nothing");

    // To B and back to A one cell further: the worldspace change rewrites every slot there.
    f.app.insert_resource(ActiveWorldspace(Some(interior())));
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(interior()), b_frame));
    f.app.update();
    assert!(f.app.world().resource::<SceneUploads>().writes.is_empty(), "B's origin leaves A's slots alone");
    let moved = a_frame + IVec3::X;
    f.app.insert_resource(ActiveWorldspace(Some(ws())));
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), moved));
    f.app.update();
    let writes = f.app.world().resource::<SceneUploads>().writes.clone();
    assert_eq!(writes.len(), plan.writes.len(), "the return to A elsewhere rewrites every slot");
    let want = frame_offset(origin_cell, moved);
    for (got, row) in writes.iter().zip(&rows) {
        assert_eq!(placed_offset(*row, got), want, "the return to A rewrites slot {} in A's new frame {moved}", got.slot);
    }
}

/// [`an_origin_shift_rewrites_every_resident_slot_in_the_new_frame`] for each origin of
/// `frames` in turn: every shift re-queues one write per resident slot, at the offset of the
/// new frame (checked against [`frame_offset`], not `cell_offset`).
fn shifts_rewrite_in_the_new_frame(frames: &[IVec3]) {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    f.app.update();
    let (origin_cell, rows) = plan_rows(&f, cell, &plan);
    for &frame in frames {
        f.app.world_mut().resource_mut::<SceneUploads>().writes.clear();
        f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), frame));
        f.app.update();
        let world = f.app.world();
        let writes = &world.resource::<SceneUploads>().writes;
        assert_eq!(writes.len(), plan.writes.len(), "shift to {frame}: one write per resident slot");
        let want = frame_offset(origin_cell, frame);
        for ((got, row), w) in writes.iter().zip(&rows).zip(&plan.writes) {
            assert_eq!((got.batch, got.slot, got.active()), (w.batch, w.slot, w.active));
            assert_eq!(placed_offset(*row, got), want, "shift to {frame}: slot {} of batch {}", got.slot, got.batch);
        }
    }
}

/// RO-18 (statics float after an upward origin shift): a Y shift up, a second one up and one
/// below the start rewrite every resident slot in the new frame.
#[test]
fn a_vertical_origin_shift_rewrites_every_resident_slot_in_the_new_frame() {
    shifts_rewrite_in_the_new_frame(&[IVec3::Y, IVec3::new(0, 2, 0), IVec3::NEG_Y]);
}

/// RO-18: the same with X, Y and Z shifted at once, then back to the start.
#[test]
fn a_diagonal_origin_shift_rewrites_every_resident_slot_in_the_new_frame() {
    shifts_rewrite_in_the_new_frame(&[IVec3::new(1, 1, 1), IVec3::new(-1, 2, 3), IVec3::ZERO]);
}

/// The write the publish leaves in each slot of `cell` (the last one per `(entity, slot)` after
/// `upload::slot_runs`' stable sort) is at `frame`'s offset.
fn assert_last_writes_in(f: &Fixture, cell: IVec3, plan: &CellPlan, frame: IVec3, case: &str) {
    let mut writes = f.app.world().resource::<SceneUploads>().writes.clone();
    slot_runs(&mut writes);
    let last: BTreeMap<(MainEntity, u32), BatchSlotWrite> = writes.into_iter().map(|w| ((w.entity, w.slot), w)).collect();
    let (origin_cell, rows) = plan_rows(f, cell, plan);
    let want = frame_offset(origin_cell, frame);
    let state = f.state();
    for (w, row) in plan.writes.iter().zip(&rows) {
        let entity = MainEntity::from(state.batches[w.batch as usize].expect("a committed batch has its entity").entity);
        let got = last.get(&(entity, w.slot)).unwrap_or_else(|| panic!("{case}: slot {} of batch {} never written", w.slot, w.batch));
        assert_eq!(placed_offset(*row, got), want, "{case}: slot {} of batch {} ends in frame {frame}'s offset", w.slot, w.batch);
    }
}

/// RO-18 ordering: an upward origin shift (Y) while a cell is mid-load (waiting on its slabs),
/// then one in the same frame as a commit, in both orders (commit then the `PostUpdate` rewrite,
/// as the streaming `Update` and `rewrite_on_origin_shift` run; and the origin moved before the
/// commit reads it). The last queued write of every slot of the cell is in the new frame.
#[test]
fn a_shift_between_plan_and_commit_leaves_no_slot_in_the_old_frame() {
    let up = IVec3::Y;
    // Mid-load: the shift lands while the cell waits on its slabs.
    {
        let dir = tempfile::tempdir().unwrap();
        let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
        f.app.world_mut().resource_mut::<ChainSlabTable>().tracking = true;
        let cell = f.cell;
        assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
        waits_for_slabs(&mut f, cell);
        f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), up));
        f.app.update();
        answer_slabs(&mut f, |_| 0);
        let plan = planned(&mut f, cell);
        assert!(commit(&mut f, cell) > 0);
        f.app.update();
        assert_last_writes_in(&f, cell, &plan, up, "shift while waiting on slabs");
    }
    // Same frame, commit first: written in the old frame, then the shift's rewrite.
    {
        let dir = tempfile::tempdir().unwrap();
        let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
        let cell = f.cell;
        f.app.update();
        assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
        let plan = planned(&mut f, cell);
        assert!(commit(&mut f, cell) > 0);
        f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), up));
        f.app.update();
        assert_last_writes_in(&f, cell, &plan, up, "commit, then the shift in the same frame");
    }
    // Same frame, shift first: the commit reads the moved origin, then the rewrite runs.
    {
        let dir = tempfile::tempdir().unwrap();
        let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
        let cell = f.cell;
        f.app.update();
        assert!(SceneHook.acquire(f.app.world(), &ws(), cell).is_some());
        let plan = planned(&mut f, cell);
        f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), up));
        assert!(commit(&mut f, cell) > 0);
        f.app.update();
        assert_last_writes_in(&f, cell, &plan, up, "the shift, then the commit in the same frame");
    }
}

// -- release (RO-18, F2) ----------------------------------------------------------

/// What the publish leaves in every slot ever written: the last queued write per
/// `(entity, slot)`, after `upload::slot_runs`' stable sort (a later write to one slot wins).
fn last_writes(f: &Fixture) -> BTreeMap<(MainEntity, u32), BatchSlotWrite> {
    let mut writes = f.app.world().resource::<SceneUploads>().writes.clone();
    slot_runs(&mut writes);
    writes.into_iter().map(|w| ((w.entity, w.slot), w)).collect()
}

/// Every `(batch entity, slot)` of the blocks `blocks` names (a plan's or a residency's).
fn block_slots(f: &Fixture, blocks: &[(BatchId, sf_scene::SlotRange, sf_scene::ChainSlot, u8)]) -> HashSet<(MainEntity, u32)> {
    let state = f.state();
    blocks
        .iter()
        .flat_map(|&(batch, range, ..)| {
            let entity = MainEntity::from(state.batches[batch as usize].expect("a live batch has its entity").entity);
            (range.start..range.end()).map(move |slot| (entity, slot))
        })
        .collect()
}

/// Every `(batch entity, slot)` a resident cell holds.
fn resident_slots(f: &Fixture) -> HashSet<(MainEntity, u32)> {
    let blocks: Vec<_> = f.state().space.cells().flat_map(|(_, r)| r.blocks.clone()).collect();
    block_slots(f, &blocks)
}

/// F2 test 1: two cells share a batch; streaming one out leaves the last queued write of every
/// slot it held inactive, and queues nothing for the other cell's slots.
#[test]
fn a_released_cells_slots_are_written_inactive() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let twin = cell + IVec3::Z;
    let first = stream_cell_in(&mut f, cell);
    post_twin(&mut f, cell, twin);
    let second = planned(&mut f, twin);
    assert!(commit(&mut f, twin) > 0);
    assert!(second.batches.iter().any(|b| first.batches.contains(b)), "the twin shares a batch");
    let released = block_slots(&f, &first.blocks);
    let kept = block_slots(&f, &second.blocks);
    assert!(released.is_disjoint(&kept));
    let before = last_writes(&f);
    let queued = f.app.world().resource::<SceneUploads>().writes.len();

    SceneHook.release(f.app.world_mut(), &ws(), cell);

    let new = &f.app.world().resource::<SceneUploads>().writes[queued..];
    assert!(new.iter().all(|w| !kept.contains(&(w.entity, w.slot))), "the twin's slots are not touched");
    let last = last_writes(&f);
    let stale: Vec<_> = released.iter().filter(|k| last.get(k).is_none_or(BatchSlotWrite::active)).collect();
    assert!(stale.is_empty(), "{} of {} released slots still draw: {stale:?}", stale.len(), released.len());
    for k in &kept {
        assert_eq!(last.get(k).map(BatchSlotWrite::active), before.get(k).map(BatchSlotWrite::active), "slot {k:?} kept");
    }
}

/// F2 test 2 (RO-18): a cell streams out, then the origin moves up one cell: every slot whose
/// last queued write draws belongs to a resident cell, so nothing is left in the old frame.
#[test]
fn no_stale_slot_draws_after_a_release_and_a_vertical_shift() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let twin = cell + IVec3::Z;
    stream_cell_in(&mut f, cell);
    post_twin(&mut f, cell, twin);
    planned(&mut f, twin);
    assert!(commit(&mut f, twin) > 0);
    f.app.update();

    SceneHook.release(f.app.world_mut(), &ws(), cell);
    let old = f.app.world().get_resource::<sf_world_bevy::WorldOrigin>().map_or(IVec3::ZERO, |o| o.cell);
    f.app.insert_resource(sf_world_bevy::WorldOrigin::new(Some(ws()), old + IVec3::Y));
    f.app.update();
    assert_eq!(f.counters().get("scene/shift/rewrites"), Some(&1.0), "the shift rewrote the resident cell");

    let resident = resident_slots(&f);
    assert!(!resident.is_empty());
    let stale: Vec<_> = last_writes(&f).into_iter().filter(|(k, w)| w.active() && !resident.contains(k)).map(|(k, _)| k).collect();
    assert!(stale.is_empty(), "{} active slots belong to no resident cell: {stale:?}", stale.len());
}

/// F2 test 3: the last cell of a batch streams out (its range list goes empty, so the batch
/// would draw its whole capacity with its own mesh): no slot of it is left active.
#[test]
fn an_emptied_batch_keeps_no_active_slot() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a_and_b), true, 4096);
    let cell = f.cell;
    let plan = stream_cell_in(&mut f, cell);
    let held = block_slots(&f, &plan.blocks);

    SceneHook.release(f.app.world_mut(), &ws(), cell);

    for &(b, _) in &plan.batches {
        assert!(f.state().space.batch_ranges(b).is_empty(), "batch {b} emptied");
    }
    let last = last_writes(&f);
    assert!(held.iter().all(|k| last.contains_key(k)));
    let active: Vec<_> = last.iter().filter(|(_, w)| w.active()).map(|(k, _)| *k).collect();
    assert!(active.is_empty(), "{} slots of emptied batches still draw: {active:?}", active.len());
}

fn scene_batches(f: &mut Fixture) -> HashSet<Entity> {
    let world = f.app.world_mut();
    world.query_filtered::<Entity, With<SceneBatch>>().iter(world).collect()
}

/// Commits `cell` and its twin (`post_twin`: the same table under another cell id).
fn stream_two_cells_in(f: &mut Fixture, cell: IVec3, twin: IVec3) {
    stream_cell_in(f, cell);
    post_twin(f, cell, twin);
    planned(f, twin);
    assert!(commit(f, twin) > 0);
    let state = f.state();
    assert!(state.space.resident(cell).is_some() && state.space.resident(twin).is_some());
    assert_eq!(state.space.stats().cells, 2);
}

/// F3: leaving the world (`reset_scene`, the game's teardown) drops every committed cell and
/// every batch entity, so the next run streams the same cells in through the scene again
/// instead of finding them resident (`plan_in` `Err(Resident)`: fallen back, drawn twice).
#[test]
fn reset_scene_forgets_the_old_run_and_the_same_cells_plan_again() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = fixture(dir.path(), TableKind::Valid(chain_a), true, 4096);
    let (cell, twin) = (f.cell, f.cell + IVec3::Z);
    stream_two_cells_in(&mut f, cell, twin);
    assert!(!scene_batches(&mut f).is_empty());

    reset_scene(f.app.world_mut());

    assert!(scene_batches(&mut f).is_empty(), "no batch entity of the old run is left");
    assert!(f.app.world().resource::<SceneSpaces>().0.is_empty(), "no space of the old run is left");
    assert!(f.app.world().resource::<SceneInbox>().lock().is_empty());
    let uploads = f.app.world().resource::<SceneUploads>();
    assert!(uploads.writes.is_empty() && uploads.zero_fills.is_empty(), "no write to a despawned batch stays queued");
    assert!(f.app.world().resource::<ChainMaterials>().0.is_empty(), "the old run's materials are let go");
    f.app.update();

    // The new run: the same cells plan (`planned` panics on a fallback) and commit through the scene.
    stream_two_cells_in(&mut f, cell, twin);
    let state = f.state();
    let live: HashSet<Entity> = state.batches.iter().flatten().map(|b| b.entity).collect();
    assert!(!live.is_empty());
    assert_eq!(scene_batches(&mut f), live, "only the new run's batches exist");
}
