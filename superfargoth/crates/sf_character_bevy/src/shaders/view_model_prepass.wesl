// sf_character_bevy: the view model's prepass vertex stage (first-person.md "View model", FP-2b;
// `ViewModelMaterial`, `MaterialExtension::{prepass_vertex_shader, deferred_vertex_shader}`).
//
// A copy of Bevy's `bevy_pbr` `prepass/prepass.wesl` `vertex` (and its `morph_vertex`,
// `morph_prev_vertex`), Bevy 0.20.0 as vendored in
// `superfargoth/vendor/bevy_pbr/src/prepass/prepass.wesl` l.20-173. RE-SYNC on every Bevy update:
// diff that file and port its changes here (and in `view_model.wesl`). The changes:
// - `out.world_position` (vendor l.92) is the pseudo-world position (`view_model_world`): the
//   prepass fragments read it only for motion vectors (vendor `prepass.wesl` l.187;
//   `StandardMaterial`'s `render/pbr_prepass.wesl` l.148 and `pbr_prepass_functions.wesl`
//   l.93-97), which then measure the motion the screen shows;
// - `out.position` (l.93) is the view-model clip position, the main pass's exactly (depth
//   prepass equality);
// - `out.previous_world_position` (l.157-160) is the previous frame's pseudo-world position,
//   through the previous view.
// The shadow-view clamp (l.94-97, `UNCLIPPED_DEPTH_ORTHO_EMULATION`) is kept as is but never runs:
// view copies are `NotShadowCaster`.

import bevy_pbr::{
    prepass::{
        bindings as prepass_bindings,
        io::{Vertex, UncompressedVertex, VertexOutput, decompress_vertex},
    },
    render::{
        mesh_bindings::mesh,
        mesh_functions,
        skinning,
        morph,
        morph::{morph_position, morph_normal, morph_tangent},
        mesh_view_bindings::view,
        view_transformations::position_world_to_clip,
    },
};
import super::view_model_common::{view_model_params, view_model_world, view_model_clip};

@if(MORPH_TARGETS)
// The instance_index parameter must match vertex_in.instance_index. This is a work around for a wgpu dx12 bug.
// See https://github.com/gfx-rs/naga/issues/2416
fn morph_vertex(vertex_in: UncompressedVertex, instance_index: u32) -> UncompressedVertex  {
    var vertex = vertex_in;
    let first_vertex = mesh[instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;
    let morph_descriptor_index  = mesh[instance_index].morph_descriptor_index;

    let weight_count = morph::layer_count(morph_descriptor_index);
    for (var i: u32 = 0u; i < weight_count; i ++) {
        let weight = morph::weight_at(i, morph_descriptor_index);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph_position(vertex_index, i, morph_descriptor_index);
@if(VERTEX_NORMALS)
        vertex.normal += weight * morph_normal(vertex_index, i, morph_descriptor_index);
@if(VERTEX_TANGENTS)
        vertex.tangent += vec4(weight * morph_tangent(vertex_index, i, morph_descriptor_index), 0.0);
    }
    return vertex;
}

@if(MORPH_TARGETS)
/// Returns the morphed position of the given vertex from the previous frame.
///
/// This function is used for motion vector calculation, and, as such, it doesn't
/// bother morphing the normals and tangents.
///
/// The instance_index parameter must match vertex_in.instance_index. This is a work around for a wgpu dx12 bug.
/// See https://github.com/gfx-rs/naga/issues/2416
fn morph_prev_vertex(vertex_in: UncompressedVertex, instance_index: u32) -> UncompressedVertex {
    var vertex = vertex_in;
    let first_vertex = mesh[instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;
    let morph_descriptor_index  = mesh[instance_index].morph_descriptor_index;
    let weight_count = morph::layer_count(morph_descriptor_index);
    for (var i: u32 = 0u; i < weight_count; i ++) {
        let weight = morph::prev_weight_at(i, morph_descriptor_index);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph_position(vertex_index, i, morph_descriptor_index);
        // Don't bother morphing normals and tangents; we don't need them for
        // motion vector calculation.
    }
    return vertex;
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let uncompressed_vertex_no_morph = decompress_vertex(vertex_no_morph, vertex_no_morph.instance_index);
@if(MORPH_TARGETS)
    var vertex = morph_vertex(uncompressed_vertex_no_morph, vertex_no_morph.instance_index);
@else
    var vertex = uncompressed_vertex_no_morph;

    let mesh_world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);

@if(SKINNED)
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    var world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        vertex_no_morph.instance_index
    );
@else // SKINNED
    var world_from_local = mesh_world_from_local;

    // View model (vendor l.92-93 were the real world position and its clip position).
    let vm = view_model_params(vertex_no_morph.instance_index);
    let real_world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.world_position = vec4(view_model_world(real_world_position.xyz, view.view_from_world, vm.fov_scale), 1.0);
    out.position = view_model_clip(position_world_to_clip(out.world_position.xyz), vm.depth);
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION) {
    out.unclipped_depth = out.position.z;
    out.position.z = min(out.position.z, 1.0); // Clamp depth to avoid clipping
}

@if(VERTEX_UVS_A)
    out.uv = vertex.uv;

@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;

@if(NORMAL_PREPASS_OR_DEFERRED_PREPASS) {
@if(VERTEX_NORMALS && SKINNED)
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
@elif(VERTEX_NORMALS)
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );

@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );
}

@if(VERTEX_COLORS)
    out.color = vertex.color;

    // Compute the motion vector for TAA among other purposes. For this we need
    // to know where the vertex was last frame.
@if(MOTION_VECTOR_PREPASS) {

    // Take morph targets into account.
@if(MORPH_TARGETS && HAS_PREVIOUS_MORPH)
    let prev_vertex = morph_prev_vertex(uncompressed_vertex_no_morph, vertex_no_morph.instance_index);
@elif(MORPH_TARGETS)
    let prev_vertex = uncompressed_vertex_no_morph;
@else   // MORPH_TARGETS
    let prev_vertex = uncompressed_vertex_no_morph;

    // Take skinning into account.
@if(SKINNED && HAS_PREVIOUS_SKIN)
    let prev_model = skinning::skin_prev_model(
        prev_vertex.joint_indices,
        prev_vertex.joint_weights,
        vertex_no_morph.instance_index
    );
@elif(SKINNED)
    // Use vertex_no_morph.instance_index instead of prev_vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    let prev_model = mesh_functions::get_previous_world_from_local(vertex_no_morph.instance_index);
@else   // SKINNED
    // Use vertex_no_morph.instance_index instead of prev_vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    let prev_model = mesh_functions::get_previous_world_from_local(vertex_no_morph.instance_index);

    // View model (vendor l.157-160 were the real previous world position): through the previous
    // view, so the stock motion vector is the drawn motion.
    let real_previous_world_position = mesh_functions::mesh_position_local_to_world(
        prev_model,
        vec4<f32>(prev_vertex.position, 1.0)
    );
    out.previous_world_position = vec4(
        view_model_world(real_previous_world_position.xyz, prepass_bindings::previous_view_uniforms.view_from_world, vm.fov_scale),
        1.0
    );
}

@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = vertex_no_morph.instance_index;

@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_no_morph.instance_index, mesh_world_from_local[3]);

    return out;
}
