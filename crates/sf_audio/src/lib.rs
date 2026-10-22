//! Bevy-free audio core (audio.md): buses, fades, attenuation, sound groups (cues), the music
//! player, footstep mapping, physics impacts and slides, ambience layers and one-shot draws. Playing is `sf_audio_bevy`'s;
//! the logic lives here so it is tested without a device.

use std::collections::HashMap;

use sf_records::RecordId;

mod ambience;
mod attenuation;
mod footsteps;
mod group;
mod impacts;
mod music;
mod rng;

pub use ambience::{AmbienceLayers, LoopChange, Space, reconcile_loops};
pub use attenuation::{Attenuation, Rolloff};
pub use footsteps::{Foot, FootstepMap, StrideTimer};
pub use group::{Cue, CueBank, SoundGroup, Variant};
pub use impacts::{ContactSample, ImpactAudio, ImpactCues, ImpactFrame, ImpactMap, ImpactParams, ImpactPlay, SlideChange};
pub use music::{MUSIC_FADE_S, MusicCommand, MusicPlayer, MusicVoiceId};
pub use rng::Rng;

/// Mixer buses (audio.md "Model"). `Master` scales all the others.
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
    const fn index(self) -> usize {
        self as usize
    }
}

/// Linear bus volumes and mutes, and the listener's outdoor exposure.
#[derive(Clone, Debug, PartialEq)]
pub struct Mixer {
    volume: [f32; Bus::COUNT],
    muted: [bool; Bus::COUNT],
    /// 1 under the open sky, 0 indoors: scales the outdoor buses (the interior
    /// snapshot, audio.md "Snapshots"). The host eases it.
    pub outdoor: f32,
}

impl Default for Mixer {
    fn default() -> Self {
        Self { volume: [1.0; Bus::COUNT], muted: [false; Bus::COUNT], outdoor: 1.0 }
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
    /// Buses heard only outdoors: the weather (rain, wind, thunder). Region ambience is drawn
    /// only outdoors instead (`sf_audio_bevy`), so the Ambience bus stays free for interiors.
    pub const fn is_outdoor(bus: Bus) -> bool {
        matches!(bus, Bus::Weather)
    }
    /// A bus's effective gain: its volume times master's (times `outdoor` for an outdoor bus);
    /// 0 when it or master is muted.
    pub fn bus_gain(&self, bus: Bus) -> f32 {
        let one = |b: Bus| if self.muted[b.index()] { 0.0 } else { self.volume[b.index()] };
        let exposure = if Self::is_outdoor(bus) { self.outdoor.clamp(0.0, 1.0) } else { 1.0 };
        if bus == Bus::Master { one(bus) } else { one(bus) * one(Bus::Master) * exposure }
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
