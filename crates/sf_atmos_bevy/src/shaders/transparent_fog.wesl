// `sf_atmos_bevy::FroxelFogMaterial`: Bevy's StandardMaterial fragment
// (forward) fogged by the froxel volume (`sf_render_order_bevy::translucent_fog`), for blended
// surfaces the fog composite can't reach (weather.md "Fog", F3 step 4).

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND, FOG_PREMULTIPLIED, FOG_ADD, FOG_MULTIPLY};
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::pbr_types::{
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY,
};
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};

// (near, far, slices, enabled) of the volume.
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var<uniform> fog_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(111) var fog_volume: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(112) var fog_sampler: sampler;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);

    let uv = (in.position.xy - view.viewport.xy) / view.viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, distance(in.world_position.xyz, view.world_position));
    let bits = pbr.material.flags & STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    var mode = FOG_BLEND;
    if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED {
        mode = FOG_PREMULTIPLIED;
    } else if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD {
        mode = FOG_ADD;
    } else if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY {
        mode = FOG_MULTIPLY;
    }
    out.color = froxel_fog_apply(out.color, fog, mode);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
