# Perf suite `slab_off`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.76 | 73 | 17.26 | 19.25 | 52 | 48.5 | 2/19, 3/15, 1/13 | 74 | 2580 | 72.8 | 73 | 8.06 | 59.2 k | 31.6 k | 286 | 22.38 | 29.04 | 22 | 18.2 k | 159 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.70 | 85 | 13.19 | 14.13 | 71 | 32.3 | 0/30, 0/21, 1/24 | 97 | 2580 | 90.7 | 74 | 9.54 | 47.0 k | 24.6 k | 210 | 25.59 | 31.68 | 4 | 14.6 k | 130 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 13.27 | 75 | 16.59 | 18.97 | 53 | 35.2 | 2/19 | 77 | 2580 | 72.9 | 73 | 8.06 | 59.2 k | 31.6 k | 286 | 22.97 | 29.04 | 22 | 18.2 k | 157 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.76 | 73 | 17.26 | 19.25 | 52 | 36.2 | 3/15 | 74 | 2580 | 72.8 | 74 | 8.12 | 59.2 k | 31.6 k | 286 | 22.38 | 28.63 | 22 | 17.9 k | 159 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 15.47 | 65 | 25.58 | 28.24 | 35 | 48.5 | 1/13 | 64 | 2580 | 64.9 | 73 | 8.06 | 59.6 k | 31.9 k | 286 | 21.15 | 31.57 | 28 | 19.0 k | 168 | 17.7 k | 18 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.70 | 85 | 17.67 | 19.74 | 51 | 26.9 | 0/30 | 88 | 2580 | 82.9 | 74 | 8.85 | 47.0 k | 24.6 k | 210 | 25.96 | 45.76 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.74 | 85 | 13.19 | 14.01 | 71 | 24.0 | 0/21 | 98 | 2580 | 90.7 | 73 | 9.54 | 47.0 k | 24.6 k | 210 | 24.29 | 27.14 | 4 | 14.6 k | 130 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.62 | 86 | 13.03 | 14.13 | 71 | 32.3 | 1/24 | 97 | 2580 | 91.0 | 74 | 9.54 | 47.0 k | 24.6 k | 210 | 25.59 | 31.68 | 4 | 14.6 k | 132 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
