# Static scene: a retained, GPU-driven static world (`sf_scene`, `sf_scene_bevy`)

Status: **building** (owner go 2026-11-08: "a future proof engine that can work flawlessly and fast"; no quick fixes).
Built: **S1a**, **S1c** `[x]`; **S0**, **S1b**, **S2**, **S3a** `[~]`, each with an owner run open (Slices).
**S3a2 is next, then S3b**; S4-S9 not started.

Corrected 2026-11-09 against three documents that hold the evidence for every citation below: the
[S3a port audit](../research/s3a-port-audit-2026-11-09.md), the
[architecture review](../research/static-scene-architecture-review-2026-11-09.md) and
[gs-patch-plan.md](gs-patch-plan.md). Both audits' verdict: the architecture is right, the spec was wrong about what the
ported upstream PR can express.

**Doc budget: an accepted exception, revisit at S9.** The house budget is 25 KB; this spec is over it and the overage is
normative — the contract for a nine-slice system carrying a vendored renderer fork, with §4's cull and LOD math the one
rule a CPU twin and a GPU pass must both be held to. Mechanism lives where it has an authoritative home instead (§2 →
the `sf_scene::table`/`::chain` module docs, §3 → [scene-runtime-plan.md](scene-runtime-plan.md), §6 →
[gs-patch-plan.md](gs-patch-plan.md), history → [archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md)).

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

**Canonical technique:** the retained GPU scene with per-view GPU instance culling and indirect draws. UE5 (ISM/HISM,
Nanite), Unity 6's GPU Resident Drawer, Ubisoft Montreal + RedLynx (SIGGRAPH 2015), id Tech 7 and SPUD (Skyrim) all
keep static placements as packed per-instance data uploaded on change and cull per view — on the GPU everywhere but
SPUD, whose cull is CPU. Creation Engine and OpenMW keep scene-graph nodes with an offline merge: **that is our row
today, and precombine was its merge step.** The sourced table, with the one row it could not source, is the
architecture review §0 (which corrected four of our rows on 2026-11-09).

We take the GPU variant of SPUD's data model because what scales with views on the CPU is the number of **phase
items**, which this design makes O(batch sets) rather than O(entities) — Haar & Aaltonen measured exactly that ("CPU:
1-2 orders of magnitude less drawcalls… GPU: 20-40 % triangles culled… only small overall gain"). The fallback if the
patch proves too large is SPUD as-is: a CPU cull feeding `multi_draw_indirect` through a bypass phase. Nothing does
this for Bevy 0.20 (survey 2026-11-08), so the base is upstream **bevyengine/bevy#25978** (§6).

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
only for the indirect pipelines, so a view that preprocesses without GPU frustum culling never tests it. **What such a
view shows is under investigation 2026-11-09**: the architecture review (S0) reads it as a promoted placement drawn
**twice**, entity plus stale instance; the port audit (A9) holds the path unreachable today because those views are
`no_indirect_drawing` and **drop instance batches entirely**, so the statics go **missing** instead. Either way the two
guards are independent and should not be: moving the test out of that block is one upstreamable line, and until it
lands statics rest on the `Culling` hardware floor below.

### 2. Data model (bake output, `sf_scene`, Bevy-free)

Per streaming cell, a `StaticCellTable`, baked by a new `scene.cells` step keyed by the cell's records + mod-set hash
([bake.md](bake.md)), versioned, **read as an owned buffer and parsed in place** by `StaticCellTableView` (§3).
**Every field, the byte layout and the S1a-S1c history are in the `sf_scene::table` and `sf_scene::chain` module
docs** — authoritative, and where an implementer looks — and in
[archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md). The decisions they implement:

- **One instance per (placement, material part)**, sorted by (placement, part), and the 80 B record *is* the GPU
  layout: stream-in uploads it rather than re-packing it. It names its placement by a **cell-local id** (so promotion
  and picking carry (cell, placement)) and its transform is **cell-relative**, so an origin shift rewrites nothing (§3).
- **Chains are cell-local** since format v2 — a dense index into the table's own chains — so **a cell loads on its
  own**; a global chain id collided silently across cells and could not index a GPU table. Stream-in maps each to a
  global dense **chain slot** keyed by mesh key. A chain record carries the material class (a pure function of the
  chain, so it moved off the instance) and a 64-bit **material key**, a content hash of the cooked material, so models
  sharing a material share one bindless slot (§5). A chain is one (model, part), levels finest first, a level without
  the part repeating the finer one.
- **Format v3 and a new `scene.chains` bake step, approved 2026-11-09.** A chain's `mesh_key` is a blake3 hash and so
  not invertible: a v2 table cannot tell the runtime which model a chain came from, and there is no path from a baked
  table to a mesh without a format change. v3 spends reserved bytes on a model index and material index per chain plus
  a model-subject section, and `scene.chains` bakes each chain's geometry so the runtime loads a handle instead of
  concatenating at stream-in. Safe to take now and **never again**: nothing has ever been baked — `bake_scene` is still
  not called by `bevyrrowind bake`. Format, bake step and costs: scene-runtime-plan.md §3.
- **Reserved means rejected on read**: `material_override` ≠ 0, `FAR_ONLY` (S7) and `OCCLUDER` (S6) make a baked table
  an error (`InstanceFlags::NOT_BAKED`, `sf_scene/src/table.rs:102,427`); `INACTIVE` is runtime-only (§1), likewise.
- **Per-part bounds**: an instance's sphere bounds its *part's* LOD0 vertices, so a small part culls as itself, not as
  its building. A cell with a hole or no land reports a land minimum of **negative infinity** and hides nothing
  behind it.

A `LodChain` is **one allocation** in Bevy's mesh allocator, its levels **concatenated, not vertex-shared**:
`sf_model::cook` ends each level with `optimize_vertex_fetch` (`sf_model/src/cook.rs:449-455`), so `chain_layout`
increments `vertex_base` per level (`sf_scene/src/chain.rs:76-95`) and a chain costs **≈ 1.7-2× its LOD0 vertex
memory** (architecture review S1). **Coordinator decision 2026-11-09: accept the concatenation; `mesh.cook` does not
change.** Per-level vertex-fetch optimisation is a real GPU win, each level keeps its own `base_vertex` in
`draw_indexed_indirect` so selecting a level is still selecting an index range, and the property the design needs — a
chain is one allocation, so **LOD never changes the instance or its batch** — holds either way. (SPUD shares vertices
because meshoptimizer's `simplify` returns indices into the original array; we traded that for fetch locality. The cost
is in the Memory risk, not a re-cook.)

### 3. The GPU scene (`sf_scene_bevy`)

The CPU half's work order is [scene-runtime-plan.md](scene-runtime-plan.md); the decisions:

- One `StaticScene` per loaded space (worldspace or interior), owning a persistent instance buffer, a slot allocator
  and `cell → slot range`.
- **Stream-in:** the streaming scheduler ([streaming.md](streaming.md) "Scheduler and budgets") uploads a cell's table
  into a contiguous range within the per-frame upload budget. No *placement* entity spawns; the chain geometry it needs
  is work the budget must still carry. **Stream-out:** the range is released after the frames in flight are done.
- **No in-place read:** `sf_bake::Store::get` returns an owned, decompressed `Vec<u8>` (`sf_bake/src/store.rs:76`), so a
  table is read as an **owned buffer** that `StaticCellTableView` parses in place. `CellsStep::compress() -> false`
  (`sf_scene/src/bake.rs:617`) is the prerequisite and is in place; a borrowing or mmap `Store` API would be its own
  `sf_bake` slice and **nobody owns one**.
- **Mesh allocator slabs can move** (`meshes_displaced_by_slab_growth`, `bevy_render/src/mesh/allocator.rs:285-296`,
  documented as *not* the full set of changed buffers): instances store a chain **slot**, **never a raw mesh-allocator
  offset**, and offsets are resolved per frame from the allocator through a small chain table. The rule rests on the
  allocator's own contract, not on any one bug (the withdrawn #26456 citation: archive): **resolve per frame, never
  cache an offset. Do not "simplify" this later.**
- **A global chain table is S3b scope** and the spec does not yet name its owner: who allocates and frees a global
  chain slot, who holds the `Handle<Mesh>` and refcounts chain mesh **residency across cells** (a chain outlives any
  one cell that referenced it), and what the cull does with an instance whose chain mesh is not yet in `MeshAllocator`
  (today, a garbage index range) (architecture review B3). The per-chain **concatenate-and-rebase** that
  `sf_scene/src/chain.rs:13-14` assigns to "the runtime" is **no longer runtime work**: it moves into the `scene.chains`
  bake step below, because at stream-in it has no budget and no streaming stage.
- **Instance-slot fragmentation is bounded by construction, not by compaction.** `sf_scene::alloc` is first-fit, never
  compacts and `high_water` only grows (`sf_scene/src/alloc.rs:1-8,86-110`), which creeps over a long session
  (architecture review S10). **Decision 2026-11-09: a segregated size-class allocator per batch** (blocks never split
  or merged, so the high-water mark converges on the ring configuration) and **batch capacity fixed at config, never
  grown at stream-in** — growing it reallocates the output buffer and rebuilds a bind group, which "stream-in ≤ 0.2 ms,
  zero spawns" cannot pay. Overflow leaves those placements out of the table, to spawn as entities. Bound and proof:
  scene-runtime-plan.md §4.4.
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
  a view and can never split a material class (port audit B2). Target ≤ ~16 classes for all statics; leaf
  diffuse transmission is the one split S2 could not fold (`bevyrrowind/docs/perf.md:302`) and needs its own class in
  that count.
- **Per-instance material data** lives in a bindless material table (Bevy's `MaterialBindGroupAllocators`,
  `bevy_render/src/material_bind_groups.rs:82`), indexed per instance: textures by bindless index, tint, flags. The
  upstream route is bevy#24922 (merged 2026-08-19), per-instance component data keyed by `MeshTag`, which #25978's
  materialize already writes — so it costs the table format nothing.
- **Textures** stay individual (bindless) first; arrays or atlases only if measured binding limits demand it.
- Measured in S0: distinct batch sets per view today; target after S2 ≤ 2× the class count. Batch sets are **not** the
  same number as bins or draws; this spec means batch sets wherever it says so.

### 6. Drawing through Bevy (vendored patch "GS")

Built on #25978's model, rebased onto our vendored `bevy_render`/`bevy_pbr` (0.20.0 + patches,
`superfargoth/vendor/README.md`). **This section is the decisions; the mechanism — hunks, shaders, sizes and every
`file:line` — is [gs-patch-plan.md](gs-patch-plan.md)**, its work order. Spec: what and why. Patch plan: how.

- **A batch entity per (space, material class, vertex slab, index slab, material bind group)**, because one indirect
  draw binds one vertex buffer, one index buffer and one material bind group and every per-range offset is
  slab-relative: §5's "pipeline × bind group × mesh slab", not (space, class) alone (gs-patch-plan C1). Slabs per class
  is a **measured** quantity in S3b, never an assumption.
- **We feed Bevy's own input buffers and never bypass its phases.** A renderer-owned pass materialises the culled work
  items into the tail of the preprocessing input and culling buffers, so prepass, deferred/forward, shadow and indirect
  paths draw them unchanged.
- **The cull AABB is per range**, not per batch as the port leaves it, or Bevy's own frustum cull is wrong for a
  multi-chain batch (gs-patch-plan C3). S3a2.
- **Two draw paths, both measured:** `multi_draw_indirect_count` per batch set where the adapter supports it; one draw
  per batch on DX12, where our vendored Bevy disables the count path whatever the adapter reports (wgpu#7974).

**S3a2 is a PREREQUISITE, not a follow-up** (both audits, independently). The port expresses one mesh and one index
range **per batch entity**, so per-instance LOD ranges are **silently ignored** — wrong geometry, not a compile error
(port audit BLOCKER 1). Its four items are the S3a2 row of Slices; the one that makes S3b *correct* is **N geometry
ranges per batch sharing one batch set** — a range = a contiguous span of the batch's instance slots plus the index
range it draws, so no sorting and no per-instance geometry field. **Per-view level selection is a second slice, S3a3**:
its per-range survivor counts are view-dependent and GPU-only, so it needs its own GPU count → atomic bump-allocate →
scatter pass (gs-patch-plan C2). S4's LOD depends on it; S3b does not. **Rejected 2026-11-09: per-(cell, chain) LOD
quantisation**, which would make every per-range count CPU-known and delete that pass — it breaks §4 step 5's
per-instance pixel rule (gs-patch-plan O6; owner may overrule).

**Both patches are own forks: we offer our additions upstream and plan around no merge.** #25978 is dirty, unmilestoned,
reviewed once, and untested with shadows, occlusion culling, Vulkan or DX12; bevy **#26149** and **#26161** were
**closed unmerged on 2026-10-30** with zero reviews (dates, PR states and review counts: architecture review S9 and B4),
so the vendor README's "Drop when" can never fire as written and **no slice may plan around their merging**. Our
additions stay separable for the offer — new files under `bevy_pbr/src/gpu_instance_batch/`, separate WGSL modules,
never hunks inside `build_indirect_params.wesl` — and cost 1-2 days per Bevy release, a full slice when the renderer
moves. **Offered upstream** (as what: gs-patch-plan §7): the cull (§4) as the producer; N geometry ranges per batch;
per-view level selection; cell-range upload/free; per-shadow-view culling and its schedule registration; motion-vector
history (§7); the per-batch generation counter the `SkipRootViewRendering` hash needs (§8).

Within CONTRIBUTING rule 4: upstreamable, with no crate of ours reaching into private render-world state. Not chosen: a
public-API bypass (`BinnedRenderPhaseType::NonMesh`), which forks material specialisation, prepass and motion vectors
and breaks on every Bevy upgrade; our own renderer (SPUD-style), which loses Solari, DLSS, GI and upstream work. Port
shape as built (S3a): `vendor/README.md` "The GPU-batched mesh port" and
[archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md).
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
upstream) and reads `BinnedRenderPhase::entity_count`, which counts **one entity per batch**: it moves when a batch
appears or disappears and never when a batch's instance set does (stream-in, promotion, the cull), so a cached face
would keep a stale map; and `has_casters = phase_entity_count > 0` (`vendor/bevy_pbr/src/render/light.rs:1559`) becomes
permanently true on every shadow-mapped lamp face as soon as one batch exists, disabling the early-out (port audit A6).
The fix is a per-batch **generation counter** fed into the hash, not a bigger entity count: per-batch visible counts
are GPU-side only. **Measure the lamp cache before and after S3b with that in mind.**

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
  [seamless-interiors research](../research/seamless-interiors-2026-11-08.md)) is one more view running §4. Whether
  interiors get a table **at all for now is an owner question, not a settled decision** (Risks).
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

The last row is a **design requirement**, not a target: today Bevy sizes a view's output by the whole *reserved* range,
not by what survived the cull, which at ≈ 176 B per `MeshUniform` is **≈ 140 MB** at this spec's own 100k instances ×
~8 views (architecture review B2). The Steam Deck floor makes that harder, not easier (one unified memory budget, no
DLSS internal-resolution discount), so right-sizing is the **producer's** duty: capacity is fixed per batch from
measured residency and never grown per cell (§3).

## Crates

- `sf_scene` (Bevy-free): table format and reader, eligibility and promotion rules (adapter hook), LOD pick math, a CPU
  twin of the cull for tests, slot allocator. Builds cold in seconds (build-perf.md).
- `sf_scene_bevy`: streaming integration, GPU buffers, cull shaders, batch entities, debug views.
- `superfargoth/vendor` patch "GS": the #25978 base plus our additions (§6), own-fork maintained, upstreamable,
  MIT/Apache, no GPL lines.
- Adapter (`mw_world` / `mw_exterior_bevy`): the eligibility hook (scripts, record kinds) and the material class map.

## Verification

- **Exact tests** (`sf_scene`, the authoritative list being its own test modules): table round trip and versioning;
  slot allocator; the perspective LOD pick (resolution and FOV scaling, monotone with distance); the **orthographic**
  LOD pick, frustum and sub-pixel cull, all three already tested (`sf_scene/src/cull/tests.rs:386,411,434,462`); CPU
  cull twin vs hand-built cases (frustum, horizon, sub-pixel); eligibility and promotion.
- **GPU vs CPU twin:** a headless test reads back per-view work items for a fixed camera and compares them with the twin
  (counts and ids, no images).
- **Measurement** (`#[ignore = "measurement: ..."]` and the perf suite): per-view instances, work items, batch sets,
  bind groups, render-thread ms, GPU cull ms, per-view output-buffer bytes; before (S0 baseline) and after each slice,
  ×3 runs.
- **Visual checks** for the owner (`bevyrrowind/docs/visual-checks/`): LOD swaps at eye height and in flight, shadows
  per cascade, DLSS ghosting on wind-swayed foliage, horizon culling on ridges, promotion of a grabbed object. Agents do
  not judge images (CONTRIBUTING "Tests and visual checks").

## Slices

S0-S3a are built — **S0** `[~]`, **S1a** `[x]`, **S1b** `[~]`, **S1c** `[x]`, **S2** `[~]`, **S3a** `[~]` — and what
each delivered, with the owner runs still open, is in
[archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md) and
`bevyrrowind/docs/test-pass-handoffs/` (`s0-instrument.md`, `s2-material-classes.md`, `s3a-gpu-batch.md`).

| # | Slice | Done when |
|---|---|---|
| S3a2 | Patch "GS" extension, split into two agent slices ([gs-patch-plan.md](gs-patch-plan.md) §3): N geometry ranges per batch sharing one batch set with the AABB per range; range unpacking inside the early preprocess pass; counters that see instance batches; guards; `is_active` out of `@if(FRUSTUM_CULLING)` (§1) | ranges draw the geometry they name; lamp shadow views unpack their own ranges; `batch_set_count` counts statics |
| S3a3 | Per-view per-instance level selection (gs-patch-plan §4) and the per-batch generation counter the lamp skip hash reads (§8); may fold into S4 | a view's instances draw their own levels; no stale cached lamp face |
| S3b | Scrib runtime on the port ([scene-runtime-plan.md](scene-runtime-plan.md)): table format v3 + the `scene.chains` bake step, batch entities fed from the baked tables, the global chain table and the size-class allocator (§3); main view only, opaque class, no LOD | one town's opaque statics drawn from the table; slabs per class and batch sets per view measured |
| S4 | All classes, LOD ranges, pixel-error presets, motion vectors; the far-field lead bands **stay** (an explicit per-instance draw-distance end in the cull) | statics fully on the table in the main view, no holes at far-field handoffs |
| S5 | Shadow views (cascades, lamp faces) + far shadows | shadows from the table; per-view cost flat |
| S6 | Horizon + Hi-Z occlusion (forward path, §4 step 6); Solari decision (§10) | numbers; decision logged |
| S7 | Far field on the table (§9) | lead bands and node ranges removed, and only here |
| S8 | Promotion and runtime demotion paths; picking | grab, script and mod cases tested |
| S9 | Retire per-entity statics and precombine; upstream PR | old path deleted, `--off scene` gone, PR open |

S2 pays on its own and de-risks S3. Each slice reports ms per preset, and each needs a stated abort condition: halving
mesh entities did not move the frame once already (§Why), so "no change" is a possible S3b result.

## Risks and open questions

- **#25978 will not merge on our schedule, and #26149/#26161 never will.** Patch GS and the shadow patch are own forks
  (§6); rebasing them on each Bevy release is the cost.
- **Statics require `GpuPreprocessingMode::Culling`** — a stated **hardware floor**. An adapter limited to
  `PreprocessingOnly` drops instance batches silently (`gpu_preprocessing.rs:2574-2576`; `push_range` returns `None` in
  `Direct` mode), and §1's `is_active` hole has the same root; there the design falls back to the entity path
  automatically with one startup `warn!`. The Steam Deck (RDNA2, Vulkan) supports culling, so the floor does not touch
  the owner's performance floor. The failure is adapter-wide or a deliberate camera setting, never per view: our
  inherited shadow views take the camera's flag, clamped by `GpuPreprocessingSupport`
  (`sf_sky_bevy/src/sun_view.rs:230-231,254-255`, `sf_shadow_cache_bevy/src/render.rs:518,546-549`), which corrects the
  port audit's A9 framing. (`bevyrrowind/docs/perf.md:53-54` still names our extra sun views as `NoIndirectDrawing`;
  that line predates the inheritance above and is stale.)
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
- **Open, for the owner: are interiors in the static scene at all?** (§11.) The case for leaving them out for now: a
  few hundred statics each against the 46-55k this design exists to remove; a door crossing cycles a whole worldspace
  in one frame, the worst case for chain churn and for §3's size-class bound; promotion is the common case indoors, so
  much of an interior stays entities anyway; and the bake grows a product per cell across hundreds of worldspaces
  (scene-runtime-plan.md §12 recommends off by default). **Owner decision.**
- Open: per-instance tint and vertex colour variation within a class (the `MeshTag` route, §5); animated-but-static NIF
  controllers (which stay entities, and how many in a town); warm-up for the compute pipelines this design adds (cull,
  materialize, per-view variants) and never precompiles.
