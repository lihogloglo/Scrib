//! LF-A1 tests: the Vvardenfell geology table parses, names every region of the island and resolves its layers.

use std::path::Path;

use sf_worldgen::geology::GeologyTable;
use sf_worldgen::guide::{Guide, RegionMap};

use crate::geology::{AdapterGeology, TABLE_FILE, coverage_report, uncovered};

const TABLE: &str = include_str!("../../../worldgen/geology.toml");

/// Legend names of the island's regions (`regions.json` `name` of the Morrowind + Tribunal + Bloodmoon REGNs).
const ISLAND_REGIONS: [&str; 15] = [
    "Ascadian Isles Region",
    "Ashlands Region",
    "Azura's Coast Region",
    "Bitter Coast Region",
    "Solstheim, Brodir Grove Region",
    "Solstheim, Felsaad Coast Region",
    "Grazelands Region",
    "Solstheim, Hirstaang Forest",
    "Solstheim, Isinfier Plains",
    "Solstheim, Moesring Mountains",
    "Molag Amur Region",
    "Red Mountain Region",
    "Sheogorad Region",
    "Thirsk Region",
    "West Gash Region",
];

fn all_layer_names(g: &AdapterGeology) -> Vec<String> {
    g.layer_names
        .values()
        .flat_map(|n| [n.top.clone(), n.face.clone(), n.scree.clone()])
        .collect()
}

#[test]
fn vvardenfell_table_parses_and_names_every_region() {
    let g = AdapterGeology::from_toml(TABLE).expect("geology.toml parses");
    let units: Vec<&str> = g.table.units.iter().map(|u| u.name.as_str()).collect();
    assert_eq!(
        units,
        ["basalt_flow", "tuff", "tephra", "capped_sediment", "coastal_basalt", "alluvium"]
    );
    let mut rows: Vec<&str> = g.table.region.keys().map(String::as_str).collect();
    let mut want = ISLAND_REGIONS.to_vec();
    rows.sort_unstable();
    want.sort_unstable();
    assert_eq!(rows, want, "one row per island region, no stray rows");
    // Every row and the default draw from at least one unit; outcrop units are layered.
    for u in &g.table.units {
        assert!(!u.outcrop || u.layered, "{}: an outcrop unit has beds", u.name);
    }
    assert!(g.table.declared().is_ok(), "[values] names declared values only");
}

#[test]
fn layer_names_resolve_against_a_palette_case_insensitively() {
    let g = AdapterGeology::from_toml(TABLE).unwrap();
    let mut palette = vec!["@default".to_string()];
    palette.extend(all_layer_names(&g).into_iter().map(|n| n.to_ascii_uppercase()));
    let t = g.resolve_layers(&palette).expect("every name resolves");
    for u in &t.units {
        let n = &g.layer_names[&u.name];
        for (id, name) in [(u.layers.top, &n.top), (u.layers.face, &n.face), (u.layers.scree, &n.scree)] {
            assert!(id > 0, "{}: {name} resolved", u.name);
            assert!(palette[usize::from(id)].eq_ignore_ascii_case(name));
        }
    }
    let short: Vec<String> = palette.iter().filter(|p| !p.eq_ignore_ascii_case("rm_grayrock")).cloned().collect();
    let err = g.resolve_layers(&short).unwrap_err().to_string();
    assert!(err.contains("basalt_flow.scree = rm_grayrock"), "{err}");
}

#[test]
fn resolved_toml_round_trips_through_the_framework() {
    let g = AdapterGeology::from_toml(TABLE).unwrap();
    let mut palette = vec!["@default".to_string()];
    palette.extend(all_layer_names(&g));
    let text = g.resolved_toml(&palette).unwrap();
    let back = GeologyTable::from_toml(&text).expect("framework parses the resolved table");
    assert_eq!(back, g.resolve_layers(&palette).unwrap());
}

#[test]
fn adapter_checks_unit_layer_names() {
    let without = TABLE.replace("[layer_names.tephra]", "[layer_names.not_a_unit]");
    let err = AdapterGeology::from_toml(&without).unwrap_err().to_string();
    assert!(err.contains("tephra has no [layer_names"), "{err}");
}

#[test]
fn region_coverage_counts_land_per_region() {
    let g = AdapterGeology::from_toml(TABLE).unwrap();
    // 4 x 2 samples at 10 m, two 20 m region pixels per row: west = Red Mountain, east = a region without a row.
    let mut names = vec![String::new(); 256];
    names[1] = "Red Mountain Region".into();
    names[2] = "Nowhere Region".into();
    let guide = Guide {
        w: 4,
        h: 2,
        spacing_m: 10.0,
        sea_level_m: 0.0,
        fill_m: None,
        origin_m: [0.0, 0.0],
        heights: vec![5.0, 5.0, 5.0, -1.0, 5.0, -2.0, 0.0, 3.0],
        regions: Some(RegionMap {
            w: 2,
            h: 1,
            cell_m: 20.0,
            index: vec![1, 2],
            names,
        }),
    };
    let cov = g.region_coverage(&guide);
    let got: Vec<(u8, usize, bool)> = cov.iter().map(|c| (c.index, c.land_samples, c.row)).collect();
    assert_eq!(got, [(1, 3, true), (2, 2, false)]);
    let missing: Vec<&str> = uncovered(&cov).iter().map(|c| c.name.as_str()).collect();
    assert_eq!(missing, ["Nowhere Region"]);
}

/// The real guide: every land region of `target/worldgen/x10s` has a row and every layer resolves in its palette;
/// writes the resolved table to `target/worldgen/lf-a1/geology.resolved.toml` for `scrib-worldgen geology`.
#[test]
#[ignore = "data: needs target/worldgen/x10s (made from the player's install)"]
fn vvardenfell_table_on_x10s() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let base = root.join("target/worldgen/x10s");
    let text = std::fs::read_to_string(root.join("bevyrrowind/worldgen").join(TABLE_FILE)).unwrap();
    let g = AdapterGeology::from_toml(&text).unwrap();
    let guide = Guide::read(&base).unwrap();
    let cov = g.region_coverage(&guide);
    println!("LF-A1 region coverage on x10s:\n{}", coverage_report(&cov));
    assert!(uncovered(&cov).is_empty(), "land regions without a row: {:?}", uncovered(&cov));
    let layers = sf_worldgen::layers::GuideLayers::read(&base).unwrap().expect("x10s has layers.json");
    let resolved = g.resolved_toml(&layers.palette).unwrap();
    let out = root.join("target/worldgen/lf-a1");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("geology.resolved.toml"), resolved).unwrap();
    let t = g.resolve_layers(&layers.palette).unwrap();
    for u in &t.units {
        println!("{:<16} layers top {} face {} scree {}", u.name, u.layers.top, u.layers.face, u.layers.scree);
    }
}
