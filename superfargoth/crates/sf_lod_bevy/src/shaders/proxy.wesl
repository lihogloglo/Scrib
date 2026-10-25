// Far-field proxies, main pass: Bevy's StandardMaterial fragment (forward;
// the material forces the forward path) plus the far shadow
// (far_shadow_map.wesl). The prepass and shadow passes use Bevy's own.

import super::far_shadow_map::far_map_shadow;
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

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    let far = far_map_shadow(in.world_position, pbr.N, in.position.xy, far_map, far_sampler, far_params);
    pbr.directional_shadow_factor = far.factor;
    pbr.world_position = far.position;
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    pbr.world_position = in.world_position;
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
