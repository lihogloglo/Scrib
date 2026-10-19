// Cloud density sampling: the uniform and noise bindings (group 1) shared by
// the trace and the cloud shadow map, so both agree on where cloud is.

import super::cloud_common::{Clouds, WeatherSample, combine_density, erode_density, weather_at, evolve_field, cirrus_at, ray_sphere};
import super::fog_bank::fog_bank_mask;

@group(1) @binding(0) var<uniform> clouds: Clouds;
// R Perlin-Worley, G Worley FBM. Tiles every `shape_tile` m; z = height fraction.
@group(1) @binding(1) var shape_texture: texture_3d<f32>;
// Worley FBM, erodes edges. Tiles every `detail_tile` m.
@group(1) @binding(2) var detail_texture: texture_3d<f32>;
// R coverage field, G type field, B storm cells. Tiles every `weather_tile` m.
@group(1) @binding(3) var weather_texture: texture_2d<f32>;
@group(1) @binding(4) var noise_sampler: sampler;
// The world weather map (WE4b, `weather_map.rs`): layers A (coverage, type,
// wetness, storm), B (base, thickness, density, erosion), C (cirrus,
// precipitation, fog density, dimming); world-anchored and toroidal.
@group(1) @binding(5) var weather_map: texture_2d_array<f32>;

// A map layer (0 A, 1 B, 2 C) at the origin-relative `p`: one bilinear
// fetch, clamped to the window's edge texels (past the map: the edge weather).
fn map_layer(p: vec3<f32>, layer: i32) -> vec4<f32> {
    let local = clamp(p.xz * clouds.map_inv_texel + clouds.map_offset, vec2(0.5), vec2(clouds.map_size - 0.5));
    let uv = (local + clouds.map_wrap) / clouds.map_size;
    return textureSampleLevel(weather_map, noise_sampler, uv, layer, 0.0);
}

fn map_a(p: vec3<f32>) -> vec4<f32> {
    return map_layer(p, 0);
}

// The uniform weather as a texel of `layer` (no map, or a uniform day):
// A coverage, type, wetness, storm; B base, thickness, density, erosion; C cirrus.
fn uniform_layer(layer: i32) -> vec4<f32> {
    if layer == 0 {
        return vec4(clouds.coverage, clouds.cloud_type, clouds.wetness, clouds.storm);
    }
    if layer == 1 {
        return vec4(clouds.base, clouds.thickness, clouds.density, clouds.erosion);
    }
    return vec4(clouds.cirrus, 0.0, 0.0, 0.0);
}

// `layer` at `p` for one density sample: the map per sample (High/Ultra),
// else the uniform weather (per-ray callers lerp `ray_layer` themselves).
fn local_layer(p: vec3<f32>, layer: i32) -> vec4<f32> {
    if clouds.map_on == 1u {
        return map_layer(p, layer);
    }
    return uniform_layer(layer);
}

// Per-ray lookup (Medium/Low): `layer` at the segment's start, middle and
// end; the uniform weather when the map is off or sampled per sample.
fn ray_layer(ro: vec3<f32>, rd: vec3<f32>, start: f32, end: f32, layer: i32) -> array<vec4<f32>, 3> {
    if clouds.map_on != 2u {
        let u = uniform_layer(layer);
        return array(u, u, u);
    }
    return array(map_layer(ro + rd * start, layer), map_layer(ro + rd * (0.5 * (start + end)), layer), map_layer(ro + rd * end, layer));
}

fn ray_a(ro: vec3<f32>, rd: vec3<f32>, start: f32, end: f32) -> array<vec4<f32>, 3> {
    return ray_layer(ro, rd, start, end, 0);
}

fn ray_b(ro: vec3<f32>, rd: vec3<f32>, start: f32, end: f32) -> array<vec4<f32>, 3> {
    return ray_layer(ro, rd, start, end, 1);
}

// `layer` at fraction `f` (0..1) along the segment of `ray` (`ray_layer`),
// or per sample at `p`.
fn ray_local(ray: array<vec4<f32>, 3>, f: f32, p: vec3<f32>, layer: i32) -> vec4<f32> {
    if clouds.map_on == 1u {
        return map_layer(p, layer);
    }
    let g = saturate(f) * 2.0;
    if g < 1.0 {
        return mix(ray[0], ray[1], g);
    }
    return mix(ray[1], ray[2], g - 1.0);
}

fn ray_local_a(ray: array<vec4<f32>, 3>, f: f32, p: vec3<f32>) -> vec4<f32> {
    return ray_local(ray, f, p, 0);
}

fn ray_local_b(ray: array<vec4<f32>, 3>, f: f32, p: vec3<f32>) -> vec4<f32> {
    return ray_local(ray, f, p, 1);
}

// `p` is an origin-relative world position; the offsets carry the origin
// phase and the wind, wrapped per texture on the CPU.
fn weather_sample(p: vec3<f32>) -> WeatherSample {
    return weather_sample_a(p, local_layer(p, 0));
}

// The weather at `p` from layer-A weather `a` (coverage, type, wetness, storm).
fn weather_sample_a(p: vec3<f32>, a: vec4<f32>) -> WeatherSample {
    let uv = p.xz / clouds.weather_tile + clouds.weather_off;
    var noise = textureSampleLevel(weather_texture, noise_sampler, uv, 0.0);
    // Evolution: the second octave is the same map, transposed, drifting a
    // little slower and sideways (`sf_atmos::scroll`), so clouds re-form.
    let uv2 = (p.xz / clouds.weather_tile).yx + clouds.weather2_off;
    let second = textureSampleLevel(weather_texture, noise_sampler, uv2, 0.0).r;
    noise.r = evolve_field(noise.r, second);
    return weather_at(noise, a.x, a.y, a.z, a.w);
}

// Height fraction at `p` through the local layer `b` (map layer B: base,
// thickness, density, erosion).
fn height_fraction(p: vec3<f32>, b: vec4<f32>) -> f32 {
    return (length(p - clouds.planet_center) - clouds.inner_radius - b.x) / max(b.y, 1.0);
}

// Density at `p` (height fraction `h` in the local layer `b`). `detailed`
// off skips the erosion sample: light and shadow marches don't need it.
fn cloud_density(p: vec3<f32>, h: f32, w: WeatherSample, b: vec4<f32>, detailed: bool) -> f32 {
    if w.coverage <= 0.0 || h < 0.0 || h > 1.0 {
        return 0.0;
    }
    let suv = vec3(p.x / clouds.shape_tile + clouds.shape_off.x, h + clouds.rise, p.z / clouds.shape_tile + clouds.shape_off.y);
    let shape = textureSampleLevel(shape_texture, noise_sampler, suv, 0.0);
    var d = combine_density(shape.r, shape.g, h, w.kind, w.coverage);
    if d <= 0.0 {
        return 0.0;
    }
    if detailed {
        let duv = vec3(
            p.x / clouds.detail_tile + clouds.detail_off.x,
            h * b.y / clouds.detail_tile + clouds.rise * 3.0,
            p.z / clouds.detail_tile + clouds.detail_off.y,
        );
        let detail = textureSampleLevel(detail_texture, noise_sampler, duv, 0.0).r;
        d = erode_density(d, detail, h, b.w);
    }
    return saturate(d) * b.z;
}

// The high (cirrus) layer: a thin 2-D sheet on the sphere at
// `cirrus_height`, one fetch of the noise texture's A channel (streaks along
// world X), its cover from the world map's layer C at the hit. Returns (distance along the ray, density 0..1); density 0 when
// the ray misses the sheet or there is no cirrus.
fn cirrus_hit(ro: vec3<f32>, rd: vec3<f32>) -> vec2<f32> {
    if clouds.cirrus_max <= 0.0 {
        return vec2(0.0);
    }
    let hit = ray_sphere(ro - clouds.planet_center, rd, clouds.inner_radius + clouds.cirrus_height);
    // From below the sheet the far root is the hit; from above, the near one.
    var t = hit.y;
    if hit.x > 0.0 {
        t = hit.x;
    }
    if t <= 0.0 {
        return vec2(0.0);
    }
    let p = ro + rd * t;
    let uv = p.xz / clouds.cirrus_tile + clouds.cirrus_off;
    let field = textureSampleLevel(weather_texture, noise_sampler, uv, 0.0).a;
    let cover = select(clouds.cirrus, map_layer(p, 2).x, clouds.map_on != 0u);
    return vec2(t, cirrus_at(field, cover));
}

// Optical depth of the cirrus sheet along `l` from the point `p` below it
// (cloud shadow map).
fn cirrus_optical_depth(p: vec3<f32>, l: vec3<f32>) -> f32 {
    let c = cirrus_hit(p, l);
    return c.y * clouds.cirrus_depth / max(l.y, 0.1);
}

// The ground-fog bank mask at `p` (`fog_bank.wesl`, shared with the froxels).
fn bank_mask(p: vec3<f32>) -> f32 {
    return fog_bank_mask(clouds.fog, weather_texture, noise_sampler, p);
}
