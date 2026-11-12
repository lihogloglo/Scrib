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
//! Who leaves which phase: `Boot` ends after `PostStartup` (this plugin), unless something holds it
//! ([`BootHolds`]: the game's intro videos, menus.md §3); then it ends in the update the last hold
//! is released (asked in `PostUpdate`, applied at the next update's `StateTransition`). `Frontend`
//! → `Loading` is the menu's New game / Continue / Load (the game); `Loading` → `InGame` is the
//! game's streaming signal (start area resident).

use std::collections::BTreeSet;

use bevy_app::{App, Plugin, PostStartup, PostUpdate};
use bevy_ecs::prelude::*;
pub use bevy_state;
use bevy_state::app::{AppExtStates, StatesPlugin};
use bevy_state::prelude::{ComputedStates, NextState, State, States, in_state};

/// The app's phase (menus.md §1 table).
#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppPhase {
    /// Profile, mods and records load; settings apply; intro videos ([`BootHolds`]).
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

/// Named holds that keep the app in [`AppPhase::Boot`] (intro videos, a splash, a first-run
/// prompt). Place one at build or in `Startup` (before `PostStartup` looks) and release it when
/// done; with none, `Boot` ends after `PostStartup` exactly as without this resource.
#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
pub struct BootHolds(BTreeSet<String>);

impl BootHolds {
    /// Holds `Boot` under `name` (again: no change).
    pub fn hold(&mut self, name: impl Into<String>) {
        self.0.insert(name.into());
    }

    /// Releases `name`; whether it was held.
    pub fn release(&mut self, name: &str) -> bool {
        self.0.remove(name)
    }

    /// Whether `name` holds `Boot`.
    pub fn is_held(&self, name: &str) -> bool {
        self.0.contains(name)
    }

    /// No hold: `Boot` may end.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Adds the states (and Bevy's `StatesPlugin` if `DefaultPlugins` didn't) and leaves `Boot`
/// for [`Self::after_boot`] once startup has run and no [`BootHolds`] remain.
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
        let leave_boot = move |phase: Res<State<AppPhase>>, holds: Res<BootHolds>, mut next: ResMut<NextState<AppPhase>>| {
            if *phase.get() == AppPhase::Boot && holds.is_empty() {
                next.set(after_boot);
            }
        };
        // `PostUpdate`: after every `Update` system that may release the last hold.
        app.init_state::<AppPhase>()
            .add_computed_state::<WorldAlive>()
            .init_resource::<BootHolds>()
            .add_systems(PostStartup, leave_boot)
            .add_systems(PostUpdate, leave_boot.run_if(in_state(AppPhase::Boot)));
    }
}

#[cfg(test)]
mod tests {
    use bevy_state::prelude::{OnEnter, OnExit};

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
    fn a_boot_hold_keeps_boot_until_released() {
        let mut app = app(AppPhase::Frontend);
        app.add_systems(bevy_app::Startup, |mut holds: ResMut<BootHolds>| {
            holds.hold("intro");
            holds.hold("splash");
        });
        frames_in_boot(&mut app, 3);
        assert!(app.world().resource::<BootHolds>().is_held("intro"));
        app.world_mut().resource_mut::<BootHolds>().release("splash");
        frames_in_boot(&mut app, 2);
        assert!(app.world_mut().resource_mut::<BootHolds>().release("intro"));
        // Released between updates: this update's PostUpdate asks, the next one's transition applies.
        app.update();
        assert_eq!(phase(&app), AppPhase::Boot);
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert!(!alive(&app));
        assert_eq!(counts(&app), (0, 0, 0));
        assert!(!app.world_mut().resource_mut::<BootHolds>().release("intro"), "released once");
    }

    fn frames_in_boot(app: &mut App, n: usize) {
        for _ in 0..n {
            app.update();
            assert_eq!(phase(app), AppPhase::Boot, "held");
        }
    }

    #[test]
    fn states_plugin_is_added_once() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin).add_plugins(AppPhasePlugin::default());
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
    }
}
