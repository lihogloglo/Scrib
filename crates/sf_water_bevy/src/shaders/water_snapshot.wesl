// sf_water_bevy: the water snapshot (water.md "Water pass" step 1): scene colour and depth after
// the sky, clouds and fog, before the water draws, so the water can refract and measure the
// path behind it while writing the live depth buffer.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var scene: texture_2d<f32>;
@if(MULTISAMPLED) @group(0) @binding(1) var scene_depth: texture_depth_multisampled_2d;
@if(!MULTISAMPLED) @group(0) @binding(1) var scene_depth: texture_depth_2d;

struct SnapshotOut {
    @location(0) color: vec4<f32>,
    @location(1) depth: vec4<f32>,
}

@fragment
fn snapshot(in: FullscreenVertexOutput) -> SnapshotOut {
    let p = vec2<i32>(in.position.xy);
    var out: SnapshotOut;
    out.color = textureLoad(scene, p, 0);
    // Sample 0 under MSAA: the water only needs the nearest surface per pixel.
    out.depth = vec4(textureLoad(scene_depth, p, 0), 0.0, 0.0, 1.0);
    return out;
}
