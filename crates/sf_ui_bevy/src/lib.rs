//! Thin Bevy plugin over the Bevy-free `sf_ui` core (ui.md "Interfaces", "Input").
//!
//! - **Resources** wrap the core: [`UiRegistryRes`], [`ScreenStackRes`], [`UiSettingsStore`],
//!   [`UiLocalizer`], plus the live [`UiSettings`] and the last-used device [`InputMode`].
//! - **Input router** ([`router`]): one `bevy_enhanced_input` context entity per screen input
//!   context id (`UiScreenInput`), created on first use. When the stack changes, only the input
//!   target's context is active; it runs at [`UI_CONTEXT_PRIORITY`], so its bound inputs are
//!   consumed before gameplay contexts see them (Common UI's activatable-widget rule).
//! - **Actions** ([`actions`]): named UI actions with keyboard/mouse and gamepad defaults in the
//!   player's [`UiKeymap`] (`sf_ui::Keymap`); a keymap change rebuilds [`UiBindings`], which
//!   rebinds every UI context (menu open included).
//! - **Keybinds** ([`keybinds`]): [`UiRebinder`] captures an input for a keymap slot from this
//!   frame's keys, buttons and sticks; UI actions stay quiet while it runs.
//! - **Navigation** ([`nav`]): Bevy's `AutoDirectionalNavigator`, kept to the input target's
//!   screen by moving the `AutoDirectionalNavigation` marker onto its [`UiFocusable`]s only
//!   (Bevy's navigation is layer-agnostic), held directions repeat; confirm triggers
//!   `bevy_ui_widgets::Activate` on the focused entity.
//! - **Device** ([`device`]): last-used device switches [`InputMode`] the same frame, and the
//!   last pad's vendor sets [`GamepadFamily`] for glyphs (`sf_ui::glyph`).
//! - **Profile** ([`profile`]): [`insert_from_profile`] builds registry, settings, strings and
//!   bindings from an `sf_mods` profile; changes are saved back when the player leaves the menus.

pub mod actions;
pub mod device;
pub mod keybinds;
pub mod nav;
pub mod profile;
pub mod router;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_profile;

use bevy_app::{App, Last, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::{EnhancedInputSystems, InputContextAppExt};
use bevy_input_focus::InputFocus;
use bevy_input_focus::directional_navigation::{
    AutoNavigationConfig, DirectionalNavigationMap, DirectionalNavigationPlugin,
};
use bevy_ecs::system::SystemParam;
use sf_ui::{
    Layer, Localizer, OpenScreen, PresetId, ScreenStack, SettingsStore, StackError, ThemeId, UiRegistry,
};

pub use actions::{HUD_SCOPE, UiAction, UiBindings, UiKeymap, ui_keymap};
pub use device::{Device, GamepadFamily, Glyph, InputMode, PadFamily, glyph};
pub use keybinds::{UiRebindEvent, UiRebinder, ui_input_open};
pub use nav::{NavRepeat, UiDefaultFocus, UiFocusable, held_direction};
pub use profile::{
    BINDINGS_CONTEXT, UiLoadReport, UiLocales, UiProfile, binding_name, insert_from_profile, parse_binding,
    parse_inputs, validate_input,
};
pub use router::{
    UI_CONTEXT_PRIORITY, UiActionEvent, UiHandlesBack, UiInputContexts, UiScreenInput, UiScreenRoot,
};

/// The resolved UI registry of the current mod set.
#[derive(Resource, Debug, Default)]
pub struct UiRegistryRes(pub UiRegistry);

/// Open screens per layer. Push and pop through [`UiCommands`] (or this); the router reacts to
/// [`ScreenStack::version`] changes.
#[derive(Resource, Debug, Default)]
pub struct ScreenStackRes(pub ScreenStack);

/// Opening and closing screens from systems: the stack with its registry, and
/// the active layout preset enforced on open (a screen whose record lists
/// presets opens only in those).
#[derive(SystemParam)]
pub struct UiCommands<'w> {
    registry: Res<'w, UiRegistryRes>,
    stack: ResMut<'w, ScreenStackRes>,
    settings: Res<'w, UiSettings>,
}

impl UiCommands<'_> {
    /// Opens a screen on top of its layer (moves it there when already open).
    pub fn push(&mut self, id: &str) -> Result<&OpenScreen, StackError> {
        let preset = self.settings.preset.as_str();
        self.stack.0.push_in(&self.registry.0, id, preset)
    }

    /// Replaces the top screen of the screen's layer (tab switches).
    pub fn replace_top(&mut self, id: &str) -> Result<&OpenScreen, StackError> {
        let preset = self.settings.preset.as_str();
        self.stack.0.replace_top_in(&self.registry.0, id, preset)
    }

    /// Closes the top screen of a layer.
    pub fn pop(&mut self, layer: Layer) -> Option<OpenScreen> {
        self.stack.0.pop(layer)
    }

    /// Closes a screen wherever it is.
    pub fn close(&mut self, id: &str) -> bool {
        self.stack.0.close(id)
    }

    pub fn stack(&self) -> &ScreenStack {
        &self.stack.0
    }
}

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
    /// Keybind capture; stack change → active context; keymap → bindings → rebinding
    /// (`PreUpdate`, before action evaluation).
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
            .init_resource::<GamepadFamily>()
            .init_resource::<UiKeymap>()
            .init_resource::<UiBindings>()
            .init_resource::<UiRebinder>()
            .init_resource::<UiInputContexts>()
            .init_resource::<nav::NavState>()
            .init_resource::<router::FocusMemory>()
            .init_resource::<router::ActivatedThisFrame>()
            .init_resource::<InputFocus>()
            .init_resource::<AutoNavigationConfig>()
            .add_message::<UiActionEvent>()
            .add_message::<UiRebindEvent>()
            .add_observer(router::record_activation)
            .add_systems(Last, (router::clear_activations, profile::sync_profile))
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
                (
                    keybinds::drive_rebind,
                    router::route_stack,
                    actions::apply_keymap,
                    actions::apply_rebinding,
                )
                    .chain()
                    .in_set(UiSystems::Route),
            )
            .add_systems(
                Update,
                (
                    router::sync_navigable,
                    router::dispatch_actions.run_if(ui_input_open),
                    nav::navigate.run_if(ui_input_open),
                )
                    .chain()
                    .in_set(UiSystems::Dispatch),
            );
    }
}
