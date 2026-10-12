//! Morrowind models as Bevy assets.
//!
//! `asset_server.load::<WorldAsset>("meshes/x/foo.nif")` parses the NIF
//! through [`mw_nif`], creates meshes and materials as labeled sub-assets,
//! and returns a [`WorldAsset`] holding the model's node hierarchy. Spawn it
//! with `WorldAssetRoot(handle)`.
//!
//! Textures resolve through the VFS with Morrowind's rule that a `.dds`
//! file replaces whatever extension the NIF names, and with modern formats
//! allowed to take the slot (see [`TEXTURE_ALTERNATES`]).
//!
//! The plugin also runs what the model needs at runtime: controllers
//! ([`animation`]), particles ([`particles`]), billboards ([`billboard`]),
//! and placed lights ([`lights`]).

pub mod animation;
pub mod billboard;
pub mod cooked;
pub mod ground_blend;
pub mod lights;
pub mod particles;

use std::sync::Arc;

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use bevy::world_serialization::WorldAsset;
use mw_nif::anim::Target;
use mw_nif::{AlphaMode as NifAlpha, Model, NodeKind, VertexColorMode};
use sf_vfs::{Vfs, VfsPath};

pub use animation::{ModelAnimation, NifAnimation, NodeAnimation};
pub use billboard::Billboard;
pub use cooked::{CookedLodSettings, CookedMaterials, CookedModelRoot, CookedSpawned};
pub use ground_blend::ground_blend_band;
pub use lights::{AttachLight, LightCalibration};
pub use particles::{ActorEmitter, LiveEmitter, LiveNode, LiveSystem, NifNode, ParticleEmitter, ParticleRange};

/// Extensions that may replace a texture reference, in preference order.
/// `dds` first is the original engine's behavior; `ktx2`/`png` let modern
/// texture mods take the slot.
pub const TEXTURE_ALTERNATES: &[&str] = &["ktx2", "dds", "png"];

/// Camera exposure the content is calibrated for (EV100). Interiors are
/// lit like a dim room; see [`LightCalibration`] for the light side.
pub const SCENE_EV100: f32 = 7.0;

/// Luminance (nits) that renders as "fully lit" white at [`SCENE_EV100`]:
/// the inverse of Bevy's exposure factor `2^-EV100 / 1.2`.
pub fn emissive_nits() -> f32 {
    SCENE_EV100.exp2() * 1.2
}

/// Shared, immutable VFS used by loaders to resolve dependencies.
#[derive(Resource, Clone)]
pub struct SharedVfs(pub Arc<Vfs>);

/// Registers the NIF loader. The VFS must also be registered as the
/// default asset source (`sf_vfs::bevy::asset_source`) before
/// `DefaultPlugins`.
pub struct MwAssetsPlugin {
    pub vfs: Arc<Vfs>,
}

impl Plugin for MwAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SharedVfs(self.vfs.clone()))
            .init_resource::<LightCalibration>()
            .init_resource::<CookedLodSettings>()
            .init_resource::<CookedMaterials>()
            .add_plugins(sf_model_bevy::SfModelPlugin)
            .init_asset::<NifAnimation>()
            .register_asset_loader(NifLoader { vfs: self.vfs.clone() })
            .add_systems(
                Update,
                (
                    animation::animate_materials,
                    (animation::assign_phases, animation::animate_nodes).chain(),
                    (particles::start_particles, particles::simulate_particles).chain(),
                    lights::attach_lights,
                    lights::animate_lights,
                    cooked::spawn_cooked,
                ),
            )
            .add_systems(PostUpdate, billboard::face_camera.before(TransformSystems::Propagate));
    }
}

/// Collision-only geometry kept for physics (never rendered).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CollisionGeometry(pub Handle<Mesh>);

#[derive(TypePath)]
pub struct NifLoader {
    vfs: Arc<Vfs>,
}

#[derive(Debug, thiserror::Error)]
pub enum NifLoadError {
    #[error("failed to read NIF: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Model(#[from] mw_nif::ModelError),
}

impl AssetLoader for NifLoader {
    type Asset = WorldAsset;
    type Settings = ();
    type Error = NifLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<WorldAsset, NifLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let model = mw_nif::load(&bytes)?;
        Ok(build_world(&model, &self.vfs, load_context))
    }

    fn extensions(&self) -> &[&str] {
        &["nif"]
    }
}

fn build_world(model: &Model, vfs: &Vfs, load_context: &mut LoadContext<'_>) -> WorldAsset {
    let materials: Vec<Handle<StandardMaterial>> = model
        .materials
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let material = standard_material(m, vfs, load_context);
            load_context.add_labeled_asset(format!("Material{i}"), material)
        })
        .collect();

    // Controllers by the node they drive; particle systems by their node.
    let mut node_controllers: Vec<Vec<u32>> = vec![Vec::new(); model.nodes.len()];
    for (i, controller) in model.controllers.iter().enumerate() {
        if let Target::Transform { node, .. } | Target::Visibility { node, .. } = controller.target {
            node_controllers[node].push(u32::try_from(i).unwrap_or(u32::MAX));
        }
    }
    let mut node_particles: Vec<Vec<u32>> = vec![Vec::new(); model.nodes.len()];
    for (i, system) in model.particles.iter().enumerate() {
        node_particles[system.node].push(u32::try_from(i).unwrap_or(u32::MAX));
    }
    let animation = (!model.controllers.is_empty() || !model.particles.is_empty()).then(|| {
        load_context.add_labeled_asset(
            "Animation".to_owned(),
            NifAnimation { controllers: model.controllers.clone(), particles: model.particles.clone(), materials: materials.clone() },
        )
    });

    let meshes: Vec<Handle<Mesh>> = model
        .meshes
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let colors = model.materials[m.material].vertex_colors != VertexColorMode::Ignore;
            load_context.add_labeled_asset(format!("Mesh{i}"), bevy_mesh(m, colors))
        })
        .collect();

    let mut world = World::default();
    let root_name = load_context.path().to_string();
    let root = world
        .spawn((
            Transform::IDENTITY,
            // Editor marker models are invisible in game.
            if model.marker { Visibility::Hidden } else { Visibility::Inherited },
            Name::new(root_name),
        ))
        .id();
    if let Some(animation) = &animation {
        // Keeps the animation (and its material controllers) alive for as
        // long as an instance exists, even when no node is animated.
        world.entity_mut(root).insert(ModelAnimation(animation.clone()));
    }

    // Nodes that keyframed emitters follow live (`particles::static_live`).
    let live_nodes = particles::live_nodes(model);
    let mut entities: Vec<Entity> = Vec::with_capacity(model.nodes.len());
    for node in &model.nodes {
        let parent = node.parent.map_or(root, |p| entities[p]);
        let t = node.transform;
        let mut entity = world.spawn((
            Transform {
                translation: Vec3::from_array(t.translation),
                rotation: Quat::from_array(t.rotation),
                scale: Vec3::splat(t.scale),
            },
            if node.visible { Visibility::Inherited } else { Visibility::Hidden },
            Name::new(node.name.clone()),
            ChildOf(parent),
        ));
        if node.kind == NodeKind::Billboard {
            entity.insert(Billboard);
        }
        if node.name.eq_ignore_ascii_case("AttachLight") {
            entity.insert(AttachLight);
        }
        if let Some(animation) = &animation {
            let index = entities.len();
            if !node_controllers[index].is_empty() {
                entity.insert(NodeAnimation { animation: animation.clone(), controllers: std::mem::take(&mut node_controllers[index]) });
            }
            if !node_particles[index].is_empty() {
                entity.insert(ParticleEmitter { animation: animation.clone(), systems: std::mem::take(&mut node_particles[index]) });
            }
            if live_nodes.binary_search(&index).is_ok() {
                entity.insert(NifNode(u32::try_from(index).unwrap_or(u32::MAX)));
            }
        }
        if let Some(mesh_index) = node.mesh {
            let mesh = &model.meshes[mesh_index];
            if mesh.collision {
                entity.insert(CollisionGeometry(meshes[mesh_index].clone()));
            } else {
                entity.insert((Mesh3d(meshes[mesh_index].clone()), MeshMaterial3d(materials[mesh.material].clone())));
            }
        }
        entities.push(entity.id());
    }

    WorldAsset::new(world)
}

fn bevy_mesh(mesh: &mw_nif::Mesh, with_colors: bool) -> Mesh {
    let mut out = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    out.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions.clone());
    let uvs = if mesh.uvs.len() == mesh.positions.len() { mesh.uvs.clone() } else { vec![[0.0; 2]; mesh.positions.len()] };
    out.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    if with_colors && mesh.colors.len() == mesh.positions.len() {
        out.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh.colors.clone());
    }
    out.insert_indices(Indices::U32(mesh.indices.clone()));
    if mesh.normals.len() == mesh.positions.len() {
        out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals.clone());
    } else {
        out.compute_smooth_normals();
    }
    out
}

fn standard_material(m: &mw_nif::Material, vfs: &Vfs, load_context: &mut LoadContext<'_>) -> StandardMaterial {
    let [r, g, b] = m.diffuse;
    let [er, eg, eb] = m.emissive;
    let surface = Surface {
        base_color: Color::srgba(r, g, b, m.alpha),
        base_texture: m.base_texture.as_deref(),
        glow_texture: m.glow_texture.as_deref(),
        emissive: Color::srgb(er, eg, eb).to_linear(),
        alpha_mode: match m.alpha_mode {
            NifAlpha::Opaque => AlphaMode::Opaque,
            NifAlpha::Mask { cutoff } => AlphaMode::Mask(cutoff),
            NifAlpha::Blend { additive: true } => AlphaMode::Add,
            NifAlpha::Blend { additive: false } => AlphaMode::Blend,
        },
        double_sided: m.double_sided,
    };
    let source = load_context.path().to_string();
    surface.to_material(|path| {
        let resolved = resolve_texture(path, vfs, &source)?;
        Some(load_context.load_builder().with_settings(image_settings).load(resolved))
    })
}

/// A NIF surface, whichever path it came from (a NIF parsed at runtime or a
/// cooked model's material). One mapping to PBR keeps both paths identical.
pub(crate) struct Surface<'a> {
    pub base_color: Color,
    pub base_texture: Option<&'a str>,
    pub glow_texture: Option<&'a str>,
    /// Linear emissive color (before exposure scaling).
    pub emissive: LinearRgba,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
}

impl Surface<'_> {
    /// `texture` resolves a VFS texture reference and starts loading it.
    pub fn to_material(&self, mut texture: impl FnMut(&str) -> Option<Handle<Image>>) -> StandardMaterial {
        let mut material = StandardMaterial {
            base_color: self.base_color,
            base_color_texture: self.base_texture.and_then(&mut texture),
            // 2002 assets carry no PBR data: matte, low reflectance until
            // generated or replacement maps exist.
            perceptual_roughness: 0.85,
            reflectance: 0.3,
            double_sided: self.double_sided,
            cull_mode: if self.double_sided { None } else { Some(bevy::render::render_resource::Face::Back) },
            alpha_mode: self.alpha_mode,
            ..StandardMaterial::default()
        };

        // Fixed-function NIF shading is `texture * (emissive + lighting)`: the
        // emissive color glows *through the base texture*. Bevy adds
        // `emissive * emissive_texture` in nits, scaled by camera exposure, so
        // the base texture becomes the emissive texture and the color is scaled
        // to read as "fully lit" at the scene exposure. A glow map replaces both.
        let emissive = self.emissive;
        if let Some(glow) = self.glow_texture.and_then(&mut texture) {
            material.emissive_texture = Some(glow);
            material.emissive = LinearRgba::WHITE * emissive_nits();
        } else if emissive.red + emissive.green + emissive.blue > 0.0 {
            material.emissive_texture.clone_from(&material.base_color_texture);
            material.emissive = emissive * emissive_nits();
        }
        material
    }
}

/// Resolves a texture reference through the VFS (see [`TEXTURE_ALTERNATES`]).
pub(crate) fn resolve_texture(path: &str, vfs: &Vfs, model: &str) -> Option<String> {
    let vpath = VfsPath::new(path).ok()?;
    let Some((resolved, _)) = vfs.resolve_preferring(&vpath, TEXTURE_ALTERNATES) else {
        warn!("missing texture {path} (in {model})");
        return None;
    };
    Some(resolved.as_str().to_owned())
}

/// Loader settings for model textures: sRGB, repeating, linear filtering.
pub(crate) fn image_settings(settings: &mut ImageLoaderSettings) {
    settings.is_srgb = true;
    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
}
