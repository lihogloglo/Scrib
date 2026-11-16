// Kick-up (surface-weather.md W7): puffs of snow and ash where a foot plants. One draw of a
// fixed mesh of BURSTS × SPRITES quads; the CPU writes only the bursts (position, age, facing,
// strength, kind, seed). A sprite's flight is closed-form, the twin of `sf_surface::kickup`
// (`sprite_at`: seeds, timing, launch, arc, floor clamp, fade, size).
//
// Mesh: four vertices per sprite. @location(0) the sprite's seeds in [0, 1)³, @location(1) the
// corner in [-1, 1]², @location(2) (burst slot, sprite index, 0, 0).
//
// Lit by Bevy's own `apply_pbr_lighting`, as the opaque surfaces and the NIF particles are (SW-7;
// not a fixed brightness: precipitation's night look, WE-9 cause 3): a rough dielectric over a
// fake sphere normal from the sprite corner, so the directional lights carry the atmosphere's
// transmittance and visible sun disc (a set sun adds nothing), their cascades are filtered as on
// the ground, and the clustered point lights, the ambient floor, the environment map and the GI
// probes all count, × the view's exposure.
// Soft against the depth prepass where there is one; fogged like any translucent (RO-3).

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND};
import bevy_pbr::render::{
    mesh_view_bindings::view,
    mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT,
    pbr_types::{PbrInput, pbr_input_new, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND},
    pbr_functions::{apply_pbr_lighting, calculate_view},
    view_transformations::{position_world_to_clip, depth_ndc_to_view_z},
};

const TAU: f32 = 6.2831853;
// Powder: fully rough, a faint dielectric sheen (F0 = 0.16 · 0.2² ≈ 0.6 %).
const PUFF_REFLECTANCE: f32 = 0.2;
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
    // The sprite's centre and radius (the lit sphere).
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

// The sprite's frame facing the camera along `f`: (right, up, f).
fn facing(f: vec3<f32>) -> mat3x3<f32> {
    let up_ref = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, -1.0), abs(f.y) > 0.999);
    let right = normalize(cross(up_ref, f));
    return mat3x3<f32>(right, cross(f, right), f);
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
    let basis = facing(to_cam / max(dist, 1e-3));
    let q = p + (basis[0] * v.corner.x + basis[1] * v.corner.y) * r;
    out.position = position_world_to_clip(q);
    out.albedo_alpha = vec4<f32>(kind.color.rgb, alpha);
    out.center = vec4<f32>(p, r);
    out.dist = distance(q, cam);
    return out;
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
    // A ball of powder: the sphere's normal under this pixel and the point on it, lit as the
    // ground is (pre-exposure in, × exposure out).
    let basis = facing(normalize(view.world_position - in.center.xyz));
    let n = normalize(basis[0] * in.corner.x + basis[1] * in.corner.y + basis[2] * sqrt(1.0 - r2));
    let at = vec4<f32>(in.center.xyz + n * in.center.w, 1.0);
    var pbr: PbrInput = pbr_input_new();
    pbr.material.base_color = vec4<f32>(in.albedo_alpha.rgb, 1.0);
    pbr.material.perceptual_roughness = 1.0;
    pbr.material.metallic = 0.0;
    pbr.material.reflectance = vec3<f32>(PUFF_REFLECTANCE);
    pbr.material.flags = STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND;
    pbr.frag_coord = in.position;
    pbr.world_position = at;
    pbr.world_normal = n;
    pbr.N = n;
    pbr.is_orthographic = view.clip_from_view[3].w == 1.0;
    pbr.V = calculate_view(at, pbr.is_orthographic);
    // The draw casts no shadow (`NotShadowCaster`) but receives: the cascades, the lamps' cubes.
    pbr.flags = MESH_FLAGS_SHADOW_RECEIVER_BIT;
    let rgb = apply_pbr_lighting(pbr).rgb;
    // Fogged like any translucent (RO-3).
    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, in.dist);
    return froxel_fog_apply(vec4<f32>(rgb, alpha), fog, FOG_BLEND);
}
