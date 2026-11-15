# Perf suite `b2_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: flatnormal · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=flatnormal ×3 | 12.47 | 80 | 15.72 | 18.41 | 54 | 36.0 | 3/21, 6/22, 3/19 | 81 | 2580 | 76.2 | 74 | 7.84 | 59.3 k | 31.7 k | 286 | 26.13 | 37.93 | 46 | 19.7 k | 92 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=flatnormal ×3 | 11.64 | 86 | 13.31 | 15.29 | 65 | 26.9 | 0/24, 0/26, 0/34 | 98 | 2580 | 88.7 | 73 | 9.64 | 47.1 k | 24.7 k | 210 | 26.56 | 40.47 | 4 | 14.6 k | 77 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=flatnormal #1 | 12.47 | 80 | 15.72 | 18.41 | 54 | 32.8 | 3/21 | 81 | 2580 | 76.2 | 74 | 7.84 | 59.3 k | 31.7 k | 286 | 26.34 | 37.93 | 46 | 19.7 k | 92 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=flatnormal #2 | 12.20 | 82 | 15.46 | 17.73 | 56 | 35.7 | 6/22 | 82 | 2580 | 77.0 | 74 | 7.73 | 59.3 k | 31.7 k | 286 | 26.13 | 37.68 | 46 | 19.7 k | 90 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=flatnormal #3 | 13.41 | 75 | 17.54 | 21.44 | 47 | 36.0 | 3/19 | 76 | 2580 | 72.2 | 73 | 8.10 | 59.3 k | 31.7 k | 286 | 24.88 | 38.24 | 46 | 19.7 k | 92 | 17.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=flatnormal #1 | 11.64 | 86 | 13.31 | 16.21 | 62 | 26.9 | 0/24 | 98 | 2580 | 88.7 | 72 | 9.64 | 47.1 k | 24.7 k | 210 | 32.13 | 46.02 | 4 | 14.6 k | 77 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=flatnormal #2 | 11.44 | 87 | 12.75 | 13.30 | 75 | 23.9 | 0/26 | 98 | 2580 | 90.2 | 73 | 9.54 | 47.1 k | 24.7 k | 210 | 26.56 | 35.79 | 4 | 14.6 k | 77 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=flatnormal #3 | 12.21 | 82 | 13.97 | 15.29 | 65 | 17.3 | 0/34 | 98 | 2580 | 88.0 | 73 | 10.17 | 47.1 k | 24.7 k | 210 | 22.96 | 40.47 | 4 | 14.6 k | 77 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
