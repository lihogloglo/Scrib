# Perf suite `lean_ultra`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 11.70 | 85 | 13.54 | 14.64 | 68 | 27.3 | 0/24, 0/29, 0/24 | 96 | 2580 | 90.7 | 74 | 9.05 | 47.0 k | 24.6 k | 210 | 26.61 | 34.50 | 5 | 18.1 k | 136 | 18.3 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 11.54 | 87 | 13.15 | 14.14 | 71 | 19.8 | 0/24 | 96 | 2581 | 89.7 | 72 | 9.05 | 47.0 k | 24.6 k | 210 | 28.37 | 36.16 | 5 | 18.1 k | 130 | 18.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 11.71 | 85 | 13.54 | 15.33 | 65 | 20.1 | 0/29 | 96 | 2580 | 90.7 | 74 | 9.00 | 47.0 k | 24.6 k | 210 | 26.32 | 34.50 | 5 | 18.1 k | 136 | 18.3 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.70 | 85 | 13.62 | 14.64 | 68 | 27.3 | 0/24 | 96 | 2580 | 91.1 | 75 | 9.08 | 47.0 k | 24.6 k | 210 | 26.61 | 32.92 | 5 | 18.1 k | 144 | 18.3 k | 0 | 0 | 0 | 0 | 0 |
