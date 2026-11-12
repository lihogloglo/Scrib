// Froxel injection (weather.md "Fog", F3, steps 1-3): per froxel, the fog's
// density (height layer + ground-fog banks, the same terms as the analytic
// fog) and the light it scatters toward the camera: the key light (sun or
// moon) × its CSM shadow × the terrain's shadow-height map × the cloud
// shadow map with the fog's phase lobe, the sky fill, and the nearest point
// lights. A sun shaft is a froxel lit through a gap. Then temporal
// reprojection: a jittered sample per frame, blended with last frame's
// volume (Wronski 2014, Hillaire 2015), the history clipped to this frame's
// 3×3 neighbourhood (FOG-CLAMP; Karis 2014).

import bevy_pbr::render::mesh_view_types::{Lights, DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT};
import super::fog::{MAX_EXPONENT, fog_phase};
import super::fog_bank::fog_bank_mask;
import super::froxel_common::{Froxels, slice_distance, slice_coord, froxel_ray};

@group(0) @binding(0) var<uniform> fx: Froxels;
@group(0) @binding(1) var<uniform> lights: Lights;
// `fog_light.wesl`: sky fill (texel 0), key light seen (1), its direction (2).
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

// sf_lod_bevy's far_shadow_map.wesl (keep in step): penumbra per meter of
// occluder distance and its floor, bias and its growth per meter of view
// distance, where the border fade starts.
const FAR_PENUMBRA: f32 = 0.012;
const FAR_PENUMBRA_MIN: f32 = 2.0;
const FAR_BIAS: f32 = 6.0;
const FAR_BIAS_PER_M: f32 = 0.0015;
const FAR_FADE_START: f32 = 0.85;

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

// A froxel's fixed offset, 0..1 per axis: pcg3d (Jarzynski and Olano 2020).
// CPU twin: `sf_atmos::froxel::froxel_hash` (keep in step).
fn froxel_hash(cell: vec3<u32>) -> vec3<f32> {
    var v = cell * 1664525u + 1013904223u;
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    v ^= v >> vec3(16u);
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    return vec3<f32>(v >> vec3(8u)) * (1.0 / 16777216.0);
}

// `Froxels::flags` bits (render.rs `FroxelUniform`).
// Per-froxel sample rotation, history read at the froxel centre (`FroxelJitter::PerFroxel`).
const FLAG_PER_FROXEL: u32 = 1u;
// Clip the history to this frame's 3×3 neighbourhood (FOG-CLAMP, `FroxelSettings::clamp`).
const FLAG_CLAMP: u32 = 2u;

// Threads per workgroup along x and y: `sf_atmos::froxel::INJECT_GROUP` (keep
// in step). Each group samples a GROUP² block of one slice into `current` and
// writes its inner TILE², so every written froxel has its 3×3 at hand.
const GROUP: u32 = 16u;
const TILE: u32 = GROUP - 2u;
var<workgroup> current: array<vec4<f32>, 256>;

// Clips `prev` to the box `lo..hi` along the line to its centre (Karis 2014).
// CPU twin: `sf_atmos::froxel::clip_history` (keep in step).
fn clip_history(prev: vec4<f32>, lo: vec4<f32>, hi: vec4<f32>) -> vec4<f32> {
    let c = 0.5 * (hi + lo);
    let a = abs(prev - c) / (0.5 * (hi - lo) + 1e-7);
    let m = max(1.0, max(max(a.x, a.y), max(a.z, a.w)));
    return c + (prev - c) / m;
}

@compute @workgroup_size(16, 16, 1)
fn inject(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_id) local: vec3<u32>) {
    let size = vec3<u32>(fx.size);
    // This thread's froxel: the tile plus a one-froxel apron, clamped into the
    // grid (edge froxels repeat). CPU twin: `sf_atmos::froxel::inject_cell`.
    let raw = vec2<i32>(group.xy * TILE + local.xy) - 1;
    let id = vec3(vec2<u32>(clamp(raw, vec2(0), vec2<i32>(size.xy) - 1)), group.z);
    var value = sample_froxel(id);
    current[local.y * GROUP + local.x] = value;
    workgroupBarrier();
    if any(local.xy < vec2(1u)) || any(local.xy > vec2(TILE)) || any(raw >= vec2<i32>(size.xy)) {
        return;
    }

    // Temporal reprojection: where this froxel was in last frame's volume. The
    // history holds the froxel's mean, so it is read at the froxel's centre
    // (a still camera reads texel centres: no drift, no blur), not at the
    // jittered sample, which would shift the whole history by the jitter.
    // Off screen or past the volume: this frame's sample alone.
    if fx.history_valid != 0u && fx.history > 0.0 {
        let mid = vec3<f32>(id) + select(fx.jitter, vec3(0.5), (fx.flags & FLAG_PER_FROXEL) != 0u);
        let q = fx.camera + froxel_ray(fx, mid.xy / fx.size.xy) * slice_distance(fx, mid.z / fx.size.z);
        let pp = q + fx.origin_shift;
        let clip = fx.prev_clip_from_world * vec4(pp, 1.0);
        if clip.w > 0.0 {
            let puv = clip.xy / clip.w * vec2(0.5, -0.5) + 0.5;
            let pd = length(pp - fx.prev_camera);
            if all(puv >= vec2(0.0)) && all(puv <= vec2(1.0)) && pd < fx.far {
                var prev = textureSampleLevel(history, linear_sampler, vec3(puv, slice_coord(fx, pd)), 0.0);
                // FOG-CLAMP: history the neighbourhood's samples don't allow
                // (a moved shadow, a light gone, a disocclusion) is clipped
                // to their box (UE4 TAA, Karis 2014), so stale light can't
                // trail and the history can weigh more.
                if (fx.flags & FLAG_CLAMP) != 0u {
                    var lo = value;
                    var hi = value;
                    for (var dy = 0u; dy < 3u; dy += 1u) {
                        for (var dx = 0u; dx < 3u; dx += 1u) {
                            let n = current[(local.y + dy - 1u) * GROUP + local.x + dx - 1u];
                            lo = min(lo, n);
                            hi = max(hi, n);
                        }
                    }
                    prev = clip_history(prev, lo, hi);
                }
                value = mix(value, prev, fx.history);
            }
        }
    }
    textureStore(volume, vec3<i32>(id), value);
}

// This frame's (in-scatter, extinction) of froxel `id`.
fn sample_froxel(id: vec3<u32>) -> vec4<f32> {
    // This frame's sample inside the froxel (weather.md "Temporal sampling"):
    // per froxel, the frame's R3 offset rotated by the froxel's hash, so
    // neighbours err independently (noise the history and TAA average) instead
    // of in step (a pulse of the whole volume).
    var j = fx.jitter;
    if (fx.flags & FLAG_PER_FROXEL) != 0u {
        j = fract(j + froxel_hash(id));
    }
    let cell = vec3<f32>(id) + j;
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
        let sky_fill = textureLoad(fog_light, vec2(0, 0), 0).rgb;
        let key_seen = textureLoad(fog_light, vec2(1, 0), 0).rgb;
        let key_dir = textureLoad(fog_light, vec2(2, 0), 0).xyz;
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
        scatter = f.albedo * sigma * (key + sky_fill * f.ambient + local) * fx.exposure;
    }
    return vec4(scatter, sigma);
}
