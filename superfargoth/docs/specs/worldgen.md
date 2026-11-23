# World generation (`sf_worldgen`, the Scrib worldgen tool)

Status: **game-renderer chain in progress** (2026-11-19: generated island streams as game terrain, GPU detail fill; look loop 2026-11-18). Built: research WG-R1-R4, `sf_worldgen` (stream power, Veloren LEM
port, layout, landscape, stratovolcano, analytic erosion filter, DEM import, measure; bin `scrib-worldgen`), viewer
`bevyrrowind worldgen-view`; outputs and owner verdicts in `docs/ledger/worldgen.md`. **Next: the screenshot
milestone** ("Handoff" at the end). This spec is agreed at WG-0.
Crates (planned): `sf_worldgen` (Bevy-free core), `sf_worldgen_gpu` (wgpu compute, no Bevy), tool binary
`scrib-worldgen`. Feeds `sf_terrain::TerrainSource` ([terrain](terrain.md)). First game adapter (Vvardenfell at 10×,
anchor rules, coordinate remap, its slices): `../../../bevyrrowind/docs/specs/worldgen-adapter.md`.

<!-- agent-summary -->
> **Agent summary.** A generic, deterministic, seeded world generator, shippable as its own tool. It **generates
> nature between kept human-made content**: anchors (settlements, wrecks, tombs, mines, shrines) keep their
> hand-made footprint at 1:1, and the land between them is regenerated at scale S (Vvardenfell: S = 10).
> Inputs are generic *guides* (intent heights, region map, strokes, anchors, water); a stage graph turns them
> into heights and fields: uplift + stream-power erosion (macro, ~25 m), hydraulic + thermal erosion and
> landform operators (mid, 2-4 m), anchor stamps, hydrology, material fields. Below 2 m, detail is synthesized
> at runtime by one deterministic function (GPU clipmap fill + CPU twin). A generated world may ship baked (owner
> 2026-11-18); raw source data never ships. Read "Goal", "Pipeline", your slice, and the "Handoff".
<!-- /agent-summary -->

## Goal

Owner (2026-11-18):

- **A world generator for the framework**, generic, not tuned to Morrowind. It is a separate tool in the Scrib
  tools suite and may ship on its own later (like [scrib-motiongen](https://github.com/scrib-tools/scrib-motiongen), split out 2026-11-23).
- **Keep the human-made parts.** Settlements, shipwrecks, ancestral tombs, mines, shrines, camps: everything
  hand-made stays. Generated space goes *between* them. Procedural generation tends towards bland worlds nobody
  wants to play in (Starfield, No Man's Sky); the authored bits carry the interest, and generated land serves them.
- **Nature first**, then "we'll see". A settlement generator that grows towns along roads comes later, or from
  mods; not now.
- **Terrain that feels true:** real geology, drainage and landforms instead of Morrowind's blobby shapes. The
  blobbiness has two causes, and both are in scope: no drainage structure, and the coarse ~1.8 m grid smoothing
  every feature.
- **The biggest scale: 10×** linear for Vvardenfell. Red Mountain becomes a real mountain.
- **Generic sources and landforms:** Vvardenfell first, then Tamriel Rebuilt data (other environments), and in the
  end whatever people want to generate (fjords, tepuys, deserts, karst, ...).
- **Earth curvature as a rendered illusion**, not a spherical world (a sphere would change the whole architecture).

Non-goals for now: spherical planets, settlement or road generation, interiors, NPCs/quests, generating the world
during play.

## Constraints (CONTRIBUTING applied)

- **Framework/game split:** `sf_worldgen` never names Morrowind formats or ids. The adapter turns game data into
  guides, and recipes name landforms, not games.
- **A generated world may ship baked (owner 2026-11-18; was "data never ships derived"):** a 10x world is
  transformed enough to count as a derivative work, like a mod or Tamriel Rebuilt, so its baked heights may be
  distributed (the whole island at 30 m is ~8 MB; finer detail is runtime). Source data itself (the game's `LAND`
  records, raw dumps) still never ships. A world recipe can still run on the player's install; when it does, the
  run must fit the bake budget ("a few minutes, once", on the 4060 laptop). Author-side iteration may be slower per
  run, but its preview must be interactive. A shipped bake also settles cross-machine equality: everyone loads the
  same file.
- **Deterministic:** same recipe + same sources = same heights on every machine (saves store positions, nav
  bakes from heights, co-op host and guests must agree). Products are content-hashed; co-op compares hashes.
  WGSL doesn't promise bit-equal floats across GPUs, so (WG-R1): the macro stage runs on the CPU (5.8 M cells,
  seconds); mid-stage GPU kernels are integer fixed-point gathers with an identical CPU reference, so the hash is a
  true equality. At bake start a reference tile is compared, and a GPU that fails falls back to the CPU path (slow,
  correct). Float GPU math is allowed only in runtime detail, which never feeds saves or nav.
- **Tunables:** every generator parameter is a recipe value or a declared tunable with unit, range and source
  ([tunables](tunables.md)). S itself is a tunable (1 = classic layout, 10 = the owner's target).
- **Compile time:** logic Bevy-free; GPU kernels in their own crate on wgpu, each with a CPU reference.
- **Everything is a mod:** recipes are `sf_records` records; products enter the game only through `TerrainSource`
  and the normal bake (`sf_bake` content-hash cache).

## Scale and numbers

Sized on the first target (adapter spec): ~5 × 5 km of source land at S = 10 is ~50 × 50 km, about a 60 km square
with the sea around it.

| Spacing | Samples (60 km square) | f32 size | Use |
|---|---|---|---|
| 25 m | 2,400² = 5.8 M | 23 MB | macro erosion: seconds |
| 4 m | 15,000² = 225 M | 0.9 GB | mid erosion, tiled |
| 2 m | 30,000² = 900 M | 3.6 GB | finest baked level |
| 0.5 m | 120,000² = 14.4 G | 58 GB | never baked: runtime synthesis |

Consequences:

- **Terrain storage changes** ([terrain](terrain.md) "Data model": base spacing 0.5 m, level 0 baked). At 10× the
  bake stores levels from ~2 m up, and the levels below are synthesized at runtime (WG-4, WG-5). This is a terrain
  spec change; it keeps "every result is a function of global sample indices".
- **Vertical relief: a per-region factor, default = S.** Measured (adapter "Source numbers"): a uniform ×10 already
  turns the 270.8 m summit into 2.7 km (Etna 3.3 km, Fuji 3.8 km), so the blobs come from missing drainage and the
  coarse grid, not from the slopes. Recipes may still remap a region's relief (e.g. a stratovolcano's concave
  profile, flatter lowlands); the look test compares uniform ×10 with the remapped arm.
- **Curvature:** the drop is d²/2R: ~200 m at 50 km, ~785 m at 100 km (R = 6,371 km, a tunable). Gameplay stays on
  a flat frame; terrain, water, statics, far field and the far shadow map apply the same view-relative drop (WG-6).
- **View distance:** the clipmap already reaches ±65 km (10 levels; level 9 = 256 m × 512 quads). The far shadow
  map reach (6 km), the water skirt (32 km) and aerial perspective need re-checking at scale (WG-7).

## Pipeline

```
game data ──adapter──> guides ──stage graph──> products ──TerrainSource / bake──> game
                       (generic)  (seeded,      (heights, fields,
                                   hash-cached)  water bodies, anchors, scatter)
```

**Guides** (generic inputs, all optional, any resolution):

- *intent height*: a coarse height the generator respects as uplift/target, not as final terrain;
- *region map*: a region id per sample → one recipe per region (landform, geology, climate);
- *strokes*: ridges, rivers, coastlines, cliffs, as polylines with weights;
- *anchors*: footprint polygon + local height field + transition band + priority + the objects they carry;
- *water*: sea level, lakes, named rivers that must exist;
- *exemplars*: optional real or authored DEM patches whose statistics drive detail (licences per WG-R1).

**Stages**, each a pure function of its inputs and the seed, cached by content hash:

1. **Placement:** anchors move to S × source position and keep their 1:1 local frame. The intent height is scaled
   by S horizontally and remapped vertically per region. The **anchor transform** (source position → world
   position: anchor offset inside a footprint, ×S elsewhere) is a product too, because door destinations and
   scripted coordinates go through it (adapter WG-A3b).
2. **Macro form (~25 m):** uplift + fluvial erosion by the stream power law (erosion ∝ Aᵐ Sⁿ: drainage area,
   slope), with the uplift and erodibility fields from the recipes. **Anchors are boundary conditions here**, not
   patches applied afterwards: their height and slope are fixed (C1), named rivers are forced as channels, and the
   coastline is kept. Anchor terrain takes part in erosion as context and is restored in stage 4. This gives
   dendritic valleys and sharp ridges, the "true" look. (Procedural Tamriel patched a finished field after the fact,
   and every structure-aware continuation it tried was rejected visually; WG-R3 §3.)
3. **Mid detail (2-4 m, tiled with overlap):** hydraulic erosion (pipe model) + thermal erosion (talus, scree),
   plus **landform operators** from the recipe: lava channels and flow fronts, cap-rock terraces (tepuy, mesa),
   glacial U-valleys and fjords, dunes, karst sinks, deltas. Landforms are data; a new one is a new operator + recipe.
4. **Anchor stamps:** each footprint gets its source heights back (upsampled at 1:1, bit-exact inside), blended
   across its transition band. Baseline band (WG-R3): a harmonic (Laplace) bridge plus a contour-normal clone, a C1
   smootherstep collar, and one outer fade to exactly zero, with distances from a harmonic or smoothed field, never
   the nearest-seam distance transform (it jumps at kinks). The unrestored field and the blend weight are cached, so
   retuning the collar width doesn't re-run erosion. Avoid template or mirror continuation and per-operator
   footprints: all were rejected with evidence.
5. **Hydrology:** flow accumulation → rivers and lakes as `sf_water` bodies at their own heights; wetness and
   moisture fields. Stream power never makes meanders or braids, so rivers get their own centreline stage
   (meander simulation, Paris 2023) before they are carved.

**Irregularity is designed in (WG-R4):** uniform rock and uplift run to a strict steady state self-organise into
evenly spaced ridges and valleys (Perron et al. 2009), the "motherboard" look the owner rejected. The layout enters
as **uplift and erodibility fields** varying per cell (rock types, region edges, cellular noise), never as a target
height; runs start noisy and stop at a finite time, not at strict equilibrium.
6. **Material fields:** slope, curvature, flow, wetness, altitude, rock type, exposure → splat inputs (the adapter
   maps them to its layers: LTEX or pack textures) and scatter density fields.
7. **Runtime detail (< 2 m):** one stateless detail function guided by the material fields: an analytic
   gradient-aligned erosion filter plus integer-hash noise, each point evaluated in isolation (WG-R1), evaluated on the GPU when clipmap levels fill and by a CPU twin for height queries,
   physics and nav (the water "wave twin" pattern). Not a bake during play: nothing is stored.
8. **Nature scatter** (later stage): rocks, trees, flora from rules learned from the source's own placements per
   region, Horizon-style rule-based placement. Method (WG-R3 §6): occupancy tables measured on the source's own
   terrain (per ~1.8 m cell: slope, elevation, relief, curvature, layer), Bernoulli draws with exponential-race
   ordering, poses replayed in the terrain frame, objects sized by bounding box, a failed placement means absence;
   per-region libraries with an explicit fallback.

**Stage rules (WG-R3):** no stage creates or removes water except hydrology. Every stage runs a wet/dry audit (no
new or dried water is a test), and every mask is rebuilt from the current field, never reused from an earlier stage
(a stale mask once caused a false 14 km uplift there). No sea-level clamping. Noise organises structure (control
fields, domain warp), never lies on top as a visible blanket. Noise and filters sample absolute world coordinates;
tiles compose by partition-of-unity cross-fades.

## Anchors: keeping the human-made parts

The framework sees an anchor as footprint + local heights + objects + transition band + priority. **Which source
objects are human-made is the adapter's call**; the framework clusters what the adapter marks (radius a tunable),
builds footprints, and drops the rest as nature to regenerate. Generic hard cases: long linear structures (walls,
fences: re-routed, not stamped), roads (re-routed later), and anchors on slopes whose new surroundings differ
(terracing or retaining in the stamp).

**Coordinates that move with anchors:** the anchor transform is a query API (`source → world`). Adapters route
every source coordinate through it (door destinations, scripted positions, travel, saves), so source game data
stays unmodified.

## Crates and tool

- `superfargoth/crates/sf_worldgen`: guides, stage graph, recipes (`sf.worldgen.recipe` records), CPU stages, the
  CPU detail twin, preview products (hillshade, region and flow panels as PNG). Bevy-free, cold build in seconds.
- `superfargoth/crates/sf_worldgen_gpu`: wgpu compute kernels (erosion, detail) on the workspace's pinned wgpu,
  CPU reference for each.
- **Recipes are data over a closed operator registry:** mods ship parameters, strokes, graphs and recipes, not code.
  Every operator ships with the framework (GPU kernel, CPU twin, hash test); a reviewed plugin API may come later.
- `scrib-worldgen`: CLI (`run <recipe>`, `preview`, `measure`), later an editor UI with painting and strokes.
- Game adapters write guides, mark anchors and hold their region → recipe table (Morrowind: `mw_worldgen`).
- Game: a `TerrainSource` over worldgen products, plus a world selection (`--world <recipe>`).

## Prior art

Game-modding prior art (Procedural Tamriel, Distant Landscapes of Tamriel) is listed in the adapter spec; the
Procedural Tamriel reading (WG-R3, cited above by section) is `../../../docs/research/procedural-tamriel-2026-11-18.md`.
Godotwind erosion/hydrology (WISHLIST §3-4) is an anecdote, not a design.

## Reuse check (WG-R1, 2026-11-18; `../research/worldgen-survey-2026-11-18.md`)

- **Bevy 0.20 / wgpu:** Bevy has no terrain generation. `sf_worldgen_gpu` uses standalone wgpu compute at the
  workspace's pinned wgpu/naga (30.0.1), so it adds no second wgpu build and no Bevy dependency.
- **Crates:** `fastlem` rejected (MPL-2.0, v0.1.4 of 2024-01, repo archived 2025-06, graph-based, no uplift input).
  `bevy_erosion_filter` rejected (MPL-2.0, Bevy-bound); its technique is reimplemented. `noise` / `fastnoise-lite`
  (MIT/Apache, stalled since 2024) don't give a GPU twin, so we write a small integer-hash gradient noise with
  identical CPU and WGSL versions. No maintained wgpu erosion crate exists. GeoTIFF exemplars through the `tiff` crate
  (to confirm at WG-1).
- **Ports (WG-R4, `../research/worldgen-oss-survey-2026-11-18.md`):** Veloren `world/src/sim/erosion.rs` (Rust,
  GPL-3.0: implicit stream power, multi-receiver flow, Cordonnier lakes, diffusion, per-cell erodibility and uplift)
  is the stage-2 base, ported with a fixed summation order, anchor boundary conditions and tiling; fastscapelib
  (GPL-3.0) for the basin-graph sink resolver and the n ≠ 1 Newton step, its Python `fastscape` as an offline
  reference oracle; Paris 2023 `Meandering-rivers` (MIT) for a river-centreline stage. Each port cites file + commit
  and gets a `THIRD_PARTY_NOTICES.md` entry. Not usable: HighMap (LGPL-2.1 vs GPL-3.0 conflict, reimplement),
  SimpleHydrology (no licence file), learned sketch-to-terrain models (no weights or licence, or CUDA only).
- **Own code, each citing its source:** stream power on a grid (Cordonnier et al. 2016; Schott et al. 2023, MIT
  reference code), pipe-model hydraulic erosion (Mei et al. 2007) and thermal erosion in fixed point, and an analytic
  gradient-aligned erosion filter for runtime detail (runevision 2026; Grenier et al., CGF 2024), reimplemented from
  the published descriptions; the MPL code is not copied.
- **Learned amplification:** Terrain Diffusion (arXiv 2512.08309, MIT code and weights) evaluated: CUDA/PyTorch,
  30-90 m per pixel, no timings, no determinism. Not adopted; it may become an import path for intent heights later.
- **Tools:** Gaea, World Machine and Houdini are import sources for intent heights, never dependencies.
- **Data:** SRTM and ETOPO are public domain (unverified); Copernicus GLO-30 has its own licence with attribution, and
  redistribution of derived patches is unconfirmed. Recipes ship **measured statistics**, never DEM tiles.

## Industry comparison (WG-R1; V = source read, U = unverified)

| Technique | Who | Us |
|---|---|---|
| Uplift + stream power for the large form | Cordonnier et al. 2016 (V); Schott et al. 2023 (V, interactive, MIT code); Gaea / World Machine erosion | match: stage 2, own grid implementation, CPU authority |
| Hydraulic + thermal erosion for mid detail | Mei et al. 2007 (U); Gaea Erosion2 on GPU, Houdini HeightField Erode (U) | match: stage 3, fixed-point wgpu kernels + CPU twin; vendors themselves treat GPU determinism as open |
| Authored content first, procedural between | Ghost Recon Wildlands, GDC 2017 (V): World Machine on a ~80-PC farm for the base, Houdini for roads and rivers, "procedural and manual don't mix"; Far Cry 5, GDC 2018 (V): procedural biome, water, cliff and texturing tools, not heights | match: anchors; the generated base is locked and hand edits are a separate layer on top |
| Rule-based placement | Horizon Zero Dawn, GDC 2017 (V): runtime GPU placement of objects (not terrain) | stage 8 / WG-N2 |
| Runtime amplification of a coarse base | Outerra: DEM + GPU fractal refinement (secondary sources); Grenier et al. 2024 real-time erosion patterns; runevision 2026 analytic filter | match: stage 7, analytic erosion filter + noise, GPU with CPU twin |
| Learned amplification | Guérin et al. 2017 cGAN (U); Terrain Diffusion 2025 (V): CUDA, 30-90 m | differ: not for the bake (stack, resolution, determinism, data provenance) |
| Curvature | the "world bending" vertex trick (Animal Crossing, Blightbound); planet engines use true spheres; no realistic-scale open world found doing it | flat gameplay, world-space drop about the camera, applied to shadows and effects too |
| Field overview | Galin et al., Eurographics STAR 2019 (U, to read) | read at WG-0 |

## Co-op

Generation runs at bake time, never in a tick. Host and guests bake the same recipe against their own installs and
compare product hashes before joining. The anchor transform is data, so positions in commands and saves stay
absolute.

## Build plan

IDs `WG-*` across both specs (framework slices here, game slices `WG-R2`, `WG-R3`, `WG-S*`, `WG-A*`, `WG-G2` in the
adapter spec); visual checks use the same IDs in `bevyrrowind/docs/visual-checks/worldgen.md`.

**Order:** WG-R1-R3 → WG-S1-S3 → WG-0 → WG-1-3 with WG-A1-A2 → WG-4, WG-5, WG-8 → WG-A3-A4 → WG-6, WG-7, WG-A5, WG-N1-N3 →
WG-G1, WG-G2.

**Owner priorities (2026-11-18), in order:** (1) a landscape the owner likes (look loop in the viewer: WG-VO, live
tuning); (2) settlements where they belong (anchors: WG-A1, WG-8, WG-A3); (3) textures on the ground (WG-N1: material
fields → Morrowind LTEX); (4) rocks and trees on the terrain (WG-N2; **one tree species is fine to start**, on the
existing static scene + far field). Millions-of-trees vegetation is parked (WISHLIST §17).

**Research (no builds; can run beside other work):**

- [x] **WG-R1** reuse + industry survey (2026-11-18): `../research/worldgen-survey-2026-11-18.md`; folded into
  "Reuse check", "Industry comparison" and the constraints. Left: Distant Landscapes of Tamriel (Nexus blocks
  fetching; needs a browser), the Galin STAR, Schott and Grenier papers read in full.

**Spec:** **WG-0** agree this spec and the adapter's after R1-R3 and the S2 look test: stage formats, determinism
authority, budgets, storage.

**Foundation (`sf_worldgen`):**

- **WG-1** core: guide types, stage graph, hash cache, recipe records, preview CLI. Test: the same hash on two runs
  of a fixture.
- **WG-2** macro stage (stream power, constraints). Test: a fixture drains to the sea without pits, deterministic.
- **WG-3** mid stage: hydraulic + thermal on wgpu with a CPU reference, seamless tiles. Test: CPU and GPU agree
  within tolerance; tile edges equal.
- **WG-4** terrain storage at scale: `sf_terrain` bakes from a coarser finest level; bake size and time measured.
- **WG-5** runtime detail: GPU clipmap fill + CPU twin for `TerrainHeightsMap`, physics, nav. Test: twin vs GPU.
- **WG-6** curvature drop on terrain, water, statics, far field, far shadow map; R a tunable. Visual check.
- **WG-7** view distance at scale: far shadow reach, water skirt, aerial perspective; measured ceiling first (perf).

- **WG-8** anchors: clustering, footprints, stamps with transition bands, the anchor transform query. Test:
  stamped heights equal the source inside footprints bit for bit; the transform round-trips. Owner look arms: the
  boundary-condition band vs the harmonic-only baseline.

**Nature:** **WG-N1** material fields → terrain layers through the adapter's mapping · **WG-N2** scatter rules
fitted to the source's own placement statistics, streamed like statics (HLOD/impostors via `sf_lod`) · **WG-N3**
groundcover hook (ROADMAP Groundcover).

**Generic proof:** **WG-G1** a world from a sketch alone (painted regions + strokes: a fjord coast and a tepuy
plateau) as a framework example, with no game data.

**Later (not planned):** road re-routing, the Ghostfence ring, settlement growth along roads, the editor UI, the
travel network at scale.

## Open questions

WG-R1 proposed answers, now in the constraints and pipeline; WG-0 confirms them: determinism authority (CPU macro,
fixed-point GPU mid with CPU twin), runtime detail (analytic erosion filter, no learned model), recipe format (data
over a closed operator registry).

1. Mid-stage throughput: can fixed-point pipe-model erosion over ~225 M samples at 4 m fit the bake budget on the
   4060 laptop? Measured at WG-3; if not, the finest baked level moves to ~8 m and stage 7 carries more.

## Handoff (2026-11-19)

Day summary: `../../../docs/reviews/worldgen-2026-11-18.md`; older handoffs: `archive/worldgen-history.md`.
Owner direction: any terrain is fine for now (working island `target/worldgen/vo/V3`); build the deterministic
procedural chain on it in the **game renderer**. State: `bevyrrowind view "world:<guide dir>"` streams a guide as game
terrain (`sf_terrain::TerrainTiles` provider, `sf_worldgen::tiles::GuideTiles`); G2 fills the clipmaps on the GPU
(`sf_worldgen/src/tiles.wgsl`, mirror of `filter.rs`, 8 runtime octaves 160 m → 1.25 m), CPU twin on 2 threads for
queries and physics, `--off gputerrain` = the G1 CPU-tile arm, console `worldgen.filter` / `worldgen.octaves`.
**Next:** owner look at G2 (detail, filter toggle, hitches vs `--off gputerrain`, plus Seyda Neen unchanged), then
trees (scatter stage → existing bake/static scene/far field, counts in steps), ground textures (removes
`FALLBACK_LOOK`), clouds/atmosphere tuning, settlements. Limits: see the day summary "Known issues".
