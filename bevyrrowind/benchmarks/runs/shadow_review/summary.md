# Perf suite `shadow_review`

- exe: `../target/profiling/bench-shadow-review.exe` (release)
- date: 2026-11-22 09:12:24 UTC
- commit: `9d58175ca9cc99e0d51baece6f4ba549af18fd26` (**dirty**: 6 tracked files changed)
- exe sha256: `332ed8bf3672d1718b1cc9a33b538b227022cf9cab3cc06cb533dee086496f24` · modified 2026-11-22 08:34:21 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-20 09:38:44 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 5 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795336535795086 (`--seed 1795336535795086` replays the order)
  - arm `base` (base): shipped defaults
  - arm `pcf0`: --set shadow.lamp.atlas.soft_pcf_radius=0
  - arm `lamps16`: --set shadow.lamp.atlas.max_lamps=16
  - arm `pcf0lamps16`: --set shadow.lamp.atlas.soft_pcf_radius=0 --set shadow.lamp.atlas.max_lamps=16
  - arm `stock`: --set shadow.lamp.atlas.enabled=off
- scenarios: `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- valid runs: 14 of 15 (5 warm-up runs discarded)
- **dropped**: balmora_walk_night @ 2560x1440 #1 [lamps16]: window out of focus during the run (after 2 retries)
- bench lock: held for the whole chain (`D:\Gamedev\Bevyrrowind\.sessions\bench.lock`), after waiting 0 min for another bench
- power plan: Balanced (no High performance / Ultimate plan on this machine; the power mode below is what applies)
- power throttling: exemption skipped (needs an elevated prompt)
- machine: quiet, every kept run checked (before each run: no cargo/rustc/bevyrrowind/ovid process nor any exe under `target`, `c:\cargo-target\scrib`, `..\target`, `c:\users\leoglo~1\appdata\local\temp`, `c:\users\leogloglo\appdata\local\temp`, `d:\gamedev\bevyrrowind\target`, no other process ≥ 15% GPU SM, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU and < 15% GPU SM, none of those processes at the end, window in focus)
- focus: clean, all 14 runs (window in front for every measured frame)

## A/B verdicts vs `base`

Rule (set before the runs): an arm differs when |Δ| > the larger arm's min-max range of run medians, or Mann-Whitney p < 0.05; else **inconclusive** (keep the code opt-in, never delete on it). Δ = median of the arm's run medians minus the base's; paired Δ = median over repeats of (arm - base) in the same repeat; thread Δ = per-frame means (main busy / render busy / render graph / GPU timed), median over runs.

| scenario | arm | runs | median ms | min-max | Δ ms | paired Δ | U | p | main Δ | render Δ | graph Δ | GPU Δ | verdict |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| balmora_walk_night @ 2560x1440 | `base` (base) | 3 | 9.79 | 9.72-9.88 | | | | | | | | | |
| balmora_walk_night @ 2560x1440 | `pcf0` | 3 | 9.79 | 9.71-10.02 | -0.00 | -0.00 | 4.0 | 1.000 | -0.01 | +0.00 | +0.00 | +0.01 | inconclusive |
| balmora_walk_night @ 2560x1440 | `lamps16` | 2 | 9.95 | 9.75-10.15 | +0.16 | +0.15 | 4.0 | 0.800 | +0.10 | +0.24 | +0.14 | +0.03 | inconclusive |
| balmora_walk_night @ 2560x1440 | `pcf0lamps16` | 3 | 9.84 | 9.78-9.90 | +0.05 | +0.05 | 6.0 | 0.700 | +0.07 | +0.11 | +0.06 | -0.01 | inconclusive |
| balmora_walk_night @ 2560x1440 | `stock` | 3 | 10.38 | 10.32-10.60 | +0.59 | +0.53 | 9.0 | 0.100 | +0.50 | +0.57 | +0.50 | -0.07 | **slower** |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=pcf0lamps16 ×3 | 9.84 | 102 | 11.52 | 13.41 | 75 | 35.7 | 2/42, 4/42, 2/41 | 84 | 2620 | 84.2 | 79 | 6.45 | 8.13 | 8.26 | 4.45 | 7.89 | 49.4 k | 21.3 k | 299 | 25.48 | 29.54 | 20 | 8009 | 130 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=stock ×3 | 10.38 | 96 | 12.46 | 14.48 | 69 | 38.6 | 3/44, 3/50, 3/40 | 80 | 2619 | 81.3 | 79 | 6.76 | 8.56 | 8.72 | 4.89 | 7.83 | 49.4 k | 21.1 k | 299 | 26.79 | 31.50 | 51 | 8449 | 81 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0 ×3 | 9.79 | 102 | 11.56 | 13.25 | 75 | 32.0 | 3/48, 2/41, 3/53 | 85 | 2618 | 85.2 | 79 | 6.50 | 8.04 | 8.15 | 4.39 | 7.92 | 49.4 k | 21.3 k | 299 | 25.29 | 29.53 | 20 | 8010 | 129 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base ×3 | 9.79 | 102 | 11.56 | 13.25 | 75 | 35.9 | 4/43, 2/48, 3/47 | 85 | 2618 | 84.8 | 80 | 6.45 | 8.06 | 8.14 | 4.39 | 7.90 | 49.4 k | 21.3 k | 299 | 25.48 | 29.46 | 19 | 8036 | 133 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=lamps16 ×2 | 9.95 | 100 | 11.75 | 13.52 | 74 | 34.3 | 2/43, 2/44 | 84 | 2621 | 83.9 | 79 | 6.64 | 8.15 | 8.38 | 4.53 | 7.93 | 49.4 k | 21.2 k | 299 | 25.24 | 29.37 | 19 | 8032 | 132 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=pcf0lamps16 #1 | 9.84 | 102 | 11.52 | 13.41 | 75 | 35.7 | 2/42 | 84 | 2619 | 84.2 | 79 | 6.62 | 8.13 | 8.26 | 4.45 | 7.88 | 49.4 k | 21.3 k | 299 | 25.48 | 30.48 | 18 | 8009 | 116 | 154.0 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=stock #1 | 10.32 | 97 | 12.42 | 14.48 | 69 | 38.6 | 3/44 | 80 | 2619 | 81.3 | 79 | 6.63 | 8.56 | 8.72 | 4.83 | 7.78 | 49.4 k | 21.1 k | 299 | 26.92 | 31.50 | 51 | 8391 | 78 | 154.0 k | 16 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0 #1 | 9.79 | 102 | 11.56 | 13.25 | 75 | 32.0 | 3/48 | 85 | 2613 | 85.2 | 79 | 6.42 | 8.04 | 8.15 | 4.39 | 7.92 | 49.4 k | 21.3 k | 299 | 25.36 | 29.54 | 20 | 8010 | 129 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #1 | 9.79 | 102 | 11.56 | 13.20 | 76 | 35.9 | 4/43 | 85 | 2622 | 84.8 | 79 | 6.39 | 8.06 | 8.13 | 4.39 | 7.90 | 49.4 k | 21.3 k | 299 | 26.01 | 29.46 | 20 | 8016 | 133 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0 #2 | 9.71 | 103 | 11.45 | 13.15 | 76 | 31.9 | 2/41 | 86 | 2619 | 86.1 | 80 | 6.50 | 8.04 | 8.10 | 4.37 | 7.90 | 49.4 k | 21.3 k | 299 | 25.29 | 29.53 | 20 | 8009 | 132 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #2 | 9.72 | 103 | 11.52 | 13.25 | 75 | 34.8 | 2/48 | 86 | 2613 | 85.4 | 80 | 6.45 | 8.01 | 8.14 | 4.38 | 7.96 | 49.4 k | 21.3 k | 299 | 25 | 29.21 | 19 | 8049 | 116 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=lamps16 #2 | 10.15 | 99 | 12.04 | 13.94 | 72 | 34.3 | 2/43 | 81 | 2622 | 82.3 | 79 | 6.78 | 8.29 | 8.63 | 4.66 | 7.96 | 49.4 k | 21.1 k | 299 | 24.52 | 29.37 | 18 | 8052 | 131 | 154.2 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=stock #2 | 10.60 | 94 | 13.00 | 15.26 | 66 | 33.5 | 3/50 | 79 | 2619 | 80.1 | 78 | 6.98 | 8.75 | 9.03 | 5.03 | 7.83 | 49.4 k | 21.3 k | 299 | 26.44 | 31.88 | 51 | 8462 | 81 | 154.1 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0lamps16 #2 | 9.90 | 101 | 11.64 | 13.48 | 74 | 31.2 | 4/42 | 84 | 2623 | 84.2 | 79 | 6.45 | 8.18 | 8.27 | 4.45 | 7.89 | 49.4 k | 21.3 k | 299 | 25.48 | 29.54 | 20 | 8009 | 133 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #3 | 9.88 | 101 | 11.57 | 13.57 | 74 | 35.3 | 3/47 | 84 | 2618 | 84.7 | 80 | 6.64 | 8.20 | 8.28 | 4.47 | 7.89 | 49.4 k | 21.5 k | 299 | 25.48 | 30.58 | 19 | 8036 | 138 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=stock #3 | 10.38 | 96 | 12.46 | 14.42 | 69 | 32.0 | 3/40 | 81 | 2619 | 81.9 | 79 | 6.76 | 8.52 | 8.69 | 4.89 | 7.88 | 49.4 k | 21.1 k | 299 | 26.79 | 31.13 | 51 | 8449 | 82 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0 #3 | 10.02 | 100 | 11.83 | 13.98 | 72 | 31.0 | 3/53 | 83 | 2618 | 83.1 | 79 | 6.58 | 8.28 | 8.49 | 4.56 | 7.93 | 49.4 k | 21.3 k | 299 | 25 | 29.33 | 20 | 8022 | 118 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=pcf0lamps16 #3 | 9.78 | 102 | 11.51 | 13.30 | 75 | 32.2 | 2/41 | 85 | 2620 | 84.8 | 79 | 6.39 | 8.05 | 8.13 | 4.36 | 7.94 | 49.4 k | 21.3 k | 299 | 25.48 | 29.41 | 20 | 8058 | 130 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=lamps16 #3 | 9.75 | 103 | 11.46 | 13.11 | 76 | 32.6 | 2/44 | 86 | 2619 | 85.5 | 80 | 6.50 | 8.01 | 8.13 | 4.39 | 7.91 | 49.4 k | 21.3 k | 299 | 25.96 | 29.37 | 20 | 8012 | 133 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
