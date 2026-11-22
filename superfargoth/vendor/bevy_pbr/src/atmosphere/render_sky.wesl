import package::atmosphere::{
    bindings::{view, settings, atmosphere_lut_sampler},
    bruneton_functions::sample_transmittance_lut_segment,
    functions::{
        sample_transmittance_lut,
        sample_sky_view_lut, direction_world_to_atmosphere,
        uv_to_ray_direction, uv_to_ndc, sample_aerial_view_lut,
        sample_sun_radiance, ndc_to_camera_dist, raymarch_atmosphere, 
        get_view_position, max_atmosphere_distance, airglow
    },
};

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@if(DUAL_SOURCE_BLENDING)
enable dual_source_blending;

@if(MULTISAMPLED)
@group(0) @binding(13) var depth_texture: texture_depth_multisampled_2d;
@else
@group(0) @binding(13) var depth_texture: texture_depth_2d;

// superfargoth patch (own, LSO: light shaft occlusion), `superfargoth/docs/specs/aerial-shadow.md`
// "S3": a screen-space light shaft occlusion (`bevy_pbr::AtmosphereLightShafts`; field order is the
// layout of `GpuAtmosphereLightShafts`). `light_shafts_occlusion.r` = how much of the way from the
// pixel to the light is open sky (Mitchell 2007, UE "Light Shaft Occlusion").
@if(ATMOSPHERE_LIGHT_SHAFTS)
struct AtmosphereLightShafts {
    // Toward the light (world space).
    light_dir: vec3<f32>,
    darkness: f32,
    lobe_g: f32,
}

@if(ATMOSPHERE_LIGHT_SHAFTS)
@group(0) @binding(14) var<uniform> light_shafts: AtmosphereLightShafts;
@if(ATMOSPHERE_LIGHT_SHAFTS)
@group(0) @binding(15) var light_shafts_occlusion: texture_2d<f32>;
// A3 key split: the aerial view LUT's twin holding only the single scattering of the light toward
// `light_shafts.light_dir` (`aerial_view_lut.wesl` `ATMOSPHERE_KEY_SINGLE`, same log encoding).
@if(ATMOSPHERE_LIGHT_SHAFTS)
@group(0) @binding(16) var aerial_view_key_lut: texture_3d<f32>;

// `functions.wesl` `sample_aerial_view_lut` on the key LUT: same uvw, first-slice fade and `exp`.
@if(ATMOSPHERE_LIGHT_SHAFTS)
fn sample_aerial_view_key_lut(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let t_max = settings.aerial_view_lut_max_distance;
    let num_slices = f32(settings.aerial_view_lut_size.z);
    let uvw = vec3(uv, saturate(t / t_max - 0.5 / num_slices));
    let sample = textureSampleLevel(aerial_view_key_lut, atmosphere_lut_sampler, uvw, 0.0);
    let t_slice = t_max / num_slices;
    let fade = saturate(t / t_slice);
    return exp(sample.rgb) * fade;
}

// A3: the aerial in-scattering with the key light's single scattering masked by `occlusion`
// (1 = open): `total - (1 - occlusion) * min(key_single, total)`. Never below the unmasked rest
// `max(total - key_single, 0)` and never above `total` (Rayleigh fill, multiple scattering, the
// moon and every other light are untouched, so no dark disc where the light hides).
@if(ATMOSPHERE_LIGHT_SHAFTS)
fn mask_key_single(total: vec3<f32>, key_single: vec3<f32>, occlusion: f32) -> vec3<f32> {
    return total - (1.0 - clamp(occlusion, 0.0, 1.0)) * min(key_single, total);
}

// The key light's visibility on the pixel: `1 - darkness * (1 - o) * lobe`, `lobe` a forward lobe
// toward the light (Henyey-Greenstein normalised to 1 at the light; exactly 1 everywhere at
// `lobe_g` 0, so a per-pixel visibility texture passes through as `1 - darkness * (1 - o)`).
// CPU twin: `sf_atmos::shafts::{lobe, occlusion}`.
@if(ATMOSPHERE_LIGHT_SHAFTS)
fn light_shaft_occlusion(uv: vec2<f32>, ray_dir_ws: vec3<f32>) -> f32 {
    if light_shafts.darkness <= 0.0 {
        return 1.0;
    }
    let open = textureSampleLevel(light_shafts_occlusion, atmosphere_lut_sampler, uv, 0.0).r;
    let g = clamp(light_shafts.lobe_g, -0.999, 0.999);
    let cos_t = clamp(dot(ray_dir_ws, light_shafts.light_dir), -1.0, 1.0);
    let lobe = min(pow((1.0 - g) * (1.0 - g) / max((1.0 - g) * (1.0 - g) + 2.0 * g * (1.0 - cos_t), 1e-6), 1.5), 1.0);
    return clamp(1.0 - clamp(light_shafts.darkness, 0.0, 1.0) * (1.0 - clamp(open, 0.0, 1.0)) * lobe, 0.0, 1.0);
}

struct RenderSkyOutput {
    @if(DUAL_SOURCE_BLENDING)
    @location(0) @blend_src(0) inscattering: vec4<f32>,
    @if(DUAL_SOURCE_BLENDING)
    @location(0) @blend_src(1) transmittance: vec4<f32>,
    @else
    @location(0) inscattering: vec4<f32>,
}

@fragment
fn main(in: FullscreenVertexOutput) -> RenderSkyOutput {
    let depth = textureLoad(depth_texture, vec2<i32>(in.position.xy), 0);

    let ray_dir_ws = uv_to_ray_direction(in.uv);
    let world_pos = get_view_position();
    let r = length(world_pos);
    let up = normalize(world_pos);
    let mu = dot(ray_dir_ws, up);
    let max_samples = settings.sky_max_samples;
    let should_raymarch = settings.rendering_method == 1u;

    var transmittance: vec3<f32>;
    var inscattering: vec3<f32>;

    let sun_radiance = sample_sun_radiance(ray_dir_ws);

    if depth == 0.0 {
        let ray_dir_as = direction_world_to_atmosphere(ray_dir_ws);
        transmittance = sample_transmittance_lut(r, mu);
        inscattering = sample_sky_view_lut(r, ray_dir_as);
        if should_raymarch {
            let t_max = max_atmosphere_distance(r, mu);
            let result = raymarch_atmosphere(world_pos, ray_dir_ws, t_max, max_samples, in.uv, true);
            // superfargoth patch (own, N2: sky glow): as the sky-view LUT carries it.
            inscattering = result.inscattering + airglow(r, mu);
            transmittance = result.transmittance;
        }
        inscattering += sun_radiance * transmittance;
    } else {
        let t = ndc_to_camera_dist(vec3(uv_to_ndc(in.uv), depth));
        inscattering = sample_aerial_view_lut(in.uv, t);
        transmittance = sample_transmittance_lut_segment(r, mu, t);
        if should_raymarch {
            let result = raymarch_atmosphere(world_pos, ray_dir_ws, t, max_samples, in.uv, false);
            inscattering = result.inscattering;
            transmittance = result.transmittance;
        }
        // superfargoth patch (own, LSO: light shaft occlusion, A3 key split): only the key
        // light's single scattering is masked where the scene hides that light (geometry pixels
        // only; the sky is unchanged). The raymarched path (`rendering_method` 1, unused here)
        // has no key split and stays unmasked.
        @if(ATMOSPHERE_LIGHT_SHAFTS)
        {
            if !should_raymarch {
                let key_single = sample_aerial_view_key_lut(in.uv, t);
                inscattering = mask_key_single(inscattering, key_single, light_shaft_occlusion(in.uv, ray_dir_ws));
            }
        }
    }

    // exposure compensation
    inscattering *= view.exposure;
    
    @if(DUAL_SOURCE_BLENDING)
    return RenderSkyOutput(vec4(inscattering, 0.0), vec4(transmittance, 1.0));
    @if(!DUAL_SOURCE_BLENDING)
    let mean_transmittance = (transmittance.r + transmittance.g + transmittance.b) / 3.0;
    @if(!DUAL_SOURCE_BLENDING)
    return RenderSkyOutput(vec4(inscattering, mean_transmittance));
    
}
