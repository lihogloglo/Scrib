//! Native record files through the pipeline: field patches from several mods,
//! conflicts vs merges in the report, positioned errors (mods.md, A2).

#![allow(clippy::float_cmp)] // Values round-trip exactly.

use std::path::Path;
use std::sync::Arc;

use semver::Version;
use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{MemSource, VfsSource};

use super::*;

const API: Version = Version::new(0, 1, 0);

fn id(s: &str) -> ModId {
    ModId::new(s).unwrap()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Light {
    name: String,
    radius: f32,
    #[serde(default)]
    color: (f32, f32, f32),
}

/// A content pack that registers `item.light` and ships a base mod whose
/// records are a native RON file in its archive.
struct Pack;

const BASE: &str = r#"[
  define(kind: "item.light", id: "lantern_01", value: (name: "Lantern", radius: 4.0)),
  define(kind: "item.light", id: "lantern_02", value: (name: "Lamp", radius: 2.0)),
]"#;

impl ContentAdapter for Pack {
    fn name(&self) -> &'static str {
        "pack"
    }
    fn discover(&self, _issues: &mut Vec<ModIssue>) -> Vec<ModManifest> {
        let mut base = ModManifest::synthesized(id("base"), "Base", Version::new(1, 0, 0));
        base.content.records = vec!["records/base.ron".into()];
        vec![base]
    }
    fn mount(&self, _manifest: &ModManifest) -> Result<Vec<Arc<dyn VfsSource>>, String> {
        Ok(vec![Arc::new(MemSource::new("base.archive").with("records/base.ron", BASE))])
    }
    fn register_kinds(&self, db: &mut RecordDb) {
        db.register_serde::<Light>("item.light").unwrap();
    }
}

fn write_mod(dir: &Path, mod_id: &str, file: &str, text: &str) {
    let root = dir.join(mod_id);
    std::fs::create_dir_all(root.join("data/records")).unwrap();
    let manifest = format!(
        "id = \"{mod_id}\"\nname = \"{mod_id}\"\nversion = \"1.0.0\"\ndescription = \"test\"\n\
         [dependencies]\nbase = \"*\"\n[content]\nassets = [\"data\"]\nrecords = [\"records/{file}\"]\n"
    );
    std::fs::write(root.join(MANIFEST_FILE), manifest).unwrap();
    std::fs::write(root.join("data/records").join(file), text).unwrap();
}

fn set(record: &str, field: &str, value: &str) -> String {
    format!("[patch(kind: \"item.light\", id: \"{record}\", set: {{ \"{field}\": {value} }})]")
}

#[test]
fn native_patches_merge_conflict_and_report_per_mod() {
    let tmp = tempfile::tempdir().unwrap();
    let mods = tmp.path().join("mods");
    let a = format!("{}\n{}", set("lantern_01", "radius", "6.0"), set("lantern_02", "radius", "3.0"));
    write_mod(&mods, "a", "a.ron", &a.replace("]\n[", ",\n"));
    write_mod(&mods, "b", "b.toml", "[[patch]]\nkind = \"item.light\"\nid = \"lantern_02\"\nset = { color = [0, 0, 1] }\n");
    write_mod(&mods, "c", "c.ron", &set("lantern_01", "radius", "8.0"));
    write_mod(
        &mods,
        "bad",
        "bad.ron",
        "[\n  patch(kind: \"item.light\", id: \"lantern_01\", set: { \"colour\": (1.0, 0.0, 0.0) }),\n  delete(kind: \"nope\", id: \"x\"),\n]",
    );
    write_mod(&mods, "broken", "broken.ron", "[\n  define(kind: \"item.light\" id: \"x\"),\n]");

    let pack = Pack;
    let adapters: [&dyn ContentAdapter; 2] = [&pack, &NativeRecords];
    let repo = ModRepository::scan(std::slice::from_ref(&mods), &adapters);
    let profile = Profile::new("p", ["base", "a", "b", "c", "bad", "broken"].map(id));
    let loaded = load_profile(&repo, &profile, &adapters, &API);

    let light = |r: &str| loaded.records.get::<Light>(&RecordId::new(r)).unwrap();
    assert_eq!(light("lantern_01").radius, 8.0, "c loads last and wins the contested field");
    assert_eq!((light("lantern_02").radius, light("lantern_02").color), (3.0, (0.0, 0.0, 1.0)), "disjoint fields merge");

    let issues = |m: &str| -> Vec<IssueKind> { loaded.report.issues_for(Some(&id(m))).map(|i| i.kind.clone()).collect() };
    let touches = |list: &[(&str, &str)]| -> Vec<(String, String)> {
        list.iter().map(|(s, w)| ((*s).to_owned(), (*w).to_owned())).collect()
    };
    assert_eq!(
        issues("c"),
        [IssueKind::RecordConflict {
            kind: "item.light".into(),
            id: "lantern_01".into(),
            fields: vec!["radius".into()],
            touches: touches(&[("base", "define"), ("a", "patch radius"), ("c", "patch radius")]),
        }]
    );
    assert_eq!(
        issues("b"),
        [IssueKind::RecordMerge {
            kind: "item.light".into(),
            id: "lantern_02".into(),
            touches: touches(&[("base", "define"), ("a", "patch radius"), ("b", "patch color")]),
        }]
    );
    assert!(issues("a").is_empty(), "{:?}", issues("a"));

    // Each failing operation is an error at its position; the file's other ops apply.
    let bad = issues("bad");
    let at: Vec<(Option<u32>, Option<u32>)> = bad
        .iter()
        .map(|k| match k {
            IssueKind::RecordFile { line, column, .. } => (*line, *column),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(at, [(Some(2), Some(64)), (Some(3), Some(3))], "{bad:?}");
    // A syntax error rejects the file at its position.
    assert!(matches!(&issues("broken")[..], [IssueKind::RecordFile { line: Some(2), .. }]), "{:?}", issues("broken"));

    let text = loaded.render_report(None, 5);
    assert!(text.contains("warning [c] record conflict item.light \"lantern_01\" on radius: base (define) -> a (patch radius) -> c (patch radius)"), "{text}");
    assert!(text.contains("info [b] record merge item.light \"lantern_02\""), "{text}");
    assert!(text.contains("record file records/bad.ron:2:64: item.light record \"lantern_01\": field path \"colour\""), "{text}");
}
