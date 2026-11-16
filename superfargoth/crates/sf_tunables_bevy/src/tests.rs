use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use serde_json::json;
use sf_tunables::{AnyDef, Def, Unit, def};

use crate::overrides::{Override, apply_overrides, parse_file, parse_set};
use crate::{
    PeelTunable, SetBy, SetTunable, TunableChanged, TunableRefused, Tunables, TunablesAppExt,
    TunablesPlugin, brp, tunable_changed,
};

const FADE: Def<f32> = def!(float "test.lamp.fade_start_m" = 15.0)
    .range(0.0, 500.0)
    .unit(Unit::Meters)
    .help("Lamp shadows at full strength within this distance");
const COUNT: Def<u32> = def!(count "test.lamp.count" = 4).range(0, 16);
const CONTACT: Def<bool> = def!(bool "test.contact" = true);
const TUNABLES: &[&dyn AnyDef] = &[&FADE, &COUNT, &CONTACT];

/// The settings resource the consumer systems already read.
#[derive(Resource, Default)]
struct Budget {
    fade_start: f32,
    count: u32,
}

/// How many frames saw `Budget` changed (Bevy change detection, in `Update`).
#[derive(Resource, Default)]
struct BudgetChanges(Vec<f32>);

#[derive(Component, Default)]
struct Contact(bool);

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin))
        .register_tunables(TUNABLES)
        .init_resource::<Budget>()
        .init_resource::<BudgetChanges>();
    let _ = app
        .add_tunable(&FADE)
        .bind_resource(|b: &mut Budget, v| b.fade_start = v);
    let _ = app
        .add_tunable(&COUNT)
        .bind_resource(|b: &mut Budget, v| b.count = v);
    app.add_systems(
        Update,
        |b: Res<Budget>, mut seen: ResMut<BudgetChanges>| {
            if b.is_changed() {
                seen.0.push(b.fade_start);
            }
        },
    );
    app
}

fn set(app: &mut App, name: &str, value: &str, layer: SetBy) {
    app.world_mut()
        .write_message(SetTunable::new(name, value, layer));
    app.update();
}

fn changes(app: &App) -> Vec<f32> {
    app.world().resource::<BudgetChanges>().0.clone()
}

#[test]
fn a_set_per_layer_reaches_the_bound_resource_once() {
    let mut app = app();
    app.update();
    // The first frame applies the starting value (once).
    assert_eq!(changes(&app), [15.0]);
    app.update();
    app.update();
    assert_eq!(changes(&app), [15.0], "idle frames don't touch the resource");

    // Every layer, weak to strong, reaches the resource the frame it is set, exactly once.
    let layers = [
        (SetBy::Mod, 20.0),
        (SetBy::Preset, 25.0),
        (SetBy::User, 30.0),
        (SetBy::CommandLine, 35.0),
        (SetBy::Code, 40.0),
        (SetBy::Console, 45.0),
    ];
    let mut want = vec![15.0];
    for (layer, v) in layers {
        set(&mut app, FADE.name(), &v.to_string(), layer);
        want.push(v);
        assert_eq!(changes(&app), want, "{layer:?}");
        app.update();
        assert_eq!(changes(&app), want, "{layer:?}: once");
        assert!((app.world().resource::<Budget>().fade_start - v).abs() < 1e-6);
    }
    assert_eq!(app.world().resource::<Budget>().count, 4, "other binding untouched");
}

#[test]
fn an_outranked_set_does_not_change_the_resource_and_a_peel_restores() {
    let mut app = app();
    app.update();
    set(&mut app, FADE.name(), "35", SetBy::CommandLine);
    assert_eq!(changes(&app), [15.0, 35.0]);

    // A weaker layer is stored but changes nothing.
    set(&mut app, FADE.name(), "20", SetBy::Mod);
    app.update();
    assert_eq!(changes(&app), [15.0, 35.0]);
    let t = app.world().resource::<Tunables>();
    let id = t.id(FADE.name()).unwrap();
    assert_eq!(t.var(id).layer(SetBy::Mod), Some(&sf_tunables::Value::Float(20.0)));

    // Peeling the command line reveals the mod's value.
    app.world_mut().write_message(PeelTunable {
        name: FADE.name().into(),
        layer: Some(SetBy::CommandLine),
    });
    app.update();
    assert_eq!(changes(&app), [15.0, 35.0, 20.0]);
    assert!((app.world().resource::<Budget>().fade_start - 20.0).abs() < 1e-6);

    // Peeling everything goes back to the default.
    app.world_mut().write_message(PeelTunable {
        name: FADE.name().into(),
        layer: None,
    });
    app.update();
    assert_eq!(changes(&app), [15.0, 35.0, 20.0, 15.0]);
}

#[test]
fn refused_requests_change_nothing_and_are_reported() {
    let mut app = app();
    app.update();
    set(&mut app, FADE.name(), "900", SetBy::Console);
    set(&mut app, FADE.name(), "far", SetBy::Console);
    set(&mut app, "test.lamp.fade_strat_m", "20", SetBy::Console);
    set(&mut app, FADE.name(), "20", SetBy::Default);
    let refused: Vec<TunableRefused> = app
        .world_mut()
        .resource_mut::<Messages<TunableRefused>>()
        .drain()
        .collect();
    assert_eq!(refused.len(), 4, "{refused:?}");
    assert_eq!(refused[2].reason, "no such tunable");
    assert_eq!(changes(&app), [15.0]);
}

#[test]
fn changes_go_out_as_messages_and_run_the_condition_once() {
    #[derive(Resource, Default)]
    struct Ran(u32);
    let mut app = app();
    let contact = app.add_tunable(&CONTACT).id();
    app.init_resource::<Ran>().add_systems(
        Update,
        (|mut r: ResMut<Ran>| r.0 += 1).run_if(tunable_changed(contact)),
    );
    app.update();
    app.update();
    assert_eq!(app.world().resource::<Ran>().0, 1, "first run only");
    set(&mut app, CONTACT.name(), "off", SetBy::Console);
    app.update();
    assert_eq!(app.world().resource::<Ran>().0, 2);
    let names: Vec<String> = app
        .world_mut()
        .resource_mut::<Messages<TunableChanged>>()
        .drain()
        .map(|m| m.name)
        .collect();
    assert_eq!(names, ["test.contact"]);
    assert!(!app.world().resource::<Tunables>().read(contact));
}

#[test]
fn component_bindings_reach_existing_and_new_components() {
    let mut app = app();
    let _ = app
        .add_tunable(&CONTACT)
        .bind_component::<Contact, ()>(|c: &mut Contact, v| c.0 = v);
    let a = app.world_mut().spawn(Contact(false)).id();
    app.update();
    assert!(app.world().get::<Contact>(a).unwrap().0, "starting value");
    set(&mut app, CONTACT.name(), "off", SetBy::Console);
    assert!(!app.world().get::<Contact>(a).unwrap().0);
    let b = app.world_mut().spawn(Contact(true)).id();
    app.update();
    assert!(!app.world().get::<Contact>(b).unwrap().0, "a new one gets the value");
}

#[test]
fn registering_twice_is_the_same_tunable() {
    let mut app = App::new();
    app.register_tunables(TUNABLES).register_tunables(TUNABLES);
    let id = app.add_tunable(&FADE).id();
    let t = app.world().resource::<Tunables>();
    assert_eq!(t.len(), 3);
    assert!((t.read(id) - 15.0).abs() < f32::EPSILON);
}

#[test]
#[should_panic(expected = "registered twice with different kinds or defaults")]
fn the_same_name_with_another_default_panics() {
    const OTHER: Def<f32> = def!(float "test.lamp.fade_start_m" = 16.0).range(0.0, 500.0);
    let mut app = App::new();
    app.register_tunables(TUNABLES);
    let _ = app.add_tunable(&OTHER);
}

#[test]
fn brp_set_validates_range_and_kind() {
    let mut t = sf_tunables::Tunables::default();
    t.register_all(TUNABLES).unwrap();

    let bad = |t: &mut sf_tunables::Tunables, params| brp::set(t, Some(&params)).unwrap_err();
    assert_eq!(
        bad(&mut t, json!({"name": FADE.name(), "value": 900})).code,
        brp::INVALID_PARAMS
    );
    assert!(bad(&mut t, json!({"name": FADE.name(), "value": "far"})).message.contains(FADE.name()));
    assert_eq!(bad(&mut t, json!({"name": COUNT.name(), "value": 2.5})).code, brp::INVALID_PARAMS);
    assert!(bad(&mut t, json!({"name": "test.nope", "value": 1})).message.contains("no such"));
    assert!(bad(&mut t, json!({"name": FADE.name()})).message.contains("value"));
    assert!(t.iter().all(|(_, v)| v.set_by() == SetBy::Default), "nothing changed");

    let ok = brp::set(&mut t, Some(&json!({"name": FADE.name(), "value": 30}))).unwrap();
    assert_eq!(ok["outcome"], "changed");
    assert_eq!(ok["tunable"]["value"], "30");
    assert_eq!(ok["tunable"]["set_by"], "console");
    let ok = brp::set(&mut t, Some(&json!({"name": CONTACT.name(), "value": false}))).unwrap();
    assert_eq!(ok["tunable"]["value"], "off");

    // Console is the strongest layer, so nothing outranks it; reset peels it, `all` clears all.
    let id = t.id(FADE.name()).unwrap();
    t.set_by(id, sf_tunables::Value::Float(35.0), SetBy::CommandLine).unwrap();
    let row = brp::reset(&mut t, Some(&json!({"name": FADE.name()}))).unwrap();
    assert_eq!((row["value"].as_str(), row["set_by"].as_str()), (Some("35"), Some("command line")));
    let row = brp::reset(&mut t, Some(&json!({"name": FADE.name(), "all": true}))).unwrap();
    assert_eq!(row["value"], "15");

    let got = brp::get(&t, Some(&json!({"name": "TEST.LAMP.COUNT"}))).unwrap();
    assert_eq!((got["value"].as_str(), got["unit"].as_str()), (Some("4"), Some("none")));
    let all = brp::list(&t, None).unwrap();
    assert_eq!(all["tunables"].as_array().unwrap().len(), 3);
    let lamp = brp::list(&t, Some(&json!({"filter": "lamp"}))).unwrap();
    assert_eq!(lamp["tunables"].as_array().unwrap().len(), 2);
    let changed = brp::list(&t, Some(&json!({"changed": true}))).unwrap();
    assert_eq!(changed["tunables"][0]["name"], CONTACT.name());
    assert!(brp::list(&t, Some(&json!({"filter": 3}))).is_err());
}

#[test]
fn set_args_and_tunables_files_parse() {
    assert_eq!(
        parse_set(" Test.Lamp.Count = 8 ").unwrap(),
        Override { name: "test.lamp.count".into(), value: "8".into() }
    );
    assert_eq!(parse_set("a=b=c").unwrap().value, "b=c");
    assert!(parse_set("test.lamp.count").is_err());
    assert!(parse_set("=3").is_err());
    assert!(parse_set("x=").is_err());

    let file = r#"
        "test.lamp.fade_start_m" = 35.5
        test.contact = false
        [test.lamp]
        count = 8
        [render]
        pair = [1, 2.5]
        mode = "high"
    "#;
    let o = parse_file(file).unwrap();
    let pairs: Vec<(&str, &str)> = o.iter().map(|o| (o.name.as_str(), o.value.as_str())).collect();
    assert_eq!(
        pairs,
        [
            ("render.mode", "high"),
            ("render.pair", "1,2.5"),
            ("test.contact", "off"),
            ("test.lamp.count", "8"),
            ("test.lamp.fade_start_m", "35.5"),
        ]
    );
    assert!(parse_file("x = [\"a\"]").is_err());
    assert!(parse_file("x = ").is_err());
}

#[test]
fn overrides_apply_on_their_layer_and_refuse_unknown_names() {
    let mut t = sf_tunables::Tunables::default();
    t.register_all(TUNABLES).unwrap();
    let o = [
        parse_set("test.lamp.count=8").unwrap(),
        parse_set("test.lamp.cuont=8").unwrap(),
        parse_set("test.lamp.fade_start_m=900").unwrap(),
    ];
    let refused = apply_overrides(&mut t, &o, SetBy::CommandLine, Some("--set"));
    assert_eq!(refused.len(), 2);
    assert_eq!(refused[0].to_string(), "test.lamp.cuont: no such tunable");
    let id = t.id(COUNT.name()).unwrap();
    assert_eq!(t.var(id).set_by(), SetBy::CommandLine);
    assert_eq!(t.var(id).note(SetBy::CommandLine), Some("--set"));
    assert_eq!(t.int(id), 8);
}
