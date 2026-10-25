//! Placement spawning: a [`PlacementSpawner`] hook per [`PlacementClass`].
//!
//! The framework spawns statics itself ([`StaticSpawner`]): one entity per
//! LOD level and material part of the cooked model, each with a
//! `VisibilityRange` from [`sf_model::lod_ranges`] (projected error, bake.md
//! "Runtime distance"), so Bevy draws the right level with dithered
//! crossfades. Content packs register their own hooks for actors, lights,
//! doors, and for models the bake didn't cook (the runtime importer path).
//!
//! Spawners run inside the spawn stage with direct `World` access, so their
//! measured cost is the real spawn cost (the budget is honest).

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use bevy_asset::{AssetServer, Handle, UntypedHandle};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, RenderLayers, VisibilityRange};
use bevy_color::{Color, LinearRgba};
use bevy_ecs::prelude::*;
use bevy_material::AlphaMode;
use bevy_math::{IVec3, Vec3};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_render::render_resource::Face;
use bevy_transform::components::Transform;
use sf_model::{LodRange, LodScreen, lod_ranges};
use sf_world::{IndexedPlacement, Level, PlacementClass, PlacementFlags, WorldspaceId};

/// A cooked model, ready to instance: every level's parts with their meshes
/// and (deduplicated) materials.
#[derive(Debug)]
pub struct ModelTemplate {
    /// LOD0 first.
    pub lods: Vec<TemplateLod>,
    /// Model-space bounds of LOD0 (used as every part's `Aabb`, so Bevy
    /// never needs the mesh data on the CPU).
    pub aabb: Aabb,
}

#[derive(Debug)]
pub struct TemplateLod {
    /// Deviation from LOD0 in model-space meters.
    pub error: f32,
    pub parts: Vec<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

/// A placement's model as the assets stage left it.
#[derive(Debug, Clone)]
pub enum LoadedModel {
    Cooked(Arc<ModelTemplate>),
    /// Not baked, or skipped by `mesh.cook` (animated, skinned): the
    /// content pack's uncooked hook spawns it, if there is one.
    Uncooked(Arc<str>),
}

/// What a spawner gets for one placement.
pub struct SpawnCx<'a> {
    pub worldspace: &'a WorldspaceId,
    /// The streaming cell and the level being loaded (`Far` or `Near`).
    pub cell: IVec3,
    pub level: Level,
    /// Spawn under this entity (it is revealed when the level completes).
    pub parent: Entity,
    pub layers: &'a RenderLayers,
    /// The placement's transform relative to the parent.
    pub local: Transform,
    pub model: Option<&'a LoadedModel>,
    /// Unload radius of the ring holding this placement: the end of its
    /// last LOD level.
    pub max_distance: f32,
    /// Width of a dithered fade-out band ending the last level at
    /// `max_distance` (0: abrupt). Set when a far-field proxy takes over
    /// from this placement (far-field.md, "Lead bands").
    pub end_band: f32,
    pub lod: &'a LodScreen,
}

/// What activation hooks get (R0: colliders, scripts, AI).
pub struct ActivateCx<'a> {
    pub worldspace: &'a WorldspaceId,
    pub cell: IVec3,
    /// The cell's entity (high precision, parent of the level groups).
    pub cell_entity: Entity,
    /// A deactivation that discards the cell's live state: the resident
    /// cells are being reloaded after a save load (`Streaming::reload_resident`),
    /// so hooks clean up but write nothing back (the overlay is the loaded
    /// one). Always false on activation.
    pub discard: bool,
}

/// Spawns (and activates) placements of one class.
pub trait PlacementSpawner: Send + Sync + 'static {
    /// Spawns one placement; returns the number of entities spawned.
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32;
    /// Whether the placement's model must be loaded before [`spawn`](Self::spawn).
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        placement.placement.model.is_some()
    }
    /// An asset [`spawn`](Self::spawn) will instance for this placement that
    /// the framework doesn't load itself (a runtime-imported model, say).
    /// The assets stage starts it once the placement's cooked model has
    /// resolved (`model`) and waits until it and its dependencies have
    /// loaded or failed, so the level reveals complete (weather.md bug B3).
    fn prepare(&self, _assets: &AssetServer, _placement: &IndexedPlacement, _model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        None
    }
    /// The cell entered R0.
    fn activate(&self, _world: &mut World, _cx: &ActivateCx<'_>, _placement: &IndexedPlacement) {}
    /// The cell left R0.
    fn deactivate(&self, _world: &mut World, _cx: &ActivateCx<'_>, _placement: &IndexedPlacement) {}
}

/// Hooks by class. Defaults: [`StaticSpawner`] for every class but
/// `Marker` (nothing to draw); no uncooked hook.
#[derive(Resource, Clone)]
pub struct PlacementSpawners {
    by_class: HashMap<PlacementClass, Arc<dyn PlacementSpawner>>,
    default: Option<Arc<dyn PlacementSpawner>>,
    uncooked: Option<Arc<dyn PlacementSpawner>>,
}

impl Default for PlacementSpawners {
    fn default() -> Self {
        let mut by_class: HashMap<PlacementClass, Arc<dyn PlacementSpawner>> = HashMap::new();
        let statics: Arc<dyn PlacementSpawner> = Arc::new(StaticSpawner);
        for class in [PlacementClass::Static, PlacementClass::Dynamic, PlacementClass::Actor, PlacementClass::Light, PlacementClass::Door] {
            by_class.insert(class, statics.clone());
        }
        Self { by_class, default: Some(statics), uncooked: None }
    }
}

impl PlacementSpawners {
    /// Replaces the hook of one class.
    pub fn set(&mut self, class: PlacementClass, spawner: Arc<dyn PlacementSpawner>) -> &mut Self {
        self.by_class.insert(class, spawner);
        self
    }

    /// Spawns nothing for this class.
    pub fn clear(&mut self, class: PlacementClass) -> &mut Self {
        self.by_class.remove(&class);
        self
    }

    /// The hook for classes without their own (`Custom` classes).
    pub fn set_default(&mut self, spawner: Option<Arc<dyn PlacementSpawner>>) -> &mut Self {
        self.default = spawner;
        self
    }

    /// The hook for models the bake didn't cook (the runtime importer).
    pub fn set_uncooked(&mut self, spawner: Option<Arc<dyn PlacementSpawner>>) -> &mut Self {
        self.uncooked = spawner;
        self
    }

    pub fn for_class(&self, class: PlacementClass) -> Option<&Arc<dyn PlacementSpawner>> {
        match class {
            PlacementClass::Marker => self.by_class.get(&class),
            _ => self.by_class.get(&class).or(self.default.as_ref()),
        }
    }

    pub fn uncooked(&self) -> Option<&Arc<dyn PlacementSpawner>> {
        self.uncooked.as_ref()
    }

    /// The hook for `placement` once its model resolved to `model`: the
    /// uncooked hook for models the bake didn't cook, else its class's.
    pub fn spawner_for(&self, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<&Arc<dyn PlacementSpawner>> {
        match model {
            Some(LoadedModel::Uncooked(_)) => self.uncooked(),
            _ => self.for_class(placement.placement.class),
        }
    }
}

/// Puts a placement's visibility range on a subtree whose meshes appear
/// later (a runtime-imported model spawned as a scene): every `Mesh3d`
/// below gets a `VisibilityRange` ending at `end` with a dithered fade
/// `[e, e + band]`, where `e = end + |node origin - root origin|`. The extra
/// offset keeps a part from fading before a far-field proxy measured from
/// the placement origin is fully drawn (far-field.md, "Lead bands"). Spawners
/// insert it on the root with [`SpawnCx::max_distance`] and
/// [`SpawnCx::end_band`] (weather.md bug B1).
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SubtreeRange {
    pub end: f32,
    pub band: f32,
}

impl SubtreeRange {
    pub fn of(cx: &SpawnCx<'_>) -> Self {
        Self { end: cx.max_distance, band: cx.end_band }
    }

    /// The range of a mesh whose origin is `offset` meters from the root's.
    pub fn range_at(&self, offset: f32) -> Option<VisibilityRange> {
        let end = self.end + offset + self.band * 0.5;
        visibility_range(&LodRange { start: 0.0, end, start_margin: 0.0, end_margin: self.band })
    }
}

/// Deepest hierarchy [`apply_subtree_ranges`] walks up looking for a root.
const MAX_SUBTREE_DEPTH: usize = 48;

/// Gives new meshes under a [`SubtreeRange`] their visibility range.
pub fn apply_subtree_ranges(
    mut commands: Commands,
    added: Query<Entity, (Added<Mesh3d>, Without<VisibilityRange>)>,
    nodes: Query<(&Transform, Option<&ChildOf>, Option<&SubtreeRange>)>,
) {
    for entity in &added {
        // The mesh origin in the current ancestor's space, walking up.
        let mut offset = Vec3::ZERO;
        let mut current = entity;
        let mut found = None;
        for _ in 0..MAX_SUBTREE_DEPTH {
            let Ok((transform, parent, range)) = nodes.get(current) else { break };
            if let Some(range) = range {
                found = Some(range.range_at((offset * transform.scale).length()));
                break;
            }
            offset = transform.transform_point(offset);
            let Some(parent) = parent else { break };
            current = parent.parent();
        }
        if let Some(Some(range)) = found {
            commands.entity(entity).try_insert(range);
        }
    }
}

/// Puts a worldspace's render layers on a subtree whose meshes appear later
/// (a runtime-imported model spawned as a scene): every new `Mesh3d` below
/// without its own `RenderLayers` gets these, so the model draws only in its
/// worldspace's view (streaming.md, "Worldspaces at runtime"). Spawners
/// insert it on the root with [`SubtreeLayers::of`].
#[derive(Component, Clone, Debug, PartialEq)]
pub struct SubtreeLayers(pub RenderLayers);

impl SubtreeLayers {
    pub fn of(cx: &SpawnCx<'_>) -> Self {
        Self(cx.layers.clone())
    }
}

/// Gives new meshes under a [`SubtreeLayers`] root its render layers.
pub fn apply_subtree_layers(
    mut commands: Commands,
    added: Query<Entity, (Added<Mesh3d>, Without<RenderLayers>)>,
    nodes: Query<(Option<&ChildOf>, Option<&SubtreeLayers>)>,
) {
    for entity in &added {
        let mut current = entity;
        for _ in 0..MAX_SUBTREE_DEPTH {
            let Ok((parent, layers)) = nodes.get(current) else { break };
            if let Some(layers) = layers {
                commands.entity(entity).try_insert(layers.0.clone());
                break;
            }
            let Some(parent) = parent else { break };
            current = parent.parent();
        }
    }
}

/// The framework's spawner for static visuals: the cooked LOD chain.
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticSpawner;

impl PlacementSpawner for StaticSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, _placement: &IndexedPlacement) -> u32 {
        let Some(LoadedModel::Cooked(template)) = cx.model else { return 0 };
        spawn_template(world, cx, template)
    }
}

/// Spawns every visible LOD part of a template under `cx.parent`.
pub fn spawn_template(world: &mut World, cx: &SpawnCx<'_>, template: &ModelTemplate) -> u32 {
    let scale = cx.local.scale.abs().max_element();
    let errors: Vec<f32> = template.lods.iter().map(|l| l.error * scale).collect();
    let ranges = chain_ranges(&errors, cx.lod, cx.max_distance, cx.end_band);
    spawn_template_ranges(world, cx, template, &ranges)
}

/// [`spawn_template`] with the levels' ranges given (one per level).
pub fn spawn_template_ranges(world: &mut World, cx: &SpawnCx<'_>, template: &ModelTemplate, ranges: &[LodRange]) -> u32 {
    let mut batch = Vec::new();
    for (lod, range) in template.lods.iter().zip(ranges) {
        let Some(visibility) = visibility_range(range) else { continue };
        for (mesh, material) in &lod.parts {
            batch.push((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                cx.local,
                visibility.clone(),
                cx.layers.clone(),
                template.aabb,
                // The model's bounds are exact already; skip Bevy's AABB scan.
                NoAutoAabb,
                ChildOf(cx.parent),
            ));
        }
    }
    let n = u32::try_from(batch.len()).unwrap_or(u32::MAX);
    world.spawn_batch(batch);
    n
}

/// Where a placement stops being drawn, with the size-based draw distance
/// (industry-audit.md J1) applied: `(end, end_band)` as streaming would use
/// them without it (the ring's unload radius, or the far-field handoff and
/// its lead band), shortened to [`LodScreen::draw_distance`] of the
/// placement's radius with a dithered fade `[d · (1 − c), d]` (`c` the
/// crossfade fraction) when that comes first.
///
/// Never shortened: actors and doors, placements flagged
/// `NO_SIZE_CULL`, placements without known bounds (radius 0), and those
/// a far-field proxy represents (`kept_radius`: `Some` for a placement that
/// hands over to a proxy, with the smallest radius its cell's proxy keeps),
/// which must reach the handoff or a hole opens.
pub fn size_culled_end(lod: &LodScreen, placement: &IndexedPlacement, end: f32, end_band: f32, kept_radius: Option<f32>) -> (f32, f32) {
    let p = &placement.placement;
    let exempt = matches!(p.class, PlacementClass::Actor | PlacementClass::Door)
        || p.flags.contains(PlacementFlags::NO_SIZE_CULL)
        || placement.radius <= 0.0
        || kept_radius.is_some_and(|k| placement.radius >= k);
    let d = lod.draw_distance(placement.radius);
    if exempt || d >= end {
        return (end, end_band);
    }
    let band = d * lod.crossfade;
    (d - band, band)
}

/// LOD ranges of a chain ending at `max_distance`, with a fade-out band of
/// `end_band` meters starting there (`[max, max + band]`) when nonzero.
pub fn chain_ranges(errors: &[f32], lod: &LodScreen, max_distance: f32, end_band: f32) -> Vec<LodRange> {
    if end_band <= 0.0 {
        return lod_ranges(errors, lod, max_distance);
    }
    // `visibility_range` centers bands on boundaries: end at the band's
    // middle with the band as its margin.
    let end = max_distance + end_band * 0.5;
    let mut ranges = lod_ranges(errors, lod, end);
    if let Some(last) = ranges.iter_mut().rev().find(|r| r.end > r.start) {
        last.end = end;
        last.end_margin = end_band;
    }
    ranges
}

/// Bevy's `VisibilityRange` for one level: fade bands centered on the
/// switch distances, shrunk if a level is narrower than its two bands.
/// `None` for a level with an empty range (its error is too close to the
/// next one's to ever be shown).
pub fn visibility_range(r: &LodRange) -> Option<VisibilityRange> {
    if r.end <= r.start {
        return None;
    }
    let width = r.end - r.start;
    let (mut sm, mut em) = (r.start_margin * 0.5, r.end_margin * 0.5);
    if sm + em > width {
        let k = width / (sm + em);
        sm *= k;
        em *= k;
    }
    let band = |center: f32, half: f32| -> Range<f32> { (center - half).max(0.0)..center + half };
    Some(shared_range(VisibilityRange { start_margin: band(r.start, sm), end_margin: band(r.end, em), use_aabb: false }))
}

/// Grid steps per doubling of distance for [`shared_range`]: 2^(1/32), about
/// 2.2 %, so 1 m to 16 km is ~450 values per bound.
pub const RANGE_STEPS_PER_OCTAVE: f32 = 32.0;

/// A visibility range with every bound rounded **up** onto a shared
/// logarithmic grid ([`RANGE_STEPS_PER_OCTAVE`]), so that placements share
/// a small set of distinct ranges (streaming.md, "Visibility range slots").
///
/// Why: Bevy 0.20's render world gives every distinct `VisibilityRange` a
/// GPU slot for the app's lifetime (`RenderVisibilityRanges`, never reused)
/// and has only 65535; past that, new ranges share slot 0 and the shader's
/// dither reads slot 0's bounds, so a mesh the CPU keeps visible can be
/// dithered out (near foliage vanishing after a flight). Per-instance
/// ranges (scale × LOD error, size culling, subtree offsets) made nearly
/// every placement's range unique. Engines share LOD distances per level or
/// per class, never per instance; this keeps our per-instance math but
/// snaps its output.
///
/// The rounding is monotone (`a ≤ b` stays `a ≤ b`) and equal inputs give
/// bit-equal outputs, so matched crossfades (a level's end band equal to
/// the next one's start band) and lead overlaps with far-field proxies
/// survive; bounds only move outward by under 2.2 %.
pub fn shared_range(r: VisibilityRange) -> VisibilityRange {
    let q = |d: f32| -> f32 {
        if d <= 0.0 || !d.is_finite() {
            return d;
        }
        let k = (d.log2() * RANGE_STEPS_PER_OCTAVE).ceil();
        let v = (k / RANGE_STEPS_PER_OCTAVE).exp2();
        // log2/exp2 rounding can land a hair below `d`: take the next step.
        if v < d { ((k + 1.0) / RANGE_STEPS_PER_OCTAVE).exp2() } else { v }
    };
    VisibilityRange {
        start_margin: q(r.start_margin.start)..q(r.start_margin.end),
        end_margin: q(r.end_margin.start)..q(r.end_margin.end),
        use_aabb: r.use_aabb,
    }
}

/// Maps a model's engine-neutral material to Bevy's.
pub type MaterialMapper = Arc<dyn Fn(&sf_model::Material, &AssetServer) -> StandardMaterial + Send + Sync>;

/// How cooked materials become `StandardMaterial`s; content packs may
/// replace it (texture paths, special flags in `extras`).
#[derive(Resource, Clone)]
pub struct ModelMaterials(pub MaterialMapper);

impl Default for ModelMaterials {
    fn default() -> Self {
        Self(Arc::new(standard_material))
    }
}

/// The default mapping. Texture paths load from the default asset source.
#[allow(clippy::many_single_char_names)] // color channels
pub fn standard_material(m: &sf_model::Material, assets: &AssetServer) -> StandardMaterial {
    let [r, g, b, a] = m.base_color;
    let [er, eg, eb] = m.emissive;
    StandardMaterial {
        base_color: Color::linear_rgba(r, g, b, a),
        base_color_texture: m.base_color_texture.as_ref().map(|p| assets.load(p.clone())),
        emissive: LinearRgba::rgb(er, eg, eb),
        alpha_mode: match m.alpha {
            sf_model::AlphaMode::Opaque => AlphaMode::Opaque,
            sf_model::AlphaMode::Mask(cutoff) => AlphaMode::Mask(cutoff),
            sf_model::AlphaMode::Blend => AlphaMode::Blend,
        },
        double_sided: m.double_sided,
        cull_mode: if m.double_sided { None } else { Some(Face::Back) },
        unlit: m.unlit,
        perceptual_roughness: 0.8,
        ..StandardMaterial::default()
    }
}

/// `Aabb` of model-space bounds.
pub fn model_aabb(bounds: &sf_model::Aabb) -> Aabb {
    if bounds.is_empty() {
        return Aabb::from_min_max(Vec3::ZERO, Vec3::ZERO);
    }
    Aabb::from_min_max(Vec3::from_array(bounds.min), Vec3::from_array(bounds.max))
}
