# Patch "GS": executable plan for the #25978 additions (static-scene S3a2 / S3a3)

Status: **design, 2026-11-09**. Work order for the vendored additions static-scene.md §6 promises. Inputs:
the [S3a port audit](../research/s3a-port-audit-2026-11-09.md), [static-scene.md](static-scene.md) §2-§9,
`vendor/README.md` "The GPU-batched mesh port". Nothing compiled or run (laptop rule). Every line:number was
re-read in the vendored tree; "unverified" marks GPU behaviour I could not check. Paths are relative to
`superfargoth/vendor/` unless they start with `superfargoth/` or `bevyrrowind/`.

---

## 1. Decision summary (the shape we are building)

1. A batch entity carries **N geometry ranges**, not one mesh. A range = a contiguous span of the batch's
   instance slots + the index range (and material slot, and model-space AABB) that span draws.
2. The N ranges of one batch get **N contiguous `IndirectParametersMetadata` slots and one shared
   `IndirectBatchSet`**, so the batch issues exactly one `multi_draw_indirect_count` of N sub-draws
   (`render/mesh.rs:5027-5047`, `:5092-5108`).
3. Output (`MeshUniform`) space is allocated **per range, summing to the batch capacity** - Σ span counts =
   capacity - so nothing is over-provisioned beyond today.
4. `materialize.wesl` writes **per-instance geometry** (`first_vertex_index`, `first_index_index`,
   `index_count`, `metadata_index`, material slot, AABB) by binary-searching the range table for the
   instance's slot. One source of truth: the span layout.
5. The existing "first work item of a draw writes `mesh_index`" rule (`mesh_preprocess.wesl:178-181`) then
   becomes **correct without sorting**: every instance sharing an indirect slot shares one range, hence
   identical geometry fields. No counting sort, no ordered work items, no CPU-ordered producer.
6. **Per-instance LOD** (S3a3) adds nothing to the instance record: every geometry field is **per range**
   and view-independent, so a chain's levels differ only in fields `GpuBatchGeometryRange` already
   carries (2d) - `first_vertex_index` and `metadata_index` included, since our cooker gives each level
   its own vertex buffer (`sf_model/src/cook.rs:449-455`; corrected 2026-11-10, this item used to say
   "a chain's levels share vertices"). All of it read **only** through `mesh_index`
   (`build_indirect_params.wesl:134-144`), never per instance (`MeshUniform` has no index fields,
   `render/mesh.rs:560-604`). So per-view LOD = "put the instance in the indirect slot of its level": it
   needs (a) a `geometry_index` on `IndirectParametersMetadata` so `mesh_index` can point at a per-level
   template, and (b) a per-(view,batch) **count → bump-allocate → scatter** pass, because per-range
   survivor counts are then GPU-only. That is S3a3, the big half.
7. Range unpacking moves **inside `early_gpu_preprocess`** so every schedule that preprocesses also unpacks
   (fixes BLOCKER 2 structurally, not per schedule).
8. A batch's ranges must share one `MeshAllocator` vertex slab, index slab and material bind group - the
   draw binds exactly one of each. Enforced, warned, and it corrects §6's batch key.

---

## 2. Audit verification and corrections

| Audit claim | Verdict |
|---|---|
| BLOCKER 1 (one mesh / one index range per batch) | **Confirmed.** `GpuBatchedMesh3d.mesh` single handle (`bevy_pbr/src/gpu_instance_batch/mod.rs:100`); template from one asset (`mod.rs:421-425` → `render/mesh.rs:678-705`); `build_indirect_params.wesl:133-144` reads geometry from `current_input[mesh_index]`; `push_range` takes one triple (`bevy_render/src/batching/gpu_preprocessing.rs:783-828`). |
| BLOCKER 2 (`unpack_ranges` only in `Core3d`) | **Confirmed.** Registered at `gpu_instance_batch/mod.rs:194-202`; `Core3dShadows` list is `allocate_uniforms, unpack_bins, early_gpu_preprocess, …` with no range unpacking (`render/gpu_preprocess.rs:633-662`); `gather_shadow_cascades_for_view` filters `LightEntity::Directional` (`gpu_preprocess.rs:1199-1220`); `sf_shadow_cache_bevy`'s `LampShadowView` schedule likewise (`superfargoth/crates/sf_shadow_cache_bevy/src/render.rs:120-147`). Uninitialised work items are then read over `arrayLength(&work_items)` (`mesh_preprocess.wesl:166-181`). |
| BLOCKER 3 (`batch_set_count` blind) | **Confirmed.** `render_phase/mod.rs:1539-1549` reads only `self.batch_sets`; instance batches draw from `instance_batch_draws` (`mod.rs:1232-1256`). `bin_count`/`entity_count` were updated (`:1500`, `:1535-1536`). |
| SERIOUS 5 (no batch-set merging) | **Confirmed.** `prepare_instance_batch_indirect` (`gpu_preprocessing.rs:2250-2270`) allocates 1 param + 1 batch set with `batch_set_index: None`. |
| SERIOUS 6 (sized by `max_capacity`) | **Confirmed** (`:2587`, `:807`, `gpu_instance_batch/mod.rs:441-442`). **But** Bevy's own multidrawable path also sizes outputs by *pre-cull* bin counts, so "capacity, not survivors" is wrong only where capacity exceeds *residency*: right-sizing is a **producer** duty. The patch's duty is that step 2's per-range split must not multiply it. |
| SERIOUS 7 (`no_indirect_drawing` silently draws nothing) | **Confirmed** (`:2574-2576`, `:790-799`), **but the audit's framing is wrong about our views**: `sf_sky_bevy/src/sun_view.rs:227-231`/`:255-257` and `sf_shadow_cache_bevy/src/render.rs:518`/`:546-549` both inherit the *camera's* `NoIndirectDrawing` and clamp by `GpuPreprocessingSupport`. Our inherited shadow views never individually lose indirect drawing; the failure is adapter-wide (`GpuPreprocessingMode::PreprocessingOnly`) or a deliberate camera setting. It is a hardware-floor warning, not a per-view fallback. |
| SERIOUS 8 (#26161 hash blind) | **Confirmed.** `entity_count()` counts one per batch (`render_phase/mod.rs:1498-1521`), folded at `render/light.rs:1509-1520` via `h.write_usize(phase_entity_count)`; `has_casters = phase_entity_count > 0` at `:1559`. |
| MINOR: `is_active` nested in `@if(FRUSTUM_CULLING)` | **Confirmed** (`mesh_preprocess.wesl:190-194`). Fixed in S3a2 step 4 (1 line). |
| MINOR: `sort_binned_render_phase` skips `instance_batches` | **Confirmed** (`batching/mod.rs:218-223`). Left as is; see open question O4. |
| (not in the audit) CPU-init-before-uninit | **Checked, not violated.** `push_init` debug-asserts `uninit_element_count == 0` (`buffer_vec.rs:933-941`). Work item buffers are per view (`gpu_preprocessing.rs:648-706`); the shared `metadata` buffer is safe only because `batch_and_prepare_binned_render_phase` does every `push_multiple_init` in its first view loop and every `push_multiple_uninit` (multidrawables, `:2794`, `:2800`) in the second (`:2317-2330`). **Our new code must stay in the first loop and keep using `UntypedPhaseIndirectParametersBuffers::allocate`.** |

### Corrections the plan forces on static-scene.md §6 (coordinator owns the edit)

- **C1. "a batch entity per (space, material class)" is not sufficient.** The draw binds one vertex buffer,
  one index buffer and one material bind group (`render/mesh.rs:4949-4978`) and the per-range offsets are
  slab-relative (`MeshInputGeometry::new`, `:706-730`). The key is **(space, material class, vertex slab,
  index slab, material bind group)**. §5 already says "pipeline × bind group × mesh slab"; §6 must agree.
  §Budgets' "≤ 2 × material classes" holds only while a class fits one slab pair - make slab count a
  measured quantity in S3b.
- **C2. "LOD index ranges per instance" is not a peer of "cell-range upload/free".** It needs a GPU
  count/allocate/scatter stage (§4.2) because per-range survivor counts are view-dependent and GPU-only.
  Split §6's bullet into "N geometry ranges per batch" (S3a2) and "per-view per-instance level selection"
  (S3a3/S4).
- **C3. The cull AABB is per batch today** (`gpu_instance_batch/mod.rs:433-437` writes the batch mesh's AABB
  to every slot), so Bevy's own frustum cull is wrong for a multi-chain batch. S3a2 makes it per range; §6
  should say so next to its `MeshCullingData::is_active` note.
- **C4. §8 "a cached face simply skips its dispatch"** holds only once SERIOUS 8 is fixed: `has_casters` is
  permanently true for every shadow-mapped lamp face as soon as one batch exists.

---

## 3. S3a2 - the minimum that makes S3b correct and measurable

Four steps. Each leaves the tree compiling and is independently revertible. Marker per hunk as noted.
**Step order matters**: step 1 removes the uninitialised-memory hazard before step 2 multiplies the number
of reserved work items.

### Step 1 - range unpacking becomes part of mesh preprocessing (BLOCKER 2)

Marker: `// superfargoth patch (own, GS: range unpack in preprocess)`.

Why not "register `unpack_ranges` in `Core3dShadows` too": three schedules today (`Core3d`, `Core3dShadows`,
`LampShadowView`) and nothing stops a fourth. `early_gpu_preprocess` already iterates exactly the right view
set in every schedule and already holds every parameter the dispatch needs.

1. `bevy_pbr/src/render/gpu_preprocess.rs`
   - Delete `pub fn unpack_ranges` (`:937-1002`). Replace with a private
     `fn dispatch_range_unpacking(command_encoder: &mut CommandEncoder, diagnostics: Option<&dyn RecordDiagnostics>, pipeline: &ComputePipeline, buffers: &BatchedInstanceBuffers<MeshUniform, MeshInputUniform>, views: &[Entity], view_query: &Query<…>)`
     with the body of the old loop (`:978-1001`): one `ComputePassDescriptor { label: Some("range unpacking") }`,
     `set_pipeline`, then per view per phase per (indexed, non-indexed) `set_bind_group(0, ranges.bind_group, &[])`
     + `dispatch_workgroups(total_instance_count.div_ceil(64), 1, 1)`.
   - In `early_gpu_preprocess` (`:1006`), after `all_views` (`:1036-1038`) and **before**
     `begin_compute_pass("early_mesh_preprocessing")` (move that call down), dispatch it when
     `preprocess_pipelines.range_unpacking.pipeline_id` resolves. Two separate compute passes are required;
     wgpu inserts the write→read barrier between them - the one place I rely on automatic barriers,
     **unverified**, but the same guarantee `unpack_bins` already relies on (`:770-870`). `early_gpu_preprocess`
     needs **no new parameters**: the bind groups live in `batched_instance_buffers`, the diagnostics
     recorder is fetched at `:1021-1022`, and both queries already filter `Without<SkipGpuPreprocess>`.
2. `bevy_pbr/src/lib.rs`: drop `unpack_ranges` from the re-export list (grep it).
3. `bevy_pbr/src/gpu_instance_batch/mod.rs`: delete the `Core3d` registration (`:194-202`) and the import
   (`:64-66`); **keep** `prepare_range_unpacking_pipeline` (`:170-178`) and
   `prepare_range_unpacking_bind_groups` (`:179-187`) where they are - with the plugin absent the pipeline
   id stays `None` and the dispatch is a no-op, so the plugin stays opt-in.

**Acceptance:** `grep -rn "unpack_ranges" vendor/` shows only `gpu_preprocess.rs`, `unpack_ranges.wesl` and
the pipeline plumbing; no schedule registration anywhere.

### Step 2 - N geometry ranges per batch, one batch set (BLOCKER 1, SERIOUS 5, C1, C3)

Marker: `// superfargoth patch (own, GS: ranges)`.

#### 2a. Main-world API (`bevy_pbr/src/gpu_instance_batch/mod.rs`)

```rust
/// One geometry range of a [`GpuBatchedMesh3d`]: a contiguous span of instance
/// slots that all draw the same index range of the same mesh allocation with
/// the same material slot.
///
/// Every range must resolve into the same `MeshAllocator` vertex slab, index
/// slab and material bind group as the batch's own `mesh`: one indirect draw
/// binds exactly one of each.
#[derive(Clone, Debug)]
pub struct GpuBatchedMeshRange {
    /// The mesh whose allocation this range indexes into.
    pub mesh: bevy_asset::Handle<Mesh>,
    /// First index relative to `mesh`'s own index allocation; first vertex for
    /// a non-indexed mesh.
    pub first_index: u32,
    /// Index count (vertex count when non-indexed); 0 = the mesh's own count.
    pub index_count: u32,
    /// First instance slot of this range in the batch's output buffer.
    pub first_slot: u32,
    /// Number of instance slots in this range.
    pub slot_count: u32,
    /// Material to bind, or `None` for the batch's own `MeshMaterial3d<M>`.
    /// Must share the batch's bind group index. Untyped because
    /// `RenderMaterialBindings::get` is keyed by `UntypedAssetId`
    /// (used at `mod.rs:418-420`).
    pub material: Option<bevy_asset::UntypedAssetId>,
}
```

```rust
#[derive(Component, Clone)]
#[require(Transform, NoFrustumCulling, Visibility, VisibilityClass)]
#[component(on_add = add_visibility_class::<Mesh3d>)]
pub struct GpuBatchedMesh3d {
    pub mesh: bevy_asset::Handle<Mesh>,
    pub max_capacity: u32,
    /// Geometry ranges, sorted by `first_slot`, partitioning `0..max_capacity`
    /// without gaps or overlap. **Empty = upstream #25978 behaviour**: one
    /// range covering the whole batch with `mesh`'s own index range.
    pub ranges: Vec<GpuBatchedMeshRange>,
}
```

Keeping `ranges: Vec::new()` as "exactly upstream" is deliberate: the port's existing semantics and
`gpu_batch_tests.rs` stay valid, and the addition is offerable upstream on its own.

#### 2b. Extraction (`extract_gpu_batched_meshes`, `mod.rs:258-345`)

`ExtractedGpuBatchedMesh` gains `ranges: Vec<ExtractedRange>` with
`struct ExtractedRange { mesh: AssetId<Mesh>, first_index: u32, index_count: u32, first_slot: u32, slot_count: u32, material: Option<UntypedAssetId>, local_bounds: Option<Aabb> }`;
`local_bounds` comes from the existing `mesh_bounds` `Local` cache (`:270`, `:330-333`), extended to every
range mesh. Validate here (once per change) and `warn_once` + clear `ranges` on failure: sorted by
`first_slot`, each `first_slot + slot_count` chaining exactly to the next, the last ending at
`max_capacity`.

#### 2c. Per-frame resolution (`prepare_gpu_batched_mesh_draws`, `mod.rs:371-444`)

Keep the structure; change the per-batch body. Per range, `MeshInputGeometry::new(allocator, range.mesh)`
(make it `pub(crate)`, `render/mesh.rs:706-730`) gives the mesh's slab-relative `first_vertex_index`,
`first_index_index`, `index_count`, `metadata_index`; the range's absolute index start is
`first_index_index + range.first_index`, its count `range.index_count` (0 ⇒ the mesh's own). Bounds-check
`range.first_index + range.index_count <= mesh index_count`, and compare
`mesh_slabs(range.mesh).{vertex_slab_id, index_slab_id}` (`bevy_render/src/mesh/allocator.rs:345`) and
`bindings.get(range.material)?.group` with the batch's own. Any mismatch ⇒ `warn_once!` naming the entity
and range, and leave `batch.input_range` **empty** - the existing "do not draw this frame" signal
(`batching/mod.rs:198-204`, honoured at `gpu_preprocessing.rs:2583-2585`). Slabs move
(`meshes_displaced_by_slab_growth`, `allocator.rs:298`; bevy #26456), so this re-validates every frame for
free. Then build the range table (2d) and store the resolved spans:

```rust
// in bevy_pbr/src/render/mesh.rs, next to RenderMeshInstanceBatch
pub struct RenderMeshInstanceBatch {
    pub asset_id: AssetId<Mesh>,
    pub input_range: Range<u32>,
    pub world_center: Vec3,
    // superfargoth patch (own, GS: ranges)
    /// One entry per geometry range, in slot order. Empty when the batch is
    /// not drawable this frame.
    pub spans: Vec<InstanceBatchSpan>,
}
```

```rust
// in bevy_render/src/batching/mod.rs, next to GetFullBatchData
/// One indirect sub-draw of a GPU-authored instance batch: a contiguous span
/// of mesh-input indices. All spans of one batch share one batch set.
#[derive(Clone, Copy, Debug)]
pub struct InstanceBatchSpan {
    /// First index in the `MeshInputUniform` buffer.
    pub base_input_index: u32,
    /// Number of instance slots.
    pub count: u32,
}
```

`GetFullBatchData::get_instance_batch` changes signature (`batching/mod.rs:196-210`,
impl at `render/mesh.rs:3304-3313`):

```rust
fn get_instance_batch<'a>(
    _param: &'a SystemParamItem<Self::Param>,
    _query_item: MainEntity,
) -> Option<&'a [InstanceBatchSpan]> { None }
```
impl: `instance_batches.get(&main_entity).map(|b| &b.spans[..])`. An empty slice replaces the old empty
range as "not drawable".

#### 2d. The GPU range table and `materialize`

New per-batch buffer, owned by `MaterializationJob` (`gpu_instance_batch/materialize.rs:35-60`):

```rust
#[derive(Clone, Copy, Default, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GpuBatchGeometryRange {   // 64 B, 16-aligned
    pub first_slot: u32,                                  // 0
    pub first_vertex_index: u32,                          // 4
    pub first_index_index: u32,                           // 8
    pub index_count: u32,                                 // 12
    pub metadata_index: u32,                              // 16
    pub material_and_lightmap_bind_group_slot: u32,       // 20
    pub flags: u32,                                       // 24  reserved, 0
    pub pad0: u32,                                        // 28
    pub aabb_center: Vec3,                                // 32..44
    pub pad1: f32,                                        // 44
    pub aabb_half_extents: Vec3,                          // 48..60
    pub pad2: f32,                                        // 60
}
```
Chosen so WGSL's `vec3<f32>` align-16 rule puts `aabb_center` at 32 and `aabb_half_extents` at 48 with no
implicit padding; stride 64 in both languages. Asserted by test T2.

`MaterializationJob` gains `ranges: RawBufferVec<GpuBatchGeometryRange>` (`BufferUsages::STORAGE`) and
`range_revision: u64`; `update()` rewrites and `write_buffer`s them only when the revision changes (for
S3b, hash `(len, first_slot, first_index_index, index_count)`). `MaterializationMetadata`
(`materialize.rs:26-33`) gains `range_count: u32` in place of one `pad` word, staying 16-aligned. The bind
group layout (`:105-116`) gains a 5th entry `storage_buffer_read_only::<GpuBatchGeometryRange>(false)` and
`bound_buffers` becomes `[BufferId; 4]` (`:39`, `:162-166`) - **include the range buffer id, or a resize
leaves a stale bind group.**

`gpu_instance_batch/materialize.wesl`:

```wgsl
// struct GeometryRange { … } - mirror of `GpuBatchGeometryRange`, field for
// field and in order, with `vec3<f32>` for the two `Vec3`s and `f32` pads.
@group(0) @binding(4) var<storage> ranges: array<GeometryRange>;

// Ranges are sorted by first_slot and partition 0..capacity, so the range of a
// slot is the last one whose first_slot <= slot.
fn range_of(slot: u32, count: u32) -> u32 {
    var low = 0u;
    var high = count;
    while (low + 1u < high) {
        let mid = low + (high - low) / 2u;
        if (ranges[mid].first_slot <= slot) { low = mid; } else { high = mid; }
    }
    return low;
}
```
and, inside `main` after `var input = metadata.mesh_input;` (`materialize.wesl:23`):
```wgsl
    var culling = metadata.bounds;
    if (metadata.range_count > 0u) {
        let r = range_of(id.x, metadata.range_count);
        input.first_vertex_index = ranges[r].first_vertex_index;
        input.first_index_index  = ranges[r].first_index_index;
        input.index_count        = ranges[r].index_count;
        input.metadata_index     = ranges[r].metadata_index;
        input.material_and_lightmap_bind_group_slot =
            ranges[r].material_and_lightmap_bind_group_slot;
        input.flags = input.flags | ranges[r].flags;
        culling.aabb_center       = ranges[r].aabb_center;
        culling.aabb_half_extents = ranges[r].aabb_half_extents;
    }
```
(the existing determinant/`tag`/`is_active` lines at `:25-34` are unchanged; `culling.is_active` keeps
coming from the instance). Binary search is ~12 iterations for 4096 ranges, once per slot per *frame*
(not per view) - the materialize dispatch is already `capacity/64` workgroups (`materialize.rs:210`).

#### 2e. Batching: N indirect slots, one batch set

`bevy_render/src/batching/gpu_preprocessing.rs`, replace `prepare_instance_batch_indirect` (`:2250-2270`):

```rust
// superfargoth patch (own, GS: ranges)
/// One indirect draw command per geometry range, all in **one** batch set, so
/// the batch issues a single `multi_draw_indirect_count`. Returns the indirect
/// parameters range, the batch set index, and the `MeshUniform` span the batch
/// occupies (allocated per range, summing to the batch's capacity).
fn prepare_instance_batch_indirect<GFBD: GetFullBatchData>(
    buffers: &mut UntypedPhaseIndirectParametersBuffers,
    data_buffer: &mut UninitBufferVec<GFBD::BufferData>,
    view: RetainedViewEntity,
    indexed: bool,
    spans: &[InstanceBatchSpan],
) -> (Range<u32>, Option<NonMaxU32>, Range<u32>) {
    let total: u32 = spans.iter().map(|s| s.count).sum();
    let output_base = data_buffer.add_multiple(total as usize) as u32;
    let ip_base = buffers.allocate(indexed, spans.len() as u32);
    let batch_set_index = buffers.get_next_batch_set_index(indexed);
    let mut output = output_base;
    for (i, span) in spans.iter().enumerate() {
        GFBD::write_batch_indirect_parameters_metadata(
            indexed, output, batch_set_index, buffers, ip_base + i as u32,
        );
        output += span.count;
    }
    let ip_range = ip_base..(ip_base + spans.len() as u32);
    buffers.add_batch_set(indexed, &view, ip_range.clone());
    (ip_range, batch_set_index, output_base..output)
}
```
`get_next_batch_set_index` must be read **after** `allocate` and immediately before `add_batch_set`
(`:1452-1456` returns `batch_set_count(indexed)`), with nothing else pushing a batch set in between.

Binned caller (`:2567-2614`):
```rust
for (key, bin) in &phase.instance_batches {
    let indexed = key.0.indexed();
    for &main_entity in bin.iter() {
        if no_indirect_drawing {
            warn_once!("GPU instance batches need indirect drawing; this view draws none");
            continue;
        }
        let Some(spans) = GFBD::get_instance_batch(&system_param_item, main_entity) else { continue };
        if spans.is_empty() { continue; }
        let (ip_range, batch_set_index, instance_range) = prepare_instance_batch_indirect::<GFBD>(
            &mut phase_indirect_parameters_buffers.buffers, data_buffer,
            extracted_view.retained_view_entity, indexed, spans,
        );
        for (i, span) in spans.iter().enumerate() {
            work_item_buffer.push_range(
                indexed, span.base_input_index, ip_range.start + i as u32, span.count,
            );
        }
        phase.instance_batch_draws.push(InstanceBatchDraw {
            batch_set_key: key.0.clone(), bin_key: key.1.clone(),
            batch: BinnedRenderPhaseBatch {
                representative_entity: (Entity::PLACEHOLDER, main_entity),
                instance_range,
                extra_index: PhaseItemExtraIndex::IndirectParametersIndex {
                    range: ip_range, batch_set_index,
                },
            },
        });
    }
}
```
`InputUniformIndex` (`:2581`) disappears. Sorted caller (`:2026-2065`): identical, with
`pending_ranges.push((indexed, span.base_input_index, ip_range.start + i, span.count))` per span so the
`push_range` calls still follow all `push` calls for that view (`:2234-2237`).

#### 2f. Rendering the batch set

`render_phase/mod.rs`, `render_instance_batches` (`:1232-1260`) forwards `extra_index` verbatim. Now that
`batch_set_index` is `Some`, it must apply the same hardware gate as `render_batchable_meshes`
(`:1277-1281`, `:1347-1351`) - `features().contains(MULTI_DRAW_INDIRECT_COUNT) && backend != Dx12` (the
wgpu #7974 workaround; **DX12 is our shipping backend, so the gate fires normally**) - and when it is
false rewrite `IndirectParametersIndex { range, .. }` to `batch_set_index: None`, keeping the range. Factor
the predicate into one `fn multi_draw_indirect_count_supported(world) -> bool` used by both.

Without `multi_draw_indirect_count`, `build_indirect_params.wesl` keeps the 1:1 batch→slot mapping and
writes `instance_count = 0` for empty ranges (`:93`, `:123`), so `multi_draw_indirect` over all N commands
is correct - it just always issues N. **That is the documented fallback for §Risks
"multi_draw_indirect_count on our target".**

### Step 3 - counters (BLOCKER 3)

Marker: `// superfargoth patch (own, GS: counters)`.

1. `bevy_render/src/render_phase/mod.rs` - `batch_set_count()` (`:1539-1549`) gains
   `+ self.instance_batch_draws.len()`; three new read-only accessors:
   `instance_batch_draw_count()` = `instance_batch_draws.len()`;
   `instance_batch_indirect_draw_count()` = Σ `IndirectParametersIndex` range lengths (sub-draws inside the
   batch sets); `instance_batch_slot_count() -> u32` = Σ `batch.instance_range.len()`, the reserved
   **pre-cull** instance count (the post-cull count lives only in `early_instance_count` on the GPU - do not
   pretend otherwise).
2. `bevy_pbr/src/render/gpu_preprocess.rs`: `PreprocessBindGroupCount` (`:343-348`) gains `pub ranges: u32`
   and `pub materialize: u32`, set by `prepare_range_unpacking_bind_groups` (`:874-932`) and
   `gpu_instance_batch/materialize.rs::prepare_bind_groups` (`:135-178`). Both are in
   `RenderSystems::PrepareBindGroups`; `count_views` is in `RenderSystems::Render` after `render_system`
   (`sf_diag_bevy/src/views.rs:68`), so no ordering work.
3. `superfargoth/crates/sf_diag*` (ours): `phase_counts` (`sf_diag_bevy/src/views.rs:122-128`) chains
   `.chain(phase.instance_batches.keys().map(|(k, _)| k))` into the pipeline-key iterator (the field is
   already `pub`, `render_phase/mod.rs:153`) - one line, and it fixes "a pipeline used only by statics
   vanishes". `PhaseCounts` (`sf_diag/src/views.rs:69-81`) gains `instance_batches, instance_draws,
   instance_slots`; `ViewFrame.bind_groups` (`:122-130`) becomes `created + ranges + materialize`.
   `work_items` (`sf_diag_bevy/src/views.rs:142-156`) needs **no change** - `PartialBufferVec::len()`
   already includes the uninitialised tail (`buffer_vec.rs:926-929`) - but its meaning ("reserved, not
   surviving") must go in its doc comment and in `bevyrrowind/docs/perf.md` "Static scene baseline (S0)".

### Step 4 - guards (SERIOUS 7 and the two MINORs that are real)

Marker: `// superfargoth patch (own, GS: guards)`.

1. `mesh_preprocess.wesl`: move the `is_active` gate (`:190-194`) out of the `@if(FRUSTUM_CULLING) {` block
   and give it its own `@if(FRUSTUM_CULLING)` statement before it (it reads `mesh_culling_data`, bound only
   under that def, `:96`), so the gate is no longer *nested inside* the frustum-cull branch. The full fix
   moves `is_active` into `MeshInput.flags` - §4.5, where flags are touched anyway.
2. Document the floor in the `GpuBatchedMesh3d` doc comment and `vendor/README.md`: **"GPU instance batches
   require `GpuPreprocessingMode::Culling`; adapters limited to `PreprocessingOnly` and cameras with
   `NoIndirectDrawing` draw none."** `prepare_gpu_batched_mesh_reservations` (`:323-345`) already warns for
   the adapter case; the per-view `warn_once` is in step 2e.
3. `gpu_instance_batch/mod.rs:206-219`: `.chain()` the two prepare systems (the audit's last MINOR). One line.

### S3a2 size

| File | ± lines |
|---|---|
| `bevy_pbr/src/gpu_instance_batch/{mod.rs, materialize.rs, materialize.wesl}` | ~305 |
| `bevy_pbr/src/render/{gpu_preprocess.rs, mesh.rs, mesh_preprocess.wesl}` | ~115 |
| `bevy_render/src/{batching/gpu_preprocessing.rs, batching/mod.rs, render_phase/mod.rs}` | ~175 |
| `superfargoth/crates/sf_diag*`, `sf_world_bevy` tests (ours) | ~150 |
| **total** | **~745, of which ~595 vendored** |

**Two agent slices, not one**: steps 1 + 3 + 4 (~200 lines, one build) then step 2 (~400 vendored lines,
one build). 1/3/4 do not depend on 2 and give S3b its instrumentation immediately.

---

## 4. S3a3 - what can wait (and what it costs)

Ordered; items 1-2 are what S4's per-instance LOD actually needs.

1. **`IndirectParametersMetadata::geometry_index`** - a 6th `u32` appended (20 B → 24 B; `#[repr(C)]` and
   WGSL stride agree). `0` = "geometry from `current_input[mesh_index]`" (today); non-zero = from
   `current_input[geometry_index]`. Input index 0 is always the dummy from `input.ensure_nonempty()`
   (`gpu_instance_batch/mod.rs:400`), so `0` is a safe sentinel and `Default` /
   `write_batch_indirect_parameters_metadata` (`render/mesh.rs:3275-3303`) need nothing but the field.
   `build_indirect_params.wesl:74` becomes
   `let geometry = select(…mesh_index, g, g != 0u);` with `:134-144` reading `current_input[geometry]`;
   also `occlusion_culling/mesh_preprocess_types.wesl:81-…` and `allocate_uniforms.wesl:119-133`, which
   must write `geometry_index = 0u` beside `mesh_index = 0u` or a slot reused by an ordinary batch set keeps
   a stale pointer. **The only change touching a struct shared by four shaders.** ~60 lines.
2. **Per-view, per-instance level selection: count → bump-allocate → scatter.** Per (view, batch) scratch:
   `counts: array<atomic<u32>>` of `range_count` entries, `bases: array<u32>`, one `RawBufferVec`, cleared
   per frame. **Pass A** (`capacity/64` workgroups): per slot, the level pick (or a `draw_index` the
   producer's own cull pass wrote into a per-(view,batch) buffer) gives `r`; `atomicAdd(&counts[r], 1u)`;
   write `work_items[wi_base + slot] = PreprocessWorkItem { input_base + slot, ip_base + r }`. This
   **replaces `unpack_ranges` for ranged batches** and needs no ordering: §1.5's invariant makes
   `mesh_preprocess`'s `mesh_index` write harmless once item 1 lands, and `geometry_index` is CPU-set.
   **Pass B** (`range_count/64`): `bases[r] = atomicAdd(&cursor, counts[r])` over the batch's own output
   block, then `indirect_parameters_metadata[ip_base + r].base_output_index = output_base + bases[r]`. An
   atomic bump, **not a prefix scan** - ranges need no particular order in the output buffer: 10 lines of
   WGSL instead of reusing `allocate_uniforms`' three-stage scan (`allocate_uniforms.wesl:84-200`), which is
   the fallback if a driver mis-handles a cross-workgroup storage cursor (**unverified**). Both passes join
   step 1's `dispatch_range_unpacking`, so they run per view in every schedule for free. Over-allocation
   becomes exactly "capacity minus level-pick survivors" - no worse than Bevy's pre-cull bin sizing.
   ~250 lines (2 WGSL entry points, one pipeline, one bind group, per-view scratch buffers).
3. **Batch-set merging across batches (SERIOUS 5): no patch work needed after step 2e.** Batches with
   different slab pairs *cannot* merge (different bound buffers); batches that share a slab pair and bind
   group should be merged by the **producer** into one batch entity with more ranges. Producer rule plus a
   measurement; drop SERIOUS 5 from the patch backlog.
4. **#26161 generation (SERIOUS 8).** `GpuBatchedMesh3d` gains `pub content_generation: u32`, carried into
   `ExtractedGpuBatchedMesh` and `RenderMeshInstanceBatch`; new
   `BinnedRenderPhase::instance_batch_entities(&self) -> impl Iterator<Item = MainEntity>` over
   `instance_batches.values()`; in `render/light.rs:1515-1520`, after `h.write_usize(phase_entity_count)`,
   fold `(entity, generation)` for each (the system gains `Res<RenderMeshInstanceBatches>`). `has_casters`
   (`:1559`) stays `phase_entity_count > 0`; what changes is that the `members` hash now moves on a content
   change. ~60 lines, marker `own, GS: batch generation`.
5. **`is_active` out of `MeshCullingData` into a `MeshInput.flags` bit**, so it is honoured independently of
   `FRUSTUM_CULLING` with no extra buffer read. `materialize.wesl`, `mesh_preprocess.wesl`,
   `MeshCullingData`, `MeshFlags`. ~40 lines; a #25978 review suggestion (cleaner than the PR's choice).
6. **One materialize dispatch for all batches.** Today one `write_buffer` + `set_bind_group` +
   `dispatch_workgroups` per batch per frame (`materialize.rs:161`, `:202-211`); with a per-batch metadata
   *array* and the batch index from `workgroup_id.y` it becomes one. Worth it only if S3b/S4 measures many
   batches per space (C1: slab count decides). ~80 lines.

---

## 5. Blast radius

| Change | Touches the same code as | How we avoid breaking it |
|---|---|---|
| Step 1 (unpack in `early_gpu_preprocess`) | **#26149** (`Core3dShadows` list, `gpu_preprocess.rs:633-662`), **#26095** (`clear_scene_unpacking_buffers`), `sf_shadow_cache_bevy`'s `LampShadowView` (`render.rs:120-147`), `sf_sky_bevy` sun views | We *remove* a schedule registration rather than add three. #26149's list is untouched; `LampShadowView` keeps importing only `allocate_uniforms, unpack_bins, early_gpu_preprocess`. #26095 clears `SceneUnpackingBuffers` and the bin/uniform bind groups, not `RangeWorkItemBuffer::bind_group` (rebuilt unconditionally, `gpu_preprocess.rs:904-931`) - no interaction. |
| Step 1 | **own: skip view preprocessing** (`SkipGpuPreprocess`, `SkipRootViewRendering`) | `early_gpu_preprocess` already filters `Without<SkipGpuPreprocess>` on both queries (`:1007`, `:1016`); the new dispatch inherits it by construction. `SkipRootViewRendering` faces never reach `batch_and_prepare` (`gpu_preprocessing.rs:2297-2299`), so they reserve no ranges. |
| Step 2d (materialize bind group) | **#26095** (bind-group clearing), **own: shared image samplers** | The range buffer is per job, rebuilt on `BufferId` change exactly as the port's own job does (`materialize.rs:162-166`); add the 4th id to the comparison or a stale bind group survives a resize. `SharedImageSamplers` is `bevy_render/texture/gpu_image.rs` - disjoint. |
| Step 2e (batch sets) | **own: phase counts** (`batch_set_count`), `MultidrawableBatchSetPreparer` (`:2754-2810`) | The init/uninit split of the shared `metadata` buffer is the hazard; our code stays in the first view loop and uses `allocate` (`push_multiple_init`). Add a `debug_assert!(spans.len() > 0)`. `batch_set_count` is updated in step 3 in the same slice. |
| Step 2f (`render_instance_batches`) | `render_range` (`render_phase/mod.rs:2136-2164`, the always-on sorted-phase change the handoff flags) | Different function; no shared state. The DX12 gate is copied verbatim from `render_batchable_meshes` so the two cannot diverge silently - put them in one `fn multi_draw_indirect_count_supported(world) -> bool`. |
| Step 3 (counters) | **own: phase counts**, **own: preprocess bind group count**, `sf_diag_bevy` | Additive fields only; `created` keeps its old meaning so the S0 baseline stays comparable. Report the new fields as separate columns in `perf.md`. |
| Step 4.1 (`is_active`) | **own: sun shadow early-out**, **H2**, **RO-11** | All of those are in `pbr_functions.wesl`, `shadow_sampling.wesl`, `shadows.wesl`, `ssr*` - no shared file with `mesh_preprocess.wesl`. |
| S3a3.1 (`geometry_index`) | **#26095**, `allocate_uniforms`, `reset_indirect_batch_sets`, **#26170** (`occlusion_culling.wesl`) | `IndirectParametersMetadata` lives in `occlusion_culling/mesh_preprocess_types.wesl`, which #26170 does **not** touch (it edits `occlusion_culling.wesl` and `downsample_depth.wesl`). Add the field at the **end** of the struct so every existing offset is unchanged. |
| S3a3.4 (generation) | **#26161**, **SH-20 / SH-20b** (`ShadowMoverRefresh`, same function in `render/light.rs`) | Fold the generation into the existing `members` hash only (`:1515-1520`), never into `motion`: SH-20b's per-light "moved light redraws whole" rule reads `motion`, and a streaming change must not look like a mover. |
| Crates keying lists by `Mesh3d` | `sf_shadow_cache_bevy/src/render.rs:490`, `sf_sky_bevy/src/sun_view.rs:181`, `mw_devtools_bevy` | **Unaffected by S3a2**: the batch pushes the `Mesh3d` class (`gpu_instance_batch/mod.rs:98`) and the shim fills the light lists (`:497-630`). **One real gap for S5**: `sf_sky_bevy/src/shadow_budget.rs:418` queries `(With<Mesh3d>, Without<NotShadowCaster>)` to compute the sun's layer mask, and a batch has no `Mesh3d` - a layer whose only casters are batches would be dropped. Log it for S5, do not fix here. |
| DLSS viewport patches (#26235/6/7, #26234) | - | No shared file (`clustered_forward.wesl`, `transmission.wesl`, `oit/*`, `ssao`). |

---

## 6. Tests - no GPU, no game, no data

Pattern: `sf_world_bevy/src/gpu_batch_tests.rs` (WGSL links with `wesl` and validates with naga for every
def set; Rust/WGSL layout assertions; a headless `MinimalPlugins` app). Extend that file; no new crate, no
new dependency.

- **T1 - shaders link and validate.** Extend the job list (`gpu_batch_tests.rs:120-151`) with `materialize`
  (now 5 bindings) and `unpack_ranges`, and with S3a3's pass A/B. Def-set sweep unchanged.
- **T2 - `GpuBatchGeometryRange` layout.** As `instance_and_culling_layouts_agree` (`:164-174`):
  `wgsl_size(&module, "GeometryRange") == GpuBatchGeometryRange::min_size() == size_of::<…>() == 64`, plus
  member-offset asserts for `aabb_center` (32) and `aabb_half_extents` (48) via `naga::proc::Layouter`.
- **T3 - the fields materialize overwrites still exist.** Assert by name that `MeshInputUniform` has
  `{world_from_local, flags, tag, first_vertex_index, first_index_index, index_count, metadata_index,
  material_and_lightmap_bind_group_slot}`, so an upstream rename fails a test rather than a frame.
- **T4 - `range_of` is a correct partition search.** Keep it as a pure
  `pub(crate) fn range_of(first_slots: &[u32], slot: u32) -> usize` in `gpu_instance_batch/mod.rs`, mirrored
  line-for-line in WGSL: exhaustive over every slot for randomised partitions of `0..256` into 1..17 ranges;
  property `first_slots[r] <= slot < first_slots[r + 1]`. WGSL cannot be executed headless, so only its
  compilation is machine-checked and the equivalence is reviewed by eye - say so in the test doc.
- **T5 - range validation rejects bad input.** Keep validation pure
  (`validate_ranges(&[GpuBatchedMeshRange], max_capacity) -> Result<(), RangeError>`); table-test
  overlapping / gapped / unsorted / over-long / empty, plus one headless case on `app()` (`:179-190`)
  asserting a bad batch extracts with `ranges` cleared.
- **T6 - batching arithmetic (the most valuable test).** `prepare_instance_batch_indirect` stays a free
  function over `UntypedPhaseIndirectParametersBuffers` + `UninitBufferVec` (both `RenderDevice`-free until
  `write_buffer`). Spans `[(100,4),(104,6),(110,2)]` ⇒ (a) three metadata entries with `base_output_index`
  = base, base+4, base+10; (b) one shared `batch_set_index`; (c) one batch set, `indirect_parameters_base ==
  ip_base`, range length 3; (d) `data_buffer.len()` grew by exactly 12 - the no-over-provisioning property.
- **T7 - the counters move.** Hand-built `BinnedRenderPhase` (`add` with
  `BinnedRenderPhaseType::InstanceBatch`): `bin_count`, `entity_count`, `batch_set_count`,
  `instance_batch_draw_count`, `instance_batch_indirect_draw_count`, `instance_batch_slot_count` before and
  after a synthetic `instance_batch_draws.push`; and that `prepare_for_new_frame` (`:1551-1555`) resets them.
- **T8 - the DX12 / no-MDI-count fallback.** Unit-test the extracted
  `fn instance_batch_extra_index(&PhaseItemExtraIndex, supported: bool) -> PhaseItemExtraIndex`:
  `false` ⇒ `batch_set_index == None`, range preserved.
- **T9 - GPU vs CPU twin (workstation, `#[ignore = "needs a GPU"]`, written with S3b).** Headless `App` with
  a real `RenderDevice`, one batch of 3 ranges over 64 slots, fixed camera,
  `IndirectParametersBuffersSettings { allow_copies_from_indirect_parameter_buffers: true, .. }`
  (`gpu_preprocessing.rs:1118-1125`), one frame, then read back (a) `indexed.data_buffer()` → per-range
  `instance_count` / `first_index` / `base_vertex` / `first_instance`, (b) `batch_sets_buffer()` →
  `indirect_parameters_count` == the number of non-empty ranges; compare counts and per-range membership
  with `sf_scene::cull::cull_cell` for the same camera (no images). Assert the `first_instance` spans are
  disjoint and contiguous - the property the whole design rests on.

Nothing here needs the game, game data, a picture, or a new dependency.

---

## 7. Upstreamability (CONTRIBUTING rule 4)

| Change | Offer as |
|---|---|
| Step 1 (unpack inside `early_gpu_preprocess`) | **#25978 review suggestion**: a bug in the PR as written (only `Core3d`), and the fix is smaller than the code it replaces. |
| Step 2 (`ranges`, per-range geometry/material/AABB, one batch set) | **Follow-up PR on #25978** ("multiple geometry ranges per GPU batch"). Empty `ranges` = current behaviour, so strictly additive. The headline contribution. |
| Step 2f (DX12 / MDI-count gate), step 4.1 and S3a3.5 (`is_active`), S3a3.6 (single dispatch) | **#25978 review suggestions**; 2f is a validation/crash bug in the PR. |
| Step 3 counters | **Scrib-specific** for now (already flagged as "upstream exposes phase counts" in `vendor/README.md`). Keep behind the existing `own: phase counts` / `own: preprocess bind group count` markers so the two bodies of work stay separable. |
| S3a3.1 (`geometry_index`), S3a3.2 (count/bump/scatter) | **Follow-up PRs** in step 2's series; 2 is the most speculative, offer last. |
| S3a3.4 (batch generation in the shadow hash) | **Scrib-specific** (#26161 is ours-carried). Marker `own, GS: batch generation`; keep it out of the #25978 diff file. |

Mechanics: all #25978-series hunks keep the existing marker
`// superfargoth patch (bevyengine/bevy#25978)`; new additions get
`// superfargoth patch (own, GS: <tag>)` with `<tag>` ∈ {`range unpack in preprocess`, `ranges`, `counters`,
`guards`, `lod draws`, `batch generation`}. After each step, refresh
`patches/bevyengine-bevy-25978-gpu-batched-mesh.diff` for the PR-derived hunks only and add a row/bullet to
`superfargoth/vendor/README.md` "The GPU-batched mesh port" listing the `own, GS:` tags. No crate is added:
**no `Cargo.toml`/`Cargo.lock` change anywhere in this plan.**

---

## 8. Owner / coordinator decisions (flagged, not taken)

- **O1 (spec edit, coordinator).** C1-C4 in §2. C1 and C2 change §6's wording materially.
- **O2.** S3a2 is two agent slices (§3 "S3a2 size"). Confirm the split, or accept one ~750-line slice with
  one build.
- **O3.** Does S3b spawn one batch per (class, slab pair) from the start (C1), or one per class with a
  `warn_once` when a class spans slabs? **Recommended default: per (class, slab pair)** - the validator in
  2c makes the wrong choice loud but silent-not-drawing, and S3b's whole point is numbers.
- **O4.** `sort_binned_render_phase` does not sort `instance_batches` (`batching/mod.rs:218-223`), so static
  draw order is `IndexMap` insertion order: no front-to-back ordering, no frame-to-frame determinism.
  **Recommended default: leave it** (opaque statics are depth-tested and the Hi-Z pass in S6 gives the
  ordering benefit properly); add `instance_batches.sort_unstable_keys()` only if S6 measures overdraw.
- **O5.** Hardware floor: statics need `GpuPreprocessingMode::Culling`. On a `PreprocessingOnly` adapter the
  choice is "no statics" or "fall back to the entity path". **Recommended default: `--off scene` behaviour
  automatically** (the entity path lives until S9 anyway) plus one startup `warn!`, and name the floor in
  §Risks. Steam Deck (RDNA2/Vulkan) supports culling; the laptop's DX12 backend supports indirect but not
  `multi_draw_indirect_count`, which step 2f covers.
- **O6.** Owner decisions I am **not** reversing, noted because the plan brushes them (GPL, Delirium
  default, Steam Deck floor, spec before code, no quick fixes): "no quick fixes" is why §4.2 is a real GPU
  count/allocate/scatter stage rather than over-reserving N × capacity, and why per-(cell, chain) LOD
  quantisation - which would have made every count CPU-known and removed S3a3 entirely, at the cost of §4
  step 5's per-instance rule - is **named and rejected** here rather than quietly taken.
