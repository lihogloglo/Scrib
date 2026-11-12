# Perf suite `floor_near_shadows`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 6.98 | 143 | 7.98 | 8.42 | 119 | 15.3 | 0/3, 0/4, 0/5 | 98 | 2580 | 99.3 | 80 | 6.97 | 8061 | 3200 | 0 | 21.15 | 27.01 | 1 | 900 | 12 | 1000 | 2 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 7.04 | 142 | 8.03 | 8.47 | 118 | 10.2 | 0/3 | 98 | 2580 | 99.1 | 80 | 7.03 | 8061 | 3200 | 0 | 22.60 | 28.12 | 1 | 900 | 12 | 1000 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 6.98 | 143 | 7.96 | 8.37 | 120 | 15.3 | 0/4 | 98 | 2580 | 99.4 | 80 | 6.97 | 8061 | 3200 | 0 | 21.15 | 27.01 | 1 | 900 | 12 | 1001 | 2 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 6.98 | 143 | 7.98 | 8.42 | 119 | 9.8 | 0/5 | 98 | 2580 | 99.3 | 80 | 6.95 | 8061 | 3200 | 0 | 21.15 | 26.43 | 1 | 894 | 12 | 995 | 2 | 0 | 0 | 0 | 0 |
