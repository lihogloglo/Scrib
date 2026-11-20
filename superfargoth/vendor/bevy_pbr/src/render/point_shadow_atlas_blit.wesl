// superfargoth patch (own, CW-13b: 2D point shadow atlas)
// Fullscreen triangle depth-clear and Static->Live depth-blit shader for sub-rects
// of the 2D shadow atlases (constrained via viewport + scissor_rect).

struct AtlasBlitUniform {
    src_origin: vec2<i32>,
    dst_origin: vec2<i32>,
    src_size: i32,
    dst_size: i32,
    _pad0: u32,
    _pad1: u32,
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
    let src_local = (local * blit_params.src_size) / max(blit_params.dst_size, 1);
    let src_coord = blit_params.src_origin + clamp(src_local, vec2<i32>(0), vec2<i32>(max(blit_params.src_size - 1, 0)));
    return textureLoad(static_atlas, src_coord, 0);
}
