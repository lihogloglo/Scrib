//! Headless plugin test: spawn, step, rebase, despawn.

use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyKind, Layer, ShapeRef};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, from_grid};

use crate::{Physics, PhysicsBody, PhysicsFocus, PhysicsPlugin, PhysicsStats};

fn app(ws: &WorldspaceId) -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())));
    app
}

fn pos_of(app: &mut App, e: Entity) -> WorldPos {
    let (cell, t) = app.world_mut().query::<(&CellCoord, &Transform)>().get(app.world(), e).map(|(c, t)| (*c, *t)).unwrap();
    from_grid(&cell, &t)
}

#[test]
fn spawn_step_rebase_despawn() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws);
    let base = WorldPos::new(IVec3::new(3, 0, -2), Vec3::new(10.0, 0.0, 20.0));
    let at = |d: Vec3| WorldTransform { pos: base.offset(d), ..WorldTransform::IDENTITY };
    let mut physics = app.world_mut().resource_mut::<Physics>();
    physics.insert(&ws, &BodyDesc::fixed(at(Vec3::ZERO), ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(Vec3::new(0.0, 4.0, 0.0)), ShapeRef::Box(Vec3::splat(0.5)), Layer::PROP);
    let id = physics.insert(&ws, &desc);
    let e = app.world_mut().spawn((PhysicsBody { worldspace: ws.clone(), id }, CellCoord::default(), Transform::default())).id();

    // Falls 3 m onto the ground and settles: center 0.5 + 0.5 above the base.
    let mut max_steps = 0;
    for _ in 0..180 {
        app.update();
        max_steps = max_steps.max(app.world().resource::<PhysicsStats>().steps);
    }
    assert!(max_steps <= 3, "at most 3 steps per frame, got {max_steps}");
    let rest = pos_of(&mut app, e);
    let d = base.delta(rest);
    assert!((d.y - 1.0).abs() < 0.02 && d.x.abs() < 0.02 && d.z.abs() < 0.02, "resting at {d:?}");

    // A focus 3 cells away rebases the world; the body doesn't move.
    let world_pose = |app: &App| app.world().resource::<Physics>().world(&ws).unwrap().pose(id).unwrap().pos;
    let before = world_pose(&app);
    app.world_mut().resource_mut::<PhysicsFocus>().0 = Some(WorldPos::new(IVec3::new(6, 0, -2), Vec3::ZERO));
    app.update();
    let w = app.world().resource::<Physics>().world(&ws).unwrap();
    assert_eq!(w.anchor().cell, IVec3::new(6, 0, -2));
    let moved = before.delta(world_pose(&app)).length();
    assert!(moved < 1e-4, "rebase moved the body {moved} m");

    // Despawning removes the body; the ground stays.
    app.world_mut().despawn(e);
    app.update();
    let w = app.world().resource::<Physics>().world(&ws).unwrap();
    assert_eq!(w.len(), 1);
    assert!(!w.contains(id));
}
