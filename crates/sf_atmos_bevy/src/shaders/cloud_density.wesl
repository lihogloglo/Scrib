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

// Map layer A at the origin-relative `p`: one bilinear fetch, clamped to the
// window's edge texels (past the map: the edge weather).
fn map_a(p: vec3<f32>) -> vec4<f32> {
    let local = clamp(p.xz * clouds.map_inv_texel + clouds.map_offset, vec2(0.5), vec2(clouds.map_size - 0.5));
    let uv = (local + clouds.map_wrap) / clouds.map_size;
    return textureSampleLevel(weather_map, noise_sampler, uv, 0, 0.0);
}

// The uniform weather as a layer-A texel (no map, or a uniform day).
fn uniform_a() -> vec4<f32> {
    return vec4(clouds.coverage, clouds.cloud_type, clouds.wetness, clouds.storm);
}

// Layer A at `p` for one density sample: the map per sample (High/Ultra),
// else the uniform weather (per-ray callers lerp `ray_a` themselves).
fn local_a(p: vec3<f32>) -> vec4<f32> {
    if clouds.map_on == 1u {
        return map_a(p);
    }
    return uniform_a();
}

// Per-ray lookup (Medium/Low): layer A at the segment's start, middle and
// end; the uniform weather when the map is off or sampled per sample.
fn ray_a(ro: vec3<f32>, rd: vec3<f32>, start: f32, end: f32) -> array<vec4<f32>, 3> {
    if clouds.map_on != 2u {
        let u = uniform_a();
        return array(u, u, u);
    }
    return array(map_a(ro + rd * start), map_a(ro + rd * (0.5 * (start + end))), map_a(ro + rd * end));
}

// Layer A at fraction `f` (0..1) along the segment of `ray_a`, or per sample at `p`.
fn ray_local_a(ray: array<vec4<f32>, 3>, f: f32, p: vec3<f32>) -> vec4<f32> {
    if clouds.map_on == 1u {
        return map_a(p);
    }
    let g = saturate(f) * 2.0;
    if g < 1.0 {
        return mix(ray[0], ray[1], g);
    }
    return mix(ray[1], ray[2], g - 1.0);
}

// `p` is an origin-relative world position; the offsets carry the origin
// phase and the wind, wrapped per texture on the CPU.
fn weather_sample(p: vec3<f32>) -> WeatherSample {
    return weather_sample_a(p, local_a(p));
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

// Height fraction through the shell at `p`.
fn height_fraction(p: vec3<f32>) -> f32 {
    return (length(p - clouds.planet_center) - clouds.inner_radius - clouds.base) / clouds.thickness;
}

// Density at `p` (height fraction `h`). `detailed` off skips the erosion
// sample: light and shadow marches don't need it.
fn cloud_density(p: vec3<f32>, h: f32, w: WeatherSample, detailed: bool) -> f32 {
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
            h * clouds.thickness / clouds.detail_tile + clouds.rise * 3.0,
            p.z / clouds.detail_tile + clouds.detail_off.y,
        );
        let detail = textureSampleLevel(detail_texture, noise_sampler, duv, 0.0).r;
        d = erode_density(d, detail, h, clouds.erosion);
    }
    return saturate(d) * clouds.density;
}

// The high (cirrus) layer: a thin 2-D sheet on the sphere at
// `cirrus_height`, one fetch of the weather map's A channel (streaks along
// world X). Returns (distance along the ray, density 0..1); density 0 when
// the ray misses the sheet or there is no cirrus.
fn cirrus_hit(ro: vec3<f32>, rd: vec3<f32>) -> vec2<f32> {
    if clouds.cirrus <= 0.0 {
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
    return vec2(t, cirrus_at(field, clouds.cirrus));
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
