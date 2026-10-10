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

use std::sync::Arc;

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::world_serialization::WorldAsset;
use mw_nif::{AlphaMode as NifAlpha, Model, NodeKind, VertexColorMode};
use sf_vfs::{Vfs, VfsPath};

/// Extensions that may replace a texture reference, in preference order.
/// `dds` first is the original engine's behavior; `ktx2`/`png` let modern
/// texture mods take the slot.
pub const TEXTURE_ALTERNATES: &[&str] = &["ktx2", "dds", "png"];

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
            .register_asset_loader(NifLoader { vfs: self.vfs.clone() });
    }
}

/// Marks a node that must face the camera (`NiBillboardNode`).
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Billboard;

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
    let mut material = StandardMaterial {
        base_color: Color::srgba(r, g, b, m.alpha),
        base_color_texture: m.base_texture.as_deref().and_then(|t| texture(t, vfs, load_context)),
        // 2002 assets carry no PBR data: matte, low reflectance until
        // generated or replacement maps exist.
        perceptual_roughness: 0.85,
        reflectance: 0.3,
        double_sided: m.double_sided,
        cull_mode: if m.double_sided { None } else { Some(bevy::render::render_resource::Face::Back) },
        alpha_mode: match m.alpha_mode {
            NifAlpha::Opaque => AlphaMode::Opaque,
            NifAlpha::Mask { cutoff } => AlphaMode::Mask(cutoff),
            NifAlpha::Blend { additive: true } => AlphaMode::Add,
            NifAlpha::Blend { additive: false } => AlphaMode::Blend,
        },
        ..StandardMaterial::default()
    };

    let [er, eg, eb] = m.emissive;
    material.emissive = Color::srgb(er, eg, eb).to_linear();
    if let Some(glow) = m.glow_texture.as_deref().and_then(|t| texture(t, vfs, load_context)) {
        material.emissive_texture = Some(glow);
        material.emissive = LinearRgba::WHITE;
    }
    material
}

/// Resolves a texture reference through the VFS and starts loading it.
fn texture(path: &str, vfs: &Vfs, load_context: &mut LoadContext<'_>) -> Option<Handle<Image>> {
    let vpath = VfsPath::new(path).ok()?;
    let Some((resolved, _)) = vfs.resolve_preferring(&vpath, TEXTURE_ALTERNATES) else {
        warn!("missing texture {path} (in {})", load_context.path());
        return None;
    };
    Some(
        load_context
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                settings.is_srgb = true;
                settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..ImageSamplerDescriptor::linear()
                });
            })
            .load(resolved.as_str().to_owned()),
    )
}
