// Surface weather on the view (`sf_surface_bevy::view`; surface-weather.md "Shader
// integration"): the mesh-view bindings 43-53 that the vendored bevy_pbr patch "SW" adds to
// views with a `bevy_pbr::SurfaceWeatherView` (render/surface_weather.rs; numbers checked by a
// test), and the entry points every lit material calls under `@if(SURFACE_WEATHER)`:
// `apply_surface_weather_standard` from bevy_pbr's `render/pbr.wesl`, `apply_surface_weather`
// from terrain and far proxies. The def is set in the forward main pass only; prepass pipelines of
// the same views get `SURFACE_WEATHER_PREPASS` and the same bindings (W6b: `surface_raise_at`).

import bevy_pbr::render::pbr_types::{
    PbrInput,
    STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MASK,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ALPHA_TO_COVERAGE,
};
import sf_surface_bevy::layer::{
    SurfaceUniform, SurfaceInput, CLASS_STATIC, CLASS_DYNAMIC, FLAG_PUDDLE, apply_surface_layer,
    surface_porosity, surface_height_of, surface_raise_amounts, surface_raise_coverage, surface_raise_of,
    surface_ring_at,
};

@group(0) @binding(43) var<uniform> surface_uniform: SurfaceUniform;
@group(0) @binding(44) var surface_cells: texture_2d<f32>;
@group(0) @binding(45) var surface_shelter: texture_2d<f32>;
@group(0) @binding(46) var surface_noise: texture_2d<f32>;
@group(0) @binding(47) var surface_sampler: sampler;
@group(0) @binding(48) var surface_shelter_sampler: sampler;
@group(0) @binding(49) var surface_canopy: texture_2d<f32>;
// The cover layer array (W5a; `cover.rs`: slices 2c albedo (sRGB bytes) + height, 2c + 1 normal
// XY + roughness + AO, c = 0 snow, 1 ash) and its trilinear, anisotropic, repeating sampler.
@group(0) @binding(50) var surface_cover: texture_2d_array<f32>;
@group(0) @binding(51) var surface_cover_sampler: sampler;
// The trail ring (W6a; `trail.rs`: R depression, G lip, m; a torus sampled with `surface_sampler`).
@group(0) @binding(52) var surface_trail: texture_2d<f32>;
// Last frame's ring (W6b: motion vectors of the raised cover; the same ring when no pass ran).
@group(0) @binding(53) var surface_trail_previous: texture_2d<f32>;

/// The raised cover over the ground point `p` (render space, ground normal `n`, surface height
/// `base` 0..1 as the shading's `SurfaceInput.height`, vertex spacing `texel_m`; W6b): x this
/// frame's raise (m), y the same without the trail (the cover's own slope, for normals), z last
/// frame's (last frame's ring and placement; the cells and shelter as now). It stands only where
/// the shading's coverage lies (`surface_raise_coverage`: drift patches, the surface's lows, the
/// look's height). Both terrain passes (main and prepass, `SURFACE_WEATHER` /
/// `SURFACE_WEATHER_PREPASS`) call this one function.
fn surface_raise_at(p: vec3<f32>, n: vec3<f32>, base: f32, texel_m: f32) -> vec3<f32> {
    let u = surface_uniform;
    let a = surface_raise_amounts(u, surface_cells, surface_sampler, surface_shelter, surface_canopy, surface_shelter_sampler, p, n);
    if a.x <= 0.0 && a.y <= 0.0 {
        return vec3(0.0);
    }
    let c = surface_raise_coverage(u, surface_noise, surface_sampler, surface_cover, surface_cover_sampler, a, p.xz, base, texel_m);
    if c.x <= 0.0 && c.y <= 0.0 {
        return vec3(0.0);
    }
    let now = surface_ring_at(surface_trail, surface_sampler, u.trail, u.trail_cells, p.xz);
    let then = surface_ring_at(surface_trail_previous, surface_sampler, u.trail_prev, u.trail_prev_cells, p.xz);
    return vec3(
        surface_raise_of(u, a, c, now, u.trail.w),
        surface_raise_of(u, a, c, vec2(0.0), 0.0),
        surface_raise_of(u, a, c, then, u.trail_prev.w),
    );
}

/// The layer on the view's resources.
fn apply_surface_weather(pbr: ptr<function, PbrInput>, s: SurfaceInput) {
    apply_surface_layer(pbr, s, surface_uniform, surface_cells, surface_sampler, surface_shelter, surface_shelter_sampler, surface_noise, surface_canopy, surface_cover, surface_cover_sampler, surface_trail);
}

/// A layer input for `pbr` of class `surface_class`: porosity from the material (Lagarde's
/// heuristic until a material carries its own, open question 3), height from the albedo (dark
/// grout fills first), no basin (terrain only), the level gate on the shading normal and no
/// footprint (no rain rings; callers that know their screen derivatives set both).
fn surface_input_of(pbr: PbrInput, surface_class: u32) -> SurfaceInput {
    var s: SurfaceInput;
    s.world_pos = pbr.world_position.xyz;
    s.height = surface_height_of(pbr.material.base_color.rgb);
    s.basin = 0.0;
    s.level_up = pbr.world_normal.y;
    s.porosity = surface_porosity(pbr.material.metallic, pbr.material.perceptual_roughness);
    s.surface_class = surface_class;
    s.instance_wet = 0.0;
    s.footprint = 0.0;
    s.dpdx = vec3(0.0);
    s.dpdy = vec3(0.0);
    return s;
}

/// StandardMaterial (bevy_pbr `render/pbr.wesl`, patch "SW"): statics, held items and fades as
/// STATIC, skinned meshes as DYNAMIC (dry until W8 feeds `instance_wet`). Unlit and
/// alpha-blended materials (glass, fog translucents) are left alone. Called in the fragment's
/// uniform control flow: the derivatives come first, before any branch.
fn apply_surface_weather_standard(pbr: ptr<function, PbrInput>) {
    let p = (*pbr).world_position.xyz;
    let dx = dpdx(p);
    let dy = dpdy(p);
    let footprint = max(length(dx), length(dy));
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
    var s = surface_input_of(*pbr, surface_class);
    s.footprint = footprint;
    s.dpdx = dx;
    s.dpdy = dy;
    s.level_up = surface_face_up(dx, dy, (*pbr).world_normal);
    apply_surface_weather(pbr, s);
}

/// Whether puddles are on in the cell window (terrain computes its basin only then).
fn surface_puddles_on() -> bool {
    return (surface_uniform.misc.x & FLAG_PUDDLE) != 0u;
}

/// The puddle gate's up (`SurfaceInput.level_up`) of a mesh: Y of the triangle's face normal from
/// the world position's screen derivatives `dx`/`dy` (exact per triangle: helper lanes extrapolate
/// the same plane), turned to the side of the shading normal `n`. Smoothed vertex normals read a
/// ramp or the edge of a step as level; the face does not. Degenerate derivatives (sub-pixel
/// triangles seen edge-on) keep `n`.
fn surface_face_up(dx: vec3<f32>, dy: vec3<f32>, n: vec3<f32>) -> f32 {
    let f = cross(dy, dx);
    let len = length(f);
    if len <= 1e-12 {
        return n.y;
    }
    return select(-f.y, f.y, dot(f, n) >= 0.0) / len;
}
