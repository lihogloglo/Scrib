//! Named UI actions and their rebindable bindings (ui.md "Input").
//!
//! Every action is a `bevy_enhanced_input` action on a UI context entity. The
//! bindings come from [`UiBindings`]; changing that resource respawns the
//! actions of every UI context, so a rebind (menu open included) applies at
//! once to open and future screens.

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_input::gamepad::{GamepadAxis, GamepadButton};
use bevy_input::keyboard::KeyCode;
use bevy_math::Vec2;

use crate::router::{UiInputContexts, UiScreenInput};

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

/// Bindings per UI action, keyboard/mouse and gamepad together. Defaults
/// follow console and PC convention (A/Enter confirm, B/Esc back, bumpers
/// tab, Start/Esc menu, Back/M map). Edit to rebind; all are rebindable.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct UiBindings(pub Vec<(UiAction, Vec<Binding>)>);

impl UiBindings {
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
        use GamepadButton as G;
        use KeyCode as K;
        let b = |list: &[Binding]| list.to_vec();
        Self(vec![
            (UiAction::NavUp, b(&[K::ArrowUp.into(), G::DPadUp.into()])),
            (
                UiAction::NavDown,
                b(&[K::ArrowDown.into(), G::DPadDown.into()]),
            ),
            (
                UiAction::NavLeft,
                b(&[K::ArrowLeft.into(), G::DPadLeft.into()]),
            ),
            (
                UiAction::NavRight,
                b(&[K::ArrowRight.into(), G::DPadRight.into()]),
            ),
            (
                UiAction::NavStick,
                b(&[
                    GamepadAxis::LeftStickX.into(),
                    GamepadAxis::LeftStickY.into(),
                ]),
            ),
            (UiAction::Confirm, b(&[K::Enter.into(), G::South.into()])),
            (
                UiAction::Back,
                b(&[K::Escape.into(), K::Backspace.into(), G::East.into()]),
            ),
            (
                UiAction::TabPrev,
                b(&[K::KeyQ.into(), G::LeftTrigger.into()]),
            ),
            (
                UiAction::TabNext,
                b(&[K::KeyE.into(), G::RightTrigger.into()]),
            ),
            (UiAction::Context1, b(&[K::KeyF.into(), G::West.into()])),
            (UiAction::Context2, b(&[K::KeyR.into(), G::North.into()])),
            (
                UiAction::Context3,
                b(&[K::KeyX.into(), G::LeftThumb.into()]),
            ),
            (
                UiAction::Context4,
                b(&[K::KeyC.into(), G::RightThumb.into()]),
            ),
            (UiAction::OpenMenu, b(&[K::Escape.into(), G::Start.into()])),
            (UiAction::OpenMap, b(&[K::KeyM.into(), G::Select.into()])),
        ])
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
            if matches!(
                binding,
                Binding::GamepadAxis(GamepadAxis::LeftStickY | GamepadAxis::RightStickY)
            ) {
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
