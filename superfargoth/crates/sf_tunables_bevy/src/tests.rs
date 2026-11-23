use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use serde_json::json;
use sf_tunables::{AnyDef, Def, Unit, def};

use crate::overrides::{Override, apply_overrides, parse_file, parse_set};
use crate::{
    FeatureSwitch, FeatureSwitches, FeatureToggled, OverrideRefusals, PeelTunable, SetBy,
    SetTunable, TunableChanged, TunableEnv, TunableRefused, Tunables, TunablesAppExt,
    TunablesPlugin, brp, feature_on, tunable_changed,
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

const SSAO_ALIAS: Def<bool> = def!(bool "test.ssao" = true).env_alias("TEST_SSAO");
const RADIUS_ALIAS: Def<f32> = def!(float "test.ai.radius_m" = 60.0)
    .range(0.0, 500.0)
    .env_alias("TEST_AI_RADIUS");

#[test]
fn env_alias_sets_the_command_line_layer_at_registration() {
    let mut app = App::new();
    app.insert_resource(TunableEnv::from_pairs(&[
        ("TEST_SSAO", "0"),
        ("TEST_AI_RADIUS", " 80 "),
    ]));
    app.add_plugins((MinimalPlugins, TunablesPlugin))
        .register_tunables(&[&SSAO_ALIAS]);
    let _ = app.add_tunable(&RADIUS_ALIAS);
    let tunables = app.world().resource::<Tunables>();
    let ssao = tunables.var(tunables.id("test.ssao").unwrap());
    assert_eq!(ssao.value(), &sf_tunables::Value::Bool(false));
    assert_eq!(ssao.set_by(), SetBy::CommandLine);
    assert_eq!(
        ssao.note(SetBy::CommandLine),
        Some("env TEST_SSAO (deprecated: --set test.ssao=value)")
    );
    let radius = tunables.var(tunables.id("test.ai.radius_m").unwrap());
    assert_eq!(radius.value(), &sf_tunables::Value::Float(80.0));
    assert!(app.world().get_resource::<OverrideRefusals>().is_none());
}

#[test]
fn env_alias_unset_or_empty_leaves_the_default() {
    let mut app = App::new();
    app.insert_resource(TunableEnv::from_pairs(&[("TEST_SSAO", "")]));
    app.register_tunables(&[&SSAO_ALIAS, &RADIUS_ALIAS]);
    let tunables = app.world().resource::<Tunables>();
    assert!(tunables.iter().all(|(_, t)| t.set_by() == SetBy::Default));
}

#[test]
fn a_bad_env_alias_value_is_refused() {
    let mut app = App::new();
    app.insert_resource(TunableEnv::from_pairs(&[
        ("TEST_SSAO", "maybe"),
        ("TEST_AI_RADIUS", "9000"),
    ]));
    app.add_plugins((MinimalPlugins, TunablesPlugin))
        .register_tunables(&[&SSAO_ALIAS, &RADIUS_ALIAS]);
    let refused = &app.world().resource::<OverrideRefusals>().0;
    let names: Vec<&str> = refused.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["test.ssao", "test.ai.radius_m"]);
    assert!(refused[0].reason.starts_with("env TEST_SSAO=maybe: "), "{}", refused[0]);
    let tunables = app.world().resource::<Tunables>();
    assert!(tunables.iter().all(|(_, t)| t.set_by() == SetBy::Default));
}

#[test]
fn command_line_overrides_apply_after_every_plugin_registered() {
    use crate::CommandLineOverrides;
    use crate::overrides::OverrideSet;
    let set = |origin: &str, list: &[&str]| OverrideSet {
        origin: origin.to_owned(),
        overrides: list.iter().map(|a| parse_set(a).unwrap()).collect(),
    };
    let mut app = App::new();
    // Added before the plugin that registers the tunables: they apply in `finish`.
    app.add_plugins((
        MinimalPlugins,
        TunablesPlugin,
        CommandLineOverrides(vec![
            set("file a.toml", &["test.lamp.count=2", "test.contact=off"]),
            set("--set", &["test.lamp.count=6", "test.lamp.cuont=3"]),
        ]),
    ));
    app.register_tunables(TUNABLES);
    app.finish();
    let tunables = app.world().resource::<Tunables>();
    let count = tunables.var(tunables.id("test.lamp.count").unwrap());
    assert_eq!(count.value(), &sf_tunables::Value::Int(6));
    assert_eq!(count.set_by(), SetBy::CommandLine);
    assert_eq!(count.note(SetBy::CommandLine), Some("--set"));
    let contact = tunables.var(tunables.id("test.contact").unwrap());
    assert_eq!(contact.note(SetBy::CommandLine), Some("file a.toml"));
    let refused = &app.world().resource::<OverrideRefusals>().0;
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].name, "test.lamp.cuont");
    assert_eq!(refused[0].reason, "no such tunable (--set)");
}

/// T5b: a driver switch (console layer) applies its level before the bindings run: the bound
/// resource has the level's value after one update; the driver shows the level again.
#[test]
fn a_driver_switch_reaches_the_bound_resource_the_same_frame() {
    use sf_tunables::presets::{MergedPresets, PresetBook, PresetRecord, RecordValue};
    let mut app = app();
    app.add_plugins(crate::PresetsPlugin);
    let record = PresetRecord {
        driver: "sg.shadows".into(),
        levels: [
            ("low".to_owned(), [("test.lamp.count".to_owned(), RecordValue::Int(0))].into()),
            ("high".to_owned(), [("test.lamp.count".to_owned(), RecordValue::Int(8))].into()),
        ]
        .into(),
        ..PresetRecord::default()
    };
    let merged = MergedPresets::merge([(&record, "base")], &[]);
    let (book, issues) = PresetBook::build(&app.world().resource::<Tunables>().0, &merged);
    assert!(issues.is_empty(), "{issues:?}");
    app.insert_resource(crate::Presets(book));
    app.update();
    assert_eq!(app.world().resource::<Budget>().count, 4, "no level yet: the default");
    app.world_mut().write_message(SetTunable::new("sg.shadows", "low", SetBy::Console));
    app.update();
    assert_eq!(app.world().resource::<Budget>().count, 0, "low, the same frame");
    app.world_mut().write_message(SetTunable::new("sg.shadows", "high", SetBy::Console));
    app.update();
    assert_eq!(app.world().resource::<Budget>().count, 8, "high");
    let t = app.world().resource::<Tunables>();
    let driver = t.var(t.id("sg.shadows").expect("driver"));
    assert_eq!((driver.value(), driver.set_by()), (&sf_tunables::Value::Str("high".into()), SetBy::Preset));
    let applied: Vec<_> = app.world_mut().resource_mut::<Messages<crate::PresetApplied>>().drain().collect();
    assert_eq!(applied.last().map(|a| (a.level.as_str(), a.by)), Some(("high", SetBy::Console)));
    app.world_mut().write_message(SetTunable::new("test.lamp.count", "3", SetBy::Console));
    app.update();
    app.update();
    assert_eq!(app.world().resource::<Tunables>().text(app.world().resource::<Tunables>().id("sg.shadows").expect("driver")), "custom");
}

/// T5b: `sf.tunable` and `sf.tunable_preset` records from a record store, in load order; the values
/// land at the `Mod` layer.
#[test]
fn record_kinds_apply_at_the_mod_layer_in_load_order() {
    use sf_records::{RecordDb, RecordFormat, apply_record_file};
    let mut db = RecordDb::new();
    crate::presets::register_kinds(&mut db).expect("kinds");
    let base = db.add_source("base", &[]);
    let later = db.add_source("later", &[base]);
    let text = r#"[
  define(kind: "sf.tunable", id: "test.lamp.count", value: (value: 6, source: "measured: x")),
  define(kind: "sf.tunable", id: "test.contact", value: (value: false)),
  define(kind: "sf.tunable_preset", id: "base/shadows", value: (driver: "sg.shadows", levels: { "low": { "test.lamp.count": 0 } })),
]"#;
    let outcome = apply_record_file(&mut db, base, text, RecordFormat::Ron).expect("parses");
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    let text = r#"[ define(kind: "sf.tunable", id: "test.lamp.fade_start_m", value: (value: 35.0)),
  define(kind: "sf.tunable", id: "test.lamp.count", value: (value: 2)) ]"#;
    let outcome = apply_record_file(&mut db, later, text, RecordFormat::Ron).expect("parses");
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    let values = crate::presets::mod_values(&db);
    let mut app = app();
    let mut t = app.world_mut().resource_mut::<Tunables>();
    let refused = sf_tunables::presets::apply_mod_values(&mut t, values.iter().map(|(n, r, m)| (n.as_str(), r, m.as_str())));
    assert!(refused.is_empty(), "{refused:?}");
    let count = t.id("test.lamp.count").expect("count");
    assert_eq!((t.int(count), t.var(count).set_by()), (2, SetBy::Mod), "the later mod wins");
    assert_eq!(t.var(count).note(SetBy::Mod), Some("mod later: guess"));
    assert!(!t.flag(t.id("test.contact").expect("contact")));
    let presets = crate::presets::preset_records(&db);
    assert_eq!(presets.len(), 1);
    assert_eq!((presets[0].0.driver.as_str(), presets[0].1.as_str()), ("sg.shadows", "base"));
    app.update();
    assert!((app.world().resource::<Budget>().fade_start - 35.0).abs() < f32::EPSILON, "bound from the mod layer");
}

const FOG_ON: Def<bool> = def!(bool "test.fog.enabled" = true);
const FROXELS_ON: Def<bool> = def!(bool "test.froxels.enabled" = true);
static FOG_SW: FeatureSwitch = FeatureSwitch::new(&FOG_ON);
static FROXELS_SW: FeatureSwitch = FeatureSwitch::new(&FROXELS_ON).requires(&["test.fog.enabled"]);

/// How many frames the froxel system ran.
#[derive(Resource, Default)]
struct FroxelRuns(u32);

fn feature_app(not_built: Option<&str>) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TunablesPlugin))
        .register_tunables(&[&FOG_ON, &FROXELS_ON])
        .add_feature_switch(&FROXELS_SW) // requires resolve lazily: any order
        .add_feature_switch(&FOG_SW)
        .add_feature_switch(&FOG_SW) // the same switch twice is one switch
        .init_resource::<FroxelRuns>()
        .add_systems(
            Update,
            (|mut r: ResMut<FroxelRuns>| r.0 += 1).run_if(feature_on("test.froxels.enabled")),
        );
    if let Some(name) = not_built {
        app.mark_feature_not_built(name, "test");
    }
    app
}

fn runs(app: &App) -> u32 {
    app.world().resource::<FroxelRuns>().0
}

fn toggles(app: &mut App) -> Vec<(&'static str, bool)> {
    app.world_mut()
        .resource_mut::<Messages<FeatureToggled>>()
        .drain()
        .map(|m| (m.name, m.running))
        .collect()
}

#[test]
fn feature_on_follows_the_console_layer_and_toggles_go_out_once() {
    let mut app = feature_app(None);
    assert_eq!(app.world().resource::<FeatureSwitches>().len(), 2);
    app.update();
    app.update();
    assert_eq!(runs(&app), 2);
    assert!(toggles(&mut app).is_empty(), "starting states are not toggles");

    set(&mut app, "test.froxels.enabled", "off", SetBy::Console);
    assert_eq!(runs(&app), 2, "stops the frame it is set");
    assert_eq!(toggles(&mut app), [("test.froxels.enabled", false)]);

    // Same value again, and an outranked set: the effective value doesn't change.
    set(&mut app, "test.froxels.enabled", "off", SetBy::Console);
    set(&mut app, "test.froxels.enabled", "on", SetBy::Mod);
    assert_eq!(runs(&app), 2);
    assert!(toggles(&mut app).is_empty());

    set(&mut app, "test.froxels.enabled", "on", SetBy::Console);
    assert_eq!(runs(&app), 3);
    assert_eq!(toggles(&mut app), [("test.froxels.enabled", true)]);

    // The required switch goes off: the dependent one stops and reports too.
    set(&mut app, "test.fog.enabled", "off", SetBy::Console);
    assert_eq!(runs(&app), 3);
    assert_eq!(
        toggles(&mut app),
        [("test.froxels.enabled", false), ("test.fog.enabled", false)]
    );
    app.world_mut()
        .write_message(PeelTunable { name: "test.fog.enabled".into(), layer: None });
    app.update();
    assert_eq!(runs(&app), 4);
    assert_eq!(
        toggles(&mut app),
        [("test.froxels.enabled", true), ("test.fog.enabled", true)]
    );
    app.update();
    assert!(toggles(&mut app).is_empty(), "idle frames send nothing");
}

#[test]
fn a_feature_not_built_never_runs_and_a_run_time_mark_reports() {
    let mut app = feature_app(Some("test.fog.enabled"));
    app.update();
    assert_eq!(runs(&app), 0, "blocked by the required switch from the first frame");
    set(&mut app, "test.froxels.enabled", "off", SetBy::Console);
    set(&mut app, "test.froxels.enabled", "on", SetBy::Console);
    set(&mut app, "test.fog.enabled", "off", SetBy::Console);
    assert_eq!(runs(&app), 0);
    assert!(toggles(&mut app).is_empty(), "not built: no tunable set changes the running state");

    let mut app = feature_app(None);
    app.update();
    app.world_mut()
        .resource_mut::<FeatureSwitches>()
        .mark_not_built("test.froxels.enabled", "device lost")
        .unwrap();
    app.update();
    assert_eq!(runs(&app), 1);
    assert_eq!(toggles(&mut app), [("test.froxels.enabled", false)]);
}

#[test]
#[should_panic(expected = "no tunable of that name")]
fn a_switch_without_its_tunable_panics() {
    static ORPHAN: FeatureSwitch = FeatureSwitch::new(&CONTACT);
    let mut app = App::new();
    app.add_plugins(TunablesPlugin).add_feature_switch(&ORPHAN);
}
