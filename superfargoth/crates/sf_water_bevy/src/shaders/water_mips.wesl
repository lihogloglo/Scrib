// sf_water_bevy: the cascade textures' mip chains (normal and foam filtering at distance). A 2x2
// box per level, from level k-1 (a sampled single-level view) into level k (a storage view); the
// box is the right prefilter for slopes (their mean); the material moves the variance it loses
// into roughness.

@group(0) @binding(0) var src: texture_2d_array<f32>;
@group(0) @binding(1) var dst: texture_storage_2d_array<rgba16float, write>;

@compute @workgroup_size(8, 8, 1)
fn downsample(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(dst);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = vec2<i32>(id.xy) * 2;
    let l = i32(id.z);
    let s = textureLoad(src, p, l, 0) + textureLoad(src, p + vec2(1, 0), l, 0)
        + textureLoad(src, p + vec2(0, 1), l, 0) + textureLoad(src, p + vec2(1, 1), l, 0);
    textureStore(dst, vec2<i32>(id.xy), l, s * 0.25);
}
