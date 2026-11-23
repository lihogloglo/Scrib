# Perf suite `gc_p0b`

- exe: `target/profiling/bench-gc.exe` (release)
- date: 2026-11-23 10:31:29 UTC
- commit: `75f496d87d6fb5d94f5238f68142afe5bba454f9` (**dirty**: 27 tracked files changed)
- exe sha256: `616ceba56770b9cf5030aae613cab4eb3eb10b620bc7d4c8d505163b3900aa0e` · modified 2026-11-23 10:14:24 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-23 05:32:17 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795428864796220 (`--seed 1795428864796220` replays the order)
  - arm `on` (base): shipped defaults
  - arm `worst`: --set groundcover.render.reserve_worst_case=true
  - arm `off`: --off=groundcover
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `gc_ascadian` groundcover standing: Ascadian Isles meadow east of Pelagiad (cell 2,-7, the `gc-ascadian` scene), eye height, 08:00
- valid runs: 18 of 18 (3 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 18 runs (window in front for every measured frame)

## A/B verdicts vs `on`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| gc_ascadian @ 2560x1440 | `on` (base) | 3 | 11.59 | 11.56-11.73 | | | | | | | | | |
| gc_ascadian @ 2560x1440 | `worst` | 3 | 38.05 | 12.53-38.61 | +26.46 | +26.32 | 9.0 | 0.100 | -0.39 | +0.66 | +0.16 | +26.69 | **slower** |
| gc_ascadian @ 2560x1440 | `off` | 3 | 10.46 | 10.46-10.48 | -1.12 | -1.12 | 0.0 | 0.100 | -0.31 | -0.53 | -0.31 | -1.04 | **faster** |
| balmora @ 2560x1440 | `on` (base) | 3 | 11.67 | 11.66-11.69 | | | | | | | | | |
| balmora @ 2560x1440 | `worst` | 3 | 12.68 | 12.59-12.77 | +1.01 | +0.99 | 9.0 | 0.100 | -0.26 | +0.23 | +0.20 | +0.90 | **slower** |
| balmora @ 2560x1440 | `off` | 3 | 11.69 | 11.69-11.71 | +0.02 | +0.01 | 8.0 | 0.200 | -0.25 | -0.28 | -0.09 | +0.02 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes | groundcover tiles | groundcover live slots |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| gc_ascadian 2560x1440 arm=off ×3 | 10.46 | 96 | 12.12 | 13.20 | 76 | 21.6 | 0/47, 0/47, 0/47 | 97 | 2625 | 87.3 | 73 | 8.34 | 8.08 | 8.36 | 4.66 | 9.39 | 43.7 k | 18.7 k | 355 | 26.44 | 31.94 | 10 | 12.1 k | 108 | 154.9 k | 9 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=worst ×3 | 38.05 | 26 | 88.56 | 98.53 | 10 | 103.4 | 0/39, 8/40, 5/22 | 99 | 2640 | 45.9 | 62 | 36.31 | 8.01 | 9.54 | 5.13 | 37.12 | 43.8 k | 18.7 k | 355 | 14.17 | 18.44 | 10 | 12.2 k | 158 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=on ×3 | 11.59 | 86 | 13.10 | 13.92 | 72 | 20.5 | 0/40, 0/39, 0/38 | 98 | 2625 | 89.0 | 72 | 9.70 | 8.39 | 8.88 | 4.97 | 10.43 | 43.8 k | 18.7 k | 355 | 25.87 | 32.07 | 10 | 12.2 k | 93 | 345.1 k | 7 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| balmora 2560x1440 arm=off ×3 | 11.69 | 86 | 12.99 | 13.43 | 74 | 76.3 | 2/28, 0/26, 0/25 | 98 | 2625 | 92.2 | 75 | 10.20 | 6.51 | 6.99 | 4.09 | 10.86 | 37.9 k | 15.5 k | 210 | 23.04 | 29.56 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=worst ×3 | 12.68 | 79 | 14.29 | 14.77 | 68 | 17.6 | 0/21, 0/28, 0/30 | 98 | 2625 | 89.9 | 73 | 11.11 | 6.50 | 7.49 | 4.38 | 11.74 | 38.0 k | 15.5 k | 210 | 24.04 | 29.50 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=on ×3 | 11.67 | 86 | 12.94 | 13.37 | 75 | 19.8 | 0/34, 0/33, 0/30 | 98 | 2625 | 92.0 | 75 | 10.19 | 6.76 | 7.27 | 4.18 | 10.84 | 37.9 k | 15.5 k | 210 | 24.36 | 29.28 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes | groundcover tiles | groundcover live slots |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| gc_ascadian 2560x1440 arm=off #1 | 10.46 | 96 | 12.08 | 13.20 | 76 | 21.6 | 0/47 | 97 | 2625 | 88.1 | 72 | 8.34 | 8.01 | 8.29 | 4.66 | 9.39 | 43.7 k | 18.7 k | 355 | 28.71 | 31.93 | 10 | 12.1 k | 146 | 154.9 k | 9 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=worst #1 | 12.53 | 80 | 13.96 | 14.42 | 69 | 17.1 | 0/39 | 98 | 2625 | 86.9 | 73 | 10.58 | 8.06 | 8.78 | 4.85 | 11.24 | 43.8 k | 18.7 k | 355 | 25 | 29.98 | 10 | 12.2 k | 187 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=on #1 | 11.59 | 86 | 12.95 | 13.45 | 74 | 20.5 | 0/40 | 98 | 2625 | 89.4 | 72 | 9.70 | 8.07 | 8.57 | 4.79 | 10.40 | 43.8 k | 18.7 k | 355 | 25 | 30.10 | 10 | 12.2 k | 131 | 345.1 k | 7 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=worst #2 | 38.05 | 26 | 88.56 | 99.97 | 10 | 103.4 | 8/40 | 99 | 2640 | 45.9 | 62 | 36.31 | 8.00 | 9.60 | 5.16 | 37.12 | 43.8 k | 18.7 k | 355 | 13.33 | 18.44 | 10 | 12.2 k | 113 | 1.50 M | 5 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=on #2 | 11.73 | 85 | 13.10 | 13.92 | 72 | 19.6 | 0/39 | 98 | 2625 | 89.0 | 72 | 9.70 | 8.39 | 8.88 | 4.97 | 10.45 | 43.8 k | 18.7 k | 355 | 25.87 | 32.07 | 11 | 12.2 k | 93 | 345.1 k | 5 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=off #2 | 10.48 | 95 | 12.48 | 13.90 | 72 | 17.5 | 0/47 | 97 | 2625 | 86.9 | 73 | 8.05 | 8.14 | 8.43 | 4.70 | 9.38 | 43.7 k | 18.7 k | 355 | 26.44 | 31.94 | 10 | 12.1 k | 72 | 154.8 k | 5 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=on #3 | 11.56 | 87 | 13.20 | 14.16 | 71 | 18.4 | 0/38 | 98 | 2625 | 88.7 | 72 | 9.65 | 8.52 | 9.20 | 5.14 | 10.43 | 43.8 k | 18.7 k | 355 | 27.82 | 33.45 | 10 | 12.1 k | 93 | 345.0 k | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| gc_ascadian 2560x1440 arm=off #3 | 10.46 | 96 | 12.12 | 13.09 | 76 | 15.1 | 0/47 | 97 | 2625 | 87.3 | 73 | 8.38 | 8.08 | 8.36 | 4.64 | 9.39 | 43.7 k | 18.7 k | 355 | 25.46 | 33.11 | 10 | 12.1 k | 108 | 154.9 k | 9 | 118 | 4 | 21.5 k | 3.15 M | — | — |
| gc_ascadian 2560x1440 arm=worst #3 | 38.61 | 26 | 90.35 | 98.53 | 10 | 100.6 | 5/22 | 99 | 2640 | 45.0 | 61 | 37.02 | 8.01 | 9.54 | 5.13 | 37.74 | 43.8 k | 18.7 k | 355 | 14.17 | 18.14 | 10 | 12.2 k | 158 | 1.50 M | 9 | 118 | 4 | 21.5 k | 3.15 M | 63 | 42.2 k |
| balmora 2560x1440 arm=off #1 | 11.69 | 86 | 13.05 | 13.64 | 73 | 76.3 | 2/28 | 98 | 2625 | 90.8 | 72 | 10.20 | 6.56 | 7.09 | 4.18 | 10.86 | 37.9 k | 15.5 k | 210 | 23.04 | 30.58 | 4 | 9408 | 62 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=worst #1 | 12.68 | 79 | 14.29 | 14.80 | 68 | 17.1 | 0/21 | 98 | 2625 | 90.0 | 73 | 11.11 | 6.43 | 7.48 | 4.36 | 11.74 | 38.0 k | 15.5 k | 210 | 25.93 | 29.69 | 4 | 9459 | 103 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=on #1 | 11.69 | 86 | 13.01 | 13.53 | 74 | 18.2 | 0/34 | 98 | 2625 | 92.0 | 75 | 10.19 | 6.77 | 7.60 | 4.44 | 10.85 | 37.9 k | 15.5 k | 210 | 24.66 | 29.61 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=worst #2 | 12.59 | 79 | 14.16 | 14.58 | 69 | 16.2 | 0/28 | 98 | 2625 | 89.9 | 74 | 11.04 | 6.50 | 7.49 | 4.38 | 11.67 | 38.0 k | 15.5 k | 210 | 22.14 | 29.50 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=on #2 | 11.66 | 86 | 12.94 | 13.36 | 75 | 19.8 | 0/33 | 98 | 2625 | 91.9 | 75 | 10.15 | 6.57 | 7.14 | 4.09 | 10.81 | 37.9 k | 15.5 k | 210 | 23.08 | 29.00 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off #2 | 11.71 | 85 | 12.99 | 13.43 | 74 | 23.2 | 0/26 | 98 | 2625 | 92.3 | 75 | 10.22 | 6.51 | 6.99 | 4.09 | 10.87 | 37.9 k | 15.5 k | 210 | 23.22 | 28.91 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=on #3 | 11.67 | 86 | 12.94 | 13.37 | 75 | 19.1 | 0/30 | 98 | 2625 | 92.7 | 75 | 10.19 | 6.76 | 7.27 | 4.18 | 10.84 | 37.9 k | 15.5 k | 210 | 24.36 | 29.28 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
| balmora 2560x1440 arm=off #3 | 11.69 | 86 | 12.95 | 13.42 | 75 | 18.5 | 0/25 | 98 | 2625 | 92.2 | 75 | 10.19 | 6.49 | 6.94 | 4.08 | 10.85 | 37.9 k | 15.5 k | 210 | 22.42 | 29.56 | 4 | 9408 | 64 | 153.9 k | 5 | 101 | 4 | 19.2 k | 3.15 M | — | — |
| balmora 2560x1440 arm=worst #3 | 12.77 | 78 | 14.40 | 14.77 | 68 | 17.6 | 0/30 | 98 | 2625 | 88.2 | 73 | 11.18 | 6.68 | 7.84 | 4.63 | 11.81 | 38.0 k | 15.5 k | 210 | 24.04 | 29.18 | 4 | 9459 | 105 | 1.50 M | 5 | 101 | 4 | 19.2 k | 3.15 M | 63 | 0 |
