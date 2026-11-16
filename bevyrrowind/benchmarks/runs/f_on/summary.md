# Perf suite `f_on`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.42 | 88 | 14.38 | 16.72 | 60 | 37.9 | 4/42, 3/39, 4/44 | 77 | 2580 | 77.1 | 72 | 7.31 | 59.3 k | 31.7 k | 286 | 26.44 | 34.21 | 45 | 19.2 k | 66 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.27 | 97 | 11.78 | 12.72 | 79 | 18.7 | 0/53, 0/57, 0/49 | 98 | 2580 | 93.9 | 73 | 8.97 | 47.1 k | 24.7 k | 210 | 28.37 | 37.27 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.87 | 84 | 14.44 | 16.72 | 60 | 37.9 | 4/42 | 74 | 2580 | 74.5 | 70 | 7.41 | 59.3 k | 31.7 k | 286 | 26.34 | 33.52 | 45 | 19.3 k | 66 | 10.8 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 11.34 | 88 | 14.29 | 16.76 | 60 | 30.8 | 3/39 | 77 | 2580 | 77.1 | 72 | 7.31 | 59.3 k | 31.7 k | 286 | 26.44 | 34.21 | 45 | 19.2 k | 65 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.42 | 88 | 14.38 | 16.66 | 60 | 34.0 | 4/44 | 78 | 2580 | 77.4 | 73 | 7.23 | 59.3 k | 31.7 k | 286 | 26.92 | 34.51 | 45 | 19.1 k | 68 | 10.7 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 10.27 | 97 | 12.04 | 13.01 | 77 | 14.4 | 0/53 | 97 | 2580 | 93.9 | 75 | 8.78 | 47.1 k | 24.7 k | 210 | 30.04 | 39.28 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.32 | 97 | 11.78 | 12.72 | 79 | 16.8 | 0/57 | 98 | 2580 | 93.1 | 72 | 9.03 | 47.1 k | 24.7 k | 210 | 28.37 | 37.27 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.05 | 99 | 11.55 | 12.16 | 82 | 18.7 | 0/49 | 98 | 2580 | 94.4 | 73 | 8.97 | 47.1 k | 24.7 k | 210 | 23.56 | 28.89 | 3 | 15.0 k | 54 | 9498 | 0 | 0 | 0 | 0 | 0 |
