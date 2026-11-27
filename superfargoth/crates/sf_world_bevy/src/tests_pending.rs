//! Queued spawns (`IN38b3a`, interaction-destruction.md "Spill"): [`Streaming::spawn_when_ready`]
//! queues a placement whose model is not loaded, starts the load at once, and the driver spawns it
//! into its still loaded cell once the model is in, or drops it.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::mesh::MeshPlugin;
use bevy::prelude::*;
use big_space::prelude::FloatingOrigin;
use glam::DVec3;
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{CookSettings, cook};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::{
    IndexParams, IndexedPlacement, Level, MemoryIndex, ModelInfo, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef,
};

use super::*;
use crate::tests::{plate, settle, ws};

/// Cell `A` (x and z 0..128 m): holds the indexed `item`.
const CELL_A: IVec3 = IVec3::new(1, 1, 1);
/// Anchor x inside `A`.
const IN_A_X: f64 = 60.0;
/// Anchor x with no content within any ring.
const AWAY_X: f64 = 5000.0;
/// Cooked, and used by no indexed placement: loads only when a queued spawn asks for it.
const SPILL: &str = "spill.mdl";
/// The indexed placement's model.
const PLATE: &str = "plate.mdl";

/// A model user's tag: one per spawned placement.
#[derive(Component)]
struct Spilled(PlacementId);

/// Spawns one tagged entity per placement; needs its model when it has one (the default).
struct ModelSpawner;

impl PlacementSpawner for ModelSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, p: &IndexedPlacement) -> u32 {
        world.spawn((Spilled(p.placement.id), cx.local, ChildOf(cx.parent)));
        1
    }
}

fn path(model: &str) -> VfsPath {
    VfsPath::new(model).unwrap()
}

fn at_x(x: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, 60.0))
}

/// A dynamic placement at `x` on the anchors' line, with `model`.
fn item(key: &str, x: f64, model: Option<&str>) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: PlacementId::from_key("test", key.as_bytes()),
            base: RecordRef::new("thing", key),
            model: model.map(path),
            transform: WorldTransform::from_pos(at_x(x)),
            class: PlacementClass::Dynamic,
            flags: PlacementFlags::empty(),
        },
        radius: 0.0,
    }
}

/// One cell `A` holding the indexed `item` (plate model), small rings (R0 and R1 load at 20 m,
/// R2 at 100 m), [`ModelSpawner`] for dynamic placements, `plate.mdl` and `spill.mdl` cooked.
/// The anchors start at `gameplay_x` / `visual_x`; returns the app, the visual and gameplay anchors.
fn one_cell(dir: &std::path::Path, config: StreamingConfig, gameplay_x: f64, visual_x: f64) -> (App, Entity, Entity) {
    let store = Store::open(dir).unwrap();
    let cooked = cook(&plate(), &CookSettings::default()).unwrap().to_bytes();
    let mut manifest = Manifest::default();
    for model in [PLATE, SPILL] {
        manifest.products.insert(ProductKey::new("mesh.cook", model), store.put(&cooked, true).unwrap());
    }
    let models = |_: &VfsPath| Some(ModelInfo { min: Vec3::splat(-0.3), max: Vec3::splat(0.3), bytes: 50_000, lod_distance: 0.0 });
    let params = IndexParams { far_min_radius: f32::MAX, ..IndexParams::default() };
    let (pages, _) = sf_world::build_pages(&params, vec![item("item", IN_A_X, Some(PLATE)).placement], &models);
    let directory = sf_world::build_directory(ws(), &params, &pages);
    let mut index = MemoryIndex::new();
    index.insert(&sf_world::WorldIndex { directory, pages, stats: sf_world::IndexStats::default() });

    let ring = sf_world::RingRadii::new(20.0, 30.0);
    let rings =
        sf_world::RingConfig { active: ring, near: ring, far: sf_world::RingRadii::new(100.0, 120.0), ..sf_world::RingConfig::default() };
    let config = StreamingConfig { scheduler: sf_world::SchedulerConfig { rings, ..config.scheduler }, ..config };
    let mut spawners = PlacementSpawners::default();
    spawners.set(PlacementClass::Dynamic, Arc::new(ModelSpawner));

    let mut app = App::new();
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(store, BakedManifest::new(manifest), BTreeMap::from([("mesh.cook".to_owned(), "sfmesh".to_owned())])),
    );
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .add_plugins(WorldStreamingPlugin { config })
        .insert_resource(WorldIndexSource(Arc::new(index)))
        .insert_resource(ActiveWorldspace(Some(ws())))
        .insert_resource(spawners);
    app.finish();
    app.cleanup();
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(at_x(visual_x));
    let visual = app.world_mut().spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root))).id();
    let (cell, transform) = to_grid(at_x(gameplay_x));
    let gameplay = app.world_mut().spawn((cell, transform, StreamingAnchor::gameplay(), ChildOf(root))).id();
    assert_eq!(Partition::STREAMING.cell_of(at_x(IN_A_X)), CELL_A);
    settle_steady(&mut app);
    (app, visual, gameplay)
}

/// [`settle`] until streaming stayed idle for 10 frames in a row.
fn settle_steady(app: &mut App) {
    let mut idle = 0;
    for _ in 0..5000 {
        settle(app);
        idle = if app.world().resource::<Streaming>().is_idle() { idle + 1 } else { 0 };
        if idle >= 10 {
            return;
        }
    }
    panic!("streaming never stayed idle");
}

/// Frames (real time: the model loads asynchronously) until the queue is empty.
fn drain(app: &mut App) {
    for _ in 0..5000 {
        if app.world().resource::<Streaming>().pending_spawns() == 0 {
            return;
        }
        app.update();
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("the queue never emptied");
}

fn queue(app: &mut App, p: IndexedPlacement) -> SpawnWhenReady {
    let spawners = app.world().resource::<PlacementSpawners>().clone();
    app.world_mut().resource_scope(|world, mut streaming: Mut<Streaming>| streaming.spawn_when_ready(world, &spawners, &ws(), CELL_A, p))
}

fn spilled(app: &mut App, id: PlacementId) -> usize {
    app.world_mut().query::<&Spilled>().iter(app.world()).filter(|s| s.0 == id).count()
}

fn users(app: &App, model: &str) -> Option<u32> {
    app.world().resource::<Streaming>().model_users(&path(model))
}

fn level_of(app: &App) -> Level {
    app.world().resource::<Streaming>().scheduler().level(&ws(), CELL_A)
}

fn active_ids(app: &App) -> Option<Vec<PlacementId>> {
    app.world().resource::<Streaming>().active_ids(&ws(), CELL_A)
}

fn dropped(app: &App) -> u64 {
    app.world().resource::<Streaming>().stats().pending_dropped
}

/// A model nobody loaded: queued with no entity and one user of the model at once (its load
/// started); once it settles, exactly one entity, listed in the Active cell, the user now the
/// near group's (released with it at the unload).
#[test]
fn a_queued_spawn_waits_for_its_model_then_spawns_once_listed() {
    let dir = tempfile::tempdir().unwrap();
    let (mut app, visual, gameplay) = one_cell(dir.path(), StreamingConfig::default(), IN_A_X, IN_A_X);
    assert_eq!(level_of(&app), Level::Active);
    assert_eq!(users(&app, SPILL), None, "nothing loaded the spill model");
    let p = item("spilled", IN_A_X + 1.0, Some(SPILL));
    let id = p.placement.id;

    assert_eq!(queue(&mut app, p), SpawnWhenReady::Queued);
    assert_eq!(spilled(&mut app, id), 0, "no entity before the model is in");
    assert_eq!(users(&app, SPILL), Some(1), "the queued entry holds one user");
    assert_eq!(app.world().resource::<Streaming>().pending_spawns(), 1);

    drain(&mut app);
    assert_eq!(spilled(&mut app, id), 1, "exactly one entity");
    assert!(active_ids(&app).unwrap().contains(&id), "listed in the Active cell");
    assert_eq!(users(&app, SPILL), Some(1), "the user passed to the near group");
    assert_eq!(dropped(&app), 0);
    for _ in 0..20 {
        app.update();
    }
    assert_eq!(spilled(&mut app, id), 1, "spawned once");

    app.world_mut().entity_mut(gameplay).insert(to_grid(at_x(AWAY_X)));
    app.world_mut().entity_mut(visual).insert(to_grid(at_x(AWAY_X)));
    settle_steady(&mut app);
    assert_eq!(spilled(&mut app, id), 0, "the near unload despawned it");
    assert_eq!(users(&app, SPILL), None, "and released its model");
}

/// The cell leaves residency before the model is in (the near unload drops the group on its first
/// frame; the model needs its load frame and a settle frame): the entry is dropped, its user
/// released, no entity spawned.
#[test]
fn a_queued_spawn_whose_cell_unloads_first_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let (mut app, visual, _) = one_cell(dir.path(), StreamingConfig::default(), AWAY_X, IN_A_X);
    assert_eq!(level_of(&app), Level::Near);
    let p = item("spilled", IN_A_X + 1.0, Some(SPILL));
    let id = p.placement.id;
    assert_eq!(queue(&mut app, p), SpawnWhenReady::Queued);
    assert_eq!(users(&app, SPILL), Some(1));
    app.world_mut().entity_mut(visual).insert(to_grid(at_x(AWAY_X)));

    settle_steady(&mut app);
    drain(&mut app);
    assert_eq!(spilled(&mut app, id), 0, "never spawned");
    assert_eq!(dropped(&app), 1);
    assert_eq!(users(&app, SPILL), None, "its user released");
}

/// The cell's own list holds the id before the model is in (here a `spawn_into` of the same id; a
/// cell's activation fetch reading it from the overlay is the same case): dropped, one entity.
#[test]
fn a_queued_spawn_the_cell_already_holds_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let (mut app, _, _) = one_cell(dir.path(), StreamingConfig::default(), IN_A_X, IN_A_X);
    let p = item("spilled", IN_A_X + 1.0, Some(SPILL));
    let id = p.placement.id;
    assert_eq!(queue(&mut app, p.clone()), SpawnWhenReady::Queued);
    let mut held = p;
    held.placement.model = None; // spawns at once: its spawner needs no model then
    let spawners = app.world().resource::<PlacementSpawners>().clone();
    let into = app.world_mut().resource_scope(|world, mut s: Mut<Streaming>| s.spawn_into(world, &spawners, &ws(), CELL_A, held));
    assert_eq!(into, Some(SpawnedInto { entities: 1, listed: true }));

    drain(&mut app);
    assert_eq!(spilled(&mut app, id), 1, "one entity, no twin");
    assert_eq!(dropped(&app), 1);
    assert_eq!(users(&app, SPILL), None, "the dropped entry's user released");
}

/// At most `pending_spawns_per_frame` spawns a frame, in call order.
#[test]
fn queued_spawns_respect_the_per_frame_cap() {
    let dir = tempfile::tempdir().unwrap();
    let config = StreamingConfig { pending_spawns_per_frame: 2, ..StreamingConfig::default() };
    let (mut app, _, _) = one_cell(dir.path(), config, IN_A_X, IN_A_X);
    let ids: Vec<PlacementId> = (0..5)
        .map(|i| {
            let p = item(&format!("coin{i}"), IN_A_X + 1.0, None);
            let id = p.placement.id;
            assert_eq!(queue(&mut app, p), SpawnWhenReady::Queued);
            id
        })
        .collect();
    let spawned = |app: &mut App| ids.iter().map(|id| spilled(app, *id)).collect::<Vec<_>>();
    app.update();
    assert_eq!(spawned(&mut app), [1, 1, 0, 0, 0], "two in call order");
    app.update();
    assert_eq!(spawned(&mut app), [1, 1, 1, 1, 0]);
    app.update();
    assert_eq!(spawned(&mut app), [1; 5]);
    assert_eq!(app.world().resource::<Streaming>().pending_spawns(), 0);
}

/// A model that fails to load and no uncooked hook: dropped (with a warning), user released.
#[test]
fn a_queued_spawn_whose_model_fails_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let (mut app, _, _) = one_cell(dir.path(), StreamingConfig::default(), IN_A_X, IN_A_X);
    let p = item("broken", IN_A_X + 1.0, Some("never-cooked.mdl"));
    let id = p.placement.id;
    assert_eq!(queue(&mut app, p), SpawnWhenReady::Queued);
    drain(&mut app);
    assert_eq!(spilled(&mut app, id), 0);
    assert_eq!(dropped(&app), 1);
    assert_eq!(users(&app, "never-cooked.mdl"), None);
}

/// Refusals change nothing: a cell not resident, an id the cell lists or the queue holds, a class
/// no spawner takes.
#[test]
fn queued_spawn_refusals_change_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let (mut app, _, _) = one_cell(dir.path(), StreamingConfig::default(), IN_A_X, IN_A_X);
    let spawners = app.world().resource::<PlacementSpawners>().clone();
    let far = app.world_mut().resource_scope(|world, mut s: Mut<Streaming>| {
        s.spawn_when_ready(world, &spawners, &ws(), IVec3::new(50, 0, 50), item("x", IN_A_X, Some(SPILL)))
    });
    assert_eq!(far, SpawnWhenReady::Refused, "cell not resident");
    assert_eq!(queue(&mut app, item("item", IN_A_X, Some(SPILL))), SpawnWhenReady::Refused, "the indexed item is listed");
    let mut marker = item("marker", IN_A_X, Some(SPILL));
    marker.placement.class = PlacementClass::Marker;
    assert_eq!(queue(&mut app, marker), SpawnWhenReady::Refused, "no spawner for markers");
    assert_eq!(users(&app, SPILL), None, "no refusal took a user");
    assert_eq!(app.world().resource::<Streaming>().pending_spawns(), 0);

    assert_eq!(queue(&mut app, item("twice", IN_A_X, Some(SPILL))), SpawnWhenReady::Queued);
    assert_eq!(queue(&mut app, item("twice", IN_A_X, Some(SPILL))), SpawnWhenReady::Refused, "already queued");
    assert_eq!(users(&app, SPILL), Some(1));
}
