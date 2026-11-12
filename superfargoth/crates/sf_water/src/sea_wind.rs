//! The sea follows the weather's wind (water.md "Weather coupling", W-WIND).
//!
//! The spectrum is a function of the 10 m wind; a weather change must neither re-roll the waves nor
//! pop. The noise of `H0::generate` is keyed by `(seed, cascade, wave vector)` alone, so two sea
//! states of one seed share every phase and differ only in amplitude: the FFT (linear) blends them
//! as `h0 = mix(h0_from, h0_to, w)` (Crest, Atlas and UE Water blend spectra the same way). This
//! module decides *when* a new pair starts and how far `w` is:
//!
//! - the weather's mean wind is low-passed (first order, [`SeaWindSettings::response_s`]): seas
//!   build and settle over minutes, gusts and wander never reach the waves;
//! - a transition `from → to` runs linearly over [`SeaWindSettings::blend_s`]; a new one starts
//!   only once the last has finished and the filtered wind has moved past a step
//!   ([`SeaWindSettings::speed_step`], [`SeaWindSettings::angle_step_deg`]). That is the rate limit
//!   of the spectrum re-inits (one `h0` generation + upload per transition).
//!
//! A host override ([`SeaWind::fixed`]) pins the wind whatever the weather.

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::WindInput;

/// Tuning of the sea's response to the wind (framework defaults).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SeaWindSettings {
    /// Time constant of the wind low-pass, seconds (game clock). Real seas take hours to build; a
    /// game compresses it to under a minute so a weather change reads.
    pub response_s: f32,
    /// Duration of one spectrum transition, seconds; also the shortest time between two re-inits.
    pub blend_s: f32,
    /// Relative speed change that starts a transition.
    pub speed_step: f32,
    /// Absolute speed change that starts one at low wind, m/s.
    pub min_speed_step: f32,
    /// Direction change that starts one, degrees (only above [`Self::calm`]).
    pub angle_step_deg: f32,
    /// Below this speed (m/s) the direction is kept: a calm has no direction.
    pub calm: f32,
}

impl Default for SeaWindSettings {
    fn default() -> Self {
        Self {
            response_s: 30.0,
            blend_s: 6.0,
            speed_step: 0.03,
            min_speed_step: 0.15,
            angle_step_deg: 3.0,
            calm: 0.5,
        }
    }
}

/// One spectrum transition: the sea is `mix(from, to, weight(t))`, `to` being the current wind.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeaTransition {
    pub from: WindInput,
    /// Start, seconds on the water's clock.
    pub start: f64,
    /// Seconds (> 0).
    pub duration: f64,
}

impl SeaTransition {
    /// How far the transition is at time `t`, `[0, 1]`, linear (continuous and monotone).
    #[allow(clippy::cast_possible_truncation)]
    pub fn weight(&self, t: f64) -> f32 {
        ((t - self.start) / self.duration.max(1e-6)).clamp(0.0, 1.0) as f32
    }

    pub fn done(&self, t: f64) -> bool {
        t >= self.start + self.duration
    }
}

/// The 10 m wind a velocity gives the spectrum (`fallback` direction in a calm).
pub fn wind_input(velocity: Vec2, fallback: Vec2, calm: f32) -> WindInput {
    let speed = velocity.length();
    let toward = if speed > calm.max(1e-3) { velocity / speed } else { fallback.try_normalize().unwrap_or(Vec2::X) };
    WindInput { speed: if speed.is_finite() { speed } else { 0.0 }, toward }
}

/// The sea's wind state: the low-passed weather wind and the running transition.
#[derive(Clone, Debug, PartialEq)]
pub struct SeaWind {
    pub settings: SeaWindSettings,
    /// Host override: the sea keeps this wind whatever the weather.
    pub fixed: Option<WindInput>,
    filtered: Option<Vec2>,
    last_t: f64,
    wind: WindInput,
    transition: Option<SeaTransition>,
}

impl SeaWind {
    /// Starts at `wind`; the first [`Self::update`] with a weather wind snaps to it (a loaded game
    /// starts on its weather's sea, no ramp).
    pub fn new(wind: WindInput, settings: SeaWindSettings) -> Self {
        Self { settings, fixed: None, filtered: None, last_t: f64::NEG_INFINITY, wind, transition: None }
    }

    /// With a host override: the sea keeps `fixed` whatever the weather (`None` = follow it).
    #[must_use]
    pub fn with_fixed(mut self, fixed: Option<WindInput>) -> Self {
        self.fixed = fixed;
        self
    }

    /// The wind the sea is heading to (all of it once the transition is done).
    pub fn wind(&self) -> WindInput {
        self.wind
    }

    pub fn transition(&self) -> Option<SeaTransition> {
        self.transition
    }

    /// The transition's weight at `t` (1 without one).
    pub fn weight(&self, t: f64) -> f32 {
        self.transition.map_or(1.0, |tr| tr.weight(t))
    }

    /// Speed the sea has at `t` (the blend of the pair's speeds; diagnostics and tests).
    pub fn speed(&self, t: f64) -> f32 {
        self.transition.map_or(self.wind.speed, |tr| {
            let w = tr.weight(t);
            tr.from.speed + (self.wind.speed - tr.from.speed) * w
        })
    }

    /// Feeds the weather's mean 10 m wind (`None` = unknown) at time `t`. Returns `true` when the
    /// spectrum must be re-generated (a new transition, a snap or the override).
    #[allow(clippy::cast_possible_truncation)]
    pub fn update(&mut self, weather: Option<Vec2>, t: f64) -> bool {
        let s = self.settings;
        if let Some(fixed) = self.fixed {
            let changed = self.wind != fixed || self.transition.is_some();
            self.wind = fixed;
            self.transition = None;
            self.filtered = Some(fixed.toward * fixed.speed);
            self.last_t = t;
            return changed;
        }
        let Some(target) = weather.filter(|v| v.is_finite()) else { return false };
        let filtered = match self.filtered {
            // First wind, or the clock went back (a new world): snap.
            None => None,
            Some(_) if t < self.last_t => None,
            Some(f) => {
                let dt = (t - self.last_t) as f32;
                Some(f + (target - f) * (1.0 - (-dt / s.response_s.max(1e-3)).exp()))
            }
        };
        self.last_t = t;
        let Some(filtered) = filtered else {
            self.filtered = Some(target);
            self.wind = wind_input(target, self.wind.toward, s.calm);
            self.transition = None;
            return true;
        };
        self.filtered = Some(filtered);
        if self.transition.is_some_and(|tr| !tr.done(t)) {
            return false;
        }
        let next = wind_input(filtered, self.wind.toward, s.calm);
        if !self.moved(next) {
            return false;
        }
        self.transition = Some(SeaTransition { from: self.wind, start: t, duration: f64::from(s.blend_s.max(1e-3)) });
        self.wind = next;
        true
    }

    /// Whether `next` is far enough from the current wind to be worth a re-init.
    fn moved(&self, next: WindInput) -> bool {
        let s = self.settings;
        let dv = (next.speed - self.wind.speed).abs();
        if dv > s.min_speed_step.max(s.speed_step * self.wind.speed.max(next.speed)) {
            return true;
        }
        next.speed.min(self.wind.speed) > s.calm && next.toward.angle_to(self.wind.toward).abs().to_degrees() > s.angle_step_deg
    }
}
