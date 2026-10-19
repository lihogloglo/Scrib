//! Cooked models (`.sfmesh` bake products) as entities.
//!
//! Spawn a placed model as `CookedModelRoot` with a handle from
//! `baked://mesh.cook/<path>.sfmesh`. Once the asset loads, its LOD chain
//! becomes child entities, one per level and material, each with a
//! `VisibilityRange` from the level's error (bake.md, "Runtime
//! distance"); Bevy crossfades between levels. Materials map exactly like
//! the NIF path's ([`crate::Surface`]), so both look the same. A model the
//! bake skipped (animated, skinned) or that fails to load falls back to
//! the source NIF.

use std::collections::HashMap;

use bevy::asset::LoadState;
use bevy::camera::visibility::VisibilityRange;
use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;
use mw_bake::extras;
use sf_model::{AlphaMode as SfAlpha, LodScreen, lod_ranges};
use sf_model::steps::MeshCookStep;
use sf_model_bevy::{CookedModelAsset, SfMeshLoaderSettings};
use sf_sky_bevy::{NoRayTracing, RayTracingMesh};

use crate::{SharedVfs, Surface, image_settings, resolve_texture};

/// Starts loading the cooked product of `model` (a VFS model path) from
/// the `baked://` source: Bevy vertex compression on, GPU-only meshes, and
/// with `ray_tracing_lod`, ray-tracing variants built from that level.
pub fn load(server: &AssetServer, model: &str, ray_tracing_lod: Option<usize>) -> Handle<CookedModelAsset> {
    let path = sf_model_bevy::baked_path(MeshCookStep::ID, model, Some(mw_bake::SFMESH_EXT));
    if ray_tracing_lod.is_none() {
        return server.load(path);
    }
    server
        .load_builder()
        .with_settings(move |s: &mut SfMeshLoaderSettings| s.ray_tracing_lod = ray_tracing_lod)
        .load(path)
}

/// A placed model to build from its cooked product.
#[derive(Component, Clone, Debug)]
pub struct CookedModelRoot {
    pub model: Handle<CookedModelAsset>,
    /// The source model's path, loaded instead when the cooked one can't be used.
    pub source: String,
    /// The placement's scale: LOD errors are in model units.
    pub scale: f32,
}

/// Added once a [`CookedModelRoot`] has its children (or fell back).
#[derive(Component, Clone, Copy, Debug)]
pub struct CookedSpawned {
    /// True when the source model was loaded instead.
    pub fallback: bool,
}

/// How LOD distances are chosen.
#[derive(Resource, Clone, Debug)]
pub struct CookedLodSettings {
    pub screen: LodScreen,
    /// The last level ends here (later: the streaming ring holding the object).
    pub max_distance: f32,
    /// Debug view: tint each level (LOD0 green, then yellow, orange, red, magenta).
    pub tint_levels: bool,
}

impl Default for CookedLodSettings {
    fn default() -> Self {
        Self { screen: LodScreen::default(), max_distance: 2000.0, tint_levels: false }
    }
}

const LEVEL_TINTS: [Color; 5] = [
    Color::srgb(0.3, 1.0, 0.3),
    Color::srgb(1.0, 1.0, 0.2),
    Color::srgb(1.0, 0.55, 0.1),
    Color::srgb(1.0, 0.2, 0.2),
    Color::srgb(1.0, 0.2, 1.0),
];

/// A cooked model and the LOD level its materials are tinted for (debug view).
type MaterialKey = (AssetId<CookedModelAsset>, Option<usize>);

/// Materials made for cooked models, shared by every instance, and the
/// textures they load.
#[derive(Resource, Default)]
pub struct CookedMaterials {
    /// (model, tinted level or `None`) -> one material per model material.
    by_model: HashMap<MaterialKey, Vec<Handle<StandardMaterial>>>,
    textures: Vec<Handle<Image>>,
}

impl CookedMaterials {
    /// Textures still loading (for "everything is loaded" checks).
    pub fn pending_textures(&self, server: &AssetServer) -> usize {
        self.textures
            .iter()
            .filter(|h| {
                !server.is_loaded_with_dependencies(*h) && !matches!(server.get_load_state(*h), Some(LoadState::Failed(_)))
            })
            .count()
    }
}

/// The NIF surface a cooked material describes (inverse of the importer's mapping).
fn surface<'a>(m: &'a sf_model::Material, vfs: &sf_vfs::Vfs) -> Surface<'a> {
    let [red, green, blue, alpha] = m.base_color;
    let [emissive_r, emissive_g, emissive_b] = m.emissive;
    let additive = m.extras.get(extras::BLEND).is_some_and(|mode| mode == "additive");
    // `--off leafmask`: cutouts promoted from an authored blend draw blended.
    let cutout = m.extras.get(extras::BLEND).map(String::as_str);
    let blended = !crate::cutout_foliage() && cutout == Some("cutout");
    let candidate = cutout == Some("cutout_candidate") && crate::texture_is_cutout(vfs, m.base_color_texture.as_deref());
    Surface {
        base_color: Color::linear_rgba(red, green, blue, alpha),
        base_texture: m.base_color_texture.as_deref(),
        glow_texture: m.extras.get(extras::GLOW_TEXTURE).map(String::as_str),
        emissive: LinearRgba::rgb(emissive_r, emissive_g, emissive_b),
        alpha_mode: match m.alpha {
            SfAlpha::Opaque => AlphaMode::Opaque,
            SfAlpha::Mask(_) if blended => AlphaMode::Blend,
            SfAlpha::Mask(cutoff) => AlphaMode::Mask(cutoff),
            SfAlpha::Blend if additive => AlphaMode::Add,
            SfAlpha::Blend if candidate => AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF),
            SfAlpha::Blend => AlphaMode::Blend,
        },
        double_sided: m.double_sided,
    }
}

#[allow(clippy::too_many_arguments)] // a system's parameters
pub(crate) fn spawn_cooked(
    mut commands: Commands,
    roots: Query<(Entity, &CookedModelRoot), Without<CookedSpawned>>,
    models: Res<Assets<CookedModelAsset>>,
    server: Res<AssetServer>,
    vfs: Res<SharedVfs>,
    settings: Res<CookedLodSettings>,
    mut cache: ResMut<CookedMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, root) in &roots {
        let Some(model) = models.get(&root.model) else {
            if matches!(server.get_load_state(&root.model), Some(LoadState::Failed(_))) {
                warn!("cooked {} failed to load; using the source model", root.source);
                fallback(&mut commands, &server, entity, root);
            }
            continue;
        };
        // Skinned cooks spawn through sf_model_bevy::spawn_skinned (actors), not here.
        if model.skipped.is_some() || model.skin.is_some() {
            fallback(&mut commands, &server, entity, root);
            continue;
        }

        let errors: Vec<f32> = model.lods.iter().map(|l| l.error * root.scale).collect();
        let ranges = (model.lods.len() > 1).then(|| lod_ranges(&errors, &settings.screen, settings.max_distance));
        for (level, lod) in model.lods.iter().enumerate() {
            let range = ranges.as_ref().and_then(|r| r.get(level));
            if range.is_some_and(|r| r.end <= r.start) {
                continue; // collapsed level: never visible
            }
            let tint = settings.tint_levels.then_some(level);
            let key = (root.model.id(), tint);
            if !cache.by_model.contains_key(&key) {
                let made = make_materials(model, tint, &server, &vfs, &root.source, &mut cache.textures, &mut materials);
                cache.by_model.insert(key, made);
            }
            let level_materials = &cache.by_model[&key];
            for (part, &(mesh, material)) in lod.parts.iter().enumerate() {
                let mut child = commands.spawn((
                    Mesh3d(model.meshes[mesh].clone()),
                    MeshMaterial3d(level_materials[material as usize].clone()),
                    Transform::IDENTITY,
                    ChildOf(entity),
                ));
                // The ray tracer ignores visibility ranges: it sees exactly
                // one level, the one its variants were built from.
                match &model.ray_tracing {
                    Some(rt) if rt.level == level => {
                        child.insert(RayTracingMesh(rt.meshes[part].clone()));
                    }
                    Some(_) => {
                        child.insert(NoRayTracing);
                    }
                    None => {}
                }
                if let Some(r) = range {
                    // Bands centered on each switch distance; level k's end band
                    // equals level k+1's start band, as Bevy's crossfade needs.
                    child.insert(VisibilityRange {
                        start_margin: (r.start - r.start_margin * 0.5)..(r.start + r.start_margin * 0.5),
                        end_margin: (r.end - r.end_margin * 0.5)..(r.end + r.end_margin * 0.5),
                        use_aabb: false,
                    });
                }
            }
        }
        commands.entity(entity).insert(CookedSpawned { fallback: false });
    }
}

fn fallback(commands: &mut Commands, server: &AssetServer, entity: Entity, root: &CookedModelRoot) {
    commands
        .entity(entity)
        .insert((WorldAssetRoot(server.load(root.source.clone())), CookedSpawned { fallback: true }));
}

/// One cooked material as Bevy's, mapped like the NIF path's: the material
/// mapper for streamed statics (`sf_world_bevy::ModelMaterials`). Textures
/// resolve through `vfs` and load from the default asset source.
pub fn cooked_material(m: &sf_model::Material, vfs: &sf_vfs::Vfs, server: &AssetServer) -> StandardMaterial {
    surface(m, vfs).to_material(|path| {
        let resolved = resolve_texture(path, vfs, "cooked model")?;
        Some(server.load_builder().with_settings(image_settings).load(resolved))
    })
}

fn make_materials(
    model: &CookedModelAsset,
    tint: Option<usize>,
    server: &AssetServer,
    vfs: &SharedVfs,
    source: &str,
    textures: &mut Vec<Handle<Image>>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Handle<StandardMaterial>> {
    model
        .materials
        .iter()
        .map(|m| {
            let mut material = surface(m, &vfs.0).to_material(|path| {
                let resolved = resolve_texture(path, &vfs.0, source)?;
                let handle: Handle<Image> = server.load_builder().with_settings(image_settings).load(resolved);
                textures.push(handle.clone());
                Some(handle)
            });
            if let Some(level) = tint {
                // Flat and unlit, so the level reads at any light level.
                let color = LEVEL_TINTS[level.min(LEVEL_TINTS.len() - 1)];
                material.base_color = color.with_alpha(material.base_color.alpha());
                material.unlit = true;
                material.emissive = LinearRgba::BLACK;
            }
            materials.add(material)
        })
        .collect()
}
