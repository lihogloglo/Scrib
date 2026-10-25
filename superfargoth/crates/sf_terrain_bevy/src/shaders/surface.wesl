// The terrain's surface shading (albedo, roughness, AO, shading normal) at a
// point, for the terrain's fragment (terrain.wesl). Spec:
// superfargoth/docs/specs/terrain.md.
//
// Only textureLoad, textureSampleLevel and textureSampleGrad (explicit
// derivatives): safe in non-uniform control flow.

import super::common::{
    P_DETAIL, P_VALID, P_MACRO,
    pf, pi, params, level_coord, in_window,
    splat_at, layer_tiling, layer_triplanar, layer_mean,
    macro_clipmap, macro_sampler, albedo_height_array, nar_array, layer_sampler,
};

const MACRO_LEVELS: u32 = 10u;

/// Linear macro albedo (bilinear, from the finest resident macro level),
/// alpha = 0 where none is resident.
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
        // Samples sit on texel centers; wrap addressing does the toroidal part.
        let t = vec2<f32>(level.xy) + c + 0.5;
        let rgb = textureSampleLevel(macro_clipmap, macro_sampler, t / 512.0, i32(l), 0.0).rgb;
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

/// One projection of a layer: two fetches (albedo + height, normal + AO +
/// roughness).
fn sample_projection(id: u32, uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>) -> LayerSample {
    var s: LayerSample;
    s.albedo_height = textureSampleGrad(albedo_height_array, layer_sampler, uv, i32(id), ddx, ddy);
    let nar = textureSampleGrad(nar_array, layer_sampler, uv, i32(id), ddx, ddy);
    let nxy = nar.xy * 2.0 - 1.0;
    s.normal = vec3<f32>(nxy, sqrt(max(1.0 - dot(nxy, nxy), 0.0)));
    s.surface = nar.zw;
    return s;
}

/// The top-down height of a layer (the height blend's first pass).
fn layer_height(id: u32, wpos: vec3<f32>, dpx: vec3<f32>, dpy: vec3<f32>) -> f32 {
    let inv = 1.0 / layer_tiling(id);
    return textureSampleGrad(albedo_height_array, layer_sampler, wpos.xz * inv, i32(id), dpx.xz * inv, dpy.xz * inv).a;
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

/// What [`terrain_surface`] returns: the inputs of the terrain's `PbrInput`.
struct TerrainSurface {
    /// Linear albedo, macro variation applied.
    albedo: vec3<f32>,
    roughness: f32,
    ao: f32,
    /// World-space shading normal.
    n: vec3<f32>,
}

/// The terrain's surface at render XZ `xz` (world position `wpos`, its
/// screen derivatives `dpx`/`dpy`), geometric normal `n_geo`, `dist` meters
/// from the camera. Height-based splat blending near the camera, the macro
/// albedo past the splat range.
fn terrain_surface(
    xz: vec2<f32>,
    wpos: vec3<f32>,
    dpx: vec3<f32>,
    dpy: vec3<f32>,
    n_geo: vec3<f32>,
    dist: f32,
) -> TerrainSurface {
    let wxz = wpos.xz;
    let macro_c = macro_albedo(xz);
    let variation = macro_variation(wxz);

    var albedo = select(vec3<f32>(0.3), macro_c.rgb, macro_c.a > 0.0);
    var roughness = 0.85;
    var ao = 1.0;
    var n = n_geo;

    let detail = pf(P_DETAIL);
    let near = 1.0 - smoothstep(0.7 * detail.x, detail.x, dist);
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
            for (var k = 0; k < 4; k += 1) {
                if bw[k] > 0.0 {
                    let l = sample_layer(s.ids[k], wpos, dpx, dpy, n_geo, steep);
                    a += bw[k] * l.albedo_height.rgb;
                    surface += bw[k] * l.surface;
                    nb += bw[k] * l.normal;
                }
            }
            // The macro map carries the tint (bake: mean albedo x tint), so
            // tint = macro / weighted mean; near and far then agree.
            var tint = vec3<f32>(1.0);
            if macro_c.a > 0.0 {
                tint = clamp(macro_c.rgb / max(mean, vec3<f32>(0.01)), vec3<f32>(0.0), vec3<f32>(4.0));
            }
            albedo = mix(albedo, a * tint, near);
            roughness = mix(roughness, surface.y, near);
            ao = mix(1.0, surface.x, near);
            n = normalize(mix(n_geo, normalize(nb), near));
        }
    }
    albedo *= variation;

    var out: TerrainSurface;
    out.albedo = albedo;
    out.roughness = roughness;
    out.ao = ao;
    out.n = n;
    return out;
}
