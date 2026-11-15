# Perf suite `ultra2`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.61 | 79 | 17.77 | 23.32 | 43 | 121.0 | 18/20, 10/23, 11/18 | 78 | 2580 | 75.9 | 74 | 7.89 | 59.3 k | 31.7 k | 286 | 24.88 | 32.94 | 47 | 23.4 k | 100 | 21.4 k | 13 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.61 | 79 | 18.92 | 25.90 | 39 | 121.0 | 18/20 | 83 | 2580 | 75.9 | 73 | 8.38 | 59.3 k | 31.5 k | 286 | 25.36 | 32.94 | 47 | 22.9 k | 97 | 20.6 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.59 | 79 | 16.66 | 20.00 | 50 | 100.8 | 10/23 | 78 | 2580 | 76.9 | 74 | 7.68 | 59.3 k | 31.7 k | 286 | 24.88 | 32.37 | 47 | 23.4 k | 100 | 21.4 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 13.26 | 75 | 17.77 | 23.32 | 43 | 80.3 | 11/18 | 76 | 2580 | 75.0 | 74 | 7.89 | 59.3 k | 31.7 k | 286 | 24.52 | 35.33 | 47 | 23.6 k | 100 | 21.5 k | 13 | 0 | 0 | 0 | 0 |
