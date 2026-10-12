// Depth-aware upsample of the half-resolution cloud history onto the main
// target: `color × T + L` (blend One, SrcAlpha). A tap whose cloud lies
// behind this pixel's geometry is dropped, so mountains and far-field
// proxies keep sharp silhouettes against the clouds.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::functions::{uv_to_ndc, ndc_to_camera_dist};
import super::cloud_common::Clouds;

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

@group(1) @binding(0) var<uniform> clouds: Clouds;
@group(1) @binding(1) var cloud_color: texture_2d<f32>;
@group(1) @binding(2) var cloud_dist: texture_2d<f32>;

@fragment
fn composite(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let depth = textureLoad(depth_texture, vec2<i32>(in.position.xy), 0);
    var scene_t = 1e30;
    if depth > 0.0 {
        scene_t = ndc_to_camera_dist(vec3(uv_to_ndc(in.uv), depth));
    }
    let hp = in.uv * clouds.half_size - 0.5;
    let base = floor(hp);
    let f = hp - base;
    let hmax = vec2<i32>(clouds.half_size) - 1;
    var acc = vec4(0.0);
    for (var i = 0; i < 4; i += 1) {
        let o = vec2(i & 1, i >> 1u);
        let q = clamp(vec2<i32>(base) + o, vec2(0), hmax);
        var c = textureLoad(cloud_color, q, 0);
        let d = textureLoad(cloud_dist, q, 0).r;
        if d > scene_t * 1.02 + 10.0 {
            c = vec4(0.0, 0.0, 0.0, 1.0);
        }
        let w = mix(1.0 - f.x, f.x, f32(o.x)) * mix(1.0 - f.y, f.y, f32(o.y));
        acc += c * w;
    }
    if acc.a > 0.9995 && max(acc.r, max(acc.g, acc.b)) < 1e-6 {
        discard;
    }
    return acc;
}
