//! Thin Bevy shell for the player camera (`docs/specs/camera.md`, slice D1). The rig math lives
//! in `sf_camera`; this crate schedules it and moves the Bevy camera.
//!
//! A player camera is a camera entity with a [`PlayerCamera`] naming the character it follows
//! (an `sf_action_bevy::ActionBody`). Per frame:
//! 1. `PreUpdate`: `bevy_enhanced_input` evaluates the [`CameraAction`]s on the player's
//!    `PlayerControls` context.
//! 2. `RunFixedMainLoop`, [`CameraSystems::Look`] (before the fixed loop, after
//!    `ActionSystems::Gather`): look input, zoom, view toggle and shoulder swap; the player's
//!    look (free, first-person limited, or eased onto a [`LockOn`] target) goes into the
//!    sampler as the next commands' `view`, and the move stick is resolved against the look's
//!    yaw (camera-relative movement). Shake, framing and push-in never reach the look.
//! 3. The fixed ticks and `ActionSystems::Interpolate` draw the body.
//! 4. `PostUpdate`, [`CameraSystems::Rig`] (before transform propagation and `big_space`'s
//!    recentering): the orbit rig on the drawn (interpolated) body, the arm's sphere cast
//!    through the ground probe resource `G` (the mover's statics: same collision as gameplay),
//!    trauma shake from [`CameraShake`] messages, and the camera's `Transform`, plus its
//!    `CellCoord` when it sits on a `big_space` grid.

mod input;
#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use bevy_app::{App, Plugin, PostUpdate, RunFixedMainLoop, RunFixedMainLoopSystems};
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelationshipTarget;
use bevy_ecs::system::SystemParam;
use bevy_enhanced_input::prelude::{ActionEvents, ActionValue, Actions};
use bevy_math::{IVec3, Quat, Vec2, Vec3};
use bevy_time::{Time, Virtual};
use bevy_transform::TransformSystems;
use bevy_transform::components::Transform;
use big_space::prelude::{BigSpaceSystems, CellCoord};
use sf_action::{Mode, ViewDir};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems, PlayerAction, PlayerControls, PlayerInput};
use sf_camera::{
    ArmCollision, CollisionParams, ComfortSettings, FirstPersonLook, LockParams, Look, LookLimits, OrbitParams, OrbitRig,
    RecentreParams, RigMode, ShakeParams, Shoulder, TraumaShake, ViewChoice, locked_look, recentre_behind,
};
use sf_controller::{GroundHit, GroundProbe, ProbeQuery};
use sf_coords::WorldPos;

pub use input::{CameraAction, CameraLook, CameraSwapShoulder, CameraToggleView, CameraZoom, spawn_camera_actions};

/// The shell's sets (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraSystems {
    /// `RunFixedMainLoop`, before the fixed loop: the player's look into the sampler.
    Look,
    /// `PostUpdate`: the rig, collision and shake to the camera's transform.
    Rig,
}

/// The player camera. `G` is the ground probe resource the arm is cast against (the same one
/// `sf_action_bevy::SfActionPlugin<G>` moves characters on). Needs `SfActionPlugin`; with
/// `EnhancedInputPlugin` and `ActionInputPlugin`, look input comes from [`CameraAction`]s.
pub struct SfCameraPlugin<G> {
    _ground: PhantomData<fn() -> G>,
}

impl<G> Default for SfCameraPlugin<G> {
    fn default() -> Self {
        Self { _ground: PhantomData }
    }
}

impl<G: GroundProbe + Resource> Plugin for SfCameraPlugin<G> {
    fn build(&self, app: &mut App) {
        app.add_message::<CameraShake>()
            .configure_sets(
                RunFixedMainLoop,
                CameraSystems::Look.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop).after(ActionSystems::Gather),
            )
            .configure_sets(
                PostUpdate,
                CameraSystems::Rig.before(TransformSystems::Propagate).before(BigSpaceSystems::RecenterLargeTransforms),
            )
            .add_systems(RunFixedMainLoop, update_look.in_set(CameraSystems::Look))
            .add_systems(PostUpdate, update_rig::<G>.in_set(CameraSystems::Rig));
        #[cfg(feature = "projection")]
        app.add_systems(PostUpdate, apply_fov.in_set(CameraSystems::Rig).after(update_rig::<G>));
    }
}

/// Camera shake: every player camera takes `trauma` (0..1, saturating; the comfort settings
/// scale or mute it). Rotation only.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct CameraShake {
    pub trauma: f32,
}

/// The locked target of a character (on the follower, e.g. the player). Its owner writes it:
/// the action layer once it keeps the lock (E2), or the host. The target needs [`Lockable`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LockOn(pub Entity);

/// A character the camera can frame when locked on.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Lockable {
    /// Aim point above the entity's root (chest), metres.
    pub aim_height: f32,
}

impl Default for Lockable {
    fn default() -> Self {
        Self { aim_height: 1.3 }
    }
}

/// First-person eye (until D2's baked camera track replaces it).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirstPersonParams {
    /// Eye above the root, and ahead of it along the look's yaw, metres.
    pub eye_height: f32,
    pub eye_forward: f32,
    /// Free-look pitch limits, radians.
    pub pitch_min: f32,
    pub pitch_max: f32,
}

impl Default for FirstPersonParams {
    fn default() -> Self {
        Self { eye_height: 1.62, eye_forward: 0.12, pitch_min: -85f32.to_radians(), pitch_max: 85f32.to_radians() }
    }
}

/// Third-person zoom: a factor on the mode's arm length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomParams {
    pub min: f32,
    pub max: f32,
    /// Factor change per zoom step.
    pub step: f32,
}

impl Default for ZoomParams {
    fn default() -> Self {
        Self { min: 0.5, max: 1.5, step: 0.1 }
    }
}

/// All camera tuning.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraParams {
    pub orbit: OrbitParams,
    pub collision: CollisionParams,
    pub lock: LockParams,
    pub shake: ShakeParams,
    pub recentre: RecentreParams,
    pub first_person: FirstPersonParams,
    pub zoom: ZoomParams,
}

/// The camera's pose this frame (read it for the body's dither fade, a centre dot, audio).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraPose {
    pub eye: WorldPos,
    /// Final rotation, shake included (camera -Z forward).
    pub rotation: Quat,
    /// The framing look before shake.
    pub look: Look,
    pub view: ViewChoice,
    pub mode: RigMode,
    /// Pivot-to-eye length in use (0 in first person), and whether the cast was blocked.
    pub arm_length: f32,
    pub blocked: bool,
    /// Followed body's fade, 0 opaque to 1 hidden (hidden in first person until D2's body).
    pub fade: f32,
    /// The framed lock target, if any.
    pub locked: Option<Entity>,
}

/// A player camera, on the camera entity. Presentation state only: nothing here is saved or
/// replicated, and the gameplay sees only the look, through the sampler.
#[derive(Component, Clone, Debug)]
pub struct PlayerCamera {
    /// The character followed: an `ActionBody` (with `PlayerInput`, its commands get the look).
    pub follow: Entity,
    pub params: CameraParams,
    pub settings: ComfortSettings,
    /// World position the `Transform` is relative to when the camera has no `CellCoord` (the
    /// host keeps it current, as `ActionBody::frame_origin`). With a `CellCoord` the camera is
    /// placed on its `big_space` grid exactly and this is unused.
    pub frame_origin: WorldPos,
    /// The current move's first-person look limits (`None`: free) and its entry yaw, and the
    /// `ignore_look` window: the action shell writes them from the move rule.
    pub look_limits: Option<LookLimits>,
    pub limits_entry_yaw: f32,
    pub ignore_look: bool,
    look: Look,
    idle_look: f32,
    zoom: f32,
    locked: Option<Entity>,
    orbit: OrbitRig,
    arm: ArmCollision,
    first_person: FirstPersonLook,
    shake: TraumaShake,
    pose: Option<CameraPose>,
}

impl PlayerCamera {
    pub fn new(follow: Entity) -> Self {
        Self {
            follow,
            params: CameraParams::default(),
            settings: ComfortSettings::default(),
            frame_origin: WorldPos::ORIGIN,
            look_limits: None,
            limits_entry_yaw: 0.0,
            ignore_look: false,
            look: Look::default(),
            idle_look: 0.0,
            zoom: 1.0,
            locked: None,
            orbit: OrbitRig::new(Shoulder::Right),
            arm: ArmCollision::new(),
            first_person: FirstPersonLook::new(),
            shake: TraumaShake::new(0x5f3c_a2b1),
            pose: None,
        }
    }

    /// The player's look (aim): what the commands carry.
    pub fn look(&self) -> Look {
        self.look
    }

    /// Sets the look (spawn, load, a scripted turn).
    pub fn set_look(&mut self, look: Look) {
        self.look = look;
    }

    pub fn view(&self) -> ViewChoice {
        self.settings.view
    }

    /// Switches first/third person: an instant cut, the rig starts again on the body. Changes no
    /// gameplay state.
    pub fn set_view(&mut self, view: ViewChoice) {
        if view != self.settings.view {
            self.settings.view = view;
            self.orbit.reset();
            self.arm.reset();
        }
    }

    pub fn toggle_view(&mut self) {
        self.set_view(match self.settings.view {
            ViewChoice::FirstPerson => ViewChoice::ThirdPerson,
            ViewChoice::ThirdPerson => ViewChoice::FirstPerson,
        });
    }

    pub fn swap_shoulder(&mut self) {
        self.orbit.swap_shoulder();
    }

    pub fn shoulder(&self) -> Shoulder {
        self.orbit.shoulder()
    }

    /// The zoom factor on the arm length.
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Zooms by `steps` (positive in), within the zoom limits; the arm eases to it.
    pub fn zoom_by(&mut self, steps: f32) {
        let z = self.params.zoom;
        self.zoom = (self.zoom - steps * z.step).clamp(z.min, z.max);
    }

    /// Adds shake trauma to this camera only.
    pub fn add_trauma(&mut self, trauma: f32) {
        self.shake.add(trauma);
    }

    /// The last pose written (`None` before the first frame).
    pub fn pose(&self) -> Option<&CameraPose> {
        self.pose.as_ref()
    }
}

/// World positions of followed bodies and lock targets: an `ActionBody` is its frame origin
/// plus its (interpolated) `Transform`; a `CellCoord` entity is its grid cell plus its
/// `Transform`; anything else is a `Transform` from the world origin.
type Spatial = (Option<&'static ActionBody>, Option<&'static CellCoord>, &'static Transform);

#[derive(SystemParam)]
struct Positions<'w, 's> {
    spatial: Query<'w, 's, Spatial, Without<PlayerCamera>>,
}

impl Positions<'_, '_> {
    fn get(&self, entity: Entity) -> Option<WorldPos> {
        let (body, cell, transform) = self.spatial.get(entity).ok()?;
        Some(match (body, cell) {
            (Some(b), _) => b.frame_origin.offset(transform.translation),
            (None, Some(c)) => WorldPos::new(IVec3::new(c.x, c.y, c.z), transform.translation),
            (None, None) => WorldPos::ORIGIN.offset(transform.translation),
        })
    }
}

/// The locked target's aim point, if `follower` is locked on a lockable entity and the view
/// allows it (no hard lock in first person: soft aim is gameplay).
fn lock_target(
    view: ViewChoice,
    lock: Option<&LockOn>,
    lockables: &Query<&Lockable>,
    positions: &Positions,
) -> Option<(Entity, WorldPos)> {
    let target = lock?.0;
    if view != ViewChoice::ThirdPerson {
        return None;
    }
    let aim = lockables.get(target).ok()?;
    Some((target, positions.get(target)?.offset(Vec3::Y * aim.aim_height)))
}

/// Screen-relative stick (x right, y forward) to world XZ for a look yaw.
fn stick_to_world(stick: Vec2, yaw: f32) -> Vec2 {
    let (s, c) = yaw.sin_cos();
    Vec2::new(-c * stick.x + s * stick.y, s * stick.x + c * stick.y)
}

type Follower<'a> = (
    &'a ActionBody,
    Option<&'a mut PlayerInput>,
    Option<&'a Actions<PlayerControls>>,
);

#[allow(clippy::needless_pass_by_value)]
fn update_look(
    time: Res<Time<Virtual>>,
    mut cameras: Query<&mut PlayerCamera>,
    mut followers: Query<Follower<'_>>,
    camera_actions: Query<(&CameraAction, &ActionValue, &ActionEvents)>,
    player_actions: Query<(&PlayerAction, &ActionValue)>,
) {
    let dt = time.delta_secs();
    for mut cam in &mut cameras {
        let cam = &mut *cam;
        let Ok((body, mut player, actions)) = followers.get_mut(cam.follow) else { continue };

        let mut input = input::CameraInput::default();
        let mut stick = None;
        for entity in actions.into_iter().flat_map(RelationshipTarget::iter) {
            if let Ok((tag, value, events)) = camera_actions.get(entity) {
                input.read(*tag, *value, *events);
            } else if let Ok((PlayerAction::Stick, value)) = player_actions.get(entity) {
                stick = Some(value.as_axis2d());
            }
        }
        if input.toggle_view {
            cam.toggle_view();
        }
        if input.swap_shoulder {
            cam.swap_shoulder();
        }
        if input.zoom != 0.0 && cam.settings.view == ViewChoice::ThirdPerson {
            cam.zoom_by(input.zoom);
        }

        let delta = Look { yaw: -input.look.x, pitch: input.look.y };
        cam.idle_look = if delta == Look::default() { cam.idle_look + dt } else { 0.0 };
        let p = cam.params;
        cam.look = match (cam.settings.view, cam.locked) {
            (ViewChoice::FirstPerson, _) => {
                cam.first_person.set_limits(cam.look_limits, cam.limits_entry_yaw);
                let fp = p.first_person;
                cam.first_person.update(cam.look, delta, cam.ignore_look, (fp.pitch_min, fp.pitch_max), dt)
            }
            // Locked: the rig eases the look onto the target on the drawn bodies (below); look
            // input waits for the release (switching targets is the lock owner's).
            (ViewChoice::ThirdPerson, Some(_)) => {
                cam.first_person.set_limits(None, 0.0);
                cam.look
            }
            (ViewChoice::ThirdPerson, None) => {
                cam.first_person.set_limits(None, 0.0);
                let look = cam.look.turned(delta, p.orbit.pitch_min, p.orbit.pitch_max);
                if cam.settings.auto_camera {
                    recentre_behind(look, body.loco.motion.velocity, cam.idle_look, &p.recentre, dt)
                } else {
                    look
                }
            }
        };

        if let Some(player) = player.as_deref_mut() {
            player.sampler.set_view(ViewDir { yaw: cam.look.yaw, pitch: cam.look.pitch });
            if let Some(stick) = stick {
                player.sampler.set_stick(stick_to_world(stick, cam.look.yaw));
            }
        }
    }
}

/// No collision (the app has no ground probe resource yet).
struct NoProbe;

impl GroundProbe for NoProbe {
    fn probe(&self, _queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        out.fill(None);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn update_rig<G: GroundProbe + Resource>(
    time: Res<Time<Virtual>>,
    ground: Option<Res<G>>,
    mut shakes: MessageReader<CameraShake>,
    mut cameras: Query<(&mut PlayerCamera, &mut Transform, Option<&mut CellCoord>)>,
    characters: Query<&ActionCharacter>,
    bodies: Query<(&ActionBody, Option<&LockOn>)>,
    lockables: Query<&Lockable>,
    positions: Positions,
) {
    let dt = time.delta_secs();
    let trauma: f32 = shakes.read().map(|s| s.trauma.max(0.0)).sum();
    let probe: &dyn GroundProbe = match &ground {
        Some(g) => &**g,
        None => &NoProbe,
    };
    for (mut cam, mut transform, cell) in &mut cameras {
        let cam = &mut *cam;
        let (Ok((body, lock_on)), Some(root)) = (bodies.get(cam.follow), positions.get(cam.follow)) else { continue };
        let p = cam.params;
        // Lock framing on this frame's drawn bodies; the look it leaves is the aim the next
        // ticks' commands carry.
        let target = lock_target(cam.settings.view, lock_on, &lockables, &positions);
        cam.locked = target.map(|(e, _)| e);
        if let Some((_, aim)) = target {
            let pivot = root.offset(Vec3::Y * p.orbit.pivot_height);
            cam.look = cam.look.eased(locked_look(pivot, aim, &p.lock), p.lock.framing_halflife, dt);
        }
        let loco = &body.loco;
        let airborne = if loco.grounded.is_none() { loco.vertical_speed } else { 0.0 };
        let velocity = loco.motion.velocity + Vec3::Y * airborne;
        let mode = match (cam.locked, characters.get(cam.follow).map(|c| c.state.mode.mode)) {
            (Some(_), _) => RigMode::Locked,
            (None, Ok(Mode::Sneak)) => RigMode::Sneak,
            (None, Ok(Mode::FreeRun)) => RigMode::FreeRun,
            (None, _) => RigMode::Normal,
        };

        let (eye, look, arm_length, blocked, fade) = match cam.settings.view {
            ViewChoice::ThirdPerson => {
                let mut orbit = p.orbit;
                for d in [&mut orbit.distance.normal, &mut orbit.distance.sneak, &mut orbit.distance.free_run, &mut orbit.distance.locked]
                {
                    *d *= cam.zoom;
                }
                let pose = cam.orbit.update(&orbit, root, velocity, cam.look, mode, dt);
                let arm = cam.arm.update(&probe, pose.pivot, pose.arm, &p.collision, dt);
                (arm.eye, pose.look, arm.length, arm.blocked, arm.fade)
            }
            ViewChoice::FirstPerson => {
                let fp = p.first_person;
                let ahead = Vec3::new(cam.look.yaw.sin(), 0.0, cam.look.yaw.cos()) * fp.eye_forward;
                (root.offset(Vec3::Y * fp.eye_height + ahead), cam.look, 0.0, false, 1.0)
            }
        };
        if trauma > 0.0 {
            cam.shake.add(trauma);
        }
        let rotation = look.rotation() * cam.shake.update(&p.shake, &cam.settings, dt);

        match cell {
            Some(mut cell) => {
                let c = IVec3::new(eye.cell.x, eye.cell.y, eye.cell.z);
                if IVec3::new(cell.x, cell.y, cell.z) != c {
                    *cell = CellCoord::new(c.x, c.y, c.z);
                }
                transform.translation = eye.local;
            }
            None => transform.translation = cam.frame_origin.delta_f32(eye),
        }
        transform.rotation = rotation;
        cam.pose = Some(CameraPose {
            eye,
            rotation,
            look,
            view: cam.settings.view,
            mode,
            arm_length,
            blocked,
            fade,
            locked: cam.locked,
        });
    }
}

/// Comfort FOV onto a perspective projection.
#[cfg(feature = "projection")]
fn apply_fov(mut cameras: Query<(&PlayerCamera, &mut bevy_camera::Projection)>) {
    for (cam, mut projection) in &mut cameras {
        if let bevy_camera::Projection::Perspective(p) = &mut *projection {
            let fov = cam.settings.fov_world.to_radians();
            if (p.fov - fov).abs() > 1e-6 {
                p.fov = fov;
            }
        }
    }
}
