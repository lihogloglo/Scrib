# Perf suite `f_noslab`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: meshslabs · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=meshslabs ×3 | 11.31 | 88 | 13.96 | 16.08 | 62 | 62.7 | 2/39, 2/48, 2/33 | 77 | 2580 | 77.3 | 72 | 7.18 | 59.3 k | 31.9 k | 286 | 26.68 | 32.34 | 45 | 19.2 k | 67 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=meshslabs ×3 | 10.09 | 99 | 11.51 | 11.99 | 83 | 18.6 | 0/55, 0/47, 0/56 | 98 | 2580 | 95.0 | 74 | 8.92 | 47.1 k | 24.7 k | 210 | 24.52 | 30.72 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=meshslabs #1 | 11.43 | 87 | 13.96 | 16.24 | 62 | 30.1 | 2/39 | 77 | 2580 | 76.3 | 72 | 7.38 | 59.3 k | 31.9 k | 286 | 26.68 | 32.86 | 45 | 19.2 k | 67 | 10.9 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=meshslabs #2 | 10.72 | 93 | 13.02 | 15.29 | 65 | 28.1 | 2/48 | 81 | 2580 | 80.8 | 74 | 6.83 | 59.3 k | 31.9 k | 286 | 26.92 | 32.34 | 45 | 19.2 k | 63 | 10.8 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=meshslabs #3 | 11.31 | 88 | 14.13 | 16.08 | 62 | 62.7 | 2/33 | 77 | 2580 | 77.3 | 72 | 7.18 | 59.3 k | 31.7 k | 286 | 26.08 | 32.29 | 45 | 19.2 k | 69 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=meshslabs #1 | 10.12 | 99 | 11.51 | 11.95 | 84 | 16.7 | 0/55 | 98 | 2580 | 95.7 | 75 | 8.99 | 47.1 k | 24.7 k | 210 | 24.52 | 32.15 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=meshslabs #2 | 10.06 | 99 | 11.46 | 12.14 | 82 | 18.6 | 0/47 | 98 | 2580 | 95.0 | 74 | 8.92 | 47.1 k | 24.7 k | 210 | 25.48 | 30.14 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=meshslabs #3 | 10.09 | 99 | 11.52 | 11.99 | 83 | 14.8 | 0/56 | 98 | 2580 | 94.7 | 74 | 8.92 | 47.1 k | 24.7 k | 210 | 24.34 | 30.72 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
