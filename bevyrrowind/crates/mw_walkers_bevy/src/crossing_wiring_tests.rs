//! `AI10c3b3`: the real [`rehome_crossers`] against a real `Streaming` (`WorldStreamingPlugin` over
//! an in-memory index, the actor class spawner wrapped in `ActorPoseHook`), so the body's survival
//! of its home cell's unload and the absence of a twin at its reload are the runtime's, not a
//! stand-in's. The recipe (rings, anchors, levels) is `sf_world_bevy`'s `two_cells` test fixture.

use std::sync::{Arc, PoisonError};
use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin, Update};
use bevy_asset::{AssetApp, AssetPlugin};
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec3, Quat, Vec3};
use bevy_mesh::MeshPlugin;
use bevy_pbr::StandardMaterial;
use bevy_time::TimePlugin;
use bevy_transform::components::Transform;
use sf_character_bevy::CharacterPlacement;
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_world::{
    IndexParams, IndexedPlacement, Level, MemoryIndex, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, RingConfig, RingRadii,
    SchedulerConfig, WorldspaceId,
};
use sf_world_bevy::big_space::prelude::FloatingOrigin;
use sf_world_bevy::{
    ActiveWorldspace, PlacementSpawner, PlacementSpawners, SpawnCx, StreamCell, Streaming, StreamingAnchor, StreamingConfig, StreamingSystems,
    WorldIndexSource, WorldOverlay, WorldStreamingPlugin, WorldspaceRegistry, to_grid,
};

use super::{BodyCell, CrossingStats, rehome_crossers};
use crate::poses::{ActorPoseHook, ResidentActors};
use crate::walkers::ActorSpace;

const PARTITION: Partition = Partition::STREAMING;
/// The walker, indexed in cell `A` 4 m west of the A/B border (x = 128 m).
const ID: PlacementId = PlacementId(42);
const HOME_X: f64 = 124.0;
/// Where it walks to: 3 m into cell `B`.
const OVER_X: f64 = 131.0;
/// Two standers that keep `B` in the rings: one 16 m past the border, one under [`IN_B_X`].
const FILLERS: [(PlacementId, f64); 2] = [(PlacementId(43), 156.0), (PlacementId(44), 200.0)];
/// Anchor x on the border: 16 m from both cells' content (R0 and R1 load at 20 m).
const BORDER_X: f64 = 140.0;
/// Anchor x inside `B`: `A`'s content 76 m away (past the 30 m drop, inside the 100 m far ring).
const IN_B_X: f64 = 200.0;
/// Anchor x with no content within any ring.
const AWAY_X: f64 = 5000.0;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_exterior")
}

fn at_x(x: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, 60.0))
}

fn actor(id: PlacementId, x: f64) -> Placement {
    Placement {
        id,
        base: RecordRef::new("NPC_", "test_npc"),
        model: None,
        transform: WorldTransform { pos: at_x(x), rotation: Quat::IDENTITY, scale: Vec3::ONE },
        class: PlacementClass::Actor,
        flags: PlacementFlags::empty(),
    }
}

/// What the spawner was handed: the cell and `SpawnCx::local`.
#[derive(Component, Clone, Copy, Debug)]
struct SpawnedAt {
    cell: IVec3,
    local: Vec3,
}

/// The actor class's inner spawner: a body as `attach` leaves one (mover framed on the cell
/// origin, its `BodyCell`, under `cx.parent` at `cx.local`).
struct Bodies;

impl PlacementSpawner for Bodies {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, p: &IndexedPlacement) -> u32 {
        let origin = PARTITION.cell_origin(cx.cell);
        world.spawn((
            CharacterPlacement(p.placement.id),
            Mover::new(p.placement.transform.pos, 0.0, MoverParams::default(), origin),
            cx.local,
            ActorSpace(cx.worldspace.clone()),
            BodyCell { worldspace: cx.worldspace.clone(), cell: cx.cell },
            SpawnedAt { cell: cx.cell, local: cx.local.translation },
            ChildOf(cx.parent),
        ));
        1
    }
    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        false
    }
}

struct Scene {
    app: App,
    visual: Entity,
    gameplay: Entity,
    a: IVec3,
    b: IVec3,
}

/// The streaming app (no models: no bake store or asset source), the crossing system with the
/// game's ordering, and both anchors on the border: `A` and `B` Active.
fn scene() -> Scene {
    let (a, b) = (PARTITION.cell_of(at_x(HOME_X)), PARTITION.cell_of(at_x(OVER_X)));
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
        .init_resource::<ResidentActors>()
        .init_resource::<CrossingStats>()
        .add_systems(Update, rehome_crossers.after(StreamingSystems::Transition));
    app.finish();
    app.cleanup();
    let mut spawners = PlacementSpawners::default();
    spawners.set(PlacementClass::Actor, Arc::new(ActorPoseHook(Arc::new(Bodies))));
    app.insert_resource(spawners);

    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &ws()).root;
    let (cell, transform) = to_grid(at_x(BORDER_X));
    let visual = app.world_mut().spawn((cell, transform, StreamingAnchor::visual(), FloatingOrigin, ChildOf(root))).id();
    let (cell, transform) = to_grid(at_x(BORDER_X));
    let gameplay = app.world_mut().spawn((cell, transform, StreamingAnchor::gameplay(), ChildOf(root))).id();
    app.world_mut().resource_mut::<Streaming>().begin_burst();
    settle_steady(&mut app);
    Scene { app, visual, gameplay, a, b }
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

fn move_anchor(s: &mut Scene, anchor: Entity, x: f64) {
    s.app.world_mut().entity_mut(anchor).insert(to_grid(at_x(x)));
    settle_steady(&mut s.app);
}

fn levels(s: &Scene) -> (Level, Level) {
    let scheduler = s.app.world().resource::<Streaming>().scheduler();
    (scheduler.level(&ws(), s.a), scheduler.level(&ws(), s.b))
}

/// Every entity carrying placement `ID`'s `CharacterPlacement`.
fn bodies(s: &mut Scene) -> Vec<Entity> {
    let world = s.app.world_mut();
    world.query::<(Entity, &CharacterPlacement)>().iter(world).filter(|(_, p)| p.0 == ID).map(|(e, _)| e).collect()
}

/// The one body of placement `ID`.
fn the_body(s: &mut Scene) -> Entity {
    let found = bodies(s);
    assert_eq!(found.len(), 1, "exactly one entity for the placement: {found:?}");
    found[0]
}

/// The cell of the body's nearest `StreamCell` ancestor.
fn ancestor_cell(s: &mut Scene, body: Entity) -> Option<IVec3> {
    let world = s.app.world_mut();
    let mut links = world.query::<(Option<&StreamCell>, Option<&ChildOf>)>();
    let mut current = body;
    for _ in 0..8 {
        let (cell, parent) = links.get(world, current).ok()?;
        if let Some(c) = cell {
            return Some(c.cell);
        }
        current = parent?.parent();
    }
    None
}

/// Frame origin + local translation: where the body is drawn.
fn drawn(s: &Scene, body: Entity) -> WorldPos {
    let world = s.app.world();
    world.get::<Mover>(body).unwrap().frame_origin.offset(world.get::<Transform>(body).unwrap().translation)
}

fn overlay_in(s: &Scene, cell: IVec3) -> Vec<PlacementId> {
    let overlay = s.app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.apply(&ws(), PARTITION, cell, &[]).iter().map(|p| p.placement.id).collect()
}

#[test]
fn a_crossed_body_outlives_its_home_cell_and_never_gets_a_twin() {
    let mut s = scene();
    assert_eq!(levels(&s), (Level::Active, Level::Active));
    let body = the_body(&mut s);
    assert_eq!(ancestor_cell(&mut s, body), Some(s.a));
    assert_eq!(s.app.world().resource::<ResidentActors>().len(), 3, "A and B listed all three");

    // 1. The walker's feet go 3 m into B, drawn there as `interpolate` writes it.
    let point = at_x(OVER_X);
    {
        let mut e = s.app.world_mut().entity_mut(body);
        let origin = {
            let mut m = e.get_mut::<Mover>().unwrap();
            m.loco.position = point;
            m.frame_origin
        };
        e.get_mut::<Transform>().unwrap().translation = origin.delta_f32(point);
    }
    let b_cell = BodyCell { worldspace: ws(), cell: s.b };
    let mut frames = 0;
    while s.app.world().get::<BodyCell>(body) != Some(&b_cell) {
        assert!(frames < 200, "the swap never landed");
        s.app.update();
        frames += 1;
    }
    assert_eq!(ancestor_cell(&mut s, body), Some(s.b), "hangs under B's cell");
    assert_eq!(s.app.world().get::<Mover>(body).unwrap().frame_origin, PARTITION.cell_origin(s.b));
    assert!(drawn(&s, body).delta_f32(point).length() < 1e-4, "no jump: {:?}", drawn(&s, body));
    assert_eq!(overlay_in(&s, s.b), [ID], "the overlay holds it in B");
    assert!(overlay_in(&s, s.a).is_empty());

    // 2. A drops to Far, B stays Active: the body lives on, alone.
    let (visual, gameplay) = (s.visual, s.gameplay);
    move_anchor(&mut s, gameplay, IN_B_X);
    assert_eq!(levels(&s), (Level::Near, Level::Active));
    move_anchor(&mut s, visual, IN_B_X);
    assert_eq!(levels(&s), (Level::Far, Level::Active));
    assert!(s.app.world().get_entity(body).is_ok(), "the body outlived A's near unload");
    assert_eq!(the_body(&mut s), body);

    // 3. A Active again: its fetch skips the placement (no twin), the body is the same entity.
    move_anchor(&mut s, visual, BORDER_X);
    move_anchor(&mut s, gameplay, BORDER_X);
    assert_eq!(levels(&s), (Level::Active, Level::Active));
    assert_eq!(the_body(&mut s), body, "no twin from A's reload");
    assert_eq!(ancestor_cell(&mut s, body), Some(s.b));

    // 4. Both unload, both load: B's list spawns it once, where it crossed to.
    move_anchor(&mut s, gameplay, AWAY_X);
    move_anchor(&mut s, visual, AWAY_X);
    assert!(s.app.world().get_entity(body).is_err(), "B's near unload despawned it");
    assert!(bodies(&mut s).is_empty());
    move_anchor(&mut s, visual, BORDER_X);
    move_anchor(&mut s, gameplay, BORDER_X);
    assert_eq!(levels(&s), (Level::Active, Level::Active));
    let again = the_body(&mut s);
    let spawned = *s.app.world().get::<SpawnedAt>(again).unwrap();
    assert_eq!(spawned.cell, s.b, "spawned by B's list");
    let at = PARTITION.cell_origin(s.b).offset(spawned.local);
    assert!(at.delta_f32(point).length() < 1e-3, "at the crossed position: {at:?}");
    assert_eq!(s.app.world().get::<BodyCell>(again), Some(&b_cell));
    assert_eq!(s.app.world().resource::<CrossingStats>().rehomed, 1, "one re-home in all");
}
