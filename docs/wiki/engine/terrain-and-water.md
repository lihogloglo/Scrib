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
picks patches per frame; the vertex shader morphs (from `terrain.cdlod.morph_start` 0.85 of each range, live; floor 0.78 = Strugar's crack-free bound on level ground, `CdlodSettings::crack_free_morph_start`) and displaces. Shading (WG-LOD): normals, curvature and the cliff rock weight read the finest height level resident around the camera, never the mesh LOD (`common.wesl` `shade_levels`: Chebyshev cover 234 texels per level, cross-fade over the outer `terrain.shade.blend_texels` 48, Losasso & Hoppe 2004), so a LOD switch changes no shading; `set terrain.shade.decoupled off` = the old coupling. Patch transforms are relative to the floating origin
(`TerrainOrigin`). `TerrainMaterial`: `material.rs:20`. Height edits: `edits.rs`.

**Tile providers** (2026-11-19): `TerrainWorld { tiles: Arc<dyn TerrainTiles> }` (`sf_terrain/src/provider.rs`); the bake
store (`BakedTerrain`) is one provider (CPU fill, unchanged), `sf_worldgen::tiles::GuideTiles` another (generated worlds,
`view world:<guide dir>`): it hands the runtime a WGSL height function + data (`GpuField`), so clipmap strips fill on the
GPU (`sf_terrain_bevy/src/gpu_fill.rs`, `sf_worldgen/src/tiles.wgsl`), CDLOD bounds come from `block_bounds` without
evaluating heights, and a CPU twin (2 threads, `terrain.twin.*`) feeds `TerrainHeightsMap` near the camera. `--off
gputerrain` = CPU tiles for every level. A world folder may carry `filter.toml` (`tiles::world_filter`, only the
`steep_fade*` values, LF-2b): the gully amplitude fades out on base slopes 40-50 deg (no dents in rock faces); without
the file it is off, so `x10s` fills bit for bit as before. A guide folder may also carry `water.json` (lakes + rivers from scrib-worldgen `worldgen-water`, Magna Urbis water chain; reader `sf_worldgen::water`), spawned by `mw_environment_bevy::worldgen_water` as `WaterMeshSurface`s (switch `water.guide.bodies`) and, since FL-5a, added to the exterior's CPU `WaterQuery` by `guide_bodies` (`worldgen_water.rs:304`, called at `water.rs:265`): a lake is a `Polygon` body at its level (priority 20), a river `Ribbon` bodies of at most `water.query.ribbon_chunk` (64) points sharing their seam point (priority 30, ocean 0), with the mesh bake's speeds, so a sample in a river returns its sloped level and its flow velocity (844 ns per river sample on x10s-b2w, 2296 ns unchunked). River joins (FL-4): `worldgen_water::resolved` (`worldgen_water.rs:307`) runs `sf_water::joins::resolve` (`joins.rs:195`) once at start, and the query bodies and the mesh bake share its splines: a river end in a lake, at the sea or on the river it joins (snapped within `water.join.snap_max_m`) gets `river_ness` 1 to 0 over `water.join.mix_widths` widths of its run inside the receiver's water, and its point speeds scale with it. `river_mesh` writes the value per vertex (`body_mesh.rs:468`), the game puts it on the ribbon as `ATTRIBUTE_WATER_RIVERNESS` (`sf_water_bevy/src/mesh.rs:29`, `worldgen_water.rs:494`; shader def `WATER_RIVERNESS`, `material.rs:232`), and the water draw discards the ribbon where `1 - water.join.fade * (1 - river_ness)` is at or under Bevy's interleaved gradient noise (`water_material.wesl:254-262`; a new pattern each frame only under `TEMPORAL_JITTER`), so the receiver's own mesh under it shows: the draw is opaque and writes depth, nothing blends by alpha. `water.join.fade` (live, default 1; 0 = the ribbon ends hard) is the A/B arm of scene `river-join-ab`. In a lake the fade follows the course, not the record: where a river ends at the lake's edge and another starts within its half width of that point (a `lake_link`), the fade-out runs on along the second, and the fade-in toward the outlet starts on the piece before the river that leaves the lake; between the two zones a piece in a lake has `river_ness` 0 and draws nothing (22 of the 23 river mouths into a lake of x10hs-b2w are carried on this way: `worldgen-water` writes a `lake_link` through its basin lakes too, as Magna Urbis does through its own; the last one fades fully on its own run). Also `roads.json` (`worldgen-roads`; reader + draped ribbon chunks `sf_worldgen::roads`), spawned by `mw_exterior_bevy::worldgen_roads` with a road LTEX texture, plus any road network a loaded mod carries under `worldgen/roads/*.json` (a town's streets, `worldgen-town --streets`) (`--off roads`, `worldgen.road_depth_bias`, `worldgen.road_opacity`). The flora pass (`mw_worldgen::flora`, scatter v2 since WG-E2i.3: the source's flora base records are the models of `bevyrrowind/worldgen/scatter_species.toml`'s rows, or a guide folder's own copy; canopy intensity comes from the file's `[classes]` stems/ha per biome class, `sf_worldgen::scatter::CanopyTargets`, its `m` and `g` from the bake's class census and calibration, `sf_worldgen::scatter_calib`, written to `<guide>/scatter/calibration.toml`: since WG-E2o.f on a hash-ordered sample of each class's tiles, no rescale, then joint checks at every class's kept `g`; since WG-E2o.h that sample is a pilot of at least `scatter.calib_tiles_min` tiles, each converged class's sample then sized from its ratio-estimator CV to `calib_z` SE within `calib_tol`, at most `calib_tiles_max`) reads the guide's `climate/` product (`flora::product_fields`; without it the pass logs the `scrib-worldgen climate` command to run and places nothing) and `water.json`, and keeps out of the road ribbons plus `clearing.road_m` (6 m) and out of the footprints of judged sites (`sites.valid.json`, else `sites.json`) plus their `clearing_m` (`flora::guide_clearings`, `sf_worldgen::sites::clearings`, WG-V1b), besides the stamps and the guide towns, as forced clearings of the stand mosaic. Rocks are learned but not placed until the rock layer's adapter (WG-E3f). At load the pass reads the guide's `scatter/` product (the kept set before the budget, WG-E2j) when its inputs hash (`flora::product_inputs`: guide, fields, learned rows, prior, towns, clearings, `flora::PRODUCT_FILES`) matches; otherwise it runs the passes and writes it (`flora::scatter_cached`). The budget and pose stay at load, and the summary line says `product read` / `written` / `rebuilt`.

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
(`query.rs:294`). **Mesh bodies** (`WaterMeshSurface { mesh, chop, look }`, `WATER_MESH` shader def): a host-supplied mesh (lake polygon at its level, sloped river ribbon; builders `sf_water::body_mesh`, spade CDT) drawn with the same material, flat (chop ripples only), Aabb-culled, counted by the visibility gate and SSPR plane (box mid height); `ATTRIBUTE_WATER_FLOW` carried, not yet shaded; no motion-vector redraw, underwater medium or shore foam for them yet. Shore: `ShoreField::bake` (`shore.rs`) runs on a thread at startup, not in the bake store
(`mw_environment_bevy/src/water.rs:172-182`), then attenuates swell and foam. At the shore the displacement settles but the
shading normal keeps the chop's wind wavelets at `ShoreSettings::normal_floor` (0.5; `shore_normal_scales`, W-SHORE). Where the
FFT swell fades, `sf_water::ShoreWaves` (`shore_waves.rs`; GPU `water_common.wesl` `shore_waves`, in the draw, the velocity redraw
and the CPU query) adds three Gerstner trains rolling in along the baked shoreward direction plus a beach swash, scaled by the
sea state (W-GERSTNER; the shore field is `Rgba16Float`: distance, openness, direction), shoaling over the local bed slope β = depth / shore distance baked from the adapter's depths (W-GERSTNER-DEPTH, `ShoreField::bed_slope`, `shore.rs`: a second `R16Float` texture, clamped 0.02-0.3, the 1:20 `beach_slope` where unknown).  The shore foam band follows the local breaking energy (`ShoreWave::breaking`, W-FOAM-GATE), so calm and enclosed water has none. The sea follows the weather's 10 m mean
wind (W-WIND, `sf_water/src/sea_wind.rs`): low-passed (30 s) and blended between two `h0` sets of one seed (6 s transitions, new
`h0` on a worker thread), the CPU twin and shore waves blended alike; the exterior copies `WindField` into `WaterWind`. Morrowind wiring:
`mw_environment_bevy/src/water.rs:141` (`add`; fixed-tick `step`, `bed`, `pools`, `sync`); `mw_water` gives bodies (sea at 0,
interiors at `WHGT`), optics by region (`optics::*`), `SEA_FETCH_M` 20 km, shore settings.

**Swimming (FL-5c, 2026-11-26):** `mw_environment_bevy/src/swim.rs`, `FixedUpdate` in `ControllerSystems::Drive`
after the water's `step`: every awake `Mover` not in `Fly` (NPCs, creatures; system `swim`) and every
`sf_action_bevy::ActionBody` not in `Fly` (the player; system `swim_bodies`, before `ActionSystems::Move`, logs
`swim: body ... starts / stops swimming`) asks `WaterQuery::immersion` for its capsule in the active
worldspace, the pure rule `sf_water::swim::medium` decides (`swim.enter_fraction`, `swim.exit_fraction` hysteresis), and the
system writes `MoverMode::Swim { surface }` or back to `Ground`, and `Locomotion::current` (the sample's horizontal
velocity x `swim.current.gain`, x `swim.current.wade_gain` x submerged share when wading, capped at
`swim.current.max_mps`; zero when dry). The mover adds the current to its displacement before the sweep, so banks stop
the drift and the intent stays relative to the water. One rule for every body in the query: sea, pools, guide lakes
and rivers. Morrowind's `fSwimHeightScale` (0.9) sets `swim.enter_fraction` at the `Mod` layer at start (`water.rs`
`WaterInputs::build`); a swimmer floats at 0.75 of its height (`MoverParams::swim_depth`), above the 0.6 exit. `Fly`
movers (pure-swimmer creature scaffold, noclip) are skipped. Not built: dive, breath, strokes, the graph's `swimming`
parameter (ROADMAP 3.6). Guide rivers are wade-deep: their surface lies ~0.6 m above uncarved 18 m ground (no bed
yet), so swimming happens in the sea, lakes and pools.

## Bevy features used

`MaterialPlugin` for both materials, batched indirect draws and per-view culling for patches (see
[Render](../bevy/render-and-visibility.md)); terrain is drawn in the depth prepass and shadow cascades.

## Switches and counters

`--water on|scaffold|off` (`mw_environment_bevy/src/water.rs:46`; scaffold is a flat plane, `exterior.rs:881`), `--water-wind`;
`--off waterssr` (environment map only), `--off shorenormals` (flat normal at the shore, the old look), `--off shorewaves` (no Gerstner shore waves or swash), `--off seawind` (the fixed 7 m/s sea breeze; `--water-wind` pins any speed), `--off shoredepth` (the shore waves over the uniform 1:20 slope proxy), `--off terrain` and `--off far` perf parts (`mw_game_core/src/perf.rs:127`). `TerrainSettings` `upload_budget`, `max_loads`, `splat_range`, `raise_m`, `hex_tiling`.
- **Anti-tiling (WG-X7a, Mikkelsen 2022):** `surface.wesl` `hex_grid`/`hex_tap` breaks periodic layer tiling with a 1024 m-periodic simplex hex grid (floating-origin invariant via `sf_terrain::bake::quantize_tiling`), rotated derivatives (`textureSampleGrad`), inverse-rotated tangent normals `R(-theta)`, and contrast-pruned taps (`w > 0.002`) shared by Pass-1 `layer_height` and Pass-2 `sample_projection`. Live tunables: `terrain.hex_tiling` (default on), `terrain.hex_tiling.rot_strength` (1.0), `terrain.hex_tiling.contrast` (7.0), `terrain.hex_tiling.scale` (1.0), written to `P_HEX` (`params.rs:25`).
- **Slope-driven cliff rock & crevices (WG-X7b):** `surface.wesl` `sample_layer_triplanar`, `cliff_strata_mod`, `terrain_curvature` height-blend a regionally tinted triplanar rock layer onto steep faces (near and far macro, with subtle horizontal strata banding) and darken concave crevices / brighten convex cliff lips (curvature at the fixed `terrain.cliff.curvature_scale_m` 2 m from four filtered normal taps, `surface.wesl` `curvature_fixed`, on the shading levels; `cliff_w` from the shading normal). Near splat/rock also multiplies `layer_edge_fade` (outer 24 texels of the coarsest layer clipmap), and `shade_levels`/`macro_albedo` cross-fade into the next coarser level near window borders so clipmap boundaries stay seamless. Live tunables: `terrain.cliff.enabled` (default on), `terrain.cliff.slope_start_deg` (28), `terrain.cliff.slope_full_deg` (45), `terrain.cliff.crevice_strength` (0.45), written to `P_CLIFF` (`params.rs:27`); host supplies `CliffShadingSettings::rock_layers` preference list (fallback: first `triplanar` layer).

**Selection gate (CW-2, on by default (owner 2026-11-17)):** `TerrainSettings::select_gate` skips CDLOD selection
and the patch diff (`sf_terrain_bevy/src/lib.rs:975`) while the camera stays within `terrain.select.move_m` (0.5 m, 3D,
`lib.rs:121`) of the last selection and its key (tile slots, edits, settings, debug, origin) is unchanged; distance only, so
turning never re-runs it. Crack-free because the shader's morph zones move the quantum closer (`params.rs:67`, test
`stale_selection_stays_crack_free_with_the_morph_slack`). On with `set select.gate on` (live; alias `BEVYRROWIND_SELECT_GATE=1`), `--off selectgate`
forces it off (`exterior.rs:284`).

`water.quality` (`low|medium|high|ultra`, T5a): the FFT tier, live (`water::bind_quality`; `WaterQuality` re-sizes the wave
images, `sf_water_bevy/src/lib.rs:529`); the quality level writes it at `Preset`.

**Water visibility gate (CW-10a, opt-in, built 2026-11-17, not measured):** without it the FFT, mips, snapshot, SSPR and
velocity run in every exterior frame (the clipmap is one `NoFrustumCulling` mesh to the 32 km skirt). `set water.gate.visible on`
(live both ways; `water.gate.margin_m` 16, `water.gate.idle_fft_s` 0.125; `sf_water_bevy/src/gate.rs`, bound in
`mw_environment_bevy/src/water.rs` `bind_gate`). Frustum term (CPU): per `WaterCamera`, boxes where water can be drawn (the shore
field's wet leaves, `sf_water::WetTiles`, plus the open sea around the field and the shown bodies, grown by the margin) culled by
the frustum planes and a view-pyramid SAT; none left: that camera's snapshot, SSPR, velocity are not queued and the FFT runs every
`idle_fft_s` with all cascades (foam stays current), re-entry runs every cascade and copies `prev_displacement` after the FFT (no
stale motion). Occlusion term (GPU, same frame, no readback): the boxes are drawn depth-only after the opaque pass in an occlusion
query (`water_gate.wesl`, `render.rs` `gate_probe`), resolved into the indirect arguments of the snapshot, SSPR and velocity
(zero when no sample passed). The camera inside a box (shore, swimming) skips the probe. Counters: `sf_water_bevy::gate_stats()`
(frustum closes, probes), GPU timer `water_gate`. On an island the frustum term rarely closes looking level (the sea is in every
direction): the occlusion term is the one that saves in streets and valleys; the FFT (~0.16 ms) is only frustum-gated.

## Known limits

Terrain layer arrays are RGBA8 at 256² (`mw_bake/src/world.rs` `LAYER_ARRAY_SIZE`): BC7/BC5 and size per preset are T6;
layer height for the height blend comes from the albedo's alpha only (a pack's `_nh` height isn't used). Far shadow map upload cost was a walk stutter source (see [perf ledger](../../ledger/perf.md); water history in the
[water ledger](../../ledger/water.md)). Underwater: U1 camera medium built (`sf_water::medium`, `WaterMedium` per `WaterCamera`, cues; interior pools are not in the CPU query, so interiors read Above); the post, waterline and caustics are U3-U5. Rivers, the rest of underwater and
wading are specced (`water.md`); check the code before assuming they are wired. SSPR only mirrors what is on screen
and opaque (no translucents, nothing off screen: the environment map fills in; the dynamic probe is specced, not built).

## Specs

`superfargoth/docs/specs/terrain.md` (Data model, Interfaces, Industry comparison), `water.md` (Ocean surface, Shoreline
and depth colour, Water pass, Rivers, Crate split), `bevyrrowind/docs/specs/water-adapter.md`, `world-adapter.md`.
