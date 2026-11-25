//! `AI8b1` sight maths: gain factors, profile scaling, darkness, stagger, ray queue, cache.

use glam::Vec3;
use sf_coords::WorldPos;
use sf_ids::PlayerKey;
use sf_records::RecordId;

use crate::*;

const EPS: f32 = 1e-5;

fn at(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(x, y, z))
}

fn resolved(profile: &PerceptionProfile) -> SightParams {
    let none = |_: &RecordId| None::<&PerceptionState>;
    resolve(profile, &PerceptionStates::new(), none, &AwarenessConfig::default(), &SightConfig::default(), &SearchConfig::default(), 0)
        .sight
}

fn globals() -> SightParams {
    resolved(&PerceptionProfile::default())
}

/// Observer at the origin facing +X; the target at `d` meters along `angle_deg` from the facing.
fn check(d: f32, angle_deg: f32, seen_before: bool) -> SightCheck {
    let a = angle_deg.to_radians();
    SightCheck {
        observer: WorldPos::ORIGIN,
        facing: Vec3::X,
        target: at(d * a.cos(), d * a.sin(), 0.0),
        seen_before,
        visibility: 1.0,
        alertness: 1.0,
    }
}

fn gain(params: &SightParams, c: &SightCheck, dt: f32) -> Option<f32> {
    sight_gain(params, &SightConfig::default(), &SightBudget::default(), c, dt)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= EPS * a.abs().max(b.abs()).max(1.0)
}

#[test]
fn gain_is_full_to_near_then_linear_to_range() {
    let p = globals();
    // Defaults: rate 2/s, near 3 m, range 30 m.
    assert!(close(gain(&p, &check(1.0, 0.0, false), 1.0).unwrap(), 2.0));
    assert!(close(gain(&p, &check(3.0, 0.0, false), 1.0).unwrap(), 2.0));
    assert!(close(gain(&p, &check(16.5, 0.0, false), 1.0).unwrap(), 1.0)); // (30 - 16.5) / 27
    assert!(close(gain(&p, &check(29.0, 0.0, false), 0.5).unwrap(), 2.0 * (1.0 / 27.0) * 0.5));
    assert_eq!(gain(&p, &check(30.0, 0.0, false), 1.0), None);
    assert_eq!(gain(&p, &check(31.0, 0.0, false), 1.0), None);
}

#[test]
fn a_seen_target_is_kept_to_the_lose_range() {
    let p = globals();
    assert_eq!(gain(&p, &check(33.0, 0.0, false), 1.0), None);
    // lose 36 m: (36 - 33) / (36 - 3).
    assert!(close(gain(&p, &check(33.0, 0.0, true), 1.0).unwrap(), 2.0 * 3.0 / 33.0));
    assert_eq!(gain(&p, &check(36.0, 0.0, true), 1.0), None);
}

#[test]
fn cone_central_peripheral_behind() {
    let p = globals();
    // Central 30°, peripheral 80° at 0.35, at near range.
    assert!(close(gain(&p, &check(3.0, 0.0, false), 1.0).unwrap(), 2.0));
    assert!(close(gain(&p, &check(3.0, 25.0, false), 1.0).unwrap(), 2.0));
    assert!(close(gain(&p, &check(3.0, -50.0, false), 1.0).unwrap(), 2.0 * 0.35));
    assert!(close(gain(&p, &check(3.0, 75.0, false), 1.0).unwrap(), 2.0 * 0.35));
    assert_eq!(gain(&p, &check(3.0, 100.0, false), 1.0), None);
    assert_eq!(gain(&p, &check(3.0, 180.0, false), 1.0), None);
    // A zero facing sees nothing; a target at the eye counts as central.
    let mut c = check(3.0, 0.0, false);
    c.facing = Vec3::ZERO;
    assert_eq!(gain(&p, &c, 1.0), None);
    let mut c = check(0.0, 0.0, false);
    c.facing = Vec3::NEG_X;
    assert!(close(gain(&p, &c, 1.0).unwrap(), 2.0));
}

#[test]
fn profile_scaling_flows_through() {
    let base = globals();
    let far = PerceptionProfile { sight_range: 2.0, ..Default::default() };
    let p = resolved(&far);
    // Range 60 m, near unscaled: 45 m is out for the globals, in for the profile.
    assert_eq!(gain(&base, &check(45.0, 0.0, false), 1.0), None);
    assert!(close(gain(&p, &check(45.0, 0.0, false), 1.0).unwrap(), 2.0 * 15.0 / 57.0));
    assert!(close(gain(&p, &check(3.0, 0.0, false), 1.0).unwrap(), 2.0));

    let keen = resolved(&PerceptionProfile { sight_gain: 2.0, ..Default::default() });
    let alert = resolved(&PerceptionProfile { alertness: 2.0, ..Default::default() });
    let c = check(16.5, 0.0, false);
    let g = gain(&base, &c, 1.0).unwrap();
    assert!(close(gain(&keen, &c, 1.0).unwrap(), 2.0 * g));
    assert!(close(gain(&alert, &c, 1.0).unwrap(), 2.0 * g));
    // The meter's alertness and the visibility multiply on top.
    let mut c2 = c;
    c2.alertness = 1.5;
    c2.visibility = 0.5;
    assert!(close(gain(&keen, &c2, 1.0).unwrap(), 2.0 * g * 1.5 * 0.5));
    // Cone scaling: 0.5 halves both half-angles (15° / 40°).
    let narrow = resolved(&PerceptionProfile { sight_cone: 0.5, ..Default::default() });
    assert!(close(gain(&narrow, &check(3.0, 25.0, false), 1.0).unwrap(), 2.0 * 0.35));
    assert_eq!(gain(&narrow, &check(3.0, 50.0, false), 1.0), None);
}

#[test]
fn below_skip_gain_is_skipped_whatever_dt() {
    let p = globals();
    let mut c = check(3.0, 0.0, false);
    // skip_gain 0.01/s: rate 2 × 0.004 = 0.008 is skipped, 2 × 0.006 = 0.012 is not.
    c.visibility = 0.004;
    assert_eq!(gain(&p, &c, 0.1), None);
    assert_eq!(gain(&p, &c, 10.0), None);
    c.visibility = 0.006;
    assert!(close(gain(&p, &c, 0.1).unwrap(), 0.0012));
    c.visibility = 0.0;
    assert_eq!(gain(&p, &c, 1.0), None);
    assert_eq!(gain(&p, &check(3.0, 0.0, false), -1.0), None);
}

#[test]
fn darkness_helper() {
    assert!(close(apply_darkness(0.2, 1.0), 0.2));
    assert!(close(apply_darkness(0.2, 0.0), 1.0));
    assert!(close(apply_darkness(0.6, 2.0), 0.2));
    assert!(close(apply_darkness(0.2, 2.0), 0.0));
    assert!(close(apply_darkness(1.0, 2.0), 1.0));
    assert!(close(apply_darkness(0.5, 0.5), 0.75));
}

#[test]
fn stagger_spreads_actors_evenly_once_per_period() {
    let budget = SightBudget::default();
    let period = budget.sight_period_ticks(1.0 / 60.0);
    assert_eq!(period, 6);
    assert_eq!(budget.tier1_period_ticks(1.0 / 60.0), 60);

    let n = 1200u32;
    let mut per_tick = [0u32; 6];
    for k in 0..n {
        let key = ActorKey(u128::from(k));
        let hits: Vec<u64> = (0..12u64).filter(|&t| due(key, t, period)).collect();
        assert_eq!(hits.len(), 2, "exactly once per period");
        assert_eq!(hits[1] - hits[0], 6);
        per_tick[usize::try_from(hits[0]).unwrap()] += 1;
    }
    // 200 per tick expected; sequential keys still spread (binomial sd ~13).
    for c in per_tick {
        assert!((150..=250).contains(&c), "{per_tick:?}");
    }
    // Stable: the same key always lands on the same phase.
    assert_eq!(stagger_phase(ActorKey(42), 6), stagger_phase(ActorKey(42), 6));
}

#[test]
fn timer_force_flag_and_elapsed() {
    let key = ActorKey(7);
    let period = 6;
    let phase = u64::from(stagger_phase(key, period));
    let mut t = SightTimer::starting_at(0);
    let off = (phase + 1) % 6 + 6; // a tick that is not due
    assert_eq!(t.poll(key, off, period), None);
    t.force();
    assert!(t.is_forced());
    assert_eq!(t.poll(key, off, period), Some(6)); // capped at one period
    assert!(!t.is_forced());
    assert_eq!(t.poll(key, off, period), None, "already checked this tick");
    t.force();
    assert_eq!(t.poll(key, off + 1, period), Some(1));
    let next = (off + 2..off + 8).find(|&x| due(key, x, period)).unwrap();
    assert_eq!(t.poll(key, next, period), Some(u32::try_from(next - off - 1).unwrap()));
}

fn player(n: u128) -> PerceptTarget {
    PerceptTarget::Player(PlayerKey(n))
}

fn sight_req(owner: u128) -> RayRequest {
    RayRequest::sight(ActorKey(owner), player(1), at(0.0, 0.0, 1.6), at(10.0, 0.0, 1.2), at(10.0, 0.0, 1.7))
}

#[test]
fn head_ray_only_after_chest_blocked() {
    let chest = at(10.0, 0.0, 1.2);
    let head = at(10.0, 0.0, 1.7);
    // (chest clear, head clear) → expected outcome and casts.
    for (chest_clear, head_clear, outcome, casts) in [
        (true, true, LosOutcome::Clear, 1),
        (false, true, LosOutcome::ClearFallback, 2),
        (false, false, LosOutcome::Blocked, 2),
    ] {
        let mut q = RayQueue::new(8, 8);
        q.push(sight_req(1), 0).unwrap();
        let mut log = Vec::new();
        let mut world = |r: &LosRay| {
            log.push(r.to);
            if r.to == chest { chest_clear } else { head_clear }
        };
        let mut out = Vec::new();
        let stats = q.process(10, &mut world, &mut out);
        assert_eq!(stats, RayTickStats { cast: casts, deferred: 0 });
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].outcome, outcome);
        assert_eq!(log[0], chest);
        if casts == 2 {
            assert_eq!(log[1], head);
        }
        assert!(q.is_empty());
    }
}

#[test]
fn cap_carries_overflow_oldest_first_results_in_actor_order() {
    let mut q = RayQueue::new(16, 16);
    // Pushed oldest first with descending keys: 50, 40 at tick 0; 30, 20, 10 at tick 1.
    for (owner, tick) in [(50, 0), (40, 0), (30, 1), (20, 1), (10, 1)] {
        q.push(sight_req(owner), tick).unwrap();
    }
    let mut cast_owners = Vec::new();
    let mut world = |r: &LosRay| {
        cast_owners.push(r.owner.0);
        true
    };
    let mut out = Vec::new();
    let stats = q.process(3, &mut world, &mut out);
    assert_eq!(stats, RayTickStats { cast: 3, deferred: 2 });
    let owners: Vec<u128> = out.iter().map(|r| r.owner.0).collect();
    assert_eq!(owners, [30, 40, 50], "oldest three, applied in ActorKey order");
    assert_eq!(out[2].enqueued, 0);
    // A newer request queues behind the carried ones.
    q.push(sight_req(5), 2).unwrap();
    let stats = q.process(2, &mut world, &mut out);
    assert_eq!(stats, RayTickStats { cast: 2, deferred: 1 });
    let owners: Vec<u128> = out.iter().map(|r| r.owner.0).collect();
    assert_eq!(owners, [10, 20]);
    assert_eq!(cast_owners, [50, 40, 30, 20, 10]);
}

#[test]
fn blocked_chest_at_the_cap_casts_only_the_head_next_tick() {
    let chest = at(10.0, 0.0, 1.2);
    let mut q = RayQueue::new(8, 8);
    q.push(sight_req(1), 0).unwrap();
    let mut log = Vec::new();
    let mut world = |r: &LosRay| {
        log.push(r.to);
        r.to != chest
    };
    let mut out = Vec::new();
    assert_eq!(q.process(1, &mut world, &mut out), RayTickStats { cast: 1, deferred: 1 });
    assert!(out.is_empty());
    q.push(sight_req(2), 1).unwrap();
    assert_eq!(q.process(1, &mut world, &mut out), RayTickStats { cast: 1, deferred: 1 });
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].owner, out[0].outcome), (ActorKey(1), LosOutcome::ClearFallback));
    assert_eq!(log, [chest, at(10.0, 0.0, 1.7)]);
}

#[test]
fn queue_dedupes_pairs_and_reports_full() {
    let mut q = RayQueue::new(2, 4);
    q.push(sight_req(1), 0).unwrap();
    q.push(sight_req(1), 3).unwrap();
    assert_eq!(q.len(), 1);
    q.push(sight_req(2), 3).unwrap();
    assert_eq!(q.push(sight_req(3), 3), Err(RayQueueFull(2)));
    q.cancel(ActorKey(1));
    assert_eq!(q.len(), 1);
    let mut out = Vec::new();
    q.process(4, &mut |_: &LosRay| true, &mut out);
    assert_eq!(out[0].owner, ActorKey(2));
}

#[test]
fn cache_hits_until_time_or_movement() {
    let budget = SightBudget::default(); // 0.25 s, 0.5 m
    let clock = |tick| PerceptClock::new(tick, 1.0 / 60.0); // 15 ticks
    let mut c = SightCache::new(2);
    let (obs, tgt) = (at(0.0, 0.0, 0.0), at(10.0, 0.0, 0.0));
    assert_eq!(c.get(ActorKey(1), player(1), obs, tgt, clock(0), &budget), None);
    assert!(c.insert(ActorKey(1), player(1), obs, tgt, 0, LosOutcome::Clear));
    assert_eq!(c.get(ActorKey(1), player(1), obs, tgt, clock(14), &budget), Some(LosOutcome::Clear));
    assert_eq!(c.get(ActorKey(1), player(1), obs, tgt, clock(15), &budget), None);
    let near = at(0.4, 0.0, 0.0);
    let moved = at(0.6, 0.0, 0.0);
    assert_eq!(c.get(ActorKey(1), player(1), near, tgt, clock(1), &budget), Some(LosOutcome::Clear));
    assert_eq!(c.get(ActorKey(1), player(1), moved, tgt, clock(1), &budget), None);
    assert_eq!(c.get(ActorKey(1), player(1), obs, at(10.0, 0.6, 0.0), clock(1), &budget), None);
    assert_eq!(c.get(ActorKey(2), player(1), obs, tgt, clock(1), &budget), None);
    // Fixed capacity: a third pair is refused until pruned.
    assert!(c.insert(ActorKey(2), player(1), obs, tgt, 10, LosOutcome::Blocked));
    assert!(!c.insert(ActorKey(3), player(1), obs, tgt, 10, LosOutcome::Blocked));
    c.prune(clock(20), &budget);
    assert_eq!(c.len(), 1);
    assert!(c.insert(ActorKey(3), player(1), obs, tgt, 20, LosOutcome::Blocked));
}

#[test]
fn same_total_gain_over_one_second_at_30_and_144_hz() {
    let p = globals();
    let budget = SightBudget::default();
    let c = check(16.5, 20.0, false);
    let total = |hz: u32| {
        #[allow(clippy::cast_precision_loss)]
        let tick_s = 1.0 / hz as f32;
        let period = budget.sight_period_ticks(tick_s);
        let key = ActorKey(99);
        let mut timer = SightTimer::starting_at(0);
        let mut sum = 0.0f32;
        for tick in 1..=u64::from(hz) {
            if tick == u64::from(hz) {
                timer.force(); // integrate to exactly 1 s
            }
            if let Some(e) = timer.poll(key, tick, period) {
                #[allow(clippy::cast_precision_loss)]
                let dt = e as f32 * tick_s;
                sum += gain(&p, &c, dt).unwrap();
            }
        }
        sum
    };
    let (a, b) = (total(30), total(144));
    assert!((a - b).abs() < 1e-4, "{a} vs {b}");
    assert!((a - 1.0).abs() < 1e-4, "rate 2 × f_dist 0.5 × 1 s = {a}");
}
