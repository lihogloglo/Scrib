// Sky bodies drawn into the sky (sf_sky_bevy `night.rs`): a procedural star
// field, the moons (textured per phase, or a shaded sphere) and the sun's
// glare. One fullscreen triangle at the far plane, depth-tested against the
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
    _pad: f32,
    radiance: vec3<f32>,
    _pad2: f32,
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

fn sample_moon(i: u32, uv: vec2<f32>) -> vec4<f32> {
    switch i {
        case 0u: { return textureSampleLevel(moon0, moon_sampler, uv, 0.0); }
        case 1u: { return textureSampleLevel(moon1, moon_sampler, uv, 0.0); }
        case 2u: { return textureSampleLevel(moon2, moon_sampler, uv, 0.0); }
        default: { return textureSampleLevel(moon3, moon_sampler, uv, 0.0); }
    }
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
        if c <= 0.0 {
            continue;
        }
        let p = vec2(dot(d, m.right), dot(d, m.up)) / (c * m.tan_radius);
        let r = length(p);
        let aa = px / m.tan_radius;
        if r > 1.0 + aa {
            continue;
        }
        // The whole disk hides what is behind it (stars, farther moons).
        let occ = 1.0 - smoothstep(m.occluder - aa, m.occluder + aa, r);
        var lit = vec3(0.0);
        var alpha = 0.0;
        if m.textured != 0u {
            let t = sample_moon(i, vec2(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5));
            lit = t.rgb * m.radiance;
            alpha = t.a;
        } else {
            let n = vec3(p, sqrt(max(1.0 - r * r, 0.0)));
            let shade = smoothstep(-0.04, 0.04, dot(n, m.light));
            alpha = 1.0 - smoothstep(1.0 - aa, 1.0 + aa, r);
            lit = m.radiance * shade;
        }
        color = color * (1.0 - max(occ, alpha)) + lit * alpha;
    }

    if night.glare_size > 0.0 {
        let theta = acos(clamp(dot(d, night.sun_direction), -1.0, 1.0));
        color += night.glare_radiance * exp(-theta / night.glare_size);
    }
    return vec4(color * view.exposure, 0.0);
}
