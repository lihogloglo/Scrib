//! Cases ported from openmw-rtx `apps/components_tests/rtx/image/shadingmap.cpp` (commit
//! `f2f8b2d5b2cb142d1b454b3bb1815f4552980562`), plus the apply step. Upstream paints linear
//! RGBA8; our input is sRGB, so the gradient test encodes its linear values first.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::float_cmp,
    reason = "test fixtures: small sizes, exact float expectations are intended"
)]

use super::*;

const SIDE: u32 = 256;

/// An opaque grey texture whose sRGB byte at `(x, y)` is `paint(x, y)`.
fn painted(width: u32, height: u32, paint: impl Fn(u32, u32) -> u8) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let grey = paint(x, y);
            rgba.extend_from_slice(&[grey, grey, grey, 255]);
        }
    }
    rgba
}

/// The sRGB byte nearest a linear value.
fn encoded(linear: f64) -> u8 {
    Srgb::get().encode(to_f32(linear))
}

fn furthest_from_neutral(grid: &ShadingGrid) -> f32 {
    grid.values()
        .iter()
        .map(|value| (value - 1.0).abs())
        .fold(0.0, f32::max)
}

fn middle_row(grid: &ShadingGrid) -> &[f32] {
    &grid.values()[EXTENT / 2 * EXTENT..][..EXTENT]
}

/// Mean linear luminance of an RGBA8 sRGB texture, alpha ignored.
fn mean_luminance(rgba: &[u8]) -> f64 {
    let srgb = Srgb::get();
    let total: f64 = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| {
            (0..3)
                .map(|c| f64::from(LUMINANCE_WEIGHTS[c] * srgb.decode[usize::from(texel[c])]))
                .sum::<f64>()
        })
        .sum();
    total / (rgba.len() / 4) as f64
}

/// A deterministic texture with varied colour and alpha.
fn noise(width: u32, height: u32) -> Vec<u8> {
    let mut state = 0x2545_f491_u32;
    (0..width * height * 4)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state.to_le_bytes()[0]
        })
        .collect()
}

#[test]
fn srgb_round_trip_is_exact_on_every_byte() {
    let srgb = Srgb::get();
    for byte in 0..=255_u8 {
        assert_eq!(srgb.encode(srgb.decode[usize::from(byte)]), byte);
    }
    assert_eq!(srgb.encode(-1.0), 0);
    assert_eq!(srgb.encode(2.0), 255);
}

#[test]
fn an_even_texture_comes_back_neutral() {
    let grid = shading_grid(&painted(SIDE, SIDE, |_, _| 137), SIDE, SIDE, true, true);
    assert!(
        furthest_from_neutral(&grid) < 1e-5,
        "{}",
        furthest_from_neutral(&grid)
    );
}

#[test]
fn a_checkerboard_is_left_alone() {
    let rgba = painted(SIDE, SIDE, |x, y| if (x + y) % 2 == 1 { 20 } else { 220 });
    assert!(furthest_from_neutral(&shading_grid(&rgba, SIDE, SIDE, true, true)) < 0.01);
}

#[test]
fn a_hole_in_a_cutout_weighs_nothing() {
    let mut rgba = painted(SIDE, SIDE, |_, _| 137);
    for y in 0..SIDE as usize {
        for x in (0..128).filter(|x| x % 4 < 2) {
            rgba[(y * SIDE as usize + x) * 4..][..4].fill(0);
        }
    }
    assert!(furthest_from_neutral(&shading_grid(&rgba, SIDE, SIDE, true, true)) < 0.01);
}

#[test]
fn a_fully_transparent_or_empty_texture_is_exactly_neutral() {
    let mut rgba = painted(64, 64, |x, _| u8::try_from(x * 4).unwrap());
    for texel in rgba.as_chunks_mut::<4>().0.iter_mut() {
        texel[3] = 0;
    }
    assert!(shading_grid(&rgba, 64, 64, true, true).is_neutral());
    assert!(shading_grid(&[], 0, 0, true, true).is_neutral());
    assert!(
        shading_grid(&painted(16, 16, |_, _| 0), 16, 16, true, true).is_neutral(),
        "black"
    );
}

#[test]
fn a_texture_smaller_than_the_grid_still_comes_back_neutral() {
    let grid = shading_grid(&painted(8, 8, |_, _| 90), 8, 8, true, true);
    assert!(furthest_from_neutral(&grid) < 0.01);
}

#[test]
fn a_painted_gradient_is_recovered_at_the_strength_it_was_painted() {
    // Linear 120 + 60 cos(2 pi x / 256), three to one; three blurs keep 0.96206 of the amplitude,
    // so the map is 1 + 0.481 cos: 0.519 to 1.481, peak at the left edge, trough halfway.
    let rgba = painted(SIDE, SIDE, |x, _| {
        let angle = 2.0 * std::f64::consts::PI * f64::from(x) / f64::from(SIDE);
        encoded((120.0 + 60.0 * angle.cos()) / 255.0)
    });
    let grid = shading_grid(&rgba, SIDE, SIDE, true, true);

    let mean = grid.values().iter().map(|&v| f64::from(v)).sum::<f64>() / CELLS as f64;
    assert!(
        (mean - 1.0).abs() < 1e-5,
        "mean {mean}: the estimate moves light rather than adding it"
    );

    let row = middle_row(&grid);
    let (peak_at, peak) = row
        .iter()
        .enumerate()
        .fold((0, f32::MIN), |b, (i, &v)| if v > b.1 { (i, v) } else { b });
    let (trough_at, trough) = row
        .iter()
        .enumerate()
        .fold((0, f32::MAX), |b, (i, &v)| if v < b.1 { (i, v) } else { b });
    assert!((peak - 1.481).abs() < 0.02, "peak {peak}");
    assert!((trough - 0.519).abs() < 0.02, "trough {trough}");
    assert!(
        (peak / trough - 2.854).abs() < 0.1,
        "ratio {}",
        peak / trough
    );
    assert_eq!(peak_at, 0);
    assert_eq!(trough_at, EXTENT / 2);
}

#[test]
fn the_correction_is_bounded_both_ways() {
    let rgba = painted(SIDE, SIDE, |x, _| if x < 128 { 3 } else { 250 });
    let grid = shading_grid(&rgba, SIDE, SIDE, true, true);
    for &value in grid.values() {
        assert!((FLOOR..=CEILING).contains(&value), "{value}");
    }
    assert!(
        (grid.values()[EXTENT / 2 * EXTENT + 1] - FLOOR).abs() < 0.001,
        "the black half is at the floor"
    );
}

#[test]
fn a_clamped_axis_is_blurred_without_its_opposite_edge() {
    let banner = painted(SIDE, SIDE, |x, _| if x < 8 { 255 } else { 60 });
    let last = EXTENT / 2 * EXTENT + EXTENT - 1;
    let middle = EXTENT / 2 * EXTENT + EXTENT / 2;

    let repeated = shading_grid(&banner, SIDE, SIDE, true, true);
    assert!(
        repeated.values()[last] > 1.05 * repeated.values()[middle],
        "the wrap carried nothing"
    );

    let clamped = shading_grid(&banner, SIDE, SIDE, false, true);
    assert_eq!(
        clamped.values()[last],
        clamped.values()[middle],
        "a clamped axis took the opposite edge"
    );
    assert!(
        clamped.values()[1] > clamped.values()[middle],
        "and still blurs inside the texture"
    );
}

#[test]
fn wrap_and_clamp_edges_differ_on_a_ramp() {
    // A left-to-right ramp: wrapping averages its dark left edge with its bright right edge.
    let ramp = painted(SIDE, SIDE, |x, _| u8::try_from(40 + x * 3 / 4).unwrap());
    let wrapped = shading_grid(&ramp, SIDE, SIDE, true, true);
    let clamped = shading_grid(&ramp, SIDE, SIDE, false, true);
    let (wrapped, clamped) = (middle_row(&wrapped), middle_row(&clamped));
    assert!(
        wrapped[0] > clamped[0],
        "wrap lifts the dark edge: {} vs {}",
        wrapped[0],
        clamped[0]
    );
    assert!(
        wrapped[EXTENT - 1] < clamped[EXTENT - 1],
        "and lowers the bright one"
    );
    assert!(
        clamped.windows(2).all(|pair| pair[0] <= pair[1]),
        "clamped stays monotonic"
    );
    // Rows do not vary down the image, so the v axis does not care how it is sampled.
    let v_clamped = shading_grid(&ramp, SIDE, SIDE, true, false);
    assert_eq!(middle_row(&v_clamped), wrapped);
}

#[test]
fn red_weighs_more_than_blue() {
    let ramp = |channel: usize| {
        let mut rgba = vec![128_u8; SIDE as usize * SIDE as usize * 4];
        for (i, texel) in rgba.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            texel[channel] = u8::try_from(60 + (i % SIDE as usize) / 2).unwrap();
            texel[3] = 255;
        }
        shading_grid(&rgba, SIDE, SIDE, true, true)
    };
    assert!(furthest_from_neutral(&ramp(0)) > furthest_from_neutral(&ramp(2)));
}

#[test]
fn factor_lookup_is_bilinear_between_cell_centres() {
    let rgba = painted(SIDE, SIDE, |x, _| u8::try_from(40 + x * 3 / 4).unwrap());
    let wrapped = shading_grid(&rgba, SIDE, SIDE, true, true);
    let row = EXTENT / 2 * EXTENT;
    let centre = |cell: usize| (cell as f32 + 0.5) / EXTENT as f32;
    assert_eq!(
        wrapped.factor_at(centre(3), centre(EXTENT / 2)),
        wrapped.values()[row + 3]
    );
    // At u = 0, half way between the first cell and (wrapping) the last.
    let edge = wrapped.factor_at(0.0, centre(EXTENT / 2));
    let expected = f32::midpoint(wrapped.values()[row], wrapped.values()[row + EXTENT - 1]);
    assert!((edge - expected).abs() < 1e-6);
    assert_eq!(
        wrapped.factor_at(1.0 + centre(3), centre(EXTENT / 2)),
        wrapped.values()[row + 3],
        "repeats"
    );

    let clamped = shading_grid(&rgba, SIDE, SIDE, false, true);
    assert_eq!(
        clamped.factor_at(0.0, centre(EXTENT / 2)),
        clamped.values()[row],
        "held at the edge"
    );
    assert_eq!(ShadingGrid::neutral(true, true).factor_at(0.3, 0.7), 1.0);
}

#[test]
fn strength_zero_is_identity() {
    let original = noise(128, 64);
    let grid = shading_grid(&original, 128, 64, true, true);
    assert!(!grid.is_neutral());
    for strength in [0.0, -1.0, f32::NAN] {
        let mut rgba = original.clone();
        delight(&mut rgba, 128, 64, &grid, strength);
        assert_eq!(rgba, original, "strength {strength}");
    }
}

#[test]
fn a_flat_texture_is_unchanged_at_full_strength() {
    let original = painted(64, 64, |_, _| 137);
    let mut rgba = original.clone();
    delight(
        &mut rgba,
        64,
        64,
        &shading_grid(&original, 64, 64, true, true),
        1.0,
    );
    assert_eq!(rgba, original);
}

#[test]
fn alpha_is_preserved() {
    let original = noise(64, 64);
    let mut rgba = original.clone();
    delight(
        &mut rgba,
        64,
        64,
        &shading_grid(&original, 64, 64, true, true),
        1.0,
    );
    assert_ne!(rgba, original, "something changed");
    for (after, before) in rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(original.as_chunks::<4>().0)
    {
        assert_eq!(after[3], before[3]);
    }
}

#[test]
fn a_saturated_texel_in_a_dark_cell_keeps_its_hue() {
    // Divided by a factor below one, (255, 128, 0) goes past one in red; the whole colour is brought
    // back so the brightest channel is one, which returns it exactly.
    let mut rgba = painted(SIDE, SIDE, |x, _| if x < 128 { 30 } else { 200 });
    let at = (128 * SIDE as usize + 64) * 4;
    rgba[at..at + 3].copy_from_slice(&[255, 128, 0]);
    let grid = shading_grid(&rgba, SIDE, SIDE, true, true);
    assert!(grid.factor_at(64.5 / 256.0, 128.5 / 256.0) < 1.0);
    delight(&mut rgba, SIDE, SIDE, &grid, 1.0);
    assert_eq!(&rgba[at..at + 3], &[255, 128, 0]);
}

#[test]
fn a_painted_dark_corner_is_lifted_and_the_mean_kept() {
    // Grey 150 with a soft dark corner round (0, 0), clamped both ways (no wrap into the others).
    let side = 256_u32;
    let corner = |x: u32, y: u32| {
        let distance = f64::from(x * x + y * y).sqrt() / 96.0;
        let shade = if distance < 1.0 {
            0.3 + 0.7 * distance
        } else {
            1.0
        };
        encoded(shade * 0.3) // 0.3 linear is about byte 149
    };
    let original = painted(side, side, corner);
    let grid = shading_grid(&original, side, side, false, false);
    let at = |x: usize, y: usize| (y * side as usize + x) * 4;

    let mut half = original.clone();
    delight(&mut half, side, side, &grid, DEFAULT_STRENGTH);
    let mut full = original.clone();
    delight(&mut full, side, side, &grid, 1.0);

    let (before, half_c, full_c) = (original[at(4, 4)], half[at(4, 4)], full[at(4, 4)]);
    assert!(
        before < half_c && half_c < full_c,
        "corner: {before} -> {half_c} -> {full_c}"
    );
    let (before, after) = (original[at(200, 200)], full[at(200, 200)]);
    assert!(
        after <= before,
        "the lit area gives some back: {before} -> {after}"
    );

    let (mean_before, mean_after) = (mean_luminance(&original), mean_luminance(&full));
    let drift = (mean_after / mean_before - 1.0).abs();
    assert!(drift < 0.05, "mean luminance {mean_before} -> {mean_after}");
}

#[test]
#[should_panic(expected = "RGBA8 texels")]
fn a_short_buffer_is_refused() {
    let _ = shading_grid(&[0; 12], 2, 2, true, true);
}

#[test]
#[ignore = "measurement: ms for a 512x512 shading grid and apply"]
fn measure_512() {
    let original = noise(512, 512);
    let runs = 20_u32;
    let start = std::time::Instant::now();
    let mut grid = ShadingGrid::neutral(true, true);
    for _ in 0..runs {
        grid = shading_grid(std::hint::black_box(&original), 512, 512, true, true);
    }
    let grid_ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(runs);
    let mut rgba = original.clone();
    let start = std::time::Instant::now();
    for _ in 0..runs {
        rgba.copy_from_slice(&original);
        delight(std::hint::black_box(&mut rgba), 512, 512, &grid, 1.0);
    }
    let apply_ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(runs);
    println!(
        "512x512: shading_grid {grid_ms:.3} ms, delight {apply_ms:.3} ms, total {:.3} ms",
        grid_ms + apply_ms
    );
}
