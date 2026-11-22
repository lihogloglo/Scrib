// Light shaft mask (aerial-shadow.md "S3"): at half the view's internal resolution, 1 where the
// depth buffer (opaque + water) shows sky (reversed-Z depth 0) or geometry farther than
// `depth_m`, else 0, averaged over the 2×2 full-resolution texels under each half-res pixel.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_render::view::View;
import super::shafts_common::{ShaftsPass, open_depth};

@group(0) @binding(0) var<uniform> shafts: ShaftsPass;
@group(0) @binding(1) var<uniform> view: View;
@if(MULTISAMPLED)
@group(0) @binding(2) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(2) var depth_texture: texture_depth_2d;

// 1 if the full-resolution texel `px` is open toward the far distance.
fn open_at(px: vec2<i32>) -> f32 {
    let uv = (vec2<f32>(px) + 0.5) / shafts.full_size;
    return open_depth(textureLoad(depth_texture, px, 0), uv, view.view_from_clip, shafts.depth_m);
}

@fragment
fn mask(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let full_max = vec2<i32>(shafts.full_size) - 1;
    let base = vec2<i32>(floor(in.position.xy)) * 2;
    var m = 0.0;
    for (var i = 0; i < 4; i += 1) {
        m += open_at(clamp(base + vec2(i & 1, i >> 1u), vec2(0), full_max));
    }
    return vec4(m * 0.25, 0.0, 0.0, 1.0);
}
