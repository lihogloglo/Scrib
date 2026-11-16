//! The Graphics page (tunables T5d): rows from records, picks live through the preset drivers,
//! rows following the drivers, restart badges, saved levels at start.

use mw_devtools_bevy::perf::Off;
use sf_tunables_bevy::overrides::{Override, OverrideSet};
use sf_tunables_bevy::sf_tunables::presets as tp;
use sf_tunables_bevy::{Presets, SetBy, Tunables};
use sf_ui::SettingValue;
use sf_ui_bevy::UiSettingsStore;
use sf_ui_screens::{OptionsChange, OptionsEvent, OptionsScreen};

use super::*;
use crate::pause_menu::tests::{app_with, frames, open, tap};
use crate::pause_menu::OPTIONS;
use crate::quality_presets::{self as qp, Quality, TEXTURE_MAX_SIZE};

const GROUPS: [&str; 8] = ["sg.view_distance", "sg.shadows", "sg.lighting", "sg.post", "sg.textures", "sg.effects", "sg.water", "sg.sky"];

/// The render settings' and the presets' tunables at `quality`, the book in `Presets` (as the
/// exterior builds them); `texture_cap` is a `--texture-cap` (`CommandLine`).
fn presets(app: &mut App, quality: Quality, texture_cap: Option<u32>) {
    if !app.is_plugin_added::<sf_tunables_bevy::TunablesPlugin>() {
        app.add_plugins(sf_tunables_bevy::TunablesPlugin);
    }
    mw_environment_bevy::render::RenderSettings::new(false, None, true, quality).expect("settings").with_shadow_cache(false).insert(app);
    qp::register(app);
    let book = qp::apply(&mut app.world_mut().resource_mut::<Tunables>(), None, quality, &Off::default(), texture_cap);
    app.insert_resource(Presets(book));
}

/// The UI test app (pause menu, options) with the presets at High.
fn ui_app() -> App {
    app_with(|app| presets(app, Quality::High, None))
}

/// A tunables-only app with the presets at `quality`.
fn tunables_app(quality: Quality, texture_cap: Option<u32>) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, sf_tunables_bevy::TunablesPlugin));
    presets(&mut app, quality, texture_cap);
    app.update();
    app
}

fn driver(app: &App, name: &str) -> String {
    let t = app.world().resource::<Tunables>();
    t.text(t.id(name).expect(name)).to_owned()
}

fn cap(app: &App) -> i64 {
    let t = app.world().resource::<Tunables>();
    t.get(t.id(TEXTURE_MAX_SIZE.name()).expect("cap")).as_int().expect("int")
}

fn book(app: &App) -> PresetBook {
    app.world().resource::<Presets>().0.clone()
}

/// Esc, Options (Resume, Save, Load, Options): the options screen's entity.
fn open_options(app: &mut App) -> Entity {
    tap(app, KeyCode::Escape);
    for _ in 0..3 {
        tap(app, KeyCode::ArrowDown);
    }
    tap(app, KeyCode::Enter);
    assert!(open(app, OPTIONS));
    let mut q = app.world_mut().query_filtered::<Entity, With<OptionsScreen>>();
    q.single(app.world()).expect("options screen")
}

/// A graphics row's shown value and restart badge.
fn row(app: &mut App, key: &str) -> (String, bool) {
    let mut q = app.world_mut().query::<&OptionsScreen>();
    let screen = q.single(app.world()).expect("options screen");
    let row = screen.model.row(key).unwrap_or_else(|| panic!("row {key}"));
    match &row.value {
        SettingValue::Text(v) => (v.clone(), row.restart),
        other => panic!("{key}: {other:?}"),
    }
}

fn stored(app: &App, key: &str) -> Option<SettingValue> {
    app.world().resource::<UiSettingsStore>().0.get(key).cloned()
}

fn pick(app: &mut App, screen: Entity, key: &str, level: &str) {
    app.world_mut().write_message(OptionsEvent { screen, change: OptionsChange::Set(key.into(), SettingValue::Text(level.into())) });
    frames(app, 3);
}

/// The records give one row per driver of the shipped book, in book order, with the drivers'
/// options, High as default (the run's default level), `Machine` scope, page `graphics`.
#[test]
fn records_are_the_drivers() {
    use sf_ui::records::{SettingKind, SettingScope};
    let app = tunables_app(Quality::High, None);
    let book = book(&app);
    let registry = registry();
    let rows: Vec<&str> = registry.settings().filter(|d| d.source.is_some()).map(|d| d.id.as_str()).collect();
    let drivers: Vec<&str> = tp::DRIVERS.iter().map(|d| d.name()).collect();
    assert_eq!(rows, drivers);
    let t = app.world().resource::<Tunables>();
    for def in registry.settings().filter(|d| d.source.is_some()) {
        let key = def.id.as_str();
        assert!(book.driver(key).is_some(), "{key} is a driver of the book");
        assert!(is_row(key));
        let SettingKind::Choice { default, options, .. } = &def.def.kind else { panic!("{key}: a choice") };
        assert_eq!(default, Quality::default().name(), "{key}");
        let sf_tunables_bevy::sf_tunables::ArgKind::Choice(want) = t.var(t.id(key).expect(key)).kind() else { panic!("{key}") };
        assert_eq!(options, want, "{key}: the driver's options");
        assert_eq!((def.def.page.as_deref(), def.def.scope), (Some(PAGE), SettingScope::Machine), "{key}");
    }
}

/// In the menu: the page is labelled from the builtin strings (no readable-id fallbacks); the
/// overall pick sets every group row and reaches the drivers' members; a group pick makes the
/// overall read Custom; a hand-set member makes its group (and the overall) read Custom while the
/// stored picks stay; rows whose levels write a `Restart` tunable carry the badge.
#[test]
fn picks_apply_live_and_rows_follow_the_drivers() {
    let mut app = ui_app();
    let screen = open_options(&mut app);
    {
        let mut q = app.world_mut().query::<&OptionsScreen>();
        let model = &q.single(app.world()).expect("options").model;
        let page = model.pages.iter().find(|p| p.id == PAGE).expect("graphics page");
        assert_eq!(page.label, "Graphics");
        let keys: Vec<&str> = page.rows.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(keys, tp::DRIVERS.iter().map(|d| d.name()).collect::<Vec<_>>());
        let overall = &page.rows[0];
        assert_eq!(overall.label, "Graphics Preset");
        assert_eq!(overall.description.as_deref().map(|d| d.starts_with("Sets every group")), Some(true));
        let sf_ui_screens::OptionKind::Choice(options) = &overall.kind else { panic!("choice") };
        let labels: Vec<&str> = options.iter().map(|(_, l)| l.as_str()).collect();
        assert_eq!(labels, ["Extremely Low", "Low", "Medium", "High", "Ultra", "Custom"]);
        assert_eq!(page.rows[2].label, "Shadows");
    }
    assert_eq!(row(&mut app, tp::GRAPHICS_PRESET.name()), ("high".into(), true), "the run's level, restart badge");
    for (key, restart) in [
        ("sg.view_distance", true),
        ("sg.shadows", false),
        ("sg.lighting", true),
        ("sg.post", false),
        ("sg.textures", true),
        ("sg.effects", false),
        ("sg.water", false),
        ("sg.sky", false),
    ] {
        assert_eq!(row(&mut app, key), ("high".into(), restart), "{key}");
    }

    // The overall: every group row follows, the members hold the level, the store keeps the picks.
    pick(&mut app, screen, tp::GRAPHICS_PRESET.name(), "extremely_low");
    let book = book(&app);
    assert!(book.is_in_effect(app.world().resource::<Tunables>(), tp::GRAPHICS_PRESET.name(), "extremely_low"));
    let lamps = {
        let t = app.world().resource::<Tunables>();
        t.get(t.id("shadow.lamp.count").expect("lamps")).as_int()
    };
    assert_eq!(lamps, Some(0), "Extremely low: no lamp shadows");
    assert_eq!(row(&mut app, tp::GRAPHICS_PRESET.name()).0, "extremely_low");
    for key in GROUPS {
        assert_eq!(row(&mut app, key).0, "extremely_low", "{key}");
        assert_eq!(driver(&app, key), "extremely_low", "{key}");
        assert_eq!(stored(&app, key), Some(SettingValue::Text("extremely_low".into())), "{key} stored");
    }
    let t = app.world().resource::<Tunables>();
    let var = t.var(t.id(tp::GRAPHICS_PRESET.name()).expect("overall"));
    assert_eq!(var.set_by(), SetBy::Preset, "the User request was taken (its layer peeled)");

    // A group: the overall reads Custom, in the rows, the drivers and the store.
    pick(&mut app, screen, "sg.shadows", "ultra");
    assert_eq!(row(&mut app, "sg.shadows").0, "ultra");
    assert_eq!(driver(&app, "sg.shadows"), "ultra");
    assert_eq!(row(&mut app, tp::GRAPHICS_PRESET.name()).0, "custom");
    assert_eq!(driver(&app, tp::GRAPHICS_PRESET.name()), "custom");
    assert_eq!(stored(&app, tp::GRAPHICS_PRESET.name()), Some(SettingValue::Text("custom".into())));
    assert_eq!(row(&mut app, "sg.water").0, "extremely_low", "other groups keep theirs");

    // Back to a level: the overall reads it again.
    pick(&mut app, screen, "sg.shadows", "extremely_low");
    assert_eq!(row(&mut app, tp::GRAPHICS_PRESET.name()).0, "extremely_low");
    assert_eq!(stored(&app, tp::GRAPHICS_PRESET.name()), Some(SettingValue::Text("extremely_low".into())));

    // A hand-set member (console): its group and the overall read Custom; the picks stay stored.
    app.world_mut().write_message(sf_tunables_bevy::SetTunable::new("shadow.lamp.count", "2", SetBy::Console));
    frames(&mut app, 3);
    assert_eq!(row(&mut app, "sg.shadows").0, "custom");
    assert_eq!(row(&mut app, tp::GRAPHICS_PRESET.name()).0, "custom");
    assert_eq!(row(&mut app, "sg.water").0, "extremely_low");
    assert_eq!(stored(&app, "sg.shadows"), Some(SettingValue::Text("extremely_low".into())), "a console value isn't saved");
}

/// Picks are `Machine` settings: written to the machine file when the menus close; read back,
/// they are the start levels (as `User` requests): the restart values start with them, a
/// `CommandLine` member (`--texture-cap`) stays above them, an explicit `--quality` wins over
/// them, and a `--set` of a group after that wins too.
#[test]
fn saved_levels_load_at_the_user_layer_under_the_command_line() {
    let dir = std::env::temp_dir().join(format!("bw-graphics-page-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("machine.toml");
    let mut app = ui_app();
    sf_ui_bevy::load_machine_settings(app.world_mut(), file.clone()).expect("no file yet");
    let screen = open_options(&mut app);
    pick(&mut app, screen, tp::GRAPHICS_PRESET.name(), "ultra");
    pick(&mut app, screen, "sg.shadows", "low");
    assert!(!file.exists(), "not written while the menus are open");
    tap(&mut app, KeyCode::Escape);
    tap(&mut app, KeyCode::Escape);
    assert!(!open(&app, OPTIONS));
    let text = std::fs::read_to_string(&file).expect("machine file written");
    assert!(text.contains("sg.shadows") && text.contains("ultra"), "{text}");

    // A restart: the saved picks, overall first (custom left out).
    let started = tunables_app(Quality::High, None);
    let book = book(&started);
    let saved = load_saved(&file, &book);
    assert!(!saved.iter().any(|(d, _)| d == tp::GRAPHICS_PRESET.name()), "the overall is custom: {saved:?}");
    assert!(saved.contains(&("sg.shadows".into(), "low".into())) && saved.contains(&("sg.textures".into(), "ultra".into())), "{saved:?}");

    // As the exterior starts: the saved levels at `User`; the restart values follow them.
    let mut app = tunables_app(Quality::High, None);
    let applied = apply_start_levels(&mut app.world_mut().resource_mut::<Tunables>(), &book, &saved, None);
    assert!(applied.iter().all(|l| l.ends_with("(options menu)")), "{applied:?}");
    let start = qp::start_values(&mut app.world_mut().resource_mut::<Tunables>(), &book, &[]);
    assert_eq!(start.texture_cap, None, "Ultra textures: no cap");
    assert_eq!((driver(&app, "sg.shadows"), driver(&app, "sg.textures"), driver(&app, tp::GRAPHICS_PRESET.name())), ("low".into(), "ultra".into(), "custom".into()));

    // A `CommandLine` member stays above the saved level.
    let mut app = tunables_app(Quality::High, Some(1024));
    let _ = apply_start_levels(&mut app.world_mut().resource_mut::<Tunables>(), &book, &saved, None);
    assert_eq!(cap(&app), 1024, "--texture-cap stays above the saved Ultra textures");
    assert_eq!(driver(&app, "sg.textures"), "custom");

    // An explicit `--quality` wins over the saved levels; a later `--set` of a group wins over it.
    let mut app = tunables_app(Quality::Low, None);
    let _ = apply_start_levels(&mut app.world_mut().resource_mut::<Tunables>(), &book, &saved, Some(Quality::Low.name()));
    for key in GROUPS {
        assert_eq!(driver(&app, key), "low", "{key}: --quality low");
    }
    let sets = [OverrideSet { origin: "--set".into(), overrides: vec![Override { name: "sg.water".into(), value: "ultra".into() }] }];
    let _ = qp::start_values(&mut app.world_mut().resource_mut::<Tunables>(), &book, &sets);
    assert_eq!((driver(&app, "sg.water"), driver(&app, tp::GRAPHICS_PRESET.name())), ("ultra".into(), "custom".into()));
    let _ = std::fs::remove_dir_all(&dir);
}

/// The menu preset is the overall level's group levels; a level's groups are all that level.
#[test]
fn menu_preset_maps_each_overall_level_to_its_groups() {
    let app = tunables_app(Quality::High, None);
    let menu = menu_preset(&book(&app)).expect("overall driver");
    assert_eq!(menu.driver, tp::GRAPHICS_PRESET.name());
    for level in tp::LEVELS {
        let groups = menu.levels.get(level).unwrap_or_else(|| panic!("{level}"));
        assert_eq!(groups.keys().map(String::as_str).collect::<Vec<_>>().len(), GROUPS.len(), "{level}");
        assert!(groups.values().all(|v| *v == SettingValue::Text(level.into())), "{level}: {groups:?}");
    }
}
