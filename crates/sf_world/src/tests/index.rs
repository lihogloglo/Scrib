use std::collections::BTreeSet;

use glam::{DVec3, IVec3, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::RecordDb;

use super::{Lcg, models, path, placement, rotated, ws};
use crate::*;

/// `n` placements scattered over a `size`-meter square (and some height),
/// with every model kind, rotations, and scales.
fn scatter(n: usize, size: f64, seed: u64) -> Vec<Placement> {
    let mut rng = Lcg(seed);
    let kinds = [Some("small.mesh"), Some("big.mesh"), Some("tower.mesh"), None, Some("unknown.mesh")];
    let classes = [PlacementClass::Static, PlacementClass::Light, PlacementClass::Actor, PlacementClass::Custom(3)];
    (0..n)
        .map(|i| {
            let pos = DVec3::new(rng.range(-size, size), rng.range(-20.0, 200.0), rng.range(-size, size));
            let model = kinds[rng.next_u32() as usize % kinds.len()];
            let class = classes[rng.next_u32() as usize % classes.len()];
            #[allow(clippy::cast_possible_truncation)]
            let angle = rng.range(0.0, 6.0) as f32;
            #[allow(clippy::cast_possible_truncation)]
            let s = rng.range(0.5, 2.0) as f32;
            rotated(placement(&format!("p{i}"), pos, model, class), angle, Vec3::new(s, s * 1.5, s))
        })
        .collect()
}

#[test]
fn every_placement_lands_in_exactly_one_cell() {
    let input = scatter(5000, 2000.0, 1);
    let params = IndexParams::default();
    let (pages, stats) = build_pages(&params, input.clone(), &models);
    assert_eq!(stats.placements, 5000);
    assert!(stats.duplicate_ids.is_empty());
    assert_eq!(stats.missing_models, vec![path("unknown.mesh")]);

    let mut seen = BTreeSet::new();
    let mut total = 0;
    for page in &pages {
        for cell in &page.cells {
            assert_eq!(Partition::page_of(cell.cell), page.page);
            for p in &cell.placements {
                assert_eq!(params.partition.cell_of(p.placement.transform.pos), cell.cell);
                assert!(seen.insert(p.placement.id), "placement indexed twice");
                total += 1;
            }
        }
    }
    assert_eq!(total, input.len());
    assert_eq!(seen, input.iter().map(|p| p.id).collect());
}

#[test]
fn content_bounds_contain_every_placement() {
    let input = scatter(3000, 1500.0, 2);
    let params = IndexParams::default();
    let (pages, _) = build_pages(&params, input, &models);
    let mut grew = false;
    for cell in pages.iter().flat_map(|p| &p.cells) {
        let origin = params.partition.cell_origin(cell.cell);
        for p in &cell.placements {
            let info = p.placement.model.as_ref().and_then(models);
            let (b, radius) = placement_bounds(&p.placement, info.as_ref());
            assert_eq!(radius, p.radius);
            let b = b.translated(origin.delta_f32(p.placement.transform.pos));
            assert!(cell.bounds.contains(&b, 1e-3), "{:?} outside {:?}", b, cell.bounds);
        }
        // Big models stick out of their 128 m cell.
        grew |= cell.bounds.max.y > 128.0 || cell.bounds.min.x < 0.0;
    }
    assert!(grew);
}

#[test]
fn rotated_bounds_are_tight_boxes() {
    let mut p = placement("t", DVec3::ZERO, Some("tower.mesh"), PlacementClass::Static);
    p.transform.rotation = glam::Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    p.transform.scale = Vec3::splat(2.0);
    let (b, radius) = placement_bounds(&p, models(&path("tower.mesh")).as_ref());
    // A 10 × 300 × 10 tower, doubled and laid on its side along x.
    assert!(b.max.abs_diff_eq(Vec3::new(300.0, 10.0, 10.0), 1e-3), "{b:?}");
    assert!(b.min.abs_diff_eq(-b.max, 1e-3));
    assert!((radius - 2.0 * Vec3::new(5.0, 150.0, 5.0).length()).abs() < 1e-3);
}

#[test]
fn cells_sort_by_class_then_size_and_ignore_input_order() {
    let input = scatter(4000, 300.0, 3);
    let params = IndexParams::default();
    let (pages, _) = build_pages(&params, input.clone(), &models);
    for cell in pages.iter().flat_map(|p| &p.cells) {
        for w in cell.placements.windows(2) {
            let (a, b) = (&w[0], &w[1]);
            assert!(a.placement.class < b.placement.class || (a.placement.class == b.placement.class && a.radius >= b.radius));
        }
    }
    let mut reversed = input;
    reversed.reverse();
    assert_eq!(build_pages(&params, reversed, &models).0, pages);
}

#[test]
fn byte_estimates_count_models_once_per_cell() {
    let pos = DVec3::new(10.0, 1.0, 10.0);
    let input = vec![
        placement("a", pos, Some("big.mesh"), PlacementClass::Static),
        placement("b", pos, Some("big.mesh"), PlacementClass::Static),
        placement("c", pos, Some("small.mesh"), PlacementClass::Static),
    ];
    let (pages, _) = build_pages(&IndexParams::default(), input, &models);
    let cell = &pages[0].cells[0];
    assert_eq!(cell.bytes_all, 500_000 + 10_000 + 3 * PLACEMENT_BYTES);
    // Only the 20 m objects are far-ring content.
    assert_eq!(cell.bytes_far, 500_000 + 2 * PLACEMENT_BYTES);
    assert_eq!(cell.summary(4.0).far_placements, 2);
    assert_eq!(cell.lod_distance, 500.0);
}

#[test]
fn pages_and_directory_round_trip() {
    let params = IndexParams::default();
    let (pages, _) = build_pages(&params, scatter(2000, 1200.0, 4), &models);
    for page in &pages {
        let bytes = page.to_bytes();
        assert_eq!(&IndexPage::from_bytes(&bytes).unwrap(), page);
    }
    let dir = build_directory(ws("world"), &params, &pages);
    assert_eq!(CellDirectory::from_bytes(&dir.to_bytes()).unwrap(), dir);
    assert!(matches!(CellDirectory::from_bytes(&pages[0].to_bytes()), Err(CodecError::BadMagic { .. })));

    // The directory finds every cell, and its page slices are the pages.
    for (pi, page) in pages.iter().enumerate() {
        assert_eq!(dir.pages[pi].page, page.page);
        let slice = dir.page_cells(&dir.pages[pi]);
        assert_eq!(slice.len(), page.cells.len());
        for c in &page.cells {
            let i = dir.find(c.cell).unwrap();
            assert_eq!(dir.cells[i].cell, c.cell);
            assert_eq!(page.cell(c.cell).unwrap(), c);
        }
    }
    assert_eq!(dir.find(IVec3::splat(10_000)), None);
}

#[test]
fn page_bytes_are_compact() {
    let params = IndexParams::default();
    // A typical dense page: many copies of few models.
    let input: Vec<Placement> = (0..2000u32)
        .map(|i| {
            let x = f64::from(i % 50) * 20.0;
            let z = f64::from(i / 50) * 20.0;
            placement(&format!("p{i}"), DVec3::new(x, 0.0, z), Some(if i % 2 == 0 { "small.mesh" } else { "big.mesh" }), PlacementClass::Static)
        })
        .collect();
    let (pages, _) = build_pages(&params, input, &models);
    let bytes: usize = pages.iter().map(|p| p.to_bytes().len()).sum();
    // id 16-17, refs ~3, position ~16-20, rotation 16, scale 1, class/flags 2, radius 4.
    println!("index pages: {} bytes per placement", bytes / 2000);
    assert!(bytes / 2000 < 72, "{} bytes per placement", bytes / 2000);
}

#[test]
fn bucketing_matches_pages() {
    let params = IndexParams::default();
    let input = scatter(1500, 2500.0, 5);
    let buckets = bucket_by_page(params.partition, input.clone());
    let (pages, _) = build_pages(&params, input, &models);
    assert_eq!(buckets.iter().map(|b| b.0).collect::<Vec<_>>(), pages.iter().map(|p| p.page).collect::<Vec<_>>());
    for ((_, bucket), page) in buckets.into_iter().zip(&pages) {
        let (rebuilt, _) = build_pages(&params, bucket, &models);
        assert_eq!(&rebuilt[0], page);
    }
}

#[test]
fn single_partition_holds_everything_in_one_cell() {
    let params = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let (pages, stats) = build_pages(&params, scatter(300, 50.0, 6), &models);
    assert_eq!((stats.pages, stats.cells), (1, 1));
    assert_eq!(pages[0].cells[0].cell, IVec3::ZERO);
    assert_eq!(pages[0].cells[0].placements.len(), 300);
}

fn group(world: &str, keys: &[&str], x: f64) -> PlacementGroup {
    PlacementGroup {
        worldspace: ws(world),
        placements: (0u32..)
            .zip(keys)
            .map(|(i, k)| GroupPlacement {
                key: (*k).into(),
                base: RecordRef::new("thing", *k),
                model: Some(path("small.mesh")),
                transform: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(x + f64::from(i) * 3.0, 0.0, 0.0))),
                class: PlacementClass::Static,
                flags: PlacementFlags::empty(),
            })
            .collect(),
    }
}

#[test]
fn native_source_indexes_groups_and_fingerprints_regions() {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let modded = db.add_source("mod", &[base]);
    db.define(base, "town".into(), group("world", &["a", "b", "c"], 0.0)).unwrap();
    db.define(base, "farm".into(), group("world", &["a", "b"], 900.0)).unwrap();
    db.define(base, "shop".into(), group("shop", &["counter"], 0.0)).unwrap();

    let regions = |src: &NativeWorldSource| {
        let mut out = Vec::new();
        src.regions(&ws("world"), &mut |r, f| out.push((r, f)));
        out
    };
    let before = NativeWorldSource::from_db(&db);
    let index = build_index(&ws("world"), &IndexParams::default(), &before, &models);
    assert_eq!(index.stats.placements, 5);
    assert!(index.stats.duplicate_ids.is_empty(), "same keys in different groups get different ids");
    assert_eq!(before.worldspaces().count(), 2);

    // A mod moves one placement of the farm: only the farm's fingerprint changes.
    db.patch::<PlacementGroup>(modded, &"farm".into(), |g| {
        g.placements[1].transform.pos = g.placements[1].transform.pos.offset(Vec3::X);
    })
    .unwrap();
    let after = NativeWorldSource::from_db(&db);
    let (r0, r1) = (regions(&before), regions(&after));
    assert_eq!(r0.len(), 2);
    assert_eq!(r0[0], r1[0], "untouched group keeps its fingerprint");
    assert_eq!(r0[1].0, r1[1].0);
    assert_ne!(r0[1].1, r1[1].1);
    let moved = build_index(&ws("world"), &IndexParams::default(), &after, &models);
    let id = PlacementGroup::placement_id(&"farm".into(), "b");
    let found = moved.pages.iter().flat_map(|p| &p.cells).flat_map(|c| &c.placements).find(|p| p.placement.id == id).unwrap();
    assert!((found.placement.transform.pos.to_f64().x - 904.0).abs() < 1e-4);
}
