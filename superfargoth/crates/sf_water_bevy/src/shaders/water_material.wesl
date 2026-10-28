// sf_water_bevy: the water draw (water.md "Water pass" step 3; UE Single Layer Water structure).
// Vertex: the camera-centred clipmap displaced by the FFT cascades, attenuated toward the shore
// by the `water.shore` field (swell by openness and shore distance, chop down to a floor).
// Fragment: wave normals from the mip-filtered slopes (the variance the mips lose widens the
// highlight), refraction from the scene snapshot (rejected where the refracted sample is in front
// of the water), Beer-Lambert absorption + single scattering over the view path with optics
// blended from the region map, Bevy PBR lighting (GGX sun/moon, clustered lights, shadows,
// environment map) with water's F0 0.02, whitecap, shore and contact foam, then aerial
// perspective (Bevy's aerial-view LUT) and the froxel fog, or the analytic height fog when the
// froxels are off. Optics shading follows bevy-aqua's structure (MIT OR Apache-2.0).

import super::water_common::{WaterParams, clipmap_xz, covered_by_finer, cascade_uv, cascade_scale, displaced, shore_at, shore_scales};
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
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var shore_field: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(111) var shore_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(112) var region_map: texture_2d<u32>;

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
    let scales = shore_scales(water, shore_at(water, shore_field, shore_sampler, x0));
    let p = displaced(water, displacement, wave_sampler, x0, camera, scales);
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
    let uv = (px - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let ndc = vec3(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth);
    let w = view.world_from_clip * vec4(ndc, 1.0);
    return w.xyz / w.w;
}

struct Look {
    absorption: vec3<f32>,
    scattering: vec3<f32>,
    whitecaps: f32,
    contact: f32,
    roughness: f32,
    shore_foam: f32,
}

fn table_look(i: u32) -> Look {
    let j = min(i, 7u) * 3u;
    let a = water.looks[j];
    let b = water.looks[j + 1u];
    let c = water.looks[j + 2u];
    return Look(a.xyz, b.xyz, a.w, b.w, c.x, c.y);
}

fn scaled(l: Look, w: f32) -> Look {
    return Look(l.absorption * w, l.scattering * w, l.whitecaps * w, l.contact * w, l.roughness * w, l.shore_foam * w);
}

fn added(a: Look, b: Look) -> Look {
    return Look(a.absorption + b.absorption, a.scattering + b.scattering, a.whitecaps + b.whitecaps, a.contact + b.contact, a.roughness + b.roughness, a.shore_foam + b.shore_foam);
}

// The optics at render-space `xz`: the body's own, or the region map's entries blended
// bilinearly between cell centres (water-adapter.md "Optics by region": the border blend).
fn look_at(xz: vec2<f32>) -> Look {
    if water.regions.w < 0.5 {
        return Look(water.absorption.rgb, water.scattering.rgb, water.foam.x, water.foam.y, water.foam.w, water.misc2.w);
    }
    let dims = vec2<i32>(textureDimensions(region_map));
    let g = (xz - water.regions.xy) * water.regions.z;
    let i0 = vec2<i32>(floor(g));
    let f = g - floor(g);
    var out = scaled(table_look(0u), 0.0);
    for (var k = 0u; k < 4u; k++) {
        let o = vec2<i32>(i32(k & 1u), i32(k >> 1u));
        let q = clamp(i0 + o, vec2(0), dims - 1);
        let w = select(1.0 - f.x, f.x, o.x == 1) * select(1.0 - f.y, f.y, o.y == 1);
        out = added(out, scaled(table_look(textureLoad(region_map, q, 0).r), w));
    }
    return out;
}

// Optical depth of the analytic height fog layer along a ray from altitude `h0` with vertical
// cosine `mu` over `d` metres (`sf_atmos::FogLayer::optical_depth` is the CPU twin).
fn layer_depth(h0: f32, mu: f32, d: f32) -> f32 {
    let k = water.fog_height.y;
    let base = water.fog_height.z;
    let e0 = exp(min(-(h0 - base) * k, 2.0));
    let x = k * mu * d;
    if abs(x) < 1e-3 {
        return water.fog_height.x * d * e0 * (1.0 - 0.5 * x);
    }
    let e1 = exp(min(-(h0 + mu * d - base) * k, 2.0));
    return water.fog_height.x * (e0 - e1) / (k * mu);
}

@fragment
fn fragment(in: WaterVarying, @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    let camera = view.world_position;
    if covered_by_finer(water, in.ring, in.lagrangian, camera.xz) {
        discard;
    }
    if water.waves.z > 0.5 && (any(in.lagrangian < water.bounds.xy) || any(in.lagrangian > water.bounds.zw)) {
        discard;
    }
    // The sea under land is not shaded (it is behind the terrain anyway).
    let sh = shore_at(water, shore_field, shore_sampler, in.lagrangian);
    if sh.x < -2.0 {
        discard;
    }
    let scales = shore_scales(water, sh);
    let look = look_at(in.world_position.xz);
    let to_camera = camera - in.world_position.xyz;
    let d = length(to_camera);
    let V = to_camera / max(d, 1e-4);

    // Wave normal from the cascades' slopes at the Lagrangian point, mip-filtered by the pixel's
    // major-axis footprint on the surface (grazing rays span `1 / |V.y|` more wave crests along the
    // view axis, WA-1); the slope variance the mips average away widens the highlight instead of
    // aliasing (Bruneton 2010, simplified).
    let pixel = 2.0 / (view.clip_from_view[1][1] * view.main_pass_viewport.w);
    let footprint = d * pixel / max(abs(V.y), 0.06);
    var s = vec4(0.0);
    var whitecap = 0.0;
    var lost = 0.0;
    let n_cascades = u32(water.misc.x);
    let res = f32(textureDimensions(slopes).x);
    let top = f32(textureNumLevels(slopes) - 1);
    for (var c = 0u; c < n_cascades; c++) {
        let texel = water.sizes[c] / res;
        let lod = clamp(log2(max(footprint / texel, 1e-3)), 0.0, top);
        lost += smoothstep(0.0, 4.0, lod) * cascade_scale(c, scales);
        let uv = cascade_uv(water, c, in.lagrangian);
        s += cascade_scale(c, scales) * textureSampleLevel(slopes, wave_sampler, uv, i32(c), lod);
        // Whitecaps from the long cascades only (the short ones fold constantly at chop scale).
        // (The displacement has no mips: foam fades out once it would alias.)
        whitecap += f32(c < 2u) * textureSampleLevel(displacement, wave_sampler, uv, i32(c), 0.0).w * scales.x * (1.0 - smoothstep(2.0, 5.0, lod));
    }
    let far = smoothstep(water.misc.z * 2.0, water.misc.z * 16.0, d);
    var N = normalize(vec3(-s.x / max(1.0 + s.z, 0.2), 1.0, -s.y / max(1.0 + s.w, 0.2)));
    N = normalize(mix(N, vec3(0.0, 1.0, 0.0), far));
    if !is_front {
        N = -N;
    }
    let roughness = clamp(look.roughness + 0.06 * lost + 0.22 * far, 0.02, 0.65);

    // The scene behind the water (snapshot after sky, clouds and fog).
    let px = vec2<i32>(in.position.xy);
    let vp_min = vec2<i32>(view.main_pass_viewport.xy);
    let dims = min(vec2<i32>(textureDimensions(snapshot_depth)), vp_min + vec2<i32>(view.main_pass_viewport.zw));
    let valid = water.misc.w > 0.5 && all(px >= vp_min) && all(px < dims);
    var behind = vec3(0.0);
    var path = 1e6;
    if valid {
        let depth0 = textureLoad(snapshot_depth, px, 0).r;
        var path0 = 1e6;
        if depth0 > 0.0 {
            path0 = distance(scene_point(in.position.xy, depth0), in.world_position.xyz);
        }
        // Refraction: offset by the normal, projected; none at contact (no smear on shores).
        let scale = water.foam.z * view.clip_from_view[1][1] * 0.5 * view.main_pass_viewport.w / max(d, 1.0);
        let offset = N.xz * scale * saturate(path0 / 2.0);
        var rpx = clamp(vec2<i32>(in.position.xy + offset), vp_min, dims - 1);
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
    let sigma_t = max(look.absorption + look.scattering, vec3(1e-4));
    let transmittance = exp(-sigma_t * path);
    let albedo = look.scattering / sigma_t;
    var body = albedo * (1.0 - transmittance);
    // Thin crests lit from behind: a little more scattering at the wave tops.
    body *= 1.0 + 0.4 * saturate(in.height);

    // Foam: whitecaps (persistent Jacobian foam), a shore band whose bands run shoreward with the
    // swell (aqua's shoreline foam), and contact foam where the bed or an object is close.
    var foam = saturate(whitecap * look.whitecaps);
    let band = water.misc2.y;
    if sh.x < band && look.shore_foam > 0.0 {
        let fade = 1.0 - smoothstep(0.0, band, max(sh.x, 0.0));
        let phase = sh.x * 0.8 + water.misc2.x * 1.3;
        let wave = 0.5 + 0.5 * sin(phase);
        foam = max(foam, look.shore_foam * fade * (0.35 + 0.65 * wave * wave) * (0.3 + 0.7 * sh.y));
    }
    if valid && path < 1e5 {
        let vertical = path * abs(V.y);
        foam = max(foam, 0.7 * (1.0 - smoothstep(0.0, look.contact, vertical)) * step(1e-3, look.contact));
    }

    var pbr = pbr_input_new();
    pbr.material.base_color = vec4(mix(body, vec3(0.8), foam), 1.0);
    pbr.material.perceptual_roughness = mix(roughness, 0.9, foam);
    pbr.material.metallic = 0.0;
    // F0 = 0.16 r^2 = 0.02 (water).
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
    // geometry), then the froxels (as `sf_atmos_bevy`'s transparent fog), or without froxels the
    // analytic height fog (the Low/Medium path, as the `sf_atmos` composite fogs the land).
    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
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
    } else if water.fog_height.w > 0.0 && d > 0.0 {
        let t = exp(-layer_depth(camera.y, -V.y, d));
        color = color * t + water.fog_color.rgb * view.exposure * (1.0 - t);
    }
    return vec4(color, 1.0);
}
