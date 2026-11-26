//! A scene's live A/B: what `resolve` accepts, what the key sets and what the line says (exact).

use super::*;
use mw_game_core::scenes::SceneArm;
use sf_tunables_bevy::sf_tunables::Def;
use sf_tunables_bevy::{AnyDef, TunablesAppExt};

const LOOK: Def<bool> = Def::new("test.look", true);
const REACH: Def<f32> = Def::new("test.reach_m", 10.0).range(0.0, 100.0);
const AT_START: Def<bool> = Def::new("test.at_start", true).lifetime(Lifetime::Restart);
const DEFS: &[&dyn AnyDef] = &[&LOOK, &REACH, &AT_START];

fn ab(set: &str, a: &str, b: &str) -> SceneAb {
    let arm = |value: &str, label: &str| SceneArm { value: value.to_owned(), label: label.to_owned() };
    SceneAb { set: set.to_owned(), a: arm(a, "new"), b: arm(b, "stock") }
}

fn store() -> sf_tunables_bevy::sf_tunables::Tunables {
    let mut store = sf_tunables_bevy::sf_tunables::Tunables::default();
    store.register_all(DEFS).expect("defs register");
    store
}

#[test]
fn resolve_parses_both_arms_for_the_tunables_kind() {
    let store = store();
    let arms = resolve(&store, &ab("test.look", "true", "false")).expect("a bool A/B");
    assert_eq!(arms.values, [Value::Bool(true), Value::Bool(false)]);
    let arms = resolve(&store, &ab("test.reach_m", "10", "25.5")).expect("a float A/B");
    assert_eq!(arms.values, [Value::Float(10.0), Value::Float(25.5)]);
    assert_eq!(arms.live(&store), Some(Arm::A), "the default is arm A's value");
}

#[test]
fn resolve_refuses_what_cannot_be_compared_live() {
    let store = store();
    let refused = |ab: SceneAb| resolve(&store, &ab).expect_err("refused");
    assert_eq!(refused(ab("test.nope", "true", "false")), "ab: no such tunable \"test.nope\"");
    assert!(refused(ab("test.look", "true", "maybe")).starts_with("ab: arm B of test.look: "));
    assert!(refused(ab("test.reach_m", "500", "10")).starts_with("ab: arm A of test.reach_m: "), "out of range");
    assert_eq!(refused(ab("test.look", "false", "false")), "ab: both arms set test.look = off");
    assert_eq!(refused(ab("test.at_start", "true", "false")), "ab: test.at_start does not change what is on screen while running (Restart)");
}

fn app(ab: SceneAb) -> App {
    let mut app = App::new();
    app.init_resource::<ButtonInput<KeyCode>>().register_tunables(DEFS).add_plugins(SceneAbPlugin(ab));
    app
}

fn tap(app: &mut App, key: KeyCode) {
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(key);
    app.update();
    let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    input.release(key);
    input.clear();
}

fn look(app: &App) -> (bool, SetBy) {
    let t = app.world().resource::<Tunables>();
    let var = t.var(t.id("test.look").expect("registered"));
    (var.value().as_bool().expect("a bool"), var.set_by())
}

fn shown(app: &mut App) -> String {
    app.world_mut().query_filtered::<&Text, With<ArmText>>().single(app.world()).expect("one arm line").0.clone()
}

/// The run starts on arm A whatever the default is, Tab goes to B and back, and the line follows.
#[test]
fn tab_switches_the_arm_and_the_line_names_it() {
    let mut app = app(ab("test.look", "false", "true"));
    app.update();
    assert_eq!(look(&app), (false, SetBy::Console), "arm A at the start, not the default");
    assert_eq!(shown(&mut app), "A: new   |   Tab: switch A/B");
    tap(&mut app, KEY);
    assert_eq!(look(&app), (true, SetBy::Console));
    assert_eq!(shown(&mut app), "B: stock   |   Tab: switch A/B");
    tap(&mut app, KEY);
    assert!(!look(&app).0);
    assert_eq!(shown(&mut app), "A: new   |   Tab: switch A/B");
    // No press, no switch.
    app.update();
    assert!(!look(&app).0);
}

/// Tab completes a name in the open console: it must not switch the arm under it.
#[test]
fn tab_in_the_open_console_switches_nothing() {
    let mut app = app(ab("test.look", "true", "false"));
    app.insert_resource(sf_console_bevy::ConsoleOpen(true));
    app.update();
    tap(&mut app, KEY);
    assert!(look(&app).0);
}

/// A third value (console, Features panel) reads as neither arm; Tab then goes to A.
#[test]
fn a_third_value_is_neither_arm() {
    let mut app = app(ab("test.reach_m", "10", "20"));
    app.update();
    let mut t = app.world_mut().resource_mut::<Tunables>();
    let id = t.id("test.reach_m").expect("registered");
    t.set_by(id, Value::Float(15.0), SetBy::Console).expect("in range");
    app.update();
    assert_eq!(shown(&mut app), "neither arm: test.reach_m = 15   |   Tab: switch A/B");
    tap(&mut app, KEY);
    assert_eq!(app.world().resource::<Tunables>().get(id), &Value::Float(10.0));
}

/// A scene whose `ab` does not resolve says so on screen instead of showing nothing.
#[test]
fn an_unknown_tunable_is_said_on_screen() {
    let mut app = app(ab("test.nope", "true", "false"));
    app.update();
    assert_eq!(shown(&mut app), "A/B not available (ab: no such tunable \"test.nope\")");
    tap(&mut app, KEY);
}

#[test]
fn the_line_says_when_shaders_compile() {
    let store = store();
    let def = ab("test.look", "true", "false");
    let arms = resolve(&store, &def).expect("resolves");
    assert_eq!(line(&def, &arms, &store, true), "A: new  (compiling shaders)   |   Tab: switch A/B");
}
