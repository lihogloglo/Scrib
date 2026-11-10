# Perf suite `au_base`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.33 | 107 | 10.54 | 12.05 | 83 | 41.0 | 1/10, 2/24, 2/25 | 97 | 2580 | 93.3 | 70 | 8.29 | 46.9 k | 24.6 k | 210 | 25 | 33.11 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora_walk 2560x1440 ×3 | 11.63 | 86 | 14.21 | 16.18 | 62 | 80.5 | 0/5, 8/5, 1/4 | 72 | 2580 | 72.7 | 70 | 7.25 | 59.2 k | 31.7 k | 286 | 23.45 | 32.08 | 22 | 18.1 k | 85 | 17.2 k | 190 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.33 | 107 | 10.71 | 12.61 | 79 | 28.3 | 1/10 | 95 | 2585 | 91.0 | 65 | 8.29 | 46.9 k | 24.6 k | 210 | 25 | 31.95 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora 2560x1440 #2 | 9.37 | 107 | 10.54 | 12.05 | 83 | 41.0 | 2/24 | 98 | 2580 | 93.3 | 70 | 8.37 | 46.9 k | 24.6 k | 210 | 24.04 | 33.11 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora 2560x1440 #3 | 9.25 | 108 | 10.44 | 11.61 | 86 | 39.9 | 2/25 | 97 | 2580 | 94.1 | 71 | 8.28 | 46.9 k | 24.6 k | 210 | 25.48 | 33.67 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora_walk 2560x1440 #1 | 11.46 | 87 | 14.21 | 16.18 | 62 | 25.1 | 0/5 | 72 | 2580 | 73.1 | 69 | 7.24 | 59.2 k | 31.6 k | 286 | 23.08 | 30.08 | 22 | 18.1 k | 85 | 17.2 k | 190 |
| balmora_walk 2560x1440 #2 | 11.70 | 85 | 15.50 | 19.21 | 52 | 66.4 | 8/5 | 72 | 2580 | 72.7 | 70 | 7.44 | 59.2 k | 31.7 k | 286 | 23.45 | 38.47 | 22 | 18.2 k | 85 | 17.4 k | 190 |
| balmora_walk 2560x1440 #3 | 11.63 | 86 | 14.19 | 15.89 | 63 | 80.5 | 1/4 | 72 | 2580 | 72.7 | 71 | 7.25 | 59.2 k | 31.7 k | 286 | 23.77 | 32.08 | 22 | 18.1 k | 84 | 17.2 k | 188 |
