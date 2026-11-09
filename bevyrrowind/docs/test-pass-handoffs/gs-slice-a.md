# Patch GS slice A: what only a GPU can prove

Agent session 2026-11-09, static-scene.md S3a2, `superfargoth/docs/specs/gs-patch-plan.md` steps 1, 3 and 4
(the plan's own two-slice split, §3 "S3a2 size"). Branch `static-scene/gs-slice-a`. Step 2 of the plan (N
geometry ranges per batch) is a separate slice and is **not** in this one.

Nothing in the game spawns a `GpuBatchedMesh3d` yet (S3b does), so there is **no visual check here** and no
`visual-checks.md` row: a visual pass would show exactly today's picture. What follows is for the first run
that does spawn one.

## Checked here, without a GPU
- `sf_diag` (Bevy-free): the instance-batch counters reach the per-kind and per-phase sums, `batch_sets`
  keeps counting them, the gauge names and `PhaseCounts::values` stay in lockstep, the bind-group parts sum
  saturating, and both `record` paths write every new gauge (0 without statics).
- `sf_world_bevy` `gpu_batch_tests.rs`: the port's WGSL still links (`wesl`) and validates (naga) for every
  def set the preprocessing pipelines specialise - including the `FRUSTUM_CULLING`-off set, which is what
  checks the un-nested `is_active` gate compiles away as before; a binned phase bins an instance batch and
  the three new counters read 0 before batching and reset with `prepare_for_new_frame`.

## Needs a GPU (nothing here is verified)
1. **Lamp shadow maps are no longer corrupt** (step 1, the real fix). Spawn one `GpuBatchedMesh3d` without
   `NotShadowCaster` inside a shadow-mapped point light's range at night, then look at the lamp's shadows.
   Before this slice the point/spot schedule (`Core3dShadows`) and `sf_shadow_cache_bevy`'s `LampShadowView`
   never ran the range unpacking, so `mesh_preprocess` read the reserved work-item tail as uninitialised GPU
   memory: expect garbage shadow geometry and, worse, `atomicAdd` corruption bleeding into *other* draws'
   instance counts (random missing or duplicated meshes anywhere in the frame). After it, lamp shadows
   should be as correct as the sun's. A `--validate`/wgpu-validation run is worth doing at the same time.
2. **The sun cascades are unchanged.** Directional cascades already unpacked (they are gathered from
   `ViewLightEntities` in `Core3d`); the dispatch moved into `early_gpu_preprocess` but keeps running after
   `unpack_bins` in all three schedules, so sun shadows must look identical.
3. **The two compute passes are correctly ordered.** The range unpacking is a separate compute pass right
   before `early_mesh_preprocessing` and relies on `wgpu`'s automatic write-read barrier between passes
   (the same guarantee `unpack_bins` already relies on). Unverified by construction: a driver that got this
   wrong would show the same garbage as (1). Visible in a capture as `range_unpacking` then
   `early_mesh_preprocessing` per schedule.
4. **The counters' non-zero side** (step 3 / plan test T7). With statics spawned, `views/*/instance_batches`
   must equal the number of batch entities drawn in the view, `instance_draws` the same number while a batch
   carries one geometry range, `instance_slots` the reserved capacity, and `views/*/batch_sets` must rise by
   `instance_batches` against an `--off scene` run. `instance_batch_draws` is only ever filled by
   `batch_and_prepare_binned_render_phase`, which allocates in a `RenderDevice` buffer, so no headless test
   can reach it.
5. **The new bind-group gauges.** `preprocess/bind_groups_ranges` should be non-zero every frame statics are
   drawn (those bind groups are rebuilt unconditionally) and `preprocess/bind_groups_materialize` should be
   non-zero only on a batch's first frame and after a capacity change (it is cached on the bound buffer ids).
6. **Hardware floor.** On an adapter limited to `GpuPreprocessingMode::PreprocessingOnly`, or with a camera
   carrying `NoIndirectDrawing`, the statics must be **absent** and `prepare_gpu_batched_mesh_reservations`
   must log its `warn_once`. Not a fallback: there is no un-culled path. The laptop's DX12 backend supports
   indirect but not `multi_draw_indirect_count`, which is a different question (plan step 2f).

## Not changed by this slice, flagged
- `has_casters = phase_entity_count > 0` is still permanently true on every shadow-mapped lamp face once one
  batch exists, so the lamp shadow cache's early-out stays disabled for statics. Needs the per-batch
  generation counter (plan §4 item 4, S3a3) - measure the lamp cache before/after S3b with that in mind.
- The `is_active` gate is still only compiled under `FRUSTUM_CULLING`; it becomes genuinely
  def-independent when `is_active` moves into `MeshInput.flags` (plan §4 item 5).
