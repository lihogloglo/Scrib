//! Morrowind NIF models as an engine-neutral [`Model`].
//!
//! The conversion resolves what the NIF format leaves implicit:
//! - properties inherit down the scene graph (a node's texturing, material,
//!   alpha, ... apply to its subtree until overridden),
//! - geometry is converted to engine space (Y up, meters; see [`coords`]),
//! - render visibility: app-culled nodes, editor markers, collision
//!   subtrees, and inactive switch children are kept but marked,
//! - materials are deduplicated per model (equal property sets share one,
//!   unless different controllers animate them),
//! - controllers become engine-space animation tracks ([`anim`]) and
//!   particle emitters ([`particles`]), with runtime evaluation for both.
//!
//! Engines turn a [`Model`] into their own meshes, materials, and entities;
//! nothing here depends on an engine.

pub mod anim;
pub mod census;
pub mod coords;
pub mod kf;
mod material;
pub mod particles;
pub mod skin;

pub use kf::{KeyframeFile, load_kf};
pub use material::{AlphaMode, Material, VertexColorMode, texture_path};
/// The math types in this crate's API (glam, as used by `tes3`).
pub use tes3::nif::glam;

use std::collections::HashMap;

use tes3::nif::glam::{Affine3A, Vec2, Vec3, Vec4};
use tes3::nif::{
    NiAVObject, NiAlphaController, NiBSAnimationNode, NiBSParticleNode, NiBillboardNode, NiGeometry, NiGeometryData,
    NiBSPArrayController, NiKey, NiKeyframeController, NiLink, NiMaterialColorController, NiNode, NiObjectNET, NiParticleSystemController,
    NiParticles, NiParticlesData, NiProperty, NiStream, NiStringExtraData, NiSwitchNode, NiTimeController,
    NiTextKeyExtraData, NiTriShapeData, NiTriStripsData, NiType, NiUVController, NiVisController, RootCollisionNode,
};

use anim::{ColorSlot, Controller, Steps, Target, Timing, UvTrack};
use coords::Trs;
use particles::ParticleSystem;

/// A converted model: a node tree plus the meshes and materials it uses.
#[derive(Clone, Debug, Default)]
pub struct Model {
    /// Parents always precede their children.
    pub nodes: Vec<Node>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    /// Animation channels (keyframes, visibility, material animation).
    pub controllers: Vec<Controller>,
    pub particles: Vec<ParticleSystem>,
    /// Root carries `MRK` extra data: an editor-only marker model.
    pub marker: bool,
    /// Root carries an `NC…` string marker (case-insensitive prefix, as the
    /// original engine and `OpenMW` read it): actors don't collide with the
    /// model. `NCC` keeps camera-only collision, anything else (`NCO`) none
    /// at all. See [`no_collision_marker`].
    pub no_collision: bool,
    /// Geometry had skinning; it was baked into its bind pose.
    pub had_skin: bool,
    /// Root text keys `(time, "Group: Event")`, one entry per line.
    pub text_keys: Vec<(f32, String)>,
    /// The first root node's authored transform (engine space), which
    /// conversion drops from [`Self::nodes`]: a placed object's root takes
    /// the reference's transform instead, but a part bolted to a bone keeps
    /// it under the bone (e.g. Morrowind's rigid pants ankle and knee parts
    /// sit 7 to 37 cm along the leg from their bones).
    pub root_transform: Trs,
}

impl Model {
    /// Key-time range of an animation group (`"Idle"`) from the text keys.
    pub fn group_range(&self, group: &str) -> Option<(f32, f32)> {
        let event_time = |event: &str| {
            self.text_keys.iter().find_map(|(time, line)| {
                let (name, what) = line.split_once(':')?;
                (name.trim().eq_ignore_ascii_case(group) && what.trim().eq_ignore_ascii_case(event)).then_some(*time)
            })
        };
        Some((event_time("start")?, event_time("stop")?))
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub parent: Option<usize>,
    /// Transform relative to the parent, in engine space.
    pub transform: Trs,
    pub kind: NodeKind,
    /// Whether this node renders (combined with its ancestors).
    pub visible: bool,
    /// Index into [`Model::meshes`] for geometry nodes.
    pub mesh: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Node,
    Geometry,
    /// Rotates to face the camera.
    Billboard,
    /// Shows only the child at `active` (an index into its children).
    Switch { active: usize },
    /// Root of collision-only geometry.
    Collision,
    /// Particle geometry; see [`Model::particles`] for its emitters.
    Particles,
}

/// Triangle mesh in engine space.
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub positions: Vec<[f32; 3]>,
    /// Empty when the NIF has none (engines should compute them).
    pub normals: Vec<[f32; 3]>,
    /// First UV set; empty if absent.
    pub uvs: Vec<[f32; 2]>,
    /// Linear RGBA; empty if absent.
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub material: usize,
    /// True for collision geometry (never rendered).
    pub collision: bool,
    /// The shape was skinned: [`Self::positions`] and [`Self::normals`] are
    /// its baked rest pose.
    pub skinned: bool,
    /// Skin data of a skinned shape, when its skin data and root resolved.
    pub skin: Option<Box<skin::Skin>>,
    /// Geometry morphs; unskinned [`Self::positions`] are morph target 0.
    pub morph: Option<skin::Morph>,
}

#[derive(Debug)]
pub enum ModelError {
    Parse(std::io::Error),
    NoRoots,
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "invalid NIF: {e}"),
            Self::NoRoots => f.write_str("NIF has no root objects"),
        }
    }
}

impl std::error::Error for ModelError {}

/// Parses and converts NIF bytes.
pub fn load(bytes: &[u8]) -> Result<Model, ModelError> {
    convert(parse(bytes)?)
}

/// Parses NIF bytes without converting them.
pub fn parse(bytes: &[u8]) -> Result<NiStream, ModelError> {
    NiStream::from_bytes(bytes).map_err(ModelError::Parse)
}

/// Converts a parsed NIF stream.
pub fn convert(mut stream: NiStream) -> Result<Model, ModelError> {
    if stream.roots.is_empty() {
        return Err(ModelError::NoRoots);
    }
    let had_skin = stream.objects_of_type::<NiGeometry>().any(|g| !g.skin_instance.is_null());
    let raw_skins = if had_skin { skin::raw_skins(&stream) } else { HashMap::new() };
    if had_skin {
        stream.apply_skin_deforms();
    }

    let mut builder = Builder {
        stream: &stream,
        model: Model { had_skin, ..Model::default() },
        material_index: HashMap::new(),
        node_index: HashMap::new(),
        emitters: Vec::new(),
        raw_skins,
        skins: Vec::new(),
    };
    for root in stream.roots.clone() {
        if let Some(av) = stream.get_as::<_, NiAVObject>(root) {
            builder.read_root_extra_data(av.base.extra_data);
        }
        builder.visit(root.key, None, &Inherited::default(), true, false);
    }
    // Emitters may be anywhere in the graph, so resolve them last.
    for (system, emitter) in std::mem::take(&mut builder.emitters) {
        if let Some(&node) = builder.node_index.get(&emitter) {
            builder.model.particles[system].emitter = node;
        }
    }
    let Builder { mut model, node_index, raw_skins, skins, .. } = builder;
    // The engine makes a model's root node the placed object's node and
    // overwrites its transform with the reference's. Authored root
    // transforms (e.g. a 180° turn on in_nord_fireplace_01) never show.
    // A part bolted to a bone keeps it: see [`Model::root_transform`].
    model.root_transform = model.nodes.iter().find(|n| n.parent.is_none()).map_or(Trs::IDENTITY, |n| n.transform);
    for node in model.nodes.iter_mut().filter(|n| n.parent.is_none()) {
        node.transform = Trs::IDENTITY;
    }
    let world = world_transforms(&model);
    for (mesh, node, shape, base) in skins {
        let Some(raw) = raw_skins.get(&shape) else { continue };
        let node_of = |key: NiKey| node_index.get(&key).copied();
        model.meshes[mesh].skin = skin::resolve(&stream, raw, base.as_deref(), node, &world, node_of).map(Box::new);
    }
    resolve_emitters(&mut model, &world);
    loop_idle_animation(&mut model);
    Ok(model)
}

/// Rest emitter transforms relative to each particle node, the nodes of
/// array emitters ([`particles::EmitFrom::Nodes`]: the emitter and every
/// node below it), and whether the model's keyframes move any of them
/// relative to the particle node.
fn resolve_emitters(model: &mut Model, world: &[Affine3A]) {
    let ancestors = |mut i: usize| {
        let mut out = vec![i];
        while let Some(p) = model.nodes[i].parent {
            out.push(p);
            i = p;
        }
        out
    };
    let keyframed: Vec<usize> =
        model.controllers.iter().filter_map(|c| if let Target::Transform { node, .. } = c.target { Some(node) } else { None }).collect();
    let mut resolved = Vec::with_capacity(model.particles.len());
    for system in &model.particles {
        let to_node = world[system.node].inverse();
        let emitter_transform = to_node * world[system.emitter];
        let targets: Vec<particles::EmitTarget> = if matches!(system.emit_from, particles::EmitFrom::Nodes { .. }) {
            (0..model.nodes.len())
                .filter(|&i| matches!(model.nodes[i].kind, NodeKind::Node | NodeKind::Billboard | NodeKind::Switch { .. }))
                .filter(|&i| ancestors(i).contains(&system.emitter))
                .map(|i| {
                    let parent = model.nodes[i].parent;
                    let transform = to_node * world[i];
                    let parent_origin = parent.map_or(transform.translation, |p| (to_node * world[p]).translation);
                    particles::EmitTarget { node: i, parent, transform, parent_origin: parent_origin.into() }
                })
                .collect()
        } else {
            Vec::new()
        };
        // Moving relative to the particle node: a keyframed node on the way
        // from the emitter (or a target, or its parent) up to an ancestor of
        // the particle node.
        let fixed = ancestors(system.node);
        let moves = |i: usize| ancestors(i).iter().take_while(|a| !fixed.contains(a)).any(|a| keyframed.contains(a));
        let animated = moves(system.emitter) || targets.iter().any(|t| moves(t.node) || t.parent.is_some_and(moves));
        resolved.push((emitter_transform, targets, animated));
    }
    for (system, (emitter_transform, targets, animated)) in model.particles.iter_mut().zip(resolved) {
        system.emitter_transform = emitter_transform;
        system.targets = targets;
        system.animated_emitter = animated;
    }
}

/// Objects (banners, Dwemer machinery) animate by looping the `Idle`
/// group of their own keyframes; the original engine plays it for any
/// animated non-actor object. Skinned models are actors, animated by
/// gameplay instead.
fn loop_idle_animation(model: &mut Model) {
    if model.had_skin {
        return;
    }
    let Some((start, stop)) = model.group_range("idle").filter(|(start, stop)| stop > start) else { return };
    for controller in &mut model.controllers {
        if !controller.auto_play && matches!(controller.target, Target::Transform { .. }) {
            controller.auto_play = true;
            controller.timing.start = start;
            controller.timing.stop = stop;
            controller.timing.cycle = anim::Cycle::Loop;
        }
    }
}

/// Properties in effect at a point of the graph.
#[derive(Clone, Copy, Default)]
struct Inherited {
    texturing: Option<NiKey>,
    material: Option<NiKey>,
    alpha: Option<NiKey>,
    vertex_color: Option<NiKey>,
    stencil: Option<NiKey>,
    zbuffer: Option<NiKey>,
    /// Set by the nearest animation node (`NiBSAnimationNode`,
    /// `NiBSParticleNode`) above: whether its controllers play by themselves.
    auto_play: Option<bool>,
    random_start: bool,
    /// Set by the nearest `NiBSParticleNode` above: particles live in the
    /// node's space instead of the world (flag 0x80).
    local_space: bool,
}

/// Animation-node flags (`NiBSAnimationNode`, `NiBSParticleNode`).
const ANIM_FLAG_AUTO_PLAY: u16 = 0x20;
const ANIM_FLAG_NOT_RANDOM: u16 = 0x40;
/// `NiBSParticleNode` flag: particles move with the node (else world space).
const PARTICLE_FLAG_LOCAL_SPACE: u16 = 0x80;
/// `NiBSPArrayController` flag: new particles start between a node and its
/// parent (on the bone), as `OpenMW` reads it (black box). Set on all 42
/// vanilla array controllers (`mw_bake/tests/particles_real_install.rs`).
const BSP_ARRAY_FLAG_AT_NODE: u16 = 0x8;

impl Inherited {
    fn with(mut self, stream: &NiStream, properties: &[NiLink<NiProperty>]) -> Self {
        for link in properties {
            let Some(object) = stream.objects.get(link.key) else { continue };
            let slot = match object {
                NiType::NiTexturingProperty(_) => &mut self.texturing,
                NiType::NiMaterialProperty(_) => &mut self.material,
                NiType::NiAlphaProperty(_) => &mut self.alpha,
                NiType::NiVertexColorProperty(_) => &mut self.vertex_color,
                NiType::NiStencilProperty(_) => &mut self.stencil,
                NiType::NiZBufferProperty(_) => &mut self.zbuffer,
                _ => continue,
            };
            *slot = Some(link.key);
        }
        self
    }
}

struct Builder<'a> {
    stream: &'a NiStream,
    model: Model,
    /// Keyed by the material plus what animates it: the animated material
    /// property and the geometry carrying a UV controller, if any.
    material_index: HashMap<(Material, Option<NiKey>, Option<NiKey>), usize>,
    /// NIF object -> node index.
    node_index: HashMap<NiKey, usize>,
    /// (particle system, emitter object) pairs to resolve after the walk.
    emitters: Vec<(usize, NiKey)>,
    /// Skinned shapes' data from before the skin was baked.
    raw_skins: HashMap<NiKey, skin::RawSkin>,
    /// `(mesh, its node, shape object, morph base)`: skins to resolve after the walk.
    skins: Vec<(usize, usize, NiKey, Option<Vec<Vec3>>)>,
}

impl Builder<'_> {
    fn read_root_extra_data(&mut self, first: NiLink<tes3::nif::NiExtraData>) {
        let mut next = first;
        let mut guard = 0;
        while !next.is_null() && guard < 64 {
            guard += 1;
            let Some(object) = self.stream.objects.get(next.key) else { break };
            if let Ok(text_keys) = <&NiTextKeyExtraData>::try_from(object) {
                self.model.text_keys.extend(kf::text_key_lines(text_keys));
            }
            if let Ok(text) = <&NiStringExtraData>::try_from(object) {
                let value = text.value.trim_start();
                self.model.marker |= value.starts_with("MRK");
                self.model.no_collision |= no_collision_marker(value);
            }
            let Ok(extra) = <&tes3::nif::NiExtraData>::try_from(object) else { break };
            next = extra.next;
        }
    }

    fn visit(&mut self, key: NiKey, parent: Option<usize>, inherited: &Inherited, visible: bool, collision: bool) {
        let Some(object) = self.stream.objects.get(key) else { return };
        let Ok(av) = <&NiAVObject>::try_from(object) else { return };

        let name = av.base.name.clone();
        let mut inherited = inherited.with(self.stream, &av.properties);
        if <&NiBSAnimationNode>::try_from(object).is_ok() || <&NiBSParticleNode>::try_from(object).is_ok() {
            inherited.auto_play = Some(av.flags & ANIM_FLAG_AUTO_PLAY != 0);
            inherited.random_start = av.flags & ANIM_FLAG_NOT_RANDOM == 0;
        }
        if <&NiBSParticleNode>::try_from(object).is_ok() {
            inherited.local_space = av.flags & PARTICLE_FLAG_LOCAL_SPACE != 0;
        }
        let is_collision = collision || <&RootCollisionNode>::try_from(object).is_ok();
        let editor_marker = name.to_ascii_lowercase().starts_with("tri editormarker");
        let visible = visible && !av.app_culled() && !is_collision && !editor_marker;
        let transform = Trs::from_affine(coords::affine(av.transform()));
        let index = self.model.nodes.len();
        self.node_index.insert(key, index);

        if let Ok(particles) = <&NiParticles>::try_from(object) {
            let found = self.particle_systems(index, &particles.base, &inherited);
            let kind = NodeKind::Particles;
            self.model.nodes.push(Node { name, parent, transform, kind, visible: visible && found, mesh: None });
            self.node_controllers(index, &av.base, &inherited);
            return;
        }

        if let Ok(geometry) = <&NiGeometry>::try_from(object) {
            let mesh = self.mesh(key, index, geometry, &inherited, is_collision);
            self.model.nodes.push(Node {
                name,
                parent,
                transform,
                kind: NodeKind::Geometry,
                visible: visible && mesh.is_some(),
                mesh,
            });
            self.node_controllers(index, &av.base, &inherited);
            return;
        }

        let Ok(node) = <&NiNode>::try_from(object) else { return };
        let kind = if <&RootCollisionNode>::try_from(object).is_ok() {
            NodeKind::Collision
        } else if let Ok(switch) = <&NiSwitchNode>::try_from(object) {
            NodeKind::Switch { active: switch.active_index }
        } else if <&NiBillboardNode>::try_from(object).is_ok() {
            NodeKind::Billboard
        } else {
            NodeKind::Node
        };
        self.model.nodes.push(Node { name, parent, transform, kind: kind.clone(), visible, mesh: None });
        self.node_controllers(index, &av.base, &inherited);

        for (i, child) in node.children.iter().enumerate() {
            let child_visible = match kind {
                NodeKind::Switch { active } => visible && i == active,
                _ => visible,
            };
            self.visit(child.key, Some(index), &inherited, child_visible, is_collision);
        }
    }

    fn mesh(&mut self, key: NiKey, node: usize, geometry: &NiGeometry, inherited: &Inherited, collision: bool) -> Option<usize> {
        let object = self.stream.objects.get(geometry.geometry_data.key)?;
        let data = <&NiGeometryData>::try_from(object).ok()?;

        let indices: Vec<u32> = if let Ok(shape) = <&NiTriShapeData>::try_from(object) {
            shape.triangles.iter().flatten().map(|&i| u32::from(i)).collect()
        } else if let Ok(strips) = <&NiTriStripsData>::try_from(object) {
            strip_triangles(&strips.strip_lengths, &strips.strips)
        } else {
            return None; // particles, lines: not triangle meshes
        };
        let vertex_count = data.vertices.len();
        if vertex_count == 0 || indices.is_empty() || indices.iter().any(|&i| i as usize >= vertex_count) {
            return None;
        }

        let material = self.material(inherited, Some((key, geometry)));
        let skinned = self.raw_skins.contains_key(&key);
        let (morph, morph_base) = skin::morph(self.stream, geometry, vertex_count).unzip();
        // Morphs blend from target 0 (as `OpenMW`'s `MorphGeometry` does); a
        // skinned shape's positions are its baked rest pose instead.
        let base_positions = morph_base.as_ref().filter(|_| !skinned).unwrap_or(&data.vertices);

        let uvs = if data.uv_sets.len() >= vertex_count { data.uv_sets[..vertex_count].iter().map(Vec2::to_array).collect() } else { Vec::new() };
        let mesh = Mesh {
            positions: base_positions.iter().map(|&v| coords::point(v).to_array()).collect(),
            normals: if data.normals.len() == vertex_count {
                data.normals.iter().map(|&n| coords::direction(n).normalize_or_zero().to_array()).collect()
            } else {
                Vec::new()
            },
            uvs,
            colors: if data.vertex_colors.len() == vertex_count {
                data.vertex_colors.iter().map(Vec4::to_array).collect()
            } else {
                Vec::new()
            },
            indices,
            material,
            collision,
            skinned,
            skin: None,
            morph,
        };
        if skinned {
            self.skins.push((self.model.meshes.len(), node, key, morph_base));
        }
        self.model.meshes.push(mesh);
        Some(self.model.meshes.len() - 1)
    }

    /// The material for the inherited properties, shared when equal and
    /// equally animated. Creates its material controllers on first use.
    fn material(&mut self, inherited: &Inherited, geometry: Option<(NiKey, &NiGeometry)>) -> usize {
        let stream = self.stream;
        let property = inherited.material.and_then(|k| stream.objects.get(k)).and_then(|o| <&NiObjectNET>::try_from(o).ok());
        let property_animated = property.is_some_and(|p| {
            p.controllers(stream).any(|c| c.is_instance_of::<NiAlphaController>() || c.is_instance_of::<NiMaterialColorController>())
        });
        let uv = geometry.and_then(|(key, g)| g.base.base.controllers_of_type::<NiUVController>(stream).next().map(|c| (key, c)));

        let material = material::from_properties(stream, inherited);
        let index_key = (material.clone(), inherited.material.filter(|_| property_animated), uv.map(|(k, _)| k));
        if let Some(&index) = self.material_index.get(&index_key) {
            return index;
        }
        self.model.materials.push(material);
        let index = self.model.materials.len() - 1;
        self.material_index.insert(index_key, index);

        if let (Some(property), true) = (property, property_animated) {
            for controller in property.controllers(stream) {
                if let Ok(alpha) = <&NiAlphaController>::try_from(controller) {
                    if let Some(data) = stream.get(alpha.base.data) {
                        let curve = anim::float_curve(&data.keys);
                        self.push_controller(&alpha.base.base, inherited, Target::Alpha { material: index, curve });
                    }
                } else if let Ok(color) = <&NiMaterialColorController>::try_from(controller)
                    && let Some(data) = stream.get(color.data)
                {
                    let slot = match (color.base.flags >> 4) & 0x7 {
                        0 => ColorSlot::Ambient,
                        2 => ColorSlot::Specular,
                        3 => ColorSlot::Emissive,
                        _ => ColorSlot::Diffuse,
                    };
                    let curve = anim::vec3_curve(&data.keys);
                    self.push_controller(&color.base, inherited, Target::MaterialColor { material: index, slot, curve });
                }
            }
        }
        if let Some((_, uv)) = uv
            && let Some(data) = stream.get(uv.data)
        {
            let track = UvTrack {
                offset_u: anim::float_curve(&data.u_offset_data.keys),
                offset_v: anim::float_curve(&data.v_offset_data.keys),
                tiling_u: anim::float_curve(&data.u_tiling_data.keys),
                tiling_v: anim::float_curve(&data.v_tiling_data.keys),
            };
            self.push_controller(&uv.base, inherited, Target::Uv { material: index, track });
        }
        index
    }

    /// Keyframe and visibility controllers on a node.
    fn node_controllers(&mut self, node: usize, net: &NiObjectNET, inherited: &Inherited) {
        let stream = self.stream;
        for controller in net.controllers(stream) {
            if let Ok(keyframe) = <&NiKeyframeController>::try_from(controller) {
                if let Some(track) = kf::transform_track(stream, keyframe) {
                    self.push_controller(&keyframe.base, inherited, Target::Transform { node, track });
                }
            } else if let Ok(vis) = <&NiVisController>::try_from(controller)
                && let Some(data) = stream.get(vis.data)
            {
                let keys = Steps::new(data.keys.iter().map(|k| (k.time, k.value != 0)));
                self.push_controller(&vis.base, inherited, Target::Visibility { node, keys });
            }
        }
    }

    /// Particle systems driven by controllers on a particles node. Returns
    /// whether any was found.
    fn particle_systems(&mut self, node: usize, geometry: &NiGeometry, inherited: &Inherited) -> bool {
        let stream = self.stream;
        let max_particles = stream
            .get_as::<_, NiParticlesData>(geometry.geometry_data)
            .map_or(0, |data| usize::from(data.num_particles).max(data.base.vertices.len()));
        let mut found = false;
        for object in geometry.base.base.controllers(stream) {
            let Ok(controller) = <&NiParticleSystemController>::try_from(object) else { continue };
            let mut system = ParticleSystem::from_controller(stream, controller, max_particles.max(1));
            if <&NiBSPArrayController>::try_from(object).is_ok() {
                // Emits from every node under the emitter (`resolve_emitters`).
                // Flag 0x10 (at the emitter geometry's vertices) is set on no
                // vanilla file: those emit from the nodes too.
                let along_bones = controller.base.flags & BSP_ARRAY_FLAG_AT_NODE != 0;
                system.emit_from = particles::EmitFrom::Nodes { along_bones };
            }
            system.node = node;
            system.emitter = node;
            system.material = self.material(inherited, None);
            system.auto_play = inherited.auto_play.unwrap_or(true);
            system.random_start = inherited.random_start;
            system.local_space = inherited.local_space;
            if system.timing.stop <= system.timing.start {
                system.timing.stop = system.timing.start + system.max_lifetime().max(1.0);
            }
            let index = self.model.particles.len();
            if !controller.emitter.is_null() {
                self.emitters.push((index, controller.emitter.key));
            }
            self.model.particles.push(system);
            found = true;
        }
        found
    }

    fn push_controller(&mut self, base: &NiTimeController, inherited: &Inherited, target: Target) {
        let mut controller = Controller {
            timing: Timing::from_controller(base),
            // Outside animation nodes, keyframes belong to actor skeletons
            // (driven by animation files); everything else is ambient.
            auto_play: inherited.auto_play.unwrap_or(!matches!(target, Target::Transform { .. })),
            random_start: inherited.random_start,
            target,
        };
        if controller.timing.stop <= controller.timing.start
            && let Some((start, stop)) = controller.key_range()
        {
            controller.timing.start = start;
            controller.timing.stop = stop;
        }
        self.model.controllers.push(controller);
    }
}

/// Triangle strips to a triangle list, alternating winding, without
/// degenerate triangles.
fn strip_triangles(lengths: &[u16], points: &[u16]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut start = 0usize;
    for &len in lengths {
        let len = usize::from(len);
        let Some(strip) = points.get(start..start + len) else { break };
        for i in 2..strip.len() {
            let (a, b, c) = (strip[i - 2], strip[i - 1], strip[i]);
            if a == b || b == c || a == c {
                continue;
            }
            if i % 2 == 0 {
                out.extend([a, b, c].map(u32::from));
            } else {
                out.extend([a, c, b].map(u32::from));
            }
        }
        start += len;
    }
    out
}

/// Whether a string extra data value turns actor collision off: any value
/// starting with `NC` in either case (`NCO` no collision, `NCC` camera
/// only; `OpenMW`'s `BulletNifLoader` reads it the same way).
pub fn no_collision_marker(value: &str) -> bool {
    value.get(..2).is_some_and(|p| p.eq_ignore_ascii_case("NC"))
}

/// Engine-space bounds of all visible geometry, `(min, max)`.
pub fn visible_bounds(model: &Model) -> Option<([f32; 3], [f32; 3])> {
    let world = world_transforms(model);
    let mut bounds: Option<(Vec3, Vec3)> = None;
    for (node, transform) in model.nodes.iter().zip(&world) {
        if let (true, Some(mesh)) = (node.visible, node.mesh) {
            for p in &model.meshes[mesh].positions {
                let p = transform.transform_point3(Vec3::from_array(*p));
                bounds = Some(bounds.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
            }
        }
    }
    bounds.map(|(lo, hi)| (lo.to_array(), hi.to_array()))
}

/// Model-space transform of every node (rest pose).
pub fn world_transforms(model: &Model) -> Vec<Affine3A> {
    // Parents precede children, so one pass suffices.
    let mut world: Vec<Affine3A> = Vec::with_capacity(model.nodes.len());
    for node in &model.nodes {
        let local = node.transform.to_affine();
        world.push(node.parent.map_or(local, |p| world[p] * local));
    }
    world
}

#[cfg(test)]
mod tests;
