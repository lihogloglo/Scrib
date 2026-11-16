// `tex show` (sf_texview_bevy): draws the shown texture into the viewer's target. Nearest texels
// (`textureLoad`, so depth, unfilterable float and integer formats all read the same way), one
// channel as grey or RGB, remapped from [min, max] to [0, 1]; NaN and infinities in magenta.
// Defs: SAMPLE_UINT, SAMPLE_SINT (integer formats); none for float and depth.

struct Params {
    // x: min, y: 1 / (max - min).
    range: vec4<f32>,
    // x: channel (0 r, 1 g, 2 b, 3 a, 4 rgb).
    mode: vec4<u32>,
}

@group(0) @binding(0) var<uniform> params: Params;
@if(SAMPLE_UINT)
@group(0) @binding(1) var source: texture_2d<u32>;
@elif(SAMPLE_SINT)
@group(0) @binding(1) var source: texture_2d<i32>;
@else
@group(0) @binding(1) var source: texture_2d<f32>;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// One triangle over the target (Bevy's fullscreen triangle, kept here so the shader stands alone).
@vertex
fn vertex(@builtin(vertex_index) index: u32) -> VertexOut {
    let uv = vec2<f32>(f32(index >> 1u), f32(index & 1u)) * 2.0;
    var out: VertexOut;
    out.position = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    out.uv = uv;
    return out;
}

// NaN or infinity: all exponent bits set (no float compare a driver may fold away).
fn non_finite(v: vec3<f32>) -> bool {
    let e = bitcast<vec3<u32>>(v) & vec3<u32>(0x7f800000u);
    return any(e == vec3<u32>(0x7f800000u));
}

// The target is unorm and the screen sRGB: decode, so a remapped 0.5 shows as mid grey (128).
fn to_display(v: vec3<f32>) -> vec3<f32> {
    let c = clamp(v, vec3<f32>(0.0), vec3<f32>(1.0));
    return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045));
}

@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(source));
    let texel = clamp(vec2<i32>(in.uv * vec2<f32>(size)), vec2<i32>(0), size - 1);
    let raw = vec4<f32>(textureLoad(source, texel, 0));
    let channel = params.mode.x;
    var v = raw.rgb;
    if channel < 4u {
        v = vec3<f32>(raw[channel]);
    }
    if non_finite(v) {
        return vec4<f32>(1.0, 0.0, 1.0, 1.0);
    }
    return vec4<f32>(to_display((v - params.range.x) * params.range.y), 1.0);
}
