//! nav.md "Build plan" `NV4c`: the interim ring's tile sets, exact. 8 m tiles for class 0 and 16 m
//! for class 1, so the expected sets are worked out by hand in each test.

use glam::{DVec3, Vec2};
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::{
    AgentClassId, ClassTiling, NavRing, RegionFrame, RingCounts, RingLayout, RingRadii, TileKey,
    TileState, tile_distance,
};

const H: AgentClassId = AgentClassId(0);
const L: AgentClassId = AgentClassId(1);

fn ws(name: &str) -> WorldspaceId {
    WorldspaceId::new(name)
}

fn at(ws_name: &str, x: f64, z: f64) -> Location {
    Location::new(ws(ws_name), WorldPos::from_f64(DVec3::new(x, 0.0, z)))
}

fn key(class: AgentClassId, region: u16, x: i32, z: i32) -> TileKey {
    TileKey {
        ws: ws("w"),
        class,
        region,
        x,
        z,
    }
}

fn layout(classes: &[(AgentClassId, f32)]) -> RingLayout {
    RingLayout {
        classes: classes
            .iter()
            .map(|&(class, tile_size)| ClassTiling { class, tile_size })
            .collect(),
        regions: std::collections::BTreeMap::default(),
        default_regions: vec![RegionFrame {
            region: 0,
            origin: WorldPos::ORIGIN,
        }],
    }
}

fn radii(load_m: f32, unload_m: f32) -> RingRadii {
    RingRadii { load_m, unload_m }
}

/// Reads every queued tile: row z = 0 is baked, the rest isn't.
fn drain(ring: &mut NavRing) {
    for k in ring.next(usize::MAX) {
        let state = if k.z == 0 {
            TileState::Resident
        } else {
            TileState::Unbaked
        };
        ring.settle(k, state);
    }
}

fn resident(ring: &NavRing) -> Vec<(i32, i32)> {
    ring.resident().map(|k| (k.x, k.z)).collect()
}

#[test]
fn tile_distance_is_zero_inside_and_to_the_nearest_edge_outside() {
    assert!(tile_distance(Vec2::new(4.0, 4.0), 0, 0, 8.0).abs() < f32::EPSILON);
    assert!((tile_distance(Vec2::new(4.0, 4.0), 1, 0, 8.0) - 4.0).abs() < 1e-6);
    assert!((tile_distance(Vec2::new(4.0, 4.0), 1, 1, 8.0) - 32f32.sqrt()).abs() < 1e-6);
    assert!((tile_distance(Vec2::new(4.0, 4.0), -2, 0, 8.0) - 12.0).abs() < 1e-6);
}

#[test]
fn tiles_within_is_nearest_first_per_class_and_region_origin() {
    // (4, 4) in 8 m tiles, radius 5: the tile itself, then its four sides at 4 m in key order;
    // the corners (5.66 m) are out.
    let one = layout(&[(H, 8.0)]);
    assert_eq!(
        one.tiles_within(&at("w", 4.0, 4.0), 5.0),
        vec![
            key(H, 0, 0, 0),
            key(H, 0, -1, 0),
            key(H, 0, 0, -1),
            key(H, 0, 0, 1),
            key(H, 0, 1, 0)
        ]
    );
    // Radius 6 takes the corners too.
    assert_eq!(one.tiles_within(&at("w", 4.0, 4.0), 6.0).len(), 9);

    // Region 3 at (1000, 0, -500) and a 16 m class: the anchor is (4, 4) in the region.
    let mut two = layout(&[(H, 8.0), (L, 16.0)]);
    two.regions.insert(
        ws("w"),
        vec![RegionFrame {
            region: 3,
            origin: WorldPos::from_f64(DVec3::new(1000.0, 0.0, -500.0)),
        }],
    );
    let got = two.tiles_within(&at("w", 1004.0, -496.0), 5.0);
    let class_l: Vec<_> = got.iter().filter(|k| k.class == L).cloned().collect();
    assert_eq!(
        class_l,
        vec![key(L, 3, 0, 0), key(L, 3, -1, 0), key(L, 3, 0, -1)]
    );
    assert_eq!(got.iter().filter(|k| k.class == H).count(), 5);
    assert!(got.iter().all(|k| k.region == 3));
    // Other worldspaces keep the default region.
    assert!(
        two.tiles_within(&at("v", 4.0, 4.0), 1.0)
            .iter()
            .all(|k| k.region == 0)
    );
}

#[test]
fn load_and_unload_radii_give_hysteresis() {
    let mut ring = NavRing::new(layout(&[(H, 8.0)]));
    let r = radii(5.0, 14.0);

    // x = 12: tiles 0, 1, 2 of row 0 (4, 0, 4 m), and (1, +-1) unbaked.
    assert!(ring.update(Some(&at("w", 12.0, 4.0)), r).is_empty());
    assert_eq!(ring.counts().pending, 5);
    drain(&mut ring);
    assert_eq!(resident(&ring), vec![(0, 0), (1, 0), (2, 0)]);
    assert_eq!(
        ring.counts(),
        RingCounts {
            resident: 3,
            unbaked: 2,
            ..RingCounts::default()
        }
    );

    // x = 21: tile 3 joins (3 m); tile 0 is 13 m away, past the load radius but within the
    // unload radius, so it stays.
    assert!(ring.update(Some(&at("w", 21.0, 4.0)), r).is_empty());
    drain(&mut ring);
    assert_eq!(resident(&ring), vec![(0, 0), (1, 0), (2, 0), (3, 0)]);

    // A still anchor plans nothing.
    assert!(ring.update(Some(&at("w", 21.5, 4.0)), r).is_empty());
    assert_eq!(ring.counts().pending, 0);
    // No anchor changes nothing.
    assert!(ring.update(None, r).is_empty());
    assert_eq!(ring.counts().resident, 4);

    // x = 28: tile 0 is 20 m away and goes; tile 1 (12 m) stays; tile 4 joins.
    assert_eq!(
        ring.update(Some(&at("w", 28.0, 4.0)), r),
        vec![key(H, 0, 0, 0)]
    );
    drain(&mut ring);
    assert_eq!(resident(&ring), vec![(1, 0), (2, 0), (3, 0), (4, 0)]);
}

#[test]
fn a_worldspace_change_drops_every_tile_of_the_old_one() {
    let mut ring = NavRing::new(layout(&[(H, 8.0)]));
    let r = radii(5.0, 14.0);
    ring.update(Some(&at("w", 12.0, 4.0)), r);
    drain(&mut ring);
    let mut dropped = ring.update(Some(&at("inside", 12.0, 4.0)), r);
    dropped.sort();
    assert_eq!(
        dropped,
        vec![key(H, 0, 0, 0), key(H, 0, 1, 0), key(H, 0, 2, 0)]
    );
    // The new worldspace's five tiles are queued; nothing of the old one is known.
    assert_eq!(
        ring.counts(),
        RingCounts {
            pending: 5,
            ..RingCounts::default()
        }
    );
    assert!(ring.next(usize::MAX).iter().all(|k| k.ws == ws("inside")));
}

#[test]
fn next_hands_out_the_nearest_first_up_to_the_cap() {
    let mut ring = NavRing::new(layout(&[(H, 8.0)]));
    ring.update(Some(&at("w", 4.0, 4.0)), radii(5.0, 6.0));
    assert_eq!(ring.next(2), vec![key(H, 0, 0, 0), key(H, 0, -1, 0)]);
    assert_eq!(ring.counts().pending, 3);
    assert_eq!(
        ring.next(10),
        vec![key(H, 0, 0, -1), key(H, 0, 0, 1), key(H, 0, 1, 0)]
    );
    assert!(ring.next(10).is_empty());
}

#[test]
fn radii_changes_replan_and_a_short_unload_radius_acts_as_the_load_radius() {
    let mut ring = NavRing::new(layout(&[(H, 8.0)]));
    ring.update(Some(&at("w", 4.0, 4.0)), radii(6.0, 6.0));
    drain(&mut ring);
    // Row 0: (-1, 0), (0, 0), (1, 0).
    assert_eq!(ring.counts().resident, 3);
    // Load 3, unload 1 (under the load radius): only tiles within 3 m stay, so the neighbours at
    // 4 m go.
    let mut dropped = ring.update(Some(&at("w", 4.0, 4.0)), radii(3.0, 1.0));
    dropped.sort();
    assert_eq!(dropped, vec![key(H, 0, -1, 0), key(H, 0, 1, 0)]);
    assert_eq!(resident(&ring), vec![(0, 0)]);
    // A failed tile is not retried while it stays in the ring.
    ring.update(Some(&at("w", 4.0, 4.0)), radii(5.0, 5.0));
    for k in ring.next(usize::MAX) {
        ring.settle(k, TileState::Failed);
    }
    ring.update(Some(&at("w", 4.0, 4.0)), radii(5.0, 5.5));
    assert_eq!(ring.counts().pending, 0);
    assert_eq!(ring.counts().failed, 4);
    ring.clear();
    assert_eq!(ring.counts(), RingCounts::default());
}
