//! WG-L2a landform census tests on synthetic fields: a cone with a summit crater, planes with nothing, a V-valley
//! and a flat-floored trench told apart, a closed lake against a bay open to the sea, determinism.

use super::{Feature, Field, Params, census, declared_defaults, features_json, set};
use crate::Grid;

const DX: f64 = 2.0;

fn params(overrides: &[(&str, f64)]) -> Params {
    let mut d = declared_defaults();
    for &(n, v) in overrides {
        set(&mut d, n, v, None).unwrap();
    }
    Params::from_declared(&d).unwrap()
}

fn field(w: usize, h: usize, sea: f64, f: impl Fn(f64, f64) -> f64) -> Field {
    let mut data = Vec::with_capacity(w * h);
    for r in 0..h {
        for c in 0..w {
            data.push(f(c as f64 * DX, r as f64 * DX));
        }
    }
    Field::new(
        Grid {
            w,
            h,
            spacing: DX,
            data,
        },
        sea,
        [0.0, 0.0],
    )
}

fn of<'a>(fs: &'a [Feature], kinds: &[&str]) -> Vec<&'a Feature> {
    fs.iter().filter(|f| kinds.contains(&f.kind)).collect()
}

// Cone 100 - 0.4 r on a plain at 0, summit crater of radius 30 m and depth 15 m below its level rim (88 m).
fn cone() -> Field {
    field(201, 201, -10.0, |x, y| {
        let r = (x - 200.0).hypot(y - 200.0);
        let cone = (100.0 - 0.4 * r).max(0.0);
        if r < 30.0 {
            88.0 - 15.0 * (1.0 - (r / 30.0).powi(2))
        } else {
            cone
        }
    })
}

#[test]
fn cone_summit_crater_is_a_crater() {
    let (fs, _) = census(&cone(), &params(&[]));
    let craters = of(&fs, &["crater"]);
    assert_eq!(
        craters.len(),
        1,
        "{:?}",
        fs.iter().map(|f| (f.kind, &f.metrics)).collect::<Vec<_>>()
    );
    let c = craters[0];
    assert!((c.centroid_m[0] - 200.0).abs() < DX && (c.centroid_m[1] + 200.0).abs() < DX);
    assert!(
        (c.metrics["max_depth_m"] - 15.0).abs() <= 0.4 * DX,
        "{:?}",
        c.metrics
    );
    assert!(
        (c.metrics["eq_diameter_m"] - 60.0).abs() <= DX,
        "{:?}",
        c.metrics
    );
    assert!(
        (c.metrics["spill_m"] - 88.0).abs() <= 0.4 * DX,
        "{:?}",
        c.metrics
    );
    assert!(c.metrics["summit"] > 0.5, "{:?}", c.metrics);
    assert!(of(&fs, &["basin", "pool"]).is_empty());
}

#[test]
fn planes_have_no_features() {
    let p = params(&[]);
    let (flat, _) = census(&field(201, 201, 0.0, |_, _| 10.0), &p);
    assert!(flat.is_empty(), "{flat:?}");
    let (tilt, _) = census(&field(201, 201, 0.0, |x, y| 10.0 + 0.05 * x + 0.02 * y), &p);
    assert!(tilt.is_empty(), "{tilt:?}");
}

// A valley running south (rows grow south), floor falling 2 %, sides draining toward it.
fn valley(wall: impl Fn(f64) -> f64) -> Field {
    field(201, 601, -10.0, move |x, y| {
        let dx = (x - 200.0).abs();
        100.0 - 0.02 * y + wall(dx) + 0.01 * dx
    })
}

fn channel_of(f: &Field) -> Vec<Feature> {
    let p = params(&[("channel_area_m2", 10000.0)]);
    let (fs, _) = census(f, &p);
    of(&fs, &["trench", "gorge", "valley"])
        .into_iter()
        .cloned()
        .collect()
}

#[test]
fn v_valley_and_flat_trench_are_told_apart() {
    // Analytic depth at the 80 m cross-section reach: 20 m of wall + 0.8 m of side tilt.
    let v = channel_of(&valley(|dx| 0.5 * dx.min(40.0)));
    assert!(!v.is_empty());
    let main = v
        .iter()
        .max_by(|a, b| a.metrics["length_m"].total_cmp(&b.metrics["length_m"]))
        .unwrap();
    assert!(main.kind != "trench", "{:?}", main.metrics);
    assert!(
        (main.metrics["mean_depth_m"] - 20.8).abs() < 1.0,
        "{:?}",
        main.metrics
    );
    assert!(main.metrics["floor_ratio"] < 0.45, "{:?}", main.metrics);

    let t = channel_of(&valley(|dx| ((dx - 20.0) * 2.0).clamp(0.0, 20.0)));
    let main = t
        .iter()
        .max_by(|a, b| a.metrics["length_m"].total_cmp(&b.metrics["length_m"]))
        .unwrap();
    assert_eq!(main.kind, "trench", "{:?}", main.metrics);
    assert!(
        (main.metrics["mean_depth_m"] - 20.8).abs() < 1.0,
        "{:?}",
        main.metrics
    );
    assert!(main.metrics["length_m"] > 400.0, "{:?}", main.metrics);
}

#[test]
fn closed_lake_and_open_bay() {
    // Land at 5 m; sea south of y = 500 m; a 30 m wide inlet 100 m deep at x = 300 m; a 30 m radius lake.
    let f = field(301, 301, 0.0, |x, y| {
        let lake = (x - 120.0).hypot(y - 200.0) < 30.0;
        let inlet = (x - 300.0).abs() < 15.0 && y > 400.0;
        if y > 500.0 || inlet || lake {
            -4.0
        } else {
            5.0
        }
    });
    let p = params(&[("bay_min_area_m2", 2000.0), ("bay_radius_m", 40.0)]);
    let (fs, _) = census(&f, &p);
    let lakes = of(&fs, &["lake"]);
    assert_eq!(lakes.len(), 1, "{fs:?}");
    assert!(
        (lakes[0].centroid_m[0] - 120.0).abs() < DX && (lakes[0].centroid_m[1] + 200.0).abs() < DX
    );
    assert!(of(&fs, &["lagoon"]).is_empty(), "{fs:?}");
    let bays = of(&fs, &["bay"]);
    assert_eq!(bays.len(), 1, "{fs:?}");
    let b = bays[0];
    assert!(
        (b.centroid_m[0] - 300.0).abs() < 2.0 * DX,
        "{:?}",
        b.centroid_m
    );
    assert!(
        (b.metrics["mouth_width_m"] - 30.0).abs() <= 2.0 * DX,
        "{:?}",
        b.metrics
    );
    assert!(
        (b.metrics["inland_depth_m"] - 100.0).abs() <= 3.0 * DX,
        "{:?}",
        b.metrics
    );
}

#[test]
fn census_is_deterministic() {
    let f = cone();
    let p = params(&[]);
    let d = declared_defaults();
    let (a, _) = census(&f, &p);
    let (b, _) = census(&f, &p);
    assert_eq!(a, b);
    let ja = features_json("cone", &f, DX, &d, &a).unwrap();
    let jb = features_json("cone", &f, DX, &d, &b).unwrap();
    assert_eq!(ja.as_bytes(), jb.as_bytes());
}
