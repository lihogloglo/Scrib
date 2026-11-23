// sf_groundcover_bevy: the groundcover main pass (groundcover.md "Renderers", GC-2; `GroundcoverMaterial`,
// `MaterialExtension::{vertex_shader, fragment_shader}`).
//
// Vertex: Bevy's `bevy_pbr` `render/mesh.wesl` `vertex` (Bevy 0.20.0 as vendored in
// `superfargoth/vendor/bevy_pbr/src/render/mesh.wesl` l.33-98) without skinning and morph targets
// (groundcover models are static cards), plus the dithered fade (a scale about the instance root) and the
// wind bend on the world position. RE-SYNC on every Bevy update.
//
// Fragment: Bevy's `render/pbr.wesl` forward path (l.36-104) without decals, surface weather and OIT, plus
// the species tint (the tag's position between the species' two tints) on the base colour before the alpha
// test. Forward only: the plugin forces `OpaqueRendererMethod::Forward` on the base.

import bevy_pbr::render::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput, FragmentOutput, decompress_vertex},
    mesh_view_bindings::{view, globals},
    view_transformations::position_world_to_clip,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
};
import super::groundcover_common::{unpack_tag, species_of, fade_scale, instance_scale, bent};

@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

@vertex
fn vertex(vertex_in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let vertex = decompress_vertex(vertex_in, vertex_in.instance_index);
    let world_from_local = mesh_functions::get_world_from_local(vertex_in.instance_index);
    let tag = unpack_tag(mesh_functions::get_tag(vertex_in.instance_index));
    let root = world_from_local[3].xyz;
    let s = fade_scale(root, view.world_position, tag.fade);
    let unbent = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position * s, 1.0));
    out.world_position = vec4<f32>(bent(unbent.xyz, root, instance_scale(world_from_local), tag, globals.time), 1.0);
    out.position = position_world_to_clip(out.world_position.xyz);

@if(VERTEX_NORMALS)
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_in.instance_index);

@if(VERTEX_UVS_A)
    out.uv = vertex.uv;
@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;

@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex_in.instance_index
    );

@if(VERTEX_COLORS)
    out.color = vertex.color;

@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = vertex_in.instance_index;

@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_in.instance_index, world_from_local[3]);

    return out;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(in.position, in.visibility_range_dither);

    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Species tint (the vertex stage passes the instance index: VERTEX_OUTPUT_INSTANCE_INDEX is always on in
    // Bevy's mesh pipeline, `render/mesh.rs`).
    let tag = unpack_tag(mesh_functions::get_tag(in.instance_index));
    let sp = species_of(tag);
    let tint = mix(sp.tint_a.rgb, sp.tint_b.rgb, tag.tint);
    pbr_input.material.base_color = vec4<f32>(pbr_input.material.base_color.rgb * tint, pbr_input.material.base_color.a);

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color
    );

    var out: FragmentOutput;
    if (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr_input);
    } else {
        out.color = pbr_input.material.base_color;
    }
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
