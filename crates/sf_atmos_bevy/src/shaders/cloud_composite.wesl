// Depth-aware upsample of the half-resolution cloud history onto the main
// target, then the fog in front of it (weather.md "Fog", F2), as one
// `color × T + L` blend (One, SrcAlpha). A tap whose cloud lies behind this
// pixel's geometry is dropped, so mountains and far-field proxies keep sharp
// silhouettes against the clouds.
//
// Fog: the height and bank layers' closed-form optical depth to this
// pixel's own depth (full resolution, so depth edges are exact), the bank
// layer scaled by the traced bank modulation; lit by the key light (HG
// lobe) and the sky fill, both read from the per-frame `fog_light` texture. Sky pixels are fogged
// to `sky_distance`, which also dims the stars, moons and clouds behind.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::bindings::view;
import bevy_pbr::atmosphere::functions::{uv_to_ndc, ndc_to_camera_dist, uv_to_ray_direction};
import super::cloud_common::{Clouds, PI};
import super::fog::{fog_depths, fog_phase};
import super::froxel_common::{Froxels, froxel_fog};

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

@group(1) @binding(0) var<uniform> clouds: Clouds;
@group(1) @binding(1) var cloud_color: texture_2d<f32>;
@group(1) @binding(2) var cloud_dist: texture_2d<f32>;
// `fog_light.wesl`: sky fill per azimuth (0-63), key light seen (64), its direction (65).
@group(1) @binding(3) var fog_light: texture_2d<f32>;
// The froxels (F3): their uniform and integrated volume; past `froxels.far` (or
// with froxels off) the analytic layers carry on from where they end.
@group(1) @binding(4) var<uniform> froxels: Froxels;
@group(1) @binding(5) var froxel_volume: texture_3d<f32>;
@group(1) @binding(6) var froxel_sampler: sampler;
const AZIMUTHS: i32 = 64;

@fragment
fn composite(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let depth = textureLoad(depth_texture, vec2<i32>(in.position.xy), 0);
    var scene_t = 1e30;
    if depth > 0.0 {
        scene_t = ndc_to_camera_dist(vec3(uv_to_ndc(in.uv), depth));
    }
    let hp = in.uv * clouds.half_size - 0.5;
    let base = floor(hp);
    let f = hp - base;
    let hmax = vec2<i32>(clouds.half_size) - 1;
    // The bank modulation is upsampled bilaterally: taps traced at a very
    // different depth (a tree's edge against the sky) count 1/50.
    let pixel_dist = min(scene_t, clouds.far);
    var acc = vec4(0.0);
    var bank = 0.0;
    var bank_w = 0.0;
    for (var i = 0; i < 4; i += 1) {
        let o = vec2(i & 1, i >> 1u);
        let q = clamp(vec2<i32>(base) + o, vec2(0), hmax);
        var c = textureLoad(cloud_color, q, 0);
        let d = textureLoad(cloud_dist, q, 0).rg;
        if d.x > scene_t * 1.02 + 10.0 {
            c = vec4(0.0, 0.0, 0.0, 1.0);
        }
        let w = mix(1.0 - f.x, f.x, f32(o.x)) * mix(1.0 - f.y, f.y, f32(o.y));
        acc += c * w;
        let near = abs(d.x - pixel_dist) < 0.1 * pixel_dist + 5.0;
        let bw = w * select(0.02, 1.0, near) + 1e-6;
        bank += d.y * bw;
        bank_w += bw;
    }

    let fog = clouds.fog;
    if fog.enabled != 0u {
        let ro_c = view.world_position - clouds.planet_center;
        let r = length(ro_c);
        let up = ro_c / r;
        let rd = uv_to_ray_direction(in.uv);
        let mu = dot(up, rd);
        let d = select(fog.sky_distance, scene_t, depth > 0.0);
        // Near: the froxels (in-scatter pre-exposed, transmittance); the analytic
        // layers from their end on, so the near fog isn't counted twice.
        var front = vec4(0.0, 0.0, 0.0, 1.0);
        var start = 0.0;
        if froxels.enabled != 0u {
            start = min(d, froxels.far);
            front = froxel_fog(froxels, froxel_volume, froxel_sampler, in.uv, start);
        }
        var far_light = vec3(0.0);
        let tau = fog_depths(fog, r - clouds.inner_radius + mu * start, mu, d - start);
        let t = exp(-(tau.x + tau.y * bank / bank_w));
        if t < 0.9999 {
            let key_seen = textureLoad(fog_light, vec2(AZIMUTHS, 0), 0).rgb;
            let key_dir = textureLoad(fog_light, vec2(AZIMUTHS + 1, 0), 0).xyz;
            let sun = key_seen * fog_phase(fog.phase_g, dot(rd, key_dir));
            // Sky fill toward the horizon under this view direction (with the
            // zenith), so the fog's colour meets the horizon's.
            let a = (atan2(rd.z, rd.x) / (2.0 * PI) + 1.0) * f32(AZIMUTHS) - 0.5;
            let a0 = floor(a);
            let i0 = (i32(a0) % AZIMUTHS + AZIMUTHS) % AZIMUTHS;
            let i1 = (i0 + 1) % AZIMUTHS;
            let sky = mix(textureLoad(fog_light, vec2(i0, 0), 0).rgb, textureLoad(fog_light, vec2(i1, 0), 0).rgb, a - a0);
            let ambient = sky * fog.ambient;
            far_light = fog.albedo * (sun + ambient) * (1.0 - t) * view.exposure;
        }
        // Behind = clouds over the scene: dimmed by both, lit by the froxels and
        // by the far layers as the froxels let them through.
        let tt = front.a * t;
        acc = vec4(acc.rgb * tt + front.rgb + front.a * far_light, acc.a * tt);
    }

    if acc.a > 0.9995 && max(acc.r, max(acc.g, acc.b)) < 1e-6 {
        discard;
    }
    return acc;
}
