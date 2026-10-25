# Perf research: how Bevy (and big Bevy projects) solve what we measured

Research only (2026-10-19, nothing built or run). Sources: Bevy **0.20.0** crate source in
`~/.cargo/registry` (released 2026-10-08, so every PR merged before then is in it), `big_space
0.14.0-rc.0` source, Bevy PRs/issues via `gh`, Bevy's `examples/large_scenes/bevy_city` at `v0.20.0`,
the Mudcrab shallow clone (`C:\cargo-target\refs\mudcrab`, `3129b57`). Our numbers: `bevyrrowind/docs/perf.md`
"Frame budget". **"Guess"** marks every saving I could not derive from a measurement.

> **Measured 2026-10-19 (phase1, `bevyrrowind/docs/perf.md` "The gameplay view"):** item 1 below
> (`NoCpuCulling` on static opaque/masked parts) made the Balmora walk **81 ms instead of 26** (render 81, GPU
> 22) and was reverted; the animation budget (item 5) gave about -1 ms main. The whole-system A/B says the cost is the
> near-ring static mesh count (`--off statics`: 7.5 ms), so mesh merging per cell is the lever. Read the rest with that.

## Headline

Most of what we measured is the **per-view CPU culling path**. Bevy 0.20 ships the standard fix:
**`NoCpuCulling`** on static meshes. After PR #23107 those meshes skip every CPU visibility system
(camera and light views), live in retained render-world sets, and are frustum- and visibility-range-culled
on the GPU per view, cascades included. Bevy's own `bevy_city` stress test has a switch for it. On `bevy_city`
it removed `check_visibility` (1.19 ms) and the directional-light mesh visibility check (4.33 ms).
We already have 50% GPU headroom to spend on that trade. Second: **big_space's low-precision propagation
has no dirty-tree pruning**. It walks every static mesh every frame, unlike Bevy's own propagation since 0.16.
Third: we have **too many views** (about 16). Most render-thread spans scale with views × phases.

## Ranked recommendations

Savings are on Balmora/walk numbers. Render = render thread, main = main thread. "Kind": B = Bevy
feature/setting, C = maintained crate (patch), O = our own code.

| # | What | Expected saving (ours) | Effort | Kind | Licence | Source |
|---|---|---|---|---|---|---|
| 1 | `NoCpuCulling` on all static **opaque and alpha-mask** meshes (cells, terrain, far field). Keep `VisibilityRange` (it is then evaluated on the GPU). Also try it on the camera. | main -2 to -4 ms (16-view `check_visibility` plus cascade/cube visibility gone). Render -2 to -5 ms, most of all when **walking**: visible sets stop churning, so bins, extraction and `write_binned_instance_buffers<Shadow>` go quiet. GPU +0.5 to 2 ms. **Guess**, but bevy_city's 5.5 ms → 0 is the precedent. | S (insert at spawn behind an `--off`/toggle, A/B) | B | MIT/Apache | [#23107](https://github.com/bevyengine/bevy/pull/23107), [#23530](https://github.com/bevyengine/bevy/pull/23530), `bevy_city/src/main.rs` (`--no-cpu-culling`) |
| 2 | Patch `big_space::Grid::propagate_low_precision` to prune unchanged subtrees with Bevy's `mark_dirty_trees` + `TransformTreeChanged` (both public in `bevy_transform` 0.20), and offer it upstream | big_space task time 2.5 ms → ~0.3 ms (only actors and movers remain). Main wall time -0.5 to -1 ms (**guess**, the work is parallel across roots) | S-M | C (patch + upstream PR) | MIT/Apache | `big_space/src/grid/propagation.rs:318`, `bevy_transform/src/systems.rs:111` |
| 3 | **Cut views.** List the ~16 views. The one shadowed point light on High is 6 cube-face views by itself. Try 3 cascades instead of 4, and a lower sun `maximum_distance` (the far field has its own far shadows) | Render: per-view spans (`prepare_preprocess_bind_groups`, `write_binned_*`, pass encoding, part of `queue_submit`) scale ~linearly with views. -6 of 16 views ≈ -1.5 to -3 ms render (**guess**). One cascade fewer ≈ -1 ms (**guess**: shadows = 4.8 ms over 4 cascades) | S (settings + A/B) | B | - | `bevy_pbr/src/render/gpu_preprocess.rs:2188` (per view × per phase loop), Mudcrab `app.rs` cost note |
| 4 | Stop Bevy's every-frame `update_skinned_mesh_bounds` for frozen or throttled actors: drop `DynamicSkinnedMeshBounds` and run our own copy filtered to "pose written this frame". Or use one fixed per-actor bound (Unity `updateWhenOffscreen=false` style) | main -1 ms (of the 1.3 ms) | S | O (small system) | - | `bevy_camera/src/visibility/mod.rs:596`, [#21837](https://github.com/bevyengine/bevy/pull/21837) |
| 5 | **Merge each actor's body-part skinned meshes** into a few meshes per material group at spawn (`Mesh::merge` + one joint palette) | `extract_skins`+`prepare_skins` 1.2 ms → ~0.3. AABB and draw count fall by the parts-per-actor factor. -0.5 to -1 ms render, -0.3 main (**guess**) | M | B building blocks + O | - | UE [modular characters / skeletal mesh merge](https://dev.epicgames.com/documentation/en-us/unreal-engine/working-with-modular-characters-in-unreal-engine), `bevy_mesh/src/mesh.rs:2208` |
| 6 | Turn our existing animation budget on: `AnimLodSettings::budget_us` is `None` by default. Give off-screen shadow casters A2 significance, as the doc comment in `lod.rs` says | main -0.5 to -1 ms of the 1.5 ms animation. Fewer changed joints also shrinks skin extraction and propagation | S | O (exists) | - | `sf_anim_bevy/src/lod.rs`, UE [Animation Budget Allocator](https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-budget-allocator-in-unreal-engine) |
| 7 | Quick checks that may be free: (a) confirm wgpu validation/debug `InstanceFlags` are off in the `profiling` build (wgpu enables them under `debug_assertions`). (b) A/B `OcclusionCulling` on the main camera: we are CPU bound, and two-phase culling doubles the preprocess bind groups (4 per phase instead of 1) and adds the depth pyramid and late passes | (a) up to several ms if it is on; (b) -0.3 to -1 ms render, GPU + some (**guess**) | S | B | - | `PhasePreprocessBindGroups` docs, `gpu_preprocess.rs` |
| 8 | Cache preprocess bind groups across frames when the buffers did not change (same pattern as #25058 for materials), as an upstream patch | render -1.5 ms of the 2.3 ms (**guess**) | M | B (patch upstream) | MIT/Apache | [#25058](https://github.com/bevyengine/bevy/pull/25058) |
| 9 | Static cascade caching / far cascade at half rate (CryEngine `r_ShadowsCache`, Unity HDRP cached and mixed cached shadows, UE5 VSM page caching). Bevy 0.20 has nothing like it: `prepare_lights` rebuilds and clears every cascade every frame | render -1 to -2 ms after #1 (**guess**) | L (fork of `bevy_pbr` light.rs + shadow pass) | O/fork | - | [Unity HDRP mixed cached shadows](https://github.com/Unity-Technologies/Graphics/pull/1559), [UE VSM](https://dev.epicgames.com/documentation/en-us/unreal-engine/virtual-shadow-maps-in-unreal-engine) |
| 10 | Bake-time per-cell merge of small statics per material (Bethesda "precombined meshes") | After #1 mostly GPU draws and entity count: -0.5 to -1 ms render, smaller main (**guess**). Costs per-object culling and `VisibilityRange` granularity | M-L (bake) | O (on `Mesh::merge`) | - | `bevy_mesh` `merge`/`transformed_by` |
| - | **Not recommended now:** meshlets (`MeshletPlugin`). Experimental, Vulkan/Metal only, deferred-only when mixed, preprocessed `MeshletMesh`, no skinning, material limits (alpha mask). It saves GPU raster, and our problem is CPU. Also: `OcclusionCulling` on shadow views (lights accept it in 0.20). It adds passes and bind groups, so it saves GPU, not CPU. And: `#[component(summary_tick)]` (0.20), which cannot be applied to Bevy's own `GlobalTransform` without a fork | - | - | - | - | `bevy_pbr/src/meshlet/mod.rs:78-103`, `render/light.rs:399` |

Order of work: 7a (minutes), 1, 3 (both A/B flags, one perf suite run), 2, 4+6, then decide 5/8/9/10 from the
new trace. Item 1 changes the cost model so much that 9 and 10 should not be designed before it is measured.

---

## Q1. Render-thread cost model: many meshes × many views (Bevy 0.20)

**Views.** Every camera, every directional cascade (`RetainedViewEntity` subview per cascade) and every
cube face of a shadowed point light is an `ExtractedView`. The per-view work, all verified in source:

- `prepare_preprocess_bind_groups` (`bevy_pbr/src/render/gpu_preprocess.rs:2188`): `for view in views
  { for phase in phase_instance_buffers { create_bind_group(...) } }`, every frame. With indirect
  drawing plus two-phase occlusion culling that is 4 bind groups per (view, phase): indexed or
  non-indexed × early or late. Then it loops over views again for bin-unpacking bind groups. Nothing is
  cached. That explains 2.3 ms at ~16 views × the 3D phases (prepass opaque/alpha-mask, opaque, alpha-mask,
  shadow, …). Levers: fewer views, fewer phases per view, an upstream patch to cache them (#8).
- `write_binned_instance_buffers<BPI>` (`bevy_render/src/batching/gpu_preprocessing.rs:2840`): per
  (view, phase), per multidraw **batch set**, it calls `write_buffer` on four GPU buffers (instance IDs,
  bin metadata, fan, bin index) and builds an unpacking job. Its cost is views × batch sets plus the bytes
  of the per-view instance lists. Batch-set count goes with distinct pipelines, material bind groups
  (StandardMaterial is bindless in 0.20: `pbr_material.rs:25`; our non-bindless `ExtendedMaterial`s
  such as `GroundBlendMaterial` each split batches) and vertex/index slabs (default max slab is 512 MiB,
  so slabs are not the splitter).
- `queue_submit`: the `count=33` in the trace is the number of **command buffers in one
  `queue.submit`**. It is not 33 submits (`bevy_core_pipeline/src/schedule.rs:272`
  `submit_pending_command_buffers`). The cost inside wgpu is (1) the flush of every `queue.write_buffer`
  staged since the last submit (the [wgpu docs](https://wgpu.rs/doc/wgpu/struct.Queue.html): writes
  "begin execution only on the next call to submit"), so the bin, uniform and skin uploads above show up
  here, (2) wgpu-core resource tracking per command buffer, which scales with the bind groups and buffers
  each pass references, and (3) the driver. The levers are the same: fewer writes (retained data),
  fewer per-frame bind groups, fewer views. Check that wgpu validation is off (#7a).
- `collect_meshes_for_gpu_building` and extraction: retained since 0.15/0.16 (#16385 retained mesh
  instances, #16985 retained material instances, #17818 retained skins, #22857 retained lights, #22966 change
  lists for re-specialization and re-queue, #23662 MeshUniform allocation moved to the GPU). With CPU
  culling, each view's `VisibleEntities` list is still rebuilt in the main world every frame, copied to
  the render world and diffed into the bins. Walking changes those sets every frame, cascades most of all,
  because they move with the camera. That is our 25 ms walk against 14-19 ms standing.

**`NoCpuCulling` in 0.20 (the main lever).** `bevy_camera/src/visibility/mod.rs:60-69`, PR #23107
(merged 2026-03-21, in 0.20): meshes with `NoCpuCulling` are skipped by `check_visibility`,
`check_dir_light_mesh_visibility` and `check_point_light_mesh_visibility` (`bevy_light/src/lib.rs:340,515`:
"receive no view-specific processing in the main world"). Extraction iterates only **changed** ones
(`RenderMeshInstanceGpuQueue::GpuCulling { changed_gpu_culling, .. }`, `bevy_pbr/src/render/mesh.rs:1100`).
A global `RenderGpuCulledEntities` set feeds every view, filtered by `RenderLayers`
(`collect_gpu_culled_meshes_for_subview`). Cascades get `GpuPreprocessingMode::Culling` with their
own frustum (`render/light.rs:1947-2030`), so the GPU frustum-culls shadow casters per cascade.
`check_visibility_ranges` skips `NoCpuCulling` meshes (`range.rs:236`) because ranges are
evaluated in the preprocess shader. bevy_city uses `NoCpuCulling` and `VisibilityRange` LODs together.
#23530 (also in 0.20) stops extracting visible entities for lights without shadow maps, which is why our
200-400 unshadowed point lights cost nothing.

Caveats before flipping it on:
- `ViewVisibility` of a `NoCpuCulling` entity becomes just `InheritedVisibility` (`check_visibility_gpu_culling`,
  `mod.rs:878`). Any of our systems that reads `ViewVisibility` on statics as "on screen" breaks.
  Grep and fix those first. Actors keep CPU culling: they are dynamic and few.
- Leave **alpha-blended / sorted** meshes on CPU culling. Sorted phases are sorted on the CPU per view.
  The doc only promises "3D meshes" for the GPU path. **Verify** sorted-phase behaviour before tagging translucents.
- GPU cost rises: every tagged mesh is a preprocess thread per view, and an indirect draw slot per bin
  even when culled. We have ~3-4 ms of GPU headroom. Measure GPU time as well.
- Per #23107 the CPU still does "O(changed entities)" work. Streaming cells in and out is a change, so cell
  loads cost what they cost today. The per-frame churn of walking goes away.

**Other settings that exist in 0.20.** `NoFrustumCulling` (it draws in every view, shadows included: a
pessimisation for statics). `VisibilityRange` (we use it; GPU-evaluated under `NoCpuCulling`; its LOD origin for
shadow views is `ShadowLodOrigin`, `bevy_camera/src/camera.rs:837`). `RenderLayers` per view (prunes
the GPU-culled set per view; useful to keep tiny clutter out of cascades 2-3: a cheap per-cascade caster
rule with no per-entity CPU cost). `NotShadowCaster`. `CascadeShadowConfigBuilder { num_cascades,
maximum_distance, first_cascade_far_bound, overlap_proportion }`, `DirectionalLightShadowMap { size }`.
There is **no** built-in shadow caching or cascade update-rate control. Our `castercull` experiment fits a
linear cost per caster entry (8% of entries ≈ 8% of 4.8 ms ≈ 0.38 ms). The per-entry cost has to go
(#1), not just the entry count.

## Q2. Animation and skinning at scale

**What Bevy gives.** `bevy_animation::animate_targets` (`lib.rs:1098`) evaluates **every**
`AnimationTargetId` entity every frame, paused or not, and has no LOD. Bevy has no animation LOD crate
that I could find. We do not use `bevy_animation` anyway: `sf_anim_bevy` already implements tiers A0-A3
with staggered A2 updates, an off-screen drop to A3 and a UE-style budget allocator (`lod.rs`), and bone
write-back only on change. The gap is configuration: `budget_us: None` by default, so the allocator is
off. Shadow-casting off-screen actors should be forced to A2 (the doc comment says so; check the host does it).

**Industry mapping.** UE: Update Rate Optimisations (URO, frame skipping by screen size) plus the
[Animation Budget Allocator](https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-budget-allocator-in-unreal-engine)
(fixed ms budget, significance-ordered throttling, interpolation for skipped frames). Unity:
[`AnimatorCullingMode`](https://docs.unity3d.com/ScriptReference/AnimatorCullingMode.html)
(`CullUpdateTransforms`/`CullCompletely` for off-screen) and
[`SkinnedMeshRenderer.updateWhenOffscreen`](https://docs.unity3d.com/ScriptReference/SkinnedMeshRenderer-updateWhenOffscreen.html)
(default false: **fixed bounds** computed at import, attached to the root bone, not refit per frame).
We match UE on throttling. We do not match Unity on bounds.

**Skinned AABB (1.3 ms).** `DynamicSkinnedMeshBounds` → `update_skinned_mesh_bounds`
(`bevy_camera/src/visibility/mod.rs:596`) runs `par_iter` over **all** marked meshes **every frame**, with no
change filter. Per mesh it transforms one AABB per influencing joint. PR #21837 measured `many_foxes`
(1000 meshes × 22 joints) at 0.05-0.14 ms. Ours is ~10× that, so we have roughly 10× the (mesh × joint) pairs:
many body-part meshes per actor, each with its own joint list. Fixes, cheapest first:
(a) our own copy of the system restricted to actors whose pose was written this frame (A2 off-frames and A3 cost 0);
(b) one bound per actor instead of per part, i.e. the union written to all parts;
(c) Unity-style fixed bounds from the baked clip set plus margin, refit only for ragdolls.
#21837 itself lists cheaper follow-ups that were not merged (cull in skin extraction ~3× faster, folding
finger/face joints).

**Skins (1.2 ms render).** `extract_skins` (`bevy_pbr/src/render/skin.rs:274`) loops every `SkinnedMesh` and
copies joints whose `GlobalTransform` changed (#23283 already removed the full table scans). Each body
part is its own skin, so the same skeleton's matrices are copied and uploaded once per part. The standard
fix is the **skeletal mesh merge** for modular characters (UE `USkeletalMeshMerge`, Unity "combine skinned
meshes"): one mesh per actor per material group, one palette. Morrowind body parts have different
textures, so a full single-draw merge needs an atlas or texture array. A per-material-group merge still
removes most duplicate palettes. Animation throttling (#6) also cuts skin extraction, since unchanged joints are skipped.

**GPU skinning.** Bevy already skins in the vertex shader (joint palette in a storage buffer). There is no
compute-skinning path to switch to. The CPU cost is palettes and bounds, not vertex work.

## Q3. Transform propagation at scale

Bevy 0.16+ `TransformPlugin` uses static-scene optimisation: `mark_dirty_trees` sets
`TransformTreeChanged` up the ancestors of changed entities, and propagation skips clean subtrees.
`StaticTransformOptimizations::Enabled` is the default (`bevy_transform/src/systems.rs:80-95`;
bevy_city sets it explicitly). **big_space replaces `TransformPlugin`** (it panics if it is present,
`plugin.rs:134`). Its high-precision path has `Stationary` pruning, which our cell roots use. Its
**low-precision** path (`Grid::propagate_low_precision`, `grid/propagation.rs:318-470`) calls
`propagate_recursive` from every `LowPrecisionRoot` and visits every descendant every frame (it only
skips the matrix multiply when nothing changed). All ~50k static meshes under our `Stationary` cell
entities are low-precision descendants, so we pay a full tree walk per frame: the 2.5 ms of task time.
Upstream has no issue or PR for this (checked `aevyrie/big_space` 2026-10-19).

Fix (item 2): schedule Bevy's public `mark_dirty_trees` before `PropagateLowPrecision` (with big_space, `TransformTreeChanged` is
already a required component of `Transform`). In `propagate_recursive`, return early when
`!changed && !tree_changed.is_changed()`, as Bevy's `propagate_descendants_unchecked` does. When the
floating origin crosses a cell the parent `GlobalTransform` changes and the whole subtree recomputes
anyway. That case is rare and correct. Patch in our workspace (`[patch.crates-io]`), PR upstream
(MIT/Apache). Alternatives (`Stationary` + `CellCoord` on every mesh) move 50k entities onto the more
expensive high-precision path. Not better.

## Q4. Mesh count reduction

- **Bevy building blocks:** `Mesh::merge` (`bevy_mesh/src/mesh.rs:2208`), `transformed_by`/`transform_by`,
  `merge_duplicate_vertices`. There is no maintained Bevy HLOD or static-batching crate. Searches found only
  Godot and UE tools. Our far field already is the HLOD layer.
- **Precombined meshes (Bethesda FO4 / Skyrim-style)** cut entity count, which helps every per-entity cost
  that survives #1 (extraction of changes, bins, draws). With `NoCpuCulling`, static CPU cost per frame
  is already ~O(changed), so merging matters mostly for GPU draws and cell-load spikes. Do it at bake, per cell
  × material, for small clutter only. Keep large statics separate so GPU culling and occlusion keep their granularity.
- **Meshlets / virtual geometry (0.20):** still experimental (`MeshletPlugin`: Vulkan/Metal,
  `TEXTURE_INT64_ATOMIC`, `Msaa::Off`, opaque deferred or forward but not mixed, offline `MeshletMesh`
  conversion, limited materials, no skinning). It removes CPU batching for converted meshes, but alpha-test
  foliage, DLSS integration and our extended materials make it a rewrite, not a setting. Revisit in Phase 4 at the earliest.

## Q5. Big Bevy projects

- **Bevy `bevy_city`** (`examples/large_scenes/bevy_city`, v0.20.0): Bevy's own large-scene stress test.
  `--no-cpu-culling` (adds `NoCpuCulling` to every mesh after spawn), `StaticTransformOptimizations::Enabled`,
  `VisibilityRange` LODs propagated to every mesh of a scene, `Msaa::Off`, contact shadows, shadow
  toggles. It is the reference configuration for item 1, and the scene #23107 was measured on.
  Siblings: `bistro`, `caldera_hotel`.
- **`many_cubes`** (stress test): has `--no-cpu-culling` and GPU-culling switches. Useful to reproduce cost
  per view on our machine without game data. **`many_foxes`**: 1000 skinned foxes, the reference for
  #21837's bounds cost.
- **Mudcrab** (Skyrim-data viewer on Bevy **0.19**, MIT OR Apache-2.0, but the converter vendors a GPL NIF
  crate: read the engine crate only, copy nothing from the converter). Rendering is stock: `OcclusionCulling` on the
  camera, 4 cascades at 2048² with `maximum_distance` fitted to the streamed radius (`engine/src/app.rs:1853-1930`,
  and their doc comment names distance > cascade count > map size as the levers, in that order). No
  `NoCpuCulling`, no merging, no shadow caching, no animation (roadmap only, see
  `../animation/10-mudcrab-borrow.md`). Worth borrowing: `engine/src/render.rs:160-215`. It is a
  **startup proof that the GPU-driven path is really on** (`GpuPreprocessingSupport` = `Culling`, indirect
  drawing active, occlusion-culling views > 0, indirect batch sets > 0), exported as a metric. We should
  add the same gauge plus a **batch sets per view** and **views** count to `perf` reports. Idea only, no code copied.
- **Tiny Glade** uses Bevy's ECS with its own renderer (from the developers' public talks, not checked
  for this note), so it is not applicable to render-thread costs.
- Bevy's renderer tracking issue [#12590](https://github.com/bevyengine/bevy/issues/12590) records the
  history (binning, retained specialization, split prepare). pcwalton's 2025-2026 PR series is the
  "retained / O(changed)" programme we should ride rather than duplicate: #16385, #16898, #16985,
  #17413, #17818, #17915, #22857, #22966, #23107, #23283, #23662, #24984, #25058.

## Licences

Everything recommended is Bevy (MIT/Apache) or big_space (MIT OR Apache-2.0, a patch we would upstream).
Mudcrab: MIT OR Apache-2.0 at the top level; ideas only. No GPL source was read for this note.

## Open checks (cheap, before implementing)

1. What are the ~16 views? Log `ExtractedView`s per frame: main, prepass, 4 cascades, 6 cube faces, water
   reflection, light probes?
2. Does `--off lights` include the shadowed point light? If it did, the "no change" result argues against item 3's cube-face estimate.
3. Batch sets per view per phase (from `ViewBinnedRenderPhases`) to see whether materials split batches.
4. Is wgpu validation on in the `profiling` build (`InstanceFlags`, `WGPU_VALIDATION`)?
5. Who reads `ViewVisibility` on static meshes (blocker list for item 1)?
