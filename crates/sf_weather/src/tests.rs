// Exact float compares are intended: ramps return exact 0 and 1 outside their windows.
#![allow(clippy::float_cmp, clippy::many_single_char_names)]

use std::time::Instant;

use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId, Value};
use sf_wind::WindSource;

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn presets() -> Vec<(RecordId, WeatherPreset)> {
    let clear = WeatherPreset {
        clouds: CloudTerms { coverage: 0.1, ..CloudTerms::default() },
        sky: SkyTerms { haze: 1.0, ..SkyTerms::default() },
        wind: WindTerms { speed: [1.0, 3.0], ..WindTerms::default() },
        ..WeatherPreset::default()
    };
    let cloudy = WeatherPreset { clouds: CloudTerms { coverage: 0.5, ..CloudTerms::default() }, ..clear.clone() };
    let rain = WeatherPreset {
        clouds: CloudTerms { coverage: 0.95, wetness: 0.6, ..CloudTerms::default() },
        sky: SkyTerms { haze: 3.0, sun_glare: 0.0, ..SkyTerms::default() },
        fog: FogTerms { height_density: 1e-3, ..FogTerms::default() },
        precipitation: vec![PrecipTerm { kind: id("rain"), intensity: 1.0 }],
        wind: WindTerms { speed: [5.0, 8.0], gustiness: 0.6, ..WindTerms::default() },
        surface: SurfaceRates { wet: 1.0, puddle: 0.3, ..SurfaceRates::default() },
        cues: vec![Cue { sound: id("rain_loop"), volume: 1.0 }],
        tags: vec![(id("wet"), 1.0)],
        ..WeatherPreset::default()
    };
    let thunder = WeatherPreset {
        lightning: Some(Lightning { per_hour: 30.0, thunder: vec![id("thunder0"), id("thunder1")], ..Lightning::default() }),
        ..rain.clone()
    };
    let snow = WeatherPreset {
        clouds: CloudTerms { coverage: 0.9, ..CloudTerms::default() },
        precipitation: vec![PrecipTerm { kind: id("snow"), intensity: 0.8 }],
        surface: SurfaceRates { snow: 0.5, ..SurfaceRates::default() },
        wind: WindTerms { speed: [6.0, 9.0], storm: 1.0, ..WindTerms::default() },
        cues: vec![Cue { sound: id("blizzard_loop"), volume: 0.8 }],
        ..WeatherPreset::default()
    };
    vec![(id("clear"), clear), (id("cloudy"), cloudy), (id("rain"), rain), (id("thunder"), thunder), (id("snow"), snow)]
}

fn precip() -> Vec<(RecordId, PrecipKind)> {
    vec![
        (id("rain"), PrecipKind::default()),
        (id("snow"), PrecipKind { look: PrecipLook::Flake, fall_speed: 1.0, extinction: 8e-3, ..PrecipKind::default() }),
    ]
}

fn climate(weights: &[(&str, f32)]) -> Climate {
    Climate { weights: weights.iter().map(|(p, w)| (id(p), *w)).collect(), ..Climate::default() }
}

fn climates() -> Vec<(RecordId, Climate)> {
    let storm = WindSource::radial(WorldPos::from_f64(glam::DVec3::new(1000.0, 0.0, 0.0)), 10.0, 5_000.0);
    vec![
        (id("temperate"), climate(&[("clear", 40.0), ("cloudy", 30.0), ("rain", 20.0), ("thunder", 10.0)])),
        (id("north"), Climate { storm_source: Some(storm), ..climate(&[("snow", 60.0), ("clear", 40.0)]) }),
        (id("sunny"), climate(&[("clear", 1.0)])),
        (id("snowy"), climate(&[("snow", 1.0)])),
        (id("stormy"), climate(&[("thunder", 1.0)])),
    ]
}

fn book() -> WeatherBook {
    let b = WeatherBook::new(presets(), climates(), precip());
    assert!(b.warnings.is_empty(), "{:?}", b.warnings);
    b
}

fn input(climates: &[(ClimateIx, f32)], hours: f64) -> WeatherInput<'_> {
    WeatherInput { hours, prev_hours: hours, hour_of_day: 12.0, altitude: 0.0, climates }
}

fn intensity(frame: &WeatherFrame, book: &WeatherBook, kind: &str) -> f32 {
    frame.precip.iter().filter(|e| book.precip_id(e.kind).as_str() == kind).map(|e| e.intensity).sum()
}

#[test]
fn forecast_is_a_pure_function_of_seed_climate_index() {
    let b = book();
    let c = b.climate(&id("temperate")).unwrap();
    let s1 = WeatherState::new(7);
    let s2 = WeatherState::new(7);
    let other = WeatherState::new(8);
    let mut differs = false;
    let mut prev = s1.slot_start(&b, c, -50);
    for k in -49..400 {
        assert_eq!(s1.slot_preset(&b, c, k), s2.slot_preset(&b, c, k));
        differs |= s1.slot_preset(&b, c, k) != other.slot_preset(&b, c, k);
        let t = s1.slot_start(&b, c, k);
        let dt = t - prev;
        assert!((8.0 - 1e-9..=16.0 + 1e-9).contains(&dt), "interval {dt} outside the climate's range");
        assert_eq!(s1.slot_at(&b, c, t + 1e-4), k);
        prev = t;
    }
    assert!(differs, "the seed changes the forecast");

    // Load order doesn't matter: a book with the records in another order
    // forecasts the same presets.
    let mut ps = presets();
    ps.reverse();
    let mut cs = climates();
    cs.reverse();
    let b2 = WeatherBook::new(ps, cs, precip());
    let c2 = b2.climate(&id("temperate")).unwrap();
    for k in 0..200 {
        let a = s1.slot_preset(&b, c, k).map(|p| b.preset_id(p).clone());
        let z = s1.slot_preset(&b2, c2, k).map(|p| b2.preset_id(p).clone());
        assert_eq!(a, z);
    }

    // Forecast lists the same changes as the slots (no overrides).
    let f = s1.forecast(&b, c, 100.0, 10);
    assert_eq!(f.len(), 10);
    let k0 = s1.slot_at(&b, c, 100.0);
    for (i, e) in f.iter().enumerate() {
        let k = k0 + i64::try_from(i).unwrap();
        assert!((e.start - s1.slot_start(&b, c, k)).abs() < 1e-9);
        assert_eq!(Some(e.preset), s1.slot_preset(&b, c, k));
    }
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn draws_match_weights() {
    let b = book();
    let c = b.climate(&id("temperate")).unwrap();
    let s = WeatherState::new(42);
    let n = 8_000;
    let mut counts = std::collections::HashMap::new();
    for k in 0..n {
        *counts.entry(b.preset_id(s.slot_preset(&b, c, k).unwrap()).to_string()).or_insert(0usize) += 1;
    }
    for (p, w) in [("clear", 0.4), ("cloudy", 0.3), ("rain", 0.2), ("thunder", 0.1)] {
        let got = counts[p] as f64 / n as f64;
        let sigma = (w * (1.0 - w) / n as f64).sqrt();
        assert!((got - w).abs() < 4.0 * sigma, "{p}: {got} vs {w}");
    }
}

#[test]
fn save_restore_with_overrides() {
    let b = book();
    let c = b.climate(&id("temperate")).unwrap();
    let mut s = WeatherState::new(1234);
    s.force(&id("temperate"), &id("snow"), 50.0, Some(2.0), Some(10.0));
    s.set_weights(&id("temperate"), 100.0, vec![(id("rain"), 1.0)]);
    let text = ron::to_string(&s).unwrap();
    let restored: WeatherState = ron::from_str(&text).unwrap();
    assert_eq!(restored, s);

    let climates = [(c, 1.0)];
    for i in 0..600 {
        let t = f64::from(i) * 0.37;
        assert_eq!(evaluate(&b, &s, &input(&climates, t)), evaluate(&b, &restored, &input(&climates, t)));
    }
    // The force applies, then the forecast resumes.
    let snow = b.preset(&id("snow")).unwrap();
    assert_eq!(s.current(&b, c, 51.0), Some(snow));
    assert_eq!(s.current(&b, c, 59.9), Some(snow));
    assert_ne!(s.current(&b, c, 60.1), Some(snow));
    let f = s.forecast(&b, c, 40.0, 4);
    assert!(f.iter().any(|e| e.forced && (e.start - 50.0).abs() < 1e-9 && e.preset == snow), "{f:?}");
    assert!(f.iter().any(|e| !e.forced && (e.start - 60.0).abs() < 1e-9), "resumes at the force's end: {f:?}");
    // The weight edit applies to slots starting after it, not before.
    let rain = b.preset(&id("rain")).unwrap();
    let k = s.slot_at(&b, c, 100.0) + 1;
    for k in k..k + 50 {
        assert_eq!(s.slot_preset(&b, c, k), Some(rain));
    }
    let before = (s.slot_at(&b, c, 0.0)..s.slot_at(&b, c, 99.0)).filter(|&k| s.slot_preset(&b, c, k) != Some(rain)).count();
    assert!(before > 0, "earlier slots keep the record's weights");
    // Forced changes don't move the slot grid.
    assert_eq!(s.slot_start(&b, c, 30), WeatherState::new(1234).slot_start(&b, c, 30));
}

/// A sunny climate forced into `preset` at hour 10 for `hold` hours, with a
/// 2-hour transition (the way back uses the climate's 1.5 h).
fn forced(book: &WeatherBook, climate: &str, preset: &str, hold: f32) -> (WeatherState, ClimateIx) {
    let mut s = WeatherState::new(5);
    s.force(&id(climate), &id(preset), 10.0, Some(2.0), Some(hold));
    (s, book.climate(&id(climate)).unwrap())
}

fn weights(blend: &Blend, p: PresetIx) -> [f32; CHANNELS] {
    blend.iter().find(|e| e.0 == p).map_or([0.0; CHANNELS], |e| e.1)
}

#[test]
fn lead_lag_curves() {
    let b = book();
    let (s, c) = forced(&b, "sunny", "rain", 6.0);
    let rain = b.preset(&id("rain")).unwrap();
    let clear = b.preset(&id("clear")).unwrap();
    let at = |frac: f64| s.blend(&b, c, 10.0 + 2.0 * frac);
    // Arrival: clouds 0-60 %, wind 20-80 %, precipitation and fog 40-100 %.
    let w = weights(&at(0.3), rain);
    assert!((w[CLOUDS] - 0.5).abs() < 1e-4, "clouds half way at 30 %: {w:?}");
    assert!(w[WIND] > 0.0 && w[WIND] < 0.5);
    assert_eq!(w[PRECIP], 0.0, "no rain before 40 %");
    assert_eq!(w[FOG], 0.0);
    let w = weights(&at(0.6), rain);
    assert!((w[CLOUDS] - 1.0).abs() < 1e-6 && w[PRECIP] > 0.0 && w[PRECIP] < 1.0);
    assert_eq!(weights(&at(1.0), rain), [1.0; CHANNELS]);
    // Continuous weights sum to 1 throughout.
    for i in 0..=20 {
        let blend = at(f64::from(i) / 20.0);
        for ch in [CLOUDS, SKY, WIND, FOG] {
            let sum: f32 = blend.iter().map(|e| e.1[ch]).sum();
            assert!((sum - 1.0).abs() < 1e-5);
        }
    }
    // Departure (rain → clear at hour 16, 1.5 h): rain stops by 60 %,
    // while clouds are still clearing.
    let w = weights(&s.blend(&b, c, 16.0 + 1.5 * 0.6), rain);
    assert_eq!(w[PRECIP], 0.0, "rain gone at 60 % of the way out");
    let clear_w = weights(&s.blend(&b, c, 16.0 + 1.5 * 0.3), clear);
    assert!(clear_w[CLOUDS] > 0.0 && clear_w[CLOUDS] < 1.0);
    // In the frame: the sky darkens before the rain starts.
    let climates = [(c, 1.0)];
    let f = evaluate(&b, &s, &input(&climates, 10.0 + 2.0 * 0.35));
    assert!(f.clouds.coverage > 0.5 && intensity(&f, &b, "rain") == 0.0);
}

#[test]
fn discrete_crossfades_never_morph() {
    let b = book();
    let mut s = WeatherState::new(9);
    let c = b.climate(&id("sunny")).unwrap();
    s.force(&id("sunny"), &id("rain"), 0.0, Some(0.0), Some(100.0));
    s.force(&id("sunny"), &id("snow"), 20.0, Some(2.0), Some(100.0));
    let climates = [(c, 1.0)];
    let (mut last_rain, mut last_snow, mut last_cover) = (f32::INFINITY, 0.0, None::<f32>);
    for i in 0..=200 {
        let t = 19.0 + f64::from(i) * 0.02;
        let f = evaluate(&b, &s, &input(&climates, t));
        for e in &f.precip {
            let k = b.precip_id(e.kind).as_str();
            assert!(k == "rain" || k == "snow", "only the two kinds, never a mix: {k}");
        }
        let (rain, snow) = (intensity(&f, &b, "rain"), intensity(&f, &b, "snow"));
        assert!(rain <= last_rain + 1e-6 && snow >= last_snow - 1e-6, "rain fades out, snow fades in");
        assert!(rain <= 1.0 + 1e-6 && snow <= 0.8 + 1e-6);
        (last_rain, last_snow) = (rain, snow);
        // Cues crossfade too.
        let rain_loop = f.cues.iter().find(|q| q.sound.as_str() == "rain_loop").map_or(0.0, |q| q.volume);
        assert!(rain_loop <= 1.0 + 1e-6);
        if let Some(prev) = last_cover {
            assert!((f.clouds.coverage - prev).abs() < 0.05, "continuous channels have no pops");
        }
        last_cover = Some(f.clouds.coverage);
    }
    assert!((last_rain - 0.0).abs() < 1e-6 && (last_snow - 0.8).abs() < 1e-6);
    // Snow's storm source is on at full strength once snow has arrived.
    let f = evaluate(&b, &s, &input(&climates, 30.0));
    assert!(f.storms.is_empty(), "the sunny climate has no storm source");
    let north = b.climate(&id("north")).unwrap();
    let mut s2 = WeatherState::new(1);
    s2.force(&id("north"), &id("snow"), 0.0, Some(0.0), Some(10.0));
    let f = evaluate(&b, &s2, &input(&[(north, 1.0)], 5.0));
    assert_eq!(f.storms.len(), 1);
    assert!((f.storms[0].strength - 1.0).abs() < 1e-5);
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn border_blend_is_continuous() {
    let b = book();
    let s = WeatherState::new(3);
    let (sunny, snowy) = (b.climate(&id("sunny")).unwrap(), b.climate(&id("snowy")).unwrap());
    assert!((border_weight(0.0, BORDER_BAND_M) - 0.5).abs() < 1e-6);
    assert!((border_weight(128.0, BORDER_BAND_M) - 1.0).abs() < 1e-6);
    let mut last: Option<WeatherFrame> = None;
    // Walk from 300 m inside "sunny" to 300 m inside "snowy", 2 m steps.
    for i in -150..=150 {
        let x = i as f32 * 2.0;
        let blend = if x < 0.0 { border_blend(sunny, Some(snowy), -x, BORDER_BAND_M) } else { border_blend(snowy, Some(sunny), x, BORDER_BAND_M) };
        let f = evaluate(&b, &s, &input(&blend, 12.0));
        if x <= -128.0 {
            assert_eq!(intensity(&f, &b, "snow"), 0.0);
        }
        if x >= 128.0 {
            assert!((intensity(&f, &b, "snow") - 0.8).abs() < 1e-6);
        }
        if let Some(p) = &last {
            assert!((f.clouds.coverage - p.clouds.coverage).abs() < 0.02, "coverage pops at {x}");
            assert!((intensity(&f, &b, "snow") - intensity(p, &b, "snow")).abs() < 0.02, "snow pops at {x}");
            assert!((f.sky.wind.velocity - p.sky.wind.velocity).length() < 0.2, "wind pops at {x}");
        }
        last = Some(f);
    }
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn lightning_is_poisson_at_the_rate() {
    let b = book();
    let s = WeatherState::new(77);
    let c = b.climate(&id("stormy")).unwrap();
    let climates = [(c, 1.0)];
    let hours = 400.0;
    let step = 1.0 / 120.0;
    let mut times = Vec::new();
    let mut prev = 0.0;
    let mut f = WeatherFrame::default();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    for i in 1..=(hours / step) as u32 {
        let t = f64::from(i) * step;
        evaluate_into(&b, &s, &WeatherInput { prev_hours: prev, ..input(&climates, t) }, &mut f);
        times.extend(f.lightning.iter().map(|l| l.time));
        assert!(f.lightning.iter().all(|l| l.thunder.is_some() && l.distance_m >= 300.0));
        prev = t;
    }
    let expected = 30.0 * hours;
    let n = times.len() as f64;
    assert!((n - expected).abs() < 4.0 * expected.sqrt(), "{n} strikes, expected {expected}");
    // Exponential gaps: their coefficient of variation is about 1.
    let gaps: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
    let mean = gaps.iter().sum::<f64>() / gaps.len() as f64;
    let var = gaps.iter().map(|g| (g - mean).powi(2)).sum::<f64>() / gaps.len() as f64;
    let cv = var.sqrt() / mean;
    assert!((cv - 1.0).abs() < 0.05, "coefficient of variation {cv}");
    // Independent of frame boundaries: coarse steps report the same strikes.
    let mut coarse = Vec::new();
    let mut prev = 0.0;
    for i in 1..=(400 * 7) {
        let t = f64::from(i) / 7.0;
        evaluate_into(&b, &s, &WeatherInput { prev_hours: prev, ..input(&climates, t) }, &mut f);
        coarse.extend(f.lightning.iter().map(|l| l.time));
        prev = t;
    }
    assert_eq!(coarse, times);
    // A clear climate never strikes.
    let sunny = [(b.climate(&id("sunny")).unwrap(), 1.0)];
    let f = evaluate(&b, &s, &WeatherInput { prev_hours: 0.0, ..input(&sunny, 0.9) });
    assert!(f.lightning.is_empty());
}

#[test]
fn wind_target_and_visibility() {
    let b = book();
    let (s, c) = forced(&b, "sunny", "rain", 6.0);
    let climates = [(c, 1.0)];
    let mut last: Option<glam::Vec2> = None;
    for i in 0..=400 {
        let t = 8.0 + f64::from(i) * 0.025;
        let f = evaluate(&b, &s, &input(&climates, t));
        let speed = f.wind().speed();
        assert!(speed <= 8.1, "speed {speed} at {t}");
        if let Some(p) = last {
            assert!((f.wind().velocity - p).length() < 0.5, "wind eases, no jumps (t {t})");
        }
        last = Some(f.wind().velocity);
        assert!((f.fog.wind - f.wind().velocity).length() < 1e-6);
    }
    let clear = evaluate(&b, &s, &input(&climates, 9.0));
    let rain = evaluate(&b, &s, &input(&climates, 14.0));
    assert!((clear.visibility_m - MAX_VISIBILITY_M).abs() < 1.0);
    // Rain: fog 1e-3 + rain 2e-3 → 3.912 / 3e-3 ≈ 1.3 km.
    assert!((rain.visibility_m - 3.912 / 3e-3).abs() < 1.0, "{}", rain.visibility_m);
    assert!(rain.sky.cloud_cover > clear.sky.cloud_cover);
    assert!(rain.surface.wet > 0.99 && rain.tags.iter().any(|t| t.0.as_str() == "wet"));
    // No climate (an interior): calm defaults, nothing discrete.
    let none = evaluate(&b, &s, &input(&[], 14.0));
    assert!(none.precip.is_empty() && none.cues.is_empty() && none.lightning.is_empty());
}

#[test]
fn records_flow_through_sf_records() {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    for (rid, p) in presets() {
        db.define_value(base, PRESET_KIND, rid, &Value::from_serialize(&p).unwrap()).unwrap();
    }
    for (rid, c) in climates() {
        db.define_value(base, CLIMATE_KIND, rid, &Value::from_serialize(&c).unwrap()).unwrap();
    }
    for (rid, k) in precip() {
        db.define(base, rid, k).unwrap();
    }
    // A mod makes the temperate climate always rain.
    let m = db.add_source("wet mod", &[base]);
    db.patch::<Climate>(m, &id("temperate"), |c| c.weights = vec![(id("RAIN"), 1.0)]).unwrap();
    let b = WeatherBook::from_db(&db);
    assert!(b.warnings.is_empty(), "{:?}", b.warnings);
    let c = b.climate(&id("temperate")).unwrap();
    let s = WeatherState::new(0);
    assert!((0..50).all(|k| s.slot_preset(&b, c, k) == b.preset(&id("rain"))));
    // Record ids serialize as their (folded) strings.
    let v = db.value_of(CLIMATE_KIND, &id("temperate")).unwrap().unwrap();
    let back: Climate = v.deserialize_into().unwrap();
    assert_eq!(back.weights[0].0, id("rain"));
}

#[test]
fn evaluate_cost() {
    let b = book();
    let mut s = WeatherState::new(11);
    s.force(&id("temperate"), &id("thunder"), 0.0, Some(2.0), Some(50.0));
    s.force(&id("north"), &id("snow"), 0.5, Some(2.0), Some(50.0));
    let climates = [(b.climate(&id("temperate")).unwrap(), 0.6), (b.climate(&id("north")).unwrap(), 0.4)];
    let mut f = WeatherFrame::default();
    let n = 20_000;
    // Mid-transition (both climates blending), 30x game time at 60 fps.
    let step = 30.0 / 60.0 / 3600.0;
    let start = Instant::now();
    let mut prev = 1.0;
    for i in 0..n {
        let t = 1.0 + f64::from(i) * step;
        evaluate_into(&b, &s, &WeatherInput { prev_hours: prev, ..input(&climates, t) }, &mut f);
        prev = t;
    }
    let ms = start.elapsed().as_secs_f64() * 1e3 / f64::from(n);
    println!("evaluate: {ms:.5} ms per frame (2 climates, mid-transition, lightning on)");
    let gate = if cfg!(debug_assertions) { 0.5 } else { 0.05 };
    assert!(ms < gate, "evaluate {ms} ms > {gate} ms");
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn one_shot_sets_draw_by_chance_and_hush_with_the_weather() {
    // A draw: each chance is a probability, the rest is silence.
    let set = OneShotSet { sounds: vec![(id("bird"), 0.2), (id("frog"), 0.1)], interval_s: [5.0, 1.0], volume: 0.5 };
    let n = 10_000;
    let (mut bird, mut frog, mut silent) = (0, 0, 0);
    for i in 0..n {
        match set.pick((i as f32 + 0.5) / n as f32) {
            Some(s) if *s == id("bird") => bird += 1,
            Some(s) if *s == id("frog") => frog += 1,
            Some(s) => panic!("unexpected {s:?}"),
            None => silent += 1,
        }
    }
    assert!((bird - 2000_i32).abs() <= 1 && (frog - 1000_i32).abs() <= 1 && (silent - 7000_i32).abs() <= 1, "{bird} {frog} {silent}");
    // Chances above 1 in total normalize: never silent.
    let loud = OneShotSet { sounds: vec![(id("a"), 1.0), (id("b"), 3.0)], ..OneShotSet::default() };
    assert_eq!(loud.pick(0.2), Some(&id("a")));
    assert_eq!(loud.pick(0.3), Some(&id("b")));
    assert_eq!(loud.pick(0.999_9), Some(&id("b")));
    assert_eq!(OneShotSet::default().pick(0.0), None);
    // Intervals: uniform in the range, either order.
    assert_eq!(set.next_interval_s(0.0), 1.0);
    assert_eq!(set.next_interval_s(0.5), 3.0);
    assert_eq!(set.next_interval_s(1.0), 5.0);

    // In the frame: the climate's sets and loops play at the weather's
    // `climate_ambience`, fading on the cue channel (snow hushes them).
    let mut presets = presets();
    presets.iter_mut().find(|(p, _)| *p == id("snow")).unwrap().1.climate_ambience = 0.0;
    let mut climates = climates();
    climates.push((
        id("birdland"),
        Climate {
            cues: vec![Cue { sound: id("crickets"), volume: 0.4 }],
            one_shots: vec![set.clone(), OneShotSet::default()],
            ..climate(&[("clear", 1.0)])
        },
    ));
    let book = WeatherBook::new(presets, climates, precip());
    assert!(book.warnings.is_empty(), "{:?}", book.warnings);
    let c = book.climate(&id("birdland")).unwrap();
    let mut state = WeatherState::new(7);
    state.force(&id("birdland"), &id("snow"), 10.0, Some(1.0), Some(100.0));
    let f = evaluate(&book, &state, &input(&[(c, 1.0)], 5.0));
    // The empty set (no sounds) isn't reported.
    assert_eq!(f.one_shots, vec![OneShotFrame { climate: c, set: 0, volume: 0.5 }]);
    assert_eq!(book.climate_record(c).one_shots[0], set);
    let crickets = |f: &WeatherFrame| f.cues.iter().find(|e| e.region && e.sound == id("crickets")).map_or(0.0, |e| e.volume);
    assert!((crickets(&f) - 0.4).abs() < 1e-6);
    let mut last = 0.5;
    for i in 1..=8 {
        let f = evaluate(&book, &state, &input(&[(c, 1.0)], 10.0 + f64::from(i) / 8.0));
        let v = f.one_shots.first().map_or(0.0, |o| o.volume);
        assert!(v <= last + 1e-6, "ambience rises mid-transition: {last} -> {v}");
        assert!((crickets(&f) - v * 0.8).abs() < 1e-5, "loops and one-shots fade together");
        last = v;
    }
    assert_eq!(last, 0.0, "snow hushes the region");
    // Half the spatial weight on a border: half the volume.
    let f = evaluate(&book, &state, &input(&[(c, 0.5), (book.climate(&id("sunny")).unwrap(), 0.5)], 5.0));
    assert!((f.one_shots[0].volume - 0.25).abs() < 1e-6);
}

#[test]
fn thunder_delay_and_flash_are_pure() {
    let s = LightningStrike {
        time: 12.345,
        climate: ClimateIx(0),
        azimuth: 0.0,
        distance_m: 3430.0,
        flash: 1.0,
        fade_s: 0.5,
        thunder: None,
    };
    assert!((s.thunder_delay_s() - 10.0).abs() < 1e-4);
    assert!((s.thunder_volume() - 500.0 / 3430.0).abs() < 1e-6);
    assert_eq!(s.flash_at(-0.1), 0.0);
    let peak = s.flash_at(0.0);
    assert!(peak > 0.9 && peak <= 1.0, "{peak}");
    assert!(s.flash_at(3.0) < 0.01, "fades out");
    assert_eq!(s.flash_at(0.13), s.clone().flash_at(0.13), "same strike, same flash");
    let samples: Vec<f32> = (0..60_u8).map(|i| s.flash_at(f32::from(i) * 0.01)).collect();
    assert!(samples.windows(2).any(|w| w[1] > w[0] + 1e-3), "no re-stroke");
}

mod map;

#[test]
fn sun_glare_follows_the_sky_channel_without_pops() {
    let b = book();
    let (s, c) = forced(&b, "sunny", "rain", 6.0);
    let climates = [(c, 1.0)];
    let glare = |t: f64| evaluate(&b, &s, &input(&climates, t)).sky.sun_glare;
    assert!((glare(5.0) - 1.0).abs() < 1e-6, "clear sky: full glare");
    assert!(glare(13.0).abs() < 1e-6, "rain: none");
    // The way in is monotone and continuous (2 h transition from hour 10).
    let mut last = glare(9.9);
    for i in 0..=100 {
        let g = glare(10.0 + 2.0 * f64::from(i) / 100.0);
        assert!(g <= last + 1e-6 && (g - last).abs() < 0.1, "no pop: {last} -> {g}");
        last = g;
    }
    assert!(glare(10.8) > 0.0 && glare(10.8) < 1.0, "mid-transition is partial");
}
