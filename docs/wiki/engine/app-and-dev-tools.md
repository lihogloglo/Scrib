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
- **Dev tools** (`mw_devtools_bevy`, `lib.rs:1`): `live_ab` (A/B keys and one-line overlay, `view`/`scene` only; F1
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
  `pacing.awk`. Repo `tools/`: `doccheck.sh`, `decisions-index.mjs`.

## Bevy features used
`bevy_remote` (BRP, via `bevy_brp_mcp`/`bevy_brp_extras`), `bevy_diagnostic`, gizmos, `PipelineCache::waiting_pipelines`
(`pipeline_watch.rs`).

## Switches and counters
`--off <parts>`, `--bench`, `--resolution`, `--hitch-ms`/`BEVYRROWIND_HITCH_MS`, `BEVYRROWIND_BRP_PORT`,
`BEVYRROWIND_BAKE_SKIP_NAV`. Live A/B and console variables as above.

## Known limits
Live A/B, console and BRP exist only in interactive `view`/`scene` runs, not screenshot or bench runs. Agents judge no
visuals; `--screenshot` is for the owner. Re-run `bevyrrowind bake` after animation slot changes.

## Specs
`bevyrrowind/docs/specs/app-split.md`, `bevyrrowind/docs/perf.md`, `superfargoth/docs/specs/diag.md`, `docs/SYSTEMS.md`.
