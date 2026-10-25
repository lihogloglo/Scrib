//! Physics impact and slide sounds (audio.md "Impacts"): [`ContactSound`] messages (one per
//! contact report, positions in the listener's frame) go through `sf_audio::ImpactAudio`; hits
//! play as cues at the contact point ([`PlaySoundScaled`]), slides as looping positional voices
//! that follow the latest contact. The physics bridge is the `physics` feature's
//! `physics::PhysicsAudioPlugin`; anything else that reports contacts can write the messages.

use std::collections::HashMap;

use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Real, Time};
use bevy_audio::SpatialListener;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_audio::{ContactSample, ImpactAudio, SlideChange};

use crate::{
    AudioBank, AudioCues, AudioEvent, AudioMixer, CueInstance, Playback, PlaySound, SoundAt, Start, Voice, listener_pos, spawn_voice,
};

/// The impact state: fill `.0.map` (surface pairs -> cues) from the content adapter.
#[derive(Resource, Default, Clone, Debug)]
pub struct ContactAudio(pub ImpactAudio);

/// One physics contact report (`sample.at` in the listener's frame: `GlobalTransform` space).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ContactSound(pub ContactSample);

/// A cue played at `volume` times its own (impacts scaled by speed; any caller may use it).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct PlaySoundScaled {
    pub play: PlaySound,
    pub volume: f32,
}

/// Lowest level of a playing slide voice (it stops on the core's `Stop`, not by fading to 0).
const MIN_LEVEL: f32 = 1e-4;

/// A slide loop's voice, keyed by the sliding body.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlideVoice(pub u64);

/// Runs the frame's contacts through the impact state: hits become [`PlaySoundScaled`], slide
/// loops start, follow their contact, and stop.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
pub(crate) fn play_contacts(
    time: Res<Time<Real>>,
    mut contacts: MessageReader<ContactSound>,
    mut state: ResMut<ContactAudio>,
    mut cues: ResMut<AudioCues>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    mut slides: Query<(&mut Voice, &mut Transform), With<SlideVoice>>,
    mut playing: Local<HashMap<u64, Entity>>,
    mut plays: MessageWriter<PlaySoundScaled>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
) {
    let samples: Vec<ContactSample> = contacts.read().map(|c| c.0).collect();
    if samples.is_empty() && state.0.slide_count() == 0 {
        return;
    }
    let frame = state.0.tick(time.delta_secs(), &samples);
    for hit in frame.impacts {
        let play = PlaySound { cue: hit.cue, at: SoundAt::Point(Vec3::from_array(hit.at)) };
        plays.write(PlaySoundScaled { play, volume: hit.volume });
    }
    let server = if playback.0 { server.as_deref() } else { None };
    let ear = listener_pos(&listener);
    for change in frame.slides {
        match change {
            SlideChange::Start { body, cue, level, at } => {
                if let Some(old) = playing.remove(&body) {
                    commands.entity(old).try_despawn();
                }
                let Some(cue) = cues.0.resolve(&cue) else { continue };
                let Some(file) = bank.0.get(&cue.sound) else { continue };
                events.write(AudioEvent::Cue { cue: cue.cue.clone(), sound: cue.sound.clone(), bus: cue.bus });
                // The core eases the level: the voice's target follows it, so the loop fade stays out
                // (and above 0: a loop at level 0 with target 0 is dropped by `fade_voices`).
                let level = level.max(MIN_LEVEL);
                let voice = Voice { sound: cue.sound.clone(), bus: cue.bus, base: file.volume * cue.volume, level, target: level, looping: true };
                let at = Vec3::from_array(at);
                let distance_gain = cue.attenuation.zip(ear).map(|(att, ear)| att.gain(at.distance(ear)));
                let start = Start { spatial: cue.attenuation, pitch: Some(cue.pitch), distance_gain };
                let e = spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &start)
                    .insert((SlideVoice(body), CueInstance(cue.cue), Transform::from_translation(at)))
                    .id();
                playing.insert(body, e);
            }
            SlideChange::Update { body, level, at } => {
                let Some((mut voice, mut t)) = playing.get(&body).and_then(|e| slides.get_mut(*e).ok()) else { continue };
                (voice.level, voice.target) = (level.max(MIN_LEVEL), level.max(MIN_LEVEL));
                t.translation = Vec3::from_array(at);
            }
            SlideChange::Stop { body } => {
                if let Some(e) = playing.remove(&body) {
                    if let Ok((voice, _)) = slides.get(e) {
                        events.write(AudioEvent::LoopStop { sound: voice.sound.clone() });
                    }
                    commands.entity(e).try_despawn();
                }
            }
        }
    }
}
