//! S2 tests: `worldgen-derive` on a fixture guide folder (tiny stamps, no game data).

use std::path::Path;

use sf_worldgen::anchors::{AnchorPlace, AnchorTransform, Footprint};
use sf_worldgen::stamps::{Stamp, StampSet};

use crate::derive::{self, Appended, DeriveArgs, DeriveFile, Dropped, ValleyArgs};

fn stamp(name: &str, x: f64, h: f32) -> Stamp {
    let heights = (0..9u8).map(|k| h + f32::from(k) * 0.25).collect();
    Stamp { name: name.into(), origin_m: [x, 0.0], dims: [3, 3], spacing_m: 1.0, heights, weights: vec![0.5; 9] }
}

fn place(name: &str, stamp: Option<usize>) -> AnchorPlace {
    AnchorPlace {
        name: name.into(),
        stamp,
        source_centre_m: [1.0, 2.0],
        world_centre_m: [10.0, 20.0],
        lift_m: 0.0,
        members: vec![format!("{name}-ref")],
        footprint: Footprint { discs: vec![[1.0, 2.0, 3.0]] },
    }
}

/// Anchors "X" (stamp 0), "Xyz" (stamp 1), "X 2" (stamp 2), "Y" (stamp 3), "X 3" (point); a heights file.
fn guide(dir: &Path) {
    let set = StampSet { stamps: (0..4u8).map(|k| stamp(&format!("s{k}"), f64::from(k) * 100.0, f32::from(k))).collect() };
    set.write(dir).unwrap();
    let anchors =
        vec![place("X", Some(0)), place("Xyz", Some(1)), place("X 2", Some(2)), place("Y", Some(3)), place("X 3", None)];
    AnchorTransform::new(10.0, anchors).write(dir).unwrap();
    std::fs::write(dir.join("heights.f32"), [1u8, 2, 3, 4]).unwrap();
}

fn args(guide: &Path, out: &Path) -> DeriveArgs {
    DeriveArgs {
        guide: guide.to_owned(),
        out: out.to_owned(),
        drop_anchors: vec!["X".into()],
        add_stamps: Vec::new(),
        stamp_samples_max: 1_000,
        valley: None,
        sharp_scale: 0.0,
    }
}

#[test]
fn drop_names() {
    assert!(derive::drops("Balmora", "Balmora"));
    assert!(derive::drops("Balmora", "Balmora 2"));
    assert!(derive::drops("Balmora", "Balmora 13"));
    assert!(!derive::drops("Balmora", "Balmoray"));
    assert!(!derive::drops("Balmora", "Balmora "));
    assert!(!derive::drops("Balmora", "Balmora 2b"));
    assert!(!derive::drops("Balmora", "Vivec, Balmora"));
}

#[test]
fn drops_anchors_zeroes_their_stamps_keeps_indices() {
    let (g, o) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    guide(g.path());
    let out = o.path().join("derived");
    let summary = derive::run(&args(g.path(), &out)).unwrap();
    assert!(summary.contains("3 dropped, 2 kept"), "{summary}");

    let t = AnchorTransform::read(&out).unwrap().unwrap();
    let names: Vec<(&str, Option<usize>)> = t.anchors.iter().map(|a| (a.name.as_str(), a.stamp)).collect();
    assert_eq!(names, [("Xyz", Some(1)), ("Y", Some(3))]);
    assert_eq!(t.anchors[0], place("Xyz", Some(1)));
    assert_eq!(t.scale.to_bits(), 10.0f64.to_bits());

    let before = StampSet::read(g.path()).unwrap().unwrap();
    let after = StampSet::read(&out).unwrap().unwrap();
    assert_eq!(after.stamps.len(), 4);
    for k in [1, 3] {
        assert_eq!(after.stamps[k], before.stamps[k]);
    }
    for k in [0, 2] {
        assert!(after.stamps[k].weights.iter().all(|w| w.to_bits() == 0));
        assert_eq!(after.stamps[k].heights, before.stamps[k].heights);
        assert_eq!(after.stamps[k].name, before.stamps[k].name);
    }
    assert_eq!(std::fs::read(out.join("heights.f32")).unwrap(), [1, 2, 3, 4]);

    let file: DeriveFile = serde_json::from_slice(&std::fs::read(out.join("derive.json")).unwrap()).unwrap();
    let hex = |p: &str| Some(blake3::hash(&std::fs::read(g.path().join(p)).unwrap()).to_hex().to_string());
    assert_eq!(file.format, 1);
    assert_eq!(file.source_guide, g.path().display().to_string());
    assert_eq!(file.source_blake3.stamps_f32, hex("stamps.f32"));
    assert_eq!(file.source_blake3.anchors_json, hex("anchors.json"));
    assert_eq!(
        file.dropped,
        [
            Dropped { name: "X".into(), stamp: Some(0), world_centre_m: Some([10.0, 20.0]), lift_m: Some(0.0) },
            Dropped { name: "X 2".into(), stamp: Some(2), world_centre_m: Some([10.0, 20.0]), lift_m: Some(0.0) },
            Dropped { name: "X 3".into(), stamp: None, world_centre_m: Some([10.0, 20.0]), lift_m: Some(0.0) },
        ]
    );
    // A derive.json written before the centre was recorded still reads.
    let old: Dropped = serde_json::from_str(r#"{ "name": "X", "stamp": 0 }"#).unwrap();
    assert_eq!((old.world_centre_m, old.lift_m), (None, None));
    assert!(file.appended.is_empty());
}

#[test]
fn appends_stamps_and_refuses_overlap() {
    let (g, o) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    guide(g.path());
    let add = o.path().join("add");
    StampSet { stamps: vec![stamp("new", 500.0, 7.0)] }.write(&add).unwrap();
    let out = o.path().join("derived");
    let mut a = args(g.path(), &out);
    a.add_stamps = vec![add.clone()];
    derive::run(&a).unwrap();
    let after = StampSet::read(&out).unwrap().unwrap();
    assert_eq!(after.stamps.len(), 5);
    assert_eq!(after.stamps[4], stamp("new", 500.0, 7.0));
    let file: DeriveFile = serde_json::from_slice(&std::fs::read(out.join("derive.json")).unwrap()).unwrap();
    assert_eq!(file.appended, [Appended { name: "new".into(), stamp: 4, from: add.display().to_string() }]);

    // On top of "Y" (stamp 3, kept at weight 0.5): refused, and the output folder is gone again.
    let bad = o.path().join("bad");
    StampSet { stamps: vec![stamp("on-y", 301.0, 7.0)] }.write(&bad).unwrap();
    let out2 = o.path().join("derived2");
    a.out.clone_from(&out2);
    a.add_stamps = vec![bad];
    let e = derive::run(&a).unwrap_err().to_string();
    assert!(e.contains("\"on-y\"") && e.contains("\"s3\""), "{e}");
    assert!(!out2.exists());
    // On top of "X" (stamp 0, dropped, weights now 0): accepted.
    let ok = o.path().join("ok");
    StampSet { stamps: vec![stamp("on-x", 1.0, 7.0)] }.write(&ok).unwrap();
    a.add_stamps = vec![ok];
    derive::run(&a).unwrap();
    assert_eq!(StampSet::read(&out2).unwrap().unwrap().stamps.len(), 5);
}

/// A 100 x 100 guide at 10 m (x 0..990, y 0..990), flat at 10 m with a wet channel at -5 m in columns 48..=52
/// (x 480..520, the whole height), and one 21 x 21 stamp "Bank" of weight 1 at 10 m on the east bank (x 540..560,
/// y 480..500); a site "Town" centred on the channel at (500, 500), 161 x 161 cells of 1 m (reach 80.5 m).
fn valley_fixture(guide: &Path, site: &Path, water_level_m: f64) {
    let n = 100;
    let heights: Vec<f32> = (0..n * n).map(|k| if (48..=52).contains(&(k % n)) { -5.0 } else { 10.0 }).collect();
    std::fs::write(guide.join("heights.f32"), heights.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>()).unwrap();
    let meta = r#"{ "format": 1, "dims": [100, 100], "sample_spacing_m": 10.0, "sea_level_m": 0.0, "origin_m": [0.0, 990.0] }"#;
    std::fs::write(guide.join("meta.json"), meta).unwrap();
    let bank =
        Stamp { name: "Bank".into(), origin_m: [540.0, 500.0], dims: [21, 21], spacing_m: 1.0, heights: vec![10.0; 441], weights: vec![1.0; 441] };
    StampSet { stamps: vec![bank] }.write(guide).unwrap();
    std::fs::create_dir_all(site).unwrap();
    let json = crate::site::SiteJson {
        format: 1,
        name: "Town".into(),
        producer: "test".into(),
        origin_m: [419.5, 580.5],
        cell_m: 1.0,
        dims: [161, 161],
        water_level_m,
        centre_m: [500.0, 500.0],
        source: "test".into(),
        claim: None,
        blake3: std::collections::BTreeMap::new(),
    };
    std::fs::write(site.join("site.json"), serde_json::to_vec_pretty(&json).unwrap()).unwrap();
}

#[test]
#[allow(clippy::cast_precision_loss, clippy::many_single_char_names)]
fn valley_stamp_is_clipped_off_existing_stamps_and_water() {
    let (g, o) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let site = o.path().join("site");
    valley_fixture(g.path(), &site, 0.0);
    let out = o.path().join("derived");
    let mut a = args(g.path(), &out);
    a.drop_anchors.clear();
    a.stamp_samples_max = 1_000_000;
    a.valley = Some(ValleyArgs { site: site.clone(), reach_m: Some(80.5), core_m: None });
    let summary = derive::run(&a).unwrap();

    let before = StampSet::read(g.path()).unwrap().unwrap();
    let after = StampSet::read(&out).unwrap().unwrap();
    assert_eq!(after.stamps.len(), 2, "{summary}");
    let (old, new) = (&before.stamps[0], &after.stamps[0]);
    assert_eq!((&old.name, old.origin_m, old.dims, old.spacing_m), (&new.name, new.origin_m, new.dims, new.spacing_m));
    let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(&old.heights), bits(&new.heights));
    assert_eq!(bits(&old.weights), bits(&new.weights));

    // The valley stamp: weight 0 on wet ground (the guide as the operator sampled it) and on the bank stamp.
    let v = &after.stamps[1];
    assert_eq!(v.name, "Town valley");
    let (tiles, _) = crate::site::guide_tiles(g.path(), None, 0.0).unwrap();
    let (mut wet, mut on_bank, mut lifted) = (0, 0, 0);
    for (k, &w) in v.weights.iter().enumerate() {
        let p = [v.origin_m[0] + (k % v.dims[0]) as f64, v.origin_m[1] - (k / v.dims[0]) as f64];
        if f64::from(tiles.height(p[0], -p[1], 0)) <= 0.0 {
            wet += 1;
            assert_eq!(w.to_bits(), 0, "wet sample at {p:?}");
        }
        if old.sample(p[0], p[1]).is_some_and(|(_, wb)| wb > 0.0) {
            on_bank += 1;
            assert_eq!(w.to_bits(), 0, "sample on the bank stamp at {p:?}");
        }
        lifted += usize::from(w > 0.0);
    }
    assert!(wet > 0 && on_bank == 441 && lifted > 0, "wet {wet}, on bank {on_bank}, weight > 0 {lifted}");

    let file: DeriveFile = serde_json::from_slice(&std::fs::read(out.join("derive.json")).unwrap()).unwrap();
    assert_eq!(file.appended, [Appended { name: "Town valley".into(), stamp: 1, from: site.display().to_string() }]);
    let rec = file.valley.unwrap();
    assert_eq!((rec.name.as_str(), rec.stamp, rec.dims), ("Town valley", 1, v.dims));
    assert_eq!((rec.reach_m, rec.core_m), (80.5, 300.0));
    // The bank stamp lies on touched ground: its 21 x 21 samples and the ring one sample around lose their weight.
    assert!(rec.clipped >= 441 && rec.clipped <= 23 * 23, "{rec:?}");
    assert!(rec.tapered > 0, "{rec:?}");
    let report: sf_worldgen::valley_terrace::ValleyReport =
        serde_json::from_slice(&std::fs::read(out.join("valley.json")).unwrap()).unwrap();
    assert_eq!((report.name.as_str(), report.centre_m, report.reach_m), ("Town valley", [500.0, 500.0], 80.5));

    // A site whose water level is not the guide's sea level is refused, and the output is removed.
    let site2 = o.path().join("site2");
    valley_fixture(g.path(), &site2, 1.0);
    let out2 = o.path().join("derived2");
    a.out.clone_from(&out2);
    a.valley = Some(ValleyArgs { site: site2, reach_m: Some(80.5), core_m: None });
    assert!(derive::run(&a).unwrap_err().to_string().contains("sea level"));
    assert!(!out2.exists());
}

#[test]
fn refuses_bad_folders_and_unknown_names() {
    let (g, o) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    guide(g.path());
    // --out = --guide, and --out inside the guide.
    assert!(derive::run(&args(g.path(), g.path())).is_err());
    assert!(derive::run(&args(g.path(), &g.path().join("sub"))).is_err());
    assert!(!g.path().join("sub").exists());
    // A non-empty --out is left alone.
    std::fs::write(o.path().join("keep.txt"), "x").unwrap();
    assert!(derive::run(&args(g.path(), o.path())).is_err());
    assert!(o.path().join("keep.txt").exists());
    // An unknown anchor name.
    let mut a = args(g.path(), &o.path().join("d"));
    a.drop_anchors = vec!["Nowhere".into()];
    assert!(derive::run(&a).unwrap_err().to_string().contains("Nowhere"));
}

/// ST-2b: `worldgen-site --guide` on a guide derived from [`valley_fixture`] with its anchor "Town" (the bank stamp,
/// mapped centre on the west bank at (440, 300)) dropped.
#[test]
#[allow(clippy::float_cmp, clippy::many_single_char_names)]
fn site_from_a_derived_guide_uses_the_dropped_anchor_and_the_valley_centre() {
    use crate::site::{self, SiteArgs, SiteParams, SiteTable, Square};
    let (g, o) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let site_dir = o.path().join("site");
    valley_fixture(g.path(), &site_dir, 0.0);
    let mut town = place("Town", Some(0));
    town.world_centre_m = [440.0, 300.0];
    town.lift_m = 2.5;
    AnchorTransform::new(10.0, vec![town, place("Other", None)]).write(g.path()).unwrap();
    let derive_with = |out: &Path, valley: bool| {
        let mut a = args(g.path(), out);
        a.drop_anchors = vec!["Town".into()];
        a.stamp_samples_max = 1_000_000;
        a.valley = valley.then(|| ValleyArgs { site: site_dir.clone(), reach_m: Some(80.5), core_m: None });
        derive::run(&a).unwrap();
    };
    let db = sf_records::RecordDb::new();
    let params = SiteParams { guide_radius_m: 100.0, cell_m: 5.0, river_search_m: 150.0, river_min_area_m2: 1000.0, ..SiteParams::default() };
    let site_args = |anchor: &str, out: &Path| SiteArgs {
        anchor: anchor.into(),
        out: out.to_owned(),
        params,
        table: SiteTable::builtin().unwrap(),
        sharp_scale: 0.0,
    };
    // The site's heights are the derived guide's tiles at the cell centres.
    let heights_ok = |guide: &Path, s: &site::Site, centre: [f64; 2]| {
        let (tiles, _) = site::guide_tiles(guide, None, 0.0).unwrap();
        let sq = Square::new(centre, params.guide_radius_m, params.cell_m);
        let n = sq.n();
        assert_eq!(s.json.dims, [n, n]);
        let want: Vec<u32> = (0..n * n)
            .map(|k| {
                let [x, y] = sq.at(k % n, k / n);
                tiles.height(x, -y, 0).to_bits()
            })
            .collect();
        assert_eq!(s.heights.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), want);
    };

    // With the valley: its centre, exactly, and no river search.
    let derived = o.path().join("derived");
    derive_with(&derived, true);
    let file: DeriveFile = serde_json::from_slice(&std::fs::read(derived.join("derive.json")).unwrap()).unwrap();
    assert_eq!(file.dropped, [Dropped { name: "Town".into(), stamp: Some(0), world_centre_m: Some([440.0, 300.0]), lift_m: Some(2.5) }]);
    let out = o.path().join("planned");
    let summary = site::run_guide(&db, &derived, &site_args("town", &out)).unwrap();
    let s = site::read(&out).unwrap();
    assert_eq!(s.json.centre_m, [500.0, 500.0], "{summary}");
    assert_eq!(s.json.name, "Town");
    assert!(s.json.source.contains("valley stamp \"Town valley\"") && s.json.source.contains("no river search"), "{}", s.json.source);
    assert!(!summary.contains("river:"), "{summary}");
    heights_ok(&derived, &s, [500.0, 500.0]);

    // Without a valley: the river reach nearest the dropped anchor's mapped centre (the channel's middle, x 500).
    let plain = o.path().join("plain");
    derive_with(&plain, false);
    let out2 = o.path().join("planned2");
    let summary = site::run_guide(&db, &plain, &site_args("Town", &out2)).unwrap();
    let s2 = site::read(&out2).unwrap();
    let (tiles, _) = site::guide_tiles(&plain, None, 0.0).unwrap();
    let river = site::find_river(&tiles, [440.0, 300.0], 150.0, 1000.0).unwrap();
    assert_eq!(s2.json.centre_m, river.centre_m, "{summary}");
    assert!((river.centre_m[0] - 500.0).abs() <= 10.0 && (river.centre_m[1] - 300.0).abs() <= 10.0, "{river:?}");
    assert!(s2.json.source.contains("river reach nearest Town") && s2.json.source.contains("dropped by worldgen-derive"));
    heights_ok(&plain, &s2, river.centre_m);

    // Neither in anchors.json nor dropped: refused.
    let e = site::run_guide(&db, &plain, &site_args("Nowhere", &o.path().join("none"))).unwrap_err().to_string();
    assert!(e.contains("Nowhere") && e.contains("derive.json"), "{e}");
    // A kept anchor still works as before (point anchor "Other", mapped centre (10, 20): no water there).
    assert!(site::run_guide(&db, &plain, &site_args("Other", &o.path().join("other"))).unwrap_err().to_string().contains("no water body"));
}
