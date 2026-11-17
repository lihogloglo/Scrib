// Rain rings (`sf_render_order_bevy::rain_rings`): procedural raindrop rings on standing water, as
// the slope of the water along X and Z. Shared by every surface rain falls on: the puddles of the
// surface weather layer (`sf_surface_bevy::layer`, W4) and the water bodies (`sf_water_bevy`
// `water_material.wesl`, R1). Here because both crates already import this crate's shader modules
// (`translucent_fog`); neither depends on the other. Also the world-anchored hash grids the layer's
// glints use.
//
// The ring profile (`blip`, `circle`, trailing rings) of files/shaders/lib/water/rain_ripples.glsl
// as Clockwork uses it on puddles: gitlab.com/OursCodeur/openmw branch clockwork-alpha, commit
// c5e5a9a6362c35512823884753d2d5284af204fa (GPL-3.0; THIRD_PARTY_NOTICES.md). Ripple layers by
// rain intensity: S. Lagarde, "Water drop 2b - Dynamic rain and its effects" (2013).
// CPU twin: `sf_surface::layer::{ring, drops, ripples, hash}` (`sf_surface_bevy` tests compare
// the constants).

/// Rain rings (W4): drop cell (m; each grid's cell divides 256), the ring's reach and band (cell
/// units), drops per second per cell, and the drop grids' cell scales.
const RIPPLE_CELL_M: f32 = 1.6;
const RIPPLE_RADIUS: f32 = 0.2;
const RIPPLE_RING: f32 = 0.16666667;
const RIPPLE_RATE: f32 = 1.2;
const RIPPLE_SCALE_0: f32 = 1.0;
const RIPPLE_SCALE_1: f32 = 1.25;
const RIPPLE_SCALE_2: f32 = 0.8;
const RIPPLE_SCALE_3: f32 = 0.625;
/// The floating origin's grid (m): render space shifts by whole cells of it. Every hash cell here
/// divides it, and hash grids wrap to it (`grid_wrap_cell`).
const ORIGIN_GRID_M: f32 = 256.0;

/// A hash grid's cell index wrapped to the origin grid (`ORIGIN_GRID_M` holds `n` cells): render
/// space moves by whole origin cells on a floating-origin shift, so a grid hashed by its wrapped
/// index is world-anchored and keeps its pattern across the shift (UE5's LWC wrap, `LWCWrap`).
fn grid_wrap_cell(c: vec2<i32>, n: i32) -> vec2<i32> {
    return ((c % n) + n) % n;
}

/// Cells of `cell_m` meters in the origin grid (`cell_m` divides it).
fn grid_cells_per_origin(cell_m: f32) -> i32 {
    return i32(round(ORIGIN_GRID_M / cell_m));
}

/// Two hash values in 0..1 for an integer cell and a seed.
fn grid_hash(c: vec2<i32>, seed: u32) -> vec2<f32> {
    var h = (u32(c.x) * 0x8da6b343u) ^ (u32(c.y) * 0xd8163841u) ^ (seed * 0x9e3779b9u);
    h = (h ^ (h >> 15u)) * 0x2c1b3c6du;
    h = (h ^ (h >> 12u)) * 0x297a2d39u;
    h = h ^ (h >> 15u);
    return vec2(f32(h & 0xffffu), f32(h >> 16u)) / 65535.0;
}

/// `rain_ripples.glsl` `blip` and `blipDerivative`.
fn rain_blip(x: f32) -> f32 {
    let y = max(0.0, 1.0 - x * x);
    return y * y * y;
}

fn rain_blip_derivative(x: f32) -> f32 {
    let y = clamp(x, -1.0, 1.0);
    let n = y * y - 1.0;
    return -6.0 * y * n * n;
}

/// One drop's ring slope (`rain_ripples.glsl` `circle`): `to` the point minus the drop (cell
/// units), `phase` the drop's cycle; zero outside the ring's band. Twin: `sf_surface::layer::ring`.
fn rain_ring(to: vec2<f32>, phase: f32) -> vec2<f32> {
    let d = length(to);
    let follower = (phase - d / RIPPLE_RADIUS) / RIPPLE_RING - 1.0;
    if follower < -1.0 || follower > 1.0 {
        return vec2(0.0);
    }
    let energy = 1.0 - phase;
    let range = rain_blip(min(follower, 0.0));
    let v = vec3(to * (-rain_blip_derivative(follower) * 5.0 * energy * energy), 0.5);
    return v.xy * (energy * range / length(v));
}

/// One drop grid at `q` (cell units): each cell drops once a cycle at a random spot (its own
/// start phase), with two trailing rings (`rain`: a - b/2 + c/4). Twin: `sf_surface::layer::drops`.
fn rain_drops(q: vec2<f32>, seed: u32, time: f32, period: i32) -> vec2<f32> {
    let cell = grid_wrap_cell(vec2<i32>(floor(q)), period);
    let f = q - floor(q);
    let start = grid_hash(cell, seed ^ 0x51ed27u).x;
    var slope = vec2(0.0);
    var weight = 1.0;
    for (var k = 0; k < 3; k++) {
        let t = time * RIPPLE_RATE + start - f32(k) * RIPPLE_RING;
        let cycle = floor(t);
        let spot = grid_hash(cell, seed ^ (u32(i32(cycle)) * 0x632be5abu));
        let drop = vec2(0.5) + (0.5 - RIPPLE_RADIUS) * (2.0 * spot - 1.0);
        slope += weight * rain_ring(f - drop, t - cycle);
        weight *= -0.5;
    }
    return slope;
}

/// Rain rings at render XZ `xz` (m): up to four drop grids, grid k on over rain k/4..(k+1)/4
/// (Lagarde's ripple layers: drizzle rings sparsely, a storm densely). `time`: seconds on a clock
/// wrapped on whole ring cycles. Returns the slope the rings tilt the normal by along X and Z (the
/// normal is `up + (slope.x, 0, slope.y)`, normalised). Twin: `sf_surface::layer::ripples`.
fn rain_ripples(xz: vec2<f32>, time: f32, rain: f32) -> vec2<f32> {
    var scales = array<f32, 4>(RIPPLE_SCALE_0, RIPPLE_SCALE_1, RIPPLE_SCALE_2, RIPPLE_SCALE_3);
    var slope = vec2(0.0);
    for (var k = 0u; k < 4u; k++) {
        let w = saturate((rain - f32(k) * 0.25) * 4.0);
        if w <= 0.0 {
            break;
        }
        let cell_m = RIPPLE_CELL_M * scales[k];
        let q = xz / cell_m + vec2(0.37, 0.71) * f32(k);
        slope += w * rain_drops(q, k + 1u, time, grid_cells_per_origin(cell_m));
    }
    return slope;
}
