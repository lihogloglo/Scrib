# Audit: the S3a `GpuBatchedMesh3d` port, S0 counters, S2 classes, and what S3b gets

Read-only audit, 2026-11-09, for the S3b dispatch. Paths are repo-relative. Nothing was compiled or run
(laptop build rule). Ranks: **BLOCKER** = S3b cannot be built on this as briefed / **SERIOUS** / **MINOR** /
**CONFIRMED-GOOD**. Claims without a `file:line` are marked "unverified".

## 0. Summary

The port is faithful and careful: both hunks the README admits needed hand work landed correctly, the
`Mesh3dVisibility` shim reproduces `bevy_light`'s rules, and the shared-path changes are sound. But the
port's data model is **one mesh asset per batch, one indirect draw per batch**, which is *not* the model
spec §6 and the S3b brief assume ("one batch entity per (space, material class)"). That is BLOCKER 1 and the
thing to settle before dispatch. Two more blockers are mechanical (shadow schedules, the S0 budget metric).

---

## A. The S3a vendored port

### A1. The API S3b actually gets — the producer contract

**Main world.** Add `GpuInstanceBatchPlugin` (`superfargoth/vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:154-250`;
not in `PbrPlugin`). Spawn, per batch:

- `GpuBatchedMesh3d { mesh: Handle<Mesh>, max_capacity: u32 }` (`mod.rs:96-102`)
- `MeshMaterial3d<M>` — **required in practice**: without a `RenderMaterialInstances` entry and a prepared
  bindless binding the batch silently draws nothing (`mod.rs:415-420`).
- **Never `Mesh3d`** (`warn_once` at `mod.rs:311-313`; `RenderMeshDraws::get` prefers the entity path,
  `bevy_pbr/src/render/mesh.rs:1373-1379`).
- Required components, auto-inserted: `Transform, NoFrustumCulling, Visibility, VisibilityClass`
  (`mod.rs:97`); an `on_add` hook pushes the `Mesh3d` visibility class (`mod.rs:98`).
- Do **not** add `NoCpuCulling` (`warn_once`, `mod.rs:314-316`; it also excludes the batch from the light
  shim, `mod.rs:521`). `max_capacity == 0` is skipped (`mod.rs:317-319`). Optional: `NotShadowCaster`,
  `NotShadowReceiver`, `TransmittedShadowReceiver`, `RenderLayers`, `Aabb` (used only as the Transparent3d
  sort center, `mod.rs:339-341`).

**Render world.** `GpuInstanceBatchReservations` (render-world `Resource`, `mod.rs:137-152`):
`get(MainEntity) -> Option<&GpuInstanceBatchReservation>`, `.output_buffer() -> &Buffer`,
`.max_capacity() -> u32` (`mod.rs:123-134`). The buffer is `STORAGE | COPY_SRC | COPY_DST`, sized
`max_capacity * 64 B` (`mod.rs:469-476`).

**Per-instance struct — the full contract** (`mod.rs:68-76`, WGSL mirror
`gpu_instance_batch/gpu_instance.wesl:3-8`; sizes asserted equal in
`superfargoth/crates/sf_world_bevy/src/gpu_batch_tests.rs:164-174`):

| off | size | field | semantics |
|---|---|---|---|
| 0 | 48 | `world_from_local: [Vec4; 3]` | affine 4x3 **transposed to 3x4**, the `MeshUniform` layout; world space (the producer adds the camera-relative cell offset itself). The sign of `det` of the first three lanes' `.xyz` is recomputed on GPU into `MESH_FLAGS_SIGN_DETERMINANT_MODEL_3X3` (`materialize.wesl:26-29`). |
| 48 | 4 | `is_active: u32` | 0 = slot skipped in preprocessing; copied verbatim into `MeshCullingData::is_active` (`materialize.wesl:32-34`, gate at `bevy_pbr/src/render/mesh_preprocess.wesl:190-194`). **A zeroed slot is inactive** (`gpu_batch_tests.rs:173`). |
| 52 | 4 | `tag: u32` | copied into `MeshInputUniform::tag` -> `MeshUniform::tag`, readable in shaders. Free for us (chain id, class, debug tint). |
| 56 | 8 | `pad: [u32; 2]` | must be written (the shader reads the whole struct); no meaning. |

Total **64 B**. There is **no index into anything else**: no chain index, no LOD index, no material-override
index, no per-instance bounds. Everything except the transform, `is_active` and `tag` is a *per-batch*
constant taken from the batch template (`MeshInputUniform::for_mesh`, `render/mesh.rs:680-705`:
`first_vertex_index`, `first_index_index`, `index_count`, `metadata_index`,
`material_and_lightmap_bind_group_slot`, `previous_input_index = u32::MAX`). `tag` is the only per-instance
channel the port gives us.

**System set.** The producer runs in the **`RenderGraph`** schedule (not `Render`), in
`GpuInstanceBatchSystems::Publish`; `GpuInstanceBatchSystems::Materialize` is chained after it, both between
`RenderGraphSystems::Begin` and `::Render` (`mod.rs:226-235`). So:

```rust
render_app.add_systems(RenderGraph, publish_scene_instances.in_set(GpuInstanceBatchSystems::Publish));
```

`Publish` is where `RenderContext` commands (a `write_buffer`, a compute pass) are recorded; they are
submitted before materialization (`mod.rs:80-85`).

**Allocation / rebinding / capacity change.**
- `prepare_gpu_batched_mesh_reservations` (`RenderSystems::PrepareMeshes`, `mod.rs:346-369`) creates missing
  reservations and `resize`s on a capacity change: new buffer, `min(old, new) * 64 B` copied with its **own
  immediate `queue.submit`**, handle replaced (`mod.rs:479-494`). Slot indices survive growth — a free-list
  allocator is safe. **CONFIRMED-GOOD.** `Buffer` identity changes, and `output_buffer()` is the only signal:
  a producer caching a bind group must compare `Buffer::id()` as the port's own job does
  (`materialize.rs:162-166`). Despawned batches are dropped by `retain` (`mod.rs:353-355`, `:388-390`).
- Every frame `prepare_gpu_batched_mesh_draws` (`mod.rs:371-444`) re-derives the batch's slice of the global
  mesh-input buffer: `scratch_end` starts at `max(input.len(), culling.len())`, each batch takes
  `max_capacity` slots (`mod.rs:405`, `:428-431`), then `input.reserve_gpu` / `culling.reserve_amortized`
  (`:441-442`). `batch.input_range` is left empty whenever the pipelines, the material, the binding or the
  mesh allocation are not ready (`:412-425`) — an empty range means "do not draw this frame"
  (`bevy_render/src/batching/mod.rs:198-204`, honoured at `gpu_preprocessing.rs:2581-2583`).
- Ordering is correct: PrepareMeshes (ranges) -> Queue -> PhaseSort -> PrepareResourcesBatchPhases
  (`batch_and_prepare`, `bevy_render/src/render_phase/mod.rs:1796-1807`) -> PrepareBindGroups -> Render;
  `RenderSystems` order at `bevy_render/src/lib.rs:335-365`. **CONFIRMED-GOOD.**

**Per frame, per batch, unconditionally** (`materialize.rs:135-213`): one `UniformBuffer::write_buffer` of
the metadata plus one `dispatch_workgroups(max_capacity/64)` with its own `set_bind_group`. Then
`unpack_ranges` expands the batch's one range entry into `max_capacity` `PreprocessWorkItem`s on the GPU
(`bevy_pbr/src/render/gpu_preprocess.rs:937-1004`), and mesh preprocessing culls them per view.

### A2. BLOCKER 1 — one mesh and one index range per batch

`GpuBatchedMesh3d` carries a single `Handle<Mesh>` (`mod.rs:100`). The template is built once per batch from
that asset (`mod.rs:421-425` -> `render/mesh.rs:683-704`), so `first_vertex_index`, `first_index_index` and
`index_count` are **batch constants**. On the GPU only the *first* work item of a draw writes the geometry
source, `indirect_parameters_metadata[idx].mesh_index = input_index`
(`render/mesh_preprocess.wesl:178-181`), and `build_indirect_params.wesl:134-144` reads
`first_vertex_index` / `index_count` / `first_index_index` from `current_input[mesh_index]` — one geometry
range for the whole indirect draw.

Consequences:

1. **The brief's "one batch entity per (space, material class)" cannot be built.** Instances of different
   chains in one batch would all draw the first chain's triangles. S3b must spawn one `GpuBatchedMesh3d`
   per **(chain mesh, material class)** — roughly one per distinct static model-part in the space
   (thousands in a Vvardenfell town; exact count unverified).
2. **Per-instance LOD index ranges (spec §4 step 5/7, §6) are structurally absent**, not merely
   unimplemented: they need one indirect command per (chain, level) group, i.e. many ranges per batch each
   with its own `mesh_index`. The port's `push_range` takes exactly one
   `(base_input_index, indirect_index, count)` triple
   (`bevy_render/src/batching/gpu_preprocessing.rs:783-828`).
3. Per-batch cost (A3) then multiplies by the number of chains, not by the number of classes.

**What S3b must do.** Either (a) accept one batch per (chain, class) for S3b, size capacities per chain, and
name the extension as the first S4 item; or (b) extend the port first so one batch emits N ranges, one per
(chain, level), sharing a batch set. (b) is the spec's §6 "what we add to #25978" and is the only route to
the §Budgets line "batch sets per view <= 2 x material classes". The coordinator should pick before dispatch.

### A3. SERIOUS — per-batch per-frame cost, and capacity not occupancy

- One `write_buffer` + one `set_bind_group` + one `dispatch_workgroups` per active batch per frame
  (`materialize.rs:161`, `:202-211`). With per-chain batches (A2) that is thousands of tiny dispatches and
  uniform writes per frame — the CPU/driver cost the whole design exists to remove.
- Each batch becomes **its own indirect batch set**: `prepare_instance_batch_indirect` allocates one entry
  and `add_batch_set(indexed, &view, index..index+1)` with `batch_set_index: None`
  (`gpu_preprocessing.rs:2253-2270`). Batches are therefore never merged into one
  `multi_draw_indirect_count` per class, however equal their batch-set keys are.
- Everything downstream is sized by **`max_capacity`**, not by how many instances survived the cull:
  `data_buffer.add_multiple(count_u32)` per view (`gpu_preprocessing.rs:2587`),
  `push_multiple_uninit(count)` work items (`:807`), and the input/culling reservations
  (`mod.rs:441-442`). Per view per frame that is `sum(capacity) x (sizeof(MeshUniform) + 8 B)`, plus
  `sum(capacity) x (112 + 32) B` once globally (`MeshInputUniform` = 112 B computed from its field list,
  `render/mesh.rs:612-677`; `MeshCullingData` = 32 B, `:766-777` — sizes computed, not asserted by a test).
  Over-provisioned capacity is paid for in full, in every view, every frame.

### A4. BLOCKER 2 — point/spot shadow views never unpack their range work items

`GpuInstanceBatchPlugin` registers `unpack_ranges` **only into `bevy_core_pipeline::Core3d`**
(`mod.rs:194-202`). #26149 moved point/spot shadow views into their own slim `Core3dShadows` schedule, whose
system list is `allocate_uniforms, unpack_bins, early_gpu_preprocess, ...` — no `unpack_ranges`
(`bevy_pbr/src/render/gpu_preprocess.rs:633-662`). `gather_shadow_cascades_for_view` (same file, just above
`late_gpu_preprocess`) filters to `LightEntity::Directional`, so the Core3d run never reaches lamp faces
either. Our `sf_shadow_cache_bevy` capture views run in yet another schedule
(`RootNonCameraView(LampShadowView.intern())`, `superfargoth/crates/sf_shadow_cache_bevy/src/render.rs:544`).

Meanwhile the light shim adds every batch to every face of every shadow-mapped point light and to every
visible shadow-mapped spot light (`mod.rs:598-627`), `queue_shadows` bins it (`render/light.rs:3480-3541`),
and `batch_and_prepare` reserves `max_capacity` **uninitialized** work-item slots for that view
(`gpu_preprocessing.rs:2596-2601` -> `:807`). `mesh_preprocess` then dispatches over
`arrayLength(&work_items)` (`mesh_preprocess.wesl:166`) and reads those slots as `input_index` /
`output_or_indirect_parameters_index` (`:171-180`) — uninitialized GPU memory, which can both corrupt the
lamp's shadow map and `atomicAdd` into *another* draw's `early_instance_count`.

**S3b mitigation (cheap):** give S3b's batches `NotShadowCaster` (the shim excludes them, `mod.rs:520`) —
consistent with "main view only". **Real fix (S5):** register `unpack_ranges` in `Core3dShadows` and in
`sf_shadow_cache_bevy`'s schedule, or move it into the shared preprocessing chain. Directional cascades are
fine today (they are gathered from `ViewLightEntities` in Core3d).

### A5. BLOCKER 3 — the S0 budget metric does not see instance batches

`BinnedRenderPhase::batch_set_count()` reads only `self.batch_sets`
(`bevy_render/src/render_phase/mod.rs:1539-1549`), while instance batches are drawn from
`instance_batch_draws` (`gpu_preprocessing.rs:2603-2612`) and register their batch sets directly on the
indirect-parameters buffers (`:2262`). So the one number the §Budgets table gates on ("batch sets per view
<= 2 x material classes") reports **zero extra draws** for all statics. `bin_count()` and `entity_count()`
*were* updated (`mod.rs:1500`, `:1535-1536`); `batch_set_count()` was not. Needs a vendored
`instance_batch_draw_count()` (the field is `pub(crate)`, `mod.rs:169`) plus one line in `sf_diag_bevy`.

### A6. What §6 / §7 need and the port does not have

| Need (spec §) | State | What S3b/S4+ must add |
|---|---|---|
| Our own cull as the producer (§4, §6) | **not in the port** — by design; the port only materializes slots a producer wrote. Bevy's own per-view GPU frustum + visibility-range + Hi-Z cull does run over the batch's instances (`mesh_preprocess.wesl:190-232`, `:260-...`), using the batch mesh's AABB per instance. | the Publish producer (CPU twin `sf_scene::cull` for S3b, compute pass later); layer / active / horizon / sub-pixel tests, which Bevy's cull has no notion of. |
| Per-instance LOD index ranges (§4.5, §6) | **not possible as ported** (A2) | many ranges per batch, each with its own `mesh_index`; group work items by (chain, level). Vendored change. |
| Cell-range upload/free (§3, §6) | **not in the port** | all of it: the port has no notion of cells or sub-ranges. `resize` preserves slot indices (`mod.rs:479-494`), so a `sf_scene::alloc` free-list over the output buffer works. |
| Per-shadow-view culling (§8) | **partial** — Bevy's GPU frustum cull runs per shadow view *if* the view is in a schedule that unpacks the ranges (A4). Cascade LOD (orthographic form) absent. | fix A4; add the cascade / cube-face variants of our cull and the orthographic LOD rule. |
| Motion-vector history (§7) | **better than the handoff claims.** `previous_input_index = u32::MAX` (`render/mesh.rs:699`) makes `mesh_preprocess` use the current transform as the previous one (`mesh_preprocess.wesl:243-250`), so rigid statics get correct **camera-motion** vectors today — exactly §7's rule. Missing: wind / UV-scroll history and a flags field for history bits. | S4: the sway / scroll double evaluation in the prepass. No change needed for rigid statics. |
| `BinnedRenderPhase::entity_count` for #26161 (§6) | **present but blind.** `entity_count()` counts one entity per batch (`mod.rs:1498-1521`), so the hash at `render/light.rs:1509-1520` changes when a batch appears or disappears but **never when its GPU contents change**. | S5: feed a per-batch generation counter into `ShadowCasterChanges::faces`. Also `has_casters = phase_entity_count > 0` (`light.rs:1559`) becomes permanently true on every face of every shadow-mapped lamp, so SH-20b's "a moved light redraws whole" now always means all six faces — measure before/after. |

### A7. Is the `Mesh3dVisibility` deviation sound? — **CONFIRMED-GOOD**

`add_gpu_batched_meshes_to_light_visibility` (`mod.rs:497-630`) runs
`.after(SimulationLightSystems::CheckLightVisibility).before(VisibilitySystems::MarkNewlyHiddenEntitiesInvisible)`
(`mod.rs:163-168`). Compared field by field with `bevy_light-0.20.0/src/lib.rs` (registry copy;
`check_dir_light_mesh_visibility` `:342-510`, `check_point_light_mesh_visibility` `:517-...`):

- shadow maps off / light not view-visible -> skipped (`mod.rs:565`) vs `bevy_light:401-404`, `:584`, `:675`
  (`bevy_light` *clears* the directional lists there; the shim correctly adds nothing).
- render layers (`mod.rs:572`, `:603`, `:619`) = `bevy_light:436-439`, `:610-613`, `:702-705`;
  `InheritedVisibility` (`mod.rs:571`) = `bevy_light:432`; `NotShadowCaster` / `NoCpuCulling` as query
  filters (`mod.rs:519-521`) = `bevy_light:545-547`.
- `VisibilityRange`: the shim's `in_range` (`mod.rs:552-557`) is logically identical to `bevy_light:442-448`
  for the directional case (per cascade view) and to `bevy_light:614-623` / `:707-716` for point/spot,
  **including** the `shadow_lod_origin == None` -> cull branch; `get_shadow_lod_origin` is the same helper
  (`mod.rs:587-591`).
- frustum tests are correctly skipped: a `NoFrustumCulling` caster goes into every cascade and cube face and
  the light-sphere test is bypassed (`bevy_light:457`, `:629`, `:639-640`); the shim inserts into every list
  (`mod.rs:577-579`, `:608-610`). `insert_sorted` (`mod.rs:558-562`) matches `bevy_light`'s `sort_unstable()`
  (`:493`, `:666`), which `DirtySpecializations::iter_to_queue` requires.
- `ViewVisibility::set_visible` is additive and `MarkNewlyHiddenEntitiesInvisible` runs after; the main view
  still sees the batch through the pushed `Mesh3d` class. Tested headless at
  `sf_world_bevy/src/gpu_batch_tests.rs:211-235` (sun only; point/spot untested, as the handoff says).

Also sound: `queue_shadows` reads `visible_entities.get::<Mesh3d>()` (`render/light.rs:3455`), and for a GPU
batch `RenderMeshDraw::flags()` returns `SHADOW_CASTER` and `matches_layers()` returns `true`
(`render/mesh.rs:1439-1455`) *because* the shim already applied both rules — consistent. The sweeps cover
the batch: `late_sweep_material_instances` (`material.rs:834-856`),
`check_entities_needing_specialization` (`:937-986`),
`mark_meshes_as_changed_if_their_materials_changed` (`:743-764`); `extract_mesh_materials` never filtered on
`Mesh3d` (`:768-794`). Nothing is left behind by dropping the 14 rename-only hunks.

**Zero surviving instances.** Nothing breaks visually: `get_instance_batch` returns the full (non-empty)
range, `mesh_preprocess` culls every instance, `early_instance_count` stays 0 and the draw is a 0-instance
indirect draw. Costs are paid in full (A3). Two side effects: the phase is **not** `is_empty()`
(`mod.rs:1487-1494`), so phase-empty early-outs do not fire; and `has_casters` stays true (A6, last row).

### A8. Interaction with our other vendored patches

- **#26149 / #26161** — the real interactions are A4 (shadow schedule) and A6/A7 (hash blindness,
  `has_casters`). `batch_and_prepare_binned_render_phase` keeps the `Without<SkipRootViewRendering>` filter
  (`gpu_preprocessing.rs:2297-2299`), so a skipped face allocates nothing. **Correct.**
- **S0 phase counters** — A5 (`batch_set_count`), plus the `pipelines` counter in
  `sf_diag_bevy/src/views.rs:122-128`, which chains four bin maps and omits the `pub` `instance_batches`
  (`mod.rs:153`): a pipeline used only by statics vanishes from `views/*/pipelines`. SERIOUS, one-line fix.
  The README's claim that the `is_empty` hunk "fuzzed into our S0 `batch_set_count` and was moved back" is
  **verified correct**: `is_empty` has the `instance_batches` clause (`mod.rs:1492-1493`) and
  `batch_set_count` is untouched our-patch code (`:1539-1549`). CONFIRMED-GOOD (A5 is a design gap, not a
  bad merge).
- **#26095** — clears `SceneUnpackingBuffers` jobs and `BinUnpackingBindGroups` /
  `UniformAllocationBindGroups` each frame. The port's range bind groups live elsewhere
  (`RangeWorkItemBuffer::bind_group`) and are rebuilt unconditionally every frame when non-empty, `None`
  when empty (`gpu_preprocess.rs:904-931`): no staleness, but no caching either, and
  `PreprocessBindGroupCount` never sees them (MINOR; the materialize bind groups at `materialize.rs:167`
  are likewise uncounted — SERIOUS for the 2.3 ms budget line S3b is meant to move).
- **CONFIRMED-GOOD, no shared code path:** sun shadow early-out / lamp filter / H2 / RO-11 (all WGSL in
  `pbr_functions.wesl`, `shadow_sampling.wesl`, `ssr*`); the DLSS viewport patches (#26235/6/7, #26234),
  #26170 and `SharedImageSamplers` (`clustered_forward.wesl`, `transmission.wesl`, `oit/*`, `ssao`,
  `occlusion_culling.wesl`, `texture/gpu_image.rs` — the port in fact *benefits* from #26170 on the shared
  Hi-Z path); and incremental `extract_visibility_ranges` (the shim reads `VisibleEntityRanges` read-only
  and the batch has no `VisibilityRange`).

### A9. Correctness bugs in the ported code

- **SERIOUS** — `no_indirect_drawing` views **silently** draw no batches: `continue` at
  `gpu_preprocessing.rs:2574-2576`, and `push_range` returns `None` in `Direct` mode (`:790-799`). No warning
  anywhere. That covers adapters limited to `GpuPreprocessingMode::PreprocessingOnly` and our inherited
  shadow views (`sf_sky_bevy/src/sun_view.rs:255-257`, `sf_shadow_cache_bevy/src/render.rs:546-548`).
  Spec §Risks already names the `multi_draw_indirect_count` fallback; this is the *no indirect at all* case
  and needs a `warn_once` plus a documented "statics require GPU culling" hardware floor.
- **MINOR** — `sort_binned_render_phase` does not sort `instance_batches` keys
  (`bevy_render/src/batching/mod.rs:218-223`): draw order inside a frame is `IndexMap` insertion order, so no
  front-to-back ordering and no determinism guarantee.
- **MINOR** — the `is_active` gate is nested inside `@if(FRUSTUM_CULLING)`
  (`mesh_preprocess.wesl:190-194`): a view that preprocesses without frustum culling would draw inactive
  slots. Unreachable today only because such views are `no_indirect_drawing` and drop instance batches
  entirely (previous item); the two guards are independent and should not be.
- **MINOR** — `extract_gpu_batched_meshes` computes `world_center` from the batch *mesh's* AABB center
  (`mod.rs:339-341`) and `refresh_gpu_batched_mesh_sort_centers` feeds it to `Transparent3d` sorting
  (`mod.rs:446-464`). Meaningless for scattered instances; harmless for S3b (opaque). Blended statics can
  never be batched correctly, which matches spec §1 ("blended parts stay entities").
- **MINOR** — `render_range` now takes `index += 1` in both arms of its `if/else`
  (`bevy_render/src/render_phase/mod.rs:2152-2164`); the branch is dead. Semantics are correct because merged
  sorted items get an empty `batch_range` (`batching/mod.rs:254-255`) — but this is an **always-on** change to
  every sorted phase, which is why the handoff asks the owner to re-check transparent sorting.
- **CONFIRMED-GOOD** — the second admitted hand-worked hunk: `prepass/mod.rs:3-16` keeps the whole SW import
  list, adds `RenderMeshDraws`, and no stale `RenderMeshInstances` reference remains in that file (grep: one
  hit, the comment at `:13`).
- **MINOR** — `prepare_gpu_batched_mesh_reservations` and `prepare_gpu_batched_mesh_draws` are in a tuple
  without `.chain()` (`mod.rs:206-219`): no ordering between them. Harmless today (draws does not read
  reservations) but fragile.

---

## B. S0 counters and S2 material classes

### B1. S0 counters

Collected by one render-world system, `count_views`, in `Render` / `RenderSystems::Render` after
`render_system` (`superfargoth/crates/sf_diag_bevy/src/views.rs:68`), registered by `RenderViewsPlugin`
(`:52-69`), enabled for benches only (`bevyrrowind/crates/mw_devtools_bevy/src/perf.rs:173`; feature `views`
at `sf_diag_bevy/Cargo.toml:14-15`). Per-view: `PhaseCounts { phase, items, bins, batch_sets, pipelines }`
(`sf_diag/src/views.rs:69-81`), `ViewCounts { label, kind, phases, work_items, pipelines }` (`:84-94`),
`ViewFrame { views, distinct_pipelines, bind_groups }` (`:122-130`). Needs vs coverage today, on the entity
path: instances ok (as `items`), work items ok (`sf_diag_bevy/src/views.rs:142-156`), batch sets ok, pipeline
states ok, bind groups **partial** (preprocess only — which is the budget line the spec names). The perf
recipe's names and units match the code exactly (`bevyrrowind/docs/perf.md` "Static scene baseline (S0)" vs
`sf_diag/src/views.rs:197-235`, `bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:375-378`) — **CONFIRMED-GOOD**.
Sorted phases (`Transparent3d`) have no counters at all (`sf_diag_bevy/src/views.rs:95-104`) — MINOR.

Once statics come from instance batches:

- `items` — **SERIOUS**, under-reports by orders of magnitude: one per batch (`render_phase/mod.rs:1500`;
  `RenderInstanceBatchBin = IndexSet<MainEntity>` at `:195`). A real instance count is needed; the CPU knows
  only `max_capacity` (`gpu_instance_batch/mod.rs:428-438`), the culled count exists only on the GPU.
- `batch_sets` — **BLOCKER**, see A5. `pipelines` / `pipelines_distinct` — **SERIOUS**, `instance_batches`
  omitted from the chain (`sf_diag_bevy/src/views.rs:122-128`).
- `work_items` — **changes meaning**: reports reserved slots (= sum of capacities), not surviving instances
  (`gpu_preprocessing.rs:2586-2600`, `:807`). `preprocess/bind_groups` — **under-reports**: the batch path's
  bind groups are created outside `prepare_preprocess_bind_groups` (`materialize.rs:167`,
  `gpu_preprocess.rs:923`) and never counted.
- `bins` — stays right (`render_phase/mod.rs:1535-1536`). MINOR, already live: a `SkipRootViewRendering`
  face reports `items>0, bins>0, batch_sets=0, work_items=0` (`gpu_preprocessing.rs:2297-2299`) — correct but
  easy to misread. MINOR: `main` is "lowest-order non-light view" (`sf_diag_bevy/src/views.rs:158-169`); a
  future negative-order 3D camera would steal the label.

### B2. S2 material classes

`MaterialClass` is one `#[repr(u16)]` type in `sf_scene/src/material.rs:15-24`, bit-packed
`two_sided | masked<<1 | wind<<2` (`:53`), 8 variants; `sf_world_bevy/src/matclass.rs:58` imports it and the
bake uses the same import (`sf_scene/src/bake.rs:56`, `bevyrrowind/crates/mw_bake/src/scene.rs:31`).
**Bake-time and runtime therefore agree by construction — same type, same discriminants, no conversion
function. CONFIRMED-GOOD** (round trip tested, `material.rs:76-98`; on-disk value validated at parse,
`sf_scene/src/table.rs:361-362`). Only 4 variants are reachable: `wind` is hard-coded false at both ends
(`matclass.rs:100`, `sf_scene/src/bake.rs:171`, `mw_bake/src/scene.rs:106`).

- **SERIOUS, spec wrong not code** — §5 names the third and fourth axes as "+/- GI `1<<26`, +/- surface
  weather `1<<27`" (`superfargoth/docs/specs/static-scene.md:186-188`). Those are **view** pipeline-key bits
  (`vendor/bevy_pbr/src/render/mesh.rs:485-493`, `:3358-3359`), not material state, so they cannot be a
  material-class axis; the code's third axis is two-sidedness, which *is* material state
  (`StandardMaterialKey::CULL_BACK`, `vendor/bevy_pbr/src/pbr_material.rs:1255-1256`). Fix §5 before S3b
  keys batches off classes, or someone will build a 32-slot key.
- **SERIOUS** — bake and runtime read **different inputs**. `mw_bake::scene_material` reads the cooked NIF
  material (`mw_bake/src/scene.rs:96-108`); `traits_of` reads the `StandardMaterial`
  (`matclass.rs:89-101`). In between, `mw_assets/src/cooked.rs:134-150` promotes `SfAlpha::Blend` +
  cutout-candidate to `AlphaMode::Mask`. The bake sees only `m.alpha`, bakes such a part `Blended`, and
  `sf_scene/src/eligibility.rs:41-45` then **excludes it from the table** — i.e. most alpha-masked foliage
  would silently not be drawn under `--scene`. Either apply the same promotion in `scene_material` (it
  already has `extras::BLEND` in hand, `scene.rs:98-99`) or S3b loses foliage. Related MINOR asymmetries:
  `no_depth` -> `Blended` at bake with no runtime counterpart (`scene.rs:94,100`); `Face::Front` -> `None` at
  runtime with no bake counterpart (`matclass.rs:89-101` vs `scene.rs:106`); `--off leafmask` /
  `--off foliagetransmission` change the runtime class after the table is baked and are not in the cache key.
- **CONFIRMED-GOOD** — the folding itself. `fold` (`matclass.rs:142-194`) canonicalises only key bits that
  select nothing (UV channel when the texture is absent `:146-155`, parallax method when `depth_map` is
  `None` `:156-158`, sub-unit depth bias that Bevy truncates to 0 `:159-165`) and reports everything else as
  a `Split` (`:167-192`). Leaf diffuse transmission is deliberately left split with its reason recorded
  (`:26-29`). Nothing look-affecting is collapsed.
- **MINOR** — the template material cache keys on the **unfolded** description
  (`sf_world_bevy/src/stream.rs:1444-1452`), so `statics/materials` will not drop with matclass on; only
  keys / pipelines / bind groups do.
- **MINOR** — the transmission / anisotropy / specular / clearcoat UV key bits are neither folded nor
  reported, being behind Bevy features nothing enables (root `Cargo.toml:76`); turning one on would silently
  split a class. **CONFIRMED-GOOD** — `SharedImageSamplers` is inserted in `WorldStreamingPlugin::finish`
  under `if self.config.material_classes` (`sf_world_bevy/src/lib.rs:144-147`), and the flag is set from
  `--off matclass` *before* the plugin is added (`mw_exterior_bevy/src/exterior.rs:269`, `:283`).

---

## C. Facts the S3b brief does not state

Real signatures, verified. The brief's §"S3b, the next slice" is `docs/briefs/static-scene-s3b.md:45-63`
(note: `docs/briefs/`, not `superfargoth/docs/briefs/`).

**`sf_scene::alloc`** (`superfargoth/crates/sf_scene/src/alloc.rs`): `SlotRange { start, len }` `:13`, `end()` `:19`; `AllocError::{ZeroLength, Overflow, NotAllocated(SlotRange)}` `:25-32`; `SlotStats` `:36-49`; `SlotAllocator::new(frames_in_flight: u32)` `:65`, `alloc(&mut self, len: u32) -> Result<SlotRange, AllocError>` `:76`, `release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError>` `:107`, `advance(&mut self, frame: u64)` `:117`, `live_range(&self, start: u32) -> Option<SlotRange>` `:146`, `stats()` `:150`. **No exhaustion error** — first fit over the coalesced free list, else the table grows at `high_water`, which *is* the GPU buffer size the Bevy side must provide (`alloc.rs:1-7,75-104`). Free is by exact range (`:218-220`). Deferred free is two-step: `release(range, frame)` then `advance(frame)` frees `at + frames_in_flight <= frame` (`:117-130`). No compaction.

**`chain_layout`** lives in `sf_scene/src/chain.rs` (not `chain_layout.rs`): `pub fn chain_layout(levels: &[LevelSource]) -> Result<ChainLayout, LayoutError>` `:65`, with `LevelSource { error: f32, submeshes: Vec<(u32, u32)> }` `:20`, `Segment { level, submesh, vertex_base, index_base, vertex_count, index_count }` `:28`, `ChainLayout { levels, segments, vertex_count, index_count }` `:43`, `LayoutError::{LevelCount, EmptyFinest, Error, TooLarge}` `:53`. A level missing the part repeats the next finer level's range and error (`:78-82`). `MAX_LOD_LEVELS = 16`.

**`StaticCellTableView<'a>`** (`sf_scene/src/table.rs:289`, `Clone + Copy`): `parse(bytes) -> Result<Self, TableError>` `:302` is the only reader entry point — it validates everything and never panics (`:199-229`). `origin_cell() -> IVec3` `:390`, `aabb() -> Aabb` `:394`, `land_min() -> f32` `:398`, `layers() -> u32` `:403`, `len()` `:407`, `is_empty()` `:411`, `instance_bytes() -> &'a [u8]` `:416`, `write_remapped(&self, slots: &[u32], out: &mut Vec<u8>)` `:425`, `instance(usize)` `:438`, `instances()` `:443`, `find(PlacementId)` `:450`, `find_all(PlacementId)` `:464`, `chain_count()` `:470`, `chain_index(mesh: u64) -> Option<u32>` `:476`, `levels(u32)` `:490`, `material_class(u32)` `:496`, `chain(u32)` `:501`, `chains()` `:506`, `to_table()` `:511`. There is no header accessor struct. `write_remapped` **appends** to `out` and **asserts** `slots.len() == chain_count()`, copying each 80-byte record verbatim and overwriting 4 bytes at offset `INSTANCE_CHAIN = 68` (`table.rs:67`). `chain_index` is the inverse direction (mesh key -> cell-local index), so build `slots` by iterating `chains()` and looking each `mesh` key up in your own global map.

**Byte layout** (module doc `table.rs:29-44`). Instance 80 B: transform `f32x12`@0, sphere@48, placement@64,
chain@68, material_override@72, flags@76. Chain 32 B: first level@0, level count@4, class `u16`@8,
reserved@10/12, mesh key `u64`@16, material key `u64`@24. Level 12 B. Header 80 B.
`MAGIC = *b"SFSCENE\0"` `:56`, `FORMAT_VERSION = 2` `:58`. Decode mirrors (`StaticInstance` `:113-126`,
`LodChain` `:169-180`, `LodLevel` `:159`, `Sphere` `:106`, `Aabb` `:91`) are **not** `repr(C)`; the bytes are
the contract. `StaticInstance::affine()` `:130`, `rows()` `:143`, `max_scale()` `:151`. The transform rows
are already Bevy's transposed layout, so they copy straight into `GpuMeshInstance::world_from_local` — but
the 80-byte table record is **not** the 64-byte `GpuMeshInstance` slot: `write_remapped`'s output is the
*scene* instance buffer, and the producer must translate scene -> batch slots.

**No file or mmap helper exists** and nothing in the repo reads `.sfscene` yet. The path is the `baked://`
asset source: `sf_model_bevy::baked_path(step, subject, ext)` (`sf_model_bevy/src/source.rs:115`) with
`cell_subject` / `parse_cell_subject`, `CELLS_STEP = "scene.cells"`, `EXTENSION = "sfscene"`
(`sf_scene/src/bake.rs:60-70`, **behind `feature = "bake"`**, `sf_scene/src/lib.rs:24-25` — a runtime crate
needing only those four items must enable the feature or re-declare them; worth moving them). Register the
extension in the map at `mw_exterior_bevy/src/exterior.rs:205-212`, copying
`sf_lod_bevy::baked_extensions()` (`sf_lod_bevy/src/lib.rs:129-131`); `AssetLoader` precedent
`sf_lod_bevy/src/assets.rs:41-66`. The view borrows its bytes, so the asset must own the `Vec<u8>`.

**CPU twin for the Publish producer**:
`cull_instance(view: &SceneView, instance: &StaticInstance, offset: Vec3, errors: impl IntoIterator<Item = f32>) -> Verdict`
(`sf_scene/src/cull.rs:207`) and
`cull_cell(view, cell: u32, table: &StaticCellTableView<'_>, offset: Vec3, horizon: Option<&HorizonGrid>, promotions: &Promotions, out: &mut Vec<WorkItem>) -> CullStats`
(`cull.rs:231-239`); `WorkItem` `:167-178` (`slot = cell range start + instance`);
`sf_scene::cell_offset(origin_cell: IVec3, frame: sf_coords::WorldPos) -> Vec3` (`lib.rs:52`).

**Streaming hand-off points** (no `sf_stream*` crate; it is
`superfargoth/crates/sf_world_bevy/src/stream.rs`): `drive_streaming(world, anchors, roots)` `:466`,
exclusive, registered `Update, StreamingSystems::Drive` at `sf_world_bevy/src/lib.rs:112` (sets at
`lib.rs:67-74`, chained `:111`). `Streaming::run_stage` `:802-818` switches on
`Stage::{Spawn, Upload, Activate, Deactivate, Despawn}` — `Stage::Upload` is the one budgeted in **bytes**
(`request.upload_bytes`, `:816`), the rest in ms. Messages (Bevy 0.20 `Message`):
`CellLoaded { worldspace, cell, level }` `:198`, `CellUnloaded { ... }` `:206`, written at `:820-831`,
registered at `lib.rs:104-105`; component `StreamCell { worldspace, cell }` `:191`. **The right hook shape is
not the messages** but the `CellProxies` trait (`sf_world_bevy/src/far.rs:33-57`) behind `FarProxies`
(`far.rs:61-66`, `lib.rs:93`): documented call order `acquire -> ready -> spawn -> release`
(`far.rs:30-32`), sites at `stream.rs:1195-1207` (spawn), `:1349-1353` (release), `:787-798` (byte
estimate). Add a sibling hook so uploads share the in-flight caps and budgets. The per-entity path being
replaced: `spawn_template_ranges(world, cx, template, ranges) -> u32`
(`sf_world_bevy/src/spawn.rs:300`; reached via `StaticSpawner::spawn` `:280-283` <- `spawn_item`
`stream.rs:1255`; precombine path `stream.rs:1176`). **There is no frames-in-flight deferral in the
streaming layer** — `despawn_item` `:1311-1355` despawns immediately; the only such mechanism is
`SlotAllocator`'s, so `sf_scene_bevy` owns the frame counter and calls `advance` itself. (Neighbouring
knobs: `MAX_GUARDED_FRAMES = 6` `:53`, `SHIFT_COOLDOWN_FRAMES = 2` `:57`.)

**Switch plumbing — copy `bevyrrowind/crates/mw_devtools_bevy/src/perf.rs`.** CLI arg
`#[arg(long, value_delimiter = ',')] pub off: Vec<String>` `:48-49`; valid names `Part` +
`const ALL: [(&str, Part); 54]` at `bevyrrowind/crates/mw_game_core/src/perf.rs:117-173`
(`("matclass", Part::MatClass)` `:161` — **adding a name means bumping the 54**);
`Off::from_flags(&[String]) -> Result<Self, String>` `:186`, `has(Part)` `:211`;
`Perf { off, resolution, bench }` `perf.rs:79-95`; inserted as a resource in `Perf::add_plugins` `:191`.
Gating chain for `--off matclass`: `mw_exterior_bevy/src/exterior.rs:269` ->
`StreamingConfig::material_classes` (`sf_world_bevy/src/stream.rs:102`, default true `:117`) ->
`sf_world_bevy/src/lib.rs:144-147`. Resolve the decision into the plugin's config **before** `add_plugins`
(`exterior.rs:283`), as the exterior does, not from `Res<Off>` in a system. For "scene-eligible placements
are not spawned as entities", copy `OffSpawner` + `apply_off(spawners, off)`
(`mw_devtools_bevy/src/perf.rs:237-276`): it already wraps every `PlacementSpawner` to return 0, exactly the
shape needed, with no `sf_world_bevy` change. `--off` is a removal list while `--scene` is an opt-in flag;
the brief uses both spellings (`docs/briefs/static-scene-s3b.md:56`) and the owner should pick.

**`bake_scene` already exists**:
`pub fn bake_scene(baker: &mut Baker, db: &RecordDb, worldspaces: &[WorldspaceId]) -> BakeReport`
(`bevyrrowind/crates/mw_bake/src/scene.rs:194`; exported `mw_bake/src/lib.rs:31`), the same shape as
`bake_far` (`mw_bake/src/far.rs:169`). Insert at **line 65** of
`bevyrrowind/crates/mw_boot_bevy/src/bake.rs`, after `bake_far` and before `bake_actors`:

```
 62	    let exterior = world::bake_exterior(&mut baker, &db, terrain, layers, &paths);
 63	    // Far field (far-field.md): L1-L3 proxies and impostors over the exterior.
 64	    let far = mw_bake::far::bake_far(&mut baker, &db, &[world::exterior()]);
 65	    // Animation (bake-adapter.md "Animation"): skeletons, clips and graphs of every actor.
 66	    let anim = bake_actors(&mut baker, &db, &vfs, &anim_sources);
```

Ordering is load-bearing: `bake_scene` reads `world.page` and level-0 `terrain.tile` products
(`mw_bake/src/scene.rs:180-190`), so it must follow `bake_exterior` (`:62`) and precede `baker.manifest()`
(`:74`) and `store.save_manifest` (`:77`). Mirror `far`'s two report lines (`bake.rs:115`, `:117-120`); the
generic per-step line `:102-111` picks `scene.cells` up automatically. Step-version plumbing needs nothing
beyond `register(baker)`: `SceneBake::register` (`sf_scene/src/bake.rs:458-460`) and `impl Step for
CellsStep` with `id()` `:499`, `version() -> 2` `:502`, `plan()` `:505`, which already folds
`FORMAT_VERSION` in as the `"table"` param (`:510`), so format and step version are keyed separately.
`SceneBake::bake_worldspaces(baker, worldspaces)` `:474`.

**Crate template**: `superfargoth/crates/sf_lod_bevy` (same role — a Bevy plugin over per-cell baked
products, implements the streaming hook, loads via `baked://`; copy-paste usage block in its module doc,
`sf_lod_bevy/src/lib.rs:1-16`). Minimal skeleton: `sf_render_order_bevy/src/lib.rs` (119 lines).
Render-world GPU buffer precedents: smallest `sf_world_bevy/src/origin.rs:95-140`; closest to a retained
storage buffer `sf_gi_bevy/src/render.rs:394-505` (`prepare` at `:430`, `.in_set(
RenderSystems::PrepareBindGroups)` at `:83`). Headless-test template:
`sf_world_bevy/src/gpu_batch_tests.rs:1-16`, `:203`.
