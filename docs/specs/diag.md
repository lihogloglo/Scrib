# Benchmark and diagnostics harness

Status: implemented (2026-10-11), core and plugin; not yet wired into an app
Crates: `sf_diag` (new, Bevy-free: counters, recorder, stats, gates,
reports, baselines, camera paths, scenario runner), `sf_diag_bevy` (new:
plugin, frame sampling, fly-through driver)

## Goal

Answer "how fast is it, and did it get slower?" with numbers a machine can
check (ARCHITECTURE §2 principle 6, WISHLIST §0). A scenario warms up, then
measures for N seconds or along a scripted camera path. It writes a
per-frame CSV and a JSON report, evaluates gates (the §0 targets), and
compares against a stored baseline. A failed gate or a regression fails
the run, and with `exit_on_finish` the process exits with code 1, so CI
and scripts can use it.

Other systems report **named counters** through one cheap, thread-safe API:
streaming stages (time, items, queue depth, oldest request age), bake steps
(per-step time and the ≤ 2 min first-bake gate), anything else.

Non-goals: a profiler (Tracy does the "why", see below), an on-screen
overlay (Bevy's `FpsOverlayPlugin` and `DiagnosticsOverlayPlugin` exist),
and GPU memory counters (see open questions).

**Synthetic vs real content.** Framework examples and tests use synthetic
worlds (superfargoth can't depend on a game's data), and those prove
correctness only. Performance gates, visual judgments, and calibration of
distances and budgets run on a real content pack (Bevyrrowind), because
real content behaves differently (little instance reuse, vertex colors,
alpha-tested parts, legacy-format models).

## Canonical technique, and what we reuse

How studios gate performance: a **repeatable scripted run** (fixed camera
path or replay after a warm-up) on fixed hardware, recording **every frame
time**, judged by **percentiles and spikes**, not averages (an average hides
hitches), and compared against the last good run, where a regression past a
tolerance fails the build. Unity's Performance Testing package and
Unreal's Gauntlet/CSV profiler work this way, and so do the frame-time tools
reviewers use (CapFrameX, PresentMon), which report median fps and
1st-percentile ("1 % low") fps.

Bevy 0.20 already provides, and we reuse rather than rebuild:

| Bevy piece | Use |
|---|---|
| `Time<Real>` | Frame time = interval between consecutive frame starts. Read at the frame boundary (`First`, after `TimeSystems`), the same source as `FrameTimeDiagnosticsPlugin`, which only records it later in `Update`. That would misalign it by a frame against counters. |
| `DiagnosticsStore` | Every Bevy diagnostic the app enables is sampled each frame into a counter column under its own path: `RenderDiagnosticsPlugin` (per-pass CPU and **GPU timestamp** timings, `render/<pass>/elapsed_gpu`), `EntityCountDiagnosticsPlugin`, `SystemInformationDiagnosticsPlugin`, render-asset and mesh-allocator counts. `sf_diag_bevy` therefore needs no rendering feature. |
| `trace_tracy` (app feature) | Profiling. Stage code adds `tracing` spans next to its counters; Tracy shows them along with Bevy's GPU zones. |
| `TimeUpdateStrategy::ManualDuration` | Deterministic frame times in headless tests. |
| `AppExit` | Exit code for scripted runs. |
| `bevy_dev_tools` `FpsOverlayPlugin`, CI testing (screenshots on frame N) | Complementary. The app adds them; we don't wrap them. |

Crates considered and not used: `iyes_perf_ui` and similar overlays (they
display, they don't gate, and they lag Bevy releases); `hdrhistogram` (a
benchmark run is at most ~100k frames, so exact sorted percentiles are
cheap and simpler); `bevy_math` cubic splines (uniform Catmull-Rom only,
no arc-length parameterization, and the core must stay Bevy-free).

Deviations, on purpose: percentiles are exact (linear interpolation,
Hyndman & Fan type 7). Paths are **centripetal** Catmull-Rom (Yuksel,
Schaefer, Keyser 2011: no cusps or self-loops with unevenly spaced
keyframes), re-parameterized by arc length so the camera moves at a
constant speed in m/s (7 m/s run, 60 m/s flight, streaming.md). The path
advances by **real** frame time, so a slow frame means a longer jump, as
it would for a player, and streaming feels true pressure.

## Data model

- **Counters** (`Counters`, `Sync`, shared as `Arc`): a registry of named
  slots, each `Sum` (accumulates within a frame: stage ms, items) or
  `Gauge` (last value wins: queue depth, request age), with a unit suffix
  (`ms`, `count`, `bytes`, or Bevy's suffix). A slot is one `AtomicU64`
  holding f64 bits (NaN = not touched), so systems and task-pool jobs add
  through `&self` without a lock or an ECS write conflict. Names are
  `area/stage/metric`, e.g. `stream/spawn/ms`, `bake/mesh.cook/ms`.
- **Recorder**: the frame-time column plus one column per counter (Sum:
  0 when untouched; Gauge: missing), and run totals. `commit_frame` drains
  the counters into a row; `flush` drains into totals only (CLI tools such
  as the bake have no frames).
- **Report** (JSON, `format: 1`, the baseline format too): scenario name,
  app-supplied metadata (machine, build, preset), frame stats (count, mean,
  min, median, p95, p99, max, median fps, 1st-percentile fps, spike count),
  per-counter summaries (unit, kind, total, per-frame stats), the worst
  spikes (frame, time, frame ms, top `ms` counters that frame: the culprit
  list), gate results, regressions, and `passed`.
- **CSV**: `frame,t_s,frame_ms,<counter>...`, one row per measured frame.
- **Scenario** (serde, a data file the content pack owns): name, warm-up
  (seconds and frames, both must pass, plus an app-controlled hold, e.g.
  until streaming is idle), measurement (`duration`, `frames`, or `path`
  with keyframes and speed), gates, spike factor (default 1.5), tolerance.
- **Gates** (serde enum): `max_frame_ms`, `worst_over_median`, `min_fps`
  (from a frame stat), `max_frame_stat`, `max_spikes`, `max_counter` (a
  counter's total or per-frame stat), `counter_worst_over_median`,
  `max_stutters` (frames over a factor × the moving median of the previous
  `window` frames, per minute; on a counter or the presented frame time;
  "Frame pacing" below). Presets: `Gate::wishlist_streaming()`
  = worst < 1.5× median and no frame > 33 ms (the original reading);
  `Gate::wishlist_streaming_stutters()` = the streaming gate as measured since
  2026-10-12 ("Frame pacing"); `Gate::target_fps(f)` = median
  fps ≥ f. A gate without data fails.
- **Baseline comparison**: median, p95 and p99 frame time, and the total
  and p95 of every `ms` counter. A regression is `current > baseline × (1 +
  relative)` **and** `current − baseline > absolute` (defaults 10 % and
  0.25 ms, so sub-millisecond noise doesn't fail runs). Worst frame is gated
  absolutely, not compared, since it is the noisiest statistic.

## Frame pacing (2026-10-12, spikes2–4)

**Work vs delivery.** The presented interval (`frame_ms`, present to
present) is what the player sees, but it mixes the engine's work with
waiting on the swap chain and the compositor. A frame's *work* is its
longest busy stage, which is what streaming changes can fix:

| counter (`sf_diag::pacing`, `sf_diag_bevy::FramePhasePlugin`) | meaning | nearest PresentMon 2.x metric |
|---|---|---|
| `frame_ms` | present to present | `MsBetweenPresents` (FrameTime) |
| `main/busy/ms` | main schedule, first to last system | `MsCPUBusy` (one thread of it) |
| `render/thread/ms` − `render/acquire/ms` − `render/present/ms` = `render/busy/ms` | render thread without swap-chain waits | `MsCPUBusy` (render thread) |
| `render/acquire/ms`, `render/present/ms` | swap-chain acquire, final submit + present | `MsCPUWait` |
| `gpu/timed/ms` | sum of Bevy's timed passes (`*/elapsed_gpu`), carried forward; untimed passes missing | `MsGPUBusy` (partial) |
| `frame/work/ms` | max(main busy, render busy, GPU timed) | none (PresentMon has no single "work" number) |

We keep our `area/stage/metric` names (renaming would break the CSVs and
baselines) and map them to PresentMon in this table. PresentMon itself
is the outside check to add (not wired yet; it sees `DisplayedTime`
and `MsAnimationError`, which an app can't measure from inside).

**Stutter, not worst/median.** The WISHLIST §0 gate was read as "worst
frame < 1.5× the median frame". One frame decides that ratio, so one OS
hiccup fails a run and one lucky run passes it; and a global median
penalizes a flight whose scene gets heavier on the way. The gate now
counts *stutters*: presented frames over 1.5× the **moving median of the
previous 60 frames** (`pacing::stutters`, `Gate::MaxStutters`), per minute,
at most `STREAMING_STUTTERS_PER_MIN` (1), plus a hard cap on the presented
frame (33 ms): `Gate::wishlist_streaming_stutters()`, in
`benchmarks/exterior_{run,flight}.json`. The report adds stutters/min on
the presented frames and, for diagnosis, on `frame/work/ms`.

**Measured, and why the gate is on presented frames** (spikes4,
2026-10-12, RTX 4060 Laptop on AC, release, DLSS off, Seyda Neen run 7 m/s
and flight 60 m/s, 5 setups interleaved × 3 rounds; GPU 2565–2580 MHz,
76–85 °C under load the whole matrix, so the cells compare with each other
but not with the throttled spikes3 runs; `target/perf/mx-*`):

| setup | flight median | flight stutters/min | run median | run stutters/min |
|---|---|---|---|---|
| 2560×1440 window on the 1080p panel | 10.9–11.1 ms | 2.5–5.0 | 10.5–10.6 ms | 0–1.5 |
| borderless 1080p, offscreen 2560×1440 | 11.1–11.4 | 0.8–6.7 | 10.9 | 3.0–4.5 |
| borderless 1080p (`AutoNoVsync`) | 8.5–8.7 | 21.0–26.8 | 8.7–8.9 | 3.0–9.1 |
| borderless 1080p, mailbox | 8.5 | 21.8–26.0 | 8.7–8.9 | 3.0–6.0 |
| borderless 1080p, fifo (60 Hz panel) | 16.7 | 0 | 16.7 | 0–1.5 |

- The window path doesn't matter at equal pixel count (rows 1 and 2): an
  oversized window and a fitting one rendering 1440p offscreen give the
  same medians and stutter rates. Pixel count does: at 1440p the GPU is
  the limiter and its waits absorb render-thread spikes.
- With GPU slack (1080p, ~118 fps), render-thread busy spikes (12–15 ms
  against a ~6 ms median, mostly in frames without mesh, spawn or origin
  churn, plus ~4 origin shifts per flight) surface as 21–27 stutters a
  minute on the flight; ~90 % of them coincide with a work stutter
  (`frame/work/ms`, same or adjacent frame). Present-path stutters (long
  present, normal work) are 1–3 per run.
- Fifo at 60 Hz hides all of it: every work spike fits the 16.7 ms
  interval, so the presented frames show 0 stutters while `frame/work/ms`
  counts 116–158 "stutters" a minute against its own median. Work over its
  own median is therefore no gate (spikes3's plan): spikes inside the slack
  are invisible. Work over the *presented* median undercounts (pipelining
  adds ~2 ms between render-thread work and the presented interval).
- So the gate is on what the player sees, measured where it is strictest:
  uncapped, in a borderless window that fits the display (perf.md "Frame
  pacing"). Today it fails: flight 21–27, run 3–9 stutters a minute
  (target ≤ 1). The work counters attribute each stutter.

**Industry comparison** (searched 2026-10-12). PresentMon 2.x (Intel,
also the base of NVIDIA FrameView and CapFrameX) splits each frame into
`MsCPUBusy`/`MsCPUWait` and `MsGPUBusy`/`MsGPUWait` next to
`MsBetweenPresents`, `MsBetweenDisplayChange`/`DisplayedTime` and
`MsAnimationError` (CPU delta vs display delta): the same work-vs-delivery
split as ours. Benchmarkers summarize with 1 % and 0.1 % lows (p99/p99.9
frame time) and count stutters relative to the *recent* frame rate
(CapFrameX's stutter metric, after NVIDIA's driver-team rule of a frame
over ~2.5–3× the average; Digital Foundry shows the frame-time graph and
calls out individual hitches). Unreal's hitch detector
(`FEnginePerformanceTargets`) needs a frame over an absolute threshold
*and* a ratio over the previous frame, with a minimum time between
hitches. Nobody gates on a single worst/median ratio. Our deviations: the
factor stays at WISHLIST's 1.5× (stricter than 2.5×, because it is a
streaming budget, not a "did the player notice" test), the reference is a
moving median (robust to the single previous frame being short, unlike
Unreal's previous-frame ratio), and the count is per minute so run lengths
compare. Sources: PresentMon `README-ConsoleApplication.md` (metric
definitions), Unreal `FEnginePerformanceTargets` API docs, CapFrameX forum
thread on PCGH (stutter factor).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
Frame pacing is compared above; this covers the rest of the benchmark
method (bevyrrowind `docs/perf.md` is the procedure). "Unverified" marks
claims from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Per-frame named counters to CSV, scripted runs | UE CSV Profiler (`csvprofile`) + Gauntlet automated perf tests with fly-through/replay paths; Unity Performance Testing package (warm-up, measurement count, sample groups) + Performance Benchmark Reporter (unverified detail) | `sf_diag` counters, scenarios (warm-up, hold, path), JSON + CSV | **match** |
| Baselines and regression thresholds | Mozilla Perfherder: medians of the 12 runs before vs after, t-test confidence, manual sheriffing; Chromium Pinpoint bisects with repeated A/B runs (unverified detail); UE/Unity studios: per-scenario budgets in CI | relative + absolute thresholds per counter, `--repeat`, median of runs | **partial**: thresholds are fixed, not statistical. With 3 interleaved rounds the report shows ranges, but no test says whether an A/B delta beats run-to-run noise. Open question 4 |
| Quiet machine, repeated and interleaved runs | universal; Perfherder retriggers noisy jobs | waits for idle, flags busy runs, interleaves setups (spikes4) | **match** |
| **Fixed GPU clocks for A/B** | Nsight Graphics GPU Trace locks clocks to base by default (= DX12 `SetStablePowerState`); NVIDIA recommends `nvidia-smi --lock-gpu-clocks` (+ memory clocks) for comparable numbers; console teams profile on fixed-clock dev kits | logs clock/temperature/power next to each batch; doesn't lock (laptop GPU throttles 2.6 → 2.3 GHz, ~1 ms of median) | **GAP (P2, small)**: open question 5. Logging is the right fallback when locking isn't allowed (needs admin, may be unsupported on laptop GeForce) |
| External present/display timing | PresentMon 2.x / FrameView / CapFrameX | mapped, not wired (above) | known gap (above) |
| GPU frame total, per-pass GPU, VRAM | UE `stat gpu`, `stat RHI`, Insights; Unity Profiler + Memory Profiler; PIX/Nsight captures | per-pass from `RenderDiagnosticsPlugin` (partial), Nsight procedure in perf.md, no VRAM | known gap (open question 2) |
| CPU timeline capture | Unreal Insights, Superluminal, PIX, Tracy | `--features tracy`, Nsight Systems | **match** |
| Hitch attribution | UE hitch detector + `stat dumphitches`; Unity Profiler frame selection | hitch table + `pacing.awk` classification by cause | **match** |

**Sources** (checked 2026-10-12):
- NVIDIA, Nsight Graphics GPU Trace "Lock Clocks to Base" (equivalent to `SetStablePowerState`): https://developer.nvidia.com/blog/optimizing-vk-vkr-and-dx12-dxr-applications-using-nsight-graphics-gpu-trace-advanced-mode-metrics
- NVIDIA developer forums, clock locking with `nvidia-smi -lgc` vs profiler clock control: https://forums.developer.nvidia.com/t/reading-performance-counters-vs-disabling-frequency-throttling/249866
- Mozilla, Perfherder alert method (12-point medians, t-test confidence): https://wiki.mozilla.org/TestEngineering/Performance/Sheriffing and https://firefox-source-docs.mozilla.org/testing/perfdocs/perftest-in-a-nutshell.html
- UE CSV Profiler and Gauntlet, Unity Performance Testing package: docs not re-read this session (unverified detail).

## Interfaces

```rust
// sf_diag (Bevy-free)
let counters = Arc::new(Counters::new());
let id = counters.register("stream/spawn/ms", CounterKind::Sum, "ms");
counters.add(id, 0.4);                       // or add_duration / set / *_named
let mut rec = Recorder::new();
rec.commit_frame(frame_ms, &counters);       // per frame; `flush` without frames
let mut report = Report::build("census", &rec, &gates, 1.5, meta);
report.compare(&baseline, "baselines/census.json", &Tolerance::default())?;
report.save(path)?; rec.write_csv(w)?;
let mut run = ScenarioRun::new(scenario)?;   // warm-up → measure → done
let step = run.advance(frame_ms, hold);      // pose to apply, record or not
```

```rust
// sf_diag_bevy
app.add_plugins(SfDiagPlugin::new(DiagSettings { output_dir, baseline, exit_on_finish, .. })
    .with_scenario(scenario)?);
fn my_stage(counters: Res<DiagCounters>) { counters.add_duration_named("stream/spawn/ms", t) }
// Entities with `FlyThroughCamera` get the path pose (f32 `Transform`). Apps on
// big_space set `apply_transform: false` and convert `FlyThroughPose` (f64)
// to grid coordinates themselves.
```

`DiagCounters` is available with or without a scenario (it costs an
atomic add per call). `DiagScenario::hold` delays the end of warm-up.
`DiagReport` holds the finished report.

## Verification

- `sf_diag` unit tests: percentiles against hand-computed values; gate
  pass and fail cases including missing data; spike attribution; CSV
  header and escaping; JSON round trip; baseline regression vs noise
  floor; constant-speed path sampling (equal arc steps), passing through
  keyframes, no overshoot with uneven spacing; scenario phases. Frame pacing:
  `derive_work` (max of the busy columns, GPU passes carried forward, no
  column without a busy time), moving-median stutters (a ramp is none, a
  spike is one), `max_stutters` per minute, missing data fails, JSON form.
- `sf_diag_bevy`: headless app (Bevy's minimal plugin set, manual 8 ms
  frames) runs warm-up and a path scenario, moves a `FlyThroughCamera`,
  records a counter, writes the JSON and CSV, and exits through `AppExit`.
- First real use: the M1 Census Office baseline (interior, 144 fps target),
  then the M2 fly-throughs (streaming.md) and the bake budget (bake.md).

## Open questions

1. **Worst frame vs noise.** WISHLIST §0 gates the single worst frame. One
   OS hitch in a 10k-frame run fails it. Keep it strict on the reference
   machine, and if it proves flaky, switch to p99.9 or "frames over budget
   per minute" (both are one `Gate` variant away) and log it in §10.
2. **GPU frame time and VRAM.** Per-pass GPU timings come from
   `RenderDiagnosticsPlugin`, but there is no single GPU frame total, and
   VRAM needs wgpu's allocator report from the render world. Both belong in
   a small render-side plugin when M2/M7 need them.
3. Comparing non-time counters (bytes, counts) against the baseline: a
   change there is often content, not a regression. Report-only for now.
4. **A/B significance (J9, P2).** Add a per-counter noise test to
   `Report::compare` and the suite summary: with ≥ 3 runs per side, a
   bootstrap confidence interval of the median difference (or
   Mann-Whitney U over per-run medians), reported next to the delta, and a
   regression only when the interval excludes zero *and* the thresholds
   above hold. Perfherder-style, sized for 3–5 runs.
5. **Clock locking (J9, P2).** `perf suite --lock-clocks`: try
   `nvidia-smi -lgc <base>,<base>` (+ `-lmc`) before the batch and `-rgc`
   after, needs elevation; when refused, keep today's logging and mark the
   run "unlocked" in the report so A/Bs across throttled and cool runs
   aren't compared silently.
