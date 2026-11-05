// The trail ring pass (surface-weather.md "Trails (W6)", slice W6a; `trail.rs`): one dispatch over
// the whole 1024² ring, reading last frame's ring and writing this frame's (ping-pong; W6b's motion
// vectors read both). Per texel: keep last frame's value if it held the same world texel (else it
// entered the ring: 0), refill it, stamp it (Min on R, Max on G), cap the lip. The CPU twin is
// `sf_surface::trail` (`TrailPass::run`, `texel_update`); constants come in the uniform.

struct Stamp {
    /// xy: render X, Z of the centre; zw: unit facing (the long axis).
    center_axis: vec4<f32>,
    /// x: half length (m), y: half width (m), z: depth share.
    half_depth: vec4<f32>,
}

struct TrailParams {
    /// xy: render X, Z of the new window's first texel's min corner; z: texel (m); w: depth (m).
    window: vec4<f32>,
    /// xy: the new window's first world texel; zw: the old one.
    mins: vec4<i32>,
    /// x: stamps; y: 1 when the old window is valid; z: ring edge (texels).
    counts: vec4<u32>,
    /// x: refill this pass (m); y: lip height (share of depth); z: lip width; w: wall.
    shape: vec4<f32>,
    stamps: array<Stamp, 64>,
}

@group(0) @binding(0) var<uniform> p: TrailParams;
@group(0) @binding(1) var prev: texture_2d<f32>;
@group(0) @binding(2) var next: texture_storage_2d<rgba16float, write>;

fn wrap(a: vec2<i32>, n: i32) -> vec2<i32> {
    return ((a % n) + n) % n;
}

/// `sf_surface::trail::profile`: (depression share, lip share) at normalised radius `r`.
fn profile(r: f32) -> vec2<f32> {
    let dep = 1.0 - smoothstep(1.0 - p.shape.w, 1.0, r);
    let x = (r - 1.0) / p.shape.z;
    let lip = select(0.0, 4.0 * x * (1.0 - x), x > 0.0 && x < 1.0);
    return vec2(dep, lip);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let n = i32(p.counts.z);
    let a = vec2<i32>(id.xy);
    if a.x >= n || a.y >= n {
        return;
    }
    let rel = wrap(a - p.mins.xy, n);
    var v = vec2(0.0);
    if p.counts.y != 0u && all(p.mins.zw + wrap(a - p.mins.zw, n) == p.mins.xy + rel) {
        v = textureLoad(prev, a, 0).xy;
    }
    let depth = p.window.w;
    let lip_h = p.shape.y;
    // Refill (`decay`).
    v = vec2(min(v.x + p.shape.x, 0.0), max(v.y - p.shape.x * lip_h, 0.0));
    // Stamps (`Stamp::at`, `combine`).
    let q = p.window.xy + (vec2<f32>(rel) + 0.5) * p.window.z;
    for (var i = 0u; i < p.counts.x; i++) {
        let s = p.stamps[i];
        let d = q - s.center_axis.xy;
        let reach = max(s.half_depth.x, s.half_depth.y) * (1.0 + p.shape.z);
        if dot(d, d) > reach * reach {
            continue;
        }
        let axis = s.center_axis.zw;
        let along = dot(d, axis) / s.half_depth.x;
        let across = dot(d, vec2(-axis.y, axis.x)) / s.half_depth.y;
        let pr = profile(length(vec2(along, across)));
        let h = depth * s.half_depth.z;
        v = vec2(min(v.x, -h * pr.x), max(v.y, lip_h * h * pr.y));
    }
    // `lip_cap`.
    let left = select(0.0, clamp(1.0 + v.x / depth, 0.0, 1.0), depth > 0.0);
    v.y = min(v.y, lip_h * depth * left);
    textureStore(next, a, vec4(v, 0.0, 0.0));
}
