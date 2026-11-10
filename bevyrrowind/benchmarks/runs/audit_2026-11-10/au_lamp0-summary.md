# Perf suite `au_lamp0`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 10.88 | 92 | 13.23 | 14.92 | 67 | 154.9 | 0/5, 92/4, 8/10 | 76 | 2580 | 75.7 | 71 | 7.41 | 59.4 k | 31.8 k | 286 | 23.08 | 38.66 | 4 | 17.2 k | 80 | 17.3 k | 180 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 10.77 | 93 | 13.23 | 14.92 | 67 | 21.9 | 0/5 | 76 | 2580 | 75.7 | 70 | 7.01 | 59.2 k | 31.8 k | 286 | 23.08 | 37.95 | 4 | 17.2 k | 80 | 17.4 k | 180 |
| balmora_walk 2560x1440 #2 | 11.20 | 89 | 15.58 | 46.13 | 22 | 154.9 | 92/4 | 72 | 2528 | 71.3 | 71 | 7.48 | 59.4 k | 31.6 k | 289 | 22.12 | 41.60 | 4 | 16.7 k | 80 | 16.8 k | 180 |
| balmora_walk 2560x1440 #3 | 10.88 | 92 | 12.93 | 14.73 | 68 | 56.7 | 8/10 | 80 | 2580 | 77.6 | 73 | 7.41 | 59.4 k | 31.8 k | 286 | 23.83 | 38.66 | 4 | 17.2 k | 79 | 17.3 k | 178 |
