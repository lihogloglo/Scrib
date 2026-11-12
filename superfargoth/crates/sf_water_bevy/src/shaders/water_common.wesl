// sf_water_bevy: what the water draw and the velocity redraw share (water.md "Mesh",
// "Floating origin"): the clipmap's vertex placement and the cascade lookups.

struct WaterParams {
    phase01: vec4<f32>,
    phase23: vec4<f32>,
    sizes: vec4<f32>,
    lambda_min: vec4<f32>,
    absorption: vec4<f32>,
    scattering: vec4<f32>,
    clip: vec4<f32>,
    foam: vec4<f32>,
    misc: vec4<f32>,
    // (swell scale, chop scale, bounded 0/1, shore field on 0/1).
    waves: vec4<f32>,
    // Render-space XZ rectangle (min x, min z, max x, max z) of a bounded body.
    bounds: vec4<f32>,
    // Shore field: render-space XZ of its corner, 1 / its extent (m).
    shore: vec4<f32>,
    // (range m, swell ramp m, chop ramp m, chop floor).
    shore2: vec4<f32>,
    // (time s, shore foam band m, shore normal floor, shore foam of the body's own look).
    misc2: vec4<f32>,
    // Optics map: render-space XZ of cell (0, 0)'s centre, 1 / cell size, on 0/1.
    regions: vec4<f32>,
    // SSPR: (wave distortion; reflection texel scale, 0 = off for this surface; plane level; 0).
    reflection: vec4<f32>,
    // Shore waves (W-GERSTNER, `sf_water::ShoreWaves`): (ω per train rad/s ×3, previous frame's time s);
    // (offshore amplitude m, swash m, beach slope, band m); (land margin m, variation, noise origin
    // x, z m: the floating origin modulo `sf_water::shore_waves::NOISE_PERIOD`).
    swash: vec4<f32>,
    swash2: vec4<f32>,
    swash3: vec4<f32>,
    // A sea-state transition (W-WIND, `sf_water::ShoreWaves::blend`): (ω of the trains crossfaded
    // from ×3, weight of `swash.xyz` against them; 1 = none).
    swash4: vec4<f32>,
    // Optics table for the map, 3 vec4 per entry: (absorption, whitecaps), (scattering, contact
    // depth), (roughness, shore foam, 0, 0).
    looks: array<vec4<f32>, 24>,
}

// Shore field sample at render-space `xz`: (signed distance to the shoreline m, openness,
// shoreward direction xz; `sf_water::ShoreField::encode_rgba16f`); open deep water outside the
// field or without one.
fn shore_at(p: WaterParams, tex: texture_2d<f32>, s: sampler, xz: vec2<f32>) -> vec4<f32> {
    if p.waves.w < 0.5 {
        return vec4(1e4, 1.0, 0.0, 0.0);
    }
    let uv = (xz - p.shore.xy) * p.shore.zw;
    if any(uv < vec2(0.0)) || any(uv > vec2(1.0)) {
        return vec4(p.shore2.x, 1.0, 0.0, 0.0);
    }
    return textureSampleLevel(tex, s, uv, 0.0);
}

// ---- Shore waves (W-GERSTNER): `sf_water::shore_waves` is the CPU twin and the reference for
// every constant and law (Eckart dispersion over the beach slope, Green's-law shoaling capped by
// the McCowan breaker, finite-depth Gerstner orbits under a no-loop limit, Stockdon swash). ----

const SW_SHARES = vec3(0.5, 0.3, 0.2);
const SW_PHASE_SPREAD = vec3(1.0, -0.7, 1.3);
const SW_BREAKER = 0.78;
const SW_MICHE = 0.44610616;
const SW_STEEPNESS = 0.75;
const SW_MIN_DEPTH = 0.1;
const SW_PHASE_NOISE = 0.8;
const SW_SET_FLOOR = 0.6;
const SW_SKEW = 0.5;
const SW_ZONE = 0.3;
const SW_ECKART_LINEAR = 4.0;
const SW_GRAVITY = 9.81;
const SW_REPEAT = 256.0;
const SW_TAU = 6.2831853;

fn sw_hash(x: u32, y: u32, seed: u32) -> f32 {
    var h = (x * 0x8da6b343u) ^ (y * 0xd8163841u) ^ (seed * 0xcb1ab31fu);
    h ^= h >> 16u;
    h *= 0x7feb352du;
    h ^= h >> 15u;
    h *= 0x846ca68bu;
    h ^= h >> 16u;
    return f32(h >> 8u) / 16777216.0;
}

fn sw_noise(uv: vec2<f32>, cells: u32, seed: u32) -> f32 {
    let g = uv * f32(cells);
    let i = floor(g);
    let f = g - i;
    let x0 = u32(i.x) % cells;
    let y0 = u32(i.y) % cells;
    let x1 = (x0 + 1u) % cells;
    let y1 = (y0 + 1u) % cells;
    let s = f * f * (vec2(3.0) - 2.0 * f);
    let top = sw_hash(x0, y0, seed) + (sw_hash(x1, y0, seed) - sw_hash(x0, y0, seed)) * s.x;
    let bottom = sw_hash(x0, y1, seed) + (sw_hash(x1, y1, seed) - sw_hash(x0, y1, seed)) * s.x;
    return top + (bottom - top) * s.y;
}

fn sw_eckart_closed(u: f32) -> f32 {
    let v = sqrt(tanh(u));
    return atanh(v) + atan(v);
}

fn sw_eckart(u: f32) -> f32 {
    if u < SW_ECKART_LINEAR {
        return sw_eckart_closed(u);
    }
    return sw_eckart_closed(SW_ECKART_LINEAR) + u - SW_ECKART_LINEAR;
}

// (local wave number rad/m, phase Φ rad) of `omega` at depth `h` on slope `beta`.
fn sw_local_wave(omega: f32, h: f32, beta: f32) -> vec2<f32> {
    let k0 = omega * omega / SW_GRAVITY;
    let k = k0 / sqrt(tanh(k0 * max(h, SW_MIN_DEPTH)));
    if h >= SW_MIN_DEPTH {
        return vec2(k, sw_eckart(k0 * h) / beta);
    }
    return vec2(k, (sw_eckart(k0 * SW_MIN_DEPTH) - k * (SW_MIN_DEPTH - h)) / beta);
}

struct ShoreWave {
    // Displacement (x, y, z), m.
    offset: vec3<f32>,
    // (∂y/∂x, ∂y/∂z, ∂Dx/∂x, ∂Dz/∂z), added to the FFT's slopes.
    slope: vec4<f32>,
    // sin of the main train's phase (1 on its crests).
    crest: f32,
}

// The shore waves at Lagrangian render-space `xz` with shore sample `sh` (`shore_at`) at time `t`
// (s mod 256). `lod_k`: trains whose wave number is above it are faded out (the clipmap's or the
// pixel's resolution; a very large value keeps all). Zero outside the band and when off.
fn shore_waves(p: WaterParams, sh: vec4<f32>, xz: vec2<f32>, t: f32, lod_k: f32) -> ShoreWave {
    let to = shore_trains(p, p.swash.xyz, sh, xz, t, lod_k);
    if p.swash4.w >= 1.0 || all(p.swash4.xyz == p.swash.xyz) {
        return to;
    }
    // A sea-state transition (W-WIND): crossfade the trains it comes from (`ShoreWaves::eval`).
    let prior = shore_trains(p, p.swash4.xyz, sh, xz, t, lod_k);
    let w = max(p.swash4.w, 0.0);
    return ShoreWave(mix(prior.offset, to.offset, w), mix(prior.slope, to.slope, w), mix(prior.crest, to.crest, w));
}

// One set of trains with angular frequencies `om` (rad/s) for `shore_waves`.
fn shore_trains(p: WaterParams, om: vec3<f32>, sh: vec4<f32>, xz: vec2<f32>, t: f32, lod_k: f32) -> ShoreWave {
    var out = ShoreWave(vec3(0.0), vec4(0.0), 0.0);
    let d = sh.x;
    let band = p.swash2.w;
    let margin = p.swash3.x;
    if !(d < band && d > -margin) || (p.swash2.x <= 0.0 && p.swash2.y <= 0.0) {
        return out;
    }
    let o = sh.y;
    let dir = sh.zw;
    let beta = p.swash2.z;
    let h = beta * max(d, 0.0);
    let h_edge = beta * band;
    let w = o * (1.0 - smoothstep(0.0, band, d));
    let noise = fract((p.swash3.zw + xz) / 1024.0);
    let fade = 0.5 - 0.5 * cos(SW_TAU * t / SW_REPEAT);
    let n1 = sw_noise(noise, 32u, 1u);
    let sets = n1 + (sw_noise(noise, 32u, 2u) - n1) * fade;
    let variation = p.swash3.y;
    let group = 1.0 - variation * (1.0 - SW_SET_FLOOR) * (1.0 - sets);
    let psi = variation * SW_PHASE_NOISE * (2.0 * sw_noise(noise, 16u, 3u) - 1.0);
    let shoal = p.swash2.x * pow(max(h_edge / max(h, SW_MIN_DEPTH), 1.0), 0.25);
    let breaker = 0.5 * SW_BREAKER * h;
    let env = shoal * breaker / pow(shoal * shoal * shoal * shoal + breaker * breaker * breaker * breaker + 1e-20, 0.25);
    let amp = env * w * group;
    var a = vec3(0.0);
    var k = vec3(0.0);
    var coth = vec3(0.0);
    var theta = vec3(0.0);
    var steep = 0.0;
    let shares = SW_SHARES;
    let spread = SW_PHASE_SPREAD;
    for (var i = 0u; i < 3u; i++) {
        let lw = sw_local_wave(om[i], h, beta);
        let th = tanh(lw.x * max(h, SW_MIN_DEPTH));
        a[i] = min(shares[i] * amp, SW_MICHE * th / lw.x);
        k[i] = lw.x;
        coth[i] = 1.0 / th;
        theta[i] = -(lw.y + om[i] * t + psi * spread[i]);
        steep += lw.x * a[i] / th;
    }
    let q = min(SW_STEEPNESS / max(steep, 1e-6), 1.0);
    var y = 0.0;
    var along = 0.0;
    var dy = 0.0;
    var dalong = 0.0;
    for (var i = 0u; i < 3u; i++) {
        // Resolution fade (the CPU twin keeps every train: lod_k = ∞).
        let r = 1.0 - smoothstep(0.5 * lod_k, lod_k, k[i]);
        let s = sin(theta[i]);
        let c = cos(theta[i]);
        y += r * a[i] * s;
        along += r * q * a[i] * coth[i] * c;
        dy += r * a[i] * k[i] * c;
        dalong -= r * q * a[i] * coth[i] * k[i] * s;
    }
    let phi0 = sw_local_wave(om.x, 0.0, beta).y;
    let theta0 = -(phi0 + om.x * t + psi * SW_PHASE_SPREAD.x);
    let swash = p.swash2.y * o * group * (1.0 - smoothstep(0.0, SW_ZONE * band, d)) * smoothstep(-margin, -0.5 * margin, d);
    y += swash * sin(theta0 - SW_SKEW * sin(theta0));
    out.offset = vec3(dir.x * along, y, dir.y * along);
    out.slope = vec4(dy * dir.x, dy * dir.y, dalong * dir.x * dir.x, dalong * dir.y * dir.y);
    out.crest = sin(theta.x);
    return out;
}

// The shore waves' vertex displacement at Lagrangian `x0`: faded with the camera distance as the
// cascades are (`displacement_weight`: a train fades where the clipmap's ≈ d/48 spacing can no longer
// carry it, at 10-16 wavelengths; `displaced`'s far fade).
fn shore_wave_offset(p: WaterParams, sh: vec4<f32>, x0: vec2<f32>, camera: vec2<f32>, t: f32) -> vec3<f32> {
    let d = distance(x0, camera);
    let lod_k = SW_TAU * 16.0 / max(d, 1e-3);
    let fade = 1.0 - smoothstep(p.misc.z * 0.75, p.misc.z, d);
    return shore_waves(p, sh, x0, t, lod_k).offset * fade;
}

// (swell, chop) amplitude scales at a shore sample (`sf_water::ShoreField::swell_scale` and
// `chop_scale` are the CPU twins), times the body's own scales.
fn shore_scales(p: WaterParams, sh: vec2<f32>) -> vec2<f32> {
    let swell = sh.y * smoothstep(0.0, p.shore2.y, sh.x);
    let chop = (p.shore2.w + (1.0 - p.shore2.w) * sh.y) * smoothstep(0.0, p.shore2.z, sh.x);
    return vec2(swell, chop) * p.waves.xy;
}

// The same for the shading normal (W-SHORE; `sf_water::ShoreField::normal_chop_scale` is the CPU
// twin): the swell fades as above, the chop's shore ramp holds at the normal floor (`misc2.z`) so
// wind wavelets stay where the vertices have settled. Floor 0 = `shore_scales`.
fn shore_normal_scales(p: WaterParams, sh: vec2<f32>) -> vec2<f32> {
    let swell = sh.y * smoothstep(0.0, p.shore2.y, sh.x);
    let chop = (p.shore2.w + (1.0 - p.shore2.w) * sh.y) * max(smoothstep(0.0, p.shore2.z, sh.x), p.misc2.z);
    return vec2(swell, chop) * p.waves.xy;
}

// The amplitude scale of cascade `c` (0-1: swell, 2-3: chop).
fn cascade_scale(c: u32, scales: vec2<f32>) -> f32 {
    return select(scales.y, scales.x, c < 2u);
}

fn ring_spacing(p: WaterParams, ring: u32) -> f32 {
    return p.clip.x * f32(1u << ring);
}

// A ring's centre: the camera snapped to twice the ring's spacing (so a ring's vertices never
// swim, and each finer ring's centre lies on the coarser ring's grid).
fn ring_centre(p: WaterParams, ring: u32, camera: vec2<f32>) -> vec2<f32> {
    let s2 = 2.0 * ring_spacing(p, ring);
    return floor(camera / s2 + 0.5) * s2;
}

// Lagrangian (undisplaced) render-space XZ of clipmap vertex `g` (grid units) of `ring_in`
// (`rings` / `rings + 1`: the skirt's inner / outer vertices).
fn clipmap_xz(p: WaterParams, g: vec2<f32>, ring_in: f32, camera: vec2<f32>) -> vec2<f32> {
    let rings = u32(p.clip.z);
    let half = p.clip.y * 0.5;
    let r = u32(ring_in + 0.5);
    if r >= rings {
        let last = rings - 1u;
        let c = ring_centre(p, last, camera);
        if r == rings {
            return c + g * ring_spacing(p, last);
        }
        return c + normalize(g) * p.misc.y;
    }
    let c = ring_centre(p, r, camera);
    // CDLOD morph: over the outer cells, odd vertices slide onto the next ring's grid, so the
    // edge matches the coarser ring exactly (no cracks, no T-junction pops).
    let edge = max(abs(g.x), abs(g.y));
    let m = saturate((edge - (half - 12.0)) / 10.0);
    let gm = g - fract(g * 0.5) * 2.0 * m;
    return c + gm * ring_spacing(p, r);
}

// True where a finer ring covers this point of `ring` (the fragment is drawn by that one).
fn covered_by_finer(p: WaterParams, ring: u32, xz: vec2<f32>, camera: vec2<f32>) -> bool {
    if ring == 0u || ring >= u32(p.clip.z) {
        return false;
    }
    let inner = ring - 1u;
    let q = abs(xz - ring_centre(p, inner, camera));
    // One fine cell short of its edge: there the finer ring is fully morphed onto this ring's
    // grid (identical surface), so the overlap hides rasterisation seams between the two.
    return max(q.x, q.y) < (p.clip.y * 0.5 - 1.0) * ring_spacing(p, inner);
}

fn cascade_phase(p: WaterParams, c: u32) -> vec2<f32> {
    switch c {
        case 0u: { return p.phase01.xy; }
        case 1u: { return p.phase01.zw; }
        case 2u: { return p.phase23.xy; }
        default: { return p.phase23.zw; }
    }
}

// Texture coordinates of render-space `xz` in cascade `c`: exact under the floating origin
// (`sf_water::origin_phase` holds the integer part of the origin).
fn cascade_uv(p: WaterParams, c: u32, xz: vec2<f32>) -> vec2<f32> {
    return fract((cascade_phase(p, c) + xz) / p.sizes[c]);
}

// Vertex displacement weight of cascade `c` at horizontal distance `d`: a cascade fades out
// where the clipmap's spacing (≈ d/48) can no longer carry its shortest waves. A function of
// distance, not of the ring, so two rings meeting at an edge displace it identically.
fn displacement_weight(p: WaterParams, c: u32, d: f32) -> f32 {
    let l = p.lambda_min[c];
    return 1.0 - smoothstep(10.0 * l, 16.0 * l, d);
}

// The displaced render-space position of Lagrangian point `x0`.
// `scales`: the shore's (swell, chop) scales at x0 (`shore_scales`).
fn displaced(p: WaterParams, tex: texture_2d_array<f32>, s: sampler, x0: vec2<f32>, camera: vec2<f32>, scales: vec2<f32>) -> vec4<f32> {
    let d = distance(x0, camera);
    var disp = vec4(0.0);
    let n = u32(p.misc.x);
    for (var c = 0u; c < n; c++) {
        let w = displacement_weight(p, c, d) * cascade_scale(c, scales);
        if w > 0.0 {
            disp += w * textureSampleLevel(tex, s, cascade_uv(p, c, x0), i32(c), 0.0);
        }
    }
    let fade = 1.0 - smoothstep(p.misc.z * 0.75, p.misc.z, d);
    // xyz: position; w: displaced height (crest scattering).
    return vec4(x0.x + disp.x * fade, p.clip.w + disp.y * fade, x0.y + disp.z * fade, disp.y * fade);
}
