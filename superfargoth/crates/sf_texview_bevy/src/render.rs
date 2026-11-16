//! Render world of the viewer: resolve the shown source, bind it, one fullscreen-triangle pass
//! into the target after the cameras. Nothing runs while nothing is shown.

use std::num::NonZeroU64;

use bevy_app::App;
use bevy_asset::{Assets, Handle, uuid_handle};
use bevy_core_pipeline::schedule::camera_driver;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{texture_2d, uniform_buffer_sized};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BufferInitDescriptor, BufferUsages,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, CommandEncoderDescriptor, FragmentState, LoadOp, Operations,
    PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, ShaderStages,
    SpecializedRenderPipeline, SpecializedRenderPipelines, StoreOp, TextureFormat, TextureSampleType, TextureView,
    VertexState,
};
use bevy_render::renderer::{PendingCommandBuffers, RenderDevice, RenderGraph, RenderGraphSystems};
use bevy_render::texture::GpuImage;
use bevy_render::{Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use sf_diag_bevy::texview::TexSource;

use crate::SampleKind;

const SHADER: Handle<Shader> = uuid_handle!("6c1f2b8e-3d47-4a9b-8e5c-0f7a2d9b4e13");

/// The shown texture as the render world needs it (main world, extracted when it changes).
#[derive(Resource, Clone, Default)]
pub struct TexViewFrame(pub(crate) Option<FrameSource>);

#[derive(Clone)]
pub(crate) struct FrameSource {
    pub source: TexSource,
    pub kind: SampleKind,
    /// The shader's uniform bytes (`crate::params`).
    pub params: [u8; 32],
    pub target: Handle<Image>,
}

impl ExtractResource<RenderApp> for TexViewFrame {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        source.clone()
    }
}

/// This frame's pass, from `prepare` to `blit`.
#[derive(Resource, Default)]
struct Ready(Option<(CachedRenderPipelineId, BindGroup, TextureView)>);

/// The pipeline's layouts, one per sample kind.
#[derive(Resource)]
struct TexViewPipeline {
    float: BindGroupLayoutDescriptor,
    uint: BindGroupLayoutDescriptor,
    sint: BindGroupLayoutDescriptor,
}

impl TexViewPipeline {
    fn layout(&self, kind: SampleKind) -> &BindGroupLayoutDescriptor {
        match kind {
            SampleKind::Float => &self.float,
            SampleKind::Uint => &self.uint,
            SampleKind::Sint => &self.sint,
        }
    }
}

impl FromWorld for TexViewPipeline {
    fn from_world(_: &mut World) -> Self {
        let layout = |label: &'static str, sample: TextureSampleType| {
            BindGroupLayoutDescriptor::new(
                label,
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (uniform_buffer_sized(false, NonZeroU64::new(32)), texture_2d(sample)),
                ),
            )
        };
        Self {
            float: layout("sf_texview_float", TextureSampleType::Float { filterable: false }),
            uint: layout("sf_texview_uint", TextureSampleType::Uint),
            sint: layout("sf_texview_sint", TextureSampleType::Sint),
        }
    }
}

impl SpecializedRenderPipeline for TexViewPipeline {
    type Key = SampleKind;

    fn specialize(&self, kind: SampleKind) -> RenderPipelineDescriptor {
        let shader_defs: Vec<_> = kind.shader_def().into_iter().map(Into::into).collect();
        RenderPipelineDescriptor {
            label: Some("sf_texview".into()),
            layout: vec![self.layout(kind).clone()],
            vertex: VertexState {
                shader: SHADER,
                shader_defs: shader_defs.clone(),
                entry_point: Some("vertex".into()),
                buffers: Vec::new(),
                constants: Vec::new(),
            },
            fragment: Some(FragmentState {
                shader: SHADER,
                shader_defs,
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                constants: Vec::new(),
            }),
            ..RenderPipelineDescriptor::default()
        }
    }
}

pub(crate) fn build(app: &mut App) {
    app.add_plugins(ExtractResourcePlugin::<TexViewFrame>::default());
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        shaders
            .insert(&SHADER, Shader::from_wesl(include_str!("texview.wesl"), "embedded://sf_texview_bevy/texview.wesl"))
            .expect("uuid handles always insert");
    }
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<Ready>()
        .init_resource::<SpecializedRenderPipelines<TexViewPipeline>>()
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<TexViewPipeline>)
        .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
        .add_systems(RenderGraph, blit.in_set(RenderGraphSystems::Render).after(camera_driver));
}

/// Resolves the shown source and the target, specializes the pipeline, binds them. Returns at
/// once while nothing is shown.
fn prepare(world: &mut World) {
    let Some(src) = world.get_resource::<TexViewFrame>().and_then(|f| f.0.clone()) else { return };
    let gpu = |world: &World, id| world.resource::<RenderAssets<GpuImage>>().get(id).map(|g| g.texture_view.clone());
    let view = match &src.source {
        TexSource::Image(handle) => gpu(world, handle.id()),
        TexSource::Render(resolve) => resolve(world),
    };
    let (Some(view), Some(target)) = (view, gpu(world, src.target.id())) else { return };
    let pipeline = world.resource_scope(|world, mut pipelines: Mut<SpecializedRenderPipelines<TexViewPipeline>>| {
        pipelines.specialize(world.resource::<PipelineCache>(), world.resource::<TexViewPipeline>(), src.kind)
    });
    let device = world.resource::<RenderDevice>();
    let params = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("sf_texview_params"),
        contents: &src.params,
        usage: BufferUsages::UNIFORM,
    });
    let layout = world.resource::<PipelineCache>().get_bind_group_layout(world.resource::<TexViewPipeline>().layout(src.kind));
    let group = device.create_bind_group("sf_texview", &layout, &BindGroupEntries::sequential((params.as_entire_binding(), &view)));
    world.resource_mut::<Ready>().0 = Some((pipeline, group, target));
}

/// The pass, once per frame after every camera; its commands go with the frame's own.
#[allow(clippy::needless_pass_by_value)]
fn blit(mut ready: ResMut<Ready>, cache: Res<PipelineCache>, device: Res<RenderDevice>, mut pending: ResMut<PendingCommandBuffers>) {
    let Some((id, group, target)) = ready.0.take() else { return };
    let Some(pipeline) = cache.get_render_pipeline(id) else { return };
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: Some("sf_texview") });
    {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("sf_texview"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &target,
                depth_slice: None,
                resolve_target: None,
                ops: Operations { load: LoadOp::Load, store: StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
    pending.push_encoder(encoder, "sf_texview");
}
