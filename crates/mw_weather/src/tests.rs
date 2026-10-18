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
