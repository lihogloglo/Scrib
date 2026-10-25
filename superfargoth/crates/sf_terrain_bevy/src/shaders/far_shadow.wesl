// Far sun visibility past the shadow cascades (weather.md "Distant shadows",
// tier 2; terrain.md "Far shadows"). Bevy 0.20 treats every fragment past
// the last cascade as lit, with a hard cut. Its engine hook,
// `PbrInput::directional_shadow_factor`, multiplies the cascade result of
// every shadowed directional light; past the last cascade that result is
// 1.0, so the factor *is* the shadow there.
//
// Cascade end: a stochastic (dithered) transition, the way engines blend
// cascades (UE's dithered CSM transitions; Bevy's own visibility-range
// crossfade). Over the last 15 % of the cascades' range a growing fraction
// of pixels (ordered 4x4 dither) takes the far term instead of the
// cascades: for those, the lighting position is moved just past the last
// cascade along the view axis, so Bevy's lookup returns 1.0 and the factor
// alone shadows them. Directional lighting doesn't depend on the position,
// so the move changes only which shadow term applies. Object shadows thus
// fade out instead of ending on a line, and the terrain's own shadow fades in.
//
// Technique: heightfield horizon test toward the light (Max 1988; Sloan and
// Cohen 2000), marched per pixel over the height clipmap the terrain already
// binds: exponentially spaced steps, each on the clipmap level whose spacing
// matches the step, with a penumbra that widens with the occluder distance.
// It runs only for far pixels facing the light, and stops once the ray is
// above the highest terrain.
// Past FAR_MARCH_END the CPU shadow-height map (far_map.rs; one fetch, the
// same one far-field proxies and impostors read) replaces the march.

import super::common::{height_at, origin_y, debug_flags, pf, P_DETAIL, P_FAR, DEBUG_NO_FAR_SHADOWS, far_map, far_sampler};
import bevy_pbr::render::{
    mesh_view_bindings::{view, lights},
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
};

/// Steps per march.
const FAR_STEPS: u32 = 14u;
/// Horizontal reach of the march, meters (shadows longer than this are
/// cut; at 3° sun elevation a 250 m ridge casts ~4.8 km).
const FAR_REACH: f32 = 5000.0;
/// Penumbra half-width per meter of occluder distance: the sun's angular
/// radius (0.27°) plus softening for the coarse far heights.
const FAR_PENUMBRA: f32 = 0.012;
/// The cascades hand over to the far term from this fraction of their range.
const FAR_FADE_START: f32 = 0.85;
/// Receiver lift and first step, per meter of view distance: far terrain is
/// drawn from coarse LODs whose surface sits off the bilinear heights.
const FAR_BIAS_PER_M: f32 = 0.0015;
const FAR_FIRST_PER_M: f32 = 0.01;
/// Past this view distance the far shadow map (one fetch; sf_terrain_bevy
/// `FarShadowMap`) replaces the march, dithered over the last 20 %.
const FAR_MARCH_END: f32 = 700.0;
/// Map receiver lift: its heights are resampled at its texel size.
const FAR_MAP_BIAS: f32 = 6.0;
const FAR_MAP_PENUMBRA_MIN: f32 = 2.0;

/// What the material hands Bevy: the shadow factor, and the position to
/// light with (moved past the cascades for dithered far pixels).
struct FarShadow {
    factor: f32,
    position: vec4<f32>,
}

/// View-space depth (positive, meters) of a render-space position.
fn view_depth(p: vec3<f32>) -> f32 {
    return -dot(vec4<f32>(
        view.view_from_world[0].z,
        view.view_from_world[1].z,
        view.view_from_world[2].z,
        view.view_from_world[3].z,
    ), vec4<f32>(p, 1.0));
}

/// Ordered 4x4 (Bayer) dither threshold in (0, 1) for a pixel.
fn bayer4(frag_xy: vec2<f32>) -> f32 {
    let p = vec2<u32>(frag_xy) & vec2<u32>(3u);
    let a = p.x ^ p.y;
    let i = ((a & 1u) << 3u) | ((p.y & 1u) << 2u) | (a & 2u) | ((p.y >> 1u) & 1u);
    return (f32(i) + 0.5) / 16.0;
}

/// Terrain visibility (0 shadowed .. 1 lit) of render-space `p` toward the
/// light direction `to_light`; `dist` is the view distance (sets the bias).
fn terrain_light_visibility(p: vec3<f32>, to_light: vec3<f32>, dist: f32) -> f32 {
    let lxz = length(to_light.xz);
    // Light below the horizon (it is fading out) or overhead: no terrain term.
    if to_light.y <= 0.0 || lxz < 1e-4 {
        return 1.0;
    }
    let dir = to_light.xz / lxz;
    let slope = to_light.y / lxz;
    let y0 = p.y + origin_y() + max(dist * FAR_BIAS_PER_M, 0.1);
    let top = pf(P_DETAIL).w;
    let first = max(dist * FAR_FIRST_PER_M, 2.0);
    let ratio = pow(FAR_REACH / first, 1.0 / f32(FAR_STEPS - 1u));
    var s = first;
    var vis = 1.0;
    for (var i = 0u; i < FAR_STEPS; i += 1u) {
        let ray = y0 + s * slope;
        let w = s * FAR_PENUMBRA + 0.5;
        // Above every resident height: nothing further can occlude.
        if ray - w > top {
            break;
        }
        // Clipmap level with spacing ~ s / 8 (level l spacing = 0.5 * 2^l m).
        let lmin = u32(clamp(log2(s * 0.25), 0.0, 9.0));
        let h = height_at(p.xz + dir * s, lmin);
        vis = min(vis, smoothstep(-w, w, ray - h));
        if vis <= 0.0 {
            break;
        }
        s *= ratio;
    }
    return vis;
}

/// Far shadow map visibility of render-space `p` at view distance `dist`.
fn map_visibility(p: vec3<f32>, dist: f32) -> f32 {
    let far = pf(P_FAR);
    let uv = (p.xz - far.xy) * far.z;
    let edge = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    if edge >= 1.0 {
        return 1.0;
    }
    let t = textureSampleLevel(far_map, far_sampler, uv, 0.0).rg;
    let hs = t.x + far.w;
    let w = t.y * FAR_PENUMBRA + FAR_MAP_PENUMBRA_MIN;
    let vis = smoothstep(hs - w, hs + w, p.y + FAR_MAP_BIAS + dist * FAR_BIAS_PER_M);
    return mix(vis, 1.0, smoothstep(0.85, 1.0, edge));
}

/// The far shadow of the shadowed directional light (sun or moon) at
/// render-space `p` with shading normal `n`, for pixel `frag_xy`.
fn far_shadow_term(p: vec4<f32>, n: vec3<f32>, frag_xy: vec2<f32>) -> FarShadow {
    var out = FarShadow(1.0, p);
    if (debug_flags() & DEBUG_NO_FAR_SHADOWS) != 0u {
        return out;
    }
    let depth = view_depth(p.xyz);
    for (var i = 0u; i < lights.n_directional_lights; i += 1u) {
        let light = &lights.directional_lights[i];
        if ((*light).flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) == 0u || (*light).num_cascades == 0u {
            continue;
        }
        let s_max = (*light).cascades[(*light).num_cascades - 1u].far_bound;
        let f = smoothstep(FAR_FADE_START * s_max, s_max, depth);
        // Inside the cascades, or this pixel keeps them in the transition.
        if f < bayer4(frag_xy) {
            return out;
        }
        if depth < s_max {
            let forward = -view.world_from_view[2].xyz;
            out.position = vec4<f32>(p.xyz + forward * (s_max - depth + 0.5), p.w);
        }
        let l = (*light).direction_to_light;
        // Faces away from the light: unlit anyway, skip the march.
        if dot(n, l) > 0.0 {
            let dist = distance(view.world_position, p.xyz);
            let to_map = smoothstep(0.8 * FAR_MARCH_END, FAR_MARCH_END, dist);
            if pf(P_FAR).z > 0.0 && to_map >= bayer4(frag_xy + vec2<f32>(2.0, 1.0)) {
                out.factor = map_visibility(p.xyz, dist);
            } else {
                out.factor = terrain_light_visibility(p.xyz, l, dist);
            }
        }
        return out;
    }
    return out;
}
