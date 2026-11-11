//! The app's phases as Bevy states (menus.md "### 1. App phases", slice F1).
//!
//! [`AppPhase`] walks `Boot` → `Frontend` (main menu, **no world**) → `Loading` (the world builds
//! at the start or save location) → `InGame`, and back to `Frontend` from the pause menu (F2).
//! Dev runs go `Boot` → `Loading` → `InGame` directly ([`AppPhasePlugin::after_boot`]).
//!
//! [`WorldAlive`] is the computed state "a world exists" (`Loading` or `InGame`). The app gates
//! world plugins' public system sets with `configure_sets(.., run_if(in_state(WorldAlive)))` and
//! moves world setup from `Startup` to `OnEnter(WorldAlive)`, so no framework crate needs to know
//! the phase. Its `OnEnter`/`OnExit` run once per world: `Loading` → `InGame` keeps the same value
//! and runs neither ([`WorldAlive::ALLOW_SAME_STATE_TRANSITIONS`] is off).
//!
//! Who leaves which phase: `Boot` ends after `PostStartup` (this plugin; the intro videos of I1
//! will hold it later); `Frontend` → `Loading` is the menu's New game / Continue / Load (the game);
//! `Loading` → `InGame` is the game's streaming signal (start area resident).

use bevy_app::{App, Plugin, PostStartup};
use bevy_ecs::prelude::*;
pub use bevy_state;
use bevy_state::app::{AppExtStates, StatesPlugin};
use bevy_state::prelude::{ComputedStates, NextState, State, States};

/// The app's phase (menus.md §1 table).
#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppPhase {
    /// Profile, mods and records load; settings apply; intro videos (later).
    #[default]
    Boot,
    /// Main menu, options, save list. No world: no streaming, player or world camera.
    Frontend,
    /// The world builds (loading screen, F4).
    Loading,
    /// The game; pause and other screens over it.
    InGame,
}

/// A world exists: [`AppPhase::Loading`] or [`AppPhase::InGame`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldAlive;

impl ComputedStates for WorldAlive {
    type SourceStates = AppPhase;

    /// `Loading` → `InGame` must not re-run the world's `OnEnter`/`OnExit`.
    const ALLOW_SAME_STATE_TRANSITIONS: bool = false;

    fn compute(phase: AppPhase) -> Option<Self> {
        matches!(phase, AppPhase::Loading | AppPhase::InGame).then_some(Self)
    }
}

/// Adds the states (and Bevy's `StatesPlugin` if `DefaultPlugins` didn't) and leaves `Boot`
/// for [`Self::after_boot`] once startup has run.
#[derive(Clone, Copy, Debug)]
pub struct AppPhasePlugin {
    /// `Frontend` for a release start, `Loading` for dev runs.
    pub after_boot: AppPhase,
}

impl Default for AppPhasePlugin {
    fn default() -> Self {
        Self { after_boot: AppPhase::Frontend }
    }
}

impl Plugin for AppPhasePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<StatesPlugin>() {
            app.add_plugins(StatesPlugin);
        }
        let after_boot = self.after_boot;
        app.init_state::<AppPhase>().add_computed_state::<WorldAlive>().add_systems(
            PostStartup,
            move |phase: Res<State<AppPhase>>, mut next: ResMut<NextState<AppPhase>>| {
                if *phase.get() == AppPhase::Boot {
                    next.set(after_boot);
                }
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy_state::prelude::{OnEnter, OnExit, in_state};

    use super::*;

    #[derive(Resource, Default)]
    struct Counts {
        entered: u32,
        exited: u32,
        gated_runs: u32,
    }

    fn app(after_boot: AppPhase) -> App {
        let mut app = App::new();
        app.add_plugins(AppPhasePlugin { after_boot })
            .init_resource::<Counts>()
            .add_systems(OnEnter(WorldAlive), |mut c: ResMut<Counts>| c.entered += 1)
            .add_systems(OnExit(WorldAlive), |mut c: ResMut<Counts>| c.exited += 1)
            .add_systems(bevy_app::Update, (|mut c: ResMut<Counts>| c.gated_runs += 1).run_if(in_state(WorldAlive)));
        app
    }

    fn phase(app: &App) -> AppPhase {
        *app.world().resource::<State<AppPhase>>().get()
    }

    fn alive(app: &App) -> bool {
        app.world().get_resource::<State<WorldAlive>>().is_some()
    }

    fn go(app: &mut App, to: AppPhase) {
        app.world_mut().resource_mut::<NextState<AppPhase>>().set(to);
        app.update();
    }

    fn counts(app: &App) -> (u32, u32, u32) {
        let c = app.world().resource::<Counts>();
        (c.entered, c.exited, c.gated_runs)
    }

    #[test]
    fn world_alive_is_loading_or_in_game() {
        assert_eq!(WorldAlive::compute(AppPhase::Boot), None);
        assert_eq!(WorldAlive::compute(AppPhase::Frontend), None);
        assert_eq!(WorldAlive::compute(AppPhase::Loading), Some(WorldAlive));
        assert_eq!(WorldAlive::compute(AppPhase::InGame), Some(WorldAlive));
    }

    #[test]
    fn release_boot_enters_the_frontend_with_no_world() {
        let mut app = app(AppPhase::Frontend);
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert!(!alive(&app));
        assert_eq!(counts(&app), (0, 0, 0), "no world setup, no world system ran");
    }

    #[test]
    fn new_game_walks_loading_in_game_and_back_once_per_world() {
        let mut app = app(AppPhase::Frontend);
        app.update();
        go(&mut app, AppPhase::Loading);
        assert_eq!(phase(&app), AppPhase::Loading);
        assert!(alive(&app));
        assert_eq!(counts(&app), (1, 0, 1), "the world's setup ran once, its systems run");
        go(&mut app, AppPhase::InGame);
        assert_eq!(phase(&app), AppPhase::InGame);
        assert_eq!(counts(&app), (1, 0, 2), "Loading -> InGame keeps the world: no re-enter");
        go(&mut app, AppPhase::Frontend);
        assert!(!alive(&app));
        assert_eq!(counts(&app), (1, 1, 2), "back to the menu: the world exits, its systems stop");
        go(&mut app, AppPhase::Loading);
        assert_eq!(counts(&app), (2, 1, 3), "a second world enters again");
    }

    #[test]
    fn dev_boot_goes_straight_to_loading() {
        let mut app = app(AppPhase::Loading);
        app.update();
        assert_eq!(phase(&app), AppPhase::Loading);
        assert!(alive(&app));
        assert_eq!(counts(&app).0, 1);
    }

    #[test]
    fn states_plugin_is_added_once() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin).add_plugins(AppPhasePlugin::default());
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
    }
}
