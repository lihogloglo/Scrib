# Perf suite `pt_night`

- exe: `target/profiling/perftests.exe` (release)
- date: 2026-11-17 09:08:41 UTC
- commit: `c4d9ef509bc4dad4ae1eb5bdc12a8e0a892705c2` (**dirty**: 20 tracked files changed)
- exe sha256: `8499f6a0e88aa33660b0e9c8f6572e089222c1e0131fe64811e0c42232516c2f` · modified 2026-11-17 07:43:48 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-17 07:07:55 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality --env BEVYRROWIND_WINDOW=borderless · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 5 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1794903885996578 (`--seed 1794903885996578` replays the order)
  - arm `base` (base): shipped defaults
  - arm `nocull`: --env BEVYRROWIND_GPU_CULL_STATICS=1
  - arm `facecull`: --set shadow.lamp.face_cull=on
  - arm `lampcache`: --set shadow.lamp.cache.enabled=on
  - arm `sungate`: --set shadow.sun.gate=on
- scenarios: `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- valid runs: 25 of 25 (5 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 25 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk_night @ 2560x1440 | `base` (base) | 5 | 10.69 | 10.40-10.96 | | | | | | | | | |
| balmora_walk_night @ 2560x1440 | `nocull` | 5 | 10.47 | 10.36-10.65 | -0.22 | -0.13 | 6.0 | 0.222 | -0.31 | -0.04 | -0.27 | -0.01 | inconclusive |
| balmora_walk_night @ 2560x1440 | `facecull` | 5 | 10.23 | 10.15-10.60 | -0.46 | -0.29 | 2.0 | 0.032 | -0.12 | -0.52 | -0.44 | -0.10 | **faster** |
| balmora_walk_night @ 2560x1440 | `lampcache` | 5 | 10.83 | 10.76-10.95 | +0.14 | +0.26 | 16.0 | 0.548 | +0.18 | +0.94 | +0.96 | +0.27 | inconclusive |
| balmora_walk_night @ 2560x1440 | `sungate` | 5 | 10.41 | 10.29-10.75 | -0.29 | -0.22 | 7.0 | 0.310 | -0.06 | -0.22 | -0.20 | -0.05 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=base ×5 | 10.69 | 94 | 14.62 | 16.38 | 61 | 30.5 | 3/48, 0/47, 2/47, 6/45, 3/45 | 72 | 2625 | 75.3 | 76 | 6.68 | 9.06 | 9.18 | 5.25 | 7.41 | 59.4 k | 31.8 k | 300 | 26.71 | 31.54 | 51 | 14.6 k | 71 | 10.5 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate ×5 | 10.41 | 96 | 14.47 | 16.25 | 62 | 33.6 | 3/39, 4/50, 2/48, 4/51, 4/51 | 72 | 2625 | 76.0 | 76 | 6.66 | 9.00 | 8.97 | 5.04 | 7.36 | 59.4 k | 31.8 k | 300 | 26.44 | 31.74 | 51 | 14.5 k | 69 | 10.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull ×5 | 10.47 | 95 | 14.85 | 17.05 | 59 | 30.7 | 2/55, 7/54, 3/58, 5/56, 3/47 | 72 | 2625 | 75.3 | 75 | 6.64 | 8.76 | 9.14 | 4.98 | 7.40 | 59.4 k | 31.8 k | 300 | 25.48 | 30.38 | 45 | 55.0 k | 71 | 41.5 k | 16 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache ×5 | 10.83 | 92 | 16.92 | 19.15 | 52 | 42.7 | 5/47, 5/50, 2/49, 4/46, 3/49 | 69 | 2625 | 71.5 | 74 | 6.96 | 9.24 | 10.12 | 6.20 | 7.68 | 59.4 k | 31.3 k | 299 | 23.66 | 29.04 | 5 | 11.0 k | 72 | 10.1 k | 21 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull ×5 | 10.23 | 98 | 13.77 | 15.58 | 64 | 32.9 | 2/52, 5/51, 5/54, 5/44, 3/48 | 73 | 2625 | 77.4 | 76 | 6.54 | 8.95 | 8.66 | 4.81 | 7.30 | 59.4 k | 31.8 k | 300 | 26.92 | 31.91 | 51 | 14.6 k | 63 | 10.5 k | 11 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=base #1 | 10.50 | 95 | 14.71 | 16.38 | 61 | 29.2 | 3/48 | 72 | 2625 | 75.3 | 76 | 6.68 | 9.06 | 9.04 | 5.06 | 7.36 | 59.4 k | 31.8 k | 300 | 26.92 | 31.83 | 51 | 14.5 k | 69 | 10.1 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate #1 | 10.75 | 93 | 14.77 | 16.76 | 60 | 32.0 | 3/39 | 71 | 2625 | 75.2 | 76 | 6.69 | 9.21 | 9.31 | 5.26 | 7.41 | 59.4 k | 31.6 k | 300 | 26.61 | 31.74 | 51 | 14.5 k | 71 | 10.4 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull #1 | 10.36 | 97 | 14.83 | 16.57 | 60 | 30.7 | 2/55 | 73 | 2623 | 76.3 | 76 | 6.75 | 8.62 | 9.03 | 4.87 | 7.40 | 59.4 k | 31.8 k | 300 | 25.48 | 30.36 | 51 | 55.0 k | 71 | 41.5 k | 16 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache #1 | 10.76 | 93 | 15.61 | 17.26 | 58 | 35.1 | 5/47 | 71 | 2625 | 73.9 | 75 | 6.81 | 8.92 | 9.65 | 5.86 | 7.63 | 59.4 k | 31.4 k | 299 | 24.52 | 29.40 | 5 | 11.1 k | 71 | 9811 | 18 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull #1 | 10.20 | 98 | 12.96 | 14.81 | 68 | 31.5 | 2/52 | 74 | 2625 | 78.1 | 77 | 6.49 | 8.90 | 8.56 | 4.78 | 7.30 | 59.4 k | 31.8 k | 300 | 26.86 | 32.11 | 51 | 14.6 k | 64 | 10.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache #2 | 10.85 | 92 | 18.22 | 21.39 | 47 | 32.0 | 5/50 | 67 | 2625 | 70.1 | 74 | 7.14 | 9.51 | 10.67 | 6.67 | 7.75 | 59.4 k | 31.0 k | 299 | 23.66 | 29.04 | 9 | 11.0 k | 74 | 10.6 k | 22 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull #2 | 10.23 | 98 | 13.43 | 15.58 | 64 | 27.5 | 5/51 | 73 | 2625 | 77.6 | 77 | 6.54 | 8.95 | 8.62 | 4.76 | 7.29 | 59.4 k | 31.8 k | 300 | 27.18 | 32.43 | 51 | 14.6 k | 63 | 10.6 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate #2 | 10.74 | 93 | 14.78 | 17.51 | 57 | 33.6 | 4/50 | 71 | 2625 | 75.0 | 76 | 6.70 | 9.20 | 9.27 | 5.23 | 7.40 | 59.4 k | 31.7 k | 300 | 26.21 | 31.87 | 45 | 14.4 k | 71 | 10.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull #2 | 10.47 | 95 | 14.85 | 16.65 | 60 | 30.1 | 7/54 | 72 | 2620 | 75.4 | 76 | 6.62 | 8.78 | 9.14 | 4.97 | 7.42 | 59.4 k | 31.8 k | 300 | 25.48 | 30.45 | 51 | 55.1 k | 69 | 41.5 k | 16 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=base #2 | 10.96 | 91 | 14.79 | 16.93 | 59 | 26.9 | 0/47 | 69 | 2625 | 73.4 | 75 | 6.80 | 9.34 | 9.58 | 5.42 | 7.41 | 59.4 k | 31.7 k | 300 | 25.96 | 31.90 | 51 | 14.7 k | 71 | 10.7 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate #3 | 10.32 | 97 | 14.39 | 16.25 | 62 | 31.5 | 2/48 | 73 | 2625 | 76.7 | 76 | 6.51 | 8.91 | 8.79 | 4.93 | 7.36 | 59.4 k | 31.8 k | 300 | 26.44 | 31.94 | 51 | 14.5 k | 69 | 10.1 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache #3 | 10.95 | 91 | 16.92 | 18.49 | 54 | 30.1 | 2/49 | 69 | 2625 | 73.1 | 75 | 6.80 | 9.23 | 10.01 | 6.19 | 7.68 | 59.4 k | 31.1 k | 299 | 23.66 | 28.91 | 9 | 11.0 k | 71 | 9922 | 18 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=base #3 | 10.69 | 94 | 14.62 | 16.41 | 61 | 28.1 | 2/47 | 72 | 2625 | 75.9 | 76 | 6.68 | 8.98 | 9.18 | 5.25 | 7.46 | 59.4 k | 31.7 k | 300 | 26.71 | 31.49 | 45 | 14.5 k | 72 | 10.5 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull #3 | 10.36 | 97 | 13.77 | 15.83 | 63 | 29.8 | 5/54 | 72 | 2625 | 76.2 | 75 | 6.71 | 9.06 | 8.95 | 4.97 | 7.34 | 59.4 k | 31.8 k | 300 | 26.92 | 31.91 | 51 | 14.6 k | 63 | 10.6 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull #3 | 10.57 | 95 | 15.26 | 17.36 | 58 | 28.9 | 3/58 | 72 | 2625 | 74.9 | 75 | 6.59 | 8.72 | 9.14 | 4.98 | 7.39 | 59.4 k | 31.7 k | 300 | 25.11 | 30.41 | 45 | 55.0 k | 71 | 41.5 k | 16 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull #4 | 10.43 | 96 | 15.31 | 17.05 | 59 | 28.3 | 5/56 | 72 | 2625 | 74.9 | 75 | 6.70 | 8.78 | 9.28 | 5.06 | 7.39 | 59.4 k | 31.6 k | 300 | 25.71 | 30.38 | 45 | 55.0 k | 70 | 41.5 k | 17 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=base #4 | 10.40 | 96 | 14.38 | 16.25 | 62 | 28.1 | 6/45 | 72 | 2625 | 76.0 | 76 | 6.51 | 9.04 | 8.88 | 4.93 | 7.32 | 59.4 k | 31.8 k | 300 | 26.92 | 31.54 | 51 | 14.6 k | 65 | 10.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull #4 | 10.15 | 98 | 13.84 | 15.57 | 64 | 30.9 | 5/44 | 74 | 2625 | 77.4 | 76 | 6.51 | 8.93 | 8.66 | 4.81 | 7.29 | 59.4 k | 31.8 k | 300 | 26.09 | 31.73 | 51 | 14.6 k | 63 | 10.5 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate #4 | 10.29 | 97 | 13.66 | 15.41 | 65 | 32.0 | 4/51 | 73 | 2625 | 76.6 | 76 | 6.66 | 8.97 | 8.80 | 4.87 | 7.30 | 59.4 k | 31.8 k | 300 | 26.46 | 31.68 | 51 | 14.5 k | 65 | 10.5 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache #4 | 10.83 | 92 | 17.38 | 19.15 | 52 | 32.1 | 4/46 | 68 | 2624 | 70.9 | 74 | 7.00 | 9.48 | 10.31 | 6.46 | 7.69 | 59.4 k | 31.3 k | 300 | 23.08 | 28.42 | 3 | 11.0 k | 72 | 10.2 k | 21 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=facecull #5 | 10.60 | 94 | 13.91 | 15.80 | 63 | 32.9 | 3/48 | 72 | 2625 | 75.8 | 76 | 6.71 | 9.16 | 9.12 | 5.11 | 7.35 | 59.4 k | 31.8 k | 300 | 26.92 | 31.87 | 51 | 14.6 k | 70 | 10.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=nocull #5 | 10.65 | 94 | 14.79 | 17.69 | 57 | 30.3 | 3/47 | 73 | 2625 | 75.3 | 75 | 6.64 | 8.76 | 9.20 | 5.08 | 7.46 | 59.4 k | 31.8 k | 300 | 25 | 30.21 | 45 | 55.0 k | 72 | 41.5 k | 16 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=lampcache #5 | 10.83 | 92 | 16.71 | 19.20 | 52 | 42.7 | 3/49 | 69 | 2625 | 71.5 | 74 | 6.96 | 9.24 | 10.12 | 6.20 | 7.65 | 59.4 k | 31.3 k | 300 | 24.06 | 29.16 | 4 | 11.0 k | 72 | 10.1 k | 22 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=base #5 | 10.88 | 92 | 14.53 | 16.33 | 61 | 30.5 | 3/45 | 70 | 2625 | 74.1 | 75 | 6.80 | 9.23 | 9.47 | 5.34 | 7.41 | 59.4 k | 31.8 k | 300 | 25.96 | 31.48 | 51 | 14.6 k | 72 | 10.5 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 arm=sungate #5 | 10.41 | 96 | 14.47 | 16.12 | 62 | 33.0 | 4/51 | 72 | 2625 | 76.0 | 76 | 6.64 | 9.00 | 8.97 | 5.04 | 7.35 | 59.4 k | 31.8 k | 300 | 26.44 | 31.72 | 51 | 14.5 k | 69 | 10.1 k | 11 | 0 | 0 | 0 | 0 |
