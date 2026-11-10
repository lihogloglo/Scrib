# First person: as-built history

Moved out of [first-person.md](../first-person.md) on 2026-11-10 (doc budget, FP-2b). Append-only; grep it.

### FP-2a, view-model projection math and view copies (as built 2026-11-10)

**As built, FP-2a (2026-11-10).** Math: `sf_camera::view_model` (`fov_scale`,
`ViewModelProjection { k, depth }`, `ComfortSettings::view_model_projection`), exact tests in
`sf_camera/src/tests_view_model.rs`. The formulas both WESL stages mirror, line for line
(`view_from_world` rigid, as Bevy's camera views are; `right`/`up` = its rows 0 and 1):

```text
fn view_model_world(p, view_from_world, k) -> vec3:       // pseudo-world position
    let v = view_from_world * vec4(p, 1.0);
    let right = vec3(view_from_world[0].x, view_from_world[1].x, view_from_world[2].x);
    let up = vec3(view_from_world[0].y, view_from_world[1].y, view_from_world[2].y);
    return p + (k - 1.0) * (v.x * right + v.y * up);    // = world_from_view S_k view_from_world p
fn view_model_clip(clip, r) -> vec4:                      // depth slice
    return vec4(clip.xy, r * clip.w + (1.0 - r) * clip.z, clip.w);
position = view_model_clip(position_world_to_clip(view_model_world(world, view.view_from_world, k)), r)
```

The rows form needs no `world_from_view`, which `PreviousViewUniforms` lacks. Tested: in view
space x, y scale by `k` and depth is kept (equal to the inverse-matrix form); through Bevy's jittered
`perspective_infinite_reverse` the result is the `fov_body` projection with the same jitter;
`ndc' = R + (1 - R) ndc` keeps order and the near plane, and only geometry inside `near / R` can
cover; Bevy's stock motion-vector formula over the pseudo positions of two frames equals the drawn
motion (zero for an arm still on screen). **FP-2b's two stages** (vendor `bevy_pbr`):
`render/mesh.wesl` `vertex` (l.34-98): keep l.67 `world_position`, replace l.69 `position` by the
line above. `prepass/prepass.wesl` `vertex` (l.70-174): l.92 `world_position` = the pseudo position
(the prepass fragment reads it only for motion vectors: l.187, StandardMaterial's
`pbr_prepass.wesl` l.148 / `pbr_prepass_functions.wesl` l.93-97), l.93 `position` as above; l.157-160
`previous_world_position` = `view_model_world(prev, previous_view_uniforms.view_from_world, k)`;
l.94-97 (shadow-view ortho clamp) never runs, the copy casting nothing.

Copies: `sf_character_bevy::roles`. `RoleLayer::view_copy` (+ `VIEW_MODEL`: `S` only plus a copy)
gives every `PartRole` part of the role a `ViewCopy { source }` child (`Name` "view copy", `W`,
`NotShadowCaster`, `NoRayTracing`, the part's `Mesh3d`, `MeshMaterial3d<StandardMaterial>`,
`SkinnedMesh` with the **same joint entities**, morph weights, `Aabb`/`NoAutoAabb` or
`DynamicSkinnedMeshBounds`). Being the part's child, it despawns with it (outfit swap, held change)
and inherits its visibility; `sync_role_layers` spawns, re-layers and despawns copies with the rule,
the root's layers (travel: the worldspace push writes `W` to copies too) and the outfit;
`follow_view_copies` (after it) re-copies the part's mesh, material, skin, morph weights and bounds
on change, so a host's material swap (`mw_actors_bevy` `held.rs` self-lit torches, which also
reaches the copy itself as a descendant of the prop) needs no hook. Bevy 0.20 skins every mesh
from its own `SkinnedMesh::joints` (one palette per mesh), and the outfit already shares one rig's
joints between meshes, so a copy is one more palette, no second rig. Traced scene and GI:
`sf_sky_bevy` `mirror_meshes` mirrors every visible `StandardMaterial` mesh in the traced layers,
skinned or not, so a copy on `W` would be mirrored: `NoRayTracing` keeps it out (its
`Without<NoRayTracing>` filter), and DDGI traces only `RaytracingMesh3d`. No `sf_sky_bevy` change.
**Gate:** the avatar asks for `first_person(ComfortSettings::view_model)`, off by default: with the
stock material a copy draws exactly where the part did (the part is off `W`, so nothing draws
twice), at one extra draw and palette for nothing, so FP-2a changes nothing in game until FP-2b.
Tests: `sf_character_bevy` (copies only with the view model, arms and held only, on `W`, parts on
`S`, shared mesh/material/joints, material swap, held change, outfit swap, back to third person,
travel with feature `world`), `mw_player_bevy` avatar (both states of the setting through the real
camera and layer systems).

FP-2b must know: (a) Bevy's clustered lighting indexes the cluster by `frag_coord.xy` with the real
view z (`pbr_functions.wesl` l.457, `clustered_forward.wesl` l.57): the copy's drawn pixel is not its
lit position's, so near the screen edge a clustered light can be missed (fix in the extension's
fragment or accept; check in FP-2c); (b) the material is the part's, swapped by hosts (torches):
derive the extension per part handle and follow `follow_view_copies`' changes; asset mutation in
place does not change the handle; (c) `StandardMaterial` is bindless in 0.20: the extension's
`k`/`R` uniform must fit that layout or the copy's pipeline is non-bindless (one bind group per
material); (d) a headless check is possible: link the stages with `wesl` against the vendored
modules per def set and naga-validate (pattern: `sf_sky_bevy/src/shader_tests.rs`,
`sf_surface_bevy`'s), defs at least `SKINNED`, `MORPH_TARGETS`, `MOTION_VECTOR_PREPASS`,
`HAS_PREVIOUS_SKIN`, `HAS_PREVIOUS_MORPH`, `VERTEX_TANGENTS`; pipeline creation on a GPU stays an
owner log-line check.
