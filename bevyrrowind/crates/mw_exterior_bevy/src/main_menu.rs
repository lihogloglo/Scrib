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
//!   Options, Quit. Continue and Load stay disabled until saves reach the menu (M3). New game
//!   closes the menu and play begins; Options opens the options screen on top as the pause
//!   menu does ([`crate::pause_menu::open_options`]); Quit asks (the screen), then exits.
//!   Labels come from the builtin mod's Fluent file (`main-menu-<id>`).

use bevy::prelude::*;
use sf_records::RecordDb;
use sf_ui::UiRegistry;
use sf_ui_bevy::{ScreenStackRes, UiLocalizer, UiRegistryRes, UiScreenRoot, UiSettingsStore};
use sf_ui_screens::{MainMenuEntry, MainMenuEvent, MainMenuModel, MainMenuScreen, ScreenSystems};

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
    app.add_systems(Startup, open_main_menu).add_systems(
        Update,
        (host_main_menu, close_main_menu).chain().after(ScreenSystems::Input).before(crate::pause_menu::pause_time),
    );
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("main menu screen");
    assert!(outcome.errors.is_empty(), "main menu screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// The menu's model: the entries, labels localized (English fallbacks).
fn model(localizer: Option<&UiLocalizer>) -> MainMenuModel {
    let label = |id: &str, fallback: &str| {
        let key = format!("main-menu-{id}");
        match localizer {
            Some(l) if l.0.has(&key) => l.0.format(&key, None).into_owned(),
            _ => fallback.to_owned(),
        }
    };
    MainMenuModel::new([
        MainMenuEntry::new(CONTINUE, label(CONTINUE, "Continue")).enabled(false),
        MainMenuEntry::new(NEW_GAME, label(NEW_GAME, "New Game")),
        MainMenuEntry::new(LOAD, label(LOAD, "Load Game")).enabled(false),
        MainMenuEntry::new(OPTIONS, label(OPTIONS, "Options")),
        MainMenuEntry::new(QUIT, label(QUIT, "Quit")),
    ])
    .with_quit(QUIT)
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_main_menu(mut commands: Commands, mut stack: ResMut<ScreenStackRes>, registry: Res<UiRegistryRes>, localizer: Option<Res<UiLocalizer>>) {
    commands.spawn((UiScreenRoot(MAIN_MENU.into()), MainMenuScreen::new(model(localizer.as_deref()))));
    if let Err(e) = stack.0.push(&registry.0, MAIN_MENU) {
        warn!("main menu: {e:?}");
    }
}

/// The menu's choices (the screen never closes itself). Quit has asked already.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn host_main_menu(
    mut commands: Commands,
    mut choices: MessageReader<MainMenuEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    store: Option<Res<UiSettingsStore>>,
    localizer: Option<Res<UiLocalizer>>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in choices.read() {
        match event.id.as_str() {
            NEW_GAME => {
                info!("main menu: new game");
                stack.0.close(MAIN_MENU);
            }
            OPTIONS => crate::pause_menu::open_options(&mut commands, &mut stack, &registry, store.as_deref(), localizer.as_deref()),
            QUIT => {
                info!("main menu: quit");
                exit.write(AppExit::Success);
            }
            // Continue and Load: disabled until saves reach the menu (M3).
            other => warn!("main menu: {other} not available yet"),
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
    use super::*;
    use crate::pause_menu::tests::{app_with, open, paused, roots, tap};
    use crate::pause_menu::{HUD, OPTIONS as OPTIONS_SCREEN, PAUSE};

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
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD), "saves come in M3");
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

    #[test]
    fn dev_runs_open_no_main_menu() {
        let mut app = app_with(|app| add(app, false));
        assert!(!open(&app, MAIN_MENU) && !paused(&app));
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
    }
}
