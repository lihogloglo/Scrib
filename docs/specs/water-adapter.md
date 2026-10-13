# Water adapter (Morrowind → sf_water)

Status: draft (2026-10-13, m6-spec). W0 census done and W1 `mw_water` skeleton built (2026-10-13, m6-w1):
see "Census results" and "As built (W1)".
Crates: `mw_water` (new, Bevy-free: `WaterSource`, `WaterOpticsSource`, optics/cue records from
`Morrowind.ini [Water]`, water-mesh census, boat rules), app wiring in `apps/bevyrrowind`
(`exterior.rs`: the `SeaPlaneScaffold` goes; interiors gain water); touches `mw_world` (worldspace
water levels, already parsed), `mw_physics` (moored floaters), `mw_assets` (water-mesh material).
Framework spec: `../../../superfargoth/docs/specs/water.md`. Related: `weather-adapter.md` (wind per
weather, rain), `physics-adapter.md` (item mass), `world-adapter.md` ("Worldspaces").

## Goal

Express Morrowind's water in the framework's terms so the original data drives the modern system:

- the sea at Morrowind's single sea level, everywhere in the exterior, to the horizon;
- interior water at each cell's own level (`WHGT`);
- the Bitter Coast swamps reading as swamps (murky, still, foggy) without new data;
- flowing rivers where the original only has sea-level channels (the Odai at Balmora), still canals
  in Vivec;
- authored water meshes (Vivec waterfalls and plaza pools, Mournhold fountains) keep their art and
  gain water shading, and you can wade in them;
- boats that float, items that float or sink;
- the old flat sea scaffold removed.

Not a 1:1 port (rule 6): Morrowind draws one flat animated water plane with an environment
reflection and a fog switch underwater. We keep its *facts* (where water is, at what level, its
underwater colour and visibility, its sounds) and render them with `sf_water`.

## Morrowind water facts

- **One exterior sea level**: every exterior cell's water is at height 0 (engine Y 0 m; OpenMW's
  behaviour). All exterior water, the open sea, the Inner Sea, inland lakes (Amaya, Masobi, Hairan),
  the Odai and the Bitter Coast swamps, is that one plane: inland lakes are terrain dipping below 0.
  Exterior `CELL` records can carry a water height subrecord; it is ignored for exteriors (to confirm
  in the census, M-W0: count exterior cells with a non-zero value).
- **Interiors**: `CELL` flag `HAS_WATER` (0x02) + `WHGT` (float, game units; older plugins: `INTV`
  integer) give the level. Already parsed: `mw_world::CellEnvironment::water_level` (meters) →
  `Worldspace::sea_level` (`mw_world/src/worldspaces.rs`). Some interiors set the flag with the level
  far below any geometry (no visible water): the census lists them and the adapter skips bodies whose
  level is below the cell's lowest geometry bound.
- **No river, lake or depth data** exists anywhere: water extent is "terrain below the level".
- **`Morrowind.ini [Water]`** (read through `mw_install`; OpenMW: `fallback=Water_*`): `UnderwaterColor`
  (012,030,037) and `UnderwaterColorWeight` 0.85; `UnderwaterSunriseFog` 3, `UnderwaterDayFog` 2.5,
  `UnderwaterSunsetFog` 3, `UnderwaterNightFog` 4, `UnderwaterIndoorFog` 3; `SurfaceTexture` `water`
  (32 frames at 12 fps), `RippleTexture`, `RippleLifetime` 3.0, `MaxNumberRipples` 75, `RippleScale`,
  `RippleRotSpeed`, `RippleAlphas`; `NearWaterRadius` 1000, `NearWaterPoints` 8,
  `NearWaterUnderwaterFreq/Volume`, `NearWaterIndoor/OutdoorTolerance`, `NearWaterIndoor/OutdoorID`
  ("Water Layer"); `World Alpha` 0.75, `Map Alpha`.
- **Authored water meshes** (Morrowind.esm/Tribunal/Bloodmoon, model paths): `x\ex_vivec_waterfall_01/03/05`,
  `x\ex_vivec_waterspout_01..05`, `x\ex_vivec_p_water_01` (Vivec plaza pools), `x\ex_waterfall_mist_01`,
  `x\ex_waterfall_mist_s_01`, Tribunal `x\ex_mh_pav_water_spray`, `i\in_om_waterfall`,
  `i\in_om_waterfall_small`, Bloodmoon `i\bm_ic_watertunnel_01`, `f\act_bm_water_parts`,
  `f\act_bm_stone_water_01`. They are UV-scrolled NIF parts (nif-runtime.md material controllers).
- **Boats** (statics): `x\ex_de_rowboat`, `x\ex_longboat`, `x\ex_longboat01`, `x\ex_longboat02`,
  `x\ex_longboatwrk` (wreck), `x\ex_gondola_01` (Vivec), `x\ex_de_ship` (walkable ship),
  `x\ex_de_shipwreck`, plus ship parts (`ex_de_ship_oarleft/right`, `ex_ship_plank`, `ex_ship_stair`,
  ship doors).
- **Swimming**: GMST `fSwimHeightScale` sets the immersion at which an actor swims; swim speeds from
  `fSwimRunBase`/`fSwimWalkBase` and athletics (rules, M7). Water breathing / water walking effects,
  drowning (breath) and slaughterfish behaviour are M7.

## Data model

### Bodies (`WaterSource` for Morrowind)

| Source | Body | Notes |
|---|---|---|
| Exterior worldspace (`sea_level` Some(0.0)) | `Ocean`, `Infinite`, level 0, priority 0, optics by region (below) | the framework's default from `sea_level`; the adapter only adds optics and fetch |
| Interior with `HAS_WATER` and a visible level | `Pool`, `Bounds`, level = `WHGT` in meters, optics `mw_water/interior` | flat, ripples, dynamic ripples; underwater fog from `UnderwaterIndoorFog` |
| Water-mesh census (M-W6) | `Pool`, `Polygon` from the horizontal water submesh's outline at its placement, level = its height, priority 10 | Vivec plaza pools, Mournhold basins: wadeable, swimmable, buoyant; rendering stays the NIF's (below) |

`river_policy()` = **`FlowOnly`**: no generated rivers above sea level. Morrowind's terrain has no river
beds above 0 m; generating rivers would put water where the original world is dry land (owner
decision 3 below). `water.flow` bakes flow on the sea plane's channels.

### Openness and fetch on one sea plane

Because all exterior water is one `Ocean` body, enclosed water must not get open-sea waves:
- `water.shore`'s openness is computed per **connected water component** at level 0 (2 m grid). The
  component touching the worldspace's outer edge is the open sea; its openness falls with geodesic
  distance into bays and channels. Every other component (Lake Amaya, swamp pools, the Odai above its
  mouth if disconnected at rest level) uses its own extent as fetch: small lakes get chop only, swamp
  pools get ripples only.
- Fetch for the open sea (spectrum input): 20 km default, a calibration constant in `mw_water` (the
  Inner Sea is narrow; Vvardenfell is compressed, weather-adapter.md "Scale").

### Flow (sea-level channels)

`water.flow` over the exterior ocean with the drainage of `water.hydro`:
- **The Odai at Balmora** gets its inflow from the West Gash/Balmora highland catchment and flows to its
  outlet toward the coast: flow-advected normals, floating items drift downstream.
- **Vivec canals, harbours, Seyda Neen bay**: little or no inflow → still (wind ripples only).
- **Bitter Coast**: many small inflows into a maze of shallow pools; speeds clamp near zero except in
  the larger channels. Expect slow drifts; tune the discharge scale on the Odai first, then check that
  the swamp stays still (WA-2).

### Optics by region (`WaterOpticsSource`)

The region name of the exterior cell (`CELL` region, the same lookup as `ClimateSource`) picks an optics
record; the bake rasterizes it per cell with a 128 m border blend.

| Region(s) (REGN names, confirm in the census) | Optics record | Look |
|---|---|---|
| no region / open sea, Azura's Coast, Sheogorad | `mw_water/sea` | clear blue-green, visibility ≈ 25 m |
| Ascadian Isles, West Gash coast, Grazelands, Molag Mar | `mw_water/coastal` | greener, visibility ≈ 15 m |
| Bitter Coast | `mw_water/swamp` | murky green-brown (high absorption + scattering), visibility ≈ 3 m, foam off, contact foam off, caustics × 0.2 |
| Ashlands, Red Mountain, Molag Amur coasts | `mw_water/ash` | grey-green, ash film tint during ash storms (cleared by rain) |
| Solstheim regions (Bloodmoon) | `mw_water/north` | cold blue-grey, visibility ≈ 20 m |
| interiors | `mw_water/interior` | dark, visibility from `UnderwaterIndoorFog` |

**Calibration from the ini**: `UnderwaterColor` (012,030,037) × `UnderwaterColorWeight` is the target
colour of the in-scatter at a few metres depth in `mw_water/sea` at noon; the `Underwater*Fog` values set
the visibility ratios between sunrise, day, sunset and night (Morrowind multiplies its fog distance by
them). The colours are calibration targets, not outputs: the optics records are physical (absorption,
scattering per metre), tuned once on WA-1/WA-5 screenshots. A "classic water" mod can patch them.

### Weather coupling (weather-adapter.md)

- `Wind Speed` per weather → `WindState` → spectrum: Clear/Cloudy (0.1-0.2) give a nearly glassy sea,
  Rain/Thunderstorm build swell and whitecaps, Ash/Blight storms chop under the radial storm wind from
  Red Mountain (so swell runs away from the mountain on the north-east coast).
- Rain → ripples and splashes on water (`Rain Ripples`, `Rain Ripple Radius/Scale/Speed` set the
  ripple atlas, weather-adapter.md); Snow → no ripples, flakes vanish on contact.
- Morrowind's own `RippleLifetime`/`MaxNumberRipples`/`RippleScale` are the calibration for the
  dynamic ripple sim's injection size and damping around wading actors.

### Cues and sounds

| Morrowind | Cue |
|---|---|
| `NearWaterOutdoorID` / `NearWaterIndoorID` ("Water Layer") within `NearWaterRadius` (1000 units ≈ 14 m) of water, tolerances 1024/512 | `water_ambient` loop, volume by distance to the shore (query + shore distance) |
| `NearWaterUnderwaterFreq` 0.3 / `NearWaterUnderwaterVolume` 0.9 | `underwater` cue parameters (low-pass, volume) |
| footsteps in shallow water | `FootWaterLeft`/`FootWaterRight` SOUN (ids to confirm) below 0.3 m depth |
| swimming strokes, water landing | `Swim Left`/`Swim Right`, `DefaultLandWater` (ids to confirm) |

The audio system (later) plays them; the adapter only maps ids.

### Authored water meshes

- **Rendering**: the NIFs keep drawing (their UV scroll is the art). `mw_assets` gives their water
  parts the framework's water mesh material (Fresnel, environment/SSPR reflection, refraction of the
  snapshot, flow normals aligned with the part's UV scroll direction, the NIF texture as the albedo
  and foam mask) instead of a plain alpha blend. Which parts: those whose texture is one of the
  census's water textures (M-W6 lists them; none are guessed here).
- **Bodies**: horizontal water submeshes of the plaza-pool models become `Pool` bodies (above), so the
  query knows them. Vertical waterfalls stay meshes only (no body): their foot gets contact foam and a
  `waterfall` cue at the base.
- Mist meshes (`ex_waterfall_mist_*`) stay as they are (particles/alpha cards).

### Boats and floating things

- **Items** (M4 dynamic items): float or sink by `body.cook` density (mass from the item weight rules in
  physics-adapter.md / volume). Calibrate so that wood, cloth, food and most ingredients float, metal
  weapons and armor sink. `water.pontoons` gives 1-2 pontoons per item.
- **Moored boats** (recommended default, owner decision 2): placed `ex_de_rowboat`, `ex_longboat*` and
  `ex_gondola_01` references whose hull intersects the water at rest become **floaters on a soft mooring**:
  a dynamic body with 4-8 pontoons and a spring-damper joint to its authored pose (stiff horizontally,
  free vertically within ±0.5 m and ±10° roll/pitch). They bob and rock with the waves and return to
  their authored spot; actors standing on them ride the platform (the mover's moving-ground support).
  Beached or wrecked boats (hull above water, `ex_longboatwrk`, `ex_de_shipwreck`) stay static.
- **Ships** (`ex_de_ship` and its parts) stay static: they are walkable multi-part sets with doors and
  NPCs; a moving deck would need every part rigidly joined. Later, if wanted.
- **Demo boat**: a dynamic, unmoored rowboat spawned by the M6 demo scene at the Seyda Neen docks
  (`--spawn-floater x\ex_de_rowboat.nif`), hull clipping on (water.md "Buoyancy").

### Interiors

- Interiors with water get a `Pool` body and the water pass in their worldspace layer. Today the
  interior path renders no water at all (only the exterior draws the scaffold), so flooded interiors
  look dry: this is new behaviour, checked on WA-6.
- Underwater in interiors uses `mw_water/interior` and the indoor fog value; Morrowind's interior
  `AMBI` fog (weather-adapter.md "Interior fog") stays the above-water fog.

## What replaces `SeaPlaneScaffold`

Today (`apps/bevyrrowind/src/exterior.rs`): `SeaPlaneScaffold`, a 64 km `Plane3d` with an alpha-blended
`StandardMaterial` (base 0.05/0.16/0.2, alpha 0.86, roughness 0.08), spawned in the exterior's render
layer and moved by `sync_origin` under the camera (snapped to 256 m) at `sea_level = −origin_cell.y ×
256 m`. Interiors have no water.

Replacement (task W2 in ROADMAP "M6"):
1. The app adds `sf_water_bevy::WaterPlugin` with `mw_water`'s `WaterSource`/`WaterOpticsSource`; the
   exterior's `Ocean` body comes from `Worldspace::sea_level` (already `Some(0.0)` from `mw_world`).
2. The scaffold stays behind `--water scaffold` until WA-1 passes (A/B), then the component, its spawn
   and the sea branch of `sync_origin` are deleted. `SkyRenderOptions::sea_level` (the atmosphere's
   planet placement) is a different concern and stays.
3. The water's floating-origin phase comes from the same origin cell `sync_origin` already reads (as
   the clouds' `CloudsOrigin`), set in the same window before the water's drive systems.

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

## Verification

- `cargo test -p mw_water`: optics by region with border blend; interior bodies from `WHGT` (and
  `INTV`); skipped invisible interiors; moored-boat classification on fixtures; ini mapping with missing
  keys falling back to vanilla values; cue id mapping.
- Visual checks (added to `docs/visual-checks.md` by the task that builds them, "awaiting owner"):
  - **WA-1 Open sea, Seyda Neen**: from the lighthouse/docks, 14:00, `--weather clear` then
    `--weather thunder`: calm sea, then swell, whitecaps and shore foam; dock and lighthouse reflected
    (SSPR), sand visible through shallow water, colour deepening offshore. A/B `--water scaffold`.
  - **WA-2 Bitter Coast swamp**: between Seyda Neen and Hla Oad, 07:00, `--weather foggy`: still,
    murky water, no waves, roots meeting the water without foam, fog banks on the water; walk in:
    ripples around the legs, wet legs after leaving.
  - **WA-3 Odai at Balmora**: noon, from a bridge: visible downstream flow; a thrown crate drifts
    downstream; the bridge reflected.
  - **WA-4 Vivec canals at sunset**: still water reflecting the cantons; a gondola bobbing on its mooring.
  - **WA-5 Through the surface**: Seyda Neen pier, camera descending: meniscus at the lens, Snell's
    window, caustics on the sand, light shafts (High), teal visibility near the ini colour.
  - **WA-6 Interior water**: the census's pick: the pool at its `WHGT` level, dark water, swim under.
  - **WA-7 Floating**: the demo rowboat and dropped items at the Seyda Neen docks: wood floats, iron
    sinks, the boat rocks in the storm and drifts with the wind/flow.
  - **WA-8 Coast flight**: 60 m/s along the west coast: no wave jumps at origin shifts, no clipmap pops,
    sea under land not drawn (GPU counter).

## Industry comparison

The rendering and simulation techniques are compared in superfargoth `water.md`. Data semantics:
Bethesda's engines (Morrowind's single water plane plus per-interior levels; Oblivion/Skyrim add
per-cell water heights, water types with fog/colour records and placed water planes) and OpenMW
(reimplements the single plane, interior levels and the `[Water]` fallback keys; its shader water adds
reflections and refraction of the same plane) keep water as a level per cell with a type for its look.
Ours: the same levels mapped to bodies, the look as physical optics per region (**match** on data,
**differ** on rendering by design); rivers derived as flow on existing water instead of new data
(**differ**, by design: no authored rivers exist). No gap at the adapter level.

## Owner decisions (recommendations; also in ROADMAP "M6")

1. **Water approach**: own `sf_water`, porting bevy-aqua's wave/foam/optics code (MIT/Apache), not
   bevy-aqua as a dependency or fork. Recommend: yes.
2. **Moored boats**: placed rowboats, longboats and gondolas in water bob on a soft mooring; ships stay
   static. Recommend: yes.
3. **No generated rivers above sea level** for Bevyrrowind (`FlowOnly`): flow on existing sea-level
   channels only. Recommend: yes (a "rivers of Vvardenfell" mod could switch to `Generate { carve }`).
4. **Swamp look**: murky optics with ~3 m visibility, no waves, no foam in the Bitter Coast. Recommend:
   yes, tune on WA-2.

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
