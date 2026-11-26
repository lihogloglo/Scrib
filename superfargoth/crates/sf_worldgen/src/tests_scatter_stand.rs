#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
#![allow(clippy::cast_precision_loss, clippy::cast_possible_wrap)] // grid indices of test fixtures
//! WG-E2c: the four classes partition the plane by the thresholds; the edge band round a forced disc clearing has
//! its declared width (node distances exact); a tile built alone answers bit for bit what a larger build answers.

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_species::{declared, Species, SpeciesFile};

const CANOPY: &str = r#"
[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
"#;

fn canopy() -> Vec<Suitability> {
    let sp: Vec<Species> = SpeciesFile::parse_toml(CANOPY).expect("fixture parses").species;
    sp.iter().map(|s| Suitability::new(s, &[])).collect()
}

fn neutral() -> NeutralFields {
    NeutralFields {
        sample: FieldSample {
            temperature_c: 10.0,
            precipitation_mm: 800.0,
            moisture: 0.5,
            class: None,
            slope_deg: 0.0,
            aspect_deg: 0.0,
            altitude_m: 100.0,
            curvature: 0.0,
            water_distance_m: 1000.0,
        },
        class_names: Vec::new(),
    }
}

fn values_with(name: &str, v: f64) -> ScatterValues {
    let mut d = declared();
    d.iter_mut().find(|p| p.name == name).expect("declared").v = v;
    ScatterValues::from_declared(&d).expect("valid values")
}

fn disc(c: [f64; 2], r: f64) -> impl Fn(f64, f64) -> bool + Sync {
    move |x, y| (x - c[0]).hypot(y - c[1]) <= r
}

#[test]
fn classes_partition_by_thresholds() {
    let values = ScatterValues::default();
    let canopy = canopy();
    let fields = neutral();
    let excl = disc([700.0, 300.0], 40.0);
    let rules = StandRules { values: &values, seed: 55, canopy: &canopy };
    let map = StandMap::build(rules, &fields, &excl, [0.0, 0.0, 1000.0, 1000.0]);
    let mut count = [0usize; 4];
    let mut n = 0usize;
    let mut y = 0.0;
    while y <= 1000.0 {
        let mut x = 0.0;
        while x <= 1000.0 {
            let p = map.at(x, y);
            assert!((0.0..=1.0).contains(&p.d), "D {} outside [0, 1]", p.d);
            let want = if excl(x, y) || p.d < values.stand_clearing_threshold {
                StandClass::Clearing
            } else if p.d < values.stand_sparse_threshold {
                StandClass::Sparse
            } else if p.d < values.stand_inner_threshold {
                StandClass::Edge
            } else {
                StandClass::Inner
            };
            assert_eq!(p.class, want, "class at ({x}, {y}), D {}", p.d);
            assert_eq!(p.forced, excl(x, y));
            if p.class == StandClass::Clearing {
                assert!(!p.edge_band, "a clearing is never in the band");
            }
            count[p.class as usize] += 1;
            n += 1;
            x += 5.0;
        }
        y += 5.0;
    }
    assert_eq!(count.iter().sum::<usize>(), n);
    assert!(count.iter().all(|&c| c > 0), "every class occurs on 1 km2: {count:?}");

    // No canopy species: D = 0 everywhere, all clearing.
    let none: Vec<Suitability> = Vec::new();
    let rules = StandRules { values: &values, seed: 55, canopy: &none };
    let map = StandMap::build(rules, &fields, &excl, [0.0, 0.0, 100.0, 100.0]);
    for (x, y) in [(0.0, 0.0), (37.0, 81.5), (100.0, 100.0)] {
        let p = map.at(x, y);
        assert_eq!((p.d, p.class), (0.0, StandClass::Clearing));
    }
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn band_width_exact_on_a_disc() {
    // No mosaic clearings (threshold 0): the forced disc is the only clearing.
    let values = values_with("scatter.stand_clearing_threshold", 0.0);
    let (c, r_disc) = ([502.3, 497.9], 60.0);
    let (band, step) = (values.edge_band_m, values.field_step_m);
    let canopy = canopy();
    let fields = neutral();
    let excl = disc(c, r_disc);
    let rules = StandRules { values: &values, seed: 7, canopy: &canopy };
    let map = StandMap::build(rules, &fields, &excl, [350.0, 350.0, 650.0, 650.0]);

    // Node distances equal the brute-force distance to the nearest clearing node, capped at the band.
    let (w, h, o) = map.grid();
    let node = |i: usize, j: usize| [o[0] + i as f64 * step, o[1] + j as f64 * step];
    let seeds: Vec<(i64, i64)> = (0..h)
        .flat_map(|j| (0..w).map(move |i| (i, j)))
        .filter(|&(i, j)| {
            let p = node(i, j);
            excl(p[0], p[1])
        })
        .map(|(i, j)| (i as i64, j as i64))
        .collect();
    assert!(!seeds.is_empty());
    for j in 0..h {
        for i in 0..w {
            let d2 = seeds
                .iter()
                .map(|&(a, b)| (a - i as i64).pow(2) + (b - j as i64).pow(2))
                .min()
                .expect("seeds");
            let want = ((d2 as f64).sqrt() * step).min(band);
            assert_eq!(map.node_distance_m(i, j), want, "node ({i}, {j})");
        }
    }

    // On 64 rays the band starts at the disc and ends at the disc + edge_band_m, within one field step.
    for k in 0..64 {
        let a = f64::from(k) * std::f64::consts::TAU / 64.0;
        let at = |r: f64| map.at(c[0] + r * a.cos(), c[1] + r * a.sin());
        let mut outer = r_disc;
        let mut r = r_disc + 0.05;
        while r < r_disc + band + 3.0 * step {
            let p = at(r);
            if !p.forced {
                if r > r_disc + step && r < r_disc + band - step {
                    assert!(p.edge_band, "ray {k}: r {r} inside the band");
                }
                if p.edge_band {
                    outer = r;
                }
            }
            r += 0.05;
        }
        let width = outer - r_disc;
        assert!((width - band).abs() <= step, "ray {k}: band width {width} m, declared {band} m");
        assert!(at(r_disc - 1.0).class == StandClass::Clearing && !at(r_disc - 1.0).edge_band);
    }
}

#[test]
fn tile_alone_is_bitwise() {
    let values = ScatterValues::default();
    let canopy = canopy();
    let fields = neutral();
    // A forced clearing straddling the tile's east edge.
    let excl = disc([250.0, 100.0], 30.0);
    let alone = StandMap::build(
        StandRules { values: &values, seed: 55, canopy: &canopy },
        &fields,
        &excl,
        [0.0, 0.0, 256.0, 256.0],
    );
    let wide = StandMap::build(
        StandRules { values: &values, seed: 55, canopy: &canopy },
        &fields,
        &excl,
        [-256.0, -256.0, 512.0, 512.0],
    );
    let mut y = 0.0;
    let mut band = 0usize;
    while y <= 256.0 {
        let mut x = 0.0;
        while x <= 256.0 {
            let (a, b) = (alone.at(x, y), wide.at(x, y));
            assert_eq!(a.clearing_distance_m.to_bits(), b.clearing_distance_m.to_bits(), "({x}, {y})");
            assert_eq!(a, b, "({x}, {y})");
            band += usize::from(a.edge_band);
            x += 1.7;
        }
        y += 1.7;
    }
    assert!(band > 0, "the fixture has an edge band");
}
