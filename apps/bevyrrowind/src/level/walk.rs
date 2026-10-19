//! Walk mode in a level (TS1 part 2): `U` toggles between the free (fly)
//! camera and walking the level's player character (Morrowind's `player`
//! record at the `start` spawn, a level actor: actors.rs). Walking: WASD
//! relative to the camera's heading, Shift runs; the mouse still turns the
//! free camera (hold the right button or toggle the grab as usual), which
//! follows the character from behind (third person, a boom of
//! [`BOOM`] m). The character's `Mover` runs on the same ground probe as
//! every NPC (physics statics: the level's colliders). Scaffold: no camera
//! collision and no jump yet (the player controller is M7 work).

use bevy::camera_controller::free_camera::{FreeCamera, run_freecamera_controller};
use bevy::prelude::*;
use sf_controller::{Gait, LocomotionIntent};
use sf_controller_bevy::{ControllerSystems, MoveIntent, Mover};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::to_grid;

/// Walk speed, m/s (Morrowind's walk for an average character).
const WALK: f32 = 1.6;
/// Run speed, m/s.
const RUN: f32 = 4.5;
/// Camera distance behind the character's eyes, m.
const BOOM: f32 = 3.5;
/// The camera's pivot above the feet, m.
const PIVOT: f32 = 1.6;

/// Whether `U` handed the controls to the player character.
#[derive(Resource, Default)]
pub struct WalkMode {
    pub on: bool,
}

/// The level actor walk mode drives.
#[derive(Component)]
pub struct WalkPlayer;

pub fn add(app: &mut App) {
    app.init_resource::<WalkMode>()
        .add_systems(Update, (toggle, input).chain().in_set(ControllerSystems::Drive))
        .add_systems(Update, follow.after(ControllerSystems::Step).after(run_freecamera_controller))
        .add_systems(Update, report.after(ControllerSystems::Step));
}

/// Logs the player's start and stop (position, wanted and measured speed),
/// so a walk run can be checked from the log.
#[allow(clippy::needless_pass_by_value)]
fn report(mode: Res<WalkMode>, players: Query<(&Mover, &MoveIntent), With<WalkPlayer>>, mut moving: Local<bool>) {
    let Ok((mover, intent)) = players.single() else { return };
    let now = mode.on && intent.0.desired_velocity.length_squared() > 1e-4;
    if now != *moving {
        *moving = now;
        info!(
            "walk: player {} at {:.2?}, wants {:.2} m/s, measured {:.2} m/s",
            if now { "starts" } else { "stops" },
            mover.loco.position,
            intent.0.desired_velocity.length(),
            mover.measured.length()
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn toggle(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<WalkMode>, players: Query<(), With<WalkPlayer>>) {
    if !keys.just_pressed(KeyCode::KeyU) {
        return;
    }
    if players.is_empty() {
        info!("walk mode: no player character yet (it spawns at the level's `start` spawn)");
        return;
    }
    mode.on = !mode.on;
    info!("walk mode {} (U toggles; WASD, Shift runs)", if mode.on { "on" } else { "off: free camera" });
}

/// WASD in the camera's heading frame → the player's intent.
#[allow(clippy::needless_pass_by_value)]
fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mode: Res<WalkMode>,
    cams: Query<&Transform, With<FreeCamera>>,
    mut players: Query<&mut MoveIntent, With<WalkPlayer>>,
) {
    let Ok(mut intent) = players.single_mut() else { return };
    let cam = cams.single().ok();
    let (Some(cam), true) = (cam, mode.on) else {
        intent.0 = LocomotionIntent::IDLE;
        return;
    };
    let forward = (cam.rotation * Vec3::NEG_Z).with_y(0.0).normalize_or_zero();
    let right = Vec3::new(-forward.z, 0.0, forward.x);
    let axis = |pos: KeyCode, neg: KeyCode| f32::from(u8::from(keys.pressed(pos))) - f32::from(u8::from(keys.pressed(neg)));
    let dir = (forward * axis(KeyCode::KeyW, KeyCode::KeyS) + right * axis(KeyCode::KeyD, KeyCode::KeyA)).normalize_or_zero();
    let run = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    intent.0 = LocomotionIntent {
        desired_velocity: dir * if run { RUN } else { WALK },
        desired_facing: None,
        gait: if run { Gait::Run } else { Gait::Walk },
    };
}

/// The camera keeps the free camera's rotation and sits [`BOOM`] m behind
/// the character's pivot (after the free camera moved it: this wins).
#[allow(clippy::needless_pass_by_value)]
fn follow(mode: Res<WalkMode>, players: Query<&Mover, With<WalkPlayer>>, mut cams: Query<(&mut CellCoord, &mut Transform), With<FreeCamera>>) {
    if !mode.on {
        return;
    }
    let (Ok(mover), Ok((mut cell, mut transform))) = (players.single(), cams.single_mut()) else { return };
    let back = transform.rotation * Vec3::Z * BOOM;
    let eye = mover.loco.render_position().offset(Vec3::Y * PIVOT + back);
    let (c, t) = to_grid(eye);
    *cell = c;
    transform.translation = t.translation;
}
