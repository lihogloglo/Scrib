//! The sight system (slice `AI8b2`; `docs/specs/ai.md` "Awareness" → "Sight gain", "Cost and
//! budget", "Perception profile" → "Hook"): [`AiSightPlugin`] runs the `sf_ai::sight` core on the
//! host's fixed tick. Host only: add it on the host (single player is the host); clients get
//! `PlayerDetection` replicated (ai.md "Net").
//!
//! One chain in `FixedUpdate`, before `ControllerSystems::Drive` (it reads the movers' positions of
//! the last step), set [`AiPerceptionSystems::Sight`]:
//!
//! 1. `begin_tick`: [`AiPerceptClock`] advances, fresh [`AiSightTimer`]s start counting, the
//!    globals' resolve is cached for actors without a profile.
//! 2. `resolve_profiles`: [`AiEffectivePerception`] rebuilt where the profile or states changed,
//!    a state expired (`needs_rebuild`), the state records or a tunable changed; dropped when the
//!    actor has neither.
//! 3. `sample_light` ([`crate::light`], `AI8d2`): each due target's [`AiLightExposure`] from the
//!    host's [`crate::AiLightInputs`] (absent: no target is lit, visibility stays the identity).
//! 4. `queue_sight`: each due observer (`SightTimer::poll`, staggered by `ActorKey` over
//!    `sight_period_s`) takes the targets of a uniform XZ grid within its reach (cell = the global
//!    lose-sight range), skips itself, computes `sight_gain` (visibility:
//!    [`light_only_visibility`]), then reads the [`AiSightCache`] or queues a sight request
//!    (chest, head if blocked) on the shared [`AiRays`] queue.
//! 5. `cast_rays`: [`RayQueue::process`] with what `max_rays_per_tick` leaves after the light
//!    rays, against the app's ground probe `G` (statics, props, terrain: the occluders; characters
//!    never block), through [`ProbeLos`].
//! 6. `apply_sight`: every hit as a `Stimulus::Sight` in (`ActorKey`, target) order, then
//!    `AwarenessMeter::update` on every observer.
//!
//! Not modelled yet: the rest of the target's visibility (crime's `DetectionRules::concealment`,
//! `LW4`: stance, speed, disguise; until then [`light_only_visibility`]), tier 1 checks (the `AI9`
//! allocator picks tiers; everyone is tier 2),
//! worldspace and interior spaces (every spawned mover is taken to share the active worldspace,
//! as the ground probe does), `sim.cpu_tier` groups for `max_rays_per_tick` (the Floor value).

use std::collections::HashMap;
use std::marker::PhantomData;

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_ai::{
    ActorKey, AwarenessMeter, EffectivePerception, LineOfSight, LosRay, PerceptClock, PerceptTarget,
    PerceptionProfile, PerceptionStates, RayKind, RayQueue, RayRequest, RayResult, RaySubject, RayTickStats,
    SightCache, SightCheck, SightTimer, Stimulus, apply_darkness, resolve, sight_gain,
};
use sf_controller::{GroundHit, GroundProbe, ProbeQuery, forward};
use sf_controller_bevy::{ControllerSystems, Dormant, Mover};
use sf_coords::WorldPos;

use crate::AiActor;
use crate::light::{AiLightExposure, LightScratch, sample_light};
use crate::perception::{
    AiAwareness, AiEffectivePerception, AiObserver, AiPerceptionProfile, AiPerceptionSettings,
    AiPerceptionStateDefs, AiPerceptionStates, AiSightTimer, Perceivable, PerceivablePose, add_perception_tunables,
};

/// The visibility of a target without a light sample (no [`crate::AiLightInputs`]): fully visible.
// tunables: const (maths: the identity of the visibility factor)
pub const VISIBILITY_UNMODELLED: f32 = 1.0;

/// The framework's default sight visibility, light only (`AI8d2`): the target's light exposure
/// through the observer's `darkness` ([`sf_ai::apply_darkness`]); [`VISIBILITY_UNMODELLED`]
/// without a sample. The seam crime's `DetectionRules::concealment` (`LW4`) replaces: it reads the
/// same light in its `VisibilityFactors` with stance, speed and disguise.
pub fn light_only_visibility(light: Option<f32>, darkness: f32) -> f32 {
    light.map_or(VISIBILITY_UNMODELLED, |l| apply_darkness(l, darkness))
}

// tunables: const (capacity: fixed-size request queue, ~10x the 48-actor tier-2 load; overflow refuses)
pub const RAY_QUEUE_CAPACITY: usize = 1024;
// tunables: const (capacity: fixed-size (observer, target) cache; a full cache prunes expired entries)
pub const SIGHT_CACHE_CAPACITY: usize = 4096;
// tunables: const (capacity: ray scratch per batch, the max_rays_per_tick range's top)
const RAY_BATCH_SCRATCH: usize = 512;
// tunables: const (maths: a segment shorter than this reaches its end)
const MIN_RAY_M: f32 = 1e-4;

/// Fixed-tick perception system sets.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiPerceptionSystems {
    /// The sight chain, before `ControllerSystems::Drive`.
    Sight,
    /// Hearing ([`crate::hearing`]), after [`Self::Sight`] (it reads this tick's clock and
    /// resolved profiles), before `ControllerSystems::Drive`.
    Hearing,
}

/// The perception clock: one tick per `FixedUpdate`, `tick_s` the fixed timestep.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct AiPerceptClock {
    pub tick: u64,
    pub tick_s: f32,
}

impl AiPerceptClock {
    /// As the `sf_ai` clock (pass it to `PerceptionState::until` when pushing a state).
    pub fn clock(&self) -> PerceptClock {
        PerceptClock::new(self.tick, self.tick_s)
    }
}

/// The one shared ray queue (sight now; light `AI8d` and concern rays later push here too) and
/// this tick's finished requests and counts (`ai.stats`). Non-sight results are left in `results`
/// for their owners to read after [`AiPerceptionSystems::Sight`].
#[derive(Resource, Debug)]
pub struct AiRays {
    pub queue: RayQueue,
    pub results: Vec<RayResult>,
    pub stats: RayTickStats,
    /// Sight requests refused this tick because the queue was full.
    pub refused: u32,
    /// Light rays cast this tick (`AI8d2`, outside the queue), charged to `max_rays_per_tick`
    /// before the queue's sight rays.
    pub light_cast: u32,
}

impl Default for AiRays {
    fn default() -> Self {
        Self {
            // tunables: const (capacities, the consts above)
            queue: RayQueue::new(RAY_QUEUE_CAPACITY, RAY_BATCH_SCRATCH),
            results: Vec::with_capacity(RAY_QUEUE_CAPACITY),
            stats: RayTickStats::default(),
            // tunables: const (a counter's start)
            refused: 0,
            // tunables: const (a counter's start)
            light_cast: 0,
        }
    }
}

/// The line-of-sight cache (`sight_cache_s`, `cache_move_m`).
#[derive(Resource, Debug)]
pub struct AiSightCache(pub SightCache);

impl Default for AiSightCache {
    fn default() -> Self {
        Self(SightCache::new(SIGHT_CACHE_CAPACITY))
    }
}

/// A target as this tick sees it.
#[derive(Clone, Copy, Debug)]
struct TargetSnap {
    cell: (i64, i64),
    target: PerceptTarget,
    entity: Entity,
    feet: WorldPos,
    chest: WorldPos,
    head: WorldPos,
    velocity: Vec3,
    /// Its light exposure sample, if any.
    light: Option<f32>,
}

/// A queued sight check waiting for its ray: the gain it integrates (summed if a later check
/// merged into the same request) and the positions as checked.
#[derive(Clone, Copy, Debug)]
struct PendingSight {
    observer: Entity,
    gain: f32,
    eye: WorldPos,
    chest: WorldPos,
    feet: WorldPos,
    velocity: Vec3,
}

/// A sight check that found its target in sight this tick.
#[derive(Clone, Copy, Debug)]
struct SightHit {
    observer: ActorKey,
    entity: Entity,
    target: PerceptTarget,
    gain: f32,
    pos: WorldPos,
    velocity: Vec3,
}

/// Reused buffers (no per-tick allocation once warm).
#[derive(Resource)]
struct SightScratch {
    global: EffectivePerception,
    targets: Vec<TargetSnap>,
    pending: HashMap<(ActorKey, PerceptTarget), PendingSight>,
    hits: Vec<SightHit>,
    los: LosScratch,
}

impl Default for SightScratch {
    fn default() -> Self {
        let s = AiPerceptionSettings::default();
        Self {
            global: global_perception(&s, 0),
            targets: Vec::new(),
            pending: HashMap::with_capacity(RAY_QUEUE_CAPACITY),
            hits: Vec::new(),
            los: LosScratch::default(),
        }
    }
}

/// The resolve of an actor with neither profile nor states: exactly the globals.
fn global_perception(s: &AiPerceptionSettings, tick: u64) -> EffectivePerception {
    resolve(&PerceptionProfile::default(), &PerceptionStates::new(), |_| None, &s.awareness, &s.sight, &s.search, tick)
}

/// Whether `meter` had contact with `target` within the last two check periods (the previous
/// check, one tick of ray deferral allowed): its range is then `lose_m` (UE `LoseSightRadius`).
pub fn seen_recently(meter: &AwarenessMeter, target: PerceptTarget, tick: u64, period_ticks: u32) -> bool {
    meter
        .get(target)
        .and_then(|a| a.last_contact)
        .is_some_and(|c| tick.saturating_sub(c) <= 2 * u64::from(period_ticks.max(1)))
}

// --- Line of sight on the ground probe --------------------------------------------------------

/// Reused query and hit buffers for [`ProbeLos`].
#[derive(Debug, Default)]
pub struct LosScratch {
    queries: Vec<ProbeQuery>,
    hits: Vec<Option<GroundHit>>,
}

/// [`LineOfSight`] on the app's ground probe (`sf_physics_bevy::Physics` in game: statics, props
/// and terrain; characters avoid each other through ORCA and never block sight). A segment is
/// clear when the probe finds no hit within its length. One batched `probe` call per batch.
pub struct ProbeLos<'a, G: ?Sized> {
    pub probe: &'a G,
    pub scratch: &'a mut LosScratch,
}

impl<G: GroundProbe + ?Sized> LineOfSight for ProbeLos<'_, G> {
    fn cast(&mut self, rays: &[LosRay], clear: &mut [bool]) {
        let s = &mut *self.scratch;
        s.queries.clear();
        for r in rays {
            let d = r.from.delta_f32(r.to);
            let len = d.length();
            let dir = if len > MIN_RAY_M { d / len } else { Vec3::Y };
            s.queries.push(ProbeQuery::ray(r.from, dir, len));
        }
        s.hits.clear();
        s.hits.resize(rays.len(), None);
        self.probe.probe(&s.queries, &mut s.hits);
        for ((c, h), q) in clear.iter_mut().zip(&s.hits).zip(&s.queries) {
            *c = q.max <= MIN_RAY_M || h.is_none();
        }
    }
}

// --- Plugin -----------------------------------------------------------------------------------

/// The sight system over ground probe `G` (the same resource the app gives
/// `SfControllerPlugin::<G>`; until it exists, requests wait in the queue). Adds
/// [`AiPerceptionSettings`] (bound to the perception tunables), [`AiPerceptClock`], [`AiRays`],
/// [`AiSightCache`], [`AiPerceptionStateDefs`] and the chain (light exposure included; the host
/// turns it on by inserting [`crate::AiLightInputs`]) in [`AiPerceptionSystems::Sight`].
pub struct AiSightPlugin<G> {
    probe: PhantomData<fn() -> G>,
}

impl<G> Default for AiSightPlugin<G> {
    fn default() -> Self {
        Self { probe: PhantomData }
    }
}

impl<G: GroundProbe + Resource> Plugin for AiSightPlugin<G> {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiPerceptionSettings>()
            .init_resource::<AiPerceptClock>()
            .init_resource::<AiRays>()
            .init_resource::<AiSightCache>()
            .init_resource::<AiPerceptionStateDefs>()
            .init_resource::<SightScratch>()
            .init_resource::<LightScratch>()
            .configure_sets(FixedUpdate, AiPerceptionSystems::Sight.before(ControllerSystems::Drive))
            .add_systems(
                FixedUpdate,
                (begin_tick, resolve_profiles, sample_light::<G>, queue_sight, cast_rays::<G>, apply_sight)
                    .chain()
                    .in_set(AiPerceptionSystems::Sight),
            );
        crate::hearing::add(app);
        add_perception_tunables(app);
    }
}

// --- Systems ----------------------------------------------------------------------------------

fn begin_tick(
    fixed: Option<Res<Time<Fixed>>>,
    settings: Res<AiPerceptionSettings>,
    mut clock: ResMut<AiPerceptClock>,
    mut scratch: ResMut<SightScratch>,
    mut timers: Query<&mut AiSightTimer, Added<AiSightTimer>>,
) {
    clock.tick += 1;
    if let Some(fixed) = fixed {
        clock.tick_s = fixed.timestep().as_secs_f32();
    }
    for mut t in &mut timers {
        t.0 = SightTimer::starting_at(clock.tick);
    }
    scratch.global = global_perception(&settings, clock.tick);
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn resolve_profiles(
    mut commands: Commands,
    settings: Res<AiPerceptionSettings>,
    defs: Res<AiPerceptionStateDefs>,
    clock: Res<AiPerceptClock>,
    mut actors: Query<
        (Entity, Option<Ref<AiPerceptionProfile>>, Option<Mut<AiPerceptionStates>>, Option<&mut AiEffectivePerception>),
        Or<(With<AiPerceptionProfile>, With<AiPerceptionStates>)>,
    >,
    stale: Query<Entity, (With<AiEffectivePerception>, Without<AiPerceptionProfile>, Without<AiPerceptionStates>)>,
) {
    let now = clock.tick;
    let reload = settings.is_changed() || defs.is_changed();
    let default_profile = PerceptionProfile::default();
    let no_states = PerceptionStates::new();
    for (entity, profile, mut states, eff) in &mut actors {
        let expired = eff.as_ref().is_some_and(|e| e.0.needs_rebuild(now));
        if expired && let Some(s) = states.as_mut() {
            s.0.expire(now);
        }
        let changed = profile.as_ref().is_some_and(Ref::is_changed) || states.as_ref().is_some_and(Mut::is_changed);
        if !(reload || expired || changed || eff.is_none()) {
            continue;
        }
        let resolved = resolve(
            profile.as_deref().map_or(&default_profile, |p| &p.0),
            states.as_deref().map_or(&no_states, |s| &s.0),
            |id| defs.0.get(id),
            &settings.awareness,
            &settings.sight,
            &settings.search,
            now,
        );
        match eff {
            Some(mut e) => e.0 = resolved,
            None => {
                commands.entity(entity).insert(AiEffectivePerception(resolved));
            }
        }
    }
    for entity in &stale {
        commands.entity(entity).remove::<AiEffectivePerception>();
    }
}

/// The grid cell of `p` (XZ, `cell_m` meters).
#[allow(clippy::cast_possible_truncation)] // world positions are far inside i64 cells
fn cell_of(p: WorldPos, cell_m: f64) -> (i64, i64) {
    let v = p.to_f64();
    ((v.x / cell_m).floor() as i64, (v.z / cell_m).floor() as i64)
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_lines, clippy::cast_precision_loss)]
fn queue_sight(
    settings: Res<AiPerceptionSettings>,
    clock: Res<AiPerceptClock>,
    mut rays: ResMut<AiRays>,
    cache: Res<AiSightCache>,
    mut scratch: ResMut<SightScratch>,
    targets: Query<(Entity, &Perceivable, Option<&Mover>, Option<&PerceivablePose>, Option<&AiLightExposure>)>,
    mut observers: Query<
        (Entity, &AiActor, &AiObserver, &Mover, &AiAwareness, &mut AiSightTimer, Option<&AiEffectivePerception>),
        Without<Dormant>,
    >,
) {
    let s = &*settings;
    let now = clock.clock();
    let tick = clock.tick;
    let cell_m = f64::from(s.sight.lose_sight_range_m.max(s.sight.sight_range_m).max(1.0));
    let period = s.budget.sight_period_ticks(clock.tick_s);
    let SightScratch { global, targets: snaps, pending, hits, .. } = &mut *scratch;
    rays.refused = 0;

    // Grid prefilter: every target bucketed by XZ cell, sorted (cell, target) for a stable order.
    snaps.clear();
    for (entity, p, mover, pose, light) in &targets {
        let from_mover = mover.map(|m| (m.loco.position, m.loco.motion.velocity));
        let Some((feet, velocity)) = from_mover.or(pose.map(|q| (q.feet, q.velocity))) else { continue };
        snaps.push(TargetSnap {
            cell: cell_of(feet, cell_m),
            target: p.target,
            entity,
            feet,
            chest: feet.offset(Vec3::Y * p.chest_m),
            head: feet.offset(Vec3::Y * p.head_m),
            velocity,
            light: light.map(|l| l.0),
        });
    }
    snaps.sort_unstable_by_key(|t| (t.cell, t.target));

    for (entity, actor, obs, mover, meter, mut timer, eff) in &mut observers {
        let Some(elapsed) = timer.0.poll(actor.0, tick, period) else { continue };
        let dt = elapsed as f32 * clock.tick_s;
        let eff = eff.map_or(&*global, |e| &e.0);
        let feet = mover.loco.position;
        let eye = feet.offset(Vec3::Y * obs.eye_m);
        let facing = forward(mover.loco.motion.yaw);
        let reach = f64::from(eff.sight.lose_m.max(eff.sight.range_m).max(0.0));
        // Small: the reach over the cell size, at most the profile's 4x range clamp.
        #[allow(clippy::cast_possible_truncation)]
        let r = (reach / cell_m).ceil() as i64;
        let (cx, cz) = cell_of(feet, cell_m);
        for x in cx - r..=cx + r {
            for z in cz - r..=cz + r {
                let lo = snaps.partition_point(|t| t.cell < (x, z));
                let hi = lo + snaps[lo..].partition_point(|t| t.cell == (x, z));
                for t in &snaps[lo..hi] {
                    if t.entity == entity || t.target == PerceptTarget::Actor(actor.0) {
                        continue;
                    }
                    let check = SightCheck {
                        observer: eye,
                        facing,
                        target: t.chest,
                        seen_before: seen_recently(&meter.0, t.target, tick, period),
                        visibility: light_only_visibility(t.light, eff.sight.darkness),
                        alertness: meter.0.alertness(&eff.cfg, t.target, now),
                    };
                    let Some(gain) = sight_gain(&eff.sight, &s.sight, &s.budget, &check, dt) else { continue };
                    if let Some(outcome) = cache.0.get(actor.0, t.target, eye, t.chest, now, &s.budget) {
                        if outcome.visible() {
                            hits.push(SightHit { observer: actor.0, entity, target: t.target, gain, pos: t.feet, velocity: t.velocity });
                        }
                        continue;
                    }
                    let req = RayRequest::sight(actor.0, t.target, eye, t.chest, t.head);
                    if rays.queue.push(req, tick).is_err() {
                        rays.refused += 1;
                        continue;
                    }
                    // A check merged into a queued request adds its gain (the time it integrates).
                    let key = (actor.0, t.target);
                    let gain = pending.get(&key).map_or(0.0, |p| p.gain) + gain;
                    pending.insert(key, PendingSight { observer: entity, gain, eye, chest: t.chest, feet: t.feet, velocity: t.velocity });
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn cast_rays<G: GroundProbe + Resource>(
    settings: Res<AiPerceptionSettings>,
    clock: Res<AiPerceptClock>,
    probe: Option<Res<G>>,
    mut rays: ResMut<AiRays>,
    mut cache: ResMut<AiSightCache>,
    mut scratch: ResMut<SightScratch>,
) {
    let Some(probe) = probe else { return };
    let SightScratch { pending, hits, los, .. } = &mut *scratch;
    let budget = settings.budget.max_rays_per_tick.saturating_sub(rays.light_cast);
    let AiRays { queue, results, stats, .. } = &mut *rays;
    let mut backend = ProbeLos { probe: &*probe, scratch: los };
    *stats = queue.process(budget, &mut backend, results);
    for r in results.iter().filter(|r| r.kind == RayKind::Sight) {
        let RaySubject::Target(target) = r.subject else { continue };
        let Some(p) = pending.remove(&(r.owner, target)) else { continue };
        if !cache.0.insert(r.owner, target, p.eye, p.chest, clock.tick, r.outcome) {
            cache.0.prune(clock.clock(), &settings.budget);
            let _ = cache.0.insert(r.owner, target, p.eye, p.chest, clock.tick, r.outcome);
        }
        if r.outcome.visible() {
            hits.push(SightHit { observer: r.owner, entity: p.observer, target, gain: p.gain, pos: p.feet, velocity: p.velocity });
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn apply_sight(
    clock: Res<AiPerceptClock>,
    mut scratch: ResMut<SightScratch>,
    mut observers: Query<(&mut AiAwareness, Option<&AiEffectivePerception>), (With<AiObserver>, Without<Dormant>)>,
) {
    let now = clock.clock();
    let SightScratch { global, hits, .. } = &mut *scratch;
    hits.sort_unstable_by_key(|h| (h.observer, h.target));
    for h in hits.drain(..) {
        let Ok((mut meter, eff)) = observers.get_mut(h.entity) else { continue };
        let cfg = eff.map_or(&global.cfg, |e| &e.0.cfg);
        let stimulus = Stimulus::Sight { target: h.target, gain: h.gain, pos: h.pos, velocity: h.velocity };
        let _ = meter.0.apply(cfg, &stimulus, now);
    }
    for (mut meter, eff) in &mut observers {
        let cfg = eff.map_or(&global.cfg, |e| &e.0.cfg);
        meter.0.update(cfg, now);
    }
}
