//! Morrowind NIF models as an engine-neutral [`Model`].
//!
//! The conversion resolves what the NIF format leaves implicit:
//! - properties inherit down the scene graph (a node's texturing, material,
//!   alpha, ... apply to its subtree until overridden),
//! - geometry is converted to engine space (Y up, meters; see [`coords`]),
//! - render visibility: app-culled nodes, editor markers, collision
//!   subtrees, and inactive switch children are kept but marked,
//! - materials are deduplicated per model (equal property sets share one).
//!
//! Engines turn a [`Model`] into their own meshes, materials, and entities;
//! nothing here depends on an engine.

pub mod coords;
mod material;

pub use material::{AlphaMode, Material, VertexColorMode, texture_path};

use std::collections::HashMap;

use tes3::nif::glam::{Affine3A, Vec2, Vec3, Vec4};
use tes3::nif::{
    NiAVObject, NiBillboardNode, NiGeometry, NiGeometryData, NiKey, NiLink, NiNode, NiProperty, NiStream,
    NiStringExtraData, NiSwitchNode, NiTriShapeData, NiTriStripsData, NiType, RootCollisionNode,
};

use coords::Trs;

/// A converted model: a node tree plus the meshes and materials it uses.
#[derive(Clone, Debug, Default)]
pub struct Model {
    /// Parents always precede their children.
    pub nodes: Vec<Node>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    /// Root carries `MRK` extra data: an editor-only marker model.
    pub marker: bool,
    /// Root carries `NCO` extra data: no collision.
    pub no_collision: bool,
    /// Geometry had skinning; it was baked into its bind pose.
    pub had_skin: bool,
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
    let stream = NiStream::from_bytes(bytes).map_err(ModelError::Parse)?;
    convert(stream)
}

/// Converts a parsed NIF stream.
pub fn convert(mut stream: NiStream) -> Result<Model, ModelError> {
    if stream.roots.is_empty() {
        return Err(ModelError::NoRoots);
    }
    let had_skin = stream.objects_of_type::<NiGeometry>().any(|g| !g.skin_instance.is_null());
    if had_skin {
        stream.apply_skin_deforms();
    }

    let mut builder = Builder { stream: &stream, model: Model { had_skin, ..Model::default() }, material_index: HashMap::new() };
    for root in stream.roots.clone() {
        if let Some(av) = stream.get_as::<_, NiAVObject>(root) {
            builder.read_root_extra_data(av.base.extra_data);
        }
        builder.visit(root.key, None, &Inherited::default(), true, false);
    }
    Ok(builder.model)
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
}

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
    material_index: HashMap<Material, usize>,
}

impl Builder<'_> {
    fn read_root_extra_data(&mut self, first: NiLink<tes3::nif::NiExtraData>) {
        let mut next = first;
        let mut guard = 0;
        while !next.is_null() && guard < 64 {
            guard += 1;
            let Some(object) = self.stream.objects.get(next.key) else { break };
            if let Ok(text) = <&NiStringExtraData>::try_from(object) {
                let value = text.value.trim_start();
                self.model.marker |= value.starts_with("MRK");
                self.model.no_collision |= value.starts_with("NCO");
            }
            let Ok(extra) = <&tes3::nif::NiExtraData>::try_from(object) else { break };
            next = extra.next;
        }
    }

    fn visit(&mut self, key: NiKey, parent: Option<usize>, inherited: &Inherited, visible: bool, collision: bool) {
        let Some(object) = self.stream.objects.get(key) else { return };
        let Ok(av) = <&NiAVObject>::try_from(object) else { return };

        let name = av.base.name.clone();
        let inherited = inherited.with(self.stream, &av.properties);
        let is_collision = collision || <&RootCollisionNode>::try_from(object).is_ok();
        let editor_marker = name.to_ascii_lowercase().starts_with("tri editormarker");
        let visible = visible && !av.app_culled() && !is_collision && !editor_marker;
        let transform = Trs::from_affine(coords::affine(av.transform()));

        if let Ok(geometry) = <&NiGeometry>::try_from(object) {
            let mesh = self.mesh(geometry, &inherited, is_collision);
            self.model.nodes.push(Node {
                name,
                parent,
                transform,
                kind: NodeKind::Geometry,
                visible: visible && mesh.is_some(),
                mesh,
            });
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
        let index = self.model.nodes.len();
        self.model.nodes.push(Node { name, parent, transform, kind: kind.clone(), visible, mesh: None });

        for (i, child) in node.children.iter().enumerate() {
            let child_visible = match kind {
                NodeKind::Switch { active } => visible && i == active,
                _ => visible,
            };
            self.visit(child.key, Some(index), &inherited, child_visible, is_collision);
        }
    }

    fn mesh(&mut self, geometry: &NiGeometry, inherited: &Inherited, collision: bool) -> Option<usize> {
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

        let material = material::from_properties(self.stream, inherited);
        let material = *self.material_index.entry(material.clone()).or_insert_with(|| {
            self.model.materials.push(material);
            self.model.materials.len() - 1
        });

        let uvs = if data.uv_sets.len() >= vertex_count { data.uv_sets[..vertex_count].iter().map(Vec2::to_array).collect() } else { Vec::new() };
        let mesh = Mesh {
            positions: data.vertices.iter().map(|&v| coords::point(v).to_array()).collect(),
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
        };
        self.model.meshes.push(mesh);
        Some(self.model.meshes.len() - 1)
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

/// Engine-space bounds of all visible geometry, `(min, max)`.
pub fn visible_bounds(model: &Model) -> Option<([f32; 3], [f32; 3])> {
    // World transforms per node, parents first.
    let mut world: Vec<Affine3A> = Vec::with_capacity(model.nodes.len());
    let mut bounds: Option<(Vec3, Vec3)> = None;
    for node in &model.nodes {
        let local = node.transform.to_affine();
        let transform = node.parent.map_or(local, |p| world[p] * local);
        world.push(transform);
        if let (true, Some(mesh)) = (node.visible, node.mesh) {
            for p in &model.meshes[mesh].positions {
                let p = transform.transform_point3(Vec3::from_array(*p));
                bounds = Some(bounds.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
            }
        }
    }
    bounds.map(|(lo, hi)| (lo.to_array(), hi.to_array()))
}

#[cfg(test)]
mod tests;
