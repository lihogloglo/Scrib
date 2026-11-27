#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! WG-E2b: hats are exact on fixture points, combine by minimum and class; the grid provider blends exactly; the
//! warm species' share rises monotonically along a temperature ramp.

use super::*;
use crate::scatter_species::SpeciesFile;

/// A cold and a warm canopy species overlapping on temperature; the warm one also has a slope hat and a class list.
const FIXTURE: &str = r#"
[columns]
hat_temperature_c = { unit = "C", min = -40, max = 40, source = "Guess" }
hat_slope_deg = { unit = "deg", min = 0, max = 90, source = "Guess" }

[[species]]
name = "cold"
layer = "canopy"
size_law = "reverse_j"
hat_temperature_c = [-10, -5, 5, 15]

[[species]]
name = "warm"
layer = "canopy"
size_law = "weibull"
classes = ["taiga"]
hat_temperature_c = [0, 4, 8, 16]
hat_slope_deg = [0, 0, 20, 40]
"#;

fn species() -> Vec<Species> {
    SpeciesFile::parse_toml(FIXTURE).expect("fixture parses").species
}

fn names(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

fn base() -> FieldSample {
    FieldSample {
        temperature_c: 6.0,
        precipitation_mm: 800.0,
        moisture: 0.5,
        class: Some(1),
        slope_deg: 10.0,
        aspect_deg: 90.0,
        altitude_m: 200.0,
        curvature: 0.0,
        water_distance_m: 100.0,
    }
}

#[test]
fn hat_is_exact_on_fixture_points() {
    let h = Hat { e_min: 0.0, i_min: 4.0, i_max: 8.0, e_max: 16.0 };
    let cases = [
        (-1.0, 0.0),
        (0.0, 0.0),
        (1.0, 0.25),
        (2.0, 0.5),
        (4.0, 1.0),
        (6.0, 1.0),
        (8.0, 1.0),
        (12.0, 0.5),
        (14.0, 0.25),
        (16.0, 0.0),
        (17.0, 0.0),
    ];
    for (x, want) in cases {
        assert_eq!(hat(&h, x), want, "hat at {x}");
    }
    assert_eq!(hat(&h, f64::NAN), 0.0);
    // A step hat (e = i on both sides): 1 on the closed interval, 0 just outside, no division by zero.
    let step = Hat { e_min: 4.0, i_min: 4.0, i_max: 8.0, e_max: 8.0 };
    for (x, want) in [(3.5, 0.0), (4.0, 1.0), (8.0, 1.0), (8.5, 0.0)] {
        assert_eq!(hat(&step, x), want, "step hat at {x}");
    }
}

#[test]
fn suitability_is_the_minimum_gated_by_class() {
    let sp = species();
    let classes = names(&["desert", "taiga"]);
    let cold = Suitability::new(&sp[0], &classes);
    let warm = Suitability::new(&sp[1], &classes);
    let mut s = base();
    // T 6: warm temperature hat 1, slope 10 -> 1; cold temperature 6 -> (15 - 6) / 10.
    assert_eq!(warm.at(&s), 1.0);
    assert_eq!(cold.at(&s), 0.9);
    // T 12 -> 0.5, slope 30 -> 0.5: the minimum is 0.5; slope 35 -> 0.25 wins.
    s.temperature_c = 12.0;
    s.slope_deg = 30.0;
    assert_eq!(warm.at(&s), 0.5);
    s.slope_deg = 35.0;
    assert_eq!(warm.at(&s), 0.25);
    // Class gate: another class, no class, and a world without the listed class all give 0.
    s = base();
    s.class = Some(0);
    assert_eq!(warm.at(&s), 0.0);
    s.class = None;
    assert_eq!(warm.at(&s), 0.0);
    // The cold species lists no class: it lives anywhere, also where no class is known.
    assert_eq!(cold.at(&s), 0.9);
    let elsewhere = Suitability::new(&sp[1], &names(&["desert"]));
    s.class = Some(0);
    assert_eq!(elsewhere.at(&s), 0.0);
    // No hats and no classes: 1 everywhere.
    let mut open = sp[0].clone();
    open.hats = crate::scatter_species::Hats::default();
    assert_eq!(Suitability::new(&open, &classes).at(&s), 1.0);
}

#[test]
fn grid_fields_blend_exactly_on_fixture_points() {
    // 3 x 3 at 10 m, origin (100, 500): column x -> temperature 0, 8, 16; class by row.
    let mut g = GridFields::filled(3, 3, 10.0, [100.0, 500.0], &base(), names(&["desert", "taiga"]));
    for r in 0..3 {
        for c in 0..3 {
            let i = r * 3 + c;
            g.temperature_c[i] = [0.0, 8.0, 16.0][c];
            g.altitude_m[i] = [300.0, 200.0, 100.0][r];
            g.classes[i] = [0, 1, NO_CLASS][r];
        }
    }
    g.check().expect("grid is valid");
    // Sample (0.5, 0): x 105, y 500 -> T 4; (1.5, 1.5) -> T 12, altitude 150.
    let a = g.sample(105.0, 500.0);
    assert_eq!(a.temperature_c, 4.0);
    assert_eq!(a.altitude_m, 300.0);
    assert_eq!(a.class, Some(0));
    let b = g.sample(115.0, 485.0);
    assert_eq!(b.temperature_c, 12.0);
    assert_eq!(b.altitude_m, 150.0);
    // Nearest class: row 1.5 rounds to 2 (no class); row 1.4 to 1.
    assert_eq!(b.class, None);
    assert_eq!(g.sample(115.0, 486.0).class, Some(1));
    // Outside the grid: clamped to the border sample.
    assert_eq!(g.sample(0.0, 900.0).temperature_c, 0.0);
    assert_eq!(g.sample(1000.0, 0.0).temperature_c, 16.0);
    // The warm species' hats on the blend: T 4 -> 1, T 12 -> 0.5 (row 1 is taiga).
    let warm = Suitability::new(&species()[1], g.class_names());
    assert_eq!(warm.at(&g.sample(115.0, 490.0)), 0.5);
}

#[test]
fn grid_fields_blend_aspect_across_north() {
    let mut g = GridFields::filled(2, 1, 10.0, [0.0, 0.0], &base(), Vec::new());
    g.aspect_deg = vec![350.0, 10.0];
    let a = g.sample(5.0, 0.0).aspect_deg;
    let off = a.min(360.0 - a);
    assert!(off < 1e-9, "350 and 10 blend to north, got {a}");
    g.aspect_deg = vec![80.0, 100.0];
    assert!((g.sample(5.0, 0.0).aspect_deg - 90.0).abs() < 1e-9);
}

#[test]
fn grid_fields_check_refuses_bad_rasters() {
    let ok = GridFields::filled(4, 2, 8.0, [0.0, 0.0], &base(), names(&["desert", "taiga"]));
    ok.check().expect("filled grid is valid");
    let mut g = ok.clone();
    g.moisture.pop();
    let e = g.check().unwrap_err().to_string();
    assert!(e.contains("moisture"), "{e}");
    let mut g = ok.clone();
    g.classes[3] = 2;
    let e = g.check().unwrap_err().to_string();
    assert!(e.contains("class id 2"), "{e}");
    let mut g = ok.clone();
    g.spacing_m = 0.0;
    assert!(g.check().is_err());
    let mut g = ok;
    g.w = 0;
    assert!(g.check().is_err());
}

#[test]
fn warm_share_is_monotone_on_a_temperature_ramp() {
    // 101 samples at 10 m, T from -10 C (west) to 30 C (east); taiga everywhere, flat.
    let mut g = GridFields::filled(101, 1, 10.0, [0.0, 0.0], &base(), names(&["desert", "taiga"]));
    g.slope_deg = vec![0.0; 101];
    for (c, t) in g.temperature_c.iter_mut().enumerate() {
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let v = (-10.0 + 0.4 * c as f64) as f32;
        *t = v;
    }
    let sp = species();
    let cold = Suitability::new(&sp[0], g.class_names());
    let warm = Suitability::new(&sp[1], g.class_names());
    let mut last = -1.0;
    let mut shares = Vec::new();
    for i in 0..=2000 {
        let x = f64::from(i) * 0.5;
        let s = g.sample(x, 0.0);
        let (a, b) = (cold.at(&s), warm.at(&s));
        if a + b == 0.0 {
            continue;
        }
        let share = b / (a + b);
        assert!(share >= last, "warm share fell from {last} to {share} at x {x} (T {})", s.temperature_c);
        last = share;
        shares.push(share);
    }
    // Pure cold at the west end, pure warm past the cold hat's edge, a mix in between.
    assert_eq!(shares.first().copied(), Some(0.0));
    assert_eq!(shares.last().copied(), Some(1.0));
    assert!(shares.iter().any(|&s| s > 0.2 && s < 0.8), "the hats overlap: some mixed share");
}
