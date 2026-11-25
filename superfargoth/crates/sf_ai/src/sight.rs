//! `AI8b1`: the sight maths core (`ai.md` "Awareness" → "Sight gain", "Perception profile" → "Hook",
//! "Cost and budget"). Pure, Bevy-free, host-only, allocation-free per tick: every buffer is owned by
//! the caller ([`RayQueue`], [`SightCache`], the result `Vec`) and sized once.
//!
//! - [`sight_gain`]: `rate × dt × f_dist × f_cone × visibility × alertness` for one observer and
//!   target, `None` out of range or cone or below `skip_gain` (no rays are queued then).
//! - [`apply_darkness`]: the actor's `darkness` on a light level (`AI8d` calls it before
//!   `DetectionRules::concealment`).
//! - [`stagger_phase`] / [`due`] / [`SightTimer`]: checks spread over `sight_period_s` by a stable
//!   hash of the [`ActorKey`], a force-next-tick flag (damage, noise, notify), and the elapsed time
//!   each check integrates.
//! - [`RayQueue`]: the one shared ray queue (sight, light, concern rays), at most
//!   `max_rays_per_tick` casts, oldest first, overflow carried to the next tick; a sight request casts
//!   the chest ray, then the head ray only if the chest is blocked (≤ 2). Results come out in
//!   [`ActorKey`] order. The cast itself is the caller's [`LineOfSight`] (physics in `sf_ai_bevy`).
//! - [`SightCache`]: the last line-of-sight result per (observer, target), valid `sight_cache_s`
//!   unless either side moved more than `cache_move_m`.

use std::collections::{HashMap, VecDeque};

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_ids::ActorKey;

use crate::perception::{PerceptClock, PerceptTarget};
use crate::profile::{SightConfig, SightParams};

// tunables: const (maths: below this distance the target sits at the eye, any direction counts as central)
const AT_EYE_M: f32 = 1e-4;

/// The sight budget tunables (`ai.perception.*`, bound in `sf_ai_bevy`; ai.md "Tunables" table).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SightBudget {
    /// A check whose gain rate (`gain / dt`) is below this queues no rays and gains nothing, 1/s
    /// (range 0-1).
    pub skip_gain: f32,
    /// Tier 2 sight check period, staggered per actor, seconds (range 0.02-10).
    pub sight_period_s: f32,
    /// Tier 1 sight check period (no rays), seconds (range 0.02-10).
    pub tier1_period_s: f32,
    /// Rays cast per tick across sight, light and concern rays (UE `MaxTracesPerTick`), count
    /// (range 4-512; per `sim.cpu_tier` Floor / Medium / High / Ultra / Dedicated 24 / 32 / 48 / 64 / 96).
    pub max_rays_per_tick: u32,
    /// How long a line-of-sight result is reused, seconds (range 0.02-10).
    pub sight_cache_s: f32,
    /// A cached result is dropped once either side moved more than this, meters (range 0-5).
    pub cache_move_m: f32,
}

impl Default for SightBudget {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.perception.skip_gain in sf_ai_bevy; Guess)
            skip_gain: 0.01,
            // tunables: const (bound from ai.perception.sight_period_s in sf_ai_bevy; Guess)
            sight_period_s: 0.1,
            // tunables: const (bound from ai.perception.tier1_period_s in sf_ai_bevy; Guess)
            tier1_period_s: 1.0,
            // tunables: const (bound from ai.perception.max_rays_per_tick in sf_ai_bevy, Floor tier; Guess)
            max_rays_per_tick: 24,
            // tunables: const (bound from ai.perception.sight_cache_s in sf_ai_bevy; Guess)
            sight_cache_s: 0.25,
            // tunables: const (bound from ai.perception.cache_move_m in sf_ai_bevy; Guess)
            cache_move_m: 0.5,
        }
    }
}

impl SightBudget {
    /// Tier 2 check period in ticks (≥ 1).
    pub fn sight_period_ticks(&self, tick_s: f32) -> u32 {
        seconds_to_ticks(self.sight_period_s, tick_s)
    }

    /// Tier 1 check period in ticks (≥ 1).
    pub fn tier1_period_ticks(&self, tick_s: f32) -> u32 {
        seconds_to_ticks(self.tier1_period_s, tick_s)
    }

    /// Cache lifetime in ticks (≥ 1): an entry is valid while its age is below this.
    pub fn cache_ticks(&self, tick_s: f32) -> u32 {
        seconds_to_ticks(self.sight_cache_s, tick_s)
    }
}

/// `seconds / tick_s` rounded to whole ticks, at least 1 (and 1 for a non-finite or non-positive input).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // clamped to 1..=u32::MAX first
pub fn seconds_to_ticks(seconds: f32, tick_s: f32) -> u32 {
    if !(seconds.is_finite() && tick_s.is_finite() && tick_s > 0.0) {
        return 1;
    }
    let t = (seconds / tick_s).round();
    if t <= 1.0 {
        1
    } else if t >= u32::MAX as f32 {
        u32::MAX
    } else {
        t as u32
    }
}

// --- Gain -------------------------------------------------------------------------------------

/// One sight check's inputs (ai.md "Sight gain").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SightCheck {
    /// The observer's eye.
    pub observer: WorldPos,
    /// The observer's facing (any non-zero length).
    pub facing: Vec3,
    /// The point checked on the target (its chest).
    pub target: WorldPos,
    /// The observer already sees this target: its range is `lose_m` (UE `LoseSightRadius`).
    pub seen_before: bool,
    /// `clamp(1 - concealment / 100, 0, 1)` (crime's `DetectionRules::concealment`), 0..1.
    pub visibility: f32,
    /// `AwarenessMeter::alertness` (the band and groggy factor, on top of the profile's).
    pub alertness: f32,
}

/// `f_dist`: 1 up to `near_m`, linear to 0 at `range_m` (`None` at or beyond it).
fn f_dist(d: f32, near_m: f32, range_m: f32) -> Option<f32> {
    if d >= range_m {
        None
    } else if d <= near_m {
        Some(1.0)
    } else {
        Some((range_m - d) / (range_m - near_m))
    }
}

/// `f_cone`: 1 within the central half-angle, `peripheral_mul` within the peripheral one, `None`
/// behind. `to` is observer → target, `d` its length.
fn f_cone(params: &SightParams, peripheral_mul: f32, facing: Vec3, to: Vec3, d: f32) -> Option<f32> {
    if d <= AT_EYE_M {
        return Some(1.0);
    }
    let fwd = facing.try_normalize()?;
    let cos = fwd.dot(to) / d;
    if cos >= params.central_half_deg.to_radians().cos() {
        Some(1.0)
    } else if cos >= params.peripheral_half_deg.to_radians().cos() {
        Some(peripheral_mul)
    } else {
        None
    }
}

/// The check's gain rate, 1/s: `params.rate × f_dist × f_cone × visibility × alertness`, `None`
/// out of range, out of the cone, or below `skip_gain` (then no rays are queued).
pub fn sight_rate(params: &SightParams, cfg: &SightConfig, budget: &SightBudget, check: &SightCheck) -> Option<f32> {
    if !(check.visibility.is_finite() && check.alertness.is_finite() && check.facing.is_finite()) {
        return None;
    }
    let to = check.observer.delta_f32(check.target);
    let d = to.length();
    if !d.is_finite() {
        return None;
    }
    let range = if check.seen_before { params.lose_m } else { params.range_m };
    let dist = f_dist(d, cfg.near_m, range)?;
    let cone = f_cone(params, cfg.peripheral_mul, check.facing, to, d)?;
    let rate = params.rate * dist * cone * check.visibility.clamp(0.0, 1.0) * check.alertness.max(0.0);
    (rate >= budget.skip_gain && rate > 0.0).then_some(rate)
}

/// One check's gain over `dt` seconds (ai.md "Sight gain"): [`sight_rate`] × `dt`. `params` is the
/// actor's resolved [`SightParams`] (`EffectivePerception::sight`, or the globals'), `cfg` gives the
/// unscaled `near_m` and `peripheral_mul`. `None` as [`sight_rate`], or for a negative or non-finite `dt`.
pub fn sight_gain(params: &SightParams, cfg: &SightConfig, budget: &SightBudget, check: &SightCheck, dt: f32) -> Option<f32> {
    if !(dt.is_finite() && dt >= 0.0) {
        return None;
    }
    sight_rate(params, cfg, budget, check).map(|r| r * dt)
}

/// The actor's `darkness` on a light level (ai.md "Perception profile"): `1 - (1 - light) ×
/// darkness`, clamped to 0..1 (0 sees in the dark, 1 normal, 2 night-blind).
pub fn apply_darkness(light: f32, darkness: f32) -> f32 {
    (1.0 - (1.0 - light.clamp(0.0, 1.0)) * darkness.max(0.0)).clamp(0.0, 1.0)
}

// --- Stagger ----------------------------------------------------------------------------------

/// splitmix64's finalizer (maths: a fixed, well-spread 64-bit mix; never the std hasher, whose
/// algorithm may change between releases).
fn mix64(mut z: u64) -> u64 {
    // tunables: const (maths: splitmix64 constants)
    const M1: u64 = 0xbf58_476d_1ce4_e5b9;
    // tunables: const (maths: splitmix64 constants)
    const M2: u64 = 0x94d0_49bb_1331_11eb;
    // tunables: const (maths: splitmix64 golden-ratio increment)
    const GOLDEN: u64 = 0x9e37_79b9_7f4a_7c15;
    z = z.wrapping_add(GOLDEN);
    z = (z ^ (z >> 30)).wrapping_mul(M1);
    z = (z ^ (z >> 27)).wrapping_mul(M2);
    z ^ (z >> 31)
}

/// A stable hash of an [`ActorKey`] (same key, same hash, on every machine and run).
#[allow(clippy::cast_possible_truncation)] // splitting a u128 into its halves
pub fn actor_hash(actor: ActorKey) -> u64 {
    let lo = actor.0 as u64;
    let hi = (actor.0 >> 64) as u64;
    mix64(lo ^ mix64(hi))
}

/// The tick within each `period_ticks` window on which `actor` checks (0..period).
#[allow(clippy::cast_possible_truncation)] // the remainder is below a u32 period
pub fn stagger_phase(actor: ActorKey, period_ticks: u32) -> u32 {
    (actor_hash(actor) % u64::from(period_ticks.max(1))) as u32
}

/// Whether `actor`'s staggered check falls on `tick` (exactly once per `period_ticks`).
pub fn due(actor: ActorKey, tick: u64, period_ticks: u32) -> bool {
    tick % u64::from(period_ticks.max(1)) == u64::from(stagger_phase(actor, period_ticks))
}

/// Per-actor check state: the last check's tick and the force-next-tick flag (damage, noise or
/// notify force a check).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SightTimer {
    last: Option<u64>,
    forced: bool,
}

impl SightTimer {
    /// A timer whose time is counted from `tick` (a fresh actor's first check integrates from here).
    pub fn starting_at(tick: u64) -> Self {
        Self { last: Some(tick), forced: false }
    }

    /// Forces a check on the next [`Self::poll`].
    pub fn force(&mut self) {
        self.forced = true;
    }

    /// Whether a check is forced.
    pub fn is_forced(&self) -> bool {
        self.forced
    }

    /// If a check runs on `tick` (forced, or its staggered phase), records it and returns the
    /// elapsed ticks it integrates (`dt = elapsed × tick_s`): since the last check, at most one
    /// period (a timer never polled counts one period). `None` if not due or already checked on `tick`.
    #[allow(clippy::cast_possible_truncation)] // capped at a u32 period first
    pub fn poll(&mut self, actor: ActorKey, tick: u64, period_ticks: u32) -> Option<u32> {
        let period = period_ticks.max(1);
        if !(self.forced || due(actor, tick, period)) {
            return None;
        }
        let elapsed = self.last.map_or(u64::from(period), |l| tick.saturating_sub(l)).min(u64::from(period));
        if elapsed == 0 {
            return None;
        }
        self.last = Some(tick);
        self.forced = false;
        Some(elapsed as u32)
    }
}

// --- Rays -------------------------------------------------------------------------------------

/// What a queued ray is for (one queue for all, ai.md "Cost and budget").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RayKind {
    /// Observer → target (chest, then head).
    Sight,
    /// Light exposure (`AI8d`).
    Light,
    /// Concern discovery (world reactions step 4).
    Concern,
}

/// What a ray is about: a perception target, or a caller id (a light, a concern).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RaySubject {
    Target(PerceptTarget),
    Id(u64),
}

/// A queued request: one ray to `to`, and if that is blocked one more to `fallback` (≤ 2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayRequest {
    pub owner: ActorKey,
    pub kind: RayKind,
    pub subject: RaySubject,
    pub from: WorldPos,
    pub to: WorldPos,
    pub fallback: Option<WorldPos>,
}

impl RayRequest {
    /// A sight request: the chest ray, then the head ray only if the chest is blocked.
    pub fn sight(observer: ActorKey, target: PerceptTarget, eye: WorldPos, chest: WorldPos, head: WorldPos) -> Self {
        Self { owner: observer, kind: RayKind::Sight, subject: RaySubject::Target(target), from: eye, to: chest, fallback: Some(head) }
    }

    fn same_job(&self, other: &Self) -> bool {
        self.owner == other.owner && self.kind == other.kind && self.subject == other.subject
    }
}

/// One segment handed to [`LineOfSight`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LosRay {
    pub owner: ActorKey,
    pub kind: RayKind,
    pub subject: RaySubject,
    pub from: WorldPos,
    pub to: WorldPos,
}

/// The caller's ray cast (physics in `sf_ai_bevy`; a fake world in tests). One call per batch, so
/// the caller may spread it over workers.
pub trait LineOfSight {
    /// Sets `clear[i]` to whether `rays[i]` reaches its end unobstructed (`clear.len() == rays.len()`).
    fn cast(&mut self, rays: &[LosRay], clear: &mut [bool]);
}

impl<F: FnMut(&LosRay) -> bool> LineOfSight for F {
    fn cast(&mut self, rays: &[LosRay], clear: &mut [bool]) {
        for (r, c) in rays.iter().zip(clear.iter_mut()) {
            *c = self(r);
        }
    }
}

/// A finished request's line of sight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LosOutcome {
    /// The first ray was clear (sight: the chest).
    Clear,
    /// The first was blocked, the fallback clear (sight: the head).
    ClearFallback,
    /// Every ray was blocked.
    Blocked,
}

impl LosOutcome {
    /// Whether the subject is in sight.
    pub fn visible(self) -> bool {
        !matches!(self, Self::Blocked)
    }
}

/// A finished request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayResult {
    pub owner: ActorKey,
    pub kind: RayKind,
    pub subject: RaySubject,
    /// The tick the request was first queued.
    pub enqueued: u64,
    pub outcome: LosOutcome,
}

/// One tick's ray counts (for `ai.stats`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RayTickStats {
    pub cast: u32,
    /// Requests left queued for the next tick.
    pub deferred: u32,
}

/// The queue is at its capacity and the request is not already queued.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("ray queue full ({0} requests)")]
pub struct RayQueueFull(pub usize);

#[derive(Clone, Copy, Debug)]
struct Pending {
    req: RayRequest,
    enqueued: u64,
    /// The first ray was blocked on an earlier tick: the fallback is next.
    first_blocked: bool,
    done: Option<LosOutcome>,
}

impl Pending {
    fn segment(&self) -> LosRay {
        let to = if self.first_blocked { self.req.fallback.unwrap_or(self.req.to) } else { self.req.to };
        LosRay { owner: self.req.owner, kind: self.req.kind, subject: self.req.subject, from: self.req.from, to }
    }
}

/// The shared ray queue (ai.md "Cost and budget"): fixed capacity, oldest first, at most
/// `max_rays` casts per [`Self::process`], the rest carried over.
#[derive(Clone, Debug)]
pub struct RayQueue {
    pending: VecDeque<Pending>,
    capacity: usize,
    batch: Vec<LosRay>,
    slot: Vec<usize>,
    clear: Vec<bool>,
}

impl RayQueue {
    /// A queue holding at most `capacity` requests, with scratch for `max_batch` rays per batch.
    pub fn new(capacity: usize, max_batch: usize) -> Self {
        Self {
            pending: VecDeque::with_capacity(capacity),
            capacity,
            batch: Vec::with_capacity(max_batch),
            slot: Vec::with_capacity(max_batch),
            clear: Vec::with_capacity(max_batch),
        }
    }

    /// Queued requests.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Queues `req` at `now`. A request already queued for the same (owner, kind, subject) keeps
    /// its place and age and takes the new end points (its first ray is tried again).
    pub fn push(&mut self, req: RayRequest, now: u64) -> Result<(), RayQueueFull> {
        if let Some(p) = self.pending.iter_mut().find(|p| p.req.same_job(&req)) {
            p.req = req;
            p.first_blocked = false;
            return Ok(());
        }
        if self.pending.len() >= self.capacity {
            return Err(RayQueueFull(self.capacity));
        }
        self.pending.push_back(Pending { req, enqueued: now, first_blocked: false, done: None });
        Ok(())
    }

    /// Drops every request `owner` made (a despawned or demoted actor).
    pub fn cancel(&mut self, owner: ActorKey) {
        self.pending.retain(|p| p.req.owner != owner);
    }

    /// Casts up to `max_rays` rays, oldest request first: each request's next ray, then the
    /// fallbacks of those just blocked while rays remain. Finished requests go to `out` (cleared
    /// first), sorted by (owner [`ActorKey`], kind, subject); the rest stay queued, in order.
    pub fn process(&mut self, max_rays: u32, los: &mut impl LineOfSight, out: &mut Vec<RayResult>) -> RayTickStats {
        out.clear();
        let mut budget = usize::try_from(max_rays).unwrap_or(usize::MAX);
        let mut cast = 0usize;

        // Pass 1: each of the oldest requests casts its next ray.
        let first = budget.min(self.pending.len());
        self.batch.clear();
        self.slot.clear();
        for (i, p) in self.pending.iter().take(first).enumerate() {
            self.batch.push(p.segment());
            self.slot.push(i);
        }
        self.cast_batch(los);
        cast += self.batch.len();
        budget -= self.batch.len();
        for (&i, &clear) in self.slot.iter().zip(&self.clear) {
            let p = &mut self.pending[i];
            p.done = match (clear, p.first_blocked, p.req.fallback.is_some()) {
                (true, false, _) => Some(LosOutcome::Clear),
                (true, true, _) => Some(LosOutcome::ClearFallback),
                (false, false, true) => {
                    p.first_blocked = true;
                    None
                }
                (false, _, _) => Some(LosOutcome::Blocked),
            };
        }

        // Pass 2: fallbacks of requests blocked in pass 1 (in queue order) while rays remain.
        self.batch.clear();
        self.slot.clear();
        for (i, p) in self.pending.iter().take(first).enumerate() {
            if budget == 0 {
                break;
            }
            if p.done.is_none() {
                self.batch.push(p.segment());
                self.slot.push(i);
                budget -= 1;
            }
        }
        self.cast_batch(los);
        cast += self.batch.len();
        for (&i, &clear) in self.slot.iter().zip(&self.clear) {
            let p = &mut self.pending[i];
            p.done = Some(if clear { LosOutcome::ClearFallback } else { LosOutcome::Blocked });
        }

        for p in &self.pending {
            if let Some(outcome) = p.done {
                out.push(RayResult { owner: p.req.owner, kind: p.req.kind, subject: p.req.subject, enqueued: p.enqueued, outcome });
            }
        }
        self.pending.retain(|p| p.done.is_none());
        out.sort_unstable_by_key(|r| (r.owner, r.kind, r.subject));
        RayTickStats { cast: u32::try_from(cast).unwrap_or(u32::MAX), deferred: u32::try_from(self.pending.len()).unwrap_or(u32::MAX) }
    }

    fn cast_batch(&mut self, los: &mut impl LineOfSight) {
        self.clear.clear();
        self.clear.resize(self.batch.len(), false);
        if !self.batch.is_empty() {
            los.cast(&self.batch, &mut self.clear);
        }
    }
}

// --- Cache ------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct CacheEntry {
    observer_at: WorldPos,
    target_at: WorldPos,
    tick: u64,
    outcome: LosOutcome,
}

/// The last line-of-sight result per (observer, target), fixed capacity (ai.md "Cost and budget").
#[derive(Clone, Debug)]
pub struct SightCache {
    entries: HashMap<(ActorKey, PerceptTarget), CacheEntry>,
    capacity: usize,
}

impl SightCache {
    /// A cache holding at most `capacity` pairs.
    pub fn new(capacity: usize) -> Self {
        Self { entries: HashMap::with_capacity(capacity), capacity }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn fresh(e: &CacheEntry, observer_at: WorldPos, target_at: WorldPos, clock: PerceptClock, budget: &SightBudget) -> bool {
        let age = clock.tick.saturating_sub(e.tick);
        let moved2 = budget.cache_move_m * budget.cache_move_m;
        age < u64::from(budget.cache_ticks(clock.tick_s))
            && e.observer_at.delta_f32(observer_at).length_squared() <= moved2
            && e.target_at.delta_f32(target_at).length_squared() <= moved2
    }

    /// The cached result for the pair, if younger than `sight_cache_s` and neither side moved more
    /// than `cache_move_m` since it was stored.
    pub fn get(
        &self,
        observer: ActorKey,
        target: PerceptTarget,
        observer_at: WorldPos,
        target_at: WorldPos,
        clock: PerceptClock,
        budget: &SightBudget,
    ) -> Option<LosOutcome> {
        let e = self.entries.get(&(observer, target))?;
        Self::fresh(e, observer_at, target_at, clock, budget).then_some(e.outcome)
    }

    /// Stores the pair's result at `tick` (positions as checked). `false` if the cache is full of
    /// other pairs (call [`Self::prune`]; nothing is evicted here).
    pub fn insert(
        &mut self,
        observer: ActorKey,
        target: PerceptTarget,
        observer_at: WorldPos,
        target_at: WorldPos,
        tick: u64,
        outcome: LosOutcome,
    ) -> bool {
        let key = (observer, target);
        if self.entries.len() >= self.capacity && !self.entries.contains_key(&key) {
            return false;
        }
        self.entries.insert(key, CacheEntry { observer_at, target_at, tick, outcome });
        true
    }

    /// Drops entries older than `sight_cache_s`.
    pub fn prune(&mut self, clock: PerceptClock, budget: &SightBudget) {
        let ttl = u64::from(budget.cache_ticks(clock.tick_s));
        self.entries.retain(|_, e| clock.tick.saturating_sub(e.tick) < ttl);
    }
}
