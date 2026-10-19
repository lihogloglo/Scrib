// Far-field proxies, main pass: Bevy's StandardMaterial fragment (forward;
// the material forces the forward path) plus the far shadow
// (far_shadow_map.wesl). The prepass and shadow passes use Bevy's own.

import super::far_shadow_map::{far_map_shadow, sun_map_visibility};
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

// (min x, min z, 1 / extent, base height) of the far shadow map.
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> far_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var far_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var far_sampler: sampler;

// The far static sun map (objects' shadows past the cascades): `SunMapParams`.
struct SunMap {
    params: vec4<f32>,
    clip_from_world: mat4x4<f32>,
}
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var<uniform> sun: SunMap;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var sun_map: texture_depth_2d;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var sun_sampler: sampler_comparison;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    let far = far_map_shadow(in.world_position, pbr.N, in.position.xy, far_map, far_sampler, far_params);
    var factor = far.factor;
    if dot(far.to_light, far.to_light) > 0.0 {
        factor = min(factor, sun_map_visibility(in.world_position.xyz, pbr.N, far.to_light, sun_map, sun_sampler, sun.params, sun.clip_from_world));
    }
    pbr.directional_shadow_factor = factor;
    pbr.world_position = far.position;
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    pbr.world_position = in.world_position;
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
