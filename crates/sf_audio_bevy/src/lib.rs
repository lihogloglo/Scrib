//! Thin Bevy plugin for `sf_audio` (audio.md): `WeatherSound` loops with fades, thunder
//! one-shots and the frame's region one-shot ambience, played through `bevy_audio` at the
//! mixer's bus gains. Every start and stop is an [`AudioEvent`] message and an `info!` line
//! (agents cannot hear; the log is the headless check).

use std::collections::BTreeMap;

use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetServer;
use bevy_audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings, Volume};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use bevy_time::{Real, Time};
use sf_audio::{Bus, FADE_S, Mixer, OUTDOOR_FADE_S, OneShotScheduler, SoundBank, fade_toward};
use sf_records::RecordId;
use sf_weather::ClimateIx;
use sf_weather_bevy::{CurrentWeather, WeatherRecords, WeatherSound};

/// The bus mixer (change a bus volume here; voices follow within a frame).
#[derive(Resource, Default, Clone, Debug)]
pub struct AudioMixer(pub Mixer);

/// Whether the listener is under the open sky (the host sets it from the active worldspace).
/// Indoors the mixer's outdoor exposure eases to 0: weather loops and thunder go silent and no
/// region ambience is drawn (audio.md "Snapshots").
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListenerOutdoors(pub bool);

impl Default for ListenerOutdoors {
    fn default() -> Self {
        Self(true)
    }
}

/// Sound id -> file, from the content adapter.
#[derive(Resource, Default, Clone, Debug)]
pub struct AudioBank(pub SoundBank);

/// What started or stopped (for logs and tests).
#[derive(Message, Clone, Debug, PartialEq)]
pub enum AudioEvent {
    LoopStart { sound: RecordId, target: f32 },
    LoopStop { sound: RecordId },
    OneShot { sound: RecordId, volume: f32, bus: Bus },
}

/// A playing sound. Loops fade `level` toward `target`; one-shots play once at `target`.
#[derive(Component, Clone, Debug)]
pub struct Voice {
    pub sound: RecordId,
    pub bus: Bus,
    /// The sound's own volume (bank).
    pub base: f32,
    pub level: f32,
    pub target: f32,
    pub looping: bool,
}

/// Adds the audio systems. `playback: false` runs the voices and events without touching the
/// asset server or an output device (tests, headless logs).
pub struct SfAudioPlugin {
    pub bank: SoundBank,
    pub playback: bool,
}

#[derive(Resource)]
struct Playback(bool);

impl Plugin for SfAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioMixer>()
            .init_resource::<ListenerOutdoors>()
            .insert_resource(AudioBank(self.bank.clone()))
            .insert_resource(Playback(self.playback))
            .add_message::<AudioEvent>()
            .add_message::<WeatherSound>()
            .add_systems(Update, (ease_outdoor, route_weather_sounds, schedule_region_one_shots, fade_voices, log_events).chain());
    }
}

fn spawn_voice(
    commands: &mut Commands,
    server: Option<&AssetServer>,
    path: &str,
    voice: Voice,
    mixer: &Mixer,
) {
    let gain = mixer.gain(voice.bus, voice.level, voice.base);
    let mut e = commands.spawn(voice.clone());
    if let Some(server) = server {
        let settings = if voice.looping { PlaybackSettings::LOOP } else { PlaybackSettings::DESPAWN };
        e.insert((AudioPlayer::<bevy_audio::AudioSource>::new(server.load(path.to_owned())), settings.with_volume(Volume::Linear(gain))));
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn route_weather_sounds(
    mut sounds: MessageReader<WeatherSound>,
    mut events: MessageWriter<AudioEvent>,
    mut voices: Query<&mut Voice>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut commands: Commands,
    mut missing: Local<Vec<RecordId>>,
) {
    let server = if playback.0 { server.as_deref() } else { None };
    for s in sounds.read() {
        let (id, volume, looping) = match s {
            WeatherSound::Loop { sound, volume, .. } => (sound, *volume, true),
            WeatherSound::OneShot { sound, volume, .. } => (sound, *volume, false),
        };
        if looping {
            if let Some(mut v) = voices.iter_mut().find(|v| v.looping && v.sound == *id) {
                if volume <= 0.0 && v.target > 0.0 {
                    events.write(AudioEvent::LoopStop { sound: id.clone() });
                }
                v.target = volume.max(0.0);
                continue;
            }
            if volume <= 0.0 {
                continue;
            }
        }
        let Some(file) = bank.0.get(id) else {
            if !missing.contains(id) {
                warn!("audio: no sound file for {id:?}");
                missing.push(id.clone());
            }
            continue;
        };
        let voice = Voice { sound: id.clone(), bus: Bus::Weather, base: file.volume, level: if looping { 0.0 } else { volume }, target: volume, looping };
        if looping {
            events.write(AudioEvent::LoopStart { sound: id.clone(), target: volume });
            spawn_voice(&mut commands, server, &file.path, voice, &mixer.0);
        } else {
            events.write(AudioEvent::OneShot { sound: id.clone(), volume, bus: Bus::Weather });
            // A one-shot's volume is its level (no fade); base stays the sound's own.
            spawn_voice(&mut commands, server, &file.path, voice, &mixer.0);
        }
    }
}

/// The frame's region one-shot sets: each on set has a scheduler; draws play on the Ambience bus.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn schedule_region_one_shots(
    current: Res<CurrentWeather>,
    records: Option<Res<WeatherRecords>>,
    time: Res<Time<Real>>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
    mut schedulers: Local<BTreeMap<(ClimateIx, usize), OneShotScheduler>>,
) {
    let Some(records) = records else { return };
    if !current.valid || mixer.0.outdoor < 0.5 {
        return;
    }
    let server = if playback.0 { server.as_deref() } else { None };
    let on: Vec<_> = current.frame.one_shots.iter().filter(|o| o.volume > 0.0).collect();
    schedulers.retain(|k, _| on.iter().any(|o| (o.climate, o.set) == *k));
    for o in on {
        let Some(set) = records.0.climate_record(o.climate).one_shots.get(o.set) else { continue };
        let seed = u64::from(o.climate.0) << 16 | o.set as u64;
        let sched = schedulers.entry((o.climate, o.set)).or_insert_with(|| OneShotScheduler::new(seed));
        let chances: Vec<f32> = set.sounds.iter().map(|(_, c)| *c).collect();
        let Some(i) = sched.tick(time.delta_secs(), set.interval_s, &chances) else { continue };
        let id = &set.sounds[i].0;
        let Some(file) = bank.0.get(id) else { continue };
        events.write(AudioEvent::OneShot { sound: id.clone(), volume: o.volume, bus: Bus::Ambience });
        let voice = Voice { sound: id.clone(), bus: Bus::Ambience, base: file.volume, level: o.volume, target: o.volume, looping: false };
        spawn_voice(&mut commands, server, &file.path, voice, &mixer.0);
    }
}

/// Eases the mixer's outdoor exposure toward the listener's space.
#[allow(clippy::needless_pass_by_value)]
fn ease_outdoor(time: Res<Time<Real>>, outdoors: Res<ListenerOutdoors>, mut mixer: ResMut<AudioMixer>) {
    if outdoors.is_changed() && !outdoors.is_added() {
        info!("audio: listener {}", if outdoors.0 { "outdoors" } else { "indoors (weather bus out)" });
    }
    let target = if outdoors.0 { 1.0 } else { 0.0 };
    if (mixer.0.outdoor - target).abs() > 0.0 {
        let next = fade_toward(mixer.0.outdoor, target, time.delta_secs(), OUTDOOR_FADE_S);
        mixer.0.outdoor = next;
    }
}

/// Eases loop levels, applies bus gains to the sinks, and drops faded-out loops.
#[allow(clippy::needless_pass_by_value)]
fn fade_voices(
    time: Res<Time<Real>>,
    mixer: Res<AudioMixer>,
    mut voices: Query<(Entity, &mut Voice, Option<&mut AudioSink>)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (e, mut v, sink) in &mut voices {
        if v.looping {
            let level = fade_toward(v.level, v.target, dt, FADE_S);
            if (level - v.level).abs() > 0.0 {
                v.level = level;
            }
            if v.level <= 0.0 && v.target <= 0.0 {
                commands.entity(e).try_despawn();
                continue;
            }
        }
        if let Some(mut sink) = sink {
            sink.set_volume(Volume::Linear(mixer.0.gain(v.bus, v.level, v.base)));
        }
    }
}

fn log_events(mut events: MessageReader<AudioEvent>) {
    for e in events.read() {
        match e {
            AudioEvent::LoopStart { sound, target } => info!("audio: loop start {} (target {target:.2})", sound.as_str()),
            AudioEvent::LoopStop { sound } => info!("audio: loop stop {}", sound.as_str()),
            AudioEvent::OneShot { sound, volume, bus } => info!("audio: one-shot {} (volume {volume:.2}, {bus:?})", sound.as_str()),
        }
    }
}

#[cfg(test)]
mod tests;
