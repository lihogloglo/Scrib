// sf_water_bevy: the water draw (water.md "Water pass" step 3; UE Single Layer Water structure).
// Vertex: the camera-centred clipmap displaced by the FFT cascades. Fragment: wave normals,
// refraction from the scene snapshot (rejected where the refracted sample is in front of the
// water), Beer-Lambert absorption + single scattering over the view path through the water,
// Bevy PBR lighting (GGX sun/moon, clustered lights, shadows, environment map) with water's
// F0 0.02, whitecap and contact foam, then aerial perspective (Bevy's aerial-view LUT) and the
// froxel fog. Optics shading follows bevy-aqua's structure (MIT OR Apache-2.0).

import super::water_common::{WaterParams, clipmap_xz, covered_by_finer, cascade_uv, displaced};
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT;
import bevy_pbr::render::pbr_types::pbr_input_new;
import bevy_pbr::render::pbr_functions::apply_pbr_lighting;

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> water: WaterParams;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var displacement: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var wave_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var slopes: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var snapshot_color: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var snapshot_depth: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(106) var froxels: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(107) var froxel_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(108) var aerial: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(109) var aerial_sampler: sampler;

struct WaterVertex {
    @location(0) position: vec3<f32>,
}

struct WaterVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) lagrangian: vec2<f32>,
    @location(2) @interpolate(flat) ring: u32,
    @location(3) height: f32,
}

@vertex
fn vertex(v: WaterVertex) -> WaterVarying {
    let camera = view.world_position.xz;
    let x0 = clipmap_xz(water, v.position.xz, v.position.y, camera);
    let p = displaced(water, displacement, wave_sampler, x0, camera);
    var out: WaterVarying;
    out.world_position = vec4(p.xyz, 1.0);
    out.position = view.clip_from_world * out.world_position;
    out.lagrangian = x0;
    out.ring = u32(v.position.y + 0.5);
    out.height = p.w;
    return out;
}

// World position of the scene at pixel `px` with reverse-Z `depth`.
fn scene_point(px: vec2<f32>, depth: f32) -> vec3<f32> {
    let uv = (px - view.viewport.xy) / view.viewport.zw;
    let ndc = vec3(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth);
    let w = view.world_from_clip * vec4(ndc, 1.0);
    return w.xyz / w.w;
}

@fragment
fn fragment(in: WaterVarying, @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    let camera = view.world_position;
    if covered_by_finer(water, in.ring, in.lagrangian, camera.xz) {
        discard;
    }
    let to_camera = camera - in.world_position.xyz;
    let d = length(to_camera);
    let V = to_camera / max(d, 1e-4);

    // Wave normal from the cascades' slopes at the Lagrangian point. A cascade fades out once a
    // pixel covers several of its texels (no mips yet); the lost slope variance widens the
    // highlight instead of aliasing (Bruneton 2010, simplified).
    let pixel = 2.0 / (view.clip_from_view[1][1] * view.viewport.w);
    var s = vec4(0.0);
    var whitecap = 0.0;
    var lost = 0.0;
    let n_cascades = u32(water.misc.x);
    let res = f32(textureDimensions(slopes).x);
    for (var c = 0u; c < n_cascades; c++) {
        let texel = water.sizes[c] / res;
        let fade = 1.0 - smoothstep(1.0, 6.0, d * pixel / texel);
        lost += 1.0 - fade;
        if fade > 0.0 {
            let uv = cascade_uv(water, c, in.lagrangian);
            s += fade * textureSampleLevel(slopes, wave_sampler, uv, i32(c), 0.0);
            // Whitecaps from the long cascades only (the short ones fold constantly at chop scale).
            whitecap += fade * f32(c < 2u) * textureSampleLevel(displacement, wave_sampler, uv, i32(c), 0.0).w;
        }
    }
    let far = smoothstep(water.misc.z * 2.0, water.misc.z * 20.0, d);
    var N = normalize(vec3(-s.x / max(1.0 + s.z, 0.2), 1.0, -s.y / max(1.0 + s.w, 0.2)));
    N = normalize(mix(N, vec3(0.0, 1.0, 0.0), far * 0.7));
    if !is_front {
        N = -N;
    }
    let roughness = clamp(water.foam.w + 0.06 * lost + 0.15 * far, 0.02, 0.6);

    // The scene behind the water (snapshot after sky, clouds and fog).
    let px = vec2<i32>(in.position.xy);
    let dims = vec2<i32>(textureDimensions(snapshot_depth));
    let valid = water.misc.w > 0.5 && all(px < dims);
    var behind = vec3(0.0);
    var path = 1e6;
    if valid {
        let depth0 = textureLoad(snapshot_depth, px, 0).r;
        var path0 = 1e6;
        if depth0 > 0.0 {
            path0 = distance(scene_point(in.position.xy, depth0), in.world_position.xyz);
        }
        // Refraction: offset by the normal, projected; none at contact (no smear on shores).
        let scale = water.foam.z * view.clip_from_view[1][1] * 0.5 * view.viewport.w / max(d, 1.0);
        let offset = N.xz * scale * saturate(path0 / 2.0);
        var rpx = clamp(vec2<i32>(in.position.xy + offset), vec2<i32>(0), dims - 1);
        var depth = textureLoad(snapshot_depth, rpx, 0).r;
        // Reject: the refracted sample lies in front of the water (reverse Z: larger is nearer).
        if depth > in.position.z {
            rpx = px;
            depth = depth0;
        }
        behind = textureLoad(snapshot_color, rpx, 0).rgb;
        path = path0;
        if depth > 0.0 {
            path = distance(scene_point(vec2<f32>(rpx) + 0.5, depth), in.world_position.xyz);
        } else {
            path = 1e6;
        }
    }

    // Beer-Lambert over the view path, single-scattering body colour (water.md "Depth colour").
    let sigma_t = max(water.absorption.rgb + water.scattering.rgb, vec3(1e-4));
    let transmittance = exp(-sigma_t * path);
    let albedo = water.scattering.rgb / sigma_t;
    var body = albedo * (1.0 - transmittance);
    // Thin crests lit from behind: a little more scattering at the wave tops.
    body *= 1.0 + 0.4 * saturate(in.height);

    // Foam: whitecaps (persistent Jacobian foam) and contact foam where the bed is close.
    var foam = saturate(whitecap * water.foam.x);
    if valid && path < 1e5 {
        let vertical = path * abs(V.y);
        foam = max(foam, 0.7 * (1.0 - smoothstep(0.0, water.foam.y, vertical)));
    }

    var pbr = pbr_input_new();
    pbr.material.base_color = vec4(mix(body, vec3(0.8), foam), 1.0);
    pbr.material.perceptual_roughness = mix(roughness, 0.9, foam);
    pbr.material.metallic = 0.0;
    // F0 = 0.16·r² = 0.02 (water).
    pbr.material.reflectance = vec3(0.354);
    pbr.frag_coord = in.position;
    pbr.world_position = in.world_position;
    pbr.world_normal = N;
    pbr.N = N;
    pbr.V = V;
    pbr.is_orthographic = false;
    pbr.flags = MESH_FLAGS_SHADOW_RECEIVER_BIT;
    var color = apply_pbr_lighting(pbr).rgb;
    let fresnel = 0.02 + 0.98 * pow(1.0 - saturate(dot(N, V)), 5.0);
    color += (1.0 - fresnel) * (1.0 - foam) * behind * transmittance;

    // Fog in front of the water: aerial perspective (as Bevy's render_sky applies to opaque
    // geometry), then the froxels (as `sf_atmos_bevy`'s transparent fog).
    let uv = (in.position.xy - view.viewport.xy) / view.viewport.zw;
    if water.aerial.w > 0.0 {
        let slices = f32(textureDimensions(aerial).z);
        let lut = textureSampleLevel(aerial, aerial_sampler, vec3(uv, saturate(d / water.aerial.w - 0.5 / slices)), 0.0);
        let inscatter = exp(lut.rgb) * saturate(d / (water.aerial.w / slices)) * view.exposure;
        color = color * exp(-water.aerial.rgb * d) + inscatter;
    }
    if water.froxel.w > 0.0 && d > 0.0 {
        let near = water.froxel.x;
        let n = water.froxel.z;
        var c = 0.0;
        if d > near {
            c = clamp(log(d / near) / log(water.froxel.y / near), 0.0, 1.0) * n;
        }
        let v = textureSampleLevel(froxels, froxel_sampler, vec3(uv, max(c - 0.5, 0.5) / n), 0.0);
        let fog = mix(vec4(0.0, 0.0, 0.0, 1.0), v, saturate(c));
        color = color * fog.a + fog.rgb;
    }
    return vec4(color, 1.0);
}
