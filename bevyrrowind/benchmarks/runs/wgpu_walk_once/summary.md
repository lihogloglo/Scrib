# Perf suite `wgpu_walk_once`

- exe: `../target/profiling/walkonce.exe` (release)
- date: 2026-11-17 06:20:00 UTC
- commit: `3cd061d32b93df0150818cfbff06aada715c8c14` (**dirty**: 28 tracked files changed)
- exe sha256: `15fad9aed713e2ff8e7d1625ace03817143f83de3c6ba1e974a9659533912156` · modified 2026-11-17 05:55:54 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-17 05:41:17 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1794894963380682 (`--seed 1794894963380682` replays the order)
  - arm `base` (base): shipped defaults
  - arm `once`: --set render.wgpu.bind_group_walk_once=on
  - arm `onetex`: --off=texvariety
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- valid runs: 9 of 9 (3 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 9 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk @ 2560x1440 | `base` (base) | 3 | 12.09 | 12.05-12.15 | | | | | | | | | |
| balmora_walk @ 2560x1440 | `once` | 3 | 10.94 | 10.71-11.26 | -1.15 | -1.11 | 0.0 | 0.100 | -0.40 | -1.37 | -1.37 | -0.01 | **faster** |
| balmora_walk @ 2560x1440 | `onetex` | 3 | 10.78 | 10.26-10.97 | -1.30 | -1.37 | 0.0 | 0.100 | -0.49 | -1.55 | -1.66 | -0.25 | **faster** |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=onetex ×3 | 10.78 | 93 | 13.86 | 19.46 | 51 | 58.3 | 34/129, 51/173, 33/157 | 75 | 2623 | 78.1 | 76 | 6.50 | 9.22 | 9.05 | 4.59 | 7.58 | 59.3 k | 31.7 k | 286 | 31.88 | 38.52 | 46 | 20.4 k | 63 | 10.6 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=once ×3 | 10.94 | 91 | 13.88 | 17.82 | 56 | 49.5 | 36/153, 37/143, 36/144 | 75 | 2620 | 78.4 | 75 | 6.78 | 9.31 | 9.23 | 4.88 | 7.83 | 59.3 k | 31.7 k | 286 | 32.58 | 38.64 | 46 | 20.4 k | 63 | 10.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base ×3 | 12.09 | 83 | 15.55 | 19.50 | 51 | 45.5 | 28/138, 30/132, 29/139 | 69 | 2623 | 73.1 | 75 | 7.01 | 9.71 | 10.60 | 6.25 | 7.83 | 59.3 k | 31.7 k | 286 | 30.82 | 37.21 | 46 | 20.4 k | 67 | 10.8 k | 15 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=onetex #1 | 10.97 | 91 | 13.88 | 19.46 | 51 | 52.2 | 34/129 | 74 | 2623 | 77.2 | 76 | 6.53 | 9.44 | 9.19 | 4.60 | 7.58 | 59.4 k | 31.7 k | 286 | 32.21 | 40.02 | 46 | 20.4 k | 63 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=once #1 | 11.26 | 89 | 14.90 | 19.14 | 52 | 44.8 | 36/153 | 72 | 2618 | 75.8 | 75 | 6.94 | 9.62 | 9.68 | 5.07 | 7.83 | 59.3 k | 31.7 k | 286 | 32.21 | 38.88 | 46 | 20.4 k | 64 | 10.6 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #1 | 12.09 | 83 | 15.53 | 18.83 | 53 | 45.5 | 28/138 | 69 | 2625 | 73.1 | 75 | 7.01 | 9.71 | 10.60 | 6.20 | 7.85 | 59.3 k | 31.7 k | 286 | 30.77 | 37.68 | 46 | 20.4 k | 68 | 10.6 k | 15 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=once #2 | 10.94 | 91 | 13.88 | 17.65 | 57 | 49.5 | 37/143 | 75 | 2625 | 78.4 | 75 | 6.78 | 9.31 | 9.23 | 4.88 | 7.80 | 59.3 k | 31.7 k | 286 | 32.58 | 38.50 | 46 | 20.3 k | 63 | 10.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #2 | 12.05 | 83 | 15.55 | 19.50 | 51 | 44.2 | 30/132 | 69 | 2623 | 73.3 | 75 | 6.96 | 9.58 | 10.55 | 6.25 | 7.83 | 59.3 k | 31.7 k | 286 | 31.25 | 37.20 | 46 | 20.5 k | 67 | 10.8 k | 15 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=onetex #2 | 10.26 | 97 | 13.86 | 24.05 | 42 | 58.3 | 51/173 | 77 | 2615 | 80.0 | 77 | 6.25 | 9.02 | 8.55 | 4.25 | 7.57 | 59.3 k | 31.7 k | 286 | 31.88 | 38.44 | 46 | 20.3 k | 63 | 10.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #3 | 12.15 | 82 | 15.89 | 20.50 | 49 | 43.5 | 29/139 | 66 | 2623 | 72.2 | 74 | 7.19 | 9.72 | 10.85 | 6.40 | 7.81 | 59.3 k | 31.7 k | 286 | 30.82 | 37.21 | 46 | 20.4 k | 65 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=onetex #3 | 10.78 | 93 | 13.82 | 18.25 | 55 | 46.0 | 33/157 | 75 | 2625 | 78.1 | 76 | 6.50 | 9.22 | 9.05 | 4.59 | 7.64 | 59.3 k | 31.7 k | 286 | 31.80 | 38.52 | 46 | 20.4 k | 68 | 10.6 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=once #3 | 10.71 | 93 | 13.82 | 17.82 | 56 | 42.0 | 36/144 | 76 | 2620 | 79.7 | 76 | 6.64 | 9.15 | 8.99 | 4.73 | 7.85 | 59.3 k | 31.7 k | 286 | 32.64 | 38.64 | 46 | 20.4 k | 63 | 10.8 k | 12 | 0 | 0 | 0 | 0 |
