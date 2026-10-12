//! Weather inputs (the M6 hook) and the shared wind.
//!
//! M2 ships the data and the few effects that need no new rendering:
//! cloud cover dims direct sun and moon light, haze scales aerosol density,
//! dust scales the absorbing dust/ash term (weather.md N1/N2).
//! M6 adds presets with blending, clouds, fog and precipitation on top of
//! the same [`Weather`] values, and replaces [`Wind`]'s analytic gusts with
//! a sampled field. Consumers (grass, cloth, particles, clouds) only ever
//! call [`Wind::velocity_at`], so that swap doesn't touch them.

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::atmosphere::MediumScales;

/// The current weather, as continuous values that blend linearly.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Weather {
    /// Sky fraction covered by cloud, `[0, 1]`.
    pub cloud_cover: f32,
    /// Fraction of direct sun/moon light that overcast blocks at full cover.
    pub overcast_dimming: f32,
    /// Aerosol density multiplier: 1 is a clear day, higher is hazy or dusty.
    pub haze: f32,
    /// Density of the absorbing dust/ash terms (weather.md N2): 0 is none,
    /// 1 a dusty day, 4-5 an ash storm.
    pub dust: f32,
    /// Per-channel multiplier on the dust terms' absorption: `ONE` is the
    /// model's grey-brown dust; more absorption in green and blue reddens
    /// what is left (a red dust storm).
    pub dust_tint: Vec3,
    pub wind: Wind,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            cloud_cover: 0.0,
            overcast_dimming: 0.8,
            haze: 1.0,
            dust: 0.0,
            dust_tint: Vec3::ONE,
            wind: Wind::default(),
        }
    }
}

impl Weather {
    /// Linear blend toward `to` (t = 0 is `self`). Wind direction blends as
    /// a vector so a 180° change passes through calm, not sideways.
    #[must_use]
    pub fn lerp(&self, to: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mix = |a: f32, b: f32| a + (b - a) * t;
        Self {
            cloud_cover: mix(self.cloud_cover, to.cloud_cover),
            overcast_dimming: mix(self.overcast_dimming, to.overcast_dimming),
            haze: mix(self.haze, to.haze),
            dust: mix(self.dust, to.dust),
            dust_tint: self.dust_tint.lerp(to.dust_tint, t),
            wind: self.wind.lerp(&to.wind, t),
        }
    }

    /// What this weather does to the atmosphere's medium.
    pub fn medium(&self) -> MediumScales {
        MediumScales {
            haze: self.haze,
            dust: self.dust,
            dust_tint: self.dust_tint,
        }
    }

    /// Multiplier on direct sun and moon light from cloud cover.
    pub fn direct_light_factor(&self) -> f32 {
        1.0 - self.cloud_cover.clamp(0.0, 1.0) * self.overcast_dimming.clamp(0.0, 1.0)
    }
}

/// Wind over the world: a mean velocity plus gusts.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Wind {
    /// Mean horizontal velocity in m/s (x, z in the engine frame).
    pub velocity: Vec2,
    /// Gust amplitude relative to the mean speed, `[0, 1]`.
    pub gustiness: f32,
    /// Length of a gust front in meters (spatial period).
    pub gust_length: f32,
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(2.0, 0.5),
            gustiness: 0.3,
            gust_length: 60.0,
        }
    }
}

impl Wind {
    #[must_use]
    pub fn lerp(&self, to: &Self, t: f32) -> Self {
        Self {
            velocity: self.velocity.lerp(to.velocity, t),
            gustiness: self.gustiness + (to.gustiness - self.gustiness) * t,
            gust_length: self.gust_length + (to.gust_length - self.gust_length) * t,
        }
    }

    /// Wind velocity (m/s) at world position `pos` (meters, origin-stable
    /// coordinates) and time `t` (seconds). Gusts are fronts that travel
    /// with the wind; deterministic, so every consumer agrees.
    pub fn velocity_at(&self, pos: Vec3, t: f32) -> Vec3 {
        let speed = self.velocity.length();
        if speed <= f32::EPSILON {
            return Vec3::ZERO;
        }
        let dir = self.velocity / speed;
        let k = std::f32::consts::TAU / self.gust_length.max(1.0);
        // Distance along the wind, minus how far the front has traveled.
        let s = (dir.dot(Vec2::new(pos.x, pos.z)) - speed * t) * k;
        // Two incommensurate waves so gusts don't look periodic.
        let gust = 0.6 * s.sin() + 0.4 * (2.31 * s + 1.7).sin();
        let v = self.velocity * (1.0 + self.gustiness * gust);
        Vec3::new(v.x, 0.0, v.y)
    }
}
