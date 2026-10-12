// Temporal resolve into the half-resolution cloud history (weather.md
// "Clouds", Rendering 5): reproject last frame's history by the cloud's
// distance (camera motion, wind, floating-origin shifts), clamp it to the
// current traced neighborhood, and blend in this frame's fresh samples.

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
    // R: cloud distance; G: the fog bank modulation.
    @location(1) dist: vec2<f32>,
};

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
    let current_rg = textureLoad(trace_dist, tp, 0).rg;
    let current_dist = current_rg.x;

    // Neighborhood of this frame's samples, for the history clamp.
    var lo = vec4(1e9);
    var hi = vec4(-1e9);
    var bank_lo = 1e9;
    var bank_hi = -1e9;
    for (var y = -1; y <= 1; y += 1) {
        for (var x = -1; x <= 1; x += 1) {
            let c = textureLoad(trace_color, clamp(tp + vec2(x, y), vec2(0), trace_max), 0);
            lo = min(lo, c);
            hi = max(hi, c);
            let b = textureLoad(trace_dist, clamp(tp + vec2(x, y), vec2(0), trace_max), 0).g;
            bank_lo = min(bank_lo, b);
            bank_hi = max(bank_hi, b);
        }
    }

    // Where this pixel's cloud was last frame: same world point, minus this
    // frame's wind step, in last frame's origin-relative coordinates.
    let rd = uv_to_ray_direction(uv);
    let p = view.world_position + rd * current_dist - clouds.wind_step + clouds.origin_shift;
    let clip = clouds.prev_clip_from_world * vec4(p, 1.0);
    let prev_uv = (clip.xy / clip.w) * vec2(0.5, -0.5) + 0.5;
    let valid = clouds.history_valid != 0u && clip.w > 0.0
        && all(prev_uv >= vec2(0.0)) && all(prev_uv <= vec2(1.0));

    var out: ResolveOutput;
    if valid {
        let history = clamp(textureSampleLevel(history_color, linear_sampler, prev_uv, 0.0), lo, hi);
        let hp = clamp(vec2<i32>(prev_uv * clouds.half_size), vec2(0), vec2<i32>(clouds.half_size) - 1);
        let history_rg = textureLoad(history_dist, hp, 0).rg;
        let history_bank = clamp(history_rg.y, bank_lo, bank_hi);
        if fresh {
            out.color = mix(history, current, 0.5);
            out.dist = vec2(current_dist, mix(history_bank, current_rg.y, 0.5));
        } else {
            out.color = history;
            out.dist = vec2(history_rg.x, history_bank);
        }
    } else {
        // No history: upsample this frame's samples.
        out.color = textureSampleLevel(trace_color, linear_sampler, uv, 0.0);
        out.dist = current_rg;
    }
    return out;
}
