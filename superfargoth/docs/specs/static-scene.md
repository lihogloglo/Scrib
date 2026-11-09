# Static scene: a retained, GPU-driven static world (`sf_scene`, `sf_scene_bevy`)

Status: **building** (owner go 2026-11-08: "a future proof engine that can work flawlessly and fast"; no quick fixes).
Built: **S1a**, **S1c** `[x]`; **S0**, **S1b**, **S2**, **S3a** `[~]`, each with an owner run open (Slices).
**S3a2 is next, then S3b**; S4-S9 not started.

Corrected 2026-11-09 against three documents that hold the evidence for every citation below: the
[S3a port audit](../research/s3a-port-audit-2026-11-09.md), the
[architecture review](../research/static-scene-architecture-review-2026-11-09.md) and
[gs-patch-plan.md](gs-patch-plan.md). Both audits' verdict: the architecture is right, the spec was wrong about what the
ported upstream PR can express.

Replaces, once complete, per-entity statics (`sf_world_bevy` `spawn_template_ranges`) and the near-cell
precombine ([precombine.md](precombine.md)); absorbs far-field selection ([far-field.md](far-field.md)). Hard problem H1
(`bevyrrowind/docs/hard-problems.md`). Research: [spud-code-2026-11-08](../research/spud-code-2026-11-08.md).

## Goal

Draw every static, non-interactive object of every loaded space from **one persistent GPU instance table per space**,
culled and LOD-selected **on the GPU, once per view** (main camera, sun cascades, lamp cube faces, probe and reflection
views), and drawn through **Bevy's own material pipeline** as one indirect draw per batch set. Gameplay objects stay
entities. Done when:

- the CPU cost of statics per view is a constant (a few dispatches and draws), whatever the number of instances;
- `balmora_walk`, `flight` and every `benchmarks/exterior_*.json` gate pass at the target setup (High, 1440p + DLSS Q,
  RTX 4060 Laptop, ≤ 11 ms median) with headroom toward the 7 ms stretch, ×3 runs;
- the same holds for 10× the instance count (heavy mod lists, denser future content), measured on a synthetic scene;
- shadows, prepass, motion vectors (DLSS), fog, GI and the far field behave as today or better (owner's visual check);
- the Bevy changes are a vendored patch meant upstream, with a PR open.

Non-goals: skinned actors, particles, blended or sorted geometry, water, terrain (it has its own CDLOD path), our own
renderer, a CPU-side cull.

## Why

- **The cost is the per-entity machinery, not the meshes.** Statics are one entity per placement × LOD level × material
  part (`spawn.rs:300`), all levels alive and hidden by `VisibilityRange`: 46-55k mesh entities in a town, 65-88k
  entities in all (perf log 2026-10-19/20). Bevy extracts, queues, bins and writes work items per entity **per view**
  (`extract_meshes_for_gpu_building` `bevy_pbr/src/render/mesh.rs:1979`, `queue_material_meshes` `material.rs:1189`,
  `queue_shadows` `render/light.rs:3412`, `batch_and_prepare_binned_render_phase` `gpu_preprocessing.rs:2079`).
- **Merging doesn't fix it.** Precombine halved mesh entities (46k → 25k) and the frame did not move (precombine.md "As
  built"). `--off statics` took the walk from 25.8 to 7.5 ms (2026-10-19). `NoCpuCulling` made it 81 ms: every part was
  queued in every view, and `NoIndirectDrawing` sun views never cull.
- **Scaling.** Today's design grows with instances × views. Every future feature that adds a view (more cascades,
  lamp shadows, reflection probes, GI probe updates, portals) or content (mods, denser towns) multiplies it.
- The ECS audit found the rest of our ECS use sound. Statics are the one place we fight it.

## Industry comparison

| Engine | Static placements are | Cull | Shadows |
|---|---|---|---|
| UE5 | packed per-instance data in GPU Scene (ISM/HISM, Nanite), uploaded on change | GPU compute per view, two-pass Hi-Z; non-Nanite occlusion cull opt-in | same instance cull per shadow view, per VSM mip/clipmap level |
| Unity 6 | MeshRenderers auto-converted to BatchRendererGroup instances (GPU Resident Drawer), persistent on the GPU | GPU; occlusion cull off by default | one view per cascade or shadow map; small-mesh thresholds per cascade |
| Ubisoft Montreal + RedLynx (SIGGRAPH 2015) | persistent GPU scene, 64-triangle clusters | GPU: frustum, baked per-cluster visibility mask, two-phase Hi-Z | same pipeline, per cascade |
| id Tech 7 (Doom Eternal) | bindless; surviving triangles merged into one indirect index buffer per Geometry Set | GPU triangle cull on top of CPU culling; **LOD is CPU-side** | cached atlas per light, not the same buffer |
| SPUD (Skyrim, C++) | per-cell POD arrays, persistent instance table | **CPU** per view, counting sort to MDI | same path |
| Creation Engine, OpenMW | scene-graph nodes, offline merge (precombines, object paging) | CPU | per node |

Rows verified 2026-11-09 by the architecture review §0 (sources, quotes, and the Decima row it could not source, which
was dropped). **Canonical technique:** the retained GPU scene with per-view GPU instance culling and indirect draws;
our row today is the Creation Engine row and precombine was its merge step. We take the GPU variant of SPUD's data
model because what scales with views on the CPU is the number of **phase items**, which this design makes
O(batch sets) rather than O(entities) — Haar & Aaltonen measured exactly that ("CPU: 1-2 orders of magnitude less
drawcalls… GPU: 20-40 % triangles culled… only small overall gain"). The fallback if the patch proves too large is SPUD
as-is: a CPU cull feeding `multi_draw_indirect` through a bypass phase. Nothing does this for Bevy 0.20 (survey
2026-11-08: meshlets opaque-only and Vulkan/Metal-only, `bevy_feronia`/`bevy_eidolon` foliage scatter,
`nightshade-renderer` read-for-ideas), so the base is upstream **bevyengine/bevy#25978** (§6).

## Design

### 1. What lives in the static scene

A placement is **scene-eligible** when all hold (the precombine "Members" rule, widened):

- its base class is static (no script attached, not a door, container, item, light, actor, activator or marker);
- its model is cooked (no NIF controllers that animate transforms, visibility or materials per frame; UV scroll and
  wind sway are allowed, they are shader functions of time);
- its materials are opaque or alpha-masked (blended and additive parts stay entities, sorted);
- no record or script names it (the adapter answers through a hook; the framework never names Morrowind rules).

Everything else stays an entity, as today. A mixed model (opaque body, blended glass) splits: the opaque parts go in the
scene, the blended parts spawn as an entity sharing the placement id.

**Promotion.** When gameplay needs an eligible object (a script or console targets it, physics wakes it, a mod record
edits it at runtime, the player grabs it), the instance is deactivated (one flag write) and an entity is spawned in its
place with the same placement id. Promotion is one-way for the life of the cell load; demotion happens only when the
cell unloads. Promotions are not saved: they are a consequence of gameplay state that is (record overlay, physics).

**Risk, and S3a2 scope.** `is_active` is read inside `@if(FRUSTUM_CULLING)` (`mesh_preprocess.wesl:190-194`), a def set
only for the indirect pipelines, so wherever a view preprocesses without GPU frustum culling a promoted placement draws
**twice** — entity plus stale instance — as does everything the cull rejected (architecture review S0). Moving the test
out of that block is one upstreamable line; until it lands, statics rest on the `Culling` hardware floor below.

### 2. Data model (bake output, `sf_scene`, Bevy-free)

Per streaming cell, a `StaticCellTable`, baked by a new `scene.cells` step keyed by the cell's records + mod-set hash
([bake.md](bake.md)), versioned, **read as an owned buffer and parsed in place** by `StaticCellTableView` (§3). Per
instance (80 B, 16-aligned, GPU layout): the cell-local **placement id** (so promotion and picking carry
(cell, placement)), a cell-relative 3×4 transform, the part's own **bounding sphere**, a cell-local **LOD chain**
index, `material_override` and flags (casts shadow, alpha-masked, wind). Per cell: the layer mask, an AABB, the land's
minimum height (§4) and the cell's range in the GPU table. Byte layout and the S1a-S1c history: the `sf_scene::table`
and `::chain` module docs and [archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md).

A `LodChain` is **one allocation** in Bevy's mesh allocator, its levels **concatenated, not vertex-shared**:
`sf_model::cook` ends each level with `optimize_vertex_fetch` (`sf_model/src/cook.rs:449-455`), so `chain_layout`
increments `vertex_base` per level (`sf_scene/src/chain.rs:76-95`) and a chain costs **≈ 1.7-2× its LOD0 vertex
memory** (architecture review S1). **Coordinator decision 2026-11-09: accept the concatenation; `mesh.cook` does not
change.** Per-level vertex-fetch optimisation is a real GPU win, each level keeps its own `base_vertex` in
`draw_indexed_indirect` so selecting a level is still selecting an index range, and the property the design needs — a
chain is one allocation, so **LOD never changes the instance or its batch** — holds either way. (SPUD shares vertices
because meshoptimizer's `simplify` returns indices into the original array; we traded that for fetch locality. The cost
is in the Memory risk, not in a re-cook.)

The rest of the live contract (full detail in the archive):

- **One instance per (placement, material part)**, sorted by (placement, part). A chain is one (model, part), levels
  finest first, a level without the part repeating the finer one; mesh key = hash(`mesh.cook` key, material index).
- **Chains are cell-local** since v2 (a dense index into the table's own chains), so a cell loads on its own.
  Stream-in maps each to a **global dense chain slot** keyed by mesh key (`chain_index`, `write_remapped`); the GPU
  chain table is indexed by slot. The 32 B chain record carries the level range, the material class (a pure function
  of the chain) and a 64-bit **material key** (content hash of the cooked material), so models sharing a material
  share one bindless slot (§5).
- `material_override`, `FAR_ONLY` (S7) and `OCCLUDER` (S6) are reserved and the v2 reader **rejects** a table that sets
  them (`InstanceFlags::NOT_BAKED`, `sf_scene/src/table.rs:102,427`); `INACTIVE` is runtime-only (§1), likewise
  rejected on read.
- **Per-part bounds:** an instance's sphere bounds its part's LOD0 vertices (coarser levels use a subset), so a small
  part culls as itself, not as its building. A cell with a hole or no land reports a land minimum of **negative
  infinity** and hides nothing behind it.

### 3. The GPU scene (`sf_scene_bevy`)

- One `StaticScene` per loaded space (worldspace or interior), owning a persistent instance buffer, a free-list slot
  allocator and `cell → slot range`.
- **Stream-in:** the streaming scheduler ([streaming.md](streaming.md) "Scheduler and budgets") uploads a cell's table
  into a contiguous range within the per-frame upload budget. No *placement* entity spawns; the chain mesh build below
  is work the budget must still carry. **Stream-out:** the range is released after the frames in flight are done.
- **No in-place read.** `sf_bake::Store::get` returns an owned `Vec<u8>` and decompresses (`sf_bake/src/store.rs:76`);
  there is no borrowing or mmap read, so a table is read as an **owned buffer** that `StaticCellTableView` parses in
  place (the view borrows its bytes, so the asset owns the `Vec`). `CellsStep::compress() -> false`
  (`sf_scene/src/bake.rs:617`) is the prerequisite and is already in place; a borrowing or mmap `Store` API is a
  separate `sf_bake` slice that **nobody owns yet**.
- **Mesh allocator slabs can move** (`meshes_displaced_by_slab_growth`, `bevy_render/src/mesh/allocator.rs:285-296`,
  documented as *not* the full set of changed buffers): instances store a chain **slot**, **never a raw mesh-allocator
  offset**, and offsets are resolved per frame from the allocator through a small chain table. The old
  bevyengine/bevy#26456 citation is withdrawn — closed as a duplicate 2026-11-08, diagnosis retracted; the real bug was
  #25595, fixed by #26095, which we already vendor. The rule rests on the allocator's own contract, not on that bug:
  **resolve per frame, never cache an offset. Do not "simplify" this later.**
- **The global chain table is missing, and is S3b scope**: who allocates and frees a global chain slot; who holds the
  `Handle<Mesh>` and refcounts chain mesh **residency across cells** (a chain outlives any one cell that referenced
  it); what the cull does with an instance whose chain mesh is not yet in `MeshAllocator` (today, a garbage index
  range); and who runs the per-chain **concatenate-and-rebase** that `sf_scene/src/chain.rs:13-14` assigns to "the
  runtime" — stream-in CPU work in no budget and no streaming stage (architecture review B3).
- **Fragmentation has no end state.** `sf_scene::alloc` is first-fit, never compacts, `high_water` only grows
  (`sf_scene/src/alloc.rs:1-8,86-110`), and every cell needs a *contiguous* range, so a long session fragments and the
  buffer creeps; `SlotStats::fragmentation` reports it and nothing acts. Compaction, per-cell size classes, or dropping
  contiguity for a per-cell range list the dispatch reads: an **open S3b design item** (architecture review S10).
- **Origin shifts** (`big_space`): instance transforms are cell-relative; the per-view pass adds the camera-relative
  cell offset, so an origin shift rewrites nothing.

### 4. Per-view culling and LOD (compute, every view)

One dispatch per view over the visible cells' ranges (cells pre-culled on the CPU by AABB: a few hundred tests), per
instance in this order:

1. layer mask vs the view's mask (per table since format v2, so on the CPU pre-cull); active flag (§1);
2. sphere vs frustum (cascade frustum for shadow views; cube face frustum for lamps);
3. **horizon** (exteriors, main view), **per cell, on the CPU pre-cull**, not per instance: the cell's AABB against the
   space's land-minimum grid, as SPUD `Space::land_hides` — approximate, may over-cull (it samples 9 points on the box
   top, so terrain between samples can pass a sight line while the cell is culled). The Greatness7 fork measured ~41 %
   fps in dense areas with the same idea, bundled with its merge and atlas. The grid is per space, built at stream-in
   from each loaded cell's land minimum (S1b), so it needs no extra bake;
4. **sub-pixel:** projected radius under `size_cull_px` → drop. The radius is in the view's **own** units: screen
   pixels in a perspective view (`ppr·r/√(d²−r²)`), **cascade map texels** in an orthographic one
   (`r · texels_per_metre`), since an orthographic projection has no distance term (`sf_scene/src/cull.rs`,
   `texels_per_metre` / `pick_level_ortho`). `size_cull_px` is a **per-view** knob: `lod_px` in the presets for the
   main view, **0 (off) in shadow views** until the owner's visual check, because that cutoff in shadow views was tried
   and **reverted the same day** — cascades refit with the view, so small casters lost and regained shadows as the
   camera turned (`bevyrrowind/docs/perf.md:55-57`). Unity ships per-cascade thresholds and UE a per-cascade Nanite LOD
   bias, so the knob's shape is right; only the default is conservative, and a caster size rule needs a stable cascade
   fit and its own measured slice;
5. **LOD:** the coarsest level with `e² · (pixels_per_radian / lod_px)² ≤ d²`, where `d` is the distance from the
   camera to the instance's bounding sphere surface (0 inside it, so close objects take full detail) and
   `pixels_per_radian` comes from the view's **internal** resolution (DLSS; UE needed a tunable offset under temporal
   upscaling, so `lod_px` takes one rather than being a pure preset constant). Level errors are model-space: the GPU
   pass, like the CPU twin, first multiplies `e` by the instance's largest axis scale (the longest column of its
   transform), so a placement scaled 2× takes each coarser level only at twice the distance. Orthographic views have no
   distance term: the coarsest level with `e · texels_per_metre ≤ lod_px`, one level for the whole cascade per
   **(chain, instance scale)** — the scale multiplication comes first, so two placements of one chain at different
   scales can take different levels in one cascade;
6. **occlusion** (main view, **forward path only**): Bevy's two-pass Hi-Z (`OcclusionCulling`,
   `bevy_render/src/occlusion_culling/mod.rs:73`). Bevy declares it **incompatible with deferred shading**
   (`occlusion_culling/mod.rs:47-49`) and `sf_sky_bevy/src/path.rs:372` already strips it when the deferred prepass
   goes on (`:403` restores it), so step 6 runs in no other view and on no other path; steps 1-5 and Bevy's own frustum
   cull are unaffected by the shading path (architecture review S8);
7. emit a work item: (batch set, chain, level, instance) — one geometry range per (chain, level) group, which is what
   S3a2 and S3a3 add to the port (§6).

**Quality presets change only `lod_px` and `size_cull_px`** (proposal: Ultra 0.5, High 1, Medium 2, Low 4 px), replacing
today's view-distance bias, size cutoff and ring distances for statics. No hard draw distance: streaming decides what
is resident, the pixel rule decides what is drawn and how coarse — which holds **only because §9 exists** (Nanite's
authors are explicit that the pixel rule does not stand alone: merged proxies must replace instances at extreme
distances, and aggregates such as leaves do not scale with resolution). §9 is therefore §4's precondition, not its
extension, and §4 keeps an explicit draw-distance end until S7 lands (§9).

**Swaps.** A level changes only when the difference is under a pixel, so no fade is needed; under DLSS a sub-pixel
change is resolved temporally anyway. If the owner sees pops, the fallback is a per-class dithered crossfade driven by
the same error, not per-instance ranges (the 65535 `VisibilityRange` slot cap, streaming.md "Visibility range slots",
disappears with this design). **The argument covers levels of one chain only** (same vertex set, same material); it
does **not** extend to §9's L0→L1→L2→L3 representation swaps, which change geometry *and* shading model (merged
vertex-colour or `FarTextures` proxies, octahedral impostors). Those pop, and §9 keeps its own lead bands for them.

### 5. Materials: few pipeline states

In a GPU-driven renderer the draw count is the number of **batch sets** (pipeline × bind group × mesh slab), not objects.
Our statics today carry the union of NIF materials (`CookedMaterials` dedups identical ones only).

- **Material classes** are the axes of **material state**, and only of material state: **alpha mode** (opaque or
  masked) × **wind** × **two-sidedness**. Two-sidedness is material state (`StandardMaterialKey::CULL_BACK`,
  `vendor/bevy_pbr/src/pbr_material.rs:1255-1256`) and is the third axis `sf_world_bevy::matclass` and
  `sf_scene::MaterialClass` already key on (`sf_scene/src/material.rs:53`, bit-packed `two_sided | masked<<1 |
  wind<<2`). **GI (`1<<26`) and surface weather (`1<<27`) are not class axes**: they are **view** pipeline-key bits
  (`vendor/bevy_pbr/src/render/mesh.rs:485-493`), set per view and not per material, so they multiply pipelines within
  a view and can never split a material class (port audit SERIOUS 11). Target ≤ ~16 classes for all statics; leaf
  diffuse transmission is the one split S2 could not fold (`bevyrrowind/docs/perf.md:302`) and needs its own class in
  that count.
- **Per-instance material data** lives in a bindless material table (Bevy's `MaterialBindGroupAllocators`,
  `bevy_render/src/material_bind_groups.rs:82`), indexed per instance: textures by bindless index, tint, flags. The
  upstream route is bevy#24922 (merged 2026-08-19), per-instance component data keyed by `MeshTag`, which #25978's
  materialize already writes (`materialize.wesl:28`) — no format v3 field needed.
- **Textures** stay individual (bindless) first; arrays or atlases only if measured binding limits demand it.
- Measured in S0: distinct batch sets per view today; target after S2 ≤ 2× the class count. Batch sets are **not** the
  same number as bins or draws; this spec means batch sets wherever it says so.

### 6. Drawing through Bevy (vendored patch "GS")

Built on #25978's model, rebased onto our vendored `bevy_render`/`bevy_pbr` (0.20.0 + patches, `superfargoth/vendor/README.md`):

- a batch entity per **(space, material class, vertex slab, index slab, material bind group)** carrying
  `GpuBatchedMesh3d`-like data: capacity, the template (class material, flags) and the persistent instance buffer. One
  indirect draw binds one vertex buffer, one index buffer and one material bind group
  (`vendor/bevy_render/src/render/mesh.rs:4949-4978`) and every per-range offset is slab-relative
  (`MeshInputGeometry::new`), so the slab pair is part of the key — §5's "pipeline × bind group × mesh slab", not
  (space, class) alone (gs-patch-plan C1);
- a renderer-owned pass materialises the culled work items into the tail of the preprocessing input and culling buffers
  (`MeshInputUniform`, `bevy_pbr/src/render/mesh.rs:604`; `MeshCullingData` gains `is_active`), so Bevy's prepass,
  deferred/forward, shadow and indirect paths draw them unchanged. The cull AABB is **per batch** today — the batch
  mesh's AABB is written into every instance slot (`gpu_instance_batch/mod.rs:433-437`) — so Bevy's frustum cull is
  wrong for a multi-chain batch until S3a2 makes the AABB per range (gs-patch-plan C3);
- one `multi_draw_indirect_count` per batch set where the adapter supports it (`gpu_preprocessing.rs:940`), else
  `multi_draw_indirect`; one draw per batch on DX12, where our vendored Bevy disables the count path regardless of
  adapter support (`render_phase/mod.rs:1278-1282`, wgpu#7974).

**S3a2 — the GS extension is a PREREQUISITE, not a follow-up** (both audits, independently). The port gives one mesh
and one index range **per batch entity**: a single `Handle<Mesh>` (`gpu_instance_batch/mod.rs:100`), the indirect
command's range taken from the batch's **first** work item (`mesh_preprocess.wesl:178-181`,
`build_indirect_params.wesl:133-144`), one indirect parameter per batch with `batch_set_index: None`
(`gpu_preprocessing.rs:2250-2270`). Per-instance LOD index ranges are therefore **silently ignored** — wrong geometry,
not a compile error. S3a2 is the minimum that makes S3b correct: **N geometry ranges per batch sharing one batch set**
(a range = a contiguous span of the batch's instance slots plus the index range it draws: no sorting, no per-instance
geometry field), with the AABB per range; **`unpack_ranges` in the shadow-view schedules** (`Core3dShadows` and
`sf_shadow_cache_bevy`'s, which otherwise read **uninitialized** work-item slots, port audit A4); **the counters seeing
instance batches** (`batch_set_count` reads only `self.batch_sets`, port audit A5); and the `is_active` gate (§1).
**Per-view per-instance level selection is a second slice, S3a3** — per-range survivor counts are view-dependent and
GPU-only, so it needs a GPU count → atomic bump-allocate → scatter stage with its own pipeline (gs-patch-plan C2); S4's
LOD depends on it, S3b does not. Mechanism, hunks and sizes: [gs-patch-plan.md](gs-patch-plan.md).

**Both patches are own forks.** #25978 has been dirty / `S-Merge-Conflicts` since 2026-10-17, has no milestone, has had
one cosmetic review ever, and is untested with shadows, occlusion culling, Vulkan or DX12 (architecture review
S9/S14). bevy **#26149** (`Core3dShadows`) and **#26161** (`SkipRootViewRendering`) were **closed unmerged on
2026-10-30** — self-closed, branches deleted, zero reviews between them, issue #25576 still open (review B4): the
vendor README's "Drop when" can never fire as written and no slice may plan around their merging. We offer our
additions upstream and keep them separable for it (new files under `bevy_pbr/src/gpu_instance_batch/`, separate WGSL
modules, never hunks inside `build_indirect_params.wesl`), but we do **not** plan around a merge: budget 1-2 days per
Bevy release, a full slice when the renderer moves.

**Our additions**, offered upstream: the cull (§4) as the producer; N geometry ranges per batch (S3a2); per-view level
selection (S3a3); cell-range upload/free; per-shadow-view culling and the schedule registration; motion-vector history
(§7); and the per-batch **generation counter** the `SkipRootViewRendering` hash needs (§8).

This is within CONTRIBUTING rule 4: a vendored patch written to be upstreamed, no code reaching into private
render-world state from our crates. Not chosen: a public-API bypass (`BinnedRenderPhaseType::NonMesh`,
`custom_phase_item`), which forks material specialisation, prepass and motion vectors and breaks on every Bevy upgrade;
our own renderer (SPUD-style), which loses Solari, DLSS, GI and upstream work. Port shape as built (S3a): the vendor
README's "The GPU-batched mesh port" and [archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md).

### 7. Motion vectors (DLSS)

#25978 has none. For statics, `MeshInputUniform::previous_input_index` points at the instance itself (previous transform
= current), and camera motion comes from the view's previous matrices, as for any static mesh today. Wind and UV-scroll
shaders take the previous frame's time to produce their own motion (the prepass vertex shader evaluates the sway twice).
A reviewer on #25978 already asked for a flags field to carry history bits; we use it.

### 8. Shadows and other views

Every view with a binned phase runs §4 with its frustum: 4 sun cascades, lamp cube faces, probe and reflection views.
Far shadows read the same table with the far-node levels. Shadow LOD uses the cascade's texel density (§4 step 5,
orthographic form), so far cascades take coarse levels by the same rule; lamp cube faces are perspective and use the
normal form. `size_cull_px` defaults to 0 in every shadow view (§4 step 4).

The lamp cache ([shadows.md](shadows.md) "Caching") keeps working — a cached face simply skips its dispatch — **only
once its skip hash stops being blind to the table**, which is S3a3 scope. The hash is our own code (§6: #26161 is dead
upstream) and reads `BinnedRenderPhase::entity_count`, which counts **one entity per batch**: it changes when a batch
appears or disappears and never when a batch's instance set does (stream-in, promotion, the cull), so a cached face
would keep a stale map; and `has_casters = phase_entity_count > 0`
(`vendor/bevy_pbr/src/render/light.rs:1559`) becomes permanently true on every shadow-mapped lamp face as soon as one
batch exists, disabling the cache's early-out. The fix is a per-batch **generation counter** fed into the hash, not a
bigger entity count: per-batch visible counts are GPU-side only.

### 9. Far field on the same table

HLOD proxies (L1-L3) and impostors become instances with their own error and a `far` flag; their source cells keep
their own instances. Selection is the same pixel rule with a residency rule from far-field.md "Selection, residency, and
crossfade" (a node draws only when its children are not all resident and selected — a CPU decision over GPU-selected
state, still to be designed). Impostors keep their own material class (octahedral atlas); L1's `FarTextures` extension
is its own class and slab.

**The lead bands stay until S7 actually lands.** They are today's no-hole guarantee — `VisibilityRange` on L0 chains and
node ranges — measured at **0.26 ms** of main-pass GPU in Balmora and **kept on that evidence**
(`far-field.md:74-100`). §4 removes `VisibilityRange` from statics, so **S4 keeps a per-instance draw-distance end in
the cull** (or S7 merges into S4): the bands and node ranges go when S7 replaces them, never earlier (architecture
review S5). Open: a proxy's LOD distance cannot be the bounding-sphere surface, since the eye is always inside a
km-scale node and the rule would pin it to LOD0.

### 10. Ray tracing (Solari, Ultra tier)

Solari's scene is retained in 0.20 but built from mesh entities. Two options, decided in slice S6 by measurement:
(a) the patch feeds Solari's TLAS from the static table (the right end state, larger patch); (b) Ultra keeps an
RT-only proxy set as entities near the camera (as today's `RayTracingMesh` level). Until then, Ultra is (b).

### 11. Everything else that touched static entities

- **Physics:** unchanged; colliders already live in the physics world keyed by placement id (`sf_physics_bevy/src/stream.rs:132`).
- **Picking, activation, console:** a CPU query over the loaded tables (sphere/ray), returning the placement id;
  activation promotes (§1).
- **Mods:** tables are bake output, so "everything is a mod" holds. A mod changing a placement's record rebakes its cell
  (`scene.cells` is per cell, seconds). A record patched at runtime without a rebake demotes the placement to an entity
  (always correct, never stale).
- **Saves, co-op:** no instance state is gameplay; nothing to save or replicate.
- **Interiors and portals:** each interior is a space with its own table; a portal view (M9,
  [seamless-interiors research](../research/seamless-interiors-2026-11-08.md)) is one more view running §4.
- **Debug:** `--off scene` (statics drawn as entities by the old path while it exists), per-class tint, LOD tint, a
  counter overlay (instances resident / culled per test / drawn, per view).

## Budgets (targets, to be confirmed by S0)

| What | Target |
|---|---|
| CPU per view for statics (main + render thread) | ≤ 0.05 ms, independent of instance count — reachable with S3a2's shared batch sets, not with O(batches) per view |
| Render thread, all statics, all views | ≤ 1 ms (estimate; today several ms, `prepare_preprocess_bind_groups` alone 2.3 ms) |
| GPU cull, all views, 100k resident instances | ≤ 0.3 ms (estimate) |
| Stream-in of a cell | ≤ 0.2 ms upload, zero placement entity spawns (plus the chain mesh build and any capacity growth, §3) |
| Batch sets per view | ≤ 2 × material classes, and only while a class fits one slab pair: slabs per class is a **measured** quantity in S3b, not an assumption (gs-patch-plan C1) |
| **Per-view output memory** (`MeshUniform` data buffer) | sized by **visible** instances, never by reserved capacity |

The last row is a **design requirement**, not a target. Today `batch_and_prepare` does
`data_buffer.add_multiple(input_range.len())` per view per frame (`gpu_preprocessing.rs:2585`) — the whole reserved
range, not the count that survived the cull — and at ≈ 176 B per `MeshUniform` (`render/mesh.rs:561-601`) that is
**≈ 140 MB** at this spec's own 100k instances × ~8 views (architecture review B2). The Steam Deck floor makes it
harder, not easier (one unified memory budget, no DLSS internal-resolution discount), so right-sizing is the producer's
duty: capacity is sized once per space from measured residency, never grown per cell.

## Crates

- `sf_scene` (Bevy-free): table format and reader, eligibility and promotion rules (adapter hook), LOD pick math, a CPU
  twin of the cull for tests, slot allocator. Builds cold in seconds (build-perf.md).
- `sf_scene_bevy`: streaming integration, GPU buffers, cull shaders, batch entities, debug views.
- `superfargoth/vendor` patch "GS": the #25978 base plus our additions (§6), own-fork maintained, upstreamable,
  MIT/Apache, no GPL lines.
- Adapter (`mw_world` / `mw_exterior_bevy`): the eligibility hook (scripts, record kinds) and the material class map.

## Verification

- **Exact tests** (`sf_scene`): table round trip and versioning; slot allocator; LOD pick (resolution and FOV scaling,
  monotone with distance); the **orthographic LOD pick** (no distance term, one level per (chain, instance scale)), the
  **orthographic frustum** (the cascade box) and the orthographic sub-pixel cull in map texels, all tested
  (`sf_scene/src/cull/tests.rs:386,411,434,462`); CPU cull twin vs hand-built cases (frustum, horizon, sub-pixel);
  eligibility and promotion.
- **GPU vs CPU twin:** a headless test reads back per-view work items for a fixed camera and compares them with the twin
  (counts and ids, no images).
- **Measurement** (`#[ignore = "measurement: ..."]` and the perf suite): per-view instances, work items, batch sets,
  bind groups, render-thread ms, GPU cull ms, per-view output-buffer bytes; before (S0 baseline) and after each slice,
  ×3 runs.
- **Visual checks** for the owner (`bevyrrowind/docs/visual-checks/`): LOD swaps at eye height and in flight, shadows
  per cascade, DLSS ghosting on wind-swayed foliage, horizon culling on ridges, promotion of a grabbed object. Agents do
  not judge images (CONTRIBUTING "Tests and visual checks").

## Slices

S0-S3a are built; what each delivered and what is still open for the owner is in
[archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md) (S3a also in
`bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md`): **S0** per-view counters `[~]`, **S1a** `sf_scene` core `[x]`,
**S1b** `scene.cells` bake + adapter hooks `[~]`, **S1c** format v2 `[x]`, **S2** material classes on the entity path
`[~]`, **S3a** the #25978 port `[~]`.

| # | Slice | Done when |
|---|---|---|
| S3a2 | Patch "GS" extension ([gs-patch-plan.md](gs-patch-plan.md)): N geometry ranges per batch sharing one batch set, AABB per range; `unpack_ranges` in the shadow-view schedules; the counters see instance batches; `is_active` out of `@if(FRUSTUM_CULLING)` (§1) | ranges draw the geometry they name; lamp shadow views unpack their own ranges; `batch_set_count` counts statics |
| S3a3 | Per-view per-instance level selection (GPU count → bump-allocate → scatter) and the per-batch generation counter the lamp skip hash reads (§8); may fold into S4 | a view's instances draw their own levels; no stale cached lamp face |
| S3b | Scrib runtime on the port: batch entities fed from the baked tables, the **global chain table** (slot allocation, `Handle<Mesh>` lifetime, cross-cell residency and refcounting, the per-chain concatenate-and-rebase) and a fragmentation policy (§3), main view only, opaque class, no LOD | one town's opaque statics drawn from the table; slabs per class and batch sets per view measured |
| S4 | All classes, LOD ranges, pixel-error presets, motion vectors; the far-field lead bands **stay** (an explicit per-instance draw-distance end in the cull) | statics fully on the table in the main view, no holes at far-field handoffs |
| S5 | Shadow views (cascades, lamp faces) + far shadows | shadows from the table; per-view cost flat |
| S6 | Horizon + Hi-Z occlusion (forward path, §4 step 6); Solari decision (§10) | numbers; decision logged |
| S7 | Far field on the table (§9) | lead bands and node ranges removed, and only here |
| S8 | Promotion and runtime demotion paths; picking | grab, script and mod cases tested |
| S9 | Retire per-entity statics and precombine; upstream PR | old path deleted, `--off scene` gone, PR open |

S2 pays on its own and de-risks S3. Each slice reports ms per preset, and each needs a stated abort condition: halving
mesh entities did not move the frame once already (§Why), so "no change" is a possible S3b result.

## Risks and open questions

- **#25978 will not merge on our schedule, and #26149/#26161 never will.** We own patch GS and the shadow patch as
  forks (§6); rebasing them on each Bevy release is the cost. Mitigation: keep our additions in new files, offer them
  upstream early, follow the Rendering working group.
- **Statics require `GpuPreprocessingMode::Culling`** — a stated **hardware floor**. An adapter limited to
  `PreprocessingOnly` drops instance batches silently (`gpu_preprocessing.rs:2574-2576`; `push_range` returns `None` in
  `Direct` mode), and §1's `is_active` hole has the same root; there the design falls back to the entity path
  automatically with one startup `warn!`. The Steam Deck (RDNA2, Vulkan) supports culling, so the floor does not touch
  the owner's performance floor. The failure is adapter-wide or a deliberate camera setting, never per view: our
  inherited shadow views take the camera's flag (`sf_sky_bevy/src/sun_view.rs:227-231,255-257`,
  `sf_shadow_cache_bevy/src/render.rs:518,546-549`), which corrects the port audit's SERIOUS 7 framing.
- **Two supported draw paths, not a driver risk:** `multi_draw_indirect_count` per batch set on Vulkan (4060 Laptop,
  Steam Deck), one draw per batch on DX12 where our vendored Bevy disables the count path (§6). Both are measured.
- **Motion vectors** for wind-swayed foliage need the prepass to evaluate two times; verify DLSS ghosting (owner).
- **Bindless limits** on the 4060 Laptop with Morrowind + mod texture counts; atlases only if measured.
- **Memory:** a full worldspace table is small (≈ 80 B × instances; 100k ≈ 8 MB), but a chain's **vertex** memory is
  ≈ 1.7-2× its LOD0 (levels concatenated, not shared, §2) and the per-view output buffer dwarfs both (Budgets); check
  heavy mod lists.
- **Eligibility errors** (a scripted static treated as static) show as "can't interact": promotion on any record or
  script reference must be conservative.
- **Open, for the owner (decide before S9): `NO_LOD` placements.** The bake bars them from the scene entirely —
  `BARRED` copies precombine's bar set (`sf_scene/src/bake.rs:69-70`) — so they stay entities for ever and pay exactly
  the per-entity cost this system exists to remove. But `NO_LOD` means "keep full detail at every distance", which the
  static scene could honour by **pinning level 0** instead. Flagged, not decided.
- Open: per-instance tint and vertex colour variation within a class (the `MeshTag` route, §5, avoids a format v3
  field); animated-but-static NIF controllers (which stay entities, and how many in a town); whether interiors (few
  hundred objects, hundreds loaded over a session, where fragmentation and chain churn show first) are worth the table
  from S3 or later; warm-up for the compute pipelines this design adds (cull, materialize, per-view variants) and never
  precompiles.
