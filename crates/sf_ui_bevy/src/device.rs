//! Last-used device and button glyphs (ui.md "Input": the last-used device
//! switches glyphs and focus visuals within the frame; it never swaps layout).

use bevy_ecs::prelude::*;
use bevy_input::ButtonState;
use bevy_input::gamepad::{Gamepad, GamepadButton};
use bevy_input::keyboard::KeyboardInput;
use bevy_input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
pub use sf_ui::{Device, Glyph, PadFamily};

use crate::actions::{UiAction, UiKeymap};

/// The last-used input device (the root attribute `input=kbm|gamepad` for
/// stylesheets in U2).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputMode(pub Device);

/// Family of the last-used gamepad (from its USB vendor id), for glyphs:
/// the same `pad:South` reads A, Cross or B.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GamepadFamily(pub PadFamily);

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

#[allow(clippy::too_many_arguments)]
pub(crate) fn track_input_mode(
    mut mode: ResMut<InputMode>,
    mut family: ResMut<GamepadFamily>,
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
        let mut used = false;
        for &button in gamepad.get_pressed() {
            used |= !edges.pressed.contains(&(entity, button));
            pressed.push((entity, button));
        }
        if gamepad.left_stick().length() > AXIS_USE || gamepad.right_stick().length() > AXIS_USE {
            used |= !edges.deflected.contains(&entity);
            deflected.push(entity);
        }
        if used && let Some(vendor) = gamepad.vendor_id() {
            family.set_if_neq(GamepadFamily(PadFamily::from_vendor_id(vendor)));
        }
        pad |= used;
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

/// Glyph of the first input bound to `action` on `device`, for the pad
/// family (labels and icon ids from `sf_ui::glyph`; icon atlases per family
/// come with the widget kit, U3). `None` when nothing on that device is bound.
pub fn glyph(
    keymap: &UiKeymap,
    action: UiAction,
    device: Device,
    family: PadFamily,
) -> Option<Glyph> {
    keymap.glyph(action, device, family)
}
