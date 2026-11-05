//! Weather inputs (the M6 hook) and the shared wind.
//!
//! M2 ships the data and the few effects that need no new rendering:
//! cloud cover dims direct sun and moon light, haze scales aerosol density,
//! dust scales the absorbing dust/ash term (weather.md N1/N2).
//! M6 adds presets with blending, clouds, fog and precipitation on top of
//! the same [`Weather`] values. [`Wind`] lives in `sf_wind` (the shared
//! wind field, weather.md "Wind") and is re-exported here.

use glam::Vec3;
use serde::{Deserialize, Serialize};
pub use sf_wind::Wind;

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
    /// Multiplier on `SunDiskLook::glare`, `[0, 1]`: 1 a clear sky, 0 no
    /// halo (overcast, rain, fog).
    pub sun_glare: f32,
    /// Linear RGB tint of the ambient floor ([`crate::SkySettings::ambient`]),
    /// its luminance relative to the renderer's default tint: luminance 1
    /// keeps the floor's brightness and changes only its colour. `None`: the
    /// renderer's default tint (no weather feeds one).
    pub ambient_tint: Option<Vec3>,
    /// Linear RGB hue of the night sky's glow ([`crate::NightGlow`]); only the hue is used
    /// (the brightness follows the ambient floor). `None`: the floor's colour.
    pub night_sky_tint: Option<Vec3>,
    /// Linear RGB hue of the moonlight (N3); only the hue is used (the
    /// brightness is [`crate::Moonlight`]'s). `None`: the brightest moon's tint.
    pub moonlight_tint: Option<Vec3>,
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
            sun_glare: 1.0,
            ambient_tint: None,
            night_sky_tint: None,
            moonlight_tint: None,
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
            sun_glare: mix(self.sun_glare, to.sun_glare),
            // A side without a tint has no say: the other side's tint holds.
            ambient_tint: match (self.ambient_tint, to.ambient_tint) {
                (Some(a), Some(b)) => Some(a.lerp(b, t)),
                (a, b) => b.or(a),
            },
            night_sky_tint: match (self.night_sky_tint, to.night_sky_tint) {
                (Some(a), Some(b)) => Some(a.lerp(b, t)),
                (a, b) => b.or(a),
            },
            moonlight_tint: match (self.moonlight_tint, to.moonlight_tint) {
                (Some(a), Some(b)) => Some(a.lerp(b, t)),
                (a, b) => b.or(a),
            },
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
