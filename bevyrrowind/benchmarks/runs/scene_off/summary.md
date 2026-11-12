# Perf suite `scene_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.72 | 85 | 16.20 | 19.11 | 52 | 108.4 | 1/5, 0/5, 1/6 | 77 | 2581 | 73.0 | 68 | 7.85 | 59.2 k | 31.7 k | 286 | 24.04 | 38.05 | 22 | 18.1 k | 83 | 17.2 k | 187 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 9.26 | 108 | 11.39 | 13.63 | 73 | 27.9 | 1/12, 0/5, 0/5 | 93 | 2581 | 90.3 | 72 | 8.23 | 47.0 k | 24.6 k | 210 | 24.55 | 35.62 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.72 | 85 | 16.57 | 19.92 | 50 | 51.3 | 1/5 | 70 | 2584 | 70.4 | 65 | 7.22 | 59.2 k | 31.7 k | 286 | 24.04 | 38.05 | 22 | 18.3 k | 82 | 17.3 k | 184 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.26 | 82 | 16.20 | 19.11 | 52 | 28.3 | 0/5 | 77 | 2580 | 73.0 | 68 | 8.09 | 59.2 k | 31.7 k | 286 | 24.17 | 38.80 | 22 | 18.1 k | 84 | 17.2 k | 190 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.36 | 88 | 15.64 | 18.63 | 54 | 108.4 | 1/6 | 79 | 2581 | 74.6 | 69 | 7.85 | 59.2 k | 31.5 k | 286 | 23.56 | 36.41 | 22 | 17.2 k | 83 | 17.0 k | 187 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 9.26 | 108 | 10.96 | 12.50 | 80 | 27.9 | 1/12 | 96 | 2580 | 91.9 | 72 | 8.23 | 47.0 k | 24.6 k | 210 | 26.44 | 35.62 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.26 | 108 | 11.39 | 13.63 | 73 | 18.5 | 0/5 | 93 | 2581 | 90.0 | 72 | 8.16 | 47.0 k | 24.6 k | 210 | 24.52 | 33.93 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.37 | 107 | 12.39 | 14.35 | 70 | 21.0 | 0/5 | 93 | 2581 | 90.3 | 71 | 8.30 | 47.0 k | 24.6 k | 210 | 24.55 | 37.40 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
