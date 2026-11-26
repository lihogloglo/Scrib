//! `AI10b` (promotion core): the resolve queue that turns a tier-0 actor the allocator raised
//! into a body placed in the world, unseen, and [`demote`], its way back (`docs/specs/ai.md`
//! "Tiers and budget": "Promotion resolve" and "Demotion").
//!
//! **Canonical technique:** a fixed spawn budget per tick (UE5 `MassRepresentation` + `MassLOD`:
//! representation actors spawn under a per-frame budget, and visual LOD changes off screen where
//! possible) with placement where the virtual state says the actor is, out of every player's view:
//! RDR2/GTA ambient peds are created outside the view frustum on their own path, AC Origins'
//! "Meta AI" (GDC 2018) realizes virtual NPCs near the player out of sight, and KCD2 runs the same
//! scripts at every LOD. Ours: a count per tick, not a time budget, so traces reproduce.
//! **Reuse:** Bevy has nothing (pure logic, no ECS); no crate does virtual-to-real NPC promotion
//! (big-brain, `bevior_tree`, dogoap are decision libraries), so this is our own.
//!
//! - **Queue** ([`PromoteQueue`]): every actor the allocator put in tier 1 or 2 that is still
//!   tier 0 in fact, ordered pins first, then nearer to the nearest player, then [`ActorKey`].
//!   [`PromoteQueue::run`] tries at most [`PromoteConfig::per_tick`] entries a tick; one that must
//!   wait is not tried again for [`PromoteConfig::retry_s`] (skips are free), so seen waiters
//!   near the player never starve the unseen ones behind them. A queued actor stays tier 0: the
//!   shell keeps stepping its [`LogicalState`].
//! - **Resolve** ([`resolve_promotion`]): the state's own spot at the resolve instant, then points
//!   back along its route (it walks into view on its own path), then the shell's occluded
//!   approaches (a door, a corner); the first one that snaps to the navmesh and no player sees
//!   wins, else the actor waits. Pins only reorder the queue: nobody is placed in view, so no
//!   combat starts in view.
//! - **Demotion** ([`demote`]): the allocator alone decides it (out of view, past the outer band,
//!   never pinned); the body's spot becomes a fresh [`LogicalState`].
//! - **Seams:** [`PlayerViews`] (gameplay viewers, also for doors and follower catch-up) and
//!   [`PromoteEnv`] (navmesh snap, smart-object slots, approaches, body size), implemented by the
//!   shell (`AI10c`).

use sf_ids::{ActorKey, GameInstant};
use sf_world::{Location, PlacementId};

use crate::alloc::{Pin, Tier};
use crate::logical::{ActivityRef, LogicalEnv, LogicalState};

/// Whether a player could see a spot: the gameplay viewers (`MassLOD` viewer info: each player's
/// view position and frustum), never render visibility. Implemented by the shell.
pub trait PlayerViews {
    /// A sphere of `radius_m` metres at `at` that any player's gameplay view contains. A player in
    /// another worldspace never sees it.
    fn seen_by_any(&self, at: &Location, radius_m: f32) -> bool;

    /// No player could see the sphere.
    fn unseen(&self, at: &Location, radius_m: f32) -> bool {
        !self.seen_by_any(at, radius_m)
    }
}

/// The world promotion places bodies in (the shell, `AI10c`).
pub trait PromoteEnv: PlayerViews {
    /// The nearest navmesh point to `at`; `None` when there is no ground there yet (the actor
    /// waits).
    fn snap(&self, at: &Location) -> Option<Location>;

    /// Claims smart-object slot `slot` for `actor` (a `Use` activity); `false` = taken.
    fn claim(&self, _actor: ActorKey, _slot: PlacementId) -> bool {
        true
    }

    /// Occluded entries near the seen spot `at` (a door, a corner), best first, appended to `out`
    /// (`AI12` fills it). None by default.
    fn approaches(&self, _actor: ActorKey, _at: &Location, _out: &mut Vec<Location>) {}

    /// The actor's body radius, metres (the sphere the view test uses).
    fn body_radius_m(&self, actor: ActorKey) -> f32;
}

/// Promotion tunables (`ai.tier.*`, the `Def`s live in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PromoteConfig {
    /// Entries tried per tick (count, range 1-64): a try is one snap plus its view tests.
    pub per_tick: u32,
    /// Seconds before a waiting entry is tried again (range 0.1-5).
    pub retry_s: f32,
    /// How far back along its route an actor may be placed to walk in from out of view, metres
    /// (range 0-200).
    pub backtrack_m: f32,
    /// Spacing of the points tried back along the route, metres (range 1-20).
    pub backtrack_step_m: f32,
}

impl Default for PromoteConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.tier.promote_per_tick in sf_ai_bevy; Guess: ai.md "Guess 4", MassLOD spawn budget)
            per_tick: 4,
            // tunables: const (bound from ai.tier.promote_retry_s in sf_ai_bevy; Guess: a view changes in about half a second)
            retry_s: 0.5,
            // tunables: const (bound from ai.tier.promote_backtrack_m in sf_ai_bevy; Guess: a short walk, GTA/RDR on-path spawn)
            backtrack_m: 40.0,
            // tunables: const (bound from ai.tier.promote_backtrack_step_m in sf_ai_bevy; Guess: a few strides)
            backtrack_step_m: 4.0,
        }
    }
}

impl PromoteConfig {
    /// [`retry_s`](Self::retry_s) in whole ticks of `tick_s`, rounded up, at least 1 (1 for a
    /// non-finite or non-positive input).
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 1..=u32::MAX first")]
    pub fn retry_ticks(&self, tick_s: f32) -> u64 {
        if !(self.retry_s.is_finite() && tick_s.is_finite() && tick_s > 0.0) {
            return 1;
        }
        // The ratio of two decimal seconds lands a hair over a whole number in f32 (0.5 / 0.1):
        // round to millionths of a tick before rounding up.
        let n = (((f64::from(self.retry_s) / f64::from(tick_s)) * 1e6).round() / 1e6).ceil();
        if n <= 1.0 {
            1
        } else if n >= f64::from(u32::MAX) {
            u64::from(u32::MAX)
        } else {
            n as u64
        }
    }
}

/// One actor the allocator put in tier 1 or 2 that is still tier 0 in fact.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PromoteRequest {
    pub actor: ActorKey,
    /// The tier it was granted.
    pub to: Tier,
    /// Distance to the nearest player of all, metres (non-finite = farthest; negative = 0).
    pub nearest_player_m: f32,
    pub pin: Option<Pin>,
}

/// How a placed actor enters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnteredFrom {
    /// Where its state puts it.
    Spot,
    /// Back along its own route, walking into view.
    Backtrack,
    /// One of the shell's occluded approaches (a door, a corner).
    Approach,
}

/// Where and how a promoted actor's body appears.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    /// The snapped spot.
    pub at: Location,
    /// Metres along its route where the walker resumes (`None`: not on its route).
    pub progress_m: Option<f32>,
    /// What it does there (`None`: the served directive is taken up from `at`).
    pub activity: Option<ActivityRef>,
    pub entered_from: EnteredFrom,
}

/// The outcome of one resolve try.
#[derive(Clone, Debug, PartialEq)]
pub enum Resolve {
    /// Place the body: the entry leaves the queue.
    Place(Placement),
    /// Every candidate was seen or had no ground: try again after `retry_s`.
    Wait,
}

/// Resolves where `actor` (tier-0 `state`) appears at `t`: candidates in order, the first that
/// snaps to the navmesh and no player sees (a sphere of its body radius) wins:
///
/// 1. its spot, [`LogicalState::position_at`] (a follower stands where its leader is); a `Use`
///    activity claims the slot, a refused claim places it standing there ([`ActivityRef::Wait`]);
/// 2. on a route, points every [`backtrack_step_m`](PromoteConfig::backtrack_step_m) back along
///    it, up to [`backtrack_m`](PromoteConfig::backtrack_m) or the route start;
/// 3. the shell's approaches, in its order.
///
/// None: [`Resolve::Wait`]. A pin never lets a seen candidate through.
pub fn resolve_promotion(
    actor: ActorKey,
    state: &LogicalState,
    t: GameInstant,
    time_scale: f64,
    env: &dyn LogicalEnv,
    world: &dyn PromoteEnv,
    config: &PromoteConfig,
) -> Resolve {
    let radius = world.body_radius_m(actor);
    let placeable = |at: &Location| world.snap(at).filter(|s| world.unseen(s, radius));

    let spot = state.position_at(t, env, time_scale);
    let progress = state.progress_at(t, time_scale);
    if let Some(at) = placeable(&spot) {
        let activity = match state.activity {
            Some(ActivityRef::Use(id)) if !world.claim(actor, id) => Some(ActivityRef::Wait),
            other => other,
        };
        return Resolve::Place(Placement { at, progress_m: progress, activity, entered_from: EnteredFrom::Spot });
    }

    if let (Some(route), Some(from)) = (&state.route, progress) {
        let (step, max) = (config.backtrack_step_m, config.backtrack_m);
        if step.is_finite() && step > 0.0 && max.is_finite() {
            let mut back = step;
            let mut last = from;
            while back <= max && last > 0.0 {
                let m = (from - back).max(0.0);
                last = m;
                if let Some(at) = placeable(&env.point_on(route, m)) {
                    return Resolve::Place(Placement {
                        at,
                        progress_m: Some(m),
                        activity: None,
                        entered_from: EnteredFrom::Backtrack,
                    });
                }
                back += step;
            }
        }
    }

    let mut approaches = Vec::new();
    world.approaches(actor, &spot, &mut approaches);
    for a in &approaches {
        if let Some(at) = placeable(a) {
            return Resolve::Place(Placement { at, progress_m: None, activity: None, entered_from: EnteredFrom::Approach });
        }
    }
    Resolve::Wait
}

/// A granted demotion: the body at `body` becomes tier 0 at `now`, serving nothing yet, so the
/// next [`LogicalState::step`] takes up the served directive from where the body stood.
///
/// The allocator alone grants it ([`Allocator::allocate`](crate::Allocator::allocate) gives `to:
/// Tier::Logical`): only out of every player's view (a visible incumbent is held in tier 1), past
/// the outer band (`reduced_margin_m` hysteresis), never pinned (combat, dialogue and scripted
/// scenes stay tier 2).
pub fn demote(body: Location, now: GameInstant) -> LogicalState {
    LogicalState::new(body, now)
}

/// One tick of the queue, for `ai.stats` (the µs are the shell's).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PromoteTick {
    /// The actors placed this tick, in try order, with their granted tier.
    pub placed: Vec<(ActorKey, Tier, Placement)>,
    /// Resolve tries this tick (at most `per_tick`).
    pub tried: u32,
    /// Entries left whose next try is after this tick (they waited).
    pub waiting: usize,
    /// Entries left in the queue.
    pub queued: usize,
}

/// One queued actor.
#[derive(Clone, Copy, Debug)]
struct Queued {
    req: PromoteRequest,
    /// The cleaned distance the order uses.
    dist: f32,
    /// The first tick it may be tried on.
    next_try: u64,
}

impl Queued {
    /// Pins first, then nearer, then key.
    fn order(&self, other: &Self) -> std::cmp::Ordering {
        self.req
            .pin
            .is_none()
            .cmp(&other.req.pin.is_none())
            .then_with(|| self.dist.total_cmp(&other.dist))
            .then_with(|| self.req.actor.cmp(&other.req.actor))
    }
}

/// A finite, non-negative distance (non-finite ranks farthest), as the allocator cleans it.
fn clean_distance(d: f32) -> f32 {
    if d.is_nan() { f32::INFINITY } else { d.max(0.0) }
}

/// The promotion resolve queue. Plain data, not saved: derived each tick from the allocation,
/// like [`Allocator`](crate::Allocator). Buffers are reused, so a warm queue does not allocate
/// (but for the actors [`run`](Self::run) reports placed).
#[derive(Clone, Debug, Default)]
pub struct PromoteQueue {
    /// The queue, in try order.
    entries: Vec<Queued>,
    /// `sync`'s incoming requests, sorted by key, deduplicated.
    incoming: Vec<Queued>,
}

impl PromoteQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Entries queued.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Whether `actor` is queued.
    pub fn contains(&self, actor: ActorKey) -> bool {
        self.entries.iter().any(|q| q.req.actor == actor)
    }

    /// The queued actors in try order.
    pub fn actors(&self) -> impl Iterator<Item = ActorKey> + '_ {
        self.entries.iter().map(|q| q.req.actor)
    }

    /// Sets the queue to `wanted` (any order; a duplicated actor counts once, its most
    /// significant entry): new actors enter (tried from the next `run`), known ones keep their
    /// retry tick and take the new distance, pin and tier, and the ones no longer wanted leave
    /// (the allocator sent them back to tier 0).
    pub fn sync(&mut self, wanted: &[PromoteRequest]) {
        let Self { entries, incoming } = self;
        incoming.clear();
        incoming.extend(wanted.iter().map(|&req| Queued { req, dist: clean_distance(req.nearest_player_m), next_try: 0 }));
        incoming.sort_unstable_by(|a, b| a.req.actor.cmp(&b.req.actor).then_with(|| a.order(b)));
        incoming.dedup_by_key(|q| q.req.actor);

        // Carry the retry ticks over (both lists in key order).
        entries.sort_unstable_by_key(|q| q.req.actor);
        let mut j = 0;
        for q in incoming.iter_mut() {
            while j < entries.len() && entries[j].req.actor < q.req.actor {
                j += 1;
            }
            if j < entries.len() && entries[j].req.actor == q.req.actor {
                q.next_try = entries[j].next_try;
                j += 1;
            }
        }
        std::mem::swap(entries, incoming);
        entries.sort_unstable_by(Queued::order);
    }

    /// Runs one tick: tries at most `config.per_tick` entries in order with `resolve`, skipping
    /// (free) those whose retry is pending. A placed entry leaves the queue and is reported; a
    /// waiting one stays, tried again at `tick + config.retry_ticks(tick_s)`.
    pub fn run(
        &mut self,
        tick: u64,
        tick_s: f32,
        config: &PromoteConfig,
        mut resolve: impl FnMut(&PromoteRequest) -> Resolve,
    ) -> PromoteTick {
        let retry = config.retry_ticks(tick_s);
        let mut out = PromoteTick::default();
        for q in &mut self.entries {
            if out.tried >= config.per_tick {
                break;
            }
            if q.next_try > tick {
                continue;
            }
            out.tried += 1;
            match resolve(&q.req) {
                Resolve::Place(p) => {
                    out.placed.push((q.req.actor, q.req.to, p));
                }
                Resolve::Wait => q.next_try = tick.saturating_add(retry),
            }
        }
        if !out.placed.is_empty() {
            let placed = &out.placed;
            self.entries.retain(|q| !placed.iter().any(|(a, _, _)| *a == q.req.actor));
        }
        out.queued = self.entries.len();
        out.waiting = self.entries.iter().filter(|q| q.next_try > tick).count();
        out
    }
}
