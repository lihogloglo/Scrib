# Independent architecture review: static scene (2026-11-09)

Independent, read-only review of `superfargoth/docs/specs/static-scene.md` (design, owner go 2026-11-08), the implemented `sf_scene` core,
and the vendored #25978 port. Nothing compiled, nothing run. Repo claims cite `file:line`; external claims a URL, or **unverified**.

## Verdict

The **direction is right**: a retained per-space GPU instance table, culled per view, drawn through Bevy's own pipeline is the canonical
technique and the only candidate that attacks H1's actual cost (per-view CPU × views). `sf_scene`'s format v2, allocator, eligibility
rules and horizon test are good work; do not touch them. But **the design is wrong about the thing S3b is built on**: #25978's batch is
*one mesh per batch entity, one `draw_indexed_indirect` per batch per view*, with the indirect command's index range read from the
**first** work item of the batch. So §6's "LOD index ranges per instance" is silently ignored by `build_indirect_params.wesl` (wrong
geometry, not a compile error), "≤ 2 × material classes batch sets" is off by orders of magnitude, and the per-view output buffer is sized
by **reserved capacity**, not visible count (≈ 176 B × Σ capacity × views ≈ 140 MB). The missing piece is SPUD's three-pass counting sort
moved to compute plus GPU-written indirect metadata — a far larger Bevy patch than §6 admits. Two facts the repo does not yet know:
`is_active` (all of §1's promotion) is ignored in views without GPU frustum culling, including our own extra sun views; and
bevy#26149/#26161 were closed unmerged on 2026-10-30, so the shadow patch and slice S5 rest on dead upstream code. **Verdict: right
architecture, with B1-B4 fixed in the spec before S3b.** Run the S3a.5 spike named below first — one slice, and it decides the shape of
S3b through S9.

## Findings: BLOCKER

**B1. #25978 cannot express per-instance LOD index ranges; the design's core claim rests on it.** The indirect command is built per
*batch* from one mesh input: `build_indirect_params.wesl:133-143` takes `index_count`, `first_index` and `base_vertex` from
`current_input[mesh_index]`, and `mesh_preprocess.wesl:179-181` writes `mesh_index` from the **first** work item of that indirect slot
only. One mesh per batch: `gpu_instance_batch/mod.rs:97-101` (`GpuBatchedMesh3d { mesh: Handle<Mesh>, max_capacity }`),
`render/mesh.rs:1348-1353` (`RenderMeshInstanceBatch { asset_id, input_range, world_center }`).
`gpu_preprocessing.rs:2253-2270` (`prepare_instance_batch_indirect`) allocates **exactly one** indirect parameter per batch and passes
`batch_set_index: None`, so the `MULTI_DRAW_INDIRECT_COUNT_SUPPORTED` path (`build_indirect_params.wesl:99-120`) is never taken for
instance batches, and `render_phase/mod.rs:1232-1261` then issues **one draw call per batch per view**. §6 as written therefore makes
every instance in a batch draw the *first surviving instance's level* — `MeshInputUniform` does carry per-instance
`first_index_index`/`index_count` (`render/mesh.rs:634-650`), they are simply ignored. Only two shapes work:
- **(a) one batch entity per `(chain, level)`** — then draws/bins per view = distinct chains × levels (thousands), and §5's "the draw
  count is the number of batch sets" is false;
- **(b) a GPU-built indirect command list** — per batch set, one command per `(chain, level)`, with counts and `base_output_index` produced
  by GPU count → prefix-sum → scatter. This is SPUD's three-pass counting sort (`spud-code-2026-11-08.md:45-50`) moved to compute, and it
  needs `indirect_parameters_metadata[].base_output_index` (today CPU-written, `gpu_preprocessing.rs:2585-2599`) to become GPU-written.

(b) is what the spec's goal statement actually describes; §4 step 7 ("emit a work item") hides all of it. **Pick (a) or (b) explicitly
before S3b** — "batch entities fed from the baked tables" is ambiguous between them, and they imply different work-item layouts, budgets
and patch sizes. **Independently corroborated:** the parallel port audit (`s3a-port-audit-2026-11-09.md` §A2) reached the same conclusion
from the port side, and adds two implementation blockers this review did not look for — `unpack_ranges` is registered only in `Core3d`, so
lamp shadow views read **uninitialized** work-item slots (its §A4; note the real fix lands in #26149's `Core3dShadows`, which per B4 we
now own outright), and `BinnedRenderPhase::batch_set_count()` was never updated for instance batches, so the one number §Budgets gates on
reports zero extra draws for all statics (§A5). Read the two reports together; they do not conflict anywhere.

**B2. Per-view output memory is sized by reserved capacity, not visible count.** `gpu_preprocessing.rs:2585`:
`let output_base = data_buffer.add_multiple(count_u32 as usize)` with `count_u32` = the batch's whole `input_range` — per view, every
frame. `MeshUniform` is ~176 B (`render/mesh.rs:561-601`). At the spec's own 100k instances × ~8 views that is **≈ 140 MB**, and ≈ 560 MB
if LOD becomes separate batches under B1(a). The spec's only memory line is the 8 MB *input* table (static-scene.md:321-322). Against an
8 GB 4060 Laptop carrying DLSS, a shadow cube array (up to 192 MiB, shadows.md:246-248) and Solari — and worse on the Steam Deck floor —
this is not affordable. Fix: GPU-side output allocation (part of B1(b)), or per-view capacity = measured max visible with a spec'd
overflow policy.

**B3. Chain mesh residency, refcounting and slot allocation are missing from §3.** §2/§3 define a global dense chain slot keyed by mesh
key and `write_remapped` fills instance records with it (`table.rs:384-404`), but nothing owns that global table — `alloc.rs` is only the
*instance* slot allocator. Undesigned, and all S3b work: who allocates and frees a chain slot; who holds the `Handle<Mesh>` (chains are
shared across cells, so refcounted by cell); what the cull does with an instance whose chain mesh is not yet in `MeshAllocator` (today: a
garbage index range); and who *builds* the chain mesh — `chain_layout` is "shared by the bake and the runtime (`sf_scene_bevy`, which
builds the buffer)" (`chain.rs:13-14`), and that per-chain concatenate-and-rebase is real stream-in CPU work in no budget and no streaming
stage (streaming.md:218-273 lists fetch/assets/spawn/activate/upload only).

**B4. Two upstream PRs the repo treats as live are dead, and one cited bug was retracted.**
**bevy#26149 (`Core3dShadows`) and #26161 (`SkipRootViewRendering`) were closed unmerged on 2026-10-30**, self-closed by their author,
branches deleted, with zero reviews between them; they were `S-Blocked` + `AI-Assisted` and died of review starvation (underlying issue
#25576 still open). So `vendor/README.md:12`'s "Drop when: the Bevy release that ships both PRs" can never be met: these are an **own
fork**, not "a vendored patch carrying an upstream PR" (CONTRIBUTING rule 4), and the whole shadow reset B1/B2 result (shadows.md:231,
hard-problems.md H2 "perf is solved") rests on code we alone maintain. §6:207-209 and slice S5 ("Shadow views … + #26161 count") plan
around a PR that no longer exists; with S6 below, S5 needs rewriting rather than amending. Separately, **bevy#26456 was closed as a
duplicate on 2026-11-08 and its reporter retracted the slab-growth diagnosis**: the real bug is #25595 (macOS corruption since 0.19) and
the fix is #26095, which we **already vendor** (`vendor/README.md:22`). static-scene.md:145 cites it as a live bug; correct that, though
the decision it justified (store a slot, resolve offsets per frame) stays right for the reason in CONFIRMED-GOOD. The durable lesson, and
why this is a BLOCKER and not a doc fix: **"carries an upstream PR" is not a stable property.** Every vendored row needs a stated answer
for "the PR dies", and `Drop when` should read "upstream release **or** we own it" where that is honest.

## Findings: SERIOUS

**S0. `is_active` — all of §1's promotion — is silently ignored in views without GPU frustum culling, and we have such views.** The gate
sits inside `@if(FRUSTUM_CULLING)` (`mesh_preprocess.wesl:190-194`), and that def is only set for the indirect pipelines:
`direct_preprocess` is specialised with `PreprocessPipelineKey::empty()` (`gpu_preprocess.rs:2095-2104`). Views with `NoIndirectDrawing`
take that path, and perf.md names ours — "a view with `NoIndirectDrawing` (`PreprocessingOnly`, e.g. our extra sun views in `sf_sky_bevy`
`sun_view.rs`) culls a `NoCpuCulling` mesh nowhere" (perf.md:51-54). A promoted placement would be drawn **twice** there (entity + stale
instance), as would everything the cull rejects — the failure mode that made `NoCpuCulling` cost 81 ms (hard-problems.md:22-23). A
correctness hole in §1 + §8, not a tuning issue. Fix upstream (move the `is_active` test out of the `FRUSTUM_CULLING` block — two lines,
eminently upstreamable) or guarantee every view that sees the table draws indirectly.

**S1. "LOD chains share vertices" is false against our own cooker, and the memory estimate goes with it.** §2:103-105 and the Memory risk
(:321) claim shared vertices. `sf_model::cook` finishes *each level* with `meshopt::optimize_vertex_fetch`, producing a compact per-level
vertex buffer (`cook.rs:449-455`), so `chain_layout` **concatenates** them, incrementing `vertex_base` per level (`chain.rs:76-95`; its
own module doc says so at :5-9). Real cost ≈ 1.7-2× the LOD0 vertex memory per chain. SPUD shares vertices because meshoptimizer's
`simplify` returns indices into the *original* array; we traded that for vertex-fetch locality. Either re-cook chains against one shared
vertex array — a `mesh.cook` format change, so decide **before** any `scene.cells` bake — or correct §2 and the memory budget.

**S2. The batch key must include the mesh slab, and must be the index range, not the level index.** §5 says "pipeline × bind group × mesh
slab" but the work item keys only on the material class (`cull.rs:171-173`). *Slab:* a chain's `first_index`/`base_vertex` are
slab-relative and a draw binds one index buffer, so the key is at least `(class, slab)`, resolved per frame from `MeshAllocator` (the spec
is right that slabs move: `allocator.rs:285-303`; with `max_slab_size` 512 MiB and `large_threshold` 256 MiB, `slab_allocator.rs:150-191`,
expect 1-2 slabs per vertex layout, so the ≤2×-classes *batch-set* target is reachable — but only if the key carries the slab).
*Level vs range:* a level with no geometry for the part repeats the finer level's exact `LodLevel` (`chain.rs:83-86`, correct, tested at
:122-129), so `pick_level` can return a coarser *index* with an identical *range* (`cull.rs:136-147`), minting a second batch for
byte-identical geometry under B1(a). Key on `(chain, first_index)`.

**S3. The CPU twin implements only the perspective form while `SceneView` claims to serve shadow views.**
`SceneView::pixels_per_radian` is documented "(shadow views: their texel density)" (`cull.rs:118`), but `cull_instance` uses
`ppr·r/√(d²−r²)` for the sub-pixel test and `e·scale·ppr/d ≤ lod_px` for LOD (`cull.rs:206-217`) — both distance-dependent, while §4 step 5
correctly gives the orthographic rule **no distance term** (`e · texels_per_metre ≤ lod_px`). There is no projection kind on `SceneView`
and no ortho branch anywhere, so S5 will either implement the ortho rule on the GPU and fail the "GPU vs CPU twin" test
(Verification:286-288) or copy the perspective rule and take coarse far-cascade geometry at near distances. Add
`SceneProjection::{Perspective{ppr}, Orthographic{texels_per_metre}}` before S5.

**S4. The sub-pixel cull in shadow views repeats a measured failure; the industry does it, but per cascade.** §8 runs §4 in every view and
`size_cull_px` is unconditional (`cull.rs:211-214`). perf.md records the caster size rule being **reverted the same day**: "its cascades
refit with the view, so small casters lost and regained shadows as the camera turned" (perf.md:55-57). Unity shipped the correct version in
6.6 — small-mesh culling that "includes shadow casters, **which have their own thresholds per cascade**" (Unity staff, 2026-11-09) — and
UE applies a Nanite **LOD bias** for shadow raster rather than the main-view target (default 2 px,
`r.Nanite.ShadowRaster.PixelsPerEdgeScaling`). So: spec `size_cull_px = 0` for shadow views now (`cull.rs:211` already treats 0 as off),
and make any caster size rule a per-cascade threshold with a stable cascade fit, in its own measured slice. Same shape for §4 step 5's
shadow LOD: a per-cascade bias on `lod_px` is what ships; make it a knob, not a derived constant.

**S5. §9 (far field on the same table) deletes a measured, working mechanism and reuses an argument that does not hold for
representation swaps.** §4 "Swaps" argues no fade is needed because a level change is sub-pixel — true for levels of one chain (same
vertices, same material). §9 extends the pixel rule to L0→L1→L2→L3, where the swap changes geometry *and* shading model: L1 is a merged
mesh with vertex-colour albedo or a `FarTextures` array, L2 drops small objects, impostors are octahedral billboards (far-field.md:49-77).
Those swaps pop. Worse, the current no-hole guarantee is the **lead bands**, implemented as `VisibilityRange` on L0 chains and node
ranges, measured at 0.26 ms GPU and *kept on evidence* (far-field.md:74-100) — and §4:178-180 removes `VisibilityRange` from statics
outright, so the moment S4 lands L0's range end and the lead-band overlap disappear while S7 is three slices later. Either keep a
per-instance draw-distance end in the cull until S7, or merge S7 into S4. §9's residency rule ("a node draws only when its children are
not all resident and selected") is also a CPU decision over GPU-selected state, designed nowhere.

**S6. The lamp shadow cache and #26161 both go blind to the static scene.** *Ordering:* the cache empties a cached light's
`CubemapVisibleEntities` "after Bevy's `check_point_light_mesh_visibility`" (shadows.md:236-239); the S3a port adds batches to the same
lists in `add_gpu_batched_meshes_to_light_visibility`, also "PostUpdate, after `CheckLightVisibility`" (`vendor/README.md:59-61`). The
unconstrained order means either the cache stops working for statics or the batches are added after the emptying; needs an explicit
`.after()`. *Staleness:* #26161's skip hash reads `BinnedRenderPhase::entity_count`, which the port makes count **one entity per batch**
(`vendor/README.md:51-52`) — insensitive to changes in the *instance* set (stream-in, promotion, the cull), so a cached lamp face can keep
a stale map. §6:207-209 names the symptom but points the fix the wrong way: the hash needs a stream-in/promotion generation counter, since
B1's per-batch visible counts are GPU-side only. (And per B4, #26161 no longer exists upstream.)

**S7. `prepare_preprocess_bind_groups` (the known 2.3 ms, hard-problems.md:24) is not addressed and may get worse.** Each
`GpuBatchedMesh3d` owns an output buffer whose "identity changes when the capacity changes" (`gpu_instance_batch/mod.rs:119-123`) and
materialize binds per batch (`materialize.wesl:12-16`). Under B1(a) that is an extra bind group per `(chain, level)` and a buffer realloc
+ bind-group rebuild **whenever a stream-in grows a batch's capacity**, which also breaks "Stream-in of a cell ≤ 0.2 ms upload, zero
entity spawns": capacity must be sized once per space, not grown per cell. The real upstream fix is bevy#26257 (`vendor/README.md:31`);
depend on it or carry it, and give this its own budget row — it is 20 % of the 11 ms frame.

**S8. §4 step 6 (Hi-Z occlusion) does not exist on the deferred path.** Bevy: "occlusion culling is currently incompatible with deferred
shading; including both `DeferredPrepass` and `OcclusionCulling` results in unspecified behavior" (`occlusion_culling/mod.rs:47-49`) — and
our own code already enforces it: `sf_sky_bevy/src/path.rs:372` removes `OcclusionCulling` when the deferred prepass goes on, :403
restores it. §4 never says step 6 is conditional on the shading path. (`is_active` and frustum culling survive deferred; only Hi-Z does
not — see S0.)

**S9. #25978 is untested in every configuration we need, and its upstream prognosis makes the "Drop when" condition unrealistic.** Its
Testing section covers macOS/Metal (M2 Max) with `gpu_particles`, `transparency_3d`, `mesh2d`, `mesh2d_alpha_mode` — **no
occlusion-culling example, no shadow scene, no Vulkan or DX12 run, no CI screenshot test** — so every §6/§8 assumption (shadow views,
Hi-Z, cascades, `multi_draw_indirect_count`) is asserted, not demonstrated. Upstream today: open, `mergeable_state: dirty`,
`S-Merge-Conflicts` since 2026-10-17 and never cleared, **no milestone**, last push 2026-09-30, **one** review ever (2026-10-03, a
cosmetic nit asking `is_active` → `flags`), `AI-Assisted`, in a queue of 187 open `A-Rendering` PRs. Bevy's 0.21 dev release notes contain
**zero** rendering entries; pcwalton, who landed the whole GPU-driven line (#23662, #23892, #24078, #24343), has opened no PR since
2026-08-04; there is no GPU-driven tracking issue (the nearest, #12590, last touched 2025-02-05). Two open PRs collide on the same files:
**#25956** ("keep each sorted instance in its own indirect draw" — opposed to "a batch is one sorted item with N instances") and
**#22284** (MeshPass v2). Plan "GS" as an own fork we offer upstream. Credit where due: §7 already reads that one review comment right.

**S10. Slot-allocator fragmentation has no end state.** `SlotAllocator` is first-fit, coalesced, never compacting, and `high_water` only
grows (`alloc.rs:1-8, 86-110`). Cells have very different instance counts and each needs one *contiguous* range (§3), so a long session
across Vvardenfell fragments and the buffer creeps; `SlotStats::fragmentation` reports it, nothing acts. Either size-class pools, or drop
contiguity (a per-cell range list the dispatch reads) — cheap, since the cull already dispatches per visible cell range.

## Findings: MINOR

**M1/M2, the LOD pick.** `max_scale` is the longest column of the 3×3 (`table.rs:181-185`), a *lower* bound on the spectral norm (equal
only for orthogonal columns): exact for Morrowind (rotation × uniform scale), too coarse for sheared or non-uniformly scaled placements —
document the assumption or use `sqrt(λ_max(MᵀM))`. And `pick_level` does not stop at the first failing level (`cull.rs:136-147`), so a
non-monotone error list picks past a rejected one; the cooker is monotone (`cook.rs:271,295` carry `prev_error` through `error.max(e)`)
and `chain_layout` preserves it, but `validate` checks only finiteness and sign (`table.rs:360-362`). Add the monotonicity check and
`break` — cheaper in WGSL too.

**M3/M4, the horizon test.** §4:158 calls it "conservative". It is not: `land_hides` samples **9 points** on the box top
(`cull.rs:333-345`), so terrain between samples can pass a sight line while the cell is still culled. SPUD accepts the same
approximation; say "approximate, may over-cull" so a ridge pop is recognised as this and not a bug. Also, the grid is built at stream-in
from loaded cells' `land_min` (§4:159-161) and unloaded cells read `NEG_INFINITY` (`cull.rs:289-295`, safe) — but far-field L2/L3 nodes
live past static-cell residency (far-field.md:116-120), so from S7 the test silently stops working for exactly the content it would help
most. Feed the grid from the terrain clipmap's residency instead.

**M5/M6, format and doc drift.** `material.rs:10-11` still says the class is stored per instance as a `u16` — stale since v2 moved it to
the chain record (`table.rs:40`). `MaterialClass` has no values for the GI (`1<<26`) or surface-weather (`1<<27`) keys §5 counts toward
"≤ ~16 classes", nor for the leaf-transmission split S2 found unavoidable (perf.md:302): the enum is extensible but the class budget needs
a recount. And `layers` is one `u32` per table (`table.rs:195-197`), so a cell whose statics ever need two masks cannot be one table and
there is no split mechanism — fine today, a format v3 driver if lamp-layer statics appear.

**M7.** Instances are sorted by `(placement, part)` for `find` (`table.rs:9-13, 466-487`), but the GPU scatter wants them grouped by chain
(coherent atomics, contiguous per-command runs) — under B1(b) that is the difference between a tight counting sort and a fully random
scatter. Chain-major order plus a sorted placement index is a v3 candidate.

**M8.** `multi_draw_indirect_count` is **disabled on DX12** in our vendored Bevy regardless of adapter support
(`render_phase/mod.rs:1278-1282`, "TODO: gfx-rs/wgpu#7974"); §6:204-206 and the "Driver support" risk (:319) should say the DX12 fallback
is one draw per batch, not `multi_draw_indirect`. The two sides also disagree — the *shader* key comes from the raw device feature with no
DX12 exclusion (`gpu_preprocess.rs:2132-2135`) while the *draw* applies it — which looks like an upstream bug; **unverified** (no DX12
run, possibly unreachable). Vulkan (4060 Laptop, Steam Deck) is unaffected.

**M9/M10, duplicated and per-frame work.** §4 step 2 (sphere vs frustum) **duplicates what Bevy's preprocess already does per view**:
`mesh_preprocess.wesl:290-305` frustum-culls every work item from `MeshCullingData`, which `materialize.wesl:31-34` fills from the batch's
mesh AABB — two sets of bounds, two tests. Consider making §4 = layers + active + horizon + sub-pixel + LOD and letting Bevy own frustum +
Hi-Z, which also removes the risk of the two disagreeing at the edges. And "retained" is weaker than §3 implies: materialize dispatches
`capacity/64` workgroups **per batch, every frame** (`gpu_instance_batch/materialize.rs:210`), rewriting `MeshInputUniform` (112 B) +
`MeshCullingData` (32 B, `render/mesh.rs:768-777`) for every reserved slot regardless of visibility, into a per-frame scratch range
(`gpu_instance_batch/mod.rs:438-442`) — ~14 MB/frame of writes at 100k instances, affordable but not SPUD's "nothing about statics is
rewritten per frame" (`spud-code-2026-11-08.md:38-39`), and it is *why* §7's `previous_input_index` trick works only for statics: input
indices are not stable across frames.

**M11. The "~41 % fps in dense areas" credited to horizon culling is not an isolated measurement.** §4:159-160 and hard-problems.md:48
both cite it, but the Greatness7 MGE XE fork's README presents its numbers as **bundled** — "Cells that used to cost hundreds or
thousands of draw calls now cost one (or zero if occluded)… **6x FPS improvement**… Very large draw distances can see **15x or more**" —
over LOD simplification, a shared atlas, per-cell merging *and* runtime occlusion culling together; no horizon-only figure is published.
The 41 % may come from our own earlier audit (`research/mge-xe-code-2026-10-25.md`): re-check before quoting it in a budget. Horizon
culling is still worth doing on SPUD's 30-line implementation alone. Two further corrections: the fork selects detail by **size class ×
distance ring**, not a screen-space pixel rule (its error budget is *offline* and world-space: `generator.statics.target_error`,
"relative error budget vs. bounding radius, 0.01 = 1 %"), so it is no precedent for our pixel rule; and its horizon data is a
**max-height** grid plus per-subset convex footprints (`XEOCCL02`, `HorizonFootprint { max_z, footprint_xy }`) — a different algorithm
from our sight-line-vs-land-minimum march, and ours is the more conservative.

## Findings: CONFIRMED-GOOD (do not touch)

- **Format v2's cell-local dense chain indices + remap at stream-in** (`table.rs:19-27`, `write_remapped` :384-404): the right fix for
  v1's colliding folded ids, and what makes a cell loadable on its own. The reader is properly defensive — every cross-reference
  validated, malformed input an error and never a panic.
- **Chain slot indirection against slab movement** (§3:144-146). `meshes_displaced_by_slab_growth` is documented as *not* the full set of
  changed buffers (`allocator.rs:285-296`), so resolving offsets per frame from the allocator is the only correct choice, not caching them.
- **Per-part bounding spheres** (§2:134-135, `bake.rs:30-35`): a door handle culling as itself rather than as its building is right, and
  the argument that coarser levels need no margin (the simplifier only drops LOD0 vertices) is sound. **Origin shifts** (§3:147-148):
  `cell_offset` takes the delta in `WorldPos` before narrowing to f32 (`lib.rs:50-54`), so a `big_space` shift rewrites nothing.
- **The horizon test using the land *minimum*** as blocker: if the segment dips below a cell's land minimum it is underground there, so the
  line is blocked — conservative in Y, and "the lowest point in a cell is at entry or exit" is a correct basis for the Amanatides-Woo
  march (`cull.rs:347-351`). Note this is the *opposite* grid from the Greatness7 max-height lead, and the min version is the sound one.
- **Deferred slot release after frames in flight** (`alloc.rs:112-125`); the one-way `Promotions` model; and the **eligibility adapter
  seam** (`eligibility.rs:7-10`) — framework names no game rules, `is_referenced` is conservative, mixed models split.
- **§7 motion vectors.** `previous_input_index` pointing at the instance itself is sound *because* statics have previous == current, and
  `MeshInputUniform::timestamp` already encodes "not updated this frame ⇒ transform unchanged" (`render/mesh.rs:658-665`). The wind and
  UV-scroll double-evaluation also satisfies render-order.md rule 3 (:83-87).
- **§4 step 6 composes with a GPU producer.** Bevy's two-pass occlusion culling is driven entirely by the work-item list and
  `MeshCullingData` indexed by input index (`mesh_preprocess.wesl:290-345`), the early→late handoff is pure GPU atomics and the late pass
  is dispatched indirectly; there is no per-instance CPU-extracted history. #25978's materialize already writes both
  (`materialize.wesl:30-34`), and our vendored #26170 fixes the edge-clamp bug. Only S8's deferred exclusion limits it.
- **S2 (material classes) as a standalone slice** paying on the current entity path — the right de-risking move, and the one slice whose
  value is independent of B1.
- **There is an upstream route for the per-instance material data §5 needs.** bevy#24922 (merged 2026-08-19) keys extracted per-instance
  component data by **`MeshTag`**, and #25978's materialize sets exactly that field (`materialize.wesl:28` `input.tag = instance.tag`;
  `MeshInputUniform::tag`, `render/mesh.rs:667`) — the CONTRIBUTING-rule-2 answer for per-instance tint, alpha cutoff and the bindless
  material slot, resolving the spec's open question (:326) without a format v3 field. §5 should name it.
- **The sub-pixel threshold is consistent with J1 and OpenMW despite looking like a 2× discrepancy:** `size_cull_px` is a projected
  *radius* (`cull.rs:208-214`), so High = 1 px radius = 2 px diameter — exactly J1's `s_min = 2` on `2r` (streaming.md:188-193) and
  OpenMW's 2 px small-feature cull. Say so in the spec so nobody "fixes" it.

## The eight questions

### 0. The "Industry comparison" table is wrong in four rows

The spec flags its own UE5/Decima/id Tech rows as "from memory (verify before quoting further)" (static-scene.md:54-55). Verified today:

| Row | Verdict |
|---|---|
| **UE5** | **Keep, two refinements.** Retained GPU Scene is right: `r.GPUScene.UploadEveryFrame` **defaults to 0** (full re-upload is marked "Useful for debugging") and dirty state is per primitive (`EPrimitiveDirtyState`). Nanite SIGGRAPH 2021: "Renderer now retained mode / GPU scene representation persists across frames / Sparsely updated where things change / **Per view: GPU instance cull**"; two-pass Hi-Z ("Main"/"Post") is Epic's own term. **(1)** Non-Nanite per-instance *occlusion* culling is **opt-in** — `r.InstanceCulling.OcclusionCull` defaults to 0; only frustum + LOD are always GPU-side for ISM/HISM. **(2)** Shadow views do run the same cull, and the VSM page names the explosion we are designing for: one instance becomes "**48 instances (8 mip levels × 6 cubemap faces) for each point light**… 17 for each Directional Light", "Instances are culled separately for each virtual shadow map mip/clipmap level." The strongest external evidence for this whole design. |
| **Unity 6** | **Keep, and fill the blank.** Instance data is "stored in a GraphicsBuffer and **remains persistent on the GPU**"; the GPU Resident Drawer updates on change. The spec's "(not documented on the page read)" shadow cell *is* documented, in the Rendering Debugger reference: "**Unity uses one view per shadow cascade or shadow map.**" Add the caveat: GPU occlusion culling is **off by default** and Unity staff today recommend "leaving it off unless your scene is very heavily instanced", because "on highly unique content the visibility check can cost more than it saves". |
| **Decima** | **Delete, or rewrite to the opposite.** The only public Decima material is GPU *procedural placement* (HZD, GDC 2017), which ends in a **CPU read-back**. The one public statement on general static rendering is McLaren, GDC 2022: "we query a **highly optimized scene graph**… **the scene graph performs occlusion culling**… spits out a visibility list… **batches are not necessarily consistent from frame to frame**", plus "we sort batches on the CPU". GPU compute culling there is *vertex-chunk*, not instance. GPU LOD, indirect geometry draws, shared shadow culls and draw distance: all **undocumented**. |
| **id Tech 6/7** | **Split; our row is wrong on LOD and shadows.** id Tech 6 (Doom 2016) documents nothing about instancing/MDI/bindless; culling is Umbra CPU plus fixed-function GPU occlusion *queries* (secondhand), 1331 draw calls/frame — and the famous "Done on CPU, 1 job per depth slice" line is light/decal binning, not geometry. id Tech 7 (Doom Eternal, SIGGRAPH 2020) is citable: fully bindless, "merge surviving triangles of multiple models in a single indirect index buffer", "one indexed indirect draw call per Geometry Set… effectively draws 256 meshes at once", GPU **triangle** compute cull "**in addition to all standard CPU culling**" (~70 % of triangles) — but "budget for 3M visible triangles **after CPU culling and LOD selection**", i.e. **LOD is CPU-side**, and shadows do **not** share the indirect buffer (cached atlas per light). |
| **Frostbite / Ubisoft** | **Relabel and correct one column.** The talk is **Ubisoft Montreal (AC Unity) + RedLynx**, not Frostbite. Persistent GPU scene confirmed verbatim ("Per instance data… **Persistent for static instances**"); 64-triangle cluster culling confirmed; two-phase Hi-Z confirmed and the direct ancestor of Nanite's Main/Post; shadow views on the same pipeline per cascade, "30-80 % shadow triangles culled". But **"cone" culling is wrong**: it is a baked per-cluster **cubemap triangle-visibility mask** (10-30 % of triangles). **And the line to quote in our own §Why:** "CPU: 1-2 orders of magnitude less drawcalls… **GPU: 20-40 % triangles culled… Only small overall gain: < 10 % of geometry rendering.**" The win was CPU-side — exactly our H1 thesis, from the canonical paper. |

### 1. Is the canonical technique correctly identified?

**Yes, and the divergence from SPUD's CPU cull is justified — for a reason the spec states imprecisely.** Retained GPU scene + per-view
GPU instance cull + indirect draws is the technique. But the spec's reason ("it scales with views and instance counts") is half right:
what H1 measures is cost per *view*, and what scales with views on the CPU is the **number of phase items**, not of instances. Under
B1(a) that becomes distinct-chains × views; under B1(b) batch-sets × views. Both beat 46-55k entities × 6-8 views, so the direction
holds — but say which number you are reducing, because S0 already measures it (`sf_diag` `views/<kind>/bins`, `sf_diag/src/views.rs:13-17`).

SPUD's CPU cull is not a worse technique, it is a different bottleneck: SPUD's main thread "has nothing left to cut"
(`spud-code-2026-11-08.md:24`) on a 2011 forward renderer with no upscaler and ~1.75 ms GPU, while ours is render-thread bound with
3.3-4.2 ms of graph *encoding* (perf.md:34). A CPU cull would not fix encoding, and SPUD's counting sort per view × 8 views over 100k
instances is exactly the per-frame CPU work we cannot afford. The divergence is justified. **But note what we also gave up:** SPUD's cull
is *where the per-(group,LOD) indirect command list is built*, and the GPU version of that is B1(b). We diverged on *where* the sort runs,
not on *whether* it is needed; the spec reads as if it were not. Alternatives not dismissed too fast: **meshlets/Nanite-style** correctly rejected (Bevy's are
opaque-only, Vulkan/Metal-only, need per-mesh preprocessing, §Why:61-63, and Morrowind is full of alpha-masked foliage); **visibility
buffer** measured by SPUD at best case ~0.4 ms, expected 0-0.2 ms after losing hardware interpolation, not built
(`spud-code-2026-11-08.md:137-139`); **CPU cull + MDI (SPUD as-is)** is the honest fallback if B1(b) proves too large — shipped and
measured, skipping the Bevy indirect patch at the cost of a bypass phase. The spec lists only "our own renderer" as the rejected CPU
option, which is not the same thing; write the fallback down.

### 2. Does the data model (§2, format v2) hold up?

Mostly yes. Holds up: the 80 B/16-aligned instance record as the GPU layout, the 32 B chain record, cell-local dense chain indices,
per-part spheres, `land_min` in the header, `material_override` reserved and rejected, `INACTIVE` runtime-only and rejected on read.
Format v3 drivers beyond `material_override`: per-instance tint / vertex-colour variation (the spec's own open question :326 — but see the
`MeshTag` route in CONFIRMED-GOOD, which avoids a field); a per-instance LOD bias or forced level (UE's `MinLOD`/`ForcedLOD` exists for a
reason); chain-major instance order (M7); two layer masks per cell (M5/M6); and under B1(b) a sub-cell cluster id so the dispatch can skip
runs.

**LOD-chain-as-one-allocation vs `MeshAllocator`:** fine. A mesh is allocated contiguously in one slab, or gets its own buffer above
`large_threshold` (256 MiB, `slab_allocator.rs:163-171`), so a chain's level ranges are one contiguous index range plus a per-frame slab
offset; with `max_slab_size` 512 MiB all static chains of one vertex layout sit in 1-2 slabs, which is what makes §5's batch-set target
reachable *if* S2's finding holds that statics carry no normal maps or extra UV sets (perf.md:302) — different vertex layouts sort into
different general slabs (`allocator.rs:780-785`). The *vertex-sharing* half of the claim is false (S1).

### 3. Is the cull/LOD order (§4) right and complete?

The order is right (cheapest and most-rejecting first; layers and horizon hoisted to the CPU per cell, correctly, since both are per-cell
facts). The perspective math is right: `e·ppr/d ≤ lod_px` is the pixel-error rule, `ppr·r/√(d²−r²)` is the exact tangent-based projected
radius (not the `r/d` approximation), `d` to the sphere *surface* with 0 inside conservatively forces LOD0 up close, and `ppr` at the
**internal** resolution is the correct DLSS answer.

Errors and gaps by cost: **S3** (no orthographic form, though §4 specifies one), **S4** (sub-pixel cull in shadow views), **S2** (level vs
index range), **M1/M2**, **M3**, **M4**. Two more:

- **Surface-distance LOD breaks for large bounding spheres.** `d = max(|c−eye| − r, 0)` (`cull.rs:216`) is 0 whenever the eye is inside the
  sphere, and for a §9 far-field L2/L3 node (km-scale bounds) the eye is *always* inside, so the node is pinned to LOD0. The rule needs the
  distance to the error-bearing surface, which for a proxy is not the sphere surface; §9 must specify this.
- **No cell-range indirection list.** "One dispatch per view over the visible cells' ranges" (:152) needs a per-view, per-frame list of
  `(slot range, cell offset)` — small (hundreds), but CPU work per view that no budget mentions, and where the `big_space` offset enters.

**Is the pixel-error rule with no draw distance sound at Morrowind scale with DLSS?** Sound for the *triangle* budget; not on its own, and
Nanite's authors say so. Karis, SIGGRAPH 2021: "**Tiny instances** — What happens when entire mesh only covers a few pixels? DAG ends at
1 root cluster / **128 triangles** / Stops scaling with resolution / Cull when too small? **Can't if structural building block**", then
"**Obviously need to merge at some point** / Even if rendering scales sublinearly, memory doesn't / **10M × float4x3 = 457 MB** / Nanite
has no special solution for merging / **Merged unique proxies must replace instances at extreme distances** / Key improvement is pushing
that distance far away", and "the property of scaling cost with screen resolution doesn't hold with **aggregates such as grass and
leaves**" — i.e. Morrowind foliage. UE ships the pixel rule *paired with* World Partition streaming and HLOD proxies, capped at 16 M
instances; Unity keeps a hard "culled" LOD band plus far-clip and per-layer cull distances. So §4's "No hard draw distance" is correct
**only as a statement that §9 exists**, which is the ordering problem in S5; say that rather than presenting it as the simplification.

On DLSS: taking `ppr` from the internal resolution is the right inference and matches UE — whose `r.Nanite.ViewMeshLODBias.Offset` exists
precisely "for the main viewport **when using TSR**" — but it *is* an inference: NVIDIA's DLSS Programming Guide 310.1.0 covers the
*texture* mip bias at length and says nothing about geometric LOD beyond "the ability to negatively bias the LOD for textures and
geometry" (§2.2). The actionable lesson is that UE needed a **tunable offset** under temporal upscaling, so `lod_px` should take a
DLSS-dependent offset rather than being a pure preset constant.

### 4. Is drawing through Bevy's own pipeline via a vendored patch (§6) the right boundary?

**Yes, the boundary is right; the cost estimate is wrong.** Rejecting a public-API bypass is correct and for the right reasons (it forks
material specialisation, prepass and motion vectors, and would collide with eight of our own `bevy_pbr` patches — GI probes, surface
weather, PCSS, DLSS viewport, `vendor/README.md:16-27`). Rejecting our own renderer is correct (loses Solari, DLSS, GI). CONTRIBUTING
rule 4 is satisfied.

Maintenance cost, **measured** (commits on `main` touching each path between release tags, per-path line counts, GitHub commits API):

| Path | 0.17→0.18 (3.5 mo) | 0.18→0.19 (5.2 mo) | 0.19→0.20 (3.7 mo) | 0.20→main (1.0 mo) |
|---|---|---|---|---|
| `bevy_pbr/src/render/mesh.rs` (4,973 lines) | 14 commits, +229/−97 | **47, +2537/−1077** | 18, +226/−164 | 2, +8/−1 |
| `bevy_render/src/batching/gpu_preprocessing.rs` (3,198) | 2, +132/−87 | **21, +954/−388** | 7, +720/−275 | 0 |
| `bevy_pbr/src/render/` (dir) | 33 | **103** | 51 | 10 |

#25978's own footprint is **+251/−70 on `mesh.rs` and +253/−5 on `gpu_preprocessing.rs`** — its single-PR footprint on the latter exceeds
that file's entire 0.17→0.18 churn, and 0.18→0.19 turned over ~50 % of `mesh.rs` and ~30 % of `gpu_preprocessing.rs` in one release. So:
**a quiet release costs hours; a cycle like 0.18→0.19 costs a rewrite of the patch.** Budget 1-2 days per release for "GS" normally and
one full slice when the renderer moves, and treat upstreaming as load-bearing. We already carry eleven PR-backed rows plus nine own
patches here, two now permanent forks (B4), and the S3a port needed two manual hunks plus one deviation (`vendor/README.md:48-61`).
Mitigation the spec should add: keep our additions in *new files* under `bevy_pbr/src/gpu_instance_batch/` (as the port already does for
`materialize.wesl`) and as separate WGSL modules, not hunks inside `build_indirect_params.wesl`. Building on an unmerged PR is acceptable
*because* we need the same code anyway — but per S9, state the fallback: if #25978 stalls, "GS" is our own fork. The `GpuBatchedMesh3d`
surface is ~100 lines; the value in #25978 is the *shared-path* changes (`MeshCullingData::is_active`, `RenderMeshDraws`, range
unpacking), which we would have to write and upstream regardless.

### 5. Does the streaming integration (§3, streaming.md) actually compose?

**Partly.** Composes: the per-cell table upload is tiny against the 8 MiB/frame budget (streaming.md:250) — 80 B × a few hundred
instances; the free list with release deferred by frames in flight is correct; origin shifts are a non-event; "no entity spawns" holds for
*placements*. Does not compose: **B3** (chain mesh residency, refcount and build cost undesigned and unbudgeted), **S7**
(`max_capacity` growth reallocates a batch's output buffer and rebuilds its bind group, so "zero entity spawns, ≤ 0.2 ms" is wrong for
any stream-in introducing a new chain or overflowing a batch), **S10** (fragmentation).

Rings: the pixel rule with no hard draw distance (:174-175) is in tension with R1 at 768/896 m and with J1's size-based draw distance
(streaming.md:155-216). The spec's answer — "streaming decides what is resident, the pixel rule decides what is drawn" — is right *once
§9 exists*; between S4 and S7 it does not (S5). Note also that J1 measured only 3 % of placements culled at High (streaming.md:188-190),
so the pixel rule is not where the win is — the win is the per-view machinery, and the spec should not sell the LOD rule as a perf feature.

**Far field (§9) is the weakest section.** Beyond S5: L1 proxies are unique merged meshes used once, so the mesh-key dedup that justifies
the global chain table buys nothing there; impostors need per-instance orientation the 80 B record has no room for; L1's `FarTextures`
extension is a different `AsBindGroup` *and* a different vertex layout (per-vertex array index), hence a different class and slab. One
paragraph needs to become a spec section before S7 is briefed.

### 6. Are the budgets realistic?

| Budget | Verdict |
|---|---|
| CPU per view ≤ 0.05 ms, independent of instance count | **Fantasy as written.** Independent of *instances*, yes; but under B1(a) it is O(batches) = O(chains × levels) per view, each an `IndexMap` lookup + `add_multiple` + `prepare_instance_batch_indirect` + `push_range` (`gpu_preprocessing.rs:2570-2612`). At 1k batches that is ≫ 0.05 ms. Realistic only under B1(b). |
| Render thread ≤ 1 ms, all statics, all views | **Fantasy as written.** One draw per batch per view (`render_phase/mod.rs:1240-1260`), and instance batches never join a multi-draw batch set (`gpu_preprocessing.rs:2267`). Thousands of draws × 6-8 views against today's 3.3-4.2 ms *total* encoding (perf.md:34). Realistic only once B1(b) gives real `multi_draw_indirect_count` batch sets. |
| GPU cull ≤ 0.3 ms, all views, 100k instances | **Plausible** and the cheapest claim here (~800k preprocess invocations + 100k materialize). Unverified, right order of magnitude. Watch write traffic (B2, M10), not ALU. |
| Stream-in ≤ 0.2 ms upload, zero entity spawns | **Wrong**: ignores chain mesh build + upload (B3) and batch capacity growth (S7). |
| Batch sets per view ≤ 2 × material classes | **Right target, wrong metric.** Reachable for *batch sets* (Q2's slab analysis); not for bins/draws without B1(b). The spec uses the two interchangeably. |
| `prepare_preprocess_bind_groups` 2.3 ms | **Not addressed, and at risk of getting worse** (S7). 20 % of the frame budget; it deserves its own row and a dependency on bevy#26257. |

The frame is already 9.5-11.1 ms with 7.2-9.0 ms render thread (perf.md:32): the design must *remove* several ms, not add any, and B2's
memory and S7's bind groups are both ways it could add instead. The Steam Deck floor makes B2 harder, not easier — no DLSS
internal-resolution discount on per-view output buffers, and one unified memory budget.

### 7. Is the slice order (S0-S9) the right de-risking order?

Mostly, with three changes.

1. **Insert S3a.5, a spike, before S3b** — the single most important change (below).
2. **Move the horizon cull out of S6 into its own slice now, on the current entity path.** Pure CPU, pure `sf_scene`, already written and
   tested (`cull.rs:259-410`), independent of the GPU scene, and even discounted per M11 the cheapest remaining H1 win. Same shape as S2.
   At S6, behind five slices that all depend on B1, it may not land for months.
3. **Do not do S4 (main view, all classes, LOD) before S5 (shadow views).** The thesis is "cost per view is flat × 6-8 views"; S4 proves
   nothing about the multiplier, and the intermediate state is worse than either end: an object drawn from the table in the main view
   still needs its shadow, so its entity must stay alive and "shadow only" in every shadow view — double residency, double memory, and a
   new correctness surface (render-order.md rule 4: shadows must match coverage, alpha cutoff included). Move whole material *classes*
   across *all* views at once, starting with opaque; that also keeps `--off scene` a clean A/B at every step.

S7 either merges into S4 or S4 keeps an explicit draw-distance end (S5). S8 (promotion, picking) is correctly late — CPU-only, rules
already written and tested — except that S0 makes promotion a correctness issue that cannot wait. S9 is correctly last.

### 8. What is missing from the design entirely?

1. **The counting sort / GPU indirect command list** (B1) — the largest omission. 2. **Chain lifecycle**: mesh build, residency, refcount,
   slot allocation, free (B3). 3. **Per-view output buffer sizing and its memory** (B2). 4. **Capacity policy**: what happens when a
   batch's `max_capacity` is exceeded (overflow, grow, drop?) and how capacity is chosen per space — #25978 warns on overflow, the spec
   has no policy.
5. **Cold start and pipeline warm-up.** SPUD's hardest-won lesson is that every pipeline is built up front through a persistent cache and
   that no IO, parsing or pipeline creation touches the render thread (`spud-code-2026-11-08.md:94-102`). This design *shrinks* the pipeline
   key space (a win for J7) but adds compute pipelines (cull, materialize, per-view variants) and never says they are precompiled. One
   `#[ignore = "measurement"]` for "frames until all static pipelines are resident" would catch it.
6. **Determinism / regression testing of what is drawn.** The part of SPUD's image-history audit that survives the no-automated-visuals
   rule is picture-free: compare per-view draw lists, LOD picks and resident cells after flying to a pose versus starting there
   (`spud-code-2026-11-08.md:104-112`). Route-dependent streaming bugs are exactly what this design can introduce. 7. **A debug path that
   survives S9:** `--off scene` disappears with the old path, leaving no A/B for the static renderer, while a `--scene-cpu` switch (draw
   the CPU twin's work items) keeps a bisect handle forever and costs little — the twin already exists.
8. **Transmission / two-sided foliage lighting** — a #25978 gap *and* an S2 finding (leaf transmission could not fold into one class,
   perf.md:302) — yet §5's class list has no transmission class and §10 does not mention it. 9. **Interiors** (the spec's own open
   question, :326): a few hundred objects each, hundreds loaded over a session — where S10's fragmentation and chain churn show first.
10. **An exit criterion.** Precombine halved mesh entities and the frame did not move (§Why:33-35). Every slice reports ms, none has a
    stated abort condition; given B1, S3b's number could be "no change", and the project should know in advance what that means.

## The one change to make before S3b

**Replace S3b with a two-day spike, S3a.5, whose only deliverable is the answer to B1 — then rewrite S3b, §5, §6 and the Budgets table
from its numbers.** (1) **Read the S0 baseline first:** `sf_diag` already reports `views/<kind>/{items, bins, batch_sets, work_items}`
(`sf_diag/src/views.rs:13-17`), and the `items / bins` ratio on `balmora_walk` *is* the ceiling on what #25978-as-is can win — bins are
per (mesh, material), and after S3b the bins are the batch entities; if `bins` is already a large fraction of `items`, option B1(a) is
precombine all over again and B1(b) is mandatory from S3b. (2) **Spike batch granularity on the synthetic scene** (no game data needed):
`N` `GpuBatchedMesh3d` entities (N = the measured distinct-chain count) × 4 levels, 100k instances, main view + 4 cascades + 2 lamp faces;
measure phase items and bins per view, draws per view, `prepare_preprocess_bind_groups` ms, render-thread encoding ms and the
`data_buffer` size (B2). That is the whole decision. (3) **Write the counting sort into §4 and §6 as a named pass**, or write down that we
accept B1(a) and change the budgets. Either way §4 step 7 must stop being one line, §5 must stop equating draws with batch sets, and §6
must stop claiming "LOD index ranges per instance" — `build_indirect_params.wesl` ignores them silently.

Everything else in this review is a fix inside a slice. This one changes which slices exist.

## Owner decisions this review would reverse

**None.** GPL licence, Delirium as the default physics, Steam Deck as the performance floor, spec-before-code, research-before-plan and
"future proof, flawlessly and fast, no quick fixes" are all untouched. B2 is the one finding where the **Steam Deck floor** tightens the
design rather than the 4060 Laptop, and it argues *for* the no-quick-fix answer (GPU-side output allocation). S4 switching the sub-pixel
cull off in shadow views reverses nothing: it restores a decision already taken on measurement (perf.md:55-57).

## Citations

**Repo** (paths from the repo root, lines as read 2026-11-09): `superfargoth/docs/specs/static-scene.md`;
`docs/research/spud-code-2026-11-08.md:24,38-39,45-50,94-102,104-112,137-139`; `docs/research/s3a-port-audit-2026-11-09.md:92-169`;
`docs/specs/streaming.md:155-216,218-273,626-640`; `far-field.md:49-120`; `shadows.md:224-248`; `render-order.md:71-100`;
`precombine.md:96-110`; `bevyrrowind/docs/hard-problems.md:19-50`; `bevyrrowind/docs/perf.md:30-40,51-60,76-110,302`;
`superfargoth/vendor/README.md:12-31,44-64,80-81`;
`crates/sf_scene/src/{lib.rs:50-54,table.rs,chain.rs,cull.rs,alloc.rs,material.rs,eligibility.rs,bake.rs}`;
`crates/sf_model/src/cook.rs:230-302,449-455`; `crates/sf_diag/src/views.rs:13-17`; `crates/sf_sky_bevy/src/path.rs:321-403`;
`crates/sf_world_bevy/src/gpu_batch_tests.rs`. **Vendored Bevy 0.20 + patches:**
`bevy_pbr/src/gpu_instance_batch/{mod.rs,materialize.rs,materialize.wesl}`; `bevy_pbr/src/render/{mesh.rs:561-677,768-777,1348-1390,
build_indirect_params.wesl, mesh_preprocess.wesl, gpu_preprocess.rs:2090-2135}`;
`bevy_render/src/{batching/gpu_preprocessing.rs:2253-2270,2555-2612, render_phase/mod.rs:1176-1282, mesh/allocator.rs:285-303,780-785,
slab_allocator.rs:150-191, occlusion_culling/mod.rs:44-50}`.

**Upstream Bevy** (PR pages + GitHub API, read 2026-11-09): **#25978** open, `mergeable_state: dirty`, `S-Merge-Conflicts` since
2026-10-17, no milestone, last push 2026-09-30, one `COMMENTED` review, `AI-Assisted` — `https://github.com/bevyengine/bevy/pull/25978`;
admitted gaps verbatim "Not supported here: motion vectors, transmissive materials, per-instance sorting, 2D"; tested only on macOS/Metal.
**#26149, #26161** closed unmerged 2026-10-30, self-closed, branches deleted, zero reviews (`/pull/26149`, `/pull/26161`); issue #25576
still open. **#26456** closed as duplicate 2026-11-08, diagnosis retracted; real bug #25595, fix #26095 (open). **#24922**
`GpuComponentArrayBuffer` per-instance data keyed by `MeshTag`, merged 2026-08-19. **#17413** two-pass GPU occlusion culling (merged
2025-01-27), graduated by **#22631** (0.18), shadow support **#17951**. **#25956** and **#22284** open and conflicting on the same files.
Churn figures: commits API with `?path=` between tags v0.17.0 … v0.20.0 and `main`.

**Industry:** Karis, *A Deep Dive into Nanite Virtualized Geometry*, SIGGRAPH 2021 —
`advances.realtimerendering.com/s2021/Karis_Nanite_SIGGRAPH_Advances_2021_final.pdf`. Epic docs under
`dev.epicgames.com/documentation/en-us/unreal-engine/`: *Mesh Drawing Pipeline*, *Nanite Technical Details*, *Virtual Shadow Maps*,
*Instanced Static Mesh Component*, *Creating and Using LODs*, *Visibility and Occlusion Culling*, *Scalability Reference*, *Foliage
Mode*, *World Partition — HLOD*, UE 5.4 release notes; cvars `r.GPUScene.UploadEveryFrame` (0), `r.InstanceCulling.OcclusionCull` (0),
`r.Nanite.MaxPixelsPerEdge` (1.0), `r.StaticMeshLODDistanceScale`, `r.Nanite.ViewMeshLODBias.Offset`,
`r.Nanite.ShadowRaster.PixelsPerEdgeScaling`. Unity (`docs.unity.com`, `docs.unity3d.com`): *DOTS Instancing shaders*,
*BatchRendererGroup — batches*, *GPU Resident Drawer*, *GPU occlusion culling*, *Rendering Debugger reference*, *LOD Group*,
`Camera.layerCullDistances`; staff status 2026-11-09 `discussions.unity.com/t/culling-update-now-and-next/1741459`. Haar (Ubisoft
Montreal) & Aaltonen (RedLynx), *GPU-Driven Rendering Pipelines*, SIGGRAPH 2015 —
`advances.realtimerendering.com/s2015/aaltonenhaar_siggraph2015_combined_final_footer_220dpi.pdf`. Geffroy, Gneiting & Wang, *Rendering
the Hellscape of Doom Eternal*, SIGGRAPH 2020 — `advances.realtimerendering.com/s2020/RenderingDoomEternal.pdf`; Sousa & Geffroy,
*idTech 666*, SIGGRAPH 2016 — `advances.realtimerendering.com/s2016/`; Courrèges, *Doom (2016) graphics study* —
`adriancourreges.com/blog/2016/09/09/doom-2016-graphics-study/`. Decima: van Muijden, GDC 2017 —
`guerrilla-games.com/read/gpu-based-procedural-placement-in-horizon-zero-dawn`; McLaren, GDC 2022 —
`media.gdcvault.com/GDC+2022/Speaker+Slides/AdventuresWithDeferred_McLaren_James.pdf`. MGE XE: `github.com/Hrnchamd/MGE-XE`, fork
`github.com/Greatness7/MGE-XE`, `mwse.github.io/MWSE/types/mgeDistantLandRenderConfig/`. NVIDIA DLSS SR Programming Guide 310.1.0
(`github.com/NVIDIA/DLSS`, `doc/`); AMD FSR mip bias (`gpuopen.com/manuals/fsr_sdk/techniques/super-resolution-upscaler/`); Intel XeSS
guide (`github.com/intel/xess/blob/main/doc/xess_sr_developer_guide_english.md`).

**Unverified, explicitly:** whether #25978 works with `OcclusionCulling`, on Vulkan/DX12, or with shadow casters (the PR tests none); the
Bevy project goals board (access-gated) and any rendering working-group notes (Discord-only); the DX12 `multi_draw_indirect_count`
inconsistency (M8); the origin of the ~41 % horizon figure (M11); whether Decima or id Tech document GPU-side static LOD selection (no
public source found); every GPU-cost estimate in the Budgets table, which only S0 and S3a.5 can settle.
