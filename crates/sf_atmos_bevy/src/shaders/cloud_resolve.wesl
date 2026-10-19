// Temporal resolve into the half-resolution cloud history (weather.md
// "Clouds", Rendering 5): reproject last frame's history by the cloud's
// distance (camera motion, wind, floating-origin shifts), clamp it to the
// current traced neighborhood, and blend in this frame's fresh samples.
//
// Taps match by population, never by cloud distance (research/
// clouds-upsampling.md): an open texel (it saw sky, B = far) only takes open
// taps, a cut one (stopped at geometry) only taps cut at about the same depth.
// So bright sky clouds never leak onto a silhouette, a texel the tower just
// uncovered drops the tower's history, and clouds of different distances in
// the open sky still accumulate (rejecting on cloud distance made their edges
// crawl as the camera moved).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::{bindings::view, functions::uv_to_ray_direction};
import super::cloud_common::{Clouds, sub_pixel};

@group(1) @binding(0) var<uniform> clouds: Clouds;
@group(1) @binding(1) var trace_color: texture_2d<f32>;
@group(1) @binding(2) var trace_dist: texture_2d<f32>;
@group(1) @binding(3) var history_color: texture_2d<f32>;
@group(1) @binding(4) var history_dist: texture_2d<f32>;
@group(1) @binding(5) var linear_sampler: sampler;

struct ResolveOutput {
    @location(0) color: vec4<f32>,
    // R: cloud distance; G: the fog bank modulation; B: the cut depth (far: open).
    @location(1) dist: vec4<f32>,
};

/// Whether a tap's cut depth `b` belongs with this texel's `cut`.
fn same_population(b: f32, cut: f32) -> bool {
    let open_tap = b >= clouds.far;
    let open_here = cut >= clouds.far;
    return (open_tap && open_here) || (!open_tap && !open_here && abs(b - cut) <= 0.1 * cut + 5.0);
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

    // Neighborhood of this frame's samples of the same population, for the
    // history clamp (the center tap always qualifies).
    var lo = vec4(1e9);
    var hi = vec4(-1e9);
    var bank_lo = 1e9;
    var bank_hi = -1e9;
    for (var y = -1; y <= 1; y += 1) {
        for (var x = -1; x <= 1; x += 1) {
            let q = clamp(tp + vec2(x, y), vec2(0), trace_max);
            let d = textureLoad(trace_dist, q, 0);
            if !same_population(d.z, cut) {
                continue;
            }
            let c = textureLoad(trace_color, q, 0);
            lo = min(lo, c);
            hi = max(hi, c);
            bank_lo = min(bank_lo, d.y);
            bank_hi = max(bank_hi, d.y);
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
    for (var i = 0; i < 4; i += 1) {
        let o = vec2(i & 1, i >> 1u);
        let q = clamp(vec2<i32>(hbase) + o, vec2(0), hmax);
        let d = textureLoad(history_dist, q, 0);
        let w = mix(1.0 - hf.x, hf.x, f32(o.x)) * mix(1.0 - hf.y, hf.y, f32(o.y)) * select(0.0, 1.0, same_population(d.z, cut));
        history_sum += textureLoad(history_color, q, 0) * w;
        history_dist_sum += d.x * w;
        history_bank_sum += d.y * w;
        history_w += w;
    }
    let valid = clouds.history_valid != 0u && clip.w > 0.0
        && all(prev_uv >= vec2(0.0)) && all(prev_uv <= vec2(1.0)) && history_w > 1e-3;

    var out: ResolveOutput;
    if valid {
        let history = clamp(history_sum / history_w, lo, hi);
        let history_bank = clamp(history_bank_sum / history_w, bank_lo, bank_hi);
        if fresh {
            out.color = mix(history, current, 0.5);
            out.dist = vec4(current_dist, mix(history_bank, current_d.y, 0.5), cut, 0.0);
        } else {
            out.color = history;
            out.dist = vec4(history_dist_sum / history_w, history_bank, cut, 0.0);
        }
    } else {
        // No usable history: this frame's own sample of the same population
        // (nearest; a checkerboard block-mate of an open texel is open too,
        // thanks to the trace's 6x6 furthest-depth window).
        out.color = current;
        out.dist = vec4(current_dist, current_d.y, cut, 0.0);
    }
    return out;
}
