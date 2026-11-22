// The sun-visibility pass (aerial-shadow.md "Architecture review", FG-12 A2; `sun_visibility.rs`):
// its uniform, the uniform its consumers read, the sample distribution, the jitter and the
// history clip. CPU twin: `sf_atmos::sun_visibility` (change both together).

import super::fog::Fog;

// The march's and the resolve's uniform; field order is the layout: `SunVisUniform`
// (sun_visibility.rs).
struct SunVisPass {
    // Last frame's (jittered) clip-from-world, in last frame's render space.
    prev_clip_from_world: mat4x4<f32>,
    // The camera (render space) and how far the samples reach (m).
    camera: vec3<f32>,
    max_distance: f32,
    // The view axis (the cascades are picked by the distance along it).
    forward: vec3<f32>,
    samples: u32,
    // Toward the sun (world space, unit): the light the far and cloud maps are made for.
    sun_dir: vec3<f32>,
    frame: u32,
    // This frame's render space minus last frame's (floating origin).
    origin_shift: vec3<f32>,
    // History weight (`sky.sun_vis.history`).
    history: f32,
    // The cloud shadow map's basis (cloud_shadow.wesl); `cloud_shadow` 0: none.
    shadow_center: vec3<f32>,
    cloud_shadow: u32,
    shadow_x: vec3<f32>,
    history_valid: u32,
    shadow_y: vec3<f32>,
    // The fog's weights start here: the froxels' far (they shadow the near fog), 0 without them.
    fog_start: f32,
    // The planet (render space) and its radius (the fog's altitude, as the composite's).
    planet_center: vec3<f32>,
    inner_radius: f32,
    // The terrain's shadow-height map: (min x, min z, 1 / extent, base y); z 0: none.
    far_map: vec4<f32>,
    // The far shadow's rule (`sky.far_shadow.*`): penumbra per m, its floor, bias, bias per m.
    far_vis: vec4<f32>,
    // The pass's (half) and the view's internal resolution.
    half_size: vec2<f32>,
    full_size: vec2<f32>,
    // Where the far shadow fades to lit (fraction of the map's half extent).
    far_fade: f32,
    // Where a sky pixel's weights end (the fog's sky distance, at least `max_distance`), m.
    sky_end: f32,
    fog: Fog,
};

// What the composite reads; field order is the layout: `SunVisUseUniform` (sun_visibility.rs).
struct SunVisUse {
    // Toward the sun (world space).
    light_dir: vec3<f32>,
    // 1: the pass ran this frame; the fog's sun term takes its green channel (V_fog).
    enabled: u32,
};

// Below this total weight a mean counts as no weight (lit): `sf_atmos::sun_visibility::MIN_WEIGHT`.
// tunables: const (maths: division guard)
const MIN_WEIGHT: f32 = 1e-12;
// Below this extinction a segment's weight is its linear limit (`atmo_segment`).
// tunables: const (maths: 0/0 guard of the closed form)
const MIN_EXTINCTION: f32 = 1e-12;
// The largest distance the raw target stores (f16), m: farther geometry reprojects from here.
// tunables: const (format: below f16's 65504 maximum)
const MAX_STORED_DISTANCE: f32 = 60000.0;

// The march's distance `x` (0..1) of the way through its samples: `reach · x²`, denser near the camera.
fn sample_distance(x: f32, reach: f32) -> f32 {
    return reach * x * x;
}

// This frame's jitter of half-resolution pixel `px` in [0, 1): R2 over the pixel, R1 over the
// frame (Roberts 2018; `sf_atmos::sun_visibility::jitter`).
fn sun_vis_jitter(px: vec2<u32>, frame: u32) -> f32 {
    let v = 0.5 + f32(px.x) * 0.7548777 + f32(px.y) * 0.5698403 + f32(frame % 4096u) * 0.618034;
    return fract(v);
}

// Clips `prev` to the box `lo..hi` along the line to its centre (Karis 2014; the froxels' rule,
// `sf_atmos::sun_visibility::clip_history`).
fn clip_history2(prev: vec2<f32>, lo: vec2<f32>, hi: vec2<f32>) -> vec2<f32> {
    let c = 0.5 * (hi + lo);
    let a = abs(prev - c) / (0.5 * (hi - lo) + 1e-7);
    let m = max(1.0, max(a.x, a.y));
    return c + (prev - c) / m;
}
