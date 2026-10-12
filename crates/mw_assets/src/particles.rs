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
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::math::Vec3A;
use bevy::prelude::*;
use mw_nif::glam as nif_glam;
use mw_nif::particles::Simulation;

use crate::animation::{NifAnimation, seed_unit};

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
    /// The emitter's [`LiveEmitter`].
    live: Option<LiveEmitter>,
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
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ActorEmitter;

/// On a [`ParticleEmitter`]: its systems emit from `node` (a skeleton joint)
/// at `offset`, wherever the pose puts it, instead of from the authored
/// rest emitter transform.
#[derive(Component, Clone, Copy, Debug)]
pub struct LiveEmitter {
    pub node: Entity,
    pub offset: bevy::math::Affine3A,
}

/// A node moving farther than this in one step teleported (or the floating
/// origin shifted): its world-space particles move with it.
const CARRY_MAX_STEP_M: f32 = 4.0;

#[derive(Component)]
pub(crate) struct ParticlesStarted;

/// Spawns one simulated child per particle system once the model's
/// animation asset is available.
#[allow(clippy::type_complexity)]
pub(crate) fn start_particles(
    mut commands: Commands,
    emitters: Query<(Entity, &ParticleEmitter, Option<&ParticleRange>, Option<&LiveEmitter>, Has<ActorEmitter>), Without<ParticlesStarted>>,
    animations: Res<Assets<NifAnimation>>,
    materials: Res<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, emitter, range, live, actor) in &emitters {
        let Some(animation) = animations.get(&emitter.animation) else { continue };
        commands.entity(entity).insert(ParticlesStarted);
        for &index in &emitter.systems {
            let Some(system) = animation.particles.get(index as usize) else { continue };
            if !system.auto_play && !actor {
                continue; // driven by gameplay (spell effects): not yet
            }
            let Some(material) = animation.materials.get(system.material) else { continue };
            let sort = materials.get(material).is_some_and(|m| matches!(m.alpha_mode, AlphaMode::Blend));
            let mut simulation = Simulation::new(u64::from(index) ^ entity.to_bits());
            simulation.prewarm(system, system.emitter_transform);
            let mesh = meshes.add(empty_mesh());
            commands.spawn((
                ParticleInstance {
                    animation: emitter.animation.clone(),
                    system: index as usize,
                    simulation,
                    mesh: mesh.clone(),
                    phase: seed_unit(entity.to_bits()) * 100.0,
                    sort,
                    last: None,
                    range: range.map(|r| r.0),
                    live: live.copied(),
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
    mut instances: Query<(&mut ParticleInstance, &GlobalTransform, &InheritedVisibility, &ViewVisibility, &mut Visibility)>,
    nodes: Query<&GlobalTransform>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((camera, frustum)) = camera.single() else { return };
    let dt = time.delta_secs().min(0.1);
    let now = time.elapsed_secs();
    for (mut instance, global, visible, in_view, mut visibility) in &mut instances {
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
        let key_time = system.timing.key_time(now + instance.phase);
        // A live emitter: its joint now, in this node's space.
        let emitter = instance
            .live
            .and_then(|l| nodes.get(l.node).ok().map(|g| global.affine().inverse() * g.affine() * l.offset))
            .map_or(system.emitter_transform, |a| nif_glam::Affine3A::from_cols_array(&a.to_cols_array()));
        instance.simulation.step(system, dt, key_time, emitter);

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
        let Some(mut mesh) = meshes.get_mut(&instance.mesh) else { continue };
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
