//! Body proxies (character-shapes.md "Physics integration", "Impacts"; slice S3; feature `controller`).
//!
//! A character root with [`BodyProxies`] gets one kinematic capsule per shape of its [`ShapeSet`] in the active
//! worldspace's physics world, on [`Layer::BODY_PROXY`]: touched only by what moves (thrown balls, props, debris,
//! carried items, ragdolls), never by the world, characters, sensors or other proxies, and outside the mover's
//! ground and push layers. Whoever shows the body writes the posed shapes into the component every frame (S3: an
//! active-ragdoll layer holding the body, [`ProxyHolder::Layer`]); the framework does the rest:
//!
//! - [`ShapeSystems::Activate`] (`Update`, batched): inserts a set's bodies when the component appears, removes them
//!   when it goes (or the entity, or the worldspace changes), and turns the frame's proxy hits into
//!   [`ImpactOnCharacter`] with `shape: Some(index)`. Inserts and removals happen only here, never inside a tick.
//! - [`ShapeSystems::Sync`] (`FixedUpdate`, before the step, one pass over every set): each proxy's kinematic target
//!   for the step's end is its last written pose extrapolated by its velocity to the step's time (the capsule's AN-7
//!   rule: poses come at frame rate, steps at 60 Hz). The kinematic velocity this gives is what
//!   `sf_physics::impact_impulse` subtracts, so a limb swinging into a resting ball is no hit.
//! - After each step: one impact per (incoming body, character) per step, the strongest (a ball touching forearm and
//!   chest at once pushes once); every new contact start is a new impact, so rapid balls each count.
//! - Counters (S0): [`ProxyStats`] per frame, [`ShapeWindow`] per second (the host's `shapes:` log line) and the
//!   `shapes/...` diag counters (bench JSON).
//!
//! The capsule of a character with proxies is the host's business (S3: a held body is `Dormant`, so it has none);
//! S4 adds the animated tier with the capsule's contact mute.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bevy_app::{App, First, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::Time;
use sf_coords::WorldTransform;
use sf_physics::{BodyDesc, BodyId, BodyKind, Layer, PhysicsEvent, ShapeRef};
use sf_ragdoll::ShapeSet;
use sf_world::WorldspaceId;

use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;

use crate::character::{CharacterPhysics, ImpactOnCharacter};
use crate::{Physics, PhysicsEvents, PhysicsSystems};

/// Longest extrapolation of a written pose, s: a writer that stops writing leaves its proxies where they were
/// heading for at most this long, not flying off.
pub const MAX_LEAD: f64 = 0.1;

/// Who drives a set's poses (routes its impacts; character-shapes.md "Impacts").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProxyHolder {
    /// The character's own animation (S4).
    Animation,
    /// A physics layer that holds the body (an external active ragdoll): every contact start is a further push.
    Layer,
    /// A dead body at rest (S6).
    Corpse,
}

/// A character's posed shapes, on its root (the entity [`ImpactOnCharacter`] names). The writer updates `poses`,
/// `velocities` and `at` every frame once the shown pose is final, in place; removing the component removes the
/// bodies. A set whose `poses` don't match `set.shapes` in length is ignored.
#[derive(Component, Clone, Debug)]
pub struct BodyProxies {
    pub set: Arc<ShapeSet>,
    /// Each shape's world frame (capsule along its Y axis). The scale is applied when the body is inserted only.
    pub poses: Vec<WorldTransform>,
    /// Each shape's linear velocity, m/s (world): the extrapolation to each step's time.
    pub velocities: Vec<Vec3>,
    /// The virtual time the poses are at, s (`Time<Virtual>` elapsed; an interpolated pose is a step behind it).
    pub at: f64,
    pub holder: ProxyHolder,
}

impl BodyProxies {
    /// The skeleton joint shape `shape` follows.
    #[must_use]
    pub fn joint(&self, shape: u16) -> Option<u16> {
        self.set.joint(shape)
    }
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeSystems {
    /// `Update`: proxy hits to impacts, then inserts and removals. Writers of [`BodyProxies`] order before it.
    Activate,
    /// `FixedUpdate`, before the step: kinematic targets.
    Sync,
}

/// Counters (character-shapes.md "Budget and measurement"); [`ShapeWindow`] sums them per second.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ProxyStats {
    pub sets: usize,
    /// Proxy bodies in the solver.
    pub proxies: usize,
    /// Impacts on proxies this frame.
    pub impacts: u32,
    /// The last tick's sync (0 with no sets) and the last frame's activation, µs.
    pub sync_us: f32,
    pub activate_us: f32,
    /// Sets whose bodies went into / out of the solver this frame (a takeover starting, a hand-back).
    pub activations: u32,
    pub removals: u32,
}

/// One window (about a second) of [`ProxyStats`]: the `shapes:` log line and the S0 bench numbers.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShapeCounters {
    pub secs: f32,
    pub frames: u32,
    /// Peak sets and proxy bodies over the window.
    pub sets: usize,
    pub proxies: usize,
    /// Fixed ticks that synced, their mean and worst sync, µs.
    pub ticks: u32,
    pub sync_us: f32,
    pub sync_max_us: f32,
    /// Mean sync per proxy body, µs (the mean tick over the peak proxies).
    pub sync_per_proxy_us: f32,
    /// Worst frame's activation, µs.
    pub activate_us: f32,
    pub activations: u32,
    pub removals: u32,
    pub impacts: u32,
}

impl ShapeCounters {
    /// Anything to report: sets live, or bodies went in or out.
    #[must_use]
    pub fn active(&self) -> bool {
        self.sets > 0 || self.activations > 0 || self.removals > 0
    }

    /// Every number finite (the S0 sanity test).
    #[must_use]
    pub fn is_finite(&self) -> bool {
        [self.secs, self.sync_us, self.sync_max_us, self.sync_per_proxy_us, self.activate_us].iter().all(|v| v.is_finite())
    }
}

impl std::fmt::Display for ShapeCounters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "shapes: {} sets, {} proxies (peak) | sync {:.1} us/tick mean, {:.1} max, {} ticks, {:.3} us/proxy | activate {:.1} us worst, \
             {} activations, {} removals | {} impacts over {:.1} s",
            self.sets,
            self.proxies,
            self.sync_us,
            self.sync_max_us,
            self.ticks,
            self.sync_per_proxy_us,
            self.activate_us,
            self.activations,
            self.removals,
            self.impacts,
            self.secs
        )
    }
}

/// [`ProxyStats`] summed over windows of [`ShapeWindow::SECS`] (`Update`, after [`ShapeSystems::Activate`]); hosts log
/// `last` as the `shapes:` line when `fresh` and [`ShapeCounters::active`]. The same numbers go to the diag counters
/// (`shapes/...`, the bench JSON) every frame.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ShapeWindow {
    /// The window being summed (`sync_us` holds the sum until it closes).
    acc: ShapeCounters,
    /// The last closed window.
    pub last: Option<ShapeCounters>,
    /// `last` closed this frame.
    pub fresh: bool,
}

impl ShapeWindow {
    /// Window length, s.
    pub const SECS: f32 = 1.0;

    fn tick(&mut self, sync_us: f32) {
        self.acc.ticks += 1;
        self.acc.sync_us += sync_us;
        self.acc.sync_max_us = self.acc.sync_max_us.max(sync_us);
    }

    fn frame(&mut self, dt: f32, s: &ProxyStats) {
        let a = &mut self.acc;
        a.secs += dt;
        a.frames += 1;
        a.sets = a.sets.max(s.sets);
        a.proxies = a.proxies.max(s.proxies);
        a.activate_us = a.activate_us.max(s.activate_us);
        a.activations += s.activations;
        a.removals += s.removals;
        a.impacts += s.impacts;
        self.fresh = a.secs >= Self::SECS;
        if self.fresh {
            let mut done = std::mem::take(a);
            #[allow(clippy::cast_precision_loss)]
            {
                done.sync_us /= done.ticks.max(1) as f32;
                done.sync_per_proxy_us = done.sync_us / done.proxies.max(1) as f32;
            }
            self.last = Some(done);
        }
    }
}

struct ProxySet {
    worldspace: WorldspaceId,
    set: Arc<ShapeSet>,
    ids: Vec<BodyId>,
}

#[derive(Resource, Default)]
pub(crate) struct Proxies {
    sets: HashMap<Entity, ProxySet>,
    /// Proxy body → (root, shape).
    by_body: HashMap<BodyId, (Entity, u16)>,
    /// Events of this frame's steps already looked at.
    cursor: usize,
    /// Impacts gathered after the steps, written in `Update`.
    pending: Vec<ImpactOnCharacter>,
}

impl Proxies {
    /// A root's proxy bodies (tests).
    #[cfg(test)]
    pub(crate) fn ids(&self, root: Entity) -> Option<&[BodyId]> {
        self.sets.get(&root).map(|s| s.ids.as_slice())
    }
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<Proxies>()
        .init_resource::<ProxyStats>()
        .init_resource::<ShapeWindow>()
        .configure_sets(FixedUpdate, ShapeSystems::Sync.before(PhysicsSystems::Step))
        .configure_sets(Update, ShapeSystems::Activate.after(PhysicsSystems::Stream))
        .add_systems(First, begin_frame)
        .add_systems(FixedUpdate, (sync.in_set(ShapeSystems::Sync), collect.after(PhysicsSystems::Step)))
        .add_systems(Update, ((impacts, activate).chain().in_set(ShapeSystems::Activate), publish.after(ShapeSystems::Activate)));
}

/// The frame's counters into the window and the diag counters (`shapes/...`).
#[allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)]
fn publish(time: Res<Time>, stats: Res<ProxyStats>, mut window: ResMut<ShapeWindow>, counters: Option<Res<DiagCounters>>) {
    let s = *stats;
    window.frame(time.delta_secs(), &s);
    let Some(c) = counters else { return };
    c.set_named("shapes/sets", unit::COUNT, s.sets as f64);
    c.set_named("shapes/proxies", unit::COUNT, s.proxies as f64);
    c.set_named("shapes/sync/ms", unit::MS, f64::from(s.sync_us) * 1e-3);
    c.add_named("shapes/activate/ms", unit::MS, f64::from(s.activate_us) * 1e-3);
    c.add_named("shapes/activations", unit::COUNT, f64::from(s.activations));
    c.add_named("shapes/removals", unit::COUNT, f64::from(s.removals));
    c.add_named("shapes/impacts", unit::COUNT, f64::from(s.impacts));
}

fn begin_frame(mut proxies: ResMut<Proxies>) {
    proxies.cursor = 0;
}

/// Inserts and removes proxy bodies (batched, `Update`).
#[allow(clippy::needless_pass_by_value)]
fn activate(mut physics: ResMut<Physics>, mut proxies: ResMut<Proxies>, mut stats: ResMut<ProxyStats>, roots: Query<(Entity, &BodyProxies)>) {
    let started = Instant::now();
    let active = physics.worlds.active().cloned();
    let Proxies { sets, by_body, .. } = &mut *proxies;
    let before = sets.len();
    // Gone, replaced, or in another worldspace: out.
    sets.retain(|e, p| {
        let keep = Some(&p.worldspace) == active.as_ref() && roots.get(*e).is_ok_and(|(_, b)| Arc::ptr_eq(&b.set, &p.set));
        if !keep {
            for id in &p.ids {
                by_body.remove(id);
            }
            if let Some(w) = physics.worlds.get_mut(&p.worldspace) {
                for id in &p.ids {
                    w.remove(*id);
                }
            }
        }
        keep
    });
    let kept = sets.len();
    if let Some(ws) = active
        && let Some(w) = physics.worlds.get_mut(&ws)
    {
        for (e, b) in &roots {
            let valid = b.poses.len() == b.set.len() && b.poses.iter().all(|p| p.rotation.is_finite() && p.pos.local.is_finite());
            if sets.contains_key(&e) || !valid {
                continue;
            }
            let ids = b
                .set
                .shapes
                .iter()
                .zip(&b.poses)
                .enumerate()
                .map(|(i, (s, pose))| {
                    let shape = ShapeRef::Capsule { r: s.capsule.radius, half: s.capsule.half };
                    let id = w.insert(&BodyDesc::new(BodyKind::Kinematic, *pose, shape, Layer::BODY_PROXY).with_user(i as u64));
                    #[allow(clippy::cast_possible_truncation)]
                    by_body.insert(id, (e, i as u16));
                    id
                })
                .collect();
            sets.insert(e, ProxySet { worldspace: ws.clone(), set: b.set.clone(), ids });
        }
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        stats.removals = (before - kept) as u32;
        stats.activations = (sets.len() - kept) as u32;
    }
    stats.sets = sets.len();
    stats.proxies = by_body.len();
    stats.activate_us = started.elapsed().as_secs_f32() * 1e6;
}

/// `FixedUpdate`, before the step: every proxy targets its written pose extrapolated to the step's end.
#[allow(clippy::needless_pass_by_value)]
fn sync(
    time: Res<Time>,
    mut physics: ResMut<Physics>,
    proxies: Res<Proxies>,
    roots: Query<&BodyProxies>,
    mut stats: ResMut<ProxyStats>,
    mut window: ResMut<ShapeWindow>,
) {
    if proxies.sets.is_empty() {
        stats.sync_us = 0.0;
        return;
    }
    let started = Instant::now();
    // In `FixedUpdate` the generic clock is the fixed one: this step's end.
    let now = time.elapsed_secs_f64();
    let Some(w) = physics.worlds.active_world_mut() else { return };
    for (root, p) in &proxies.sets {
        let Ok(b) = roots.get(*root) else { continue };
        #[allow(clippy::cast_possible_truncation)]
        let lead = (now - b.at).clamp(0.0, MAX_LEAD) as f32;
        for ((id, pose), v) in p.ids.iter().zip(&b.poses).zip(b.velocities.iter().chain(std::iter::repeat(&Vec3::ZERO))) {
            let v = if v.is_finite() { *v } else { Vec3::ZERO };
            w.set_kinematic_target(*id, WorldTransform { pos: pose.pos.offset(v * lead), ..*pose });
        }
    }
    stats.sync_us = started.elapsed().as_secs_f32() * 1e6;
    window.tick(stats.sync_us);
}

/// After each step: hits on proxies, the strongest per (incoming body, character) of the step.
#[allow(clippy::needless_pass_by_value)]
fn collect(events: Res<PhysicsEvents>, settings: Res<CharacterPhysics>, mut proxies: ResMut<Proxies>) {
    let Proxies { by_body, cursor, pending, .. } = &mut *proxies;
    let new = events.0.get(*cursor..).unwrap_or_default();
    *cursor = events.0.len();
    if by_body.is_empty() {
        return;
    }
    let mut keys: Vec<(BodyId, Entity)> = Vec::new();
    let start = pending.len();
    for e in new {
        let PhysicsEvent::Hit { target, by, impulse, point, direction } = *e else { continue };
        if impulse < settings.impact_threshold {
            continue;
        }
        let Some(&(entity, shape)) = by_body.get(&target) else { continue };
        let impact = ImpactOnCharacter { entity, impulse, point, direction, shape: Some(shape) };
        match keys.iter().position(|k| *k == (by, entity)) {
            Some(i) if pending[start + i].impulse < impulse => pending[start + i] = impact,
            Some(_) => {}
            None => {
                keys.push((by, entity));
                pending.push(impact);
            }
        }
    }
}

/// The steps' proxy impacts → [`ImpactOnCharacter`].
fn impacts(mut proxies: ResMut<Proxies>, mut stats: ResMut<ProxyStats>, mut out: MessageWriter<ImpactOnCharacter>) {
    #[allow(clippy::cast_possible_truncation)]
    let n = proxies.pending.len() as u32;
    stats.impacts = n;
    if n > 0 {
        out.write_batch(proxies.pending.drain(..));
    }
}
