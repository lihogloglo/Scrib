# Perf suite `ceil_base`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.53 | 105 | 10.76 | 11.70 | 86 | 32.0 | 0/6, 1/10, 0/7 | 97 | 2581 | 91.7 | 69 | 8.56 | 47.0 k | 24.6 k | 210 | 25.18 | 31.65 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 10.95 | 91 | 13.66 | 15.25 | 66 | 42.2 | 2/7, 1/4, 1/5 | 78 | 2580 | 76.5 | 71 | 7.48 | 59.2 k | 31.8 k | 286 | 24.04 | 31.12 | 22 | 18.3 k | 83 | 17.3 k | 186 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.49 | 105 | 10.73 | 12.17 | 82 | 20.3 | 0/6 | 97 | 2585 | 90.5 | 66 | 8.55 | 47.0 k | 24.6 k | 210 | 24.22 | 30.62 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.53 | 105 | 10.76 | 11.70 | 86 | 32.0 | 1/10 | 97 | 2581 | 91.7 | 69 | 8.56 | 47.0 k | 24.6 k | 210 | 26.79 | 33.16 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.60 | 104 | 10.79 | 11.60 | 86 | 16.5 | 0/7 | 97 | 2580 | 91.8 | 71 | 8.59 | 47.0 k | 24.6 k | 210 | 25.18 | 31.65 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 10.96 | 91 | 13.66 | 15.68 | 64 | 42.2 | 2/7 | 78 | 2581 | 76.1 | 70 | 7.50 | 59.2 k | 31.8 k | 286 | 24.52 | 30.93 | 22 | 18.1 k | 84 | 17.3 k | 189 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 10.95 | 91 | 13.79 | 15.25 | 66 | 29.2 | 1/4 | 78 | 2580 | 76.5 | 71 | 7.48 | 59.2 k | 31.8 k | 286 | 24.04 | 31.12 | 22 | 18.3 k | 82 | 17.3 k | 184 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 10.86 | 92 | 13.47 | 14.86 | 67 | 28.4 | 1/5 | 79 | 2580 | 77.4 | 71 | 7.45 | 59.2 k | 31.7 k | 286 | 24.04 | 31.15 | 22 | 18.4 k | 83 | 17.4 k | 186 | 0 | 0 | 0 | 0 |
