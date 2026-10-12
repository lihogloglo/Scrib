// Cloud density sampling: the uniform and noise bindings (group 1) shared by
// the trace and the cloud shadow map, so both agree on where cloud is.

import super::cloud_common::{Clouds, WeatherSample, combine_density, erode_density, weather_at};

@group(1) @binding(0) var<uniform> clouds: Clouds;
// R Perlin-Worley, G Worley FBM. Tiles every `shape_tile` m; z = height fraction.
@group(1) @binding(1) var shape_texture: texture_3d<f32>;
// Worley FBM, erodes edges. Tiles every `detail_tile` m.
@group(1) @binding(2) var detail_texture: texture_3d<f32>;
// R coverage field, G type field, B storm cells. Tiles every `weather_tile` m.
@group(1) @binding(3) var weather_texture: texture_2d<f32>;
@group(1) @binding(4) var noise_sampler: sampler;

// `p` is an origin-relative world position; the offsets carry the origin
// phase and the wind, wrapped per texture on the CPU.
fn weather_sample(p: vec3<f32>) -> WeatherSample {
    let uv = p.xz / clouds.weather_tile + clouds.weather_off;
    let noise = textureSampleLevel(weather_texture, noise_sampler, uv, 0.0);
    return weather_at(noise, clouds.coverage, clouds.cloud_type, clouds.wetness, clouds.storm);
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
    let suv = vec3(p.x / clouds.shape_tile + clouds.shape_off.x, h, p.z / clouds.shape_tile + clouds.shape_off.y);
    let shape = textureSampleLevel(shape_texture, noise_sampler, suv, 0.0);
    var d = combine_density(shape.r, shape.g, h, w.kind, w.coverage);
    if d <= 0.0 {
        return 0.0;
    }
    if detailed {
        let duv = vec3(
            p.x / clouds.detail_tile + clouds.detail_off.x,
            h * clouds.thickness / clouds.detail_tile,
            p.z / clouds.detail_tile + clouds.detail_off.y,
        );
        let detail = textureSampleLevel(detail_texture, noise_sampler, duv, 0.0).r;
        d = erode_density(d, detail, h, clouds.erosion);
    }
    return saturate(d) * clouds.density;
}
