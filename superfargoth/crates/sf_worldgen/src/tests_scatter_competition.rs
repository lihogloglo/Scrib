#![allow(clippy::cast_precision_loss)] // counts of test fixtures
#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! WG-E2e: v2 competition. Winners never sit closer than their conflict distance `R_ij`; the bucket grid finds
//! exactly the brute-force conflicts; g(r) of the winners is ~0 below 0.8 R; later rounds only add winners; a tile
//! alone equals the same square of a larger build, bit for bit; any thread count gives the same winners.

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_stand::StandRules;

/// Two dense canopy species of one layer, of different `crown_k` (a mixed `R_ij`), clumped hard.
const DENSE: &str = r#"
[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }
offspring_mean = { unit = "count", min = 1, max = 60, source = "Guess" }
cluster_sigma_m = { unit = "m", min = 2, max = 40, source = "Guess" }
size_shape = { unit = "ratio", min = 0.5, max = 6, source = "Guess" }
crown_k = { unit = "ratio", min = 0.3, max = 1.2, source = "Guess" }

[[species]]
name = "pine"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 30.0
offspring_mean = 30.0
cluster_sigma_m = 6.0

[[species]]
name = "birch"
layer = "canopy"
size_law = "weibull"
size_shape = 2.0
crown_k = 1.1
parent_per_ha = 30.0
offspring_mean = 30.0
cluster_sigma_m = 6.0
"#;

/// One dense canopy species of a narrow size range (4-5 m: `R` within 12 % of its mean).
const NARROW: &str = r#"
[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }
offspring_mean = { unit = "count", min = 1, max = 60, source = "Guess" }
cluster_sigma_m = { unit = "m", min = 2, max = 40, source = "Guess" }
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
size_min_m = 4.0
size_max_m = 5.0
parent_per_ha = 30.0
offspring_mean = 30.0
cluster_sigma_m = 6.0
"#;

fn neutral() -> NeutralFields {
    NeutralFields {
        sample: FieldSample {
            temperature_c: 10.0,
            precipitation_mm: 800.0,
            moisture: 0.5,
            class: Some(0),
            slope_deg: 0.0,
            aspect_deg: 0.0,
            altitude_m: 100.0,
            curvature: 0.0,
            water_distance_m: 1000.0,
        },
        class_names: vec!["forest".to_owned()],
    }
}

fn one(_: usize, _: f64, _: f64) -> f64 {
    1.0
}

fn parse(text: &str) -> (SpeciesFile, Vec<Suitability>) {
    let file = SpeciesFile::parse_toml(text).expect("fixture parses");
    let suit = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    (file, suit)
}

/// The candidates of `rect`, the stand built over the same rect.
fn candidates(file: &SpeciesFile, suit: &[Suitability], rect: [f64; 4]) -> Vec<Candidate> {
    let fields = neutral();
    let none = |_: f64, _: f64| false;
    let stand = StandMap::build(StandRules { values: &file.values, seed: 55, canopy: suit }, &fields, &none, rect);
    let cand = CandidateRules { file, suitability: suit, seed: 55, tile_m: 256.0, prior: &one, water: &WaterMask::default(), height: &|_: f64, _: f64| 0.0, rocks: None, canopy: test_targets(file) };
    cand.candidates(&fields, &stand, rect)
}

/// `R_ij` of two candidates.
fn r_ij(file: &SpeciesFile, a: &Candidate, b: &Candidate) -> f64 {
    let k = |c: &Candidate| file.species[c.key.species as usize].crown_k;
    k(a) * a.crown_m + k(b) * b.crown_m
}

fn winners(cands: &[Candidate], won: &[bool]) -> Vec<Candidate> {
    cands.iter().zip(won).filter(|(_, w)| **w).map(|(c, _)| *c).collect()
}

#[test]
fn winners_never_conflict_and_rounds_only_add() {
    let (file, suit) = parse(DENSE);
    let cands = candidates(&file, &suit, [0.0, 0.0, 400.0, 400.0]);
    // The bucket grid finds exactly the brute-force conflicts (on a corner of the set).
    let small: Vec<Candidate> = cands.iter().filter(|c| c.pos[0] < 120.0 && c.pos[1] < 120.0).copied().collect();
    let grid = layer_conflicts(&file, &small);
    for (i, a) in small.iter().enumerate() {
        let brute: Vec<u32> = (0..small.len())
            .filter(|&j| j != i && (a.pos[0] - small[j].pos[0]).hypot(a.pos[1] - small[j].pos[1]) < r_ij(&file, a, &small[j]))
            .map(|j| j as u32)
            .collect();
        assert_eq!(grid[i], brute, "conflicts of candidate {i}");
    }
    let rounds = file.values.competition_rounds;
    assert!(rounds >= 2, "the default runs re-admission rounds");
    let won = compete(&file, &cands, rounds);
    let w = winners(&cands, &won);
    for (i, a) in w.iter().enumerate() {
        for b in &w[i + 1..] {
            let d = (a.pos[0] - b.pos[0]).hypot(a.pos[1] - b.pos[1]);
            assert!(d >= r_ij(&file, a, b), "winners {:?} and {:?} at {d} m", a.key, b.key);
        }
    }
    // A candidate with no conflict wins; the largest candidate wins.
    let nbr = layer_conflicts(&file, &cands);
    assert!(nbr.iter().zip(&won).all(|(n, w)| !n.is_empty() || *w));
    let top = (0..cands.len()).max_by(|&i, &j| priority(&cands[i], &cands[j])).expect("candidates");
    assert!(won[top]);
    // Round 1 (Matern II) winners stay winners; later rounds re-admit some losers.
    let won1 = compete(&file, &cands, 1);
    assert!(won1.iter().zip(&won).all(|(a, b)| !a || *b));
    let (n1, nk) = (won1.iter().filter(|w| **w).count(), w.len());
    let conflicted = nbr.iter().filter(|n| !n.is_empty()).count();
    println!("{} candidates, {conflicted} in a conflict; winners: {n1} after 1 round, {nk} after {rounds}", cands.len());
    assert!(cands.len() > 5000 && conflicted > cands.len() / 4, "the fixture competes");
    assert!(nk > n1 && nk < cands.len());
}

/// g(r) over `[r0, r1)` with border edge correction (centres at least `r1` from the square's sides), self pairs out.
fn pair_correlation(pts: &[[f64; 2]], side: f64, r0: f64, r1: f64) -> f64 {
    let lambda = pts.len() as f64 / (side * side);
    let centres: Vec<usize> = (0..pts.len())
        .filter(|&i| pts[i][0] >= r1 && pts[i][0] <= side - r1 && pts[i][1] >= r1 && pts[i][1] <= side - r1)
        .collect();
    let mut pairs = 0usize;
    for &i in &centres {
        let c = pts[i];
        // Every other point (a centre is not its own neighbour at r = 0).
        pairs += (0..pts.len()).filter(|&j| j != i && (r0..r1).contains(&(pts[j][0] - c[0]).hypot(pts[j][1] - c[1]))).count();
    }
    let ring = std::f64::consts::PI * (r1 * r1 - r0 * r0);
    pairs as f64 / (centres.len() as f64 * lambda * ring)
}

#[test]
fn pair_correlation_is_zero_below_the_conflict_distance() {
    let side = 400.0;
    let (file, suit) = parse(NARROW);
    let cands = candidates(&file, &suit, [0.0, 0.0, side, side]);
    let won = compete(&file, &cands, file.values.competition_rounds);
    let pts: Vec<[f64; 2]> = winners(&cands, &won).iter().map(|c| c.pos).collect();
    let sp = &file.species[0];
    let r_mean = sp.crown_k * 2.0 * (sp.crown_a_m + sp.crown_b * 0.5 * (sp.size_min_m + sp.size_max_m));
    let below = pair_correlation(&pts, side, 0.0, 0.8 * r_mean);
    let near = pair_correlation(&pts, side, r_mean, 1.5 * r_mean);
    let cand_pts: Vec<[f64; 2]> = cands.iter().map(|c| c.pos).collect();
    let before = pair_correlation(&cand_pts, side, 0.0, 0.8 * r_mean);
    println!(
        "{} winners of {}; R {r_mean:.2} m; g(<0.8 R) = {below:.3} (candidates {before:.2}); g(R-1.5 R) = {near:.2}",
        pts.len(),
        cands.len()
    );
    assert!(pts.len() > 1000);
    assert!(before > 1.0, "the candidates crowd below R");
    assert!(below < 0.05, "g(r) below 0.8 R {below:.3}");
}

type Bits = (CandidateKey, [u64; 4], u64);

fn bits(c: &[Candidate]) -> Vec<Bits> {
    c.iter()
        .map(|c| (c.key, [c.pos[0].to_bits(), c.pos[1].to_bits(), c.size_m.to_bits(), c.crown_m.to_bits()], c.tiebreak))
        .collect()
}

#[test]
fn a_tile_alone_equals_the_same_square_of_a_larger_build() {
    let (file, suit) = parse(DENSE);
    let fields = neutral();
    let none = |_: f64, _: f64| false;
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one, water: &WaterMask::default(), height: &|_: f64, _: f64| 0.0, rocks: None, canopy: test_targets(&file) };
    let tile = (1, 1);
    let rect = cand.tile_rect(tile);
    let stand = StandMap::build(StandRules { values: &file.values, seed: 55, canopy: &suit }, &fields, &none, rect);
    let alone = cand.tile_winners(&fields, &stand, tile);
    // A build over three tiles a side plus the reach: the middle tile is exact in it.
    let g = cand.competition_reach_m();
    let big_rect = [-g, -g, 768.0 + g, 768.0 + g];
    let big = candidates(&file, &suit, big_rect);
    let won = compete(&file, &big, file.values.competition_rounds);
    let inside = |c: &Candidate| c.pos[0] >= 256.0 && c.pos[0] < 512.0 && c.pos[1] >= 256.0 && c.pos[1] < 512.0;
    let cut: Vec<Candidate> = winners(&big, &won).into_iter().filter(inside).collect();
    println!("{} winners in the tile; reach {g:.1} m", alone.len());
    assert!(alone.len() > 1000, "{} winners", alone.len());
    assert_eq!(bits(&alone), bits(&cut));
    assert!(alone.windows(2).all(|w| w[0].key < w[1].key));
}

#[test]
fn winners_are_the_same_on_any_thread_count() {
    let (file, suit) = parse(DENSE);
    let cands = candidates(&file, &suit, [0.0, 0.0, 400.0, 400.0]);
    let run = |threads: usize| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| compete(&file, &cands, file.values.competition_rounds))
    };
    let one = run(1);
    assert_eq!(one, run(4));
    assert_eq!(one, run(7));
}
