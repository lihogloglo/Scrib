# Perf suite `lampsel_new`

- exe: `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows; `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 ×3 | 11.43 | 88 | 56.25 | 59.02 | 17 | 128.6 | 95/195, 0/2, 2/31 | 98 | 2580 | 65.0 | 73 | 15.87 | 47.1 k | 24.7 k | 222 | 23.56 | 34.91 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 13.08 | 76 | 18.54 | 26.37 | 38 | 50.3 | 10/20, 2/18, 8/14 | 72 | 2580 | 72.7 | 75 | 7.91 | 59.2 k | 31.5 k | 286 | 24.04 | 31.23 | 46 | 19.4 k | 125 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 ×3 | 12.08 | 83 | 16.53 | 18.96 | 53 | 37.0 | 2/21, 3/19, 3/18 | 70 | 2580 | 70.7 | 74 | 7.16 | 59.3 k | 31.5 k | 300 | 25 | 31.81 | 51 | 13.2 k | 104 | 11.0 k | 10 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 #1 | 11.43 | 88 | 56.25 | 59.02 | 17 | 62.7 | 95/195 | 98 | 2580 | 65.0 | 73 | 15.87 | 47.1 k | 24.7 k | 222 | 23.56 | 34.91 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #2 | 106.73 | 9 | 113.22 | 116.33 | 9 | 128.6 | 0/2 | 99 | 2580 | 51.6 | 68 | 89.45 | 47.1 k | 24.7 k | 222 | 10.79 | 34.66 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #3 | 10.28 | 97 | 11.86 | 13.60 | 74 | 32.8 | 2/31 | 97 | 2580 | 92.3 | 76 | 8.49 | 47.1 k | 24.7 k | 222 | 33.50 | 37.83 | 3 | 11.3 k | 94 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 13.41 | 75 | 26.02 | 29.07 | 34 | 49.3 | 10/20 | 67 | 2580 | 68.0 | 74 | 8.00 | 59.2 k | 30.9 k | 286 | 24.04 | 38.77 | 34 | 17.0 k | 123 | 15.0 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.90 | 78 | 15.89 | 18.29 | 55 | 34.3 | 2/18 | 78 | 2580 | 75.8 | 76 | 7.91 | 59.2 k | 31.6 k | 286 | 24.55 | 30.34 | 46 | 19.4 k | 125 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.08 | 76 | 18.54 | 26.37 | 38 | 50.3 | 8/14 | 72 | 2580 | 72.7 | 75 | 7.71 | 59.2 k | 31.5 k | 286 | 23.66 | 31.23 | 46 | 19.5 k | 125 | 17.1 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #1 | 11.97 | 84 | 16.53 | 18.59 | 54 | 36.1 | 2/21 | 70 | 2580 | 70.7 | 74 | 7.16 | 59.3 k | 31.5 k | 300 | 25 | 30.84 | 51 | 13.2 k | 106 | 11.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #2 | 12.08 | 83 | 16.46 | 18.96 | 53 | 37.0 | 3/19 | 70 | 2580 | 71.6 | 74 | 7.06 | 59.3 k | 31.5 k | 299 | 25.48 | 31.81 | 51 | 13.7 k | 104 | 11.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk_night 2560x1440 #3 | 12.28 | 81 | 17.53 | 20.29 | 49 | 35.7 | 3/18 | 72 | 2580 | 70.4 | 73 | 7.49 | 59.3 k | 31.7 k | 300 | 24.04 | 32.01 | 45 | 13.1 k | 104 | 10.9 k | 10 | 0 | 0 | 0 | 0 |
