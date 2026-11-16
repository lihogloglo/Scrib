# Perf suite `f_thr12`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.35 | 88 | 13.89 | 16.05 | 62 | 32.7 | 2/38, 3/39, 3/40 | 77 | 2580 | 77.2 | 72 | 7.36 | 59.3 k | 31.7 k | 286 | 28.37 | 35.12 | 45 | 19.3 k | 66 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.11 | 99 | 11.46 | 11.98 | 83 | 20.0 | 0/53, 0/54, 0/53 | 98 | 2580 | 95.0 | 74 | 9.00 | 47.1 k | 24.7 k | 210 | 26.21 | 31.81 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.46 | 87 | 14.10 | 16.05 | 62 | 31.7 | 2/38 | 76 | 2580 | 75.9 | 71 | 7.36 | 59.3 k | 31.7 k | 286 | 27.88 | 34.92 | 45 | 19.3 k | 65 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 11.35 | 88 | 13.88 | 15.84 | 63 | 32.7 | 3/39 | 77 | 2580 | 77.2 | 72 | 7.37 | 59.3 k | 31.7 k | 286 | 28.37 | 35.12 | 45 | 19.3 k | 66 | 10.7 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.32 | 88 | 13.89 | 16.34 | 61 | 31.3 | 3/40 | 77 | 2580 | 77.5 | 73 | 7.30 | 59.3 k | 31.7 k | 286 | 28.37 | 35.37 | 45 | 19.2 k | 68 | 10.9 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.17 | 98 | 11.54 | 12.03 | 83 | 16.7 | 0/53 | 98 | 2580 | 96.1 | 75 | 9.05 | 47.1 k | 24.7 k | 210 | 25.81 | 31.81 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.11 | 99 | 11.46 | 11.98 | 83 | 15.1 | 0/54 | 98 | 2580 | 94.5 | 74 | 9.00 | 47.1 k | 24.7 k | 210 | 29.33 | 34.63 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.09 | 99 | 11.41 | 11.87 | 84 | 20.0 | 0/53 | 98 | 2580 | 95.0 | 74 | 8.98 | 47.1 k | 24.7 k | 210 | 26.21 | 30.32 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
