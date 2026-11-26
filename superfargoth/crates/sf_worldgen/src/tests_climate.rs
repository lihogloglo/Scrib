#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests

use super::*;
use crate::hash_f64;

fn grid(w: usize, h: usize, spacing: f64, f: impl Fn(usize, usize) -> f64) -> Grid {
    let mut data = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            data.push(f(x, y));
        }
    }
    Grid {
        w,
        h,
        spacing,
        data,
    }
}

fn rain(heights: &Grid, c: &Climate) -> Rain {
    precipitation(heights, &temperature(heights, c), c).unwrap()
}

fn with_wind(deg: f64) -> Climate {
    Climate {
        wind_toward_deg: deg,
        ..Climate::default()
    }
}

/// 128 x 16, 1 km: sea at columns 0-19, a ridge up to 3000 m at column 50 and back to 0 at column 80, then 10 m land.
fn ridge() -> Grid {
    grid(128, 16, 1000.0, |x, _| match x {
        0..20 => -100.0,
        20..=50 => 3000.0 * (x - 20) as f64 / 30.0,
        51..=80 => 3000.0 * (80 - x) as f64 / 30.0,
        _ => 10.0,
    })
}

/// Rotates a grid by one quarter turn clockwise (north edge becomes the east edge).
fn rot(g: &Grid) -> Grid {
    let (w, h) = (g.h, g.w);
    grid(w, h, g.spacing, |x, y| g.data[(g.h - 1 - x) * g.w + y])
}

#[test]
fn lapse_on_a_cone() {
    let cone = grid(33, 33, 100.0, |x, y| {
        let r = (x as f64 - 16.0).hypot(y as f64 - 16.0);
        2000.0 - 150.0 * r
    });
    let c = Climate::default();
    let t = temperature(&cone, &c);
    for (x, want_h) in [(16, 2000.0), (20, 1400.0), (24, 800.0)] {
        let i = 16 * 33 + x;
        assert_eq!(cone.data[i], want_h);
        let want = c.sea_temperature_c - c.lapse_k_per_km * want_h / 1000.0;
        assert!(
            (t.data[i] - want).abs() < 1e-4,
            "T({want_h} m) = {}, want {want}",
            t.data[i]
        );
    }
    let mut sea = 0;
    for (z, tv) in cone.data.iter().zip(&t.data) {
        if *z <= 0.0 {
            assert_eq!(*tv, c.sea_temperature_c);
            sea += 1;
        }
    }
    assert!(sea > 0);
    let c2 = Climate {
        vertical_scale: 2.0,
        ..c
    };
    let t2 = temperature(&cone, &c2);
    for ((z, a), b) in cone.data.iter().zip(&t.data).zip(&t2.data) {
        if *z > 0.0 {
            let (d1, d2) = (c.sea_temperature_c - a, c.sea_temperature_c - b);
            assert!(
                (d2 - 2.0 * d1).abs() <= 1e-9 * 2.0 * d1,
                "drop {d1} -> {d2}"
            );
        }
    }
}

#[test]
fn ridge_casts_a_rain_shadow() {
    let g = ridge();
    let r = rain(&g, &with_wind(90.0));
    let p = &r.p.data;
    assert!(p.iter().all(|v| v.is_finite() && *v >= 0.0));
    let w = g.w;
    for y in 1..g.h {
        assert_eq!(&p[y * w..(y + 1) * w], &p[..w], "row {y}");
    }
    for (wind, lee, h) in [(25, 75, 500.0), (35, 65, 1500.0), (45, 55, 2500.0)] {
        assert_eq!(g.data[wind], h);
        assert_eq!(g.data[lee], h);
        assert!(
            p[lee] < 0.3 * p[wind],
            "{h} m: lee {} vs windward {}",
            p[lee],
            p[wind]
        );
    }
    assert!(
        p[80] < p[20],
        "lee foot {} vs windward foot {}",
        p[80],
        p[20]
    );
    println!(
        "foot ratio P[80] / P[20] = {:.4} ({:.1} / {:.1} mm/yr)",
        p[80] / p[20],
        p[80],
        p[20]
    );
}

#[test]
fn flat_land_dries_downwind() {
    let g = grid(64, 8, 1000.0, |_, _| 10.0);
    let r = rain(&g, &with_wind(90.0));
    for y in 0..g.h {
        let row = &r.p.data[y * g.w..(y + 1) * g.w];
        for x in 1..g.w {
            assert!(
                row[x] < row[x - 1],
                "row {y} column {x}: {} !< {}",
                row[x],
                row[x - 1]
            );
        }
    }
}

#[test]
fn moisture_is_conserved() {
    let g = ridge();
    let c = Climate {
        land_recycling: 0.0,
        ..with_wind(90.0)
    };
    let r = rain(&g, &c);
    let b = r.budget;
    let err = (b.inflow + b.source - b.rain - b.outflow).abs();
    assert!(
        err <= 1e-6 * (b.inflow + b.source),
        "budget {b:?}, error {err}"
    );
    let l_km = g.spacing / 1000.0;
    let sum: f64 = r.p.data.iter().sum::<f64>() * l_km;
    assert!(
        (sum - b.rain).abs() <= 1e-9 * b.rain,
        "sum(P) l = {sum}, rain = {}",
        b.rain
    );
}

#[test]
fn axis_winds_are_symmetric() {
    let field = grid(48, 48, 500.0, |x, y| {
        if x + y < 12 {
            return -80.0;
        }
        let r = (x as f64 - 14.0).hypot(y as f64 - 30.0);
        1500.0 - 120.0 * r
    });
    let c = Climate {
        latitude_k_per_km: 0.0,
        ..Climate::default()
    };
    let base = rain(
        &field,
        &Climate {
            wind_toward_deg: 90.0,
            ..c
        },
    );
    let (mut f, mut p) = (field.clone(), base.p.clone());
    for k in 1..=3 {
        f = rot(&f);
        p = rot(&p);
        let deg = f64::from((90 + 90 * k) % 360);
        let r = rain(
            &f,
            &Climate {
                wind_toward_deg: deg,
                ..c
            },
        );
        assert_eq!(
            hash_f64(&r.p.data),
            hash_f64(&p.data),
            "quarter turn {k} (wind {deg} deg)"
        );
    }
}

#[test]
fn same_rain_on_any_thread_count() {
    let (w, h) = (200, 160);
    let field = grid(w, h, 1000.0, |x, y| {
        if x < 6 || y < 6 || x >= w - 6 || y >= h - 6 {
            return -50.0;
        }
        let r = (x as f64 - 90.0).hypot(y as f64 - 85.0);
        (2500.0 - 25.0 * r).max(20.0) + 200.0 * (x as f64 * 0.3).sin() * (y as f64 * 0.2).cos()
    });
    let c = with_wind(30.0);
    let run_in = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        pool.install(|| hash_f64(&rain(&field, &c).p.data))
    };
    let one = run_in(1);
    assert_eq!(run_in(2), one);
    assert_eq!(run_in(8), one);
}

#[test]
#[ignore = "measurement: prints the cost of temperature + precipitation on 6.4 M samples"]
fn cost_on_a_world_sized_grid() {
    let (w, h) = (2530, 2530);
    let field = grid(w, h, 20.0, |x, y| {
        let r = (x as f64 - 1265.0).hypot(y as f64 - 1265.0);
        (3000.0 - 3.0 * r).max(-50.0) + 100.0 * (x as f64 * 0.01).sin() * (y as f64 * 0.013).cos()
    });
    for deg in [90.0, 30.0] {
        let c = with_wind(deg);
        let start = std::time::Instant::now();
        let t = temperature(&field, &c);
        let t_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = std::time::Instant::now();
        let r = precipitation(&field, &t, &c).unwrap();
        let p_ms = start.elapsed().as_secs_f64() * 1000.0;
        let mut p = r.p.data.clone();
        p.sort_by(f64::total_cmp);
        println!(
            "climate cost {w} x {h}, wind {deg}: temperature {t_ms:.0} ms, precipitation {p_ms:.0} ms, {} threads; P median {:.0}, p99 {:.0}, max {:.0} mm/yr",
            rayon::current_num_threads(),
            p[p.len() / 2],
            p[p.len() * 99 / 100],
            p[p.len() - 1]
        );
    }
}

const FILE: &str = r#"
[params]
lapse_k_per_km = { v = 6.0, unit = "K/km", min = 4, max = 9.8, source = "Industry(standard atmosphere lapse rate)" }
sea_temperature_c = { v = 22, unit = "C", min = -5, max = 30, source = "Guess" }
vertical_scale = { v = 2, unit = "ratio", min = 0.5, max = 4, source = "Guess" }
latitude_k_per_km = { v = 0.01, unit = "K/km", min = -0.05, max = 0.05, source = "Guess" }
wind_toward_deg = { v = 270, unit = "deg", min = 0, max = 360, source = "Guess", help = "x" }
capacity = { v = 200000, unit = "mm/yr km", min = 50000, max = 500000, source = "Guess" }
capacity_per_k = { v = 0.07, unit = "1/K", min = 0.04, max = 0.08, source = "Industry(Clausius-Clapeyron)" }
evaporation_km = { v = 80, unit = "km", min = 20, max = 200, source = "Guess" }
land_recycling = { v = 0.1, unit = "ratio", min = 0, max = 0.5, source = "Guess" }
fallout_km = { v = 8, unit = "km", min = 2, max = 20, source = "Guess" }
background_km = { v = 500, unit = "km", min = 100, max = 1000, source = "Guess" }
inflow_saturation = { v = 0.9, unit = "ratio", min = 0, max = 1, source = "Guess" }
pet_mm_per_c = { v = 60, unit = "mm/yr/C", min = 30, max = 90, source = "Industry(Holdridge PET)" }
drain_area_min_m2 = { v = 1e4, unit = "m2", min = 1e3, max = 1e7, source = "Guess" }
drain_area_max_m2 = { v = 1e8, unit = "m2", min = 1e7, max = 1e11, source = "Guess" }
drain_weight = { v = 0.4, unit = "ratio", min = 0, max = 1, source = "Guess" }
override_blend_m = { v = 1500, unit = "m", min = 0, max = 20000, source = "Guess" }
drain_flow_exponent = { v = 1.5, unit = "ratio", min = 0.5, max = 10, source = "Guess" }
"#;

const OVERRIDES: &str = r#"
[override_columns]
t_add = { unit = "K", min = -30, max = 30, source = "Guess" }
p_scale = { unit = "ratio", min = 0, max = 5, source = "Guess" }
p_pin = { unit = "mm/yr", min = 0, max = 10000, source = "Guess" }

[regions."Ash Plain"]
p_scale = 0.3
t_add = 2.0

[regions."North Isles"]
p_pin = 900
"#;

fn with_overrides() -> String {
    format!("{FILE}{OVERRIDES}")
}

#[test]
fn parses_region_overrides() {
    let f = ClimateFile::parse_toml(&with_overrides()).unwrap();
    assert_eq!(f.overrides.len(), 2);
    assert_eq!(f.overrides[0].region, "Ash Plain");
    assert_eq!(
        f.overrides[0].rain,
        Adjust { scale: 0.3, add: 0.0, pin: None }
    );
    assert_eq!(
        f.overrides[0].temperature,
        Adjust { scale: 1.0, add: 2.0, pin: None }
    );
    assert_eq!(f.overrides[1].region, "North Isles");
    assert_eq!(f.overrides[1].rain.pin, Some(900.0));
    assert_eq!(f.overrides[1].temperature, Adjust::default());
    assert_eq!(f.climate.override_blend_m, 1500.0);
    assert_eq!(f.declared.len(), 18);
    assert_eq!(f.climate.drain_flow_exponent, 1.5);
    let sea = f.declared.iter().find(|d| d.name == "sea_temperature_c").unwrap();
    assert_eq!(sea.v, 22.0);
}

#[test]
fn rejects_bad_overrides() {
    let good = with_overrides();
    let p_add_column =
        "[override_columns]\np_add = { unit = \"mm/yr\", min = 0, max = 100, source = \"Guess\" }";
    let cases: &[(&str, &str, &str)] = &[
        // a column name outside the six
        (
            "unknown column",
            "[override_columns]",
            "[override_columns]\nt_mul = { unit = \"ratio\", min = 0, max = 1, source = \"Guess\" }",
        ),
        // a column whose unit is not the fixed one
        ("wrong unit", "unit = \"K\", min = -30", "unit = \"C\", min = -30"),
        // a column source failing check_source
        ("bad source", "max = 5, source = \"Guess\"", "max = 5, source = \"Hunch\""),
        // a column with min > max
        ("inverted range", "min = 0, max = 5,", "min = 6, max = 5,"),
        // a column with a non-finite bound
        ("non-finite bound", "min = -30, max = 30,", "min = -30, max = inf,"),
        // a region value whose column is not declared
        ("undeclared column", "t_add = 2.0", "p_add = 2.0"),
        // a region value outside its column's range
        ("out of range", "p_scale = 0.3", "p_scale = 7.0"),
        // a pin together with a scale of the same field
        ("pin and scale", "p_pin = 900", "p_pin = 900\np_scale = 0.5"),
        // a region table with no value
        ("empty region", "p_pin = 900\n", ""),
        // an empty region name
        ("empty name", "[regions.\"North Isles\"]", "[regions.\"\"]"),
        // an unknown key inside a region table
        ("unknown key", "p_pin = 900", "p_pin = 900\nsnow = 1"),
    ];
    for (what, from, to) in cases {
        let text = good.replace(from, to);
        assert_ne!(text, good, "{what}: the replace changed nothing");
        assert!(ClimateFile::parse_toml(&text).is_err(), "{what}: accepted");
    }
    // a pin together with an add of the same field (p_add declared, so only the pin rule refuses it)
    let declared = good.replace("[override_columns]", p_add_column);
    assert_ne!(declared, good);
    assert!(ClimateFile::parse_toml(&declared).is_ok());
    let text = declared.replace("p_pin = 900", "p_pin = 900\np_add = 10");
    assert_ne!(text, declared);
    assert!(ClimateFile::parse_toml(&text).is_err(), "pin and add: accepted");
}

#[test]
fn a_file_without_overrides_has_none() {
    let f = ClimateFile::parse_toml(FILE).unwrap();
    assert!(f.overrides.is_empty());
    let d = ClimateFile::defaults();
    assert_eq!(d.declared.len(), 18);
    assert_eq!(d.climate.drain_flow_exponent, 1.1);
    assert!(d.overrides.is_empty());
    assert_eq!(d.climate, Climate::default());
    assert_eq!(Climate::parse_toml(FILE).unwrap(), f.climate);
}

#[test]
fn parses_and_rejects_params_files() {
    let c = Climate::parse_toml(FILE).unwrap();
    assert_eq!(c.sea_temperature_c, 22.0);
    assert_eq!(c.wind_toward_deg, 270.0);
    assert_eq!(c.inflow_saturation, 0.9);
    assert_eq!(c.pet_mm_per_c, 60.0);
    assert_eq!(c.drain_area_max_m2, 1e8);
    assert_eq!(c.drain_weight, 0.4);
    let missing = FILE.replace("fallout_km =", "# fallout_km =");
    assert!(Climate::parse_toml(&missing).is_err());
    let missing = FILE.replace("drain_weight =", "# drain_weight =");
    assert!(Climate::parse_toml(&missing).is_err());
    assert_eq!(c.override_blend_m, 1500.0);
    let missing = FILE.replace("override_blend_m =", "# override_blend_m =");
    assert!(Climate::parse_toml(&missing).is_err());
    let missing = FILE.replace("drain_flow_exponent =", "# drain_flow_exponent =");
    assert!(Climate::parse_toml(&missing).is_err());
    let inverted = FILE.replace("v = 1e8,", "v = 1e7,").replace("v = 1e4,", "v = 1e7,");
    assert!(Climate::parse_toml(&inverted).is_err());
    let unknown =
        format!("{FILE}extra = {{ v = 1, unit = \"m\", min = 0, max = 2, source = \"Guess\" }}\n");
    assert!(Climate::parse_toml(&unknown).is_err());
    let range = FILE.replace("v = 6.0,", "v = 12,");
    assert!(Climate::parse_toml(&range).is_err());
    assert_eq!(declared_defaults().len(), PARAM_NAMES.len());
    assert_eq!(Climate::default().lapse_k_per_km, 6.5);
}

/// One-sample grids of `t` and `p`, with drainage area `a`, through [`moisture`] at the defaults.
fn m1(t: f64, p: f64, a: f64) -> f64 {
    let c = Climate::default();
    let g = |v| grid(1, 1, 1000.0, |_, _| v);
    moisture(&g(t), &g(p), &[a], &c).unwrap().data[0]
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12, "{a} vs {b}");
}

#[test]
fn moisture_matches_the_formula() {
    // T 20: PET = 58.93 * 20 = 1178.6, so P 1178.6 gives AI 1 and m_clim 0.5.
    close(m1(20.0, 1178.6, 1e4), 0.5);
    close(m1(20.0, 1178.6, 1e7), 0.625);
    close(m1(20.0, 1178.6, 1e9), 0.75);
    close(m1(20.0, 1178.6, 1e12), 0.75);
    close(m1(20.0, 0.0, 1e4), 0.0);
}

#[test]
fn moisture_is_monotone_and_bounded() {
    let c = Climate::default();
    let n = 32;
    // x = P step, y = T step.
    let t = grid(n, n, 1000.0, |_, y| -10.0 + 45.0 * y as f64 / (n - 1) as f64);
    let p = grid(n, n, 1000.0, |x, _| 5000.0 * x as f64 / (n - 1) as f64);
    let ms: Vec<Grid> = [1e4, 1e7, 1e10]
        .iter()
        .map(|&a| moisture(&t, &p, &vec![a; n * n], &c).unwrap())
        .collect();
    for m in &ms {
        for &v in &m.data {
            assert!(v.is_finite() && (0.0..=1.0).contains(&v), "{v}");
        }
        for y in 0..n {
            for x in 1..n {
                assert!(m.data[y * n + x] >= m.data[y * n + x - 1], "P rise dries at ({x}, {y})");
            }
        }
    }
    for k in 1..ms.len() {
        for i in 0..n * n {
            assert!(ms[k].data[i] >= ms[k - 1].data[i], "area rise dries at {i}");
        }
    }
}

#[test]
fn frozen_ground_reads_wet() {
    // T -5: biotemperature 0, PET 0, floored to 1 mm/yr, so AI = 100.
    close(m1(-5.0, 100.0, 1e4), 100.0 / 101.0);
}

#[test]
fn moisture_refuses_mismatched_sizes() {
    let c = Climate::default();
    let t = grid(4, 3, 1000.0, |_, _| 10.0);
    let p = grid(4, 3, 1000.0, |_, _| 500.0);
    assert!(moisture(&t, &p, &[1e6; 12], &c).is_ok());
    assert!(moisture(&t, &p, &[1e6; 11], &c).is_err());
    let wide = grid(3, 4, 1000.0, |_, _| 500.0);
    assert!(moisture(&t, &wide, &[1e6; 12], &c).is_err());
}
