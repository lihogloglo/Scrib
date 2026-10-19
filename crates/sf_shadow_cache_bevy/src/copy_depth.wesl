// Shadow cache depth copy (sf_shadow_cache_bevy): one fullscreen triangle that
// writes a source depth layer into the bound depth attachment (compare Always).
// Used both ways: Bevy's freshly drawn cube face -> our persistent cache
// (capture), and our cache -> Bevy's per-frame cube face before its shadow
// pass (restore). Bevy's shadow texture has no COPY usage, so a draw it is.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var source: texture_depth_2d;

@fragment
fn copy_depth(in: FullscreenVertexOutput) -> @builtin(frag_depth) f32 {
    return textureLoad(source, vec2<i32>(floor(in.position.xy)), 0);
}
