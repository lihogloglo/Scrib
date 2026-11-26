//! ai.md "Tiers and budget" `AI10b`: the promotion queue (order, sync, budget, retry, no
//! starvation), unseen placement (spot, backtrack, approach, snap, claim), a queued actor that
//! keeps advancing, and the demotion rule through the allocator.

use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;

use glam::DVec3;
use sf_coords::WorldPos;
use sf_ids::GameInstant;
use sf_world::{Location, PlacementId, WorldspaceId};

use crate::*;

const NPC: ActorKey = ActorKey(0xA1);
/// 30 game seconds per simulated second.
const TS: f64 = 30.0;
const SPEED: f32 = 2.0;
const RADIUS: f32 = 0.5;

fn at(x: f64) -> Location {
    Location::new(WorldspaceId::new("test.ws"), WorldPos::from_f64(DVec3::new(x, 0.0, 0.0)))
}

fn x_of(l: &Location) -> f64 {
    l.pos.to_f64().x
}

/// Straight routes along x.
struct Env;

impl LogicalEnv for Env {
    #[expect(clippy::cast_possible_truncation, reason = "test routes are short")]
    fn route(&self, _actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
        let length_m = (x_of(to) - x_of(from)).abs() as f32;
        Some(RouteRef { from: from.clone(), to: to.clone(), length_m, speed_mps: SPEED })
    }

    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
        let (a, b) = (x_of(&route.from), x_of(&route.to));
        let m = f64::from(progress_m.clamp(0.0, route.length_m));
        at(a + (b - a).signum() * m)
    }

    fn placement(&self, _id: PlacementId) -> Option<Location> {
        None
    }

    fn actor_position(&self, _actor: ActorKey, _t: GameInstant) -> Option<Location> {
        None
    }
}

/// Players see the half-space `x >= seen_from` (a body sphere touching it counts).
struct World {
    seen_from: Cell<f64>,
    ground: Cell<bool>,
    claim_ok: Cell<bool>,
    approaches: RefCell<Vec<Location>>,
}

impl World {
    fn seeing_from(x: f64) -> Self {
        Self { seen_from: Cell::new(x), ground: Cell::new(true), claim_ok: Cell::new(true), approaches: RefCell::new(Vec::new()) }
    }

    fn blind() -> Self {
        Self::seeing_from(f64::INFINITY)
    }
}

impl PlayerViews for World {
    fn seen_by_any(&self, at: &Location, radius_m: f32) -> bool {
        x_of(at) + f64::from(radius_m) >= self.seen_from.get()
    }
}

impl PromoteEnv for World {
    fn snap(&self, at: &Location) -> Option<Location> {
        self.ground.get().then(|| at.clone())
    }

    fn claim(&self, _actor: ActorKey, _slot: PlacementId) -> bool {
        self.claim_ok.get()
    }

    fn approaches(&self, _actor: ActorKey, _at: &Location, out: &mut Vec<Location>) {
        out.extend(self.approaches.borrow().iter().cloned());
    }

    fn body_radius_m(&self, _actor: ActorKey) -> f32 {
        RADIUS
    }
}

fn req(actor: u128, dist: f32, pin: Option<Pin>) -> PromoteRequest {
    PromoteRequest { actor: ActorKey(actor), to: Tier::Full, nearest_player_m: dist, pin }
}

fn dummy() -> Placement {
    Placement { at: at(0.0), progress_m: None, activity: None, entered_from: EnteredFrom::Spot }
}

fn placed_keys(t: &PromoteTick) -> Vec<u128> {
    t.placed.iter().map(|(a, _, _)| a.0).collect()
}

fn config(per_tick: u32) -> PromoteConfig {
    PromoteConfig { per_tick, ..PromoteConfig::default() }
}

/// Standing at `x` (no route) at `t`.
fn standing(x: f64, t: u64) -> LogicalState {
    LogicalState::new(at(x), GameInstant(t))
}

/// On the 100 m route from x = 0, `progress` metres along it at instant 0.
fn on_route(progress: f32) -> LogicalState {
    let mut s = LogicalState::new(at(f64::from(progress)), GameInstant(0));
    s.route = Env.route(NPC, &at(0.0), &at(100.0));
    s.progress_m = progress;
    s
}

fn resolve_now(state: &LogicalState, world: &World, config: &PromoteConfig) -> Resolve {
    resolve_promotion(NPC, state, state.at, TS, &Env, world, config)
}

fn placement(r: Resolve) -> Placement {
    match r {
        Resolve::Place(p) => p,
        Resolve::Wait => panic!("expected a placement, got Wait"),
    }
}

#[test]
fn queue_order_pins_by_distance_then_nearest_first() {
    let mut q = PromoteQueue::new();
    q.sync(&[
        req(6, 30.0, None),
        req(5, 10.0, Some(Pin::Combat)),
        req(4, 20.0, None),
        req(3, 50.0, Some(Pin::Dialogue)),
        req(2, 20.0, None),
        req(1, 5.0, None),
    ]);
    let c = config(4);
    let t0 = q.run(0, 0.1, &c, |_| Resolve::Place(dummy()));
    assert_eq!(placed_keys(&t0), [5, 3, 1, 2]);
    assert_eq!((t0.tried, t0.queued), (4, 2));
    let t1 = q.run(1, 0.1, &c, |_| Resolve::Place(dummy()));
    assert_eq!(placed_keys(&t1), [4, 6]);
    assert_eq!((t1.tried, t1.queued), (2, 0));
}

#[test]
fn sync_drops_unwanted_reorders_and_counts_a_duplicate_once() {
    let mut q = PromoteQueue::new();
    q.sync(&[req(1, 10.0, None), req(2, 20.0, None), req(3, 30.0, None), req(2, 25.0, None)]);
    assert_eq!(q.len(), 3);
    assert_eq!(q.actors().map(|a| a.0).collect::<Vec<_>>(), [1, 2, 3]);
    // 1 is no longer wanted, 3 came nearer, non-finite / negative distances clean like the allocator's.
    q.sync(&[req(2, 20.0, None), req(3, 5.0, None), req(4, f32::NAN, None), req(5, -3.0, None)]);
    assert!(!q.contains(ActorKey(1)));
    assert_eq!(q.actors().map(|a| a.0).collect::<Vec<_>>(), [5, 3, 2, 4]);
}

#[test]
fn budget_places_per_tick_four_four_two() {
    let mut q = PromoteQueue::new();
    let wanted: Vec<_> = (1..=10).map(|i| req(i, f32::from(u8::try_from(i).unwrap()), None)).collect();
    q.sync(&wanted);
    let c = config(4);
    let mut counts = Vec::new();
    for tick in 0..3 {
        let t = q.run(tick, 0.1, &c, |_| Resolve::Place(dummy()));
        counts.push((t.placed.len(), t.queued));
    }
    assert_eq!(counts, [(4, 6), (4, 2), (2, 0)]);
}

#[test]
fn a_seen_spot_waits_and_is_placed_on_the_first_try_after_it_is_unseen() {
    let world = World::seeing_from(0.0);
    let state = standing(30.0, 0);
    let c = PromoteConfig { per_tick: 4, retry_s: 0.5, ..PromoteConfig::default() };
    assert_eq!(c.retry_ticks(0.1), 5);
    let mut q = PromoteQueue::new();
    q.sync(&[req(NPC.0, 30.0, Some(Pin::Combat))]);
    let mut tries = Vec::new();
    for tick in 0..=10 {
        if tick == 7 {
            world.seen_from.set(1000.0);
        }
        let t = q.run(tick, 0.1, &c, |_| resolve_now(&state, &world, &c));
        if tick < 10 {
            assert!(t.placed.is_empty(), "placed in view or before its retry at tick {tick}");
            assert_eq!((t.waiting, t.queued), (1, 1), "tick {tick}");
        } else {
            assert_eq!(placed_keys(&t), [NPC.0]);
            assert_eq!(t.placed[0].2.entered_from, EnteredFrom::Spot);
            assert_eq!((t.waiting, t.queued), (0, 0));
        }
        tries.push(t.tried);
    }
    // Tried on ticks 0, 5 and 10 only (retry_s = 0.5 s = 5 ticks of 0.1 s).
    assert_eq!(tries, [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
}

#[test]
fn seen_waiters_do_not_starve_farther_unseen_actors() {
    let seen: BTreeSet<u128> = (1..=4).collect();
    let mut q = PromoteQueue::new();
    let wanted: Vec<_> = (1..=8).map(|i| req(i, f32::from(u8::try_from(i).unwrap()), None)).collect();
    q.sync(&wanted);
    let c = config(4);
    let decide = |r: &PromoteRequest| if seen.contains(&r.actor.0) { Resolve::Wait } else { Resolve::Place(dummy()) };
    let t0 = q.run(0, 0.1, &c, &decide);
    assert!(t0.placed.is_empty());
    assert_eq!((t0.tried, t0.waiting), (4, 4));
    let t1 = q.run(1, 0.1, &c, &decide);
    assert_eq!(placed_keys(&t1), [5, 6, 7, 8]);
    assert_eq!((t1.tried, t1.waiting, t1.queued), (4, 4, 4));
}

#[test]
fn backtrack_places_back_along_the_route_out_of_view() {
    // Spot at 30 m seen; the route is unseen before 22 m (sphere edge 0.5 m).
    let world = World::seeing_from(22.0);
    let state = on_route(30.0);
    let p = placement(resolve_now(&state, &world, &PromoteConfig::default()));
    assert_eq!(p.entered_from, EnteredFrom::Backtrack);
    assert_eq!(p.progress_m, Some(18.0));
    assert!((x_of(&p.at) - 18.0).abs() < 1e-6);
    assert_eq!(p.activity, None);

    let short = PromoteConfig { backtrack_m: 8.0, ..PromoteConfig::default() };
    assert_eq!(resolve_now(&state, &world, &short), Resolve::Wait);
}

#[test]
fn backtrack_stops_at_the_route_start() {
    // Spot at 6 m, everything from 0.6 m on is seen: 2 m is seen, the start (0 m) is not.
    let world = World::seeing_from(0.6);
    let p = placement(resolve_now(&on_route(6.0), &world, &PromoteConfig::default()));
    assert_eq!((p.entered_from, p.progress_m), (EnteredFrom::Backtrack, Some(0.0)));
    // Seen from -1 m: nothing on the route is unseen, and the start is tried once.
    let world = World::seeing_from(-1.0);
    assert_eq!(resolve_now(&on_route(6.0), &world, &PromoteConfig::default()), Resolve::Wait);
}

#[test]
fn an_approach_is_used_when_the_spot_is_seen() {
    let world = World::seeing_from(0.0);
    world.approaches.replace(vec![at(5.0), at(-10.0)]);
    let p = placement(resolve_now(&standing(30.0, 0), &world, &PromoteConfig::default()));
    assert_eq!(p.entered_from, EnteredFrom::Approach);
    assert!((x_of(&p.at) + 10.0).abs() < 1e-6);
    assert_eq!(p.progress_m, None);
}

#[test]
fn no_ground_waits() {
    let world = World::blind();
    world.ground.set(false);
    world.approaches.replace(vec![at(-10.0)]);
    assert_eq!(resolve_now(&standing(30.0, 0), &world, &PromoteConfig::default()), Resolve::Wait);
    assert_eq!(resolve_now(&on_route(30.0), &world, &PromoteConfig::default()), Resolve::Wait);
}

#[test]
fn a_refused_claim_places_the_actor_standing_there() {
    let world = World::blind();
    let mut state = standing(30.0, 0);
    state.activity = Some(ActivityRef::Use(PlacementId(7)));
    let p = placement(resolve_now(&state, &world, &PromoteConfig::default()));
    assert_eq!((p.entered_from, p.activity), (EnteredFrom::Spot, Some(ActivityRef::Use(PlacementId(7)))));
    world.claim_ok.set(false);
    let p = placement(resolve_now(&state, &world, &PromoteConfig::default()));
    assert_eq!(p.activity, Some(ActivityRef::Wait));
    assert!((x_of(&p.at) - 30.0).abs() < 1e-6);
}

#[test]
fn a_queued_actor_keeps_advancing_and_is_placed_where_it_is_at_resolve() {
    // Ticks of 1 s simulated (30 000 game ms); a GoTo 100 m east at 2 m/s.
    const TICK_MS: u64 = 30_000;
    let tr = LogicalTransitions::new();
    let ctx = LogicalCtx { env: &Env, transitions: &tr, policy: &ModernPolicy, time_scale: TS };
    let mut stack = DirectiveStack::new(NPC, GameTime::from_minutes(0));
    stack.push(
        DirectiveSpec::new(DirectiveKind::GoTo(at(100.0)), DirectiveSource::Quest),
        GameTime::from_minutes(0),
        &ModernPolicy,
        &mut Vec::new(),
    );
    let mut state = standing(0.0, 0);
    let mut events = Vec::new();
    state.step(&mut stack, GameInstant(0), &ctx, &mut events);
    assert!(state.route.is_some());

    let world = World::seeing_from(-1000.0);
    let c = PromoteConfig { retry_s: 2.0, ..PromoteConfig::default() };
    let mut q = PromoteQueue::new();
    q.sync(&[req(NPC.0, 40.0, None)]);
    let mut placed = None;
    for tick in 0..=2u64 {
        let now = GameInstant(tick * TICK_MS);
        state.step(&mut stack, now, &ctx, &mut events);
        if tick == 2 {
            world.seen_from.set(1000.0);
        }
        let t = q.run(tick, 1.0, &c, |r| resolve_promotion(r.actor, &state, now, TS, &Env, &world, &c));
        if let Some((_, _, p)) = t.placed.into_iter().next() {
            placed = Some((tick, p));
        }
    }
    let (tick, p) = placed.expect("placed once unseen");
    assert_eq!(tick, 2);
    // 2 s at 2 m/s: 4 m along, not the 0 m it stood at when it was queued.
    let progress = p.progress_m.expect("on its route");
    assert!((progress - 4.0).abs() < 1e-4, "{progress}");
    assert!((x_of(&p.at) - 4.0).abs() < 1e-4);
    assert_eq!(p.entered_from, EnteredFrom::Spot);
}

#[test]
fn demotion_only_for_the_unseen_unpinned_actor() {
    const PINNED: ActorKey = ActorKey(1);
    const SEEN: ActorKey = ActorKey(2);
    const FAR: ActorKey = ActorKey(3);
    let input = |actor, nearest_player_m, visible, pin| AllocInput { actor, nearest_player_m, visible, pin };
    let caps = TierCaps { full: 1, reduced: 1 };
    let cfg = AllocConfig::default();
    let mut alloc = Allocator::new();
    let ticks = [
        // FAR is nearest and gets the tier-1 slot; SEEN is far and unseen.
        [input(PINNED, 1000.0, false, Some(Pin::Dialogue)), input(SEEN, 300.0, false, None), input(FAR, 5.0, false, None)],
        // SEEN comes into view: it takes the slot, FAR (unseen, far) drops to tier 0.
        [input(PINNED, 1000.0, false, Some(Pin::Combat)), input(SEEN, 300.0, true, None), input(FAR, 500.0, false, None)],
        // FAR comes back nearest but unseen: the visible incumbent keeps tier 1.
        [input(PINNED, 2000.0, false, Some(Pin::Combat)), input(SEEN, 400.0, true, None), input(FAR, 1.0, false, None)],
    ];
    let mut demoted = Vec::new();
    for (tick, inputs) in (0u64..).zip(ticks.iter()) {
        let a = alloc.allocate(inputs, caps, &cfg, tick, 0.1);
        demoted.extend(a.changes.iter().filter(|c| c.to == Tier::Logical && c.from != Tier::Logical).map(|c| (tick, c.actor)));
    }
    assert_eq!(demoted, [(1, FAR)]);
    assert_eq!(alloc.tier(PINNED), Tier::Full);
    assert_eq!(alloc.tier(SEEN), Tier::Reduced);

    let body = at(12.5);
    let s = demote(body.clone(), GameInstant(77_000));
    assert_eq!((s.pos, s.at, s.route, s.directive), (body, GameInstant(77_000), None, None));
}
