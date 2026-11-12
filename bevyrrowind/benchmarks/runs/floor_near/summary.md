# Perf suite `floor_near`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 7.71 | 130 | 8.95 | 9.85 | 102 | 31.4 | 0/5, 2/4, 0/5 | 98 | 2580 | 96.3 | 77 | 7.24 | 8061 | 3200 | 0 | 22.12 | 31.70 | 4 | 1495 | 17 | 1595 | 3 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 7.66 | 131 | 8.89 | 9.66 | 103 | 18.3 | 0/5 | 98 | 2582 | 95.3 | 72 | 7.20 | 8061 | 3200 | 0 | 22.12 | 31.06 | 4 | 1494 | 17 | 1594 | 3 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 7.71 | 130 | 9.00 | 9.94 | 101 | 31.4 | 2/4 | 98 | 2580 | 97.5 | 78 | 7.24 | 8061 | 3200 | 0 | 23.67 | 34.88 | 4 | 1495 | 17 | 1595 | 3 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 7.75 | 129 | 8.95 | 9.85 | 102 | 23.8 | 0/5 | 97 | 2580 | 96.3 | 77 | 7.27 | 8061 | 3200 | 0 | 21.74 | 31.70 | 4 | 1505 | 17 | 1605 | 3 | 0 | 0 | 0 | 0 |
