//! `IN33a` swing door: toggle (also mid-swing), step (exact arrival, blocked pause), swing side,
//! leaf pose, close-behind schedule, defaults and save round trip.
#![allow(clippy::float_cmp)] // exact arrival on 0 / 1 is the contract

use std::f32::consts::FRAC_PI_2;

use glam::{Quat, Vec3};
use sf_ids::{ActorKey, PlayerKey};
use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::smallvec;

use crate::*;

const DT: f32 = 1.0 / 60.0;
const SWING: f32 = 0.8;
const DOOR: PlacementId = PlacementId(7);

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

fn player(n: u128) -> UserKey {
    UserKey::Player(PlayerKey(n))
}

/// Hinge at +0.5 x, axis Y, 90 degrees; the doorway (closed leaf centre) is the origin, so a
/// positive turn moves the leaf centre toward +z.
fn door_def(both_ways: bool, close_behind: bool) -> DoorDef {
    DoorDef { hinge: Vec3::new(0.5, 0.0, 0.0), axis: Vec3::Y, open_deg: 90.0, both_ways, close_behind }
}

const DOORWAY: Vec3 = Vec3::ZERO;
const FRONT: Vec3 = Vec3::new(0.0, 0.0, 1.0); // the +n side
const BACK: Vec3 = Vec3::new(0.0, 0.0, -1.0);

fn world_point(f: &Frame, local: Vec3) -> Vec3 {
    f.translation + f.rotation * local
}

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).length() < 1e-5
}

#[test]
fn toggle_from_closed_arrives_exactly_on_step_48() {
    let def = door_def(false, false);
    let mut s = DoorState::default();
    let ev = s.toggle(DOOR, &def, DOORWAY, Some((player(1), FRONT)), Tick(5));
    assert_eq!(ev, DoorChanged { placement: DOOR, open: true, user: Some(player(1)), tick: Tick(5) });
    assert!(s.open);
    assert_eq!(s.opener, Some(player(1)));
    assert!(s.is_moving());
    for i in 1..=47 {
        assert!(s.step(DT, SWING, false), "step {i} moves");
        assert!(s.t < 1.0, "step {i} has not arrived: t = {}", s.t);
        assert!(s.is_moving());
    }
    assert!(s.step(DT, SWING, false));
    assert_eq!(s.t, 1.0);
    assert!(!s.is_moving());
    assert!(!s.step(DT, SWING, false));
    assert_eq!(s.t, 1.0);
}

#[test]
fn toggle_mid_swing_reverses_and_keeps_the_sign() {
    let def = door_def(true, false);
    let mut s = DoorState::default();
    s.toggle(DOOR, &def, DOORWAY, Some((actor(1), FRONT)), Tick(0));
    assert_eq!(s.sign, -1);
    for _ in 0..24 {
        s.step(DT, SWING, false);
    }
    let mid = s.t;
    assert!(mid > 0.0 && mid < 1.0);
    let ev = s.toggle(DOOR, &def, DOORWAY, Some((actor(2), BACK)), Tick(24));
    assert_eq!(ev, DoorChanged { placement: DOOR, open: false, user: Some(actor(2)), tick: Tick(24) });
    assert!(!s.open);
    assert_eq!(s.sign, -1);
    assert_eq!(s.t, mid);
    let mut prev = s.t;
    let mut steps = 0;
    while s.step(DT, SWING, false) {
        assert!(s.t < prev);
        prev = s.t;
        steps += 1;
        assert!(steps <= 30);
    }
    assert_eq!(s.t, 0.0);
    assert!(!s.is_moving());
}

#[test]
fn blocked_pauses_and_resumes_from_the_same_t() {
    let def = door_def(false, false);
    let mut s = DoorState::default();
    s.toggle(DOOR, &def, DOORWAY, None, Tick(0));
    for _ in 0..10 {
        s.step(DT, SWING, false);
    }
    let t = s.t;
    let before = s;
    assert!(!s.step(DT, SWING, true));
    assert_eq!(s, before);
    assert!(s.open);
    assert_eq!(s.stepped(DT, SWING).t, t + DT / SWING);
    assert!(s.step(DT, SWING, false));
    assert_eq!(s.t, t + DT / SWING);
    // `stepped` does not commit; `swing_s <= 0` arrives at once.
    assert_eq!(s.stepped(DT, 0.0).t, 1.0);
    assert_eq!(s.t, t + DT / SWING);
}

#[test]
fn swing_sign_turns_away_from_the_user() {
    let both = door_def(true, false);
    let one = door_def(false, false);
    assert_eq!(swing_sign(&both, DOORWAY, Some(FRONT)), -1);
    assert_eq!(swing_sign(&both, DOORWAY, Some(BACK)), 1);
    assert_eq!(swing_sign(&one, DOORWAY, Some(FRONT)), 1);
    assert_eq!(swing_sign(&one, DOORWAY, Some(BACK)), 1);
    assert_eq!(swing_sign(&both, DOORWAY, None), 1);
    // Doorway on the axis: degenerate, +1.
    assert_eq!(swing_sign(&both, both.hinge, Some(FRONT)), 1);

    // The leaf centre moves away from the user on either side.
    let object = Frame::default();
    for user in [FRONT, BACK] {
        let mut s = DoorState::default();
        s.toggle(DOOR, &both, DOORWAY, Some((player(1), user)), Tick(0));
        s.t = 1.0;
        let moved = world_point(&s.leaf(&both, &object), DOORWAY);
        assert!(moved.distance(user) > DOORWAY.distance(user), "user {user}: leaf centre at {moved}");
    }

    // A toggle from fully open, or opening again mid-swing, keeps the sign.
    let mut s = DoorState::resting(true, -1);
    s.toggle(DOOR, &both, DOORWAY, Some((player(1), BACK)), Tick(0));
    assert_eq!(s.sign, -1);
    s.step(DT, SWING, false);
    assert!(s.t > 0.0 && s.t < 1.0);
    s.toggle(DOOR, &both, DOORWAY, Some((player(1), BACK)), Tick(1));
    assert!(s.open);
    assert_eq!(s.sign, -1);
}

#[test]
fn leaf_turns_about_the_hinge_in_world_space() {
    let def = door_def(false, false);
    let object = Frame { translation: Vec3::new(10.0, 0.0, 5.0), rotation: Quat::from_rotation_y(FRAC_PI_2) };
    assert_eq!(DoorState::default().leaf(&def, &object), object);
    let hinge_world = world_point(&object, def.hinge);
    for t in [0.0, 0.5, 1.0] {
        let s = DoorState { t, ..DoorState::resting(true, 1) };
        assert!(close(world_point(&s.leaf(&def, &object), def.hinge), hinge_world), "t = {t}");
    }
    // Local (-0.5, 0, 0) from the hinge turns +90 about Y to (0, 0, 0.5): (0.5, 0, 0.5) in the
    // object frame, which the object's own +90 about Y maps to (0.5, 0, -0.5), plus (10, 0, 5).
    let open = DoorState::resting(true, 1);
    let at = world_point(&open.leaf(&def, &object), DOORWAY);
    assert!(close(at, Vec3::new(10.5, 0.0, 4.5)), "doorway at {at}");
    // smoothstep: half the time is half the angle, a quarter of the time less than a quarter.
    let half = DoorState { t: 0.5, ..open };
    assert!((half.angle_rad(&def) - FRAC_PI_2 / 2.0).abs() < 1e-6);
    let quarter = DoorState { t: 0.25, ..open };
    assert!(quarter.angle_rad(&def) < FRAC_PI_2 / 4.0);
}

#[test]
fn close_behind_fires_once_for_the_opener_only() {
    let def = door_def(false, true);
    let mut s = DoorState::default();
    s.toggle(DOOR, &def, DOORWAY, Some((player(1), FRONT)), Tick(10));
    s.released(player(1), &def, Tick(100), 60);
    assert_eq!(s.close_at, Some(Tick(160)));
    assert_eq!(s.opener, None);
    assert_eq!(s.poll(DOOR, &def, DOORWAY, Tick(159)), None);
    assert_eq!(
        s.poll(DOOR, &def, DOORWAY, Tick(160)),
        Some(DoorChanged { placement: DOOR, open: false, user: None, tick: Tick(160) })
    );
    assert!(!s.open);
    assert_eq!(s.poll(DOOR, &def, DOORWAY, Tick(161)), None);

    // Another user's release schedules nothing.
    let mut s = DoorState::default();
    s.toggle(DOOR, &def, DOORWAY, Some((player(1), FRONT)), Tick(10));
    s.released(actor(2), &def, Tick(100), 60);
    assert_eq!(s.close_at, None);
    assert_eq!(s.opener, Some(player(1)));

    // A one-way-and-stays door schedules nothing.
    let stays = door_def(false, false);
    let mut s = DoorState::default();
    s.toggle(DOOR, &stays, DOORWAY, Some((player(1), FRONT)), Tick(10));
    s.released(player(1), &stays, Tick(100), 60);
    assert_eq!(s.close_at, None);

    // A user who found it open did not open it.
    let mut s = DoorState::resting(true, 1);
    s.released(player(1), &def, Tick(100), 60);
    assert_eq!(s.close_at, None);

    // A manual toggle before the due tick cancels the schedule.
    let mut s = DoorState::default();
    s.toggle(DOOR, &def, DOORWAY, Some((player(1), FRONT)), Tick(10));
    s.released(player(1), &def, Tick(100), 60);
    s.toggle(DOOR, &def, DOORWAY, Some((actor(3), BACK)), Tick(130));
    assert_eq!(s.close_at, None);
    s.toggle(DOOR, &def, DOORWAY, Some((actor(3), BACK)), Tick(140));
    assert_eq!(s.poll(DOOR, &def, DOORWAY, Tick(160)), None);
    assert!(s.open);
}

#[test]
fn defaults_tag_and_save_round_trip() {
    let p = InteractParams::default();
    assert_eq!(p.door_swing_s, 0.8);
    assert_eq!(p.door_close_behind_s, 1.0);
    assert_eq!(p.tick_hz, 60.0);
    assert_eq!(p.door_close_behind_ticks(), 60);
    assert_eq!(DOOR_OPEN, TagId::of("door.open"));
    assert_eq!(DoorState::default(), DoorState { open: false, t: 0.0, sign: 1, opener: None, close_at: None });

    // `opener` stays None: ron (no `integer128` feature here) cannot write the u128 user keys.
    let s = DoorState { open: true, t: 0.25, sign: -1, opener: None, close_at: Some(Tick(99)) };
    let text = ron::to_string(&s).unwrap();
    assert_eq!(ron::from_str::<DoorState>(&text).unwrap(), s);
}

#[test]
fn def_finds_its_first_door_slot() {
    let slot = |kind: SlotKind| SlotDef {
        kind,
        local: Frame { translation: Vec3::new(0.0, 1.0, 0.0), rotation: Quat::IDENTITY },
        band: 0.0,
        verb: VerbDef { verb: TagId::of("open"), priority: 0, requires: TagFilter::default() },
        entrances: smallvec![],
        clips: ClipFamily("door".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    let mut def = InteractableDef {
        id: RecordId::new("door"),
        slots: vec![slot(SlotKind::Light), slot(SlotKind::Door(door_def(true, true)))],
        verbs: smallvec![],
        destructible: None,
        carry: None,
        adverts: vec![],
    };
    let (i, s, d) = def.door().unwrap();
    assert_eq!(i, 1);
    assert_eq!(s.local.translation, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(*d, door_def(true, true));
    def.slots.truncate(1);
    assert!(def.door().is_none());
}
