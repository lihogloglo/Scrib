# MGE XE code read (2026-10-25)

**Verdict update (Greatness7 fork, same day).** The "no merging, no atlas" verdict below is true only of Hrnchamd's
`a2a2a527`. The **Greatness7 fork** (`9bb5e97`, 2026-10-20, GPLv2) does exactly what the owner was told: per exterior cell
it merges placed statics into one baked-world-space mesh per (atlas page, alpha class) subset, with all static textures
packed into shared BC1/BC3 atlas pages, plus terrain horizon culling and streamed residency. This **confirms rows 2 and 5
of the ranked list as the direction** (one material per chunk via atlas; one draw per chunk per class), no longer "idea
only". The row-2 mechanism is an offline atlas with per-vertex UV-bound clamping, not a texture array. It is still not
proven to move a Bevy frame (their evidence is a D3D9-on-DXVK draw-count win; row 1 trace stays the gate). See the
section "Greatness7 fork" at the end.

Slice `mgexe`. Question (owner): MGE XE "merges far cells and uses an atlas and gets ~500 fps; what can we learn?"
Source: `C:\cargo-target\refs\mge-xe`, clone of github.com/Hrnchamd/MGE-XE, `master` @ `a2a2a527aa0c37c314c491f08e2f8b45da7cce38`
(2024-11-11; the maintainer's repo, taken as canonical. Greatness7/MGE-XE is a separate repo with the same
description and a push on 2026-10-20; not inspected, worth a look for newer work). Paths below are relative to that clone.

## Licence

`license.txt` is the GPL **version 2** text. No source file carries a per-file licence header (grep for "later
version" / "GPL" in `src/` and `MGEgui/` finds nothing). Under GPLv2 section 9, a program that does not specify a
version may be used under any version, so code is combinable into our GPL-3.0-or-later work, but that is an
inference from the absence of a header, not an explicit "or later". If we port code, record file@commit in the file
header and in this note; the owner may want to confirm with the maintainer (Hrnchamd) before shipping a non-trivial
port. Technique-only reuse has no licence question. `3rdparty/` (niflib etc.) has its own licences.

## Verdict

**The premise is mostly wrong, and the code contains no trick that explains 500 fps.** MGE XE does *not* merge
distant statics into combined meshes, and the atlas is *not* a statics texture atlas.

- Distant **land** is the only merged-plus-atlased thing: the whole world heightfield is tessellated offline
  (ROAM, `MGEfuncs/LandTessellator.cpp` `TessellateLandscapeAtlased`) into patches that all sample **one** world
  colour atlas, **one** normal atlas and one detail texture (`MGEgui/DistantLand/DistantLandForm.cs`
  `AtlasSetup`, `WorldTexCreator`). So land = a handful of draws, one material, no texture switches.
- Distant **statics** are one **unmerged draw per instance per texture subset**, each with its own world matrix set
  as a shader constant (`src/mge/quadtree.cpp` `VisibleSet::Render`, `DrawIndexedPrimitive` per instance). They are
  batched only by state-sorting (texture, then vertex buffer) and by D3D9 draw-call cheapness. Only **grass** is
  hardware-instanced (`rendergrass.cpp`). Textures are individual DDS files per source texture (mips skipped by 1
  by default); no static atlas, no texture array.
- What keeps the count low is **selection and culling**, not merging: a size threshold at bake time (default radius
  150 units, about 2 m), three size tiers with distance bands, a spherical distance cut per tier, a depth-10
  quadtree with frustum tests, a deliberately **vanilla near field** (Morrowind's own short draw distance and
  culling), and only 2 shadow cascades of 1000 and 4000 units radius.
- "500 fps" is a different workload class: a D3D9 immediate-mode renderer where a draw costs a few microseconds of
  driver time, one main view plus two small shadow layers, no per-entity ECS cost, on hardware far faster than the
  engine was built for. The MGE readme itself says the base engine is single-threaded and old and that cities are
  slow, and its own top advice is "reduce distant land draw distance". It is not evidence that merged chunks beat
  our design; it is evidence that **fewer, cheaper views and a hard size/distance budget** win.

Transferable lessons: (1) a per-view entity/draw budget enforced by size-tiered distance bands, (2) one material for
the whole land (atlas) and as few as possible per chunk, (3) cheap shadows (two cascades, casters only from the
size-filtered set), (4) state-sort / instance by mesh. None needs their code; the code worth re-reading is the tier
assignment and the grass instance packing (both tiny).

## 1. Distant Land generation (offline wizard)

Files: `MGEgui/DistantLand/DistantLandForm.cs` (3146 lines, C# WinForms), `MGEfuncs/NifConverter.cpp` (native
mesh export, niflib), `MGEfuncs/LandTessellator.cpp`, `MGEgui/DirectX/DistantLandTextures.cs`.

- **Selection** (`workerCreateStatics`, DistantLandForm.cs ~1135-1330): parse all ESMs, collect every placed static
  per exterior cell (plus large interiors and "behaves like exterior" interiors by option), build the list of distinct
  NIFs. `NativeMethods.ProcessNif(data, size, simplify, minSize, type)` (`NifConverter.cpp` 778) rejects a NIF when
  `radius < cutoff` (wizard default **150** units, `udStatMinSize`, INI `MinStat`); type `STATIC_BUILDING` (path
  `x\`) uses `radius*2 < cutoff`; `grass\` and `trees\` are forced to grass/tree types; per-NIF overrides come from
  `.ovr` classifier files (`assets/MGE XE Default Statics Classifiers.ovr`: `near`, `far`, `very_far`, `ignore`).
  A `<name>_dist.nif` replacement is loaded first when present (hand-made LODs). Quest-dependent statics get a
  dynamic visibility group (`DynamicVisDataSet`) toggled at runtime.
- **Simplification: none.** `simplify` is hard-coded `1.0f` ("until the simplifier code is fixed",
  DistantLandForm.cs ~1190); the readme says the reduction feature was removed as unreliable and recommends
  hand-made `_dist` meshes. The `progmesh` code is dead for statics. There are no per-tier triangle budgets.
- **Merging: per NIF only.** `ExportedNif::Optimize` (`NifConverter.cpp` 656) merges *subsets of one NIF that share
  a texture path* (`MergeShape`), then vertex-cache-optimises (cache 16). Output `statics.mesh`: per NIF a node
  list, each node with sphere, AABB, flags (alpha test or blend, UV controller), compressed vertices, 16-bit
  indices, texture path. Placements are written separately (`usage.data`: ref, pos, scale, rotation, per cell).
  Nothing is merged across placements or cells.
- **Textures** (`StaticTexCreator`, `DistantLandTextures.cs` 75-190): each used texture is loaded, mips `>= SkipMip`
  (default 1) dropped, converted to DXT1 / DXT3 (DXT3 if the DXT1 has any alpha, "mipmaps generate smooth alphas")
  or kept DXT5, written as its own DDS under `Data Files\MGE XE\distant land\statics`. Grass and tree textures are
  skipped (they use the originals). **No atlas, no texture array, no UV remap for statics.**
- **Land atlas** (`AtlasSetup`, DistantLandForm.cs 731-880): cells with land are grouped into connected regions,
  regions are packed row by row into one big span (`AtlasSpanX/Y`); `WorldTexCreator` renders per-cell blended
  ground textures (`CellTexCreator`, 65x65 vertex splat render) into the world colour atlas (DXT1, sRGB, mips
  built) and a second atlas for normals (`TexRes`, `NormRes` options). `TessellateLandscapeAtlased` takes the
  heightfield and atlas rectangles and produces ROAM patches (error tolerance, tree depth 14) with atlas UVs.
  Alpha-tested foliage is not atlased anywhere; it keeps `hasAlpha` per subset and uses alpha test / alpha-to-coverage.
- **What does not fit:** anything under the size cutoff is simply dropped (that is the whole LOD policy), as are
  `ignore`d NIFs; actors, lights and anything that is not a static/activator/misc (by option) never enter.

## 2. Runtime

Files: `src/mge/distantinit.cpp` (load, tier assignment 1067-1200), `quadtree.cpp/.h`, `renderexterior.cpp`,
`rendergrass.cpp`, `rendershadow.cpp`, `renderdepth.cpp`, `distantland.cpp` (`renderStage0`).

- **Structure:** at world load every placement x every subset becomes a `QuadTreeMesh` (sphere, AABB, matrix,
  texture, VB/IB pointers) inserted into one of four loose quadtrees (`QUADTREE_MAX_DEPTH` 10): `NearStatics`,
  `FarStatics`, `VeryFarStatics`, `GrassStatics` (plus `LandQuadTree`). Tier = placement radius (post-scale, x2 for
  buildings): `<= SizeFar` (600) near tier, `<= SizeVFar` (800) far tier, else very far; overridable per NIF.
- **Per frame** (`renderStage0`, before the vanilla scene): shadow map early; land (`renderDistantLand`: frustum plus
  spherical range cull of land patches, draw); `cullDistantStatics` runs three frustum cullings, each with its own
  projection clipped to `[nearViewRange - 768, tierEnd]` (tier ends `NearStaticEnd` / `FarStaticEnd` /
  `VeryFarStaticEnd` x 8192, capped by fog end), appends to `visDistant`, then `SortByState` (texture, then VB).
  `VisibleSet::Render` loops: on texture change set texture and alpha-test state, on VB change set the stream, per
  mesh `SetMatrix(world)` + `CommitChanges` + `DrawIndexedPrimitive`. **Draw count = visible instances x subsets**;
  state changes only per texture/VB group. CPU cost = quadtree walk + sort + that loop, all single-threaded and small.
- **Culling:** sphere then box frustum tests with an "inside" shortcut (a fully inside node skips per-mesh tests);
  spherical (not planar) distance cut `range <= tierEnd + radius`; size culling is implicit through tiers (small
  things exist only in the near tier, which ends sooner). No occlusion or horizon culling.
- **Near/far boundary:** the vanilla engine draws the near field with its own short distance and culling. Distant
  things use a near plane of `nearViewRange - 768` and an alpha-to-coverage dissolve across the boundary
  (`vsr.beginAlphaToCoverage`), drawn behind vanilla geometry via a biased projection. Interiors clip with a user
  plane at the same distance. A **depth pre-pass** (`renderDepth.cpp`) redraws recorded vanilla draws, land,
  statics and grass for the depth texture used by fog/SSAO post effects.
- **Grass:** `cullGrass` (range `[4, nearViewRange]`, coarse quadtree test), `buildGrassInstanceVB` packs 4x3
  matrices of all visible instances into one dynamic VB (cap `MaxGrassElements`), `renderGrassCommon` issues **one
  hardware-instanced draw per distinct mesh** (`SetStreamSourceFreq`). Density is thinned offline by the wizard.
- **Shadows:** `renderShadowMap` renders **2 cascades** (radius 1000 and 4000 units, orthographic, ESM, one shared
  texture with two viewports, stencil-limited to the visible frustum, texel-snapped), casters = land + the three
  static quadtrees via `GetVisibleMeshesCoarse` (node-level test only), then a separable blur. Casters therefore come
  from the size-filtered distant set, not from vanilla draws. Receiving is done by **redrawing the recorded vanilla
  draws** with a blend shader (`renderShadow`, `recordMW`): a whole extra pass over the near field.
- **Cost model:** D3D9 draw ~ 2-10 us driver time, so thousands per frame is fine. Matrix constant per draw, no
  instance buffer except grass. One thread; nothing per entity beyond a `QuadTreeMesh` struct.

## 3. Why their draws are cheap and our precombine did not move the frame

Their distant statics are **not** merged chunks (see verdict); the real comparison is "few cheap views over a
size-budgeted instance set in an immediate-mode API" vs "Bevy ECS with 25-55 k mesh entities in 6+ views". Ranked
explanations tied to Bevy's cost model. Items 1-3 fit our measurements; none is proven without a trace:

1. **Cost scales with (batches x views), not entities, and precombine left both about unchanged.** Bevy queues and
   bins per view per `(pipeline, material bind group, mesh buffer)` batch and encodes multi-draws per batch;
   `prepare_preprocess_bind_groups`, `queue_submit` (33 submits), shadow binning and encoding (measured 2.3 + 2.4 +
   1.4 + 1.2 ms) scale with distinct bind groups, work items and views. Our merged chunk keeps **the union of its
   members' materials** (precombine.md "Design"), so a town cell still has dozens of materials per chunk, every LOD
   level is a separate entity, and size buckets multiply that. Meshes fell 45 %, but distinct (material, mesh)
   batches per view barely fell, and that is what the render thread pays.
2. **View multiplication.** MGE renders the world once plus 2 small shadow layers (casters pre-filtered). We render
   main + 4 sun cascades + extra sun views (`NoIndirectDrawing` views cull nowhere) + prepass (perf.md "Lead").
   Every per-view cost (queue, bin, prepare, encode, submit) is multiplied by 6-8.
3. **Per-material cost is not removed by merging geometry.** MGE avoids the material explosion by construction:
   land is one material, statics sort by texture so a change is one `SetTexture`. In Bevy each material is an
   asset with a bind group and a specialisation key; thousands of small materials in view cost bind-group lookups
   per view even when meshes are merged. Bindless/array materials are the way out.
4. **Fixed work outside statics.** `--off statics` also removes ~290 point lights, items, doors and NIF objects
   (perf.md says so). Lights drive clustering and, if casting, extra views. A measurement confound, not an MGE gap.
5. **The atlas** matters only through (3): it is how land gets to one material. It is not why statics are cheap in
   MGE, because they have no atlas.

Decisive experiment before spending effort (not run; no builds in this slice): per-view counts of
`(batches, bind groups, work items)` from a trace on `balmora_walk` with precombine on/off, and with `--off statics`
but lights kept. If batches per view are about equal with precombine on/off, point (1) is confirmed.

## Ranked changes for us

Effort: S under 1 day, M 1-3 days, L a week or more. "Technique" = no MGE code needed. Where code is worth porting it
is tiny and needs only a provenance comment (file@a2a2a52).

| # | Change | MGE XE reference (@a2a2a52) | Expected effect on our render thread | Effort | Port | Where / spec |
|---|---|---|---|---|---|---|
| 1 | **Trace batches, bind groups and work items per view** with precombine on/off before building anything below. | n/a | Decides which row below pays; avoids another null result like precombine. | S | n/a | bw perf harness; `docs/perf.md`, `hard-problems.md` H1 |
| 2 | **One material per chunk via texture array / bindless atlas** for precombined buckets: members' materials remapped to array slots (layer id as vertex attribute), alpha-tested foliage a separate bucket with its own pipeline. Chunk = 1-3 batches per view. | Idea only; MGE does not do this for statics (land atlas: `DistantLandForm.cs` `AtlasSetup`, `WorldTexCreator`, `LandTessellator.cpp`) | Largest plausible win if (1) confirms: batches and bind groups per view drop by an order of magnitude, times every view (main, 4 cascades, prepass). | L | Technique (Rust/WGSL from scratch; arrays need uniform size per array, so bake groups textures by size class) | sf `sf_lod` bake `lod.near`, `sf_world_bevy` material; changes `precombine.md` "Design" (materials = union) |
| 3 | **Cut shadow views MGE-style**: 2 cascades (about 14 m and 55 m radius), casters only from the size-filtered set, node-level coarse tests, no small casters. We run 4 cascades plus extra sun views, each re-queuing the ring. | `rendershadow.cpp` `renderShadowMap`/`renderShadowLayer`, `shadowNearRadius` 1000, `shadowFarRadius` 4000 | Direct per-view multiplier: 4-6 down to 2 shadow views cuts binning + encoding (2.6 ms measured) and prepare by about the view ratio. Quality tradeoff for the owner (far shadow distance). | M | Technique | bw render settings + sf shadow code; `perf.md` "Lead"; owner fps/shadow decisions |
| 4 | **Size/distance tier bands as a hard entity budget** at bake: radius thresholds to three bands with distance ends (MGE: min radius 150 u, tiers 600 / 800 u), spherical range, per-NIF override list; dissolve (alpha-to-coverage) at the boundary. Small clutter never becomes an entity beyond its band. | `distantinit.cpp` 1103-1160 (tier select), `renderexterior.cpp` `cullDistantStatics`, `NifConverter.cpp` `ProcessNif` cutoff, `.ovr` classifiers | Cuts entities and batches in all views at the source; stacks with 2. Pop risk mitigated by dissolve. | M | Technique (a few lines of logic; `.ovr` idea maps to content-pack overrides) | sf `sf_lod` size cull (industry-audit J1), `far-field.md`, `precombine.md` bucket edges |
| 5 | **One entity per chunk per material class** (opaque / alpha-test / blended) with LOD levels inside the chunk instead of one entity per level per bucket. | Idea only | Smaller but real: fewer entities in extract/visibility and fewer `VisibilityRange` evaluations; secondary to 2. | M | Technique | sf `sf_lod`; `precombine.md` ("level k = every member's LOD k") |
| 6 | **Land as one world atlas and a few patches** for far terrain; check whether ours already is, else fold per-cell terrain materials. | `LandTessellator.cpp` (ROAM, atlased UVs), `AtlasSetup` (row packing of connected regions, ~150 lines) | Terrain draws collapse to a few per view; likely small (bare baseline is GPU-bound at 1440p). | M | Technique; `AtlasSetup` is portable verbatim if wanted | sf terrain crate; `far-field.md` |
| 7 | **Instance-buffer grass**: one packed matrix buffer, one instanced draw per mesh, density thinned at bake. | `rendergrass.cpp` `buildGrassInstanceVB`, `renderGrassCommon` | None now; matters when groundcover lands (Bevy batches identical meshes already if keys match). | S | Technique | later groundcover slice |
| 8 | **Dynamic visibility groups** (quest-toggled statics without rebake) instead of per-reference exclusion in precombine. | `DynamicVisDataSet`, `distantland.cpp` `resolveDynamicVisGroups`, `.ovr` quest lists | No render effect; lets more members merge. | S | Technique | bw adapter `PrecombineSource::exclude`; `precombine.md` |

Not worth porting: the D3D9/HLSL pipeline; the quadtree (big_space + Bevy visibility cover it, and it only helps
main-thread culling, not the render thread); `progmesh` simplification (they disabled it as unreliable); the
per-draw matrix constant loop (what Bevy GPU preprocessing replaces).

## Greatness7 fork (9bb5e97f08f0ff513556f0669d1e3394a388e862)

Source: `C:\cargo-target\refs\mge-xe-g7`, shallow clone of github.com/Greatness7/MGE-XE, HEAD 2026-10-20 ("Allow
NiBillboardNode in distant statics"). Licence: `license.txt` is GPLv2 text, same as upstream (same inference as above:
no explicit "or later" seen). Read from the fork's own docs (`README.md`, `CHANGELOG.md`, `distantland/ARCHITECTURE.md`,
`distantland/docs/architecture/statics.md`, `docs/architecture/{distantland-data,horizon-culling,distant-static-residency,
static-lod,shadows}.md`) plus module layout; the Rust generator code was not read line by line, and **no runtime
measurements of draw counts were reproduced** (their numbers are the author's claims). It is a rewrite: Rust generator
(`distantland/`, crates `statics`, `terrain`, `texture`, `usage`...), Rust 64-bit host (`mgeHost64`); the 32-bit D3D8 proxy
in `d3d8/cpp/mge/` is still C++. Data version `MGE_DL_VERSION` 16 (incompatible with upstream). Needs a custom DXVK
(`d3d9.dll`), so the runtime is D3D9-on-Vulkan, not native D3D9.

**What it adds over `a2a2a52`, in distant land**

1. **Per-cell merge (the "one or two draws per cell" claim).** Generation: after per-NIF optimise (meshopt simplify,
   vertex-cache) and a post-optimise size filter, `distantland/crates/statics/src/merge.rs` (`plan_exterior_merge_groups`,
   `build_merge_geometry`) takes, per exterior cell, every static with scaled radius >= 32 and no dynamic-visibility
   group, builds a BVH (`obvhs`) over their world AABBs and cuts groups whose node half-diagonal stays under
   `merge_group_radius` (so "per cell" is really "per spatial group under a radius"). Members' subsets that share a texture
   state are concatenated into one merged static with **vertices baked at absolute world positions**; the remaining
   statics stay ordinary instanced records (one draw per placement per subset, as before). Merged geometry gets a
   "subterrain cull" (drops triangles fully below terrain height minus terrain error; ~7.7 % of merged triangles at
   `high`). Subset key = texture state, which after atlasing is (atlas page, opaque vs alpha-test/blend). Hence "one or two
   draws" = one opaque + one alpha subset per group, assuming everything fits one page; multi-page atlases or other state
   splits add draws. The docs give no hard "1-2" guarantee.
2. **Statics texture atlas (did not exist upstream).** `distantland/crates/statics/src/atlas/` (`manager.rs`, `pack.rs`,
   `plan.rs`, `reconcile.rs`, `sizing/`, `uv.rs`; MaxRects packer via `tex-packer-core`). Two families: opaque pages
   `statics\textures\_mge_xe_atlas{N}.dds` (BC1) and alpha pages `_mge_xe_atlas_alpha{N}.dds` (BC3), full mip chains, page
   size capped by `TextureAxisCaps`. Textures are density-sized first (`AnalyzeTextureDensity`: target resolution from UV
   Jacobian per texture) and de-duplicated by content fingerprint. It is a classic **atlas, not a texture array**. Tiled
   UVs are handled by a per-subset **UV-bound palette** (16 B/entry, `XESTAT06` v6, `distantland-data.md`): each vertex's
   `position.w` is an ordinal into the subset's palette and the shader clamps/remaps wrapped UVs into that sub-rectangle
   (the shadow shader uses `tex2Dgrad` with explicit derivatives because `frac()` breaks mip selection). Vertex = **20
   bytes** (grass 20, was 28), 16-bit indices, terrain indices narrowest-width. Atlas slots are stable across regenerations
   (reserved rectangles, `Carry`/`Build` pages) so incremental rebuilds touch only dirty pages.
3. **Packed format and sharding.** 128 fixed shards `statics\static_meshes_000..127` plus `usage.data`; v0.21 is ~23 % smaller
   in VRAM/disk. Merged statics are 2-8 GiB on big modded installs (their docs), so merging trades memory for draws.
4. **Distance tiers inside a merged mesh (no draw multiplication).** `static-lod.md`: each merged subset keeps
   `ComponentRecord` triangle ranges (radius, type) and a **cumulative index buffer ordered [very-far][far-only][near-only]**;
   the host picks a total/far/very-far *prefix* per query, so tiers do not split a merged subset into three draws. Their
   docs say the earlier tier-split design was rejected because it "multiplied merged subsets and their draw/sort/quadtree
   overhead". Directly relevant to our "one entity per LOD level per bucket". Also "three triangle budgets per object".
5. **Horizon (terrain) occlusion culling.** `docs/architecture/horizon-culling.md`; generated `terrain_occlusion.bin`
   (max-height grid); host builds a camera-relative polar horizon table (max-height pyramid, async rebuild, adaptive
   self-suspend), prunes quadtree nodes and meshes before draw submission. Terrain is the only occluder; no building
   occlusion, no GPU queries. Reported: ~41 % FPS in dense Sunad Mora, a Balmora route "roughly" similar.
6. **Streaming.** Only merged statics stream (`distant-static-residency.md`, client `d3d8/cpp/mge/distantstatics.cpp`, host
   `plan_residency`): VRAM cap from a DXVK budget interface, resampled every 500 ms, ratcheted downward only. Everything
   else (ordinary statics, textures, palettes, terrain) uploads once; terrain uploads in frame-budgeted chunks.
7. **Shadows unchanged in structure:** still 2 cascades, ESM; casters are distant terrain + distant statics only (merged
   meshes cast; alpha test samples the atlas region); Morrowind's near field casts nothing in MGE shadows.
8. **Other:** incremental regeneration, native depth capture (reads the real depth buffer instead of redrawing the scene,
   removing the depth redraw), BC7, camera-relative rendering, grass in interiors.

**Draw-call counts.** Claimed: "cells that used to cost hundreds or thousands of draw calls now cost one (or zero if
occluded)"; 6x FPS on the author's 9-year-old machine at 16-cell distance, 15x+ at very large distances, 2x in dense modded
cells (README). D3D9/DXVK, draw-count-bound numbers on an engine whose near field (Morrowind's own draws) is untouched.
Not independently checked.

**Does it change the previous verdict and ranked list?**

- *Premise:* yes for this fork; the owner's description is correct (merge per cell + atlas + horizon culling + streaming).
  The old note was right about Hrnchamd's repo and wrong to treat it as the whole story.
- *Row 2 (one material per chunk via atlas/array):* **upgraded from "idea only" to a proven recipe, in a different API.**
  Offline atlas of all static textures in two families (opaque, alpha), per-vertex UV-bound ordinal for tiled UVs, BC1/BC3,
  density-based sizing, dedupe, 20-byte vertices. For Bevy the same atlas gives 1-2 materials per chunk (an array/bindless
  set avoids UV clamping but needs uniform sizes). Evidence of benefit for *Bevy* is still absent. Keep row 1 (trace
  batches x views) as the gate, but row 2 is now the expected winner, and it fixes the precombine flaw ("material = union of
  members'"). Merge without atlas is what our precombine already tried; the fork only pays because the atlas collapses
  materials, which matches explanations 1 and 3 above.
- *Row 5 (one entity per chunk per material class, LOD inside the chunk):* **confirmed and sharpened.** Use the cumulative
  far-to-near index buffer with per-tier prefix ranges (sub-range draws) instead of one entity/mesh per LOD level.
- *New row 2b:* **horizon culling against a baked terrain max-height grid** plus a **merge-group size cap** (BVH half-diagonal
  < `merge_group_radius`) so merged meshes stay cullable. Not in our list before; the fork reports 40 %+ FPS in dense
  routes. Candidate for `far-field.md`.
- *Row 3 (cut shadow views):* unchanged; the fork still uses 2 cascades with distant-set casters.
- *Row 4 (size/distance tiers):* unchanged, but tiers become index ranges inside merged chunks, not entities. Concrete bake
  rule: merge only radius >= 32 units, no dynamic-vis group.
- *Row 8 (dynamic visibility groups):* confirmed relevant; it is the exact exclusion that keeps statics out of merges.
- *Caveats:* (a) merged meshes bake world positions and cost GiBs on heavy mod lists, hence their streaming; our precombine
  will face it. (b) Atlas costs: tiled textures lose hardware wrap (UV clamp or exclusion), mip bleed at low mips (needs
  gutters), alpha-tested foliage needs its own page family and derivative-correct sampling.
- *Licensing:* GPLv2, no explicit "or later"; same caution as above. Technique reuse is free; do not copy `uv.rs`/`merge.rs`
  without a provenance header and owner sign-off.

Read next if we implement: `distantland/crates/statics/src/atlas/{pack,uv,sizing}.rs`, `distantland/crates/statics/src/merge.rs`,
`distantland/crates/statics/src/model/pack.rs`, `docs/architecture/{static-lod,horizon-culling}.md`.
