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
fn displaced(p: WaterParams, tex: texture_2d_array<f32>, s: sampler, x0: vec2<f32>, camera: vec2<f32>) -> vec4<f32> {
    let d = distance(x0, camera);
    var disp = vec4(0.0);
    let n = u32(p.misc.x);
    for (var c = 0u; c < n; c++) {
        let w = displacement_weight(p, c, d);
        if w > 0.0 {
            disp += w * textureSampleLevel(tex, s, cascade_uv(p, c, x0), i32(c), 0.0);
        }
    }
    let fade = 1.0 - smoothstep(p.misc.z * 0.75, p.misc.z, d);
    // xyz: position; w: displaced height (crest scattering).
    return vec4(x0.x + disp.x * fade, p.clip.w + disp.y * fade, x0.y + disp.z * fade, disp.y * fade);
}
