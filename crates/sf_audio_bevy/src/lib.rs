//! Thin Bevy plugin for `sf_audio` (audio.md): bus loops and one-shots ([`BusSound`]: weather
//! beds, thunder), random one-shot sets ([`OneShotSets`]: region ambience), cue one-shots (2D or
//! positional), physics impacts and slide loops ([`ContactSound`]; the `physics` feature's
//! [`physics::PhysicsAudioPlugin`] feeds them), looping emitters, ambience beds and the music player, all
//! played through `bevy_audio` at the mixer's bus gains. The weather glue is the `weather`
//! feature's [`weather::WeatherAudioPlugin`], so this crate builds without the render stack. Positional voices take their distance
//! curve from `sf_audio::Attenuation`; Bevy's `SpatialListener` (the host puts it on the camera)
//! only pans them. Every start and stop is an [`AudioEvent`] message and an `info!` line (agents
//! cannot hear; the log is the headless check).

use std::collections::BTreeMap;

use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetServer;
use bevy_audio::{
    AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings, SpatialAudioSink, SpatialListener, SpatialScale,
    Volume,
};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use bevy_math::Vec3;
use bevy_time::{Real, Time};
use bevy_transform::components::{GlobalTransform, Transform};
use sf_audio::{
    AmbienceLayers, Attenuation, Bus, Cue, CueBank, FADE_S, LoopChange, Mixer, MusicCommand, MusicPlayer,
    MusicVoiceId, OUTDOOR_FADE_S, OneShotScheduler, SoundBank, equal_power, fade_toward, reconcile_loops,
};
use sf_records::RecordId;

mod contacts;
#[cfg(feature = "physics")]
pub mod physics;
#[cfg(feature = "weather")]
pub mod weather;

pub use contacts::{ContactAudio, ContactSound, PlaySoundScaled, SlideVoice};

/// One-shot voices allowed at once; plays over the budget are dropped (audio.md "Voice budget").
pub const MAX_ONE_SHOTS: usize = 64;

/// The bus mixer (change a bus volume here; voices follow within a frame).
#[derive(Resource, Default, Clone, Debug)]
pub struct AudioMixer(pub Mixer);

/// Whether the listener is under the open sky (the host sets it from the active worldspace).
/// Indoors the mixer's outdoor exposure eases to 0: weather loops and thunder go silent, no
/// region ambience is drawn and outdoor ambience beds give way to indoor ones (audio.md "Snapshots").
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

/// Cue id -> sound group (variants, bus, attenuation), from the content adapter.
#[derive(Resource, Default, Clone, Debug)]
pub struct AudioCues(pub CueBank);

/// Ambience bed layers; the plugin keeps one looping voice per wanted sound.
#[derive(Resource, Default, Clone, Debug)]
pub struct AmbienceLoops(pub AmbienceLayers);

/// The music player: the host sets playlists and calls `request(Some("mood"))`.
#[derive(Resource, Clone, Debug)]
pub struct Music(pub MusicPlayer);

impl Default for Music {
    fn default() -> Self {
        Self(MusicPlayer::new(0x5EED))
    }
}

/// Where a cue plays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SoundAt {
    /// Non-positional (UI, the player's own sounds); the cue's attenuation is ignored.
    Listener,
    /// A fixed world point.
    Point(Vec3),
    /// Follows an entity (the voice is its child).
    Entity(Entity),
    /// Follows an entity at an offset in its local space (a door beside the player that
    /// travels with them through a load).
    Offset(Entity, Vec3),
}

/// A non-positional sound on a bus from a system that keeps its own state (weather beds and
/// thunder). A `Loop` sets the looping sound's target level on its bus (0 fades it out; a new
/// one starts silent and fades in); a `OneShot` plays once at `volume`.
#[derive(Message, Clone, Debug, PartialEq)]
pub enum BusSound {
    Loop { sound: RecordId, volume: f32, bus: Bus },
    OneShot { sound: RecordId, volume: f32, bus: Bus },
}

/// A set of random one-shots drawn while on (region ambience: birds, distant wind).
#[derive(Clone, Debug, PartialEq)]
pub struct OneShotSet {
    /// Stable key (keeps the set's scheduler across frames; its seed).
    pub key: u64,
    /// Seconds between draws, uniform in this range.
    pub interval_s: [f32; 2],
    /// Sound and chance (0..1) per draw.
    pub sounds: Vec<(RecordId, f32)>,
    pub volume: f32,
}

/// The one-shot sets on this frame; drawn on the Ambience bus while the listener is outdoors.
#[derive(Resource, Default, Clone, Debug)]
pub struct OneShotSets(pub Vec<OneShotSet>);

/// The plugin's systems (feeders such as the weather bridge run before).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioSystems;

/// Play a cue once (gameplay, UI and animation events write these).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct PlaySound {
    pub cue: RecordId,
    pub at: SoundAt,
}

/// A looping positional sound on its entity (a fire, a waterfall). The plugin adds a child voice.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct LoopEmitter {
    pub cue: RecordId,
}

/// What started or stopped (for logs and tests).
#[derive(Message, Clone, Debug, PartialEq)]
pub enum AudioEvent {
    LoopStart { sound: RecordId, target: f32 },
    LoopStop { sound: RecordId },
    OneShot { sound: RecordId, volume: f32, bus: Bus },
    Cue { cue: RecordId, sound: RecordId, bus: Bus },
    MusicStart { path: String },
}

/// A playing sound. Loops fade `level` toward `target`; one-shots play once at `target`.
#[derive(Component, Clone, Debug)]
pub struct Voice {
    pub sound: RecordId,
    pub bus: Bus,
    /// The sound's own volume (bank, times the cue's random volume).
    pub base: f32,
    pub level: f32,
    pub target: f32,
    pub looping: bool,
}

/// A positional voice's distance curve (gain from the listener distance each frame).
#[derive(Component, Clone, Copy, Debug)]
pub struct SpatialVoice(pub Attenuation);

/// The cue a voice was started for (instance caps).
#[derive(Component, Clone, Debug)]
pub struct CueInstance(pub RecordId);

/// Marks an ambience bed voice (owned by `AmbienceLoops`).
#[derive(Component)]
pub struct AmbienceBed;

/// A music track voice.
#[derive(Component, Clone, Copy, Debug)]
pub struct MusicVoice(pub MusicVoiceId);

/// Marks an emitter whose child voice has been spawned.
#[derive(Component)]
struct EmitterStarted;

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
            .init_resource::<AudioCues>()
            .init_resource::<AmbienceLoops>()
            .init_resource::<Music>()
            .init_resource::<OneShotSets>()
            .init_resource::<ContactAudio>()
            .insert_resource(AudioBank(self.bank.clone()))
            .insert_resource(Playback(self.playback))
            .add_message::<AudioEvent>()
            .add_message::<BusSound>()
            .add_message::<PlaySound>()
            .add_message::<PlaySoundScaled>()
            .add_message::<ContactSound>()
            .add_systems(
                Update,
                (
                    ease_outdoor,
                    route_bus_sounds,
                    schedule_one_shot_sets,
                    reconcile_ambience,
                    contacts::play_contacts,
                    play_cues,
                    start_emitters,
                    drive_music,
                    fade_voices,
                    log_events,
                )
                    .chain()
                    .in_set(AudioSystems),
            );
    }
}

/// How to start a voice beyond its `Voice` data.
#[derive(Default)]
struct Start {
    spatial: Option<Attenuation>,
    pitch: Option<f32>,
    /// Gain from distance at start (1 when non-positional).
    distance_gain: Option<f32>,
}

fn spawn_voice<'a>(
    commands: &'a mut Commands,
    server: Option<&AssetServer>,
    path: &str,
    voice: Voice,
    mixer: &Mixer,
    start: &Start,
) -> EntityCommands<'a> {
    let gain = mixer.gain(voice.bus, voice.level, voice.base) * start.distance_gain.unwrap_or(1.0);
    let looping = voice.looping;
    let mut e = commands.spawn(voice);
    if let Some(server) = server {
        let mut settings = if looping { PlaybackSettings::LOOP } else { PlaybackSettings::DESPAWN };
        settings = settings.with_volume(Volume::Linear(gain));
        if let Some(att) = start.spatial {
            // Scale the world so the whole audible range sits inside 1 unit, where rodio's own
            // inverse-square law is flat: it only pans and our curve sets the distance gain.
            settings = settings.with_spatial(true).with_spatial_scale(SpatialScale::new(1.0 / att.max_m.max(1.0)));
        }
        if let Some(pitch) = start.pitch {
            settings = settings.with_speed(pitch);
        }
        e.insert((AudioPlayer::<bevy_audio::AudioSource>::new(server.load(path.to_owned())), settings));
    }
    if let Some(att) = start.spatial {
        e.insert(SpatialVoice(att));
    }
    e
}

#[allow(clippy::too_many_arguments)]
fn route_bus_sounds(
    mut sounds: MessageReader<BusSound>,
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
        let (id, volume, bus, looping) = match s {
            BusSound::Loop { sound, volume, bus } => (sound, *volume, *bus, true),
            BusSound::OneShot { sound, volume, bus } => (sound, *volume, *bus, false),
        };
        if looping {
            if let Some(mut v) = voices.iter_mut().find(|v| v.looping && v.bus == bus && v.sound == *id) {
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
        // A one-shot's volume is its level (no fade); base stays the sound's own.
        let voice = Voice { sound: id.clone(), bus, base: file.volume, level: if looping { 0.0 } else { volume }, target: volume, looping };
        events.write(if looping {
            AudioEvent::LoopStart { sound: id.clone(), target: volume }
        } else {
            AudioEvent::OneShot { sound: id.clone(), volume, bus }
        });
        spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &Start::default());
    }
}

/// The frame's one-shot sets: each on set has a scheduler; draws play on the Ambience bus.
#[allow(clippy::too_many_arguments)]
fn schedule_one_shot_sets(
    sets: Res<OneShotSets>,
    time: Res<Time<Real>>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
    mut schedulers: Local<BTreeMap<u64, OneShotScheduler>>,
) {
    if mixer.0.outdoor < 0.5 {
        return;
    }
    let server = if playback.0 { server.as_deref() } else { None };
    let on: Vec<_> = sets.0.iter().filter(|o| o.volume > 0.0).collect();
    schedulers.retain(|k, _| on.iter().any(|o| o.key == *k));
    for o in on {
        let sched = schedulers.entry(o.key).or_insert_with(|| OneShotScheduler::new(o.key));
        let chances: Vec<f32> = o.sounds.iter().map(|(_, c)| *c).collect();
        let Some(i) = sched.tick(time.delta_secs(), o.interval_s, &chances) else { continue };
        let id = &o.sounds[i].0;
        let Some(file) = bank.0.get(id) else { continue };
        events.write(AudioEvent::OneShot { sound: id.clone(), volume: o.volume, bus: Bus::Ambience });
        let voice = Voice { sound: id.clone(), bus: Bus::Ambience, base: file.volume, level: o.volume, target: o.volume, looping: false };
        spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &Start::default());
    }
}

/// Keeps one looping Ambience voice per wanted bed, weighted by the outdoor exposure.
#[allow(clippy::too_many_arguments)]
fn reconcile_ambience(
    loops: Res<AmbienceLoops>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut beds: Query<&mut Voice, With<AmbienceBed>>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
) {
    let wanted = loops.0.desired(mixer.0.outdoor);
    let playing: Vec<_> = beds.iter().map(|v| (v.sound.clone(), v.target)).collect();
    if wanted.is_empty() && playing.iter().all(|(_, t)| *t <= 0.0) {
        return;
    }
    let server = if playback.0 { server.as_deref() } else { None };
    for change in reconcile_loops(&playing, &wanted) {
        match change {
            LoopChange::Start { sound, target } => {
                let Some(file) = bank.0.get(&sound) else { continue };
                events.write(AudioEvent::LoopStart { sound: sound.clone(), target });
                let voice = Voice { sound, bus: Bus::Ambience, base: file.volume, level: 0.0, target, looping: true };
                spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &Start::default()).insert(AmbienceBed);
            }
            LoopChange::Retarget { sound, target } => {
                beds.iter_mut().filter(|v| v.sound == sound).for_each(|mut v| v.target = target);
            }
            LoopChange::Stop { sound } => {
                events.write(AudioEvent::LoopStop { sound: sound.clone() });
                beds.iter_mut().filter(|v| v.sound == sound).for_each(|mut v| v.target = 0.0);
            }
        }
    }
}

fn listener_pos(listener: &Query<&GlobalTransform, With<SpatialListener>>) -> Option<Vec3> {
    listener.iter().next().map(GlobalTransform::translation)
}

/// Resolves `PlaySound` and `PlaySoundScaled` cues and starts them, culled by distance and capped
/// per cue and overall.
#[allow(clippy::too_many_arguments)]
fn play_cues(
    mut plays: MessageReader<PlaySound>,
    mut scaled: MessageReader<PlaySoundScaled>,
    mut cues: ResMut<AudioCues>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    places: Query<&GlobalTransform>,
    instances: Query<(&CueInstance, &Voice)>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
) {
    let server = if playback.0 { server.as_deref() } else { None };
    let ear = listener_pos(&listener);
    let mut one_shots = instances.iter().filter(|(_, v)| !v.looping).count();
    let mut started: Vec<RecordId> = Vec::new();
    for (play, scale) in plays.read().map(|p| (p, 1.0)).chain(scaled.read().map(|s| (&s.play, s.volume.max(0.0)))) {
        let Some(cue) = cues.0.resolve(&play.cue) else { continue };
        let Some(file) = bank.0.get(&cue.sound) else { continue };
        let at = match play.at {
            SoundAt::Listener => None,
            SoundAt::Point(p) => Some(p),
            SoundAt::Entity(e) => Some(places.get(e).map_or(Vec3::ZERO, GlobalTransform::translation)),
            SoundAt::Offset(e, offset) => Some(places.get(e).map_or(Vec3::ZERO, |g| g.transform_point(offset))),
        };
        let spatial = if at.is_some() { cue.attenuation } else { None };
        let distance_gain = match (spatial, at, ear) {
            (Some(att), Some(p), Some(ear)) => att.gain(p.distance(ear)),
            _ => 1.0,
        };
        let live = instances.iter().filter(|(c, _)| c.0 == cue.cue).count() + started.iter().filter(|c| **c == cue.cue).count();
        if distance_gain <= 0.0 || one_shots >= MAX_ONE_SHOTS || (cue.max_instances > 0 && live >= usize::from(cue.max_instances)) {
            continue;
        }
        one_shots += 1;
        started.push(cue.cue.clone());
        events.write(AudioEvent::Cue { cue: cue.cue.clone(), sound: cue.sound.clone(), bus: cue.bus });
        let Cue { cue: cue_id, sound, bus, volume, pitch, .. } = cue;
        let voice = Voice { sound, bus, base: file.volume * volume * scale, level: 1.0, target: 1.0, looping: false };
        let start = Start { spatial, pitch: Some(pitch), distance_gain: Some(distance_gain) };
        let mut e = spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &start);
        e.insert(CueInstance(cue_id));
        match play.at {
            SoundAt::Listener => {}
            SoundAt::Point(p) => {
                e.insert(Transform::from_translation(p));
            }
            SoundAt::Entity(parent) => {
                e.insert((Transform::IDENTITY, ChildOf(parent)));
            }
            SoundAt::Offset(parent, offset) => {
                e.insert((Transform::from_translation(offset), ChildOf(parent)));
            }
        }
    }
}

/// Gives each new `LoopEmitter` a looping positional child voice (it goes with its entity).
fn start_emitters(
    emitters: Query<(Entity, &LoopEmitter), Without<EmitterStarted>>,
    mut cues: ResMut<AudioCues>,
    bank: Res<AudioBank>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut commands: Commands,
) {
    let server = if playback.0 { server.as_deref() } else { None };
    for (entity, emitter) in &emitters {
        commands.entity(entity).insert(EmitterStarted);
        let Some(cue) = cues.0.resolve(&emitter.cue) else { continue };
        let Some(file) = bank.0.get(&cue.sound) else { continue };
        let voice = Voice { sound: cue.sound.clone(), bus: cue.bus, base: file.volume * cue.volume, level: 1.0, target: 1.0, looping: true };
        let start = Start { spatial: Some(cue.attenuation.unwrap_or_default()), pitch: Some(cue.pitch), distance_gain: Some(0.0) };
        spawn_voice(&mut commands, server, &file.path, voice, &mixer.0, &start)
            .insert((CueInstance(cue.cue), Transform::IDENTITY, ChildOf(entity)));
    }
}

/// Ticks the music player, plays its commands and reports finished tracks.
#[allow(clippy::too_many_arguments)]
fn drive_music(
    time: Res<Time<Real>>,
    mut music: ResMut<Music>,
    mixer: Res<AudioMixer>,
    playback: Res<Playback>,
    server: Option<Res<AssetServer>>,
    mut voices: Query<(Entity, &MusicVoice, Option<&mut AudioSink>)>,
    mut events: MessageWriter<AudioEvent>,
    mut commands: Commands,
) {
    for (_, voice, sink) in &voices {
        if sink.is_some_and(AudioSinkPlayback::empty) {
            music.0.finished(voice.0);
        }
    }
    let server = if playback.0 { server.as_deref() } else { None };
    for cmd in music.0.tick(time.delta_secs()) {
        match cmd {
            MusicCommand::Start { voice, path } => {
                events.write(AudioEvent::MusicStart { path: path.clone() });
                let mut e = commands.spawn(MusicVoice(voice));
                if let Some(server) = server {
                    // Starts silent; the level below sets its gain from the next frame on.
                    let settings = PlaybackSettings::ONCE.with_volume(Volume::Linear(0.0));
                    e.insert((AudioPlayer::<bevy_audio::AudioSource>::new(server.load(path)), settings));
                }
            }
            MusicCommand::Stop { voice } => {
                for (e, v, _) in &voices {
                    if v.0 == voice {
                        commands.entity(e).try_despawn();
                    }
                }
            }
        }
    }
    let levels: BTreeMap<_, _> = music.0.voices().collect();
    for (_, v, sink) in &mut voices {
        if let (Some(mut sink), Some(level)) = (sink, levels.get(&v.0)) {
            sink.set_volume(Volume::Linear(mixer.0.gain(Bus::Music, equal_power(*level), 1.0)));
        }
    }
}

/// Eases the mixer's outdoor exposure toward the listener's space.
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

/// A voice's output gain this frame: mixer x fade x distance curve.
pub fn voice_gain(mixer: &Mixer, voice: &Voice, spatial: Option<(&SpatialVoice, Vec3)>, ear: Option<Vec3>) -> f32 {
    let distance = match (spatial, ear) {
        (Some((s, at)), Some(ear)) => s.0.gain(at.distance(ear)),
        _ => 1.0,
    };
    mixer.gain(voice.bus, voice.level, voice.base) * distance
}

type VoiceItem = (
    Entity,
    &'static mut Voice,
    Option<&'static mut AudioSink>,
    Option<&'static mut SpatialAudioSink>,
    Option<&'static SpatialVoice>,
    Option<&'static GlobalTransform>,
);

/// Eases loop levels, applies bus and distance gains to the sinks, and drops faded-out loops.
fn fade_voices(
    time: Res<Time<Real>>,
    mixer: Res<AudioMixer>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    mut voices: Query<VoiceItem>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    let ear = listener_pos(&listener);
    for (e, mut v, sink, spatial_sink, spatial, at) in &mut voices {
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
        let gain = voice_gain(&mixer.0, &v, spatial.zip(at.map(GlobalTransform::translation)), ear);
        if let Some(mut sink) = sink {
            sink.set_volume(Volume::Linear(gain));
        }
        if let Some(mut sink) = spatial_sink {
            sink.set_volume(Volume::Linear(gain));
        }
    }
}

fn log_events(mut events: MessageReader<AudioEvent>) {
    for e in events.read() {
        match e {
            AudioEvent::LoopStart { sound, target } => info!("audio: loop start {} (target {target:.2})", sound.as_str()),
            AudioEvent::LoopStop { sound } => info!("audio: loop stop {}", sound.as_str()),
            AudioEvent::OneShot { sound, volume, bus } => info!("audio: one-shot {} (volume {volume:.2}, {bus:?})", sound.as_str()),
            AudioEvent::Cue { cue, sound, bus } => info!("audio: cue {} -> {} ({bus:?})", cue.as_str(), sound.as_str()),
            AudioEvent::MusicStart { path } => info!("audio: music {path}"),
        }
    }
}

#[cfg(test)]
mod tests;
