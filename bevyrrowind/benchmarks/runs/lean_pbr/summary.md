# Perf suite `lean_pbr`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax ×3 | 11.42 | 88 | 14.40 | 16.68 | 60 | 34.1 | 7/26, 4/24, 3/27 | 84 | 2579 | 80.9 | 76 | 7.28 | 59.2 k | 31.7 k | 286 | 25.90 | 33.18 | 22 | 18.2 k | 111 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax ×3 | 10.46 | 96 | 11.76 | 12.78 | 78 | 25.3 | 0/25, 0/31, 0/30 | 98 | 2580 | 93.0 | 78 | 8.67 | 47.0 k | 24.6 k | 210 | 25.36 | 33.05 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #1 | 11.07 | 90 | 13.92 | 16.25 | 62 | 33.9 | 7/26 | 86 | 2580 | 81.8 | 76 | 7.15 | 59.3 k | 31.8 k | 286 | 26.44 | 33.34 | 22 | 18.4 k | 108 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #2 | 11.42 | 88 | 14.40 | 16.68 | 60 | 34.1 | 4/24 | 84 | 2579 | 80.9 | 77 | 7.28 | 59.2 k | 31.7 k | 286 | 25.90 | 32.42 | 22 | 18.2 k | 112 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax #3 | 11.46 | 87 | 14.50 | 17.21 | 58 | 34.0 | 3/27 | 83 | 2578 | 79.6 | 76 | 7.28 | 59.2 k | 31.7 k | 286 | 25.84 | 33.18 | 22 | 18.1 k | 111 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #1 | 10.92 | 92 | 12.22 | 12.78 | 78 | 25.3 | 0/25 | 98 | 2580 | 92.0 | 78 | 8.95 | 47.0 k | 24.6 k | 210 | 29.37 | 34.34 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #2 | 10.42 | 96 | 11.76 | 12.95 | 77 | 23.6 | 0/31 | 98 | 2580 | 93.4 | 78 | 8.61 | 47.0 k | 24.6 k | 210 | 25.36 | 31.93 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax #3 | 10.46 | 96 | 11.59 | 12.18 | 82 | 19.6 | 0/30 | 98 | 2580 | 93.0 | 74 | 8.67 | 47.0 k | 24.6 k | 210 | 24.85 | 33.05 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
