# Perf suite `shadow_review2`

- exe: `../target/profiling/bench-shadow-review.exe` (release)
- date: 2026-11-22 09:32:03 UTC
- commit: `9d58175ca9cc99e0d51baece6f4ba549af18fd26` (**dirty**: 9 tracked files changed)
- exe sha256: `332ed8bf3672d1718b1cc9a33b538b227022cf9cab3cc06cb533dee086496f24` · modified 2026-11-22 08:34:21 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-20 09:38:44 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 3 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1795338783570601 (`--seed 1795338783570601` replays the order)
  - arm `base` (base): shipped defaults
  - arm `nocontact`: --off=contactshadows
  - arm `nolamps`: --set shadow.lamp.count=0
- scenarios: `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
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
| balmora_walk_night @ 2560x1440 | `base` (base) | 3 | 9.73 | 9.70-10.55 | | | | | | | | | |
| balmora_walk_night @ 2560x1440 | `nocontact` | 3 | 9.88 | 9.66-9.89 | +0.15 | +0.16 | 4.0 | 1.000 | +0.10 | +0.19 | +0.08 | -0.09 | inconclusive |
| balmora_walk_night @ 2560x1440 | `nolamps` | 3 | 9.46 | 9.14-9.52 | -0.27 | -0.56 | 0.0 | 0.100 | -0.13 | -0.26 | -0.19 | -0.46 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=nocontact ×3 | 9.88 | 101 | 11.55 | 13.47 | 74 | 32.4 | 2/40, 2/44, 3/38 | 84 | 2618 | 84.9 | 79 | 6.63 | 8.12 | 8.31 | 4.47 | 7.85 | 49.4 k | 21.3 k | 299 | 25 | 29.42 | 19 | 8039 | 123 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base ×3 | 9.73 | 103 | 11.49 | 13.33 | 75 | 32.3 | 3/48, 3/40, 2/39 | 86 | 2621 | 85.4 | 80 | 6.59 | 8.01 | 8.12 | 4.40 | 7.94 | 49.4 k | 21.3 k | 299 | 25.48 | 29.88 | 20 | 8062 | 132 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nolamps ×3 | 9.46 | 106 | 11.06 | 12.82 | 78 | 33.4 | 2/42, 3/41, 3/41 | 85 | 2619 | 86.2 | 80 | 6.26 | 7.88 | 7.86 | 4.20 | 7.48 | 49.4 k | 21.6 k | 299 | 25.70 | 29.76 | 3 | 7971 | 71 | 154.0 k | 13 | 117 | 4 | 21.7 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 arm=nocontact #1 | 9.88 | 101 | 11.56 | 13.54 | 74 | 29.0 | 2/40 | 83 | 2614 | 83.7 | 79 | 6.68 | 8.24 | 8.31 | 4.47 | 7.80 | 49.4 k | 21.5 k | 299 | 24.94 | 30.13 | 19 | 8024 | 121 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #1 | 9.70 | 103 | 11.46 | 13.33 | 75 | 28.7 | 3/48 | 86 | 2614 | 86.0 | 80 | 6.59 | 8.01 | 8.12 | 4.39 | 7.94 | 49.4 k | 21.3 k | 300 | 25.48 | 29.88 | 20 | 8062 | 132 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nolamps #1 | 9.14 | 109 | 11.05 | 12.27 | 81 | 33.4 | 2/42 | 87 | 2619 | 88.2 | 81 | 6.02 | 7.65 | 7.51 | 4.05 | 7.46 | 49.4 k | 21.8 k | 299 | 26.33 | 30.29 | 3 | 7975 | 69 | 154.0 k | 13 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nolamps #2 | 9.46 | 106 | 11.06 | 12.82 | 78 | 31.2 | 3/41 | 85 | 2621 | 86.2 | 80 | 6.26 | 7.88 | 7.86 | 4.20 | 7.48 | 49.4 k | 21.6 k | 299 | 25.70 | 29.74 | 3 | 7971 | 71 | 154.0 k | 13 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nocontact #2 | 9.66 | 104 | 11.26 | 13.06 | 77 | 30.4 | 2/44 | 85 | 2618 | 85.3 | 80 | 6.57 | 7.98 | 8.07 | 4.37 | 7.86 | 49.4 k | 21.3 k | 299 | 25.30 | 29.31 | 20 | 8063 | 128 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #2 | 10.55 | 95 | 12.40 | 14.68 | 68 | 32.3 | 3/40 | 79 | 2621 | 80.4 | 78 | 6.99 | 8.64 | 9.05 | 4.87 | 7.92 | 49.4 k | 21.3 k | 299 | 25.48 | 36.69 | 18 | 8028 | 134 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=base #3 | 9.73 | 103 | 11.49 | 13.09 | 76 | 32.0 | 2/39 | 86 | 2622 | 85.4 | 80 | 6.57 | 8.01 | 8.12 | 4.40 | 7.94 | 49.4 k | 21.3 k | 299 | 25.48 | 29.24 | 20 | 8062 | 130 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nolamps #3 | 9.52 | 105 | 11.13 | 13.01 | 77 | 32.2 | 3/41 | 85 | 2618 | 85.8 | 80 | 6.32 | 7.90 | 7.93 | 4.26 | 7.49 | 49.4 k | 21.5 k | 299 | 25.48 | 29.76 | 3 | 7964 | 72 | 154.0 k | 13 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk_night 2560x1440 arm=nocontact #3 | 9.89 | 101 | 11.55 | 13.47 | 74 | 32.4 | 3/38 | 84 | 2618 | 84.9 | 79 | 6.63 | 8.12 | 8.31 | 4.47 | 7.85 | 49.4 k | 21.3 k | 299 | 25 | 29.42 | 19 | 8039 | 123 | 154.1 k | 15 | 117 | 4 | 21.7 k | 3.15 M |
