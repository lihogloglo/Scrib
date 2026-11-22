// Sun-visibility march (aerial-shadow.md "Architecture review", FG-12 A2; `sun_visibility.rs`): at
// half the view's internal resolution, each pixel marches its view ray from the camera to
// min(scene depth, `max_distance`) (sky: to `max_distance`), `samples` jittered samples placed at
// `reach · x²` (denser near the camera), each reading the shared `sun_visibility(p)`
// (`sun_visibility.wesl`: cascades, far map, cloud shadow). One march, two weighted means:
//
// - V_atmo (R): weights = Bevy's medium scattering density (the combined density LUT, row 1) ×
//   the medium's transmittance from the camera (rows 0 + 1), each segment's closed form.
// - V_fog (G): weights = our height fog's density × its transmittance, from the froxels' far on
//   (they shadow the near fog themselves); the ground banks are left out (their modulation is the
//   cloud trace's, per traced pixel).
//
// Past the reach the ray counts as lit: one closed-form segment to the scene (sky: `sky_end`), the
// medium at its midpoint. No weight = 1. B: the scene distance the resolve reprojects from (0: sky).
// CPU twin: `sf_atmos::sun_visibility::march`.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::bindings::{atmosphere, view, lights};
import bevy_pbr::atmosphere::functions::{uv_to_ndc, uv_to_ray_direction, sample_density_lut};
import super::fog::fog_depths;
import super::sun_visibility::{SunShadowMaps, csm_visibility, far_visibility, cloud_visibility, sun_visibility};
import super::sun_vis_common::{SunVisPass, MIN_WEIGHT, MIN_EXTINCTION, MAX_STORED_DISTANCE, sample_distance, sun_vis_jitter};

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

@group(1) @binding(0) var<uniform> sv: SunVisPass;
@group(1) @binding(1) var shadow_maps: texture_depth_2d_array;
@group(1) @binding(2) var shadow_sampler: sampler_comparison;
@group(1) @binding(3) var far_map: texture_2d<f32>;
@group(1) @binding(4) var cloud_shadow: texture_2d<f32>;
// Clamped (the far map), repeating (the toroidal cloud shadow map).
@group(1) @binding(5) var linear_sampler: sampler;
@group(1) @binding(6) var repeat_sampler: sampler;

// The sun's directional light (the one toward `sv.sun_dir`), or none.
fn sun_light_index() -> u32 {
    var best = 0.999;
    var li = 0xffffffffu;
    for (var i = 0u; i < lights.n_directional_lights; i += 1u) {
        let c = dot(lights.directional_lights[i].direction_to_light.xyz, sv.sun_dir);
        if c > best {
            best = c;
            li = i;
        }
    }
    return li;
}

// The atmosphere medium at distance `t` along the ray: (scattering, absorption), the mean over
// the channels, per atmosphere unit.
fn medium_at(cam_a: vec3<f32>, rd_a: vec3<f32>, t: f32) -> vec2<f32> {
    let r = length(cam_a + rd_a * t);
    let third = vec3(1.0 / 3.0);
    return vec2(dot(sample_density_lut(r, 1.0), third), dot(sample_density_lut(r, 0.0), third));
}

// One segment of length `len` through medium `m` seen through `transmittance`: (weight, its own
// transmittance) (`sf_atmos::sun_visibility::atmo_segment`).
fn atmo_segment(transmittance: f32, m: vec2<f32>, len: f32) -> vec2<f32> {
    let s = max(m.x, 0.0);
    let e = s + max(m.y, 0.0);
    let l = max(len, 0.0);
    if e <= MIN_EXTINCTION {
        return vec2(transmittance * s * l, 1.0);
    }
    let t = exp(-e * l);
    return vec2(transmittance * (s / e) * (1.0 - t), t);
}

// The height fog's optical depth over [t0, t1] (the composite's closed form, planar from the camera).
fn fog_tau(h_cam: f32, mu: f32, t0: f32, t1: f32) -> f32 {
    if sv.fog.enabled == 0u || t1 <= t0 {
        return 0.0;
    }
    return fog_depths(sv.fog, h_cam + mu * t0, mu, t1 - t0).x;
}

@fragment
fn march(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let px = vec2<u32>(floor(in.position.xy));
    // The nearest of the 2×2 full-resolution depths: `render_sky` masks geometry pixels only, so
    // at a silhouette the geometry's ray wins.
    let full_max = vec2<i32>(sv.full_size) - 1;
    let base = vec2<i32>(px) * 2;
    var depth = 0.0;
    var at = base;
    for (var i = 0; i < 4; i += 1) {
        let q = clamp(base + vec2(i & 1, i >> 1u), vec2(0), full_max);
        let d = textureLoad(depth_texture, q, 0);
        if d > depth {
            depth = d;
            at = q;
        }
    }
    let rd = uv_to_ray_direction(in.uv);
    let geometry = depth > 0.0;
    var scene = 0.0;
    if geometry {
        let uv = (vec2<f32>(at) + 0.5) / sv.full_size;
        let vp = view.view_from_clip * vec4(uv_to_ndc(uv), depth, 1.0);
        scene = length(vp.xyz / vp.w);
    }
    let reach = select(sv.max_distance, min(scene, sv.max_distance), geometry);
    let end = select(max(sv.sky_end, reach), scene, geometry);

    // The atmosphere in its own space (Bevy's `world_to_atmosphere`; t stays in meters).
    let cam_a = (atmosphere.world_to_atmosphere * vec4(sv.camera, 1.0)).xyz;
    let rd_a = (atmosphere.world_to_atmosphere * vec4(rd, 0.0)).xyz;
    let atmo_scale = length(rd_a);
    // The fog's altitude frame (the composite's).
    let ro_c = sv.camera - sv.planet_center;
    let r = length(ro_c);
    let mu = dot(ro_c / r, rd);
    let h_cam = r - sv.inner_radius;

    let li = sun_light_index();
    let maps = SunShadowMaps(sv.far_map, sv.far_vis, sv.camera, sv.far_fade, sv.shadow_center, sv.cloud_shadow, sv.shadow_x, sv.shadow_y);
    let n = max(sv.samples, 1u);
    let nf = f32(n);
    let j = sun_vis_jitter(px, sv.frame);
    var ta = 1.0;
    var tau = 0.0;
    var wa = 0.0;
    var sa = 0.0;
    var wf = 0.0;
    var sf = 0.0;
    for (var i = 0u; i < n; i += 1u) {
        let t0 = sample_distance(f32(i) / nf, reach);
        let t1 = sample_distance(f32(i + 1u) / nf, reach);
        let t = sample_distance((f32(i) + j) / nf, reach);
        let p = sv.camera + rd * t;
        var csm = 1.0;
        if li != 0xffffffffu {
            csm = csm_visibility(lights.directional_lights[li], shadow_maps, shadow_sampler, p, sv.sun_dir, dot(p - sv.camera, sv.forward));
        }
        let v = clamp(sun_visibility(csm, far_visibility(maps, far_map, linear_sampler, p), cloud_visibility(maps, cloud_shadow, repeat_sampler, p)), 0.0, 1.0);
        let seg = atmo_segment(ta, medium_at(cam_a, rd_a, t), (t1 - t0) * atmo_scale);
        wa += seg.x;
        sa += seg.x * v;
        ta *= seg.y;
        let a0 = clamp(sv.fog_start, t0, t1);
        let front = fog_tau(h_cam, mu, t0, a0);
        let back = fog_tau(h_cam, mu, a0, t1);
        if t1 > a0 {
            let w = exp(-(tau + front)) * (1.0 - exp(-back));
            wf += w;
            sf += w * v;
        }
        tau += front + back;
    }
    // The lit remainder past the reach.
    if end > reach {
        let seg = atmo_segment(ta, medium_at(cam_a, rd_a, 0.5 * (reach + end)), (end - reach) * atmo_scale);
        wa += seg.x;
        sa += seg.x;
        let a0 = clamp(sv.fog_start, reach, end);
        let front = fog_tau(h_cam, mu, reach, a0);
        let back = fog_tau(h_cam, mu, a0, end);
        if end > a0 {
            let w = exp(-(tau + front)) * (1.0 - exp(-back));
            wf += w;
            sf += w;
        }
    }
    let v_atmo = select(1.0, clamp(sa / max(wa, MIN_WEIGHT), 0.0, 1.0), wa > MIN_WEIGHT);
    let v_fog = select(1.0, clamp(sf / max(wf, MIN_WEIGHT), 0.0, 1.0), wf > MIN_WEIGHT);
    return vec4(v_atmo, v_fog, min(scene, MAX_STORED_DISTANCE), 1.0);
}
