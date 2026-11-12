# Perf suite `m_actors`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.09 | 110 | 10.04 | 10.61 | 94 | 22.1 | 0/4, 0/4, 0/4 | 97 | 2580 | 94.8 | 75 | 7.57 | 27.7 k | 21.7 k | 210 | 21.87 | 29.50 | 4 | 12.2 k | 50 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 8.51 | 118 | 10.11 | 11.46 | 87 | 40.1 | 0/6, 1/6, 1/6 | 95 | 2574 | 93.2 | 79 | 6.14 | 35.1 k | 28.0 k | 286 | 31.08 | 38.44 | 28 | 16.3 k | 56 | 14.9 k | 8 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.09 | 110 | 10.00 | 10.46 | 96 | 22.1 | 0/4 | 97 | 2580 | 94.8 | 73 | 7.59 | 27.7 k | 21.7 k | 210 | 22.19 | 29.50 | 4 | 12.2 k | 50 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.04 | 111 | 10.04 | 10.61 | 94 | 15.0 | 0/4 | 96 | 2580 | 94.8 | 75 | 7.39 | 27.7 k | 21.7 k | 210 | 21.06 | 28.69 | 4 | 12.2 k | 50 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.12 | 110 | 10.12 | 10.66 | 94 | 19.8 | 0/4 | 97 | 2580 | 94.7 | 75 | 7.57 | 27.7 k | 21.7 k | 210 | 21.87 | 30.09 | 4 | 12.2 k | 50 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 8.54 | 117 | 10.11 | 11.46 | 87 | 25.0 | 0/6 | 95 | 2580 | 92.4 | 77 | 6.14 | 35.1 k | 28.0 k | 286 | 31.08 | 38.70 | 28 | 16.3 k | 56 | 14.9 k | 8 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 8.51 | 118 | 10.04 | 11.35 | 88 | 30.3 | 1/6 | 96 | 2571 | 93.7 | 79 | 6.14 | 35.1 k | 28.0 k | 286 | 30.29 | 37.00 | 28 | 16.3 k | 56 | 14.9 k | 8 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 8.50 | 118 | 10.21 | 11.82 | 85 | 40.1 | 1/6 | 95 | 2574 | 93.2 | 79 | 6.21 | 35.1 k | 28.0 k | 286 | 31.26 | 38.44 | 22 | 16.1 k | 56 | 14.9 k | 8 | 0 | 0 | 0 | 0 |
