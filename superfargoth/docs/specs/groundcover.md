# Groundcover: species, rules, runtime scatter

Status: **GC-1 core + GC-2a mesh renderer built (2026-11-22)**, not yet run on a GPU or in the game; GC-2b (game wiring) next. Crates (planned): `sf_groundcover` (Bevy-free core),
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
  function (`with_density_from_fn` or a baked mask per block), ground from our terrain heights, wind from `sf_wind`
  through `GrassWindFrame`, quality tier from our preset. Blades take their colour from the terrain layer under
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
- **Trampling:** one trample source per camera shared by both renderers (`bevy_grass`'s interactor texture, or the
  surface-weather trails ring W6 if that's the cheaper shared source; GC-3 decides).

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
- **GC-2** mesh renderer (`sf_groundcover_bevy`, ~600-900 lines, guess):
  - Tile ring manager around the camera, with hysteresis.
  - Task scatter from the GC-1 rules plus terrain `height_at`, written into a fixed tile-block slot pool, one
    `GpuBatchedMesh3d` per (model, material); several models share a batch through ranges when they share slab and
    bind group.
  - `ExtendedMaterial<StandardMaterial, GroundcoverWind>`: alpha mask, height-weighted bend from `sf_wind`,
    prepass previous-wind motion vectors.
  - Near batch casts shadows; the rest is `NotShadowCaster`.
  - Origin shift through the statics' rewrite, and live/capacity slot gauges.
  - Test: deterministic scatter and a valid slot layout (headless), plus an `#[ignore]` measurement printing
    instances and bytes per tile.
  - Then the adapter's first species set on one cell: a test scene registered (`visual-checks/groundcover.md`,
    "awaiting owner") and the cost measured (untraced A/B, `ledger/perf.md`).
  - Out of scope: GPU producer, visibility-range patch, LOD meshes.
- **GC-3** `bevy_grass` blade species: dependency added (hakari, `bevy_variants.sh` group [1]), wind bridge, terrain
  colour, tier mapping; same scene + A/B.
- **GC-4** streaming over the walk (tile blocks with the terrain rings), hitch check on the Seyda Neen → Balmora walk.
- **GC-5** explicit placements from mods (adapter), priority vs procedural.
- **GC-6** worldgen hook (WG-N3): rule table reads material fields on a generated world.

## Open questions

- Owner (look): blades only, meshes only, or both together in the same place: settled by owner verdicts on the GC-2/
  GC-3 scenes, not in advance.
- Mods' explicit groundcover vs procedural in the same cell (replace, add, or player choice): default "replace inside
  their region" until the owner says otherwise.
