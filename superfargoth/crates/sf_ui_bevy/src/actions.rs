//! Named UI actions and their rebindable bindings (ui.md "Input", "Keybinds").
//!
//! Every action is a `bevy_enhanced_input` action on a UI context entity. The
//! player's bindings live in [`UiKeymap`] (the Bevy-free `sf_ui::Keymap`, as
//! input names); [`UiBindings`] is its engine form. A keymap change rebuilds
//! `UiBindings`, which respawns the actions of every UI context, so a rebind
//! (menu open included) applies at once to open and future screens.

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_input::gamepad::GamepadAxis;
use bevy_math::Vec2;
use sf_ui::{ActionDef, ActionRef, Keymap, PressMode};

use crate::profile::{BINDINGS_CONTEXT, parse_inputs, validate_input};
use crate::router::{UiInputContexts, UiScreenInput};

/// Keymap scope of the HUD's actions (open menu, open map): live beside
/// gameplay, so a game links its gameplay scope to it
/// (`keymap.link_scopes("gameplay", HUD_SCOPE)`) to see conflicts.
pub const HUD_SCOPE: &str = "ui.hud";

/// The framework's UI actions. Screens and games react to them through
/// [`crate::UiActionEvent`]; navigation and confirm also drive focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UiAction {
    NavUp,
    NavDown,
    NavLeft,
    NavRight,
    /// Analog navigation (left stick), 2D with a dead zone.
    NavStick,
    Confirm,
    Back,
    TabPrev,
    TabNext,
    Context1,
    Context2,
    Context3,
    Context4,
    OpenMenu,
    OpenMap,
}

impl UiAction {
    pub const ALL: [UiAction; 15] = [
        UiAction::NavUp,
        UiAction::NavDown,
        UiAction::NavLeft,
        UiAction::NavRight,
        UiAction::NavStick,
        UiAction::Confirm,
        UiAction::Back,
        UiAction::TabPrev,
        UiAction::TabNext,
        UiAction::Context1,
        UiAction::Context2,
        UiAction::Context3,
        UiAction::Context4,
        UiAction::OpenMenu,
        UiAction::OpenMap,
    ];

    /// Actions a HUD-layer context carries: only the ones that open screens,
    /// so the HUD never swallows gameplay input such as the d-pad.
    pub const HUD: [UiAction; 2] = [UiAction::OpenMenu, UiAction::OpenMap];

    /// Stable name (rebinding files, glyph lookups, logs).
    pub fn name(self) -> &'static str {
        match self {
            UiAction::NavUp => "nav_up",
            UiAction::NavDown => "nav_down",
            UiAction::NavLeft => "nav_left",
            UiAction::NavRight => "nav_right",
            UiAction::NavStick => "nav_stick",
            UiAction::Confirm => "confirm",
            UiAction::Back => "back",
            UiAction::TabPrev => "tab_prev",
            UiAction::TabNext => "tab_next",
            UiAction::Context1 => "context_1",
            UiAction::Context2 => "context_2",
            UiAction::Context3 => "context_3",
            UiAction::Context4 => "context_4",
            UiAction::OpenMenu => "open_menu",
            UiAction::OpenMap => "open_map",
        }
    }

    /// The action with this [`name`](Self::name).
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.name() == name)
    }

    /// The action in the keymap (context [`BINDINGS_CONTEXT`]).
    pub fn id(self) -> ActionRef {
        ActionRef::new(BINDINGS_CONTEXT, self.name())
    }

    /// Keymap definition with the defaults: console and PC convention
    /// (A/Enter confirm, B/Esc back, bumpers tab, Start/Esc menu, View/M map).
    /// Navigation, confirm, back and menu open are required: rebindable, never
    /// left unbound on a device.
    pub fn def(self) -> ActionDef {
        let name = self.name();
        let button =
            |defaults: &[&str]| ActionDef::button(BINDINGS_CONTEXT, name, defaults.iter().copied());
        match self {
            UiAction::NavUp => button(&["key:ArrowUp", "pad:DPadUp"]).required(),
            UiAction::NavDown => button(&["key:ArrowDown", "pad:DPadDown"]).required(),
            UiAction::NavLeft => button(&["key:ArrowLeft", "pad:DPadLeft"]).required(),
            UiAction::NavRight => button(&["key:ArrowRight", "pad:DPadRight"]).required(),
            UiAction::NavStick => ActionDef::stick(BINDINGS_CONTEXT, name, ["stick:Left"]),
            UiAction::Confirm => button(&["key:Enter", "pad:South"]).required(),
            UiAction::Back => button(&["key:Escape", "key:Backspace", "pad:East"]).required(),
            UiAction::TabPrev => button(&["key:KeyQ", "pad:LeftTrigger"]),
            UiAction::TabNext => button(&["key:KeyE", "pad:RightTrigger"]),
            UiAction::Context1 => button(&["key:KeyF", "pad:West"]),
            UiAction::Context2 => button(&["key:KeyR", "pad:North"]),
            UiAction::Context3 => button(&["key:KeyX", "pad:LeftThumb"]),
            UiAction::Context4 => button(&["key:KeyC", "pad:RightThumb"]),
            UiAction::OpenMenu => button(&["key:Escape", "pad:Start"])
                .required()
                .also_in(HUD_SCOPE),
            UiAction::OpenMap => button(&["key:KeyM", "pad:Select"]).also_in(HUD_SCOPE),
        }
    }
}

/// A keymap holding the UI actions at their defaults, with the engine's input
/// check. Back and open menu share Esc by design (Esc closes a menu and opens
/// the main menu from play).
pub fn ui_keymap() -> Keymap {
    let mut keymap = Keymap::new();
    keymap.set_validator(validate_input);
    for action in UiAction::ALL {
        keymap
            .register(action.def())
            .expect("UI action defaults are valid");
    }
    keymap.allow_shared(UiAction::Back.id(), UiAction::OpenMenu.id());
    keymap
}

/// The player's keymap: the UI actions, plus any actions a game registers
/// (its gameplay context) so one keybinds screen and one profile table cover
/// both. Edit it to rebind (or through [`crate::UiRebinder`]); changes reach
/// [`UiBindings`] and the profile.
#[derive(Resource, Clone, Debug)]
pub struct UiKeymap(pub Keymap);

impl Default for UiKeymap {
    fn default() -> Self {
        Self(ui_keymap())
    }
}

impl UiKeymap {
    /// Glyph of the first input bound to a UI action on `device`.
    pub fn glyph(
        &self,
        action: UiAction,
        device: sf_ui::Device,
        family: sf_ui::PadFamily,
    ) -> Option<sf_ui::Glyph> {
        self.0.glyph(&action.id(), device, family)
    }
}

/// Engine bindings of any keymap action (a game's gameplay actions too),
/// keyboard/mouse first. A stick becomes its two axes: feed the vertical one
/// to Y ([`is_vertical_axis`]).
pub fn keymap_bindings(keymap: &Keymap, action: &ActionRef) -> Vec<Binding> {
    keymap
        .bound_all(action)
        .filter_map(|name| parse_inputs(name).ok())
        .flatten()
        .collect()
}

/// A stick's vertical axis (it feeds the Y of a 2D action: `SwizzleAxis::YXZ`).
pub fn is_vertical_axis(binding: &Binding) -> bool {
    matches!(
        binding,
        Binding::GamepadAxis(GamepadAxis::LeftStickY | GamepadAxis::RightStickY)
    )
}

/// The input condition for an action's hold / toggle mode
/// ([`sf_ui::PressMode`]): toggle is `bevy_enhanced_input`'s [`Toggle`] on
/// the action entity (a press switches it on, the next one off); hold, or no
/// choice, is the plain input state (`None`).
pub fn press_condition(keymap: &Keymap, action: &ActionRef) -> Option<Toggle> {
    (keymap.press_mode(action) == Some(PressMode::Toggle)).then(Toggle::default)
}

/// Tags each spawned action entity with its [`UiAction`], so one query reads
/// every UI action in its dynamically typed form.
#[derive(Component, Clone, Copy, Debug)]
pub struct UiActionTag(pub UiAction);

macro_rules! ui_actions {
    ($($ty:ident: $out:ty;)*) => {
        $(
            #[derive(InputAction)]
            #[action_output($out)]
            pub struct $ty;
        )*
    };
}

ui_actions! {
    UiNavUp: bool;
    UiNavDown: bool;
    UiNavLeft: bool;
    UiNavRight: bool;
    UiNavStick: Vec2;
    UiConfirm: bool;
    UiBack: bool;
    UiTabPrev: bool;
    UiTabNext: bool;
    UiContext1: bool;
    UiContext2: bool;
    UiContext3: bool;
    UiContext4: bool;
    UiOpenMenu: bool;
    UiOpenMap: bool;
}

/// Engine bindings per UI action, keyboard/mouse and gamepad together, built
/// from [`UiKeymap`] whenever it changes (rebind through the keymap: it is
/// what the profile saves). The default is [`ui_keymap`]'s.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct UiBindings(pub Vec<(UiAction, Vec<Binding>)>);

impl UiBindings {
    /// The UI actions' bindings in `keymap` (keyboard/mouse first). A stick
    /// becomes its two axes.
    pub fn from_keymap(keymap: &Keymap) -> Self {
        Self(
            UiAction::ALL
                .into_iter()
                .map(|action| (action, keymap_bindings(keymap, &action.id())))
                .collect(),
        )
    }

    pub fn get(&self, action: UiAction) -> &[Binding] {
        self.0
            .iter()
            .find(|(a, _)| *a == action)
            .map_or(&[], |(_, b)| b.as_slice())
    }

    /// Replaces the bindings of one action.
    pub fn set(&mut self, action: UiAction, bindings: Vec<Binding>) {
        match self.0.iter_mut().find(|(a, _)| *a == action) {
            Some(entry) => entry.1 = bindings,
            None => self.0.push((action, bindings)),
        }
    }
}

impl Default for UiBindings {
    fn default() -> Self {
        Self::from_keymap(&ui_keymap())
    }
}

/// Keymap → engine bindings, only when they differ (a capture that changed
/// nothing doesn't respawn actions).
pub(crate) fn apply_keymap(keymap: Res<UiKeymap>, mut bindings: ResMut<UiBindings>) {
    if keymap.is_changed() {
        bindings.set_if_neq(UiBindings::from_keymap(&keymap.0));
    }
}

/// Spawns `actions` with their bindings on the UI context entity `context`.
pub(crate) fn spawn_actions(
    commands: &mut Commands,
    context: Entity,
    actions: &[UiAction],
    bindings: &UiBindings,
) {
    for &action in actions {
        let settings = ActionSettings {
            // A context that opens or regains input while its button is still
            // held must not fire (Start opens the menu; the menu's Start
            // must not close it again in the next frame).
            require_reset: true,
            consume_input: true,
            ..Default::default()
        };
        let mut entity = commands.spawn((
            ActionOf::<UiScreenInput>::new(context),
            UiActionTag(action),
            settings,
        ));
        match action {
            UiAction::NavUp => entity.insert(Action::<UiNavUp>::new()),
            UiAction::NavDown => entity.insert(Action::<UiNavDown>::new()),
            UiAction::NavLeft => entity.insert(Action::<UiNavLeft>::new()),
            UiAction::NavRight => entity.insert(Action::<UiNavRight>::new()),
            UiAction::NavStick => entity.insert((Action::<UiNavStick>::new(), DeadZone::default())),
            UiAction::Confirm => entity.insert(Action::<UiConfirm>::new()),
            UiAction::Back => entity.insert(Action::<UiBack>::new()),
            UiAction::TabPrev => entity.insert(Action::<UiTabPrev>::new()),
            UiAction::TabNext => entity.insert(Action::<UiTabNext>::new()),
            UiAction::Context1 => entity.insert(Action::<UiContext1>::new()),
            UiAction::Context2 => entity.insert(Action::<UiContext2>::new()),
            UiAction::Context3 => entity.insert(Action::<UiContext3>::new()),
            UiAction::Context4 => entity.insert(Action::<UiContext4>::new()),
            UiAction::OpenMenu => entity.insert(Action::<UiOpenMenu>::new()),
            UiAction::OpenMap => entity.insert(Action::<UiOpenMap>::new()),
        };
        let action_entity = entity.id();
        for binding in bindings.get(action) {
            let mut b = commands.spawn((*binding, BindingOf(action_entity)));
            // A vertical axis feeds the Y of a 2D action.
            if is_vertical_axis(binding) {
                b.insert(SwizzleAxis::YXZ);
            }
        }
    }
}

/// Rebinding: when [`UiBindings`] changes, respawn the actions of every UI
/// context with the new bindings.
pub(crate) fn apply_rebinding(
    mut commands: Commands,
    bindings: Res<UiBindings>,
    contexts: Res<UiInputContexts>,
    actions: Query<&Actions<UiScreenInput>>,
) {
    if !bindings.is_changed() || bindings.is_added() {
        return;
    }
    for (context, set) in contexts.entities() {
        if let Ok(existing) = actions.get(context) {
            for action in existing {
                commands.entity(action).despawn();
            }
        }
        spawn_actions(&mut commands, context, set, &bindings);
    }
}
