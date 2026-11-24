//! S3a tests: `worldgen-town` on a fixture (no game data): three instances (one of an unknown kit id), a two-model
//! kit, read back through the native record loader.

// Exact results are the point of these tests.
#![allow(clippy::float_cmp)]

use glam::{DVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_world::{PlacementClass, PlacementFlags, PlacementGroup};

use crate::kit::{self, KitJson, KitKind, KitModel};
use crate::town::{self, PlacementsFile, TownArgs};
use mw_world::source::Base;

const HOUSE: &str = "meshes/x/ex_test_b_01.nif";
const TOWER: &str = "meshes/x/ex_test_tower.nif";
const BARE: &str = "meshes/x/ex_test_bare.nif";

fn model(id: &str, tags: &[&str]) -> KitModel {
    KitModel {
        id: id.into(),
        kind: KitKind::Building,
        tags: tags.iter().map(|t| (*t).to_owned()).collect(),
        roles: Vec::new(),
        footprint: vec![[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]],
        obb: None,
        doors: Vec::new(),
        height_m: 3.0,
        floors: None,
        base_offset_m: 0.0,
        ground_z_m: None,
        foundation_depth_m: None,
        yaw_step_deg: 0.0,
        weight: 1.0,
        blake3: "x".into(),
    }
}

fn kit_json() -> KitJson {
    KitJson {
        format: kit::FORMAT,
        name: "test".into(),
        producer: "test".into(),
        units: "m".into(),
        frame: kit::FRAME.into(),
        models: vec![
            model(HOUSE, &["base:ex_test_b_01", "base:ex_test_b_01_alt"]),
            model(TOWER, &["base:ex_test_tower"]),
            model(BARE, &[]),
        ],
    }
}

fn resolve(id: &str) -> Option<Base> {
    match id {
        "ex_test_b_01" => Some(Base { tag: "STAT", class: PlacementClass::Static, model: Some(HOUSE.into()) }),
        "ex_test_tower" => Some(Base { tag: "ACTI", class: PlacementClass::Custom(2), model: Some(TOWER.into()) }),
        _ => None,
    }
}

const POS: [[f64; 3]; 3] = [
    [-2_767.713_395_627_877_6, -1_376.808_848_756_661, 26.627_262_832_641_6],
    [10.0, 20.0, 3.5],
    [-3_125.828_381_736_532_7, -1_543.953_628_264_560_2, -0.25],
];
const YAW: [f64; 3] = [158.821_895_593_837_95, 45.0, 90.0];

fn placements_json(kit_blake3: &str) -> String {
    let inst = |i: usize, kit: &str| {
        format!(
            r#"{{"id": "i{i}", "kit": "{kit}", "lot": null, "pos": [{:?}, {:?}, {:?}], "yaw_deg": {:?}, "role": "building", "door": null}}"#,
            POS[i][0], POS[i][1], POS[i][2], YAW[i]
        )
    };
    format!(
        r#"{{"format": 1, "name": "testtown", "producer": "settle 0.1.0", "recipe": "r", "seed": "s",
  "kit_blake3": "{kit_blake3}", "origin_m": [0.0, 0.0], "lots": [], "instances": [{}, {}, {}],
  "streets": [], "features": []}}"#,
        inst(0, HOUSE),
        inst(1, "meshes/x/not_in_kit.nif"),
        inst(2, TOWER)
    )
}

/// The flora conversion (`flora::placements`) of an upright item at guide `pos`, yaw `yaw_deg`.
fn flora_transform(pos: [f64; 3], yaw_deg: f64) -> WorldTransform {
    let up = [0.0f32, 0.0, 1.0];
    let up = Vec3::new(up[0], up[2], -up[1]);
    #[allow(clippy::cast_possible_truncation)]
    let yaw = yaw_deg.to_radians() as f32;
    WorldTransform {
        pos: WorldPos::from_f64(DVec3::new(pos[0], pos[2], -pos[1])),
        rotation: Quat::from_rotation_arc(Vec3::Y, up) * Quat::from_rotation_y(yaw),
        scale: Vec3::splat(1.0),
    }
}

/// Writes the kit and placements into a fresh folder; returns (dir, args for `out`).
fn fixture(out: &str) -> (tempfile::TempDir, TownArgs) {
    let tmp = tempfile::tempdir().unwrap();
    let kit_dir = tmp.path().join("kit");
    kit::write(&kit_json(), &kit_dir).unwrap();
    let hash = kit::hash(&std::fs::read(kit_dir.join(kit::KIT_FILE)).unwrap());
    let placements = tmp.path().join("placements.json");
    std::fs::write(&placements, placements_json(&hash)).unwrap();
    let args =
        TownArgs { placements, kit: kit_dir, out: tmp.path().join(out), id: None, streets: None, gardens: None, guide: None };
    (tmp, args)
}

#[test]
fn town_mod_reads_back_as_the_flora_conversion() {
    let (_tmp, args) = fixture("town-a");
    let report = town::run(&args, resolve).unwrap();
    assert!(report.contains("2 placements of 3 instances; skipped 1 unknown kit id, 0 kit model without base, 0 unresolved base; 1 models with several bases"), "{report}");
    assert!(report.contains("skipped kit id: meshes/x/not_in_kit.nif"), "{report}");

    let text = std::fs::read_to_string(args.out.join("data").join(town::RECORDS_PATH)).unwrap();
    let mut db = RecordDb::new();
    db.register_serde::<PlacementGroup>(PlacementGroup::KIND).unwrap();
    let source = db.add_source("town-a", &[]);
    let outcome = apply_record_file(&mut db, source, &text, RecordFormat::Ron).unwrap();
    assert!(outcome.errors.is_empty(), "{:?}\n{text}", outcome.errors);
    assert_eq!(outcome.defined, 1);
    let group = db.get::<PlacementGroup>(&town::group_id("testtown")).expect("the group is defined");
    assert_eq!(group.worldspace, mw_world::ids::exterior());
    assert_eq!(group.placements.len(), 2);

    let (a, b) = (&group.placements[0], &group.placements[1]);
    assert_eq!((a.key.as_str(), b.key.as_str()), ("i0", "i2"));
    assert_eq!((&*a.base.kind, a.base.id.as_str()), ("STAT", "ex_test_b_01"), "the first base tag");
    assert_eq!((&*b.base.kind, b.base.id.as_str()), ("ACTI", "ex_test_tower"));
    assert_eq!(a.model.as_ref().map(sf_vfs::VfsPath::as_str), Some(HOUSE));
    assert_eq!((a.class, b.class), (PlacementClass::Static, PlacementClass::Custom(2)));
    assert_eq!((a.flags, b.flags), (PlacementFlags::empty(), PlacementFlags::empty()));
    // The flora conversion as any native record carries it (`WorldTransform`'s serde form normalizes the rotation
    // on read: up to an ulp from the written one).
    let loaded = |t: WorldTransform| WorldTransform { rotation: t.rotation.normalize(), ..t };
    assert_eq!(a.transform, loaded(flora_transform(POS[0], YAW[0])));
    assert_eq!(b.transform, loaded(flora_transform(POS[2], YAW[2])));
    // Sanity on the frame: guide +y (north) is engine -z, guide z is engine y (`WorldPos` keeps f32 in its cell).
    let p = b.transform.pos.to_f64();
    assert!((p - DVec3::new(POS[2][0], POS[2][2], -POS[2][1])).abs().max_element() < 1e-3, "{p}");

    // The manifest names the record file under its asset root.
    let manifest: toml::Table = std::fs::read_to_string(args.out.join("mod.toml")).unwrap().parse().unwrap();
    assert_eq!(manifest["id"].as_str(), Some("town-a"));
    assert_eq!(manifest["content"]["assets"].as_array().unwrap()[0].as_str(), Some(town::ASSET_ROOT));
    assert_eq!(manifest["content"]["records"].as_array().unwrap()[0].as_str(), Some(town::RECORDS_PATH));
    assert!(town::valid_mod_id("town-a"));

    // Same inputs, same bytes.
    let (_tmp2, args2) = fixture("town-a");
    town::run(&args2, resolve).unwrap();
    assert_eq!(std::fs::read(args2.out.join("data").join(town::RECORDS_PATH)).unwrap(), text.into_bytes());
    assert_eq!(std::fs::read(args2.out.join("mod.toml")).unwrap(), std::fs::read(args.out.join("mod.toml")).unwrap());
}

#[test]
fn town_skips_bare_models_and_refuses_repeated_ids_and_other_kits() {
    let kit = kit_json();
    let mut file = PlacementsFile::parse(placements_json("h").as_bytes()).unwrap();
    file.instances[1].kit = BARE.into();
    let t = town::build(&file, &kit, resolve).unwrap();
    assert_eq!((t.counts.placed, t.counts.no_base, t.counts.unknown_kit), (2, 1, 0));
    file.instances[2].kit = "meshes/x/ex_test_b_02.nif".into();
    file.instances[1].kit = HOUSE.into();
    let mut kit2 = kit.clone();
    kit2.models.push(model("meshes/x/ex_test_b_02.nif", &["base:gone"]));
    let t = town::build(&file, &kit2, resolve).unwrap();
    assert_eq!((t.counts.placed, t.counts.unresolved), (2, 1));
    file.instances[2].id = "i0".into();
    assert!(town::build(&file, &kit2, resolve).is_err(), "a repeated instance id");

    let (_tmp, args) = fixture("town-b");
    std::fs::write(&args.placements, placements_json("0000")).unwrap();
    assert!(town::run(&args, resolve).is_err(), "kit blake3 mismatch");
    let (_tmp2, args2) = fixture("Bad Id");
    assert!(town::run(&args2, resolve).is_err(), "invalid mod id");
}

/// A two-segment street network (road network v1, guide frame), one bridge crossing.
const STREETS: &str = r#"{
  "format": 1,
  "segments": [
    { "id": 0, "class": "street", "surface": "flagstone", "width_m": 4.0, "a": 0, "b": 1,
      "points": [[10, 20], [40, 22]], "crossings": [] },
    { "id": 1, "class": "street", "surface": "dirt", "width_m": 3.0, "a": 1, "b": 2,
      "points": [[40, 22], [70, 50]], "crossings": [] }
  ],
  "junctions": [{ "id": 0, "pos": [10, 20] }, { "id": 1, "pos": [40, 22] }, { "id": 2, "pos": [70, 50] }]
}"#;

#[test]
fn town_copies_valid_streets_and_rejects_invalid_ones() {
    let (tmp, mut args) = fixture("town-s");
    let streets = tmp.path().join("streets.json");
    std::fs::write(&streets, STREETS).unwrap();
    args.streets = Some(streets);
    let report = town::run(&args, resolve).unwrap();
    assert!(report.contains("streets: 2 segments"), "{report}");
    let copy = args.out.join("data").join(town::ROADS_DIR).join("town-s.json");
    assert_eq!(copy, args.out.join(town::streets_path("town-s")));
    assert_eq!(std::fs::read(&copy).unwrap(), STREETS.as_bytes(), "copied byte for byte");

    // Without `--streets`, a stale copy goes.
    args.streets = None;
    town::run(&args, resolve).unwrap();
    assert!(!copy.exists(), "the stale streets file is removed");

    // An invalid file (wrong format version, then not JSON) fails before anything is written.
    let (tmp2, mut args2) = fixture("town-t");
    let bad = tmp2.path().join("streets.json");
    for text in [STREETS.replace("\"format\": 1", "\"format\": 9"), "not json".to_owned()] {
        std::fs::write(&bad, text).unwrap();
        args2.streets = Some(bad.clone());
        let err = town::run(&args2, resolve).unwrap_err().to_string();
        assert!(err.contains("streets.json"), "the error names the file: {err}");
        assert!(!args2.out.exists(), "nothing written");
    }
}

// G1b garden planting (temporary scaffold, `garden.rs`): on the shipped table.

const GARDENS: &str = include_str!("../../../worldgen/town-gardens.toml");

fn garden_table() -> crate::garden::GardenTable {
    crate::garden::GardenTable::from_toml(GARDENS).unwrap()
}

/// Every `flora_` id resolves (a static with its own model); the rest as [`resolve`].
fn resolve_flora(id: &str) -> Option<Base> {
    if id.starts_with("flora_") {
        return Some(Base { tag: "STAT", class: PlacementClass::Static, model: Some(format!("meshes/f/{id}.nif")) });
    }
    resolve(id)
}

fn garden(source: &str, lot: &str, poly: &[[f64; 2]]) -> serde_json::Value {
    serde_json::json!({ "kind": "garden", "source": source, "lot": lot, "polygon": poly })
}

fn square(x0: f64, y0: f64, w: f64, h: f64) -> Vec<[f64; 2]> {
    vec![[x0, y0], [x0 + w, y0], [x0 + w, y0 + h], [x0, y0 + h]]
}

/// Guide ground of the tests: a gentle slope.
fn ground(x: f64, y: f64) -> f64 {
    7.0 + 0.01 * x - 0.02 * y
}

#[test]
fn garden_square_gets_spaced_trees_inside_the_inset_and_is_deterministic() {
    use crate::garden::{self, Class};
    let table = garden_table();
    let p = &table.params;
    let poly = square(-2400.0, -1850.0, 40.0, 30.0);
    let features = vec![garden("empty_plot", "lot-a", &poly)];
    let (plants, counts) = garden::plant(&table, &features, resolve_flora, &ground).unwrap();
    let c = counts.by_source["empty_plot"];
    assert!(c.trees > 0 && c.shrubs > 0, "{c:?}");
    assert!(counts.unresolved.is_empty());
    assert_eq!(plants.len(), c.trees + c.shrubs);

    // The scatter itself: trees at least the spacing apart, everything inside the inset polygon.
    let usable = vec![true; table.species.len()];
    let seed = 42;
    let s = garden::scatter(&table, &poly, seed, &usable);
    let trees: Vec<[f64; 2]> = s.iter().filter(|q| q.class == Class::Tree).map(|q| q.pos).collect();
    assert!(!trees.is_empty());
    for (i, a) in trees.iter().enumerate() {
        for b in &trees[i + 1..] {
            assert!((a[0] - b[0]).hypot(a[1] - b[1]) >= p.tree_spacing_m.v, "{a:?} {b:?}");
        }
        assert!(garden::inside_inset(&poly, *a, p.edge_inset_m.v + p.tree_clearance_m.v), "{a:?}");
    }
    for q in &s {
        assert!(garden::inside_inset(&poly, q.pos, p.edge_inset_m.v), "{q:?}");
        assert_eq!(table.species[q.species].class, q.class);
    }
    assert_eq!(s, garden::scatter(&table, &poly, seed, &usable), "same seed, same plants");
    assert_ne!(s, garden::scatter(&table, &poly, seed + 1, &usable), "another seed, other plants");

    // Placements: stable unique keys, GROUNDED, unit scale, the ground's height; the same features give the same.
    let mut keys = std::collections::BTreeSet::new();
    for (n, pl) in plants.iter().enumerate() {
        assert_eq!(pl.key, format!("garden:lot-a:{n}"));
        assert!(keys.insert(pl.key.clone()));
        assert_eq!(pl.flags, PlacementFlags::GROUNDED);
        assert_eq!(pl.transform.scale, Vec3::ONE);
        let w = pl.transform.pos.to_f64();
        assert!((w.y - ground(w.x, -w.z)).abs() < 1e-3, "{w}");
        assert!(pl.base.id.as_str().starts_with("flora_"));
    }
    assert_eq!(garden::plant(&table, &features, resolve_flora, &ground).unwrap().0, plants);
}

#[test]
fn garden_sliver_gets_only_shrubs_or_nothing() {
    use crate::garden::{self, Class};
    let table = garden_table();
    let sliver = square(100.0, 200.0, 40.0, 3.0);
    let usable = vec![true; table.species.len()];
    for seed in 0..8 {
        let s = garden::scatter(&table, &sliver, seed, &usable);
        assert!(s.iter().all(|q| q.class == Class::Shrub), "seed {seed}: {s:?}");
    }
    let (plants, counts) = garden::plant(&table, &[garden("sliver", "lot-s", &sliver)], resolve_flora, &ground).unwrap();
    assert_eq!(counts.by_source["sliver"].trees, 0);
    assert_eq!(plants.len(), counts.by_source["sliver"].shrubs);
}

#[test]
fn garden_ignores_other_feature_kinds_and_drops_unresolved_species() {
    use crate::garden;
    let table = garden_table();
    let features = vec![
        serde_json::json!({ "kind": "fountain", "pos": [1.0, 2.0] }),
        serde_json::json!({ "kind": "orchard", "lot": "x" }),
        garden("mu_garden", "lot-m", &square(0.0, 0.0, 30.0, 30.0)),
    ];
    let (plants, counts) = garden::plant(&table, &features, resolve_flora, &ground).unwrap();
    assert_eq!(counts.other_kinds, 2);
    assert_eq!(counts.by_source.len(), 1);
    assert!(!plants.is_empty());
    let report = garden::report_line(&counts);
    assert!(report.contains("mu_garden 1 features") && report.contains("2 other features ignored"), "{report}");

    // Nothing resolves: no plants, every species listed.
    let (none, counts) = garden::plant(&table, &features, |_| None, &ground).unwrap();
    assert!(none.is_empty());
    assert_eq!(counts.unresolved.len(), table.species.len());
    // A garden without its polygon is a broken contract.
    let bad = vec![serde_json::json!({ "kind": "garden", "source": "sliver", "lot": "x" })];
    assert!(garden::plant(&table, &bad, resolve_flora, &ground).is_err());
}

#[test]
fn town_without_gardens_is_byte_identical_and_gardens_read_back_grounded() {
    let (tmp, args) = fixture("town-g");
    town::run(&args, resolve).unwrap();
    let records = args.out.join("data").join(town::RECORDS_PATH);
    let before = (std::fs::read(&records).unwrap(), std::fs::read(args.out.join("mod.toml")).unwrap());

    // The same placements with garden features, no `--gardens`: the same bytes.
    let text = std::fs::read_to_string(&args.placements).unwrap();
    let features = serde_json::to_string(&vec![garden("empty_plot", "lot-a", &square(0.0, 0.0, 40.0, 40.0))]).unwrap();
    std::fs::write(&args.placements, text.replace(r#""features": []"#, &format!(r#""features": {features}"#))).unwrap();
    town::run(&args, resolve).unwrap();
    assert_eq!((std::fs::read(&records).unwrap(), std::fs::read(args.out.join("mod.toml")).unwrap()), before);

    // `--gardens` without `--guide` is refused.
    let toml = tmp.path().join("town-gardens.toml");
    std::fs::write(&toml, GARDENS).unwrap();
    let with = TownArgs { gardens: Some(toml), ..args.clone() };
    assert!(town::run(&with, resolve).unwrap_err().to_string().contains("--guide"));

    // With gardens (as `run` adds them): the plants follow the instances and read back GROUNDED.
    let file = PlacementsFile::parse(&std::fs::read(&args.placements).unwrap()).unwrap();
    let mut t = town::build(&file, &kit_json(), resolve_flora).unwrap();
    town::add_gardens(&mut t, &file, &garden_table(), resolve_flora, &ground).unwrap();
    let n = t.group.placements.len();
    assert!(n > 2);
    let ron = town::to_ron(&t, &file);
    let mut db = RecordDb::new();
    db.register_serde::<PlacementGroup>(PlacementGroup::KIND).unwrap();
    let source = db.add_source("town-g", &[]);
    let outcome = apply_record_file(&mut db, source, &ron, RecordFormat::Ron).unwrap();
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    let group = db.get::<PlacementGroup>(&town::group_id("testtown")).unwrap();
    assert_eq!(group.placements.len(), n);
    assert_eq!(group.placements[0].flags, PlacementFlags::empty(), "kit instances stay ungrounded");
    assert!(group.placements[2..].iter().all(|p| p.flags == PlacementFlags::GROUNDED && p.key.starts_with("garden:lot-a:")));
}
