//! The Advanced page (MF-6): rows generated from the feature switches (player, live ones only),
//! picks at `User` live, rows following the switches, not-built rows disabled, picks saved per
//! machine and applied at the next start.

use sf_tunables_bevy::sf_tunables::{Category, Def, Scope, def};
use sf_tunables_bevy::{SetBy, Tunables, TunablesAppExt};
use sf_ui::records::{SettingKind, SettingScope};
use sf_ui::SettingValue;
use sf_ui_bevy::UiSettingsStore;
use sf_ui_screens::{OptionsChange, OptionsEvent, OptionsScreen};

use super::*;
use crate::pause_menu::tests::{app_with, frames, open, tap};
use crate::pause_menu::OPTIONS;

const A_ON: Def<bool> = def!(bool "test.adv.fog" = true).help("Test fog").scope(Scope::Player);
const B_CHOICE: Def<&'static str> = def!(choice "test.adv.shadows" = "high").choices(&["off", "low", "high"]).help("Test shadows").scope(Scope::Player);
const C_DEV: Def<bool> = def!(bool "test.adv.dev" = true).help("Dev only");
const D_STARTUP: Def<bool> = def!(bool "test.adv.startup" = true).help("Startup only").scope(Scope::Player);

const A: FeatureSwitch = FeatureSwitch::new(&A_ON).label("Test fog").category(Category::Sky);
const B: FeatureSwitch = FeatureSwitch::choice(&B_CHOICE, "off").label("Test shadows").category(Category::Shadows);
const C: FeatureSwitch = FeatureSwitch::new(&C_DEV).label("Dev switch").category(Category::Debug);
const D: FeatureSwitch = FeatureSwitch::new(&D_STARTUP).label("Startup switch").category(Category::World).startup_only("--off startup");

/// The test switches and their tunables (a Dev one and a startup-only one included).
fn switches(app: &mut App) {
    if !app.is_plugin_added::<sf_tunables_bevy::TunablesPlugin>() {
        app.add_plugins(sf_tunables_bevy::TunablesPlugin);
    }
    app.register_tunables(&[&A_ON, &B_CHOICE, &C_DEV, &D_STARTUP]);
    app.add_feature_switch(&A).add_feature_switch(&B).add_feature_switch(&C).add_feature_switch(&D);
}

/// The UI test app with the switches and the page; `extra` runs before the page is generated
/// (`cleanup`), as the plugins and the machine file load do in the game.
fn ui_app(apply_saved: bool, extra: impl FnOnce(&mut App)) -> App {
    let mut app = app_with(|app| {
        switches(app);
        app.add_plugins(AdvancedPagePlugin { apply_saved });
        extra(app);
    });
    app.cleanup();
    frames(&mut app, 2);
    app
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

fn row(app: &mut App, key: &str) -> sf_ui_screens::OptionRow {
    let mut q = app.world_mut().query::<&OptionsScreen>();
    let screen = q.single(app.world()).expect("options screen");
    screen.model.row(key).unwrap_or_else(|| panic!("row {key}")).clone()
}

fn text(s: &str) -> SettingValue {
    SettingValue::Text(s.into())
}

fn send(app: &mut App, screen: Entity, change: OptionsChange) {
    app.world_mut().write_message(OptionsEvent { screen, change });
    frames(app, 3);
}

fn layer(app: &App, name: &str, by: SetBy) -> Option<Value> {
    let t = app.world().resource::<Tunables>();
    t.var(t.id(name).expect(name)).layer(by).cloned()
}

fn effective(app: &App, name: &str) -> Value {
    let t = app.world().resource::<Tunables>();
    t.get(t.id(name).expect(name)).clone()
}

fn set(app: &mut App, name: &str, value: Value, by: SetBy) {
    let mut t = app.world_mut().resource_mut::<Tunables>();
    let id = t.id(name).expect(name);
    t.set_by(id, value, by).expect("set");
    frames(app, 2);
}

/// Exactly the player, live switches get a row, by category (Shadows before Sky), page
/// `graphics_advanced`, `Machine` scope, the tunable's values plus `custom` (the default).
#[test]
fn the_page_lists_the_player_live_switches_only() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    switches(&mut app);
    let world = app.world();
    let registry = registry(world.resource::<FeatureSwitches>(), world.resource::<Tunables>());
    let rows: Vec<&str> = registry.settings().filter(|d| d.source.is_some()).map(|d| d.id.as_str()).collect();
    assert_eq!(rows, ["test.adv.shadows", "test.adv.fog"]);
    let fog = registry.settings().find(|d| d.id.as_str() == "test.adv.fog").expect("fog").def.clone();
    assert_eq!(fog.page.as_deref(), Some(PAGE));
    assert_eq!(fog.scope, SettingScope::Machine);
    assert_eq!(fog.group.as_deref(), Some("sky"));
    assert_eq!(fog.label, "set-feature-test-adv-fog");
    match fog.kind {
        SettingKind::Choice { default, options, .. } => {
            assert_eq!(default, CUSTOM_LEVEL);
            assert_eq!(options, ["off", "on", CUSTOM_LEVEL]);
        }
        other => panic!("{other:?}"),
    }
    let shadows = registry.settings().find(|d| d.id.as_str() == "test.adv.shadows").expect("shadows").def.clone();
    match shadows.kind {
        SettingKind::Choice { options, .. } => assert_eq!(options, ["off", "low", "high", CUSTOM_LEVEL]),
        other => panic!("{other:?}"),
    }
}

/// A pick sets the switch at `User` and it flips live; the row shows the effective value (a
/// preset's or the console's too) with the preset's as its default; Reset clears the `User` value.
#[test]
fn a_pick_sets_user_live_and_the_row_follows() {
    let mut app = ui_app(false, |_| {});
    let screen = open_options(&mut app);
    {
        let mut q = app.world_mut().query::<&OptionsScreen>();
        let screen = q.single(app.world()).expect("options screen");
        let ids: Vec<&str> = screen.model.pages.iter().map(|p| p.id.as_str()).collect();
        let at = ids.iter().position(|p| *p == PAGE).unwrap_or_else(|| panic!("{ids:?}"));
        assert!(ids.iter().position(|p| *p == crate::graphics_page::PAGE).is_none_or(|g| g < at), "after Graphics: {ids:?}");
        assert_eq!(screen.model.pages[at].label, "Advanced");
        let keys: Vec<&str> = screen.model.pages[at].rows.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(keys, ["test.adv.shadows", "test.adv.fog"]);
    }
    let fog = row(&mut app, "test.adv.fog");
    assert_eq!((fog.value, fog.default, fog.label.as_str()), (text("on"), text("on"), "Test fog"));
    assert_eq!(fog.description.as_deref(), Some("Test fog"), "the tunable's help");

    // A preset turns it off: the row shows it, as its default; nothing stored.
    set(&mut app, "test.adv.fog", Value::Bool(false), SetBy::Preset);
    let fog = row(&mut app, "test.adv.fog");
    assert_eq!((fog.value, fog.default), (text("off"), text("off")));

    // The player turns it on: `User`, live, stored (a pick, though `on` is the tunable's default).
    send(&mut app, screen, OptionsChange::Set("test.adv.fog".into(), text("on")));
    assert_eq!(layer(&app, "test.adv.fog", SetBy::User), Some(Value::Bool(true)));
    assert_eq!(effective(&app, "test.adv.fog"), Value::Bool(true));
    assert!(app.world().resource::<FeatureSwitches>().running(app.world().resource::<Tunables>(), "test.adv.fog"));
    assert_eq!(app.world().resource::<UiSettingsStore>().0.get("test.adv.fog"), Some(&text("on")));
    let fog = row(&mut app, "test.adv.fog");
    assert_eq!((fog.value, fog.default), (text("on"), text("off")), "the preset's value as the default");

    // A choice switch.
    send(&mut app, screen, OptionsChange::Set("test.adv.shadows".into(), text("low")));
    assert_eq!(effective(&app, "test.adv.shadows"), Value::Str("low".into()));
    // The console wins over the pick and shows.
    set(&mut app, "test.adv.shadows", Value::Str("off".into()), SetBy::Console);
    assert_eq!(row(&mut app, "test.adv.shadows").value, text("off"));

    // Reset: the `User` value cleared, the preset decides again.
    send(&mut app, screen, OptionsChange::Reset(vec!["test.adv.fog".into()]));
    assert_eq!(layer(&app, "test.adv.fog", SetBy::User), None);
    assert_eq!(effective(&app, "test.adv.fog"), Value::Bool(false));
    assert_eq!(app.world().resource::<UiSettingsStore>().0.get("test.adv.fog"), Some(&text(CUSTOM_LEVEL)));
    assert_eq!(row(&mut app, "test.adv.fog").value, text("off"));
}

/// A switch the app did not build this run: its row is disabled, with the reason.
#[test]
fn a_not_built_switch_row_is_disabled_with_its_reason() {
    let mut app = ui_app(false, |app| {
        app.mark_feature_not_built("test.adv.fog", "started with --off fog");
    });
    let _ = open_options(&mut app);
    let fog = row(&mut app, "test.adv.fog");
    assert!(!fog.enabled);
    assert_eq!(fog.disabled_reason.as_deref(), Some("started with --off fog"));
    assert!(row(&mut app, "test.adv.shadows").enabled);
}

/// Picks are `Machine` settings: written to the machine file when the menus close; at the next
/// start they are kept for the generated rows and apply at `User`.
#[test]
fn picks_persist_through_the_machine_file() {
    let dir = std::env::temp_dir().join(format!("bw-advanced-page-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("machine.toml");
    let mut app = ui_app(false, |_| {});
    sf_ui_bevy::load_machine_settings(app.world_mut(), file.clone()).expect("no file yet");
    let screen = open_options(&mut app);
    send(&mut app, screen, OptionsChange::Set("test.adv.shadows".into(), text("low")));
    send(&mut app, screen, OptionsChange::Set("test.adv.fog".into(), text("on")));
    tap(&mut app, KeyCode::Escape);
    tap(&mut app, KeyCode::Escape);
    assert!(!open(&app, OPTIONS));
    let saved = std::fs::read_to_string(&file).expect("machine file written");
    assert!(saved.contains("test.adv.shadows") && saved.contains("low") && saved.contains("test.adv.fog"), "{saved}");

    // A restart: the machine file is read before the rows exist (as `pause_menu::add`), the rows
    // take their values, the saved picks apply at `User`.
    let path = file.clone();
    let app = ui_app(true, move |app| {
        sf_ui_bevy::load_machine_settings(app.world_mut(), path).expect("machine file");
    });
    assert_eq!(app.world().resource::<UiSettingsStore>().0.get("test.adv.shadows"), Some(&text("low")));
    assert_eq!(layer(&app, "test.adv.shadows", SetBy::User), Some(Value::Str("low".into())));
    assert_eq!(layer(&app, "test.adv.fog", SetBy::User), Some(Value::Bool(true)));
    let _ = std::fs::remove_dir_all(&dir);
}
