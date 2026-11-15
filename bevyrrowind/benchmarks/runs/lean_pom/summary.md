# Perf suite `lean_pom`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: parallax · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=parallax ×3 | 12.95 | 77 | 16.82 | 19.14 | 52 | 38.4 | 1/18, 3/14, 0/17 | 76 | 2580 | 73.5 | 74 | 7.78 | 59.2 k | 31.6 k | 286 | 23.81 | 29.68 | 22 | 17.9 k | 149 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=parallax ×3 | 10.81 | 93 | 12.40 | 13.77 | 73 | 27.6 | 1/28, 0/27, 0/31 | 98 | 2580 | 92.9 | 76 | 8.68 | 47.0 k | 24.6 k | 210 | 27.64 | 33.06 | 4 | 14.6 k | 97 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=parallax #1 | 12.95 | 77 | 16.94 | 19.14 | 52 | 32.8 | 1/18 | 76 | 2580 | 72.9 | 72 | 7.76 | 59.2 k | 31.6 k | 286 | 23.81 | 29.68 | 22 | 18.0 k | 149 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=parallax #2 | 13.04 | 77 | 16.82 | 19.45 | 51 | 38.4 | 3/14 | 76 | 2580 | 73.5 | 74 | 7.78 | 59.2 k | 31.6 k | 286 | 23.66 | 29.61 | 22 | 17.4 k | 157 | 16.9 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=parallax #3 | 12.73 | 79 | 16.40 | 18.54 | 54 | 31.7 | 0/17 | 78 | 2580 | 74.6 | 75 | 7.84 | 59.2 k | 31.6 k | 286 | 24.04 | 29.89 | 22 | 17.9 k | 146 | 17.2 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=parallax #1 | 10.81 | 93 | 12.40 | 14.06 | 71 | 27.6 | 1/28 | 97 | 2580 | 92.5 | 77 | 8.66 | 47.0 k | 24.6 k | 210 | 29.60 | 36.15 | 4 | 14.6 k | 97 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=parallax #2 | 10.61 | 94 | 11.83 | 12.69 | 79 | 26.2 | 0/27 | 98 | 2580 | 92.9 | 76 | 8.68 | 47.0 k | 24.6 k | 210 | 27.64 | 32.76 | 4 | 14.6 k | 97 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=parallax #3 | 11.20 | 89 | 12.63 | 13.77 | 73 | 19.8 | 0/31 | 98 | 2580 | 93.0 | 76 | 9.16 | 47.0 k | 24.6 k | 210 | 24.76 | 33.06 | 4 | 14.6 k | 97 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
