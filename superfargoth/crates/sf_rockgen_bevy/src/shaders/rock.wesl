// Rock main pass (rockgen.md "Materials", slice RG-3): Bevy's StandardMaterial fragment (forward; the
// extension forces the forward path) with base colour, normal, roughness / metallic and occlusion from a
// world-axis triplanar projection of one PBR texture set, so rock meshes need no UVs. The prepass and
// shadow passes are Bevy's own (geometric normal).
//
// TEMPORARY SCAFFOLD (RG-3, named "rock triplanar scaffold"): the triplanar + hex-tiling code below is a
// minimal copy of the terrain's `sample_layer_triplanar` / `hex_*` (sf_terrain_bevy shaders/surface.wesl),
// which reads the terrain's own layer arrays and parameter slots and could not be imported as-is.
// Removal path (slice RG-3b, after the terrain LOD fix lands): move the projection and hex-tiling
// functions out of surface.wesl into a shared WESL module parameterised by texture + sampler, import it
// here and in the terrain, and delete this copy.
//
// Projection space: world axes, origin at the rock's own translation (`world - translation`), plus a
// per-rock offset from its MeshTag. That keeps the texture fixed on the rock under the floating origin
// (the render origin moves by whole grid cells; the rock's translation moves with it) and gives the
// terrain's texel density (`inv_tiling` = 1 / the terrain layer tiling, set by the game).

import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    mesh_functions::{get_world_from_local, get_tag},
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

/// `RockParams` (rust `material.rs`): the same fields in the same order.
struct RockParams {
    inv_tiling: f32,
    blend_sharpness: f32,
    hex_contrast: f32,
    hex_rotation: f32,
    hex_scale: f32,
    flags: u32,
    lod: u32,
    pad: u32,
}

// Flag bits (rust `material.rs` FLAG_*).
const FLAG_ALBEDO: u32 = 1u;
const FLAG_NORMAL: u32 = 2u;
const FLAG_ORM: u32 = 4u;
const FLAG_OCCLUSION: u32 = 8u;
const FLAG_FLIP_Y: u32 = 16u;
const FLAG_HEX: u32 = 32u;
const FLAG_LOD_TINT: u32 = 64u;

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> rock: RockParams;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var rock_albedo: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var rock_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var rock_normal: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var rock_orm: texture_2d<f32>;

/// One texture-set sample: albedo (rgba), tangent-space normal, ORM (occlusion, roughness, metallic).
struct RockSample {
    albedo: vec4<f32>,
    normal: vec3<f32>,
    orm: vec3<f32>,
}

fn has(bit: u32) -> bool {
    return (rock.flags & bit) != 0u;
}

/// The three maps at one UV with explicit derivatives (no derivative ops in non-uniform control flow).
fn fetch(uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>) -> RockSample {
    var s: RockSample;
    s.albedo = textureSampleGrad(rock_albedo, rock_sampler, uv, ddx, ddy);
    s.normal = vec3<f32>(0.0, 0.0, 1.0);
    if has(FLAG_NORMAL) {
        // XY only, Z rebuilt: works for RGB and two-channel (BC5 / RG) normal maps alike.
        var nxy = textureSampleGrad(rock_normal, rock_sampler, uv, ddx, ddy).xy * 2.0 - 1.0;
        if has(FLAG_FLIP_Y) {
            nxy.y = -nxy.y;
        }
        s.normal = vec3<f32>(nxy, sqrt(max(1.0 - dot(nxy, nxy), 0.0)));
    }
    s.orm = textureSampleGrad(rock_orm, rock_sampler, uv, ddx, ddy).rgb;
    return s;
}

/// Three uniform hashes in [0, 1] per (hex vertex, salt): x = angle, yz = UV offset (the terrain's).
fn hex_hash3(v: vec2<i32>, salt: u32) -> vec3<f32> {
    var h = (u32(v.x) * 0x8da6b343u) ^ (u32(v.y) * 0xd8163841u) ^ (salt * 0x9e3779b9u);
    h = (h ^ (h >> 15u)) * 0x2c1b3c6du;
    let h0 = h ^ (h >> 15u);
    h = (h0 ^ (h0 >> 12u)) * 0x297a2d39u;
    let h1 = h ^ (h >> 15u);
    h = (h1 ^ (h1 >> 13u)) * 0x85ebca6bu;
    let h2 = h ^ (h >> 16u);
    return vec3<f32>(f32(h0 & 0xffffu), f32(h1 & 0xffffu), f32(h2 & 0xffffu)) * (1.0 / 65535.0);
}

/// One projection: plain, or Mikkelsen (2022) hex-tiling over a skewed triangle grid of `hex_scale` cells
/// per repeat, each vertex a randomly rotated and offset copy, contrast-weighted, sub-LSB taps pruned. Not
/// periodic (the terrain's copy wraps on its 1024 m torus): rock coordinates are small and rock-local.
fn projection(uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>, salt: u32) -> RockSample {
    if !has(FLAG_HEX) {
        return fetch(uv, ddx, ddy);
    }
    let sx = rock.hex_scale;
    let sy = rock.hex_scale * 1.15470054;
    let skewed = vec2<f32>(uv.x * sx - 0.5 * uv.y * sy, uv.y * sy);
    let base = vec2<i32>(floor(skewed));
    let f = skewed - floor(skewed);
    var verts: array<vec2<i32>, 3>;
    var w: vec3<f32>;
    if f.x + f.y <= 1.0 {
        verts = array<vec2<i32>, 3>(base, base + vec2<i32>(1, 0), base + vec2<i32>(0, 1));
        w = vec3<f32>(1.0 - f.x - f.y, f.x, f.y);
    } else {
        verts = array<vec2<i32>, 3>(base + vec2<i32>(1, 1), base + vec2<i32>(1, 0), base + vec2<i32>(0, 1));
        w = vec3<f32>(f.x + f.y - 1.0, 1.0 - f.y, 1.0 - f.x);
    }
    w = pow(max(w, vec3<f32>(1e-5)), vec3<f32>(rock.hex_contrast));
    w /= max(w.x + w.y + w.z, 1e-6);
    w = select(vec3<f32>(0.0), w, w > vec3<f32>(0.002));
    w /= max(w.x + w.y + w.z, 1e-6);

    var out: RockSample;
    out.albedo = vec4<f32>(0.0);
    out.orm = vec3<f32>(0.0);
    var n_acc = vec3<f32>(0.0);
    for (var k = 0u; k < 3u; k += 1u) {
        let wk = w[k];
        if wk > 0.0 {
            let v = verts[k];
            let center = vec2<f32>((f32(v.x) + 0.5 * f32(v.y)) / sx, f32(v.y) / sy);
            let rnd = hex_hash3(v, salt);
            let theta = (rnd.x * 2.0 - 1.0) * 3.14159265 * rock.hex_rotation;
            let c = cos(theta);
            let s = sin(theta);
            let duv = uv - center;
            let tuv = vec2<f32>(c * duv.x - s * duv.y, s * duv.x + c * duv.y) + rnd.yz;
            let tdx = vec2<f32>(c * ddx.x - s * ddx.y, s * ddx.x + c * ddx.y);
            let tdy = vec2<f32>(c * ddy.x - s * ddy.y, s * ddy.x + c * ddy.y);
            let raw = fetch(tuv, tdx, tdy);
            // Rotate the tangent normal back by R(-theta) so relief lights along the rotated features.
            let rn = vec3<f32>(c * raw.normal.x + s * raw.normal.y, -s * raw.normal.x + c * raw.normal.y, raw.normal.z);
            out.albedo += wk * raw.albedo;
            out.orm += wk * raw.orm;
            n_acc += wk * rn;
        }
    }
    out.normal = normalize(n_acc);
    return out;
}

/// X/Y/Z triplanar with whiteout normal blending (Golus 2017), axes under 1 % weight skipped (the
/// terrain's `sample_layer_triplanar`).
fn triplanar(p: vec3<f32>, dpx: vec3<f32>, dpy: vec3<f32>, n: vec3<f32>, salt: u32) -> RockSample {
    var blend = pow(max(abs(n), vec3<f32>(1e-5)), vec3<f32>(rock.blend_sharpness));
    blend /= max(blend.x + blend.y + blend.z, 1e-6);
    blend = select(vec3<f32>(0.0), blend, blend > vec3<f32>(0.01));
    blend /= max(blend.x + blend.y + blend.z, 1e-6);

    var out: RockSample;
    out.albedo = vec4<f32>(0.0);
    out.orm = vec3<f32>(0.0);
    var nw = vec3<f32>(0.0);
    if blend.x > 0.0 {
        let sx = projection(p.zy, dpx.zy, dpy.zy, salt);
        let nx = vec3<f32>(sx.normal.xy + n.zy, abs(sx.normal.z) * n.x);
        out.albedo += sx.albedo * blend.x;
        out.orm += sx.orm * blend.x;
        nw += nx.zyx * blend.x;
    }
    if blend.y > 0.0 {
        let sy = projection(p.xz, dpx.xz, dpy.xz, salt + 1u);
        let ny = vec3<f32>(sy.normal.xy + n.xz, abs(sy.normal.z) * n.y);
        out.albedo += sy.albedo * blend.y;
        out.orm += sy.orm * blend.y;
        nw += ny.xzy * blend.y;
    }
    if blend.z > 0.0 {
        let sz = projection(p.xy, dpx.xy, dpy.xy, salt + 2u);
        let nz = vec3<f32>(sz.normal.xy + n.xy, abs(sz.normal.z) * n.z);
        out.albedo += sz.albedo * blend.z;
        out.orm += sz.orm * blend.z;
        nw += nz.xyz * blend.z;
    }
    out.normal = normalize(nw);
    return out;
}

/// LOD debug tint: LOD0 green, then yellow, orange, red, magenta, blue, cyan, white.
fn lod_tint(lod: u32) -> vec3<f32> {
    var tints = array<vec3<f32>, 8>(
        vec3<f32>(0.2, 1.0, 0.2),
        vec3<f32>(1.0, 1.0, 0.2),
        vec3<f32>(1.0, 0.55, 0.1),
        vec3<f32>(1.0, 0.15, 0.1),
        vec3<f32>(1.0, 0.2, 1.0),
        vec3<f32>(0.2, 0.4, 1.0),
        vec3<f32>(0.2, 1.0, 1.0),
        vec3<f32>(1.0, 1.0, 1.0),
    );
    return tints[min(lod, 7u)];
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    // Rock-local, world-axis position in texture repeats, plus the rock's offset (MeshTag hash).
    let origin = get_world_from_local(in.instance_index)[3].xyz;
    let salt = get_tag(in.instance_index);
    let offset = hex_hash3(vec2<i32>(i32(salt & 0xffffu), i32(salt >> 16u)), 7u) * 16.0;
    let p = (in.world_position.xyz - origin) * rock.inv_tiling + offset;
    let dpx = dpdx(p);
    let dpy = dpdy(p);

    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    let n = normalize(pbr.world_normal);
    let t = triplanar(p, dpx, dpy, n, salt);
    if has(FLAG_ALBEDO) {
        pbr.material.base_color = vec4<f32>(pbr.material.base_color.rgb * t.albedo.rgb, pbr.material.base_color.a);
    }
    if has(FLAG_NORMAL) {
        pbr.N = t.normal;
    }
    if has(FLAG_ORM) {
        pbr.material.perceptual_roughness = clamp(pbr.material.perceptual_roughness * t.orm.g, 0.089, 1.0);
        pbr.material.metallic = pbr.material.metallic * t.orm.b;
        if has(FLAG_OCCLUSION) {
            pbr.diffuse_occlusion *= t.orm.r;
        }
    }
    if has(FLAG_LOD_TINT) {
        pbr.material.base_color = vec4<f32>(pbr.material.base_color.rgb * lod_tint(rock.lod), pbr.material.base_color.a);
    }

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
