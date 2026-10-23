//! The camera's `bevy_enhanced_input` actions, on the local player's [`PlayerControls`] context
//! (one context per local player, shared with the action layer's buttons). Bindings are the
//! host's (keybind settings); this module defines the actions only, so no key is hard-coded.

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_math::Vec2;
use sf_action_bevy::PlayerControls;

/// Look input this frame, radians: `x` turns right, `y` looks up. The host's bindings carry the
/// units: mouse motion with a `Scale` (sensitivity, rad per count) and a `Negate::y()`; a stick
/// with `DeltaScale` and a `Scale` (rad/s), so a held stick turns at the same rate at any frame
/// rate.
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct CameraLook;

/// Zoom steps this frame, positive in (a wheel notch is 1).
#[derive(InputAction)]
#[action_output(f32)]
pub struct CameraZoom;

/// First/third-person switch (on press).
#[derive(InputAction)]
#[action_output(bool)]
pub struct CameraToggleView;

/// Shoulder side swap (on press).
#[derive(InputAction)]
#[action_output(bool)]
pub struct CameraSwapShoulder;

/// Tags each spawned camera action entity with what it feeds, so one query reads them all.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraAction {
    Look,
    Zoom,
    ToggleView,
    SwapShoulder,
}

impl CameraAction {
    pub const ALL: [Self; 4] = [Self::Look, Self::Zoom, Self::ToggleView, Self::SwapShoulder];
}

/// Spawns the camera's action entities on `context` (the player's entity with
/// [`PlayerControls`]), unbound. Returns them with their tags so the host can attach bindings.
pub fn spawn_camera_actions(commands: &mut Commands, context: Entity) -> Vec<(CameraAction, Entity)> {
    CameraAction::ALL
        .into_iter()
        .map(|tag| {
            let mut e = commands.spawn((ActionOf::<PlayerControls>::new(context), tag));
            match tag {
                CameraAction::Look => e.insert(Action::<CameraLook>::new()),
                CameraAction::Zoom => e.insert(Action::<CameraZoom>::new()),
                CameraAction::ToggleView => e.insert(Action::<CameraToggleView>::new()),
                CameraAction::SwapShoulder => e.insert(Action::<CameraSwapShoulder>::new()),
            };
            (tag, e.id())
        })
        .collect()
}

/// One frame of camera input, gathered from a context's action entities.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CameraInput {
    pub look: Vec2,
    pub zoom: f32,
    pub toggle_view: bool,
    pub swap_shoulder: bool,
}

impl CameraInput {
    pub(crate) fn read(&mut self, tag: CameraAction, value: ActionValue, events: ActionEvents) {
        match tag {
            CameraAction::Look => self.look += value.as_axis2d(),
            CameraAction::Zoom => self.zoom += value.as_axis1d(),
            CameraAction::ToggleView => self.toggle_view |= events.contains(ActionEvents::START),
            CameraAction::SwapShoulder => self.swap_shoulder |= events.contains(ActionEvents::START),
        }
    }
}
