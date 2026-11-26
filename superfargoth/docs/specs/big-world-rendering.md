# Big-world rendering: millions of trees and dense scatter (`sf_bigworld`, `sf_bigworld_bevy`)

Status: **BW-1 built, BW-2 and BW-2b measured** (2026-11-26; "As built"): the Bevy-free core `sf_bigworld` exists, the
ceiling bench tripped the stop rule, and the fixes it priced are the "Fill rules" under "Design". Nothing draws in game
yet. Owner go for Stage A (BW-1, BW-2) 2026-11-26. **Owner 2026-11-26 (BW1): far tree shadows come from a simple
opaque proxy, not from each impostor card** ("the fastest option"; the A/B stays for the owner check). Lane roadmap: [docs/roadmaps/bigworld.md](../../../docs/roadmaps/bigworld.md)
(later build slices `[p]` until the owner's go). Owner brief (2026-11-26): render a world about 10x Vvardenfell with on the
order of 10 million trees and dense statics, the way Chlorophyll, UE5 Nanite foliage and PCG, Decima, RDR2 and Cyberpunk
do, within the frame target. It runs **on the side**: new crates only, and no build time or crates taken from other lanes.
Earlier idea: `superfargoth/docs/WISHLIST.md` §17.

## Goal

A forest you can walk into and fly over. Trees stand as individual crowns from your feet to the horizon. Dense rocks and
props sit between them. None of it pops, and the frame cost depends on the **view**, not on the world's size. The
reference load is about 12-15 M trees on about 2,500 km² of land: 50-70 trees/ha on average (Chlorophyll's test,
10 M trees on 15 km², averages 67/ha), with stands up to 400/ha. The budget is in "Cost model".

Non-goals: grass and small plants (groundcover.md); buildings and hand-placed statics (static-scene.md and
far-field.md keep them); individual trees as gameplay objects (felling or harvest come later, as overrides); placement
rules (worldgen.md stage 8 owns *where* trees go); ray tracing (Solari, open).

## What runs today (checked in code, 2026-11-26)

- **Morrowind exteriors.** Near statics come from the static scene: STAT only, LOD 0 to the cell edge, culled by the
  vendored GPU preprocess (`mw_exterior_bevy/src/exterior.rs:364`, `sf_scene_bevy`). Past 400 m the far field takes
  over: L1-L3 HLOD and octahedral impostors. There is one impostor entity per node batch, with 4 vertices per instance
  and no per-instance cull. Atlases are 8×8 frames of 32 px, 256² RGBA8 (`far-field.md` "Data model",
  `sf_lod/src/impostor.rs:84`). Measured: 121 impostor models, 80 MiB of atlases.
- **Generated worlds** (x10s, Balmora 2.0) turn off the static scene, the precombine **and** the far field (scaffold
  `WorldgenPlacementStatics`, `mw_exterior_bevy/src/worldgen.rs:10`). Every scattered tree is one entity. Owner run,
  2026-11-19 (ledger worldgen): full flora of 7.21 M placements ran at **26 fps**. 200 k ran at 66-108 fps, with 23 k
  shown entities and 890 pipelines pending at reveal. Removal path: X6, a baked worldspace (world10x lane).
- **Placement:** `sf_worldgen::scatter` is deterministic per 256 m tile (counter hash, tiles bitwise independent). A
  full 7.21 M run takes 6.3 s; one 256 m tile takes about 1 ms (`sf_worldgen/src/scatter.rs:1-30`).
- **Groundcover** uses CPU tile scatter into the vendored `GpuBatchedMesh3d` (64 B slots, cost ~ capacity × views, no
  per-instance range). Its budget is 0.8 ms (`groundcover.md` "Renderers", "Budget").
- **Frame:** exteriors run 10.5-11.6 ms median today against the 8.3 ms target (`docs/ledger/perf.md`), so this system's
  budget has to fit the frame plan (ROADMAP 1.22). It is not extra time.

## Reuse check (CONTRIBUTING rule 0/2)

Sources: research notes `bevy-and-crates` and `industry-forests` (bigworld-1, 2026-11-26). Code was read from the Bevy
0.20 registry sources, our `superfargoth/vendor/bevy_pbr`, and clones of each crate. Facts below cite those.

**Bevy 0.20.**
- **Meshlets (`MeshletPlugin`): rejected for vegetation.**
  - Opaque only: alpha-masked materials are refused (`bevy_pbr/src/meshlet/asset.rs:30`).
  - No vertex-shader hook, so no wind. No skinning.
  - Vulkan/Metal with 64-bit atomics only (`meshlet/mod.rs:96,118-127`).
  - No streaming.
  - Every instance is re-extracted and re-uploaded every frame (`instance_manager.rs:241` TODO), so the CPU cost scales
    per entity.
  - Upstream tracking (#11518) is stale. The far-forest aggregate is opaque, so meshlets are re-checked for that tier
    only at BW-7 (see "Open questions").
- **Stock GPU preprocess: used, as is.**
  - Per instance and per view it runs an OBB frustum test, a visibility range (`NoCpuCulling` only) and two-phase hi-Z
    occlusion (`mesh_preprocess.wesl:205-300`). Hi-Z covers directional cascades too (`light.rs:2036-2061`).
  - The game camera already carries `OcclusionCulling` (`exterior.rs:1052`).
  - Per-entity CPU visibility and extraction rule out 10 M entities, so instances never become entities.
- **Vendored `GpuBatchedMesh3d` (bevy#25978 port, patch GS): the draw path.**
  - 64 B slots. A compute producer fills them in `GpuInstanceBatchSystems::Publish`.
  - Slots draw through Bevy's own mesh pipeline with any material: alpha mask, prepass, cascades, hi-Z and lamp views
    come for free.
  - Limits: `materialize` and the preprocess run over **capacity**, not over live slots, in every view. There is no
    per-instance LOD or range. Patch GS ranges allow LOD only on the producer side, by slot span. So capacity must stay
    around 10⁵-10⁶, and the producer must cull before it writes.
  - Bevy never raises `max_storage_buffer_binding_size` (128 MiB by default). We raise it at device creation.
- **Upstream:** #25978 is open and blocked. GPU visibility-range culling was reverted (#23115 by #24252). Nothing
  exists upstream for impostors or per-instance LOD.

**Crates.**
- **`meshopt` 0.7** (already Bevy's dependency, MIT/Apache): `simplify_with_attributes_and_locks`, `simplify_sloppy`
  and `simplify_prune` for leaf clouds, plus the voxel `remesh`. **Used** for LOD chains and the canopy aggregate.
- **boimp** (MIT/Apache): an octahedral baker and renderer with depth parallax. It is stuck on Bevy 0.15 and draws one
  entity per impostor. **Not used**: `sf_lod::impostor` already captures octahedral atlases (CPU, Bevy-free). boimp's
  depth-correction shader (`fragment.wgsl:62-89`) is prior art for BW-4.
- **bevy_eidolon 0.5 / bevy_feronia** (MIT/Apache, Bevy 0.19): chunk AABB cull, per-instance distance bands and
  indirect compaction, with 32 B instances. It is the **pattern** our producer follows (BW-3), not a dependency: it has
  no shadows, no alpha mask and its own pipeline.
- **feathrise_vegetation** (0.20, MIT/Apache): proves that alpha-mask plus shadows plus dither LOD work through stock
  PBR, but its batches are CPU-fed. Reference only.
- **bevy_flora** (0.20): has 8-yaw impostors only, at about 10⁵ trees.
- **ze_tree**: has no licence, so it is a blocker until the author adds one (ez-tree itself is MIT).

**Our own code.**
- `sf_lod`: `impostor::capture`, `octa`, cluster and simplify (`cluster.rs`, `build.rs`).
- `sf_lod_bevy`: `FarShadowExt` (the terrain far shadow map) and the lead-band `VisibilityRange` crossfade.
- `sf_wind::sample` (shared wind) and `sf_coords` (`WorldPos`, floating origin).

**Not reused, and why:**
- The far field's per-node impostor batches: per-instance data in every L1 and L2 node scales with the world.
- The static scene's cell tables: per-placement, baked per source cell.
- `bevy_grass`: grass-only render phase.

## Industry comparison

| Engine | Near | Mid | Far | Placement / culling | Source |
|---|---|---|---|---|---|
| Chlorophyll (indie, D3D12) | meshlet DAG; small triangles turn into voxels by screen error (modified meshoptimizer) | same DAG, voxels | past ~1 km: unique per-tile voxel proxies with no per-instance data, built on the player's machine; 2D option on low-end | mesh shaders, visibility buffer, compute shading; semi-static VSM-like shadows plus a small dynamic map; far animated trees get contact shadows only; grass is not in shadow maps. 3080 at 1440p: ~130 fps average, ~80 in dense forest | author's own statements, graded in zye04/Meridian `docs/superpowers/specs/2026-11-21-chlorophyll-baseline.md` (F2-F4, F9, F19, F22, F26) |
| UE 5.7 Nanite Foliage | assemblies (shared parts), skinned wind | voxel clusters (128 bricks of 4³) when voxel error < triangle error | HLOD layers: Instancing (ISM of the lowest LOD or impostors), Merged, Simplified | GPU instance culling, cap 16 M instances; wind skinning ~0.1 ms per 100 k bones, animation off below screen size 0.1; VSM caching breaks under WPO or skinning | Epic docs: Nanite Foliage, Assemblies, World Partition HLOD |
| Decima (HZD) | mesh LODs | mesh LODs | heightfield far terrain and shadow | GPU placement ~250 µs/frame, >100 k objects, ~4 MB/km² source data, tree blocks 128 m; compute visibility for 0.5-1.5 M static instances in 1-2 ms on PS4 (filter, LOD, frustum, size, hi-Z; camera only, not shadows) | GDC 2017 (van Muijden); GCAP 2017 |
| Fortnite / UE impostors | mesh | octahedral impostor, 2048² atlas, 12×12 frames, 3-frame blend, 8-triangle card, optional depth parallax | HLOD | n/a | Epic Impostor Baker docs |
| SmartImposter (open demo) | mesh to ~50 m | parallax hemi-octahedral impostor to ~3 km | per-tile canopy proxies (HLOD) | 6.2 M plants in an 8 km radius, ~10 ms at 1440p on a 5080 (hardware RT) | repo README, `docs/EARTH_PLAN.md:141-179` |
| RDR2, Cyberpunk, Witcher 3, KCD2, Far Cry 5, MSFS | no public numbers found for trees; not compared | | | | industry note "unsourced" |

**Where we match:** instances are never entities. Placement is rule-driven and deterministic per tile. GPU culling runs
per view with hi-Z. Tiers are chosen by screen size. Mid trees are octahedral impostors, with no per-instance data in
the far field. Shadows sit on a cached, distance-limited path, and animation stops at a screen-size cutoff.

**Where we differ, and why:**
- **No meshlet DAG and no voxel raster of small triangles** (Chlorophyll, UE 5.7). Bevy 0.20 has no mesh-shader
  pipeline and its meshlets are opaque-only. Our mid tier is octahedral impostors (the UE/Fortnite pre-Nanite standard,
  SmartImposter). Our far tier is an opaque, voxel-remeshed **canopy proxy per tile**. That is the same idea as
  Chlorophyll's per-tile voxel aggregate, expressed as triangles (UE HLOD "Simplified"; Loubet & Neyret 2017 hybrid
  mesh/volume prefiltering).
- **No visibility buffer.** Alpha-tested overdraw on near crowns is paid in the depth prepass with early-Z. It is
  watched in BW-2.

## Design

### Tiers

A tree's tier comes from its screen size at the **internal** render resolution:

`px(d) = h · ppr / d`, where `ppr = H / (2·tan(fov_y/2))`.

High preset: DLSS Q internal `H` = 960, fov_y = 60° (`sf_camera/src/comfort.rs:51`), so ppr ≈ 831 px/rad.

| Tier | Shown while | High (12 m tree, 8 m crown) | Representation | Shadows |
|---|---|---|---|---|
| T0 mesh | height ≥ `mesh.switch_px` (80 px) | 0 → ~125 m | cooked LOD chain (meshopt), geometry leaves where the model has them (alpha-card leaves are tested once, fill rule 5), wind | casts through a shadow LOD of its chain, in its own slot range (fill rule 4); wind only inside T0 |
| T1 impostor | crown width ≥ `aggregate.switch_px` (3 px) | ~125 m → ~2.2 km | octahedral impostor, one card per tree on a trimmed hull, 12×12 frames, depth + normal, 3-frame blend, dither fade; flat card depth (no `frag_depth`) | the card casts nothing; inside the cascade range (~500 m) an opaque proxy casts for it (fill rules 1, 2); none past it |
| T2 aggregate | below that | ~2.2 km → horizon | per-tile canopy proxy (opaque, baked), quadtree levels by pixel error | receives the terrain far shadow map; casts nothing |

Rules:
- Neighbouring tiers overlap in a dithered crossfade band (`fade_band`). Inside the band an instance is written to both
  tiers, with its fade in the slot's `tag`. That is the same no-hole rule as the far field's lead bands.
- T2 tiles keep the far field's `VisibilityRange` lead bands, so a coarser level is drawn fully before a finer one
  fades out.

### Fill rules: where the alpha test and the cascades are paid (BW-2b; owner 2026-11-26)

The ceiling bench showed that the slot path is cheap and that alpha-tested content cast into cascades is not ("Cost
model", "Tripped"). Six rules follow, each a standard technique. Sources and `path:line`:
`superfargoth/docs/research/far-vegetation-fill-2026-11-26.md` (part 1 industry: the HZD talk details and the UE
cvar come from secondary sources there; part 3 the Bevy code-read). The ms are BW-2b's `stand` spans at High.

1. **T1 impostor cards cast into no cascade** (`NotShadowCaster` on the batch). UE far LODs and billboards cast
   nothing; HZD keeps shadow casters apart from the visual meshes. Measured: the cards' cascade spans go from 1.24 ms
   to 0, and the bookkeeping from 1.20 to 0.83 ms, because the cards leave four views.
2. **Far tree shadows come from an opaque shadow proxy (owner 2026-11-26, BW1: "the fastest option").** Between the
   mesh end and `shadow.mesh_range_m` the producer also writes each tree into a shadow slot range: a low-poly opaque
   crown per species that only the lights see, on the shadow-only `RenderLayers` layer every worldspace already has
   (`docs/wiki/engine/streaming.md`). An opaque caster takes the depth-only pipeline, with no fragment shader.
   Proxy for its cost: the opaque cards' cascade spans, 0.29 ms (masked cards: 1.24 ms). `bigworld.shadow.proxy`
   turns it off; `bigworld.shadow.impostors` turns the cards' own masked shadows back on, which is the owner's A/B.
3. **T0 leaves are geometry where the model has them** (Chlorophyll F17: trees fully triangles). Measured: the
   trees' prepass + main fall from 2.93 to 0.40 ms without the alpha test. A model with alpha-card leaves keeps its
   mask and follows rule 5.
4. **T0 casts through a shadow LOD.** The visual T0 batch casts nothing; a lower level of the same chain sits in a
   shadow slot range on the shadow-only layer (HZD: one fixed shadow LOD per cascade). Its leaves are opaque
   there. Full meshes in three cascades cost 2.74 ms opaque and 4.33 ms masked; the shadow LOD is not priced.
5. **The alpha test runs once, in the camera prepass.** A masked material on this path (the impostor, alpha-card
   leaves) has a mask-only prepass, and a main pass at depth-equal with no depth write and no discard (UE
   `r.EarlyZPassOnlyMaterialMasking`). In Bevy 0.20 that is `Material::specialize`, with no patch to the vendored
   `bevy_pbr`. It holds only if both vertex stages mark the position `@invariant`, the camera prepass writes the
   rasterised depth (so no `frag_depth`: the impostor's depth frames feed the frame blend and the lighting only),
   and the view has a depth prepass (a view without one keeps the stock state and the discard). Proxy for its
   cost: the cards' main pass, 0.16 ms opaque against 2.73 ms masked; the masked prepass stays near 1.0 ms.
6. **The impostor card is a trimmed hull,** not a quad: 4 to 8 vertices per species keep 52-60 % of the quad's area
   (Persson, Avalanche). Not priced.

Shadow ranges are ordinary slot ranges, so the producer, materialize and Bevy's per-view preprocess treat them like
any other; no pass of our own.

### Data flow (one frame)

1. **Tiles.** The world is cut into tiles of `tile_m` (256 m, the scatter tile). A CPU ring planner (`sf_bigworld`)
   keeps T0 and T1 tiles resident around the camera, with hysteresis. Entering tiles are generated on
   `AsyncComputeTaskPool` through an **`InstanceSource`**: `fn tile(&self, key, tile_m, &mut Vec<Instance>)`, deterministic
   and pure. The adapter implements it, for example over `sf_worldgen::scatter`. Nothing is stored: about 1 ms per tile,
   and about 5 tiles/s while flying at 60 m/s.
2. **Compact pool.** Each tile's instances go into one GPU pool as **16 B instances** (four u32 words): tile-local x
   and z as u16 (4 mm steps over 256 m), height as u16 over the tile's own height range (kept in its header), yaw as u8,
   scale as u8 (log), the up axis as 2 × u8 (octahedral, for props that lean with the slope), species as u16, seed as
   u16, flags as u16. They are uploaded in one write per tile, and a freed tile is one block return. No 10 M array ever
   exists.
3. **Producer** (our compute pass in `GpuInstanceBatchSystems::Publish`, the eidolon pattern):
   - one workgroup per resident tile; one tile-AABB test first;
   - then per instance: tier from distance and screen size, fade, and a frustum test. Visual ranges are seen by the
     camera only (fill rules 1 and 4), so they take the test everywhere. Shadow ranges (the T0 shadow LOD, the T1
     proxy) hold every instance inside `shadow.mesh_range_m`, untested: a tree behind the camera still casts;
   - atomic bump-allocate into the slot span of its (species, tier, LOD level, visual or shadow) range of a
     `GpuBatchedMesh3d`, writing the 64 B slot.
   Inactive slots are written `is_active = 0`. The producer's CPU twin is the same function in `sf_bigworld`; GPU tests
   check them against each other (the groundcover `wgsl_twin` pattern).
4. **Bevy draws.** `materialize`, then the stock per-view preprocess (frustum, hi-Z), then multi-draw indirect: one batch
   per (species, tier) and material, plus the shadow batches on the shadow-only layer. Camera, cascades and lamp faces
   all come for free.
5. **T2** canopy tiles are ordinary `Mesh3d` entities with `VisibilityRange`, a few hundred at most (count in "Cost
   model").

**Capacity** per batch is the tier's budget. When a tier would overflow, distant instances are thinned by a stable hash
(the scatter's budget rule). They fade out; they never go missing in a hole. Overflow is counted
(`bigworld/overflow`).

### Species and assets (bake, once, under a few minutes)

- A species is a model plus its tier data:
  - a LOD chain (meshopt; `simplify_sloppy`/`prune` on leaf clouds);
  - an impostor atlas: `sf_lod::impostor::capture` at 12×12 frames, `impostor.atlas_px` per preset (High 1024²:
    ~85 px frames, enough for an 80 px tree), albedo+alpha and normal+depth in BC7, ~2.8 MB with mips;
  - crown radius, height, pivot, and an average colour for T2.
- Atlases live in one texture array per worldspace.
- **T2 canopy proxy** per tile and quadtree level:
  1. rasterise the tile's crowns (from the `InstanceSource`, the same instances) into a coverage voxel grid at the
     level's error;
  2. remesh it (`meshopt` voxel remesh), then simplify to the far field's one-pixel error budget at the level's nearest
     distance;
  3. store prefiltered vertex albedo, normal and occlusion (Loubet & Neyret; Chlorophyll's filtered voxel opacity and
     occlusion, F21).
  Estimate: about 10 k tiles at 512 m, about 5 ms each, about 5 s on 12 threads, about 25 KB per tile (u16 positions,
  octahedral normals), about 250 MB on disk at 2,500 km². Measured in BW-7. This is the one bake product, and it is
  rebuilt only when the source rules or seed change (content hash, `sf_vfs`).

### Dense statics

Rocks, boulders and repeated props use the same path. A species without an impostor goes T0 then T2, or T0 only, set by
policy. The adapter decides which models are bigworld species and which stay placements (static scene or far field).

### Wind

`sf_wind::sample` drives a height-weighted bend in the T0 vertex shader (the groundcover and UE SimpleGrassWind
pattern). Motion vectors come from wind at t and t-dt. Animation stops below `wind.min_px` (UE
`r.Skinning.DefaultAnimationMinScreenSize`; Chlorophyll F19), so cascades of static impostors and of far trees stay
cacheable.

### Hooks: what other lanes' crates see (none of them change in this lane)

- **Worldgen (world10x lane).** `sf_worldgen::scatter` stays the authority on placement. A new adapter crate,
  `mw_bigworld_bevy` (this lane), implements `InstanceSource` over it for the species the adapter flags as bigworld.
  Those placements are no longer pushed into the in-memory exterior index (one filter in the adapter, BW-9, agreed with
  world10x through meta).
- **Static scene and far field.** Untouched. Bigworld species never reach them. Buildings and hand-placed content keep
  their own paths.
- **Groundcover.** Shares `GpuInstanceBatchPlugin` (both check `is_plugin_added`) and `sf_wind`. Groundcover stays out
  from under trunks through its existing `Overrides` footprints, fed from a `BigworldFootprints` query (the groundcover
  lane wires it when it wants).
- **Physics and nav.** T0 tiles publish `NearInstances` (tile key + instance list, CPU twin). `mw_bigworld_bevy`
  spawns trunk colliders from them. The nav bake reads the same `InstanceSource` for obstacles (nav lane, later).
- **Distant shadows (weather lane).** Optional: T2 canopy heights may feed the far shadow heightfield (a later slice in
  their lane).
- **Saves and net:** presentation only, deterministic from the seed. Felled or removed trees come later as an
  override list (the groundcover `Overrides` pattern).

## Cost model (RTX 4060 laptop, 1440p + DLSS Q, internal 1707×960)

Bottom-up estimates, `Guess` until BW-2/BW-10 measure them. **BW-2 measured the stand column of the slot path on
2026-11-26 (last column; "As built" BW-2 has the table): the stop rule tripped.** The avg column and T2 are still
`Guess`. 4060 laptop: ~14.6 TFLOPS, 256 GB/s, so bookkeeping
passes are bandwidth-bound. Densities: **avg** 60/ha everywhere, **stand** 400/ha everywhere (the worst case).

| Item (High) | avg | stand | Basis | stand, measured (BW-2) |
|---|---|---|---|---|
| T0 trees in ring / drawn (≈¼ in frustum) | 295 / ~75 | 1,960 / ~490 | π·125² m² × density | 1,973 in the ring |
| T1 trees in ring / drawn | 91 k / ~23 k | 608 k / ~150 k | π·(2.2² − 0.125²) km² × density | 179,901 active cards |
| Pool bytes (16 B) / slot bytes (64 B + 144 B materialize tail) | 1.5 MB / 10 MB | 10 MB / 60 MB | 1.6 × drawn slots for headroom | 254,096 slots |
| Producer + materialize (GPU) | 0.05 ms | 0.3 ms | bytes / 256 GB/s, ×1.5 overhead | **0.33 ms** materialize alone (no producer in the bench): 1.3 ns per slot of capacity. Holds |
| Preprocess, camera + 4 cascades (GPU) | 0.05 ms | 0.25 ms | 144 B in + 176 B per visible instance per view | **1.04 ms** (4.2×): 0.8 ns per slot of capacity per view, not per visible instance |
| T0 raster, camera + cascades (avg LOD 1.5 k tris) | 0.1 ms | 0.6 ms | ~1.8 M tris at stand, ~2.5 Gtri/s effective | with the next row: **12.4 ms** (10×) |
| Foliage fragment delta (prepass + main, over the terrain it covers) | 0.2 ms | 0.6 ms | overdraw ~1.5 on ~40 % of the screen; today's opaque+prepasses 2-3.2 ms per screen | prepasses 1.85, cascades 1-3 5.92, main opaque 4.60 ms: far more fill than an overdraw of 1.5 gives, with PBR alpha-masked quads seen at a grazing angle |
| T2 canopy (~360 tiles drawn, ≤ 0.7 M tris) | 0.15 ms | 0.2 ms | 512 m tiles to 8 km, 2 km tiles beyond | not in the bench |
| **GPU total** | **~0.55 ms** | **~1.95 ms** | | **13.7 ms** without T2 (model 1.75); whole-frame GPU delta 13.97 ms |
| Main thread (ring planner, task joins, uploads) | ≤ 0.1 ms | ≤ 0.1 ms | ~5 tiles/s; no per-instance CPU work |
| Render thread (~2 batches per species, ~300 T2 entities) | ≤ 0.15 ms | ≤ 0.2 ms | 16 species ≈ 32 batches × views |
| VRAM (16 species of atlases + LOD chains, pools, T2 resident) | ~150 MB | ~200 MB | atlases ~45 MB, chains ~40 MB, T2 resident ~50 MB |

**Budget (High): ≤ 1.2 ms GPU for typical scenes, ≤ 2.0 ms GPU in worst-case stands, ≤ 0.1 ms main, ≤ 0.2 ms render.**
Groundcover's 0.8 ms is separate. Anchors: Horizon's placement is 0.25 ms; Decima's visibility was 1-2 ms on a GPU
about 6× slower; Chlorophyll's whole dense-forest frame on a 3080 is ~12.5 ms.

| Preset | mesh.switch_px | aggregate.switch_px | T1 end (8 m crown) | atlas | Tree shadows | First model, GPU (avg / stand) |
|---|---|---|---|---|---|---|
| Ultra | 50 | 2 | ~3.3 km | 2048² | shadow LOD + proxy | 0.9 / 3.0 ms |
| High | 80 | 3 | ~2.2 km | 1024² | shadow LOD + proxy | 0.55 / 1.95 ms |
| Medium | 120 | 4 | ~1.7 km | 1024² | shadow LOD only | 0.4 / 1.3 ms |
| Low | 200 | 6 | ~1.1 km | 512² | shadow LOD within 60 m | 0.25 / 0.7 ms |

At Medium and Low, `ppr` drops with the internal resolution and the distances shrink with it.

**Stop rule:** if BW-2 measures more than 2× these numbers for the slot path, stop before BW-3. The fallback is a
standalone indirect pass in the eidolon pattern (own pipeline, our own alpha mask and shadow variants); see "Open
questions".

**Tripped (BW-2, 2026-11-26; BW-2b the same day priced the fixes: "As built" BW-2b):** stand minus empty = 13.97 ms
GPU (14.96 with window focus) against the 3.5 ms limit, so BW-3 does not start and the budget above is not met by
the bench's content. With no alpha test and no cascade casting the same slots cost 1.84 ms: the content assumptions
of the two fill rows were wrong, not the slot path. Two rows broke: preprocess (4.2×, the slot path's own
bookkeeping) and raster + fragment (10×, the fill of alpha-masked quads lit by `StandardMaterial` in the camera and
three cascades). Materialize holds. The preset table above is the unmeasured model. What is known and what is not:
"Risks".

**Expected with the fill rules (stand, High).** Sums of BW-2b's measured spans and of proxies for what no arm ran;
not measured as one arm. The slot path stays, with no standalone pass (BW-2b's recommendation, folded in
2026-11-26).

| Item | GPU ms | Basis |
|---|---|---|
| Bookkeeping, 254 k slots, cards in the camera view only | 0.83 | measured (`stand/cards.noshadow`); the shadow ranges (~31 k trees within 500 m, four views) come on top, not priced |
| T1 camera prepass, masked | ~1.0 | measured (1.00) |
| T1 main pass, depth-equal | ~0.2 | proxy: opaque cards 0.16 (masked 2.73); the real impostor shader costs more per pixel |
| T1 shadow proxy | ~0.3 | proxy: opaque cards' cascade spans 0.29 (masked 1.24) |
| T0 prepass + main, geometry leaves | ~0.4 | measured (`stand/trees.opaque`: 0.29 + 0.10); alpha-card leaves by rule 5 are not priced |
| T0 shadow LOD | 0 to 2.74 | not priced; 2.74 is the full opaque meshes |
| Trimmed hull | about −0.4 on T1 | BW-2b's estimate (T1 2.0 → 1.6), not priced |
| **Total** | **about 2.5-3.3** | BW-2b's estimate: under the 3.5 ms stop limit, over the 2.0 ms budget for worst-case stands |

## Tunables (`tunables.md`; prefix `bigworld.`, group `Group::Foliage`)

All `Source::Guess` unless noted. Presets as in the table above.

| Name | Default (High) | Range | Unit | Source | Lifetime |
|---|---|---|---|---|---|
| `bigworld.tile_m` | 256 | 64-1024 | m | Reference: `sf_worldgen` `scatter.tile_m` | Restart |
| `bigworld.ring.hysteresis_tiles` | 1 | 0-4 | count | Guess | Live |
| `bigworld.mesh.switch_px` | 80 | 20-400 | px | Industry: screen-size LOD, UE/Fortnite | Live |
| `bigworld.aggregate.switch_px` | 3 | 1-16 | px | Industry: Chlorophyll tile proxies past ~1 km, SmartImposter ~3 km | Live |
| `bigworld.fade_band` | 0.1 | 0-0.5 | ratio | Guess | Live |
| `bigworld.density_scale` | 1.0 | 0-2 | ratio | Guess | Live |
| `bigworld.impostor.max_instances` | 250 000 | 1 k-2 M | count | Guess (stand at High + headroom) | Restart |
| `bigworld.mesh.max_instances` | 4 096 | 256-64 k | count | Guess | Restart |
| `bigworld.impostor.atlas_px` | 1024 | 256-4096 | px | Industry: Fortnite 2048², 12×12 | Restart (rebake) |
| `bigworld.impostor.frames` | 12 | 4-16 | count | Industry: Fortnite 12×12 | Restart (rebake) |
| `bigworld.shadow.impostors` | false | bool | - | Ledger 2026-11-26 (BW-2b: masked cards in the cascades 1.24 ms); `true` is the owner's A/B arm | Live |
| `bigworld.shadow.proxy` | true | bool | - | Owner(2026-11-26), BW1 | Live |
| `bigworld.shadow.mesh_lod_bias` | 1 | 0-4 | levels | Guess (HZD: one fixed shadow LOD per cascade) | Live |
| `bigworld.shadow.mesh_range_m` | 500 | 0-2000 | m | Reference: cascade reach today (~504 m, far-field.md) | Live |
| `bigworld.wind.min_px` | 40 | 0-400 | px | Industry: UE `DefaultAnimationMinScreenSize` 0.1 | Live |
| `bigworld.aggregate.error_px` | 1.0 | 0.5-8 | px | Reference: far-field one-pixel budget | Restart (rebake) |
| `bigworld.upload.tiles_per_frame` | 4 | 1-64 | count | Guess | Live |

Not tunables (`const`): the 16 B instance layout, the 64 B slot (vendored), the quantisation steps, and workgroup sizes.

## Counters (`sf_diag`)

`bigworld/tiles/{t0,t1,t2}`, `bigworld/instances/{t0,t1}` (ring), `bigworld/slots/{t0,t1,shadow}` (written),
`bigworld/overflow`, `bigworld/upload_bytes`, `bigworld/tile_gen_ms` (tasks), and GPU passes `bigworld_producer`,
`bigworld_materialize` (the vendored pass, attributed per batch).

## Slices

One deliverable plus its test each. Every slice reports ms per preset where it draws anything. Verification of the look
is the owner's (visual checks `BW-`). Build slices wait for the owner's go (roadmap `[p]`).

| # | Slice | Crates | Test / measurement |
|---|---|---|---|
| BW-0 | This spec + lane roadmap + SYSTEMS row | docs | - |
| BW-1 | `sf_bigworld` core: tile keys, ring planner with hysteresis, tier bands from the px rule, 16 B instance pack/unpack, `InstanceSource` trait + a synthetic jittered-grid source | NEW `sf_bigworld` (Bevy-free, hakari final-excludes) | exact: pack round-trip, band math vs the formula, planner determinism and hysteresis; `#[ignore = "measurement"]` prints ring counts at avg/stand per preset |
| BW-2 | Ceiling bench: example `forest_ceiling`, N slots (10⁵, 3·10⁵, 10⁶) of a 1.5 k-tri alpha-masked tree and of a card, CPU-filled once, camera + 4 cascades | NEW `sf_bigworld_bevy` (example only) | orchestrator runs it, one run per arm: GPU ms of materialize, preprocess per view, prepass, main; decides the stop rule |
| BW-2c | Fill rule 5 on today's impostor: `ImpostorMaterial` gets the depth-equal main pass through `Material::specialize`, the discard stays only for views without a depth prepass | `sf_lod_bevy` (granted by meta 2026-11-26) | pipeline-descriptor test (main: `Equal`, no depth write; prepass and shadow unchanged; no depth prepass: stock); a counter of main-pass draws that no prepass drew under `OcclusionCulling`, expected 0; the far field's impostor visual check re-opened for the owner |
| BW-2d | Bench arms for what BW-2b did not price: shadow LOD, trimmed hull, the depth-equal main pass | `sf_bigworld_bevy` (example only) | orchestrator runs it, one run, on meta's go |
| BW-3 | GPU producer: tile pool upload, per-tile + per-instance tier/fade/frustum, slot writes into ranges | `sf_bigworld_bevy` | GPU twin test vs the `sf_bigworld` CPU function (slot counts and order-free sets); headless timing at 10⁶ pool instances |
| BW-4 | Slot impostor material on a trimmed hull: octahedral 3-frame blend, mask-only prepass, depth-equal main pass (no discard, no `frag_depth`), dither fade from `tag`, casts nothing; atlas capture at `atlas_px` | `sf_bigworld_bevy` (+ `sf_lod::impostor` read-only) | naga compile of every variant; capture test (coverage, depth range); owner look `BW-impostor` (example scene) |
| BW-5 | T0 mesh tier: LOD chain levels as ranges, geometry leaves or leaves masked once (fill rules 3, 5), crossfade with T1, wind in vertex and prepass | `sf_bigworld_bevy` | slot-span test per level; owner look `BW-near` (example scene, walk-in) |
| BW-5b | Shadow ranges on the shadow-only layer: the T0 shadow LOD and the T1 opaque proxy inside `shadow.mesh_range_m` (fill rules 2, 4) | `sf_bigworld_bevy` | slot-span test (every instance inside the range is in exactly one shadow range, none in a camera view); ms per preset; owner look `BW-shadow` (proxy on / off / card shadows A/B) |
| BW-6 | Tile streaming on tasks: generate, upload, evict, overflow thinning | `sf_bigworld_bevy` | headless: flight at 60 m/s over a synthetic world, no frame over 1.5× median from uploads, `tile_gen_ms` printed |
| BW-7 | T2 canopy bake: voxel coverage, remesh, simplify to the error, prefiltered vertex attributes | `sf_bigworld` (feature `bake`) | bake time and bytes per tile printed (measurement); error-bound test vs the input coverage |
| BW-8 | T2 runtime: canopy tile entities, lead-band crossfade with T1, far shadow map via `FarShadowExt` | `sf_bigworld_bevy` (+ `sf_lod_bevy` read-only) | no-hole test (coarser drawn before finer fades); owner look `BW-far` (example flight) |
| BW-9 | Adapter: `InstanceSource` over `sf_worldgen::scatter` for flagged species; trunk colliders from `NearInstances`; register in the game for generated worlds | NEW `mw_bigworld_bevy`; one plugin line in the game app and one filter in the worldgen adapter (meta + world10x OK) | headless: same instances as the scatter for a tile; owner look `BW-x10s` |
| BW-10 | Perf suite on x10s (`perf suite`, one run per arm: bigworld on / off, High and Low) | none | ms per preset into `docs/ledger/bigworld.md` and `perf.md` |

Later (each its own slice, after BW-10): terrain-horizon occlusion of tiles (cinevva 2026; MGE-XE horizon culling),
GPU scatter in the producer (Horizon), canopy into distant shadows (weather lane), groundcover footprints (groundcover
lane), nav obstacles (nav lane), and Morrowind's vanilla trees on this path (a policy choice for the owner).

## As built

Every slice's detail (types and formats, the bench's scene and arms, both measurement tables, caveats) moved verbatim
to [archive/big-world-rendering-history.md](archive/big-world-rendering-history.md) on 2026-11-26 (doc budget). New
slices append their detail there and keep a few lines here.

<!-- agent-summary -->
- **BW-1 `sf_bigworld`** (49b7ae15, 11b0cf8f, 1d134b3d): Bevy-free, one dependency (`sf_tunables`). `TileKey`;
  `PackedInstance` (16 B: position in the tile, height over the tile's range, yaw, log scale, species, seed, flags,
  octahedral up axis); `InstanceSource` and `SyntheticForest`; `Bands` / `TierFades` (the px rule, written to port
  to WGSL one-to-one); the `Ring` planner with hysteresis; five tunables `bigworld.*` (no plugin registers them
  yet). The ring counts match the cost model within 5 %.
- **BW-2 `forest_ceiling`** (e1b8c973, 0c9b0fab, 62d29ecc): `sf_bigworld_bevy` is an empty library plus this
  example: 64 B `GpuMeshInstance` slots filled once on the CPU, the DLSS-path camera at 1707×960, 4 cascades to
  500 m, arms `empty`, `card:<n>`, `tree:<n>` and `stand`. Build:
  `cargo build -p sf_bigworld_bevy --example forest_ceiling`; only the lane's orchestrator launches it. Result:
  `stand` +13.97 ms GPU, so the stop rule tripped. Materialize 1.2-1.4 ns per slot of capacity, linear to 10⁶;
  preprocess ~0.8 ns per slot of capacity per view; no VRAM fault at 10⁶ slots. Run lines:
  `bevyrrowind/benchmarks/runs/bw2_ceiling/run.log`.
- **BW-2b `stand` variants** (04d2f7f6): `stand[/trees[.mod..]][/cards[.mod..]]` with `.unlit`, `.opaque` and
  `.noshadow`; run lines in `run2b.log` beside the first. GPU ms minus `empty`, focus kept: stand 14.96; trees
  alone 7.58, opaque 3.44 (2.74 of it three cascades), no shadow 3.07; cards alone 6.25, no shadow 4.62, unlit and
  no shadow 3.46, opaque 1.80. Opaque and casting nothing, both batches: 1.84 ms (a sum of spans; model 1.75).
  Bookkeeping 0.83-1.25 ms (model 0.55). Not priced by an arm: a trimmed hull, a masked prepass with a depth-equal
  main pass, a shadow LOD, the real impostor shader.
- **BW-2c depth-equal impostor main pass** (`sf_lod_bevy/src/depth_equal.rs`): today's far-field
  `ImpostorMaterial` follows fill rule 5. `depth_equal_main_pass` runs in `Material::specialize` and switches only
  a main-pass descriptor of a view with a depth prepass; `impostor.wesl` drops its alpha test and range dither
  under `DEPTH_EQUAL_MAIN`. Switch `far.impostor.depth_equal` (default on, live). Counter
  `far/impostors/no_prepass` (expected 0). No ms measured.
<!-- /agent-summary -->

## Risks and open questions

- **Slot path ceiling: the path holds, the content did not** (BW-2, BW-2b, 2026-11-26). The stop rule tripped on
  `stand` (14.96 ms with focus). BW-2b shows why: with no alpha test and nothing cast into the cascades the same
  254 k slots cost 1.84 ms (model 1.75); bookkeeping is 0.83-1.25 ms (model 0.55). The standalone indirect pass
  (own alpha-mask and shadow pipelines, a rule 4 cost) would buy back part of that bookkeeping only, so the slot
  path stays, with the "Fill rules". BW-3 and later still wait for the owner's go.
- **The fill rules are over budget at stand density, and half of them are not priced.** Expected 2.5-3.3 ms against
  a 2.0 ms budget ("Cost model"). No arm ran the shadow LOD, the trimmed hull, the depth-equal main pass, the real
  impostor shader or the shadow ranges' bookkeeping: BW-2d prices the first three.
- **Alpha-card tree models.** Fill rule 3 needs geometry leaves. The 10x world starts with Morrowind's own tree
  models (roadmap, owner answer 2), which have alpha-card leaves: they take rule 5 (the bench trees cost 3.07 ms
  masked without shadows against 0.69 ms opaque; rule 5 should remove most of the 2.15 ms main pass, not the
  0.78 ms prepass). Not priced.
- **Depth-equal main pass** ([research note, part 3](../research/far-vegetation-fill-2026-11-26.md)). Traced
  in BW-2c: Bevy culls each render phase on the GPU with the same shader on the same inputs, so with
  `OcclusionCulling` the main pass cannot draw an instance its prepass phase culled. The two can differ only on the
  CPU, while a part's main-pass pipeline is ready and its prepass pipeline is missing or still compiling: the part
  is then missing for those frames (`far/impostors/no_prepass`, expected 0). The impostor loses per-pixel depth on the camera path (no
  `frag_depth`): intersections with terrain and buildings are those of a flat card past ~125 m.
- **Shadow-only ranges.** Not checked in code: that a batch on the shadow-only layer costs no preprocess in the
  camera view, and how lamp (point and spot) views treat that layer. Check in BW-5b.
- **The look of proxy shadows** is the owner's to judge (`BW-shadow`: proxy on, off, and the cards' own masked
  shadows as the A/B). The proxy's shape per species (a low-poly crown) is content, baked with the species.
- **Render-thread CPU per slot** (BW-2, dev profile, not conclusive): render busy rose 0.5 ms at 10⁵ slots and
  2.3 ms at 10⁶ (model ≤ 0.2 ms). Needs a release run.
- **Meshlets for T2 only.** The canopy is opaque, which is the case where Bevy meshlets work. Re-check at BW-7: the DX12
  vs Vulkan backend, the per-entity upload (~300 entities is fine), and open flicker issues (#26467).
- **Impostor lighting:** match T0 at the crossfade (normal + depth frames lit through `apply_pbr_lighting`, as
  `ImpostorMaterial` does). The owner judges it (BW-impostor).
- **Storage limits:** raise `max_storage_buffer_binding_size` at device creation (app-level, BW-3). Check the
  adapter's limits on non-NVIDIA hardware.
