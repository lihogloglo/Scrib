// sf_character_bevy: the first-person view model's projection (first-person.md "View model",
// FP-2b), shared by the main-pass and prepass vertex stages of `ViewModelMaterial`
// (`view_model.wesl`, `view_model_prepass.wesl`). The math is `sf_camera::view_model`
// (`ViewModelProjection::pseudo_world`, `depth_slice`), mirrored line for line and tested there;
// `shader_tests.rs` checks the two helpers below still match its "WESL mirror" lines.
//
// The extension's data, `ViewModel` (Rust `ViewModelUniform`), at material binding 100
// (non-bindless) or in the bindless arrays at 101 (index table) and 102 (data), as Bevy 0.20's
// `extended_material_bindless` example lays them out next to `StandardMaterial`'s 0..31.

import bevy_pbr::render::mesh_bindings::mesh;

/// `fov_scale` = k = tan(fov_world/2) / tan(fov_body/2); `depth` = R, the reverse-Z share kept
/// for the view model (0: no slice).
struct ViewModel {
    fov_scale: f32,
    depth: f32,
}

@if(BINDLESS) {
/// One entry per bindless slot: the slot's index into `view_model_array`.
struct ViewModelIndices {
    material: u32,
}
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var<storage> view_model_indices: array<ViewModelIndices>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var<storage> view_model_array: array<ViewModel>;
}
@else {
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> view_model: ViewModel;
}

/// The drawing mesh's view-model parameters (its material's bindless slot, or the uniform).
fn view_model_params(instance_index: u32) -> ViewModel {
@if(BINDLESS)
    let slot = mesh[instance_index].material_and_lightmap_bind_group_slot & 0xffffu;
@if(BINDLESS)
    return view_model_array[view_model_indices[slot].material];
@else
    return view_model;
}

/// The pseudo-world position: `world_from_view * S_k * view_from_world * p` in the rows form
/// (`view_from_world` rigid, as Bevy's camera views are; needs no `world_from_view`, which the
/// previous-view uniforms lack). `sf_camera::ViewModelProjection::pseudo_world`.
fn view_model_world(p: vec3<f32>, view_from_world: mat4x4<f32>, k: f32) -> vec3<f32> {
    let v = view_from_world * vec4(p, 1.0);
    let right = vec3(view_from_world[0].x, view_from_world[1].x, view_from_world[2].x);
    let up = vec3(view_from_world[0].y, view_from_world[1].y, view_from_world[2].y);
    return p + (k - 1.0) * (v.x * right + v.y * up);
}

/// The depth slice: `z' = R w + (1 - R) z` (reverse Z: `ndc' = R + (1 - R) ndc`).
/// `sf_camera::ViewModelProjection::depth_slice`.
fn view_model_clip(clip: vec4<f32>, r: f32) -> vec4<f32> {
    return vec4(clip.xy, r * clip.w + (1.0 - r) * clip.z, clip.w);
}
