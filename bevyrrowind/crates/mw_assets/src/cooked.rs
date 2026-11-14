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

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Weak};

use bevy::asset::LoadState;
use bevy::camera::visibility::VisibilityRange;
use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;
use mw_bake::extras;
use sf_model::{AlphaMode as SfAlpha, LodScreen, lod_ranges};
use sf_model::steps::MeshCookStep;
use sf_model_bevy::{CookedModelAsset, SfMeshLoaderSettings};
use sf_sky_bevy::{NoRayTracing, RayTracingMesh};

use crate::{SharedVfs, Surface, image_settings_clamped, resolve_texture};

/// The cooked LOD level the ray tracer sees (viewer and streamed interiors).
/// Interiors are small enough for full detail; exteriors will want a
/// reduced level.
pub const RAY_TRACING_LOD: usize = 0;

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

/// Canonical identity of a resolved cooked surface, so identical materials
/// across thousands of static models share a single `StandardMaterial` asset
/// and bindless slab slot (`H1`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SurfaceKey {
    base_color: [u32; 4],
    base_texture: Option<String>,
    base_clamp: [bool; 2],
    glow_texture: Option<String>,
    normal_texture: Option<String>,
    /// The resolved ORM file and whether it binds occlusion too.
    orm: Option<(String, bool)>,
    /// The depth companion's served path (tx-pom).
    depth_texture: Option<String>,
    emissive: [u32; 3],
    alpha: (u8, u32),
    double_sided: bool,
    unlit: bool,
    foliage: bool,
    tint: Option<usize>,
}

/// Materials made for cooked models, shared globally across identical surfaces,
/// and the unique textures they load.
#[derive(Resource, Default)]
pub struct CookedMaterials {
    /// (model, tinted level or `None`) -> one material per model material.
    by_model: HashMap<MaterialKey, Vec<Handle<StandardMaterial>>>,
    by_surface: HashMap<SurfaceKey, Handle<StandardMaterial>>,
    /// (resolved path, clamp, sRGB) -> the loading texture.
    texture_cache: HashMap<(String, [bool; 2], bool), Handle<Image>>,
    textures: Vec<Handle<Image>>,
    /// The companions of the VFS they were read from (re-read if [`SharedVfs`] is replaced).
    companions: Option<(Weak<sf_vfs::Vfs>, TextureCompanions)>,
}

/// The normal-map and ORM companions the mounted bake layer lists
/// ([`mw_bake::cook::read_companions`], [`mw_bake::cook::read_orms`]), read once per VFS instead
/// of per material: base texture stem -> companion. Empty without a bake or without a pack
/// shipping `_n`/`_nh`/`_spec` maps.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextureCompanions {
    normals: Arc<BTreeMap<String, String>>,
    /// ORM-slot companions ([`mw_bake::cook::read_orms`], TS-2).
    orms: Arc<BTreeMap<String, mw_bake::cook::OrmCompanion>>,
    /// Depth companions ([`mw_bake::cook::read_depths`], tx-pom): base stem -> served depth map.
    depths: Arc<BTreeMap<String, String>>,
}

impl TextureCompanions {
    /// The companions `vfs` serves (one read and decode of each of the layer's maps).
    pub fn read(vfs: &sf_vfs::Vfs) -> Self {
        Self {
            normals: Arc::new(mw_bake::cook::read_companions(vfs)),
            orms: Arc::new(mw_bake::cook::read_orms(vfs)),
            depths: Arc::new(mw_bake::cook::read_depths(vfs)),
        }
    }

    /// The normal-map companion of a base texture reference ([`mw_bake::cook::companion_of`]).
    pub fn of(&self, reference: &str) -> Option<&str> {
        mw_bake::cook::companion_of(&self.normals, reference)
    }

    /// The ORM companion of a base texture reference ([`mw_bake::cook::orm_of`]).
    pub fn orm_of(&self, reference: &str) -> Option<&mw_bake::cook::OrmCompanion> {
        mw_bake::cook::orm_of(&self.orms, reference)
    }

    /// The depth map of a base texture reference ([`mw_bake::cook::depth_of`]): its served path.
    pub fn depth_of(&self, reference: &str) -> Option<&str> {
        mw_bake::cook::depth_of(&self.depths, reference)
    }

    /// These companions with `orms` as the ORM ones.
    #[must_use]
    pub fn with_orms(self, orms: BTreeMap<String, mw_bake::cook::OrmCompanion>) -> Self {
        Self { orms: Arc::new(orms), ..self }
    }

    /// These companions with `depths` (base stem -> served depth map) as the depth ones.
    #[must_use]
    pub fn with_depths(self, depths: BTreeMap<String, String>) -> Self {
        Self { depths: Arc::new(depths), ..self }
    }

    /// Base textures with a depth companion (the texture lab's panel).
    pub fn depth_count(&self) -> usize {
        self.depths.len()
    }
}

/// Normal-map companions only.
impl From<BTreeMap<String, String>> for TextureCompanions {
    fn from(map: BTreeMap<String, String>) -> Self {
        Self { normals: Arc::new(map), orms: Arc::default(), depths: Arc::default() }
    }
}

impl CookedMaterials {
    fn companions(&mut self, vfs: &Arc<sf_vfs::Vfs>) -> TextureCompanions {
        let current = Arc::downgrade(vfs);
        match &self.companions {
            Some((read_from, companions)) if Weak::ptr_eq(read_from, &current) => companions.clone(),
            _ => {
                let companions = TextureCompanions::read(vfs);
                self.companions = Some((current, companions.clone()));
                companions
            }
        }
    }

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

/// The NIF surface a cooked material describes (inverse of the importer's mapping), with the base
/// texture's normal-map and ORM companions if the bake serves them.
fn surface<'a>(m: &'a sf_model::Material, vfs: &sf_vfs::Vfs, companions: &'a TextureCompanions) -> Surface<'a> {
    let [red, green, blue, alpha] = m.base_color;
    let [emissive_r, emissive_g, emissive_b] = m.emissive;
    let additive = m.extras.get(extras::BLEND).is_some_and(|mode| mode == "additive");
    // `--off leafmask`: cutouts promoted from an authored blend draw blended.
    let cutout = m.extras.get(extras::BLEND).map(String::as_str);
    let blended = !crate::cutout_foliage() && cutout == Some("cutout");
    let candidate = cutout == Some("cutout_candidate") && crate::texture_is_cutout(vfs, m.base_color_texture.as_deref());
    let base_clamp = match m.extras.get(extras::BASE_CLAMP).map(String::as_str) {
        Some("st") => [true, true],
        Some("s") => [true, false],
        Some("t") => [false, true],
        _ => [false, false],
    };
    let orm = m.base_color_texture.as_deref().and_then(|base| companions.orm_of(base));
    let alpha_mode = match m.alpha {
        SfAlpha::Opaque => AlphaMode::Opaque,
        SfAlpha::Mask(_) if blended => AlphaMode::Blend,
        SfAlpha::Mask(cutoff) => AlphaMode::Mask(cutoff),
        SfAlpha::Blend if additive => AlphaMode::Add,
        SfAlpha::Blend if candidate => AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF),
        SfAlpha::Blend => AlphaMode::Blend,
    };
    Surface {
        base_color: Color::linear_rgba(red, green, blue, alpha),
        base_texture: m.base_color_texture.as_deref(),
        base_clamp,
        glow_texture: m.extras.get(extras::GLOW_TEXTURE).map(String::as_str),
        normal_texture: m.base_color_texture.as_deref().and_then(|base| companions.of(base)),
        depth_texture: m.base_color_texture.as_deref().and_then(|base| companions.depth_of(base)),
        orm,
        emissive: LinearRgba::rgb(emissive_r, emissive_g, emissive_b),
        // The leaf rule, as the NIF path's (`Surface::foliage`).
        foliage: matches!(cutout, Some("cutout" | "cutout_candidate")) && matches!(alpha_mode, AlphaMode::Mask(_)),
        alpha_mode,
        double_sided: m.double_sided,
        unlit: m.unlit,
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
                let made = make_materials(model, tint, &server, &vfs, &root.source, &mut cache, &mut materials);
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
        commands.entity(entity).try_insert(CookedSpawned { fallback: false });
    }
}

fn fallback(commands: &mut Commands, server: &AssetServer, entity: Entity, root: &CookedModelRoot) {
    commands
        .entity(entity)
        .try_insert((WorldAssetRoot(server.load(root.source.clone())), CookedSpawned { fallback: true }));
}

/// One cooked material as Bevy's, mapped like the NIF path's: the material
/// mapper for streamed statics (`sf_world_bevy::ModelMaterials`). Textures
/// resolve through `vfs` and load from the default asset source. No companion maps (normal, ORM): callers that
/// want them read [`TextureCompanions`] once and use [`cooked_material_with`] or [`material_mapper`].
pub fn cooked_material(m: &sf_model::Material, vfs: &sf_vfs::Vfs, server: &AssetServer) -> StandardMaterial {
    cooked_material_with(m, vfs, &TextureCompanions::default(), server)
}

/// [`cooked_material`] with the base texture's normal-map and ORM companions from `companions`.
pub fn cooked_material_with(
    m: &sf_model::Material,
    vfs: &sf_vfs::Vfs,
    companions: &TextureCompanions,
    server: &AssetServer,
) -> StandardMaterial {
    surface(m, vfs, companions).to_material(|path, clamp, srgb| {
        let resolved = resolve_texture(path, vfs, "cooked model")?;
        Some(server.load_builder().with_settings(move |s| image_settings_clamped(s, clamp, srgb)).load(resolved))
    })
}

/// The material mapper for streamed statics (`sf_world_bevy::ModelMaterials`, wrap it in an `Arc`):
/// [`cooked_material_with`] over the companions `vfs` serves, read once here.
pub fn material_mapper(vfs: Arc<sf_vfs::Vfs>) -> impl Fn(&sf_model::Material, &AssetServer) -> StandardMaterial + Send + Sync + 'static {
    let companions = TextureCompanions::read(&vfs);
    move |m, server| cooked_material_with(m, &vfs, &companions, server)
}

fn surface_key(
    s: &Surface<'_>,
    base_resolved: Option<String>,
    glow_resolved: Option<String>,
    normal_resolved: Option<String>,
    orm_resolved: Option<(String, bool)>,
    tint: Option<usize>,
) -> SurfaceKey {
    let bc = s.base_color.to_linear();
    let alpha = match s.alpha_mode {
        AlphaMode::Opaque => (0, 0),
        AlphaMode::Mask(c) => (1, c.to_bits()),
        AlphaMode::Blend => (2, 0),
        AlphaMode::Add => (3, 0),
        AlphaMode::Premultiplied => (4, 0),
        AlphaMode::Multiply => (5, 0),
        AlphaMode::AlphaToCoverage => (6, 0),
    };
    SurfaceKey {
        base_color: [bc.red.to_bits(), bc.green.to_bits(), bc.blue.to_bits(), bc.alpha.to_bits()],
        base_texture: base_resolved,
        base_clamp: s.base_clamp,
        glow_texture: glow_resolved,
        normal_texture: normal_resolved,
        orm: orm_resolved,
        depth_texture: s.depth_texture.map(str::to_owned),
        emissive: [s.emissive.red.to_bits(), s.emissive.green.to_bits(), s.emissive.blue.to_bits()],
        alpha,
        double_sided: s.double_sided,
        unlit: s.unlit,
        foliage: s.foliage,
        tint,
    }
}

fn make_materials(
    model: &CookedModelAsset,
    tint: Option<usize>,
    server: &AssetServer,
    vfs: &SharedVfs,
    source: &str,
    cache: &mut CookedMaterials,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Handle<StandardMaterial>> {
    let companions = cache.companions(&vfs.0);
    model
        .materials
        .iter()
        .map(|m| {
            let s = surface(m, &vfs.0, &companions);
            let resolve = |p: &str| resolve_texture(p, &vfs.0, source);
            let key = surface_key(
                &s,
                s.base_texture.and_then(resolve),
                s.glow_texture.and_then(resolve),
                s.normal_texture.and_then(resolve),
                s.orm.and_then(|c| Some((resolve(&c.file)?, c.occlusion))),
                tint,
            );
            if let Some(existing) = cache.by_surface.get(&key) {
                return existing.clone();
            }
            let mut material = s.to_material(|path, clamp, srgb| {
                let resolved = resolve_texture(path, &vfs.0, source)?;
                if let Some(h) = cache.texture_cache.get(&(resolved.clone(), clamp, srgb)) {
                    return Some(h.clone());
                }
                let handle: Handle<Image> = server
                    .load_builder()
                    .with_settings(move |st| image_settings_clamped(st, clamp, srgb))
                    .load(resolved.clone());
                cache.texture_cache.insert((resolved, clamp, srgb), handle.clone());
                cache.textures.push(handle.clone());
                Some(handle)
            });
            if let Some(level) = tint {
                // Flat and unlit, so the level reads at any light level.
                let color = LEVEL_TINTS[level.min(LEVEL_TINTS.len() - 1)];
                material.base_color = color.with_alpha(material.base_color.alpha());
                material.unlit = true;
                material.emissive = LinearRgba::BLACK;
            }
            let handle = materials.add(material);
            cache.by_surface.insert(key, handle.clone());
            handle
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn material(base: &str, unlit: bool) -> sf_model::Material {
        sf_model::Material { base_color_texture: Some(base.to_owned()), unlit, ..sf_model::Material::default() }
    }

    /// T4: a cooked material gets `normal_map_texture` iff the bake lists a companion for its base
    /// texture (any spelling of the reference), loaded linear with the base slot's clamp; the base
    /// texture stays sRGB. Unlit surfaces take none.
    #[test]
    fn normal_map_iff_the_base_texture_has_a_companion() {
        let vfs = sf_vfs::Vfs::builder().build();
        assert_eq!(TextureCompanions::read(&vfs), TextureCompanions::default(), "no bake layer, no companions");
        let companions =
            TextureCompanions::from(BTreeMap::from([("textures/tx_wall".to_owned(), "textures/tx_wall_n.png".to_owned())]));
        let build = |m: &sf_model::Material| {
            let mut loads = Vec::new();
            let made = surface(m, &vfs, &companions).to_material(|path, clamp, srgb| {
                loads.push((path.to_owned(), clamp, srgb));
                Some(Handle::default())
            });
            (made, loads)
        };

        let (wall, loads) = build(&material("Textures\\TX_Wall.dds", false));
        assert!(wall.normal_map_texture.is_some());
        assert!(!wall.flip_normal_map_y, "the cook wrote OpenGL Y");
        assert_eq!(
            loads,
            [
                ("Textures\\TX_Wall.dds".to_owned(), [false, false], true),
                ("textures/tx_wall_n.png".to_owned(), [false, false], false),
            ]
        );

        let (other, loads) = build(&material("textures/tx_other.dds", false));
        assert!(other.normal_map_texture.is_none());
        assert_eq!(loads, [("textures/tx_other.dds".to_owned(), [false, false], true)]);

        let (unlit, _) = build(&material("textures/tx_wall.dds", true));
        assert!(unlit.normal_map_texture.is_none());

        // Without companions (the plain `cooked_material` path) nothing changes.
        let none = TextureCompanions::default();
        assert!(surface(&material("textures/tx_wall.dds", false), &vfs, &none).normal_texture.is_none());
    }

    /// TS-2: a cooked material gets `metallic_roughness_texture` (and `occlusion_texture` for a
    /// packed ORM, the same image) iff the bake lists an ORM companion for its base texture, loaded
    /// linear with the base slot's clamp, with factors 1.0 and reflectance 0.5; a converted
    /// specular map binds no occlusion. Without one the material is the old one exactly.
    #[test]
    fn orm_textures_iff_the_base_texture_has_an_orm_companion() {
        use mw_bake::cook::OrmCompanion;
        let vfs = sf_vfs::Vfs::builder().build();
        let orm = |file: &str, occlusion| OrmCompanion { file: file.to_owned(), occlusion };
        let companions = TextureCompanions::default().with_orms(BTreeMap::from([
            ("textures/tx_armor".to_owned(), orm("textures/tx_armor_spec.dds", true)),
            ("textures/tx_wall".to_owned(), orm("textures/tx_wall_spec.png", false)),
        ]));
        let build = |m: &sf_model::Material, companions: &TextureCompanions| {
            let mut loads = Vec::new();
            let made = surface(m, &vfs, companions).to_material(|path, clamp, srgb| {
                loads.push((path.to_owned(), clamp, srgb));
                Some(Handle::default())
            });
            (made, loads)
        };

        let (armor, loads) = build(&material("Textures\\TX_Armor.dds", false), &companions);
        assert!(armor.metallic_roughness_texture.is_some());
        assert_eq!(armor.occlusion_texture, armor.metallic_roughness_texture, "one image, both slots");
        assert_eq!((armor.metallic, armor.perceptual_roughness, armor.reflectance), (1.0, 1.0, 0.5));
        assert_eq!(loads[1], ("textures/tx_armor_spec.dds".to_owned(), [false, false], false), "linear");

        let (wall, _) = build(&material("textures/tx_wall.dds", false), &companions);
        assert!(wall.metallic_roughness_texture.is_some() && wall.occlusion_texture.is_none());

        // No companion, no bake, or unlit: the material is the one without ORM, field for field.
        let (plain, loads) = build(&material("textures/tx_other.dds", false), &companions);
        let (before, _) = build(&material("textures/tx_other.dds", false), &TextureCompanions::default());
        assert_eq!(loads.len(), 1);
        assert!(plain.metallic_roughness_texture.is_none() && plain.occlusion_texture.is_none());
        assert_eq!(
            (plain.metallic, plain.perceptual_roughness, plain.reflectance),
            (before.metallic, before.perceptual_roughness, before.reflectance)
        );
        assert_eq!((before.metallic, before.perceptual_roughness, before.reflectance), (0.0, 0.85, 0.3));
        let (unlit, _) = build(&material("textures/tx_armor.dds", true), &companions);
        assert!(unlit.metallic_roughness_texture.is_none());
    }

    /// tx-pom: a cooked material gets `depth_map` (loaded linear with the base slot's clamp) and the
    /// preset's parallax iff the bake lists a depth companion for its base texture; without one,
    /// or unlit, the parallax fields are Bevy's defaults, field for field.
    #[test]
    fn depth_map_iff_the_base_texture_has_a_depth_companion() {
        let vfs = sf_vfs::Vfs::builder().build();
        let companions = TextureCompanions::default()
            .with_depths(BTreeMap::from([("textures/tx_wall".to_owned(), "textures/tx_wall_nh_depth.ktx2".to_owned())]));
        let build = |m: &sf_model::Material| {
            let mut loads = Vec::new();
            let made = surface(m, &vfs, &companions).to_material(|path, clamp, srgb| {
                loads.push((path.to_owned(), clamp, srgb));
                Some(Handle::default())
            });
            (made, loads)
        };
        let (wall, loads) = build(&material("Textures\\TX_Wall.dds", false));
        assert!(wall.depth_map.is_some());
        assert_eq!(loads[1], ("textures/tx_wall_nh_depth.ktx2".to_owned(), [false, false], false), "linear");
        let preset = crate::parallax().expect("High by default");
        assert_eq!((wall.parallax_mapping_method, wall.max_parallax_layer_count), (preset.method, preset.max_layers));
        assert!((wall.parallax_depth_scale - crate::PARALLAX_DEPTH_SCALE).abs() < f32::EPSILON);

        let plain = StandardMaterial::default();
        for (m, what) in [(build(&material("textures/tx_other.dds", false)).0, "no companion"), (build(&material("textures/tx_wall.dds", true)).0, "unlit")] {
            assert!(m.depth_map.is_none(), "{what}");
            assert_eq!(
                (m.parallax_depth_scale, m.parallax_mapping_method, m.max_parallax_layer_count),
                (plain.parallax_depth_scale, plain.parallax_mapping_method, plain.max_parallax_layer_count),
                "{what}"
            );
        }
    }
}
