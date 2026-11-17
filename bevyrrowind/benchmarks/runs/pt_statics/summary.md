# Perf suite `pt_statics`

- exe: `target/profiling/perftests.exe` (release)
- date: 2026-11-17 08:23:30 UTC
- commit: `c4d9ef509bc4dad4ae1eb5bdc12a8e0a892705c2` (**dirty**: 18 tracked files changed)
- exe sha256: `8499f6a0e88aa33660b0e9c8f6572e089222c1e0131fe64811e0c42232516c2f` · modified 2026-11-17 07:43:48 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-17 07:07:55 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality --env BEVYRROWIND_WINDOW=borderless · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 4 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1794901450081791 (`--seed 1794901450081791` replays the order)
  - arm `base` (base): shipped defaults
  - arm `scene`: --arg=--scene
  - arm `nocull`: --env BEVYRROWIND_GPU_CULL_STATICS=1
  - arm `noprecomb`: --off=precombine
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- valid runs: 32 of 32 (4 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 32 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk @ 2560x1440 | `base` (base) | 4 | 10.62 | 10.52-10.78 | | | | | | | | | |
| balmora_walk @ 2560x1440 | `scene` | 4 | 10.57 | 10.34-10.65 | -0.05 | -0.02 | 5.0 | 0.486 | -0.32 | +0.02 | +0.07 | +0.16 | inconclusive |
| balmora_walk @ 2560x1440 | `nocull` | 4 | 10.79 | 10.65-10.85 | +0.18 | +0.21 | 14.0 | 0.114 | -0.38 | +0.30 | +0.06 | +0.09 | inconclusive |
| balmora_walk @ 2560x1440 | `noprecomb` | 4 | 11.65 | 11.61-11.76 | +1.03 | +1.00 | 16.0 | 0.029 | +1.36 | +0.33 | +0.11 | -0.04 | **slower** |
| balmora @ 2560x1440 | `base` (base) | 4 | 9.71 | 9.68-9.76 | | | | | | | | | |
| balmora @ 2560x1440 | `scene` | 4 | 9.49 | 9.47-9.57 | -0.22 | -0.21 | 0.0 | 0.029 (approx.) | +0.00 | +0.25 | +0.05 | -0.27 | **faster** |
| balmora @ 2560x1440 | `nocull` | 4 | 9.92 | 9.90-10.07 | +0.20 | +0.20 | 16.0 | 0.029 (approx.) | -0.10 | +0.19 | +0.06 | +0.03 | **slower** |
| balmora @ 2560x1440 | `noprecomb` | 4 | 9.60 | 9.47-9.66 | -0.11 | -0.12 | 0.0 | 0.029 (approx.) | +0.50 | -0.05 | -0.10 | -0.09 | **faster** |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=nocull ×4 | 10.79 | 93 | 13.80 | 17.86 | 56 | 48.3 | 31/60, 31/57, 31/51, 31/52 | 78 | 2621 | 79.7 | 78 | 6.81 | 8.69 | 9.17 | 4.79 | 7.89 | 59.3 k | 31.7 k | 286 | 25.12 | 30.31 | 46 | 69.0 k | 73 | 41.4 k | 19 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=scene ×4 | 10.57 | 95 | 13.19 | 16.49 | 61 | 62.8 | 6/40, 6/43, 4/39, 2/36 | 80 | 2619 | 81.7 | 78 | 6.53 | 8.75 | 8.88 | 4.81 | 7.97 | 48.7 k | 20.6 k | 286 | 26.15 | 31.17 | 46 | 11.7 k | 78.50 | 154.0 k | 17.50 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 arm=noprecomb ×4 | 11.65 | 86 | 15.03 | 19.34 | 52 | 70.8 | 31/32, 29/35, 31/35, 30/32 | 70 | 2622 | 75.0 | 77 | 6.93 | 10.43 | 9.20 | 4.85 | 7.77 | 83.3 k | 55.0 k | 286 | 27.11 | 32.95 | 46 | 28.9 k | 68 | 14.8 k | 13.50 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base ×4 | 10.62 | 94 | 13.46 | 16.79 | 60 | 51.7 | 32/54, 34/48, 35/45, 31/53 | 77 | 2620 | 80.5 | 78 | 6.52 | 9.07 | 8.87 | 4.74 | 7.81 | 59.3 k | 31.7 k | 286 | 26.45 | 32.23 | 46 | 20.5 k | 64 | 10.7 k | 12.50 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=nocull ×4 | 9.92 | 101 | 11.30 | 11.85 | 84 | 28.5 | 0/65, 1/56, 1/62, 1/57 | 98 | 2621 | 97.5 | 79 | 8.58 | 7.08 | 7.17 | 4.16 | 9.26 | 47.1 k | 24.7 k | 210 | 26.99 | 32.44 | 4 | 56.2 k | 58 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=scene ×4 | 9.49 | 105 | 10.90 | 11.63 | 86 | 18.6 | 0/45, 0/41, 0/40, 0/37 | 97 | 2624 | 97.8 | 79 | 8.24 | 7.18 | 7.24 | 4.15 | 8.97 | 37.4 k | 15.1 k | 210 | 27.16 | 32.39 | 4 | 9462 | 64 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 arm=noprecomb ×4 | 9.60 | 104 | 10.95 | 11.66 | 86 | 42.1 | 1/36, 1/37, 1/41, 1/40 | 98 | 2619 | 99.1 | 80 | 8.39 | 7.68 | 6.93 | 4.00 | 9.15 | 68.4 k | 46.1 k | 210 | 27.69 | 33.17 | 4 | 25.1 k | 52 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=base ×4 | 9.71 | 103 | 11.10 | 11.59 | 86 | 43.3 | 2/58, 1/60, 1/54, 1/59 | 98 | 2625 | 98.6 | 79 | 8.55 | 7.17 | 6.99 | 4.10 | 9.24 | 47.1 k | 24.7 k | 210 | 26.20 | 29.94 | 4 | 15.3 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=nocull #1 | 10.65 | 94 | 13.59 | 17.74 | 56 | 40.1 | 31/60 | 78 | 2622 | 79.4 | 76 | 6.80 | 8.62 | 9.04 | 4.71 | 7.89 | 59.3 k | 31.7 k | 286 | 25 | 30.17 | 46 | 69.0 k | 72 | 41.4 k | 19 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=scene #1 | 10.34 | 97 | 12.92 | 16.20 | 62 | 35.4 | 6/40 | 82 | 2619 | 82.7 | 78 | 6.41 | 8.55 | 8.60 | 4.67 | 7.97 | 48.7 k | 20.6 k | 286 | 26.44 | 31.18 | 46 | 11.7 k | 78 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 arm=noprecomb #1 | 11.76 | 85 | 15.29 | 19.28 | 52 | 62.7 | 31/32 | 69 | 2622 | 74.3 | 76 | 7.07 | 10.58 | 9.53 | 5.03 | 7.77 | 83.3 k | 55.0 k | 286 | 27.40 | 32.94 | 46 | 28.9 k | 68 | 14.7 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #1 | 10.78 | 93 | 13.52 | 16.82 | 59 | 44.2 | 32/54 | 77 | 2620 | 80.0 | 78 | 6.48 | 9.16 | 8.97 | 4.76 | 7.80 | 59.4 k | 31.7 k | 286 | 26.44 | 31.85 | 46 | 20.4 k | 64 | 10.7 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=scene #2 | 10.51 | 95 | 13.17 | 16.25 | 62 | 62.8 | 6/43 | 80 | 2619 | 81.9 | 78 | 6.45 | 8.73 | 8.80 | 4.76 | 7.96 | 48.7 k | 20.6 k | 286 | 25.59 | 31.10 | 46 | 11.8 k | 78 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 arm=base #2 | 10.52 | 95 | 13.35 | 16.18 | 62 | 48.5 | 34/48 | 78 | 2620 | 81.0 | 78 | 6.48 | 9.07 | 8.63 | 4.59 | 7.79 | 59.4 k | 31.7 k | 286 | 26.91 | 32.13 | 46 | 20.4 k | 62 | 10.7 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=nocull #2 | 10.78 | 93 | 13.75 | 17.69 | 57 | 48.3 | 31/57 | 78 | 2619 | 79.9 | 78 | 6.82 | 8.68 | 9.13 | 4.78 | 7.90 | 59.3 k | 31.7 k | 286 | 25.24 | 30.19 | 46 | 69.0 k | 74 | 41.4 k | 18 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=noprecomb #2 | 11.67 | 86 | 14.94 | 19.39 | 52 | 65.2 | 29/35 | 71 | 2620 | 75.3 | 77 | 6.54 | 10.36 | 9.11 | 4.80 | 7.77 | 83.3 k | 55.0 k | 286 | 26.92 | 32.88 | 46 | 28.9 k | 68 | 14.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=noprecomb #3 | 11.61 | 86 | 15.00 | 19.52 | 51 | 65.6 | 31/35 | 70 | 2622 | 75.6 | 76 | 6.87 | 10.41 | 9.13 | 4.82 | 7.78 | 83.3 k | 55.0 k | 286 | 26.92 | 32.96 | 46 | 28.9 k | 68 | 14.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=nocull #3 | 10.85 | 92 | 14.13 | 17.97 | 56 | 46.5 | 31/51 | 77 | 2620 | 78.9 | 77 | 6.89 | 8.70 | 9.29 | 4.90 | 7.92 | 59.3 k | 31.7 k | 286 | 25.48 | 30.43 | 46 | 69.0 k | 74 | 41.4 k | 20 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #3 | 10.59 | 94 | 13.40 | 16.92 | 59 | 51.7 | 35/45 | 78 | 2620 | 81.0 | 78 | 6.56 | 9.05 | 8.79 | 4.71 | 7.82 | 59.3 k | 31.7 k | 286 | 26.44 | 32.43 | 46 | 20.5 k | 64 | 10.7 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=scene #3 | 10.65 | 94 | 13.41 | 16.73 | 60 | 32.2 | 4/39 | 80 | 2625 | 81.5 | 78 | 6.61 | 8.78 | 9.04 | 4.93 | 7.97 | 48.5 k | 20.5 k | 286 | 25.96 | 31.17 | 46 | 11.7 k | 81 | 154.0 k | 18 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 arm=base #4 | 10.64 | 94 | 13.62 | 16.76 | 60 | 40.7 | 31/53 | 77 | 2623 | 80.1 | 78 | 6.68 | 9.07 | 8.94 | 4.81 | 7.84 | 59.3 k | 31.7 k | 286 | 26.46 | 32.33 | 46 | 20.5 k | 70 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=noprecomb #4 | 11.62 | 86 | 15.05 | 19.02 | 53 | 70.8 | 30/32 | 69 | 2625 | 74.6 | 77 | 6.98 | 10.45 | 9.27 | 4.87 | 7.76 | 83.3 k | 55.0 k | 286 | 27.29 | 33.29 | 46 | 29.1 k | 67 | 14.9 k | 15 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=scene #4 | 10.63 | 94 | 13.22 | 17.18 | 58 | 34.2 | 2/36 | 81 | 2619 | 81.4 | 79 | 6.64 | 8.76 | 8.97 | 4.85 | 7.98 | 48.7 k | 20.6 k | 286 | 26.34 | 31.56 | 46 | 11.8 k | 79 | 154.0 k | 18 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 arm=nocull #4 | 10.81 | 93 | 13.86 | 18.08 | 55 | 43.1 | 31/52 | 78 | 2622 | 80.7 | 78 | 6.75 | 8.72 | 9.20 | 4.81 | 7.88 | 59.3 k | 31.7 k | 286 | 24.76 | 30.75 | 46 | 69.0 k | 71 | 41.4 k | 19 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=nocull #1 | 10.07 | 99 | 11.45 | 11.95 | 84 | 25.1 | 0/65 | 97 | 2613 | 97.3 | 80 | 8.56 | 7.08 | 7.13 | 4.05 | 9.26 | 47.1 k | 24.7 k | 210 | 29.81 | 34.23 | 4 | 56.2 k | 58 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=scene #1 | 9.47 | 106 | 10.88 | 11.95 | 84 | 18.6 | 0/45 | 97 | 2625 | 97.8 | 80 | 8.21 | 7.34 | 7.56 | 4.39 | 8.99 | 37.4 k | 15.1 k | 210 | 32.64 | 39.11 | 4 | 9462 | 64 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 arm=noprecomb #1 | 9.61 | 104 | 10.99 | 11.84 | 84 | 42.1 | 1/36 | 97 | 2621 | 99.2 | 80 | 8.40 | 7.90 | 7.04 | 4.05 | 9.17 | 68.4 k | 46.1 k | 210 | 33.65 | 36.12 | 4 | 25.1 k | 52 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=base #1 | 9.68 | 103 | 11.10 | 11.83 | 85 | 43.3 | 2/58 | 97 | 2625 | 98.7 | 80 | 8.51 | 7.41 | 7.20 | 4.22 | 9.22 | 47.1 k | 24.7 k | 210 | 33.33 | 38.99 | 4 | 15.3 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=scene #2 | 9.57 | 104 | 10.93 | 11.36 | 88 | 17.3 | 0/41 | 98 | 2616 | 97.8 | 80 | 8.31 | 7.07 | 7.10 | 4.08 | 8.98 | 37.4 k | 15.1 k | 210 | 26.44 | 32.51 | 4 | 9462 | 64 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 arm=base #2 | 9.76 | 102 | 11.11 | 11.65 | 86 | 27.0 | 1/60 | 97 | 2614 | 98.4 | 80 | 8.61 | 7.20 | 7.05 | 4.15 | 9.27 | 47.1 k | 24.7 k | 210 | 26.44 | 30.55 | 4 | 15.3 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=nocull #2 | 9.90 | 101 | 11.32 | 11.91 | 84 | 28.1 | 1/56 | 97 | 2625 | 97.7 | 79 | 8.59 | 7.10 | 7.31 | 4.25 | 9.26 | 47.1 k | 24.7 k | 210 | 27.88 | 33.16 | 4 | 56.2 k | 58 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=noprecomb #2 | 9.59 | 104 | 11.00 | 11.97 | 84 | 41.4 | 1/37 | 98 | 2616 | 99.1 | 80 | 8.38 | 7.66 | 6.83 | 3.96 | 9.13 | 68.4 k | 46.1 k | 210 | 27.01 | 30.80 | 4 | 25.1 k | 52 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=noprecomb #3 | 9.47 | 106 | 10.90 | 11.48 | 87 | 42.0 | 1/41 | 98 | 2625 | 99.1 | 80 | 8.34 | 7.69 | 7.15 | 4.27 | 9.07 | 68.4 k | 46.1 k | 210 | 28.37 | 33.85 | 4 | 25.1 k | 52 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=nocull #3 | 9.91 | 101 | 11.21 | 11.79 | 85 | 28.5 | 1/62 | 98 | 2625 | 98.1 | 79 | 8.57 | 7.07 | 7.21 | 4.18 | 9.24 | 47.1 k | 24.7 k | 210 | 25.79 | 31.16 | 4 | 56.2 k | 58 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=base #3 | 9.74 | 103 | 11.11 | 11.54 | 87 | 26.4 | 1/54 | 98 | 2625 | 98.5 | 79 | 8.59 | 7.03 | 6.82 | 3.98 | 9.25 | 47.1 k | 24.7 k | 210 | 25.96 | 29.17 | 4 | 15.3 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=scene #3 | 9.50 | 105 | 10.94 | 11.80 | 85 | 17.0 | 0/40 | 98 | 2622 | 97.7 | 79 | 8.09 | 7.15 | 7.23 | 4.12 | 8.92 | 37.4 k | 15.1 k | 210 | 25.96 | 31.88 | 4 | 9462 | 64 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 arm=base #4 | 9.68 | 103 | 11.08 | 11.53 | 87 | 26.9 | 1/59 | 98 | 2625 | 99.0 | 79 | 8.48 | 7.14 | 6.93 | 4.05 | 9.18 | 47.1 k | 24.7 k | 210 | 25.48 | 29.32 | 4 | 15.3 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=noprecomb #4 | 9.66 | 103 | 10.91 | 11.41 | 88 | 40.8 | 1/40 | 98 | 2616 | 98.0 | 79 | 8.57 | 7.64 | 6.78 | 3.91 | 9.22 | 68.4 k | 46.1 k | 210 | 26.92 | 32.48 | 4 | 25.1 k | 52 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 arm=scene #4 | 9.49 | 105 | 10.84 | 11.46 | 87 | 17.6 | 0/37 | 97 | 2625 | 97.5 | 79 | 8.27 | 7.20 | 7.25 | 4.19 | 8.95 | 37.4 k | 15.1 k | 210 | 27.88 | 32.28 | 4 | 9462 | 64 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 arm=nocull #4 | 9.92 | 101 | 11.28 | 11.70 | 85 | 26.9 | 1/57 | 98 | 2617 | 96.8 | 79 | 8.62 | 7.01 | 7.13 | 4.13 | 9.28 | 47.1 k | 24.7 k | 210 | 26.09 | 31.73 | 4 | 56.2 k | 58 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
