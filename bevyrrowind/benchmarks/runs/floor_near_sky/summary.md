# Perf suite `floor_near_sky`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 5.21 | 192 | 6.07 | 6.54 | 153 | 14.3 | 0/5, 0/5, 0/5 | 95 | 2580 | 94.4 | 78 | 4.51 | 8042 | 3200 | 0 | 25.96 | 32.03 | 1 | 908 | 12 | 1010 | 2 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 5.21 | 192 | 6.05 | 6.49 | 154 | 14.3 | 0/5 | 95 | 2580 | 94.0 | 78 | 4.54 | 8042 | 3200 | 0 | 25.96 | 31.82 | 1 | 906 | 12 | 1008 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 5.23 | 191 | 6.07 | 6.54 | 153 | 11.8 | 0/5 | 94 | 2580 | 94.4 | 78 | 4.51 | 8042 | 3200 | 0 | 25.48 | 32.03 | 1 | 912 | 12 | 1015 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 5.18 | 193 | 6.08 | 6.59 | 152 | 12.1 | 0/5 | 95 | 2580 | 95.2 | 78 | 4.50 | 8042 | 3200 | 0 | 25.97 | 32.07 | 1 | 908 | 12 | 1010 | 2 | 0 | 0 | 0 | 0 |
