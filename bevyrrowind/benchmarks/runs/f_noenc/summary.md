# Perf suite `f_noenc`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: cascadeencoders · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadeencoders ×3 | 11.26 | 89 | 14.29 | 16.45 | 61 | 41.6 | 5/43, 2/47, 4/41 | 77 | 2580 | 77.4 | 71 | 7.13 | 59.3 k | 31.7 k | 286 | 26.32 | 33.81 | 45 | 19.2 k | 67 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders ×3 | 10.25 | 98 | 11.73 | 12.34 | 81 | 53.1 | 0/51, 0/56, 1/55 | 98 | 2580 | 94.7 | 74 | 8.88 | 47.1 k | 24.7 k | 210 | 25 | 33.36 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadeencoders #1 | 11.26 | 89 | 14.57 | 17.01 | 59 | 41.6 | 5/43 | 77 | 2580 | 77.0 | 71 | 7.13 | 59.3 k | 31.7 k | 286 | 26.32 | 34.06 | 45 | 19.2 k | 64 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadeencoders #2 | 11.21 | 89 | 14.02 | 16.24 | 62 | 31.2 | 2/47 | 78 | 2580 | 77.4 | 71 | 7.13 | 59.3 k | 31.7 k | 286 | 26.92 | 33.57 | 45 | 19.3 k | 68 | 10.9 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadeencoders #3 | 11.28 | 89 | 14.29 | 16.45 | 61 | 30.6 | 4/41 | 77 | 2580 | 77.5 | 72 | 7.02 | 59.3 k | 31.7 k | 286 | 25.96 | 33.81 | 45 | 19.2 k | 67 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders #1 | 10.29 | 97 | 11.73 | 12.28 | 81 | 14.1 | 0/51 | 98 | 2580 | 94.7 | 75 | 9.02 | 47.1 k | 24.7 k | 210 | 25.53 | 33.36 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders #2 | 10.25 | 98 | 11.70 | 12.34 | 81 | 20.0 | 0/56 | 98 | 2580 | 94.7 | 74 | 8.88 | 47.1 k | 24.7 k | 210 | 25 | 33.56 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders #3 | 10.15 | 98 | 11.74 | 12.53 | 80 | 53.1 | 1/55 | 98 | 2580 | 94.8 | 74 | 8.85 | 47.1 k | 24.7 k | 210 | 23.56 | 29.77 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
