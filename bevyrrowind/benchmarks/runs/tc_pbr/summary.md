# Perf suite `tc_pbr`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax ×3 | 13.19 | 76 | 15.99 | 18.98 | 53 | 57.0 | 6/21, 4/20, 3/18 | 94 | 2570 | 91.2 | 82 | 9.67 | 59.4 k | 31.8 k | 286 | 29.33 | 45.18 | 22 | 18.6 k | 110 | 17.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax ×3 | 15.55 | 64 | 16.82 | 18.20 | 55 | 84.6 | 2/20, 0/13, 0/14 | 98 | 2578 | 99.3 | 81 | 13.61 | 47.0 k | 24.6 k | 210 | 19.18 | 24.51 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #1 | 13.19 | 76 | 16.45 | 21.59 | 46 | 57.0 | 6/21 | 92 | 2574 | 89.1 | 78 | 9.67 | 59.4 k | 31.8 k | 286 | 28.10 | 44.20 | 22 | 18.5 k | 108 | 17.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #2 | 12.98 | 77 | 15.69 | 18.88 | 53 | 39.0 | 4/20 | 94 | 2566 | 91.6 | 82 | 9.65 | 59.4 k | 31.8 k | 286 | 29.33 | 45.18 | 22 | 18.6 k | 110 | 17.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #3 | 13.36 | 75 | 15.99 | 18.98 | 53 | 36.7 | 3/18 | 94 | 2570 | 91.2 | 83 | 9.95 | 59.4 k | 31.8 k | 286 | 29.96 | 46.63 | 22 | 18.7 k | 111 | 17.6 k | 12 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #1 | 15.55 | 64 | 16.82 | 18.20 | 55 | 84.6 | 2/20 | 98 | 2577 | 99.9 | 82 | 13.61 | 47.0 k | 24.6 k | 210 | 21.88 | 26.09 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #2 | 15.43 | 65 | 16.67 | 17.67 | 57 | 26.4 | 0/13 | 98 | 2580 | 99.3 | 81 | 13.60 | 47.0 k | 24.6 k | 210 | 17.43 | 24.23 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #3 | 16.11 | 62 | 17.83 | 19.51 | 51 | 25.1 | 0/14 | 98 | 2578 | 98.3 | 80 | 14.22 | 47.0 k | 24.6 k | 210 | 19.18 | 24.51 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
