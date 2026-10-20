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

## Frame budget (ROADMAP 1.2)

Target (owner decision 1): High, 2560x1440 output + DLSS Quality (1707x960 internal) on the RTX 4060
Laptop: **median <= 11 ms** in every exterior benchmark (`max_frame_stat` median gate in
`benchmarks/exterior_*.json`; the gates assume this setup: `perf suite <tag> --resolution 2560x1440
--env BEVYRROWIND_DLSS=quality` with a `dlss` build). Bevy pipelines the main thread, the render thread
and the GPU, so a frame costs about the slowest of the three: each gets the whole 11 ms minus ~1.5 ms
headroom for spikes and thermal clocks.

| lane (High, 1440p DLSS Q) | budget ms | Balmora | Seyda Neen | Red Mountain | flight |
|---|---:|---:|---:|---:|---:|
| frame (median) | 11.0 | 14.3 | 17.7 | 17.2 | 19.3 |
| render thread busy | 9.5 | 12.9 | 15.9 | 15.5 | 16.7 |
| - of it graph encoding | 4.0 | 5.0 | 6.1 | 5.8 | 5.9 |
| main thread busy | 9.5 | 8.8 | 10.0 | 10.5 | 13.7 |
| GPU (Bevy-timed) | 9.5 | 8.4 | 8.7 | 7.9 | 7.7 |
| - opaque + prepass | 3.5 | 2.6 | 2.1 | 2.0 | 2.0 |
| - DLSS SR | 2.1 | 2.1 | 2.1 | 2.0 | 2.0 |
| - sky, clouds, fog, light probes | 1.5 | 1.6 | 1.6 | 1.7 | 1.7 |

Measured 2026-10-19 (phase1, `target/perf/p1-dlssq`, profiling `dlss` build, 2 runs each, quiet machine,
GPU 2.58 GHz, 65 °C): **the render thread is the bottleneck everywhere** (GPU 53-78 % busy), at
44-52 k meshes and 65-88 k entities. Next: a per-system trace of the render thread (extract, prepare,
queue, batching per view and shadow cascade), then cut what is over. Medium and Low get their own rows
once High meets its budget.

What costs what (Balmora, same session, warm machine: baseline 15.3 ms; A/B runs vary +-0.4 ms, so only
differences above ~0.5 ms count):

| `--off` | frame | render | main | GPU | reading |
|---|---:|---:|---:|---:|---|
| (none) | 15.3 | 13.7 | 9.2 | 8.5 | reference |
| shadows | 10.5 | 9.1 | 8.5 | 7.9 | **sun cascades: -4.8 ms**, all CPU (12.6 k caster entries over 4 cascades) |
| actors | 12.4 | 11.2 | 5.8 | 8.6 | **actors: -3 ms main** (anim, skinning, skinned AABBs), -2 ms render |
| farshadows, particles, lights | 15.1-15.3 | 13.4-13.6 | 9.0-9.2 | 8.3-8.4 | nothing measurable |
| shadowcache (old build) | 14.8 | 13.3 | 9.0 | 8.6 | -0.5 ms: the parked cube cache's tracking; its plugin is gone now |
| castercull (on vs off, 3 pairs each) | 14.7 / 15.0 | 13.1 / 13.4 | | | caster size rule removes 8 % of entries: ~-0.3 ms, at the noise floor; off again (shadows popped with view) |
| farfield | 37.3 | 34.7 | 24.5 | 10.5 | the far field is what makes towns affordable |

Trace (`perf trace`, profiling-trace build): the main thread waits ~4.5 ms per frame in `RenderExtractApp` for the
render thread (Bevy's pipelined extract), confirming the render thread as the critical path. Its biggest
spans: `prepare_preprocess_bind_groups` 2.3 ms, `queue_submit` 2.4 ms (33 submits), shadow
`write_binned_instance_buffers` 1.4 ms + shadow pass encoding 1.2 ms, skins 1.2 ms. Main thread:
big_space low-precision propagation 2.5 ms (task time), visibility 2-4 ms (x16 views), animation 1.5 ms.
Cuts to make (they belong to 1.4 shadows, 1.9 culling levers and 3.x animation): fewer or cheaper caster
draws per cascade (static cascade caching or a far cascade at half rate), actor animation LOD, and
fewer views (check what the 16 visibility passes are).

**The gameplay view: `balmora_walk`** (eye height on Balmora's path grid, 4 m/s, 254 m; added 2026-10-19).
It is the worst case: **25.8-26.0 ms** median, render 23.9-24.3, main 12.1, GPU 8.9. Whole-system A/B on it:

| `--off` | frame | render | main | reading |
|---|---:|---:|---:|---|
| statics (= near) | **7.5** | 4.7 | 3.5 | **the near-ring statics are the cost**: everything else is small |
| sky (no sun, so no sun shadows) | 17.1 | 15.2 | 9.7 | mostly the shadows below |
| shadows | 18.3 | 16.4 | 9.4 | -7.7 ms walking (-4.8 standing): cascades re-cull as they follow the camera |
| actors | 21.2 | 19.8 | 7.4 | -4.8 ms main, -4.1 render |
| physics, fog, terrain, walkers | 25.0-25.7 | | | -0.3 to -1 ms each |
| clouds, froxels, envmap, godrays, far | 25.5-26.3 | | | nothing |

Tried and dropped: Bevy 0.20 `NoCpuCulling` on static opaque/masked parts (the research's top item): **81 ms**
on the walk (render 81, GPU 22): with no CPU culling every part is queued for every view (cascades, point-light
faces, the water snapshot) and every LOD level, and the GPU path doesn't make up for it in our setup.
**Lead (not yet tested):** Bevy runs the GPU frustum + `VisibilityRange` test only in views with GPU culling; a view
with `NoIndirectDrawing` (`PreprocessingOnly`, e.g. our extra sun views in `sf_sky_bevy` `sun_view.rs`) culls a
`NoCpuCulling` mesh nowhere, so every part at every LOD draws there. Retry once every view supports GPU culling. Reverted the same day: the animation budget (1 ms; it "saved" ~1 ms by freezing poses: characters slid) and the
caster size rule (its cascades refit with the view, so small casters lost and regained shadows as the camera turned).
So the lever is the number of static mesh entities in the near ring and the views they're queued in: merge statics
per cell at bake time (OpenMW active-grid paging, MGE XE: a cell in one or two draws;
`docs/research/perf-morrowind-engines.md`) and cut shadow views; the big_space propagation patch and skinned
bounds next (`superfargoth/docs/research/perf/README.md`).

**Bare baseline and precombine (2026-10-20, precombine3; profiling build without `dlss`, native 1440p TAA, 1 run each,
`target/perf/{bare-pre,full-pre,pc-on,pc-off,bare-pc}`).** "Bare" = terrain + models only (`--off sky,envmap,shadows,
farshadows,lights,particles,clouds,fog,froxels,actors,walkers,physics,terrainblend`; `terrainblend` is gone since 2026-10-20):

| run | bare | bare + precombine | full, `--off precombine` | full + precombine | meshes full (off → on) |
|---|---:|---:|---:|---:|---:|
| balmora | 7.1 (GPU 98 %) | 7.0 | 15.3 | 15.7 | 46.1 k → 25.1 k |
| balmora_walk | 6.1 | 5.9 | 24.4 | 25.0 | 55.1 k → 32.4 k |
| seyda_neen | 7.3 (GPU 97 %) | 7.2 | 16.3 | 17.7 | 46.2 k → 23.3 k |
| CPU floor 960x600 | 4.8-5.4 | 4.7-5.6 | | | |

Reading: models + terrain alone are GPU-bound at 1440p native and far under budget. Precombine halves the mesh
entities but **does not move the frame** (walk render thread 22.6 → 23.3 ms, main 11.2 → 9.9): the render-thread
cost does not scale with static mesh entities, so the `--off statics` 7.5 ms above came from something else that
switch also removes (it drops every near placement: ~290 point lights, NIF-path objects, items, doors). Next:
per-system render-thread trace of the walk and `--off lights` on the walk.

**Feature ladder (2026-10-20, precombine3; same build, 1 run each, other processes ~27 % CPU throughout,
`target/perf/ladder-NN-*`).** Each step turns one system back on, cumulatively. `balmora_walk`, frame median ms (Δ):

| step | 1440p | Δ | render Δ | GPU Δ | 1707x960 | Δ |
|---|---:|---:|---:|---:|---:|---:|
| bare (terrain + models) | 6.5 | | | | 6.1 | |
| + sky | 6.4 | 0 | 0 | +0.1 | 6.3 | +0.2 |
| + envmap | 7.1 | +0.7 | +0.7 | +1.2 | 6.5 | +0.2 |
| + shadows | 8.0 | +0.8 | +0.9 | 0 | 7.2 | +0.7 |
| + farshadows | 8.4 | +0.4 | +0.4 | +0.3 | 7.8 | +0.6 |
| + lights | 10.4 | +2.0 | +2.0 | +1.7 | 9.9 | +2.2 |
| + fog, froxels | 11.1 | +0.7 | +0.6 | +0.7 | 10.2 | +0.3 |
| + clouds | 11.7 | +0.7 | +0.7 | +0.6 | 10.3 | +0.1 |
| + particles | 11.6 | 0 | 0 | 0 | 11.1 | +0.9 |
| + actors | 14.8 | +3.2 | +2.9 (main +2.2) | 0 | 14.3 | +3.2 |
| + walkers, physics | 15.0 | +0.2 | +0.2 (main +1.7) | 0 | 14.9 | +0.5 |
| + terrainblend (= full) | 25.9 | **+10.9** | +10.5 | +2.0 | 27.9 | **+13.0** |

Standing (`balmora`) 1440p: bare 7.1, envmap +1.5, shadows +1.2, lights +1.3, fog +0.8, terrainblend +2.5,
full 15.9. Reading: the walk is render-thread bound (full: GPU 10.9 vs frame 25.9), so lower internal
resolution doesn't help; terrain blend (non-batched `GroundBlendMaterial` per part) was the walk's main cost
and is removed (owner 2026-10-20). Next: lights, then shadows.

**Terrain blend removed (2026-10-20, lights1; ROADMAP 1.6; profiling build, `lod.near` v3 re-baked, 1 run each,
CPU ~26 % background, `target/perf/tb-removed` vs `ladder-12-terrainblend`).** Frame median ms, render thread busy
in brackets:

| run | before | after | Δ |
|---|---:|---:|---:|
| balmora 1440p | 15.9 (13.5) | **13.1** (8.2, GPU 98 %) | -2.8 |
| balmora 960p | 14.6 (12.9) | **9.5** (7.7) | -5.1 |
| balmora_walk 1440p | 25.9 (23.8) | **14.7** (13.0) | -11.2 |
| balmora_walk 960p | 27.9 (26.5) | **14.6** (12.8) | -13.3 |

Standing Balmora at 1440p is now GPU bound; the walk is still render-thread bound (13 ms busy: lights, shadows,
actors next). Meshes 25.1 k → 24.6 k (precombine buckets no longer split by band).

**Lights (2026-10-20, lights1; superfargoth `specs/lights.md`; same setup, 1 run each).** Frame median ms
(render thread busy; GPU timed):

| run (1440p / 960p) | today | stale views fixed | `--point-shadows 0` | `--off lights` |
|---|---:|---:|---:|---:|
| balmora | 13.1 / 9.5 | 13.5 / 9.2 | 13.0 / 9.2 | 12.4 / 9.5 (GPU 10.1 vs 10.7) |
| balmora_walk | 14.7 / 14.6 (13.0; 8.7) | **12.7 / 12.4** (10.7; 8.1) | 11.2 / 11.0 (9.3; 7.5) | 11.5 / 11.6 (9.6; 7.4) |

Reading: the ~290 unshadowed lights cost ~0.5 ms GPU standing and nothing measurable on the walk (Bevy 0.20 GPU
clustering). The walk's light cost was High's **one shadowed lamp**, plus a Bevy 0.20 leak: a lamp that loses its
shadow keeps its 6 cube-face views, each running the whole `Core3d` schedule (trace: 15.7 `Core3d` runs/frame for
a budget of 1). `sf_sky_bevy` now despawns them (shadows.md "Stale cube-face views"): walk -2.0 ms. The one lamp
still costs ~1.5 ms frame / 1.4 ms render thread on the walk (`staleviews` vs `ps0`). Traces: `trl-on`/`trl-off`
(deleted, summaries only). Note the walk's frame (12.7) sits above every lane (render 10.7, main 9.4, GPU 8.1):
most likely extraction (`RenderExtractApp` ~2 ms inclusive), which runs between the main and render threads.

**Audit (2026-10-20, lights1): Nsight + A/B traces, walk and Balmora.** Nsight (`target/perf/nsys/walk1-passes.md`,
walk 1440p, GPU sum 10.2 ms): main opaque 3.16, `atmosphere_luts` 0.69 (every frame), cascade 3 0.59, env-map
filtering 0.54 + 0.29 (every frame), indirect parameters 0.51, TAA 0.43, prepass 0.39; the one lamp's six faces
0.17 in all (its cost is CPU). Fixes and leads (1 run each, `target/perf/audit1*`):

| item | finding | change | measured |
|---|---|---|---|
| big_space low-precision propagation | walked every static mesh every frame (no dirty-tree pruning) | vendored patch (`superfargoth/vendor/big_space`) | trace 2.7 + 0.7 → 0.44 + 0.27 ms main-thread tasks; walk main busy 9.45 → 8.78 |
| sky env map | re-filtered every frame | `EnvMapRefresh` every 8 frames, `--off envslice` | Balmora 1440p 13.3 → 12.4, 960p 9.5 → 8.9 |
| NIF particles | CPU quads rewritten into `Mesh` assets every frame: Bevy rescans all `Mesh3d` (0.74 ms), re-allocates and re-prepares meshes | **open**: GPU-expanded billboards from a buffer | `--off particles`: walk 1440p 12.7 → 11.7 (render -1.0, main -0.9) |
| `atmosphere_luts` | Bevy recomputes all LUTs each frame | open (upstream) | 0.69 ms GPU |

Anti-aliasing at native 1440p, Balmora (ROADMAP 1.3): MSAA 18.5 ms (GPU 14.3), **TAA 15.4 ms (GPU 12.1, TAA
pass 0.39)**, DLSS Quality 14.8 (GPU 8.6). TAA is cheaper than the 4x MSAA it replaces.

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
`sizecull` (no size-based draw distance: every object drawn to its ring or the far-field handoff), `precombine` (near
cells spawn one entity per static again instead of their merged `lod.near` buckets; no rebake). Compare against a run without.

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
