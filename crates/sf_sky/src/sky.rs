//! Sky settings and the per-frame sky state derived from them.

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::atmosphere::{AtmosphereModel, visible_disk_fraction};
use crate::celestial::{Celestial, Orbit};
use crate::clock::{Calendar, SkyClock};
use crate::curve::KeyCurve;
use crate::night::{MAX_MOONS, Moon, MoonState, NightSky, StarField, SunDiskLook};
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
    /// The moons. Empty: one Earth-like moon from the orbit's lunar month,
    /// [`Self::moon_illuminance`] and [`Self::moon_angular_size`]. At most
    /// [`MAX_MOONS`] are evaluated; one moonlight carries their sum.
    pub moons: Vec<Moon>,
    /// The star field.
    pub stars: StarField,
    /// How the sun disk is drawn (size, glare).
    pub sun_disk: SunDiskLook,
    /// Camera EV100 as a function of sun elevation in degrees.
    pub exposure: KeyCurve,
    /// Limits of the camera EV100, whatever drives it (this schedule or
    /// histogram auto exposure): UE's auto-exposure Min/Max EV100. The
    /// floor keeps a night from opening the camera so far that local lights,
    /// calibrated for a brighter exposure, blow out.
    pub exposure_range: ExposureRange,
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
            moons: Vec::new(),
            stars: StarField::default(),
            sun_disk: SunDiskLook::default(),
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
            exposure_range: ExposureRange::default(),
            ambient: KeyCurve::new(vec![(-12.0, 0.03), (-4.0, 0.0)]),
            sun_shadow_min_elevation_deg: -1.0,
            moon_shadow_min_elevation_deg: 3.0,
        }
    }
}

/// The range a camera's EV100 may take (inclusive). The default is wide
/// (no clamp); games set the floor from their local-light calibration.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExposureRange {
    /// Darkest scenes open the camera no further than this (night floor).
    pub min_ev100: f32,
    /// Brightest scenes close it no further than this.
    pub max_ev100: f32,
}

impl Default for ExposureRange {
    fn default() -> Self {
        Self { min_ev100: -10.0, max_ev100: 20.0 }
    }
}

impl ExposureRange {
    /// `ev100` within the range (`max` below `min` counts as `min`).
    pub fn clamp(&self, ev100: f32) -> f32 {
        ev100.clamp(self.min_ev100, self.max_ev100.max(self.min_ev100))
    }

    /// For histogram auto exposure that adds `compensation - metered` EV to a
    /// camera at `base_ev100` (Bevy's `AutoExposure` compensation curve, whose
    /// axis is the metered average log2 luminance at the base exposure): the
    /// compensation that keeps the resulting EV100 in range.
    pub fn clamp_compensation(&self, base_ev100: f32, metered: f32, compensation: f32) -> f32 {
        metered + base_ev100 - self.clamp(base_ev100 - (compensation - metered))
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
    /// The drawn moons and the stars' visibility.
    pub night: NightSky,
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
        let night = self.evaluate_night(clock, weather, celestial.sun.elevation_deg(), horizon);
        let mut moon = moonlight(night.moons());
        let sun_deg = celestial.sun.elevation_deg();
        if sun_deg > self.sun_shadow_min_elevation_deg {
            sun.casts_shadows = true;
        } else if moon.elevation.to_degrees() > self.moon_shadow_min_elevation_deg
            && moon.illuminance > 0.0
        {
            moon.casts_shadows = true;
        }
        SkyState {
            hour: clock.hour(&self.calendar),
            celestial,
            sun,
            moon,
            ev100: self.exposure_range.clamp(self.exposure.sample(sun_deg)),
            ambient: self.ambient.sample(sun_deg),
            night,
        }
    }

    /// The moons to draw: [`Self::moons`], or the Earth-like default.
    pub fn moon_list(&self) -> std::borrow::Cow<'_, [Moon]> {
        if !self.moons.is_empty() {
            return std::borrow::Cow::Borrowed(&self.moons);
        }
        if self.orbit.lunar_month_days <= 0.0 {
            return std::borrow::Cow::Borrowed(&[]);
        }
        std::borrow::Cow::Owned(vec![Moon {
            angular_size: self.moon_angular_size,
            phase_period_days: self.orbit.lunar_month_days,
            phase_at_epoch: self.orbit.moon_phase_at_epoch,
            illuminance: self.moon_illuminance,
            ..Moon::default()
        }])
    }

    fn evaluate_night(&self, clock: &SkyClock, weather: &Weather, sun_deg: f32, horizon: f32) -> NightSky {
        let direct = weather.direct_light_factor();
        let mut night = NightSky::default();
        for (slot, moon) in night.moons.iter_mut().zip(self.moon_list().iter()) {
            let (phase, frame) = moon.phase(clock);
            let position = moon.position(&self.orbit, clock, &self.calendar, phase);
            let (illuminated, brightness) = crate::night::phase_light(phase);
            let full = moon.illuminance * direct;
            #[expect(clippy::cast_possible_truncation, reason = "fraction")]
            let phase = phase as f32;
            *slot = MoonState {
                position,
                phase,
                illuminated,
                brightness,
                frame,
                illuminance: full * brightness,
                full_illuminance: full,
                transmittance: self.atmosphere.transmittance(
                    self.observer_altitude,
                    position.elevation,
                    weather.haze,
                    TRANSMITTANCE_SAMPLES,
                ) * visible_disk_fraction(position.elevation, horizon, moon.angular_size),
                angular_size: moon.angular_size,
                tint: Vec3::from(moon.tint),
                occluder: moon.occluder,
            };
            night.moon_count += 1;
        }
        night.stars = if self.stars.enabled {
            self.stars.visibility.sample(sun_deg).clamp(0.0, 1.0)
        } else {
            0.0
        };
        night
    }
}

/// One moonlight for all moons: from the brightest moon at the ground, with
/// the illuminance that makes its ground light the sum of all of theirs
/// (one shadow-casting light rather than one per moon, as games do).
fn moonlight(moons: &[MoonState]) -> SkyLight {
    let Some(brightest) = moons
        .iter()
        .max_by(|a, b| a.ground_illuminance().total_cmp(&b.ground_illuminance()))
    else {
        return SkyLight {
            direction: Vec3::NEG_Y,
            elevation: -std::f32::consts::FRAC_PI_2,
            ..SkyLight::default()
        };
    };
    let total: f32 = moons.iter().map(MoonState::ground_illuminance).sum();
    let own = luminance(brightest.transmittance);
    let illuminance = if own > 1e-6 { total / own } else { brightest.illuminance };
    // The disk tint colors the light too (normalized to luminance 1).
    let tint = brightest.tint / luminance(brightest.tint).max(1e-6);
    SkyLight {
        direction: brightest.position.direction,
        elevation: brightest.position.elevation,
        illuminance,
        transmittance: brightest.transmittance * tint,
        casts_shadows: false,
    }
}

const _: () = assert!(MAX_MOONS >= 1);
