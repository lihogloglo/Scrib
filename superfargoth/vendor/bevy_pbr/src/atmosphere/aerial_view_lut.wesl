import package::{
    atmosphere::{
        bindings::{settings, atmosphere, view},
        functions::{
            sample_density_lut, sample_local_inscattering, uv_to_ray_direction, get_view_position,
            sample_local_inscattering_shadowed,
            MIDPOINT_RATIO, MIN_EXTINCTION, ABSORPTION_DENSITY, SCATTERING_DENSITY,
        },
    }
};
// superfargoth patch (own, LSO: light shaft occlusion, A3 key split): only the key variant
// imports these (the stock link stays byte-identical).
@if(ATMOSPHERE_KEY_SINGLE)
import package::atmosphere::{
    bindings::lights,
    bruneton_functions::ray_intersects_ground,
    functions::{sample_transmittance_lut, sample_scattering_lut},
};


@group(0) @binding(13) var aerial_view_lut_out: texture_storage_3d<rgba16float, write>;

// superfargoth patch (own, AS: aerial shadow), `superfargoth/docs/specs/aerial-shadow.md`: a
// top-down height-field shadow for one directional light (`bevy_pbr::AtmosphereHeightShadow`;
// field order is the layout of `GpuAtmosphereHeightShadow`).
@if(ATMOSPHERE_HEIGHT_SHADOW)
struct AtmosphereHeightShadow {
    // Render space: (min x, min z, 1 / extent, base height); extent 0 = none.
    params: vec4<f32>,
    // Toward the shadowed light (world space).
    light_dir: vec3<f32>,
    penumbra_per_m: f32,
    penumbra_min: f32,
    bias: f32,
    bias_per_m: f32,
    fade_start: f32,
}

@if(ATMOSPHERE_HEIGHT_SHADOW)
@group(0) @binding(14) var<uniform> height_shadow: AtmosphereHeightShadow;
// Texels: (shadow height - base height, occluder distance), meters.
@if(ATMOSPHERE_HEIGHT_SHADOW)
@group(0) @binding(15) var height_shadow_map: texture_2d<f32>;
@if(ATMOSPHERE_HEIGHT_SHADOW)
@group(0) @binding(16) var height_shadow_sampler: sampler;

// The light's visibility at render-space `p`, `dist` meters from the camera: a penumbra widening
// with occluder distance, a bias growing with distance, a fade to lit toward the map's border
// (the rule of the froxels' `far_visibility`, sf_atmos_bevy `froxel_inject.wesl`).
@if(ATMOSPHERE_HEIGHT_SHADOW)
fn height_shadow_visibility(p: vec3<f32>, dist: f32) -> f32 {
    let hs_params = height_shadow.params;
    if hs_params.z <= 0.0 {
        return 1.0;
    }
    let uv = (p.xz - hs_params.xy) * hs_params.z;
    let edge = max(abs(uv.x - 0.5), abs(uv.y - 0.5)) * 2.0;
    if edge >= 1.0 {
        return 1.0;
    }
    let t = textureSampleLevel(height_shadow_map, height_shadow_sampler, uv, 0.0).rg;
    let hs = t.x + hs_params.w;
    let w = t.y * height_shadow.penumbra_per_m + height_shadow.penumbra_min;
    let y = p.y + height_shadow.bias + dist * height_shadow.bias_per_m;
    return mix(smoothstep(hs - w, hs + w, y), 1.0, smoothstep(height_shadow.fade_start, 1.0, edge));
}

// superfargoth patch (own, LSO: light shaft occlusion, A3 key split),
// `superfargoth/docs/specs/aerial-shadow.md` "Architecture review": the view's light shafts
// (`bevy_pbr::AtmosphereLightShafts`; field order is the layout of `GpuAtmosphereLightShafts`) and
// a second LUT of the same size and log encoding holding only the single scattering of the light
// toward `light_shafts.light_dir`, so `render_sky` can mask that light alone (the Rayleigh fill,
// multiple scattering and the other lights stay in `total - key`).
@if(ATMOSPHERE_KEY_SINGLE)
struct AtmosphereLightShafts {
    // Toward the light (world space).
    light_dir: vec3<f32>,
    darkness: f32,
    lobe_g: f32,
}

@if(ATMOSPHERE_KEY_SINGLE)
@group(0) @binding(17) var<uniform> light_shafts: AtmosphereLightShafts;
@if(ATMOSPHERE_KEY_SINGLE)
@group(0) @binding(18) var aerial_view_key_lut_out: texture_storage_3d<rgba16float, write>;

// The single-scattering term of `sample_local_inscattering` for the light(s) whose
// `direction_to_light` is within `dot > 0.99` of `light_shafts.light_dir` (planet shadow, E1b
// scale), times `shadow_visibility` when that light is also the one toward `shadow_dir` (the AS
// height shadow; a zero `shadow_dir` matches none), exactly as the total carries it.
@if(ATMOSPHERE_KEY_SINGLE)
fn sample_key_single_scattering(
    ray_dir: vec3<f32>,
    world_pos: vec3<f32>,
    shadow_dir: vec3<f32>,
    shadow_visibility: f32,
) -> vec3<f32> {
    let local_r = length(world_pos);
    let local_up = normalize(world_pos);
    var single = vec3(0.0);
    for (var light_i: u32 = 0u; light_i < lights.n_directional_lights; light_i++) {
        let light = &lights.directional_lights[light_i];
        if dot((*light).direction_to_light, light_shafts.light_dir) <= 0.99 {
            continue;
        }
        let mu_light = dot((*light).direction_to_light, local_up);
        let neg_LdotV = dot((*light).direction_to_light, ray_dir);
        let transmittance_to_light = sample_transmittance_lut(local_r, mu_light);
        let shadow_factor = transmittance_to_light * f32(!ray_intersects_ground(local_r, mu_light));
        let scattering_coeff = sample_scattering_lut(local_r, neg_LdotV);
        let shadowed = select(1.0, shadow_visibility, dot((*light).direction_to_light, shadow_dir) > 0.99);
        single += (*light).color.rgb * (*light).atmosphere_scatter * shadow_factor * scattering_coeff * shadowed;
    }
    return single;
}

@compute
@workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) idx: vec3<u32>) {
    if any(idx.xy > settings.aerial_view_lut_size.xy) { return; }

    let uv = (vec2<f32>(idx.xy) + 0.5) / vec2<f32>(settings.aerial_view_lut_size.xy);
    let ray_dir = uv_to_ray_direction(uv);
    let world_pos = get_view_position();

    let r = length(world_pos);
    let t_max = settings.aerial_view_lut_max_distance;
    // superfargoth patch (own, AS: aerial shadow): `world_pos` is in atmosphere space (the
    // atmosphere entity's frame: world translated and scaled, never rotated, `get_view_position`)
    // and `ray_dir` in world space, so a sample at atmosphere distance `t` sits at render-space
    // `view.world_position + ray_dir * t / atmo_per_world` (camera-relative: exact in f32, where
    // undoing the planet-centred position would lose metres).
    @if(ATMOSPHERE_HEIGHT_SHADOW)
    let atmo_per_world = length(atmosphere.world_to_atmosphere[0].xyz);

    var prev_t = 0.0;
    var total_inscattering = vec3(0.0);
    var throughput = vec3(1.0);
    // superfargoth patch (own, LSO: light shaft occlusion, A3 key split)
    @if(ATMOSPHERE_KEY_SINGLE)
    var key_inscattering = vec3(0.0);

    for (var slice_i: u32 = 0; slice_i < settings.aerial_view_lut_size.z; slice_i++) {
        for (var step_i: u32 = 0; step_i < settings.aerial_view_lut_samples; step_i++) {
            let t_i = t_max * (f32(slice_i) + ((f32(step_i) + MIDPOINT_RATIO) / f32(settings.aerial_view_lut_samples))) / f32(settings.aerial_view_lut_size.z);
            let dt = (t_i - prev_t);
            prev_t = t_i;

            let sample_pos = world_pos + ray_dir * t_i;
            let local_r = length(sample_pos);
            let local_up = normalize(sample_pos);

            let absorption = sample_density_lut(local_r, ABSORPTION_DENSITY);
            let scattering = sample_density_lut(local_r, SCATTERING_DENSITY);
            let extinction = absorption + scattering;

            let sample_optical_depth = extinction * dt;
            let sample_transmittance = exp(-sample_optical_depth);

            // evaluate one segment of the integral
            @if(!ATMOSPHERE_HEIGHT_SHADOW)
            var inscattering = sample_local_inscattering(scattering, ray_dir, sample_pos);
            // superfargoth patch (own, AS: aerial shadow): the shadowed light's single scattering
            // times the height field's visibility at the sample.
            @if(ATMOSPHERE_HEIGHT_SHADOW)
            let world_t = t_i / atmo_per_world;
            @if(ATMOSPHERE_HEIGHT_SHADOW)
            let visibility = height_shadow_visibility(view.world_position + ray_dir * world_t, world_t);
            @if(ATMOSPHERE_HEIGHT_SHADOW)
            var inscattering = sample_local_inscattering_shadowed(scattering, ray_dir, sample_pos, height_shadow.light_dir, visibility);

            // Analytical integration of the single scattering term in the radiance transfer equation
            let s_int = (inscattering - inscattering * sample_transmittance) / max(extinction, MIN_EXTINCTION);
            total_inscattering += throughput * s_int;
            // superfargoth patch (own, LSO: light shaft occlusion, A3 key split): the key light's
            // single scattering, integrated and attenuated exactly like the total.
            @if(ATMOSPHERE_KEY_SINGLE && !ATMOSPHERE_HEIGHT_SHADOW)
            let key_single = sample_key_single_scattering(ray_dir, sample_pos, vec3(0.0), 1.0);
            @if(ATMOSPHERE_KEY_SINGLE && ATMOSPHERE_HEIGHT_SHADOW)
            let key_single = sample_key_single_scattering(ray_dir, sample_pos, height_shadow.light_dir, visibility);
            @if(ATMOSPHERE_KEY_SINGLE)
            key_inscattering += throughput * (key_single - key_single * sample_transmittance) / max(extinction, MIN_EXTINCTION);

            throughput *= sample_transmittance;
            if all(throughput < vec3(0.001)) {
                break;
            }
        }

        // Store in log space to allow linear interpolation of exponential values between slices
        let log_inscattering = log(max(total_inscattering, vec3(1e-6)));
        textureStore(aerial_view_lut_out, vec3(vec2<u32>(idx.xy), slice_i), vec4(log_inscattering, 0.0));
        // superfargoth patch (own, LSO: light shaft occlusion, A3 key split): same log encoding.
        @if(ATMOSPHERE_KEY_SINGLE)
        textureStore(aerial_view_key_lut_out, vec3(vec2<u32>(idx.xy), slice_i), vec4(log(max(key_inscattering, vec3(1e-6))), 0.0));
    }
}
