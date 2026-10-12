// The cloud shadow map (weather.md "Cloud shadows"): optical depth through
// the cloud layer along the sun direction, for a square of the sun's light
// plane centered on the camera. It reaches every PBR material through the
// sun's `DirectionalLightTexture`. Its edge fades to the weather's mean
// cover, so past the map the ground reads as "average cloud", never black
// (Megalophobia's out-of-range-is-lit rule, generalized).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::cloud_common::{ramp, shell_segment};
import super::cloud_density::{clouds, weather_sample, height_fraction, cloud_density, cirrus_optical_depth};

@fragment
fn shadow(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Bevy reads a light texture at uv = local.xy * (-0.5, 0.5) + 0.5.
    let p = clouds.shadow_center + clouds.shadow_x * (1.0 - 2.0 * in.uv.x) + clouds.shadow_y * (2.0 * in.uv.y - 1.0);
    let l = clouds.light_dir;
    let seg = shell_segment(p - clouds.planet_center, l, clouds.inner_radius, clouds.base, clouds.thickness);
    // Direct-light transmittance through the low layer (full extinction: the
    // cookie only scales the sun's direct light; sky fill stays) and the
    // high sheet.
    var od = 0.0;
    if seg.y > seg.x && l.y > 0.0 {
        let n = max(clouds.shadow_steps, 1u);
        let span = min(seg.y - seg.x, clouds.thickness * 4.0);
        let ds = span / f32(n);
        var tau = 0.0;
        for (var i = 0u; i < n; i += 1u) {
            let s = p + l * (seg.x + (f32(i) + 0.5) * ds);
            // With the erosion octave: the shadow edge is as crisp as the cloud edge.
            tau += cloud_density(s, height_fraction(s), weather_sample(s), true) * ds;
        }
        od = tau * clouds.extinction;
    }
    if l.y > 0.0 {
        od += cirrus_optical_depth(p, l);
    }
    var lit = mix(1.0, exp(-od), clouds.shadow_strength);
    let edge = max(abs(in.uv.x * 2.0 - 1.0), abs(in.uv.y * 2.0 - 1.0));
    let average = 1.0 - clouds.mean_cover * clouds.shadow_strength * 0.6;
    lit = mix(lit, average, ramp(0.75, 1.0, edge));
    return vec4(lit, lit, lit, 1.0);
}
