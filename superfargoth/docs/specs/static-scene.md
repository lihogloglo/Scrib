# Static scene: a retained, GPU-driven static world (`sf_scene`, `sf_scene_bevy`)

Status: **building** (owner go 2026-11-08: "a future proof engine that can work flawlessly and fast"; no quick fixes).
Built and committed: **S1a**, **S1c** `[x]`; **S0** `[~]` (counters in, baseline run open), **S1b** `[~]`
(bake step in, the Vvardenfell measurement and the `bake_scene` call in `mw_boot_bevy::bake` open), **S2** `[~]`
(material-class fold in, perf A/B and visual check SS-matclass open), **S3a** `[~]` (#25978 ported and tested, nothing
draws through it yet). **S3b is next**; S4-S9 not started. As-built detail: "Slices" below.

Replaces, once complete, per-entity statics (`sf_world_bevy` `spawn_template_ranges`) and the near-cell
precombine ([precombine.md](precombine.md)); absorbs far-field selection ([far-field.md](far-field.md)). Hard problem H1
(`bevyrrowind/docs/hard-problems.md`). Research: [spud-code-2026-11-08](../research/spud-code-2026-11-08.md); the
three-agent study of 2026-11-08 (industry, Bevy fit, crates, ECS audit) is summarised in "Why" and "Industry comparison".

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
- The ECS audit found the rest of our ECS use sound (change-gated writes, guarded markers, `par_iter`, off-ECS physics
  and placement lists). Statics are the one place we fight it.

## Industry comparison

| Engine | Static placements are | Cull | Shadows |
|---|---|---|---|
| UE5 | packed per-instance data in GPU Scene (ISM/HISM, Nanite), uploaded on change | GPU compute per view, two-pass Hi-Z | same instance cull per shadow view (VSM) |
| Unity 6 | MeshRenderers auto-converted to BatchRendererGroup instances (GPU Resident Drawer) | GPU, occlusion | (not documented on the page read) |
| Frostbite / Ubisoft (Haar & Aaltonen, SIGGRAPH 2015) | persistent GPU scene, clusters | GPU: frustum, cone, Hi-Z | same pipeline |
| Decima, id Tech 6/7 | packed instance data, bindless, multi-draw-indirect | GPU | same |
| SPUD (Skyrim, C++) | per-cell POD arrays, persistent instance table | **CPU** per view, counting sort to MDI | same path |
| Creation Engine, OpenMW | scene-graph nodes, offline merge (precombines, object paging) | CPU | per node |

Citations: UE ISM docs `dev.epicgames.com/documentation/en-us/unreal-engine/instanced-static-mesh-component-in-unreal-engine`;
Unity `docs.unity.com/.../gpu-resident-drawer`; Haar & Aaltonen
`advances.realtimerendering.com/s2015/aaltonenhaar_siggraph2015_combined_final_footer_220dpi.pdf`. UE5 GPU Scene,
Decima, id Tech rows are from memory (verify before quoting further).

**Canonical technique:** the retained GPU scene with per-view GPU instance culling and indirect draws. Our row today is
the Creation Engine row; precombine was its merge step. SPUD shows the same data model with a CPU cull; we take the
GPU variant because it scales with views and instance counts and matches where Bevy is heading (below).

**Crates and engines checked (2026-11-08):** nothing does this for Bevy 0.20. Bevy meshlets are opaque-only (no alpha
mask: Morrowind foliage, fences), Vulkan/Metal-only and need per-mesh preprocessing. `bevy_feronia`/`bevy_eidolon` are
foliage scatter; `nightshade-renderer` (wgpu, MIT/Apache) is read-for-ideas for its cull/MDI-count/shadow-cull passes.
Upstream: **bevyengine/bevy#25978** `GpuBatchedMesh3d` (open, needs review, merge conflicts, no milestone; no motion
vectors, transmission or per-instance sorting) is the right shape and the base of this design.

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

### 2. Data model (bake output, `sf_scene`, Bevy-free)

Per streaming cell, a `StaticCellTable`, baked by a new `scene.cells` step keyed by the cell's records + mod-set hash
([bake.md](bake.md)), versioned, mmap-read:

| Field | Type | Note |
|---|---|---|
| placement id | u32 | cell-local: (cell, placement) links to records, physics, promotion |
| transform | 3×4 f32, cell-relative | the cell origin is a `big_space` grid cell; GPU positions are camera-relative |
| bounding sphere | 4 f32, cell-relative | of the instance's own part; for every cull test |
| LOD chain | u32 cell-local index → `LodChain` | one mesh per chain: shared vertices, index range per level, error per level in model-space metres; the chain carries mesh key, material key and material class (§5) |
| material override | u32 | 0 = the chain's material; reserved for per-placement retextures and tint |
| flags | u32 | casts shadow, alpha-masked, wind; reserved: far-only (S7), occluder (S6) |

The layer mask (replaces `RenderLayers`, lamp vs worldspace layers) is one per table.

A `LodChain` is one allocation in Bevy's mesh allocator (meshoptimizer levels over the same vertices, as `sf_lod` already
cooks them; SPUD and the Greatness7 MGE XE fork use the same layout). Selecting a level is choosing an index range, so
**LOD never changes the instance or its batch**.

Per cell also: an AABB, the land's minimum height (for the horizon test, §4) and the cell's table range in the GPU table.

Settled in S1a (2026-11-08):
- **Active flag.** `INACTIVE` is a runtime-only flag bit (promotion, §1). A baked table that sets it is rejected.
- **Terrain holes.** A cell with a hole or no land reports a land minimum of negative infinity: it never hides
  anything behind it.

Settled in S1b (2026-11-08; `sf_scene::bake`, `sf_scene::chain` docs):
- **One instance per (placement, material part)**, sorted by (placement, part); siblings share the placement id.
  A chain is one (model, part): its levels concatenated finest first, a level without the part repeats the finer one
  (`chain_layout`, also used by the runtime). Mesh key = hash(`mesh.cook` key, material index).
- **Placement id** = the placement's index in its world-index cell (the 128-bit `sf_world` id does not fit a u32), so
  cell-local: promotion and picking carry (cell, placement).
- **Land minimum** = level-0 terrain block minima over the cell; no terrain tile (or no grid) → negative infinity.

Format v2 (S1c, 2026-11-09; `sf_scene::table` docs; supersedes S1a's global chain id and S1b's xor-folded id, which
collided silently across cells and could not index a GPU table):
- **Chains are cell-local.** An instance's chain field is a dense index into its table's chains (sorted by mesh key,
  each used). Each table carries copies of its chains, so a cell loads on its own. **Stream-in** (S3) maps each
  cell-local index to a global dense chain slot keyed by mesh key (`chain_index`, `write_remapped` copies the instance
  section with the slots); the GPU chain table is indexed by slot.
- **Chain record** (32 B): level range, material class (moved from the instance: a pure function of the chain), mesh
  key, and a 64-bit **material key**, a content hash of the cooked material (`sf_bake::Digest`; the name is not
  hashed), so models sharing a material share one bindless slot (§5).
- **Instance record** (80 B, 16-aligned): transform, the part's own sphere, placement, chain index, `material_override`
  (0 = the chain's material; other values reserved for a later per-placement material table, rejected by the v2
  reader), flags (u32; `FAR_ONLY` and `OCCLUDER` reserved for S7/S6, not set yet). **Layers** move to the 80 B header.
- **Per-part bounds.** Each instance's sphere bounds its part's LOD0 vertices (coarser levels use a subset of them),
  so a small part culls as itself, not as its building.

### 3. The GPU scene (`sf_scene_bevy`)

- One `StaticScene` per loaded space (worldspace or interior), owning a persistent instance buffer, a free-list slot
  allocator and `cell → slot range`.
- **Stream-in:** the streaming scheduler ([streaming.md](streaming.md) "Scheduler and budgets") uploads a cell's table
  into a contiguous range within the per-frame upload budget. No entity spawns. **Stream-out:** the range is released
  after the frames in flight are done.
- **Mesh allocator slabs can move** (`MeshAllocator::meshes_displaced_by_slab_growth`, `bevy_render/src/mesh/allocator.rs:298`;
  upstream bug bevyengine/bevy#26456): instances store a chain slot (remapped from the table's cell-local index at
  stream-in), never a raw offset; offsets are resolved per frame from the allocator through a small chain table.
- **Origin shifts** (`big_space`): instance transforms are cell-relative; the per-view pass adds the camera-relative
  cell offset, so an origin shift rewrites nothing.

### 4. Per-view culling and LOD (compute, every view)

One dispatch per view over the visible cells' ranges (cells pre-culled on the CPU by AABB: a few hundred tests), per
instance in this order:

1. layer mask vs the view's mask (per table since format v2, so on the CPU pre-cull); active flag;
2. sphere vs frustum (cascade frustum for shadow views; cube face frustum for lamps);
3. **horizon** (exteriors, main view), **per cell, on the CPU pre-cull**, not per instance: the cell's AABB against the
   space's land-minimum grid, as SPUD `Space::land_hides` (conservative; the Greatness7 fork measured ~41 % fps in
   dense areas with the same idea). The grid is per space, built at stream-in from each loaded cell's land minimum
   (S1b), so it needs no extra bake;
4. **sub-pixel:** projected radius under `size_cull_px` → drop. `size_cull_px` equals `lod_px` in the presets for now;
   it becomes its own knob only if measurement asks for it;
5. **LOD:** the coarsest level with `e² · (pixels_per_radian / lod_px)² ≤ d²`, where `d` is the distance from the
   camera to the instance's bounding sphere surface (0 inside it, so close objects take full detail) and
   `pixels_per_radian` comes from the view's **internal** resolution (DLSS). Level errors are model-space: the GPU
   pass, like the CPU twin, first multiplies `e` by the instance's largest axis scale (the longest column of its
   transform), so a placement scaled 2× takes each coarser level only at twice the distance. Orthographic views (sun cascades) have
   no distance term: the coarsest level with `e · texels_per_metre ≤ lod_px`, one level for the whole cascade per
   chain;
6. **occlusion** (main view): Bevy's two-pass Hi-Z (`OcclusionCulling`, `bevy_render/src/occlusion_culling/mod.rs:73`);
7. emit a work item: (batch set, chain, level, instance).

**Quality presets change only `lod_px` and `size_cull_px`** (proposal: Ultra 0.5, High 1, Medium 2, Low 4 px), replacing
today's view-distance bias, size cutoff and ring distances for statics. No hard draw distance: streaming decides what
is resident at full detail, the pixel rule decides what is drawn and how coarse.

**Swaps.** A level changes only when the difference is under a pixel, so no fade is needed; under DLSS a sub-pixel
change is resolved temporally anyway. If the owner sees pops, the fallback is a per-class dithered crossfade driven by
the same error (one extra level drawn in a narrow band), not per-instance ranges (the 65535 `VisibilityRange` slot cap,
streaming.md "Visibility range slots", disappears with this design).

### 5. Materials: few pipeline states

In a GPU-driven renderer the draw count is the number of **batch sets** (pipeline × bind group × mesh slab), not objects.
Our statics today carry the union of NIF materials (`CookedMaterials` dedups identical ones only).

- **Material classes:** opaque, alpha-masked, opaque + wind, masked + wind, each ± the effects our patches key on
  (GI `1<<26`, surface weather `1<<27`). Target ≤ ~16 classes for all statics.
- **Per-instance material data** lives in a bindless material table (Bevy's `MaterialBindGroupAllocators`,
  `bevy_render/src/material_bind_groups.rs:82`), indexed per instance: textures by bindless index, tint, flags.
- **Textures** stay individual (bindless) first; texture arrays or atlases only if measured binding limits demand it
  (the Greatness7 fork atlases because D3D9 had no bindless).
- Measured in slice S0: the number of distinct batch sets per view today. Target after S2: ≤ 2× the class count.

### 6. Drawing through Bevy (vendored patch "GS")

Built on #25978's model, rebased onto our vendored `bevy_render`/`bevy_pbr` (0.20.0 + patches, `superfargoth/vendor/README.md`):

- a batch entity per (space, material class) carries `GpuBatchedMesh3d`-like data: capacity, the template (class
  material, mesh slab, flags) and the persistent instance buffer;
- a renderer-owned pass materialises the culled work items into the tail of the preprocessing input and culling buffers
  (`MeshInputUniform`, `bevy_pbr/src/render/mesh.rs:604`; `MeshCullingData` gains `is_active`), so Bevy's prepass,
  deferred/forward, shadow and indirect paths draw them unchanged;
- one `multi_draw_indirect_count` per batch set where the adapter supports it (`gpu_preprocessing.rs:940`), else
  `multi_draw_indirect`.

**What we add to #25978** (and offer upstream): our own cull (§4) as the producer, LOD index ranges per instance,
cell-range upload/free, per-shadow-view culling, motion-vector history (§7), and the item count that our vendored
#26161 `SkipRootViewRendering` hash reads (`BinnedRenderPhase::entity_count`), or lamp faces would skip.

This is within CONTRIBUTING rule 4: a vendored patch carrying an upstream PR, written to be upstreamed, no code
reaching into private render-world state from our crates. Not chosen: a public-API bypass (`BinnedRenderPhaseType::NonMesh`,
`custom_phase_item`): it forks material specialisation, prepass and motion vectors, and breaks on every Bevy upgrade;
our own renderer (SPUD-style): loses Solari, DLSS, GI and upstream work.

**Port shape (S3a, 2026-11-09).** Commit `36e79e21` of #25978, `bevy_pbr`/`bevy_render` hunks only, applied by context
(our 0.20 files had not diverged there). One deviation: no `Mesh3dVisibility` class (it lives in `bevy_mesh`, would
need `bevy_mesh`/`bevy_camera`/`bevy_light` vendored and breaks our crates that key lists by `Mesh3d`): the batch
pushes the `Mesh3d` class and a `bevy_pbr` system adds it to light shadow lists. `GpuInstanceBatchPlugin` is opt-in.
Detail: `superfargoth/vendor/README.md` "The GPU-batched mesh port".

### 7. Motion vectors (DLSS)

#25978 has none. For statics, `MeshInputUniform::previous_input_index` points at the instance itself (previous transform
= current), and camera motion comes from the view's previous matrices, as for any static mesh today. Wind and UV-scroll
shaders take the previous frame's time to produce their own motion (the prepass vertex shader evaluates the sway twice).
A reviewer on #25978 already asked for a flags field to carry history bits; we use it.

### 8. Shadows and other views

Every view with a binned phase runs §4 with its frustum: 4 sun cascades, lamp cube faces (the lamp cache in
[shadows.md](shadows.md) "Caching" keeps working: a cached face simply skips its dispatch), probe and reflection views.
Far shadows (the far shadow map) read the same table with the far-node levels. Shadow LOD uses the cascade's texel
density (§4 step 5, orthographic form), so far cascades take coarse levels by the same rule; lamp cube faces are
perspective and use the normal form.

### 9. Far field on the same table

HLOD proxies (L1-L3) and impostors become instances with their own error and a `far` flag; their source cells keep
their own instances. Selection is the same pixel rule with a residency rule from far-field.md "Selection, residency, and
crossfade" (a node draws only when its children are not all resident and selected). The lead bands and per-node
`VisibilityRange`s go away. Impostors keep their own material class (octahedral atlas).

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
| CPU per view for statics (main + render thread) | ≤ 0.05 ms, independent of instance count |
| Render thread, all statics, all views | ≤ 1 ms (estimate; today several ms, `prepare_preprocess_bind_groups` alone 2.3 ms) |
| GPU cull, all views, 100k resident instances | ≤ 0.3 ms (estimate) |
| Stream-in of a cell | ≤ 0.2 ms upload, zero entity spawns |
| Batch sets per view | ≤ 2 × material classes |

## Crates

- `sf_scene` (Bevy-free): table format and reader, eligibility and promotion rules (adapter hook), LOD pick math, a CPU
  twin of the cull for tests, slot allocator. Builds cold in seconds (build-perf.md).
- `sf_scene_bevy`: streaming integration, GPU buffers, cull shaders, batch entities, debug views.
- `superfargoth/vendor` patch "GS": the #25978 base plus our additions (§6), upstreamable, MIT/Apache, no GPL lines.
- Adapter (`mw_world` / `mw_exterior_bevy`): the eligibility hook (scripts, record kinds) and the material class map.

## Verification

- **Exact tests** (`sf_scene`): table round trip and versioning; slot allocator; LOD pick (resolution and FOV scaling,
  monotone with distance); CPU cull twin vs hand-built cases (frustum, horizon, sub-pixel); eligibility and promotion.
- **GPU vs CPU twin:** a headless test reads back per-view work items for a fixed camera and compares them with the twin
  (counts and ids, no images).
- **Measurement** (`#[ignore = "measurement: ..."]` and the perf suite): per-view instances, work items, batch sets,
  bind groups, render-thread ms, GPU cull ms; before (S0 baseline) and after each slice, ×3 runs.
- **Visual checks** for the owner (`bevyrrowind/docs/visual-checks/`): LOD swaps at eye height and in flight, shadows
  per cascade, DLSS ghosting on wind-swayed foliage, horizon culling on ridges, promotion of a grabbed object. Agents do
  not judge images (CONTRIBUTING "Tests and visual checks").

## Slices

| # | Slice | Done when |
|---|---|---|
| S0 | Instrumentation: per-view instances, work items, batch sets, bind groups, pipeline states in the perf report | baseline on all benches, ×3. **[~] 2026-11-08** |
| S1a | `sf_scene` core: table format v1 (`table.rs` docs), eligibility trait + `Promotions`, slot allocator, CPU cull/LOD twin | **[x] 2026-11-08** |
| S1b | `scene.cells` bake + adapter eligibility hook and material class map | Vvardenfell tables baked, tests pass, bake time within bake.md gates. **[~] 2026-11-08** |
| S1c | Table format v2 (review fixes before any bake) | §2 "Format v2 (S1c)". **[x] 2026-11-09** |
| S2 | Material classes + bindless material table for statics (on the current entity path first) | batch sets per view measured down; look unchanged (owner). **[~] 2026-11-08** |
| S3a | Patch "GS" base: #25978 ported into vendored `bevy_pbr`/`bevy_render` 0.20 (§6 "Port shape"), with its sanity test | **[~] 2026-11-09** |
| S3b | Scrib runtime on the port: batch entities fed from the baked tables, main view only, opaque class, no LOD | one town’s opaque statics drawn from the table; numbers |
| S4 | All classes, LOD ranges, pixel-error presets, motion vectors | statics fully on the table in the main view |
| S5 | Shadow views (cascades, lamp faces, #26161 count) + far shadows | shadows from the table; per-view cost flat |
| S6 | Horizon + Hi-Z occlusion; Solari decision (§10) | numbers; decision logged |
| S7 | Far field on the table (§9) | lead bands and node ranges removed |
| S8 | Promotion and runtime demotion paths; picking | grab, script and mod cases tested |
| S9 | Retire per-entity statics and precombine; upstream PR | old path deleted, `--off scene` gone, PR open |

As built per slice (what each built, what is still open, the S1c follow-ups): [archive/static-scene-s0-s2.md](archive/static-scene-s0-s2.md); S3a: `bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md`.

S2 pays on its own and de-risks S3. Each slice reports ms per preset.

## Risks and open questions

- **#25978 may change or stall upstream.** We carry it as a patch either way; rebasing it on each Bevy release is the
  cost. Mitigation: keep our additions separable, offer them upstream early, follow the Rendering working group.
- **Motion vectors** for wind-swayed foliage need the prepass to evaluate two times; verify DLSS ghosting (owner).
- **Driver support:** `multi_draw_indirect_count` on our target (Vulkan, DX12 via wgpu); fallback path needed.
- **Bindless limits** on the 4060 Laptop with Morrowind + mod texture counts; atlases only if measured.
- **Memory:** a full worldspace table is small (≈ 80 B × instances; 100k ≈ 8 MB), LOD chains share vertices; check heavy
  mod lists.
- **Eligibility errors** (a scripted static treated as static) show as "can't interact": promotion on any record or
  script reference must be conservative.
- Open: per-instance tint and vertex colour variation within a class; animated-but-static NIF controllers (which stay
  entities, and how many in a town); whether interiors (few hundred objects) are worth the table from S3 or later.
