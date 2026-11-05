// Kick-up (surface-weather.md W7): puffs of snow and ash where a foot plants. One draw of a
// fixed mesh of BURSTS × SPRITES quads; the CPU writes only the bursts (position, age, facing,
// strength, kind, seed). A sprite's flight is closed-form, the twin of `sf_surface::kickup`
// (`sprite_at`: seeds, timing, launch, arc, floor clamp, fade, size).
//
// Mesh: four vertices per sprite. @location(0) the sprite's seeds in [0, 1)³, @location(1) the
// corner in [-1, 1]², @location(2) (burst slot, sprite index, 0, 0).
//
// Lit as the surfaces are (not a fixed brightness: precipitation's night look, WE-9 cause 3):
// the view's directional lights (sun, moon) with one shadow tap at the sprite, the flat ambient
// (the night floor) and the sky's diffuse environment map, all × the view's exposure. The mesh
// view's light bindings are fragment-only, so the light is evaluated per pixel at the sprite's
// centre (flat inputs: the same value over the sprite). Point lights are not summed (logged).
// Soft against the depth prepass where there is one; fogged like any translucent (RO-3).

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND};
import bevy_render::maths::quat_rotate;
import bevy_pbr::render::{
    mesh_view_bindings as bindings,
    mesh_view_bindings::{view, lights, light_probes},
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
    shadows::{get_cascade_index, world_to_directional_light_local},
    shadow_sampling::sample_shadow_map_hardware,
    view_transformations::{position_world_to_clip, depth_ndc_to_view_z},
};

const TAU: f32 = 6.2831853;
const INV_PI: f32 = 0.31830989;
// `sf_surface::kickup::SEED_STEP`.
const SEED_STEP: vec3<f32> = vec3<f32>(0.618034, 0.7548777, 0.5698403);
// `sf_surface::kickup::FLOOR_BELOW_M`.
const FLOOR_BELOW_M: f32 = 0.05;
// Bursts fade out over the last metres of `sf_surface::kickup::RANGE_M`.
const RANGE_M: f32 = 30.0;
// Soft-particle depth range (m).
const SOFT_M: f32 = 0.12;

// `sf_surface::kickup::KickLook`.
struct KickKind {
    // life s, drag 1/s, gravity m/s², wind follow
    motion: vec4<f32>,
    // speed m/s, up m/s, forward bias, delay s
    launch: vec4<f32>,
    // radius at birth m, at death m, 0, 0
    size: vec4<f32>,
    // albedo rgb (linear), peak opacity
    color: vec4<f32>,
}

struct KickBurst {
    // render xyz, age s (< 0: empty slot)
    a: vec4<f32>,
    // forward x, z; strength; kind
    b: vec4<f32>,
    // seed, 0, 0, 0
    c: vec4<f32>,
}

struct KickupParams {
    kinds: array<KickKind, 2>,
    // xyz: the ground wind (render space, m/s), w: 0
    wind: vec4<f32>,
    bursts: array<KickBurst, 64>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> kick: KickupParams;
// `TranslucentFog`: (near, far, slices, enabled), the integrated volume.
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var<uniform> fog_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var fog_volume: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var fog_sampler: sampler;

struct Vertex {
    @location(0) seed: vec3<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) kick: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) corner: vec2<f32>,
    @location(1) @interpolate(flat) albedo_alpha: vec4<f32>,
    // The sprite's centre (lighting, shadow) and radius.
    @location(2) @interpolate(flat) center: vec4<f32>,
    @location(3) dist: f32,
}

// `sf_surface::kickup::arc`: linear drag toward the wind under gravity, exact.
fn arc(p0: vec3<f32>, v0: vec3<f32>, wind: vec3<f32>, k_in: f32, g: f32, t: f32) -> vec3<f32> {
    let k = max(k_in, 1e-3);
    let v_inf = wind - vec3<f32>(0.0, g / k, 0.0);
    return p0 + v_inf * t + (v0 - v_inf) * ((1.0 - exp(-k * t)) / k);
}

// `sf_surface::kickup::launch`.
fn launch(kind: KickKind, forward: vec2<f32>, strength: f32, s: vec3<f32>) -> vec3<f32> {
    let az = TAU * s.x;
    let h = vec2<f32>(cos(az), sin(az)) + forward * kind.launch.z;
    let hs = kind.launch.x * strength * (0.35 + 0.65 * s.y);
    let vs = kind.launch.y * strength * (0.45 + 0.55 * s.z);
    return vec3<f32>(h.x * hs, vs, h.y * hs);
}

// `sf_surface::kickup::fade`.
fn fade(u: f32) -> f32 {
    if u < 0.0 || u > 1.0 {
        return 0.0;
    }
    return smoothstep(0.0, 0.06, u) * (1.0 - smoothstep(0.35, 1.0, u));
}

// `sf_surface::kickup::size`.
fn sprite_size(kind: KickKind, u: f32, s: vec3<f32>) -> f32 {
    let uc = clamp(u, 0.0, 1.0);
    return (kind.size.x + (kind.size.y - kind.size.x) * sqrt(uc)) * (0.7 + 0.6 * s.y);
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.corner = v.corner;
    out.albedo_alpha = vec4<f32>(0.0);
    out.center = vec4<f32>(0.0);
    out.dist = 0.0;
    // Off: a degenerate position, nothing rasterizes.
    out.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    let burst = kick.bursts[min(u32(v.kick.x), 63u)];
    let age = burst.a.w;
    if age < 0.0 {
        return out;
    }
    let kind = kick.kinds[min(u32(burst.b.w), 1u)];
    let s = fract(v.seed + burst.c.x * SEED_STEP);
    let delay = kind.launch.w * s.z;
    let life = kind.motion.x * (0.75 + 0.5 * s.x);
    let t = age - delay;
    if t < 0.0 || t > life {
        return out;
    }
    let u = t / life;
    let p0 = burst.a.xyz;
    let strength = burst.b.z;
    let r = sprite_size(kind, u, s);
    let v0 = launch(kind, burst.b.xy, strength, s);
    var p = arc(p0, v0, kick.wind.xyz * kind.motion.w, kind.motion.y, kind.motion.z, t);
    p.y = max(p.y, p0.y - FLOOR_BELOW_M + 0.5 * r);
    let cam = view.world_position;
    let to_cam = cam - p;
    let dist = length(to_cam);
    let fade_d = smoothstep(0.25, 0.6, dist) * (1.0 - smoothstep(0.8 * RANGE_M, RANGE_M, dist));
    let alpha = kind.color.a * strength * fade(u) * fade_d;
    if alpha < 0.002 {
        return out;
    }
    let f = to_cam / max(dist, 1e-3);
    let up_ref = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, -1.0), abs(f.y) > 0.999);
    let right = normalize(cross(up_ref, f));
    let up = cross(f, right);
    let q = p + (right * v.corner.x + up * v.corner.y) * r;
    out.position = position_world_to_clip(q);
    out.albedo_alpha = vec4<f32>(kind.color.rgb, alpha);
    out.center = vec4<f32>(p, r);
    out.dist = distance(q, cam);
    return out;
}

// The sky's diffuse light toward `n` (the view's environment map), as `bevy_pbr`'s
// `environment_map_light` reads it for the view's own cubemap.
fn sky_irradiance(n: vec3<f32>) -> vec3<f32> {
    var e = vec3<f32>(0.0);
    @if(ENVIRONMENT_MAP) {
        let ix = light_probes.view_cubemap_index;
        if ix >= 0 {
            var d = quat_rotate(light_probes.view_rotation, n);
            // Cube maps are left-handed.
            d.z = -d.z;
            @if(MULTIPLE_LIGHT_PROBES_IN_ARRAY)
            e = textureSampleLevel(bindings::diffuse_environment_maps[ix], bindings::environment_map_sampler, d, 0.0).rgb;
            @else
            e = textureSampleLevel(bindings::diffuse_environment_map, bindings::environment_map_sampler, d, 0.0).rgb;
            e *= light_probes.intensity_for_view;
        }
    }
    return e;
}

// One hardware PCF tap of directional light `i`'s cascades at `p` (1: lit).
fn sun_shadow(i: u32, p: vec3<f32>) -> f32 {
    let light = &lights.directional_lights[i];
    if ((*light).flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) == 0u {
        return 1.0;
    }
    let view_z = (view.view_from_world * vec4<f32>(p, 1.0)).z;
    let cascade = get_cascade_index(i, view_z);
    if cascade >= (*light).num_cascades {
        return 1.0;
    }
    // Biased toward the light (as the surfaces' depth bias), a little more for the puff's size.
    let biased = p + (*light).direction_to_light * ((*light).shadow_depth_bias + 0.05);
    let local = world_to_directional_light_local(i, cascade, vec4<f32>(biased, 1.0));
    if local.w == 0.0 {
        return 1.0;
    }
    return sample_shadow_map_hardware(local.xy, local.z, i32((*light).depth_texture_base_index + cascade));
}

// Radiance per unit albedo of a puff at `p` seen along `-f` (pre-exposure): the directional
// lights over a diffuse sphere (Lambert's phase: brighter seen from the lit side), the ambient
// floor, the sky.
fn puff_light(p: vec3<f32>, f: vec3<f32>) -> vec3<f32> {
    var l = vec3<f32>(0.0);
    for (var i = 0u; i < lights.n_directional_lights; i = i + 1u) {
        let light = &lights.directional_lights[i];
        let phase = 0.5 + 0.25 * dot((*light).direction_to_light, f);
        l += (*light).color.rgb * (INV_PI * phase * sun_shadow(i, p));
    }
    l += lights.ambient_color.rgb;
    l += 0.5 * (sky_irradiance(vec3<f32>(0.0, 1.0, 0.0)) + sky_irradiance(f));
    return l;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let r2 = dot(in.corner, in.corner);
    if r2 >= 1.0 {
        discard;
    }
    // A soft ball of powder.
    let shape = (1.0 - r2) * (1.0 - r2);
    var alpha = in.albedo_alpha.a * shape;
    // Soft against the scene: fades where the sprite meets the ground or a wall.
    @if(DEPTH_PREPASS) {
        let scene_z = depth_ndc_to_view_z(bevy_pbr::prepass::utils::prepass_depth(in.position, 0u));
        let own_z = depth_ndc_to_view_z(in.position.z);
        alpha *= clamp((own_z - scene_z) / SOFT_M, 0.0, 1.0);
    }
    if alpha < 0.002 {
        discard;
    }
    let cam = view.world_position;
    let f = normalize(cam - in.center.xyz);
    let rgb = in.albedo_alpha.rgb * puff_light(in.center.xyz, f) * view.exposure;
    // Fogged like any translucent (RO-3).
    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, in.dist);
    return froxel_fog_apply(vec4<f32>(rgb, alpha), fog, FOG_BLEND);
}
