# Terrain and water

## What it does

Terrain: baked canonical heightfield tiles, streamed into GPU clipmaps and drawn as CDLOD patches (one shared grid mesh,
batched indirect draws), plus a far shadow map and CPU height queries. Water: a GPU FFT ocean with a CPU twin
(`WaterQuery`) shared by rendering, buoyancy, swimming and sound, plus lakes and interior pools at their own level.
Crates: `sf_terrain`, `sf_terrain_bevy`, `sf_water`, `sf_water_bevy`; Morrowind facts in `mw_water`.

## How it works

**Terrain bake** (`sf_terrain/src/bake.rs:46-51`): `terrain.tile` (level 0: `upsample` Catmull-Rom of the source,
`constrain` under footprints), `terrain.pyramid` (level k from four k-1 tiles), `terrain.macro` (257² far albedo),
`terrain.layer_table`/`terrain.layers` (splat textures as arrays), `terrain.rt_proxy`. Tiles are 513² samples,
`TILE_QUADS` 512 (`tile.rs:10`), level k at `0.5*2^k` m. Adapters implement `TerrainSource` (`source.rs`); Morrowind:
`mw_bake::world` (`MwTerrainSource`, `MwLayers`).

**Terrain runtime** (`sf_terrain_bevy/src/lib.rs`): `TerrainPlugin` `:230`; `TerrainWorld` (`:150`) streams tiles from the
store on the async pool around the `TerrainCamera` (`:146`), feeds `TerrainHeightsMap` (CPU queries, `sf_terrain/src/query.rs`).
Clipmaps: 10 levels of 512² `R32Float` heights, 3 splat levels, 10 macro levels, toroidal strip updates under
`TerrainSettings::upload_budget` (`:73`; fill in `sf_terrain/src/clipmap.rs:155,194,259`). `cdlod::select` (`cdlod.rs:144`)
picks patches per frame; the vertex shader morphs and displaces. Patch transforms are relative to the floating origin
(`TerrainOrigin`). `TerrainMaterial`: `material.rs:20`. Height edits: `edits.rs`.

**Far shadow map** (`far_map.rs`): `far_shadow_map` (`sf_terrain/src/shadow.rs`, march 40 steps, reach 6000 m) over
`FAR_MAP_SIZE` 384 texels, computed on the async pool and uploaded as a texture; far proxies and impostors read it (one
fetch), the terrain marches the clipmap itself. `update_far_map` runs before `update_terrain` (`lib.rs:256-258`).

**Water**: `WaterPlugin` (`sf_water_bevy/src/lib.rs:280`). The host supplies `OceanSpectrum`, `WaterFrame`, `WaterLook`,
`WaterShoreInput`, `WaterRegionsInput`, `WaterQuality` and marks `WaterCamera`, `OceanSurface`, `WaterSurface`. GPU FFT
cascades share `h0` with the CPU twin (`sf_water/src/twin.rs`, `spectrum.rs`): waves are a pure function of seed, inputs,
time and absolute XZ. Surface is a camera-centred clipmap with a horizon skirt (`mesh.rs`); the water pass takes a colour
and depth snapshot after sky, clouds and fog, then redraws motion vectors for DLSS/TAA (`render.rs`). `WaterQuery::sample`
(`query.rs:294`). Shore: `ShoreField::bake` (`shore.rs`) runs on a thread at startup, not in the bake store
(`mw_environment_bevy/src/water.rs:178-183`), then attenuates swell and foam. Morrowind wiring:
`mw_environment_bevy/src/water.rs:141` (`add`; fixed-tick `step`, `bed`, `pools`, `sync`); `mw_water` gives bodies (sea at 0,
interiors at `WHGT`), optics by region (`optics::*`), `SEA_FETCH_M` 20 km, shore settings.

## Bevy features used

`MaterialPlugin` for both materials, batched indirect draws and per-view culling for patches (see
[Render](../bevy/render-and-visibility.md)); terrain is drawn in the depth prepass and shadow cascades.

## Switches and counters

`--water on|scaffold|off` (`mw_environment_bevy/src/water.rs:46`; scaffold is a flat plane, `exterior.rs:881`), `--water-wind`;
`--off terrain` and `--off far` perf parts (`mw_game_core/src/perf.rs:127`). `TerrainSettings` `upload_budget`,
`max_loads`, `splat_range`, `raise_m`.

## Known limits

Far shadow map upload cost was a walk stutter source (see [perf ledger](../../ledger/perf.md)). Rivers, underwater and
wading are specced (`water.md`); check the code before assuming they are wired.

## Specs

`superfargoth/docs/specs/terrain.md` (Data model, Interfaces, Industry comparison), `water.md` (Ocean surface, Shoreline
and depth colour, Water pass, Rivers, Crate split), `bevyrrowind/docs/specs/water-adapter.md`, `world-adapter.md`.
