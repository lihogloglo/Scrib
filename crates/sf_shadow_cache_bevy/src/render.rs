//! Render world: the persistent cube array and the depth-copy passes around
//! Bevy's point-light shadow pass (restore before, capture after). Cached
//! faces with no mover lose their Bevy view right after `prepare_lights`
//! ([`detach_views`]) and are restored once per frame from the camera's
//! schedule ([`restore_detached`]).

use std::collections::HashMap;

use bevy_app::App;
use bevy_asset::{load_embedded_asset, Handle};
use bevy_core_pipeline::schedule::{Core3d, RootNonCameraView};
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_pbr::{prepare_lights, shared_shadow_pass, ExtractedPointLight, LightEntity, ShadowView, CORE_3D_SHADOW_MAP_FORMAT, EARLY_SHADOW_PASS, LATE_SHADOW_PASS};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_resource::binding_types::texture_depth_2d;
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, CachedRenderPipelineId, CompareFunction, DepthBiasState,
    DepthStencilState, Extent3d, FragmentState, LoadOp, Operations, PipelineCache, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipelineDescriptor, ShaderStages, StencilState, StoreOp, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension,
};
use bevy_render::renderer::{RenderContext, RenderDevice, ViewQuery};
use bevy_render::sync_world::MainEntity;
use bevy_render::texture::DepthStencilViews;
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

/// The frame's cached lights and the cache texture's shape.
#[derive(Resource, Clone, Default)]
pub(crate) struct CacheFrame {
    pub(crate) map_size: u32,
    pub(crate) slots: u32,
    pub(crate) lights: Vec<CachedLight>,
}

pub(crate) fn build(app: &mut App, shared: SharedCaptures) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<CacheFrame>()
        .init_resource::<CacheTexture>()
        .init_resource::<LightModes>()
        .init_resource::<Detached>()
        .insert_resource(shared)
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<CopyPipeline>)
        .add_systems(
            Render,
            (detach_views.in_set(RenderSystems::CreateViews).after(prepare_lights), prepare.in_set(RenderSystems::PrepareBindGroups)),
        );
}

pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app.add_systems(
        Core3d,
        (
            restore_pass.before(shared_shadow_pass::<EARLY_SHADOW_PASS>),
            restore_detached.before(shared_shadow_pass::<EARLY_SHADOW_PASS>),
            capture_pass.after(shared_shadow_pass::<LATE_SHADOW_PASS>),
        ),
    );
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
) {
    modes.0.clear();
    detached.faces.clear();
    detached.restored = false;
    if frame.lights.is_empty() || frame.slots == 0 {
        return;
    }
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
            detached.faces.push((view, cached.slot * 6 + face));
        }
    }
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

/// After Bevy's shadow pass: a capturing light's freshly drawn face (static
/// casters only) into our cache. Confirmed to the main world only while no
/// pipeline is still compiling (a caster whose pipeline isn't ready is
/// skipped by Bevy and would be missing from the cache).
#[allow(clippy::too_many_arguments)]
fn capture_pass(
    view: ViewQuery<(&ShadowView, &LightEntity)>,
    modes: Res<LightModes>,
    texture: Res<CacheTexture>,
    pipeline: Option<Res<CopyPipeline>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    shared: Res<SharedCaptures>,
    mut ctx: RenderContext,
) {
    let (shadow, light) = view.into_inner();
    let Some((cached, face)) = cached_face(light, &modes) else { return };
    if !cached.capture {
        return;
    }
    let Some(pipeline) = pipeline else { return };
    let Some(render_pipeline) = cache.get_render_pipeline(pipeline.id) else { return };
    let Some(target) = texture.layers.get((cached.slot * 6 + face) as usize) else { return };
    let DepthStencilViews::DepthOnly { depth_view } = &shadow.depth_attachment.depth_stencil_views else { return };
    let group = device.create_bind_group("shadow_cache_capture", &cache.get_bind_group_layout(&pipeline.layout), &BindGroupEntries::single(depth_view));
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("shadow_cache_capture"),
        color_attachments: &[],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: target,
            depth_ops: Some(Operations { load: LoadOp::Clear(0.0), store: StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "shadow_cache_capture");
    pass.set_pipeline(render_pipeline);
    pass.set_bind_group(0, &group, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
    drop(pass);
    if cache.waiting_pipelines().next().is_some() {
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
