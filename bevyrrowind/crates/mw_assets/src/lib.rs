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
pub use lights::{AttachLight, LightCalibration};
pub use particles::{ActorEmitter, LiveEmitter, LiveNode, LiveSystem, NifNode, ParticleEmitter, ParticleExt, ParticleMaterial, ParticleRange};

/// Extensions that may replace a texture reference, in preference order
/// (defined beside the bake, which resolves textures the same way).
pub use mw_bake::textures::TEXTURE_ALTERNATES;

/// Camera exposure the content is calibrated for (EV100). Interiors are
/// lit like a dim room; see [`LightCalibration`] for the light side.
pub const SCENE_EV100: f32 = 7.0;

/// Luminance (nits) that renders as "fully lit" white at [`SCENE_EV100`]:
/// the inverse of Bevy's exposure factor `2^-EV100 / 1.2`.
pub fn emissive_nits() -> f32 {
    SCENE_EV100.exp2() * 1.2
}

/// Sets a NIF emissive color (linear, 1 = "fully lit") on `material`: in
/// nits ([`emissive_nits`]) and *exposed* like lit surfaces. Bevy 0.20's
/// `emissive_exposure_weight` defaults to 0, which adds emissive nits to
/// the exposed image unscaled: 1 became ~150x white (torchfix 2026-10-12).
pub fn set_emissive(material: &mut StandardMaterial, color: LinearRgba) {
    material.emissive = color * emissive_nits();
    material.emissive_exposure_weight = 1.0;
}

/// A surface inside its own light's saturation radius, as a light model's
/// own geometry (a carried torch around its flame). Morrowind's light term
/// clamps to 1 within a third of the radius, so the original shows the
/// plain texture there; Bevy's point lights have no near clamp
/// (`1 / max(d², 1 cm²)`), so the light a few centimeters away blows it out
/// to white. The surface is mapped as its texture glowing at "fully lit"
/// (the emissive rule with emissive 1) and takes no further light: black
/// base, no specular. Blended surfaces (flames, smoke) stay as they are: `None`.
pub fn self_lit(material: &StandardMaterial) -> Option<StandardMaterial> {
    if matches!(material.alpha_mode, AlphaMode::Blend | AlphaMode::Add | AlphaMode::Premultiplied | AlphaMode::Multiply) {
        return None;
    }
    let mut m = material.clone();
    let base = m.base_color.to_linear();
    if m.emissive_texture.is_none() {
        m.emissive_texture.clone_from(&m.base_color_texture);
    }
    set_emissive(&mut m, LinearRgba::rgb(base.red, base.green, base.blue));
    m.base_color = Color::linear_rgba(0.0, 0.0, 0.0, base.alpha);
    m.reflectance = 0.0;
    Some(m)
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
        bevy::asset::embedded_asset!(app, "shaders/particles.wesl");
        // Particles fog themselves from `TranslucentFog` (render-order.md rule 1).
        if !app.is_plugin_added::<sf_render_order_bevy::RenderOrderPlugin>() {
            app.add_plugins(sf_render_order_bevy::RenderOrderPlugin);
        }
        app.add_plugins(MaterialPlugin::<ParticleMaterial>::default())
            .init_resource::<particles::ParticleBuffers>()
            .add_observer(particles::release_slot);
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
                    (particles::sync_materials, particles::start_particles, particles::simulate_particles, particles::upload_particles).chain(),
                    lights::attach_lights,
                    lights::mark_lamp_bodies,
                    lights::animate_lights,
                    cooked::spawn_cooked,
                    leaf_shadow_receivers,
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
    // Nodes under an `NiBillboardNode` (flames, glows): they turn with the
    // camera, so a light's shadow of them would change with the view (and
    // the lamp shadow cache would redraw every face around its own flame).
    let mut billboarded: Vec<bool> = Vec::with_capacity(model.nodes.len());
    for node in &model.nodes {
        let parent = node.parent.map_or(root, |p| entities[p]);
        let in_billboard = node.kind == NodeKind::Billboard || node.parent.is_some_and(|p| billboarded[p]);
        billboarded.push(in_billboard);
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
                if in_billboard {
                    entity.insert(bevy::light::NotShadowCaster);
                }
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
    let vtx_emissive_unlit = m.vertex_colors == VertexColorMode::Emissive
        && (m.unlit_lighting || m.diffuse == [0.0, 0.0, 0.0]);
    let [r, g, b] = if vtx_emissive_unlit {
        [1.0, 1.0, 1.0]
    } else if m.unlit_lighting {
        [0.0, 0.0, 0.0]
    } else {
        m.diffuse
    };
    let [er, eg, eb] = m.emissive;
    let alpha_mode = match m.alpha_mode {
        NifAlpha::Opaque => AlphaMode::Opaque,
        NifAlpha::Mask { .. } if m.cutout != mw_nif::Cutout::No && !cutout_foliage() => AlphaMode::Blend,
        NifAlpha::Blend { additive: false } if m.cutout == mw_nif::Cutout::Candidate && texture_is_cutout(vfs, m.base_texture.as_deref()) => {
            AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF)
        }
        NifAlpha::Mask { cutoff } => AlphaMode::Mask(cutoff),
        NifAlpha::Blend { additive: true } => AlphaMode::Add,
        NifAlpha::Blend { additive: false } => AlphaMode::Blend,
    };
    let surface = Surface {
        base_color: Color::srgba(r, g, b, m.alpha),
        base_texture: m.base_texture.as_deref(),
        base_clamp: m.base_clamp,
        glow_texture: m.glow_texture.as_deref(),
        // No normal map on the runtime NIF path: its meshes carry no tangents (T4 cooks them).
        normal_texture: None,
        emissive: Color::srgb(er, eg, eb).to_linear(),
        // The leaf rule: an authored blend drawn as a cutout (`mw_nif::Cutout`).
        foliage: m.cutout != mw_nif::Cutout::No && matches!(alpha_mode, AlphaMode::Mask(_)),
        alpha_mode,
        double_sided: m.double_sided,
        unlit: vtx_emissive_unlit,
    };
    let source = load_context.path().to_string();
    surface.to_material(|path, clamp, srgb| {
        let resolved = resolve_texture(path, vfs, &source)?;
        Some(load_context.load_builder().with_settings(move |s| image_settings_clamped(s, clamp, srgb)).load(resolved))
    })
}

/// A/B switch (`--off leafmask`): authored blend + "alpha > 0" surfaces
/// (foliage) draw as cutouts (on, the default) or blended as authored (off).
static CUTOUT_FOLIAGE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

/// Sets the [`CUTOUT_FOLIAGE`] A/B; call before models load.
pub fn set_cutout_foliage(on: bool) {
    CUTOUT_FOLIAGE.store(on, std::sync::atomic::Ordering::Relaxed);
}

/// Whether cutout surfaces (`mw_nif::Cutout`) draw as cutouts.
pub fn cutout_foliage() -> bool {
    CUTOUT_FOLIAGE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Whether a [`mw_nif::Cutout::Candidate`] surface draws as a cutout: its
/// texture's alpha has holes, not translucency ([`mw_bake::textures::alpha_is_cutout`]).
/// Decodes the top mip once per texture (cached). Scaffold: the bake step it
/// belongs in now exists (`texture.alpha_class`, baked for every cutout candidate
/// by `mw_bake::delight::bake_delight` and read by the static-scene bake). Removal
/// path: read that index here instead of decoding, so no load-time decode is left.
pub(crate) fn texture_is_cutout(vfs: &Vfs, texture: Option<&str>) -> bool {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<String, bool>>> =
        std::sync::LazyLock::new(Default::default);
    let Some(texture) = texture.filter(|_| cutout_foliage()) else { return false };
    if let Some(known) = CACHE.lock().ok().and_then(|c| c.get(texture).copied()) {
        return known;
    }
    let cutout = VfsPath::new(texture)
        .ok()
        .and_then(|p| match mw_bake::delight::layer_index(vfs) {
            // De-lit KTX2s keep their source's alpha: classify the source (no BC7 decode here).
            Some(layer) => vfs.resolve_preferring_below(&p, TEXTURE_ALTERNATES, layer),
            None => vfs.resolve_preferring(&p, TEXTURE_ALTERNATES),
        })
        .and_then(|(resolved, _)| Some((vfs.read(&resolved).ok()?, resolved)))
        .and_then(|(bytes, resolved)| mw_bake::textures::decode_rgba(resolved.as_str(), &bytes).ok())
        .is_some_and(|image| mw_bake::textures::alpha_is_cutout(&image));
    if let Ok(mut cache) = CACHE.lock() {
        cache.insert(texture.to_owned(), cutout);
    }
    cutout
}

/// A NIF surface, whichever path it came from (a NIF parsed at runtime or a
/// cooked model's material). One mapping to PBR keeps both paths identical.
pub(crate) struct Surface<'a> {
    pub base_color: Color,
    pub base_texture: Option<&'a str>,
    pub base_clamp: [bool; 2],
    pub glow_texture: Option<&'a str>,
    /// The base texture's tangent-space normal map (T4 companion, `mw_bake::cook::read_companions`):
    /// linear, OpenGL Y (the cook converted it), sampled with the base slot's clamp.
    pub normal_texture: Option<&'a str>,
    /// Linear emissive color (before exposure scaling).
    pub emissive: LinearRgba,
    pub alpha_mode: AlphaMode,
    pub double_sided: bool,
    pub unlit: bool,
    /// A leaf surface: an authored blend drawn as a cutout (`mw_nif::Cutout` promoted or a
    /// candidate whose texture has holes). Gets [`LEAF_DIFFUSE_TRANSMISSION`].
    pub foliage: bool,
}

/// A/B switch (`--off foliagetransmission`): leaf surfaces ([`Surface::foliage`]) transmit
/// light (on, the default) or are opaque cards as before.
static FOLIAGE_TRANSMISSION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

/// Sets the [`FOLIAGE_TRANSMISSION`] A/B; call before models load.
pub fn set_foliage_transmission(on: bool) {
    FOLIAGE_TRANSMISSION.store(on, std::sync::atomic::Ordering::Relaxed);
}

/// Whether leaf surfaces get diffuse transmission.
pub fn foliage_transmission() -> bool {
    FOLIAGE_TRANSMISSION.load(std::sync::atomic::Ordering::Relaxed)
}

/// Leaf meshes (a material with diffuse transmission, [`Surface::foliage`]) take shadows on their
/// transmission lobe too (Bevy `TransmittedShadowReceiver`): without it every leaf in a backlit
/// canopy glows, the ones behind the trunk, a hill or other trees included. Covers every spawn
/// path (streamed statics, NIF scenes, cooked models) by looking at new mesh entities' materials.
fn leaf_shadow_receivers(
    mut commands: Commands,
    added: Query<(Entity, &MeshMaterial3d<StandardMaterial>), Added<MeshMaterial3d<StandardMaterial>>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    for (entity, material) in &added {
        if materials.get(&material.0).is_some_and(|m| m.diffuse_transmission > 0.0) {
            commands.entity(entity).try_insert(bevy::light::TransmittedShadowReceiver);
        }
    }
}

/// Share of a leaf's diffuse light that goes through it (Bevy `StandardMaterial::diffuse_transmission`,
/// screen-space-effects research 4.3; Frostbite/Crysis-style thin translucency as a second, flipped
/// Lambert lobe, ALU only). Bevy splits the energy: the front lobe keeps `1 - t`, so 0.5 halves a
/// front-lit leaf's *direct* term and gives a backlit one half of the sun, with ambient unchanged
/// (front + back lobes sum to the old total under uniform sky). Bevy calls > 0.5 "more transmitted
/// than reflected"; 0.5 is the owner's pick (RO-15, 2026-11-02: 0.3 read too faint, 1.0 tried and
/// liked, "keep at 0.5 at least").
pub const LEAF_DIFFUSE_TRANSMISSION: f32 = 0.5;

/// Depth (meters) of the transmission lobe behind a leaf card (`StandardMaterial::thickness`): the
/// lobe's shadow and point-light lookups happen this far behind the surface. A card stands for a
/// cluster of leaves, so a few centimeters, not a leaf's 0.3 mm; small enough that a lamp right
/// behind a hedge still lights it.
pub const LEAF_THICKNESS: f32 = 0.05;

impl Surface<'_> {
    /// `texture` resolves a VFS texture reference (with `(clamp_s, clamp_t)` and whether it holds
    /// sRGB colour, false for normal maps) and starts loading it.
    pub fn to_material(&self, mut texture: impl FnMut(&str, [bool; 2], bool) -> Option<Handle<Image>>) -> StandardMaterial {
        let mut material = StandardMaterial {
            base_color: self.base_color,
            base_color_texture: self.base_texture.and_then(|p| texture(p, self.base_clamp, true)),
            // 2002 assets carry no PBR data: matte, low reflectance until
            // generated or replacement maps exist.
            perceptual_roughness: 0.85,
            reflectance: 0.3,
            double_sided: self.double_sided,
            cull_mode: if self.double_sided { None } else { Some(bevy::render::render_resource::Face::Back) },
            alpha_mode: self.alpha_mode,
            unlit: self.unlit,
            ..StandardMaterial::default()
        };
        if self.foliage && !self.unlit && foliage_transmission() {
            material.diffuse_transmission = LEAF_DIFFUSE_TRANSMISSION;
            material.thickness = LEAF_THICKNESS;
        }
        // Unlit surfaces skip lighting, so a normal map would only cost a fetch. Bevy uses it only
        // on meshes with tangents; `flip_normal_map_y` stays false (the cook wrote OpenGL Y), and
        // BC5 reconstructs Z (`TWO_COMPONENT_NORMAL_MAP`).
        if !self.unlit {
            material.normal_map_texture = self.normal_texture.and_then(|p| texture(p, self.base_clamp, false));
        }

        // Fixed-function NIF shading is `texture * (emissive + lighting)`: the
        // emissive color glows *through the base texture*. Bevy adds
        // `emissive * emissive_texture` in nits, scaled by camera exposure
        // ([`set_emissive`]), so the base texture becomes the emissive texture
        // and the color is scaled to read as "fully lit" at the scene
        // exposure. A glow map replaces both. Unlit surfaces bypass PBR lighting.
        if !self.unlit {
            let emissive = self.emissive;
            if let Some(glow) = self.glow_texture.and_then(|p| texture(p, [false, false], true)) {
                material.emissive_texture = Some(glow);
                set_emissive(&mut material, LinearRgba::WHITE);
            } else if emissive.red + emissive.green + emissive.blue > 0.0 {
                material.emissive_texture.clone_from(&material.base_color_texture);
                set_emissive(&mut material, emissive);
            }
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

/// Loader settings for model textures: sRGB colour (`srgb`; false for normal maps, and the
/// loader's flag overrides the file's own format), linear filtering, optional S/T clamp.
pub(crate) fn image_settings_clamped(settings: &mut ImageLoaderSettings, clamp: [bool; 2], srgb: bool) {
    let mode = |clamped: bool| {
        if clamped {
            ImageAddressMode::ClampToEdge
        } else {
            ImageAddressMode::Repeat
        }
    };
    settings.is_srgb = srgb;
    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: mode(clamp[0]),
        address_mode_v: mode(clamp[1]),
        ..ImageSamplerDescriptor::linear()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn surface(foliage: bool, alpha_mode: AlphaMode, unlit: bool) -> Surface<'static> {
        Surface {
            base_color: Color::WHITE,
            base_texture: None,
            base_clamp: [false, false],
            glow_texture: None,
            normal_texture: None,
            emissive: LinearRgba::BLACK,
            alpha_mode,
            double_sided: true,
            unlit,
            foliage,
        }
    }

    /// Leaf cutouts transmit light (the default A/B state); other surfaces, unlit leaves
    /// included, keep Bevy's opaque default.
    #[test]
    fn only_lit_leaf_surfaces_get_diffuse_transmission() {
        assert!(foliage_transmission(), "on by default");
        let leaf = surface(true, AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF), false).to_material(|_, _, _| None);
        assert!((leaf.diffuse_transmission - LEAF_DIFFUSE_TRANSMISSION).abs() < f32::EPSILON);
        assert!((leaf.thickness - LEAF_THICKNESS).abs() < f32::EPSILON);
        assert!(leaf.diffuse_transmission > 0.0 && leaf.diffuse_transmission <= 1.0, "a share of the diffuse light");
        assert!(leaf.specular_transmission.abs() < f32::EPSILON, "no refraction path");
        let wall = surface(false, AlphaMode::Opaque, false).to_material(|_, _, _| None);
        assert!(wall.diffuse_transmission.abs() < f32::EPSILON && wall.thickness.abs() < f32::EPSILON);
        let unlit = surface(true, AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF), true).to_material(|_, _, _| None);
        assert!(unlit.diffuse_transmission.abs() < f32::EPSILON);
    }
}
