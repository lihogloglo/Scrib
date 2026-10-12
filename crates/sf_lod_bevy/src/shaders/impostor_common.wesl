// Octahedral impostors: bindings and the vertex logic shared by the main
// and prepass shaders. Mirrors sf_lod::octa (encode, frame_dir,
// frame_basis, blend_frames) exactly. Spec: docs/specs/far-field.md.
//
// Mesh: four vertices per instance. @location(0) the capture sphere's
// center (node space), @location(1) the corner in [-1, 1]², @location(2)
// (radius × scale, yaw, atlas layer, 0).

import bevy_pbr::render::{mesh_functions, mesh_view_bindings::view};

struct ImpostorParams {
    // x: frames per side, y: mode (0 hemi, 1 full), z: frame texels, w: unused
    grid: vec4<u32>,
    // rgb: debug tint (white = none), a: alpha cutoff
    tint: vec4<f32>,
    // the far shadow map: (min x, min z, 1 / extent, base height); extent 0 = none
    far: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> params: ImpostorParams;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var albedo_atlas: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var atlas_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var normal_depth_atlas: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(4) var far_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(5) var far_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) center: vec3<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) data: vec4<f32>,
}

struct Impostor {
    world_position: vec3<f32>,
    // Per frame: uv in [0, 1]² of that frame.
    uv0: vec2<f32>,
    uv1: vec2<f32>,
    uv2: vec2<f32>,
    weights: vec3<f32>,
    // Per frame: i + j * n.
    frames: vec3<u32>,
    layer: u32,
    yaw: f32,
}

fn rotate_y(v: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(c * v.x + s * v.z, v.y, -s * v.x + c * v.z);
}

fn sign2(v: vec2<f32>) -> vec2<f32> {
    return select(vec2<f32>(-1.0), vec2<f32>(1.0), v >= vec2<f32>(0.0));
}

fn octa_encode(mode: u32, dir: vec3<f32>) -> vec2<f32> {
    let d = normalize(dir);
    let p = d / (abs(d.x) + abs(d.y) + abs(d.z));
    var e: vec2<f32>;
    if mode == 0u {
        let q = vec3<f32>(p.x, max(p.y, 0.0), p.z);
        let r = q / max(abs(q.x) + q.y + abs(q.z), 1e-20);
        e = vec2<f32>(r.x + r.z, r.x - r.z);
    } else {
        let xz = vec2<f32>(p.x, p.z);
        e = select((vec2<f32>(1.0) - vec2<f32>(abs(xz.y), abs(xz.x))) * sign2(xz), xz, p.y >= 0.0);
    }
    return clamp(e * 0.5 + 0.5, vec2<f32>(0.0), vec2<f32>(1.0));
}

fn octa_decode(mode: u32, uv: vec2<f32>) -> vec3<f32> {
    let e = uv * 2.0 - 1.0;
    if mode == 0u {
        let x = (e.x + e.y) * 0.5;
        let z = (e.x - e.y) * 0.5;
        return normalize(vec3<f32>(x, 1.0 - abs(x) - abs(z), z));
    }
    let y = 1.0 - abs(e.x) - abs(e.y);
    let t = max(-y, 0.0);
    let xz = e - sign2(e) * t;
    return normalize(vec3<f32>(xz.x, y, xz.y));
}

fn frame_dir(mode: u32, n: u32, ij: vec2<u32>) -> vec3<f32> {
    let s = f32(max(n, 2u) - 1u);
    return octa_decode(mode, vec2<f32>(ij) / s);
}

// (right, up, toward viewer), right-handed, up as close to +Y as possible.
fn frame_basis(dir: vec3<f32>) -> mat3x3<f32> {
    let f = normalize(dir);
    let world_up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, -1.0), abs(f.y) > 0.999);
    let right = normalize(cross(world_up, f));
    let up = cross(f, right);
    return mat3x3<f32>(right, up, f);
}

// UV of a model-space offset `q` (already divided by the radius) in frame
// `ij`, projected along the view direction `dm` onto the frame's plane.
fn frame_uv(mode: u32, n: u32, ij: vec2<u32>, q: vec3<f32>, dm: vec3<f32>) -> vec2<f32> {
    let b = frame_basis(frame_dir(mode, n, ij));
    let t = dot(q, b[2]) / max(dot(dm, b[2]), 0.1);
    let h = q - dm * t;
    return vec2<f32>(dot(h, b[0]) * 0.5 + 0.5, 0.5 - dot(h, b[1]) * 0.5);
}

// Builds the camera-facing quad and the three frames to blend.
fn impostor_vertex(v: Vertex) -> Impostor {
    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    let center = (world_from_local * vec4<f32>(v.center, 1.0)).xyz;
    let radius = v.data.x;
    let yaw = v.data.y;
    let n = params.grid.x;
    let mode = params.grid.y;

    // Toward the viewer: from the camera position, or the view's +Z for
    // orthographic (shadow) views.
    let ortho = view.clip_from_view[3].w == 1.0;
    let to_cam = view.world_position - center;
    let dir = select(normalize(to_cam), normalize(view.world_from_view[2].xyz), ortho);
    let quad = frame_basis(dir);
    var out: Impostor;
    out.world_position = center + (quad[0] * v.corner.x + quad[1] * v.corner.y) * radius;

    // Model space: undo the instance's yaw.
    let dm = rotate_y(dir, -yaw);
    let q = rotate_y(out.world_position - center, -yaw) / radius;

    let s = f32(max(n, 2u) - 1u);
    let g = octa_encode(mode, dm) * s;
    let base = clamp(floor(g), vec2<f32>(0.0), vec2<f32>(s - 1.0));
    let f = g - base;
    let ij = vec2<u32>(base);
    var a: vec2<u32>;
    var b: vec2<u32>;
    let c = ij + vec2<u32>(1u, 1u);
    if f.x >= f.y {
        a = ij;
        b = ij + vec2<u32>(1u, 0u);
        out.weights = vec3<f32>(1.0 - f.x, f.x - f.y, f.y);
    } else {
        a = ij;
        b = ij + vec2<u32>(0u, 1u);
        out.weights = vec3<f32>(1.0 - f.y, f.y - f.x, f.x);
    }
    out.uv0 = frame_uv(mode, n, a, q, dm);
    out.uv1 = frame_uv(mode, n, b, q, dm);
    out.uv2 = frame_uv(mode, n, c, q, dm);
    out.frames = vec3<u32>(a.x + a.y * n, b.x + b.y * n, c.x + c.y * n);
    out.layer = u32(v.data.z + 0.5);
    out.yaw = yaw;
    return out;
}

struct ImpostorSample {
    albedo: vec3<f32>,
    alpha: f32,
    // Model-space normal.
    normal: vec3<f32>,
}

fn sample_frame(frame: u32, uv: vec2<f32>, layer: u32) -> array<vec4<f32>, 2> {
    let n = params.grid.x;
    let texel = 1.0 / f32(params.grid.z);
    let inside = all(uv >= vec2<f32>(0.0)) && all(uv <= vec2<f32>(1.0));
    // Half a texel in from the frame edge, so filtering stays in the frame.
    let local = clamp(uv, vec2<f32>(0.5 * texel), vec2<f32>(1.0 - 0.5 * texel));
    let cell = vec2<f32>(f32(frame % n), f32(frame / n));
    let atlas_uv = (cell + local) / f32(n);
    var a = textureSample(albedo_atlas, atlas_sampler, atlas_uv, layer);
    let nd = textureSample(normal_depth_atlas, atlas_sampler, atlas_uv, layer);
    a.a = select(0.0, a.a, inside);
    return array<vec4<f32>, 2>(a, nd);
}

fn impostor_sample(uv0: vec2<f32>, uv1: vec2<f32>, uv2: vec2<f32>, weights: vec3<f32>, frames: vec3<u32>, layer: u32) -> ImpostorSample {
    let s0 = sample_frame(frames.x, uv0, layer);
    let s1 = sample_frame(frames.y, uv1, layer);
    let s2 = sample_frame(frames.z, uv2, layer);
    let w = weights * vec3<f32>(s0[0].a, s1[0].a, s2[0].a);
    let wsum = max(w.x + w.y + w.z, 1e-5);
    var out: ImpostorSample;
    out.alpha = dot(weights, vec3<f32>(s0[0].a, s1[0].a, s2[0].a));
    out.albedo = (s0[0].rgb * w.x + s1[0].rgb * w.y + s2[0].rgb * w.z) / wsum;
    let nrm = (s0[1].xyz * w.x + s1[1].xyz * w.y + s2[1].xyz * w.z) / wsum;
    out.normal = normalize(nrm * 2.0 - 1.0);
    return out;
}
