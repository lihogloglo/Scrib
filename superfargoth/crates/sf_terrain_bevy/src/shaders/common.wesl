// Terrain bindings and the CDLOD vertex logic shared by the main and
// prepass shaders. Spec: superfargoth/docs/specs/terrain.md.
//
// Bindings start at 100, clear of Bevy's material bindings 0..30.
//
// Parameters live in one storage buffer of vec4<u32> slots (layout in
// `params.rs`), written once per frame without touching the material.

import bevy_pbr::render::mesh_functions;
// Raised cover (surface weather W6b): the view's surface weather bindings (vendored bevy_pbr patch
// "SW"), in the main pass and, for the same views, the prepass.
@if(SURFACE_WEATHER || SURFACE_WEATHER_PREPASS)
import sf_surface_bevy::view::surface_raise_at;

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<storage, read> params: array<vec4<u32>>;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var height_clipmap: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var layer_id_clipmap: texture_2d_array<u32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var layer_weight_clipmap: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var macro_clipmap: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var macro_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(106) var albedo_height_array: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(107) var layer_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(108) var normal_array: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(109) var normal_clipmap: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var normal_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(111) var far_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(112) var far_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(113) var ao_roughness_array: texture_2d_array<f32>;

// Slot indices (keep in sync with params.rs).
const P_CAMERA: u32 = 0u;       // xyz: main camera (render space), w: origin absolute Y
const P_ORIGIN: u32 = 1u;       // xy: f32 fraction of the origin's level-0 sample coords, zw: i32 integer part mod 1024
const P_MISC: u32 = 2u;         // xy: f32 origin XZ mod 1024 m, z: f32 fill height, w: u32 debug flags
const P_VALID: u32 = 3u;        // x: height levels, y: layer levels, z: macro levels (bit masks), w: layer count
const P_MORPH: u32 = 4u;        // 12 LODs from MIN_LOD: x morph start, y 1/(end-start), z spacing
const P_HEIGHT: u32 = 16u;      // 10 levels: xy texel of the origin's sample, zw window min relative to it
const P_LAYER: u32 = 26u;       // 3 levels
const P_MACRO: u32 = 29u;       // 10 levels
const P_DETAIL: u32 = 39u;      // x: splat range (m), y: height-blend depth, z: ground reflectance (Bevy scale), w: max terrain height (world m)
const P_LAYERS: u32 = 40u;      // 2 slots per layer: (tiling, displacement, triplanar, -), (mean rgb, mean height)
const P_FAR: u32 = 552u;       // far shadow map: (min x, min z, 1 / extent, base height), render space

const MIN_LOD: i32 = -2;
const HEIGHT_LEVELS: u32 = 10u;
const LAYER_LEVELS: u32 = 3u;
const SIZE: i32 = 512;

const DEBUG_LOD: u32 = 1u;
const DEBUG_WIREFRAME: u32 = 2u;
const DEBUG_NO_FAR_SHADOWS: u32 = 4u;
// RO-16 veil diagnosis (scaffold: remove with the veil fix; params.rs documents each).
const DEBUG_NO_SPECULAR: u32 = 8u;
const DEBUG_NO_INDIRECT: u32 = 16u;
const DEBUG_RAW_LAYERS: u32 = 32u;
const DEBUG_NO_ENV_SPECULAR: u32 = 64u;

fn pf(i: u32) -> vec4<f32> {
    return bitcast<vec4<f32>>(params[i]);
}

fn pi(i: u32) -> vec4<i32> {
    return bitcast<vec4<i32>>(params[i]);
}

fn camera_position() -> vec3<f32> {
    return pf(P_CAMERA).xyz;
}

fn origin_y() -> f32 {
    return pf(P_CAMERA).w;
}

fn fill_height() -> f32 {
    return pf(P_MISC).z;
}

fn debug_flags() -> u32 {
    return params[P_MISC].w;
}

/// The ground's dielectric reflectance (`TerrainSettings::reflectance`, Bevy scale).
fn base_reflectance() -> f32 {
    return pf(P_DETAIL).z;
}

/// World XZ (meters, continuous across origin moves for periods dividing 1024 m).
fn world_xz(xz: vec2<f32>) -> vec2<f32> {
    return xz + pf(P_MISC).xy;
}

/// Continuous sample coordinate, relative to the origin's sample, of a
/// render-space XZ on the lattice of level-0 index >> shift.
fn level_coord(xz: vec2<f32>, shift: u32) -> vec2<f32> {
    let o = pi(P_ORIGIN);
    let mask = (1 << shift) - 1;
    let low = vec2<f32>(f32(o.z & mask), f32(o.w & mask));
    return (low + pf(P_ORIGIN).xy + 2.0 * xz) / f32(1u << shift);
}

/// Whether relative samples i and i + extra lie in a level's window.
fn in_window(level: vec4<i32>, rel: vec2<i32>, lo: i32, hi: i32) -> bool {
    let d = rel - level.zw;
    return all(d >= vec2<i32>(lo)) && all(d < vec2<i32>(SIZE - hi));
}

fn texel(level: vec4<i32>, rel: vec2<i32>) -> vec2<i32> {
    return (level.xy + rel) & vec2<i32>(SIZE - 1);
}

fn height_texel(l: u32, level: vec4<i32>, rel: vec2<i32>) -> f32 {
    return textureLoad(height_clipmap, texel(level, rel), i32(l), 0).r;
}

/// The finest resident height level >= `lmin` whose window holds `xz`
/// with `lo`/`hi` samples of margin, or HEIGHT_LEVELS.
fn height_level(xz: vec2<f32>, lmin: u32, lo: i32, hi: i32) -> u32 {
    let valid = params[P_VALID].x;
    for (var l = lmin; l < HEIGHT_LEVELS; l += 1u) {
        if ((valid >> l) & 1u) == 0u {
            continue;
        }
        let i0 = vec2<i32>(floor(level_coord(xz, l)));
        if in_window(pi(P_HEIGHT + l), i0, lo, hi) {
            return l;
        }
    }
    return HEIGHT_LEVELS;
}

/// Bilinear height (absolute meters) from level `l` (must contain xz).
fn height_bilinear(xz: vec2<f32>, l: u32) -> f32 {
    let c = level_coord(xz, l);
    let i0 = vec2<i32>(floor(c));
    let f = c - floor(c);
    let level = pi(P_HEIGHT + l);
    let h00 = height_texel(l, level, i0);
    let h10 = height_texel(l, level, i0 + vec2<i32>(1, 0));
    let h01 = height_texel(l, level, i0 + vec2<i32>(0, 1));
    let h11 = height_texel(l, level, i0 + vec2<i32>(1, 1));
    return mix(mix(h00, h10, f.x), mix(h01, h11, f.x), f.y);
}

/// Height at xz from the finest resident level >= lmin.
fn height_at(xz: vec2<f32>, lmin: u32) -> f32 {
    let l = height_level(xz, lmin, 0, 1);
    if l >= HEIGHT_LEVELS {
        return fill_height();
    }
    return height_bilinear(xz, l);
}

fn catmull_rom_weights(t: f32) -> vec4<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    return vec4<f32>(
        -0.5 * t3 + t2 - 0.5 * t,
        1.5 * t3 - 2.5 * t2 + 1.0,
        -1.5 * t3 + 2.0 * t2 + 0.5 * t,
        0.5 * t3 - 0.5 * t2,
    );
}

/// Catmull-Rom (interpolating bicubic) height from level 0, for the
/// detail-displacement LODs finer than the bake. Falls back to bilinear.
fn height_bicubic(xz: vec2<f32>) -> f32 {
    let l = height_level(xz, 0u, 1, 2);
    if l != 0u {
        return height_at(xz, 0u);
    }
    let c = level_coord(xz, 0u);
    let i0 = vec2<i32>(floor(c));
    let f = c - floor(c);
    let wx = catmull_rom_weights(f.x);
    let wz = catmull_rom_weights(f.y);
    let level = pi(P_HEIGHT);
    var h = 0.0;
    var lo = 1e30;
    var hi = -1e30;
    for (var j = 0; j < 4; j += 1) {
        var row = 0.0;
        for (var i = 0; i < 4; i += 1) {
            let s = height_texel(0u, level, i0 + vec2<i32>(i - 1, j - 1));
            row += s * wx[i];
            if i >= 1 && i <= 2 && j >= 1 && j <= 2 {
                lo = min(lo, s);
                hi = max(hi, s);
            }
        }
        h += row * wz[j];
    }
    // Clamp overshoot to the inner quad's range (as the bake's upsampler).
    return clamp(h, lo, hi);
}

/// Layer data at a render XZ: up to four (array slice, weight) pairs from a
/// bilinear merge of the four surrounding layer samples. Weights sum to 1.
struct Splat {
    ids: vec4<u32>,
    weights: vec4<f32>,
    valid: bool,
}

fn merge(s: ptr<function, Splat>, id: u32, w: f32) {
    if w <= 0.0 {
        return;
    }
    for (var k = 0; k < 4; k += 1) {
        if (*s).ids[k] == id && (*s).weights[k] > 0.0 {
            (*s).weights[k] += w;
            return;
        }
    }
    // Replace the lightest slot if this one is heavier.
    var m = 0;
    for (var k = 1; k < 4; k += 1) {
        if (*s).weights[k] < (*s).weights[m] {
            m = k;
        }
    }
    if w > (*s).weights[m] {
        (*s).ids[m] = id;
        (*s).weights[m] = w;
    }
}

fn layer_level(xz: vec2<f32>) -> u32 {
    let valid = params[P_VALID].y;
    for (var l = 0u; l < LAYER_LEVELS; l += 1u) {
        if ((valid >> l) & 1u) == 0u {
            continue;
        }
        let i0 = vec2<i32>(floor(level_coord(xz, l + 1u)));
        if in_window(pi(P_LAYER + l), i0, 0, 1) {
            return l;
        }
    }
    return LAYER_LEVELS;
}

fn splat_at(xz: vec2<f32>) -> Splat {
    var s: Splat;
    s.ids = vec4<u32>(0u);
    s.weights = vec4<f32>(0.0);
    s.valid = false;
    let l = layer_level(xz);
    if l >= LAYER_LEVELS {
        return s;
    }
    let level = pi(P_LAYER + l);
    let c = level_coord(xz, l + 1u);
    let i0 = vec2<i32>(floor(c));
    let f = c - floor(c);
    let corner_w = vec4<f32>((1.0 - f.x) * (1.0 - f.y), f.x * (1.0 - f.y), (1.0 - f.x) * f.y, f.x * f.y);
    var offsets = array<vec2<i32>, 4>(vec2<i32>(0, 0), vec2<i32>(1, 0), vec2<i32>(0, 1), vec2<i32>(1, 1));
    for (var k = 0; k < 4; k += 1) {
        let t = texel(level, i0 + offsets[k]);
        let ids = textureLoad(layer_id_clipmap, t, i32(l), 0);
        let ws = textureLoad(layer_weight_clipmap, t, i32(l), 0);
        for (var n = 0; n < 4; n += 1) {
            merge(&s, ids[n], ws[n] * corner_w[k]);
        }
    }
    let total = s.weights.x + s.weights.y + s.weights.z + s.weights.w;
    s.weights = s.weights / max(total, 1e-5);
    s.valid = true;
    return s;
}

fn layer_tiling(id: u32) -> f32 {
    return pf(P_LAYERS + 2u * id).x;
}

fn layer_displacement(id: u32) -> f32 {
    return pf(P_LAYERS + 2u * id).y;
}

fn layer_triplanar(id: u32) -> bool {
    return params[P_LAYERS + 2u * id].z != 0u;
}

fn layer_mean(id: u32) -> vec4<f32> {
    return pf(P_LAYERS + 2u * id + 1u);
}

/// Detail displacement (meters, around the layers' mean height): the splat
/// layers' height maps, weighted, scaled per layer. Sampled at a mip whose
/// texels match the vertex spacing.
fn detail_displacement(xz: vec2<f32>, spacing: f32) -> f32 {
    let s = splat_at(xz);
    if !s.valid {
        return 0.0;
    }
    let size = f32(textureDimensions(albedo_height_array).x);
    let wxz = world_xz(xz);
    var d = 0.0;
    for (var k = 0; k < 4; k += 1) {
        let w = s.weights[k];
        if w <= 0.0 {
            continue;
        }
        let id = s.ids[k];
        let tiling = layer_tiling(id);
        let mip = max(log2(spacing * size / tiling), 0.0);
        let h = textureSampleLevel(albedo_height_array, layer_sampler, wxz / tiling, i32(id), mip).a;
        d += w * (h - layer_mean(id).w) * layer_displacement(id);
    }
    return d;
}

/// The blended layer height (0..1; 0.5 past the splat range) at render XZ `xz`, `dist` meters
/// from the camera, prefiltered to texels of `spacing`: the vertex-rate twin of
/// `surface.wesl` `terrain_surface`'s `height` (the same Mishkinis height blend over top-down
/// heights and the same splat-range fade), for the raised cover's coverage (W6b).
fn layer_blend_height(xz: vec2<f32>, spacing: f32, dist: f32) -> f32 {
    let detail = pf(P_DETAIL);
    let near = 1.0 - smoothstep(0.7 * detail.x, detail.x, dist);
    if near <= 0.0 {
        return 0.5;
    }
    let s = splat_at(xz);
    if !s.valid {
        return 0.5;
    }
    let size = f32(textureDimensions(albedo_height_array).x);
    let wxz = world_xz(xz);
    var hl = vec4<f32>(0.0);
    var hw = vec4<f32>(-1e9);
    for (var k = 0; k < 4; k += 1) {
        let w = s.weights[k];
        if w > 0.0 {
            let id = s.ids[k];
            let tiling = layer_tiling(id);
            let mip = max(log2(spacing * size / tiling), 0.0);
            hl[k] = textureSampleLevel(albedo_height_array, layer_sampler, wxz / tiling, i32(id), mip).a;
            hw[k] = hl[k] + w;
        }
    }
    let top = max(max(hw.x, hw.y), max(hw.z, hw.w)) - detail.y;
    let b = max(hw - vec4<f32>(top), vec4<f32>(0.0));
    let bw = b / max(b.x + b.y + b.z + b.w, 1e-5);
    return mix(0.5, dot(bw, hl), near);
}

/// Raised-cover fade (W6b; twin `sf_terrain::cdlod::CdlodSettings::raise_fade`): 1 out to
/// `RAISE_FADE_START` × LOD 0's range, a smoothstep down to exactly 0 at `RAISE_FADE_END` × that
/// range, by the vertex's 3D distance `d` from the camera. A function of the vertex position only,
/// so neighbors across a LOD seam weigh shared vertices alike; it ends inside LOD 0, so LOD 1
/// (always past LOD 0's range) needs none.
const RAISE_FADE_START: f32 = 0.375;
const RAISE_FADE_END: f32 = 0.9;

fn raise_fade(d: f32) -> f32 {
    let m = pf(P_MORPH + u32(-MIN_LOD));
    let range0 = m.x + 1.0 / m.y;
    return 1.0 - smoothstep(RAISE_FADE_START * range0, RAISE_FADE_END * range0, d);
}

/// A morphed CDLOD vertex.
struct TerrainVertex {
    /// Render-space position.
    position: vec3<f32>,
    /// Patch grid coordinates before morphing (for the wireframe view).
    grid: vec2<f32>,
    /// LOD plus morph progress (fractional = morphing toward LOD + 1).
    lod: f32,
    /// Weight of the detail displacement: 1 at LOD -2, fading out across LOD -1's morph zone, 0
    /// from LOD 0.
    near_w: f32,
    /// Weight of the raised cover (W6b, `raise_fade`): smooth in world distance, 0 before LOD 0
    /// ends (and on every coarser level).
    raise_w: f32,
    /// The vertex spacing as morphed (m; the LOD's, doubling across its morph zone): the raised
    /// cover's texture prefilter, equal on both sides of a seam.
    texel_m: f32,
}

/// Strugar's CDLOD vertex: morph odd grid vertices toward their even
/// neighbors over the LOD's morph zone, then sample the height there.
/// `detail`: add detail displacement (off for shadow maps: a few centimeters
/// of relief there cost more than they show).
fn terrain_vertex(grid: vec2<f32>, instance_index: u32, detail: bool) -> TerrainVertex {
    let world_from_local = mesh_functions::get_world_from_local(instance_index);
    let tag = mesh_functions::get_tag(instance_index);
    let lod = i32(tag & 0xffu) + MIN_LOD;
    let morph = pf(P_MORPH + u32(lod - MIN_LOD));
    let spacing = morph.z;
    let node = world_from_local[3].xz;
    let lmin = u32(max(lod, 0));

    let p0 = node + grid * spacing;
    // The morph distance only needs a height every neighbor computes the same
    // way at shared vertices: bilinear is exact on the lattices they share.
    let h0 = height_at(p0, lmin);
    let d = distance(camera_position(), vec3<f32>(p0.x, h0 - origin_y(), p0.y));
    let k = clamp((d - morph.x) * morph.y, 0.0, 1.0);
    let odd = fract(grid * 0.5) * 2.0;
    let gm = grid - odd * k;
    let p = node + gm * spacing;

    var out: TerrainVertex;
    out.near_w = 0.0;
    var h: f32;
    if lod < 0 {
        h = height_bicubic(p);
        // Full detail at LOD -2, fading out across LOD -1's morph zone, so
        // it is zero where LOD -1 meets LOD 0.
        var w = 1.0 - k;
        if lod <= MIN_LOD {
            w = 1.0;
        }
        out.near_w = w;
        if detail && w > 0.0 {
            h += w * detail_displacement(p, spacing);
        }
    } else {
        // Blend toward the coarser level as the vertex morphs: fully morphed
        // vertices then read exactly what the LOD + 1 neighbor reads (levels
        // are quantized separately in the bake, so they differ by up to a
        // centimeter), and seams stay closed.
        h = height_at(p, lmin);
        if k > 0.0 {
            h = mix(h, height_at(p, lmin + 1u), k);
        }
    }

    out.position = vec3<f32>(p.x, h - origin_y(), p.y);
    out.grid = grid;
    out.lod = f32(lod) + k;
    out.raise_w = 0.0;
    if lod <= 0 {
        out.raise_w = raise_fade(distance(camera_position(), out.position));
    }
    out.texel_m = spacing * exp2(k);
    return out;
}

/// Raised cover over a terrain vertex (surface weather W6b, `sf_surface_bevy::view::surface_raise_at`):
/// x now, y now without prints, z last frame (motion vectors), in meters, weighted by `raise_w`
/// (LOD <= 0, faded smoothly in world distance to 0 inside LOD 0); w is y unweighted (the
/// fragment tilts its normal by w's slope times the weight, so the fade's own ramp never shades).
/// The coverage takes the layers' blended height at this vertex, as the shading takes it per
/// pixel. The main pass and the prepass both displace by this one function, so their depths
/// match; the node bounds grow by `TerrainSettings::raise_m`. Camera motion moves the fade with
/// it; z keeps this frame's weight (a smooth fade moves < 2 mm a frame at a run).
@if(SURFACE_WEATHER || SURFACE_WEATHER_PREPASS)
fn terrain_raise(t: TerrainVertex) -> vec4<f32> {
    if t.raise_w <= 0.0 {
        return vec4<f32>(0.0);
    }
    let base = layer_blend_height(t.position.xz, t.texel_m, distance(camera_position(), t.position));
    let r = surface_raise_at(t.position, terrain_normal(t.position.xz, t.lod), base, t.texel_m);
    return vec4<f32>(r * t.raise_w, r.y);
}

/// Normal at height level `l`: the baked central differences, filtered.
fn normal_at_level(xz: vec2<f32>, l: u32) -> vec3<f32> {
    let c = level_coord(xz, l);
    let t = vec2<f32>(pi(P_HEIGHT + l).xy) + c + 0.5;
    let n = textureSampleLevel(normal_clipmap, normal_sampler, t / f32(SIZE), i32(l), 0.0).rg;
    return vec3<f32>(n.x, sqrt(max(1.0 - dot(n, n), 0.0)), n.y);
}

/// The terrain normal at xz for a surface at LOD `lod` (fractional: blends
/// toward the next level as the geometry morphs).
fn terrain_normal(xz: vec2<f32>, lod: f32) -> vec3<f32> {
    let lf = max(lod, 0.0);
    let l = height_level(xz, u32(floor(lf)), 1, 2);
    if l >= HEIGHT_LEVELS {
        return vec3<f32>(0.0, 1.0, 0.0);
    }
    var n = normal_at_level(xz, l);
    let t = lf - floor(lf);
    if t > 0.01 && l == u32(floor(lf)) {
        let l1 = height_level(xz, l + 1u, 1, 2);
        if l1 < HEIGHT_LEVELS {
            n = normalize(mix(n, normal_at_level(xz, l1), t));
        }
    }
    return n;
}
