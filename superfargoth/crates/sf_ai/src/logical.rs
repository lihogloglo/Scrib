//! `AI10a` (tier-0 core): the logical state machine every actor outside the simulated set runs,
//! and [`fast_forward`] for rest, wait and long travel (`docs/specs/ai.md` "Tiers and budget").
//!
//! **Canonical technique:** an LOD-unaware schedule state machine with no planner, stepped at
//! events (KCD2's off-screen NPCs run the same scripts, only coarser; UE Mass's "Off" LOD keeps a
//! fragment and a timer; RDR2's ambient population advances along its schedule). No drives, HTN
//! or perception run: the served directive's kind gives the state through its registered
//! [`Transition`] (a route, an activity, following an actor, or a hold).
//!
//! - **State** ([`LogicalState`]): the spec's `(route, progress_m, activity, until)`, plus where the
//!   actor stands (`pos`) at its last step (`at`), the directive the state serves and, on a
//!   route, what arriving does. Plain serde data: `AI10c` saves it with the stack.
//! - **Events only:** [`LogicalState::step`] jumps from event to event up to `now`: an arrival, a
//!   [`Limit`](crate::Limit) running out ([`DirectiveStack::next_deadline`]) or a schedule window
//!   edge ([`LogicalEnv::next_window_edge`], AI11), so the cost is O(events), not O(ticks).
//!   `until` is the next of them, a [`GameInstant`] (game milliseconds): an arrival falls between
//!   minutes, and the stack's [`GameTime`] minute is taken from it the way every tier reads the
//!   clock (rounded down).
//! - **Tier invariance:** a route advances at the actor's walk speed in simulated seconds (game
//!   seconds over the clock's `time_scale`), the speed a tier-2 mover walks, and a directive the
//!   arrival fulfils ends at the arrival itself, not at the next event, so tier 0 and a per-tick
//!   executor end the same directives in the same order within one tick (`tests_logical`).
//! - **Per kind** ([`LogicalTransitions::transition`]): `StayNear` stays put within its radius
//!   (else walks home) and waits (no wander simulated); `GoTo` walks and ends `Completed` on
//!   arrival; `Follow` stands where the leader's own logical state puts it; `Escort` walks to its
//!   goal and ends on arrival (the escorted actor's `Follow` keeps it alongside); `Use` walks to
//!   the placement and holds the activity until its limit; `Fight` holds (combat is pinned at tier
//!   2 by the allocator's [`Pin::Combat`](crate::Pin), so tier 0 never runs it); `Custom` kinds
//!   use the content pack's registered transition. The built-in match is exhaustive, so a new
//!   built-in kind fails to compile without one; [`LogicalTransitions::validate`] fails at load
//!   for a custom kind nobody registered.
//! - **Seams:** [`LogicalEnv`] (the shell's routes on `WorldGraph` + the coarse route graph,
//!   placements, other actors' positions, schedule edges; `AI10c`) and [`FastForwardRules`] (the
//!   adapter's `fastForward` rules, MA7; [`ModernFastForward`] adds none).

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_ids::clock::{MS_PER_SECOND, hours_ms};
use sf_ids::{ActorKey, GameInstant};
use sf_records::RecordId;
use sf_rpg::GameTime;
use sf_world::{Location, PlacementId};

use crate::directive::{DirectiveDone, DirectiveKind, DirectiveStack, MAX_REACTIONS, StackPolicy};
use crate::ids::DirectiveId;

/// A route as the [`LogicalEnv`] planned it for one actor: enough to place the actor anywhere
/// along it ([`LogicalEnv::point_on`]) and to time its arrival.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteRef {
    pub from: Location,
    pub to: Location,
    /// Walk length, metres (the route's cost metres: what the walker covers at `speed_mps`).
    pub length_m: f32,
    /// The actor's walking speed on it, metres per simulated second.
    pub speed_mps: f32,
}

/// What an actor does in place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityRef {
    /// Stands (or sits) where it is.
    Wait,
    /// Uses a placement (a bench, a forge, a bed).
    Use(PlacementId),
}

/// What reaching the end of a route does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arrival {
    /// The directive is fulfilled: it ends `Completed` at the arrival.
    Finish,
    /// The actor takes up an activity there (until a limit or another event).
    Activity(ActivityRef),
}

/// A directive kind's tier-0 state (ai.md: "the stack top's kind gives the next state").
#[derive(Clone, Debug, PartialEq)]
pub enum Transition {
    /// Walk to `to`, then [`Arrival`].
    Travel { to: Location, arrive: Arrival },
    /// An activity where the actor stands.
    Activity(ActivityRef),
    /// Stand where `leader`'s own logical state puts it.
    Follow(ActorKey),
    /// Nothing moves (a kind tier 0 doesn't run, or a route the env can't plan).
    Hold,
}

/// What a transition sees: the actor, the served kind, where the actor stands, the world.
pub struct TransitionInput<'a> {
    pub actor: ActorKey,
    pub kind: &'a DirectiveKind,
    pub pos: &'a Location,
    pub env: &'a dyn LogicalEnv,
}

/// A content pack's tier-0 transition for one custom kind.
pub type CustomTransition = Box<dyn Fn(&TransitionInput<'_>) -> Transition + Send + Sync>;

/// A custom directive kind with no tier-0 transition: a load error.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("directive kind `{0}` has no tier-0 transition (register it in LogicalTransitions)")]
pub struct UnregisteredKind(pub RecordId);

/// The tier-0 transition of every kind: the built-ins' in code, the content pack's custom kinds
/// registered by record id (the kind's name).
#[derive(Default)]
pub struct LogicalTransitions {
    custom: BTreeMap<RecordId, CustomTransition>,
}

impl fmt::Debug for LogicalTransitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.custom.keys()).finish()
    }
}

impl LogicalTransitions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or replaces) the transition of custom kind `kind`.
    pub fn register(
        &mut self,
        kind: impl Into<RecordId>,
        transition: impl Fn(&TransitionInput<'_>) -> Transition + Send + Sync + 'static,
    ) {
        self.custom.insert(kind.into(), Box::new(transition));
    }

    pub fn contains(&self, kind: &RecordId) -> bool {
        self.custom.contains_key(kind)
    }

    /// The load-time rule: every kind content uses has a tier-0 transition. The first custom
    /// kind without one is the error.
    pub fn validate<'k>(&self, kinds: impl IntoIterator<Item = &'k DirectiveKind>) -> Result<(), UnregisteredKind> {
        for kind in kinds {
            if let DirectiveKind::Custom(id) = kind
                && !self.contains(id)
            {
                return Err(UnregisteredKind(id.clone()));
            }
        }
        Ok(())
    }

    /// The tier-0 state for `input.kind` (module docs, "Per kind").
    pub fn transition(&self, input: &TransitionInput<'_>) -> Transition {
        match input.kind {
            DirectiveKind::StayNear { home, radius } => {
                let near = input.pos.worldspace == home.worldspace
                    && input.pos.pos.delta(home.pos).length() <= f64::from(*radius);
                if near {
                    Transition::Activity(ActivityRef::Wait)
                } else {
                    Transition::Travel { to: home.clone(), arrive: Arrival::Activity(ActivityRef::Wait) }
                }
            }
            DirectiveKind::GoTo(to) | DirectiveKind::Escort { to, .. } => {
                Transition::Travel { to: to.clone(), arrive: Arrival::Finish }
            }
            DirectiveKind::Follow(leader) => Transition::Follow(*leader),
            DirectiveKind::Use(id) => match input.env.placement(*id) {
                Some(at) => Transition::Travel { to: at, arrive: Arrival::Activity(ActivityRef::Use(*id)) },
                None => Transition::Hold,
            },
            // Pinned at tier 2 (`Pin::Combat`): tier 0 never runs a fight.
            DirectiveKind::Fight(_) => Transition::Hold,
            // `validate` at load keeps this branch from meeting an unregistered kind.
            DirectiveKind::Custom(id) => self.custom.get(id).map_or(Transition::Hold, |f| f(input)),
        }
    }
}

/// The world as tier 0 sees it (the shell implements it, `AI10c`).
pub trait LogicalEnv {
    /// The route `actor` would walk from `from` to `to` (doors and worldspaces included), or
    /// `None` when there is none (the actor holds).
    fn route(&self, actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef>;
    /// The point `progress_m` metres along `route` (clamped to it).
    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location;
    /// Where a placement is (for `Use`).
    fn placement(&self, id: PlacementId) -> Option<Location>;
    /// Where `actor` is at `at` (its own logical state, or its body at tiers 1-2), for `Follow`.
    fn actor_position(&self, actor: ActorKey, at: GameInstant) -> Option<Location>;
    /// The first schedule window edge for `actor` strictly after `after` (AI11). None by default.
    fn next_window_edge(&self, _actor: ActorKey, _after: GameInstant) -> Option<GameInstant> {
        None
    }
    /// The window edge at `at` is reached: push and retire the schedule's directives (AI11).
    fn on_window_edge(
        &self,
        _actor: ActorKey,
        _at: GameTime,
        _stack: &mut DirectiveStack,
        _policy: &dyn StackPolicy,
        _events: &mut Vec<DirectiveDone>,
    ) {
    }
}

/// What one tier-0 step needs besides the actor's own state and stack.
#[derive(Clone, Copy)]
pub struct LogicalCtx<'a> {
    pub env: &'a dyn LogicalEnv,
    pub transitions: &'a LogicalTransitions,
    pub policy: &'a dyn StackPolicy,
    /// Game seconds per simulated second (`GameClock::time_scale`): routes advance at walk speed
    /// in simulated time, as a tier-2 mover does.
    pub time_scale: f64,
}

/// An actor's tier-0 state (ai.md: `LogicalState { route, progress_m, activity, until }`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicalState {
    /// The route being walked.
    pub route: Option<RouteRef>,
    /// Metres along `route` at `at`.
    pub progress_m: f32,
    /// What the actor does where it stands.
    pub activity: Option<ActivityRef>,
    /// The next event (arrival, limit, window edge); `None` when nothing is due.
    pub until: Option<GameInstant>,
    /// Where it stands at `at`.
    pub pos: Location,
    /// The last instant it was stepped to.
    pub at: GameInstant,
    /// The actor it follows.
    pub follow: Option<ActorKey>,
    /// On `route`: what arriving does.
    pub arrive: Option<Arrival>,
    /// On `route`: when it arrives, fixed when the route is taken up (from its start at walk
    /// speed), so stepping often or rarely gives the same instant.
    pub arrives: Option<GameInstant>,
    /// The directive this state serves (`None`: none served).
    pub directive: Option<DirectiveId>,
}

impl LogicalState {
    /// An actor standing at `pos` at `at`, serving nothing yet: the first [`step`](Self::step)
    /// takes up the stack's served directive.
    pub fn new(pos: Location, at: GameInstant) -> Self {
        Self {
            route: None,
            progress_m: 0.0,
            activity: None,
            until: None,
            pos,
            at,
            follow: None,
            arrive: None,
            arrives: None,
            directive: None,
        }
    }

    /// Where the actor is at `t` (at or after `at`; earlier reads as `at`): along its route, at
    /// its leader's position, or where it stands.
    pub fn position_at(&self, t: GameInstant, env: &dyn LogicalEnv, time_scale: f64) -> Location {
        if let Some(leader) = self.follow {
            return env.actor_position(leader, t).unwrap_or_else(|| self.pos.clone());
        }
        match &self.route {
            Some(route) => env.point_on(route, self.progress_after(route, t, time_scale)),
            None => self.pos.clone(),
        }
    }

    /// Metres along its route at `t` (`None`: not on a route, or following): where a promoted
    /// walker resumes (`promote`, `AI10b`).
    pub fn progress_at(&self, t: GameInstant, time_scale: f64) -> Option<f32> {
        if self.follow.is_some() {
            return None;
        }
        self.route.as_ref().map(|route| self.progress_after(route, t, time_scale))
    }

    /// When the actor reaches the end of its route (`None`: not on one, or not moving).
    pub fn arrival(&self) -> Option<GameInstant> {
        self.route.as_ref().and(self.arrives)
    }

    /// Metres along `route` at `t` (its length from the arrival on).
    #[expect(clippy::cast_possible_truncation, reason = "metres along one route")]
    fn progress_after(&self, route: &RouteRef, t: GameInstant, time_scale: f64) -> f32 {
        if self.arrival().is_some_and(|a| a <= t) {
            return route.length_m;
        }
        let sim_s = sim_seconds(t.0.saturating_sub(self.at.0), time_scale);
        let m = f64::from(self.progress_m) + f64::from(route.speed_mps) * sim_s;
        m.min(f64::from(route.length_m)) as f32
    }

    /// Steps the state and `stack` to `now` through every event on the way, in time order: an
    /// arrival ends its directive (`Completed`) at the arrival's minute, limits end theirs at their
    /// deadlines, a window edge calls the env's hook, and each change of the served directive
    /// takes up its kind's [`Transition`] at that instant. Returns the events stepped.
    ///
    /// Step to an instant before changing the stack there (a push, a cancel), then step again
    /// at the same instant: the new served directive is taken up from where the actor stands.
    /// A `now` before `at` does nothing.
    pub fn step(
        &mut self,
        stack: &mut DirectiveStack,
        now: GameInstant,
        ctx: &LogicalCtx<'_>,
        events: &mut Vec<DirectiveDone>,
    ) -> usize {
        if now < self.at {
            return 0;
        }
        let actor = stack.actor();
        self.refresh(stack, ctx);
        let mut stepped = 0;
        let mut same_instant = 0;
        while let Some(next) = self.until.filter(|u| *u <= now) {
            let next = next.max(self.at);
            same_instant = if next == self.at { same_instant + 1 } else { 1 };
            if same_instant > MAX_REACTIONS {
                // A content loop at one instant (a repeating `GoTo` already at its goal): hold
                // until the clock moves on.
                self.until = None;
                break;
            }
            stepped += 1;
            let arrived = self.arrival().is_some_and(|a| a <= next);
            let edge = ctx.env.next_window_edge(actor, self.at).is_some_and(|e| e <= next);
            self.move_to(next, ctx);
            let minute = GameTime::from_instant(next);
            if edge {
                ctx.env.on_window_edge(actor, minute, stack, ctx.policy, events);
            }
            stack.advance(minute, ctx.policy, events);
            if arrived {
                self.arrive_now(stack, minute, ctx, events);
            }
            self.refresh(stack, ctx);
        }
        self.move_to(now, ctx);
        stack.advance(GameTime::from_instant(now), ctx.policy, events);
        self.refresh(stack, ctx);
        stepped
    }

    /// The route's end is reached at `self.at`.
    fn arrive_now(
        &mut self,
        stack: &mut DirectiveStack,
        minute: GameTime,
        ctx: &LogicalCtx<'_>,
        events: &mut Vec<DirectiveDone>,
    ) {
        let arrive = self.arrive.take();
        self.arrives = None;
        self.route = None;
        self.progress_m = 0.0;
        match arrive {
            Some(Arrival::Finish) => {
                if let Some(id) = self.directive {
                    stack.finish(id, minute, ctx.policy, events);
                }
            }
            Some(Arrival::Activity(a)) => self.activity = Some(a),
            None => {}
        }
    }

    /// Moves position and progress to `t` (no events).
    fn move_to(&mut self, t: GameInstant, ctx: &LogicalCtx<'_>) {
        if t <= self.at {
            return;
        }
        if let Some(leader) = self.follow {
            if let Some(p) = ctx.env.actor_position(leader, t) {
                self.pos = p;
            }
        } else if let Some(route) = &self.route {
            let m = self.progress_after(route, t, ctx.time_scale);
            self.pos = ctx.env.point_on(route, m);
            self.progress_m = m;
        }
        self.at = t;
    }

    /// Takes up the served directive if it changed, then sets `until`.
    fn refresh(&mut self, stack: &DirectiveStack, ctx: &LogicalCtx<'_>) {
        let served = stack.serving();
        if served.map(|d| d.id) != self.directive {
            self.directive = served.map(|d| d.id);
            self.route = None;
            self.progress_m = 0.0;
            self.activity = None;
            self.follow = None;
            self.arrive = None;
            self.arrives = None;
            if let Some(d) = served {
                let input = TransitionInput { actor: stack.actor(), kind: &d.spec.kind, pos: &self.pos, env: ctx.env };
                match ctx.transitions.transition(&input) {
                    Transition::Travel { to, arrive } => {
                        if let Some(route) = ctx.env.route(stack.actor(), &self.pos, &to) {
                            self.arrives = arrival_from(self.at, &route, ctx.time_scale);
                            self.route = Some(route);
                            self.arrive = Some(arrive);
                        }
                    }
                    Transition::Activity(a) => self.activity = Some(a),
                    Transition::Follow(leader) => {
                        self.follow = Some(leader);
                        if let Some(p) = ctx.env.actor_position(leader, self.at) {
                            self.pos = p;
                        }
                    }
                    Transition::Hold => {}
                }
            }
        }
        let deadline = stack.next_deadline().map(GameTime::instant);
        let edge = ctx.env.next_window_edge(stack.actor(), self.at);
        self.until = [self.arrival(), deadline, edge].into_iter().flatten().min();
    }
}

/// The adapter's fast-forward rules (ai.md: "the adapter keeps `OpenMW`'s `fastForward` rules";
/// Morrowind's port is MA7). Runs after tier 0 stepped the skipped span.
pub trait FastForwardRules: Send + Sync {
    /// The span `from..to` was skipped (rest, wait, long travel) and stepped: adjust the result.
    fn after_fast_forward(
        &self,
        _state: &mut LogicalState,
        _stack: &mut DirectiveStack,
        _span: (GameInstant, GameInstant),
        _ctx: &LogicalCtx<'_>,
        _events: &mut Vec<DirectiveDone>,
    ) {
    }
}

/// Modern rules: a skipped span is exactly the tier-0 stepping of it.
#[derive(Clone, Copy, Debug, Default)]
pub struct ModernFastForward;

impl FastForwardRules for ModernFastForward {}

/// Advances an actor's tier-0 state `hours` game hours (rest, wait, long travel): every event in
/// the span, in order ([`LogicalState::step`]), then the adapter's [`FastForwardRules`]. Returns
/// the instant reached.
pub fn fast_forward(
    state: &mut LogicalState,
    stack: &mut DirectiveStack,
    hours: f64,
    ctx: &LogicalCtx<'_>,
    rules: &dyn FastForwardRules,
    events: &mut Vec<DirectiveDone>,
) -> GameInstant {
    let from = state.at;
    let to = from.plus(hours_ms(hours));
    state.step(stack, to, ctx, events);
    rules.after_fast_forward(state, stack, (from, to), ctx, events);
    to
}

/// When a walker leaving the start of `route` at `at` reaches its end (`None`: it never moves).
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "finite, >= 0")]
fn arrival_from(at: GameInstant, route: &RouteRef, time_scale: f64) -> Option<GameInstant> {
    let length = f64::from(route.length_m);
    if length <= 0.0 {
        return Some(at);
    }
    let speed = f64::from(route.speed_mps);
    if !(speed > 0.0 && time_scale > 0.0 && time_scale.is_finite()) {
        return None;
    }
    let ms = (length / speed * time_scale * ms_per_second()).ceil();
    ms.is_finite().then(|| at.plus(ms as u64))
}

/// Game milliseconds as simulated seconds at `time_scale` (0 when the scale is not positive).
#[expect(clippy::cast_precision_loss, reason = "exact below 2^53 ms")]
fn sim_seconds(game_ms: u64, time_scale: f64) -> f64 {
    if time_scale > 0.0 && time_scale.is_finite() { game_ms as f64 / ms_per_second() / time_scale } else { 0.0 }
}

/// The clock's milliseconds per game second, as `f64` (exact).
#[expect(clippy::cast_precision_loss, reason = "1000 is exact")]
fn ms_per_second() -> f64 {
    MS_PER_SECOND as f64
}
