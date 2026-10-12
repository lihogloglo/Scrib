//! NIF particle systems in Bevy: each emitter instance runs its own
//! [`mw_nif::particles::Simulation`] and draws camera-facing quads from a
//! per-instance dynamic mesh.
//!
//! CPU simulation over a GPU particle crate: vanilla emitters hold tens of
//! particles, the NIF semantics (box emitters, grow/fade, color keys) stay
//! exact and unit-tested in a Bevy-free crate, and there's no third-party
//! renderer to track across Bevy releases. Revisit for dense exterior
//! weather if profiling asks for it.

use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::{Frustum, Sphere};
use bevy::camera::visibility::NoFrustumCulling;
use bevy::math::Vec3A;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use mw_nif::glam as nif_glam;
use mw_nif::particles::Simulation;

use crate::animation::{ModelAnimation, NifAnimation, seed_unit};

/// On a particle node: the particle systems (indices into
/// [`NifAnimation::particles`]) it renders.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ParticleEmitter {
    pub animation: Handle<NifAnimation>,
    pub systems: Vec<u32>,
}

/// Runtime state of one emitter instance (a child of the particle node).
#[derive(Component)]
pub struct ParticleInstance {
    animation: Handle<NifAnimation>,
    system: usize,
    simulation: Simulation,
    mesh: Handle<Mesh>,
    /// Seconds added to the clock, so copies of a model differ.
    phase: f32,
    /// Back-to-front sorting, for alpha-blended (not additive) systems.
    sort: bool,
    /// Node-to-world transform at the last step, for world-space systems
    /// (`None`: paused, or not stepped yet).
    last: Option<bevy::math::Affine3A>,
    /// The emitter's [`ParticleRange`].
    range: Option<f32>,
    /// Its system's entry in the emitter's [`LiveEmitter`].
    live: Option<LiveSystem>,
    /// An actor's system: its emitter may carry an [`ActorEmitter`] gate.
    actor: bool,
    /// Live array targets, rebuilt each step.
    targets: Vec<mw_nif::particles::EmitTarget>,
}

/// On a [`ParticleEmitter`]: beyond this distance from the camera (meters)
/// its particles hide and pause. For emitters on moving actors, which no
/// visibility range covers.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct ParticleRange(pub f32);

/// On a [`ParticleEmitter`] of an actor or a held prop: every system plays,
/// auto-play flag or not (an actor model's controllers run on its animation
/// time, nif-runtime.md "Actor parts"); the host gates them by visibility.
///
/// Emission follows the actor's animation: the host writes, per system,
/// whether its emission window is open at the current animation time (the
/// creature's `emit:` clip curves, `mw_anim::effects`). Systems it doesn't
/// list (a held torch) emit on their own controller clock, as on a placed
/// model.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ActorEmitter {
    /// `(system, emitting)`; `system` indexes [`NifAnimation::particles`].
    pub emitting: Vec<(u32, bool)>,
}

impl ActorEmitter {
    /// Whether the host says `system` emits now (`None`: not gated).
    pub fn emits(&self, system: u32) -> Option<bool> {
        self.emitting.iter().find(|e| e.0 == system).map(|e| e.1)
    }
}

/// On a [`ParticleEmitter`]: systems that emit from where the pose puts their
/// emitter node (and, for array emitters, their [`EmitTarget`] nodes)
/// instead of from the rest transforms: a creature's joints, or a placed
/// model's keyframed nodes.
///
/// [`EmitTarget`]: mw_nif::particles::EmitTarget
#[derive(Component, Clone, Debug, Default)]
pub struct LiveEmitter {
    pub systems: Vec<LiveSystem>,
}

/// One system of a [`LiveEmitter`].
#[derive(Clone, Debug, Default)]
pub struct LiveSystem {
    /// Index into [`NifAnimation::particles`].
    pub system: u32,
    /// The emitter node (`None`: its rest transform).
    pub emitter: Option<LiveNode>,
    /// Per [`EmitTarget`] of the system, same order: the node and its parent
    /// (`None`: rest). Empty: all at rest.
    ///
    /// [`EmitTarget`]: mw_nif::particles::EmitTarget
    pub targets: Vec<(Option<LiveNode>, Option<LiveNode>)>,
}

/// A model node as placed at runtime: `offset` in the space of entity `node`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiveNode {
    pub node: Entity,
    pub offset: bevy::math::Affine3A,
}

/// On the nodes of a placed model that an animated emitter needs live:
/// the node's index in the model (`mw_nif::Model::nodes`).
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct NifNode(pub u32);

/// A node moving farther than this in one step teleported (or the floating
/// origin shifted): its world-space particles move with it.
const CARRY_MAX_STEP_M: f32 = 4.0;

#[derive(Component)]
pub(crate) struct ParticlesStarted;

/// Spawns one simulated child per particle system once the model's
/// animation asset is available.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn start_particles(
    mut commands: Commands,
    time: Res<Time>,
    emitters: Query<
        (Entity, &ParticleEmitter, Option<&ParticleRange>, Option<&LiveEmitter>, Option<&ActorEmitter>),
        Without<ParticlesStarted>,
    >,
    parents: Query<&ChildOf>,
    roots: Query<(), With<ModelAnimation>>,
    children: Query<&Children>,
    nodes: Query<&NifNode>,
    animations: Res<Assets<NifAnimation>>,
    materials: Res<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, emitter, range, live, gate) in &emitters {
        let actor = gate.is_some();
        let Some(animation) = animations.get(&emitter.animation) else {
            continue;
        };
        commands.entity(entity).insert(ParticlesStarted);
        // A placed model's keyframed emitter nodes, by model node index.
        let model_nodes = || {
            let root = parents.iter_ancestors(entity).find(|&a| roots.contains(a)).unwrap_or(entity);
            let found: Vec<(u32, Entity)> = children.iter_descendants(root).filter_map(|e| nodes.get(e).ok().map(|n| (n.0, e))).collect();
            found
        };
        let mut model_nodes_cache: Option<Vec<(u32, Entity)>> = None;
        for &index in &emitter.systems {
            let Some(system) = animation.particles.get(index as usize) else {
                continue;
            };
            if !system.auto_play && !actor {
                continue; // driven by gameplay (spell effects): not yet
            }
            let Some(material) = animation.materials.get(system.material) else {
                continue;
            };
            let sort = materials.get(material).is_some_and(|m| matches!(m.alpha_mode, AlphaMode::Blend));
            let live = match live {
                Some(l) => l.systems.iter().find(|s| s.system == index).cloned(),
                None if system.animated_emitter && !actor => {
                    let found = model_nodes_cache.get_or_insert_with(model_nodes);
                    Some(static_live(index, system, found))
                }
                None => None,
            };
            // The model's controller clock: the phase its keyframed nodes use
            // (`animation::assign_phases`), so emission windows line up with
            // the motion.
            let phase = if system.random_start {
                seed_unit(parents.iter_ancestors(entity).last().unwrap_or(entity).to_bits()) * 600.0
            } else {
                0.0
            };
            let mut simulation = Simulation::new(u64::from(index) ^ entity.to_bits());
            let emitting = gate.and_then(|g| g.emits(index));
            simulation.prewarm(system, time.elapsed_secs() + phase, emitting);
            let mesh = meshes.add(empty_mesh());
            commands.spawn((
                ParticleInstance {
                    animation: emitter.animation.clone(),
                    system: index as usize,
                    simulation,
                    mesh: mesh.clone(),
                    phase,
                    sort,
                    last: None,
                    range: range.map(|r| r.0),
                    live,
                    actor,
                    targets: Vec::new(),
                },
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                // Particles drift outside any bounds computed at spawn.
                NoFrustumCulling,
                Transform::IDENTITY,
                Visibility::Inherited,
                ChildOf(entity),
            ));
        }
    }
}

/// A placed model's system whose emitter or array nodes are keyframed:
/// follows those node entities (marked [`NifNode`] by the loader).
fn static_live(index: u32, system: &mw_nif::particles::ParticleSystem, found: &[(u32, Entity)]) -> LiveSystem {
    let at = |node: usize| {
        let node = u32::try_from(node).ok()?;
        found.iter().find(|(n, _)| *n == node).map(|&(_, e)| LiveNode { node: e, offset: bevy::math::Affine3A::IDENTITY })
    };
    LiveSystem {
        system: index,
        emitter: at(system.emitter),
        targets: system.targets.iter().map(|t| (at(t.node), t.parent.and_then(at))).collect(),
    }
}

/// The nodes of `model` that [`static_live`] needs as entities.
pub(crate) fn live_nodes(model: &mw_nif::Model) -> Vec<usize> {
    let mut out: Vec<usize> = model
        .particles
        .iter()
        .filter(|s| s.animated_emitter)
        .flat_map(|s| std::iter::once(s.emitter).chain(s.targets.iter().flat_map(|t| std::iter::once(t.node).chain(t.parent))))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Radius around an emitter node that its particles may reach (drift, rising
/// smoke), for the view test below. Generous: a false "visible" only costs a
/// mesh rewrite.
const PARTICLE_REACH_M: f32 = 12.0;

/// Steps every emitter the camera can see and rebuilds its quads facing the
/// camera. Emitters outside the view frustum or their visibility range
/// (`ViewVisibility`: particles opt out of frustum culling, so it only carries
/// the range) pause: every rewrite re-uploads the mesh (Bevy frees and
/// reallocates it in the mesh slabs), which cost ~1.2 ms of render thread per
/// frame on a town fly-through when all ~120 resident emitters ran
/// (streaming.md, "Spikes").
pub(crate) fn simulate_particles(
    time: Res<Time>,
    animations: Res<Assets<NifAnimation>>,
    camera: Query<(&GlobalTransform, Option<&Frustum>), With<Camera3d>>,
    mut instances: Query<(&mut ParticleInstance, &ChildOf, &GlobalTransform, &InheritedVisibility, &ViewVisibility, &mut Visibility)>,
    actor_gates: Query<&ActorEmitter>,
    nodes: Query<&GlobalTransform>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((camera, frustum)) = camera.single() else {
        return;
    };
    let dt = time.delta_secs().min(0.1);
    let now = time.elapsed_secs();
    for (mut instance, parent, global, visible, in_view, mut visibility) in &mut instances {
        if let Some(range) = instance.range {
            let far = global.translation().distance_squared(camera.translation()) > range * range;
            visibility.set_if_neq(if far { Visibility::Hidden } else { Visibility::Inherited });
            if far {
                instance.last = None;
                continue;
            }
        }
        let reach = Sphere { center: Vec3A::from(global.translation()), radius: PARTICLE_REACH_M };
        if !visible.get() || !in_view.get() || frustum.is_some_and(|f| !f.intersects_sphere(&reach, true)) {
            instance.last = None;
            continue;
        }
        let Some(system) = animations.get(&instance.animation).and_then(|a| a.particles.get(instance.system)) else {
            continue;
        };
        let instance = &mut *instance;
        if !system.local_space && !carry(instance, global.affine()) {
            continue; // collapsed: a hidden actor joint scales its subtree to zero
        }
        let index = u32::try_from(instance.system).unwrap_or(u32::MAX);
        let gated = if instance.actor { actor_gates.get(parent.parent()).ok().and_then(|g| g.emits(index)) } else { None };
        let emitting = gated.unwrap_or_else(|| system.emits_at(now + instance.phase));
        // Live nodes: where the pose puts them now, in this node's space.
        let to_node = global.affine().inverse();
        let place = |l: &LiveNode| nodes.get(l.node).ok().map(|g| to_nif(to_node * g.affine() * l.offset));
        let mut emitter = system.emitter_transform;
        instance.targets.clear();
        if let Some(live) = &instance.live {
            if let Some(at) = live.emitter.as_ref().and_then(place) {
                emitter = at;
            }
            for (rest, (node, parent)) in system.targets.iter().zip(&live.targets) {
                let transform = node.as_ref().and_then(place).unwrap_or(rest.transform);
                let parent_origin = match (parent, rest.parent) {
                    (Some(p), _) => place(p).map_or(rest.parent_origin, |a| a.translation.into()),
                    (None, Some(_)) => rest.parent_origin,
                    (None, None) => transform.translation.into(),
                };
                instance.targets.push(mw_nif::particles::EmitTarget { transform, parent_origin, ..*rest });
            }
        }
        let ParticleInstance { simulation, targets, .. } = &mut *instance;
        simulation.step(system, dt, emitting, emitter, targets);

        // Camera axes in the node's local space.
        let to_local = global.affine().inverse();
        let right = to_local.transform_vector3(camera.right().into()).normalize_or_zero();
        let up = to_local.transform_vector3(camera.up().into()).normalize_or_zero();
        let eye = to_local.transform_point3(camera.translation());
        let toward_camera = right.cross(up).normalize_or_zero();

        let particles = &mut instance.simulation.particles;
        if instance.sort {
            let distance = |p: &nif_glam::Vec3| (Vec3::from_array(p.to_array()) - eye).length_squared();
            particles.sort_by(|a, b| distance(&b.position).total_cmp(&distance(&a.position)));
        }
        let Some(mut mesh) = meshes.get_mut(&instance.mesh) else {
            continue;
        };
        write_quads(&mut mesh, particles, right, up, toward_camera);
    }
}

/// Moves a world-space system's particles by its node's motion since the
/// last step. `false`: the node is degenerate (scale zero) and can't host
/// particles this frame.
fn carry(instance: &mut ParticleInstance, now: bevy::math::Affine3A) -> bool {
    if now.matrix3.determinant().abs() < 1e-9 {
        instance.last = None;
        return false;
    }
    if let Some(before) = instance.last.replace(now)
        && before != now
        && before.translation.distance(now.translation) < CARRY_MAX_STEP_M
    {
        let delta = now.inverse() * before;
        instance.simulation.carry(nif_glam::Affine3A::from_cols_array(&delta.to_cols_array()));
    }
    true
}

fn to_nif(a: bevy::math::Affine3A) -> nif_glam::Affine3A {
    nif_glam::Affine3A::from_cols_array(&a.to_cols_array())
}

fn empty_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    write_quads(&mut mesh, &[], Vec3::X, Vec3::Y, Vec3::Z);
    mesh
}

fn write_quads(mesh: &mut Mesh, particles: &[mw_nif::particles::Particle], right: Vec3, up: Vec3, normal: Vec3) {
    let n = particles.len();
    let mut positions = Vec::with_capacity(n * 4);
    let mut colors = Vec::with_capacity(n * 4);
    let mut uvs = Vec::with_capacity(n * 4);
    let mut indices = Vec::with_capacity(n * 6);
    for p in particles {
        let center = Vec3::from_array(p.position.to_array());
        let half = p.size;
        let (sin, cos) = p.angle.sin_cos();
        let r = (right * cos + up * sin) * half;
        let u = (up * cos - right * sin) * half;
        let base = u32::try_from(positions.len()).unwrap_or(u32::MAX);
        positions.extend([center - r + u, center + r + u, center + r - u, center - r - u].map(|v| v.to_array()));
        uvs.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend([p.color.to_array(); 4]);
        indices.extend([base, base + 2, base + 1, base, base + 3, base + 2]);
    }
    if positions.is_empty() {
        // One degenerate triangle: GPU buffers can't be empty.
        positions.extend([[0.0; 3]; 3]);
        uvs.extend([[0.0; 2]; 3]);
        colors.extend([[0.0; 4]; 3]);
        indices.extend([0, 1, 2]);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal.to_array(); positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
}

#[cfg(test)]
mod tests {
    use super::*;
    use mw_nif::particles::{EmitTarget, ParticleSystem};

    #[test]
    fn keyframed_emitters_follow_their_tagged_nodes() {
        let target = EmitTarget { node: 4, parent: Some(3), transform: nif_glam::Affine3A::IDENTITY, parent_origin: nif_glam::Vec3::ZERO };
        let moving = ParticleSystem { emitter: 2, targets: vec![target], animated_emitter: true, ..Default::default() };
        let still = ParticleSystem { emitter: 7, ..Default::default() };
        let model = mw_nif::Model { particles: vec![moving.clone(), still], ..Default::default() };
        assert_eq!(live_nodes(&model), [2, 3, 4], "only animated systems' nodes are tagged");

        let mut world = World::new();
        let [emitter, parent] = [world.spawn_empty().id(), world.spawn_empty().id()];
        let live = static_live(0, &moving, &[(2, emitter), (3, parent)]);
        assert_eq!(live.emitter.map(|l| l.node), Some(emitter));
        assert_eq!(live.targets.len(), 1);
        assert!(live.targets[0].0.is_none(), "untagged target: rest pose");
        assert_eq!(live.targets[0].1.map(|l| l.node), Some(parent));
    }
}
