//! `SettingChanged` and the machine settings file (menus.md "Settings v2").

use std::path::Path;

use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{SettingKey, SettingValue, SettingsStore, UiRegistry};

use crate::*;

/// `m.res` (Machine choice) and `m.subs` (Profile bool).
fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).unwrap();
    let src = db.add_source("m", &[]);
    let text = r#"
[[define]]
kind = "ui.setting"
id = "m.res"
value = { label = "res", kind = { Choice = { default = "1080p", options = ["1080p", "1440p"] } }, scope = "Machine" }

[[define]]
kind = "ui.setting"
id = "m.subs"
value = { label = "subs", kind = { Bool = { default = false } } }
"#;
    let outcome = apply_record_file(&mut db, src, text, RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    UiRegistry::build(&db).0
}

#[derive(Resource, Default)]
struct Seen(Vec<SettingChanged>);

fn collect(mut changes: MessageReader<SettingChanged>, mut seen: ResMut<Seen>) {
    seen.0.extend(changes.read().cloned());
}

fn start(machine_file: Option<&Path>) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin, SfUiPlugin))
        .init_resource::<Seen>()
        .add_systems(Last, collect);
    app.insert_resource(UiSettingsStore(SettingsStore::new(&registry())));
    if let Some(path) = machine_file {
        let _ = load_machine_settings(app.world_mut(), path.to_path_buf());
    }
    app.finish();
    app.update();
    app
}

fn set(app: &mut App, key: &str, value: SettingValue) {
    app.world_mut()
        .resource_mut::<UiSettingsStore>()
        .0
        .set(key, value)
        .unwrap();
}

fn seen(app: &mut App) -> Vec<SettingChanged> {
    std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0)
}

fn text(s: &str) -> SettingValue {
    SettingValue::Text(s.into())
}

#[test]
fn a_change_sends_one_setting_changed_and_the_same_value_none() {
    let mut app = start(None);
    assert_eq!(seen(&mut app), [], "defaults send nothing");

    set(&mut app, "m.subs", SettingValue::Bool(true));
    app.update();
    assert_eq!(
        seen(&mut app),
        [SettingChanged {
            key: SettingKey("m.subs".into()),
            old: Some(SettingValue::Bool(false)),
            new: SettingValue::Bool(true),
        }]
    );
    app.update();
    assert_eq!(seen(&mut app), [], "sent once");

    set(&mut app, "m.subs", SettingValue::Bool(true));
    app.update();
    assert_eq!(seen(&mut app), [], "the same value is no change");

    // A writer inside the frame (any system before PostUpdate) arrives the same frame.
    app.add_systems(Update, |mut store: ResMut<UiSettingsStore>| {
        store
            .0
            .set("m.res", SettingValue::Text("1440p".into()))
            .unwrap();
    });
    app.update();
    assert_eq!(
        seen(&mut app),
        [SettingChanged {
            key: SettingKey("m.res".into()),
            old: Some(text("1080p")),
            new: text("1440p"),
        }]
    );
}

#[test]
fn machine_settings_save_to_the_machine_file_and_load_back() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("user").join("machine.toml");
    let mut app = start(Some(&path));
    assert!(
        app.world()
            .resource::<UiMachineSettings>()
            .last_error
            .is_none()
    );

    set(&mut app, "m.subs", SettingValue::Bool(true));
    app.update();
    assert!(
        !path.exists(),
        "a Profile change doesn't write the machine file"
    );

    set(&mut app, "m.res", text("1440p"));
    app.update();
    let machine = app.world().resource::<UiMachineSettings>();
    assert!(
        !machine.dirty && machine.last_error.is_none(),
        "{:?}",
        machine.last_error
    );
    let file: toml::Table = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        file,
        toml::from_str::<toml::Table>("[m]\nres = \"1440p\"").unwrap(),
        "only the Machine key"
    );

    // Restart: the machine file restores the Machine value, the Profile one stays default
    // (it comes from the profile).
    let app = start(Some(&path));
    let store = &app.world().resource::<UiSettingsStore>().0;
    assert_eq!(store.get("m.res"), Some(&text("1440p")));
    assert_eq!(store.get_bool("m.subs"), Some(false));
}

#[test]
fn a_corrupt_machine_file_loads_defaults_and_reports_it() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("machine.toml");
    std::fs::write(&path, "[m\nres = ").unwrap();

    let mut world = World::new();
    world.insert_resource(UiSettingsStore(SettingsStore::new(&registry())));
    let err = load_machine_settings(&mut world, path.clone()).unwrap_err();
    assert!(
        matches!(err, sf_ui::MachineFileError::Parse { .. }),
        "{err}"
    );
    assert_eq!(
        world.resource::<UiSettingsStore>().0.get("m.res"),
        Some(&text("1080p"))
    );
    let machine = world.resource::<UiMachineSettings>();
    assert_eq!(
        machine.last_error.as_deref(),
        Some(err.to_string().as_str())
    );
    assert_eq!(machine.path, path);
}
