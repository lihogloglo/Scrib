# Perf suite `f_h1`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.24 | 89 | 13.99 | 16.21 | 62 | 31.1 | 2/39, 2/42, 3/38 | 78 | 2580 | 77.1 | 72 | 7.40 | 59.3 k | 31.7 k | 286 | 25.45 | 30.64 | 45 | 56.9 k | 70 | 41.4 k | 20 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.30 | 97 | 11.72 | 12.29 | 81 | 17.7 | 0/51, 0/53, 0/53 | 98 | 2580 | 94.0 | 73 | 9.02 | 47.1 k | 24.7 k | 210 | 23.56 | 27.90 | 3 | 46.7 k | 59 | 34.7 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.05 | 91 | 13.95 | 16.10 | 62 | 28.1 | 2/39 | 80 | 2580 | 77.8 | 72 | 7.32 | 59.3 k | 31.7 k | 286 | 25.58 | 30.71 | 45 | 56.9 k | 69 | 41.4 k | 20 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 11.44 | 87 | 13.99 | 16.21 | 62 | 30.8 | 2/42 | 78 | 2580 | 76.6 | 72 | 7.42 | 59.3 k | 31.7 k | 286 | 25.45 | 30.56 | 45 | 56.9 k | 70 | 41.4 k | 20 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.24 | 89 | 14.46 | 16.35 | 61 | 31.1 | 3/38 | 78 | 2580 | 77.1 | 72 | 7.40 | 59.3 k | 31.7 k | 286 | 25.36 | 30.64 | 45 | 56.9 k | 70 | 41.3 k | 20 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.37 | 96 | 11.83 | 12.68 | 79 | 15.6 | 0/51 | 98 | 2580 | 94.0 | 75 | 8.98 | 47.1 k | 24.7 k | 210 | 25.79 | 32.19 | 3 | 46.7 k | 59 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.29 | 97 | 11.72 | 12.29 | 81 | 17.7 | 0/53 | 98 | 2580 | 94.0 | 73 | 9.02 | 47.1 k | 24.7 k | 210 | 23.56 | 27.90 | 3 | 46.7 k | 59 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.30 | 97 | 11.68 | 12.12 | 83 | 16.0 | 0/53 | 98 | 2580 | 94.0 | 73 | 9.04 | 47.1 k | 24.7 k | 210 | 22.77 | 27.46 | 3 | 46.7 k | 59 | 34.7 k | 0 | 0 | 0 | 0 | 0 |
