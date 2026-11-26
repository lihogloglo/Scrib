//! The head tracking rule (`AN8b`, [`crate::head_track`]): exact cases on the pure functions.

use std::f32::consts::FRAC_PI_2;

use bevy_math::Vec3;
use sf_controller::CharacterFrame;
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{GameSetting, GameSettingValue};

use super::*;

/// Exterior range: 400 game units in meters (5.715 m).
fn range_m() -> f32 {
    mw_world::convert::meters(MAX_HEAD_TRACK_DISTANCE)
}

/// An NPC at `(x, 0, z)` facing yaw `yaw` (0: +Z), standing, outdoors, not engaged.
fn npc(x: f32, z: f32, yaw: f32) -> Tracker {
    Tracker {
        frame: CharacterFrame::new(WorldPos::from_f64(bevy_math::DVec3::new(f64::from(x), 0.0, f64::from(z))), yaw),
        knocked_down: false,
        interior: false,
        engaged: false,
    }
}

/// A seen, living, unhidden player at `(x, 0, z)`.
fn player(x: f32, z: f32) -> HeadCandidate {
    HeadCandidate {
        feet: WorldPos::from_f64(bevy_math::DVec3::new(f64::from(x), 0.0, f64::from(z))),
        dead: false,
        invisibility: 0.0,
        chameleon: 0.0,
        seen: true,
    }
}

fn rules() -> HeadTrackRules {
    HeadTrackRules::default()
}

fn head(c: &HeadCandidate) -> WorldPos {
    c.feet.offset(Vec3::Y * 1.6)
}

fn target(n: &Tracker, c: &HeadCandidate) -> Option<Vec3> {
    head_target(n, c, head(c), Vec3::ONE, &rules())
}

#[test]
fn in_range_and_in_front_targets_the_head() {
    let (n, p) = (npc(0.0, 0.0, 0.0), player(0.0, 3.0));
    let t = target(&n, &p).expect("tracks");
    assert!((t - Vec3::new(0.0, 1.6, 3.0)).length() < 1e-5, "{t:?}");
    // Just inside the exterior range, off to one side but in front.
    assert!(target(&n, &player(1.0, range_m() - 1.0)).is_some());
}

#[test]
fn out_of_range_is_none() {
    let n = npc(0.0, 0.0, 0.0);
    assert!(target(&n, &player(0.0, range_m() + 0.01)).is_none());
    // Range is 3D (feet to feet): a player above, horizontally in range.
    let mut above = player(0.0, 3.0);
    above.feet = above.feet.offset(Vec3::Y * 5.0);
    assert!(target(&n, &above).is_none());
}

#[test]
fn interiors_halve_the_range() {
    assert!((rules().max_distance(true) - range_m() * 0.5).abs() < 1e-6);
    let mut n = npc(0.0, 0.0, 0.0);
    let p = player(0.0, 4.0);
    assert!(target(&n, &p).is_some());
    n.interior = true;
    assert!(target(&n, &p).is_none());
    assert!(target(&n, &player(0.0, 2.5)).is_some());
}

#[test]
fn outside_the_front_half_is_none() {
    let n = npc(0.0, 0.0, 0.0);
    assert!(target(&n, &player(0.0, -2.0)).is_none(), "behind");
    assert!(target(&n, &player(2.0, 0.0)).is_none(), "exactly abeam (dot 0)");
    assert!(target(&n, &player(2.0, 0.05)).is_some(), "just in front of abeam");
    // Facing is the yaw: turned to face +X, a player on +Z is abeam (its left).
    assert!(target(&npc(0.0, 0.0, FRAC_PI_2), &player(0.0, 2.0)).is_none());
    // Height doesn't count for the half-space (horizontal test).
    let mut low = player(0.0, 0.5);
    low.feet = low.feet.offset(Vec3::Y * -3.0);
    assert!(target(&n, &low).is_some());
}

#[test]
fn knocked_down_tracker_is_none() {
    let mut n = npc(0.0, 0.0, 0.0);
    n.knocked_down = true;
    assert!(target(&n, &player(0.0, 3.0)).is_none());
    n.engaged = true;
    assert!(target(&n, &player(0.0, 3.0)).is_none(), "even in combat");
}

#[test]
fn dead_target_is_none() {
    let mut p = player(0.0, 3.0);
    p.dead = true;
    assert!(target(&npc(0.0, 0.0, 0.0), &p).is_none());
}

#[test]
fn magically_hidden_target_is_none() {
    let n = npc(0.0, 0.0, 0.0);
    let mut p = player(0.0, 3.0);
    p.invisibility = 1.0;
    assert!(target(&n, &p).is_none(), "invisible");
    p.invisibility = 0.0;
    p.chameleon = 75.0;
    assert!(target(&n, &p).is_none(), "chameleon 75");
    p.chameleon = 74.0;
    assert!(target(&n, &p).is_some(), "chameleon 74");
    let mut engaged = n;
    engaged.engaged = true;
    p.chameleon = 80.0;
    assert!(target(&engaged, &p).is_none(), "hidden even from a combat target");
}

#[test]
fn unseen_target_is_none() {
    // No LOS or not aware (sneaking unseen).
    let mut p = player(0.0, 3.0);
    p.seen = false;
    let mut n = npc(0.0, 0.0, 0.0);
    assert!(target(&n, &p).is_none());
    n.engaged = true;
    assert!(target(&n, &p).is_none(), "combat still needs LOS and awareness");
}

#[test]
fn engaged_skips_range_and_facing() {
    let mut n = npc(0.0, 0.0, 0.0);
    n.engaged = true;
    assert!(target(&n, &player(0.0, -20.0)).is_some(), "far and behind");
    n.interior = true;
    assert!(target(&n, &player(10.0, 0.0)).is_some());
}

#[test]
fn rotated_npc_sees_the_player_on_its_forward_axis() {
    // At (10, 0, 5), turned 90 degrees to face world +X; the player 3 m along +X.
    let n = npc(10.0, 5.0, FRAC_PI_2);
    let p = player(13.0, 5.0);
    let t = target(&n, &p).expect("in front");
    assert!((t - Vec3::new(0.0, 1.6, 3.0)).length() < 1e-4, "on +Z (forward) in character space: {t:?}");
    // A body drawn 1.25x tall: the unscaled skeleton space divides by the scale.
    let scaled = head_target(&n, &p, head(&p), Vec3::splat(1.25), &rules()).unwrap();
    assert!((scaled - Vec3::new(0.0, 1.28, 2.4)).length() < 1e-4, "{scaled:?}");
    // The player 2 m to its left (world -Z when facing +X): character +X is the left of +Z forward (right = forward x up).
    let side = to_character_space(&n.frame, Vec3::ONE, WorldPos::from_f64(bevy_math::DVec3::new(10.0, 0.0, 3.0)));
    assert!((side - Vec3::new(2.0, 0.0, 0.0)).length() < 1e-4, "{side:?}");
}

fn gmst(db: &mut RecordDb, src: sf_records::SourceId, id: &str, value: f32) {
    db.define(src, RecordId::new(id), GameSetting { id: id.into(), value: GameSettingValue::Float(value), ..Default::default() })
        .unwrap();
}

#[test]
fn rules_read_the_gmsts() {
    let mut db = RecordDb::new();
    db.register::<GameSetting>("GMST").unwrap();
    let src = db.add_source("test", &[]);
    assert_eq!(HeadTrackRules::from_records(&db), HeadTrackRules::default());
    gmst(&mut db, src, "fMaxHeadTrackDistance", 400.0);
    gmst(&mut db, src, "fInteriorHeadTrackMult", 0.5);
    assert_eq!(HeadTrackRules::from_records(&db), HeadTrackRules::default());
    let m = db.add_source("mod", &[src]);
    gmst(&mut db, m, "fMaxHeadTrackDistance", 640.0);
    gmst(&mut db, m, "fInteriorHeadTrackMult", 0.25);
    let r = HeadTrackRules::from_records(&db);
    assert!((r.max_distance_m - 640.0 * 0.9144 / 64.0).abs() < 1e-5, "{r:?}");
    assert!((r.max_distance(true) - 640.0 * 0.9144 / 64.0 * 0.25).abs() < 1e-5);
    assert!((range_m() - 5.715).abs() < 1e-4);
}

#[test]
fn settings_default_to_the_tunables() {
    let s = HeadTrackSettings::default();
    assert!(s.on);
    assert!((s.refresh_s - 0.3).abs() < 1e-6);
    assert!((s.rules.chameleon_hides - 75.0).abs() < 1e-6);
}
