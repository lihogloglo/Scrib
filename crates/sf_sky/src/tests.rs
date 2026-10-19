use glam::Vec3;

use crate::*;

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

/// Day of year of the June solstice, December solstice and equinoxes for
/// the default orbit (spring equinox on day 79).
fn season_day(quarter: f64) -> f64 {
    (79.0 + quarter * f64::from(Calendar::default().days_per_year) / 4.0).round()
}

fn sun_at(orbit: &Orbit, day: f64, hour: f64) -> SkyPosition {
    let cal = Calendar::default();
    orbit.evaluate(&SkyClock::at(day, hour, &cal), &cal).sun
}

fn orbit_at(latitude: f32) -> Orbit {
    Orbit {
        latitude_deg: latitude,
        ..Orbit::default()
    }
}

// ---- clock ----

#[test]
fn clock_advances_with_time_scale_and_pauses() {
    let cal = Calendar::default();
    let mut clock = SkyClock::at(10.0, 6.0, &cal);
    clock.time_scale = 60.0;
    // 60 real seconds at 60x is one game hour.
    clock.advance(60.0, &cal);
    assert!(close(clock.hour(&cal), 7.0, 1e-3));
    clock.paused = true;
    clock.advance(1000.0, &cal);
    assert!(close(clock.hour(&cal), 7.0, 1e-3));
    clock.set_hour(23.5, &cal);
    assert!(close(clock.hour(&cal), 23.5, 1e-3));
    assert!((clock.days - 10.0).abs() < 1.0);
}

#[test]
fn clock_wraps_days_and_years() {
    let cal = Calendar {
        hours_per_day: 20.0,
        days_per_year: 100.0,
    };
    let clock = SkyClock::at(250.0, 15.0, &cal);
    assert!(close(clock.hour(&cal), 15.0, 1e-3));
    assert!((clock.day_of_year(&cal) - 50.75).abs() < 1e-9);
}

// ---- sun ----

#[test]
fn equinox_noon_elevation_is_ninety_minus_latitude() {
    for lat in [0.0_f32, 30.0, 45.0, 60.0, -35.0] {
        let sun = sun_at(&orbit_at(lat), season_day(0.0), 12.0);
        assert!(
            close(sun.elevation_deg(), 90.0 - lat.abs(), 0.05),
            "lat {lat}: {}",
            sun.elevation_deg()
        );
    }
}

#[test]
fn solstice_noon_elevations() {
    let orbit = orbit_at(45.0);
    let tilt = orbit.axial_tilt_deg;
    // June solstice: 90 - 45 + 23.44; December: 90 - 45 - 23.44.
    let summer = sun_at(&orbit, season_day(1.0), 12.0);
    let winter = sun_at(&orbit, season_day(3.0), 12.0);
    assert!(close(summer.elevation_deg(), 45.0 + tilt, 0.05));
    assert!(close(winter.elevation_deg(), 45.0 - tilt, 0.05));
    // Southern hemisphere: seasons flip.
    let south = orbit_at(-45.0);
    assert!(close(
        sun_at(&south, season_day(3.0), 12.0).elevation_deg(),
        45.0 + tilt,
        0.05
    ));
    // Summer midnight at 45°N: sun below the horizon by 90 - 45 - tilt.
    let midnight = sun_at(&orbit, season_day(1.0), 0.0);
    assert!(close(midnight.elevation_deg(), -(45.0 - tilt), 0.05));
}

#[test]
fn polar_day_and_night() {
    let arctic = orbit_at(80.0);
    // June: the sun never sets; December: it never rises.
    for h in 0..24 {
        let hour = f64::from(h);
        assert!(sun_at(&arctic, season_day(1.0), hour).elevation > 0.0);
        assert!(sun_at(&arctic, season_day(3.0), hour).elevation < 0.0);
    }
}

#[test]
fn equinox_sun_rises_east_at_six_and_sets_west_at_eighteen() {
    let orbit = orbit_at(40.0);
    let rise = sun_at(&orbit, season_day(0.0), 6.0);
    let set = sun_at(&orbit, season_day(0.0), 18.0);
    assert!(close(rise.elevation_deg(), 0.0, 0.05));
    assert!(close(set.elevation_deg(), 0.0, 0.05));
    assert!(
        rise.direction.abs_diff_eq(Vec3::X, 1e-3),
        "{:?}",
        rise.direction
    );
    assert!(set.direction.abs_diff_eq(Vec3::NEG_X, 1e-3));
    // Northern hemisphere: the noon sun is due south (+Z).
    let noon = sun_at(&orbit, season_day(0.0), 12.0);
    assert!(noon.direction.z > 0.0 && noon.direction.x.abs() < 1e-4);
}

#[test]
fn north_heading_rotates_the_sky() {
    let orbit = Orbit {
        north_heading_deg: 90.0,
        ..orbit_at(40.0)
    };
    // North now points to +X, so east is +Z.
    let rise = sun_at(&orbit, season_day(0.0), 6.0);
    assert!(
        rise.direction.abs_diff_eq(Vec3::Z, 1e-3),
        "{:?}",
        rise.direction
    );
}

#[test]
fn day_curve_rises_monotonically_from_midnight_to_noon() {
    let orbit = orbit_at(45.0);
    for quarter in [0.0, 1.0, 2.0, 3.0] {
        let day = season_day(quarter);
        let mut prev = f32::NEG_INFINITY;
        for step in 0..=120 {
            let hour = f64::from(step) * 0.1;
            let e = sun_at(&orbit, day, hour).elevation;
            assert!(e >= prev - 1e-6, "hour {hour}");
            prev = e;
        }
        // And symmetric around noon.
        let am = sun_at(&orbit, day, 9.0).elevation;
        let pm = sun_at(&orbit, day, 15.0).elevation;
        assert!(close(am, pm, 1e-3));
    }
}

#[test]
fn days_are_longer_in_summer() {
    let orbit = orbit_at(50.0);
    let daylight_hours = |day: f64| {
        (0..240)
            .filter(|s| sun_at(&orbit, day, f64::from(*s) * 0.1).elevation > 0.0)
            .count()
    };
    assert!(daylight_hours(season_day(1.0)) > daylight_hours(season_day(0.0)));
    assert!(daylight_hours(season_day(0.0)) > daylight_hours(season_day(3.0)));
}

// ---- moon ----

#[test]
fn full_moon_transits_at_midnight_and_is_bright() {
    let cal = Calendar::default();
    let orbit = Orbit {
        moon_phase_at_epoch: 0.5,
        ..orbit_at(40.0)
    };
    let c = orbit.evaluate(&SkyClock::at(0.0, 0.0, &cal), &cal);
    assert!(close(c.moon_phase, 0.5, 1e-3));
    assert!(close(c.moon_illuminated, 1.0, 1e-3));
    assert!(close(c.moon_brightness, 1.0, 1e-3));
    // Opposite the sun.
    assert!(c.moon.direction.dot(c.sun.direction) < -0.9);
    assert!(c.moon.elevation > 0.5);

    let new_moon = Orbit {
        moon_phase_at_epoch: 0.0,
        ..orbit
    }
    .evaluate(&SkyClock::at(0.0, 12.0, &cal), &cal);
    assert!(new_moon.moon_brightness < 0.01);
    assert!(new_moon.moon.direction.dot(new_moon.sun.direction) > 0.9);
}

#[test]
fn quarter_moon_is_half_lit_but_dimmer_than_half() {
    let cal = Calendar::default();
    let orbit = Orbit {
        moon_phase_at_epoch: 0.25,
        ..Orbit::default()
    };
    let c = orbit.evaluate(&SkyClock::at(0.0, 0.0, &cal), &cal);
    assert!(close(c.moon_illuminated, 0.5, 0.01));
    assert!(c.moon_brightness < 0.4 && c.moon_brightness > 0.25);
}

// ---- atmosphere ----

#[test]
fn falloff_matches_its_endpoints() {
    for f in [
        Falloff::Linear,
        Falloff::Exponential { scale: 8.0 / 60.0 },
        Falloff::Exponential { scale: -0.3 },
    ] {
        assert!(close(f.sample(1.0), 1.0, 1e-5), "{f:?}");
        assert!(close(f.sample(0.0), 0.0, 1e-5), "{f:?}");
    }
    let tent = Falloff::Tent {
        center: 0.75,
        width: 0.3,
    };
    assert!(close(tent.sample(0.75), 1.0, 1e-6));
    assert!(close(tent.sample(0.6), 0.0, 1e-6));
    assert!(close(tent.sample(0.2), 0.0, 1e-6));
}

#[test]
fn transmittance_reddens_toward_the_horizon() {
    let atm = AtmosphereModel::earth();
    let zenith = atm.transmittance(0.0, 90f32.to_radians(), &MediumScales::haze(1.0), 64);
    let low = atm.transmittance(0.0, 5f32.to_radians(), &MediumScales::haze(1.0), 64);
    let below = atm.transmittance(0.0, -5f32.to_radians(), &MediumScales::haze(1.0), 64);
    // Clear-sky zenith transmittance is high, blue lowest.
    assert!(zenith.x > 0.9 && zenith.z > 0.6 && zenith.z < zenith.x);
    // Near the horizon: much dimmer and red-dominant.
    assert!(luminance(low) < 0.5 * luminance(zenith));
    assert!(low.x > low.y && low.y > low.z);
    assert_eq!(below, Vec3::ZERO);
    // Haze only dims.
    let hazy = atm.transmittance(0.0, 30f32.to_radians(), &MediumScales::haze(4.0), 64);
    let clear = atm.transmittance(0.0, 30f32.to_radians(), &MediumScales::haze(1.0), 64);
    assert!(hazy.cmple(clear).all() && luminance(hazy) < 0.95 * luminance(clear));
}

#[test]
fn transmittance_is_monotonic_in_elevation() {
    let atm = AtmosphereModel::earth();
    let mut prev = 0.0;
    for deg in 0..=90 {
        #[expect(clippy::cast_precision_loss)]
        let t = luminance(atm.transmittance(0.0, (deg as f32).to_radians(), &MediumScales::haze(1.0), 48));
        assert!(t >= prev, "{deg}°");
        prev = t;
    }
}

#[test]
fn dust_dims_and_warms_the_sun() {
    let atm = AtmosphereModel::earth();
    let at = |scales: MediumScales| atm.transmittance(0.0, 30f32.to_radians(), &scales, 64);
    let clear = at(MediumScales::haze(2.0));
    // No dust is exactly the dust-free medium.
    let mut none = MediumScales::haze(2.0);
    none.dust_tint = Vec3::new(3.0, 0.5, 2.0);
    assert_eq!(at(none), clear);
    let storm = at(MediumScales { dust: 4.0, ..MediumScales::haze(2.0) });
    assert!(luminance(storm) < 0.6 * luminance(clear), "{storm} vs {clear}");
    // Blue absorbed most: the remaining sunlight is warmer.
    assert!(storm.x / storm.z > clear.x / clear.z);
    // A tint with more green/blue absorption reddens it further.
    let red = at(MediumScales { dust: 4.0, dust_tint: Vec3::new(0.6, 1.4, 1.8), ..MediumScales::haze(2.0) });
    assert!(red.x / red.z > storm.x / storm.z);
    // Dust sits low: a summit 3 km up sees much less of it.
    let summit = atm.transmittance(3000.0, 30f32.to_radians(), &MediumScales { dust: 4.0, ..MediumScales::haze(2.0) }, 64);
    assert!(luminance(summit) > 1.5 * luminance(storm));
}

#[test]
fn dust_term_is_absorbing_and_low() {
    let atm = AtmosphereModel::earth();
    let dust = atm.terms.iter().find(|t| t.weather == WeatherScale::Dust).expect("dust term");
    let albedo = dust.scattering / dust.extinction();
    assert!(albedo.x > albedo.y && albedo.y > albedo.z && albedo.z > 0.6 && albedo.x < 0.95, "{albedo}");
    let ground = atm.extinction_at(0.0, &MediumScales { dust: 1.0, ..MediumScales::haze(0.0) });
    let high = atm.extinction_at(2000.0, &MediumScales { dust: 1.0, ..MediumScales::haze(0.0) });
    // Ground visibility at dust 1 (Koschmieder 3.912 / extinction): tens of km.
    let visibility = 3.912 / ground.y;
    assert!((25_000.0..60_000.0).contains(&visibility), "{visibility}");
    assert!(high.y < 0.5 * ground.y);
}

#[test]
fn medium_change_is_in_steps() {
    let steps = Vec3::new(0.05, 0.02, 0.02);
    let a = MediumScales::haze(2.0);
    assert!(a.change(&a, steps) < 1e-6);
    assert!(close(a.change(&MediumScales::haze(2.1), steps), 2.0, 1e-4));
    let dusty = MediumScales { dust: 0.01, ..a };
    assert!(a.change(&dusty, steps) < 1.0);
    let tinted = MediumScales { dust_tint: Vec3::new(1.0, 1.0, 1.1), ..a };
    assert!(a.change(&tinted, steps) > 4.0);
}

#[test]
fn weather_blends_dust() {
    let a = Weather::default();
    let b = Weather { dust: 4.0, dust_tint: Vec3::new(0.6, 1.4, 1.8), haze: 3.0, ..Weather::default() };
    let m = a.lerp(&b, 0.5).medium();
    assert!(close(m.dust, 2.0, 1e-6) && close(m.haze, 2.0, 1e-6));
    assert!(close(m.dust_tint.z, 1.4, 1e-6));
}

#[test]
fn visible_disk_fraction_is_a_segment_area() {
    let size = 0.01;
    assert!(close(visible_disk_fraction(0.0, 0.0, size), 0.5, 1e-6));
    assert!(close(visible_disk_fraction(0.006, 0.0, size), 1.0, 1e-6));
    assert!(close(visible_disk_fraction(-0.006, 0.0, size), 0.0, 1e-6));
    assert!(visible_disk_fraction(0.002, 0.0, size) > 0.5);
}

// ---- curves ----

#[test]
fn key_curve_interpolates_and_clamps() {
    let c = KeyCurve::new(vec![(10.0, 1.0), (0.0, 0.0), (20.0, 5.0)]);
    assert!(close(c.sample(-5.0), 0.0, 1e-6));
    assert!(close(c.sample(5.0), 0.5, 1e-6));
    assert!(close(c.sample(15.0), 3.0, 1e-6));
    assert!(close(c.sample(99.0), 5.0, 1e-6));
    assert!(c.is_monotonic());
    assert!(close(KeyCurve::default().sample(3.0), 0.0, 0.0));
    let json = serde_json::to_string(&c).unwrap();
    assert_eq!(json, "[[0.0,0.0],[10.0,1.0],[20.0,5.0]]");
}

// ---- sky state ----

#[test]
fn default_exposure_curve_brightens_with_the_sun() {
    let s = SkySettings::default();
    assert!(s.exposure.is_monotonic());
}

#[test]
fn sky_state_over_a_day() {
    let settings = SkySettings::default();
    let weather = Weather::default();
    let cal = settings.calendar;
    let at = |hour: f64| settings.evaluate(&SkyClock::at(season_day(1.0), hour, &cal), &weather);

    let noon = at(12.0);
    assert!(noon.sun.casts_shadows && !noon.moon.casts_shadows);
    assert!(noon.sun.ground_illuminance() > 80_000.0);
    assert!(noon.ev100 > 13.0);

    let midnight = at(0.0);
    assert!(!midnight.sun.casts_shadows);
    assert!(midnight.sun.ground_illuminance() <= 0.0);
    assert!(midnight.ev100 < noon.ev100 - 10.0);
    assert!(midnight.ambient > 0.0);

    // Sunset light is warm.
    let (tint, _) = at(19.5).sun.ground_light();
    assert!(tint.x >= tint.y && tint.y > tint.z);
}

#[test]
fn overcast_dims_direct_light() {
    let settings = SkySettings::default();
    let clock = SkyClock::at(100.0, 12.0, &settings.calendar);
    let clear = settings.evaluate(&clock, &Weather::default());
    let overcast = settings.evaluate(
        &clock,
        &Weather {
            cloud_cover: 1.0,
            ..Weather::default()
        },
    );
    assert!(overcast.sun.illuminance < 0.3 * clear.sun.illuminance);
}

#[test]
fn wind_is_deterministic_and_gusts_around_the_mean() {
    let wind = Wind::default();
    let a = wind.velocity_at(Vec3::new(10.0, 0.0, 3.0), 4.0);
    let b = wind.velocity_at(Vec3::new(10.0, 0.0, 3.0), 4.0);
    assert_eq!(a, b);
    let mean = wind.velocity.length();
    for i in 0..100 {
        #[expect(clippy::cast_precision_loss)]
        let v = wind.velocity_at(Vec3::new(i as f32 * 7.3, 0.0, 0.0), 0.0);
        assert!(v.y.abs() < f32::EPSILON);
        assert!(v.length() <= mean * (1.0 + wind.gustiness) + 1e-4);
        assert!(v.length() >= mean * (1.0 - wind.gustiness) - 1e-4);
    }
    let calm = Wind {
        velocity: glam::Vec2::ZERO,
        ..wind
    };
    assert_eq!(calm.velocity_at(Vec3::ONE, 1.0), Vec3::ZERO);
}

// ---- render path and proxies ----

#[test]
fn render_path_falls_back_to_raster() {
    assert_eq!(RenderPath::RayTraced.resolve(true), RenderPath::RayTraced);
    assert_eq!(RenderPath::RayTraced.resolve(false), RenderPath::Raster);
    assert_eq!(RenderPath::Raster.resolve(true), RenderPath::Raster);
}

#[test]
fn upscaling_parses_and_falls_back() {
    use crate::{UpscaleQuality, Upscaling};
    assert_eq!(Upscaling::parse("off"), Ok(Upscaling::Off));
    assert_eq!(Upscaling::parse("Quality"), Ok(Upscaling::Dlss(UpscaleQuality::Quality)));
    assert_eq!(Upscaling::parse("dlaa"), Ok(Upscaling::Dlss(UpscaleQuality::Native)));
    assert_eq!(
        Upscaling::parse("ultra_performance"),
        Ok(Upscaling::Dlss(UpscaleQuality::UltraPerformance))
    );
    assert!(Upscaling::parse("fsr").is_err());
    let q = Upscaling::Dlss(UpscaleQuality::Balanced);
    assert_eq!(q.resolve(true), q);
    assert_eq!(q.resolve(false), Upscaling::Taa, "no DLSS: the temporal resolve stays");
    assert_eq!(Upscaling::parse("TAA"), Ok(Upscaling::Taa));
    assert_eq!(Upscaling::Taa.resolve(false), Upscaling::Taa);
    assert_eq!(Upscaling::Off.resolve(true), Upscaling::Off);
    assert!(Upscaling::Taa.is_temporal() && q.is_temporal() && !Upscaling::Off.is_temporal());
    assert_eq!(q.label(), "balanced");
    for (name, quality) in UpscaleQuality::NAMES {
        assert_eq!(Upscaling::parse(name), Ok(Upscaling::Dlss(quality)));
    }
    let json = serde_json::to_string(&q).unwrap();
    assert_eq!(serde_json::from_str::<Upscaling>(&json).unwrap(), q);
}

#[test]
fn sphere_proxy_matches_point_light_intensity() {
    let lumens = 800.0;
    let r = 0.1;
    let l = sphere_light_luminance(lumens, r);
    // Intensity seen from afar: luminance × projected area = Φ / 4π.
    let intensity = l * std::f32::consts::PI * r * r;
    assert!(close(
        intensity,
        lumens / (4.0 * std::f32::consts::PI),
        1e-3
    ));
}

#[test]
fn settings_round_trip_through_json() {
    let s = SkySettings::default();
    let json = serde_json::to_string(&s).unwrap();
    let back: SkySettings = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
    // Partial data fills in defaults.
    let partial: SkySettings = serde_json::from_str(r#"{"orbit":{"latitude_deg":10}}"#).unwrap();
    assert!(close(partial.orbit.latitude_deg, 10.0, 0.0));
    assert!(close(partial.sun_illuminance, 130_000.0, 0.0));
}

#[test]
fn exposure_range_floors_the_night_and_leaves_the_day() {
    let mut settings = SkySettings::default();
    let weather = Weather::default();
    let cal = settings.calendar;
    let open = settings.evaluate(&SkyClock::at(season_day(1.0), 0.0, &cal), &weather).ev100;
    assert!(open < 3.5, "the default schedule opens past the floor at midnight: {open}");
    settings.exposure_range.min_ev100 = 3.5;
    let at = |hour: f64| settings.evaluate(&SkyClock::at(season_day(1.0), hour, &cal), &weather).ev100;
    assert!(close(at(0.0), 3.5, 1e-6));
    assert!(at(12.0) > 13.0);
    // A max below the min collapses to the min.
    let r = ExposureRange { min_ev100: 4.0, max_ev100: 2.0 };
    assert!(close(r.clamp(10.0), 4.0, 1e-6) && close(r.clamp(-3.0), 4.0, 1e-6));
}

#[test]
fn clamped_compensation_keeps_auto_exposure_in_range() {
    let r = ExposureRange { min_ev100: 3.5, max_ev100: 15.0 };
    // Base 7, a night street metered at -11 with -4.5 compensation: EV100 0.5, floored to 3.5.
    let c = r.clamp_compensation(7.0, -11.0, -4.5);
    assert!(close(7.0 - (c + 11.0), 3.5, 1e-5));
    // In range: unchanged.
    assert!(close(r.clamp_compensation(7.0, -3.0, -3.6), -3.6, 1e-6));
}

// ---- night sky: moons, stars, sun disk ----

fn two_moons() -> SkySettings {
    let circuit = |hour_angle_at_epoch_deg, tilt_deg| MoonPath::Circuit {
        circuit_days: 25.0 / 24.0,
        hour_angle_at_epoch_deg,
        tilt_deg,
        declination_deg: 0.0,
    };
    let frames = |n: &str| (0..8).map(|k| format!("{n}_{k}")).collect();
    SkySettings {
        moons: vec![
            Moon {
                name: "big".into(),
                angular_size: 0.12,
                phase_period_days: 24.0,
                phase_at_epoch: 0.5,
                phase_steps: 8,
                path: circuit(0.0, 35.0),
                illuminance: 0.8,
                textures: frames("big"),
                ..Moon::default()
            },
            Moon {
                name: "small".into(),
                angular_size: 0.05,
                phase_period_days: 24.0,
                phase_at_epoch: 0.5,
                phase_steps: 8,
                path: circuit(-20.0, 50.0),
                illuminance: 0.2,
                textures: frames("small"),
                ..Moon::default()
            },
        ],
        ..SkySettings::default()
    }
}

#[test]
fn circuit_moons_transit_at_epoch_and_rise_later_each_day() {
    let s = two_moons();
    let midnight = s.evaluate(&SkyClock { days: 0.0, time_scale: 0.0, paused: true }, &Weather::default());
    let big = midnight.night.moons()[0];
    // Highest at day 0 midnight, tilted 35° from the zenith.
    assert!(close(big.position.elevation_deg(), 55.0, 0.5), "{}", big.position.elevation_deg());
    // A day later it is 1/25 of a circuit behind: not yet back at transit.
    let next = s.evaluate(&SkyClock { days: 1.0, time_scale: 0.0, paused: true }, &Weather::default());
    let later = s.evaluate(&SkyClock { days: 1.0 + 1.0 / 24.0, time_scale: 0.0, paused: true }, &Weather::default());
    assert!(later.night.moons()[0].position.elevation > next.night.moons()[0].position.elevation);
    assert_eq!(midnight.night.moon_count, 2);
}

#[test]
fn moon_phases_snap_to_frames_and_cycle() {
    let s = two_moons();
    let at = |days: f64| s.evaluate(&SkyClock { days, ..SkyClock::default() }, &Weather::default()).night.moons()[0];
    let full = at(0.0);
    assert_eq!(full.frame, Some(4));
    assert!(close(full.brightness, 1.0, 1e-3));
    // 3 days per step of a 24-day cycle.
    assert_eq!(at(3.1).frame, Some(5));
    let new = at(12.0);
    assert_eq!(new.frame, Some(0));
    assert!(new.illuminance < 1e-3);
    assert_eq!(at(24.0).frame, Some(4));
}

#[test]
fn one_moonlight_sums_the_moons_toward_the_brightest() {
    let s = two_moons();
    let sky = s.evaluate(&SkyClock { days: 0.0, time_scale: 0.0, paused: true }, &Weather::default());
    let moons = sky.night.moons();
    let total: f32 = moons.iter().map(MoonState::ground_illuminance).sum();
    assert!(close(sky.moon.ground_illuminance(), total, total * 0.02 + 1e-6));
    assert!(sky.moon.direction.dot(moons[0].position.direction) > 0.999);
}

#[test]
fn default_sky_has_one_earth_moon_and_stars_fade_in_at_dusk() {
    let s = SkySettings::default();
    assert_eq!(s.moon_list().len(), 1);
    let noon = s.evaluate(&SkyClock::at(172.0, 12.0, &s.calendar), &Weather::default());
    let midnight = s.evaluate(&SkyClock::at(172.0, 0.0, &s.calendar), &Weather::default());
    assert!(close(noon.night.stars, 0.0, 1e-6));
    assert!(close(midnight.night.stars, 1.0, 1e-6));
    // The drawn sun is bigger than the physical one.
    assert!(s.sun_disk.angular_size > s.sun_angular_size * 2.0);
}

#[test]
fn far_shadow_view_is_texel_snapped_and_covers_the_reach() {
    let s = FarShadowSettings::default();
    assert!(close(s.texel(), 1.0, 1e-6));
    let to_sun = Vec3::new(0.4, 0.8, -0.3).normalize();
    // Re-centring moves the map by whole texels in the light's plane (texel
    // edges stay put: no shimmer), and the centre stays within a texel of the camera.
    let a = FarShadowView::fit(to_sun, Vec3::new(100.0, 20.0, 300.0), &s);
    let b = FarShadowView::fit(to_sun, Vec3::new(137.3, 22.0, 281.6), &s);
    let shift = a.view_from_world.transform_point3(b.centre);
    for v in [shift.x, shift.y] {
        assert!(close(v, v.round(), 1e-3), "{shift:?}");
    }
    assert!(a.centre.distance(Vec3::new(100.0, 20.0, 300.0)) <= s.texel());
    // The centre is at clip (0, 0) and a point `reach` along the light's x at the map edge.
    let c = a.clip_from_world.project_point3(a.centre);
    assert!(close(c.x, 0.0, 1e-4) && close(c.y, 0.0, 1e-4) && close(c.z, 0.5, 1e-4), "{c:?}");
    // Reverse Z: toward the sun is nearer (larger depth).
    let up = a.clip_from_world.project_point3(a.centre + to_sun * 100.0);
    assert!(up.z > c.z);
    let side = a.view_from_world.inverse().transform_vector3(Vec3::X);
    let edge = a.clip_from_world.project_point3(a.centre + side * s.reach);
    assert!(close(edge.x, 1.0, 1e-3) && close(edge.y, 0.0, 1e-3), "{edge:?}");
}

#[test]
fn far_shadow_cache_refreshes_on_sun_turn_recentre_and_invalidate_in_strips() {
    let s = FarShadowSettings { strips: 3, ..FarShadowSettings::default() };
    let sun = Vec3::new(0.3, 0.9, 0.1).normalize();
    let mut cache = FarShadowCache::default();
    let drain = |cache: &mut FarShadowCache, sun: Vec3, cam: Vec3| {
        let mut strips = Vec::new();
        while let FarShadowWork::Strip { strip, target, .. } = cache.update(sun, cam, &s) {
            strips.push((strip, target));
        }
        strips
    };
    // First refresh: three strips into map 0, then it is the front map.
    assert_eq!(drain(&mut cache, sun, Vec3::ZERO), vec![(0, 0), (1, 0), (2, 0)]);
    assert_eq!(cache.front().map(|(_, i)| i), Some(0));
    // Small sun turn and camera step: nothing.
    let small = glam::Quat::from_rotation_z(0.1_f32.to_radians()) * sun;
    assert!(drain(&mut cache, small, Vec3::new(50.0, 0.0, 0.0)).is_empty());
    // Sun past the threshold: refresh into the back map (1), then swap.
    let big = glam::Quat::from_rotation_z(0.4_f32.to_radians()) * sun;
    assert_eq!(drain(&mut cache, big, Vec3::ZERO), vec![(0, 1), (1, 1), (2, 1)]);
    assert_eq!(cache.front().map(|(_, i)| i), Some(1));
    // The camera moving 1/16 of the reach recentres.
    assert_eq!(drain(&mut cache, big, Vec3::new(140.0, 0.0, 0.0)).len(), 3);
    // A far cell arriving forces one.
    cache.invalidate();
    assert_eq!(drain(&mut cache, big, Vec3::new(140.0, 0.0, 0.0)).len(), 3);
}
