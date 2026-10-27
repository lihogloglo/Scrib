//! Lock-on ownership tests (combat.md "C1c"): acquire within 15 m and the cone, break past 20 m
//! and on a mantle, re-offer on touchdown; release, line of sight and death. Logic: exact.

use glam::Vec3;
use sf_action::{LockRequest, LockSide, TargetId};
use sf_camera::{BreakReason, Candidate, LockParams, LockTimers};
use sf_combat::{LockEvent, LockInput, update_lock};
use sf_coords::WorldPos;

const DT: f32 = 1.0 / 60.0;
const DUMMY: TargetId = TargetId(7);

/// A candidate at `(x, z)` from the origin, visible.
fn at(id: TargetId, x: f32, z: f32) -> Candidate<TargetId> {
    Candidate { id, pos: WorldPos::ORIGIN.offset(Vec3::new(x, 0.0, z)), los: true }
}

/// The view looks down +Z (yaw 0), grounded, no traversal.
fn input(request: LockRequest, candidates: &[Candidate<TargetId>]) -> LockInput<'_> {
    LockInput { request, origin: WorldPos::ORIGIN, view_yaw: 0.0, candidates, traversing: false, grounded: true }
}

struct Locker {
    lock: Option<TargetId>,
    timers: LockTimers,
    p: LockParams,
}

impl Locker {
    fn new() -> Self {
        Self { lock: None, timers: LockTimers::default(), p: LockParams::default() }
    }
    fn tick(&mut self, s: &LockInput<'_>) -> LockEvent {
        update_lock(&mut self.lock, &mut self.timers, s, &self.p, DT)
    }
    fn press(&mut self, candidates: &[Candidate<TargetId>]) -> LockEvent {
        self.tick(&input(LockRequest::Toggle, candidates))
    }
}

#[test]
fn a_press_acquires_within_15_m_and_the_cone() {
    let mut l = Locker::new();
    assert_eq!(l.press(&[at(DUMMY, 0.0, 14.5)]), LockEvent::Acquired(DUMMY));
    assert_eq!(l.lock, Some(DUMMY));

    // Too far: no lock, the camera recentres.
    let mut l = Locker::new();
    assert_eq!(l.press(&[at(DUMMY, 0.0, 15.5)]), LockEvent::Recentre);
    assert_eq!(l.lock, None);

    // 10 m away but 60 degrees off the view (cone 35): no lock.
    let (s, c) = 60f32.to_radians().sin_cos();
    let mut l = Locker::new();
    assert_eq!(l.press(&[at(DUMMY, 10.0 * s, 10.0 * c)]), LockEvent::Recentre);

    // Behind a wall: no lock.
    let mut l = Locker::new();
    assert_eq!(l.press(&[Candidate { los: false, ..at(DUMMY, 0.0, 5.0) }]), LockEvent::Recentre);

    // Of two, the more centred wins over the slightly nearer one far off-axis.
    let off = TargetId(8);
    let (s, c) = 30f32.to_radians().sin_cos();
    let mut l = Locker::new();
    assert_eq!(l.press(&[at(off, 6.0 * s, 6.0 * c), at(DUMMY, 0.0, 7.0)]), LockEvent::Acquired(DUMMY));
}

#[test]
fn a_second_press_releases() {
    let mut l = Locker::new();
    let c = [at(DUMMY, 0.0, 5.0)];
    l.press(&c);
    assert_eq!(l.tick(&input(LockRequest::None, &c)), LockEvent::Kept);
    assert_eq!(l.press(&c), LockEvent::Released);
    assert_eq!(l.lock, None);
}

#[test]
fn the_lock_holds_to_20_m_and_breaks_past_it() {
    let mut l = Locker::new();
    l.press(&[at(DUMMY, 0.0, 10.0)]);
    // Past the acquire range but inside the break range (hysteresis), and out of the cone.
    assert_eq!(l.tick(&input(LockRequest::None, &[at(DUMMY, 12.0, 15.0)])), LockEvent::Kept);
    assert_eq!(l.tick(&input(LockRequest::None, &[at(DUMMY, 0.0, 19.9)])), LockEvent::Kept);
    assert_eq!(l.tick(&input(LockRequest::None, &[at(DUMMY, 0.0, 20.1)])), LockEvent::Broke(BreakReason::Range));
    assert_eq!(l.lock, None);
}

#[test]
fn the_lock_breaks_after_1_5_s_without_line_of_sight_or_when_the_target_dies() {
    let mut l = Locker::new();
    l.press(&[at(DUMMY, 0.0, 5.0)]);
    let hidden = [Candidate { los: false, ..at(DUMMY, 0.0, 5.0) }];
    let mut ticks = 0;
    while l.tick(&input(LockRequest::None, &hidden)) == LockEvent::Kept {
        ticks += 1;
        assert!(ticks < 200, "never broke");
    }
    assert_eq!(l.lock, None);
    assert!((89..=91).contains(&ticks), "1.5 s at 60 Hz, broke after {ticks} ticks");

    // Dead targets leave the candidate list.
    let mut l = Locker::new();
    l.press(&[at(DUMMY, 0.0, 5.0)]);
    assert_eq!(l.tick(&input(LockRequest::None, &[])), LockEvent::Broke(BreakReason::Lost));
}

#[test]
fn a_mantle_breaks_the_lock_and_touchdown_offers_it_back() {
    let mut l = Locker::new();
    let c = [at(DUMMY, 0.0, 6.0)];
    l.press(&c);
    let climbing = LockInput { traversing: true, grounded: false, ..input(LockRequest::None, &c) };
    assert_eq!(l.tick(&climbing), LockEvent::Broke(BreakReason::Traversal));
    assert_eq!(l.lock, None);
    // Still on the ledge's way up: no lock, and a press is ignored.
    assert_eq!(l.tick(&climbing), LockEvent::Kept);
    assert_eq!(l.tick(&LockInput { request: LockRequest::Toggle, ..climbing }), LockEvent::Kept);
    assert_eq!(l.lock, None);
    // Touchdown: the same target comes back without a press (even out of the cone).
    let behind = [at(DUMMY, 0.0, -6.0)];
    assert_eq!(l.tick(&input(LockRequest::None, &behind)), LockEvent::Reacquired(DUMMY));
    assert_eq!(l.lock, Some(DUMMY));
}

#[test]
fn the_offer_lapses_after_the_window() {
    let mut l = Locker::new();
    let c = [at(DUMMY, 0.0, 6.0)];
    l.press(&c);
    let climbing = LockInput { traversing: true, grounded: false, ..input(LockRequest::None, &c) };
    l.tick(&climbing);
    // Landed with the target out of sight for longer than the window: gone for good.
    let hidden = [Candidate { los: false, ..c[0] }];
    for _ in 0..70 {
        assert_eq!(l.tick(&input(LockRequest::None, &hidden)), LockEvent::Kept);
    }
    assert_eq!(l.tick(&input(LockRequest::None, &c)), LockEvent::Kept, "re-offered after the 1 s window");
    assert_eq!(l.lock, None);
}

#[test]
fn next_switches_to_the_side() {
    let mut l = Locker::new();
    let right = TargetId(9);
    // Screen right of +Z is -X (yaw grows counter-clockwise seen from above).
    let c = [at(DUMMY, 0.0, 6.0), at(right, -3.0, 6.0)];
    l.press(&c);
    assert_eq!(l.lock, Some(DUMMY));
    assert_eq!(l.tick(&input(LockRequest::Next(LockSide::Left), &c)), LockEvent::Kept, "nothing on the left");
    assert_eq!(l.tick(&input(LockRequest::Next(LockSide::Right), &c)), LockEvent::Switched(right));
}
