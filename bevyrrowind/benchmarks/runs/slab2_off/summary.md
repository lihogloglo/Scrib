# Perf suite `slab2_off`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.40 | 75 | 17.32 | 19.36 | 52 | 137.1 | 2/21, 4/15, 162/167 | 75 | 2580 | 72.0 | 74 | 8.12 | 59.2 k | 31.6 k | 286 | 22.45 | 28.84 | 22 | 18.0 k | 157 | 17.0 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 11.29 | 89 | 12.68 | 13.74 | 73 | 59.4 | 0/26, 0/28, 0/5 | 98 | 2580 | 90.6 | 77 | 9.37 | 47.0 k | 24.6 k | 210 | 22.60 | 27.84 | 4 | 14.6 k | 128 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 13.67 | 73 | 17.32 | 19.36 | 52 | 37.2 | 2/21 | 74 | 2580 | 72.0 | 74 | 8.12 | 59.2 k | 31.6 k | 286 | 22.45 | 28.80 | 22 | 18.1 k | 162 | 17.3 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.40 | 75 | 16.43 | 18.85 | 53 | 49.0 | 4/15 | 75 | 2580 | 74.0 | 75 | 7.96 | 59.2 k | 31.6 k | 286 | 23.22 | 29.06 | 22 | 18.0 k | 157 | 17.0 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.18 | 76 | 31.18 | 70.40 | 14 | 137.1 | 162/167 | 83 | 2580 | 70.5 | 74 | 10.04 | 59.2 k | 31.4 k | 289 | 21.63 | 28.84 | 4 | 14.9 k | 147 | 15.0 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.29 | 89 | 12.68 | 13.74 | 73 | 16.3 | 0/26 | 98 | 2580 | 93.0 | 78 | 9.37 | 47.0 k | 24.6 k | 210 | 24.52 | 30.53 | 4 | 14.6 k | 128 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.14 | 90 | 12.49 | 13.54 | 74 | 25.1 | 0/28 | 96 | 2580 | 90.6 | 77 | 9.07 | 47.0 k | 24.6 k | 210 | 22.60 | 27.84 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 51.70 | 19 | 55.83 | 57.48 | 17 | 59.4 | 0/5 | 99 | 2580 | 53.1 | 69 | 37.10 | 47.0 k | 24.6 k | 210 | 12.71 | 27.03 | 4 | 14.6 k | 128 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
