// sf_water_bevy: the water draw (water.md "Water pass" step 3; UE Single Layer Water structure).
// Vertex: the camera-centred clipmap displaced by the FFT cascades, attenuated toward the shore
// by the `water.shore` field (swell by openness and shore distance, chop down to a floor).
// Fragment: wave normals from the mip-filtered slopes (the variance the mips lose widens the
// highlight), refraction from the scene snapshot (rejected where the refracted sample is in front
// of the water), Beer-Lambert absorption + single scattering over the view path with optics
// blended from the region map, Bevy PBR lighting (GGX sun/moon, clustered lights, shadows,
// environment map) with water's F0 0.02, local reflections from the SSPR image where it hits
// (W-SSPR, the environment map elsewhere), whitecap, shore and contact foam. No fog: the sky,
// aerial perspective, clouds and fog run after the water by its depth (W-ORDER, render-order.md).
// Optics shading follows bevy-aqua's structure (MIT OR Apache-2.0).

import super::water_common::{WaterParams, clipmap_xz, covered_by_finer, cascade_uv, cascade_scale, displaced, shore_at, shore_normal_scales, shore_scales, shore_slope, shore_wave_offset, shore_waves};
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT;
import bevy_pbr::render::pbr_types::{PbrInput, pbr_input_new};
import bevy_pbr::render::pbr_functions::{apply_pbr_lighting, calculate_F0_dielectric};
import bevy_pbr::render::pbr_lighting::{F_AB, LAYER_BASE, LightingInput, perceptualRoughnessToRoughness};
import bevy_pbr::render::clustered_forward::{view_fragment_cluster_index, unpack_clusterable_object_index_ranges};
@if(ENVIRONMENT_MAP) import bevy_pbr::light_probe::environment_map;

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> water: WaterParams;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var displacement: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var wave_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var slopes: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var snapshot_color: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var snapshot_depth: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(106) var reflection: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(107) var reflection_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var shore_field: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(111) var shore_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(112) var region_map: texture_2d<u32>;
// Bed slope β (W-GERSTNER-DEPTH, `sf_water::ShoreField::encode_slope_r16f`), sampled with `shore_sampler`.
@group(constants::MATERIAL_BIND_GROUP) @binding(113) var shore_bed: texture_2d<f32>;

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
    let sh = shore_at(water, shore_field, shore_sampler, x0);
    let scales = shore_scales(water, sh.xy);
    // FFT cascades, plus the shore waves where the swell fades (W-GERSTNER).
    let g = shore_wave_offset(water, sh, shore_slope(water, shore_bed, shore_sampler, x0), x0, camera, water.misc2.x);
    let p = displaced(water, displacement, wave_sampler, x0, camera, scales) + vec4(g, g.y);
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

// The environment-map specular `apply_pbr_lighting` added for `pbr` (pre-exposure), recomputed as
// Bevy's SSR pass does (`bevy_pbr` `ssr.wesl`), so the SSPR can take its place where it hits.
fn environment_specular(pbr: PbrInput) -> vec3<f32> {
    var out = vec3(0.0);
    @if(ENVIRONMENT_MAP) {
        let NdotV = max(dot(pbr.N, pbr.V), 0.0001);
        var input: LightingInput;
        input.layers[LAYER_BASE].NdotV = NdotV;
        input.layers[LAYER_BASE].N = pbr.N;
        input.layers[LAYER_BASE].R = reflect(-pbr.V, pbr.N);
        input.layers[LAYER_BASE].perceptual_roughness = pbr.material.perceptual_roughness;
        input.layers[LAYER_BASE].roughness = perceptualRoughnessToRoughness(pbr.material.perceptual_roughness);
        input.P = pbr.world_position.xyz;
        input.V = pbr.V;
        input.diffuse_color = vec3(0.0);
        input.metallic = 0.0;
        input.F0_dielectric = calculate_F0_dielectric(pbr.material.reflectance);
        input.F0_metallic = pbr.material.base_color.rgb;
        input.F_ab = F_AB(pbr.material.perceptual_roughness, NdotV);
        let view_z = dot(vec4(view.view_from_world[0].z, view.view_from_world[1].z, view.view_from_world[2].z, view.view_from_world[3].z), pbr.world_position);
        var ranges = unpack_clusterable_object_index_ranges(view_fragment_cluster_index(pbr.frag_coord.xy, view_z, false));
        out = environment_map::environment_map_light(&input, &ranges, true).specular * pbr.specular_occlusion;
    }
    return out;
}

// SSPR at the wave-perturbed reflection (W-SSPR): the reflected ray R is looked up at the screen
// pixel of its mirror direction (sf_water::sspr::lookup_direction; a flat surface reads its own
// pixel), blurred vertically with roughness (glossy water stretches reflections along the view).
// Premultiplied colour, confidence in alpha; 0 where SSPR missed.
fn sspr_sample(N: vec3<f32>, V: vec3<f32>, roughness: f32) -> vec4<f32> {
    let n = normalize(mix(vec3(0.0, 1.0, 0.0), N, water.reflection.x));
    let R = reflect(-V, n);
    if R.y <= 0.0 {
        return vec4(0.0);
    }
    let clip = view.clip_from_world * vec4(R.x, -R.y, R.z, 0.0);
    if clip.w <= 1e-6 {
        return vec4(0.0);
    }
    let ndc = clip.xy / clip.w;
    let uv = vec2(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    if any(uv < vec2(0.0)) || any(uv > vec2(1.0)) {
        return vec4(0.0);
    }
    let texels = vec2<f32>(textureDimensions(reflection)) * water.reflection.y;
    let at = (view.main_pass_viewport.xy + uv * view.main_pass_viewport.zw) / texels;
    let spread = roughness * roughness * 0.04;
    var sum = vec4(0.0);
    for (var k = 0; k < 4; k++) {
        let o = (f32(k) - 1.5) * spread;
        sum += textureSampleLevel(reflection, reflection_sampler, at + vec2(0.0, o), 0.0);
    }
    // Rough water (far, foamy) keeps the environment map.
    return sum * 0.25 * (1.0 - smoothstep(0.45, 0.65, roughness));
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
    // (The swash lifts the water up to its land margin onto the beach, W-GERSTNER.)
    if sh.x < -max(2.0, water.swash3.x) {
        discard;
    }
    let scales = shore_scales(water, sh.xy);
    // The normal keeps wind wavelets up to the shoreline where the displacement settles (W-SHORE).
    let nscales = shore_normal_scales(water, sh.xy);
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
        lost += smoothstep(0.0, 4.0, lod) * cascade_scale(c, nscales);
        let uv = cascade_uv(water, c, in.lagrangian);
        s += cascade_scale(c, nscales) * textureSampleLevel(slopes, wave_sampler, uv, i32(c), lod);
        // Whitecaps from the long cascades only (the short ones fold constantly at chop scale).
        // (The displacement has no mips: foam fades out once it would alias.)
        whitecap += f32(c < 2u) * textureSampleLevel(displacement, wave_sampler, uv, i32(c), 0.0).w * scales.x * (1.0 - smoothstep(2.0, 5.0, lod));
    }
    // Shore waves (W-GERSTNER): their analytic slopes, trains filtered out once a pixel's footprint
    // can't carry them (λ < 4 footprints).
    let gw = shore_waves(water, sh, shore_slope(water, shore_bed, shore_sampler, in.lagrangian), in.lagrangian, water.misc2.x, 3.14159265 / max(footprint, 1e-4));
    s += gw.slope;
    let far = smoothstep(water.misc.z * 2.0, water.misc.z * 16.0, d);
    var N = normalize(vec3(-s.x / max(1.0 + s.z, 0.2), 1.0, -s.y / max(1.0 + s.w, 0.2)));
    N = normalize(mix(N, vec3(0.0, 1.0, 0.0), far));
    if !is_front {
        N = -N;
    }
    let roughness = clamp(look.roughness + 0.06 * lost + 0.22 * far, 0.02, 0.65);

    // The scene behind the water (snapshot of the opaque scene, before the sky, clouds and fog).
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
        // With the shore waves on, the bands ride their main train's breaking crests (W-GERSTNER).
        let wave = select(0.5 + 0.5 * sin(phase), 0.5 + 0.5 * gw.crest, water.swash2.x > 0.0);
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
    // Local reflections (W-SSPR): where SSPR hits, its colour replaces the environment map's
    // specular with the same split-sum Fresnel (Bevy SSR's composition: env × (1 − hit)). The
    // snapshot is pre-exposed, the environment term is not.
    if water.reflection.y > 0.0 && is_front && valid {
        let hit = sspr_sample(N, V, pbr.material.perceptual_roughness) * (1.0 - foam);
        if hit.a > 0.0 {
            let NdotV = max(dot(N, V), 0.0001);
            let F0 = calculate_F0_dielectric(pbr.material.reflectance);
            let F_ab = F_AB(pbr.material.perceptual_roughness, NdotV);
            let FssEss = (F0 * F_ab.x + F_ab.y) * saturate(dot(F0, vec3(50.0 * 0.33)));
            color += FssEss * hit.rgb - hit.a * view.exposure * environment_specular(pbr);
        }
    }
    let fresnel = 0.02 + 0.98 * pow(1.0 - saturate(dot(N, V)), 5.0);
    color += (1.0 - fresnel) * (1.0 - foam) * behind * transmittance;

    return vec4(color, 1.0);
}
