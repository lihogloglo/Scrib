# Perf suite `f_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: cascadeencoders,cascadehold,meshslabs · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadeencoders,cascadehold,meshslabs ×3 | 12.47 | 80 | 15.80 | 18.07 | 55 | 62.8 | 1/36, 3/43, 4/43 | 73 | 2580 | 74.1 | 71 | 7.40 | 59.3 k | 31.7 k | 286 | 25.24 | 31.58 | 46 | 19.2 k | 84 | 17.0 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders,cascadehold,meshslabs ×3 | 10.58 | 94 | 11.89 | 12.72 | 79 | 17.5 | 0/49, 0/52, 0/47 | 98 | 2580 | 94.1 | 74 | 8.89 | 47.1 k | 24.7 k | 210 | 27.88 | 35.82 | 4 | 14.5 k | 71 | 14.7 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=cascadeencoders,cascadehold,meshslabs #1 | 12.63 | 79 | 15.80 | 18.07 | 55 | 35.6 | 1/36 | 72 | 2580 | 73.0 | 70 | 7.40 | 59.3 k | 31.7 k | 286 | 25.24 | 31.58 | 46 | 19.2 k | 84 | 17.1 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadeencoders,cascadehold,meshslabs #2 | 12.25 | 82 | 15.12 | 17.18 | 58 | 62.8 | 3/43 | 74 | 2580 | 74.6 | 71 | 7.32 | 59.3 k | 31.7 k | 286 | 25.24 | 31.52 | 46 | 19.2 k | 82 | 17.0 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=cascadeencoders,cascadehold,meshslabs #3 | 12.47 | 80 | 16.27 | 20.68 | 48 | 36.8 | 4/43 | 73 | 2580 | 74.1 | 71 | 7.42 | 59.3 k | 31.7 k | 286 | 25.48 | 37.73 | 46 | 19.2 k | 85 | 17.0 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders,cascadehold,meshslabs #1 | 10.60 | 94 | 12.11 | 13.34 | 75 | 17.5 | 0/49 | 98 | 2580 | 94.0 | 74 | 8.92 | 47.1 k | 24.7 k | 210 | 29.33 | 36.57 | 4 | 14.5 k | 71 | 14.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders,cascadehold,meshslabs #2 | 10.58 | 94 | 11.89 | 12.72 | 79 | 14.4 | 0/52 | 98 | 2580 | 95.4 | 74 | 8.85 | 47.1 k | 24.7 k | 210 | 27.88 | 35.82 | 4 | 14.5 k | 71 | 14.7 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=cascadeencoders,cascadehold,meshslabs #3 | 10.48 | 95 | 11.63 | 12.32 | 81 | 16.8 | 0/47 | 98 | 2580 | 94.1 | 73 | 8.89 | 47.1 k | 24.7 k | 210 | 25.84 | 32.71 | 4 | 14.5 k | 69 | 14.7 k | 0 | 0 | 0 | 0 | 0 |
