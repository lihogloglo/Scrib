# Perf suite `gc_p0`

- exe: `target/profiling/bench-gc.exe` (release)
- date: 2026-11-23 09:37:20 UTC
- commit: `75f496d87d6fb5d94f5238f68142afe5bba454f9` (**dirty**: 18 tracked files changed)
- exe sha256: `f48cffb34074bb04a4bd3608ca26e1370bea146b0328fcd0500716234081a195` · modified 2026-11-23 09:18:24 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-23 05:32:17 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795425512479896 (`--seed 1795425512479896` replays the order)
  - arm `on` (base): shipped defaults
  - arm `off`: --off=groundcover
  - arm `empty`: --set groundcover.density_scale=0
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `gc_ascadian` groundcover standing: Ascadian Isles meadow east of Pelagiad (cell 2,-7, the `gc-ascadian` scene), eye height, 08:00
- valid runs: 17 of 18 (3 warm-up runs discarded)
- **dropped**: gc_ascadian @ 2560x1440 #1 [empty]: failed: no report (exit Some(ExitStatus(ExitStatus(0)))); see target/perf\gc_p0\gc_ascadian@2560x1440_arm=empty-1\log.txt
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 17 runs (window in front for every measured frame)

## A/B verdicts vs `on`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| gc_ascadian @ 2560x1440 | `on` (base) | 3 | 38.21 | 37.36-40.57 | | | | | | | | | |
| gc_ascadian @ 2560x1440 | `off` | 3 | 10.44 | 10.40-10.49 | -27.77 | -27.77 | 0.0 | 0.100 | +0.12 | -1.29 | -0.58 | -26.86 | **faster** |
| gc_ascadian @ 2560x1440 | `empty` | 2 | 24.11 | 11.52-36.70 | -14.10 | -13.67 | 0.0 | 0.200 | +0.11 | -0.29 | -0.07 | -13.52 | inconclusive |
| balmora @ 2560x1440 | `on` (base) | 3 | 12.70 | 12.61-12.84 | | | | | | | | | |
| balmora @ 2560x1440 | `off` | 3 | 11.69 | 11.68-11.75 | -1.01 | -0.94 | 0.0 | 0.100 | +0.23 | -0.26 | -0.11 | -0.91 | **faster** |
| balmora @ 2560x1440 | `empty` | 3 | 12.65 | 12.58-12.71 | -0.04 | -0.03 | 3.0 | 0.700 | +0.01 | -0.04 | -0.05 | -0.08 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes | groundcover tiles | groundcover live slots |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| gc_ascadian 2560x1440 arm=off ×3 | 10.44 | 96 | 12.44 | 13.41 | 75 | 19.9 | 0/48, 0/48, 0/52 | 97 | 2625 | 87.5 | 75 | 8.14 | 8.16 | 8.39 | 4.67 | 9.40 | 43.7 k | 18.7 k | 355 | 27.10 | 32.27 | 10 | 12.1 k | 70 | 154.8 k | 5 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=on ×3 | 38.21 | 26 | 65.35 | 98.03 | 10 | 103.6 | 0/14, 9/37, 11/31 | 99 | 2625 | 46.5 | 65 | 35.52 | 8.04 | 9.68 | 5.25 | 36.25 | 43.8 k | 18.7 k | 355 | 14.02 | 18.20 | 10 | 12.2 k | 187 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=empty ×2 | 24.11 | 41 | 49.25 | 53.58 | 19 | 95.0 | 0/37, 7/37 | 99 | 2632 | 65.8 | 69 | 21.98 | 8.15 | 9.39 | 5.18 | 22.73 | 43.8 k | 18.7 k | 355 | 20.54 | 26.13 | 10.50 | 12.2 k | 150 | 1.50 M | 7 | 118 | 4 | 21.5 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=empty ×3 | 12.65 | 79 | 14.12 | 14.56 | 69 | 18.6 | 0/31, 0/25, 0/21 | 98 | 2625 | 90.9 | 78 | 11.06 | 6.46 | 7.39 | 4.29 | 11.69 | 38.0 k | 15.5 k | 210 | 24.52 | 29.67 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off ×3 | 11.69 | 86 | 13.12 | 13.68 | 73 | 19.8 | 0/32, 0/21, 0/29 | 98 | 2625 | 93.1 | 77 | 10.19 | 6.68 | 7.17 | 4.23 | 10.86 | 37.9 k | 15.5 k | 210 | 24.94 | 31.01 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=on ×3 | 12.70 | 79 | 14.11 | 14.64 | 68 | 22.8 | 0/28, 0/21, 0/37 | 98 | 2625 | 90.5 | 78 | 11.14 | 6.45 | 7.43 | 4.34 | 11.77 | 38.0 k | 15.5 k | 210 | 23.21 | 28.88 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes | groundcover tiles | groundcover live slots |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| gc_ascadian 2560x1440 arm=off #1 | 10.49 | 95 | 12.44 | 13.41 | 75 | 17.9 | 0/48 | 97 | 2625 | 87.5 | 75 | 8.14 | 8.16 | 8.39 | 4.67 | 9.39 | 43.7 k | 18.7 k | 355 | 29.33 | 35.98 | 10 | 12.1 k | 70 | 154.8 k | 5 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=on #1 | 40.57 | 25 | 48.92 | 49.56 | 20 | 49.9 | 0/14 | 99 | 2625 | 46.5 | 65 | 35.52 | 8.05 | 9.72 | 5.30 | 36.25 | 43.8 k | 18.7 k | 355 | 13.19 | 18.20 | 10 | 12.2 k | 151 | 1.50 M | 7 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=off #2 | 10.44 | 96 | 12.11 | 13.14 | 76 | 18.3 | 0/48 | 97 | 2625 | 88.9 | 76 | 8.44 | 8.02 | 8.25 | 4.65 | 9.40 | 43.7 k | 18.7 k | 355 | 27.10 | 31.49 | 10 | 12.2 k | 146 | 154.9 k | 9 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=on #2 | 38.21 | 26 | 89.91 | 100.99 | 10 | 103.6 | 9/37 | 99 | 2625 | 45.9 | 65 | 36.95 | 8.04 | 9.62 | 5.14 | 37.66 | 43.8 k | 18.7 k | 355 | 14.02 | 17.71 | 11 | 12.2 k | 204 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=empty #2 | 11.52 | 87 | 13.14 | 13.90 | 72 | 17.0 | 0/37 | 98 | 2625 | 87.2 | 75 | 9.52 | 8.39 | 8.93 | 5.00 | 10.30 | 43.8 k | 18.7 k | 355 | 27.23 | 33.22 | 10 | 12.2 k | 187 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 0 |
| gc_ascadian 2560x1440 arm=on #3 | 37.36 | 27 | 65.35 | 98.03 | 10 | 102.6 | 11/31 | 99 | 2625 | 47.5 | 64 | 34.73 | 7.96 | 9.68 | 5.25 | 35.43 | 43.8 k | 18.7 k | 355 | 14.22 | 18.47 | 10 | 12.2 k | 187 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=empty #3 | 36.70 | 27 | 85.37 | 93.25 | 11 | 95.0 | 7/37 | 99 | 2640 | 44.4 | 63 | 34.45 | 7.92 | 9.85 | 5.36 | 35.16 | 43.8 k | 18.7 k | 355 | 13.84 | 19.03 | 11 | 12.2 k | 113 | 1.50 M | 5 | 118 | 4 | 21.5 k | 3.15 M | 63 | 0 |
| gc_ascadian 2560x1440 arm=off #3 | 10.40 | 96 | 12.49 | 13.85 | 72 | 19.9 | 0/52 | 96 | 2625 | 87.0 | 74 | 7.98 | 8.19 | 8.49 | 4.71 | 9.40 | 43.7 k | 18.7 k | 355 | 25.96 | 32.27 | 10 | 12.1 k | 70 | 154.8 k | 5 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| balmora 2560x1440 arm=empty #1 | 12.71 | 79 | 14.27 | 14.84 | 67 | 18.6 | 0/31 | 98 | 2625 | 89.4 | 74 | 11.14 | 6.36 | 7.37 | 4.29 | 11.77 | 38.0 k | 15.5 k | 210 | 24.52 | 29.15 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off #1 | 11.75 | 85 | 13.12 | 13.83 | 72 | 17.8 | 0/32 | 98 | 2625 | 91.0 | 75 | 10.25 | 6.56 | 7.04 | 4.15 | 10.90 | 37.9 k | 15.5 k | 210 | 24.04 | 30.10 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=on #1 | 12.70 | 79 | 14.04 | 14.64 | 68 | 17.8 | 0/28 | 98 | 2625 | 90.0 | 76 | 11.14 | 6.47 | 7.43 | 4.34 | 11.77 | 38.0 k | 15.5 k | 210 | 23.21 | 28.88 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off #2 | 11.69 | 86 | 13.12 | 13.68 | 73 | 19.8 | 0/21 | 98 | 2625 | 93.1 | 77 | 10.19 | 6.73 | 7.43 | 4.44 | 10.86 | 37.9 k | 15.5 k | 210 | 24.94 | 31.01 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=on #2 | 12.84 | 78 | 14.54 | 15.02 | 67 | 22.8 | 0/21 | 98 | 2625 | 90.5 | 78 | 11.17 | 6.45 | 7.51 | 4.40 | 11.83 | 38.0 k | 15.5 k | 210 | 26.44 | 30.21 | 4 | 9459 | 103 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=empty #2 | 12.65 | 79 | 14.12 | 14.51 | 69 | 16.8 | 0/25 | 98 | 2625 | 90.9 | 78 | 11.06 | 6.67 | 7.72 | 4.54 | 11.69 | 38.0 k | 15.5 k | 210 | 26.01 | 29.67 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=on #3 | 12.61 | 79 | 14.11 | 14.58 | 69 | 17.1 | 0/37 | 98 | 2625 | 91.1 | 78 | 11.02 | 6.42 | 7.40 | 4.30 | 11.65 | 38.0 k | 15.5 k | 210 | 22.54 | 28.13 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=empty #3 | 12.58 | 79 | 14.06 | 14.56 | 69 | 16.9 | 0/21 | 98 | 2625 | 91.7 | 78 | 11.01 | 6.46 | 7.39 | 4.28 | 11.64 | 38.0 k | 15.5 k | 210 | 24.52 | 30.35 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off #3 | 11.68 | 86 | 12.99 | 13.45 | 74 | 17.9 | 0/29 | 98 | 2625 | 93.5 | 79 | 10.18 | 6.68 | 7.17 | 4.23 | 10.84 | 37.9 k | 15.5 k | 210 | 25.22 | 31.77 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
