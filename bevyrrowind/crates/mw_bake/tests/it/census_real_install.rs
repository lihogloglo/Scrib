//! Furniture and race census (`mw_bake::census`, motion-generation.md MG4)
//! on the real game data: sanity only, it finds seat, table and bed bands.
//! Skips (passes) without a Morrowind install. `-- --nocapture` prints the
//! census tables (measurement); `bevyrrowind furniture-census` writes them.

use mw_bake::census::{self, Kind};
use mw_install::MorrowindInstall;
use sf_vfs::Vfs;

#[test]
fn census_finds_seat_table_and_bed_bands() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let census = census::run(&db, &vfs);
    eprintln!("{census}");
    for kind in [Kind::Seat, Kind::Table, Kind::Bed] {
        let bands: Vec<_> = census.bands.iter().filter(|b| b.kind == kind).collect();
        assert!(!bands.is_empty(), "no {} band", kind.key());
        for b in bands {
            assert!(
                b.center.is_finite() && b.center > 0.0 && b.center <= kind.cap() * 2.0,
                "{b:?}"
            );
            assert!(b.refs > 0 || b.members > 0, "{b:?}");
        }
    }
    assert!(
        census.races.iter().any(|r| r.playable && r.male > 0.0),
        "playable races with heights"
    );
    let json: serde_json::Value = serde_json::from_str(&census.bands_json()).unwrap();
    assert!(json["points"]["seat"].is_array() && json["points"]["ground"].is_array());
}
