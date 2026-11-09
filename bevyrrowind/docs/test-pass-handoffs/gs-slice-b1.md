# Patch GS slice B1: what only a GPU can prove

Agent session 2026-11-10, static-scene.md S3a2, `superfargoth/docs/specs/gs-patch-plan.md` step 2a-2d (the
range **data path**). Branch `static-scene/gs-slice-b1`, continuing the interrupted session that wrote
steps 4-7. Plan steps 2e (N indirect slots per batch set) and 2f (the draw-time
`multi_draw_indirect_count` gate) are **not** in this slice: the ranges are built, validated, uploaded and
read per instance by `materialize`, while **drawing still behaves exactly as slice A left it**.

Nothing in the game spawns a `GpuBatchedMesh3d` yet (S3b does), so there is **no visual check here** and no
`visual-checks.md` row: a visual pass would show exactly today's picture. What follows is for the first run
that does spawn one with ranges.

## Checked here, without a GPU
`sf_world_bevy` `gpu_batch_tests.rs`, 25 tests (19 before the slice), clippy `--all-targets -D warnings`
clean:
- `materialize.wesl` still links (`wesl`) and naga-validates with the 5th binding, the `GeometryRange`
  struct and the `range_of` search, across every def set the 0.20 preprocessing pipelines specialise.
- `GpuBatchGeometryRange` is 64 B and 16-aligned, and **every** field sits at the same byte offset in
  `#[repr(C)]` and in naga's WGSL layout (`aabb_center` 32, `aabb_half_extents` 48 - the `vec3<f32>`
  align-16 rule inserting no padding `#[repr(C)]` would not). `range_count` took the first of
  `MaterializationMetadata`'s two pad words, so the uniform's size and every earlier offset are unchanged.
- The `MeshInput` and `MeshCullingData` fields `materialize` overwrites per instance still exist by name:
  an upstream rename fails a test rather than a frame.
- `validate_ranges` accepts exactly the slot partitions of `0..max_capacity` and names each way of not
  being one (empty span, gap, overlap, unsorted, short, long, `u32` overflow).
- `range_of` is exhaustive over every slot of 200 randomised partitions, with the partition itself as the
  oracle (exactly one range contains each slot), plus the boundary and empty-table cases.
- `batch_spans` turns an empty range table into the single whole-batch span upstream #25978 reserved, and
  a 3-range table into three spans tiling the same capacity.
- A headless app (no render app) runs the plugin with a ranged batch: it joins the `Mesh3d` visibility
  class and the sun's cascade lists exactly as a range-less batch does.

## Needs a GPU (nothing here is verified)
1. **A ranged batch draws its ranges' geometry.** Not yet: with `ranges` non-empty, `materialize` writes
   per-instance `first_vertex_index` / `first_index_index` / `index_count` / `metadata_index` /
   `material_and_lightmap_bind_group_slot` and per-range culling bounds, but the batch is still **one**
   indirect sub-draw, so the draw reads the *first* work item's `mesh_index` and every slot renders the
   first range's geometry. That is plan step 2e's job. Until it lands, **the only GPU-meaningful check of
   this slice is a regression check**: a batch with `ranges` **empty** must look and measure exactly as it
   did after slice A (no range table is read - `range_count == 0` - and the batch reserves one span over
   its whole capacity).
2. **The 5th bind-group entry and the range buffer.** `materialize`'s layout gained
   `storage_buffer_read_only::<GpuBatchGeometryRange>(false)` at `@binding(4)`, and each job owns a
   `RawBufferVec` that is **never empty** (a storage binding cannot be: a range-less batch uploads one
   zeroed record it never reads). `min_binding_size` is 64 B and the buffer is 64 B at minimum, so a
   wgpu-validation run (`--validate`) on the first frame a batch exists is the check that matters. The
   range buffer's `BufferId` is in `bound_buffers`, so a resize rebuilds the bind group; watch
   `preprocess/bind_groups_materialize` rise on a batch's first frame and after a capacity change, and
   **not** every frame (the table is compared record-for-record and only re-uploaded when it changes).
3. **Per-frame range rejection.** `prepare_gpu_batched_mesh_draws` re-resolves every range each frame
   against `MeshAllocator::mesh_slabs` and `RenderMaterialBindings`, because slabs move
   (`meshes_displaced_by_slab_growth`). A range whose mesh is in a different vertex slab, index slab or
   material bind group than the batch's own mesh ⇒ `warn_once` naming the entity and the range index, and
   the batch draws **nothing** that frame (empty `input_range`, empty `spans`). On real content this is
   the failure mode to watch for in the log: it is a producer bug (S3b's batch key must include both slab
   ids and the material bind group index), not a renderer one, and it makes a whole batch vanish.
4. **Slot-layout rejection is log-only.** A `ranges` list that is not a partition of `0..max_capacity` is
   caught on extraction, `warn_once`d and **cleared**, so the batch falls back to upstream behaviour
   (drawing its own mesh over every slot) rather than disappearing. The headless half of plan test T5 is
   unreachable from `sf_world_bevy` (see "Plan inaccuracies" below), so this branch is reviewed, not
   tested: if a producer's ranges are wrong, expect the warning plus whole-mesh instances.
5. **A non-indexed range.** `resolve_gpu_range` offsets `first_vertex_index` by `range.first_index` when
   the mesh has no index slab, and `first_index_index` otherwise (what `build_indirect_params.wesl`
   reads). Only the indexed path will ever be exercised by S3b's cooked meshes; the non-indexed path has
   never run anywhere.

## For the slice that takes plan steps 2e and 2f
- The spans are the whole interface: `GetFullBatchData::get_instance_batch` returns
  `Option<&[InstanceBatchSpan]>` (`bevy_render/src/batching/mod.rs`), `{ base_input_index, count }` per
  range, in slot order, their counts summing to the batch's capacity. **An empty slice is #25978's empty
  range**: not drawable this frame.
- Both batching callers currently collapse the spans back to what #25978 reserved - `spans[0]
  .base_input_index` and the summed count, one work-item range, one indirect parameters slot, one batch
  set (`gpu_preprocessing.rs` ~2039 sorted, ~2585 binned). Step 2e replaces exactly those two blocks; it
  does not need to touch extraction, resolution, the range table or `materialize`.
- `batch_spans(base, max_capacity, slot_spans)` in `gpu_instance_batch/mod.rs` is the one place a batch's
  spans are built, and the "empty ⇒ one whole-batch span" rule is tested there. Keep it that way: step 2e
  wants `debug_assert!(!spans.is_empty())`, not a second construction site.
- `GpuBatchGeometryRange::flags` is reserved and always 0 today (`materialize` ORs it into the instance's
  flags). A per-range `MeshFlags` need is free to take it.
- `materialize.wesl`'s `range_of` is a hand-kept mirror of Rust `range_of`; nothing executes the WGSL
  headlessly, so the equivalence is a review duty. Change one, change both.

## Plan inaccuracies found (`gs-patch-plan.md`)
- `gs-patch-plan.md:525-528` (test T5) asks for "one headless case on `app()` asserting a bad batch extracts
  with `ranges` cleared". **Not reachable**: `extract_gpu_batched_meshes` is a render-world
  `ExtractSchedule` system writing the `pub(crate)` `ExtractedGpuBatchedMeshes`, and the headless test app
  has no render app. The pure half (`validate_ranges` returns the error; `batch_spans` of an empty table
  is the upstream span) is tested instead, and the warn-and-clear branch between them is two lines.
- `gs-patch-plan.md:250-252` specifies `MaterializationJob` keeping a `range_revision: u64` hash. Built as an
  exact record-for-record comparison plus a `ranges_dirty` flag instead: the table is short, the
  comparison cannot collide, and `RawBufferVec::write_buffer` re-uploads unconditionally so the flag is
  needed either way.
