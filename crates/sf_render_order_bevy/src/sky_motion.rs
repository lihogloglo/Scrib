//! Camera motion vectors for sky and cloud pixels (render-order.md RO-7).
//!
//! Bevy's prepass clears the motion-vector target to 0 and the sky and cloud
//! passes write colour only, so DLSS/TAA see the sky as screen-fixed and smear
//! it while the camera turns. After the transparent pass (like water's velocity
//! redraw), a fullscreen pass writes the camera's rotational motion into every
//! pixel whose depth is still at the far plane. Scaffold: Bevy's `Atmosphere`
//! `render_sky` writes no motion vectors; removal path: upstream does.
//! [`SkyMotion`] is the A/B switch (default on); single-sample views only
//! (DLSS and TAA both need MSAA off).

use std::collections::HashMap;

use bevy_app::{App, Plugin};
use bevy_asset::{AssetServer, Handle, load_embedded_asset};
use bevy_core_pipeline::FullscreenShader;
use bevy_core_pipeline::core_3d::main_transparent_pass_3d;
use bevy_core_pipeline::prepass::ViewPrepassTextures;
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_ecs::prelude::*;
use bevy_math::{Mat4, Vec4};
use bevy_render::render_resource::binding_types::{texture_depth_2d, uniform_buffer};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
    PipelineCache, RenderPassDescriptor, RenderPipelineDescriptor, ShaderStages, ShaderType, TextureFormat, UniformBuffer,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;

/// A/B switch for the sky motion-vector pass (on by default).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkyMotion(pub bool);

impl Default for SkyMotion {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Clone, Copy, Default, ShaderType)]
struct SkyMotionUniform {
    inv_clip_from_dir: Mat4,
    prev_clip_from_dir: Mat4,
}

#[derive(Resource)]
struct SkyMotionPipeline {
    layout: BindGroupLayoutDescriptor,
    shader: Handle<Shader>,
    fullscreen: FullscreenShader,
    id: Option<CachedRenderPipelineId>,
}

#[derive(Resource, Default)]
struct SkyMotionPrev(HashMap<Entity, Mat4>);

#[derive(Component)]
struct SkyMotionBindings(BindGroup);

/// Adds the pass (added by `RenderOrderPlugin`).
pub struct SkyMotionPlugin;

impl Plugin for SkyMotionPlugin {
    fn build(&self, app: &mut App) {
        bevy_asset::embedded_asset!(app, "sky_motion.wesl");
        app.init_resource::<SkyMotion>();
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .init_resource::<SkyMotion>()
            .init_resource::<SkyMotionPrev>()
            .add_systems(ExtractSchedule, extract_switch)
            .add_systems(RenderStartup, init_pipeline)
            .add_systems(Render, prepare.in_set(RenderSystems::PrepareBindGroups))
            .add_systems(Core3d, sky_motion_pass.after(main_transparent_pass_3d).before(Core3dSystems::EarlyPostProcess));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn init_pipeline(mut commands: Commands, assets: Res<AssetServer>, fullscreen: Res<FullscreenShader>) {
    let layout = BindGroupLayoutDescriptor::new(
        "sky_motion",
        &BindGroupLayoutEntries::sequential(ShaderStages::FRAGMENT, (uniform_buffer::<SkyMotionUniform>(false), texture_depth_2d())),
    );
    let shader = load_embedded_asset!(assets.as_ref(), "sky_motion.wesl");
    commands.insert_resource(SkyMotionPipeline { layout, shader, fullscreen: fullscreen.clone(), id: None });
}

/// Rotation-only `clip_from_world` (translation zeroed): the sky is at infinity.
fn rotation_clip(view: &ExtractedView) -> Mat4 {
    let mut view_from_world = view.world_from_view.to_matrix().inverse();
    view_from_world.w_axis = Vec4::W;
    view.clip_from_view * view_from_world
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
fn prepare(
    mut commands: Commands,
    mut pipeline: ResMut<SkyMotionPipeline>,
    mut prev: ResMut<SkyMotionPrev>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    switch: Res<SkyMotion>,
    views: Query<(Entity, &ExtractedView, &ViewDepthStencilTexture, &Msaa, Option<&ViewPrepassTextures>)>,
) {
    if !switch.0 {
        prev.0.clear();
        return;
    }
    if pipeline.id.is_none() {
        let descriptor = RenderPipelineDescriptor {
            label: Some("sky_motion".into()),
            layout: vec![pipeline.layout.clone()],
            vertex: pipeline.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: pipeline.shader.clone(),
                targets: vec![Some(ColorTargetState { format: TextureFormat::Rg16Float, blend: None, write_mask: ColorWrites::ALL })],
                ..FragmentState::default()
            }),
            ..RenderPipelineDescriptor::default()
        };
        pipeline.id = Some(cache.queue_render_pipeline(descriptor));
    }
    for (entity, view, depth, msaa, prepass) in &views {
        let has_mv = prepass.is_some_and(|p| p.motion_vectors.is_some());
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else { continue };
        if msaa.samples() != 1 || !has_mv {
            continue;
        }
        let now = rotation_clip(view);
        let before = prev.0.insert(entity, now).unwrap_or(now);
        let mut uniform = UniformBuffer::from(SkyMotionUniform { inv_clip_from_dir: now.inverse(), prev_clip_from_dir: before });
        uniform.write_buffer(&device, &queue);
        let Some(binding) = uniform.binding() else { continue };
        let group = device.create_bind_group("sky_motion", &cache.get_bind_group_layout(&pipeline.layout), &BindGroupEntries::sequential((binding, depth_view)));
        commands.entity(entity).insert(SkyMotionBindings(group));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sky_motion_pass(
    view: ViewQuery<(&SkyMotionBindings, &ViewPrepassTextures)>,
    pipeline: Res<SkyMotionPipeline>,
    switch: Res<SkyMotion>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    if !switch.0 {
        return;
    }
    let (bindings, prepass) = view.into_inner();
    let Some(motion) = prepass.motion_vectors.as_ref() else { return };
    let Some(render_pipeline) = pipeline.id.and_then(|id| cache.get_render_pipeline(id)) else { return };
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("sky_motion"),
        color_attachments: &[Some(motion.get_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(render_pipeline);
    pass.set_bind_group(0, &bindings.0, &[]);
    pass.draw(0..3, 0..1);
}

#[allow(clippy::needless_pass_by_value)]
fn extract_switch(main: Extract<Res<SkyMotion>>, mut switch: ResMut<SkyMotion>) {
    *switch = **main;
}
