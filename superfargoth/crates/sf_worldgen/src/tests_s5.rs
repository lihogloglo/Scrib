//! WG-S5 tests: the Veloren LEM port (drains to the sea, lakes resolved, conservation, determinism, fixed cells,
//! finite-time stop) and the rock-type map.
#![allow(clippy::float_cmp)] // exact comparisons are the point of some of these tests

use crate::lem::{self, BOUNDARY, LemParams, MfdLaw, Stop, route};
use crate::lithology::{RockMap, RockType, uplift_factor};
use crate::perturb;
use crate::{Audit, Grid, hash_f64};

const SEA: f64 = 0.0;
const LAW: MfdLaw = MfdLaw {
    base: 0.5,
    slope_coef: 0.6,
};

/// S4's fixture: disc island just above sea with a small hash ripple, sea at -50 m.
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

struct Fields {
    k: Vec<f64>,
    u: Vec<f64>,
    d: Vec<f64>,
    talus: Vec<f64>,
}

fn fields(g: &Grid, scale: f64) -> Fields {
    let n = g.len();
    let u: Vec<f64> = g.data.iter().map(|&v| if v >= SEA { 2.0 * scale } else { 0.0 }).collect();
    Fields {
        k: vec![0.5 * scale; n],
        d: u.iter().map(|u| u * 250.0).collect(),
        u,
        talus: vec![35f64.to_radians().tan(); n],
    }
}

fn params<'a>(f: &'a Fields, fixed: Option<&'a [bool]>, cap: usize, stop: Stop) -> LemParams<'a> {
    LemParams {
        m: 0.5,
        n: 1.0,
        k_dt: &f.k,
        uplift_dt: &f.u,
        diffusion_dt: &f.d,
        tan_talus: &f.talus,
        fixed,
        mfd: LAW,
        nonlinear_max_ratio: 10.0,
        sea_level: SEA,
        max_iterations: cap,
        stop,
    }
}

fn boundary_of(g: &Grid) -> Vec<bool> {
    (0..g.len()).map(|i| g.data[i] < SEA || g.is_border(i)).collect()
}

/// Every node reaches a boundary node along its single receiver, and the drainage area arriving at the boundary
/// is the whole grid's area.
fn assert_drains(g: &Grid, r: &lem::Routing, boundary: &[bool]) {
    let n = g.len();
    for i in 0..n {
        let mut v = i;
        for _ in 0..=n {
            if r.receiver[v] == BOUNDARY {
                break;
            }
            v = r.receiver[v] as usize;
        }
        assert!(boundary[v], "node {i} does not reach the boundary");
        if !boundary[i] {
            assert_ne!(r.mrec[i], 0, "land node {i} has no receiver");
        }
    }
    let cell = g.spacing * g.spacing;
    let out: f64 = (0..n).filter(|&i| boundary[i]).map(|i| r.area[i]).sum();
    let tot = n as f64 * cell;
    assert!((out / tot - 1.0).abs() < 1e-5, "area at the boundary {out} vs {tot}");
}

#[test]
fn lem_island_reaches_steady_state_drains_to_the_sea_and_keeps_classes() {
    let mut g = low_island(81, 40.0, 32.0);
    let before = g.data.clone();
    let f = fields(&g, 1.0);
    let cap = 600;
    let st = lem::evolve(&mut g, &params(&f, None, cap, Stop::Steady { rel: 0.05 })).expect("lem");
    assert!(st.converged, "not steady after {} steps: rel {}", st.iterations, st.residual_rel);
    assert!(Audit::compare(&before, &g.data, SEA).is_clean());
    let top = g.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(top.is_finite() && top > 10.0, "no relief emerged: {top}");
    assert!(g.data.iter().all(|v| v.is_finite()));
    let b = boundary_of(&g);
    let r = route(g.w, g.h, g.spacing, &g.data, &b, LAW).expect("route");
    assert_drains(&g, &r, &b);
    println!(
        "lem steady after {} steps, rel {:.5}, top {top:.1} m, lakes max {} (final {}), flooded max {}",
        st.iterations, st.residual_rel, st.lakes_max, r.lakes, st.lake_nodes_max
    );
}

#[test]
fn closed_basin_is_routed_over_its_pass_and_conserves_area() {
    // A south-tilted plane to the sea with a 20 m deep bowl in the middle: one lake, spilling south.
    let (w, h) = (41, 41);
    let mut g = Grid::filled(w, h, 30.0, 0.0);
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            g.data[i] = if y + 3 >= h {
                -10.0
            } else {
                let base = 2.0 + (h - y) as f64 * 1.5;
                let d = ((x as f64 - 20.0).hypot(y as f64 - 18.0) / 6.0).min(1.0);
                base - 20.0 * (1.0 - d * d)
            };
        }
    }
    let b = boundary_of(&g);
    let r = route(w, h, g.spacing, &g.data, &b, LAW).expect("route");
    assert!(r.lakes >= 1, "no lake found");
    assert!(r.lake_nodes > 0, "no flooded node re-routed");
    assert_drains(&g, &r, &b);
    // The bowl's bottom drains (through the lake surface) to the sea: its area ends up at the boundary.
    let bottom = 18 * w + 20;
    assert_ne!(r.mrec[bottom], 0);
}

#[test]
fn lem_is_deterministic_across_thread_counts() {
    let run_in = |threads: usize| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("pool")
            .install(|| {
                let mut g = low_island(61, 40.0, 24.0);
                let f = fields(&g, 1.0);
                lem::evolve(&mut g, &params(&f, None, 40, Stop::Steps)).expect("lem");
                hash_f64(&g.data)
            })
    };
    let one = run_in(1);
    assert_eq!(one, run_in(4));
    assert_eq!(one, run_in(1));
}

#[test]
fn fixed_cells_keep_their_height() {
    let mut g = low_island(61, 40.0, 24.0);
    let fixed: Vec<bool> = (0..g.len())
        .map(|i| {
            let (x, y) = (i % g.w, i / g.w);
            (26..34).contains(&x) && (26..34).contains(&y)
        })
        .collect();
    for (v, &fx) in g.data.iter_mut().zip(&fixed) {
        if fx {
            *v = 40.0;
        }
    }
    let before = g.data.clone();
    let f = fields(&g, 1.0);
    lem::evolve(&mut g, &params(&f, Some(&fixed), 30, Stop::Steps)).expect("lem");
    for i in 0..g.len() {
        if fixed[i] {
            assert_eq!(g.data[i].to_bits(), before[i].to_bits(), "anchor {i} moved");
        }
    }
    assert!(Audit::compare(&before, &g.data, SEA).is_clean());
}

#[test]
fn finite_time_stops_at_the_relief_target_and_newton_stays_sane() {
    let mut g = low_island(61, 40.0, 24.0);
    let f = fields(&g, 0.1);
    let cap = 2000;
    let target = 10.0;
    let st = lem::evolve(&mut g, &params(&f, None, cap, Stop::LandP95 { target_m: target }))
        .expect("lem");
    assert!(st.converged, "target not reached in {cap} steps: p95 {}", st.land_p95_m);
    assert!(st.iterations > 1 && st.iterations < cap);
    assert!(st.land_p95_m >= target);
    // A fixed step count runs exactly that many steps.
    let mut g2 = low_island(61, 40.0, 24.0);
    let st2 = lem::evolve(&mut g2, &params(&f, None, 7, Stop::Steps)).expect("lem");
    assert_eq!(st2.iterations, 7);
    assert!(!st2.converged);
    // n = 1.5 (local Newton): finite, land stays land.
    let mut g3 = low_island(61, 40.0, 24.0);
    let before = g3.data.clone();
    let mut p = params(&f, None, 30, Stop::Steps);
    p.n = 1.5;
    lem::evolve(&mut g3, &p).expect("lem");
    assert!(g3.data.iter().all(|v| v.is_finite()));
    assert!(Audit::compare(&before, &g3.data, SEA).is_clean());
    println!("finite stop after {} steps at p95 {:.2} m", st.iterations, st.land_p95_m);
}

#[test]
fn rock_map_is_deterministic_world_anchored_and_mixed() {
    let types = [
        RockType { k: 0.4, share: 0.3 },
        RockType { k: 1.0, share: 0.5 },
        RockType { k: 2.0, share: 0.2 },
    ];
    let map = RockMap {
        cell_m: 2000.0,
        jitter: 0.9,
        warp_m: 300.0,
        warp_wavelength_m: 3000.0,
        edge_band_m: 150.0,
        edge_k: 1.8,
        seed: 9,
    };
    let field_in = |threads: usize, ox: f64| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("pool")
            .install(|| {
                map.field(200, 150, |x, y| [ox + x as f64 * 50.0, 1000.0 - y as f64 * 50.0], &types)
            })
    };
    let (k1, t1) = field_in(1, 0.0);
    let (k4, t4) = field_in(4, 0.0);
    assert_eq!(hash_f64(&k1), hash_f64(&k4));
    assert_eq!(t1, t4);
    // Shift the grid by 10 samples: the overlap is the same world, so the same values.
    let (ks, _) = field_in(1, 500.0);
    for y in 0..150 {
        for x in 0..190 {
            assert_eq!(ks[y * 200 + x].to_bits(), k1[y * 200 + x + 10].to_bits());
        }
    }
    let mut seen = [false; 3];
    for (&k, &t) in k1.iter().zip(&t1) {
        seen[usize::from(t)] = true;
        let base = types[usize::from(t)].k;
        assert!(k >= base - 1e-12 && k <= base * 1.8 + 1e-12, "k {k} for type {t}");
    }
    assert!(seen.iter().all(|s| *s), "not every rock type appears: {seen:?}");
    // Edge bands exist: some samples sit above their type's base K.
    assert!(k1.iter().zip(&t1).any(|(&k, &t)| k > types[usize::from(t)].k * 1.01));
    let u = uplift_factor(64, 64, |x, y| [x as f64 * 100.0, y as f64 * 100.0], 0.3, 2000.0, 4);
    assert!(u.iter().all(|&v| (0.7 - 1e-12..=1.3 + 1e-12).contains(&v)));
}
