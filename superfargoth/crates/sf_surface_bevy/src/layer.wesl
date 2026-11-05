// Surface weather layer (`sf_surface_bevy::layer`; surface-weather.md "Shader integration").
// Every lit material calls `apply_surface_layer` after building its PbrInput and before
// lighting. Binding-agnostic: the caller passes the uniform and textures (`view.wesl` passes the
// mesh-view bindings of the vendored bevy_pbr patch "SW").
//
// As built W3: shelter exposure (vertical depth map, Clockwork's 6-texel tent over nine gathers
// with a soft cover ramp), the cell window lookup, the debug views, and wetness: Lagarde's wet
// surface model (porosity-weighted albedo darkening and gloss), a thin water film past
// saturation (water roughness and F0) that flattens the normal-map detail toward the
// geometric normal in Mikkelsen's surface-gradient framework, and world-plane wall streaks.
// Puddles (W4) and cover (W5) add to the same gradient before the one resolve.
//
// Ported shapes (constants re-calibrated in meters): `surfaceExposure` (tent weights, cover
// ramp ROOF_NEAR/ROOF_FULL, slope rise) and `surfaceWater` (wall streaks, WALL_WET, faces turned
// down stay dry, WET_SATURATION) from Clockwork, files/shaders/lib/light/surface.glsl,
// gitlab.com/OursCodeur/openmw branch clockwork-alpha, commit
// c5e5a9a6362c35512823884753d2d5284af204fa (GPL-3.0; THIRD_PARTY_NOTICES.md).
// Wet model: S. Lagarde, "Water drop 3b - Physically based wet surfaces" (2013).
// Normals: M. Mikkelsen, "Surface Gradient-Based Bump Mapping Framework" (JCGT 2020).
// The CPU twin of the wet math is `sf_surface::layer` (tests compare the constants).

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
    /// Reserved for W4: x seconds, y rain at the camera.
    time: vec4<f32>,
}

const FLAG_WET: u32 = 1u;
const FLAG_PUDDLE: u32 = 2u;
const FLAG_SNOW: u32 = 4u;
const FLAG_ASH: u32 = 8u;

const DEBUG_OFF: u32 = 0u;
const DEBUG_EXPOSURE: u32 = 1u;
const DEBUG_CELLS: u32 = 2u;
const DEBUG_POROSITY: u32 = 3u;

const CLASS_STATIC: u32 = 0u;
const CLASS_TERRAIN: u32 = 1u;
/// Skinned: wetness only, from `instance_wet` (W8).
const CLASS_DYNAMIC: u32 = 2u;
/// Far proxies and impostors: wetness darkening and gloss only.
const CLASS_FAR: u32 = 3u;

/// Horizontal offset of the shelter lookup along the surface normal, meters: a wall looks
/// past its own face (an eave that deep shelters it), a slope past its own rise.
const SHELTER_NORMAL_OFFSET_M: f32 = 0.35;
/// Cover starts sheltering this far above the point, and shelters whole from the second
/// height (Clockwork ROOF_NEAR 8 u, ROOF_FULL 24 u): no acne, no hard edge at a ledge.
const SHELTER_NEAR_M: f32 = 0.12;
const SHELTER_FULL_M: f32 = 0.35;
/// The outer part of the map (fraction of its half extent) fades to open sky.
const SHELTER_FADE_START: f32 = 0.875;

/// Lagarde's porosity heuristic: `(1 - metallic) * saturate((roughness - LOW) / SPAN)`.
const POROSITY_ROUGH_LOW: f32 = 0.3;
const POROSITY_ROUGH_SPAN: f32 = 0.5;
/// Albedo factor of a fully porous, soaked surface (Lagarde: lerp(1, 0.2, porosity)).
const WET_DARK_POROUS: f32 = 0.2;
/// Colour a soaked porous surface gains as it darkens (Clockwork WET_SATURATION).
const WET_SATURATION: f32 = 0.3;
/// Walls hold less water than the ground (Clockwork WALL_WET).
const WALL_WET: f32 = 0.6;
/// The water film: it forms past this wetness, porous surfaces keep this share of it
/// (they drink the rest), its perceptual roughness and its reflectance (F0 0.02).
const FILM_START: f32 = 0.35;
const FILM_POROUS_HOLD: f32 = 0.6;
const FILM_ROUGHNESS: f32 = 0.25;
const WATER_REFLECTANCE: f32 = 0.354;
/// Noise atlas tiling, meters: drying patches (R), wall streaks (B, A: narrow and tall).
const PATCH_TILE_M: f32 = 32.0;
const STREAK_TILE_U_M: f32 = 22.0;
const STREAK_TILE_V_M: f32 = 18.0;

const LUMA: vec3<f32> = vec3(0.2126, 0.7152, 0.0722);

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

/// Lagarde's porosity from the material: metals and smooth surfaces don't drink.
fn surface_porosity(metallic: f32, perceptual_roughness: f32) -> f32 {
    return (1.0 - metallic) * saturate((perceptual_roughness - POROSITY_ROUGH_LOW) / POROSITY_ROUGH_SPAN);
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
/// Clockwork's `surfaceExposure` shape: nine gathers cover a 6x6-texel tent (0.75 m of
/// slanting rain) around the point looked up `SHELTER_NORMAL_OFFSET_M` out along the normal;
/// a caster shelters by how far it sits above the point (ramp NEAR..FULL, raised by the
/// slope's own rise over two texels). Open sky outside the map (far roofs are exposed).
fn surface_exposure(shelter: texture_2d<f32>, shelter_sampler: sampler, u: SurfaceUniform, p: vec3<f32>, n_geo: vec3<f32>) -> f32 {
    if u.shelter.x == 0.0 {
        return 1.0;
    }
    let q = p + vec3(n_geo.x, 0.0, n_geo.z) * SHELTER_NORMAL_OFFSET_M;
    let c = u.shelter_clip_from_world * vec4(q, 1.0);
    let edge = max(abs(c.x), abs(c.y));
    if edge >= 1.0 {
        return 1.0;
    }
    let size = vec2<f32>(textureDimensions(shelter));
    let uv = c.xy * vec2(0.5, -0.5) + 0.5;
    // A slope rises across two texels without sheltering itself (tan capped at 4).
    let ny = max(abs(n_geo.y), 0.25);
    let rise = 2.0 * u.shelter.z * sqrt(max(1.0 - ny * ny, 0.0)) / ny;
    let near = vec4(SHELTER_NEAR_M + rise);
    let full = vec4(SHELTER_FULL_M + rise);
    let blend = fract(uv * size - 0.5);
    var open = 0.0;
    var weight = 0.0;
    for (var row = 0; row < 3; row++) {
        for (var column = 0; column < 3; column++) {
            let shift = vec2(f32(2 * column - 2), f32(2 * row - 2)) / size;
            // Reverse Z: a larger depth is higher; meters above the point.
            let over = (textureGather(0, shelter, shelter_sampler, uv + shift) - vec4(c.z)) * u.shelter.y;
            let gap = vec4(1.0) - smoothstep(near, full, over);
            // Tent weights of the four texels (gather order: (0,1), (1,1), (1,0), (0,0)).
            let low = vec2(f32(2 * column), f32(2 * row));
            let high = low + 1.0;
            let lw = select(low + 1.0 - blend, 5.0 - low + blend, low >= vec2(3.0));
            let hw = select(high + 1.0 - blend, 5.0 - high + blend, high >= vec2(3.0));
            let w = vec4(lw.x * hw.y, hw.x * hw.y, hw.x * lw.y, lw.x * lw.y);
            open += dot(gap, w);
            weight += dot(w, vec4(1.0));
        }
    }
    return mix(open / weight, 1.0, smoothstep(SHELTER_FADE_START, 1.0, edge));
}

/// Surface gradient of a perturbed normal `nn` against the geometric normal `n`
/// (Mikkelsen 2020, `SurfgradFromPerturbedNormal`); `surface_resolve` inverts it.
fn surface_gradient_of(n: vec3<f32>, nn: vec3<f32>) -> vec3<f32> {
    let k = dot(n, nn);
    return (k * n - nn) / max(abs(k), 1e-4);
}

fn surface_resolve(n: vec3<f32>, gradient: vec3<f32>) -> vec3<f32> {
    return normalize(n - gradient);
}

/// How much water a face keeps by its facing: ground whole, walls `WALL_WET` along world-plane
/// streaks (still as faces sway), faces turned down dry (Clockwork `surfaceWater`).
fn surface_facing_wet(noise: texture_2d<f32>, samp: sampler, p: vec3<f32>, n: vec3<f32>) -> f32 {
    let up = n.y;
    if up < -0.3 {
        return 0.0;
    }
    // 1: the face looks along X (its plane runs along Z), 0: along Z.
    let facing = n.x * n.x / max(n.x * n.x + n.z * n.z, 1e-4);
    let v = -p.y / STREAK_TILE_V_M;
    let along_z = textureSampleLevel(noise, samp, vec2(p.z / STREAK_TILE_U_M, v), 0.0).b;
    let along_x = textureSampleLevel(noise, samp, vec2(p.x / STREAK_TILE_U_M + 0.37, v), 0.0).a;
    let streak = smoothstep(0.05, 0.5, mix(along_x, along_z, facing));
    let wall = WALL_WET * (0.3 + 0.7 * streak);
    return mix(wall, 1.0, smoothstep(0.3, 0.8, up)) * smoothstep(-0.3, 0.0, up);
}

/// The film of standing water on a surface past saturation (0..1).
fn surface_film(wet: f32, porosity: f32) -> f32 {
    return wet * smoothstep(FILM_START, 1.0, wet) * (1.0 - FILM_POROUS_HOLD * porosity);
}

/// Lagarde's wet surface on the material: darker and richer by porosity, glossier, and the
/// film's water roughness and F0. Returns the film amount (it flattens the normal detail).
fn surface_wet_material(pbr: ptr<function, PbrInput>, wet: f32, porosity: f32) -> f32 {
    let albedo = (*pbr).material.base_color.rgb;
    let factor = mix(1.0, WET_DARK_POROUS, porosity);
    let richer = max(mix(vec3(dot(albedo, LUMA)), albedo, 1.0 + WET_SATURATION * wet * porosity), vec3(0.0));
    (*pbr).material.base_color = vec4(richer * mix(1.0, factor, wet), (*pbr).material.base_color.a);
    // Gloss rises as the pores fill (Lagarde: gloss = lerp(1, gloss, lerp(1, factor, 0.5 wet))).
    let gloss = 1.0 - (*pbr).material.perceptual_roughness;
    var rough = 1.0 - mix(1.0, gloss, mix(1.0, factor, 0.5 * wet));
    let film = surface_film(wet, porosity);
    rough = mix(rough, min(rough, FILM_ROUGHNESS), film);
    (*pbr).material.perceptual_roughness = rough;
    let dielectric_film = film * (1.0 - (*pbr).material.metallic);
    (*pbr).material.reflectance = mix((*pbr).material.reflectance, vec3(WATER_REFLECTANCE), dielectric_film);
    return film;
}

/// The layer: modifies `pbr` in place.
fn apply_surface_layer(
    pbr: ptr<function, PbrInput>,
    s: SurfaceInput,
    u: SurfaceUniform,
    cells: texture_2d<f32>,
    samp: sampler,
    shelter: texture_2d<f32>,
    shelter_sampler: sampler,
    noise: texture_2d<f32>,
) {
    let flags = u.misc.x;
    let debug = u.misc.y;
    // Dry, bare and no debug view: one uniform branch (design rule 3).
    if flags == 0u && debug == DEBUG_OFF {
        return;
    }
    if debug != DEBUG_OFF {
        let albedo = (*pbr).material.base_color.rgb;
        let luma = dot(albedo, LUMA);
        let shade = 0.45 + 0.55 * sqrt(saturate(luma * 2.0));
        var tint = vec3(0.5);
        if debug == DEBUG_EXPOSURE {
            let e = surface_exposure(shelter, shelter_sampler, u, s.world_pos, (*pbr).world_normal);
            // Sheltered blue, open sky warm yellow; detail kept through the albedo's luminance.
            tint = mix(vec3(0.05, 0.2, 1.0), vec3(1.0, 0.8, 0.15), e);
        } else if debug == DEBUG_CELLS {
            let v = surface_cell_load(cells, u, s.world_pos);
            // Wet blue, puddle green, ash red, snow white over grey; dark 32 m cell borders.
            tint = vec3(0.15) + vec3(v.a, v.g, v.r) * 0.75 + vec3(v.b * 0.6);
            let f = fract(surface_cell_coord(u, s.world_pos));
            let border = min(min(f.x, 1.0 - f.x), min(f.y, 1.0 - f.y));
            tint *= select(1.0, 0.3, border < 0.015);
            if !surface_in_window(u, s.world_pos) {
                tint = vec3(0.5, 0.0, 0.5);
            }
        } else if debug == DEBUG_POROSITY {
            // Porosity black (sealed: metal, polish) to white (drinks); far proxies red-tinted.
            tint = vec3(s.porosity) * select(vec3(1.0), vec3(1.0, 0.5, 0.5), s.surface_class == CLASS_FAR);
        }
        (*pbr).material.base_color = vec4(tint * shade, (*pbr).material.base_color.a);
        return;
    }
    if (flags & FLAG_WET) == 0u {
        return;
    }
    let n_geo = (*pbr).world_normal;
    var wet = 0.0;
    if s.surface_class == CLASS_DYNAMIC {
        wet = s.instance_wet;
    } else {
        wet = surface_cells_at(cells, samp, u, s.world_pos).r;
        if wet <= 0.0 {
            return;
        }
        wet *= surface_facing_wet(noise, samp, s.world_pos, n_geo);
        // Drying leaves patches: low noise dries last.
        let n = textureSampleLevel(noise, samp, s.world_pos.xz / PATCH_TILE_M, 0.0).r;
        wet = smoothstep(n * 0.6, n * 0.6 + 0.4, wet);
        if s.surface_class != CLASS_FAR && wet > 0.0 {
            wet *= surface_exposure(shelter, shelter_sampler, u, s.world_pos, n_geo);
        }
    }
    if wet <= 0.0 {
        return;
    }
    // Normals compose as surface gradients; W4-W5 add theirs before the one resolve.
    var gradient = surface_gradient_of(n_geo, (*pbr).N);
    let film = surface_wet_material(pbr, wet, s.porosity);
    if s.surface_class == CLASS_FAR {
        return;
    }
    // The film fills the micro-detail: the normal map's gradient fades toward the macro normal.
    gradient *= 1.0 - film;
    (*pbr).N = surface_resolve(n_geo, gradient);
}
