// Surface weather on the view (`sf_surface_bevy::view`; surface-weather.md "Shader
// integration"): the mesh-view bindings 43-48 that the vendored bevy_pbr patch "SW" adds to
// views with a `bevy_pbr::SurfaceWeatherView` (render/surface_weather.rs; numbers checked by a
// test), and the entry points every lit material calls under `@if(SURFACE_WEATHER)`:
// `apply_surface_weather_standard` from bevy_pbr's `render/pbr.wesl`, `apply_surface_weather`
// from terrain and far proxies. The def is set in the forward main pass only.

import bevy_pbr::render::pbr_types::{
    PbrInput,
    STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MASK,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ALPHA_TO_COVERAGE,
};
import sf_surface_bevy::layer::{
    SurfaceUniform, SurfaceInput, CLASS_STATIC, CLASS_DYNAMIC, apply_surface_layer, surface_porosity,
};

@group(0) @binding(43) var<uniform> surface_uniform: SurfaceUniform;
@group(0) @binding(44) var surface_cells: texture_2d<f32>;
@group(0) @binding(45) var surface_shelter: texture_2d<f32>;
@group(0) @binding(46) var surface_noise: texture_2d<f32>;
@group(0) @binding(47) var surface_sampler: sampler;
@group(0) @binding(48) var surface_shelter_sampler: sampler;

/// The layer on the view's resources.
fn apply_surface_weather(pbr: ptr<function, PbrInput>, s: SurfaceInput) {
    apply_surface_layer(pbr, s, surface_uniform, surface_cells, surface_sampler, surface_shelter, surface_shelter_sampler, surface_noise);
}

/// A layer input for `pbr` of class `surface_class`: porosity from the material (Lagarde's
/// heuristic until a material carries its own, open question 3), no height or basin (W4).
fn surface_input_of(pbr: PbrInput, surface_class: u32) -> SurfaceInput {
    var s: SurfaceInput;
    s.world_pos = pbr.world_position.xyz;
    s.height = 0.5;
    s.basin = 0.0;
    s.porosity = surface_porosity(pbr.material.metallic, pbr.material.perceptual_roughness);
    s.surface_class = surface_class;
    s.instance_wet = 0.0;
    return s;
}

/// StandardMaterial (bevy_pbr `render/pbr.wesl`, patch "SW"): statics, held items and fades as
/// STATIC, skinned meshes as DYNAMIC (dry until W8 feeds `instance_wet`). Unlit and
/// alpha-blended materials (glass, fog translucents) are left alone.
fn apply_surface_weather_standard(pbr: ptr<function, PbrInput>) {
    let flags = (*pbr).material.flags;
    if (flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) != 0u {
        return;
    }
    let alpha = flags & STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    if alpha != STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE
        && alpha != STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MASK
        && alpha != STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ALPHA_TO_COVERAGE {
        return;
    }
    var surface_class = CLASS_STATIC;
@if(SKINNED)
    surface_class = CLASS_DYNAMIC;
    apply_surface_weather(pbr, surface_input_of(*pbr, surface_class));
}
