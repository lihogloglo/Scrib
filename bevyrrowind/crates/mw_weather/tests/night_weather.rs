//! WE-9 (owner 2026-11-06): forced Foggy / Thunderstorm seemed to go away at night. These tests
//! pin what the weather data does through the night (it doesn't change) and print what the look
//! does: precipitation brightness, fog densities, wetting rates, and what a live force (the C key)
//! does when the hour keys wrap past midnight (`SkyClock::set_hour` keeps the day: 23.5 + 0.5 is
//! 0:00 of the *same* day, 24 game hours back, before the force started).

use mw_weather::{MwWeather, WeatherType, climate_id};
use sf_records::{RecordDb, RecordId};
use sf_weather::sf_sky::{SkyClock, SkySettings, Weather};
use sf_weather::{WeatherBook, WeatherFrame, WeatherInput, WeatherState};
use tes3::esp::{Region, WeatherChances};

const ISLES: &str = "Ascadian Isles Region";
/// `--day` default (main.rs).
const DAY: f64 = 172.0;

fn book() -> WeatherBook {
    let mut db = RecordDb::new();
    db.register::<Region>("REGN").unwrap();
    let esm = db.add_source("Test.esm", &[]);
    let region = Region {
        id: ISLES.to_owned(),
        name: ISLES.to_owned(),
        // Morrowind.esm's Ascadian Isles chances.
        weather_chances: WeatherChances { clear: 45, cloudy: 30, foggy: 0, overcast: 10, rain: 10, thunder: 5, ash: 0, blight: 0, snow: 0, blizzard: 0 },
        ..Region::default()
    };
    db.define(esm, RecordId::new(ISLES), region).unwrap();
    let w = MwWeather::build(&db, None);
    WeatherBook::new(w.presets, w.climates, w.precip_kinds)
}

fn frame(book: &WeatherBook, state: &WeatherState, hours: f64) -> WeatherFrame {
    let climates = [(book.climate(&climate_id(ISLES)).unwrap(), 1.0)];
    #[expect(clippy::cast_possible_truncation, reason = "an hour fits f32")]
    let hour_of_day = hours.rem_euclid(24.0) as f32;
    let input = WeatherInput { hours, prev_hours: hours, hour_of_day, altitude: 0.0, climates: &climates };
    sf_weather::evaluate(book, state, &input)
}

/// `--weather <ty>` at launch (`WeatherSetup::build`): forced from 1e4 h before, held 1e7 h.
fn launch_forced(book: &WeatherBook, ty: WeatherType, launch: f64) -> WeatherState {
    let mut state = WeatherState::new(0);
    let c = book.climate(&climate_id(ISLES)).unwrap();
    state.force(book.climate_id(c), &ty.preset_id(), launch - 1.0e4, Some(0.0), Some(1.0e7));
    state
}

fn rain(f: &WeatherFrame) -> f32 {
    f.precip.iter().map(|p| p.intensity).sum()
}

/// The night hours the owner looked at, as absolute game hours from `DAY` 12:00 on.
fn night_hours() -> Vec<f64> {
    [12.0, 18.0, 21.0, 23.5, 24.0, 27.0, 30.0, 31.0].iter().map(|h| DAY * 24.0 + h).collect()
}

/// Logic: a weather forced at launch keeps its precipitation, fog and wetting all night.
#[test]
fn launch_forced_weather_holds_through_the_night() {
    let book = book();
    let storm = launch_forced(&book, WeatherType::Thunderstorm, DAY * 24.0 + 10.0);
    let fog = launch_forced(&book, WeatherType::Foggy, DAY * 24.0 + 10.0);
    let noon_fog = frame(&book, &fog, DAY * 24.0 + 12.0).fog;
    for h in night_hours() {
        let s = frame(&book, &storm, h);
        assert!(rain(&s) > 0.99, "thunderstorm rain at {h}: {}", rain(&s));
        assert!(s.surface.wet > 0.0 && s.surface.puddle > 0.0, "wetting at {h}");
        let f = frame(&book, &fog, h).fog;
        assert!(f.height.density >= noon_fog.height.density - 1e-9, "fog thins at {h}");
        assert!(f.banks.layer.density >= noon_fog.banks.layer.density - 1e-9, "banks thin at {h}");
    }
}

/// `sf_weather_bevy::fx::precip_brightness` (copied: that crate is Bevy).
fn precip_brightness(sun_elevation: f32, overcast_dimming: f32) -> f32 {
    let t = ((sun_elevation + 0.1) / 0.3).clamp(0.0, 1.0);
    let day = t * t * (3.0 - 2.0 * t);
    (0.04 + 0.96 * day) * (1.0 - 0.5 * overcast_dimming.clamp(0.0, 1.0))
}

#[test]
#[ignore = "measurement: WE-9 night weather look terms"]
fn print_night_weather_terms() {
    let book = book();
    let sky = SkySettings::default();
    let launch = DAY * 24.0 + 10.0;
    for ty in [WeatherType::Foggy, WeatherType::Thunderstorm] {
        let state = launch_forced(&book, ty, launch);
        println!("{} forced at launch:", ty.name());
        println!("  hour  sun_deg  rain  brightness  height_ext  bank_ext  vis_m  wet  puddle");
        for h in night_hours() {
            let f = frame(&book, &state, h);
            let clock = SkyClock::at(0.0, h, &sky.calendar);
            let sun = sky.evaluate(&clock, &Weather::default()).sun.elevation;
            println!(
                "  {:5.2} {:7.1} {:5.2} {:10.3} {:11.5} {:9.5} {:6.0} {:4.2} {:7.2}",
                h.rem_euclid(24.0),
                sun.to_degrees(),
                rain(&f),
                precip_brightness(sun, f.sky.overcast_dimming),
                f.fog.height.density,
                f.fog.banks.layer.density,
                f.visibility_m,
                f.surface.wet,
                f.surface.puddle,
            );
        }
    }
    // Live force (the C key, `force_live`): from `now`, held 1e6 h. Then the `]` key steps the
    // hour with `SkyClock::set_hour`, which keeps the day count.
    let mut clock = SkyClock::at(DAY, 7.0, &sky.calendar);
    let hours = |c: &SkyClock| c.days * f64::from(sky.calendar.hours_per_day);
    let mut state = WeatherState::new(0);
    let c = book.climate(&climate_id(ISLES)).unwrap();
    state.force(book.climate_id(c), &WeatherType::Thunderstorm.preset_id(), hours(&clock), None, Some(1.0e6));
    println!("Thunderstorm forced live at 7:00, hour keys (]) through midnight:");
    for _ in 0..48 {
        let h = f64::from(clock.hour(&sky.calendar)) + 0.5;
        clock.set_hour(h, &sky.calendar);
        let f = frame(&book, &state, hours(&clock));
        let natural = state.current(&book, c, hours(&clock)).map(|p| book.preset_id(p).as_str().to_owned());
        println!("  shown {:5.2}  weather clock {:9.2}  rain {:4.2}  weather {natural:?}", clock.hour(&sky.calendar), hours(&clock), rain(&f));
    }
}

/// WE-9 cause 2: the fog's ambient in-scatter for Foggy at noon, 21:00 and 23:30. The fog's fill is
/// the sky fill (GPU only: the sky-view LUT, not printed) plus the view's ambient, the surfaces' own
/// ambient floor (`sf_atmos::fog_fill`, `VIEW_AMBIENT_IN_FOG` 1). Printed: the floor (nits, the
/// weather's tint), the thick-fog radiance it adds (albedo × fill × the weather's fog `ambient`),
/// a grey stone's (albedo 0.4) under the same floor, both exposed (EV100). Before the fix the fog's
/// column was 0 at night: a black absorber over floor-lit ground.
#[test]
#[ignore = "measurement: WE-9 night fog in-scatter"]
fn print_night_fog_inscatter() {
    use sf_weather::sf_sky::{ExposureRange, KeyCurve, luminance};
    use glam::Vec3;
    // `mw_assets::LightCalibration` (Bevy crate, copied): `apply_sky` and its defaults.
    let night_ambient_nits = 4.0;
    let sky = SkySettings {
        exposure_range: ExposureRange { min_ev100: 3.5, ..ExposureRange::default() },
        ambient: KeyCurve::new(vec![(-12.0, night_ambient_nits), (-4.0, 0.0)]),
        ..SkySettings::default()
    };
    // `sf_sky_bevy::SkyRenderOptions::ambient_color` (the tint's reference luminance, copied).
    let reference = luminance(Vec3::new(0.55, 0.65, 1.0));
    let book = book();
    let state = launch_forced(&book, WeatherType::Foggy, DAY * 24.0 + 10.0);
    println!("Foggy forced at launch: fog ambient in-scatter (nits; exposed = x 2^-EV / 1.2)");
    println!("  hour  sun_deg   ev100  floor  view_ambient(rgb)        fog_fill_lum  stone_lum  fog_exposed  stone_exposed  height_ext");
    for h in [12.0, 21.0, 23.5].map(|h| DAY * 24.0 + h) {
        let f = frame(&book, &state, h);
        let clock = SkyClock::at(0.0, h, &sky.calendar);
        let s = sky.evaluate(&clock, &f.sky);
        let tint = s.ambient_tint.map_or(Vec3::new(0.55, 0.65, 1.0), |t| t.max(Vec3::ZERO) * reference);
        let view_ambient = tint * s.ambient;
        let fog = f.fog.albedo * view_ambient * f.fog.ambient;
        let stone = 0.4 * view_ambient;
        let exposure = 2f32.powf(-s.ev100) / 1.2;
        println!(
            "  {:5.2} {:7.1} {:7.2} {:6.2}  ({:5.2}, {:5.2}, {:5.2})  {:12.3} {:10.3} {:12.4} {:14.4} {:11.5}",
            h.rem_euclid(24.0),
            s.sun.elevation.to_degrees(),
            s.ev100,
            s.ambient,
            view_ambient.x,
            view_ambient.y,
            view_ambient.z,
            luminance(fog),
            luminance(stone),
            luminance(fog) * exposure,
            luminance(stone) * exposure,
            f.fog.height.density,
        );
    }
}

/// N2: the night sky glow (`sf_sky::NightGlow`, added to the atmosphere's sky-view LUT) for Clear
/// and Overcast at 21:00, 23:30 and 3:00. Printed: the floor (nits) and its luminance, the glow's
/// zenith and horizon luminance (nits, and exposed: x 2^-EV / 1.2), its hue (weather's `Sky Night
/// Color`, linear, luminance 1), and the extra fill an upward surface gets from the glow through the
/// environment map (irradiance / pi over the floor's luminance). Clouds then dim the glow on the GPU
/// (cloud cover printed); not modelled here.
#[test]
#[ignore = "measurement: N2 night sky glow zenith/horizon luminance"]
fn print_night_sky_glow() {
    use sf_weather::sf_sky::{ExposureRange, KeyCurve, luminance};
    use glam::Vec3;
    // `mw_assets::LightCalibration` and `sf_sky_bevy::SkyRenderOptions::ambient_color` (copied).
    let night_ambient_nits = 4.0;
    let sky = SkySettings {
        exposure_range: ExposureRange { min_ev100: 3.5, ..ExposureRange::default() },
        ambient: KeyCurve::new(vec![(-12.0, night_ambient_nits), (-4.0, 0.0)]),
        ..SkySettings::default()
    };
    let default_floor = Vec3::new(0.55, 0.65, 1.0);
    let reference = luminance(default_floor);
    let glow = sky.night_glow;
    println!("ratio {} horizon {}: luminance in nits; exposed = x 2^-EV / 1.2", glow.ratio, glow.horizon);
    println!("  weather   hour  sun_deg  ev100  floor_lum  zenith  horizon  elev10  zenith_exp  horizon_exp  env_fill  cover  hue(rgb)");
    let book = book();
    for ty in [WeatherType::Clear, WeatherType::Overcast] {
        let state = launch_forced(&book, ty, DAY * 24.0 + 10.0);
        for h in [21.0, 23.5, 27.0].map(|h| DAY * 24.0 + h) {
            let f = frame(&book, &state, h);
            let clock = SkyClock::at(0.0, h, &sky.calendar);
            let s = sky.evaluate(&clock, &f.sky);
            let floor_rgb = s.ambient_tint.map_or(default_floor, |t| t.max(Vec3::ZERO) * reference);
            let floor = s.ambient * luminance(floor_rgb);
            let zenith = glow.zenith(floor, s.night_sky_tint.unwrap_or(floor_rgb));
            let z = luminance(zenith);
            let hue = zenith / z.max(1e-9);
            let exposure = 2f32.powf(-s.ev100) / 1.2;
            println!(
                "  {:9} {:5.2} {:7.1} {:6.2} {:9.3} {:7.3} {:8.3} {:7.3} {:11.4} {:12.4} {:8.2}% {:6.2}  ({:.3}, {:.3}, {:.3})",
                format!("{ty:?}"),
                h.rem_euclid(24.0),
                s.sun.elevation.to_degrees(),
                s.ev100,
                floor,
                z,
                z * glow.profile(0.0),
                z * glow.profile(10f32.to_radians().sin()),
                z * exposure,
                z * glow.profile(0.0) * exposure,
                if floor > 0.0 { z * glow.ground_factor() / floor * 100.0 } else { 0.0 },
                f.sky.cloud_cover,
                hue.x,
                hue.y,
                hue.z,
            );
        }
    }
}

/// N3: the moonlight at full, three-quarter waning, half (the scene's default `--day 172`)
/// and new moon, clear and overcast: the light's lux above the atmosphere and at the ground,
/// its share of the floor, and exposed values (x 2^-EV / 1.2) for a stone albedo of 0.3 on the
/// shadowed side, a face square to the moon and the flat ground. The old moonlight (the
/// disks' own 0.8 + 0.2 lx) for comparison. Each row is the night's hour of most moonlight.
#[test]
#[ignore = "measurement: N3 moonlight lux and on-screen values by phase and weather"]
fn print_moonlight() {
    use glam::Vec3;
    use sf_weather::sf_sky::{ExposureRange, KeyCurve, Moon, MoonPath, Moonlight, luminance};
    // `mw_assets::LightCalibration` (night floor 4 nits, EV floor 3.5, FULL_MOONLIGHT_FLOOR_RATIO
    // 2.5), `sf_sky_bevy::SkyRenderOptions::ambient_color` and `mw_environment_bevy::night_sky`
    // (Masser, Secunda) copied.
    let night_ambient_nits = 4.0;
    let full_lux = 2.5 * std::f32::consts::PI * night_ambient_nits;
    let moon = |name: &str, size: f32, tilt: f32, inc: f32, ha: f32, lux: f32, tint: [f32; 3]| Moon {
        name: name.into(),
        angular_size: (size * 0.1f32).to_radians(),
        phase_period_days: 24.0,
        phase_at_epoch: 0.5,
        phase_steps: 8,
        path: MoonPath::Circuit { circuit_days: 1.0 + inc / 24.0, hour_angle_at_epoch_deg: ha, tilt_deg: tilt, declination_deg: 0.0 },
        illuminance: lux,
        tint,
        ..Moon::default()
    };
    let mut sky = SkySettings {
        exposure_range: ExposureRange { min_ev100: 3.5, ..ExposureRange::default() },
        ambient: KeyCurve::new(vec![(-12.0, night_ambient_nits), (-4.0, 0.0)]),
        moons: vec![
            moon("masser", 94.0, 35.0, 1.0, 0.0, 0.8, [1.0, 0.9, 0.82]),
            moon("secunda", 40.0, 50.0, 1.2, -30.0, 0.2, [0.9, 0.93, 1.0]),
        ],
        moonlight: Moonlight { full_lux, artistic: true },
        ..SkySettings::default()
    };
    let default_floor = Vec3::new(0.55, 0.65, 1.0);
    let reference = luminance(default_floor);
    let albedo = 0.3;
    println!("full_lux {full_lux:.2} lx; lum = nits, exp = exposed (1 = white before tonemapping)");
    println!("  weather   phase        day  hour  elev  light_lx ground_lx old_lx  floor  face/floor  shadow_exp face_exp ground_exp  hue(rgb)");
    let book = book();
    for ty in [WeatherType::Clear, WeatherType::Overcast] {
        let state = launch_forced(&book, ty, DAY * 24.0 + 10.0);
        let weather = frame(&book, &state, DAY * 24.0 + 23.5).sky;
        // The phase at the night's hours (snapped to 8 steps); 172 is the scene's default day.
        for (phase, day) in [("full", 167.0), ("three_wan", 170.0), ("half_wan", 172.0), ("new", 179.0)] {
            let at = |sky: &SkySettings, h: f64| sky.evaluate(&SkyClock::at(day, h, &sky.calendar), &weather);
            // Full night only (the floor at its plateau, the sun below -12°).
            let night = (0..40).map(|i| 20.0 + f64::from(i) * 0.25).filter(|h| at(&sky, *h).celestial.sun.elevation_deg() < -12.0);
            let hour = night.max_by(|a, b| at(&sky, *a).moon.ground_illuminance().total_cmp(&at(&sky, *b).moon.ground_illuminance())).unwrap();
            let s = at(&sky, hour);
            sky.moonlight.artistic = false;
            let old = at(&sky, hour).moon.ground_illuminance();
            sky.moonlight.artistic = true;
            let floor_rgb = s.ambient_tint.map_or(default_floor, |t| t.max(Vec3::ZERO) * reference);
            let floor = s.ambient * luminance(floor_rgb);
            let ground = s.moon.ground_illuminance();
            let exposure = 2f32.powf(-s.ev100) / 1.2;
            let face = ground / std::f32::consts::PI;
            let flat = face * s.moon.elevation.sin().max(0.0);
            println!(
                "  {:9} {:10} {:5.0} {:5.2} {:5.1} {:8.2} {:9.2} {:6.3} {:6.2} {:10.2} {:11.4} {:8.4} {:10.4}  ({:.2}, {:.2}, {:.2})",
                format!("{ty:?}"),
                phase,
                day,
                hour.rem_euclid(24.0),
                s.moon.elevation.to_degrees(),
                s.moon.illuminance,
                ground,
                old,
                floor,
                if floor > 0.0 { face / floor } else { 0.0 },
                albedo * floor * exposure,
                albedo * (floor + face) * exposure,
                albedo * (floor + flat) * exposure,
                s.moon.tint.x,
                s.moon.tint.y,
                s.moon.tint.z,
            );
        }
    }
}
