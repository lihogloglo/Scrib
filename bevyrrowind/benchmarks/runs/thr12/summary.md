# Perf suite `thr12`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 12.59 | 79 | 17.39 | 21.01 | 48 | 73.3 | 4/19, 5/21, 4/23 | 78 | 2580 | 74.9 | 74 | 7.80 | 59.3 k | 31.7 k | 286 | 26.44 | 40.09 | 46 | 19.5 k | 85 | 17.2 k | 10 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.59 | 79 | 17.39 | 21.01 | 48 | 36.1 | 4/19 | 78 | 2581 | 73.6 | 71 | 7.80 | 59.3 k | 31.7 k | 286 | 25.48 | 39.68 | 46 | 19.3 k | 84 | 17.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 12.47 | 80 | 15.59 | 18.62 | 54 | 41.4 | 5/21 | 82 | 2580 | 76.0 | 74 | 7.91 | 59.3 k | 31.7 k | 286 | 26.44 | 40.09 | 46 | 19.5 k | 85 | 17.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 12.81 | 78 | 17.44 | 21.11 | 47 | 73.3 | 4/23 | 77 | 2580 | 74.9 | 74 | 7.66 | 59.3 k | 31.7 k | 286 | 27.40 | 42.16 | 46 | 19.6 k | 85 | 17.5 k | 11 | 0 | 0 | 0 | 0 |
