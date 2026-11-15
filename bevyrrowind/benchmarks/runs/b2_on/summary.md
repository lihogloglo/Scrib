# Perf suite `b2_on`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.27 | 82 | 15.44 | 18.66 | 54 | 74.3 | 4/38, 2/23, 4/23 | 77 | 2580 | 76.2 | 73 | 7.54 | 59.3 k | 31.7 k | 286 | 25.82 | 35.18 | 46 | 19.6 k | 85 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.96 | 91 | 12.66 | 13.85 | 72 | 18.7 | 0/22, 0/32, 0/25 | 98 | 2580 | 93.1 | 76 | 9.09 | 47.1 k | 24.7 k | 210 | 29.50 | 39.83 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.67 | 79 | 16.60 | 19.50 | 51 | 74.3 | 4/38 | 77 | 2580 | 75.7 | 73 | 7.54 | 59.3 k | 31.7 k | 286 | 25.96 | 39.36 | 46 | 19.6 k | 87 | 17.6 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.03 | 83 | 15.34 | 18.66 | 54 | 33.0 | 2/23 | 78 | 2580 | 76.7 | 73 | 7.42 | 59.3 k | 31.7 k | 286 | 24.57 | 34.16 | 46 | 19.7 k | 85 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 12.27 | 82 | 15.44 | 18.28 | 55 | 35.3 | 4/23 | 77 | 2580 | 76.2 | 73 | 7.55 | 59.3 k | 31.7 k | 286 | 25.82 | 35.18 | 46 | 19.6 k | 85 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.96 | 91 | 12.66 | 13.85 | 72 | 18.7 | 0/22 | 98 | 2580 | 93.1 | 76 | 9.09 | 47.1 k | 24.7 k | 210 | 29.86 | 39.83 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.10 | 90 | 12.85 | 13.90 | 72 | 16.2 | 0/32 | 98 | 2580 | 92.1 | 76 | 9.17 | 47.1 k | 24.7 k | 210 | 29.50 | 45.02 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.81 | 93 | 12.06 | 12.70 | 79 | 13.7 | 0/25 | 97 | 2580 | 93.6 | 76 | 9.09 | 47.1 k | 24.7 k | 210 | 29.33 | 38.78 | 4 | 14.6 k | 73 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
