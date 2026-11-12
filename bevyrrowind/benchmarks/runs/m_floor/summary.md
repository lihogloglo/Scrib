# Perf suite `m_floor`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 7.24 | 138 | 8.36 | 8.99 | 111 | 66.6 | 1/4, 1/5, 2/4 | 97 | 2572 | 101.6 | 81 | 6.07 | 8063 | 3200 | 0 | 23.56 | 29.73 | 4 | 1497 | 17 | 1598 | 2 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 7.24 | 138 | 8.42 | 9.04 | 111 | 29.2 | 1/4 | 97 | 2580 | 102.1 | 82 | 6.07 | 8064 | 3200 | 0 | 23.56 | 29.75 | 4 | 1483 | 17 | 1582 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 7.13 | 140 | 8.15 | 8.66 | 115 | 27.7 | 1/5 | 97 | 2568 | 101.6 | 80 | 5.99 | 8063 | 3200 | 0 | 22.95 | 28.06 | 4 | 1497 | 17 | 1598 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 7.27 | 138 | 8.36 | 8.99 | 111 | 66.6 | 2/4 | 97 | 2572 | 101.0 | 81 | 6.10 | 8063 | 3200 | 0 | 23.60 | 29.73 | 4 | 1501 | 17 | 1602 | 2 | 0 | 0 | 0 | 0 |
