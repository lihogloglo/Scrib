// Screen-space light shaft occlusion (aerial-shadow.md "S3", FG-12; Mitchell, GPU Gems 3 ch. 13;
// UE "Light Shaft Occlusion" / "Light Shaft Bloom"): the passes' uniforms, the open-sky test, the
// bloom's source colour and falloff, and the occlusion the composite applies to the fog's sun term. CPU twin: `sf_atmos::shafts` (change both together). The vendored `render_sky`
// (patch "own, LSO") carries its own copy of `shaft_occlusion` (it cannot import this crate).

// One pass (mask or one radial blur); field order is the layout: `ShaftsPassUniform` (shafts.rs).
struct ShaftsPass {
    // The sun's screen position (UV, y down).
    sun_uv: vec2<f32>,
    // Blur: this pass's step, as a fraction of the way from the pixel to the sun.
    step: f32,
    taps: u32,
    // The view's internal resolution (the depth texels the mask reads).
    full_size: vec2<f32>,
    // Mask: geometry farther than this counts as open sky, meters.
    depth_m: f32,
};

// The light shaft bloom's uniform (S3b); field order is the layout: `ShaftsBloomUniform` (shafts.rs).
struct ShaftsBloom {
    // Added colour per unit of blurred bloom: tint x scale x the off-screen fade x the night gate.
    tint: vec3<f32>,
    // Pre-exposed luminance that must be exceeded to bloom (`sf_atmos::shafts::bloom_color`).
    threshold: f32,
    // Pre-exposed luminance a scene colour is rescaled down to before the threshold.
    max_luminance: f32,
    // Falloff radius around the sun, screen heights (`sf_atmos::shafts::bloom_falloff`).
    radius: f32,
    // The view's width / height.
    aspect: f32,
};

// 1 if a texel with reversed-Z `depth` at viewport `uv` shows sky (depth 0) or geometry farther
// than `depth_m`, else 0 (the mask's and the bloom source's open-sky test).
fn open_depth(depth: f32, uv: vec2<f32>, view_from_clip: mat4x4<f32>, depth_m: f32) -> f32 {
    if depth <= 0.0 {
        return 1.0;
    }
    let ndc = vec3(uv * vec2(2.0, -2.0) + vec2(-1.0, 1.0), depth);
    let p = view_from_clip * vec4(ndc, 1.0);
    return select(0.0, 1.0, length(p.xyz / p.w) > depth_m);
}

// Light shaft bloom's source colour of one pre-exposed scene colour: luminance rescaled down to
// `max_luminance`, then only its excess over `threshold`, hue kept (`sf_atmos::shafts::bloom_color`).
fn bloom_color(rgb_in: vec3<f32>, threshold: f32, max_luminance: f32) -> vec3<f32> {
    let rgb = max(rgb_in, vec3(0.0));
    let l = dot(rgb, vec3(0.2126, 0.7152, 0.0722));
    if l <= 6.1e-5 {
        return vec3(0.0);
    }
    let kept = max(min(l, max(max_luminance, 0.0)) - max(threshold, 0.0), 0.0);
    return rgb * (kept / l);
}

// `(1 - d / radius)^2` around the sun, `d` in screen heights (`sf_atmos::shafts::bloom_falloff`).
fn bloom_falloff(uv: vec2<f32>, sun: vec2<f32>, aspect: f32, radius: f32) -> f32 {
    if radius <= 0.0 {
        return 0.0;
    }
    let w = 1.0 - clamp(length((uv - sun) * vec2(aspect, 1.0)) / radius, 0.0, 1.0);
    return w * w;
}

// The occlusion's uniform; field order is the layout: `bevy_pbr::GpuAtmosphereLightShafts`.
struct LightShafts {
    // Toward the sun (world space).
    light_dir: vec3<f32>,
    // 0 = nothing masked (off, or the sun off screen).
    darkness: f32,
    lobe_g: f32,
};

// The multiplier on the in-scattering of the light toward `light_dir` seen along `ray_dir`:
// `1 - darkness * (1 - o) * lobe`, `o` the blurred mask at `uv` (open sky toward the sun), `lobe`
// Henyey-Greenstein normalised to 1 at the sun (`sf_atmos::shafts::{lobe, occlusion}`).
fn shaft_occlusion(ls: LightShafts, occlusion: texture_2d<f32>, s: sampler, uv: vec2<f32>, ray_dir: vec3<f32>) -> f32 {
    if ls.darkness <= 0.0 {
        return 1.0;
    }
    let open = textureSampleLevel(occlusion, s, uv, 0.0).r;
    let g = clamp(ls.lobe_g, -0.999, 0.999);
    let cos_t = clamp(dot(ray_dir, ls.light_dir), -1.0, 1.0);
    let lobe = min(pow((1.0 - g) * (1.0 - g) / max((1.0 - g) * (1.0 - g) + 2.0 * g * (1.0 - cos_t), 1e-6), 1.5), 1.0);
    return clamp(1.0 - clamp(ls.darkness, 0.0, 1.0) * (1.0 - clamp(open, 0.0, 1.0)) * lobe, 0.0, 1.0);
}
