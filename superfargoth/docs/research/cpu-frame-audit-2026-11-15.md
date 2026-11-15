# CPU frame audit: where the walk frame goes, threading, SPUD, plan to 8.3 ms (2026-11-15)

Diagnosis only: nothing built, run or changed. Scene `balmora_walk`, High, 2560x1440 + DLSS Quality, RTX 4060 Laptop.
Sources: run `dkeys_on` (untraced, after fixes A + B1; `bevyrrowind/benchmarks/runs/dkeys_on/balmora_walk@2560x1440-2.md`),
traces `bevyrrowind/target/perf/{tcd_trbase,tcd_trpbr,tr_on,tr_off}.trace.txt` (traced, **before** A + B1), the lean
ladder (`lean_summary-2026-11-15.md`), Bevy 0.20 vendored source and wgpu-core 30.0.1. **F** = fact (measured or read
in source), **I** = inference. Traced numbers are ~1.2-1.4x the untraced walls and are used for shares only.

## 1. Where the 12.7 ms goes

**Frame model (I, fits the numbers):** frame = render thread + extract. `render/thread` 11.49 + extract ~1.1
(`ExtractSchedule` 1.25 traced) = 12.6 vs 12.46-12.73 measured. The main thread (9.27) finishes early and waits.

| stage | ms | source |
|---|---:|---|
| **Render thread** | **11.49** | `render/thread/ms`, dkeys_on :68 (F) |
| `RenderGraph`: run view schedules, encode, submit | 6.68 | `render/graph/ms` :71 (F) |
| - recording the passes (systems) | ~0.5 | "CPU ms" column: main opaque 0.064, prepass 0.097 (F) |
| - `encoder.finish()` = the real wgpu encode, one task per pass | ~3-4 wall | trace: cascades 3.35, main opaque 2.33, prepass 2.31, lamp faces 1.76 thread-ms, in parallel (F); wall = the longest one (I) |
| - `queue.submit`, serial | ~1.3-1.7 | trace `queue_submit` rows sum 1.73; **1.70 with the near ring off** (`tr_off`) (F) |
| prepare / queue / extract-command apply | ~4.2 | 11.49 - 6.68 - present 0.47 - acquire 0.09 (F); content: `write_binned_instance_buffers` x3 1.49, preprocess bind groups 0.35, mesh bind groups 0.24, `GpuShaderBuffer` 0.52 (traced thread-ms) |
| present + acquire | 0.57 | :75, :87 (F) |
| **Main thread** | **9.27** | `main/busy/ms` :70 (F) |
| PostUpdate / Update / rest | ~59 % / 24 % / 17 % | traced 7.16 / 2.87 / 2.1 of 12.15 (`tcd_trbase`); untraced split **not measured** |
| **GPU (timed passes)** | 7.46 walk, 8.87 standing | DLSS 2.08, main opaque 1.66, prepass 0.52, bloom 0.32, atmosphere LUTs 0.32, water velocity + SSPR 0.50 (F) |

Counts on the walk (dkeys_on gauges :305-341): 46 views (1 main, 3 cascades, 42 lamp faces, most skipped by the face
cache), 113 batch sets (main 66, cascades 48), 93 distinct pipelines, 7351 bins, 19.5 k phase items, 17 mesh slabs,
1 material slab, 59.2 k entities, 31.6 k meshes.

Three facts that change the picture:

1. **No heavy system exists on the main thread.** In `tr_off` (near ring off) PostUpdate is still 3.42 ms and Update
   2.20 with no system above 0.45 ms; 535-603 executor spans per frame. The main thread's time is many small systems
   and fork/join waits, not one hot loop (F for the trace, I for the cause).
2. **`main/busy` follows render load, not main-world work.** Render-only changes move it: PBR maps off 9.83 -> 8.85,
   all shadows off 8.77 -> 8.11, Low 7.46 (lean ladder). Removing the CPU scans of near statics does **not** move it
   (H1: 9.56 vs 9.35). So a large part of `main/busy` is waiting for pool threads the render world is using (I).
3. **The floor is already close to the target.** Near ring off: 7.2-7.7 ms, GPU timed 7.24. Leanest High (all niceties
   and shadows off) is 9.37 walk, still CPU bound (main 8.11, render 7.84). Standing is GPU bound today at 8.87 ms of
   timed passes, above 8.3 on its own. 120 fps needs CPU **and** GPU cuts.

## 2. Threading verdict

**What runs in parallel (F):**
- Pipelined rendering is on: `DefaultPlugins` with only `TransformPlugin` disabled
  (`mw_exterior_bevy/src/exterior.rs:299-303`); main and render overlap, joined at extract.
- Task pools are Bevy's defaults (no `TaskPoolPlugin` override anywhere in the game): 8 compute threads, 4 IO.
  Each compute thread is busy ~4.9 ms of a 17.3 ms traced frame (28 %); process CPU is 24 % of the machine.
- Pass encoding is parallel **across** passes: every pass system owns an encoder, and `finish_parallel` finishes them
  on the compute pool (`vendor/bevy_render/src/renderer/render_context.rs:117-148`). wgpu-core 30 defers the real
  encode to `finish` (`wgpu-core-30.0.1/src/command/mod.rs:1093`), which is why recording costs 0.06 ms and finishing 2.3.
- Visibility scans and extraction use `par_iter`.

**What is serial (F):**
- `camera_driver` runs each view's schedule one after another, exclusively (`vendor/bevy_core_pipeline/src/schedule.rs:211-275`).
- **All sun cascades go into one encoder** (`per_view_shadow_pass` loops the view's lights,
  `vendor/bevy_pbr/src/render/light.rs:3730-3809`): 3.35 ms traced, the longest single encode, so it sets the wall
  time of the parallel finish (I).
- `queue.submit` (36-61 command buffers per call, ~1.7 ms traced per frame), on the render thread.
- The visibility scans are chained (ranges -> camera -> lights -> mark hidden) and reach ~1.9x parallelism
  (`statics-cpu-cost-2026-11-12.md:33`).
- Main and render worlds share the one 8-thread pool; multi-ms `finish` tasks do not yield (fact 2 above).

**Bevy features not used, and why:** `NoCpuCulling` (lost twice, opt-in); `OcclusionCulling` is already inserted on the
camera (`exterior.rs:881`, its condition not checked here) and saves GPU, not this CPU cost; `MeshAllocatorSettings` is left at defaults (17 mesh slabs; slab and
vertex layout both split batch sets); `TaskPoolOptions` untouched. Nothing obvious is switched off by mistake. We use
Bevy as designed; the cost is Bevy's and wgpu's per-pass and per-batch-set overhead, which is far above a native
engine's (tens of us per batch set against ~1 us per draw in a Vulkan renderer).

## 3. Why SPUD is faster

What the claim is measured on (our note `spud-code-2026-11-08.md:20-24`, from SPUD's `docs/PERF_CHANGES.md`): ~375 fps
(2.67 ms), GPU mean 1.75 ms, **hardware and resolution not stated**, running **Skyrim SE** at the commit we read (not
Morrowind), forward lighting with 4x MSAA. Not re-checked today.

- **It does far less on the GPU.** No upscaler, atmosphere, clouds, froxel fog, water FFT/SSPR, SSAO, bloom, PCSS or
  shadowed lamps. Our DLSS pass alone (2.08 ms) is more than SPUD's whole GPU frame; our timed GPU is 7.5-8.9 ms.
  That part of "2x" is features, not engine.
- **Its CPU frame has no second world.** No extraction, no per-view retained bins, no per-pass command encoder with
  validation, no deferred re-encode at `finish`, no usage-scope tracking at submit. It writes Vulkan commands once.
- **Draw calls = pipeline states, and there are few.** Cull mode, depth test/write and bias are dynamic state; one
  vertex arena and one index arena. We have 93 distinct pipelines and 113 batch sets because wgpu bakes those states
  into the pipeline, vertex layouts differ and meshes sit in 17 slabs.
- **Statics are a flat array per cell** culled in one loop: no per-entity visibility components, no hash maps.

What does **not** explain it: entity count. Our own SPUD-style path (`--scene`) reached parity only and is frozen
(statics ledger 2026-11-12; statics alone ~1.1 ms of the walk). The gap is per-pass and per-batch-set overhead plus
the feature set, so the plan below does not reopen the static scene.

## 4. Plan to 8.3 ms (walk needs about -4.4 ms CPU; standing needs about -1.5 ms GPU)

Measurements first (the orchestrator runs them; each is one small diagnostic slice):

- **M1. Untraced per-stage walls.** `Instant` gauges like `phases.rs` for First / PreUpdate / Update / PostUpdate on
  main, and extract-commands / prepare / queue / `camera_driver` / finish / submit on render. Settles the untraced
  split of the 9.27 and the 4.2 that this audit could only take from traces.
- **M2. Pool contention.** Same exe, walk x3: compute threads 8 vs 12 (`TaskPoolOptions`), and once with
  `PipelinedRenderingPlugin` disabled (diagnostic only: frame becomes main + render, and `main/busy` then shows the
  main thread's true cost alone). Confirms or kills fact 2's inference.
- **M3. Encode cost per encoder.** Untraced `Instant` around each `encoder.finish()` and around `queue.submit`
  (`render_context.rs:130-137`, `schedule.rs:317`), reported per system name with its batch-set count. Gives us
  per batch set, the critical-path encoder and the real submit cost.

| # | change | industry reference | expected walk (untraced) | size | risk | needs |
|---|---|---|---|---|---|---|
| 1 | **One encoder per shadow view** (flush the encoder between cascades in `per_view_shadow_pass`), so cascades encode in parallel | UE parallel command lists per pass and per cascade (`FParallelCommandListSet`); id Tech 7 one job per view | -0.5 to -1.0: longest encode drops from the 3-cascade one to main opaque / prepass (I) | S (vendored patch) | low | M3 |
| 2 | **Fewer pipelines and batch sets**: finish B2 (one vertex layout), then one mesh slab per layout (`MeshAllocatorSettings`), fold remaining material key bits per class | UE: one vertex factory per static mesh type, state-sorted mesh draw commands; SPUD: draws = pipeline states | -1.0 to -2.0 for 113 -> ~60 sets, at the 27-57 us per set that A and B1 measured | M | low | M3 for the rate |
| 3 | **Cut `queue.submit`**: fewer command buffers (merge the small compute/post encoders into one; 36-61 today) and check what wgpu tracks per buffer for the 2.5 k-texture slab | Native engines submit 1-4 command buffers per frame (RAGE, Decima: one per worker per pass) | -0.5 to -1.0 of ~1.5 serial | S-M | low | M3 |
| 4 | **Stagger the far cascades**: redraw cascade 2-3 every other frame or on camera travel, keep 0-1 per frame | UE CSM caching and virtual shadow map page cache; RAGE/Decima update far cascades at a lower rate | -0.4 to -0.8 (all shadows off is -1.55, cascades are 48 of 113 sets) | M (vendored; `SkipRootViewRendering` covers lamp faces only) | medium: sun-shadow lag, owner visual check | none |
| 5 | **Give render its own threads / size the pool** | UE: separate game, render and RHI threads plus a task graph; no shared small pool | 0 to -1.0 on `main/busy`; frame only once render is cut | S | low | M2 |
| 6 | **Thin the main schedules**: merge or gate the many sub-0.1 ms systems, run conditions on idle ones | Decima / id Tech: a job graph of a few dozen coarse jobs per frame, not hundreds of tiny ones | unknown until M1; the main thread must drop 9.3 -> ~7 for 8.3 to hold | M | low | M1 |
| 7 | **GPU budget for standing**: lighter DLSS preset or cost check (2.08 ms; Performance mode measured *slower*, 2.71), time-slice atmosphere LUTs, half-rate SSPR | UE real-time sky capture time slicing; half-rate SSR is standard | -1.0 to -1.5 standing GPU, ~0 on the walk today | S-M | medium: image quality, owner check | none |
| 8 | NPCs (~2.0 ms of the walk on 2026-11-12, before P4-P6) | crowd LOD and animation budgets (UE animation budget allocator) | re-measure first: `--off actors` A/B on today's build | - | - | A/B |

Order: M1-M3, then 1, 2, 3 (render thread, the bound), 5, 4, then 6 and 7. Items 1-3 sum to -2.0 to -4.0 (I): they
overlap, since all three shorten the same encode + submit stage, so count on the low half. Reaching 8.3 on High then
still needs item 4 or 6, and item 7 for standing. If M3 shows that wgpu costs tens of us per batch set even with one
slab and one layout, the remaining lever is upstream (a wgpu issue with our numbers), not more engine work on our side.

## 5. Not established here

- The untraced split of the main thread (M1) and the cause of fact 2 (M2).
- What makes a batch set cost tens of us in wgpu after fix A. Read: `set_bind_group` merges every resource of the
  group into the pass scope and each flush walks its texture init actions (`wgpu-core-30.0.1/src/command/pass.rs:111-117,
  :148-166`); `multi_draw_indirect` itself has no per-resource loop (`render.rs:3231`). Not measured (M3).
- SPUD's hardware, resolution and current numbers.
