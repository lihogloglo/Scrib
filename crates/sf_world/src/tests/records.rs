use std::f32::consts::FRAC_PI_2;

use glam::{DVec3, IVec3, Quat, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId};

use super::{path, ws};
use crate::*;

fn end(space: &str, pos: DVec3, yaw: f32) -> LinkEnd {
    let frame = WorldTransform { pos: WorldPos::from_f64(pos), rotation: Quat::from_rotation_y(yaw), scale: Vec3::ONE };
    LinkEnd { worldspace: ws(space), frame, arrival: frame, door: None, extent: Vec2::new(1.5, 2.5) }
}

fn close(a: WorldPos, b: WorldPos) -> bool {
    a.delta(b).length() < 1e-4
}

#[test]
fn portal_maps_behind_a_to_in_front_of_b() {
    let a = end("outside", DVec3::new(1.0e6, 10.0, -2.0e5), 0.3);
    let b = end("inside", DVec3::new(5.0, 0.0, 7.0), -1.1);
    // 0.25 m behind a's plane (just stepped out of a), drifting sideways.
    let behind_a = a.frame.rotation * Vec3::new(0.4, 1.0, -0.25);
    let pose = WorldTransform { pos: a.frame.pos.offset(behind_a), rotation: a.frame.rotation, scale: Vec3::ONE };
    let out = DoorLink::cross(&a, &b, &pose);
    let local = b.frame.rotation.inverse() * b.frame.pos.delta_f32(out.pos);
    // The half-turn mirrors x and z: in front of b, height kept.
    assert!(local.abs_diff_eq(Vec3::new(-0.4, 1.0, 0.25), 1e-4), "{local}");
    // Walking out of a (−Z in a's frame) is walking into b (+Z in b's).
    let dir = DoorLink::cross_direction(&a, &b, a.frame.rotation * Vec3::NEG_Z);
    assert!((b.frame.rotation.inverse() * dir).abs_diff_eq(Vec3::Z, 1e-5));
    // The traveler faced along a's frame; now faces along b's frame turned around.
    let facing = out.rotation * Vec3::NEG_Z;
    assert!(facing.abs_diff_eq(b.frame.rotation * Vec3::Z, 1e-5), "{facing}");
}

#[test]
fn crossing_back_is_identity() {
    let a = end("a", DVec3::new(-3.0e5, 0.0, 12.0), FRAC_PI_2);
    let b = end("b", DVec3::new(100.0, 50.0, 100.0), 2.0);
    let pose = WorldTransform {
        pos: a.frame.pos.offset(Vec3::new(1.0, 1.7, -0.5)),
        rotation: Quat::from_rotation_y(0.7) * Quat::from_rotation_x(0.1),
        scale: Vec3::splat(1.1),
    };
    let there = DoorLink::cross(&a, &b, &pose);
    let back = DoorLink::cross(&b, &a, &there);
    assert!(close(back.pos, pose.pos));
    assert!(back.rotation.abs_diff_eq(pose.rotation, 1e-5) || back.rotation.abs_diff_eq(-pose.rotation, 1e-5));
    assert_eq!(back.scale, pose.scale);
}

#[test]
fn one_way_links_have_one_exit() {
    let mut link = DoorLink { a: end("a", DVec3::ZERO, 0.0), b: end("b", DVec3::ZERO, 0.0), two_way: true };
    assert_eq!(link.exits().count(), 2);
    link.two_way = false;
    let exits: Vec<_> = link.exits().collect();
    assert_eq!(exits.len(), 1);
    assert_eq!(exits[0].0.worldspace, ws("a"));
}

#[test]
fn placement_ids_are_stable_and_namespaced() {
    let a = PlacementId::from_key("mod", b"key");
    assert_eq!(a, PlacementId::from_key("mod", b"key"));
    assert_ne!(a, PlacementId::from_key("mo", b"dkey"));
    assert_ne!(a, PlacementId::from_key("other", b"key"));
    let g = RecordId::new("group");
    assert_eq!(PlacementGroup::placement_id(&g, "x"), PlacementGroup::placement_id(&RecordId::new("GROUP"), "x"));
    assert_ne!(PlacementGroup::placement_id(&g, "x"), PlacementGroup::placement_id(&g, "y"));
}

#[test]
fn partition_cells_and_pages() {
    let p = Partition::STREAMING;
    assert_eq!(p.cell_extent(), Some(128.0));
    // 128 m cells share the grid's corners (256·k − 128), so 0 is a corner.
    assert_eq!(p.cell_of(WorldPos::from_f64(DVec3::new(-128.0, 0.0, 0.0))), IVec3::new(0, 1, 1));
    assert_eq!(p.cell_of(WorldPos::from_f64(DVec3::new(-128.1, 0.0, 0.0))), IVec3::new(-1, 1, 1));
    assert_eq!(p.cell_of(WorldPos::from_f64(DVec3::new(-0.1, 0.0, 0.0))), IVec3::new(0, 1, 1));
    assert_eq!(p.cell_of(WorldPos::from_f64(DVec3::new(0.1, 0.0, 0.0))), IVec3::new(1, 1, 1));
    assert_eq!(p.cell_origin(IVec3::new(1, 1, 1)).to_f64(), DVec3::ZERO);
    assert_eq!(Partition::page_of(IVec3::new(7, -1, -8)), IVec3::new(0, -1, -1));
    assert_eq!(Partition::page_of(IVec3::new(8, -9, 15)), IVec3::new(1, -2, 1));
    assert_eq!(Partition::Single.cell_of(WorldPos::from_f64(DVec3::splat(1.0e6))), IVec3::ZERO);
}

#[test]
fn records_register_define_and_patch() {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let modded = db.add_source("mod", &[base]);
    db.define(base, "world".into(), Worldspace { has_terrain: true, ..Worldspace::new("World") }).unwrap();
    let group = PlacementGroup {
        worldspace: ws("world"),
        placements: vec![GroupPlacement {
            key: "rock".into(),
            base: RecordRef::new("thing", "rock"),
            model: Some(path("small.mesh")),
            transform: WorldTransform::IDENTITY,
            class: PlacementClass::Static,
            flags: PlacementFlags::empty(),
        }],
    };
    db.define(base, "rocks".into(), group).unwrap();
    db.patch::<PlacementGroup>(modded, &"rocks".into(), |g| {
        let mut extra = g.placements[0].clone();
        extra.key = "rock2".into();
        g.placements.push(extra);
    })
    .unwrap();
    assert!(db.get::<Worldspace>(&"world".into()).unwrap().has_terrain);
    assert_eq!(db.get::<PlacementGroup>(&"rocks".into()).unwrap().placements.len(), 2);
}

#[test]
fn records_read_from_text() {
    let json = r#"{
        "worldspace": "World",
        "placements": [
            { "key": "a", "base": { "kind": "thing", "id": "Rock" }, "model": "Meshes\\Rock.mesh",
              "transform": { "pos": [1000.5, 2.0, -3.0] }, "flags": "GROUNDED | NO_LOD" },
            { "key": "b", "base": { "kind": "thing", "id": "lamp" }, "class": "Light",
              "transform": { "pos": { "cell": [1, 0, 0], "local": [0.0, 0.0, 0.0] }, "scale": [2.0, 2.0, 2.0] } }
        ]
    }"#;
    let g: PlacementGroup = serde_json::from_str(json).unwrap();
    assert_eq!(g.worldspace, ws("world"));
    let a = &g.placements[0];
    assert_eq!(a.model.as_ref().unwrap().as_str(), "meshes/rock.mesh");
    assert_eq!(a.base.id.as_str(), "rock");
    assert_eq!(a.flags, PlacementFlags::GROUNDED | PlacementFlags::NO_LOD);
    assert_eq!(a.class, PlacementClass::Static);
    assert!((a.transform.pos.to_f64().x - 1000.5).abs() < 1e-6);
    assert_eq!(g.placements[1].class, PlacementClass::Light);
    assert_eq!(g.placements[1].transform.pos.to_f64().x, 256.0);

    let w: Worldspace = serde_json::from_str(r#"{ "name": "Shop", "partition": "Single" }"#).unwrap();
    assert_eq!(w.partition, Partition::Single);
    let w: Worldspace = serde_json::from_str(r#"{ "name": "World", "rings": { "far": { "load": 5000, "unload": 5500 } } }"#).unwrap();
    assert_eq!(w.partition, Partition::STREAMING);
    let rings = RingConfig::default().with_overrides(&w.rings.unwrap());
    assert_eq!(rings.far.load, 5000.0);
    assert_eq!(rings.near, RingConfig::default().near);

    // Round trip, ids as hex strings.
    let link = DoorLink {
        a: LinkEnd { door: Some(PlacementId(u128::MAX - 7)), ..end("a", DVec3::new(1.0, 2.0, 3.0), 0.0) },
        b: end("b", DVec3::ZERO, 0.0),
        two_way: false,
    };
    let text = serde_json::to_string(&link).unwrap();
    assert!(text.contains("fffffffffffffffffffffffffffffff8"));
    assert_eq!(serde_json::from_str::<DoorLink>(&text).unwrap(), link);
}
