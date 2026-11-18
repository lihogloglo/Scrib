//! Headless Bevy integration tests for [`AiExecPlugin`] + [`AiExecutor`] (`docs/specs/ai.md`
//! Build plan AI6: headless goals reached, `MoveTo` -> `Face` -> `Wait`, and no stuck on `NoPath`
//! or 3 s `NoProgress`).

use std::sync::Arc;
use std::time::Duration;

use bevy::MinimalPlugins;
use bevy::time::{Fixed, Time, TimeUpdateStrategy};
use glam::{Affine3A, DVec3, Vec2, Vec3};
use sf_ai::{
    ExecGait, FaceTarget, MoveOperatorIds, OperatorBindings, OperatorKind, OperatorState,
    OperatorStatus, Operators, PathFailure, PathState, PlanOutcome, StateKeys, WorldState,
    register_kinds, register_move_operators, wrap_angle, yaw_toward,
};
use sf_console_bevy::ConsolePlugin;
use sf_controller::{
    CollisionBuilder, CollisionMesh, GroundHit, GroundProbe, HeightSurface, MoverParams, Nearest,
    ProbeQuery, StaticShape, StaticWorld, SurfaceId, TerrainProbe,
};
use sf_controller_bevy::{MoveIntent, Mover, SfControllerPlugin};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, BuildConfig, MAX_CLASSES, NavBackend,
    PolyFlags, QueryFilter, TileInput, TileKey,
};
use sf_nav_bevy::{NavCorridor, NavCorridorPlugin, NavMeshes, NavPlugin, NavQueryPlugin};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_world::{Location, WorldspaceId};

use super::*;

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);

fn ws() -> WorldspaceId {
    WorldspaceId::new("ai_exec_room")
}

fn wpos(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::from_f64(DVec3::new(f64::from(x), f64::from(y), f64::from(z)))
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(ws(), wpos(x, y, z))
}

fn xyz(p: WorldPos) -> Vec3 {
    WorldPos::ORIGIN.delta_f32(p)
}

struct FlatFloor;

impl HeightSurface for FlatFloor {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -xyz(origin).y / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

#[derive(Resource)]
struct RoomGround {
    terrain: TerrainProbe<FlatFloor>,
    statics: StaticWorld,
}

impl GroundProbe for RoomGround {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

fn box_collision(half: Vec3) -> Arc<StaticShape> {
    let c = |x: f32, y: f32, z: f32| [x * half.x, y * half.y, z * half.z];
    let corners = [
        c(-1.0, -1.0, -1.0),
        c(1.0, -1.0, -1.0),
        c(1.0, 1.0, -1.0),
        c(-1.0, 1.0, -1.0),
        c(-1.0, -1.0, 1.0),
        c(1.0, -1.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(-1.0, 1.0, 1.0),
    ];
    let idx = [
        0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3,
        1, 2, 6, 1, 6, 5,
    ];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(1));
    let mesh: CollisionMesh = b.finish();
    Arc::new(StaticShape::new(&mesh).unwrap())
}

#[derive(Default)]
struct RoomSoup {
    verts: Vec<Vec3>,
    tris: Vec<[u32; 3]>,
    areas: Vec<AreaId>,
}

impl RoomSoup {
    fn quad(&mut self, q: [Vec3; 4], area: AreaId) {
        let n = u32::try_from(self.verts.len()).unwrap();
        self.verts.extend(q);
        self.tris.extend([[n, n + 1, n + 2], [n, n + 2, n + 3]]);
        self.areas.extend([area, area]);
    }

    fn floor(&mut self, x0: f32, z0: f32, x1: f32, z1: f32, y: f32) {
        self.quad(
            [
                Vec3::new(x0, y, z0),
                Vec3::new(x0, y, z1),
                Vec3::new(x1, y, z1),
                Vec3::new(x1, y, z0),
            ],
            GROUND,
        );
    }

    fn block(&mut self, lo: Vec3, hi: Vec3) {
        self.floor(lo.x, lo.z, hi.x, hi.z, hi.y);
        for [a, b] in [
            [Vec2::new(lo.x, lo.z), Vec2::new(hi.x, lo.z)],
            [Vec2::new(hi.x, lo.z), Vec2::new(hi.x, hi.z)],
            [Vec2::new(hi.x, hi.z), Vec2::new(lo.x, hi.z)],
            [Vec2::new(lo.x, hi.z), Vec2::new(lo.x, lo.z)],
        ] {
            self.quad(
                [
                    Vec3::new(a.x, lo.y, a.y),
                    Vec3::new(b.x, lo.y, b.y),
                    Vec3::new(b.x, hi.y, b.y),
                    Vec3::new(a.x, hi.y, a.y),
                ],
                AreaId::NULL,
            );
        }
    }
}

fn table() -> AreaTable {
    AreaTable::new(vec![AreaDef {
        id: GROUND,
        name: "ground".into(),
        cost: [1.0; MAX_CLASSES],
        flags: PolyFlags::WALK,
    }])
    .unwrap()
}

fn filter() -> QueryFilter {
    QueryFilter::for_class(&table(), H)
}

fn build_room_app() -> App {
    let cfg = BuildConfig {
        class: AgentClass {
            id: H,
            radius: 0.3,
            height: 1.8,
            max_climb: 0.4,
            max_slope_deg: 45.0,
            can_swim: false,
        },
        cell_size: 0.2,
        cell_height: 0.1,
        tile_cells: 40,
    };

    let obstacles = [
        (Vec3::new(4.0, 0.0, 7.0), Vec3::new(6.0, 3.0, 9.0)),
        (Vec3::new(10.0, 0.0, 0.0), Vec3::new(10.6, 3.0, 7.4)),
        (Vec3::new(10.0, 0.0, 8.6), Vec3::new(10.6, 3.0, 16.0)),
    ];

    let mut soup = RoomSoup::default();
    soup.floor(0.0, 0.0, 16.0, 16.0, 0.0);
    let mut statics = StaticWorld::new(WorldPos::ORIGIN);
    for (lo, hi) in obstacles {
        soup.block(lo, hi);
        let shape = box_collision((hi - lo) * 0.5);
        statics.insert(
            &shape,
            WorldTransform::from_pos(WorldPos::ORIGIN.offset((lo + hi) * 0.5)),
        );
    }
    statics.rebuild();

    let areas = table();
    let mut backend = RecastBackend::new(RecastConfig::new(vec![cfg.clone()])).unwrap();
    for tz in 0..2 {
        for tx in 0..2 {
            let input = TileInput {
                tile_x: tx,
                tile_z: tz,
                verts: soup.verts.clone(),
                tris: soup.tris.clone(),
                tri_areas: soup.areas.clone(),
                volumes: Vec::new(),
            };
            let blob = backend.build_tile(&input, &cfg, &areas).unwrap();
            backend
                .add_tile(
                    TileKey {
                        ws: ws(),
                        class: H,
                        region: 0,
                        x: tx,
                        z: tz,
                    },
                    &blob,
                )
                .unwrap();
        }
    }

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ConsolePlugin,
        SfControllerPlugin::<RoomGround>::default(),
        NavPlugin,
        NavQueryPlugin,
        NavCorridorPlugin,
        AiPlugin { timings: false },
        AiExecPlugin,
    ))
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .insert_resource(RoomGround {
        terrain: TerrainProbe(FlatFloor),
        statics,
    })
    .insert_resource(NavMeshes(Box::new(backend)));
    app.update();
    app
}

fn spawn_patrol_actor(app: &mut App, goal: &Location, lookout: WorldPos) -> (Entity, ActorKey, MoveOperatorIds) {
    const RECORDS: &str = r#"[
      define(kind: "sf.ai.task", id: "patrol_point", value: Compound),
      define(kind: "sf.ai.task", id: "walk_to_post", value: Primitive(operator: "move_to")),
      define(kind: "sf.ai.task", id: "face_lookout", value: Primitive(operator: "face")),
      define(kind: "sf.ai.task", id: "hold_watch", value: Primitive(operator: "wait")),

      define(kind: "sf.ai.method", id: "do_patrol_point", value: (
        task: "patrol_point",
        priority: 10,
        subtasks: [Task("walk_to_post"), Task("face_lookout"), Pause, Task("hold_watch")]
      )),
    ]"#;

    let keys = StateKeys::new();
    let mut ops = Operators::new();
    let op_ids = register_move_operators(&mut ops);
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let outcome = apply_record_file(&mut db, src, RECORDS, RecordFormat::Ron).unwrap();
    assert_eq!(outcome.errors, []);
    assert!(load_library(app.world_mut(), &db, &keys, &ops));

    let lib = &app.world().resource::<AiLibrary>().library;
    let root = lib.task_id("patrol_point").unwrap();
    let t_move = lib.task_id("walk_to_post").unwrap();
    let t_face = lib.task_id("face_lookout").unwrap();
    let t_wait = lib.task_id("hold_watch").unwrap();

    let mut bindings = OperatorBindings::new();
    bindings
        .bind_task(
            t_move,
            OperatorKind::MoveTo {
                goal: goal.clone(),
                gait: ExecGait::Walk,
                speed: 2.2,
            },
        )
        .bind_task(
            t_face,
            OperatorKind::Face {
                target: FaceTarget::Point(lookout),
                tolerance_rad: 0.05,
            },
        )
        .bind_task(
            t_wait,
            OperatorKind::Wait {
                duration_s: 0.5,
                facing: yaw_toward(goal.pos, lookout),
                idle_variant: Some(2),
            },
        );

    let actor_key = ActorKey(101);
    let nav = NavCorridor::new(actor_key, H, ws(), 2.2).with_filter(filter());
    let exec = AiExecutor::with_htn(ws(), root, WorldState::new(&keys), bindings);
    let entity = app
        .world_mut()
        .spawn((
            AiActor(actor_key),
            Mover::new(wpos(2.0, 0.0, 8.0), 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav,
            exec,
        ))
        .id();
    (entity, actor_key, op_ids)
}

#[test]
fn actor_executes_htn_move_to_face_and_wait_sequence_to_completion() {
    let mut app = build_room_app();
    let goal = loc(13.5, 0.0, 8.0);
    let lookout = wpos(13.5, 0.0, 14.0);
    let (entity, actor_key, op_ids) = spawn_patrol_actor(&mut app, &goal, lookout);

    let mut wait_started_tick = None;
    let mut completed_tick = None;
    for tick in 1..=900 {
        app.update();
        let exec = app.world().get::<AiExecutor>(entity).unwrap();
        if wait_started_tick.is_none()
            && exec
                .last_output
                .as_ref()
                .and_then(|o| o.operator)
                .is_some_and(|op| op.operator == op_ids.wait && op.status == OperatorStatus::Running)
        {
            wait_started_tick = Some(tick);
        }
        if exec.is_complete() {
            completed_tick = Some(tick);
            break;
        }
        assert!(!exec.is_failed(), "executor failed at tick {tick}");
    }

    let wait_start = wait_started_tick.expect("never entered Wait operator");
    let done_tick = completed_tick.expect("never completed MoveTo -> Face -> Wait plan");
    assert!(
        done_tick - wait_start >= 29,
        "wait finished too early: start={wait_start}, done={done_tick}"
    );

    let mover = app.world().get::<Mover>(entity).unwrap();
    let final_pos = xyz(mover.loco.position);
    let goal_pos = xyz(goal.pos);
    let dist = Vec2::new(final_pos.x - goal_pos.x, final_pos.z - goal_pos.z).length();
    assert!(dist <= 0.25, "final position {final_pos:?} too far from goal ({dist:.3} m)");

    let expected_yaw = yaw_toward(mover.loco.position, lookout).unwrap();
    let yaw_err = wrap_angle(expected_yaw - mover.loco.motion.yaw).abs();
    assert!(
        yaw_err <= 0.06,
        "final yaw {:.3} not facing lookout {:.3} (err {:.3})",
        mover.loco.motion.yaw,
        expected_yaw,
        yaw_err
    );
    assert!(mover.loco.motion.velocity.length() < 0.05);

    let traces = app.world().resource::<AiTraces>();
    let trace = traces.get(actor_key).expect("actor trace recorded");
    assert!(trace.iter().any(|e| e.outcome == Some(PlanOutcome::Planned)));
    assert!(trace.iter().any(|e| e.outcome == Some(PlanOutcome::Continued)));
    assert!(trace.iter().any(|e| e.path == PathState::Following));
    for op in [op_ids.move_to, op_ids.face, op_ids.wait] {
        assert!(
            trace.iter().any(|e| {
                e.operator
                    == Some(OperatorState {
                        operator: op,
                        status: OperatorStatus::Succeeded,
                    })
            }),
            "missing Succeeded trace entry for operator {op:?}"
        );
    }
}

#[test]
fn unreachable_goal_fails_with_no_path_and_blocked_mover_fails_with_no_progress() {
    let mut app = build_room_app();
    let mut ops = Operators::new();
    let op_ids = register_move_operators(&mut ops);

    // 1. Off-mesh / unreachable goal fails with PathFailure::NoPath.
    let actor_nopath = ActorKey(201);
    let nav_nopath = NavCorridor::new(actor_nopath, H, ws(), 2.2).with_filter(filter());
    let exec_nopath = AiExecutor::with_operators(
        ws(),
        [(
            op_ids.move_to,
            OperatorKind::MoveTo {
                goal: loc(100.0, 0.0, 100.0),
                gait: ExecGait::Walk,
                speed: 2.2,
            },
        )],
    );
    let e_nopath = app
        .world_mut()
        .spawn((
            AiActor(actor_nopath),
            Mover::new(wpos(2.0, 0.0, 8.0), 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav_nopath,
            exec_nopath,
        ))
        .id();

    for _ in 0..10 {
        app.update();
        if app.world().get::<AiExecutor>(e_nopath).unwrap().is_failed() {
            break;
        }
    }
    let exec_np = app.world().get::<AiExecutor>(e_nopath).unwrap();
    assert!(exec_np.is_failed());
    assert_eq!(exec_np.last_status(), Some(OperatorStatus::Failed));
    assert_eq!(exec_np.last_path(), PathState::Failed(PathFailure::NoPath));

    // 2. Mover pinned/blocked against an obstacle for 3.0 s fails with PathFailure::NoProgress.
    let actor_stuck = ActorKey(202);
    let pinned_pos = wpos(2.0, 0.0, 8.0);
    let nav_stuck = NavCorridor::new(actor_stuck, H, ws(), 2.2).with_filter(filter());
    let exec_stuck = AiExecutor::with_operators(
        ws(),
        [(
            op_ids.move_to,
            OperatorKind::MoveTo {
                goal: loc(13.5, 0.0, 8.0),
                gait: ExecGait::Walk,
                speed: 2.2,
            },
        )],
    );
    let e_stuck = app
        .world_mut()
        .spawn((
            AiActor(actor_stuck),
            Mover::new(pinned_pos, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            nav_stuck,
            exec_stuck,
        ))
        .id();

    let mut failed_tick = None;
    for tick in 1..=240 {
        app.update();
        {
            let mut mover = app.world_mut().get_mut::<Mover>(e_stuck).unwrap();
            mover.loco.position = pinned_pos;
            mover.loco.motion.velocity = Vec3::ZERO;
            mover.snap();
        }
        if app.world().get::<AiExecutor>(e_stuck).unwrap().is_failed() {
            failed_tick = Some(tick);
            break;
        }
    }

    let stuck_tick = failed_tick.expect("pinned mover should fail with NoProgress after 3.0 s");
    assert!((179..=183).contains(&stuck_tick), "got tick {stuck_tick}");
    let exec_st = app.world().get::<AiExecutor>(e_stuck).unwrap();
    assert_eq!(exec_st.last_status(), Some(OperatorStatus::Failed));
    assert_eq!(exec_st.last_path(), PathState::Failed(PathFailure::NoProgress));

    let traces = app.world().resource::<AiTraces>();
    assert!(
        traces
            .get(actor_nopath)
            .unwrap()
            .iter()
            .any(|e| e.path == PathState::Failed(PathFailure::NoPath))
    );
    assert!(
        traces
            .get(actor_stuck)
            .unwrap()
            .iter()
            .any(|e| e.path == PathState::Failed(PathFailure::NoProgress))
    );
}

/// The `ai.exec.*` tunable defaults and [`sf_ai::ExecConfig::default`] (the Bevy-free twin the
/// core and headless callers use) are the same values, field by field.
#[test]
fn exec_tunable_defaults_match_exec_config_default() {
    assert_eq!(AiExecSettings::default().config, sf_ai::ExecConfig::default());
}
