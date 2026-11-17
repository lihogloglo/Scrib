//! Rain rings on the water (water.md "Ripples plan", slice R1): the puddles' procedural raindrop
//! rings (`sf_render_order_bevy::rain_rings`, CPU twin `sf_surface::layer::ripples`) tilt every
//! water surface's normal while it rains, scaled by the host's [`WaterRain`] intensity and faded by
//! the pixel footprint (no shimmer far off). The host feeds [`WaterRain`] (its weather's rain; 0
//! where no sky rains on the water, e.g. interiors) and binds [`WaterRainSettings`] from
//! [`crate::TUNABLES`]; this crate depends on no weather or surface crate.

use bevy_ecs::prelude::*;
use bevy_math::Vec4;
use sf_tunables::{def, Def, Group, Source, Unit};

/// The ring clock's wrap (s): the rings' drop rate (`RIPPLE_RATE`, 1.2 a second) times it is a whole
/// number of cycles, so the wrap never pops; the puddles' clock wraps the same
/// (`sf_surface::layer::RIPPLE_TIME_WRAP_S`).
pub const RAIN_CLOCK_WRAP_S: f64 = 1000.0; // tunables: const (maths: whole ring cycles per wrap)

/// `water.rain_ripple.strength`: how far one unit of ring slope tilts the water's normal.
pub const WATER_RAIN_RIPPLE_STRENGTH: Def<f32> = def!(float "water.rain_ripple.strength" = 1.0)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("Rain rings on the water: normal tilt per unit of ring slope (0 = no rings)")
    .source(Source::Guess) // WA-10 2026-11-17: the puddles' 0.5 was invisible on storm water
    .group(Group::Water);

/// `water.rain_ripple.fade_near_m`: the pixel footprint where the rings start to fade.
pub const WATER_RAIN_RIPPLE_FADE_NEAR_M: Def<f32> = def!(float "water.rain_ripple.fade_near_m" = 0.04)
    .range(0.0, 0.5)
    .unit(Unit::Meters)
    .help("Rain rings on the water start to fade once a pixel spans this much of the surface")
    .source(Source::Reference("the puddles' rings, sf_surface::layer::RIPPLE_FADE_NEAR_M"))
    .group(Group::Water);

/// `water.rain_ripple.fade_far_m`: the pixel footprint where the rings are gone (far and grazing
/// water): past it a ring is thinner than a pixel and would only shimmer under TAA or DLSS.
pub const WATER_RAIN_RIPPLE_FADE_FAR_M: Def<f32> = def!(float "water.rain_ripple.fade_far_m" = 0.15)
    .range(0.001, 1.0)
    .unit(Unit::Meters)
    .help("Rain rings on the water are gone once a pixel spans this much of the surface")
    .source(Source::Guess) // WA-10 2026-11-17: 0.06 m hid the rings past ~20 m
    .group(Group::Water);

/// The rain falling on the water this frame (the host sets it before [`crate::WaterSystems::Drive`]).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WaterRain {
    /// 0 dry .. 1 a storm (the ring grids turn on quarter by quarter). 0 where no sky rains on
    /// the water (interiors) and with the host's switch off.
    pub intensity: f32,
}

/// The rings' look (bound from [`crate::TUNABLES`] by the host; live both ways).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterRainSettings {
    pub strength: f32,
    pub fade_near_m: f32,
    pub fade_far_m: f32,
}

impl Default for WaterRainSettings {
    fn default() -> Self {
        Self {
            strength: WATER_RAIN_RIPPLE_STRENGTH.default_value(),
            fade_near_m: WATER_RAIN_RIPPLE_FADE_NEAR_M.default_value(),
            fade_far_m: WATER_RAIN_RIPPLE_FADE_FAR_M.default_value(),
        }
    }
}

/// The uniform's `rain` and `rain2` slots (`water_common.wesl` `WaterParams`): (intensity 0..1,
/// ring clock s, strength, 0) and (fade near m, fade far m, 0, 0). No rain or no strength: all
/// zero, and the water shader skips the rings.
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn slots(time: f64, rain: WaterRain, settings: WaterRainSettings) -> [Vec4; 2] {
    let intensity = if rain.intensity.is_finite() { rain.intensity.clamp(0.0, 1.0) } else { 0.0 };
    let strength = settings.strength.max(0.0);
    if intensity <= 0.0 || strength <= 0.0 {
        return [Vec4::ZERO; 2];
    }
    let clock = time.rem_euclid(RAIN_CLOCK_WRAP_S) as f32;
    let near = settings.fade_near_m.max(0.0);
    let far = settings.fade_far_m.max(near + 1e-4);
    [Vec4::new(intensity, clock, strength, 0.0), Vec4::new(near, far, 0.0, 0.0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)] // exact: the clamp, the slots passed through
    fn no_rain_no_rings() {
        let s = WaterRainSettings::default();
        assert_eq!(slots(12.0, WaterRain { intensity: 0.0 }, s), [Vec4::ZERO; 2]);
        assert_eq!(slots(12.0, WaterRain { intensity: -1.0 }, s), [Vec4::ZERO; 2]);
        assert_eq!(slots(12.0, WaterRain { intensity: f32::NAN }, s), [Vec4::ZERO; 2]);
        assert_eq!(slots(12.0, WaterRain { intensity: 1.0 }, WaterRainSettings { strength: 0.0, ..s }), [Vec4::ZERO; 2]);
        let [rain, fade] = slots(1012.5, WaterRain { intensity: 3.0 }, s);
        assert_eq!(rain.x, 1.0, "intensity clamps to a storm");
        assert!((rain.y - 12.5).abs() < 1e-3, "the clock wraps: {}", rain.y);
        assert_eq!(rain.z, s.strength);
        assert!(fade.x < fade.y);
        // The defaults are the puddles' (sf_surface::layer RIPPLE_TILT, RIPPLE_FADE_*).
        assert_eq!((s.strength, s.fade_near_m, s.fade_far_m), (0.5, 0.02, 0.06));
    }
}
