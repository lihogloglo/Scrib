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
