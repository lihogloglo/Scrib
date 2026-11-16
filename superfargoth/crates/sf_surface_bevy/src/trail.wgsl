// The trail ring pass (surface-weather.md "Trails (W6)", slice W6a; `trail.rs`): one dispatch over
// the whole 1024² ring, reading last frame's ring and writing this frame's (ping-pong; W6b's motion
// vectors read both). Per texel: keep last frame's value if it held the same world texel (else it
// entered the ring: 0), refill it, stamp it (Min on R, Max on G), cap the lip, and merge the
// deformer capture into B (S1a, Min: the ceiling the raised cover stands under). The CPU twins are
// `sf_surface::trail` (`TrailPass::run`, `texel_update`; `TrailPass::run_ceiling`) and
// `sf_surface::deform` (`convert`, `GroundGrid::at`, `Capture::texel_of`); constants come in the
// uniform.

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
    /// x: stamps; y: 1 when the old window is valid; z: ring edge (texels); w: capture edge
    /// (texels; 0 = no capture this pass).
    counts: vec4<u32>,
    /// x: refill this pass (m); y: lip height (share of depth); z: lip width; w: wall.
    shape: vec4<f32>,
    /// The capture (S1a): xy render X, Z of its first texel's min corner; z its texel (m); w the
    /// render Y of the slab's bottom (depth 1).
    capture: vec4<f32>,
    /// x: the slab's height (m; depth 0 at its top); y: the ceiling reference (ring depth plus
    /// lip, m); zw: render X, Z of ground sample (0, 0).
    slab: vec4<f32>,
    /// x: ground step (m); y: ground edge (samples).
    ground: vec4<f32>,
    stamps: array<Stamp, 64>,
}

@group(0) @binding(0) var<uniform> p: TrailParams;
@group(0) @binding(1) var prev: texture_2d<f32>;
@group(0) @binding(2) var next: texture_storage_2d<rgba16float, write>;
/// The deformer capture drawn last frame (reverse Z: 1 at the slab's bottom, 0 cleared).
@group(0) @binding(3) var capture_depth: texture_depth_2d;
/// The ground under the capture (render Y), row-major by Z then X.
@group(0) @binding(4) var<storage, read> ground_heights: array<f32>;

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

/// `sf_surface::deform::GroundGrid::at`: the ground (render Y) at render XZ `q`, bilinear,
/// clamped at the grid's edge.
fn ground_at(q: vec2<f32>) -> f32 {
    let last = p.ground.y - 1.0;
    let g = clamp((q - p.slab.zw) / p.ground.x, vec2(0.0), vec2(last));
    let i = min(floor(g), vec2(max(last - 1.0, 0.0)));
    let f = g - i;
    let e = u32(p.ground.y);
    let x0 = u32(i.x);
    let z0 = u32(i.y);
    let x1 = min(x0 + 1u, e - 1u);
    let z1 = min(z0 + 1u, e - 1u);
    let a = mix(ground_heights[z0 * e + x0], ground_heights[z0 * e + x1], f.x);
    let b = mix(ground_heights[z1 * e + x0], ground_heights[z1 * e + x1], f.x);
    return mix(a, b, f.y);
}

/// `sf_surface::deform::Capture::value_at`: the capture's B value (≤ 0, 0 = no deformer) at
/// render XZ `q`: the texel holding it (column along +X, row against Z), its lowest point's
/// height over the ground, clamped to the ceiling reference (`deform::convert`).
fn captured(q: vec2<f32>) -> f32 {
    let size = i32(p.counts.w);
    let t = vec2<i32>(floor((q - p.capture.xy) / p.capture.z));
    if any(t < vec2(0)) || any(t >= vec2(size)) {
        return 0.0;
    }
    let d = textureLoad(capture_depth, vec2(t.x, size - 1 - t.y), 0);
    let h = p.slab.y;
    if d <= 0.0 || h <= 0.0 {
        return 0.0;
    }
    let bottom = p.capture.w + (1.0 - min(d, 1.0)) * p.slab.x;
    return clamp(bottom - ground_at(q) - h, -h, 0.0);
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
    var b = 0.0;
    if p.counts.y != 0u && all(p.mins.zw + wrap(a - p.mins.zw, n) == p.mins.xy + rel) {
        let last = textureLoad(prev, a, 0);
        v = last.xy;
        b = last.z;
    }
    let depth = p.window.w;
    let lip_h = p.shape.y;
    // Refill (`decay`; the ceiling like the prints, `deform::ceiling_update`).
    v = vec2(min(v.x + p.shape.x, 0.0), max(v.y - p.shape.x * lip_h, 0.0));
    b = min(b + p.shape.x, 0.0);
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
    // The deformer capture (S1a, `deform::merge`).
    if p.counts.w != 0u {
        b = min(b, captured(q));
    }
    // `lip_cap`.
    let left = select(0.0, clamp(1.0 + v.x / depth, 0.0, 1.0), depth > 0.0);
    v.y = min(v.y, lip_h * depth * left);
    textureStore(next, a, vec4(v, b, 0.0));
}
