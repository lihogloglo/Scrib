// Terrain prepass and shadow shaders: the same CDLOD geometry as the main
// pass (morph driven by the main camera, never the light).
//
// Positions are `@invariant` here and in the main pass: the two vertex
// shaders are different modules, and without invariance the compiler may
// round them differently, so the main pass fails its depth test against
// the prepass (screen-wide stripes at a distance).

import super::common;
import bevy_pbr::render::{
    view_transformations::position_world_to_clip,
    mesh_view_bindings::view,
};
@if(MOTION_VECTOR_PREPASS)
import bevy_pbr::prepass::bindings::previous_view_uniforms;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct PrepassOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) lod: f32,
    @if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    @location(2) unclipped_depth: f32,
    // Where this surface point was last frame (W6b: the raised cover moves under prints).
    @if(MOTION_VECTOR_PREPASS)
    @location(3) previous_world_position: vec4<f32>,
}

@vertex
fn vertex(v: Vertex) -> PrepassOutput {
    var detail = true;
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    detail = false;
    let t = common::terrain_vertex(v.position.xz, v.instance_index, detail);
    var out: PrepassOutput;
    var p = t.position;
    var previous = t.position;
    // Raised cover (surface weather W6b): the main pass's displacement (`common::terrain_raise`),
    // and last frame's (last frame's ring) for the motion vectors. Shadow views never get it.
@if(SURFACE_WEATHER_PREPASS) {
    let r = common::terrain_raise(t);
    p.y += r.x;
    previous.y += r.z;
}
    out.world_position = vec4<f32>(p, 1.0);
    out.position = position_world_to_clip(p);
@if(MOTION_VECTOR_PREPASS)
    out.previous_world_position = vec4<f32>(previous, 1.0);
    out.lod = t.lod;
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION) {
    out.unclipped_depth = out.position.z;
    out.position.z = min(out.position.z, 1.0);
}
    return out;
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
    var out: PrepassFragmentOutput;
@if(NORMAL_PREPASS)
    out.normal = vec4<f32>(common::terrain_normal(in.world_position.xz, in.lod) * 0.5 + vec3<f32>(0.5), 1.0);
@if(MOTION_VECTOR_PREPASS) {
    // The ground is static (only the camera moves); the raised cover over it moves where prints
    // are stamped or refill (W6b): its last-frame height came from last frame's ring.
    let clip = view.unjittered_clip_from_world * in.world_position;
    let previous = previous_view_uniforms.unjittered_clip_from_world * in.previous_world_position;
    out.motion_vector = (clip.xy / clip.w - previous.xy / previous.w) * vec2<f32>(0.5, -0.5);
}
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    out.frag_depth = in.unclipped_depth;
    return out;
}
