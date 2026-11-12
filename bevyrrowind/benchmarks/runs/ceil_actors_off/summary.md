# Perf suite `ceil_actors_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: actors · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=actors ×3 | 9.13 | 110 | 10.52 | 11.55 | 87 | 24.9 | 0/4, 0/5, 0/3 | 97 | 2582 | 92.4 | 68 | 8.34 | 27.7 k | 21.7 k | 210 | 25.35 | 37.40 | 4 | 12.2 k | 50 | 12.4 k | 120 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=actors ×3 | 8.90 | 112 | 11.54 | 13.43 | 74 | 34.2 | 4/7, 2/4, 0/9 | 91 | 2580 | 87.3 | 74 | 7.27 | 35.1 k | 28.0 k | 286 | 27.75 | 40.78 | 22 | 16.0 k | 56 | 14.9 k | 132 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=actors #1 | 9.17 | 109 | 10.75 | 12.99 | 77 | 24.9 | 0/4 | 97 | 2586 | 92.0 | 64 | 8.35 | 27.7 k | 21.7 k | 210 | 25.85 | 40.25 | 4 | 12.2 k | 50 | 12.4 k | 120 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=actors #2 | 9.13 | 110 | 10.52 | 11.55 | 87 | 22.8 | 0/5 | 97 | 2582 | 92.4 | 68 | 8.34 | 27.7 k | 21.7 k | 210 | 25.35 | 37.40 | 4 | 12.2 k | 50 | 12.4 k | 120 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=actors #3 | 9.02 | 111 | 9.98 | 10.56 | 95 | 12.8 | 0/3 | 98 | 2580 | 94.9 | 70 | 8.24 | 27.7 k | 21.7 k | 210 | 22.52 | 29.48 | 4 | 12.2 k | 50 | 12.4 k | 120 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=actors #1 | 8.88 | 113 | 11.73 | 13.82 | 72 | 34.2 | 4/7 | 90 | 2580 | 86.5 | 74 | 7.21 | 35.4 k | 28.1 k | 286 | 26.92 | 39.61 | 28 | 16.4 k | 57 | 15.0 k | 134 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=actors #2 | 8.93 | 112 | 11.25 | 13.08 | 76 | 30.7 | 2/4 | 91 | 2580 | 87.3 | 74 | 7.27 | 35.1 k | 28.0 k | 286 | 27.88 | 41.22 | 22 | 16.0 k | 56 | 14.9 k | 132 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=actors #3 | 8.90 | 112 | 11.54 | 13.43 | 74 | 23.2 | 0/9 | 93 | 2580 | 88.8 | 75 | 7.41 | 35.0 k | 28.0 k | 286 | 27.75 | 40.78 | 22 | 16.0 k | 56 | 14.9 k | 132 | 0 | 0 | 0 | 0 |
