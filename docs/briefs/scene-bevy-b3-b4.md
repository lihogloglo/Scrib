# Brief: scene-bevy B3 and B4 (batch formation, and the render-world writer)

Written 2026-11-10 on `static-scene/brief-b3-b4` off `main` = `e3112be`. Research only: nothing was compiled, no
cargo command was run (the coordinator owns the single build slot). Every `file:line` below was re-read in the tree
at `e3112be`; a claim I could not settle without a compiler says so.

Reads, in this order: §0 and §1 of this brief, then `docs/briefs/scene-core-a5-a6.md` §2-§3 (the as-built `sf_scene`
API and the fragmentation measurement), then `superfargoth/docs/specs/scene-runtime-plan.md` §4.5/§6/§7 **with §3 of
this brief open beside it** — the plan is stale in twenty-two places that matter to B3/B4.

Run every cargo command through `bash bevyrrowind/tools/cargo_turn.sh`. It **always exits 0** (it loses cargo's exit
code, `bevyrrowind/tools/cargo_turn.sh:25`): judge every run by output — `^error`, `could not compile`,
`test result:`, `warning:`, `Finished` — never by `$?`. Never `--workspace`; always `-p <crate>`. Always
`--all-targets` on clippy (without it `cfg(test)` is never linted; that found real lints twice). Workspace lints put
`clippy::pedantic` at warn already, so `-D warnings` denies it — do **not** pass `-W clippy::pedantic`. The repo is
**not** `cargo fmt`-clean and fmt is not in the gate: match the surrounding style (~120 columns, single-line struct
literals), never run `cargo fmt`.

---

## 0. Where it stands, and what S3b can honestly deliver

| Step | State at `e3112be` |
|---|---|
| A1-A2 (`product.rs` out of `feature = "bake"`, table format v3) | **done**, `sf_scene/src/{lib,product,table}.rs` |
| A3-A4 (`ChainRegistry`; `alloc.rs`: `SlotAllocator` + `ClassAllocator`) | **done**, `sf_scene/src/{chains,alloc}.rs` |
| A5 (`SceneSpace`, `CellPlan`, `BatchKey`, `batch_ranges`, `SceneError`) | **done**, `sf_scene/src/space.rs` (1 368 lines) |
| A6 (v3 bake, `ChainsStep`, `bake_chains`) | **done**, `sf_scene/src/bake.rs` |
| Patch GS (slice A: unpack-in-preprocess, counters, guards; B-i: the `ranges` data path; B-ii: the drawing half) | **done and landed**, `superfargoth/vendor/README.md:51-153` |
| B1-B2 (crate `sf_scene_bevy` + the `sf_world_bevy` seam) | briefed separately; **assume they exist** when B3 starts |
| **B3** | `SceneSpaces`, chain asset loading + the per-slot side table, batch entities and their `ranges`, materials, `poll_cell_loads`, the `CellProxies` impl, every fallback path |
| **B4** | `extract_scene_uploads`, `prepare_scene_buffers`, `publish_scene` — the M1 slot writer |
| B5-B6, C1-C2 | after B3/B4 |

**S3b's honest ceiling. Say this in the CHANGELOG line and in the visual-check entry, or the first measurement will
read as a failure.** S3b delivers exactly three things:

1. **The CPU-cost win.** 46-55k static entities, their transforms, their visibility and their per-view draw items
   stop existing. `views/<kind>/items` collapses to roughly one per batch (`sf_diag_bevy/src/views.rs:147` feeds it
   from `BinnedRenderPhase::entity_count`, which counts one per instance batch,
   `vendor/bevy_render/src/render_phase/mod.rs:1545-1547`); `views/<kind>/instance_slots` is where the instances went.
2. **Residency.** Cells stream their statics in and out through two retained GPU buffers, with no entity spawns.
3. **Promotion.** A placement can leave the scene for the entity path mid-frame, by one 4-byte write.

S3b delivers **none** of: our own cull, the horizon test, the sub-pixel reject, per-view per-instance LOD. All four
need per-view work items, which need the GPU count → bump-allocate → scatter pass of `gs-patch-plan.md` §4.2 (S3a3),
which is **parked**. Under M1 the only per-view work is Bevy's own GPU frustum + Hi-Z cull
(`vendor/bevy_pbr/src/render/mesh_preprocess.wesl:217-248`, and `:277+` under `@if(OCCLUSION_CULLING)`), there is one
LOD level per instance for every view (LOD0 — `sf_scene/src/space.rs:82` `M1_LEVEL`, `:797-803` `top_level`), and the
*reserved* per-view output cost is sized by capacity, not by survivors. So the expected S3b result is **a large CPU
win, a GPU cost that is flat to slightly worse than the entity path, and a VRAM bill set by capacity** (§1.3). That
is the designed outcome of the milestone, not a regression.

---

## 1. Read this first: three structural findings

Nothing in §4.5/§6/§7 is *impossible* on the as-built tree — the old `static-scene-s3b.md` failure mode ("one batch
entity per (space, class)", which `GpuBatchedMesh3d` structurally could not express) does not recur, because A5
already carries the five-part batch key and the GS patch already carries `ranges`. But three things are materially
harder than the plan says, and the first decides B3's shape.

### 1.1 Slab identity lives in the render world only. B3 cannot key a batch without a feedback path.

The batch key the coordinator fixed is `(space, material class, vertex slab, index slab, material bind group)`. The
port enforces it **per frame, in the render world**, by comparing every range's slabs against the slabs of the
batch's own `mesh`:

```rust
// vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:780-788
    let slabs = allocator.mesh_slabs(&range.mesh);
    let (Some(slabs), Some(batch_slabs)) = (slabs.as_ref(), batch_slabs) else {
        return Err("has no allocated slabs yet");
    };
    if slabs.vertex_slab_id != batch_slabs.vertex_slab_id
        || slabs.index_slab_id != batch_slabs.index_slab_id
    {
        return Err("is not in its batch's vertex and index slabs");
    }
```

`MeshAllocator` is a **render-world-only** resource: `MeshAllocatorPlugin::finish` does
`render_app.init_gpu_resource::<MeshAllocator>()` (`vendor/bevy_render/src/mesh/allocator.rs:214-221`) and
`allocate_and_free_meshes` runs in `Render` (`:249-252`). `mesh_slabs()` is `pub` (`:345`) but there is no main-world
copy. The bind group likewise: `RenderMaterialBindings::get(&material.asset_id)` (`mod.rs:666`, `:808-813`).

**Consequence, and it is the whole of B3's design risk.** `SceneSpace::plan_in` takes the resolver as an argument —
`plan_in(&mut self, cell: IVec3, table: &StaticCellTableView<'_>, slabs: &impl ChainSlabs)`
(`sf_scene/src/space.rs:384-389`) — and A5 anticipated exactly this: `ChainSlabs::slabs(chain, level) ->
Option<DrawSlabs>` returning `None` means "not resolvable yet", and such a chain gets a batch of its own
(`space.rs:94-110`; `BatchKey::chain`, `:137`; `UNRESOLVED`, `:76`). `impl ChainSlabs for ()` resolves nothing
(`:105-109`). So B3 *compiles and runs* with `&()`. But:

- With `&()`, **every chain level gets its own batch.** §1.3's arithmetic rules that out.
- A batch key is decided once, at the cell's `plan_in`. A chain first referenced before its slabs resolve stays in
  its own `BatchKey { chain: Some(..) }` batch **for the whole life of the chain slot** — there is no re-keying path
  in A5, and adding one would move a cell's blocks between batches, i.e. re-upload everything. So "resolve later" is
  not a recovery; B3 must resolve **before** the first `plan_in` that references the chain.
- A *stale* key is not an error either: `resolve_gpu_ranges` returns `None` and the batch "draws nothing this frame"
  with one `warn_once!` (`mod.rs:723-751`). One wrong slab id silently blanks **every chain in that batch**.

**The mechanism, and it is sanctioned rather than an injection.** In `ExtractSchedule` a render-world system may take
`ResMut<MainWorld>`; two of our crates already publish render-world facts back that way —
`sf_surface_bevy/src/cells.rs:143-150` and `sf_terrain_bevy/src/render.rs:33`, both named `extract_writes`. So B3
gets a render-world `extract_chain_slabs` system that reads `MeshAllocator` + `RenderMaterialBindings` (both
legitimately `pub`, both read-only — no private state, no ordering surgery) and writes a main-world
`ChainSlabTable`. One frame of lag is safe, and the reason is load-bearing:

- a slab id is recorded once per allocation (`SlabAllocator::record_allocation`,
  `vendor/bevy_render/src/slab_allocator.rs:915-917`) and is never rewritten while the allocation lives;
- growth keeps the slab id and only moves the *offsets* inside it (`grow_if_necessary`, `:1133-1163`;
  `keys_displaced_by_slab_growth`, `:623`), and the port re-resolves offsets every frame anyway (`mod.rs:617-721`);
- so **per-mesh slab identity is stable from allocation to free.** A one-frame-late snapshot cannot be *wrong*, only
  *absent* — and absent is precisely what `ChainSlabs` already models as `None`.

`large_threshold` is 256 MiB by default (`slab_allocator.rs:163-171`), far above any chain product, so chain meshes
share general slabs (`min_slab_size` 1 MiB, `max_slab_size` 512 MiB) and the class → slab-pair fan-out should be
small. **Make it a measured quantity**, which is what `gs-patch-plan.md` C1 asks for: a `scene/batches/count`
counter plus the `#[ignore = "measurement"]` walk of §6.3.

Industry name for what this is: the standard "cached mesh draw command keyed on the vertex factory / buffer
binding, invalidated when the allocation moves" that UE and Unity both use. We get the invalidation for free, because
the port re-validates per frame and tells us in the log.

### 1.2 `poll_cell_loads`'s readiness condition is one level stricter than the plan says

The plan's `poll_cell_loads` plans "a cell whose table and chains resolved". A `Handle<Mesh>` being loaded in the
main world is **not** the same as that mesh being allocated in a slab: allocation happens in the render world's
`allocate_and_free_meshes` (`vendor/bevy_render/src/mesh/allocator.rs:249`) the frame *after* the mesh is extracted.
The real gate is three-deep:

1. the cell's `SceneTableAsset` resolved;
2. every chain's `CookedModelAsset` resolved and its per-level `Handle<Mesh>` present in `Assets<Mesh>`;
3. **every chain level's mesh present in `ChainSlabTable`** (§1.1) — the render world has seen and allocated it.

Step 3 adds about two frames of latency per *new* chain, once, and none for a chain another resident cell already
holds. That is well inside the plan's "stream-in ≤ 0.2 ms" budget because it is latency, not CPU work — but it has
to be designed in: `CellLoad` needs a third state, not two.

### 1.3 The capacity bill, computed. `batches × batch_capacity` is the number that matters.

Verified sizes, per *reserved* instance slot:

| Buffer | Bytes/slot | Multiplicity | Where |
|---|---|---|---|
| `GpuMeshInstance` output | **64** | once per batch | `gpu_instance_batch/mod.rs:79-88`; buffer at `:854-865` |
| `MeshInputUniform` scratch | **112** | once, global | `render/mesh.rs:614-679`; reserved at `mod.rs:714-717` |
| `MeshCullingData` | **32** | once, global | reserved at `mod.rs:716` |
| `MeshUniform` output | **176** | **per view** | `render/mesh.rs:563-606`; `data_buffer.add_multiple(count)`, `gpu_preprocessing.rs:2654`+ |

A reserved slot therefore costs `208 + 176·views` bytes. At the eight views an exterior frame runs (camera +
cascades + sun + lamp faces) that is **1 616 B per reserved slot**, and `gs-patch-plan.md` §1.3's "output space is
allocated per range, summing to the capacity" means *reserved*, not *live*: nothing is right-sized until S3a3.

- 100 000 reserved slots × 8 views ⇒ **162 MB**. The architecture review's ≈140 MB is the `MeshUniform` part alone:
  100 000 × 176 × 8 = 141 MB — **CONFIRMED**.
- A Steam Deck budget of roughly 100 MB for this gives **≈ 64 000 reserved slots, total, for the whole space.**

With `SpaceConfig::batch_capacity` at its default 4096 (`space.rs:289-291`) that is **≈ 16 batches**. Hence:

- **`BatchGranularity::PerChain` is not a shippable configuration.** Hundreds of resident chains × 4096 slots is
  hundreds of megabytes. It stays what A5 calls it: the one-range fallback and the test oracle (`space.rs:113-125`).
  **B3 must not ship with `&()` as its resolver.**
- **The real knob is the product, not the factor.** `SpaceStats::batch_capacity` already reports
  `batches × batch_capacity` (`space.rs:698`) — wire it to `scene/slots/batch_capacity` in B6 and treat it as the
  VRAM gauge it is. Its doc comment calls those slots "**per view**", which is loose: the 64 B output slot and the
  112 B `MeshInputUniform` are reserved once; only the 176 B `MeshUniform` is per view (correction, §3 item 8).
- **`AllocError::Exhausted` → entity fallback is a hot path, not a corner.** First fit measured
  `high_water / peak_live = 1.17` on the A4 walk (`docs/briefs/scene-core-a5-a6.md` §3), so a batch holding ~3 500
  live instances is already near 4096. `plan_in` turns `Exhausted` into `CellPlan::overflowed` plus `NO_SLOT` in
  `batch_slots` and leaves the rest of the cell resident (`space.rs:447-449`); the placement then spawns **whole** as
  an entity, and its other parts' slots are written `active: false` (`space.rs:464-479`). B3 must exercise that path
  in a test with a deliberately tiny `batch_capacity` — see §6.2 — not hope it never fires.
- Do **not** re-litigate the allocators. As built, a batch's slots are `SlotAllocator` (first fit) too: the
  coordinator's 2026-11-10 decision, recorded verbatim in `sf_scene/src/space.rs:33-46`, supersedes the
  "`ClassAllocator` per batch" line this slice was briefed with. `Batch { key, slots: SlotAllocator, blocks }`,
  `space.rs:293-303`. Flagged in the report, not changed here.

---

## 2. The API contract, quoted

### 2.1 `GpuBatchedMesh3d` — `vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:124-146`

```rust
#[derive(Component, Clone)]
#[require(Transform, NoFrustumCulling, Visibility, VisibilityClass)]
#[component(on_add = add_visibility_class::<Mesh3d>)]
pub struct GpuBatchedMesh3d {
    pub mesh: bevy_asset::Handle<Mesh>,
    pub max_capacity: u32,
    /// Geometry ranges, sorted by [`GpuBatchedMeshRange::first_slot`] and
    /// partitioning `0..max_capacity` without gaps or overlap.
    pub ranges: Vec<GpuBatchedMeshRange>,
}
```

Plan §6's component rules are **CONFIRMED**, with one stale citation and one line that must be deleted:

- `max_capacity == 0` is skipped entirely on extraction (`mod.rs:529-531`) — a batch with no blocks yet must still
  carry a non-zero capacity, or it never reserves.
- **never `Mesh3d`**: `warn_once!("GpuBatchedMesh3d and Mesh3d must not coexist on {entity:?}")`, `mod.rs:523-525`.
- **never `NoCpuCulling`**: `warn_once!("NoCpuCulling removes GpuBatchedMesh3d {entity:?} from every view")`,
  `mod.rs:526-528`, and `add_gpu_batched_meshes_to_light_visibility` filters `Without<NoCpuCulling>`
  (`mod.rs:901-909`).
- `MeshMaterial3d<M>` is **required in practice**, and the plan's citation `mod.rs:415-420` is stale: the real site
  is `mod.rs:663-668` — `materials.instances.get(&entity)` then `bindings.get(&material.asset_id)`, with `continue`
  on either miss, i.e. no material ⇒ the batch reserves but never draws. The type's own doc says it: "Pair with
  `MeshMaterial3d<M>`, not `Mesh3d`" (`mod.rs:97`).
- **`NotShadowCaster` is no longer needed — delete that line from plan §6.** Audit §A4's mitigation was for
  BLOCKER 2; GS slice A step 1 fixed BLOCKER 2 structurally by moving the range-unpacking dispatch inside
  `early_gpu_preprocess`, so every preprocessing schedule unpacks (`vendor/README.md:109-114`, and the comment that
  replaced the old `Core3d` registration, `mod.rs:396-407`). Adding `NotShadowCaster` now would *remove* every
  static from every shadow map (`mod.rs:893-909` filters `Without<NotShadowCaster>`) — a visible bug, not a
  safety net.
- `Transform::IDENTITY` comes free from `#[require(Transform, ...)]`; the plan's explicit insert is harmless.

### 2.2 The range list and its invariant — `mod.rs:148-241`

```rust
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
    /// `RenderMaterialBindings::get` is keyed by `UntypedAssetId`.
    pub material: Option<bevy_asset::UntypedAssetId>,
}
```

Every field name and type in plan §1 item 1 is **CONFIRMED**, in that order.

`index_count: 0` = "the mesh's own count", and `first_index` relative to the mesh's own allocation: **CONFIRMED** in
the resolver, `mod.rs:766-806` —

```rust
    let index_count = if range.index_count == 0 {
        geometry.index_count.checked_sub(range.first_index).ok_or("starts past the end of its mesh")?
    } else { range.index_count };
```

— with the slab-relative add `geometry.first_index_index + range.first_index` at `:790-798`. So A5's
`{ mesh: meshes[level], first_index: 0, index_count: 0 }` (`space.rs:154-158`) needs **no arithmetic at all**, which
is exactly what makes §3.2's one-mesh-per-LOD-level chain products work. The same geometry reaches the shader as
`GpuBatchGeometryRange` (`mod.rs:815-829`) and is read per instance by a binary search in
`gpu_instance_batch/materialize.wesl:44-57` / `:59-92`.

**The validator exists, is `pub`, and our test can call it.**
`pub fn validate_ranges(ranges: &[GpuBatchedMeshRange], max_capacity: u32) -> Result<(), RangeError>` at
`mod.rs:207-241`; `pub enum RangeError { EmptySpan, NotContiguous, SlotOverflow, CapacityMismatch }` at `:176-196`;
`pub fn range_of(first_slots: &[u32], slot: u32) -> usize` at `:247-259`; `pub fn batch_spans(...)` at `:270-287`.
All are in `bevy_pbr`'s `pub mod gpu_instance_batch`.

So §11's `batch_ranges_partition_the_capacity_with_no_gaps` **can** be driven through gs's own validator — but
**not from `sf_scene`**, which is Bevy-free and must stay so (`CONTRIBUTING.md:73-74`). Verdict: keep the
hand-written partition assertion in `sf_scene`'s own `space` tests, and add a `sf_scene_bevy` test that runs a real
cell plan's `batch_ranges` through `validate_ranges` (that crate already depends on `bevy_pbr`). That is the only
arrangement that is both honest and Bevy-free where it has to be. Note the port already has
`validate_ranges_accepts_exactly_the_partitions` (`sf_world_bevy/src/gpu_batch_tests.rs:309`) and
`range_of_is_a_correct_partition_search` (`:332`), so the new test must assert **our producer's output**, not the
validator — duplicating the validator's own test would be vacuous.

Rules the list must satisfy, from the validator body: non-empty spans; `first_slot[i] == first_slot[i-1] +
slot_count[i-1]`; the last range ends at `max_capacity`; and **`ranges: vec![]` is legal and means upstream #25978**
(the whole batch draws its own `mesh` across the whole capacity). A5's `batch_ranges` satisfies all of it by
construction — one range per live block, free slots merged into the preceding range, the leading gap into the
following one, the last range stretched to the capacity (`space.rs:618-638`).

**The empty case is B3's one real footgun.** A batch with no live block yields `vec![]`, which the port reads as
"draw `mesh` across all `max_capacity` slots". That is harmless *only because* every slot of such a batch is
`is_active = 0`. If B3 creates a batch entity before its slots are written, one frame of uninitialised GPU memory
draws the batch's `mesh` up to 4096 times. **Zero the whole capacity on a batch's first frame** (§4.3 step 5, and
the test in §6.2).

A bad list is **not** fatal: extraction validates once per change, `warn_once!`s and clears `ranges`, which "is
exactly upstream behaviour rather than a dropped batch" (`mod.rs:532-561`). Loud in the log, wrong on screen.

### 2.3 `GpuMeshInstance` — `mod.rs:79-88`

```rust
/// One instance slot published by a GPU producer. Zeroed slots are inactive.
#[derive(Clone, Copy, Default, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GpuMeshInstance {
    pub world_from_local: [Vec4; 3],
    pub is_active: u32,
    pub tag: u32,
    pub pad: [u32; 2],
}
```

64 B, with `is_active` at **offset 48** and `tag` at 52. Plan §7's "a 4-byte `write_buffer` at `slot * 64 + 48`" is
**CONFIRMED** (`COPY_BUFFER_ALIGNMENT` is 4, so a lone word is a legal write). `world_from_local` is the *affine
transpose* — three `Vec4` rows; the shader rebuilds the linear part from the first three lanes
(`materialize.wesl:83-88`) — so B4 must write transposed rows, not a column-major `Mat4`. `tag` is the only other
per-instance field a producer owns, and `GpuMeshInstance::default().is_active == 0` is asserted by the port's own
test (`gpu_batch_tests.rs:214`).

### 2.4 The batch key, and where each part comes from

```rust
// sf_scene/src/space.rs:126-139
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BatchKey {
    pub space: SpaceId,
    pub class: MaterialClass,
    pub vertex_slab: SlabId,
    pub index_slab: SlabId,
    pub bind_group: SlabId,
    /// `Some((chain, level))` when this batch holds one chain level only: either
    /// [`BatchGranularity::PerChain`], or a chain whose slabs were [`UNRESOLVED`] at stream-in.
    pub chain: Option<(ChainSlot, u8)>,
}
```

| Part | Who supplies it | Where |
|---|---|---|
| `space` | `SpaceConfig::space` | `space.rs:280` |
| `class` | `ChainEntry::class`, baked into the v3 table | `chains.rs:71` |
| `vertex_slab`, `index_slab` | `MeshAllocator::mesh_slabs(mesh)` — **render world** (§1.1) | `allocator.rs:345`; `MeshSlabs`, `:117-130` |
| `bind_group` | `RenderMaterialBindings::get(material).group` — **render world** | `mod.rs:808-813` |
| `chain` | A5, when `ChainSlabs` returned `None` | `space.rs:137-139`, `:94-110` |

The port compares only `vertex_slab_id` and `index_slab_id` (not `metadata_slab_id`) and only `binding.group` (not
the slot), so `DrawSlabs` needs exactly three `u32`s, as A5 has it (`space.rs:85-90`).

---

## 3. Everything the plan and the handoffs get wrong

`scene-runtime-plan.md` line numbers are at `e3112be`.

**§1 and §4.5**

1. `:41-42`, `:294` — `plan_in` takes **three** arguments: `plan_in(&mut self, cell: IVec3, table:
   &StaticCellTableView<'_>, slabs: &impl ChainSlabs)` (`space.rs:384-389`). The plan's two-argument signature
   predates `ChainSlabs`, which is the whole of §1.1.
2. `:299` — `plan_out` returns `Result<CellRelease, SceneError>`, not `CellRelease` (`space.rs:547`); `promote`
   returns `Result<Vec<SlotWrite>, SceneError>` (`:569-574`); and there is a `cancel_in(&mut self, plan: &CellPlan)`
   the plan never mentions (`:533`) — the cancelled-load path of §5.3 **needs** it.
3. `:268-277` — `CellPlan` as built has **eleven** fields, not eight: it also carries `chain_slots` (the
   `write_remapped` argument) and `batch_slots`, and it does not carry `promotions`. Quoted in full at
   `space.rs:172-205`.
4. `:303` — `batch_ranges` returns `Vec<RangeSpec>` with `RangeSpec { first_slot, slot_count, chain, level }`
   (`space.rs:159-165`); the plan's inline comment has the tuple order right but not the type.
5. `:283` — "`write_remapped(...)` (`table.rs:425`)" is now **`table.rs:645`**, and it does **panic** unless
   `slots.len() == chain_count()` (`table.rs:645-646`: `assert_eq!(slots.len(), self.chain_count(), "one global slot
   per chain of the table")`). But **B3 never calls it**: `plan_in` does, at `space.rs:410-412`, and hands B3 the
   finished `CellPlan::scene_bytes`. For B3's error paths this means the panic is already unreachable from the Bevy
   side — the only way to trip it is to call `write_remapped` directly, so don't. The invariant B3 inherits instead
   is that `CellPlan::chain_slots` holds one slot per cell-local chain in index order *including overflowed ones*,
   which is why a `CellPlan` is safe to publish even when a batch was full.
6. `:291` — `SceneError` has five variants as predicted, but `Chain(ChainError)` is `#[error(transparent)]` and
   `ChainError` has three variants, not two (`chains.rs:96`; `docs/briefs/scene-core-a5-a6.md` §4). The
   `Resident`/`NotResident` pair means `poll_cell_loads` must consult `SceneSpace::resident(cell)`
   (`space.rs:363`) before planning, and `plan_out` only for a cell it knows is resident.
7. `:34` — "`is_active`'s `@if(FRUSTUM_CULLING)` nesting is fixed in gs step 4": **CONFIRMED, landed.** The gate has
   its own `@if` statement at `vendor/bevy_pbr/src/render/mesh_preprocess.wesl:212-215`, with twenty lines of comment
   above it (`:190-211`) explaining that it is defence in depth, not a fix.
8. `space.rs:247-249` — `SpaceStats::batch_capacity`'s doc says those slots are reserved "**per view**". Loose: the
   64 B output slot and the 112 B `MeshInputUniform` are reserved once; only the 176 B `MeshUniform` is per view
   (§1.3). Worth a one-word fix when someone next edits `space.rs`.

**§6**

9. `:435` — "`+ NotShadowCaster` until gs step 1 lands (audit §A4)". **Step 1 has landed; adding `NotShadowCaster`
   is now a bug** (§2.1).
10. `:435` — "(required in practice, `mod.rs:415-420`)": the site is `mod.rs:663-668`.
11. `:443-447` (the systems table) — `GpuInstanceBatchSystems::Publish` is **CONFIRMED** as the right home and as a
    **`RenderGraph`**-schedule set: `configure_sets(RenderGraph, (Publish, Materialize).chain()
    .after(RenderGraphSystems::Begin).before(RenderGraphSystems::Render))`, `mod.rs:429-439`. The plan's citations
    `mod.rs:226-235` and `:80-85` are stale: the set enum is `mod.rs:89-100`, and the materialize system is
    registered `.in_set(GpuInstanceBatchSystems::Materialize)` at `gpu_instance_batch/materialize.rs:178`, with its
    dispatch at `materialize.rs:319` (`capacity.div_ceil(64)`).
12. `:437` — "`mesh` is the first range's mesh". True, and load-bearing in a way the plan does not say: the port
    takes the *batch's own* `mesh` as the slab reference every range is compared against (`batch_slabs =
    allocator.mesh_slabs(&draw.mesh_asset_id)`, `mod.rs:745`) **and** as the `MeshInputUniform` template
    (`mod.rs:669-672`). So `GpuBatchedMesh3d::mesh` must be a mesh that is itself in the batch's slabs — one of the
    ranges' meshes — and it must be re-pointed whenever `batch_ranges` empties and refills. A batch whose `mesh`
    handle was dropped with its last block draws nothing, silently.
13. `:448` — "doubling, encoder copy as `mod.rs:479-494`": the growth-with-carry-over pattern is now
    `GpuInstanceBatchReservation::resize`, `mod.rs:867-879`. It is the right model: allocate the replacement,
    `encoder.copy_buffer_to_buffer` the `min` of the two sizes, `queue.submit`, replace.
14. `:456-459` — `frames_in_flight = 3` is on `SpaceConfig`, not `SceneConfig` (`space.rs:279-287`), and it is
    *already* the default (`:289-291`). The plan's "**unverified** — wgpu states no bound" caveat stands; I did not
    settle it either.
15. `:425-429` — the crate's dependency list is right in spirit, but B3 additionally needs `bevy_render` for
    `MainWorld` / `ExtractSchedule` (§1.1) and `bevy_asset` for `AssetId<Mesh>`; both are already listed.

**§7, §9**

16. `:466` — `promote` needs the **table view** as well as the cell and placement (`space.rs:569-574`), because it
    reads `table.find_all(placement)` (`table.rs:684`) and `table.levels(instance.chain)` (`:710`) to locate the
    block. So the cell's `SceneTableAsset` handle must stay alive for as long as the cell is resident — it is not
    just a stream-in input. **B3's residency bookkeeping must retain it**, or B5 cannot be written at all.
17. `:470-490` (the `is_active` hole) — **CONFIRMED on both halves; the coordinator's decision 9 is right.** A view
    that does not draw indirectly **drops instance batches entirely**: statics go *missing*, they never double-draw.
    - binned: `vendor/bevy_render/src/batching/gpu_preprocessing.rs:2643-2651` —
      `if no_indirect_drawing { warn_once!(...); continue; }` inside the `phase.instance_batches` loop;
    - sorted: `:2054-2063` — `if no_indirect_drawing || spans.is_empty() { ...; *phase.items[current_index]
      .batch_range_mut() = 0..0; continue; }`;
    - the port's own type doc says it in words: "such a view reserves nothing, queues no draw record and shows no
      batch - the statics are simply **absent** there, not drawn without culling" (`mod.rs:101-116`);
    - and the shader comment repeats it (`mesh_preprocess.wesl:199-209`).

    The architecture review's "double-draw" reading is **wrong**; `gs-patch-plan.md` §2's SERIOUS 7 row already said
    so. The plan's line numbers (`:2573-2576`) are stale — use `:2054` and `:2645`. Both sites now `warn_once!`
    instead of failing silently (`own, GS: guards`). Consequence: §7's promotion path has to guarantee **nothing
    extra**, and `promotion_is_safe_without_the_shader_fix` must be rewritten (§6.2) — asserting a doubling that
    cannot happen would be a vacuous test.
18. `:478` — the adapter floor is `GpuPreprocessingSupport::is_culling_supported()`
    (`gpu_preprocessing.rs:152-154`), a **render-world** resource built by `FromWorld` from the adapter
    (`:1684-1742`). B5's "startup `warn!` and behave as `--off scene`" therefore cannot be a main-world startup
    system: the support level must be published back to the main world the same way as the slab table, or the
    plugin must let the port's own `warn_once!("GpuBatchedMesh3d requires GPU culling support; nothing will be
    drawn")` (`mod.rs:604`) be the warning. Recommended default: the latter in B3/B4; the published-back
    `SceneSupported(bool)` in B5, where `suppress_scene_placements` needs it.
19. §9 — `scene/batches/instances` has no source in `SpaceStats`. The three numbers that exist are
    `batch_live_slots`, `batch_high_water` and `batch_capacity` (`space.rs:235-256`, `:655-700`). B6 should use those
    names.
20. A second hardware path the plan never mentions, and B3/B4 need do nothing about it: on **DX12** — a backend we
    ship — `multi_draw_indirect_count` is unavailable (wgpu #7974), and
    `render_phase::multi_draw_indirect_count_supported` (`vendor/bevy_render/src/render_phase/mod.rs:955-966`) makes
    `render_instance_batches` fall back to a plain `multi_draw_indirect` over all N commands (`:1280-1300`,
    `:1326-1400`). Every range still draws; only GPU-side command compaction is lost. Normal path, documented at
    `vendor/README.md:144-152`.

**The handoffs**

21. `bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md:49` — "**No motion vectors**: the template's
    `previous_input_index` is `u32::MAX`". **WRONG, and the audit already corrected it**
    (`superfargoth/docs/research/s3a-port-audit-2026-11-09.md:178`). `MeshInputUniform::for_mesh` sets
    `previous_input_index: u32::MAX` (`render/mesh.rs:701`) and the shader's else-branch then uses the *current*
    transform as the previous one:

    ```wgsl
    // vendor/bevy_pbr/src/render/mesh_preprocess.wesl:265-272
    let previous_input_index = current_input[input_index].previous_input_index;
    var previous_world_from_local_affine_transpose: mat3x4<f32>;
    if (mesh_changed_this_frame && previous_input_index != 0xffffffffu) {
        previous_world_from_local_affine_transpose =
            previous_input[previous_input_index].world_from_local;
    } else {
        previous_world_from_local_affine_transpose = world_from_local_affine_transpose;
    }
    ```

    A zero object-space delta plus a moving camera **is** the correct motion vector for a rigid static, which is
    what DLSS and TAA need. The coordinator is right; the handoff line should be struck. (Note the batch's
    `timestamp` is rewritten every frame — `template.timestamp = frame.0`, `mod.rs:675` — so
    `mesh_changed_this_frame` is always true and the else-branch is taken on the `u32::MAX` test alone.) What *is*
    missing is wind / UV-scroll history, which is S4 and has nothing to do with B3/B4.
22. `s3a-port-audit-2026-11-09.md:155` — the `NotShadowCaster` mitigation is **obsolete** (item 9).

**The counters (plan §13 Q8): both one-liners are fixed; the A/B is readable today.**
`BinnedRenderPhase::batch_set_count()` is `entity_path_batch_sets + self.instance_batch_draws.len()`
(`vendor/bevy_render/src/render_phase/mod.rs:1590-1602`); `entity_count()` adds
`self.instance_batches.values().map(IndexSet::len).sum()` (`:1545-1547`); `bin_count()` adds
`self.instance_batches.len()` (`:1574-1583`); and three new read-only counters exist —
`instance_batch_draw_count` (`:1610`), `instance_batch_indirect_draw_count` (`:1618`),
`instance_batch_slot_count` (`:1636`). `sf_diag` carries them end to end as
`PhaseCounts::{instance_batches, instance_draws, instance_slots}` (`sf_diag/src/views.rs:93-110`), summed per kind
and into `all` (`:216-253`), with its own tests (`:573-589`, `:660-671`); `sf_diag_bevy/src/views.rs:147` feeds
`items` from `entity_count()`. So in the A/B, `items` legitimately collapses to about one per batch while
`instance_slots` carries the instance count: **that is the win, not a blind counter.** §13 Q8 is closed.

---

## 4. B3, step by step

Files: `sf_scene_bevy/src/{lib,residency,chains,slabs}.rs`. `slabs.rs` is new relative to the plan (§1.1).

### 4.1 Resources

```rust
#[derive(Resource)] pub struct SceneSpaces(pub BTreeMap<WorldspaceId, SpaceState>);
pub struct SpaceState {
    pub space: SceneSpace,
    /// Indexed by `ChainSlot`; sized once from `SpaceStats::chains.high_water`.
    pub chains: Vec<Option<ChainAssets>>,
    /// Indexed by `BatchId` (A5 promises dense, never-reused ids, `space.rs:62-64`).
    pub batches: Vec<Option<Entity>>,
    pub loading: HashMap<IVec3, CellLoad>,
    /// Kept per resident cell: `SceneSpace::promote` needs the table view (§3 item 16).
    pub tables: HashMap<IVec3, Handle<SceneTableAsset>>,
}
pub struct ChainAssets {
    pub product: Handle<CookedModelAsset>,
    pub levels: Vec<Handle<Mesh>>,
    pub material: Handle<StandardMaterial>,
}
#[derive(Resource, Default)] pub struct ChainSlabTable(HashMap<AssetId<Mesh>, DrawSlabs>);
#[derive(Resource, Default)] pub struct SceneFrame(pub u64);
#[derive(Resource, Default)] pub struct SceneUploads { pub cells: Vec<CellUpload>, pub writes: Vec<SlotWrite> }
```

`ChainStats::high_water` **is** surfaced — `SpaceStats::chains: ChainStats` (`space.rs:239-241`) and
`ChainStats { live, pending, high_water }` (`chains.rs:136-143`) — so `chains` is sized once per frame with
`resize(stats.chains.high_water as usize, None)` and never grown per acquire. The A5/A6 brief §6 item 4 ask
**landed**; there is no gap to name.

`ChainSlabTable` implements `sf_scene::ChainSlabs` through the space's own registry: for `(chain, level)`,
`space.chains().entry(chain)` (`chains.rs:226`) names the `ChainEntry`, `chains[chain].levels[level]` names the
`Handle<Mesh>`, and the table answers for its `AssetId`. All three must be present or the answer is `None` —
exactly A5's contract. Keep the impl in `slabs.rs` as a small borrow struct over `(&ChainRegistry, &[Option<ChainAssets>], &ChainSlabTable)`.

`SceneTableAsset(pub Vec<u8>)` is a one-field asset with a loader registered for `CELLS_EXT`. There is no
`(pub Vec<u8>)` asset anywhere in `superfargoth/crates` today, so copy the *shape* from
`sf_lod_bevy/src/assets.rs:68-89` (`FarDirectoryAsset(pub FarDirectory)` + `FarDirectoryLoader`, `extensions() ->
&["sffar"]`) and keep the raw bytes instead of a decoded type, because `StaticCellTableView` borrows them (audit
§C). Register as at `sf_lod_bevy/src/lib.rs:178-185` (`init_asset` + `register_asset_loader`), and copy
`baked_extensions()` from `sf_lod_bevy/src/lib.rs:128-131`:

```rust
/// File extensions of far-field products for `sf_model_bevy::baked_source`.
pub fn baked_extensions() -> impl Iterator<Item = (String, String)> {
    sf_lod::products::EXTENSIONS.iter().map(|(s, e)| ((*s).to_owned(), (*e).to_owned()))
}
```

### 4.2 The hook

B3 implements **`CellProxies`**, `sf_world_bevy/src/far.rs:33-57`, the same trait `sf_lod_bevy` implements:

```rust
pub trait CellProxies: Send + Sync + 'static {
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3);
    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool;
    fn spawn(&self, world: &mut World, cx: &ProxySpawnCx<'_>) -> u32;
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3);
    fn kept_radius(&self, _world: &World, _worldspace: &WorldspaceId, _cell: IVec3) -> f32 { 0.0 }
    fn bytes(&self, _worldspace: &WorldspaceId, _cell: IVec3) -> u64 { 0 }
}
```

Call order is `acquire`, then `ready` until true, then `spawn`, and eventually `release` (`far.rs:30-32`). The five
methods run inside `drive_streaming` (exclusive), so `acquire`/`ready` get `&World` and `spawn`/`release` get
`&mut World`. `ProxySpawnCx` is `far.rs:20-28`; installation is `FarProxies` (`far.rs:60-66`). Map them:

- `acquire` → start the cell's table load and, for chains already known, their products. Insert the `CellLoad`.
- `ready` → the three-deep gate of §1.2. **Read-only**: it may not plan.
- `spawn` → one call of the planning path for this cell (§4.3 steps 4-9), returning the number of instances taken.
- `release` → the stream-out path (§4.3 step 10).
- `bytes` → `plan.scene_bytes.len() + writes * 64`, so the streaming budget sees the scene's cost.

If B1/B2 instead gave `sf_scene_bevy` the per-placement `PlacementSpawner` seam (`sf_world_bevy/src/spawn.rs:98-117`)
for suppression, that is the *other* half (B5's `suppress_scene_placements`), not this one.

### 4.3 Systems, sets, and `poll_cell_loads`

`pub enum SceneSystems { Frame, Residency, Promote }`.

| System | Schedule / set | Ordering |
|---|---|---|
| `tick_frame` (`SceneFrame += 1`, then `SceneSpace::advance(frame)` per space, `space.rs:639-648`) | `First`, `SceneSystems::Frame` | before everything else in the plugin |
| `poll_cell_loads` | `Update`, `SceneSystems::Residency` | `.after(StreamingSystems::Drive)` — `sf_world_bevy/src/lib.rs:66-74` |
| `apply_promotions` | `Update`, `SceneSystems::Promote` | `.after(SceneSystems::Residency)` |
| `extract_chain_slabs` (render → main, §1.1) | `ExtractSchedule` | before `extract_scene_uploads` |

`poll_cell_loads`, in order:

1. For each `(cell, load)` in `loading`, drop it if the cell is no longer wanted; if it had already been planned,
   `SceneSpace::cancel_in(&plan)` (`space.rs:533`) — the slots then wait out the frames in flight like any other
   release, which costs nothing because they were never uploaded.
2. Readiness (§1.2). Not ready ⇒ leave it; `scene/cells/loading`.
3. A chain product that **failed** ⇒ the *whole cell* falls back: release its acquired refs, `error!` once naming
   the chain key, `scene/chains/failed`, and the cell's placements spawn as entities because `covered` is never
   published (plan §5.3 row 3 — **CONFIRMED** as the right shape).
4. Ready ⇒ build the `ChainSlabs` view and call `SceneSpace::plan_in(cell, &table_view, &slabs)`. On `Err`, the cell
   falls back the same way and nothing is mutated: `plan_in` is atomic on its one reachable error
   (`space.rs:372-383`, and A5's `plan_in_is_atomic_on_error`).
5. For every `(batch_id, key)` in `plan.batches` (`space.rs:182`), make sure a batch entity exists. **Order matters,
   and this is §2.2's footgun**: spawn with `max_capacity = config.batch_capacity`, `MeshMaterial3d<StandardMaterial>`,
   no `Mesh3d`, no `NoCpuCulling`, no `NotShadowCaster`, `ranges: Vec::new()` for this one tick, **and queue a
   zero-fill of the whole capacity into `SceneUploads`** so the batch's first published frame has every slot
   `is_active = 0`.
6. Materials: once per `ChainEntry::material_key` (`chains.rs:74`), from the chain product's `materials[0]` through
   `ModelMaterials` + `sf_world_bevy::matclass::fold`. Copy `build_template`'s dedupe-by-description closure
   verbatim in shape — `stream.rs:1434-1457`:

   ```rust
   let mut material = |i: u32| -> Handle<StandardMaterial> {
       let desc = asset.materials.get(i as usize).cloned().unwrap_or_default();
       let key = format!("{desc:?}");
       cache.entry(key).or_insert_with(|| {
           let mut made = (mapper.0)(&desc, asset_server);
           if fold { crate::matclass::fold(&mut made); }
           materials.add(made)
       }).clone()
   };
   ```

   `fold` is `sf_world_bevy/src/matclass.rs:141-143`; `ModelMaterials(pub MaterialMapper)` is `spawn.rs:443` with
   its default mapper `standard_material` at `spawn.rs:453`; the resource is fetched inside a
   `world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| ...)` exactly as `stream.rs:957` and
   `stream.rs:1139` do. Doing it this way is what makes scene materials fold into classes identically to the entity
   path and keeps `--off matclass` meaningful.
7. `plan.scene_bytes` and `plan.writes` into `SceneUploads` (B4's input), then `commit_in(cell, &plan)`
   (`space.rs:509`), then record the cell's table handle in `tables` (§3 item 16).
8. **After** `commit_in`, rebuild `ranges` for every id in `plan.ranges` (`space.rs:197-199`) from
   `SceneSpace::batch_ranges(id)`, mapping each `RangeSpec` to
   `GpuBatchedMeshRange { mesh: chains[r.chain].levels[r.level].clone(), first_index: 0, index_count: 0,
   first_slot: r.first_slot, slot_count: r.slot_count, material }`, and **re-point `GpuBatchedMesh3d::mesh` at the
   first range's mesh** (§3 item 12). `material` is `None` when the batch's own `MeshMaterial3d` is the chain's
   material, `Some(handle.id().untyped())` otherwise; the port only requires a shared `binding.group`
   (`mod.rs:808-813`).
9. `plan.covered` back to the seam so the entity path skips those placements; `plan.overflowed` and
   `SceneSpace::take_overflow_warning()` (`space.rs:650`) into the counters and one `warn!`.
10. Stream-out: `SceneSpace::plan_out(cell)` ⇒ `CellRelease { scene, freed_chains, ranges }` (`space.rs:208-216`).
    Drop the `ChainAssets` of every `freed_chains` entry and the cell's table handle; rebuild `ranges` for every
    listed batch. **Despawn a batch entity only when `batch_ranges` has been empty for more than
    `frames_in_flight`** — respawning reallocates its output buffer (`mod.rs:867-879`), but keeping it forever
    spends §1.3's VRAM on nothing.

### 4.4 Every fallback path ends at the entity path

| Failure | B3's behaviour | Counter |
|---|---|---|
| no `scene.cells` product (not baked; a mod added a cell) | `ready` true, `covered` `None`, models load, spawners spawn; `debug!` once per cell | `scene/cells/missing` |
| table fails to parse (`TableError`) | as above plus `error!` once per cell with the error — a corrupt cache is a bug worth seeing | `scene/cells/failed` |
| a chain product missing or failed, or `ChainError::Disagreement` | the **cell** falls back entirely; acquired refs released; `error!` naming the key | `scene/chains/failed` |
| `plan_in` returns any `Err` | same, via `cancel_in` if a plan already existed | `scene/cells/failed` |
| load cancelled between `plan_in` and `commit_in` | `cancel_in(&plan)`; the slots wait out the frames in flight | — |
| `AllocError::Exhausted` | the cell publishes **without** those instances; their placements are absent from `covered` so they spawn whole as entities, and their *other* parts' slots are written `active: false` (`space.rs:464-479`) | `scene/slots/overflow`, `scene/overflow/cells` |
| adapter is `PreprocessingOnly` | the port warns and draws nothing (`mod.rs:600-606`); B3 does not suppress — §3 item 18 | — |

---

## 5. B4, step by step

One file, `sf_scene_bevy/src/render.rs`. **Reuse, do not invent** — but note there is no single in-repo trio that
has all three parts in the shape B4 needs, so B4 takes two halves from two places.

### 5.1 The trio to copy

**Registration and extract: copy `sf_world_bevy/src/origin.rs`.** It is the closest shape in the tree, in our own
crate, and already proven in headless tests — `origin.rs:95-109`:

```rust
render_app
    .init_resource::<WorldOriginBuffer>()
    .init_resource::<WorldOriginUniform>()
    .add_systems(ExtractSchedule, extract_world_origin)
    .add_systems(Render, prepare_world_origin.in_set(RenderSystems::PrepareResources));
```

with `extract_world_origin` at `origin.rs:124-128` and `prepare_world_origin` at `:130-140`. It is a
`UniformBuffer`, not a storage buffer, so take only the registration and extract halves from it.

**The device-dependent resource: copy `sf_atmos_bevy/src/weather_map.rs:181-194`** — `RenderStartup` +
`bevy_render::init_gpu_resource::<WeatherMapGpu>` so the resource's `FromWorld` (`weather_map.rs:152-179`) can take
`RenderDevice`, plus a hand-written `extract` (`:196-198`) and a `prepare` in `RenderSystems::PrepareResources`
(`:205`) that creates `BufferUsages::STORAGE` buffers keyed on a generation (`:224-233`). This is the only place in
`superfargoth/crates` that creates a storage buffer in a prepare system.

**The growth-with-carry-over: copy `GpuInstanceBatchReservation`, `vendor/.../gpu_instance_batch/mod.rs:853-882`** —
`BufferUsages::STORAGE | COPY_SRC | COPY_DST`, and on resize: allocate the replacement, `copy_buffer_to_buffer` the
`min` of the two sizes, `queue.submit`, replace. The plan (`:450-451`) already names this as the template; the
citation is just stale.

**Extraction of a plain main-world resource: do NOT use `ExtractResourcePlugin`.** `sf_world_bevy` deliberately
hand-registers the generic instead — `sf_world_bevy/src/lib.rs:151`:

```rust
.add_systems(ExtractSchedule, extract_resource::<matclass::StaticMaterialIds, RenderApp, ()>)
```

because `ExtractResourcePlugin` logs an error when there is no render app, which is exactly the headless-test
configuration B3/B4's tests run in (reason recorded in
`bevyrrowind/docs/test-pass-handoffs/s2-material-classes.md:49-50`). `ExtractResourcePlugin` appears exactly once in
our crates (`sf_sky_bevy/src/env_refresh.rs:41`) and this is not the place for the second.

**Systems in the `RenderGraph` schedule**: present in our tree but only as timing stamps —
`sf_diag_bevy/src/phases.rs:103-104`, which takes no `RenderContext`. For *recording commands* the in-repo example
is `sf_surface_bevy/src/trail.rs:306-318` (a system taking `mut ctx: RenderContext` and calling
`ctx.command_encoder().begin_compute_pass(...)`, registered at `trail.rs:256`). `publish_scene`'s writes are
`RenderQueue::write_buffer` calls, so it needs `Res<RenderQueue>`, not a `RenderContext` — but it must still sit in
`GpuInstanceBatchSystems::Publish` (§5.3) so its writes are ordered before `Materialize`.

### 5.2 The three systems

| System | Schedule / set |
|---|---|
| `extract_scene_uploads` (take `SceneUploads` out of the main world; also the §1.1 slab publish-back, if it lives here) | `ExtractSchedule` |
| `prepare_scene_buffers` (create or grow each space's scene buffer) | `Render`, `RenderSystems::PrepareResources` |
| `publish_scene` (scene-range writes, batch slot writes, `is_active` flips) | `RenderGraph`, `GpuInstanceBatchSystems::Publish` |

The scene buffer resource is device-dependent, so register it with `RenderStartup` +
`init_gpu_resource::<SceneBuffers>` as `weather_map.rs:187` does.

### 5.3 What M1 writes, and when

`gs-patch-plan.md`'s M1 model is **CONFIRMED** against the shader: the CPU writes a batch's slots at stream-in and
leaves them. `is_active` carries residency and promotion **only**; Bevy's own per-view GPU frustum + Hi-Z cull does
all per-view work (`mesh_preprocess.wesl:212-215` for the `is_active` gate, `:217-248` for frustum and
visibility-range, `:277`+ under `@if(OCCLUSION_CULLING)` for Hi-Z); and the materialize pass re-publishes every
slot's transform and geometry into the shared `MeshInputUniform` scratch once per frame
(`materialize.wesl:59-92`, dispatched `capacity.div_ceil(64)` at `materialize.rs:319`, one thread per slot).

Three writes, and only three:

1. **Scene-buffer range** — `CellPlan::scene_bytes`, 80 B per instance, one contiguous `write_buffer` at
   `plan.scene.start * 80`. Ours, one buffer per space, `STORAGE | COPY_SRC | COPY_DST`, grown by doubling with the
   carry-over copy of §5.1. **Nothing reads it under M1** — it is the input to the S3a3 cull — so the honest framing
   in the commit message and the CHANGELOG line is "uploaded, not yet consumed".
2. **Batch output slots** — one `GpuMeshInstance` per `SlotWrite`, `write_buffer` at
   `slot * size_of::<GpuMeshInstance>()`, into `GpuInstanceBatchReservations::get(main_entity).output_buffer()`
   (`mod.rs:324-357`). Coalesce consecutive slots of one batch into one write: A5 hands them out in table order and
   a bucket's slots are contiguous (`space.rs:434-446`), so most cells are a handful of writes. The transform is the
   affine transpose (§2.3) of the instance's cell-relative transform plus the cell origin.
3. **`is_active` flips** — a 4-byte `write_buffer` at `slot * 64 + 48` (§2.3).

All three live in `GpuInstanceBatchSystems::Publish` (`mod.rs:89-100`, `:429-439`), which is in the **`RenderGraph`**
schedule, chained before `Materialize` and after `RenderGraphSystems::Begin`. That is the only correct home: it runs
after `RenderSystems::PrepareMeshes`, where the port creates reservations (`mod.rs:592-615`) and resolves ranges
(`:617-721`), and its commands are submitted before materialization. A reservation that does not exist yet (the
frame a batch is spawned) simply has no `get()` result — skip it and let the zero-fill retry next frame.

**Slot reuse and `frames_in_flight`.** A released block waits `frames_in_flight` frames before reuse
(`space.rs:639-648`, `SlotAllocator::advance`), so B4 never fences: by the time a slot is written again the GPU is
done with its old contents. Getting the number wrong costs one frame of a stale transform in a reused slot — never
UB, because the buffer is not resized. It is 3, and the plan's "wgpu states no bound" caveat stands.

**Origin shifts.** `cell_offset` (`sf_scene/src/lib.rs:52-55`) keeps the scene buffer cell-relative, so an origin
shift never touches it. The **batch outputs** are render-space and do need a rewrite, which plan §5.4 defers past
S3b (it becomes unnecessary if gs §4.2 lands first). B4 must not silently skip it: log it as a known gap in the
handoff and the ROADMAP.

---

## 6. Tests

### 6.1 `sf_scene` (Bevy-free, seconds) — A5 already wrote §11's list

B3/B4 add one case A5 could not have had: a `ChainSlabs` impl that resolves *some* chains and not others, asserting
that an `UNRESOLVED` chain lands in its own `BatchKey { chain: Some(..) }` batch while the resolved ones share one.
Everything else in §11's `space`/`chains`/`alloc`/`table` lists is in place.

### 6.2 `sf_scene_bevy` — headless, no render app

**The template has moved.** The plan and this slice's brief cite `sf_world_bevy/src/gpu_batch_tests.rs:178-190`;
that range is now `wgsl_struct()` / `wgsl_size()`. The headless template is at
**`sf_world_bevy/src/gpu_batch_tests.rs:377-387`**:

```rust
/// `MinimalPlugins` + assets + the plugin; no render app (no GPU), so only the main-world half runs.
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<Mesh>()
        .init_asset::<Shader>()
        .add_plugins(GpuInstanceBatchPlugin);
    app.finish();
    app.cleanup();
    app
}
```

The explicit `app.finish(); app.cleanup();` matters: `GpuInstanceBatchPlugin::finish` runs the render-app check
(`mod.rs:450-461`). Helpers to reuse: `sun()` `:391`, `batch()` `:402`, `batch_with()` `:407`, `cascade_lists()`
`:412`. A working example of the whole shape is `batch_draws_as_a_mesh3d_class_and_casts_sun_shadows_headless`
(`:418`).

| Test | Asserts | Kind |
|---|---|---|
| `a_resident_cell_spawns_its_batches_and_despawns_them_on_unload` | the batch entity count, and that each carries `GpuBatchedMesh3d` + `MeshMaterial3d` and **not** `Mesh3d` / `NoCpuCulling` / `NotShadowCaster` | exact |
| `batch_ranges_pass_the_ports_own_validator` | `validate_ranges(&batch.ranges, batch.max_capacity) == Ok(())` for every batch after a stream-in, a partial unload and a full unload. **This is §11's `batch_ranges_partition_the_capacity_with_no_gaps` driven through gs's own validator**, and it belongs here, not in Bevy-free `sf_scene` (§2.2). Assert the batch count first so an empty world cannot pass it | exact |
| `a_new_batch_zeroes_its_whole_capacity_before_its_first_range` | the `SceneUploads` of the frame a batch is created cover `0..max_capacity` | exact — guards §2.2's footgun |
| `chain_handles_outlive_the_first_cell_and_drop_with_the_last` | `ChainAssets` present after cell A unloads while B holds the chain; `None` after both | exact |
| `an_unresolved_chain_gets_its_own_batch_and_a_resolved_one_shares` | the `ChainSlabTable` → `BatchKey` path of §1.1 | exact |
| `promotion_queues_one_write_per_part` | one `SlotWrite { active: false }` per table instance of the placement | exact |
| `promotion_is_safe_without_the_shader_fix` | **rewrite from the plan.** Decision 9 is settled (§3 item 17), so a doubling assertion would test something that cannot happen. Assert instead the two facts that make promotion safe: (a) `INDIRECT ⇒ FRUSTUM_CULLING` — `direct_preprocess` is specialised with `PreprocessPipelineKey::empty()` while every other prepared key contains `FRUSTUM_CULLING`; (b) a `no_indirect_drawing` view queues no instance-batch draw at all. (a) is a pipeline-key fact assertable from `bevy_pbr`'s API **if** `PreprocessPipelineKey` and the specialisation are reachable from outside the crate — **I could not confirm reachability without a compiler.** If they are private, drop (a) and leave a comment citing `vendor/bevy_pbr/src/render/gpu_preprocess.rs`; do not replace it with something weaker that still passes. (b) is not unit-testable without a render app: assert instead that `GpuPreprocessingSupport::is_culling_supported` is the gate B5 reads | see note |
| `frames_in_flight_defers_every_reuse` | drives `SceneFrame` by hand; a released slot is not handed out before the window | exact |
| `a_missing_table_falls_back_to_entities` | `covered` empty, no batch entity, no panic | exact |
| `a_failed_chain_product_falls_back_the_whole_cell` | refs released, `scene/chains/failed` incremented | exact |
| `overflow_leaves_the_rest_of_the_cell_resident` | with `batch_capacity: 8`, the overflowing placement is absent from `covered` and its sibling parts are `active: false` | exact — §1.3 says this is a hot path |

`sf_world_bevy` (the B2 seam, re-run here because B3 changes what it feeds):
`scene_covered_placements_load_no_models_and_spawn_nothing`, `a_cell_without_a_scene_hook_streams_as_before`,
`release_runs_on_the_near_unload_and_after_a_cancelled_load`.

**Vacuity warnings — the repo already has ~100 install-gated tests that pass having asserted nothing, and no new
test may join them.** Three of the above are at risk and must be written so they cannot silently pass:

- **Anything that needs resolved slabs.** A headless app has no render app, hence **no `MeshAllocator`**, hence an
  always-empty `ChainSlabTable` and `UNRESOLVED` for every chain. Tests of the *resolved* path must inject a
  `ChainSlabTable` by hand and **assert its length first**. A test that passes because the table was empty is
  vacuous.
- **Anything that needs baked data.** Write synthetic tables through `StaticCellTable::write`, as
  `sf_scene/src/table.rs`'s own tests do. Never gate a new test on a Morrowind install.
- **`promotion_is_safe_without_the_shader_fix`** — see its row; it is the one test in the list that could end up
  asserting nothing.

Note that these are all logic-and-data tests (plan-and-upload bookkeeping), so **exact** assertions are required;
nothing here is a look-and-feel sanity test.

### 6.3 `#[ignore = "measurement: …"]`, owner's machine only

- the §8 `--scene` / `--off scene` A/B: `views/all/items`, `views/all/instance_slots`, `views/all/batch_sets`, and
  frame ms per preset. **Readable today** (§3, the counters paragraph).
- `scene/slots/*` and `scene/batches/count` after a 20-minute Balmora walk: §1.3's reserved-slot bill in the field,
  and the class → slab-pair fan-out that `gs-patch-plan.md` C1 asks to measure.
- frames until every static pipeline is resident (the review's "cold start").
- determinism: the same pose reached by flying vs. starting there gives the same resident cells, chain slots, batch
  list and `ranges`.

### 6.4 Visual check

Rendering work ends with the build ready plus a `bevyrrowind/docs/visual-checks/static-scene.md` entry and its index
line, status **"awaiting owner"**, naming camera, time, flags and concretely what to look for: statics present at
all; nothing doubled where a placement was promoted; statics present in the sun cascades and in lamp shadows; no
4096-fold copy of one mesh at a cell boundary (§2.2's footgun). **No agent judges any of this from a screenshot.**
The index line is in a shared file — report it, let the coordinator apply it.

---

## 7. Build economy

`sf_scene_bevy` is a Bevy crate, so every command is a build slot. Cheapest first; stop at the first `^error`.

```
# 1. Bevy-free, seconds. Run it first: it catches most API drift for free.
bash bevyrrowind/tools/cargo_turn.sh test   -p sf_scene --all-targets
#    judge by: "test result: ok. N passed"

# 2. Does it compile at all? No test harness, no test-binary linking.
bash bevyrrowind/tools/cargo_turn.sh check  -p sf_scene_bevy --all-targets
#    judge by: no "^error", no "could not compile"; then "Finished"

# 3. The tests.
bash bevyrrowind/tools/cargo_turn.sh test   -p sf_scene_bevy
#    judge by: "test result: ok." and the per-test names. "0 passed" is a red flag, not a pass

# 4. The gate. --all-targets is mandatory: without it cfg(test) is never linted.
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_scene_bevy --all-targets -- -D warnings
#    judge by: no "^error", no "warning:"

# 5. Only if B3/B4 touched the seam.
bash bevyrrowind/tools/cargo_turn.sh test   -p sf_world_bevy --all-targets
```

Never `--workspace`. Never `-W clippy::pedantic` (workspace lints already set it to warn, and `-D warnings` denies
it; passing it re-enables three lints the owner set to allow). Never `cargo fmt`. **Never launch the game**, and
never judge rendering from a screenshot.

Expect **four** Bevy build slots if B3 and B4 are one slice: one for B3's `check`, one for B3's `test`, one for B4's
`check`+`test`, one for the final `clippy --all-targets` plus the re-run of whatever clippy made you change. A new
plugin crate's first compile is rarely clean, so the first of those is the one that slips.

---

## 8. Open questions, each with a recommended default

1. **Does B3 ship the render→main slab feedback, or `&()`?** (§1.1, §1.3.) Default: **ship it.** `&()` is
   memory-infeasible at the default capacity, and the batch key cannot be re-decided later. It is ~60 lines
   (`extract_chain_slabs` + `ChainSlabTable` + the `ChainSlabs` impl), and the pattern exists twice in the tree
   already (`sf_surface_bevy/src/cells.rs:143-150`, `sf_terrain_bevy/src/render.rs:33`).
2. **`batch_capacity`, given §1.3's 1 616 B per reserved slot at eight views.** Default: **keep 4096, make the
   product visible** — `scene/slots/batch_capacity` from `SpaceStats::batch_capacity` (`space.rs:698`), with the
   owner-machine measurement of §6.3 — and make `Exhausted` → entity fallback a tested first-class path. Do not pick
   a new number without the measurement. **For the owner:** at 4096 the design affords about 16 batches inside a
   100 MB Steam Deck budget. If the measured class × slab-pair × bind-group fan-out exceeds that, the fix is a
   *total* slot budget in `SpaceConfig` with the per-batch capacity derived from the batch count — an A5 change, not
   a B3 one.
3. **Despawn an empty batch entity, or keep it?** Default: **keep for `frames_in_flight`, then despawn.** Respawning
   reallocates the output buffer (`mod.rs:867-879`); keeping it forever spends §1.3's money on nothing.
4. **Does B3 retain each resident cell's `SceneTableAsset` handle?** Default: **yes.** `SceneSpace::promote` needs
   the table view (§3 item 16), so B5 is unwritable otherwise. Cost: one handle per resident cell.
5. **Who re-points `GpuBatchedMesh3d::mesh` when the first range changes?** Default: `poll_cell_loads`, in the same
   step that rebuilds `ranges` (§4.3 step 8). One line, and its absence is a silently blank batch (§3 item 12).
6. **`SceneTableAsset` as `Vec<u8>` or a decoded type?** Default: **`Vec<u8>`**, because `StaticCellTableView`
   borrows the bytes (audit §C) — the first such asset in the framework, so follow
   `sf_lod_bevy/src/assets.rs:68-89`'s *shape* rather than its decode.
7. **B3 + B4 in one agent slice, or two?** See the report; recommended **two**, with B3 (and its `ChainSlabTable`)
   landing first so B4 has non-empty batches to write into.
8. **Spec corrections for the coordinator** (shared files, untouched here): plan §1 item 1's `plan_in` signature;
   §4.5's `CellPlan` and `plan_out`/`promote` return types; §6's `NotShadowCaster` line and its stale `mod.rs`
   citations; §7's `gpu_preprocessing.rs` line numbers; §9's `scene/batches/instances`; `space.rs:247-249`'s "per
   view"; `bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md:49`'s "no motion vectors"; and
   `s3a-port-audit-2026-11-09.md:155`'s obsolete `NotShadowCaster` mitigation. All twenty-two are itemised in §3,
   and §13 Q8 can be marked closed.
