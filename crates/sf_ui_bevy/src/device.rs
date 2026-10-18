//! Last-used device and button glyphs (ui.md "Input": the last-used device
//! switches glyphs and focus visuals within the frame; it never swaps layout).

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::Binding;
use bevy_input::ButtonState;
use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
use bevy_input::keyboard::{KeyCode, KeyboardInput};
use bevy_input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};

use crate::actions::{UiAction, UiBindings};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Device {
    #[default]
    Kbm,
    Gamepad,
}

/// The last-used input device (the root attribute `input=kbm|gamepad` for
/// stylesheets in U2).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputMode(pub Device);

/// Stick deflection that counts as gamepad use (ignores drift).
const AXIS_USE: f32 = 0.5;
/// Mouse travel per frame, in pixels, that counts as mouse use.
const MOUSE_USE: f32 = 4.0;

/// Gamepad state seen last frame: edges are detected on the `Gamepad`
/// components (what actions read), not on raw messages, so any source that
/// drives the components counts.
#[derive(Default)]
pub(crate) struct PadEdges {
    pressed: Vec<(Entity, GamepadButton)>,
    deflected: Vec<Entity>,
}

pub(crate) fn track_input_mode(
    mut mode: ResMut<InputMode>,
    mut keys: MessageReader<KeyboardInput>,
    mut buttons: MessageReader<MouseButtonInput>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut edges: Local<PadEdges>,
) {
    // `|`, not `||`: every reader is drained each frame.
    let kbm = keys.read().any(|k| k.state == ButtonState::Pressed)
        | buttons.read().any(|b| b.state == ButtonState::Pressed)
        | (motion
            .read()
            .map(|m| m.delta)
            .sum::<bevy_math::Vec2>()
            .length()
            > MOUSE_USE)
        | (wheel.read().count() > 0);
    let mut pad = false;
    let mut pressed = Vec::new();
    let mut deflected = Vec::new();
    for (entity, gamepad) in &gamepads {
        for &button in gamepad.get_pressed() {
            pad |= !edges.pressed.contains(&(entity, button));
            pressed.push((entity, button));
        }
        if gamepad.left_stick().length() > AXIS_USE || gamepad.right_stick().length() > AXIS_USE {
            pad |= !edges.deflected.contains(&entity);
            deflected.push(entity);
        }
    }
    *edges = PadEdges { pressed, deflected };
    // Both in one frame: the gamepad wins (mouse jitter is common, a stray
    // pad press is not).
    let device = if pad {
        Device::Gamepad
    } else if kbm {
        Device::Kbm
    } else {
        return;
    };
    if mode.0 != device {
        mode.0 = device;
    }
}

/// Label of the first input bound to `action` on `device` (our own small
/// table, Xbox-style names; icon atlases per pad family come with the widget
/// kit, U3). `None` when nothing on that device is bound.
pub fn glyph(bindings: &UiBindings, action: UiAction, device: Device) -> Option<String> {
    bindings.get(action).iter().find_map(|b| match (device, b) {
        (Device::Gamepad, Binding::GamepadButton(button)) => Some(pad_button(*button).to_owned()),
        (Device::Gamepad, Binding::GamepadAxis(axis)) => Some(pad_axis(*axis).to_owned()),
        (Device::Kbm, Binding::Keyboard { key, .. }) => Some(key_name(*key)),
        (Device::Kbm, Binding::MouseButton { button, .. }) => Some(format!("Mouse {button:?}")),
        _ => None,
    })
}

fn pad_button(button: GamepadButton) -> &'static str {
    use GamepadButton as G;
    match button {
        G::South => "A",
        G::East => "B",
        G::West => "X",
        G::North => "Y",
        G::LeftTrigger => "LB",
        G::RightTrigger => "RB",
        G::LeftTrigger2 => "LT",
        G::RightTrigger2 => "RT",
        G::Select => "View",
        G::Start => "Menu",
        G::LeftThumb => "LS",
        G::RightThumb => "RS",
        G::DPadUp => "D-pad Up",
        G::DPadDown => "D-pad Down",
        G::DPadLeft => "D-pad Left",
        G::DPadRight => "D-pad Right",
        _ => "?",
    }
}

fn pad_axis(axis: GamepadAxis) -> &'static str {
    match axis {
        GamepadAxis::LeftStickX | GamepadAxis::LeftStickY => "Left Stick",
        GamepadAxis::RightStickX | GamepadAxis::RightStickY => "Right Stick",
        _ => "?",
    }
}

fn key_name(key: KeyCode) -> String {
    match key {
        KeyCode::Escape => "Esc".to_owned(),
        KeyCode::ArrowUp => "Up".to_owned(),
        KeyCode::ArrowDown => "Down".to_owned(),
        KeyCode::ArrowLeft => "Left".to_owned(),
        KeyCode::ArrowRight => "Right".to_owned(),
        other => {
            let name = format!("{other:?}");
            name.strip_prefix("Key")
                .or_else(|| name.strip_prefix("Digit"))
                .map_or_else(|| name.clone(), str::to_owned)
        }
    }
}
