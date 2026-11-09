//! ai.md "Build plan" AI3: headless smoke (the console commands' exact lines over synthetic trace
//! entries: nothing decides in game before AI4/AI6), the load report's log line, the BRP answer.

use std::sync::{Mutex, Once};

use bevy::MinimalPlugins;
use serde_json::json;
use sf_ai::{
    MethodChoice, OperatorState, OperatorStatus, Operators, Plan, PlanOutcome, PlanStep, StateKeys, TraceEntry,
    register_kinds,
};
use sf_console_bevy::sf_console::Level;
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit, DevConsole};
use sf_records::{RecordFormat, apply_record_file};

use super::*;

const LIBRARY: &str = r#"[
  define(kind: "sf.ai.task", id: "be_guard", value: Compound),
  define(kind: "sf.ai.task", id: "p_wander", value: Primitive(operator: "Wander")),
  define(kind: "sf.ai.task", id: "p_wait", value: Primitive(operator: "Wait")),
  define(kind: "sf.ai.method", id: "idle", value: (task: "be_guard", subtasks: [Task("p_wander"), Pause, Task("p_wait")])),
]"#;

/// One method naming a task nobody defines.
const BAD_LIBRARY: &str = r#"[
  define(kind: "sf.ai.task", id: "p_wait", value: Primitive(operator: "Wait")),
  define(kind: "sf.ai.method", id: "bad_method", value: (task: "no_such_task", subtasks: [Task("p_wait")])),
]"#;

const A: ActorKey = ActorKey(7);
const A_HEX: &str = "00000000000000000000000000000007";
const B: ActorKey = ActorKey(u128::MAX);

fn db(text: &str) -> RecordDb {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let outcome = apply_record_file(&mut db, src, text, RecordFormat::Ron).unwrap();
    assert_eq!(outcome.errors, []);
    db
}

fn operators() -> Operators {
    let mut ops = Operators::new();
    ops.add("Wander");
    ops.add("Wait");
    ops
}

fn app(library: &str) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin, AiPlugin { timings: false }));
    load_library(app.world_mut(), &db(library), &StateKeys::new(), &operators());
    app.update();
    app
}

/// The idle plan as it would be recorded at `tick`: wander running, wait after the pause.
fn idle_entry(lib: &Library, tick: u64) -> TraceEntry {
    let be_guard = lib.task_id("be_guard").unwrap();
    let wander = lib.task_id("p_wander").unwrap();
    let op = lib.operators().get("wander").unwrap();
    TraceEntry {
        methods: vec![MethodChoice { task: be_guard, method: lib.method_id("idle").unwrap() }],
        plan: Plan { steps: vec![PlanStep { task: wander, operator: op }], cursor: 0, partial: true },
        operator: Some(OperatorState { operator: op, status: OperatorStatus::Running }),
        mtr: vec![0],
        outcome: Some(PlanOutcome::Planned),
        ..TraceEntry::new(tick, 2)
    }
}

fn run(app: &mut App, line: &str) -> Vec<(Level, String)> {
    let before = app.world().resource::<DevConsole>().log().lines().len();
    app.world_mut().write_message(ConsoleSubmit::exec(line));
    app.update();
    app.world().resource::<DevConsole>().log().lines().skip(before).map(|l| (l.level, l.text.clone())).collect()
}

fn info(lines: &[(Level, String)]) -> Vec<&str> {
    lines.iter().filter(|(level, _)| *level == Level::Info).map(|(_, t)| t.as_str()).collect()
}

#[test]
fn ai_trace_prints_the_selected_actors_steps_with_names() {
    let mut app = app(LIBRARY);
    assert_eq!(
        info(&run(&mut app, &format!("ai.select {A_HEX}"))),
        [format!("selected actor {A_HEX}: watched, trace 0/64")]
    );
    let entry = idle_entry(&app.world().resource::<AiLibrary>().library, 10);
    assert!(app.world_mut().resource_mut::<AiTraces>().push(A, entry));
    assert_eq!(
        info(&run(&mut app, "ai.trace")),
        [
            format!("actor {A_HEX}: last 1 of 1 entries").as_str(),
            "#10 t2 scheduled drive - via be_guard:idle rej - plan planned op wander running path -",
        ]
    );
    // A short key and an explicit count work too; unknown ids fall back to indices.
    let mut synthetic = TraceEntry::new(11, 1);
    synthetic.methods.push(MethodChoice { task: TaskId(40), method: MethodId(41) });
    synthetic.outcome = Some(PlanOutcome::KeptNotBetter);
    app.world_mut().resource_mut::<AiTraces>().push(A, synthetic);
    assert_eq!(
        info(&run(&mut app, "ai.trace 1 7")),
        [
            format!("actor {A_HEX}: last 1 of 2 entries").as_str(),
            "#11 t1 scheduled drive - via t40:m41 rej - plan kept_not_better op - path -",
        ]
    );
    assert_eq!(
        info(&run(&mut app, "ai.tiers")),
        ["tiers (provisional: each traced actor's last step; no allocator before AI9): t2 0, t1 1, t0 0 (1 traced)"]
    );
    assert_eq!(
        info(&run(&mut app, "ai.stats")),
        ["traces on, admit tier2, timings off: 1 traced of cap 256, 1 watched; library 3 tasks, 1 methods, 0 load errors"]
    );
    assert_eq!(
        info(&run(&mut app, "ai.select off")),
        [format!("deselected actor {A_HEX} (unwatched, trace dropped)")]
    );
    let errors = run(&mut app, "ai.trace");
    assert_eq!(errors.last().map(|(l, _)| *l), Some(Level::Error));
    assert!(errors.last().unwrap().1.contains("no actor selected"), "{errors:?}");
    let errors = run(&mut app, "ai.trace 3 nothex");
    assert!(errors.last().unwrap().1.contains("bad actor key \"nothex\""), "{errors:?}");
}

#[test]
fn an_unknown_actor_says_why_it_has_no_trace() {
    let mut app = app(LIBRARY);
    let lines = run(&mut app, "ai.trace 8 2a");
    assert_eq!(lines.last().map(|(l, _)| *l), Some(Level::Error));
    assert!(
        lines.last().unwrap().1.ends_with(
            "actor 0000000000000000000000000000002a: not traced; `ai.select` it to trace it from its next step"
        ),
        "{lines:?}"
    );
}

#[test]
fn overlay_text_shows_the_last_decision_or_says_there_is_none() {
    let app = app(LIBRARY);
    let lib = &app.world().resource::<AiLibrary>().library;
    let mut traces = TraceStore::new();
    traces.watch(A);
    assert_eq!(overlay_lines(&traces, lib, A), [format!("ai {A_HEX} (watched)"), "no decision traced yet".into()]);
    let mut entry = idle_entry(lib, 12);
    entry.plan.steps.push(PlanStep { task: lib.task_id("p_wait").unwrap(), operator: lib.operators().get("wait").unwrap() });
    entry.plan.cursor = 1;
    traces.push(A, entry);
    assert_eq!(
        overlay_lines(&traces, lib, A),
        [
            format!("ai {A_HEX} (watched)"),
            "#12 t2 plan planned".into(),
            "plan p_wander > [p_wait] | paused".into(),
            "op wander running".into(),
        ]
    );
}

/// Log records of target `sf_ai`, from every test in this binary (filter by text).
static CAPTURED: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Capture;

impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        if record.target() == "sf_ai" {
            CAPTURED.lock().unwrap().push(format!("{} {}", record.level(), record.args()));
        }
    }
    fn flush(&self) {}
}

#[test]
fn a_bad_method_record_is_logged_by_name_and_the_app_keeps_running() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| {
        log::set_logger(&Capture).expect("no other logger in this test binary");
        log::set_max_level(log::LevelFilter::Info);
    });
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin, AiPlugin::default()));
    assert!(!load_library(app.world_mut(), &db(BAD_LIBRARY), &StateKeys::new(), &operators()));
    let logged = CAPTURED.lock().unwrap().clone();
    let errors: Vec<&str> = logged.iter().map(String::as_str).filter(|l| l.starts_with("ERROR")).collect();
    assert_eq!(
        errors,
        [
            "ERROR HTN library: sf.ai.method `bad_method`: unknown task `no_such_task`",
            "ERROR HTN library: 1 error(s); NPCs plan with an empty library",
        ]
    );
    for _ in 0..3 {
        app.update();
    }
    let lib = app.world().resource::<AiLibrary>();
    assert_eq!((lib.library.task_count(), lib.library.method_count()), (0, 0));
    assert!(info(&run(&mut app, "ai.stats"))[0].ends_with("library 0 tasks, 0 methods, 1 load errors"));
}

#[test]
fn brp_answers_the_synthetic_store_exactly() {
    let mut app = app(LIBRARY);
    let mut no_plan = TraceEntry::new(5, 2);
    no_plan.outcome = Some(PlanOutcome::NoPlan);
    {
        let mut traces = app.world_mut().resource_mut::<AiTraces>();
        assert!(traces.push(B, no_plan.clone()));
        traces.watch(A);
        let mut later = no_plan.clone();
        later.tick = 6;
        assert!(traces.push(A, no_plan));
        assert!(traces.push(A, later));
    }
    app.world_mut().resource_mut::<AiSelected>().0 = Some(A);
    let entry = |tick: u64| {
        json!({
            "tick": tick, "tier": 2, "trigger": "scheduled", "directive": null, "drives": [], "drive": null,
            "methods": [], "rejected": [], "plan": {"steps": [], "cursor": 0, "partial": false},
            "operator": null, "path": "none", "budget_us": null, "mtr": [], "outcome": "no_plan",
        })
    };
    assert_eq!(
        brp_trace(app.world(), None).unwrap(),
        json!({
            "enabled": true,
            "cap": 256,
            "selected": A_HEX,
            "actors": [
                {"actor": A_HEX, "watched": true, "entries": [entry(5), entry(6)]},
                {"actor": "ffffffffffffffffffffffffffffffff", "watched": false, "entries": [entry(5)]},
            ],
        })
    );
    // Through a one-shot system, as BRP runs it, with both params.
    let id = app.world_mut().register_system(brp_trace_system);
    let answer = app.world_mut().run_system_with(id, Some(json!({"actor": "7", "last": 1}))).unwrap().unwrap();
    assert_eq!(
        answer["actors"],
        json!([{"actor": A_HEX, "watched": true, "entries": [entry(6)]}])
    );
    let bad = app.world_mut().run_system_with(id, Some(json!({"actors": "7"}))).unwrap();
    assert!(bad.unwrap_err().starts_with("sf_ai/trace: unknown field `actors`"));
}

#[test]
fn actor_keys_round_trip_as_hex() {
    assert_eq!(parse_actor("0x2A"), Some(ActorKey(42)));
    assert_eq!(parse_actor(&actor_hex(B)), Some(B));
    assert_eq!(parse_actor(""), None);
    assert_eq!(parse_actor(&"1".repeat(33)), None);
    assert_eq!(parse_actor("g"), None);
}
