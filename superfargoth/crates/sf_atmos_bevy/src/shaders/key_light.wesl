// The key light the clouds and the fog are lit by: the brightest directional
// light *as seen from the camera* (sun by day, moon by night), through the
// atmosphere's transmittance and the planet's shadow.

import bevy_pbr::atmosphere::{bindings::lights, functions::sample_transmittance_lut};

// The planet's shadow: 0 where the light at radius `r` and cosine `mu` is
// below the horizon. Bevy's transmittance LUT clamps such rays to the
// horizon (deep red, not 0): unshadowed, high clouds glowed red at night
// (the "red streaks" with `--clouds clear`, nightsky 2026-10-13). Smoothed
// over ~0.5° so the terminator doesn't alias.
fn planet_lit_at(inner_radius: f32, r: f32, mu: f32) -> f32 {
    let rho = inner_radius / max(r, inner_radius);
    let mu_horizon = -sqrt(max(1.0 - rho * rho, 0.0));
    return smoothstep(mu_horizon - 0.004, mu_horizon + 0.004, mu);
}

struct KeyLight {
    // Toward the light.
    dir: vec3<f32>,
    // The light's unattenuated color × illuminance.
    color: vec3<f32>,
    // What reaches the camera's altitude (transmittance × planet shadow).
    seen: vec3<f32>,
};

// The raster path keeps the sun's unattenuated illuminance after sunset
// (the sky needs it for twilight), so ranking raw colors picked the set sun
// all night: rank by what reaches the camera instead.
fn key_light(ro_c: vec3<f32>, inner_radius: f32) -> KeyLight {
    var k: KeyLight;
    k.dir = vec3(0.0, 1.0, 0.0);
    k.color = vec3(0.0);
    k.seen = vec3(0.0);
    var brightest = -1.0;
    let r = length(ro_c);
    let up = ro_c / r;
    for (var i = 0u; i < lights.n_directional_lights; i += 1u) {
        let c = lights.directional_lights[i].color.rgb;
        let dir = normalize(lights.directional_lights[i].direction_to_light.xyz);
        let mu = dot(up, dir);
        let seen = c * sample_transmittance_lut(r, mu) * planet_lit_at(inner_radius, r, mu);
        let lum = seen.r + seen.g + seen.b + (c.r + c.g + c.b) * 1e-6;
        if lum > brightest {
            brightest = lum;
            k.dir = dir;
            k.color = c;
            k.seen = seen;
        }
    }
    return k;
}
