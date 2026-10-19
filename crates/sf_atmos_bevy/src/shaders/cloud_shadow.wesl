// The cloud shadow map (weather.md "Cloud shadows"): optical depth through
// the cloud layer along the sun direction, for a square of the sun's light
// plane around the camera. It reaches every PBR material through the sun's
// `DirectionalLightTexture`. Its edge fades to the weather's mean cover, so
// past the map the ground reads as "average cloud", never black
// (Megalophobia's out-of-range-is-lit rule, generalized).
//
// Toroidal: the light texture repeats and its transform sits at a world
// anchor (`shadow_center`) that moves in 2 km steps, so a texel always holds
// the same column of sky: the one, among the texel's repeats, nearest the
// camera. That lets the map refresh a band of rows per frame (`sf_atmos::shadow`).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::cloud_common::{ramp, shell_segment};
import super::cloud_density::{clouds, weather_sample_a, ray_a, ray_local_a, height_fraction, cloud_density, cirrus_optical_depth};

@fragment
fn shadow(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Bevy reads a light texture at uv = local.xy * (-0.5, 0.5) + 0.5, local
    // in half extents from the anchor; one repeat is 2 local units.
    let texel = vec2(1.0 - 2.0 * in.uv.x, 2.0 * in.uv.y - 1.0);
    let to_cam = clouds.shadow_camera - clouds.shadow_center;
    let cam = vec2(dot(to_cam, clouds.shadow_x), dot(to_cam, clouds.shadow_y)) / vec2(dot(clouds.shadow_x, clouds.shadow_x), dot(clouds.shadow_y, clouds.shadow_y));
    var d = texel - cam;
    d -= 2.0 * round(d * 0.5);
    let l = clouds.light_dir;
    var p = clouds.shadow_center + clouds.shadow_x * (cam.x + d.x) + clouds.shadow_y * (cam.y + d.y);
    // The texel stands for the whole line along the light: march it from
    // sea level up (flat approximation over the map, < 6 m of curvature).
    if l.y > 0.0 {
        let sea = clouds.planet_center.y + clouds.inner_radius + 1.0;
        p += l * ((sea - p.y) / l.y);
    }
    let seg = shell_segment(p - clouds.planet_center, l, clouds.inner_radius, clouds.base, clouds.thickness);
    // Direct-light transmittance through the low layer (full extinction: the
    // cookie only scales the sun's direct light; sky fill stays) and the
    // high sheet.
    var od = 0.0;
    if seg.y > seg.x && l.y > 0.0 {
        let n = max(clouds.shadow_steps, 1u);
        let span = min(seg.y - seg.x, clouds.thickness * 4.0);
        let ds = span / f32(n);
        // The world weather map: per sample, or 3 lookups along this texel's ray (Medium/Low).
        let ray = ray_a(p, l, seg.x, seg.x + span);
        var tau = 0.0;
        for (var i = 0u; i < n; i += 1u) {
            let s = p + l * (seg.x + (f32(i) + 0.5) * ds);
            // With the erosion octave: the shadow edge is as crisp as the cloud edge.
            let a = ray_local_a(ray, (f32(i) + 0.5) / f32(n), s);
            tau += cloud_density(s, height_fraction(s), weather_sample_a(s, a), true) * ds;
        }
        od = tau * clouds.extinction;
    }
    if l.y > 0.0 {
        od += cirrus_optical_depth(p, l);
    }
    var lit = mix(1.0, exp(-od), clouds.shadow_strength);
    let edge = max(abs(d.x), abs(d.y));
    let average = 1.0 - clouds.mean_cover * clouds.shadow_strength * 0.6;
    lit = mix(lit, average, ramp(0.75, 1.0, edge));
    return vec4(lit, lit, lit, 1.0);
}
