//! `def_for` on synthetic measured furniture: exact slot counts, frames, groups, bands, entrances.

use std::f32::consts::{FRAC_PI_2, PI};

use glam::{Quat, Vec3};
use sf_interaction::{
    ClipFamily, DOOR_OPEN, DoorDef, Frame, IkTargets, InteractableDef, Occupancy, Side, SlotDef, SlotKind, SlotRef,
    TagFilter, TagId, VERB_CLOSE, VERB_OPEN, VERB_SLEEP, VerbDef, VerbTarget, rank_verbs,
};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

use crate::activation::{RecordClass, verb};
use crate::defs::{
    Basis, CLASS_VERB_PRIORITY, Facing, Measured, POSE_VERB_PRIORITY, USER_NPC, bed_verb, class_verb, def_for, door_def,
};
use crate::furniture::Kind;
use crate::tunables::FurnitureParams;

const EPS: f32 = 1.0e-5;

fn measured(footprint: [f32; 2], top: f32, front: Facing) -> Measured {
    Measured { center: [0.0, 0.0], footprint, floor: 0.0, top, front, head: front, basis: Basis::Upright }
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
    let m = Measured { center: [1.0, 2.0], footprint: [0.6, 0.4], floor: 0.0, top: 0.54, front: Facing::NegX, head: Facing::NegX, basis: Basis::Upright };
    let d = def(Kind::Seat, &m);
    assert_eq!(d.slots.len(), 1);
    let s = &d.slots[0];
    assert!(near(s.local.translation, Vec3::new(1.0, 0.0, 2.0)));
    assert!(near(faces(s), Vec3::NEG_X));
    assert!(s.local.rotation.abs_diff_eq(Quat::from_rotation_y(-FRAC_PI_2), EPS));
    // Depth along x: half 0.3, + 0.3.
    assert!(near(s.entrances[0].approach, Vec3::new(0.4, 0.0, 2.0)));
}

/// A model authored with its origin above its floor: roots and approaches stand on the floor,
/// the band stays the height above it.
#[test]
fn floor_offset_moves_roots_and_approaches() {
    let m = Measured { floor: -0.45, ..measured([0.5, 0.5], 0.45, Facing::PosZ) };
    for kind in [Kind::Seat, Kind::Bed, Kind::Table, Kind::Counter] {
        let d = def(kind, &m);
        assert!(!d.slots.is_empty(), "{kind:?}");
        for s in &d.slots {
            assert!((s.local.translation.y + 0.45).abs() < EPS, "{kind:?}");
            assert!(s.entrances.iter().all(|e| (e.approach.y + 0.45).abs() < EPS), "{kind:?}");
            assert!((s.band - 0.45).abs() < EPS, "{kind:?}");
        }
    }
    let nan = Measured { floor: f32::NAN, ..m };
    assert!(def_for(&RecordId::new("furn_x"), Kind::Seat, &nan, &FurnitureParams::default()).is_none());
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

/// AN25: the lie slot's +Z is the head, entered from the long sides (the lier's left and right);
/// one edge seat per long side facing out; all three share bit 0.
#[test]
fn bed_lies_head_first_entered_from_the_long_sides() {
    // 2.0 m long along z, head at +Z; 1.0 m wide.
    let d = def(Kind::Bed, &measured([1.0, 2.0], 0.74, Facing::PosZ));
    assert_eq!(d.slots.len(), 3);
    let (lie, left, right) = (&d.slots[0], &d.slots[1], &d.slots[2]);
    check_pose(lie, "bed", "lie", "bed.lie", 0.74);
    assert!(near(lie.local.translation, Vec3::ZERO));
    assert!(near(faces(lie), Vec3::Z), "head at +Z");
    let sides: Vec<_> = lie.entrances.iter().map(|e| (e.side, e.approach)).collect();
    assert_eq!(sides.len(), 2);
    // Face up, head +Z: the lier's right is +X. Half width 0.5 + approach 0.3.
    assert_eq!(sides[0].0, Side::Left);
    assert!(near(sides[0].1, Vec3::new(-0.8, 0.0, 0.0)));
    assert_eq!(sides[1].0, Side::Right);
    assert!(near(sides[1].1, Vec3::new(0.8, 0.0, 0.0)));
    for (edge, out) in [(left, Vec3::NEG_X), (right, Vec3::X)] {
        check_pose(edge, "seat", "sit", "seat.bed_edge", 0.74);
        // Half width 0.5 - inset 0.2, facing out.
        assert!(near(edge.local.translation, out * 0.3));
        assert!(near(faces(edge), out));
        assert_eq!(edge.entrances.len(), 1);
        assert_eq!(edge.entrances[0].side, Side::Front);
        assert!(near(edge.entrances[0].approach, out * 0.8));
    }
    assert!(d.slots.iter().all(|s| s.group == Some(0b1)), "they exclude each other");
}

/// The head, not the front, sets the bed frame: a head at -X turns everything.
#[test]
fn bed_frame_follows_the_head() {
    let m = Measured { center: [1.0, 0.0], footprint: [2.0, 1.0], floor: 0.0, top: 0.5, front: Facing::PosZ, head: Facing::NegX, basis: Basis::Upright };
    let d = def(Kind::Bed, &m);
    let lie = &d.slots[0];
    assert!(near(faces(lie), Vec3::NEG_X));
    assert!(lie.local.rotation.abs_diff_eq(Quat::from_rotation_y(-FRAC_PI_2), EPS));
    // Head -X: the lier's right is Y x -X = +Z.
    assert_eq!(lie.entrances[0].side, Side::Left);
    assert!(near(lie.entrances[0].approach, Vec3::new(1.0, 0.0, -0.8)));
    assert_eq!(lie.entrances[1].side, Side::Right);
    assert!(near(lie.entrances[1].approach, Vec3::new(1.0, 0.0, 0.8)));
    assert!(near(faces(&d.slots[1]), Vec3::NEG_Z));
    assert!(near(faces(&d.slots[2]), Vec3::Z));
}

#[test]
fn bedroll_edge_seats_stay_inside() {
    let d = def(Kind::Bed, &measured([0.3, 1.8], 0.22, Facing::PosZ));
    // Half width 0.15 < inset 0.2: both edge seats sit at the centre, not beyond it.
    assert!(near(d.slots[1].local.translation, Vec3::ZERO));
    assert!(near(d.slots[2].local.translation, Vec3::ZERO));
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
    let m = Measured {
        center: [f32::INFINITY, 0.0],
        footprint: [0.5, 0.5],
        floor: 0.0,
        top: 0.5,
        front: Facing::PosZ,
        head: Facing::PosZ,
        basis: Basis::Upright,
    };
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
    assert!((p.floor_seat_band_m - 0.3).abs() < EPS);
    let names: Vec<&str> = crate::TUNABLES.iter().map(|d| d.name()).collect();
    assert_eq!(
        names,
        [
            "mw.rules.interaction.sit",
            "mw.rules.interaction.snuff",
            "mw.rules.interaction.break",
            "mw.rules.interaction.break_crate_hp",
            "mw.rules.interaction.break_barrel_hp",
            "mw.rules.interaction.break_urn_hp",
            "mw.rules.interaction.break_sack_hp",
            "mw.rules.interaction.break_pottery_hp",
            "mw.rules.interaction.break_glass_hp",
            "mw.rules.interaction.break_blunt",
            "mw.rules.interaction.break_blade",
            "mw.rules.interaction.break_arrow",
            "mw.rules.interaction.break_hand",
            "mw.rules.interaction.break_fire_wood",
            "mw.rules.interaction.break_spill_loss_knee",
            "mw.rules.interaction.break_spill_loss_span",
            "mw.rules.interaction.break_spill_loss_max",
            "mw.rules.interaction.break_spill_health_per_overkill",
            "mw.rules.interaction.break_spill_health_max",
            "mw.rules.interaction.break_spill_radius_m",
            "mw.rules.interaction.break_spill_lift_m",
            "mw.rules.interaction.break_spill_layer_m",
            "mw.interact.bench_seat_m",
            "mw.interact.approach_m",
            "mw.interact.edge_seat_inset_m",
            "mw.interact.counter_spacing_m",
            "mw.interact.counter_stand_m",
            "mw.interact.floor_seat_band_m",
            "mw.interact.measure.backrest_above_m",
            "mw.interact.measure.backrest_offset",
            "mw.interact.measure.upright_min_area",
            "mw.interact.measure.bench_aspect",
            "mw.interact.measure.headboard_above_m",
            "mw.interact.measure.bed_end_share",
            "mw.interact.measure.head_share",
        ]
    );
    // Every number has a range: the shell's registration accepts the whole list.
    let mut registry = sf_tunables::Tunables::default();
    registry.register_all(crate::TUNABLES).expect("registers");
}

// --- Seat families (IN32c0) -----------------------------------------------------------------------

fn seat_clips(m: &Measured) -> Vec<String> {
    def(Kind::Seat, m).slots.iter().map(|s| s.clips.0.clone()).collect()
}

fn with_basis(mut m: Measured, basis: Basis) -> Measured {
    m.basis = basis;
    m
}

#[test]
fn seat_families_by_places_backrest_and_band() {
    let one = measured([0.45, 0.45], 0.45, Facing::PosZ);
    assert_eq!(seat_clips(&one), ["seat.chair"], "a backrest: chair");
    for basis in [Basis::PlusZ, Basis::LongSide] {
        assert_eq!(seat_clips(&with_basis(one, basis)), ["seat.stool"], "no backrest: stool ({basis:?})");
    }
    // Two places: a bench, with or without a backrest.
    let bench = measured([1.1, 0.4], 0.45, Facing::PosZ);
    assert_eq!(seat_clips(&bench), ["seat.bench"; 3]);
    assert_eq!(seat_clips(&with_basis(bench, Basis::LongSide)), ["seat.bench"; 3]);
}

#[test]
fn a_low_seat_is_a_floor_seat_whatever_its_shape() {
    let band = FurnitureParams::default().floor_seat_band_m;
    for (footprint, basis) in [([0.45, 0.45], Basis::Upright), ([0.45, 0.45], Basis::PlusZ), ([1.1, 0.4], Basis::LongSide)] {
        let low = with_basis(measured(footprint, 0.2, Facing::PosZ), basis);
        let d = def(Kind::Seat, &low);
        assert!(d.slots.iter().all(|s| s.clips.0 == "seat.floor" && (s.band - 0.2).abs() < EPS), "{footprint:?} {basis:?}");
    }
    // The rule is strict: a top exactly at the threshold is not a floor seat.
    assert_eq!(seat_clips(&measured([0.45, 0.45], band, Facing::PosZ)), ["seat.chair"]);
    assert_eq!(seat_clips(&with_basis(measured([0.45, 0.45], band, Facing::PosZ), Basis::PlusZ)), ["seat.stool"]);
    // The threshold is the parameter's.
    let p = FurnitureParams { floor_seat_band_m: 0.5, ..FurnitureParams::default() };
    let d = def_for(&RecordId::new("furn_x"), Kind::Seat, &measured([0.45, 0.45], 0.45, Facing::PosZ), &p).expect("a def");
    assert_eq!(d.slots[0].clips.0, "seat.floor");
}

#[test]
fn bed_edges_keep_their_family_however_low() {
    let d = def(Kind::Bed, &measured([0.7, 1.9], 0.12, Facing::PosZ));
    let clips: Vec<&str> = d.slots.iter().map(|s| s.clips.0.as_str()).collect();
    assert_eq!(clips, ["bed.lie", "seat.bed_edge", "seat.bed_edge"]);
}

// --- Class verbs (IN32c0) -------------------------------------------------------------------------

#[test]
fn containers_and_activators_keep_their_primary_verb_above_the_pose_verbs() {
    let open = class_verb(RecordClass::Container).expect("a container opens");
    assert_eq!((open.verb, open.priority), (TagId::of("open"), CLASS_VERB_PRIORITY));
    let used = class_verb(RecordClass::Activator).expect("an activator is used");
    assert_eq!((used.verb, used.priority), (TagId::of("use"), CLASS_VERB_PRIORITY));
    const { assert!(CLASS_VERB_PRIORITY > POSE_VERB_PRIORITY) };
    assert!(def(Kind::Seat, &measured([0.45, 0.45], 0.45, Facing::PosZ)).slots.iter().all(|s| s.verb.priority == POSE_VERB_PRIORITY));
    // Furniture's own rows and a creature (nothing unconditional offered) give none.
    assert_eq!(class_verb(RecordClass::Furniture(Kind::Seat)), None);
    assert_eq!(class_verb(RecordClass::Creature), None);
    assert!(def(Kind::Table, &measured([1.2, 0.8], 0.84, Facing::PosZ)).verbs.is_empty(), "def_for adds no class verb");
}

/// A placement id for `rank_verbs` (the crate has no `sf_world` dependency to name the type: its
/// text form, a hex `u128`, read back through serde).
fn placement<P: serde::de::DeserializeOwned>() -> P {
    use serde::de::IntoDeserializer as _;
    let hex: serde::de::value::StrDeserializer<'_, serde::de::value::Error> = "2a".into_deserializer();
    P::deserialize(hex).expect("a hex placement id")
}

/// `IN33c1`: the swing door's def, field by field.
#[test]
fn door_def_is_one_npc_doorway_and_the_toggle_verbs() {
    let id = RecordId::new("Door_Test");
    let doorway = Vec3::new(0.6, 1.1, 0.05);
    let d = door_def(&id, doorway);
    let want = InteractableDef {
        id: id.clone(),
        slots: vec![SlotDef {
            kind: SlotKind::Door(DoorDef {
                hinge: Vec3::ZERO,
                axis: Vec3::Y,
                open_deg: 90.0,
                both_ways: false,
                close_behind: true,
            }),
            local: Frame { translation: doorway, rotation: Quat::IDENTITY },
            band: 0.0,
            verb: VerbDef { verb: TagId::of(verb::PASS), priority: POSE_VERB_PRIORITY, requires: TagFilter::default() },
            entrances: SmallVec::new(),
            clips: ClipFamily(String::new()),
            ik: IkTargets::default(),
            reach: 0.0,
            occupancy: Occupancy::Pose,
            control: None,
            users: TagFilter { all: smallvec![TagId::of(USER_NPC)], any: SmallVec::new(), none: SmallVec::new() },
            group: None,
            queue: None,
        }],
        verbs: smallvec![
            VerbDef {
                verb: VERB_OPEN,
                priority: CLASS_VERB_PRIORITY,
                requires: TagFilter { all: SmallVec::new(), any: SmallVec::new(), none: smallvec![DOOR_OPEN] },
            },
            VerbDef {
                verb: VERB_CLOSE,
                priority: CLASS_VERB_PRIORITY,
                requires: TagFilter { all: smallvec![DOOR_OPEN], any: SmallVec::new(), none: SmallVec::new() },
            },
        ],
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    };
    assert_eq!(d, want);
    let (i, slot, _) = d.door().expect("a door slot");
    assert_eq!(i, 0);
    assert_eq!(slot.local.translation, doorway);
    assert!(slot.queues_by_default());
}

/// `IN33c1`: the player (refused the slot by its `users` filter) gets exactly the toggle verb of the
/// door's state; an NPC gets `open` first, then `pass` on the doorway slot.
#[test]
fn door_def_verbs_for_the_player_and_an_npc() {
    let d = door_def(&RecordId::new("Door_Test"), Vec3::new(0.6, 1.1, 0.0));
    let p = placement();
    let verbs = |tags: &[TagId], claim: bool| -> Vec<(TagId, VerbTarget)> {
        rank_verbs(&d, p, tags, |_| claim).iter().map(|c| (c.verb, c.target)).collect()
    };
    assert_eq!(verbs(&[], false), [(VERB_OPEN, VerbTarget::Slotless { placement: p, verb: 0 })]);
    assert_eq!(verbs(&[DOOR_OPEN], false), [(VERB_CLOSE, VerbTarget::Slotless { placement: p, verb: 1 })]);
    assert_eq!(
        verbs(&[], true),
        [
            (VERB_OPEN, VerbTarget::Slotless { placement: p, verb: 0 }),
            (TagId::of(verb::PASS), VerbTarget::Slot(SlotRef { placement: p, slot: 0 })),
        ]
    );
}

/// `IN33c1`: the door class's own verb is unchanged by the swing rows (the travel row's `open`).
#[test]
fn door_class_verb_is_still_open() {
    let open = class_verb(RecordClass::Door).expect("a door opens");
    assert_eq!((open.verb, open.priority), (TagId::of(verb::OPEN), CLASS_VERB_PRIORITY));
    assert!(open.requires.matches(&[]));
}

/// `IN3a`: a static bed (no class verb) gets the slotless `sleep` at the class rank, so its press is
/// Sleep and `lie` / the edge `sit`s follow on the hold list; a scripted `ACTI` or a `CONT` bed and
/// every other kind get none.
#[test]
fn static_beds_get_sleep_above_their_pose_verbs() {
    let sleep = VerbDef { verb: VERB_SLEEP, priority: CLASS_VERB_PRIORITY, requires: TagFilter::default() };
    assert_eq!(VERB_SLEEP, TagId::of(verb::SLEEP));
    assert_eq!(bed_verb(Kind::Bed, None), Some(sleep.clone()));
    assert_eq!(bed_verb(Kind::Bed, Some(RecordClass::Furniture(Kind::Bed))), Some(sleep.clone()));
    assert_eq!(bed_verb(Kind::Bed, Some(RecordClass::Activator)), None);
    assert_eq!(bed_verb(Kind::Bed, Some(RecordClass::Container)), None);
    for kind in [Kind::Seat, Kind::Table, Kind::Counter, Kind::Shelf] {
        assert_eq!(bed_verb(kind, None), None, "{kind:?}");
    }
    let mut d = def(Kind::Bed, &measured([1.0, 2.0], 0.74, Facing::PosZ));
    d.verbs.push(sleep);
    let p = placement();
    let ranked: Vec<TagId> = rank_verbs(&d, p, &[], |_| true).iter().map(|c| c.verb).collect();
    assert_eq!(ranked[0], VERB_SLEEP);
    assert_eq!(ranked.len(), 4);
    assert!(ranked[1..].iter().all(|v| *v == TagId::of(verb::LIE) || *v == TagId::of(verb::SIT)), "{ranked:?}");
}
