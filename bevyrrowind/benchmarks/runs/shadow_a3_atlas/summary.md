# Perf suite `shadow_a3_atlas`

- exe: `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` (release)
- date: 2026-11-19 18:14:31 UTC
- commit: `e3a49407a28239cad1af880060d10f039871eb41` (**dirty**: 57 tracked files changed)
- exe sha256: `998067327a1c8c10ff071e07dde6fc94d7bf6acd1444c79ef8f72b2a7a6e35e0` · modified 2026-11-19 17:48:46 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-19 14:09:12 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795110556297966 (`--seed 1795110556297966` replays the order)
  - arm `base` (base): shipped defaults
  - arm `atlas8`: --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8
  - arm `atlas48`: --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48
- scenarios: `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street; `census` interior: Seyda Neen Census and Excise Office walk-through
- valid runs: 18 of 18 (3 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 18 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk_night @ 2560x1440 | `base` (base) | 3 | 10.63 | 10.61-10.72 | | | | | | | | | |
| balmora_walk_night @ 2560x1440 | `atlas8` | 3 | 9.79 | 9.76-9.82 | -0.84 | -0.85 | 0.0 | 0.100 | -0.58 | -0.98 | -0.78 | +0.05 | **faster** |
| balmora_walk_night @ 2560x1440 | `atlas48` | 3 | 9.87 | 9.73-10.01 | -0.76 | -0.85 | 0.0 | 0.100 | -0.55 | -0.90 | -0.74 | +0.05 | **faster** |
| census @ 2560x1440 | `base` (base) | 3 | 6.85 | 6.82-6.90 | | | | | | | | | |
| census @ 2560x1440 | `atlas8` | 3 | 6.32 | 6.32-6.35 | -0.53 | -0.50 | 0.0 | 0.100 | -0.33 | -0.84 | -0.52 | +0.04 | **faster** |
| census @ 2560x1440 | `atlas48` | 3 | 6.52 | 6.52-6.55 | -0.32 | -0.30 | 0.0 | 0.100 | +0.02 | -0.83 | -0.52 | +0.03 | **faster** |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=atlas8 ×3 | 9.79 | 102 | 11.60 | 13.40 | 75 | 37.2 | 2/48, 4/45, 4/45 | 87 | 2625 | 83.5 | 75 | 6.40 | 8.09 | 8.16 | 4.36 | 7.95 | 49.4 k | 21.3 k | 299 | 25.49 | 31.74 | 17 | 7993 | 85 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base ×3 | 10.63 | 94 | 13.25 | 15.66 | 64 | 41.9 | 2/46, 2/47, 5/47 | 81 | 2625 | 77.3 | 74 | 6.83 | 8.66 | 9.13 | 5.14 | 7.90 | 49.4 k | 21.1 k | 299 | 25.96 | 32.65 | 51 | 8444 | 82 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas48 ×3 | 9.87 | 101 | 11.61 | 13.75 | 73 | 34.0 | 3/51, 3/44, 4/43 | 87 | 2625 | 83.2 | 74 | 6.48 | 8.12 | 8.23 | 4.40 | 7.94 | 49.4 k | 21.3 k | 299 | 25.29 | 31.62 | 20 | 7998 | 85 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| census 2560x1440 arm=atlas8 ×3 | 6.32 | 158 | 7.65 | 8.46 | 118 | 19.7 | 0/30, 0/36, 0/32 | 96 | 2625 | 100.9 | 75 | 5.38 | 5.06 | 4.77 | 2.72 | 5.87 | 13.1 k | 6100 | 156 | 29.81 | 36.84 | 9 | 598 | 26 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base ×3 | 6.85 | 146 | 10.11 | 11.79 | 85 | 19.8 | 0/35, 0/48, 0/29 | 87 | 2623 | 90.1 | 72 | 5.13 | 5.39 | 5.61 | 3.24 | 5.84 | 13.1 k | 6100 | 156 | 27.43 | 33.21 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas48 ×3 | 6.52 | 153 | 7.73 | 8.30 | 121 | 80.0 | 1/36, 1/36, 0/34 | 97 | 2625 | 98.5 | 74 | 5.28 | 5.41 | 4.78 | 2.72 | 5.86 | 13.1 k | 6100 | 156 | 28.93 | 37.73 | 10 | 552 | 23 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=atlas8 #1 | 9.76 | 102 | 11.60 | 13.40 | 75 | 34.3 | 2/48 | 87 | 2625 | 83.4 | 75 | 6.41 | 8.09 | 8.16 | 4.35 | 7.93 | 49.4 k | 21.3 k | 299 | 25.49 | 31.66 | 17 | 7996 | 85 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #1 | 10.61 | 94 | 12.91 | 15.05 | 66 | 31.4 | 2/46 | 82 | 2625 | 78.6 | 74 | 6.74 | 8.66 | 8.94 | 5.00 | 7.90 | 49.4 k | 21.1 k | 299 | 26.01 | 33.19 | 51 | 8379 | 82 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas48 #1 | 10.01 | 100 | 11.84 | 14.01 | 71 | 34.0 | 3/51 | 85 | 2625 | 81.8 | 73 | 6.48 | 8.23 | 8.39 | 4.50 | 7.94 | 49.4 k | 21.3 k | 299 | 25.29 | 31.70 | 20 | 8006 | 85 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #2 | 10.63 | 94 | 13.36 | 15.66 | 64 | 36.4 | 2/47 | 81 | 2625 | 77.2 | 73 | 6.83 | 8.64 | 9.13 | 5.14 | 7.94 | 49.4 k | 21.1 k | 299 | 25.96 | 32.65 | 51 | 8514 | 83 | 154.1 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas48 #2 | 9.73 | 103 | 11.57 | 13.38 | 75 | 30.1 | 3/44 | 87 | 2625 | 83.7 | 75 | 6.29 | 8.06 | 8.09 | 4.30 | 7.86 | 49.4 k | 21.3 k | 299 | 25.59 | 31.62 | 20 | 7970 | 81 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas8 #2 | 9.79 | 102 | 11.62 | 13.39 | 75 | 32.9 | 4/45 | 87 | 2625 | 83.5 | 76 | 6.31 | 8.12 | 8.15 | 4.36 | 7.95 | 49.4 k | 21.3 k | 299 | 25.79 | 31.77 | 17 | 7993 | 83 | 154.0 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas48 #3 | 9.87 | 101 | 11.61 | 13.75 | 73 | 32.6 | 4/43 | 87 | 2625 | 83.2 | 74 | 6.48 | 8.12 | 8.23 | 4.40 | 7.98 | 49.4 k | 21.3 k | 300 | 25.06 | 31.50 | 19 | 7998 | 85 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=atlas8 #3 | 9.82 | 102 | 11.51 | 13.52 | 74 | 37.2 | 4/45 | 87 | 2625 | 84.1 | 75 | 6.40 | 8.08 | 8.17 | 4.38 | 8.01 | 49.4 k | 21.3 k | 299 | 24.66 | 31.74 | 17 | 7988 | 85 | 154.1 k | 14 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #3 | 10.72 | 93 | 13.25 | 15.79 | 63 | 41.9 | 5/47 | 80 | 2625 | 77.3 | 74 | 6.92 | 8.93 | 9.20 | 5.25 | 7.89 | 49.2 k | 21.1 k | 299 | 25.75 | 32.17 | 51 | 8444 | 82 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| census 2560x1440 arm=atlas8 #1 | 6.32 | 158 | 7.81 | 9.33 | 107 | 13.9 | 0/30 | 95 | 2624 | 99.1 | 75 | 5.38 | 5.12 | 4.77 | 2.72 | 5.88 | 13.1 k | 6100 | 156 | 29.81 | 36.84 | 9 | 598 | 26 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #1 | 6.82 | 147 | 9.89 | 12.17 | 82 | 19.8 | 0/35 | 88 | 2620 | 88.1 | 70 | 5.13 | 5.35 | 5.54 | 3.22 | 5.86 | 13.1 k | 6100 | 156 | 25.48 | 32.37 | 49 | 4168 | 26.50 | 98.9 k | 8 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas48 #1 | 6.52 | 153 | 7.73 | 8.23 | 121 | 25.7 | 1/36 | 97 | 2626 | 98.5 | 73 | 5.28 | 5.41 | 4.78 | 2.72 | 5.85 | 13.1 k | 6100 | 156 | 30.66 | 38.23 | 11 | 552 | 23 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #2 | 6.90 | 145 | 10.47 | 11.79 | 85 | 13.7 | 0/48 | 86 | 2623 | 90.3 | 72 | 5.30 | 5.56 | 5.82 | 3.35 | 5.84 | 13.1 k | 6100 | 156 | 30.14 | 38.40 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas48 #2 | 6.52 | 153 | 7.77 | 8.67 | 115 | 80.0 | 1/36 | 96 | 2625 | 98.1 | 74 | 5.22 | 5.41 | 4.85 | 2.75 | 5.86 | 13.1 k | 6100 | 156 | 26.92 | 34.82 | 10 | 548 | 23 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas8 #2 | 6.32 | 158 | 7.64 | 8.24 | 121 | 19.7 | 0/36 | 97 | 2625 | 100.9 | 75 | 5.40 | 5.06 | 4.74 | 2.72 | 5.87 | 13.1 k | 6100 | 156 | 30.77 | 37.67 | 9 | 588 | 23 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas48 #3 | 6.55 | 153 | 7.70 | 8.30 | 121 | 13.7 | 0/34 | 97 | 2625 | 100.9 | 75 | 5.57 | 5.41 | 4.77 | 2.70 | 5.98 | 13.1 k | 6100 | 156 | 28.93 | 37.73 | 10 | 552 | 23 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=atlas8 #3 | 6.35 | 158 | 7.65 | 8.46 | 118 | 13.2 | 0/32 | 96 | 2625 | 101.3 | 73 | 5.32 | 5.04 | 4.80 | 2.74 | 5.85 | 13.1 k | 6100 | 156 | 25.53 | 33.16 | 9 | 605 | 29 | 98.9 k | 4 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #3 | 6.85 | 146 | 10.11 | 10.89 | 92 | 13.1 | 0/29 | 87 | 2625 | 90.1 | 72 | 5.13 | 5.39 | 5.61 | 3.24 | 5.84 | 13.1 k | 6100 | 156 | 27.43 | 33.21 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
