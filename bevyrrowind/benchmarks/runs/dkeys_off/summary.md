# Perf suite `dkeys_off`

- exe: `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-off\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.51 | 74 | 17.13 | 19.10 | 52 | 39.7 | 3/20, 1/16, 1/17 | 70 | 2580 | 71.8 | 74 | 7.54 | 59.2 k | 31.6 k | 286 | 23.70 | 30.17 | 46 | 19.5 k | 125 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.99 | 91 | 13.02 | 14.38 | 70 | 39.4 | 2/29, 0/31, 0/29 | 96 | 2580 | 92.0 | 77 | 8.89 | 47.0 k | 24.6 k | 210 | 28.95 | 41.22 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 13.15 | 76 | 16.82 | 18.98 | 53 | 38.9 | 3/20 | 71 | 2580 | 71.9 | 74 | 7.50 | 59.2 k | 31.6 k | 286 | 24.82 | 29.17 | 46 | 18.9 k | 125 | 17.1 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.51 | 74 | 17.13 | 19.10 | 52 | 39.7 | 1/16 | 70 | 2580 | 71.8 | 74 | 7.54 | 59.2 k | 31.6 k | 286 | 23.56 | 35.37 | 46 | 19.6 k | 125 | 17.3 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.59 | 74 | 17.52 | 20.23 | 49 | 34.7 | 1/17 | 70 | 2580 | 71.1 | 73 | 7.57 | 59.2 k | 31.6 k | 286 | 23.70 | 30.17 | 46 | 19.5 k | 127 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 11.26 | 89 | 15.83 | 20.43 | 49 | 39.4 | 2/29 | 92 | 2580 | 89.2 | 76 | 8.89 | 47.0 k | 24.6 k | 210 | 28.95 | 43.55 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.99 | 91 | 13.02 | 14.38 | 70 | 20.7 | 0/31 | 96 | 2580 | 92.0 | 77 | 8.62 | 47.0 k | 24.6 k | 210 | 31.01 | 41.22 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.87 | 92 | 12.20 | 13.54 | 74 | 24.0 | 0/29 | 98 | 2580 | 93.9 | 78 | 8.91 | 47.0 k | 24.6 k | 210 | 24.58 | 33.02 | 4 | 14.6 k | 109 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
