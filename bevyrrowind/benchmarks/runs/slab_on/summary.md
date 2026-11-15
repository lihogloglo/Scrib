# Perf suite `slab_on`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.70 | 79 | 16.49 | 18.73 | 53 | 38.9 | 3/22, 2/20, 2/17 | 76 | 2580 | 75.5 | 76 | 7.95 | 59.2 k | 31.6 k | 286 | 23.56 | 29.59 | 22 | 18.1 k | 128 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.20 | 89 | 12.72 | 13.92 | 72 | 27.3 | 0/25, 0/30, 0/26 | 98 | 2580 | 92.9 | 79 | 9.12 | 47.0 k | 24.6 k | 210 | 31.73 | 36.22 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.50 | 80 | 16.49 | 18.86 | 53 | 38.9 | 3/22 | 76 | 2578 | 75.5 | 77 | 7.76 | 59.2 k | 31.6 k | 286 | 24.04 | 29.59 | 22 | 18.1 k | 125 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.27 | 75 | 16.54 | 18.73 | 53 | 35.3 | 2/20 | 75 | 2580 | 74.0 | 75 | 7.97 | 59.2 k | 31.6 k | 286 | 23.56 | 30.04 | 22 | 18.0 k | 128 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 12.70 | 79 | 15.98 | 18.09 | 55 | 33.7 | 2/17 | 79 | 2580 | 76.8 | 76 | 7.95 | 59.2 k | 31.6 k | 286 | 23.08 | 29.31 | 22 | 18.1 k | 129 | 17.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.20 | 89 | 12.61 | 13.56 | 74 | 25.1 | 0/25 | 98 | 2569 | 92.9 | 79 | 9.14 | 47.0 k | 24.6 k | 210 | 31.73 | 36.22 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.20 | 89 | 12.75 | 13.92 | 72 | 16.9 | 0/30 | 98 | 2580 | 94.9 | 79 | 8.97 | 47.0 k | 24.6 k | 210 | 33.02 | 39.59 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.15 | 90 | 12.72 | 14.34 | 70 | 27.3 | 0/26 | 97 | 2580 | 92.7 | 77 | 9.12 | 47.0 k | 24.6 k | 210 | 30.05 | 35.95 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
