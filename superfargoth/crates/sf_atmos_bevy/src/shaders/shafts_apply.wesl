// Light shaft bloom (aerial-shadow.md "S3b"; UE "Light Shaft Bloom"): the radially blurred bloom
// added back over the scene times `tint` (tint x scale x off-screen fade x night gate; additive
// blend, alpha kept), after the clouds composite, before the transparents and the post-processing.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::shafts_common::ShaftsBloom;

@group(0) @binding(0) var<uniform> bloom: ShaftsBloom;
@group(0) @binding(1) var shafts_result: texture_2d<f32>;
@group(0) @binding(2) var shafts_sampler: sampler;

@fragment
fn apply(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let rays = textureSampleLevel(shafts_result, shafts_sampler, in.uv, 0.0).rgb;
    return vec4(rays * bloom.tint, 0.0);
}
