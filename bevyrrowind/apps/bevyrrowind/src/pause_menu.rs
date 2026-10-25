//! The pause (escape) menu (superfargoth `ui.md` "Stock screens"): the stock `sf_ui_screens`
//! pause menu and, on top of it, the stock options screen.
//!
//! - **Open**: the rebindable UI "open menu" action (`sf_ui_bevy::UiAction::OpenMenu`: Esc,
//!   gamepad Start) in player mode. UI actions reach only the stack's input target, so the
//!   game keeps a HUD-layer screen ("hud") open as the target of play: with no menu open, Esc
//!   is that screen's open-menu action; with a menu (dialogue, journal, inventory, keybinds)
//!   open, Esc is the menu's Back and closes it first. The console blocks the keys while open.
//!   Scaffold: the "hud" screen has no root yet (no HUD drawn); the HUD slice spawns its
//!   `HudScreen` on the same record.
//! - **Pause**: the "pause" and "options" records declare `pause = "WhileOpen"`; while
//!   `ScreenStack::pauses_game` holds, Bevy's virtual clock is paused (`Time<Virtual>`, the
//!   engine's pause), so the fixed-step physics, animation, the sky clock and NPCs stop. The
//!   player's controls pause through `dialogue.rs`'s menu seam (`PlayerMode::paused`, any
//!   Menu-layer screen).
//! - **Choices**: Resume (or Back) closes the menu (the screen does); Quit, after its yes/no,
//!   exits (`AppExit::Success`); Options opens the options screen on top (its Back returns
//!   here). Save and Load are disabled until the save/load screen is wired. Option changes
//!   are not applied yet (logged): the options-apply slice.

use bevy::prelude::*;
use sf_console_bevy::ConsoleOpen;
use sf_records::RecordDb;
use sf_ui::{Layer, UiRegistry};
use sf_ui_bevy::{ScreenStackRes, UiAction, UiActionEvent, UiLocalizer, UiRegistryRes, UiScreenRoot, UiSettingsStore, UiSystems};
use sf_ui_screens::{
    OptionsChange, OptionsEvent, OptionsModel, OptionsScreen, PauseChoice, PauseMenuEvent, PauseMenuModel, PauseMenuScreen,
    ScreenSystems,
};

use crate::player::PlayerMode;

/// The HUD screen: the input target of play (carries the open-menu action).
pub const HUD: &str = "hud";
/// The pause menu's screen id.
pub const PAUSE: &str = "pause";
/// The options screen's id.
pub const OPTIONS: &str = "options";

/// The game's records of these screens. TODO(HUD slice): with the other screens, in the
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
    let Some(mut registry) = app.world_mut().get_resource_mut::<UiRegistryRes>() else { return };
    registry.0.extend_screens(&screens());
    app.add_systems(
        Update,
        (
            (keep_hud, open_pause_menu).chain().after(UiSystems::Dispatch).before(ScreenSystems::Spawn),
            (host_pause_menu, host_options, close_screens, pause_time).chain().after(ScreenSystems::Input),
        ),
    );
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("pause screens");
    assert!(outcome.errors.is_empty(), "pause screens: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// The pause menu's model: Save and Load disabled until the save/load screen is wired.
fn model() -> PauseMenuModel {
    let mut model = PauseMenuModel::default();
    model.set_enabled(PauseChoice::Save, false).set_enabled(PauseChoice::Load, false);
    model
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
#[allow(clippy::needless_pass_by_value)] // system parameters
fn host_pause_menu(
    mut commands: Commands,
    mut choices: MessageReader<PauseMenuEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    store: Option<Res<UiSettingsStore>>,
    localizer: Option<Res<UiLocalizer>>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in choices.read() {
        match event.choice {
            PauseChoice::Quit => {
                info!("pause menu: quit");
                exit.write(AppExit::Success);
            }
            PauseChoice::Options if !stack.0.is_open(OPTIONS) => {
                let model = OptionsModel::from_settings(&registry.0, store.as_deref().map(|s| &s.0), localizer.as_deref().map(|l| &l.0));
                commands.spawn((UiScreenRoot(OPTIONS.into()), OptionsScreen::new(model)));
                if let Err(e) = stack.0.push(&registry.0, OPTIONS) {
                    warn!("options: {e:?}");
                }
            }
            // Resume: the screen has closed itself. Options: already open.
            PauseChoice::Resume | PauseChoice::Options => {}
            PauseChoice::Save | PauseChoice::Load => warn!("pause menu: {:?} is not wired yet", event.choice),
        }
    }
}

/// Option changes. TODO(options-apply slice): store, save the profile, apply.
fn host_options(mut events: MessageReader<OptionsEvent>) {
    for event in events.read() {
        match &event.change {
            OptionsChange::Back => {}
            OptionsChange::Set(key, value) => info!("options: {key} = {value:?} (not applied yet)"),
            OptionsChange::OpenKeybinds => info!("options: key bindings (not wired yet)"),
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
fn pause_time(stack: Res<ScreenStackRes>, mut time: ResMut<Time<Virtual>>, mut was: Local<bool>) {
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
mod tests {
    //! Headless: UI layout and text without a window (a camera with a fixed target), Esc and
    //! the arrows through `ButtonInput`, as `sf_ui_screens`'s screen tests drive them.
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::input::InputPlugin;
    use bevy::picking::{InteractionPlugin, PickingPlugin};
    use bevy::text::TextPlugin;
    use bevy::ui::UiPlugin;
    use bevy_enhanced_input::prelude::EnhancedInputPlugin;
    use sf_ui_bevy::{SfUiPlugin, UiTextPlugin};
    use sf_ui_screens::SfScreensPlugin;

    use super::*;

    fn app() -> App {
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
        .insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None });
        add(&mut app);
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

    fn frames(app: &mut App, n: usize) {
        for _ in 0..n {
            app.update();
        }
    }

    fn tap(app: &mut App, key: KeyCode) {
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(key);
        app.update();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(key);
        frames(app, 3);
    }

    fn open(app: &App, id: &str) -> bool {
        app.world().resource::<ScreenStackRes>().0.is_open(id)
    }

    fn paused(app: &App) -> bool {
        app.world().resource::<Time<Virtual>>().is_paused()
    }

    fn roots(app: &mut App, id: &str) -> usize {
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
        assert!(!screen.model.is_enabled(PauseChoice::Save) && !screen.model.is_enabled(PauseChoice::Load));
        assert!(screen.model.is_enabled(PauseChoice::Resume) && screen.model.is_enabled(PauseChoice::Quit));

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
        // Resume -> Options (Save and Load are skipped).
        tap(&mut app, KeyCode::ArrowDown);
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
}
