// The exposure pack pass (SW-5/SW-6; `exposure.rs`, CPU twin `sf_surface::view` "Exposure pack":
// `exposure_texel`, `exposure_source`, `ExposureClock`): one dispatch over the 1024² pack, reading
// last pack and writing this one (ping-pong). Per texel: R, G the solid and leaf casters' heights
// over the map centre (m, from the depth maps; empty = the bottom plane), B the lag (eased minus
// fresh exposure), A the fresh ground exposure (`ground`). The layer (`layer.wesl`
// `surface_exposure`) runs its 3D test on R and G and adds the lag.

struct ExposureParams {
    /// xy: the shift (a new texel's source in last pack is texel + shift); z: mode (MODE_*);
    /// w: pack edge (texels).
    shift_mode: vec4<i32>,
    /// x: the lag's factor this pass; y: depth range (m); z: depth below the centre (m);
    /// w: share of the rain a full leaf canopy lets through.
    ease: vec4<f32>,
}

@group(0) @binding(0) var<uniform> p: ExposureParams;
@group(0) @binding(1) var shelter: texture_2d<f32>;
@group(0) @binding(2) var canopy: texture_2d<f32>;
@group(0) @binding(3) var prev: texture_2d<f32>;
@group(0) @binding(4) var samp: sampler;
@group(0) @binding(5) var next: texture_storage_2d<rgba16float, write>;

/// `sf_surface::view::ExposureMode::code`.
const MODE_FRESH: i32 = 0;
const MODE_CARRY: i32 = 1;
const MODE_EASE: i32 = 2;

/// The layer's (`layer.wesl`, a test checks they match): the cover ramp and the edge fade.
const SHELTER_NEAR_M: f32 = 0.12;
const SHELTER_FULL_M: f32 = 0.35;
const SHELTER_FADE_START: f32 = 0.875;

/// Ground exposure at texel `a`: `layer.wesl` `surface_exposure` at the texel's centre (tent
/// blend 0) for a receiver on the bottom plane (below every caster: any caster shelters it), no
/// normal offset, with the edge fade. What the ground shows once the lag is gone.
fn ground(a: vec2<i32>, n: i32) -> f32 {
    let size = vec2<f32>(f32(n));
    let uv = (vec2<f32>(a) + 0.5) / size;
    let near = vec4(SHELTER_NEAR_M);
    let full = vec4(SHELTER_FULL_M);
    var open = 0.0;
    var leaves = 0.0;
    var weight = 0.0;
    for (var row = 0; row < 3; row++) {
        for (var column = 0; column < 3; column++) {
            let shift = vec2(f32(2 * column - 2), f32(2 * row - 2)) / size;
            // Reverse Z: depth over the bottom plane, in meters.
            let over = textureGather(0, shelter, samp, uv + shift) * p.ease.y;
            let gap = vec4(1.0) - smoothstep(near, full, over);
            // Tent weights of the four texels (gather order: (0,1), (1,1), (1,0), (0,0)).
            let low = vec2(f32(2 * column), f32(2 * row));
            let high = low + 1.0;
            let lw = select(low + 1.0, 5.0 - low, low >= vec2(3.0));
            let hw = select(high + 1.0, 5.0 - high, high >= vec2(3.0));
            let w = vec4(lw.x * hw.y, hw.x * hw.y, hw.x * lw.y, lw.x * lw.y);
            open += dot(gap, w);
            weight += dot(w, vec4(1.0));
            let leaf_over = textureGather(0, canopy, samp, uv + shift) * p.ease.y;
            leaves += dot(smoothstep(near, full, leaf_over), w);
        }
    }
    let leaf_hold = (1.0 - p.ease.w) * (leaves / weight);
    let ndc = abs(uv * 2.0 - 1.0);
    let edge = max(ndc.x, ndc.y);
    return mix(open / weight * (1.0 - leaf_hold), 1.0, smoothstep(SHELTER_FADE_START, 1.0, edge));
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let n = p.shift_mode.w;
    let a = vec2<i32>(id.xy);
    if a.x >= n || a.y >= n {
        return;
    }
    let mode = p.shift_mode.z;
    if mode == MODE_EASE {
        // The maps did not change: keep the heights and the target, decay the lag.
        let v = textureLoad(prev, a, 0);
        textureStore(next, a, vec4(v.r, v.g, v.b * p.ease.x, v.a));
        return;
    }
    let range = p.ease.y;
    let down = p.ease.z;
    let heights = vec2(textureLoad(shelter, a, 0).r, textureLoad(canopy, a, 0).r) * range - down;
    let fresh = ground(a, n);
    var lag = 0.0;
    if mode == MODE_CARRY {
        // What this world texel showed: last pack's eased value at its texel, open sky where the
        // old map did not reach.
        let src = a + p.shift_mode.xy;
        var shown = 1.0;
        if all(src >= vec2(0)) && all(src < vec2(n)) {
            let v = textureLoad(prev, src, 0);
            shown = v.b + v.a;
        }
        lag = (shown - fresh) * p.ease.x;
    }
    textureStore(next, a, vec4(heights, lag, fresh));
}
