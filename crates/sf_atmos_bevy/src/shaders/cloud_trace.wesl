// Cloud raymarch at reduced resolution (weather.md "Clouds", Rendering 1-4).
//
// One fragment per traced pixel: in checkerboard mode the target is a
// quarter of the half-resolution history (one pixel of each 2×2 block per
// frame, rotating over 4 frames); otherwise it is the half-resolution
// history itself. Lighting uses Bevy's atmosphere LUTs: sun/moon radiance
// through the transmittance LUT at the sample (sunset reddening), sky fill
// from the sky-view LUT, ground bounce, and aerial perspective between the
// camera and the cloud (aerial-view LUT, transmittance LUT beyond its range).
// The march stops at the main depth buffer (no prepass needed).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::{
    bindings::{view, lights, settings},
    functions::{
        sample_transmittance_lut, sample_sky_view_lut, sample_aerial_view_lut,
        direction_world_to_atmosphere, uv_to_ray_direction, uv_to_ndc, ndc_to_camera_dist,
    },
    bruneton_functions::sample_transmittance_lut_segment,
};
import super::cloud_common::{PI, ramp, multi_scatter, powder, shell_segment, sub_pixel, ign, henyey_greenstein, ray_sphere};
import super::cloud_density::{clouds, weather_sample, height_fraction, cloud_density, cirrus_hit};

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

struct TraceOutput {
    // Premultiplied cloud radiance (aerial perspective applied, exposed), and
    // the cloud's transmittance.
    @location(0) color: vec4<f32>,
    // Transmittance-weighted distance to the cloud (reprojection, upsampling).
    @location(1) dist: f32,
};

// Optical depth (density × m) from `p` toward the light. Steps grow, so a few
// samples reach the far side of the layer.
fn light_optical_depth(p: vec3<f32>, l: vec3<f32>) -> f32 {
    var tau = 0.0;
    var step = clouds.thickness / f32(max(clouds.light_steps, 1u));
    var t = step * 0.5;
    for (var i = 0u; i < clouds.light_steps; i += 1u) {
        let s = p + l * t;
        let h = height_fraction(s);
        if h < 0.0 || h > 1.0 {
            break;
        }
        tau += cloud_density(s, h, weather_sample(s), false) * step;
        step *= 1.5;
        t += step;
    }
    return tau;
}

@fragment
fn trace(in: FullscreenVertexOutput) -> TraceOutput {
    let block = vec2<u32>(in.position.xy);
    var half_px = vec2<f32>(block) + 0.5;
    if clouds.checkerboard != 0u {
        half_px = vec2<f32>(block * 2u + sub_pixel(clouds.frame)) + 0.5;
    }
    let uv = half_px / clouds.half_size;
    let full_px = clamp(vec2<i32>(uv * clouds.full_size), vec2(0), vec2<i32>(clouds.full_size) - 1);
    let depth = textureLoad(depth_texture, full_px, 0);
    var scene_t = 1e30;
    if depth > 0.0 {
        scene_t = ndc_to_camera_dist(vec3(uv_to_ndc(uv), depth));
    }

    let rd = uv_to_ray_direction(uv);
    let ro = view.world_position;
    let ro_c = ro - clouds.planet_center;
    let seg = shell_segment(ro_c, rd, clouds.inner_radius, clouds.base, clouds.thickness);
    let start = seg.x;
    let end = min(seg.y, min(scene_t, clouds.far));

    var out: TraceOutput;
    out.color = vec4(0.0, 0.0, 0.0, 1.0);
    out.dist = min(scene_t, clouds.far);
    let has_low = end > start;
    if !has_low && clouds.cirrus <= 0.0 {
        return out;
    }

    // The key light is the brightest directional light (sun by day, moon by night).
    var l = vec3(0.0, 1.0, 0.0);
    var light_color = vec3(0.0);
    var brightest = -1.0;
    for (var i = 0u; i < lights.n_directional_lights; i += 1u) {
        let c = lights.directional_lights[i].color.rgb;
        let lum = c.r + c.g + c.b;
        if lum > brightest {
            brightest = lum;
            l = normalize(lights.directional_lights[i].direction_to_light.xyz);
            light_color = c;
        }
    }
    let cos_t = dot(rd, l);

    // Fill terms, once per pixel: sky radiance from above, and the ground's
    // Lambertian bounce of the light from below.
    let r_cam = length(ro_c);
    let up = ro_c / r_cam;
    let sky_up = sample_sky_view_lut(r_cam, direction_world_to_atmosphere(up));
    let mu_l_ground = dot(up, l);
    let ground_light = light_color * sample_transmittance_lut(clouds.inner_radius + 1.0, mu_l_ground) * max(mu_l_ground, 0.0);
    let bounce = clouds.ground_albedo * ground_light / PI * clouds.ground_bounce;

    let span = max(end - start, 0.0);
    let n = select(0u, u32(clamp(span / (clouds.thickness / 24.0), f32(clouds.min_steps), f32(clouds.max_steps))), has_low);
    let ds = span / f32(n);
    var t = start + ds * ign(half_px, clouds.frame);

    var transmittance = 1.0;
    var luminance = vec3(0.0);
    var dist_sum = 0.0;
    var weight_sum = 0.0;
    for (var i = 0u; i < n; i += 1u) {
        if transmittance < 0.01 {
            break;
        }
        let p = ro + rd * t;
        let pc = p - clouds.planet_center;
        let r = length(pc);
        let h = (r - clouds.inner_radius - clouds.base) / clouds.thickness;
        let w = weather_sample(p);
        // Fade the deck over the last quarter of the reach, in the density
        // itself so it stops absorbing as well as glowing.
        let reach = 1.0 - ramp(clouds.far * 0.75, clouds.far, t);
        let density = cloud_density(p, h, w, true) * reach;
        if density > 0.0 {
            let sigma = density * clouds.extinction;
            let mu_l = dot(pc / r, l);
            let sun_t = sample_transmittance_lut(r, mu_l);
            let dark = 1.0 + 2.0 * w.wetness;
            let od = light_optical_depth(p, l) * clouds.extinction * dark;
            let ms = multi_scatter(od, cos_t, clouds.phase_g, clouds.ms_falloff, clouds.ms_octaves);
            let sun = light_color * sun_t * ms * powder(density, cos_t, clouds.powder);
            let fill = mix(bounce, sky_up, saturate(h)) * clouds.ambient * (1.0 - 0.6 * w.wetness);
            // × sigma: the step integral divides by sigma (bounded, scale-free).
            let scattered = (sun + fill) * sigma;
            let step_t = exp(-sigma * ds);
            luminance += transmittance * (scattered - scattered * step_t) / sigma;
            let absorbed = transmittance * (1.0 - step_t);
            dist_sum += t * absorbed;
            weight_sum += absorbed;
            transmittance *= step_t;
        }
        t += ds;
    }

    // The march stops at T < 0.01. Snap that remainder to zero (Nubis): the
    // HDR sun disk is ~10^5 times the sky, so 1 % of it behind "opaque"
    // cloud would still burn through and bloom.
    transmittance = saturate((transmittance - 0.01) / 0.99);
    let opacity = 1.0 - transmittance;
    var dist = select(min(scene_t, clouds.far), end, has_low);
    if weight_sum > 1e-4 {
        dist = dist_sum / weight_sum;
    }

    let mu_v = dot(up, rd);
    let low_air = aerial(uv, r_cam, mu_v, rd, dist);
    var color = luminance * low_air.transmittance + low_air.inscatter * opacity;

    // High layer (cirrus), behind the low one: seen through what it lets
    // pass. A thin ice sheet: single scattering with a strong forward lobe
    // plus sky fill, alpha from the slant optical depth. Fades past the
    // reach (no mips: far texels would shimmer) and never under the horizon.
    let ci = cirrus_hit(ro, rd);
    let below_horizon = ray_sphere(ro_c, rd, clouds.inner_radius).x > 0.0;
    if ci.y > 0.0 && ci.x < scene_t && !below_horizon && transmittance > 0.0 {
        let pc = ro + rd * ci.x - clouds.planet_center;
        let r = length(pc);
        let n_up = pc / r;
        let slant = ci.y * clouds.cirrus_depth / max(abs(dot(n_up, rd)), 0.15);
        let fade = 1.0 - ramp(clouds.far * 0.6, clouds.far * 1.6, ci.x);
        let alpha = (1.0 - exp(-slant)) * fade;
        let sun_t = sample_transmittance_lut(r, dot(n_up, l));
        let ice_phase = mix(henyey_greenstein(0.2, cos_t), henyey_greenstein(0.8, cos_t), 0.5);
        let lum = light_color * sun_t * ice_phase + sky_up * clouds.ambient;
        let air = aerial(uv, r_cam, mu_v, rd, ci.x);
        color += transmittance * alpha * (lum * air.transmittance + air.inscatter);
        if opacity < 0.05 {
            // Where cirrus is all there is, reproject by its distance.
            dist = ci.x;
        }
        transmittance *= 1.0 - alpha;
    }
    out.color = vec4(color * view.exposure, transmittance);
    out.dist = dist;
    return out;
}

struct Aerial {
    transmittance: vec3<f32>,
    inscatter: vec3<f32>,
};

// Aerial perspective between the camera and a cloud at `dist`: the
// aerial-view LUT up to its range, beyond it the sky's in-scattering in
// proportion to the extinction reached (uniform in-scatter per unit extinction).
fn aerial(uv: vec2<f32>, r_cam: f32, mu_v: f32, rd: vec3<f32>, dist: f32) -> Aerial {
    var a: Aerial;
    a.transmittance = sample_transmittance_lut_segment(r_cam, mu_v, dist);
    let lut_max = settings.aerial_view_lut_max_distance;
    a.inscatter = sample_aerial_view_lut(uv, min(dist, lut_max));
    if dist > lut_max {
        let sky = sample_sky_view_lut(r_cam, direction_world_to_atmosphere(rd));
        let t_inf = sample_transmittance_lut(r_cam, mu_v);
        let f = saturate((vec3(1.0) - a.transmittance) / max(vec3(1.0) - t_inf, vec3(1e-4)));
        a.inscatter = max(a.inscatter, sky * f);
    }
    return a;
}
