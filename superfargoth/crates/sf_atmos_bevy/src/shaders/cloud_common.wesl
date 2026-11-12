// Cloud model shared by every cloud pass: the uniform, the Nubis density
// math and the lighting terms. The density functions are the GPU twin of
// `sf_atmos::density` (change both together).
//
// Provenance: density, phase, multiple scattering and Beer-powder are ported
// from the owner's Megalophobia (`cloud_density.wgsl`, `cloud_deck.wgsl`),
// MIT OR Apache-2.0 (ARCHITECTURE §10, 2026-10-12). Recipes: Schneider 2015/
// 2017 (Nubis), Hillaire 2016 (analytic step integral, octave multiple
// scattering after Wrenninge).

import super::fog::Fog;

// Field order is the layout: it matches `CloudUniform` (render.rs).
struct Clouds {
    planet_center: vec3<f32>,
    inner_radius: f32,
    ground_albedo: vec3<f32>,
    base: f32,
    wind_step: vec3<f32>,
    thickness: f32,
    origin_shift: vec3<f32>,
    weather_tile: f32,
    weather_off: vec2<f32>,
    shape_off: vec2<f32>,
    detail_off: vec2<f32>,
    shape_tile: f32,
    detail_tile: f32,
    coverage: f32,
    cloud_type: f32,
    density: f32,
    wetness: f32,
    storm: f32,
    erosion: f32,
    extinction: f32,
    ambient: f32,
    ground_bounce: f32,
    powder: f32,
    phase_g: f32,
    ms_falloff: f32,
    far: f32,
    shadow_strength: f32,
    mean_cover: f32,
    min_steps: u32,
    max_steps: u32,
    light_steps: u32,
    ms_octaves: u32,
    frame: u32,
    checkerboard: u32,
    history_valid: u32,
    shadow_steps: u32,
    full_size: vec2<f32>,
    half_size: vec2<f32>,
    trace_size: vec2<f32>,
    shadow_center: vec3<f32>,
    shadow_x: vec3<f32>,
    shadow_y: vec3<f32>,
    light_dir: vec3<f32>,
    prev_clip_from_world: mat4x4<f32>,
    weather2_off: vec2<f32>,
    cirrus_off: vec2<f32>,
    cirrus_tile: f32,
    cirrus: f32,
    cirrus_height: f32,
    cirrus_depth: f32,
    rise: f32,
    shadow_camera: vec3<f32>,
    fog: Fog,
    clouds_on: u32,
    map_on: u32,
    map_size: f32,
    map_inv_texel: f32,
    map_offset: vec2<f32>,
    map_wrap: vec2<f32>,
    shell_base: f32,
    shell_thickness: f32,
    cirrus_max: f32,
    far_weather: u32,
    // The history (`sf_atmos::CloudTemporal`, WE-10 fix 1): fresh weight, variance-clip γ (0: min/max), STBN jitter.
    history_weight: f32,
    clip_gamma: f32,
    blue_noise: u32,
    // 1: the trace classes a checkerboard block by its whole footprint and the resolve falls back to
    // same-population neighbours (S1, `CloudEdgeStable`); 0: the old window and raw sample.
    stable_edges: u32,
    // 1: fog and far weather split at the cloud's depth (WE12-FIX, `FogCloudSplit`); 0: the old order.
    cloud_fog: u32,
    // The far storm's shape (CW-1, `sf_atmos::StormShape`): on, edge and noise (m), rain at sea level, band (m).
    storm_shape: u32,
    storm_edge_m: f32,
    storm_edge_noise_m: f32,
    storm_ground: f32,
    storm_band_m: f32,
    glows: array<vec4<f32>, 4>,
    // 1: the storm cloud shape (S3, `CumulonimbusShape`; `--off cbshape`: 0, the old profile).
    cb_shape: u32,
};

const PI: f32 = 3.14159265;

fn remap(v: f32, lo: f32, hi: f32, nlo: f32, nhi: f32) -> f32 {
    return nlo + (v - lo) * (nhi - nlo) / max(hi - lo, 1e-5);
}

// `smoothstep` with edges that may run backwards.
fn ramp(a: f32, b: f32, x: f32) -> f32 {
    let t = saturate((x - a) / (b - a));
    return t * t * (3.0 - 2.0 * t);
}

fn height_gradient(h: f32, kind: f32) -> f32 {
    let bottom = ramp(0.0, mix(0.07, 0.22, kind), h);
    let top = ramp(1.0, mix(0.5, 0.85, kind), h);
    return bottom * top;
}

fn combine_density(shape_r: f32, shape_g: f32, h: f32, kind: f32, coverage: f32) -> f32 {
    var d = remap(shape_r, shape_g - 1.0, 1.0, 0.0, 1.0);
    d *= height_gradient(h, kind);
    return remap(d, 1.0 - coverage, 1.0, 0.0, 1.0) * coverage;
}

fn erode_density(d: f32, detail: f32, h: f32, erosion: f32) -> f32 {
    let erode = mix(detail, 1.0 - detail, saturate(h * 4.0));
    return remap(d, erode * erosion, 1.0, 0.0, 1.0);
}

// Storm cloud shape (S3, `cbshape`): `sf_atmos::density`'s `CB_*` and the
// `*_shaped` functions. Gated by the local storm scalar `cb` (0: the old
// functions, so calm decks never change).
const CB_TOP_MIN: f32 = 0.35;
const CB_TOP_EXP: f32 = 1.5;
const CB_CELL_SHIFT: f32 = 0.8;
const CB_ANVIL_WIDEN: f32 = 0.15;
const CB_ANVIL_LO: f32 = 0.72;
const CB_ANVIL_RISE: f32 = 0.08;
const CB_ANVIL_COVER: f32 = 0.6;
const CB_RAG_H: f32 = 0.1;
const CB_RAG_BOOST: f32 = 1.0;

// `sf_atmos::shape_height`: the shape volume's vertical coordinate, world altitude in shape tiles in a storm.
fn shape_height(h: f32, thickness: f32, shape_tile: f32, cb: f32) -> f32 {
    if cb <= 0.0 {
        return h;
    }
    return h * (1.0 + cb * (thickness / max(shape_tile, 1.0) - 1.0));
}

fn anvil_lift(h: f32, cb: f32, anvil: f32) -> f32 {
    return cb * anvil * ramp(CB_ANVIL_LO, CB_ANVIL_LO + CB_ANVIL_RISE, h) * ramp(1.0, 0.92, h);
}

// `sf_atmos::storm_gradient`: the top follows coverage (domed cells), plus the anvil layer.
fn storm_gradient(h: f32, kind: f32, coverage: f32, cb: f32, anvil: f32) -> f32 {
    let top_h = 1.0 - cb * (1.0 - (CB_TOP_MIN + (1.0 - CB_TOP_MIN) * pow(saturate(coverage), CB_TOP_EXP)));
    let bottom = ramp(0.0, mix(0.07, 0.22, kind), h);
    let top = ramp(top_h, top_h * mix(0.5, 0.85, kind), h);
    return max(bottom * top, anvil_lift(h, cb, anvil));
}

// `sf_atmos::combine_density_shaped`: the storm profile, the anvil's lent
// coverage and the Nubis anvil bias (Schneider 2017).
fn combine_density_shaped(shape_r: f32, shape_g: f32, h: f32, kind: f32, coverage: f32, cb: f32, anvil: f32) -> f32 {
    if cb <= 0.0 {
        return combine_density(shape_r, shape_g, h, kind, coverage);
    }
    let d = remap(shape_r, shape_g - 1.0, 1.0, 0.0, 1.0) * storm_gradient(h, kind, coverage, cb, anvil);
    let bias = remap(clamp(h, 0.7, 0.8), 0.7, 0.8, 1.0, 1.0 - 0.5 * anvil * cb);
    let cov = pow(saturate(max(coverage, anvil_lift(h, cb, anvil) * CB_ANVIL_COVER)), bias);
    return remap(d, 1.0 - cov, 1.0, 0.0, 1.0) * cov;
}

// `sf_atmos::erode_density_shaped`: the ragged storm base.
fn erode_density_shaped(d: f32, detail: f32, h: f32, erosion: f32, cb: f32) -> f32 {
    if cb <= 0.0 {
        return erode_density(d, detail, h, erosion);
    }
    let rag = cb * ramp(CB_RAG_H, 0.0, h);
    let erode = mix(mix(detail, 1.0 - detail, saturate(h * 4.0)), 1.0 - detail, rag);
    return remap(d, erode * erosion * (1.0 + CB_RAG_BOOST * rag), 1.0, 0.0, 1.0);
}

struct WeatherSample {
    coverage: f32,
    kind: f32,
    wetness: f32,
    // The storm-shape gate `cb` and the anvil's wide cell term (S3); 0 without the storm shape.
    storm: f32,
    anvil: f32,
};

// `sf_atmos::weather_at`: fixed weather noise, thresholded by the weather scalars.
fn weather_at(noise: vec4<f32>, coverage: f32, cloud_type: f32, wetness: f32, storm_amount: f32) -> WeatherSample {
    return weather_at_shaped(noise, coverage, cloud_type, wetness, storm_amount, false);
}

// `sf_atmos::weather_at_shaped`: with `shaped`, storm cells shrink by threshold toward a weaker storm.
fn weather_at_shaped(noise: vec4<f32>, coverage: f32, cloud_type: f32, wetness: f32, storm_amount: f32, shaped: bool) -> WeatherSample {
    let threshold = 1.25 - 1.5 * saturate(coverage);
    let base = ramp(threshold - 0.2, threshold + 0.2, noise.r);
    let s = saturate(storm_amount);
    var storm = s * ramp(0.45, 0.85, noise.b);
    var w: WeatherSample;
    w.storm = 0.0;
    w.anvil = 0.0;
    if shaped {
        let lo = 0.45 + CB_CELL_SHIFT * (1.0 - s);
        storm = ramp(lo, lo + 0.4, noise.b);
        w.storm = s;
        w.anvil = ramp(lo - CB_ANVIL_WIDEN, lo + 0.4 - CB_ANVIL_WIDEN, noise.b);
    }
    w.coverage = max(base, storm);
    w.kind = saturate(cloud_type + (noise.g - 0.5) * 0.4 + storm * 0.5);
    w.wetness = saturate(wetness + storm * 0.5);
    return w;
}

fn henyey_greenstein(g: f32, cos_t: f32) -> f32 {
    let g2 = g * g;
    let d = 1.0 + g2 - 2.0 * g * cos_t;
    return (1.0 - g2) / (4.0 * PI * d * sqrt(max(d, 1e-4)));
}

// Two lobes: a cloud with the sun behind the viewer is bright too.
fn phase(g: f32, cos_t: f32) -> f32 {
    return mix(henyey_greenstein(g, cos_t), henyey_greenstein(-g * 0.5, cos_t), 0.35);
}

// Octave multiple scattering (Wrenninge; Hillaire 2016). `od` is the optical
// depth toward the light. Without it thick cloud renders black.
fn multi_scatter(od: f32, cos_t: f32, g: f32, falloff: f32, octaves: u32) -> f32 {
    var lum = 0.0;
    var attenuation = 1.0;
    var contribution = 1.0;
    var eccentricity = 1.0;
    for (var n = 0u; n < octaves; n += 1u) {
        lum += contribution * phase(g * eccentricity, cos_t) * exp(-od * attenuation);
        attenuation *= falloff;
        contribution *= falloff;
        eccentricity *= falloff;
    }
    return lum;
}

// Beer-powder: a sunlit face darkens at thin edges.
fn powder(density: f32, cos_t: f32, strength: f32) -> f32 {
    let sugar = 1.0 - exp(-density * 4.0);
    return mix(1.0, mix(1.0, sugar, strength), saturate(0.5 - cos_t * 0.5));
}

// Ray-sphere hits (near, far) for a ray starting at `ro` relative to the
// sphere's center; both -1e30 on a miss.
fn ray_sphere(ro: vec3<f32>, rd: vec3<f32>, radius: f32) -> vec2<f32> {
    let r = length(ro);
    let b = dot(ro, rd);
    let c = (r - radius) * (r + radius);
    let disc = b * b - c;
    if disc < 0.0 {
        return vec2(-1e30);
    }
    let s = sqrt(disc);
    return vec2(-b - s, -b + s);
}

// The march segment (start, end) through the cloud shell for a ray from
// `ro` (relative to the planet center); end <= start when there is none.
// Rays that hit the ground stop there, so nothing below the horizon gets cloud.
fn shell_segment(ro: vec3<f32>, rd: vec3<f32>, inner: f32, base: f32, thickness: f32) -> vec2<f32> {
    let rb = inner + base;
    let rt = rb + thickness;
    let r = length(ro);
    let top = ray_sphere(ro, rd, rt);
    if top.y < 0.0 {
        return vec2(0.0, -1.0);
    }
    let bottom = ray_sphere(ro, rd, rb);
    var s = 0.0;
    var e = top.y;
    if r < rb {
        s = bottom.y;
    } else {
        if r >= rt {
            s = max(top.x, 0.0);
        }
        if bottom.x > 0.0 {
            e = bottom.x;
        }
    }
    let ground = ray_sphere(ro, rd, inner);
    if ground.x > 0.0 {
        e = min(e, ground.x);
    }
    return vec2(s, e);
}

// Which pixel of each 2×2 block a checkerboard frame traces.
fn sub_pixel(frame: u32) -> vec2<u32> {
    let i = frame % 4u;
    return vec2(select(0u, 1u, i == 1u || i == 2u), select(0u, 1u, i == 1u || i == 3u));
}

// Interleaved gradient noise (Jimenez 2014), animated per frame for the
// temporal accumulation.
fn ign(p: vec2<f32>, frame: u32) -> f32 {
    let q = p + 5.588238 * f32(frame % 64u);
    return fract(52.9829189 * fract(dot(q, vec2(0.06711056, 0.00583715))));
}

// `sf_atmos::evolve_field`: the cloud field from the weather map's two
// drifting octaves, contrast-restored.
fn evolve_field(a: f32, b: f32) -> f32 {
    return saturate((mix(a, b, 0.35) - 0.5) * 1.35 + 0.5);
}

// `sf_atmos::cirrus_at`: high-layer density from its streak field and cover.
fn cirrus_at(field: f32, cover: f32) -> f32 {
    let c = saturate(cover);
    let t = 1.1 - 1.2 * c;
    return ramp(t - 0.25, t + 0.25, field) * min(c * 4.0, 1.0);
}
