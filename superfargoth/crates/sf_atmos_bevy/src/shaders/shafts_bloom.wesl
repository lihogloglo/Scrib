// Light shaft bloom (aerial-shadow.md "S3b"; UE "Light Shaft Bloom"; Mitchell, GPU Gems 3 ch. 13).
//
// `source`, at half the view's internal resolution after the clouds composite: per 2x2 block of
// full-resolution texels, the pre-exposed scene colour of the open-sky texels (the mask's depth
// test) rescaled to `max_luminance` and thresholded (`bloom_color`), averaged, times the falloff
// around the sun's screen position. `shafts_blur` then blurs it radially toward the sun like the
// mask. `shafts_apply` adds the result back over the scene.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_render::view::View;
import super::shafts_common::{ShaftsPass, ShaftsBloom, open_depth, bloom_color, bloom_falloff};

@group(0) @binding(0) var<uniform> shafts: ShaftsPass;
@group(0) @binding(1) var<uniform> view: View;
@if(MULTISAMPLED)
@group(0) @binding(2) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(2) var depth_texture: texture_depth_2d;
@group(0) @binding(3) var<uniform> bloom: ShaftsBloom;
// The resolved scene colour (single-sampled), internal resolution at the texture's origin.
@group(0) @binding(4) var scene_color: texture_2d<f32>;

@fragment
fn source(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let full_max = vec2<i32>(shafts.full_size) - 1;
    let base = vec2<i32>(floor(in.position.xy)) * 2;
    var sum = vec3(0.0);
    for (var i = 0; i < 4; i += 1) {
        let px = clamp(base + vec2(i & 1, i >> 1u), vec2(0), full_max);
        let uv = (vec2<f32>(px) + 0.5) / shafts.full_size;
        let open = open_depth(textureLoad(depth_texture, px, 0), uv, view.view_from_clip, shafts.depth_m);
        if open > 0.0 {
            sum += bloom_color(textureLoad(scene_color, px, 0).rgb, bloom.threshold, bloom.max_luminance);
        }
    }
    let falloff = bloom_falloff(in.uv, shafts.sun_uv, bloom.aspect, bloom.radius);
    return vec4(sum * 0.25 * falloff, 1.0);
}
