//! S3b tests: a guide's own mods (`<guide>/mods/<name>/`) become `town:<group id>` extra entries (no game data).

use glam::DVec3;
use sf_records::{RecordDb, RecordId};
use sf_world::PlacementGroup;

use super::{GUIDE_MODS_DIR, guide_towns};

const POS: [[f64; 3]; 2] = [[-2_767.713_394_165_039, 26.627_262_115_478_516, 1_376.808_845_520_019_5], [10.0, 3.5, -20.0]];

/// A fresh guide folder under the system temp folder (removed on drop).
struct Guide(std::path::PathBuf);

impl Guide {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("bw-world-tests-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    /// `mods/<id>/` as `worldgen-town` writes it: `mod.toml` + `data/records/town.ron`, one exterior group `group`.
    fn with_town(self, id: &str, group: &str, worldspace: &str) -> Self {
        let dir = self.0.join(GUIDE_MODS_DIR).join(id);
        std::fs::create_dir_all(dir.join("data/records")).unwrap();
        std::fs::write(
            dir.join("mod.toml"),
            format!("id = \"{id}\"\nname = \"t\"\nversion = \"0.1.0\"\n\n[content]\nassets = [\"data\"]\nrecords = [\"records/town.ron\"]\n"),
        )
        .unwrap();
        let p = |i: usize| {
            format!(
                "(key: \"i{i}\", base: (kind: \"STAT\", id: \"ex_test_{i}\"), model: Some(\"meshes/x/ex_test_{i}.nif\"), \
                 transform: (pos: [{:?}, {:?}, {:?}], rotation: (0.0, 0.9829705, 0.0, 0.18376352)), class: Static)",
                POS[i][0], POS[i][1], POS[i][2]
            )
        };
        let ron = format!(
            "// test town\n[\n  define(kind: \"sf.placement_group\", id: \"{group}\", value: (\n    worldspace: \"{worldspace}\",\n    \
             placements: [\n      {},\n      {},\n    ],\n  )),\n]\n",
            p(0),
            p(1)
        );
        std::fs::write(dir.join("data/records/town.ron"), ron).unwrap();
        self
    }
}

impl Drop for Guide {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_guide_town_mod_becomes_one_extra_entry_at_its_stored_places() {
    let guide = Guide::new("town").with_town("t", "worldgen.town.t", "@exterior").with_town("u", "worldgen.town.cave", "somewhere");
    // The content as a normal run loads it: no guide mods, no placement group.
    let mut db = RecordDb::new();
    sf_world::register_kinds(&mut db).unwrap();
    assert_eq!(db.len::<PlacementGroup>(), 0, "a normal run never sees a guide's mods");

    let towns = guide_towns(&mut db, &guide.0).unwrap();
    assert_eq!(towns.len(), 1, "only exterior groups: {:?}", towns.iter().map(|t| &t.name).collect::<Vec<_>>());
    let t = &towns[0];
    assert_eq!(t.name, "town:worldgen.town.t");
    assert_eq!(t.placements.len(), 2);
    assert_eq!(t.unresolved, 2, "the fixture's bases are not in the (empty) content");
    let group = RecordId::new("worldgen.town.t");
    let stored = db.get::<PlacementGroup>(&group).unwrap();
    for (i, p) in t.placements.iter().enumerate() {
        assert_eq!(p.id, PlacementGroup::placement_id(&group, &format!("i{i}")));
        assert_eq!(p.transform, stored.placements[i].transform, "transforms as stored");
        let d = (p.transform.pos.to_f64() - DVec3::from_array(POS[i])).abs().max_element();
        assert!(d < 1e-3, "position {i} off by {d} m");
        assert_eq!(p.base.id.as_str(), format!("ex_test_{i}"));
    }
    assert_ne!(t.placements[0].id, t.placements[1].id);
    // Both mods applied under sources of their own.
    assert!(db.sources().iter().any(|s| s.name == "guide:t") && db.sources().iter().any(|s| s.name == "guide:u"));
}

#[test]
fn a_guide_without_mods_has_no_town() {
    let guide = Guide::new("bare");
    let mut db = RecordDb::new();
    let sources = db.sources().len();
    assert!(guide_towns(&mut db, &guide.0).unwrap().is_empty());
    assert_eq!(db.sources().len(), sources, "db untouched");
    // A `mods/` folder with no mod manifest in it: still nothing.
    std::fs::create_dir_all(guide.0.join(GUIDE_MODS_DIR).join("empty")).unwrap();
    assert!(guide_towns(&mut db, &guide.0).unwrap().is_empty());
    assert!(db.kind_names().is_empty(), "db untouched");
}
