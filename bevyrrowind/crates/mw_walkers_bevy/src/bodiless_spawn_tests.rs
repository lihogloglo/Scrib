//! `AI10c3c2b1`: a bodiless tier-0 actor that walks from a cell that is not Active into an Active
//! one gets its body at once, through the real `Streaming::spawn_into` (`WorldStreamingPlugin`
//! over an in-memory index, the actor class spawner wrapped in `ActorPoseHook`, as
//! `crossing_wiring_tests.rs`), at the spot the promotion rules pick on a grid navmesh; a seen
//! spot waits and is tried again; a cell that is not loaded spawns nothing.

use std::sync::{Arc, PoisonError};
use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin, Update};
use bevy_asset::{AssetApp, AssetPlugin};
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec3, Quat, Vec3};
use bevy_mesh::MeshPlugin;
use bevy_pbr::StandardMaterial;
use bevy_time::TimePlugin;
use sf_ai::{DirectiveStack, GameTime, LogicalState, RouteRef};
use sf_ai_bevy::{AiActor, AiDirectives, AiLogical, AiLogicalBody};
use sf_character_bevy::CharacterPlacement;
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::{ActorKey, GameInstant};
use sf_nav::{GridBackend, GridCell, GridConfig, PolyFlags};
use sf_nav_bevy::NavMeshes;
use sf_sky_bevy::GameClock;
use sf_world::{
    IndexParams, IndexedPlacement, Level, Location, MemoryIndex, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef,
    RingConfig, RingRadii, SchedulerConfig, WorldspaceId,
};
use sf_world_bevy::big_space::prelude::FloatingOrigin;
use sf_world_bevy::{
    ActiveWorldspace, PlacementSpawner, PlacementSpawners, SpawnCx, Streaming, StreamingAnchor, StreamingConfig, StreamingSystems,
    WorldIndexSource, WorldOverlay, WorldStreamingPlugin, WorldspaceRegistry, to_grid,
};

use super::{BodilessSpawns, follow_bodiless, spawn_bodiless};
use crate::in_view::{AiViewer, View};
use crate::logical::{Tier0Transitions, enter_tier0};
use crate::poses::{ActorPoseHook, ResidentActors};
use crate::walkers::ActorSpace;

const PARTITION: Partition = Partition::STREAMING;
/// The walker, indexed in cell `A` 4 m west of the A/B border (x = 128 m).
const ID: PlacementId = PlacementId(42);
const HOME_X: f64 = 124.0;
/// Where it walks to, 12 m into cell `B`.
const GOAL_X: f64 = 140.0;
/// Walk speed, m/s.
const SPEED: f32 = 2.0;
/// One test tick, game ms (time scale 1: 0.5 m at [`SPEED`]).
const TICK_MS: u64 = 250;
/// Two standers that keep `B` in the rings.
const FILLERS: [(PlacementId, f64); 2] = [(PlacementId(43), 156.0), (PlacementId(44), 200.0)];
/// Anchor x inside `B`: `A`'s content 76 m away (past the 30 m drop: `A` is not Active).
const IN_B_X: f64 = 200.0;
/// Anchor x with no content within any ring.
const AWAY_X: f64 = 5000.0;
/// The row every placement stands on.
const Z: f64 = 60.0;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_exterior")
}

fn at_x(x: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, Z))
}

fn actor(id: PlacementId, x: f64) -> Placement {
    Placement {
        id,
        base: RecordRef::new("NPC_", "test_npc"),
        model: None,
        transform: WorldTransform { pos: at_x(x), rotation: Quat::from_rotation_y(0.7), scale: Vec3::ONE },
        class: PlacementClass::Actor,
        flags: PlacementFlags::empty(),
    }
}

/// What the spawner was handed: the cell and the placement's position.
#[derive(Component, Clone, Copy, Debug)]
struct SpawnedAt {
    cell: IVec3,
    pos: WorldPos,
}

/// The actor class's inner spawner: a body as `attach` leaves one (its `AiActor`, a mover on the
/// placement, under `cx.parent` at `cx.local`).
struct Bodies;

impl PlacementSpawner for Bodies {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, p: &IndexedPlacement) -> u32 {
        let origin = PARTITION.cell_origin(cx.cell);
        world.spawn((
            CharacterPlacement(p.placement.id),
            AiActor(ActorKey(p.placement.id.0)),
            Mover::new(p.placement.transform.pos, 0.0, MoverParams::default(), origin),
            cx.local,
            ActorSpace(cx.worldspace.clone()),
            SpawnedAt { cell: cx.cell, pos: p.placement.transform.pos },
            ChildOf(cx.parent),
        ));
        1
    }
    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        false
    }
}

/// A flat walkable grid over x 0-219, z 0-99 (cell size 1 m), at y = 0.
fn navmesh() -> NavMeshes {
    let mut backend = GridBackend::new(GridConfig { cell_size: 1.0, tile_cells: 40 }).unwrap();
    let row = ".".repeat(220);
    let rows: Vec<&str> = (0..100).map(|_| row.as_str()).collect();
    backend
        .insert_rows(&ws(), mw_bake::nav::HUMANOID, 0.5, &rows, |c| {
            (c == '.').then_some(GridCell::new(0.0, mw_bake::nav::GROUND, PolyFlags::WALK))
        })
        .unwrap();
    NavMeshes(Box::new(backend))
}

struct Scene {
    app: App,
    anchors: [Entity; 2],
    a: IVec3,
    b: IVec3,
}

/// The streaming app, the tier-0 clock at instant 0 (x1), the store, the navmesh and the bodiless
/// chain (`enter_tier0`, then `follow_bodiless` and `spawn_bodiless` as the game chains them),
/// both anchors at `anchor_x`.
fn scene(anchor_x: f64) -> Scene {
    let (a, b) = (PARTITION.cell_of(at_x(HOME_X)), PARTITION.cell_of(at_x(GOAL_X)));
    assert_ne!(a, b, "the walk crosses the border");
    assert!(FILLERS.iter().all(|(_, x)| PARTITION.cell_of(at_x(*x)) == b));
    let params = IndexParams { far_min_radius: f32::MAX, ..IndexParams::default() };
    assert_eq!(params.partition, PARTITION);
    let placements = std::iter::once(actor(ID, HOME_X)).chain(FILLERS.iter().map(|(id, x)| actor(*id, *x)));
    let (pages, _) = sf_world::build_pages(&params, placements, &|_| None);
    let directory = sf_world::build_directory(ws(), &params, &pages);
    let mut index = MemoryIndex::new();
    index.insert(&sf_world::WorldIndex { directory, pages, stats: sf_world::IndexStats::default() });

    let ring = RingRadii::new(20.0, 30.0);
    let rings = RingConfig { active: ring, near: ring, far: RingRadii::new(100.0, 120.0), ..RingConfig::default() };
    let config = StreamingConfig { scheduler: SchedulerConfig { rings, ..SchedulerConfig::default() }, ..StreamingConfig::default() };
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .add_plugins(WorldStreamingPlugin { config })
        .insert_resource(WorldIndexSource(Arc::new(index)))
        .insert_resource(ActiveWorldspace(Some(ws())))
        .insert_resource(GameClock::new(GameInstant::ZERO, 1.0))
        .insert_resource(navmesh())
        .init_resource::<ResidentActors>()
        .init_resource::<AiLogical>()
        .init_resource::<AiDirectives>()
        .init_resource::<Tier0Transitions>()
        .init_resource::<BodilessSpawns>()
        .add_systems(Update, (enter_tier0, follow_bodiless, spawn_bodiless).chain().after(StreamingSystems::Transition));
    app.finish();
    app.cleanup();
    let mut spawners = PlacementSpawners::default();
    spawners.set(PlacementClass::Actor, Arc::new(ActorPoseHook(Arc::new(Bodies))));
    app.insert_resource(spawners);

    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(at_x(anchor_x));
    let visual = app.world_mut().spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root))).id();
    let (cell, transform) = to_grid(at_x(anchor_x));
    let gameplay = app.world_mut().spawn((cell, transform, StreamingAnchor::gameplay(), ChildOf(root))).id();
    app.world_mut().resource_mut::<Streaming>().begin_burst();
    settle_steady(&mut app);
    Scene { app, anchors: [visual, gameplay], a, b }
}

/// Frames until streaming stayed idle for 10 in a row (async IO needs real time).
fn settle_steady(app: &mut App) {
    let mut idle = 0;
    for frame in 0..20_000 {
        app.update();
        idle = if frame > 3 && app.world().resource::<Streaming>().is_idle() { idle + 1 } else { 0 };
        if idle >= 10 {
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("streaming never stayed idle");
}

fn move_anchors(s: &mut Scene, x: f64) {
    for anchor in s.anchors {
        s.app.world_mut().entity_mut(anchor).insert(to_grid(at_x(x)));
    }
    settle_steady(&mut s.app);
}

fn levels(s: &Scene) -> (Level, Level) {
    let scheduler = s.app.world().resource::<Streaming>().scheduler();
    (scheduler.level(&ws(), s.a), scheduler.level(&ws(), s.b))
}

/// A stored walker from `HOME_X` to `GOAL_X` since instant 0, its stack, and its placement kept as
/// its cell's deactivation keeps it (indexed in `A`).
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "test metres and ms")]
fn store_walker(s: &mut Scene) {
    let (from, to) = (Location::new(ws(), at_x(HOME_X)), Location::new(ws(), at_x(GOAL_X)));
    let length_m = from.pos.delta_f32(to.pos).length();
    let mut state = LogicalState::new(from.clone(), GameInstant::ZERO);
    state.route = Some(RouteRef { from, to, length_m, speed_mps: SPEED });
    state.arrives = Some(GameInstant((f64::from(length_m / SPEED) * 1000.0) as u64));
    let world = s.app.world_mut();
    world.resource_mut::<AiLogical>().insert(ActorKey(ID.0), state);
    world.resource_mut::<AiDirectives>().insert(DirectiveStack::new(ActorKey(ID.0), GameTime::default()));
    world.resource_mut::<ResidentActors>().away_mut().insert(ID, (ws(), actor(ID, HOME_X)));
}

/// The walker's logical x at instant `ms`.
#[expect(clippy::cast_precision_loss, reason = "test ms")]
fn logical_x(ms: u64) -> f64 {
    (HOME_X + f64::from(SPEED) * ms as f64 / 1000.0).min(GOAL_X)
}

/// One frame at game instant `ms`.
fn tick(s: &mut Scene, ms: u64) {
    s.app.world_mut().resource_mut::<GameClock>().now = GameInstant(ms);
    s.app.update();
}

/// Every entity carrying placement `ID`'s `CharacterPlacement`.
fn bodies(s: &mut Scene) -> Vec<Entity> {
    let world = s.app.world_mut();
    world.query::<(Entity, &CharacterPlacement)>().iter(world).filter(|(_, p)| p.0 == ID).map(|(e, _)| e).collect()
}

fn the_body(s: &mut Scene) -> Entity {
    let found = bodies(s);
    assert_eq!(found.len(), 1, "exactly one entity for the placement: {found:?}");
    found[0]
}

fn overlay_placed(s: &Scene) -> Option<(WorldspaceId, Placement)> {
    let overlay = s.app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.placed(ID).map(|(w, p)| (w.clone(), p.clone()))
}

fn kept(s: &Scene) -> bool {
    s.app.world().resource::<ResidentActors>().kept_placement(ID).is_some()
}

fn spawned(s: &Scene) -> u32 {
    s.app.world().resource::<BodilessSpawns>().spawned
}

fn stored(s: &Scene) -> bool {
    s.app.world().resource::<AiLogical>().contains(ActorKey(ID.0))
}

/// Ticks from instant 0 until a body exists; returns the instant it appeared.
fn walk_until_body(s: &mut Scene, max_ticks: u64) -> Option<u64> {
    for i in 0..max_ticks {
        let ms = i * TICK_MS;
        tick(s, ms);
        if !bodies(s).is_empty() {
            return Some(ms);
        }
    }
    None
}

#[test]
fn a_bodiless_walker_entering_an_active_cell_gets_one_body_at_once_at_its_unseen_spot() {
    let mut s = scene(IN_B_X);
    assert_eq!(levels(&s).1, Level::Active);
    assert_ne!(levels(&s).0, Level::Active, "A is not simulated");
    assert!(bodies(&mut s).is_empty());
    store_walker(&mut s);

    let appeared = walk_until_body(&mut s, 40).expect("a body appeared");
    assert_eq!(PARTITION.cell_of(at_x(logical_x(appeared))), s.b, "on the tick it stands in B");
    assert_ne!(PARTITION.cell_of(at_x(logical_x(appeared - TICK_MS))), s.b, "not before it entered B");
    let body = the_body(&mut s);
    let spawned_at = *s.app.world().get::<SpawnedAt>(body).unwrap();
    assert_eq!(spawned_at.cell, s.b, "spawned into B");
    let spot = at_x(logical_x(appeared));
    let d = spot.delta_f32(spawned_at.pos);
    assert!(d.y.abs() < 1e-3 && d.with_y(0.0).length() < 1.0, "at its spot, snapped: {:?} vs {spot:?}", spawned_at.pos);
    let (w, p) = overlay_placed(&s).expect("an overlay entry");
    assert_eq!((w, p.transform.pos, p.transform.rotation), (ws(), spawned_at.pos, Quat::from_rotation_y(0.7)), "the overlay holds the spawn");
    assert!(!kept(&s), "the kept entry is consumed");
    assert!(stored(&s), "the store keeps it until it is placed");
    assert_eq!(spawned(&s), 1);

    // The next ticks mark the body and spawn no twin.
    for i in 1..8 {
        tick(&mut s, appeared + i * TICK_MS);
    }
    assert_eq!(the_body(&mut s), body, "no twin");
    assert!(s.app.world().entity(body).contains::<AiLogicalBody>(), "re-marked as a stored body");
    let state = s.app.world().resource::<AiLogical>().get(ActorKey(ID.0)).unwrap().clone();
    assert_eq!(state.route.as_ref().map(|r| r.to.pos), Some(at_x(GOAL_X)), "its state untouched");
    assert_eq!(spawned(&s), 1);

    // B unloads and loads again: its own list spawns the one body, where it was spawned.
    move_anchors(&mut s, AWAY_X);
    assert!(bodies(&mut s).is_empty(), "B's unload despawned it");
    move_anchors(&mut s, IN_B_X);
    assert_eq!(levels(&s).1, Level::Active);
    let again = the_body(&mut s);
    assert_eq!(s.app.world().get::<SpawnedAt>(again).unwrap().pos, spawned_at.pos, "B's list, at the spawned spot");
    assert_eq!(spawned(&s), 1, "B's fetch, not a second spawn_into");
}

#[test]
fn a_seen_spot_waits_and_is_tried_again_once_unseen() {
    let mut s = scene(IN_B_X);
    // A viewer 15 m south of the walk, looking north over x 122-146: every candidate is seen.
    let eye = WorldPos::from_f64(DVec3::new(134.0, 1.6, Z + 15.0));
    let viewer = s.app.world_mut().spawn(AiViewer(View { eye, rotation: Quat::IDENTITY, fov_y: 1.0, aspect: 1.5 })).id();
    store_walker(&mut s);
    let mut entered = None;
    for i in 0..40u64 {
        let ms = i * TICK_MS;
        tick(&mut s, ms);
        if entered.is_none() && PARTITION.cell_of(at_x(logical_x(ms))) == s.b {
            entered = Some(i);
        }
        assert!(bodies(&mut s).is_empty(), "spawned in view at {ms} ms");
    }
    let entered = entered.expect("it entered B");
    assert!(kept(&s) && stored(&s), "it waits, kept and stored");
    let (w, p) = overlay_placed(&s).expect("the follow wrote it into B");
    assert_eq!((w, PARTITION.cell_of(p.transform.pos)), (ws(), s.b));

    // Out of view: the retry (0.5 s at x1 = 2 ticks) spawns it within a few ticks.
    s.app.world_mut().despawn(viewer);
    let last = 39 * TICK_MS;
    let mut appeared = None;
    for i in 1..=4u64 {
        tick(&mut s, last + i * TICK_MS);
        if !bodies(&mut s).is_empty() {
            appeared = Some(i);
            break;
        }
    }
    assert!(appeared.is_some(), "never tried again (entered B at tick {entered})");
    let body = the_body(&mut s);
    assert_eq!(s.app.world().get::<SpawnedAt>(body).unwrap().cell, s.b);
    assert!(!kept(&s));
    assert_eq!(spawned(&s), 1);
}

#[test]
fn a_walker_entering_an_unloaded_cell_gets_no_body_until_its_fetch() {
    let mut s = scene(AWAY_X);
    assert_eq!(levels(&s), (Level::Unloaded, Level::Unloaded));
    store_walker(&mut s);
    for i in 0..40u64 {
        tick(&mut s, i * TICK_MS);
    }
    assert!(bodies(&mut s).is_empty(), "nothing spawned into an unloaded cell");
    assert_eq!(spawned(&s), 0);
    assert!(kept(&s), "still kept");
    let (_, p) = overlay_placed(&s).expect("the follow wrote it into B");
    assert_eq!(PARTITION.cell_of(p.transform.pos), s.b);

    // B loads: its own fetch spawns the one body from the overlay (AI10c3c2a), no spawn_into.
    move_anchors(&mut s, IN_B_X);
    assert_eq!(levels(&s).1, Level::Active);
    let body = the_body(&mut s);
    assert_eq!(s.app.world().get::<SpawnedAt>(body).unwrap().cell, s.b);
    assert_eq!(spawned(&s), 0);
    assert!(!kept(&s), "activation dropped the kept entry");
}
