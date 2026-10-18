use super::*;
use sf_weather::WeatherBook;

fn region(id: &str, chances: [u8; 10]) -> Region {
    let [
        clear,
        cloudy,
        foggy,
        overcast,
        rain,
        thunder,
        ash,
        blight,
        snow,
        blizzard,
    ] = chances;
    Region {
        id: id.to_owned(),
        name: id.to_owned(),
        weather_chances: WeatherChances {
            clear,
            cloudy,
            foggy,
            overcast,
            rain,
            thunder,
            ash,
            blight,
            snow,
            blizzard,
        },
        ..Region::default()
    }
}

fn content_db(timescale: Option<f32>) -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Region>("REGN").unwrap();
    db.register::<GlobalVariable>("GLOB").unwrap();
    let esm = db.add_source("Test.esm", &[]);
    db.define(
        esm,
        RecordId::new("Ascadian Isles Region"),
        region("Ascadian Isles Region", [45, 30, 0, 10, 10, 5, 0, 0, 0, 0]),
    )
    .unwrap();
    db.define(
        esm,
        RecordId::new("Red Mountain Region"),
        region("Red Mountain Region", [0, 0, 0, 0, 0, 0, 70, 30, 0, 0]),
    )
    .unwrap();
    db.define(
        esm,
        RecordId::new("Moesring Mountains Region"),
        region(
            "Moesring Mountains Region",
            [0, 0, 0, 0, 0, 0, 0, 0, 60, 40],
        ),
    )
    .unwrap();
    db.define(
        esm,
        RecordId::new("Empty Region"),
        region("Empty Region", [0; 10]),
    )
    .unwrap();
    if let Some(t) = timescale {
        let global = GlobalVariable {
            id: "timescale".into(),
            value: GlobalValue::Float(t),
            ..GlobalVariable::default()
        };
        db.define(esm, RecordId::new("timescale"), global).unwrap();
    }
    db
}

fn climate_of<'a>(w: &'a MwWeather, region: &str) -> &'a Climate {
    &w.climates
        .iter()
        .find(|(id, _)| *id == climate_id(region))
        .unwrap()
        .1
}

fn preset_of(w: &MwWeather, ty: WeatherType) -> &WeatherPreset {
    &w.presets[ty.index()].1
}

#[test]
fn regn_weights_map_to_presets_in_index_order() {
    let w = MwWeather::build(&content_db(None), None);
    assert_eq!(w.climates.len(), 4);
    let ids = |c: &Climate| {
        c.weights
            .iter()
            .map(|(id, x)| (id.as_str().to_owned(), *x))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        ids(climate_of(&w, "ascadian isles region")),
        [
            ("mw_weather/clear".to_owned(), 45.0),
            ("mw_weather/cloudy".to_owned(), 30.0),
            ("mw_weather/overcast".to_owned(), 10.0),
            ("mw_weather/rain".to_owned(), 10.0),
            ("mw_weather/thunderstorm".to_owned(), 5.0),
        ]
    );
    assert_eq!(
        ids(climate_of(&w, "Red Mountain Region")),
        [
            ("mw_weather/ashstorm".to_owned(), 70.0),
            ("mw_weather/blight".to_owned(), 30.0)
        ]
    );
    // Bloodmoon's two extra chances.
    assert_eq!(
        ids(climate_of(&w, "Moesring Mountains Region")),
        [
            ("mw_weather/snow".to_owned(), 60.0),
            ("mw_weather/blizzard".to_owned(), 40.0)
        ]
    );
    assert_eq!(
        ids(climate_of(&w, "Empty Region")),
        [("mw_weather/clear".to_owned(), 1.0)]
    );
    for ty in WeatherType::ALL {
        assert_eq!(WeatherType::from_index(ty.index()), Some(ty));
        assert_eq!(w.presets[ty.index()].0, ty.preset_id());
    }
}

#[test]
fn vanilla_values_without_an_ini() {
    let w = MwWeather::build(&content_db(None), None);
    assert!((w.timescale - 30.0).abs() < 1e-6);
    let ai = climate_of(&w, "Ascadian Isles Region");
    assert_eq!(ai.change_interval_hours, (15.0, 25.0));
    // 1 / 0.015 real s = 66.7 s = 0.556 game h at timescale 30, mostly.
    assert!(
        ai.transition_hours > 0.5 && ai.transition_hours < 0.56,
        "{}",
        ai.transition_hours
    );

    let rain = preset_of(&w, WeatherType::Rain);
    assert_eq!(rain.precipitation.len(), 1);
    assert_eq!(rain.precipitation[0].kind, Precip::Rain.id());
    assert!((rain.precipitation[0].intensity - 450.0 / 650.0).abs() < 1e-6);
    assert!((rain.transition.precipitation.start - 0.6).abs() < 1e-6);
    assert_eq!(
        rain.cues
            .iter()
            .map(|c| c.sound.as_str())
            .collect::<Vec<_>>(),
        ["rain"]
    );
    assert!(rain.lightning.is_none());

    let storm = preset_of(&w, WeatherType::Thunderstorm);
    assert!((storm.precipitation[0].intensity - 1.0).abs() < 1e-6);
    let lightning = storm.lightning.as_ref().unwrap();
    assert!(
        (lightning.per_hour - 0.8).abs() < 1e-6,
        "{}",
        lightning.per_hour
    );
    assert_eq!(
        lightning
            .thunder
            .iter()
            .map(RecordId::as_str)
            .collect::<Vec<_>>(),
        ["thunder0", "thunder1", "thunder2", "thunder3"]
    );

    let ash = preset_of(&w, WeatherType::Ashstorm);
    assert_eq!(ash.precipitation[0].kind, Precip::Ash.id());
    assert_eq!(ash.cues[0].sound, RecordId::new("ashstorm"));
    assert!((ash.transition.precipitation.start - 0.7).abs() < 1e-6);
    assert!(ash.surface.ash > 0.0);
    let blight = preset_of(&w, WeatherType::Blight);
    assert_eq!(blight.tags, [(RecordId::new(BLIGHT_EXPOSURE_TAG), 0.1)]);
    assert_eq!(
        preset_of(&w, WeatherType::Snow).precipitation[0].kind,
        Precip::Snow.id()
    );
    assert!((preset_of(&w, WeatherType::Snow).precipitation[0].intensity - 0.5).abs() < 1e-6);
    assert_eq!(
        preset_of(&w, WeatherType::Blizzard).cues[0].sound,
        RecordId::new("BM Blizzard")
    );
    let clear = preset_of(&w, WeatherType::Clear);
    assert!(clear.precipitation.is_empty() && clear.cues.is_empty() && clear.surface.drying > 0.0);

    let rain_kind = &w
        .precip_kinds
        .iter()
        .find(|(id, _)| *id == Precip::Rain.id())
        .unwrap()
        .1;
    let snow_kind = &w
        .precip_kinds
        .iter()
        .find(|(id, _)| *id == Precip::Snow.id())
        .unwrap()
        .1;
    assert!((rain_kind.fall_speed - 575.0 * 0.9144 / 64.0).abs() < 1e-4);
    assert!(
        snow_kind.fall_speed < rain_kind.fall_speed
            && snow_kind.accumulates == SurfaceChannel::Snow
    );
}

const INI: &[u8] = b"[Weather]\r\nHours Between Weather Changes=10\r\nPrecip Gravity=oops\r\n\
    [Weather Thunderstorm]\r\nThunder Frequency=.8\r\nThunder Sound ID 2=None\r\nMax Raindrops=900\r\n\
    [Weather Rain]\r\nRain Loop Sound ID=None\r\nRain Threshold=0.3\r\n\
    [Weather Clear]\r\nAmbient Loop Sound ID=Birds\r\nTransition Delta=0.5\r\n";

#[test]
fn ini_values_override_vanilla_and_missing_keys_fall_back() {
    let ini = MorrowindIni::parse(INI);
    let w = MwWeather::build(&content_db(None), Some(&ini));
    assert_eq!(
        climate_of(&w, "Red Mountain Region").change_interval_hours,
        (7.5, 12.5)
    );
    let storm = preset_of(&w, WeatherType::Thunderstorm);
    let lightning = storm.lightning.as_ref().unwrap();
    assert!((lightning.per_hour - 1.6).abs() < 1e-6);
    assert_eq!(lightning.thunder.len(), 3);
    let rain = preset_of(&w, WeatherType::Rain);
    assert!((rain.precipitation[0].intensity - 450.0 / 900.0).abs() < 1e-6);
    assert!((rain.transition.precipitation.start - 0.3).abs() < 1e-6);
    assert!(rain.cues.is_empty());
    assert_eq!(
        preset_of(&w, WeatherType::Clear).cues[0].sound,
        RecordId::new("birds")
    );
    // Unparsable `Precip Gravity` falls back to vanilla.
    assert_eq!(
        w.precip_kinds,
        MwWeather::build(&content_db(None), None).precip_kinds
    );
    // Clear's 2 real seconds pull the Ascadian Isles mean down.
    assert!(climate_of(&w, "Ascadian Isles Region").transition_hours < 0.4);
}

#[test]
fn real_time_rates_use_the_timescale() {
    assert!((lightning_per_game_hour(0.4, 30.0) - 0.8).abs() < 1e-6);
    assert!((transition_game_hours(0.015, 30.0).unwrap() - 30.0 / 54.0).abs() < 1e-6);
    assert_eq!(transition_game_hours(0.0, 30.0), None);
    let w = MwWeather::build(&content_db(Some(10.0)), None);
    assert!((w.timescale - 10.0).abs() < 1e-6);
    let per_hour = preset_of(&w, WeatherType::Thunderstorm)
        .lightning
        .as_ref()
        .unwrap()
        .per_hour;
    assert!((per_hour - 2.4).abs() < 1e-5, "{per_hour}");
}

#[test]
fn fingerprint_follows_the_values_read() {
    let db = content_db(None);
    let vanilla = MwWeather::build(&db, None).fingerprint;
    assert_eq!(vanilla, MwWeather::build(&db, None).fingerprint);
    let unrelated =
        MorrowindIni::parse(b"[General]\nFoo=1\n[Weather Clear]\nSky Day Color=1,2,3\n");
    assert_eq!(MwWeather::build(&db, Some(&unrelated)).fingerprint, vanilla);
    let edited = MorrowindIni::parse(b"[Weather]\nHours Between Weather Changes=21\n");
    assert_ne!(MwWeather::build(&db, Some(&edited)).fingerprint, vanilla);
    assert_ne!(
        MwWeather::build(&content_db(Some(20.0)), None).fingerprint,
        vanilla
    );
}

#[test]
fn records_land_in_a_synthetic_source_that_mods_patch() {
    let mut db = content_db(None);
    let w = MwWeather::build(&db, None);
    let source = w.define_into(&mut db, &[SourceId(0)]).unwrap();
    assert_eq!(db.source(source).name, SOURCE_NAME);

    let book = WeatherBook::from_db(&db);
    assert!(book.climate(&climate_id("Red Mountain Region")).is_some());
    for ty in WeatherType::ALL {
        assert!(book.preset(&ty.preset_id()).is_some(), "{ty:?}");
    }

    // A record mod on top patches one field.
    let mod_source = db.add_source("weather tweaks", &[source]);
    db.patch::<Climate>(mod_source, &climate_id("Red Mountain Region"), |c| {
        c.change_interval_hours = (1.0, 2.0);
    })
    .unwrap();
    let patched = db
        .get::<Climate>(&climate_id("red mountain region"))
        .unwrap();
    assert_eq!(patched.change_interval_hours, (1.0, 2.0));
    assert_eq!(patched.weights.len(), 2);
}

// -- we2b: visual fields match the app's scaffold tables -------------------------

/// The app's tables as of we2b (`apps/bevyrrowind/src/exterior.rs`:
/// `CLOUD_CHOICES` genus and speed, `SKY_*` haze/dust, `FogLook`
/// visibility, height, banks, `banks_day`), copied so WE3's switch-over can be
/// checked against them: (type, genus, cloud speed, haze, dust, visibility,
/// fog height, banks, banks by day).
type AppRow = (WeatherType, CloudPreset, f32, f32, f32, f32, f32, f32, f32);

#[rustfmt::skip]
const APP_TABLE: [AppRow; 10] = [
    (WeatherType::Clear, CloudPreset::Clear, 1.25, 20.0, 0.0, 40_000.0, 100.0, 0.0, 0.0),
    (WeatherType::Cloudy, CloudPreset::Fair, 2.0, 24.0, 0.0, 30_000.0, 120.0, 0.0, 0.0),
    (WeatherType::Foggy, CloudPreset::Altostratus, 1.25, 60.0, 0.0, 3_000.0, 150.0, 1.0, 0.35),
    (WeatherType::Overcast, CloudPreset::Overcast, 1.5, 32.0, 0.0, 8_000.0, 300.0, 0.0, 0.0),
    (WeatherType::Rain, CloudPreset::Overcast, 2.0, 48.0, 0.0, 3_000.0, 250.0, 0.25, 0.0),
    (WeatherType::Thunderstorm, CloudPreset::Storm, 3.0, 48.0, 0.0, 2_000.0, 250.0, 0.3, 0.0),
    (WeatherType::Ashstorm, CloudPreset::Stratocumulus, 7.0, 24.0, 4.0, 450.0, 500.0, 0.0, 0.0),
    (WeatherType::Blight, CloudPreset::Stratocumulus, 9.0, 24.0, 4.0, 450.0, 500.0, 0.0, 0.0),
    (WeatherType::Snow, CloudPreset::Overcast, 1.5, 48.0, 0.0, 3_000.0, 250.0, 0.25, 0.0),
    (WeatherType::Blizzard, CloudPreset::Overcast, 7.5, 80.0, 0.0, 300.0, 400.0, 0.0, 0.0),
];

use crate::visual::{BANK_DENSITY, BANK_FALLOFF, ground_fog, morning_haze};
use sf_weather::sf_atmos::fog::extinction_for_visibility;
use sf_weather::sf_atmos::{CloudPreset, CloudSettings};
use sf_weather::sf_sky::KeyCurve;

fn by_hour(c: &KeyCurve, h: f32) -> f32 {
    if c.keys().is_empty() {
        1.0
    } else {
        c.sample(h)
    }
}

fn close(a: f32, b: f32, what: &str) {
    assert!(
        (a - b).abs() <= 1e-2 * b.abs().max(1e-3),
        "{what}: {a} vs {b}"
    );
}

#[test]
#[allow(clippy::float_cmp)]
fn presets_match_the_app_tables() {
    let w = MwWeather::build(&content_db(None), None);
    for (ty, genus, speed, haze, dust, vis, height, banks, banks_day) in APP_TABLE {
        let p = preset_of(&w, ty);
        let name = ty.name();
        // Clouds: the app's `cloud_settings` (genus, halved heights, speed × 10).
        let mut app = CloudSettings::preset(genus).weather.scaled_heights(0.5);
        app.wind = app.wind.normalize_or(glam::Vec2::X) * speed * 10.0;
        let dir = app.wind.normalize_or(glam::Vec2::X);
        let mut got = p.clouds.to_weather(dir);
        assert!(
            got.wind.abs_diff_eq(app.wind, 1e-4),
            "{name} cloud wind {} vs {}",
            got.wind,
            app.wind
        );
        got.wind = app.wind;
        assert_eq!(got, app, "{name} clouds");
        // Sky and fog over the day: `drive_sky_weather` and `fog_settings`.
        for i in 0..=96 {
            #[allow(clippy::cast_precision_loss)]
            let h = i as f32 * 0.25;
            let g = ground_fog(h);
            close(
                p.sky.haze * by_hour(&p.sky.haze_by_hour, h),
                haze * morning_haze(h),
                &format!("{name} haze {h}"),
            );
            let fog = &p.fog;
            close(
                fog.height_density * by_hour(&fog.height_by_hour, h),
                extinction_for_visibility(vis) * (1.0 + 0.5 * g),
                &format!("{name} height fog {h}"),
            );
            close(
                fog.bank_density * by_hour(&fog.banks_by_hour, h),
                BANK_DENSITY * banks * (banks_day + (1.0 - banks_day) * g),
                &format!("{name} banks {h}"),
            );
        }
        assert_eq!(p.sky.dust, dust, "{name} dust");
        assert_eq!(
            (p.fog.height_falloff, p.fog.height_base),
            (height, 0.0),
            "{name} fog height"
        );
        assert_eq!(p.fog.bank_falloff, BANK_FALLOFF);
    }
    let blight = preset_of(&w, WeatherType::Blight);
    assert_eq!(blight.sky.dust_tint, glam::Vec3::new(0.6, 1.4, 1.9));
    assert_eq!(blight.fog.albedo, glam::Vec3::new(0.62, 0.42, 0.36));
    assert_eq!(
        preset_of(&w, WeatherType::Ashstorm).fog.albedo,
        glam::Vec3::new(0.55, 0.47, 0.38)
    );
    assert_eq!(
        preset_of(&w, WeatherType::Clear).fog.albedo,
        glam::Vec3::ONE
    );
}

#[test]
#[allow(clippy::float_cmp, clippy::cast_possible_truncation)]
fn wind_storm_and_flash_from_the_ini() {
    let w = MwWeather::build(&content_db(None), None);
    // `Wind Speed` 0..1 → 1..15 m/s mean, ±25 %.
    let speed = |ty| preset_of(&w, ty).wind.speed;
    assert_eq!(speed(WeatherType::Foggy), [0.75, 1.25]);
    let ash = speed(WeatherType::Ashstorm);
    close(f32::midpoint(ash[0], ash[1]), 1.0 + 14.0 * 0.8, "ash wind");
    for ty in WeatherType::ALL {
        let storm = matches!(ty, WeatherType::Ashstorm | WeatherType::Blight);
        assert_eq!(
            preset_of(&w, ty).wind.storm,
            if storm { 1.0 } else { 0.0 },
            "{}",
            ty.name()
        );
    }
    let thunder = preset_of(&w, WeatherType::Thunderstorm)
        .lightning
        .as_ref()
        .unwrap();
    close(thunder.fade_s, 0.25, "flash fade");

    // Ash/blight climates blow away from the summit; others have no source.
    assert!(
        climate_of(&w, "Ascadian Isles Region")
            .storm_source
            .is_none()
    );
    let source = climate_of(&w, "Red Mountain Region").storm_source.unwrap();
    let calm = sf_weather::sf_wind::Wind {
        velocity: glam::Vec2::ZERO,
        gustiness: 0.0,
        ..Default::default()
    };
    let mut state = sf_weather::sf_wind::WindState::new(calm);
    state.add_source(source).unwrap();
    let settings = sf_weather::sf_wind::WindSettings::default();
    state.advance(1.0, &settings);
    let center = source.center.to_f64();
    for dir in [
        glam::DVec3::X,
        glam::DVec3::NEG_X,
        glam::DVec3::Z,
        glam::DVec3::NEG_Z,
    ] {
        let pos = sf_coords::WorldPos::from_f64(center + dir * 1_000.0);
        let v = state.velocity_at(&settings, pos, 10.0);
        let out = glam::Vec2::new(v.x, v.z).dot(glam::Vec2::new(dir.x as f32, dir.z as f32));
        assert!(out > 0.5, "{dir}: {v}");
    }
}

#[test]
fn hour_curves_stay_small() {
    let c = visual::hour_curve(ground_fog);
    assert!(c.keys().len() < 60, "{}", c.keys().len());
    assert_eq!(visual::hour_curve(|_| 1.0).keys().len(), 2);
}
