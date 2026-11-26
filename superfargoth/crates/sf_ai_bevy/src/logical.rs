//! The tier-0 shell on the fixed tick (slice `AI10c1`; `docs/specs/ai.md` "Tiers and budget"):
//! the store of logical states, the step and promote helpers that run `sf_ai`'s cores
//! ([`LogicalState::step`], [`PromoteQueue`], [`resolve_promotion`]), the hold that keeps a
//! queued actor at tier 0, the `ai.tier.promote_*` tunables and the stats `ai.stats` prints.
//!
//! **Three checks.** Bevy: resources and fixed-tick system sets; nothing native for agent LOD.
//! Crates: none (no Mass-like LOD crate for Bevy; the AI crates are decision libraries). Industry:
//! UE5 Mass (the "Off" LOD keeps the fragments, representation actors spawn under a per-frame
//! budget), KCD2 (LOD-unaware scripts), AC Origins' Meta AI (virtual NPCs become real out of
//! sight).
//!
//! - **Store** ([`AiLogical`]): every tier-0 state by [`ActorKey`]. **An actor is tier 0 in fact
//!   iff its key is in it**, whether or not it has a body. A spawned body of a stored actor
//!   carries [`AiLogicalBody`] (the game's walker stands down on it); the game inserts both
//!   through [`demote_body`] when the allocator gives the actor tier 0.
//! - **Hold** ([`AiLogicalSystems::Hold`], right after the allocator): every held body's
//!   [`AiTier`] reads `{ Logical, None }` (sight, hearing and the planner treat it as tier 0),
//!   and each one the allocator granted tier 1 or 2 becomes a [`PromoteRequest`] with the
//!   allocator's own distance and pin, synced into [`AiPromoteQueue`].
//! - **Step** ([`AiLogicalSystems::Step`], the game's system): [`AiTierZero::step`] (or the plain
//!   [`step_logical`]) steps the states that are new or whose `until` is due, leaders before
//!   followers, each group in key order; O(due states) through a due index. A follower reads its
//!   leader through the store (this tick's state), the game's env only for bodies.
//! - **Promote** ([`AiLogicalSystems::Promote`], the game's system): [`AiTierZero::promote`] (or
//!   [`promote_logical`]) runs the queue; a body already on its state's spot is placed there with
//!   no view test (nothing moves), else [`resolve_promotion`]. A placed actor leaves the store;
//!   the game moves its body to the placement.
//! - **Cleanup** ([`AiLogicalSystems::Cleanup`]): a body whose key left the store loses its
//!   marker and gets the allocated tier back, with a staggered `first_decide_tick`.
//!
//! Host only (tiers are host-authoritative). The store is plain data (MA22 saves it).

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};
use std::time::Instant;

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_time::{Fixed, Time};
use sf_ai::{
    ActivityRef, ActorKey, DirectiveDone, DirectiveStack, EnteredFrom, GameTime, LogicalCtx, LogicalEnv, LogicalState, Placement,
    PromoteConfig, PromoteEnv, PromoteQueue, PromoteRequest, PromoteTick, Resolve, RouteRef, StackPolicy, Tier, Transition,
    TransitionInput, first_decide_after, resolve_promotion,
};
use sf_controller_bevy::ControllerSystems;
use sf_ids::GameInstant;
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Type, Unit, def};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::{Location, PlacementId};

use crate::alloc::{AiAllocSettings, AiAllocStats, AiAllocSystems, AiTier, AllocState};
use crate::sight::AiPerceptionSystems;
use crate::{AiActor, AiDirectives};

// --- Tunables ---------------------------------------------------------------------------------

/// Gameplay metadata shared by every promotion tunable (host-authoritative, dev scope, live).
const fn gp<T: Type>(d: Def<T>) -> Def<T> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

pub const PROMOTE_PER_TICK: Def<u32> = gp(def!(count "ai.tier.promote_per_tick" = 4)
    .range(1, 64)
    .unit(Unit::Count)
    .help("Promotion tries per tick: one navmesh snap plus its view tests each (ai.md Guess 4, MassLOD spawn budget)"));
pub const PROMOTE_RETRY_S: Def<f32> = gp(def!(float "ai.tier.promote_retry_s" = 0.5)
    .range(0.1, 5.0)
    .unit(Unit::Seconds)
    .help("Seconds before a promotion that found every spot seen is tried again"));
pub const PROMOTE_BACKTRACK_M: Def<f32> = gp(def!(float "ai.tier.promote_backtrack_m" = 40.0)
    .range(0.0, 200.0)
    .unit(Unit::Meters)
    .help("How far back along its route a promoted actor may appear, to walk into view (GTA/RDR on-path spawn)"));
pub const PROMOTE_BACKTRACK_STEP_M: Def<f32> = gp(def!(float "ai.tier.promote_backtrack_step_m" = 4.0)
    .range(1.0, 20.0)
    .unit(Unit::Meters)
    .help("Spacing of the points tried back along the route"));

/// The promotion tunables (`ai.tier.promote_*`), bound by [`add_promote_tunables`].
pub const PROMOTE_TUNABLES: &[&dyn AnyDef] =
    &[&PROMOTE_PER_TICK, &PROMOTE_RETRY_S, &PROMOTE_BACKTRACK_M, &PROMOTE_BACKTRACK_STEP_M];

/// The promotion settings, bound from [`PROMOTE_TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AiPromoteSettings {
    pub config: PromoteConfig,
}

impl Default for AiPromoteSettings {
    fn default() -> Self {
        Self {
            config: PromoteConfig {
                per_tick: PROMOTE_PER_TICK.default_value(),
                retry_s: PROMOTE_RETRY_S.default_value(),
                backtrack_m: PROMOTE_BACKTRACK_M.default_value(),
                backtrack_step_m: PROMOTE_BACKTRACK_STEP_M.default_value(),
            },
        }
    }
}

/// Registers [`PROMOTE_TUNABLES`] and binds them to [`AiPromoteSettings`] (the caller inserts the
/// resource; [`AiLogicalPlugin`] does).
pub fn add_promote_tunables(app: &mut App) {
    let _ = app.add_tunable(&PROMOTE_PER_TICK).bind_resource(|s: &mut AiPromoteSettings, v| s.config.per_tick = v);
    let _ = app.add_tunable(&PROMOTE_RETRY_S).bind_resource(|s: &mut AiPromoteSettings, v| s.config.retry_s = v);
    let _ = app.add_tunable(&PROMOTE_BACKTRACK_M).bind_resource(|s: &mut AiPromoteSettings, v| s.config.backtrack_m = v);
    let _ = app
        .add_tunable(&PROMOTE_BACKTRACK_STEP_M)
        .bind_resource(|s: &mut AiPromoteSettings, v| s.config.backtrack_step_m = v);
}

// --- Store ------------------------------------------------------------------------------------

/// One stored actor: its state (taken out only while it steps) and its key in the due index.
#[derive(Clone, Debug)]
struct Slot {
    state: Option<LogicalState>,
    due: Option<GameInstant>,
}

/// Every tier-0 state by [`ActorKey`] (like [`AiDirectives`]): an actor is tier 0 in fact iff its
/// key is here. A due index keeps [`step_logical`] O(due states): a state is due when new
/// (serving nothing yet) or at its `until`.
#[derive(Resource, Clone, Debug, Default)]
pub struct AiLogical {
    states: BTreeMap<ActorKey, Slot>,
    /// `(due instant, key)` of every state with something due.
    due: BTreeSet<(GameInstant, ActorKey)>,
    /// [`step_logical`]'s due keys of one call (reused).
    scratch: Vec<ActorKey>,
}

impl AiLogical {
    /// Adds or replaces `key`'s state; returns the old one. A state serving nothing yet is due at
    /// once, any other at its `until`.
    pub fn insert(&mut self, key: ActorKey, state: LogicalState) -> Option<LogicalState> {
        let due = if state.directive.is_none() { Some(GameInstant::ZERO) } else { state.until };
        let old = self.states.insert(key, Slot { state: Some(state), due });
        if let Some(d) = old.as_ref().and_then(|s| s.due) {
            self.due.remove(&(d, key));
        }
        if let Some(d) = due {
            self.due.insert((d, key));
        }
        old.and_then(|s| s.state)
    }

    /// A granted demotion ([`sf_ai::demote`]): the body at `body` becomes tier 0 at `now`. Does
    /// nothing (false) when `key` is tier 0 already.
    pub fn demote(&mut self, key: ActorKey, body: Location, now: GameInstant) -> bool {
        if self.contains(key) {
            return false;
        }
        self.insert(key, sf_ai::demote(body, now));
        true
    }

    /// Steps `key` on the next [`step_logical`] (call it after changing a tier-0 actor's stack
    /// from outside the step: a push, a cancel). False when `key` is not stored.
    pub fn wake(&mut self, key: ActorKey) -> bool {
        let Some(slot) = self.states.get_mut(&key) else { return false };
        if let Some(d) = slot.due {
            self.due.remove(&(d, key));
        }
        slot.due = Some(GameInstant::ZERO);
        self.due.insert((GameInstant::ZERO, key));
        true
    }

    pub fn get(&self, key: ActorKey) -> Option<&LogicalState> {
        self.states.get(&key).and_then(|s| s.state.as_ref())
    }

    /// Drops `key` from tier 0 (promoted, or deleted from the world).
    pub fn remove(&mut self, key: ActorKey) -> Option<LogicalState> {
        let slot = self.states.remove(&key)?;
        if let Some(d) = slot.due {
            self.due.remove(&(d, key));
        }
        slot.state
    }

    pub fn contains(&self, key: ActorKey) -> bool {
        self.states.contains_key(&key)
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// In key order.
    pub fn iter(&self) -> impl Iterator<Item = (ActorKey, &LogicalState)> {
        self.states.iter().filter_map(|(k, s)| s.state.as_ref().map(|s| (*k, s)))
    }

    /// Where `key` is at `t` ([`LogicalState::position_at`]), a follower reading its leader's
    /// state from this store (`env` only for actors not stored: bodies at tiers 1-2).
    pub fn position_at(&self, key: ActorKey, t: GameInstant, env: &dyn LogicalEnv, time_scale: f64) -> Option<Location> {
        let store_env = StoreEnv::new(env, &self.states, time_scale);
        self.get(key).map(|s| s.position_at(t, &store_env, time_scale))
    }
}

// tunables: const (capacity: follow chains read through the store, deeper ones read the host env)
const MAX_FOLLOW_DEPTH: u8 = 8;

/// The host's [`LogicalEnv`] with other actors' positions read from the store first, so a
/// follower stands where its leader's state (this tick's, once stepped) puts it. A cycle or a
/// chain past [`MAX_FOLLOW_DEPTH`] falls back to the host.
struct StoreEnv<'a> {
    inner: &'a dyn LogicalEnv,
    states: &'a BTreeMap<ActorKey, Slot>,
    time_scale: f64,
    depth: Cell<u8>,
}

impl<'a> StoreEnv<'a> {
    fn new(inner: &'a dyn LogicalEnv, states: &'a BTreeMap<ActorKey, Slot>, time_scale: f64) -> Self {
        Self { inner, states, time_scale, depth: Cell::new(0) }
    }
}

impl LogicalEnv for StoreEnv<'_> {
    fn route(&self, actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
        self.inner.route(actor, from, to)
    }

    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
        self.inner.point_on(route, progress_m)
    }

    fn placement(&self, id: PlacementId) -> Option<Location> {
        self.inner.placement(id)
    }

    fn actor_position(&self, actor: ActorKey, at: GameInstant) -> Option<Location> {
        let depth = self.depth.get();
        if depth < MAX_FOLLOW_DEPTH
            && let Some(state) = self.states.get(&actor).and_then(|s| s.state.as_ref())
        {
            self.depth.set(depth + 1);
            let p = state.position_at(at, self, self.time_scale);
            self.depth.set(depth);
            return Some(p);
        }
        self.inner.actor_position(actor, at)
    }

    fn next_window_edge(&self, actor: ActorKey, after: GameInstant) -> Option<GameInstant> {
        self.inner.next_window_edge(actor, after)
    }

    fn on_window_edge(
        &self,
        actor: ActorKey,
        at: GameTime,
        stack: &mut DirectiveStack,
        policy: &dyn StackPolicy,
        events: &mut Vec<DirectiveDone>,
    ) {
        self.inner.on_window_edge(actor, at, stack, policy, events);
    }
}

// --- Helpers ----------------------------------------------------------------------------------

/// One [`step_logical`] call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LogicalStepStats {
    /// States stored.
    pub states: usize,
    /// States stepped (due ones with a stack).
    pub stepped: usize,
    /// Events the cores stepped through ([`LogicalState::step`]'s count).
    pub events: usize,
}

/// Steps every due state to `now`: new ones (serving nothing yet) and those whose `until <=
/// now`; actors that follow nobody first, then followers (so a leader's position is this
/// tick's), each group in key order. An actor without a stack is skipped (it stays due).
/// `ctx.env` serves routes and bodies; other stored actors' positions come from the store.
pub fn step_logical(
    store: &mut AiLogical,
    directives: &mut AiDirectives,
    now: GameInstant,
    ctx: &LogicalCtx<'_>,
    events: &mut Vec<DirectiveDone>,
) -> LogicalStepStats {
    let AiLogical { states, due, scratch } = store;
    let mut out = LogicalStepStats { states: states.len(), ..LogicalStepStats::default() };
    scratch.clear();
    while let Some(&(at, key)) = due.first() {
        if at > now {
            break;
        }
        due.pop_first();
        scratch.push(key);
    }
    scratch.sort_unstable_by_key(|k| (follows(states, directives, ctx, *k), *k));

    for &key in scratch.iter() {
        let Some(slot) = states.get_mut(&key) else { continue };
        let Some(stack) = directives.get_mut(key) else {
            if let Some(d) = slot.due {
                due.insert((d, key));
            }
            continue;
        };
        let Some(mut state) = slot.state.take() else { continue };
        let n = {
            let env = StoreEnv::new(ctx.env, states, ctx.time_scale);
            state.step(stack, now, &LogicalCtx { env: &env, ..*ctx }, events)
        };
        out.stepped += 1;
        out.events += n;
        let next = state.until;
        if let Some(slot) = states.get_mut(&key) {
            slot.state = Some(state);
            slot.due = next;
        }
        if let Some(d) = next {
            due.insert((d, key));
        }
    }
    scratch.clear();
    out
}

/// Whether `key` steps as a follower: it follows someone, or the directive its next step takes
/// up (served, not yet served by the state: a new state, a changed stack) transitions to
/// `Follow`.
fn follows(states: &BTreeMap<ActorKey, Slot>, directives: &AiDirectives, ctx: &LogicalCtx<'_>, key: ActorKey) -> bool {
    let Some(state) = states.get(&key).and_then(|s| s.state.as_ref()) else { return false };
    match directives.get(key).and_then(DirectiveStack::serving) {
        Some(d) if Some(d.id) != state.directive => matches!(
            ctx.transitions.transition(&TransitionInput { actor: key, kind: &d.spec.kind, pos: &state.pos, env: ctx.env }),
            Transition::Follow(_)
        ),
        _ => state.follow.is_some(),
    }
}

/// A body already standing on its state's spot (same worldspace, within its body radius): placed
/// there with no view test, since nothing moves and nothing pops in.
fn on_body(
    actor: ActorKey,
    state: &LogicalState,
    now: GameInstant,
    time_scale: f64,
    env: &dyn LogicalEnv,
    world: &dyn PromoteEnv,
    body: Location,
) -> Option<Placement> {
    let spot = state.position_at(now, env, time_scale);
    let radius = f64::from(world.body_radius_m(actor));
    if spot.worldspace != body.worldspace || spot.pos.delta(body.pos).length() > radius {
        return None;
    }
    let activity = match state.activity {
        Some(ActivityRef::Use(id)) if !world.claim(actor, id) => Some(ActivityRef::Wait),
        other => other,
    };
    Some(Placement { at: body, progress_m: state.progress_at(now, time_scale), activity, entered_from: EnteredFrom::Spot })
}

/// Runs one tick of the promotion queue: for each try, a body on its state's spot is placed there
/// ([`EnteredFrom::Spot`], no view test), else [`resolve_promotion`] decides. Placed actors leave
/// the store; the caller moves their bodies, and [`AiLogicalSystems::Cleanup`] gives them their
/// allocated tier. `env` serves routes and bodies (stored actors read through the store);
/// `body_at` is where an actor's spawned body stands (`None`: no body).
#[expect(clippy::too_many_arguments, reason = "the queue's tick inputs plus the host's three seams")]
pub fn promote_logical(
    queue: &mut PromoteQueue,
    store: &mut AiLogical,
    tick: u64,
    tick_s: f32,
    config: &PromoteConfig,
    now: GameInstant,
    time_scale: f64,
    env: &dyn LogicalEnv,
    world: &dyn PromoteEnv,
    body_at: &dyn Fn(ActorKey) -> Option<Location>,
) -> PromoteTick {
    let out = {
        let store_env = StoreEnv::new(env, &store.states, time_scale);
        let states = &store.states;
        queue.run(tick, tick_s, config, |req| {
            let Some(state) = states.get(&req.actor).and_then(|s| s.state.as_ref()) else { return Resolve::Wait };
            if let Some(p) =
                body_at(req.actor).and_then(|body| on_body(req.actor, state, now, time_scale, &store_env, world, body))
            {
                return Resolve::Place(p);
            }
            resolve_promotion(req.actor, state, now, time_scale, &store_env, world, config)
        })
    };
    for (key, ..) in &out.placed {
        store.remove(*key);
    }
    out
}

/// Inserts `key`'s tier-0 state at its body's spot ([`AiLogical::demote`]) and marks the body
/// ([`AiLogicalBody`]): the game calls it when the allocator gives a body tier 0. Returns whether
/// the state was new.
pub fn demote_body(
    commands: &mut Commands,
    store: &mut AiLogical,
    entity: Entity,
    key: ActorKey,
    body: Location,
    now: GameInstant,
) -> bool {
    commands.entity(entity).insert(AiLogicalBody);
    store.demote(key, body, now)
}

// --- Components and resources -----------------------------------------------------------------

/// On an [`AiActor`] body whose key is in [`AiLogical`]: tier 0 in fact, the game's walker stands
/// down. Added by [`demote_body`], removed in [`AiLogicalSystems::Cleanup`] once the key left the
/// store.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiLogicalBody;

/// The promotion queue ([`PromoteQueue`]) and the hold's request buffer, reused every tick.
#[derive(Resource, Debug, Default)]
pub struct AiPromoteQueue {
    queue: PromoteQueue,
    requests: Vec<PromoteRequest>,
    timings: bool,
}

impl Deref for AiPromoteQueue {
    type Target = PromoteQueue;

    fn deref(&self) -> &PromoteQueue {
        &self.queue
    }
}

impl DerefMut for AiPromoteQueue {
    fn deref_mut(&mut self) -> &mut PromoteQueue {
        &mut self.queue
    }
}

/// This tick's tier-0 counts, for `ai.stats`.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct AiLogicalStats {
    /// States stored.
    pub states: usize,
    /// States stepped and core events this tick.
    pub stepped: usize,
    pub events: usize,
    /// Promotion queue entries after the tick, those waiting on a retry, tries, placements.
    pub queued: usize,
    pub waiting: usize,
    pub tried: u32,
    pub placed: usize,
    /// Microseconds of the promote call (`None` with timings off).
    pub promote_us: Option<f32>,
}

/// Fixed-tick tier-0 sets, chained after [`AiAllocSystems::Allocate`] and before
/// [`AiPerceptionSystems::Sight`] and `ControllerSystems::Drive`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiLogicalSystems {
    /// Held bodies read tier 0; the allocator's grants become promotion requests.
    Hold,
    /// The game's system calling [`AiTierZero::step`].
    Step,
    /// The game's system calling [`AiTierZero::promote`] and moving the placed bodies.
    Promote,
    /// Bodies whose key left the store get their allocated tier back.
    Cleanup,
}

/// The tier-0 shell: [`AiLogical`], [`AiPromoteQueue`], [`AiPromoteSettings`] (bound to
/// [`PROMOTE_TUNABLES`]), [`AiLogicalStats`], the sets and the hold and cleanup systems.
/// [`crate::AiPlugin`] adds it.
#[derive(Clone, Copy, Debug)]
pub struct AiLogicalPlugin {
    /// Whether [`AiLogicalStats::promote_us`] is measured.
    pub timings: bool,
}

impl Default for AiLogicalPlugin {
    fn default() -> Self {
        // tunables: const (switch the host passes, like AiPlugin::timings)
        Self { timings: true }
    }
}

impl Plugin for AiLogicalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiLogical>()
            .init_resource::<AiDirectives>()
            .init_resource::<AiLogicalStats>()
            .init_resource::<AiPromoteSettings>()
            .insert_resource(AiPromoteQueue { timings: self.timings, ..AiPromoteQueue::default() })
            .configure_sets(
                FixedUpdate,
                (AiLogicalSystems::Hold, AiLogicalSystems::Step, AiLogicalSystems::Promote, AiLogicalSystems::Cleanup)
                    .chain()
                    .after(AiAllocSystems::Allocate)
                    .before(AiPerceptionSystems::Sight)
                    .before(ControllerSystems::Drive),
            )
            .add_systems(
                FixedUpdate,
                (hold_logical.in_set(AiLogicalSystems::Hold), release_bodies.in_set(AiLogicalSystems::Cleanup)),
            );
        add_promote_tunables(app);
    }
}

/// What the game's Step and Promote systems take: the store, the stacks, the queue and its
/// settings, and the stats they fill.
#[derive(SystemParam)]
pub struct AiTierZero<'w> {
    store: ResMut<'w, AiLogical>,
    directives: ResMut<'w, AiDirectives>,
    queue: ResMut<'w, AiPromoteQueue>,
    settings: Res<'w, AiPromoteSettings>,
    stats: ResMut<'w, AiLogicalStats>,
    alloc: Option<Res<'w, AiAllocStats>>,
    fixed: Option<Res<'w, Time<Fixed>>>,
}

impl AiTierZero<'_> {
    pub fn store(&self) -> &AiLogical {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut AiLogical {
        &mut self.store
    }

    /// [`step_logical`] over the store and [`AiDirectives`], counted into [`AiLogicalStats`].
    pub fn step(&mut self, now: GameInstant, ctx: &LogicalCtx<'_>, events: &mut Vec<DirectiveDone>) -> LogicalStepStats {
        let s = step_logical(&mut self.store, &mut self.directives, now, ctx, events);
        self.stats.states = s.states;
        self.stats.stepped += s.stepped;
        self.stats.events += s.events;
        s
    }

    /// [`promote_logical`] on the allocator's tick with [`AiPromoteSettings`], counted (and timed
    /// with timings on) into [`AiLogicalStats`].
    pub fn promote(
        &mut self,
        now: GameInstant,
        time_scale: f64,
        env: &dyn LogicalEnv,
        world: &dyn PromoteEnv,
        body_at: &dyn Fn(ActorKey) -> Option<Location>,
    ) -> PromoteTick {
        let tick = self.alloc.as_deref().map_or(0, |a| a.tick);
        let tick_s = tick_seconds(self.fixed.as_deref());
        let config = self.settings.config;
        let AiPromoteQueue { queue, timings, .. } = &mut *self.queue;
        let started = (*timings).then(Instant::now);
        let out = promote_logical(queue, &mut self.store, tick, tick_s, &config, now, time_scale, env, world, body_at);
        let s = &mut *self.stats;
        s.promote_us = started.map(|t| t.elapsed().as_secs_f32() * MICROS_PER_S);
        s.states = self.store.len();
        s.tried += out.tried;
        s.placed += out.placed.len();
        s.waiting = out.waiting;
        s.queued = out.queued;
        out
    }
}

// --- Systems ----------------------------------------------------------------------------------

// tunables: const (maths: seconds to microseconds)
const MICROS_PER_S: f32 = 1e6;

/// A held body's tier.
// tunables: const (state: tier 0 has no decide cadence)
const HELD: AiTier = AiTier { tier: Tier::Logical, first_decide_tick: None };

fn tick_seconds(fixed: Option<&Time<Fixed>>) -> f32 {
    fixed.map_or(Time::<Fixed>::default().timestep(), Time::<Fixed>::timestep).as_secs_f32()
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn hold_logical(
    alloc: Option<Res<AllocState>>,
    store: Res<AiLogical>,
    mut queue: ResMut<AiPromoteQueue>,
    mut stats: ResMut<AiLogicalStats>,
    mut bodies: Query<(Entity, &AiActor, Option<&mut AiTier>), With<AiLogicalBody>>,
    mut commands: Commands,
) {
    let AiPromoteQueue { queue, requests, .. } = &mut *queue;
    requests.clear();
    for (entity, actor, tier) in &mut bodies {
        let key = actor.0;
        if !store.contains(key) {
            continue;
        }
        match tier {
            Some(mut t) => {
                t.set_if_neq(HELD);
            }
            None => {
                commands.entity(entity).insert(HELD);
            }
        }
        let Some(alloc) = alloc.as_deref() else { continue };
        let to = alloc.allocated(key);
        if to == Tier::Logical {
            continue;
        }
        let (nearest_player_m, pin) = alloc.input(key).map_or((f32::INFINITY, None), |i| (i.nearest_player_m, i.pin));
        requests.push(PromoteRequest { actor: key, to, nearest_player_m, pin });
    }
    queue.sync(requests);
    *stats = AiLogicalStats { states: store.len(), queued: queue.len(), ..AiLogicalStats::default() };
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn release_bodies(
    alloc: (Option<Res<AllocState>>, Option<Res<AiAllocSettings>>, Option<Res<AiAllocStats>>),
    fixed: Option<Res<Time<Fixed>>>,
    store: Res<AiLogical>,
    mut bodies: Query<(Entity, &AiActor, Option<&mut AiTier>), With<AiLogicalBody>>,
    mut commands: Commands,
) {
    let tick_s = tick_seconds(fixed.as_deref());
    for (entity, actor, tier) in &mut bodies {
        let key = actor.0;
        if store.contains(key) {
            continue;
        }
        commands.entity(entity).remove::<AiLogicalBody>();
        let (Some(granted_by), Some(settings), Some(clock)) = (alloc.0.as_deref(), alloc.1.as_deref(), alloc.2.as_deref()) else {
            // No allocator: the hold's tier 0 goes, the actor is untiered again.
            commands.entity(entity).remove::<AiTier>();
            continue;
        };
        let to = granted_by.allocated(key);
        let first_decide_tick = settings.config.decide_period_ticks(to, tick_s).map(|p| first_decide_after(key, clock.tick, p));
        let granted = AiTier { tier: to, first_decide_tick };
        match tier {
            Some(mut t) => {
                t.set_if_neq(granted);
            }
            None => {
                commands.entity(entity).insert(granted);
            }
        }
    }
}
