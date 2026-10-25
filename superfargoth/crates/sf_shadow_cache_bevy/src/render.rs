//! Render world: the persistent cube array, capture views (static casters into
//! the cache, [`spawn_capture_views`]), the restore copy before Bevy's shadow pass,
//! and the slim [`LampShadowView`] schedule. Cached faces with no mover lose
//! their Bevy view right after `prepare_lights` ([`detach_views`]) and are
//! restored, when stale, once per frame from the camera's schedule ([`restore_detached`]).

use core::any::TypeId;
use std::collections::HashMap;

use bevy_app::App;
use bevy_asset::{load_embedded_asset, Handle};
use bevy_camera::primitives::Frustum;
use bevy_camera::visibility::RenderLayers;
use bevy_core_pipeline::schedule::{Core3d, RootNonCameraView};
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::common_conditions::any_match_filter;
use bevy_ecs::schedule::{Schedule, ScheduleBuildSettings, ScheduleLabel};
use bevy_ecs::system::SystemChangeTick;
use bevy_mesh::Mesh3d;
use bevy_pbr::{
    allocate_uniforms, early_gpu_preprocess, early_prepass_build_indirect_parameters, late_gpu_preprocess, late_prepass_build_indirect_parameters,
    prepare_lights, shared_shadow_pass, unpack_bins, ExtractedPointLight, GlobalClusterableObjectMeta, LightEntity, PreprocessBindGroups, Shadow,
    ShadowBatchSetKey,
    ShadowView, SkipGpuPreprocess, ViewShadowBindings, CORE_3D_SHADOW_MAP_FORMAT, EARLY_SHADOW_PASS, LATE_SHADOW_PASS,
};
use bevy_render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::occlusion_culling::OcclusionCulling;
use bevy_render::render_phase::ViewBinnedRenderPhases;
use bevy_render::render_resource::binding_types::texture_depth_2d;
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, CachedRenderPipelineId, CompareFunction, DepthBiasState,
    DepthStencilState, Extent3d, FragmentState, PipelineCache, RenderPassDescriptor, RenderPipelineDescriptor, ShaderStages, StencilState,
    StoreOp, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureId, TextureUsages, TextureView, TextureViewDescriptor,
    TextureViewDimension,
};
use bevy_render::renderer::{RenderContext, RenderDevice, ViewQuery};
use bevy_render::sync_world::MainEntity;
use bevy_render::texture::{DepthStencilViewAttachment, DepthStencilViews};
use bevy_render::view::{
    collect_visible_cpu_culled_entities, ExtractedView, NoIndirectDrawing, RenderExtractedShadowMapVisibleEntities, RenderExtractedVisibleEntities,
    RenderExtractedVisibleEntitiesClass, RenderShadowMapVisibleEntities, RenderVisibleEntities, RetainedViewEntity,
};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;

use crate::SharedCaptures;

/// One cached light this frame (main world → render world).
#[derive(Clone, Copy, Debug)]
pub(crate) struct CachedLight {
    /// The light's main-world entity.
    pub(crate) light: Entity,
    pub(crate) slot: u32,
    /// Capture (store Bevy's faces) or restore (load ours into Bevy's).
    pub(crate) capture: bool,
    pub(crate) generation: u32,
    /// Faces whose visible list holds a moving caster (bit per face).
    pub(crate) movers: u8,
}

/// A light whose static casters are drawn into its cache slot this frame:
/// per face, `(render entity, main entity)` sorted (Bevy's visible lists).
#[derive(Clone, Debug)]
pub(crate) struct CaptureRequest {
    pub(crate) light: Entity,
    pub(crate) slot: u32,
    pub(crate) faces: [Vec<(Entity, MainEntity)>; 6],
}

/// The frame's cached lights and the cache texture's shape.
#[derive(Resource, Clone, Default)]
pub(crate) struct CacheFrame {
    pub(crate) map_size: u32,
    pub(crate) slots: u32,
    pub(crate) lights: Vec<CachedLight>,
    /// Lights capturing this frame (at most `captures_per_frame`).
    pub(crate) captures: Vec<CaptureRequest>,
    /// Trust Bevy's cube texture to keep a detached face between frames
    /// (`ShadowCacheSettings::reuse_layers`).
    pub(crate) reuse: bool,
}

pub(crate) fn build(app: &mut App, shared: SharedCaptures) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<CacheFrame>()
        .init_resource::<CacheTexture>()
        .init_resource::<LightModes>()
        .init_resource::<Detached>()
        .init_resource::<CleanLayers>()
        .insert_resource(shared)
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<CopyPipeline>)
        .add_systems(
            Render,
            (
                inject_captures.in_set(RenderSystems::PrepareAssets).before(collect_visible_cpu_culled_entities),
                (prepare, spawn_capture_views, detach_views).chain().in_set(RenderSystems::CreateViews).after(prepare_lights),
            ),
        );
}

/// The schedule a point light's cube-face view runs instead of the whole
/// `Core3d` (shadows v2 V1): GPU preprocessing, our restore, Bevy's shadow
/// passes, our capture. `Core3d` runs every camera system once per face
/// (each checks the view and skips); with dozens of host systems that was
/// most of a face's ~0.35 ms. Same split as Bevy PR #26149
/// (`Core3dShadows`, open); drop ours when Bevy ships it.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LampShadowView;

pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    let mut schedule = Schedule::new(LampShadowView);
    schedule.set_build_settings(ScheduleBuildSettings { auto_insert_apply_deferred: false, ..Default::default() });
    render_app.add_schedule(schedule);
    render_app.add_systems(
        LampShadowView,
        (
            (
                allocate_uniforms,
                unpack_bins,
                early_gpu_preprocess,
                early_prepass_build_indirect_parameters.run_if(any_match_filter::<(
                    With<PreprocessBindGroups>,
                    Without<SkipGpuPreprocess>,
                    Without<NoIndirectDrawing>,
                    With<ShadowView>,
                )>),
            )
                .chain(),
            restore_pass,
            shared_shadow_pass::<EARLY_SHADOW_PASS>,
            (
                late_gpu_preprocess,
                late_prepass_build_indirect_parameters.run_if(any_match_filter::<(
                    With<PreprocessBindGroups>,
                    Without<SkipGpuPreprocess>,
                    Without<NoIndirectDrawing>,
                    With<ShadowView>,
                    With<OcclusionCulling>,
                )>),
            )
                .chain(),
            shared_shadow_pass::<LATE_SHADOW_PASS>,
            confirm_capture,
        )
            .chain(),
    );
    render_app.add_systems(Core3d, restore_detached.before(shared_shadow_pass::<EARLY_SHADOW_PASS>));
}

fn extract(mut frame: ResMut<CacheFrame>, main: Extract<Res<CacheFrame>>) {
    frame.clone_from(&main);
}

#[derive(Resource)]
struct CopyPipeline {
    layout: BindGroupLayoutDescriptor,
    id: CachedRenderPipelineId,
}

impl FromWorld for CopyPipeline {
    fn from_world(world: &mut World) -> Self {
        let layout = BindGroupLayoutDescriptor::new(
            "shadow_cache_copy",
            &BindGroupLayoutEntries::single(ShaderStages::FRAGMENT, texture_depth_2d()),
        );
        let fullscreen = world.resource::<FullscreenShader>().clone();
        let shader: Handle<Shader> = load_embedded_asset!(world, "copy_depth.wesl");
        let descriptor = RenderPipelineDescriptor {
            label: Some("shadow_cache_copy".into()),
            layout: vec![layout.clone()],
            vertex: fullscreen.to_vertex_state(),
            fragment: Some(FragmentState { shader, entry_point: Some("copy_depth".into()), targets: vec![], ..FragmentState::default() }),
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_SHADOW_MAP_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::Always),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            ..RenderPipelineDescriptor::default()
        };
        let id = world.resource::<PipelineCache>().queue_render_pipeline(descriptor);
        Self { layout, id }
    }
}

/// The persistent cube array: `slots × 6` depth layers, one view and one
/// restore bind group per layer. Kept across frames; recreated (and every
/// light recaptured, decided on the main side) only when its shape changes.
#[derive(Resource, Default)]
struct CacheTexture {
    shape: (u32, u32),
    texture: Option<Texture>,
    layers: Vec<TextureView>,
    restore: Vec<BindGroup>,
}

/// This frame's cached lights by render-world light entity.
#[derive(Resource, Default)]
struct LightModes(HashMap<Entity, CachedLight>);

/// Face views taken out of Bevy's camera driver this frame (view entity,
/// cache layer), restored once by [`restore_detached`].
#[derive(Resource, Default)]
struct Detached {
    faces: Vec<(Entity, u32)>,
    restored: bool,
    /// Report counters: frames, detached faces, faces restored, texture changes.
    counts: [u64; 4],
}

/// Bevy's cube-texture layers known to hold a cached light's static face
/// (what [`restore_detached`] wrote, nothing drawn over it since), and the
/// texture they are in. Bevy takes the cube array from its `TextureCache`
/// each frame: the same texture while the shadowed-light count (its
/// descriptor) is unchanged, and nothing clears a layer whose view is
/// detached. So a detached face is restored only when its layer is not
/// clean: a new texture, the light moved to another layer (Bevy's sort),
/// a new capture generation or slot, or movers were drawn over it.
#[derive(Resource, Default)]
struct CleanLayers {
    texture: Option<TextureId>,
    /// Bevy layer → (light main entity, face, generation, slot).
    layers: HashMap<u32, (Entity, u32, u32, u32)>,
}

/// Right after `prepare_lights`: this frame's cached lights by render
/// entity, and the faces of restoring lights that hold no mover leave Bevy's
/// camera driver (no `RootNonCameraView`: Bevy runs no schedule for them).
/// Bevy re-inserts the marker when a light changes (a flicker is a change),
/// so this runs every frame.
fn detach_views(
    mut commands: Commands,
    frame: Res<CacheFrame>,
    mut modes: ResMut<LightModes>,
    mut detached: ResMut<Detached>,
    lights: Query<(Entity, &MainEntity), With<ExtractedPointLight>>,
    views: Query<(Entity, &LightEntity), With<RootNonCameraView>>,
    (meta, bindings, mut clean): (Res<GlobalClusterableObjectMeta>, Query<&ViewShadowBindings>, ResMut<CleanLayers>),
) {
    modes.0.clear();
    detached.faces.clear();
    detached.restored = false;
    // Every point face view (cached or not) runs the slim schedule.
    for (view, light) in &views {
        if matches!(light, LightEntity::Point { .. }) {
            commands.entity(view).insert(RootNonCameraView(LampShadowView.intern()));
        }
    }
    let texture = bindings.iter().next().map(|b| b.point_light_depth_texture.id());
    if frame.lights.is_empty() || frame.slots == 0 || !frame.reuse || texture != clean.texture {
        clean.layers.clear();
    }
    if texture != clean.texture {
        detached.counts[3] += 1;
    }
    clean.texture = texture;
    detached.counts[0] += 1;
    if detached.counts[0].is_multiple_of(600) {
        let [frames, faces, restored, textures] = detached.counts;
        log::debug!("shadow cache: {frames} frames, {faces} detached faces, {restored} restored, {textures} cube textures");
    }
    if frame.lights.is_empty() || frame.slots == 0 {
        return;
    }
    let mut still = HashMap::with_capacity(clean.layers.len());
    let by_main: HashMap<Entity, CachedLight> = frame.lights.iter().map(|l| (l.light, *l)).collect();
    for (entity, main) in &lights {
        if let Some(light) = by_main.get(&main.id()) {
            modes.0.insert(entity, *light);
        }
    }
    for (view, light) in &views {
        let Some((cached, face)) = cached_face(light, &modes) else { continue };
        if !cached.capture && cached.movers & (1 << face) == 0 {
            commands.entity(view).remove::<RootNonCameraView>();
            let LightEntity::Point { light_entity, .. } = light else { continue };
            let Some(index) = meta.entity_to_index.get(light_entity) else { continue };
            let layer = u32::try_from(*index).unwrap_or(u32::MAX / 8) * 6 + face;
            let key = (cached.light, face, cached.generation, cached.slot);
            if clean.layers.get(&layer) != Some(&key) {
                detached.faces.push((view, cached.slot * 6 + face));
                detached.counts[2] += 1;
            }
            still.insert(layer, key);
            detached.counts[1] += 1;
        }
    }
    clean.layers = still;
}

fn prepare(
    frame: Res<CacheFrame>,
    device: Res<RenderDevice>,
    pipeline: Option<Res<CopyPipeline>>,
    cache: Res<PipelineCache>,
    mut texture: ResMut<CacheTexture>,
) {
    let Some(pipeline) = pipeline else { return };
    if frame.lights.is_empty() || frame.slots == 0 {
        return;
    }
    let shape = (frame.map_size, frame.slots);
    if texture.shape != shape || texture.texture.is_none() {
        let new = device.create_texture(&TextureDescriptor {
            label: Some("shadow_cache_cubes"),
            size: Extent3d { width: shape.0, height: shape.0, depth_or_array_layers: shape.1 * 6 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: CORE_3D_SHADOW_MAP_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let layout = cache.get_bind_group_layout(&pipeline.layout);
        texture.layers = (0..shape.1 * 6)
            .map(|layer| {
                new.create_view(&TextureViewDescriptor {
                    label: Some("shadow_cache_face"),
                    dimension: Some(TextureViewDimension::D2),
                    aspect: TextureAspect::DepthOnly,
                    base_array_layer: layer,
                    array_layer_count: Some(1),
                    ..TextureViewDescriptor::default()
                })
            })
            .collect();
        texture.restore = texture
            .layers
            .iter()
            .map(|view| device.create_bind_group("shadow_cache_restore", &layout, &BindGroupEntries::single(view)))
            .collect();
        texture.texture = Some(new);
        texture.shape = shape;
    }
}

/// The cached light and face a shadow view draws, if any.
fn cached_face(light: &LightEntity, modes: &LightModes) -> Option<(CachedLight, u32)> {
    let LightEntity::Point { light_entity, face_index } = light else { return None };
    let face = u32::try_from(*face_index).ok()?;
    modes.0.get(light_entity).map(|l| (*l, face))
}

/// Before Bevy's shadow pass: our cached static faces into Bevy's layer. It
/// takes the attachment's first use (Bevy's clear), so Bevy then loads and
/// draws the moving casters over the copy.
fn restore_pass(
    view: ViewQuery<(&ShadowView, &LightEntity)>,
    modes: Res<LightModes>,
    texture: Res<CacheTexture>,
    pipeline: Option<Res<CopyPipeline>>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (shadow, light) = view.into_inner();
    let Some((cached, face)) = cached_face(light, &modes) else { return };
    if cached.capture {
        return;
    }
    let Some(pipeline) = pipeline else { return };
    let Some(render_pipeline) = cache.get_render_pipeline(pipeline.id) else { return };
    let Some(group) = texture.restore.get((cached.slot * 6 + face) as usize) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("shadow_cache_restore"),
        color_attachments: &[],
        depth_stencil_attachment: Some(shadow.depth_attachment.get_attachment(StoreOp::Store)),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "shadow_cache_restore");
    pass.set_pipeline(render_pipeline);
    pass.set_bind_group(0, group, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}

/// From the first camera's schedule, before its shadow pass: every detached
/// face's cached depth into Bevy's cube layer (the depth compare is Always,
/// so the attachment's clear-or-load doesn't matter).
fn restore_detached(
    _camera: ViewQuery<&ExtractedCamera>,
    mut detached: ResMut<Detached>,
    faces: Query<&ShadowView>,
    texture: Res<CacheTexture>,
    pipeline: Option<Res<CopyPipeline>>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    if detached.restored || detached.faces.is_empty() {
        return;
    }
    detached.restored = true;
    let Some(pipeline) = pipeline else { return };
    let Some(render_pipeline) = cache.get_render_pipeline(pipeline.id) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    for &(view, layer) in &detached.faces {
        let (Ok(shadow), Some(group)) = (faces.get(view), texture.restore.get(layer as usize)) else { continue };
        let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
            label: Some("shadow_cache_restore_detached"),
            color_attachments: &[],
            depth_stencil_attachment: Some(shadow.depth_attachment.get_attachment(StoreOp::Store)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        let span = diagnostics.pass_span(&mut pass, "shadow_cache_restore_detached");
        pass.set_pipeline(render_pipeline);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
        span.end(&mut pass);
    }
}

/// After Bevy's shadow pass, in a capture view: confirms the face to the main
/// world once every pipeline its bins use is compiled (Bevy skips a draw whose
/// pipeline isn't ready, so that caster would be missing from the cache). Only
/// the capture's own pipelines count: while streaming, some pipeline somewhere
/// is always compiling.
fn confirm_capture(
    view: ViewQuery<(&LightEntity, &ExtractedView)>,
    modes: Res<LightModes>,
    cache: Res<PipelineCache>,
    phases: Res<ViewBinnedRenderPhases<Shadow>>,
    shared: Res<SharedCaptures>,
    mut waits: Local<(u64, u64)>,
) {
    let (light, extracted) = view.into_inner();
    let LightEntity::Point { light_entity, face_index } = light else { return };
    let Some(face) = face_index.checked_sub(CAPTURE_SUBVIEW as usize) else { return };
    let Some(cached) = modes.0.get(light_entity).filter(|c| c.capture) else { return };
    let Some(phase) = phases.get(&extracted.retained_view_entity) else { return };
    let ready = |key: &ShadowBatchSetKey| cache.get_render_pipeline(key.pipeline).is_some();
    waits.0 += 1;
    if !(phase.multidrawable_meshes.keys().all(ready)
        && phase.batchable_meshes.keys().all(|(k, _)| ready(k))
        && phase.unbatchable_meshes.keys().all(|(k, _)| ready(k)))
    {
        waits.1 += 1;
        if waits.1.is_power_of_two() {
            log::debug!("shadow cache: {} of {} capture faces unconfirmed (their pipelines compiling)", waits.1, waits.0);
        }
        return;
    }
    if let Ok(mut faces) = shared.0.lock() {
        let entry = faces.entry(cached.light).or_insert((cached.generation, 0));
        if entry.0 != cached.generation {
            *entry = (cached.generation, 0);
        }
        entry.1 |= 1 << face;
    }
}

/// Subview index of face 0's capture view (Bevy's cube faces use 0..6, and
/// its extraction drops every other point-light subview each frame).
const CAPTURE_SUBVIEW: u32 = 6;

/// A capture view's key in the light's visible-entity tables and phases.
fn capture_key(light: Entity, face: usize) -> RetainedViewEntity {
    RetainedViewEntity::new(MainEntity::from(light), None, CAPTURE_SUBVIEW + u32::try_from(face).unwrap_or(0))
}

/// Before Bevy collects visible entities: each capture's static casters as
/// six more subviews of its light (fresh each capture, every caster "added",
/// so Bevy specialises and bins them all for the new views).
fn inject_captures(
    frame: Res<CacheFrame>,
    ticks: SystemChangeTick,
    mut lights: Query<(&MainEntity, &mut RenderShadowMapVisibleEntities, &mut RenderExtractedShadowMapVisibleEntities), With<ExtractedPointLight>>,
) {
    if frame.captures.is_empty() {
        return;
    }
    for (main, mut visible, mut extracted) in &mut lights {
        let Some(request) = frame.captures.iter().find(|c| c.light == main.id()) else { continue };
        for (face, list) in request.faces.iter().enumerate() {
            let key = capture_key(request.light, face);
            visible.subviews.insert(key, (ticks.this_run(), RenderVisibleEntities::default()));
            let mut classes = RenderExtractedVisibleEntities::default();
            classes.classes.insert(TypeId::of::<Mesh3d>(), RenderExtractedVisibleEntitiesClass { entities: list.clone() });
            extracted.subviews.insert(key, classes);
        }
    }
}

/// After `prepare_lights`: one view per capturing face, a copy of Bevy's face
/// view (frustum, projection, layers) keyed as our subview and drawing into
/// the cache layer. Despawned the next frame (Bevy drops its phase).
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn spawn_capture_views(
    mut commands: Commands,
    frame: Res<CacheFrame>,
    texture: Res<CacheTexture>,
    lights: Query<(Entity, &MainEntity, &RenderShadowMapVisibleEntities), With<ExtractedPointLight>>,
    faces: Query<(&LightEntity, &ExtractedView, &Frustum, Option<&RenderLayers>, Has<NoIndirectDrawing>), With<ShadowView>>,
    mut phases: ResMut<ViewBinnedRenderPhases<Shadow>>,
    support: Res<GpuPreprocessingSupport>,
    mut spawned: Local<Vec<Entity>>,
) {
    for old in spawned.drain(..) {
        commands.entity(old).despawn();
    }
    if frame.captures.is_empty() || texture.shape != (frame.map_size, frame.slots) {
        return;
    }
    for request in &frame.captures {
        let Some((light, _, visible)) = lights.iter().find(|(_, main, _)| main.id() == request.light) else { continue };
        for (light_entity, view, frustum, layers, no_indirect) in &faces {
            let LightEntity::Point { light_entity, face_index } = *light_entity else { continue };
            if light_entity != light || face_index >= 6 {
                continue;
            }
            let key = capture_key(request.light, face_index);
            let Some(layer) = texture.layers.get((request.slot * 6) as usize + face_index) else { continue };
            if !visible.subviews.contains_key(&key) {
                continue;
            }
            let depth_attachment = DepthStencilViewAttachment::new(DepthStencilViews::DepthOnly { depth_view: layer.clone() }, Some(0.0), None);
            let mut entity = commands.spawn((
                ShadowView { depth_attachment, pass_name: "shadow_cache_capture".into() },
                ExtractedView {
                    retained_view_entity: key,
                    viewport: view.viewport,
                    world_from_view: view.world_from_view,
                    clip_from_world: view.clip_from_world,
                    clip_from_view: view.clip_from_view,
                    target_format: view.target_format,
                    color_grading: bevy_render::view::ColorGrading::default(),
                    invert_culling: view.invert_culling,
                },
                *frustum,
                LightEntity::Point { light_entity: light, face_index: CAPTURE_SUBVIEW as usize + face_index },
                layers.cloned().unwrap_or_default(),
                RootNonCameraView(LampShadowView.intern()),
            ));
            if no_indirect {
                entity.insert(NoIndirectDrawing);
            }
            let mode = support.min(if no_indirect { GpuPreprocessingMode::PreprocessingOnly } else { GpuPreprocessingMode::Culling });
            phases.prepare_for_new_frame(key, mode);
            spawned.push(entity.id());
        }
    }
}
