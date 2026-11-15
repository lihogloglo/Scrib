# Perf suite `scene_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.60 | 86 | 14.14 | 16.23 | 62 | 42.6 | 3/23, 3/28, 4/16 | 79 | 2580 | 78.8 | 73 | 7.20 | 59.3 k | 31.7 k | 286 | 25.96 | 32.38 | 46 | 19.6 k | 87 | 17.4 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.56 | 95 | 11.85 | 12.62 | 79 | 113.8 | 3/26, 0/29, 0/31 | 98 | 2580 | 94.3 | 75 | 8.78 | 47.1 k | 24.7 k | 210 | 30.36 | 39.32 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.73 | 85 | 14.67 | 17.17 | 58 | 37.4 | 3/23 | 78 | 2582 | 77.2 | 69 | 7.16 | 59.3 k | 31.7 k | 286 | 26.15 | 31.53 | 46 | 19.4 k | 86 | 17.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 11.47 | 87 | 13.64 | 15.58 | 64 | 32.3 | 3/28 | 81 | 2580 | 79.1 | 73 | 7.20 | 59.3 k | 31.7 k | 286 | 25.94 | 32.72 | 46 | 19.6 k | 87 | 17.4 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.60 | 86 | 14.14 | 16.23 | 62 | 42.6 | 4/16 | 79 | 2580 | 78.8 | 74 | 7.21 | 59.3 k | 31.7 k | 286 | 25.96 | 32.38 | 46 | 19.6 k | 87 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.59 | 94 | 12.05 | 13.87 | 72 | 113.8 | 3/26 | 98 | 2580 | 94.4 | 76 | 8.77 | 47.1 k | 24.7 k | 210 | 29.76 | 39.64 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.56 | 95 | 11.72 | 12.16 | 82 | 14.7 | 0/29 | 97 | 2580 | 93.1 | 73 | 8.82 | 47.1 k | 24.7 k | 210 | 30.36 | 35.74 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.54 | 95 | 11.85 | 12.62 | 79 | 23.3 | 0/31 | 98 | 2580 | 94.3 | 75 | 8.78 | 47.1 k | 24.7 k | 210 | 31.88 | 39.32 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
