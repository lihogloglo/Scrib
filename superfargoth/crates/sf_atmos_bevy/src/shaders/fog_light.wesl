// The fog's lighting, once per frame into a 66×1 texture (weather.md "Fog",
// F2): texels 0-63 the sky fill per view azimuth (the sky toward the horizon
// at that azimuth and the zenith, averaged), texel 64 the key light as seen
// at the camera, texel 65 its direction. Everything here depends on the
// camera only, so the full-resolution composite reads three texels instead
// of walking the lights and sampling the LUTs per pixel.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::{bindings::view, functions::{sample_sky_view_lut, direction_world_to_atmosphere}};
import super::cloud_common::{Clouds, PI};
import super::key_light::key_light;

@group(1) @binding(0) var<uniform> clouds: Clouds;

// Keep in step with `FOG_AZIMUTHS` (render.rs) and the composite.
const AZIMUTHS: u32 = 64u;

@fragment
fn fog_light(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let x = u32(in.position.x);
    let ro_c = view.world_position - clouds.planet_center;
    let r = length(ro_c);
    let up = ro_c / r;
    if x < AZIMUTHS {
        let phi = (f32(x) + 0.5) / f32(AZIMUTHS) * 2.0 * PI;
        let horizon = normalize(vec3(cos(phi), 0.05, sin(phi)));
        let sky_h = sample_sky_view_lut(r, direction_world_to_atmosphere(horizon));
        let sky_up = sample_sky_view_lut(r, direction_world_to_atmosphere(up));
        return vec4((sky_h + sky_up) * 0.5, 1.0);
    }
    let key = key_light(ro_c, clouds.inner_radius);
    if x == AZIMUTHS {
        return vec4(key.seen, 1.0);
    }
    return vec4(key.dir, 1.0);
}
