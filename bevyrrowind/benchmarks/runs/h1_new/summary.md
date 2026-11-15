# Perf suite `h1_new`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 16.00 | 63 | 20.04 | 23.16 | 43 | 46.3 | 0/14, 1/15, 3/14 | 65 | 2580 | 64.4 | 71 | 8.13 | 59.2 k | 31.7 k | 286 | 21.03 | 26.37 | 22 | 67.5 k | 182 | 66.8 k | 21 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.75 | 85 | 13.55 | 14.65 | 68 | 30.5 | 1/25, 0/25, 1/24 | 98 | 2580 | 90.8 | 77 | 9.29 | 47.0 k | 24.6 k | 210 | 25.30 | 32.81 | 4 | 55.8 k | 134 | 56.2 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 ×3 | 10.85 | 92 | 12.51 | 13.73 | 73 | 33.3 | 0/28, 0/30, 2/27 | 97 | 2580 | 87.5 | 75 | 8.71 | 47.1 k | 24.7 k | 222 | 26.44 | 33.35 | 3 | 45.0 k | 120 | 45.4 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 16.32 | 61 | 20.15 | 23.16 | 43 | 40.3 | 0/14 | 64 | 2580 | 63.8 | 71 | 8.13 | 59.2 k | 31.7 k | 286 | 21.03 | 25.89 | 22 | 67.5 k | 189 | 66.8 k | 27 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 16.00 | 63 | 20.04 | 23.51 | 43 | 40.8 | 1/15 | 65 | 2580 | 64.4 | 71 | 8.13 | 59.2 k | 31.7 k | 286 | 21.63 | 32.30 | 22 | 67.5 k | 182 | 66.7 k | 21 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 15.17 | 66 | 18.45 | 21.48 | 47 | 46.3 | 3/14 | 70 | 2580 | 67.7 | 72 | 8.20 | 59.2 k | 31.6 k | 286 | 20.90 | 26.37 | 22 | 67.5 k | 181 | 66.8 k | 19 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.55 | 87 | 12.86 | 13.73 | 73 | 29.3 | 1/25 | 98 | 2579 | 91.1 | 77 | 9.29 | 47.0 k | 24.6 k | 210 | 25.30 | 32.44 | 4 | 55.8 k | 138 | 56.2 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.75 | 85 | 13.55 | 14.65 | 68 | 26.1 | 0/25 | 97 | 2580 | 90.6 | 78 | 9.14 | 47.0 k | 24.6 k | 210 | 25.11 | 33.19 | 4 | 55.8 k | 134 | 56.2 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.90 | 84 | 13.59 | 14.76 | 68 | 30.5 | 1/24 | 98 | 2580 | 90.8 | 75 | 9.56 | 47.0 k | 24.6 k | 210 | 25.71 | 32.81 | 4 | 55.8 k | 130 | 56.2 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #1 | 10.96 | 91 | 12.57 | 13.73 | 73 | 17.9 | 0/28 | 97 | 2580 | 87.5 | 73 | 8.88 | 47.1 k | 24.7 k | 222 | 27.31 | 33.91 | 3 | 45.0 k | 111 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #2 | 10.63 | 94 | 12.00 | 12.81 | 78 | 16.2 | 0/30 | 97 | 2580 | 91.4 | 76 | 8.71 | 47.1 k | 24.7 k | 222 | 26.44 | 33.35 | 3 | 45.0 k | 120 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 #3 | 10.85 | 92 | 12.51 | 13.95 | 72 | 33.3 | 2/27 | 94 | 2580 | 86.4 | 75 | 8.42 | 47.1 k | 24.7 k | 222 | 25.37 | 32.62 | 3 | 45.0 k | 120 | 45.4 k | 0 | 0 | 0 | 0 | 0 |
