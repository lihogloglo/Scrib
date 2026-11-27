//! Headless tests for [`AiLogicalPlugin`] (slice AI10c1): tier invariance against a per-tick
//! stand-in, event-driven stepping, leaders before followers, the hold, the promotion budget,
//! the body-on-its-spot rule, demotion and back, the `ai.stats` line, the live tunable.
//!
//! Fakes: straight-line routes ([`Env`]), a seen half-space ([`Views`]: x above a line is in
//! view), a test clock ([`Clock`], advanced by hand: `GameClock` is a resource only with
//! `sf_ids/bevy`, which this crate does not enable), and test systems in
//! [`AiLogicalSystems::Step`] / [`AiLogicalSystems::Promote`] calling [`AiTierZero`].

use bevy::time::{Fixed, Time};
use bevy_app::FixedUpdate;
use glam::{DVec3, Vec3};
use sf_ai::{
    DirectiveDone, DirectiveEnd, DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, EnteredFrom, FastForwardRules,
    GameTime,
    LogicalCtx, LogicalEnv, LogicalState, LogicalTransitions, ModernPolicy, Placement, PlayerViews, PromoteConfig, PromoteEnv,
    RouteRef, Tier, TierCaps,
};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::{ActorKey, GameClock, GameInstant};
use sf_world::{Location, PlacementId, WorldspaceId};

use super::*;
use crate::sight_tests::mover;

const HZ: f64 = 60.0;
/// Game seconds per simulated second (the clock's `time_scale`).
const TS: f64 = 30.0;
/// One 1/60 s tick at x30, game milliseconds.
const TICK_MS: u64 = 500;
const SPEED: f32 = 2.0;
const BIG: TierCaps = TierCaps { full: 64, reduced: 64 };

fn ws() -> WorldspaceId {
    WorldspaceId::new("test.ws")
}

fn loc(x: f64, z: f64) -> Location {
    Location::new(ws(), WorldPos::from_f64(DVec3::new(x, 0.0, z)))
}

/// Straight routes at [`SPEED`]; no placements; bodies unknown (every test actor is stored).
struct Env;

impl LogicalEnv for Env {
    #[expect(clippy::cast_possible_truncation, reason = "test routes are short")]
    fn route(&self, _actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
        let length_m = from.pos.delta(to.pos).length() as f32;
        Some(RouteRef { from: from.clone(), to: to.clone(), length_m, speed_mps: SPEED })
    }

    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
        let (a, b) = (route.from.pos.to_f64(), route.to.pos.to_f64());
        let t = if route.length_m > 0.0 { f64::from((progress_m / route.length_m).clamp(0.0, 1.0)) } else { 1.0 };
        Location::new(route.from.worldspace.clone(), WorldPos::from_f64(a.lerp(b, t)))
    }

    fn placement(&self, _id: PlacementId) -> Option<Location> {
        None
    }

    fn actor_position(&self, _actor: ActorKey, _at: GameInstant) -> Option<Location> {
        None
    }
}

/// Players see every sphere reaching past `x > seen_x`; the navmesh is everywhere.
#[derive(Resource, Clone, Copy)]
struct Views {
    seen_x: f64,
}

impl PlayerViews for Views {
    fn seen_by_any(&self, at: &Location, radius_m: f32) -> bool {
        at.pos.to_f64().x + f64::from(radius_m) > self.seen_x
    }
}

impl PromoteEnv for Views {
    fn snap(&self, at: &Location) -> Option<Location> {
        Some(at.clone())
    }

    fn body_radius_m(&self, _actor: ActorKey) -> f32 {
        0.5
    }
}

#[derive(Resource)]
struct Clock(GameClock);

/// Every directive end the store's step reported, with the instant it was stepped at.
#[derive(Resource, Default)]
struct Ends(Vec<(DirectiveDone, GameInstant)>);

/// Every placement, with the allocator tick it happened on.
#[derive(Resource, Default)]
struct Placed(Vec<(ActorKey, Tier, Placement, u64)>);

#[derive(Resource)]
struct Transitions(LogicalTransitions);

fn step_system(mut t0: AiTierZero, clock: Res<Clock>, tr: Res<Transitions>, mut ends: ResMut<Ends>) {
    let now = clock.0.now;
    let ctx = LogicalCtx { env: &Env, transitions: &tr.0, policy: &ModernPolicy, time_scale: TS };
    let mut events = Vec::new();
    t0.step(now, &ctx, &mut events);
    ends.0.extend(events.into_iter().map(|e| (e, now)));
}

fn promote_system(
    mut t0: AiTierZero,
    clock: Res<Clock>,
    world: Res<Views>,
    alloc: Res<AiAllocStats>,
    mut placed: ResMut<Placed>,
    mut bodies: Query<(&AiActor, &mut Mover)>,
) {
    let now = clock.0.now;
    let body_at = |k: ActorKey| bodies.iter().find(|(a, _)| a.0 == k).map(|(_, m)| Location::new(ws(), m.loco.position));
    let out = t0.promote(now, TS, &Env, &*world, &body_at);
    for (key, tier, p) in out.placed {
        if let Some((_, mut m)) = bodies.iter_mut().find(|(a, _)| a.0 == key) {
            m.loco.position = p.at.pos;
        }
        placed.0.push((key, tier, p, alloc.tick));
    }
}

fn app(caps: TierCaps, world: Views) -> App {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(HZ));
    with_shell(app, caps, world)
}

/// Adds the allocator and the shell (unless the app has them), the fakes and the test systems.
fn with_shell(mut app: App, caps: TierCaps, world: Views) -> App {
    if !app.is_plugin_added::<AiAllocPlugin>() {
        app.add_plugins((AiAllocPlugin { timings: false }, AiLogicalPlugin { timings: false }));
    }
    app.world_mut().resource_mut::<AiAllocSettings>().set_caps(caps);
    app.insert_resource(Clock(GameClock::new(GameInstant::from_minutes(60), 30.0)))
        .insert_resource(Transitions(LogicalTransitions::new()))
        .insert_resource(world)
        .init_resource::<Ends>()
        .init_resource::<Placed>()
        .add_systems(
            FixedUpdate,
            (step_system.in_set(AiLogicalSystems::Step), promote_system.in_set(AiLogicalSystems::Promote)),
        );
    app
}

/// Advances the clock one tick, then runs the tick.
fn step(app: &mut App) {
    app.world_mut().resource_mut::<Clock>().0.skip(TICK_MS);
    app.world_mut().run_schedule(FixedUpdate);
}

fn now(app: &App) -> GameInstant {
    app.world().resource::<Clock>().0.now
}

fn minute(app: &App) -> GameTime {
    GameTime::from_instant(now(app))
}

fn stack_with(app: &mut App, key: ActorKey, kind: DirectiveKind, source: DirectiveSource) {
    let m = minute(app);
    let mut stack = DirectiveStack::new(key, m);
    let _id = stack.push(DirectiveSpec::new(kind, source), m, &ModernPolicy, &mut Vec::new());
    app.world_mut().resource_mut::<AiDirectives>().insert(stack);
}

fn store_state(app: &mut App, key: ActorKey, at: Location) {
    let t = now(app);
    app.world_mut().resource_mut::<AiLogical>().insert(key, LogicalState::new(at, t));
}

fn stay_near(at: Location) -> DirectiveKind {
    DirectiveKind::StayNear { home: at, radius: 5.0 }
}

#[test]
fn step_is_event_driven_waiting_states_step_once() {
    let mut app = app(BIG, Views { seen_x: 1e9 });
    for i in 0..100u32 {
        let key = ActorKey(1000 + u128::from(i));
        let home = loc(f64::from(i), 0.0);
        stack_with(&mut app, key, stay_near(home.clone()), DirectiveSource::Record);
        store_state(&mut app, key, home);
    }
    let mut stepped = 0;
    for _ in 0..600 {
        step(&mut app);
        stepped += app.world().resource::<AiLogicalStats>().stepped;
    }
    assert_eq!(stepped, 100);
    let stats = *app.world().resource::<AiLogicalStats>();
    assert_eq!((stats.states, stats.stepped, stats.queued), (100, 0, 0));
}

/// The per-tick stand-in for a tier-2 mover: walks its served `GoTo` at [`SPEED`] every tick and
/// finishes it on the tick it arrives.
#[derive(Resource)]
struct Walker {
    key: ActorKey,
    pos: Location,
    walking: Option<(DirectiveId, RouteRef, f64)>,
    ends: Vec<(DirectiveDone, GameInstant)>,
}

fn walker_system(mut w: ResMut<Walker>, mut directives: ResMut<AiDirectives>, clock: Res<Clock>) {
    let now = clock.0.now;
    let minute = GameTime::from_instant(now);
    let Walker { key, pos, walking, ends } = &mut *w;
    let Some(stack) = directives.get_mut(*key) else { return };
    let mut events = Vec::new();
    stack.advance(minute, &ModernPolicy, &mut events);
    let served = stack.serving().map(|d| (d.id, d.spec.kind.clone()));
    if walking.as_ref().map(|w| w.0) != served.as_ref().map(|s| s.0) {
        *walking = match served {
            Some((id, DirectiveKind::GoTo(to))) => Env.route(*key, pos, &to).map(|r| (id, r, 0.0)),
            _ => None,
        };
    }
    if let Some((id, route, along)) = walking {
        *along += f64::from(SPEED) / HZ;
        #[expect(clippy::cast_possible_truncation, reason = "metres")]
        let m = *along as f32;
        *pos = Env.point_on(route, m.min(route.length_m));
        if *along + 1e-6 >= f64::from(route.length_m) {
            stack.finish(*id, minute, &ModernPolicy, &mut events);
        }
    }
    ends.extend(events.into_iter().map(|e| (e, now)));
}

#[test]
fn tier_zero_and_a_per_tick_walker_complete_the_same_go_to_in_the_same_minute() {
    let mut app = app(BIG, Views { seen_x: 1e9 });
    let (logical, ticked) = (ActorKey(1), ActorKey(2));
    let goal = loc(60.0, 0.0);
    for key in [logical, ticked] {
        stack_with(&mut app, key, DirectiveKind::GoTo(goal.clone()), DirectiveSource::Quest);
    }
    store_state(&mut app, logical, loc(0.0, 0.0));
    app.insert_resource(Walker { key: ticked, pos: loc(0.0, 0.0), walking: None, ends: Vec::new() })
        .add_systems(FixedUpdate, walker_system.in_set(AiLogicalSystems::Step));
    // 60 m at 2 m/s = 30 s simulated = 15 game minutes at x30: 1800 ticks.
    for _ in 0..1900 {
        step(&mut app);
    }
    let summary = |v: &[(DirectiveDone, GameInstant)]| v.iter().map(|(e, _)| (e.id, e.end, e.at)).collect::<Vec<_>>();
    let ends0 = summary(&app.world().resource::<Ends>().0);
    let ends2 = summary(&app.world().resource::<Walker>().ends);
    assert_eq!(ends0, vec![(DirectiveId(0), DirectiveEnd::Completed, GameTime::from_minutes(75))]);
    assert_eq!(ends0, ends2);
}

#[test]
fn a_follower_stands_where_its_leader_is_after_the_same_tick() {
    let mut app = app(BIG, Views { seen_x: 1e9 });
    // The follower's key sorts first: key order alone would step it before its leader.
    let (follower, leader) = (ActorKey(2), ActorKey(9));
    stack_with(&mut app, leader, DirectiveKind::GoTo(loc(100.0, 0.0)), DirectiveSource::Quest);
    stack_with(&mut app, follower, DirectiveKind::Follow(leader), DirectiveSource::Quest);
    // The leader was last stepped a game minute ago: this tick's step walks it 2 s x 2 m/s on.
    let earlier = GameInstant(now(&app).0 - 60_000);
    app.world_mut().resource_mut::<AiLogical>().insert(leader, LogicalState::new(loc(0.0, 0.0), earlier));
    store_state(&mut app, follower, loc(0.0, 0.0));
    step(&mut app);
    let t = now(&app);
    let store = app.world().resource::<AiLogical>();
    let (f, l) = (store.get(follower).unwrap(), store.get(leader).unwrap());
    // Both stepped this tick, the leader first: the follower stands on the leader's new spot.
    assert_eq!((f.at, l.at, f.follow), (t, t, Some(leader)));
    assert_eq!(f.pos, l.pos);
    assert!(f.pos.pos.delta(loc(0.0, 0.0).pos).length() > 4.0, "{:?}", f.pos);
    // Between events, the follower reads its leader through the store.
    for _ in 0..3 {
        step(&mut app);
        let t = now(&app);
        let store = app.world().resource::<AiLogical>();
        assert_eq!(store.position_at(follower, t, &Env, TS), store.position_at(leader, t, &Env, TS));
    }
}

/// A held body (marker + stored state) at `body_x`, its state's spot at `spot_x`.
fn held(app: &mut App, key: u128, body_x: f32, spot_x: f64) -> Entity {
    let e = app.world_mut().spawn((AiActor(ActorKey(key)), mover(Vec3::new(body_x, 0.0, 0.0), 0.0), AiLogicalBody)).id();
    store_state(app, ActorKey(key), loc(spot_x, 0.0));
    e
}

#[test]
fn a_queued_actor_stays_tier_zero_and_sight_skips_it() {
    let mut app = sight_tests::app(HZ);
    sight_tests::every_tick(&mut app, HZ);
    let mut app = with_shell(app, BIG, Views { seen_x: 5.0 });
    sight_tests::target(&mut app, Vec3::new(0.0, 0.0, 5.0));
    // Held: its state stands at x = 10, in view, away from its body: it waits.
    let queued = sight_tests::observer(&mut app, 1, Vec3::ZERO);
    app.world_mut().entity_mut(queued).insert(AiLogicalBody);
    store_state(&mut app, ActorKey(1), loc(10.0, 0.0));
    let free = sight_tests::observer(&mut app, 2, Vec3::new(0.5, 0.0, 0.0));
    for _ in 0..30 {
        step(&mut app);
        assert_eq!(*app.world().get::<AiTier>(queued).unwrap(), AiTier { tier: Tier::Logical, first_decide_tick: None });
        let s = *app.world().resource::<AiLogicalStats>();
        assert_eq!((s.states, s.queued, s.waiting), (1, 1, 1), "{s:?}");
    }
    assert_eq!(app.world().get::<AiTier>(free).unwrap().tier, Tier::Full);
    assert!(sight_tests::level(&app, free).is_some_and(|l| l > 0.0));
    assert!(sight_tests::level(&app, queued).is_none_or(|l| l == 0.0));
    assert!(app.world().resource::<Placed>().0.is_empty());
}

#[test]
fn promotions_follow_the_per_tick_budget() {
    let mut app = app(BIG, Views { seen_x: 0.0 });
    sight_tests::target(&mut app, Vec3::ZERO);
    let bodies: Vec<Entity> = (1..=10u8).map(|i| held(&mut app, u128::from(i), -f32::from(i), -50.0 - f64::from(i))).collect();
    let mut left = Vec::new();
    for _ in 0..4 {
        step(&mut app);
        left.push(10 - app.world().resource::<AiLogical>().len());
    }
    assert_eq!(left, [4, 8, 10, 10]);
    let placed = &app.world().resource::<Placed>().0;
    let ticks: Vec<u64> = placed.iter().map(|p| p.3).collect();
    assert_eq!(ticks, [1, 1, 1, 1, 2, 2, 2, 2, 3, 3]);
    for (key, tier, p, tick) in placed {
        assert_eq!((*tier, p.entered_from), (Tier::Full, EnteredFrom::Spot), "{key:?}");
        let e = bodies[usize::try_from(key.0).unwrap() - 1];
        let t = *app.world().get::<AiTier>(e).unwrap();
        assert_eq!(t.tier, Tier::Full);
        assert!(t.first_decide_tick.is_some_and(|f| f > *tick), "{t:?} placed on {tick}");
        assert!(!app.world().entity(e).contains::<AiLogicalBody>());
        assert_eq!(app.world().get::<Mover>(e).unwrap().loco.position, p.at.pos);
    }
}

#[test]
fn a_body_on_its_spot_is_placed_in_view_at_once() {
    let mut app = app(BIG, Views { seen_x: -1e9 });
    sight_tests::target(&mut app, Vec3::ZERO);
    let on_spot = held(&mut app, 1, 3.0, 3.0);
    let _away = held(&mut app, 2, 4.0, 20.0);
    step(&mut app);
    let placed = &app.world().resource::<Placed>().0;
    assert_eq!(placed.len(), 1);
    let (key, _, p, _) = &placed[0];
    assert_eq!((*key, p.entered_from, &p.at), (ActorKey(1), EnteredFrom::Spot, &loc(3.0, 0.0)));
    assert!(!app.world().entity(on_spot).contains::<AiLogicalBody>());
    let s = *app.world().resource::<AiLogicalStats>();
    assert_eq!((s.states, s.queued, s.waiting, s.tried, s.placed), (1, 1, 1, 2, 1));
}

/// The game's demotion: a body the allocator put at tier 0 becomes a stored state.
fn demote_system(
    mut commands: Commands,
    mut store: ResMut<AiLogical>,
    clock: Res<Clock>,
    bodies: Query<(Entity, &AiActor, &AiTier, &Mover), Without<AiLogicalBody>>,
) {
    for (e, actor, tier, m) in &bodies {
        if tier.tier == Tier::Logical {
            demote_body(&mut commands, &mut store, e, actor.0, Location::new(ws(), m.loco.position), clock.0.now);
        }
    }
}

#[test]
fn demoted_at_its_body_then_promoted_again() {
    let mut app = app(TierCaps { full: 0, reduced: 0 }, Views { seen_x: 1e9 });
    app.add_systems(FixedUpdate, demote_system.in_set(AiLogicalSystems::Step).before(step_system));
    sight_tests::target(&mut app, Vec3::ZERO);
    let e = app.world_mut().spawn((AiActor(ActorKey(5)), mover(Vec3::new(7.0, 0.0, 0.0), 0.0))).id();
    step(&mut app);
    let at = app.world().resource::<AiLogical>().get(ActorKey(5)).map(|s| s.pos.clone());
    assert_eq!(at, Some(loc(7.0, 0.0)));
    assert!(app.world().entity(e).contains::<AiLogicalBody>());
    step(&mut app);
    assert_eq!(app.world().get::<AiTier>(e).unwrap().tier, Tier::Logical);
    assert!(app.world().resource::<AiPromoteQueue>().is_empty());

    app.world_mut().resource_mut::<AiAllocSettings>().set_caps(BIG);
    step(&mut app);
    assert!(app.world().resource::<AiLogical>().is_empty());
    assert!(!app.world().entity(e).contains::<AiLogicalBody>());
    assert_eq!(app.world().get::<AiTier>(e).unwrap().tier, Tier::Full);
    let placed = &app.world().resource::<Placed>().0;
    assert_eq!(placed.iter().map(|p| (p.0, p.2.entered_from)).collect::<Vec<_>>(), [(ActorKey(5), EnteredFrom::Spot)]);
}

#[test]
fn ai_stats_prints_the_tier_zero_line() {
    let mut app = App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_console_bevy::ConsolePlugin, AiPlugin { timings: false }));
    app.update();
    app.world_mut().resource_mut::<Time<bevy::time::Virtual>>().pause();
    let mut app = with_shell(app, BIG, Views { seen_x: 5.0 });
    sight_tests::target(&mut app, Vec3::ZERO);
    for key in [ActorKey(1), ActorKey(2)] {
        stack_with(&mut app, key, stay_near(loc(-30.0, 0.0)), DirectiveSource::Record);
        store_state(&mut app, key, loc(-30.0, 0.0));
    }
    // No stack: skipped by the step; its spot is in view: it waits.
    held(&mut app, 3, 0.0, 10.0);
    step(&mut app);
    let lines = tests::run(&mut app, "ai.stats");
    assert_eq!(
        tests::info(&lines).last().copied(),
        Some("logical: 3 states, 2 stepped; promote: queue 1, waiting 1, tried 1, placed 0, timings off")
    );
}

#[test]
fn ai_stats_prints_the_bodiless_line_only_when_the_game_inserts_it() {
    let mut app = App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_console_bevy::ConsolePlugin, AiPlugin { timings: false }));
    app.update();
    let last = |app: &mut App| tests::info(&tests::run(app, "ai.stats")).last().map(|l| (*l).to_owned());
    assert!(last(&mut app).is_some_and(|l| l.starts_with("logical: ")), "no bodiless line without the resource");
    app.insert_resource(AiBodilessStats { spawned: 2, moved: 5 });
    assert_eq!(last(&mut app).as_deref(), Some("bodiless: 2 bodies spawned into loaded cells, 5 kept placements moved"));
}

#[test]
fn promote_per_tick_is_live() {
    assert_eq!(AiPromoteSettings::default().config, PromoteConfig::default());
    assert_eq!(PROMOTE_TUNABLES.len(), 4);
    let mut app = App::new();
    app.add_plugins((bevy::MinimalPlugins, sf_tunables_bevy::TunablesPlugin));
    app.insert_resource(Time::<Fixed>::from_hz(HZ));
    let mut app = with_shell(app, BIG, Views { seen_x: 0.0 });
    app.update();
    app.world_mut().resource_mut::<Time<bevy::time::Virtual>>().pause();
    app.world_mut().write_message(sf_tunables_bevy::SetTunable::new(
        "ai.tier.promote_per_tick",
        "1",
        sf_tunables_bevy::SetBy::Console,
    ));
    app.update();
    assert_eq!(app.world().resource::<AiPromoteSettings>().config.per_tick, 1);
    sight_tests::target(&mut app, Vec3::ZERO);
    for i in 1..=3u8 {
        held(&mut app, u128::from(i), -f32::from(i), -20.0 - f64::from(i));
    }
    let mut left = Vec::new();
    for _ in 0..3 {
        step(&mut app);
        left.push(app.world().resource::<AiLogical>().len());
    }
    assert_eq!(left, [2, 1, 0]);
}

/// `MA22b`: `sf/ai` v2 keeps the tier-0 store. Walkers mid-route, one standing actor and one woken
/// after a push from outside the step save and load into a fresh app with the same clock: the
/// states, their `until` and the due index read back exactly, and both apps then finish the same
/// directives at the same instants in the same order.
#[test]
fn a_saved_tier_zero_store_loads_exactly_and_carries_on_identically() {
    let mut a = app(BIG, Views { seen_x: 1e9 });
    for (i, x) in [(1u128, 40.0), (2, 90.0), (3, 25.0)] {
        let key = ActorKey(i);
        stack_with(&mut a, key, DirectiveKind::GoTo(loc(x, 0.0)), DirectiveSource::Quest);
        store_state(&mut a, key, loc(0.0, 0.0));
    }
    let stand = ActorKey(4);
    stack_with(&mut a, stand, stay_near(loc(5.0, 5.0)), DirectiveSource::Record);
    store_state(&mut a, stand, loc(5.0, 5.0));
    for _ in 0..300 {
        step(&mut a);
    }
    // A push from outside the step, then a wake: due at once although its state has no `until`.
    let m = minute(&a);
    a.world_mut().resource_mut::<AiDirectives>().get_mut(stand).unwrap().push(
        DirectiveSpec::new(DirectiveKind::GoTo(loc(-30.0, 5.0)), DirectiveSource::Script).priority(9),
        m,
        &ModernPolicy,
        &mut Vec::new(),
    );
    assert!(a.world_mut().resource_mut::<AiLogical>().wake(stand));

    let saved = ai_save(a.world()).unwrap();
    assert_eq!(saved.actors.len(), 4);
    let woken = saved.actors[&stand].logical.as_ref().unwrap();
    assert_eq!(woken.due, Some(GameInstant::ZERO));
    assert_eq!(woken.state.until, None, "a plain wait: nothing due but the wake");
    let walker = saved.actors[&ActorKey(2)].logical.as_ref().unwrap();
    assert!(walker.state.route.is_some() && walker.state.until == walker.due, "{walker:?}");

    let mut b = app(BIG, Views { seen_x: 1e9 });
    b.world_mut().resource_mut::<Clock>().0 = a.world().resource::<Clock>().0;
    // Stale state the load must replace.
    store_state(&mut b, ActorKey(99), loc(1.0, 1.0));
    // (The codec's byte round trip is `sf_ai`'s and `sf_save_bevy`'s test.)
    ai_load(b.world_mut(), saved.clone());
    // Every state with its due instant (the due index is `(due, key)` of exactly these).
    assert_eq!(ai_save(b.world()), Some(saved.clone()));
    assert!(!b.world().resource::<AiLogical>().contains(ActorKey(99)));

    a.world_mut().resource_mut::<Ends>().0.clear();
    for _ in 0..3000 {
        step(&mut a);
        step(&mut b);
    }
    let ends = |app: &App| app.world().resource::<Ends>().0.iter().map(|(e, at)| (e.actor, e.id, e.end, e.at, *at)).collect::<Vec<_>>();
    assert_eq!(ends(&b), ends(&a));
    assert_eq!(ends(&a).len(), 4, "three walkers arrive, the pushed GoTo completes: {:?}", ends(&a));
    assert_eq!(ai_save(b.world()), ai_save(a.world()));
}

// --- Time skip (MA7b) ---------------------------------------------------------------------------

/// Records which actors the rules saw, in order, and moves every `StayNear` state 1 m along x.
#[derive(Default)]
struct Nudge(std::sync::Mutex<Vec<ActorKey>>);

impl FastForwardRules for Nudge {
    fn after_fast_forward(
        &self,
        state: &mut LogicalState,
        stack: &mut DirectiveStack,
        _span: (GameInstant, GameInstant),
        _ctx: &LogicalCtx<'_>,
        _events: &mut Vec<DirectiveDone>,
    ) {
        self.0.lock().unwrap().push(stack.actor());
        if matches!(
            stack.top().map(|d| &d.spec.kind),
            Some(DirectiveKind::StayNear { .. })
        ) {
            state.pos = Location::new(ws(), state.pos.pos.offset(Vec3::X));
        }
    }
}

#[test]
fn a_skip_crosses_every_stored_state_not_only_due_ones() {
    let mut app = app(BIG, Views { seen_x: 1e9 });
    let (walker, waiter, bare) = (ActorKey(1), ActorKey(2), ActorKey(3));
    stack_with(
        &mut app,
        walker,
        DirectiveKind::GoTo(loc(100.0, 0.0)),
        DirectiveSource::Record,
    );
    store_state(&mut app, walker, loc(0.0, 0.0));
    stack_with(
        &mut app,
        waiter,
        stay_near(loc(0.0, 10.0)),
        DirectiveSource::Record,
    );
    store_state(&mut app, waiter, loc(0.0, 10.0));
    store_state(&mut app, bare, loc(5.0, 5.0));
    // One tick: both take up their directive; the walk is due at its arrival (25 game min on).
    step(&mut app);
    let from = now(&app);
    let to = from.plus(3_600_000);
    let tr = LogicalTransitions::new();
    let ctx = LogicalCtx {
        env: &Env,
        transitions: &tr,
        policy: &ModernPolicy,
        time_scale: TS,
    };
    let rules = Nudge::default();
    let mut events = Vec::new();
    let world = app.world_mut();
    let mut directives = world.remove_resource::<AiDirectives>().unwrap();
    let mut store = world.resource_mut::<AiLogical>();
    let stats = fast_forward_logical(
        &mut store,
        &mut directives,
        (from, to),
        &ctx,
        &rules,
        &mut events,
    );

    assert_eq!((stats.states, stats.stepped), (3, 2), "{stats:?}");
    assert_eq!(
        *rules.0.lock().unwrap(),
        [walker, waiter],
        "key order; the actor without a stack is skipped"
    );
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(
        (events[0].actor, events[0].end),
        (walker, DirectiveEnd::Completed)
    );
    let at = |k: ActorKey| store.get(k).unwrap().position_at(to, &Env, TS);
    assert!(
        at(walker).pos.delta(loc(100.0, 0.0).pos).length() < 1e-6,
        "{:?}",
        at(walker)
    );
    assert!(
        at(waiter).pos.delta(loc(1.0, 10.0).pos).length() < 1e-6,
        "{:?}",
        at(waiter)
    );
    assert!(at(bare).pos.delta(loc(5.0, 5.0).pos).length() < 1e-6);
    for (key, saved) in store.to_saved() {
        if key != bare {
            assert_eq!(
                saved.due, saved.state.until,
                "{key:?}: the due index is rebuilt"
            );
        }
        assert!(saved.state.at <= to, "{key:?}");
    }
}

#[test]
fn skips_merge_into_one_span_until_taken() {
    let (a, b, c) = (
        GameInstant::from_minutes(60),
        GameInstant::from_minutes(120),
        GameInstant::from_minutes(180),
    );
    let mut skips = AiTimeSkips::default();
    skips.record(b, a);
    assert_eq!(skips.pending(), None, "no skip backwards");
    skips.record(a, b);
    skips.record(b, c);
    assert_eq!(skips.take(), Some((a, c)));
    assert_eq!(skips.take(), None);
}

/// The promote system's arm on the tick after a skip: [`AiTierZero::promote_dark`], nothing seen.
#[derive(Resource)]
struct Dark;

fn dark_promote_system(
    mut t0: AiTierZero,
    clock: Res<Clock>,
    mut placed: ResMut<Placed>,
    alloc: Res<AiAllocStats>,
    mut bodies: Query<(&AiActor, &mut Mover)>,
) {
    let now = clock.0.now;
    let body_at = |k: ActorKey| {
        bodies
            .iter()
            .find(|(a, _)| a.0 == k)
            .map(|(_, m)| Location::new(ws(), m.loco.position))
    };
    let out = t0.promote_dark(now, TS, &Env, &Views { seen_x: 1e9 }, &body_at);
    for (key, tier, p) in out.placed {
        if let Some((_, mut m)) = bodies.iter_mut().find(|(a, _)| a.0 == key) {
            m.loco.position = p.at.pos;
        }
        placed.0.push((key, tier, p, alloc.tick));
    }
}

#[test]
fn in_the_dark_every_waiting_entry_is_placed_in_one_tick() {
    let mut app = app(BIG, Views { seen_x: -1e9 });
    sight_tests::target(&mut app, Vec3::ZERO);
    let bodies: Vec<Entity> = (1..=10u8)
        .map(|i| held(&mut app, u128::from(i), -f32::from(i), -50.0 - f64::from(i)))
        .collect();
    for _ in 0..3 {
        step(&mut app);
    }
    let s = *app.world().resource::<AiLogicalStats>();
    assert_eq!(
        (s.states, s.queued, s.placed),
        (10, 10, 0),
        "every spot is seen: {s:?}"
    );
    assert!(s.waiting > 0, "{s:?}");
    app.insert_resource(Dark).add_systems(
        FixedUpdate,
        dark_promote_system
            .in_set(AiLogicalSystems::Promote)
            .run_if(resource_exists::<Dark>)
            .after(promote_system),
    );
    step(&mut app);
    assert!(
        app.world().resource::<AiLogical>().is_empty(),
        "all ten placed on the dark tick"
    );
    let placed = &app.world().resource::<Placed>().0;
    assert_eq!(placed.len(), 10);
    for (key, _, p, _) in placed {
        let e = bodies[usize::try_from(key.0).unwrap() - 1];
        assert_eq!(
            app.world().get::<Mover>(e).unwrap().loco.position,
            p.at.pos,
            "{key:?}"
        );
        assert!(
            p.at.pos
                .delta(loc(-50.0 - f64::from(u8::try_from(key.0).unwrap()), 0.0).pos)
                .length()
                < 1e-6,
            "{key:?} on its state's spot"
        );
    }
}
