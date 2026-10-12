// Froxel volumetric fog (weather.md "Fog", F3): the uniform, the slice
// distribution and the volume lookup, shared by the inject and integrate
// compute passes and the composite. CPU twin: `sf_atmos::froxel` (change
// both together). Froxels are screen tiles × exponential slices of distance
// along the view ray (not view z), so a pixel's slice is a function of its
// own distance.

import super::fog::Fog;

// A point light lit into the fog (the nearest ones, chosen on the CPU).
struct FogLight {
    // Render-space position, range (m).
    position_range: vec4<f32>,
    // Linear colour × luminous intensity (cd: lm / 4π), as Bevy lights it.
    color: vec4<f32>,
};

// Keep in step with `sf_atmos::MAX_FOG_LIGHTS`.
const MAX_FOG_LIGHTS: u32 = 32u;

// Field order is the layout: it matches `FroxelUniform` (render.rs).
struct Froxels {
    world_from_clip: mat4x4<f32>,
    prev_clip_from_world: mat4x4<f32>,
    camera: vec3<f32>,
    exposure: f32,
    prev_camera: vec3<f32>,
    near: f32,
    forward: vec3<f32>,
    far: f32,
    origin_shift: vec3<f32>,
    history: f32,
    // Tiles across, down, slices (as floats).
    size: vec3<f32>,
    frame: u32,
    planet_center: vec3<f32>,
    inner_radius: f32,
    // The cloud shadow map's basis (cloud_shadow.wesl); `cloud_shadow` 0: none.
    shadow_center: vec3<f32>,
    cloud_shadow: u32,
    shadow_x: vec3<f32>,
    history_valid: u32,
    shadow_y: vec3<f32>,
    n_lights: u32,
    // The terrain's shadow-height map: (min x, min z, 1 / extent, base y); z 0: none.
    far_map: vec4<f32>,
    // This frame's sample offset inside each froxel, 0..1.
    jitter: vec3<f32>,
    // 1: the froxels run; 0: the analytic layers fog everything.
    enabled: u32,
    // The shadowed directional light the cloud and far maps were made for.
    shadow_light: vec3<f32>,
    pad: f32,
    fog: Fog,
    lights: array<FogLight, MAX_FOG_LIGHTS>,
};

// Distance of slice coordinate `z` (0..1) along the view ray.
fn slice_distance(fx: Froxels, z: f32) -> f32 {
    return fx.near * pow(fx.far / fx.near, z);
}

// Inverse of `slice_distance`, clamped to 0..1.
fn slice_coord(fx: Froxels, d: f32) -> f32 {
    if d <= fx.near {
        return 0.0;
    }
    return clamp(log(d / fx.near) / log(fx.far / fx.near), 0.0, 1.0);
}

// The view ray through screen `uv` (0,0 top left).
fn froxel_ray(fx: Froxels, uv: vec2<f32>) -> vec3<f32> {
    let p = fx.world_from_clip * vec4(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 1.0, 1.0);
    return normalize(p.xyz / p.w - fx.camera);
}

// The integrated fog (rgb: in-scattered light, pre-exposed; a:
// transmittance) between the camera and distance `d` (≤ `far`) through
// screen `uv`. Texel s of the integrated volume holds the integral to the
// far boundary of slice s. Transparent materials call this too.
fn froxel_fog(fx: Froxels, volume: texture_3d<f32>, samp: sampler, uv: vec2<f32>, d: f32) -> vec4<f32> {
    let n = fx.size.z;
    let c = slice_coord(fx, d) * n;
    let v = textureSampleLevel(volume, samp, vec3(uv, max(c - 0.5, 0.5) / n), 0.0);
    // Inside the first slice: from nothing at the camera.
    return mix(vec4(0.0, 0.0, 0.0, 1.0), v, saturate(c));
}
