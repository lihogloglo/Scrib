# Perf suite `push_nocull`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: cpuculltag · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=cpuculltag ×3 | 9.98 | 100 | 11.08 | 11.62 | 86 | 20.8 | 0/3, 0/5, 0/9 | 97 | 2580 | 91.9 | 71 | 9.11 | 47.0 k | 24.6 k | 210 | 24.94 | 30.97 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cpuculltag ×3 | 10.32 | 97 | 12.54 | 13.94 | 72 | 31.8 | 0/7, 5/8, 2/5 | 85 | 2580 | 80.8 | 73 | 7.81 | 59.4 k | 31.8 k | 286 | 26.46 | 33.86 | 22 | 18.3 k | 86 | 17.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=cpuculltag ×3 | 9.03 | 111 | 10.14 | 11.08 | 90 | 34.2 | 0/8, 0/11, 1/7 | 98 | 2580 | 93.0 | 75 | 8.47 | 47.1 k | 24.7 k | 222 | 25.23 | 32.02 | 3 | 11.3 k | 60 | 11.5 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=cpuculltag #1 | 9.98 | 100 | 11.08 | 11.62 | 86 | 20.8 | 0/3 | 98 | 2580 | 92.7 | 74 | 9.11 | 47.0 k | 24.6 k | 210 | 22.96 | 30.97 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cpuculltag #2 | 10.54 | 95 | 11.74 | 12.29 | 81 | 18.9 | 0/5 | 97 | 2581 | 88.5 | 70 | 9.54 | 47.0 k | 24.6 k | 210 | 24.94 | 38.71 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cpuculltag #3 | 9.82 | 102 | 10.92 | 11.59 | 86 | 20.8 | 0/9 | 97 | 2580 | 91.9 | 71 | 8.90 | 47.0 k | 24.6 k | 210 | 25 | 30.34 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cpuculltag #1 | 10.32 | 97 | 12.54 | 13.94 | 72 | 23.7 | 0/7 | 85 | 2580 | 80.8 | 73 | 7.69 | 59.2 k | 31.8 k | 286 | 25.84 | 31.70 | 22 | 18.2 k | 86 | 17.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cpuculltag #2 | 10.88 | 92 | 14.14 | 20.82 | 48 | 31.8 | 5/8 | 84 | 2580 | 78.7 | 73 | 8.17 | 59.4 k | 31.8 k | 286 | 26.46 | 39.97 | 22 | 18.3 k | 84 | 17.3 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cpuculltag #3 | 9.98 | 100 | 12.17 | 13.53 | 74 | 30.9 | 2/5 | 89 | 2580 | 83.4 | 73 | 7.81 | 59.4 k | 31.8 k | 286 | 27.66 | 33.86 | 22 | 18.4 k | 87 | 17.6 k | 10 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=cpuculltag #1 | 8.99 | 111 | 9.98 | 10.57 | 95 | 13.9 | 0/8 | 98 | 2580 | 93.3 | 75 | 8.45 | 47.1 k | 24.7 k | 222 | 24.40 | 30.36 | 3 | 11.3 k | 60 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=cpuculltag #2 | 9.35 | 107 | 11.09 | 18.99 | 53 | 24.9 | 0/11 | 96 | 2580 | 90.2 | 75 | 8.85 | 47.1 k | 24.7 k | 222 | 26.08 | 42.35 | 3 | 11.3 k | 60 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=cpuculltag #3 | 9.03 | 111 | 10.14 | 11.08 | 90 | 34.2 | 1/7 | 98 | 2580 | 93.0 | 72 | 8.47 | 47.1 k | 24.7 k | 222 | 25.23 | 32.02 | 3 | 11.3 k | 60 | 11.5 k | 0 | 0 | 0 | 0 | 0 |
