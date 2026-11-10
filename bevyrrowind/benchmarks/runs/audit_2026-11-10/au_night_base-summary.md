# Perf suite `au_night_base`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows; `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 ×3 | 8.92 | 112 | 10.80 | 11.77 | 85 | 19.3 | 0/6, 0/9, 0/4 | 94 | 2581 | 92.2 | 73 | 8.15 | 47.0 k | 24.7 k | 222 | 25.49 | 39.37 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_walk_night 2560x1440 ×3 | 11.31 | 88 | 15.82 | 17.36 | 58 | 132.4 | 1/5, 1/6, 7/5 | 68 | 2580 | 69.9 | 70 | 7.10 | 59.3 k | 31.7 k | 300 | 24.35 | 37.30 | 21 | 11.6 k | 69 | 11.0 k | 157 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 #1 | 8.92 | 112 | 11.39 | 12.52 | 80 | 19.3 | 0/6 | 94 | 2580 | 92.2 | 73 | 8.15 | 47.0 k | 24.7 k | 222 | 24.70 | 46.49 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_night 2560x1440 #2 | 8.92 | 112 | 10.80 | 11.77 | 85 | 16.6 | 0/9 | 94 | 2581 | 91.0 | 74 | 8.22 | 47.0 k | 24.7 k | 222 | 25.68 | 39.37 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_night 2560x1440 #3 | 8.53 | 117 | 10.06 | 11.08 | 90 | 14.0 | 0/4 | 95 | 2581 | 93.4 | 72 | 7.89 | 47.0 k | 24.7 k | 222 | 25.49 | 36.72 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_walk_night 2560x1440 #1 | 11.31 | 88 | 15.75 | 17.23 | 58 | 43.9 | 1/5 | 69 | 2580 | 69.9 | 70 | 7.11 | 59.3 k | 31.7 k | 300 | 23.74 | 32.38 | 21 | 11.5 k | 71 | 11.0 k | 161 |
| balmora_walk_night 2560x1440 #2 | 11.31 | 88 | 15.82 | 17.36 | 58 | 132.4 | 1/6 | 67 | 2580 | 68.8 | 70 | 7.10 | 59.3 k | 31.9 k | 301 | 24.55 | 37.30 | 21 | 11.6 k | 69 | 11.0 k | 157 |
| balmora_walk_night 2560x1440 #3 | 11.05 | 91 | 16.61 | 19.96 | 50 | 41.4 | 7/5 | 68 | 2580 | 69.9 | 71 | 7.06 | 59.3 k | 31.7 k | 300 | 24.35 | 43.37 | 21 | 11.6 k | 67 | 11.0 k | 153 |
