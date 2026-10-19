#![allow(clippy::float_cmp)]

use std::time::Instant;

use bevy_app::App;
use sf_sky_bevy::sf_sky::SkyClock;
use sf_weather::sf_wind::{Wind, WindSource};
use sf_weather::{Climate, SkyTerms, WeatherPreset, WindTerms};

use super::*;

fn id(s: &str) -> sf_records::RecordId {
    sf_records::RecordId::new(s)
}

fn book() -> WeatherBook {
    let clear = WeatherPreset {
        sky: SkyTerms {
            haze: 20.0,
            ..SkyTerms::default()
        },
        wind: WindTerms {
            speed: [2.0, 4.0],
            ..WindTerms::default()
        },
        ..WeatherPreset::default()
    };
    let ash = WeatherPreset {
        sky: SkyTerms {
            haze: 24.0,
            dust: 4.0,
            ..SkyTerms::default()
        },
        wind: WindTerms {
            speed: [10.0, 12.0],
            storm: 1.0,
            ..WindTerms::default()
        },
        ..WeatherPreset::default()
    };
    let source = WindSource::radial(sf_coords::WorldPos::default(), 6.0, 3000.0);
    let climate = Climate {
        weights: vec![(id("clear"), 1.0)],
        storm_source: Some(source),
        ..Climate::default()
    };
    WeatherBook::new(
        [(id("clear"), clear), (id("ash"), ash)],
        [(id("land"), climate)],
        [],
    )
}

fn app() -> App {
    let book = Arc::new(book());
    let mut app = App::new();
    app.insert_resource(SkyTime(SkyClock::default()))
        .init_resource::<SkyConfig>()
        .init_resource::<sf_sky_bevy::WeatherState>()
        .insert_resource(WindField::new(
            Wind::default(),
            sf_weather::sf_wind::WindSettings::default(),
        ))
        .add_plugins(WeatherPlugin {
            book,
            state: WeatherState::new(7),
        });
    app
}

fn site(app: &mut App) {
    let land = app
        .world()
        .resource::<WeatherRecords>()
        .0
        .climate(&id("land"))
        .unwrap();
    app.world_mut()
        .resource_mut::<WeatherSite>()
        .climates
        .push((land, 1.0));
}

#[test]
fn no_site_keeps_the_sky() {
    let mut app = app();
    app.update();
    assert!(!app.world().resource::<CurrentWeather>().valid);
    assert_eq!(
        app.world().resource::<sf_sky_bevy::WeatherState>().0.haze,
        sf_sky_bevy::sf_sky::Weather::default().haze
    );
}

#[test]
fn frame_drives_sky_and_wind() {
    let mut app = app();
    site(&mut app);
    app.update();
    let w = app.world();
    let current = w.resource::<CurrentWeather>();
    assert!(current.valid);
    assert_eq!(w.resource::<sf_sky_bevy::WeatherState>().0.haze, 20.0);
    let speed = w.resource::<WindField>().state.wind.speed();
    assert!((2.0..=4.0).contains(&speed), "{speed}");
    assert_eq!(w.resource::<WindField>().state.sources().count(), 0);

    app.world_mut().resource_mut::<WeatherOverrides>().haze = Some(5.0);
    app.update();
    assert_eq!(
        app.world().resource::<sf_sky_bevy::WeatherState>().0.haze,
        5.0
    );
}

#[test]
fn forced_storm_adds_one_source() {
    let mut app = app();
    site(&mut app);
    let now = 12.0;
    app.world_mut().resource_mut::<WorldWeather>().0.force(
        &id("land"),
        &id("ash"),
        now - 48.0,
        Some(0.0),
        Some(1000.0),
    );
    for _ in 0..3 {
        app.update();
    }
    let w = app.world();
    assert_eq!(w.resource::<sf_sky_bevy::WeatherState>().0.dust, 4.0);
    let sources: Vec<_> = w
        .resource::<WindField>()
        .state
        .sources()
        .map(|(_, s)| s.strength)
        .collect();
    assert_eq!(sources, vec![6.0]);
}

/// Gate (weather.md "WE3 as built"): evaluate + apply ≤ 0.05 ms per frame.
#[test]
fn cost_per_frame() {
    let mut app = app();
    site(&mut app);
    app.update();
    let n = 2000;
    let t0 = Instant::now();
    for i in 0..n {
        app.world_mut().resource_mut::<SkyTime>().0.days += 1e-4 * f64::from(i);
        app.update();
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0 / f64::from(n);
    let eval = app.world().resource::<WeatherStats>().mean_ms;
    println!("app.update {ms:.4} ms/frame, weather evaluate mean {eval:.5} ms");
    assert!(eval < 0.05, "{eval} ms");
}
