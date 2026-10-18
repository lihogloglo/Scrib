// sf_water_bevy: what the water draw and the velocity redraw share (water.md "Mesh",
// "Floating origin"): the clipmap's vertex placement and the cascade lookups.

struct WaterParams {
    phase01: vec4<f32>,
    phase23: vec4<f32>,
    sizes: vec4<f32>,
    lambda_min: vec4<f32>,
    absorption: vec4<f32>,
    scattering: vec4<f32>,
    aerial: vec4<f32>,
    froxel: vec4<f32>,
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
    // (time s, shore foam band m, 0, shore foam of the body's own look).
    misc2: vec4<f32>,
    // Optics map: render-space XZ of cell (0, 0)'s centre, 1 / cell size, on 0/1.
    regions: vec4<f32>,
    // Analytic height fog when the froxels are off: (extinction at base 1/m, 1 / falloff m,
    // base render-space Y, on 0/1), and its in-scattered radiance (rgb, before exposure).
    fog_height: vec4<f32>,
    fog_color: vec4<f32>,
    // Optics table for the map, 3 vec4 per entry: (absorption, whitecaps), (scattering, contact
    // depth), (roughness, shore foam, 0, 0).
    looks: array<vec4<f32>, 24>,
}

// Shore field sample at render-space `xz`: (signed distance to the shoreline m, openness); open
// deep water outside the field or without one.
fn shore_at(p: WaterParams, tex: texture_2d<f32>, s: sampler, xz: vec2<f32>) -> vec2<f32> {
    if p.waves.w < 0.5 {
        return vec2(1e4, 1.0);
    }
    let uv = (xz - p.shore.xy) * p.shore.zw;
    if any(uv < vec2(0.0)) || any(uv > vec2(1.0)) {
        return vec2(p.shore2.x, 1.0);
    }
    let t = textureSampleLevel(tex, s, uv, 0.0).rg;
    let e = t.r * 2.0 - 1.0;
    return vec2(sign(e) * e * e * p.shore2.x, t.g);
}

// (swell, chop) amplitude scales at a shore sample (`sf_water::ShoreField::swell_scale` and
// `chop_scale` are the CPU twins), times the body's own scales.
fn shore_scales(p: WaterParams, sh: vec2<f32>) -> vec2<f32> {
    let swell = sh.y * smoothstep(0.0, p.shore2.y, sh.x);
    let chop = (p.shore2.w + (1.0 - p.shore2.w) * sh.y) * smoothstep(0.0, p.shore2.z, sh.x);
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
