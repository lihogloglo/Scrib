//! NIF particle systems in Bevy: each emitter instance runs its own
//! [`mw_nif::particles::Simulation`] on the CPU and the GPU draws its
//! particles as camera-facing quads pulled from a shared buffer.
//!
//! CPU simulation over a GPU particle crate: vanilla emitters hold tens of
//! particles, the NIF semantics (box emitters, grow/fade, color keys) stay
//! exact and unit-tested in a Bevy-free crate, and there's no third-party
//! renderer to track across Bevy releases (`bevy_hanabi` isn't on 0.20).
//!
//! Rendering is vertex pulling, as Bevy 0.20's own `storage_buffer` example
//! (and UE Niagara's CPU-sim sprite renderer) does it: every emitter owns a
//! slot of one [`ShaderBuffer`] ([`ParticleBuffers`]); its [`MeshTag`] is the
//! slot's first index; its mesh is a static, shared quad list; the vertex
//! shader (`shaders/particles.wesl`) places and turns each quad toward the
//! view. Each frame only the buffer's contents change: one upload, no `Mesh`
//! asset churn (rewriting a mesh per emitter per frame made Bevy rescan every
//! `Mesh3d` and reallocate the mesh slabs: ~1 ms on the Balmora walk,
//! perf.md "Audit").

use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::{Frustum, Sphere};
use bevy::camera::visibility::NoFrustumCulling;
use bevy::math::Vec3A;
use bevy::mesh::{Indices, MeshTag, PrimitiveTopology};
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::storage::ShaderBuffer;
use bevy::shader::ShaderRef;
use mw_nif::glam as nif_glam;
use mw_nif::particles::Simulation;
use sf_render_order_bevy::TranslucentFog;

use crate::animation::{ModelAnimation, NifAnimation, seed_unit};

const SHADER: &str = "embedded://mw_assets/shaders/particles.wesl";

/// A particle system's `StandardMaterial` drawn from the particle buffer and
/// fogged like `sf_atmos_bevy::FroxelFogMaterial` (render-order.md rule 1).
pub type ParticleMaterial = ExtendedMaterial<StandardMaterial, ParticleExt>;

/// The particle buffer and fog bindings of [`ParticleMaterial`] (bindings
/// 110-113 of the material group; 110-112 as `FroxelFogExt`).
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
pub struct ParticleExt {
    /// [`TranslucentFog::params`]; kept current by [`sync_materials`].
    #[uniform(110)]
    fog_params: Vec4,
    #[texture(111, dimension = "3d")]
    #[sampler(112)]
    fog_volume: Handle<Image>,
    /// [`ParticleBuffers::buffer`].
    #[storage(113, read_only)]
    particles: Handle<ShaderBuffer>,
}

impl MaterialExtension for ParticleExt {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    /// Translucents cast no shadows (render-order.md rule 4).
    fn enable_shadows() -> bool {
        false
    }

    /// Always blended (see [`particle_base`]): no depth prepass, whose
    /// default vertex shader couldn't place the quads.
    fn enable_prepass() -> bool {
        false
    }
}

/// One particle in the buffer (`Particle` in the shader, 48 bytes): node-space
/// center and radius, linear RGBA, angle and padding, as raw f32 bits.
type GpuParticle = [u32; 12];

/// Particles per buffer growth step (48 KB).
const BUFFER_CHUNK: usize = 1024;

/// The GPU side of every emitter: one shared particle buffer carved into
/// per-instance slots, the quad mesh per slot size, and a [`ParticleMaterial`]
/// per source material.
#[derive(Resource)]
pub(crate) struct ParticleBuffers {
    buffer: Handle<ShaderBuffer>,
    /// The buffer's contents, copied into the asset when `dirty`.
    data: Vec<GpuParticle>,
    /// End of the last slot ever handed out (`data` grows in chunks past it).
    end: u32,
    dirty: bool,
    /// Freed slots' first indices, by slot size.
    free: HashMap<u32, Vec<u32>>,
    meshes: HashMap<u32, Handle<Mesh>>,
    materials: HashMap<AssetId<StandardMaterial>, Handle<ParticleMaterial>>,
}

impl FromWorld for ParticleBuffers {
    fn from_world(world: &mut World) -> Self {
        let data = vec![[0; 12]; BUFFER_CHUNK];
        let buffer = world.resource_mut::<Assets<ShaderBuffer>>().add(ShaderBuffer::new(data.clone(), RenderAssetUsages::default()));
        Self { buffer, data, end: 0, dirty: false, free: HashMap::new(), meshes: HashMap::new(), materials: HashMap::new() }
    }
}

/// A run of [`ParticleBuffers::data`] owned by one emitter instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Slot {
    first: u32,
    size: u32,
}

impl ParticleBuffers {
    /// A cleared slot for `max` particles (sizes are powers of two, so slots
    /// and meshes are shared).
    fn allocate(&mut self, max: usize) -> Slot {
        let size = u32::try_from(max.max(1).next_power_of_two()).unwrap_or(u32::MAX);
        let first = if let Some(first) = self.free.get_mut(&size).and_then(Vec::pop) {
            first
        } else {
            let first = self.end;
            self.end += size;
            if self.end as usize > self.data.len() {
                // Grows in chunks; Bevy rebinds the materials to the new buffer.
                self.data.resize((self.end as usize).next_multiple_of(BUFFER_CHUNK), [0; 12]);
            }
            first
        };
        self.data[first as usize..(first + size) as usize].fill([0; 12]);
        self.dirty = true;
        Slot { first, size }
    }

    fn release(&mut self, slot: Slot) {
        self.free.entry(slot.size).or_default().push(slot.first);
    }

    /// Writes `particles` into `slot`, clearing the ones left over from the
    /// last write (`written`).
    fn write(&mut self, slot: Slot, particles: &[mw_nif::particles::Particle], written: &mut u32) {
        let out = &mut self.data[slot.first as usize..(slot.first + slot.size) as usize];
        let n = particles.len().min(out.len());
        for (out, p) in out.iter_mut().zip(&particles[..n]) {
            let mut gpu = [0.0f32; 12];
            gpu[..3].copy_from_slice(&p.position.to_array());
            gpu[3] = p.size;
            gpu[4..8].copy_from_slice(&p.color.to_array());
            gpu[8] = p.angle;
            *out = gpu.map(f32::to_bits);
        }
        let stale = (*written as usize).min(out.len());
        if stale > n {
            out[n..stale].fill([0; 12]);
        }
        *written = u32::try_from(n).unwrap_or(u32::MAX);
        self.dirty = true;
    }

    /// The shared quad mesh for slots of `size` particles.
    fn mesh(&mut self, size: u32, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        self.meshes.entry(size).or_insert_with(|| meshes.add(quad_mesh(size))).clone()
    }

    /// The [`ParticleMaterial`] drawing `source`.
    fn material(
        &mut self,
        source: &Handle<StandardMaterial>,
        standard: &Assets<StandardMaterial>,
        fog: &TranslucentFog,
        materials: &mut Assets<ParticleMaterial>,
    ) -> Option<Handle<ParticleMaterial>> {
        if let Some(handle) = self.materials.get(&source.id()) {
            return Some(handle.clone());
        }
        let base = particle_base(standard.get(source)?);
        let extension = ParticleExt { fog_params: fog.params, fog_volume: fog.volume.clone(), particles: self.buffer.clone() };
        let handle = materials.add(ParticleMaterial { base, extension });
        self.materials.insert(source.id(), handle.clone());
        Some(handle)
    }
}

/// A particle system's material as drawn: always blended. NIF particles are
/// blended or additive; the rare alpha-tested or opaque one blends too (the
/// quads have no depth prepass, and the fog composite would fog an opaque
/// one a second time).
fn particle_base(source: &StandardMaterial) -> StandardMaterial {
    let mut base = source.clone();
    if matches!(base.alpha_mode, AlphaMode::Opaque | AlphaMode::Mask(_)) {
        base.alpha_mode = AlphaMode::Blend;
    }
    base
}

/// `size` quads for the shader to place: four vertices each, the corner in
/// `uv`, the particle's index in `color.x`. Positions are all zero, so the
/// bounds (and the translucent sort key) sit at the particle node.
fn quad_mesh(size: u32) -> Mesh {
    let n = size as usize;
    let mut uvs = Vec::with_capacity(n * 4);
    let mut colors = Vec::with_capacity(n * 4);
    let mut indices = Vec::with_capacity(n * 6);
    for i in 0..size {
        let base = i * 4;
        #[allow(clippy::cast_precision_loss)] // sizes stay far below 2^24
        let index = i as f32;
        uvs.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend([[index, 0.0, 0.0, 0.0]; 4]);
        indices.extend([base, base + 2, base + 1, base, base + 3, base + 2]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0f32; 3]; n * 4]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Copies the particle buffer into its asset when an emitter wrote to it.
pub(crate) fn upload_particles(mut buffers: ResMut<ParticleBuffers>, mut assets: ResMut<Assets<ShaderBuffer>>) {
    if !buffers.dirty {
        return;
    }
    buffers.dirty = false;
    if let Some(mut asset) = assets.get_mut(&buffers.buffer) {
        asset.clear();
        asset.extend_from_slice(&buffers.data);
    }
}

/// Keeps every [`ParticleMaterial`] matching its source (NIF material
/// controllers animate alpha, colour and UVs) and the fog volume.
pub(crate) fn sync_materials(
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    fog: Res<TranslucentFog>,
    buffers: Res<ParticleBuffers>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id } = event
            && let (Some(handle), Some(source)) = (buffers.materials.get(id), standard.get(*id))
            && let Some(mut m) = materials.get_mut(handle)
        {
            m.base = particle_base(source);
        }
    }
    if fog.is_changed() {
        for handle in buffers.materials.values() {
            if let Some(mut m) = materials.get_mut(handle) {
                m.extension.fog_params = fog.params;
                m.extension.fog_volume = fog.volume.clone();
            }
        }
    }
}

/// Frees a despawned emitter instance's slot.
pub(crate) fn release_slot(remove: On<Remove<ParticleInstance>>, instances: Query<&ParticleInstance>, mut buffers: ResMut<ParticleBuffers>) {
    if let Ok(instance) = instances.get(remove.entity) {
        buffers.release(instance.slot);
    }
}

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
    /// Its run of the particle buffer, and how many particles it last wrote.
    slot: Slot,
    written: u32,
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
    standard: Res<Assets<StandardMaterial>>,
    fog: Res<TranslucentFog>,
    mut buffers: ResMut<ParticleBuffers>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, emitter, range, live, gate) in &emitters {
        let actor = gate.is_some();
        let Some(animation) = animations.get(&emitter.animation) else {
            continue;
        };
        let source = |index: &u32| animation.particles.get(*index as usize).and_then(|s| animation.materials.get(s.material));
        if emitter.systems.iter().filter_map(source).any(|m| !standard.contains(m)) {
            continue; // a material still loading
        }
        commands.entity(entity).try_insert(ParticlesStarted);
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
            if (!system.auto_play && !actor) || system.max_particles == 0 {
                continue; // driven by gameplay (spell effects): not yet
            }
            let Some(material) = animation.materials.get(system.material).and_then(|m| buffers.material(m, &standard, &fog, &mut materials))
            else {
                continue;
            };
            let sort = materials.get(&material).is_some_and(|m| matches!(m.base.alpha_mode, AlphaMode::Blend));
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
            let slot = buffers.allocate(system.max_particles);
            let mesh = buffers.mesh(slot.size, &mut meshes);
            commands.spawn((
                ParticleInstance {
                    animation: emitter.animation.clone(),
                    system: index as usize,
                    simulation,
                    slot,
                    written: 0,
                    phase,
                    sort,
                    last: None,
                    range: range.map(|r| r.0),
                    live,
                    actor,
                    targets: Vec::new(),
                },
                Mesh3d(mesh),
                MeshMaterial3d(material),
                MeshTag::new(slot.first),
                // The quads' bounds are a point (the shader places them).
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
/// step.
const PARTICLE_REACH_M: f32 = 12.0;

/// Steps every emitter the camera can see and writes its particles into its
/// buffer slot. Emitters outside the view frustum or their visibility range
/// (`ViewVisibility`: particles opt out of frustum culling, so it only carries
/// the range) pause.
pub(crate) fn simulate_particles(
    time: Res<Time>,
    animations: Res<Assets<NifAnimation>>,
    camera: Query<(&GlobalTransform, Option<&Frustum>), With<Camera3d>>,
    mut instances: Query<(&mut ParticleInstance, &ChildOf, &GlobalTransform, &InheritedVisibility, &ViewVisibility, &mut Visibility)>,
    actor_gates: Query<&ActorEmitter>,
    nodes: Query<&GlobalTransform>,
    mut buffers: ResMut<ParticleBuffers>,
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

        let particles = &mut instance.simulation.particles;
        if instance.sort {
            // Back to front within the emitter (the shader keeps the order).
            let eye = to_node.transform_point3(camera.translation());
            let distance = |p: &nif_glam::Vec3| (Vec3::from_array(p.to_array()) - eye).length_squared();
            particles.sort_by(|a, b| distance(&b.position).total_cmp(&distance(&a.position)));
        }
        buffers.write(instance.slot, particles, &mut instance.written);
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

#[cfg(test)]
mod tests {
    use super::*;
    use mw_nif::particles::{EmitTarget, Particle, ParticleSystem};

    #[test]
    fn slots_share_sizes_reuse_freed_runs_and_clear_stale_particles() {
        let mut buffers = ParticleBuffers {
            buffer: Handle::default(),
            data: Vec::new(),
            end: 0,
            dirty: false,
            free: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
        };
        let a = buffers.allocate(20);
        let b = buffers.allocate(3);
        assert_eq!((a, b), (Slot { first: 0, size: 32 }, Slot { first: 32, size: 4 }));
        assert_eq!(buffers.data.len(), BUFFER_CHUNK, "grows in chunks");

        let p = |x: f32| Particle {
            position: nif_glam::Vec3::new(x, 2.0, 3.0),
            velocity: nif_glam::Vec3::ZERO,
            age: 0.0,
            lifetime: 1.0,
            size: 0.5,
            color: nif_glam::Vec4::new(0.1, 0.2, 0.3, 0.4),
            angle: 1.5,
        };
        let mut written = 0;
        buffers.write(b, &[p(1.0), p(7.0)], &mut written);
        assert_eq!(written, 2);
        assert_eq!(buffers.data[33].map(f32::from_bits)[..9], [7.0, 2.0, 3.0, 0.5, 0.1, 0.2, 0.3, 0.4, 1.5]);
        buffers.write(b, &[p(1.0)], &mut written);
        assert_eq!(buffers.data[33], [0; 12], "a particle that died draws nothing");

        buffers.release(b);
        let c = buffers.allocate(4);
        assert_eq!(c, b, "a freed slot of the same size is reused");
        assert_eq!(buffers.data[32], [0; 12], "and cleared");
        assert_eq!(buffers.allocate(2000).first, 36);
        assert_eq!(buffers.data.len(), 3 * BUFFER_CHUNK);
    }

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
