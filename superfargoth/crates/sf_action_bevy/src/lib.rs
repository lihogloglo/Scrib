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
//!    net layer write theirs), [`ActionSystems::Step`] (`sf_action::step` per character),
//!    [`ActionSystems::Move`] (launch, then one batched `KinematicMover::step_batch` for every
//!    character against the ground resource `G`).
//! 4. `RunFixedMainLoop`, [`ActionSystems::Interpolate`] (after the fixed loop): the drawn
//!    root is the previous and current tick poses blended by `Time<Fixed>::overstep_fraction`,
//!    in `WorldPos` space, plus the correction spring's offset, written to the `Transform`
//!    relative to [`ActionBody::frame_origin`]. Cameras run after this set.

mod input;
#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use bevy_app::{App, FixedUpdate, Plugin, RunFixedMainLoop, RunFixedMainLoopSystems};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec2, Vec3};
use bevy_time::{Fixed, Time, Virtual};
use bevy_transform::components::Transform;
use sf_action::{ActionOutput, ActionState, CharacterId, CharacterParams, Command, Focus, RuleSet, Sense, SlotSource, StepCtx, Tick};
pub use sf_action::{PlayerId, TICK_HZ};
use sf_controller::{CorrectionParams, GroundProbe, KinematicMover, Locomotion, MoveInput, MoverMode, MoverParams, RenderCorrection, RootMotion};
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
            .add_systems(FixedUpdate, (input::sample.in_set(ActionSystems::Sample), step_actions.in_set(ActionSystems::Step)))
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
}

fn step_actions(rules: Option<Res<ActionRules>>, mut characters: Query<(&mut ActionCharacter, &ActionBody)>) {
    let Some(rules) = rules else { return };
    for (mut c, body) in &mut characters {
        let c = &mut *c;
        let ctx = StepCtx { rules: &rules.rules, slots: &*rules.slots, params: &c.params, source: c.id };
        let v = body.loco.motion.velocity;
        let sense = Sense {
            grounded: body.loco.grounded.is_some(),
            vertical_speed: body.loco.vertical_speed,
            planar_speed: Vec3::new(v.x, 0.0, v.z).length(),
            yaw: body.loco.motion.yaw,
            focus: c.focus,
            // B2a shell step (next): the context probe (`sf_controller::ContextProbes`, ObstacleAhead along
            // the stick) fills this; until then no traversal rule can fire.
            obstacle: None,
        };
        sf_action::step(&ctx, &mut c.state, &c.command, &sense, &mut c.output);
    }
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
