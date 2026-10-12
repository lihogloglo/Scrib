// Object-terrain blending (terrain.md "Object-terrain blending"): Bevy's
// StandardMaterial fragment (forward; the material forces the forward path)
// for placed objects marked to blend into the ground (rocks, cliffs, roots,
// foundations), plus the terrain's own surface shading near their base.
//
// Technique: the UE runtime-virtual-texture mesh blend and CryEngine's
// "terrain layer blending into objects": the object's shader reads the
// terrain's height and surface and lerps albedo, roughness, AO and the
// shading normal toward the ground's by height above the terrain, over a
// per-model band. Our clipmaps play the virtual texture's role: the same
// height, splat layers and macro albedo the terrain reads (bindings 100..112,
// common.wesl), so the base matches the ground it stands on exactly. The
// layers' height maps break up the edge (the height blend's rule: grains and
// pebbles creep higher than flat soil).
//
// The prepass and shadow passes stay StandardMaterial's: depth, motion
// vectors and shadows are unchanged (no pixel depth offset).

import super::common::{DEBUG_NO_GROUND_BLEND, world_xz, origin_y, debug_flags, height_bicubic};
import super::surface::terrain_surface;
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    mesh_view_bindings::view,
};
@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

// x: band height (m above the terrain where the blend ends; 0 = off),
// y: edge break-up by the layers' height maps (0..1), zw: unused.
@group(constants::MATERIAL_BIND_GROUP) @binding(113) var<uniform> ground_blend: vec4<f32>;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    // Derivatives in uniform control flow, for the layer fetches below.
    let xz = in.world_position.xz;
    let wxz = world_xz(xz);
    let wpos = vec3<f32>(wxz.x, in.world_position.y + origin_y(), wxz.y);
    let dpx = dpdx(wpos);
    let dpy = dpdy(wpos);

    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);

    let band = ground_blend.x;
    if band > 0.0 && (debug_flags() & DEBUG_NO_GROUND_BLEND) == 0u {
        // Outside the band this lookup is the whole cost.
        let above = wpos.y - height_bicubic(xz);
        if above < band {
            let dist = distance(view.world_position, in.world_position.xyz);
            // The ground's layers, tint and variation at this XZ, oriented by
            // the object's own surface: top-down on flat parts (continuing
            // the ground texture), triplanar on walls and rock faces (the
            // texture side-on, not streaked from above as a pure RVT blend
            // would), so the lighting keeps the object's shape.
            let g = terrain_surface(xz, wpos, dpx, dpy, normalize(pbr.world_normal), dist, true);
            let t = clamp(1.0 - above / band, 0.0, 1.0);
            let e = t + ground_blend.y * (g.height - 0.5);
            let w = smoothstep(0.2, 0.8, e);
            pbr.material.base_color = vec4<f32>(mix(pbr.material.base_color.rgb, g.albedo, w), pbr.material.base_color.a);
            pbr.material.perceptual_roughness = mix(pbr.material.perceptual_roughness, clamp(g.roughness, 0.045, 1.0), w);
            pbr.material.metallic = mix(pbr.material.metallic, 0.0, w);
            pbr.material.emissive = mix(pbr.material.emissive, vec4<f32>(0.0, 0.0, 0.0, pbr.material.emissive.a), w);
            pbr.diffuse_occlusion = pbr.diffuse_occlusion * mix(1.0, g.ao, w);
            pbr.N = normalize(mix(pbr.N, g.n, w));
        }
    }

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
