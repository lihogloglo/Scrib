// superfargoth patch (own, CW-13b: 2D point shadow atlas)
// Fullscreen triangle depth-clear and Static->Live depth-blit shader for sub-rects
// of the 2D shadow atlases (constrained via viewport + scissor_rect).

struct AtlasBlitUniform {
    src_origin: vec2<i32>,
    dst_origin: vec2<i32>,
    src_size: i32,
    dst_size: i32,
    guard_ratio: f32,
    _pad: f32,
};

@group(0) @binding(0) var static_atlas: texture_depth_2d;
@group(0) @binding(1) var<uniform> blit_params: AtlasBlitUniform;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(vertex_index & 1u) * 4 - 1);
    let y = f32(i32(vertex_index >> 1u) * 4 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_clear() -> @builtin(frag_depth) f32 {
    // Reverse-Z infinite far plane.
    return 0.0;
}

@fragment
fn fs_blit(@builtin(position) frag_coord: vec4<f32>) -> @builtin(frag_depth) f32 {
    let local = vec2<i32>(frag_coord.xy) - blit_params.dst_origin;
    if (blit_params.src_size == blit_params.dst_size) {
        return textureLoad(static_atlas, blit_params.src_origin + local, 0);
    }
    // Remap through guard-band NDC so blitting between different tile resolutions (e.g. 256 -> 512
    // during a tier resize or live step-down) aligns static depth with mover depth exactly.
    let u_dst = (vec2<f32>(local) + vec2<f32>(0.5)) / max(f32(blit_params.dst_size), 1.0);
    let ndc_src = (u_dst * 2.0 - vec2<f32>(1.0)) * blit_params.guard_ratio;
    let u_src = ndc_src * 0.5 + vec2<f32>(0.5);
    let max_src = max(blit_params.src_size - 1, 0);
    let src_local = clamp(
        vec2<i32>(floor(u_src * f32(blit_params.src_size))),
        vec2<i32>(0),
        vec2<i32>(max_src),
    );
    return textureLoad(static_atlas, blit_params.src_origin + src_local, 0);
}
