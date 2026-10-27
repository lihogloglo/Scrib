//! Bevy glue for the Bevy-free locomotion core (`sf_controller`).
//! Spec: `docs/specs/locomotion.md` "Bevy integration: `sf_controller_bevy` as built".
//!
//! A moving character is an entity with a [`Mover`] (its saved
//! [`Locomotion`] state and [`MoverParams`]), a [`MoveIntent`] that a driver
//! writes (player input, AI, the adapter's path-walk scaffold), and
//! optionally an [`Avoider`] (it takes part in local avoidance as a reactive
//! agent). Other avoidance participants that never react (the player, the
//! camera) carry an [`AvoidObstacle`] the host keeps current. Per frame:
//!
//! 1. [`ControllerSystems::Drive`] (`Update`): host systems write
//!    [`MoveIntent`]s and [`AvoidObstacle`]s. Empty here.
//! 2. [`ControllerSystems::Step`] (`Update`): one system, in the industry
//!    order (UE `CharacterMovementComponent` + RVO, Unity `NavMeshAgent`,
//!    Detour crowd: steering → avoidance → movement): ORCA
//!    ([`Avoidance::solve`]) turns every reactive agent's desired velocity
//!    into a collision-free one, then [`KinematicMover::step_batch`] moves
//!    every character with one batched probe call per stage against the
//!    ground resource `G`, then the root `Transform`s are written.
//! 3. [`ControllerSystems::Feed`] (`PostUpdate`, in
//!    [`AnimSystems::Probe`]): each mover's [`Animator`] gets its velocity,
//!    `moved` delta and turn rate in character space, the adapter-named
//!    locomotion parameters ([`AnimParamNames`]), and per-leg ground
//!    samples (one batched probe at the graph's `probe_origins`) for foot
//!    placement.
//!
//! The ground `G` is any [`GroundProbe`] resource (terrain heights plus the
//! static query world in a game); the host keeps it current.
//!
//! Positions: [`Mover::frame_origin`] is the world position of the
//! entity's parent frame (unrotated, unscaled), so the root `Transform` is
//! `render_position - frame_origin`. This keeps the crate free of the
//! floating-origin and streaming crates; the host sets it at spawn.

use std::marker::PhantomData;
use std::time::Instant;

use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec2, Vec3};
use bevy_time::Time;
use bevy_transform::components::Transform;
use sf_anim_bevy::{AnimSystems, Animator};
use sf_anim_graph::GroundSample;
use sf_controller::{
    AvoidAgent, Avoidance, CharacterFrame, Gait, GroundHit, KinematicMover, MoveInput, ProbeQuery,
};
pub use sf_controller::{
    AvoidParams, AvoidRole, GroundProbe, Locomotion, LocomotionIntent, MoverMode, MoverParams,
    RootMotion,
};
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;

#[cfg(test)]
mod tests;

/// The sets, chained within their schedules (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerSystems {
    /// `Update`: intents and obstacles (host systems).
    Drive,
    /// `Update`: avoidance, mover, transform write-back.
    Step,
    /// `PostUpdate`, inside `AnimSystems::Probe`: the animation feed.
    Feed,
}

/// Registers the stage over the ground resource `G`. Gizmos: feature `debug`.
pub struct SfControllerPlugin<G> {
    ground: PhantomData<fn() -> G>,
}

impl<G> Default for SfControllerPlugin<G> {
    fn default() -> Self {
        Self {
            ground: PhantomData,
        }
    }
}

impl<G: GroundProbe + Resource> Plugin for SfControllerPlugin<G> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllerSettings>()
            .init_resource::<ControllerStats>()
            .configure_sets(
                Update,
                (ControllerSystems::Drive, ControllerSystems::Step).chain(),
            )
            .configure_sets(
                PostUpdate,
                ControllerSystems::Feed.in_set(AnimSystems::Probe),
            )
            .add_systems(Update, step::<G>.in_set(ControllerSystems::Step))
            .add_systems(
                PostUpdate,
                feed_animators::<G>.in_set(ControllerSystems::Feed),
            );
        // Drawn only when the app added Bevy's gizmo plugin (headless apps and
        // tests run without it).
        #[cfg(feature = "debug")]
        app.init_resource::<ControllerGizmos>().add_systems(
            PostUpdate,
            draw_movers
                .after(ControllerSystems::Feed)
                .run_if(resource_exists::<bevy_gizmos::config::GizmoConfigStore>),
        );
    }
}

// --- Components and resources ------------------------------------------------------------

/// A character moved by the kinematic mover.
#[derive(Component, Clone, Debug)]
pub struct Mover {
    pub loco: Locomotion,
    pub params: MoverParams,
    /// World position of the parent frame the entity's `Transform` is
    /// relative to (unrotated and unscaled): the host sets it at spawn.
    pub frame_origin: WorldPos,
    /// Root motion of the animation's last update (capsule-driven: default).
    pub root: RootMotion,
    /// The entity with this character's [`Animator`], if animated.
    pub animator: Option<Entity>,
    /// The intent the mover ran with last step (after avoidance).
    pub applied: LocomotionIntent,
    /// Scale of the animated body under the mover frame (race height and
    /// weight). The graph works in the unscaled skeleton space, so the feed
    /// scales its probe origins up and its samples and motion back down.
    pub body_scale: Vec3,
    /// Measured horizontal velocity, character space, m/s: the last step's
    /// displacement over its time (after avoidance, collision and step
    /// handling), lightly smoothed. The feed gives the graph this, never the
    /// intent, so playback follows the distance actually travelled.
    pub measured: Vec3,
    /// Measured yaw rate, rad/s, positive to the left.
    pub measured_turn: f32,
    /// What the feed told the graph last (its hysteresis state).
    pub gait: FeedGait,
}

/// The feed's locomotion state, kept between frames for hysteresis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeedGait {
    pub moving: bool,
    pub running: bool,
}

/// Halflife of the measured velocity's smoothing, seconds: removes the
/// spikes of step-ups and collision slides, keeps starts and stops crisp.
pub const MEASURE_HALFLIFE: f32 = 0.05;

impl Mover {
    /// Standing at `position` facing `yaw`, in a parent frame at `frame_origin`.
    pub fn new(position: WorldPos, yaw: f32, params: MoverParams, frame_origin: WorldPos) -> Self {
        Self {
            loco: Locomotion::new(position, yaw),
            params,
            frame_origin,
            root: RootMotion::default(),
            animator: None,
            applied: LocomotionIntent::IDLE,
            body_scale: Vec3::ONE,
            measured: Vec3::ZERO,
            measured_turn: 0.0,
            gait: FeedGait::default(),
        }
    }

    /// Yaw (about +Y, 0 facing +Z) of a rotation's forward (+Z) axis.
    pub fn yaw_of(rotation: Quat) -> f32 {
        let f = rotation * Vec3::Z;
        f.x.atan2(f.z)
    }

    /// The root `Transform` translation and rotation for the current state.
    pub fn root_transform(&self) -> (Vec3, Quat) {
        (
            self.frame_origin.delta_f32(self.loco.render_position()),
            self.loco.render_rotation(),
        )
    }
}

/// What the character's driver wants this frame (the driver's preference;
/// avoidance changes only what the mover gets, [`Mover::applied`]).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct MoveIntent(pub LocomotionIntent);

/// Takes part in local avoidance as a reactive agent.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Avoider {
    /// Coarse rank: of two opposing agents the lower yields (`AvoidAgent::priority`).
    pub priority: f32,
    /// Direction of travel along the route (the path segment), zero: the intent's.
    pub travel: Vec3,
    /// Fastest speed avoidance may ask for (at least the intent's speed).
    pub max_speed: f32,
}

impl Default for Avoider {
    fn default() -> Self {
        Self {
            priority: 0.0,
            travel: Vec3::ZERO,
            max_speed: 0.0,
        }
    }
}

/// Outside the host's processing range (locomotion.md "Processing range"):
/// the mover is not stepped, stands still (its velocity is zeroed when the
/// marker arrives), takes no part in avoidance, and its animator gets zero
/// motion and no ground samples. Removing the marker resumes it where it
/// stood. The host decides the range (in a streamed world: only where the
/// colliders around the character are loaded).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dormant;

/// An outside system owns this body (a ragdoll, a physics takeover, death):
/// [`Dormant`] comes with it (required), so no step and no motion or ground
/// samples in the feed, and the host's processing range must leave the mover
/// alone (never wake it, never touch its significance) until the owner
/// removes both. One concept for every owner (UE `DisableMovement` while
/// ragdolled; `PuppetMaster` disables the controller while unpinned).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[require(Dormant)]
pub struct Held;

/// A non-reactive avoidance participant the host updates every frame (the
/// player, a camera): the others avoid it, it never yields.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AvoidObstacle {
    pub position: WorldPos,
    pub velocity: Vec3,
    pub radius: f32,
}

/// Graph parameter names the feed writes (adapter data; `None` skips one).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnimParamNames {
    /// Bool: moving (measured speed above [`ControllerSettings::moving_speed`],
    /// until it falls below [`ControllerSettings::idle_speed`]).
    pub moving: Option<String>,
    /// Vec2: unit movement direction in the character frame, `x` right, `y` forward.
    pub direction: Option<String>,
    /// Bool: running (measured speed above [`ControllerSettings::run_speed`]
    /// until below [`ControllerSettings::run_exit`]; with no finite run
    /// speed, the applied gait is `Run` or `Sprint`).
    pub running: Option<String>,
    /// Float: yaw rate, rad/s, positive to the left.
    pub turn: Option<String>,
    /// Float: horizontal speed, m/s.
    pub speed: Option<String>,
}

/// Stage tuning.
#[derive(Resource, Clone, Debug)]
pub struct ControllerSettings {
    pub enabled: bool,
    pub avoidance: bool,
    pub avoid: AvoidParams,
    /// Longest step; longer frames (hitches, loading) are clamped.
    pub max_dt: f32,
    /// Measured speed above which a still character counts as moving for
    /// the graph, m/s…
    pub moving_speed: f32,
    /// …and below which a moving one stops (hysteresis: no idle/walk
    /// flicker while avoidance slows a walker down).
    pub idle_speed: f32,
    /// …or below which a moving one stops while its intent asks for no
    /// motion: it is braking, so the idle blend overlaps the velocity
    /// spring's settle instead of the loop playing at its rate floor (the
    /// stop transition fires on the stop request, as in Lyra). Keep it
    /// under `moving_speed` for hysteresis.
    pub stop_speed: f32,
    /// Measured speed above which the graph is told `running`, m/s (walk or
    /// run by the speed actually travelled; the adapter sets it between its
    /// walk and run cycles' speeds). Infinite: by the applied gait.
    pub run_speed: f32,
    /// Measured speed below which running stops.
    pub run_exit: f32,
    /// Per-leg ground probes start this far above the leg's probe origin…
    pub foot_probe_up: f32,
    /// …and reach this far below it.
    pub foot_probe_down: f32,
    /// Per-leg ground samples for foot placement (off: the graph gets none
    /// and its foot placement holds the animated pose).
    pub foot_probes: bool,
    pub params: AnimParamNames,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            avoidance: true,
            avoid: AvoidParams::default(),
            max_dt: 0.1,
            moving_speed: 0.3,
            idle_speed: 0.15,
            stop_speed: 0.25,
            run_speed: f32::INFINITY,
            run_exit: f32::INFINITY,
            foot_probe_up: 0.5,
            foot_probe_down: 0.75,
            foot_probes: true,
            params: AnimParamNames::default(),
        }
    }
}

/// Last frame's work (also counters `controller/...` with `SfDiagPlugin`).
#[derive(Resource, Clone, Debug, Default)]
pub struct ControllerStats {
    pub movers: usize,
    pub reactive: usize,
    pub obstacles: usize,
    pub avoid_us: f32,
    pub move_us: f32,
    pub write_us: f32,
    pub feed_us: f32,
    pub foot_probes: usize,
}

// --- Step ---------------------------------------------------------------------------------

/// Reused buffers of the step system.
#[derive(Default)]
struct StepScratch {
    mover: KinematicMover,
    avoidance: Avoidance,
    agents: Vec<AvoidAgent>,
    out: Vec<Vec3>,
    intents: Vec<LocomotionIntent>,
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::needless_pass_by_value
)]
fn step<G: GroundProbe + Resource>(
    time: Res<Time>,
    settings: Res<ControllerSettings>,
    ground: Option<Res<G>>,
    mut stats: ResMut<ControllerStats>,
    counters: Option<Res<DiagCounters>>,
    mut scratch: Local<StepScratch>,
    mut movers: Query<
        (&mut Mover, &MoveIntent, Option<&Avoider>, &mut Transform),
        Without<Dormant>,
    >,
    mut sleeping: Query<&mut Mover, (With<Dormant>, Added<Dormant>)>,
    obstacles: Query<&AvoidObstacle>,
) {
    // Newly dormant movers stop where they stand.
    for mut m in &mut sleeping {
        m.loco.motion.velocity = Vec3::ZERO;
        m.loco.motion.acceleration = Vec3::ZERO;
        m.loco.motion.angular_velocity = 0.0;
        std::mem::take(&mut m.loco.moved);
        m.applied = LocomotionIntent::IDLE;
        m.measured = Vec3::ZERO;
        m.measured_turn = 0.0;
        m.gait = FeedGait::default();
    }
    let dt = time.delta_secs().min(settings.max_dt);
    let Some(ground) = ground else { return };
    if !settings.enabled || dt <= 0.0 {
        return;
    }
    let StepScratch {
        mover,
        avoidance,
        agents,
        out,
        intents,
    } = &mut *scratch;
    let mut items: Vec<_> = movers.iter_mut().collect();
    intents.clear();
    intents.extend(items.iter().map(|(_, intent, _, _)| intent.0));

    // Avoidance: every mover is an agent (reactive with an `Avoider`), then the obstacles.
    let t0 = Instant::now();
    let mut reactive = 0;
    if settings.avoidance && !items.is_empty() {
        agents.clear();
        for (m, intent, avoider, _) in &items {
            let preferred = intent.0.desired_velocity;
            let (role, priority, travel, max_speed) = match avoider {
                Some(a) => {
                    reactive += 1;
                    (
                        AvoidRole::Reactive,
                        a.priority,
                        a.travel,
                        a.max_speed.max(preferred.length()),
                    )
                }
                None => (AvoidRole::NonReactive, 0.0, Vec3::ZERO, preferred.length()),
            };
            agents.push(AvoidAgent {
                position: m.loco.position,
                velocity: m.loco.motion.velocity,
                preferred,
                radius: m.params.radius,
                max_speed,
                role,
                priority,
                travel,
            });
        }
        for o in &obstacles {
            agents.push(AvoidAgent {
                position: o.position,
                velocity: o.velocity,
                preferred: o.velocity,
                radius: o.radius,
                max_speed: o.velocity.length(),
                role: AvoidRole::NonReactive,
                priority: f32::MAX,
                travel: Vec3::ZERO,
            });
        }
        out.clear();
        out.resize(agents.len(), Vec3::ZERO);
        avoidance.solve(agents, &settings.avoid, dt, out);
        for (i, (_, _, avoider, _)) in items.iter().enumerate() {
            if avoider.is_some() {
                intents[i].desired_velocity = out[i];
            }
        }
    }
    let avoid_us = micros(t0);

    // The mover: one batch, one probe call per stage.
    let t1 = Instant::now();
    {
        let mut inputs: Vec<MoveInput<'_>> = items
            .iter_mut()
            .zip(intents.iter())
            .map(|((m, _, _, _), intent)| {
                let Mover {
                    loco,
                    params,
                    root,
                    applied,
                    ..
                } = &mut **m;
                *applied = *intent;
                MoveInput {
                    loco,
                    intent: *intent,
                    params,
                    root: *root,
                }
            })
            .collect();
        mover.step_batch(&mut inputs, &*ground, dt);
    }
    let move_us = micros(t1);

    // Measured motion: what the step actually did, in the new frame.
    let keep = 0.5f32.powf(dt / MEASURE_HALFLIFE);
    for (m, _, _, _) in &mut items {
        let moved = m.loco.moved;
        let raw = Quat::from_rotation_y(-moved.yaw) * moved.translation / dt;
        let raw = if raw.is_finite() {
            raw.with_y(0.0)
        } else {
            Vec3::ZERO
        };
        let turn = if moved.yaw.is_finite() {
            moved.yaw / dt
        } else {
            0.0
        };
        m.measured = raw.lerp(m.measured, keep);
        m.measured_turn = turn + (m.measured_turn - turn) * keep;
    }

    let t2 = Instant::now();
    for (m, _, _, t) in &mut items {
        let (translation, rotation) = m.root_transform();
        if t.translation != translation || t.rotation != rotation {
            t.translation = translation;
            t.rotation = rotation;
        }
    }
    let write_us = micros(t2);

    *stats = ControllerStats {
        movers: items.len(),
        reactive,
        obstacles: obstacles.iter().len(),
        avoid_us,
        move_us,
        write_us,
        feed_us: stats.feed_us,
        foot_probes: stats.foot_probes,
    };
    if let Some(c) = counters {
        let ms = sf_diag_bevy::sf_diag::unit::MS;
        c.add_named("controller/avoid/ms", ms, f64::from(avoid_us) / 1000.0);
        c.add_named("controller/move/ms", ms, f64::from(move_us) / 1000.0);
        c.add_named("controller/write/ms", ms, f64::from(write_us) / 1000.0);
        #[allow(clippy::cast_precision_loss)]
        c.set_named(
            "controller/movers/count",
            sf_diag_bevy::sf_diag::unit::COUNT,
            items.len() as f64,
        );
    }
}

fn micros(since: Instant) -> f32 {
    since.elapsed().as_secs_f32() * 1e6
}

// --- Animation feed -----------------------------------------------------------------------

/// The graph-facing summary of one mover's last step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FeedValues {
    /// Velocity in character space.
    pub velocity: Vec3,
    pub speed: f32,
    pub moving: bool,
    /// Unit direction, `x` right, `y` forward (forward when still).
    pub direction: Vec2,
    pub running: bool,
    pub turn: f32,
}

impl FeedValues {
    /// From the mover's measured motion ([`Mover::measured`]), with the
    /// moving and running thresholds' hysteresis against `last`.
    pub fn of(mover: &Mover, settings: &ControllerSettings, last: FeedGait) -> Self {
        let braking = mover.applied.desired_velocity.length_squared() < 1e-6;
        Self::measured(
            mover.measured,
            mover.measured_turn,
            braking,
            mover.applied.gait,
            settings,
            last,
        )
    }

    /// From a measured character-space `velocity` and `turn` rate of any
    /// host (an action-layer body measures its drawn root): `braking` when
    /// the intent asks for no motion, `gait` the applied one.
    pub fn measured(
        velocity: Vec3,
        turn: f32,
        braking: bool,
        gait: Gait,
        settings: &ControllerSettings,
        last: FeedGait,
    ) -> Self {
        let flat = Vec2::new(-velocity.x, velocity.z);
        let speed = flat.length();
        let moving = speed
            > match (last.moving, braking) {
                (false, _) => settings.moving_speed,
                (true, false) => settings.idle_speed,
                (true, true) => settings.stop_speed.max(settings.idle_speed),
            };
        let direction = if speed > 1e-3 { flat / speed } else { Vec2::Y };
        let running = moving
            && if settings.run_speed.is_finite() {
                speed
                    > if last.running {
                        settings.run_exit
                    } else {
                        settings.run_speed
                    }
            } else {
                matches!(gait, Gait::Run | Gait::Sprint)
            };
        Self {
            velocity,
            speed,
            moving,
            direction,
            running,
            turn,
        }
    }

    pub fn gait(&self) -> FeedGait {
        FeedGait {
            moving: self.moving,
            running: self.running,
        }
    }
}

/// Reused buffers of the feed system.
#[derive(Default)]
struct FeedScratch {
    queries: Vec<ProbeQuery>,
    hits: Vec<Option<GroundHit>>,
    /// Per-leg probe origins of one mover, scaled to the mover frame.
    origins: Vec<Vec3>,
    /// (animator entity, first query, query count, mover frame, body scale).
    spans: Vec<(Entity, usize, usize, CharacterFrame, Vec3)>,
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
fn feed_animators<G: GroundProbe + Resource>(
    settings: Res<ControllerSettings>,
    ground: Option<Res<G>>,
    mut stats: ResMut<ControllerStats>,
    counters: Option<Res<DiagCounters>>,
    mut scratch: Local<FeedScratch>,
    mut movers: Query<(&mut Mover, Has<Dormant>)>,
    mut animators: Query<&mut Animator>,
) {
    let t0 = Instant::now();
    let FeedScratch {
        queries,
        hits,
        origins,
        spans,
    } = &mut *scratch;
    queries.clear();
    spans.clear();
    let names = &settings.params;
    for (mut m, dormant) in &mut movers {
        let Some(entity) = m.animator else { continue };
        let Ok(mut animator) = animators.get_mut(entity) else {
            continue;
        };
        let mut v = FeedValues::of(&m, &settings, m.gait);
        if dormant {
            v = FeedValues {
                velocity: Vec3::ZERO,
                speed: 0.0,
                moving: false,
                direction: Vec2::Y,
                running: false,
                turn: 0.0,
            };
        }
        m.gait = v.gait();
        // Mover space -> the body's unscaled skeleton space.
        let inv = m.body_scale.recip();
        let a = &mut *animator;
        a.drive.velocity = v.velocity * inv;
        a.drive.acceleration = if dormant {
            Vec3::ZERO
        } else {
            m.loco.frame().dir_to_local(m.loco.motion.acceleration) * inv
        };
        a.drive.turn_rate = v.turn;
        // Composed until the graph's next update consumes it (A2 updates every
        // other frame, AN-1); zero when the mover went dormant (and not stepped since).
        let mut moved = m.loco.moved;
        moved.translation *= inv;
        a.drive.moved = a.drive.moved.then(moved);
        if let Some(n) = &names.moving {
            a.set_bool(n, v.moving);
        }
        if let Some(n) = &names.direction {
            a.set_vec2(n, v.direction);
        }
        if let Some(n) = &names.running {
            a.set_bool(n, v.running);
        }
        if let Some(n) = &names.turn {
            a.set_float(n, v.turn);
        }
        if let Some(n) = &names.speed {
            a.set_float(n, v.speed);
        }
        let legs = a.output.probe_origins.len().min(a.drive.ground.len());
        if legs > 0 && (dormant || !settings.foot_probes) {
            a.drive.ground.fill(None);
        } else if legs > 0 && ground.is_some() {
            let first = queries.len();
            let scale = m.body_scale;
            origins.clear();
            origins.extend(a.output.probe_origins[..legs].iter().map(|&o| o * scale));
            queries.extend(m.loco.frame().foot_queries(
                origins,
                settings.foot_probe_up * scale.y,
                settings.foot_probe_down * scale.y,
            ));
            spans.push((entity, first, legs, m.loco.frame(), scale));
        }
    }
    if let (Some(ground), false) = (ground, queries.is_empty()) {
        hits.clear();
        hits.resize(queries.len(), None);
        ground.probe(queries, hits);
        for &(entity, first, n, ref frame, scale) in spans.iter() {
            let Ok(mut animator) = animators.get_mut(entity) else {
                continue;
            };
            let inv = scale.recip();
            for (slot, hit) in animator
                .drive
                .ground
                .iter_mut()
                .zip(&hits[first..first + n])
            {
                *slot = hit.as_ref().map(|h| {
                    let (point, normal) = frame.ground_sample(h);
                    // Points scale by 1/s; normals by the inverse transpose, s.
                    GroundSample {
                        point: point * inv,
                        normal: (normal * scale).normalize_or(Vec3::Y),
                    }
                });
            }
        }
    }
    stats.feed_us = micros(t0);
    stats.foot_probes = queries.len();
    if let Some(c) = counters {
        c.add_named(
            "controller/feed/ms",
            sf_diag_bevy::sf_diag::unit::MS,
            f64::from(stats.feed_us) / 1000.0,
        );
    }
}

// --- Gizmos -------------------------------------------------------------------------------

/// Trajectory and capsule gizmos (feature `debug`), relative to a world
/// position the host sets to the render origin.
#[cfg(feature = "debug")]
#[derive(Resource, Clone, Debug, Default)]
pub struct ControllerGizmos {
    pub enabled: bool,
    /// World position of the render origin (floating origin), so gizmos land
    /// in render space.
    pub origin: WorldPos,
}

#[cfg(feature = "debug")]
#[allow(clippy::needless_pass_by_value)]
fn draw_movers(
    settings: Res<ControllerGizmos>,
    movers: Query<&Mover>,
    mut gizmos: bevy_gizmos::gizmos::Gizmos,
) {
    use bevy_color::Color;
    if !settings.enabled {
        return;
    }
    let walk = Color::srgb(0.2, 0.9, 0.3);
    let path = Color::srgb(0.95, 0.8, 0.2);
    let air = Color::srgb(0.9, 0.2, 0.2);
    for m in &movers {
        let feet = settings.origin.delta_f32(m.loco.position);
        let color = if m.loco.grounded.is_some() { walk } else { air };
        let iso = bevy_math::Isometry3d::from_translation(feet + Vec3::Y * 0.02);
        gizmos.circle(
            bevy_math::Isometry3d::new(
                iso.translation,
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
            m.params.radius,
            color,
        );
        gizmos.arrow(
            feet + Vec3::Y * 0.05,
            feet + Vec3::Y * 0.05 + m.loco.motion.velocity,
            color,
        );
        let mut prev = feet;
        for p in &m.loco.trajectory {
            let at = feet + p.offset;
            gizmos.line(prev + Vec3::Y * 0.05, at + Vec3::Y * 0.05, path);
            prev = at;
        }
    }
}
