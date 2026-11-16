use crate::{
    AnyDef, ArgKind, Def, Group, Lifetime, Net, RegisterError, Scope, SetBy, SetOutcome, Source,
    Spec, Tunables, Unit, Value, def,
};

const FADE: Def<f32> = def!(float "shadow.lamp.fade_start_m" = 15.0)
    .range(0.0, 500.0)
    .unit(Unit::Meters)
    .help("Lamp shadows at full strength within this distance")
    .source(Source::Industry("UE local light fade"))
    .group(Group::Shadows)
    .scope(Scope::Player);
const FADE_RANGE: Def<[f32; 2]> =
    def!(float2 "shadow.lamp.fade_m" = [15.0, 25.0]).range([0.0, 0.0], [500.0, 500.0]);
const LIGHTS: Def<u32> = def!(count "shadow.lamp.count" = 4)
    .range(0, 42)
    .unit(Unit::Count)
    .source(Source::Measured("docs/ledger/shadows.md 2026-11-12"));
const FILTER: Def<&str> = def!(choice "shadow.filter" = "pcf").choices(&["hard", "pcf", "pcss"]);
const CONTACT: Def<bool> = def!(bool "shadow.contact" = true).lifetime(Lifetime::Recompile);
const AI_RADIUS: Def<f32> = def!(float "ai.wake_radius_m" = 0.1)
    .range(0.0, 1000.0)
    .net(Net::Gameplay);
const TUNABLES: &[&dyn AnyDef] = &[&FADE, &FADE_RANGE, &LIGHTS, &FILTER, &CONTACT, &AI_RADIUS];
// Not registered by `store()`: refused or mismatched.
const WRONG: Def<bool> = def!(bool "shadow.lamp.fade_start_m" = false);
const MISSING: Def<bool> = def!(bool "nope" = false);
const NO_RANGE: Def<f32> = def!(float "x.no_range" = 1.0);
const OUTSIDE: Def<u32> = def!(count "x.outside" = 50).range(0, 42);
const BAD_CHOICE: Def<&str> = def!(choice "x.choice" = "ultra").choices(&["low", "high"]);

fn store() -> Tunables {
    let mut t = Tunables::default();
    t.register_all(TUNABLES).unwrap();
    t
}

#[test]
#[allow(clippy::float_cmp)] // exact on purpose: defaults read back bit for bit
fn defs_register_and_read_typed() {
    let mut t = Tunables::default();
    let fade = t.register_def(&FADE).unwrap();
    let pair = t.register_def(&FADE_RANGE).unwrap();
    let lights = t.register_def(&LIGHTS).unwrap();
    let filter = t.register_def(&FILTER).unwrap();
    let contact = t.register_def(&CONTACT).unwrap();
    let radius = t.register_def(&AI_RADIUS).unwrap();
    assert!((t.read(fade) - 15.0).abs() < f32::EPSILON);
    assert_eq!(t.read(pair), [15.0, 25.0]);
    assert_eq!(t.read(lights), 4);
    assert_eq!(t.read(filter), "pcf");
    assert!(t.read(contact));
    // f32 defaults go through their shortest decimal form.
    assert_eq!(t.get(radius.untyped()), &Value::Float(0.1));
    assert!((t.read(radius) - 0.1).abs() < f32::EPSILON);
    let var = t.var(fade.untyped());
    assert_eq!(var.meta().unit, Unit::Meters);
    assert_eq!(var.meta().group, Some(Group::Shadows));
    assert_eq!(
        var.kind(),
        &ArgKind::Float {
            min: 0.0,
            max: 500.0
        }
    );
    let (file, line) = var.meta().location.unwrap();
    assert!(file.ends_with("tests.rs") && line > 0, "{file}:{line}");
    assert_eq!(
        var.describe(),
        "shadow.lamp.fade_start_m = 15 [float 0..=500 m]: Lamp shadows at full strength within this distance"
    );
    // Another crate gets the typed handle by def; the kind must match.
    assert_eq!(t.id_of(&FADE), Ok(fade));
    assert!(matches!(
        t.id_of(&WRONG),
        Err(RegisterError::WrongKind { .. })
    ));
    assert_eq!(
        t.id_of(&MISSING),
        Err(RegisterError::Unknown("nope".into()))
    );
}

#[test]
fn registration_is_validated() {
    let mut t = store();
    assert_eq!(
        t.register_def(&FADE),
        Err(RegisterError::Duplicate("shadow.lamp.fade_start_m".into()))
    );
    assert_eq!(
        t.register_def(&NO_RANGE),
        Err(RegisterError::NoRange("x.no_range".into()))
    );
    assert!(matches!(
        t.register_def(&OUTSIDE),
        Err(RegisterError::BadDefault { .. })
    ));
    assert!(matches!(
        t.register_def(&BAD_CHOICE),
        Err(RegisterError::BadDefault { .. })
    ));
    assert!(matches!(
        t.register(Spec::bool("9lives", true)),
        Err(RegisterError::InvalidName(_))
    ));
    // Names are case-insensitive.
    let id = t.register(Spec::bool("X.Mixed", true)).unwrap();
    assert_eq!(t.id("x.MIXED"), Some(id));
    assert_eq!(t.var(id).name(), "x.mixed");
}

#[test]
fn layers_set_and_peel() {
    let mut t = store();
    let fade = t.id_of(&FADE).unwrap();
    let id = fade.untyped();
    // Weak to strong: each stronger layer takes effect.
    for (layer, v) in [
        (SetBy::Mod, 35.0),
        (SetBy::Preset, 30.0),
        (SetBy::User, 20.0),
        (SetBy::CommandLine, 10.0),
        (SetBy::Code, 5.0),
        (SetBy::Console, 1.0),
    ] {
        assert_eq!(t.write(fade, v, layer), Ok(SetOutcome::Changed));
        assert_eq!(t.var(id).set_by(), layer);
        assert!((t.read(fade) - v).abs() < f32::EPSILON);
    }
    // A weaker layer is stored but doesn't take effect.
    assert_eq!(
        t.write(fade, 31.0, SetBy::Preset),
        Ok(SetOutcome::Outranked(SetBy::Console))
    );
    assert_eq!(t.var(id).layer(SetBy::Preset), Some(&Value::Float(31.0)));
    // Peeling uncovers the next layer down, one at a time.
    for (layer, below) in [
        (SetBy::Console, 5.0),
        (SetBy::Code, 10.0),
        (SetBy::CommandLine, 20.0),
        (SetBy::User, 31.0),
        (SetBy::Preset, 35.0),
        (SetBy::Mod, 15.0),
    ] {
        assert!(t.clear(id, layer));
        assert!(
            (t.read(fade) - below).abs() < f32::EPSILON,
            "after peeling {layer:?}"
        );
    }
    assert_eq!(t.var(id).set_by(), SetBy::Default);
    assert!(!t.clear(id, SetBy::Default), "the default can't be peeled");
    assert!(!t.clear(id, SetBy::Console), "nothing left to peel");
    // A middle layer cleared under a stronger one changes nothing visible.
    t.write(fade, 40.0, SetBy::Mod).unwrap();
    t.write(fade, 50.0, SetBy::Console).unwrap();
    assert!(!t.clear(id, SetBy::Mod));
    // `reset` clears every layer; `clear_layer` one layer on every tunable.
    assert!(t.reset(id));
    assert_eq!(t.var(id).layers().count(), 1, "only the default is left");
    let lights = t.id_of(&LIGHTS).unwrap();
    t.write(fade, 99.0, SetBy::Preset).unwrap();
    t.write(lights, 8, SetBy::Preset).unwrap();
    t.write(lights, 2, SetBy::User).unwrap();
    assert_eq!(
        t.clear_layer(SetBy::Preset),
        1,
        "lights stays on its user value"
    );
    assert_eq!(t.read(lights), 2);
    assert!(t.set_by(id, Value::Float(1.0), SetBy::Default).is_err());
}

#[test]
fn sets_are_validated() {
    let mut t = store();
    let fade = t.id("shadow.lamp.fade_start_m").unwrap();
    let pair = t.id("shadow.lamp.fade_m").unwrap();
    let lights = t.id("shadow.lamp.count").unwrap();
    let filter = t.id("shadow.filter").unwrap();
    let rev = t.revision();
    assert!(t.set_by(fade, Value::Float(501.0), SetBy::Console).is_err());
    assert!(
        t.set_by(fade, Value::Float(f64::NAN), SetBy::Console)
            .is_err()
    );
    assert!(t.set_by(fade, Value::Bool(true), SetBy::Console).is_err());
    assert!(t.set_by(lights, Value::Int(43), SetBy::Mod).is_err());
    assert!(t.set_text_by(filter, "ultra", SetBy::User).is_err());
    assert!(t.set_text_by(pair, "1,600", SetBy::User).is_err());
    assert!(t.set_text_by(pair, "1,2,3", SetBy::User).is_err());
    assert_eq!(t.revision(), rev, "a refused set changes nothing");
    assert!(t.var(fade).layer(SetBy::Console).is_none());
    // Integers widen for floats; text parses per kind.
    assert_eq!(
        t.set_by(fade, Value::Int(20), SetBy::Code),
        Ok(SetOutcome::Changed)
    );
    assert_eq!(t.get(fade), &Value::Float(20.0));
    assert_eq!(
        t.set_text_by(pair, "35,45", SetBy::Mod),
        Ok(SetOutcome::Changed)
    );
    assert_eq!(t.get(pair), &Value::Float2([35.0, 45.0]));
    assert_eq!(t.get(pair).to_string(), "35,45");
    assert_eq!(
        t.set_text_by(filter, "PCSS", SetBy::User),
        Ok(SetOutcome::Changed)
    );
    assert_eq!(t.text(filter), "pcss");
}

#[test]
fn changes_fire_once_per_change() {
    let mut t = store();
    let fade = t.id("shadow.lamp.fade_start_m").unwrap();
    let lights = t.id("shadow.lamp.count").unwrap();
    let contact = t.id("shadow.contact").unwrap();
    assert!(t.take_changes().is_empty(), "registering is not a change");
    t.set_by(lights, Value::Int(8), SetBy::User).unwrap();
    t.set_by(fade, Value::Float(30.0), SetBy::User).unwrap();
    t.set_by(lights, Value::Int(9), SetBy::User).unwrap();
    // Same effective value from another layer, or a weaker layer under a stronger one: no change.
    assert_eq!(
        t.set_by(fade, Value::Float(30.0), SetBy::Console),
        Ok(SetOutcome::Unchanged)
    );
    assert_eq!(
        t.var(fade).set_by(),
        SetBy::Console,
        "the stronger layer still owns it"
    );
    assert_eq!(
        t.set_by(fade, Value::Float(1.0), SetBy::Mod),
        Ok(SetOutcome::Outranked(SetBy::Console))
    );
    assert_eq!(t.revision(), 3);
    assert_eq!(t.var(lights).generation(), 2);
    assert_eq!(
        t.take_changes(),
        [lights, fade],
        "each once, in first-change order"
    );
    assert!(t.take_changes().is_empty());
    // Peeling that changes the effective value is a change; one that doesn't is not.
    assert!(
        !t.clear(fade, SetBy::Console),
        "user layer holds the same 30"
    );
    t.set(contact, Value::Bool(false)).unwrap();
    t.set(contact, Value::Bool(true)).unwrap();
    assert_eq!(
        t.take_changes(),
        [contact],
        "back and forth is still one entry"
    );
    assert_eq!(t.var(contact).generation(), 2);
    assert!(t.reset(lights));
    assert_eq!(t.take_changes(), [lights]);
}

#[test]
fn dump_is_stable_and_sorted() {
    let mut a = store();
    let mut b = Tunables::default();
    for d in TUNABLES.iter().rev() {
        b.register_any(*d).unwrap();
    }
    assert_eq!(
        a.dump_markdown(),
        b.dump_markdown(),
        "registration order doesn't matter"
    );
    let lights = a.id("shadow.lamp.count").unwrap();
    a.set_noted(
        lights,
        Value::Int(6),
        SetBy::Mod,
        Some("base.esp: measured".into()),
    )
    .unwrap();
    let rows = a.rows();
    let names: Vec<&str> = rows.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "ai.wake_radius_m",
            "shadow.contact",
            "shadow.filter",
            "shadow.lamp.count",
            "shadow.lamp.fade_m",
            "shadow.lamp.fade_start_m"
        ]
    );
    let row = &rows[3];
    assert_eq!(
        (row.value.as_str(), row.default.as_str(), row.set_by),
        ("6", "4", "mod")
    );
    assert_eq!(row.note.as_deref(), Some("base.esp: measured"));
    assert_eq!(row.layers, [("mod", "6".to_owned())]);
    assert!(
        row.location
            .as_deref()
            .unwrap()
            .contains("sf_tunables/src/tests.rs:")
    );
    assert_eq!(a.guesses(), 4);
    let md = a.dump_markdown();
    let lines: Vec<&str> = md.lines().collect();
    assert_eq!(lines[0], "6 tunable(s), 4 guess(es)");
    assert_eq!(lines.len(), 4 + 6);
    let count_line = lines
        .iter()
        .find(|l| l.starts_with("| `shadow.lamp.count`"))
        .unwrap();
    let cells: Vec<&str> = count_line.split(" | ").collect();
    assert_eq!(
        cells[..10],
        [
            "| `shadow.lamp.count`",
            "6",
            "4",
            "int 0..=42",
            "count",
            "mod (base.esp: measured)",
            "measured: docs/ledger/shadows.md 2026-11-12",
            "live",
            "presentation",
            "dev"
        ]
    );
    let filter_line = lines
        .iter()
        .find(|l| l.starts_with("| `shadow.filter`"))
        .unwrap();
    assert!(
        filter_line.contains(" hard\\|pcf\\|pcss "),
        "pipes escaped: {filter_line}"
    );
}

#[test]
fn config_is_the_user_layer() {
    assert_eq!(SetBy::Config, SetBy::User);
    assert_eq!(SetBy::User.describe(), "config");
    assert!(SetBy::ALL.windows(2).all(|w| w[0] < w[1]));
}
