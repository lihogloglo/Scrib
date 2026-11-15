# Perf suite `lean_base`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 14.15 | 71 | 18.29 | 21.87 | 46 | 53.2 | 3/19, 4/17, 7/15 | 72 | 2580 | 70.0 | 72 | 8.05 | 59.2 k | 31.6 k | 286 | 23.00 | 30.62 | 22 | 17.9 k | 157 | 17.2 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.41 | 88 | 13.21 | 14.27 | 70 | 30.1 | 0/22, 1/28, 0/28 | 96 | 2580 | 90.6 | 74 | 9.09 | 47.0 k | 24.6 k | 210 | 28.85 | 36.62 | 4 | 14.6 k | 130 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 14.15 | 71 | 18.29 | 22.37 | 45 | 53.2 | 3/19 | 72 | 2580 | 70.0 | 71 | 8.05 | 59.2 k | 31.6 k | 286 | 24.07 | 37.03 | 22 | 17.9 k | 155 | 17.2 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 14.87 | 67 | 19.16 | 21.87 | 46 | 39.7 | 4/17 | 69 | 2580 | 68.6 | 72 | 8.12 | 59.2 k | 31.6 k | 286 | 22.77 | 30.62 | 22 | 18.1 k | 163 | 17.3 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.05 | 77 | 16.60 | 21.29 | 47 | 36.9 | 7/15 | 79 | 2580 | 74.2 | 72 | 7.96 | 59.2 k | 31.6 k | 286 | 23.00 | 29.61 | 22 | 17.9 k | 157 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.41 | 88 | 13.43 | 14.78 | 68 | 26.4 | 0/22 | 97 | 2580 | 90.2 | 74 | 9.17 | 47.0 k | 24.6 k | 210 | 28.85 | 36.62 | 4 | 14.6 k | 130 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.46 | 87 | 13.21 | 14.27 | 70 | 30.1 | 1/28 | 96 | 2580 | 90.6 | 74 | 9.09 | 47.0 k | 24.6 k | 210 | 29.81 | 39.00 | 4 | 14.6 k | 129 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.33 | 88 | 13.06 | 13.81 | 72 | 16.9 | 0/28 | 96 | 2580 | 91.4 | 76 | 8.83 | 47.0 k | 24.6 k | 210 | 26.92 | 35.90 | 4 | 14.6 k | 131 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
