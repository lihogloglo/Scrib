# Procedural world tool suite: pipeline, ecosystem, industry, gaps (research-1, 2026-11-26)

Research only (no builds, no code). V = read in source or doc this session; U = unverified (search snippet, memory, or secondary).
Paths: `SF` = `superfargoth/crates/sf_worldgen/src/`, `MW` = `bevyrrowind/crates/mw_worldgen/src/`, `SW` = `D:\Gamedev\scrib-worldgen\`.
Not compared against `superfargoth/docs/research/worldgen-survey-2026-11-18.md` (not opened, per brief).

## 1. Summary

- The **terrain half is mature** (macro stream power, landforms, geology, scarps, sharp runtime filter, anchors, height curve HC-1). The **life half is thin**: no climate/biome fields, flora and rocks are the source's own placements replayed per region (`SF/scatter.rs`, `MW/flora.rs`), no points of interest beyond the kept anchors, no names, no ground-material rules (layers are learned from the source).
- Ranked gaps for the 10x world (about 100x the area, same ~630 authored anchors): (1) ecology-driven forest/rock scatter (clustering, size spread, self-thinning, moisture/slope/aspect driven), (2) the Magna Urbis water chain made the default guide stage (rivers carved, Balmora astride the Odai), (3) wilderness sites (POIs: camps, ruins, shrines, mines) placed by MU activity/remote-site data plus kit prefabs, (4) climate fields (temperature lapse, rain shadow, moisture) as the shared input of scatter, splat and MU's forest field, (5) procedural ground-material rules from material fields, (6) preview/override/per-stage-cache loop, (7) names, (8) reachability validation of anything placed.
- Canonical techniques: stream power + hydraulic/thermal erosion (have), ecosystem/competition-based placement with rules and density maps (Horizon GPU placement; Far Cry 5 Houdini rules; Wildlands biome ecosystems; UE5 PCG + Biome Core), Whittaker-style climate from lapse rate and orographic precipitation, tiled deterministic bakes with per-stage caches (Gaea Tiled Builds, Houdini PDG), POI placement as constraint-based kit placement with a reachability check (Valheim distance/biome rules, Diablo IV tilesets; Skyrim radiant is the anti-example).
- First slices: WG-E1 climate fields; WG-E2 scatter v2 (clustering + size + competition) with a headless statistics test; WG-W1 water chain as `worldgen-x10` stage; WG-P0 site-table spec then WG-P1 MU remote/activity sites export; WG-T1 stage cache and "arms contact sheet" loop.
- Owner questions (section 9): climate source, exterior-only POIs first, generated names, forest density vs sight lines.

## 2. Our pipeline today, stage by stage

Spec: `superfargoth/docs/specs/worldgen.md` (Pipeline, stages 1-8; build plan WG-0..N3/G1). Evidence: `docs/ledger/worldgen.md` (68 lines). Lane state: `.sessions/world10x.md`.

| Stage | Status | Where | Ledger evidence |
|---|---|---|---|
| Guides/adapter (heights, regions, strokes, layers) | built | `SF/guide.rs`, `layers.rs`; `MW/x10.rs`, `classify.rs`, `layers.rs` | WG-X3: 131 layers, 1.95 MB, 0.11 s |
| Anchors (human-made kept, stamps, transition band) | built | `SF/anchors.rs`, `stamps.rs`; `MW/anchor.rs` | WG-X2: 631 anchors (278 stamped), 4.16 M stamp samples 3.6 s |
| Height curve (sea-level anchored) | built, uncommitted | `SF/height_curve.rs`, `bevyrrowind/worldgen/x10-heights.toml` | HC-1: Balmora plain 6-8 m over Odai (was 24); land < 5 m 5.0 % |
| Macro form (stream power, MFD, thermal, Veloren port) | built | `SF/stream.rs`, `mfd.rs`, `landscape.rs`, `thermal.rs`, `flood.rs` | S2/S4/S5: 113 s for 4 arms on 6.4 M samples; "motherboard" look rejected, uplift/erodibility fields fix it |
| Landforms, geology, scarps, volcano, cliffs | built | `SF/landform.rs`, `geology.rs`, `cliffs.rs`, `placed/`, `volcano.rs`, `stratovolcano.rs`, `valley_terrace.rs` | LF-1/2/2b, WG-VO, Option A; owner "amazing" on rock families, scarps iterated |
| Mid detail (pipe-model hydraulic, 2-4 m tiled) | **not built** (WG-3 open) | spec only | runtime filter stands in |
| Runtime detail (analytic erosion filter, GPU + CPU twin) | built | `SF/filter.rs`, `tiles.rs`, `tiles.wgsl` | G2: GPU vs CPU 1.2e-4 m; sharp ops default off since 2026-11-22 |
| Hydrology: rivers, lakes | **prototype** via MU `worldgen-water` (SW `crates/water`), reader `SF/water.rs` | WR-1: both/breach/none arms; 3.2 M of 3.3 M land samples lowered, Red Mountain crater silted +515 m (needs anchors-aware masking) |
| Roads (regional) | **prototype** via MU (SW `crates/roads`), `SF/roads.rs` reader | WR-2: 8.8 s, 130 segments, 32 bridges; 2 sites unreachable before HC-1, 22 bridges / 2 unreachable after |
| Settlements | prototype via MU (SW `crates/settle`) on Balmora 2.0 | density arms, V-1 vendoring (59 tests, byte-identical output) |
| Material fields (slope, curvature, flow, wetness, rock) | partial | `SF/measure.rs`, `geology.rs`, `layers.rs`; shader `surface.wesl` slope/curvature | X7a hex tiling, X7b cliff rock; ground layers still source-learned |
| Nature scatter | built (v1), weak look | `SF/scatter.rs`, `MW/flora.rs`, `flora_towns.rs` | WG-X5: 7.21 M in 6.3 s; owner look 2026-11-19: world empty, trees not snug, rocks one size, "sprayed like bits on a donut", 26 fps at full density |
| Rocks | built | `sf_rockgen` (boulders RG-1..4, libraries RG-2b, formations RG-8a) | classes, 0.59 s for 9 classes x 16 |
| Groundcover (blades) | spec/lane | `groundcover.md`, GC-* | separate lane |
| Big-world instancing (millions of trees) | roadmap, all `[p]` | `docs/roadmaps/bigworld.md` BW-1..10 | parked |
| Climate / biomes | **missing** | grep: no biome, rain-shadow or lapse code in sf_worldgen scatter paths; MU has climate only as two numbers + forest field | n/a |
| POIs / dungeons / wilderness sites | **missing** (anchors only = authored) | n/a | "world still empty" |
| Names | **missing** | WG-L2a census joins source names only | n/a |
| Terrain ML | experiment, no adoption | `tools/terrain-diffusion/` (MIT code + weights, SIGGRAPH 2026); TD-1..3 | whole island ~23 s on the 4060, lowfreq blurs; stock model loses shape (summit 5493 -> 1685) |
| Preview/viewer | built | `bevyrrowind worldgen-view`, `view world:<guide dir>`, `scrib-worldgen preview/measure/landforms` | owner look loop works |
| Stage cache | partial | `SF/run.rs` hashes final heights (blake3); spec WG-1 "hash cache" per stage not confirmed by me (U) | n/a |

MU state in `SW` (V, `SW/docs/research/magna-urbis-capabilities-2026-11-24.md`): vendored rev 21e2217 under `vendor/magna-urbis` (AGPL-3.0-only, trimmed workspace, VENDOR.md patch log). We call 2 of ~12 urban stages, 4 fields of the building request, the water crate, the roads crate. 28-row capability table; 13-slice adoption order (C-2..C-14) already written; most options reachable through `[magna_urbis.layout]`/`[magna_urbis.parcels]` TOML with zero code; data files are `include_str!`-compiled, so a culture is a vendored-file edit.

## 3. Bevy / Rust ecosystem per stage (crates.io API + clones, 2026-11-26)

| Stage | Crate | Version / updated | Licence | Bevy | Verdict |
|---|---|---|---|---|---|
| Noise | `noise` (noise-rs) | 0.9.0, 2024-03, last commit 2025-02 (V clone) | Apache-2.0/MIT | none | stale but stable; we use integer-hash noise in `SF/perturb.rs` (deterministic, GPU twin) so no need |
| Noise | `libnoise` | 1.2.0, 2025-04 | MIT | none | maintained; optional for tools |
| Noise | `fastnoise-lite` | 1.1.1, 2024-03 | MIT | none | simple, ports to WGSL; optional |
| Noise SIMD | `simdnoise` | 3.1.6, 2020 | MIT | none | abandoned |
| Blue-noise scatter | `fast_poisson` | 1.0.2, 2025-03 | MIT/Apache | none | Bridson, variable-radius via distance function; fits scatter v2 only if tile-local determinism is kept (U: not read); hash-grid dart throwing in our own code is simpler |
| Blue-noise | `poisson-diskus` | 1.0.0, 2021 | (not checked) | none | abandoned |
| WFC | `ghx_proc_gen` + `bevy_ghx_proc_gen` | 0.10.0, 2026-10-13 (V clone, commit 520c9e8) | MIT OR Apache-2.0 | **bevy 0.20** | maintained, grid WFC 2D/3D, rules/weights/socket model; right tool for dungeon/kit/tile layouts (POI interiors, ruin footprints); not for terrain or settlements |
| WFC | `wfc` | 0.10.7, 2022 | (not checked) | none | stale |
| L-systems | `lindenmayer` | 0.3.0, 2023 | (not checked) | none | stale |
| Trees | `bevy_procedural_tree` | 0.4.0, 2026-07-09 | MIT OR Apache-2.0 | bevy ^0.19 (not 0.20) | U: Ez-Tree style meshes; our tree art is Morrowind meshes + impostors (BW lane), so low value |
| Voronoi/Delaunay | `spade` 2.15.1 (2026-03), `delaunator` 1.1.0 (2026-05), `voronoice`/`voronator` (stale) | MIT/Apache | none | spade for region/site graphs |
| Geometry | `geo` 0.33.1 (2026-04) | MIT/Apache | none | polygon ops for footprints |
| Graph | `petgraph` 0.8.3 | MIT/Apache | none | road/site graphs |
| Names | `markov` 1.1.0 (2020) | (not checked) | none | stale; a char n-gram generator is ~100 lines; MU ships a TS grammar + 549 k name pack (not Rust) |
| Erosion/terrain | none that fits | | | ours: stream power, Veloren port (GPL-3.0), analytic filter; Bevy terrain crates (`bevy_heightmap` 0.19.0) are loaders, not generators |
| Bevy native | none for worldgen | | | Bevy 0.20 provides ECS, tasks (`AsyncComputeTaskPool`), compute shaders; no PCG |

Reuse verdict (CONTRIBUTING rule 0-2): Bevy has no PCG framework; no maintained crate does ecology scatter, climate or erosion; WFC is covered by `ghx_proc_gen` (MIT/Apache, Bevy 0.20, adopt for tile/kit layouts only, in a Bevy-free core use `ghx_proc_gen` without the `bevy` feature); everything else stays ours or comes through Magna Urbis.

## 4. Industry: canonical technique per stage

| Stage | Canonical technique | Who / source | Our match |
|---|---|---|---|
| Terrain macro | Uplift-driven stream power erosion + thermal; layout by uplift/erodibility fields, finite time (Cordonnier 2016, Braun-Willett 2013, Perron 2009) | papers (V in ledger/spec); Gaea, World Machine, Houdini HeightField Erode | match |
| Terrain detail | Analytic gradient-aligned erosion filter at runtime | Grenier 2024, Tricard 2019 phasor noise, runevision 2025 (V in spec) | match |
| ML terrain | Latent diffusion terrain at 30 m, multi-scale | terrain-diffusion, SIGGRAPH 2026 (V in ledger TD-1..3) | tried, parked; steering loses shape |
| Biomes/climate | Whittaker diagram: temperature (sea-level T minus lapse rate x height, ~6.5 K/km) and precipitation (orographic: windward uplift raises, leeward rain shadow lowers) classify biome; Ghost Recon Wildlands "studied heat, drought, erosion and set 11 ecosystems" (80.lv interview, U); Valheim: noise + distance bands, first match wins (fan sources, U) | secondary sources | **gap** |
| Hydrology | Flow accumulation to rivers, lakes with water budget, carved bed; meanders/braids as separate centreline stage (Paris 2023) | MU `vector_water::condition_and_water` (V) | prototype WR-1 |
| Rocks | Geology-driven: unit hardness picks class, size follows power law, clustered at scarp/talus bases | rockgen spec | classes exist; scatter ignores size/cluster (owner look) |
| Flora scatter | Rule + density map placement on GPU, tile-local, deterministic; 500+ asset types, ~250 placed/frame, ~250 us GPU (Horizon, GDC 2017 slides, V search result, U on detail); Far Cry 5: Houdini rules for biomes, freshwater, cliff rocks, but "by and large the map is handcrafted" (PlayStation blog/Ubisoft, U); UE5.7 PCG production-ready, GPU compute (only some nodes: Copy Points, Static Mesh Spawner, Custom HLSL), Biome Core local + global graphs and runtime GPU generation around camera (Epic docs, V search) | see sources | we have v1 rules; lack ecology |
| Ecosystem sim | Plant competition for light/space with succession and self-thinning; layered (Deussen 1998, Makowski 2019, Hadrich 2019 Synthetic Silviculture; Cordonnier 2017 ecosystems + erosion, U) | papers (secondary) | **gap**: offline bake feasible, rule approximation preferred |
| Settlements/roads | Magna Urbis (owner decision) | | adopted |
| POIs | Constraint placement of authored kits by biome, distance, slope, then reachability (Valheim: distance/biome bands, 10 camp locations >= 1500 m from centre, U fan); Diablo IV: 150+ dungeons mixing handcrafted and procedural from tilesets (Blizzard 2022, U on algorithm); Elden Ring-like: hand-authored sites, no generation (U, memory); Skyrim radiant dungeons: anti-example, same tile kit repeated, no site identity (memory); Dwarf Fortress history-driven sites (memory) | secondary | **gap** |
| Naming | Per-culture Markov/grammar over a name list | MU TS: 51 grammars over 549 k pack (V `SW` capabilities row 26) | **gap** |
| Tooling loop | Tiled builds with masks and seeds (Gaea Tiled Builds, Houdini PDG parallel tiles, U), Horizon: iteration speed, determinism, local stability as goals (slides, V) | | partial |

## 5. Gap list (ranked by what the 10x world needs first)

Bake cost entries are estimates from our ledger numbers (U) unless a ledger line is cited. Runtime ms: none of these add frame cost except scatter density (BW lane budget); frame target 8.3 ms median / 11 ms p99 at High 1440p DLSS Q.

| # | Stage | Gap | Why for 10x | Owner look fixed | Canonical technique | Bake cost | Runtime ms | Spec/slices changed |
|---|---|---|---|---|---|---|---|---|
| 1 | Flora/rock scatter | Replays source placements: no clustering, uniform size, no competition, no moisture/aspect, trees not snug; "sprayed" because density is a per-region constant times a tile-local Bernoulli | 10x land has ~100x the trees to place; v1 looks like noise at that scale | forest, rocks one size, donut | Rule+density-map placement with clump processes (Thomas/Matern), self-thinning by size-radius competition, power-law sizes, slope/aspect/moisture/talus-base terms; density rasters from climate (gap 4) | seconds to a minute (v1: 7.2 M in 6.3 s; competition pass E 2-5x) | 0 added (same instance count); count/size changes BW budget | `worldgen.md` stage 8, `groundcover.md` scatter, `rockgen.md` placement; WG-E2 |
| 2 | Hydrology | MU water chain only a prototype arm; anchors not masked; Red Mountain crater silted; quay kind undiagnosed; rivers not in `worldgen-x10` default chain | Rivers are the 10x land's skeleton; Balmora must straddle Odai; roads/settlements depend on it | Balmora astride river, blobby relief | MU `condition_and_water`; `h' = h + delta*(1 - anchor_w)` then stage-4 stamps; river lines rerouted around footprints | WR-1: seconds at /3 (55 m) (V ledger); /1 minutes (U) | 0 (water bodies exist in `sf_water`) | `worldgen.md` stage 5, adapter; WG-W1 |
| 3 | POIs / wilderness sites | None generated; 631 authored anchors over ~2500 km2 | Gameplay-first priority (owner 2026-11-25): a world that is empty between 31 towns | world still empty | Site table per biome/slope/distance constraints, MU remote + activity sites (caps 11-13, 22, 25), prefab kits, reachability check, optional WFC for footprints | seconds-minutes (E) | 0 (static scene/HLOD like anchors) | new spec `sites.md` (write first), `worldgen.md` stage 9; WG-P0/P1 |
| 4 | Climate / biome fields | No temperature, precipitation, wetness-from-wind; moisture only from flow accumulation | Shared input for scatter, splat, MU forest field (`build_forest_field`), cultures; rain shadow explains Red Mountain Ashlands vs Bitter Coast | forest, textures | Lapse rate + orographic precipitation (upwind ray march), Whittaker-style class raster; region map stays authoritative where authored | < 10 s on 6.4 M samples (E) | 0 | `worldgen.md` stage 6; WG-E1 |
| 5 | Ground materials | Layers learned from source; no rule map from slope/curvature/flow/wetness/rock/biome; macro albedo variation limited | 10x land has far more ground than the source ever textured | textures super repeated | Rule-driven splat from material fields (Wildlands: slope, height, flow, density, deposition, curvature, U); macro-variation noise + hex tiling (X7a done) | seconds | shader only (existing layer map 2.5 ms CPU-side per ledger WG-X3) | adapter WG-N1, `terrain.md` materials; WG-M1 |
| 6 | Tooling loop | Per-stage cache, arms contact sheet, live scatter tune, override masks, seed stability not all present | Each look iteration must stay minutes; owner rates by elimination | all | Stage DAG with content-hash cache (Horizon determinism/local stability; Gaea Tiled Builds) | n/a | 0 | `worldgen.md` WG-1; WG-T1..T3 |
| 7 | Mid detail | WG-3 pipe erosion unbuilt; relief "blobby" mitigated by landforms + runtime filter | Close-up hillsides at 10x | relief smooth/blobby (partly done) | Pipe-model hydraulic + thermal, tiled | E 10+ min if full (open question 1) | 0 | WG-3; run only if landform owner looks still say blobby |
| 8 | Names | None | New sites, rivers, ranges need names for map/quests | none | Per-culture Markov/grammar; port MU grammar+pack (data) | ms | 0 | WG-NM1 |
| 9 | Validation | Walkability exists (`SF/measure.rs`, 46 deg); no reachability for placed sites/flora exclusion, no navmesh-aware POI check | POIs that cannot be reached are waste; navmesh tiles are baked once (CONTRIBUTING 2026-11-26) | n/a | Slope-cost flood from roads/towns; drop or re-seat unreachable sites | seconds | 0 | WG-V1 |
| 10 | Caves/dungeon entrances | `rockgen.md` RG-7 specced, MU Underdark not adopted | Entrances give POIs a reason to exist | n/a | Entrance prefab on cliff band + interior kit (out of scope today) | n/a | n/a | RG-7; later |
| 11 | ML terrain | Parked | none now | | | | | keep TD tools; no slice |

## 6. Magna Urbis: what our vendored copy still needs (data-driven only)

Existing plan: `SW/docs/research/magna-urbis-capabilities-2026-11-24.md` section 4 (C-2..C-14). Additions for the 10x world:

| Need | Via data | Code patch (generic) |
|---|---|---|
| Culture entries for Dunmer styles (Hlaalu, Redoran, Telvanni, Ashlander camps) | `catalogue.json` cultures + presets, `forms/styles.json` kits, `services.json`, `enclosures.json`, `fortifications.json`, `harbours.json`, `activities.json` (C-5) | `MAPPINGS` read from `catalogue.json`; per-culture `densityMax` (clamp 0.5..2.0 at `U/morph.rs:137`) |
| Wilderness sites for gap 3 | `Terrain::place` in `list` mode: kept towns by `position` + generated hamlets, farms, remote places (summit temple, tomb in a gorge, ruins), activity sites (mines, quarries, camps) on `resources/` anchors (C-10, C-11) | expose site archetype weights per biome class (our climate raster as `Input` biome plane for `landuse`) |
| Forest/land cover prior | `terrain::build_forest_field` (canopy, height, needleleaf share, bank band) fed by our climate fields (gap 4) | accept our T/P rasters instead of two scalars (terrain crate takes `[T, P]` only, V) |
| Names | port grammar + pack JSON (51 grammars, 549 k names) into vendored Rust as data (C-14) | Rust loader; TS is the original |
| Rivers vs anchors | none | anchor mask on `delta`, river reroute (our glue, in `SW/crates/water`) |
| River quays, cliff shores | `harbours.json` stepped/beach-landing types | gentle-bank rule by data at `U/macroplan/harbour.rs:234` (C-8) |
| Hand-kept lots | `LayoutConfig.overrides`/`ParcelConfig.overrides` stable ids (C-13) | none |
| Reachability | none | roads crate output reused as the cost network (gap 9) |

Rule check: no Morrowind-only logic; Morrowind enters only as culture data and kits.

## 7. Editor / tooling loop: what we lack

| Need | Have | Lack |
|---|---|---|
| Fast preview | `worldgen-view`, `view world:` game streaming, hillshade PNG, contact sheets, `measure` tables | scatter and site preview as a layer in the viewer; in-game live knob for scatter (like `worldgen.sharp`) |
| Iteration time | macro arms 113 s for 4; water 8.8 s; scatter 6.3 s | per-stage content-hash cache so a scatter retune does not rerun macro (spec WG-1 says so; verify per stage, U) |
| Overrides | anchors (kept human-made); MU lot overrides | a mod-level "authored override" layer: paint masks (forest density, no-scatter, pinned site, forced clearing) read as guide layers; sidecar file, never in the generated product |
| Determinism | tile-local hashed scatter bitwise equal across threads (WG-X5), GPU/CPU twin 1e-4 | seed-stable ids for sites/rivers so a quest reference survives a re-bake (Horizon "local stability") |
| Arms comparison | owner rates by elimination | one command producing N arms with labelled contact sheet + stats (exists ad hoc per arm) |
| Tiles | scatter 256 m tiles, guide tiles, MU at 1024 raster/59.5 m | tiled MU (water at /1 18.3 m full island is memory-heavy, U) |

## 8. Proposed slices (one deliverable + its test each)

Tunables: each new value declared with unit, range, source (`tunables.md`), `Guess` allowed; never bare consts. Agents for code: strong model; research/docs separate. Owner looks go to the owner directly.

| ID | Deliverable | Test | Tunables (unit, range, source) |
|---|---|---|---|
| WG-E1 | `sf_worldgen::climate`: temperature (lapse) + precipitation (orographic, wind dir) + moisture raster from a guide; TOML recipe, regions may override | headless: same hash twice; windward > leeward precip on a ramp fixture; lapse exact on a cone | `climate.lapse_k_per_km` K/km 2-10 (6.5, physics), `climate.wind_dir_deg`, `climate.rain_decay_km` km 5-200 Guess |
| WG-E2 | Scatter v2 in `SF/scatter.rs`: clump process, size power law, competition thinning, climate/aspect terms; species params in recipe TOML | tile-alone == whole-run bitwise; statistics test on a fixture: pair-correlation g(r) > 1 at clump scale, size distribution slope within tolerance; `#[ignore]` measurement prints | `scatter.clump_radius_m`, `scatter.size_exponent`, `scatter.compete_k` (Guess) |
| WG-E3 | Rock placement rules: size by talus/scarp distance, cluster at scarp base | same statistics test | per rock class Guess |
| WG-W1 | Water chain as a `worldgen-x10` stage (anchor-masked `delta`, river reroute), read via `SF/water.rs` | wet/dry audit zero new water; anchor interiors bitwise equal; Red Mountain crater depth within 5 m | mask band m (existing) |
| WG-P0 | Spec `superfargoth/docs/specs/sites.md` (research-first rule 0 checks, site table format) | doc review only | n/a |
| WG-P1 | Site export: MU `Terrain::place` list mode + remote/activity sites to `sites.json` (SW), generic `SF/sites.rs` reader | deterministic; all sites on land, slope <= declared max; reachability stats | per archetype density Guess |
| WG-V1 | Reachability validator: cost flood from towns/roads, report/drop unreachable | fixture with island + bridge | max walk deg = 46 (`MoverParams`) |
| WG-M1 | Material rule map: splat weights from slope/curvature/flow/wetness/climate via adapter table | per-sample weights sum to 1; region fallback | thresholds Guess |
| WG-T1 | Stage-hash cache across `scrib-worldgen run` stages | retune scatter reruns only scatter (test counts stage runs) | n/a |
| WG-T2 | Override layer (paint masks as guide layers) | masked cell never gets scatter/site | n/a |
| WG-NM1 | Names: loader for MU grammar/pack data + Dunmer grammar data | deterministic, no duplicates in 1000 draws | n/a |
| WG-R | Research: ecology-driven placement vs rules (Cordonnier 2017, Makowski 2019 read in full) before E2 final | report | n/a |

Order: WG-E1 and WG-W1 in parallel (separate crates/lanes), then WG-E2/E3, WG-P0 -> P1 -> V1, then M1, T1/T2, NM1. BW lane (`bigworld.md`) consumes E2's counts; keep both under its budgets.

## 9. Owner questions (taste/scope), recommended answers

1. Climate source: generic rule model from height + wind with the existing region map overriding, or paint moisture/temperature by hand? Recommend rule model + region override (generic, no Morrowind names).
2. First wilderness sites exterior-only (ruins, camps, shrines, mines, standing stones; no new interiors), later assigning existing vanilla interior templates? Recommend exterior-only first.
3. Do generated places and landforms get generated names (Dunmer grammar), or stay unnamed until a quest needs one? Recommend generated names, shown on the map only when discovered.
4. May dense forest block sight lines in regions like the Bitter Coast and Ascadian Isles (Valheim/Elden Ring style) with clearings at roads and sites? Recommend yes, with guaranteed clearings.

Not asked (industry-standard decisions): placement technique, bake budgets, formats, cache layout.

## 10. Sources

- UE5.7 release and PCG: https://www.unrealengine.com/news/unreal-engine-5-7-is-now-available ; PCG GPU: https://dev.epicgames.com/documentation/unreal-engine/using-pcg-with-gpu-processing-in-unreal-engine ; Biome Core: https://dev.epicgames.com/documentation/unreal-engine/biome-core-runtime (search snippets only, U).
- Horizon placement: https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf (goals/numbers from search summary, U on detail).
- Far Cry 5: https://gdcvault.com/play/1025557/Procedural-World-Generation-of-Far ; https://blog.playstation.com/2018/03/22/the-procedural-world-generation-of-far-cry-5/ (U).
- Ghost Recon Wildlands: https://gdcvault.com/play/1024029/ ; https://80.lv/articles/procedural-technology-in-ghost-recon-wildlands/ (interview, U).
- Houdini heightfields: https://www.sidefx.com/docs/houdini/heightfields/erosion.html ; Gaea features: https://quadspinner.com/Gaea/Features (Tiled Builds, U on Gaea 2).
- Valheim generation (fan sources, U): https://metabot.gg/en/valheim/map ; Diablo IV dungeons (Blizzard quarterly 2022, U): https://wccftech.com/diablo-iv-150-dungeons-regions-locations-videos/
- Ecosystem papers (secondary, U): https://www.cs.purdue.edu/homes/bbenes/papers/Nocca21ToG.pdf ; https://80.lv/articles/002sgr-generating-procedural-plant-ecosystems/
- Crates: crates.io API pulled 2026-11-26 (versions/dates above, V); `ghx_proc_gen` clone `C:\Users\leogloglo\AppData\Local\Temp\research-1\worldgen\ghx_proc_gen` @ 520c9e8 (MIT OR Apache-2.0, bevy 0.20); `noise-rs` @ 77de338 (2025-02).
- Internal (V): `docs/ledger/worldgen.md`; `superfargoth/docs/specs/{worldgen,landforms,rockgen,groundcover}.md`; `bevyrrowind/docs/specs/worldgen-adapter.md`; `docs/roadmaps/bigworld.md`; `SW/docs/research/magna-urbis-{capabilities,terrain,roads,slopes}-2026-11-24.md`; `SW/vendor/magna-urbis/VENDOR.md` (rev 21e22171f0b1766650b2cfef5d4bdaf574a1d325).
- From memory, unverified: Skyrim radiant dungeon critique, Elden Ring hand-authored sites, Dwarf Fortress history sites, Cordonnier 2017 (SIGGRAPH ecosystems + erosion), Whittaker 6.5 K/km lapse rate (standard atmosphere value).
