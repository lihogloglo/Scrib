//! `AI8h1` band reactions core: Curious faces, Suspicious steps toward the LKP, double take, hold,
//! bands owned elsewhere, spent placements.

use glam::Vec3;
use sf_coords::WorldPos;
use sf_ids::PlayerKey;
use sf_world::WorldspaceId;

use crate::*;

const EPS: f32 = 1e-4;
const PLAYER: PerceptTarget = PerceptTarget::Player(PlayerKey(7));
/// 10 Hz: the default 0.3 s double take is 3 ticks, the 2 s hold 20 ticks.
const TICK_S: f32 = 0.1;

fn at(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(x, y, z))
}

fn clock(tick: u64) -> PerceptClock {
    PerceptClock::new(tick, TICK_S)
}

fn entry(band: AwarenessBand, lkp: WorldPos, seen_tick: u64) -> ReactInput {
    ReactInput {
        target: PLAYER,
        band,
        last_seen: LastSeen { pos: lkp, tick: seen_tick, velocity: Vec3::ZERO, confidence: 1.0 },
    }
}

fn near(a: WorldPos, b: WorldPos) -> bool {
    a.delta_f32(b).length() < EPS
}

/// Runs `band` at `lkp` from tick 0 through the default double take; returns the first goal.
fn react(state: &mut ReactState, cfg: &ReactConfig, band: AwarenessBand, lkp: WorldPos, actor: WorldPos) -> ReactGoal {
    for t in 0..3 {
        assert_eq!(state.update(cfg, Some(entry(band, lkp, 0)), actor, clock(t)), ReactGoal::None, "double take at tick {t}");
    }
    state.update(cfg, Some(entry(band, lkp, 0)), actor, clock(3))
}

#[test]
fn unaware_and_no_entry_give_none() {
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    assert_eq!(s.update(&cfg, None, WorldPos::ORIGIN, clock(10)), ReactGoal::None);
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Unaware, at(5.0, 0.0, 0.0), 0)), WorldPos::ORIGIN, clock(10)), ReactGoal::None);
    assert_eq!(s.goal(), ReactGoal::None);
    assert_eq!(s.revision(), 0);
}

#[test]
fn curious_faces_the_lkp() {
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    let lkp = at(4.0, 0.0, -3.0);
    assert_eq!(react(&mut s, &cfg, AwarenessBand::Curious, lkp, WorldPos::ORIGIN), ReactGoal::Face { toward: lkp });
    assert_eq!(s.goal(), ReactGoal::Face { toward: lkp });
    assert_eq!(s.revision(), 1);
}

#[test]
fn face_delay_is_respected_and_zero_delay_is_immediate() {
    let cfg = ReactConfig { face_delay_s: 0.5, ..ReactConfig::default() };
    let mut s = ReactState::new();
    let lkp = at(0.0, 0.0, 5.0);
    for t in 10..15 {
        assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, lkp, 10)), WorldPos::ORIGIN, clock(t)), ReactGoal::None, "tick {t}");
    }
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, lkp, 10)), WorldPos::ORIGIN, clock(15)), ReactGoal::Face { toward: lkp });

    let zero = ReactConfig { face_delay_s: 0.0, ..ReactConfig::default() };
    let mut s = ReactState::new();
    assert_eq!(s.update(&zero, Some(entry(AwarenessBand::Curious, lkp, 0)), WorldPos::ORIGIN, clock(0)), ReactGoal::Face { toward: lkp });

    // Back to idle re-arms the double take.
    let mut s = ReactState::new();
    react(&mut s, &cfg, AwarenessBand::Curious, lkp, WorldPos::ORIGIN);
    s.update(&cfg, Some(entry(AwarenessBand::Unaware, lkp, 0)), WorldPos::ORIGIN, clock(20));
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, lkp, 21)), WorldPos::ORIGIN, clock(21)), ReactGoal::None);
}

#[test]
fn suspicious_steps_exactly_step_m_toward_the_lkp() {
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    let actor = at(10.0, 0.0, 10.0);
    let lkp = at(16.0, 0.0, 18.0); // 10 m away, direction (0.6, 0, 0.8)
    let ReactGoal::Investigate { to, then_face } = react(&mut s, &cfg, AwarenessBand::Suspicious, lkp, actor) else {
        panic!("expected Investigate");
    };
    assert_eq!(then_face, lkp);
    assert!(near(to, at(11.8, 0.0, 12.4)), "{to:?}");
    assert!((actor.delta_f32(to).length() - cfg.step_m).abs() < EPS);
}

#[test]
fn investigate_point_clamps_to_the_lkp_and_follows_the_slope() {
    let actor = WorldPos::ORIGIN;
    let close = at(1.0, 0.5, 1.0);
    assert_eq!(investigate_point(actor, close, 3.0), close);
    let exactly = at(0.0, 0.0, 3.0);
    assert_eq!(investigate_point(actor, exactly, 3.0), exactly);
    // Planar step 3 of a planar 10, height on the same line.
    assert!(near(investigate_point(actor, at(0.0, 4.0, 10.0), 3.0), at(0.0, 1.2, 3.0)));
    // Directly above or below: the LKP itself.
    let above = at(0.0, 6.0, 0.0);
    assert_eq!(investigate_point(actor, above, 3.0), above);
    // Bad step: no step (face from where it stands).
    assert_eq!(investigate_point(actor, at(0.0, 0.0, 10.0), f32::NAN), actor);
}

#[test]
fn hold_keeps_the_goal_for_small_moves_and_retargets_on_large_ones() {
    let cfg = ReactConfig::default(); // retarget 1.5 m, hold 2 s = 20 ticks
    let mut s = ReactState::new();
    let actor = WorldPos::ORIGIN;
    let lkp = at(0.0, 0.0, 10.0);
    let first = react(&mut s, &cfg, AwarenessBand::Suspicious, lkp, actor); // issued at tick 3
    assert_eq!(s.revision(), 1);

    // 1 m move within the hold: same goal.
    let small = at(1.0, 0.0, 10.0);
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Suspicious, small, 5)), actor, clock(5)), first);
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Suspicious, small, 5)), actor, clock(22)), first);
    assert_eq!(s.revision(), 1);

    // The same small move once the hold ran out (tick 23 = issued + 20): re-target.
    let held = s.update(&cfg, Some(entry(AwarenessBand::Suspicious, small, 5)), actor, clock(23));
    assert_eq!(held, ReactGoal::Investigate { to: investigate_point(actor, small, cfg.step_m), then_face: small });
    assert_eq!(s.revision(), 2);

    // A 2 m move inside the new hold: re-target at once.
    let big = at(3.0, 0.0, 10.0);
    let moved = s.update(&cfg, Some(entry(AwarenessBand::Suspicious, big, 24)), actor, clock(24));
    assert_eq!(moved, ReactGoal::Investigate { to: investigate_point(actor, big, cfg.step_m), then_face: big });
    assert_eq!(s.revision(), 3);

    // Unmoved LKP long after the hold, actor already at the step point: no creep.
    let ReactGoal::Investigate { to, .. } = moved else { panic!("expected Investigate") };
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Suspicious, big, 24)), to, clock(200)), moved);
    assert_eq!(s.revision(), 3);
}

#[test]
fn curious_hold_works_the_same() {
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    let lkp = at(0.0, 0.0, 10.0);
    react(&mut s, &cfg, AwarenessBand::Curious, lkp, WorldPos::ORIGIN);
    let small = at(1.4, 0.0, 10.0);
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, small, 4)), WorldPos::ORIGIN, clock(4)), ReactGoal::Face { toward: lkp });
    let big = at(1.6, 0.0, 10.0);
    assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, big, 5)), WorldPos::ORIGIN, clock(5)), ReactGoal::Face { toward: big });
}

#[test]
fn escalating_or_switching_target_retargets_without_a_new_delay() {
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    let lkp = at(0.0, 0.0, 10.0);
    react(&mut s, &cfg, AwarenessBand::Curious, lkp, WorldPos::ORIGIN);
    let up = s.update(&cfg, Some(entry(AwarenessBand::Suspicious, lkp, 0)), WorldPos::ORIGIN, clock(4));
    assert_eq!(up, ReactGoal::Investigate { to: at(0.0, 0.0, 3.0), then_face: lkp });
    assert_eq!(s.revision(), 2);

    let other = Some(ReactInput { target: PerceptTarget::Player(PlayerKey(8)), ..entry(AwarenessBand::Suspicious, lkp, 0) });
    s.update(&cfg, other, WorldPos::ORIGIN, clock(5));
    assert_eq!(s.revision(), 3);
}

#[test]
fn searching_and_alerted_give_none_and_spend_the_placement() {
    let cfg = ReactConfig::default();
    let lkp = at(0.0, 0.0, 10.0);
    for band in [AwarenessBand::Searching, AwarenessBand::Alerted] {
        let mut s = ReactState::new();
        react(&mut s, &cfg, AwarenessBand::Suspicious, lkp, WorldPos::ORIGIN);
        assert_eq!(s.update(&cfg, Some(entry(band, lkp, 0)), WorldPos::ORIGIN, clock(10)), ReactGoal::None, "{band:?}");
        assert_eq!(s.goal(), ReactGoal::None);

        // Decaying back down with the same placement: back to routine.
        for (t, down) in [(50, AwarenessBand::Suspicious), (80, AwarenessBand::Curious)] {
            assert_eq!(s.update(&cfg, Some(entry(down, lkp, 0)), WorldPos::ORIGIN, clock(t)), ReactGoal::None);
        }
        assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, lkp, 0)), WorldPos::ORIGIN, clock(85)), ReactGoal::None);

        // A newer placement re-arms it (after the double take).
        let fresh = at(2.0, 0.0, 10.0);
        assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, fresh, 90)), WorldPos::ORIGIN, clock(90)), ReactGoal::None);
        assert_eq!(s.update(&cfg, Some(entry(AwarenessBand::Curious, fresh, 90)), WorldPos::ORIGIN, clock(93)), ReactGoal::Face { toward: fresh });
    }
}

#[test]
fn goals_map_to_the_move_executors() {
    let ws = WorldspaceId::new("test");
    let lkp = at(0.0, 0.0, 10.0);
    let to = at(0.0, 0.0, 3.0);
    assert_eq!(ReactGoal::None.operators(&ws, ExecGait::Walk, 1.5, 0.1), [None, None]);
    let face = OperatorKind::Face { target: FaceTarget::Point(lkp), tolerance_rad: 0.1 };
    assert_eq!(ReactGoal::Face { toward: lkp }.operators(&ws, ExecGait::Walk, 1.5, 0.1), [Some(face.clone()), None]);
    let walk = OperatorKind::MoveTo { goal: sf_world::Location::new(ws.clone(), to), gait: ExecGait::Walk, speed: 1.5 };
    assert_eq!(ReactGoal::Investigate { to, then_face: lkp }.operators(&ws, ExecGait::Walk, 1.5, 0.1), [Some(walk), Some(face)]);
}

#[test]
fn reads_the_meters_strongest_entry() {
    let acfg = AwarenessConfig::default();
    let mut meter = AwarenessMeter::new();
    let lkp = at(0.0, 0.0, 8.0);
    let seen = LastSeen { pos: lkp, tick: 0, velocity: Vec3::ZERO, confidence: 1.0 };
    meter.raise_to_floor(&acfg, PLAYER, AwarenessBand::Suspicious, seen, clock(0));
    let cfg = ReactConfig::default();
    let mut s = ReactState::new();
    let mut goal = ReactGoal::None;
    for t in 0..=3 {
        goal = s.update(&cfg, meter.strongest().map(ReactInput::from), WorldPos::ORIGIN, clock(t));
    }
    assert_eq!(goal, ReactGoal::Investigate { to: at(0.0, 0.0, 3.0), then_face: lkp });
}
