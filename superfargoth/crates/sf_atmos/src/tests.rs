#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::float_cmp)]

use glam::{Vec2, Vec3};

use crate::density::{cirrus_at, combine_density, erode_density, evolve_field, height_gradient, weather_at};
use crate::noise::{weather_noise, worley_fbm, Cells, Lattice, DETAIL_OCTAVES, DETAIL_SIZE, SHAPE_OCTAVES, SHAPE_SIZE, WEATHER_SIZE};
use crate::scroll::{CloudScroll, SCROLL_PERIOD};
use crate::settings::{CloudLook, CloudPreset, CloudWeather};

/// Every period must divide its texture, or the wrap seam is a wall.
#[test]
fn octaves_tile_their_textures() {
    for p in SHAPE_OCTAVES {
        assert_eq!(i64::from(SHAPE_SIZE) % p, 0);
    }
    for p in DETAIL_OCTAVES {
        assert_eq!(i64::from(DETAIL_SIZE) % p, 0);
    }
}

/// Every tile must divide the scroll/phase period, or origin moves shift noise.
#[test]
fn tiles_divide_the_phase_period() {
    let look = CloudLook::default();
    for t in [look.weather_tile, look.shape_tile, look.detail_tile] {
        assert!((SCROLL_PERIOD / f64::from(t)).fract().abs() < 1e-9, "tile {t}");
    }
}

#[test]
fn worley_wraps_and_has_range() {
    let c = Cells::new(8, 11);
    for i in 0..16 {
        let t = i as f32 / 16.0;
        let (a, b) = (c.distance(Vec3::new(0.999, t, t)), c.distance(Vec3::new(0.0, t, t)));
        assert!((a - b).abs() < 0.05, "seam at {t}: {a} vs {b}");
    }
    let octaves: Vec<Cells> = DETAIL_OCTAVES.iter().map(|p| Cells::new(*p, 3)).collect();
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for i in 0..512 {
        let t = i as f32 / 512.0;
        let v = worley_fbm(&octaves, Vec3::new(t, t * 0.37, t * 0.71));
        lo = lo.min(v);
        hi = hi.max(v);
    }
    assert!(hi - lo > 0.25, "worley is flat: {lo}..{hi}");
}

#[test]
fn lattice_wraps() {
    let l = Lattice::new(4, 7);
    let (a, b) = (l.sample(Vec3::new(0.9999, 0.3, 0.6)), l.sample(Vec3::new(0.0, 0.3, 0.6)));
    assert!((a - b).abs() < 1e-2);
}

/// The shared offset of the wind travel must track the true displacement in
/// each texture's own tile, across many wraps (the Megalophobia bug).
#[test]
fn each_texture_wraps_in_its_own_tile() {
    let look = CloudLook::default();
    let mut s = CloudScroll::default();
    let wind = Vec2::new(370.0, -110.0);
    let mut travelled = [0.0f64; 2];
    for i in 0..400 {
        s.advance(wind, 0.0, 1.0, look.shape_tile);
        travelled[0] += 370.0;
        travelled[1] -= 110.0;
        let o = s.offsets([0.0, 0.0], &look);
        for (got, tile, d) in [(o.shape.x, look.shape_tile, travelled[0]), (o.detail.y, look.detail_tile, travelled[1]), (o.weather.x, look.weather_tile, travelled[0])] {
            let want = (-d / f64::from(tile)).rem_euclid(1.0) as f32;
            let err = (got - want).rem_euclid(1.0);
            assert!(err.min(1.0 - err) < 1e-3, "step {i}: {got} vs {want} (tile {tile})");
        }
    }
}

/// Moving the origin by any number of 256 m cells and moving the relative
/// position back by the same amount must read the same texel.
#[test]
fn origin_shift_keeps_noise_still() {
    let look = CloudLook::default();
    let s = CloudScroll { travel: [1234.5, 77.25], ..CloudScroll::default() };
    let rel = 913.0f64;
    for cells in [1i64, 7, 64, 1000, -333] {
        let shift = cells as f64 * 256.0;
        for tile in [look.weather_tile, look.shape_tile, look.detail_tile] {
            let a = s.offsets([0.0, 0.0], &look);
            let b = s.offsets([shift, 0.0], &look);
            let pick = |o: crate::scroll::TileOffsets| if tile == look.weather_tile { o.weather.x } else if tile == look.shape_tile { o.shape.x } else { o.detail.x };
            let ua = (rel / f64::from(tile) + f64::from(pick(a))).rem_euclid(1.0);
            let ub = ((rel - shift) / f64::from(tile) + f64::from(pick(b))).rem_euclid(1.0);
            let err = (ua - ub).rem_euclid(1.0);
            assert!(err.min(1.0 - err) < 1e-5, "cells {cells} tile {tile}: {ua} vs {ub}");
        }
    }
}

/// The weather-noise map must open holes and close them at mid coverage,
/// and the coverage scalar must sweep from nothing to everything.
#[test]
fn coverage_sweeps_clear_to_overcast() {
    let map = weather_noise(5);
    let n = (WEATHER_SIZE * WEATHER_SIZE) as usize;
    let texel = |i: usize| [0, 1, 2, 3].map(|c| f32::from(map[i * 4 + c]) / 255.0);
    let covered = |w: &CloudWeather| (0..n).step_by(7).filter(|&i| weather_at(texel(i), w).coverage > 0.5).count() as f32 / (n / 7) as f32;
    let mut w = CloudWeather::preset(CloudPreset::Fair);
    w.coverage = 0.0;
    assert!(covered(&w) < 0.01, "clear has cloud");
    w.coverage = 1.0;
    assert!(covered(&w) > 0.99, "overcast has holes");
    w.coverage = 0.5;
    let mid = covered(&w);
    assert!((0.2..0.8).contains(&mid), "mid coverage {mid}");
    let mut last = 0.0;
    for c in 0..=10 {
        w.coverage = c as f32 / 10.0;
        let f = covered(&w);
        assert!(f + 1e-6 >= last, "coverage not monotonic at {c}");
        last = f;
    }
}

#[test]
fn storm_cells_raise_cover_and_wetness() {
    let calm = CloudWeather { storm: 0.0, coverage: 0.2, ..CloudWeather::default() };
    let storm = CloudWeather { storm: 1.0, ..calm };
    let core = [0.2, 0.5, 1.0, 1.0];
    assert!(weather_at(core, &storm).coverage > weather_at(core, &calm).coverage + 0.5);
    assert!(weather_at(core, &storm).wetness > weather_at(core, &calm).wetness);
}

#[test]
fn density_profile_and_threshold() {
    // Zero at both faces, positive inside.
    for kind in [0.0, 0.5, 1.0] {
        assert!(height_gradient(0.0, kind) < 1e-6 && height_gradient(1.0, kind) < 1e-6);
        assert!(height_gradient(0.4, kind) > 0.3);
    }
    // No coverage, no cloud; more coverage, more cloud.
    assert!(combine_density(0.8, 0.7, 0.4, 1.0, 0.0) <= 0.0);
    assert!(combine_density(0.8, 0.7, 0.4, 1.0, 0.9) > combine_density(0.8, 0.7, 0.4, 1.0, 0.5));
    // Erosion only removes.
    let d = 0.6;
    assert!(erode_density(d, 0.8, 0.1, 0.5) <= d + 1e-6);
}

#[test]
fn presets_parse_and_blend() {
    assert_eq!(CloudPreset::parse("Overcast"), Some(CloudPreset::Overcast));
    assert_eq!(CloudPreset::parse("cumulus"), Some(CloudPreset::Cumulus));
    assert_eq!(CloudPreset::parse("fog"), None);
    let a = CloudWeather::preset(CloudPreset::Clear);
    let b = CloudWeather::preset(CloudPreset::Storm);
    assert_eq!(a.lerp(&b, 0.0), a);
    assert_eq!(a.lerp(&b, 1.0), b);
    let m = a.lerp(&b, 0.5);
    assert!(m.coverage > a.coverage && m.coverage < b.coverage);
    assert!(a.mean_cover() < b.mean_cover());
    assert!((a.scaled_heights(0.5).base - a.base * 0.5).abs() < 1e-3);
}

/// The evolution blend keeps the field's range and order, and a second
/// octave that differs changes the result (the field re-forms).
#[test]
fn evolution_blend_keeps_range() {
    for a in [0.0f32, 0.2, 0.5, 0.8, 1.0] {
        assert!((evolve_field(a, a) - a).abs() < 0.2 + 1e-6, "same field drifts: {a}");
        let mut last = -1.0;
        for b in [0.0f32, 0.25, 0.5, 0.75, 1.0] {
            let f = evolve_field(a, b);
            assert!((0.0..=1.0).contains(&f) && f >= last);
            last = f;
        }
    }
    assert!((evolve_field(0.6, 0.0) - evolve_field(0.6, 1.0)).abs() > 0.3);
}

/// The two weather octaves and the high layer drift apart (evolution and
/// parallax), and the billows rise; nothing moves without wind.
#[test]
fn layers_drift_apart() {
    let look = CloudLook::default();
    let mut s = CloudScroll::default();
    s.advance(Vec2::ZERO, look.evolve, 100.0, look.shape_tile);
    assert_eq!(s, CloudScroll::default());
    for _ in 0..60 {
        s.advance(Vec2::new(10.0, 0.0), look.evolve, 1.0, look.shape_tile);
    }
    assert!((s.travel[0] - 600.0).abs() < 1e-6);
    assert!((s.travel[0] - s.evolve[0]).abs() > 100.0, "no evolution drift");
    assert!((s.cirrus[0] - 600.0 * f64::from(crate::scroll::CIRRUS_WIND)).abs() < 1e-6);
    assert!(s.rise > 0.0);
}

/// Cirrus: none at zero cover, more with cover, and the noise channel has
/// streaks (contrast) to threshold.
#[test]
fn cirrus_layer() {
    for f in [0.0f32, 0.5, 1.0] {
        assert!(cirrus_at(f, 0.0) < 1e-6);
    }
    assert!(cirrus_at(0.6, 0.8) > cirrus_at(0.6, 0.2));
    let map = weather_noise(7);
    let (lo, hi) = map.chunks(4).fold((255u8, 0u8), |(lo, hi), t| (lo.min(t[3]), hi.max(t[3])));
    assert!(hi - lo > 150, "cirrus channel flat: {lo}..{hi}");
}

/// The genera are distinct layers: low cumulus/stratocumulus, mid alto-,
/// a deep storm, and cirrus only up high. Scaled looks keep tiles that
/// divide the phase period.
#[test]
fn genera_and_scaling() {
    let w = |p| CloudWeather::preset(p);
    assert!(w(CloudPreset::Altocumulus).base > 2.0 * w(CloudPreset::Fair).base);
    assert!(w(CloudPreset::Storm).thickness > 2.5 * w(CloudPreset::Cumulus).thickness);
    assert!(w(CloudPreset::Cirrus).coverage < 1e-6 && w(CloudPreset::Cirrus).cirrus > 0.5);
    assert!(w(CloudPreset::Overcast).mean_cover() > 0.9 && w(CloudPreset::Clear).mean_cover() < 0.05);
    assert_eq!(CloudPreset::ALL.len(), 9);
    for (name, p) in CloudPreset::ALL {
        assert_eq!(CloudPreset::parse(name), Some(p));
        assert_eq!(p.name(), name);
    }
    let look = CloudLook::default().scaled(0.5);
    assert!((look.weather_tile - 4_096.0).abs() < 1e-3 && (look.shape_tile - 1_024.0).abs() < 1e-3);
    for t in [look.weather_tile, look.shape_tile, look.detail_tile, look.cirrus_tile, CloudLook::default().scaled(0.3).weather_tile] {
        assert!((SCROLL_PERIOD / f64::from(t)).fract().abs() < 1e-9, "tile {t}");
    }
    let h = w(CloudPreset::Fair).scaled_heights(0.5);
    assert!((h.cirrus_height - 4_000.0).abs() < 1e-3);
}

/// The time-sliced shadow map: bands cover every row once per cycle; the
/// map is redrawn whole when the anchor moves, the light turns or the
/// weather jumps, but not for a gradual blend.
#[test]
fn shadow_slices() {
    use crate::shadow::{ShadowRows, ShadowSlicer};
    let full = ShadowRows { start: 0, count: 1000 };
    let weather = CloudWeather::preset(CloudPreset::Fair);
    let sun = Vec3::new(0.3, 0.8, 0.2).normalize();
    let mut s = ShadowSlicer::default();
    assert_eq!(s.rows(1000, 8, [0, 0], sun, &weather, 3_000.0), full, "first frame");
    let mut covered = vec![0u32; 1000];
    for _ in 0..8 {
        let r = s.rows(1000, 8, [0, 0], sun, &weather, 3_000.0);
        assert!(r.count < 1000);
        for row in r.start..r.start + r.count {
            covered[row as usize] += 1;
        }
    }
    assert!(covered.iter().all(|&c| c == 1), "each row once per cycle");
    assert_eq!(s.rows(1000, 8, [1, 0], sun, &weather, 3_000.0), full, "anchor moved");
    // A slow blend: no full refresh.
    let mut blend = weather;
    blend.coverage += 0.005;
    assert_ne!(s.rows(1000, 8, [1, 0], sun, &blend, 3_000.0), full);
    let jump = CloudWeather::preset(CloudPreset::Overcast);
    assert_eq!(s.rows(1000, 8, [1, 0], sun, &jump, 3_000.0), full, "weather jump");
    // 0.01° per frame at 3 km is 0.5 m: fine; one degree is 52 m: redraw.
    let tiny = (sun + Vec3::X * 0.000_17).normalize();
    assert_ne!(s.rows(1000, 8, [1, 0], tiny, &jump, 3_000.0), full);
    let turned = (sun + Vec3::X * 0.02).normalize();
    assert_eq!(s.rows(1000, 8, [1, 0], turned, &jump, 3_000.0), full, "time-of-day jump");
    assert_eq!(s.rows(1000, 1, [1, 0], turned, &jump, 3_000.0), full, "one slice is every frame");
}

/// Storm shape (S3, `cbshape`): weather with the storm shape (cells by threshold, gate, anvil term).
fn storm_weather(storm: f32) -> CloudWeather {
    CloudWeather { coverage: 0.0, cloud_type: 1.0, storm, ..CloudWeather::preset(CloudPreset::Storm) }
}

/// The gate: without storm every shaped function is the old one, bit for bit.
#[test]
fn storm_shape_leaves_calm_decks_unchanged() {
    use crate::density::{combine_density_shaped, erode_density_shaped, shape_height, storm_gradient, weather_at_shaped};
    let steps = |n: usize| (0..=n).map(move |i| i as f32 / n as f32);
    for h in steps(20) {
        assert_eq!(shape_height(h, 1_600.0, 1_024.0, 0.0), h);
        for kind in steps(4) {
            for cov in steps(5) {
                assert_eq!(storm_gradient(h, kind, cov, 0.0, 1.0), height_gradient(h, kind));
                for r in steps(4) {
                    let (a, b) = (combine_density_shaped(r, 0.6, h, kind, cov, 0.0, 1.0), combine_density(r, 0.6, h, kind, cov));
                    assert_eq!(a.to_bits(), b.to_bits(), "combine h {h} kind {kind} cov {cov} r {r}");
                    assert_eq!(erode_density_shaped(r, cov, h, kind, 0.0).to_bits(), erode_density(r, cov, h, kind).to_bits());
                }
            }
        }
    }
    let map = weather_noise(5);
    for preset in [CloudPreset::Clear, CloudPreset::Fair, CloudPreset::Cumulus, CloudPreset::Overcast, CloudPreset::Altostratus] {
        let w = CloudWeather::preset(preset);
        for i in (0..(WEATHER_SIZE * WEATHER_SIZE) as usize).step_by(97) {
            let texel = [0, 1, 2, 3].map(|c| f32::from(map[i * 4 + c]) / 255.0);
            let (old, new) = (weather_at(texel, &w), weather_at_shaped(texel, &w, true));
            assert_eq!(new, old, "{preset:?} texel {i}");
            assert_eq!(new.storm, 0.0);
        }
    }
    // Full storm: the cells are the old cells.
    for c in steps(20) {
        let noise = [0.3, 0.5, c, 0.0];
        assert_eq!(weather_at_shaped(noise, &storm_weather(1.0), true).coverage, weather_at(noise, &storm_weather(1.0)).coverage);
    }
}

/// Highest height fraction with profile > 0.01.
fn storm_top(kind: f32, cov: f32, cb: f32, anvil: f32) -> f32 {
    (0..=200).map(|i| i as f32 / 200.0).filter(|&h| crate::density::storm_gradient(h, kind, cov, cb, anvil) > 0.01).fold(0.0, f32::max)
}

#[test]
fn storm_top_follows_coverage_and_anvil_spreads() {
    use crate::density::{combine_density_shaped, erode_density_shaped, weather_at_shaped};
    // Domed cells: lower tops at the rim.
    assert!(storm_top(1.0, 0.2, 1.0, 0.0) + 0.3 < storm_top(1.0, 1.0, 1.0, 0.0));
    assert!(storm_top(1.0, 0.2, 0.0, 0.0) > 0.95, "the old storm profile is flat-topped");
    // Anvil: beyond the tower (low coverage) the cloud reaches out above 0.75, not at mid height.
    let (mid, high) = (combine_density_shaped(0.8, 0.8, 0.55, 1.0, 0.15, 1.0, 1.0), combine_density_shaped(0.8, 0.8, 0.85, 1.0, 0.15, 1.0, 1.0));
    assert!(high > 0.05 && high > mid + 0.05, "anvil {high} vs mid {mid}");
    // Region edge: a weaker storm shrinks cells (rims gone, cores stay) instead of scaling them.
    let rim = [0.0, 0.5, 0.7, 0.0];
    let core = [0.0, 0.5, 1.0, 0.0];
    let edge = storm_weather(0.7);
    assert!(weather_at(rim, &edge).coverage > 0.3);
    assert!(weather_at_shaped(rim, &edge, true).coverage < 0.05);
    assert!(weather_at_shaped(core, &edge, true).coverage > 0.3);
    // Ragged base: more erosion low in a storm, none above the base band.
    assert!(erode_density_shaped(0.5, 0.3, 0.02, 0.35, 1.0) < erode_density(0.5, 0.3, 0.02, 0.35) - 0.05);
    assert_eq!(erode_density_shaped(0.5, 0.3, 0.3, 0.35, 1.0), erode_density(0.5, 0.3, 0.3, 0.35));
}

#[test]
fn storm_shape_is_finite_and_bounded() {
    use crate::density::{combine_density_shaped, erode_density_shaped, shape_height, storm_gradient};
    let steps = |n: usize| (0..=n).map(move |i| i as f32 / n as f32);
    for h in steps(40) {
        assert!(shape_height(h, 1_600.0, 1_024.0, 1.0).is_finite());
        for cb in steps(4) {
            for cov in steps(5) {
                for anvil in steps(4) {
                    let g = storm_gradient(h, 1.0, cov, cb, anvil);
                    assert!((0.0..=1.0).contains(&g), "gradient {g}");
                    for r in steps(4) {
                        let d = combine_density_shaped(r, 0.7, h, 1.0, cov, cb, anvil);
                        let e = erode_density_shaped(d, r, h, 0.35, cb);
                        assert!(d.is_finite() && d <= 1.0 + 1e-6 && e.is_finite() && e <= 1.0 + 1e-6, "d {d} e {e}");
                    }
                }
            }
        }
    }
}

/// Prints the storm's vertical profile through a cell (rim to core) and toward the region edge:
/// per height the fraction of the shape volume's layer with density > 0.1, old (`--off cbshape`) / new.
#[test]
#[ignore = "measurement: storm profile table"]
#[allow(clippy::cast_sign_loss, reason = "a height fraction in 0..1")]
fn storm_profile_table() {
    use crate::density::{combine_density_shaped, shape_height, weather_at_shaped};
    use crate::noise::shape_volume;
    let vol = shape_volume(0xC10D);
    let size = SHAPE_SIZE as usize;
    let (thickness, tile) = (1_600.0, 1_024.0);
    let layer = |y: f32| ((y.rem_euclid(1.0) * size as f32) as usize).min(size - 1);
    let footprint = |noise: [f32; 4], storm: f32, h: f32, shaped: bool| {
        let sample = if shaped { weather_at_shaped(noise, &storm_weather(storm), true) } else { weather_at(noise, &storm_weather(storm)) };
        let iy = layer(shape_height(h, thickness, tile, sample.storm));
        let mut hit = 0;
        let mut total = 0;
        for iz in (0..size).step_by(2) {
            for ix in (0..size).step_by(2) {
                let i = ((iz * size + iy) * size + ix) * 2;
                let (r, g) = (f32::from(vol[i]) / 255.0, f32::from(vol[i + 1]) / 255.0);
                hit += usize::from(combine_density_shaped(r, g, h, sample.kind, sample.coverage, sample.storm, sample.anvil) > 0.1);
                total += 1;
            }
        }
        hit as f32 / total as f32
    };
    let heights: Vec<f32> = (0..10).map(|i| 0.05 + i as f32 * 0.1).collect();
    let print = |title: &str, cols: &[(String, [f32; 4], f32)]| {
        println!("\n{title} (cloud fraction old/new per height fraction; storm deck {thickness} m, shape tile {tile} m)");
        println!("{:>5} | {}", "h", cols.iter().map(|c| format!("{:>11}", c.0)).collect::<Vec<_>>().join(" "));
        for &h in heights.iter().rev() {
            let row: Vec<String> = cols.iter().map(|(_, noise, s)| format!("{:>5.2}/{:<5.2}", footprint(*noise, *s, h, false), footprint(*noise, *s, h, true))).collect();
            println!("{h:>5.2} | {}", row.join(" "));
        }
    };
    let cells: Vec<(String, [f32; 4], f32)> = [1.0, 0.85, 0.75, 0.65, 0.55, 0.45, 0.35].iter().map(|&c| (format!("cell {c:.2}"), [0.0, 0.5, c, 0.0], 1.0)).collect();
    print("Through a storm cell, core (cell 1.0) to beyond the rim (0.35), storm 1", &cells);
    let edge: Vec<(String, [f32; 4], f32)> = [1.0, 0.85, 0.7, 0.55, 0.4].iter().map(|&s| (format!("storm {s:.2}"), [0.0, 0.5, 0.8, 0.0], s)).collect();
    print("Toward the region edge (storm scalar falls over the band), cell value 0.8", &edge);
}
