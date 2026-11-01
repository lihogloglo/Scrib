// GI probe update (docs/specs/gi.md "Update pass"): one workgroup per probe of this frame's
// window, one ray per lane, through Solari's TLAS (group 0, bevy_solari scene/bindings.wesl).
// Hits: sun (one shadow ray) + the light list (point/spot lights, one visibility ray each,
// Bevy's falloff) + emissive + last frame's probes (second bounce, DDGI-weighted from the
// history); misses: the sky environment map. Results blend into DDGI's octahedral tiles
// (Majercik et al. 2019, sf_gi::ddgi): irradiance 6x6 (cosine-weighted mean radiance) and
// visibility 14x14 (mean and mean squared hit distance, clamped), history in storage buffers,
// written with their one-texel borders into the two atlases the PBR shader samples
// (vendored bevy_pbr render/gi_probes.wesl).

import bevy_solari::scene::bindings::{trace_ray, resolve_ray_hit_full, RAY_T_MIN, RAY_T_MAX};
import bevy_solari::scene::sampling::sample_environment_map_light;

enable wgpu_ray_query;

struct GiUniforms {
    // xyz: the grid's min corner (probe 0 sits half a spacing inside); w: spacing.
    origin_spacing: vec4<f32>,
    // xyz: probes per axis; w: rays per probe.
    dims_rays: vec4<u32>,
    // x: first probe of this frame's window, y: count, z: total probes, w: flags.
    window: vec4<u32>,
    // xyz: direction to the sun; w: hysteresis.
    sun_dir_hysteresis: vec4<f32>,
    // rgb: sun irradiance at normal incidence (colour x illuminance); w: hysteresis of probes
    // within a changed light's range.
    sun_irradiance: vec4<f32>,
    // This frame's direction rotation, columns.
    rotation_x: vec4<f32>,
    rotation_y: vec4<f32>,
    rotation_z: vec4<f32>,
    // x: lights in `lights`.
    light_count: vec4<u32>,
    // x: max ray distance (sf_gi::max_ray_distance), y: self-shadow bias (sf_gi::self_shadow_bias).
    ddgi: vec4<f32>,
}

// sf_gi::GiLight::to_gpu: Bevy's GPU light units (colour x lumens / 4pi = candela).
struct GiLight {
    position: vec3<f32>,
    inverse_range_sq: f32,
    color_intensity: vec3<f32>,
    flags: u32,
    spot_direction: vec3<f32>,
    spot_scale: f32,
    spot_offset: f32,
    // Occluders nearer the light than this don't shadow it (shadow_map_near_z).
    near: f32,
    pad: vec2<f32>,
}

@group(1) @binding(0) var<uniform> gi: GiUniforms;
@group(1) @binding(1) var<storage, read> directions: array<vec4<f32>>;
// Interior texels only, per probe: IRR_COUNT (w > 0: has history) and VIS_COUNT.
@group(1) @binding(2) var<storage, read_write> irradiance_history: array<vec4<f32>>;
@group(1) @binding(3) var<storage, read_write> visibility_history: array<vec2<f32>>;
@group(1) @binding(4) var irradiance_atlas: texture_storage_2d<rgba16float, write>;
@group(1) @binding(5) var visibility_atlas: texture_storage_2d<rgba16float, write>;
@group(1) @binding(6) var<storage, read> lights: array<GiLight>;

const PI: f32 = 3.141592653589793;
const FLAG_SUN: u32 = 1u;
const FLAG_BOUNCE: u32 = 2u;
const LIGHT_SPOT: u32 = 1u;
const LIGHT_CHANGED: u32 = 2u;
const MAX_RAYS: u32 = 64u;
// Hit points are pushed off the surface by this much before secondary rays (metres).
const SURFACE_BIAS: f32 = 0.01;
// sf_gi::IRRADIANCE_TEXELS, VISIBILITY_TEXELS (interior, per side), DEPTH_SHARPNESS.
const IRR_TEXELS: u32 = 6u;
const VIS_TEXELS: u32 = 14u;
const IRR_COUNT: u32 = 36u;
const VIS_COUNT: u32 = 196u;
const DEPTH_SHARPNESS: f32 = 50.0;

var<workgroup> sample_dirs: array<vec3<f32>, MAX_RAYS>;
var<workgroup> sample_radiance: array<vec3<f32>, MAX_RAYS>;
var<workgroup> sample_distance: array<f32, MAX_RAYS>;
var<workgroup> tile_irradiance: array<vec3<f32>, IRR_COUNT>;
var<workgroup> tile_visibility: array<vec2<f32>, VIS_COUNT>;
var<workgroup> probe_hysteresis: f32;
var<workgroup> probe_has_history: u32;

fn probe_coord(index: u32) -> vec3<u32> {
    let d = gi.dims_rays.xyz;
    return vec3(index % d.x, (index / d.x) % d.y, index / (d.x * d.y));
}

fn probe_position(c: vec3<u32>) -> vec3<f32> {
    return gi.origin_spacing.xyz + (vec3<f32>(c) + 0.5) * gi.origin_spacing.w;
}

fn sign_not_zero(v: vec2<f32>) -> vec2<f32> {
    return select(vec2(-1.0), vec2(1.0), v >= vec2(0.0));
}

// sf_gi::oct_encode / oct_decode.
fn oct_encode(n: vec3<f32>) -> vec2<f32> {
    let p = n.xy / (abs(n.x) + abs(n.y) + abs(n.z));
    if n.z < 0.0 {
        return (1.0 - abs(p.yx)) * sign_not_zero(p);
    }
    return p;
}

fn oct_decode(p: vec2<f32>) -> vec3<f32> {
    var n = vec3(p, 1.0 - abs(p.x) - abs(p.y));
    if n.z < 0.0 {
        n = vec3((1.0 - abs(n.yx)) * sign_not_zero(n.xy), n.z);
    }
    return normalize(n);
}

// sf_gi::oct_texel_dir: the direction at interior texel t's centre.
fn texel_dir(t: vec2<u32>, inner: u32) -> vec3<f32> {
    return oct_decode((vec2<f32>(t) + 0.5) / f32(inner) * 2.0 - 1.0);
}

// The interior texel (linear, 0-based) holding direction d.
fn dir_texel(d: vec3<f32>, inner: u32) -> u32 {
    let t = min(vec2<u32>((oct_encode(d) * 0.5 + 0.5) * f32(inner)), vec2(inner - 1u));
    return t.x + t.y * inner;
}

// sf_gi::border_source: for bordered-tile texel t (0..inner+2), the bordered-tile texel it
// repeats (itself for an interior texel).
fn border_source(t: vec2<u32>, inner: u32) -> vec2<u32> {
    let last = inner + 1u;
    let on_x = t.x == 0u || t.x == last;
    let on_y = t.y == 0u || t.y == last;
    if on_x && on_y {
        return vec2(select(1u, inner, t.x == 0u), select(1u, inner, t.y == 0u));
    }
    if on_y {
        return vec2(last - t.x, select(inner, 1u, t.y == 0u));
    }
    if on_x {
        return vec2(select(inner, 1u, t.x == 0u), last - t.y);
    }
    return t;
}

// sf_gi::AtlasLayout::tile x tile size: the tile's first texel.
fn tile_origin(c: vec3<u32>, tile_size: u32) -> vec2<u32> {
    return vec2(c.x + gi.dims_rays.x * c.y, c.z) * tile_size;
}

// sf_gi::chebyshev.
fn chebyshev(mean: f32, mean_sq: f32, distance: f32) -> f32 {
    if distance <= mean {
        return 1.0;
    }
    let variance = abs(mean * mean - mean_sq);
    let d = distance - mean;
    let w = variance / (variance + d * d);
    return max(w * w * w, 0.0);
}

// Last frame's probes at a world point along unit normal n (the second bounce): the eight
// surrounding probes, trilinear x smooth back-face x Chebyshev visibility, nearest texels from
// the history (the atlases are write-only here); as the PBR shader's gi_probe_irradiance.
fn previous_irradiance(p: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    let d = gi.dims_rays.xyz;
    // Bias along the normal only (no view direction for a ray hit).
    let biased = p + n * gi.ddgi.y;
    // Probe space: probe (0,0,0) at 0.
    let local = (biased - gi.origin_spacing.xyz) / gi.origin_spacing.w - 0.5;
    if any(local < vec3(-0.5)) || any(local > vec3<f32>(d) - 0.5) {
        return vec3(0.0);
    }
    let clamped = clamp(local, vec3(0.0), vec3<f32>(d - 1u));
    let base = min(vec3<u32>(clamped), d - 1u);
    let alpha = clamped - vec3<f32>(base);
    var sum = vec3(0.0);
    var weight_sum = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let offset = vec3(i & 1u, (i >> 1u) & 1u, (i >> 2u) & 1u);
        let c = min(base + offset, d - 1u);
        let probe = c.x + d.x * (c.y + d.y * c.z);
        if irradiance_history[probe * IRR_COUNT].w <= 0.0 {
            continue;
        }
        let to_point = biased - probe_position(c);
        let distance = max(length(to_point), 0.0001);
        let dir = to_point / distance;
        let tri = mix(1.0 - alpha, alpha, vec3<f32>(offset));
        var w = (dot(-dir, n) + 1.0) * 0.5;
        w = w * w + 0.2;
        let moments = visibility_history[probe * VIS_COUNT + dir_texel(dir, VIS_TEXELS)];
        w *= chebyshev(moments.x, moments.y, distance);
        w = max(w, 0.000001) * tri.x * tri.y * tri.z;
        let irradiance = irradiance_history[probe * IRR_COUNT + dir_texel(n, IRR_TEXELS)].rgb;
        sum += sqrt(max(irradiance, vec3(0.0))) * w;
        weight_sum += w;
    }
    if weight_sum <= 0.0 {
        return vec3(0.0);
    }
    let s = sum / weight_sum;
    return s * s;
}

// Bevy's punctual attenuation, copied from bevy_pbr 0.20 render/pbr_lighting.wesl
// getDistanceAttenuation / getRangeFalloff (sf_gi::distance_attenuation, tested there).
fn distance_attenuation(distance_sq: f32, inverse_range_sq: f32) -> f32 {
    let factor = distance_sq * inverse_range_sq;
    let smooth_factor = saturate(1.0 - factor * factor);
    return smooth_factor * smooth_factor / max(distance_sq, 0.0001);
}

// Irradiance at hit point p (normal n) from the light list: NdotL x attenuation x spot cone
// (pbr_lighting.wesl spot_light: saturate(cd * scale + offset)^2) x a visibility ray that stops
// `near` short of the light (sf_gi::GiLight::irradiance plus visibility).
fn list_irradiance(p: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    var sum = vec3(0.0);
    for (var i = 0u; i < gi.light_count.x; i++) {
        let light = lights[i];
        let to_light = light.position - p;
        let d2 = dot(to_light, to_light);
        if d2 * light.inverse_range_sq >= 1.0 || d2 <= 0.0 || all(light.color_intensity == vec3(0.0)) {
            continue;
        }
        let d = sqrt(d2);
        let l = to_light / d;
        let n_dot_l = dot(n, l);
        if n_dot_l <= 0.0 {
            continue;
        }
        var attenuation = distance_attenuation(d2, light.inverse_range_sq);
        if (light.flags & LIGHT_SPOT) != 0u {
            let a = saturate(dot(light.spot_direction, -l) * light.spot_scale + light.spot_offset);
            attenuation *= a * a;
        }
        if attenuation <= 0.0 {
            continue;
        }
        let t_max = d - light.near;
        if t_max > RAY_T_MIN {
            let shadow = trace_ray(p, l, RAY_T_MIN, t_max, RAY_FLAG_TERMINATE_ON_FIRST_HIT);
            if shadow.kind != RAY_QUERY_INTERSECTION_NONE {
                continue;
            }
        }
        sum += light.color_intensity * (n_dot_l * attenuation);
    }
    return sum;
}

// Whether a changed light's range covers probe position p (sf_gi::probe_hysteresis).
fn near_changed_light(p: vec3<f32>) -> bool {
    for (var i = 0u; i < gi.light_count.x; i++) {
        let light = lights[i];
        let to_light = light.position - p;
        if (light.flags & LIGHT_CHANGED) != 0u && dot(to_light, to_light) * light.inverse_range_sq < 1.0 {
            return true;
        }
    }
    return false;
}

// Radiance arriving at `origin` from direction `dir` (rgb) and the hit distance (w; a miss
// is RAY_T_MAX, clamped by the caller).
fn incoming(origin: vec3<f32>, dir: vec3<f32>) -> vec4<f32> {
    let hit = trace_ray(origin, dir, RAY_T_MIN, RAY_T_MAX, RAY_FLAG_NONE);
    if hit.kind == RAY_QUERY_INTERSECTION_NONE {
        return vec4(sample_environment_map_light(dir), RAY_T_MAX);
    }
    let h = resolve_ray_hit_full(hit);
    // Two-sided until relocation and back-face classification (a later slice).
    var n = h.world_normal;
    if dot(n, dir) > 0.0 {
        n = -n;
    }
    let albedo = h.material.base_color * (1.0 - h.material.metallic);
    let p = h.world_position + n * SURFACE_BIAS;
    var radiance = h.material.emissive;
    if (gi.window.w & FLAG_SUN) != 0u {
        let l = gi.sun_dir_hysteresis.xyz;
        let n_dot_l = dot(n, l);
        if n_dot_l > 0.0 {
            let shadow = trace_ray(p, l, RAY_T_MIN, RAY_T_MAX, RAY_FLAG_TERMINATE_ON_FIRST_HIT);
            if shadow.kind == RAY_QUERY_INTERSECTION_NONE {
                radiance += albedo / PI * gi.sun_irradiance.rgb * n_dot_l;
            }
        }
    }
    // Lambert: albedo / pi x irradiance.
    radiance += albedo / PI * list_irradiance(p, n);
    if (gi.window.w & FLAG_BOUNCE) != 0u {
        // Probes hold irradiance / pi: Lambertian exitance is albedo x probe value.
        radiance += albedo * previous_irradiance(p, n);
    }
    return vec4(radiance, hit.t);
}

@compute @workgroup_size(64, 1, 1)
fn update(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    let total = gi.window.z;
    let probe = (gi.window.x + group.x) % total;
    let c = probe_coord(probe);
    let rays = min(gi.dims_rays.w, MAX_RAYS);
    let max_distance = gi.ddgi.x;
    if lane == 0u {
        var h = gi.sun_dir_hysteresis.w;
        if near_changed_light(probe_position(c)) {
            h = min(h, gi.sun_irradiance.w);
        }
        probe_hysteresis = h;
        probe_has_history = select(0u, 1u, irradiance_history[probe * IRR_COUNT].w > 0.0);
    }
    if lane < rays {
        let rotation = mat3x3(gi.rotation_x.xyz, gi.rotation_y.xyz, gi.rotation_z.xyz);
        let dir = normalize(rotation * directions[lane].xyz);
        sample_dirs[lane] = dir;
        let result = incoming(probe_position(c), dir);
        var radiance = result.rgb;
        // Sanity: no NaN or infinity reaches the history.
        if !all(radiance == radiance) || any(abs(radiance) > vec3(65000.0)) {
            radiance = vec3(0.0);
        }
        sample_radiance[lane] = radiance;
        sample_distance[lane] = clamp(result.w, 0.0, max_distance);
    }
    // Both read only after the barrier inside workgroupUniformLoad.
    let hysteresis = workgroupUniformLoad(&probe_hysteresis);
    let has_history = workgroupUniformLoad(&probe_has_history) != 0u;
    // Irradiance texels: cosine-weighted mean radiance (sf_gi::irradiance_texel), blended
    // (sf_gi::hysteresis_blend).
    if lane < IRR_COUNT {
        let texel = texel_dir(vec2(lane % IRR_TEXELS, lane / IRR_TEXELS), IRR_TEXELS);
        var sum = vec3(0.0);
        var weight = 0.0;
        for (var i = 0u; i < rays; i++) {
            let w = max(dot(texel, sample_dirs[i]), 0.0);
            sum += sample_radiance[i] * w;
            weight += w;
        }
        var value = vec3(0.0);
        if weight > 0.0 {
            value = sum / weight;
        }
        let slot = probe * IRR_COUNT + lane;
        if has_history {
            value = mix(value, irradiance_history[slot].rgb, hysteresis);
        }
        irradiance_history[slot] = vec4(value, 1.0);
        tile_irradiance[lane] = value;
    }
    // Visibility texels: mean and mean squared hit distance under a sharp cosine lobe.
    for (var v = lane; v < VIS_COUNT; v += 64u) {
        let texel = texel_dir(vec2(v % VIS_TEXELS, v / VIS_TEXELS), VIS_TEXELS);
        var sum = vec2(0.0);
        var weight = 0.0;
        for (var i = 0u; i < rays; i++) {
            let w = pow(max(dot(texel, sample_dirs[i]), 0.0), DEPTH_SHARPNESS);
            let d = sample_distance[i];
            sum += vec2(d, d * d) * w;
            weight += w;
        }
        let slot = probe * VIS_COUNT + v;
        var value = vec2(max_distance, max_distance * max_distance);
        if has_history {
            value = visibility_history[slot];
        }
        if weight > 0.0 {
            let new_value = sum / weight;
            value = select(new_value, mix(new_value, value, hysteresis), has_history);
        }
        visibility_history[slot] = value;
        tile_visibility[v] = value;
    }
    workgroupBarrier();
    // Both bordered tiles into the atlases (sf_gi::AtlasLayout::texel, border_source).
    let irr_size = IRR_TEXELS + 2u;
    let irr_origin = tile_origin(c, irr_size);
    for (var i = lane; i < irr_size * irr_size; i += 64u) {
        let t = vec2(i % irr_size, i / irr_size);
        let s = border_source(t, IRR_TEXELS) - 1u;
        textureStore(irradiance_atlas, irr_origin + t, vec4(tile_irradiance[s.x + s.y * IRR_TEXELS], 1.0));
    }
    let vis_size = VIS_TEXELS + 2u;
    let vis_origin = tile_origin(c, vis_size);
    for (var i = lane; i < vis_size * vis_size; i += 64u) {
        let t = vec2(i % vis_size, i / vis_size);
        let s = border_source(t, VIS_TEXELS) - 1u;
        textureStore(visibility_atlas, vis_origin + t, vec4(tile_visibility[s.x + s.y * VIS_TEXELS], 0.0, 1.0));
    }
}
