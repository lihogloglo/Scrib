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

    pub(super) struct Items;
    impl ItemContent for Items {
        fn is_item(&self, p: &Placement) -> bool {
            p.class == PlacementClass::Dynamic
        }
        fn item_body(&self, _: &WorldspaceId, _: &Placement, _: f32) -> ItemBody {
            ItemBody { mass: Some(1.0), hold: false }
        }
    }

    pub(super) fn cube() -> Arc<sf_physics::BodyShape> {
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
            let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false };
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
        let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false };
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

/// Fallfix (2026-10-18): characters fell through an interior floor whose
/// static was still waiting for insert budget when they started simulating.
mod streamed_ground {
    use std::sync::Arc;

    use bevy_app::App;
    use bevy_ecs::prelude::*;
    use bevy_math::{Affine3A, IVec3, Vec3};
    use sf_controller::{CollisionBuilder, GroundProbe, ProbeQuery, StaticShape, SurfaceId};
    use sf_coords::{WorldPos, WorldTransform};
    use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
    use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx};

    use super::app;
    use crate::{Physics, PhysicsHook, PhysicsShapeSource, PhysicsStreaming};

    struct Nothing;
    impl PlacementSpawner for Nothing {
        fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
            0
        }
    }

    /// A one-sided 10 m floor slab at y = 0 (model space), facing up.
    pub(super) fn floor() -> Arc<StaticShape> {
        let mut b = CollisionBuilder::new(1e-3);
        let p = [[-5.0, 0.0, -5.0], [5.0, 0.0, -5.0], [5.0, 0.0, 5.0], [-5.0, 0.0, 5.0]];
        b.add(&p, &[0, 2, 1, 0, 3, 2], Affine3A::IDENTITY, SurfaceId::DEFAULT);
        Arc::new(StaticShape::new(&b.finish()).expect("floor mesh"))
    }

    fn base() -> WorldPos {
        WorldPos::new(IVec3::new(1, 0, 1), Vec3::new(5.0, 0.0, 5.0))
    }

    fn floor_placement() -> IndexedPlacement {
        IndexedPlacement {
            placement: Placement {
                id: PlacementId(3),
                base: RecordRef::new("STAT", "floor"),
                model: Some(sf_vfs::VfsPath::new("floor.nif").unwrap()),
                transform: WorldTransform { pos: base(), ..WorldTransform::IDENTITY },
                class: PlacementClass::Static,
                flags: PlacementFlags::empty(),
            },
            radius: 7.1,
        }
    }

    /// Activates the floor and steps until its body is in the world.
    #[cfg(feature = "controller")]
    fn stream_floor(app: &mut App, ws: &WorldspaceId) {
        let shape = floor();
        app.insert_resource(PhysicsShapeSource(Arc::new(move |_: &str| Some(shape.clone()))));
        let cell_entity = app.world_mut().spawn_empty().id();
        let hook = PhysicsHook { inner: Arc::new(Nothing), statics: true };
        hook.activate(app.world_mut(), &ActivateCx { worldspace: ws, cell: IVec3::ZERO, cell_entity, discard: false }, &floor_placement());
        for _ in 0..500 {
            app.update();
            if app.world().resource::<PhysicsStreaming>().body(ws, PlacementId(3)).is_some() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!("floor not inserted");
    }

    /// The collision-ready gate: an activated static counts as loading
    /// around its bounds until its body is in the world, and then the ground
    /// probe stands on it. Far away and other worldspaces never count.
    #[test]
    fn collision_loading_until_the_static_is_in() {
        let ws = WorldspaceId::new("test");
        bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let mut app = app(&ws);
        let shape = floor();
        app.insert_resource(PhysicsShapeSource(Arc::new(move |_: &str| Some(shape.clone()))));
        let s = |app: &App, ws: &WorldspaceId, at: Vec3| app.world().resource::<PhysicsStreaming>().collision_loading(None, ws, base().offset(at), 4.0);
        assert!(!s(&app, &ws, Vec3::ZERO), "nothing activated");
        let cell_entity = app.world_mut().spawn_empty().id();
        let hook = PhysicsHook { inner: Arc::new(Nothing), statics: true };
        hook.activate(app.world_mut(), &ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false }, &floor_placement());
        assert!(s(&app, &ws, Vec3::new(1.0, 1.0, -2.0)), "on the floor");
        assert!(s(&app, &ws, Vec3::new(10.0, 0.0, 0.0)), "within its bounds + the margin");
        assert!(!s(&app, &ws, Vec3::new(30.0, 0.0, 0.0)), "far away");
        assert!(!s(&app, &WorldspaceId::new("other"), Vec3::ZERO), "another worldspace");
        // Before the floor is in, nothing is under a character standing on it.
        let down = ProbeQuery::down(base().offset(Vec3::new(1.0, 0.5, 1.0)), 2.0);
        assert!(app.world().resource::<Physics>().probe_one(down).is_none());
        let mut frames = 0;
        while s(&app, &ws, Vec3::ZERO) {
            frames += 1;
            assert!(frames < 500, "floor never inserted");
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert!(app.world().resource::<PhysicsStreaming>().body(&ws, PlacementId(3)).is_some());
        let hit = app.world().resource::<Physics>().probe_one(down).expect("the floor is ground once in");
        assert!(base().delta_f32(hit.point).y.abs() < 1e-3);
    }

    /// The out-of-world safety net: a walking character under the kill
    /// height goes back to its last walkable ground and stands there again.
    #[cfg(feature = "controller")]
    #[test]
    fn fallen_character_returns_to_its_last_ground() {
        use sf_controller_bevy::{MoveIntent, Mover, MoverParams, SfControllerPlugin};

        use crate::CharacterStats;

        let ws = WorldspaceId::new("test");
        bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let mut app = app(&ws);
        app.add_plugins(SfControllerPlugin::<Physics>::default());
        stream_floor(&mut app, &ws);
        let start = base().offset(Vec3::new(1.0, 0.2, -1.0));
        let e = app
            .world_mut()
            .spawn((Mover::new(start, 0.0, MoverParams::default(), base()), MoveIntent::default(), bevy_transform::components::Transform::default()))
            .id();
        let feet = |app: &App| app.world().get::<Mover>(e).unwrap().loco.position;
        for _ in 0..30 {
            app.update();
        }
        let stood = feet(&app);
        assert!(app.world().get::<Mover>(e).unwrap().loco.grounded.is_some(), "landed on the floor");
        assert!(base().delta_f32(stood).y.abs() < 0.01, "on the floor: {:?}", base().delta_f32(stood));
        // Fallen out (as if the floor had been missing): 50 m under it, falling.
        {
            let mut m = app.world_mut().get_mut::<Mover>(e).unwrap();
            m.loco.position = base().offset(Vec3::new(1.0, -50.0, -1.0));
            m.loco.grounded = None;
            m.loco.vertical_speed = -50.0;
        }
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.world().resource::<CharacterStats>().fell_out, 1);
        let back = feet(&app);
        assert!(stood.delta_f32(back).length() < 0.05, "back on its ground: {:?}", stood.delta_f32(back));
        assert!(app.world().get::<Mover>(e).unwrap().loco.grounded.is_some(), "standing again");
    }

    /// AN-7: a walking character's capsule keeps a steady place under it at
    /// any frame rate. The mover runs per frame and physics at 60 Hz: at
    /// 144 fps, targeting the last frame's feet moved the capsule by 0, 1 or 2
    /// frames of walking per step, so the drawn (interpolated) capsule shook
    /// against the body by about a centimetre at walking speed.
    #[cfg(feature = "controller")]
    #[test]
    fn walking_capsule_keeps_steady_under_the_mover_at_144_fps() {
        use bevy_time::{Fixed, Time, TimeUpdateStrategy};
        use sf_controller::LocomotionIntent;
        use sf_controller_bevy::{MoveIntent, Mover, MoverParams, SfControllerPlugin};

        use crate::character::Capsules;

        let ws = WorldspaceId::new("test");
        bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let mut app = app(&ws);
        app.add_plugins(SfControllerPlugin::<Physics>::default())
            .insert_resource(TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(1.0 / 144.0)));
        stream_floor(&mut app, &ws);
        let start = base().offset(Vec3::new(-2.0, 0.0, -1.0));
        let intent = LocomotionIntent { desired_velocity: Vec3::X * 1.4, ..LocomotionIntent::IDLE };
        let e = app
            .world_mut()
            .spawn((Mover::new(start, 0.0, MoverParams::default(), base()), MoveIntent(intent), bevy_transform::components::Transform::default()))
            .id();
        let mut offsets = Vec::new();
        for frame in 0..400 {
            app.update();
            if frame < 200 {
                continue;
            }
            let feet = app.world().get::<Mover>(e).unwrap().loco.position;
            let alpha = app.world().resource::<Time<Fixed>>().overstep_fraction();
            let id = app.world().resource::<Capsules>().0[&e].id;
            let pose = app.world().resource::<Physics>().worlds.active_world().unwrap().pose_at(id, alpha).unwrap();
            offsets.push(feet.delta_f32(pose.pos).x);
        }
        let (lo, hi) = offsets.iter().fold((f32::MAX, f32::MIN), |(l, h), &o| (l.min(o), h.max(o)));
        assert!(lo < -0.01, "interpolation draws a step behind: {lo}");
        assert!(hi - lo < 0.002, "the capsule shakes {:.1} mm against the mover ({lo:.4}..{hi:.4} m)", (hi - lo) * 1000.0);
    }
}

/// G1 (physics-fix-plan.md slice 1): the scheduler activates cells a few
/// placements per frame, so an item's cell can be active while the cell
/// holding the floor under it is not. The item must wait for that floor.
mod activation_gate {
    use std::sync::Arc;

    use bevy_app::App;
    use bevy_ecs::prelude::*;
    use bevy_math::{DVec3, IVec3, Vec3};
    use sf_coords::{WorldPos, WorldTransform};
    use sf_world::{
        Anchor, AnchorId, AnchorRole, IndexParams, IndexedPlacement, Level, Location, ModelInfo, Placement, PlacementClass, PlacementFlags,
        PlacementId, RecordRef, Scheduler, Stage, StageStep, TickInput, WorldspaceId, build_directory, build_pages,
    };
    use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, Streaming, StreamingConfig};

    use super::app;
    use super::items::{Items, cube};
    use super::streamed_ground::floor;
    use crate::{Physics, PhysicsBodySource, PhysicsHook, PhysicsShapeSource, PhysicsStreaming};

    struct Nothing;
    impl PlacementSpawner for Nothing {
        fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
            0
        }
    }

    /// The item stands 5 cm left of the 128 m cell boundary at x = 256; the
    /// floor's origin is in the next cell, its 10 m slab under both.
    fn item_pos() -> WorldPos {
        WorldPos::from_f64(DVec3::new(255.95, 0.1, 64.0))
    }

    fn floor_pos() -> WorldPos {
        WorldPos::from_f64(DVec3::new(258.0, 0.0, 64.0))
    }

    fn placement(id: u128, pos: WorldPos, model: &str, class: PlacementClass, radius: f32) -> IndexedPlacement {
        IndexedPlacement {
            placement: Placement {
                id: PlacementId(id),
                base: RecordRef::new("TEST", model),
                model: Some(sf_vfs::VfsPath::new(model).unwrap()),
                transform: WorldTransform { pos, ..WorldTransform::IDENTITY },
                class,
                flags: PlacementFlags::empty(),
            },
            radius,
        }
    }

    /// A scheduler that knows the two cells (content: the item, the floor).
    fn scheduler(ws: &WorldspaceId, item: &IndexedPlacement, floor: &IndexedPlacement) -> Scheduler {
        let params = IndexParams::default();
        let models = |_: &sf_vfs::VfsPath| Some(ModelInfo { min: Vec3::splat(-0.5), max: Vec3::splat(0.5), bytes: 1000, lod_distance: 100.0 });
        let (pages, _) = build_pages(&params, [item.placement.clone(), floor.placement.clone()], &models);
        let mut s = Scheduler::new(StreamingConfig::default().scheduler);
        assert!(s.add_worldspace(build_directory(ws.clone(), &params, &pages), None));
        s
    }

    /// One scheduler frame, every stage finishing at once except the
    /// activation of `hold` (time-sliced activation still running there).
    fn drive(app: &mut App, ws: &WorldspaceId, now: &mut f64, hold: Option<IVec3>) {
        let mut streaming = app.world_mut().resource_mut::<Streaming>();
        let s = streaming.scheduler_mut();
        let anchors: Vec<Anchor> = [AnchorRole::Gameplay, AnchorRole::Visual]
            .into_iter()
            .zip(0..)
            .map(|(role, i)| Anchor::new(AnchorId(i), role, Location::new(ws.clone(), item_pos())))
            .collect();
        let report = s.update(&TickInput { now: *now, anchors: &anchors });
        for id in report.start {
            match s.request(id).map(|r| r.stage) {
                Some(Stage::Fetch) => s.fetched(id, *now, 1, 1),
                Some(Stage::Assets) => s.assets_loaded(id, *now),
                _ => false,
            };
        }
        for stage in [Stage::Deactivate, Stage::Despawn, Stage::Spawn, Stage::Upload, Stage::Activate] {
            s.run_stage(stage, &mut |r| StageStep { cost: 0.01, done: !(stage == Stage::Activate && Some(r.cell) == hold) });
        }
        *now += 1.0 / 60.0;
    }

    fn level(app: &App, ws: &WorldspaceId, pos: WorldPos) -> Level {
        let s = app.world().resource::<Streaming>().scheduler();
        s.level(ws, s.directory(ws).unwrap().partition.cell_of(pos))
    }

    fn body(app: &App, ws: &WorldspaceId, id: u128) -> Option<sf_physics::BodyId> {
        app.world().resource::<PhysicsStreaming>().body(ws, PlacementId(id))
    }

    /// Red on the old gate (it only saw activated statics): the item went in
    /// on an empty world and fell, or woke when the floor arrived.
    #[test]
    fn item_waits_for_the_floor_of_a_later_cell() {
        let ws = WorldspaceId::new("test");
        bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let mut app = app(&ws);
        let item = placement(7, item_pos(), "cube.nif", PlacementClass::Dynamic, 0.2);
        let slab = placement(3, floor_pos(), "floor.nif", PlacementClass::Static, 7.1);
        let (body_shape, static_shape) = (cube(), floor());
        let mut streaming = Streaming::new(StreamingConfig::default());
        *streaming.scheduler_mut() = scheduler(&ws, &item, &slab);
        app.insert_resource(PhysicsBodySource { loader: Arc::new(move |_: &str| Some(body_shape.clone())), items: Arc::new(Items), supports: None })
            .insert_resource(PhysicsShapeSource(Arc::new(move |_: &str| Some(static_shape.clone()))))
            .insert_resource(streaming);
        let cell_of = |app: &App, pos| app.world().resource::<Streaming>().scheduler().directory(&ws).unwrap().partition.cell_of(pos);
        let (item_cell, floor_cell) = (cell_of(&app, item_pos()), cell_of(&app, floor_pos()));
        assert_ne!(item_cell, floor_cell);

        // The item's cell activates; the floor's is still activating.
        let mut now = 0.0;
        let mut frames = 0;
        while level(&app, &ws, item_pos()) < Level::Active {
            drive(&mut app, &ws, &mut now, Some(floor_cell));
            frames += 1;
            assert!(frames < 2000, "the item's cell never activated");
        }
        assert!(level(&app, &ws, floor_pos()) < Level::Active);
        let hook = PhysicsHook { inner: Arc::new(Nothing), statics: true };
        let cell_entity = app.world_mut().spawn_empty().id();
        hook.activate(app.world_mut(), &ActivateCx { worldspace: &ws, cell: item_cell, cell_entity, discard: false }, &item);
        for _ in 0..60 {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert!(body(&app, &ws, 7).is_none(), "the item went in before the floor's cell was active");
        let loading = |app: &App| {
            let (p, s) = (app.world().resource::<PhysicsStreaming>(), app.world().resource::<Streaming>());
            p.collision_loading(Some(s.scheduler()), &ws, item_pos(), 0.2)
        };
        assert!(loading(&app), "collision under the item reads as loading");

        // The floor's cell activates (its static is queued), then the item follows it in.
        hook.activate(app.world_mut(), &ActivateCx { worldspace: &ws, cell: floor_cell, cell_entity, discard: false }, &slab);
        while level(&app, &ws, floor_pos()) < Level::Active {
            drive(&mut app, &ws, &mut now, None);
            frames += 1;
            assert!(frames < 4000, "the floor's cell never activated");
        }
        let mut waited = 0;
        while body(&app, &ws, 7).is_none() {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(1));
            waited += 1;
            assert!(waited < 500, "item never inserted");
            if body(&app, &ws, 7).is_some() {
                assert!(body(&app, &ws, 3).is_some(), "the item went in before the floor's body");
            }
        }
        assert!(!loading(&app));
        for _ in 0..240 {
            app.update();
        }
        let id = body(&app, &ws, 7).unwrap();
        let w = app.world().resource::<Physics>().world(&ws).unwrap();
        let rest = w.pose(id).unwrap().pos;
        let d = floor_pos().delta(rest);
        assert!(d.is_finite(), "NaN pose");
        assert!(d.y > -0.05, "the item fell through the floor: {d:?}");
        assert!(item_pos().delta(rest).length() < 0.05, "the item moved from its spot: {:?}", item_pos().delta(rest));
        assert_eq!(w.is_sleeping(id), Some(true), "the item is asleep on the floor");
    }
}
