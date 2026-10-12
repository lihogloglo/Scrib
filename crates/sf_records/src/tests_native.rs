//! Native record files, generic patches, field-level conflicts (mods.md
//! "Native record files", "Verification").

#![allow(clippy::float_cmp)] // Values round-trip exactly.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Flicker {
    Slow,
    Fast { rate: f32 },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Shape {
    Point,
    Spot(f32),
    Area { w: f32, h: f32 },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Light {
    name: String,
    radius: f32,
    #[serde(default)]
    color: (f32, f32, f32),
    #[serde(default)]
    flicker: Option<Flicker>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    stats: BTreeMap<String, i32>,
    shape: Shape,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Attrs {
    strength: u8,
    agility: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Npc {
    name: String,
    inventory: Vec<(String, u32)>,
    attrs: Attrs,
}

/// A kind registered without serde.
struct Opaque;

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register_serde::<Light>("item.light").unwrap();
    db.register_serde::<Npc>("npc").unwrap();
    db.register::<Opaque>("opaque").unwrap();
    db
}

const BASE: &str = r#"
// base content
[
  define(kind: "item.light", id: "lantern_01", value: (
    name: "Lantern", radius: 4.0, color: (1.0, 1.0, 1.0), flicker: Slow,
    tags: ["warm"], stats: { "str": 1, "int": 2 }, shape: Point,
  )),
  define(kind: "npc", id: "guard_01", value: (
    name: "Guard", inventory: [("sword", 1)], attrs: (strength: 50, agility: 40),
  )),
  define(kind: "item.light", id: "lantern_ugly", value: (name: "Ugly", radius: 1.0, shape: Spot(30.0))),
]
"#;

fn apply(db: &mut RecordDb, source: SourceId, text: &str) -> FileOutcome {
    apply_record_file(db, source, text, RecordFormat::Ron).unwrap()
}

fn light(db: &RecordDb, id: &str) -> Light {
    db.get::<Light>(&id.into()).unwrap().clone()
}

#[test]
fn define_reads_typed_values() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    let outcome = apply(&mut db, base, BASE);
    assert_eq!(outcome, FileOutcome { defined: 3, ..FileOutcome::default() });
    let l = light(&db, "LANTERN_01");
    assert_eq!(l.flicker, Some(Flicker::Slow), "implicit Some by default");
    assert_eq!(l.stats["int"], 2);
    assert_eq!(light(&db, "lantern_ugly").shape, Shape::Spot(30.0));
    assert_eq!(light(&db, "lantern_ugly").color, (0.0, 0.0, 0.0), "serde default");
}

#[test]
fn every_patch_op_on_nested_types() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let m = db.add_source("mod", &[base]);
    let outcome = apply(
        &mut db,
        m,
        r#"[
          patch(kind: "item.light", id: "lantern_01",
            set: { "radius": 5.0, "color": (0.9, 0.8, 0.6), "shape": Area(w: 1.0, h: 2.0), "stats.luck": 7,
                   "flicker": Fast(rate: 3.0) },
            remove: ["tags.0", "stats.str"]),
          patch(kind: "item.light", id: "lantern_ugly", set: { "shape": Spot(45.0) }),
          patch(kind: "npc", id: "guard_01",
            merge: { "attrs": (agility: 60) },
            append: { "inventory": [("lantern_01", 1)] }),
          patch(kind: "item.light", id: "lantern_01", set: { "flicker.rate": 4.0 }, remove: ["color"]),
          delete(kind: "item.light", id: "lantern_ugly"),
        ]"#,
    );
    assert_eq!(outcome.errors, []);
    assert_eq!((outcome.patched, outcome.deleted), (4, 1));

    let l = light(&db, "lantern_01");
    assert_eq!(l.radius, 5.0);
    assert_eq!(l.color, (0.0, 0.0, 0.0), "removed struct field resets to its default");
    assert_eq!(l.shape, Shape::Area { w: 1.0, h: 2.0 });
    assert_eq!(l.flicker, Some(Flicker::Fast { rate: 4.0 }), "paths reach through Some and enum payloads");
    assert!(l.tags.is_empty());
    assert_eq!(l.stats, BTreeMap::from([("int".to_owned(), 2), ("luck".to_owned(), 7)]));

    let npc = db.get::<Npc>(&"guard_01".into()).unwrap();
    assert_eq!(npc.attrs, Attrs { strength: 50, agility: 60 }, "merge keeps other fields");
    assert_eq!(npc.inventory, [("sword".to_owned(), 1), ("lantern_01".to_owned(), 1)]);
    assert!(db.get::<Light>(&"lantern_ugly".into()).is_none());

    // One touch per source, with every field it wrote.
    let history = db.history_of("item.light", &"lantern_01".into()).unwrap().unwrap();
    let fields: Vec<&str> = history[1].fields.as_ref().unwrap().iter().map(|f| &**f).collect();
    assert_eq!(fields, ["color", "flicker", "flicker.rate", "radius", "shape", "stats.luck", "stats.str", "tags.0"]);
    let history = db.history_of("npc", &"guard_01".into()).unwrap().unwrap();
    assert_eq!(history[1].fields.as_deref().unwrap(), [Box::from("attrs.agility"), Box::from("inventory")]);
}

#[test]
fn toml_files_define_patch_delete() {
    let mut db = db();
    let s = db.add_source("s", &[]);
    let text = r#"
[[define]]
kind = "item.light"
id = "blue"
value = { name = "Blue", radius = 6, shape = { Area = { w = 1.0, h = 2.0 } }, flicker = "Slow" }

[[define]]
kind = "item.light"
id = "gone"
value = { name = "Gone", radius = 1.0, shape = "Point" }

[[patch]]
kind = "item.light"
id = "blue"
set = { "stats.k" = 3, color = [0.5, 0.5, 0.5] }
append = { tags = ["cold"] }

[[delete]]
kind = "item.light"
id = "gone"
"#;
    let outcome = apply_record_file(&mut db, s, text, RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    let l = light(&db, "blue");
    assert_eq!(l.radius, 6.0, "integers read as floats");
    assert_eq!(l.shape, Shape::Area { w: 1.0, h: 2.0 });
    assert_eq!(l.flicker, Some(Flicker::Slow));
    assert_eq!((l.stats["k"], l.color, l.tags), (3, (0.5, 0.5, 0.5), vec!["cold".to_owned()]));
    assert!(db.get::<Light>(&"gone".into()).is_none());

    let err = apply_record_file(&mut db, s, "[[define]]\nkind = \"x\"\nid = 5\nvalue = 1\n", RecordFormat::Toml).unwrap_err();
    assert_eq!(err.line, 3, "{err}");
}

#[test]
fn errors_have_line_and_column() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);

    // Syntax errors reject the file.
    let err = apply_record_file(&mut db, base, "[\n  define(kind: \"npc\",, )\n]", RecordFormat::Ron).unwrap_err();
    assert_eq!(err.line, 2, "{err}");
    assert!(err.column > 1, "{err}");

    let text = [
        "[",                                                                         // 1
        "  define(kind: \"nope\", id: \"a\", value: ()),",                           // 2: unknown kind
        "  patch(kind: \"item.light\", id: \"missing\", set: { \"radius\": 1.0 }),", // 3: missing record
        "  patch(kind: \"item.light\", id: \"lantern_01\",",                         // 4
        "    set: { \"radius\": \"big\" }),",                                        // 5: wrong type, in the value
        "  patch(kind: \"item.light\", id: \"lantern_01\", set: { \"raduis\": 1.0 }),", // 6: unknown field
        "  patch(kind: \"item.light\", id: \"lantern_01\", sett: { }),",             // 7: unknown op field
        "  patch(kind: \"npc\", id: \"guard_01\", set: { \"inventory.5\": (\"x\", 1) }),", // 8: index out of range
        "  define(kind: \"opaque\", id: \"o\", value: ()),",                         // 9: not serde
        "  patch(kind: \"item.light\", id: \"lantern_01\", set: { \"radius\": 2.0 }),", // 10: fine
        "]",
    ]
    .join("\n");
    let outcome = apply(&mut db, base, &text);
    assert_eq!(outcome.patched, 1);
    let at: Vec<(u32, u32)> = outcome.errors.iter().map(|e| (e.line, e.column)).collect();
    let lines: Vec<u32> = at.iter().map(|p| p.0).collect();
    assert_eq!(lines, [2, 3, 5, 6, 7, 8, 9], "{:#?}", outcome.errors);
    assert_eq!(at[0].1, 3, "op position");
    assert!(at[2].1 >= 22, "inside the value: {:?}", at[2]);
    assert!(outcome.errors[0].message.contains("unknown record kind"), "{}", outcome.errors[0]);
    assert!(outcome.errors[1].message.contains("does not exist"), "{}", outcome.errors[1]);
    assert!(outcome.errors[3].message.contains("raduis"), "{}", outcome.errors[3]);
    assert!(outcome.errors[5].message.contains("out of range"), "{}", outcome.errors[5]);
    assert_eq!(light(&db, "lantern_01").radius, 2.0, "failed patches change nothing; later ones apply");
}

/// base defines; `a` and `b` and `c` depend only on base; `d` depends on `a`.
#[test]
fn field_conflicts_vs_merges() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let a = db.add_source("a", &[base]);
    let b = db.add_source("b", &[base]);
    let patch = |field: &str, value: &str| {
        format!("[patch(kind: \"item.light\", id: \"lantern_01\", set: {{ \"{field}\": {value} }})]")
    };
    apply(&mut db, a, &patch("radius", "6.0"));
    apply(&mut db, b, &patch("color", "(0.0, 0.0, 1.0)"));

    // Disjoint fields: both apply, reported as a merge.
    let l = light(&db, "lantern_01");
    assert_eq!((l.radius, l.color), (6.0, (0.0, 0.0, 1.0)));
    let report = db.conflict_report();
    assert_eq!((report.total_conflicts(), report.total_merges()), (0, 1));

    // Same field from an unrelated mod: a conflict on that field; last wins.
    let c = db.add_source("c", &[base]);
    apply(&mut db, c, &patch("radius", "8.0"));
    let report = db.conflict_report();
    let kind = report.kinds.iter().find(|k| k.kind == "item.light").unwrap();
    assert_eq!(kind.conflicts.len(), 1);
    assert_eq!(kind.conflicts[0].fields, [Box::from("radius")]);
    assert_eq!(light(&db, "lantern_01").radius, 8.0);
    let text = report.render(&db, 10);
    assert!(text.contains("a (patch radius) -> b (patch color) -> c (patch radius) [contested: radius]"), "{text}");

    // A nested path conflicts with its parent.
    assert!(paths_overlap("stats", "stats.luck") && !paths_overlap("stats", "statsx") && !paths_overlap("a.b", "a.c"));
}

#[test]
fn dependency_order_decides_the_winner() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let a = db.add_source("a", &[base]);
    let b = db.add_source("b", &[base, a]);
    let set = |r: f32| format!("[patch(kind: \"item.light\", id: \"lantern_01\", set: {{ \"radius\": {r:?} }})]");
    apply(&mut db, a, &set(6.0));
    apply(&mut db, b, &set(7.0));
    assert_eq!(light(&db, "lantern_01").radius, 7.0, "the dependent mod applies later and wins");
    let report = db.conflict_report();
    assert_eq!((report.total_conflicts(), report.total_merges()), (0, 0), "b saw a's change: an intended override");

    // A whole-record define after an unrelated field patch is a conflict on that field.
    let c = db.add_source("c", &[base]);
    apply(&mut db, c, "[define(kind: \"item.light\", id: \"lantern_01\", value: (name: \"New\", radius: 1.0, shape: Point))]");
    let kind = db.conflict_report().kinds.into_iter().find(|k| k.kind == "item.light").unwrap();
    assert_eq!(kind.conflicts[0].fields, [Box::from("radius")]);
}

#[test]
fn touches_collapse_and_closure_patches_touch_everything() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let a = db.add_source("a", &[base]);
    let b = db.add_source("b", &[base]);
    apply(&mut db, a, "[patch(kind: \"item.light\", id: \"lantern_01\", set: { \"radius\": 2.0 })]");
    db.patch::<Light>(b, &"lantern_01".into(), |l| l.name = "Renamed".into()).unwrap();
    let history = db.history_of("item.light", &"lantern_01".into()).unwrap().unwrap();
    assert_eq!(history[2].fields, None, "closure patches write the whole record");
    let kind = db.conflict_report().kinds.into_iter().find(|k| k.kind == "item.light").unwrap();
    assert_eq!(kind.conflicts.len(), 1, "unknown fields conflict with any patch");
}

#[test]
fn fingerprints_follow_values() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let id: RecordId = "lantern_01".into();
    let before = db.fingerprint("item.light", &id).unwrap().unwrap();
    let m = db.add_source("m", &[base]);
    apply(&mut db, m, "[patch(kind: \"item.light\", id: \"lantern_01\", set: { \"radius\": 9.0 })]");
    let after = db.fingerprint("item.light", &id).unwrap().unwrap();
    assert_ne!(before, after);
    apply(&mut db, m, "[patch(kind: \"item.light\", id: \"lantern_01\", set: { \"radius\": 4.0 })]");
    assert_eq!(db.fingerprint("item.light", &id).unwrap(), Some(before), "fingerprints hash values, not history");
    assert_eq!(db.fingerprint("item.light", &"nope".into()).unwrap(), None);
    assert!(matches!(db.fingerprint("opaque", &id), Err(RecordError::NoFingerprint(_))));
    db.set_fingerprint::<Opaque>(|_| [7; 32]).unwrap();
    assert!(db.fingerprint("opaque", &id).unwrap().is_none());
}

#[test]
fn value_round_trip_and_inspection() {
    let mut db = db();
    let base = db.add_source("base", &[]);
    apply(&mut db, base, BASE);
    let tree = db.value_of("item.light", &"lantern_01".into()).unwrap().unwrap();
    let back: Light = tree.deserialize_into().unwrap();
    assert_eq!(back, light(&db, "lantern_01"));
    assert!(patch::path_exists(&tree, "stats.int") && !patch::path_exists(&tree, "stats.nope"));
    assert_eq!(db.kind_names(), ["item.light", "npc", "opaque"]);
    assert!(db.is_serde_kind("npc") && !db.is_serde_kind("opaque"));
}
