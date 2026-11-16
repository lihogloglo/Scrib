# Perf suite `f_nohold`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: cascadehold · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadehold ×3 | 11.60 | 86 | 14.25 | 16.73 | 60 | 57.9 | 3/40, 3/40, 3/42 | 80 | 2580 | 78.0 | 73 | 7.80 | 59.3 k | 31.7 k | 286 | 26.92 | 34.54 | 45 | 18.6 k | 76 | 16.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadehold ×3 | 10.61 | 94 | 11.65 | 12.22 | 82 | 17.3 | 0/49, 0/49, 0/48 | 98 | 2580 | 94.1 | 74 | 9.48 | 47.1 k | 24.7 k | 210 | 24.52 | 30.05 | 3 | 14.4 k | 69 | 14.6 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadehold #1 | 11.78 | 85 | 14.67 | 17.26 | 58 | 57.9 | 3/40 | 80 | 2580 | 77.3 | 72 | 7.85 | 59.3 k | 31.7 k | 286 | 26.92 | 34.38 | 45 | 18.5 k | 78 | 16.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadehold #2 | 11.60 | 86 | 14.25 | 16.73 | 60 | 31.5 | 3/40 | 80 | 2580 | 78.0 | 73 | 7.80 | 59.3 k | 31.7 k | 286 | 26.34 | 34.54 | 45 | 18.6 k | 76 | 16.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadehold #3 | 11.54 | 87 | 14.16 | 16.35 | 61 | 31.0 | 3/42 | 82 | 2580 | 78.8 | 73 | 7.77 | 59.3 k | 31.9 k | 286 | 27.40 | 35.22 | 45 | 18.6 k | 75 | 16.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadehold #1 | 10.60 | 94 | 11.59 | 12.21 | 82 | 14.9 | 0/49 | 98 | 2580 | 93.7 | 74 | 9.48 | 47.1 k | 24.7 k | 210 | 24.28 | 28.89 | 3 | 14.4 k | 69 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadehold #2 | 10.61 | 94 | 11.65 | 12.22 | 82 | 15.0 | 0/49 | 98 | 2580 | 94.3 | 74 | 9.48 | 47.1 k | 24.7 k | 210 | 25 | 30.05 | 3 | 14.4 k | 69 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadehold #3 | 10.67 | 94 | 11.79 | 12.52 | 80 | 17.3 | 0/48 | 98 | 2580 | 94.1 | 73 | 9.51 | 47.1 k | 24.7 k | 210 | 24.52 | 31.14 | 3 | 14.4 k | 69 | 14.6 k | 0 | 0 | 0 | 0 | 0 |
