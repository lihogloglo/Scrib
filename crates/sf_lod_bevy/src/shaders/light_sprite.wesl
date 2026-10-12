// Distant light sprites: camera-facing additive glows with a minimum size
// in pixels, so a town's lights stay visible as points across the map.
// Faded by a night factor (0 by day) set from the sky.
//
// Mesh: four vertices per light. @location(0) light position (node space),
// @location(1) corner in [-1, 1]², @location(2) (linear rgb, intensity),
// @location(3) (radius m, 0, 0, 0).

import bevy_pbr::render::{
    mesh_functions,
    mesh_view_bindings::view,
    view_transformations::position_world_to_clip,
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

struct SpriteParams {
    // x: night factor, y: minimum size (px), z: world size per radius meter,
    // w: brightness scale
    p: vec4<f32>,
    // rgb: debug tint, a: unused
    tint: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> sprite: SpriteParams;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) center: vec3<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) data: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) corner: vec2<f32>,
    @location(1) @interpolate(flat) color: vec4<f32>,
    @if(VISIBILITY_RANGE_DITHER)
    @location(2) @interpolate(flat) visibility_range_dither: i32,
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    let center = (world_from_local * vec4<f32>(v.center, 1.0)).xyz;
    let to_cam = view.world_position - center;
    let dist = length(to_cam);
    let f = to_cam / max(dist, 1e-3);
    let up_ref = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, -1.0), abs(f.y) > 0.999);
    let right = normalize(cross(up_ref, f));
    let up = cross(f, right);
    // World size of one pixel at this distance.
    let pixel = dist * 2.0 / (view.clip_from_view[1][1] * view.viewport.w);
    let size = max(v.data.x * sprite.p.z, sprite.p.y * pixel);
    // Sub-pixel-limited sprites keep their energy: dim as they shrink below
    // their natural size, so the minimum size doesn't brighten far lights.
    let energy = clamp(v.data.x * sprite.p.z / size, 0.0, 1.0);
    var out: VertexOutput;
    // Pulled slightly toward the camera so the glow isn't buried in its lamp.
    let p = center + f * min(v.data.x, dist * 0.5) + (right * v.corner.x + up * v.corner.y) * size;
    out.position = position_world_to_clip(p);
    // Off (by day): a degenerate position, so nothing rasterizes.
    if sprite.p.x <= 0.0 {
        out.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    }
    out.corner = v.corner;
    out.color = vec4<f32>(v.color.rgb * sprite.tint.rgb, v.color.a * sprite.p.x * sprite.p.w * mix(0.35, 1.0, energy));
@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(v.instance_index, world_from_local[3]);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);
    let r2 = dot(in.corner, in.corner);
    if r2 >= 1.0 {
        discard;
    }
    // A bright core and a soft halo.
    let glow = exp(-r2 * 12.0) * 2.0 + exp(-r2 * 3.0) * 0.4;
    // Bevy's additive mode is premultiplied blending: alpha 0 adds.
    return vec4<f32>(in.color.rgb * in.color.a * glow, 0.0);
}
