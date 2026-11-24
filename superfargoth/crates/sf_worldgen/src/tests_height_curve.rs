// Exact maths: identity and zero are bit-exact by construction.
#![allow(clippy::float_cmp)]

use super::*;

fn curves() -> Vec<HeightCurve> {
    vec![
        HeightCurve::default(),
        HeightCurve {
            land: Branch { slope: 0.5, knee_m: 50.0 },
            water: Branch { slope: 1.0, knee_m: 1000.0 },
        },
        HeightCurve {
            land: Branch { slope: 0.01, knee_m: 2000.0 },
            water: Branch { slope: 0.05, knee_m: 10.0 },
        },
    ]
}

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * a.abs().max(b.abs()).max(1.0)
}

#[test]
fn zero_maps_to_zero() {
    for c in curves() {
        assert_eq!(c.apply(0.0), 0.0);
        assert_eq!(c.apply_f32(0.0), 0.0);
    }
}

#[test]
fn identity_beyond_the_knees() {
    for c in curves() {
        for k in 0..100 {
            let x = f64::from(k) * 37.5;
            let land = c.land.knee_m + x;
            let water = -(c.water.knee_m + x);
            assert_eq!(c.apply(land), land);
            assert_eq!(c.apply(water), water);
        }
    }
}

#[test]
fn strictly_monotone_and_sign_kept() {
    for c in curves() {
        let k = c.land.knee_m.max(c.water.knee_m);
        let n = 200_000;
        let mut prev = f64::NEG_INFINITY;
        for i in 0..=n {
            let h = -2.0 * k + 4.0 * k * f64::from(i) / f64::from(n);
            let f = c.apply(h);
            assert!(f > prev, "not increasing at h = {h}: {f} <= {prev}");
            prev = f;
            assert_eq!(f < 0.0, h < 0.0, "sign changed at h = {h}");
            assert!(f.abs() <= h.abs() + 1e-9, "moved away from sea level at h = {h}");
        }
    }
}

#[test]
fn value_and_derivative_continuous_at_the_knees() {
    let e = 1e-4;
    for c in curves() {
        for (k, dir) in [(c.land.knee_m, 1.0), (-c.water.knee_m, -1.0)] {
            // Value: both sides meet the identity at the knee.
            assert!(close(c.apply(k - dir * e), k - dir * e, 1e-6));
            assert!(close(c.apply(k), k, 1e-12));
            // Derivative: one-sided differences on both sides are 1.
            let e = 1e-6;
            let inner = (c.apply(k) - c.apply(k - dir * e)) / (dir * e);
            let outer = (c.apply(k + dir * e) - c.apply(k)) / (dir * e);
            assert!(close(inner, 1.0, 1e-6), "inner derivative {inner} at {k}");
            assert!(close(outer, 1.0, 1e-6), "outer derivative {outer} at {k}");
        }
    }
}

#[test]
fn slope_at_the_shore() {
    let e = 1e-6;
    for c in curves() {
        let land = c.apply(e) / e;
        let water = c.apply(-e) / -e;
        assert!(close(land, c.land.slope, 1e-6), "land f'(0) = {land}");
        assert!(close(water, c.water.slope, 1e-6), "water f'(0) = {water}");
    }
}

#[test]
fn f32_keeps_the_sign_of_tiny_samples() {
    let c = HeightCurve::default();
    let tiny = f32::from_bits(1); // smallest positive subnormal
    assert!(c.apply_f32(-tiny) < 0.0);
    assert!(c.apply_f32(tiny) > 0.0);
    assert!(c.apply_f32(-1e-3) < 0.0);
}

#[test]
fn defaults_flatten_a_scaled_shore() {
    let c = HeightCurve::default();
    let f = c.apply(24.0);
    assert!((f - 5.7).abs() < 0.05, "24 m -> {f} m");
}

const FILE: &str = r#"
[params]
land_slope = { v = 0.2, unit = "ratio", min = 0.001, max = 1, source = "Guess" }
land_knee_m = { v = 250, unit = "m", min = 1, max = 10000, source = "Guess", help = "x" }
water_slope = { v = 0.4, unit = "ratio", min = 0.001, max = 1, source = "Guess" }
water_knee_m = { v = 120, unit = "m", min = 1, max = 10000, source = "Guess" }
"#;

#[test]
fn parses_a_params_file() {
    let c = HeightCurve::parse_toml(FILE).unwrap();
    assert_eq!(c.land, Branch { slope: 0.2, knee_m: 250.0 });
    assert_eq!(c.water, Branch { slope: 0.4, knee_m: 120.0 });
}

#[test]
fn rejects_bad_params_files() {
    let missing = FILE.replace("water_knee_m", "# water_knee_m");
    assert!(HeightCurve::parse_toml(&missing).is_err());
    let unknown = format!("{FILE}extra = {{ v = 1, unit = \"m\", min = 0, max = 2, source = \"Guess\" }}\n");
    assert!(HeightCurve::parse_toml(&unknown).is_err());
    let unit = FILE.replace("v = 250, unit = \"m\"", "v = 250, unit = \"km\"");
    assert!(HeightCurve::parse_toml(&unit).is_err());
    let range = FILE.replace("v = 0.2,", "v = 2,");
    assert!(HeightCurve::parse_toml(&range).is_err());
    let source = FILE.replace("v = 0.4, unit = \"ratio\", min = 0.001, max = 1, source = \"Guess\"", "v = 0.4, unit = \"ratio\", min = 0.001, max = 1, source = \"Vibes\"");
    assert!(HeightCurve::parse_toml(&source).is_err());
    let zero = FILE.replace("v = 0.2, unit = \"ratio\", min = 0.001", "v = 0, unit = \"ratio\", min = 0");
    assert!(HeightCurve::parse_toml(&zero).is_err());
}
