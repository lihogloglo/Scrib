# Perf suite `pt_nomat`

- exe: `target/profiling/perftests.exe` (release)
- date: 2026-11-17 10:33:53 UTC
- commit: `c4d9ef509bc4dad4ae1eb5bdc12a8e0a892705c2` (**dirty**: 23 tracked files changed)
- exe sha256: `8499f6a0e88aa33660b0e9c8f6572e089222c1e0131fe64811e0c42232516c2f` · modified 2026-11-17 07:43:48 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-17 07:07:55 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality --env BEVYRROWIND_WINDOW=borderless · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 2 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1794910744707131 (`--seed 1794910744707131` replays the order)
  - arm `base` (base): shipped defaults
  - arm `nomat`: --set render.depth_only.skip_material=on
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- valid runs: 8 of 8 (2 warm-up runs discarded)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 8 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk @ 2560x1440 | `base` (base) | 4 | 10.78 | 10.58-10.84 | | | | | | | | | |
| balmora_walk @ 2560x1440 | `nomat` | 4 | 10.62 | 10.48-10.69 | -0.16 | -0.22 | 3.0 | 0.200 | -0.12 | -0.31 | -0.20 | -0.05 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=nomat ×4 | 10.62 | 94 | 13.40 | 16.59 | 60 | 59.0 | 33/59, 34/55, 36/46, 29/49 | 77 | 2625 | 80.1 | 75 | 6.69 | 8.96 | 8.75 | 4.67 | 7.81 | 59.3 k | 31.7 k | 286 | 26.50 | 32.29 | 46 | 20.5 k | 66.50 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base ×4 | 10.78 | 93 | 13.69 | 17.29 | 58 | 47.7 | 34/52, 33/51, 32/50, 33/41 | 77 | 2624 | 79.2 | 75 | 6.62 | 9.08 | 9.05 | 4.87 | 7.86 | 59.3 k | 31.7 k | 286 | 26.25 | 32.00 | 46 | 20.4 k | 69 | 10.8 k | 13 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 arm=nomat #1 | 10.61 | 94 | 13.40 | 17.48 | 57 | 55.6 | 33/59 | 77 | 2625 | 80.1 | 75 | 6.70 | 8.99 | 8.79 | 4.69 | 7.82 | 59.4 k | 31.7 k | 286 | 26.70 | 32.78 | 46 | 20.5 k | 66 | 10.8 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #1 | 10.84 | 92 | 13.69 | 17.36 | 58 | 44.2 | 34/52 | 77 | 2625 | 79.3 | 75 | 6.74 | 9.10 | 9.12 | 5.00 | 7.88 | 59.3 k | 31.7 k | 286 | 26.70 | 32.02 | 46 | 20.4 k | 74 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #2 | 10.73 | 93 | 13.70 | 18.08 | 55 | 47.2 | 33/51 | 76 | 2624 | 79.2 | 75 | 6.56 | 9.06 | 8.98 | 4.76 | 7.79 | 59.4 k | 31.7 k | 286 | 25.84 | 31.99 | 46 | 20.5 k | 66 | 10.8 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=nomat #2 | 10.48 | 95 | 13.20 | 16.40 | 61 | 42.1 | 34/55 | 78 | 2625 | 80.6 | 76 | 6.68 | 8.95 | 8.70 | 4.62 | 7.81 | 59.4 k | 31.9 k | 286 | 26.32 | 31.80 | 46 | 20.5 k | 64 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=nomat #3 | 10.69 | 94 | 13.50 | 16.79 | 60 | 59.0 | 36/46 | 77 | 2624 | 79.5 | 76 | 6.73 | 8.94 | 8.97 | 4.87 | 7.88 | 59.3 k | 31.7 k | 286 | 26.68 | 31.70 | 46 | 20.4 k | 74 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #3 | 10.58 | 95 | 13.30 | 16.51 | 61 | 42.7 | 32/50 | 79 | 2624 | 80.0 | 75 | 6.53 | 8.99 | 8.74 | 4.68 | 7.84 | 59.3 k | 31.7 k | 286 | 26.54 | 32.17 | 46 | 20.5 k | 65 | 10.7 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=base #4 | 10.83 | 92 | 14.05 | 17.22 | 58 | 47.7 | 33/41 | 77 | 2624 | 78.6 | 75 | 6.68 | 9.10 | 9.14 | 4.98 | 7.90 | 59.3 k | 31.7 k | 286 | 25.96 | 31.98 | 46 | 20.4 k | 72 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 arm=nomat #4 | 10.63 | 94 | 13.41 | 16.21 | 62 | 43.4 | 29/49 | 77 | 2625 | 80.1 | 75 | 6.43 | 8.97 | 8.70 | 4.65 | 7.80 | 59.3 k | 31.7 k | 286 | 25.96 | 32.86 | 46 | 20.5 k | 67 | 10.7 k | 14 | 0 | 0 | 0 | 0 |
