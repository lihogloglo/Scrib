// sf_water_bevy: the water draw (water.md "Water pass" step 3; UE Single Layer Water structure).
// Vertex: the camera-centred clipmap displaced by the FFT cascades, attenuated toward the shore
// by the `water.shore` field (swell by openness and shore distance, chop down to a floor); with
// `WATER_MESH`, a host mesh body (lake polygon, river ribbon) placed flat by its transform; with
// `WATER_FLOW` (its mesh carries the flow attribute) the ripples ride the river's flow (two-phase
// flow-advected normals, Vlachos 2010) and fast water foams.
// Fragment: wave normals from the mip-filtered slopes (the variance the mips lose widens the
// highlight), refraction from the scene snapshot (rejected where the refracted sample is in front
// of the water), Beer-Lambert absorption + single scattering over the view path with optics
// blended from the region map, Bevy PBR lighting (GGX sun/moon, clustered lights, shadows,
// environment map) with water's F0 0.02, local reflections from the SSPR image where it hits
// (W-SSPR, the environment map elsewhere), whitecap and breaker foam (SF-1: bore, swash and contact
// lace by age; `--off surffoam` = the WA-6 shore band and constant contact foam). No fog: the sky,
// aerial perspective, clouds and fog run after the water by its depth (W-ORDER, render-order.md).
// Optics shading follows bevy-aqua's structure (MIT OR Apache-2.0).

import super::water_common::{SurfInput, WaterParams, clipmap_xz, covered_by_finer, cascade_uv, cascade_scale, displaced, shore_at, shore_foam_weight, shore_normal_scales, shore_scales, shore_slope, shore_wave_offset, shore_waves, surf_birth, surf_foam, sw_cells, sw_hash, sw_noise};
import sf_render_order_bevy::rain_rings::rain_ripples;
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT;
import bevy_pbr::render::pbr_types::{PbrInput, pbr_input_new};
import bevy_pbr::render::pbr_functions::{apply_pbr_lighting, calculate_F0_dielectric};
import bevy_pbr::render::pbr_lighting::{F_AB, LAYER_BASE, LightingInput, perceptualRoughnessToRoughness};
import bevy_pbr::render::clustered_forward::{view_fragment_cluster_index, unpack_clusterable_object_index_ranges};
@if(ENVIRONMENT_MAP) import bevy_pbr::light_probe::environment_map;
@if(WATER_MESH) import bevy_pbr::render::mesh_functions::get_world_from_local;

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

// A host mesh body's vertex: its position and, on a river, its flow (`sf_water_bevy::mesh::
// ATTRIBUTE_WATER_FLOW`, local XZ m/s; location `material.rs` `FLOW_LOCATION`).
@if(WATER_MESH)
struct WaterMeshVertex {
    @location(0) position: vec3<f32>,
    @if(WATER_FLOW) @location(10) flow: vec2<f32>,
}

struct WaterVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) lagrangian: vec2<f32>,
    @location(2) @interpolate(flat) ring: u32,
    @location(3) height: f32,
    // The flow velocity, render-space XZ m/s.
    @if(WATER_FLOW) @location(4) flow: vec2<f32>,
}

@if(!WATER_MESH)
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

// A host mesh body (`sf_water_bevy::WaterMeshSurface`, water.md "Water bodies at any height": a
// lake's polygon at its level, a river's sloped ribbon): its own vertices through the entity's
// transform, the rest surface per vertex. Flat: no vertex displacement (a host mesh is far coarser
// than the clipmap, its chop would alias, and the banks would crack); the chop shows in the normal
// (the fragment's slopes, `waves.y`), as UE's water body meshes do. One ring, no finer one covers it.
@if(WATER_MESH)
@vertex
fn vertex(v: WaterMeshVertex, @builtin(instance_index) instance: u32) -> WaterVarying {
    let world_from_local = get_world_from_local(instance);
    let world = world_from_local * vec4(v.position, 1.0);
    var out: WaterVarying;
    // The flow is a vector in the mesh's local XZ: it turns (and scales) with the body.
    @if(WATER_FLOW) {
        out.flow = (world_from_local * vec4(v.flow.x, 0.0, v.flow.y, 0.0)).xz;
    }
    out.world_position = vec4(world.xyz / world.w, 1.0);
    out.position = view.clip_from_world * out.world_position;
    out.lagrangian = out.world_position.xz;
    out.ring = 0u;
    out.height = 0.0;
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

// Two-phase flow advection (Vlachos 2010; CPU twin `sf_water_bevy::flow_phases`): at cycle
// position `t`, the phases (x, y) in [0, 1) half a cycle apart and their triangle weights (z, w),
// 0 where a phase wraps, summing to 1.
fn flow_phases(t: f32) -> vec4<f32> {
    let a = fract(t);
    let b = fract(t + 0.5);
    return vec4(a, b, 1.0 - abs(1.0 - 2.0 * a), 1.0 - abs(1.0 - 2.0 * b));
}

// The phase noise's seed (`sw_noise`; the surf lace uses 7, 11 and 13).
const FLOW_NOISE_SEED = 17u;

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
    // River flow (`WATER_FLOW`): the slopes are read twice, upstream of the point by the distance the
    // water carried each phase's ripples, and cross-faded (Vlachos 2010); the phase is shifted per
    // point by a smooth noise fixed to the water's points so the cross-fade never pulses in step.
    // Still water (no attribute, or zero flow) reads once at the point, as before.
    var taps = 1u;
    var offsets = array<vec2<f32>, 2>(vec2(0.0), vec2(0.0));
    var weights = vec2(1.0, 0.0);
    var speed = 0.0;
    @if(WATER_FLOW) {
        speed = length(in.flow);
        if speed > 0.0 {
            let noise = sw_noise(fract((water.shore3.zw + in.lagrangian) / 1024.0), u32(max(water.flow2.x, 1.0)), FLOW_NOISE_SEED);
            let ph = flow_phases(water.flow.x / max(water.flow.y, 1e-3) + water.flow.w * noise);
            // Metres the ripples travel in one cycle.
            let travel = in.flow * water.flow.y * water.flow.z;
            offsets = array<vec2<f32>, 2>(-travel * ph.x, -travel * ph.y);
            weights = ph.zw;
            taps = 2u;
        }
    }
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
        for (var k = 0u; k < taps; k++) {
            let uv = cascade_uv(water, c, in.lagrangian + offsets[k]);
            s += weights[k] * cascade_scale(c, nscales) * textureSampleLevel(slopes, wave_sampler, uv, i32(c), lod);
            // Whitecaps from the long cascades only (the short ones fold constantly at chop scale).
            // (The displacement has no mips: foam fades out once it would alias.)
            whitecap += weights[k] * f32(c < 2u) * textureSampleLevel(displacement, wave_sampler, uv, i32(c), 0.0).w * scales.x * (1.0 - smoothstep(2.0, 5.0, lod));
        }
    }
    // Fast water is rougher: the ripples' slopes strengthen with speed up to the full foam speed.
    @if(WATER_FLOW) {
        s = vec4(s.xy * (1.0 + water.flow2.y * saturate(speed / max(water.flow2.w, 1e-3))), s.zw);
    }
    // Shore waves (W-GERSTNER): their analytic slopes, trains filtered out once a pixel's footprint
    // can't carry them (λ < 4 footprints).
    let gw = shore_waves(water, sh, shore_slope(water, shore_bed, shore_sampler, in.lagrangian), in.lagrangian, water.misc2.x, 3.14159265 / max(footprint, 1e-4));
    s += gw.slope;
    // Rain rings (R1): the puddles' procedural drop rings (`sf_render_order_bevy::rain_rings`) at
    // the surface point's render XZ (rain falls on space, the swell does not carry the rings),
    // × the host's rain (`rain.x`: 0 dry, indoors or `--off water_rain`) and the strength, faded
    // as a pixel spans a ring's breadth (no shimmer far off). Front face only: rain falls from
    // above. `rain_ripples` tilts the normal by `+slope`; `s.xy` is the height's gradient.
    if is_front && water.rain.x > 0.0 {
        let rings = water.rain.z * (1.0 - smoothstep(water.rain2.x, water.rain2.y, footprint));
        if rings > 0.0 {
            let tilt = rain_ripples(in.world_position.xz, water.rain.y, water.rain.x) * rings;
            s = vec4(s.xy - tilt, s.zw);
        }
    }
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
    // SF-1 breaker foam (`shore3.y` on) replaces the band where the shore waves run, and the contact
    // foam everywhere; `--off surffoam` (`shore3.y` 0) keeps the WA-6 band and the constant contact
    // foam below unchanged (the A/B arm).
    let surf_on = water.shore3.y > 0.5;
    let trains = water.swash2.x > 0.0;
    var foam = saturate(whitecap * look.whitecaps);
    var foam_rgb = vec3(0.8);
    // White water on a fast river (`WATER_FLOW`): foam cover rising with the flow speed.
    @if(WATER_FLOW) {
        foam = max(foam, water.flow3.x * smoothstep(water.flow2.z, water.flow2.w, speed));
    }
    let band = water.misc2.y;
    if !(surf_on && trains) && sh.x < band && look.shore_foam > 0.0 {
        let fade = 1.0 - smoothstep(0.0, band, max(sh.x, 0.0));
        let phase = sh.x * 0.8 + water.misc2.x * 1.3;
        // With the shore waves on, the bands ride their main train's breaking crests (W-GERSTNER).
        let wave = select(0.5 + 0.5 * sin(phase), 0.5 + 0.5 * gw.crest, water.swash2.x > 0.0);
        // The band follows the local wave energy (WA-6): the shore waves' breaking ratio, or without
        // them the open-water swell this openness would carry (no foam on calm or enclosed water).
        let breaking = select(sh.y * water.waves.x, gw.breaking, water.swash2.x > 0.0);
        foam = max(foam, look.shore_foam * fade * (0.35 + 0.65 * wave * wave) * shore_foam_weight(breaking, water.shore3.x));
    }
    if !surf_on && valid && path < 1e5 {
        let vertical = path * abs(V.y);
        foam = max(foam, 0.7 * (1.0 - smoothstep(0.0, look.contact, vertical)) * step(1e-3, look.contact));
    }
    if surf_on {
        // Bore foam trailing the main crest, swash and contact lace (`sf_water::surf_foam`), on a
        // cellular lace fixed to the water's Lagrangian points (it rides with the water) whose
        // width varies per wave; resolved lace fades to its mean once a pixel spans its cells.
        let vertical = select(1e6, path * abs(V.y), valid && path < 1e5);
        let birth = select(0.0, surf_birth(water, gw.breaking) * saturate(look.shore_foam), trains);
        let reach = look.contact * (1.0 + water.surf5.z);
        if birth > 0.0 || (look.contact > 1e-3 && vertical < reach) {
            let cells = max(water.surf2.x, 1.0);
            let cell_m = 1024.0 / cells;
            let uv = fract((water.shore3.zw + in.lagrangian) / 1024.0);
            let lace = sw_cells(uv, u32(cells), 7u);
            // The wave's count, wrapped at the whole cycles per clock period (no pop at the wrap).
            let cycles = max(round(water.swash.x * 256.0 / 6.2831853), 1.0);
            let n = floor((0.5 * 3.14159265 + max(water.surf.w, 1e-3) - gw.phase) / 6.2831853);
            let wave = u32(n - cycles * floor(n / cycles));
            let shift = vec2(sw_hash(wave, 0u, 11u), sw_hash(wave, 1u, 11u));
            let variation = sw_noise(fract(uv + shift), max(u32(cells) / 4u, 1u), 13u);
            let theta = atan2(gw.crest, gw.crest_cos);
            let detail = 1.0 - smoothstep(0.25, 1.0, footprint / cell_m);
            let t = surf_foam(water, SurfInput(gw.breaking, theta, gw.swash, vertical, look.contact, lace.x, variation, saturate(look.shore_foam) * select(0.0, 1.0, trains), detail));
            // Look by age: fresh foam dense and bright, aged foam thinner, translucent and tinted by
            // the body's hue (bubbles on water, not paint); the whitecaps keep their own colour.
            let age = 1.0 - t.fresh;
            let hue = body / max(max(body.r, max(body.g, body.b)), 1e-4);
            let surf_rgb = mix(water.surf3.z, water.surf3.y, t.fresh) * mix(vec3(1.0), hue, water.surf3.w * age);
            let cover = t.coverage * mix(1.0, water.surf4.x, age);
            foam_rgb = (foam * vec3(0.8) + cover * surf_rgb) / max(foam + cover, 1e-4);
            foam = 1.0 - (1.0 - foam) * (1.0 - cover);
            // The under-roller glow: air injected by breaking lights the body.
            body *= 1.0 + t.glow;
            // Foam relief (height `surf4.z` m × foam) catches the light: its slope from the lace gradient.
            let slope = clamp(water.surf4.z * t.d_lace / cell_m, -1.0, 1.0) * lace.yz;
            N = normalize(N + select(-1.0, 1.0, is_front) * vec3(-slope.x, 0.0, -slope.y));
        }
    }

    var pbr = pbr_input_new();
    pbr.material.base_color = vec4(mix(body, foam_rgb, foam), 1.0);
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
