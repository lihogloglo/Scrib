//! Material classes for statics on the entity path (static-scene.md §5, slice S2).
//!
//! Every static material folds into one of [`MaterialClass`]'s classes: its [`MaterialTraits`] are
//! read from what the pipeline sees (alpha mode, face culling; wind is not in yet), and
//! [`fold`] puts every field that only selects a shader variant into the class's canonical value, so
//! materials of one class produce the same Bevy pipeline key ([`StandardMaterialKey`] plus the
//! alpha-mode bits of the mesh key) and differ only in per-material data (textures, tint, cutoff,
//! emissive, unlit, fog flag: the bindless material table). Nothing that changes the look is
//! touched: a field whose canonical value would change the picture is left as it is and reported as
//! a [`Split`].
//!
//! Key-affecting `StandardMaterial` fields and what folding does with them:
//! - `cull_mode`: from the class (back faces culled unless two-sided). The class is read from
//!   `cull_mode` itself, so this never changes what is drawn; `Face::Front` has no class.
//! - `*_channel` (UV set of base, emissive, metallic-roughness, occlusion, normal map): `Uv0` when
//!   the matching texture is absent (the channel then selects nothing); a texture on another UV set
//!   is a [`Split::UvChannel`].
//! - `parallax_mapping_method`: Bevy's default when there is no depth map (unused then); with one,
//!   [`Split::DepthMap`].
//! - `normal_map_texture`: kept. Statics carry none today (2002 assets); a replacer with normal maps
//!   is a [`Split::NormalMap`] (a shared flat normal map for the rest of the class needs tangents on
//!   every mesh of the class, a bake change).
//! - `depth_bias`: zero when under one unit (Bevy truncates it to an integer for the key and the
//!   depth state, so the fraction selects nothing); otherwise kept, a [`Split::DepthBias`] (no
//!   static sets one today).
//! - `diffuse_transmission` (leaf cutouts, `--off foliagetransmission`): kept, [`Split::Leaf`]. One
//!   key for leaves and other masked surfaces would need a nonzero transmission on every masked
//!   material: Bevy then runs a second lighting lobe per light on all of them and renders them
//!   forward on the deferred (`--rt`) path, which changes how they are lit there.
//! - `clearcoat`, `anisotropy_strength`, `specular_transmission`: kept; statics never set them
//!   ([`Split::Layered`]).
//!
//! The A/B switch is [`crate::StreamingConfig::material_classes`] (the app's `--off matclass`).
//!
//! Gauges (`sf_diag::views::statics`): while `DiagCounters` exist, [`track_static_materials`]
//! follows the materials of live [`StaticPart`]s (distinct materials, classes, keys, pipeline
//! keys) and [`count_static_bind_groups`] reads which material bind groups hold them in the render
//! world. Bindless (`StandardMaterial`'s `#[bindless]`, Bevy's slab allocator) needs the device's
//! binding-array features and `max_binding_array_sampler_elements_per_shader_stage` ≥ 6 samplers ×
//! 2048 slab slots: desktop Vulkan has it, DX12's 2048-sampler heap does not (`statics/bindless`
//! says which).

use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bevy_asset::{AssetId, Assets, UntypedAssetId};
use bevy_ecs::prelude::*;
use bevy_material::{AlphaMode, OpaqueRendererMethod};
use bevy_mesh::UvChannel;
use bevy_pbr::{Material, MeshMaterial3d, ParallaxMappingMethod, StandardMaterial, StandardMaterialKey};
use bevy_render::extract_resource::ExtractResource;
use bevy_render::material_bind_groups::{MaterialBindGroupAllocator, MaterialBindGroupAllocators, RenderMaterialBindings};
use bevy_render::RenderApp;
use bevy_render::render_resource::Face;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::views::{StaticBindGroups, StaticMaterials};
use sf_scene::{MaterialClass, MaterialTraits};

/// A key-affecting field [`fold`] leaves as it is, because its class value would change the look.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Split {
    /// Leaf cutout with diffuse transmission (`StandardMaterialKey::DIFFUSE_TRANSMISSION`).
    Leaf,
    /// A normal map (`NORMAL_MAP`).
    NormalMap,
    /// A nonzero depth bias (the key's high 32 bits).
    DepthBias,
    /// A texture read from a UV set other than the first (`*_UV`).
    UvChannel,
    /// A depth map with relief mapping (`RELIEF_MAPPING`).
    DepthMap,
    /// Clearcoat, anisotropy or specular transmission.
    Layered,
}

/// What [`fold`] made of a material.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Folded {
    pub class: MaterialClass,
    /// Fields kept off the class key, sorted; empty when the material has exactly
    /// [`class_key`].
    pub splits: Vec<Split>,
}

/// The pipeline-relevant traits of a material as Bevy will draw it; `None` for one that is no
/// scene material (blended, additive, premultiplied, multiplied, alpha-to-coverage, or culling
/// front faces).
pub fn traits_of(material: &StandardMaterial) -> Option<MaterialTraits> {
    let masked = match material.alpha_mode {
        AlphaMode::Opaque => false,
        AlphaMode::Mask(_) => true,
        _ => return None,
    };
    let two_sided = match material.cull_mode {
        None => true,
        Some(Face::Back) => false,
        Some(Face::Front) => return None,
    };
    Some(MaterialTraits { masked, two_sided, wind: false })
}

/// The class of a material, if it has one ([`traits_of`]).
pub fn class_of(material: &StandardMaterial) -> Option<MaterialClass> {
    traits_of(material).map(MaterialClass::of)
}

/// The `StandardMaterialKey` every fully folded material of `class` has.
pub fn class_key(class: MaterialClass) -> StandardMaterialKey {
    if class.traits().two_sided { StandardMaterialKey::empty() } else { StandardMaterialKey::CULL_BACK }
}

/// The pipeline-selecting part of a material: what Bevy specializes its material pipeline on
/// (`StandardMaterialKey`, the alpha-mode bucket of the mesh key, the opaque render method).
/// Materials with equal `PipelineKey`s on the same mesh layout share a pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PipelineKey {
    pub key: u64,
    /// 0 opaque, 1 alpha-tested, 2 anything else (blended kinds).
    pub alpha: u8,
    /// 0 forward, 1 deferred, 2 the camera's default.
    pub method: u8,
}

impl PipelineKey {
    pub fn of(material: &StandardMaterial) -> Self {
        let alpha = match material.alpha_mode {
            AlphaMode::Opaque => 0,
            AlphaMode::Mask(_) => 1,
            _ => 2,
        };
        let method = match Material::opaque_render_method(material) {
            OpaqueRendererMethod::Forward => 0,
            OpaqueRendererMethod::Deferred => 1,
            OpaqueRendererMethod::Auto => 2,
        };
        Self { key: StandardMaterialKey::from(material).bits(), alpha, method }
    }
}

/// Folds `material` into its class (module docs); `None`, untouched, when it has none.
pub fn fold(material: &mut StandardMaterial) -> Option<Folded> {
    let class = class_of(material)?;
    let traits = class.traits();
    material.cull_mode = if traits.two_sided { None } else { Some(Face::Back) };
    let uv = |texture: bool, channel: &mut UvChannel| {
        if !texture {
            *channel = UvChannel::Uv0;
        }
    };
    uv(material.base_color_texture.is_some(), &mut material.base_color_channel);
    uv(material.emissive_texture.is_some(), &mut material.emissive_channel);
    uv(material.metallic_roughness_texture.is_some(), &mut material.metallic_roughness_channel);
    uv(material.occlusion_texture.is_some(), &mut material.occlusion_channel);
    uv(material.normal_map_texture.is_some(), &mut material.normal_map_channel);
    if material.depth_map.is_none() {
        material.parallax_mapping_method = ParallaxMappingMethod::default();
    }
    // Bevy keys (and biases the pipeline's depth state) on the bias truncated to an integer, so a
    // bias under one unit selects nothing.
    #[allow(clippy::cast_possible_truncation)] // Bevy's own conversion
    let bias = material.depth_bias as i32;
    if bias == 0 {
        material.depth_bias = 0.0;
    }

    let mut splits = Vec::new();
    if material.diffuse_transmission > 0.0 {
        splits.push(Split::Leaf);
    }
    if material.normal_map_texture.is_some() {
        splits.push(Split::NormalMap);
    }
    if bias != 0 {
        splits.push(Split::DepthBias);
    }
    let channels = [
        &material.base_color_channel,
        &material.emissive_channel,
        &material.metallic_roughness_channel,
        &material.occlusion_channel,
        &material.normal_map_channel,
    ];
    if channels.iter().any(|c| **c != UvChannel::Uv0) {
        splits.push(Split::UvChannel);
    }
    if material.depth_map.is_some() && matches!(material.parallax_mapping_method, ParallaxMappingMethod::Relief { .. }) {
        splits.push(Split::DepthMap);
    }
    if material.clearcoat > 0.0 || material.anisotropy_strength > 0.0 || material.specular_transmission > 0.0 {
        splits.push(Split::Layered);
    }
    Some(Folded { class, splits })
}

/// Marker on every mesh part the framework spawns for a static template
/// ([`crate::spawn_template_ranges`], per-placement and precombined alike): what the S2 gauges count.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StaticPart;

/// Live static materials (main world): reference counts by material, so the gauges follow
/// spawns and despawns in `O(changes)`.
#[derive(Resource, Default)]
pub(crate) struct StaticMaterialTracker {
    by_entity: HashMap<Entity, AssetId<StandardMaterial>>,
    counts: HashMap<AssetId<StandardMaterial>, u32>,
    dirty: bool,
    stats: StaticMaterials,
}

impl StaticMaterialTracker {
    fn release(&mut self, id: AssetId<StandardMaterial>) {
        if let Some(n) = self.counts.get_mut(&id) {
            *n -= 1;
            if *n == 0 {
                self.counts.remove(&id);
            }
        }
        self.dirty = true;
    }
}

/// The live static materials, extracted for the render world's bind-group count.
#[derive(Resource, Clone, Default)]
pub struct StaticMaterialIds(pub Arc<[UntypedAssetId]>);

impl ExtractResource<RenderApp> for StaticMaterialIds {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        source.clone()
    }
}

/// Static parts whose material was set or replaced since the last run.
type ChangedStaticMaterial = (With<StaticPart>, Changed<MeshMaterial3d<StandardMaterial>>);

/// Follows static parts' materials and sets the `statics/material*` gauges (main world half).
pub(crate) fn track_static_materials(
    counters: Res<DiagCounters>,
    mut tracker: ResMut<StaticMaterialTracker>,
    mut ids: ResMut<StaticMaterialIds>,
    changed: Query<(Entity, &MeshMaterial3d<StandardMaterial>), ChangedStaticMaterial>,
    mut removed_parts: RemovedComponents<StaticPart>,
    mut removed_materials: RemovedComponents<MeshMaterial3d<StandardMaterial>>,
    materials: Option<Res<Assets<StandardMaterial>>>,
) {
    let tracker = &mut *tracker;
    for entity in removed_parts.read().chain(removed_materials.read()) {
        if let Some(id) = tracker.by_entity.remove(&entity) {
            tracker.release(id);
        }
    }
    for (entity, material) in &changed {
        let id = material.id();
        match tracker.by_entity.insert(entity, id) {
            Some(old) if old == id => continue,
            Some(old) => tracker.release(old),
            None => {}
        }
        *tracker.counts.entry(id).or_default() += 1;
        tracker.dirty = true;
    }
    if tracker.dirty
        && let Some(materials) = materials
    {
        tracker.dirty = false;
        let mut classes = HashSet::new();
        let mut keys = HashSet::new();
        let mut pipelines = HashSet::new();
        let mut stats = StaticMaterials { materials: small(tracker.counts.len()), ..StaticMaterials::default() };
        for material in tracker.counts.keys().filter_map(|id| materials.get(*id)) {
            match class_of(material) {
                Some(class) => {
                    classes.insert(class);
                }
                None => stats.unclassed += 1,
            }
            let pipeline = PipelineKey::of(material);
            keys.insert(pipeline.key);
            pipelines.insert(pipeline);
        }
        stats.classes = small(classes.len());
        stats.keys = small(keys.len());
        stats.pipelines = small(pipelines.len());
        tracker.stats = stats;
        ids.0 = tracker.counts.keys().map(|id| id.untyped()).collect();
    }
    tracker.stats.record(&counters);
}

/// The render-world half: distinct material bind groups (bindless slabs) holding the live static
/// materials, read from Bevy's public `RenderMaterialBindings` and `MaterialBindGroupAllocators`.
pub(crate) fn count_static_bind_groups(
    ids: Option<Res<StaticMaterialIds>>,
    bindings: Option<Res<RenderMaterialBindings>>,
    allocators: Option<Res<MaterialBindGroupAllocators>>,
    counters: Res<DiagCounters>,
) {
    let (Some(ids), Some(bindings), Some(allocators)) = (ids, bindings, allocators) else { return }; // not extracted yet, or no PBR renderer
    let mut groups = HashSet::new();
    let mut bound = 0usize;
    for binding in ids.0.iter().filter_map(|id| bindings.get(id)) {
        groups.insert(binding.group.0);
        bound += 1;
    }
    let bindless =
        matches!(allocators.get(&TypeId::of::<StandardMaterial>()), Some(MaterialBindGroupAllocator::Bindless(_)));
    StaticBindGroups { bind_groups: small(groups.len()), bound: small(bound), bindless }.record(&counters);
}

fn small(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use bevy_asset::Handle;
    use bevy_color::{Color, LinearRgba};
    use sf_diag_bevy::sf_diag::Counters;

    use super::*;

    /// A static surface as `mw_assets::Surface::to_material` builds it: matte, textured, culling
    /// from `double_sided`.
    fn surface(alpha_mode: AlphaMode, double_sided: bool) -> StandardMaterial {
        StandardMaterial {
            base_color_texture: Some(Handle::default()),
            perceptual_roughness: 0.85,
            reflectance: 0.3,
            double_sided,
            cull_mode: if double_sided { None } else { Some(Face::Back) },
            alpha_mode,
            ..StandardMaterial::default()
        }
    }

    fn key(m: &StandardMaterial) -> u64 {
        StandardMaterialKey::from(m).bits()
    }

    #[test]
    fn traits_follow_alpha_and_culling() {
        let t = |m: &StandardMaterial| traits_of(m);
        let tr = |masked, two_sided| Some(MaterialTraits { masked, two_sided, wind: false });
        assert_eq!(t(&surface(AlphaMode::Opaque, false)), tr(false, false));
        assert_eq!(t(&surface(AlphaMode::Opaque, true)), tr(false, true));
        assert_eq!(t(&surface(AlphaMode::Mask(0.5), false)), tr(true, false));
        assert_eq!(t(&surface(AlphaMode::Mask(0.3), true)), tr(true, true));
        for blended in [AlphaMode::Blend, AlphaMode::Add, AlphaMode::Premultiplied, AlphaMode::Multiply, AlphaMode::AlphaToCoverage] {
            assert_eq!(t(&surface(blended, false)), None, "{blended:?} is no scene material");
        }
        let front = StandardMaterial { cull_mode: Some(Face::Front), ..surface(AlphaMode::Opaque, false) };
        assert_eq!(t(&front), None);
        assert_eq!(class_of(&surface(AlphaMode::Mask(0.5), true)), Some(MaterialClass::MaskedTwoSided));
    }

    /// Walls, banners, cutouts and glowing or unlit surfaces of one class end on one key.
    #[test]
    fn one_key_per_class_for_representative_statics() {
        let wall = surface(AlphaMode::Opaque, false);
        let tinted = StandardMaterial { base_color: Color::srgb(0.5, 0.4, 0.3), ..surface(AlphaMode::Opaque, false) };
        let window = StandardMaterial {
            emissive_texture: Some(Handle::default()),
            emissive: LinearRgba::rgb(2.0, 1.5, 1.0),
            ..surface(AlphaMode::Opaque, false)
        };
        let sign = StandardMaterial { unlit: true, fog_enabled: false, ..surface(AlphaMode::Opaque, false) };
        let untextured = StandardMaterial { base_color_texture: None, ..surface(AlphaMode::Opaque, false) };
        let banner = surface(AlphaMode::Opaque, true);
        let fence = surface(AlphaMode::Mask(0.5), false);
        let grate = surface(AlphaMode::Mask(0.25), true);
        let cases = [
            (wall, MaterialClass::Opaque),
            (tinted, MaterialClass::Opaque),
            (window, MaterialClass::Opaque),
            (sign, MaterialClass::Opaque),
            (untextured, MaterialClass::Opaque),
            (banner, MaterialClass::OpaqueTwoSided),
            (fence, MaterialClass::Masked),
            (grate, MaterialClass::MaskedTwoSided),
        ];
        for (mut m, class) in cases {
            let folded = fold(&mut m).expect("a scene material");
            assert_eq!(folded, Folded { class, splits: vec![] });
            assert_eq!(key(&m), class_key(class).bits(), "{class:?}");
        }
        assert_eq!(class_key(MaterialClass::Opaque).bits(), StandardMaterialKey::CULL_BACK.bits());
        assert_eq!(class_key(MaterialClass::Masked).bits(), StandardMaterialKey::CULL_BACK.bits());
        assert_eq!(class_key(MaterialClass::OpaqueTwoSided).bits(), StandardMaterialKey::empty().bits());
        assert_eq!(class_key(MaterialClass::MaskedTwoSided).bits(), StandardMaterialKey::empty().bits());
        // Opaque and masked share a StandardMaterialKey; the mesh key's alpha bits split them.
        let (o, k) = (PipelineKey::of(&surface(AlphaMode::Opaque, false)), PipelineKey::of(&surface(AlphaMode::Mask(0.5), false)));
        assert_eq!((o.key, o.alpha, k.key, k.alpha), (StandardMaterialKey::CULL_BACK.bits(), 0, StandardMaterialKey::CULL_BACK.bits(), 1));
        // Cutoffs are material data, not pipeline state.
        assert_eq!(PipelineKey::of(&surface(AlphaMode::Mask(0.1), true)), PipelineKey::of(&surface(AlphaMode::Mask(0.9), true)));
    }

    /// Shader-variant fields that select nothing are canonicalized; ones that would change the
    /// look stay and are reported.
    #[test]
    fn unused_variant_fields_fold_and_look_changes_split() {
        let mut stray = StandardMaterial {
            emissive_channel: UvChannel::Uv1,
            normal_map_channel: UvChannel::Uv1,
            parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 8 },
            depth_bias: -0.5,
            ..surface(AlphaMode::Opaque, false)
        };
        assert_ne!(key(&stray), StandardMaterialKey::CULL_BACK.bits());
        assert_eq!(fold(&mut stray).map(|f| f.splits), Some(vec![]));
        assert_eq!(key(&stray), StandardMaterialKey::CULL_BACK.bits());
        assert_eq!(stray.depth_bias.to_bits(), 0.0f32.to_bits());

        let mut leaf = StandardMaterial { diffuse_transmission: 0.5, thickness: 0.05, ..surface(AlphaMode::Mask(0.5), true) };
        assert_eq!(fold(&mut leaf), Some(Folded { class: MaterialClass::MaskedTwoSided, splits: vec![Split::Leaf] }));
        assert_eq!(key(&leaf), StandardMaterialKey::DIFFUSE_TRANSMISSION.bits(), "leaves keep their own key");
        assert!((leaf.diffuse_transmission - 0.5).abs() < f32::EPSILON);

        let mut decal = StandardMaterial {
            depth_bias: 4.0,
            normal_map_texture: Some(Handle::default()),
            base_color_channel: UvChannel::Uv1,
            ..surface(AlphaMode::Opaque, false)
        };
        let before = key(&decal);
        let folded = fold(&mut decal).expect("opaque");
        assert_eq!(folded.splits, vec![Split::NormalMap, Split::DepthBias, Split::UvChannel]);
        assert_eq!(key(&decal), before, "nothing that changes the look is touched");

        let mut blended = surface(AlphaMode::Blend, false);
        blended.emissive_channel = UvChannel::Uv1;
        assert_eq!(fold(&mut blended), None);
        assert_eq!(blended.emissive_channel, UvChannel::Uv1, "non-scene materials are left alone");
    }

    fn gauges(counters: &Counters) -> HashMap<String, f64> {
        let mut values = HashMap::new();
        counters.drain(|_, def, v| {
            if let Some(v) = v {
                values.insert(def.name.clone(), v);
            }
        });
        values
    }

    /// The gauges count distinct materials, classes, keys and pipeline keys of live static parts
    /// only, and follow despawns.
    #[test]
    fn tracker_counts_live_static_materials() {
        use sf_diag_bevy::sf_diag::views::statics;
        let mut world = World::new();
        let counters = Arc::new(Counters::new());
        world.insert_resource(DiagCounters(counters.clone()));
        world.init_resource::<StaticMaterialTracker>();
        world.init_resource::<StaticMaterialIds>();
        let mut assets = Assets::<StandardMaterial>::default();
        let wall = assets.add(surface(AlphaMode::Opaque, false));
        let plaster = assets.add(StandardMaterial { base_color: Color::srgb(0.9, 0.8, 0.7), ..surface(AlphaMode::Opaque, false) });
        let leaf = assets.add(StandardMaterial { diffuse_transmission: 0.5, ..surface(AlphaMode::Mask(0.5), true) });
        let glass = assets.add(surface(AlphaMode::Blend, false));
        let actor = assets.add(surface(AlphaMode::Opaque, true));
        world.insert_resource(assets);
        let mut schedule = Schedule::default();
        schedule.add_systems(track_static_materials);

        let first = world.spawn((StaticPart, MeshMaterial3d(wall.clone()))).id();
        world.spawn((StaticPart, MeshMaterial3d(wall)));
        world.spawn((StaticPart, MeshMaterial3d(plaster)));
        world.spawn((StaticPart, MeshMaterial3d(leaf)));
        let pane = world.spawn((StaticPart, MeshMaterial3d(glass))).id();
        world.spawn(MeshMaterial3d(actor)); // not a static part
        schedule.run(&mut world);
        let g = gauges(&counters);
        assert_eq!(g.get(statics::MATERIALS), Some(&4.0));
        assert_eq!(g.get(statics::CLASSES), Some(&2.0), "Opaque, MaskedTwoSided");
        assert_eq!(g.get(statics::KEYS), Some(&2.0), "CULL_BACK (walls, glass), DIFFUSE_TRANSMISSION (leaf)");
        assert_eq!(g.get(statics::PIPELINES), Some(&3.0), "opaque, masked leaf, blended");
        assert_eq!(g.get(statics::UNCLASSED), Some(&1.0));
        assert_eq!(world.resource::<StaticMaterialIds>().0.len(), 4);

        world.despawn(first);
        world.despawn(pane);
        schedule.run(&mut world);
        let g = gauges(&counters);
        assert_eq!(g.get(statics::MATERIALS), Some(&3.0), "the other wall part keeps its material");
        assert_eq!(g.get(statics::UNCLASSED), Some(&0.0));
        assert_eq!(g.get(statics::PIPELINES), Some(&2.0));
    }
}
