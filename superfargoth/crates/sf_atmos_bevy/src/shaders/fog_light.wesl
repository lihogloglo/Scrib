// The fog's lighting, once per frame into a 3×1 texture (weather.md "Fog",
// F2): texel 0 the sky fill, texel 1 the key light as seen at the camera,
// texel 2 its direction. Everything here depends on the camera only, so the
// froxels and the full-resolution composite read texels instead of walking
// the lights and sampling the LUTs per pixel.
//
// Sky fill: the sky dome's mean radiance (its SH L0 band), the isotropic sky
// ambient every froxel and far-fog pixel in-scatters whatever its view
// direction, as UE's volumetric fog and Frostbite (Hillaire 2015) light their
// fog with the sky light. It was the sky toward the horizon per view azimuth
// (with the zenith): a low sun's aureole then lit a vertical column of fog
// toward the sun at every pitch (FG-9). The sun's own forward lobe comes from
// the key light's phase function. CPU twin and tests: `sf_atmos::fog::sky_fill`.
// The fill also carries the view's flat ambient (`Fog.view_ambient`, the host's
// ambient floor that lights the surfaces): without it a night floor lights the
// ground but not the fog, a black absorber seen far through (WE-9).
// `sf_atmos::fog::fog_fill` is the sum's twin.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::{bindings::view, functions::{sample_sky_view_lut, direction_world_to_atmosphere}};
import super::cloud_common::Clouds;
import super::key_light::key_light;

@group(1) @binding(0) var<uniform> clouds: Clouds;

// Keep in step with `sf_atmos::fog::SKY_FILL_SAMPLES` and its spiral.
const SKY_FILL_SAMPLES: u32 = 64u;
const GOLDEN_ANGLE: f32 = 2.3999632;

@fragment
fn fog_light(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let x = u32(in.position.x);
    let ro_c = view.world_position - clouds.planet_center;
    let key = key_light(ro_c, clouds.inner_radius);
    if x == 1u {
        return vec4(key.seen, 1.0);
    }
    if x == 2u {
        return vec4(key.dir, 1.0);
    }
    // A Fibonacci spiral over the upper hemisphere in the sky-view LUT's frame
    // (y up), uniform in solid angle, turned with the key light's azimuth so
    // the aureole's share doesn't change as the sun crosses sample points.
    let r = length(ro_c);
    let key_as = direction_world_to_atmosphere(key.dir);
    let azimuth = atan2(key_as.z, key_as.x);
    var sum = vec3(0.0);
    for (var i = 0u; i < SKY_FILL_SAMPLES; i += 1u) {
        let mu = 1.0 - (f32(i) + 0.5) / f32(SKY_FILL_SAMPLES);
        let phi = f32(i) * GOLDEN_ANGLE + azimuth;
        let s = sqrt(max(1.0 - mu * mu, 0.0));
        sum += sample_sky_view_lut(r, vec3(s * cos(phi), mu, s * sin(phi)));
    }
    // Plus the surfaces' flat ambient (`sf_atmos::fog_fill`): the same light on fog and ground.
    return vec4(sum / f32(SKY_FILL_SAMPLES) + clouds.fog.view_ambient, 1.0);
}
