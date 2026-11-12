# Bevyrrowind

Morrowind content pack and game binary on the superfargoth framework (`../superfargoth`, same repo and Cargo
workspace). **All working rules are in `../CONTRIBUTING.md`** (the one source of truth: reading order, building,
parallel sessions, tests, docs). This file only adds game-specific reference.

## Commands

Compiling commands go through the build queue: `bash tools/cargo_turn.sh <cargo args>` from here,
`bash bevyrrowind/tools/cargo_turn.sh ...` from the root (`../CONTRIBUTING.md` "Building").

```sh
cargo dev view "Seyda Neen, Census and Excise Office"   # Bevy dynamically linked (its own Bevy build)
cargo run --release -p bevyrrowind -- info              # load all content, report
cargo run --release -p bevyrrowind -- bake              # re-bake after animation slot/source changes
cargo dev scene --list                                  # test scenes (scenes.ron); `scene <name> [view flags]` opens one, F1 shows the A/B keys (docs/visual-checks.md)
cargo dev view level:ledges [--camera beam]             # synthetic greybox test level (levels/*.ron, docs/specs/test-levels.md); scenes `level-*`
cargo run -p bevyrrowind --features dev,solari,dlss -- view "<cell>" --rt [--dlss quality]   # ray traced + DLSS (SDK in ../sdk)
```

Game runs are for the owner's checks: sub-agents never launch the game, and the session talking to the owner
uses `tools/owner_launch.sh` (`../CONTRIBUTING.md` "Tests and visual checks"). Game crates may take GPL
crates; Delirium (`--features delirium`) is GPL-3.0-only and off by default (app `Cargo.toml`).

## Live inspection (BRP)

Dev builds (`remote` feature, part of `dev`) serve the Bevy Remote Protocol
on 127.0.0.1 in interactive `view`/`scene` runs, for the `bevy_brp` MCP
server (`bevy_brp_mcp`, registered for this workspace; owner 2026-10-18).
Port `BEVYRROWIND_BRP_PORT`, default 15712, shown in the
window title. Code: `bevyrrowind/crates/mw_devtools_bevy/src/remote.rs`.

- **Windows:** interactive runs open borderless fullscreen on the primary monitor (owner 2026-10-30). Agent runs pass
  `--resolution 1280x720` (a centred window) so they don't cover the owner's screen; owner check scripts pass nothing.
- **Attach only.** Build (`cargo build -p bevyrrowind --features dev`); the owner
  starts it with `powershell -File tools/run_dev.ps1 view "<cell>" > target/run_dev.pid`
  (or `scene <name>`; it prints the pid; from bash redirect to a file, since a
  `$(...)` pipe stays open while the game runs), not `cargo dev`: a `cargo run` stays alive while
  the game is open and stalls every other session's build check. Then
  talk to it. `brp_launch`/`brp_shutdown` are denied:
  launching builds behind our back, and the owner may be looking at the
  window. Don't call `brp_extras/shutdown` through `brp_execute` either;
  close only processes you started.
- **Game tools first:** `brp_list_agent_tools` lists `bevyrrowind/status`
  (scene, hour, A/B switches, camera, `ready`) and `bevyrrowind/set`
  (switches, hour, clock, clouds, camera); call them with `brp_execute`.
  Poll `ready` before `brp_extras_screenshot`. Other A/B keys (F4 DLSS, B
  bodies, ...) via `brp_extras_send_keys`. `world_query` and friends only
  see reflected, registered types (Bevy's, and few of ours yet).
- **Not a replacement** for headless tests, benchmarks or the owner's sign-off: screenshots never judge
  the look (`../CONTRIBUTING.md` "Tests and visual checks"). Never benchmark with BRP on (`--bench` runs
  don't start it).
- Need a new live control? Add it to `bevyrrowind/set` or a new
  `bevyrrowind/*` agent tool in `remote.rs` (one place, typed schema),
  not another env-var hook.
