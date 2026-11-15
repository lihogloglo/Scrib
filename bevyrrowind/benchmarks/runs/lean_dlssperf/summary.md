# Perf suite `lean_dlssperf`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.81 | 72 | 17.82 | 20.24 | 49 | 36.8 | 1/15, 1/16, 2/14 | 75 | 2576 | 77.4 | 76 | 8.32 | 59.2 k | 31.6 k | 286 | 22.71 | 29.25 | 22 | 18.0 k | 164 | 17.1 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.64 | 86 | 13.11 | 14.36 | 70 | 23.6 | 0/26, 0/21, 0/29 | 97 | 2573 | 97.5 | 79 | 9.53 | 47.0 k | 24.6 k | 210 | 24.25 | 31.28 | 4 | 14.6 k | 129 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 13.81 | 72 | 18.07 | 20.87 | 48 | 35.2 | 1/15 | 75 | 2578 | 77.4 | 75 | 8.28 | 59.2 k | 31.6 k | 286 | 22.60 | 29.78 | 22 | 17.7 k | 160 | 17.1 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.76 | 73 | 17.70 | 20.16 | 50 | 34.7 | 1/16 | 75 | 2576 | 78.3 | 76 | 8.32 | 59.2 k | 31.6 k | 286 | 22.71 | 28.81 | 22 | 18.0 k | 164 | 17.1 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 14.13 | 71 | 17.82 | 20.24 | 49 | 36.8 | 2/14 | 74 | 2575 | 76.8 | 76 | 8.34 | 59.2 k | 31.7 k | 286 | 23.08 | 29.25 | 22 | 18.4 k | 165 | 17.5 k | 14 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.64 | 86 | 13.22 | 14.75 | 68 | 23.6 | 0/26 | 97 | 2573 | 97.5 | 79 | 9.45 | 47.0 k | 24.6 k | 210 | 24.19 | 31.28 | 4 | 14.6 k | 129 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.64 | 86 | 13.11 | 14.36 | 70 | 21.9 | 0/21 | 97 | 2580 | 98.2 | 79 | 9.53 | 47.0 k | 24.6 k | 210 | 25.06 | 31.43 | 4 | 14.6 k | 131 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.63 | 86 | 12.97 | 14.15 | 71 | 18.9 | 0/29 | 98 | 2570 | 97.3 | 79 | 9.61 | 47.0 k | 24.6 k | 210 | 24.25 | 30.38 | 4 | 14.6 k | 129 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
