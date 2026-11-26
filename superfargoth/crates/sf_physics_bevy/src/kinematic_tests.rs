//! `KinematicPlacements`: a listed streamed static goes in as a kinematic body
//! the owner can move; an unlisted one stays fixed; deactivation removes it.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, IVec3, Vec3};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use sf_controller::{CollisionBuilder, StaticShape, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyId, BodyKind};
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, PlacementSpawner, SpawnCx};

use crate::{KinematicPlacements, Physics, PhysicsHook, PhysicsPlugin, PhysicsShapeSource, PhysicsStreaming};

const ID: PlacementId = PlacementId(5);

struct Nothing;
impl PlacementSpawner for Nothing {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
}

fn app(ws: &WorldspaceId) -> App {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())));
    // A 2 x 2 m upright panel (a door leaf), one-sided.
    let mut b = CollisionBuilder::new(1e-3);
    let p = [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 2.0, 0.0], [-1.0, 2.0, 0.0]];
    b.add(&p, &[0, 1, 2, 0, 2, 3], Affine3A::IDENTITY, SurfaceId::DEFAULT);
    let shape = Arc::new(StaticShape::new(&b.finish()).expect("panel mesh"));
    app.insert_resource(PhysicsShapeSource(Arc::new(move |_: &str| Some(shape.clone()))));
    app
}

fn leaf() -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: ID,
            base: RecordRef::new("DOOR", "leaf"),
            model: Some(sf_vfs::VfsPath::new("leaf.nif").unwrap()),
            transform: WorldTransform { pos: WorldPos::new(IVec3::new(1, 0, 1), Vec3::new(5.0, 0.0, 5.0)), ..WorldTransform::IDENTITY },
            class: PlacementClass::Static,
            flags: PlacementFlags::empty(),
        },
        radius: 1.5,
    }
}

/// Activates the leaf and steps until its body is in; returns the cell context's entity and the body.
fn stream_leaf(app: &mut App, ws: &WorldspaceId, hook: &PhysicsHook) -> (Entity, BodyId) {
    let cell_entity = app.world_mut().spawn_empty().id();
    hook.activate(app.world_mut(), &ActivateCx { worldspace: ws, cell: IVec3::ZERO, cell_entity, discard: false }, &leaf());
    for _ in 0..500 {
        app.update();
        if let Some(b) = app.world().resource::<PhysicsStreaming>().body(ws, ID) {
            return (cell_entity, b);
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("leaf not inserted");
}

fn hook() -> PhysicsHook {
    PhysicsHook { inner: Arc::new(Nothing), statics: true }
}

fn kind(app: &App, ws: &WorldspaceId, b: BodyId) -> Option<BodyKind> {
    app.world().resource::<Physics>().world(ws).unwrap().kind(b)
}

#[test]
fn listed_static_goes_in_kinematic_and_follows_its_target() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws);
    assert!(app.world_mut().resource_mut::<KinematicPlacements>().insert(&ws, ID));
    let (_, b) = stream_leaf(&mut app, &ws, &hook());
    assert_eq!(kind(&app, &ws, b), Some(BodyKind::Kinematic));
    let start = app.world().resource::<Physics>().world(&ws).unwrap().pose(b).unwrap();
    let target = WorldTransform { pos: start.pos.offset(Vec3::new(0.5, 0.0, 0.0)), ..start };
    assert!(app.world_mut().resource_mut::<Physics>().world_mut(&ws).unwrap().set_kinematic_target(b, target));
    // The fixed clock steps on the next frames; the target holds until replaced.
    for _ in 0..3 {
        app.update();
    }
    let now = app.world().resource::<Physics>().world(&ws).unwrap().pose(b).unwrap();
    let off = target.pos.delta(now.pos).length();
    assert!(off < 1e-4, "kinematic leaf {off} m off its target");
}

#[test]
fn unlisted_static_stays_fixed() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws);
    let (_, b) = stream_leaf(&mut app, &ws, &hook());
    assert_eq!(kind(&app, &ws, b), Some(BodyKind::Fixed));
    let start = app.world().resource::<Physics>().world(&ws).unwrap().pose(b).unwrap();
    let target = WorldTransform { pos: start.pos.offset(Vec3::new(0.5, 0.0, 0.0)), ..start };
    assert!(!app.world_mut().resource_mut::<Physics>().world_mut(&ws).unwrap().set_kinematic_target(b, target));
}

#[test]
fn deactivation_removes_the_kinematic_body_and_keeps_the_entry() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws);
    app.world_mut().resource_mut::<KinematicPlacements>().insert(&ws, ID);
    let hook = hook();
    let (cell_entity, b) = stream_leaf(&mut app, &ws, &hook);
    hook.deactivate(app.world_mut(), &ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false }, &leaf());
    app.update();
    assert_eq!(app.world().resource::<PhysicsStreaming>().body(&ws, ID), None);
    assert!(!app.world().resource::<Physics>().world(&ws).unwrap().contains(b), "body left the world");
    assert!(app.world().resource::<KinematicPlacements>().contains(&ws, ID), "the owner removes the entry");
}
