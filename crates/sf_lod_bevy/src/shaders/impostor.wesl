// Octahedral impostors, main pass: blend three frames, alpha-test, light
// through Bevy's PbrInput so shadows, clustered lights, fog and atmosphere
// match the meshes they stand in for.

import super::impostor_common;
import super::impostor_common::{params, rotate_y, far_map, far_sampler};
import super::far_shadow_map::far_map_shadow;
import bevy_pbr::render::{
    pbr_types::{PbrInput, pbr_input_new, STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, calculate_view},
    mesh_bindings::mesh,
    mesh_view_bindings::view,
    view_transformations::position_world_to_clip,
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::{mesh_functions, pbr_functions::visibility_range_dither};

struct VertexOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) uv1: vec2<f32>,
    @location(3) uv2: vec2<f32>,
    @location(4) @interpolate(flat) weights: vec3<f32>,
    @location(5) @interpolate(flat) frames: vec3<u32>,
    @location(6) @interpolate(flat) layer_instance: vec2<u32>,
    @location(7) @interpolate(flat) yaw: f32,
    @if(VISIBILITY_RANGE_DITHER)
    @location(8) @interpolate(flat) visibility_range_dither: i32,
}

@vertex
fn vertex(v: impostor_common::Vertex) -> VertexOutput {
    let imp = impostor_common::impostor_vertex(v);
    var out: VertexOutput;
    out.world_position = vec4<f32>(imp.world_position, 1.0);
    out.position = position_world_to_clip(imp.world_position);
    out.uv0 = imp.uv0;
    out.uv1 = imp.uv1;
    out.uv2 = imp.uv2;
    out.weights = imp.weights;
    out.frames = imp.frames;
    out.layer_instance = vec2<u32>(imp.layer, v.instance_index);
    out.yaw = imp.yaw;
@if(VISIBILITY_RANGE_DITHER) {
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(v.instance_index, world_from_local[3]);
}
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    let s = impostor_common::impostor_sample(in.uv0, in.uv1, in.uv2, in.weights, in.frames, in.layer_instance.x);
    if s.alpha < params.tint.a {
        discard;
    }
    let n = rotate_y(s.normal, in.yaw);

    var pbr: PbrInput = pbr_input_new();
    pbr.material.base_color = vec4<f32>(s.albedo * params.tint.rgb, 1.0);
    pbr.material.perceptual_roughness = 0.9;
    pbr.material.metallic = 0.0;
    pbr.material.flags = STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT;
    pbr.frag_coord = in.position;
    pbr.world_position = in.world_position;
    pbr.world_normal = n;
    pbr.N = n;
    pbr.is_orthographic = view.clip_from_view[3].w == 1.0;
    pbr.V = calculate_view(in.world_position, pbr.is_orthographic);
    pbr.flags = mesh[in.layer_instance.y].flags;
    // Terrain shadow past the cascades, dithered cascade end.
    let far = far_map_shadow(in.world_position, n, in.position.xy, far_map, far_sampler, params.far);
    pbr.directional_shadow_factor = far.factor;
    pbr.world_position = far.position;
    let color = apply_pbr_lighting(pbr);
    pbr.world_position = in.world_position;
    return main_pass_post_lighting_processing(pbr, color);
}
