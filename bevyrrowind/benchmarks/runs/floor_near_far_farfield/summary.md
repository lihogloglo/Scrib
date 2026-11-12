# Perf suite `floor_near_far_farfield`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 6.93 | 144 | 7.87 | 8.29 | 121 | 11.0 | 0/3, 0/3, 0/3 | 98 | 2577 | 100.0 | 81 | 6.67 | 10.9 k | 519 | 0 | 22.12 | 26.77 | 4 | 857 | 5 | 858 | 1 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 6.93 | 144 | 7.87 | 8.32 | 120 | 11.0 | 0/3 | 98 | 2577 | 99.9 | 81 | 6.66 | 10.9 k | 519 | 0 | 22.12 | 26.77 | 4 | 857 | 5 | 858 | 1 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 6.93 | 144 | 7.88 | 8.29 | 121 | 10.8 | 0/3 | 98 | 2566 | 100.0 | 81 | 6.67 | 10.9 k | 519 | 0 | 23.08 | 27.73 | 4 | 857 | 5 | 858 | 1 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 6.93 | 144 | 7.85 | 8.24 | 121 | 10.5 | 0/3 | 98 | 2580 | 100.1 | 81 | 6.67 | 10.9 k | 519 | 0 | 20.48 | 26.53 | 4 | 857 | 5 | 858 | 1 | 0 | 0 | 0 | 0 |
