//! The AI debug overlay over three actors with a **synthetic** trace (visual check AI-1). Nothing
//! decides in game before AI4/AI6, so the entries are pushed at startup, the way the headless tests
//! push them; the overlay draws only what the store holds.
//!
//! `cargo run -p sf_ai_bevy --example ai_overlay --features debug`
//!
//! Backtick opens the console; WASD moves the camera while it is closed. Click a capsule, then
//! `ai.select` (or `ai.select a1`): a ring at its feet and its last decision above its head.
//! `guard_a1` (key a1) planned, then a replan kept its plan (amber ring); `guard_a2` (a2) found no
//! plan (red); `guard_a3` (a3) has no trace yet (grey, "no decision traced yet"). `ai.trace`,
//! `ai.tiers`, `ai.stats`, `ai.gizmos off`, `ai.select off`.

use bevy::prelude::*;
use sf_ai::{
    ActorKey, MethodChoice, OperatorState, OperatorStatus, Operators, Plan, PlanOutcome, PlanStep, RejectReason,
    Rejection, StateKeys, TraceEntry, register_kinds,
};
use sf_ai_bevy::{AiActor, AiLibrary, AiPlugin, AiTraces, load_library};
use sf_console_bevy::overlay::{ConsoleCamera, ConsoleOverlayPlugin, PickRoot};
use sf_console_bevy::{ConsolePlugin, console_closed};
use sf_records::{RecordDb, RecordFormat, apply_record_file};

const LIBRARY: &str = r#"[
  define(kind: "sf.ai.task", id: "be_guard", value: Compound),
  define(kind: "sf.ai.task", id: "p_moveto", value: Primitive(operator: "MoveTo")),
  define(kind: "sf.ai.task", id: "p_wander", value: Primitive(operator: "Wander")),
  define(kind: "sf.ai.task", id: "p_wait", value: Primitive(operator: "Wait")),
  define(kind: "sf.ai.method", id: "patrol", value: (task: "be_guard", priority: 10,
    pre: [Bit(key: "on_duty", value: true)], subtasks: [Task("p_moveto"), Task("p_wait")])),
  define(kind: "sf.ai.method", id: "idle", value: (task: "be_guard",
    subtasks: [Task("p_wander"), Pause, Task("p_wait")])),
]"#;

const ACTORS: [(&str, u128, f32); 3] = [("guard_a1", 0xa1, -2.5), ("guard_a2", 0xa2, 0.0), ("guard_a3", 0xa3, 2.5)];

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ConsolePlugin, ConsoleOverlayPlugin, AiPlugin { timings: false }));
    let mut db = RecordDb::new();
    register_kinds(&mut db).expect("sf.ai kinds register");
    let src = db.add_source("ai_overlay", &[]);
    apply_record_file(&mut db, src, LIBRARY, RecordFormat::Ron).expect("library records parse");
    let mut keys = StateKeys::new();
    keys.add_bit("on_duty").expect("one key");
    let mut ops = Operators::new();
    for op in ["MoveTo", "Wander", "Wait"] {
        ops.add(op);
    }
    load_library(app.world_mut(), &db, &keys, &ops);
    push_synthetic(app.world_mut());
    app.add_systems(Startup, setup).add_systems(Update, fly.run_if(console_closed)).run();
}

/// a1: planned `idle` at tick 100, then a replan at 110 kept it (patrol rejected: not on duty).
/// a2: no plan at 100 (patrol's precondition 0 false, idle's subtask failed). a3: nothing.
fn push_synthetic(world: &mut World) {
    let lib = world.resource::<AiLibrary>().library.clone();
    let task = |n: &str| lib.task_id(n).expect("task");
    let method = |n: &str| lib.method_id(n).expect("method");
    let op = |n: &str| lib.operators().get(n).expect("operator");
    let idle = TraceEntry {
        methods: vec![MethodChoice { task: task("be_guard"), method: method("idle") }],
        plan: Plan { steps: vec![PlanStep { task: task("p_wander"), operator: op("wander") }], cursor: 0, partial: true },
        operator: Some(OperatorState { operator: op("wander"), status: OperatorStatus::Running }),
        mtr: vec![1],
        outcome: Some(PlanOutcome::Planned),
        rejected: vec![Rejection { task: task("be_guard"), method: method("patrol"), reason: RejectReason::Precondition(0) }],
        ..TraceEntry::new(100, 2)
    };
    let mut kept = idle.clone();
    kept.tick = 110;
    kept.outcome = Some(PlanOutcome::KeptNotBetter);
    let no_plan = TraceEntry {
        rejected: vec![
            Rejection { task: task("be_guard"), method: method("patrol"), reason: RejectReason::Precondition(0) },
            Rejection { task: task("be_guard"), method: method("idle"), reason: RejectReason::SubtaskFailed(task("p_wander")) },
        ],
        outcome: Some(PlanOutcome::NoPlan),
        ..TraceEntry::new(100, 2)
    };
    let mut traces = world.resource_mut::<AiTraces>();
    traces.push(ActorKey(0xa1), idle);
    traces.push(ActorKey(0xa1), kept);
    traces.push(ActorKey(0xa2), no_plan);
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Name::new("camera"),
        Camera3d::default(),
        ConsoleCamera,
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
    ));
    commands.spawn((DirectionalLight::default(), Transform::from_xyz(3.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y)));
    commands.spawn((
        Name::new("floor"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.32, 0.3))),
    ));
    let body = meshes.add(Capsule3d::new(0.3, 1.2));
    let skin = materials.add(Color::srgb(0.7, 0.6, 0.5));
    for (name, key, x) in ACTORS {
        // The pick root at the feet, the mesh one level down, as a loaded character would be.
        commands
            .spawn((Name::new(name), PickRoot, AiActor(ActorKey(key)), Transform::from_xyz(x, 0.0, 0.0), Visibility::default()))
            .with_child((Name::new(format!("{name}_mesh")), Mesh3d(body.clone()), MeshMaterial3d(skin.clone()), Transform::from_xyz(0.0, 0.9, 0.0)));
    }
}

/// WASD moves the camera (gated: typing in the console doesn't fly it).
fn fly(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut camera: Query<&mut Transform, With<ConsoleCamera>>) {
    let Ok(mut t) = camera.single_mut() else { return };
    let mut v = Vec3::ZERO;
    for (key, dir) in [
        (KeyCode::KeyW, t.forward().as_vec3()),
        (KeyCode::KeyS, -t.forward().as_vec3()),
        (KeyCode::KeyA, t.left().as_vec3()),
        (KeyCode::KeyD, t.right().as_vec3()),
    ] {
        if keys.pressed(key) {
            v += dir;
        }
    }
    t.translation += v * 5.0 * time.delta_secs();
}
