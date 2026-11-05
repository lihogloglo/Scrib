// Surface weather layer (`sf_surface_bevy::layer`; surface-weather.md "Shader integration").
// Every lit material calls `apply_surface_layer` after building its PbrInput and before
// lighting. Binding-agnostic: the caller passes the uniform and textures (`view.wesl` passes the
// mesh-view bindings of the vendored bevy_pbr patch "SW").
//
// As built W3 (+ W2b canopy): shelter exposure (vertical depth maps, solid and leaves,
// Clockwork's 6-texel tent over nine gathers with a soft cover ramp), the cell window lookup,
// the debug views, and wetness: Lagarde's wet surface model (porosity-weighted albedo
// darkening and gloss), a thin water film past saturation (water roughness and F0) that flattens
// the normal-map detail toward the geometric normal in Mikkelsen's surface-gradient framework,
// and world-plane wall streaks.
// Puddles (W4): Lagarde's accumulated water (a level against the height map, soft shoreline,
// a flat water surface with F0 0.02 and mirror gloss) on level faces only (`level_up`: the true
// surface, not a smoothed vertex normal) with procedural rain rings; their gradient joins the
// same sum. Cover (W5b, snow and ash): the cell's amount × a slope ramp × exposure, a height
// blend (height lerp: the cover's level rises over the surface's own height mixed with the cover
// texture's and a drift noise), the look's albedo, roughness, F0 and AO, its normal's gradient
// replacing the surface's in the sum, and snow glints; on statics and terrain only.
//
// Ported shapes (constants re-calibrated in meters): `surfaceExposure` (tent weights, cover
// ramp ROOF_NEAR/ROOF_FULL, slope rise) and `surfaceWater` (wall streaks, WALL_WET, faces turned
// down stay dry, WET_SATURATION) from Clockwork, files/shaders/lib/light/surface.glsl,
// gitlab.com/OursCodeur/openmw branch clockwork-alpha, commit
// c5e5a9a6362c35512823884753d2d5284af204fa (GPL-3.0; THIRD_PARTY_NOTICES.md).
// Rain rings: the ring profile (`blip`, `circle`, trailing rings) of
// files/shaders/lib/water/rain_ripples.glsl as Clockwork uses it on puddles (same commit).
// Wet model: S. Lagarde, "Water drop 3b - Physically based wet surfaces" (2013); puddles and
// ripple layers by rain intensity: "Water drop 2b - Dynamic rain and its effects" (2013).
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
    /// x: animation seconds (wrapped at RIPPLE_TIME_WRAP_S; still while dry), y: rain at the
    /// camera (0..1; the puddles' rings, W4).
    time: vec4<f32>,
    /// x: 1 when the canopy map is valid; y: share of the rain a full leaf canopy lets through.
    canopy: vec4<f32>,
    /// Cover looks (W5a; the layer array at binding 50, slices 2c albedo+height, 2c+1 normal XY,
    /// roughness, AO): x 1 / tiling (1/m, 0 = no layer), y depth (m), z sparkle, w contrast.
    snow_layer: vec4<f32>,
    ash_layer: vec4<f32>,
    /// The cover's world-aligned UV at the render origin (W5b; `fract(origin.xz / tiling)`,
    /// computed in f64): xy snow, zw ash. UV = render XZ / tiling + this.
    cover_uv: vec4<f32>,
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

/// Puddles (W4; `sf_surface::layer` documents each): water level reach, pool span of the
/// height map, shoreline, level-face ramp (none from cos 6 deg, full from cos 3 deg), terrain
/// concavity, water roughness, pool noise tile, object height from albedo.
const PUDDLE_REACH: f32 = 0.3;
const PUDDLE_DEPTH: f32 = 0.1;
const PUDDLE_EDGE: f32 = 0.1;
const PUDDLE_FLAT_LOW: f32 = 0.994522;
const PUDDLE_FLAT_HIGH: f32 = 0.998630;
const BASIN_FULL_M: f32 = 0.1;
const BASIN_POOL: f32 = 0.35;
const PUDDLE_ROUGHNESS: f32 = 0.06;
const POOL_TILE_M: f32 = 64.0;
const HEIGHT_FROM_LUMA: f32 = 2.0;
/// Rain rings: drop cell, ring radius (cell share), ring breadth (cycle share), cycles a
/// second, tilt, footprint fade (m per pixel), clock wrap, and the four grids' cell scales.
const RIPPLE_CELL_M: f32 = 1.6;
const RIPPLE_RADIUS: f32 = 0.2;
const RIPPLE_RING: f32 = 0.16666667;
const RIPPLE_RATE: f32 = 1.2;
const RIPPLE_TILT: f32 = 0.5;
const RIPPLE_FADE_NEAR_M: f32 = 0.02;
const RIPPLE_FADE_FAR_M: f32 = 0.06;
const RIPPLE_TIME_WRAP_S: f32 = 1000.0;
const RIPPLE_SCALE_0: f32 = 1.0;
const RIPPLE_SCALE_1: f32 = 1.25;
const RIPPLE_SCALE_2: f32 = 0.8;
const RIPPLE_SCALE_3: f32 = 0.625;
/// Cover (W5; `sf_surface::layer` documents each): slope ramp on the shading normal's Y, the
/// height blend's soft edge, the cover texture's share of the compared height, drift swing and
/// tile (m), cover normal strength, ash F0 (snow takes ice's, WATER_REFLECTANCE).
const COVER_UP_LOW: f32 = 0.45;
const COVER_UP_HIGH: f32 = 0.8;
const COVER_EDGE: f32 = 0.08;
const COVER_HEIGHT_MIX: f32 = 0.5;
const COVER_DRIFT: f32 = 0.5;
const COVER_DRIFT_TILE_M: f32 = 64.0;
const COVER_NORMAL: f32 = 1.0;
const ASH_REFLECTANCE: f32 = 0.5;
/// Snow sparkle: glint cell (m), glinting share at sparkle 1, facet tilt, facet roughness, and
/// the pixel footprint (m) where glints fade and are gone.
const SPARKLE_CELL_M: f32 = 0.02;
const SPARKLE_DENSITY: f32 = 0.03;
const SPARKLE_TILT: f32 = 0.35;
const SPARKLE_ROUGHNESS: f32 = 0.1;
const SPARKLE_FADE_NEAR_M: f32 = 0.006;
const SPARKLE_FADE_FAR_M: f32 = 0.02;
/// Cover layer array slices (cover.rs): channel c's albedo + height at 2c, normal XY + roughness
/// + AO at 2c + 1.
const COVER_SNOW: u32 = 0u;
const COVER_ASH: u32 = 1u;

const LUMA: vec3<f32> = vec3(0.2126, 0.7152, 0.0722);

struct SurfaceInput {
    /// Render space (the uniform's window and shelter follow the floating origin).
    world_pos: vec3<f32>,
    /// 0..1 height signal (low fills first): terrain the blended layer height; objects their
    /// albedo luminance (`surface_height_of`: dark grout and cracks).
    height: f32,
    /// Terrain only: concavity from the height clipmap in meters (`h_avg(±2 m) - h`, >= 0;
    /// puddle sites, W4), else 0.
    basin: f32,
    /// Y of the true surface normal for the puddle gate (level faces only, W4): a mesh's
    /// triangle face normal (position derivatives), the terrain's height field; never a
    /// smoothed vertex normal, which reads a ramp as level.
    level_up: f32,
    porosity: f32,
    surface_class: u32,
    /// DYNAMIC: per-character wetness (W8), else 0.
    instance_wet: f32,
    /// Meters per pixel at the point (screen derivatives; rain rings fade out below a pixel),
    /// 0 when unknown.
    footprint: f32,
    /// Screen derivatives of `world_pos` (taken in uniform control flow by the caller) for the
    /// cover's texture gradients (W5); zero when unknown (cover samples the top mip).
    dpdx: vec3<f32>,
    dpdy: vec3<f32>,
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
/// Leaves (the canopy map, same texels and tent) cover a fraction of the tent and hold back
/// `1 - canopy.y` of the rain where they cover it all (Clockwork `roofLeaves` / `LEAF_HOLD`:
/// partial shelter under trees; ours reads leaf cover from its own map, not a checkerboard).
fn surface_exposure(shelter: texture_2d<f32>, canopy: texture_2d<f32>, shelter_sampler: sampler, u: SurfaceUniform, p: vec3<f32>, n_geo: vec3<f32>) -> f32 {
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
    let leaves_on = u.canopy.x > 0.0;
    var open = 0.0;
    var leaves = 0.0;
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
            if leaves_on {
                let leaf_over = (textureGather(0, canopy, shelter_sampler, uv + shift) - vec4(c.z)) * u.shelter.y;
                leaves += dot(smoothstep(near, full, leaf_over), w);
            }
        }
    }
    let leaf_hold = (1.0 - u.canopy.y) * (leaves / weight);
    return mix(open / weight * (1.0 - leaf_hold), 1.0, smoothstep(SHELTER_FADE_START, 1.0, edge));
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

/// How level a face is for standing water, `up` the true surface normal's Y (`level_up`).
fn surface_puddle_flat(up: f32) -> f32 {
    return smoothstep(PUDDLE_FLAT_LOW, PUDDLE_FLAT_HIGH, up);
}

/// The normal's Y of a height field with gradient `g` (rise per meter along X and Z).
fn surface_up_of_gradient(g: vec2<f32>) -> f32 {
    return inverseSqrt(1.0 + dot(g, g));
}

/// The pool site score: pool noise, raised by terrain concavity (`basin` meters).
fn surface_pool_of(noise: f32, basin: f32) -> f32 {
    return saturate(noise + BASIN_POOL * saturate(basin / BASIN_FULL_M));
}

/// An object's height signal from its albedo: dark grout and cracks fill first (Clockwork).
fn surface_height_of(albedo: vec3<f32>) -> f32 {
    return saturate(HEIGHT_FROM_LUMA * dot(albedo, LUMA));
}

/// Standing water (0..1) from the puddle state `p` (sheltered), the pool score and the height:
/// Lagarde's water level against the height map (Clockwork's `surfaceWater` height branch),
/// soft over `PUDDLE_EDGE` of depth.
fn surface_puddle(p: f32, pool: f32, height: f32) -> f32 {
    return smoothstep(0.0, PUDDLE_EDGE, (pool - 1.0 + PUDDLE_REACH * p) / PUDDLE_DEPTH - height);
}

/// Under standing water the surface is water: mirror gloss, F0 0.02, a dielectric top (Lagarde).
fn surface_puddle_material(pbr: ptr<function, PbrInput>, puddle: f32) {
    (*pbr).material.perceptual_roughness = mix((*pbr).material.perceptual_roughness, PUDDLE_ROUGHNESS, puddle);
    (*pbr).material.reflectance = mix((*pbr).material.reflectance, vec3(WATER_REFLECTANCE), puddle);
    (*pbr).material.metallic = mix((*pbr).material.metallic, 0.0, puddle);
}

/// Rings fade as a pixel covers more than a ring's breadth (no shimmer under DLSS or TAA).
fn surface_ripple_fade(footprint: f32) -> f32 {
    return 1.0 - smoothstep(RIPPLE_FADE_NEAR_M, RIPPLE_FADE_FAR_M, footprint);
}

/// Two hash values in 0..1 for an integer cell and a seed.
fn surface_hash(c: vec2<i32>, seed: u32) -> vec2<f32> {
    var h = (u32(c.x) * 0x8da6b343u) ^ (u32(c.y) * 0xd8163841u) ^ (seed * 0x9e3779b9u);
    h = (h ^ (h >> 15u)) * 0x2c1b3c6du;
    h = (h ^ (h >> 12u)) * 0x297a2d39u;
    h = h ^ (h >> 15u);
    return vec2(f32(h & 0xffffu), f32(h >> 16u)) / 65535.0;
}

/// `rain_ripples.glsl` `blip` and `blipDerivative`.
fn surface_blip(x: f32) -> f32 {
    let y = max(0.0, 1.0 - x * x);
    return y * y * y;
}

fn surface_blip_derivative(x: f32) -> f32 {
    let y = clamp(x, -1.0, 1.0);
    let n = y * y - 1.0;
    return -6.0 * y * n * n;
}

/// One drop's ring slope (`rain_ripples.glsl` `circle`): `to` the point minus the drop (cell
/// units), `phase` the drop's cycle; zero outside the ring's band. Twin: `sf_surface::layer::ring`.
fn surface_ring(to: vec2<f32>, phase: f32) -> vec2<f32> {
    let d = length(to);
    let follower = (phase - d / RIPPLE_RADIUS) / RIPPLE_RING - 1.0;
    if follower < -1.0 || follower > 1.0 {
        return vec2(0.0);
    }
    let energy = 1.0 - phase;
    let range = surface_blip(min(follower, 0.0));
    let v = vec3(to * (-surface_blip_derivative(follower) * 5.0 * energy * energy), 0.5);
    return v.xy * (energy * range / length(v));
}

/// One drop grid at `q` (cell units): each cell drops once a cycle at a random spot (its own
/// start phase), with two trailing rings (`rain`: a - b/2 + c/4).
fn surface_drops(q: vec2<f32>, seed: u32, time: f32) -> vec2<f32> {
    let cell = vec2<i32>(floor(q));
    let f = q - floor(q);
    let start = surface_hash(cell, seed ^ 0x51ed27u).x;
    var slope = vec2(0.0);
    var weight = 1.0;
    for (var k = 0; k < 3; k++) {
        let t = time * RIPPLE_RATE + start - f32(k) * RIPPLE_RING;
        let cycle = floor(t);
        let spot = surface_hash(cell, seed ^ (u32(i32(cycle)) * 0x632be5abu));
        let drop = vec2(0.5) + (0.5 - RIPPLE_RADIUS) * (2.0 * spot - 1.0);
        slope += weight * surface_ring(f - drop, t - cycle);
        weight *= -0.5;
    }
    return slope;
}

/// Rain rings at render XZ `xz` (m): up to four drop grids, grid k on over rain k/4..(k+1)/4
/// (Lagarde's ripple layers: drizzle rings sparsely, a storm densely). Returns the water's
/// slope along X and Z.
fn surface_ripples(xz: vec2<f32>, time: f32, rain: f32) -> vec2<f32> {
    var scales = array<f32, 4>(RIPPLE_SCALE_0, RIPPLE_SCALE_1, RIPPLE_SCALE_2, RIPPLE_SCALE_3);
    var slope = vec2(0.0);
    for (var k = 0u; k < 4u; k++) {
        let w = saturate((rain - f32(k) * 0.25) * 4.0);
        if w <= 0.0 {
            break;
        }
        let q = xz / (RIPPLE_CELL_M * scales[k]) + vec2(0.37, 0.71) * f32(k);
        slope += w * surface_drops(q, k + 1u, time);
    }
    return slope;
}

/// Cover (W5) twins: `sf_surface::layer::{cover_slope, cover_amount, cover_height, cover_blend,
/// sparkle_fade}`.
fn surface_cover_slope(up: f32) -> f32 {
    return smoothstep(COVER_UP_LOW, COVER_UP_HIGH, up);
}

fn surface_cover_amount(cell: f32, up: f32, exposure: f32) -> f32 {
    return cell * surface_cover_slope(up) * exposure;
}

fn surface_cover_height(base: f32, cover: f32, drift: f32) -> f32 {
    return saturate(mix(base, 1.0 - cover, COVER_HEIGHT_MIX) + (drift - 0.5) * COVER_DRIFT);
}

fn surface_cover_blend(amount: f32, h: f32, contrast: f32) -> f32 {
    let e = COVER_EDGE / max(contrast, 0.01);
    let level = amount * (1.0 + 2.0 * e) - e;
    return smoothstep(-e, e, level - h);
}

fn surface_sparkle_fade(footprint: f32) -> f32 {
    if footprint <= 0.0 {
        return 0.0;
    }
    return 1.0 - smoothstep(SPARKLE_FADE_NEAR_M, SPARKLE_FADE_FAR_M, footprint);
}

/// sRGB bytes (the array's albedo slices) to linear.
fn surface_srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    return select(pow((c + 0.055) / 1.055, vec3(2.4)), c / 12.92, c <= vec3(0.04045));
}

/// One cover look at a point: linear albedo, its height (0..1, thick = high), the normal map's
/// volume height gradient (world, the top-down projection), roughness and AO.
struct SurfaceCover {
    albedo: vec3<f32>,
    height: f32,
    gradient: vec3<f32>,
    roughness: f32,
    ao: f32,
}

/// Samples channel `c`'s look top-down (world-aligned: UV = render XZ / tiling + the origin's
/// fraction `uv0`) with the caller's derivatives: trilinear and anisotropic, in any control flow.
fn surface_cover_sample(
    cover: texture_2d_array<f32>,
    cover_sampler: sampler,
    layer: vec4<f32>,
    uv0: vec2<f32>,
    c: u32,
    s: SurfaceInput,
) -> SurfaceCover {
    let uv = s.world_pos.xz * layer.x + uv0;
    let gx = s.dpdx.xz * layer.x;
    let gy = s.dpdy.xz * layer.x;
    let a = textureSampleGrad(cover, cover_sampler, uv, i32(2u * c), gx, gy);
    let n = textureSampleGrad(cover, cover_sampler, uv, i32(2u * c + 1u), gx, gy);
    var out: SurfaceCover;
    out.albedo = surface_srgb_to_linear(a.rgb);
    out.height = a.a;
    // Tangent space (+X along +U = world +X, +Y toward the image's top = world -Z): the height
    // gradient is -t / nz per axis (Mikkelsen 2020, volume gradient of a planar projection).
    let t = n.xy * 2.0 - 1.0;
    let nz = max(sqrt(saturate(1.0 - dot(t, t))), 0.1);
    out.gradient = vec3(-t.x / nz, 0.0, t.y / nz) * COVER_NORMAL;
    out.roughness = n.z;
    out.ao = n.w;
    return out;
}

/// A snow glint facet at `xz` (W5 sparkle): x, z the facet's height gradient, y its weight (0 when
/// this cell doesn't glint or glints are below a pixel). Cells are world-fixed, so a glint holds
/// still and flashes only as the view or the sun moves.
fn surface_sparkle(xz: vec2<f32>, sparkle: f32, footprint: f32) -> vec3<f32> {
    let fade = surface_sparkle_fade(footprint);
    if sparkle <= 0.0 || fade <= 0.0 {
        return vec3(0.0);
    }
    let cell = vec2<i32>(floor(xz / SPARKLE_CELL_M));
    if surface_hash(cell, 11u).x >= sparkle * SPARKLE_DENSITY {
        return vec3(0.0);
    }
    let tilt = (surface_hash(cell, 12u) * 2.0 - 1.0) * SPARKLE_TILT;
    return vec3(tilt.x, fade, tilt.y);
}

/// Lays channel `c` over the material where its height blend covers (`amount` the cell's ×
/// slope × exposure, `drift` the drift noise): its albedo, roughness (snow glints), F0 (ice for
/// snow, a plain dielectric for ash), AO, and its normal replacing the surface's in the gradient
/// sum. Returns the blend (0..1).
fn surface_apply_cover(
    pbr: ptr<function, PbrInput>,
    gradient: ptr<function, vec3<f32>>,
    s: SurfaceInput,
    layer: vec4<f32>,
    uv0: vec2<f32>,
    c: u32,
    amount: f32,
    drift: f32,
    n_geo: vec3<f32>,
    cover: texture_2d_array<f32>,
    cover_sampler: sampler,
) -> f32 {
    let look = surface_cover_sample(cover, cover_sampler, layer, uv0, c, s);
    let b = surface_cover_blend(amount, surface_cover_height(s.height, look.height, drift), layer.w);
    if b <= 0.0 {
        return 0.0;
    }
    var rough = look.roughness;
    var g = look.gradient;
    let glint = surface_sparkle(s.world_pos.xz, layer.z, s.footprint);
    if glint.y > 0.0 {
        rough = mix(rough, SPARKLE_ROUGHNESS, glint.y);
        g += vec3(glint.x, 0.0, glint.z) * glint.y;
    }
    let f0 = select(ASH_REFLECTANCE, WATER_REFLECTANCE, c == COVER_SNOW);
    (*pbr).material.base_color = vec4(mix((*pbr).material.base_color.rgb, look.albedo, b), (*pbr).material.base_color.a);
    (*pbr).material.perceptual_roughness = mix((*pbr).material.perceptual_roughness, rough, b);
    (*pbr).material.metallic = mix((*pbr).material.metallic, 0.0, b);
    (*pbr).material.reflectance = mix((*pbr).material.reflectance, vec3(f0), b);
    (*pbr).diffuse_occlusion *= mix(1.0, look.ao, b);
    *gradient = mix(*gradient, g - n_geo * dot(n_geo, g), b);
    return b;
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
    canopy: texture_2d<f32>,
    cover: texture_2d_array<f32>,
    cover_sampler: sampler,
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
            let e = surface_exposure(shelter, canopy, shelter_sampler, u, s.world_pos, (*pbr).world_normal);
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
    let want_wet = (flags & FLAG_WET) != 0u;
    // Standing water on statics and terrain only (characters move; far proxies are sub-pixel).
    let want_puddle = (flags & FLAG_PUDDLE) != 0u
        && (s.surface_class == CLASS_STATIC || s.surface_class == CLASS_TERRAIN);
    // Cover (W5) on statics and terrain with a look: never on characters (DYNAMIC) or far
    // proxies (FAR: wetness only).
    let solid = s.surface_class == CLASS_STATIC || s.surface_class == CLASS_TERRAIN;
    let want_snow = solid && (flags & FLAG_SNOW) != 0u && u.snow_layer.x > 0.0;
    let want_ash = solid && (flags & FLAG_ASH) != 0u && u.ash_layer.x > 0.0;
    if !want_wet && !want_puddle && !want_snow && !want_ash {
        return;
    }
    let n_geo = (*pbr).world_normal;
    var wet = 0.0;
    var puddle = 0.0;
    var rain = 0.0;
    var snow = 0.0;
    var ash = 0.0;
    if s.surface_class == CLASS_DYNAMIC {
        wet = s.instance_wet;
    } else {
        let cell = surface_cells_at(cells, samp, u, s.world_pos);
        if want_wet && cell.r > 0.0 {
            wet = cell.r * surface_facing_wet(noise, samp, s.world_pos, n_geo);
            // Drying leaves patches: low noise dries last.
            let n = textureSampleLevel(noise, samp, s.world_pos.xz / PATCH_TILE_M, 0.0).r;
            wet = smoothstep(n * 0.6, n * 0.6 + 0.4, wet);
        }
        // Puddle state where the surface is level (its true normal: water runs off a slope).
        let flat = surface_puddle_flat(s.level_up);
        var p = 0.0;
        if want_puddle && flat > 0.0 {
            p = cell.g;
        }
        // Cover lies on faces that look up (the shading normal: the tops of bumps first).
        let up = (*pbr).N.y;
        if want_snow {
            snow = surface_cover_amount(cell.b, up, 1.0);
        }
        if want_ash {
            ash = surface_cover_amount(cell.a, up, 1.0);
        }
        if s.surface_class != CLASS_FAR && (wet > 0.0 || p > 0.0 || snow > 0.0 || ash > 0.0) {
            // Shelter: none under roofs, less (smaller puddles, fewer rings) under leaves.
            let e = surface_exposure(shelter, canopy, shelter_sampler, u, s.world_pos, n_geo);
            wet *= e;
            p *= e;
            rain = u.time.y * e;
            snow *= e;
            ash *= e;
        }
        if p > 0.0 {
            let n = textureSampleLevel(noise, samp, s.world_pos.xz / POOL_TILE_M + vec2(0.31, 0.57), 0.0).g;
            puddle = flat * surface_puddle(p, surface_pool_of(n, s.basin), s.height);
        }
    }
    if wet <= 0.0 && puddle <= 0.0 && snow <= 0.0 && ash <= 0.0 {
        return;
    }
    // Normals compose as surface gradients, resolved once at the end.
    var gradient = surface_gradient_of(n_geo, (*pbr).N);
    // Cover first (spec order: cover, wetness, puddles): snow, then ash over it. Covered ground
    // drinks like snow or ash does.
    var porosity = s.porosity;
    if snow > 0.0 || ash > 0.0 {
        let drift = textureSampleLevel(noise, samp, s.world_pos.xz / COVER_DRIFT_TILE_M + vec2(0.17, 0.83), 0.0).g;
        if snow > 0.0 {
            let b = surface_apply_cover(pbr, &gradient, s, u.snow_layer, u.cover_uv.xy, COVER_SNOW, snow, drift, n_geo, cover, cover_sampler);
            porosity = mix(porosity, 1.0, b);
        }
        if ash > 0.0 {
            let b = surface_apply_cover(pbr, &gradient, s, u.ash_layer, u.cover_uv.zw, COVER_ASH, ash, drift, n_geo, cover, cover_sampler);
            porosity = mix(porosity, 1.0, b);
        }
    }
    if wet <= 0.0 && puddle <= 0.0 {
        (*pbr).N = surface_resolve(n_geo, gradient);
        return;
    }
    // Standing water soaks what it covers (Lagarde: wet = saturate(wet + accumulated water)).
    let film = surface_wet_material(pbr, max(wet, puddle), porosity);
    if s.surface_class == CLASS_FAR {
        return;
    }
    // The film fills the micro-detail and standing water lies level: the normal map's gradient
    // fades toward the macro (geometric) normal.
    gradient *= (1.0 - film) * (1.0 - puddle);
    if puddle > 0.0 {
        surface_puddle_material(pbr, puddle);
        // Rain rings on the water, while it rains on it, until they shrink below a pixel.
        let rings = puddle * surface_ripple_fade(s.footprint);
        if rain > 0.0 && rings > 0.0 {
            let slope = surface_ripples(s.world_pos.xz, u.time.x, rain) * (RIPPLE_TILT * rings);
            let g = -vec3(slope.x, 0.0, slope.y);
            gradient += g - n_geo * dot(n_geo, g);
        }
    }
    (*pbr).N = surface_resolve(n_geo, gradient);
}
