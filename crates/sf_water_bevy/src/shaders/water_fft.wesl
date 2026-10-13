// sf_water_bevy: the ocean's FFT cascades (water.md "Simulation"), one dispatch chain per frame:
// spectrum (h0 evolved to t: height, choppy displacement, slopes, displacement derivatives, packed
// as 4 complex signals per texel) → inverse FFT along rows → along columns (radix-2
// Cooley-Tukey in workgroup memory, one line per workgroup) → assemble (textures + persistent
// Jacobian foam). Conventions match the CPU twin (`sf_water::CascadeTwin`): f(x) = Σ F(k)·e^{ik·x},
// h̃ = h0·e^{iωt} + conj(h0(−k))·e^{−iωt}, D̃ = λ·i·k̂·h̃, ω rounded to whole cycles per repeat
// period. Wave model after Tessendorf 2001 and bevy-aqua (MIT OR Apache-2.0).

struct FftParams {
    sizes: vec4<f32>,
    n: u32,
    cascades: u32,
    // (t mod T) / T.
    cycles_t: f32,
    dt: f32,
    choppiness: f32,
    foam_decay: f32,
    foam_threshold: f32,
    foam_gain: f32,
}

@group(0) @binding(0) var<uniform> params: FftParams;
// (h0.re, h0.im, cycles per period, 0) per texel and cascade.
@group(0) @binding(1) var h0: texture_2d_array<f32>;
// Two vec4 (four complex) per texel: [h + i·Dx, Dz + i·Sx], [Sz + i·Jxx, Jzz + i·Jxz].
@group(0) @binding(2) var<storage, read_write> spec: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> foam: array<f32>;
@group(0) @binding(4) var displacement: texture_storage_2d_array<rgba16float, write>;
@group(0) @binding(5) var slopes: texture_storage_2d_array<rgba16float, write>;

const N: u32 = 256u;
const LOG_N: u32 = 8u;
const TAU: f32 = 6.283185307179586;

fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn mul_i(a: vec2<f32>) -> vec2<f32> {
    return vec2(-a.y, a.x);
}

fn signed_freq(i: u32) -> f32 {
    if i <= N / 2u {
        return f32(i);
    }
    return f32(i) - f32(N);
}

fn spec_index(c: u32, z: u32, x: u32) -> u32 {
    return ((c * N + z) * N + x) * 2u;
}

@compute @workgroup_size(16, 16, 1)
fn spectrum(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let z = id.y;
    let c = id.z;
    if x >= N || z >= N || c >= params.cascades {
        return;
    }
    let t0 = textureLoad(h0, vec2<i32>(i32(x), i32(z)), i32(c), 0);
    let t1 = textureLoad(h0, vec2<i32>(i32((N - x) % N), i32((N - z) % N)), i32(c), 0);
    let k = vec2(signed_freq(x), signed_freq(z)) * (TAU / params.sizes[c]);
    let kl = length(k);
    var khat = vec2(0.0);
    if kl > 0.0 {
        khat = k / kl;
    }
    let phase = TAU * fract(t0.z * params.cycles_t);
    let e = vec2(cos(phase), sin(phase));
    let a = cmul(t0.xy, e);
    let b = cmul(vec2(t1.x, -t1.y), vec2(e.x, -e.y));
    let h = a + b;
    let lam = params.choppiness;
    let ih = mul_i(h);
    let dx = ih * (khat.x * lam);
    let dz = ih * (khat.y * lam);
    let sx = ih * k.x;
    let sz = ih * k.y;
    let jxx = h * (-lam * k.x * khat.x);
    let jzz = h * (-lam * k.y * khat.y);
    let jxz = h * (-lam * k.x * khat.y);
    let i = spec_index(c, z, x);
    spec[i] = vec4(h + mul_i(dx), dz + mul_i(sx));
    spec[i + 1u] = vec4(sz + mul_i(jxx), jzz + mul_i(jxz));
}

var<workgroup> fft_buf: array<vec4<f32>, 512>;

// Inverse FFT of one line (`N` texels, element `e` at `base + e·stride·2`), in place.
fn fft_line(lane: u32, base: u32, stride: u32) {
    for (var e = lane; e < N; e += N / 2u) {
        let r = reverseBits(e) >> (32u - LOG_N);
        let src = base + e * stride * 2u;
        fft_buf[r * 2u] = spec[src];
        fft_buf[r * 2u + 1u] = spec[src + 1u];
    }
    workgroupBarrier();
    for (var s = 0u; s < LOG_N; s++) {
        let span = 1u << s;
        let j = lane % span;
        let i0 = (lane / span) * 2u * span + j;
        let i1 = i0 + span;
        let angle = TAU * f32(j) / f32(2u * span);
        let w = vec2(cos(angle), sin(angle));
        let a0 = fft_buf[i0 * 2u];
        let a1 = fft_buf[i0 * 2u + 1u];
        let b0 = fft_buf[i1 * 2u];
        let b1 = fft_buf[i1 * 2u + 1u];
        let wb0 = vec4(cmul(b0.xy, w), cmul(b0.zw, w));
        let wb1 = vec4(cmul(b1.xy, w), cmul(b1.zw, w));
        fft_buf[i0 * 2u] = a0 + wb0;
        fft_buf[i0 * 2u + 1u] = a1 + wb1;
        fft_buf[i1 * 2u] = a0 - wb0;
        fft_buf[i1 * 2u + 1u] = a1 - wb1;
        workgroupBarrier();
    }
    for (var e = lane; e < N; e += N / 2u) {
        let dst = base + e * stride * 2u;
        spec[dst] = fft_buf[e * 2u];
        spec[dst + 1u] = fft_buf[e * 2u + 1u];
    }
}

@compute @workgroup_size(128, 1, 1)
fn fft_rows(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {
    fft_line(lid.x, (wid.y * N + wid.x) * N * 2u, 1u);
}

@compute @workgroup_size(128, 1, 1)
fn fft_cols(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {
    fft_line(lid.x, (wid.y * N * N + wid.x) * 2u, N);
}

@compute @workgroup_size(16, 16, 1)
fn assemble(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let z = id.y;
    let c = id.z;
    if x >= N || z >= N || c >= params.cascades {
        return;
    }
    let i = spec_index(c, z, x);
    let p0 = spec[i];
    let p1 = spec[i + 1u];
    let h = p0.x;
    let dx = p0.y;
    let dz = p0.z;
    let sx = p0.w;
    let sz = p1.x;
    let jxx = p1.y;
    let jzz = p1.z;
    let jxz = p1.w;
    // Whitecaps where the surface folds (Jacobian below the threshold), kept with decay
    // (Dupuy and Bruneton 2012).
    let jacobian = (1.0 + jxx) * (1.0 + jzz) - jxz * jxz;
    let fi = (c * N + z) * N + x;
    let fresh = saturate((params.foam_threshold - jacobian) * params.foam_gain);
    let f = max(foam[fi] * exp(-params.dt / max(params.foam_decay, 1e-3)), fresh);
    foam[fi] = f;
    let p = vec2<i32>(i32(x), i32(z));
    textureStore(displacement, p, i32(c), vec4(dx, h, dz, f));
    textureStore(slopes, p, i32(c), vec4(sx, sz, jxx, jzz));
}
