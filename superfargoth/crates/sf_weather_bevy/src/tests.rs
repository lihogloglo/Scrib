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

/// Two presets with clouds and fog: a clear sky and an overcast one.
fn sky_book() -> WeatherBook {
    use sf_weather::{CloudTerms, FogTerms};
    let clear = WeatherPreset {
        clouds: CloudTerms {
            coverage: 0.2,
            speed: 10.0,
            ..CloudTerms::default()
        },
        ..WeatherPreset::default()
    };
    let overcast = WeatherPreset {
        clouds: CloudTerms {
            coverage: 0.95,
            speed: 15.0,
            ..CloudTerms::default()
        },
        sky: SkyTerms {
            overcast_dimming: 0.7,
            ..SkyTerms::default()
        },
        fog: FogTerms {
            height_density: 1e-3,
            bank_density: 0.02,
            albedo: glam::Vec3::new(0.5, 0.4, 0.3),
            ..FogTerms::default()
        },
        ..WeatherPreset::default()
    };
    let climate = Climate {
        weights: vec![(id("clear"), 1.0)],
        ..Climate::default()
    };
    WeatherBook::new(
        [(id("clear"), clear), (id("overcast"), overcast)],
        [(id("land"), climate)],
        [],
    )
}

fn sky_app() -> App {
    let mut app = app();
    app.insert_resource(WeatherRecords(Arc::new(sky_book())))
        .insert_resource(sf_atmos_bevy::Clouds {
            settings: sf_atmos_bevy::sf_atmos::CloudSettings::default(),
            enabled: true,
            shadows: false,
            glows: [bevy_math::Vec4::ZERO; sf_atmos_bevy::GLOWS],
            temporal: sf_atmos_bevy::sf_atmos::CloudTemporal::default(),
        })
        .insert_resource(sf_atmos_bevy::Fog {
            settings: FogSettings::default(),
            enabled: true,
            froxels: None,
            view_ambient: sf_atmos_bevy::sf_atmos::VIEW_AMBIENT_IN_FOG,
        });
    site(&mut app);
    app
}

fn set_hours(app: &mut App, hours: f64) {
    let per_day = f64::from(app.world().resource::<SkyConfig>().0.calendar.hours_per_day);
    app.world_mut().resource_mut::<SkyTime>().0.days = hours / per_day;
    app.update();
}

#[test]
fn frame_drives_clouds_and_fog() {
    let mut app = sky_app();
    app.world_mut().resource_mut::<WorldWeather>().0.force(
        &id("land"),
        &id("overcast"),
        -100.0,
        Some(0.0),
        Some(1e4),
    );
    app.world_mut().resource_mut::<WeatherOverrides>().fog_scale = Some(2.0);
    set_hours(&mut app, 10.0);
    let w = app.world();
    let frame = &w.resource::<CurrentWeather>().frame;
    let clouds = w.resource::<sf_atmos_bevy::Clouds>().settings.weather;
    assert_eq!(clouds, frame.clouds);
    assert!((clouds.coverage - 0.95).abs() < 1e-5);
    assert!((clouds.wind.length() - 15.0).abs() < 1e-3);
    let fog = w.resource::<sf_atmos_bevy::Fog>().settings;
    assert!((fog.height.density - 2e-3).abs() < 1e-7);
    assert!((fog.banks.layer.density - 0.04).abs() < 1e-6);
    assert_eq!(fog.albedo, glam::Vec3::new(0.5, 0.4, 0.3));
    // The renderer's look fields stay.
    assert_eq!(fog.far, FogSettings::default().far);
    let sky = &w.resource::<sf_sky_bevy::WeatherState>().0;
    assert!((sky.cloud_cover - clouds.mean_cover()).abs() < 1e-6);
    assert!((sky.overcast_dimming - 0.7).abs() < 1e-6);

    // A pinned genus keeps the weather's wind.
    let pin = CloudWeather::preset(sf_atmos_bevy::sf_atmos::CloudPreset::Cirrus);
    app.world_mut().resource_mut::<WeatherOverrides>().clouds = Some(pin);
    set_hours(&mut app, 10.1);
    let clouds = app.world().resource::<sf_atmos_bevy::Clouds>().settings.weather;
    assert_eq!(clouds.coverage, pin.coverage);
    assert!((clouds.wind.length() - 15.0).abs() < 1e-3);
}

/// A forced change clear -> overcast over 2 h glides (no step larger than a
/// smooth ramp's) and the clouds lead the fog (weather.md "Transitions").
#[test]
fn transition_glides_clouds_lead() {
    let mut app = sky_app();
    set_hours(&mut app, 10.0);
    let start = 10.0;
    app.world_mut().resource_mut::<WorldWeather>().0.force(
        &id("land"),
        &id("overcast"),
        start,
        Some(2.0),
        Some(100.0),
    );
    let steps = 400;
    let (mut last_cov, mut last_fog) = (None::<f32>, None::<f32>);
    let (mut max_cloud_step, mut max_fog_step) = (0.0f32, 0.0f32);
    let mut cloud_half = None;
    let mut fog_half = None;
    for i in 0..=steps {
        let h = start + 2.0 * f64::from(i) / f64::from(steps);
        set_hours(&mut app, h);
        let w = app.world();
        let cov = w.resource::<sf_atmos_bevy::Clouds>().settings.weather.coverage;
        let fog = w.resource::<sf_atmos_bevy::Fog>().settings.height.density;
        if let Some(c) = last_cov {
            max_cloud_step = max_cloud_step.max((cov - c).abs());
        }
        if let Some(f) = last_fog {
            max_fog_step = max_fog_step.max((fog - f).abs());
        }
        last_cov = Some(cov);
        last_fog = Some(fog);
        if cloud_half.is_none() && cov >= 0.2 + 0.5 * 0.75 {
            cloud_half = Some(i);
        }
        if fog_half.is_none() && fog >= 0.5e-3 {
            fog_half = Some(i);
        }
    }
    assert!((last_cov.unwrap() - 0.95).abs() < 1e-4, "{last_cov:?}");
    assert!((last_fog.unwrap() - 1e-3).abs() < 1e-7, "{last_fog:?}");
    // A smoothstep over 60 % of 400 steps rises at most 1.5/240 per step.
    assert!(max_cloud_step < 0.75 * 1.6 / 240.0, "cloud step {max_cloud_step}");
    assert!(max_fog_step < 1e-3 * 1.6 / 240.0, "fog step {max_fog_step}");
    assert!(cloud_half.unwrap() < fog_half.unwrap(), "{cloud_half:?} {fog_half:?}");
}

fn storm_app() -> App {
    let storm = WeatherPreset {
        precipitation: vec![sf_weather::PrecipTerm { kind: id("rain"), intensity: 0.8 }],
        cues: vec![sf_weather::Cue { sound: id("rain_loop"), volume: 1.0 }],
        lightning: Some(sf_weather::Lightning {
            per_hour: 100.0,
            thunder: vec![id("thunder")],
            ..sf_weather::Lightning::default()
        }),
        ..WeatherPreset::default()
    };
    let climate = Climate {
        weights: vec![(id("storm"), 1.0)],
        ..Climate::default()
    };
    let book = Arc::new(WeatherBook::new(
        [(id("storm"), storm)],
        [(id("land"), climate)],
        [(id("rain"), sf_weather::PrecipKind::default())],
    ));
    let mut app = App::new();
    app.insert_resource(SkyTime(SkyClock::default()))
        .init_resource::<SkyConfig>()
        .init_resource::<sf_sky_bevy::WeatherState>()
        .init_resource::<sf_precip_bevy::Precip>()
        .add_plugins(WeatherPlugin { book, state: WeatherState::new(3) });
    site(&mut app);
    app
}

#[test]
fn frame_drives_precip_cues_and_lightning() {
    let mut app = storm_app();
    app.update();
    let p = app.world().resource::<sf_precip_bevy::Precip>().clone();
    let slot = p.slots[0].expect("rain falls");
    assert_eq!(slot.shape, sf_precip_bevy::PrecipShape::Streak);
    assert!((slot.intensity - 0.8).abs() < 1e-5, "{}", slot.intensity);
    assert!(p.slots[1].is_none());
    assert!(p.brightness > 0.5, "noon without sky: day light");
    let loops: Vec<_> = app
        .world_mut()
        .resource_mut::<Messages<WeatherSound>>()
        .drain()
        .collect();
    assert_eq!(loops, vec![WeatherSound::Loop { sound: id("rain_loop"), volume: 1.0, region: false }]);
    // ~100 strikes per game hour: step the clock until one fires.
    let mut struck = false;
    for _ in 0..400 {
        app.world_mut().resource_mut::<SkyTime>().0.days += 0.05 / 24.0;
        app.update();
        if app.world().resource::<LightningFlash>().strike.is_some() {
            struck = true;
            break;
        }
    }
    assert!(struck, "no strike");
    let flash = app.world().resource::<LightningFlash>();
    assert!(flash.level > 0.5 && flash.age_s == 0.0, "{flash:?}");
    // No real clock in the test: the thunder is due now (delay counted from 0
    // only when real time runs), so it waits; the light spawns next frame.
    app.update();
    let lights = app
        .world_mut()
        .query_filtered::<&bevy_light::DirectionalLight, With<LightningLight>>()
        .iter(app.world())
        .count();
    assert_eq!(lights, 1);
}

#[test]
fn slots_keep_their_kind_through_a_crossfade() {
    use sf_weather::PrecipIx;
    let mut a = [None; sf_precip_bevy::SLOTS];
    assign_slots(&mut a, &[(PrecipIx(1), 1.0)]);
    assert_eq!(a, [Some(PrecipIx(1)), None]);
    assign_slots(&mut a, &[(PrecipIx(0), 0.9), (PrecipIx(1), 0.2)]);
    assert_eq!(a, [Some(PrecipIx(1)), Some(PrecipIx(0))], "rain stays in its box");
    assign_slots(&mut a, &[(PrecipIx(0), 1.0)]);
    assert_eq!(a, [None, Some(PrecipIx(0))]);
    assert!(precip_brightness(-0.5, 0.0, 0.0) < 0.05);
    assert!(precip_brightness(0.5, 1.0, 0.0) < precip_brightness(0.5, 0.0, 0.0));
    assert!(lightning_direction(0.0).y < -0.5, "light shines down");
}

#[test]
fn only_near_strikes_light_the_land() {
    assert!((near_factor(1000.0) - 1.0).abs() < 1e-6);
    assert!(near_factor(6000.0) > 0.0 && near_factor(6000.0) < 1.0);
    assert!(near_factor(12_000.0).abs() < 1e-6);
}

/// Runs a console line on the app's world; returns the lines it logged.
fn console(app: &mut App, line: &str) -> Result<Vec<String>, String> {
    let world = app.world_mut();
    world.resource_scope(|world, mut c: Mut<sf_console_bevy::DevConsole>| {
        let before = c.log().lines().last().map_or(0, |l| l.seq);
        let r = c.exec(world, line).map_err(|e| e.to_string());
        let out = c.log().lines().filter(|l| l.seq > before).map(|l| l.text.clone()).collect();
        r.map(|()| out)
    })
}

#[test]
fn surface_steps_on_game_time_and_the_console_drives_it() {
    let mut app = app();
    let land = app.world().resource::<WeatherRecords>().0.climate(&id("land")).unwrap();
    let raster = ClimateRaster::new(DVec2::ZERO, 128.0, 2, 2, &[Some(land); 4], 256.0).unwrap();
    app.insert_resource(WorldWeatherMap::new(raster).with_worldspace(id("ws")));
    set_hours(&mut app, 100.0);
    app.update();
    {
        let s = app.world().resource::<WorldSurface>();
        assert!((s.state.t_h - 100.0).abs() < 1e-9, "stepped to the clock: {}", s.state.t_h);
        assert_eq!(s.grid().map(sf_surface::CellGrid::len), Some(64));
    }
    console(&mut app, "surface set wet 0.5 land").unwrap();
    assert_eq!(app.world().resource::<WorldSurface>().state.climate(land).wet, 0.5);
    // Clear weather, no drying rate: 1 / dry_h (0.5 per hour) dries it in one hour.
    set_hours(&mut app, 101.0);
    app.update();
    assert!(app.world().resource::<WorldSurface>().state.climate(land).wet < 1e-5);
    // Cover on every cell the climate weighs; it decays (ash 4x slower) when nothing falls.
    console(&mut app, "surface set ash 1 land").unwrap();
    assert_eq!(app.world().resource::<WorldSurface>().state.active_cells(), 64);
    site(&mut app);
    console(&mut app, "surface set puddle 0.25").unwrap();
    let dump = console(&mut app, "surface dump").unwrap();
    assert!(dump[0].starts_with("surface t 101.00 h") && dump[0].contains("64 active cells"), "{dump:?}");
    assert!(dump.iter().any(|l| l.contains("land wet 0.000 puddle 0.250")), "{dump:?}");
    assert!(dump.iter().any(|l| l.starts_with("cells ws: 64 active (0 snow, 64 ash)")), "{dump:?}");
    console(&mut app, "surface clear").unwrap();
    assert_eq!(app.world().resource::<WorldSurface>().state.active_cells(), 0);
    assert!(console(&mut app, "surface set wet 0.5 nowhere").is_err());
    // Every change the GPU copy follows bumps the generation; the debug view is a resource.
    let g = app.world().resource::<WorldSurface>().generation();
    console(&mut app, "surface set wet 0.25 land").unwrap();
    assert_eq!(app.world().resource::<WorldSurface>().generation(), g + 1);
    console(&mut app, "surface debug exposure").unwrap();
    assert_eq!(app.world().resource::<SurfaceDebug>().0, sf_surface::view::DebugView::Exposure);
    assert!(console(&mut app, "surface debug wet").is_err());
    console(&mut app, "surface clear").unwrap();
    assert_eq!(app.world().resource::<WorldSurface>().generation(), g + 2);
    // A save round trip through the book.
    let book = app.world().resource::<WeatherRecords>().0.clone();
    let mut s = app.world_mut().resource_mut::<WorldSurface>();
    s.state.set_climate(land, sf_surface::Channel::Wet, 0.75);
    let save = s.save(&book);
    s.restore(None, &book);
    assert!(!s.state.started(), "a v1 save starts over");
    s.restore(Some(&save), &book);
    assert!((s.state.climate(land).wet - 0.75).abs() <= sf_surface::QUANTUM);
}

#[test]
fn capped_slot_follows_the_drift_formula() {
    let rain = sf_precip_bevy::PrecipSlot { fall_speed: 8.0, wind_follow: 0.8, ..sf_precip_bevy::PrecipSlot::default() };
    let wind = bevy_math::Vec2::new(3.0, 4.0);
    let s = capped_slot(rain, wind, 20.0);
    let want = sf_weather::drop_drift(5.0 * 0.8, 8.0, 20.0);
    assert!((s.wind_follow * 5.0 - want).abs() < 1e-5, "{} vs {want}", s.wind_follow * 5.0);
    assert!(sf_weather::tilt_deg(s.wind_follow * 5.0, 8.0) < 20.0);
    // Calm: nothing to rescale.
    assert_eq!(capped_slot(rain, bevy_math::Vec2::ZERO, 20.0), rain);
}

#[test]
fn precip_leans_in_the_smoothed_capped_wind_and_flips_back() {
    let mut app = storm_app();
    app.update();
    let target = app.world().resource::<CurrentWeather>().frame.wind().velocity;
    let cap = app.world().resource::<CurrentWeather>().frame.precip_tilt_max_deg;
    assert_eq!(cap, sf_weather::UNCAPPED_DEG, "default presets don't cap");
    let p = app.world().resource::<sf_precip_bevy::Precip>().clone();
    // First frame snaps to the frame's wind (no wind field here: profile 1).
    assert!(p.wind.distance(target) < 1e-4, "{:?} vs {target:?}", p.wind);
    let slot = p.slots[0].expect("rain falls");
    let want = sf_weather::drop_drift(target.length() * 0.8, slot.fall_speed, cap);
    assert!((slot.wind_follow * p.wind.length() - want).abs() < 1e-4);
    // A/B off: the frame's wind and the kind's own share.
    app.world_mut().resource_mut::<PrecipWind>().slant_cap = false;
    app.update();
    let p = app.world().resource::<sf_precip_bevy::Precip>().clone();
    assert_eq!(p.wind, app.world().resource::<CurrentWeather>().frame.wind().velocity);
    assert!((p.slots[0].expect("rain falls").wind_follow - 0.8).abs() < 1e-6);
}
