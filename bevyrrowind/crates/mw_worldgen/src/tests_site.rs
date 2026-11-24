//! ST-1 and ST-2 tests: the site folder format round-trip, the crop, the hull and the road table, on fixtures (no game data).

// Exact results are the point of these tests.
#![allow(clippy::float_cmp)]

use std::collections::BTreeMap;

use crate::anchor::Field;
use crate::site::{
    self, Claim, Crop, FootprintKind, FootprintPolygon, HEIGHTS_FILE, ROADS_FILE, SITE_FILE, Site, SiteJson, SiteParams,
    SiteTable, WATER_FILE, convex_hull, crop_around,
};

fn fixture() -> Site {
    let (w, h) = (5, 3);
    let heights: Vec<f32> = (0..w * h).map(|i| f32::from(u8::try_from(i).unwrap()) * 0.5 - 2.0).collect();
    let water = heights.iter().map(|&v| u8::from(v < 0.0)).collect();
    let roads = (0..w * h).map(|i| u8::from(i % 5 == 2)).collect();
    let json = SiteJson {
        format: site::FORMAT,
        name: "Fixture".into(),
        producer: "test".into(),
        origin_m: [-100.5, 2000.25],
        cell_m: 1.8288,
        dims: [w, h],
        water_level_m: 0.0,
        centre_m: [-96.0, 1997.5],
        source: "synthetic".into(),
        claim: Some(Claim {
            footprints: vec![
                FootprintPolygon { id: "a".into(), kind: FootprintKind::Building, polygon: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]] },
                FootprintPolygon {
                    id: "b".into(),
                    kind: FootprintKind::Other,
                    polygon: vec![[-1.5, 2.0], [1.0, 2.0], [1.0, 5.0], [-1.5, 5.0]],
                },
            ],
        }),
        blake3: BTreeMap::new(),
    };
    Site { json, heights, water, roads: Some(roads) }
}

#[test]
fn site_folder_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let s = fixture();
    let written = site::write(&s, dir.path()).unwrap();
    assert_eq!(written.blake3.len(), 3);
    for (name, bytes) in [(HEIGHTS_FILE, 15 * 4), (WATER_FILE, 15), (ROADS_FILE, 15)] {
        let raw = std::fs::read(dir.path().join(name)).unwrap();
        assert_eq!(raw.len(), bytes, "{name}");
        assert_eq!(written.blake3[name], blake3::hash(&raw).to_hex().to_string(), "{name}");
    }
    let back = site::read(dir.path()).unwrap();
    assert_eq!(back.json, written);
    assert_eq!(back.json.dims, [5, 3]);
    assert_eq!(back.json.origin_m, [-100.5, 2000.25]);
    assert_eq!(back.json.claim, s.json.claim);
    assert_eq!(back.heights, s.heights);
    assert_eq!(back.water, s.water);
    assert_eq!(back.roads, s.roads);
    // The contract's spellings.
    let text = std::fs::read_to_string(dir.path().join(SITE_FILE)).unwrap();
    assert!(text.contains("\"kind\": \"building\""), "{text}");
    assert!(text.contains("\"water.u8\""), "{text}");
}

#[test]
fn site_read_rejects_a_changed_raster() {
    let dir = tempfile::tempdir().unwrap();
    site::write(&fixture(), dir.path()).unwrap();
    let path = dir.path().join(WATER_FILE);
    let mut raw = std::fs::read(&path).unwrap();
    raw[0] ^= 1;
    std::fs::write(&path, raw).unwrap();
    assert!(site::read(dir.path()).is_err());
}

#[test]
fn site_without_roads_or_claim() {
    let dir = tempfile::tempdir().unwrap();
    let mut s = fixture();
    s.roads = None;
    s.json.claim = None;
    let written = site::write(&s, dir.path()).unwrap();
    assert!(!written.blake3.contains_key(ROADS_FILE));
    let back = site::read(dir.path()).unwrap();
    assert_eq!(back.roads, None);
    assert_eq!(back.json.claim, None);
    assert!(!std::fs::read_to_string(dir.path().join(SITE_FILE)).unwrap().contains("claim"));
}

#[test]
fn crop_is_centred_and_clamped() {
    let data = vec![0.0f32; 20 * 10];
    let f = Field { w: 20, h: 10, origin_m: [100.0, 50.0], spacing_m: 2.0, data: &data };
    // Centre at sample (10, 4): x = 120, y = 42.
    assert_eq!(crop_around(&f, [120.0, 42.0], 6.0), Some(Crop { col0: 7, row0: 1, w: 7, h: 7 }));
    // Near the north-west corner: clamped.
    assert_eq!(crop_around(&f, [102.0, 50.0], 6.0), Some(Crop { col0: 0, row0: 0, w: 5, h: 4 }));
    assert_eq!(crop_around(&f, [0.0, 50.0], 6.0), None);
}

#[test]
fn hull_is_counter_clockwise_without_inner_or_collinear_points() {
    let pts = [[0.0, 0.0], [2.0, 0.0], [1.0, 0.0], [2.0, 2.0], [0.0, 2.0], [1.0, 1.0], [0.0, 0.0]];
    assert_eq!(convex_hull(&pts), Some(vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]));
    assert_eq!(convex_hull(&[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]), None);
    assert_eq!(convex_hull(&[[0.0, 0.0], [1.0, 1.0]]), None);
}

#[test]
fn road_table_matches_ids_and_patterns() {
    let t = SiteTable::from_toml("[roads]\nltex_ids = [\"Road Dirt\"]\ntexture_patterns = [\"cobble\"]\n").unwrap();
    assert!(t.roads.is_road("road dirt", "tx_x.dds"));
    assert!(t.roads.is_road("other", "TX_Cobblestone_01.TGA"));
    assert!(!t.roads.is_road("grass", "tx_grass.dds"));
    SiteTable::builtin().unwrap();
    let p = SiteParams::default();
    assert_eq!(p.radius_m, 1200.0);
    assert_eq!(p.anchor_scale, 10.0);
    assert_eq!(p.guide_radius_m, 1500.0);
    assert_eq!(p.cell_m, 1.8288);
}

/// A 64 x 64 guide at 10 m (x 0..630, y 0..630), flat at 10 m with a channel at -5 m in columns 26..=30 (x 260..300,
/// the whole height); two 5 x 5 stamps of weight 1: "Keep" at 100 m (x 100..140, y 460..500), "Drop" at 200 m
/// (x 200..240); their anchors "Keep Town" and "Drop Town"; layers: grass everywhere but one global road texel
/// (x 85..125, y 445..485), Keep's own grid grass, Drop's own grid road.
fn guide_fixture(dir: &std::path::Path) {
    use sf_worldgen::anchors::{AnchorPlace, AnchorTransform, Footprint};
    use sf_worldgen::layers::{GuideLayerGrid, GuideLayers};
    use sf_worldgen::stamps::{Stamp, StampSet};
    let n = 64;
    let heights: Vec<f32> = (0..n * n).map(|k| if (26..=30).contains(&(k % n)) { -5.0 } else { 10.0 }).collect();
    std::fs::write(dir.join("heights.f32"), heights.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    let meta = r#"{ "format": 1, "dims": [64, 64], "sample_spacing_m": 10.0, "sea_level_m": 0.0, "origin_m": [0.0, 630.0] }"#;
    std::fs::write(dir.join("meta.json"), meta).unwrap();
    let stamp = |name: &str, x: f64, h: f32| Stamp {
        name: name.into(),
        origin_m: [x, 500.0],
        dims: [5, 5],
        spacing_m: 10.0,
        heights: vec![h; 25],
        weights: vec![1.0; 25],
    };
    StampSet { stamps: vec![stamp("Keep", 100.0, 100.0), stamp("Drop", 200.0, 200.0)] }.write(dir).unwrap();
    let place = |name: &str, stamp: usize, c: [f64; 2]| AnchorPlace {
        name: name.into(),
        stamp: Some(stamp),
        source_centre_m: [c[0] / 10.0, c[1] / 10.0],
        world_centre_m: c,
        lift_m: 0.0,
        members: vec![],
        footprint: Footprint { discs: vec![] },
    };
    AnchorTransform::new(10.0, vec![place("Keep Town", 0, [120.0, 480.0]), place("Drop Town", 1, [220.0, 480.0])]).write(dir).unwrap();
    let grid = |origin_m: [f64; 2], dims: [usize; 2], spacing_m: f64, texels: Vec<u16>| GuideLayerGrid {
        origin_m,
        dims,
        spacing_m,
        blend_m: 0.0,
        texels,
        tint: None,
    };
    let mut global = vec![2u16; 16 * 16];
    global[4 * 16 + 2] = 1;
    GuideLayers {
        palette: vec!["@default".into(), "road".into(), "grass".into()],
        default_layer: "@default".into(),
        global: grid([5.0, 645.0], [16, 16], 40.0, global),
        stamps: vec![(0, grid([95.0, 505.0], [5, 5], 10.0, vec![2; 25])), (1, grid([195.0, 505.0], [5, 5], 10.0, vec![1; 25]))],
    }
    .write(dir)
    .unwrap();
}

#[test]
fn guide_site_drops_the_named_stamp_and_keeps_the_others() {
    use sf_worldgen::filter::FilterParams;
    let dir = tempfile::tempdir().unwrap();
    guide_fixture(dir.path());
    // No detail: the flat guide's heights stay exact.
    let plain = |t: sf_worldgen::tiles::GuideTiles| t.with_params(FilterParams { octaves: 0, ..*t.params() }.with_sharp_scale(0.0));
    let (tiles, dropped) = site::guide_tiles(dir.path(), Some(1), 0.0).unwrap();
    assert_eq!(dropped.map(|s| s.name), Some("Drop".to_owned()));
    let tiles = plain(tiles);
    let road_of = [false, true, false];

    // The river reach nearest Drop Town's centre: the channel's middle column (x 280), same row.
    let river = site::find_river(&tiles, [220.0, 480.0], 150.0, 1000.0).unwrap();
    assert_eq!(river.nearest_m, [260.0, 480.0]);
    assert_eq!(river.centre_m, [280.0, 480.0]);
    assert_eq!(river.area_m2, 5.0 * 31.0 * 100.0);
    assert!(river.open);
    assert!(site::find_river(&tiles, [220.0, 480.0], 150.0, 1.0e6).is_none());

    let g = site::guide_site(&tiles, "Drop Town", [220.0, 480.0], 100.0, 10.0, &road_of).unwrap();
    let s = &g.site;
    assert_eq!(s.json.dims, [21, 21]);
    assert_eq!(s.json.origin_m, [115.0, 585.0]);
    assert_eq!(s.json.claim, None);
    // Cell (c, r) is centred on x = 120 + 10 c, y = 580 - 10 r.
    let at = |x: usize, y: usize| (580 - y) / 10 * 21 + (x - 120) / 10;
    assert_eq!(s.heights[at(120, 480)], 100.0, "Keep's stamp stays");
    assert_eq!(s.heights[at(220, 480)], 10.0, "Drop's stamp is gone");
    assert_eq!(s.heights[at(280, 480)], -5.0);
    assert_eq!(s.water.iter().map(|&w| usize::from(w)).sum::<usize>(), 5 * 21);
    // Roads: the global road texel's cell outside Keep's stamp only (Keep draws its own grass grid; dropped Drop's
    // road grid no longer applies).
    let roads = s.roads.as_ref().unwrap();
    assert_eq!(roads.iter().map(|&w| usize::from(w)).sum::<usize>(), 1);
    assert_eq!(roads[at(120, 450)], 1);

    // Without the drop, Drop's stamp shows.
    let (kept, none) = site::guide_tiles(dir.path(), None, 0.0).unwrap();
    assert!(none.is_none());
    let k = site::guide_site(&plain(kept), "Drop Town", [220.0, 480.0], 100.0, 10.0, &road_of).unwrap();
    assert_eq!(k.site.heights[at(220, 480)], 200.0);
    assert_eq!(k.site.roads.as_ref().unwrap()[at(220, 480)], 1);

    // The site folder round-trips.
    let out = tempfile::tempdir().unwrap();
    let written = site::write(s, out.path()).unwrap();
    assert_eq!(written.blake3.len(), 3);
    let back = site::read(out.path()).unwrap();
    assert_eq!(back.json, written);
    assert_eq!(back.json.dims, [21, 21]);
    assert_eq!(back.json.origin_m, [115.0, 585.0]);
    assert_eq!(back.heights, s.heights);
    assert_eq!(back.water, s.water);
    assert_eq!(back.roads, s.roads);
}
