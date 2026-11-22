// sf_groundcover_bevy: the groundcover prepass / shadow vertex stage (groundcover.md "Renderers", GC-2;
// `GroundcoverMaterial`, `MaterialExtension::{prepass_vertex_shader, deferred_vertex_shader}`). The prepass
// fragment stays `StandardMaterial`'s (alpha mask from the base colour texture).
//
// Bevy's `bevy_pbr` `prepass/prepass.wesl` `vertex` (Bevy 0.20.0 as vendored in
// `superfargoth/vendor/bevy_pbr/src/prepass/prepass.wesl` l.80-173) without skinning and morph targets, plus the
// main pass's fade and bend (`groundcover.wesl`; same functions, so depth matches the main pass exactly) and
// a motion vector that includes the plant's own motion: `previous_world_position` is the same vertex bent by
// last frame's wind field (`bent_prev`; batch instances never move, so the previous model matrix is the
// current one, `mesh_preprocess.wesl`). RE-SYNC on every Bevy update.

import bevy_pbr::{
    prepass::io::{Vertex, VertexOutput, decompress_vertex},
    render::{
        mesh_functions,
        mesh_view_bindings::view,
        view_transformations::position_world_to_clip,
    },
};
import super::groundcover_common::{unpack_tag, fade_scale, instance_scale, bent, bent_prev};

@vertex
fn vertex(vertex_in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let vertex = decompress_vertex(vertex_in, vertex_in.instance_index);
    let world_from_local = mesh_functions::get_world_from_local(vertex_in.instance_index);
    let tag = unpack_tag(mesh_functions::get_tag(vertex_in.instance_index));
    let root = world_from_local[3].xyz;
    let s = fade_scale(root, view.world_position, tag.fade);
    let unbent = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position * s, 1.0));
    out.world_position = vec4<f32>(bent(unbent.xyz, root, instance_scale(world_from_local), tag), 1.0);
    out.position = position_world_to_clip(out.world_position.xyz);
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION) {
    out.unclipped_depth = out.position.z;
    out.position.z = min(out.position.z, 1.0); // Clamp depth to avoid clipping
}

@if(VERTEX_UVS_A)
    out.uv = vertex.uv;

@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;

@if(NORMAL_PREPASS_OR_DEFERRED_PREPASS) {
@if(VERTEX_NORMALS)
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_in.instance_index);

@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex_in.instance_index
    );
}

@if(VERTEX_COLORS)
    out.color = vertex.color;

@if(MOTION_VECTOR_PREPASS) {
    let prev_model = mesh_functions::get_previous_world_from_local(vertex_in.instance_index);
    let prev_root = prev_model[3].xyz;
    let prev_unbent = mesh_functions::mesh_position_local_to_world(prev_model, vec4<f32>(vertex.position * s, 1.0));
    out.previous_world_position = vec4<f32>(bent_prev(prev_unbent.xyz, prev_root, instance_scale(prev_model), tag), 1.0);
}

@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = vertex_in.instance_index;

@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_in.instance_index, world_from_local[3]);

    return out;
}
