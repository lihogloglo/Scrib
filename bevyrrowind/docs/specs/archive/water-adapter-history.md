# Water adapter history

Dated history blocks moved verbatim from [water-adapter.md](../water-adapter.md) (doc cleanup S10b, 2026-10-26).

## Census (task M-W0, real install, counts only)

As built: `cargo run --release -p mw_world --example water_census` (prints counts only, 1.1 s; an
example rather than an ignored `mw_water` test because it needs `mw_world`'s placements, `mw_nif`
bounds and `LAND`, which `mw_water` should not depend on). Planned list:
- exterior cells with a water height subrecord, and any non-zero value;
- interiors with `HAS_WATER`: count, levels, how many below all their geometry (skipped), the 10 largest
  visible water areas (WA-6 picks one);
- placements of each boat model, how many intersect the water at rest (moored), beached;
- placements of the authored water meshes and their textures (feeds the water-mesh material list);
- connected sea-level water components and their areas (open sea, lakes, swamp pools);
- region names present on cells with water (confirms the optics table).

### Census results (W0, 2026-10-13, m6-w1; Morrowind + Tribunal + Bloodmoon)

- **Exterior**: 1,559 cells, all with `HAS_WATER`, **none** with a water height subrecord: one sea level (0)
  confirmed, nothing to ignore.
- **Interiors**: 1,328; 237 with `HAS_WATER`: **216 visible** (level above the lowest placed geometry), **21
  below all geometry** (skipped). Visible levels −42.9 to +217.9 m, median 0, 82 exactly at 0. Largest wet
  extents (XZ box of geometry reaching below the level, a coarse proxy that overstates open floors):
  Mournhold Temple Courtyard 3.4 ha, Falasmaryon Sewers 2.0, Old Mournhold: Bazaar Sewers 2.0 (−10 m),
  Norenen-dur 2.0 (−3.1 m), Old Mournhold: Temple Sewers 1.8 (−5.7 m), Hlormaren Sewers 1.7 (−2.8 m). The
  Mournhold "interiors" are city cells behaving as exteriors with a level at 0 that may sit under the floor:
  W3 checks the top entries by eye and picks WA-6 from the sewers/ruins.
- **Boats** (placements; moored = hull AABB crosses the level, above, sunk): `ex_de_rowboat` 34 (17 / 16 / 1),
  `ex_longboat` 6 (6 / 0 / 0), `ex_longboat01` 3 (3), `ex_longboat02` 8 (8), `ex_gondola_01` 5 (5), `ex_de_ship` 9
  (9, static by decision), `ex_longboatwrk` 6 (3 / 0 / 3), `ex_de_shipwreck` 21 (16 / 1 / 4). Moored-floater
  candidates (rowboats, longboats, gondolas in water): **39**. The 16 rowboats "above" are beached or on decks.
- **Authored water meshes**: 35 placed models name water in their path or base texture; 9 water textures:
  `tx_waterfall_01/02` (`.tga` Vivec, `.dds` Mournhold), `tx_v_water_01` (Vivec plaza pool), `tx_water_enviro00`
  (cave pools: `furn_moldcave_pool00` 93 placements, `pycave` 13, `lavacave` 3, `mudcave` 3),
  `tx_sewer_waterwell00` (sewer corridors and canals, 13 `in_sewer_*` models, 293 placements), `tx_eggmine1_water00`
  (`in_dwrv_obsrv10`, `ex_dwrv_cosmo00`), `tx_alpha_waterspout00`. Vivec: `ex_vivec_waterfall_01` 305 placements,
  `_03` 17, `_05` 62, waterspouts `_01/_02/_03/_05` 70/446/18/8 (no water texture: alpha cards), plaza pool 1.
  False positive: `in_cave_plant00` (2,519, a plant whose texture name contains "water"): W9 filters by
  texture list, not names. Bloodmoon `f/act_bm_*water*` have no water texture (their parts use other textures).
- **Sea-level water** (LAND vertex grid, 1.83 m, 4-connected, cells without LAND are sea): 150 components.
  Open sea (touching the outer ring) **26.0 km²**; 149 enclosed, 0.12 km² in total: 47 under 100 m², 83 up to
  0.1 ha, 18 up to 1 ha, one of 5.7 ha (Ascadian Isles, cell 1,−7). The inland lakes (Amaya, Masobi, Hairan) and
  the Odai are **connected** to the open sea at this resolution.
- **Bitter Coast**: 156 LAND cells (2.1 km²), 78.6 % below sea level: 1.63 km² connected to the open sea, only
  0.054 km² in 87 enclosed pools. **Consequence for W3**: the swamp's calm cannot come from "its own component"
  fetch (spec "Openness and fetch"); it must come from openness (geodesic distance through narrow water to the
  open region) plus the swamp optics/wave terms by region. Keep openness's geodesic falloff steep enough that
  1-2 m channels count as enclosed.
- **Regions with water** (open-sea / enclosed km², wet share of LAND): Azura's Coast 3.78 / 0.003 (73 %),
  Sheogorad 1.67 (72 %), Bitter Coast 1.63 / 0.054 (79 %), West Gash 1.16 (45 %), Ascadian Isles 0.73 / 0.061
  (50 %), Hirstaang Forest 0.43 (61 %), Felsaad Coast 0.20 (53 %), Moesring Mountains 0.17, Isinfier Plains 0.14,
  Grazelands 0.04, Molag Mar 0.04, Ashlands 0.003, no region (4 cells) 0.055; Red Mountain, Brodir Grove, Thirsk
  dry. The optics table's region names are confirmed (no "Molag Amur" region exists: its coast is Molag Mar and
  Ashlands).

## As built (W1, 2026-10-13, m6-w1)

`crates/mw_water` (Bevy-free; deps `sf_water`, `sf_world`, `sf_coords`, `encoding_rs`):
- `MwWaterSource: WaterSource`: the framework's body from `Worldspace::sea_level` (exterior `Ocean`,
  interiors `Pool`/`Bounds` at `WHGT`) with optics `mw_water/sea` / `mw_water/interior`; `river_policy` =
  `FlowOnly`; `ocean_spectrum` fetch `SEA_FETCH_M` = 20 km.
- `optics_for_region(name)` (table above, names as the census found them) and `MwWaterOptics { region_at }`:
  `WaterOpticsSource` over a region lookup the app passes in (so the crate needs no cell grid).
- `optics_records(&WaterIni)`: six `WaterOptics` built from a target visibility (sea 25 m, coastal 15, swamp 3,
  ash 12 with a film tint, north 20, interior from `UnderwaterIndoorFog` vs `UnderwaterDayFog`), swamp without
  foam and caustics × 0.2. Placeholder physics until W3 tunes them on WA-1/WA-2/WA-5.
- `WaterIni::parse(bytes)`: the `[Water]` keys listed in "Morrowind water facts", vanilla fallbacks per key. The
  app reads `Morrowind.ini` bytes and passes them (`mw_install` keeps parsing only what decides content loading).
- Not yet: hidden-interior skipping (needs placement bounds: the census logic moves into the world adapter at
  W3), moored boats (W6), water-mesh bodies and material (W9), cue ids. 3 tests.

## As built (W2, 2026-10-13, m6-w2)

App `src/water.rs`: `--water on|scaffold|off` (default on), `--water-wind <m/s>` (the `WindInput` scaffold, default
7 m/s toward the north-east, until WD1). The exterior ocean body comes from `MwWaterSource` (a `Worldspace` with
`sea_level` 0); `Water { WaterWorld, time }` steps in `FixedUpdate` before `PhysicsSystems::Step` with a 64² twin of
cascade 0 (same seed `0x5EA00001` and cascades as the GPU); bed = the resident `TerrainHeightsMap` (re-cloned at most
once a second while it changes); `WaterFrame` from `TerrainOrigin` after `TerrainSystems::Update` (so after
`sync_origin`, in the frame of the origin shift); look = `mw_water/sea` optics; `WaterCamera` on the exterior
camera; froxel volume wired from `sf_atmos_bevy::FroxelVolume`. `SeaPlaneScaffold` spawns only with
`--water scaffold`; the scaffold, its spawn and the sea branch of `sync_origin` go once WA-1 passes. Scenes
`wa-seyda-noon`, `wa-vivec-waterfront`, `wa-bittercoast`, `wa-sunset-sea`, `wa-night-moons` (shots in `shots/w2/`).

## As built (W3, 2026-10-18, m6-w3 + w3fin)

- `mw_water`: `wet_mask(bounds, lands)` (every 2nd `LAND` vertex, 3.66 m texels, wet where the height is below 0, no
  `LAND` = water), `cell_bounds` (+1 cell of sea), `optics_map(bounds, region_of)` (one texel per exterior cell, index
  in `optics_index` order = `optics_records` order: sea, coastal, swamp, ash, north, interior), `shore_settings()`
  (framework defaults: the WA-2 calibration point).
- **Optics retuned** (owner 6a): `scatter_share` is now the water's diffuse reflectance, sized like real backscatter
  (≈ 0.33 b_b/a, 2-5 %; W2's 7-25 % read milky): sea 25 m, blue clearest, 0.025; coastal 14 m, green clearest
  (turquoise lagoons), 0.04; swamp 3 m, tea-olive (blue absorbed), 0.035, no whitecaps or shore band; ash 10 m
  grey-brown, 0.05; north 18 m steel blue, 0.02; interior from `UnderwaterIndoorFog`, 0.03. Shore band per region
  (sea 0.45, coastal 0.3, north 0.6, ash 0.3) and contact foam ≤ 0.7.
- App `water.rs`: `WaterInputs::build(db)` in `exterior_options` (wet mask from every `LAND`, region map from the
  exterior cells' `region`); the `ShoreField` bakes on a worker thread at start (**0.49-0.63 s** in release for
  1729 × 1601 texels, 147 components, 1 open; gate ≤ 10 s) and goes to `WaterShoreInput` and the CPU query
  (`WorldspaceWater::set_shore`); `WaterRegionsInput` with the six looks; interiors with `HAS_WATER` get a
  `WaterSurface::pool` at their level (render space follows the floating origin) in their render layer while active;
  `fog_input` fills the analytic height fog (`WaterFogInput.height`, in-scatter = albedo × (sun / 4π + camera ambient ×
  fog ambient)) in the exterior, used when the froxels are off; `--quality` picks the `WaterQuality` tier.
- Scenes: `wa-interior-pool` (Hlormaren, Sewers, −2.77 m) added; shots `shots/w3/`; WA-2 and WA-6 awaiting owner.
- Open: the interior pool draws opaque near-black (no valid refraction snapshot in interiors; WA-6); in-app GPU
  counters at 1440p, the WA-8 flight and the DLSS smear check (no quiet machine on 2026-10-18); "sea under land not
  drawn" counter (fragments are discarded via the shore field, no GPU counter); wet band (SU1); body overlap report;
  hidden-interior skipping (the 21 below-geometry interiors still get a pool, invisible under their floors).
