//! WG-S4 tests: layout low-pass, steady-state landscape, volcano profile, determinism, structure sanity.
#![allow(clippy::float_cmp)] // exact comparisons are the point of some of these tests

use crate::landscape::{LandscapeParams, evolve};
use crate::layout::{gaussian, layout, sigma_for_cutoff};
use crate::perturb;
use crate::structure::structure;
use crate::volcano::{Strato, blend_weight};
use crate::{Audit, Grid, hash_f64};

const SEA: f64 = 0.0;

/// Disc island of radius `r_cells` on a `n` × `n` grid: land just above sea with a small hash ripple, sea at -50 m.
fn low_island(n: usize, spacing: f64, r_cells: f64) -> Grid {
    let mut g = Grid::filled(n, n, spacing, -50.0);
    let c = (n - 1) as f64 / 2.0;
    for y in 0..n {
        for x in 0..n {
            let (dx, dy) = (x as f64 - c, y as f64 - c);
            if dx.hypot(dy) <= r_cells {
                let p = [x as f64 * spacing, y as f64 * spacing];
                g.data[y * n + x] = 1.0 + perturb::value_noise(p, 2.0 * spacing, 7).abs();
            }
        }
    }
    g
}

fn run_island(g: &mut Grid, cap: usize) -> crate::landscape::LandscapeStats {
    let n = g.len();
    let land: Vec<bool> = g.data.iter().map(|&v| v >= SEA).collect();
    let u: Vec<f64> = land.iter().map(|&l| if l { 2.0 } else { 0.0 }).collect();
    let d: Vec<f64> = u.iter().map(|u| u * 250.0).collect();
    let k = vec![0.5; n];
    let talus = vec![35f64.to_radians().tan(); n];
    evolve(
        g,
        &LandscapeParams {
            m: 0.5,
            k_dt: &k,
            uplift_dt: &u,
            diffusion_dt: &d,
            tan_talus: &talus,
            flow_exponent: 1.1,
            sea_level: SEA,
            max_iterations: cap,
            steady_rel: 0.05,
        },
    )
    .expect("evolve")
}

#[test]
fn layout_lowpass_keeps_a_constant_constant_and_the_coast_mask() {
    let c = Grid::filled(57, 43, 20.0, 123.25);
    let f = gaussian(&c, sigma_for_cutoff(1500.0));
    for &v in &f.data {
        assert!((v - 123.25).abs() < 1e-9, "constant drifted to {v}");
    }
    let src = low_island(65, 20.0, 25.0);
    let lay = layout(&src, SEA, sigma_for_cutoff(400.0));
    for i in 0..src.len() {
        assert_eq!(lay.land[i], src.data[i] >= SEA, "mask changed at {i}");
        if lay.land[i] {
            assert!(lay.relief.data[i] >= 0.0);
        } else {
            assert_eq!(lay.relief.data[i], 0.0);
            assert!(lay.sea_floor[i] < SEA, "sea floor reached sea level at {i}");
        }
    }
    // Cutoff relation: amplitude 1/2 at the cutoff wavelength.
    let sigma = sigma_for_cutoff(1000.0);
    let amp = (-2.0 * std::f64::consts::PI.powi(2) * sigma * sigma / 1e6).exp();
    assert!((amp - 0.5).abs() < 1e-12);
}

#[test]
fn steady_state_landscape_converges_and_keeps_classes() {
    let mut g = low_island(81, 40.0, 32.0);
    let before = g.data.clone();
    let cap = 600;
    let st = run_island(&mut g, cap);
    assert!(st.converged, "not steady after {} steps: rel {}", st.iterations, st.residual_rel);
    assert!(st.iterations < cap);
    assert_eq!(st.pits_after_routing, 0);
    assert!(Audit::compare(&before, &g.data, SEA).is_clean());
    let top = g.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(top.is_finite() && top > 10.0, "no relief emerged: {top}");
    println!(
        "steady after {} steps, rel {:.5}, top {top:.1} m, D dt/dx2 {:.2}",
        st.iterations, st.residual_rel, st.diffusion_ratio_max
    );
}

#[test]
fn volcano_profile_is_monotone_outside_the_crater_and_hits_its_summit() {
    let v = Strato {
        height_m: 3300.0,
        radius_m: 17000.0,
        decay_m: 6000.0,
        crater_radius_m: 400.0,
        crater_depth_m: 150.0,
    };
    assert!((v.profile(v.crater_radius_m) - 3300.0).abs() < 1e-9);
    assert!((v.profile(0.0) - 3150.0).abs() < 1e-9);
    assert_eq!(v.profile(v.radius_m), 0.0);
    let mut prev = v.profile(v.crater_radius_m);
    let mut r = v.crater_radius_m;
    while r < v.radius_m + 2000.0 {
        r += 5.0;
        let h = v.profile(r);
        assert!(h <= prev, "profile rises at r = {r}: {prev} -> {h}");
        assert!(h >= 0.0);
        prev = h;
    }
    // Inside the crater: rises from the floor to the rim.
    let mut prev = v.profile(0.0);
    let mut r = 0.0;
    while r < v.crater_radius_m {
        r += 5.0;
        let h = v.profile(r.min(v.crater_radius_m));
        assert!(h >= prev);
        prev = h;
    }
    assert_eq!(blend_weight(0.1, 0.6, 1.0), 1.0);
    assert_eq!(blend_weight(1.0, 0.6, 1.0), 0.0);
    let mid = blend_weight(0.8, 0.6, 1.0);
    assert!((mid - 0.5).abs() < 1e-12);
}

#[test]
fn landscape_and_layout_are_deterministic_across_thread_counts() {
    let run_in = |threads: usize| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("pool")
            .install(|| {
                let mut g = low_island(61, 40.0, 24.0);
                run_island(&mut g, 80);
                let f = gaussian(&g, 300.0);
                (hash_f64(&g.data), hash_f64(&f.data))
            })
    };
    let one = run_in(1);
    assert_eq!(one, run_in(4));
    assert_eq!(one, run_in(1));
}

#[test]
fn parallel_grooves_score_a_high_coherence() {
    // A south-tilted plane cut by parallel north-south V grooves every 8 samples, sea along the south edge.
    let (w, h) = (96, 96);
    let mut g = Grid::filled(w, h, 20.0, -10.0);
    for y in 0..h - 4 {
        for x in 0..w {
            let groove = ((x % 8) as f64 - 4.0).abs();
            g.data[y * w + x] = 5.0 + (h - y) as f64 * 2.0 + groove * 3.0;
        }
    }
    let s = structure(&g, SEA, 20.0 * 20.0 * 20.0, 400.0, 5);
    assert!(s.channel_samples > 0);
    assert!(s.coherence_mean > 0.9, "coherence {}", s.coherence_mean);
    assert!(s.drainage_density > 0.0);
}
