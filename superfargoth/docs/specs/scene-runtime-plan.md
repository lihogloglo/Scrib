# Scene runtime plan: the CPU half of `sf_scene_bevy`

Work order, 2026-11-09. Owns the **CPU-side ownership model** of slice S3b: chain slots, chain
meshes, instance slots, the frame counter, the stream-in/out lifecycle, promotion, the `--scene`
switch, and where `bake_scene` is called. Nothing compiled; `file:line` for every claim about
existing code, "**unverified**" where inferred.
Reads: `docs/specs/static-scene.md`; `docs/research/s3a-port-audit-2026-11-09.md` (API contract);
`docs/research/static-scene-architecture-review-2026-11-09.md` (B2, B3, S7, S10);
`docs/specs/gs-patch-plan.md` (the sibling's GPU half — **landed while this was written and
reconciled below**). Where spec and audits conflict, the audits win.
## 1. What the GPU half gives us, and what follows
From `gs-patch-plan.md` §1, §2, 2a-2e:
1. A batch entity carries `ranges: Vec<GpuBatchedMeshRange>` in the **main world**, each
   `{ mesh: Handle<Mesh>, first_index, index_count, first_slot, slot_count, material }`, sorted by
   `first_slot` and **partitioning `0..max_capacity` with no gaps or overlap** (gs 2a); the port
   resolves each range's slab-relative geometry per frame through `MeshAllocator` and validates it
   (gs 2c). ⇒ **We own no GPU chain table and cache no slab offsets.** The range list *is* the
   geometry table, and it is plain main-world data.
2. `index_count: 0` means "the mesh's own index count", `first_index` is relative to that mesh's own
   allocation (gs 2a). ⇒ A range may be **one whole mesh**, which is what makes §3.2's per-level
   chain meshes work with no arithmetic.
3. `material: Option<UntypedAssetId>` is **per range**, constrained to the batch's bind group
   (gs 2a), and the batch key is `(space, class, vertex slab, index slab, material bind group)`
   (gs C1, O3 default). ⇒ The key carries the bind group, not the material instance.
4. Output (`MeshUniform`) space is allocated **per range, summing to the capacity** (gs §1.3), so
   per-view cost stays capacity-sized until the count/allocate/scatter pass of gs §4.2 (S3a3).
   ⇒ **Right-sizing capacity is our duty** (gs §2, SERIOUS 6): §4.4.
5. Each batch keeps **its own output buffer**; nothing in the GS plan shares one ⇒ batch slots stay
   retained and per batch in both milestones, so an allocator per batch is permanent, not interim.
6. `is_active`'s `@if(FRUSTUM_CULLING)` nesting is fixed in gs step 4, the hardware floor is gs O5
   ("fall back to the entity path on a `PreprocessingOnly` adapter") ⇒ §7's verdict, and per-view
   **per-instance LOD is S3a3** (gs §1.6, §4.2), after S3b.

**One conflict to flag.** gs §1.6's "a chain's levels share vertices" is false against our cooker —
`sf_model::cook` runs `meshopt::optimize_vertex_fetch` per level (`sf_model/src/cook.rs:449-455`),
so levels have disjoint vertex buffers (review S1; `sf_scene/src/chain.rs:5-9` says so too) — but
**harmless**: `GpuBatchGeometryRange` already carries a per-range `first_vertex_index` and
`metadata_index` (gs 2d), so per-level ranges just differ in those fields. Delete the sentence.
### 1.1 Two instance buffers, not one

The spec's §3 "retained GPU instance table per space" and the port's batch output buffer are
different objects, and conflating them is why B3/S10 looked unsolvable. The **scene buffer** is
ours, one per space: the table's 80 B instance records verbatim (`sf_scene/src/table.rs:32`) —
cell-relative transform, sphere, placement, global chain slot, flags — written once per cell at
stream-in through `StaticCellTableView::write_remapped` (`table.rs:425`), allocated by a
`SlotAllocator` per space, one contiguous range per cell. The **batch outputs** are the port's, one
per batch: 64 B `GpuMeshInstance` (render-space transform, `is_active`, `tag`;
`vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:68-76`), allocated by a `ClassAllocator` per batch
(§4.4), retained in S3b and rewritten per frame by the cull from S3a3. The 80 B record is not the
64 B slot, so a verbatim copy was never possible (audit §C): the scene buffer is the **input** to
the cull of §4 (S3a3+), the batch outputs are its result. Three things fall out: `write_remapped` is
exactly right as the upload path, §3's per-space `SlotAllocator` is the right model, and **origin
shifts never touch the scene buffer** (`cell_offset`, `sf_scene/src/lib.rs:52-55`; review
CONFIRMED-GOOD) — §5.4 for the batch outputs.
### 1.2 Two milestones, one ownership model

`is_active` is per instance **slot**, not per view, and S3b has no per-view work items, so:
**M1** (S3b, on today's vendor tree + gs S3a2) has the CPU write the batch slots at stream-in and
leave them, `is_active` carrying residency and promotion only, Bevy's own per-view frustum + Hi-Z
cull doing the per-view work (`render/mesh_preprocess.wesl:190-232`), one level per instance
(§13 Q4) and no horizon, sub-pixel or per-view LOD — those need per-view work items. **M2**
(S3a3/S4) has our cull read the scene buffer and write `draw_index` per slot per view, with the GPU
count/allocate/scatter pass picking levels (gs §4.2). Everything below is written so that only the
**slot writer** differs: the ownership model, allocators, lifecycle and switch are identical.
## 2. The ownership model

`sf_scene_bevy` owns one `SceneSpace` per worldspace — a Bevy-free `sf_scene::space::SceneSpace`
inside a main-world resource. The space owns: a `ChainRegistry` (mesh key → global dense chain
slot, refcounted by resident cell, slots allocated by a `SlotAllocator` with `alloc(1)` so reuse is
deferred by the frames in flight), a `SlotAllocator` over the scene buffer (one contiguous range
per resident cell), one `ClassAllocator` per batch over that batch's output slots, the resident-cell
map holding each cell's per-instance slot vector and its allocated blocks, and the **frame
counter** — which `sf_scene_bevy` increments in `First` and feeds to every `advance`, because
streaming has no frames-in-flight deferral of its own (`sf_world_bevy/src/stream.rs:1311-1355`
despawns immediately; audit §C). `Handle<CookedModelAsset>`/`Handle<Mesh>`/
`Handle<StandardMaterial>` for a chain live in a side map indexed by chain slot in `sf_scene_bevy`,
dropped when the registry reports the last cell released it; Bevy's asset refcount then frees the
mesh and `MeshAllocator` reclaims the slab. Each batch entity's `ranges` vector is rebuilt from the
allocator's block list whenever a block is taken or freed (§4.5). Nothing else in the engine holds
a chain mesh, a scene slot, a batch slot or the frame counter.
## 3. Two things the format and the bake must change
### 3.1 A chain's geometry cannot be found from a v2 table — format v3

`mesh_key = blake3(mesh.cook id, model path, material index)[..8]` (`sf_scene/src/bake.rs:106-117`)
is **a hash, not invertible**: nothing in a v2 table tells the runtime which model a chain came
from. v2 has 6 reserved bytes per chain record (`table.rs:33`, validated zero at `:386-389`) and
three reserved header words (`:31`, `:345-347`), and **nothing has ever been baked**
(`bake_scene` is not called — `bevyrrowind/crates/mw_boot_bevy/src/bake.rs:62-66`), so the format
is free now and never again.

**v3, same record sizes:**
- chain record: `model index u16 @10`, `material index u32 @12` (both were reserved).
- header: `model count u32 @68`, `model bytes u32 @72` (were reserved); `@76` stays reserved.
- new section after `levels`: `model count` × `u32` ascending byte offsets (first = 0) into a
  following `model bytes`-long UTF-8 blob of `mesh.cook` subjects.
- `validate` additions: offsets ascending and in range, blob valid UTF-8, every `model index` <
  `model count`, every model used by some chain, `FORMAT_VERSION = 3`.
- under `feature = "bake"` only, the writer asserts `mesh_key(model, material) == chain.mesh`
  (`mesh_key` needs `sf_bake::Digest`, so the runtime trusts the file — it is a content-keyed cache).
### 3.2 Chain geometry is baked, not built at stream-in

`chain.rs:14-15` gives the concatenate-and-rebase job to the runtime. Besides §3.1, `sf_bake::Store::get`
returns an **owned `Vec<u8>`** (audit §C) and a chain needs every level's submeshes meshopt-decoded
(`sf_model/src/cooked.rs:96-105`) to keep one material's; stream-in is budgeted in bytes at
`Stage::Upload` (`stream.rs:816`) and in milliseconds elsewhere, and this work has no stage and no
budget (review B3).

**Decision: a bake step `scene.chains`, subject `<model>/<material>`, product = an `sf_model`
`.sfmesh` with one LOD level per chain level, one submesh each** (that level's submeshes of the
material, concatenated and rebased by `chain_layout`). The runtime loads it through the existing
`baked://` source and `SfMeshLoader` on the asset pool (`sf_model_bevy/src/source.rs:115`,
`loader.rs:112-147`), and `CookedModelAsset::meshes[level]` is the level's `Handle<Mesh>`
(`loader.rs:149-198`); `materials[0]` is its material. **No new asset type, no new loader, no new
dependency**, no runtime concat, and the table's level records and the uploaded geometry come from
one function. A range is then `{ mesh: meshes[level], first_index: 0, index_count: 0 }` (§1 item 2).

Costs, stated: one extra offline copy of scene-eligible static geometry on disk (**unverified**,
measure at bake) and one more bake step. Per-level meshes make `LodLevel::first_index` describe a
concatenation nothing builds — **spec correction** (§12). `sf_model::CookedSubmesh::encode` is
`pub(crate)` (`cooked.rs:107`): make it `pub` (one line, Bevy-free).
### 3.3 `bake_scene` and `bake_chains`

`bake_scene(baker, db, worldspaces) -> Result<BakeReport, BakeError>` already exists
(`bevyrrowind/crates/mw_bake/src/scene.rs:200`). Call it in `mw_boot_bevy/src/bake.rs` **after
`bake_far` (`:64`) and before `bake_actors` (`:66`)**: it reads `world.page` and level-0
`terrain.tile` (`mw_bake/src/scene.rs:180-190`, `sf_scene/src/bake.rs:560-585`), so it must follow
`bake_exterior` (`:62`) and precede `baker.manifest()` (`:75`) and `store.save_manifest` (`:77`).
`bake_chains` runs immediately after in the same baker: `SceneBake::chain_keys` derives its subjects
from the baked tables' **model sections** (§3.1), so chains dedup across cells and a chain whose
model did not change is a cache hit. Mirror `far`'s two report lines (`bake.rs:115-120`) and print at
most ten failures, as the neighbours do; the generic per-step line (`:102-111`) picks both steps up.
Registration needs nothing new (`SceneBake::register`, `sf_scene/src/bake.rs:488-490`);
`ChainsStep::version() = 1`, and `plan` declares `cx.product(MeshCookStep::ID, model)` (a *static*
input, so invalidation is exact) plus a `"layout"` param of `chain::LAYOUT_VERSION`. A failed chain
product fails only its own subject; §5.3 handles it at runtime.
## 4. `sf_scene` additions — Bevy-free, seconds to test
### 4.1 `product.rs` (always compiled)

```rust
pub const CELLS_STEP: &str = "scene.cells";      pub const CELLS_EXT: &str = "sfscene";
pub const CHAINS_STEP: &str = "scene.chains";    pub const CHAINS_EXT: &str = "sfmesh";
pub fn cell_subject(ws: &str, cell: IVec3) -> String;          // `<ws>/<x>,<y>,<z>`
pub fn parse_cell_subject(s: &str) -> Option<(String, IVec3)>;
pub fn chain_subject(model: &str, material: u32) -> String;     // `<model>/<material>`
pub fn parse_chain_subject(s: &str) -> Option<(&str, u32)>;
```
`WorldspaceId` lives in `sf_world`, which the runtime must not pull (§12), so these take `&str`;
`bake` re-exports wrappers with today's signatures so `mw_bake` is unchanged (`bake.rs:93-104`).
### 4.2 `chains.rs` — the global chain table

```rust
pub type ChainKey = u64;    // the table's 64-bit mesh key (table.rs:16-21)
pub type ChainSlot = u32;   // the global dense slot write_remapped writes into instances
pub struct ChainSource { pub model: String, pub material: u32, pub levels: u32 }
pub struct ChainEntry { pub key: ChainKey, pub source: ChainSource, pub class: MaterialClass,
                        pub material_key: u64, pub levels: Vec<LodLevel>, pub refs: u32 }
/// `Disagreement`: two cells disagree about one mesh key — a stale product in the store.
pub enum ChainError { Disagreement { key: ChainKey, field: &'static str }, NotResident { key: ChainKey } }
pub enum Acquired { Added(ChainSlot), Shared(ChainSlot) }
pub struct ChainRegistry { /* slots: Vec<Option<ChainEntry>>, by_key: BTreeMap, alloc: SlotAllocator */ }
impl ChainRegistry {
    pub fn new(frames_in_flight: u32) -> Self;
    pub fn acquire(&mut self, entry: ChainEntry) -> Result<Acquired, ChainError>;
    /// Drops one ref; `Some(slot)` when it was the last (the caller drops the handles).
    pub fn release(&mut self, key: ChainKey, frame: u64) -> Result<Option<ChainSlot>, ChainError>;
    pub fn advance(&mut self, frame: u64);
    pub fn slot(&self, key: ChainKey) -> Option<ChainSlot>;
    pub fn entry(&self, slot: ChainSlot) -> Option<&ChainEntry>;
    pub fn stats(&self) -> ChainStats;   // live, pending, high_water
}
```
Rules, each a test: slots are **dense and deterministic** (first fit over the coalesced free list
is the lowest free slot, `alloc.rs:79-108`), reuse is **deferred by the frames in flight**
(`alloc.rs:110-134`) so a reused slot can never be read by a frame holding the old geometry, and a
**disagreement is an error, not an assert** — equal mesh key must mean equal class, material key,
level count and every `LodLevel`; a mismatch means a stale `scene.cells` product, fails that cell
(§5.3), and mutates nothing. `ChainRegistry` holds **no Bevy types**: the Bevy half keeps
`Vec<Option<ChainAssets>>` indexed by slot.
### 4.3 Chain residency

A chain is resident when every level mesh is loaded. A cell is not published until all its chains are
resident (§5.2); the slot writer also sets `is_active = 0` for an instance whose chain is not
resident; and the port independently leaves `batch.input_range` empty while a range's mesh allocation
is missing (`gpu_instance_batch/mod.rs:412-425`, re-validated every frame by gs 2c). Three belts, so
a garbage index range (review B3's open question) is unreachable.
### 4.4 `alloc.rs` — `ClassAllocator`: the fragmentation policy

New type; `SlotAllocator` is untouched (review: "do not touch"). `AllocError` gains
`Exhausted { want: u32, capacity: u32 }`.

```rust
/// Block sizes: multiples of 64 slots to 512, then geometric ×1.25 rounded up to 64.
/// 64 because the materialize dispatch is `capacity / 64` workgroups (`materialize.rs:210`).
pub const CLASSES: usize = 32;
pub fn size_class(len: u32) -> Option<(u8, u32)>;  // (class, block slots), None for 0 or > 129_088
pub struct ClassAllocator {
    capacity: u32, bump: u32,            // capacity = the batch's fixed output size
    free: [Vec<u32>; CLASSES],           // released block starts per class, LIFO (deterministic)
    pending: Vec<(u64, u32, u8)>, frames_in_flight: u64,
}
impl ClassAllocator {
    pub fn new(capacity: u32, frames_in_flight: u32) -> Self;
    pub fn alloc(&mut self, len: u32, tag: u64) -> Result<SlotRange, AllocError>;  // never splits
    pub fn release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError>;
    pub fn advance(&mut self, frame: u64);
    /// Live blocks in address order, each with the caller's tag: §4.5 builds `ranges` from this.
    pub fn blocks(&self) -> impl Iterator<Item = (SlotRange, u64)>;
    pub fn stats(&self) -> ClassStats;   // bump, live, pending, free per class, internal waste
}
```
**Verdict: corrected 2026-11-10 by measurement (A4) — the bound below is false and the policy was
reversed.** Blocks are never split and never merged, so a freed class-`c` block satisfies *any*
later class-`c` request: **external fragmentation inside a class is zero**, and the only growth is
`bump = Σ_c size(c)·peak(c)`, with `peak(c)` the most class-`c` blocks ever *occupied* at once —
live, or still waiting for the frames in flight. That identity is exact
(`bump_is_bounded_under_a_city_walk` asserts it at twenty stages of a 200 000-cycle walk), and with
it comes the one bound that holds a priori: `peak(c) ≤ resident cells + frames in flight`, so
`bump ≤ (resident + frames in flight) · Σ_{c reachable} size(c)` — computable from the ring
configuration before a cell streams in, and **independent of session length**. That is what
"bounded" means here, and all it means.

**What this section claimed and the measurement denies:** `bump` is *not*
`≤ 1.25·Σ_c peak_c·len_c + 64·CLASSES` ("≤ 1.25 × the worst resident footprint plus one block of
slack per class"), and it does *not* converge. The classes do not peak together, and `peak(c)` is a
record process whose records keep being beaten, ever more rarely. On a synthetic 5×5 ring over 40
cell footprints (7..=638 instances, nine classes) `bump / peak_live` measures **3.0× after 200 000
stream-in cycles and 3.33× after 2 M** (≈ 9 h of play at 60 fps), still creeping, against an
a-priori ceiling of 7.0×. The same sequence through first-fit `SlotAllocator` settles at **1.2×**
(`first_fit_high_water_on_the_same_walk`): Robson's ≈ 11× is an adversarial bound, and a streaming
ring with a content-bounded size set is not an adversary. Measured again in A5 through the whole
space (`batch_slot_high_water_curve`): **1.14×**, identical at 1 000, 10 000, 50 000 and 200 000
cycles, with two or three coalesced free ranges.

So the case for segregated classes over a batch's output slots is the a-priori ceiling (no appeal to
the sequence), `BLOCK_GRAIN`-aligned block starts for the materialize dispatch, and O(1) allocation
with no free-list scan — **not** a smaller high-water mark. Budget a batch's `capacity` from the
ceiling, never from `1.25 × the live set`. The internal waste this section predicts does hold:
`size(c) < 1.25·len + 64`, measured under 1 000 slots across 25 live cells.

**Coordinator decision 2026-11-10: first fit.** The scene-buffer records, the per-batch output slots
and the chain table all allocate with `SlotAllocator` (`SlotAllocator::with_capacity` adds the fixed
ceiling a batch needs, answering `Exhausted` instead of growing). Two reasons beyond the 2.6×:
per-view output memory is sized by **capacity**, so allocator waste multiplies by the view count in
a quantity already estimated at ~140 MB; and at a 4096-slot capacity the size-class allocator would
hold only ~1 300 live slots, making "exhausted, fall back to entities" the main path instead of a
corner. `ClassAllocator` stays in the tree, tested, carrying the corrected verdict in its doc
comment: its a-priori ceiling and 64-slot-aligned block starts may earn it a place later. This also
supersedes static-scene.md §3's "a segregated size-class allocator per batch" bullet, which points
here for its proof. What does **not** change: batch capacity is fixed at config and never grown at
stream-in, and `Exhausted` → entity fallback stays a counted, warned, tested path — first fit has no
a-priori ceiling, and its 1.14-1.17× was measured on one synthetic walk, so a heavy mod list could
differ and we need to be able to see it.

**Capacity policy** (the review's missing item 4; our duty per gs SERIOUS 6): `capacity` is fixed
per batch at plugin configuration (`SceneConfig::batch_capacity`, default 4096), **never grown at
stream-in** — growing it reallocates the output buffer and rebuilds the materialize bind group
(`mod.rs:479-494`, `materialize.rs:162-166`, review S7), breaking "stream-in ≤ 0.2 ms, zero
spawns". On `Exhausted` the cell publishes **without** that batch's instances, those placements are
left out of `covered` so they spawn as entities (§5.3), `scene/overflow/cells` counts it and a
`warn_once` names the batch key.
### 4.5 `space.rs` — the lifecycle, and the range list

```rust
pub struct BatchKey {            // gs C1 / O3: per (class, slab pair, bind group)
    pub class: u16, pub bind_group: u32, pub vertex_slab: u32, pub index_slab: u32,
    pub chain: ChainSlot, pub level: u8,   // set only under BatchGranularity::PerChain
}
pub enum BatchGranularity { PerChain, PerClass }   // PerChain = one range per batch, the fallback
pub type BatchId = u32;
pub struct CellResidency {
    pub table_len: u32,
    pub scene: SlotRange,              // the cell's contiguous scene-buffer range
    pub chain_slots: Vec<ChainSlot>,   // cell-local chain index -> global slot (write_remapped arg)
    pub batch_slots: Vec<u32>,         // one per table instance, table order; u32::MAX where the
                                       // batch overflowed. The promotion index.
    pub blocks: Vec<(BatchId, SlotRange, ChainSlot, u8)>,   // for release and the range list
    pub promotions: Promotions,
}
pub struct CellPlan {
    pub scene: SlotRange,
    pub load: Vec<(ChainSlot, ChainSource)>,     // first reference: start the asset load
    pub batches: Vec<(BatchId, BatchKey)>,       // batches that must exist
    pub scene_bytes: Vec<u8>,                    // write_remapped output, cell-relative, 80 B each
    pub writes: Vec<SlotWrite>,                  // (batch, slot, table index, active)
    pub ranges: Vec<BatchId>,                    // batches whose `ranges` vector must be rebuilt
    pub covered: Vec<PlacementId>,               // placements the entity path must not spawn
    pub overflowed: Vec<BatchKey>,
}
pub enum SceneError { Table(TableError), Chain(ChainError), Scene(AllocError), Resident(IVec3),
                      NotResident(IVec3) }
pub struct SceneSpace { /* chains, scene: SlotAllocator, batches: Vec<(BatchKey, ClassAllocator)>,
                          cells: BTreeMap<IVec3, CellResidency>, frame, stats */ }
impl SceneSpace {
    pub fn new(config: SpaceConfig) -> Self;   // granularity, frames_in_flight, capacities
    /// Pure: validates, allocates, returns the plan. On error nothing is mutated.
    pub fn plan_in(&mut self, cell: IVec3, table: &StaticCellTableView<'_>) -> Result<CellPlan, SceneError>;
    pub fn commit_in(&mut self, cell: IVec3, plan: &CellPlan);
    pub fn plan_out(&mut self, cell: IVec3) -> CellRelease;   // chains whose last ref went, batches to rebuild
    pub fn promote(&mut self, cell: IVec3, p: PlacementId, table: &StaticCellTableView<'_>) -> Vec<SlotWrite>;
    /// Blocks in address order, each tagged (chain, level); a free block merges into the previous
    /// live range (the first into the following one), so the list partitions `0..capacity` with no
    /// gaps as gs 2a/2b require, with no filler ranges — a merged range's extra slots are
    /// `is_active = 0` and draw nothing.
    pub fn batch_ranges(&self, batch: BatchId) -> Vec<RangeSpec>;  // (first_slot, slot_count, chain, level)
    pub fn advance(&mut self, frame: u64);
    pub fn stats(&self) -> SpaceStats;
}
```
`plan_in` is the heart and entirely Bevy-free: acquire a chain slot per chain of the table
(`ChainEntry::from_table`, `chains.rs`; the `chains()` iterator is `table.rs:763`), build
`chain_slots` in cell-local order, allocate the scene range, `write_remapped(&chain_slots, &mut
scene_bytes)` (`table.rs:645`, which **panics** unless `chain_slots` holds one slot per chain,
overflowed ones included), bucket instances by batch key, allocate one block per
(batch, chain, level) bucket, fill `batch_slots`, collect `covered`, report overflows. One pass over
the table plus one per bucket; three vectors, nothing per instance.
**Atomic:** on any error every acquired chain ref and allocated block is rolled back (tested); as
built, the chains a cell shares with a resident one are acquired *first*, so the one reachable error
(a stale-product disagreement) is found before anything is allocated and unwinds to refcounts alone.
Why `batch_slots` and not arithmetic: the table is sorted by `(placement, part)` (`table.rs:7-12`),
not by chain, so a cell's instances for one chain are not contiguous and
`slot = cell start + WorkItem::instance` (`cull.rs:304-311`; there is no `WorkItem::slot`) holds
under no batch granularity. 4 B per instance (400 KB at 100k) buys exact promotion and release;
chain-major instance order is a v3+ driver (review M7) that would reduce it to a base per bucket.
## 5. The streaming lifecycle
### 5.1 The seam: a sibling of `CellProxies`

Not `CellLoaded`/`CellUnloaded` (`stream.rs:198-212`) — they fire after the fact and carry no
budget — but a sibling hook of `CellProxies` (`sf_world_bevy/src/far.rs:33-57`), so scene loads
share the in-flight caps, the byte budget and the reveal-at-once rule:

```rust
// sf_world_bevy/src/scene.rs (new, ~90 lines)
pub struct SceneCellCx<'a> { pub worldspace: &'a WorldspaceId, pub cell: IVec3,
                             pub cell_origin: WorldPos, pub level: Level }
/// Order for one cell: `acquire` -> `ready` until true -> `covered` -> `upload` -> `release`.
pub trait CellScene: Send + Sync + 'static {
    fn acquire(&self, world: &World, ws: &WorldspaceId, cell: IVec3);
    fn ready(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> bool;
    /// Placements the table draws: their models are not loaded and they do not spawn.
    fn covered(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> Option<Arc<HashSet<PlacementId>>>;
    /// Queues this cell's uploads; returns (bytes queued, done) — resumable.
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> (u64, bool);
    fn release(&self, world: &mut World, ws: &WorldspaceId, cell: IVec3);
    fn bytes(&self, _ws: &WorldspaceId, _cell: IVec3) -> u64 { 0 }
}
#[derive(Resource, Clone, Default)] pub struct SceneCells(pub Option<Arc<dyn CellScene>>);
```
Call sites in `stream.rs`, one or two lines each:

| Where | What |
|---|---|
| `start_fetch` `:695-726` | installed, `to == Level::Near`, `kind == Load` ⇒ `acquire`, `work.scene = true` |
| `poll_fetches` `:779-792` | union `covered(..)` into `work.members`, so the model gather at `:760-770` skips them; `stats.scene_covered` |
| readiness `:496-510` | `&& (!w.scene \|\| scene.ready(..))`, exactly as `w.proxy` does |
| `reveal` (`Stage::Upload`) `:1262-1275` | `scene.upload(world, &cx)` before revealing the group; the stage's cost is already `request.upload_bytes` (`:816`) |
| `despawn_item` `:1346-1352` | `if request.from == Level::Near { scene.release(..) }`, beside the far proxy release |
| `upload_estimate` `:787-798` | add `scene.bytes(..)` to the `Level::Near` arms |

`work.members` already means "drawn by something else, do not spawn, still activate"
(`stream.rs:1232-1236` skips the spawn; `activate_item` `:1276-1307` runs for every placement), so
colliders, scripts and AI are untouched and **no new concept enters `sf_world_bevy`**.

**Why `Near`:** with a far field the near level holds every placement of the cell and R1 is sized
just past the object handoff (`sf_lod_bevy/src/lib.rs:138-141`), so table residency and entity
residency coincide. Without one, R2 spawns far-eligible placements as coarse entities
(`stream.rs:1398-1410`); those are not in the near step's `members`, so they stay entities and
their table instances must stay inactive — which is exactly promotion: `plan_in` writes
`is_active = 1` only for instances whose placement is in `covered`. One mechanism, both
configurations, no double draw.
### 5.2 Readiness and upload budgeting

`ready` is true when the `.sfscene` asset reached `Loaded` or `Failed` **and** (if loaded) every
chain's `CookedModelAsset` reached `Loaded` or `Failed`
(`AssetServer::get_recursive_dependency_load_state`, as `sf_lod_bevy/src/runtime.rs:226-233`); a
failed chain fails the cell (§5.3), so reveal-at-once holds and a cell never publishes half its
statics. `upload` queues the cell's `scene_bytes` (80 B × instances ≈ 8-40 KB) and its `writes`
(64 B × instances) into `SceneUploads`, drained by extraction; in the render world each becomes one
`write_buffer` per contiguous run, so a cell costs 1 + (buckets touched) writes. The residency keeps
a cursor and `upload` returns `done == false` when the byte budget cut it short, so the scheduler
re-invokes it next frame (`run_stage` `:802-818`). `RenderAssetBytesPerFrame` is already disabled
(`sf_world_bevy/src/lib.rs:136-141`) and does not interfere.
### 5.3 Missing or failed products — every path ends at the entity path

| Failure | Behaviour |
|---|---|
| no `scene.cells` product (not baked, a mod added a cell) | `BakedReader` returns `NotFound` (`sf_model_bevy/src/source.rs:65-79`) ⇒ `ready` true, `covered` `None` ⇒ no suppression, models load, spawners spawn. `scene/cells/missing`, `debug!` once per cell. |
| table fails to parse (`TableError`) | as above, plus `error!` once per cell with the error: a corrupt cache is a bug worth seeing. |
| a chain's `scene.chains` product missing or failed, or `ChainError::Disagreement` | the **cell** falls back entirely; acquired chain refs released. `scene/chains/failed`, `error!` naming the key and (for a disagreement) the field. |
| `AllocError::Exhausted` | the cell publishes without that batch's instances; they are absent from `covered`, so they spawn as entities. `scene/overflow/cells`. |

`covered` is consulted after `ready`, so the decision is always made on a resolved table — the same
shape as the precombine's `valid_members` fallback (`stream.rs:783-789`).
### 5.4 Origin shifts

The scene buffer is cell-relative, so a `big_space` shift changes nothing there (`cell_offset`,
`lib.rs:50-55`), and under **M2** the batch slots are rewritten every frame by the cull: a shift is
a non-event end to end. Under **M1** the retained batch slots hold render-space transforms and must
be rewritten — the plugin reacts to the shift the streaming guard already detects
(`stream.rs:596-616`, `SHIFT_COOLDOWN_FRAMES` `:57`) and queues a rewrite of every resident cell's
writes, spread by `SceneConfig::shift_rewrite_bytes` (default 1 MiB/frame) while the guard holds the
other stages: 6.4 MB at 100k instances, ~7 frames every 256 m of travel (`GRID_CELL`), reported as
`scene/shift/rewrite_bytes`. M1's one price for having no GPU cull, and the reason M1 is a milestone
and not the shipping configuration.
## 6. `sf_scene_bevy`

**Crate.** `superfargoth/crates/sf_scene_bevy`, template `sf_lod_bevy` (same role: a Bevy plugin
over per-cell baked products implementing a streaming hook, `sf_lod_bevy/src/lib.rs:1-16`).
Dependencies, all already in the app graph — **no new external dependency**: `bevy_app`,
`bevy_asset`, `bevy_camera`, `bevy_ecs`, `bevy_math`, `bevy_mesh`, `bevy_pbr`, `bevy_platform`,
`bevy_render`, `bevy_transform`, `glam`, `log`, `sf_coords`, `sf_diag_bevy`, `sf_model_bevy`,
`sf_scene`, `sf_world`, `sf_world_bevy`, `workspace-hack`. Unavoidable cost, flagged: a new
workspace member changes the root `Cargo.toml` and `Cargo.lock` and needs
`cargo hakari generate && cargo hakari manage-deps` (`superfargoth/CLAUDE.md` "Commands") — **one
cold rebuild**, so create the crate in the same build slot as its first real code (§10 B1+B2).

```rust
#[derive(Resource)] pub struct SceneSpaces(pub BTreeMap<WorldspaceId, SpaceState>);
pub struct SpaceState { pub space: SceneSpace, pub chains: Vec<Option<ChainAssets>>,
                        pub batches: Vec<Entity>, pub loading: HashMap<IVec3, CellLoad> }
pub struct ChainAssets { pub product: Handle<CookedModelAsset>, pub levels: Vec<Handle<Mesh>>,
                         pub material: Handle<StandardMaterial> }
#[derive(Resource, Default)] pub struct SceneFrame(pub u64);
#[derive(Resource, Default)] pub struct SceneUploads { pub cells: Vec<CellUpload>, pub writes: Vec<SlotWrite> }
```
`SceneTableAsset(pub Vec<u8>)` is a one-field asset with a loader registered for `CELLS_EXT`
(template `sf_lod_bevy/src/assets.rs:41-66`); the view borrows its bytes, so the asset must own the
`Vec` (audit §C). `sf_scene_bevy::baked_extensions()` copies `sf_lod_bevy::baked_extensions()`
(`sf_lod_bevy/src/lib.rs:129-131`) and joins the map at
`bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs:205-207`; `CHAINS_STEP → "sfmesh"` makes
`SfMeshLoader` pick chain products up.

**Batch entities.** Per `BatchId`, one entity with
`GpuBatchedMesh3d { mesh, max_capacity: config.batch_capacity, ranges }` (gs 2a) +
`MeshMaterial3d<StandardMaterial>` (required in practice, `mod.rs:415-420`) + `Transform::IDENTITY`;
**never `Mesh3d`**, never `NoCpuCulling` (`mod.rs:311-316`); plus `NotShadowCaster` until gs step 1
lands (audit §A4: lamp shadow views do not unpack their range work items and would read
uninitialized slots), removed by whichever slice takes that step. `mesh` is the first range's mesh,
`ranges` comes from `SceneSpace::batch_ranges`. Materials are built once per `material_key` from the
chain product's `materials[0]` through `ModelMaterials` + `sf_world_bevy::matclass::fold`
(`stream.rs:1437-1457`, `matclass.rs:142-194`), so scene materials fold into classes exactly as
entity-path materials do and `--off matclass` keeps working.

**Systems.** `pub enum SceneSystems { Frame, Residency, Promote }`.

| System | Schedule / set | Ordering |
|---|---|---|
| `tick_frame` (`SceneFrame += 1`, then `advance` every space) | `First`, `Frame` | before the rest of the plugin |
| `poll_cell_loads` (plan a cell whose table and chains resolved; spawn/despawn batch entities; rebuild `ranges`) | `Update`, `Residency` | `.after(StreamingSystems::Drive)` — the hook ran this frame |
| `apply_promotions` (drain the queue into `SceneUploads::writes`) | `Update`, `Promote` | `.after(Residency)` |
| `extract_scene_uploads` (take `SceneUploads`) | `ExtractSchedule` | — |
| `prepare_scene_buffers` (create/grow the space's scene buffer: `STORAGE \| COPY_SRC \| COPY_DST`, doubling, encoder copy as `mod.rs:479-494`) | `Render`, `RenderSystems::PrepareResources` | — |
| `publish_scene` (scene-range writes, batch slot writes, `is_active` flips) | `RenderGraph`, `GpuInstanceBatchSystems::Publish` | — |

(`report_counters` in `Last`, §9.) `Publish` is the only correct home for the writes: it is where
`RenderContext` commands are recorded and they are submitted before `Materialize`
(`mod.rs:226-235`, `:80-85`), and it runs after `RenderSystems::PrepareMeshes`, where the port
creates reservations and resolves ranges (`mod.rs:346-369`, `:371-444`). The hook's five methods run
inside `drive_streaming` (exclusive, `stream.rs:466-477`), as `CellProxies` does.
`frames_in_flight = 3` (`SceneConfig`): the render world is one frame behind and the GPU may be
another behind submission (**unverified** — wgpu states no bound); being wrong costs one frame of a
stale transform in a reused slot, never UB, as the buffer is not resized.
## 7. Promotion and demotion

Gameplay calls `SceneSpaces::promote(ws, cell, placement)` (or writes `PromotePlacement` when it
holds only `Commands`). `SceneSpace::promote` records it in `Promotions` (`eligibility.rs:124-160`:
one-way, dropped with the cell) and returns one `SlotWrite { active: false }` per table instance of
that placement (`find_all`, `table.rs:464`, then `batch_slots`); `publish_scene` turns each into a
4-byte `write_buffer` at `slot * 64 + 48` (`GpuMeshInstance::is_active`, `mod.rs:68-76`;
`COPY_BUFFER_ALIGNMENT` is 4, so one word is a legal write). The caller then spawns the entity with
the same placement id through the existing spawner (`StaticSpawner`, `spawn.rs:280-283`): the scene
side never spawns, it only stops drawing. Demotion is the cell unload (`Promotions` dies with the
residency); there is nothing to undo.

**The `is_active` hole — verdict: promotion does not need the GPU fix first.** The gate is nested in
`@if(FRUSTUM_CULLING)` (`render/mesh_preprocess.wesl:190-194`), and `FRUSTUM_CULLING` is set for
every indirect preprocessing pipeline and only those: `direct_preprocess` is specialised with
`PreprocessPipelineKey::empty()` (`render/gpu_preprocess.rs:2095-2099`) while every other prepared
key contains it (`:2100-2123`), so `INDIRECT ⇒ FRUSTUM_CULLING ⇒ is_active honoured`. And a view
that does not draw indirectly drops instance batches entirely (`if no_indirect_drawing { continue }`,
`bevy_render/src/batching/gpu_preprocessing.rs:2573-2576`), so it draws **no** statics rather than
drawing them twice: a promoted placement can never double, it only goes missing there — the hardware
floor gs O5 names (and gs SERIOUS 7 corrects the audit: our inherited shadow views never
individually lose indirect drawing, they inherit the camera's setting,
`sf_sky_bevy/src/sun_view.rs:227-231`, `sf_shadow_cache_bevy/src/render.rs:518`). So the plugin logs
one startup `warn!` and behaves as `--off scene` when `GpuPreprocessingSupport` is
`PreprocessingOnly` (gs O5's default), and `promotion_is_safe_without_the_shader_fix` asserts both
pipeline-key facts from the vendored source so the day someone prepares a `FRUSTUM_CULLING`-less
indirect pipeline it fails. gs step 4 un-nests the guard anyway (one line), after which that test's
second half is redundant and its comment says so.
## 8. The `--scene` switch and the A/B

Framework side: `pub fn suppress_scene_placements(spawners: &mut PlacementSpawners)` in
`sf_scene_bevy`, wrapping every spawner so a placement a resident table draws spawns nothing (shape
copied from `mw_devtools_bevy`'s `OffSpawner`/`apply_off`, `perf.rs:236-276`). It is a belt to the
braces: the primary mechanism is `CellScene::covered` feeding `work.members` (§5.1), which also
stops the **model loads**; `needs_model` cannot be cell-aware (`IndexedPlacement` has no cell,
`sf_world/src/index.rs:76-80`), which is exactly why `members` is primary. Game side
(`bevyrrowind`):
- `#[arg(long)] pub scene: bool`, beside `lod_tint` (`apps/bevyrrowind/src/main.rs:395-397`),
  threaded into `ExteriorOptions::scene` (`mw_exterior_bevy/src/exterior.rs:78`), **off by default**.
- `("scene", Part::Scene)` in `Part::ALL` — **the count becomes 55** (`mw_game_core/src/perf.rs:117`)
  — so the A/B reads the same both ways once it flips on:
  `let scene = options.scene && !perf.off.has(Part::Scene);`
- Resolved into plugin config **before `add_plugins`**, as `--off matclass` is (`exterior.rs:269`,
  `:283`): `ScenePlugin { config }` with `config.enabled = scene`, plus
  `streaming.precombine &= !scene` — the precombine draws the same placements
  (`stream.rs:1176-1207`), so the two must never both be on.
- `if scene { sf_scene_bevy::suppress_scene_placements(&mut spawners); }` where `apply_off` runs.

A/B recipe: `balmora_walk` with and without `--scene`, reading `views/<kind>/{items, bins,
batch_sets}` (`sf_diag/src/views.rs:13-17`) and `stream/*`. **Blocked on two one-liners outside this
plan:** `batch_set_count()` is blind to instance batches (audit §A5) and `items` under-reports by
orders of magnitude (§B1) — the A/B is unreadable without them.
## 9. Counters (`sf_diag`, namespace `scene/`)

`chains/{live,pending,high_water,loading,failed}`, `cells/{resident,missing,failed,covered}`,
`slots/{scene_bump,scene_live,batch_bump,batch_live,waste,overflow}`,
`batches/{count,ranges,instances}`, `upload/bytes`, `shift/rewrite_bytes`, `plan/ms` — set in `Last`
behind `resource_exists::<DiagCounters>`, as `matclass::track_static_materials` is
(`sf_world_bevy/src/lib.rs:131-133`).
## 10. Steps
"Bevy?" = needs a Bevy build slot. `sf_scene` steps compile and test in seconds; group them.

| # | Step | Files | Bevy? | Group |
|---|---|---|---|---|
| A1 | `product.rs`: the four step/subject items out of `feature = "bake"`, plus the chain ones; re-export from `bake` | `sf_scene/src/{lib,product,bake}.rs` | no | α |
| A2 | Format v3 (§3.1): chain model/material index, model section, header words, `validate`, writer, `StaticCellTable` | `sf_scene/src/table.rs` | no | α after A1 |
| A3 | `ChainRegistry` | `sf_scene/src/{chains,lib}.rs` | no | β after A2 |
| A4 | `ClassAllocator`, `size_class`, `AllocError::Exhausted` | `sf_scene/src/alloc.rs` | no | β ∥ A3 |
| A5 | `SceneSpace`, `CellPlan`, `BatchKey`, `batch_ranges`, `SceneError` | `sf_scene/src/{space,lib}.rs` | no | γ after A3+A4 |
| A6 | `bake`: write v3, `ChainsStep`, `SceneBake::{chain_keys,bake_chains}`; `sf_model::CookedSubmesh::encode` → `pub` | `sf_scene/src/bake.rs`, `sf_model/src/cooked.rs` | no | γ after A2 |
| B1 | Create `sf_scene_bevy`: crate, manifest, workspace member, hakari, `SceneTableAsset` + loader, `baked_extensions`, empty `ScenePlugin` | new crate, root `Cargo.toml`, `Cargo.lock`, `workspace-hack` | **yes** | δ (one cold rebuild — bundle with B2) |
| B2 | `CellScene`/`SceneCells` + the six call sites (§5.1) | `sf_world_bevy/src/{scene,stream,lib}.rs` | **yes** | δ |
| B3 | `SceneSpaces`, chain asset loading, batch entities + `ranges`, materials, `poll_cell_loads`, the hook impl | `sf_scene_bevy/src/{lib,residency,chains}.rs` | **yes** | ε after B1+B2+A5 |
| B4 | Render world: `extract_scene_uploads`, `prepare_scene_buffers`, `publish_scene` (M1 writer) | `sf_scene_bevy/src/render.rs` | **yes** | ε |
| B5 | Promotion, the `PreprocessingOnly` fallback + warning, `suppress_scene_placements` | `sf_scene_bevy/src/{promote,spawners}.rs` | **yes** | ζ after B3 |
| B6 | Counters | `sf_scene_bevy/src/diag.rs` | **yes** | ζ |
| C1 | `bake_scene` + `bake_chains` calls and report lines (§3.3) | `mw_boot_bevy/src/bake.rs`, `mw_bake/src/scene.rs` | no (bake binary) | η after A6 |
| C2 | `--scene`, `Part::Scene` (ALL → 55), plugin wiring, `precombine` interlock | `apps/bevyrrowind/src/main.rs`, `mw_game_core/src/perf.rs`, `mw_exterior_bevy/src/exterior.rs` | **yes** | η after B5 |

Every step leaves the tree compiling: A1-A6 are additive in a crate nothing yet calls (A2's v3
breaks only `bake.rs`, changed in A6 in the same test slot), B1-B6 build a plugin nobody adds, C1-C2
turn it on. Run α+β+γ in one `cargo test -p sf_scene` slot, δ+ε in one Bevy slot, ζ+η in a second;
B3's `NotShadowCaster` comes off in whichever slice takes gs step 1.
## 11. Tests

`sf_scene`, Bevy-free and in seconds, synthetic tables through `StaticCellTable::write` as
`table.rs`'s own tests do:
- `table`: `v3_round_trips_the_model_section`, `rejects_a_model_index_out_of_range`,
  `rejects_unsorted_or_out_of_range_model_offsets`, `rejects_a_model_no_chain_uses`,
  `rejects_version_2`, `mesh_key_matches_the_chain_source` (`cfg(feature = "bake")`).
- `chains`: `a_second_cell_shares_the_slot_and_bumps_the_refcount`,
  `the_last_release_frees_after_the_frames_in_flight`, `slots_are_dense_and_lowest_first`,
  `disagreeing_level_lists_are_an_error_and_mutate_nothing`,
  `a_released_slot_is_not_reused_inside_the_window`.
- `alloc`: `size_class_waste_is_under_a_quarter`, `a_freed_block_satisfies_its_class`,
  `blocks_are_never_split`, `exhaustion_is_an_error_not_a_panic`, and
  `bump_is_bounded_under_a_city_walk` — 10 000 deterministic stream-in/out cycles over 40 cell
  sizes, asserting `bump ≤ 1.25·peak_live + 64·CLASSES` and that `bump` stops growing after the
  first full traversal.
- `space`: `plan_in_remaps_every_chain_index`, `plan_in_is_atomic_on_error`,
  `batch_slots_index_promotion`, `promote_deactivates_every_part_of_the_placement`,
  `plan_out_releases_ranges_and_chains`, `overflow_leaves_the_rest_of_the_cell`,
  `two_cells_sharing_a_chain_keep_it_through_one_unload`,
  `batch_ranges_partition_the_capacity_with_no_gaps` (the gs 2b invariant, ideally through gs's own
  `validate_ranges`), `batch_ranges_merge_free_blocks_into_a_neighbour`,
  `per_chain_and_per_class_granularity_agree_on_slot_counts`.
- `bake`: `chain_product_levels_match_the_table`, `chain_subject_round_trips`,
  `chain_keys_deduplicates_across_cells`.

Headless apps, no render app (template `gpu_batch_tests.rs:178-190`) — `sf_world_bevy`:
`scene_covered_placements_load_no_models_and_spawn_nothing`,
`a_cell_without_a_scene_hook_streams_as_before`,
`release_runs_on_the_near_unload_and_after_a_cancelled_load`. `sf_scene_bevy`:
`a_resident_cell_spawns_its_batches_and_despawns_them_on_unload`,
`chain_handles_outlive_the_first_cell_and_drop_with_the_last`,
`promotion_queues_one_write_per_part`, `promotion_is_safe_without_the_shader_fix` (§7),
`frames_in_flight_defers_every_reuse` (drives `SceneFrame` by hand),
`a_missing_table_falls_back_to_entities`. Owner machine only, `#[ignore = "measurement"]`: the §8
A/B; `scene/slots/*` after a 20-minute Balmora walk (the §4.4 bound in the field); frames until all
static pipelines are resident (review "cold start"); determinism — the same pose reached by flying
vs. starting there gives the same resident cells, chain slots, batch list and `ranges` (item 6).
## 12. Recommendations and flags

**`feature = "bake"`: move the four items, do not enable the feature.** `CELLS_STEP`, `EXTENSION`,
`cell_subject` and `parse_cell_subject` sit behind `bake` (`sf_scene/src/lib.rs:26-27`), which pulls
`sf_bake`, `sf_model`, `sf_terrain`, `sf_world` **and** `sf_terrain/bake` + `sf_world/bake`
(`sf_scene/Cargo.toml:11`); `sf_world_bevy` already depends on `sf_scene` with default features, so
enabling `bake` downstream would unify those bake features into the app build — bake code in the
shipped binary, a longer cold build, the runtime depending on the baker. Step A1 moves them.

**Interiors: exteriors only for now** (`SceneConfig::worldspaces` names which spaces get a table;
default, those with a `scene.cells` product). A few hundred statics × 6-8 views is a small slice of
the 46-55k this design exists to remove; a door crossing cycles a whole worldspace in one frame, the
worst case for chain churn and for §4.4's bound; promotion is the common case indoors (containers,
scripted statics), so much of an interior stays entities anyway; and the bake would grow a product
per cell across hundreds of worldspaces. The bake already *can* (a landless worldspace is legal,
`bake.rs:472-484`), so this is a runtime switch. **Owner decision; default off.**

**Spec corrections requested** (the spec is owned elsewhere): (1) `chain.rs:14-15` and §2/§3 — the
*bake* emits a chain's geometry, the runtime holds handles (§3.2); (2) §2/§3 — format **v3**, a mesh
key is not invertible (§3.1); (3) §3 — name the **two** buffers (§1.1); (4) §1/§4 — `is_active` is
per instance, not per view, so our cull and per-view LOD need gs S3a3 (§1.2); (5) §Budgets —
"stream-in ≤ 0.2 ms, zero entity spawns" must gain the chain asset load and the fixed-capacity
policy (§4.4); (6) gs §1.6's "levels share vertices" should go (§1).

**Owner decisions this plan would reverse: none.** Two places where "no quick fixes" made the design
*larger*, said out loud: baking chain geometry instead of concatenating at stream-in (§3.2), and a
format v3 instead of guessing a chain's model at runtime (§3.1). One where the Steam Deck floor
tightens it: §4.4's fixed batch capacity, because per-view output is sized by capacity until gs §4.2
(review B2) and the Deck has one unified budget.
## 13. Open questions, with a recommended default

| # | Question | Default |
|---|---|---|
| 1 | Batch granularity | **`PerClass` per (class, slab pair, bind group)**, matching gs O3; `PerChain` stays as the one-range fallback and as a test oracle. |
| 2 | Batch capacity | **4096 slots**, fixed per batch, overflow per §4.4. Re-tune from `scene/slots/*` after the first walk. |
| 3 | Chain product shape, and interiors | **One mesh per level** (§3.2; a concatenated variant needs only `ChainsStep` and the range builder to change); interiors **off** (§12). |
| 4 | One level per instance under M1 — which level? | **LOD0**, and measure the GPU cost against `--off scene`. The alternative (one slot per level, `is_active` picking) costs 4× capacity, which is the argument for gs §4.2, not against LOD0. |
| 5 | `frames_in_flight` | **3**, in `SceneConfig`; wrong by at most one frame of a stale transform. |
| 6 | `--scene` or `--off scene` | **Both**: `--scene` opts in now, `Part::Scene` (ALL → 55) keeps the A/B after it becomes the default. |
| 7 | A baked per-worldspace chain directory instead of runtime slot allocation? | **No** — it would make a cell unloadable on its own (review CONFIRMED-GOOD) and a mod adding one model would rebake the directory. Runtime allocation, as the spec says. |
| 8 | Who fixes `batch_set_count()` and `items` (audit §A5, §B1)? | **Not this plan — coordinator.** The §8 A/B is unreadable without them. |
## 14. Size

≈ 4 000 lines — `sf_scene` ≈ 1 400 + 700 of tests (table v3 +260, chains +230, alloc +210, space
+460, product +60, bake +180), `sf_world_bevy` ≈ 110, `sf_scene_bevy` ≈ 1 250 + 250 of tests,
`mw_*` ≈ 70 — in **4 agent slices**: A1-A6 (Bevy-free, one test slot), B1-B2 (crate + seam, one cold
build), B3-B4 (the plugin), B5-B6+C1-C2. Deferrable past S3b: the M1 origin-shift rewrite (§5.4,
unnecessary if gs §4.2 lands first), interiors (§12), and the `--scene-cpu` debug path the review
asks for (its item 7), its own small slice once `cull` is wired.
