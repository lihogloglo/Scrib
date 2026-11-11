//! Playback control (video.md "Design", V3b): skip on input, pause while hidden or the window is
//! minimised, and duck the other buses while a sounded video plays. See the crate docs for the rules.

use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_input::ButtonInput;
use bevy_input::gamepad::{Gamepad, GamepadButton};
use bevy_input::keyboard::KeyCode;
use bevy_input::mouse::MouseButton;
use bevy_time::{Real, Time};
use bevy_window::{PrimaryWindow, Window, WindowOccluded};
use sf_audio::{Bus, DuckRule, DuckTrigger};
use sf_audio_bevy::AudioDucking;

use crate::{VideoFinished, VideoPauseWhenHidden, VideoPlayer, VideoSkipped};

/// Keys that skip a skippable video (confirm, back, the space bar).
pub const SKIP_KEYS: [KeyCode; 3] = [KeyCode::Escape, KeyCode::Enter, KeyCode::Space];
/// Mouse buttons that skip a skippable video (a click).
pub const SKIP_MOUSE: [MouseButton; 1] = [MouseButton::Left];
/// Gamepad buttons that skip a skippable video (confirm, back, start).
pub const SKIP_PAD: [GamepadButton; 3] = [
    GamepadButton::South,
    GamepadButton::East,
    GamepadButton::Start,
];

/// The `sf_audio_bevy` ducking hold on while a sounded video plays.
pub const VIDEO_DUCK_HOLD: &str = "video";
/// How far [`DUCKED_BUSES`] go down under a sounded video: near silence (`OpenMW` pauses them).
pub const VIDEO_DUCK_DB: f32 = 60.0;
/// The buses a sounded video ducks (`Weather` follows `Ambience`; UI and the movie itself stay).
pub const DUCKED_BUSES: [Bus; 4] = [Bus::Music, Bus::Ambience, Bus::Sfx, Bus::Voice];
const DUCK_ATTACK_S: f32 = 0.25;
const DUCK_RELEASE_S: f32 = 1.0;

/// Ends every playing skippable video on a just-pressed skip input, consuming the press.
pub(crate) fn skip_players(
    keys: Option<ResMut<ButtonInput<KeyCode>>>,
    mouse: Option<ResMut<ButtonInput<MouseButton>>>,
    mut pads: Query<&mut Gamepad>,
    mut players: Query<(Entity, &mut VideoPlayer)>,
    mut skipped: MessageWriter<VideoSkipped>,
    mut finished: MessageWriter<VideoFinished>,
) {
    let skippable = |p: &VideoPlayer| p.is_skippable() && p.is_playing();
    if !players.iter().any(|(_, p)| skippable(p)) {
        return;
    }
    let mut pressed = false;
    if let Some(mut keys) = keys
        && SKIP_KEYS.iter().any(|k| keys.just_pressed(*k))
    {
        for key in SKIP_KEYS {
            pressed |= keys.clear_just_pressed(key);
        }
    }
    if let Some(mut mouse) = mouse
        && SKIP_MOUSE.iter().any(|b| mouse.just_pressed(*b))
    {
        for button in SKIP_MOUSE {
            pressed |= mouse.clear_just_pressed(button);
        }
    }
    for mut pad in &mut pads {
        if SKIP_PAD.iter().any(|b| pad.just_pressed(*b)) {
            for button in SKIP_PAD {
                pressed |= pad.digital_mut().clear_just_pressed(button);
            }
        }
    }
    if !pressed {
        return;
    }
    for (entity, mut player) in &mut players {
        if skippable(&player) {
            player.stop();
            skipped.write(VideoSkipped { entity });
            finished.write(VideoFinished { entity });
        }
    }
}

/// The primary window's state the players pause on.
#[derive(SystemParam)]
pub(crate) struct WindowState<'w, 's> {
    windows: Query<'w, 's, (Entity, &'static Window), With<PrimaryWindow>>,
    occlusion: Option<MessageReader<'w, 's, WindowOccluded>>,
    /// The primary window's last `WindowOccluded` state (winit reports it on macOS, iOS, X11, Web).
    occluded: Local<'s, bool>,
}

impl WindowState<'_, '_> {
    /// Whether the primary window is minimised or otherwise out of sight. Windows reports a minimised
    /// window as a zero-size resize (winit has no `Occluded` there); no primary window: not hidden.
    fn hidden(&mut self) -> bool {
        let primary = self.windows.single().ok();
        if let Some(reader) = self.occlusion.as_mut() {
            for message in reader.read() {
                if primary.is_some_and(|(entity, _)| entity == message.window) {
                    *self.occluded = message.occluded;
                }
            }
        }
        let Some((_, window)) = primary else {
            return false;
        };
        *self.occluded
            || !window.visible
            || window.physical_width() == 0
            || window.physical_height() == 0
    }
}

/// Pauses players that are hidden or whose window is minimised, and resumes them when they show again.
pub(crate) fn pause_players(
    time: Res<Time<Real>>,
    when_hidden: Option<Res<VideoPauseWhenHidden>>,
    mut window: WindowState,
    mut players: Query<(&mut VideoPlayer, Option<&InheritedVisibility>)>,
) {
    let minimised = window.hidden();
    let check_visibility = when_hidden.is_some_and(|w| w.0);
    let now = time.elapsed();
    for (mut player, visibility) in &mut players {
        let hidden = check_visibility && visibility.is_some_and(|v| !v.get());
        let paused = minimised || hidden;
        if player.is_paused() != paused {
            player.set_paused(paused, now);
        }
    }
}

/// Holds [`VIDEO_DUCK_HOLD`] while any player's sound track plays (adding its rules on first use).
pub(crate) fn duck_other_audio(
    ducking: Option<ResMut<AudioDucking>>,
    players: Query<&VideoPlayer>,
) {
    let Some(mut ducking) = ducking else {
        return;
    };
    let sounding = players.iter().any(VideoPlayer::is_sounding);
    if ducking.0.is_held(VIDEO_DUCK_HOLD) == sounding {
        return;
    }
    let trigger = DuckTrigger::Hold(VIDEO_DUCK_HOLD.to_owned());
    if !ducking.0.rules().iter().any(|r| r.trigger == trigger) {
        for target in DUCKED_BUSES {
            ducking.0.add_rule(DuckRule {
                trigger: trigger.clone(),
                target,
                depth_db: VIDEO_DUCK_DB,
                attack_s: DUCK_ATTACK_S,
                release_s: DUCK_RELEASE_S,
            });
        }
    }
    ducking.0.set_hold(VIDEO_DUCK_HOLD, sounding);
}
