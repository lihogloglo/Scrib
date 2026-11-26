//! Furniture interactable defs (`mw_bake::interact`, `mw.interact.defs`) on the real game data:
//! measurement, prints the def counts per kind (records, defs, slots, facing rule, render fallback,
//! floor offsets). Skips (passes) without a Morrowind install.

use std::collections::BTreeMap;

use mw_bake::census;
use mw_bake::interact::{self, InteractReport};
use mw_install::MorrowindInstall;
use mw_interaction::{FurnitureParams, Kind, MeasureParams};
use sf_vfs::{Vfs, VfsPath};

#[test]
#[ignore = "measurement: def counts per kind on the install"]
fn interact_defs_on_the_install() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let records = census::furniture_records(&db);
    let mut soups = BTreeMap::new();
    for r in &records {
        let path = r.model.to_ascii_lowercase();
        let s = VfsPath::new(&path)
            .map_err(|e| e.to_string())
            .and_then(|p| vfs.read(&p).map_err(|e| e.to_string()))
            .and_then(|bytes| mw_nif::load(&bytes).map_err(|e| e.to_string()))
            .map(|nif| census::model_soups(&nif));
        soups.insert(path, s);
    }
    let out = interact::interact_defs(&records, &soups, &FurnitureParams::default(), &MeasureParams::default());
    for (path, error) in &out.failed {
        eprintln!("  failed: {path}: {error}");
    }
    // The records off their floor, for the spec.
    for r in &records {
        if let Some(Ok(s)) = soups.get(&r.model.to_ascii_lowercase())
            && let Some((m, source)) = interact::measure_record(r.kind, s, &MeasureParams::default())
            && (m.floor * 100.0).round().abs() >= 1.0
        {
            eprintln!("  floor {:+.3} m: {} ({}, {source})", m.floor, r.id, r.kind.key());
        }
    }
    let defs = out.defs.len();
    eprintln!("{}", InteractReport { defs: Some(out.clone()), ..InteractReport::default() });
    for kind in [Kind::Seat, Kind::Table, Kind::Counter, Kind::Bed] {
        let c = out.count(kind);
        assert!(c.records > 0 && c.defs > 0, "{}: {c:?}", kind.key());
        assert!(c.slots >= c.defs, "{}: {c:?}", kind.key());
    }
    for d in &out.defs {
        for s in &d.slots {
            assert!(s.local.translation.is_finite() && s.band.is_finite() && s.band > 0.0, "{:?}", d.id);
        }
    }
    assert!(defs > 0);
}
