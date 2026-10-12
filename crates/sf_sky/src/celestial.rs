//! Sun and moon positions from time of day, season and latitude.
//!
//! The standard low-precision model (e.g. Meeus, *Astronomical Algorithms*,
//! ch. 13 and 25, reduced to a circular orbit): solar declination from the
//! axial tilt and the season, hour angle from local solar time, then the
//! equatorial-to-horizontal transform. Clock time is local solar time (no
//! equation of time, no time zones). The moon keeps the sun's declination
//! law at its own ecliptic longitude and lags the sun by its phase; its
//! orbital inclination is ignored. Good to well under a degree for the sun,
//! a few degrees for the moon: plenty for lighting.

use std::f64::consts::{PI, TAU};

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::clock::{Calendar, SkyClock};

/// Where the world sits on its planet, and the planet's orbit and moon.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Orbit {
    /// Latitude in degrees, positive north. Sets day length and sun height.
    pub latitude_deg: f32,
    /// Axial tilt in degrees (Earth: 23.44). 0 means no seasons.
    pub axial_tilt_deg: f32,
    /// Day of the year (in [`SkyClock::day_of_year`] terms) of the equinox
    /// after which the northern hemisphere has summer.
    pub spring_equinox_day: f32,
    /// Heading of geographic north in the engine frame, in degrees,
    /// clockwise seen from above. 0: north is −Z (and east is +X).
    pub north_heading_deg: f32,
    /// Synodic month in days (new moon to new moon). 0 disables the moon.
    pub lunar_month_days: f32,
    /// Moon phase at day 0, in `[0, 1)`: 0 new, 0.5 full.
    pub moon_phase_at_epoch: f32,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            latitude_deg: 45.0,
            axial_tilt_deg: 23.44,
            // ~March 20 with day 0 = January 1.
            spring_equinox_day: 79.0,
            north_heading_deg: 0.0,
            lunar_month_days: 29.530_59,
            moon_phase_at_epoch: 0.0,
        }
    }
}

/// A body's direction in the sky.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SkyPosition {
    /// Unit vector from the observer toward the body, engine frame
    /// (Y up, north per [`Orbit::north_heading_deg`]).
    pub direction: Vec3,
    /// Angle above the mathematical horizon, in radians.
    pub elevation: f32,
}

impl SkyPosition {
    /// From declination, hour angle and latitude (radians).
    fn from_equatorial(declination: f64, hour_angle: f64, latitude: f64, north: Quat) -> Self {
        let (sin_d, cos_d) = declination.sin_cos();
        let (sin_h, cos_h) = hour_angle.sin_cos();
        let (sin_p, cos_p) = latitude.sin_cos();
        // East-north-up components.
        let east = -cos_d * sin_h;
        let north_c = cos_p * sin_d - sin_p * cos_d * cos_h;
        let up = sin_p * sin_d + cos_p * cos_d * cos_h;
        #[expect(clippy::cast_possible_truncation, reason = "unit vector")]
        let enu = Vec3::new(east as f32, up as f32, -north_c as f32).normalize();
        #[expect(clippy::cast_possible_truncation, reason = "angle")]
        let elevation = up.clamp(-1.0, 1.0).asin() as f32;
        Self {
            direction: north * enu,
            elevation,
        }
    }

    /// Elevation in degrees.
    pub fn elevation_deg(&self) -> f32 {
        self.elevation.to_degrees()
    }
}

/// The sun and moon at one instant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Celestial {
    pub sun: SkyPosition,
    pub moon: SkyPosition,
    /// Moon phase in `[0, 1)`: 0 new, 0.25 first quarter, 0.5 full.
    pub moon_phase: f32,
    /// Lit fraction of the moon's disk, in `[0, 1]`.
    pub moon_illuminated: f32,
    /// Moon brightness relative to full moon, in `[0, 1]` (Lambert phase law
    /// of a diffuse sphere; falls off faster than the lit fraction).
    pub moon_brightness: f32,
}

impl Orbit {
    /// Rotation from the canonical frame (north = −Z) to the engine frame.
    fn north_rotation(&self) -> Quat {
        Quat::from_rotation_y(-self.north_heading_deg.to_radians())
    }

    /// Solar declination (radians) on fractional day-of-year `day`.
    pub fn solar_declination(&self, day: f64, calendar: &Calendar) -> f64 {
        let longitude = self.solar_longitude(day, *calendar);
        (f64::from(self.axial_tilt_deg).to_radians().sin() * longitude.sin()).asin()
    }

    /// Sun's ecliptic longitude (radians), 0 at the spring equinox.
    ///
    /// Held constant through each day (taken at the day's start), so every
    /// day is symmetric around noon and repeats exactly; the real drift is
    /// at most 0.4° of declination per day, and the step lands at midnight.
    fn solar_longitude(&self, day: f64, calendar: Calendar) -> f64 {
        TAU * (day.floor() - f64::from(self.spring_equinox_day)) / f64::from(calendar.days_per_year)
    }

    /// Sun and moon positions at `clock`.
    pub fn evaluate(&self, clock: &SkyClock, calendar: &Calendar) -> Celestial {
        let north = self.north_rotation();
        let latitude = f64::from(self.latitude_deg).to_radians();
        let tilt = f64::from(self.axial_tilt_deg).to_radians();
        let day = clock.day_of_year(calendar);
        // Solar noon at half the day.
        let sun_hour_angle = TAU * (clock.day_fraction() - 0.5);
        let sun_longitude = self.solar_longitude(day, *calendar);
        let sun_declination = (tilt.sin() * sun_longitude.sin()).asin();
        let sun = SkyPosition::from_equatorial(sun_declination, sun_hour_angle, latitude, north);

        if self.lunar_month_days <= 0.0 {
            return Celestial {
                sun,
                moon: SkyPosition {
                    direction: Vec3::NEG_Y,
                    elevation: -std::f32::consts::FRAC_PI_2,
                },
                ..Celestial::default()
            };
        }
        let phase = (clock.days / f64::from(self.lunar_month_days)
            + f64::from(self.moon_phase_at_epoch))
        .rem_euclid(1.0);
        // The moon runs ahead of the sun in longitude by its phase, so it
        // transits later: full moon (phase 0.5) transits at midnight.
        let moon_longitude = sun_longitude + TAU * phase;
        let moon_declination = (tilt.sin() * moon_longitude.sin()).asin();
        let moon_hour_angle = sun_hour_angle - TAU * phase;
        let moon = SkyPosition::from_equatorial(moon_declination, moon_hour_angle, latitude, north);

        // Phase angle (sun-moon-observer): π at new moon, 0 at full.
        let phase_angle = (PI - TAU * phase).abs();
        let illuminated = f64::midpoint(1.0, phase_angle.cos());
        let brightness = ((PI - phase_angle) * phase_angle.cos() + phase_angle.sin()) / PI;
        #[expect(clippy::cast_possible_truncation, reason = "fractions")]
        Celestial {
            sun,
            moon,
            moon_phase: phase as f32,
            moon_illuminated: illuminated.clamp(0.0, 1.0) as f32,
            moon_brightness: brightness.clamp(0.0, 1.0) as f32,
        }
    }
}
