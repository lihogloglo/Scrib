//! Tests of the composable stratovolcano (WG-VO): profile monotone and on target, apron gradient in range,
//! determinism across runs and thread counts, coast mask kept, distance transform exact, profile fit recovers a
//! known profile.

use std::collections::BTreeMap;

use super::*;

/// The fixture's declared values: (name, value, unit, min, max). Fixture data (the framework never reads the
/// game's params file).
// tunables: const (test fixture)
const FIXTURE: &[(&str, f64, &str, f64, f64)] = &[
    ("seed", 1.0, "id", 0.0, 4_294_967_295.0),
    ("summit_m", 5500.0, "m", 1000.0, 9000.0),
    ("ref_quantile", 0.5, "ratio", 0.0, 1.0),
    ("foot_major_m", 12000.0, "m", 1000.0, 60000.0),
    ("foot_minor_m", 9000.0, "m", 1000.0, 60000.0),
    ("foot_major_bearing_deg", 160.0, "deg", 0.0, 360.0),
    ("foot_offset_m", 2000.0, "m", 0.0, 30000.0),
    ("foot_offset_bearing_deg", 160.0, "deg", 0.0, 360.0),
    ("foot_warp", 0.12, "ratio", 0.0, 0.4),
    ("foot_warp_wavelength_m", 9000.0, "m", 500.0, 100_000.0),
    ("domain_warp_m", 500.0, "m", 0.0, 5000.0),
    ("domain_warp_wavelength_m", 5000.0, "m", 500.0, 100_000.0),
    ("cone_exponent", 1.4, "ratio", 1.0, 4.0),
    ("cone_exponent_var", 0.18, "ratio", 0.0, 0.6),
    ("cone_exponent_wavelength_m", 14000.0, "m", 500.0, 100_000.0),
    ("crater_radius_m", 350.0, "m", 10.0, 5000.0),
    ("crater_depth_m", 120.0, "m", 0.0, 1000.0),
    ("blend_in", 0.8, "ratio", 0.0, 2.0),
    ("blend_out", 1.15, "ratio", 0.0, 3.0),
    ("coast_fade_m", 1500.0, "m", 0.0, 20000.0),
    ("apron_gradient", 0.02, "ratio", 0.005, 0.04),
    ("apron_max_m", 600.0, "m", 0.0, 3000.0),
    ("apron_inner_m", 18000.0, "m", 1000.0, 80000.0),
    ("apron_outer_m", 26000.0, "m", 1000.0, 100_000.0),
    ("apron_flatten", 0.8, "ratio", 0.0, 1.0),
    ("flatten_knee_m", 100.0, "m", 1.0, 2000.0),
    ("smooth_max_m", 60.0, "m", 0.0, 1000.0),
    ("caldera_radius_m", 4000.0, "m", 100.0, 30000.0),
    ("caldera_offset_m", 1000.0, "m", 0.0, 20000.0),
    ("caldera_offset_bearing_deg", 150.0, "deg", 0.0, 360.0),
    ("caldera_wall_m", 400.0, "m", 0.0, 2000.0),
    ("caldera_wall_width_m", 300.0, "m", 10.0, 5000.0),
    ("caldera_warp", 0.12, "ratio", 0.0, 0.5),
    ("caldera_warp_wavelength_m", 3000.0, "m", 100.0, 50000.0),
    ("caldera_open_bearing_deg", 330.0, "deg", 0.0, 360.0),
    ("caldera_open_half_deg", 50.0, "deg", 0.0, 180.0),
    ("caldera_open_fade_deg", 25.0, "deg", 1.0, 180.0),
    ("young_radius_m", 3200.0, "m", 100.0, 30000.0),
    ("young_exponent", 1.0, "ratio", 1.0, 4.0),
    ("secondary_distance_m", 3200.0, "m", 0.0, 30000.0),
    ("secondary_bearing_deg", 235.0, "deg", 0.0, 360.0),
    ("secondary_height_ratio", 0.82, "ratio", 0.0, 0.98),
    ("secondary_radius_m", 6000.0, "m", 100.0, 30000.0),
    ("secondary_exponent", 1.2, "ratio", 1.0, 4.0),
    ("secondary_crater_radius_m", 400.0, "m", 10.0, 3000.0),
    ("secondary_crater_depth_m", 100.0, "m", 0.0, 1000.0),
    ("subpeak_count", 4.0, "count", 0.0, 32.0),
    ("subpeak_r_min", 0.2, "ratio", 0.0, 1.5),
    ("subpeak_r_max", 0.55, "ratio", 0.0, 1.5),
    ("subpeak_h_min_m", 150.0, "m", 0.0, 3000.0),
    ("subpeak_h_max_m", 450.0, "m", 0.0, 3000.0),
    ("subpeak_radius_min_m", 700.0, "m", 50.0, 10000.0),
    ("subpeak_radius_max_m", 1500.0, "m", 50.0, 10000.0),
    ("subpeak_aspect", 1.8, "ratio", 1.0, 6.0),
    ("collapse_bearing_deg", 100.0, "deg", 0.0, 360.0),
    ("collapse_head", 0.18, "ratio", 0.0, 1.5),
    ("collapse_toe", 0.85, "ratio", 0.0, 2.0),
    ("collapse_width_head_m", 4000.0, "m", 100.0, 30000.0),
    ("collapse_width_toe_m", 5500.0, "m", 100.0, 30000.0),
    ("collapse_depth_m", 900.0, "m", 0.0, 3000.0),
    ("collapse_depth_fade", 0.45, "ratio", 0.0, 0.99),
    ("collapse_wall_m", 250.0, "m", 10.0, 5000.0),
    ("collapse_wall_warp_m", 180.0, "m", 0.0, 3000.0),
    ("collapse_wall_warp_wavelength_m", 1200.0, "m", 50.0, 50000.0),
    ("rift_count", 3.0, "count", 0.0, 3.0),
    ("rift_1_bearing_deg", 35.0, "deg", 0.0, 360.0),
    ("rift_2_bearing_deg", 195.0, "deg", 0.0, 360.0),
    ("rift_3_bearing_deg", 285.0, "deg", 0.0, 360.0),
    ("rift_start", 0.12, "ratio", 0.0, 2.0),
    ("rift_end", 1.25, "ratio", 0.0, 3.0),
    ("rift_taper", 0.2, "ratio", 0.01, 0.5),
    ("rift_curve_deg", 12.0, "deg", 0.0, 60.0),
    ("rift_curve_wavelength_m", 5000.0, "m", 200.0, 50000.0),
    ("rift_ridge_m", 120.0, "m", 0.0, 1000.0),
    ("rift_ridge_width_m", 900.0, "m", 10.0, 10000.0),
    ("rift_cones", 8.0, "count", 0.0, 64.0),
    ("rift_cone_lateral_m", 300.0, "m", 0.0, 5000.0),
    ("cone_width_min_m", 300.0, "m", 20.0, 5000.0),
    ("cone_width_max_m", 1200.0, "m", 20.0, 5000.0),
    ("cone_height_ratio", 0.18, "ratio", 0.01, 0.6),
    ("cone_crater_ratio", 0.40, "ratio", 0.01, 0.9),
    ("cone_crater_depth_ratio", 0.4, "ratio", 0.0, 1.0),
    ("cone_knee", 0.15, "ratio", 0.0, 0.45),
    ("ravine_steps", 60.0, "count", 0.0, 2000.0),
    ("ravine_k_dt", 0.02, "ratio", 0.0, 10.0),
    ("ravine_m", 0.5, "ratio", 0.3, 0.7),
    ("ravine_flow_exponent", 4.0, "ratio", 0.5, 10.0),
    ("ravine_seed_m", 6.0, "m", 0.0, 100.0),
    ("ravine_seed_wavelength_m", 400.0, "m", 20.0, 10000.0),
    ("ravine_seed_fine_m", 1.5, "m", 0.0, 50.0),
    ("ravine_seed_fine_wavelength_m", 60.0, "m", 10.0, 5000.0),
    ("flank_k_var", 0.7, "ratio", 0.0, 0.99),
    ("flank_k_wavelength_m", 12000.0, "m", 500.0, 100_000.0),
    ("band_thickness_m", 180.0, "m", 5.0, 5000.0),
    ("band_edge", 0.15, "ratio", 0.01, 1.0),
    ("band_k_var", 0.6, "ratio", 0.0, 0.99),
    ("band_warp_m", 60.0, "m", 0.0, 1000.0),
    ("cell_size_m", 1500.0, "m", 50.0, 20000.0),
    ("cell_k_var", 0.5, "ratio", 0.0, 0.99),
    ("fracture_count", 8.0, "count", 0.0, 64.0),
    ("fracture_start_min", 0.1, "ratio", 0.0, 2.0),
    ("fracture_start_max", 0.35, "ratio", 0.0, 2.0),
    ("fracture_len_min", 0.25, "ratio", 0.0, 2.0),
    ("fracture_len_max", 0.6, "ratio", 0.0, 2.0),
    ("fracture_angle_deg", 25.0, "deg", 0.0, 90.0),
    ("fracture_wander_wavelength_m", 2500.0, "m", 100.0, 50000.0),
    ("fracture_depth_m", 25.0, "m", 0.0, 500.0),
    ("fracture_width_m", 40.0, "m", 5.0, 1000.0),
    ("fracture_taper", 0.15, "ratio", 0.01, 0.5),
    ("fracture_k_boost", 2.0, "ratio", 0.0, 20.0),
    ("ravine_talus_deg", 40.0, "deg", 10.0, 80.0),
    ("ravine_thermal_steps", 30.0, "count", 0.0, 1000.0),
    ("ravine_thermal_rate", 0.25, "ratio", 0.01, 0.5),
    ("detail_spacing_m", 5.0, "m", 1.0, 100.0),
    ("detail_crop_m", 16000.0, "m", 1000.0, 100_000.0),
    ("fit_bin_m", 250.0, "m", 20.0, 5000.0),
    ("fit_max_r_m", 22000.0, "m", 1000.0, 100_000.0),
];

fn spec(features: Features, ravine_steps: f64) -> Spec {
    let mut params = BTreeMap::new();
    for &(n, v, unit, min, max) in FIXTURE {
        params.insert(
            n.to_string(),
            Param {
                v,
                unit: unit.into(),
                min,
                max,
                source: "Guess".into(),
                help: String::new(),
            },
        );
    }
    params.get_mut("ravine_steps").expect("declared").v = ravine_steps;
    let s = Spec {
        centre: CentreSpec {
            explicit_m: Some([0.0, 0.0]),
            ..CentreSpec::default()
        },
        reference: ReferenceSpec::default(),
        params,
        arms: vec![arm(features)],
    };
    s.validate().expect("fixture valid");
    s
}

const ALL: Features = Features {
    caldera: true,
    secondary: true,
    subpeaks: true,
    collapse: true,
    rifts: true,
    ravines: true,
};

/// Island radius, metres, of the synthetic base (fixture).
const ISLAND_M: f64 = 20000.0;

/// A circular island centred on (0, 0) at 100 m: flat land at 10 m (plus hash hills when `hills`), sea −30 m.
fn island(hills: bool) -> Field {
    let (n, sp) = (500usize, 100.0);
    let origin = [-(n as f64 - 1.0) * sp / 2.0, (n as f64 - 1.0) * sp / 2.0];
    let data = (0..n * n)
        .map(|i| {
            let p = [
                origin[0] + (i % n) as f64 * sp,
                origin[1] - (i / n) as f64 * sp,
            ];
            if len(p) > ISLAND_M {
                -30.0
            } else if hills {
                10.0 + 200.0 * (0.5 + 0.5 * value_noise(p, 4000.0, 99))
            } else {
                10.0
            }
        })
        .collect();
    Field {
        grid: Grid {
            w: n,
            h: n,
            spacing: sp,
            data,
        },
        origin_m: origin,
        sea_level_m: 0.0,
        fill_m: Some(-30.0),
    }
}

fn arm(features: Features) -> ArmSpec {
    ArmSpec {
        name: "T".into(),
        help: String::new(),
        features,
        detail_from: None,
    }
}

#[test]
fn v1_profile_is_monotone_and_hits_the_summit() {
    let s = spec(Features::default(), 0.0);
    let prep = prepare(&s, island(false)).expect("prepare");
    let mut shape = Shape::new(
        &s,
        Features::default(),
        prep.centre_m,
        0.0,
        prep.ref_m,
        prep.under_summit_m,
    );
    let target = s.v("summit_m");
    let reached = calibrate(&mut shape, &prep.base, &prep.d_coast, target);
    assert!((reached - target).abs() < 1e-2, "calibrated max {reached}");
    let (h, rep) = stamp_arm(&s, &prep, &arm(Features::default())).expect("stamp");
    assert!(
        (rep.max_m - target).abs() <= 0.01 * target,
        "max {}",
        rep.max_m
    );
    assert_eq!(h.len(), prep.base.grid.len());
    // Along rays from the summit, outside the crater (plus the warp), heights never rise.
    let r0 = s.v("crater_radius_m") + 2.0 * s.v("domain_warp_m");
    for k in 0..32 {
        let d = dir(TAU * f64::from(k) / 32.0);
        let mut last = f64::INFINITY;
        let mut r = r0;
        while r < 30000.0 {
            let p = [r * d[0], r * d[1]];
            let dc = (ISLAND_M - r).max(0.0);
            let (v, _) = shape.surface(p, 10.0, dc);
            assert!(v <= last + 1e-6, "ray {k}: rises at r {r}: {last} -> {v}");
            last = v;
            r += 25.0;
        }
    }
}

#[test]
fn apron_gradient_within_declared_range() {
    let s = spec(Features::default(), 0.0);
    let prep = prepare(&s, island(false)).expect("prepare");
    let (_, rep) = stamp_arm(&s, &prep, &arm(Features::default())).expect("stamp");
    let p = &s.params["apron_gradient"];
    assert!(rep.apron_rays > 0, "no usable apron ray");
    assert!(
        rep.apron_gradient >= p.min && rep.apron_gradient <= p.max,
        "apron gradient {} outside [{}, {}]",
        rep.apron_gradient,
        p.min,
        p.max
    );
    assert!(
        (rep.apron_gradient / p.v - 1.0).abs() < 0.15,
        "apron gradient {} vs declared {}",
        rep.apron_gradient,
        p.v
    );
}

#[test]
fn deterministic_across_runs_and_threads_and_keeps_the_coast() {
    let s = spec(ALL, 8.0);
    let run = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("pool");
        pool.install(|| {
            let prep = prepare(&s, island(true)).expect("prepare");
            stamp_arm(&s, &prep, &arm(ALL)).expect("stamp").1
        })
    };
    let (a, b, c) = (run(1), run(4), run(4));
    assert_eq!(a.heights_blake3, b.heights_blake3, "1 vs 4 threads");
    assert_eq!(b.heights_blake3, c.heights_blake3, "two runs");
    assert!(a.audit.is_clean(), "wet/dry audit {:?}", a.audit);
    let r = a.ravines.expect("ravines ran");
    assert_eq!(r.steps, 8);
    assert!(r.incision_max_m.is_finite() && r.incision_max_m >= 0.0);
    assert!(
        (a.max_m - s.v("summit_m")).abs() <= 0.02 * s.v("summit_m"),
        "max {}",
        a.max_m
    );
}

#[test]
fn every_feature_arm_keeps_the_coast_and_changes_the_shape() {
    let s = spec(Features::default(), 0.0);
    let prep = prepare(&s, island(true)).expect("prepare");
    let none = Features::default();
    let mut hashes = Vec::new();
    for f in [
        none,
        Features {
            caldera: true,
            ..none
        },
        Features {
            secondary: true,
            ..none
        },
        Features {
            subpeaks: true,
            ..none
        },
        Features {
            collapse: true,
            ..none
        },
        Features {
            rifts: true,
            ..none
        },
    ] {
        let (h, rep) = stamp_arm(&s, &prep, &arm(f)).expect("stamp");
        assert!(rep.audit.is_clean(), "{f:?}: audit {:?}", rep.audit);
        assert!(h.iter().all(|v| v.is_finite()), "{f:?}: non-finite height");
        hashes.push(rep.heights_blake3);
    }
    let mut uniq = hashes.clone();
    uniq.sort();
    uniq.dedup();
    assert_eq!(uniq.len(), hashes.len(), "a feature switch changed nothing");
}

#[test]
fn coast_distance_matches_brute_force() {
    let (w, h) = (37usize, 23usize);
    let wet: Vec<bool> = (0..w * h).map(|i| u01(i as i64, 0, 5) < 0.07).collect();
    let d = coast_distance(&wet, w, h, 2.0);
    for (i, &di) in d.iter().enumerate() {
        let (x, y) = ((i % w) as f64, (i / w) as f64);
        let best = (0..w * h)
            .filter(|&j| wet[j])
            .map(|j| ((j % w) as f64 - x).hypot((j / w) as f64 - y))
            .fold(f64::INFINITY, f64::min);
        assert!(
            (di - 2.0 * best).abs() < 1e-9,
            "sample {i}: {} vs {}",
            di,
            2.0 * best
        );
    }
}

#[test]
fn fit_recovers_a_known_profile() {
    let (n, sp) = (400usize, 50.0);
    let origin = [-(n as f64 - 1.0) * sp / 2.0, (n as f64 - 1.0) * sp / 2.0];
    let (rb, p, g) = (6000.0, 1.7, 0.02);
    let data = (0..n * n)
        .map(|i| {
            let r = len([
                origin[0] + (i % n) as f64 * sp,
                origin[1] - (i / n) as f64 * sp,
            ]);
            if r < rb {
                300.0 + 2000.0 * (1.0 - r / rb).powf(p)
            } else {
                300.0 - g * (r - rb)
            }
        })
        .collect();
    let f = Field {
        grid: Grid {
            w: n,
            h: n,
            spacing: sp,
            data,
        },
        origin_m: origin,
        sea_level_m: -1e9,
        fill_m: None,
    };
    let fit = fit_profile(&radial_profile(&f, [0.0, 0.0], 250.0, 9500.0)).expect("fit");
    assert!((fit.power_p - p).abs() < 0.15, "p {}", fit.power_p);
    assert!((fit.break_r_m - rb).abs() <= 500.0, "break {}", fit.break_r_m);
    assert!(
        (fit.apron_gradient - g).abs() < 0.003,
        "apron {}",
        fit.apron_gradient
    );
}

#[test]
fn spec_rejects_unknown_and_missing_params() {
    let mut s = spec(Features::default(), 0.0);
    s.params.insert(
        "nonsense".into(),
        Param {
            v: 1.0,
            unit: "m".into(),
            min: 0.0,
            max: 2.0,
            source: "Guess".into(),
            help: String::new(),
        },
    );
    assert!(s.validate().is_err());
    s.params.remove("nonsense");
    s.params.remove("summit_m");
    assert!(s.validate().is_err());
}
