#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::float_cmp)]

use glam::{Vec2, Vec3};

use crate::density::{combine_density, erode_density, height_gradient, weather_at};
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
        s.advance(wind, 1.0);
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
    let s = CloudScroll { travel: [1234.5, 77.25] };
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
    assert_eq!(CloudPreset::parse("cumulus"), Some(CloudPreset::Fair));
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
