# App and dev tools

## What it does
One binary, `bevyrrowind`, over the user's Morrowind install. No subcommand launches the game at the new-game start
under the main menu; subcommands are inspection, bake, viewer, scene and benchmark tools. Dev builds add live A/B keys,
a console and BRP for agents.

## How it works
- **Entry**: `bevyrrowind/apps/bevyrrowind/src/main.rs:1` (clap `Command` at `:52`). `play` (`:907`) is the no-subcommand
  path (`ExteriorOptions::start_in_main_menu`, see ui-and-menus). Inspection: `detect`, `info`, `record`, `find`, `ls`,
  `uses`, `extract`, `which`, `nif`, `anim`, `nif-sweep`, `weather`, `furniture-census`, `anim-sources`. Build and
  content: `bake` (`mw_boot_bevy/src/bake.rs`: collision, terrain, nav, far field, textures), `mods`
  (profiles, load order). Viewers: `view "<cell>"`, `character`, `scene <name>`. `clip-preview` only with feature `dev`.
- **Scenes**: `scenes.ron` (`bevyrrowind/scenes.ron`; read from `--file`, cwd, next to the exe, then built in;
  `apps/bevyrrowind/src/scenes.rs:1`). A scene is a saved `view` command line plus what to look for; `scene --list`
  lists them. Visual-check entries live in `bevyrrowind/docs/visual-checks.md` and `visual-checks/<system>.md`.
- **`--off` parts**: comma list or env `BEVYRROWIND_OFF`; enum `Part` in `mw_game_core/src/perf.rs:11` (terrain, near,
  far, sky, envmap, shadows, lights, particles, nif, farfield, ssao, bloom, gi, shadowcache, clouds, actors, walkers,
  physics, fog, froxels, ...). Decided at startup; `apply_off` (`mw_devtools_bevy/src/perf.rs:259`) drops spawners.
  Cheap ones also flip live (below). Presets: Low to Ultra.
- **Perf suite** (`apps/bevyrrowind/src/perf/cli.rs`): `perf list | args | sweep | suite | trace | spikes | nsys`.
  `SCENARIOS` (`:25`: `balmora`, `balmora_walk`, `balmora_night`, ...) run with `--repeat N`; results under
  `<out>/<tag>/`; `--bench` runs take `--bench-out`, `--bench-baseline`, `--bench-seconds`, `--resolution WxH`. Run the profiling build
  from the repo root. Frame sections: see frame-and-views. Counters come from `sf_diag_bevy`.
- **Dev tools** (`mw_devtools_bevy`, `lib.rs:1`): `live_ab` (A/B keys and one-line overlay, `view`/`scene` only; F1 help, F3 perf panel below;
  help), `console` (backquote; game commands, other lines run as MWScript), `remote` (BRP, feature `remote` inside
  `dev`, never release), `pipeline_watch` (logs pipelines still compiling; a draw with a pending pipeline is skipped),
  `ready`, `perf`.
- **BRP**: loopback, port `BEVYRROWIND_BRP_PORT` (default `15712`, `remote.rs:40`; shown in the window title). Agent tools
  `bevyrrowind/status` (scene, hour, switches, camera, settled flag) and `bevyrrowind/set`, plus `bevy_brp_extras`
  (keys, mouse, frame diagnostics). Sub-agents never launch the game (CONTRIBUTING).
- **Scripts** (`bevyrrowind/tools/`): `cargo_turn.sh` runs cargo through a first-come queue (`.sessions/build-queue/`),
  BelowNormal, waiting while any rustc or other cargo runs (`bash tools/cargo_turn.sh build -p bevyrrowind --features
  dev`; `--queue` shows it). `owner_launch.sh scene|view ...` waits until the machine is free (no build, no game
  window, owner not in a fullscreen app or call), copies the dev build to `%TEMP%\bw-owner-*`, starts it fullscreen
  and muted (adds the `view`/`scene` flag `--mute`: master bus silent that run, saved volumes untouched; the script's
  `--sound` opts out, owner 2026-11-12); `OWNER_LAUNCH_NO_WAIT=1` checks once. Others: `run_dev.ps1`, `perf_nsys.ps1` (Nsight to `perf nsys`), `ab_shots.ps1`,
  `pacing.awk`. Repo `tools/`: `doccheck.sh`, `decisions-index.mjs`, `bevy_variants.sh` (how many Bevy builds a set of
  cargo selections needs, from `cargo tree`; CONTRIBUTING "Building").

## Texture viewer (`tex`, Unreal's `vis`)
- **Console** (dev runs, backquote): `tex list` (name, size, format, description); `tex show <name> [r|g|b|a|rgb]
  [min max]` draws it in the bottom-right corner (45% of the window height): one channel as grey or `rgb`, values
  remapped from `[min, max]` (default `0 1`; `max < min` inverts) so the grey level is the remapped value; nearest
  texels; NaN/inf magenta. A unique part of a name is enough (`tex show trail g -0.3 0`). `tex hide`. Alias `vis`.
- **Registered**: `surface.shelter`, `surface.canopy` (depth maps, 1024² Depth32Float; try `r 0 1` / `r 1 0`),
  `surface.cells` (128² RGBA8: R wet, G puddle, B snow, A ash), `surface.exposure` (current pack, 1024² RGBA16F: R/G
  solid/leaf caster height m, B lag, A exposure), `surface.trail` (current ring, 1024² RGBA16F: R depression m <= 0,
  G lip m), `precip.occlusion` (512² Depth32Float).
- **Registering a target** (framework, any system): `sf_diag_bevy` feature `texview`, `DebugTextures`
  (`sf_diag_bevy/src/texview.rs:96`). An `Image` asset: `DebugTexture::image(name, description, handle, &image)`
  (`:58`; e.g. `sf_precip_bevy/src/occlusion.rs:228`, `sf_surface_bevy/src/exposure.rs:206`). A render-world
  texture: `app.register_debug_texture(DebugTexture::render(name, description, size, format, |render_world| view))`
  (`:70`, `:148`; e.g. `exposure.rs:183`, `trail.rs:250`); the closure runs only while that name is shown.
- **Viewer**: `sf_texview_bevy` (`lib.rs:49`, added by `mw_devtools_bevy/src/console.rs:91`). On a change
  (`apply_view`, `lib.rs:324`) it makes an `Rgba8Unorm` target `Image` (source size, long side <= 1024) in an
  `ImageNode`; the render world resolves the source (`render.rs:142`) and one fullscreen pass `texview.wesl`
  (`textureLoad`: float, depth and uint/sint variants, no copy) runs in the root `RenderGraph` after `camera_driver`
  (`render.rs:137`); the UI shows it one frame later. Hidden: no target, node or pass. Not viewable: depth+stencil
  formats, multisampled, array layers/mips past the view's first. Bevy's own `RenderDebugOverlayPlugin`
  (bevy_dev_tools) only shows the camera's prepass buffers full-screen.

## Perf panel (F3, OpenMW's F3 / UE `stat unit`)
`mw_devtools_bevy/src/perf_panel.rs` (pure half `perf_panel_model.rs`), in every interactive run incl. release (added
by `live_ab::add`). F3 cycles off / compact / full (top right); Shift+F3 is now terrain far shadows; console/BRP
`flip perfpanel`. Every number is a mean over `diag.panel.window_s` (1 s), redrawn every `diag.panel.refresh_s` (0.25 s).
- **frame** ms/fps, p99, max: `Time<Real>` delta. **main / render**: `main/busy/ms`, `render/busy/ms` (`sf_diag_bevy`
  `FramePhasePlugin`, render one frame behind). **GPU**: sum of the top-level `render/<pass>/elapsed_gpu` (Bevy
  `RenderDiagnosticsPlugin`; partial, untimed passes missing); "n/a" unless `diag.panel.gpu_timers` (env
  `BEVYRROWIND_PANEL_GPU_TIMERS=1`, read at startup: it adds timestamp queries + a readback every frame) or the GI's timer.
- **bound by**: the busiest of GPU/render/main if it is at least `diag.panel.bound_ratio` (0.85) of the frame, else
  "none" (vsync, a cap, untimed GPU).
- Full only: top 3 `stage/main/*`, top `diag.panel.top_passes` GPU passes, `views/*` (camera, cascades, lamp faces,
  `lamp_face/culled`, items, batch sets; `RenderViewsPlugin`), point+spot lights total/shadowed (queried at refresh),
  `encode/*` (encoders, command buffers, submits + ms, poll; CW-5b), entities, the graphics preset (`sf.graphics_preset`;
  `custom (high; shadows custom, post low)`: the usual group level and the groups that differ), every tunable set above the
  preset with its layer. Streaming: n/a (no live gauge).
- Cost: the gauge plugins are gated by `sf_diag_bevy::set_gauges_live` (off while hidden or with `diag.panel.gauges`
  off) and `DiagCounters` leaves the main world, so counter-gated reporters stop too. Not measured.
- Reuse check (2026-11-16): Bevy 0.20 `bevy_dev_tools` `FpsOverlayPlugin` (frame only) and `DiagnosticsOverlay`
  (reads `DiagnosticsStore` only, needs `bevy_picking`, behind Bevy's `dev` feature: not in release, and our gauges live
  in `sf_diag` counters); `iyes_perf_ui` 0.5 (MIT/Apache) is on Bevy 0.16. So a plain `bevy_ui` text node. Industry:
  OpenMW F3 (frame/update/render/GPU bars), UE `stat unit` (Frame/Game/Draw/GPU, bound = the one near Frame), `stat gpu`.

## Bevy features used
`bevy_remote` (BRP, via `bevy_brp_mcp`/`bevy_brp_extras`), `bevy_diagnostic`, gizmos, `PipelineCache::waiting_pipelines`
(`pipeline_watch.rs`).

## Switches and counters
`--off <parts>`, `--bench`, `--resolution`, `--hitch-ms`/`BEVYRROWIND_HITCH_MS`, `BEVYRROWIND_BRP_PORT`,
`BEVYRROWIND_BAKE_SKIP_NAV`. Live A/B and console variables as above.

**Tunables** (spec `superfargoth/docs/specs/tunables.md`; index `docs/tunables-index.md`, regenerate with
`bevyrrowind tunables --md`, `--json` for tools): `view`/`scene` (so benches) take `--set name=value` (repeatable) and
`--tunables <file.toml>` at the command-line layer (`PerfFlags`, `mw_devtools_bevy/src/perf/tunables.rs`), applied in
`Plugin::finish` once every plugin registered; a refused name or value logs and exits 2 before the first frame.
Interactive runs (no `--bench`/`--screenshot`, `main.rs` `is_interactive`) also read `<user data>/tunables.toml`
(logged); benches only the files they are passed. A def's `env_alias` (deprecated env switch) sets the same layer at
registration. Console: `vars` (value, unit, kind/range, layer), `help <name>` (source, lifetime, env alias, layers),
`tunables diff`. Perf: `--set` in `perf suite` and `--arm`, `perf sweep --var set:<name>`; every run report gets a
"Tunables" section (report meta `tunables`). A crate's `TUNABLES` list goes in `perf/tunables.rs` `CATALOG` (a run
warns about a declared tunable missing from it).

**Graphics presets, live** (tunables T5b; `mw_exterior_bevy/src/quality_presets.rs`, `sf_tunables/src/presets.rs`):
levels `extremely_low`, `low`, `medium`, `high`, `ultra` of `sf.graphics_preset`, which sets the groups `sg.view_distance`,
`sg.shadows`, `sg.lighting`, `sg.post`, `sg.textures`, `sg.effects`, `sg.water`, `sg.sky` (values: the builtin mod's
`bevyrrowind/crates/mw_mods/data/records/graphics_presets.ron`, table in `docs/tunables-index.md`). Start level
`--quality <level>` (`extremely-low` too; `BEVYRROWIND_QUALITY`). Switch while running: **Shift+F4** cycles the overall
level (overlay note says what changed); console `preset` (list), `preset ultra`, `preset shadows low`, or
`set sf.graphics_preset ultra` / `set sg.shadows low`; BRP `sf.tunables/set`; `--set sf.graphics_preset=ultra` at start.
A level writes the `Preset` layer, so console/`--set`/`--off` values stay above it and the driver reads `custom`; `reset
<name>` goes back. Not live: `texture.max_size_px`, `far.handoff_scale`, `render.gi.probes` (`Restart`: stored, the
note says "applies on restart"). F3 shows the level.

## Known limits
Live A/B, console and BRP exist only in interactive `view`/`scene` runs, not screenshot or bench runs. Agents judge no
visuals; `--screenshot` is for the owner. Re-run `bevyrrowind bake` after animation slot changes.

## Specs
`bevyrrowind/docs/specs/app-split.md`, `bevyrrowind/docs/perf.md`, `superfargoth/docs/specs/diag.md`, `docs/SYSTEMS.md`.
