# Groundcover: species, rules, runtime scatter

Status: **2026-11-23: mesh path sized to demand (GC-M1: fault and +1.0 ms floor gone; meadow +1.1 ms for 42k plants)
with sway (GC-M2); blade carpet through the vendored `bevy_grass` built up to the wind bridge (GC-3a to GC-3c), off by
default (`groundcover.blades.enabled`), first GPU run +0.36 ms; awaiting owner look 1 (scene `gc-blades`).** Next:
GC-3b4 (rebuild spike), the trail-ring trampling (GC-T1, GC-T2, GC-M3, GC-3d), "Build plan".
Crates: `sf_groundcover` (Bevy-free core),
`sf_groundcover_bevy` (thin plugin: both renderers, wind and trample bridges). Adapter:
`../../../bevyrrowind/docs/specs/groundcover-adapter.md`. Research: `../research/groundcover-2026-11-22.md` (industry
read + source read of `bevy_grass`, `bevy_feronia`, `bevy_meadow`, `bevy_foliage_tool`). ROADMAP: Phase 4
"Groundcover" (owner go 2026-11-22). Worldgen hook: `worldgen.md` WG-N3.

<!-- agent-summary -->
> **Agent summary.** A *species* is a data record (render kind blade or mesh, footprint, density, slope/altitude/
> water constraints, scale/tint ranges, wind and shadow flags). A *rule table* maps terrain layer weights (and,
> for generated worlds, material fields) to weighted species, like Skyrim's land texture → grass list and UE's
> Landscape Grass Type. Instances are never stored: they are scattered at runtime, deterministically per world
> tile around the camera, then culled, LODed and drawn instanced. Blade species draw through `bevy_grass`; mesh
> species (authored plant models with alpha cards) through our own instanced path. Overrides on top: painted
> density, exclusion (roads, water, under buildings), "only this species here" regions, priorities. Explicit
> placements from mods feed the mesh path.

## Goal

Grass, ferns, flowers and small plants on exterior terrain that look dense and alive near the player, cost well
under a millisecond on High, work on any terrain the content or the worldgen suite produces, and can be steered by
a designer without hand-placing every plant. Procedural by default (owner 2026-11-22: "procedural in the long run,
to work with our world gen"), with a way to say "this species here".

Non-goals: trees and large shrubs (statics / worldgen scatter, `worldgen.md` stage 8); gameplay use of individual
plants (harvestable plants stay records/statics); ray-traced grass in Solari (open, below).

## Reuse check (CONTRIBUTING rule 0/2)

- **Bevy 0.20:** no grass or foliage feature. It gives GPU preprocessing, indirect draws, `VisibilityRange`, our
  vendored `GpuInstanceBatchPlugin` (bevy#25978 port, used by the static scene, `docs/wiki/engine/statics-and-lod.md`)
  and the material/prepass machinery the renderers build on.
- **Crates (source read 2026-11-22):**
  - `bevy_grass` 0.20.6 (MIT, Bevy 0.20, active, ~308 tests): GPU blades from a hash per 16 m chunk, nothing
    stored, 3 LODs + far-grass material, exact motion vectors, prepass + deferred, directional shadows (Off/Near/
    Full), external wind (`GrassWindFrame`), trampling, runtime mask edits. One style per field; up to 16 species per
    style share the field density (`src/style.rs:19`, `docs/styles.md:208-222`), so a species with its own density
    = its own overlapping field (the crate's documented pattern). **Blades are opaque, untextured geometry**
    (`docs/design.md` item 8: no alpha testing), and arbitrary meshes are a non-goal. **Adopted for blade species.**
  - `bevy_feronia` 0.9 (MIT/Apache, Bevy **0.19**, "early, not for production"): mesh/scene species with LODs, one
    layer per species with its own density map, CPU scatter per chunk, GPU cull unfinished. **Not adopted**; its
    layer/chunk model is reference for the mesh path.
  - `bevy_meadow` 0.1 (MIT, Bevy `main`): one blade variant active at a time, 100-200 MB VRAM per variant, the only
    one with Solari RT casters. **Not adopted** (Bevy main, one active variant); its RT caster selection is
    reference if grass ever enters Solari.
  - `bevy_foliage_tool` (Bevy 0.16, dead): no.
- **Nothing covers the species/rule layer**: every crate takes a density mask per field. That layer is ours and
  small (data + a pure function), so it lives in a Bevy-free core.

## Industry comparison

Sources and per-claim confidence: `../research/groundcover-2026-11-22.md` §1.

| Engine / game | Species model | Placement | Overrides | Ours |
|---|---|---|---|---|
| UE5 Landscape Grass Type | list of varieties (mesh, density, jitter, cull distances, shadow flag) per paint layer | runtime near the camera from layer weight maps | paint the layer | Match: rule table + species |
| UE5 PCG Biome Core | biome = priority + weighted asset list | runtime graphs, `$Density` | exclusion volumes/splines, priorities, injected points | Match: priorities, exclusion, explicit placements |
| Horizon Zero Dawn (Decima) | per-asset density graph, footprint | fully runtime GPU, 64² grid per 32 m (grass) block, ~0.25 ms | painted maps override; road/water/object masks multiply | Match: runtime, deterministic, masks multiply |
| Ghost of Tsushima | procedural blades by terrain texture; flowers/pampas as instanced artist meshes | runtime compute per tile, far = texture on terrain | (unverified) | Match: blades + mesh path, far colour |
| Skyrim | LTEX lists GRAS (model, density, slope, water, ranges) | per cell near the player | land texture painting | Match: the adapter's table is exactly this |
| OpenMW `groundcover=` | none: hand-placed statics from plugins | instanced, density + distance settings | the plugin | Mesh path's explicit placements |

## Design

### Data model (core, `sf_groundcover`)

**Species** (a record, loaded through `sf_records`, so mods add and override them):

| Field | Meaning |
|---|---|
| `id` | record id |
| `render` | `Blade { style }` (a `bevy_grass` style, `.grass.ron`) or `Mesh { model, lods }` (a model through `sf_model`) |
| `footprint_m` | spacing between instances of this species (Horizon footprint); bounds the scatter grid |
| `density` | instances per m² at weight 1 |
| `slope_deg`, `altitude`, `water` | allowed ranges; `water` = min/max distance to the water surface (negative = submerged, for later reeds/lilies) |
| `scale`, `rotation`, `tint`, `align_to_surface` | per-instance random ranges (Skyrim GRAS, UE varieties) |
| `wind`, `trample` | response factors |
| `z_offset_m` | origin height above the ground at scale 1 (models whose geometry hangs below the origin; UE foliage Z offset) |
| `shadow` | `None` / `Near` (cast inside the shadow range) |
| `range` | max draw distance class: `Near`, `Mid` (tunable distances per preset) |

**Rule table** (a record set): input channels → `(species, weight)` lists. Channels are the terrain layer weights the
terrain already has (`terrain.layer_table`, splat clipmaps) and, for generated worlds, the worldgen material fields
(slope, wetness, altitude, rock type, exposure; `worldgen.md` Pipeline 6). Weight × channel weight × constraint
tests × override masks = final density. The adapter fills the table (from its game's data; the framework names no
format).

**Overrides** (Horizon's painted maps, UE PCG's exclusions), all multiplying density:
- *Density paint* per species group: an R8 map per terrain tile, authored or edited, saved as world data.
- *Exclusion:* layers with no rule (roads), water (from the water query), under static geometry (from the static
  placements' footprints, or the surface-weather shelter map, decided in GC-1), explicit volumes.
- *"Only this species here"*: a region record (volume or painted mask) that replaces the rule table's list inside it.
  Regions have a priority, like PCG biomes (higher wins).
- *Explicit placements*: instances listed by content (a mod's groundcover placements) go straight to the mesh
  renderer and suppress procedural species of the same `range` inside their region (priority rule above).

### Scatter (core + GPU)

Deterministic and stateless: positions are a hash of (world integer tile, species, index), jittered on a grid sized
by `footprint_m` (UE "use grid", Tsushima, Horizon dithering). Tiles are world-anchored integers, so the result is
the same on every client and across floating-origin shifts (net-ready: presentation only, never fed to gameplay; a
CPU twin of the density function answers gameplay queries such as stealth or footsteps later). The core holds the
density function and its CPU twin; tests pin the twin to the GPU path.

### Renderers (`sf_groundcover_bevy`)

- **Blades: `bevy_grass`.** One `GrassField` per blade species per streamed tile block, density from the rule
  function (`with_density_from_fn` or a baked mask per block), ground from our terrain heights, wind parameters from the
  weather's `Wind` through `GrassWindFrame` (the crate draws its own gust texture: "Wind layers and trampling"),
  quality tier from our preset. Blades take their colour from the terrain layer under
  them (Tsushima, UE grass on landscape colour), so they continue the ground texture. If `bevy_grass` can't take a
  per-blade colour source, that is an upstream PR or a vendored patch (rule 4), checked in GC-3.
- **Meshes: our instanced path.** Mesh species are authored plant models (alpha-tested cards). Instances per tile
  near the camera, GPU-culled and LODed, drawn instanced with prepass, motion vectors and near shadows, wind in the
  vertex shader (height-weighted bend from `sf_wind::sample`, the OpenMW groundcover and UE SimpleGrassWind pattern).
  **Decided (GC-R, 2026-11-22): CPU scatter per tile on a task, written into retained `GpuBatchedMesh3d` batches**
  (the vendored `GpuInstanceBatchPlugin`, bevy#25978 port, that the static scene uses). Why:
  - UE Landscape Grass does the same: an async task per landscape component builds instances into a HISM
    (`FAsyncGrassTask`, API docs; the build details are unverified). Horizon's GPU placement reads its points back
    to the CPU (search snippet of the slides, unverified).
  - Batches draw through Bevy's standard mesh pipeline with any `MeshMaterial3d` (`gpu_instance_batch/mod.rs:116`),
    so alpha mask, prepass, deferred, two-phase occlusion culling, cascades and lamp views come for free. Wind is an
    `ExtendedMaterial` vertex/prepass override.
  - The output buffer is `STORAGE | COPY_DST` and built for a compute producer (`mod.rs:1-7`, `:1007`), so a GPU
    scatter can replace the CPU writer later without touching the draw path.
  - The CPU scatter *is* the core's density CPU twin, so CPU and GPU can't disagree.
  - Rejected: `bevy_grass`-style own render phase (`bevy_glue.rs`, ~5.5k lines of render code; would need vertex
    pulling, our own alpha-mask PBR, Hi-Z and lamp queueing, and it breaks at every Bevy upgrade). Also rejected:
    per-instance entities (the ECS/extract cost the statics ledger measured) and tile-merged meshes (~5 MB per tile, a guess).

  Facts that shape the build:
  - The instance is 64 B (`mod.rs:90-95`): a 3×4 transform plus one `tag: u32`, read in shaders with `get_tag`.
    Wind phase, tint and fade seed pack into the tag or come from a world-position hash.
  - Previous transform = current (`mesh_preprocess.wesl:271-286`), which is right for static plants. Wind motion
    vectors come from the prepass vertex shader evaluating wind at t and t-dt (the `bevy_feronia` pattern, previous
    wind time in a uniform).
  - Cost scales with **capacity**, not live slots: `materialize` dispatches `capacity/64` workgroups per batch every
    frame (`materialize.rs:328`), and the preprocess runs per view. Capacity per preset is a tunable, measured in GC-2.
  - No per-instance visibility range: batch flags set the range index to `u16::MAX` (`mod.rs:581`). Distance comes
    from CPU tile rings plus a dithered fade in the vertex shader. A small vendored patch carrying the batch's
    `VisibilityRange` index follows later (per-view range cull, shadow range in cascades and lamp faces).
  - Origin shift rewrites every live slot (`sf_scene_bevy/src/shift.rs`). Groundcover uses the same path; the planned
    per-batch origin delta fixes both.
  - Nothing is drawn without GPU culling (`mod.rs:119-133`); that's an accepted hardware floor.
  - Slot pool: fixed tile blocks of `max_per_tile` slots, so there's no fragmentation, one write run per tile, and
    freeing a tile is one zero-fill. Churn (guess): walking ≈ 640 KB/s, flight ≈ 3 MB/s of `write_buffer`.
- **Far:** past the last instance, the terrain itself carries the colour (Tsushima's far texture; `bevy_grass`'s
  `FarGrassMaterial`). No impostors in v1.
- **Trampling:** one trample source for snow, ash, mesh plants and blades: the surface-weather trail ring (owner
  2026-11-23; "Build plan" GC-T1, GC-T2, GC-M3, GC-3d). The crate's own interactor window is not used.

### Tunables (`tunables.md`; all `Guess` until measured)

Per preset: blade range (High 100 m, `bevy_grass` default), mesh near range (60 m), mid range (100 m), density scale
(1.0), shadow range (30 m), tile size (16 m, `bevy_grass` chunk), max instances per camera, wind and trample scales.
Low preset = off or far colour only.

### Budget

Target **≤ 0.8 ms GPU** for all groundcover at High, 1440p + DLSS Quality on the 4060 laptop (Guess). Basis:
`bevy_grass` measured 0.32 ms at High 1080p 4×MSAA on a 4070 Ti (its `docs/performance.md`); the 4060 laptop is
roughly half a 4070 Ti, DLSS Quality's internal 960p is close to 1080p's pixel count; Horizon's placement 0.25 ms.
Tsushima's ~2.5 ms (secondhand) is a console's whole-field figure and is the ceiling we stay well under.

### Solari (Ultra)

Grass stays out of the RT acceleration structure in v1 (raster shadows near; RT GI sees bare ground). `bevy_meadow`'s
caster selection is the reference if the owner wants it later.

## Build plan

One deliverable plus its test per slice.

- **GC-R** done 2026-11-22: mesh path = CPU tile scatter into `GpuBatchedMesh3d` ("Renderers").
- **GC-1** done 2026-11-22: `sf_groundcover` (Bevy-free, hakari excluded, 18 tests). As built:
  - API: `Groundcover::new(species, &RuleTable)`, `Overrides::new(&cover, &OverrideSet)`, `Scatterer::scatter_tile(&ground,
    TileKey, SpeciesIndex, batch_slot, &mut Vec<Instance>) -> TileStats`, `density_at` (the CPU twin), `GroundSampler` trait.
  - Instances: 36 B, tile-local f32 x/z from the tile's min corner + world height; the caller keeps the `TileKey`, so an
    origin shift changes only the per-tile translation (`translation_from`, f64).
  - Grid: `n = round(tile / footprint)` cells per axis (footprint snapped to divide the tile), one jittered candidate per cell,
    accept = `hash < p x 2^32` (integer). Hash: SplitMix64 over (tile, FNV-1a species id, cell), pinned by a test: a format.
  - Tag: bits 0-7 wind phase, 8-15 tint lerp, 16-23 fade seed, 24-31 batch slot.
  - Priority: highest "only" region wins (tie: later), its weights replace the rule list; exclusion beats "only"; explicit
    regions suppress their range class only. Exclusion under statics is left to the adapter/GC-2 as regions or masks.
  - Tunables: `groundcover.tile_size_m` 16, `groundcover.max_instances_per_tile` 16384, `groundcover.density_scale` 1.0 (Guess).
  - Cost: one ground sample per candidate per species; GC-2 should share samples across species with equal grids.
- **GC-2a** done 2026-11-22 (`sf_groundcover_bevy`, 17 tests, headless; not run on a GPU). As built:
  - Game API: insert `GroundcoverSource { cover, overrides, ground: Arc<dyn GroundSource> }` (`GroundSource` = `GroundSampler`
    + `ready(rect)`: a tile scatters only once its terrain is loaded; `FnGround` closure helper; absolute world metres),
    `GroundcoverModels` (model key -> mesh + `StandardMaterial` + `bend_height_m`, may arrive late), `GroundcoverViewer` on
    the camera, `GroundcoverPlugin` after `DefaultPlugins` and the wind plugin.
  - Batches: one plain + one casting batch per model, capacity sized from the ring's worst case, capped by
    `groundcover.render.max_blocks_per_batch` (256; overflow shows as `groundcover/tiles/unplaced`). Writer mirrors
    `sf_scene_bevy/src/upload.rs`; origin shifts follow `sf_world_bevy::WorldOrigin`, live slots rewritten from per-tile CPU copies.
  - Material: `ExtendedMaterial<StandardMaterial, GroundcoverWind>`, forced forward (the tint needs a fragment override);
    sway comes from the moving gust field only (no time binding in the prepass), motion vectors from wind at t and t-dt.
  - Optional per-tile ground cache shared across species (pays only when the real terrain query is expensive: 42 us direct
    vs 124 us cached on flat synthetic ground). Measured (synthetic, 4 species): ~303 instances, 19 KB GPU, 11 KB CPU, ~42 us per tile.
  - Gauges `groundcover/*`; shadow split (near species inside the shadow range cast, the rest `NotShadowCaster`).
- **GC-2b1** done 2026-11-22: `TerrainGround` (`sf_groundcover_bevy/src/terrain_ground.rs`), a `GroundSource` over
  level-0 terrain: heights/normals from `TerrainHeightsMap`, channels from the new `sf_terrain_bevy::TerrainLayersMap`
  (resident level-0 `Arc<LayerMap>` per tile, synced at the end of `TerrainSystems::Update`, written only on change) through a
  caller-supplied `LayerChannels` remap (`LayerId.0` -> rule channel), nearest 1 m sample, weight/255. Snapshot of `Arc`s
  refreshed by `TerrainGroundPlugin` (PostUpdate); `water_distance` = height - the worldspace sea level given at construction
  (lakes and rivers not read yet). Gaps: no rescatter under terrain edits; a GPU world without layer maps gets no groundcover.
- **GC-2b2** done 2026-11-22: Morrowind data `bevyrrowind/data/groundcover.ron` (13 species from the vanilla flora meshes,
  7 rules, 7 named channels keyed by LTEX ids, every number `Guess`) + resolver `mw_world::groundcover` (`GroundcoverData::builtin`,
  `build`, `resolve(&LayerTable) -> ChannelMap { pairs, unmatched }`, `model_keys`, keys like `meshes/f/flora_grass_01.nif`).
  `LayerId` is a 32-bit name hash, so `LayerChannels` is sparse (`from_pairs`, binary search; fixed by the orchestrator).
- **GC-2b3** done 2026-11-22: `mw_exterior_bevy::groundcover::GroundcoverGamePlugin` (registered after the wind plugin):
  layer table from `BakedTerrain::layer_table()`, sea level from the exterior record, NIF models through the runtime loader
  (first visible mesh, node transform baked), `GroundcoverViewer` on the `TerrainCamera`, exterior render layer only; adds
  `GpuInstanceBatchPlugin` itself under `--off scene`. Off switch `--off groundcover` (`Part::Groundcover`). Log line
  `groundcover: species S, channels matched M/N, unmatched [..], models loaded L/13`. Gaps: first mesh only; no normal/ORM
  companions; `ImageRebindPlugin` doesn't re-bind the cloned materials. Scenes `gc-ascadian`, `gc-grazelands`, `gc-bittercoast` (GC-2).
- **Audit 2026-11-23** (owner: sparse, sway reads as uniform, costly; code read, nothing measured). The mesh path was
  doing the carpet's job with vanilla tufts: ~4 clumps/m² in a full meadow, and one candidate per 0.6 m cell caps a species
  at 2.8/m². Roles from here: **blades are the carpet, meshes are accents** (ferns, flowers, tall tufts, small bushes), as
  in Tsushima. The plan below replaces the old GC-3; the original GC-2 plan is in `archive/groundcover-history.md`.

Rules for every slice below: one deliverable plus its test; one build agent at a time in this lane (the slices share
`sf_groundcover_bevy`); sub-agents never launch the game; every new value is a tunable; one ledger line per slice.

**Measure first**
- **GC-P0** (orchestrator; one exe, ~15 min of bench): `gc-ascadian`, three arms interleaved ×3: groundcover off / on /
  on with `groundcover.density_scale` 0.01 (same slot capacity, almost no live plants). It splits the cost into capacity
  overhead (13 models × ~100 tiles × 729 slots ≈ 950k reserved slots preprocessed per view, for ≤ ~45k live) and drawing.
  Median, p99 and hitches per minute per arm, in `ledger/perf.md`. Gates GC-M1.

**Blade lane (the carpet, `bevy_grass`)**
- **GC-3R** research, no build: source read of `bevy_grass` 0.20.6 against our renderer, written as "Blades:
  integration facts" below. One fact with `path:line` and a verdict (as-is / adapter code / upstream patch) per question:
  Bevy features vs `bevy_variants.sh` group [1], render path and motion vectors, floating origin, ground and density
  sources, field tiling over a streamed world, blade colour, wind frame, trample, live off, tiers, shadows.
- **GC-3a1** the dependency: `bevy_grass` 0.20.6 in the workspace (hakari, `bevy_variants.sh` group [1] kept), `GrassPlugins`
  (minus `FarGrassPlugin`, `GrassStyleFilePlugin`) added by a blade plugin in `sf_groundcover_bevy` behind
  `groundcover.blades.enabled`, `GrassShadows::Off`. Test: the plugin builds in a headless app (proves it compiles
  against our vendored Bevy, unverified so far).
  **Done 2026-11-23:** `sf_groundcover_bevy/src/blades.rs` `BladesPlugin`; builds on the vendored Bevy unpatched; a headless app registers `Shader`, `Image` and `Mesh` before it.
- **GC-3a2** one blade style drawn on `gc-ascadian`: the 64 m grid blocks around the viewer at startup, ground sampled at
  0.5 m from `TerrainGround` once ready, mask = `density_at` / peak. Test: headless block plan (blocks, mask = CPU twin).
  **Done 2026-11-23** (GC-3a2a-c, headless): `blades.rs` `blocks_around`, `sample_block`, `BladeStyles`, `BladeBlock`; game species `ascadian_meadow_blades` (style "meadow"), scene `gc-blades`; off by default (`groundcover.blades.enabled`).
- **GC-3b1** the crate vendored in `superfargoth/vendor/bevy_grass` with the world-offset patch (origin XZ in the field
  seed, the noise and the trample cell: today every origin shift reshuffles every blade and wipes the trails) and the
  adapter that feeds it and shifts the gust offsets. Test: the same blades before and after a shift. DECISIONS line.
  **Done 2026-11-23** (GC-3b1a-c): vendored; `GrassWorldOffset` + `field_seed` (world corner, whole cm), `world_offset` in the wind uniform for the gust and patch lookups; our `write_grass_offset`. Also own: surface-weather prepass view layout (`bevy_glue.rs`), found by the first GPU run.
- **GC-3b2** vendor patch: `View::origin_rebase` in the blade motion vector (no false motion on shift frames). Test: shader.
  **Done 2026-11-23:** `view_origin_rebase()` in `bevy_bridge.wesl`, added to the previous blade position.
- **GC-3b3** blocks follow the viewer (5×5 at High). Any field change re-plans and re-uploads every field, so the slice
  prints the rebuild time (`GrassStats::tables.last_rebuild`) in a measurement test; over ~1 ms it gets a spread-over-
  frames fix before the lane goes on. Entry cap: 256 = blocks × (1 + species), so ≤ 9 species per style at High.
  **Done 2026-11-23:** ring follows the viewer, one batch per move, `groundcover.blades.hysteresis_blocks` 1. Measured headless: table rebuild 5.2 ms per one-block move, 17.6 ms first plan: **GC-3b4** (incremental or spread rebuild in the vendored crate) is next.
- **GC-3b4** the table rebuild made incremental in the vendored crate. **Done 2026-11-23** (GC-3b4a diagnosis, b-d2,
  own patches listed in `superfargoth/vendor/README.md`): no echo rebuild on `AssetEvent::Added` (`changed_id`), raw byte
  image reads, slot-stable tables (best-fit free list, unchanged fields untouched, freed chunks skipped in `cull`, writes
  recorded per table), partial upload (`upload_plan` + `write_buffer`, Arcs released after upload). Headless: a move
  5.5 ms twice -> 0.08-0.43 ms once; in game `grass/rebuild_time` max 0.94 ms; `gc_walk` hitches equal with and
  without blades (`docs/ledger/perf.md` `gc_b4`). Tests: `grass_reads_tests.rs`, `grass_stable_tests.rs` (the vendored
  crate's own tests cannot run in the workspace; `bevy_grass::superfargoth_test` exposes the hooks).
- **GC-3g** blade density from the owner's look 1 ("could be denser"): `ascadian_meadow_blades` 60 -> 150 per m²
  (coverage 3 at 2 cm blades; no shipped game publishes a figure, `docs/research/grass-blade-density-2026-11-23.md`).
  The species density already sets the field style's density (`blades.rs` `style.density = grids.peak`). A blade-only
  live knob is a later slice if the owner wants to tune by eye.
- **GC-3c** wind bridge: `GrassPlugin { wind: false }`, our system writes `GrassWindFrame` in `GrassWindSystems::Sample`
  from the weather's `Wind` (direction, strength 0-1 from m/s, gust scroll at the wind speed); gust size, contrast, sway
  and flutter as live tunables. Test: the mapping, exactly. **Owner look 1** after this slice.
  **Done 2026-11-23:** `blade_wind_frame` + `sample_blade_wind` (`Update`, `GrassWindSystems::Sample`), crate wind off; tunables `groundcover.blades.wind_full_mps` 12, `wind_max_strength` 1.5, `gust_scale_m` 20, `gust_contrast_per_gustiness` 3.3, `sway`, `sway_speed`, `flutter` 1.0. Also `mark_grass_cameras`: `NoGrass` on every camera but an enabled viewer.
- **GC-3d** blades trampled by our trail ring (owner 2026-11-23: one source from the start, no interim on the crate's
  own interactor window): vendor patch, the blade shaders read the ring's press channel and the crate's own trample
  pass is off. After GC-T1 and GC-T2. Test: shader + the sampled press at a known texel.
- **GC-3s** vendor patch: blades cast only into the `GrassSun` cascades (today also the moon's and our `sun_view` rain,
  shelter and canopy maps); then `GrassShadows::Near`. Test: view filter.
- **GC-3e** Morrowind blade styles and rules for every grass channel (one field per style, two where regions meet); mesh
  species re-weighted as accents (data only). Terrain colour under the blade needs a vendor patch: its own slice after
  the owner has seen the per-style tints.
- **GC-3f** preset tiers and the live switch (`NoGrass` on the camera when `groundcover.enabled` is off). Test: switch.
- **Patch policy:** every vendor patch is written to be sent upstream (github.com/GEMI/bevy-grass) and listed in
  `superfargoth/vendor/README.md`.

**Mesh lane (accents; `sf_groundcover_bevy` mesh renderer)**
- **GC-M1** batches sized to demand (GC-P0: +1.0 ms everywhere for 658k reserved slots; GC-D1: the reserve likely pushes
  the 8 GB card over its VRAM budget, the bistable +25 to +80 ms). A batch opens when a resident tile first has instances
  of its model, starts small and doubles (the reservation resizes with a copy), closes when its last block frees; block
  size from the species' highest expected count plus a margin, the cap counter for overflow. The worst-case reserve
  stays as a switch (`groundcover.render.reserve_worst_case`, off) for the A/B. Test: capacity against live slots for a
  one-region ring, slots kept across a growth. Then GC-P0 again with the switch as an arm.
  **Done 2026-11-23** (headless): capacity / live 2.22x (was 11.13x) on a one-region ring; tunables `initial_blocks` 8, `block_sigma` 5, `reserve_worst_case` false. GC-P0b: fault and +1.0 ms floor gone, meadow 11.59 ms vs 10.46 off.
- **GC-M2** wind layers (below): per-instance sway and tip flutter scaled by the local wind, lull-to-front contrast from
  the gust envelope (`wind_gust`), live tunables, motion vectors still exact. Test: shader tests + naga compile.
  **Done 2026-11-23** (headless; awaiting owner, check GC-M2). As built: `wind_sample_gusty` / `wind_sample_prev_gusty`
  (`sf_wind_bevy/src/wind.wesl`, a gustiness floor; the plain functions call them with 0); sway, cross sway (0.63 × the
  sway Hz, uniform `sway_cross.x`) and tip flutter in `groundcover_common.wesl`; time is Bevy's `globals` (the prepass
  declares it itself, group 0 binding 1), each frequency quantised to whole cycles per hour so the hourly wrap is
  seamless (`quantise_hz`). Tunables `groundcover.render.gust_contrast` 0.7, `sway_amount` 0.35, `sway_hz` 0.9,
  `flutter_amount` 0.12, `flutter_hz` 3.5 (Guess, live).
- **One trample source** (owner 2026-11-23): snow, ash, mesh plants and blades all read `surface.trail` (mesh-view
  binding 52: 64 m, 6.25 cm texels, world-anchored, whole bodies through the deformer capture,
  `sf_surface_bevy/src/{trail,deform}.rs`). Two additions to the ring first, each its own slice:
  - **GC-T1** a press channel that springs back in seconds (snow prints stay): CPU twin in `sf_surface::trail` + the
    compute pass. Test: exact decay and stamp values against the twin.
  - **GC-T2** the stamps and the deformer capture also run when groundcover is in range (today only over snow or ash).
    Test: the gate.
- **GC-M3** mesh plants pushed down and aside by the press channel (vertex stage of the groundcover material). Test:
  push direction and amount from a known press, headless.
- **GC-M4** (order 2026-11-23: next after GC-3g, before the trample chain; covers blades too, since they read the
  same `density_at` with the overrides) exclusion under statics: placements' footprints as exclusion regions (the game passes `Overrides::none()`
  today, `mw_exterior_bevy/src/groundcover.rs:135`). Test: no instance inside a footprint.
- **GC-M5** `RangeClass::Mid` in the renderer (a second ring for small bushes) and whole models (every mesh of the NIF,
  normal maps). Test: ring membership per class.
- **Owner look 2** after GC-T1, GC-T2, GC-3d, GC-M2 and GC-M3: trampling by whole bodies on blades and clumps alike, accents moving with the carpet.

**End of the lane**
- **GC-V** pre-PR checks, dev build, the `gc-*` scenes launched for the owner, then one A/B: off / meshes only / blades
  only / both (the player's case), median + p99 + hitches per minute, against the 0.8 ms budget.
- **GC-4** streaming over the walk (tile blocks with the terrain rings), hitch check on the Seyda Neen → Balmora walk.
- **GC-5** explicit placements from mods (adapter), priority vs procedural.
- **GC-6** worldgen hook (WG-N3): rule table reads material fields on a generated world.

### Wind layers and trampling (rule 0 checks for GC-3c, GC-M2, GC-M3)

- **Bevy:** nothing native. **Crates:** `bevy_grass` has all three layers for blades (gust texture with `gust_scale`
  and `gust_contrast`, per-blade `sway`, tip `flutter`: `src/wind/mod.rs:180`) and a per-camera trample window (256² cells
  of 0.25 m, toroidal, world-anchored, ~0.02 ms: `src/trample.rs`). **Industry** (from memory, each slice verifies its
  source): main bending from a wind field plus detail bending per instance (Crysis, GPU Gems 3 ch. 16; UE
  `SimpleGrassWind`; Tsushima's global noise + per-blade sine); trampling from a camera-centred displacement texture
  stamped by characters (Tsushima; ours is the surface-weather trail ring `surface.trail`).
- **Why the mesh sway reads as uniform today:** the lean is `wind_sample` only. A gust cell is 60 m (the mesh range is
  60 m), gustiness 0.3 gives a lean of 0.7-1.3×, the log profile leaves a 0.4 m plant ~35 % of the wind, and nothing
  moves faster than the gust field drifts.
- **Two gust fields (accepted for v1):** `GrassWindFrame` is parametric (direction, strength, gust scale and scroll); the
  blades draw the crate's own gust texture, not `sf_wind`'s field, and ignore its local sources. The bridge keeps
  direction, strength and drift speed equal, so fronts move together but do not coincide with the accents' and trees'.
  If the owner sees the mismatch: an upstream PR for an external gust texture (rule 4), never a fork of the shader here.

### Blades: integration facts (GC-3R)

Read from `bevy_grass` 0.20.6 source (`bg/` = the crate root in the cargo registry), 2026-11-23. Not compiled or run.

| # | Fact | Verdict |
|---|---|---|
| 1 | `bevy` 0.20 with only `std, bevy_asset, bevy_log, bevy_render, bevy_core_pipeline, bevy_pbr` (`bg/Cargo.toml:171-181`), all inside our `3d` profile (`bevy-0.20.0/Cargo.toml:2725-2753`): no new Bevy, group [1] kept (run `bevy_variants.sh` in GC-3a). `bytemuck`, `ron` 0.12, `serde` already in tree. MIT. Builds on our vendored Bevy crates: unverified | as-is |
| 2 | One NonMesh item per camera in `Opaque3d`, `Opaque3dPrepass`, `Opaque3dDeferred`, `Shadow` (`render/mod.rs:54-57`, `render/bevy_glue.rs:572-830`), from per-camera compute before `camera_driver` (`render/mod.rs:90-95`). Our raster camera (forward + depth and motion prepass, `exterior.rs:1047`) gets prepass + forward; Solari gets the G-buffer (`bevy_glue.rs:682-714`). Motion vectors written (`grass_prepass.wesl:71-81`). `RenderLayers` 0-31 honoured (`fields.rs:352-360`); lamp faces untouched. Casts into **every** directional view with `cascade_index` < 1/2 (`bevy_glue.rs:792-830`): moon cascades and our `sun_view` maps (cascade 0, `sf_sky_bevy/src/sun_view.rs:282`: rain, shelter, canopy) | as-is; **patch**: shadows for the `GrassSun` cascades only |
| 2b | Previous blade ignores our `View::origin_rebase` (`vendor/bevy_render/src/view.wesl:97-100`): false motion on shift frames | **patch** (vendor-only): add it at `grass_prepass.wesl:81` |
| 3 | Fields follow `GlobalTransform` (`fields.rs:391-399`), so blades stay on the ground across a shift, but the seed hashes the corner's f32 bits (`fields.rs:1179-1182`): every shift reshuffles all blades. Clumps repeat at 64 m (`generate.wesl:211-215`, seamless); tufts at 64 × spacing (`:219-225`, seamless if it divides 4 m); bare and colour patches jump (`:228-234`, `look.wesl:40-42`). Gusts: `uv = root/gust_scale - gust_offset` (`wind.wesl:54`). Trails are cleared (`trample.wesl:31-38`). A shift rebuilds all tables (`fields.rs:328-334`). f32 precision fine | **adapter**: add `shift/gust_scale` to both offsets. **Patch**: world-offset resource (origin XZ mod 16384 m) in seed, noise and trample cell |
| 4 | `with_ground_from_fn` samples `f` once per cell centre at spawn (field-local, height above field Y, NaN = hole) into R32Float ≤ 4096² (`api.rs:154-180`, `ground.rs:28-30, 193-202`), on the calling thread. Changing the image rebuilds (`lib.rs:133-137`) | **adapter**: after `TerrainGround::ready`, sample 0.5 m in a task, pass a grid lookup |
| 5 | `with_density_from_fn`: Rg8 mask, same cells (`api.rs:208-222`, `packed.rs:101-118`). Blades/m² = `style.density × mask × settings.density_scale × falloff` (`generate.wesl:359-362`); tiers 0.6/0.8/1/1 (`settings.rs:218-288`). Species shares are fixed per style (`style.rs:141-160`). `GrassEdit` uploads touched rows only | **adapter**: `density` = peak (tunable), mask = `density_at`/peak; one field per style |
| 6 | 16 m chunks (`fields.rs:42`); ≤ 256 entries, field = 1 + species (`fields.rs:46, 798-804`). Any spawn/despawn/move re-plans all fields and re-uploads all maps (`fields.rs:1001-1083`, `render/resources.rs:402-435`), ~96 KB per 64 m field. 32 B × `instance_capacity` per camera (High 19 MB). Overlaps draw twice | **adapter**: 64 m grid blocks, 5×5 at High (100 m), 7×7 Ultra; hitch unmeasured (`GrassStats::tables.last_rebuild`) |
| 7 | Styles in code (`style.rs:30-160`) or `.grass.ron`; `GrassFieldOverrides { tint, density_scale, height_scale, wind_scale }` (`api.rs:250-266`). Colour = root/tip, one `ground_tint`, noise patches; no texture or position input | adapter; **patch** for terrain colour (GC-3e) |
| 8 | `GrassPlugin { wind: false }`; write the `GrassWindFrame` resource in `GrassWindSystems::Sample` (`Update`), `advance(dt, speed)` moves both offsets (`wind/mod.rs:180-256, 286-307`). `strength` 0-1 = still-storm. CALM 0.1/40 m/2 m/s, BREEZE 0.35/25/4, GUSTY 0.6/15/7, STORM 1.0/8/12 (`:88-127`) | adapter (GC-3c) |
| 9 | `GrassInteractor { radius, strength, max_height }` (`api.rs:474-500`), nearest `max_interactors` (High 64) in a 64 m window, radius ≤ 2 m. Trample buffer private (`render/view.rs:104-105`) | as-is; **patch** to share it |
| 10 | `GrassSettings` tiers (`settings.rs:218-288`), all live (`:12-16`); `GrassShadows` Off/Near/Full = 0/1/2 cascades (`render/shadows.rs:5-11`). Cheapest off: `NoGrass` on the camera (no buffers or compute, `render/view.rs:296`); `density_scale` 0 still runs trample and cull | adapter (`groundcover.enabled`) |
| 11 | `GrassPlugins` after `DefaultPlugins`, minus `FarGrassPlugin` and `GrassStyleFilePlugin` (`lib.rs:209-262`); field changes `.before(GrassSystems::DetectChanges)` (`lib.rs:117-178`); `GrassSun` on our sun (`sun.rs:1-8`). Vulkan and DX12 supported (README) | as-is |
| 12 | Opaque untextured blades; no tilted fields; 0.32 ms is a 4070 Ti at 1080p with 4× MSAA, no 4060 figure; struct literals break each 0.20.x (CHANGELOG 0.20.6) | measure in GC-P0 |

**GC-3a calls.** `GrassPlugins.build().disable::<FarGrassPlugin>().disable::<GrassStyleFilePlugin>()`;
`styles.add(GrassStyle { density: peak, ..GrassStyle::MEADOW })` (`style.rs:138`);
`GrassField::new(Vec2::splat(64.0), style, &mut styles)` (`api.rs:107`) `.with_ground_from_fn(&mut images, 0.5, h)`
(`api.rs:154`) `.with_density_from_fn(&mut images, 0.5, d)` (`api.rs:208`), spawned with the block centre's
`Transform` and the exterior `RenderLayers`; `GrassSun` (`api.rs:428`); `GrassStats` to check.

**Blockers.** None for one static block, with `GrassShadows::Off` until the shadow patch. Before GC-3b: the
world-offset and `origin_rebase` patches (crate vendored in `superfargoth/vendor/`). Unverified: build against our
vendored Bevy, rebuild hitches, whether `NoGrass` frees the buffers.
## Open questions

- Owner (look): blades only, meshes only, or both together in the same place: settled by owner verdicts on the GC-2/
  GC-3 scenes, not in advance.
- Mods' explicit groundcover vs procedural in the same cell (replace, add, or player choice): default "replace inside
  their region" until the owner says otherwise.
