//! Setting presets: levels, the derived "custom" driver, validation and mod
//! additions (menus.md "Settings v2", Presets).

use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::records::SettingPresetDef;
use crate::*;

const SETTINGS: &str = r#"
[[define]]
kind = "ui.setting"
id = "m.preset"
value = { label = "set-preset", kind = { Choice = { default = "custom", options = ["low", "high", "custom"] } } }

[[define]]
kind = "ui.setting"
id = "m.shadows"
value = { label = "set-shadows", kind = { Choice = { default = "low", options = ["low", "high"] } } }

[[define]]
kind = "ui.setting"
id = "m.distance"
value = { label = "set-distance", kind = { Float = { default = 5.0, min = 0.0, max = 10.0 } } }

[[define]]
kind = "ui.setting"
id = "m.ao"
value = { label = "set-ao", kind = { Bool = { default = false } } }

[[define]]
kind = "ui.setting"
id = "m.count"
value = { label = "set-count", kind = { Int = { default = 2, min = 0, max = 4 } } }
"#;

const GRAPHICS: &str = r#"
[[define]]
kind = "ui.setting_preset"
id = "m.graphics"
value = { driver = "m.preset", levels = { low = { "m.shadows" = "low", "m.distance" = 2.0, "m.ao" = false }, high = { "m.shadows" = "high", "m.distance" = 8, "m.ao" = true } } }
"#;

/// Loads one TOML file per mod, in load order (each mod depends on the ones before).
fn build(mods: &[(&str, &str)]) -> (UiRegistry, Vec<UiConflict>) {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let mut sources = Vec::new();
    for (name, text) in mods {
        let src = db.add_source(*name, &sources);
        let outcome = apply_record_file(&mut db, src, text, RecordFormat::Toml).unwrap();
        assert_eq!(outcome.errors, [], "{text}");
        sources.push(src);
    }
    UiRegistry::build(&db)
}

fn graphics() -> (UiRegistry, SettingPresetDef) {
    let (reg, conflicts) = build(&[("m", SETTINGS), ("m2", GRAPHICS)]);
    assert_eq!(conflicts, []);
    let preset = reg.setting_preset("m.graphics").unwrap().def.clone();
    (reg, preset)
}

fn text(s: &str) -> SettingValue {
    SettingValue::Text(s.into())
}

#[test]
fn a_level_applies_and_reads_custom_once_a_member_changes() {
    let (reg, preset) = graphics();
    assert_eq!(
        preset.levels["high"]["m.distance"],
        SettingValue::Float(8.0),
        "an int for a float setting is stored as the store holds it"
    );
    let mut store = SettingsStore::new(&reg);
    assert_eq!(derived_level(&preset, &store), None, "defaults match no level");

    apply_level(&preset, "low", &mut store).unwrap();
    assert_eq!(store.get("m.shadows"), Some(&text("low")));
    assert_eq!(store.get("m.distance"), Some(&SettingValue::Float(2.0)));
    assert_eq!(derived_level(&preset, &store), Some("low".into()));
    assert_eq!(store.get("m.preset"), Some(&text("low")));

    store.set("m.distance", SettingValue::Float(3.0)).unwrap();
    assert_eq!(sync_driver(&preset, &mut store), Ok(None));
    assert_eq!(store.get("m.preset"), Some(&text(CUSTOM_LEVEL)));

    store.set("m.distance", SettingValue::Float(2.0)).unwrap();
    assert_eq!(sync_driver(&preset, &mut store), Ok(Some("low".into())));
    assert_eq!(store.get("m.preset"), Some(&text("low")));

    apply_level(&preset, "high", &mut store).unwrap();
    assert_eq!(store.get("m.ao"), Some(&SettingValue::Bool(true)));
    assert_eq!(store.get("m.preset"), Some(&text("high")));
}

#[test]
fn choosing_custom_changes_nothing() {
    let (reg, preset) = graphics();
    let mut store = SettingsStore::new(&reg);
    apply_level(&preset, "low", &mut store).unwrap();

    apply_level(&preset, CUSTOM_LEVEL, &mut store).unwrap();
    assert_eq!(store.get("m.distance"), Some(&SettingValue::Float(2.0)));
    assert_eq!(
        store.get("m.preset"),
        Some(&text("low")),
        "the driver is re-derived, not stored: the values still match low"
    );

    store.set("m.ao", SettingValue::Bool(true)).unwrap();
    apply_level(&preset, CUSTOM_LEVEL, &mut store).unwrap();
    assert_eq!(store.get("m.ao"), Some(&SettingValue::Bool(true)));
    assert_eq!(store.get("m.preset"), Some(&text(CUSTOM_LEVEL)));

    assert_eq!(
        apply_level(&preset, "ultra", &mut store),
        Err(PresetError::UnknownLevel("ultra".into()))
    );
}

#[test]
fn levels_with_the_same_values_resolve_to_the_first_level_id() {
    let settings = SETTINGS.replace(r#"["low", "high", "custom"]"#, r#"["b", "a", "custom"]"#);
    let preset = r#"
[[define]]
kind = "ui.setting_preset"
id = "m.graphics"
value = { driver = "m.preset", levels = { b = { "m.ao" = true }, a = { "m.ao" = true } } }
"#;
    let (reg, conflicts) = build(&[("m", &settings), ("m2", preset)]);
    assert_eq!(conflicts, []);
    let preset = reg.setting_preset("m.graphics").unwrap().def.clone();
    let mut store = SettingsStore::new(&reg);
    apply_level(&preset, "b", &mut store).unwrap();
    assert_eq!(
        derived_level(&preset, &store),
        Some("a".into()),
        "level-id order (byte-wise), not the driver's option order"
    );
    assert_eq!(store.get("m.preset"), Some(&text("a")));
}

#[test]
fn a_missing_or_non_choice_driver_drops_the_preset() {
    for (driver, reason) in [
        ("m.nope", "driver `m.nope` is not a setting"),
        ("m.ao", "driver `m.ao` is not a Choice setting"),
    ] {
        let preset = GRAPHICS.replace(r#"driver = "m.preset""#, &format!(r#"driver = "{driver}""#));
        let (reg, conflicts) = build(&[("m", SETTINGS), ("m2", &preset)]);
        assert_eq!(
            conflicts,
            [UiConflict::InvalidSettingPreset {
                preset: "m.graphics".into(),
                level: None,
                key: None,
                owner: ModId::from("m2"),
                reason: reason.into(),
            }]
        );
        assert_eq!(conflicts[0].severity(), Severity::Error);
        assert!(reg.setting_preset("m.graphics").is_none());
    }
}

#[test]
fn an_unknown_key_or_bad_value_drops_that_value_naming_the_mod() {
    let addition = r#"
[[define]]
kind = "ui.setting_preset"
id = "x.graphics"
value = { driver = "m.preset", levels = { low = { "x.unknown" = 1, "m.distance" = 11.0 }, high = { "m.count" = "many" } } }
"#;
    let (reg, conflicts) = build(&[("m", SETTINGS), ("m2", GRAPHICS), ("x", addition)]);
    let bad = |level: &str, key: &str, reason: &str| UiConflict::InvalidSettingPreset {
        preset: "x.graphics".into(),
        level: Some(level.into()),
        key: Some(SettingKey::from(key)),
        owner: ModId::from("x"),
        reason: reason.into(),
    };
    assert_eq!(
        conflicts,
        [
            bad("high", "m.count", "setting `m.count` expects an integer"),
            bad(
                "low",
                "m.distance",
                "11 is out of range or off step (the setting would hold 10)"
            ),
            bad("low", "x.unknown", "unknown setting"),
        ]
    );
    assert_eq!(
        conflicts[2].to_string(),
        "x: setting preset `x.graphics` level `low`: `x.unknown` dropped: unknown setting"
    );
    let preset = &reg.setting_preset("m.graphics").unwrap().def;
    assert_eq!(
        preset.levels["low"],
        graphics().1.levels["low"],
        "the bad values are dropped, the base level is untouched"
    );
    assert!(reg.setting_preset("x.graphics").is_none(), "merged, not a preset of its own");
}

#[test]
fn a_mod_adds_a_key_to_a_level() {
    let addition = r#"
[[define]]
kind = "ui.setting_preset"
id = "x.graphics"
value = { driver = "m.preset", levels = { low = { "m.count" = 1 } } }
"#;
    let retune = r#"
[[define]]
kind = "ui.setting_preset"
id = "y.graphics"
value = { driver = "m.preset", levels = { low = { "m.count" = 0, "m.distance" = 1.5 } } }
"#;
    let (reg, conflicts) = build(&[("m", SETTINGS), ("m2", GRAPHICS), ("x", addition)]);
    assert_eq!(conflicts, []);
    let def = reg.setting_preset("m.graphics").unwrap();
    assert_eq!(def.owner, ModId::from("m2"), "the first record's mod still owns it");
    assert_eq!(def.def.levels["low"]["m.count"], SettingValue::Int(1));
    assert_eq!(def.def.levels["low"]["m.shadows"], text("low"), "merged, not replaced");
    assert_eq!(def.def.levels["high"].len(), 3, "other levels untouched");
    assert_eq!(
        reg.setting_preset_for_driver("m.preset").map(|d| d.id.as_str()),
        Some("m.graphics")
    );

    let (later, conflicts) =
        build(&[("m", SETTINGS), ("m2", GRAPHICS), ("x", addition), ("y", retune)]);
    assert_eq!(conflicts, []);
    let low = &later.setting_preset("m.graphics").unwrap().def.levels["low"];
    assert_eq!(low["m.count"], SettingValue::Int(0), "the last mod in load order wins");
    assert_eq!(low["m.distance"], SettingValue::Float(1.5));

    let preset = def.def.clone();
    let mut store = SettingsStore::new(&reg);
    apply_level(&preset, "low", &mut store).unwrap();
    assert_eq!(store.get("m.count"), Some(&SettingValue::Int(1)));
    assert_eq!(store.get("m.preset"), Some(&text("low")));
    store.set("m.count", SettingValue::Int(2)).unwrap();
    assert_eq!(sync_driver(&preset, &mut store), Ok(None), "the added key is a member");
}
