// superfargoth patch (own: trample source): the whole file.
//! The render side of an external trample texture (`crate::trample_source`):
//! the source the generator reads, its uniform and its bind group (group 3 of
//! the `generate` pipeline built with `TRAMPLE_TEXTURE`).

use bevy::ecs::prelude::*;
use bevy::math::{IVec4, Vec4};
use bevy::render::Extract;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::binding_types::{texture_2d, uniform_buffer};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, PipelineCache,
    ShaderStages, ShaderType, TextureSampleType, TextureView, UniformBuffer,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::texture::GpuImage;

use super::resources::GrassLayouts;
use crate::trample_source::{GrassTrampleLookup, GrassTramplePlacement, GrassTrampleTexture};

/// The shader def of the `generate` pipeline that reads an external trample
/// texture.
pub(crate) const TRAMPLE_TEXTURE_DEF: &str = "TRAMPLE_TEXTURE";

/// An external trample texture in the render world: what blade generation
/// reads instead of its own trample cells while this resource exists. An app
/// that writes its press on the GPU inserts it (and updates its views and
/// placements each frame) before `RenderSystems::PrepareBindGroups`; one that
/// has images uses [`GrassTrampleTexture`] instead, which fills this.
#[derive(Resource, Debug, Clone)]
pub struct GrassTrampleSource {
    /// This frame's press.
    pub current: TextureView,
    /// Last frame's press, for the motion vectors.
    pub previous: TextureView,
    /// Where `current` lies.
    pub current_placement: GrassTramplePlacement,
    /// Where `previous` lay.
    pub previous_placement: GrassTramplePlacement,
    /// How both are read.
    pub lookup: GrassTrampleLookup,
}

/// Mirrors `TrampleSource` in `generate.wesl`.
#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub(crate) struct TrampleSourceUniform {
    /// xy: the window's first corner; z: texels per metre (0: not placed).
    pub now: Vec4,
    /// xy: the texel holding the first corner; z: texels per side (0: not
    /// placed); w: the press channel.
    pub now_texel: IVec4,
    /// `now` and `now_texel` for last frame.
    pub then: Vec4,
    pub then_texel: IVec4,
    /// x: strength; y: edge margin; z: edge fade; w: direction floor.
    pub lookup: Vec4,
}

impl TrampleSourceUniform {
    pub(crate) fn of(
        now: &GrassTramplePlacement,
        then: &GrassTramplePlacement,
        lookup: &GrassTrampleLookup,
    ) -> Self {
        let channel = lookup.channel.min(3) as i32;
        let place = |p: &GrassTramplePlacement| {
            if !p.is_placed() {
                return (Vec4::ZERO, IVec4::new(0, 0, 0, channel));
            }
            // Reduced here so the shader wraps with one subtraction.
            let first = (p.first_texel % p.size).as_ivec2();
            (
                Vec4::new(p.corner.x, p.corner.y, p.texels_per_metre, 0.0),
                IVec4::new(first.x, first.y, p.size as i32, channel),
            )
        };
        let (now, now_texel) = place(now);
        let (then, then_texel) = place(then);
        Self {
            now,
            now_texel,
            then,
            then_texel,
            lookup: Vec4::new(
                lookup.strength.max(0.0),
                lookup.edge_margin.max(0.0),
                lookup.edge_fade.max(0.0),
                lookup.direction_floor.max(0.0),
            ),
        }
    }
}

/// Group 3 of the `TRAMPLE_TEXTURE` generate pipeline: this frame's texture,
/// last frame's, and the placements.
pub(crate) fn layout() -> BindGroupLayoutDescriptor {
    // Unfilterable: the texels are loaded, never sampled, so any float format
    // binds.
    let texture = || texture_2d(TextureSampleType::Float { filterable: false });
    BindGroupLayoutDescriptor::new(
        "grass_trample_source",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                texture(),
                texture(),
                uniform_buffer::<TrampleSourceUniform>(false),
            ),
        ),
    )
}

/// The main world's [`GrassTrampleTexture`], and whether the render world's
/// [`GrassTrampleSource`] was made from it (so only that one is removed with
/// it).
#[derive(Resource, Default)]
pub(crate) struct ExtractedTrampleTexture {
    texture: Option<GrassTrampleTexture>,
    made_source: bool,
}

pub(crate) fn extract_trample_texture(
    mut extracted: ResMut<ExtractedTrampleTexture>,
    texture: Extract<Option<Res<GrassTrampleTexture>>>,
) {
    extracted.texture = texture.as_deref().cloned();
}

/// Turns [`GrassTrampleTexture`]'s images into a [`GrassTrampleSource`] once
/// both are on the GPU; removes it with the resource.
pub(crate) fn resolve_trample_texture(
    mut commands: Commands,
    mut extracted: ResMut<ExtractedTrampleTexture>,
    images: Res<RenderAssets<GpuImage>>,
) {
    let source = extracted.texture.as_ref().and_then(|texture| {
        let current = images.get(&texture.current)?;
        let previous = images.get(&texture.previous)?;
        Some(GrassTrampleSource {
            current: current.texture_view.clone(),
            previous: previous.texture_view.clone(),
            current_placement: texture.current_placement,
            previous_placement: texture.previous_placement,
            lookup: texture.lookup,
        })
    });
    match source {
        Some(source) => {
            commands.insert_resource(source);
            extracted.made_source = true;
        }
        None if extracted.made_source => {
            commands.remove_resource::<GrassTrampleSource>();
            extracted.made_source = false;
        }
        None => {}
    }
}

/// The source's bind group, rebuilt every frame (the views and placements of
/// a ring change every frame); `None` while there is no source.
#[derive(Resource, Default)]
pub(crate) struct GrassTrampleSourceGroup {
    uniform: UniformBuffer<TrampleSourceUniform>,
    pub group: Option<BindGroup>,
}

pub(crate) fn prepare_trample_source_group(
    mut group: ResMut<GrassTrampleSourceGroup>,
    source: Option<Res<GrassTrampleSource>>,
    layouts: Res<GrassLayouts>,
    pipeline_cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    let Some(source) = source else {
        group.group = None;
        return;
    };
    group.uniform.set(TrampleSourceUniform::of(
        &source.current_placement,
        &source.previous_placement,
        &source.lookup,
    ));
    group.uniform.write_buffer(&device, &queue);
    let Some(uniform) = group.uniform.binding() else {
        group.group = None;
        return;
    };
    let bind_group = device.create_bind_group(
        "grass_trample_source",
        &pipeline_cache.get_bind_group_layout(&layouts.trample_source),
        &BindGroupEntries::sequential((&source.current, &source.previous, uniform)),
    );
    group.group = Some(bind_group);
}
