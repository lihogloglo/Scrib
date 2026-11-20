//! superfargoth patch (own, CW-13b: 2D point shadow atlas), ROADMAP 1.25 CW-13.
//! Audit & design: `docs/reviews/shadow-audit-2026-11-19.md` §7.
//!
//! Unified cached 2D Shadow Atlas for point lights (Doom 2016 / Unity HDRP / UE Cached Whole-Scene
//! Shadows pattern):
//! - Two persistent 2D `Depth32Float` textures: **Static Atlas** (`4096²` High) for cached static
//!   geometry and **Live Atlas** (`2048²` High) for active mover faces and moving lights (torches).
//! - Variable power-of-two tiles (`512² / 256² / 128²`) packed by the host's buddy allocator
//!   (`sf_sky::shadow_atlas`), with empty-face omission (`atlas_kind == 0`, zero texture fetch)
//!   and a widened guard-band projection (`guard_texels`) preventing 2×2 PCF seam bleed.
//! - Zero `RootNonCameraView` schedules and zero cube-array layer cap (42-cube limit lifted when
//!   enabled; stock cube array shrinks to 1×1×6 dummy).
//! - All active shadow draws run inside ONE [`RenderContext`] command encoder
//!   ([`record_point_shadow_atlas`]):
//!   1. Batched GPU preprocess (`allocate_uniforms`, `unpack_bins`, `early_gpu_preprocess`).
//!   2. Static captures (0–4/frame) & moving-light faces (indirect drawing): `run_build_indirect_parameters`
//!      + per-tile `RenderPass` (`LoadOp::Load`) with viewport/scissor clear quad + `shadow_phase.render`.
//!   3. All active mover-only faces (`NoIndirectDrawing`, no GPU instance batches): **ONE single
//!      `RenderPass`** over the Live Atlas (`LoadOp::Load`), emitting per-tile `set_viewport` +
//!      `set_scissor_rect` + `textureLoad` depth-blit quad (`Static -> Live`) or clear quad +
//!      `shadow_phase.render`.

use alloc::sync::Arc;
use std::sync::Mutex;

use bevy_app::App;
use bevy_asset::{embedded_asset, load_embedded_asset, AssetServer};
use bevy_camera::primitives::{face_index_to_name, Frustum};
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::{
    entity::EntityHashMap,
    prelude::*,
    system::{lifetimeless::Read, SystemParam},
};
use bevy_light::PointLight;
use bevy_math::{proj, IVec2, Mat4, UVec4, Vec2};
use bevy_platform::collections::HashSet;
use bevy_reflect::Reflect;
use bevy_render::{
    batching::gpu_preprocessing::{
        BatchedInstanceBuffers, BuildIndirectParametersMetadata, GpuPreprocessingMode,
        IndirectParametersBuffers, PhaseBatchedInstanceBuffers,
    },
    diagnostic::RecordDiagnostics,
    extract_resource::ExtractResource,
    render_phase::{BinnedRenderPhase, ViewBinnedRenderPhases},
    render_resource::{
        binding_types::{storage_buffer_read_only_sized, texture_2d, uniform_buffer},
        BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BufferId,
        CachedRenderPipelineId, CompareFunction, DepthBiasState, DepthStencilState,
        DynamicBindGroupEntries, DynamicBindGroupLayoutEntries, DynamicUniformBuffer, Extent3d,
        FragmentState, LoadOp, Operations, PipelineCache, PrimitiveState,
        RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
        ShaderStages, ShaderType, StencilState, StorageBuffer, StoreOp, Texture, TextureAspect,
        TextureDescriptor, TextureDimension, TextureId, TextureSampleType, TextureUsages,
        TextureView, TextureViewDescriptor, TextureViewDimension, VertexState,
    },
    renderer::{RenderContext, RenderDevice, RenderQueue},
    sync_world::{MainEntity, RenderEntity},
    texture::{DepthStencilViewAttachment, DepthStencilViews},
    view::{
        ExtractedView, NoIndirectDrawing, RenderViewLodOrigins, RetainedViewEntity,
        SkipRootViewRendering, ViewUniformOffset,
    },
    Extract, RenderApp,
};
use bevy_shape::ViewFrustum;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::{
    record_allocate_uniforms_for_views, record_early_gpu_preprocess_for_views,
    record_unpack_bins_for_views, run_build_indirect_parameters, BinUnpackingBindGroups,
    BuildIndirectParametersBindGroups, ExtractedPointLight, LightEntity, MeshInputUniform,
    MeshUniform, PendingShadowQueues, PreprocessBindGroups, PreprocessPipelines, Shadow,
    ShadowView, SkipGpuPreprocess, UniformAllocationBindGroups, ALL_POINT_SHADOW_FACES,
    CORE_3D_SHADOW_MAP_FORMAT,
};

/// `@group(0)` binding numbers for the 2D point shadow atlas (after GI 39–42 and surface weather 43–53).
pub const POINT_SHADOW_ATLAS_STATIC_BINDING: u32 = 54;
pub const POINT_SHADOW_ATLAS_LIVE_BINDING: u32 = 55;
pub const POINT_SHADOW_ATLAS_TILES_BINDING: u32 = 56;

/// Main-world / extracted configuration for the 2D point shadow atlas (`shadow.lamp.atlas.*`).
#[derive(Resource, Clone, Copy, Debug, PartialEq, ExtractResource, Reflect)]
#[reflect(Resource, Debug, Clone)]
#[extract_app(RenderApp)]
pub struct PointShadowAtlasConfig {
    pub enabled: bool,
    pub static_size: u32,
    pub live_size: u32,
    pub guard_texels: f32,
    /// Rotated 4-tap PCF filter radius in tile texels (`0.0` = crisp 1-tap 2x2 HW PCF).
    pub soft_pcf_radius: f32,
    pub max_lamps: u32,
}

impl Default for PointShadowAtlasConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            static_size: 4096,
            live_size: 2048,
            guard_texels: 2.0,
            soft_pcf_radius: 1.0,
            max_lamps: 48,
        }
    }
}

/// Main-world component on a `PointLight` whose shadows live in the 2D Shadow Atlas.
/// Keep `PointLight::shadow_maps_enabled = false` while this component is attached so Bevy's
/// stock caster scan and hashing skip it; `extract_lights` shades it as shadowed automatically.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AtlasPointShadowMap {
    /// Per-face `[x, y, size]` in the Static Atlas safe to sample (and blit from).
    /// During a tier resize this keeps the retiring rect until the new rect is confirmed drawn.
    pub static_rects: [Option<[u16; 3]>; 6],
    /// Per-face `[x, y, size]` in the Static Atlas targeted by static captures (`None` = use
    /// `static_rects[face]`). Separated from `static_rects` so static captures can draw into a
    /// newly allocated tile while the shader keeps sampling the previous confirmed tile.
    pub draw_static_rects: [Option<[u16; 3]>; 6],
    /// Per-face `[x, y, size]` in the Live Atlas (`None` = sample Static Atlas or Empty).
    pub live_rects: [Option<[u16; 3]>; 6],
    /// Faces that keep a render view entity and retained shadow phase.
    pub view_faces: u8,
    /// Faces drawn this frame (into `draw_static_rects`/`static_rects` or `live_rects`).
    pub draw_faces: u8,
    /// Subset of `draw_faces`: target is `live_rects[face]` (else `draw_static_rects`/`static_rects`).
    pub draw_to_live: u8,
    /// Subset of `draw_to_live`: blit `static_rects[face] -> live_rects[face]` before drawing
    /// movers (when bit is 0, clear `live_rects[face]` to 0.0 before drawing).
    pub blit_from_static: u8,
    /// Subset of `draw_to_live`: draws movers only (no GPU instance batches), so it uses
    /// `NoIndirectDrawing` and shares ONE single `RenderPass` over the Live Atlas.
    pub mover_only_faces: u8,
    /// Host plan number echoed in [`AtlasPointShadowReport::plan`].
    pub plan: u64,
}

impl AtlasPointShadowMap {
    /// Sanitised masks: 6 faces only, `draw ⊆ view`, `draw_to_live ⊆ draw`,
    /// `blit_from_static ⊆ draw_to_live`, `mover_only_faces ⊆ draw_to_live`.
    #[must_use]
    pub fn sanitized(&self) -> Self {
        let draw_faces = self.draw_faces & ALL_POINT_SHADOW_FACES;
        let view_faces = (self.view_faces & ALL_POINT_SHADOW_FACES) | draw_faces;
        let draw_to_live = self.draw_to_live & draw_faces;
        let blit_from_static = self.blit_from_static & draw_to_live;
        let mover_only_faces = self.mover_only_faces & draw_to_live;
        Self {
            static_rects: self.static_rects,
            draw_static_rects: self.draw_static_rects,
            live_rects: self.live_rects,
            view_faces,
            draw_faces,
            draw_to_live,
            blit_from_static,
            mover_only_faces,
            plan: self.plan,
        }
    }

    /// Compares all fields except `plan` so idle lamps avoid per-frame `Changed` churn.
    #[must_use]
    pub fn same_state(&self, other: &Self) -> bool {
        self.static_rects == other.static_rects
            && self.draw_static_rects == other.draw_static_rects
            && self.live_rects == other.live_rects
            && self.view_faces == other.view_faces
            && self.draw_faces == other.draw_faces
            && self.draw_to_live == other.draw_to_live
            && self.blit_from_static == other.blit_from_static
            && self.mover_only_faces == other.mover_only_faces
    }
}

/// Render-world copy of [`AtlasPointShadowMap`] on the light's render entity.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ExtractedAtlasPointShadowMap {
    pub map: AtlasPointShadowMap,
    /// Written by the host this frame: draws apply only when `true`.
    pub fresh: bool,
}

/// Render-world feedback report for one atlas-shadowed lamp.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AtlasPointShadowReport {
    pub plan: u64,
    /// Static capture faces drawn with all pipelines ready and nothing pending.
    pub confirmed: u8,
    /// Atlas textures were recreated (size changed or newly enabled): recapture everything.
    pub lost: bool,
}

/// Main-world + render-world shared feedback channel for the 2D Shadow Atlas.
#[derive(Resource, Clone, Default)]
pub struct PointShadowAtlasFeedback(Arc<Mutex<EntityHashMap<AtlasPointShadowReport>>>);

impl PointShadowAtlasFeedback {
    #[must_use]
    pub fn drain(&self) -> Vec<(Entity, AtlasPointShadowReport)> {
        self.0
            .lock()
            .map(|mut map| map.drain().collect())
            .unwrap_or_default()
    }

    fn report(&self, light: Entity, update: impl FnOnce(&mut AtlasPointShadowReport)) {
        if let Ok(mut map) = self.0.lock() {
            update(map.entry(light).or_default());
        }
    }
}

/// Render-world counts for diagnostics (`shadow_atlas/render/*`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointShadowAtlasStats {
    pub lamps: u32,
    pub views: u32,
    pub static_draws: u32,
    pub live_indirect_draws: u32,
    pub live_direct_draws: u32,
    pub blits: u32,
    pub clears: u32,
    pub static_size: u32,
    pub live_size: u32,
}

/// GPU tile metadata entry (`16` bytes, mirrors `ShadowAtlasTile` in `mesh_view_types.wesl`).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct GpuShadowAtlasTile {
    pub uv_offset: Vec2,
    pub uv_scale: f32,
    /// Packed `{ atlas_kind: u2 (0=empty, 1=static, 2=live), tile_px: u11 (0..2047), pcf_radius_q3: u3 (0.25 px steps), inv_guard_tan_q16: u16 }`.
    pub flags_and_size: u32,
}

/// Atlas kind bits in [`GpuShadowAtlasTile::flags_and_size`].
pub const ATLAS_KIND_EMPTY: u32 = 0;
pub const ATLAS_KIND_STATIC: u32 = 1;
pub const ATLAS_KIND_LIVE: u32 = 2;

/// Guard-band `tan(fov/2)` for a face of `tile_px` pixels and `guard_texels` border per side.
#[must_use]
pub fn atlas_guard_tan_half(tile_px: u32, guard_texels: f32) -> f32 {
    let size = tile_px.max(1) as f32;
    let inner = (size - 2.0 * guard_texels.max(0.0)).max(1.0);
    size / inner
}

/// Guard-band inverse tangent `(tile_px - 2 * guard) / tile_px` used by `shadows.wesl`.
#[must_use]
pub fn atlas_guard_inv_tan(tile_px: u32, guard_texels: f32) -> f32 {
    1.0 / atlas_guard_tan_half(tile_px, guard_texels)
}

/// Packs one tile record for the GPU tile storage buffer.
#[must_use]
pub fn pack_shadow_atlas_tile(
    atlas_kind: u32,
    rect: [u16; 3],
    atlas_size: u32,
    guard_texels: f32,
    soft_pcf_radius: f32,
) -> GpuShadowAtlasTile {
    if atlas_kind == ATLAS_KIND_EMPTY || rect[2] == 0 || atlas_size == 0 {
        return GpuShadowAtlasTile::default();
    }
    let inv_atlas = 1.0 / atlas_size as f32;
    let tile_px = u32::from(rect[2]);
    let inv_guard_tan = atlas_guard_inv_tan(tile_px, guard_texels).clamp(0.0, 1.0);
    let inv_guard_q16 = (inv_guard_tan * 65535.0).round() as u32;
    let pcf_q3 = ((soft_pcf_radius.clamp(0.0, 1.75) * 4.0).round() as u32).min(7);
    let flags_and_size = (atlas_kind & 3)
        | ((tile_px & 0x7FF) << 2)
        | ((pcf_q3 & 7) << 13)
        | ((inv_guard_q16 & 0xFFFF) << 16);
    GpuShadowAtlasTile {
        uv_offset: Vec2::new(f32::from(rect[0]) * inv_atlas, f32::from(rect[1]) * inv_atlas),
        uv_scale: tile_px as f32 * inv_atlas,
        flags_and_size,
    }
}

/// Uniform for `point_shadow_atlas_blit.wesl` (`fs_blit`).
#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub struct AtlasBlitUniform {
    pub src_origin: IVec2,
    pub dst_origin: IVec2,
    pub src_size: i32,
    pub dst_size: i32,
    /// `tan_half(dst_size, guard) / tan_half(src_size, guard)` for exact guard-band UV remapping.
    pub guard_ratio: f32,
    pub _pad: f32,
}

/// Render-world marker on shadow views owned by the 2D Shadow Atlas (never routed through
/// `camera_driver` or `record_batched_point_shadows`).
#[derive(Component, Clone, Copy, Debug)]
pub struct AtlasShadowView {
    pub light_entity: Entity,
    pub main_light: MainEntity,
    pub face: u8,
    pub draws_this_frame: bool,
    pub is_static_capture: bool,
    pub plan: u64,
}

/// Cached per-face state of an atlas shadow view to skip redundant `ShadowView` / `ExtractedView` writes.
#[derive(Component, Clone, PartialEq)]
pub struct AtlasShadowFaceState {
    texture_id: TextureId,
    dst_rect: [u16; 3],
    guard_texels_bits: u32,
    world_from_view: GlobalTransform,
    range: f32,
    near_z: f32,
    render_layers: RenderLayers,
    light_entity: Entity,
    mover_only: bool,
}

/// One scheduled draw into the Static or Live 2D Shadow Atlas for this frame.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AtlasDrawJob {
    pub view_entity: Entity,
    pub retained: RetainedViewEntity,
    pub dst_rect: [u16; 3],
    pub to_live: bool,
    pub mover_only: bool,
    pub blit_offset: Option<u32>,
}

/// Pipelines for sub-rect depth clear and `Static -> Live` depth blit.
#[derive(Resource)]
pub struct PointShadowAtlasPipelines {
    pub clear_pipeline: CachedRenderPipelineId,
    pub blit_pipeline: CachedRenderPipelineId,
    pub blit_bind_group_layout: BindGroupLayoutDescriptor,
}

/// Embeds `point_shadow_atlas_blit.wesl` so `load_embedded_asset!` resolves in the same module.
pub(crate) fn embed_point_shadow_atlas_shaders(app: &mut App) {
    embedded_asset!(app, "point_shadow_atlas_blit.wesl");
}

/// Initializes the depth-clear and depth-blit pipelines at `RenderStartup`.
pub fn init_point_shadow_atlas_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let shader = load_embedded_asset!(asset_server.as_ref(), "point_shadow_atlas_blit.wesl");

    let blit_bind_group_layout = BindGroupLayoutDescriptor::new(
        "point_shadow_atlas_blit_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Depth),
                uniform_buffer::<AtlasBlitUniform>(true),
            ),
        ),
    );

    let depth_stencil = Some(DepthStencilState {
        format: CORE_3D_SHADOW_MAP_FORMAT,
        depth_write_enabled: Some(true),
        depth_compare: Some(CompareFunction::Always),
        stencil: StencilState::default(),
        bias: DepthBiasState::default(),
    });

    let clear_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("point_shadow_atlas_clear_pipeline".into()),
        layout: vec![],
        immediate_size: 0,
        vertex: VertexState {
            shader: shader.clone(),
            entry_point: Some("vs_main".into()),
            ..Default::default()
        },
        primitive: PrimitiveState {
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: depth_stencil.clone(),
        multisample: Default::default(),
        fragment: Some(FragmentState {
            shader: shader.clone(),
            entry_point: Some("fs_clear".into()),
            ..Default::default()
        }),
        zero_initialize_workgroup_memory: false,
    });

    let blit_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("point_shadow_atlas_blit_pipeline".into()),
        layout: vec![blit_bind_group_layout.clone()],
        immediate_size: 0,
        vertex: VertexState {
            shader: shader.clone(),
            entry_point: Some("vs_main".into()),
            ..Default::default()
        },
        primitive: PrimitiveState {
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil,
        multisample: Default::default(),
        fragment: Some(FragmentState {
            shader,
            entry_point: Some("fs_blit".into()),
            ..Default::default()
        }),
        zero_initialize_workgroup_memory: false,
    });

    commands.insert_resource(PointShadowAtlasPipelines {
        clear_pipeline,
        blit_pipeline,
        blit_bind_group_layout,
    });
}

/// Render-world GPU state of the 2D Shadow Atlas.
#[derive(Resource, Default)]
pub struct PointShadowAtlasGpu {
    static_atlas: Option<(Texture, TextureView, u32)>,
    live_atlas: Option<(Texture, TextureView, u32)>,
    tiles_buffer: StorageBuffer<Vec<GpuShadowAtlasTile>>,
    blit_uniforms: DynamicUniformBuffer<AtlasBlitUniform>,
    blit_bind_group: Option<(TextureId, BufferId, BindGroup)>,
    jobs: Vec<AtlasDrawJob>,
    face_views: EntityHashMap<[Option<Entity>; 6]>,
    origin_views_now: HashSet<RetainedViewEntity>,
    origin_views_last: HashSet<RetainedViewEntity>,
    active: bool,
    atlas_reallocated: bool,
    guard_texels: f32,
    soft_pcf_radius: f32,
}

impl PointShadowAtlasGpu {
    /// Whether the 2D Shadow Atlas is active and has valid textures + tile buffer for this frame.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.active
            && self.static_atlas.is_some()
            && self.live_atlas.is_some()
            && self.tiles_buffer.binding().is_some()
    }

    pub(crate) fn static_view(&self) -> Option<&TextureView> {
        self.static_atlas.as_ref().map(|(_, v, _)| v)
    }

    pub(crate) fn live_view(&self) -> Option<&TextureView> {
        self.live_atlas.as_ref().map(|(_, v, _)| v)
    }

    pub(crate) fn tiles_binding(&self) -> Option<bevy_render::render_resource::BindingResource<'_>> {
        self.tiles_buffer.binding()
    }
}

/// Appends `@group(0)` layout entries `54..=56` for `MeshPipelineViewLayoutKey::POINT_SHADOW_ATLAS`.
pub(crate) fn point_shadow_atlas_layout_entries(
    entries: DynamicBindGroupLayoutEntries,
) -> DynamicBindGroupLayoutEntries {
    entries.extend_with_indices((
        (
            POINT_SHADOW_ATLAS_STATIC_BINDING,
            texture_2d(TextureSampleType::Depth),
        ),
        (
            POINT_SHADOW_ATLAS_LIVE_BINDING,
            texture_2d(TextureSampleType::Depth),
        ),
        (
            POINT_SHADOW_ATLAS_TILES_BINDING,
            storage_buffer_read_only_sized(false, Some(GpuShadowAtlasTile::min_size())),
        ),
    ))
}

/// Appends `@group(0)` bind-group entries `54..=56` from [`PointShadowAtlasGpu`].
pub(crate) fn point_shadow_atlas_bind_entries<'a>(
    entries: DynamicBindGroupEntries<'a>,
    gpu: &'a PointShadowAtlasGpu,
) -> DynamicBindGroupEntries<'a> {
    let static_view = gpu.static_view().expect("atlas ready");
    let live_view = gpu.live_view().expect("atlas ready");
    let tiles = gpu.tiles_binding().expect("atlas ready");
    entries.extend_with_indices((
        (POINT_SHADOW_ATLAS_STATIC_BINDING, static_view),
        (POINT_SHADOW_ATLAS_LIVE_BINDING, live_view),
        (POINT_SHADOW_ATLAS_TILES_BINDING, tiles),
    ))
}

/// Extracts [`AtlasPointShadowMap`] from main world to render world.
pub fn extract_atlas_point_shadow_maps(
    mut commands: Commands,
    lamps: Extract<Query<(Entity, RenderEntity, Ref<AtlasPointShadowMap>), With<PointLight>>>,
    point_lights: Extract<Query<&PointLight>>,
    mut extracted_lights: Query<&mut ExtractedPointLight>,
    mut last: Local<EntityHashMap<Entity>>,
    mut now: Local<EntityHashMap<Entity>>,
) {
    now.clear();
    for (main, render, atlas) in &lamps {
        now.insert(main, render);
        let fresh = atlas.is_changed();
        let Ok(mut entity) = commands.get_entity(render) else {
            continue;
        };
        entity.insert(ExtractedAtlasPointShadowMap {
            map: atlas.sanitized(),
            fresh,
        });
    }
    for (main, render) in last.iter() {
        if now.contains_key(main) {
            continue;
        }
        if let Ok(mut entity) = commands.get_entity(*render) {
            entity.remove::<ExtractedAtlasPointShadowMap>();
        }
        if let Ok(mut light) = extracted_lights.get_mut(*render) {
            let stock = point_lights.get(*main).is_ok_and(|l| l.shadow_maps_enabled);
            if light.shadow_maps_enabled != stock {
                light.shadow_maps_enabled = stock;
            }
        }
    }
    core::mem::swap(&mut *last, &mut *now);
}

/// `prepare_lights`'s access to the 2D Shadow Atlas.
#[derive(SystemParam)]
pub struct PointShadowAtlasPrepare<'w, 's> {
    config: Option<Res<'w, PointShadowAtlasConfig>>,
    atlas_maps: Query<'w, 's, Read<ExtractedAtlasPointShadowMap>>,
    face_states: Query<'w, 's, Read<AtlasShadowFaceState>>,
    gpu: ResMut<'w, PointShadowAtlasGpu>,
    stats: ResMut<'w, PointShadowAtlasStats>,
    feedback: Option<Res<'w, PointShadowAtlasFeedback>>,
    pipelines: Option<Res<'w, PointShadowAtlasPipelines>>,
    pipeline_cache: Res<'w, PipelineCache>,
    shadow_phase_buffers: Option<ResMut<'w, PhaseBatchedInstanceBuffers<Shadow, MeshUniform>>>,
    batched_instance_buffers: Option<
        ResMut<'w, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    >,
    lights: Query<'w, 's, (), With<crate::PointAndSpotLightViewEntities>>,
}

/// One atlas lamp's inputs from `prepare_lights`'s light loop.
pub(crate) struct AtlasLightInputs<'a> {
    pub light_entity: Entity,
    pub light_main_entity: MainEntity,
    pub light: &'a ExtractedPointLight,
    pub light_render_layers: &'a RenderLayers,
    pub light_index: Option<usize>,
    pub cube_face_rotations: &'a [Transform],
    pub gpu_mode: GpuPreprocessingMode,
}

fn create_atlas_texture(device: &RenderDevice, label: &'static str, size: u32) -> (Texture, TextureView, u32) {
    let texture = device.create_texture(&TextureDescriptor {
        label: Some(label),
        size: Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: CORE_3D_SHADOW_MAP_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&TextureViewDescriptor {
        label: Some(label),
        format: None,
        dimension: Some(TextureViewDimension::D2),
        usage: None,
        aspect: TextureAspect::DepthOnly,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: Some(1),
    });
    (texture, view, size)
}

impl PointShadowAtlasPrepare<'_, '_> {
    #[must_use]
    pub(crate) fn is_enabled(&self) -> bool {
        self.config.as_deref().is_some_and(|c| c.enabled)
    }

    #[must_use]
    pub(crate) fn max_lamps(&self) -> usize {
        self.config
            .as_deref()
            .map_or(48, |c| c.max_lamps.max(1) as usize)
    }

    /// Before the light loop: ensures Static & Live atlas textures exist (or frees them if disabled)
    /// and prepares tile & blit buffers for this frame.
    pub(crate) fn begin_frame(
        &mut self,
        device: &RenderDevice,
        _queue: &RenderQueue,
        max_shadowed_index: usize,
    ) {
        *self.stats = PointShadowAtlasStats::default();
        let gpu = &mut *self.gpu;
        gpu.jobs.clear();
        gpu.blit_uniforms.clear();
        gpu.atlas_reallocated = false;

        let Some(cfg) = self.config.as_deref().filter(|c| c.enabled) else {
            gpu.active = false;
            gpu.static_atlas = None;
            gpu.live_atlas = None;
            gpu.blit_bind_group = None;
            return;
        };

        let max_dim = device.limits().max_texture_dimension_2d;
        let static_size = cfg.static_size.clamp(512, max_dim);
        let live_size = cfg.live_size.clamp(512, max_dim);
        gpu.guard_texels = cfg.guard_texels.max(0.0);
        gpu.soft_pcf_radius = cfg.soft_pcf_radius.clamp(0.0, 1.75);

        if gpu.static_atlas.as_ref().map(|(_, _, s)| *s) != Some(static_size) {
            gpu.static_atlas = Some(create_atlas_texture(
                device,
                "point_shadow_atlas_static",
                static_size,
            ));
            gpu.blit_bind_group = None;
            gpu.atlas_reallocated = true;
        }
        if gpu.live_atlas.as_ref().map(|(_, _, s)| *s) != Some(live_size) {
            gpu.live_atlas = Some(create_atlas_texture(
                device,
                "point_shadow_atlas_live",
                live_size,
            ));
            gpu.atlas_reallocated = true;
        }

        let tile_count = (max_shadowed_index.max(1)) * 6;
        let tiles = gpu.tiles_buffer.get_mut();
        tiles.clear();
        tiles.resize(tile_count, GpuShadowAtlasTile::default());

        self.stats.static_size = static_size;
        self.stats.live_size = live_size;
        gpu.active = true;
    }

    /// Prepares one point light if it carries [`ExtractedAtlasPointShadowMap`].
    /// Returns `true` if handled by the atlas (stock/CW-4 cube path skips it).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prepare_light(
        &mut self,
        commands: &mut Commands,
        inputs: &AtlasLightInputs,
        view_entities: &mut Vec<Entity>,
        shadow_render_phases: &mut ViewBinnedRenderPhases<Shadow>,
        light_key_cache: &mut crate::LightKeyCache,
        live_shadow_mapping_lights: &mut HashSet<RetainedViewEntity>,
        mut lod_origins: Option<&mut RenderViewLodOrigins>,
    ) -> bool {
        let light_entity = inputs.light_entity;
        let main = inputs.light_main_entity;
        let Ok(extracted) = self.atlas_maps.get(light_entity) else {
            if let Some(slots) = self.gpu.face_views.remove(&light_entity) {
                despawn_atlas_views(commands, core::mem::take(view_entities));
                purge_atlas_retained(
                    shadow_render_phases,
                    light_key_cache,
                    self.shadow_phase_buffers.as_deref_mut(),
                    self.batched_instance_buffers.as_deref_mut(),
                    main,
                    slot_mask(&slots),
                );
            }
            return false;
        };
        if !self.gpu.active {
            if let Some(slots) = self.gpu.face_views.remove(&light_entity) {
                despawn_atlas_views(commands, core::mem::take(view_entities));
                purge_atlas_retained(
                    shadow_render_phases,
                    light_key_cache,
                    self.shadow_phase_buffers.as_deref_mut(),
                    self.batched_instance_buffers.as_deref_mut(),
                    main,
                    slot_mask(&slots),
                );
            }
            return true;
        }

        let map = extracted.map;
        let gpu = &mut *self.gpu;
        let stats = &mut *self.stats;

        let slots = gpu.face_views.entry(light_entity).or_default();
        let mut known: Vec<Entity> = slots.iter().flatten().copied().collect();
        let mut listed = view_entities.clone();
        known.sort_unstable();
        listed.sort_unstable();
        if known != listed {
            despawn_atlas_views(commands, core::mem::take(view_entities));
            purge_atlas_retained(
                shadow_render_phases,
                light_key_cache,
                self.shadow_phase_buffers.as_deref_mut(),
                self.batched_instance_buffers.as_deref_mut(),
                main,
                slot_mask(slots),
            );
            *slots = [None; 6];
        }

        stats.lamps += 1;

        let Some(light_index) = inputs.light_index else {
            let faces = slot_mask(slots);
            let stale: Vec<Entity> = slots.iter_mut().filter_map(Option::take).collect();
            despawn_atlas_views(commands, stale);
            purge_atlas_retained(
                shadow_render_phases,
                light_key_cache,
                self.shadow_phase_buffers.as_deref_mut(),
                self.batched_instance_buffers.as_deref_mut(),
                main,
                faces,
            );
            view_entities.clear();
            return true;
        };

        let lost = gpu.atlas_reallocated;
        if let Some(feedback) = self.feedback.as_ref().filter(|_| lost || extracted.fresh) {
            feedback.report(main.id(), |report| {
                report.lost |= lost;
                if extracted.fresh {
                    report.plan = map.plan;
                }
            });
        }

        let (static_tex_id, static_view, static_size) = {
            let (t, v, s) = gpu.static_atlas.as_ref().expect("atlas active");
            (t.id(), v.clone(), *s)
        };
        let (live_tex_id, live_view, live_size) = {
            let (t, v, s) = gpu.live_atlas.as_ref().expect("atlas active");
            (t.id(), v.clone(), *s)
        };
        let guard_texels = gpu.guard_texels;
        let soft_pcf_radius = gpu.soft_pcf_radius;

        // Populate the 6 GPU tile records for this light (indexed by `light_index * 6 + face`).
        let base_tile = light_index * 6;
        let tiles = gpu.tiles_buffer.get_mut();
        if tiles.len() < base_tile + 6 {
            tiles.resize(base_tile + 6, GpuShadowAtlasTile::default());
        }
        for face_index in 0..6usize {
            let tile = if let Some(live_rect) = map.live_rects[face_index] {
                pack_shadow_atlas_tile(
                    ATLAS_KIND_LIVE,
                    live_rect,
                    live_size,
                    guard_texels,
                    soft_pcf_radius,
                )
            } else if let Some(static_rect) = map.static_rects[face_index] {
                pack_shadow_atlas_tile(
                    ATLAS_KIND_STATIC,
                    static_rect,
                    static_size,
                    guard_texels,
                    soft_pcf_radius,
                )
            } else {
                GpuShadowAtlasTile::default()
            };
            tiles[base_tile + face_index] = tile;
        }

        // Create/update face views only for `view_faces`.
        let light = inputs.light;
        let view_translation = GlobalTransform::from_translation(light.transform.translation());
        let direct_mode = match inputs.gpu_mode {
            GpuPreprocessingMode::Culling => GpuPreprocessingMode::PreprocessingOnly,
            other => other,
        };

        let slots = gpu.face_views.entry(light_entity).or_default();
        for (face_index, view_rotation) in inputs.cube_face_rotations.iter().enumerate().take(6) {
            let bit = 1u8 << face_index;
            let face = face_index as u32;
            let retained = RetainedViewEntity::new(main, None, face);

            if map.view_faces & bit == 0 {
                if let Some(view) = slots[face_index].take() {
                    despawn_atlas_views(commands, vec![view]);
                    purge_atlas_retained(
                        shadow_render_phases,
                        light_key_cache,
                        self.shadow_phase_buffers.as_deref_mut(),
                        self.batched_instance_buffers.as_deref_mut(),
                        main,
                        bit,
                    );
                }
                continue;
            }

            let draws_this_frame = extracted.fresh && (map.draw_faces & bit != 0);
            let existing_prev = slots[face_index].and_then(|v| self.face_states.get(v).ok());
            // On non-draw frames (Half/Far mover skips, deferred frames, Linger) keep the existing
            // view's mode, target texture, and rect so we never flip `mover_only` (`NoIndirectDrawing`)
            // and wipe its retained shadow phase / work-item buffers.
            let (to_live, mover_only, dst_rect) = if draws_this_frame {
                let to_live = map.draw_to_live & bit != 0;
                let mover_only = to_live && (map.mover_only_faces & bit != 0);
                let rect = if to_live {
                    map.live_rects[face_index]
                } else {
                    map.draw_static_rects[face_index].or(map.static_rects[face_index])
                };
                (to_live, mover_only, rect)
            } else if let Some(prev) = existing_prev {
                let to_live = prev.texture_id == live_tex_id;
                (to_live, prev.mover_only, Some(prev.dst_rect))
            } else {
                let to_live = map.live_rects[face_index].is_some();
                let mover_only = to_live;
                let rect = if to_live {
                    map.live_rects[face_index]
                } else {
                    map.draw_static_rects[face_index].or(map.static_rects[face_index])
                };
                (to_live, mover_only, rect)
            };

            let Some(dst_rect) = dst_rect else {
                if let Some(view) = slots[face_index].take() {
                    despawn_atlas_views(commands, vec![view]);
                    purge_atlas_retained(
                        shadow_render_phases,
                        light_key_cache,
                        self.shadow_phase_buffers.as_deref_mut(),
                        self.batched_instance_buffers.as_deref_mut(),
                        main,
                        bit,
                    );
                }
                continue;
            };

            let target_tex_id = if to_live { live_tex_id } else { static_tex_id };
            let target_view = if to_live { &live_view } else { &static_view };

            let view_entity = *slots[face_index].get_or_insert_with(|| commands.spawn_empty().id());
            stats.views += 1;

            let world_from_view = view_translation * *view_rotation;
            let previous = self.face_states.get(view_entity).ok();
            let mode_changed = previous.is_some_and(|p| p.mover_only != mover_only);
            let state = AtlasShadowFaceState {
                texture_id: target_tex_id,
                dst_rect,
                guard_texels_bits: guard_texels.to_bits(),
                world_from_view,
                range: light.range,
                near_z: light.shadow_map_near_z,
                render_layers: inputs.light_render_layers.clone(),
                light_entity,
                mover_only,
            };

            let face_gpu_mode = if mover_only {
                direct_mode
            } else {
                inputs.gpu_mode
            };

            if previous != Some(&state) {
                let needs_shadow_view = !previous.is_some_and(|p| p.texture_id == target_tex_id);
                if needs_shadow_view {
                    commands.entity(view_entity).insert(ShadowView {
                        depth_attachment: DepthStencilViewAttachment::new(
                            DepthStencilViews::DepthOnly {
                                depth_view: target_view.clone(),
                            },
                            None,
                            None,
                        ),
                        pass_name: format!(
                            "shadow_point_atlas_{}_{}",
                            light_index,
                            face_index_to_name(face_index)
                        ),
                    });
                }

                let tan_half = atlas_guard_tan_half(u32::from(dst_rect[2]), guard_texels);
                let fov_y = 2.0 * tan_half.atan();
                let projection = proj::perspective_infinite_reverse(fov_y, 1.0, light.shadow_map_near_z);
                let clip_from_world = projection * Mat4::from(world_from_view.affine().inverse());
                let face_backward = world_from_view.back();
                let frustum = Frustum(ViewFrustum::from_clip_from_world_custom_far(
                    &clip_from_world,
                    &light.transform.translation(),
                    &face_backward,
                    light.range,
                ));

                commands.entity(view_entity).insert((
                    ExtractedView {
                        retained_view_entity: retained,
                        viewport: UVec4::new(
                            u32::from(dst_rect[0]),
                            u32::from(dst_rect[1]),
                            u32::from(dst_rect[2]),
                            u32::from(dst_rect[2]),
                        ),
                        world_from_view,
                        clip_from_world: None,
                        clip_from_view: projection,
                        target_format: CORE_3D_SHADOW_MAP_FORMAT,
                        color_grading: Default::default(),
                        invert_culling: false,
                    },
                    frustum,
                    LightEntity::Point {
                        light_entity,
                        face_index,
                    },
                    inputs.light_render_layers.clone(),
                    state,
                ));

                if !matches!(face_gpu_mode, GpuPreprocessingMode::Culling) {
                    commands.entity(view_entity).insert(NoIndirectDrawing);
                } else {
                    commands.entity(view_entity).remove::<NoIndirectDrawing>();
                }
            }

            commands.entity(view_entity).insert(AtlasShadowView {
                light_entity,
                main_light: main,
                face: face_index as u8,
                draws_this_frame,
                is_static_capture: draws_this_frame && !to_live,
                plan: map.plan,
            });

            if draws_this_frame {
                let blit_offset = if to_live
                    && (map.blit_from_static & bit != 0)
                    && let Some(src_rect) = map.static_rects[face_index]
                    && src_rect[2] > 0
                    && dst_rect[2] > 0
                {
                    let guard_ratio = if src_rect[2] == dst_rect[2] {
                        1.0
                    } else {
                        atlas_guard_tan_half(u32::from(dst_rect[2]), guard_texels)
                            / atlas_guard_tan_half(u32::from(src_rect[2]), guard_texels)
                    };
                    let offset = gpu.blit_uniforms.push(&AtlasBlitUniform {
                        src_origin: IVec2::new(i32::from(src_rect[0]), i32::from(src_rect[1])),
                        dst_origin: IVec2::new(i32::from(dst_rect[0]), i32::from(dst_rect[1])),
                        src_size: i32::from(src_rect[2]),
                        dst_size: i32::from(dst_rect[2]),
                        guard_ratio,
                        _pad: 0.0,
                    });
                    stats.blits += 1;
                    Some(offset)
                } else {
                    stats.clears += 1;
                    None
                };

                if !to_live {
                    stats.static_draws += 1;
                } else if mover_only {
                    stats.live_direct_draws += 1;
                } else {
                    stats.live_indirect_draws += 1;
                }

                gpu.jobs.push(AtlasDrawJob {
                    view_entity,
                    retained,
                    dst_rect,
                    to_live,
                    mover_only,
                    blit_offset,
                });
            }

            if let Some(origins) = lod_origins.as_deref_mut() {
                origins.0.insert(retained, light.transform.translation());
                gpu.origin_views_now.insert(retained);
            }
            if previous.is_none() || mode_changed {
                shadow_render_phases.remove(&retained);
                // Dropping the phase without forgetting `light_key_cache` leaves
                // `dirty_specializations.views` unset, so `queue_shadows` only queues
                // `added_entities()` and silently drops every already-visible caster
                // (SH-33 disappearing shadows on mode_changed / resize / resume).
                light_key_cache.remove(&retained);
                // `get_or_create_work_item_buffer` reuses an occupied `RetainedViewEntity`
                // entry without checking `no_indirect_drawing`, so clear both double-buffered
                // maps whenever a face view starts fresh or switches between Direct and Indirect.
                if let Some(phase_bufs) = self.shadow_phase_buffers.as_deref_mut() {
                    phase_bufs.buffers.work_item_buffers.remove(&retained);
                }
                if let Some(batched) = self.batched_instance_buffers.as_deref_mut()
                    && let Some(phase_bufs) = batched
                        .phase_instance_buffers
                        .get_mut(&core::any::TypeId::of::<Shadow>())
                {
                    phase_bufs.work_item_buffers.remove(&retained);
                }
            }
            shadow_render_phases.prepare_for_new_frame(retained, face_gpu_mode);
            live_shadow_mapping_lights.insert(retained);
        }

        view_entities.clear();
        view_entities.extend(slots.iter().flatten().copied());
        true
    }

    /// After the light loop: writes GPU buffers, refreshes the blit bind group, and cleans up
    /// despawned lights' state.
    pub(crate) fn end_frame(
        &mut self,
        device: &RenderDevice,
        queue: &RenderQueue,
        mut lod_origins: Option<&mut RenderViewLodOrigins>,
    ) {
        let gpu = &mut *self.gpu;
        let lights = &self.lights;
        gpu.face_views.retain(|light, _| lights.contains(*light));
        if let Some(origins) = lod_origins.as_deref_mut() {
            for retained in gpu.origin_views_last.difference(&gpu.origin_views_now) {
                origins.0.remove(retained);
            }
        }
        core::mem::swap(&mut gpu.origin_views_last, &mut gpu.origin_views_now);
        gpu.origin_views_now.clear();

        if !gpu.active {
            return;
        }

        gpu.tiles_buffer.write_buffer(device, queue);
        if !gpu.blit_uniforms.is_empty() {
            gpu.blit_uniforms.write_buffer(device, queue);
            if let (Some((static_tex, static_view, _)), Some(blit_buf), Some(pipelines)) = (
                &gpu.static_atlas,
                gpu.blit_uniforms.buffer(),
                self.pipelines.as_deref(),
            ) {
                let tex_id = static_tex.id();
                let buf_id = blit_buf.id();
                let needs_rebuild = !gpu
                    .blit_bind_group
                    .as_ref()
                    .is_some_and(|(t, b, _)| *t == tex_id && *b == buf_id);
                if needs_rebuild {
                    let layout = self
                        .pipeline_cache
                        .get_bind_group_layout(&pipelines.blit_bind_group_layout);
                    let bg = device.create_bind_group(
                        "point_shadow_atlas_blit_bg",
                        &layout,
                        &BindGroupEntries::sequential((
                            static_view,
                            gpu.blit_uniforms.binding().expect("blit uniform buffer"),
                        )),
                    );
                    gpu.blit_bind_group = Some((tex_id, buf_id, bg));
                }
            }
        }
    }
}

/// Forgets the retained shadow phase and both double-buffered work-item buffers of `faces` of a
/// light's atlas views being despawned. The stock / CW-4 path reuses the same
/// `RetainedViewEntity` per face, and `get_or_create_work_item_buffer` keeps an occupied entry
/// without checking `no_indirect_drawing`: a mover-only (direct) atlas face handed to the stock
/// indirect path bound the direct preprocess bind group to the GPU-culling pipeline (wgpu
/// validation error, "build mesh uniforms direct bind group layout ... not compatible", live
/// `set shadow.lamp.atlas.enabled off`, SH-33 2026-11-20).
fn purge_atlas_retained(
    shadow_render_phases: &mut ViewBinnedRenderPhases<Shadow>,
    light_key_cache: &mut crate::LightKeyCache,
    shadow_phase_buffers: Option<&mut PhaseBatchedInstanceBuffers<Shadow, MeshUniform>>,
    batched_instance_buffers: Option<&mut BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    main: MainEntity,
    faces: u8,
) {
    let mut phase_bufs = shadow_phase_buffers;
    let mut batched = batched_instance_buffers.and_then(|b| {
        b.phase_instance_buffers
            .get_mut(&core::any::TypeId::of::<Shadow>())
    });
    for face in 0..6u32 {
        if faces & (1 << face) == 0 {
            continue;
        }
        let retained = RetainedViewEntity::new(main, None, face);
        shadow_render_phases.remove(&retained);
        light_key_cache.remove(&retained);
        if let Some(bufs) = phase_bufs.as_deref_mut() {
            bufs.buffers.work_item_buffers.remove(&retained);
        }
        if let Some(bufs) = batched.as_deref_mut() {
            bufs.work_item_buffers.remove(&retained);
        }
    }
}

/// Bitmask of the faces that hold a view entity.
fn slot_mask(slots: &[Option<Entity>; 6]) -> u8 {
    slots
        .iter()
        .enumerate()
        .fold(0u8, |m, (i, s)| if s.is_some() { m | (1 << i) } else { m })
}

fn despawn_atlas_views(commands: &mut Commands, views: Vec<Entity>) {
    for view in views {
        if let Ok(mut entity) = commands.get_entity(view) {
            entity.try_despawn();
        }
    }
}

/// Runs right after `skip_unchanged_shadow_views` (and before `ApplyDeferred` +
/// `prepare_view_uniforms`): sets/clears `SkipRootViewRendering` on [`AtlasShadowView`] entities
/// and reports confirmed static captures to [`PointShadowAtlasFeedback`].
pub fn finalize_point_shadow_atlas_views(
    mut commands: Commands,
    views: Query<(Entity, &ExtractedView, &AtlasShadowView, Has<SkipRootViewRendering>)>,
    phases: Res<ViewBinnedRenderPhases<Shadow>>,
    pipeline_cache: Res<PipelineCache>,
    pending: Res<PendingShadowQueues>,
    feedback: Option<Res<PointShadowAtlasFeedback>>,
) {
    for (view_entity, view, atlas, skipped) in &views {
        if atlas.draws_this_frame {
            if skipped {
                commands.entity(view_entity).remove::<SkipRootViewRendering>();
            }
            if atlas.is_static_capture {
                let retained = view.retained_view_entity;
                let complete = phases
                    .get(&retained)
                    .is_some_and(|phase| atlas_phase_ready(phase, &pipeline_cache))
                    && pending
                        .get(&retained)
                        .is_none_or(|queues| queues.current_frame.is_empty());
                if complete && let Some(feedback) = feedback.as_deref() {
                    feedback.report(atlas.main_light.id(), |report| {
                        report.plan = atlas.plan;
                        report.confirmed |= 1 << atlas.face;
                    });
                }
            }
        } else if !skipped {
            commands.entity(view_entity).insert(SkipRootViewRendering);
        }
    }
}

fn atlas_phase_ready(phase: &BinnedRenderPhase<Shadow>, cache: &PipelineCache) -> bool {
    let ready = |id: CachedRenderPipelineId| cache.get_render_pipeline(id).is_some();
    phase.multidrawable_meshes.keys().all(|key| ready(key.pipeline))
        && phase.batchable_meshes.keys().all(|(key, _)| ready(key.pipeline))
        && phase.unbatchable_meshes.keys().all(|(key, _)| ready(key.pipeline))
        && phase.non_mesh_items.keys().all(|(key, _)| ready(key.pipeline))
        && phase.instance_batches.keys().all(|(key, _)| ready(key.pipeline))
}

#[derive(SystemParam)]
pub struct PointShadowAtlasRenderParams<'w, 's> {
    world: &'w World,
    gpu: Res<'w, PointShadowAtlasGpu>,
    pipelines: Option<Res<'w, PointShadowAtlasPipelines>>,
    view_flags: Query<
        'w,
        's,
        (
            Has<PreprocessBindGroups>,
            Has<SkipGpuPreprocess>,
            Has<NoIndirectDrawing>,
        ),
        With<AtlasShadowView>,
    >,
    simple_view_query: Query<'w, 's, &'static ExtractedView, Without<SkipGpuPreprocess>>,
    preprocess_view_query: Query<
        'w,
        's,
        (
            &'static ExtractedView,
            Option<&'static PreprocessBindGroups>,
            Option<&'static ViewUniformOffset>,
            Has<NoIndirectDrawing>,
            Has<bevy_render::occlusion_culling::OcclusionCulling>,
        ),
        Without<SkipGpuPreprocess>,
    >,
    shadow_render_phases: Res<'w, ViewBinnedRenderPhases<Shadow>>,
    batched_instance_buffers: Option<Res<'w, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>>,
    pipeline_cache: Res<'w, PipelineCache>,
    preprocess_pipelines: Option<Res<'w, PreprocessPipelines>>,
    uniform_allocation_bind_groups: Option<Res<'w, UniformAllocationBindGroups>>,
    bin_unpacking_bind_groups: Option<Res<'w, BinUnpackingBindGroups>>,
    build_indirect_params_bind_groups: Option<Res<'w, BuildIndirectParametersBindGroups>>,
    indirect_parameters_buffers: Option<Res<'w, IndirectParametersBuffers>>,
    build_indirect_parameters_metadata: Option<Res<'w, BuildIndirectParametersMetadata>>,
}

/// Records all 2D Shadow Atlas GPU preprocessing, depth clears/blits, and shadow draws into ONE
/// single [`RenderContext`] command encoder. Opens zero encoders when no atlas tile draws.
pub fn record_point_shadow_atlas(params: PointShadowAtlasRenderParams, mut ctx: RenderContext) {
    let PointShadowAtlasRenderParams {
        world,
        gpu,
        pipelines,
        view_flags,
        simple_view_query,
        preprocess_view_query,
        shadow_render_phases,
        batched_instance_buffers,
        pipeline_cache,
        preprocess_pipelines,
        uniform_allocation_bind_groups,
        bin_unpacking_bind_groups,
        build_indirect_params_bind_groups,
        indirect_parameters_buffers,
        build_indirect_parameters_metadata,
    } = params;

    if !gpu.is_ready() || gpu.jobs.is_empty() {
        return;
    }
    let Some(pipelines) = pipelines.as_deref() else {
        return;
    };
    let Some(clear_pipeline) = pipeline_cache.get_render_pipeline(pipelines.clear_pipeline) else {
        return;
    };
    let blit_pipeline = pipeline_cache.get_render_pipeline(pipelines.blit_pipeline);
    let blit_bg = gpu.blit_bind_group.as_ref().map(|(_, _, bg)| bg);
    let static_view = gpu.static_view().expect("atlas ready");
    let live_view = gpu.live_view().expect("atlas ready");

    let active_ids: Vec<Entity> = gpu.jobs.iter().map(|j| j.view_entity).collect();

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let time_span = diagnostics.time_span(ctx.command_encoder(), "point_shadow_atlas");

    // 1. Batched GPU preprocessing over all active atlas draw views (1 compute pass per stage).
    if let (Some(batched_instance_buffers), Some(preprocess_pipelines)) = (
        batched_instance_buffers.as_deref(),
        preprocess_pipelines.as_deref(),
    ) {
        if let Some(uniform_allocation_bind_groups) = uniform_allocation_bind_groups.as_deref() {
            record_allocate_uniforms_for_views(
                &active_ids,
                &simple_view_query,
                batched_instance_buffers,
                &pipeline_cache,
                preprocess_pipelines,
                uniform_allocation_bind_groups,
                &mut ctx,
            );
        }
        if let Some(bin_unpacking_bind_groups) = bin_unpacking_bind_groups.as_deref() {
            record_unpack_bins_for_views(
                &active_ids,
                &simple_view_query,
                batched_instance_buffers,
                &pipeline_cache,
                preprocess_pipelines,
                bin_unpacking_bind_groups,
                &mut ctx,
            );
        }
        record_early_gpu_preprocess_for_views(
            &active_ids,
            &preprocess_view_query,
            batched_instance_buffers,
            &pipeline_cache,
            preprocess_pipelines,
            &mut ctx,
        );
    }

    // 2. Static captures (0-4/frame) & indirect moving-lamp draws (need per-view indirect params
    //    because `IndirectParametersBuffers` is global per phase).
    for job in gpu.jobs.iter().filter(|j| !j.to_live || !j.mover_only) {
        if let Ok((has_bg, skip_gpu, no_indirect)) = view_flags.get(job.view_entity)
            && has_bg
            && !skip_gpu
            && !no_indirect
            && let (Some(preprocess_pipelines), Some(build_indirect_parameters_metadata)) = (
                preprocess_pipelines.as_deref(),
                build_indirect_parameters_metadata.as_deref(),
            )
        {
            run_build_indirect_parameters(
                &mut ctx,
                job.retained,
                build_indirect_params_bind_groups.as_deref(),
                &pipeline_cache,
                indirect_parameters_buffers.as_deref(),
                build_indirect_parameters_metadata,
                &preprocess_pipelines.early_phase,
                "point_shadow_atlas_indirect_params",
            );
        }

        let target_view = if job.to_live { live_view } else { static_view };
        let label = if job.to_live {
            "point_shadow_atlas_live_indirect"
        } else {
            "point_shadow_atlas_static_capture"
        };
        let mut render_pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: target_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_atlas_tile(
            &mut render_pass,
            job,
            clear_pipeline,
            blit_pipeline,
            blit_bg,
            &shadow_render_phases,
            world,
        );
    }

    // 3. All active mover-only faces share ONE SINGLE `RenderPass` on the Live Atlas!
    let has_live_direct = gpu.jobs.iter().any(|j| j.to_live && j.mover_only);
    if has_live_direct {
        let mut render_pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("point_shadow_atlas_live_movers"),
            color_attachments: &[],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: live_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        for job in gpu.jobs.iter().filter(|j| j.to_live && j.mover_only) {
            render_atlas_tile(
                &mut render_pass,
                job,
                clear_pipeline,
                blit_pipeline,
                blit_bg,
                &shadow_render_phases,
                world,
            );
        }
    }

    time_span.end(ctx.command_encoder());
}

fn render_atlas_tile<'w>(
    render_pass: &mut bevy_render::render_phase::TrackedRenderPass<'w>,
    job: &AtlasDrawJob,
    clear_pipeline: &'w bevy_render::render_resource::RenderPipeline,
    blit_pipeline: Option<&'w bevy_render::render_resource::RenderPipeline>,
    blit_bg: Option<&'w BindGroup>,
    shadow_render_phases: &'w ViewBinnedRenderPhases<Shadow>,
    world: &'w World,
) {
    let [x, y, size] = job.dst_rect;
    render_pass.set_viewport(
        f32::from(x),
        f32::from(y),
        f32::from(size),
        f32::from(size),
        0.0,
        1.0,
    );
    render_pass.set_scissor_rect(u32::from(x), u32::from(y), u32::from(size), u32::from(size));

    if let (Some(offset), Some(blit_pipe), Some(bg)) = (job.blit_offset, blit_pipeline, blit_bg) {
        render_pass.set_render_pipeline(blit_pipe);
        render_pass.set_bind_group(0, bg, &[offset]);
        render_pass.draw(0..3, 0..1);
    } else {
        render_pass.set_render_pipeline(clear_pipeline);
        render_pass.draw(0..3, 0..1);
    }

    if let Some(shadow_phase) = shadow_render_phases.get(&job.retained)
        && let Err(err) = shadow_phase.render(render_pass, world, job.view_entity)
    {
        tracing::error!("Error rendering point shadow atlas tile: {err:?}");
    }
}

/// Whether `extract_lights` keeps a light's list for shadow subview `subview`: every face of a
/// stock lamp, or only `view_faces` of an atlas lamp.
pub(crate) fn atlas_face_listed(atlas: Option<u8>, subview: u32) -> bool {
    atlas.is_none_or(|mask| subview < 6 && u32::from(mask) & (1 << subview) != 0)
}
