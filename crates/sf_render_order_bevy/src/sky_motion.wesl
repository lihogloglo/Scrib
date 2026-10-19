// sf_render_order_bevy: camera motion vectors for the pixels nothing wrote (render-order.md
// RO-7). Sky and cloud passes write colour only, so the prepass motion-vector target holds 0 at
// far depth and DLSS/TAA treat the sky as screen-fixed: it smears when the camera turns. Where
// the depth buffer is still at the far plane (reverse-Z: 0), reproject the view ray with last
// frame's unjittered rotation: sky at infinity, so camera translation does not move it.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct SkyMotionView {
    // Unjittered clip_from_world of this frame (inverse) and last frame, view translation zeroed.
    inv_clip_from_dir: mat4x4<f32>,
    prev_clip_from_dir: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> view: SkyMotionView;
@group(0) @binding(1) var scene_depth: texture_depth_2d;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let p = vec2<i32>(in.position.xy);
    if textureLoad(scene_depth, p, 0) > 0.0 {
        discard;
    }
    let size = vec2<f32>(textureDimensions(scene_depth));
    // Pixel centre in NDC, unjittered (the matrices are unjittered too).
    let ndc = (in.position.xy / size) * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
    let dir = view.inv_clip_from_dir * vec4(ndc, 1.0, 1.0);
    let prev = view.prev_clip_from_dir * vec4(dir.xyz / dir.w, 1.0);
    let prev_ndc = prev.xy / prev.w;
    return vec4((ndc - prev_ndc) * vec2(0.5, -0.5), 0.0, 1.0);
}
