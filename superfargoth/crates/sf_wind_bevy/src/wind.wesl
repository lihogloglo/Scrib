// The wind field's bindings (weather.md "Wind"): one read-only storage
// buffer (`sf_wind_bevy::WindBuffer`) with this frame's and last frame's
// field, and the samplers every consumer calls.
//
// The consumer picks the slot with two shader defs (UInt), e.g. in a
// material's `specialize`: `sf_wind_bevy::wind_shader_defs(group, binding)`
// (`SF_WIND_GROUP`, `SF_WIND_BINDING`). Then:
//
//     import sf_wind_bevy::wind::{wind_sample, wind_sample_prev};
//     let v = wind_sample(rel, above_ground);           // m/s, this frame
//     let v_prev = wind_sample_prev(rel, above_ground); // last frame (motion vectors)
//
// `rel` is the origin-relative world position (the mesh's world position
// in Bevy's frame). Both frames are expressed around the *current* floating
// origin, so a consumer passes the same kind of position to both: last
// frame's rendering, re-expressed after an origin shift. The pure field
// functions are `sf_wind::field` (`sf_wind::WESL`, CPU twin `WindFrame`).

import sf_wind::field::{WindParams, WindSource, wind_global, wind_source, wind_source_count};

const WIND_MAX_SOURCES: u32 = 32u;

struct WindData {
    params: WindParams,
    prev_params: WindParams,
    sources: array<WindSource, 32>,
    prev_sources: array<WindSource, 32>,
}

@group(constants::SF_WIND_GROUP) @binding(constants::SF_WIND_BINDING)
var<storage, read> wind: WindData;

// Velocity (m/s) at an origin-relative position `above_ground` meters over
// the ground: the weather's wind plus every local source.
fn wind_sample(rel: vec3<f32>, above_ground: f32) -> vec3<f32> {
    var v = wind_global(wind.params, rel, above_ground);
    let n = min(wind_source_count(wind.params), WIND_MAX_SOURCES);
    for (var i = 0u; i < n; i++) {
        v += wind_source(wind.sources[i], rel);
    }
    return v;
}

// The same field as last frame (prepass motion vectors, `time_prev` style).
fn wind_sample_prev(rel: vec3<f32>, above_ground: f32) -> vec3<f32> {
    var v = wind_global(wind.prev_params, rel, above_ground);
    let n = min(wind_source_count(wind.prev_params), WIND_MAX_SOURCES);
    for (var i = 0u; i < n; i++) {
        v += wind_source(wind.prev_sources[i], rel);
    }
    return v;
}
