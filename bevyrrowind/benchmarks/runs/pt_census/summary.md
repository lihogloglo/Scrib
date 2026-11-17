# Perf suite `pt_census`

- exe: `target/profiling/perftests.exe` (release)
- date: 2026-11-17 10:18:42 UTC
- commit: `c4d9ef509bc4dad4ae1eb5bdc12a8e0a892705c2` (**dirty**: 22 tracked files changed)
- exe sha256: `8499f6a0e88aa33660b0e9c8f6572e089222c1e0131fe64811e0c42232516c2f` · modified 2026-11-17 07:43:48 UTC
- bake: profile `mw-d802348b85b1`, manifest 2026-11-17 07:07:55 UTC
- power: plan Balanced (381b4222-f694-41f0-9685-ff5bb260df2e) · mode best performance · AC
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- settings for every run: --env BEVYRROWIND_DLSS=quality --env BEVYRROWIND_WINDOW=borderless · hitch threshold: 2.5 × median, ≥ 25 ms
- A/B: 2 arms interleaved (counterbalanced ABBA/BAAB per repeat, one warm-up per arm discarded), seed 1794910435821577 (`--seed 1794910435821577` replays the order)
  - arm `base` (base): shipped defaults
  - arm `park`: --set stream.interior.disable_exterior=on
- scenarios: `census` interior: Seyda Neen Census and Excise Office walk-through
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
| census @ 2560x1440 | `base` (base) | 4 | 6.69 | 6.66-6.79 | | | | | | | | | |
| census @ 2560x1440 | `park` | 4 | 6.71 | 6.53-6.74 | +0.02 | -0.01 | 9.0 | 0.886 | -0.01 | -0.10 | -0.05 | +0.00 | inconclusive |

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| census 2560x1440 arm=base ×4 | 6.69 | 149 | 9.55 | 10.20 | 98 | 16.4 | 0/45, 0/41, 0/40, 0/39 | 80 | 2625 | 90.1 | 73 | 5.13 | 5.32 | 5.57 | 3.21 | 5.40 | 15.7 k | 8725 | 156 | 23.80 | 30.44 | 49 | 4157 | 18 | 578 | 2 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=park ×4 | 6.71 | 149 | 9.46 | 10.53 | 95 | 19.9 | 0/45, 0/44, 0/38, 0/39 | 81 | 2625 | 90.0 | 73 | 5.03 | 5.31 | 5.47 | 3.16 | 5.40 | 15.7 k | 8725 | 156 | 25.24 | 30.41 | 49 | 4158 | 18 | 575 | 3 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | main busy ms (mean) | render busy ms (mean) | render graph ms (mean) | GPU timed ms (mean) | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| census 2560x1440 arm=base #1 | 6.66 | 150 | 10.14 | 10.92 | 92 | 12.6 | 0/45 | 78 | 2625 | 87.1 | 72 | 5.13 | 5.38 | 5.72 | 3.29 | 5.39 | 15.7 k | 8725 | 156 | 25.96 | 30.73 | 49 | 4157 | 18 | 563 | 2 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=park #1 | 6.70 | 149 | 9.63 | 11.53 | 87 | 19.9 | 0/45 | 81 | 2626 | 88.1 | 73 | 5.07 | 5.34 | 5.53 | 3.19 | 5.41 | 15.7 k | 8725 | 156 | 25.69 | 30.66 | 49 | 4157 | 18 | 582 | 3 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=park #2 | 6.74 | 148 | 9.30 | 10.00 | 100 | 18.8 | 0/44 | 82 | 2625 | 90.1 | 73 | 4.99 | 5.28 | 5.41 | 3.13 | 5.40 | 15.7 k | 8725 | 156 | 24.94 | 30.40 | 49 | 4158 | 18 | 582 | 3 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=base #2 | 6.70 | 149 | 9.42 | 10.15 | 99 | 14.8 | 0/41 | 81 | 2625 | 90.2 | 73 | 5.11 | 5.31 | 5.55 | 3.20 | 5.39 | 15.7 k | 8725 | 156 | 23.08 | 29.87 | 49 | 4157 | 18 | 578 | 2 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=base #3 | 6.69 | 149 | 9.59 | 10.17 | 98 | 14.2 | 0/40 | 80 | 2625 | 90.0 | 73 | 5.13 | 5.34 | 5.59 | 3.22 | 5.40 | 15.7 k | 8725 | 156 | 22.33 | 30.16 | 49 | 4157 | 18 | 578 | 2 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=park #3 | 6.53 | 153 | 9.29 | 10.24 | 98 | 13.2 | 0/38 | 83 | 2625 | 92.7 | 74 | 4.93 | 5.24 | 5.35 | 3.09 | 5.40 | 15.7 k | 8725 | 156 | 25.54 | 30.41 | 49 | 4160 | 18 | 567 | 2 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=park #4 | 6.72 | 149 | 9.84 | 10.82 | 92 | 15.2 | 0/39 | 78 | 2625 | 89.9 | 73 | 5.18 | 5.37 | 5.70 | 3.30 | 5.39 | 15.7 k | 8725 | 156 | 23.80 | 30.29 | 49 | 4157 | 18 | 568 | 3 | 0 | 0 | 0 | 0 |
| census 2560x1440 arm=base #4 | 6.79 | 147 | 9.50 | 10.23 | 98 | 16.4 | 0/39 | 82 | 2625 | 90.7 | 74 | 5.12 | 5.30 | 5.55 | 3.21 | 5.40 | 15.7 k | 8725 | 156 | 24.52 | 31.10 | 49 | 4157 | 18 | 582 | 3 | 0 | 0 | 0 | 0 |
