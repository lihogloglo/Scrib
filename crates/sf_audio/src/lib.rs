//! Bevy-free audio core (audio.md): buses, fades, attenuation, sound groups (cues), the music
//! player, footstep mapping, ambience layers, one-shot draws and the player's volume mixer
//! (slider curve, mutes, ducking). Playing is `sf_audio_bevy`'s;
//! the logic lives here so it is tested without a device.

use std::collections::HashMap;

use sf_records::RecordId;

mod ambience;
mod attenuation;
mod footsteps;
mod group;
mod music;
mod rng;
mod volume;

pub use ambience::{AmbienceLayers, LoopChange, Space, reconcile_loops};
pub use attenuation::{Attenuation, Rolloff};
pub use footsteps::{Foot, FootstepMap, StrideTimer};
pub use group::{Cue, CueBank, SoundGroup, Variant};
pub use music::{MUSIC_FADE_S, MusicCommand, MusicPlayer, MusicVoiceId};
pub use rng::Rng;
pub use volume::{
    BusVolume, DuckRule, DuckTrigger, Ducker, VOLUME_RANGE_DB, VOLUME_TAPER, VolumeSettings, db_to_gain, gain_to_db,
    gain_to_slider, slider_to_db, slider_to_gain,
};

/// Mixer buses (audio.md "Model"), a tree: `Master` scales all the others, `Ambience` also
/// scales `Weather` (one "ambience" slider for beds, region sounds and weather).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bus {
    Master,
    Ambience,
    Weather,
    Sfx,
    Music,
    Ui,
    /// Speech (dialogue, greetings, combat barks).
    Voice,
}

impl Bus {
    pub const COUNT: usize = 7;
    pub const ALL: [Bus; Self::COUNT] =
        [Bus::Master, Bus::Ambience, Bus::Weather, Bus::Sfx, Bus::Music, Bus::Ui, Bus::Voice];
    /// The buses a settings menu shows (audio.md "Volume mixer"); `Weather` follows `Ambience`.
    pub const USER: [Bus; 6] = [Bus::Master, Bus::Music, Bus::Sfx, Bus::Voice, Bus::Ambience, Bus::Ui];
    const fn index(self) -> usize {
        self as usize
    }
    /// The bus this one feeds (`None` for `Master`).
    pub const fn parent(self) -> Option<Bus> {
        match self {
            Bus::Master => None,
            Bus::Weather => Some(Bus::Ambience),
            _ => Some(Bus::Master),
        }
    }
    /// Stable lowercase name (settings keys, logs).
    pub const fn name(self) -> &'static str {
        match self {
            Bus::Master => "master",
            Bus::Ambience => "ambience",
            Bus::Weather => "weather",
            Bus::Sfx => "sfx",
            Bus::Music => "music",
            Bus::Ui => "ui",
            Bus::Voice => "voice",
        }
    }
    pub fn from_name(name: &str) -> Option<Bus> {
        Self::ALL.into_iter().find(|b| b.name() == name)
    }
}

/// Linear bus volumes, mutes and duck gains, and the listener's outdoor exposure.
#[derive(Clone, Debug, PartialEq)]
pub struct Mixer {
    volume: [f32; Bus::COUNT],
    muted: [bool; Bus::COUNT],
    /// Ducking attenuation per bus (linear, 1 = none), written by `Ducker::apply`.
    duck: [f32; Bus::COUNT],
    /// 1 under the open sky, 0 indoors: scales the outdoor buses (the interior
    /// snapshot, audio.md "Snapshots"). The host eases it.
    pub outdoor: f32,
}

impl Default for Mixer {
    fn default() -> Self {
        Self { volume: [1.0; Bus::COUNT], muted: [false; Bus::COUNT], duck: [1.0; Bus::COUNT], outdoor: 1.0 }
    }
}

impl Mixer {
    pub fn set_volume(&mut self, bus: Bus, volume: f32) {
        self.volume[bus.index()] = volume.max(0.0);
    }
    pub fn volume(&self, bus: Bus) -> f32 {
        self.volume[bus.index()]
    }
    pub fn set_muted(&mut self, bus: Bus, muted: bool) {
        self.muted[bus.index()] = muted;
    }
    pub fn is_muted(&self, bus: Bus) -> bool {
        self.muted[bus.index()]
    }
    /// Sets a bus's ducking gain (0..1).
    pub fn set_duck(&mut self, bus: Bus, gain: f32) {
        self.duck[bus.index()] = gain.clamp(0.0, 1.0);
    }
    pub fn duck(&self, bus: Bus) -> f32 {
        self.duck[bus.index()]
    }
    /// Buses heard only outdoors: the weather (rain, wind, thunder). Region ambience is drawn
    /// only outdoors instead (`sf_audio_bevy`), so the Ambience bus stays free for interiors.
    pub const fn is_outdoor(bus: Bus) -> bool {
        matches!(bus, Bus::Weather)
    }
    /// A bus's effective gain: volume x duck of it and every bus above it (`Bus::parent`), times
    /// `outdoor` for an outdoor bus; 0 when it or a bus above is muted.
    pub fn bus_gain(&self, bus: Bus) -> f32 {
        let exposure = if Self::is_outdoor(bus) { self.outdoor.clamp(0.0, 1.0) } else { 1.0 };
        let mut gain = exposure;
        let mut at = Some(bus);
        while let Some(b) = at {
            if self.muted[b.index()] {
                return 0.0;
            }
            gain *= self.volume[b.index()] * self.duck[b.index()];
            at = b.parent();
        }
        gain
    }
    /// A voice's output gain: `level` (its fade, 0..1) x `base` (the sound's own volume) x bus gain.
    pub fn gain(&self, bus: Bus, level: f32, base: f32) -> f32 {
        level.max(0.0) * base.max(0.0) * self.bus_gain(bus)
    }
}

/// Seconds a loop takes to fade from silence to its full volume (and back).
pub const FADE_S: f32 = 2.0;
/// Seconds the outdoor exposure takes to swing fully (through a door, under the fade).
pub const OUTDOOR_FADE_S: f32 = 0.5;

/// Moves `level` toward `target` by at most `dt / fade_s` per call (linear, no overshoot).
/// Two loops fading opposite ways are a crossfade.
pub fn fade_toward(level: f32, target: f32, dt: f32, fade_s: f32) -> f32 {
    if fade_s <= 0.0 {
        return target;
    }
    let step = dt.max(0.0) / fade_s;
    if target >= level { (level + step).min(target) } else { (level - step).max(target) }
}

/// Equal-power fade law: maps a linear fade position (0..1) to a gain whose square sums to 1
/// across a crossfade (`sin(x * pi/2)`), so tonal material (music) does not dip mid-fade.
pub fn equal_power(level: f32) -> f32 {
    (level.clamp(0.0, 1.0) * std::f32::consts::FRAC_PI_2).sin()
}

/// A sound file a bank entry resolves to: asset path in the VFS and the sound's own volume (0..1).
#[derive(Clone, Debug, PartialEq)]
pub struct SoundFile {
    pub path: String,
    pub volume: f32,
}

/// Sound id -> file, filled by the content adapter (the framework never names a format's records).
#[derive(Clone, Debug, Default)]
pub struct SoundBank(pub HashMap<RecordId, SoundFile>);

impl SoundBank {
    pub fn get(&self, id: &RecordId) -> Option<&SoundFile> {
        self.0.get(id)
    }
}

/// Draws one sound every `interval` seconds (uniform in the range) from weighted choices
/// (a scatterer: audio.md "One-shots"). Deterministic from its seed.
#[derive(Clone, Debug)]
pub struct OneShotScheduler {
    rng: Rng,
    wait_s: Option<f32>,
}

impl OneShotScheduler {
    pub fn new(seed: u64) -> Self {
        Self { rng: Rng::new(seed), wait_s: None }
    }

    /// Advances by `dt` seconds; returns the index of the drawn choice when a draw is due and
    /// lands on one. `choices` hold chances per draw; sums above 1 are normalized (no silence),
    /// sums below 1 leave the rest as silence.
    pub fn tick(&mut self, dt: f32, interval_s: [f32; 2], choices: &[f32]) -> Option<usize> {
        let wait = self.wait_s.unwrap_or_else(|| self.rng.range(interval_s[0].max(0.0), interval_s[1].max(0.0)));
        let wait = wait - dt;
        if wait > 0.0 {
            self.wait_s = Some(wait);
            return None;
        }
        self.wait_s = None;
        let total: f32 = choices.iter().map(|c| c.max(0.0)).sum();
        let norm = total.max(1.0);
        let mut r = self.rng.unit();
        for (i, c) in choices.iter().enumerate() {
            let p = c.max(0.0) / norm;
            if r < p {
                return Some(i);
            }
            r -= p;
        }
        None
    }
}

#[cfg(test)]
mod tests;
