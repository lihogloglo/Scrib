#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests

use std::fmt::Write as _;

use crate::flood::priority_flood_eps;
use crate::resample::resample;
use crate::shade::{Light, hillshade};
use crate::stream::{StreamParams, erode, raw_pits, route};
use crate::thermal::{ThermalParams, relax};
use crate::{Audit, Grid, WetDry, hash_f64};

const SEA: f64 = 0.0;

/// Cone island: 300 m summit, shoreline at ~37 cells from the centre, a few dug pits on its flanks.
fn cone_island() -> Grid {
    let (w, h) = (97, 97);
    let mut g = Grid::filled(w, h, 20.0, 0.0);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 - 48.0, y as f64 - 48.0);
            let r = (dx * dx + dy * dy).sqrt();
            // Gentle deterministic ripple so the flanks are not perfectly radial.
            let ripple = 3.0 * ((x * 7 + y * 13) % 5) as f64;
            g.data[y * w + x] = 320.0 * (1.0 - r / 40.0) - 20.0 + ripple;
        }
    }
    for &(x, y) in &[(30usize, 40usize), (60, 55), (48, 30), (52, 66)] {
        let i = y * w + x;
        g.data[i] -= 40.0;
    }
    g
}

fn wet_of(g: &Grid) -> Vec<bool> {
    g.data.iter().map(|&v| v < SEA).collect()
}

fn params<'a>(k: &'a [f64], restore: &'a [f64]) -> StreamParams<'a> {
    StreamParams {
        m: 0.5,
        iterations: 20,
        k_dt: k,
        restore,
        sea_level: SEA,
    }
}

#[test]
fn cone_island_drains_to_the_sea_with_no_pits_after_routing() {
    let g = cone_island();
    let wet = wet_of(&g);
    assert!(raw_pits(&g, &wet) >= 4, "fixture has dug pits");
    let mut z = Vec::new();
    let r = route(&g, &wet, SEA, &mut z);
    assert_eq!(r.pits, 0);
    assert_eq!(r.unrouted, 0);
    // Every dry sample's receiver chain ends on a wet sample, strictly descending on the routing surface.
    for start in 0..g.len() {
        if wet[start] {
            continue;
        }
        let mut i = start;
        let mut steps = 0;
        while !wet[i] {
            let next = r.receiver[i] as usize;
            assert_ne!(next, i, "dry sample {i} has no receiver");
            assert!(z[next] < z[i]);
            i = next;
            steps += 1;
            assert!(steps < g.len());
        }
    }
    // The whole island's area reaches the sea.
    let dry = wet.iter().filter(|w| !**w).count() as f64 * 400.0;
    let into_sea: f64 = (0..g.len())
        .filter(|&i| !wet[i] && wet[r.receiver[i] as usize])
        .map(|i| r.area[i])
        .sum();
    assert_eq!(into_sea, dry);
}

#[test]
fn flood_raises_only_pits() {
    let g = cone_island();
    let wet = wet_of(&g);
    let mut z = Vec::new();
    priority_flood_eps(g.w, g.h, &g.data, &wet, SEA, &mut z);
    for i in 0..g.len() {
        if wet[i] {
            assert_eq!(z[i], SEA);
        } else {
            assert!(z[i] >= g.data[i]);
        }
    }
    let raised = (0..g.len())
        .filter(|&i| !wet[i] && z[i] > g.data[i])
        .count();
    assert!(raised >= 4);
}

#[test]
fn erosion_keeps_sea_fixed_and_land_dry() {
    let mut g = cone_island();
    let start = g.clone();
    let n = g.len();
    let (k, restore) = (vec![0.02; n], vec![0.05; n]);
    let stats = erode(&mut g, &start.data, &params(&k, &restore)).unwrap();
    let audit = Audit::compare(&start.data, &g.data, SEA);
    assert!(audit.is_clean(), "{audit:?}");
    assert_eq!(WetDry::count(&start.data, SEA), WetDry::count(&g.data, SEA));
    assert_eq!(stats.pits_after_routing, 0);
    assert!(stats.max_incision_m > 0.0, "something eroded");
    assert!(g.data.iter().all(|v| v.is_finite()));
}

#[test]
fn erosion_is_deterministic_across_runs_and_thread_counts() {
    let base = cone_island();
    let n = base.len();
    let (k, restore) = (vec![0.02; n], vec![0.05; n]);
    let run_in = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        pool.install(|| {
            let mut g = base.clone();
            erode(&mut g, &base.data, &params(&k, &restore)).unwrap();
            let tan = vec![0.6; n];
            relax(
                &mut g,
                &ThermalParams {
                    iterations: 5,
                    rate: 0.25,
                    tan_talus: &tan,
                    sea_level: SEA,
                },
            )
            .unwrap();
            hash_f64(&g.data)
        })
    };
    let a = run_in(1);
    assert_eq!(a, run_in(1));
    assert_eq!(a, run_in(4));
}

#[test]
fn thermal_keeps_classes_and_mass() {
    let mut g = cone_island();
    let start = g.clone();
    let tan = vec![0.3; g.len()];
    let moved = relax(
        &mut g,
        &ThermalParams {
            iterations: 10,
            rate: 0.5,
            tan_talus: &tan,
            sea_level: SEA,
        },
    )
    .unwrap();
    assert!(moved > 0.0);
    assert!(Audit::compare(&start.data, &g.data, SEA).is_clean());
    let mass = |g: &Grid| g.data.iter().filter(|v| **v >= SEA).sum::<f64>();
    let (m0, m1) = (mass(&start), mass(&g));
    assert!((m0 - m1).abs() <= 1e-9 * m0.abs(), "{m0} vs {m1}");
}

#[test]
fn resample_of_a_constant_stays_constant() {
    let src = Grid::filled(40, 33, 1.8288, 12.345_678);
    for spacing in [0.7, 1.0, 2.0, 1.8288 * 0.37] {
        let out = resample(&src, spacing);
        assert!(out.w > 1 && out.h > 1);
        assert!(
            out.data.iter().all(|&v| v == 12.345_678),
            "spacing {spacing}"
        );
    }
}

#[test]
fn resample_hits_the_source_samples() {
    let mut src = Grid::filled(16, 16, 2.0, 0.0);
    for (i, v) in src.data.iter_mut().enumerate() {
        *v = ((i * 37) % 11) as f64;
    }
    let out = resample(&src, 2.0);
    assert_eq!(out.data, src.data);
}

#[test]
fn hillshade_of_flat_land_is_the_altitude_sine() {
    let g = Grid::filled(8, 8, 20.0, 5.0);
    let img = hillshade(
        &g,
        SEA,
        Light {
            azimuth_deg: 315.0,
            altitude_deg: 45.0,
            sea_gray: 0,
        },
    );
    let expect = (45f64.to_radians().sin() * 255.0).round() as u8;
    assert!(img.px.iter().all(|&p| p == expect));
    let sea = Grid::filled(4, 4, 20.0, -1.0);
    let img = hillshade(
        &sea,
        SEA,
        Light {
            azimuth_deg: 315.0,
            altitude_deg: 45.0,
            sea_gray: 17,
        },
    );
    assert!(img.px.iter().all(|&p| p == 17));
}

#[test]
fn recipe_refuses_out_of_range_values() {
    let p =
        |v: f64| format!("{{ v = {v}, unit = \"ratio\", min = 0, max = 1, source = \"Guess\" }}");
    let text = |scale: f64| {
        let mut s = String::from("[guide]\ndir = \"g\"\n[output]\ndir = \"o\"\n[params]\n");
        for name in [
            "macro_spacing_m",
            "sea_margin_m",
            "stream_m",
            "stream_k_dt",
            "stream_restore",
            "stream_iterations",
            "thermal_iterations",
            "thermal_rate",
            "light_azimuth_deg",
            "light_altitude_deg",
            "sea_gray",
            "overview_max_px",
            "crop_size_m",
            "crop_px",
        ] {
            let _ = writeln!(s, "{name} = {}", p(0.5));
        }
        let _ = writeln!(s, "scale = {}", p(scale));
        s += "[region_params]\nrelief = { unit = \"ratio\", min = 0, max = 20, source = \"Guess\" }\n";
        s += "[regions.default]\nerodibility = 1\nuplift = 1\nrelief = 10\ntalus_deg = 30\n";
        s += "[[arm]]\nname = \"A0\"\nregional = false\nerosion = false\nthermal = false\n";
        s
    };
    let undeclared: crate::recipe::Recipe = toml::from_str(&text(0.5)).unwrap();
    // Region columns other than `relief` are undeclared here, so validation must refuse them.
    assert!(undeclared.validate().is_err());
    let bad: crate::recipe::Recipe = toml::from_str(&text(2.0)).unwrap();
    assert!(bad.validate().unwrap_err().to_string().contains("scale"));
}
