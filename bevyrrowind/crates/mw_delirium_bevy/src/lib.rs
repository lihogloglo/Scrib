//! Delirium (`mw_delirium`, GPL-3.0-only) takes over animated Morrowind characters (ROADMAP 3.2b slice 2a).
//!
//! - **Takeover**: a [`DeliriumReaction`] on a character's root (the entity with `CharacterBody`). Two posed frames
//!   later (the second gives part velocities) its evaluated pose becomes Delirium parts (`bones_to_parts`), the agent
//!   spawns at the actor's size (`Build::scaled` from its scale) on a floor at the ground and starts `bodyBalance`
//!   (`catchFall` when already falling). The root goes `Held` (so `Dormant`: no step, no motion or ground
//!   feed, the processing range leaves it alone) until the hand-back.
//! - **Step**: `FixedUpdate`, one Delirium step per fixed tick.
//! - **Copy-back**: every frame the parts (interpolated between the last two steps) go into the takeover pose
//!   (`pose_to_bones`; bones without a part keep the takeover pose) and to the animator as a [`PhysicsPose`] at
//!   weight 1, the death/knockdown ragdoll's path (`sf_physics_bevy` read-back): the graph shows it, ops skip it, root
//!   motion is off. The body's frame follows the pelvis (heading from the hips, on the floor) as in the knockdown
//!   mode, at the uniform height scale (restored at the hand-back). On the floor, not at the mover's render height: the
//!   root (its mover too, `height_offset` zeroed) takes the settled height at the hand-back, not the pelvis.
//! - **Hand-back**: recovered (`mw_delirium::Rules`: no catch step, centre of mass slow) → the blend-out (slice 2b):
//!   the physics-pose weight follows `Takeover::weight` from 1 to 0, the graph's pose (the idle, kept alive under
//!   `Held`) shows through, the frame stays where the blend started; at 0 the graph simply carries on (no
//!   inertialization). A push during it brings the physics back. Timeout (or a blow-up) → the graph inertializes from
//!   the shown pose (`inertialize_from`). Down → the graph's `knocked_out` flag is set while it lies there; once still
//!   (or after a few seconds) Delirium picks the get-up nearest the way the body lies (the graph's [`GET_UP_SLOTS`],
//!   LAFAN1 face up / face down / sides, chosen by the `get_up` param; without them `Knockout@tail`, which starts on
//!   its side) and rolls the body over to its first pose's tilt only past the tolerance (`Takeover::set_get_ups`,
//!   DE-1); lying, the body is relaxed (`bodyRelax`). Then the frame moves under that pose's pelvis (yaw and
//!   translation only, `get_up_root` without its roll) and the get-up plays out of the lying pose. The root takes the body's move (the walkers'
//!   knockdown glue) and wakes; [`DeliriumHandedBack`] tells the host.
//! - **World**: one flat floor at the ground under the pelvis (one ray down through the walkers' [`Physics`] ground
//!   probe; no physics or no hit: the mover's render height), lowered to the lowest part when the pose starts below it
//!   (on a slope, the downhill foot). Next: copy the Rapier statics around the actor.
//! - **Click-to-push** ([`DeliriumPlugin::click`], dev builds): left click on a human NPC picks the Delirium part under
//!   the cursor (capsules) and takes it over with an impulse there along the view ray (Ctrl 20, plain 45, Shift
//!   80 N s); on a taken-over one it pushes that part.
//!
//! Human skeletons only ([`HUMAN_SKELETONS`]): Delirium's asset is one humanoid per process; beast races are refused
//! and logged.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_camera::{Camera, Camera3d};
use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_input::mouse::MouseButton;
use bevy_log::{info, warn};
use bevy_math::{Affine3A, Quat, Vec3};
use bevy_time::{Fixed, Time};
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_window::{PrimaryWindow, Window};
use mw_delirium::delirium::prelude::Transform as Part;
use mw_delirium::delirium::ragdoll::Build;
use mw_delirium::takeover::pick;
use mw_delirium::{Body, HandBack, Options, Rules, Takeover, authoring_from_model, bones_to_parts, character_from_skeleton, pose_to_bones};
use sf_anim::Trs;
use sf_anim_bevy::{Animator, PhysicsPose};
use sf_character_bevy::CharacterBody;
use sf_controller::ProbeQuery;
use sf_controller_bevy::{ControllerSystems, Dormant, GroundProbe, Held, Mover};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics_bevy::sf_ragdoll::{get_up_root, yaw_of};
use sf_physics_bevy::{FrameQuery, Physics, Ragdoll, move_frame, world_frame};
use sf_world_bevy::{WorldPosOf, WorldspaceRoot};

/// The skeletons Delirium's humanoid asset fits (male and female Morrowind humans).
pub const HUMAN_SKELETONS: [&str; 2] = ["meshes/base_anim.nif", "meshes/base_anim_female.nif"];

/// The graph flag the fallen hand-back sets while it lies there and clears for the get-up (`mw_graphs` `knocked_out`).
pub const KNOCKED_OUT: &str = "knocked_out";

/// The get-up clip (`mw_graphs` `knockout_end`) when the graph has none of [`GET_UP_SLOTS`].
pub const GET_UP_CLIP: &str = "Knockout@tail";

/// The graph's get-up choice (`mw_graphs::build::params::GET_UP`): 0 `Knockout@tail`, `i + 1` [`GET_UP_SLOTS`]`[i]`.
pub const GET_UP_PARAM: &str = "get_up";

/// The get-up clip slots by how the body lies (`mw_graphs::build::GET_UPS`, sources.ron `getup.*`: LAFAN1 takes).
pub const GET_UP_SLOTS: [&str; 4] = ["getup.supine", "getup.prone", "getup.side_left", "getup.side_right"];

/// Hand-back inertialization, s (timeouts, blow-ups, the get-up).
pub const HAND_BACK_BLEND: f32 = 0.3;

/// A physics weight under this at the hand-back counts as blended out (no inertialization).
pub const BLENDED_OUT: f32 = 0.1;

/// The takeover's ground ray: from the pelvis straight down this far, m.
pub const GROUND_RAY: f32 = 4.0;

/// Click impulses, N s: Ctrl, plain, Shift.
pub const CLICK_IMPULSE: [f32; 3] = [20.0, 45.0, 80.0];

/// Adds the takeover systems; `click`: the dev click-to-push tool.
#[derive(Default)]
pub struct DeliriumPlugin {
    pub click: bool,
}

impl Plugin for DeliriumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Takeovers>()
            .init_resource::<DeliriumStats>()
            .add_message::<DeliriumHandedBack>()
            .add_systems(FixedUpdate, step)
            // After the controller step: a root move (hand-back) is never a frame behind.
            .add_systems(Update, (start, read_back.after(ControllerSystems::Step)).chain());
        if self.click {
            app.add_systems(Update, click.before(start));
        }
    }
}

/// Take this character over (on its root, the `CharacterBody` entity), optionally with a hit; on one already taken
/// over, the hit pushes its body.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DeliriumReaction {
    pub hit: Option<DeliriumHit>,
}

/// An impulse on the body.
#[derive(Clone, Copy, Debug)]
pub struct DeliriumHit {
    /// World point.
    pub point: WorldPos,
    /// N s, world.
    pub impulse: Vec3,
    /// Delirium part (`mw_delirium::body::NAMES`); `None`: the part nearest `point`.
    pub part: Option<usize>,
}

/// A character went back to its animation (on the root).
#[derive(Message, Clone, Copy, Debug)]
pub struct DeliriumHandedBack {
    pub root: Entity,
    pub how: HandBack,
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct DeliriumStats {
    pub live: usize,
    pub started: usize,
    pub refused: usize,
    pub handed_back: usize,
    /// Last fixed tick's Delirium time over every live character, and the peak, µs.
    pub step_us: f32,
    pub peak_step_us: f32,
}

/// A skeleton's Delirium character and its authored build (for picks before a takeover).
struct Rig {
    body: Body,
    build: Build,
}

/// The sim frame of one takeover: sim space is the actor's authoring frame (Z up, facing +Y) at the takeover,
/// anchored at its feet.
#[derive(Clone, Copy)]
struct SimFrame {
    anchor: WorldPos,
    /// World axes (relative to `anchor`) from sim axes: a rotation.
    world_from_sim: Affine3A,
}

impl SimFrame {
    fn at(frame: &WorldTransform) -> Self {
        Self { anchor: frame.pos, world_from_sim: Affine3A::from_quat(yaw_of(frame.rotation)) * authoring_from_model() }
    }

    /// Sim from the model space of an entity at world frame `frame`, uniform scale `scale`.
    fn sim_from_model(&self, frame: &WorldTransform, scale: f32) -> Affine3A {
        let rel = Affine3A::from_scale_rotation_translation(Vec3::splat(scale), frame.rotation, self.anchor.delta_f32(frame.pos));
        self.world_from_sim.inverse() * rel
    }

    fn sim_point(&self, p: WorldPos) -> Vec3 {
        self.world_from_sim.inverse().transform_point3(self.anchor.delta_f32(p))
    }

    fn sim_dir(&self, v: Vec3) -> Vec3 {
        self.world_from_sim.inverse().transform_vector3(v)
    }

    fn world_point(&self, p: Vec3) -> WorldPos {
        self.anchor.offset(self.world_from_sim.transform_point3(p))
    }
}

struct Pending {
    pose: Vec<Trs>,
    frame: WorldTransform,
    scale: f32,
    secs: f32,
    hit: Option<DeliriumHit>,
}

struct Live {
    body: Entity,
    rig: Arc<Rig>,
    takeover: Takeover,
    sim: SimFrame,
    /// The takeover pose: bones without a part keep it.
    frozen: Vec<Trs>,
    scale: f32,
    /// The body's local transform before the takeover (its own scale included).
    body_local: Transform,
    /// The pelvis joint's model position at the takeover (the frame keeps its horizontal offset).
    pelvis: Vec3,
    yaw: Quat,
    /// The last two steps' part frames (interpolated by the fixed overstep).
    frames: [Vec<Part>; 2],
    /// The last two steps' physics weights (`Takeover::weight`, the blend-out).
    weights: [f32; 2],
    /// The weight shown last frame.
    shown_weight: f32,
    pushes: Vec<(usize, Vec3, Vec3)>,
    hand_back: Option<HandBack>,
    flagged: bool,
    last_pose: Option<Vec<Trs>>,
    /// The graph's get-ups, in the takeover's `set_get_ups` order: (`get_up` value, clip slot).
    get_ups: Vec<(f32, usize)>,
}

#[derive(Resource, Default)]
pub struct Takeovers {
    rigs: HashMap<String, Option<Arc<Rig>>>,
    pending: HashMap<Entity, Pending>,
    live: HashMap<Entity, Live>,
}

impl Takeovers {
    /// Roots under Delirium now.
    pub fn live(&self) -> impl Iterator<Item = Entity> + '_ {
        self.live.keys().copied()
    }

    fn rig(&mut self, subject: &str, skeleton: &sf_anim::Skeleton) -> Option<Arc<Rig>> {
        self.rigs
            .entry(subject.to_owned())
            .or_insert_with(|| {
                if !HUMAN_SKELETONS.contains(&subject) {
                    info!("delirium: {subject} is not a human skeleton (beast races and creatures are not supported: one humanoid asset)");
                    return None;
                }
                match character_from_skeleton(skeleton, &Options::default()) {
                    Ok(body) => {
                        let build = body.character.build()?.0;
                        Some(Arc::new(Rig { body, build }))
                    }
                    Err(e) => {
                        warn!("delirium: no character for {subject}: {e}");
                        None
                    }
                }
            })
            .clone()
    }
}

fn lerp_parts(a: &[Part], b: &[Part], t: f32) -> Vec<Part> {
    a.iter().zip(b).map(|(a, b)| Part::from_translation(a.translation.lerp(b.translation, t)).with_rotation(a.rotation.slerp(b.rotation, t))).collect()
}

fn nearest_part(parts: &[Part], at: Vec3) -> usize {
    parts.iter().enumerate().min_by(|a, b| a.1.translation.distance_squared(at).total_cmp(&b.1.translation.distance_squared(at))).map_or(0, |p| p.0)
}

/// Requests: sample the pose, then (next posed frame) take over; hits on live takeovers become pushes.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines)]
fn start(
    time: Res<Time>,
    mut takeovers: ResMut<Takeovers>,
    mut stats: ResMut<DeliriumStats>,
    requests: Query<(Entity, &CharacterBody, &DeliriumReaction)>,
    animators: Query<(&Animator, Has<Ragdoll>)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    roots: Query<&WorldspaceRoot>,
    ground: Option<Res<Physics>>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    let Takeovers { pending, live, .. } = &mut *takeovers;
    pending.retain(|e, _| requests.contains(*e));
    for p in pending.values_mut() {
        p.secs += dt;
    }
    let mut ready = Vec::new();
    for (root, cb, request) in &requests {
        if let Some(l) = live.get_mut(&root) {
            if let Some(hit) = request.hit {
                let at = l.sim.sim_point(hit.point);
                let part = hit.part.unwrap_or_else(|| nearest_part(&l.frames[1], at));
                l.pushes.push((part, at, l.sim.sim_dir(hit.impulse)));
            }
            commands.entity(root).try_remove::<DeliriumReaction>();
            continue;
        }
        let Ok((animator, ragdoll)) = animators.get(cb.body) else { continue };
        if ragdoll || !HUMAN_SKELETONS.contains(&cb.skeleton.as_str()) {
            info!("delirium: refused {root} ({}{})", cb.skeleton, if ragdoll { ", already a ragdoll" } else { ": not a human skeleton" });
            stats.refused += 1;
            commands.entity(root).try_remove::<DeliriumReaction>();
            continue;
        }
        if !animator.posed() {
            continue;
        }
        let Some((_, frame, scale)) = world_frame(cb.body, &frames.p0(), &roots) else { continue };
        let skeleton = &animator.asset.skeleton;
        let pose: Vec<Trs> = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
        match pending.remove(&root) {
            None => {
                pending.insert(root, Pending { pose, frame, scale, secs: 0.0, hit: request.hit });
            }
            Some(prev) => ready.push((root, cb.body, cb.skeleton.clone(), prev, pose, frame, scale)),
        }
    }
    for (root, body, subject, prev, pose, frame, scale) in ready {
        commands.entity(root).try_remove::<DeliriumReaction>();
        let Ok((animator, _)) = animators.get(body) else { continue };
        let skeleton = &animator.asset.skeleton;
        let Some(rig) = takeovers.rig(&subject, skeleton) else {
            stats.refused += 1;
            continue;
        };
        let sim = SimFrame::at(&frame);
        let parts = bones_to_parts(&rig.body.table, skeleton, &pose, &sim.sim_from_model(&frame, scale));
        let before = bones_to_parts(&rig.body.table, skeleton, &prev.pose, &sim.sim_from_model(&prev.frame, prev.scale));
        // The body's own scale: height on Y, build on X/Z; Delirium scales lengths by the height and mass by the volume.
        let Some((body_local, own)) = frames.p1().get_mut(body).ok().map(|mut local| {
            let before = *local;
            local.scale = Vec3::splat(before.scale.y);
            (before, before.scale)
        }) else {
            continue;
        };
        let weight = scale.powi(3) * (own.x * own.z) / (own.y * own.y).max(1e-6);
        #[allow(clippy::cast_precision_loss)]
        let seed = (root.to_bits() % 1000) as f32 / 1000.0;
        // The floor at the ground under the pelvis (`Takeover::start` lowers it to the lowest part); no hit: at the
        // frame (the mover's render height).
        let ground_z = ground
            .as_deref()
            .and_then(|g| g.probe_one(ProbeQuery::down(sim.world_point(parts[0].translation), GROUND_RAY)))
            .map_or(0.0, |hit| sim.sim_point(hit.point).z);
        let mut takeover = Takeover::start(&rig.body, &parts, Some((&before, prev.secs.max(1e-3))), Some((scale, weight)), ground_z, seed, Rules::default());
        // Down, it takes the get-up nearest the way it lies, rolling over to its first pose only past the tolerance
        // (DE-1: no turn left to the blend).
        let get_ups = get_up_clips(animator);
        let tilts: Vec<Vec3> = get_ups.iter().map(|&(_, c)| mw_delirium::clip_tilt(&rig.body.table, skeleton, &first_pose(animator, c).to_vec())).collect();
        takeover.set_get_ups(&tilts);
        let hit = prev.hit.map(|h| {
            let at = sim.sim_point(h.point);
            let part = h.part.unwrap_or_else(|| nearest_part(&parts, at));
            takeover.push(part, at, sim.sim_dir(h.impulse));
            (part, h.impulse.length())
        });
        let model = mw_delirium::pose::model_pose(skeleton, &pose);
        let pelvis = Vec3::from(model[rig.body.table.parts[0].joint].translation);
        info!(
            "delirium: took over {root} ({subject}, scale {scale:.2}) with {}, floor {:+.2} m (ground {ground_z:+.2}){}",
            takeover.behaviour,
            takeover.floor,
            hit.map_or(String::new(), |(p, i)| format!(", {i:.0} N s on {}", mw_delirium::body::NAMES[p]))
        );
        commands.entity(root).try_insert(Held);
        stats.started += 1;
        takeovers.live.insert(
            root,
            Live {
                body,
                rig,
                frames: [parts.clone(), parts],
                weights: [1.0; 2],
                shown_weight: 1.0,
                takeover,
                sim,
                frozen: pose,
                scale,
                body_local,
                pelvis,
                yaw: yaw_of(frame.rotation),
                pushes: Vec::new(),
                hand_back: None,
                flagged: false,
                last_pose: None,
                get_ups,
            },
        );
    }
    stats.live = takeovers.live.len();
}

/// One Delirium step per fixed tick.
#[allow(clippy::needless_pass_by_value)]
fn step(fixed: Res<Time<Fixed>>, mut takeovers: ResMut<Takeovers>, mut stats: ResMut<DeliriumStats>) {
    let dt = fixed.delta_secs();
    let t0 = Instant::now();
    for (root, l) in &mut takeovers.live {
        if l.hand_back.is_some() {
            continue;
        }
        for (part, at, impulse) in l.pushes.drain(..) {
            l.takeover.push(part, at, impulse);
        }
        let was = l.takeover.blending();
        l.hand_back = l.takeover.step(dt);
        if !was && l.takeover.blending() {
            info!("delirium: {root} recovered, blending out to the animation ({:.1} s after the takeover)", l.takeover.time);
        }
        let now = l.takeover.frames();
        l.frames = [std::mem::replace(&mut l.frames[1], now.clone()), now];
        l.weights = [l.weights[1], l.takeover.weight];
    }
    stats.step_us = t0.elapsed().as_secs_f32() * 1e6;
    stats.peak_step_us = stats.peak_step_us.max(stats.step_us);
}

/// Where the body's frame goes while Delirium has it: on the floor under the pelvis (keeping the pelvis' horizontal
/// offset in the model), heading from the hips (kept while they are near vertical). On the floor, not at the
/// takeover's render height: the hand-back's root move then carries the settled height.
fn follow_frame(l: &mut Live, parts: &[Part]) -> WorldTransform {
    let left = l.sim.world_from_sim.transform_vector3(parts[1].translation - parts[4].translation).with_y(0.0);
    if left.length() > 0.05 {
        let facing = left.normalize().cross(Vec3::Y);
        l.yaw = Quat::from_rotation_y(facing.x.atan2(facing.z));
    }
    // Sim Z is world Y: the floor's sim height is its height over the anchor.
    let pelvis = l.sim.world_from_sim.transform_point3(parts[0].translation).with_y(l.takeover.floor);
    let offset = l.yaw * (Vec3::new(l.pelvis.x, 0.0, l.pelvis.z) * l.scale);
    WorldTransform { pos: l.sim.anchor.offset(pelvis - offset), rotation: l.yaw, scale: Vec3::ONE }
}

/// The local pose that shows `parts` on a body at `frame`.
fn shown_pose(l: &Live, skeleton: &sf_anim::Skeleton, parts: &[Part], frame: &WorldTransform) -> Vec<Trs> {
    let mut pose = l.frozen.clone();
    pose_to_bones(&l.rig.body.table, skeleton, parts, &l.sim.sim_from_model(frame, l.scale).inverse(), &mut pose);
    pose
}

/// The graph's get-ups as (`get_up` value, clip slot): the [`GET_UP_SLOTS`] clips the graph resolved (a clip library
/// such as LAFAN1 present), else `Knockout@tail` (value 0), else none.
fn get_up_clips(animator: &Animator) -> Vec<(f32, usize)> {
    let graph = &animator.asset.graph;
    #[allow(clippy::cast_precision_loss)]
    let slots: Vec<(f32, usize)> =
        GET_UP_SLOTS.iter().enumerate().filter_map(|(i, s)| graph.slot_clip(s).map(|c| ((i + 1) as f32, usize::from(c)))).collect();
    if !slots.is_empty() {
        return slots;
    }
    graph.clips.iter().position(|c| c.reference == GET_UP_CLIP).map(|c| (0.0, c)).into_iter().collect()
}

/// A clip's first frame (local pose).
fn first_pose(animator: &Animator, clip: usize) -> sf_anim::LocalPose {
    let mut local = sf_anim::LocalPose::rest(&animator.asset.skeleton);
    if let Some(clip) = animator.asset.clips.get(clip) {
        clip.sample(0.0, &mut local, None);
    }
    local
}

/// The get-up frame for a body lying in `pose` at `frame`: the picked get-up clip's first pelvis pose on the lying
/// pelvis (yaw and translation only), on the floor. Delirium picked the clip nearest the way the body lies and rolled
/// it to the clip's tilt first (`Takeover::set_get_ups`), so the yaw alone leaves the hand-back's blend little
/// whole-body turn (DE-1); no `ROLL_TO_BACK`.
fn get_up_frame(l: &Live, animator: &Animator, pose: &[Trs], frame: &WorldTransform) -> Option<WorldTransform> {
    let skeleton = &animator.asset.skeleton;
    let joint = l.rig.body.table.parts[0].joint;
    let local = first_pose(animator, l.get_ups.get(l.takeover.get_up()?)?.1);
    let mut model = sf_anim::ModelPose::new(skeleton.len());
    sf_anim::local_to_model(skeleton, &local, &mut model, &Affine3A::IDENTITY);
    let (_, clip_rot, clip_pos) = model.mats[joint].to_scale_rotation_translation();
    let (_, rot, pos) = mw_delirium::pose::model_pose(skeleton, pose)[joint].to_scale_rotation_translation();
    let body = WorldTransform { pos: frame.pos.offset(frame.rotation * (pos * l.scale)), rotation: frame.rotation * rot, scale: Vec3::ONE };
    let (_, rest, _) = skeleton.rest_model()[joint].to_scale_rotation_translation();
    let to = get_up_root(&body, (clip_rot, clip_pos), rest.inverse() * Vec3::Y, l.scale, false);
    let floor = l.sim.anchor.offset(Vec3::Y * l.takeover.floor);
    Some(WorldTransform { pos: to.pos.offset(Vec3::Y * to.pos.delta_f32(floor).y), ..to })
}

/// Copy-back every frame (the parts as a physics pose at weight 1, the frame on the pelvis) and the hand-backs.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines)]
fn read_back(
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    mut takeovers: ResMut<Takeovers>,
    mut stats: ResMut<DeliriumStats>,
    mut animators: Query<(&mut Animator, Option<&mut PhysicsPose>)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    mut movers: Query<&mut Mover>,
    roots: Query<&WorldspaceRoot>,
    mut handed: MessageWriter<DeliriumHandedBack>,
    mut commands: Commands,
) {
    let alpha = fixed.overstep_fraction();
    let dt = time.delta_secs().max(1e-4);
    let mut done = Vec::new();
    let live = &mut takeovers.live;
    // Despawned characters drop their takeover.
    live.retain(|_, l| animators.contains(l.body));
    for (&root, l) in live.iter_mut() {
        let Ok((mut animator, physics)) = animators.get_mut(l.body) else { continue };
        let Some((_, mut frame, _)) = world_frame(l.body, &frames.p0(), &roots) else { continue };
        let parts = lerp_parts(&l.frames[0], &l.frames[1], alpha);
        let weight = l.weights[0] + (l.weights[1] - l.weights[0]) * alpha;
        let asset = animator.asset.clone();
        let skeleton = &asset.skeleton;
        let finite = parts.iter().all(|p| p.translation.is_finite() && p.rotation.is_finite());
        let pose = if finite {
            // The frame follows the pelvis under full physics; during the blend-out it stays put: the graph's pose
            // stands in it (no slide), and the physics pose is read in whichever frame it is.
            if weight >= 1.0 {
                let to = follow_frame(l, &parts);
                if let Ok(mut t) = frames.p1().get_mut(l.body) {
                    move_frame(&mut t, &frame, l.scale, &to);
                    frame = to;
                }
            }
            shown_pose(l, skeleton, &parts, &frame)
        } else {
            l.hand_back = Some(HandBack::Exploded);
            l.last_pose.clone().unwrap_or_else(|| l.frozen.clone())
        };
        if l.takeover.fell() && !l.flagged {
            l.flagged = true;
            animator.set_bool(KNOCKED_OUT, true);
            animator.set_float(GET_UP_PARAM, 0.0);
            info!("delirium: {root} fell ({:.1} s after the takeover)", l.takeover.time);
        }
        let Some(how) = l.hand_back else {
            match physics {
                Some(mut p) => {
                    p.pose.clone_from(&pose);
                    p.weight = weight;
                    p.weights.clear();
                }
                None => {
                    commands.entity(l.body).try_insert(PhysicsPose { pose: pose.clone(), weight, weights: Vec::new() });
                }
            }
            l.shown_weight = weight;
            l.last_pose = Some(pose);
            continue;
        };
        // Hand back: the graph inertializes out of the shown pose, read in the frame it goes on in.
        let previous = l.last_pose.take().unwrap_or_else(|| pose.clone());
        let get_up = if how == HandBack::Fallen { get_up_frame(l, &animator, &pose, &frame) } else { None };
        if let Some(up) = get_up {
            let left = l.takeover.get_up_left().unwrap_or(0.0).to_degrees();
            let (value, _) = l.takeover.get_up().and_then(|i| l.get_ups.get(i).copied()).unwrap_or((0.0, 0));
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let name = if value > 0.0 { GET_UP_SLOTS[value as usize - 1] } else { GET_UP_CLIP };
            animator.set_float(GET_UP_PARAM, value);
            info!(
                "delirium: {root} gets up with {name} (down {:.1} s, rolled over {:.2} s, {left:.0} deg left to the blend)",
                l.takeover.down.unwrap_or(0.0),
                l.takeover.rolling.unwrap_or(0.0)
            );
            if let Ok(mut t) = frames.p1().get_mut(l.body) {
                move_frame(&mut t, &frame, l.scale, &up);
            }
            let lying = shown_pose(l, skeleton, &parts, &up);
            animator.instance.inertialize_from(&lying, &lying, 1.0 / 60.0, HAND_BACK_BLEND);
        } else if how == HandBack::Exploded {
            warn!("delirium: {root} blew up: handed back from its last good pose");
            animator.instance.inertialize_from(&previous, &previous, dt, HAND_BACK_BLEND);
        } else if how == HandBack::Balanced && l.shown_weight < BLENDED_OUT {
            // Blended out: the graph's pose is the one shown already; it carries on.
        } else {
            if how == HandBack::Fallen {
                info!("delirium: {root} has no {GET_UP_CLIP} clip: handed back lying, the graph blends it up");
            }
            animator.instance.inertialize_from(&pose, &previous, dt, HAND_BACK_BLEND);
        }
        if l.flagged {
            animator.set_bool(KNOCKED_OUT, false);
        }
        done.push((root, l.body, l.body_local, how, l.takeover.time));
    }
    // The root takes the body's move (heading included, its mover too), and the body its own local transform back.
    // The move carries the floor's height under the takeover's render height: the mover goes to its new render
    // position with `height_offset` zeroed, so the controller doesn't smooth the same height a second time.
    for (root, body, l0, how, secs) in done {
        live.remove(&root);
        let mut transforms = frames.p1();
        if let Ok([mut t, mut local]) = transforms.get_many_mut([root, body]) {
            let yaw = Mover::yaw_of(t.rotation * local.rotation * l0.rotation.inverse());
            let rotation = Quat::from_rotation_y(yaw);
            let shift = t.rotation * (t.scale * local.translation) - rotation * (t.scale * l0.translation);
            t.rotation = rotation;
            t.translation += shift;
            *local = l0;
            if let Ok(mut m) = movers.get_mut(root) {
                m.loco.position = m.loco.render_position().offset(shift);
                m.loco.height_offset = 0.0;
                m.loco.height_offset_velocity = 0.0;
                m.loco.motion.yaw = yaw;
                m.loco.motion.facing_goal = yaw;
                m.loco.motion.angular_velocity = 0.0;
            }
        }
        commands.entity(body).try_remove::<PhysicsPose>();
        commands.entity(root).try_remove::<(Held, Dormant)>();
        stats.handed_back += 1;
        handed.write(DeliriumHandedBack { root, how });
        info!("delirium: {root} handed back ({how:?}) after {secs:.1} s");
    }
    stats.live = live.len();
}

/// Dev click-to-push: left click picks the Delirium part under the cursor on a human NPC (Ctrl 20, plain 45,
/// Shift 80 N s along the view ray).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn click(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera3d>>,
    pos_of: WorldPosOf,
    mut takeovers: ResMut<Takeovers>,
    characters: Query<(Entity, &CharacterBody)>,
    animators: Query<&Animator>,
    frames: FrameQuery,
    roots: Query<&WorldspaceRoot>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = windows.iter().next().and_then(Window::cursor_position) else { return };
    let Some((cam, camera, view)) = cameras.iter().filter(|(_, c, _)| c.is_active).max_by_key(|(_, c, _)| c.order) else { return };
    let (Ok(ray), Some(eye)) = (camera.viewport_to_world(view, cursor), pos_of.get(cam)) else { return };
    let origin = eye.offset(ray.origin - view.translation());
    let dir = *ray.direction;
    let strength = if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        CLICK_IMPULSE[0]
    } else if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        CLICK_IMPULSE[2]
    } else {
        CLICK_IMPULSE[1]
    };
    let mut best: Option<(f32, Entity, usize, WorldPos)> = None;
    for (root, cb) in &characters {
        let Ok(animator) = animators.get(cb.body) else { continue };
        let skeleton = &animator.asset.skeleton;
        let (sim, hit) = if let Some(l) = takeovers.live.get(&root) {
            let parts = l.takeover.frames();
            let hit = pick(&l.takeover.build, &parts, l.sim.sim_point(origin), l.sim.sim_dir(dir), 100.0);
            (l.sim, hit)
        } else {
            if !HUMAN_SKELETONS.contains(&cb.skeleton.as_str()) {
                continue;
            }
            let Some((_, frame, scale)) = world_frame(cb.body, &frames, &roots) else { continue };
            // Cheap reject: nowhere near the ray.
            let feet = origin.delta_f32(frame.pos.offset(Vec3::Y));
            if (feet - dir * feet.dot(dir)).length() > 2.0 {
                continue;
            }
            let Some(rig) = takeovers.rig(&cb.skeleton, skeleton) else { continue };
            let sim = SimFrame::at(&frame);
            let pose: Vec<Trs> = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
            let parts = bones_to_parts(&rig.body.table, skeleton, &pose, &sim.sim_from_model(&frame, scale));
            let hit = pick(&rig.build, &parts, sim.sim_point(origin), sim.sim_dir(dir), 100.0);
            (sim, hit)
        };
        if let Some((part, t, at)) = hit
            && best.is_none_or(|b| t < b.0)
        {
            best = Some((t, root, part, sim.world_point(at)));
        }
    }
    let Some((_, root, part, point)) = best else { return };
    info!("delirium: click on {root}, {} ({strength:.0} N s)", mw_delirium::body::NAMES[part]);
    commands.entity(root).try_insert(DeliriumReaction { hit: Some(DeliriumHit { point, impulse: dir * strength, part: Some(part) }) });
}
