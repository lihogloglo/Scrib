# Perf suite `push_all`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 10.56 | 95 | 11.73 | 12.23 | 82 | 28.6 | 1/8, 0/8, 0/7 | 98 | 2580 | 89.4 | 70 | 9.41 | 47.0 k | 24.6 k | 210 | 28.37 | 38.25 | 4 | 55.9 k | 71 | 56.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 10.93 | 92 | 14.96 | 21.98 | 45 | 101.7 | 42/31, 36/25, 39/21 | 82 | 2580 | 78.0 | 72 | 8.04 | 59.2 k | 31.6 k | 286 | 24.16 | 31.64 | 22 | 233.4 k | 83 | 69.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 ×3 | 9.31 | 107 | 10.39 | 10.95 | 91 | 29.3 | 0/7, 0/5, 1/5 | 98 | 2580 | 92.6 | 74 | 8.57 | 47.1 k | 24.7 k | 222 | 30.77 | 37.46 | 3 | 45.0 k | 63 | 45.4 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 10.71 | 93 | 11.99 | 12.73 | 79 | 28.6 | 1/8 | 98 | 2580 | 87.3 | 69 | 9.51 | 47.0 k | 24.6 k | 210 | 30.77 | 44.74 | 4 | 55.9 k | 71 | 56.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.56 | 95 | 11.73 | 12.23 | 82 | 13.8 | 0/8 | 98 | 2580 | 89.4 | 70 | 9.41 | 47.0 k | 24.6 k | 210 | 28.18 | 37.00 | 4 | 55.9 k | 71 | 56.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.12 | 99 | 11.40 | 12.16 | 82 | 24.2 | 0/7 | 97 | 2580 | 90.4 | 71 | 8.92 | 47.0 k | 24.6 k | 210 | 28.37 | 38.25 | 4 | 55.9 k | 71 | 56.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 10.66 | 94 | 14.96 | 21.63 | 46 | 101.7 | 42/31 | 82 | 2580 | 78.2 | 71 | 7.96 | 59.2 k | 31.6 k | 286 | 24.16 | 31.64 | 22 | 231.7 k | 83 | 69.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 10.93 | 92 | 14.16 | 21.98 | 45 | 83.7 | 36/25 | 83 | 2580 | 78.0 | 72 | 8.04 | 59.2 k | 31.8 k | 286 | 24.28 | 29.84 | 22 | 246.2 k | 85 | 69.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.32 | 88 | 15.34 | 24.41 | 41 | 88.3 | 39/21 | 79 | 2580 | 74.9 | 72 | 8.18 | 59.2 k | 31.6 k | 286 | 23.31 | 33.70 | 22 | 233.4 k | 83 | 69.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #1 | 9.33 | 107 | 10.39 | 10.87 | 92 | 13.1 | 0/7 | 98 | 2580 | 92.9 | 75 | 8.59 | 47.1 k | 24.7 k | 222 | 30.77 | 37.83 | 3 | 45.0 k | 63 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #2 | 9.31 | 107 | 10.40 | 10.95 | 91 | 16.6 | 0/5 | 98 | 2580 | 92.6 | 74 | 8.57 | 47.1 k | 24.7 k | 222 | 30.95 | 37.46 | 3 | 45.0 k | 63 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #3 | 9.16 | 109 | 10.28 | 11.21 | 89 | 29.3 | 1/5 | 97 | 2580 | 92.3 | 74 | 8.37 | 47.1 k | 24.7 k | 222 | 26.50 | 34.25 | 3 | 45.0 k | 63 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
