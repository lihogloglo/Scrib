//! The world adapter on the real game data. Skips (passes) when no
//! Morrowind install is detected. Full numbers:
//! `cargo run --release -p mw_world --example world_report`.

use mw_install::MorrowindInstall;
use mw_world::{MwTerrainSource, MwWorldSource, SourceOptions, WorldRecords, ids, terrain};
use sf_terrain::TerrainSource;
use sf_vfs::Vfs;
use sf_world::{IndexParams, PlacementFlags, build_index};

#[test]
fn morrowind_world_adapts() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;

    let records = WorldRecords::build(db);
    assert!(records.worldspaces.len() > 600, "one worldspace per interior");
    assert!(records.doors.paired > 1000, "{:?}", records.doors.paired);
    assert!(records.doors.one_way * 20 < records.doors.paired, "most doors pair up");
    // The census office's front door and the exterior door into it pair up.
    let census = ids::interior("Seyda Neen, Census and Excise Office");
    assert!(
        records.door_links.iter().any(|(_, l)| l.two_way
            && [&l.a.worldspace, &l.b.worldspace].contains(&&census)
            && [&l.a.worldspace, &l.b.worldspace].contains(&&ids::exterior())),
        "census office door link"
    );

    let source = MwWorldSource::build(db, &SourceOptions::default());
    let stats = source.stats();
    assert!(stats.placements > 100_000, "{stats:?}");
    assert!(stats.grounded > 10_000 && stats.editor_only > 0);
    let index = build_index(&ids::exterior(), &IndexParams::default(), &source, &|_| None);
    assert!(index.stats.duplicate_ids.is_empty(), "{:?}", index.stats.duplicate_ids.len());
    assert!(index.stats.cells > 500);

    // Seyda Neen's land reads, and its grounded statics sit on it.
    let land = MwTerrainSource::new(db);
    assert_eq!(land.stats().unresolved_textures, 0);
    let tile = land.read(&ids::exterior(), terrain::tile_id((-2, -9))).expect("Seyda Neen has land");
    assert_eq!(tile.heights.len(), 65 * 65);
    let grounded = source.all_placements(&ids::exterior()).filter(|p| p.flags.contains(PlacementFlags::GROUNDED)).count();
    assert_eq!(grounded, stats.grounded, "only exteriors ground");
}

/// Every interior indexes as one `Single` cell, and every door link end
/// leads to a worldspace with a record (unresolved: 0).
#[test]
fn every_interior_indexes_and_every_door_resolves() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;
    let records = WorldRecords::build(db);
    let known: std::collections::HashSet<_> = records.worldspaces.iter().map(|(id, _)| id).collect();
    let mut unresolved = 0;
    for (_, link) in &records.door_links {
        let ends = if link.two_way { vec![&link.a, &link.b] } else { vec![&link.b] };
        unresolved += ends.iter().filter(|e| !known.contains(&e.worldspace)).count();
    }
    assert_eq!(unresolved, 0, "door link ends without a worldspace");

    let source = MwWorldSource::build(db, &SourceOptions::default());
    let mut indexed = 0;
    for (id, ws) in records.worldspaces.iter().filter(|(id, _)| *id != ids::exterior()) {
        assert_eq!(ws.partition, sf_world::Partition::Single, "{id:?}");
        let params = IndexParams { partition: ws.partition, ..IndexParams::default() };
        let index = build_index(id, &params, &source, &|_| None);
        assert!(index.stats.cells <= 1, "{id:?} loads whole: {} cells", index.stats.cells);
        indexed += usize::from(index.stats.cells == 1);
    }
    eprintln!("interiors with placements indexed: {indexed} of {}", records.worldspaces.len() - 1);
    assert!(indexed > 600);

    // The test scene `door-balmora-guild` stands in front of this door.
    let guild = ids::interior("Balmora, Guild of Mages");
    for (_, l) in records.door_links.iter().filter(|(_, l)| l.a.worldspace == guild || l.b.worldspace == guild) {
        eprintln!(
            "guild link: {} at {:.2?} (arrival {:.2?}) <-> {} at {:.2?}",
            l.a.worldspace.as_str(),
            l.a.frame.pos.to_f64(),
            l.a.arrival.pos.to_f64(),
            l.b.worldspace.as_str(),
            l.b.frame.pos.to_f64()
        );
    }
}

/// Portal frames from door geometry (M8 prep): every door model has
/// bounds, most links can render as portals, arrival markers face out of
/// their door, and the census office's doors pair up as portals.
/// Full numbers: `cargo run --release -p mw_world --example portal_report`.
#[test]
fn door_links_have_portal_frames() {
    use mw_world::portal::{self, Facing, PortalClass};
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;
    let records = WorldRecords::build(db);
    let (ends, _) = mw_world::worldspaces::door_ends(db);
    let models = portal::door_models(db);
    let bounds = portal::load_bounds(&vfs, &models);
    let report = portal::analyze(&ends, &records.door_links, &models, &|m| bounds.get(m).copied());
    let (ready, adjust, fade) =
        (report.count(PortalClass::Ready), report.count(PortalClass::Adjust), report.count(PortalClass::FadeOnly));
    eprintln!("portal classes: ready {ready}, adjust {adjust}, fade-only {fade}; ambiguous doors {}", report.ambiguous.len());
    assert_eq!(report.links.len(), records.door_links.len());
    assert!(report.doors.iter().all(|d| d.geometry.shape != portal::DoorShape::Missing), "every door model has bounds");
    assert!(ready * 10 > report.links.len() * 7, "most links are portal-ready: {ready}");
    assert!(fade * 10 < report.links.len() * 2, "fade-only stays a minority: {fade}");

    // Arrivals at upright doors face out of the door (forward = engine -Z).
    let mut away = 0;
    let mut total = 0;
    for l in report.links.iter().filter(|l| l.b.is_some()) {
        for (door, check) in [(l.a, l.arrival_a), (l.b.unwrap(), l.arrival_b)] {
            if report.doors[door].geometry.shape != portal::DoorShape::Horizontal {
                let check = check.unwrap();
                total += 1;
                away += usize::from(check.facing_class() == Facing::Away);
                assert!(check.ahead >= 0.0, "arrival in front of its door");
            }
        }
    }
    assert!(away * 100 > total * 95, "{away} of {total} arrivals face away from their door");

    // The census office's exit door and the street door it leads to.
    let census = ids::interior("Seyda Neen, Census and Excise Office");
    let exit = report.links.iter().find(|l| {
        let a = &report.doors[l.a];
        l.b.is_some() && (a.base.eq_ignore_ascii_case("CharGen Exit Door") || report.doors[l.b.unwrap()].base.eq_ignore_ascii_case("CharGen Exit Door"))
    });
    let exit = exit.expect("census office exit link");
    assert_ne!(exit.class, PortalClass::FadeOnly, "{exit:?}");
    assert!(exit.size_ratio < 1.3, "{exit:?}");
    assert!(records.door_links.iter().any(|(_, l)| l.a.worldspace == census || l.b.worldspace == census));
}
