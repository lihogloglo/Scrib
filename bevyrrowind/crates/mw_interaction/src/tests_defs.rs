//! `def_for` on synthetic measured furniture: exact slot counts, frames, groups, bands, entrances.

use std::f32::consts::{FRAC_PI_2, PI};

use glam::{Quat, Vec3};
use sf_interaction::{InteractableDef, Side, SlotDef, SlotKind, TagId};
use sf_records::RecordId;

use crate::defs::{Facing, Measured, def_for};
use crate::furniture::Kind;
use crate::tunables::FurnitureParams;

const EPS: f32 = 1.0e-5;

fn measured(footprint: [f32; 2], top: f32, front: Facing) -> Measured {
    Measured { center: [0.0, 0.0], footprint, top, front }
}

fn def(kind: Kind, m: &Measured) -> InteractableDef {
    def_for(&RecordId::new("Furn_Test"), kind, m, &FurnitureParams::default()).expect("a def")
}

fn near(a: Vec3, b: Vec3) -> bool {
    a.distance(b) < EPS
}

/// The way a slot frame faces (+Z of the frame).
fn faces(s: &SlotDef) -> Vec3 {
    s.local.rotation * Vec3::Z
}

fn family(s: &SlotDef) -> TagId {
    match s.kind {
        SlotKind::Pose { family } => family,
        ref k => panic!("not a pose slot: {k:?}"),
    }
}

fn check_pose(s: &SlotDef, fam: &str, verb: &str, clips: &str, band: f32) {
    assert_eq!(family(s), TagId::of(fam));
    assert_eq!(s.verb.verb, TagId::of(verb));
    assert_eq!(s.clips.0, clips);
    assert!((s.band - band).abs() < EPS, "band {} != {band}", s.band);
    assert!(s.local.translation.y.abs() < EPS, "root on the floor");
}

#[test]
fn chair_is_one_front_seat() {
    let d = def(Kind::Seat, &measured([0.5, 0.5], 0.48, Facing::PosZ));
    assert_eq!(d.id, RecordId::new("furn_test"));
    assert!(d.verbs.is_empty() && d.carry.is_none() && d.destructible.is_none());
    assert_eq!(d.slots.len(), 1);
    let s = &d.slots[0];
    check_pose(s, "seat", "sit", "seat.chair", 0.48);
    assert!(near(s.local.translation, Vec3::ZERO));
    assert!(near(faces(s), Vec3::Z));
    assert_eq!(s.group, None);
    assert!(s.reach.abs() < EPS);
    assert_eq!(s.entrances.len(), 1);
    assert_eq!(s.entrances[0].side, Side::Front);
    // Half depth 0.25 + approach 0.3.
    assert!(near(s.entrances[0].approach, Vec3::new(0.0, 0.0, 0.55)));
}

#[test]
fn chair_facing_neg_x_off_centre() {
    let m = Measured { center: [1.0, 2.0], footprint: [0.6, 0.4], top: 0.54, front: Facing::NegX };
    let d = def(Kind::Seat, &m);
    assert_eq!(d.slots.len(), 1);
    let s = &d.slots[0];
    assert!(near(s.local.translation, Vec3::new(1.0, 0.0, 2.0)));
    assert!(near(faces(s), Vec3::NEG_X));
    assert!(s.local.rotation.abs_diff_eq(Quat::from_rotation_y(-FRAC_PI_2), EPS));
    // Depth along x: half 0.3, + 0.3.
    assert!(near(s.entrances[0].approach, Vec3::new(0.4, 0.0, 2.0)));
}

#[test]
fn chair_facing_neg_z_matches_the_shell_convention() {
    let d = def(Kind::Seat, &measured([0.5, 0.5], 0.45, Facing::NegZ));
    assert!(d.slots[0].local.rotation.abs_diff_eq(Quat::from_rotation_y(PI), EPS));
}

/// Two places plus the one between them: def.rs's example masks.
#[test]
fn bench_of_two_has_three_overlapping_slots() {
    let d = def(Kind::Seat, &measured([1.2, 0.4], 0.64, Facing::PosZ));
    assert_eq!(d.slots.len(), 3);
    let xs: Vec<f32> = d.slots.iter().map(|s| s.local.translation.x).collect();
    for (x, want) in xs.iter().zip([-0.3, 0.0, 0.3]) {
        assert!((x - want).abs() < EPS, "{xs:?}");
    }
    let groups: Vec<_> = d.slots.iter().map(|s| s.group).collect();
    assert_eq!(groups, [Some(0b01), Some(0b11), Some(0b10)]);
    for s in &d.slots {
        check_pose(s, "seat", "sit", "seat.bench", 0.64);
        assert!(near(faces(s), Vec3::Z));
        assert_eq!(s.entrances.len(), 1);
        let entrance = s.entrances[0];
        assert_eq!(entrance.side, Side::Front);
        assert!(near(entrance.approach, s.local.translation + Vec3::new(0.0, 0.0, 0.5)));
    }
}

/// 1.10 m is exactly two places (fit tolerance), 1.09 m one.
#[test]
fn bench_place_count_at_the_boundary() {
    assert_eq!(def(Kind::Seat, &measured([1.1, 0.4], 0.5, Facing::PosZ)).slots.len(), 3);
    assert_eq!(def(Kind::Seat, &measured([1.09, 0.4], 0.5, Facing::PosZ)).slots.len(), 1);
    // The places run across the front, not along the longest side.
    assert_eq!(def(Kind::Seat, &measured([0.4, 1.2], 0.5, Facing::PosZ)).slots.len(), 1);
    assert_eq!(def(Kind::Seat, &measured([0.4, 1.2], 0.5, Facing::PosX)).slots.len(), 3);
}

/// Five places fill the 8-bit mask (9 slots, bits 0-7); six get whole places only.
#[test]
fn long_benches() {
    let five = def(Kind::Seat, &measured([3.0, 0.4], 0.5, Facing::PosZ));
    assert_eq!(five.slots.len(), 9);
    let groups: Vec<u8> = five.slots.iter().map(|s| s.group.expect("grouped")).collect();
    assert_eq!(groups, [0b1, 0b11, 0b110, 0b1100, 0b1_1000, 0b11_0000, 0b110_0000, 0b1100_0000, 0b1000_0000]);
    // Neighbours exclude each other, slots two apart don't.
    for k in 0..groups.len() - 1 {
        assert_ne!(groups[k] & groups[k + 1], 0);
        if k + 2 < groups.len() {
            assert_eq!(groups[k] & groups[k + 2], 0);
        }
    }
    let six = def(Kind::Seat, &measured([3.5, 0.4], 0.5, Facing::PosZ));
    assert_eq!(six.slots.len(), 6);
    assert!(six.slots.iter().all(|s| s.group.is_none()));
    let pitch = 3.5 / 6.0;
    let mut want = -1.75 + pitch / 2.0;
    for s in &six.slots {
        assert!((s.local.translation.x - want).abs() < EPS);
        want += pitch;
    }
}

#[test]
fn bed_lies_and_sits_on_its_edge() {
    let d = def(Kind::Bed, &measured([2.0, 1.0], 0.74, Facing::PosZ));
    assert_eq!(d.slots.len(), 2);
    let (lie, edge) = (&d.slots[0], &d.slots[1]);
    check_pose(lie, "bed", "lie", "bed.lie", 0.74);
    check_pose(edge, "seat", "sit", "seat.bed_edge", 0.74);
    assert!(near(lie.local.translation, Vec3::ZERO));
    assert!(near(faces(lie), Vec3::Z));
    let sides: Vec<_> = lie.entrances.iter().map(|e| (e.side, e.approach)).collect();
    assert_eq!(sides.len(), 2);
    assert_eq!(sides[0].0, Side::Front);
    assert!(near(sides[0].1, Vec3::new(0.0, 0.0, 0.8)));
    assert_eq!(sides[1].0, Side::Back);
    assert!(near(sides[1].1, Vec3::new(0.0, 0.0, -0.8)));
    // Half depth 0.5 - inset 0.2.
    assert!(near(edge.local.translation, Vec3::new(0.0, 0.0, 0.3)));
    assert!(near(faces(edge), Vec3::Z));
    assert_eq!(edge.entrances.len(), 1);
    assert_eq!(edge.entrances[0].side, Side::Front);
    assert!(near(edge.entrances[0].approach, Vec3::new(0.0, 0.0, 0.8)));
    // They exclude each other.
    assert_eq!(lie.group, Some(0b1));
    assert_eq!(edge.group, Some(0b1));
}

#[test]
fn bedroll_edge_seat_stays_inside() {
    let d = def(Kind::Bed, &measured([1.8, 0.3], 0.22, Facing::PosZ));
    // Half depth 0.15 < inset 0.2: the edge seat sits at the centre, not behind it.
    assert!(near(d.slots[1].local.translation, Vec3::ZERO));
}

#[test]
fn table_has_stations_on_four_edges() {
    let d = def(Kind::Table, &measured([1.2, 0.8], 0.84, Facing::PosZ));
    // Front and back: 1.2 m / 0.6 = 2 each; the 0.8 m sides: 1 each.
    assert_eq!(d.slots.len(), 6);
    for s in &d.slots {
        check_pose(s, "counter", "lean", "counter.table", 0.84);
        assert_eq!(s.group, None);
        assert_eq!(s.entrances.len(), 1);
        assert_eq!(s.entrances[0].side, Side::Back);
        // Entered from behind: the approach is 0.3 m back from the root, against the facing.
        assert!(near(s.entrances[0].approach, s.local.translation - faces(s) * 0.3));
    }
    let roots: Vec<Vec3> = d.slots.iter().map(|s| s.local.translation).collect();
    let want = [
        Vec3::new(-0.3, 0.0, 0.7),
        Vec3::new(0.3, 0.0, 0.7),
        Vec3::new(0.3, 0.0, -0.7),
        Vec3::new(-0.3, 0.0, -0.7),
        Vec3::new(0.9, 0.0, 0.0),
        Vec3::new(-0.9, 0.0, 0.0),
    ];
    for (root, w) in roots.iter().zip(want) {
        assert!(near(*root, w), "{roots:?}");
    }
    let facings: Vec<Vec3> = d.slots.iter().map(faces).collect();
    for (face, w) in facings.iter().zip([Vec3::NEG_Z, Vec3::NEG_Z, Vec3::Z, Vec3::Z, Vec3::NEG_X, Vec3::X]) {
        assert!(near(*face, w), "{facings:?}");
    }
    // Reach to the centre line over the station's share: front hypot(0.4 + 0.3, 0.3), side
    // hypot(0.6 + 0.3, 0.4).
    assert!((d.slots[0].reach - 0.7f32.hypot(0.3)).abs() < EPS);
    assert!((d.slots[4].reach - 0.9f32.hypot(0.4)).abs() < EPS);
}

#[test]
fn counter_has_stations_front_and_back() {
    let d = def(Kind::Counter, &measured([2.0, 0.6], 0.94, Facing::PosZ));
    // 2.0 m / 0.6 = 3 per side.
    assert_eq!(d.slots.len(), 6);
    for s in &d.slots {
        check_pose(s, "counter", "lean", "counter.bar", 0.94);
        assert!((s.reach - 0.6f32.hypot(1.0 / 3.0)).abs() < EPS);
    }
    let front = d.slots.iter().filter(|s| near(faces(s), Vec3::NEG_Z)).count();
    let back = d.slots.iter().filter(|s| near(faces(s), Vec3::Z)).count();
    assert_eq!((front, back), (3, 3));
    assert!(d.slots.iter().all(|s| (s.local.translation.z.abs() - 0.6).abs() < EPS));
}

#[test]
fn shelf_and_degenerate_shapes_have_no_def() {
    let p = FurnitureParams::default();
    let id = RecordId::new("furn_x");
    assert!(def_for(&id, Kind::Shelf, &measured([1.0, 0.4], 1.5, Facing::PosZ), &p).is_none());
    assert!(def_for(&id, Kind::Seat, &measured([0.0, 0.4], 0.5, Facing::PosZ), &p).is_none());
    assert!(def_for(&id, Kind::Seat, &measured([0.5, 0.4], -0.1, Facing::PosZ), &p).is_none());
    assert!(def_for(&id, Kind::Table, &measured([f32::NAN, 0.4], 0.8, Facing::PosZ), &p).is_none());
    let m = Measured { center: [f32::INFINITY, 0.0], footprint: [0.5, 0.5], top: 0.5, front: Facing::PosZ };
    assert!(def_for(&id, Kind::Seat, &m, &p).is_none());
}

/// Classic: `mw.rules.interaction.sit` off = no furniture def at all.
#[test]
fn sit_switch_off_gives_no_furniture_defs() {
    let p = FurnitureParams { sit: false, ..FurnitureParams::default() };
    let id = RecordId::new("furn_x");
    for kind in Kind::ALL {
        assert!(def_for(&id, kind, &measured([1.2, 0.8], 0.6, Facing::PosZ), &p).is_none(), "{kind:?}");
    }
    assert!(FurnitureParams::default().sit);
}

#[test]
fn params_default_to_the_declarations() {
    let p = FurnitureParams::default();
    assert!((p.bench_seat_m - 0.55).abs() < EPS);
    assert!((p.approach_m - 0.3).abs() < EPS);
    assert!((p.edge_seat_inset_m - 0.2).abs() < EPS);
    assert!((p.counter_spacing_m - 0.6).abs() < EPS);
    assert!((p.counter_stand_m - 0.3).abs() < EPS);
    let names: Vec<&str> = crate::TUNABLES.iter().map(|d| d.name()).collect();
    assert_eq!(
        names,
        [
            "mw.rules.interaction.sit",
            "mw.interact.bench_seat_m",
            "mw.interact.approach_m",
            "mw.interact.edge_seat_inset_m",
            "mw.interact.counter_spacing_m",
            "mw.interact.counter_stand_m",
        ]
    );
    // Every number has a range: the shell's registration accepts the whole list.
    let mut registry = sf_tunables::Tunables::default();
    registry.register_all(crate::TUNABLES).expect("registers");
}
