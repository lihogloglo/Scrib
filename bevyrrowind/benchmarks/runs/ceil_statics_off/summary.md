# Perf suite `ceil_statics_off`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: statics · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=statics ×3 | 7.52 | 133 | 8.49 | 9.03 | 111 | 26.0 | 0/5, 0/5, 1/4 | 98 | 2580 | 100.9 | 76 | 7.26 | 6837 | 2697 | 0 | 21.15 | 30.63 | 4 | 1377 | 17 | 1501 | 54 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=statics ×3 | 7.21 | 139 | 8.28 | 8.99 | 111 | 29.5 | 0/4, 2/4, 0/17 | 97 | 2580 | 100.7 | 78 | 6.74 | 8059 | 3200 | 0 | 22.70 | 28.90 | 4 | 1504 | 17 | 1605 | 54 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=statics #1 | 7.52 | 133 | 8.47 | 9.03 | 111 | 24.7 | 0/5 | 98 | 2580 | 102.1 | 76 | 7.24 | 6837 | 2697 | 0 | 22.22 | 31.19 | 4 | 1377 | 17 | 1501 | 54 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=statics #2 | 7.52 | 133 | 8.49 | 8.91 | 112 | 16.6 | 0/5 | 98 | 2580 | 100.0 | 73 | 7.26 | 6837 | 2697 | 0 | 20.67 | 26.13 | 4 | 1377 | 17 | 1501 | 54 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=statics #3 | 7.61 | 131 | 8.74 | 9.32 | 107 | 26.0 | 1/4 | 98 | 2580 | 100.9 | 76 | 7.34 | 6837 | 2697 | 0 | 21.15 | 30.63 | 4 | 1377 | 17 | 1501 | 54 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=statics #1 | 7.21 | 139 | 8.28 | 8.94 | 112 | 23.7 | 0/4 | 97 | 2580 | 100.7 | 78 | 6.74 | 8059 | 3200 | 0 | 22.12 | 28.19 | 4 | 1504 | 17 | 1605 | 54 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=statics #2 | 7.25 | 138 | 8.32 | 9.07 | 110 | 29.5 | 2/4 | 97 | 2580 | 100.3 | 78 | 6.76 | 8059 | 3200 | 0 | 22.70 | 32.35 | 4 | 1507 | 17 | 1610 | 54 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=statics #3 | 7.16 | 140 | 8.25 | 8.99 | 111 | 21.8 | 0/17 | 97 | 2580 | 100.8 | 78 | 6.68 | 8060 | 3200 | 0 | 23.08 | 28.90 | 4 | 1497 | 17 | 1598 | 54 | 0 | 0 | 0 | 0 |
