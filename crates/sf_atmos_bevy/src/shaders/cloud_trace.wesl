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
    bindings::{view, settings},
    functions::{
        sample_transmittance_lut, sample_sky_view_lut, sample_aerial_view_lut,
        direction_world_to_atmosphere, uv_to_ray_direction, uv_to_ndc, ndc_to_camera_dist,
    },
    bruneton_functions::sample_transmittance_lut_segment,
};
import super::cloud_common::{PI, WeatherSample, ramp, multi_scatter, powder, shell_segment, sub_pixel, ign, henyey_greenstein, ray_sphere};
import super::fog::fog_phase;
import super::cloud_density::{clouds, weather_sample_a, map_a, map_layer, ray_a, ray_b, ray_local_a, ray_local_b, height_fraction, cloud_density, cirrus_hit, bank_mask};
import super::key_light::{key_light, planet_lit_at};

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

struct TraceOutput {
    // Premultiplied cloud radiance (aerial perspective applied, exposed), and
    // the cloud's transmittance.
    @location(0) color: vec4<f32>,
    // R: transmittance-weighted distance to the cloud (reprojection,
    // upsampling). G: the ground-fog bank modulation (`bank_modulation`).
    @location(1) dist: vec2<f32>,
};

// Optical depth (density × m) from `p` toward the light. Steps grow, so a few
// samples reach the far side of the layer.
// `a`: the primary sample's layer-A weather (per-ray map lookup or uniform);
// the per-sample map refetches it along the light. `b`: the primary sample's
// layer B (base, thickness, density, erosion), kept along the light (heights
// change over the border band, wider than the light march's reach).
fn light_optical_depth(p: vec3<f32>, l: vec3<f32>, steps: u32, w0: WeatherSample, a: vec4<f32>, b: vec4<f32>) -> f32 {
    var tau = 0.0;
    // Fewer steps cover the same reach (steps grow by 1.5: the reach of n
    // steps is a geometric sum), only more coarsely.
    let full = f32(max(clouds.light_steps, 1u));
    let reach_ratio = (pow(1.5, full) - 1.0) / (pow(1.5, f32(max(steps, 1u))) - 1.0);
    var step = max(b.y, 1.0) / full * reach_ratio;
    var t = step * 0.5;
    for (var i = 0u; i < steps; i += 1u) {
        let s = p + l * t;
        let h = height_fraction(s, b);
        if h < 0.0 || h > 1.0 {
            break;
        }
        // The first sample is a tenth of the layer away: the weather field
        // (kilometer-sized) is the one at `p` there; skip its two fetches.
        var w = w0;
        if i > 0u {
            var ai = a;
            if clouds.map_on == 1u {
                ai = map_a(s);
            }
            w = weather_sample_a(s, ai);
        }
        tau += cloud_density(s, h, w, b, false) * step;
        step *= 1.5;
        t += step;
    }
    return tau;
}

// The planet's shadow (`key_light.wesl`).
fn planet_lit(r: f32, mu: f32) -> f32 {
    return planet_lit_at(clouds.inner_radius, r, mu);
}

// The ground-fog banks along the ray (weather.md "Fog", the far fog): how
// much of the bank layer's analytic optical depth the bank noise lets
// through, weighted by what the eye still sees (0 between banks, 1 inside).
// The composite multiplies the layer's closed form by it at full resolution,
// so depth edges stay exact. Steps are quadratic in distance (fine near the
// eye, where shapes are read); the march stops where the fog turns opaque.
fn bank_modulation(ro: vec3<f32>, rd: vec3<f32>, h0: f32, mu: f32, d_max: f32, jitter: f32) -> f32 {
    let f = clouds.fog;
    let top = f.bank_base + 6.0 * f.bank_falloff;
    var t0 = 0.0;
    var t1 = d_max;
    if h0 > top {
        if mu > -1e-4 {
            return 1.0;
        }
        t0 = (top - h0) / mu;
    } else if mu > 1e-4 {
        t1 = min(t1, (top - h0) / mu);
    }
    if t1 <= t0 {
        return 1.0;
    }
    let n = f32(max(f.steps, 1u));
    let span = t1 - t0;
    var plain = 0.0;
    var noisy = 0.0;
    var tau = 0.0;
    for (var i = 0u; i < f.steps; i += 1u) {
        let u = (f32(i) + jitter) / n;
        let t = t0 + span * u * u;
        let ds = span * (2.0 * f32(i) + 1.0) / (n * n);
        let h = h0 + mu * t;
        let sigma = f.bank_density * exp(-max(h - f.bank_base, 0.0) / f.bank_falloff);
        let mask = bank_mask(ro + rd * t);
        let w = sigma * ds * exp(-tau);
        plain += w;
        noisy += w * mask;
        tau += sigma * mask * ds;
        if tau > 8.0 {
            break;
        }
    }
    if plain <= 1e-7 {
        return 1.0;
    }
    return noisy / plain;
}

// Distant weather (weather.md WE4d): precipitation shafts under wet map
// cells and far fog by location, marched below the cloud deck out to the
// map window's half width (High/Ultra: per-sample map only). Both add only
// what the camera's own weather lacks: shaft extinction is the map's
// precipitation extinction (`PrecipKind.extinction` × intensity, layer D, the
// number `visibility_m` uses) minus the camera's, and the fog term the map's
// height-fog density minus the camera's, on the camera layer's height
// profile. At the camera both are zero, so the near rain (particles) and the
// analytic fog are unchanged and crossing a border band fades the curtain out
// exactly as the camera's weather fades in (no pop, no double rain).
struct FarWeather {
    color: vec3<f32>,
    transmittance: f32,
    dist_sum: f32,
    weight: f32,
};

const FAR_STEPS: u32 = 8u;
// Horizontal drift of the rain per meter of fall (slanted curtains).
const FAR_SHEAR: f32 = 0.35;

fn far_weather(ro: vec3<f32>, rd: vec3<f32>, scene_t: f32, jitter: f32, light_color: vec3<f32>, l: vec3<f32>, cos_t: f32, fill0: vec3<f32>) -> FarWeather {
    var out: FarWeather;
    out.color = vec3(0.0);
    out.transmittance = 1.0;
    out.dist_sum = 0.0;
    out.weight = 0.0;
    let t0 = 32.0;
    let t1 = min(scene_t, 0.5 * clouds.map_size / max(clouds.map_inv_texel, 1e-9));
    if t1 <= t0 {
        return out;
    }
    let cam_c = map_layer(ro, 2);
    let cam_d = map_layer(ro, 3);
    let f = clouds.fog;
    let fog_on = f.enabled != 0u && f.height_falloff > 0.0;
    let wl = length(clouds.wind_step.xz);
    let shear = select(vec2(0.0), clouds.wind_step.xz / max(wl, 1e-9) * FAR_SHEAR, wl > 1e-6);
    let sun_phase = fog_phase(0.6, cos_t);
    let n = f32(FAR_STEPS);
    let span = t1 - t0;
    var tr = 1.0;
    for (var i = 0u; i < FAR_STEPS; i += 1u) {
        // Quadratic in distance like the bank march: fine near the hand-off.
        let u = (f32(i) + jitter) / n;
        let t = t0 + span * u * u;
        let ds = span * (2.0 * f32(i) + 1.0) / (n * n);
        let p = ro + rd * t;
        let pc = p - clouds.planet_center;
        let r = length(pc);
        let alt = r - clouds.inner_radius;
        let b = map_layer(p, 1);
        let d = map_layer(p, 3);
        var sigma_p = 0.0;
        var sigma_f = 0.0;
        var shadow = 1.0;
        var dim = 1.0;
        var grey = 0.0;
        let dp = max(d.x - cam_d.x, 0.0) * 1e-3;
        if dp > 0.0 && alt < b.x {
            // The rain at `p` left the base upwind; it falls where the deck's
            // cells are (the clouds' own weather noise), so curtains hang under
            // the storm's cells, not under the whole region.
            let q = p + vec3(shear.x, 0.0, shear.y) * (b.x - alt);
            let w = weather_sample_a(q, map_a(q));
            sigma_p = dp * ramp(0.05, 0.5, w.coverage) * (1.0 - ramp(b.x - 150.0, b.x, alt)) * ramp(32.0, 128.0, t);
            // Key light through the deck above (a column estimate from the
            // map: dark under the cell, bright at its sunlit edge), sky fill
            // dimmed by the wet deck like the clouds' own.
            shadow = exp(-w.coverage * b.z * b.y * clouds.extinction * 0.3);
            dim = 1.0 - 0.6 * w.wetness;
            // Under the deck the sky fill is cloud light: grey, not the blue zenith.
            grey = w.coverage;
        }
        if fog_on {
            let c = map_layer(p, 2);
            sigma_f = max(c.z - cam_c.z, 0.0) * exp(min(-(alt - f.height_base) / f.height_falloff, 2.0));
        }
        let sigma = sigma_p + sigma_f;
        if sigma <= 1e-9 {
            continue;
        }
        let mu_l = dot(pc / r, l);
        let sun = light_color * sample_transmittance_lut(r, mu_l) * planet_lit(r, mu_l) * sun_phase;
        let tint = d.yzw / max(d.x, 1e-6);
        let fill = mix(fill0, vec3(dot(fill0, vec3(0.2126, 0.7152, 0.0722))), grey);
        var scattered = sigma_p * tint * (sun * shadow + fill * dim);
        if sigma_f > 0.0 {
            scattered += sigma_f * f.albedo * (sun + fill0 * f.ambient);
        }
        let step_t = exp(-sigma * ds);
        out.color += tr * (scattered - scattered * step_t) / sigma;
        let absorbed = tr * (1.0 - step_t);
        out.dist_sum += t * absorbed;
        out.weight += absorbed;
        tr *= step_t;
        if tr < 0.01 {
            break;
        }
    }
    out.transmittance = tr;
    return out;
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
    // The envelope shell: lowest base to highest top over the map's climates,
    // so a taller neighbour's deck is marched too (WE4b).
    let seg = shell_segment(ro_c, rd, clouds.inner_radius, clouds.shell_base, clouds.shell_thickness);
    let start = seg.x;
    let end = min(seg.y, min(scene_t, clouds.far));

    // The ground-fog banks (F2): marched for the fog even where no cloud is.
    var bank = 1.0;
    let fog = clouds.fog;
    if fog.enabled != 0u && fog.bank_density > 0.0 {
        let r0 = length(ro_c);
        bank = bank_modulation(ro, rd, r0 - clouds.inner_radius, dot(ro_c / r0, rd), min(scene_t, fog.far), ign(half_px, clouds.frame));
    }

    var out: TraceOutput;
    out.color = vec4(0.0, 0.0, 0.0, 1.0);
    out.dist = vec2(min(scene_t, clouds.far), bank);
    let clouds_on = clouds.clouds_on != 0u;
    let has_low = end > start && clouds_on;
    let far_on = clouds.far_weather != 0u && clouds_on;
    if !has_low && (clouds.cirrus_max <= 0.0 || !clouds_on) && !far_on {
        return out;
    }

    let key = key_light(ro_c, clouds.inner_radius);
    let l = key.dir;
    let light_color = key.color;
    let cos_t = dot(rd, l);

    // Fill terms, once per pixel: sky radiance from above, and the ground's
    // Lambertian bounce of the light from below.
    let r_cam = length(ro_c);
    let up = ro_c / r_cam;
    let sky_up = sample_sky_view_lut(r_cam, direction_world_to_atmosphere(up));
    let mu_l_ground = dot(up, l);
    let ground_light = light_color * sample_transmittance_lut(clouds.inner_radius + 1.0, mu_l_ground) * max(mu_l_ground, 0.0);
    let bounce = clouds.ground_albedo * ground_light / PI * clouds.ground_bounce;

    // Step length grows with distance, `ds = ds0 + g·t` (a constant angular
    // footprint, as Nubis and UE's volumetric clouds scale steps with view
    // distance): near cloud keeps its local layer's step (thickness / 20)
    // however tall the envelope shell, and the budget still reaches `end`:
    // `g` lets `max_steps` coarse (2·ds) steps grow t geometrically from
    // `t_ref` to `end`. Before (WE4c) one step served the whole span
    // (span / max_steps): ~780 m on every grazing ray through an 8 km storm
    // shell, so near decks drew as banded grey slabs. Continuous in the span
    // (no integer count: that drew bands), capped at span / min_steps.
    let span = max(end - start, 0.0);
    let ds0 = clouds.thickness / 20.0;
    let t_ref = max(start, 8.0 * ds0);
    let g = 0.5 * max(pow(max(end, t_ref) / t_ref, 1.0 / f32(clouds.max_steps)) - 1.0, 0.0);
    let ds_max = max(span / f32(clouds.min_steps), 1.0);
    var ds = min(ds0 + g * start, ds_max);
    var t = start + ds * ign(half_px, clouds.frame);
    // The world weather map: per sample (High/Ultra) or 3 lookups per ray (Medium/Low).
    let ray = ray_a(ro, rd, start, end);
    let ray_heights = ray_b(ro, rd, start, end);
    let inv_span = 1.0 / max(span, 1e-3);

    var transmittance = 1.0;
    var luminance = vec3(0.0);
    var dist_sum = 0.0;
    var weight_sum = 0.0;
    // Empty-space skipping (Schneider 2015): outside cloud, march at twice
    // the step with the cheap (unlit, uneroded) density; on a hit, step back
    // and march at full rate until a run of empty samples.
    var coarse = true;
    var empty_run = 0u;
    var guard = 0u;
    while has_low && t < end && transmittance >= 0.01 && guard < clouds.max_steps * 2u {
        guard += 1u;
        let p = ro + rd * t;
        let pc = p - clouds.planet_center;
        let r = length(pc);
        let f = (t - start) * inv_span;
        let a = ray_local_a(ray, f, p);
        // The local layer (base, thickness, density, erosion): map layer B.
        let b = ray_local_b(ray_heights, f, p);
        // The step: a twentieth of the local layer (fine in a thin deck, long
        // in an 8 km storm tower) plus the distance term.
        ds = min(0.05 * max(b.y, 1.0) + g * t, ds_max);
        let h = (r - clouds.inner_radius - b.x) / max(b.y, 1.0);
        let w = weather_sample_a(p, a);
        if coarse {
            if cloud_density(p, h, w, b, false) > 0.0 {
                coarse = false;
                empty_run = 0u;
                t = max(t - ds, start);
            } else {
                t += 2.0 * ds;
            }
            continue;
        }
        // Fade the deck over the last quarter of the reach, in the density
        // itself so it stops absorbing as well as glowing.
        let reach = 1.0 - ramp(clouds.far * 0.75, clouds.far, t);
        let density = cloud_density(p, h, w, b, true) * reach;
        if density <= 0.0 {
            empty_run += 1u;
            coarse = empty_run >= 4u;
        } else {
            empty_run = 0u;
            let sigma = density * clouds.extinction;
            let mu_l = dot(pc / r, l);
            let sun_t = sample_transmittance_lut(r, mu_l) * planet_lit(r, mu_l);
            let dark = 1.0 + 2.0 * w.wetness;
            // Lighting LOD: deep in the cloud (little of it still reaches the
            // eye) and far away (a few pixels per cloud) two light samples do.
            let full_light = transmittance > 0.3 && t < clouds.far * 0.15;
            let light_n = select(min(clouds.light_steps, 2u), clouds.light_steps, full_light);
            let od = light_optical_depth(p, l, light_n, w, a, b) * clouds.extinction * dark;
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
        let sun_t = sample_transmittance_lut(r, dot(n_up, l)) * planet_lit(r, dot(n_up, l));
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
    // Distant shafts and far fog (WE4d): below the deck, so in front of it.
    if far_on {
        let fw = far_weather(ro, rd, scene_t, fract(ign(half_px, clouds.frame) + 0.5), light_color, l, cos_t, mix(bounce, sky_up, 0.6));
        if fw.weight > 1e-4 {
            let fw_dist = fw.dist_sum / fw.weight;
            let air = aerial(uv, r_cam, mu_v, rd, fw_dist);
            color = fw.color * air.transmittance + air.inscatter * (1.0 - fw.transmittance) + fw.transmittance * color;
            if 1.0 - fw.transmittance > (1.0 - transmittance) * fw.transmittance {
                dist = fw_dist;
            }
            transmittance *= fw.transmittance;
        }
    }
    out.color = vec4(color * view.exposure, transmittance);
    out.dist = vec2(dist, bank);
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
