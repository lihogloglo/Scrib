# Perf suite `dkeys_on`

- exe: `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-on\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.66 | 79 | 16.00 | 18.43 | 54 | 48.2 | 5/23, 3/20, 2/24 | 74 | 2580 | 74.5 | 74 | 7.46 | 59.2 k | 31.6 k | 286 | 24.19 | 30.13 | 46 | 19.5 k | 113 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.68 | 94 | 11.86 | 12.86 | 78 | 38.9 | 2/28, 1/25, 0/43 | 98 | 2580 | 93.8 | 77 | 8.87 | 47.0 k | 24.6 k | 210 | 26.57 | 32.00 | 4 | 14.6 k | 99 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.68 | 79 | 16.00 | 19.28 | 52 | 48.2 | 5/23 | 73 | 2580 | 74.5 | 74 | 7.42 | 59.2 k | 31.7 k | 286 | 24.19 | 31.69 | 46 | 19.5 k | 113 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.46 | 80 | 15.51 | 17.51 | 57 | 34.3 | 3/20 | 76 | 2580 | 75.7 | 75 | 7.46 | 59.2 k | 31.6 k | 286 | 24.31 | 30.13 | 46 | 19.5 k | 113 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 12.66 | 79 | 16.27 | 18.43 | 54 | 35.0 | 2/24 | 74 | 2580 | 74.3 | 74 | 7.49 | 59.2 k | 31.6 k | 286 | 24.04 | 29.35 | 46 | 19.5 k | 115 | 17.3 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.70 | 93 | 13.13 | 20.82 | 48 | 38.9 | 2/28 | 98 | 2580 | 92.9 | 77 | 9.10 | 47.0 k | 24.6 k | 210 | 26.57 | 32.00 | 4 | 14.6 k | 99 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.68 | 94 | 11.86 | 12.86 | 78 | 27.0 | 1/25 | 98 | 2580 | 93.8 | 76 | 8.87 | 47.0 k | 24.6 k | 210 | 27.14 | 32.48 | 4 | 14.6 k | 99 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.67 | 94 | 11.73 | 12.32 | 81 | 17.1 | 0/43 | 98 | 2580 | 94.5 | 77 | 8.84 | 47.0 k | 24.6 k | 210 | 26.44 | 30.68 | 4 | 14.6 k | 99 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
