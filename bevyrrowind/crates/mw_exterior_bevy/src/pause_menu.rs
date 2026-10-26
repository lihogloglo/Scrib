//! The pause (escape) menu (superfargoth `ui.md` "Stock screens"): the stock `sf_ui_screens`
//! pause menu and, on top of it, the stock options screen.
//!
//! - **Open**: the rebindable UI "open menu" action (`sf_ui_bevy::UiAction::OpenMenu`: Esc,
//!   gamepad Start) in player mode. UI actions reach only the stack's input target, so the
//!   game keeps a HUD-layer screen ("hud") open as the target of play: with no menu open, Esc
//!   is that screen's open-menu action; with a menu (dialogue, journal, inventory, keybinds)
//!   open, Esc is the menu's Back and closes it first. The console blocks the keys while open.
//!   `mw_player_bevy::hud` spawns the `HudScreen` root on the same record and draws the HUD.
//! - **Pause**: the "pause" and "options" records declare `pause = "WhileOpen"`; while
//!   `ScreenStack::pauses_game` holds, Bevy's virtual clock is paused (`Time<Virtual>`, the
//!   engine's pause), so the fixed-step physics, animation, the sky clock and NPCs stop. The
//!   player's controls pause through `dialogue.rs`'s menu seam (`PlayerMode::paused`, any
//!   Menu-layer screen).
//! - **Choices**: Resume (or Back) closes the menu (the screen does); Quit, after its yes/no,
//!   exits (`AppExit::Success`); Options opens the options screen on top (its Back returns
//!   here); Save and Load open the save/load screen on top (`mw_saves_bevy::menu`, which lists
//!   the save folder and carries the choices out; its Back returns here).
//! - **Options**: the settings store (`UiSettingsStore`, every `ui.setting` record, values from
//!   the player's profile `profiles/<name>.toml` `[settings.<mod>]`, only non-defaults) is
//!   inserted here at startup; strings come from the builtin mod's Fluent file (`controls.rs`).
//!   A change is written to the store at once and applies live where a consumer reads the store
//!   (the framework's audio page: `sf_audio_bevy` `SettingsAudioPlugin` in `mw_audio_bevy`);
//!   other settings are stored only (logged). The profile file is written when the player
//!   leaves the menus, or on exit (`sf_ui_bevy` `sync_profile`, console/PC convention: not
//!   once per slider step). "Key bindings" opens the keybinds screen on top (`controls.rs`);
//!   its Back returns to the options.

use bevy::prelude::*;
use sf_console_bevy::ConsoleOpen;
use sf_records::RecordDb;
use sf_ui::{Layer, SettingsStore, UiRegistry};
use sf_ui_bevy::{ScreenStackRes, UiAction, UiActionEvent, UiLocalizer, UiRegistryRes, UiScreenRoot, UiSettingsStore, UiSystems};
use sf_ui_screens::{
    OptionsChange, OptionsEvent, OptionsModel, OptionsScreen, PauseChoice, PauseMenuEvent, PauseMenuModel, PauseMenuScreen, SaveLoadMode,
    ScreenSystems,
};

use mw_player_bevy::PlayerMode;
use mw_saves_bevy::OpenSaveLoad;

/// The HUD screen: the input target of play (carries the open-menu action).
pub const HUD: &str = mw_player_bevy::hud::HUD_SCREEN;
/// The pause menu's screen id.
pub const PAUSE: &str = "pause";
/// The options screen's id.
pub const OPTIONS: &str = "options";

/// The game's records of these screens. TODO(ui data): with the other screens, in the
/// builtin mod's data.
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "pause"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "pause" } }

[[define]]
kind = "ui.screen"
id = "options"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "options" } }
"#;

/// After `dialogue::add` (the UI plugins and the game's screen registry).
pub fn add(app: &mut App) {
    let world = app.world_mut();
    let Some(mut registry) = world.get_resource_mut::<UiRegistryRes>() else { return };
    registry.0.extend_screens(&screens());
    if !world.contains_resource::<UiSettingsStore>() {
        let profile = mw_boot_bevy::mods::bindings_profile();
        let store = settings_store(&world.resource::<UiRegistryRes>().0, profile.as_ref().map(|(_, p)| p));
        world.insert_resource(store);
    }
    app.add_systems(
        Update,
        (
            (keep_hud, open_pause_menu).chain().after(UiSystems::Dispatch).before(ScreenSystems::Spawn),
            (host_pause_menu, host_options, close_screens, pause_time).chain().after(ScreenSystems::Input),
        ),
    );
    // The save/load screen's request (`mw_saves_bevy::menu` hosts it and registers it too).
    app.add_message::<OpenSaveLoad>();
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("pause screens");
    assert!(outcome.errors.is_empty(), "pause screens: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// The pause menu's model: every entry.
fn model() -> PauseMenuModel {
    PauseMenuModel::default()
}

/// Keeps the HUD screen open (a load clears only the Menu and Modal layers).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn keep_hud(mut stack: ResMut<ScreenStackRes>, registry: Res<UiRegistryRes>) {
    if !stack.0.is_open(HUD)
        && let Err(e) = stack.0.push(&registry.0, HUD)
    {
        warn!("pause menu: {e:?}");
    }
}

/// The open-menu action on the HUD opens the pause menu in player mode.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_pause_menu(
    mut commands: Commands,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    mode: Option<Res<PlayerMode>>,
    console: Option<Res<ConsoleOpen>>,
) {
    let pressed = actions.read().any(|a| a.action == UiAction::OpenMenu && a.screen.as_str() == HUD);
    if !pressed || !mode.is_some_and(|m| m.on) || stack.0.top(Layer::Menu).is_some() {
        return;
    }
    // Esc that just closed the console isn't a menu press.
    if console.is_some_and(|c| c.0 || c.is_changed()) {
        return;
    }
    commands.spawn((UiScreenRoot(PAUSE.into()), PauseMenuScreen::new(model())));
    if let Err(e) = stack.0.push(&registry.0, PAUSE) {
        warn!("pause menu: {e:?}");
    }
}

/// The menu's choices. Resume has already closed the screen.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn host_pause_menu(
    mut commands: Commands,
    mut choices: MessageReader<PauseMenuEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    store: Option<Res<UiSettingsStore>>,
    localizer: Option<Res<UiLocalizer>>,
    mut exit: MessageWriter<AppExit>,
    mut open_saves: MessageWriter<OpenSaveLoad>,
) {
    for event in choices.read() {
        match event.choice {
            PauseChoice::Quit => {
                info!("pause menu: quit");
                exit.write(AppExit::Success);
            }
            PauseChoice::Options => open_options(&mut commands, &mut stack, &registry, store.as_deref(), localizer.as_deref()),
            // Resume: the screen has closed itself.
            PauseChoice::Resume => {}
            PauseChoice::Save => {
                open_saves.write(OpenSaveLoad(SaveLoadMode::Save));
            }
            PauseChoice::Load => {
                open_saves.write(OpenSaveLoad(SaveLoadMode::Load));
            }
        }
    }
}

/// Opens the options screen on top (its Back returns to the menu under it), unless it is open.
/// The pause menu and the main menu (`main_menu.rs`) both open it here.
pub(crate) fn open_options(
    commands: &mut Commands,
    stack: &mut ScreenStackRes,
    registry: &UiRegistryRes,
    store: Option<&UiSettingsStore>,
    localizer: Option<&UiLocalizer>,
) {
    if stack.0.is_open(OPTIONS) {
        return;
    }
    let model = OptionsModel::from_settings(&registry.0, store.map(|s| &s.0), localizer.map(|l| &l.0));
    commands.spawn((UiScreenRoot(OPTIONS.into()), OptionsScreen::new(model)));
    if let Err(e) = stack.0.push(&registry.0, OPTIONS) {
        warn!("options: {e:?}");
    }
}

/// The settings store of every `ui.setting` in `registry`, with the values `profile` saved.
pub(crate) fn settings_store(registry: &UiRegistry, profile: Option<&sf_mods::Profile>) -> UiSettingsStore {
    let mut store = SettingsStore::new(registry);
    for issue in profile.map(|p| store.load_from(p)).unwrap_or_default() {
        warn!("settings: {issue:?}");
    }
    UiSettingsStore(store)
}

/// Settings with a live consumer of the store (the audio mixer reads the volume page).
fn applies_live(key: &str) -> bool {
    key.starts_with("sf.audio_")
}

/// Option changes go to the store (saved into the profile when the menus close, consumers
/// apply them); Key bindings opens the keybinds screen on top. Back has closed the screen.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn host_options(
    mut commands: Commands,
    mut events: MessageReader<OptionsEvent>,
    mut store: Option<ResMut<UiSettingsStore>>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
) {
    for event in events.read() {
        match &event.change {
            OptionsChange::Back => {}
            OptionsChange::Set(key, value) => {
                let Some(store) = store.as_mut() else {
                    warn!("options: no settings store, {key} not kept");
                    continue;
                };
                match store.0.set(key, value.clone()) {
                    Ok(value) if applies_live(key) => info!("options: {key} = {value:?}"),
                    Ok(value) => info!("options: {key} = {value:?} (stored; nothing applies it live yet)"),
                    Err(e) => warn!("options: {key}: {e:?}"),
                }
            }
            OptionsChange::OpenKeybinds => mw_controls_bevy::open_keybinds(&mut commands, &mut stack, &registry),
        }
    }
}

/// Roots of the pause menu and the options screen despawn once off the stack.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_screens(mut commands: Commands, stack: Res<ScreenStackRes>, roots: Query<(Entity, &UiScreenRoot)>) {
    for (e, root) in &roots {
        let id = root.0.as_str();
        if matches!(id, PAUSE | OPTIONS) && !stack.0.is_open(id) {
            commands.entity(e).despawn();
        }
    }
}

/// The virtual clock pauses while a pausing screen is open. Only on the stack's edges, so a
/// pause set elsewhere isn't overridden every frame.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn pause_time(stack: Res<ScreenStackRes>, mut time: ResMut<Time<Virtual>>, mut was: Local<bool>) {
    let pauses = stack.0.pauses_game();
    if pauses == *was {
        return;
    }
    *was = pauses;
    if pauses {
        time.pause();
    } else {
        time.unpause();
    }
}

#[cfg(test)]
pub(crate) mod tests {
    //! Headless: UI layout and text without a window (a camera with a fixed target), Esc and
    //! the arrows through `ButtonInput`, as `sf_ui_screens`'s screen tests drive them.
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::input::InputPlugin;
    use bevy::picking::{InteractionPlugin, PickingPlugin};
    use bevy::text::TextPlugin;
    use bevy::ui::UiPlugin;
    use bevy_enhanced_input::prelude::EnhancedInputPlugin;
    use sf_ui_bevy::{SfUiPlugin, UiTextPlugin};
    use sf_ui_screens::{SaveLoadChoice, SaveLoadEvent, SaveLoadModel, SaveLoadScreen, SfScreensPlugin};

    use super::*;

    fn app() -> App {
        app_with(|_| {})
    }

    /// The test app, `extra` added before it finishes.
    pub(crate) fn app_with(extra: impl FnOnce(&mut App)) -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            AssetPlugin::default(),
            ImagePlugin::default(),
            bevy::image::TextureAtlasPlugin,
            TextPlugin,
            UiPlugin,
            InputPlugin,
            PickingPlugin,
            InteractionPlugin,
            EnhancedInputPlugin,
            SfUiPlugin,
            UiTextPlugin,
            SfScreensPlugin,
        ))
        .insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None })
        // A built registry, as `dialogue.rs` inserts (the framework's built-in settings).
        .insert_resource(UiRegistryRes(UiRegistry::build(&RecordDb::new()).0));
        add(&mut app);
        // The keybinds screen (Key bindings) and the builtin strings, as `player.rs` adds them.
        mw_controls_bevy::add(&mut app);
        extra(&mut app);
        let target = UVec2::new(1600, 1000);
        app.world_mut().spawn((
            Camera2d,
            Camera {
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo { physical_size: target, scale_factor: 1.0 }),
                    ..default()
                },
                viewport: Some(Viewport { physical_size: target, ..default() }),
                ..default()
            },
        ));
        app.finish();
        frames(&mut app, 3);
        app
    }

    pub(crate) fn frames(app: &mut App, n: usize) {
        for _ in 0..n {
            app.update();
        }
    }

    pub(crate) fn tap(app: &mut App, key: KeyCode) {
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(key);
        app.update();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(key);
        frames(app, 3);
    }

    pub(crate) fn open(app: &App, id: &str) -> bool {
        app.world().resource::<ScreenStackRes>().0.is_open(id)
    }

    pub(crate) fn paused(app: &App) -> bool {
        app.world().resource::<Time<Virtual>>().is_paused()
    }

    pub(crate) fn roots(app: &mut App, id: &str) -> usize {
        let mut q = app.world_mut().query::<&UiScreenRoot>();
        q.iter(app.world()).filter(|r| r.0.as_str() == id).count()
    }

    #[test]
    fn esc_opens_the_pause_menu_and_resume_closes_it() {
        let mut app = app();
        assert!(open(&app, HUD) && !open(&app, PAUSE) && !paused(&app));

        tap(&mut app, KeyCode::Escape);
        assert!(open(&app, PAUSE), "Esc in player mode opens the pause menu");
        assert_eq!(roots(&mut app, PAUSE), 1);
        assert!(paused(&app), "the game pauses while it is open");
        let mut q = app.world_mut().query::<&PauseMenuScreen>();
        let screen = q.single(app.world()).unwrap();
        assert!(PauseChoice::ALL.iter().all(|c| screen.model.is_enabled(*c)), "every entry, Save and Load too");

        // Esc again is the menu's Back: resume.
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, PAUSE), "Back resumes");
        assert_eq!(roots(&mut app, PAUSE), 0);
        assert!(!paused(&app), "the game runs again");
        assert!(open(&app, HUD));
    }

    #[test]
    fn not_in_free_camera() {
        let mut app = app();
        app.world_mut().resource_mut::<PlayerMode>().on = false;
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, PAUSE) && !paused(&app));
    }

    #[test]
    fn options_opens_on_top_and_quit_exits_after_yes() {
        let mut app = app();
        tap(&mut app, KeyCode::Escape);
        // Resume -> Save -> Load -> Options.
        for _ in 0..3 {
            tap(&mut app, KeyCode::ArrowDown);
        }
        tap(&mut app, KeyCode::Enter);
        assert!(open(&app, OPTIONS) && open(&app, PAUSE), "options on top of the menu");
        assert_eq!(roots(&mut app, OPTIONS), 1);
        assert!(paused(&app));
        // Esc: options Back, back to the menu, still paused.
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, OPTIONS) && open(&app, PAUSE));
        assert_eq!(roots(&mut app, OPTIONS), 0);
        assert!(paused(&app));
        assert!(app.should_exit().is_none());

        // Options -> Quit; it asks (No focused); left to Yes; confirm.
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(app.should_exit().is_none(), "Quit asks first");
        tap(&mut app, KeyCode::ArrowLeft);
        tap(&mut app, KeyCode::Enter);
        assert_eq!(app.should_exit(), Some(AppExit::Success));
    }

    fn master_volume(app: &App) -> Option<f64> {
        app.world().resource::<UiSettingsStore>().0.get_f64(&sf_ui::audio_volume_key("master"))
    }

    /// Options -> step the master volume: the store changes at once, the profile file gets it
    /// once the menus close, and a fresh store from that file has it (a restart). Key bindings
    /// opens the keybinds screen on top; its Back returns to the options.
    #[test]
    fn options_change_the_store_and_persist_in_the_profile() {
        let dir = std::env::temp_dir().join(format!("bw-options-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp profiles dir");
        let mut app = app();
        app.insert_resource(sf_ui_bevy::UiProfile {
            profile: sf_mods::Profile::new("test", []),
            dir: Some(dir.clone()),
            dirty: false,
            last_error: None,
        });
        assert_eq!(master_volume(&app), Some(1.0), "the default");

        tap(&mut app, KeyCode::Escape);
        for _ in 0..3 {
            tap(&mut app, KeyCode::ArrowDown);
        }
        tap(&mut app, KeyCode::Enter);
        assert!(open(&app, OPTIONS));
        let mut q = app.world_mut().query::<(Entity, &OptionsScreen)>();
        let (screen, options) = q.single(app.world()).expect("options screen");
        assert_eq!(options.model.pages[0].label, "Audio", "page label from the builtin strings");
        let master = sf_ui::audio_volume_key("master");
        assert_eq!(options.model.row(&master).expect("master row").label, "Master Volume");

        // The focus starts on the first row (master volume): left steps it down.
        tap(&mut app, KeyCode::ArrowLeft);
        let stepped = master_volume(&app).expect("master volume");
        assert!((stepped - (1.0 - sf_ui::AUDIO_VOLUME_STEP)).abs() < 1e-9, "store follows the slider: {stepped}");
        let file = sf_mods::Profile::path_in(&dir, "test");
        assert!(!file.exists(), "not written while the menus are open");

        // Key bindings on top of the options; its Back returns to them.
        app.world_mut().write_message(OptionsEvent { screen, change: OptionsChange::OpenKeybinds });
        frames(&mut app, 3);
        let keybinds = mw_controls_bevy::KEYBINDS_SCREEN;
        assert!(open(&app, keybinds) && open(&app, OPTIONS), "keybinds on top of the options");
        assert_eq!(roots(&mut app, keybinds), 1);
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, keybinds) && open(&app, OPTIONS), "Back returns to the options");
        assert_eq!(roots(&mut app, keybinds), 0);

        // Leave the menus: the profile is saved, with only the changed value.
        tap(&mut app, KeyCode::Escape);
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, OPTIONS) && !open(&app, PAUSE));
        let text = std::fs::read_to_string(&file).expect("profile written");
        assert!(text.contains("audio_master_volume") && !text.contains("audio_music_volume"), "{text}");

        // A restart: a fresh store from the saved file has the value.
        let saved = mw_controls_bevy::reload(&dir, "test");
        let store = settings_store(&app.world().resource::<UiRegistryRes>().0, Some(&saved));
        assert_eq!(store.0.get_f64(&master), Some(stepped));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The open save/load screen.
    fn saveload(app: &mut App) -> (Entity, SaveLoadModel) {
        let mut q = app.world_mut().query::<(Entity, &SaveLoadScreen)>();
        let (e, screen) = q.single(app.world()).expect("save/load screen");
        (e, screen.model.clone())
    }

    fn choose(app: &mut App, choice: SaveLoadChoice) {
        let (screen, _) = saveload(app);
        app.world_mut().write_message(SaveLoadEvent { screen, choice });
    }

    /// Pause menu -> Save -> New: a manual save in the world's folder, listed; Back, Load lists it;
    /// Load -> that entry loads it and closes the menus; Delete removes the file and the row.
    #[test]
    fn save_new_load_and_delete_from_the_pause_menu() {
        use bevy::ecs::message::Messages;
        use mw_saves_bevy::SAVELOAD;
        use sf_save_bevy::{LoadDone, SaveConfig, SfSavePlugin};

        let root = std::env::temp_dir().join(format!("bw-saveload-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let config = SaveConfig { root: root.clone(), background: false, ..default() };
        let mut app = app_with(|app| {
            app.add_plugins(SfSavePlugin { config })
                .insert_resource(sf_world_bevy::WorldOverlay::default())
                .insert_resource(sf_sky_bevy::SkyTime::default())
                .insert_resource(sf_world_bevy::ActiveWorldspace(Some(sf_world::WorldspaceId::new("Seyda Neen, Census and Excise Office"))));
            mw_saves_bevy::register(app);
            mw_saves_bevy::menu::add(app);
        });
        app.world_mut().resource_mut::<sf_sky_bevy::SkyTime>().0.days = 2.75;

        // Esc -> Save: the screen on top of the menu, nothing saved yet.
        tap(&mut app, KeyCode::Escape);
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(open(&app, SAVELOAD) && open(&app, PAUSE) && paused(&app));
        let (_, model) = saveload(&mut app);
        assert_eq!(model.mode, SaveLoadMode::Save);
        assert!(model.entries.is_empty());
        assert_eq!(model.new_name, "Day 3 - 18 Last Seed 6 p.m.", "OpenMW's default name");

        // New: one manual save, listed again with its place and in-game date.
        choose(&mut app, SaveLoadChoice::New(model.new_name.clone()));
        frames(&mut app, 3);
        let (_, model) = saveload(&mut app);
        assert_eq!(model.entries.len(), 1, "{:?}", model.entries);
        let entry = &model.entries[0];
        assert_eq!((entry.id.as_str(), entry.order), ("000001-manual.sfsave", 1));
        assert_eq!(entry.title, "seyda neen, census and excise office", "the place (the worldspace id without records)");
        assert!(entry.subtitle.starts_with("Day 3 - 18 Last Seed 6 p.m. · "), "{}", entry.subtitle);
        assert!(entry.reason_label.is_empty() && !entry.damaged);
        let world_id = app.world().resource::<sf_save_bevy::SaveSession>().world_id;
        let file = sf_save_bevy::sf_save::SlotDir::world(&root, sf_save_bevy::sf_save::SaveMode::Single, world_id).dir().join("000001-manual.sfsave");
        assert!(file.is_file(), "{}", file.display());

        // Back to the menu; Load lists the save.
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, SAVELOAD) && open(&app, PAUSE));
        assert_eq!(roots(&mut app, SAVELOAD), 0);
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        let (_, model) = saveload(&mut app);
        assert_eq!(model.mode, SaveLoadMode::Load);
        assert_eq!(model.entries.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), ["000001-manual.sfsave"]);

        // Load it: done, the menus close and the game runs.
        choose(&mut app, SaveLoadChoice::Load("000001-manual.sfsave".into()));
        app.update();
        let loaded: Vec<_> = app.world_mut().resource_mut::<Messages<LoadDone>>().drain().collect();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].0.as_ref().expect("loaded").path, file);
        frames(&mut app, 3);
        assert!(!open(&app, SAVELOAD) && !open(&app, PAUSE) && !paused(&app));
        assert_eq!(roots(&mut app, SAVELOAD), 0);

        // Esc -> Load -> Delete: the file is gone and the list is empty.
        tap(&mut app, KeyCode::Escape);
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert_eq!(saveload(&mut app).1.entries.len(), 1);
        choose(&mut app, SaveLoadChoice::Delete("000001-manual.sfsave".into()));
        frames(&mut app, 2);
        assert!(!file.exists());
        assert!(saveload(&mut app).1.entries.is_empty(), "listed again");
        let _ = std::fs::remove_dir_all(&root);
    }
}
