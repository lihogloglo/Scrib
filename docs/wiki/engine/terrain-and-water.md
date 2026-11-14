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
`mw_bake::world` (`MwTerrainSource`, `MwLayers`). Layer maps (T5): `MwLayers::with_sources` gives each `LTEX` its pack
normal + ARM companions (`world/layer_maps.rs`, the cook's lookups and per-source profiles), converted to canonical in
`LayerSource::decode` (normal +Y up; ARM R = AO, G = roughness); `normal_to_array_frame` (`bake.rs`) stores them in
the arrays' +V frame that `surface.wesl` `sample_projection` reads (x along +U = world +X, y along +V = world +Z).
Bake line `terrain layers: N, n with a normal map, a with ARM`. Arrays (T6, `encode_layer`): albedo + height BC7 sRGB
(ISPC Fast), normal X/Y BC5, AO + roughness BC5, full chains, `LayerArrays` FORMAT 3 (`product.rs`); edge = the host's
max (`mw_bake` `LAYER_ARRAY_SIZE` 2048) capped at the largest source (`LayerInfo::source_edge`); `terrain.layers` v4.
Load: `BakedTerrain::layers_capped` copies only the levels `TerrainSettings::layer_cap` keeps (preset cap + 1 GiB
budget, `DEFAULT_LAYER_BUDGET`); log `terrain layers: N x S², X MiB (L levels)`; no BC on the device = error + flat layers.

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
and depth snapshot of the opaque scene, draws in Bevy's transmissive pass (alpha 1, depth-write, no fog of its own; the
sky, aerial perspective, clouds and fog run after it by its depth, W-ORDER), then redraws motion vectors for DLSS/TAA
(`render.rs`, `lib.rs` `WaterCamera`). Local reflections: SSPR of the snapshot (`water_sspr.wesl`, `sspr_pass` in
`PassStage::Reflection` between snapshot and water; plane = nearest shown body below the camera, `sf_water/src/sspr.rs`,
keyed by source height above the plane); the water draw swaps the environment-map specular for it where it hits
(`water_material.wesl` `sspr_sample`, `environment_specular`). `WaterReflections` (on, `wave_distortion` 0.6);
`WaterQuality::sspr_scale` Low off, Medium ½, High/Ultra full. Reflections are unfogged (fog is applied over the water after). `WaterQuery::sample`
(`query.rs:294`). Shore: `ShoreField::bake` (`shore.rs`) runs on a thread at startup, not in the bake store
(`mw_environment_bevy/src/water.rs:172-182`), then attenuates swell and foam. At the shore the displacement settles but the
shading normal keeps the chop's wind wavelets at `ShoreSettings::normal_floor` (0.5; `shore_normal_scales`, W-SHORE). Where the
FFT swell fades, `sf_water::ShoreWaves` (`shore_waves.rs`; GPU `water_common.wesl` `shore_waves`, in the draw, the velocity redraw
and the CPU query) adds three Gerstner trains rolling in along the baked shoreward direction plus a beach swash, scaled by the
sea state (W-GERSTNER; the shore field is `Rgba16Float`: distance, openness, direction), shoaling over the local bed slope β = depth / shore distance baked from the adapter's depths (W-GERSTNER-DEPTH, `ShoreField::bed_slope`, `shore.rs`: a second `R16Float` texture, clamped 0.02-0.3, the 1:20 `beach_slope` where unknown). The sea follows the weather's 10 m mean
wind (W-WIND, `sf_water/src/sea_wind.rs`): low-passed (30 s) and blended between two `h0` sets of one seed (6 s transitions, new
`h0` on a worker thread), the CPU twin and shore waves blended alike; the exterior copies `WindField` into `WaterWind`. Morrowind wiring:
`mw_environment_bevy/src/water.rs:141` (`add`; fixed-tick `step`, `bed`, `pools`, `sync`); `mw_water` gives bodies (sea at 0,
interiors at `WHGT`), optics by region (`optics::*`), `SEA_FETCH_M` 20 km, shore settings.

## Bevy features used

`MaterialPlugin` for both materials, batched indirect draws and per-view culling for patches (see
[Render](../bevy/render-and-visibility.md)); terrain is drawn in the depth prepass and shadow cascades.

## Switches and counters

`--water on|scaffold|off` (`mw_environment_bevy/src/water.rs:46`; scaffold is a flat plane, `exterior.rs:881`), `--water-wind`;
`--off waterssr` (environment map only), `--off shorenormals` (flat normal at the shore, the old look), `--off shorewaves` (no Gerstner shore waves or swash), `--off seawind` (the fixed 7 m/s sea breeze; `--water-wind` pins any speed), `--off shoredepth` (the shore waves over the uniform 1:20 slope proxy), `--off terrain` and `--off far` perf parts (`mw_game_core/src/perf.rs:127`). `TerrainSettings` `upload_budget`,
`max_loads`, `splat_range`, `raise_m`.

## Known limits

Terrain layer arrays are RGBA8 at 256² (`mw_bake/src/world.rs` `LAYER_ARRAY_SIZE`): BC7/BC5 and size per preset are T6;
layer height for the height blend comes from the albedo's alpha only (a pack's `_nh` height isn't used). Far shadow map upload cost was a walk stutter source (see [perf ledger](../../ledger/perf.md); water history in the
[water ledger](../../ledger/water.md)). Rivers, underwater and
wading are specced (`water.md`); check the code before assuming they are wired. SSPR only mirrors what is on screen
and opaque (no translucents, nothing off screen: the environment map fills in; the dynamic probe is specced, not built).

## Specs

`superfargoth/docs/specs/terrain.md` (Data model, Interfaces, Industry comparison), `water.md` (Ocean surface, Shoreline
and depth colour, Water pass, Rivers, Crate split), `bevyrrowind/docs/specs/water-adapter.md`, `world-adapter.md`.
