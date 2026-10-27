//! Thin Bevy shell for the action layer (`docs/specs/action.md` "As built (A2)"). The decisions
//! live in `sf_action`, the motion in `sf_controller`; this crate only schedules them.
//!
//! An action-driven character is an entity with an [`ActionCharacter`] (chooser state and this
//! tick's [`Command`]) and an [`ActionBody`] (the gameplay capsule, its interpolation poses and
//! the render-body correction spring). A local player also carries [`PlayerInput`] and, with
//! [`ActionInputPlugin`], a [`PlayerControls`] `bevy_enhanced_input` context.
//!
//! Per frame (the `physics_in_fixed_timestep` pattern of Bevy 0.20):
//! 1. `PreUpdate`: `bevy_enhanced_input` evaluates the bindings.
//! 2. `RunFixedMainLoop`, [`ActionSystems::Gather`] (before the fixed loop): every local
//!    player's action values go into its [`sf_action::InputSampler`] at frame rate.
//! 3. `FixedUpdate` at [`TICK_HZ`], zero or more times: [`ActionSystems::Sample`] (one
//!    [`Command`] per local player and tick), [`ActionSystems::Command`] (AI, scripts and the
//!    net layer write theirs), [`ActionSystems::Step`] (`sf_action::step` per character; on a
//!    Jump or Up press the context probe's `ObstacleAhead` fills `Sense::obstacle`),
//!    [`ActionSystems::Move`] (a Warp move's body is placed on its warped root path; the others
//!    launch, then one batched `KinematicMover::step_batch` against the ground resource `G`).
//! 4. `RunFixedMainLoop`, [`ActionSystems::Interpolate`] (after the fixed loop): the drawn
//!    root is the previous and current tick poses blended by `Time<Fixed>::overstep_fraction`,
//!    in `WorldPos` space, plus the correction spring's offset, written to the `Transform`
//!    relative to [`ActionBody::frame_origin`]. Cameras run after this set.

mod input;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_mantle;

use std::marker::PhantomData;

use bevy_app::{App, FixedUpdate, Plugin, RunFixedMainLoop, RunFixedMainLoopSystems};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec2, Vec3};
use bevy_time::{Fixed, Time, Virtual};
use bevy_transform::components::Transform;
use sf_action::{
    ActionClass, ActionOutput, ActionState, CharacterId, CharacterParams, Command, Focus, Obstacle, ObstacleClass, RuleSet, Sense, SlotSource,
    StepCtx, Tick, WarpOutput,
};
pub use sf_action::{PlayerId, TICK_HZ};
use sf_controller::{
    Affordance, ContextHit, ContextParams, ContextProbes, ContextQuery, ContextRequest, CorrectionParams, GroundProbe, KinematicMover, Locomotion,
    MoveInput, MoverMode, MoverParams, RenderCorrection, RootMotion,
};
use sf_coords::WorldPos;

pub use input::{ActionInputPlugin, PlayerAction, PlayerControls, PlayerInput, spawn_player_actions};

/// The shell's sets, chained in their schedules (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionSystems {
    /// `RunFixedMainLoop`, before the fixed loop: device actions into the samplers.
    Gather,
    /// `FixedUpdate`: local players' commands for this tick.
    Sample,
    /// `FixedUpdate`: other command writers (AI, scripts, remote players).
    Command,
    /// `FixedUpdate`: the chooser.
    Step,
    /// `FixedUpdate`: the batched mover.
    Move,
    /// `RunFixedMainLoop`, after the fixed loop: interpolation and correction to `Transform`.
    Interpolate,
}

/// The fixed tick, the chooser and the mover, with interpolated rendering. `G` is the ground
/// probe resource (the same one `sf_controller_bevy` uses). Sets `Time<Fixed>` to [`TICK_HZ`]
/// (the physics rate too).
pub struct SfActionPlugin<G> {
    _ground: PhantomData<fn() -> G>,
}

impl<G> Default for SfActionPlugin<G> {
    fn default() -> Self {
        Self { _ground: PhantomData }
    }
}

impl<G: GroundProbe + Resource> Plugin for SfActionPlugin<G> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(f64::from(TICK_HZ)))
            .init_resource::<ActionTick>()
            .configure_sets(
                FixedUpdate,
                (ActionSystems::Sample, ActionSystems::Command, ActionSystems::Step, ActionSystems::Move).chain(),
            )
            .configure_sets(RunFixedMainLoop, ActionSystems::Gather.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop))
            .configure_sets(RunFixedMainLoop, ActionSystems::Interpolate.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop))
            .add_systems(FixedUpdate, (input::sample.in_set(ActionSystems::Sample), step_actions::<G>.in_set(ActionSystems::Step)))
            .add_systems(FixedUpdate, move_bodies::<G>.in_set(ActionSystems::Move))
            .add_systems(RunFixedMainLoop, interpolate.in_set(ActionSystems::Interpolate));
    }
}

/// The current gameplay tick: advanced once per fixed step, before sampling.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActionTick(pub Tick);

/// The move rules and slot source every character steps with (the host loads them: base rules
/// plus content packs).
#[derive(Resource)]
pub struct ActionRules {
    pub rules: RuleSet,
    pub slots: Box<dyn SlotSource + Send + Sync>,
}

/// The chooser half of a character.
#[derive(Component, Clone, Debug)]
pub struct ActionCharacter {
    pub id: CharacterId,
    pub state: ActionState,
    pub params: CharacterParams,
    /// This tick's command: sampled for a local player, else written in
    /// [`ActionSystems::Command`].
    pub command: Command,
    /// The interaction query's result (the host's view cone; `sf_interaction` from C1).
    pub focus: Option<Focus>,
    /// The last step's output (presentation reads it).
    pub output: ActionOutput,
}

impl ActionCharacter {
    pub fn new(id: CharacterId, state: ActionState, params: CharacterParams) -> Self {
        Self { id, state, params, command: Command::default(), focus: None, output: ActionOutput::default() }
    }
}

/// A drawn root pose at the end of a tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TickPose {
    pub position: WorldPos,
    pub rotation: Quat,
}

impl TickPose {
    pub fn of(loco: &Locomotion) -> Self {
        Self { position: loco.render_position(), rotation: loco.render_rotation() }
    }

    /// `self` toward `next` by `alpha` in 0..=1.
    #[must_use]
    pub fn lerp(self, next: Self, alpha: f32) -> Self {
        Self {
            position: self.position.offset(self.position.delta_f32(next.position) * alpha),
            rotation: self.rotation.slerp(next.rotation, alpha),
        }
    }
}

/// No-clip (a dev cheat: Bethesda's `ToggleCollision`, Unreal's `ghost`): the body flies
/// (`MoverMode::Fly`, no gravity) and steps against no collision at all. The intent's
/// planar speed climbs or dives along `look` (the host's view direction), so walking
/// forward goes where the camera points. Removing it leaves the body in `Fly`: the host
/// puts it back in `Ground`.
#[derive(Component, Clone, Copy, Debug)]
pub struct NoClip {
    pub look: Vec3,
}

/// The body holds its pose until the host removes this: no gravity, no input motion, no
/// launch (its saved velocity and mode resume afterwards). The host sets it while the
/// collision under the body is still streaming in (a door arrival, a save load) and removes it
/// once the ground there is ready, so the body can't fall through a floor that isn't in yet.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AwaitingGround;

/// A probe that never hits ([`NoClip`] bodies).
struct NoHits;

impl GroundProbe for NoHits {
    fn probe(&self, _: &[sf_controller::ProbeQuery], out: &mut [Option<sf_controller::GroundHit>]) {
        out.fill(None);
    }
}

/// The vertical speed that moves a planar `velocity` along `look` (zero for a flat or
/// vertical look).
fn no_clip_climb(velocity: Vec3, look: Vec3) -> f32 {
    let flat = Vec2::new(look.x, look.z);
    let len = flat.length();
    if len < 1e-3 {
        return 0.0;
    }
    Vec2::new(velocity.x, velocity.z).dot(flat / len) * look.y / len
}

/// The motion half of a character: the gameplay capsule (truth) and how it is drawn.
#[derive(Component, Clone, Debug)]
pub struct ActionBody {
    pub loco: Locomotion,
    pub params: MoverParams,
    /// Root motion for the next mover step (B1's warp and clip owners fill it).
    pub root: RootMotion,
    /// World position of the parent frame the `Transform` is relative to (unrotated,
    /// unscaled): the host keeps it current, as for `sf_controller_bevy::Mover`.
    pub frame_origin: WorldPos,
    pub correction: RenderCorrection,
    pub correction_params: CorrectionParams,
    prev: TickPose,
    curr: TickPose,
    /// A Warp move's start-tick feet: its root poses are relative to them.
    warp_anchor: Option<WorldPos>,
    /// A Warp move's chain progress at the previous and current tick, beside `prev`/`curr`.
    warp_progress: Option<(f32, f32)>,
}

impl ActionBody {
    pub fn new(loco: Locomotion, params: MoverParams, frame_origin: WorldPos) -> Self {
        let pose = TickPose::of(&loco);
        Self {
            loco,
            params,
            root: RootMotion::default(),
            frame_origin,
            correction: RenderCorrection::default(),
            correction_params: CorrectionParams::default(),
            prev: pose,
            curr: pose,
            warp_anchor: None,
            warp_progress: None,
        }
    }

    /// Moves the body without drawing the way there (spawn, load, a scripted teleport).
    pub fn teleport(&mut self, position: WorldPos, yaw: f32) {
        self.restore(Locomotion::new(position, yaw));
    }

    /// Puts back a saved mover state (save load: velocity, stance, airborne state with the
    /// pose), without drawing the way there.
    pub fn restore(&mut self, loco: Locomotion) {
        self.loco = loco;
        self.correction = RenderCorrection::default();
        self.warp_anchor = None;
        self.warp_progress = None;
        self.curr = TickPose::of(&self.loco);
        self.prev = self.curr;
    }

    /// Replaces the gameplay state with an authoritative one (co-op reconciliation): the
    /// difference goes into the correction spring, so the drawn body glides instead of popping.
    pub fn reconcile(&mut self, loco: Locomotion) {
        let delta = self.loco.render_position().delta_f32(loco.render_position());
        let yaw = sf_controller::wrap_angle(loco.motion.yaw - self.loco.motion.yaw);
        self.correction.absorb(delta, yaw, &self.correction_params);
        self.loco = loco;
        let pose = TickPose::of(&self.loco);
        let shift = self.curr.position.delta_f32(pose.position);
        self.prev = TickPose { position: self.prev.position.offset(shift), rotation: pose.rotation };
        self.curr = pose;
    }

    /// The previous and current tick poses the interpolation blends.
    pub fn poses(&self) -> (TickPose, TickPose) {
        (self.prev, self.curr)
    }

    /// A Warp move's chain progress (`WarpOutput::progress`) at the same two ticks as
    /// [`Self::poses`], `None` off a Warp move. A clip played by it and lerped by the same
    /// `overstep_fraction` stays on the drawn body however many ticks ran this frame. On the
    /// move's first tick the previous one is 0 (its clock's start, where the previous pose is).
    pub fn warp_progress(&self) -> Option<(f32, f32)> {
        self.warp_progress
    }
}

/// How far ahead the ledge probe looks, m (the context probe's `ObstacleAhead` range).
const OBSTACLE_RANGE_M: f32 = 1.5;

#[allow(clippy::needless_pass_by_value)]
fn step_actions<G: GroundProbe + Resource>(
    rules: Option<Res<ActionRules>>,
    ground: Option<Res<G>>,
    mut probes: Local<ContextProbes>,
    mut characters: Query<(&mut ActionCharacter, &ActionBody)>,
) {
    let Some(rules) = rules else { return };
    for (mut c, body) in &mut characters {
        let c = &mut *c;
        let ctx = StepCtx { rules: &rules.rules, slots: &*rules.slots, params: &c.params, source: c.id };
        let v = body.loco.motion.velocity;
        // Only a jump or up press can start a traversal: probe then, not every tick.
        let wants = [ActionClass::Jump, ActionClass::Up].into_iter().any(|a| c.command.pressed(a) || c.command.held(a));
        let obstacle = match &ground {
            Some(ground) if wants => sense_obstacle(&mut probes, &**ground, body, c.command.stick),
            _ => None,
        };
        let sense = Sense {
            // A warped traversal holds the body off the ground on purpose: not a fall.
            grounded: body.loco.grounded.is_some() || body.loco.traversing,
            vertical_speed: body.loco.vertical_speed,
            planar_speed: Vec3::new(v.x, 0.0, v.z).length(),
            yaw: body.loco.motion.yaw,
            focus: c.focus,
            obstacle,
        };
        sf_action::step(&ctx, &mut c.state, &c.command, &sense, &mut c.output);
    }
}

/// The context probe's obstacle ahead along the stick (else the facing), as the chooser reads it.
fn sense_obstacle(probes: &mut ContextProbes, ground: &dyn GroundProbe, body: &ActionBody, stick: Vec2) -> Option<Obstacle> {
    let dir = if stick.length_squared() > 1e-4 {
        Vec3::new(stick.x, 0.0, stick.y).normalize()
    } else {
        sf_controller::forward(body.loco.motion.yaw)
    };
    let params = ContextParams::from_mover(&body.params);
    let query = ContextQuery::ObstacleAhead { range: OBSTACLE_RANGE_M };
    let mut out = [None];
    probes.run(&[ContextRequest { feet: body.loco.position, dir, params: &params, query }], ground, &mut out);
    out[0].and_then(|hit| obstacle(body.loco.position, &hit))
}

/// A context hit as an [`Obstacle`] (positions from `feet`); `None` for kinds no traversal class
/// covers.
fn obstacle(feet: WorldPos, hit: &ContextHit) -> Option<Obstacle> {
    let class = match hit.kind {
        Affordance::Step => ObstacleClass::Step,
        Affordance::Vault => ObstacleClass::Vault,
        Affordance::Mantle => ObstacleClass::Mantle,
        Affordance::Climb => ObstacleClass::Climb,
        _ => return None,
    };
    Some(Obstacle { class, lip: feet.delta_f32(hit.point), normal: hit.normal, height: hit.height, depth: hit.depth, clearance: hit.clearance })
}

/// Puts a Warp move's body where the move says, outside the mover: no gravity, steering or
/// collision (the probe checked the end has room; UE's and ALS's mantles move the capsule the
/// same way). The body stays marked `traversing`, so the first mover step after the move
/// re-grounds it with the step-down probe.
fn place_warped(body: &mut ActionBody, warp: &WarpOutput, started: bool, dt: f32) {
    let anchor = match body.warp_anchor {
        Some(a) if !started => a,
        _ => *body.warp_anchor.insert(body.loco.position),
    };
    // The move's clock starts at 0 on its first tick, where the previous tick pose is the anchor
    // (root path at 0): the drawn body and the clip leave it together.
    body.warp_progress = Some(match body.warp_progress {
        _ if started => (0.0, warp.progress),
        Some((_, curr)) => (curr, warp.progress),
        None => (warp.progress, warp.progress),
    });
    let feet = anchor.offset(warp.root.position);
    let loco = &mut body.loco;
    let moved = loco.position.delta_f32(feet);
    let v = if dt > 0.0 { moved / dt } else { Vec3::ZERO };
    loco.position = feet;
    // Planar momentum carries out of the move; the climb does not.
    loco.motion.velocity = Vec3::new(v.x, 0.0, v.z);
    loco.motion.acceleration = Vec3::ZERO;
    loco.motion.yaw = warp.root.yaw;
    loco.motion.facing_goal = warp.root.yaw;
    loco.motion.angular_velocity = 0.0;
    loco.vertical_speed = 0.0;
    loco.grounded = None;
    loco.traversing = true;
    loco.height_offset = 0.0;
    loco.height_offset_velocity = 0.0;
}

#[allow(clippy::needless_pass_by_value)]
fn move_bodies<G: GroundProbe + Resource>(
    time: Res<Time>,
    ground: Option<Res<G>>,
    mut mover: Local<KinematicMover>,
    mut characters: Query<(&ActionCharacter, &mut ActionBody, Option<&NoClip>), Without<AwaitingGround>>,
    mut waiting: Query<&mut ActionBody, With<AwaitingGround>>,
) {
    // Held bodies keep their pose; `prev = curr` so the drawn body doesn't lerp either.
    for mut body in &mut waiting {
        body.prev = body.curr;
        body.warp_progress = body.warp_progress.map(|(_, curr)| (curr, curr));
    }
    let Some(ground) = ground else { return };
    // In `FixedUpdate` the generic clock is the fixed one.
    let dt = time.delta_secs();
    let mut bodies: Vec<_> = characters.iter_mut().collect();
    let mut items: Vec<MoveInput<'_>> = Vec::with_capacity(bodies.len());
    let mut ghosts: Vec<MoveInput<'_>> = Vec::new();
    for (c, body, no_clip) in &mut bodies {
        let body = &mut **body;
        body.prev = body.curr;
        if let Some(warp) = &c.output.warp {
            place_warped(body, warp, c.output.started, dt);
            continue;
        }
        body.warp_anchor = None;
        body.warp_progress = None;
        if let Some(v) = c.output.launch {
            body.loco.launch(v);
        }
        let mut intent = c.output.intent;
        if let Some(no_clip) = no_clip {
            body.loco.mode = MoverMode::Fly { hover: None };
            intent.desired_velocity.y = no_clip_climb(intent.desired_velocity, no_clip.look);
            ghosts.push(MoveInput { loco: &mut body.loco, intent, params: &body.params, root: body.root });
        } else {
            items.push(MoveInput { loco: &mut body.loco, intent, params: &body.params, root: body.root });
        }
    }
    mover.step_batch(&mut items, &*ground, dt);
    if !ghosts.is_empty() {
        mover.step_batch(&mut ghosts, &NoHits, dt);
    }
    drop(items);
    drop(ghosts);
    for (_, body, _) in &mut bodies {
        body.curr = TickPose::of(&body.loco);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn interpolate(fixed: Res<Time<Fixed>>, frame: Res<Time<Virtual>>, mut bodies: Query<(&mut ActionBody, &mut Transform)>) {
    let alpha = fixed.overstep_fraction().clamp(0.0, 1.0);
    let dt = frame.delta_secs();
    for (mut body, mut transform) in &mut bodies {
        let body = &mut *body;
        let pose = body.prev.lerp(body.curr, alpha);
        let v = body.loco.motion.velocity;
        body.correction.update(&body.correction_params, Vec3::new(v.x, 0.0, v.z).length(), dt);
        transform.translation = body.frame_origin.delta_f32(pose.position) + body.correction.offset;
        transform.rotation = body.correction.rotation() * pose.rotation;
    }
}
