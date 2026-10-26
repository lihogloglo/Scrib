//! The main (title) menu of a release start (ROADMAP 2.2, visual check MM-1): the stock
//! `sf_ui_screens` main menu over the world at the new-game start, which streams in behind it.
//!
//! - **Open**: once at startup when `ExteriorOptions::start_in_main_menu` is set (running
//!   `bevyrrowind` with no subcommand). Dev runs (`view`, `scene`) never open it.
//! - **Pause**: the "`main_menu`" record declares `pause = "WhileOpen"`, so the virtual clock is
//!   paused while it is open (`pause_menu.rs` `pause_time`, the pause menu's mechanism) and the
//!   player's controls pause through the Menu-layer seam; the HUD hides under Menu screens.
//! - **Entries**, in Morrowind's order (`OpenMW` `apps/openmw/mwgui/mainmenu.cpp`, commit
//!   675146bd: return/new game/save/load/options/credits/exit): Continue, New game, Load,
//!   Options, Quit. Continue and Load are enabled while a single-player save of any world exists
//!   (checked at open and again when the save/load screen closes, a delete may have emptied it).
//!   Continue loads the newest save of any reason (`mw_saves_bevy::newest_save`, quickload's
//!   search; `OpenMW`'s "return" does the same); Load opens the save/load screen in Load mode on
//!   top (`mw_saves_bevy::menu`; its Back pops it, so this menu is the top again). A finished load
//!   clears the Menu layer (`mw_saves_bevy` `close_menus`), so the menu closes and play runs. New
//!   game closes the menu and play begins; Options opens the options screen on top as the pause
//!   menu does ([`crate::pause_menu::open_options`]); Quit asks (the screen), then exits.
//!   Labels come from the builtin mod's Fluent file (`main-menu-<id>`).

use bevy::prelude::*;
use mw_saves_bevy::{OpenSaveLoad, SAVELOAD};
use sf_records::RecordDb;
use sf_save_bevy::{LoadRequest, SaveConfig, SaveStorage};
use sf_ui::UiRegistry;
use sf_ui_bevy::{ScreenStackRes, UiLocalizer, UiRegistryRes, UiScreenRoot, UiSettingsStore};
use sf_ui_screens::{MainMenuEntry, MainMenuEvent, MainMenuModel, MainMenuScreen, SaveLoadMode, ScreenSystems};

/// The main menu's screen id.
pub const MAIN_MENU: &str = "main_menu";

/// Entry ids (also the Fluent keys' suffixes).
pub const CONTINUE: &str = "continue";
pub const NEW_GAME: &str = "new";
pub const LOAD: &str = "load";
pub const OPTIONS: &str = "options";
pub const QUIT: &str = "quit";

/// The game's record of the screen. TODO(ui data): with the other screens, in the builtin
/// mod's data.
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "main_menu"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "main_menu" } }
"#;

/// After `pause_menu::add`. `open`: open the menu at startup (the release start).
pub fn add(app: &mut App, open: bool) {
    if !open {
        return;
    }
    let Some(mut registry) = app.world_mut().get_resource_mut::<UiRegistryRes>() else { return };
    registry.0.extend_screens(&screens());
    // Continue's request and Load's (the save plugin and `pause_menu` register them too).
    app.add_message::<LoadRequest>().add_message::<OpenSaveLoad>();
    app.add_systems(Startup, open_main_menu).add_systems(
        Update,
        (host_main_menu, refresh_saves, close_main_menu).chain().after(ScreenSystems::Input).before(crate::pause_menu::pause_time),
    );
}

/// The newest save of any world and reason (Continue's target); none without the save plugin.
fn newest(storage: Option<&SaveStorage>, config: Option<&SaveConfig>) -> Option<std::path::PathBuf> {
    mw_saves_bevy::newest_save(storage?, config?, None)
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("main menu screen");
    assert!(outcome.errors.is_empty(), "main menu screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// The menu's model: the entries, labels localized (English fallbacks); Continue and Load
/// enabled when `saves` exist.
fn model(localizer: Option<&UiLocalizer>, saves: bool) -> MainMenuModel {
    let label = |id: &str, fallback: &str| {
        let key = format!("main-menu-{id}");
        match localizer {
            Some(l) if l.0.has(&key) => l.0.format(&key, None).into_owned(),
            _ => fallback.to_owned(),
        }
    };
    MainMenuModel::new([
        MainMenuEntry::new(CONTINUE, label(CONTINUE, "Continue")).enabled(saves),
        MainMenuEntry::new(NEW_GAME, label(NEW_GAME, "New Game")),
        MainMenuEntry::new(LOAD, label(LOAD, "Load Game")).enabled(saves),
        MainMenuEntry::new(OPTIONS, label(OPTIONS, "Options")),
        MainMenuEntry::new(QUIT, label(QUIT, "Quit")),
    ])
    .with_quit(QUIT)
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_main_menu(
    mut commands: Commands,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    localizer: Option<Res<UiLocalizer>>,
    storage: Option<Res<SaveStorage>>,
    config: Option<Res<SaveConfig>>,
) {
    let saves = newest(storage.as_deref(), config.as_deref()).is_some();
    commands.spawn((UiScreenRoot(MAIN_MENU.into()), MainMenuScreen::new(model(localizer.as_deref(), saves))));
    if let Err(e) = stack.0.push(&registry.0, MAIN_MENU) {
        warn!("main menu: {e:?}");
    }
}

/// The menu's choices (the screen never closes itself). Quit has asked already.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn host_main_menu(
    mut commands: Commands,
    mut choices: MessageReader<MainMenuEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    store: Option<Res<UiSettingsStore>>,
    localizer: Option<Res<UiLocalizer>>,
    saves: (Option<Res<SaveStorage>>, Option<Res<SaveConfig>>),
    mut loads: MessageWriter<LoadRequest>,
    mut open_saves: MessageWriter<OpenSaveLoad>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in choices.read() {
        match event.id.as_str() {
            CONTINUE => {
                if let Some(path) = newest(saves.0.as_deref(), saves.1.as_deref()) {
                    info!("main menu: continue from {}", path.display());
                    loads.write(LoadRequest::path(path));
                } else {
                    warn!("main menu: no save to continue");
                }
            }
            NEW_GAME => {
                info!("main menu: new game");
                stack.0.close(MAIN_MENU);
            }
            LOAD => {
                open_saves.write(OpenSaveLoad(SaveLoadMode::Load));
            }
            OPTIONS => crate::pause_menu::open_options(&mut commands, &mut stack, &registry, store.as_deref(), localizer.as_deref()),
            QUIT => {
                info!("main menu: quit");
                exit.write(AppExit::Success);
            }
            other => warn!("main menu: unknown entry {other}"),
        }
    }
}

/// Continue and Load follow the save folder when the save/load screen closes (a delete may
/// have emptied it).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn refresh_saves(
    stack: Res<ScreenStackRes>,
    storage: Option<Res<SaveStorage>>,
    config: Option<Res<SaveConfig>>,
    mut menus: Query<&mut MainMenuScreen>,
    mut was_open: Local<bool>,
) {
    let open = stack.0.is_open(SAVELOAD);
    let closed = *was_open && !open;
    *was_open = open;
    if !closed || menus.is_empty() {
        return;
    }
    let saves = newest(storage.as_deref(), config.as_deref()).is_some();
    for mut menu in &mut menus {
        if menu.model.is_enabled(CONTINUE) != saves || menu.model.is_enabled(LOAD) != saves {
            menu.model.set_enabled(CONTINUE, saves).set_enabled(LOAD, saves);
        }
    }
}

/// The menu's root despawns once off the stack.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_main_menu(mut commands: Commands, stack: Res<ScreenStackRes>, roots: Query<(Entity, &UiScreenRoot), With<MainMenuScreen>>) {
    if stack.0.is_open(MAIN_MENU) {
        return;
    }
    for (e, root) in &roots {
        if root.0.as_str() == MAIN_MENU {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    //! Headless, on the pause menu's test app (`pause_menu.rs` tests).
    use std::path::PathBuf;

    use bevy::ecs::message::Messages;
    use sf_save_bevy::sf_save::SaveReason;
    use sf_save_bevy::{LoadDone, SaveDone, SaveRequest, SfSavePlugin};
    use sf_ui_screens::SaveLoadScreen;

    use super::*;
    use crate::pause_menu::tests::{app_with, frames, open, paused, roots, tap};
    use crate::pause_menu::{HUD, OPTIONS as OPTIONS_SCREEN, PAUSE};

    /// The files of the finished loads.
    #[derive(Resource, Default)]
    struct Loads(Vec<PathBuf>);

    fn record_loads(mut done: MessageReader<LoadDone>, mut loads: ResMut<Loads>) {
        loads.0.extend(done.read().filter_map(|d| d.0.as_ref().ok().map(|r| r.path.clone())));
    }

    fn release() -> App {
        app_with(|app| add(app, true))
    }

    fn menu(app: &mut App) -> MainMenuScreen {
        let mut q = app.world_mut().query::<&MainMenuScreen>();
        q.single(app.world()).expect("one main menu").clone()
    }

    #[test]
    fn release_start_opens_the_menu_paused_and_new_game_plays() {
        let mut app = release();
        assert!(open(&app, MAIN_MENU) && open(&app, HUD), "the menu over the HUD screen");
        assert_eq!(roots(&mut app, MAIN_MENU), 1);
        assert!(paused(&app), "the game is paused under the menu");
        let screen = menu(&mut app);
        assert_eq!(screen.model.ids(), [CONTINUE, NEW_GAME, LOAD, OPTIONS, QUIT]);
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD), "no save plugin, no saves");
        assert_eq!(screen.model.entry(NEW_GAME).map(|e| e.label.as_str()), Some("New Game"), "the Fluent string");

        // Esc is the menu's Back: nothing at the top level, and no pause menu over it.
        tap(&mut app, KeyCode::Escape);
        assert!(open(&app, MAIN_MENU) && !open(&app, PAUSE) && paused(&app));

        // Focus starts on New game (Continue is disabled); confirm.
        tap(&mut app, KeyCode::Enter);
        assert!(!open(&app, MAIN_MENU), "New game closes the menu");
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
        assert!(!paused(&app), "play begins");
        assert!(app.should_exit().is_none());

        // Play: Esc is the pause menu now.
        tap(&mut app, KeyCode::Escape);
        assert!(open(&app, PAUSE) && paused(&app));
    }

    #[test]
    fn options_open_on_top_and_quit_exits_after_yes() {
        let mut app = release();
        // New game -> (Load, disabled, skipped) -> Options.
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(open(&app, OPTIONS_SCREEN) && open(&app, MAIN_MENU), "options over the menu");
        assert_eq!(roots(&mut app, OPTIONS_SCREEN), 1);
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, OPTIONS_SCREEN) && open(&app, MAIN_MENU) && paused(&app));
        assert_eq!(roots(&mut app, OPTIONS_SCREEN), 0);

        // Options -> Quit; it asks (No focused); left to Yes; confirm.
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(app.should_exit().is_none(), "Quit asks first");
        tap(&mut app, KeyCode::ArrowLeft);
        tap(&mut app, KeyCode::Enter);
        assert_eq!(app.should_exit(), Some(AppExit::Success));
    }

    /// A release start with the save plugin on `root` (the save/load screen hosted).
    fn release_with_saves(root: &std::path::Path) -> App {
        let config = SaveConfig { root: root.to_path_buf(), background: false, ..default() };
        app_with(|app| {
            app.add_plugins(SfSavePlugin { config })
                .insert_resource(sf_world_bevy::WorldOverlay::default())
                .insert_resource(sf_sky_bevy::SkyTime::default());
            mw_saves_bevy::register(app);
            mw_saves_bevy::menu::add(app);
            add(app, true);
            app.init_resource::<Loads>().add_systems(Update, record_loads);
        })
    }

    /// A quicksave, then a manual save (the newest: Continue takes any reason, quickload only
    /// quicksaves), written by an earlier run into `root`. Returns (quick, manual) files.
    fn earlier_run(root: &std::path::Path) -> (PathBuf, PathBuf) {
        let mut app = App::new();
        app.add_plugins(SfSavePlugin { config: SaveConfig { root: root.to_path_buf(), background: false, ..default() } })
            .insert_resource(sf_world_bevy::WorldOverlay::default())
            .insert_resource(sf_sky_bevy::SkyTime::default());
        mw_saves_bevy::register(&mut app);
        let mut save = |request: SaveRequest| {
            app.world_mut().write_message(request);
            app.update();
            let done: Vec<_> = app.world_mut().resource_mut::<Messages<SaveDone>>().drain().collect();
            done[0].0.as_ref().expect("saved").path.clone()
        };
        let quick = save(SaveRequest::quick());
        let manual = save(SaveRequest::slot(SaveReason::Manual));
        (quick, manual)
    }

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bw-main-menu-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    fn choose(app: &mut App, id: &str) {
        let mut q = app.world_mut().query_filtered::<Entity, With<MainMenuScreen>>();
        let screen = q.single(app.world()).expect("one main menu");
        app.world_mut().write_message(MainMenuEvent { screen, id: id.into() });
    }

    #[test]
    fn no_saves_leave_continue_and_load_disabled() {
        let root = temp_root("empty");
        let mut app = release_with_saves(&root);
        let screen = menu(&mut app);
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn continue_loads_the_newest_save_and_plays() {
        let root = temp_root("continue");
        let (quick, manual) = earlier_run(&root);
        assert!(quick.is_file() && manual.is_file());
        let mut app = release_with_saves(&root);
        let (storage, config) = (app.world().resource::<SaveStorage>(), app.world().resource::<SaveConfig>());
        assert_eq!(mw_saves_bevy::newest_save(storage, config, Some(SaveReason::Quick)), Some(quick), "quickload's target");
        let screen = menu(&mut app);
        assert!(screen.model.is_enabled(CONTINUE) && screen.model.is_enabled(LOAD));
        assert!(open(&app, MAIN_MENU) && paused(&app));

        // Focus starts on Continue now; confirm: the newest save of any reason loads.
        tap(&mut app, KeyCode::Enter);
        let loads = app.world().resource::<Loads>().0.clone();
        assert_eq!(loads, [manual], "the manual save, newer than the quicksave");
        assert!(!open(&app, MAIN_MENU) && !paused(&app), "the load closes the menu and play runs");
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn load_opens_the_save_screen_and_back_returns_to_the_main_menu() {
        let root = temp_root("load");
        let (quick, manual) = earlier_run(&root);
        let mut app = release_with_saves(&root);

        choose(&mut app, LOAD);
        frames(&mut app, 3);
        assert!(open(&app, SAVELOAD) && open(&app, MAIN_MENU) && paused(&app), "the save screen over the main menu");
        let mut q = app.world_mut().query::<&SaveLoadScreen>();
        let model = q.single(app.world()).expect("save/load screen").model.clone();
        assert_eq!(model.mode, SaveLoadMode::Load);
        assert_eq!(model.entries.len(), 2, "a fresh run lists every world's saves");

        // Back: the main menu again, still paused, no pause menu.
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, SAVELOAD) && open(&app, MAIN_MENU) && !open(&app, PAUSE) && paused(&app));

        // Both saves deleted elsewhere: Continue and Load turn off when the screen closes next.
        choose(&mut app, LOAD);
        frames(&mut app, 3);
        std::fs::remove_file(&quick).unwrap();
        std::fs::remove_file(&manual).unwrap();
        tap(&mut app, KeyCode::Escape);
        let screen = menu(&mut app);
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD));
        assert!(app.world().resource::<Loads>().0.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn dev_runs_open_no_main_menu() {
        let mut app = app_with(|app| add(app, false));
        assert!(!open(&app, MAIN_MENU) && !paused(&app));
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
    }
}
