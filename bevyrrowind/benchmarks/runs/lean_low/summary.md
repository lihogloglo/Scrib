# Perf suite `lean_low`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 9.98 | 100 | 12.23 | 13.66 | 73 | 28.6 | 0/18, 1/21, 0/23 | 74 | 2580 | 74.6 | 76 | 5.65 | 39.7 k | 22.4 k | 189 | 23.38 | 36.28 | 4 | 10.0 k | 111 | 10.1 k | 7 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 8.78 | 114 | 10.70 | 11.83 | 85 | 46.7 | 0/21, 4/26, 0/18 | 93 | 2580 | 89.8 | 75 | 6.89 | 29.5 k | 15.6 k | 125 | 26.31 | 39.72 | 4 | 7825 | 92 | 7940 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 9.98 | 100 | 12.23 | 13.66 | 73 | 24.7 | 0/18 | 74 | 2580 | 74.6 | 76 | 5.65 | 39.7 k | 22.3 k | 189 | 23.90 | 36.26 | 4 | 10.0 k | 110 | 10.1 k | 7 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 9.93 | 101 | 12.03 | 13.35 | 75 | 28.6 | 1/21 | 74 | 2580 | 74.7 | 75 | 5.64 | 39.7 k | 22.4 k | 189 | 23.21 | 36.28 | 4 | 10.0 k | 111 | 10.1 k | 7 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 10.69 | 94 | 13.40 | 14.95 | 67 | 25.9 | 0/23 | 69 | 2579 | 71.5 | 76 | 5.66 | 39.7 k | 22.4 k | 189 | 23.38 | 43.03 | 4 | 10.1 k | 111 | 10.2 k | 8 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 8.83 | 113 | 10.70 | 11.83 | 85 | 15.1 | 0/21 | 93 | 2580 | 89.8 | 76 | 6.82 | 29.5 k | 15.6 k | 125 | 28.37 | 44.10 | 4 | 7825 | 92 | 7940 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 8.78 | 114 | 10.77 | 11.91 | 84 | 46.7 | 4/26 | 93 | 2580 | 89.7 | 75 | 7.17 | 29.5 k | 15.6 k | 125 | 26.31 | 37.73 | 4 | 7825 | 92 | 7940 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 8.76 | 114 | 10.40 | 11.42 | 88 | 13.5 | 0/18 | 94 | 2580 | 90.5 | 74 | 6.89 | 29.5 k | 15.6 k | 125 | 25.23 | 39.72 | 4 | 7825 | 92 | 7940 | 0 | 0 | 0 | 0 | 0 |
