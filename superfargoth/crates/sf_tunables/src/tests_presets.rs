//! Presets as data (slice T5b): merge, book checks, apply, derived level and custom, requests.

use std::collections::BTreeMap;

use crate::presets::{
    self, CUSTOM, MergedPresets, PresetBook, PresetRecord, RecordValue, TunableRecord,
};
use crate::{AnyDef, Def, SetBy, Tunables, Value, def};

const LAMPS: Def<u32> = def!(count "t.lamps" = 4).range(0, 16);
const FADE: Def<[f32; 2]> = def!(float2 "t.fade_m" = [15.0, 25.0]).range([0.0, 0.0], [500.0, 500.0]);
const BIAS: Def<f32> = def!(float "t.bias" = 1.0).range(0.25, 4.0);
const WATER: Def<&str> = def!(choice "t.water" = "high").choices(&["low", "high"]);
const DEFS: &[&dyn AnyDef] = &[&LAMPS, &FADE, &BIAS, &WATER];

fn store() -> Tunables {
    let mut t = Tunables::default();
    t.register_all(DEFS).unwrap();
    t.register_all(presets::TUNABLES).unwrap();
    t
}

fn level(pairs: &[(&str, RecordValue)]) -> BTreeMap<String, RecordValue> {
    pairs.iter().map(|(k, v)| ((*k).to_owned(), v.clone())).collect()
}

fn record(driver: &str, levels: &[(&str, &[(&str, RecordValue)])]) -> PresetRecord {
    PresetRecord {
        driver: driver.to_owned(),
        source: "guess".to_owned(),
        levels: levels.iter().map(|(l, v)| ((*l).to_owned(), level(v))).collect(),
        ..PresetRecord::default()
    }
}

use RecordValue::{Float as F, Int as I, Text as T};

fn records() -> Vec<PresetRecord> {
    vec![
        record(
            "sf.graphics_preset",
            &[
                ("low", &[("sg.shadows", T("low".into())), ("sg.view_distance", T("low".into()))]),
                ("high", &[("sg.shadows", T("high".into())), ("sg.view_distance", T("high".into()))]),
            ],
        ),
        record("sg.shadows", &[("low", &[("t.lamps", I(0))]), ("high", &[("t.lamps", I(8))])]),
        record(
            "sg.view_distance",
            &[("low", &[("t.bias", F(0.75)), ("t.water", T("low".into()))]), ("high", &[("t.bias", I(1)), ("t.water", T("high".into()))])],
        ),
    ]
}

fn book(store: &Tunables, records: &[PresetRecord], caps: &[&str]) -> PresetBook {
    let merged = MergedPresets::merge(records.iter().map(|r| (r, "base")), caps);
    let (book, issues) = PresetBook::build(store, &merged);
    assert!(issues.is_empty(), "{issues:?}");
    book
}

#[test]
fn a_level_writes_its_members_at_the_preset_layer_and_the_drivers_follow() {
    let mut t = store();
    let book = book(&t, &records(), &[]);
    assert_eq!(book.drivers[0].name, "sf.graphics_preset", "overall first");
    let applied = book.apply(&mut t, "sf.graphics_preset", "low").unwrap();
    assert_eq!(applied.changed.len(), 3);
    book.sync(&mut t);
    let lamps = t.id("t.lamps").unwrap();
    assert_eq!((t.get(lamps), t.var(lamps).set_by()), (&Value::Int(0), SetBy::Preset));
    assert_eq!(t.var(lamps).note(SetBy::Preset), Some("sg.shadows low; guess"));
    for driver in ["sf.graphics_preset", "sg.shadows", "sg.view_distance"] {
        assert_eq!(t.text(t.id(driver).unwrap()), "low", "{driver}");
    }
    assert_eq!(t.text(t.id("sg.water").unwrap()), CUSTOM, "a driver without records reads custom (its default)");
}

#[test]
fn a_hand_change_reads_custom_and_a_reset_goes_back() {
    let mut t = store();
    let book = book(&t, &records(), &[]);
    book.apply(&mut t, "sf.graphics_preset", "high").unwrap();
    book.sync(&mut t);
    let lamps = t.id("t.lamps").unwrap();
    t.set_by(lamps, Value::Int(3), SetBy::Console).unwrap();
    book.sync(&mut t);
    assert_eq!(t.text(t.id("sg.shadows").unwrap()), CUSTOM);
    assert_eq!(t.text(t.id("sf.graphics_preset").unwrap()), CUSTOM, "custom climbs to the overall driver");
    assert_eq!(t.text(t.id("sg.view_distance").unwrap()), "high", "the other group keeps its level");
    // The level, applied again, sits under the console value: still custom.
    let applied = book.apply(&mut t, "sg.shadows", "high").unwrap();
    assert_eq!(applied.outranked, vec![(lamps, SetBy::Console)]);
    assert!(applied.summary(&t).contains("kept above the preset: t.lamps (console)"));
    t.clear(lamps, SetBy::Console);
    book.sync(&mut t);
    assert_eq!(t.text(t.id("sf.graphics_preset").unwrap()), "high", "back to the level");
}

#[test]
#[allow(clippy::float_cmp)] // exact: values set by hand
fn layers_mod_under_preset_under_console() {
    let mut t = store();
    let book = book(&t, &records(), &[]);
    let fade = TunableRecord { value: T("35,45".into()), source: "measured: x".into() };
    let bias = TunableRecord { value: F(2.0), source: String::new() };
    let refused = presets::apply_mod_values(&mut t, [("t.fade_m", &fade, "base"), ("t.bias", &bias, "base"), ("t.nope", &bias, "m2")]);
    assert_eq!(refused.len(), 1, "unknown name refused: {refused:?}");
    let (fade_id, bias_id) = (t.id("t.fade_m").unwrap(), t.id("t.bias").unwrap());
    assert_eq!((t.get(fade_id), t.var(fade_id).set_by()), (&Value::Float2([35.0, 45.0]), SetBy::Mod));
    assert_eq!(t.var(fade_id).note(SetBy::Mod), Some("mod base: measured: x"));
    book.apply(&mut t, "sg.view_distance", "low").unwrap();
    assert_eq!((t.float(bias_id), t.var(bias_id).set_by()), (0.75, SetBy::Preset), "preset over mod");
    t.set_by(bias_id, Value::Float(3.0), SetBy::Console).unwrap();
    assert_eq!(t.float(bias_id), 3.0, "console over preset");
    t.clear(bias_id, SetBy::Console);
    t.clear(bias_id, SetBy::Preset);
    assert_eq!(t.float(bias_id), 2.0, "mod under both");
}

#[test]
fn requests_apply_and_clear_their_layer() {
    let mut t = store();
    let book = book(&t, &records(), &[]);
    book.apply(&mut t, "sf.graphics_preset", "high").unwrap();
    book.sync(&mut t);
    let overall = t.id("sf.graphics_preset").unwrap();
    let shadows = t.id("sg.shadows").unwrap();
    t.set_text_by(overall, "low", SetBy::Console).unwrap();
    t.set_text_by(shadows, "high", SetBy::CommandLine).unwrap();
    let done = book.take_requests(&mut t);
    assert_eq!(done.iter().map(|(d, l, r)| (d.as_str(), l.as_str(), r.is_ok())).collect::<Vec<_>>(), [("sf.graphics_preset", "low", true), ("sg.shadows", "high", true)], "overall first");
    assert_eq!(t.var(overall).set_by(), SetBy::Preset, "the request's layer is consumed");
    assert_eq!(t.text(overall), CUSTOM, "low, but shadows high");
    assert_eq!(t.int(t.id("t.lamps").unwrap()), 8);
    assert!(book.requests(&t).is_empty());
    assert!(!book.out_of_sync(&t));
}

#[test]
fn a_later_record_wins_a_key_when_and_bad_entries_are_reported() {
    let mut t = store();
    let mut later = record("sg.shadows", &[("high", &[("t.lamps", I(12))])]);
    later.sources.insert("high".into(), [("t.lamps".to_owned(), "owner: 2026".to_owned())].into());
    let mut dlss_only = record("sg.shadows", &[("low", &[("t.lamps", I(1))])]);
    dlss_only.when = Some("dlss".into());
    let mut without = record("sg.shadows", &[("low", &[("t.lamps", I(2))])]);
    without.when = Some("!dlss".into());
    let all: Vec<PresetRecord> = records().into_iter().chain([later, dlss_only, without]).collect();
    let merged = MergedPresets::merge(all.iter().map(|r| (r, "m")), &[]);
    let flat = merged.flatten("sf.graphics_preset", "high");
    assert_eq!((flat["t.lamps"].value.clone(), flat["t.lamps"].source.as_str()), (I(12), "owner: 2026"));
    assert_eq!(merged.flatten("sg.shadows", "low")["t.lamps"].value, I(2), "!dlss without the capability");
    let with = MergedPresets::merge(all.iter().map(|r| (r, "m")), &["dlss"]);
    assert_eq!(with.flatten("sg.shadows", "low")["t.lamps"].value, I(1));
    let bad = [
        record("t.lamps", &[("low", &[("t.bias", F(1.0))])]),
        record("sg.water", &[("epic", &[("t.water", T("low".into()))]), ("low", &[("t.nope", I(1)), ("t.bias", F(9.0)), ("t.lamps", I(1))])]),
        record("sg.sky", &[("low", &[("t.lamps", I(1))])]),
    ];
    let merged = MergedPresets::merge(bad.iter().map(|r| (r, "m")), &[]);
    let (book, issues) = PresetBook::build(&t, &merged);
    assert_eq!(issues.len(), 5, "{issues:#?}");
    assert_eq!(book.driver("sg.water").unwrap().level("low").unwrap().values.len(), 1);
    assert_eq!(book.driver("sg.water").unwrap().next_level("low"), Some("low"), "one level wraps to itself");
    assert!(book.apply(&mut t, "sg.water", "ultra").is_err());
}

#[test]
fn identical_levels_show_the_one_asked_for() {
    let mut t = store();
    let same: &[(&str, RecordValue)] = &[("t.water", T("low".into()))];
    let records = [
        record("sf.graphics_preset", &[("extremely_low", &[("sg.water", T("extremely_low".into()))]), ("low", &[("sg.water", T("low".into()))])]),
        record("sg.water", &[("extremely_low", same), ("low", same)]),
    ];
    let book = book(&t, &records, &[]);
    book.apply(&mut t, "sf.graphics_preset", "low").unwrap();
    book.sync(&mut t);
    assert_eq!(t.text(t.id("sg.water").unwrap()), "low");
    assert_eq!(t.text(t.id("sf.graphics_preset").unwrap()), "low", "not custom: the group matches low's values");
    t.set_text_by(t.id("t.water").unwrap(), "high", SetBy::Console).unwrap();
    book.sync(&mut t);
    t.clear(t.id("t.water").unwrap(), SetBy::Console);
    book.sync(&mut t);
    assert_eq!(t.text(t.id("sf.graphics_preset").unwrap()), "extremely_low", "after custom: the first matching level");
}
