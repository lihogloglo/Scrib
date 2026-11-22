// The sun's visibility at a world point (aerial-shadow.md "Architecture review", FG-12 A1): one
// rule for every pass that lights a medium with the sun. Bevy's cascades inside their range, the
// terrain's shadow-height map (far map) beyond, × the cloud shadow map. The froxels
// (`froxel_inject.wesl`) and the sun-visibility march (`sun_vis_march.wesl`) import it; each binds
// its own textures and passes them in.

import bevy_pbr::render::mesh_view_types::{DirectionalLight, DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT};

// Where the far map and the cloud shadow map sit, and the far shadow's rule (a value, built by
// the caller from its uniform).
struct SunShadowMaps {
    // The terrain's shadow-height map: (min x, min z, 1 / extent, base y); z 0: none.
    far_map: vec4<f32>,
    // The far shadow's rule (`sf_atmos_bevy::FarShadowShape`, the `sky.far_shadow.*` tunables):
    // penumbra per meter of occluder distance, its floor (m), bias (m), bias per meter of view
    // distance.
    far_vis: vec4<f32>,
    // The camera (the bias grows with the distance from it).
    camera: vec3<f32>,
    // Where the far shadow starts fading to lit (fraction of the map's half extent).
    far_fade: f32,
    // The cloud shadow map's basis (cloud_shadow.wesl); `cloud_shadow` 0: none.
    shadow_center: vec3<f32>,
    cloud_shadow: u32,
    shadow_x: vec3<f32>,
    shadow_y: vec3<f32>,
};

// The CSM visibility of `light` at `p` (Bevy's cascades, hard compare: the callers' samples are
// far coarser than a texel). `dir`: toward the light; `fwd`: distance along the view axis.
fn csm_visibility(light: DirectionalLight, shadow_maps: texture_depth_2d_array, shadow_sampler: sampler_comparison, p: vec3<f32>, dir: vec3<f32>, fwd: f32) -> f32 {
    if (light.flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) == 0u {
        return 1.0;
    }
    for (var c = 0u; c < light.num_cascades; c += 1u) {
        if fwd < light.cascades[c].far_bound {
            let bias = light.shadow_depth_bias * dir;
            let clip = light.cascades[c].clip_from_world * vec4(p + bias, 1.0);
            let ndc = clip.xyz / clip.w;
            if any(abs(ndc.xy) > vec2(1.0)) || ndc.z < 0.0 || ndc.z > 1.0 {
                return 1.0;
            }
            let uv = ndc.xy * vec2(0.5, -0.5) + 0.5;
            let layer = i32(light.depth_texture_base_index + c);
            return textureSampleCompareLevel(shadow_maps, shadow_sampler, uv, layer, ndc.z);
        }
    }
    return 1.0;
}

// The terrain's shadow-height map (sf_lod_bevy `map_visibility`'s rule: a penumbra widening with
// occluder distance, a bias growing with view distance, a fade toward the map's border). The
// values are the `sky.far_shadow.*` tunables, shared with the aerial view LUT's height shadow
// (vendored bevy_pbr patch "own, AS").
fn far_visibility(m: SunShadowMaps, far_map: texture_2d<f32>, s: sampler, p: vec3<f32>) -> f32 {
    let far = m.far_map;
    if far.z <= 0.0 {
        return 1.0;
    }
    let uv = (p.xz - far.xy) * far.z;
    let edge = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    if edge >= 1.0 {
        return 1.0;
    }
    let t = textureSampleLevel(far_map, s, uv, 0.0).rg;
    let hs = t.x + far.w;
    let w = t.y * m.far_vis.x + m.far_vis.y;
    let y = p.y + m.far_vis.z + distance(m.camera, p) * m.far_vis.w;
    return mix(smoothstep(hs - w, hs + w, y), 1.0, smoothstep(m.far_fade, 1.0, edge));
}

// The cloud shadow map (toroidal: sample it with a repeating sampler; cloud_shadow.wesl's basis).
fn cloud_visibility(m: SunShadowMaps, cloud_shadow: texture_2d<f32>, s: sampler, p: vec3<f32>) -> f32 {
    if m.cloud_shadow == 0u {
        return 1.0;
    }
    let o = p - m.shadow_center;
    let local = vec2(dot(o, m.shadow_x) / dot(m.shadow_x, m.shadow_x), dot(o, m.shadow_y) / dot(m.shadow_y, m.shadow_y));
    let uv = local * vec2(-0.5, 0.5) + 0.5;
    return textureSampleLevel(cloud_shadow, s, uv, 0.0).r;
}

// The sun's visibility from its three terms: the cascades and the far map both say "terrain",
// so the darker wins; the clouds multiply.
fn sun_visibility(csm: f32, far: f32, cloud: f32) -> f32 {
    return min(csm, far) * cloud;
}
