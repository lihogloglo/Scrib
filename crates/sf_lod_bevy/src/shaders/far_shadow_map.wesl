// Far-field shadows past the cascades (weather.md "Distant shadows", tier
// 2; far-field.md "Far shadows"): the terrain's shadow-height map (one
// fetch; sf_terrain_bevy's `FarShadowMap`, handed over by the app) through
// Bevy's `PbrInput::directional_shadow_factor`, with the same dithered
// cascade end as the terrain (mirrors sf_terrain_bevy's far_shadow.wesl:
// band, dither and the lighting-position move must stay identical so
// proxies, impostors and terrain change over together).

import bevy_pbr::render::{
    mesh_view_bindings::{view, lights},
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
};

/// The cascades hand over to the far term from this fraction of their range.
const FAR_FADE_START: f32 = 0.85;
/// Penumbra half-width per meter of occluder distance, plus a floor for the
/// coarse map.
const FAR_PENUMBRA: f32 = 0.012;
const FAR_PENUMBRA_MIN: f32 = 2.0;
/// Receiver lift: the map's heights are resampled at its texel size, so
/// lit ground can sit a few meters under them; grows with view distance.
const FAR_BIAS: f32 = 6.0;
const FAR_BIAS_PER_M: f32 = 0.0015;

/// The shadow factor, and the position to light with (moved past the
/// cascades for dithered far pixels).
struct FarShadow {
    factor: f32,
    position: vec4<f32>,
}

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

/// Map visibility (0 shadowed .. 1 lit) of render-space `p`. `far`: (min x,
/// min z, 1 / extent, base height); extent 0 = no map yet.
fn map_visibility(p: vec3<f32>, map: texture_2d<f32>, samp: sampler, far: vec4<f32>) -> f32 {
    if far.z <= 0.0 {
        return 1.0;
    }
    let uv = (p.xz - far.xy) * far.z;
    let edge = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    if edge >= 1.0 {
        return 1.0;
    }
    let t = textureSampleLevel(map, samp, uv, 0.0).rg;
    let hs = t.x + far.w;
    let w = t.y * FAR_PENUMBRA + FAR_PENUMBRA_MIN;
    let y = p.y + FAR_BIAS + distance(view.world_position, p) * FAR_BIAS_PER_M;
    let vis = smoothstep(hs - w, hs + w, y);
    // Fade out toward the map's border.
    return mix(vis, 1.0, smoothstep(0.85, 1.0, edge));
}

/// The far shadow of the shadowed directional light at render-space `p`
/// with shading normal `n`, for pixel `frag_xy`.
fn far_map_shadow(p: vec4<f32>, n: vec3<f32>, frag_xy: vec2<f32>, map: texture_2d<f32>, samp: sampler, far: vec4<f32>) -> FarShadow {
    var out = FarShadow(1.0, p);
    // No map (none yet, or switched off): the cascades alone, as Bevy draws them.
    if far.z <= 0.0 {
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
        if f < bayer4(frag_xy) {
            return out;
        }
        if depth < s_max {
            let forward = -view.world_from_view[2].xyz;
            out.position = vec4<f32>(p.xyz + forward * (s_max - depth + 0.5), p.w);
        }
        if dot(n, (*light).direction_to_light) > 0.0 {
            out.factor = map_visibility(p.xyz, map, samp, far);
        }
        return out;
    }
    return out;
}
