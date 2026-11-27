//! Tests of `sites`: the `sites.json` reader (formats 1 and 2).

// Values are copied through JSON, never recomputed, so they compare exactly.
// Coordinates are copied verbatim from a real file.
#![allow(clippy::float_cmp, clippy::unreadable_literal)]

use super::*;

/// Format 1, shaped from the head of a real router input (`s2d-roads/sites-cocentred-r1.json`).
const V1: &str = r#"{
 "format": 1,
 "producer": "scratchpad sites.mjs (lead anchors; populations Guess)",
 "sites": [
  { "key": "vivec", "name": "Vivec", "population": 4000, "culture": "dunmer",
    "centre_m": [4690.9, -13037.8], "radius_m": 289, "anchor": "Vivec, St. Olms" },
  { "key": "balmora-2", "name": "Balmora 2.0", "population": 3000, "culture": "dunmer",
    "centre_m": [-2866.352667678369, -2039.7624680161573], "radius_m": 1, "anchor": null }
 ]
}"#;

fn generated() -> Site {
    let mut s = Site::from(SiteV1 {
        key: "site:ruin.crag:remote:7".into(),
        name: String::new(),
        population: 0,
        culture: "c".into(),
        centre_m: [120.25, -3.5],
        radius_m: 0.0,
        laid_out: false,
        anchor: None,
    });
    s.name = "Ald Crag".into();
    s.name_key = "g.ruin".into();
    s.kind = SiteKind::Remote;
    s.kept = false;
    s.row = "ruin.crag".into();
    s.template = "crag_ruin".into();
    s.mode = "remote_seasonal".into();
    s.parent = "vivec".into();
    s.ground_m = Some(212.125);
    s.yaw_deg = 33.0;
    s.footprint = vec![[110.0, -10.0], [130.0, -10.0], [130.0, 4.0], [110.0, 4.0]];
    s.access = Some([108.0, -12.0]);
    s.access_class = Some(AccessClass::Trail);
    s.approach = "rock-steps".into();
    s.situation = "summit".into();
    s.kit = "ruin.k".into();
    s.clearing_m = 12.0;
    s.pad = true;
    s.reach = Reach::Foot;
    s.buildings = vec![serde_json::json!({ "ring": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], "kind": "tower" })];
    s.installations = vec![serde_json::json!({ "kind": "rock-steps", "line": [[1.0, 2.0], [3.0, 4.0]] })];
    s.status = Some(Status::Reseated);
    s.moved_m = Some(6.5);
    s.travel_s = Some(1834.0);
    s.via = Some(Via::Swim);
    s
}

fn v2() -> SitesFile {
    let mut dropped = generated();
    dropped.key = "site:ruin.crag:remote:8".into();
    dropped.status = Some(Status::Dropped);
    dropped.reason = "no reachable cell within 60 m".into();
    let town = SitesFile::from_reader(V1.as_bytes()).unwrap().sites.remove(0);
    SitesFile {
        seed: 0x5eed_0000_0000_0001,
        guide_blake3: "aa".repeat(32),
        table_blake3: "bb".repeat(32),
        packs_blake3: "cc".repeat(32),
        producer: String::new(),
        sites: vec![town, generated(), dropped],
    }
}

#[test]
fn format_2_round_trips_exactly() {
    let file = v2();
    let json = file.to_json().unwrap();
    assert!(json.starts_with("{\n  \"format\": 2,"), "{json}");
    assert_eq!(SitesFile::from_reader(json.as_bytes()).unwrap(), file);

    let dir = std::env::temp_dir().join(format!("sf_worldgen_sites_rt_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("sites.json");
    file.write(&path).unwrap();
    let back = SitesFile::read(&path).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(back, file);
    assert_eq!(back.site("site:ruin.crag:remote:8").unwrap().reason, "no reachable cell within 60 m");
}

#[test]
fn format_1_reads_as_kept_towns() {
    let file = SitesFile::from_reader(V1.as_bytes()).unwrap();
    assert_eq!(file.producer, "scratchpad sites.mjs (lead anchors; populations Guess)");
    assert_eq!((file.seed, file.guide_blake3.as_str()), (0, ""));
    assert_eq!(file.sites.len(), 2);
    let v = &file.sites[0];
    assert_eq!((v.key.as_str(), v.name.as_str(), v.population, v.culture.as_str()), ("vivec", "Vivec", 4000, "dunmer"));
    assert_eq!((v.centre_m, v.radius_m, v.anchor.as_deref()), ([4690.9, -13037.8], 289.0, Some("Vivec, St. Olms")));
    let b = &file.sites[1];
    assert_eq!(b.centre_m, [-2866.352667678369, -2039.7624680161573]);
    assert_eq!(b.anchor, None);
    for s in &file.sites {
        assert_eq!((s.kind, s.kept, s.laid_out, s.pad, s.clearing_m, s.reach), (SiteKind::Town, true, false, false, 0.0, Reach::Any));
        assert!(s.access_class.is_none() && s.status.is_none() && s.footprint.is_empty());
    }
    // Written back as format 2, read again: the same.
    assert_eq!(SitesFile::from_reader(file.to_json().unwrap().as_bytes()).unwrap(), file);
}

fn err(json: &str) -> String {
    SitesFile::from_reader(json.as_bytes()).unwrap_err().to_string()
}

#[test]
fn wrong_or_missing_format_is_refused() {
    let three = V1.replacen("\"format\": 1", "\"format\": 3", 1);
    assert_eq!(err(&three), "sites.json format 3 (reads 1 and 2)");
    let text = V1.replacen("\"format\": 1", "\"format\": \"2\"", 1);
    assert_eq!(err(&text), "sites.json format \"2\" (reads 1 and 2)");
    let none = V1.replacen("\"format\": 1,", "", 1);
    assert_eq!(err(&none), "sites.json: no `format`");
}

#[test]
fn unknown_fields_are_refused() {
    // In a format-1 site, at the format-1 top level, and in a format-2 site.
    let site = V1.replacen("\"radius_m\": 289,", "\"radius_m\": 289, \"colour\": 1,", 1);
    assert!(err(&site).contains("unknown field `colour`"), "{}", err(&site));
    let top = V1.replacen("\"producer\"", "\"tool\": 0, \"producer\"", 1);
    assert!(err(&top).contains("unknown field `tool`"), "{}", err(&top));
    // A format-2 field is unknown in a format-1 file.
    let v2_in_v1 = V1.replacen("\"radius_m\": 289,", "\"radius_m\": 289, \"kept\": true,", 1);
    assert!(err(&v2_in_v1).contains("unknown field `kept`"), "{}", err(&v2_in_v1));
    let json = v2().to_json().unwrap().replacen("\"kind\": \"remote\",", "\"kind\": \"remote\", \"mood\": 2,", 1);
    assert!(err(&json).contains("unknown field `mood`"), "{}", err(&json));
    let header = v2().to_json().unwrap().replacen("\"seed\"", "\"frame\": 0, \"seed\"", 1);
    assert!(err(&header).contains("unknown field `frame`"), "{}", err(&header));
}

#[test]
fn bad_values_and_duplicate_keys_are_refused() {
    let kind = v2().to_json().unwrap().replacen("\"kind\": \"remote\"", "\"kind\": \"castle\"", 1);
    assert!(err(&kind).contains("unknown variant `castle`"), "{}", err(&kind));
    let missing = v2().to_json().unwrap().replacen("\"kept\": false,", "", 1);
    assert!(err(&missing).contains("missing field `kept`"), "{}", err(&missing));
    let twice = V1.replacen("\"key\": \"balmora-2\"", "\"key\": \"vivec\"", 1);
    assert_eq!(err(&twice), "sites.json: site key `vivec` twice");
}
