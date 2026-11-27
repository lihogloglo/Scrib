//! `mw.interact.defs` on synthetic triangle soups (boxes): exact facings, entrances, fallbacks,
//! floor offsets and counts, then the step through a baker with no files.

use std::collections::BTreeMap;
use std::sync::Arc;

use glam::Vec3;
use sf_bake::{Digest, Inputs, Store};
use mw_interaction::{Side, SlotDef, TagId};

use super::*;

const EPS: f32 = 1.0e-4;

type Soup = Vec<[Vec3; 3]>;

/// The 12 triangles of an axis-aligned box.
fn cube(lo: [f32; 3], hi: [f32; 3]) -> Soup {
    let corner = |ix: usize, iy: usize, iz: usize| Vec3::new([lo[0], hi[0]][ix], [lo[1], hi[1]][iy], [lo[2], hi[2]][iz]);
    let quads = [
        [corner(0, 0, 0), corner(1, 0, 0), corner(1, 1, 0), corner(0, 1, 0)],
        [corner(0, 0, 1), corner(1, 0, 1), corner(1, 1, 1), corner(0, 1, 1)],
        [corner(0, 0, 0), corner(0, 0, 1), corner(0, 1, 1), corner(0, 1, 0)],
        [corner(1, 0, 0), corner(1, 0, 1), corner(1, 1, 1), corner(1, 1, 0)],
        [corner(0, 0, 0), corner(1, 0, 0), corner(1, 0, 1), corner(0, 0, 1)],
        [corner(0, 1, 0), corner(1, 1, 0), corner(1, 1, 1), corner(0, 1, 1)],
    ];
    quads.iter().flat_map(|&[p0, p1, p2, p3]| [[p0, p1, p2], [p0, p2, p3]]).collect()
}

fn soup(boxes: &[([f32; 3], [f32; 3])]) -> Soup {
    boxes.iter().flat_map(|&(lo, hi)| cube(lo, hi)).collect()
}

/// A 0.5 m chair, seat top 0.45 m, backrest at -Z (to 1.0 m), arms on both sides (to 0.7 m).
fn chair() -> Soup {
    soup(&[
        ([-0.25, 0.0, -0.25], [0.25, 0.45, 0.25]),
        ([-0.25, 0.45, -0.25], [0.25, 1.0, -0.2]),
        ([-0.25, 0.45, -0.25], [-0.2, 0.7, 0.25]),
        ([0.2, 0.45, -0.25], [0.25, 0.7, 0.25]),
    ])
}

/// A 2 m bed along x, top 0.6 m, headboard at +X (to 1.2 m), footboard at -X (to 0.8 m).
fn bed() -> Soup {
    soup(&[
        ([-1.0, 0.0, -0.5], [1.0, 0.6, 0.5]),
        ([0.95, 0.6, -0.5], [1.0, 1.2, 0.5]),
        ([-1.0, 0.6, -0.5], [-0.95, 0.8, 0.5]),
    ])
}

fn record(id: &str, kind: Kind) -> FurnitureRecord {
    FurnitureRecord { id: id.to_owned(), record: "STAT", kind, model: format!("meshes/f/{id}.nif") }
}

/// Records whose models are `collision` soups (no render geometry).
fn bake(models: &[(&str, Kind, Soup)]) -> InteractDefs {
    let records: Vec<FurnitureRecord> = models.iter().map(|(id, kind, _)| record(id, *kind)).collect();
    let soups = models.iter().map(|(id, _, s)| (format!("meshes/f/{id}.nif"), Ok((s.clone(), Vec::new())))).collect();
    interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default())
}

fn faces(s: &SlotDef) -> Vec3 {
    s.local.rotation * Vec3::Z
}

#[test]
fn chair_with_a_backrest_faces_away_from_it() {
    let out = bake(&[("furn_chair", Kind::Seat, chair())]);
    let d = out.def(&RecordId::new("furn_chair")).expect("a def");
    assert_eq!(d.slots.len(), 1);
    assert!(faces(&d.slots[0]).abs_diff_eq(Vec3::Z, EPS));
    assert!(d.slots[0].local.translation.abs_diff_eq(Vec3::ZERO, EPS));
    assert!((d.slots[0].band - 0.45).abs() < EPS);
    let c = out.count(Kind::Seat);
    assert_eq!((c.records, c.defs, c.slots, c.upright, c.render, c.floor_offset), (1, 1, 1, 1, 0, 0));
}

#[test]
fn bed_with_a_headboard_lies_toward_it_entered_from_both_sides() {
    let out = bake(&[("furn_bed", Kind::Bed, bed())]);
    let d = out.def(&RecordId::new("furn_bed")).expect("a def");
    // Lie + two edge seats.
    assert_eq!(d.slots.len(), 3);
    let lie = &d.slots[0];
    assert!(faces(lie).abs_diff_eq(Vec3::X, EPS));
    assert!((lie.band - 0.6).abs() < EPS);
    let sides: Vec<Side> = lie.entrances.iter().map(|e| e.side).collect();
    assert_eq!(sides, [Side::Left, Side::Right]);
    // The lier's right = Y x X = -Z; approach 0.5 + 0.3 out.
    assert!(lie.entrances[0].approach.abs_diff_eq(Vec3::new(0.0, 0.0, 0.8), EPS));
    assert!(lie.entrances[1].approach.abs_diff_eq(Vec3::new(0.0, 0.0, -0.8), EPS));
    let c = out.count(Kind::Bed);
    assert_eq!((c.records, c.defs, c.slots, c.upright, c.long_end), (1, 1, 3, 1, 0));
}

#[test]
fn fallbacks_are_counted_per_rule() {
    let out = bake(&[
        // Stool: no backrest, square: +Z.
        ("furn_stool", Kind::Seat, soup(&[([-0.2, 0.0, -0.2], [0.2, 0.6, 0.2])])),
        // Backless bench along z: the + side of the short axis, +X.
        ("furn_bench", Kind::Seat, soup(&[([-0.2, 0.0, -0.8], [0.2, 0.45, 0.8])])),
        // Bedroll along z: no boards, the + end of the long axis.
        ("furn_bedroll", Kind::Bed, soup(&[([-0.3, 0.0, -0.9], [0.3, 0.22, 0.9])])),
        // Table and counter: the long-side rule.
        ("furn_table", Kind::Table, soup(&[([-0.6, 0.0, -0.4], [0.6, 0.84, 0.4])])),
        ("furn_bar", Kind::Counter, soup(&[([-1.0, 0.0, -0.3], [1.0, 0.94, 0.3])])),
    ]);
    let seat = out.count(Kind::Seat);
    assert_eq!((seat.records, seat.defs, seat.upright, seat.plus_z, seat.long_side), (2, 2, 0, 1, 1));
    let stool = out.def(&RecordId::new("furn_stool")).unwrap();
    assert!(faces(&stool.slots[0]).abs_diff_eq(Vec3::Z, EPS));
    let bench = out.def(&RecordId::new("furn_bench")).unwrap();
    assert!(bench.slots.iter().all(|s| faces(s).abs_diff_eq(Vec3::X, EPS)));
    let bedroll = out.def(&RecordId::new("furn_bedroll")).unwrap();
    assert!(faces(&bedroll.slots[0]).abs_diff_eq(Vec3::Z, EPS));
    assert_eq!(out.count(Kind::Bed).long_end, 1);
    let (table, bar) = (out.count(Kind::Table), out.count(Kind::Counter));
    assert_eq!((table.defs, table.long_side, bar.defs, bar.long_side), (1, 1, 1, 1));
    assert_eq!(out.defs.len(), 5);
    assert!(out.defs.windows(2).all(|w| w[0].id < w[1].id), "sorted by id");
}

#[test]
fn render_geometry_when_the_collision_has_no_top() {
    // Collided as one thin column: no usable top under the seat cap; the render chair has one.
    let column = soup(&[([-0.1, 0.0, -0.1], [0.1, 1.0, 0.1])]);
    let records = [record("furn_chair", Kind::Seat)];
    let soups = BTreeMap::from([("meshes/f/furn_chair.nif".to_owned(), Ok((column, chair())))]);
    let out = interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default());
    let c = out.count(Kind::Seat);
    assert_eq!((c.defs, c.render, c.upright), (1, 1, 1));
    let d = out.def(&RecordId::new("furn_chair")).unwrap();
    assert!(faces(&d.slots[0]).abs_diff_eq(Vec3::Z, EPS));
}

#[test]
fn floor_off_the_origin_moves_the_slots_to_it() {
    // A chair authored with its origin at seat height.
    let lifted: Soup = chair().iter().map(|t| t.map(|p| p - Vec3::Y * 0.45)).collect();
    let out = bake(&[("furn_chair", Kind::Seat, lifted), ("furn_ok", Kind::Seat, chair())]);
    let d = out.def(&RecordId::new("furn_chair")).unwrap();
    assert!(d.slots[0].local.translation.abs_diff_eq(Vec3::new(0.0, -0.45, 0.0), EPS));
    assert!((d.slots[0].entrances[0].approach.y + 0.45).abs() < EPS);
    assert!((d.slots[0].band - 0.45).abs() < EPS);
    assert_eq!(out.count(Kind::Seat).floor_offset, 1);
}

#[test]
fn shelves_missing_and_flat_models_have_no_def() {
    let records = [
        record("furn_shelf", Kind::Shelf),
        record("furn_gone", Kind::Seat),
        record("furn_broken", Kind::Table),
        record("furn_flat", Kind::Bed),
    ];
    let soups = BTreeMap::from([
        ("meshes/f/furn_shelf.nif".to_owned(), Ok((soup(&[([-0.5, 0.0, -0.2], [0.5, 1.5, 0.2])]), Vec::new()))),
        ("meshes/f/furn_broken.nif".to_owned(), Err("bad nif".to_owned())),
        // A floor plate: below the census's lowest surface.
        ("meshes/f/furn_flat.nif".to_owned(), Ok((soup(&[([-1.0, 0.0, -0.5], [1.0, 0.05, 0.5])]), Vec::new()))),
    ]);
    let out = interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default());
    assert!(out.defs.is_empty());
    let shelf = out.count(Kind::Shelf);
    assert_eq!((shelf.records, shelf.defs, shelf.long_side), (1, 0, 1));
    assert_eq!(out.count(Kind::Seat).unmeasured, 1);
    assert_eq!(out.count(Kind::Table).unmeasured, 1);
    assert_eq!(out.count(Kind::Bed).unmeasured, 1);
    assert_eq!(out.failed, [("meshes/f/furn_broken.nif".to_owned(), "bad nif".to_owned())]);
}

#[test]
fn sit_off_measures_but_gives_no_defs() {
    let records = [record("furn_chair", Kind::Seat)];
    let soups = BTreeMap::from([("meshes/f/furn_chair.nif".to_owned(), Ok((chair(), Vec::new())))]);
    let off = FurnitureParams { sit: false, ..FurnitureParams::default() };
    let out = interact_defs(&records, &soups, &off, &MeasureParams::default());
    let c = out.count(Kind::Seat);
    assert_eq!((c.records, c.defs, c.upright), (1, 0, 1));
}

struct NoInputs;

impl Inputs for NoInputs {
    fn file_fingerprint(&self, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Err(BakeError::Other(format!("no file {path}")))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// The step through a baker: models missing, every record counted unmeasured, product decodes,
/// and a second bake is a hit.
#[test]
fn step_bakes_and_decodes_without_files() {
    let dir = tempfile::tempdir().unwrap();
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(NoInputs));
    let records = vec![record("furn_chair", Kind::Seat), record("furn_bed", Kind::Bed)];
    baker.register(InteractDefsStep::new(records, FurnitureParams::default(), MeasureParams::default()));
    let key = ProductKey::new(INTERACT_DEFS, SUBJECT);
    let report = baker.bake(std::slice::from_ref(&key));
    assert_eq!(report.failures().count(), 0);
    let bytes = baker.store().get(&baker.product(&key).unwrap()).unwrap();
    let out = decode_interact_defs(&bytes).unwrap();
    assert!(out.defs.is_empty());
    assert_eq!((out.count(Kind::Seat).unmeasured, out.count(Kind::Bed).unmeasured), (1, 1));
    assert_eq!(out.failed.len(), 2);
    assert!(out.failed.iter().all(|(_, e)| e == "missing"));
    // A second session over the same store: a hit.
    let records = vec![record("furn_chair", Kind::Seat), record("furn_bed", Kind::Bed)];
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), Arc::new(NoInputs));
    baker.register(InteractDefsStep::new(records, FurnitureParams::default(), MeasureParams::default()));
    assert_eq!(baker.bake(std::slice::from_ref(&key)).failures().count(), 0);
    let s = baker.step_stats()[INTERACT_DEFS];
    assert_eq!((s.hits, s.built), (1, 0));
}

/// `IN32c0`: seat families from the measured backrest and band, and a def'd `CONT` / `ACTI` keeps its
/// class's primary verb (slotless, above the pose verbs); a `STAT` gets none.
#[test]
fn families_and_class_verbs() {
    let records = vec![
        FurnitureRecord { record: "CONT", ..record("furn_chest_seat", Kind::Seat) },
        record("furn_chair", Kind::Seat),
        FurnitureRecord { record: "ACTI", ..record("furn_cushion", Kind::Seat) },
        record("furn_stool", Kind::Seat),
    ];
    let model = |id: &str| format!("meshes/f/{id}.nif");
    let soups = [
        ("furn_chest_seat", soup(&[([-0.2, 0.0, -0.2], [0.2, 0.45, 0.2])])),
        ("furn_chair", chair()),
        // A floor cushion, 0.15 m high.
        ("furn_cushion", soup(&[([-0.25, 0.0, -0.25], [0.25, 0.15, 0.25])])),
        ("furn_stool", soup(&[([-0.2, 0.0, -0.2], [0.2, 0.6, 0.2])])),
    ]
    .into_iter()
    .map(|(id, s)| (model(id), Ok((s, Vec::new()))))
    .collect();
    let out = interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default());
    let def = |id: &str| out.def(&RecordId::new(id)).unwrap();
    let clips = |id: &str| def(id).slots.iter().map(|s| s.clips.0.clone()).collect::<Vec<_>>();
    assert_eq!(clips("furn_chair"), ["seat.chair"]);
    assert_eq!(clips("furn_stool"), ["seat.stool"]);
    assert_eq!(clips("furn_chest_seat"), ["seat.stool"]);
    assert_eq!(clips("furn_cushion"), ["seat.floor"]);
    let verbs = |id: &str| def(id).verbs.iter().map(|v| (v.verb, v.priority)).collect::<Vec<_>>();
    assert_eq!(verbs("furn_chest_seat"), [(TagId::of("open"), mw_interaction::CLASS_VERB_PRIORITY)]);
    assert_eq!(verbs("furn_cushion"), [(TagId::of("use"), mw_interaction::CLASS_VERB_PRIORITY)]);
    assert!(def("furn_chair").verbs.is_empty() && def("furn_stool").verbs.is_empty());
    assert_eq!((out.container_defs, out.activator_defs), (1, 1));
    let shown = InteractReport { defs: Some(out), ..InteractReport::default() }.to_string();
    assert!(shown.contains("CONT 1 defs, ACTI 1 defs"), "{shown}");
}

/// `IN3a`: a static (`STAT`) bed gets the slotless `sleep` at the class rank, above its pose verbs; a
/// scripted (`ACTI`) bed keeps `use` and no `sleep` (its script owns the sleep); a seat gets neither.
#[test]
fn static_beds_sleep_and_scripted_beds_keep_use() {
    let records = vec![
        record("furn_bed", Kind::Bed),
        FurnitureRecord { record: "ACTI", ..record("active_bed", Kind::Bed) },
        record("furn_chair", Kind::Seat),
    ];
    let soups = [("furn_bed", bed()), ("active_bed", bed()), ("furn_chair", chair())]
        .into_iter()
        .map(|(id, s)| (format!("meshes/f/{id}.nif"), Ok((s, Vec::new()))))
        .collect();
    let out = interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default());
    let verbs = |id: &str| out.def(&RecordId::new(id)).unwrap().verbs.iter().map(|v| (v.verb, v.priority)).collect::<Vec<_>>();
    let rank = mw_interaction::CLASS_VERB_PRIORITY;
    assert_eq!(verbs("furn_bed"), [(TagId::of("sleep"), rank)]);
    assert_eq!(verbs("active_bed"), [(TagId::of("use"), rank)]);
    assert!(verbs("furn_chair").is_empty());
    let bed = out.def(&RecordId::new("furn_bed")).unwrap();
    assert!(bed.slots.iter().all(|s| s.verb.priority < rank));
    assert_eq!(out.activator_defs, 1);
}
