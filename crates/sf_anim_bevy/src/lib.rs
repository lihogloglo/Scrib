//! Bevy glue for the Bevy-free pose runtime (`sf_anim`, `sf_anim_graph`).
//! Spec: `docs/specs/animation.md` "Bevy integration (`sf_anim_bevy`)".
//!
//! A character is a root entity with an [`Animator`] (shared baked asset +
//! its own graph instance and output), an [`AnimLod`] and a
//! [`SkeletonBinding`]: one bone entity per skeleton joint, spawned by
//! [`spawn_rig`] and tagged [`AnimJoint`]. Skinned meshes bind to those
//! entities by name (`sf_model_bevy::resolve_joints`). Every frame, in
//! `PostUpdate` before transform propagation:
//!
//! 1. [`AnimSystems::Lod`]: tier per character from projected height and
//!    significance, then the budget allocator ([`lod`]).
//! 2. [`AnimSystems::Probe`]: host systems fill [`Animator::drive`] (ground
//!    samples, IK and warp targets, velocity). Empty here.
//! 3. [`AnimSystems::Evaluate`]: `par_iter` over characters due this frame;
//!    [`sf_anim_graph::GraphInstance::update`] is pure, so this is
//!    deterministic on any thread count.
//! 4. [`AnimSystems::Apply`]: bone `Transform`s from the local pose
//!    (`par_iter` over bone entities, written only when they change so a
//!    frozen character costs no propagation), root motion, events as
//!    [`AnimEvent`] messages and [`AnimNotify`] entity events for observers.
//!
//! Bevy then propagates transforms and skins on the GPU, as for glTF.

pub mod lod;
#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_camera::{Camera, Projection};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_time::Time;
use bevy_transform::TransformSystems;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_anim::{Clip, EventEdge, RootDelta, Skeleton, Trs};
use sf_anim_graph::{
    BindError, Binding, CompiledGraph, GraphInstance, GraphOutput, GroundSample, IkTarget,
    MontageId, ParamId, UpdateInputs, WarpTarget,
};
use sf_diag_bevy::DiagCounters;

pub use lod::{AnimBudget, AnimLod, AnimLodSettings, AnimTier};

/// The `PostUpdate` sets, chained, before `TransformSystems::Propagate`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimSystems {
    Lod,
    Probe,
    Evaluate,
    Apply,
}

/// Registers the schedule, messages and resources. Gizmos: feature `debug`.
pub struct SfAnimPlugin;

impl Plugin for SfAnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AnimEvent>()
            .init_resource::<AnimLodSettings>()
            .init_resource::<AnimBudget>()
            .init_resource::<AnimStats>()
            .init_resource::<AnimFrame>()
            .configure_sets(
                PostUpdate,
                (
                    AnimSystems::Lod,
                    AnimSystems::Probe,
                    AnimSystems::Evaluate,
                    AnimSystems::Apply,
                )
                    .chain()
                    .before(TransformSystems::Propagate),
            )
            .add_systems(PostUpdate, select_tiers.in_set(AnimSystems::Lod))
            .add_systems(PostUpdate, evaluate.in_set(AnimSystems::Evaluate))
            .add_systems(
                PostUpdate,
                (write_bones, apply_root_and_events).in_set(AnimSystems::Apply),
            );
        // Drawn only when the app added Bevy's gizmo plugin (headless apps and
        // tests run without it).
        #[cfg(feature = "debug")]
        app.init_resource::<AnimGizmos>().add_systems(
            PostUpdate,
            draw_skeletons
                .after(TransformSystems::Propagate)
                .run_if(resource_exists::<bevy_gizmos::config::GizmoConfigStore>),
        );
    }
}

// --- Assets and components ---------------------------------------------------------------

/// A baked character animation: skeleton, compiled graph and its clips in
/// clip-slot order, validated to fit together. Shared by every character
/// that plays it.
pub struct AnimAsset {
    pub skeleton: Arc<Skeleton>,
    pub graph: Arc<CompiledGraph>,
    pub clips: Vec<Arc<Clip>>,
    /// `graph.events`, shared into [`AnimEvent`]s without a copy per event.
    pub event_names: Vec<Arc<str>>,
}

impl AnimAsset {
    pub fn new(
        skeleton: Arc<Skeleton>,
        graph: Arc<CompiledGraph>,
        clips: Vec<Arc<Clip>>,
    ) -> Result<Self, BindError> {
        Binding::new(&graph, &skeleton, &clips)?;
        let event_names = graph.events.iter().map(|e| Arc::from(e.as_str())).collect();
        Ok(Self {
            skeleton,
            graph,
            clips,
            event_names,
        })
    }

    pub fn binding(&self) -> Binding<'_> {
        Binding {
            graph: &self.graph,
            skeleton: &self.skeleton,
            clips: &self.clips,
        }
    }

    /// Bounding height of the rest pose (LOD input), metres.
    pub fn rest_height(&self) -> f32 {
        let model = self.skeleton.rest_model();
        let (lo, hi) = model.iter().fold((f32::MAX, f32::MIN), |(lo, hi), m| {
            (lo.min(m.translation.y), hi.max(m.translation.y))
        });
        if hi > lo { hi - lo } else { 1.0 }
    }
}

/// Per-update inputs from gameplay and the Probe stage, character space.
#[derive(Clone, Debug, Default)]
pub struct AnimDrive {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub turn_rate: f32,
    pub look_target: Option<Vec3>,
    /// How the character moved since the previous update (its frame then).
    pub moved: RootDelta,
    /// Per graph leg: ground under it, probed at last output's `probe_origins`.
    pub ground: Vec<Option<GroundSample>>,
    pub ik_targets: Vec<Option<IkTarget>>,
    pub warp_targets: Vec<Option<WarpTarget>>,
}

/// The character's graph: shared asset, own instance (parameters inside)
/// and last output.
#[derive(Component)]
pub struct Animator {
    pub asset: Arc<AnimAsset>,
    pub instance: GraphInstance,
    pub output: GraphOutput,
    pub drive: AnimDrive,
    /// Playback rate multiplier (1 = real time).
    pub time_scale: f32,
    /// Whether the last Evaluate updated this character, and posed it.
    updated: bool,
    posed: bool,
}

impl Animator {
    pub fn new(asset: Arc<AnimAsset>) -> Self {
        let b = asset.binding();
        let instance = GraphInstance::new(&b);
        let output = GraphOutput::new(&asset.graph);
        let drive = AnimDrive {
            ground: vec![None; asset.graph.legs.len()],
            ik_targets: vec![None; asset.graph.ik_targets.len()],
            warp_targets: vec![None; asset.graph.warp_targets.len()],
            ..AnimDrive::default()
        };
        Self {
            asset,
            instance,
            output,
            drive,
            time_scale: 1.0,
            updated: false,
            posed: false,
        }
    }

    pub fn param(&self, name: &str) -> Option<ParamId> {
        self.asset.graph.param(name)
    }

    /// Sets a float parameter by name; false if the graph has none.
    pub fn set_float(&mut self, name: &str, v: f32) -> bool {
        self.param(name)
            .map(|id| self.instance.params.set_float(id, v))
            .is_some()
    }

    pub fn set_bool(&mut self, name: &str, v: bool) -> bool {
        self.param(name)
            .map(|id| self.instance.params.set_bool(id, v))
            .is_some()
    }

    pub fn set_vec2(&mut self, name: &str, v: bevy_math::Vec2) -> bool {
        self.param(name)
            .map(|id| self.instance.params.set_vec2(id, v))
            .is_some()
    }

    pub fn fire(&mut self, name: &str) -> bool {
        self.param(name)
            .map(|id| self.instance.params.fire(id))
            .is_some()
    }

    /// Whether the last frame's Evaluate posed this character.
    pub fn posed(&self) -> bool {
        self.posed
    }

    /// One graph update (what Evaluate runs per character).
    pub fn update(&mut self, dt: f32, tier: AnimTier) {
        let Self {
            asset,
            instance,
            output,
            drive,
            time_scale,
            ..
        } = self;
        let inputs = UpdateInputs {
            velocity: drive.velocity,
            moved: drive.moved,
            ground: &drive.ground,
            look_target: drive.look_target,
            ik_targets: &drive.ik_targets,
            warp_targets: &drive.warp_targets,
            turn_rate: drive.turn_rate,
            acceleration: drive.acceleration,
            poses: &[],
        };
        instance.update(
            &asset.binding(),
            dt * *time_scale,
            tier.index(),
            &inputs,
            output,
        );
    }
}

/// Skeleton joint index → bone entity, on the character root.
#[derive(Component, Clone, Debug, Default)]
pub struct SkeletonBinding {
    pub joints: Vec<Entity>,
}

impl SkeletonBinding {
    /// The bone entity of a joint name: exact, then ASCII case-insensitive
    /// (exporters disagree on case; skins and skeletons may come from
    /// different files).
    pub fn find(&self, skeleton: &Skeleton, name: &str) -> Option<Entity> {
        let joint = skeleton.find(name).or_else(|| {
            skeleton
                .names()
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
        })?;
        self.joints.get(joint).copied()
    }
}

/// On each bone entity: its character and skeleton joint.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimJoint {
    pub character: Entity,
    pub joint: u16,
}

/// Root motion of the last update, for the mover (locomotion.md), or
/// applied to the root's `Transform` here when `apply` is set (no mover).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AnimRootMotion {
    pub delta: RootDelta,
    pub drive_weight: f32,
    pub apply: bool,
}

/// A graph event, as a buffered message (`MessageReader<AnimEvent>`).
#[derive(Message, Clone, Debug)]
pub struct AnimEvent {
    pub entity: Entity,
    /// Graph-wide event id and name (`CompiledGraph::events`).
    pub id: u16,
    pub name: Arc<str>,
    pub edge: EventEdge,
    pub weight: f32,
    pub montage: Option<MontageId>,
}

/// The same event triggered on the character for entity observers
/// (`commands.entity(e).observe(|n: On<AnimNotify>| ..)`).
#[derive(EntityEvent, Clone, Debug)]
pub struct AnimNotify {
    pub entity: Entity,
    pub id: u16,
    pub name: Arc<str>,
    pub edge: EventEdge,
    pub weight: f32,
}

/// Last frame's numbers (also reported as `anim/...` counters when the app
/// runs `SfDiagPlugin`).
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimStats {
    pub per_tier: [usize; 4],
    /// `FootPlacement` guards that fired in the last frame's updates…
    pub feet: sf_anim_graph::FootStats,
    /// …and since the app started (diagnostics: a healthy crowd rarely clamps).
    pub feet_total: sf_anim_graph::FootStats,
    pub evaluated: usize,
    /// Wall time of Evaluate and Apply, µs.
    pub evaluate_us: f32,
    pub apply_us: f32,
    /// Evaluation time summed over threads, µs.
    pub cpu_us: f32,
    pub bones_written: usize,
    pub events: usize,
}

#[derive(Resource, Default)]
struct AnimFrame(u32);

fn trs_transform(t: Trs) -> Transform {
    Transform {
        translation: t.translation,
        rotation: t.rotation,
        scale: t.scale,
    }
}

/// Spawns one bone entity per joint of `skeleton` (rest transforms, `Name`,
/// [`AnimJoint`]) under `character`, inserts the [`SkeletonBinding`] and
/// returns a copy. The character still needs an [`Animator`] and an
/// [`AnimLod`] to animate.
pub fn spawn_rig(
    commands: &mut Commands,
    character: Entity,
    skeleton: &Skeleton,
) -> SkeletonBinding {
    let rest = skeleton.rest();
    let joints: Vec<Entity> = (0..skeleton.len())
        .map(|j| {
            let joint = u16::try_from(j).expect("skeletons have at most 65535 joints");
            commands
                .spawn((
                    Name::new(skeleton.name(j).to_owned()),
                    trs_transform(rest[j]),
                    AnimJoint { character, joint },
                ))
                .id()
        })
        .collect();
    for (j, &e) in joints.iter().enumerate() {
        let parent = skeleton.parent(j).map_or(character, |p| joints[p]);
        commands.entity(e).insert(ChildOf(parent));
    }
    let binding = SkeletonBinding { joints };
    commands.entity(character).insert(binding.clone());
    binding
}

// --- Systems --------------------------------------------------------------------------------

fn select_tiers(
    settings: Res<AnimLodSettings>,
    mut budget: ResMut<AnimBudget>,
    time: Res<Time>,
    mut frame: ResMut<AnimFrame>,
    cameras: Query<(&Camera, &GlobalTransform, &Projection)>,
    mut chars: Query<(Entity, Option<&GlobalTransform>, &mut AnimLod), With<Animator>>,
) {
    frame.0 = frame.0.wrapping_add(1);
    let dt = time.delta_secs();
    let view = cameras
        .iter()
        .filter(|(c, ..)| c.is_active)
        .max_by_key(|(c, ..)| c.order)
        .and_then(|(_, t, p)| match p {
            Projection::Perspective(p) => Some((t.affine().inverse(), p.fov, p.aspect_ratio)),
            _ => None,
        });
    for (entity, at, mut lod) in &mut chars {
        let current = lod.tier;
        let tier = if let Some(t) = lod.forced {
            t
        } else if let (Some((to_view, fov, aspect)), Some(at)) = (view, at) {
            let center = at.translation() + Vec3::Y * (lod.height * 0.5);
            let local = to_view.transform_point3(center);
            let distance = local.length();
            lod.pixels =
                lod::projected_pixels(lod.height, distance, fov, settings.reference_height);
            // In front and inside the view cone, padded by the character's size.
            let depth = -local.z;
            let pad = lod.height;
            let half_y = (fov * 0.5).tan();
            let visible = depth > -pad
                && local.x.abs() <= (depth.max(0.0) * half_y * aspect) + pad
                && local.y.abs() <= (depth.max(0.0) * half_y) + pad;
            if settings.cull_offscreen && !visible {
                AnimTier::A3
            } else {
                settings.tier(lod.pixels * lod.significance, current)
            }
        } else {
            AnimTier::A0
        };
        lod.tier = tier;
        if lod.phase == 0 {
            // Stagger reduced-rate updates by entity (1-based so 0 = unset).
            #[allow(clippy::cast_possible_truncation)]
            let low = entity.to_bits() as u32;
            lod.phase = low % lod::A2_INTERVAL + 1;
        }
    }
    if let Some(limit) = settings.budget_us {
        let mut picks: Vec<(f32, AnimTier)> = Vec::new();
        let mut who = Vec::new();
        for (entity, _, lod) in &chars {
            if lod.forced.is_none() {
                picks.push((lod.pixels * lod.significance, lod.tier));
                who.push(entity);
            }
        }
        let (predicted, demoted) = lod::allocate(&mut picks, &budget, limit);
        budget.predicted_us = predicted;
        budget.demoted = demoted;
        for (e, (_, t)) in who.into_iter().zip(picks) {
            if let Ok((_, _, mut lod)) = chars.get_mut(e) {
                lod.tier = t;
            }
        }
    }
    for (_, _, mut lod) in &mut chars {
        lod.pending += dt;
        let interval = lod.tier.interval();
        lod.due = (frame.0 + lod.phase).is_multiple_of(interval);
    }
}

fn evaluate(
    mut chars: Query<(&mut Animator, &mut AnimLod, Option<&PhysicsPose>)>,
    mut stats: ResMut<AnimStats>,
    mut budget: ResMut<AnimBudget>,
    counters: Option<Res<DiagCounters>>,
) {
    let start = Instant::now();
    let ns: [AtomicU64; 4] = Default::default();
    let count: [AtomicU64; 4] = Default::default();
    chars.par_iter_mut().for_each(|(mut animator, mut lod, physics)| {
        animator.updated = lod.due;
        animator.posed = false;
        if !lod.due {
            return;
        }
        match physics.filter(|p| p.weight > 0.0) {
            Some(p) => {
                let weights: Vec<f32> = (0..p.pose.len()).map(|j| p.joint_weight(j)).collect();
                animator.instance.set_physics(Some((&p.pose, &weights)));
            }
            None if animator.instance.has_physics() => animator.instance.set_physics(None),
            None => {}
        }
        let t0 = Instant::now();
        let dt = std::mem::take(&mut lod.pending);
        animator.update(dt, lod.tier);
        animator.posed = animator.output.posed;
        let i = lod.tier.index();
        #[allow(clippy::cast_possible_truncation)]
        ns[i].fetch_add(t0.elapsed().as_nanos() as u64, Ordering::Relaxed);
        count[i].fetch_add(1, Ordering::Relaxed);
    });
    let wall = start.elapsed();
    let mut per_tier = [0; 4];
    let mut feet = sf_anim_graph::FootStats::default();
    for (animator, lod, _) in &chars {
        per_tier[lod.tier.index()] += 1;
        if animator.updated {
            feet += animator.output.feet;
        }
    }
    stats.feet = feet;
    stats.feet_total += feet;
    let mut cpu = 0.0;
    let mut evaluated = 0;
    for t in AnimTier::ALL {
        let n = count[t.index()].load(Ordering::Relaxed);
        #[allow(clippy::cast_precision_loss)]
        let us = ns[t.index()].load(Ordering::Relaxed) as f32 / 1000.0;
        cpu += us;
        if n > 0 {
            #[allow(clippy::cast_precision_loss)]
            budget.observe(t, us / n as f32);
        }
        evaluated += usize::try_from(n).unwrap_or(usize::MAX);
    }
    stats.per_tier = per_tier;
    stats.evaluated = evaluated;
    stats.evaluate_us = wall.as_secs_f32() * 1e6;
    stats.cpu_us = cpu;
    if let Some(c) = counters {
        c.add_duration_named("anim/evaluate/ms", wall);
        c.add_named(
            "anim/evaluate_cpu/ms",
            sf_diag_bevy::sf_diag::unit::MS,
            f64::from(cpu) / 1000.0,
        );
        #[allow(clippy::cast_precision_loss)]
        {
            c.set_named(
                "anim/characters/count",
                sf_diag_bevy::sf_diag::unit::COUNT,
                per_tier.iter().sum::<usize>() as f64,
            );
            c.set_named(
                "anim/evaluated/count",
                sf_diag_bevy::sf_diag::unit::COUNT,
                evaluated as f64,
            );
            for (t, name) in [
                "anim/a0/count",
                "anim/a1/count",
                "anim/a2/count",
                "anim/a3/count",
            ]
            .iter()
            .enumerate()
            {
                c.set_named(name, sf_diag_bevy::sf_diag::unit::COUNT, per_tier[t] as f64);
            }
        }
    }
}

/// Bone write-back, one `par_iter` over bone entities (the shape of
/// `bevy_animation::animate_targets`): each reads its character's pose and
/// writes its `Transform` only when the value changed, so unchanged and
/// frozen bones never mark their subtree for propagation.
fn write_bones(
    animators: Query<(&Animator, Option<&PhysicsPose>)>,
    mut bones: Query<(&AnimJoint, &mut Transform)>,
    mut stats: ResMut<AnimStats>,
) {
    let start = Instant::now();
    let written = AtomicU64::new(0);
    bones.par_iter_mut().for_each(|(joint, mut transform)| {
        let Ok((animator, physics)) = animators.get(joint.character) else {
            return;
        };
        let j = usize::from(joint.joint);
        // The graph blends the physics pose in (`GraphInstance::set_physics`);
        // frames it doesn't pose still follow a fully physical joint.
        let t = if animator.posed {
            trs_transform(animator.output.pose.get(j))
        } else {
            match physics.filter(|p| j < p.pose.len() && p.joint_weight(j) >= 1.0) {
                Some(p) => trs_transform(p.pose[j]),
                None => return,
            }
        };
        if *transform != t {
            *transform = t;
            written.fetch_add(1, Ordering::Relaxed);
        }
    });
    stats.bones_written = usize::try_from(written.into_inner()).unwrap_or(usize::MAX);
    stats.apply_us = start.elapsed().as_secs_f32() * 1e6;
}

/// A physics pose over the animation (physics.md "Ragdolls and the
/// animation blend"), on the character's `Animator` entity: the ragdoll's
/// local pose, written by the physics layer every frame it moves; `weight`
/// 0 = animation, 1 = physics (bones then follow it even when the graph
/// doesn't pose this frame). Evaluate hands it to the graph
/// (`GraphInstance::set_physics`, P8), which blends it per joint after
/// inertialization and before the procedural ops, so a get-up inertializes
/// out of it (`GraphInstance::inertialize_from`).
#[derive(Component, Clone, Debug, Default)]
pub struct PhysicsPose {
    pub pose: Vec<Trs>,
    pub weight: f32,
    /// Per joint, multiplying `weight` (a ragdoll's per-region weights);
    /// empty = 1 everywhere.
    pub weights: Vec<f32>,
}

impl PhysicsPose {
    /// The physics weight of joint `j`.
    pub fn joint_weight(&self, j: usize) -> f32 {
        self.weight * self.weights.get(j).copied().unwrap_or(1.0)
    }
}

type RootQuery = (
    Entity,
    &'static Animator,
    Option<&'static mut AnimRootMotion>,
    Option<&'static mut Transform>,
);

fn apply_root_and_events(
    mut chars: Query<RootQuery, Without<AnimJoint>>,
    mut messages: MessageWriter<AnimEvent>,
    mut commands: Commands,
    mut stats: ResMut<AnimStats>,
    counters: Option<Res<DiagCounters>>,
) {
    let start = Instant::now();
    let mut events = 0;
    for (entity, animator, root, transform) in &mut chars {
        if !animator.updated {
            continue;
        }
        let out = &animator.output;
        if let Some(mut root) = root {
            root.delta = out.root;
            root.drive_weight = out.drive_weight;
            if root.apply
                && let Some(mut t) = transform
            {
                let step = t.rotation * out.root.translation;
                t.translation += step;
                t.rotation *= Quat::from_rotation_y(out.root.yaw);
            }
        }
        for e in &out.events {
            let name = animator
                .asset
                .event_names
                .get(usize::from(e.id))
                .cloned()
                .unwrap_or_else(|| Arc::from("?"));
            messages.write(AnimEvent {
                entity,
                id: e.id,
                name: name.clone(),
                edge: e.edge,
                weight: e.weight,
                montage: e.montage,
            });
            commands.trigger(AnimNotify {
                entity,
                id: e.id,
                name,
                edge: e.edge,
                weight: e.weight,
            });
            events += 1;
        }
    }
    stats.events = events;
    stats.apply_us += start.elapsed().as_secs_f32() * 1e6;
    if let Some(c) = counters {
        c.add_named(
            "anim/apply/ms",
            sf_diag_bevy::sf_diag::unit::MS,
            f64::from(stats.apply_us) / 1000.0,
        );
        #[allow(clippy::cast_precision_loss)]
        {
            c.add_named(
                "anim/events/count",
                sf_diag_bevy::sf_diag::unit::COUNT,
                events as f64,
            );
            c.add_named(
                "anim/bones_written/count",
                sf_diag_bevy::sf_diag::unit::COUNT,
                stats.bones_written as f64,
            );
        }
    }
}

// --- Gizmos ---------------------------------------------------------------------------------

/// Skeleton gizmos: a line per bone, coloured by LOD tier (A0 green, A1
/// yellow, A2 orange, A3 red). Toggle `enabled`.
#[cfg(feature = "debug")]
#[derive(Resource, Clone, Debug, Default)]
pub struct AnimGizmos {
    pub enabled: bool,
}

#[cfg(feature = "debug")]
fn draw_skeletons(
    settings: Res<AnimGizmos>,
    mut gizmos: bevy_gizmos::gizmos::Gizmos,
    chars: Query<(&SkeletonBinding, &Animator, Option<&AnimLod>)>,
    bones: Query<&GlobalTransform>,
) {
    use bevy_color::Color;
    if !settings.enabled {
        return;
    }
    for (binding, animator, lod) in &chars {
        let color = match lod.map_or(AnimTier::A0, |l| l.tier) {
            AnimTier::A0 => Color::srgb(0.2, 1.0, 0.3),
            AnimTier::A1 => Color::srgb(1.0, 0.95, 0.2),
            AnimTier::A2 => Color::srgb(1.0, 0.55, 0.1),
            AnimTier::A3 => Color::srgb(1.0, 0.15, 0.1),
        };
        let skeleton = &animator.asset.skeleton;
        for (j, &e) in binding.joints.iter().enumerate() {
            let Some(p) = skeleton.parent(j) else {
                continue;
            };
            if let (Ok(a), Ok(b)) = (bones.get(binding.joints[p]), bones.get(e)) {
                gizmos.line(a.translation(), b.translation(), color);
            }
        }
    }
}
