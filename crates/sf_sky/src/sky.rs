//! Sky settings and the per-frame sky state derived from them.

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::atmosphere::{AtmosphereModel, visible_disk_fraction};
use crate::celestial::{Celestial, Orbit};
use crate::clock::{Calendar, SkyClock};
use crate::curve::KeyCurve;
use crate::weather::Weather;

/// Everything that defines a world's sky. Data-driven: adapters fill it from
/// their records; everything has a sensible Earth-like default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkySettings {
    pub calendar: Calendar,
    pub orbit: Orbit,
    pub atmosphere: AtmosphereModel,
    /// Observer altitude above the atmosphere's ground, meters (for the CPU
    /// transmittance; the GPU sky uses the camera's real position).
    pub observer_altitude: f32,
    /// Sun illuminance above the atmosphere, lux (Earth ≈ 130 000).
    pub sun_illuminance: f32,
    /// Sun disk angular diameter, radians (Earth ≈ 0.0093).
    pub sun_angular_size: f32,
    /// Full-moon illuminance above the atmosphere, lux. Earth's is ≈ 0.25;
    /// games usually exaggerate it so nights stay readable.
    pub moon_illuminance: f32,
    /// Moon disk angular diameter, radians.
    pub moon_angular_size: f32,
    /// Camera EV100 as a function of sun elevation in degrees.
    pub exposure: KeyCurve,
    /// Extra ambient luminance (nits) as a function of sun elevation in
    /// degrees. Physically based sky light comes from the atmosphere's
    /// environment map; this is a small artistic floor for night readability.
    pub ambient: KeyCurve,
    /// Below this sun elevation (degrees) the moon takes over shadows.
    pub sun_shadow_min_elevation_deg: f32,
    /// The moon casts shadows only above this elevation (degrees).
    pub moon_shadow_min_elevation_deg: f32,
}

impl Default for SkySettings {
    fn default() -> Self {
        Self {
            calendar: Calendar::default(),
            orbit: Orbit::default(),
            atmosphere: AtmosphereModel::earth(),
            observer_altitude: 0.0,
            sun_illuminance: 130_000.0,
            sun_angular_size: 0.009_308_42,
            moon_illuminance: 1.0,
            moon_angular_size: 0.009_04,
            exposure: KeyCurve::new(vec![
                (-18.0, 0.5),
                (-12.0, 2.0),
                (-6.0, 5.5),
                (-2.0, 8.5),
                (2.0, 10.5),
                (10.0, 12.5),
                (25.0, 13.5),
                (45.0, 14.0),
            ]),
            ambient: KeyCurve::new(vec![(-12.0, 0.03), (-4.0, 0.0)]),
            sun_shadow_min_elevation_deg: -1.0,
            moon_shadow_min_elevation_deg: 3.0,
        }
    }
}

/// One directional light (sun or moon) as the renderer should see it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SkyLight {
    /// Unit vector from the scene toward the body (engine frame).
    pub direction: Vec3,
    /// Elevation above the horizon, radians.
    pub elevation: f32,
    /// Illuminance above the atmosphere, lux, after phase and weather. The
    /// value for shaders that apply atmospheric transmittance themselves.
    pub illuminance: f32,
    /// Atmospheric transmittance at the observer, times the visible part of
    /// the disk (RGB in `[0, 1]`). For renderers that don't apply it.
    pub transmittance: Vec3,
    /// This light should cast shadows (at most one of sun and moon does).
    pub casts_shadows: bool,
}

impl SkyLight {
    /// Illuminance reaching the ground, lux (luminance-weighted).
    pub fn ground_illuminance(&self) -> f32 {
        self.illuminance * luminance(self.transmittance)
    }

    /// Normalized RGB tint of the light reaching the ground (max = 1).
    pub fn ground_tint(&self) -> Vec3 {
        let max = self.transmittance.max_element();
        if max > 0.0 {
            self.transmittance / max
        } else {
            Vec3::ONE
        }
    }

    /// `(tint, illuminance)` such that `tint * illuminance` is the light at
    /// the ground, with `tint`'s largest channel 1.
    pub fn ground_light(&self) -> (Vec3, f32) {
        (
            self.ground_tint(),
            self.illuminance * self.transmittance.max_element(),
        )
    }
}

/// Rec. 709 relative luminance of a linear RGB value.
pub fn luminance(rgb: Vec3) -> f32 {
    rgb.dot(Vec3::new(0.2126, 0.7152, 0.0722))
}

/// The sky at one instant: what the renderer applies each frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SkyState {
    /// Hour of the day.
    pub hour: f32,
    pub celestial: Celestial,
    pub sun: SkyLight,
    pub moon: SkyLight,
    /// Camera EV100.
    pub ev100: f32,
    /// Ambient floor, nits.
    pub ambient: f32,
}

/// Quadrature steps for the CPU transmittance (error < 0.5% at the horizon).
const TRANSMITTANCE_SAMPLES: u32 = 48;

impl SkySettings {
    /// The sky at `clock` under `weather`.
    pub fn evaluate(&self, clock: &SkyClock, weather: &Weather) -> SkyState {
        let celestial: Celestial = self.orbit.evaluate(clock, &self.calendar);
        let direct = weather.direct_light_factor();
        let horizon = self.atmosphere.horizon_elevation(self.observer_altitude);
        let light = |pos: crate::SkyPosition, illuminance: f32, angular_size: f32| SkyLight {
            direction: pos.direction,
            elevation: pos.elevation,
            illuminance: illuminance * direct,
            transmittance: self.atmosphere.transmittance(
                self.observer_altitude,
                pos.elevation,
                weather.haze,
                TRANSMITTANCE_SAMPLES,
            ) * visible_disk_fraction(pos.elevation, horizon, angular_size),
            casts_shadows: false,
        };
        let mut sun = light(celestial.sun, self.sun_illuminance, self.sun_angular_size);
        let mut moon = light(
            celestial.moon,
            self.moon_illuminance * celestial.moon_brightness,
            self.moon_angular_size,
        );
        let sun_deg = celestial.sun.elevation_deg();
        if sun_deg > self.sun_shadow_min_elevation_deg {
            sun.casts_shadows = true;
        } else if celestial.moon.elevation_deg() > self.moon_shadow_min_elevation_deg
            && moon.illuminance > 0.0
        {
            moon.casts_shadows = true;
        }
        SkyState {
            hour: clock.hour(&self.calendar),
            celestial,
            sun,
            moon,
            ev100: self.exposure.sample(sun_deg),
            ambient: self.ambient.sample(sun_deg),
        }
    }
}
