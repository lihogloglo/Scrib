//! World weather map (WE4a): raster fill, border continuity, map = camera
//! weather, determinism, uniform skip, toroidal recentering, cost.

use glam::{DVec2, IVec2};

use super::*;

/// A `w × h` raster of 100 m squares from (0, 0), band 200 m.
fn raster(w: u32, h: u32, f: impl Fn(u32, u32) -> Option<ClimateIx>) -> ClimateRaster {
    let cells: Vec<_> = (0..h).flat_map(|z| (0..w).map(move |x| (x, z))).map(|(x, z)| f(x, z)).collect();
    ClimateRaster::new(DVec2::ZERO, 100.0, w, h, &cells, 200.0).expect("raster has climates")
}

/// Temperate turning to thunder and north turning to snow, both mid-transition at 1.5 h.
fn storming(seed: u64) -> WeatherState {
    let mut s = WeatherState::new(seed);
    s.force(&id("temperate"), &id("thunder"), 0.0, Some(2.0), Some(50.0));
    s.force(&id("north"), &id("snow"), 0.5, Some(2.0), Some(50.0));
    s
}

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol * a.abs().max(b.abs()).max(1.0)
}

#[test]
fn raster_fills_sea_with_the_nearest_region() {
    let b = book();
    let (t, n) = (b.climate(&id("temperate")).unwrap(), b.climate(&id("north")).unwrap());
    // Islands at columns 0-2 (temperate) and 9-11 (north), sea between.
    let r = raster(12, 3, |x, _| match x {
        0..=2 => Some(t),
        9.. => Some(n),
        _ => None,
    });
    let cols: Vec<_> = (0..12).map(|x| r.climate(x, 1).unwrap()).collect();
    assert_eq!(cols, [t, t, t, t, t, t, n, n, n, n, n, n], "sea takes the nearest island's climate");
    assert_eq!(r.climates(), &[t, n]);
    // The two seas meet at x = 600 m, blended like a land border.
    let at = |x: f64| r.site_at(DVec2::new(x, 150.0));
    let (l, rr) = (at(600.0 - 1e-6), at(600.0 + 1e-6));
    assert_eq!(l[0].0, t);
    assert_eq!(rr[0].0, n);
    assert!((l[0].1 - 0.5).abs() < 1e-4 && (rr[0].1 - 0.5).abs() < 1e-4, "{l:?} {rr:?}");
    // Mid-sea, a band away from the sea border: one climate.
    assert_eq!(at(350.0).as_slice(), &[(t, 1.0)]);
    // Past the raster: the edge squares'.
    assert_eq!(r.site_at(DVec2::new(-5_000.0, -5_000.0)).as_slice(), &[(t, 1.0)]);
    assert!(ClimateRaster::new(DVec2::ZERO, 100.0, 2, 1, &[None, None], 200.0).is_none());
}

/// The map's sample along a 2 m border walk equals `evaluate(site)` (the
/// `CurrentWeather` path) in every continuous channel, and changes smoothly.
#[test]
fn map_at_the_camera_is_the_current_weather() {
    let b = book();
    let (t, n) = (b.climate(&id("temperate")).unwrap(), b.climate(&id("north")).unwrap());
    let r = raster(8, 4, |x, _| Some(if x < 4 { t } else { n }));
    let s = storming(3);
    let hours = 1.5;
    let mut map = WeatherMap::new(r, 32, 25.0);
    map.update(&b, &s, hours, 12.0, DVec2::new(400.0, 150.0));
    let mut prev: Option<ClimateRow> = None;
    let mut blended = 0;
    for i in 0..=150 {
        let p = DVec2::new(250.0 + 2.0 * f64::from(i), 150.0);
        let site = map.raster().site_at(p);
        assert!((site.iter().map(|e| e.1).sum::<f32>() - 1.0).abs() < 1e-6);
        blended += usize::from(site.len() == 2);
        let frame = evaluate(&b, &s, &WeatherInput { altitude: 30.0, ..input(&site, hours) });
        let want = ClimateRow::from_frame(&b, &frame);
        let got = map.sample(p);
        for (k, (g, w)) in got.ch.iter().zip(&want.ch).enumerate() {
            assert!(close(*g, *w, 1e-5), "channel {k} at x {}: map {g} vs evaluate {w}", p.x);
        }
        assert_eq!(got.look, want.look);
        assert!(close(got.visibility_m(30.0), frame.visibility_m, 1e-4), "{} vs {}", got.visibility_m(30.0), frame.visibility_m);
        if let Some(q) = prev {
            // 2 m steps across a 200 m band: the smoothstep's slope bounds every step.
            assert!((got.ch[row::COVERAGE] - q.ch[row::COVERAGE]).abs() < 0.02, "coverage pops at x {}", p.x);
            assert!((got.ch[row::PRECIP] - q.ch[row::PRECIP]).abs() < 0.02, "precipitation pops at x {}", p.x);
        }
        prev = Some(got);
        // The resolved texel at its center is the same sample.
        let c = map.texel_center(map.texel_of(p));
        let texel = map.texel_at(p).expect("inside the window");
        assert!(texel.max_diff(&MapTexel::from_row(&map.sample(c))) < 1e-6);
    }
    assert!(blended > 50, "the walk crosses the band ({blended} blended steps)");
}

#[test]
fn map_is_a_pure_function_of_seed_overrides_clock_raster() {
    let b = book();
    let (t, n) = (b.climate(&id("temperate")).unwrap(), b.climate(&id("north")).unwrap());
    let make = |seed| {
        let r = raster(8, 4, |x, z| Some(if x + z < 5 { t } else { n }));
        let mut m = WeatherMap::new(r, 32, 25.0);
        m.update(&b, &WeatherState::new(seed), 37.25, 9.0, DVec2::new(400.0, 200.0));
        m
    };
    let (a, a2) = (make(5), make(5));
    assert_eq!(a.rows(), a2.rows());
    assert_eq!(a.texels(), a2.texels());
    let differs = (0..40).any(|h| {
        let (mut x, mut y) = (make(5), make(6));
        let hours = f64::from(h) * 7.0;
        x.update_table(&b, &WeatherState::new(5), hours, 12.0);
        y.update_table(&b, &WeatherState::new(6), hours, 12.0);
        x.rows() != y.rows()
    });
    assert!(differs, "the seed changes the map");
}

#[test]
fn uniform_day_skips_and_recentering_writes_only_new_strips() {
    let b = book();
    let (t, sunny) = (b.climate(&id("temperate")).unwrap(), b.climate(&id("sunny")).unwrap());
    let r = raster(8, 4, |x, _| Some(if x < 4 { t } else { sunny }));
    let mut s = WeatherState::new(9);
    s.force(&id("temperate"), &id("clear"), 0.0, Some(0.1), Some(100.0));
    let mut map = WeatherMap::new(r, 32, 25.0);
    let cam = DVec2::new(400.0, 200.0);
    // Both regions clear (their winds differ, the map channels don't): nothing resolved.
    let u = map.update(&b, &s, 5.0, 12.0, cam);
    assert!(u.uniform && u.texels == 0, "{u:?}");
    assert!(map.uniform_row().is_some() && map.texel_at(cam).is_none());

    // A storm in one region: the whole window resolves once.
    s.force(&id("temperate"), &id("thunder"), 5.0, Some(0.1), Some(100.0));
    let u = map.update(&b, &s, 6.0, 12.0, cam);
    assert!(!u.uniform && u.texels == 32 * 32, "{u:?}");
    let u = map.update(&b, &s, 6.0, 12.0, cam);
    assert_eq!(u, MapUpdate::default(), "nothing changed, nothing written");

    // One 1/8-map step east (4 texels): only the entering strip is written,
    // and every texel still holds its world position's sample.
    let (w0, size) = map.window();
    let u = map.update(&b, &s, 6.0, 12.0, cam + DVec2::new(100.0, 0.0));
    assert!(u.recentered && !u.table_changed && u.texels == 4 * 32, "{u:?}");
    let (w1, _) = map.window();
    assert_eq!(w1 - w0, IVec2::new(4, 0));
    for j in 0..size {
        for i in 0..size {
            let c = map.texel_center(w1 + IVec2::new(i, j));
            let texel = map.texel_at(c).expect("inside the window");
            assert!(texel.max_diff(&MapTexel::from_row(&map.sample(c))) < 1e-6, "texel {i},{j}");
        }
    }
    // A floating-origin shift changes nothing: addressing is world-anchored.
    assert_eq!(map.texel_of(DVec2::new(1e6 + 12.5, -3e5)), IVec2::new(40_000, -12_000));
}

/// weather.md WE4 "Cost": the table (≤ 16 evaluates) ≤ 0.02 ms (release).
/// The 256² CPU resolve is the GPU pass's twin, printed for reference.
#[test]
#[allow(clippy::cast_possible_truncation)]
fn map_cost() {
    let mut cs = climates();
    for i in 0..11u8 {
        let w = [("clear", 1.0 + f32::from(i)), ("cloudy", 3.0), ("rain", 2.0), ("thunder", 1.0), ("snow", f32::from(i % 3))];
        cs.push((id(&format!("c{i}")), climate(&w)));
    }
    let b = WeatherBook::new(presets(), cs, precip());
    let all: Vec<ClimateIx> = (0..16).map(ClimateIx).collect();
    let r = ClimateRaster::new(DVec2::ZERO, 117.0, 64, 64, &(0..64 * 64).map(|i| Some(all[(i % 64 / 16) + (i / 64 / 16) * 4])).collect::<Vec<_>>(), 234.0)
        .unwrap();
    assert_eq!(r.climates().len(), 16);
    let s = WeatherState::new(17);
    let mut map = WeatherMap::new(r, MAP_SIZE, 64.0);
    let n = 2_000;
    let start = Instant::now();
    for i in 0..n {
        map.update_table(&b, &s, 3.0 + f64::from(i) * 0.01, 12.0);
    }
    let table_ms = start.elapsed().as_secs_f64() * 1e3 / f64::from(n);
    let start = Instant::now();
    let u = map.update(&b, &s, 3.0, 12.0, DVec2::new(3_700.0, 3_700.0));
    let resolve_ms = start.elapsed().as_secs_f64() * 1e3;
    println!("weather map: table {table_ms:.5} ms (16 climates); CPU resolve {resolve_ms:.2} ms for {} texels (GPU twin)", u.texels);
    let gate = if cfg!(debug_assertions) { 0.2 } else { 0.02 };
    assert!(table_ms < gate, "table {table_ms} ms > {gate} ms");
}
