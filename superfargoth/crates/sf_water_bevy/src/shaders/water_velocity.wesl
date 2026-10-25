// sf_water_bevy: the water's motion vectors (water.md "Water pass" step 4). The transparent
// phase writes no motion vectors, so after it the clipmap is drawn again (procedurally, the same
// vertex placement as the water draw) into the prepass motion-vector texture, where the water is
// the nearest surface: each Lagrangian point displaced now vs last frame, through this and last
// frame's unjittered view. Without it DLSS/TAA move the waves with the bed's motion (smear).

import super::water_common::{WaterParams, clipmap_xz, covered_by_finer, displaced, shore_at, shore_scales};

struct VelocityView {
    clip_from_world: mat4x4<f32>,
    prev_clip_from_world: mat4x4<f32>,
    camera: vec4<f32>,
}

@group(0) @binding(0) var<uniform> water: WaterParams;
@group(0) @binding(1) var<uniform> vel: VelocityView;
@group(0) @binding(2) var displacement: texture_2d_array<f32>;
@group(0) @binding(3) var prev_displacement: texture_2d_array<f32>;
@group(0) @binding(4) var wave_sampler: sampler;
@group(0) @binding(6) var shore_field: texture_2d<f32>;
@group(0) @binding(7) var shore_sampler: sampler;
@if(MULTISAMPLED) @group(0) @binding(5) var scene_depth: texture_depth_multisampled_2d;
@if(!MULTISAMPLED) @group(0) @binding(5) var scene_depth: texture_depth_2d;

struct VelocityVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) clip_now: vec4<f32>,
    @location(1) clip_prev: vec4<f32>,
    @location(2) lagrangian: vec2<f32>,
    @location(3) @interpolate(flat) ring: u32,
}

@vertex
fn vertex(@builtin(vertex_index) vi: u32, @builtin(instance_index) ring: u32) -> VelocityVarying {
    let quads = u32(water.clip.y);
    let half = i32(quads / 2u);
    let hole = i32(quads / 4u) - 2;
    let q = vi / 6u;
    let corner = array<vec2<i32>, 6>(vec2(0, 0), vec2(0, 1), vec2(1, 0), vec2(1, 0), vec2(0, 1), vec2(1, 1))[vi % 6u];
    let cell = vec2<i32>(i32(q % quads), i32(q / quads)) - vec2(half);
    var out: VelocityVarying;
    if ring > 0u && cell.x >= -hole && cell.x < hole && cell.y >= -hole && cell.y < hole {
        out.position = vec4(0.0, 0.0, 0.0, 1.0);
        return out;
    }
    let g = vec2<f32>(cell + corner);
    let camera = vel.camera.xz;
    let x0 = clipmap_xz(water, g, f32(ring), camera);
    let scales = shore_scales(water, shore_at(water, shore_field, shore_sampler, x0));
    let now = displaced(water, displacement, wave_sampler, x0, camera, scales);
    let prev = displaced(water, prev_displacement, wave_sampler, x0, camera, scales);
    out.clip_now = vel.clip_from_world * vec4(now.xyz, 1.0);
    out.clip_prev = vel.prev_clip_from_world * vec4(prev.xyz, 1.0);
    out.position = out.clip_now;
    out.lagrangian = x0;
    out.ring = ring;
    return out;
}

@fragment
fn fragment(in: VelocityVarying) -> @location(0) vec4<f32> {
    if covered_by_finer(water, in.ring, in.lagrangian, vel.camera.xz) {
        discard;
    }
    // Only where the water is the nearest surface (it wrote this depth in the transparent pass;
    // a relative tolerance absorbs the jitter between the two draws).
    let scene = textureLoad(scene_depth, vec2<i32>(in.position.xy), 0);
    if scene > in.position.z * 1.02 + 1e-7 {
        discard;
    }
    let now = in.clip_now.xy / in.clip_now.w;
    let prev = in.clip_prev.xy / in.clip_prev.w;
    return vec4((now - prev) * vec2(0.5, -0.5), 0.0, 1.0);
}
