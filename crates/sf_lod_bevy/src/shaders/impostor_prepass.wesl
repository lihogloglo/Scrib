// Octahedral impostors, prepass and shadows: the same quad and frames as
// the main pass (for a shadow view the quad faces the light), alpha-tested
// so depth and shadow silhouettes match the main pass.
//
// Positions are `@invariant` here and in the main pass, so both compute
// identical depth (the main pass tests equal-or-nearer against it).

import super::impostor_common;
import super::impostor_common::{params, rotate_y};
import bevy_pbr::render::{view_transformations::position_world_to_clip, mesh_view_bindings::view};
@if(MOTION_VECTOR_PREPASS)
import bevy_pbr::prepass::bindings::previous_view_uniforms;
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::{mesh_functions, pbr_functions::visibility_range_dither};

struct PrepassOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) uv1: vec2<f32>,
    @location(3) uv2: vec2<f32>,
    @location(4) @interpolate(flat) weights: vec3<f32>,
    @location(5) @interpolate(flat) frames: vec3<u32>,
    @location(6) @interpolate(flat) layer: u32,
    @location(7) @interpolate(flat) yaw: f32,
    @if(VISIBILITY_RANGE_DITHER)
    @location(8) @interpolate(flat) visibility_range_dither: i32,
    @if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    @location(9) unclipped_depth: f32,
}

@vertex
fn vertex(v: impostor_common::Vertex) -> PrepassOutput {
    let imp = impostor_common::impostor_vertex(v);
    var out: PrepassOutput;
    out.world_position = vec4<f32>(imp.world_position, 1.0);
    out.position = position_world_to_clip(imp.world_position);
    out.uv0 = imp.uv0;
    out.uv1 = imp.uv1;
    out.uv2 = imp.uv2;
    out.weights = imp.weights;
    out.frames = imp.frames;
    out.layer = imp.layer;
    out.yaw = imp.yaw;
@if(VISIBILITY_RANGE_DITHER) {
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(v.instance_index, world_from_local[3]);
}
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION) {
    out.unclipped_depth = out.position.z;
    out.position.z = min(out.position.z, 1.0);
}
    return out;
}

fn alpha_test(in: PrepassOutput) -> vec3<f32> {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);
    let s = impostor_common::impostor_sample(in.uv0, in.uv1, in.uv2, in.weights, in.frames, in.layer);
    if s.alpha < params.tint.a {
        discard;
    }
    return rotate_y(s.normal, in.yaw);
}

@if(PREPASS_FRAGMENT || UNCLIPPED_DEPTH_ORTHO_EMULATION)
struct PrepassFragmentOutput {
    @if(NORMAL_PREPASS)
    @location(0) normal: vec4<f32>,
    @if(MOTION_VECTOR_PREPASS)
    @location(1) motion_vector: vec2<f32>,
    @if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    @builtin(frag_depth) frag_depth: f32,
}

@if(PREPASS_FRAGMENT || UNCLIPPED_DEPTH_ORTHO_EMULATION)
@fragment
fn fragment(in: PrepassOutput) -> PrepassFragmentOutput {
    let n = alpha_test(in);
    var out: PrepassFragmentOutput;
@if(NORMAL_PREPASS)
    out.normal = vec4<f32>(n * 0.5 + vec3<f32>(0.5), 1.0);
@if(MOTION_VECTOR_PREPASS) {
    let clip = view.unjittered_clip_from_world * in.world_position;
    let previous = previous_view_uniforms.unjittered_clip_from_world * in.world_position;
    out.motion_vector = (clip.xy / clip.w - previous.xy / previous.w) * vec2<f32>(0.5, -0.5);
}
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    out.frag_depth = in.unclipped_depth;
    return out;
}

@if(!(PREPASS_FRAGMENT || UNCLIPPED_DEPTH_ORTHO_EMULATION))
@fragment
fn fragment(in: PrepassOutput) {
    alpha_test(in);
}
