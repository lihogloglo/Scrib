# Perf suite `h1_old`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.01 | 77 | 17.25 | 19.76 | 51 | 50.3 | 2/23, 0/17, 3/16 | 69 | 2580 | 69.6 | 73 | 7.59 | 59.2 k | 31.6 k | 286 | 23.08 | 28.85 | 22 | 18.1 k | 153 | 17.1 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.00 | 91 | 13.05 | 14.14 | 71 | 53.7 | 1/27, 1/32, 0/21 | 96 | 2580 | 91.6 | 77 | 8.67 | 47.0 k | 24.6 k | 210 | 28.77 | 35.77 | 4 | 14.6 k | 124 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 ×3 | 10.73 | 93 | 12.28 | 13.28 | 75 | 39.2 | 3/37, 0/29, 0/25 | 97 | 2580 | 90.5 | 74 | 8.85 | 47.1 k | 24.7 k | 222 | 30.29 | 34.40 | 3 | 11.3 k | 111 | 11.5 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.97 | 77 | 17.25 | 19.76 | 51 | 50.3 | 2/23 | 69 | 2580 | 69.6 | 73 | 7.23 | 58.7 k | 30.9 k | 286 | 22.97 | 28.54 | 22 | 17.5 k | 148 | 16.5 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.92 | 72 | 18.29 | 21.12 | 47 | 34.4 | 0/17 | 68 | 2580 | 69.1 | 73 | 7.59 | 59.2 k | 31.6 k | 286 | 23.08 | 28.85 | 22 | 18.2 k | 163 | 17.3 k | 15 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.01 | 77 | 16.86 | 19.27 | 52 | 37.4 | 3/16 | 73 | 2580 | 72.0 | 74 | 7.63 | 59.2 k | 31.6 k | 286 | 23.56 | 28.97 | 22 | 18.1 k | 153 | 17.1 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.00 | 91 | 13.05 | 14.43 | 69 | 44.6 | 1/27 | 94 | 2577 | 91.6 | 77 | 8.41 | 47.0 k | 24.6 k | 210 | 28.77 | 35.77 | 4 | 14.6 k | 123 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.94 | 91 | 12.46 | 13.58 | 74 | 53.7 | 1/32 | 96 | 2580 | 92.7 | 77 | 8.67 | 47.0 k | 24.6 k | 210 | 29.26 | 36.16 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.51 | 87 | 13.20 | 14.14 | 71 | 22.3 | 0/21 | 97 | 2580 | 91.6 | 76 | 9.10 | 47.0 k | 24.6 k | 210 | 27.42 | 34.13 | 4 | 14.6 k | 124 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #1 | 10.50 | 95 | 12.34 | 14.14 | 71 | 39.2 | 3/37 | 95 | 2576 | 90.7 | 77 | 8.22 | 47.1 k | 24.7 k | 222 | 30.29 | 43.80 | 3 | 11.3 k | 109 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #2 | 10.73 | 93 | 12.16 | 13.28 | 75 | 23.1 | 0/29 | 97 | 2581 | 88.1 | 70 | 8.85 | 47.1 k | 24.7 k | 222 | 28.84 | 34.32 | 3 | 11.3 k | 111 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #3 | 10.90 | 92 | 12.28 | 13.01 | 77 | 19.3 | 0/25 | 98 | 2580 | 90.5 | 74 | 9.20 | 47.1 k | 24.7 k | 222 | 30.80 | 34.40 | 3 | 11.3 k | 111 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
