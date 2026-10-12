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

mod items {
    use std::sync::Arc;

    use bevy_app::App;
    use bevy_ecs::prelude::*;
    use bevy_math::{IVec3, Vec3};
    use sf_coords::{WorldPos, WorldTransform};
    use sf_physics::cook::{FitSettings, fit_render};
    use sf_physics::{BodyDesc, Layer, ShapeRef};
    use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
    use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, WorldOverlay};

    use super::app;
    use crate::{ItemBody, ItemContent, Physics, PhysicsBodySource, PhysicsHook, PhysicsShapeSource};

    struct Nothing;
    impl PlacementSpawner for Nothing {
        fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
            0
        }
    }

    struct Items;
    impl ItemContent for Items {
        fn is_item(&self, p: &Placement) -> bool {
            p.class == PlacementClass::Dynamic
        }
        fn item_body(&self, _: &WorldspaceId, _: &Placement, _: f32) -> ItemBody {
            ItemBody { mass: Some(1.0), hold: false }
        }
    }

    fn cube() -> Arc<sf_physics::BodyShape> {
        let h = 0.1;
        let p: Vec<Vec3> = (0..8)
            .map(|i| Vec3::new(if i & 1 == 0 { -h } else { h }, if i & 2 == 0 { -h } else { h }, if i & 4 == 0 { -h } else { h }))
            .collect();
        let t = vec![[0, 2, 1], [1, 2, 3], [4, 5, 6], [5, 7, 6], [0, 1, 4], [1, 5, 4], [2, 6, 3], [3, 6, 7], [0, 4, 2], [2, 4, 6], [1, 3, 5], [3, 7, 5]];
        Arc::new(fit_render(&p, &t, &FitSettings::default()).unwrap())
    }

    /// PH-5 in miniature: an item knocked off its spot is written to the
    /// overlay when its cell deactivates and comes back where it rested, even
    /// when the activation hands over the stale (pre-move) placement.
    #[test]
    fn moved_items_persist_through_deactivation() {
        let ws = WorldspaceId::new("test");
        bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let mut app = app(&ws);
        let shape = cube();
        app.insert_resource(PhysicsBodySource { loader: Arc::new(move |_: &str| Some(shape.clone())), items: Arc::new(Items), supports: None })
            .insert_resource(PhysicsShapeSource(Arc::new(|_: &str| None)))
            .init_resource::<WorldOverlay>();
        let base = WorldPos::new(IVec3::new(1, 0, 1), Vec3::new(5.0, 0.0, 5.0));
        let ground = WorldTransform { pos: base.offset(Vec3::new(0.0, -0.5, 0.0)), ..WorldTransform::IDENTITY };
        app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::fixed(ground, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));
        let placement = IndexedPlacement {
            placement: Placement {
                id: PlacementId(7),
                base: RecordRef::new("MISC", "cube"),
                model: Some(sf_vfs::VfsPath::new("cube.nif").unwrap()),
                transform: WorldTransform { pos: base.offset(Vec3::new(0.0, 0.1, 0.0)), ..WorldTransform::IDENTITY },
                class: PlacementClass::Dynamic,
                flags: PlacementFlags::empty(),
            },
            radius: 0.2,
        };
        let hook = PhysicsHook { inner: Arc::new(Nothing), statics: true };
        let cell_entity = app.world_mut().spawn_empty().id();
        let activate = |app: &mut App, hook: &PhysicsHook| {
            let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity };
            hook.activate(app.world_mut(), &cx, &placement);
            for _ in 0..500 {
                app.update();
                if let Some(id) = app.world().resource::<crate::stream::PhysicsStreaming>().body(&ws, PlacementId(7)) {
                    return id;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            panic!("item not inserted")
        };
        let id = activate(&mut app, &hook);
        for _ in 0..3 {
            app.update();
        }
        let w = app.world().resource::<Physics>().world(&ws).unwrap();
        assert_eq!(w.is_sleeping(id), Some(true), "items go in asleep");
        // Knock it 1 m sideways and let it rest.
        app.world_mut().resource_mut::<Physics>().world_mut(&ws).unwrap().apply_impulse(id, Vec3::new(4.0, 0.5, 0.0), None);
        for _ in 0..240 {
            app.update();
        }
        let rested = app.world().resource::<Physics>().world(&ws).unwrap().pose(id).unwrap();
        assert!(placement.placement.transform.pos.delta(rested.pos).length() > 0.3, "knocked item moved");
        // Leave: the overlay records it; return with the stale placement: it is where it rested.
        let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity };
        hook.deactivate(app.world_mut(), &cx, &placement);
        {
            let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap();
            let (_, moved) = overlay.placed(PlacementId(7)).expect("moved item in the overlay");
            assert!(moved.transform.pos.delta(rested.pos).length() < 1e-4);
            assert_eq!(overlay.state(PlacementId(7), crate::STATE_OWNER).map(|b| b[0]), Some(crate::STATE_VERSION));
        }
        let id = activate(&mut app, &hook);
        let back = app.world().resource::<Physics>().world(&ws).unwrap().pose(id).unwrap();
        assert!(back.pos.delta(rested.pos).length() < 1e-3, "back where it rested");
        assert!(back.rotation.angle_between(rested.rotation) < 1e-3);
    }
}
