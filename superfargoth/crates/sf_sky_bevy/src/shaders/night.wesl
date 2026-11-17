// Sky bodies drawn into the sky (sf_sky_bevy `night.rs`): a procedural star
// field, the moons (textured per phase, or a shaded sphere) with their glow,
// and the sun's glare. One fullscreen triangle at the far plane, depth-tested against the
// scene so it only touches sky pixels; additive onto Bevy's sky, before the
// cloud composite (which then hides all of it behind clouds). Radiances are
// in nits and already carry the atmosphere's transmittance (CPU, per moon
// and per elevation for stars); the sky's own brightness hides the stars
// and moons by day, as in reality.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_render::view::View;

struct GpuMoon {
    direction: vec3<f32>,
    tan_radius: f32,
    right: vec3<f32>,
    occluder: f32,
    up: vec3<f32>,
    textured: u32,
    // Moon-local light direction (x right, y up, z toward the viewer).
    light: vec3<f32>,
    // Opacity of the whole moon: its disk and the stars it hides.
    alpha: f32,
    radiance: vec3<f32>,
    // Lit surface (1) versus a plain disk that only hides the stars (0).
    texture_blend: f32,
    // Glow (sf_sky::Moon::glow): radiance at the disk edge before the
    // texture's hue, the edge's angular radius and the 1/e distance beyond it
    // (radians; 0: no glow).
    glow_radiance: vec3<f32>,
    glow_edge: f32,
    glow_falloff: f32,
}

struct Night {
    moons: array<GpuMoon, 4>,
    sun_direction: vec3<f32>,
    glare_size: f32,
    glare_radiance: vec3<f32>,
    star_density: f32,
    star_lux: f32,
    twinkle: f32,
    time: f32,
    moon_count: u32,
    // Transmittance by elevation, index sqrt(sin(elevation)) * 15.
    transmittance: array<vec4<f32>, 16>,
}

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<uniform> night: Night;
@group(0) @binding(2) var moon0: texture_2d<f32>;
@group(0) @binding(3) var moon1: texture_2d<f32>;
@group(0) @binding(4) var moon2: texture_2d<f32>;
@group(0) @binding(5) var moon3: texture_2d<f32>;
@group(0) @binding(6) var moon_sampler: sampler;

const PI: f32 = 3.14159265;
// The hue grid: 4x4 bilinear taps between texel pairs cover an 8x8 mip once.
const HUE_GRID: u32 = 4u; // tunables: const (layout: covers the 8x8 level exactly)
// A glow term below this share of the frame's white (after exposure) is not
// evaluated: under one 12-bit output step.
const GLOW_NEGLIGIBLE: f32 = 0.000244; // tunables: const (maths: 1/4096, below one 12-bit output step)

fn pcg3d(v_in: vec3<u32>) -> vec3<u32> {
    var v = v_in * 1664525u + 1013904223u;
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    v ^= v >> vec3(16u);
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    return v;
}

fn face_dir(face: u32, uv: vec2<f32>) -> vec3<f32> {
    switch face {
        case 0u: { return normalize(vec3(1.0, uv.x, uv.y)); }
        case 1u: { return normalize(vec3(-1.0, uv.x, uv.y)); }
        case 2u: { return normalize(vec3(uv.x, 1.0, uv.y)); }
        case 3u: { return normalize(vec3(uv.x, -1.0, uv.y)); }
        case 4u: { return normalize(vec3(uv.x, uv.y, 1.0)); }
        default: { return normalize(vec3(uv.x, uv.y, -1.0)); }
    }
}

// One layer of stars: at most one per cell of a cube map with `cells` cells
// per face side. A star is a Gaussian about a pixel wide (`px` radians)
// whose integral is its illuminance, so brightness is resolution independent.
fn star_layer(d: vec3<f32>, cells: f32, presence: f32, scale: f32, power: f32, seed: u32, px: f32) -> vec3<f32> {
    let a = abs(d);
    var face = 0u;
    var uv = vec2(0.0);
    if a.x >= a.y && a.x >= a.z {
        face = select(1u, 0u, d.x > 0.0);
        uv = d.yz / a.x;
    } else if a.y >= a.z {
        face = select(3u, 2u, d.y > 0.0);
        uv = d.xz / a.y;
    } else {
        face = select(5u, 4u, d.z > 0.0);
        uv = d.xy / a.z;
    }
    let g = (uv * 0.5 + 0.5) * cells;
    let cell = floor(g);
    let h = vec3<f32>(pcg3d(vec3<u32>(vec2<u32>(cell), face * 7919u + seed))) / 4294967295.0;
    if h.x > presence {
        return vec3(0.0);
    }
    // Kept off the cell edges so one cell's star never needs its neighbors.
    let star_uv = (cell + 0.25 + 0.5 * h.yz) / cells * 2.0 - 1.0;
    let s = face_dir(face, star_uv);
    let theta2 = dot(d - s, d - s);
    let sigma2 = px * px;
    let e = night.star_lux * scale * pow(fract(h.y * 7.31 + h.z * 3.17), power);
    let elevation = max(d.y, 0.0);
    let twinkle = 1.0 + night.twinkle * (1.0 - elevation) * (1.0 - elevation)
        * sin(night.time * (4.0 + 6.0 * h.z) + 40.0 * h.y);
    let tint = mix(vec3(1.0, 0.82, 0.66), vec3(0.78, 0.88, 1.0), h.z);
    return tint * (e * twinkle / (PI * sigma2)) * exp(-theta2 / sigma2);
}

fn sky_transmittance(d: vec3<f32>) -> vec3<f32> {
    let x = sqrt(clamp(d.y, 0.0, 1.0)) * 15.0;
    let i = u32(floor(x));
    let j = min(i + 1u, 15u);
    return mix(night.transmittance[i].rgb, night.transmittance[j].rgb, fract(x));
}

fn sample_moon_level(i: u32, uv: vec2<f32>, level: f32) -> vec4<f32> {
    switch i {
        case 0u: { return textureSampleLevel(moon0, moon_sampler, uv, level); }
        case 1u: { return textureSampleLevel(moon1, moon_sampler, uv, level); }
        case 2u: { return textureSampleLevel(moon2, moon_sampler, uv, level); }
        default: { return textureSampleLevel(moon3, moon_sampler, uv, level); }
    }
}

fn sample_moon(i: u32, uv: vec2<f32>) -> vec4<f32> {
    return sample_moon_level(i, uv, 0.0);
}

// (base width, mip count) of moon `i`'s bound texture.
fn moon_texture_size(i: u32) -> vec2<u32> {
    switch i {
        case 0u: { return vec2(textureDimensions(moon0).x, textureNumLevels(moon0)); }
        case 1u: { return vec2(textureDimensions(moon1).x, textureNumLevels(moon1)); }
        case 2u: { return vec2(textureDimensions(moon2).x, textureNumLevels(moon2)); }
        default: { return vec2(textureDimensions(moon3).x, textureNumLevels(moon3)); }
    }
}

// The glow's hue: the bound phase texture's average colour, alpha-weighted
// (the transparent margin adds nothing) and normalised to luminance 1. Read
// at the level nearest 8x8 (Morrowind's phase DDS stop there: 512 with 7
// levels, 256 with 6), or the smallest the texture has, on a 4x4 tap grid.
// Dark texels (the unlit side of a phase) weigh little, so the lit part sets
// the hue; a black texture (new moon) falls back to white, and its glow is
// ~0 anyway (no light at new moon).
fn moon_hue(i: u32) -> vec3<f32> {
    let size = moon_texture_size(i);
    let level = min(size.y - 1u, max(firstLeadingBit(max(size.x, 1u)), 3u) - 3u);
    var sum = vec3(0.0);
    for (var y = 0u; y < HUE_GRID; y += 1u) {
        for (var x = 0u; x < HUE_GRID; x += 1u) {
            let uv = (vec2(f32(x), f32(y)) + 0.5) / f32(HUE_GRID);
            let t = sample_moon_level(i, uv, f32(level));
            sum += t.rgb * t.a;
        }
    }
    let l = dot(sum, vec3(0.2126, 0.7152, 0.0722));
    return select(vec3(1.0), sum / l, l > 1e-6);
}

@fragment
fn night_sky(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let ndc = vec2(in.uv.x * 2.0 - 1.0, 1.0 - in.uv.y * 2.0);
    let near = view.world_from_clip * vec4(ndc, 1.0, 1.0);
    let d = normalize(near.xyz / near.w - view.world_position);
    // Angular size of a pixel (radians), for star footprints and disk edges.
    let px = max(length(fwidth(d)), 1e-5);

    var color = vec3(0.0);
    if night.star_lux > 0.0 && d.y > -0.02 {
        var stars = star_layer(d, 48.0 * night.star_density, 0.35, 1.0, 3.0, 11u, px);
        stars += star_layer(d, 160.0 * night.star_density, 0.5, 0.12, 2.0, 23u, px);
        stars += star_layer(d, 420.0 * night.star_density, 0.6, 0.025, 1.5, 37u, px);
        color = stars * sky_transmittance(d);
    }

    for (var i = 0u; i < night.moon_count; i += 1u) {
        let m = night.moons[i];
        let c = dot(d, m.direction);
        if c <= 0.0 || m.alpha <= 0.0 {
            continue;
        }
        // Gnomonic (tangent plane) offset: |q| = tan(angle from the centre).
        let q = vec2(dot(d, m.right), dot(d, m.up)) / c;
        // Glow, under the disk (the disk hides it where it is opaque):
        // exponential beyond the disk edge, flat inside it.
        if m.glow_falloff > 0.0 {
            let x = max(atan(length(q)) - m.glow_edge, 0.0) / m.glow_falloff;
            let g = m.glow_radiance * exp(-x);
            if max(g.r, max(g.g, g.b)) * view.exposure > GLOW_NEGLIGIBLE {
                var hue = vec3(1.0);
                if m.textured != 0u {
                    hue = moon_hue(i);
                }
                color += g * hue;
            }
        }
        let p = q / m.tan_radius;
        let r = length(p);
        let aa = px / m.tan_radius;
        if r > 1.0 + aa {
            continue;
        }
        // The whole disk hides what is behind it (stars, farther moons).
        let occ = (1.0 - smoothstep(m.occluder - aa, m.occluder + aa, r)) * m.alpha;
        var lit = vec3(0.0);
        var alpha = 0.0;
        if m.textured != 0u {
            let t = sample_moon(i, vec2(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5));
            lit = t.rgb * m.radiance * m.texture_blend;
            alpha = t.a * m.alpha;
        } else {
            let n = vec3(p, sqrt(max(1.0 - r * r, 0.0)));
            let shade = smoothstep(-0.04, 0.04, dot(n, m.light));
            alpha = (1.0 - smoothstep(1.0 - aa, 1.0 + aa, r)) * m.alpha;
            lit = m.radiance * shade * m.texture_blend;
        }
        color = color * (1.0 - max(occ, alpha)) + lit * alpha;
    }

    if night.glare_size > 0.0 {
        let theta = acos(clamp(dot(d, night.sun_direction), -1.0, 1.0));
        color += night.glare_radiance * exp(-theta / night.glare_size);
    }
    return vec4(color * view.exposure, 0.0);
}
