// Terrain main pass: CDLOD vertex (common.wesl) and a PBR fragment that
// builds Bevy's PbrInput from height-blended splat layers (surface.wesl), so shadows,
// clustered lights, fog and atmosphere match every other object.

import super::common;
import super::common::{DEBUG_LOD, DEBUG_WIREFRAME, DEBUG_NO_SPECULAR, DEBUG_NO_INDIRECT, DEBUG_NO_ENV_SPECULAR, world_xz, origin_y, debug_flags, base_reflectance, terrain_normal, height_at};
import super::surface::terrain_surface;
import super::far_shadow::far_shadow_term;
// Surface weather (surface-weather.md "Shader integration"): the shared layer on the view's
// bindings (vendored bevy_pbr patch "SW"; the def comes with the view, main pass only).
@if(SURFACE_WEATHER)
import sf_surface_bevy::layer::{
    SurfaceInput, CLASS_TERRAIN, surface_porosity, surface_puddle_flat, surface_up_of_gradient,
};
@if(SURFACE_WEATHER)
import sf_surface_bevy::view::{apply_surface_weather, surface_puddles_on};
import bevy_pbr::render::{
    pbr_types::{PbrInput, pbr_input_new, STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, calculate_view},
    mesh_bindings::mesh,
    mesh_view_bindings::view,
    view_transformations::position_world_to_clip,
    pbr_lighting::perceptualRoughnessToRoughness,
};
@if(SCREEN_SPACE_AMBIENT_OCCLUSION)
import bevy_pbr::render::mesh_view_bindings::screen_space_ambient_occlusion_texture;
@if(SCREEN_SPACE_AMBIENT_OCCLUSION)
import bevy_pbr::ssao::utils::ssao_multibounce;

/// Specular occlusion from an ambient visibility term, as StandardMaterial
/// does it (bevy_pbr pbr_fragment.wesl): Lagarde and de Rousiers 2014,
/// "Moving Frostbite to Physically Based Rendering".
fn specular_occlusion(n_dot_v: f32, visibility: f32, perceptual_roughness: f32) -> f32 {
    let roughness = perceptualRoughnessToRoughness(perceptual_roughness);
    return saturate(pow(n_dot_v + visibility, exp2(-16.0 * roughness - 1.0)) - 1.0 + visibility);
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) grid: vec2<f32>,
    @location(2) lod: f32,
    @location(3) @interpolate(flat) instance_index: u32,
}

@vertex
fn vertex(v: Vertex) -> TerrainVertexOutput {
    let t = common::terrain_vertex(v.position.xz, v.instance_index, true);
    var out: TerrainVertexOutput;
    out.world_position = vec4<f32>(t.position, 1.0);
    out.position = position_world_to_clip(t.position);
    out.grid = t.grid;
    out.lod = t.lod;
    out.instance_index = v.instance_index;
    return out;
}

fn lod_color(lod: f32) -> vec3<f32> {
    var palette = array<vec3<f32>, 12>(
        vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(1.0, 0.4, 0.8), vec3<f32>(1.0, 0.2, 0.2), vec3<f32>(1.0, 0.6, 0.1),
        vec3<f32>(1.0, 1.0, 0.2), vec3<f32>(0.4, 1.0, 0.2), vec3<f32>(0.1, 0.9, 0.7), vec3<f32>(0.2, 0.6, 1.0),
        vec3<f32>(0.4, 0.3, 1.0), vec3<f32>(0.7, 0.3, 0.9), vec3<f32>(0.5, 0.5, 0.5), vec3<f32>(0.2, 0.2, 0.2),
    );
    let i = clamp(i32(floor(lod)) + 2, 0, 10);
    return mix(palette[i], palette[i + 1], fract(lod));
}

/// The ground at render XZ `xz` for surface weather's puddles (surface-weather.md
/// W4; CPU twin `sf_surface::layer::ground`), from the height clipmap (R32F, the
/// finest resident level; not the detail displacement), heights 2 m out along
/// ±X and ±Z: xy the slope (central differences over 4 m, rise per meter; the
/// puddle gate), z the concavity (their mean minus the height here, meters,
/// >= 0: a hollow; ridges and even slopes give 0).
fn terrain_ground(xz: vec2<f32>) -> vec3<f32> {
    let r = 2.0;
    let px = height_at(xz + vec2<f32>(r, 0.0), 0u);
    let nx = height_at(xz - vec2<f32>(r, 0.0), 0u);
    let pz = height_at(xz + vec2<f32>(0.0, r), 0u);
    let nz = height_at(xz - vec2<f32>(0.0, r), 0u);
    let basin = max(0.25 * (px + nx + pz + nz) - height_at(xz, 0u), 0.0);
    return vec3<f32>(vec2<f32>(px - nx, pz - nz) / (2.0 * r), basin);
}

/// Distance to the nearest triangle edge in pixels (quads split along the
/// canonical (i, j)-(i+1, j+1) diagonal).
fn edge_pixels(grid: vec2<f32>) -> f32 {
    let f = fract(grid);
    let w = max(fwidth(grid), vec2<f32>(1e-5));
    let e = min(min(f.x, 1.0 - f.x) / w.x, min(f.y, 1.0 - f.y) / w.y);
    let diag = abs(fract(grid.x - grid.y + 0.5) - 0.5) / (w.x + w.y);
    return min(e, diag);
}

@fragment
fn fragment(in: TerrainVertexOutput) -> @location(0) vec4<f32> {
    let xz = in.world_position.xz;
    let wxz = world_xz(xz);
    let wpos = vec3<f32>(wxz.x, in.world_position.y + origin_y(), wxz.y);
    // Derivatives in uniform control flow, for every texture fetch below.
    let dpx = dpdx(wpos);
    let dpy = dpdy(wpos);
    let edges = edge_pixels(in.grid);
    let debug = debug_flags();

    let n_geo = terrain_normal(xz, in.lod);
    let dist = distance(view.world_position, in.world_position.xyz);
    let s = terrain_surface(xz, wpos, dpx, dpy, n_geo, dist);
    var albedo = s.albedo;

    if (debug & DEBUG_LOD) != 0u {
        albedo = lod_color(in.lod) * 0.6;
    }

    var pbr: PbrInput = pbr_input_new();
    pbr.material.base_color = vec4<f32>(albedo, 1.0);
    pbr.material.perceptual_roughness = clamp(s.roughness, 0.045, 1.0);
    pbr.material.metallic = 0.0;
    // The ground's reflectance (TerrainSettings::reflectance), before surface weather mixes
    // wet film and puddles toward water's, so rain sheen builds on it.
    pbr.material.reflectance = vec3<f32>(base_reflectance());
    pbr.material.flags = STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT;
    pbr.frag_coord = in.position;
    pbr.world_position = in.world_position;
    pbr.world_normal = n_geo;
    pbr.N = s.n;
    pbr.is_orthographic = view.clip_from_view[3].w == 1.0;
    pbr.V = calculate_view(in.world_position, pbr.is_orthographic);
    pbr.flags = mesh[in.instance_index].flags;
@if(SURFACE_WEATHER) {
    var si: SurfaceInput;
    si.world_pos = in.world_position.xyz;
    si.height = s.height;
    // Puddles (W4) only on level ground: the steeper of the clipmap normal here and the
    // height field over 4 m (the filtered 8-bit normal of a coarse LOD reads gentle slopes as
    // level). The concavity raises the pool score in hollows. Only while puddles are on.
    si.level_up = n_geo.y;
    si.basin = 0.0;
    if surface_puddles_on() && surface_puddle_flat(n_geo.y) > 0.0 {
        let ground = terrain_ground(xz);
        si.level_up = min(n_geo.y, surface_up_of_gradient(ground.xy));
        si.basin = ground.z;
    }
    si.porosity = surface_porosity(0.0, pbr.material.perceptual_roughness);
    si.surface_class = CLASS_TERRAIN;
    si.instance_wet = 0.0;
    si.footprint = max(length(dpx), length(dpy));
    si.dpdx = dpx;
    si.dpdy = dpy;
    apply_surface_weather(&pbr, si);
}
    // Ambient occlusion as StandardMaterial applies it: the layer AO, SSAO
    // (multibounce) when the view has it, and specular occlusion from the
    // combined visibility so the env map's grazing Fresnel is occluded too.
    // After the surface layer, so wetness roughness feeds the specular term.
    let n_dot_v = max(dot(pbr.N, pbr.V), 1e-4);
    var visibility = s.ao;
    var diffuse_occlusion = vec3<f32>(s.ao);
@if(SCREEN_SPACE_AMBIENT_OCCLUSION) {
    let ssao = textureLoad(screen_space_ambient_occlusion_texture, vec2<i32>(in.position.xy - view.main_pass_viewport.xy), 0i).r;
    diffuse_occlusion = min(diffuse_occlusion, ssao_multibounce(ssao, pbr.material.base_color.rgb));
    visibility = min(visibility, ssao);
}
    pbr.diffuse_occlusion = diffuse_occlusion;
    pbr.specular_occlusion = specular_occlusion(n_dot_v, visibility, pbr.material.perceptual_roughness);
    // RO-16 veil diagnosis (scaffold: remove with the veil fix): the terrain without specular
    // (every lobe: sun, lamps, env map, flat ambient) or without indirect light (env map, flat
    // ambient, GI), live from the console (`flip terrain_specular`, `flip terrain_indirect`).
    if (debug & DEBUG_NO_SPECULAR) != 0u {
        pbr.material.reflectance = vec3<f32>(0.0);
    }
    // Env-map specular alone (Bevy multiplies only that lobe by `specular_occlusion`; the flat
    // ambient's specular rides on `diffuse_occlusion`): `flip terrain_env_specular`.
    if (debug & DEBUG_NO_ENV_SPECULAR) != 0u {
        pbr.specular_occlusion = 0.0;
    }
    if (debug & DEBUG_NO_INDIRECT) != 0u {
        pbr.diffuse_occlusion = vec3<f32>(0.0);
        pbr.specular_occlusion = 0.0;
    }
    // Terrain shadows past the cascades (Bevy multiplies its CSM term by the
    // factor) and the dithered cascade end (the lighting position).
    let far = far_shadow_term(in.world_position, pbr.N, in.position.xy);
    pbr.directional_shadow_factor = far.factor;
    pbr.world_position = far.position;
    var color = apply_pbr_lighting(pbr);
    pbr.world_position = in.world_position;
    if (debug & DEBUG_WIREFRAME) != 0u {
        let line = 1.0 - smoothstep(0.5, 1.5, edges);
        color = vec4<f32>(mix(color.rgb, vec3<f32>(0.0), line * 0.85), color.a);
    }
    return main_pass_post_lighting_processing(pbr, color);
}
