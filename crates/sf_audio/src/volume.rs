//! The player's volume mixer (audio.md "Volume mixer"): per-bus slider levels and mutes, the
//! perceptual slider-to-dB curve, and ducking (one bus or a host hold attenuating others).

use std::collections::BTreeSet;

use crate::{Bus, Mixer};

/// Decibels a full slider travel spans: the top is 0 dB, the bottom of the exponential part is
/// near -60 dB. Linear in dB is how loudness is heard, so equal slider steps sound like equal
/// steps; 60 dB is the usual range for a volume control (an amplitude-linear slider crams the
/// audible range into its last few percent).
pub const VOLUME_RANGE_DB: f32 = 60.0;
/// Below this slider position the curve is linear in amplitude down to true silence at 0
/// (a pure dB curve never reaches 0).
pub const VOLUME_TAPER: f32 = 0.1;

/// Linear gain of `db` decibels (-inf -> 0).
pub fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

/// Decibels of a linear gain (0 -> -inf).
pub fn gain_to_db(gain: f32) -> f32 {
    20.0 * gain.max(0.0).log10()
}

/// A volume slider (0..1) as linear gain: 1 at the top, [`VOLUME_RANGE_DB`] below it linearly in
/// dB down to [`VOLUME_TAPER`], then a straight line to 0. Continuous and strictly increasing.
pub fn slider_to_gain(slider: f32) -> f32 {
    let s = if slider.is_nan() { 0.0 } else { slider.clamp(0.0, 1.0) };
    if s >= VOLUME_TAPER {
        db_to_gain(-VOLUME_RANGE_DB * (1.0 - s))
    } else {
        db_to_gain(-VOLUME_RANGE_DB * (1.0 - VOLUME_TAPER)) * s / VOLUME_TAPER
    }
}

/// [`slider_to_gain`] in decibels (-inf at 0).
pub fn slider_to_db(slider: f32) -> f32 {
    gain_to_db(slider_to_gain(slider))
}

/// The slider position giving `gain` (the inverse of [`slider_to_gain`], clamped to 0..1).
pub fn gain_to_slider(gain: f32) -> f32 {
    let taper = db_to_gain(-VOLUME_RANGE_DB * (1.0 - VOLUME_TAPER));
    let g = gain.clamp(0.0, 1.0);
    if g >= taper { 1.0 + gain_to_db(g) / VOLUME_RANGE_DB } else { g / taper * VOLUME_TAPER }
}

/// One bus's player setting: slider position (0..1) and mute (kept apart, so unmuting restores
/// the level).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BusVolume {
    pub level: f32,
    pub muted: bool,
}

impl Default for BusVolume {
    fn default() -> Self {
        Self { level: 1.0, muted: false }
    }
}

/// The player's volume settings for every bus ([`Bus::USER`] are the ones a settings menu shows;
/// the rest stay at full).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeSettings([BusVolume; Bus::COUNT]);

impl VolumeSettings {
    pub fn get(&self, bus: Bus) -> BusVolume {
        self.0[bus as usize]
    }
    /// Sets a slider level (clamped to 0..1; NaN is 0).
    pub fn set_level(&mut self, bus: Bus, level: f32) {
        self.0[bus as usize].level = if level.is_nan() { 0.0 } else { level.clamp(0.0, 1.0) };
    }
    pub fn set_muted(&mut self, bus: Bus, muted: bool) {
        self.0[bus as usize].muted = muted;
    }
    /// Writes every bus's curved gain and mute into the mixer.
    pub fn apply(&self, mixer: &mut Mixer) {
        for bus in Bus::ALL {
            let v = self.get(bus);
            mixer.set_volume(bus, slider_to_gain(v.level));
            mixer.set_muted(bus, v.muted);
        }
    }
}

/// What makes a [`DuckRule`] duck.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DuckTrigger {
    /// Any voice playing on the bus (speech ducking music).
    Bus(Bus),
    /// A named hold the host switches ([`Ducker::set_hold`]): a pause menu, a cutscene.
    Hold(String),
}

/// While the trigger is on, `target` is attenuated by `depth_db`, reached over `attack_s` and
/// released over `release_s` (linear in dB, Wwise auto-ducking's duck volume, fade out and fade in).
#[derive(Clone, Debug, PartialEq)]
pub struct DuckRule {
    pub trigger: DuckTrigger,
    pub target: Bus,
    /// Attenuation in dB at full duck (positive: 8 = -8 dB).
    pub depth_db: f32,
    pub attack_s: f32,
    pub release_s: f32,
}

/// Ducking envelopes per rule. Several rules on one bus do not stack: the deepest wins (stacked
/// ducks would bury the bed; Wwise caps them with a maximum ducking volume). A duck on a bus also
/// ducks its children (`Bus::parent`), since it is a bus gain.
#[derive(Clone, Debug, Default)]
pub struct Ducker {
    rules: Vec<DuckRule>,
    /// Envelope per rule, 0 (off) .. 1 (full depth).
    env: Vec<f32>,
    holds: BTreeSet<String>,
}

impl Ducker {
    pub fn new(rules: impl IntoIterator<Item = DuckRule>) -> Self {
        let mut d = Self::default();
        for r in rules {
            d.add_rule(r);
        }
        d
    }
    pub fn add_rule(&mut self, rule: DuckRule) {
        self.rules.push(rule);
        self.env.push(0.0);
    }
    pub fn rules(&self) -> &[DuckRule] {
        &self.rules
    }
    /// Turns a named hold on or off (rules with `DuckTrigger::Hold(name)`).
    pub fn set_hold(&mut self, name: &str, on: bool) {
        if on {
            self.holds.insert(name.to_owned());
        } else {
            self.holds.remove(name);
        }
    }
    pub fn is_held(&self, name: &str) -> bool {
        self.holds.contains(name)
    }
    /// Advances every envelope by `dt` seconds; `playing(bus)` says whether a voice sounds on it.
    pub fn tick(&mut self, dt: f32, playing: impl Fn(Bus) -> bool) {
        for (rule, env) in self.rules.iter().zip(&mut self.env) {
            let on = match &rule.trigger {
                DuckTrigger::Bus(bus) => playing(*bus),
                DuckTrigger::Hold(name) => self.holds.contains(name),
            };
            let (target, time) = if on { (1.0, rule.attack_s) } else { (0.0, rule.release_s) };
            *env = crate::fade_toward(*env, target, dt, time);
        }
    }
    /// The bus's own attenuation now, in dB (positive; 0 = not ducked).
    pub fn duck_db(&self, bus: Bus) -> f32 {
        self.rules
            .iter()
            .zip(&self.env)
            .filter(|(r, _)| r.target == bus)
            .map(|(r, e)| r.depth_db.max(0.0) * e)
            .fold(0.0, f32::max)
    }
    /// Writes every bus's duck gain into the mixer.
    pub fn apply(&self, mixer: &mut Mixer) {
        for bus in Bus::ALL {
            mixer.set_duck(bus, db_to_gain(-self.duck_db(bus)));
        }
    }
}
