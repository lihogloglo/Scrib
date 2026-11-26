//! ai.md "Tiers and budget" `AI10a`: the tier-0 state machine stepped at events, tier invariance
//! against a per-tick executor, limits, fast-forward, following, and the load-time registry rule.

use std::cell::RefCell;
use std::collections::BTreeMap;

use glam::DVec3;
use sf_coords::WorldPos;
use sf_ids::GameInstant;
use sf_records::RecordId;
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::*;

const NPC: ActorKey = ActorKey(0xA1);
const LEADER: ActorKey = ActorKey(0xB2);
/// Morrowind's default clock: 30 game seconds per simulated second.
const TS: f64 = 30.0;
const SPEED: f32 = 2.0;
/// One fixed tick of 1/60 s at x30, in game milliseconds.
const TICK_MS: u64 = 500;

fn at(x: f64, z: f64) -> Location {
    Location::new(WorldspaceId::new("test.ws"), WorldPos::from_f64(DVec3::new(x, 0.0, z)))
}

fn ms(ms: u64) -> GameInstant {
    GameInstant(ms)
}

fn close(a: &Location, b: &Location) {
    assert_eq!(a.worldspace, b.worldspace);
    assert!(a.pos.delta(b.pos).length() < 1e-3, "{a:?} != {b:?}");
}

/// Routes are two edges: along x to the goal's x, then along z (`from` -> corner -> `to`).
struct Env {
    leaders: RefCell<BTreeMap<ActorKey, LogicalState>>,
}

impl Env {
    fn new() -> Self {
        Self { leaders: RefCell::new(BTreeMap::new()) }
    }

    fn corners(route: &RouteRef) -> [DVec3; 3] {
        let (a, b) = (route.from.pos.to_f64(), route.to.pos.to_f64());
        [a, DVec3::new(b.x, a.y, a.z), b]
    }
}

impl LogicalEnv for Env {
    #[expect(clippy::cast_possible_truncation, reason = "test routes are short")]
    fn route(&self, _actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
        let mut r = RouteRef { from: from.clone(), to: to.clone(), length_m: 0.0, speed_mps: SPEED };
        let [a, c, b] = Self::corners(&r);
        r.length_m = ((c - a).length() + (b - c).length()) as f32;
        Some(r)
    }

    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
        let [start, corner, end] = Self::corners(route);
        let along = f64::from(progress_m);
        let first = (corner - start).length();
        let point = if along <= first {
            start + (corner - start).normalize_or_zero() * along
        } else {
            corner + (end - corner).normalize_or_zero() * (along - first).min((end - corner).length())
        };
        Location::new(route.from.worldspace.clone(), WorldPos::from_f64(point))
    }

    fn placement(&self, _id: PlacementId) -> Option<Location> {
        None
    }

    fn actor_position(&self, actor: ActorKey, t: GameInstant) -> Option<Location> {
        self.leaders.borrow().get(&actor).map(|s| s.position_at(t, self, TS))
    }
}

fn ctx<'a>(env: &'a Env, transitions: &'a LogicalTransitions) -> LogicalCtx<'a> {
    LogicalCtx { env, transitions, policy: &ModernPolicy, time_scale: TS }
}

fn push(stack: &mut DirectiveStack, spec: DirectiveSpec) -> DirectiveId {
    stack.push(spec, GameTime::from_minutes(0), &ModernPolicy, &mut Vec::new())
}

fn go_to(to: Location, source: DirectiveSource) -> DirectiveSpec {
    DirectiveSpec::new(DirectiveKind::GoTo(to), source)
}

/// A quest `GoTo` the far corner (210 m: 105 s at 2 m/s, 52.5 game minutes at x30), a script
/// `GoTo` back (another 210 m), and a record `StayNear` home `limit` at the bottom.
fn two_trips(limit: Option<Limit>) -> DirectiveStack {
    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    let mut stay = DirectiveSpec::new(DirectiveKind::StayNear { home: at(0.0, 0.0), radius: 5.0 }, DirectiveSource::Record);
    stay.limit = limit;
    push(&mut stack, stay);
    push(&mut stack, go_to(at(0.0, 0.0), DirectiveSource::Script));
    push(&mut stack, go_to(at(100.0, 110.0), DirectiveSource::Quest));
    stack
}

fn done(id: u32, end: DirectiveEnd, minute: u64) -> DirectiveDone {
    DirectiveDone { actor: NPC, id: DirectiveId(id), end, at: GameTime::from_minutes(minute) }
}

#[test]
fn go_to_finishes_at_the_exact_arrival_and_the_next_top_takes_over() {
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let mut stack = two_trips(None);
    let mut state = LogicalState::new(at(0.0, 0.0), ms(0));
    let mut events = Vec::new();
    state.step(&mut stack, ms(0), &c, &mut events);
    assert_eq!(state.directive, Some(DirectiveId(2)));
    // 210 m at 2 m/s = 105 s simulated = 3150 game s.
    assert_eq!(state.until, Some(ms(3_150_000)));
    close(&state.position_at(ms(1_575_000), &env, TS), &at(100.0, 5.0));

    state.step(&mut stack, ms(3_149_999), &c, &mut events);
    assert!(events.is_empty());
    assert_eq!(state.until, Some(ms(3_150_000)));
    assert_eq!(state.step(&mut stack, ms(3_150_000), &c, &mut events), 1);
    // Ended at the arrival's own minute (52.5 min reads 52), not at a later event.
    assert_eq!(events, vec![done(2, DirectiveEnd::Completed, 52)]);
    assert_eq!(stack.top().map(|d| d.id), Some(DirectiveId(1)));
    assert_eq!(state.directive, Some(DirectiveId(1)));
    close(&state.pos, &at(100.0, 110.0));
    assert_eq!(state.route.as_ref().map(|r| &r.to), Some(&at(0.0, 0.0)));
    assert_eq!(state.until, Some(ms(6_300_000)));
}

/// The tier-2 stand-in: every 1/60 s it advances the stack, walks the same route and finishes
/// the served `GoTo` on the tick it arrives. Returns each end with the instant it was noticed.
fn per_tick(mut stack: DirectiveStack, env: &Env, end: GameInstant) -> Vec<(DirectiveDone, GameInstant)> {
    let mut out = Vec::new();
    let (mut pos, mut now) = (at(0.0, 0.0), ms(0));
    let mut walking: Option<(DirectiveId, RouteRef, f64)> = None;
    while now < end {
        now = now.plus(TICK_MS);
        let mut events = Vec::new();
        stack.advance(GameTime::from_instant(now), &ModernPolicy, &mut events);
        let served = stack.serving().map(|d| (d.id, d.spec.kind.clone()));
        if walking.as_ref().map(|w| w.0) != served.as_ref().map(|s| s.0) {
            walking = match served {
                Some((id, DirectiveKind::GoTo(to))) => env.route(NPC, &pos, &to).map(|r| (id, r, 0.0)),
                _ => None,
            };
        }
        if let Some((id, route, m)) = &mut walking {
            *m += f64::from(SPEED) / 60.0;
            #[expect(clippy::cast_possible_truncation, reason = "metres")]
            let along = *m as f32;
            pos = env.point_on(route, along.min(route.length_m));
            if *m + 1e-6 >= f64::from(route.length_m) {
                stack.finish(*id, GameTime::from_instant(now), &ModernPolicy, &mut events);
            }
        }
        out.extend(events.into_iter().map(|e| (e, now)));
    }
    out
}

#[test]
fn tier_zero_and_a_per_tick_executor_end_the_same_directives_within_one_tick() {
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let end = GameInstant::from_minutes(180);
    let stack = two_trips(Some(Limit::For { minutes: 10 }));
    let ticked = per_tick(stack.clone(), &env, end);

    // Tier 0: one step per event, remembering the instant of each.
    let mut stack0 = stack;
    let mut state = LogicalState::new(at(0.0, 0.0), ms(0));
    let mut logical = Vec::new();
    state.step(&mut stack0, ms(0), &c, &mut Vec::new());
    while let Some(u) = state.until.filter(|u| *u <= end) {
        let mut events = Vec::new();
        state.step(&mut stack0, u, &c, &mut events);
        logical.extend(events.into_iter().map(|e| (e, u)));
    }

    let order = |v: &[(DirectiveDone, GameInstant)]| v.iter().map(|(e, _)| (e.id, e.end)).collect::<Vec<_>>();
    assert_eq!(order(&logical), order(&ticked));
    assert_eq!(
        order(&logical),
        vec![
            (DirectiveId(2), DirectiveEnd::Completed),
            (DirectiveId(1), DirectiveEnd::Completed),
            (DirectiveId(0), DirectiveEnd::Expired)
        ]
    );
    for ((e0, t0), (e2, t2)) in logical.iter().zip(&ticked) {
        assert!(t2.0 >= t0.0 && t2.0 - t0.0 <= TICK_MS, "{e0:?} at {t0:?} vs {e2:?} at {t2:?}");
        assert_eq!(e0.at, GameTime::from_instant(*t0));
        assert_eq!(e2.at, GameTime::from_instant(*t2));
    }
    // Arrivals at 52.5 and 105 min; the stay's 10 served minutes run out at 115.
    let times: Vec<_> = logical.iter().map(|(e, _)| e.at.total_minutes()).collect();
    assert_eq!(times, vec![52, 105, 115]);
}

#[test]
fn stay_near_with_a_two_hour_limit_is_one_event_at_home() {
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let home = at(0.0, 0.0);
    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    let spec = DirectiveSpec::new(DirectiveKind::StayNear { home: home.clone(), radius: 5.0 }, DirectiveSource::Record)
        .limit(Limit::For { minutes: 120 });
    push(&mut stack, spec);
    let mut state = LogicalState::new(home.clone(), ms(0));
    let mut events = Vec::new();
    state.step(&mut stack, ms(0), &c, &mut events);
    assert_eq!(state.activity, Some(ActivityRef::Wait));
    assert_eq!(state.until, Some(GameInstant::from_minutes(120)));
    for minute in [1, 30, 119, 120, 150, 180] {
        let stepped = state.step(&mut stack, GameInstant::from_minutes(minute), &c, &mut events);
        assert_eq!(stepped, usize::from(minute == 120));
        assert_eq!(state.pos, home);
    }
    assert_eq!(events, vec![done(0, DirectiveEnd::Expired, 120)]);
    assert_eq!(state.until, None);
    assert_eq!(state.directive, None);
}

#[test]
fn fast_forward_equals_stepping_every_event_and_crosses_two_arrivals_in_order() {
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let start = |stack: &mut DirectiveStack| {
        let mut s = LogicalState::new(at(0.0, 0.0), ms(0));
        s.step(stack, ms(0), &c, &mut Vec::new());
        s
    };
    let mut stack_a = two_trips(None);
    let mut a = start(&mut stack_a);
    let mut events_a = Vec::new();
    let reached = fast_forward(&mut a, &mut stack_a, 3.0, &c, &ModernFastForward, &mut events_a);
    assert_eq!(reached, GameInstant::from_minutes(180));

    let mut stack_b = two_trips(None);
    let mut b = start(&mut stack_b);
    let mut events_b = Vec::new();
    while let Some(u) = b.until.filter(|u| *u <= reached) {
        b.step(&mut stack_b, u, &c, &mut events_b);
    }
    b.step(&mut stack_b, reached, &c, &mut events_b);

    assert_eq!(events_a, events_b);
    assert_eq!(a, b);
    assert_eq!(stack_a, stack_b);
    assert_eq!(events_a, vec![done(2, DirectiveEnd::Completed, 52), done(1, DirectiveEnd::Completed, 105)]);
    // Home again, waiting under the unlimited stay.
    close(&a.pos, &at(0.0, 0.0));
    assert_eq!(a.activity, Some(ActivityRef::Wait));
    assert_eq!(a.until, None);
}

#[test]
fn a_follower_stands_where_its_leader_is_and_escort_ends_at_arrival() {
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let goal = at(100.0, 110.0);
    let mut lead_stack = DirectiveStack::new(LEADER, GameTime::from_minutes(0));
    push(&mut lead_stack, DirectiveSpec::new(DirectiveKind::Escort { who: NPC, to: goal.clone() }, DirectiveSource::Quest));
    let mut leader = LogicalState::new(at(0.0, 0.0), ms(0));
    let mut lead_events = Vec::new();
    leader.step(&mut lead_stack, ms(0), &c, &mut lead_events);
    env.leaders.borrow_mut().insert(LEADER, leader.clone());

    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    push(&mut stack, DirectiveSpec::new(DirectiveKind::Follow(LEADER), DirectiveSource::Quest));
    let mut follower = LogicalState::new(at(0.0, 0.0), ms(0));
    follower.step(&mut stack, ms(0), &c, &mut Vec::new());
    assert_eq!(follower.follow, Some(LEADER));

    // Every leader event (its arrival), and points between.
    let mut checks = vec![ms(600_000), ms(1_575_000)];
    checks.extend(leader.until);
    checks.push(GameInstant::from_minutes(90));
    for t in checks {
        leader.step(&mut lead_stack, t, &c, &mut lead_events);
        env.leaders.borrow_mut().insert(LEADER, leader.clone());
        follower.step(&mut stack, t, &c, &mut Vec::new());
        assert_eq!(follower.pos, leader.position_at(t, &env, TS));
        assert_eq!(follower.position_at(t, &env, TS), leader.pos);
    }
    assert_eq!(
        lead_events,
        vec![DirectiveDone { actor: LEADER, id: DirectiveId(0), end: DirectiveEnd::Completed, at: GameTime::from_minutes(52) }]
    );
    close(&follower.pos, &goal);
}

#[test]
fn fight_holds_in_tier_zero() {
    // Combat is pinned at tier 2 (`Pin::Combat`); should tier 0 ever see a fight, nothing moves.
    let (env, tr) = (Env::new(), LogicalTransitions::new());
    let c = ctx(&env, &tr);
    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    push(&mut stack, DirectiveSpec::new(DirectiveKind::Fight(LEADER), DirectiveSource::Script));
    let mut state = LogicalState::new(at(3.0, 4.0), ms(0));
    state.step(&mut stack, GameInstant::from_minutes(60), &c, &mut Vec::new());
    assert_eq!((state.route.as_ref(), state.activity, state.follow, state.until), (None, None, None, None));
    assert_eq!(state.pos, at(3.0, 4.0));
}

#[test]
fn an_unregistered_custom_kind_fails_validation_by_name() {
    let patrol = DirectiveKind::Custom(RecordId::new("test.patrol"));
    let kinds = [DirectiveKind::GoTo(at(1.0, 0.0)), DirectiveKind::Fight(LEADER), patrol.clone()];
    let mut tr = LogicalTransitions::new();
    let err = tr.validate(&kinds).unwrap_err();
    assert_eq!(err, UnregisteredKind(RecordId::new("test.patrol")));
    assert!(err.to_string().contains("test.patrol"), "{err}");

    tr.register("test.patrol", |_: &TransitionInput<'_>| Transition::Travel {
        to: at(0.0, 50.0),
        arrive: Arrival::Activity(ActivityRef::Wait),
    });
    assert_eq!(tr.validate(&kinds), Ok(()));

    // The registered transition runs: walk 50 m, then wait there.
    let env = Env::new();
    let c = ctx(&env, &tr);
    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    push(&mut stack, DirectiveSpec::new(patrol, DirectiveSource::Record));
    let mut state = LogicalState::new(at(0.0, 0.0), ms(0));
    state.step(&mut stack, GameInstant::from_minutes(60), &c, &mut Vec::new());
    close(&state.pos, &at(0.0, 50.0));
    assert_eq!((state.route.as_ref(), state.activity), (None, Some(ActivityRef::Wait)));
}
