# Perf suite `h1_old_nomaps`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax ×3 | 11.22 | 89 | 40.67 | 82.22 | 12 | 141.0 | 7/25, 0/3, 390/161 | 91 | 2580 | 73.3 | 74 | 10.01 | 59.2 k | 31.6 k | 286 | 23.08 | 31.74 | 22 | 18.3 k | 111 | 17.4 k | 10 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #1 | 11.22 | 89 | 13.94 | 16.30 | 61 | 36.8 | 7/25 | 85 | 2580 | 80.6 | 75 | 7.14 | 59.2 k | 31.8 k | 286 | 25.96 | 31.74 | 22 | 18.3 k | 111 | 17.4 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #2 | 67.93 | 15 | 81.63 | 93.76 | 11 | 141.0 | 0/3 | 98 | 2580 | 46.6 | 67 | 48.80 | 59.2 k | 31.6 k | 286 | 11.61 | 28.94 | 22 | 18.5 k | 119 | 17.7 k | 44 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #3 | 11.12 | 90 | 40.67 | 82.22 | 12 | 122.1 | 390/161 | 91 | 2578 | 73.3 | 74 | 10.01 | 59.2 k | 31.2 k | 289 | 23.08 | 32.10 | 4 | 13.8 k | 103 | 14.0 k | 10 | 0 | 0 | 0 | 0 |
