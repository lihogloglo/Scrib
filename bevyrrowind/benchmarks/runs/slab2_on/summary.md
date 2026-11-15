# Perf suite `slab2_on`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.70 | 79 | 16.63 | 19.34 | 52 | 233.6 | 3/20, 486/174, 3/17 | 75 | 2580 | 73.8 | 73 | 7.91 | 59.2 k | 31.6 k | 286 | 23.08 | 29.13 | 22 | 18.0 k | 126 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.46 | 87 | 13.11 | 14.36 | 70 | 84.9 | 1/25, 0/23, 131/165 | 98 | 2580 | 92.6 | 78 | 9.50 | 47.0 k | 24.6 k | 210 | 24.76 | 29.88 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.70 | 79 | 16.63 | 19.34 | 52 | 35.7 | 3/20 | 75 | 2580 | 74.1 | 73 | 7.84 | 59.2 k | 31.6 k | 286 | 23.08 | 29.54 | 22 | 18.0 k | 126 | 17.1 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.19 | 82 | 53.89 | 88.54 | 11 | 233.6 | 486/174 | 87 | 2580 | 67.7 | 73 | 12.14 | 59.2 k | 30.7 k | 289 | 21.63 | 29.13 | 4 | 13.7 k | 120 | 14.0 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.12 | 76 | 16.25 | 18.45 | 54 | 36.1 | 3/17 | 75 | 2580 | 73.8 | 76 | 7.91 | 59.2 k | 31.6 k | 286 | 23.49 | 29.00 | 22 | 18.1 k | 128 | 17.3 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.04 | 91 | 12.43 | 13.43 | 74 | 31.4 | 1/25 | 96 | 2580 | 92.6 | 78 | 9.03 | 47.0 k | 24.6 k | 210 | 29.86 | 35.49 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.46 | 87 | 13.11 | 14.36 | 70 | 18.2 | 0/23 | 98 | 2580 | 93.6 | 78 | 9.50 | 47.0 k | 24.6 k | 210 | 24.76 | 29.88 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.71 | 85 | 74.10 | 82.93 | 12 | 84.9 | 131/165 | 98 | 2580 | 68.4 | 73 | 16.87 | 47.0 k | 24.6 k | 210 | 18.01 | 28.30 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
