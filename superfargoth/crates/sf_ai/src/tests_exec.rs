//! Unit tests for `sf_ai::exec` (`MoveTo`, `Wait`, `Face`, `PlanRunner`; `docs/specs/ai.md` "Executors",
//! slice AI6).

use std::f32::consts::FRAC_PI_2;

use glam::DVec3;
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_world::{Location, WorldspaceId};

use crate::*;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_world")
}

fn wpos(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::from_f64(DVec3::new(f64::from(x), f64::from(y), f64::from(z)))
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(ws(), wpos(x, y, z))
}

fn ctx_at(x: f32, z: f32, yaw: f32, dt: f32, nav: NavMoveState) -> ExecContext {
    ExecContext {
        location: loc(x, 0.0, z),
        yaw,
        dt,
        nav,
    }
}

#[test]
fn move_to_succeeds_on_arrival_or_initial_proximity() {
    let mut ops = Operators::new();
    let ids = register_move_operators(&mut ops);
    let cfg = ExecConfig::default();
    let goal = loc(10.0, 0.0, 0.0);

    // 1. Already within arrive_dist_m on the first step succeeds immediately without requesting a path.
    let mut exec = OperatorExec::new(
        ids.move_to,
        OperatorKind::MoveTo {
            goal: goal.clone(),
            gait: ExecGait::Walk,
            speed: 2.0,
        },
    );
    let out = exec.step(&ctx_at(9.9, 0.0, 0.0, 0.5, NavMoveState::Idle), &cfg);
    assert_eq!(
        out.operator,
        Some(OperatorState {
            operator: ids.move_to,
            status: OperatorStatus::Succeeded,
        })
    );
    assert_eq!(out.path, PathState::None);
    assert_eq!(out.nav, NavCommand::Stop);

    // 2. Normal flow: Pending -> Following -> Arrived.
    let mut exec = OperatorExec::new(
        ids.move_to,
        OperatorKind::MoveTo {
            goal: goal.clone(),
            gait: ExecGait::Run,
            speed: 3.5,
        },
    );
    let out1 = exec.step(&ctx_at(0.0, 0.0, 0.0, 0.5, NavMoveState::Idle), &cfg);
    assert_eq!(out1.operator.unwrap().status, OperatorStatus::Running);
    assert_eq!(out1.path, PathState::Pending);
    assert_eq!(
        out1.nav,
        NavCommand::MoveTo {
            goal,
            gait: ExecGait::Run,
            speed: 3.5,
        }
    );

    let out2 = exec.step(&ctx_at(5.0, 0.0, FRAC_PI_2, 0.5, NavMoveState::Following), &cfg);
    assert_eq!(out2.operator.unwrap().status, OperatorStatus::Running);
    assert_eq!(out2.path, PathState::Following);

    let out3 = exec.step(&ctx_at(10.0, 0.0, FRAC_PI_2, 0.5, NavMoveState::Arrived), &cfg);
    assert_eq!(out3.operator.unwrap().status, OperatorStatus::Succeeded);
    assert_eq!(out3.path, PathState::None);
    assert_eq!(out3.nav, NavCommand::Stop);
}

#[test]
fn move_to_fails_with_no_path_and_no_progress_after_stuck_timeout() {
    let mut ops = Operators::new();
    let ids = register_move_operators(&mut ops);
    let cfg = ExecConfig::default();
    let goal = loc(20.0, 0.0, 0.0);

    // 1. NavMoveState::Failed -> PathFailure::NoPath immediately.
    let mut exec = OperatorExec::new(
        ids.move_to,
        OperatorKind::MoveTo {
            goal: goal.clone(),
            gait: ExecGait::Walk,
            speed: 2.0,
        },
    );
    let _ = exec.step(&ctx_at(0.0, 0.0, 0.0, 0.5, NavMoveState::Pending), &cfg);
    let failed = exec.step(&ctx_at(0.0, 0.0, 0.0, 0.5, NavMoveState::Failed), &cfg);
    assert_eq!(failed.operator.unwrap().status, OperatorStatus::Failed);
    assert_eq!(failed.path, PathState::Failed(PathFailure::NoPath));
    assert_eq!(failed.nav, NavCommand::Stop);

    // 2. Stuck timer resets when >= progress_min_m (0.25 m) is closed, then fails at 3.0 s without progress.
    let mut exec = OperatorExec::new(
        ids.move_to,
        OperatorKind::MoveTo {
            goal,
            gait: ExecGait::Walk,
            speed: 2.0,
        },
    );
    let _ = exec.step(&ctx_at(0.0, 0.0, 0.0, 0.0, NavMoveState::Following), &cfg);
    for _ in 0..5 {
        let out = exec.step(&ctx_at(0.1, 0.0, 0.0, 0.5, NavMoveState::Following), &cfg);
        assert_eq!(out.operator.unwrap().status, OperatorStatus::Running);
    }
    let progressed = exec.step(&ctx_at(0.5, 0.0, 0.0, 0.5, NavMoveState::Following), &cfg);
    assert_eq!(progressed.operator.unwrap().status, OperatorStatus::Running);

    for _ in 0..5 {
        let out = exec.step(&ctx_at(0.5, 0.0, 0.0, 0.5, NavMoveState::Following), &cfg);
        assert_eq!(out.operator.unwrap().status, OperatorStatus::Running);
    }
    let stuck = exec.step(&ctx_at(0.5, 0.0, 0.0, 0.5, NavMoveState::Following), &cfg);
    assert_eq!(stuck.operator.unwrap().status, OperatorStatus::Failed);
    assert_eq!(stuck.path, PathState::Failed(PathFailure::NoProgress));
    assert_eq!(stuck.nav, NavCommand::Stop);
}

#[test]
fn wait_counts_down_and_holds_facing_and_idle_variant() {
    let mut ops = Operators::new();
    let ids = register_move_operators(&mut ops);
    let cfg = ExecConfig::default();

    let mut zero = OperatorExec::new(
        ids.wait,
        OperatorKind::Wait {
            duration_s: 0.0,
            facing: Some(1.25),
            idle_variant: Some(3),
        },
    );
    let out = zero.step(&ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Idle), &cfg);
    assert_eq!(out.operator.unwrap().status, OperatorStatus::Succeeded);
    assert_eq!(out.facing, Some(1.25));
    assert_eq!(out.idle_variant, Some(3));
    assert_eq!(out.nav, NavCommand::Stop);

    let mut wait = OperatorExec::new(
        ids.wait,
        OperatorKind::Wait {
            duration_s: 0.5,
            facing: Some(-0.5),
            idle_variant: Some(2),
        },
    );
    for _ in 0..2 {
        let step = wait.step(&ctx_at(0.0, 0.0, 0.0, 0.2, NavMoveState::Idle), &cfg);
        assert_eq!(step.operator.unwrap().status, OperatorStatus::Running);
        assert_eq!(step.facing, Some(-0.5));
        assert_eq!(step.idle_variant, Some(2));
    }
    let done = wait.step(&ctx_at(0.0, 0.0, 0.0, 0.2, NavMoveState::Idle), &cfg);
    assert_eq!(done.operator.unwrap().status, OperatorStatus::Succeeded);
}

#[test]
fn face_yaw_and_point_succeed_within_tolerance() {
    let mut ops = Operators::new();
    let ids = register_move_operators(&mut ops);
    let cfg = ExecConfig::default();

    let mut face_yaw = OperatorExec::new(
        ids.face,
        OperatorKind::Face {
            target: FaceTarget::Yaw(FRAC_PI_2),
            tolerance_rad: 0.05,
        },
    );
    let r1 = face_yaw.step(&ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Idle), &cfg);
    assert_eq!(r1.operator.unwrap().status, OperatorStatus::Running);
    assert!((r1.facing.unwrap() - FRAC_PI_2).abs() < 1e-6);

    let r2 = face_yaw.step(&ctx_at(0.0, 0.0, FRAC_PI_2 - 0.03, 0.1, NavMoveState::Idle), &cfg);
    assert_eq!(r2.operator.unwrap().status, OperatorStatus::Succeeded);

    let mut face_pt = OperatorExec::new(
        ids.face,
        OperatorKind::Face {
            target: FaceTarget::Point(wpos(10.0, 0.0, 0.0)),
            tolerance_rad: 0.05,
        },
    );
    let p1 = face_pt.step(&ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Idle), &cfg);
    assert_eq!(p1.operator.unwrap().status, OperatorStatus::Running);
    assert!((p1.facing.unwrap() - FRAC_PI_2).abs() < 1e-6);

    let p2 = face_pt.step(&ctx_at(0.0, 0.0, FRAC_PI_2 + 0.02, 0.1, NavMoveState::Idle), &cfg);
    assert_eq!(p2.operator.unwrap().status, OperatorStatus::Succeeded);
}

struct HtnFixture {
    lib: Library,
    root: TaskId,
    k_alert: BitKey,
    k_patrol: BitKey,
    op_ids: MoveOperatorIds,
    bindings: OperatorBindings,
    state: WorldState,
}

fn build_htn_fixture() -> HtnFixture {
    const RECORDS: &str = r#"[
      define(kind: "sf.ai.task", id: "root", value: Compound),
      define(kind: "sf.ai.task", id: "t_move", value: Primitive(
        operator: "move_to",
        pre: [Bit(key: "can_patrol", value: true)]
      )),
      define(kind: "sf.ai.task", id: "t_face", value: Primitive(operator: "face")),
      define(kind: "sf.ai.task", id: "t_wait", value: Primitive(operator: "wait")),
      define(kind: "sf.ai.task", id: "t_alert_face", value: Primitive(operator: "face")),

      define(kind: "sf.ai.method", id: "m_alert", value: (
        task: "root",
        priority: 20,
        pre: [Bit(key: "alert", value: true)],
        subtasks: [Task("t_alert_face")]
      )),
      define(kind: "sf.ai.method", id: "m_patrol", value: (
        task: "root",
        priority: 10,
        pre: [Bit(key: "can_patrol", value: true)],
        subtasks: [Task("t_move"), Task("t_face"), Pause, Task("t_wait")]
      )),
      define(kind: "sf.ai.method", id: "m_fallback", value: (
        task: "root",
        priority: 1,
        subtasks: [Task("t_wait")]
      )),
    ]"#;

    let mut keys = StateKeys::new();
    let k_alert = keys.add_bit("alert").unwrap();
    let k_patrol = keys.add_bit("can_patrol").unwrap();
    let mut ops = Operators::new();
    let op_ids = register_move_operators(&mut ops);

    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let outcome = apply_record_file(&mut db, src, RECORDS, RecordFormat::Ron).unwrap();
    assert_eq!(outcome.errors, []);
    let lib = Library::build(&db, &keys, &ops).unwrap();

    let root = lib.task_id("root").unwrap();
    let t_move = lib.task_id("t_move").unwrap();
    let t_face = lib.task_id("t_face").unwrap();
    let t_wait = lib.task_id("t_wait").unwrap();
    let t_alert_face = lib.task_id("t_alert_face").unwrap();

    let mut bindings = OperatorBindings::new();
    bindings
        .bind_task(
            t_move,
            OperatorKind::MoveTo {
                goal: loc(5.0, 0.0, 0.0),
                gait: ExecGait::Walk,
                speed: 2.0,
            },
        )
        .bind_task(
            t_face,
            OperatorKind::Face {
                target: FaceTarget::Yaw(FRAC_PI_2),
                tolerance_rad: 0.05,
            },
        )
        .bind_task(
            t_wait,
            OperatorKind::Wait {
                duration_s: 0.2,
                facing: Some(FRAC_PI_2),
                idle_variant: Some(1),
            },
        )
        .bind_task(
            t_alert_face,
            OperatorKind::Face {
                target: FaceTarget::Yaw(0.0),
                tolerance_rad: 0.05,
            },
        );

    let mut state = WorldState::new(&keys);
    state.set_bit(k_patrol, true);
    HtnFixture {
        lib,
        root,
        k_alert,
        k_patrol,
        op_ids,
        bindings,
        state,
    }
}

#[test]
fn plan_runner_executes_htn_resumes_pause_monitors_preconditions_and_mtr_replans() {
    let mut f = build_htn_fixture();
    let cfg = ExecConfig::default();
    let mut runner = PlanRunner::new();
    let mut store = TraceStore::new();
    store.set_timings(false);
    let actor = ActorKey(42);

    // Tick 1: decomposes `m_patrol` up to `Pause` ([t_move, t_face], partial = true), starts `t_move`.
    let out1 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Pending),
        &cfg,
        store.record(actor, 1, 2),
    );
    assert_eq!(out1.plan_outcome, Some(PlanOutcome::Planned));
    assert_eq!(out1.operator.unwrap().operator, f.op_ids.move_to);
    assert_eq!(out1.path, PathState::Pending);

    // Tick 2: arrive at `goal` -> `t_move` succeeds.
    let out2 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(5.0, 0.0, 0.0, 0.1, NavMoveState::Arrived),
        &cfg,
        store.record(actor, 2, 2),
    );
    assert_eq!(out2.operator.unwrap().status, OperatorStatus::Succeeded);

    // Tick 3: `t_face` runs and succeeds once yaw == FRAC_PI_2.
    let out3 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(5.0, 0.0, FRAC_PI_2, 0.1, NavMoveState::Idle),
        &cfg,
        store.record(actor, 3, 2),
    );
    assert_eq!(out3.operator.unwrap().status, OperatorStatus::Succeeded);

    // Tick 4: resumes past `Pause` (`PlanOutcome::Continued`) into `[t_wait]` and steps `t_wait`.
    let out4 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(5.0, 0.0, FRAC_PI_2, 0.1, NavMoveState::Idle),
        &cfg,
        store.record(actor, 4, 2),
    );
    assert_eq!(out4.plan_outcome, Some(PlanOutcome::Continued));
    assert_eq!(out4.operator.unwrap().status, OperatorStatus::Running);

    // Tick 5: `t_wait` finishes -> plan is complete!
    let out5 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(5.0, 0.0, FRAC_PI_2, 0.1, NavMoveState::Idle),
        &cfg,
        store.record(actor, 5, 2),
    );
    assert_eq!(out5.operator.unwrap().status, OperatorStatus::Succeeded);
    assert!(runner.is_complete());

    // Precondition monitoring (`Library::holds`) and dirty-key MTR replan:
    runner.request_replan();
    let _ = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Following),
        &cfg,
        store.record(actor, 6, 2),
    );
    f.state.set_bit(f.k_patrol, false);
    let out7 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Following),
        &cfg,
        store.record(actor, 7, 2),
    );
    assert_eq!(out7.plan_outcome, Some(PlanOutcome::Planned));
    assert_eq!(out7.operator.unwrap().operator, f.op_ids.wait);

    f.state.set_bit(f.k_alert, true);
    runner.mark_dirty(StateKey::Bit(f.k_alert));
    let out8 = runner.step_htn(
        f.root,
        &f.state,
        &f.lib,
        &f.bindings,
        &ctx_at(0.0, 0.0, 0.0, 0.1, NavMoveState::Idle),
        &cfg,
        store.record(actor, 8, 2),
    );
    assert_eq!(out8.plan_outcome, Some(PlanOutcome::Replaced));
    assert_eq!(out8.operator.unwrap().status, OperatorStatus::Succeeded);

    let trace = store.get(actor).unwrap();
    assert_eq!(trace.len(), 8);
    assert_eq!(trace.iter().nth(6).unwrap().trigger, Trigger::Replan);
    assert_eq!(trace.iter().nth(7).unwrap().trigger, Trigger::Replan);
}
