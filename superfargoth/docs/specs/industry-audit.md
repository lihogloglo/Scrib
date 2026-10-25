# Industry gap audit: streaming, rendering, LOD

Status: audit complete (2026-10-12), research only, nothing built. Owners
of each gap are named in the table; jobs J1–J9 are in the roadmap's "Next
up" (P0 and P1); notes added to streaming.md and far-field.md; ARCHITECTURE
§10 "process: industry gap audit"; every spec now carries an "Industry
comparison" section (specs README).
Crates touched by the jobs: `sf_model` (lod.rs), `sf_world`, `sf_world_bevy`,
`sf_lod`/`sf_lod_bevy`, the app's presets.

## Why this exists

The owner pointed out a basic technique our specs missed: **per-object draw
distance from screen size**. We spawn and draw every cup out to the far
field's handoff `d1` (400 m), where it is a third of a pixel wide. Every
shipped open-world engine culls small objects much earlier: UE cull distance
volumes and per-primitive max draw distance, Unity LODGroup's "Culled" state,
Bethesda's fade distances (a separate one for "items"), RAGE's per-entity
`lodDist`, OpenMW's small-feature culling. Our specs carried screen-size
*LOD selection* (bake.md) but never screen-size *culling*, and no spec had
to say what the big engines do for its system. This audit compares the whole
pipeline system by system and adds that rule to the spec template
(README.md "Industry comparison").

## How Bevy 0.20 actually spends per entity (read from source)

Read from `bevy_camera-0.20.0`, `bevy_light-0.20.0`, `bevy_render-0.20.0` and
`bevy_pbr-0.20.0` in the local cargo registry. N = resident mesh entities
(Balmora: ~48k, each placement spawns one entity per LOD level × material
part). V = views (the camera, plus 4 sun cascades).

**Main world, `PostUpdate`, every frame (CPU culling, our current path):**

| system | work | scales with |
|---|---|---|
| `check_visibility_ranges` (camera/visibility/range.rs) | distance test per entity per view (cameras + `ShadowLodOrigin`, ≤ 32 views), result pushed into a hash map | N_ranged × views |
| `reset_view_visibility`, `mark_newly_hidden_entities_invisible` | a pass over every `ViewVisibility` each | 2 × N |
| `check_visibility_cpu_culling` | per active camera: inherited visibility, layers, range hash lookup, sphere then OBB frustum test, then collect and **sort** the visible list | N per camera |
| `check_dir_light_mesh_visibility` (light/lib.rs) | per directional light per camera: range lookup, then an OBB test per cascade (near plane off), lists per cascade | N_casters × 4 cascades |
| `calculate_bounds` | skipped for our statics (`NoAutoAabb`) | 0 |

**Extraction and render world:**

- The camera's and every cascade's `VisibleEntities` list is copied into the
  render world and diffed each frame (`RenderVisibleEntitiesClass::update`,
  sorted O(visible) per view).
- `extract_meshes_for_gpu_building` is change-driven: `Changed<ViewVisibility>`,
  `GlobalTransform`, `Aabb`, `Mesh3d`, `VisibilityRange`, `RenderLayers`, and so
  on. So **turning the camera re-extracts every entity that enters or leaves the
  frustum**, and an origin shift re-extracts every instance (all
  `GlobalTransform`s change).
- `extract_visibility_ranges`: any added, changed or removed range clears the
  table and re-inserts **every** ranged entity (~1.1 ms at ~40k, measured).
  Ranges are deduplicated into one GPU buffer, but ours are mostly unique
  (per-placement scale), so deduplication saves little.
- Queue and specialize are incremental (`DirtySpecializations`, retained bins:
  only added and removed entities are touched).
- Prepare: retained multidraw batch sets keep their GPU buffers; a bin-unpacking
  compute shader builds the preprocess work items. An unchanged instance costs
  almost no render-thread CPU.
- Mesh allocator: `allocate_and_free_meshes` allocates and copies every mesh
  extracted that frame (the 2–7 ms upload bursts). `RenderAssetBytesPerFrame`
  only defers `prepare_assets`, which created the stale-instance pitfall
  (far-field.md).
- Pipelines compile asynchronously (an object without a ready pipeline is
  skipped, not stalled); there is **no on-disk pipeline cache** (no use of
  wgpu's `PipelineCache` in bevy_render 0.20).

**GPU, per instance per view per frame (`mesh_preprocess.wesl`):** transform,
OBB frustum cull, **`VisibilityRange` cull** (`camera_distance < range.x ||
>= range.w`, measured from `lod_view_world_position`, which is the camera or
`ShadowLodOrigin` for shadow views), two-phase HZB occlusion when enabled,
then the indirect arguments. Draws are multi-draw indirect per batch set;
bindless materials let one batch set span materials; each bin is one mesh, so
**the same mesh and material are instanced automatically** (one indirect draw,
N instances). The crossfade dither runs in the fragment shader.

**`NoCpuCulling` (per entity or per camera):**

- The entity skips all four CPU passes above. `check_visibility_gpu_culling`
  sets `ViewVisibility = InheritedVisibility`, and only when that changes.
- Render side, it lives in `RenderGpuCulledEntities` and joins every view's
  `entities_gpu_culling` table with matching layers (camera **and every shadow
  cascade**) incrementally: only added and removed entities
  (`collect_gpu_culled_meshes`).
- **`VisibilityRange` keeps working.** The CPU never tests the range (the entity
  is not in `VisibleEntityRanges`), but the GPU preprocess shader culls by it in
  every view, cascades included, and the dither still fades the margins. LOD
  switching, far-field lead bands and size culling all survive.
- What it costs: every resident instance goes through the GPU preprocess in
  every view each frame (~48k × 5 ≈ 240k threads; expected well under 0.1 ms,
  to be measured). Indirect buffers are sized by resident count, not visible
  count. `ViewVisibility` no longer means "in the frustum", so anything that
  reads it must stay on CPU culling (the particle pause in `mw_assets` does).
  Gizmos and picking that walk `VisibleEntities` don't see these entities.
- What it doesn't remove: re-collection at origin shifts, the full
  `extract_visibility_ranges` rebuild, and the per-entity ECS and memory cost.
  Only fewer entities removes those, which is what size culling and
  precombining do.

**Consequence.** Our per-frame CPU visibility cost is O(resident entities ×
views), and resident entities include every coarse LOD level of every cup out
to 400 m. Two independent levers: **fewer entities** (size-based draw
distance, not spawning what can't be seen, precombining) and **GPU culling for
what stays** (`NoCpuCulling`). Bevy has no screen-size culling of its own
(nothing in `bevy_camera`/`bevy_pbr`/`bevy_render` 0.20 outside meshlets), so
size culling is ours, expressed through `VisibilityRange`.

## The audit

Status: **have**, **partial**, **missing**, **n/a** (not applicable to
Morrowind content or our scope). Priority: **P0** now, **P1** within M2,
**P2** later milestones. Gains are estimates for Morrowind exteriors on the
RTX 4060 Laptop unless marked measured.

| # | Technique | Industry | Ours | Expected gain (Morrowind) | Cost | Pri | Owner |
|---|---|---|---|---|---|---|---|
| 1 | **Per-object max draw distance from screen size** (cull below N px, with a fade) | UE per-primitive Max Draw Distance + Cull Distance Volumes (size → distance pairs); Unity LODGroup "Culled"; Bethesda fade multipliers per category (objects/items/actors); RAGE per-entity `lodDist`; OpenMW small-feature culling (2 px) and object paging min size (size/distance 0.01) | **missing**: everything ends at `d1` or R1 | Large in towns: small props drop from 400 m to 30–150 m (formula below). Entities, instances and CPU visibility in proportion; near-object GPU (~2.2 ms of main pass in Balmora) partly. Histogram first (J1). | S: `sf_model::lod` + `SpawnCx` | **P0** | streaming.md, bake.md; `sf_model`, `sf_world_bevy` |
| 2 | **Don't load what can't be drawn** (spawn distance = draw distance, size-tiered streaming) | RAGE `lodDist` is the unload distance; UE World Partition runtime grids each have their own loading range (small actors on a short-range grid); OpenMW pages objects by size | **missing**: a cell at R1 spawns every placement | Fewer entities, spawn/despawn work and uploads per frame; small-object bursts disappear from far cells. | M: scheduler sub-levels | **P0** | streaming.md; `sf_world`, `sf_world_bevy` |
| 3 | **GPU-only culling for statics** | Unity GPU Resident Drawer (BRG, GPU occlusion); UE GPUScene + GPU instance culling; AC Unity (Haar/Aaltonen 2015); Frostbite (Wihlidal 2016) | **partial**: Bevy GPU preprocessing culls too, but every entity is also CPU-culled per view and per cascade | Removes O(N × views) CPU per frame and the camera-rotation re-extraction; render-thread lists stop being copied and diffed per cascade. | S: one component + A/B | **P0** | streaming.md; `sf_world_bevy`, `sf_lod_bevy` |
| 4 | Screen-size LOD selection | universal (UE screen size, Unity screen-relative height, 3D Tiles SSE) | **have** (1 px at 1440p/70° × bias, bake.md) | | | | bake.md |
| 5 | Dithered LOD crossfade | UE dithered LOD transition, Unity LODGroup cross-fade | **have** (`VisibilityRange` margins; lead bands) | | | | far-field.md |
| 6 | Quality presets scale distances (one bias) | UE `r.ViewDistanceScale`, scalability groups; Bethesda fade sliders | **partial**: `LodScreen::bias` exists, presets not wired (Next up 4) | Low preset headroom on weaker GPUs | S | P1 (with J1) | streaming.md, app |
| 7 | HLOD (merged + simplified clusters past a handoff) | UE World Partition HLOD layers; Bethesda object LOD; OpenMW object paging; GTA SLOD | **have** (L1–L3, measured) | | | | far-field.md |
| 8 | Instanced impostors for trees | UE HLOD Instancing layer + impostors; SpeedTree billboards | **have** (octahedral) | | | | far-field.md |
| 9 | Distant light sprites | Bethesda/RAGE distant light LOD, OpenMW none | **have** | | | | far-field.md |
| 10 | **Precombined near geometry** (merge non-interactive statics per cell) | Fallout 4/76 precombines; OpenMW "object paging active grid" (merges active cells too, "significantly improves framerate when CPU-limited") | **missing**: near statics are one entity per object per LOD per part | Biggest CPU entity cut after size culling; fewer instances for the GPU. Costs culling granularity, per-object LOD and memory (unique geometry instead of instances). Decide on J1–J3 numbers. | L: bake step + spawner | P1 | far-field.md (L0.5), bake.md; `sf_lod` |
| 11 | Automatic instancing of repeated meshes | UE 4.22+ auto-instancing; Unity SRP batcher / GPU Resident Drawer | **have** (Bevy bins: same mesh + material = one indirect draw; MDI; bindless) | | | | Bevy |
| 12 | Instanced repeated props without per-instance entities (ISM/HISM, foliage instancer) | UE ISM/HISM/foliage with per-instance cull distances; Unity `DrawMeshInstancedIndirect`/BRG | **missing** near; **have** for impostors | Only if J2/J3 leave ECS cost dominant (rocks, flora, kollops repeat a lot) | M: custom instanced material per (cell, model) | P2 | far-field.md; `sf_lod_bevy` |
| 13 | **Shadow caster culling by size and distance** | UE `r.Shadow.RadiusThreshold` (casters below a screen size skipped), per-primitive cast-shadow distance; Unity per-renderer shadow casting + LOD bias; virtual/cached shadow maps | **missing**: every resident static casts into all 4 cascades to 504 m | CPU cascade tests (no longer needed with #3); GPU shadow-pass draws: small casters cover < 1 texel in cascades 3–4 | S–M: size rule → `NotShadowCaster` or a shadow-only range | P1 | weather/distant-shadows.md (distant shadows), sky-lighting.md |
| 14 | Static shadow caching | UE virtual shadow map page caching; UE4 cached far cascades; HDRP cached shadows | **missing** (Bevy re-renders cascades every frame) | Shadow pass is large in vistas; static geometry dominates | L (renderer work, upstream candidate) | P2 | weather.md |
| 15 | Occlusion culling | Bethesda previs (precomputed); Frostbite software occlusion (Collin 2011); UE HZB + software occlusion; Unity GPU occlusion | **partial**: Bevy two-phase HZB exists but needs the depth prepass, which breaks the dither (pitfall 2); measured slower in Balmora (13.9 vs 11.9 ms) | Dense towns (Vivec cantons, Mournhold), interiors; terrain hides most of the island from the ground | S to patch (two lines, sky-lighting.md), M to evaluate | P1 | sky-lighting.md, far-field.md |
| 16 | Terrain/horizon occlusion of cells and far nodes | common in terrain engines (horizon culling, Far Cry/RAGE height-based occlusion) | **missing** | Far nodes behind Red Mountain or hills skip drawing | M | P2 | terrain.md, far-field.md |
| 17 | Front-to-back opaque order or depth prepass to cut overdraw | UE depth prepass (default), Frostbite/Decima prepass or sorted opaque | **missing** in exteriors: no prepass (pitfall 2), and Bevy's binned opaque phase is unsorted | Terrain is 2.9 ms of main pass; overdraw of expensive splat shading | depends on #15 fix | P1 (with #15) | sky-lighting.md, terrain.md |
| 18 | World partition grid, rings, hysteresis, velocity prefetch | UE World Partition; Spider-Man/RAGE/Decima cell streaming | **have** | | | | streaming.md |
| 19 | Time-budgeted integration (ms per frame) | UE `s.LevelStreamingActorsUpdateTimeLimit`; Unity `backgroundLoadingPriority` | **have** | | | | streaming.md |
| 20 | Upload pacing by bytes per frame | UE texture/mesh streaming pools with per-frame budgets; Unity async upload time slice + buffer size | **partial**: Bevy limiter off (pitfall 1), load-side mesh budget in progress (spikes3) | the 2–7 ms burst frames | in progress | P0 (running) | streaming.md |
| 21 | Burst loading behind a fade | universal (loading screens, fast travel) | **have** | | | | streaming.md |
| 22 | **Pipeline (PSO) pre-warm and disk cache** | UE PSO precaching (5.1+) + bundled PSO caches; Unity `ShaderVariantCollection.WarmUp`/PSO tracing; DX12/Vulkan pipeline caches | **partial**: bake.md plans a warm-up screen; not built; Bevy compiles async (no hitch, but objects missing until compiled) and has no disk cache | First-sight pop-in of new material variants; cold start | S–M (warm-up list from `pipelines/created`); disk cache = upstream (wgpu `PipelineCache`, Vulkan) | P1 | bake.md, diag.md |
| 23 | Texture streaming (mip residency under a VRAM budget) | UE texture streaming pool; Unity mipmap streaming; id Tech/Frostbite virtual texturing | **missing** (vanilla fits in VRAM; WISHLIST §10, M7) | None for vanilla; required for HD packs | L | P2 | WISHLIST §10 |
| 24 | Lower-resolution textures for coarse LODs | universal (texture LOD bias per mesh LOD, HLOD atlases) | **partial**: far field uses vertex colors/atlases; near LOD chain shares full textures | VRAM and bandwidth for distant L0 objects | M (bake) | P2 | bake.md |
| 25 | Terrain virtual texture (cache the splat result) | Far Cry 4 adaptive virtual texture (Chen 2015); UE runtime virtual texturing | **missing**: splats every pixel (macro albedo only past the splat range) | Terrain is the largest GPU item (2.9 ms) | L | P2 (P1 if terrain stays first) | terrain.md |
| 26 | Grass/clutter as GPU density maps, no entities | Horizon (GPU procedural placement, van Muijden 2017); Ghost of Tsushima grass (GDC 2021); UE procedural foliage/grass maps | **designed** (ARCHITECTURE §7.1), not built; Morrowind has no grass, its flora are placed objects | n/a for vanilla content; mods add grass | L | P2 | §7.1 |
| 27 | Animated/particle update culling (don't simulate unseen) | UE Niagara scalability + significance; Unity culling groups / animator culling | **have** for particles; animation LOD in animation.md (M3) | | | | animation.md |
| 28 | Light LOD (per-light max draw distance + fade) | UE light Max Draw Distance + Max Distance Fade Range; Unity HDRP light fade distance | **partial**: far lights are sprites past `d1`; near `PointLight`s are resident with R1, with a cutoff `range = 4 × radius` (`mw_assets` lights) and no distance fade (not verified in the running app) | Fewer lights per cluster in towns at night | S (fold into J1) | P1 | streaming.md, sky-lighting.md |
| 29 | Large-world rendering without re-uploading every instance on rebase | UE5 Large World Coordinates (double precision, GPU camera-relative); UE4 origin rebasing (also full) | **partial**: big_space floating origin; Bevy re-collects every instance on a shift (3–4 ms), shift guard hides it | Removes the origin-shift spike class | L (upstream) | P2 | world-coordinates.md |
| 30 | Incremental render-world tables | UE GPUScene dirty-primitive updates | **partial**: Bevy's mesh extraction is incremental, `extract_visibility_ranges` is not (~1.1 ms per change frame) | Spawn/despawn frames | S upstream patch | P1 | streaming.md (upstream candidate 3) |
| 31 | Precomputed visibility for interiors (portals/PVS) | Bethesda room bounds/portals, previs; Source/id PVS | **n/a now**: Morrowind interiors are small separate worldspaces | | | P2 (M8 portals) | streaming.md |
| 32 | Virtualized geometry (Nanite, meshlets) | UE5 Nanite (Karis 2021) | **n/a**: Bevy meshlets rejected (far-field.md) | | | P2 revisit | far-field.md |
| 33 | Frame pacing / present mode in measurements | Every platform team gates on present-to-present with a fixed present mode | **partial** (spikes3 measuring Mailbox/Fifo, window ≤ panel) | Removes false gate failures | in progress | P0 (running) | diag.md |
| 34 | Per-category importance (never cull actors, doors, quest items) | Bethesda per-category fades; GTA `priorityLevel`; UE significance manager | **missing** | Correctness for size culling | S (part of J1) | P0 | streaming.md |

## The size-based draw distance (J1, J2)

**Formula.** One constant shared with LOD selection (`sf_model::LodScreen`):

```
K       = H / (2 · tan(fov_y / 2))           // 1028.3 at the 1440p / 70° reference
s(d)    = K · 2r / d                         // projected bounding-sphere diameter in px
d_size  = K · 2r / s_min · bias              // distance where it shrinks to s_min px
d_draw  = clamp(d_size, d_floor, d_end)
```

- `r` is the placement's world bounds radius (model bounds × max scale), which
  the index already stores per placement, cells sorted by descending radius.
  The sphere overestimates long thin objects, so it errs toward keeping them.
- `s_min` is in reference pixels (an angle), so the preset sets it and the
  output resolution and DLSS mode don't change what is drawn.
- `bias` is the same preset multiplier as LOD selection.
- `d_end` = `d1` for far-eligible placements (their lead band), the R1 unload
  radius for the rest. `d_floor` keeps anything from vanishing near the player;
  as built it is not scaled by `bias` (`max(K·2r/s_min · bias, d_floor)`).
  Placements without known bounds (radius 0: no model info) are never
  size-culled.
- Fade: `VisibilityRange` end margin `[d_draw · (1 − c), d_draw]` with
  `c = LodScreen::crossfade` (0.1): the object dithers out between ~2.2 and
  2 px at High, never pops.
- LOD chain: `lod_ranges(errors, screen, d_draw)` instead of the ring radius.
  Levels that would start at or past `d_draw` are **not spawned**, so a small
  prop loses its coarse levels entirely.

**Presets** (High = OpenMW's default small-feature cutoff; Ultra = our 1 px
LOD budget). Distances for `bias = 1`, capped by `d1`:

| preset | `s_min` | `d_floor` | `d1` | cup r 0.06 m | bottle 0.12 | plate/book 0.15 | barrel 0.45 | crate 0.7 |
|---|---|---|---|---|---|---|---|---|
| Low | 4 px | 16 m | 300 m | 31 m | 62 m | 77 m | 231 m | 300 (cap) |
| Medium | 3 px | 20 m | 350 m | 41 m | 82 m | 103 m | 308 m | 350 (cap) |
| High | 2 px | 24 m | 400 m | 62 m | 123 m | 154 m | 400 (cap) | 400 (cap) |
| Ultra | 1 px | 32 m | 400 m | 123 m | 247 m | 308 m | 400 (cap) | 400 (cap) |

For scale: vanilla Morrowind drew everything only out to its fogged view
distance (about one and a half cells, ~170 m), and OpenMW culls paged distant
objects below a size/distance ratio of 0.01 (~10 px at 1440p).

**Exceptions** (class rules plus an adapter flag, `PlacementFlags::NO_SIZE_CULL`):
actors and doors are never size-culled inside R1; a light's mesh follows
the rule, but its `PointLight` gets a light rule (a max distance from its
radius with an intensity fade, row 28); markers have no mesh. Adapters flag
quest items and landmarks.

**Interaction with lead bands and the far field.**
- A placement that **contributes to its L1 proxy** keeps `d_draw = d1` at every
  preset: culling it before the handoff would open a hole until its proxy
  appears. **Corrected by J1 (2026-10-12):** the L1 bake keeps objects whose
  *diameter* (not radius) projects to at least 2 px at `max(d1 − lead, d1/4)`
  (`FarLevels::min_radius`), so r ≥ 0.39 m at 400 m with no lead, ~0.3 m at a
  typical lead, never under 0.1 m. Those have `d_size` as low as `d1 − lead`
  at High and ~0.4 · d1 at Low, so the clamp is a real behavior, not a
  guard. Built as `CellProxies::kept_radius` (the far field evaluates the
  bake's rule with the loaded L1 node's lead).
- A placement **not in L1** ends at `min(d_size, d1)` with its fade and is gone
  past it, as it already is past `d1` today, only sooner.
- `SubtreeRange` (runtime-imported models) uses the same `d_draw`, pushed out
  by node offset as now.
- R2 and the far field are unchanged.

**Don't spawn what will never be drawn (J2).** At a cell's R1 level step,
placement p spawns only if `d_draw(p) ≥ D − m`, where D is the scheduler's
effective distance from the visual anchor to the cell's content AABB
(velocity prefetch included) and m the prefetch margin (80 m, as R1's).
Pages are already sorted by descending radius per cell, so the spawned set is
a **prefix**, found by binary search on the cutoff radius
`r_c = s_min · (D − m) / (2K · bias)`. Residency gets **detail tiers**:
sub-levels of R1 keyed on `d_draw` buckets (≥ 256 m, 128, 64, 32, floor).
Tier k loads at `T_k + m` and unloads at `1.25 · T_k + m`, moving one level
at a time like every other step, so the hysteresis and cancellation rules
carry over. `CellSummary` gains per-tier counts and byte estimates (a bake
change). Gameplay activation (R0, 192 m) doesn't wait for visual tiers: a
cup 150 m away is active (scripts, item state) without a mesh. That splits
activation from visuals, as RAGE and UE do.

### J1 as built and measured (2026-10-12, `.sessions/sizecull.md`)

Built: `LodScreen::{min_pixels, min_draw_distance, focal_px, draw_distance}`
(`sf_model`), `size_culled_end` + `StreamStats::size_culled` (`sf_world_bevy`,
applied to every spawner through `SpawnCx::max_distance`/`end_band`, so the
runtime-NIF `SubtreeRange` path follows too), `CellProxies::kept_radius`
(far field reports the L1 keep rule per cell), `PlacementFlags::NO_SIZE_CULL`,
app `--quality low|medium|high|ultra` and `--off sizecull`. Bevy 0.20 checked
from source: `VisibilityRange` is tested against the mesh origin (`use_aabb`
false) from `lod_view_world_position`, i.e. the camera or `ShadowLodOrigin`
in shadow views, on CPU (`check_visibility_ranges`) and GPU
(`mesh_preprocess.wesl`), and the dither (`get_visibility_range_dither_level`)
runs in the main and prepass/shadow shaders; so a size-culled object fades
out of its shadow at the same distance. The design held; changes: the
L1-contributor clamp is real (see above), `d_floor` is absolute, radius 0
(unknown bounds) is exempt.

**Histogram** (`cargo run --release -p mw_bake --example size_histogram`,
147,897 exterior placements with a model; radius p5 0.66 m, p10 1.2 m,
p50 3.6 m): placements size-culled before their ring/handoff end: Low 11,318
(8 %), Medium 8,737 (6 %), High 4,773 (3 %), Ultra 491 (0.3 %), almost all
between 128 m and the handoff. Eligible ones the L1 clamp may keep: 1937 /
1254 / 528 / 126. **Vanilla exteriors have few small objects**: the clutter
the audit pictured (cups, plates, books) is interior content; there are only
432 item placements outside.

**A/B** (`perf suite`, 2 interleaved rounds on/off, release `solari,dlss`,
DLSS off, RTX 4060 Laptop at 2573–2584 MHz, 66–82 °C; `target/perf/sc-*`):
Balmora at High size-culls 503 of 13,357 resident placements; entities
48,419 either way (culled chains are single-level, no level skipped).
Medians on/off: Balmora 1440p 11.95, 12.02 / 12.02, 12.12 ms (main opaque
6.29, 6.39 / 6.31, 6.43); **Balmora CPU floor (960x600) 6.03, 6.01 / 6.32,
6.32 ms**, main busy 4.76, 4.67 / 5.05, 5.01 (the only consistent gain,
~0.3 ms: fewer entities pass frustum tests, sorting and extraction); run
1440p 10.54, 10.46 / 10.49, 10.54, floor 7.70, 7.52 / 7.81, 7.60; flight 1440p
10.90, 11.03 / 10.99, 11.02, floor 6.65, 6.68 / 6.77, 6.75. Stutter gate
(borderless 1080p, uncapped): flight 20.1, 21.0 / 21.8, 21.8 a minute, run
3.0, 1.5 / 3.0, 3.0; it fails either way. Draw counts were not measured
(no counter; Nsight). Verdict: correct and cheap, keep on (High default),
but no measurable GPU win on vanilla exteriors. This lowers the expected value of
J3 (size-tiered spawning): at most ~3–8 % of exterior placements, so J2
(`NoCpuCulling`) and J6 (precombine) are the bigger entity/CPU levers.

## Prioritized jobs

Each job has one deliverable and is sized for a ~100-tool-call agent. "Measure"
means `perf suite` A/B on an idle machine (perf.md), Balmora, run and flight,
1440p and 720p (CPU floor).

**P0**
1. **J1: size-based draw distance.** First, a histogram of exterior placements
   by `d_draw` bucket per preset (from the baked index, test or example, no
   app run). Then `sf_model::lod`: `LodScreen::draw_distance(r)` plus
   `s_min`/`d_floor` fields; `sf_world_bevy` `SpawnCx`: `d_draw` caps the chain
   and its range, levels past it are skipped, the L1-contributor clamp,
   class exceptions and `NO_SIZE_CULL`; app presets wire `s_min`, `d_floor`,
   `d1` and `bias` (closes Next up 4's preset part); `--off sizecull` A/B.
   Deliverable: entities, instances, main-pass GPU and CPU-floor deltas in
   streaming.md, plus a screenshot pair at the High cutoff.
2. **J2: `NoCpuCulling` A/B for streamed statics and far-field proxies.** A
   `StreamingConfig::gpu_culling` flag (default after measurement); keep
   particles and anything that reads `ViewVisibility` on CPU culling; check
   cascades still cull (shadow screenshots), crossfades and lead bands
   unchanged, `stream/*` and trace counters for `check_visibility*`,
   `check_dir_light_mesh_visibility` and extraction. Deliverable: decision +
   numbers in streaming.md and §10.
3. **J3: size-tiered spawn residency.** Detail tiers in the `sf_world`
   scheduler (`CellSummary` tiers baked, prefix spawn by cutoff radius,
   hysteresis tests: an anchor oscillating at a tier edge causes at most one
   load and one unload per crossing), activation decoupled from visual
   tiers. Deliverable: scheduler tests + flight/run entity and spawn
   counts, worst/median.

**P1**
4. **J4: shadow-caster rules.** Nsight per-cascade shadow pass cost; casters
   below a texel threshold per preset get `NotShadowCaster` (Low/Medium:
   r < 0.25 m) or a shadow-only range; contact shadows cover the small ones.
   Deliverable: weather/distant-shadows.md rule + measured shadow-pass delta.
5. **J5: depth prepass dither fix, then prepass/occlusion A/B.** Apply the
   two-line fix (sky-lighting.md) as a local `[patch]` and file it upstream;
   measure a depth-only prepass (overdraw on terrain) and occlusion culling in
   Balmora, Vivec and Mournhold. Deliverable: decision per worldspace type.
6. **J6: near-cell precombine evaluation.** After J1–J3: bake per-cell merged
   meshes of non-interactive, non-animated statics per material within one
   LOD band (an L0.5 level, Fallout 4 precombine / OpenMW active-grid paging
   style); A/B entities, CPU, GPU and memory against instanced placements.
   Deliverable: far-field.md section with numbers and a keep/drop decision.
7. **J7: pipeline warm-up.** Record `pipelines/created` over a flight; compile
   that variant list on the bake/loading screen; check wgpu `PipelineCache`
   (Vulkan) for a disk cache upstream. Deliverable: zero pipeline creations
   after warm-up in a flight, bake.md updated.
8. **J8: upstream incremental `extract_visibility_ranges`** (Bevy PR: track
   added/changed/removed instead of rebuilding; the fields are private).
   Deliverable: a PR, measured locally via `[patch]`.
9. **J9: "Industry comparison" sections** in the existing specs (streaming,
   far-field, terrain, bake, sky-lighting, weather, world-coordinates, diag,
   animation, character, ui, locomotion), each listing what UE/Unity/
   Bethesda/RAGE/Decima/OpenMW do and why we match or differ. Docs only.

**P2:** texture streaming and coarse-LOD textures (rows 23–24, M7), terrain
virtual texture (25), GPU instanced grass (26), ISM-style instance batches
(12), static shadow caching (14), terrain horizon occlusion (16), interior
portals (31, M8), camera-relative instance data upstream (29).

## Sources

Checked this session (2026-10-12):
- UE, Cull Distance Volumes: https://dev.epicgames.com/documentation/en-us/unreal-engine/cull-distance-volumes-in-unreal-engine
- UE, Visibility and occlusion culling reference: https://dev.epicgames.com/documentation/unreal-engine/visibility-and-occlusion-culling-reference-in-unreal-engine
- UE, `UInstancedStaticMeshComponent` (per-instance start/end cull distance): https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/UInstancedStaticMeshComponent
- UE3 visibility culling (MaxDrawDistance, volumes use the lower value): https://docs.unrealengine.com/udk/Three/VisibilityCulling.html
- UE, World Partition HLOD (layers with their own runtime grid cell size and loading range): https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition---hierarchical-level-of-detail-in-unreal-engine
- UE 4.22 mesh drawing pipeline refactor (automatic instancing, cached static draw commands): https://www.unrealengine.com/events/unreal-fest-europe-2019/refactoring-the-mesh-drawing-pipeline-for-unreal-engine-4-22
- Unity, GPU Resident Drawer: https://docs.unity3d.com/6000.3/Documentation/Manual/urp/gpu-resident-drawer.html
- Unity, LOD Group: https://docs.unity3d.com/6000.4/Documentation/Manual/class-LODGroup.html
- OpenMW camera settings (small feature culling, pixel size 2.0): https://openmw.readthedocs.io/en/openmw-0.47.0_a/reference/modding/settings/camera.html
- OpenMW terrain settings (object paging, min size 0.01, active grid paging): https://openmw-trav.readthedocs.io/en/latest/reference/modding/settings/terrain.html ; defaults: https://git.cijber.net/mirror/openmw/src/tag/stable/files/settings-default.cfg
- Fallout 4 precombines and previs (Bethesda staff quoted secondhand): https://forums.nexusmods.com/topic/5522717-fallout-4-optimization-and-performance-systems-explained/
- Skyrim SE fade multipliers (objects 5, items 3): https://stepmodifications.org/wiki/SkyrimSE:SkyrimPrefs_INI/LOD
- RAGE (GTA V) entity `lodDist`, `childLodDist`, priority level: https://docs.sollumz.org/documentation/maps/entities ; https://docs.fivem.net/docs/assets-manual/beginner-series/part-6/
- Horizon Zero Dawn GPU procedural placement (van Muijden, GDC 2017): https://www.guerrilla-games.com/read/gpu-based-procedural-placement-in-horizon-zero-dawn

- Haar, Aaltonen, *GPU-Driven Rendering Pipelines*, SIGGRAPH 2015 Advances (Ubisoft, AC Unity; GPU
  decides visibility per viewport, motivated by long draw distances, many small objects and shadow-map
  draw counts): https://history.siggraph.org/?p=60117 (course page http://advances.realtimerendering.com/s2015/)
- Wihlidal, *Optimizing the Graphics Pipeline with Compute*, GDC 2016 (Frostbite): https://gdcvault.com/play/1023109/Optimizing-the-Graphics-Pipeline-With
- UE shadow-caster screen-size cull: `r.Shadow.RadiusThreshold` and project setting "Min Screen Radius for
  Cascaded Shadow Maps" (both default 0.03 per forum reports; higher at Medium scalability and below):
  https://issues.unrealengine.com/issue/UE-372125 and the visibility culling reference above

**Unverified** (known references, cited from memory, not fetched this session; check before quoting
details):
- Collin, *Culling the Battlefield*, GDC 2011 (Frostbite software occlusion)
- Karis et al., *Nanite: A Deep Dive*, SIGGRAPH 2021 Advances
- Chen, *Adaptive Virtual Texture Rendering in Far Cry 4*, GDC 2015
- Wohllaib, *Procedural Grass in Ghost of Tsushima*, GDC 2021
- Ruskin, *Marvel's Spider-Man: A Technical Postmortem*, GDC 2019 (cell streaming budgets)
- UE PSO precaching (5.1+) and texture streaming docs on dev.epicgames.com
- Courrèges, *GTA V Graphics Study*: https://adriancourreges.com/blog/2015/11/02/gta-v-graphics-study-part-2

Bevy 0.20 sources read: `bevy_camera/src/visibility/{mod,range}.rs`,
`bevy_light/src/lib.rs` (`check_dir_light_mesh_visibility`),
`bevy_render/src/view/visibility/{mod,range}.rs`,
`bevy_render/src/render_phase/mod.rs`, `bevy_render/src/batching/gpu_preprocessing.rs`,
`bevy_pbr/src/render/{mesh.rs, mesh_preprocess.wesl, gpu_preprocess.rs}`.

## M3 follow-ups (j9m3)

J9 done for the M3 character specs (2026-10-12, `.sessions/j9m3.md`): "Industry
comparison" sections in animation.md, character.md, locomotion.md and
rig-profile.md; bake.md's skinned section already had one (one gap note
added). Still without one: streaming, far-field, terrain, sky-lighting,
weather, world-coordinates, diag, ui. Techniques checked against UE5
(Animation Blueprints, Pose Search, Control Rig, IK Retargeter, Mutable, mesh
merge, Animation Budget Allocator, URO, Animation Sharing, AnimToTexture,
Skin Cache), Unity (Mecanim, Animation Rigging, Kinematica, Animation
Instancing), Creation Engine (Havok Behavior, process levels, FaceGen),
RAGE (Euphoria, ped components), Decima (no public motion-matching source
found), Ubisoft (motion matching, AC Unity crowds) and OpenMW (processing
range, 0.49 blending rules, NPC avoidance settings). Gaps, ranked:

| # | Gap | Spec | Priority |
|---|---|---|---|
| G1 | Morph targets on merged meshes: Bevy 0.20 stores them densely per vertex of the whole mesh (48 B/vertex/target) and loops every target per vertex per pass; UE's merge drops morphs, Mutable and FaceGen bake shape morphs | character.md, bake.md | P1 |
| G2 | Local avoidance between characters (RVO/ORCA; UE CMC RVO or Detour Crowd, OpenMW "NPCs avoid collisions"/"give way"); we only push apart. **Resolved 2026-10-12 (M3-b)** | locomotion.md | P1 |
| G3 | Skinning cost per pass unmeasured; Bevy re-skins in every pass incl. each cascade; no far-cascade rule for characters (UE `bCastFarShadow`) or skin cache | animation.md | P1 |
| G4 | Gameplay-driven one-shot clips (UE montage slots, OpenMW play-group on request): our clips are compile-time graph slots only. **Resolved 2026-10-12 (M3-d)** | animation.md | P1 |
| G5 | Crowd pose sharing (UE Animation Sharing) and per-character phase/rate desync | animation.md | P2 |
| G6 | Distant animated crowd tier (VAT / bone textures, instanced: UE AnimToTexture, Unity Animation Instancing) | animation.md | P2 |
| G7 | Distance matching for starts/stops/pivots, with a stop/pivot point from the trajectory springs | animation.md, locomotion.md | P2 |

Small refinements logged as open questions, not jobs: foot placement pelvis
modes and heel lift (animation.md 6), allocator caps borrowed from UE
(max interpolated, max off-screen ticked), twist distribution (character.md
4), procedural stepping for clip-less creatures (rig-profile.md 3).

Proposed jobs (one deliverable each, about 100 tool calls):

**P1**
- **M3-a: morph-bearing parts.** **Done 2026-10-12 (m3a; bake.md "Morph layout", character.md "Morph-bearing parts").** `sf_model::skin` cook: per-part target
  lists (no zero blocks), all-zero targets stripped, runtime targets kept
  only on LOD 0–1; `sf_character` assembly: morphed parts stay out of the
  per-material merge (own `SkinnedMesh` on the same joints), non-animated
  shape weights baked into vertices at assembly, a shape/expression flag on
  targets from the adapter. Deliverable: unit tests (merged outfit carries
  no morph data; baked shape equals runtime morph at the same weight within
  1e-5), memory per cached outfit before/after in character.md.
- **M3-b: ORCA local avoidance in `sf_controller`.** **Done 2026-10-12 (orca; locomotion.md "Local avoidance (ORCA)"; own ORCA, dodgy not taken).** Evaluate `dodgy_2d`
  0.5.5 (MIT/Apache, glam 0.29 vs our 0.33: convert or patch), else a
  ~300-line ORCA on our types; neighbours from a per-frame grid; player as
  a non-reactive agent; give-way for idle NPCs. Deliverable: tests (two
  agents head-on pass without contact, 8 agents swapping through a 1.5 m
  doorway finish without deadlock in < 20 s simulated, batched = single,
  ≤ 0.1 ms for 150 agents single-thread) and locomotion.md update.
- **M3-c: skinned-character GPU cost breakdown** (needs app runs, so after
  the build-timing agent and once `sf_model_bevy` loads skins). Crowd street
  with 150 NPCs: GPU time per pass (prepass with motion vectors, main, each
  cascade) for skinned meshes; A/B with characters out of cascades 3–4 (no
  per-cascade mask in Bevy 0.20: `NotShadowCaster` by animation tier or
  distance, or a local patch) and A3 characters not casting. Deliverable:
  numbers and a rule in animation.md "LOD and budget"; decide whether a
  compute skin pre-pass (open question 3) moves before M7.
- **M3-d: `Slot` node in `sf_anim_graph`.** **Done 2026-10-12 (slotnode; animation.md "Slots and montages").** Montage-style one-shot playback
  of a clip chosen at runtime (by id from the binding or a late-bound set),
  blend in/out by inertialization, root motion mode, events and warp
  windows, interrupt and queue rules; adapter hook for scripted "play
  this animation group" requests. Deliverable: node + tests (events fire
  once, root motion matches `Drive`, interrupt mid-clip continuous) and an
  animation.md section.

**P2**
- **M3-e: crowd pose sharing + desync.** A2 characters bucketed by
  (skeleton, clip, quantized phase) share one evaluation; per-character
  random start phase and ±5 % rate. Deliverable: crowd-street µs per tier
  before/after and a no-lockstep visual check.
- **M3-f: animation-texture crowd tier for R1.** Bake bone-matrix textures
  (GPU Gems 3 style, fewer bytes than per-vertex VAT and outfit-independent)
  from cooked clips for the ambient set; an instanced material that skins
  from the texture; handoff A3 ↔ texture tier with a crossfade. Deliverable:
  design section in animation.md + a prototype count (NPCs at 60 fps in R1).
- **M3-g: distance matching.** `distance` curves at `anim.cook`, a
  seek-by-distance `Player` mode, closed-form stop/pivot points from
  `sf_controller`'s springs. Deliverable: tests (stop lands within 3 cm of
  the predicted point) once library start/stop clips are baked.

Sources for this section: the "Industry comparison" sections of the four
specs (each lists its own, verified and unverified).

## Follow-ups (j9rest)

J9 finished for the remaining specs (2026-10-12, `.sessions/j9rest.md`, docs
only): "Industry comparison" sections in terrain, sky-lighting, weather,
world-coordinates, diag (benchmark method, next to the existing frame-pacing
comparison), ui, far-field, bake (whole pipeline), portals, streaming (index
of this audit + door comparison), and the seven bevyrrowind adapter specs.
smooth-bodies.md and the four M3 specs already had theirs. Gaps found, ranked
by visible impact and cost (each also an open question in its spec):

| # | Gap | Spec | Priority |
|---|---|---|---|
| R1 | Raster ambient occlusion (GTAO) and contact shadows: Bevy 0.20 ships both (`ScreenSpaceAmbientOcclusion` = XeGTAO port, `ContactShadows`) but both need the depth prepass, which exteriors leave off; every compared engine has AO on its raster path | sky-lighting.md 6 | **P1** (after J5) |
| R2 | Object-terrain blending: rocks/walls meet terrain with a hard seam; UE RVT mesh blending is the standard fix; our layer clipmap + macro albedo can serve it without a virtual texture | terrain.md 2 | **P1** (visuals) |
| R3 | Reactive/transparency mask for transparents without motion vectors under DLSS/TAA (rain, particles, UV-scrolled water/lava); Bevy 0.20's DLSS node passes `bias: None // TODO` | weather.md 9, nif-runtime.md | **P1** (with precipitation) |
| R4 | Soft particles (depth fade) | bevyrrowind nif-runtime.md | P2 (after J5) |
| R5 | Baked indirect light for raster interiors (irradiance volumes; Bevy has volumes but no baker) | sky-lighting.md 7 | P2 |
| R6 | Hex tiling (Mikkelsen 2022) for terrain layers | terrain.md 1 | P2 |
| R7 | Runtime-toggled placement groups (UE Data Layers, Creation enable parents) + far-field consistency | streaming.md 2 | P2 |
| R8 | Bloom on game cameras | sky-lighting.md 8 | P2 (small) |
| R9 | A/B significance test in reports; GPU clock locking for perf runs | diag.md 4, 5 | P2 (small) |
| R10 | Subtitles/captions settings; safe-area margins | ui.md 5, 6 | P2 (M5) |
| R11 | Sound through open portals (Wwise Rooms and Portals) | portals.md comparison | P2 (M8, with an audio spec) |

Checked and **not** gaps (verdicts in the specs): CDLOD/clipmaps vs Nanite
landscape (meshlets still rejected: Bevy 0.20 `MeshletPlugin` limits
unchanged, re-read in source), aerial perspective (Bevy's Hillaire LUTs),
exposure (portal2's `AutoExposure` + compensation curve = UE's pattern;
local exposure noted), Nubis/froxel fog/wetness (weather.md already
canonical), floating origin vs LWC (only row 29), octahedral impostors vs
UE 5.7 Nanite voxel foliage (n/a in Bevy), derived-data cache design, UI
stack (CommonUI/UI Toolkit family).

Proposed jobs (one deliverable each, ~100 tool calls):

**P1**
- **R-a: raster AO + contact shadows** (after J5's prepass fix). Add
  `ScreenSpaceAmbientOcclusion` and `ContactShadows` to the raster
  `SkyCamera` set behind a preset quality (Low off, Medium AO low, High AO
  medium + contact shadows); check interaction with the far-field dither and
  DLSS. Deliverable: Balmora/Census screenshot pairs in visual-checks.md and
  measured GPU cost at 1440p in sky-lighting.md.
- **R-b: object-terrain blending.** Bake flags statics touching terrain;
  `sf_terrain_bevy` exports a WESL import that samples the layer clipmap
  (near) / macro albedo (far) at a world position; `WorldMaterial` blends
  albedo and normal by height above terrain over a per-model fade. Check
  bind-group cost and the Solari path. Deliverable: before/after shots of
  rocks and wall bases, GPU delta, terrain.md section.
- **R-c: reactive mask for DLSS.** Local `[patch]` of `bevy_anti_alias`
  feeding a per-camera bias texture to `dlss_wgpu` (and an upstream PR);
  particle and (later) precipitation materials write it. Deliverable: rain/
  particle screenshots at DLSS performance with and without, and the PR.

**P2**
- **R-d: soft particles** in `mw_assets` particles (after J5). Deliverable:
  fireplace/smoke shots, no cost regression.
- **R-e: interior irradiance bake.** One irradiance volume per interior
  baked at bake time (Solari offline pass or a CPU path tracer), only if a
  3-interior visual test shows a clear gain; needs `RenderLayers` on light
  probes for portals. Deliverable: decision + 3 screenshot pairs + bake time.
- **R-f: hex tiling** on terrain layers flagged `anti_tile`. Deliverable:
  screenshot pair on a large uniform layer + GPU A/B.
- **R-g: placement groups and enable state** in `sf_world` overlay +
  far-field exclusion rule; adapter maps script-toggled refs. Deliverable:
  tests (toggle round-trips through save, a toggled group never appears in a
  proxy) and streaming.md section.
- **R-h: small polish batch**: bloom preset, `perf suite --lock-clocks`,
  bootstrap CI in the suite summary. Deliverable: each with its doc line.
- UI subtitles/safe area (R10) and portal audio (R11) go with M5 and M8
  rather than as standalone jobs.

Sources for this section: the "Industry comparison" sections of the specs
named above (each lists its own, verified and unverified).
