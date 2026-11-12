# Frame and views (exterior) and how the perf suite measures it

## What it does

Describes which render views one exterior frame has and what the perf suite records about them. The exterior has **one**
`Camera3d` (the player or fly camera) plus a UI `Camera2d`; everything else is a shadow-pass view or a compute pass inside the
main view.

## How it works

**Cameras.** The exterior camera is spawned in `mw_exterior_bevy/src/exterior.rs:788` (`Camera3d`, `Hdr`, `FloatingOrigin`,
`StreamingAnchor::visual()`, `SkyCamera`, optional `TerrainCamera`, SSAO/prepass components, `exterior.rs:795-843`). The UI is a
`Camera2d` at order 1 (`exterior.rs:869`). No other `Camera3d` exists in the exterior: water, sky, atmosphere, GI and fog extract
the main camera and add passes in its `Core3d` schedule (`sf_water_bevy/src/render.rs`, `sf_atmos_bevy/src/render.rs:1-3`).

**Views per frame** (what `sf_diag::views::ViewKind` counts: `Main`, `Cascade`, `LampFace`, `Other`, `sf_diag/src/views.rs:5-17`):
- `main`: the camera.
- `cascade<i>`: sun cascades, 3 on Low/Medium/High (max distance 160/190/205 m), 4 on Ultra (280 m)
  (`exterior.rs:1575-1580`; built by `sf_sky_bevy::CascadeSplits`). The moon is a second directional light (`MoonLight`)
  that spawns with shadows off and has them switched per frame by `apply_lights` (`sf_sky_bevy/src/lib.rs:470-476, 682`), so a night frame can have moon cascades.
- `lamp.face<f>` / `lamp.spot`: point-light cube faces, six per shadowed lamp. `PointShadowBudget` (`sf_sky_bevy/src/shadow_budget.rs:51`)
  caps shadowed lamps (default 8 at 512 px, fade 35-45 m, `mw_environment_bevy/src/render.rs:28-33`; per preset
  `quality.point_shadows()`, `exterior.rs:344`). Face caching is **vendored `bevy_pbr`'s** `skip_unchanged_shadow_views`
  (`vendor/bevy_pbr/src/lib.rs:410`), not `sf_shadow_cache_bevy`: the game compiles that plugin out with
  `POINT_SHADOW_CACHE = false` (`render.rs:37, 185`), so its capture/restore views and `ShadowCacheSystems` do not run in the game.
- `other` (extra depth views drawn by Bevy's shadow pass through `sf_sky_bevy::sun_view`, `sun_view.rs:1-30`, only on redraw
  frames): rain occlusion map (`sf_precip_bevy/src/occlusion.rs`) and the surface shelter and canopy maps
  (`sf_surface_bevy/src/lib.rs:212-213`).
- Not views: probe GI traces rays in compute through Solari's scene plugin (`sf_gi_bevy`), and only in interiors, High/Ultra
  (`exterior.rs:346-353`); the sky environment map is Bevy's `AtmosphereEnvironmentMapLight` (time-sliced by
  `sf_sky_bevy::EnvMapRefresh`, `--off envslice`); the far shadow is a map read by materials (`sf_lod_bevy` `FarShadowInput`).

**Plugins that drive the render** (`exterior.rs`): `DefaultPlugins` (`:287`), `WorldStreamingPlugin` (`:295`), `ScenePlugin`
(`:299`), `SkyPlugin` (`:360`), `sf_atmos_bevy` clouds and fog (`:441, :478`), `FarFieldPlugin` (`:483`), `TerrainPlugin`
(`:499`), `sf_water_bevy`, `sf_wind_bevy` (`:429`), `sf_shadow_cache_bevy` (not added, see above), `sf_gi_bevy` (interiors).

**The perf suite.** `bevyrrowind perf suite <tag> [scenarios] [--repeat N] [--resolution WxH] [--off a,b] [--arg=--scene]
[--env K=V] [--floor]` (`bevyrrowind/apps/bevyrrowind/src/perf/cli.rs:137`). Scenarios are `SCENARIOS` (`cli.rs:25`): balmora,
seyda_neen, red_mountain, run, balmora_walk (+ `_night`), balmora_night, flight, census (interior), light_lab; each launches the
game with `--bench <json> --bench-out --resolution`. Before each run it waits for a quiet machine (no busy process, GPU and CPU
idle) and retries busy runs (`cli.rs:437, 501`); results go to `target/perf/<tag>/summary.md` (`cli.rs:371-402`).
In the game, `--bench` adds (`mw_devtools_bevy/src/perf.rs:164-180`) `SfDiagPlugin` (frame times, Bevy diagnostics, scenario
camera path, JSON/CSV report), `RenderDiagnosticsPlugin` (GPU pass timings), `ChurnPlugin`, `FramePhasePlugin`
(`main/busy/ms`, `render/thread/ms`, `render/acquire/ms`, `render/graph/ms`, `render/present/ms`, `sf_diag_bevy/src/phases.rs:1-18`),
`RenderViewsPlugin` (per-view counts) and asset churn plugins.

**Report sections** (`sf_diag/src/markdown.rs`): Gates, GPU passes (GPU ms, p95, CPU ms), Per-frame costs (ms counters: mean, p95,
max), Gauges (counts, sizes), Churn, Hitches (frames over `max(2.5 x median, 25 ms)`, `report.rs:224`, with the counters that
changed). The suite summary adds columns for entities, meshes, point lights, CPU %, `views/all/{count,items,batch_sets,work_items}`,
`preprocess/bind_groups` and the `scene/*` gauges (`cli.rs:372-387`). Per-view counters are `views/<kind>/{count,items,bins,
batch_sets,work_items,pipelines,instance_*}` and `views/all/pipelines_distinct` (`sf_diag/src/views.rs:11-18`), one frame behind.

## Bevy features used

`Camera3d`, cascaded and point shadow views, Bevy `RenderDiagnosticsPlugin` GPU timers, pipelined rendering (the render thread
timers above), vendored `bevy_pbr` shadow-view skipping. See `bevy/` pages when they exist.

## Switches and counters

`--off shadows|lights|shadowcache|movershadows|lampfilter|softshadows|contactshadows|ssao|bloom|gi|envslice|clouds|fog|froxels|
sky|terrain|particles|actors|walkers|physics|precipocc|surface` (`mw_game_core/src/perf.rs`). `shadowcache` is a no-op for the
raster path while `POINT_SHADOW_CACHE` is false. Env `BEVYRROWIND_WINDOW`, `BEVYRROWIND_PRESENT`, `BEVYRROWIND_DLSS`.

## Known limits

- Render-thread cost grows with views times statics: [ledger/statics.md](../../ledger/statics.md), [ledger/perf.md](../../ledger/perf.md).
- Streaming spikes show as hitches: [ledger/streaming.md](../../ledger/streaming.md).

## Specs

`superfargoth/docs/specs/diag.md`, `shadows.md`, `sky-lighting.md`, `gi.md`.
