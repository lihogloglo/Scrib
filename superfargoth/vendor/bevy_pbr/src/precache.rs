//! superfargoth patch (own, W3): pipeline precache "PW" (`superfargoth/docs/specs/pipeline-warmup.md` §5.2).
//!
//! Bevy specialises a mesh's pipelines only when an entity is first visible from a view, so a new
//! material compiles on the frame it is needed (bevy#10871). The precache takes **specimens** (a mesh
//! asset, a material asset and the entity-level key bits an entity would have) and specialises them
//! ahead of time for every view kind it knows: the camera views' main-pass and prepass keys (live,
//! or the last ones while no camera is live) and the shadow keys (directional cascade; point face and spot
//! share one perspective key), always including both shadow templates. UE5 PSO precaching does the
//! same: it predicts PSOs from mesh + material at load, per pass and view kind.
//!
//! The draw path (`specialize_material_meshes`, `specialize_prepass_material_meshes`,
//! `specialize_shadows`) builds its keys with the same functions ([`main_pass_mesh_key`],
//! [`prepass_mesh_key`], [`shadow_mesh_key`], [`shadow_view_key`], [`material_pipeline_key`]) and
//! calls the same `base_specialize` / `prepass_specialize`, which key
//! `SpecializedMeshPipelines<…Specializer>` by `(layout, key)`: an entity drawn after its specimen was
//! precached is a hash hit there and gets the precached pipeline id (no second compile). The
//! per-entity caches (`SpecializedMaterialPipelineCache` …) map entities to those ids and cannot be
//! filled before the entity exists.
//!
//! Draws the precache did not predict are counted once per pipeline ([`PrecacheStats::missed`],
//! UE's "missed" PSOs) once anything was requested; a draw specialised in the frame its specimen is
//! resolved counts as a hit. Each miss is logged once (`info!`, "precache missed: ...", at most
//! [`MISS_LOG_CAP`] lines) with its pass, mesh key, material key hash and the bits it differs by from
//! the nearest predicted key of the same pass and material type. The main-world surface (tags,
//! status, counters) lives in the framework (`sf_world_bevy::Precache`).
//!
//! Low priority (W4b1-throughput, UE5's PSO precache priority): with a [`PrecacheLimit`]
//! ([`PipelinePrecache::set_limits`]) at most `draw_waiting` precache pipelines are queued or
//! compiling while a draw-path pipeline is queued or compiling, and at most `idle` otherwise (the
//! compile threads, never unlimited: a draw that arrives later waits for one task to finish, not for
//! the whole catalog). Precache work is admitted one pipeline at a time, so slots refill exactly to
//! the limit (a specimen may be submitted over several frames and stays waiting, counted as queued,
//! until all its pipelines are submitted), and a hash hit or an already-submitted pipeline costs no
//! slot. A pipeline whose compile task has finished frees its slot at once (not at the next
//! `process_queue`). Draw-path specialisations are never held; the precache runs after the main and
//! shadow specialisers (the prepass's run earlier, in `PrepareMeshes`), so it sees the same frame's
//! draws before it submits. The pipeline cache starts every queued pipeline's task in one
//! `process_queue`, so holding precache pipelines back at submission is what keeps them from
//! taking every compile thread ahead of the draws.
//!
//! Known gaps: the draw path also requires the material's draw function for the pass (always there
//! for materials of `MaterialPlugin`), which the precache does not check; a view key first seen after
//! a specimen was resolved (a settings change, W6) does not re-run it.

use core::any::TypeId;

use alloc::sync::Arc;
use bevy_asset::{AssetId, UntypedAssetId};
use bevy_core_pipeline::{
    core_3d::{AlphaMask3d, Opaque3d, Transparent3d},
    deferred::{AlphaMask3dDeferred, Opaque3dDeferred},
    prepass::{AlphaMask3dPrepass, DeferredPrepass, MotionVectorPrepass, Opaque3dPrepass},
};
use bevy_ecs::{
    prelude::*,
    schedule::ScheduleConfigs,
    system::{ScheduleSystem, SystemParam, SystemState},
};
use bevy_material::{
    key::{ErasedMaterialKey, ErasedMaterialPipelineKey, ErasedMeshPipelineKey},
    AlphaMode, MaterialProperties, OpaqueRendererMethod, RenderPhaseType,
};
use bevy_mesh::{BaseMeshPipelineKey, Mesh, MeshVertexBufferLayoutRef};
use bevy_platform::{
    collections::{HashMap, HashSet},
    hash::FixedHasher,
};
use bevy_render::{
    erased_render_asset::ErasedRenderAssets,
    mesh::RenderMesh,
    render_asset::{prepare_assets, RenderAssets},
    render_phase::{ViewBinnedRenderPhases, ViewSortedRenderPhases},
    render_resource::{CachedPipelineState, CachedRenderPipelineId, PipelineCache},
    sync_world::MainEntity,
    view::{ExtractedView, Msaa, RenderVisibilityRanges},
    RenderSystems,
};
use bevy_shader::ShaderCacheError;
use core::hash::BuildHasher;
use tracing::{error, info};

use crate::{
    alpha_mode_pipeline_key, material::MaterialPropertiesExt, LightEntity, LightKeyCache,
    MeshPipelineKey, PreparedMaterial, RenderLightmaps, RenderMeshDraw, RenderMeshInstanceFlags, Shadow,
    Transmissive3d,
    ViewKeyCache, ViewKeyPrepassCache,
};

bitflags::bitflags! {
    /// The entity-level bits of a mesh pipeline key: what an entity adds to its mesh and material.
    /// A specimen carries them in place of an entity (`NotShadowReceiver` and transmitted shadows
    /// are mesh-uniform flags, not key bits).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EntityKeyFlags: u8 {
        /// Casts shadows (no `NotShadowCaster`): shadow views specialise it.
        const SHADOW_CASTER = 1;
        /// Has a `Lightmap`.
        const LIGHTMAPPED = 1 << 1;
        /// Its lightmap samples bicubic (`Lightmap::bicubic_sampling`); only with `LIGHTMAPPED`.
        const LIGHTMAP_BICUBIC_SAMPLING = 1 << 2;
        /// Has a crossfading (non-abrupt) `VisibilityRange`.
        const VISIBILITY_RANGE_DITHER = 1 << 3;
        /// Skinned and has last frame's joints (from its second frame on).
        const HAS_PREVIOUS_SKIN = 1 << 4;
        /// Morphed and has last frame's weights (from its second frame on).
        const HAS_PREVIOUS_MORPH = 1 << 5;
    }
}

/// The entity bits of a drawn mesh, as the draw path reads them. Batches (`RenderMeshDraw::Gpu`)
/// carry only the shadow-caster flag. `visibility_ranges: None` (shadows: no dither bit) skips
/// that lookup.
pub(crate) fn entity_key_flags(
    draw: &RenderMeshDraw,
    entity: MainEntity,
    lightmaps: &RenderLightmaps,
    visibility_ranges: Option<&RenderVisibilityRanges>,
) -> EntityKeyFlags {
    let mut flags = EntityKeyFlags::empty();
    let draw_flags = draw.flags();
    flags.set(
        EntityKeyFlags::SHADOW_CASTER,
        draw_flags.contains(RenderMeshInstanceFlags::SHADOW_CASTER),
    );
    if let RenderMeshDraw::Extracted(_) = draw {
        if let Some(lightmap) = lightmaps.render_lightmaps.get(&entity) {
            flags |= EntityKeyFlags::LIGHTMAPPED;
            flags.set(EntityKeyFlags::LIGHTMAP_BICUBIC_SAMPLING, lightmap.bicubic_sampling);
        }
        flags.set(
            EntityKeyFlags::VISIBILITY_RANGE_DITHER,
            visibility_ranges.is_some_and(|r| r.entity_has_crossfading_visibility_ranges(entity)),
        );
        flags.set(
            EntityKeyFlags::HAS_PREVIOUS_SKIN,
            draw_flags.contains(RenderMeshInstanceFlags::HAS_PREVIOUS_SKIN),
        );
        flags.set(
            EntityKeyFlags::HAS_PREVIOUS_MORPH,
            draw_flags.contains(RenderMeshInstanceFlags::HAS_PREVIOUS_MORPH),
        );
    }
    flags
}

/// The main-pass (forward: opaque, alpha-mask, transmissive, transparent) mesh key of a mesh with
/// `material` and `entity` bits in a view with `view_key` (`ViewKeyCache`).
#[must_use]
pub fn main_pass_mesh_key(
    view_key: MeshPipelineKey,
    mesh_key_bits: &BaseMeshPipelineKey,
    material: &MaterialProperties,
    entity: EntityKeyFlags,
) -> MeshPipelineKey {
    let mut material_bits: MeshPipelineKey = material.mesh_pipeline_key_bits.downcast();
    material_bits.insert(alpha_mode_pipeline_key(
        material.alpha_mode,
        &Msaa::from_samples(view_key.msaa_samples()),
    ));
    let mut key = view_key | MeshPipelineKey::from_bits_retain(mesh_key_bits.bits()) | material_bits;
    if entity.contains(EntityKeyFlags::LIGHTMAPPED) {
        key |= MeshPipelineKey::LIGHTMAPPED;
        if entity.contains(EntityKeyFlags::LIGHTMAP_BICUBIC_SAMPLING) {
            key |= MeshPipelineKey::LIGHTMAP_BICUBIC_SAMPLING;
        }
    }
    if entity.contains(EntityKeyFlags::VISIBILITY_RANGE_DITHER) {
        key |= MeshPipelineKey::VISIBILITY_RANGE_DITHER;
    }
    if view_key.contains(MeshPipelineKey::MOTION_VECTOR_PREPASS) {
        key |= previous_frame_bits(entity);
    }
    key
}

/// The prepass or deferred-prepass mesh key, or `None` when the material has no prepass in this
/// view (prepass off, blended, reads the view transmission texture). `view_key` is the
/// `ViewKeyPrepassCache` key, `msaa` the view's, `deferred_prepass` / `motion_vector_prepass` the
/// view's `DeferredPrepass` / `MotionVectorPrepass`.
#[must_use]
pub fn prepass_mesh_key(
    view_key: MeshPipelineKey,
    mesh_key_bits: &BaseMeshPipelineKey,
    material: &MaterialProperties,
    msaa: &Msaa,
    deferred_prepass: bool,
    motion_vector_prepass: bool,
    entity: EntityKeyFlags,
) -> Option<MeshPipelineKey> {
    if !material.prepass_enabled {
        return None;
    }
    let mut key = view_key | MeshPipelineKey::from_bits_retain(mesh_key_bits.bits());
    match material.alpha_mode {
        AlphaMode::Opaque | AlphaMode::AlphaToCoverage | AlphaMode::Mask(_) => {
            key |= alpha_mode_pipeline_key(material.alpha_mode, msaa);
        }
        AlphaMode::Blend | AlphaMode::Premultiplied | AlphaMode::Add | AlphaMode::Multiply => {
            return None;
        }
    }
    // Materials reading `ViewTransmissionTexture` are not drawn in `Opaque3d`, so not in the prepass.
    if material.reads_view_transmission_texture {
        return None;
    }
    let forward = match material.render_method {
        OpaqueRendererMethod::Forward => true,
        OpaqueRendererMethod::Deferred => false,
        OpaqueRendererMethod::Auto => unreachable!(),
    };
    let deferred = deferred_prepass && !forward;
    if deferred {
        key |= MeshPipelineKey::DEFERRED_PREPASS;
    }
    if entity.contains(EntityKeyFlags::LIGHTMAPPED) {
        // Unused by the forward prepass, but `SetMeshBindGroup` binds the lightmap data, so the
        // layout entries must be there.
        key |= MeshPipelineKey::LIGHTMAPPED;
        if entity.contains(EntityKeyFlags::LIGHTMAP_BICUBIC_SAMPLING) && deferred {
            key |= MeshPipelineKey::LIGHTMAP_BICUBIC_SAMPLING;
        }
    }
    if entity.contains(EntityKeyFlags::VISIBILITY_RANGE_DITHER) {
        key |= MeshPipelineKey::VISIBILITY_RANGE_DITHER;
    }
    if motion_vector_prepass {
        key |= previous_frame_bits(entity);
    }
    if material.prepass_reads_material() {
        key |= MeshPipelineKey::PREPASS_READS_MATERIAL;
    }
    Some(key)
}

/// The shadow-view key of a light view (`LightKeyCache`): a directional cascade is orthographic
/// with unclipped depth; point faces and spots are perspective (one key for both).
#[must_use]
pub fn shadow_view_key(directional: bool) -> MeshPipelineKey {
    let mut light_key = MeshPipelineKey::DEPTH_PREPASS;
    light_key.set(
        MeshPipelineKey::VIEW_PROJECTION_ORTHOGRAPHIC | MeshPipelineKey::UNCLIPPED_DEPTH_ORTHO,
        directional,
    );
    light_key.set(MeshPipelineKey::VIEW_PROJECTION_PERSPECTIVE, !directional);
    light_key
}

/// The shadow mesh key in a light view with `light_key`, or `None` when the mesh casts no shadow
/// (material shadows off, or no `SHADOW_CASTER`).
#[must_use]
pub fn shadow_mesh_key(
    light_key: MeshPipelineKey,
    mesh_key_bits: &BaseMeshPipelineKey,
    material: &MaterialProperties,
    entity: EntityKeyFlags,
) -> Option<MeshPipelineKey> {
    if !material.shadows_enabled || !entity.contains(EntityKeyFlags::SHADOW_CASTER) {
        return None;
    }
    let mut key = light_key | MeshPipelineKey::from_bits_retain(mesh_key_bits.bits());
    // Unused by the shadow map, but `SetMeshBindGroup` binds the lightmap data.
    if entity.contains(EntityKeyFlags::LIGHTMAPPED) {
        key |= MeshPipelineKey::LIGHTMAPPED;
    }
    key |= match material.alpha_mode {
        AlphaMode::Mask(_)
        | AlphaMode::Blend
        | AlphaMode::Premultiplied
        | AlphaMode::Add
        | AlphaMode::AlphaToCoverage => MeshPipelineKey::MAY_DISCARD,
        _ => MeshPipelineKey::NONE,
    };
    if material.prepass_reads_material() {
        key |= MeshPipelineKey::PREPASS_READS_MATERIAL;
    }
    Some(key)
}

/// The specialiser key (`SpecializedMeshPipelines` keys by it and the vertex layout).
#[must_use]
pub fn material_pipeline_key(
    material_type_id: TypeId,
    mesh_key: MeshPipelineKey,
    material: &MaterialProperties,
) -> ErasedMaterialPipelineKey {
    ErasedMaterialPipelineKey {
        type_id: material_type_id,
        mesh_key: ErasedMeshPipelineKey::new(mesh_key),
        material_key: material.material_key.clone(),
    }
}

fn previous_frame_bits(entity: EntityKeyFlags) -> MeshPipelineKey {
    let mut key = MeshPipelineKey::NONE;
    if entity.contains(EntityKeyFlags::HAS_PREVIOUS_SKIN) {
        key |= MeshPipelineKey::HAS_PREVIOUS_SKIN;
    }
    if entity.contains(EntityKeyFlags::HAS_PREVIOUS_MORPH) {
        key |= MeshPipelineKey::HAS_PREVIOUS_MORPH;
    }
    key
}

/// A mesh + material + entity bits to specialise before any entity draws it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Specimen {
    pub mesh: AssetId<Mesh>,
    /// A material asset of a `MaterialPlugin` material (its `PreparedMaterial`).
    pub material: UntypedAssetId,
    pub flags: EntityKeyFlags,
}

/// A camera view's prepass inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrepassViewKey {
    /// `ViewKeyPrepassCache` key.
    pub key: MeshPipelineKey,
    pub msaa: Msaa,
    pub deferred_prepass: bool,
    pub motion_vector_prepass: bool,
}

/// The view keys a specimen is specialised for: the live camera views' keys (the last ones while none
/// is live) plus every shadow key seen and the two shadow templates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrecacheViewKeys {
    pub main: Vec<MeshPipelineKey>,
    pub prepass: Vec<PrepassViewKey>,
    pub shadow: Vec<MeshPipelineKey>,
}

impl Default for PrecacheViewKeys {
    fn default() -> Self {
        Self {
            main: Vec::new(),
            prepass: Vec::new(),
            shadow: vec![shadow_view_key(true), shadow_view_key(false)],
        }
    }
}

impl PrecacheViewKeys {
    pub fn insert_main(&mut self, key: MeshPipelineKey) {
        if !self.main.contains(&key) {
            self.main.push(key);
        }
    }
    pub fn insert_prepass(&mut self, key: PrepassViewKey) {
        if !self.prepass.contains(&key) {
            self.prepass.push(key);
        }
    }
    pub fn insert_shadow(&mut self, key: MeshPipelineKey) {
        if !self.shadow.contains(&key) {
            self.shadow.push(key);
        }
    }
}

/// Which pass a planned pipeline is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrecachePass {
    /// `base_specialize` (`SpecializedMeshPipelines<MaterialPipelineSpecializer>`).
    Main,
    /// `prepass_specialize` (`SpecializedMeshPipelines<PrepassPipelineSpecializer>`).
    Prepass,
    /// `prepass_specialize` with a shadow key.
    Shadow,
}

/// The specialiser keys of a specimen in every known view kind, without duplicates. A skinned or
/// morphed specimen (`HAS_PREVIOUS_*`) also gets every subset of those bits: the first-frame keys
/// (no previous data yet) and, for a skinned mesh with morph targets, previous skin without previous
/// morph and the reverse (the draw path sets each bit on its own: a rig whose morph weights did not
/// change has previous joints but no previous weights; census run 2026-11-15, the last 2 misses).
#[must_use]
pub fn plan_specimen(
    views: &PrecacheViewKeys,
    mesh_key_bits: &BaseMeshPipelineKey,
    material: &MaterialProperties,
    material_type_id: TypeId,
    flags: EntityKeyFlags,
) -> Vec<(PrecachePass, ErasedMaterialPipelineKey)> {
    let previous = EntityKeyFlags::HAS_PREVIOUS_SKIN | EntityKeyFlags::HAS_PREVIOUS_MORPH;
    let base = flags - previous;
    let mut variants = vec![flags];
    for part in [EntityKeyFlags::HAS_PREVIOUS_SKIN, EntityKeyFlags::HAS_PREVIOUS_MORPH, EntityKeyFlags::empty()] {
        let variant = base | (flags & part);
        if !variants.contains(&variant) {
            variants.push(variant);
        }
    }
    let mut plan: Vec<(PrecachePass, ErasedMaterialPipelineKey)> = Vec::new();
    let mut push = |pass, mesh_key| {
        let entry = (pass, material_pipeline_key(material_type_id, mesh_key, material));
        if !plan.contains(&entry) {
            plan.push(entry);
        }
    };
    for &entity in &variants {
        if material.base_specialize.is_some() {
            for &view_key in &views.main {
                push(PrecachePass::Main, main_pass_mesh_key(view_key, mesh_key_bits, material, entity));
            }
        }
        if material.prepass_specialize.is_none() {
            continue;
        }
        // The prepass draw path skips these phases before specialising.
        if !matches!(
            material.render_phase_type,
            RenderPhaseType::Transmissive | RenderPhaseType::Transparent
        ) {
            for view in &views.prepass {
                if let Some(key) = prepass_mesh_key(
                    view.key,
                    mesh_key_bits,
                    material,
                    &view.msaa,
                    view.deferred_prepass,
                    view.motion_vector_prepass,
                    entity,
                ) {
                    push(PrecachePass::Prepass, key);
                }
            }
        }
        for &light_key in &views.shadow {
            if let Some(key) = shadow_mesh_key(light_key, mesh_key_bits, material, entity) {
                push(PrecachePass::Shadow, key);
            }
        }
    }
    plan
}

/// A specimen still waiting.
#[derive(Clone, Copy, Debug)]
pub struct PrecacheWaiting {
    pub tag: u32,
    pub specimen: Specimen,
    /// Its `RenderMesh` and `PreparedMaterial` were there at the last try (then it waits for a
    /// camera view key).
    pub assets_ready: bool,
}

/// A specimen specialised for every known view kind.
#[derive(Clone, Debug)]
pub struct PrecacheResolved {
    pub tag: u32,
    pub specimen: Specimen,
    /// The pipeline ids the draw path will get for it.
    pub pipelines: Vec<CachedRenderPipelineId>,
    /// Specialisations that returned an error (logged).
    pub errors: u32,
}

/// Running totals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrecacheStats {
    /// Specimens requested.
    pub requested: u64,
    /// Distinct pipelines the draw path specialised that a specimen had predicted.
    pub hit: u64,
    /// Distinct pipelines the draw path specialised that no specimen had predicted.
    pub missed: u64,
    /// Specimens waiting for their mesh or material (last frame).
    pub pending_assets: u32,
    /// Specimens ready to specialise but held back by the limit (last frame), including ones
    /// partly submitted.
    pub throttled: u32,
    /// Precache pipelines queued or compiling (last frame).
    pub outstanding: u32,
    /// Draw-path pipelines queued or compiling that are not the precache's own (last frame).
    pub draw_waiting: u32,
    /// The limit applied (last frame; 0: no limit).
    pub limit: u32,
}

/// Most precache pipelines queued or compiling at once ([`PipelinePrecache::set_limits`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrecacheLimit {
    /// While no draw-path pipeline is queued or compiling.
    pub idle: usize,
    /// While a draw-path pipeline (not one the precache submitted) is queued or compiling.
    pub draw_waiting: usize,
}

impl PrecacheLimit {
    /// The same limit whether a draw waits or not.
    #[must_use]
    pub const fn uniform(k: usize) -> Self {
        Self { idle: k, draw_waiting: k }
    }

    /// Both at least 1.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self { idle: self.idle.max(1), draw_waiting: self.draw_waiting.max(1) }
    }

    /// The limit that applies.
    #[must_use]
    pub const fn applied(self, draw_waiting: bool) -> usize {
        if draw_waiting { self.draw_waiting } else { self.idle }
    }
}

/// Most "precache missed" lines logged (then one "not logged" line).
pub const MISS_LOG_CAP: u32 = 256;

/// A draw-path pipeline the precache had not predicted, noted this frame.
#[derive(Clone, Copy, Debug)]
struct MissNote {
    id: CachedRenderPipelineId,
    pass: PrecachePass,
    /// (mesh key bits, material type, material key hash); `None` from the shadow path.
    key: Option<(u64, TypeId, u64)>,
}

/// The render-world precache queue. [`precache_specimens`] resolves it each frame.
#[derive(Resource, Default)]
pub struct PipelinePrecache {
    waiting: Vec<PrecacheWaiting>,
    resolved: Vec<PrecacheResolved>,
    views: PrecacheViewKeys,
    precached: HashSet<CachedRenderPipelineId>,
    draw_seen: HashSet<CachedRenderPipelineId>,
    stats: PrecacheStats,
    /// Most precache pipelines queued or compiling at once (`None`: no limit).
    limit: Option<PrecacheLimit>,
    /// Precache pipelines queued or compiling at the last resolve.
    outstanding: HashSet<CachedRenderPipelineId>,
    /// Draw-path pipelines that were queued or compiling when first drawn (pruned each resolve).
    draw_pending: HashSet<CachedRenderPipelineId>,
    /// This frame's misses, settled at the next resolve.
    recent_misses: Vec<MissNote>,
    /// Predicted keys by (pass, material type): (mesh key bits, material key hash).
    predicted: HashMap<(PrecachePass, TypeId), HashSet<(u64, u64)>>,
    misses_logged: u32,
}

fn material_key_hash(key: &ErasedMaterialKey) -> u64 {
    FixedHasher.hash_one(key)
}

impl PipelinePrecache {
    /// Queues a specimen under `tag`.
    pub fn request(&mut self, tag: u32, specimen: Specimen) {
        self.stats.requested += 1;
        self.waiting.push(PrecacheWaiting { tag, specimen, assets_ready: false });
    }

    /// Forgets every specimen of `tag` (its pipelines stay in the caches).
    pub fn release(&mut self, tag: u32) {
        self.waiting.retain(|w| w.tag != tag);
        self.resolved.retain(|r| r.tag != tag);
    }

    #[must_use]
    pub fn waiting(&self) -> &[PrecacheWaiting] {
        &self.waiting
    }

    #[must_use]
    pub fn resolved(&self) -> &[PrecacheResolved] {
        &self.resolved
    }

    #[must_use]
    pub fn views(&self) -> &PrecacheViewKeys {
        &self.views
    }

    /// The known view keys: seed camera keys while no camera view is live (W6: a preset's keys);
    /// the live views' keys replace them.
    pub fn views_mut(&mut self) -> &mut PrecacheViewKeys {
        &mut self.views
    }

    #[must_use]
    pub fn stats(&self) -> PrecacheStats {
        self.stats
    }

    /// At most `limit` precache pipelines queued or compiling at once, draw waiting or not
    /// ([`Self::set_limits`] with [`PrecacheLimit::uniform`]).
    pub fn set_limit(&mut self, limit: Option<usize>) {
        self.set_limits(limit.map(PrecacheLimit::uniform));
    }

    /// At most `limit.draw_waiting` precache pipelines queued or compiling at once while a
    /// draw-path pipeline is, `limit.idle` otherwise (`None`: no limit; 0 counts as 1). Work is
    /// admitted one pipeline at a time, so the limit is never passed; a specimen not fully
    /// submitted stays waiting and goes on next frame.
    pub fn set_limits(&mut self, limit: Option<PrecacheLimit>) {
        self.limit = limit.map(PrecacheLimit::normalized);
    }

    #[must_use]
    pub fn limit(&self) -> Option<PrecacheLimit> {
        self.limit
    }

    /// Precache pipelines queued or compiling at the last resolve.
    #[must_use]
    pub fn outstanding(&self) -> usize {
        self.outstanding.len()
    }

    /// The draw path got `id` for an entity: counts it once as hit or missed ([`Self::note_draw_with`]
    /// with a compiled pipeline and no key).
    pub fn note_draw(&mut self, id: CachedRenderPipelineId) {
        self.note_draw_with(id, PrecachePass::Main, None, false);
    }

    /// The draw path got `id` (its `pass`, mesh key and specialiser key when known) for an entity:
    /// counts it once as hit or missed, and remembers it as a waiting draw while `in_progress`
    /// (queued or compiling). A miss is settled (and logged) at the next resolve: a draw
    /// specialised in the frame its specimen resolves counts as a hit.
    pub fn note_draw_with(
        &mut self,
        id: CachedRenderPipelineId,
        pass: PrecachePass,
        key: Option<(MeshPipelineKey, TypeId, &ErasedMaterialKey)>,
        in_progress: bool,
    ) {
        if self.stats.requested == 0 || !self.draw_seen.insert(id) {
            return;
        }
        if in_progress {
            self.draw_pending.insert(id);
        }
        if self.precached.contains(&id) {
            self.stats.hit += 1;
        } else {
            self.stats.missed += 1;
            self.recent_misses.push(MissNote {
                id,
                pass,
                key: key.map(|(mesh, material_type, k)| (mesh.bits(), material_type, material_key_hash(k))),
            });
        }
    }

    /// Settles this frame's misses: a pipeline predicted since counts as a hit, the rest are
    /// logged once.
    fn settle_misses(&mut self) {
        for miss in core::mem::take(&mut self.recent_misses) {
            if self.precached.contains(&miss.id) {
                self.stats.missed -= 1;
                self.stats.hit += 1;
                continue;
            }
            if self.misses_logged > MISS_LOG_CAP {
                continue;
            }
            self.misses_logged += 1;
            if self.misses_logged > MISS_LOG_CAP {
                info!("precache missed: further misses not logged ({MISS_LOG_CAP} logged)");
                continue;
            }
            info!("{}", self.miss_line(&miss));
        }
    }

    /// The log line of a miss.
    fn miss_line(&self, miss: &MissNote) -> String {
        let waiting = self.waiting.len();
        let Some((bits, type_id, material)) = miss.key else {
            return alloc::format!(
                "precache missed: {:?} pipeline {} key unknown (waiting {waiting})",
                miss.pass,
                miss.id.id()
            );
        };
        let nearest = self.predicted.get(&(miss.pass, type_id)).and_then(|keys| {
            keys.iter()
                .map(|&(b, m)| (m != material, (b ^ bits).count_ones(), b ^ bits))
                .min()
        });
        let near = match nearest {
            Some((material_differs, _, xor)) => alloc::format!(
                "nearest predicted differs by {xor:#018x} {:?}{}",
                MeshPipelineKey::from_bits_retain(xor),
                if material_differs { " and material key" } else { "" }
            ),
            None => "no predicted key of this pass and material type".into(),
        };
        alloc::format!(
            "precache missed: {:?} pipeline {} mesh key {bits:#018x} {:?} material key {material:#018x} (waiting {waiting}); {near}",
            miss.pass,
            miss.id.id(),
            MeshPipelineKey::from_bits_retain(bits),
        )
    }
}

/// Called by the shadow draw-path specialiser for each pipeline id it gets.
pub(crate) fn note_draw_pipeline(world: &mut World, id: CachedRenderPipelineId) {
    note_draw(world, id, PrecachePass::Shadow, None);
}

/// Called by the main and prepass draw-path specialisers for each pipeline id they get.
pub(crate) fn note_draw_pipeline_key(
    world: &mut World,
    id: CachedRenderPipelineId,
    pass: PrecachePass,
    mesh_key: MeshPipelineKey,
    material_type_id: TypeId,
    material_key: &ErasedMaterialKey,
) {
    note_draw(world, id, pass, Some((mesh_key, material_type_id, material_key)));
}

fn note_draw(
    world: &mut World,
    id: CachedRenderPipelineId,
    pass: PrecachePass,
    key: Option<(MeshPipelineKey, TypeId, &ErasedMaterialKey)>,
) {
    // Hot (every specialised entity): only a pipeline's first draw reads its state.
    let first = world
        .get_resource::<PipelinePrecache>()
        .is_some_and(|p| p.stats.requested > 0 && !p.draw_seen.contains(&id));
    if !first {
        return;
    }
    let in_progress = world
        .get_resource::<PipelineCache>()
        .is_some_and(|cache| pipeline_in_progress(cache.get_render_pipeline_state(id)));
    if let Some(mut precache) = world.get_resource_mut::<PipelinePrecache>() {
        precache.bypass_change_detection().note_draw_with(id, pass, key, in_progress);
    }
}

#[derive(SystemParam)]
pub struct PrecacheParam<'w, 's> {
    cameras: Query<'w, 's, &'static ExtractedView>,
    prepass_views: Query<
        'w,
        's,
        (
            &'static ExtractedView,
            &'static Msaa,
            Has<MotionVectorPrepass>,
            Has<DeferredPrepass>,
        ),
    >,
    light_views: Query<'w, 's, &'static ExtractedView, With<LightEntity>>,
    view_key_cache: Option<Res<'w, ViewKeyCache>>,
    prepass_key_cache: Option<Res<'w, ViewKeyPrepassCache>>,
    light_key_cache: Option<Res<'w, LightKeyCache>>,
    opaque: Option<Res<'w, ViewBinnedRenderPhases<Opaque3d>>>,
    alpha_mask: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3d>>>,
    transmissive: Option<Res<'w, ViewSortedRenderPhases<Transmissive3d>>>,
    transparent: Option<Res<'w, ViewSortedRenderPhases<Transparent3d>>>,
    opaque_prepass: Option<Res<'w, ViewBinnedRenderPhases<Opaque3dPrepass>>>,
    alpha_mask_prepass: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3dPrepass>>>,
    opaque_deferred: Option<Res<'w, ViewBinnedRenderPhases<Opaque3dDeferred>>>,
    alpha_mask_deferred: Option<Res<'w, ViewBinnedRenderPhases<AlphaMask3dDeferred>>>,
    shadow: Option<Res<'w, ViewBinnedRenderPhases<Shadow>>>,
    render_meshes: Option<Res<'w, RenderAssets<RenderMesh>>>,
    render_materials: Option<Res<'w, ErasedRenderAssets<PreparedMaterial>>>,
}

macro_rules! has_phase {
    ($phases:expr, $view:expr) => {
        $phases.as_ref().is_some_and(|p| p.contains_key($view))
    };
}

struct PrecacheWork {
    pass: PrecachePass,
    key: ErasedMaterialPipelineKey,
    layout: MeshVertexBufferLayoutRef,
    properties: Arc<MaterialProperties>,
}

/// Candidates planned per frame while a limit holds specimens back (the rest wait, unplanned).
const THROTTLED_CANDIDATES: usize = 64;

/// A pipeline in this state still needs a compile thread: queued, compiling (its task not finished:
/// a finished one waits only for the next `process_queue` to store it, on no thread), or waiting
/// for its shader to load (the cache retries it).
#[must_use]
pub fn pipeline_in_progress(state: &CachedPipelineState) -> bool {
    match state {
        CachedPipelineState::Queued
        | CachedPipelineState::Err(
            ShaderCacheError::ShaderNotLoaded(_) | ShaderCacheError::ShaderImportNotYetAvailable,
        ) => true,
        CachedPipelineState::Creating(task) => !task.is_finished(),
        _ => false,
    }
}

/// [`precache_specimens`] in `RenderSystems::Specialize`, after the view-key systems, the meshes'
/// preparation and the main and shadow draw-path specialisers (so it sees the same frame's draws
/// before it submits; the prepass specialiser runs earlier, in `PrepareMeshes`).
#[must_use]
pub fn precache_specimens_systems() -> ScheduleConfigs<ScheduleSystem> {
    precache_specimens
        .in_set(RenderSystems::Specialize)
        .after(prepare_assets::<RenderMesh>)
        .after(crate::check_views_need_specialization)
        .after(crate::check_prepass_views_need_specialization)
        .after(crate::check_views_lights_need_specialization)
        .after(crate::specialize_material_meshes)
        .after(crate::specialize_shadows)
}

/// Learns the live views' keys, then specialises the waiting specimens whose mesh and material
/// are prepared, once a camera key is known, within the limit ([`PipelinePrecache::set_limits`]).
/// Render world: [`precache_specimens_systems`].
pub fn precache_specimens(world: &mut World, state: &mut SystemState<PrecacheParam>) {
    precache_specimens_with(world, state, |world, id| {
        world
            .get_resource::<PipelineCache>()
            .is_some_and(|cache| pipeline_in_progress(cache.get_render_pipeline_state(id)))
    });
}

/// [`precache_specimens`] with `in_progress` telling whether a pipeline is still queued or
/// compiling (the system reads the [`PipelineCache`]; tests without a render device pass their own).
pub fn precache_specimens_with(
    world: &mut World,
    state: &mut SystemState<PrecacheParam>,
    mut in_progress: impl FnMut(&World, CachedRenderPipelineId) -> bool,
) {
    let Some(mut waiting) = world
        .get_resource_mut::<PipelinePrecache>()
        .map(|mut p| core::mem::take(&mut p.waiting))
    else {
        return;
    };
    let (limit, mut outstanding, mut draw_pending) = {
        let mut p = world.resource_mut::<PipelinePrecache>();
        (p.limit, core::mem::take(&mut p.outstanding), core::mem::take(&mut p.draw_pending))
    };
    outstanding.retain(|&id| in_progress(world, id));
    draw_pending.retain(|&id| in_progress(world, id));
    // A draw waiting on a precache pipeline competes for no extra thread.
    let draw_waiting = draw_pending.iter().filter(|id| !outstanding.contains(*id)).count();
    let k = limit.map(|l| l.applied(draw_waiting > 0));
    let mut views = world.resource::<PipelinePrecache>().views.clone();
    // (index in `waiting`, its planned work), in request order.
    let mut candidates: Vec<(usize, Vec<PrecacheWork>)> = Vec::new();
    {
        let p = state.get(world).unwrap();
        // Live camera keys replace the remembered ones (a settings change drops the old keys);
        // with no camera view live, the last ones stay the templates.
        let mut live = PrecacheViewKeys { main: Vec::new(), prepass: Vec::new(), shadow: Vec::new() };
        if let Some(cache) = &p.view_key_cache {
            for view in &p.cameras {
                let v = &view.retained_view_entity;
                if (has_phase!(p.opaque, v)
                    || has_phase!(p.alpha_mask, v)
                    || has_phase!(p.transmissive, v)
                    || has_phase!(p.transparent, v))
                    && let Some(&key) = cache.get(v)
                {
                    live.insert_main(key);
                }
            }
        }
        if let Some(cache) = &p.prepass_key_cache {
            for (view, msaa, motion_vectors, deferred) in &p.prepass_views {
                let v = &view.retained_view_entity;
                if (has_phase!(p.opaque_prepass, v)
                    || has_phase!(p.alpha_mask_prepass, v)
                    || has_phase!(p.opaque_deferred, v)
                    || has_phase!(p.alpha_mask_deferred, v))
                    && let Some(&key) = cache.get(v)
                {
                    live.insert_prepass(PrepassViewKey {
                        key,
                        msaa: *msaa,
                        deferred_prepass: deferred,
                        motion_vector_prepass: motion_vectors,
                    });
                }
            }
        }
        if !live.main.is_empty() {
            views.main = live.main;
            views.prepass = live.prepass;
        }
        if let Some(cache) = &p.light_key_cache {
            for view in &p.light_views {
                let v = &view.retained_view_entity;
                if has_phase!(p.shadow, v)
                    && let Some(&key) = cache.get(v)
                {
                    views.insert_shadow(key);
                }
            }
        }

        if let (Some(meshes), Some(materials)) = (&p.render_meshes, &p.render_materials) {
            let planned = if limit.is_some() { THROTTLED_CANDIDATES } else { usize::MAX };
            for (index, w) in waiting.iter_mut().enumerate() {
                let (Some(mesh), Some(material)) =
                    (meshes.get(w.specimen.mesh), materials.get(w.specimen.material))
                else {
                    w.assets_ready = false;
                    continue;
                };
                w.assets_ready = true;
                if views.main.is_empty() || candidates.len() >= planned {
                    continue;
                }
                let work = plan_specimen(
                    &views,
                    &mesh.key_bits,
                    &material.properties,
                    w.specimen.material.type_id(),
                    w.specimen.flags,
                )
                .into_iter()
                .map(|(pass, key)| PrecacheWork {
                    pass,
                    key,
                    layout: mesh.layout.clone(),
                    properties: material.properties.clone(),
                })
                .collect();
                candidates.push((index, work));
            }
        }
    }

    // One pipeline at a time, so the limit sees what each one really queued: a hash hit on a
    // compiled pipeline, or on one already outstanding (a specimen partly submitted last frame
    // goes through its submitted pipelines again), costs no slot. A specimen stopped by the limit
    // stays waiting.
    let full = |outstanding: &HashSet<CachedRenderPipelineId>| k.is_some_and(|k| outstanding.len() >= k);
    let mut resolved = Vec::new();
    let mut predicted = Vec::new();
    let mut submitted = Vec::new();
    let mut taken = vec![false; waiting.len()];
    for (index, work) in candidates {
        if full(&outstanding) {
            break;
        }
        let w = waiting[index];
        let mut entry = PrecacheResolved {
            tag: w.tag,
            specimen: w.specimen,
            pipelines: Vec::with_capacity(work.len()),
            errors: 0,
        };
        let mut complete = true;
        for item in work {
            if full(&outstanding) {
                complete = false;
                break;
            }
            let class = (
                (item.pass, item.key.type_id),
                (
                    item.key.mesh_key.downcast::<MeshPipelineKey>().bits(),
                    material_key_hash(&item.key.material_key),
                ),
            );
            let specialize = match item.pass {
                PrecachePass::Main => item
                    .properties
                    .base_specialize
                    .map(|f| f(world, item.key, &item.layout, &item.properties)),
                PrecachePass::Prepass | PrecachePass::Shadow => item
                    .properties
                    .prepass_specialize
                    .map(|f| f(world, &item.key, &item.layout, &item.properties)),
            };
            match specialize {
                Some(Ok(id)) => {
                    entry.pipelines.push(id);
                    submitted.push(id);
                    predicted.push(class);
                    // A draw's pipeline counts as the draw's (it already holds the draw limit).
                    if !draw_pending.contains(&id) && in_progress(world, id) {
                        outstanding.insert(id);
                    }
                }
                Some(Err(err)) => {
                    entry.errors += 1;
                    error!("precache {:?}: {}", item.pass, err);
                }
                None => {}
            }
        }
        if complete {
            taken[index] = true;
            resolved.push(entry);
        }
    }
    let mut index = 0;
    waiting.retain(|_| {
        let keep = !taken[index];
        index += 1;
        keep
    });

    let mut precache = world.resource_mut::<PipelinePrecache>();
    // Submitted pipelines count as predicted at once, before their specimen completes.
    precache.precached.extend(submitted);
    for (class, key) in predicted {
        precache.predicted.entry(class).or_default().insert(key);
    }
    precache.resolved.extend(resolved);
    precache.stats.pending_assets = waiting.iter().filter(|w| !w.assets_ready).count() as u32;
    precache.stats.throttled = if views.main.is_empty() {
        0
    } else {
        waiting.iter().filter(|w| w.assets_ready).count() as u32
    };
    precache.stats.outstanding = outstanding.len() as u32;
    precache.stats.draw_waiting = draw_waiting as u32;
    precache.stats.limit = k.map_or(0, |k| k as u32);
    precache.outstanding = outstanding;
    precache.draw_pending = draw_pending;
    precache.waiting = waiting;
    precache.views = views;
    precache.settle_misses();
}
