//! Tests of the runtime-detail erosion filter ([`super`]): exactness, flatness, determinism, tiling, the bound,
//! the wet/dry class, declared values.

use super::*;

/// A synthetic 96 × 96 base at 20 m: a sloping coast (sea in the south-west corner) with hills.
fn hills() -> (Vec<f32>, usize, usize) {
    let (w, h) = (96usize, 96usize);
    let mut v = Vec::with_capacity(w * h);
    for r in 0..h {
        for c in 0..w {
            let (x, y) = (c as f64 * 20.0, -(r as f64) * 20.0);
            let z = 0.25 * x + 0.2 * (y + 1900.0) - 120.0
                + 180.0 * (x / 310.0).sin() * (y / 270.0).cos()
                + 60.0 * ((x + y) / 130.0).sin();
            v.push(z as f32);
        }
    }
    (v, w, h)
}

fn field(heights: &[f32], w: usize, h: usize) -> BaseField<'_> {
    BaseField {
        w,
        h,
        spacing_m: 20.0,
        origin_m: [0.0, 0.0],
        heights,
    }
}

fn params_with(name: &str, v: f64) -> FilterParams {
    let mut d = declared_defaults();
    set(&mut d, name, v, None).unwrap();
    FilterParams::from_declared(&d).unwrap()
}

#[test]
fn zero_amplitude_returns_bicubic_base() {
    let (hs, w, h) = hills();
    let b = field(&hs, w, h);
    let lat = Lattice::new(&b, 5.0, None).unwrap();
    for prm in [params_with("max_amplitude_m", 0.0), params_with("relief_fraction", 0.0)] {
        let (base, out) = evaluate(&b, 5.0, &lat, 0.0, &prm);
        for j in 0..lat.h {
            for i in 0..lat.w {
                let want = bicubic_with(i as f64 * 0.25, j as f64 * 0.25, |c, r| b.at(c, r)) as f32;
                assert_eq!(out[j * lat.w + i].to_bits(), want.to_bits());
                assert_eq!(base[j * lat.w + i].to_bits(), want.to_bits());
            }
        }
    }
}

#[test]
fn flat_input_stays_flat() {
    let hs = vec![137.5f32; 32 * 32];
    let b = field(&hs, 32, 32);
    let lat = Lattice::new(&b, 3.0, None).unwrap();
    let (_, out) = evaluate(&b, 3.0, &lat, 0.0, &FilterParams::default());
    assert!(out.iter().all(|v| v.to_bits() == 137.5f32.to_bits()));
}

#[test]
fn deterministic_across_runs_and_threads() {
    let (hs, w, h) = hills();
    let b = field(&hs, w, h);
    let lat = Lattice::new(&b, 5.0, Some([200.0, -1500.0, 1500.0, -200.0])).unwrap();
    let prm = FilterParams::default();
    let run = |n: usize| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build()
            .unwrap()
            .install(|| evaluate(&b, 5.0, &lat, 0.0, &prm).1)
    };
    let one = run(1);
    let four = run(4);
    let again = run(4);
    let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(&one), bits(&four));
    assert_eq!(bits(&four), bits(&again));
    // The filter does something on these slopes.
    assert!(delta_stats(&evaluate(&b, 5.0, &lat, 0.0, &prm).0, &one, 0.0).mean_abs_m > 0.1);
}

#[test]
fn tile_split_shares_edges_bitwise() {
    let (hs, w, h) = hills();
    let b = field(&hs, w, h);
    let prm = FilterParams::default();
    // Crops sharing the column at x = 900 m and the row at y = -900 m.
    let whole = Lattice::new(&b, 5.0, Some([400.0, -1400.0, 1400.0, -400.0])).unwrap();
    let west = Lattice::new(&b, 5.0, Some([400.0, -1400.0, 900.0, -400.0])).unwrap();
    let east = Lattice::new(&b, 5.0, Some([900.0, -1400.0, 1400.0, -400.0])).unwrap();
    let south = Lattice::new(&b, 5.0, Some([400.0, -1400.0, 1400.0, -900.0])).unwrap();
    assert_eq!(west.col0 + west.w - 1, east.col0);
    let (_, ow) = evaluate(&b, 5.0, &whole, 0.0, &prm);
    let (_, a) = evaluate(&b, 5.0, &west, 0.0, &prm);
    let (_, e) = evaluate(&b, 5.0, &east, 0.0, &prm);
    let (_, s) = evaluate(&b, 5.0, &south, 0.0, &prm);
    for j in 0..whole.h {
        let shared_a = a[j * west.w + west.w - 1].to_bits();
        let shared_e = e[j * east.w].to_bits();
        assert_eq!(shared_a, shared_e);
        assert_eq!(shared_a, ow[j * whole.w + west.w - 1].to_bits());
    }
    // Every sample of the south tile equals the whole grid's (row offset only).
    let dr = south.row0 - whole.row0;
    for j in 0..south.h {
        for i in 0..south.w {
            assert_eq!(
                s[j * south.w + i].to_bits(),
                ow[(j + dr) * whole.w + i].to_bits()
            );
        }
    }
}

#[test]
fn change_bounded_by_declared_amplitude() {
    let (hs, w, h) = hills();
    let b = field(&hs, w, h);
    let lat = Lattice::new(&b, 5.0, None).unwrap();
    for (cap, frac) in [(20.0, 0.25), (3.0, 2.0), (50.0, 2.0)] {
        let mut d = declared_defaults();
        set(&mut d, "max_amplitude_m", cap, None).unwrap();
        set(&mut d, "relief_fraction", frac, None).unwrap();
        let prm = FilterParams::from_declared(&d).unwrap();
        let (base, out) = evaluate(&b, 5.0, &lat, 0.0, &prm);
        let st = delta_stats(&base, &out, 0.0);
        // f32 rounding of the sum: a few ulps of the height.
        assert!(st.max_abs_m <= cap * (1.0 + 1e-5) + 1e-3, "{cap}: {st:?}");
        assert!(st.max_abs_m > 0.0);
        assert_eq!((st.new_wet, st.new_dry), (0, 0), "wet/dry class kept");
        assert!(out.iter().all(|v| v.is_finite()));
    }
}

#[test]
fn sea_points_never_move() {
    let (hs, w, h) = hills();
    let b = field(&hs, w, h);
    let lat = Lattice::new(&b, 5.0, None).unwrap();
    let (base, out) = evaluate(&b, 5.0, &lat, 0.0, &params_with("max_amplitude_m", 100.0));
    let wet = base.iter().filter(|v| **v < 0.0).count();
    assert!(wet > 100, "fixture has sea");
    assert!(base
        .iter()
        .zip(&out)
        .all(|(b, o)| *b >= 0.0 || b.to_bits() == o.to_bits()));
}

#[test]
fn declared_values_parse_and_check() {
    let mut d = declared_defaults();
    apply_overrides_toml(
        &mut d,
        "octaves = 3\nwavelength_m = 120.0\ncarve = { v = 0.25, source = \"Measured(test)\" }\n",
    )
    .unwrap();
    let p = FilterParams::from_declared(&d).unwrap();
    assert_eq!(p.octaves, 3);
    assert_eq!(p.wavelength_m.to_bits(), 120.0f32.to_bits());
    assert_eq!(p.carve.to_bits(), 0.25f32.to_bits());
    assert_eq!(d.iter().find(|x| x.name == "carve").unwrap().source, "Measured(test)");
    assert!(apply_overrides_toml(&mut declared_defaults(), "nope = 1").is_err());
    let mut bad = declared_defaults();
    set(&mut bad, "octaves", 2.5, None).unwrap();
    assert!(FilterParams::from_declared(&bad).is_err());
    let mut bad = declared_defaults();
    set(&mut bad, "carve", 3.0, None).unwrap();
    assert!(FilterParams::from_declared(&bad).is_err());
}

#[test]
fn derivative_weights_match_finite_differences() {
    let e = 1e-6;
    for t in [0.0, 0.3, 0.71, 1.0] {
        let cr = catmull_rom_derivative(t);
        let bd = bspline_derivative(t);
        let b2 = bspline_second(t);
        let (cp, cm) = (catmull_rom_weights(t + e), catmull_rom_weights(t - e));
        let (bp, bm) = (bspline_weights(t + e), bspline_weights(t - e));
        let (dp, dm) = (bspline_derivative(t + e), bspline_derivative(t - e));
        for i in 0..4 {
            assert!((cr[i] - (cp[i] - cm[i]) / (2.0 * e)).abs() < 1e-6);
            assert!((bd[i] - (bp[i] - bm[i]) / (2.0 * e)).abs() < 1e-6);
            assert!((b2[i] - (dp[i] - dm[i]) / (2.0 * e)).abs() < 1e-6);
        }
    }
}

#[test]
#[allow(clippy::float_cmp)]
fn sharp_operators_are_bounded_zero_below_sea_and_nonzero_on_slopes() {
    let prm = FilterParams::default().with_sharp_defaults();
    let max_b = prm.max_total_amplitude_m();
    assert!(max_b > prm.max_amplitude_m);

    // Below sea: strictly 0.
    let sea_pt = BaseSample { h: -5.0, grad: [0.4, 0.3], plan_curvature: 0.0 };
    assert_eq!(detail_offset([123.4, 567.8], &sea_pt, 0.0, &prm), 0.0);

    // Flat ground: strictly 0.
    let flat_pt = BaseSample { h: 120.0, grad: [0.0, 0.0], plan_curvature: 0.0 };
    assert_eq!(detail_offset([123.4, 567.8], &flat_pt, 0.0, &prm), 0.0);

    // Steep slope: non-zero, differs from gully-only baseline, and stays inside conservative bound.
    let gully_only = FilterParams { relief_fraction: prm.relief_fraction, ..FilterParams::default() };
    let mut diffs = 0usize;
    for i in 0..20 {
        let p = [f64::from(i) * 17.5, f64::from(i) * 11.25];
        let slope_pt = BaseSample { h: 180.0 + f64::from(i) * 4.7, grad: [0.45, 0.25], plan_curvature: 0.001 };
        let d_sharp = detail_offset(p, &slope_pt, 0.0, &prm);
        let d_base = detail_offset(p, &slope_pt, 0.0, &gully_only);
        assert!(d_sharp.abs() <= max_b + 1e-4, "|{d_sharp}| > {max_b}");
        if (d_sharp - d_base).abs() > 0.2 {
            diffs += 1;
        }
    }
    assert!(diffs >= 10, "expected sharp operators to move steep samples, got {diffs}/20");
}
