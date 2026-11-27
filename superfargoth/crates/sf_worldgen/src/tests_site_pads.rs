//! Tests of the site pads (WG-P2): a small guide folder (a coast, a plateau, a ramp, one anchor stamp) and one site
//! per case: a flat ring pad, a pad on the ramp, and one site per refusal.

#![allow(clippy::float_cmp)] // exact results are the point of these tests

use std::path::{Path, PathBuf};

use super::*;

/// Guide fixture: 60 x 30 samples at 10 m, origin (0, 0), sea level 0.
const W: usize = 60;
const H: usize = 30;
const S: f64 = 10.0;

/// Sea west of x = 45 m, a plateau at 10 m, a 0.2 ramp from x = 300 m to 450 m, a plateau at 40 m beyond.
fn height(c: usize) -> f32 {
    match c {
        0..5 => -10.0,
        5..30 => 10.0,
        30..45 => 10.0 + (c - 30) as f32 * 2.0,
        _ => 40.0,
    }
}

/// The anchor stamp: 11 x 11 at 2 m from (450, -100), weight 1 inside a zero border, heights 42.
fn anchor() -> Stamp {
    let n = 11;
    let weights = (0..n * n).map(|k| if (1..n - 1).contains(&(k % n)) && (1..n - 1).contains(&(k / n)) { 1.0 } else { 0.0 });
    Stamp {
        name: "anchor:a".into(),
        origin_m: [450.0, -100.0],
        dims: [n, n],
        spacing_m: 2.0,
        heights: vec![42.0; n * n],
        weights: weights.collect(),
    }
}

fn write_guide(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    let meta = serde_json::json!({ "format": 1, "dims": [W, H], "sample_spacing_m": S, "sea_level_m": 0.0,
                                   "origin_m": [0.0, 0.0] });
    std::fs::write(dir.join("meta.json"), meta.to_string()).unwrap();
    let heights: Vec<u8> = (0..W * H).flat_map(|i| height(i % W).to_le_bytes()).collect();
    std::fs::write(dir.join("heights.f32"), heights).unwrap();
    StampSet { stamps: vec![anchor()] }.write(dir).unwrap();
}

fn site(key: &str, c: [f64; 2], r: f64) -> serde_json::Value {
    serde_json::json!({ "key": key, "row": "r", "centre_m": c, "radius_m": r, "kind": "remote", "kept": false,
                        "pad": true, "status": "placed" })
}

fn write_sites(dir: &Path) -> PathBuf {
    let mut ring = site("flat", [250.0, -150.0], 0.0);
    ring["footprint"] = serde_json::json!([[240.0, -160.0], [260.0, -160.0], [260.0, -140.0], [240.0, -140.0]]);
    let mut dropped = site("dropped", [250.0, -250.0], 5.0);
    dropped["status"] = "dropped".into();
    let mut flat_no_pad = site("no-pad", [520.0, -250.0], 5.0);
    flat_no_pad["pad"] = false.into();
    let sites = serde_json::json!({ "format": 2, "seed": 1, "guide_blake3": "g", "table_blake3": "t",
        "packs_blake3": "p", "sites": [
            ring,
            site("ramp", [330.0, -100.0], 8.0),
            site("cut", [380.0, -220.0], 20.0),
            site("coast", [60.0, -150.0], 10.0),
            site("by-anchor", [478.0, -110.0], 4.0),
            site("ov-a", [150.0, -200.0], 10.0),
            site("ov-b", [172.0, -200.0], 10.0),
            dropped,
            flat_no_pad,
        ] });
    let p = dir.join("sites.valid.json");
    std::fs::write(&p, sites.to_string()).unwrap();
    p
}

fn fixture(name: &str) -> (PathBuf, PadRequest) {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_pads_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    write_guide(&dir);
    let sites = write_sites(&dir);
    (dir.clone(), PadRequest { guide: dir, sites, sets: Vec::new() })
}

fn open(dir: &Path) -> GuideTiles {
    GuideTiles::open(dir, WorldspaceId::new("t")).unwrap()
}

fn written(dir: &Path) -> Vec<Stamp> {
    StampSet::read(dir).unwrap().unwrap().stamps.into_iter().filter(|s| s.name.starts_with(PAD_PREFIX)).collect()
}

#[test]
fn each_refusal_counted() {
    let (dir, req) = fixture("refusals");
    let rep = run_pads(&req).unwrap();
    assert_eq!(rep.asked, 7, "dropped and pad = false sites ask for nothing");
    assert_eq!(rep.written, 3);
    let keys: Vec<(&str, Refusal)> = rep.refusals.iter().map(|(k, r)| (k.as_str(), *r)).collect();
    assert_eq!(keys, [("cut", Refusal::Cut), ("coast", Refusal::Water), ("by-anchor", Refusal::Anchor), ("ov-b", Refusal::Overlap)]);
    let names: Vec<String> = written(&dir).into_iter().map(|s| s.name).collect();
    assert_eq!(names, ["pad:flat", "pad:ramp", "pad:ov-a"]);
    assert!(rep.cut_max_m > 0.0 && rep.cut_max_m <= 3.0, "cut max {}", rep.cut_max_m);

    // A lower band limit refuses the ramp pad for its band.
    let (_, mut req) = fixture("band");
    req.sets.push(("pad.max_band_deg".into(), 25.0));
    let rep = run_pads(&req).unwrap();
    assert_eq!(rep.refused.get(&Refusal::Band), Some(&1));
    assert_eq!(rep.refusals.iter().find(|(_, r)| *r == Refusal::Band).unwrap().0, "ramp");
}

#[test]
fn pad_shape_and_median() {
    let (dir, req) = fixture("shape");
    run_pads(&req).unwrap();
    let pads = written(&dir);
    let flat = &pads[0];
    let [w, h] = flat.dims;
    // Footprint 240..260 grown by 8 m, on 2 m multiples: 232..268 and -168..-132.
    assert_eq!(flat.origin_m, [232.0, -132.0]);
    assert_eq!(flat.dims, [19, 19]);
    for c in 0..w {
        assert_eq!(flat.weights[c], 0.0);
        assert_eq!(flat.weights[(h - 1) * w + c], 0.0);
    }
    for r in 0..h {
        assert_eq!(flat.weights[r * w], 0.0);
        assert_eq!(flat.weights[r * w + w - 1], 0.0);
    }
    assert_eq!(flat.weights[9 * w + 9], 1.0, "inside the ring");
    let band = flat.weights[9 * w + 1]; // 6 m outside: smootherstep(0.25)
    assert!((f64::from(band) - smootherstep(0.25)).abs() < 1e-6, "{band}");
    // Every height is the median of the ground inside the footprint (the plateau, 10 m before detail).
    assert!(flat.heights.iter().all(|&v| v == flat.heights[0]));
    assert!((flat.heights[0] - 10.0).abs() < 1.0, "median {}", flat.heights[0]);
    // The ramp pad sits at its centre's ground, give or take the ramp across the disc.
    let ramp = &pads[1];
    let g = f64::from(open(&dir).height(330.0, 100.0, ground_level(2.0)));
    assert!((f64::from(ramp.heights[0]) - g).abs() < 0.5, "ramp median {} vs centre {g}", ramp.heights[0]);
}

#[test]
fn anchor_interior_bit_exact() {
    let (dir, req) = fixture("anchor");
    let before = open(&dir);
    run_pads(&req).unwrap();
    let after = open(&dir);
    assert_eq!(after.stamps().unwrap().set().stamps.len(), 4);
    let a = anchor();
    let mut checked = 0;
    for k in 0..a.weights.len() {
        let p = sample_pos(&a, k);
        for level in [0, 2, 5] {
            assert_eq!(before.height(p[0], -p[1], level).to_bits(), after.height(p[0], -p[1], level).to_bits());
            checked += 1;
        }
    }
    // And a 0.5 m lattice over the anchor's rectangle.
    for i in 0..=40 {
        for j in 0..=40 {
            let (x, y) = (450.0 + f64::from(i) * 0.5, -100.0 - f64::from(j) * 0.5);
            assert_eq!(before.height(x, -y, 0).to_bits(), after.height(x, -y, 0).to_bits());
        }
    }
    assert_eq!(checked, 363);
}

#[test]
fn wet_dry_audit_zero_and_band_walkable() {
    let (dir, req) = fixture("audit");
    let before = open(&dir);
    run_pads(&req).unwrap();
    let after = open(&dir);
    let water = WaterMask::sea(0.0, 0.0);
    let level = ground_level(2.0);
    let max_tan = 46f64.to_radians().tan();
    let (mut flips, mut band) = (0, 0);
    for st in written(&dir) {
        for k in 0..st.weights.len() {
            let p = sample_pos(&st, k);
            let h0 = f64::from(before.height(p[0], -p[1], level));
            let h1 = f64::from(after.height(p[0], -p[1], level));
            flips += usize::from(water.wet(p[0], p[1], h0) != water.wet(p[0], p[1], h1));
            if st.weights[k] > 0.0 && st.weights[k] < 1.0 {
                band += 1;
                let t = band_tan(&after, level, p, 2.0);
                assert!(t <= max_tan, "{} band slope {:.1} deg at {p:?}", st.name, t.atan().to_degrees());
            }
        }
    }
    assert_eq!(flips, 0);
    assert!(band > 100, "band samples {band}");
}

#[test]
fn rerun_writes_same_bytes() {
    let (dir, req) = fixture("rerun");
    run_pads(&req).unwrap();
    let (j1, f1) = (std::fs::read(dir.join("stamps.json")).unwrap(), std::fs::read(dir.join("stamps.f32")).unwrap());
    run_pads(&req).unwrap();
    assert_eq!(std::fs::read(dir.join("stamps.json")).unwrap(), j1);
    assert_eq!(std::fs::read(dir.join("stamps.f32")).unwrap(), f1);
}

#[test]
fn pads_go_last() {
    let mut pad = anchor();
    pad.name = "pad:x".into();
    let set = StampSet { stamps: vec![anchor(), pad.clone(), pad.clone()] };
    assert_eq!(strip_pads(set).unwrap().stamps.len(), 1);
    let set = StampSet { stamps: vec![pad, anchor()] };
    assert!(strip_pads(set).is_err());
}

#[test]
fn bad_set_refused() {
    let (_, mut req) = fixture("sets");
    req.sets.push(("reach.cell_m".into(), 10.0));
    assert!(run_pads(&req).is_err());
    let (_, mut req) = fixture("sets2");
    req.sets.push(("pad.blend_m".into(), 100.0));
    assert!(run_pads(&req).is_err());
}
