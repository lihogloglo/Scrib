# Perf suite `m_farnpcs`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.17 | 109 | 10.18 | 10.81 | 93 | 27.2 | 1/7, 0/7, 0/5 | 97 | 2580 | 94.8 | 75 | 7.67 | 47.0 k | 24.6 k | 210 | 25 | 33.34 | 4 | 12.3 k | 54 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 9.66 | 103 | 12.10 | 13.65 | 73 | 35.3 | 2/6, 2/6, 2/6 | 87 | 2580 | 83.8 | 75 | 6.24 | 59.2 k | 31.8 k | 286 | 26.44 | 32.57 | 22 | 16.6 k | 76 | 15.6 k | 9 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.06 | 110 | 10.13 | 10.81 | 93 | 27.2 | 1/7 | 95 | 2580 | 93.8 | 73 | 7.45 | 47.0 k | 24.6 k | 210 | 28.00 | 33.34 | 4 | 12.3 k | 54 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.42 | 106 | 10.54 | 11.08 | 90 | 21.1 | 0/7 | 97 | 2580 | 94.9 | 75 | 7.96 | 47.0 k | 24.6 k | 210 | 22.82 | 32.24 | 4 | 12.3 k | 54 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.17 | 109 | 10.18 | 10.60 | 94 | 20.0 | 0/5 | 97 | 2580 | 94.8 | 75 | 7.67 | 47.0 k | 24.6 k | 210 | 25 | 33.63 | 4 | 12.3 k | 54 | 12.4 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 9.66 | 103 | 12.23 | 14.00 | 71 | 35.3 | 2/6 | 86 | 2580 | 83.8 | 75 | 6.03 | 59.2 k | 31.8 k | 286 | 26.44 | 32.57 | 22 | 16.6 k | 76 | 15.6 k | 9 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 9.57 | 105 | 12.10 | 13.65 | 73 | 25.9 | 2/6 | 89 | 2580 | 85.1 | 75 | 6.24 | 59.2 k | 31.8 k | 286 | 26.01 | 32.74 | 22 | 16.5 k | 76 | 15.7 k | 9 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 9.82 | 102 | 12.10 | 13.53 | 74 | 25.3 | 2/6 | 87 | 2580 | 83.7 | 73 | 6.25 | 59.2 k | 31.8 k | 286 | 26.92 | 32.15 | 22 | 16.6 k | 76 | 15.6 k | 10 | 0 | 0 | 0 | 0 |
