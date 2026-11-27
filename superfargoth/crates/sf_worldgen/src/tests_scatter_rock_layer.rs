#![allow(clippy::cast_precision_loss)] // counts of test fixtures
//! WG-E3d: the rock layer runs first in the passes. On a plateau with a lake, a river, the sea, an 80 deg scarp
//! shedding talus, a site footprint and a road (spec "Tests"): no two rock winners closer than `R_ij = rock_k (r_i +
//! r_j)`; none on water or in an exclusion (and none of the zones is vacuous: a dry build puts rocks in every one);
//! no plant's stem footprint in a rock footprint (and without the rock layer some would be); a tile built alone
//! matches a larger build bit for bit, rocks and plants.

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_rocks::RockValues;
use crate::scatter_species::RockContext;
use crate::scatter_stand::StandRules;
use crate::water::{Lake, River};

/// A canopy tree and a ground fern over three rock contexts on one unit.
const SPECIES: &str = r#"
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
field_stone_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 20.0
offspring_mean = 20.0
cluster_sigma_m = 8.0
footprint_ratio = 0.05

[[species]]
name = "fern"
layer = "ground"
size_law = "reverse_j"
size_min_m = 0.2
size_max_m = 0.6
crown_a_m = 0.1
shade_tolerance = 0.7
parent_per_ha = 80.0
offspring_mean = 30.0
cluster_sigma_m = 4.0
footprint_ratio = 0.2

[rocks.talus]
size_min_m = 0.4
size_max_m = 6.0

[rocks.outcrop]
size_min_m = 1.0
size_max_m = 8.0

[rocks.field]
size_min_m = 0.3
size_max_m = 1.5

[rock_units.basalt]
field_stone_per_ha = 300.0
talus = [{ kind = "class", class = "block", weight = 1 }]
outcrop = [{ kind = "class", class = "block", weight = 1 }]
field = [{ kind = "class", class = "block", weight = 1 }]
"#;

const SEED: u64 = 91;
const MARGIN: f64 = 1.0;
/// The scarp's top edge, its height above the apron, the apron's height and the face angle.
const FACE_X: f64 = 180.0;
const DROP_M: f64 = 60.0;
const APRON_M: f64 = 10.0;
const FACE_DEG: f64 = 80.0;

fn face_w() -> f64 {
    DROP_M / FACE_DEG.to_radians().tan()
}

/// Sea west of x = 15 m (ground at -1 m), a plateau at 70 m, the face down to the apron at 10 m.
fn height(x: f64, _: f64) -> f64 {
    if x < 15.0 {
        -1.0
    } else if x < FACE_X {
        APRON_M + DROP_M
    } else if x < FACE_X + face_w() {
        APRON_M + DROP_M - (x - FACE_X) * FACE_DEG.to_radians().tan()
    } else {
        APRON_M
    }
}

/// The fixture ground: one unit, the face east-facing at 80 deg, flat elsewhere.
struct Ground;

impl RockGround for Ground {
    fn height_m(&self, x: f64, y: f64) -> f64 {
        height(x, y)
    }
    fn slope_aspect_deg(&self, x: f64, _y: f64) -> (f64, f64) {
        (if (FACE_X..FACE_X + face_w()).contains(&x) { FACE_DEG } else { 0.0 }, 90.0)
    }
    fn cliff(&self, _x: f64, _y: f64) -> bool {
        false
    }
    fn unit(&self, _x: f64, _y: f64) -> Option<&str> {
        Some("basalt")
    }
}

/// A square lake 40-110 m with an island 65-85 m on the plateau; a river along y = 190 m, half width 5 m.
fn network() -> GuideWater {
    let square = |a: f64, b: f64| vec![[a, a], [b, a], [b, b], [a, b]];
    GuideWater {
        sea_level_m: 0.0,
        lakes: vec![Lake { level_m: 69.0, outline: square(40.0, 110.0), holes: vec![square(65.0, 85.0)], ..Lake::default() }],
        rivers: vec![River {
            points: vec![[-300.0, 190.0, 10.0, 5.0], [600.0, 190.0, 10.0, 5.0]],
            ..River::default()
        }],
    }
}

/// A site footprint on the talus apron: the disc of radius 20 m round (225, 70).
fn in_footprint(x: f64, y: f64) -> bool {
    (x - 225.0).hypot(y - 70.0) < 20.0
}

/// A road: the strip y in [140, 150) east of x = 100 m, across the plateau, the face and the apron.
fn on_road(x: f64, y: f64) -> bool {
    x >= 100.0 && (140.0..150.0).contains(&y)
}

fn excluded(x: f64, y: f64) -> bool {
    in_footprint(x, y) || on_road(x, y)
}

fn never(_: f64, _: f64) -> bool {
    false
}

/// Brute force: inside the lake (outline minus island).
fn in_lake(x: f64, y: f64) -> bool {
    let inside = |a: f64, b: f64| x > a && x < b && y > a && y < b;
    inside(40.0, 110.0) && !inside(65.0, 85.0)
}

/// Brute force: distance to the nearest lake edge (both squares' sides), m.
fn lake_edge_distance(x: f64, y: f64) -> f64 {
    let box_edge = |a: f64, b: f64| {
        let (cx, cy) = (x.clamp(a, b), y.clamp(a, b));
        let outside = (x - cx).hypot(y - cy);
        if outside > 0.0 { outside } else { (x - a).min(b - x).min(y - a).min(b - y) }
    };
    box_edge(40.0, 110.0).min(box_edge(65.0, 85.0))
}

/// Distance from the river's bank (negative inside), m.
fn river_edge_distance(_x: f64, y: f64) -> f64 {
    (y - 190.0).abs() - 5.0
}

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

/// Dense talus (200 sources/ha, 20 blocks each) and outcrops (10/ha) so every zone is reached.
fn values() -> RockValues {
    RockValues { talus_parent_per_ha: 200.0, talus_child_mean: 20.0, outcrop_parent_per_ha: 10.0, ..RockValues::default() }
}

struct Fixture {
    file: SpeciesFile,
    suit: Vec<Suitability>,
    fields: NeutralFields,
}

impl Fixture {
    fn new() -> Self {
        let file = SpeciesFile::parse_toml(SPECIES).expect("fixture parses");
        let suit = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
        Self { file, suit, fields: neutral() }
    }

    /// The winners inside `square`, with or without the rock layer.
    fn build(
        &self,
        water: &WaterMask,
        excluded: &(dyn Fn(f64, f64) -> bool + Sync),
        rocks: bool,
        square: [f64; 4],
    ) -> LayerWinners {
        let rules = RockRules { file: &self.file, values: values(), seed: SEED, tile_m: 256.0, prior: None };
        let layer = RockLayer { rules: &rules, ground: &Ground };
        let cand = CandidateRules {
            file: &self.file,
            suitability: &self.suit,
            seed: SEED,
            tile_m: 256.0,
            prior: &one,
            water,
            height: &height,
            rocks: rocks.then_some(layer),
            canopy: test_targets(&self.file),
        };
        let g = cand.square_reach_m();
        let rect = [square[0] - g, square[1] - g, square[2] + g, square[3] + g];
        let stand = StandMap::build(StandRules { values: &self.file.values, seed: SEED, canopy: &self.suit }, &self.fields, excluded, rect);
        cand.square_layers(&self.fields, &stand, square)
    }

    fn tile(&self, water: &WaterMask, excluded: &(dyn Fn(f64, f64) -> bool + Sync), rocks: bool) -> LayerWinners {
        self.build(water, excluded, rocks, [0.0, 0.0, 256.0, 256.0])
    }

    fn foot(&self, c: &Candidate) -> f64 {
        self.file.species[c.key.species as usize].footprint_ratio * c.size_m
    }
}

/// Rocks per zone: lake, river band, sea, site footprint, road.
fn zones(rocks: &[RockCandidate]) -> [usize; 5] {
    let count = |f: &dyn Fn(f64, f64) -> bool| rocks.iter().filter(|c| f(c.pos[0], c.pos[1])).count();
    [
        count(&in_lake),
        count(&|x, y| river_edge_distance(x, y) < MARGIN),
        count(&|x, _| x < 15.0),
        count(&in_footprint),
        count(&on_road),
    ]
}

/// Plants whose stem footprint overlaps a rock footprint (`distance < s / 2 + foot`).
fn plants_in_rocks(fx: &Fixture, plants: &[Candidate], rocks: &[RockCandidate]) -> usize {
    plants
        .iter()
        .filter(|p| rocks.iter().any(|r| (p.pos[0] - r.pos[0]).hypot(p.pos[1] - r.pos[1]) < r.size_m / 2.0 + fx.foot(p)))
        .count()
}

#[test]
fn rocks_first_no_pair_no_water_no_exclusion_no_plant_in_a_rock() {
    let fx = Fixture::new();
    // Non-vacuous: a dry build with no exclusion puts rocks in every zone.
    let dry = fx.tile(&WaterMask::default(), &never, true);
    let dry_zones = zones(&dry.rocks);
    println!("dry build: {} rocks, zones (lake, river, sea, footprint, road) {dry_zones:?}", dry.rocks.len());
    assert!(dry_zones.iter().all(|&n| n > 0), "every zone holds rocks in a dry build");

    let water = WaterMask::new(&network(), MARGIN);
    let won = fx.tile(&water, &excluded, true);
    let per = |k: RockContext| won.rocks.iter().filter(|c| c.key.context == k).count();
    println!(
        "wet build: {} rocks (talus {}, outcrop {}, field {}), {} plants",
        won.rocks.len(),
        per(RockContext::Talus),
        per(RockContext::Outcrop),
        per(RockContext::Field),
        won.plants.len()
    );
    for k in [RockContext::Talus, RockContext::Outcrop, RockContext::Field] {
        assert!(per(k) > 0, "{k:?} present");
    }
    assert!(won.rocks.windows(2).all(|w| w[0].key < w[1].key), "rocks sorted by key");

    // Exclusion: nothing on water, in the site footprint or on the road; outside every water margin.
    assert_eq!(zones(&won.rocks), [0; 5], "a rock on water or in an exclusion");
    for c in &won.rocks {
        let (x, y) = (c.pos[0], c.pos[1]);
        assert!(lake_edge_distance(x, y) >= MARGIN - 1e-9 || in_lake(x, y), "{c:?} within the lake margin");
        assert!(river_edge_distance(x, y) >= MARGIN - 1e-9, "{c:?} within the river margin");
    }

    // Competition: no pair under R_ij = rock_k (r_i + r_j), r = s / 2.
    let k = values().rock_k;
    let close = won
        .rocks
        .iter()
        .enumerate()
        .flat_map(|(i, a)| won.rocks[i + 1..].iter().map(move |b| (a, b)))
        .filter(|(a, b)| (a.pos[0] - b.pos[0]).hypot(a.pos[1] - b.pos[1]) < k * (a.size_m + b.size_m) / 2.0)
        .count();
    assert_eq!(close, 0, "two rock winners closer than R_ij");

    // Rock over tree: no plant in a rock; without the rock layer some would be (non-vacuous).
    for l in [Layer::Canopy, Layer::Ground] {
        assert!(won.plants.iter().any(|c| c.key.layer == l), "{l:?} present");
    }
    assert_eq!(plants_in_rocks(&fx, &won.plants, &won.rocks), 0, "a plant in a rock footprint");
    let bare = fx.tile(&water, &excluded, false);
    assert!(bare.rocks.is_empty());
    let would = plants_in_rocks(&fx, &bare.plants, &won.rocks);
    println!("without the rock layer {would} of {} plants stand in a rock", bare.plants.len());
    assert!(would > 0, "the rocks displace plants");
}

#[test]
fn rock_layer_tile_alone() {
    let fx = Fixture::new();
    let water = WaterMask::new(&network(), MARGIN);
    let alone = fx.tile(&water, &excluded, true);
    let big = fx.build(&water, &excluded, true, [-256.0, -256.0, 512.0, 512.0]);
    let cut = |p: [f64; 2]| in_rect(p, [0.0, 0.0, 256.0, 256.0]);
    let rocks: Vec<RockCandidate> = big.rocks.into_iter().filter(|c| cut(c.pos)).collect();
    let plants: Vec<Candidate> = big.plants.into_iter().filter(|c| cut(c.pos)).collect();
    println!("tile alone: {} rocks, {} plants", alone.rocks.len(), alone.plants.len());
    assert!(!alone.rocks.is_empty() && !alone.plants.is_empty());
    assert_eq!(alone.rocks, rocks, "rocks: tile alone == larger build");
    assert_eq!(alone.plants, plants, "plants: tile alone == larger build");
}
