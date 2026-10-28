// Precipitation (weather.md "Precipitation"): one nested camera-centered
// box per draw. A drop's position is closed-form: its seed spread over the
// box, moved by the integrated velocity (CPU, wrapped), wrapped around the
// camera. No simulation. Streaks stretch along the velocity and face the
// camera; flakes are soft discs that flutter. A minimum width in pixels
// keeps thin drops visible far off (dimmed to keep their energy).
//
// Mesh: four vertices per drop. @location(0) seed in [0, 1)³,
// @location(1) corner in [-1, 1]², @location(2) (rank, size t, phase, 0):
// a drop shows while rank < intensity.

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND};
import bevy_pbr::render::{
    mesh_view_bindings::view,
    view_transformations::position_world_to_clip,
};

struct PrecipParams {
    // x: box size m, y: inner half-size m (nested; 0 none), z: streak
    // length s, w: shape (0 streak, 1 flake)
    volume: vec4<f32>,
    // xyz: integrated velocity offset (wrapped), w: time s (wrapped)
    offset: vec4<f32>,
    // xyz: velocity m/s, w: flutter amplitude m
    velocity: vec4<f32>,
    // x, y: width range m, z: intensity, w: minimum width px
    look: vec4<f32>,
    // rgb: linear color (lit), a: opacity
    color: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> precip: PrecipParams;
// `TranslucentFog`: (near, far, slices, enabled), the integrated volume.
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var<uniform> fog_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var fog_volume: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var fog_sampler: sampler;

// Occlusion map along the drops' travel (reverse Z, 1 at the upstream plane;
// top-down for falling rain): clip_from_world, (valid, depth range m, 0, 0),
// the depth texture.
@group(constants::MATERIAL_BIND_GROUP) @binding(4) var<uniform> occ_clip: mat4x4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(5) var<uniform> occ_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(6) var occ_map: texture_depth_2d;

// True when something in the map is upstream of `p` (under a roof or a
// tree, in the lee of a wall or a hill).
fn occluded(p: vec3<f32>) -> bool {
    if occ_params.x < 0.5 {
        return false;
    }
    let c = occ_clip * vec4<f32>(p, 1.0);
    let uv = vec2<f32>(c.x * 0.5 + 0.5, 0.5 - c.y * 0.5);
    // Outside the map's square or depth range: not covered, never killed.
    if any(uv < vec2<f32>(0.0)) || any(uv >= vec2<f32>(1.0)) || c.z <= 0.0 || c.z > 1.0 {
        return false;
    }
    let dims = vec2<f32>(textureDimensions(occ_map));
    let top = textureLoad(occ_map, vec2<i32>(uv * dims), 0);
    // 0.3 m of slack so a drop at a surface's own height isn't cut.
    return top > c.z + 0.3 / occ_params.y;
}

struct Vertex {
    @location(0) seed: vec3<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) drop: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) corner: vec2<f32>,
    @location(1) @interpolate(flat) alpha: f32,
    @location(2) dist: f32,
}

fn max3(v: vec3<f32>) -> f32 {
    return max(max(v.x, v.y), v.z);
}

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.corner = v.corner;
    out.alpha = 0.0;
    // Off: a degenerate position, nothing rasterizes.
    out.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    if v.drop.x >= precip.look.z {
        return out;
    }
    let size = precip.volume.x;
    let cam = view.world_position;
    let base = v.seed * size + precip.offset.xyz;
    let rel = (fract((base - cam) / size) - 0.5) * size;
    var p = cam + rel;
    let flutter = precip.velocity.w;
    if flutter > 0.0 {
        let t = precip.offset.w;
        let ph = v.drop.z * 6.2831853;
        p += vec3<f32>(sin(t * 1.7 + ph), 0.35 * sin(t * 2.3 + ph * 3.0), cos(t * 1.3 + ph * 1.7)) * flutter;
    }
    if occluded(p) {
        return out;
    }
    // Fade at the box faces (no pop on wrap) and inside the nested box.
    let a = abs(rel);
    var fade = 1.0 - smoothstep(0.8, 1.0, max3(a) / (0.5 * size));
    let inner = precip.volume.y;
    if inner > 0.0 {
        fade *= smoothstep(inner * 0.8, inner, max3(a));
    }
    let to_cam = cam - p;
    let dist = length(to_cam);
    fade *= smoothstep(0.3, 1.0, dist);
    let f = to_cam / max(dist, 1e-3);
    let pixel = dist * 2.0 / (view.clip_from_view[1][1] * view.main_pass_viewport.w);
    let width0 = mix(precip.look.x, precip.look.y, v.drop.y);
    let width = max(width0, precip.look.w * pixel);
    let energy = mix(0.35, 1.0, clamp(width0 / width, 0.0, 1.0));
    let vel = precip.velocity.xyz;
    if precip.volume.w < 0.5 {
        let speed = max(length(vel), 1e-3);
        let axis = vel / speed;
        let len = speed * precip.volume.z;
        var side = cross(axis, f);
        side = side / max(length(side), 1e-4);
        p += axis * (v.corner.y * 0.5 * len) + side * (v.corner.x * width);
    } else {
        let up_ref = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, -1.0), abs(f.y) > 0.999);
        let right = normalize(cross(up_ref, f));
        let up = cross(f, right);
        p += (right * v.corner.x + up * v.corner.y) * width;
    }
    out.position = position_world_to_clip(p);
    out.alpha = precip.color.a * fade * energy;
    out.dist = distance(p, cam);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var shape: f32;
    if precip.volume.w < 0.5 {
        // Soft across, tapered toward both ends.
        shape = (1.0 - in.corner.x * in.corner.x) * (1.0 - in.corner.y * in.corner.y);
    } else {
        let r2 = dot(in.corner, in.corner);
        if r2 >= 1.0 {
            discard;
        }
        shape = 1.0 - r2;
    }
    let alpha = in.alpha * shape;
    if alpha < 0.002 {
        discard;
    }
    // Fogged like any translucent (RO-3): streaks fade into the fog with distance.
    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, in.dist);
    return froxel_fog_apply(vec4<f32>(precip.color.rgb, alpha), fog, FOG_BLEND);
}
