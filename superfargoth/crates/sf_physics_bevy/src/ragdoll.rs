//! Ragdolls on characters (feature `ragdoll`; physics.md "Ragdolls and the
//! animation blend", P7 limp, P8 powered).
//!
//! The host puts [`Ragdoll`] on a character's `Animator` entity (its model
//! frame) after starting the death or knockdown clip. While `delay` runs
//! the clip plays and the last two animated model poses are kept; then the
//! skeleton's [`RagdollDef`] (from [`RagdollSource`]) is spawned at the
//! animated pose with its velocities, and every frame the bodies' pose goes
//! to the animator as a [`PhysicsPose`] (every joint, including those above
//! the root body, at weight 1) whose weight ramps to 1 over
//! [`RagdollConfig::fade`]. In every mode the entity's frame follows the
//! ragdoll's implied frame (heading and position), so culling and cell
//! moves follow the body, and its own scale is made uniform (bodies scale
//! uniformly) until the get-up restores it. Root motion is off while a
//! physics pose shows (`sf_anim_bevy`).
//!
//! Modes ([`RagdollMode`]), driven every fixed step toward the handoff pose:
//! - `Death { crumple }`: with `crumple` the joints hold the handoff pose at
//!   full strength and fade to limp along the reaction set's death curves (the
//!   knees give first by gravity, a powered crumple rather than a sack).
//! - `Knockdown`: the hit's impulse lands on the nearest body, the joints
//!   brace and fade along the reaction set's knockdown curves; once the
//!   controller settles (pelvis slower than the set's rest speed for its rest
//!   time) and enters its get-up the character gets up: its frame moves to [`get_up_root`] for the get-up
//!   clip's first root-body pose, the graph inertializes from the ragdoll's
//!   pose, the host's `get_up_flag` graph bool is cleared, the bodies are
//!   removed and the component with them ([`RagdollGotUp`] tells the host).
//!
//! At most [`RagdollConfig::max_powered`] ragdolls are driven at once; the
//! rest go limp at the handoff. Limp bodies are `RAGDOLL` (holders can drag
//! them by a limb), powered ones `POWERED`. Removing the component or
//! despawning the entity removes the bodies.
//!
//! Hosts sample the get-up clip's first frame with [`clip_root_body`].
//!
//! Not yet: sleeping-pose persistence across deactivation, placed corpses.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Quat, Vec3};
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use sf_anim::Trs;
use sf_anim_bevy::{Animator, PhysicsPose};
use sf_coords::{WorldPos, WorldTransform};
use sf_ragdoll::{
    Drive, DriveParams, DriveTarget, HitImpulse, RagdollDef, RagdollInstance, RagdollSettings, ReactionClass, ReactionController, ReactionRequest, ReactionSet, ReactionState, Region, Sensed, SpawnPose,
    StrengthProfile, get_up_root, yaw_of,
};
use sf_world::WorldspaceId;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{WorldspaceRoot, from_grid};

use crate::{Physics, PhysicsSystems};

#[cfg(test)]
#[path = "ragdoll_tests.rs"]
mod tests;

/// The host's ragdoll definitions, by the skeleton subject it names in
/// [`Ragdoll::skeleton`] (usually the `ragdoll.cook` product).
#[derive(Resource, Clone)]
pub struct RagdollSource(pub RagdollLoader);

/// Loads a skeleton's ragdoll by subject.
pub type RagdollLoader = Arc<dyn Fn(&str) -> Option<Arc<RagdollDef>> + Send + Sync>;

#[derive(Resource, Clone, Debug)]
pub struct RagdollConfig {
    pub settings: RagdollSettings,
    pub drive: DriveParams,
    /// Animation → physics blend time after the handoff, s.
    pub fade: f32,
    /// The reaction curves and thresholds: the death crumple, the knockdown
    /// brace and its fade, the rest speed and time before the get-up
    /// (`sf_ragdoll::react`, RON `ReactionSet`).
    pub reactions: Arc<ReactionSet>,
    /// Get-up hand-back inertialization, s.
    pub get_up_blend: f32,
    /// Powered ragdolls at once (the budget; physics.md "Budgets").
    pub max_powered: usize,
}

impl Default for RagdollConfig {
    fn default() -> Self {
        Self {
            settings: RagdollSettings::default(),
            drive: DriveParams::default(),
            fade: 0.1,
            reactions: Arc::new(ReactionSet::default()),
            get_up_blend: 0.3,
            max_powered: 8,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RagdollMode {
    Death {
        crumple: bool,
    },
    Knockdown {
        /// The hit: world point, impulse (N·s, world).
        point: WorldPos,
        impulse: Vec3,
        /// The get-up clip's first-frame root body in model space at scale 1
        /// (rotation, position); `None`: [`LYING_REST_GET_UP`].
        get_up: Option<(Quat, Vec3)>,
        /// The graph bool cleared at the get-up (the host's knocked-out flag).
        get_up_flag: String,
    },
}

/// **`LYING_REST_GET_UP` scaffold**: without the get-up clip's first-frame
/// root body, the rest pose's root body turned onto its back (−90° about X)
/// at the pelvis height. Removal: hosts pass `RagdollMode::Knockdown::get_up`
/// sampled from their get-up clip.
pub const LYING_REST_GET_UP: f32 = -std::f32::consts::FRAC_PI_2;

/// Hand the character to physics: on its `Animator` entity.
#[derive(Component, Clone, Debug)]
pub struct Ragdoll {
    pub skeleton: String,
    /// Seconds of death or knockdown clip before the handoff.
    pub delay: f32,
    pub mode: RagdollMode,
}

/// A knocked-down character got up (the component and bodies are gone).
#[derive(Message, Clone, Copy, Debug)]
pub struct RagdollGotUp {
    pub entity: Entity,
}

/// A ragdoll request that couldn't be handed to physics (no ragdoll for the
/// skeleton, no frame or world): the [`Ragdoll`] component is removed and the
/// character stays animated; the host undoes what it set up for the ragdoll
/// (a knockdown's flag, a parked mover). Every knockdown request ends in
/// [`RagdollGotUp`] or this, unless the host removes it first (AN-4).
#[derive(Message, Clone, Copy, Debug)]
pub struct RagdollFailed {
    pub entity: Entity,
}

/// Live ragdolls and requests, by their `Animator` entity.
#[derive(Resource, Default)]
pub struct Ragdolls {
    pending: HashMap<Entity, Pending>,
    live: HashMap<Entity, Live>,
}

impl Ragdolls {
    /// Live ragdolls: (entity, worldspace, instance).
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &WorldspaceId, &RagdollInstance)> {
        self.live.iter().map(|(e, l)| (*e, &l.worldspace, &l.instance))
    }

    pub fn len(&self) -> usize {
        self.live.len()
    }

    pub fn is_empty(&self) -> bool {
        self.live.is_empty()
    }

    /// Ragdolls driven this step.
    pub fn powered(&self) -> usize {
        self.live.values().filter(|l| l.instance.powered).count()
    }
}

struct Pending {
    left: f32,
    /// The last two animated model poses, with the frame time between them.
    poses: [Option<Vec<Affine3A>>; 2],
    dt: f32,
}

struct Live {
    worldspace: WorldspaceId,
    def: Arc<RagdollDef>,
    instance: RagdollInstance,
    mode: RagdollMode,
    frozen: Vec<Trs>,
    /// The handoff model pose: the drive target and the implied-frame reference.
    target: Vec<Affine3A>,
    /// The model frame at the handoff (the drive's root).
    root: WorldTransform,
    scale: f32,
    weight: f32,
    /// Per joint: the owning body's region weight (1 = physics).
    weights: Vec<f32>,
    /// The reaction state machine: strength timeline, rest detection, get-up.
    controller: ReactionController,
    /// Per ragdoll region, the controller region its curves address.
    regions: Vec<Region>,
    /// Within the powered budget and not yet faded to limp.
    driving: bool,
    /// The entity's own scale before the handoff made it uniform, restored
    /// at the get-up (bodies are scaled uniformly; see [`handoff`]).
    own_scale: Vec3,
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct RagdollStats {
    pub live: usize,
    pub awake: usize,
    pub powered: usize,
    /// Largest joint separation this frame, m.
    pub max_stretch: f32,
    pub spawned: usize,
    pub failed: usize,
    pub got_up: usize,
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<RagdollConfig>()
        .init_resource::<Ragdolls>()
        .init_resource::<RagdollStats>()
        .add_message::<RagdollGotUp>()
        .add_message::<RagdollFailed>()
        .add_systems(FixedUpdate, drive.before(PhysicsSystems::Step))
        .add_systems(Update, (handoff, read_back, get_up).chain().after(PhysicsSystems::WriteBack));
}

type FrameQuery<'w, 's> = Query<'w, 's, (Option<&'static CellCoord>, &'static Transform, &'static ChildOf)>;

/// The entity's frame in its worldspace: composed `Transform`s up to the
/// nearest high-precision (`CellCoord`) ancestor. Scale: its Y (uniform).
fn world_frame(entity: Entity, spatial: &FrameQuery<'_, '_>, roots: &Query<&WorldspaceRoot>) -> Option<(WorldspaceId, WorldTransform, f32)> {
    let mut local = Transform::IDENTITY;
    let mut current = entity;
    for _ in 0..64 {
        let (cell, transform, parent) = spatial.get(current).ok()?;
        let composed = transform.mul_transform(local);
        if let Some(cell) = cell {
            let root = roots.get(parent.parent()).ok()?;
            let frame = WorldTransform { pos: from_grid(cell, &composed), rotation: composed.rotation, scale: Vec3::ONE };
            return Some((root.id.clone(), frame, composed.scale.y));
        }
        local = composed;
        current = parent.parent();
    }
    None
}

/// Moves an entity whose world frame is `current` (scale `scale`, its local
/// `transform`) to the world frame `to` by editing its local transform.
fn move_frame(transform: &mut Transform, current: &WorldTransform, scale: f32, to: &WorldTransform) {
    let parent_rot = current.rotation * transform.rotation.inverse();
    let parent_scale = if transform.scale.y.abs() > 1e-6 { scale / transform.scale.y } else { 1.0 };
    let delta = current.pos.delta_f32(to.pos);
    transform.translation += parent_rot.inverse() * delta / parent_scale.max(1e-6);
    transform.rotation = (parent_rot.inverse() * to.rotation).normalize();
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines)]
fn handoff(
    time: Res<Time>,
    config: Res<RagdollConfig>,
    source: Option<Res<RagdollSource>>,
    mut physics: ResMut<Physics>,
    mut ragdolls: ResMut<Ragdolls>,
    mut stats: ResMut<RagdollStats>,
    requests: Query<(Entity, &Ragdoll, &Animator)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    roots: Query<&WorldspaceRoot>,
    mut commands: Commands,
    mut failed: MessageWriter<RagdollFailed>,
) {
    let dt = time.delta_secs();
    let Ragdolls { pending, live } = &mut *ragdolls;
    // Removed requests and despawned characters drop their bodies.
    pending.retain(|e, _| requests.contains(*e));
    live.retain(|e, l| {
        let keep = requests.contains(*e);
        if !keep && let Some(w) = physics.worlds.get_mut(&l.worldspace) {
            l.instance.remove(w);
        }
        keep
    });
    let Some(source) = source else { return };
    for (entity, request, animator) in &requests {
        if let Some(l) = live.get_mut(&entity) {
            // A death on a live knockdown (the host replaced the request):
            // the same bodies crumple through the controller's terminal
            // `Dead` and never get up (AN-14: the new mode was ignored).
            if matches!(request.mode, RagdollMode::Death { .. }) && !matches!(l.mode, RagdollMode::Death { .. }) {
                let seed = entity.to_bits();
                l.controller.request(ReactionRequest { class: ReactionClass::Death, body: 0, point: WorldPos::default(), impulse: Vec3::ZERO, tick: 0, seed });
                l.mode = request.mode.clone();
            }
            continue;
        }
        let p = pending.entry(entity).or_insert_with(|| Pending { left: request.delay, poses: [None, None], dt: 0.0 });
        if animator.posed() {
            p.poses = [p.poses[1].take(), Some(animator.output.model.mats.clone())];
            p.dt = dt;
        }
        p.left -= dt;
        if p.left > 0.0 {
            continue;
        }
        let p = pending.remove(&entity).expect("present");
        let skeleton = &animator.asset.skeleton;
        let model = p.poses[1].clone().unwrap_or_else(|| skeleton.rest_model());
        let (Some(def), Some((ws, root, scale))) = ((source.0)(&request.skeleton), world_frame(entity, &frames.p0(), &roots)) else {
            stats.failed += 1;
            commands.entity(entity).try_remove::<Ragdoll>();
            failed.write(RagdollFailed { entity });
            continue;
        };
        if def.owner.len() != skeleton.len() || model.len() != skeleton.len() {
            stats.failed += 1;
            commands.entity(entity).try_remove::<Ragdoll>();
            failed.write(RagdollFailed { entity });
            continue;
        }
        let Some(world) = physics.worlds.get_mut(&ws) else {
            stats.failed += 1;
            commands.entity(entity).try_remove::<Ragdoll>();
            failed.write(RagdollFailed { entity });
            continue;
        };
        let previous = p.poses[0].as_deref().map(|prev| (prev, p.dt));
        let pose = SpawnPose { root, scale, model: &model, previous, root_velocity: Vec3::ZERO };
        let mut instance = RagdollInstance::spawn(world, &def, &pose, &config.settings);
        let budget = live.values().filter(|l| l.driving).count() < config.max_powered;
        // The controller owns the strength timeline (R2): a death crumples
        // along the set's death curves, a knockdown braces along its fall
        // curves; outside the powered budget (or a death without a crumple)
        // the bodies stay limp whatever the curves say.
        let mut controller = ReactionController::new(config.reactions.clone());
        let seed = entity.to_bits();
        let (driving, class, body, point, impulse) = match &request.mode {
            RagdollMode::Death { crumple } => (*crumple && budget, ReactionClass::Death, 0, WorldPos::default(), Vec3::ZERO),
            RagdollMode::Knockdown { point, impulse, .. } => {
                let body = instance.nearest_body(world, *point);
                if let Some(body) = body {
                    instance.hit(world, &def, &HitImpulse { body, point: *point, impulse: *impulse, dip: 1.0, recover: 0.5 });
                }
                (budget, ReactionClass::Knockdown, body.unwrap_or(0), *point, *impulse)
            }
        };
        controller.request(ReactionRequest { class, body, point, impulse, tick: 0, seed });
        let regions = def.regions.iter().map(|name| Region::of_name(name)).collect();
        // Every joint shows the ragdoll (all regions simulated), including
        // the joints above the root body: they carry the body's placement
        // in the frame (R0: at weight 0 the clip's root moved the mesh off
        // the bodies). Partial ragdolls will set region weights below 1.
        let weights = vec![1.0; skeleton.len()];
        let frozen = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
        // Bodies scale uniformly (`scale` is the frame's Y): a non-uniform
        // build scale (height on Y, build on X/Z) would skew bones lying
        // sideways off their bodies, so the ragdoll shows the uniform height
        // scale until the get-up restores the build.
        let own_scale = frames.p1().get_mut(entity).map_or(Vec3::ONE, |mut t| {
            let own = t.scale;
            t.scale = Vec3::splat(own.y);
            own
        });
        live.insert(
            entity,
            Live {
                worldspace: ws,
                def,
                instance,
                mode: request.mode.clone(),
                frozen,
                target: model,
                root,
                scale,
                weight: 0.0,
                weights,
                controller,
                regions,
                driving,
                own_scale,
            },
        );
        stats.spawned += 1;
    }
}

/// Every fixed step: the reaction controller (strength timeline, rest
/// detection), the powered drives it sets, and dips.
#[allow(clippy::needless_pass_by_value)]
fn drive(fixed: Res<Time<Fixed>>, config: Res<RagdollConfig>, mut physics: ResMut<Physics>, mut ragdolls: ResMut<Ragdolls>) {
    let dt = fixed.delta_secs();
    for l in ragdolls.live.values_mut() {
        let Some(w) = physics.worlds.get_mut(&l.worldspace) else { continue };
        // `upright` and `grounded` are for the stagger and balance slices (R3/R4):
        // knockdown and death only use the pelvis speed.
        let sensed = Sensed { pelvis_speed: l.instance.pelvis_speed(w), ..Sensed::default() };
        let out = l.controller.step(dt, &sensed);
        if l.driving {
            let region = l.regions.iter().zip(0u8..).map(|(r, i)| (i, Drive::Powered { strength: out.strength(*r) })).collect();
            let profile = StrengthProfile { region, pelvis_world: 0.0 };
            let target = DriveTarget { root: l.root, model: &l.target, previous: None };
            l.instance.drive(w, &l.def, &target, &profile, &config.drive);
            l.driving = out.max_strength() > 0.0;
        }
        l.instance.advance(dt);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn read_back(
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    config: Res<RagdollConfig>,
    physics: Res<Physics>,
    mut ragdolls: ResMut<Ragdolls>,
    mut stats: ResMut<RagdollStats>,
    mut characters: Query<(&Animator, Option<&mut PhysicsPose>)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    roots: Query<&WorldspaceRoot>,
    mut commands: Commands,
) {
    let alpha = fixed.overstep_fraction();
    let dt = time.delta_secs();
    let (mut awake, mut stretch) = (0, 0.0f32);
    for (entity, l) in &mut ragdolls.live {
        let Some(w) = physics.worlds.get(&l.worldspace) else { continue };
        let Ok((animator, pose)) = characters.get_mut(*entity) else { continue };
        let asleep = l.instance.asleep(w);
        awake += usize::from(!asleep);
        stretch = stretch.max(l.instance.max_stretch(w));
        let fading = l.weight < 1.0;
        l.weight = if config.fade > 0.0 { (l.weight + dt / config.fade).min(1.0) } else { 1.0 };
        if asleep && !fading && pose.is_some() {
            continue;
        }
        // The pose is read back relative to the entity's current frame,
        // which first follows the ragdoll in every mode (heading and
        // position), so culling, LOD and cell moves follow the body.
        let Some((_, mut frame, scale)) = world_frame(*entity, &frames.p0(), &roots) else { continue };
        if let Some(implied) = l.instance.implied_frame(w, &l.def, &l.target, alpha) {
            let to = WorldTransform { pos: implied.pos, rotation: yaw_of(implied.rotation), scale: Vec3::ONE };
            if let Ok(mut t) = frames.p1().get_mut(*entity) {
                move_frame(&mut t, &frame, scale, &to);
                frame = to;
            }
        }
        let mut out = Vec::new();
        if !l.instance.local_pose(w, &l.def, &animator.asset.skeleton, &frame, alpha, &l.frozen, &mut out) {
            continue;
        }
        match pose {
            Some(mut p) => {
                p.pose = out;
                p.weight = l.weight;
                p.weights.clone_from(&l.weights);
            }
            None => {
                commands.entity(*entity).try_insert(PhysicsPose { pose: out, weight: l.weight, weights: l.weights.clone() });
            }
        }
    }
    stats.live = ragdolls.live.len();
    stats.powered = ragdolls.powered();
    stats.awake = awake;
    stats.max_stretch = stretch;
}

/// The get-up's ground probe: from this far above the frame (m), and this
/// far below it.
const GET_UP_PROBE: (f32, f32) = (1.0, 2.0);

/// Knocked-down ragdolls at rest get up.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn get_up(
    config: Res<RagdollConfig>,
    mut physics: ResMut<Physics>,
    mut ragdolls: ResMut<Ragdolls>,
    mut stats: ResMut<RagdollStats>,
    mut characters: Query<&mut Animator>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    roots: Query<&WorldspaceRoot>,
    mut got_up: MessageWriter<RagdollGotUp>,
    mut commands: Commands,
) {
    let ready: Vec<Entity> = ragdolls
        .live
        .iter()
        // Past the get-up, too: several fixed steps between two `Update`s can
        // carry the controller through its get-up timeline into `Tracking`,
        // and a poll for `GettingUp` alone then never handed it back (AN-4).
        .filter(|(_, l)| matches!(l.mode, RagdollMode::Knockdown { .. }) && matches!(l.controller.state(), ReactionState::GettingUp | ReactionState::Tracking))
        .map(|(e, _)| *e)
        .collect();
    for entity in ready {
        let l = ragdolls.live.remove(&entity).expect("listed");
        let RagdollMode::Knockdown { get_up: clip_body, get_up_flag, .. } = &l.mode else { continue };
        let Some(w) = physics.worlds.get_mut(&l.worldspace) else {
            commands.entity(entity).try_remove::<(Ragdoll, PhysicsPose)>();
            got_up.write(RagdollGotUp { entity });
            continue;
        };
        let (Some(pelvis), Some((_, frame, scale)), Ok(mut animator)) = (w.pose(l.instance.bodies[0]), world_frame(entity, &frames.p0(), &roots), characters.get_mut(entity)) else {
            // Handed back where it lies, unaligned: the host still restores it.
            l.instance.remove(w);
            commands.entity(entity).try_remove::<(Ragdoll, PhysicsPose)>();
            got_up.write(RagdollGotUp { entity });
            continue;
        };
        let clip_body = clip_body.unwrap_or_else(|| lying_rest(&l.def, &l.target));
        // The clip plays under the build's own (non-uniform) scale, restored
        // below: its root body sits at that scale, not the uniform one.
        let build = l.own_scale / l.own_scale.y.max(1e-4);
        let clip_body = (clip_body.0, clip_body.1 * build);
        let face_down = l.instance.implied_frame(w, &l.def, &l.target, 1.0).is_some_and(|f| (f.rotation * Vec3::Z).y < 0.0);
        // The root body's long axis in its own frame: model up in the rest pose.
        let b0 = &l.def.bodies[0];
        let rest = animator.asset.skeleton.rest_model();
        let spine = rest.get(usize::from(b0.joint)).map_or(Vec3::Y, |m| (m.to_scale_rotation_translation().1 * b0.local.1).inverse() * Vec3::Y);
        let mut to = get_up_root(&pelvis, clip_body, spine, l.scale, face_down);
        // On the ground under it (physics.md "get-up"): the frame is where the
        // capsule goes and where the clip stands up, so the mover doesn't
        // re-ground it after the hand-back.
        let probe = sf_controller::ProbeQuery::ray(to.pos.offset(Vec3::Y * GET_UP_PROBE.0), Vec3::NEG_Y, GET_UP_PROBE.0 + GET_UP_PROBE.1);
        if let Some((_, hit)) = w.probe(sf_physics::Layer::WORLD).cast(&probe) {
            to.pos = to.pos.offset(Vec3::Y * to.pos.delta_f32(hit.point).y);
        }
        if let Ok(mut t) = frames.p1().get_mut(entity) {
            move_frame(&mut t, &frame, scale, &to);
            t.scale = l.own_scale;
        }
        // Hand back: the graph inertializes out of the ragdoll's pose, read
        // relative to the new frame (at rest: no velocity).
        let mut pose = Vec::new();
        if l.instance.local_pose(w, &l.def, &animator.asset.skeleton, &to, 1.0, &l.frozen, &mut pose) {
            // That pose is in the uniform scale the ragdoll showed; under the
            // restored build scale its pelvis would move (5 cm for a 0.95/0.9
            // build, AN-5). Shift the whole pose so the pelvis stays exactly on
            // the body; the limbs' few-% skew blends out with the clip.
            anchor_pelvis(&animator.asset.skeleton, usize::from(l.def.bodies[0].joint), build, &mut pose);
            animator.instance.inertialize_from(&pose, &pose, 1.0 / 60.0, config.get_up_blend);
        }
        animator.set_bool(get_up_flag, false);
        l.instance.remove(w);
        commands.entity(entity).try_remove::<(Ragdoll, PhysicsPose)>();
        got_up.write(RagdollGotUp { entity });
        stats.got_up += 1;
    }
}

/// Shifts a local pose (model space at the uniform scale) so `pelvis` keeps
/// its world position when shown under the per-axis scale ratio `build`.
fn anchor_pelvis(skeleton: &sf_anim::Skeleton, pelvis: usize, build: Vec3, pose: &mut [Trs]) {
    let mut chain = vec![pelvis];
    while let Some(p) = skeleton.parent(*chain.last().expect("non-empty")) {
        chain.push(p);
    }
    let Some(&top) = chain.last() else { return };
    if chain.iter().any(|&j| j >= pose.len()) {
        return;
    }
    let model = chain.iter().rev().fold(Affine3A::IDENTITY, |m, &j| m * pose[j].to_affine());
    let p = Vec3::from(model.translation);
    pose[top].translation += p / build.max(Vec3::splat(1e-4)) - p;
}

/// The get-up clip's root body at `time` (usually 0, its first frame), in
/// model space at scale 1: what [`RagdollMode::Knockdown::get_up`] takes.
/// The clip is sampled over the rest pose, so joints it doesn't animate
/// keep their rest transforms.
pub fn clip_root_body(def: &RagdollDef, skeleton: &sf_anim::Skeleton, clip: &sf_anim::Clip, time: f32) -> Option<(Quat, Vec3)> {
    let b0 = def.bodies.first()?;
    let joint = usize::from(b0.joint);
    if joint >= skeleton.len() {
        return None;
    }
    let mut local = sf_anim::LocalPose::rest(skeleton);
    clip.sample(time, &mut local, None);
    let mut model = sf_anim::ModelPose::new(skeleton.len());
    sf_anim::local_to_model(skeleton, &local, &mut model, &Affine3A::IDENTITY);
    let (_, r, t) = model.mats[joint].to_scale_rotation_translation();
    Some(((r * b0.local.1).normalize(), t + r * b0.local.0))
}

/// [`LYING_REST_GET_UP`]: the handoff pose's root body turned onto its back.
fn lying_rest(def: &RagdollDef, model: &[Affine3A]) -> (Quat, Vec3) {
    let b0 = &def.bodies[0];
    let (_, r, t) = model[usize::from(b0.joint)].to_scale_rotation_translation();
    let lying = Quat::from_rotation_x(LYING_REST_GET_UP);
    let pos = t + r * b0.local.0;
    // On its back at the pelvis body's radius above the ground.
    (lying * r * b0.local.1, lying * Vec3::new(pos.x, 0.0, pos.z) + Vec3::Y * b0.shape.radius)
}
