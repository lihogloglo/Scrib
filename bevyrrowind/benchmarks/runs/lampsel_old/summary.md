# Perf suite `lampsel_old`

- exe: `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows; `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 ×3 | 10.29 | 97 | 11.57 | 12.74 | 79 | 95.6 | 3/8, 1/35, 0/35 | 98 | 2580 | 93.1 | 77 | 8.68 | 47.1 k | 24.7 k | 222 | 30.77 | 38.48 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 12.39 | 81 | 15.69 | 18.16 | 55 | 45.8 | 4/23, 4/24, 5/22 | 79 | 2580 | 76.6 | 76 | 7.69 | 59.2 k | 31.6 k | 286 | 24.04 | 29.36 | 22 | 18.1 k | 125 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 ×3 | 11.74 | 85 | 16.33 | 18.35 | 55 | 53.3 | 6/26, 7/20, 6/21 | 77 | 2580 | 74.5 | 76 | 7.53 | 59.3 k | 31.7 k | 300 | 25 | 31.12 | 21 | 11.5 k | 104 | 11.0 k | 10 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 #1 | 35.28 | 28 | 46.05 | 49.30 | 20 | 95.6 | 3/8 | 98 | 2580 | 55.6 | 70 | 22.66 | 45.6 k | 24.5 k | 222 | 38.99 | 62.50 | 3 | 11.1 k | 94 | 11.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #2 | 10.21 | 98 | 11.50 | 12.74 | 79 | 27.9 | 1/35 | 98 | 2580 | 93.1 | 77 | 8.60 | 47.1 k | 24.7 k | 222 | 29.26 | 34.67 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #3 | 10.29 | 97 | 11.57 | 12.34 | 81 | 18.7 | 0/35 | 98 | 2580 | 93.1 | 78 | 8.68 | 47.1 k | 24.7 k | 222 | 30.77 | 38.48 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 12.39 | 81 | 15.57 | 17.96 | 56 | 45.8 | 4/23 | 80 | 2580 | 77.0 | 76 | 7.69 | 59.2 k | 31.7 k | 286 | 24.04 | 29.63 | 22 | 18.0 k | 124 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.41 | 81 | 15.88 | 18.16 | 55 | 32.6 | 4/24 | 77 | 2580 | 76.5 | 76 | 7.81 | 59.2 k | 31.6 k | 286 | 24.04 | 29.36 | 22 | 18.1 k | 128 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 12.05 | 83 | 15.69 | 18.39 | 54 | 38.3 | 5/22 | 79 | 2580 | 76.6 | 76 | 7.59 | 59.2 k | 31.6 k | 286 | 24.32 | 29.25 | 22 | 18.1 k | 125 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #1 | 11.26 | 89 | 14.46 | 16.61 | 60 | 47.5 | 6/26 | 78 | 2580 | 76.6 | 77 | 7.36 | 59.3 k | 31.7 k | 300 | 25.65 | 31.12 | 21 | 11.8 k | 104 | 11.1 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #2 | 11.74 | 85 | 16.33 | 18.35 | 55 | 53.3 | 7/20 | 77 | 2580 | 74.5 | 76 | 7.69 | 59.3 k | 31.7 k | 300 | 25 | 31.18 | 21 | 11.5 k | 104 | 11.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #3 | 11.75 | 85 | 17.02 | 18.94 | 53 | 35.2 | 6/21 | 75 | 2580 | 73.4 | 75 | 7.53 | 59.3 k | 31.7 k | 300 | 24.41 | 30.47 | 21 | 11.4 k | 105 | 11.0 k | 11 | 0 | 0 | 0 | 0 |
