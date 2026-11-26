# Big-world rendering: millions of trees and dense scatter (`sf_bigworld`, `sf_bigworld_bevy`)

Status: **BW-1 built** (the Bevy-free core `sf_bigworld`, 2026-11-26; "As built"); nothing draws yet. Owner go for
Stage A (BW-1, BW-2) 2026-11-26. Lane roadmap: [docs/roadmaps/bigworld.md](../../../docs/roadmaps/bigworld.md)
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
| T0 mesh | height ≥ `mesh.switch_px` (80 px) | 0 → ~125 m | cooked LOD chain (meshopt), alpha-tested leaves, wind | casts in every cascade it reaches; wind only inside T0 |
| T1 impostor | crown width ≥ `aggregate.switch_px` (3 px) | ~125 m → ~2.2 km | octahedral impostor, one card per tree, 12×12 frames, depth + normal, 3-frame blend, dither fade | casts with impostor depth inside the cascade range (~500 m); none past it |
| T2 aggregate | below that | ~2.2 km → horizon | per-tile canopy proxy (opaque, baked), quadtree levels by pixel error | receives the terrain far shadow map; casts nothing |

Rules:
- Neighbouring tiers overlap in a dithered crossfade band (`fade_band`). Inside the band an instance is written to both
  tiers, with its fade in the slot's `tag`. That is the same no-hole rule as the far field's lead bands.
- T2 tiles keep the far field's `VisibilityRange` lead bands, so a coarser level is drawn fully before a finer one
  fades out.

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
   - then per instance: tier from distance and screen size, fade, and a frustum test **only past the shadow range**,
     because nothing past it can cast into a cascade;
   - atomic bump-allocate into the slot span of its (species, tier, LOD level) range of a `GpuBatchedMesh3d`, writing
     the 64 B slot.
   Inactive slots are written `is_active = 0`. The producer's CPU twin is the same function in `sf_bigworld`; GPU tests
   check them against each other (the groundcover `wgsl_twin` pattern).
4. **Bevy draws.** `materialize`, then the stock per-view preprocess (frustum, hi-Z), then multi-draw indirect: one batch
   per (species, tier) and material. Camera, cascades and lamp faces all come for free.
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

Bottom-up estimates, all `Guess` until BW-2/BW-10 measure them. 4060 laptop: ~14.6 TFLOPS, 256 GB/s, so bookkeeping
passes are bandwidth-bound. Densities: **avg** 60/ha everywhere, **stand** 400/ha everywhere (the worst case).

| Item (High) | avg | stand | Basis |
|---|---|---|---|
| T0 trees in ring / drawn (≈¼ in frustum) | 295 / ~75 | 1,960 / ~490 | π·125² m² × density |
| T1 trees in ring / drawn | 91 k / ~23 k | 608 k / ~150 k | π·(2.2² − 0.125²) km² × density |
| Pool bytes (16 B) / slot bytes (64 B + 144 B materialize tail) | 1.5 MB / 10 MB | 10 MB / 60 MB | 1.6 × drawn slots for headroom |
| Producer + materialize (GPU) | 0.05 ms | 0.3 ms | bytes / 256 GB/s, ×1.5 overhead |
| Preprocess, camera + 4 cascades (GPU) | 0.05 ms | 0.25 ms | 144 B in + 176 B per visible instance per view |
| T0 raster, camera + cascades (avg LOD 1.5 k tris) | 0.1 ms | 0.6 ms | ~1.8 M tris at stand, ~2.5 Gtri/s effective |
| Foliage fragment delta (prepass + main, over the terrain it covers) | 0.2 ms | 0.6 ms | overdraw ~1.5 on ~40 % of the screen; today's opaque+prepasses 2-3.2 ms per screen |
| T2 canopy (~360 tiles drawn, ≤ 0.7 M tris) | 0.15 ms | 0.2 ms | 512 m tiles to 8 km, 2 km tiles beyond |
| **GPU total** | **~0.55 ms** | **~1.95 ms** | |
| Main thread (ring planner, task joins, uploads) | ≤ 0.1 ms | ≤ 0.1 ms | ~5 tiles/s; no per-instance CPU work |
| Render thread (~2 batches per species, ~300 T2 entities) | ≤ 0.15 ms | ≤ 0.2 ms | 16 species ≈ 32 batches × views |
| VRAM (16 species of atlases + LOD chains, pools, T2 resident) | ~150 MB | ~200 MB | atlases ~45 MB, chains ~40 MB, T2 resident ~50 MB |

**Budget (High): ≤ 1.2 ms GPU for typical scenes, ≤ 2.0 ms GPU in worst-case stands, ≤ 0.1 ms main, ≤ 0.2 ms render.**
Groundcover's 0.8 ms is separate. Anchors: Horizon's placement is 0.25 ms; Decima's visibility was 1-2 ms on a GPU
about 6× slower; Chlorophyll's whole dense-forest frame on a 3080 is ~12.5 ms.

| Preset | mesh.switch_px | aggregate.switch_px | T1 end (8 m crown) | atlas | T0 shadows | Expected GPU (avg / stand) |
|---|---|---|---|---|---|---|
| Ultra | 50 | 2 | ~3.3 km | 2048² | T0 + T1 | 0.9 / 3.0 ms |
| High | 80 | 3 | ~2.2 km | 1024² | T0 + T1 | 0.55 / 1.95 ms |
| Medium | 120 | 4 | ~1.7 km | 1024² | T0 only | 0.4 / 1.3 ms |
| Low | 200 | 6 | ~1.1 km | 512² | T0 within 60 m | 0.25 / 0.7 ms |

At Medium and Low, `ppr` drops with the internal resolution and the distances shrink with it.

**Stop rule:** if BW-2 measures more than 2× these numbers for the slot path, stop before BW-3. The fallback is a
standalone indirect pass in the eidolon pattern (own pipeline, our own alpha mask and shadow variants); see "Open
questions".

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
| `bigworld.shadow.impostors` | true | bool | - | Guess | Live |
| `bigworld.shadow.mesh_range_m` | 500 | 0-2000 | m | Reference: cascade reach today (~504 m, far-field.md) | Live |
| `bigworld.wind.min_px` | 40 | 0-400 | px | Industry: UE `DefaultAnimationMinScreenSize` 0.1 | Live |
| `bigworld.aggregate.error_px` | 1.0 | 0.5-8 | px | Reference: far-field one-pixel budget | Restart (rebake) |
| `bigworld.upload.tiles_per_frame` | 4 | 1-64 | count | Guess | Live |

Not tunables (`const`): the 16 B instance layout, the 64 B slot (vendored), the quantisation steps, and workgroup sizes.

## Counters (`sf_diag`)

`bigworld/tiles/{t0,t1,t2}`, `bigworld/instances/{t0,t1}` (ring), `bigworld/slots/{t0,t1}` (written),
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
| BW-3 | GPU producer: tile pool upload, per-tile + per-instance tier/fade/frustum, slot writes into ranges | `sf_bigworld_bevy` | GPU twin test vs the `sf_bigworld` CPU function (slot counts and order-free sets); headless timing at 10⁶ pool instances |
| BW-4 | Slot impostor material: octahedral 3-frame blend, depth, prepass/shadow variant, dither fade from `tag`; atlas capture at `atlas_px` | `sf_bigworld_bevy` (+ `sf_lod::impostor` read-only) | naga compile of every variant; capture test (coverage, depth range); owner look `BW-impostor` (example scene) |
| BW-5 | T0 mesh tier: LOD chain levels as ranges, crossfade with T1, wind in vertex and prepass | `sf_bigworld_bevy` | slot-span test per level; owner look `BW-near` (example scene, walk-in) |
| BW-6 | Tile streaming on tasks: generate, upload, evict, overflow thinning | `sf_bigworld_bevy` | headless: flight at 60 m/s over a synthetic world, no frame over 1.5× median from uploads, `tile_gen_ms` printed |
| BW-7 | T2 canopy bake: voxel coverage, remesh, simplify to the error, prefiltered vertex attributes | `sf_bigworld` (feature `bake`) | bake time and bytes per tile printed (measurement); error-bound test vs the input coverage |
| BW-8 | T2 runtime: canopy tile entities, lead-band crossfade with T1, far shadow map via `FarShadowExt` | `sf_bigworld_bevy` (+ `sf_lod_bevy` read-only) | no-hole test (coarser drawn before finer fades); owner look `BW-far` (example flight) |
| BW-9 | Adapter: `InstanceSource` over `sf_worldgen::scatter` for flagged species; trunk colliders from `NearInstances`; register in the game for generated worlds | NEW `mw_bigworld_bevy`; one plugin line in the game app and one filter in the worldgen adapter (meta + world10x OK) | headless: same instances as the scatter for a tile; owner look `BW-x10s` |
| BW-10 | Perf suite on x10s (`perf suite`, one run per arm: bigworld on / off, High and Low) | none | ms per preset into `docs/ledger/bigworld.md` and `perf.md` |

Later (each its own slice, after BW-10): terrain-horizon occlusion of tiles (cinevva 2026; MGE-XE horizon culling),
GPU scatter in the producer (Horizon), canopy into distant shadows (weather lane), groundcover footprints (groundcover
lane), nav obstacles (nav lane), and Morrowind's vanilla trees on this path (a policy choice for the owner).

## As built

### BW-1: `sf_bigworld` core (2026-11-26; 49b7ae15, 11b0cf8f, 1d134b3d)

Bevy-free, one dependency (`sf_tunables`). Frame: metres, x and z on the ground, y up. 27 exact tests plus one
measurement test.

- **Tiles** (`tile.rs`): `TileKey { x: i32, z: i32 }`, `containing`, `min_corner`, `distance_to` (0 inside the square).
- **Instance format** (`instance.rs`): `PackedInstance` = four little-endian u32 words.
  - `w0` = x | z << 16, each `round(v / tile_m × 65535)`.
  - `w1` = y | yaw << 16 | scale << 24. Height is quantised over the tile's `[y_min, y_max]`, kept in `TileHeader`, so a
    tile's step is its relief / 65535. Yaw has 256 steps. Scale is logarithmic over [0.25, 4] (about 1.1 % steps).
  - `w2` = species | seed << 16.
  - `w3` = flags | up << 16: the up axis, octahedral with +y as the pole (Cigolle et al. 2014), 2 × u8, about 1° of
    error; upright is exact.
  - `PackedTile::pack` / `unpack` / `world_position` / `write_le_bytes`. Packing an unpacked tile again gives the same
    bits. A point on a shared tile edge can decode to exactly `tile_m`.
- **Sources** (`source.rs`): `InstanceSource::tile(key, tile_m, out)` (`Send + Sync`, pure). `SyntheticForest` is a
  jittered grid hashed per tile with SplitMix64 (`hash.rs`), with optional sine relief: for tests, BW-2 and example
  scenes.
- **Tier bands** (`bands.rs`): `ViewScale` (pixels per radian), `SpeciesDims` (height, crown), `Bands::new` gives
  `mesh_end_m = height · ppr / mesh.switch_px` and `impostor_end_m = crown · ppr / aggregate.switch_px` (never below the
  mesh end). `Bands::fades(dist, scale)` returns `TierFades { mesh, impostor }` with `m` and `i` the two ends times the
  instance's scale and `b` = `fade_band`:
  - mesh falls from 1 to 0 over `[m(1−b), m]`; the impostor rises over the same band, so the two always sum to 1 there;
  - the impostor falls from 1 to 0 over `[i(1−b), i]`. The T2 tile must be fully drawn from `aggregate_lead_m = i(1−b)`
    (BW-8 owns that); when a species' two bands touch, this is what keeps it from showing a hole;
  - `b = 0` is a hard switch. Bad inputs (NaN, zero or negative sizes) never give NaN.
  The maths is f32 saturate-style, written to port to the producer's WGSL one-to-one (BW-3).
- **Ring planner** (`ring.rs`): `Ring::update(viewer, &RingSettings) -> RingDelta`. A tile enters when the viewer is
  within `radius_m` of its square and leaves past `radius_m + hysteresis_tiles × tile_m`; optional world `bounds`.
  `enter` is sorted nearest first (the upload order). `clear` is for teleports and for a changed `tile_m`. The renderer
  keeps two rings, from `ring_radii(bands, max_scale)`: the impostor radius (the GPU pool) and the mesh radius
  (`NearInstances`).
- **Tunables** (`tunables.rs`, `Group::Foliage`): `bigworld.tile_m`, `bigworld.ring.hysteresis_tiles`,
  `bigworld.mesh.switch_px`, `bigworld.aggregate.switch_px`, `bigworld.fade_band`. No plugin registers them until
  `sf_bigworld_bevy` exists, so `docs/tunables-index.md` does not list them yet.
- **Measured** (`measure_ring_counts`, release, one thread, ppr 831, one 12 m species with an 8 m crown; ledger
  2026-11-26). The ring counts match the cost model within 5 %:

  | Preset | Density | Tiles | Instances resident (pool) | In the mesh tier | In the impostor tier |
  |---|---|---|---|---|---|
  | High | 60/ha | 276 | 110 k (1.7 MB) | 298 | 95 k |
  | High | 400/ha | 276 | 718 k (11.0 MB) | 1,962 | 618 k |
  | Ultra | 400/ha | 568 | 1.48 M (22.5 MB) | 5,018 | 1.39 M |
  | Low | 400/ha | 80 | 208 k (3.2 MB) | 318 | 155 k |

  The synthetic source generates and packs a tile in 0.02 ms at 60/ha and 0.12 ms at 400/ha. A real scatter with
  terrain queries will cost more; BW-6 measures it.

### BW-2: ceiling bench `forest_ceiling` (2026-11-26; e1b8c973, 0c9b0fab, 62d29ecc)

`sf_bigworld_bevy` exists with an empty library: its one target is the example
`superfargoth/crates/sf_bigworld_bevy/examples/forest_ceiling/` (24 unit tests in the example harness, `test = true`).
The plugin arrives with BW-3. No new Bevy feature set (group [1] of `tools/bevy_variants.sh`).

- **Scene.** A flat `SyntheticForest` at 400 trees/ha around the origin. Camera: the game's DLSS-path camera (`Hdr`,
  no MSAA, depth and motion-vector prepass, `OcclusionCulling`) in a 1707×960 window, fov 60°, 60 m up and pitched 10°
  down, so nothing hides the forest (the hard case). Sun at 45° with 4 cascades to 500 m, 2048 px. One ground plane.
- **Meshes** (`meshes.rs`): a 12 m tree with an 8 m crown, 1,464 triangles (trunk plus 700 alpha-masked leaf quads); a
  2-triangle 8 × 12 m card turned to the camera on the CPU (a stand-in for the impostor billboard); one procedural mask
  texture, one `StandardMaterial` with `AlphaMode::Mask`.
- **Slots** (`fill.rs`, `upload.rs`): 64 B `GpuMeshInstance` slots built once on the CPU and written with
  `RenderQueue::write_buffer` in `GpuInstanceBatchSystems::Publish` (the groundcover writer's pattern, runs split at
  16 MB). Inactive slots are zeroed.
- **Arms** (`--arms`, default all eight, one launch):
  - `empty`: the bare scene, the baseline to subtract;
  - `card:<n>` and `tree:<n>` at 10⁵, 3·10⁵ and 10⁶: the n trees nearest the viewer in one batch of capacity n, all
    active (10⁶ = a 2.8 km disc);
  - `stand`: the cost model's worst case as the producer will write it, from `sf_bigworld::Bands` at the High defaults
    (mesh end 124.7 m, impostor end 2,217 m, horizontal distance): 1,973 trees in a 4,096-slot batch, and 179,901 cards
    in a 250,000-slot batch (every tree with an impostor fade that is inside the 500 m shadow range or the camera
    frustum). The model's counts are 1,960 and about 180 k.
- **Measurement** (`bench.rs`, `stats.rs`): per arm Settle (old batches gone, next arm built) → Warm (4 s) → Measure
  (12 s and 120 frames, capped at 60 s), a 9 min watchdog. It prints `bw2 ceiling | ...` lines: frame median and p99,
  main and render thread ms (`FramePhasePlugin`), the GPU total over top-level spans, every GPU span's median and p95,
  frames without window focus, and a warning when a batch's per-view buffer would pass a wgpu limit.
- **Run it:** `target/debug/examples/forest_ceiling.exe` with no arguments (dev profile: dependencies are optimised),
  built by `cargo build -p sf_bigworld_bevy --example forest_ceiling`. Only the lane's orchestrator launches it.
- **Adapter facts** (smoke launch): RTX 4060 Laptop on Vulkan reports `max_storage_buffer_binding_size` 2 GB and
  `max_buffer_size` 4 GB, so a 10⁶-slot batch (about 176 MB per view) needs no raised limit there.

## Risks and open questions

- **Slot path ceiling** (BW-2). Capacity-linear `materialize` and the preprocess over every view could cost more than
  the model says. The fallback is a standalone indirect pass, which means our own alpha-mask and shadow pipelines, a
  rule 4 cost.
- **Alpha overdraw** in dense near crowns. If BW-2 shows main and prepass over 0.6 ms at stand density, there are two
  options: geometry leaves at T0 (Chlorophyll F17: trees fully triangles, no alpha) or hashed alpha (Wyman & McGuire
  2017).
- **Meshlets for T2 only.** The canopy is opaque, which is the case where Bevy meshlets work. Re-check at BW-7: the DX12
  vs Vulkan backend, the per-entity upload (~300 entities is fine), and open flicker issues (#26467).
- **Impostor lighting:** match T0 at the crossfade (normal + depth frames lit through `apply_pbr_lighting`, as
  `ImpostorMaterial` does). The owner judges it (BW-impostor).
- **Storage limits:** raise `max_storage_buffer_binding_size` at device creation (app-level, BW-3). Check the
  adapter's limits on non-NVIDIA hardware.
