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
