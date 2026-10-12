// Froxel injection (weather.md "Fog", F3, steps 1-3): per froxel, the fog's
// density (height layer + ground-fog banks, the same terms as the analytic
// fog) and the light it scatters toward the camera: the key light (sun or
// moon) × its CSM shadow × the terrain's shadow-height map × the cloud
// shadow map with the fog's phase lobe, the sky fill, and the nearest point
// lights. A sun shaft is a froxel lit through a gap. Then temporal
// reprojection: a jittered sample per frame, blended with last frame's
// volume (Wronski 2014, Hillaire 2015).

import bevy_pbr::render::mesh_view_types::{Lights, DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT};
import super::fog::{MAX_EXPONENT, fog_phase};
import super::fog_bank::fog_bank_mask;
import super::froxel_common::{Froxels, slice_distance, slice_coord, froxel_ray};

@group(0) @binding(0) var<uniform> fx: Froxels;
@group(0) @binding(1) var<uniform> lights: Lights;
// `fog_light.wesl`: sky fill per azimuth (0-63), key light seen (64), its direction (65).
@group(0) @binding(2) var fog_light: texture_2d<f32>;
@group(0) @binding(3) var weather_texture: texture_2d<f32>;
@group(0) @binding(4) var noise_sampler: sampler;
@group(0) @binding(5) var cloud_shadow: texture_2d<f32>;
@group(0) @binding(6) var far_map: texture_2d<f32>;
@group(0) @binding(7) var linear_sampler: sampler;
@group(0) @binding(8) var shadow_maps: texture_depth_2d_array;
@group(0) @binding(9) var shadow_sampler: sampler_comparison;
@group(0) @binding(10) var history: texture_3d<f32>;
@group(0) @binding(11) var volume: texture_storage_3d<rgba16float, write>;

const AZIMUTHS: i32 = 64;
const PI: f32 = 3.14159265;
// sf_lod_bevy's far_shadow_map.wesl (keep in step): penumbra per meter of
// occluder distance and its floor, bias and its growth per meter of view
// distance, where the border fade starts.
const FAR_PENUMBRA: f32 = 0.012;
const FAR_PENUMBRA_MIN: f32 = 2.0;
const FAR_BIAS: f32 = 6.0;
const FAR_BIAS_PER_M: f32 = 0.0015;
const FAR_FADE_START: f32 = 0.85;

// Sky fill toward the horizon under `rd` (the composite's lookup).
fn sky_fill(rd: vec3<f32>) -> vec3<f32> {
    let a = (atan2(rd.z, rd.x) / (2.0 * PI) + 1.0) * f32(AZIMUTHS) - 0.5;
    let a0 = floor(a);
    let i0 = (i32(a0) % AZIMUTHS + AZIMUTHS) % AZIMUTHS;
    let i1 = (i0 + 1) % AZIMUTHS;
    return mix(textureLoad(fog_light, vec2(i0, 0), 0).rgb, textureLoad(fog_light, vec2(i1, 0), 0).rgb, a - a0);
}

// The key light's CSM visibility at `p` (Bevy's cascades, hard compare: the
// froxel is far coarser than a texel). `fwd`: distance along the view axis.
fn csm_visibility(p: vec3<f32>, dir: vec3<f32>, fwd: f32) -> f32 {
    var best = 0.999;
    var li = 0xffffffffu;
    for (var i = 0u; i < lights.n_directional_lights; i += 1u) {
        let c = dot(lights.directional_lights[i].direction_to_light.xyz, dir);
        if c > best {
            best = c;
            li = i;
        }
    }
    if li == 0xffffffffu || (lights.directional_lights[li].flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) == 0u {
        return 1.0;
    }
    let n = lights.directional_lights[li].num_cascades;
    for (var c = 0u; c < n; c += 1u) {
        if fwd < lights.directional_lights[li].cascades[c].far_bound {
            let bias = lights.directional_lights[li].shadow_depth_bias * dir;
            let clip = lights.directional_lights[li].cascades[c].clip_from_world * vec4(p + bias, 1.0);
            let ndc = clip.xyz / clip.w;
            if any(abs(ndc.xy) > vec2(1.0)) || ndc.z < 0.0 || ndc.z > 1.0 {
                return 1.0;
            }
            let uv = ndc.xy * vec2(0.5, -0.5) + 0.5;
            let layer = i32(lights.directional_lights[li].depth_texture_base_index + c);
            return textureSampleCompareLevel(shadow_maps, shadow_sampler, uv, layer, ndc.z);
        }
    }
    return 1.0;
}

// The terrain's shadow-height map (sf_lod_bevy `map_visibility`'s rule: a
// penumbra widening with occluder distance, a bias growing with view
// distance, a fade toward the map's border).
fn far_visibility(p: vec3<f32>) -> f32 {
    let far = fx.far_map;
    if far.z <= 0.0 {
        return 1.0;
    }
    let uv = (p.xz - far.xy) * far.z;
    let edge = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    if edge >= 1.0 {
        return 1.0;
    }
    let t = textureSampleLevel(far_map, linear_sampler, uv, 0.0).rg;
    let hs = t.x + far.w;
    let w = t.y * FAR_PENUMBRA + FAR_PENUMBRA_MIN;
    let y = p.y + FAR_BIAS + distance(fx.camera, p) * FAR_BIAS_PER_M;
    return mix(smoothstep(hs - w, hs + w, y), 1.0, smoothstep(FAR_FADE_START, 1.0, edge));
}

// The cloud shadow map (toroidal; cloud_shadow.wesl's basis).
fn cloud_visibility(p: vec3<f32>) -> f32 {
    if fx.cloud_shadow == 0u {
        return 1.0;
    }
    let o = p - fx.shadow_center;
    let local = vec2(dot(o, fx.shadow_x) / dot(fx.shadow_x, fx.shadow_x), dot(o, fx.shadow_y) / dot(fx.shadow_y, fx.shadow_y));
    let uv = local * vec2(-0.5, 0.5) + 0.5;
    return textureSampleLevel(cloud_shadow, noise_sampler, uv, 0.0).r;
}

@compute @workgroup_size(8, 8, 1)
fn inject(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec3<u32>(fx.size);
    if any(id >= size) {
        return;
    }
    let cell = vec3<f32>(id) + fx.jitter;
    let uv = cell.xy / fx.size.xy;
    let rd = froxel_ray(fx, uv);
    let d = slice_distance(fx, cell.z / fx.size.z);
    let p = fx.camera + rd * d;
    let h = length(p - fx.planet_center) - fx.inner_radius;

    let f = fx.fog;
    var sigma = f.height_density * exp(min(-(h - f.height_base) / f.height_falloff, MAX_EXPONENT));
    let bank = f.bank_density * exp(min(-(h - f.bank_base) / f.bank_falloff, MAX_EXPONENT));
    if bank > 1e-6 {
        sigma += bank * fog_bank_mask(f, weather_texture, noise_sampler, p);
    }

    var scatter = vec3(0.0);
    if sigma > 1e-7 {
        let key_seen = textureLoad(fog_light, vec2(AZIMUTHS, 0), 0).rgb;
        let key_dir = textureLoad(fog_light, vec2(AZIMUTHS + 1, 0), 0).xyz;
        var key = vec3(0.0);
        if max(key_seen.r, max(key_seen.g, key_seen.b)) > 0.0 {
            var vis = csm_visibility(p, key_dir, dot(p - fx.camera, fx.forward));
            // The cloud and terrain maps are made for the shadowed light (the sun).
            if dot(key_dir, fx.shadow_light) > 0.99 {
                vis = min(vis, far_visibility(p)) * cloud_visibility(p);
            }
            key = key_seen * fog_phase(f.phase_g, dot(rd, key_dir)) * vis;
        }
        var local = vec3(0.0);
        for (var i = 0u; i < fx.n_lights; i += 1u) {
            let l = fx.lights[i];
            let to = l.position_range.xyz - p;
            let d2 = dot(to, to);
            let r2 = l.position_range.w * l.position_range.w;
            if d2 < r2 {
                let x = d2 / r2;
                let fade = saturate(1.0 - x * x);
                local += l.color.rgb * (fade * fade / max(d2, 0.25)) * fog_phase(f.phase_g, dot(rd, to * inverseSqrt(max(d2, 1e-6))));
            }
        }
        scatter = f.albedo * sigma * (key + sky_fill(rd) * f.ambient + local) * fx.exposure;
    }
    var value = vec4(scatter, sigma);

    // Temporal reprojection: where this froxel's point was in last frame's volume.
    if fx.history_valid != 0u && fx.history > 0.0 {
        let pp = p + fx.origin_shift;
        let clip = fx.prev_clip_from_world * vec4(pp, 1.0);
        if clip.w > 0.0 {
            let puv = clip.xy / clip.w * vec2(0.5, -0.5) + 0.5;
            let pd = length(pp - fx.prev_camera);
            if all(puv >= vec2(0.0)) && all(puv <= vec2(1.0)) && pd < fx.far {
                let prev = textureSampleLevel(history, linear_sampler, vec3(puv, slice_coord(fx, pd)), 0.0);
                value = mix(value, prev, fx.history);
            }
        }
    }
    textureStore(volume, vec3<i32>(id), value);
}
