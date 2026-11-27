#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! WG-E2j.1: the inputs hash is the same twice and moves with the seed, the guide, the climate and every named
//! part; a written product reads back bit for bit (plants from a real layered run, rocks of every context), a
//! product of other inputs is stale, and a tampered or truncated file is refused.

use super::*;
use crate::climate::ClimateFile;
use crate::climate_classes::ClassTable;
use crate::guide::RegionMap;
use crate::scatter::{CandidateRules, WaterMask};
use crate::scatter_fields::{FieldSample, NeutralFields, Suitability};
use crate::scatter_species::SpeciesFile;
use crate::scatter_stand::{StandMap, StandRules};

/// A canopy, an understorey and ground flora.
const SPECIES: &str = r#"
[params]
"scatter.stand_clearing_threshold" = { v = 0.0, unit = "ratio", min = 0, max = 1, source = "Guess" }

[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 500, source = "Guess" }
offspring_mean = { unit = "count", min = 1, max = 60, source = "Guess" }
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }

[[species]]
name = "tree"
layer = "canopy"
size_law = "reverse_j"
parent_per_ha = 20.0
offspring_mean = 20.0

[[species]]
name = "shrub"
layer = "understorey"
size_law = "reverse_j"
size_min_m = 0.5
size_max_m = 3.0

[[species]]
name = "fern"
layer = "ground"
size_law = "weibull"
size_min_m = 0.2
size_max_m = 0.6
"#;

fn guide() -> Guide {
    let (w, h) = (16, 12);
    let heights = (0..w * h).map(|i| 10.0 + (i % w) as f32 * 40.0 + (i / w) as f32 * 5.0).collect();
    let mut names = vec![String::new(); 256];
    names[1] = "Ash Plain".into();
    Guide {
        w,
        h,
        spacing_m: 100.0,
        sea_level_m: 0.0,
        fill_m: None,
        origin_m: [500.0, -300.0],
        heights,
        regions: Some(RegionMap { w: 4, h: 3, cell_m: 400.0, index: vec![1; 12], names }),
    }
}

fn climate(g: &Guide) -> ClimateMap {
    crate::climate_map::build(g, &ClimateFile::defaults(), &ClassTable::whittaker()).unwrap()
}

fn hash(seed: u64, g: &Guide, c: &ClimateMap, species: &str) -> String {
    InputsHash::new().guide(g).climate(c).seed(seed).bytes("species", species.as_bytes()).value("tile_m", 256.0).finish()
}

fn one(_: usize, _: f64, _: f64) -> f64 {
    1.0
}

/// The kept set of one layered tile on neutral fields, plus a rock of every context (negative tile, odd floats).
fn computed() -> LayerWinners {
    let file = SpeciesFile::parse_toml(SPECIES).unwrap();
    let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, &[])).collect();
    let fields = NeutralFields {
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
    };
    let none = |_: f64, _: f64| false;
    let rules = CandidateRules {
        file: &file,
        suitability: &suit,
        seed: 7,
        tile_m: 256.0,
        prior: &one,
        water: &WaterMask::default(),
        height: &|_: f64, _: f64| 0.0,
        rocks: None,
    };
    let stand = StandMap::build(StandRules { values: &file.values, seed: 7, canopy: &suit }, &fields, &none, rules.tile_rect((-1, 2)));
    let mut won = rules.tile_layers(&fields, &stand, (-1, 2));
    for (i, context) in [RockContext::Talus, RockContext::Outcrop, RockContext::Field].into_iter().enumerate() {
        let i = i as u32;
        won.rocks.push(RockCandidate {
            key: RockKey { context, tile: (-3, 4), parent: i, offspring: u32::MAX - i },
            pos: [-12.5 + f64::from(i), 0.1 + 0.2],
            size_m: 1.0 / 3.0,
            tiebreak: u64::MAX - u64::from(i),
            source: [-0.0, 1e-300],
        });
    }
    won
}

fn temp_dir(tag: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("sf_worldgen_scatter_product_{tag}_{}", std::process::id()))
}

#[test]
fn the_inputs_hash_is_stable_and_moves_with_every_input() {
    let g = guide();
    let c = climate(&g);
    let base = hash(7, &g, &c, SPECIES);
    assert_eq!(base, hash(7, &g, &c, SPECIES), "same inputs, same hash");
    assert_ne!(base, hash(8, &g, &c, SPECIES), "seed");
    assert_ne!(base, hash(7, &g, &c, &SPECIES.replace("20.0", "21.0")), "species file");
    let mut g2 = g.clone();
    g2.heights[5] += 0.5;
    assert_ne!(base, hash(7, &g2, &c, SPECIES), "a guide height");
    let mut g3 = g.clone();
    g3.regions.as_mut().unwrap().index[3] = 2;
    assert_ne!(base, hash(7, &g3, &c, SPECIES), "the region grid");
    let mut c2 = c.clone();
    c2.moisture[0] += 0.01;
    assert_ne!(base, hash(7, &g, &c2, SPECIES), "a climate raster");
    let tile = InputsHash::new().guide(&g).climate(&c).seed(7).bytes("species", SPECIES.as_bytes()).value("tile_m", 128.0).finish();
    assert_ne!(base, tile, "a named value");
    // Length prefixes: moving bytes from one part into the next is another input.
    assert_ne!(InputsHash::new().bytes("a", b"xy").bytes("b", b"z").finish(), InputsHash::new().bytes("a", b"x").bytes("b", b"yz").finish());
}

#[test]
fn reads_back_bit_for_bit_and_knows_stale_and_tampered_products() {
    let won = computed();
    let of = |l: Layer| won.plants.iter().filter(|c| c.key.layer == l).count();
    println!("{} trees, {} shrubs, {} ferns, {} rocks", of(Layer::Canopy), of(Layer::Understorey), of(Layer::Ground), won.rocks.len());
    assert!(of(Layer::Canopy) > 0 && of(Layer::Understorey) > 0 && of(Layer::Ground) > 0);
    let dir = temp_dir("read");
    let _ = std::fs::remove_dir_all(&dir);
    assert!(cached(&dir, "inputs-a").unwrap().is_none(), "no product yet");
    let product = write(&dir, "inputs-a", &won).unwrap();
    let (inputs, back) = read(&dir).unwrap();
    assert_eq!(inputs, "inputs-a");
    assert_eq!(back, won);
    let bits = |w: &LayerWinners| {
        let p: Vec<u64> = w.plants.iter().flat_map(|c| [c.pos[0], c.pos[1], c.size_m, c.crown_m]).map(f64::to_bits).collect();
        let r: Vec<u64> = w.rocks.iter().flat_map(|c| [c.pos[0], c.pos[1], c.size_m, c.source[0], c.source[1]]).map(f64::to_bits).collect();
        (p, r)
    };
    assert_eq!(bits(&back), bits(&won), "floats bit for bit (-0.0 kept)");
    assert_eq!(cached(&dir, "inputs-a").unwrap(), Some(won.clone()));
    assert!(cached(&dir, "inputs-b").unwrap().is_none(), "other inputs: stale");
    assert_eq!(write(&dir, "inputs-a", &won).unwrap(), product, "same set, same product hash");

    // One flipped byte: refused, naming the file (by read and by cached alike).
    let path = dir.join(DIR).join("plants.bin");
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[9] ^= 0x01;
    std::fs::write(&path, &bytes).unwrap();
    let err = read(&dir).unwrap_err().to_string();
    assert!(err.contains("plants.bin blake3"), "{err}");
    assert!(cached(&dir, "inputs-a").is_err());
    // A truncated file: refused on its size.
    bytes.truncate(bytes.len() - 1);
    std::fs::write(&path, &bytes).unwrap();
    let err = read(&dir).unwrap_err().to_string();
    assert!(err.contains(&format!("plants.bin is {} bytes", bytes.len())), "{err}");
    std::fs::remove_dir_all(&dir).unwrap();
}
