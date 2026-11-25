//! `AI8f1` search core: LKP first, look-arounds, widening seeded rings, reachability, ttl.

use std::collections::BTreeSet;

use glam::Vec3;
use sf_coords::WorldPos;
use sf_records::RecordId;

use crate::*;

const EPS: f32 = 1e-3;

fn at(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(x, y, z))
}

fn globals(profile: &PerceptionProfile) -> SearchParams {
    let none = |_: &RecordId| None::<&PerceptionState>;
    resolve(profile, &PerceptionStates::new(), none, &AwarenessConfig::default(), &SightConfig::default(), &SearchConfig::default(), 0)
        .search
}

fn start(lkp: WorldPos, confidence: f32, seed: u64) -> SearchState {
    SearchState::start(lkp, confidence, globals(&PerceptionProfile::default()), &SearchPatternConfig::default(), seed, 100).unwrap()
}

fn clock(tick: u64, hz: f32) -> PerceptClock {
    PerceptClock::new(tick, 1.0 / hz)
}

fn all(_: WorldPos) -> bool {
    true
}

fn planar(a: WorldPos, b: WorldPos) -> f32 {
    let d = a.delta_f32(b);
    d.x.hypot(d.z)
}

/// The `GoTo` points of `steps` calls, one call every `every` ticks from tick 100 at 30 Hz.
fn points(s: &mut SearchState, steps: u64, every: u64) -> Vec<(u64, WorldPos)> {
    let mut out = Vec::new();
    for i in 0..steps {
        let t = 100 + i * every;
        if let SearchGoal::GoTo(p) = s.next_goal(clock(t, 30.0), &mut all) {
            out.push((t, p));
        }
    }
    out
}

#[test]
fn first_goal_is_the_lkp_then_a_look_around() {
    let lkp = at(10.0, 2.0, -4.0);
    let mut s = start(lkp, 1.0, 7);
    assert_eq!(s.next_goal(clock(100, 30.0), &mut all), SearchGoal::GoTo(lkp));
    let SearchGoal::LookAround { yaw_from, yaw_to } = s.next_goal(clock(130, 30.0), &mut all) else {
        panic!("a look-around after arriving");
    };
    // The default half-arc is 60 degrees: a 120-degree sweep.
    let sweep = wrap_angle(yaw_to - yaw_from);
    assert!((sweep - 120f32.to_radians()).abs() < EPS, "sweep {sweep}");
    assert!(matches!(s.next_goal(clock(160, 30.0), &mut all), SearchGoal::GoTo(_)));
}

#[test]
fn goals_alternate_go_to_and_look_around_facing_outward() {
    let lkp = at(0.0, 0.0, 0.0);
    let mut s = start(lkp, 1.0, 3);
    s.next_goal(clock(100, 30.0), &mut all);
    s.next_goal(clock(101, 30.0), &mut all);
    for i in 0..10u64 {
        let SearchGoal::GoTo(p) = s.next_goal(clock(102 + 2 * i, 30.0), &mut all) else { panic!("GoTo") };
        let SearchGoal::LookAround { yaw_from, yaw_to } = s.next_goal(clock(103 + 2 * i, 30.0), &mut all) else {
            panic!("LookAround")
        };
        let center = wrap_angle(yaw_from + wrap_angle(yaw_to - yaw_from) / 2.0);
        let outward = yaw_toward(lkp, p).unwrap();
        assert!(wrap_angle(center - outward).abs() < EPS, "look {center} vs outward {outward}");
    }
}

#[test]
#[allow(clippy::cast_precision_loss)] // small tick counts
fn points_lie_within_the_current_radius_and_grow_with_time() {
    let lkp = at(5.0, 0.0, 5.0);
    let mut s = start(lkp, 1.0, 11);
    // One call per second for 29 s: the rings are drawn at growing radii.
    let pts = points(&mut s, 29, 30);
    assert!(pts.len() >= 12, "{} points", pts.len());
    #[allow(clippy::cast_precision_loss)] // small tick deltas in a test
    let r = |t: u64| s.radius_at((t - 100) as f32 / 30.0);
    // Globals: r(t) = 3 + 0.5 t at confidence 1.
    assert!((r(100) - 3.0).abs() < EPS);
    assert!((r(100 + 300) - 8.0).abs() < EPS);
    for &(t, p) in &pts[1..] {
        assert!(planar(lkp, p) <= r(t) + EPS, "point at {} > r {}", planar(lkp, p), r(t));
    }
    // Ring 1 (drawn on the third call, at 2 s) lies within r(2 s); later rings reach farther.
    let first_ring_max = pts[1..7].iter().map(|&(_, p)| planar(lkp, p)).fold(0.0, f32::max);
    let last_ring_max = pts[pts.len() - 6..].iter().map(|&(_, p)| planar(lkp, p)).fold(0.0, f32::max);
    assert!(first_ring_max <= r(160) + EPS);
    assert!(last_ring_max > r(160), "last ring {last_ring_max} vs r(2 s) {}", r(160));
    // Each ring point sits in [inner_frac, 1] of its ring radius, so never closer than half of r(2 s).
    for &(_, p) in &pts[1..7] {
        assert!(planar(lkp, p) >= 0.5 * r(160) - EPS);
    }
}

#[test]
fn low_confidence_widens_the_radius() {
    let s1 = start(WorldPos::ORIGIN, 1.0, 1);
    let s0 = start(WorldPos::ORIGIN, 0.0, 1);
    let sh = start(WorldPos::ORIGIN, 0.5, 1);
    assert!((s1.radius_at(4.0) - 5.0).abs() < EPS);
    // uncertain_mul 2: confidence 0 doubles, 0.5 is 1.5x; out-of-range confidence clamps.
    assert!((s0.radius_at(4.0) - 10.0).abs() < EPS);
    assert!((sh.radius_at(4.0) - 7.5).abs() < EPS);
    assert!((start(WorldPos::ORIGIN, 3.0, 1).radius_at(4.0) - 5.0).abs() < EPS);
    assert!((start(WorldPos::ORIGIN, -1.0, 1).radius_at(4.0) - 10.0).abs() < EPS);
}

#[test]
fn unreachable_candidates_are_skipped() {
    let lkp = WorldPos::ORIGIN;
    // Only the +X half-plane is reachable; the LKP itself (x = 0) is not.
    let mut half = |p: WorldPos| WorldPos::ORIGIN.delta_f32(p).x > 0.0;
    let mut s = start(lkp, 1.0, 5);
    let mut asked = 0;
    for i in 0..40u64 {
        match s.next_goal(clock(100 + i * 15, 30.0), &mut half) {
            SearchGoal::GoTo(p) => {
                asked += 1;
                assert!(WorldPos::ORIGIN.delta_f32(p).x > 0.0, "unreachable point issued");
                assert_ne!(p, lkp);
            }
            SearchGoal::LookAround { .. } => {}
            SearchGoal::Done => panic!("not yet"),
        }
    }
    assert!(asked > 5);

    // Nothing reachable: look-arounds in place, never a GoTo, until the ttl.
    let mut s = start(lkp, 1.0, 5);
    let mut calls = 0;
    let mut never = |_: WorldPos| {
        calls += 1;
        false
    };
    for i in 0..10 {
        assert!(matches!(s.next_goal(clock(100 + i, 30.0), &mut never), SearchGoal::LookAround { .. }));
    }
    // Bounded work: the LKP once, then at most two rings of 6 per call.
    assert!(calls <= 1 + 10 * 12, "{calls} reachability queries");
}

#[test]
fn same_seed_same_points_different_actor_different() {
    let lkp = at(1.0, 0.0, 1.0);
    let a = search_seed(ActorKey(42), 100);
    let b = search_seed(ActorKey(43), 100);
    let c = search_seed(ActorKey(42), 101);
    assert_eq!(a, search_seed(ActorKey(42), 100));
    assert_ne!(a, b);
    assert_ne!(a, c);
    let run = |seed| points(&mut start(lkp, 1.0, seed), 20, 30);
    assert_eq!(run(a), run(a));
    assert_ne!(run(a), run(b));
    assert_ne!(run(a), run(c));
    // Distinct points within a run.
    let set: BTreeSet<_> = run(a).iter().map(|(_, p)| p.to_bytes()).collect();
    assert_eq!(set.len(), run(a).len());
}

#[test]
fn done_exactly_at_ttl_at_30_and_144_hz() {
    for (hz, ttl_ticks) in [(30.0, 900u64), (144.0, 4320)] {
        let mut s = start(WorldPos::ORIGIN, 1.0, 9);
        assert_eq!(s.ttl_ticks(1.0 / hz), ttl_ticks);
        assert!(!matches!(s.next_goal(clock(100, hz), &mut all), SearchGoal::Done));
        assert!(!matches!(s.next_goal(clock(100 + ttl_ticks - 1, hz), &mut all), SearchGoal::Done));
        assert_eq!(s.next_goal(clock(100 + ttl_ticks, hz), &mut all), SearchGoal::Done);
        assert!(s.is_done());
        // Sticky: even an earlier clock stays Done.
        assert_eq!(s.next_goal(clock(100, hz), &mut all), SearchGoal::Done);
        // Same game time within one tick.
        #[allow(clippy::cast_precision_loss)]
        let secs = ttl_ticks as f32 / hz;
        assert!((secs - 30.0).abs() <= 1.0 / hz);
    }
    // A first call already past the ttl is Done.
    let mut s = start(WorldPos::ORIGIN, 1.0, 9);
    assert_eq!(s.next_goal(clock(100 + 5000, 30.0), &mut all), SearchGoal::Done);
}

#[test]
fn profile_search_ttl_2_doubles_the_duration() {
    let params = globals(&PerceptionProfile { search_ttl: 2.0, ..Default::default() });
    assert!((params.ttl_s - 60.0).abs() < EPS);
    let mut s = SearchState::start(WorldPos::ORIGIN, 1.0, params, &SearchPatternConfig::default(), 1, 0).unwrap();
    assert_eq!(s.ttl_ticks(1.0 / 30.0), 1800);
    assert!(!matches!(s.next_goal(clock(1799, 30.0), &mut all), SearchGoal::Done));
    assert_eq!(s.next_goal(clock(1800, 30.0), &mut all), SearchGoal::Done);
}

#[test]
fn zero_ttl_is_done_at_once() {
    let params = SearchParams { ttl_s: 0.0, error_m: 3.0, widen_mps: 0.5 };
    let mut s = SearchState::start(WorldPos::ORIGIN, 1.0, params, &SearchPatternConfig::default(), 1, 10).unwrap();
    assert_eq!(s.next_goal(clock(10, 30.0), &mut all), SearchGoal::Done);
}

#[test]
fn nan_and_negative_params_are_rejected() {
    let ok = SearchParams { ttl_s: 30.0, error_m: 3.0, widen_mps: 0.5 };
    let pat = SearchPatternConfig::default();
    let bad = [
        ("ttl_s", SearchParams { ttl_s: f32::NAN, ..ok }),
        ("ttl_s", SearchParams { ttl_s: -1.0, ..ok }),
        ("ttl_s", SearchParams { ttl_s: f32::INFINITY, ..ok }),
        ("error_m", SearchParams { error_m: -0.1, ..ok }),
        ("widen_mps", SearchParams { widen_mps: f32::NAN, ..ok }),
    ];
    for (field, p) in bad {
        let err = SearchState::start(WorldPos::ORIGIN, 1.0, p, &pat, 1, 0).unwrap_err();
        assert_eq!(err.field, field);
    }
    let err = SearchState::start(WorldPos::ORIGIN, f32::NAN, ok, &pat, 1, 0).unwrap_err();
    assert_eq!(err.field, "confidence");
    let err = SearchState::start(WorldPos::ORIGIN, 1.0, ok, &SearchPatternConfig { inner_frac: -0.5, ..pat }, 1, 0).unwrap_err();
    assert_eq!(err.field, "inner_frac");
    // Zero is a valid value everywhere; a zero ring count is clamped to one point.
    let zero = SearchParams { ttl_s: 10.0, error_m: 0.0, widen_mps: 0.0 };
    let mut s = SearchState::start(WorldPos::ORIGIN, 1.0, zero, &SearchPatternConfig { ring_points: 0, ..pat }, 1, 0).unwrap();
    s.next_goal(clock(0, 30.0), &mut all);
    s.next_goal(clock(1, 30.0), &mut all);
    assert_eq!(s.next_goal(clock(2, 30.0), &mut all), SearchGoal::GoTo(WorldPos::ORIGIN));
}

#[test]
fn goals_map_onto_the_move_operators() {
    let ws = sf_world::WorldspaceId(RecordId::new("test_ws"));
    let p = at(1.0, 0.0, 2.0);
    let [a, b] = SearchGoal::GoTo(p).operators(&ws, ExecGait::Walk, 1.5, 0.1);
    assert_eq!(a, Some(OperatorKind::MoveTo { goal: sf_world::Location::new(ws.clone(), p), gait: ExecGait::Walk, speed: 1.5 }));
    assert_eq!(b, None);
    let [a, b] = SearchGoal::LookAround { yaw_from: -1.0, yaw_to: 1.0 }.operators(&ws, ExecGait::Walk, 1.5, 0.1);
    assert_eq!(a, Some(OperatorKind::Face { target: FaceTarget::Yaw(-1.0), tolerance_rad: 0.1 }));
    assert_eq!(b, Some(OperatorKind::Face { target: FaceTarget::Yaw(1.0), tolerance_rad: 0.1 }));
    assert_eq!(SearchGoal::Done.operators(&ws, ExecGait::Walk, 1.5, 0.1), [None, None]);
    let mut ops = Operators::new();
    let id = register_search_operator(&mut ops);
    assert_eq!(ops.get(OP_SEARCH), Some(id));
}

#[test]
fn state_round_trips_through_serde() {
    let mut s = start(at(3.0, 0.0, 3.0), 0.7, 99);
    for i in 0..5 {
        s.next_goal(clock(100 + i * 20, 30.0), &mut all);
    }
    let json = serde_json::to_string(&s).unwrap();
    let mut back: SearchState = serde_json::from_str(&json).unwrap();
    assert_eq!(back, s);
    assert_eq!(back.next_goal(clock(300, 30.0), &mut all), s.next_goal(clock(300, 30.0), &mut all));
}
