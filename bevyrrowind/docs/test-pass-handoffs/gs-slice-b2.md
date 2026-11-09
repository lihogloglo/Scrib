# Patch GS slice B2: what only a GPU can prove

Agent session 2026-11-10, static-scene.md S3a2, `superfargoth/docs/specs/gs-patch-plan.md` steps 2e and 2f
(the range **drawing** path). Branch `static-scene/gs-slice-b2`, continuing `gs-slice-b1.md`. With this slice
**patch GS step 2 is complete**: a `GpuBatchedMesh3d` batch with N geometry ranges now draws N geometries.

## What changed
- `instance_batch_sub_draws(indirect_base, output_base, spans)` (`bevy_render/src/batching/mod.rs`) is the
  one place a batch's sub-draw layout is decided. Span *i* ⇒ indirect parameters slot `indirect_base + i`
  and output slots `output_base + Σ counts[..i]`. Metadata writes, binned work items and sorted work items
  all read it.
- `prepare_instance_batch_indirect` allocates `spans.len()` contiguous indirect slots in **one**
  `IndirectBatchSet`, writes one `IndirectParametersMetadata` per span, and returns the range plus the
  phase item's `extra_index`. One `multi_draw_indirect_count` of N sub-draws per batch.
- `render_instance_batches` applies `render_phase::multi_draw_indirect_count_supported(features, backend)`
  — the same test `render_batchable_meshes` used inline, now shared — and drops `batch_set_index` (keeping
  the range) when it is false, so DX12 draws all N commands with `multi_draw_indirect`.
- Sorted phases pass `use_batch_set: false`: no batch set index in their metadata, so
  `build_indirect_params.wesl` keeps the 1:1 mapping their plain `multi_draw_indirect` needs.

## Checked here, without a GPU
`sf_world_bevy` **29 tests** (25 before), clippy `--all-targets -D warnings` clean:
- `instance_batch_sub_draws` of a 3-range batch: 3 commands, contiguous from the allocation base, in span
  order, output slots partitioned, counts summing to the capacity.
- A range-less batch is **exactly one** command at the allocation base over the whole capacity — asserted
  as a whole-struct equality, the pre-slice behaviour byte for byte.
- The real `UntypedPhaseIndirectParametersBuffers` (no `RenderDevice` needed until upload): `allocate(3)`
  ⇒ `batch_count` 3, `get_next_batch_set_index` read after it, `add_batch_set` ⇒ `batch_set_count` 1; a
  second batch's commands follow contiguously in its own set; the other mesh class is untouched.
- The gate: feature + Vulkan/Metal ⇒ count form; feature + **DX12 ⇒ not**; no feature ⇒ not, on any
  backend.

## Needs a GPU (nothing here is verified)
1. **The one thing this slice exists for**: a batch with N ranges draws N geometries. Nothing on this
   machine issues a draw. On the workstation, the first frame a ranged batch exists:
   `multi_draw_indexed_indirect_count` (or `multi_draw_indirect` on DX12) with
   `indirect_parameters_count == N`, and each range's slots showing that range's mesh. Watch
   `sf_diag`'s `instance_draws` (now N per batch, not 1) against `instance_batches`.
2. **DX12 inherits upstream's wgpu [#7974](https://github.com/gfx-rs/wgpu/issues/7974) compromise, and we
   should look at it on real content.** The *metadata* always carries the batch set index, while the
   *draw* drops it on DX12. `build_indirect_params.wesl` enables its compaction branch on the feature
   alone (`bevy_pbr/src/render/gpu_preprocess.rs:2193-2204`, no backend test), so on DX12 **with** the
   feature the shader `atomicAdd`s its commands into the front of the batch set while the draw issues all
   N slots — a command whose instances were all culled leaves its slot holding whatever the
   `UninitBufferVec` had. This is **exactly** what upstream's multidrawable batch sets already do there
   (`prepare_multidrawable_binned_batch_set` + `allocate_uniforms.wesl:125`), so it is not a regression
   introduced here and fixing it is an upstream-shaped change (gate the shader def on the backend too, or
   zero the tail). It only bites when a range is fully culled, so a ranged batch straddling the frustum
   edge on DX12 is the case to look at: stray geometry or a flickering sliver, never a crash.
3. **The work-item count per batch is now N ranges, not 1.** `push_range` reserves
   `push_multiple_uninit(count)` per span, so a batch's work items are still exactly its capacity in
   total, but spread over N `PreprocessWorkItemRange`s. Watch the preprocess dispatch count and
   `work_items` in `sf_diag` rise by N-1 per ranged batch and **not** otherwise.
4. **Sorted-phase instance batches have never run anywhere.** A `GpuBatchedMesh3d` in a sorted
   (transparent) phase takes the `use_batch_set: false` path, which exists so the shader's 1:1 mapping
   matches a plain `multi_draw_indirect`. S3b's statics are opaque, so this path is reviewed, not tested
   and not measured. If a producer ever puts a ranged batch in `Transparent3d`, check it before trusting
   it.
5. **The `NoIndirectDrawing` warnings.** Both batching callers now `warn_once` instead of dropping every
   instance batch silently. Expect exactly one line per phase kind on an adapter clamped to
   `PreprocessingOnly` or a camera with `NoIndirectDrawing`, and **none** otherwise — a line in a normal
   run means a view lost indirect drawing and its statics are absent (the documented hardware floor).

## What a producer must satisfy (unchanged from slice B1)
`batch_spans` is still the only way spans are built and the contract is the same: slot-ordered, counts
summing to `max_capacity`, empty range list ⇒ one whole-batch span. The two additions a producer should
know:
- **Range order is draw order within the batch**, and with `multi_draw_indirect_count` the GPU compacts
  the non-empty commands, so the *issued* order is not the span order. Nothing may depend on sub-draw
  order inside one batch (fine for opaque; a transparent ranged batch cannot sort its ranges).
- An empty `spans` slice still means "not drawable this frame" and now trips
  `debug_assert!(!spans.is_empty())` only *inside* `prepare_instance_batch_indirect`; both callers still
  drop such a batch before reaching it.

## Plan inaccuracies found (`gs-patch-plan.md`), beyond slice B1's
- `gs-patch-plan.md:297-328` sketches `prepare_instance_batch_indirect` as taking `data_buffer` and doing
  its own `add_multiple`, returning `(Range<u32>, Option<NonMaxU32>, Range<u32>)`. The tree's version takes
  `output_base` and returns `(Range<u32>, PhaseItemExtraIndex)`; the `add_multiple` stayed in the callers,
  where slice A left it. (Already flagged in `aced45f`; re-confirmed.)
- `gs-patch-plan.md:360-362` says the sorted caller is "identical" to the binned one. **It must not be.**
  `SortedRenderBatchSet::flush` (`gpu_preprocessing.rs:1914-1922`) always writes `batch_set_index: None`,
  and `SortedRenderPhase::render_range` has no hardware gate, so a sorted batch is always drawn with a
  plain `multi_draw_indirect` over its whole range. Writing a real batch set index into its *metadata*
  would make `build_indirect_params.wesl` compact the commands into slots that draw does not read — a
  regression on every MDI-count-capable backend, not just DX12. Hence the `use_batch_set` parameter: the
  binned caller passes `true`, the sorted caller `false`.
- `gs-patch-plan.md:370-383` (step 2f) says `render_instance_batches` is at `render_phase/mod.rs:1232-1260`
  and `render_batchable_meshes`'s gate at `:1277-1281`/`:1347-1351`. Those were right before this slice;
  the gate now lives in one `multi_draw_indirect_count_supported` near `InstanceBatchDraw`.
- `gs-patch-plan.md:496` asks for `debug_assert!(spans.len() > 0)`. Written as
  `debug_assert!(!spans.is_empty())` (clippy `len_zero`).
