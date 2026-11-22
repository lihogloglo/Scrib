// The terrain's surface shading (albedo, roughness, AO, shading normal) at a
// point, for the terrain's fragment (terrain.wesl). Spec:
// superfargoth/docs/specs/terrain.md.
//
// Only textureLoad, textureSampleLevel and textureSampleGrad (explicit
// derivatives): safe in non-uniform control flow.

import super::common::{
    P_DETAIL, P_VALID, P_MACRO, P_HEX, P_CLIFF, P_HEIGHT, P_SHADE, HEIGHT_LEVELS, DEBUG_RAW_LAYERS, debug_flags,
    ShadeLevels, normal_at_level,
    pf, pi, params, level_coord, in_window, window_edge_dist, layer_edge_fade, height_level, height_bilinear,
    splat_at, layer_tiling, layer_triplanar, layer_mean,
    macro_clipmap, macro_sampler, albedo_height_array, normal_array, ao_roughness_array, layer_sampler,
};

const MACRO_LEVELS: u32 = 10u;

fn macro_sample_level(xz: vec2<f32>, l: u32) -> vec3<f32> {
    let c = level_coord(xz, l + 1u);
    let level = pi(P_MACRO + l);
    let t = vec2<f32>(level.xy) + c + 0.5;
    return textureSampleLevel(macro_clipmap, macro_sampler, t / 512.0, i32(l), 0.0).rgb;
}

/// Linear macro albedo (bilinear, from the finest resident macro level, blended smoothly into the
/// next coarser level across the outer 16 texels of its window so clipmap edges never seam).
/// Alpha = 0 where none is resident.
fn macro_albedo(xz: vec2<f32>) -> vec4<f32> {
    let valid = params[P_VALID].z;
    for (var l = 0u; l < MACRO_LEVELS; l += 1u) {
        if ((valid >> l) & 1u) == 0u {
            continue;
        }
        let c = level_coord(xz, l + 1u);
        let level = pi(P_MACRO + l);
        if !in_window(level, vec2<i32>(floor(c)), 0, 1) {
            continue;
        }
        var rgb = macro_sample_level(xz, l);
        let edge = window_edge_dist(level, c);
        if edge < 16.0 {
            for (var l2 = l + 1u; l2 < MACRO_LEVELS; l2 += 1u) {
                if ((valid >> l2) & 1u) == 0u {
                    continue;
                }
                let c2 = level_coord(xz, l2 + 1u);
                if in_window(pi(P_MACRO + l2), vec2<i32>(floor(c2)), 0, 1) {
                    let w = smoothstep(1.0, 16.0, edge);
                    rgb = mix(macro_sample_level(xz, l2), rgb, w);
                    break;
                }
            }
        }
        return vec4<f32>(rgb, 1.0);
    }
    return vec4<f32>(0.0);
}

fn hash(p: vec2<i32>) -> f32 {
    var h = (u32(p.x) * 0x8da6b343u) ^ (u32(p.y) * 0xd8163841u);
    h = (h ^ (h >> 15u)) * 0x2c1b3c6du;
    h = (h ^ (h >> 12u)) * 0x297a2d39u;
    h = h ^ (h >> 15u);
    return f32(h & 0xffffu) / 65535.0;
}

/// Value noise with `period` cells, so the pattern repeats every 1024 m and
/// stays continuous across floating-origin moves (world-coordinates.md).
fn value_noise(p: vec2<f32>, period: i32) -> f32 {
    let i = vec2<i32>(floor(p));
    let f = p - floor(p);
    let u = f * f * (3.0 - 2.0 * f);
    let m = vec2<i32>(period - 1);
    let a = hash(i & m);
    let b = hash((i + vec2<i32>(1, 0)) & m);
    let c = hash((i + vec2<i32>(0, 1)) & m);
    let d = hash((i + vec2<i32>(1, 1)) & m);
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

/// Low-frequency brightness and hue variation against visible tiling.
fn macro_variation(wxz: vec2<f32>) -> vec3<f32> {
    let n1 = value_noise(wxz / 128.0, 8);
    let n2 = value_noise(wxz / 32.0, 32);
    let n3 = value_noise(wxz / 8.0, 128);
    let v = 0.55 * n1 + 0.3 * n2 + 0.15 * n3;
    return vec3<f32>(0.82 + 0.36 * v, 0.84 + 0.32 * v, 0.86 + 0.28 * v);
}

struct LayerSample {
    albedo_height: vec4<f32>,
    /// Tangent-space normal (x along +U, y along +V, z up).
    normal: vec3<f32>,
    /// x: ambient occlusion, y: perceptual roughness.
    surface: vec2<f32>,
}

/// Integer floor division `floor(n / d)` for `d > 0`.
fn floor_div_i(n: i32, d: i32) -> i32 {
    var q = n / d;
    if (n % d != 0) && (n < 0) {
        q -= 1;
    }
    return q;
}

/// Wrap a hex-lattice vertex `(q, r)` onto the 1024 m floating-origin torus (`pq` along q, even
/// `pr` along r). Because `sf_terrain::bake::quantize_tiling` makes `R = 1024 / tiling_m` an
/// integer, shifting UV by `(±R, 0)` or `(0, ±R)` maps to exact lattice shifts `(±pq, 0)` and
/// `(∓pr/2, ±pr)`, leaving the wrapped vertex id and local UV offset invariant across origin moves.
fn hex_wrap_vertex(v: vec2<i32>, pq: i32, pr: i32) -> vec2<i32> {
    let k = floor_div_i(v.y, pr);
    let vy = v.y - k * pr;
    let vx_s = v.x + k * (pr >> 1u);
    let vx = vx_s - floor_div_i(vx_s, pq) * pq;
    return vec2<i32>(vx, vy);
}

/// Three independent uniform hashes in [0, 1] per (wrapped hex vertex, layer id): x = angle,
/// yz = random UV offset.
fn hex_hash3(v: vec2<i32>, layer_id: u32) -> vec3<f32> {
    var h = (u32(v.x) * 0x8da6b343u) ^ (u32(v.y) * 0xd8163841u) ^ (layer_id * 0x9e3779b9u);
    h = (h ^ (h >> 15u)) * 0x2c1b3c6du;
    let h0 = h ^ (h >> 15u);
    h = (h0 ^ (h0 >> 12u)) * 0x297a2d39u;
    let h1 = h ^ (h >> 15u);
    h = (h1 ^ (h1 >> 13u)) * 0x85ebca6bu;
    let h2 = h ^ (h >> 16u);
    return vec3<f32>(f32(h0 & 0xffffu), f32(h1 & 0xffffu), f32(h2 & 0xffffu)) * (1.0 / 65535.0);
}

struct HexGrid {
    v0: vec2<i32>,
    v1: vec2<i32>,
    v2: vec2<i32>,
    w: vec3<f32>,
    sx: f32,
    sy: f32,
    pq: i32,
    pr: i32,
}

/// Practical Real-Time Hex-Tiling simplex grid (Mikkelsen 2022, JCGT), made exact-periodic over
/// 1024 m. Contrast-preserving power weights prune sub-LSB taps (< 0.002) so most pixels take
/// 1-2 taps instead of 3.
fn hex_grid(uv: vec2<f32>, tiling_m: f32, contrast: f32, scale: f32) -> HexGrid {
    let repeats = max(round(1024.0 / max(tiling_m, 1e-3)), 1.0);
    let pq = max(i32(round(repeats * scale)), 1);
    let pr_half = max(i32(round(0.5 * repeats * scale * 1.15470054)), 1);
    let pr = pr_half * 2;
    let sx = f32(pq) / repeats;
    let sy = f32(pr) / repeats;

    let skewed = vec2<f32>(uv.x * sx - 0.5 * uv.y * sy, uv.y * sy);
    let base = vec2<i32>(floor(skewed));
    let f = skewed - floor(skewed);

    var g: HexGrid;
    g.sx = sx;
    g.sy = sy;
    g.pq = pq;
    g.pr = pr;
    if f.x + f.y <= 1.0 {
        g.v0 = base;
        g.v1 = base + vec2<i32>(1, 0);
        g.v2 = base + vec2<i32>(0, 1);
        g.w = vec3<f32>(1.0 - f.x - f.y, f.x, f.y);
    } else {
        g.v0 = base + vec2<i32>(1, 1);
        g.v1 = base + vec2<i32>(1, 0);
        g.v2 = base + vec2<i32>(0, 1);
        g.w = vec3<f32>(f.x + f.y - 1.0, 1.0 - f.y, 1.0 - f.x);
    }

    var w = pow(max(g.w, vec3<f32>(1e-5)), vec3<f32>(contrast));
    w /= max(w.x + w.y + w.z, 1e-6);
    w = select(vec3<f32>(0.0), w, w > vec3<f32>(0.002));
    w /= max(w.x + w.y + w.z, 1e-6);
    g.w = w;
    return g;
}

struct HexTap {
    uv: vec2<f32>,
    ddx: vec2<f32>,
    ddy: vec2<f32>,
    /// `(cos(theta), sin(theta))` of the UV rotation, used to inverse-rotate tangent normals.
    cs: vec2<f32>,
}

/// Rotated and offset UV + screen derivatives for one hex vertex around its cell center.
fn hex_tap(
    uv: vec2<f32>,
    ddx: vec2<f32>,
    ddy: vec2<f32>,
    v: vec2<i32>,
    g: HexGrid,
    layer_id: u32,
    rot_strength: f32,
) -> HexTap {
    let center = vec2<f32>((f32(v.x) + 0.5 * f32(v.y)) / g.sx, f32(v.y) / g.sy);
    let duv = uv - center;
    let rnd = hex_hash3(hex_wrap_vertex(v, g.pq, g.pr), layer_id);
    let theta = (rnd.x * 2.0 - 1.0) * 3.14159265 * rot_strength;
    let c = cos(theta);
    let s = sin(theta);
    var t: HexTap;
    t.uv = vec2<f32>(c * duv.x - s * duv.y, s * duv.x + c * duv.y) + rnd.yz;
    t.ddx = vec2<f32>(c * ddx.x - s * ddx.y, s * ddx.x + c * ddx.y);
    t.ddy = vec2<f32>(c * ddy.x - s * ddy.y, s * ddy.x + c * ddy.y);
    t.cs = vec2<f32>(c, s);
    return t;
}

/// Single un-tiled projection fetch: three fetches, one per array (T6: albedo + height BC7, normal
/// X/Y BC5, AO + roughness BC5).
fn sample_projection_raw(id: u32, uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>) -> LayerSample {
    var s: LayerSample;
    s.albedo_height = textureSampleGrad(albedo_height_array, layer_sampler, uv, i32(id), ddx, ddy);
    let nxy = textureSampleGrad(normal_array, layer_sampler, uv, i32(id), ddx, ddy).xy * 2.0 - 1.0;
    s.normal = vec3<f32>(nxy, sqrt(max(1.0 - dot(nxy, nxy), 0.0)));
    s.surface = textureSampleGrad(ao_roughness_array, layer_sampler, uv, i32(id), ddx, ddy).xy;
    return s;
}

/// One projection of a layer, using Mikkelsen (2022) hex-tiling when `P_HEX.x > 0` (live A/B
/// tunable `terrain.hex_tiling`). Tangent-space normal XY is rotated back by `R(-theta)` so bump
/// lighting stays aligned with the rotated texture features.
fn sample_projection(id: u32, uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>) -> LayerSample {
    let hex = pf(P_HEX);
    if hex.x <= 0.0 {
        return sample_projection_raw(id, uv, ddx, ddy);
    }
    let g = hex_grid(uv, layer_tiling(id), hex.z, hex.w);
    var verts = array<vec2<i32>, 3>(g.v0, g.v1, g.v2);
    var ah = vec4<f32>(0.0);
    var n_acc = vec3<f32>(0.0);
    var surf = vec2<f32>(0.0);
    for (var k = 0u; k < 3u; k += 1u) {
        let wk = g.w[k];
        if wk > 0.0 {
            let t = hex_tap(uv, ddx, ddy, verts[k], g, id, hex.y);
            let raw = sample_projection_raw(id, t.uv, t.ddx, t.ddy);
            let rn = vec3<f32>(
                t.cs.x * raw.normal.x + t.cs.y * raw.normal.y,
                -t.cs.y * raw.normal.x + t.cs.x * raw.normal.y,
                raw.normal.z,
            );
            ah += wk * raw.albedo_height;
            n_acc += wk * rn;
            surf += wk * raw.surface;
        }
    }
    var out: LayerSample;
    out.albedo_height = ah;
    out.normal = normalize(n_acc);
    out.surface = surf;
    return out;
}

/// The top-down height of a layer (the height blend's first pass), using the same hex-tiling grid
/// as [`sample_projection`] so height-blend transitions follow the rotated/offset texture features.
fn layer_height(id: u32, wpos: vec3<f32>, dpx: vec3<f32>, dpy: vec3<f32>) -> f32 {
    let tiling = layer_tiling(id);
    let inv = 1.0 / tiling;
    let uv = wpos.xz * inv;
    let ddx = dpx.xz * inv;
    let ddy = dpy.xz * inv;
    let hex = pf(P_HEX);
    if hex.x <= 0.0 {
        return textureSampleGrad(albedo_height_array, layer_sampler, uv, i32(id), ddx, ddy).a;
    }
    let g = hex_grid(uv, tiling, hex.z, hex.w);
    var verts = array<vec2<i32>, 3>(g.v0, g.v1, g.v2);
    var h = 0.0;
    for (var k = 0u; k < 3u; k += 1u) {
        let wk = g.w[k];
        if wk > 0.0 {
            let t = hex_tap(uv, ddx, ddy, verts[k], g, id, hex.y);
            h += wk * textureSampleGrad(albedo_height_array, layer_sampler, t.uv, i32(id), t.ddx, t.ddy).a;
        }
    }
    return h;
}

/// A layer at a point, with a world-space normal: top-down projection, plus
/// triplanar side projections where the slope needs them (whiteout normal
/// blending, Golus 2017).
fn sample_layer(
    id: u32,
    wpos: vec3<f32>,
    dpx: vec3<f32>,
    dpy: vec3<f32>,
    n: vec3<f32>,
    steep: f32,
) -> LayerSample {
    let inv = 1.0 / layer_tiling(id);
    var top = sample_projection(id, wpos.xz * inv, dpx.xz * inv, dpy.xz * inv);
    let ty = vec3<f32>(top.normal.xy + n.xz, abs(top.normal.z) * n.y);
    top.normal = ty.xzy;
    if steep <= 0.0 || !layer_triplanar(id) {
        return top;
    }
    var blend = pow(abs(n), vec3<f32>(4.0));
    blend = blend / (blend.x + blend.y + blend.z);
    let sx = sample_projection(id, wpos.zy * inv, dpx.zy * inv, dpy.zy * inv);
    let sz = sample_projection(id, wpos.xy * inv, dpx.xy * inv, dpy.xy * inv);
    let nx = vec3<f32>(sx.normal.xy + n.zy, abs(sx.normal.z) * n.x);
    let nz = vec3<f32>(sz.normal.xy + n.xy, abs(sz.normal.z) * n.z);
    var out: LayerSample;
    out.albedo_height = mix(
        top.albedo_height,
        sx.albedo_height * blend.x + top.albedo_height * blend.y + sz.albedo_height * blend.z,
        steep,
    );
    out.surface = mix(top.surface, sx.surface * blend.x + top.surface * blend.y + sz.surface * blend.z, steep);
    out.normal = mix(top.normal, nx.zyx * blend.x + top.normal * blend.y + nz.xyz * blend.z, steep);
    return out;
}

/// Full X/Y/Z triplanar projection of a rock layer (with hex-tiling and whiteout normal blending),
/// pruning axes with negligible weight (< 0.01) so steep faces facing one axis skip invisible sides.
fn sample_layer_triplanar(
    id: u32,
    wpos: vec3<f32>,
    dpx: vec3<f32>,
    dpy: vec3<f32>,
    n: vec3<f32>,
) -> LayerSample {
    let inv = 1.0 / layer_tiling(id);
    var blend = pow(max(abs(n), vec3<f32>(1e-5)), vec3<f32>(6.0));
    blend /= max(blend.x + blend.y + blend.z, 1e-6);
    blend = select(vec3<f32>(0.0), blend, blend > vec3<f32>(0.01));
    blend /= max(blend.x + blend.y + blend.z, 1e-6);

    var ah = vec4<f32>(0.0);
    var surf = vec2<f32>(0.0);
    var nw = vec3<f32>(0.0);
    if blend.x > 0.0 {
        let sx = sample_projection(id, wpos.zy * inv, dpx.zy * inv, dpy.zy * inv);
        let nx = vec3<f32>(sx.normal.xy + n.zy, abs(sx.normal.z) * n.x);
        ah += sx.albedo_height * blend.x;
        surf += sx.surface * blend.x;
        nw += nx.zyx * blend.x;
    }
    if blend.y > 0.0 {
        let sy = sample_projection(id, wpos.xz * inv, dpx.xz * inv, dpy.xz * inv);
        let ny = vec3<f32>(sy.normal.xy + n.xz, abs(sy.normal.z) * n.y);
        ah += sy.albedo_height * blend.y;
        surf += sy.surface * blend.y;
        nw += ny.xzy * blend.y;
    }
    if blend.z > 0.0 {
        let sz = sample_projection(id, wpos.xy * inv, dpx.xy * inv, dpy.xy * inv);
        let nz = vec3<f32>(sz.normal.xy + n.xy, abs(sz.normal.z) * n.z);
        ah += sz.albedo_height * blend.z;
        surf += sz.surface * blend.z;
        nw += nz.xyz * blend.z;
    }
    var out: LayerSample;
    out.albedo_height = ah;
    out.surface = surf;
    out.normal = normalize(nw);
    return out;
}

/// Subtle geological horizontal rock-strata banding along warped world elevation (floating-origin
/// safe: XZ warp uses 1024 m-periodic `value_noise`).
fn cliff_strata_mod(wpos: vec3<f32>) -> f32 {
    let warp = (value_noise(wpos.xz / 64.0, 16) - 0.5) * 14.0
        + (value_noise(wpos.xz / 16.0, 64) - 0.5) * 3.5;
    let y = wpos.y + warp;
    let coarse = sin(y * (6.2831855 / 18.0));
    let fine = sin(y * (6.2831855 / 4.2) + coarse * 1.3);
    return 0.88 + 0.10 * coarse + 0.07 * fine;
}

/// Dimensionless curvature at one height clipmap level `l`.
fn curvature_at_level(xz: vec2<f32>, l: u32) -> f32 {
    let s = 0.5 * exp2(f32(l));
    let hc = height_bilinear(xz, l);
    let avg = 0.25 * (
        height_bilinear(xz + vec2<f32>(s, 0.0), l)
        + height_bilinear(xz - vec2<f32>(s, 0.0), l)
        + height_bilinear(xz + vec2<f32>(0.0, s), l)
        + height_bilinear(xz - vec2<f32>(0.0, s), l)
    );
    return clamp((avg - hc) / s, -1.0, 1.0);
}

/// The old coupled curvature (`terrain.shade.decoupled` off): at the morphed CDLOD `lod`'s level and that
/// level's own spacing (blending into `l + 1` across the morph zone and near the window edge).
fn terrain_curvature_lod(xz: vec2<f32>, lod: f32) -> f32 {
    let lf = max(lod, 0.0);
    let l0 = u32(floor(lf));
    let l = height_level(xz, l0, 2, 3);
    if l >= HEIGHT_LEVELS {
        return 0.0;
    }
    var curv = curvature_at_level(xz, l);
    let morph_t = select(0.0, lf - floor(lf), l == l0);
    let edge = window_edge_dist(pi(P_HEIGHT + l), level_coord(xz, l));
    let edge_t = 1.0 - smoothstep(3.0, 20.0, edge);
    let t = max(morph_t, edge_t);
    if t > 0.01 {
        let l1 = height_level(xz, l + 1u, 2, 3);
        if l1 < HEIGHT_LEVELS {
            curv = mix(curv, curvature_at_level(xz, l1), t);
        }
    }
    return curv;
}

/// The height gradient (dh/dx, dh/dz) at xz from level `l`'s filtered normals.
fn gradient_at_level(xz: vec2<f32>, l: u32) -> vec2<f32> {
    let n = normal_at_level(xz, l);
    return -n.xz / max(n.y, 0.05);
}

/// Dimensionless curvature at a fixed world scale `s_m` from level `l` (WG-LOD): `s_m / 4` times the
/// Laplacian, the divergence of the gradient across +-`s_m` (four filtered normal taps), so the same
/// world feature reads alike on every level (the old `curvature_at_level` scaled with the level's
/// spacing); a level coarser than `s_m` gives its own smoothed curvature, never a bilinear grid.
fn curvature_fixed(xz: vec2<f32>, l: u32, s_m: f32) -> f32 {
    let ex = vec2<f32>(s_m, 0.0);
    let ez = vec2<f32>(0.0, s_m);
    let dgx = gradient_at_level(xz + ex, l).x - gradient_at_level(xz - ex, l).x;
    let dgz = gradient_at_level(xz + ez, l).y - gradient_at_level(xz - ez, l).y;
    return clamp((dgx + dgz) * 0.125, -1.0, 1.0);
}

/// Dimensionless local terrain curvature at xz for a surface at LOD `lod`, from the shading levels
/// `s` (`shade_levels`, decoupled from the mesh LOD by default) at the fixed scale `P_SHADE.w`.
/// Positive in concave crevices/valleys, negative on convex ridges and cliff lips.
fn terrain_curvature(xz: vec2<f32>, lod: f32, s: ShadeLevels) -> f32 {
    let shade = pf(P_SHADE);
    if shade.x <= 0.5 {
        return terrain_curvature_lod(xz, lod);
    }
    if s.l0 >= HEIGHT_LEVELS {
        return 0.0;
    }
    var curv = curvature_fixed(xz, s.l0, shade.w);
    if s.t > 0.01 && s.l1 < HEIGHT_LEVELS {
        curv = mix(curv, curvature_fixed(xz, s.l1, shade.w), s.t);
    }
    return curv;
}

/// What [`terrain_surface`] returns: the inputs of the terrain's `PbrInput`.
struct TerrainSurface {
    /// Linear albedo, macro variation applied.
    albedo: vec3<f32>,
    roughness: f32,
    ao: f32,
    /// World-space shading normal.
    n: vec3<f32>,
    /// The blended layer height (0..1; 0.5 past the splat range): surface weather fills puddles
    /// from its lows (surface-weather.md W4).
    height: f32,
}

/// The terrain's surface at render XZ `xz` (world position `wpos`, its
/// screen derivatives `dpx`/`dpy`), geometric normal `n_geo`, `dist` meters
/// from the camera, its shading levels `shade` (`shade_levels`: `n_geo` and the curvature read them).
/// Height-based splat blending near the camera, the macro albedo past the splat range.
fn terrain_surface(
    xz: vec2<f32>,
    wpos: vec3<f32>,
    dpx: vec3<f32>,
    dpy: vec3<f32>,
    n_geo: vec3<f32>,
    dist: f32,
    lod: f32,
    shade: ShadeLevels,
) -> TerrainSurface {
    let wxz = wpos.xz;
    let macro_c = macro_albedo(xz);
    let variation = macro_variation(wxz);
    let raw = (debug_flags() & DEBUG_RAW_LAYERS) != 0u;

    var albedo = select(vec3<f32>(0.3), macro_c.rgb, macro_c.a > 0.0);
    var roughness = 0.85;
    var ao = 1.0;
    var n = n_geo;
    var height = 0.5;

    let detail = pf(P_DETAIL);
    let cliff_f = pf(P_CLIFF);
    let cliff_layer = params[P_CLIFF].w;
    let cliff_on = cliff_layer < params[P_VALID].w;
    let cliff_w = select(0.0, 1.0 - smoothstep(cliff_f.y, cliff_f.x, n_geo.y), cliff_on);
    let strata = select(1.0, cliff_strata_mod(wpos), cliff_w > 0.0);

    // Distant cliff rock (past splat_range): blend macro albedo toward regionally-tinted rock mean
    // with elevation strata so cliffs kilometers away read as exposed rock rather than grass/ash.
    if cliff_w > 0.0 && !raw {
        let rock_mean = layer_mean(cliff_layer).rgb;
        let region_tint = clamp(albedo / max(rock_mean, vec3<f32>(0.05)), vec3<f32>(0.55), vec3<f32>(1.35));
        let far_rock = rock_mean * mix(vec3<f32>(1.0), region_tint, 0.45) * strata;
        albedo = mix(albedo, far_rock, cliff_w);
    }

    let near = (1.0 - smoothstep(0.7 * detail.x, detail.x, dist)) * layer_edge_fade(xz);
    if near > 0.0 {
        let s = splat_at(xz);
        if s.valid {
            let steep = smoothstep(0.8, 0.6, n_geo.y);
            // Height-based blending (Mishkinis 2013), in two passes so no
            // per-layer samples are kept: heights first, then only the
            // layers that survive the blend are shaded.
            var hw = vec4<f32>(-1e9);
            var mean = vec3<f32>(0.0);
            for (var k = 0; k < 4; k += 1) {
                let w = s.weights[k];
                if w > 0.0 {
                    hw[k] = layer_height(s.ids[k], wpos, dpx, dpy) + w;
                    mean += w * layer_mean(s.ids[k]).rgb;
                }
            }
            let top = max(max(hw.x, hw.y), max(hw.z, hw.w)) - detail.y;
            let b = max(hw - vec4<f32>(top), vec4<f32>(0.0));
            let bw = b / max(b.x + b.y + b.z + b.w, 1e-5);
            var a = vec3<f32>(0.0);
            var surface = vec2<f32>(0.0);
            var nb = vec3<f32>(0.0);
            var h = 0.0;
            for (var k = 0; k < 4; k += 1) {
                if bw[k] > 0.0 {
                    let l = sample_layer(s.ids[k], wpos, dpx, dpy, n_geo, steep);
                    a += bw[k] * l.albedo_height.rgb;
                    surface += bw[k] * l.surface;
                    nb += bw[k] * l.normal;
                    h += bw[k] * l.albedo_height.a;
                }
            }
            // The macro map carries the tint (bake: mean albedo x tint), so
            // tint = macro / weighted mean; near and far then agree.
            var tint = vec3<f32>(1.0);
            // RO-16 scaffold (remove with the veil fix): `terrain_tint off` keeps the raw layers.
            if macro_c.a > 0.0 && !raw {
                tint = clamp(macro_c.rgb / max(mean, vec3<f32>(0.01)), vec3<f32>(0.0), vec3<f32>(4.0));
            }
            var near_albedo = a * tint;
            var near_surface = surface;
            var near_n = normalize(nb);
            var near_h = h;

            // WG-X7b: height-blended triplanar cliff rock on steep slopes.
            if cliff_w > 0.0 {
                let rock = sample_layer_triplanar(cliff_layer, wpos, dpx, dpy, n_geo);
                var rock_albedo = rock.albedo_height.rgb * strata;
                if macro_c.a > 0.0 && !raw {
                    let rock_mean = layer_mean(cliff_layer).rgb;
                    let region_tint = clamp(macro_c.rgb / max(rock_mean, vec3<f32>(0.05)), vec3<f32>(0.55), vec3<f32>(1.35));
                    rock_albedo *= mix(vec3<f32>(1.0), region_tint, 0.45);
                }
                // Height-blend rock ribs through soil/grass along the slope transition.
                let hg = near_h + (1.0 - cliff_w);
                let hr = rock.albedo_height.a + cliff_w;
                let htop = max(hg, hr) - detail.y;
                let bg = max(hg - htop, 0.0);
                let br = max(hr - htop, 0.0);
                let rw = br / max(bg + br, 1e-5);
                near_albedo = mix(near_albedo, rock_albedo, rw);
                near_surface = mix(near_surface, rock.surface, rw);
                near_n = normalize(mix(near_n, rock.normal, rw));
                near_h = mix(near_h, rock.albedo_height.a, rw);
            }

            albedo = mix(albedo, near_albedo, near);
            roughness = mix(roughness, near_surface.y, near);
            ao = mix(1.0, near_surface.x, near);
            n = normalize(mix(n_geo, near_n, near));
            height = mix(height, near_h, near);
        }
    }
    if !raw {
        albedo *= variation;
        // WG-X7b: curvature crevice darkening and subtle convex cliff-lip highlight.
        let crev_k = cliff_f.z;
        if crev_k > 0.0 {
            let curv = terrain_curvature(xz, lod, shade);
            let dark = saturate(curv * 2.5);
            let lip = saturate(-curv * 2.5);
            albedo *= (1.0 - crev_k * dark) * (1.0 + 0.12 * crev_k * lip);
            ao *= 1.0 - 0.5 * crev_k * dark;
        }
    }

    var out: TerrainSurface;
    out.albedo = albedo;
    out.roughness = roughness;
    out.ao = ao;
    out.n = n;
    out.height = height;
    return out;
}
