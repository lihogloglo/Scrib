// The `wind` example's ground: wind speed at 2 m from the GPU field
// (`sf_wind_bevy::wind`), so the CPU-twin arrows above it should agree.

import bevy_pbr::render::forward_io::VertexOutput;
import sf_wind_bevy::wind::{wind, wind_sample};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let rel = in.world_position.xyz;
    let v = wind_sample(rel, 2.0);
    // Calm dark blue, mean speed green, twice the mean (gust fronts, bursts) yellow.
    let mean = max(wind.params.axis_speed.z, 0.1);
    let t = clamp(length(v) / (2.0 * mean), 0.0, 1.0);
    var c = mix(vec3(0.02, 0.03, 0.12), vec3(0.05, 0.45, 0.15), smoothstep(0.0, 0.5, t));
    c = mix(c, vec3(1.0, 0.85, 0.2), smoothstep(0.5, 1.0, t));
    // 15 m grid (the arrow spacing).
    let cell = abs(fract(rel.xz / 15.0 + 0.5) - 0.5) * 15.0;
    let line = 1.0 - smoothstep(0.0, 0.4, min(cell.x, cell.y));
    return vec4(mix(c, vec3(0.3), line * 0.5), 1.0);
}
