#![allow(clippy::float_cmp, clippy::cast_precision_loss)] // exact values by design (0, clamps, bit-exact round trips); small texel counts

use super::*;
use glam::{UVec2, Vec2};

#[test]
fn grid_index_round_trips() {
    let grid = GiSettings::default().grid();
    assert_eq!(grid.probe_count(), 16 * 8 * 16);
    for i in 0..grid.probe_count() {
        assert_eq!(grid.index(grid.coord(i)), i);
    }
    assert_eq!(grid.coord(17), UVec3::new(1, 1, 0));
    assert_eq!(grid.coord(16 * 8), UVec3::new(0, 0, 1));
}

#[test]
fn probes_sit_at_cell_centres() {
    let grid = GridDesc { dims: UVec3::new(4, 2, 2), spacing: 1.0 };
    assert_eq!(grid.extent(), Vec3::new(4.0, 2.0, 2.0));
    assert_eq!(grid.probe_offset(UVec3::ZERO), Vec3::new(-1.5, -0.5, -0.5));
    assert_eq!(grid.probe_offset(UVec3::new(3, 1, 1)), Vec3::new(1.5, 0.5, 0.5));
}

#[test]
fn fibonacci_directions_are_unit_and_balanced() {
    let dirs = fibonacci_sphere(64);
    assert_eq!(dirs.len(), 64);
    for d in &dirs {
        assert!((d.length() - 1.0).abs() < 1e-5, "{d}");
    }
    // Well distributed: the mean is near zero and every axis has samples facing it.
    let mean = dirs.iter().copied().sum::<Vec3>() / 64.0;
    assert!(mean.length() < 0.05, "{mean}");
    for axis in [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z] {
        assert!(dirs.iter().filter(|d| d.dot(axis) > 0.5).count() >= 10, "{axis}");
    }
}

#[test]
fn frame_rotations_are_orthonormal_and_differ() {
    let r0 = frame_rotation(0);
    let r1 = frame_rotation(1);
    for r in [r0, r1] {
        assert!((r.determinant() - 1.0).abs() < 1e-4);
        assert!((r * r.transpose()).abs_diff_eq(Mat3::IDENTITY, 1e-4));
    }
    assert!(!r0.abs_diff_eq(r1, 1e-3));
    assert_eq!(frame_rotation(7), frame_rotation(7));
}

#[test]
fn frame_window_covers_the_grid_round_robin() {
    assert_eq!(frame_window(0, 512, 2048), (0, 512));
    assert_eq!(frame_window(3, 512, 2048), (1536, 512));
    assert_eq!(frame_window(4, 512, 2048), (0, 512));
    assert_eq!(frame_window(1, 512, 100), (0, 100));
    assert_eq!(frame_window(2, 300, 1000), (600, 300));
    assert_eq!(frame_window(5, 1, 0), (0, 0));
}

#[test]
fn octahedral_encode_decode_round_trips() {
    for d in fibonacci_sphere(500).into_iter().chain([Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z]) {
        let p = oct_encode(d);
        assert!(p.abs().max_element() <= 1.0 + 1e-6, "{p}");
        let back = oct_decode(p);
        assert!((back - d).length() < 1e-5, "{d} -> {p} -> {back}");
    }
    // The poles and the fold: +z at the centre, -z at the corners.
    assert_eq!(oct_encode(Vec3::Z), Vec2::ZERO);
    assert_eq!(oct_encode(Vec3::NEG_Z), Vec2::ONE);
    assert!((oct_decode(Vec2::new(-1.0, 1.0)) - Vec3::NEG_Z).length() < 1e-6);
    // Texel centres decode to unit vectors that encode back into their own texel.
    for inner in [IRRADIANCE_TEXELS, VISIBILITY_TEXELS] {
        for y in 0..inner {
            for x in 0..inner {
                let d = oct_texel_dir(UVec2::new(x, y), inner);
                assert!((d.length() - 1.0).abs() < 1e-5);
                let t = ((oct_encode(d) * 0.5 + 0.5) * inner as f32).floor().as_uvec2();
                assert_eq!(t, UVec2::new(x, y));
            }
        }
    }
}

#[test]
fn atlas_tiles_round_trip_and_fit() {
    let dims = UVec3::new(16, 8, 16);
    let irr = AtlasLayout::irradiance(dims);
    let vis = AtlasLayout::visibility(dims);
    assert_eq!((irr.tile_size(), vis.tile_size()), (8, 16));
    assert_eq!(irr.tiles(), UVec2::new(128, 16));
    assert_eq!(irr.size(), UVec2::new(1024, 128));
    assert_eq!(vis.size(), UVec2::new(2048, 256));
    let grid = GridDesc { dims, spacing: 1.0 };
    let mut seen = std::collections::HashSet::new();
    for i in 0..grid.probe_count() {
        let c = grid.coord(i);
        let tile = irr.tile(c);
        assert!(tile.x < irr.tiles().x && tile.y < irr.tiles().y);
        assert_eq!(irr.probe(tile), c);
        assert!(seen.insert(tile));
    }
    assert_eq!(irr.tile(UVec3::new(3, 2, 5)), UVec2::new(35, 5));
    assert_eq!(irr.texel(UVec3::new(3, 2, 5), UVec2::new(1, 1)), UVec2::new(281, 41));
    // The interior of a tile maps to texels 1..=inner of it: oct (-1, -1) is the interior's
    // corner, (1, 1) the far corner.
    let c = UVec3::new(1, 0, 0);
    assert_eq!(irr.uv(c, Vec2::splat(-1.0)) * irr.size().as_vec2(), Vec2::new(9.0, 1.0));
    assert_eq!(irr.uv(c, Vec2::ONE) * irr.size().as_vec2(), Vec2::new(15.0, 7.0));
}

#[test]
fn border_texels_repeat_the_octahedral_wrap() {
    let n = IRRADIANCE_TEXELS; // tile 8x8, interior 1..=6
    assert_eq!(border_source(UVec2::new(3, 3), n), None);
    assert_eq!(border_source(UVec2::new(1, 6), n), None);
    // Edges: the row/column inside, mirrored.
    assert_eq!(border_source(UVec2::new(1, 0), n), Some(UVec2::new(6, 1)));
    assert_eq!(border_source(UVec2::new(2, 0), n), Some(UVec2::new(5, 1)));
    assert_eq!(border_source(UVec2::new(2, 7), n), Some(UVec2::new(5, 6)));
    assert_eq!(border_source(UVec2::new(0, 2), n), Some(UVec2::new(1, 5)));
    assert_eq!(border_source(UVec2::new(7, 6), n), Some(UVec2::new(6, 1)));
    // Corners: the opposite interior corner.
    assert_eq!(border_source(UVec2::new(0, 0), n), Some(UVec2::new(6, 6)));
    assert_eq!(border_source(UVec2::new(7, 0), n), Some(UVec2::new(1, 6)));
    assert_eq!(border_source(UVec2::new(0, 7), n), Some(UVec2::new(6, 1)));
    assert_eq!(border_source(UVec2::new(7, 7), n), Some(UVec2::new(1, 1)));
    // Every border texel has an interior source; the 28 border texels of an 8x8 tile.
    let mut border = 0;
    for y in 0..n + 2 {
        for x in 0..n + 2 {
            if let Some(s) = border_source(UVec2::new(x, y), n) {
                assert!((1..=n).contains(&s.x) && (1..=n).contains(&s.y));
                border += 1;
            }
        }
    }
    assert_eq!(border, 28);
    // Geometrically: an edge border texel's direction is next to its source's (the wrap).
    let v = VISIBILITY_TEXELS;
    for x in 1..=v {
        let src = border_source(UVec2::new(x, 0), v).unwrap_or(UVec2::ZERO);
        let outside = oct_decode(Vec2::new((x as f32 - 0.5) / v as f32 * 2.0 - 1.0, -1.0 - 1.0 / v as f32).clamp(Vec2::NEG_ONE, Vec2::ONE));
        let inside = oct_texel_dir(src - 1, v);
        assert!(outside.dot(inside) > 0.95, "{x}: {outside} vs {inside}");
    }
}

#[test]
fn chebyshev_weights_known_distances() {
    // Nearer than the mean: fully visible.
    assert_eq!(chebyshev(2.0, 4.0, 1.0), 1.0);
    assert_eq!(chebyshev(2.0, 4.0, 2.0), 1.0);
    // mean 2, mean square 4.5 -> variance 0.5; at d = 3: 0.5 / (0.5 + 1) = 1/3, cubed.
    assert!((chebyshev(2.0, 4.5, 3.0) - 1.0 / 27.0).abs() < 1e-6);
    // Zero variance behind the mean: occluded.
    assert_eq!(chebyshev(1.0, 1.0, 1.5), 0.0);
    // Falls off with distance.
    assert!(chebyshev(2.0, 4.5, 2.5) > chebyshev(2.0, 4.5, 4.0));
    assert!((max_ray_distance(1.0) - 1.5 * 3f32.sqrt()).abs() < 1e-6);
    assert!((self_shadow_bias(2.0) - 0.45).abs() < 1e-6);
}

#[test]
fn irradiance_texels_are_cosine_weighted_means() {
    let samples: Vec<(Vec3, Vec3)> = fibonacci_sphere(64).into_iter().map(|d| (d, Vec3::splat(2.0))).collect();
    for dir in [Vec3::X, Vec3::NEG_Y, oct_texel_dir(UVec2::new(2, 3), IRRADIANCE_TEXELS)] {
        assert!((irradiance_texel(dir, &samples) - Vec3::splat(2.0)).abs().max_element() < 1e-5);
    }
    // Light from above only: texels facing down see nothing.
    let one_sided = [(Vec3::Y, Vec3::ONE), (Vec3::NEG_Y, Vec3::ZERO)];
    assert_eq!(irradiance_texel(Vec3::Y, &one_sided), Vec3::ONE);
    assert_eq!(irradiance_texel(Vec3::NEG_Y, &one_sided), Vec3::ZERO);
    assert_eq!(irradiance_texel(Vec3::X, &one_sided), Vec3::ZERO);
    // Visibility texels use a much sharper lobe.
    let (cos, sharp) = texel_weights(Vec3::Y, Vec3::new(0.0, 0.9, 0.436).normalize());
    assert!(sharp < cos * 0.1);
}

#[test]
fn blend_is_the_hysteresis_mix() {
    let (new, old) = (Vec3::ONE, Vec3::ZERO);
    assert_eq!(hysteresis_blend(new, old, 0.0), new);
    assert_eq!(hysteresis_blend(new, old, 1.0), old);
    assert!((hysteresis_blend(new, old, 0.95) - Vec3::splat(0.05)).abs().max_element() < 1e-6);
}

#[test]
fn rays_are_clamped_to_the_workgroup() {
    let s = GiSettings { rays: 500, ..GiSettings::default() };
    assert_eq!(s.rays(), MAX_RAYS);
    assert_eq!(GiSettings { rays: 0, ..s }.rays(), 1);
}

/// Bevy's `getDistanceAttenuation` evaluated by hand in f64 (`pbr_lighting.wesl`).
fn bevy_attenuation(d: f64, range: f64) -> f64 {
    let d2 = d * d;
    let factor = d2 / (range * range);
    let smooth = (1.0 - factor * factor).clamp(0.0, 1.0);
    smooth * smooth / d2.max(0.0001)
}

#[test]
fn attenuation_matches_bevy() {
    // d = 2, range 4: factor 1/4, smooth 15/16, (15/16)^2 / 4.
    assert!((distance_attenuation(4.0, 1.0 / 16.0) - 0.219_726_56).abs() < 1e-5);
    for (d, range) in [(0.5f32, 4.0f32), (1.0, 4.0), (3.5, 4.0), (6.0, 10.0), (0.001, 2.0)] {
        let got = distance_attenuation(d * d, 1.0 / (range * range));
        let want = bevy_attenuation(f64::from(d), f64::from(range));
        #[allow(clippy::cast_possible_truncation)]
        let want = want as f32;
        assert!((got - want).abs() <= 1e-5 * want.max(1.0), "d {d} range {range}: {got} vs {want}");
    }
    assert_eq!(distance_attenuation(16.0, 1.0 / 16.0), 0.0);
    assert_eq!(distance_attenuation(25.0, 1.0 / 16.0), 0.0);
}

#[test]
fn spot_cone_matches_bevy() {
    let (scale, offset) = spot_scale_offset(0.0, 60f32.to_radians());
    assert!((scale - 2.0).abs() < 1e-5 && (offset + 1.0).abs() < 1e-5);
    assert!((spot_attenuation(1.0, scale, offset) - 1.0).abs() < 1e-6);
    assert!(spot_attenuation(0.5, scale, offset) < 1e-6);
    assert_eq!(spot_attenuation(0.3, scale, offset), 0.0);
    let a = 3f32.sqrt() - 1.0; // cos 30 deg x 2 - 1
    assert!((spot_attenuation(30f32.to_radians().cos(), scale, offset) - a * a).abs() < 1e-5);
}

#[test]
fn lights_use_bevy_units() {
    let pi = std::f32::consts::PI;
    let light = GiLight::point(Vec3::ZERO, Vec3::ONE, 4.0 * pi * 10.0, 4.0, 0.25);
    assert!((light.color_intensity - Vec3::splat(10.0)).abs().max_element() < 1e-4);
    // A floor 2 m below, facing up: 10 cd x 1 (NdotL) x Bevy's attenuation at 2 m.
    let e = light.irradiance(Vec3::new(0.0, -2.0, 0.0), Vec3::Y);
    assert!((e.x - 10.0 * 0.219_726_56).abs() < 1e-4);
    assert_eq!(light.irradiance(Vec3::new(0.0, -2.0, 0.0), Vec3::NEG_Y), Vec3::ZERO);
    assert_eq!(light.irradiance(Vec3::new(0.0, -5.0, 0.0), Vec3::Y), Vec3::ZERO);
    // A spot pointing down lights below, not beside.
    let spot = light.with_spot(Vec3::NEG_Y, 0.0, 30f32.to_radians());
    assert!(spot.irradiance(Vec3::new(0.0, -2.0, 0.0), Vec3::Y).x > 0.0);
    assert_eq!(spot.irradiance(Vec3::new(2.0, 0.0, 0.0), Vec3::NEG_X), Vec3::ZERO);
    let words = spot.to_gpu();
    assert_eq!(words[7], FLAG_SPOT);
    assert_eq!(f32::from_bits(words[3]), 1.0 / 16.0);
    assert_eq!(f32::from_bits(words[13]), 0.25);
}

#[test]
fn keeps_the_nearest_lights_that_reach_the_volume() {
    let light = |x: f32, range: f32| GiLight::point(Vec3::new(x, 0.0, 0.0), Vec3::ONE, 100.0, range, 0.0);
    let (min, max) = (Vec3::splat(-1.0), Vec3::splat(1.0));
    let mut lights = vec![light(5.0, 3.0), light(0.5, 1.0), light(3.0, 3.0), light(-0.2, 1.0), light(10.0, 3.0)];
    select_nearest(&mut lights, min, max, 2);
    let xs: Vec<f32> = lights.iter().map(|l| l.position.x).collect();
    assert_eq!(xs, vec![-0.2, 0.5]);
    let mut lights = vec![light(5.0, 3.0), light(0.5, 1.0), light(3.0, 3.0), light(10.0, 3.0)];
    select_nearest(&mut lights, min, max, 64);
    let xs: Vec<f32> = lights.iter().map(|l| l.position.x).collect();
    assert_eq!(xs, vec![0.5, 3.0], "x 5 is 4 m from the box (range 3), x 10 further");
    assert_eq!(box_distance(Vec3::new(3.0, 0.0, 0.0), min, max), 2.0);
    assert_eq!(box_distance(Vec3::ZERO, min, max), 0.0);
}

#[test]
fn tracker_flags_moves_power_changes_and_removals() {
    let mut t = LightTracker::default();
    t.begin_frame();
    assert!(t.observe(1u32, Vec3::ZERO, 100.0, 4.0), "new");
    assert!(t.observe(2u32, Vec3::X, 100.0, 4.0));
    assert!(t.end_frame().is_empty());
    t.begin_frame();
    assert!(!t.observe(1, Vec3::new(0.03, 0.0, 0.0), 105.0, 4.0), "3 cm, +5%: same");
    assert!(t.end_frame().len() == 1, "light 2 gone");
    t.begin_frame();
    assert!(t.observe(1, Vec3::new(0.1, 0.0, 0.0), 105.0, 4.0), "moved 7 cm");
    t.end_frame();
    t.begin_frame();
    assert!(t.observe(1, Vec3::new(0.1, 0.0, 0.0), 130.0, 4.0), "+24%");
    assert!(t.end_frame().is_empty());
    assert_eq!(t.len(), 1);
    t.begin_frame();
    let gone = t.end_frame();
    assert_eq!(gone.len(), 1);
    assert!(gone[0].changed && gone[0].color_intensity == Vec3::ZERO && gone[0].range == 4.0);
    assert!(t.is_empty());
}

#[test]
fn changed_lights_lower_hysteresis_in_their_range() {
    let mut light = GiLight::point(Vec3::ZERO, Vec3::ONE, 100.0, 4.0, 0.0);
    let near = Vec3::new(3.0, 0.0, 0.0);
    let far = Vec3::new(5.0, 0.0, 0.0);
    assert_eq!(probe_hysteresis(near, &[light], 0.95, 0.8), 0.95, "unchanged");
    light.changed = true;
    assert_eq!(probe_hysteresis(near, &[light], 0.95, 0.8), 0.8);
    assert_eq!(probe_hysteresis(far, &[light], 0.95, 0.8), 0.95);
    assert_eq!(probe_hysteresis(near, &[light], 0.5, 0.8), 0.5, "never raises it");
    assert_eq!(probe_hysteresis(near, &[], 0.95, 0.8), 0.95);
}
