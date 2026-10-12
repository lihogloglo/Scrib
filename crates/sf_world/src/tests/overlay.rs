use glam::{DVec3, Quat};
use sf_coords::WorldPos;

use super::{models, placement, ws};
use crate::*;

fn cell_ids(o: &PlacementOverlay, world: &str, page: &IndexPage, cell: glam::IVec3) -> Vec<PlacementId> {
    let indexed = page.cell(cell).map_or(&[][..], |c| &c.placements[..]);
    o.apply(&ws(world), page.partition, cell, indexed).iter().map(|p| p.placement.id).collect()
}

#[test]
fn overlay_removes_moves_and_spawns() {
    let params = IndexParams::default();
    let a = placement("a", DVec3::new(10.0, 1.0, 10.0), Some("small.mesh"), PlacementClass::Dynamic);
    let b = placement("b", DVec3::new(20.0, 1.0, 10.0), Some("small.mesh"), PlacementClass::Dynamic);
    let c = placement("c", DVec3::new(30.0, 1.0, 10.0), Some("small.mesh"), PlacementClass::Dynamic);
    let far = placement("far", DVec3::new(150.0, 1.0, 10.0), Some("small.mesh"), PlacementClass::Static);
    let (pages, _) = build_pages(&params, vec![a.clone(), b.clone(), c.clone(), far.clone()], &models);
    let page = &pages[0];
    let home = params.partition.cell_of(a.transform.pos);
    let next = params.partition.cell_of(far.transform.pos);
    assert_ne!(home, next);

    let mut o = PlacementOverlay::new();
    o.remove(b.id);
    // c moves into the next cell.
    let to = Location::new(ws("world"), WorldPos::from_f64(DVec3::new(140.0, 1.0, 12.0)));
    o.move_to(&c, &to, Quat::from_rotation_y(1.0));
    let spawned = o.spawn(ws("world"), placement("ignored", DVec3::new(12.0, 1.0, 12.0), None, PlacementClass::Dynamic));
    o.set_state(a.id, "inventory", vec![1, 2, 3]);

    assert_eq!(cell_ids(&o, "world", page, home), vec![a.id, spawned]);
    let mut next_ids = vec![far.id, c.id];
    assert_eq!(cell_ids(&o, "world", page, next), next_ids);
    // Other worldspaces never see them.
    assert_eq!(cell_ids(&o, "other", page, next), vec![far.id]);
    assert_eq!(o.state(a.id, "inventory"), Some(&[1, 2, 3][..]));
    assert_eq!(o.states(a.id).count(), 1);
    assert!(o.is_removed(b.id) && o.is_spawned(spawned));

    // Round trip through save bytes.
    let back = PlacementOverlay::from_bytes(&o.to_bytes()).unwrap();
    assert_eq!(back, o);

    // Moving c into another worldspace takes it out of this one.
    o.move_to(&c, &Location::new(ws("cave"), WorldPos::ORIGIN), Quat::IDENTITY);
    next_ids.pop();
    assert_eq!(cell_ids(&o, "world", page, next), next_ids);
    assert_eq!(o.placed(c.id).unwrap().0, &ws("cave"));
    // A removed spawned object is forgotten; ids are never reused.
    o.remove(spawned);
    assert!(!o.is_spawned(spawned) && !o.is_removed(spawned));
    let again = o.spawn(ws("world"), placement("x", DVec3::ZERO, None, PlacementClass::Dynamic));
    assert_ne!(again, spawned);
    // Placing a removed object restores it.
    o.place(ws("world"), b.clone());
    assert!(!o.is_removed(b.id));
    assert!(cell_ids(&o, "world", page, home).contains(&b.id));
}

#[test]
fn overlay_buckets_work_for_every_partition() {
    let p = placement("p", DVec3::new(700.0, 5.0, -300.0), None, PlacementClass::Dynamic);
    let mut o = PlacementOverlay::new();
    o.place(ws("w"), p.clone());
    for partition in [Partition::Grid { shift: -2 }, Partition::Grid { shift: 0 }, Partition::Grid { shift: 2 }, Partition::Single] {
        let cell = partition.cell_of(p.transform.pos);
        let got = o.apply(&ws("w"), partition, cell, &[]);
        assert_eq!(got.len(), 1, "{partition:?}");
        assert_eq!(got[0].radius, 0.0);
        if partition != Partition::Single {
            assert!(o.apply(&ws("w"), partition, cell + glam::IVec3::X, &[]).is_empty());
        }
    }
}
