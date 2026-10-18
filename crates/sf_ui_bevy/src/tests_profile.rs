//! Profile glue: input names, and settings + bindings surviving a restart
//! (load a profile, change values in a headless app, save, load again).

use std::path::Path;

use bevy::input::InputPlugin;
use bevy::input::gamepad::{GamepadAxis, GamepadButton};
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use semver::Version;
use sf_mods::{
    ContentAdapter, LoadedProfile, ModId, ModRepository, NativeRecords, Profile, load_profile,
};
use sf_ui::{SettingValue, UiRecordKinds};

use crate::*;

#[test]
fn binding_names_round_trip() {
    for (_, list) in &UiBindings::default().0 {
        for b in list {
            let name = binding_name(b).expect("every default has a name");
            assert_eq!(parse_binding(&name).as_ref(), Ok(b), "{name}");
        }
    }
    let ctrl_s = Binding::Keyboard {
        key: KeyCode::KeyS,
        mod_keys: ModKeys::CONTROL | ModKeys::SHIFT,
    };
    assert_eq!(
        binding_name(&ctrl_s).as_deref(),
        Some("key:Ctrl+Shift+KeyS")
    );
    assert_eq!(parse_binding("key:ctrl+shift+KeyS"), Ok(ctrl_s));
    assert_eq!(parse_binding("mouse:Left"), Ok(MouseButton::Left.into()));
    assert_eq!(
        parse_binding("axis:RightStickY"),
        Ok(GamepadAxis::RightStickY.into())
    );
    assert_eq!(parse_binding("mouse_wheel"), Ok(Binding::mouse_wheel()));
    assert_eq!(
        binding_name(&GamepadButton::Other(3).into()),
        None,
        "no name, not saved"
    );
    for bad in [
        "Enter",
        "key:Hyper+KeyS",
        "pad:Ctrl+South",
        "key:NotAKey",
        "joy:South",
        "key:Unidentified",
    ] {
        assert!(parse_binding(bad).is_err(), "{bad}");
    }
}

#[test]
fn bindings_table_holds_only_changes() {
    let mut b = UiBindings::default();
    assert!(b.to_table().is_empty());
    b.set(
        UiAction::Confirm,
        vec![KeyCode::Space.into(), GamepadButton::South.into()],
    );
    let table = b.to_table();
    assert_eq!(table.len(), 1);
    assert_eq!(
        table["confirm"],
        toml::Value::Array(vec!["key:Space".into(), "pad:South".into()])
    );

    let mut text: toml::Table =
        toml::from_str("open_map = [\"key:Tab\", \"pad:Nope\"]\nwhatever = []\nback = 3").unwrap();
    text.extend(table);
    let (read, issues) = UiBindings::from_table(&text);
    assert_eq!(issues.len(), 3, "{issues:?}");
    assert_eq!(read.get(UiAction::Confirm), b.get(UiAction::Confirm));
    assert_eq!(
        read.get(UiAction::OpenMap),
        [Binding::from(KeyCode::Tab)],
        "bad entry dropped, rest kept"
    );
    assert_eq!(
        read.get(UiAction::Back),
        UiBindings::default().get(UiAction::Back)
    );
}

const RECORDS: &str = r#"
[[define]]
kind = "ui.screen"
id = "options"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "options" } }

[[define]]
kind = "ui.setting"
id = "game.text_scale"
value = { label = "set-text-scale", kind = { Float = { default = 1.0, min = 1.0, max = 2.0, step = 0.25 } } }
"#;

fn write_game_mod(dir: &Path) {
    let root = dir.join("game");
    std::fs::create_dir_all(root.join("data/records")).unwrap();
    std::fs::create_dir_all(root.join("data/l10n/en")).unwrap();
    std::fs::write(
        root.join(sf_mods::MANIFEST_FILE),
        "id = \"game\"\nname = \"game\"\nversion = \"1.0.0\"\ndescription = \"test\"\n\
         [content]\nassets = [\"data\"]\nrecords = [\"records/ui.toml\"]\n",
    )
    .unwrap();
    std::fs::write(root.join("data/records/ui.toml"), RECORDS).unwrap();
    std::fs::write(
        root.join("data/l10n/en/game.ftl"),
        "set-text-scale = Text size\n",
    )
    .unwrap();
}

fn load(mods: &Path, profile: &Profile) -> LoadedProfile {
    let adapters: [&dyn ContentAdapter; 2] = [&UiRecordKinds, &NativeRecords];
    let repo = ModRepository::scan(&[mods.to_path_buf()], &adapters);
    let loaded = load_profile(&repo, profile, &adapters, &Version::new(0, 1, 0));
    assert!(
        !loaded.report.has_errors(),
        "{}",
        loaded.render_report(None, 10)
    );
    loaded
}

fn start_app(loaded: &LoadedProfile, dir: &Path) -> (App, UiLoadReport) {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin, SfUiPlugin));
    let locales = UiLocales {
        requested: vec!["en-GB".into()],
        default: "en".parse().unwrap(),
        use_isolating: false,
    };
    let report = insert_from_profile(app.world_mut(), loaded, &locales, Some(dir.to_path_buf()));
    app.finish();
    app.update();
    (app, report)
}

#[test]
fn settings_and_bindings_survive_a_restart() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    let profiles = tmp.path().join("profiles");
    write_game_mod(&mods);
    let profile = Profile::new("player", [ModId::new("game").unwrap()]);
    let saved = Profile::path_in(&profiles, "player");

    let loaded = load(&mods, &profile);
    let (mut app, report) = start_app(&loaded, &profiles);
    assert!(report.conflicts.is_empty() && report.strings.is_empty() && report.settings.is_empty());
    assert!(report.bindings.is_empty());
    assert_eq!(
        app.world()
            .resource::<UiLocalizer>()
            .0
            .format("set-text-scale", None),
        "Text size"
    );

    // The player opens the options screen and changes a setting and a binding.
    let world = app.world_mut();
    let registry = world.remove_resource::<UiRegistryRes>().unwrap();
    world
        .resource_mut::<ScreenStackRes>()
        .0
        .push(&registry.0, "options")
        .unwrap();
    world.insert_resource(registry);
    world
        .resource_mut::<UiSettingsStore>()
        .0
        .set("game.text_scale", SettingValue::Float(1.5))
        .unwrap();
    world.resource_mut::<UiBindings>().set(
        UiAction::OpenMenu,
        vec![KeyCode::F10.into(), GamepadButton::Start.into()],
    );
    app.update();
    assert!(app.world().resource::<UiProfile>().dirty);
    assert!(!saved.exists(), "not written while the menu is open");

    // Leaving the menu writes the profile once.
    app.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .pop(Layer::Menu);
    app.update();
    let ui_profile = app.world().resource::<UiProfile>();
    assert!(
        !ui_profile.dirty && ui_profile.last_error.is_none(),
        "{:?}",
        ui_profile.last_error
    );
    assert!(saved.exists());

    // Restart: read the file, load the mods again, build the UI again.
    let reread = Profile::load(&saved).unwrap();
    assert_eq!(
        reread.bindings[BINDINGS_CONTEXT]["open_menu"],
        toml::Value::Array(vec!["key:F10".into(), "pad:Start".into()])
    );
    let loaded = load(&mods, &reread);
    let (app, report) = start_app(&loaded, &profiles);
    assert!(report.settings.is_empty() && report.bindings.is_empty());
    let world = app.world();
    assert_eq!(
        world
            .resource::<UiSettingsStore>()
            .0
            .get_f64("game.text_scale"),
        Some(1.5)
    );
    assert_eq!(
        world.resource::<UiBindings>().get(UiAction::OpenMenu),
        [
            Binding::from(KeyCode::F10),
            Binding::from(GamepadButton::Start)
        ]
    );
    assert_eq!(
        world.resource::<UiBindings>().get(UiAction::Confirm),
        UiBindings::default().get(UiAction::Confirm)
    );
}

#[test]
fn ui_commands_enforce_the_active_preset() {
    use bevy::ecs::system::RunSystemOnce;
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    write_game_mod(&mods);
    let records = mods.join("game/data/records/ui.toml");
    let extra = "\n[[define]]\nkind = \"ui.screen\"\nid = \"window\"\n\
                 value = { layer = \"Menu\", implementation = { Template = \"w\" }, presets = [\"pc_dense\"] }\n";
    std::fs::write(&records, format!("{RECORDS}{extra}")).unwrap();
    let loaded = load(&mods, &Profile::new("p", [ModId::new("game").unwrap()]));
    let (mut app, _) = start_app(&loaded, tmp.path());
    app.world_mut().resource_mut::<UiSettings>().preset = "controller".into();
    let opened = app
        .world_mut()
        .run_system_once(|mut ui: UiCommands| {
            (ui.push("window").is_ok(), ui.push("options").is_ok())
        })
        .unwrap();
    assert_eq!(opened, (false, true));
    app.world_mut().resource_mut::<UiSettings>().preset = "pc_dense".into();
    let top = app
        .world_mut()
        .run_system_once(|mut ui: UiCommands| ui.push("window").map(|s| s.id.0.clone()))
        .unwrap();
    assert_eq!(top.as_deref(), Ok("window"));
}
