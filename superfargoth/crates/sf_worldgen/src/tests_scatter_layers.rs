#![allow(clippy::cast_precision_loss)] // counts of test fixtures
//! WG-E2f: v2 lower layers. No understorey or ground winner sits in an earlier layer's stem footprint (and the
//! obstacle is not vacuous: candidates built with nothing over them do); lower-layer winners sit in more light than
//! uniform points; a layered tile alone equals the same square of a larger build, bit for bit.

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_stand::StandRules;

/// A dense canopy of thick stems, a shade-intolerant understorey and ground flora, no mosaic clearing.
const LAYERED: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }

[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }
offspring_mean = { unit = "count", min = 1, max = 60, source = "Guess" }
cluster_sigma_m = { unit = "m", min = 2, max = 40, source = "Guess" }
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
crown_a_m = { unit = "m", min = 0, max = 5, source = "Guess" }
shade_tolerance = { unit = "ratio", min = 0, max = 1, source = "Guess" }
footprint_ratio = { unit = "ratio", min = 0, max = 0.5, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 20.0
offspring_mean = 20.0
cluster_sigma_m = 8.0
footprint_ratio = 0.08

[[species]]
name = "shrub"
layer = "understorey"
size_law = "weibull"
size_min_m = 0.5
size_max_m = 2.0
crown_a_m = 0.3
shade_tolerance = 0.0
parent_per_ha = 60.0
offspring_mean = 40.0
cluster_sigma_m = 6.0
footprint_ratio = 0.15

[[species]]
name = "fern"
layer = "ground"
size_law = "reverse_j"
size_min_m = 0.2
size_max_m = 0.6
crown_a_m = 0.1
shade_tolerance = 0.5
parent_per_ha = 80.0
offspring_mean = 40.0
cluster_sigma_m = 4.0
footprint_ratio = 0.3
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

fn parse() -> (SpeciesFile, Vec<Suitability>) {
    let file = SpeciesFile::parse_toml(LAYERED).expect("fixture parses");
    let suit = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    (file, suit)
}

fn foot(file: &SpeciesFile, c: &Candidate) -> f64 {
    file.species[c.key.species as usize].footprint_ratio * c.size_m
}

fn dist(a: &Candidate, b: &Candidate) -> f64 {
    (a.pos[0] - b.pos[0]).hypot(a.pos[1] - b.pos[1])
}

/// Pairs (earlier-layer stem, later-layer instance) whose footprints overlap.
fn overlaps(file: &SpeciesFile, early: &[Candidate], late: &[Candidate]) -> usize {
    late.iter().map(|l| early.iter().filter(|e| dist(e, l) < foot(file, e) + foot(file, l)).count()).sum()
}

#[test]
fn no_lower_instance_in_an_earlier_stem_footprint() {
    let (file, suit) = parse();
    let fields = neutral();
    let none = |_: f64, _: f64| false;
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one };
    let rect = cand.tile_rect((0, 0));
    let stand = StandMap::build(StandRules { values: &file.values, seed: 55, canopy: &suit }, &fields, &none, rect);
    let won = cand.tile_winners(&fields, &stand, (0, 0));
    let of = |l: Layer| won.iter().filter(|c| c.key.layer == l).copied().collect::<Vec<_>>();
    let (trees, shrubs, ferns) = (of(Layer::Canopy), of(Layer::Understorey), of(Layer::Ground));
    println!("{} trees, {} shrubs, {} ferns", trees.len(), shrubs.len(), ferns.len());
    assert!(trees.len() > 300 && shrubs.len() > 300 && ferns.len() > 300);
    let mut earlier = trees.clone();
    assert_eq!(overlaps(&file, &earlier, &shrubs), 0);
    earlier.extend(&shrubs);
    assert_eq!(overlaps(&file, &earlier, &ferns), 0);
    // Not vacuous: with nothing over them, shrub candidates do land on the trees' stems.
    let bare = cand.layer_candidates(&fields, &stand, [0.0, 0.0, 256.0, 256.0], Layer::Understorey, &Cover::new(1.0));
    let hits = overlaps(&file, &trees, &bare);
    println!("{hits} bare shrub candidates on a tree stem");
    assert!(hits > 0);
}

#[test]
fn lower_layers_sit_in_more_light_than_uniform_points() {
    let (file, suit) = parse();
    let fields = neutral();
    let none = |_: f64, _: f64| false;
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one };
    let rect = cand.tile_rect((0, 0));
    let stand = StandMap::build(StandRules { values: &file.values, seed: 55, canopy: &suit }, &fields, &none, rect);
    let won = cand.tile_winners(&fields, &stand, (0, 0));
    let trees: Vec<Candidate> = won.iter().filter(|c| c.key.layer == Layer::Canopy).copied().collect();
    let mut cover = Cover::new(file.values.shade_per_crown);
    cover.add(&file, &trees);
    // Inside by the largest crown, every crown over a point has its centre in the tile.
    let tree = &file.species[0];
    let m = tree.crown_a_m + tree.crown_b * tree.size_max_m;
    let interior = |x: f64, y: f64| x >= m && x < 256.0 - m && y >= m && y < 256.0 - m;
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    let mut uniform = Vec::new();
    let mut y = m + 0.5;
    while y < 256.0 - m {
        let mut x = m + 0.5;
        while x < 256.0 - m {
            uniform.push(cover.light(x, y));
            x += 1.0;
        }
        y += 1.0;
    }
    let at = |l: Layer| -> Vec<f64> {
        won.iter().filter(|c| c.key.layer == l && interior(c.pos[0], c.pos[1])).map(|c| cover.light(c.pos[0], c.pos[1])).collect()
    };
    let (shrubs, ferns) = (at(Layer::Understorey), at(Layer::Ground));
    let (lu, ls, lf) = (mean(&uniform), mean(&shrubs), mean(&ferns));
    println!("mean L: uniform {lu:.3}, shrubs {ls:.3} ({}), ferns {lf:.3} ({})", shrubs.len(), ferns.len());
    assert!(lu < 0.9, "the canopy shades the fixture");
    assert!(ls > lu && lf > lu);
    // Shade-intolerant shrubs sit in more light than half-tolerant ferns.
    assert!(ls > lf);
}

type Bits = (CandidateKey, [u64; 4], u64);

fn bits(c: &[Candidate]) -> Vec<Bits> {
    c.iter()
        .map(|c| (c.key, [c.pos[0].to_bits(), c.pos[1].to_bits(), c.size_m.to_bits(), c.crown_m.to_bits()], c.tiebreak))
        .collect()
}

#[test]
fn a_layered_tile_alone_equals_the_same_square_of_a_larger_build() {
    let (file, suit) = parse();
    let fields = neutral();
    let none = |_: f64, _: f64| false;
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 55, tile_m: 256.0, prior: &one };
    let grows = cand.layer_grows();
    println!("grows {grows:?}");
    assert_eq!(grows.iter().map(|g| g.0).collect::<Vec<_>>(), [Layer::Canopy, Layer::Understorey, Layer::Ground]);
    assert!(grows.windows(2).all(|w| w[0].1 > w[1].1), "earlier layers reach further");
    let tile = (1, 1);
    let stand_rules = || StandRules { values: &file.values, seed: 55, canopy: &suit };
    let stand = StandMap::build(stand_rules(), &fields, &none, cand.tile_rect(tile));
    let alone = cand.tile_winners(&fields, &stand, tile);
    let g = cand.square_reach_m();
    let big_stand = StandMap::build(stand_rules(), &fields, &none, [-g, -g, 768.0 + g, 768.0 + g]);
    let big = cand.square_winners(&fields, &big_stand, [0.0, 0.0, 768.0, 768.0]);
    let inside = |c: &&Candidate| c.pos[0] >= 256.0 && c.pos[0] < 512.0 && c.pos[1] >= 256.0 && c.pos[1] < 512.0;
    let cut: Vec<Candidate> = big.iter().filter(inside).copied().collect();
    println!("{} winners in the tile; reach {g:.1} m", alone.len());
    assert!(alone.iter().any(|c| c.key.layer == Layer::Ground));
    assert_eq!(bits(&alone), bits(&cut));
    assert!(alone.windows(2).all(|w| w[0].key < w[1].key));
}
