# Perf suite `shadow_a1_batched`

- exe: `..\target\profiling\bench-shadow-a1.exe` (release)
- date: 2026-11-19 14:05:53 UTC
- commit: `e3a49407a28239cad1af880060d10f039871eb41` (**dirty**: 41 tracked files changed)
- exe sha256: `9ee9efe39899326a253d4761182608d69fc81b7084f9b37909b0fb674cdaa197` · modified 2026-11-19 13:30:18 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-19 13:14:11 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795095314006712 (`--seed 1795095314006712` replays the order)
  - arm `base` (base): shipped defaults
  - arm `cache`: --set shadow.lamp.cache.enabled=on
  - arm `batched`: --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on
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
| balmora_walk_night @ 2560x1440 | `base` (base) | 3 | 10.53 | 10.22-10.95 | | | | | | | | | |
| balmora_walk_night @ 2560x1440 | `cache` | 3 | 11.21 | 10.87-11.41 | +0.67 | +0.65 | 8.0 | 0.200 | -0.46 | +0.36 | +0.52 | +0.47 | inconclusive |
| balmora_walk_night @ 2560x1440 | `batched` | 3 | 9.83 | 9.64-10.31 | -0.70 | -0.58 | 1.0 | 0.200 | -0.81 | -0.80 | -0.52 | -0.20 | inconclusive |
| census @ 2560x1440 | `base` (base) | 3 | 6.98 | 6.95-7.22 | | | | | | | | | |
| census @ 2560x1440 | `cache` | 3 | 6.85 | 6.81-7.05 | -0.12 | -0.09 | 2.0 | 0.400 | -0.45 | -0.27 | +0.04 | +0.21 | inconclusive |
| census @ 2560x1440 | `batched` | 3 | 6.61 | 6.61-6.71 | -0.37 | -0.37 | 0.0 | 0.100 | -0.26 | -0.52 | -0.36 | +0.16 | **faster** |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=cache ×3 | 11.21 | 89 | 14.64 | 16.68 | 60 | 33.2 | 2/40, 1/39, 2/38 | 79 | 2625 | 75.4 | 73 | 6.93 | 8.42 | 9.39 | 5.62 | 8.22 | 49.1 k | 20.9 k | 299 | 24.35 | 31.49 | 13 | 8006 | 85 | 154.0 k | 25 | 120 | 4 | 22.3 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base ×3 | 10.53 | 95 | 13.63 | 15.35 | 65 | 35.4 | 2/44, 6/45, 2/41 | 78 | 2625 | 76.6 | 74 | 6.98 | 8.89 | 9.03 | 5.10 | 7.75 | 49.2 k | 21.1 k | 299 | 25.96 | 33.14 | 51 | 8459 | 83 | 154.1 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=batched ×3 | 9.83 | 102 | 11.64 | 13.47 | 74 | 33.0 | 3/44, 2/51, 4/50 | 82 | 2625 | 81.1 | 75 | 6.67 | 8.08 | 8.23 | 4.58 | 7.55 | 49.4 k | 21.1 k | 299 | 25 | 29.95 | 18 | 8018 | 85 | 154.1 k | 27 | 117 | 4 | 21.7 k | 3.15 M |
| census 2560x1440 arm=cache ×3 | 6.85 | 146 | 8.97 | 9.82 | 102 | 24.9 | 0/29, 0/33, 0/34 | 92 | 2625 | 93.0 | 71 | 5.53 | 5.06 | 5.66 | 3.41 | 6.04 | 13.1 k | 6100 | 156 | 23.92 | 30.44 | 7 | 620 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base ×3 | 6.98 | 143 | 9.98 | 10.91 | 92 | 57.6 | 0/41, 2/33, 0/35 | 86 | 2625 | 89.7 | 72 | 5.32 | 5.51 | 5.93 | 3.37 | 5.83 | 13.1 k | 6100 | 156 | 25.30 | 32.75 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=batched ×3 | 6.61 | 151 | 8.38 | 10.90 | 92 | 17.1 | 0/29, 0/26, 0/31 | 94 | 2625 | 97.2 | 73 | 5.34 | 5.25 | 5.41 | 3.02 | 5.99 | 13.1 k | 6100 | 156 | 26.02 | 35.70 | 8 | 675 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=cache #1 | 10.87 | 92 | 14.24 | 15.95 | 63 | 33.1 | 2/40 | 75 | 2625 | 74.8 | 73 | 6.80 | 8.33 | 9.35 | 5.62 | 7.80 | 49.1 k | 20.9 k | 299 | 24.35 | 29.13 | 13 | 8038 | 85 | 154.0 k | 25 | 120 | 4 | 22.3 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #1 | 10.22 | 98 | 12.83 | 14.72 | 68 | 29.1 | 2/44 | 78 | 2625 | 77.6 | 74 | 6.73 | 8.44 | 8.67 | 4.95 | 7.49 | 49.2 k | 21.1 k | 299 | 25.96 | 30.98 | 51 | 8459 | 83 | 154.1 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=batched #1 | 9.64 | 104 | 11.64 | 13.20 | 76 | 31.5 | 3/44 | 82 | 2624 | 81.2 | 75 | 6.39 | 7.90 | 8.06 | 4.50 | 7.52 | 49.4 k | 21.1 k | 299 | 25 | 29.70 | 18 | 8030 | 85 | 154.1 k | 28 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=batched #2 | 9.83 | 102 | 11.46 | 13.47 | 74 | 32.4 | 2/51 | 82 | 2625 | 81.1 | 75 | 6.67 | 8.08 | 8.23 | 4.58 | 7.55 | 49.4 k | 21.3 k | 299 | 25 | 29.95 | 18 | 8018 | 85 | 154.1 k | 27 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=cache #2 | 11.21 | 89 | 14.64 | 16.68 | 60 | 28.4 | 1/39 | 80 | 2625 | 75.8 | 74 | 6.93 | 8.42 | 9.39 | 5.61 | 8.22 | 49.1 k | 20.9 k | 299 | 23.92 | 31.49 | 13 | 8001 | 83 | 154.0 k | 26 | 120 | 4 | 22.3 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #2 | 10.95 | 91 | 14.18 | 16.28 | 61 | 35.4 | 6/45 | 78 | 2625 | 76.1 | 74 | 7.24 | 8.95 | 9.53 | 5.44 | 8.01 | 49.2 k | 20.9 k | 299 | 25.48 | 33.14 | 51 | 8602 | 83 | 154.1 k | 19 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #3 | 10.53 | 95 | 13.63 | 15.35 | 65 | 30.0 | 2/41 | 79 | 2625 | 76.6 | 72 | 6.98 | 8.89 | 9.03 | 5.10 | 7.75 | 49.2 k | 21.1 k | 299 | 25.96 | 33.38 | 51 | 8417 | 78 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=batched #3 | 10.31 | 97 | 12.27 | 14.07 | 71 | 33.0 | 4/50 | 84 | 2625 | 80.2 | 75 | 6.79 | 8.39 | 8.59 | 4.77 | 8.06 | 49.4 k | 21.1 k | 299 | 24.32 | 32.24 | 16 | 8006 | 86 | 154.0 k | 27 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=cache #3 | 11.41 | 88 | 14.94 | 17.07 | 59 | 33.2 | 2/38 | 79 | 2625 | 75.4 | 73 | 7.12 | 8.60 | 9.62 | 5.78 | 8.27 | 49.1 k | 20.9 k | 299 | 24.52 | 31.67 | 12 | 8006 | 85 | 154.0 k | 25 | 120 | 4 | 22.3 k | 3.15 M |
| census 2560x1440 arm=cache #1 | 6.81 | 147 | 8.95 | 9.82 | 102 | 16.1 | 0/29 | 93 | 2625 | 94.3 | 73 | 5.53 | 5.02 | 5.61 | 3.37 | 6.04 | 13.1 k | 6100 | 156 | 22.12 | 30.03 | 7 | 620 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #1 | 7.22 | 139 | 10.66 | 14.94 | 67 | 17.2 | 0/41 | 84 | 2625 | 87.4 | 72 | 5.37 | 5.78 | 6.13 | 3.46 | 5.89 | 13.1 k | 6100 | 156 | 31.25 | 39.03 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=batched #1 | 6.61 | 151 | 8.38 | 10.90 | 92 | 17.1 | 0/29 | 92 | 2625 | 94.7 | 70 | 5.16 | 5.27 | 5.43 | 3.02 | 5.87 | 13.1 k | 6100 | 156 | 26.02 | 35.70 | 7 | 675 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=batched #2 | 6.61 | 151 | 8.46 | 11.91 | 84 | 15.2 | 0/26 | 94 | 2626 | 97.2 | 73 | 5.34 | 5.25 | 5.37 | 3.01 | 5.99 | 13.1 k | 6100 | 156 | 27.71 | 36.32 | 8 | 675 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=cache #2 | 7.05 | 142 | 9.15 | 9.84 | 102 | 24.9 | 0/33 | 89 | 2634 | 88.0 | 68 | 5.59 | 5.12 | 5.89 | 3.55 | 6.01 | 13.1 k | 6100 | 156 | 23.92 | 30.48 | 7 | 613 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #2 | 6.98 | 143 | 9.76 | 10.91 | 92 | 57.6 | 2/33 | 87 | 2625 | 91.4 | 72 | 5.13 | 5.43 | 5.73 | 3.25 | 5.80 | 13.1 k | 6100 | 156 | 23.94 | 32.75 | 49 | 4180 | 26 | 98.9 k | 8 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=base #3 | 6.95 | 144 | 9.98 | 10.69 | 94 | 21.1 | 0/35 | 86 | 2626 | 89.7 | 70 | 5.32 | 5.51 | 5.93 | 3.37 | 5.83 | 13.1 k | 6100 | 156 | 25.30 | 32.64 | 49 | 4165 | 26 | 98.9 k | 7 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=batched #3 | 6.71 | 149 | 8.20 | 8.99 | 111 | 14.3 | 0/31 | 95 | 2625 | 99.3 | 74 | 5.44 | 5.20 | 5.41 | 3.03 | 6.01 | 13.1 k | 6100 | 156 | 25.24 | 32.76 | 8 | 695 | 38 | 99.0 k | 6 | 29 | 4 | 5288 | 3.15 M |
| census 2560x1440 arm=cache #3 | 6.85 | 146 | 8.97 | 9.82 | 102 | 20.0 | 0/34 | 92 | 2625 | 93.0 | 71 | 5.52 | 5.06 | 5.66 | 3.41 | 6.06 | 13.1 k | 6100 | 156 | 23.95 | 30.44 | 7 | 622 | 35 | 98.9 k | 6 | 29 | 4 | 5288 | 3.15 M |
