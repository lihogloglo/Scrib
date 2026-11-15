# Perf suite `tc_pbrwater`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,waterssr,shorewaves · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves ×3 | 12.36 | 81 | 14.93 | 17.28 | 58 | 37.3 | 2/20, 2/22, 3/21 | 95 | 2569 | 93.2 | 83 | 9.25 | 59.4 k | 31.8 k | 286 | 29.02 | 35.52 | 22 | 18.6 k | 110 | 17.6 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves ×3 | 14.76 | 68 | 16.12 | 17.19 | 58 | 24.5 | 0/20, 0/17, 0/20 | 98 | 2573 | 98.2 | 79 | 12.92 | 47.0 k | 24.6 k | 210 | 19.44 | 25.91 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #1 | 12.26 | 82 | 14.63 | 17.24 | 58 | 33.1 | 2/20 | 95 | 2569 | 93.2 | 82 | 9.24 | 59.4 k | 31.8 k | 286 | 28.37 | 35.41 | 22 | 18.7 k | 109 | 17.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #2 | 12.48 | 80 | 15.19 | 17.28 | 58 | 37.3 | 2/22 | 94 | 2566 | 92.3 | 83 | 9.25 | 59.4 k | 31.8 k | 286 | 29.52 | 35.86 | 22 | 18.6 k | 110 | 17.7 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #3 | 12.36 | 81 | 14.93 | 17.72 | 56 | 33.3 | 3/21 | 95 | 2572 | 93.4 | 83 | 9.32 | 59.4 k | 31.8 k | 286 | 29.02 | 35.52 | 22 | 18.5 k | 111 | 17.5 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #1 | 14.76 | 68 | 16.12 | 17.09 | 59 | 21.8 | 0/20 | 98 | 2573 | 98.2 | 79 | 12.92 | 47.0 k | 24.6 k | 210 | 20.19 | 25.91 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #2 | 14.88 | 67 | 16.29 | 17.21 | 58 | 21.1 | 0/17 | 98 | 2580 | 99.0 | 80 | 13.07 | 47.0 k | 24.6 k | 210 | 18.79 | 25.41 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,waterssr,shorewaves #3 | 14.58 | 69 | 16.00 | 17.19 | 58 | 24.5 | 0/20 | 93 | 2573 | 92.4 | 76 | 11.25 | 47.0 k | 24.6 k | 210 | 19.44 | 26.44 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
