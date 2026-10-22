//! The keybinds screen's engine side (ui.md "Keybinds", U6): feeds the
//! Bevy-free `sf_ui::Rebinder` the inputs of each frame as input names, and
//! keeps UI actions quiet while a capture runs.
//!
//! A screen starts a capture with `UiRebinder::start` (a slot of a
//! [`UiKeymap`] action), reads [`UiRebindEvent`]s (applied, conflict, refused,
//! cancelled), and answers a conflict with `UiRebinder::resolve`. The applied
//! binding reaches [`crate::UiBindings`] and the profile like any keymap edit.

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::{Binding, ModKeys};
use bevy_input::ButtonInput;
use bevy_input::gamepad::Gamepad;
use bevy_input::keyboard::KeyCode;
use bevy_input::mouse::MouseButton;
use bevy_time::Time;
use sf_ui::{CaptureFrame, KeymapError, RebindEvent, Rebinder, Resolution, SlotRef};

use crate::actions::UiKeymap;
use crate::profile::binding_name;

/// Stick deflection that counts as a press while capturing.
const STICK_PRESS: f32 = 0.5;

/// The rebinding flow of the keybinds screen.
#[derive(Resource, Debug, Default)]
pub struct UiRebinder(pub Rebinder);

impl UiRebinder {
    /// Starts capturing an input for `slot`.
    pub fn start(&mut self, keymap: &UiKeymap, slot: SlotRef) -> Result<(), KeymapError> {
        self.0.start(&keymap.0, slot)
    }

    /// Applies the player's choice for the pending conflict.
    pub fn resolve(
        &mut self,
        keymap: &mut UiKeymap,
        resolution: Resolution,
    ) -> Option<RebindEvent> {
        self.0.resolve(&mut keymap.0, resolution)
    }
}

/// What the rebinding flow did this frame.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct UiRebindEvent(pub RebindEvent);

/// Run condition: UI actions may fire (no capture is running or draining).
pub fn ui_input_open(rebinder: Res<UiRebinder>) -> bool {
    !rebinder.0.blocks_ui()
}

/// Inputs of this frame as names, then one rebinder step. Touches the keymap
/// (change detection) only when a binding changed.
///
/// A press is a name held now and not last frame (edges on the input state,
/// like the device tracker, so any source driving the `Gamepad` components
/// counts), plus keys and buttons pressed and released within this frame.
#[allow(clippy::too_many_arguments)]
pub(crate) fn drive_rebind(
    time: Res<Time>,
    keys: Option<Res<ButtonInput<KeyCode>>>,
    mouse: Option<Res<ButtonInput<MouseButton>>>,
    gamepads: Query<&Gamepad>,
    mut rebinder: ResMut<UiRebinder>,
    mut keymap: ResMut<UiKeymap>,
    mut out: MessageWriter<UiRebindEvent>,
    mut last_held: Local<Vec<String>>,
) {
    let mut held = Vec::new();
    let mut tapped = Vec::new();
    let add = |list: &mut Vec<String>, binding: Binding| {
        if let Some(name) = binding_name(&binding)
            && !list.contains(&name)
        {
            list.push(name);
        }
    };
    if let Some(keys) = &keys {
        let key = |key| Binding::Keyboard {
            key,
            mod_keys: ModKeys::empty(),
        };
        keys.get_pressed().for_each(|k| add(&mut held, key(*k)));
        keys.get_just_pressed()
            .for_each(|k| add(&mut tapped, key(*k)));
    }
    if let Some(mouse) = &mouse {
        let button = |button| Binding::MouseButton {
            button,
            mod_keys: ModKeys::empty(),
        };
        mouse.get_pressed().for_each(|b| add(&mut held, button(*b)));
        mouse
            .get_just_pressed()
            .for_each(|b| add(&mut tapped, button(*b)));
    }
    for gamepad in &gamepads {
        for &button in gamepad.get_pressed() {
            add(&mut held, Binding::GamepadButton(button));
        }
        for (stick, value) in [
            ("Left", gamepad.left_stick()),
            ("Right", gamepad.right_stick()),
        ] {
            let name = format!("stick:{stick}");
            if value.length() > STICK_PRESS && !held.contains(&name) {
                held.push(name);
            }
        }
    }
    let mut pressed: Vec<String> = held
        .iter()
        .filter(|n| !last_held.contains(n))
        .cloned()
        .collect();
    for name in tapped {
        if !pressed.contains(&name) && !last_held.contains(&name) {
            pressed.push(name);
        }
    }
    last_held.clone_from(&held);

    if rebinder.0.capturing().is_none() && !rebinder.0.blocks_ui() {
        return;
    }
    let version = keymap.0.version();
    let event = rebinder.0.update(
        &mut keymap.bypass_change_detection().0,
        time.delta_secs(),
        CaptureFrame {
            pressed: &pressed,
            held: &held,
        },
    );
    if keymap.0.version() != version {
        keymap.set_changed();
    }
    if let Some(event) = event {
        out.write(UiRebindEvent(event));
    }
}
