# Performance measurement

Measure, don't guess. Framework side (reports, hitch table, counters, suite helpers):
`../superfargoth/docs/specs/diag.md`. Adapted from the Ovid project's proven workflow, without Python.

## Build

Measure a release-optimized build, never `cargo dev` (dynamic linking, dev profile):

```sh
cargo build --profile profiling -p bevyrrowind      # release + line tables -> target/profiling/
cargo build --release -p bevyrrowind                # same codegen, no symbols -> target/release/
cargo build --release -p bevyrrowind --features solari,dlss   # ray tracing + DLSS ("ultra")
```

`dlss` builds need the SDK subset in `../sdk` (`sdk/fetch.sh`; `.cargo/config.toml` `[env]` points
at it); `build.rs` copies the two NVIDIA DLLs next to the executable. `dlss` alone (without
`solari`) is enough for Super Resolution on raster.

## Dev loop

Compile and bake times are gated like frame times (owner 2026-10-11: "must be super fast for the dev
cycle"). Gates and rules: `../superfargoth/docs/ARCHITECTURE.md` §8b. Measure on an idle machine (no other
`cargo`/`rustc`/`bevyrrowind` in `tasklist`), Ryzen 7 7435HS 16 threads, target on C: (junction).

| What (2026-10-12, health) | Command | Measured | Gate |
|---|---|---|---|
| No-op `cargo dev` build | `cargo build -p bevyrrowind --features dev` | 0.83 s | ≤ 1 s |
| Touch the app (rustc ~2 s + rust-lld 0.73 s) | `touch apps/bevyrrowind/src/main.rs` | 3.3-3.6 s | ≤ 5 s |
| Touch a leaf plugin crate (`mw_assets`, `sf_terrain_bevy`, `sf_lod_bevy`) | 2 crates rebuild | 4.5-4.7 s | ≤ 5 s |
| Touch `sf_world_bevy` / `mw_nif` / `sf_model_bevy` | 3 / 6 / 5 crates | 5.2 / 5.7 / 5.8 s | ≤ 8 s |
| Touch `sf_model` | 13 crates | 7.7-8.1 s | ≤ 8 s (at the limit) |
| Edit a function body (`sf_world_bevy`, `sf_model`, `mw_nif`) | one `black_box` line | 6.5-8.4 / 8.2-10.3 / 6.2-6.6 s | ≤ 10 s |
| Clean `cargo dev` build | `cargo build -p bevyrrowind --features dev --timings` (fresh `CARGO_TARGET_DIR`) | 8 min 38 s (579 units) | ≤ 12 min |
| Cold bake (fresh `--cache`) | `target/release/bevyrrowind.exe bake --cache <empty dir>` | 47.7 s | ≤ 2 min |
| No-op bake (all hits) | `target/release/bevyrrowind.exe bake` | 1.75 s | ≤ 2.5 s |

Where the time goes:

- Incremental: cargo's freshness check ~0.8 s, the app crate ~2 s of rustc (Bevy metadata and
  monomorphized systems), the link 0.73 s (`-Clink-arg=/time`; `/OPT:NOICF` saves nothing). Each crate
  between the edited one and the app adds 0.5-1 s (`sf_world_bevy` 0.85 s, `sf_lod_bevy` 1.1 s in a
  `--timings` run). `sf_model` has the widest fan-out (13 dependents): ARCHITECTURE §8b rule 5 applies.
- Clean: Bevy is the critical path. `bevy_ecs` (103 s) → `bevy_render` (226 s) → `bevy_pbr` (221 s) →
  `bevy_dylib` starts at 493 s of 518 s; all our crates are done by ~470 s. Biggest units by CPU:
  `bevy_render`, `bevy_pbr`, `bevy_ui` 186 s, `bevy_sprite_render` 173 s, `bevy_picking` 142 s,
  `bevy_dev_tools` 131 s, `wesl` 123 s (a hard dependency of `bevy_shader`), `bevy_gltf` 99 s,
  `bevy_animation` 99 s (the app's `3d`/`ui` feature profiles pull gltf, picking and UI widgets).
  Report: `C:/cargo-target/health-timings/clean-dev-2026-10-12.html` (outside the repos;
  the 4.1 GB scratch target it came from was deleted).
- Bake: per-step numbers in `docs/specs/bake-adapter.md` ("Dev-loop health check").

Tried and not adopted (no measurable gain): superfargoth crates at opt-level 1 in this workspace
(they are path dependencies, so `[profile.dev.package."*"]` builds them at opt-level 3 without line
tables; touch and body-edit times were within noise either way, and O3 keeps framework hot paths fast in
`cargo dev`), and the app crate at opt-level 0 (3.34 vs 3.37 s).

Benchmark scripts (re-runnable): touch each hot crate twice and time `cargo build -p bevyrrowind
--features dev`; for body edits insert `std::hint::black_box(N);` as the first line of a hot function and
restore the file afterwards.

## Quiet machine

Numbers from a busy machine are noise. The suite waits, before every run, until no `cargo`, `rustc`,
`bevyrrowind` or `ovid` process runs (other agents build here), GPU load <= 15 % and CPU load <= 20 %
(`--wait-for name,...` adds names; `--max-wait-min`, `--no-wait`). During the run it compares system and
process CPU (Bevy's `SystemInformationDiagnosticsPlugin`): other processes above 25 % mark the run busy and it
is retried (`--retries`, default 2). Busy runs are flagged in the report and the summary.

## Suite runner

Run from the repo root:

```sh
target/profiling/bevyrrowind.exe perf list                       # named scenarios and their arguments
target/profiling/bevyrrowind.exe perf suite <tag> [scenario...] --repeat 3 \
    --resolution 2560x1440 --resolution 1280x720 --floor [--off terrain] [--hitch-ms 25] \
    [--save benchmarks/runs/<tag>]
```

Scenarios: `balmora` (standing), `seyda_neen` (docks, standing), `red_mountain` (horizon view), `run`
(7 m/s), `flight` (60 m/s), `census` (interior). `--floor` adds a 960x600 run of each: the GPU drops out,
what remains is the CPU floor. Output in `target/perf/<tag>/`: per run `<scenario>.md` (frame stats,
gates, Bevy-timed GPU passes, per-frame stage costs, gauges incl. entities/meshes/lights, churn, hitch
table, nvidia-smi load/clock/power/temperature over the measured window), `.json`, per-frame `.csv`,
`.warmup.csv`, `gpu.csv`, `log.txt`; and `summary.md` (median of runs: median, p95, p99, 1 % low, worst,
hitches, GPU load, timed GPU passes, entity/mesh counts). `--save` copies md + json into the repo.

A single benchmark without the runner: `view <cell> --bench benchmarks/<file>.json [--resolution WxH]
[--bench-out dir] [--bench-baseline path|none] [--bench-seconds N] [--hitch-ms N] [--off ...]`.

## Frame pacing (measurement setup)

Streaming benchmarks are judged on presented *stutters* (frames over 1.5x the moving median of the
previous 60, per minute), not on worst/median (superfargoth `diag.md` "Frame pacing": counters, PresentMon
mapping, the setup matrix and why). The app adds `sf_diag_bevy::FramePhasePlugin`
in perf runs, so every CSV has `main/busy/ms`, `render/{thread,acquire,graph,present,busy}/ms`,
`gpu/timed/ms` and `frame/work/ms`. `benchmarks/exterior_run.json` and `exterior_flight.json` gate on
`max_stutters` (presented, <= 1 per minute) and presented frames <= 33 ms. Run them uncapped
(default `auto-novsync`) in a borderless window at the display's resolution: that is where render-thread
spikes show (2026-10-12: flight 21-27, run 3-9 stutters/min at 1080p; at 1440p the GPU-bound waits hide most
of them, and fifo at 60 Hz hides all of them).

Env switches (A/B, pass with `perf suite ... --env NAME=value`):

- `BEVYRROWIND_WINDOW=borderless`: borderless fullscreen on the primary monitor at its resolution. Use
  it for pacing runs. A window larger than the display (2560x1440 on a 1080p panel) measured the same as
  a fitting window rendering 2560x1440 offscreen (2026-10-12): its problem is the pixel count (GPU-bound
  waits hide render-thread spikes), not the window path. The OS may also shrink an oversized window (one
  spikes3 run got 1920x1009 and was invalid).
- `BEVYRROWIND_RENDER_SIZE=WxH`: render the exterior camera into an offscreen WxH image shown scaled
  in the window (a `Camera2d` + full-window `ImageNode`), so pixel count and window path vary
  independently (e.g. borderless 1080p window, 1440p pixels).
- `BEVYRROWIND_PRESENT=immediate|mailbox|fifo` (default `auto-novsync`).
- `BEVYRROWIND_LOADING_MESHES=<n>`: `StreamingConfig::max_loading_meshes` (default 64, 0 = unpaced).

Classify a run's spikes: `gawk -F, -v verbose=1 -f tools/pacing.awk target/perf/<tag>/<run>/exterior_flight.csv`
(per spike frame: present, acquire, render thread, work, main/render busy, meshes added, spawns, origin
shifts -> shift / stream / present / other; then frame and work stutters per minute and p99.9). Sample
the GPU's temperature and clock next to every batch (`nvidia-smi --query-gpu=temperature.gpu,clocks.gr,
power.draw --format=csv -l 5`): a laptop GPU throttling from ~2.6 to ~2.3 GHz moves the median by about 1 ms.

## Hitch table

Every frame over the threshold (`--hitch-ms` / `BEVYRROWIND_HITCH_MS`; default 2.5 x median, >= 25 ms),
warm-up (loading) included, with what changed in that frame and the one before: entities
(`ecs/entities_delta`), asset events per type (`assets/<Type>/added|modified|removed`), pipelines created
(`pipelines/created`), streaming stage items and times (`stream/*`), terrain, origin shifts
(`world/origin_shifts`), big_space propagation.

## A/B switches

`view ... --off a,b` (env `BEVYRROWIND_OFF`): `terrain` (no terrain camera: no selection, no draw),
`near`/`statics` (R1 placements not spawned), `far` (R2 placements), `farfield`/`lod` (no far-field
proxies), `sky` (no atmosphere: also the sun and its shadows), `envmap` (sky environment light only),
`shadows`, `lights`, `particles`, `nif` (models on the runtime NIF path), `denoiser` (DLSS Ray
Reconstruction off on `--rt`). `farshadows` (terrain/proxy/impostor far shadow and the dithered cascade
end off), `shadowtune` (the old 600 m / 20 m cascades instead of the derived 504 m / 10 m),
`sizecull` (no size-based draw distance: every object drawn to its ring or the far-field handoff). Compare against a run without.

Render options (not `--off` parts):

- `--quality <low|medium|high|ultra>` (env `BEVYRROWIND_QUALITY`, default high): one view-distance bias
  (0.75 / 0.875 / 1 / 1) for LOD selection, size culling and the far-field handoff (400 m × bias, rings
  follow), plus the size-culling cutoff (4 / 3 / 2 / 1 reference px, floor 16 / 20 / 24 / 32 m).
- `--dlss <off|auto|native|quality|balanced|performance|ultra-performance>` (env `BEVYRROWIND_DLSS`,
  default **auto** in `dlss` builds since 2026-10-12, off otherwise; `native` = DLAA): Super Resolution on
  raster, the Ray Reconstruction quality with `--rt`.
  Needs a `dlss` build and an NVIDIA RTX GPU (native elsewhere). Baselines comparable with older runs need
  `--env BEVYRROWIND_DLSS=off` (or `--dlss off`); DLSS runs use their own tag, e.g. `perf suite dlssq ... --env
  BEVYRROWIND_DLSS=quality`.
- `--prepass` (exteriors): depth + motion-vector prepass with occlusion culling, an A/B experiment.
  Measured slower in Balmora (13.9 vs 11.9 ms at 1440p: prepass 0.96 ms + depth downsample 0.72 ms).
  `--dlss` adds the same prepass on its own.

Reference (2026-10-12, RTX 4060 Laptop on AC, release `solari,dlss`, Balmora 2560x1440 median): native
11.9 ms, DLSS quality 8.9 (SR pass 2.1 ms), performance 9.2 (SR 2.8 ms), DLAA 11.7. Census Office 1440p:
raster 3.5, `--rt` + RR auto 8.4, `--rt --off denoiser` 15.4, RR quality 12.9, RR native 26.5.
Reports in `target/perf/dlss/`.

## GPU per pass (Nsight Systems, elevated)

Bevy's `RenderDiagnosticsPlugin` times only some passes. Nsight times every wgpu pass label:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/perf_nsys.ps1 -Name balmora1 [-Scenario balmora] [-Resolution 2560x1440] [-Delay 35] [-Duration 8] [-Off shadows] [-DryRun]
```

One UAC prompt (nsys registers its Vulkan layer); someone must be at the machine. Writes
`target/perf/nsys/<Name>-passes.md` (per-frame GPU ms per pass), the CSV, the `.nsys-rep`, and the run's
own sf_diag report. `bevyrrowind perf nsys <csv>` re-summarizes any `vulkan_gpu_marker_sum` CSV.

## CPU per system (trace)

```sh
cargo build --profile profiling-trace -p bevyrrowind --features trace   # own target dir
target/profiling-trace/bevyrrowind.exe perf suite trace1 balmora --trace   # TRACE_CHROME per run
target/profiling-trace/bevyrrowind.exe perf trace target/perf/trace1/<run>/trace.json   # measured window by default
target/profiling-trace/bevyrrowind.exe perf spikes <trace.json> --min-ms 3
```

Delete the multi-GB traces afterwards. `--features tracy` is the GUI alternative (Tracy).
