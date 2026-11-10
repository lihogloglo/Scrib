# Perf suite `au_night_lamp0`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk_night` balmora_walk at 22:00, clear: walkers carry lit torches (flame emitters), lamps light the street
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 ×3 | 9.49 | 105 | 10.93 | 12.06 | 83 | 22.9 | 0/4, 0/6, 0/7 | 77 | 2580 | 78.8 | 73 | 6.64 | 59.5 k | 31.9 k | 300 | 24.52 | 33.15 | 3 | 11.6 k | 66 | 11.7 k | 151 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk_night 2560x1440 #1 | 9.49 | 105 | 10.97 | 12.31 | 81 | 21.6 | 0/4 | 77 | 2580 | 78.8 | 73 | 6.64 | 59.6 k | 31.9 k | 300 | 24.52 | 33.91 | 3 | 11.6 k | 66 | 11.8 k | 151 |
| balmora_walk_night 2560x1440 #2 | 9.39 | 106 | 10.45 | 11.33 | 88 | 19.5 | 0/6 | 78 | 2580 | 78.8 | 73 | 6.62 | 59.5 k | 31.9 k | 300 | 24.55 | 31.41 | 3 | 11.6 k | 66 | 11.7 k | 151 |
| balmora_walk_night 2560x1440 #3 | 9.67 | 103 | 10.93 | 12.06 | 83 | 22.9 | 0/7 | 77 | 2580 | 77.4 | 73 | 6.69 | 59.5 k | 31.9 k | 300 | 24.04 | 33.15 | 3 | 11.5 k | 66 | 11.7 k | 151 |
