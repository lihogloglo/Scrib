//! Glue tests over a real mod pipeline: folder mods with Fluent files in two
//! locales, an override from a later mod, and settings through a profile.

use std::path::Path;

use semver::Version;
use sf_mods::{
    ContentAdapter, LoadedProfile, ModId, ModRepository, NativeRecords, Profile, load_profile,
};

use crate::*;

/// Writes a folder mod `<dir>/<id>` whose asset root is `data/`, with its
/// UI records in `records/ui.toml`.
fn write_mod(dir: &Path, id: &str, files: &[(&str, &str)]) {
    let root = dir.join(id);
    let manifest = format!(
        "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\ndescription = \"test\"\n\
         [content]\nassets = [\"data\"]\nrecords = [\"records/ui.toml\"]\n"
    );
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join(sf_mods::MANIFEST_FILE), manifest).unwrap();
    for (path, text) in files {
        let path = root.join("data").join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
}

const BASE_RECORDS: &str = r#"
[[define]]
kind = "ui.setting"
id = "base.text_scale"
value = { label = "set-text-scale", kind = { Float = { default = 1.0, min = 1.0, max = 2.0, step = 0.25 } } }

[[define]]
kind = "ui.setting"
id = "base.subtitles"
value = { label = "set-subtitles", kind = { Choice = { default = "on", options = ["off", "on", "speaker"] } } }
"#;

const REWORD_RECORDS: &str = r#"
[[define]]
kind = "ui.setting"
id = "reword.loud"
value = { label = "set-loud", kind = { Bool = { default = false } } }
"#;

fn load(dir: &Path, profile: &Profile) -> LoadedProfile {
    let ui = UiRecordKinds;
    let adapters: [&dyn ContentAdapter; 2] = [&ui, &NativeRecords];
    let repo = ModRepository::scan(&[dir.to_path_buf()], &adapters);
    load_profile(&repo, profile, &adapters, &Version::new(0, 1, 0))
}

fn two_mods(dir: &Path) {
    write_mod(
        dir,
        "base",
        &[
            ("records/ui.toml", BASE_RECORDS),
            (
                "l10n/en-US/base.ftl",
                "menu-inventory = Inventory\nmenu-map = Map\nhello = Hello, { $name }!\n",
            ),
            (
                "l10n/de/base.ftl",
                "menu-inventory = Inventar\nhello = Hallo, { $name }!\n",
            ),
        ],
    );
    write_mod(
        dir,
        "reword",
        &[
            ("records/ui.toml", REWORD_RECORDS),
            // Rewords one message of `base` from its own file.
            ("l10n/en-US/reword.ftl", "menu-inventory = Pack\n"),
            // A file named after another mod: reported, ignored.
            ("l10n/de/base.ftl", "menu-inventory = Rucksack\n"),
            // One syntax error; the valid message still loads.
            ("l10n/de/reword.ftl", "menu-map = Karte\nbroken = { \n"),
        ],
    );
}

fn id(s: &str) -> ModId {
    ModId::new(s).unwrap()
}

#[test]
fn fluent_from_mod_files_in_load_order_with_override() {
    let tmp = tempfile::tempdir().unwrap();
    two_mods(tmp.path());
    let profile = Profile::new("p", [id("base"), id("reword")]);
    let loaded = load(tmp.path(), &profile);
    assert!(
        !loaded.report.has_errors(),
        "{}",
        loaded.render_report(None, 10)
    );

    let (sources, issues) = l10n_sources(&loaded);
    let order: Vec<(&str, String)> = sources
        .iter()
        .map(|s| (s.mod_id.as_str(), s.locale.to_string()))
        .collect();
    assert_eq!(
        order,
        [
            ("base", "de".into()),
            ("base", "en-US".into()),
            ("reword", "de".into()),
            ("reword", "en-US".into())
        ]
    );
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(
        (issues[0].mod_id.as_str(), issues[0].path.as_str()),
        ("reword", "l10n/de/base.ftl")
    );
    assert_eq!(available_locales(&sources).len(), 2);

    let en: LanguageIdentifier = "en-US".parse().unwrap();
    let (english, issues) = Localizer::from_profile(&loaded, &["en-GB"], &en, false);
    assert_eq!(issues.len(), 1, "only the misnamed file: {issues:?}");
    assert_eq!(
        english.format("menu-inventory", None),
        "Pack",
        "a later mod rewords"
    );
    assert_eq!(english.format("menu-map", None), "Map");

    let (german, issues) = Localizer::from_profile(&loaded, &["de-AT"], &en, false);
    assert_eq!(issues.len(), 2, "misnamed file + syntax error: {issues:?}");
    assert!(issues.iter().all(|i| i.mod_id == "reword"));
    assert_eq!(
        german
            .locales()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["de", "en-US"]
    );
    assert_eq!(
        german.format("menu-inventory", None),
        "Inventar",
        "the misnamed file doesn't shadow"
    );
    assert_eq!(german.format("menu-map", None), "Karte");
    let mut args = FluentArgs::new();
    args.set("name", "Vivec");
    assert_eq!(german.format("hello", Some(&args)), "Hallo, Vivec!");
    assert_eq!(
        german.format("broken", None),
        "broken",
        "missing everywhere: the key"
    );
}

#[test]
fn settings_round_trip_through_a_profile_file() {
    let tmp = tempfile::tempdir().unwrap();
    two_mods(tmp.path());
    let mut profile = Profile::new("p", [id("base"), id("reword")]);
    profile
        .settings
        .insert(id("gone"), toml::from_str("keep = 7").unwrap());
    let loaded = load(tmp.path(), &profile);
    let (registry, _) = UiRegistry::build(&loaded.records);

    let mut store = SettingsStore::new(&registry);
    assert_eq!(store.load_from(&profile), []);
    store
        .set("base.text_scale", SettingValue::Float(1.4))
        .unwrap();
    store.set("reword.loud", SettingValue::Bool(true)).unwrap();
    assert!(store.save_to(&mut profile));
    assert!(!store.save_to(&mut profile), "saving again changes nothing");
    assert_eq!(profile.settings[&id("base")].len(), 1, "only non-defaults");

    let dir = tmp.path().join("profiles");
    let path = profile.save(&dir).unwrap();
    let reread = Profile::load(&path).unwrap();
    assert_eq!(reread, profile);

    let mut fresh = SettingsStore::new(&registry);
    assert_eq!(fresh.load_from(&reread), []);
    assert_eq!(
        fresh.get_f64("base.text_scale"),
        Some(1.5),
        "snapped to the step, then kept"
    );
    assert_eq!(fresh.get_bool("reword.loud"), Some(true));
    assert_eq!(
        fresh.get("base.subtitles"),
        Some(&SettingValue::Text("on".into()))
    );
    let mut again = reread.clone();
    fresh.save_to(&mut again);
    assert_eq!(
        again.settings[&id("gone")]["keep"],
        toml::Value::Integer(7),
        "unknown mod's values pass through"
    );
}

#[test]
fn push_in_enforces_screen_presets() {
    let mut db = sf_records::RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let text = r#"
[[define]]
kind = "ui.screen"
id = "classic_window"
value = { layer = "Menu", implementation = { Template = "w" }, presets = ["pc_dense"] }

[[define]]
kind = "ui.screen"
id = "any"
value = { layer = "Menu", implementation = { Template = "a" } }
"#;
    let outcome =
        sf_records::apply_record_file(&mut db, base, text, sf_records::RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    let (registry, _) = UiRegistry::build(&db);
    let mut stack = ScreenStack::new();
    assert_eq!(
        stack
            .push_in(&registry, "classic_window", "controller")
            .unwrap_err(),
        StackError::UnsupportedPreset {
            screen: "classic_window".into(),
            preset: "controller".into()
        }
    );
    assert_eq!(stack.version(), 0);
    stack
        .push_in(&registry, "classic_window", "pc_dense")
        .unwrap();
    stack
        .replace_top_in(&registry, "any", "controller")
        .unwrap();
    assert_eq!(stack.top(Layer::Menu).unwrap().id.as_str(), "any");
    assert!(matches!(
        stack.push_in(&registry, "nope", "pc_dense"),
        Err(StackError::UnknownScreen(_))
    ));
}
