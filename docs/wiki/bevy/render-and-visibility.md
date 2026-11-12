# Bevy 0.20: extraction, visibility, culling, bins, views, shadows

Facts only, from the stock 0.20.0 source (`~/.cargo/registry/src/*/<crate>-0.20.0/`); paths are
`<crate>/src/...:line` there (vendored copies differ). Examples at tag v0.20.0, local copies in
`C:\cargo-target\refs\bevy-examples-0.20\`.

## 1. Main world: visibility (PostUpdate)

- `Visibility` -> `InheritedVisibility` -> `ViewVisibility` (visible in at least one view),
  `bevy_camera/src/visibility/mod.rs:83,164,226`. Per-view lists are per `VisibilityClass` (`mod.rs:211`).
- `reset_view_visibility` (`mod.rs:733`) and `mark_newly_hidden_entities_invisible` (`mod.rs:908`) each walk every
  entity without `NoCpuCulling` every frame. They bypass change detection, so `ViewVisibility` is `Changed` only
  on a real visible/hidden transition.
- `check_visibility_cpu_culling` (`mod.rs:748`): for each active camera, a `par_iter_mut` over every entity
  without `NoCpuCulling` (`mod.rs:786`): inherited visibility, render layers, visibility range lookup, sphere then
  OBB frustum test. Survivors go in the camera's `VisibleEntities`, then sorted (`mod.rs:873`). Cost is
  cameras x entities every frame, moving or not.
- `NoCpuCulling` (`mod.rs:69`) on an entity skips all of the above: `check_visibility_gpu_culling` (`mod.rs:884`)
  touches it only when `InheritedVisibility` changed or the marker was added, and sets `ViewVisibility` =
  `InheritedVisibility`. On a camera it skips the frustum test for that camera only.
- `NoFrustumCulling` (`mod.rs:318`): no frustum test, but still scanned and listed per view. `NoAutoAabb`
  (`mod.rs:551`): `calculate_bounds` (`mod.rs:557`) computes no `Aabb`. `DynamicSkinnedMeshBounds` (`mod.rs:331`):
  `update_skinned_mesh_bounds` (`mod.rs:596`) refits a skinned mesh's `Aabb` from its joints each frame.

## 2. Visibility ranges (LOD)

- `VisibilityRange { start_margin, end_margin, use_aabb }` (`bevy_camera/src/visibility/range.rs:80`). One entity
  per LOD level; there is no LOD-group component. The crossfade is a dither in the fragment shader.
- `check_visibility_ranges` (`range.rs:230`) runs every frame: clears `VisibleEntityRanges`, tests every entity
  with a range and without `NoCpuCulling` against each view (cameras and `ShadowLodOrigin`, first 32 only,
  `range.rs:248`), and inserts every in-range entity into an `EntityHashMap<u32>` bitmask (`range.rs:188`),
  rebuilt from scratch each frame. Culling systems then do one hash lookup per entity per view.
- Render world: `RenderVisibilityRanges` (`bevy_render/src/view/visibility/range.rs:63`) deduplicates identical
  ranges into a GPU buffer indexed by `NonMaxU16`; the index goes into the mesh flags at extraction.
- For `NoCpuCulling` entities the range is evaluated per view in the GPU preprocess shader.

## 3. Main world: light visibility

- `check_dir_light_mesh_visibility` (`bevy_light/src/lib.rs:342`): per directional light and view, one `par_iter`
  over all `Mesh3d` without `NotShadowCaster` / `NoCpuCulling`, tested against every cascade frustum; results in
  `CascadesVisibleEntities`, sorted (`lib.rs:493`). It also sets `ViewVisibility` visible (`lib.rs:506`).
- `check_point_light_mesh_visibility` (`lib.rs:517`): for each shadow-mapped point light visible from any view,
  one full `par_iter_mut` over the same caster query (`lib.rs:594`): light sphere vs OBB, then six face frusta;
  results in `CubemapVisibleEntities` (six sorted lists). Spot lights: one pass each (`lib.rs:685`). Cost is
  shadowed lights x caster entities every frame. Lights with `shadow_maps_enabled == false` are skipped.
- Both skip `NoCpuCulling` entities entirely (`lib.rs:367,547`).
- Cascades: `CascadeShadowConfig` (`bevy_light/src/cascade.rs:73`); each cascade is one shadow view.

## 4. Extraction (main -> render world)

- `extract_meshes_for_gpu_building` (`bevy_pbr/src/render/mesh.rs:1953`): the query is filtered by
  `Changed<ViewVisibility | GlobalTransform | PreviousGlobalTransform | Aabb | Mesh3d | VisibilityRange |
  RenderLayers | SkinnedMesh | ...>`. The filter is a full table scan of all mesh entities each frame
  (`mesh.rs:2055`), run with `par_iter`; only changed rows do work.
- An entity whose `ViewVisibility` is false is removed from `RenderMeshInstances` and its input uniform freed
  (`mesh.rs:2166`). A CPU-culled mesh that leaves every frustum and comes back is extracted again from scratch.
  A `NoCpuCulling` mesh stays resident as long as it is not hidden.
- `RenderGpuCulledEntities` (`mesh.rs:1719`) is the table of `NoCpuCulling` meshes; `collect_gpu_culled_meshes`
  (`mesh.rs:2287`) copies it into every camera's and every shadow subview's `RenderVisibleEntities`, filtered only
  by `RenderLayers`. A view component that was just added gets a full flush of the whole table (`mesh.rs:2345`).
- `extract_cameras` (`bevy_render/src/camera.rs:473`) and `extract_lights` (`bevy_pbr/src/render/light.rs:333`)
  copy each camera's, cascade's and face's sorted visible list.
- `extract_skins` (`bevy_pbr/src/render/skin.rs:272`) handles skins whose `ViewVisibility` / `SkinnedMesh`
  changed and reads joints from a `Changed<GlobalTransform>` query. `no_automatic_skin_batching` (`skin.rs:502`)
  adds `NoAutomaticBatching` only when skins use uniform buffers (WebGL-class limits); with storage buffers
  skinned meshes batch normally.

## 5. Render world: retained per-view lists, specialization, bins

- Views are identified by `RetainedViewEntity` (`bevy_render/src/view/mod.rs:367`): main entity + subview index.
  Phases and caches are keyed by it and persist while the same key keeps showing up.
- `collect_visible_cpu_culled_entities` (`bevy_render/src/view/visibility/mod.rs:325`) diffs this frame's sorted
  list against last frame's for each view and subview (lockstep march, O(visible)) and produces
  `added_entities` / `removed_entities` (`mod.rs:90`).
- `DirtySpecializations` (`bevy_render/src/camera.rs:826`): specialization and queueing visit only entities newly
  visible in the view or whose mesh/material changed (`iter_to_specialize` `camera.rs:910`, `iter_to_dequeue`
  `:950`, `iter_to_queue` `:990`). Pipeline ids are cached per view per entity
  (`SpecializedMaterialViewPipelineCache`, `bevy_pbr/src/material.rs:904`).
- `queue_material_meshes` (`bevy_pbr/src/material.rs:1189`) removes dequeued entities from the phase (`:1241`)
  and inserts newly queued ones (`:1258`). `queue_shadows` (`bevy_pbr/src/render/light.rs:2670`) likewise.
- `BinnedRenderPhase` (`bevy_render/src/render_phase/mod.rs:109`) is retained: `multidrawable_meshes` (batch set
  -> bins -> instances), `batchable_meshes`, `unbatchable_meshes` (one draw each). Entity -> bin is cached.
- `RenderMultidrawableBatchSet` (`render_phase/mod.rs:476`) keeps GPU buffers of binned mesh instances; a compute
  step (bin unpacking + uniform allocation, `bevy_render/src/batching/gpu_preprocessing.rs:1052-1135`) turns
  them into `PreprocessWorkItem`s (`gpu_preprocessing.rs:786`) on the GPU. The CPU writes only deltas.
- So a mesh that stays in a view's list costs no per-frame CPU in queue/bin code; cost appears when entities
  enter or leave a list, and when a view key is new (everything in its list is "added").

## 6. GPU preprocessing, culling, indirect draws

- `GpuPreprocessingMode` (`gpu_preprocessing.rs:150`): `None`, `PreprocessingOnly` (view has `NoIndirectDrawing`,
  `bevy_render/src/view/mod.rs:903`), `Culling` (default). `GpuPreprocessingSupport` (`:116`) is the adapter's cap.
- In `Culling` mode each view runs the mesh preprocess compute pass over its work items: frustum cull (and
  visibility range for GPU-culled meshes), writes `MeshUniform`s and indirect draw parameters; batch sets draw
  with multi-draw indirect. Work items per view = instances in that view's bins, so a `NoCpuCulling` mesh is a
  work item in every view whose layers it intersects.
- `OcclusionCulling` (`bevy_render/src/occlusion_culling/mod.rs:73`) on a camera (needs `DepthPrepass`) or on a
  directional light adds two-phase hi-Z occlusion culling to that view's preprocess.
- Meshes live in shared slabs (`MeshAllocatorSettings`, `bevy_render/src/mesh/allocator.rs:67`);
  `allocate_and_free_meshes` (`:249`) uploads every newly extracted mesh in the frame it arrives, with no
  per-frame limit. Same slab + pipeline + material bind group can share a batch set (multi-draw).

## 7. Views and schedules

- Every shadow subview is a render-world entity with `ExtractedView` + `ShadowView`
  (`bevy_pbr/src/render/light.rs:998`), created in `prepare_lights` (`light.rs:1051`): six per point light
  (`create_point_shadow_maps`, `light.rs:2134`), one per spot light, one per cascade per camera.
- Shadow views carry `RootNonCameraView(Core3d)` (`light.rs:2239,2332`; type at
  `bevy_core_pipeline/src/schedule.rs:126`). The camera driver runs the named schedule once for every entity with
  that component (`schedule.rs:142,222`), so each cube face and each cascade runs a schedule of its own.
- Stock 0.20 redraws every shadow view every frame; there is no shadow map caching.

## 8. Transforms

- `mark_dirty_trees` (`bevy_transform/src/systems.rs:111`) propagates a dirty bit to ancestors so propagation skips
  clean subtrees; controlled by `StaticTransformOptimizations` (`systems.rs:89`, enabled by default).

## 9. What the reference examples configure

- `bevy_city` (`main.rs`): stock `DefaultPlugins` and `TransformPlugin`, `StaticTransformOptimizations::Enabled`
  (`:113`); one `Camera3d` with `Hdr`, `Msaa::Off`, TAA, bloom, atmosphere, contact shadows; one shadowed
  `DirectionalLight` (default cascades); no point lights; one entity per mesh; LOD by `VisibilityRange` copied
  onto each mesh (`propagate_visibility_range`, `:461`); `NoCpuCulling` on every mesh behind `--no-cpu-culling`
  (`:61,428-458`), inserted once after spawning; moving cars are ordinary entities.
- `many_cubes`: switches for `NoFrustumCulling`, `NoAutomaticBatching`, `NoCpuCulling` per mesh and
  `NoIndirectDrawing` / `NoCpuCulling` on the camera (`many_cubes.rs:209-220`); shadows only with `--shadows`.
- `bistro`: `OcclusionCulling` on camera and directional light, optional `NoCpuCulling` (`main.rs:258,290,294`).
