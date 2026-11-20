// Blended far-field proxies (B5a): Bevy's StandardMaterial fragment (forward,
// AlphaMode::Blend) plus the terrain's far shadow (`far_shadow_map.wesl`) and
// froxel fog (`sf_render_order_bevy::translucent_fog`, render-order.md rule 1).

import super::far_shadow_map::far_map_shadow;
import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND};
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

// (min x, min z, 1 / extent, base height) of the far shadow map.
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> far_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var far_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var far_sampler: sampler;
// `TranslucentFog`: (near, far, slices, enabled), the integrated froxel volume.
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var<uniform> fog_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var fog_volume: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var fog_sampler: sampler;

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

    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, distance(in.world_position.xyz, view.world_position));
    out.color = froxel_fog_apply(out.color, fog, FOG_BLEND);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
