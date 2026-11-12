// Temporal resolve into the half-resolution cloud history (weather.md
// "Clouds", Rendering 5): reproject last frame's history by the cloud's
// distance (camera motion, wind, floating-origin shifts), clip it to the
// current traced neighborhood (variance clip in YCoCg, Salvi 2016) and fold in
// this frame's fresh samples with a small weight (an exponential history of
// ~16 samples, WE-10 fix 1; `sf_atmos::temporal` is the CPU twin).
//
// Taps match by population, never by cloud distance (research/
// clouds-upsampling.md): an open texel (it saw sky, B = far) only takes open
// taps, a cut one (stopped at geometry) only taps cut at about the same depth.
// So bright sky clouds never leak onto a silhouette, a texel the tower just
// uncovered drops the tower's history, and clouds of different distances in
// the open sky still accumulate (rejecting on cloud distance made their edges
// crawl as the camera moved).
//
// With `fog.bank_edges` (FOG-BANK) the bank comes as two (distance, bank) pairs, at the
// trace window's furthest and nearest depth: the far one follows the population above, the
// near one any tap whose near distance matches (the nearest surface stays the same when an
// edge texel flips between open and cut, so its bank keeps its history).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::{bindings::view, functions::uv_to_ray_direction};
import super::cloud_common::{Clouds, sub_pixel};
import super::bank_edges::{bank_decode, bank_encode, bank_same_depth};

@group(1) @binding(0) var<uniform> clouds: Clouds;
@group(1) @binding(1) var trace_color: texture_2d<f32>;
@group(1) @binding(2) var trace_dist: texture_2d<f32>;
@group(1) @binding(3) var history_color: texture_2d<f32>;
@group(1) @binding(4) var history_dist: texture_2d<f32>;
@group(1) @binding(5) var linear_sampler: sampler;

struct ResolveOutput {
    @location(0) color: vec4<f32>,
    // R: cloud distance; G: the fog bank modulation; B: the cut depth (far: open).
    // A: spare. With `fog.bank_edges` G and A are the far and near `bank_encode` pairs.
    @location(1) dist: vec4<f32>,
};

/// Whether a tap's cut depth `b` belongs with this texel's `cut`.
fn same_population(b: f32, cut: f32) -> bool {
    let open_tap = b >= clouds.far;
    let open_here = cut >= clouds.far;
    return (open_tap && open_here) || (!open_tap && !open_here && abs(b - cut) <= 0.1 * cut + 5.0);
}

// The clip's colour space (`sf_atmos::temporal`): YCoCg with alpha as is under
// the variance clip, plain RGB for the legacy min/max clamp.
fn to_clip_space(c: vec4<f32>) -> vec4<f32> {
    if clouds.clip_gamma <= 0.0 {
        return c;
    }
    return vec4(dot(c.rgb, vec3(0.25, 0.5, 0.25)), dot(c.rgb, vec3(0.5, 0.0, -0.5)), dot(c.rgb, vec3(-0.25, 0.5, -0.25)), c.a);
}

fn from_clip_space(y: vec4<f32>) -> vec4<f32> {
    if clouds.clip_gamma <= 0.0 {
        return y;
    }
    return vec4(y.x + y.y - y.z, y.x + y.z, y.x - y.y - y.z, y.w);
}

// `sf_atmos::temporal::clip_toward_center`: moves `h` toward the box centre
// until it lies inside (Pedersen 2016), keeping its hue better than a clamp.
fn clip_toward_center(h: vec4<f32>, lo: vec4<f32>, hi: vec4<f32>) -> vec4<f32> {
    let c = 0.5 * (lo + hi);
    let v = h - c;
    let a = abs(v) / max(0.5 * (hi - lo), vec4(1e-6));
    let reach = max(max(a.x, a.y), max(a.z, a.w));
    return select(h, c + v / reach, reach > 1.0);
}

// The far bank as stored: a pair at this frame's far distance (FOG-BANK), or the plain value.
fn store_bank(t: f32, bank: f32, edges: bool) -> f32 {
    return select(bank, bank_encode(t, bank), edges);
}

@fragment
fn resolve(in: FullscreenVertexOutput) -> ResolveOutput {
    let px = vec2<u32>(in.position.xy);
    let uv = (vec2<f32>(px) + 0.5) / clouds.half_size;
    let trace_max = vec2<i32>(clouds.trace_size) - 1;
    var tp = vec2<i32>(px);
    var fresh = true;
    if clouds.checkerboard != 0u {
        tp = vec2<i32>(px / 2u);
        fresh = all(px % 2u == sub_pixel(clouds.frame));
    }
    tp = min(tp, trace_max);
    let current = textureLoad(trace_color, tp, 0);
    let current_d = textureLoad(trace_dist, tp, 0);
    let current_dist = current_d.x;
    let cut = current_d.z;
    // The bank pairs (FOG-BANK): (distance, bank) to the window's far and near depth.
    let edges = clouds.fog.bank_edges != 0u;
    var current_far = vec2(0.0, current_d.y);
    var current_near = vec2(0.0);
    if edges {
        current_far = bank_decode(current_d.y);
        current_near = bank_decode(current_d.w);
    }

    // Neighborhood of this frame's samples of the same population, for the
    // history clip (the center tap always qualifies): min/max and, for the
    // variance clip, the moments in YCoCg (alpha as a fourth channel).
    var lo = vec4(1e9);
    var hi = vec4(-1e9);
    var m1 = vec4(0.0);
    var m2 = vec4(0.0);
    var n = 0.0;
    var bank_lo = 1e9;
    var bank_hi = -1e9;
    var bank_m1 = 0.0;
    var bank_m2 = 0.0;
    // The near bank's neighborhood: taps whose near distance matches this texel's.
    var near_lo = 1e9;
    var near_hi = -1e9;
    var near_m1 = 0.0;
    var near_m2 = 0.0;
    var near_n = 0.0;
    for (var y = -1; y <= 1; y += 1) {
        for (var x = -1; x <= 1; x += 1) {
            let q = clamp(tp + vec2(x, y), vec2(0), trace_max);
            var d = textureLoad(trace_dist, q, 0);
            if edges {
                let near = bank_decode(d.w);
                if bank_same_depth(near.x, current_near.x) {
                    near_lo = min(near_lo, near.y);
                    near_hi = max(near_hi, near.y);
                    near_m1 += near.y;
                    near_m2 += near.y * near.y;
                    near_n += 1.0;
                }
                d.y = bank_decode(d.y).y;
            }
            if !same_population(d.z, cut) {
                continue;
            }
            let c = to_clip_space(textureLoad(trace_color, q, 0));
            lo = min(lo, c);
            hi = max(hi, c);
            m1 += c;
            m2 += c * c;
            n += 1.0;
            bank_lo = min(bank_lo, d.y);
            bank_hi = max(bank_hi, d.y);
            bank_m1 += d.y;
            bank_m2 += d.y * d.y;
        }
    }
    // Variance clip (Salvi 2016): mean ± γσ inside the min/max box; γ = 0
    // keeps the plain min/max clamp (the legacy resolve).
    if clouds.clip_gamma > 0.0 {
        let mean = m1 / n;
        let sigma = sqrt(max(m2 / n - mean * mean, vec4(0.0)));
        lo = max(lo, mean - clouds.clip_gamma * sigma);
        hi = min(hi, mean + clouds.clip_gamma * sigma);
        let bank_mean = bank_m1 / n;
        let bank_sigma = sqrt(max(bank_m2 / n - bank_mean * bank_mean, 0.0));
        bank_lo = max(bank_lo, bank_mean - clouds.clip_gamma * bank_sigma);
        bank_hi = min(bank_hi, bank_mean + clouds.clip_gamma * bank_sigma);
        if near_n > 0.0 {
            let near_mean = near_m1 / near_n;
            let near_sigma = sqrt(max(near_m2 / near_n - near_mean * near_mean, 0.0));
            near_lo = max(near_lo, near_mean - clouds.clip_gamma * near_sigma);
            near_hi = min(near_hi, near_mean + clouds.clip_gamma * near_sigma);
        }
    }

    // Where this pixel's cloud was last frame: same world point, minus this
    // frame's wind step, in last frame's origin-relative (and jittered)
    // clip space.
    let rd = uv_to_ray_direction(uv);
    let p = view.world_position + rd * current_dist - clouds.wind_step + clouds.origin_shift;
    let clip = clouds.prev_clip_from_world * vec4(p, 1.0);
    let prev_uv = (clip.xy / clip.w) * vec2(0.5, -0.5) + 0.5;

    // History: bilinear over the four texels around `prev_uv`, each counted
    // only when it held the same population last frame, renormalized.
    let hpf = prev_uv * clouds.half_size - 0.5;
    let hbase = floor(hpf);
    let hf = hpf - hbase;
    let hmax = vec2<i32>(clouds.half_size) - 1;
    var history_sum = vec4(0.0);
    var history_dist_sum = 0.0;
    var history_bank_sum = 0.0;
    var history_w = 0.0;
    var history_near_sum = 0.0;
    var history_near_w = 0.0;
    for (var i = 0; i < 4; i += 1) {
        let o = vec2(i & 1, i >> 1u);
        let q = clamp(vec2<i32>(hbase) + o, vec2(0), hmax);
        var d = textureLoad(history_dist, q, 0);
        let bilinear = mix(1.0 - hf.x, hf.x, f32(o.x)) * mix(1.0 - hf.y, hf.y, f32(o.y));
        if edges {
            let near = bank_decode(d.w);
            let wn = bilinear * select(0.0, 1.0, bank_same_depth(near.x, current_near.x));
            history_near_sum += near.y * wn;
            history_near_w += wn;
            d.y = bank_decode(d.y).y;
        }
        let w = bilinear * select(0.0, 1.0, same_population(d.z, cut));
        history_sum += textureLoad(history_color, q, 0) * w;
        history_dist_sum += d.x * w;
        history_bank_sum += d.y * w;
        history_w += w;
    }
    let reprojected = clouds.history_valid != 0u && clip.w > 0.0
        && all(prev_uv >= vec2(0.0)) && all(prev_uv <= vec2(1.0));
    let valid = reprojected && history_w > 1e-3;

    // The near bank: its own history (by near distance), else this frame's.
    var near_bank = current_near.y;
    if edges && reprojected && history_near_w > 1e-3 {
        let history_near = clamp(history_near_sum / history_near_w, near_lo, near_hi);
        near_bank = select(history_near, mix(history_near, current_near.y, clouds.history_weight), fresh);
    }
    let near_out = select(0.0, bank_encode(current_near.x, near_bank), edges);

    var out: ResolveOutput;
    if valid {
        var history = to_clip_space(history_sum / history_w);
        var history_bank = history_bank_sum / history_w;
        if clouds.clip_gamma > 0.0 {
            history = clip_toward_center(history, lo, hi);
        } else {
            history = clamp(history, lo, hi);
        }
        history_bank = clamp(history_bank, bank_lo, bank_hi);
        let rgb_history = from_clip_space(history);
        if fresh {
            // Exponential history: the fresh sample weighs `history_weight`
            // (`sf_atmos::temporal::accumulate`).
            out.color = mix(rgb_history, current, clouds.history_weight);
            out.dist = vec4(current_dist, store_bank(current_far.x, mix(history_bank, current_far.y, clouds.history_weight), edges), cut, near_out);
        } else {
            out.color = rgb_history;
            out.dist = vec4(history_dist_sum / history_w, store_bank(current_far.x, history_bank, edges), cut, near_out);
        }
    } else {
        // No usable history: this frame's own sample of the same population
        // (nearest; a checkerboard block-mate of an open texel is open too,
        // thanks to the trace's 6x6 furthest-depth window).
        out.color = current;
        out.dist = vec4(current_dist, current_d.y, cut, near_out);
    }
    return out;
}
