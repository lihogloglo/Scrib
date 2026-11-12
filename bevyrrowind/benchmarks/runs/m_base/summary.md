# Perf suite `m_base`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.42 | 106 | 10.50 | 11.17 | 90 | 25.5 | 0/8, 1/9, 0/6 | 97 | 2581 | 91.6 | 69 | 7.84 | 47.0 k | 24.6 k | 210 | 27.05 | 34.01 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 9.96 | 100 | 12.10 | 13.63 | 73 | 35.3 | 3/5, 2/5, 2/7 | 87 | 2580 | 83.4 | 75 | 6.25 | 59.2 k | 31.8 k | 286 | 26.92 | 32.84 | 22 | 18.2 k | 84 | 17.4 k | 9 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.45 | 106 | 10.62 | 11.17 | 90 | 22.5 | 0/8 | 98 | 2582 | 91.6 | 67 | 7.89 | 47.0 k | 24.6 k | 210 | 29.83 | 36.36 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.41 | 106 | 10.50 | 11.21 | 89 | 25.5 | 1/9 | 96 | 2581 | 90.5 | 69 | 7.64 | 47.0 k | 24.6 k | 210 | 27.05 | 34.01 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.42 | 106 | 10.49 | 11.06 | 90 | 19.4 | 0/6 | 97 | 2580 | 93.1 | 73 | 7.84 | 47.0 k | 24.6 k | 210 | 26.13 | 33.07 | 4 | 14.6 k | 67 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 9.94 | 101 | 12.10 | 13.57 | 74 | 31.6 | 3/5 | 87 | 2580 | 83.7 | 74 | 6.25 | 59.4 k | 31.8 k | 286 | 26.38 | 33.55 | 22 | 18.5 k | 84 | 17.5 k | 9 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 9.96 | 100 | 12.58 | 13.99 | 71 | 27.5 | 2/5 | 87 | 2580 | 83.3 | 75 | 6.25 | 59.2 k | 31.7 k | 286 | 26.92 | 32.84 | 22 | 18.2 k | 86 | 17.4 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 10.02 | 100 | 12.07 | 13.63 | 73 | 35.3 | 2/7 | 86 | 2580 | 83.4 | 75 | 6.32 | 59.2 k | 31.8 k | 286 | 26.92 | 32.84 | 22 | 18.2 k | 84 | 17.3 k | 9 | 0 | 0 | 0 | 0 |
