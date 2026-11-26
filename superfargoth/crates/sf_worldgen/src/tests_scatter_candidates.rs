#![allow(clippy::cast_precision_loss)] // counts of test fixtures
#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! WG-E2d: v2 candidates. A tile built alone returns, bit for bit, what a larger build returns over its rect; the
//! pair correlation g(r) at the clump scale shows the Thomas clusters; the reverse-J size marks have the declared
//! slope of log N(>h).

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_stand::StandRules;

/// One canopy tree on the declared defaults.
const TREE: &str = r#"
[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
"#;

/// One canopy tree of a steep reverse J (6 e-folds over 1-10 m), and no mosaic clearing (threshold 0).
const STEEP: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }

[columns]
size_shape = { unit = "ratio", min = 0.5, max = 6, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
size_shape = 6.0
"#;

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

fn one(_: usize, _: f64, _: f64) -> f64 {
    1.0
}

/// The candidates of `rect` for a file, its stand built over `stand_rect`, an exclusion disc.
fn build(text: &str, rect: [f64; 4], stand_rect: [f64; 4], disc: [f64; 3]) -> Vec<Candidate> {
    let file = SpeciesFile::parse_toml(text).expect("fixture parses");
    let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    let fields = neutral();
    let excl = move |x: f64, y: f64| (x - disc[0]).hypot(y - disc[1]) <= disc[2];
    let rules = StandRules { values: &file.values, seed: 55, canopy: &suit };
    let stand = StandMap::build(rules, &fields, &excl, stand_rect);
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one };
    cand.candidates(&fields, &stand, rect)
}

fn rules_rect(text: &str, tile: (i32, i32)) -> ([f64; 4], f64) {
    let file = SpeciesFile::parse_toml(text).expect("fixture parses");
    let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one };
    (cand.tile_rect(tile), cand.competition_reach_m())
}

type Bits = (CandidateKey, [u64; 4], u64);

fn bits(c: &[Candidate]) -> Vec<Bits> {
    c.iter()
        .map(|c| (c.key, [c.pos[0].to_bits(), c.pos[1].to_bits(), c.size_m.to_bits(), c.crown_m.to_bits()], c.tiebreak))
        .collect()
}

#[test]
fn a_tile_alone_equals_the_same_rect_of_a_larger_build() {
    // A forced clearing across the tile's west edge: the stand's clearings and edge band reach into both builds.
    let disc = [256.0, 400.0, 50.0];
    let (rect, g) = rules_rect(TREE, (1, 1));
    assert!(g > 0.0);
    let alone = build(TREE, rect, rect, disc);
    let big_rect = [-g, -g, 768.0 + g, 768.0 + g];
    let big = build(TREE, big_rect, big_rect, disc);
    let inside = |c: &&Candidate| c.pos[0] >= rect[0] && c.pos[0] < rect[2] && c.pos[1] >= rect[1] && c.pos[1] < rect[3];
    let cut: Vec<Candidate> = big.iter().filter(inside).copied().collect();
    assert!(alone.len() > 50, "{} candidates in the tile rect", alone.len());
    assert!(big.len() > 4 * alone.len());
    assert_eq!(bits(&alone), bits(&cut));
    // Keys are unique and sorted; nothing in the forced clearing; sizes in range; crowns by the allometry.
    assert!(alone.windows(2).all(|w| w[0].key < w[1].key));
    let file = SpeciesFile::parse_toml(TREE).expect("fixture parses");
    let sp = &file.species[0];
    for c in &alone {
        assert!((c.pos[0] - disc[0]).hypot(c.pos[1] - disc[1]) > disc[2], "candidate in the forced clearing");
        assert!((sp.size_min_m..=sp.size_max_m).contains(&c.size_m));
        assert_eq!(c.crown_m, sp.crown_a_m + sp.crown_b * c.size_m);
    }
    assert!(alone.iter().any(|c| c.key.parent == BACKGROUND) && alone.iter().any(|c| c.key.parent != BACKGROUND));
}

/// g(r) over `[r0, r1)` with border edge correction: centres at least `r1` from the square's sides.
fn pair_correlation(pts: &[[f64; 2]], side: f64, r0: f64, r1: f64) -> f64 {
    let lambda = pts.len() as f64 / (side * side);
    let centres: Vec<&[f64; 2]> =
        pts.iter().filter(|p| p[0] >= r1 && p[0] <= side - r1 && p[1] >= r1 && p[1] <= side - r1).collect();
    let mut pairs = 0usize;
    for c in &centres {
        pairs += pts.iter().filter(|p| (r0..r1).contains(&(p[0] - c[0]).hypot(p[1] - c[1]))).count();
    }
    let ring = std::f64::consts::PI * (r1 * r1 - r0 * r0);
    pairs as f64 / (centres.len() as f64 * lambda * ring)
}

#[test]
fn pair_correlation_shows_clumps_at_the_cluster_scale() {
    let side = 1000.0;
    let rect = [0.0, 0.0, side, side];
    let pts: Vec<[f64; 2]> = build(TREE, rect, rect, [-1e6, -1e6, 1.0]).iter().map(|c| c.pos).collect();
    let file = SpeciesFile::parse_toml(TREE).expect("fixture parses");
    let sigma = file.species[0].cluster_sigma_m;
    let g = pair_correlation(&pts, side, 0.5 * sigma, sigma);
    let far = pair_correlation(&pts, side, 8.0 * sigma, 9.0 * sigma);
    println!("{} candidates on 1 km2; g({}-{} m) = {g:.2}; g(8-9 sigma) = {far:.2}", pts.len(), 0.5 * sigma, sigma);
    assert!(pts.len() > 1000, "{} candidates", pts.len());
    assert!(g > 1.5, "g(r) at the clump scale {g:.2}");
}

#[test]
fn reverse_j_size_slope_matches_the_declared_rate() {
    let side = 1000.0;
    let rect = [0.0, 0.0, side, side];
    let mut sizes: Vec<f64> = build(STEEP, rect, rect, [-1e6, -1e6, 1.0]).iter().map(|c| c.size_m).collect();
    let file = SpeciesFile::parse_toml(STEEP).expect("fixture parses");
    let sp = &file.species[0];
    let want = -sp.size_shape / (sp.size_max_m - sp.size_min_m);
    sizes.sort_by(f64::total_cmp);
    let n = sizes.len() as f64;
    // ln N(>h) / n on 20 heights from size_min up to where 5 % are left; least-squares slope.
    let h_end = sp.size_min_m + 0.05f64.ln() / want;
    let (mut sx, mut sy, mut sxx, mut sxy) = (0.0, 0.0, 0.0, 0.0);
    let k = 20;
    for i in 0..k {
        let h = sp.size_min_m + (h_end - sp.size_min_m) * f64::from(i) / f64::from(k);
        let above = sizes.len() - sizes.partition_point(|&s| s <= h);
        let y = (above as f64 / n).ln();
        sx += h;
        sy += y;
        sxx += h * h;
        sxy += h * y;
    }
    let kf = f64::from(k);
    let slope = (kf * sxy - sx * sy) / (kf * sxx - sx * sx);
    println!("{} sizes; slope {slope:.4} /m, declared {want:.4} /m", sizes.len());
    assert!(sizes.len() > 1000);
    assert!(((slope - want) / want).abs() < 0.1, "slope {slope:.4} vs {want:.4}");
}

#[test]
fn size_marks_stay_in_range_for_both_laws() {
    let mut file = SpeciesFile::parse_toml(TREE).expect("fixture parses");
    let sp = &mut file.species[0];
    for law in [SizeLaw::ReverseJ, SizeLaw::Weibull] {
        sp.size_law = law;
        let mut last = f64::NEG_INFINITY;
        for i in 0..=1000 {
            let u = f64::from(i) / 1001.0;
            let h = size_mark(sp, u);
            assert!((sp.size_min_m..=sp.size_max_m).contains(&h), "{law:?} {h} at u {u}");
            assert!(h >= last, "{law:?} not monotone at u {u}");
            last = h;
        }
    }
    assert_eq!(poisson(0.0, 0.5), 0);
    let mean = (0..10_000).map(|i| f64::from(poisson(12.0, (f64::from(i) + 0.5) / 10_000.0))).sum::<f64>() / 10_000.0;
    assert!((mean - 12.0).abs() < 0.05, "Poisson(12) mean {mean}");
}
