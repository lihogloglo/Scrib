//! Tests of the `reach` run (WG-V1.3): a tiny guide folder (an island with a pond and one road, a far islet), the
//! verdict file written and read back, the census counts.

#![allow(clippy::float_cmp)] // exact results are the point of these tests

use std::path::Path;

use super::*;
use crate::anchors::AnchorPlace;
use crate::sites::Via;

/// Guide fixture: 100 x 40 samples at 10 m, origin (0, 0), sea level 0.
const W: usize = 100;
const H: usize = 40;
const S: f64 = 10.0;

fn island(c: usize, r: usize) -> bool {
    (5..=55).contains(&c) && (5..=34).contains(&r)
}

fn islet(c: usize, r: usize) -> bool {
    (75..=90).contains(&c) && (15..=25).contains(&r)
}

fn pond(c: usize, r: usize) -> bool {
    (20..=24).contains(&c) && (15..=19).contains(&r)
}

/// Writes the guide folder: plateaus at 5 m in a -10 m sea, a 2 m pond bed under a 4.5 m lake, one road.
fn write_fixture(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    let meta = serde_json::json!({ "format": 1, "dims": [W, H], "sample_spacing_m": S, "sea_level_m": 0.0,
                                   "origin_m": [0.0, 0.0] });
    std::fs::write(dir.join("meta.json"), meta.to_string()).unwrap();
    let heights: Vec<u8> = (0..W * H)
        .flat_map(|i| {
            let (c, r) = (i % W, i / W);
            let h: f32 = if pond(c, r) {
                2.0
            } else if island(c, r) || islet(c, r) {
                5.0
            } else {
                -10.0
            };
            h.to_le_bytes()
        })
        .collect();
    std::fs::write(dir.join("heights.f32"), heights).unwrap();
    let water = serde_json::json!({ "format": 1, "sea_level_m": 0.0, "lakes": [{ "id": 1, "level_m": 4.5,
        "outline": [[195.0, -145.0], [245.0, -145.0], [245.0, -195.0], [195.0, -195.0]] }], "rivers": [] });
    std::fs::write(dir.join("water.json"), water.to_string()).unwrap();
    let roads = r#"{ "format": 1, "segments": [{ "id": 1, "class": "local", "surface": "dirt", "width_m": 4.0,
        "a": 0, "b": 1, "points": [[100.0, -100.0], [500.0, -100.0]], "crossings": [], "length_m": 400.0,
        "grade": 0.0 }], "junctions": [] }"#;
    std::fs::write(dir.join("roads.json"), roads).unwrap();
    let site = |key: &str, reach: &str, c: [f64; 2]| {
        serde_json::json!({ "key": key, "row": "r", "reach": reach, "centre_m": c, "radius_m": 5.0,
                            "kind": "remote", "kept": false })
    };
    let sites = serde_json::json!({ "format": 2, "seed": 1, "guide_blake3": "g", "table_blake3": "t",
        "packs_blake3": "p", "sites": [
            { "key": "home", "centre_m": [100.0, -100.0], "radius_m": 20.0, "kind": "town", "kept": true },
            site("inland", "foot", [400.0, -300.0]),
            site("isle", "foot", [820.0, -200.0]),
            site("isle-swim", "swim", [820.0, -200.0]),
        ] });
    std::fs::write(dir.join("sites.json"), sites.to_string()).unwrap();
}

fn request(dir: &Path) -> ReachRequest {
    ReachRequest {
        guide: dir.to_path_buf(),
        sites: dir.join("sites.json"),
        roads: dir.join("roads.json"),
        out: dir.join("out").join(super::super::VALID_FILE),
        sets: vec![("reach.cell_m".into(), S)],
        rules: BTreeMap::from([("r".to_owned(), SiteRule::NONE)]),
    }
}

#[test]
fn ground_level_is_the_coarsest_spacing_within_a_cell() {
    // Level spacings 0.5 * 2^L m: 16 m (L 5) <= 18.288 < 32 m.
    assert_eq!(ground_level(18.288), 5);
    assert_eq!(ground_level(8.0), 4);
    assert_eq!(ground_level(0.3), 0);
}

#[test]
fn anchor_footprints_move_rigidly() {
    let a = AnchorPlace {
        name: "a".into(),
        stamp: None,
        source_centre_m: [10.0, 20.0],
        world_centre_m: [100.0, 200.0],
        lift_m: 0.0,
        members: Vec::new(),
        footprint: Footprint { discs: vec![[12.0, 18.0, 3.0]] },
    };
    let t = AnchorTransform::new(10.0, vec![a]);
    assert_eq!(anchor_footprints(&t), vec![Footprint { discs: vec![[102.0, 198.0, 3.0]] }]);
}

#[test]
fn run_on_a_guide_folder_writes_verdicts_and_census() {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_reach_run_{}", std::process::id()));
    write_fixture(&dir);
    let req = request(&dir);

    // Ground: the lake's surface over the pond, the sea level over the sea, land elsewhere.
    let tiles = GuideTiles::open(&dir, WorldspaceId::new("reach")).unwrap();
    let water = WaterMask::new(&GuideWater::read(&dir).unwrap().unwrap(), WATER_MARGIN_M);
    let (g, level) = final_ground(&tiles, &water, S).unwrap();
    assert_eq!(([g.w, g.h], level), ([W, H], 4));
    let at = |c: usize, r: usize| (g.height[r * W + c], g.water[r * W + c]);
    assert_eq!(at(22, 17), (4.5, true), "pond: the lake level");
    assert_eq!(at(65, 20), (0.0, true), "sea: the sea level");
    assert!(!at(40, 10).1 && (at(40, 10).0 - 5.0).abs() < 1.0, "plateau is dry ground near 5 m: {:?}", at(40, 10));

    let (valid, census, _) = run_reach(&req).unwrap();
    assert_eq!(SitesFile::read(&req.out).unwrap(), valid, "the written file is the returned one");
    let s = |k: &str| valid.site(k).unwrap();
    assert_eq!((s("home").status, s("home").travel_s), (Some(Status::Placed), Some(0.0)));
    assert_eq!((s("inland").status, s("inland").via), (Some(Status::Placed), Some(Via::Foot)));
    assert!(s("inland").travel_s.unwrap() > 0.0);
    assert_eq!(s("isle").status, Some(Status::Dropped));
    assert_eq!((s("isle-swim").status, s("isle-swim").via), (Some(Status::Placed), Some(Via::Swim)));

    // Census: land = island minus pond + islet; the islet is reached only by swimming.
    let land_island = 51 * 30 - 25;
    let land_islet = 16 * 11;
    assert_eq!((census.grid, census.level, census.land_cells), ([W, H], 4, land_island + land_islet));
    assert_eq!((census.land_foot, census.land_swim_only), (land_island, land_islet));
    let kept = &census.groups["kept town"];
    assert_eq!((kept.placed, kept.unreached, kept.travel_p50_s), (1, 0, Some(0.0)));
    let r = &census.groups["r"];
    assert_eq!((r.placed, r.reseated, r.dropped, r.unreached), (2, 0, 1, 0));
    assert!(r.travel_p95_s >= r.travel_p50_s && r.travel_p50_s.is_some());
    assert!(census.to_string().contains("  r: placed 2, reseated 0, dropped 1, unreached 0;"), "{census}");

    // Same result twice; a value outside `reach.*` is refused.
    assert_eq!(run_reach(&req).unwrap().0, valid);
    let mut bad = request(&dir);
    bad.sets = vec![("scatter.water_margin_m".into(), 1.0)];
    assert!(run_reach(&bad).unwrap_err().to_string().contains("not a reach value"));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn percentiles_nearest_rank() {
    let v = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    assert_eq!((percentile(&v, 0.5), percentile(&v, 0.95)), (Some(5.0), Some(10.0)));
    assert_eq!(percentile(&[], 0.5), None);
}
