//! superfargoth patch (own, SW): surface weather bindings in the mesh view bind group and, for
//! vertex displacement (W6b), in the prepass view bind group.
//!
//! A view with a [`SurfaceWeatherView`] component gets [`MeshPipelineViewLayoutKey::SURFACE_WEATHER`]
//! (and the `SURFACE_WEATHER` shader def, main pass only): bindings 43-53 of the mesh view bind
//! group hold the surface weather uniform, the cell window, the shelter depth map, the noise
//! atlas, a filtering sampler, a non-filtering one (the shelter gather) and the canopy depth
//! map (leaf casters only; they shelter partially), then the cover layer array (snow and ash
//! looks) and its sampler, the trail ring (prints in the cover, W6a) and last frame's ring (W6b's
//! motion vectors). The surface weather system (superfargoth `sf_surface_bevy`) owns the
//! resources, inserts the component on its cameras' render-world view entities, and provides the
//! WESL module `sf_surface_bevy::view` that declares these bindings and that `render/pbr.wesl`
//! calls between `pbr_input_from_standard_material` and `apply_pbr_lighting`. That module is the
//! only coupling; this file knows nothing of the uniform's layout. The bindings travel with the
//! component, so a view's layout key and its bind group can never disagree.
//!
//! Prepass (W6b): the same view also gets `MeshPipelineKey::SURFACE_WEATHER` in its prepass key;
//! prepass pipelines then use the prepass view layout plus these bindings (same numbers, group 0)
//! and the `SURFACE_WEATHER_PREPASS` def, and the view carries a
//! [`SurfaceWeatherPrepassBindGroups`] made beside the shared prepass view bind groups. Displaced
//! geometry (raised snow) must displace identically in the prepass (depth, motion vectors) and the
//! main pass. All bindings are visible to the vertex and fragment stages. Shadow views never
//! carry the component: they draw undisplaced.
//!
//! Drop when upstream Bevy gets a view-level material hook (global shader bindings plus a
//! fragment extension point for every `StandardMaterial`, available to prepass pipelines too).
//!
//! [`MeshPipelineViewLayoutKey::SURFACE_WEATHER`]: crate::MeshPipelineViewLayoutKey::SURFACE_WEATHER

use bevy_ecs::component::Component;
use bevy_render::render_resource::{
    binding_types::{sampler, texture_2d, texture_2d_array, uniform_buffer_sized},
    BindGroup, Buffer, DynamicBindGroupEntries, DynamicBindGroupLayoutEntries, Sampler,
    SamplerBindingType, ShaderStages, TextureSampleType, TextureView,
};

/// Binding numbers in the mesh view bind group (group 0), after the GI patch's (39-42).
pub const SURFACE_WEATHER_UNIFORM_BINDING: u32 = 43;
pub const SURFACE_WEATHER_CELLS_BINDING: u32 = 44;
pub const SURFACE_WEATHER_SHELTER_BINDING: u32 = 45;
pub const SURFACE_WEATHER_NOISE_BINDING: u32 = 46;
pub const SURFACE_WEATHER_SAMPLER_BINDING: u32 = 47;
pub const SURFACE_WEATHER_SHELTER_SAMPLER_BINDING: u32 = 48;
pub const SURFACE_WEATHER_CANOPY_BINDING: u32 = 49;
pub const SURFACE_WEATHER_COVER_BINDING: u32 = 50;
pub const SURFACE_WEATHER_COVER_SAMPLER_BINDING: u32 = 51;
/// The trail ring (W6a): a filterable 2D texture, sampled with the filtering sampler (47).
pub const SURFACE_WEATHER_TRAIL_BINDING: u32 = 52;
/// Last frame's trail ring (W6b): same format, for the displaced geometry's motion vectors.
pub const SURFACE_WEATHER_TRAIL_PREVIOUS_BINDING: u32 = 53;

/// What a view reads: a uniform buffer (layout owned by the provider), a filterable 2D cell
/// texture, an unfilterable-float 2D shelter depth texture (`Depth32Float` is fine), a filterable
/// 2D noise texture, a filtering sampler (cells, noise), a non-filtering one (shelter, canopy),
/// an unfilterable-float 2D canopy depth texture, a filterable 2D array (the cover layers) and
/// its filtering sampler, a filterable 2D trail ring and last frame's ring.
#[derive(Clone)]
pub struct SurfaceWeatherBindings {
    pub uniform: Buffer,
    pub cells: TextureView,
    pub shelter: TextureView,
    pub noise: TextureView,
    pub sampler: Sampler,
    pub shelter_sampler: Sampler,
    pub canopy: TextureView,
    pub cover: TextureView,
    pub cover_sampler: Sampler,
    pub trail: TextureView,
    pub trail_previous: TextureView,
}

/// On a render-world view entity: the view's lit materials apply surface weather.
#[derive(Component, Clone)]
pub struct SurfaceWeatherView(pub SurfaceWeatherBindings);

/// On a render-world view entity with a [`SurfaceWeatherView`]: the prepass view bind groups
/// with the surface weather bindings (made by `prepare_prepass_view_bind_group`).
#[derive(Component, Clone)]
pub struct SurfaceWeatherPrepassBindGroups {
    pub motion_vectors: Option<BindGroup>,
    pub no_motion_vectors: BindGroup,
}

/// Appends the layout entries (bindings 43-53, vertex and fragment).
pub(crate) fn surface_weather_layout_entries(
    entries: DynamicBindGroupLayoutEntries,
) -> DynamicBindGroupLayoutEntries {
    let vf = ShaderStages::VERTEX_FRAGMENT;
    entries.extend_with_indices((
        (SURFACE_WEATHER_UNIFORM_BINDING, uniform_buffer_sized(false, None).visibility(vf)),
        (SURFACE_WEATHER_CELLS_BINDING, texture_2d(TextureSampleType::Float { filterable: true }).visibility(vf)),
        (SURFACE_WEATHER_SHELTER_BINDING, texture_2d(TextureSampleType::Float { filterable: false }).visibility(vf)),
        (SURFACE_WEATHER_NOISE_BINDING, texture_2d(TextureSampleType::Float { filterable: true }).visibility(vf)),
        (SURFACE_WEATHER_SAMPLER_BINDING, sampler(SamplerBindingType::Filtering).visibility(vf)),
        (SURFACE_WEATHER_SHELTER_SAMPLER_BINDING, sampler(SamplerBindingType::NonFiltering).visibility(vf)),
        (SURFACE_WEATHER_CANOPY_BINDING, texture_2d(TextureSampleType::Float { filterable: false }).visibility(vf)),
        (SURFACE_WEATHER_COVER_BINDING, texture_2d_array(TextureSampleType::Float { filterable: true }).visibility(vf)),
        (SURFACE_WEATHER_COVER_SAMPLER_BINDING, sampler(SamplerBindingType::Filtering).visibility(vf)),
        (SURFACE_WEATHER_TRAIL_BINDING, texture_2d(TextureSampleType::Float { filterable: true }).visibility(vf)),
        (SURFACE_WEATHER_TRAIL_PREVIOUS_BINDING, texture_2d(TextureSampleType::Float { filterable: true }).visibility(vf)),
    ))
}

/// Appends the bind group entries matching [`surface_weather_layout_entries`].
pub(crate) fn surface_weather_bind_entries<'a>(
    entries: DynamicBindGroupEntries<'a>,
    sw: &'a SurfaceWeatherBindings,
) -> DynamicBindGroupEntries<'a> {
    entries.extend_with_indices((
        (SURFACE_WEATHER_UNIFORM_BINDING, sw.uniform.as_entire_binding()),
        (SURFACE_WEATHER_CELLS_BINDING, &sw.cells),
        (SURFACE_WEATHER_SHELTER_BINDING, &sw.shelter),
        (SURFACE_WEATHER_NOISE_BINDING, &sw.noise),
        (SURFACE_WEATHER_SAMPLER_BINDING, &sw.sampler),
        (SURFACE_WEATHER_SHELTER_SAMPLER_BINDING, &sw.shelter_sampler),
        (SURFACE_WEATHER_CANOPY_BINDING, &sw.canopy),
        (SURFACE_WEATHER_COVER_BINDING, &sw.cover),
        (SURFACE_WEATHER_COVER_SAMPLER_BINDING, &sw.cover_sampler),
        (SURFACE_WEATHER_TRAIL_BINDING, &sw.trail),
        (SURFACE_WEATHER_TRAIL_PREVIOUS_BINDING, &sw.trail_previous),
    ))
}
