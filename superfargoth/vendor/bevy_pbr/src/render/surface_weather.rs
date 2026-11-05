//! superfargoth patch (own, SW): surface weather bindings in the mesh view bind group.
//!
//! A view with a [`SurfaceWeatherView`] component gets [`MeshPipelineViewLayoutKey::SURFACE_WEATHER`]
//! (and the `SURFACE_WEATHER` shader def, main pass only): bindings 43-52 of the mesh view bind
//! group hold the surface weather uniform, the cell window, the shelter depth map, the noise
//! atlas, a filtering sampler, a non-filtering one (the shelter gather) and the canopy depth
//! map (leaf casters only; they shelter partially), then the cover layer array (snow and ash
//! looks) and its sampler, and the trail ring (prints in the cover, W6a). The surface weather
//! system (superfargoth `sf_surface_bevy`) owns the resources, inserts the component on its
//! cameras' render-world view entities, and provides the WESL module `sf_surface_bevy::view`
//! that declares these bindings and that `render/pbr.wesl` calls between
//! `pbr_input_from_standard_material` and `apply_pbr_lighting`. That module is the only
//! coupling; this file knows nothing of the uniform's layout. The bindings travel with the
//! component, so a view's layout key and its bind group can never disagree.
//!
//! Drop when upstream Bevy gets a view-level material hook (global shader bindings plus a
//! fragment extension point for every `StandardMaterial`).
//!
//! [`MeshPipelineViewLayoutKey::SURFACE_WEATHER`]: crate::MeshPipelineViewLayoutKey::SURFACE_WEATHER

use bevy_ecs::component::Component;
use bevy_render::render_resource::{Buffer, Sampler, TextureView};

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

/// What a view reads: a uniform buffer (layout owned by the provider), a filterable 2D cell
/// texture, an unfilterable-float 2D shelter depth texture (`Depth32Float` is fine), a filterable
/// 2D noise texture, a filtering sampler (cells, noise), a non-filtering one (shelter, canopy),
/// an unfilterable-float 2D canopy depth texture, a filterable 2D array (the cover layers) and
/// its filtering sampler, and a filterable 2D trail ring.
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
}

/// On a render-world view entity: the view's lit materials apply surface weather.
#[derive(Component, Clone)]
pub struct SurfaceWeatherView(pub SurfaceWeatherBindings);
