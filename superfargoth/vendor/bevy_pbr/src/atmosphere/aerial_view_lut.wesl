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

            throughput *= sample_transmittance;
            if all(throughput < vec3(0.001)) {
                break;
            }
        }

        // Store in log space to allow linear interpolation of exponential values between slices
        let log_inscattering = log(max(total_inscattering, vec3(1e-6)));
        textureStore(aerial_view_lut_out, vec3(vec2<u32>(idx.xy), slice_i), vec4(log_inscattering, 0.0));
    }
}
