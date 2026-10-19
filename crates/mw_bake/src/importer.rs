//! NIF as an `sf_model` importer: [`mw_nif::Model`] → [`sf_model::Model`].
//!
//! Engine space is already handled by `mw_nif` (Y up, meters, root
//! transforms dropped), so nodes and geometry copy over unchanged. What this
//! adds:
//!
//! - **Render set.** Collision geometry, hidden nodes (app-culled, editor
//!   markers, inactive switch children) and whole marker models (`MRK`)
//!   carry no mesh.
//! - **Dynamic parts** ([`Dynamic`]). Any controller, billboard, particle
//!   system, or switch node makes its node `animated`; material animation
//!   marks every node drawing that material; a geometry morph marks its
//!   node; a skinned model marks its root (actors move). `mesh.cook` skips
//!   models with animated nodes unless they deform, and the runtime keeps
//!   the NIF path for all of them.
//! - **Skins and morphs** ([`mw_nif::skin`]): skinned shapes carry their
//!   bind-space vertices, joints (node indices + inverse binds) and
//!   influences; morph targets 1.. become [`MorphTarget`]s named
//!   `"<node>#<target>"`, all [`MorphKind::Expression`] (keyframed, never a
//!   static shape). `mesh.cook` then makes a `Cooked::Skinned`; rigid
//!   meshes of a skinned model become single-joint parts there.
//! - **Materials** in linear color (the NIF stores sRGB-ish colors that M1
//!   renders as sRGB), with the NIF specifics the runtime needs in
//!   [`Material::extras`] (keys in [`extras`]).
//! - **LOD policy.** Alpha-blended parts are kept whole in every level;
//!   parts smaller than [`DROP_FRACTION`] of the model leave after LOD0;
//!   the rest is simplified.
//!
//! Spec: `docs/specs/nif-runtime.md` ("Bake adapter").

use std::collections::BTreeSet;

use mw_nif::anim::Target;
use mw_nif::glam::{Affine3A, Mat4, Vec3};
use mw_nif::{AlphaMode as NifAlpha, NodeKind, VertexColorMode};
use sf_model::{
    AlphaMode, ImportError, LodPolicy, Material, MeshData, MeshSkin, Model, ModelImporter, MorphKind, MorphTarget, Node, SkinJoint,
};

/// Parts whose bounding radius is below this fraction of the model's leave
/// after LOD0: at LOD1's switch distance (about 10× the model radius at
/// the reference screen) they cover about two pixels.
pub const DROP_FRACTION: f32 = 0.02;

/// Keys of [`Material::extras`] set by the NIF importer.
pub mod extras {
    /// `"ignore"`, `"emissive"`, or `"ambient_diffuse"` (NIF vertex color mode).
    pub const VERTEX_COLORS: &str = "nif.vertex_colors";
    /// VFS path of the glow map; replaces the emissive color when present.
    pub const GLOW_TEXTURE: &str = "nif.glow_texture";
    /// `"additive"` for additive blending (`AlphaMode::Blend` only); `"cutout"`
    /// for a `Mask` promoted from an authored blend (`mw_nif::Cutout::Promoted`); `"cutout_candidate"` for a plain
    /// blend whose texture alpha decides at load (`mw_nif::Cutout::Candidate`).
    pub const BLEND: &str = "nif.blend";
    /// `"false"` when the surface doesn't write depth.
    pub const DEPTH_WRITE: &str = "nif.depth_write";
    /// `"false"` when the surface ignores depth.
    pub const DEPTH_TEST: &str = "nif.depth_test";
}

/// Imports `.nif` files.
#[derive(Clone, Copy, Debug, Default)]
pub struct NifImporter;

impl NifImporter {
    /// Bump when [`to_model`]'s output changes (invalidates cooked products).
    /// 2: skin data and geometry morphs (only skinned or morphing NIFs change).
    /// 3: foliage cutouts (`nif.blend = "cutout"` / `"cutout_candidate"`, `mw_nif::Cutout`).
    pub const VERSION: u32 = 3;
}

impl ModelImporter for NifImporter {
    fn extensions(&self) -> &[&str] {
        &["nif"]
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        let nif = mw_nif::load(bytes).map_err(|e| ImportError { path: path.to_owned(), message: e.to_string() })?;
        Ok(to_model(&nif))
    }
}

/// Why a model can't be cooked statically (yet).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dynamic {
    /// Skinned geometry (actors, creatures, body parts).
    Skinned,
    /// Geometry morph controller (heads).
    Morph,
    Particles,
    Billboard,
    /// Switch or LOD node: which child shows changes at runtime.
    Switch,
    /// Keyframe controller that plays by itself.
    Keyframes,
    /// Keyframe controller that waits for gameplay (scripts, actors).
    GameplayKeyframes,
    Visibility,
    /// Alpha or material color controller.
    MaterialAnimation,
    UvAnimation,
}

impl Dynamic {
    pub fn label(self) -> &'static str {
        match self {
            Self::Skinned => "skinned",
            Self::Morph => "geometry morph",
            Self::Particles => "particles",
            Self::Billboard => "billboard",
            Self::Switch => "switch/LOD node",
            Self::Keyframes => "keyframes (auto-play)",
            Self::GameplayKeyframes => "keyframes (gameplay-driven)",
            Self::Visibility => "visibility controller",
            Self::MaterialAnimation => "material color/alpha controller",
            Self::UvAnimation => "UV controller",
        }
    }
}

/// Everything in `nif` that moves or changes at runtime.
pub fn dynamic_parts(nif: &mw_nif::Model) -> BTreeSet<Dynamic> {
    let mut out = BTreeSet::new();
    if nif.had_skin {
        out.insert(Dynamic::Skinned);
    }
    if nif.meshes.iter().any(|m| m.morph.is_some()) {
        out.insert(Dynamic::Morph);
    }
    if !nif.particles.is_empty() {
        out.insert(Dynamic::Particles);
    }
    for node in &nif.nodes {
        match node.kind {
            NodeKind::Billboard => {
                out.insert(Dynamic::Billboard);
            }
            NodeKind::Switch { .. } => {
                out.insert(Dynamic::Switch);
            }
            NodeKind::Particles => {
                out.insert(Dynamic::Particles);
            }
            NodeKind::Node | NodeKind::Geometry | NodeKind::Collision => {}
        }
    }
    for controller in &nif.controllers {
        out.insert(match controller.target {
            Target::Transform { .. } if controller.auto_play => Dynamic::Keyframes,
            Target::Transform { .. } => Dynamic::GameplayKeyframes,
            Target::Visibility { .. } => Dynamic::Visibility,
            Target::Alpha { .. } | Target::MaterialColor { .. } => Dynamic::MaterialAnimation,
            Target::Uv { .. } => Dynamic::UvAnimation,
        });
    }
    out
}

/// Per node: whether it moves, changes, or draws an animated material.
fn animated_nodes(nif: &mw_nif::Model) -> Vec<bool> {
    let mut animated: Vec<bool> = nif
        .nodes
        .iter()
        .map(|n| matches!(n.kind, NodeKind::Billboard | NodeKind::Switch { .. } | NodeKind::Particles))
        .collect();
    let mut materials = vec![false; nif.materials.len()];
    for controller in &nif.controllers {
        match controller.target {
            Target::Transform { node, .. } | Target::Visibility { node, .. } => {
                if let Some(a) = animated.get_mut(node) {
                    *a = true;
                }
            }
            Target::Alpha { material, .. } | Target::MaterialColor { material, .. } | Target::Uv { material, .. } => {
                if let Some(m) = materials.get_mut(material) {
                    *m = true;
                }
            }
        }
    }
    for system in &nif.particles {
        if let Some(a) = animated.get_mut(system.node) {
            *a = true;
        }
    }
    for (node, a) in nif.nodes.iter().zip(&mut animated) {
        if let Some(mesh) = node.mesh.and_then(|m| nif.meshes.get(m))
            && (mesh.morph.is_some() || materials.get(mesh.material).copied().unwrap_or(false))
        {
            *a = true;
        }
        // Skinned models are actors: the root is the moving actor.
        *a |= nif.had_skin && node.parent.is_none();
    }
    animated
}

/// Converts a NIF model to the framework's neutral model.
pub fn to_model(nif: &mw_nif::Model) -> Model {
    let animated = animated_nodes(nif);
    let world = mw_nif::world_transforms(nif);
    // The mesh a node renders, if any.
    let rendered = |node: &mw_nif::Node| -> Option<usize> {
        let mesh = node.mesh?;
        (!nif.marker && node.visible && !nif.meshes[mesh].collision).then_some(mesh)
    };
    let model_radius = {
        let mut bounds = Bounds::default();
        for (node, transform) in nif.nodes.iter().zip(&world) {
            if let Some(mesh) = rendered(node) {
                bounds.add_mesh(&nif.meshes[mesh], transform);
            }
        }
        bounds.radius()
    };

    let mut out = Model::default();
    // Source material index -> output index; only used materials are kept.
    let mut material_map: Vec<Option<u32>> = vec![None; nif.materials.len()];
    for (i, node) in nif.nodes.iter().enumerate() {
        let t = node.transform;
        let mut n = Node::new(node.name.clone());
        n.parent = node.parent.map(index);
        n.translation = t.translation;
        n.rotation = t.rotation;
        n.scale = [t.scale; 3];
        n.animated = animated[i];
        if let Some(m) = rendered(node) {
            let mesh = &nif.meshes[m];
            let source = &nif.materials[mesh.material];
            let material = *material_map[mesh.material].get_or_insert_with(|| {
                out.materials.push(material(source, out.materials.len()));
                index(out.materials.len() - 1)
            });
            let mut part = Bounds::default();
            part.add_mesh(mesh, &world[i]);
            n.lod = if matches!(source.alpha_mode, NifAlpha::Blend { .. }) || source.cutout != mw_nif::Cutout::No {
                LodPolicy::Keep
            } else if part.radius() < model_radius * DROP_FRACTION {
                LodPolicy::Drop
            } else {
                LodPolicy::Simplify
            };
            n.mesh = Some(index(out.meshes.len()));
            out.meshes.push(mesh_data(mesh, &node.name, material, source.vertex_colors != VertexColorMode::Ignore));
        }
        out.nodes.push(n);
    }
    out
}

fn index(i: usize) -> u32 {
    u32::try_from(i).expect("model element counts fit in u32")
}

/// One mesh; skinned meshes in bind space (their node's transform is
/// ignored), morph targets named `"<node>#<target>"` (targets are per mesh
/// in a NIF, while `sf_model` matches them across meshes by name).
fn mesh_data(mesh: &mw_nif::Mesh, node: &str, material: u32, vertex_colors: bool) -> MeshData {
    let (positions, normals) = match &mesh.skin {
        Some(skin) => (&skin.positions, &skin.normals),
        None => (&mesh.positions, &mesh.normals),
    };
    let n = positions.len();
    let skin = mesh.skin.as_ref().map(|skin| MeshSkin {
        joints: skin
            .bones
            .iter()
            .map(|b| SkinJoint { node: index(b.node), inverse_bind: Mat4::from(b.inverse_bind).to_cols_array() })
            .collect(),
        influences: skin.influences.clone(),
    });
    let morph_targets = mesh.morph.as_ref().map_or_else(Vec::new, |morph| {
        morph
            .targets
            .iter()
            .zip(morph.rest_weights())
            .enumerate()
            .map(|(i, (target, weight))| MorphTarget {
                name: format!("{node}#{}", i + 1),
                positions: target.deltas.clone(),
                normals: None,
                default_weight: weight,
                // Every NIF geometry morph is keyframed (heads blink and talk by text keys;
                // effects and bows animate), so none is a static shape to bake at assembly.
                kind: MorphKind::Expression,
            })
            .collect()
    });
    MeshData {
        positions: positions.clone(),
        normals: Some(if normals.len() == n { normals.clone() } else { smooth_normals(positions, &mesh.indices) }),
        uvs: (mesh.uvs.len() == n).then(|| mesh.uvs.clone()),
        colors: (vertex_colors && mesh.colors.len() == n).then(|| mesh.colors.clone()),
        skinned: mesh.skinned,
        skin,
        morph_targets,
        indices: mesh.indices.clone(),
        material,
    }
}

/// Area-weighted vertex normals, for the rare NIF shape without normals.
fn smooth_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for tri in indices.as_chunks::<3>().0 {
        let [a, b, c] = tri.map(|i| i as usize);
        let p = |i: usize| Vec3::from_array(positions[i]);
        let face = (p(b) - p(a)).cross(p(c) - p(a));
        for i in [a, b, c] {
            normals[i] += face;
        }
    }
    normals.into_iter().map(|n| n.normalize_or(Vec3::Y).to_array()).collect()
}

/// The NIF's colors are rendered as sRGB (M1); the neutral model is linear.
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn material(m: &mw_nif::Material, index: usize) -> Material {
    let [r, g, b] = m.diffuse.map(srgb_to_linear);
    let mut out = Material {
        name: m.base_texture.as_deref().map_or_else(
            || format!("material{index}"),
            |t| t.rsplit('/').next().unwrap_or(t).to_owned(),
        ),
        base_color_texture: m.base_texture.clone(),
        base_color: [r, g, b, m.alpha],
        emissive: m.emissive.map(srgb_to_linear),
        alpha: match m.alpha_mode {
            NifAlpha::Opaque => AlphaMode::Opaque,
            NifAlpha::Mask { cutoff } => AlphaMode::Mask(cutoff),
            NifAlpha::Blend { .. } => AlphaMode::Blend,
        },
        double_sided: m.double_sided,
        unlit: false,
        extras: std::collections::BTreeMap::new(),
    };
    let vertex_colors = match m.vertex_colors {
        VertexColorMode::Ignore => "ignore",
        VertexColorMode::Emissive => "emissive",
        VertexColorMode::AmbientDiffuse => "ambient_diffuse",
    };
    out.extras.insert(extras::VERTEX_COLORS.into(), vertex_colors.into());
    if let Some(glow) = &m.glow_texture {
        out.extras.insert(extras::GLOW_TEXTURE.into(), glow.clone());
    }
    if matches!(m.alpha_mode, NifAlpha::Blend { additive: true }) {
        out.extras.insert(extras::BLEND.into(), "additive".into());
    } else if m.cutout == mw_nif::Cutout::Promoted {
        out.extras.insert(extras::BLEND.into(), "cutout".into());
    } else if m.cutout == mw_nif::Cutout::Candidate {
        out.extras.insert(extras::BLEND.into(), "cutout_candidate".into());
    }
    if !m.depth_write {
        out.extras.insert(extras::DEPTH_WRITE.into(), "false".into());
    }
    if !m.depth_test {
        out.extras.insert(extras::DEPTH_TEST.into(), "false".into());
    }
    out
}

/// Model-space bounds of transformed meshes.
#[derive(Default)]
struct Bounds(Option<(Vec3, Vec3)>);

impl Bounds {
    fn add_mesh(&mut self, mesh: &mw_nif::Mesh, transform: &Affine3A) {
        for p in &mesh.positions {
            let p = transform.transform_point3(Vec3::from_array(*p));
            self.0 = Some(self.0.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
        }
    }

    fn radius(&self) -> f32 {
        self.0.map_or(0.0, |(lo, hi)| (hi - lo).length() * 0.5)
    }
}

#[cfg(test)]
mod tests {
    use mw_nif::anim::{Controller, Steps, Timing};
    use mw_nif::coords::Trs;

    use super::*;

    fn node(name: &str, parent: Option<usize>, kind: NodeKind, mesh: Option<usize>) -> mw_nif::Node {
        mw_nif::Node { name: name.into(), parent, transform: Trs::IDENTITY, kind, visible: true, mesh }
    }

    fn quad(size: f32, material: usize) -> mw_nif::Mesh {
        mw_nif::Mesh {
            positions: vec![[0.0, 0.0, 0.0], [size, 0.0, 0.0], [size, size, 0.0], [0.0, size, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            uvs: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            colors: vec![[0.5, 0.5, 0.5, 1.0]; 4],
            indices: vec![0, 1, 2, 0, 2, 3],
            material,
            ..mw_nif::Mesh::default()
        }
    }

    /// Root with a big quad, a tiny quad, a glass quad, a collision quad,
    /// and a hidden quad. Materials: 0 wood, 1 glass, 2 unused.
    fn sample() -> mw_nif::Model {
        let mut nif = mw_nif::Model {
            nodes: vec![
                node("root", None, NodeKind::Node, None),
                node("big", Some(0), NodeKind::Geometry, Some(0)),
                node("tiny", Some(0), NodeKind::Geometry, Some(1)),
                node("glass", Some(0), NodeKind::Geometry, Some(2)),
                node("collision", Some(0), NodeKind::Collision, None),
                node("collider", Some(4), NodeKind::Geometry, Some(3)),
                node("hidden", Some(0), NodeKind::Geometry, Some(4)),
            ],
            meshes: vec![quad(2.0, 0), quad(0.01, 0), quad(1.0, 1), quad(1.0, 0), quad(1.0, 2)],
            materials: vec![
                mw_nif::Material {
                    base_texture: Some("textures/tx_wood.tga".into()),
                    diffuse: [0.5, 1.0, 0.0],
                    ..mw_nif::Material::default()
                },
                mw_nif::Material {
                    alpha_mode: NifAlpha::Blend { additive: true },
                    vertex_colors: VertexColorMode::Ignore,
                    ..mw_nif::Material::default()
                },
                mw_nif::Material { double_sided: true, ..mw_nif::Material::default() },
            ],
            ..mw_nif::Model::default()
        };
        nif.meshes[3].collision = true;
        nif.nodes[6].visible = false;
        nif
    }

    #[test]
    fn render_set_lods_and_materials() {
        let model = to_model(&sample());
        assert_eq!(model.nodes.len(), 7, "every node is kept for the hierarchy");
        let mesh_of = |name: &str| model.nodes.iter().find(|n| n.name == name).unwrap().mesh;
        assert!(mesh_of("collider").is_none() && mesh_of("hidden").is_none() && mesh_of("root").is_none());
        assert_eq!(model.meshes.len(), 3);
        // Unused material 2 (only on the hidden node) is dropped.
        assert_eq!(model.materials.len(), 2);

        let lod_of = |name: &str| model.nodes.iter().find(|n| n.name == name).unwrap().lod;
        assert_eq!(lod_of("big"), LodPolicy::Simplify);
        assert_eq!(lod_of("tiny"), LodPolicy::Drop);
        assert_eq!(lod_of("glass"), LodPolicy::Keep);

        let wood = &model.materials[0];
        assert_eq!(wood.base_color_texture.as_deref(), Some("textures/tx_wood.tga"));
        assert_eq!(wood.name, "tx_wood.tga");
        assert!((wood.base_color[0] - 0.214).abs() < 1e-3, "sRGB 0.5 is linear 0.214");
        assert!((wood.base_color[1] - 1.0).abs() < 1e-6);
        let glass = &model.materials[1];
        assert_eq!(glass.alpha, AlphaMode::Blend);
        assert_eq!(glass.extras.get(extras::BLEND).map(String::as_str), Some("additive"));
        // Vertex colors only where the material uses them.
        let glass_mesh = &model.meshes[mesh_of("glass").unwrap() as usize];
        assert!(glass_mesh.colors.is_none());
        assert!(model.meshes[mesh_of("big").unwrap() as usize].colors.is_some());

        assert!(!sf_model::ModelInfo::of(&model).animated);
        let cooked = sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap();
        assert!(matches!(cooked, sf_model::Cooked::Static(_)));
    }

    #[test]
    fn dynamic_parts_make_the_model_skip_cooking() {
        let mut nif = sample();
        assert!(dynamic_parts(&nif).is_empty());
        nif.controllers.push(Controller {
            timing: Timing::default(),
            auto_play: true,
            random_start: false,
            target: Target::Visibility { node: 2, keys: Steps::new([(0.0, true)]) },
        });
        assert_eq!(dynamic_parts(&nif), BTreeSet::from([Dynamic::Visibility]));
        let model = to_model(&nif);
        assert!(model.nodes[2].animated && !model.nodes[1].animated);
        assert!(matches!(
            sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap(),
            sf_model::Cooked::Skipped { .. }
        ));

        // A material controller animates every node drawing that material.
        let mut nif = sample();
        nif.controllers.push(Controller {
            timing: Timing::default(),
            auto_play: true,
            random_start: false,
            target: Target::Alpha { material: 0, curve: mw_nif::anim::Curve::default() },
        });
        let model = to_model(&nif);
        let animated: Vec<&str> = model.nodes.iter().filter(|n| n.animated).map(|n| n.name.as_str()).collect();
        assert_eq!(animated, ["big", "tiny", "collider"]);
        // The rest pose (`mesh.rest`, far use) cooks it statically.
        assert!(matches!(
            sf_model::cook_rest(&model, &sf_model::CookSettings::default()).unwrap(),
            sf_model::Cooked::Static(_)
        ));

        // A skinned model whose skins didn't resolve: skipped; its root moves.
        let mut nif = sample();
        nif.had_skin = true;
        for mesh in &mut nif.meshes {
            mesh.skinned = true;
        }
        let model = to_model(&nif);
        assert!(model.meshes.iter().all(|m| m.skinned) && model.nodes[0].animated);
        assert!(matches!(
            sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap(),
            sf_model::Cooked::Skipped { .. }
        ));
        nif.nodes[0].kind = NodeKind::Billboard;
        assert!(dynamic_parts(&nif).contains(&Dynamic::Billboard));
    }

    #[test]
    fn skins_and_morphs_cook_as_skinned() {
        use mw_nif::skin::{Morph, MorphTarget as NifMorph, Skin, SkinBone};
        let mut nif = sample();
        nif.had_skin = true;
        // "big" is skinned to "root" and "glass"; "glass" morphs.
        let big = &mut nif.meshes[0];
        big.skinned = true;
        big.skin = Some(Box::new(Skin {
            positions: big.positions.iter().map(|p| [p[0], p[1], p[2] + 1.0]).collect(),
            normals: big.normals.clone(),
            bones: vec![
                SkinBone { node: 0, inverse_bind: Affine3A::IDENTITY },
                SkinBone { node: 3, inverse_bind: Affine3A::from_translation(Vec3::NEG_Z) },
            ],
            influences: vec![vec![(0, 0.5), (1, 0.5)]; 4],
            offset_bind: false,
        }));
        nif.meshes[2].morph = Some(Morph {
            timing: Timing::default(),
            targets: vec![NifMorph { deltas: vec![[0.0, 0.1, 0.0]; 4], weight: mw_nif::anim::Curve::default() }],
        });
        assert!(dynamic_parts(&nif).is_superset(&BTreeSet::from([Dynamic::Skinned, Dynamic::Morph])));
        let model = to_model(&nif);
        let mesh_of = |name: &str| &model.meshes[model.nodes.iter().find(|n| n.name == name).unwrap().mesh.unwrap() as usize];
        let big = mesh_of("big");
        let skin = big.skin.as_ref().unwrap();
        assert!(big.skinned && (big.positions[0][2] - 1.0).abs() < 1e-6, "bind-space positions");
        assert_eq!(skin.joints.iter().map(|j| j.node).collect::<Vec<_>>(), [0, 3]);
        assert!((skin.joints[1].inverse_bind[14] + 1.0).abs() < 1e-6, "column-major translation");
        let glass = mesh_of("glass");
        assert!(!glass.skinned && glass.morph_targets[0].name == "glass#1");
        assert!(model.nodes[0].animated, "skinned models are actors");
        let info = sf_model::ModelInfo::of(&model);
        assert!(info.skinned && info.animated, "the app keeps the runtime path");

        let sf_model::Cooked::Skinned(cooked) = sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap() else {
            panic!("expected a skinned cook");
        };
        let names: BTreeSet<&str> = cooked.joints.iter().map(|j| j.name.as_str()).collect();
        assert!(names.contains("root") && names.contains("glass"), "{names:?}");
        assert_eq!(cooked.morph_targets.len(), 1);
        assert_eq!(cooked.parts.len(), 3, "one part per rendered mesh");

        // A morph-only model (a head) marks its node animated, so the app
        // never takes it for a static model.
        let mut nif = sample();
        nif.meshes[0].morph = nif.meshes[2].morph.take().or_else(|| {
            Some(Morph { timing: Timing::default(), targets: vec![NifMorph { deltas: vec![[0.0; 3]; 4], ..NifMorph::default() }] })
        });
        let model = to_model(&nif);
        assert!(sf_model::ModelInfo::of(&model).animated);
        assert!(matches!(sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap(), sf_model::Cooked::Skinned(_)));
    }

    #[test]
    fn marker_models_render_nothing() {
        let mut nif = sample();
        nif.marker = true;
        let model = to_model(&nif);
        assert!(model.meshes.is_empty() && model.materials.is_empty());
    }

    #[test]
    fn missing_normals_are_computed() {
        let mut nif = sample();
        nif.meshes[0].normals.clear();
        let model = to_model(&nif);
        for n in model.meshes[0].normals.as_ref().unwrap() {
            assert!((n[2] - 1.0).abs() < 1e-6);
        }
    }
}
