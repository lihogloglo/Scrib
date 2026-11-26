//! Approach tests (`IN2c4`): an offered seat the enter can't start from walks the body onto the
//! enter clip's start pose, turns it, and sits; every other end answers the offer once. Reuses the
//! seat harness ([`crate::tests_seat`]) with a stand-in mover: the body moves by the intent's
//! velocity x tick and turns toward `desired_facing` at [`TURN_RATE`] (for `sf_controller`).

use std::f32::consts::{FRAC_PI_2, PI};

use glam::Vec3;
use sf_anim::RootPose;

use super::*;
use crate::tests_seat::{ENTER, LOOP, Run, SLOT, WorldSeat, press, starts, stick, yaw_diff};

const APPROACH: &str = "interact.sit.approach";
/// The rule data's numbers (`seat_rules.ron`).
const ARRIVE_M: f32 = 0.05;
const ARRIVE_RAD: f32 = 0.09;
const TIMEOUT_S: f32 = 3.0;
/// The stand-in mover's turn rate, rad/s.
const TURN_RATE: f32 = 10.0;
/// The `SitDown` stand-in's whole root travel: 0.5 m forward, a half turn.
const TRAVEL: f32 = 0.5;

#[allow(clippy::cast_precision_loss)]
fn dt() -> f32 {
    1.0 / TICK_HZ as f32
}

/// A seat whose frame is at `at`, facing `yaw` (the way the sitter faces).
fn seat_at(at: Vec3, yaw: f32) -> WorldSeat {
    let front = Vec3::new(yaw.sin(), 0.0, yaw.cos());
    WorldSeat { enter: RootPose::new(at, yaw), exit: RootPose::new(at + front * 0.55, yaw) }
}

/// The enter clip's start pose for `seat` (world): `TRAVEL` behind the frame, facing away from it.
fn start_pose(seat: WorldSeat) -> RootPose {
    let yaw = seat.enter.yaw - PI;
    RootPose::new(seat.enter.position - Vec3::new(yaw.sin(), 0.0, yaw.cos()) * TRAVEL, yaw)
}

/// One tick, then the stand-in mover (when `mover`) where no warp or hold places the body.
fn tick(r: &mut Run, cmd: Command, mover: bool) {
    r.step(cmd);
    if !mover || r.out.warp.is_some() {
        return;
    }
    let i = r.out.intent;
    r.feet += i.desired_velocity * dt();
    if let Some(f) = i.desired_facing {
        r.yaw += yaw_diff(f, r.yaw).clamp(-TURN_RATE * dt(), TURN_RATE * dt());
    }
}

fn at(r: &Run, id: &str) -> bool {
    r.state.mov == RuleKey::of(id)
}

/// Offered `seat` from the origin facing `yaw`: the approach starts on the offer tick.
fn approaching(seat: WorldSeat, yaw: f32) -> Run {
    let mut r = Run::new(Some(seat));
    r.yaw = yaw;
    tick(&mut r, Command::default(), true);
    assert!(at(&r, APPROACH), "events {:?}, refusals {:?}", r.out.events, r.out.refusals);
    assert_eq!(r.refused(ENTER), Some(Refused::MaxCorrection), "the enter is tried first: {:?}", r.out.refusals);
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 0);
    r
}

fn refusals_of(r: &Run, id: &str) -> Vec<Refused> {
    r.out.refusals.iter().filter(|x| x.rule == RuleKey::of(id)).map(|x| x.reason).collect()
}

#[test]
fn an_approach_from_in_front_or_the_side_walks_turns_and_sits() {
    for (name, seat, yaw) in [
        ("0.9 m in front", seat_at(Vec3::new(0.0, 0.0, 0.9), PI), 0.0),
        ("1.2 m in front", seat_at(Vec3::new(0.0, 0.0, 1.2), PI), 0.0),
        ("1.5 m in front", seat_at(Vec3::new(0.0, 0.0, 1.5), PI), 0.0),
        ("1.2 m at its left", seat_at(Vec3::new(1.2, 0.0, 0.0), PI), FRAC_PI_2),
    ] {
        let mut r = approaching(seat, yaw);
        let goal = start_pose(seat);
        let mut ticks = 0;
        while at(&r, APPROACH) {
            let (feet, yaw) = (r.feet, r.yaw);
            tick(&mut r, Command::default(), true);
            ticks += 1;
            assert!(ticks < 180, "{name}: never arrived, feet {:?} yaw {}", r.feet, r.yaw);
            if at(&r, ENTER) {
                // The arrival tick: the body stood within tolerance of the start pose.
                let off = Vec3::new(feet.x - goal.position.x, 0.0, feet.z - goal.position.z).length();
                assert!(off <= ARRIVE_M + 1e-4, "{name}: arrived {off} m off");
                assert!(yaw_diff(yaw, goal.yaw).abs() <= ARRIVE_RAD + 1e-4, "{name}: arrived {} rad off", yaw_diff(yaw, goal.yaw));
            }
        }
        assert!(at(&r, ENTER), "{name}: {:?}, refusals {:?}", r.log, r.out.refusals);
        let ratio = r.state.warp.expect("the enter warp plans").plan.ratio();
        assert!(ratio < 0.25, "{name}: warp ratio {ratio}");
        r.until(LOOP, Command::default(), 70);
        assert_eq!(r.count(EventKind::SeatEntered(SLOT)), 1, "{name}: {:?}", r.log);
        assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 0, "{name}: {:?}", r.log);
        assert_eq!(starts(&r, APPROACH), 1, "{name}");
        assert_eq!(starts(&r, ENTER), 1, "{name}");
        assert!(r.state.approach.is_none(), "{name}");
    }
}

#[test]
fn on_the_start_pose_the_enter_starts_at_once() {
    let seat = seat_at(Vec3::new(0.0, 0.0, 1.2), PI);
    let goal = start_pose(seat);
    let mut r = Run::new(Some(seat));
    (r.feet, r.yaw) = (goal.position, goal.yaw);
    tick(&mut r, Command::default(), true);
    assert!(at(&r, ENTER), "{:?}", r.out.refusals);
    assert!(r.state.warp.expect("planned").plan.ratio() < 1e-3);
    r.until(LOOP, Command::default(), 70);
    assert_eq!(starts(&r, APPROACH), 0);
    assert_eq!(r.count(EventKind::SeatEntered(SLOT)), 1);
}

/// Walks the 1.5 m approach for `n` ticks (still approaching).
fn midway(n: usize) -> Run {
    let mut r = approaching(seat_at(Vec3::new(0.0, 0.0, 1.5), PI), 0.0);
    for _ in 0..n {
        tick(&mut r, Command::default(), true);
    }
    assert!(at(&r, APPROACH));
    assert!(r.feet.z > 0.1, "it walks: {:?}", r.feet);
    r
}

#[test]
fn the_stick_cancels_the_approach() {
    let mut r = midway(10);
    tick(&mut r, stick(1.0), true);
    assert!(at(&r, "loco"), "{:?}", r.out.events);
    assert_eq!(refusals_of(&r, APPROACH), vec![Refused::ApproachCancelled]);
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1, "{:?}", r.log);
    for _ in 0..30 {
        tick(&mut r, stick(1.0), true);
    }
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1);
    assert_eq!(starts(&r, ENTER), 0);
    assert!(r.state.approach.is_none());
}

/// A Jump cancels it and the press jumps (`air.jump` starts on the same tick: the approach accepts
/// every class and the Jump fires once it lapses).
#[test]
fn a_jump_cancels_the_approach_and_jumps() {
    let mut r = midway(10);
    tick(&mut r, press(ActionClass::Jump), true);
    assert!(at(&r, "air.jump"), "{:?}", r.out.events);
    assert_eq!(r.count(EventKind::Launch), 1);
    assert_eq!(refusals_of(&r, APPROACH), vec![Refused::ApproachCancelled]);
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1, "{:?}", r.log);
    assert_eq!(starts(&r, ENTER), 0);
}

#[test]
fn a_body_that_cannot_move_times_out() {
    let mut r = approaching(seat_at(Vec3::new(0.0, 0.0, 1.5), PI), 0.0);
    let feet = r.feet;
    let mut ticks = 0u32;
    while at(&r, APPROACH) {
        tick(&mut r, Command::default(), false);
        ticks += 1;
        assert!(ticks < 400, "never timed out");
        assert!(r.out.intent.desired_velocity.length() > 0.0 || !at(&r, APPROACH), "it keeps asking to walk");
    }
    #[allow(clippy::cast_precision_loss)]
    let secs = ticks as f32 * dt();
    assert!((secs - TIMEOUT_S).abs() <= 2.0 * dt(), "ended after {secs} s");
    assert!(at(&r, "loco"));
    assert_eq!(refusals_of(&r, APPROACH), vec![Refused::ApproachTimeout]);
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1, "{:?}", r.log);
    assert_eq!(starts(&r, ENTER), 0);
    assert_eq!(r.feet, feet, "the mover was off");
}

#[test]
fn an_offer_past_max_m_is_refused_on_the_offer_tick() {
    let mut r = Run::new(Some(seat_at(Vec3::new(0.0, 0.0, 4.0), PI)));
    tick(&mut r, Command::default(), true);
    assert!(at(&r, "loco"));
    assert_eq!(r.refused(ENTER), Some(Refused::MaxCorrection), "{:?}", r.out.refusals);
    assert_eq!(r.refused(APPROACH), Some(Refused::ApproachTooFar), "{:?}", r.out.refusals);
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1, "{:?}", r.log);
    for _ in 0..30 {
        tick(&mut r, Command::default(), true);
    }
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 1);
    assert_eq!(starts(&r, APPROACH), 0);
    assert_eq!(r.feet, Vec3::ZERO);
}

#[test]
fn losing_the_seat_mid_approach_goes_back_to_locomotion_quietly() {
    let mut r = midway(10);
    r.seat = None;
    tick(&mut r, Command::default(), true);
    assert!(at(&r, "loco"), "{:?}", r.out.events);
    assert!(refusals_of(&r, APPROACH).is_empty(), "{:?}", r.out.refusals);
    for _ in 0..30 {
        tick(&mut r, Command::default(), true);
    }
    assert_eq!(r.count(EventKind::SeatRefused(SLOT)), 0, "{:?}", r.log);
    assert_eq!(starts(&r, ENTER), 0);
    assert!(r.state.approach.is_none());
}

#[test]
fn arrived_needs_an_approach_owner_and_an_approach_needs_arrived() {
    let bad = r#"[(id: "x", kind: Interaction, category: Interact, slots: ["loco.move"], entry: (offered: true), exits: [(on: Arrived, to: Loco)])]"#;
    let errors = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, bad).build().expect_err("refused");
    assert!(errors.iter().any(|e| e.message.contains("Arrived")), "{errors:?}");
    let bad = r#"[(id: "x", kind: Interaction, category: Interact, slots: ["loco.move"], owner: Approach(arrive_m: 0.05, arrive_rad: 0.09, timeout_s: 3.0, max_m: 2.5), entry: (offered: true))]"#;
    let errors = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, bad).build().expect_err("refused");
    assert!(errors.iter().any(|e| e.message.contains("Approach")), "{errors:?}");
}
