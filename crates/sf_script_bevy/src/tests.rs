//! Headless integration tests (scripting.md "Verification": integration).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::Time;
use sf_diag_bevy::DiagCounters;
use sf_script::{ContextConfig, FaultPolicy, Handle, ScriptRuntime, ScriptSource, StateValue, Status, storage_value};

use crate::{
    ActivationOutcome, Activated, PLAYER, Placed, ScriptDiagnostics, ScriptFact, ScriptPlacements, ScriptSlot, Scripts,
    ScriptsPlugin,
};

struct MemSource(HashMap<String, String>);

impl ScriptSource for MemSource {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get(path).map(|s| s.as_bytes().to_vec())
    }
}

const ALPHA: &str = r#"
local core = require("@sf/core")
local events = require("@sf/events")
local timers = require("@sf/timers")
local world = require("@sf/world")
local s = core.storage()
s.activations = s.activations or 0
s.dings = s.dings or 0
s.order = s.order or ""
timers.register("ding", function(p)
    s.dings += 1
    s.ding_record = world.record(p.h)
end)
events.on("input_action", function(e)
    s.order ..= "i"
    s.last_action = e.action
end)
events.on("update", function(dt)
    s.order ..= "u"
end)
events.on("activated", function(e)
    s.activations += 1
    s.last_record = world.record(e.target)
    s.by_player = e.actor == world.player()
    s.loaded = world.is_loaded(e.target)
    local x, y, z = world.position(e.target)
    s.x = x
    timers.after(0.25, "ding", { h = e.target })
    if world.record(e.target) == "chest" then
        return "handled"
    end
    return nil
end)
"#;

const BAD: &str = r#"
local events = require("@sf/events")
events.on("activated", function(e)
    error("boom")
end)
"#;

const CHEST: Handle = Handle(1);
const DOOR: Handle = Handle(2);

/// Activations to send in the next frame (before the `Update` slot).
#[derive(Resource, Default)]
struct Pending(Vec<Activated>);

#[derive(Resource, Default)]
struct Seen {
    outcomes: Vec<ActivationOutcome>,
    diagnostics: Vec<String>,
    /// `storage.order` right after the `Input` slot, per frame.
    order_after_input: Vec<String>,
}

fn send(mut pending: ResMut<Pending>, mut out: MessageWriter<Activated>) {
    for a in pending.0.drain(..) {
        out.write(a);
    }
}

fn collect(mut seen: ResMut<Seen>, mut outcomes: MessageReader<ActivationOutcome>, mut diags: MessageReader<ScriptDiagnostics>) {
    seen.outcomes.extend(outcomes.read().copied());
    seen.diagnostics.extend(diags.read().flat_map(|d| d.0.iter().map(|d| format!("{}: {}", d.context, d.message))));
}

fn text(scripts: &Scripts, ctx: &str, key: &str) -> Option<String> {
    match storage_value(scripts.runtime.context(ctx)?, key)? {
        StateValue::Str(b) => Some(String::from_utf8_lossy(&b).into_owned()),
        _ => None,
    }
}

fn number(scripts: &Scripts, ctx: &str, key: &str) -> Option<f64> {
    match storage_value(scripts.runtime.context(ctx)?, key)? {
        StateValue::Number(n) => Some(n),
        _ => None,
    }
}

fn after_input(scripts: Res<Scripts>, mut seen: ResMut<Seen>) {
    let order = text(&scripts, "alpha", "order").unwrap_or_default();
    seen.order_after_input.push(order);
}

fn app() -> App {
    let source = MemSource(HashMap::from([
        ("scripts/alpha/main.luau".to_owned(), ALPHA.to_owned()),
        ("scripts/bad/main.luau".to_owned(), BAD.to_owned()),
    ]));
    let mut runtime = ScriptRuntime::default();
    runtime.set_policy(FaultPolicy { max_errors: 3, ..FaultPolicy::default() });
    let mut scripts = Scripts::new(runtime);
    // `bad` loads first: its faults must not stop `alpha` (load order is dispatch order).
    scripts.start(vec![ContextConfig::for_mod("bad"), ContextConfig::for_mod("alpha")], Arc::new(source));
    let mut app = App::new();
    app.insert_resource(scripts)
        .insert_resource(Time::<()>::default())
        .insert_resource(DiagCounters::default())
        .add_plugins(ScriptsPlugin)
        .init_resource::<Pending>()
        .init_resource::<Seen>()
        .add_systems(Update, send.before(ScriptSlot::Update))
        .add_systems(Update, collect.after(ScriptSlot::Update))
        .add_systems(PreUpdate, after_input.after(ScriptSlot::Input));
    let placed = |record: &str, x: f64| Placed { record: record.into(), worldspace: "test".into(), position: [x, 0.0, 0.0], radius: 0.5 };
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    p.loaded.insert(CHEST, placed("chest", 3.0));
    p.loaded.insert(DOOR, placed("door", 7.0));
    p.player = Some(placed("player", 0.0));
    app
}

fn frame(app: &mut App) {
    app.world_mut().resource_mut::<Time>().advance_by(Duration::from_millis(100));
    app.update();
}

#[test]
fn activation_reaches_the_target_and_handled_suppresses_the_default() {
    let mut app = app();
    app.world_mut().resource_mut::<Pending>().0.push(Activated { target: CHEST, actor: PLAYER });
    frame(&mut app);
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.outcomes, vec![ActivationOutcome { activation: Activated { target: CHEST, actor: PLAYER }, handled: true }]);
    let scripts = app.world().resource::<Scripts>();
    assert_eq!(text(scripts, "alpha", "last_record").as_deref(), Some("chest"));
    assert_eq!(number(scripts, "alpha", "x"), Some(3.0));
    let ctx = scripts.runtime.context("alpha").unwrap();
    assert_eq!(storage_value(ctx, "by_player"), Some(StateValue::Bool(true)));
    assert_eq!(storage_value(ctx, "loaded"), Some(StateValue::Bool(true)));

    // A door: alpha doesn't claim it, so the default action runs.
    app.world_mut().resource_mut::<Pending>().0.push(Activated { target: DOOR, actor: PLAYER });
    frame(&mut app);
    let seen = app.world().resource::<Seen>();
    assert_eq!(seen.outcomes.len(), 2);
    assert!(!seen.outcomes[1].handled);
    assert_eq!(text(app.world().resource::<Scripts>(), "alpha", "last_record").as_deref(), Some("door"));
}

#[test]
fn handlers_run_in_their_slots_and_timers_fire_with_world_access() {
    let mut app = app();
    app.world_mut().resource_mut::<Pending>().0.push(Activated { target: CHEST, actor: PLAYER });
    frame(&mut app); // update slot: "u"; the activation schedules "ding" in 0.25 s
    app.world_mut().write_message(ScriptFact::input_action("jump", true));
    frame(&mut app); // input slot: "i", then update slot: "u"
    let seen = app.world().resource::<Seen>();
    // After frame 1's Input slot nothing ran yet; after frame 2's the input handler had run but
    // not frame 2's update.
    assert_eq!(seen.order_after_input, vec![String::new(), "ui".to_owned()]);
    let scripts = app.world().resource::<Scripts>();
    assert_eq!(text(scripts, "alpha", "order").as_deref(), Some("uiu"));
    assert_eq!(text(scripts, "alpha", "last_action").as_deref(), Some("jump"));
    assert_eq!(number(scripts, "alpha", "dings"), Some(0.0));
    for _ in 0..3 {
        frame(&mut app);
    }
    let scripts = app.world().resource::<Scripts>();
    assert_eq!(number(scripts, "alpha", "dings"), Some(1.0));
    assert_eq!(text(scripts, "alpha", "ding_record").as_deref(), Some("chest"));
    // Counters for sf_diag.
    let counters = app.world().resource::<DiagCounters>();
    for name in ["script/update/ms", "script/input/ms", "script/ctx/alpha/ms", "script/ctx/alpha/calls"] {
        assert!(counters.id(name).is_some(), "{name}");
    }
}

#[test]
fn faults_suspend_one_context_and_surface_as_diagnostics() {
    let mut app = app();
    for _ in 0..4 {
        app.world_mut().resource_mut::<Pending>().0.push(Activated { target: CHEST, actor: PLAYER });
        frame(&mut app);
    }
    let scripts = app.world().resource::<Scripts>();
    assert!(matches!(scripts.runtime.context("bad").unwrap().status(), Status::Suspended { .. }));
    assert_eq!(scripts.stopped().count(), 1);
    // The suspended mod doesn't block the other: all four activations handled.
    assert_eq!(number(scripts, "alpha", "activations"), Some(4.0));
    let seen = app.world().resource::<Seen>();
    assert!(seen.outcomes.iter().all(|o| o.handled));
    let boom = seen.diagnostics.iter().filter(|d| d.starts_with("bad: activated:") && d.contains("boom")).count();
    assert_eq!(boom, 3, "{:?}", seen.diagnostics);
    assert!(seen.diagnostics.iter().any(|d| d.starts_with("bad: suspended")), "{:?}", seen.diagnostics);
    assert!(scripts.recent().any(|d| d.message.starts_with("suspended")));
    let counters = app.world().resource::<DiagCounters>();
    assert!(counters.id("script/ctx/bad/errors").is_some());
}

#[test]
fn world_access_fails_outside_a_slot() {
    let mut app = app();
    frame(&mut app);
    let mut scripts = app.world_mut().resource_mut::<Scripts>();
    let policy = *scripts.runtime.policy();
    let ctx = scripts.runtime.context_mut("alpha").unwrap();
    let r: Option<String> =
        ctx.eval("local w = require('@sf/world'); return w.record(w.player())", &policy);
    assert!(r.is_none());
    assert!(ctx.last_error().is_some_and(|e| e.contains("outside a script slot")), "{:?}", ctx.last_error());
    // The resource went back to the world after the slot.
    assert_eq!(app.world().resource::<ScriptPlacements>().loaded.len(), 2);
}
