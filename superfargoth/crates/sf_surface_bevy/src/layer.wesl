// Surface weather layer (`sf_surface_bevy::layer`; surface-weather.md "Shader integration").
// Every lit material calls `apply_surface_layer` after building its PbrInput and before
// lighting. Binding-agnostic: the caller passes the uniform and textures (terrain: its
// material bindings; W3: the mesh-view bindings of the vendored bevy_pbr patch "SW").
//
// As built W2: the shelter exposure (vertical depth map, tent-filtered 3x3 load, normal
// offset) and the cell window lookup, shown only by the debug views (`surface debug
// exposure|cells`); the wet, puddle and cover layers (W3-W5) are not in yet, so with the
// debug view off the layer changes nothing.

import bevy_pbr::render::pbr_types::PbrInput;

/// Keep in sync with `uniform.rs` (slots of vec4).
struct SurfaceUniform {
    /// Render space -> shelter clip (x, y in -1..1; z reverse depth: 1 top plane, 0 bottom).
    shelter_clip_from_world: mat4x4<f32>,
    /// xy: render-space XZ of the cell window's first cell's min corner; z: 1 / cell (1/m);
    /// w: 1 when the window is placed.
    window: vec4<f32>,
    /// xy: the first cell's texel (cell mod 128); z: window edge in cells.
    cells: vec4<i32>,
    /// x: 1 when the shelter map is valid; y: its depth range (m); z: texel (m); w: half extent (m).
    shelter: vec4<f32>,
    /// x: layer flags (FLAG_*); y: debug view (DEBUG_*).
    misc: vec4<u32>,
    /// Reserved for W3-W4: x seconds, y rain at the camera.
    time: vec4<f32>,
}

const FLAG_WET: u32 = 1u;
const FLAG_PUDDLE: u32 = 2u;
const FLAG_SNOW: u32 = 4u;
const FLAG_ASH: u32 = 8u;

const DEBUG_OFF: u32 = 0u;
const DEBUG_EXPOSURE: u32 = 1u;
const DEBUG_CELLS: u32 = 2u;

const CLASS_STATIC: u32 = 0u;
const CLASS_TERRAIN: u32 = 1u;
/// Skinned: wetness only.
const CLASS_DYNAMIC: u32 = 2u;
/// Far proxies and impostors: wetness only.
const CLASS_FAR: u32 = 3u;

/// Horizontal offset of the shelter lookup along the surface normal, meters: a wall looks
/// past its own face (an eave that deep shelters it), a slope past its own rise.
const SHELTER_NORMAL_OFFSET_M: f32 = 0.35;
/// Constant shelter bias, meters (depth quantisation and filtering).
const SHELTER_BIAS_M: f32 = 0.05;
/// The outer part of the map (fraction of its half extent) fades to open sky.
const SHELTER_FADE_START: f32 = 0.875;

struct SurfaceInput {
    /// Render space (the uniform's window and shelter follow the floating origin).
    world_pos: vec3<f32>,
    /// 0..1 height signal: terrain layer height; objects: the noise atlas (W4-W5).
    height: f32,
    /// Terrain only: concavity from the height clipmap (puddle sites, W4), else 0.
    basin: f32,
    porosity: f32,
    surface_class: u32,
    /// DYNAMIC: per-character wetness (W8), else 0.
    instance_wet: f32,
}

/// Continuous cell coordinates from the window's first cell.
fn surface_cell_coord(u: SurfaceUniform, p: vec3<f32>) -> vec2<f32> {
    return (p.xz - u.window.xy) * u.window.z;
}

/// Whether render-space `p` lies inside the cell window.
fn surface_in_window(u: SurfaceUniform, p: vec3<f32>) -> bool {
    let c = surface_cell_coord(u, p);
    let n = f32(u.cells.z);
    return u.window.w > 0.0 && all(c >= vec2(0.0)) && all(c < vec2(n));
}

/// The cell texture at `p`, bilinear between cell centres (repeat sampler: the torus wraps
/// on its own). Zero outside the window.
fn surface_cells_at(cells: texture_2d<f32>, samp: sampler, u: SurfaceUniform, p: vec3<f32>) -> vec4<f32> {
    if !surface_in_window(u, p) {
        return vec4(0.0);
    }
    let n = f32(u.cells.z);
    let uv = (surface_cell_coord(u, p) + vec2<f32>(u.cells.xy)) / n;
    return textureSampleLevel(cells, samp, uv, 0.0);
}

/// The cell texel holding `p` (no filtering; debug view).
fn surface_cell_load(cells: texture_2d<f32>, u: SurfaceUniform, p: vec3<f32>) -> vec4<f32> {
    if !surface_in_window(u, p) {
        return vec4(0.0);
    }
    let n = u.cells.z;
    let t = (vec2<i32>(floor(surface_cell_coord(u, p))) + u.cells.xy) % vec2(n);
    return textureLoad(cells, (t + vec2(n)) % vec2(n), 0);
}

/// Exposure to the open sky at `p` with geometric normal `n_geo`: 1 open, 0 under cover.
/// Tent-filtered 3x3 comparison against the vertical shelter depth map, looked up
/// `SHELTER_NORMAL_OFFSET_M` out along the normal, with a slope-scaled bias; open sky
/// outside the map (far roofs are exposed).
fn surface_exposure(shelter: texture_2d<f32>, u: SurfaceUniform, p: vec3<f32>, n_geo: vec3<f32>) -> f32 {
    if u.shelter.x == 0.0 {
        return 1.0;
    }
    let q = p + vec3(n_geo.x, 0.0, n_geo.z) * SHELTER_NORMAL_OFFSET_M;
    let c = u.shelter_clip_from_world * vec4(q, 1.0);
    let edge = max(abs(c.x), abs(c.y));
    if edge >= 1.0 {
        return 1.0;
    }
    let size = vec2<i32>(textureDimensions(shelter));
    let uv = c.xy * vec2(0.5, -0.5) + 0.5;
    let center = clamp(vec2<i32>(floor(uv * vec2<f32>(size))), vec2(0), size - 1);
    // Slope bias: a texel's worth of rise along the steepest slope (tan capped at 4).
    let ny = max(abs(n_geo.y), 0.2);
    let tan_slope = min(sqrt(max(1.0 - ny * ny, 0.0)) / ny, 4.0);
    let depth = c.z + (SHELTER_BIAS_M + u.shelter.z * 1.5 * tan_slope) / u.shelter.y;
    var open = 0.0;
    for (var dy = -1; dy <= 1; dy++) {
        for (var dx = -1; dx <= 1; dx++) {
            let w = f32((2 - abs(dx)) * (2 - abs(dy))) / 16.0;
            let t = clamp(center + vec2(dx, dy), vec2(0), size - 1);
            let caster = textureLoad(shelter, t, 0).r;
            open += select(w, 0.0, caster > depth);
        }
    }
    return mix(open, 1.0, smoothstep(SHELTER_FADE_START, 1.0, edge));
}

/// The layer: modifies `pbr` in place (W2: debug views only).
fn apply_surface_layer(
    pbr: ptr<function, PbrInput>,
    s: SurfaceInput,
    u: SurfaceUniform,
    cells: texture_2d<f32>,
    cell_sampler: sampler,
    shelter: texture_2d<f32>,
    noise: texture_2d<f32>,
    noise_sampler: sampler,
) {
    let debug = u.misc.y;
    // Dry, bare and no debug view: one uniform branch (design rule 3).
    if u.misc.x == 0u && debug == DEBUG_OFF {
        return;
    }
    let albedo = (*pbr).material.base_color.rgb;
    let luma = dot(albedo, vec3(0.2126, 0.7152, 0.0722));
    let shade = 0.45 + 0.55 * sqrt(saturate(luma * 2.0));
    if debug == DEBUG_EXPOSURE {
        let e = surface_exposure(shelter, u, s.world_pos, (*pbr).world_normal);
        // Sheltered blue, open sky warm yellow; detail kept through the albedo's luminance.
        let tint = mix(vec3(0.05, 0.2, 1.0), vec3(1.0, 0.8, 0.15), e);
        (*pbr).material.base_color = vec4(tint * shade, (*pbr).material.base_color.a);
        return;
    }
    if debug == DEBUG_CELLS {
        let v = surface_cell_load(cells, u, s.world_pos);
        // Wet blue, puddle green, ash red, snow white over grey; dark 32 m cell borders.
        var tint = vec3(0.15) + vec3(v.a, v.g, v.r) * 0.75 + vec3(v.b * 0.6);
        let f = fract(surface_cell_coord(u, s.world_pos));
        let border = min(min(f.x, 1.0 - f.x), min(f.y, 1.0 - f.y));
        tint *= select(1.0, 0.3, border < 0.015);
        if !surface_in_window(u, s.world_pos) {
            tint = vec3(0.5, 0.0, 0.5);
        }
        (*pbr).material.base_color = vec4(tint * shade, (*pbr).material.base_color.a);
        return;
    }
    // W3-W5: exposure, cover, wetness, puddles, gradient resolve.
}
