# Perf suite `h1_new_nomaps`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax ×3 | 11.27 | 89 | 14.21 | 17.12 | 58 | 129.2 | 5/32, 6/29, 351/166 | 86 | 2574 | 81.6 | 77 | 7.20 | 59.2 k | 31.7 k | 286 | 23.32 | 29.61 | 22 | 67.6 k | 111 | 66.8 k | 11 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #1 | 11.27 | 89 | 13.97 | 17.12 | 58 | 49.5 | 5/32 | 86 | 2574 | 81.8 | 77 | 7.20 | 59.4 k | 31.8 k | 286 | 24.87 | 29.88 | 22 | 67.5 k | 111 | 66.7 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #2 | 11.18 | 89 | 14.21 | 17.05 | 59 | 53.9 | 6/29 | 86 | 2574 | 81.6 | 78 | 7.17 | 59.2 k | 31.7 k | 286 | 23.32 | 29.61 | 22 | 67.6 k | 112 | 66.8 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #3 | 11.39 | 88 | 45.41 | 86.86 | 12 | 129.2 | 351/166 | 90 | 2576 | 71.0 | 75 | 10.33 | 59.2 k | 31.0 k | 289 | 22.12 | 29.36 | 4 | 68.0 k | 111 | 67.8 k | 11 | 0 | 0 | 0 | 0 |
