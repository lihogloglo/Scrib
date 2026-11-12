# Bevy 0.20: skinning, joint buffers, morph targets, animation graph

Facts only, from the stock 0.20.0 source (`~/.cargo/registry/src/*/<crate>-0.20.0/`); paths are
`<crate>/src/...:line`. Patch noted in section 2. Visibility side of skinned bounds:
[render-and-visibility.md](render-and-visibility.md) section 1.

## 1. Main world

- `SkinnedMesh { inverse_bindposes: Handle<SkinnedMeshInverseBindposes>, joints: Vec<Entity> }`
  (`bevy_mesh/src/skinning.rs:16`); the mesh asset carries joint indices and weights as vertex attributes.
  Joint matrix = joint `GlobalTransform.affine()` * inverse bindpose.
- Shader limit `MAX_JOINTS = 256` (`bevy_pbr/src/render/skin.rs:28`).
- `DynamicSkinnedMeshBounds` (`bevy_camera/src/visibility/mod.rs:331`) refits the entity `Aabb` from joints each
  frame (`:594`).

## 2. Extraction and the joint buffer

- Schedule: `extract_skins` and `extract_morphs` run in `ExtractSchedule` parallel with mesh extraction;
  `prepare_skins` in `RenderSystems::PrepareResources`; `write_morph_buffers` in `PrepareResourcesFlush`
  (`bevy_pbr/src/render/mesh.rs:200-216`). `prepare_mesh_bind_groups` in `PrepareBindGroups` (:217).
- `SkinUniforms` (`skin.rs:76`): one CPU staging `Vec<Mat4>`, two GPU buffers (`current_buffer`, `prev_buffer`), an
  offset allocator in units of 4 matrices (256 B, :41), and a per-skinned-entity `SkinUniformInfo` (offset + joint
  entity list).
- `extract_skins` (`skin.rs:272`):
  1. Skins whose `ViewVisibility`, `SkinnedMesh` or inverse-bindpose asset changed are removed and, if visible,
     re-added: allocate a block, write every joint matrix (`add_or_delete_skins` :327, `add_skin` :417). A skin
     that is not `ViewVisibility` visible has no block.
  2. For every other skinned entity (the loop is over all `SkinnedMesh` entities every frame, :303), walk its joint
     list and look each joint up in a `Changed<GlobalTransform>` query; changed joints overwrite their staging
     slot (`extract_joints_for_skin` :367). Cost is skinned entities x joints per frame, lookups only.
- `prepare_skins` (:176): returns early if staging is empty; swaps `current_buffer` and `prev_buffer` (:188), grows
  both by 1.5x when needed (:190-225, new `prev_buffer` is filled from the new staging), then `write_buffer` of the
  whole staging array into `current_buffer` (:236). Previous-frame matrices (motion vectors) are therefore the
  buffer that was current last frame, same offsets. Storage buffers unless `skins_use_uniform_buffers` (:171).
- Patch P5 (vendor README, `bevy_pbr/src/render/skin.rs`): `SkinJointBlocks` shares one refcounted block between
  skinned meshes with the same inverse-bindpose asset and joint entity list, so joints are walked and uploaded
  once per block, not per mesh. Stock keys per mesh entity.

## 3. Mesh instance data, bind groups, batching

- Each skinned instance stores `current_skin_index` (byte offset / 64) in its input uniform
  (`mesh.rs:555,618,1497`; `u32::MAX` = none). `set_mesh_motion_vector_flags` (:2455) sets `HAS_PREVIOUS_SKIN` /
  `HAS_PREVIOUS_MORPH` after extraction for every entity in `SkinUniforms::all_skins()` (:2461) and the morph map.
- Bind groups: `MeshPhaseBindGroups` (`mesh.rs:3815`) holds `model_only`, `skinned` (keyed by metadata
  `MeshSlabId`), morph and lightmap maps; the skinned group binds `current_buffer` and `prev_buffer`
  (:4132-4146). `prepare_mesh_bind_groups` (:3998) calls `reset()` (:3900; call sites :4036, :4082) and refills them in `PrepareBindGroups`, so they are rebuilt with the buffers as swapped that frame.
- Pipelines: `SKINNED` shader def and skinned/morphed layout variants are chosen from the mesh vertex layout
  (`mesh.rs:3323-3375`).
- Batching: with storage buffers, skinned and morphed meshes batch like any other (one offset per instance).
  `no_automatic_skin_batching` (`skin.rs:502`) and `no_automatic_morph_batching` (`morph.rs:433`) insert
  `NoAutomaticBatching` only when skins use uniform buffers (WebGL-class limits).

## 4. Morph targets

- `MorphWeights` / `MeshMorphWeights`, `MAX_MORPH_WEIGHTS = 256` (`bevy_mesh/src/morph.rs:15,81`). Morph
  displacement data lives in the mesh slab allocator as `ElementClass::MorphTarget`
  ([render-assets-and-materials.md](render-assets-and-materials.md) section 5).
- `MorphIndices` (`bevy_pbr/src/render/morph.rs:36`): Uniform variant (per-entity current/prev offsets) or Storage
  variant (`MorphWeightsInfo` with current and previous weight offsets, `GpuMorphDescriptor` indices with a free
  list). `extract_morphs` (:249), `prepare_morph_descriptors` (:350), `write_morph_buffers` (:192).

## 5. bevy_animation (0.20)

- `AnimationPlugin` (`bevy_animation/src/lib.rs:1312`) registers in `PostUpdate`, chained, before
  `TransformSystems::Propagate` (:1322-1340): `thread_animation_graphs` (before `AssetEventSystems`),
  `advance_transitions`, `advance_animations`, `animate_targets`, `trigger_untargeted_animation_events`,
  `expire_completed_transitions`.
- Data: `AnimationClip` (curves keyed by `AnimationTargetId`, a UUID from hashing the bone name path,
  :184-187), `AnimationGraph` asset (`graph.rs:115`) of nodes `Clip` / `Blend` / `Add` (`graph.rs:214`) with
  weights, `AnimationGraphHandle` on the player (:139), `AnimationPlayer` (:748) holding `ActiveAnimation`s
  (:522), `AnimationTransitions` (`transition.rs:33`), `AnimationTargetId` + `AnimatedBy(player)` on each
  animated entity (:215).
- `advance_animations` and `animate_targets` are `par_iter_mut` (:1055, :1112): `animate_targets` iterates every
  entity with `AnimationTargetId` + `AnimatedBy`, evaluates the player's graph for it, and writes the animated
  field (e.g. `Transform::translation`) directly; evaluation state is thread-local (:1106). Morph weight
  curves: `bevy_animation/src/morph.rs`.

Scrib uses this in [frame and views](../engine/frame-and-views.md).
