//! Thin Bevy plugin over the Bevy-free `sf_ui` core (ui.md "Interfaces", "Input").
//!
//! - **Resources** wrap the core: [`UiRegistryRes`], [`ScreenStackRes`], [`UiSettingsStore`],
//!   [`UiLocalizer`], plus the live [`UiSettings`] and the last-used device [`InputMode`].
//! - **Input router** ([`router`]): one `bevy_enhanced_input` context entity per screen input
//!   context id (`UiScreenInput`), created on first use. When the stack changes, only the input
//!   target's context is active; it runs at [`UI_CONTEXT_PRIORITY`], so its bound inputs are
//!   consumed before gameplay contexts see them (Common UI's activatable-widget rule).
//! - **Actions** ([`actions`]): named UI actions with keyboard/mouse and gamepad defaults in
//!   [`UiBindings`]; changing the resource rebinds every UI context (menu open included).
//! - **Navigation** ([`nav`]): Bevy's `AutoDirectionalNavigator`, kept to the input target's
//!   screen by moving the `AutoDirectionalNavigation` marker onto its [`UiFocusable`]s only
//!   (Bevy's navigation is layer-agnostic), held directions repeat; confirm triggers
//!   `bevy_ui_widgets::Activate` on the focused entity.
//! - **Device** ([`device`]): last-used device switches [`InputMode`] the same frame; a small
//!   glyph table of our own labels the bound input per device.

pub mod actions;
pub mod device;
pub mod nav;
pub mod router;

#[cfg(test)]
mod tests;

use bevy_app::{App, Last, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::{EnhancedInputSystems, InputContextAppExt};
use bevy_input_focus::InputFocus;
use bevy_input_focus::directional_navigation::{
    AutoNavigationConfig, DirectionalNavigationMap, DirectionalNavigationPlugin,
};
use sf_ui::{Localizer, PresetId, ScreenStack, SettingsStore, ThemeId, UiRegistry};

pub use actions::{UiAction, UiBindings};
pub use device::{Device, InputMode, glyph};
pub use nav::{NavRepeat, UiDefaultFocus, UiFocusable, held_direction};
pub use router::{
    UI_CONTEXT_PRIORITY, UiActionEvent, UiHandlesBack, UiInputContexts, UiScreenInput, UiScreenRoot,
};

/// The resolved UI registry of the current mod set.
#[derive(Resource, Debug, Default)]
pub struct UiRegistryRes(pub UiRegistry);

/// Open screens per layer. Push and pop through it; the router reacts to
/// [`ScreenStack::version`] changes.
#[derive(Resource, Debug, Default)]
pub struct ScreenStackRes(pub ScreenStack);

/// Values of the `ui.setting` registry (saved per profile by the app).
#[derive(Resource)]
pub struct UiSettingsStore(pub SettingsStore);

/// Fluent strings for the negotiated locale chain.
#[derive(Resource)]
pub struct UiLocalizer(pub Localizer);

/// Live presentation settings: a change restyles, it never respawns screens
/// (ui.md "Interfaces"). Consumed by the stylesheet composer (U2) and widgets (U3).
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct UiSettings {
    pub theme: ThemeId,
    pub preset: PresetId,
    pub text_scale: f32,
    pub hud_scale: f32,
    pub menu_scale: f32,
    pub high_contrast: bool,
    pub reduce_motion: bool,
    pub classic_windows: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: ThemeId::from("modern"),
            preset: PresetId::from("pc_dense"),
            text_scale: 1.0,
            hud_scale: 1.0,
            menu_scale: 1.0,
            high_contrast: false,
            reduce_motion: false,
            classic_windows: false,
        }
    }
}

/// Ordering of the UI systems in `PreUpdate` (after input contexts are
/// evaluated) and `Update`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiSystems {
    /// Last-used device (`PreUpdate`, before action evaluation).
    Device,
    /// Stack change → active context; rebinding (`PreUpdate`, before action evaluation).
    Route,
    /// Forward fired actions, move focus (`Update`).
    Dispatch,
}

/// Router, actions, navigation and device tracking. Needs `EnhancedInputPlugin`
/// and an input plugin (Bevy's `InputPlugin`); `bevy_ui` layout and
/// `bevy_input_focus`'s `InputFocusPlugin` for navigation (all in `DefaultPlugins`).
pub struct SfUiPlugin;

impl Plugin for SfUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<DirectionalNavigationPlugin>()
            && !app.world().contains_resource::<DirectionalNavigationMap>()
        {
            app.add_plugins(DirectionalNavigationPlugin);
        }
        app.init_resource::<UiRegistryRes>()
            .init_resource::<ScreenStackRes>()
            .init_resource::<UiSettings>()
            .init_resource::<InputMode>()
            .init_resource::<UiBindings>()
            .init_resource::<UiInputContexts>()
            .init_resource::<nav::NavState>()
            .init_resource::<router::FocusMemory>()
            .init_resource::<router::ActivatedThisFrame>()
            .init_resource::<InputFocus>()
            .init_resource::<AutoNavigationConfig>()
            .add_message::<UiActionEvent>()
            .add_observer(router::record_activation)
            .add_systems(Last, router::clear_activations)
            .add_input_context::<UiScreenInput>()
            .configure_sets(
                PreUpdate,
                (UiSystems::Device, UiSystems::Route)
                    .chain()
                    .after(bevy_input::InputSystems)
                    // Context (de)activation and rebinding must land before BEI's `Prepare`, which
                    // collects held inputs that `require_reset` actions keep consuming.
                    .before(EnhancedInputSystems::Prepare),
            )
            .add_systems(
                PreUpdate,
                device::track_input_mode.in_set(UiSystems::Device),
            )
            .add_systems(
                PreUpdate,
                (router::route_stack, actions::apply_rebinding)
                    .chain()
                    .in_set(UiSystems::Route),
            )
            .add_systems(
                Update,
                (
                    router::sync_navigable,
                    router::dispatch_actions,
                    nav::navigate,
                )
                    .chain()
                    .in_set(UiSystems::Dispatch),
            );
    }
}
