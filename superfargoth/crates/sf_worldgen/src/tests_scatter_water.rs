#![allow(clippy::cast_precision_loss)] // counts of test fixtures
//! WG-E2g: v2 exclusion. The water mask's answers are exact on simple geometry (a lake with an island, a river of
//! growing width, the sea); no winner of any layer sits in a lake, a river band, the sea, a settlement footprint, an
//! excluded cell or an earlier stem footprint (and none of the zones is vacuous: a dry build puts instances in
//! every one); a shore species grows only near inland water, and not at all without the water network.

use super::*;
use crate::scatter_fields::{FieldSample, NeutralFields};
use crate::scatter_stand::StandRules;
use crate::water::{Lake, River};

/// Three layers and a shore reed (water distance hat 0-4 m full, gone at 12 m).
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
hat_water_distance_m = { unit = "m", min = 0, max = 10000, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 20.0
offspring_mean = 20.0
cluster_sigma_m = 8.0
footprint_ratio = 0.05

[[species]]
name = "shrub"
layer = "understorey"
size_law = "weibull"
size_min_m = 0.5
size_max_m = 2.0
crown_a_m = 0.3
shade_tolerance = 0.5
parent_per_ha = 60.0
offspring_mean = 30.0
cluster_sigma_m = 6.0
footprint_ratio = 0.1

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

[[species]]
name = "reed"
layer = "ground"
size_law = "reverse_j"
size_min_m = 0.5
size_max_m = 1.5
crown_a_m = 0.1
shade_tolerance = 1.0
parent_per_ha = 200.0
offspring_mean = 20.0
cluster_sigma_m = 3.0
footprint_ratio = 0.05
hat_water_distance_m = [0, 0, 4, 12]
"#;

const MARGIN: f64 = 1.0;
const REED: u32 = 3;

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
            // Far: only the exact inland water brings a reed's hat in range.
            water_distance_m: 1000.0,
        },
        class_names: Vec::new(),
    }
}

fn one(_: usize, _: f64, _: f64) -> f64 {
    1.0
}

/// Sea west of x = 15 m (ground at -1 m under a sea at 0 m), 10 m elsewhere.
fn height(x: f64, _: f64) -> f64 {
    if x < 15.0 { -1.0 } else { 10.0 }
}

/// A square lake 40-110 m with a square island 65-85 m; a straight river along y = 190 m, 6 m wide at x = 0
/// widening to 10 m at x = 128, then 10 m.
fn network() -> GuideWater {
    let square = |a: f64, b: f64| vec![[a, a], [b, a], [b, b], [a, b]];
    GuideWater {
        sea_level_m: 0.0,
        lakes: vec![Lake { level_m: 5.0, outline: square(40.0, 110.0), holes: vec![square(65.0, 85.0)], ..Lake::default() }],
        rivers: vec![River {
            points: vec![[0.0, 190.0, 6.0, 5.0], [128.0, 190.0, 10.0, 5.0], [256.0, 190.0, 10.0, 5.0]],
            ..River::default()
        }],
    }
}

/// A settlement footprint: the disc of radius 30 m round (190, 70).
fn in_footprint(x: f64, y: f64) -> bool {
    (x - 190.0).hypot(y - 70.0) < 30.0
}

/// An excluded cell: [128, 160) x [100, 132).
fn in_cell(x: f64, y: f64) -> bool {
    (128.0..160.0).contains(&x) && (100.0..132.0).contains(&y)
}

fn excluded(x: f64, y: f64) -> bool {
    in_footprint(x, y) || in_cell(x, y)
}

/// Brute force, independent of the mask: inside the lake (outline minus island).
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

/// Brute force: distance from the river's bank (negative inside), by dense sampling of its centre line (never
/// below the exact swept-disc distance).
fn river_edge_distance(x: f64, y: f64) -> f64 {
    let pts = [[0.0, 190.0, 3.0], [128.0, 190.0, 5.0], [256.0, 190.0, 5.0]];
    let mut best = f64::INFINITY;
    for w in pts.windows(2) {
        for i in 0..=4_000 {
            let t = f64::from(i) / 4_000.0;
            let c = [w[0][0] + t * (w[1][0] - w[0][0]), w[0][1] + t * (w[1][1] - w[0][1])];
            let half = w[0][2] + t * (w[1][2] - w[0][2]);
            best = best.min((x - c[0]).hypot(y - c[1]) - half);
        }
    }
    best
}

fn parse() -> (SpeciesFile, Vec<Suitability>) {
    let file = SpeciesFile::parse_toml(SPECIES).expect("fixture parses");
    let suit = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    (file, suit)
}

/// Winners of tile (0, 0) with the given water and exclusion.
fn winners(water: &WaterMask, excluded: &(dyn Fn(f64, f64) -> bool + Sync)) -> (SpeciesFile, Vec<Candidate>) {
    let (file, suit) = parse();
    let fields = neutral();
    let cand = CandidateRules { file: &file, suitability: &suit, seed: 77, tile_m: 256.0, prior: &one, water, height: &height };
    let stand = StandMap::build(StandRules { values: &file.values, seed: 77, canopy: &suit }, &fields, excluded, cand.tile_rect((0, 0)));
    let won = cand.tile_winners(&fields, &stand, (0, 0));
    (file, won)
}

#[test]
fn the_mask_answers_exactly_on_a_lake_a_river_and_the_sea() {
    let m = WaterMask::new(&network(), MARGIN);
    // Lake and island.
    assert!(m.in_lake(50.0, 75.0) && m.wet(50.0, 75.0, 10.0));
    assert!(!m.in_lake(75.0, 75.0), "the island is dry");
    assert_eq!(m.distance_within_m(50.0, 75.0, 50.0), Some(0.0));
    assert_eq!(m.distance_within_m(75.0, 75.0, 50.0), Some(10.0));
    assert_eq!(m.distance_within_m(30.0, 75.0, 50.0), Some(10.0));
    assert_eq!(m.distance_within_m(30.0, 75.0, 5.0), None, "beyond the cap");
    assert!(m.wet(39.5, 75.0, 10.0) && !m.wet(38.5, 75.0, 10.0), "the margin round the shore");
    // River: centre y = 190, half width 5 past x = 128.
    assert!((m.distance_within_m(192.0, 200.0, 50.0).unwrap() - 5.0).abs() < 1e-9);
    assert!(m.wet(192.0, 195.5, 10.0) && !m.wet(192.0, 196.5, 10.0));
    assert_eq!(m.distance_within_m(192.0, 193.0, 50.0), Some(0.0));
    // The widening piece (half width 3 -> 5 over 128 m): the swept discs, a hair under the 6 m at the foot.
    let tapered = m.distance_within_m(64.0, 200.0, 50.0).unwrap();
    let k: f64 = 2.0 / 128.0;
    let u = 64.0 + k * 10.0 / (1.0 - k * k).sqrt();
    assert!((tapered - ((u - 64.0f64).hypot(10.0) - (3.0 + k * u))).abs() < 1e-9, "tapered {tapered}");
    assert!(tapered < 6.0 && tapered > 5.99);
    assert!((tapered - river_edge_distance(64.0, 200.0)).abs() < 1e-3, "brute force agrees");
    // Sea: ground under sea level + margin.
    assert!(m.wet(200.0, 10.0, 0.5) && !m.wet(200.0, 10.0, 1.5));
    // No network: only the sea rule, no distance.
    let sea = WaterMask::sea(0.0, MARGIN);
    assert!(!sea.wet(50.0, 75.0, 10.0) && sea.wet(50.0, 75.0, 0.0));
    assert_eq!(sea.distance_within_m(50.0, 75.0, 1e6), None);
    assert!(!WaterMask::default().wet(0.0, 0.0, -1e9));
}

#[test]
fn nothing_in_lake_river_sea_footprint_or_excluded_cell() {
    // Non-vacuous: a dry build with no exclusion puts instances in every zone.
    let never = |_: f64, _: f64| false;
    let (_, dry) = winners(&WaterMask::default(), &never);
    let zones = |won: &[Candidate]| {
        let count = |f: &dyn Fn(f64, f64) -> bool| won.iter().filter(|c| f(c.pos[0], c.pos[1])).count();
        [
            count(&in_lake),
            count(&|x, y| river_edge_distance(x, y) < MARGIN),
            count(&|x, _| x < 15.0),
            count(&in_footprint),
            count(&in_cell),
        ]
    };
    let dry_zones = zones(&dry);
    println!("dry build: {} instances, zones (lake, river, sea, footprint, cell) {dry_zones:?}", dry.len());
    assert!(dry_zones.iter().all(|&n| n > 0), "every zone holds instances in a dry build");
    assert_eq!(dry.iter().filter(|c| c.key.species == REED).count(), 0, "no reed far from water");

    let water = WaterMask::new(&network(), MARGIN);
    let (file, won) = winners(&water, &excluded);
    println!("wet build: {} instances", won.len());
    assert_eq!(zones(&won), [0; 5], "nothing on water or in an exclusion");
    for c in &won {
        let (x, y) = (c.pos[0], c.pos[1]);
        assert!(lake_edge_distance(x, y) >= MARGIN - 1e-9 || in_lake(x, y), "{c:?} within the lake margin");
        assert!(river_edge_distance(x, y) >= MARGIN - 1e-6, "{c:?} within the river margin");
    }
    // Every layer is present and no later instance sits in an earlier stem footprint.
    let foot = |c: &Candidate| file.species[c.key.species as usize].footprint_ratio * c.size_m;
    for l in [Layer::Canopy, Layer::Understorey, Layer::Ground] {
        assert!(won.iter().any(|c| c.key.layer == l), "{l:?} present");
    }
    let overlaps = won
        .iter()
        .flat_map(|a| won.iter().map(move |b| (a, b)))
        .filter(|(a, b)| a.key.layer < b.key.layer)
        .filter(|(a, b)| (a.pos[0] - b.pos[0]).hypot(a.pos[1] - b.pos[1]) < foot(a) + foot(b))
        .count();
    assert_eq!(overlaps, 0, "a later layer in an earlier stem footprint");

    // The shore reed: present, and only within its hat's reach of inland water.
    let reeds: Vec<&Candidate> = won.iter().filter(|c| c.key.species == REED).collect();
    let far = reeds
        .iter()
        .map(|c| lake_edge_distance(c.pos[0], c.pos[1]).min(river_edge_distance(c.pos[0], c.pos[1]).max(0.0)))
        .fold(0.0, f64::max);
    println!("{} reeds, farthest {far:.2} m from water", reeds.len());
    assert!(reeds.len() > 20, "reeds grow along the shores");
    assert!(far <= 12.0 + 1e-6, "a reed beyond its shore hat");
}
