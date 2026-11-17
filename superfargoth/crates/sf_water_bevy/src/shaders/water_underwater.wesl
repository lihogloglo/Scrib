// sf_water_bevy: the underwater post (water.md "Underwater", slice U3; research
// `underwater-2026-11-17.md` "Underwater medium"). Drawn only on a view whose camera is fully under
// the water (`WaterMediumUniform.state` 2, `medium.rs` `post_on`), after the cloud/fog composite
// (which skips such a view, as Bevy's `render_sky` does: `bevy_pbr::SkyOccluded`) and before the
// transparent pass (render-order.md step 8b). Per pixel, over the distance from the lens to the
// depth buffer's hit: `scene * T + albedo * (ambient + sun) * exp(-Kd * z) * (1 - T)`,
// `T = exp(-sigma_t * d)` per channel, `z` the camera's depth below the surface. A pixel that hit
// nothing is infinitely far: T = 0, the medium's colour. The CPU reference and its exact tests are
// `sf_water::underwater` (`underwater_colour`).

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_render::view::View;

@if(DUAL_SOURCE_BLENDING)
enable dual_source_blending;

// `medium.rs` `UnderwaterUniform` (field order is the layout).
struct Underwater {
    // Per metre, linear RGB (the body's optics, as its surface draw reads them).
    absorption: vec4<f32>,
    scattering: vec4<f32>,
    // Radiance-like light at the surface (before exposure): the directional lights'
    // `sf_water::sun_term`, and the ambient plus the sky fill.
    sun: vec4<f32>,
    ambient: vec4<f32>,
    // (camera depth below the surface m, >= 0; Kd scale; 0; 0).
    params: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<uniform> medium: Underwater;
@if(MULTISAMPLED) @group(0) @binding(2) var scene_depth: texture_depth_multisampled_2d;
@if(!MULTISAMPLED) @group(0) @binding(2) var scene_depth: texture_depth_2d;

// sf_water::underwater::MIN_EXTINCTION (a division guard; the water draw floors sigma_t the same).
const MIN_EXTINCTION: f32 = 1e-4;

struct UnderwaterOut {
    // Dual-source: colour = inscatter + scene * transmittance per channel; else the mean
    // transmittance in alpha (Bevy's `render_sky` fallback).
    @if(DUAL_SOURCE_BLENDING) @location(0) @blend_src(0) inscatter: vec4<f32>,
    @if(DUAL_SOURCE_BLENDING) @location(0) @blend_src(1) transmittance: vec4<f32>,
    @if(!DUAL_SOURCE_BLENDING) @location(0) inscatter: vec4<f32>,
}

// Metres from the lens to the scene point at viewport `uv` with reverse-Z `depth` (> 0).
fn view_distance(uv: vec2<f32>, depth: f32) -> f32 {
    let ndc = vec3(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth);
    let w = view.world_from_clip * vec4(ndc, 1.0);
    return distance(w.xyz / w.w, view.world_position);
}

@fragment
fn underwater(in: FullscreenVertexOutput) -> UnderwaterOut {
    // Sample 0 under MSAA, as the snapshot: one path per pixel.
    let depth = textureLoad(scene_depth, vec2<i32>(in.position.xy), 0);
    let sigma_t = max(medium.absorption.rgb + medium.scattering.rgb, vec3(MIN_EXTINCTION));
    let albedo = max(medium.scattering.rgb, vec3(0.0)) / sigma_t;
    // Reverse Z: 0 is the far plane at infinity, nothing was hit (full extinction).
    var transmittance = vec3(0.0);
    if depth > 0.0 {
        transmittance = exp(-sigma_t * view_distance(in.uv, depth));
    }
    let z = max(medium.params.x, 0.0);
    let falloff = exp(-sigma_t * max(medium.params.y, 0.0) * z);
    // The scene colour is pre-exposed; the in-scatter is lit like it.
    let inscatter = albedo * (medium.ambient.rgb + medium.sun.rgb) * falloff * (1.0 - transmittance) * view.exposure;

    var out: UnderwaterOut;
    @if(DUAL_SOURCE_BLENDING) {
        out.inscatter = vec4(inscatter, 0.0);
        out.transmittance = vec4(transmittance, 1.0);
    }
    @if(!DUAL_SOURCE_BLENDING) {
        out.inscatter = vec4(inscatter, (transmittance.r + transmittance.g + transmittance.b) / 3.0);
    }
    return out;
}
