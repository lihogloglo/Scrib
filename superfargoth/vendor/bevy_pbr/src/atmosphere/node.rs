use bevy_camera::{MainPassResolutionOverride, Viewport};
use bevy_ecs::system::{Res, ResMut};
use bevy_math::{UVec2, Vec3Swizzles};
use bevy_render::{
    camera::ExtractedCamera,
    diagnostic::RecordDiagnostics,
    extract_component::DynamicUniformIndex,
    render_resource::{ComputePass, ComputePassDescriptor, PipelineCache, RenderPassDescriptor},
    // superfargoth patch (own, LSO: light shaft occlusion, A3 key split)
    render_resource::{BindGroup, ComputePipeline},
    renderer::{RenderContext, ViewQuery},
    view::{ViewTarget, ViewUniformOffset},
};

use crate::{resources::GpuAtmosphere, ViewLightsUniformOffset};

use super::{
    resources::{
        AtmosphereBindGroups, AtmosphereLutPipelines, AtmosphereStaticLutCache,
        AtmosphereTransformsOffset, RenderSkyPipelineId,
        // superfargoth patch (own, LSO: light shaft occlusion)
        RenderSkyLightShaftsPipelineId,
    },
    GpuAtmosphereHeightShadow, GpuAtmosphereSettings,
};

/// superfargoth patch (own, LSO: light shaft occlusion, A3 key split): the aerial view LUT pass
/// that also writes the key LUT (`ATMOSPHERE_KEY_SINGLE`), when the view has its group and the
/// pipeline is compiled: with the height shadow (and its dynamic offset) whenever the view's AS
/// pass would run with one, so the key LUT carries the same shadow as the total. `render_sky` reads
/// the key LUT (its light shafts variant) only when this is `Some`, so it never reads a key LUT
/// this frame did not write.
fn aerial_view_lut_key_single<'a>(
    bind_groups: &'a AtmosphereBindGroups,
    height_shadow_offset: Option<&DynamicUniformIndex<GpuAtmosphereHeightShadow>>,
    pipelines: &AtmosphereLutPipelines,
    pipeline_cache: &'a PipelineCache,
) -> Option<(&'a ComputePipeline, &'a BindGroup, Option<u32>)> {
    match height_shadow_offset.filter(|_| bind_groups.aerial_view_lut_height_shadow.is_some()) {
        Some(offset) => Some((
            pipeline_cache.get_compute_pipeline(pipelines.aerial_view_lut_height_shadow_key_single)?,
            bind_groups.aerial_view_lut_height_shadow_key_single.as_ref()?,
            Some(offset.index()),
        )),
        None => Some((
            pipeline_cache.get_compute_pipeline(pipelines.aerial_view_lut_key_single)?,
            bind_groups.aerial_view_lut_key_single.as_ref()?,
            None,
        )),
    }
}

pub fn atmosphere_luts(
    view: ViewQuery<(
        &GpuAtmosphereSettings,
        &AtmosphereBindGroups,
        &DynamicUniformIndex<GpuAtmosphere>,
        &DynamicUniformIndex<GpuAtmosphereSettings>,
        &AtmosphereTransformsOffset,
        &ViewUniformOffset,
        &ViewLightsUniformOffset,
        // superfargoth patch (own, AS: aerial shadow)
        Option<&DynamicUniformIndex<GpuAtmosphereHeightShadow>>,
    )>,
    pipelines: Res<AtmosphereLutPipelines>,
    pipeline_cache: Res<PipelineCache>,
    mut static_cache: ResMut<AtmosphereStaticLutCache>,
    mut ctx: RenderContext,
) {
    let (
        settings,
        bind_groups,
        atmosphere_uniforms_offset,
        settings_uniforms_offset,
        atmosphere_transforms_offset,
        view_uniforms_offset,
        lights_uniforms_offset,
        height_shadow_offset,
    ) = view.into_inner();

    let (
        Some(transmittance_lut_pipeline),
        Some(multiscattering_lut_pipeline),
        Some(sky_view_lut_pipeline),
        Some(aerial_view_lut_pipeline),
        Some(medium_lut_pipeline),
    ) = (
        pipeline_cache.get_compute_pipeline(pipelines.transmittance_lut),
        pipeline_cache.get_compute_pipeline(pipelines.multiscattering_lut),
        pipeline_cache.get_compute_pipeline(pipelines.sky_view_lut),
        pipeline_cache.get_compute_pipeline(pipelines.aerial_view_lut),
        pipeline_cache.get_compute_pipeline(pipelines.medium_lut),
    )
    else {
        return;
    };

    let need_static_luts =
        static_cache.0.get(&bind_groups.static_lut_textures) != Some(&bind_groups.static_lut_key);

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();

    let command_encoder = ctx.command_encoder();

    let mut luts_pass = command_encoder.begin_compute_pass(&ComputePassDescriptor {
        label: Some("atmosphere_luts"),
        timestamp_writes: None,
    });
    let pass_span = diagnostics.pass_span(&mut luts_pass, "atmosphere_luts");

    fn dispatch_2d(compute_pass: &mut ComputePass, size: UVec2) {
        const WORKGROUP_SIZE: u32 = 16;
        let workgroups_x = size.x.div_ceil(WORKGROUP_SIZE);
        let workgroups_y = size.y.div_ceil(WORKGROUP_SIZE);
        compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);
    }

    if need_static_luts {
        // superfargoth patch (own, P1b): the combined medium LUTs first (unit LUTs x the live
        // coefficients), so every LUT below samples one texel per lookup like stock Bevy. The
        // key already covers the medium and its scales.
        luts_pass.set_pipeline(medium_lut_pipeline);
        luts_pass.set_bind_group(0, &bind_groups.medium_lut, &[atmosphere_uniforms_offset.index()]);
        dispatch_2d(&mut luts_pass, bind_groups.medium_lut_size);

        // Transmittance LUT
        luts_pass.set_pipeline(transmittance_lut_pipeline);
        luts_pass.set_bind_group(
            0,
            &bind_groups.transmittance_lut,
            &[
                atmosphere_uniforms_offset.index(),
                settings_uniforms_offset.index(),
            ],
        );
        dispatch_2d(&mut luts_pass, settings.transmittance_lut_size);

        // Multiscattering LUT
        luts_pass.set_pipeline(multiscattering_lut_pipeline);
        luts_pass.set_bind_group(
            0,
            &bind_groups.multiscattering_lut,
            &[
                atmosphere_uniforms_offset.index(),
                settings_uniforms_offset.index(),
            ],
        );
        luts_pass.dispatch_workgroups(
            settings.multiscattering_lut_size.x,
            settings.multiscattering_lut_size.y,
            1,
        );
        static_cache
            .0
            .insert(bind_groups.static_lut_textures, bind_groups.static_lut_key);
    }

    // Sky View LUT

    luts_pass.set_pipeline(sky_view_lut_pipeline);
    luts_pass.set_bind_group(
        0,
        &bind_groups.sky_view_lut,
        &[
            atmosphere_uniforms_offset.index(),
            settings_uniforms_offset.index(),
            atmosphere_transforms_offset.index(),
            view_uniforms_offset.offset,
            lights_uniforms_offset.offset,
        ],
    );

    dispatch_2d(&mut luts_pass, settings.sky_view_lut_size);

    // Aerial View LUT

    // superfargoth patch (own, AS: aerial shadow): the shadowed variant when the view has a height
    // shadow on the GPU and its pipeline is ready; otherwise the stock pass.
    // superfargoth patch (own, LSO: light shaft occlusion, A3 key split): first the variant that
    // also writes the key LUT, when the view has light shafts.
    let height_shadow = bind_groups
        .aerial_view_lut_height_shadow
        .as_ref()
        .zip(height_shadow_offset)
        .zip(pipeline_cache.get_compute_pipeline(pipelines.aerial_view_lut_height_shadow));
    if let Some((pipeline, bind_group, offset)) = aerial_view_lut_key_single(
        bind_groups,
        height_shadow_offset,
        &pipelines,
        &pipeline_cache,
    ) {
        let offsets = [
            atmosphere_uniforms_offset.index(),
            settings_uniforms_offset.index(),
            view_uniforms_offset.offset,
            lights_uniforms_offset.offset,
            offset.unwrap_or_default(),
        ];
        let count = if offset.is_some() { 5 } else { 4 };
        luts_pass.set_pipeline(pipeline);
        luts_pass.set_bind_group(0, bind_group, &offsets[..count]);
    } else if let Some(((bind_group, offset), pipeline)) = height_shadow {
        luts_pass.set_pipeline(pipeline);
        luts_pass.set_bind_group(
            0,
            bind_group,
            &[
                atmosphere_uniforms_offset.index(),
                settings_uniforms_offset.index(),
                view_uniforms_offset.offset,
                lights_uniforms_offset.offset,
                offset.index(),
            ],
        );
    } else {
        luts_pass.set_pipeline(aerial_view_lut_pipeline);
        luts_pass.set_bind_group(
            0,
            &bind_groups.aerial_view_lut,
            &[
                atmosphere_uniforms_offset.index(),
                settings_uniforms_offset.index(),
                view_uniforms_offset.offset,
                lights_uniforms_offset.offset,
            ],
        );
    }

    dispatch_2d(&mut luts_pass, settings.aerial_view_lut_size.xy());
    pass_span.end(&mut luts_pass);
}

pub fn render_sky(
    // superfargoth patch (own, U3: sky occluded): a view under water draws neither sky nor aerial
    // perspective (`super::SkyOccluded`).
    view: ViewQuery<
        (
            &ExtractedCamera,
            &AtmosphereBindGroups,
            &ViewTarget,
            &DynamicUniformIndex<GpuAtmosphere>,
            &DynamicUniformIndex<GpuAtmosphereSettings>,
            &AtmosphereTransformsOffset,
            &ViewUniformOffset,
            &ViewLightsUniformOffset,
            &RenderSkyPipelineId,
            Option<&MainPassResolutionOverride>,
            // superfargoth patch (own, LSO: light shaft occlusion)
            Option<&RenderSkyLightShaftsPipelineId>,
            // superfargoth patch (own, LSO: light shaft occlusion, A3 key split)
            Option<&DynamicUniformIndex<GpuAtmosphereHeightShadow>>,
        ),
        bevy_ecs::query::Without<super::SkyOccluded>,
    >,
    pipeline_cache: Res<PipelineCache>,
    // superfargoth patch (own, LSO: light shaft occlusion, A3 key split)
    lut_pipelines: Res<AtmosphereLutPipelines>,
    mut ctx: RenderContext,
) {
    let (
        camera,
        atmosphere_bind_groups,
        view_target,
        atmosphere_uniforms_offset,
        settings_uniforms_offset,
        atmosphere_transforms_offset,
        view_uniforms_offset,
        lights_uniforms_offset,
        render_sky_pipeline_id,
        resolution_override,
        light_shafts_pipeline_id,
        height_shadow_offset,
    ) = view.into_inner();

    let Some(render_sky_pipeline) = pipeline_cache.get_render_pipeline(render_sky_pipeline_id.0)
    else {
        return;
    }; //TODO: warning

    // superfargoth patch (own, LSO: light shaft occlusion): the occluded variant when the view
    // has its bind group and the pipeline is compiled (A3 key split: and this frame's aerial pass
    // writes the key LUT); otherwise the stock pass.
    let key_lut_written = aerial_view_lut_key_single(
        atmosphere_bind_groups,
        height_shadow_offset,
        &lut_pipelines,
        &pipeline_cache,
    )
    .is_some();
    let (render_sky_pipeline, render_sky_bind_group) = light_shafts_pipeline_id
        .filter(|_| key_lut_written)
        .and_then(|id| pipeline_cache.get_render_pipeline(id.0))
        .zip(atmosphere_bind_groups.render_sky_light_shafts.as_ref())
        .unwrap_or((render_sky_pipeline, &atmosphere_bind_groups.render_sky));

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();

    let command_encoder = ctx.command_encoder();

    let mut render_sky_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("render_sky"),
        color_attachments: &[Some(view_target.get_color_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let pass_span = diagnostics.pass_span(&mut render_sky_pass, "render_sky");

    if let Some(viewport) =
        Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution_override)
    {
        render_sky_pass.set_viewport(
            viewport.physical_position.x as f32,
            viewport.physical_position.y as f32,
            viewport.physical_size.x as f32,
            viewport.physical_size.y as f32,
            viewport.depth.start,
            viewport.depth.end,
        );
    }

    render_sky_pass.set_pipeline(render_sky_pipeline);
    render_sky_pass.set_bind_group(
        0,
        render_sky_bind_group,
        &[
            atmosphere_uniforms_offset.index(),
            settings_uniforms_offset.index(),
            atmosphere_transforms_offset.index(),
            view_uniforms_offset.offset,
            lights_uniforms_offset.offset,
        ],
    );
    render_sky_pass.draw(0..3, 0..1);
    pass_span.end(&mut render_sky_pass);
}
