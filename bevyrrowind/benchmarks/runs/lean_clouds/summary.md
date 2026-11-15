# Perf suite `lean_clouds`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds ×3 | 11.01 | 91 | 13.95 | 15.92 | 63 | 31.8 | 5/29, 3/22, 3/31 | 85 | 2580 | 81.4 | 77 | 7.09 | 59.2 k | 31.6 k | 286 | 25 | 31.87 | 22 | 18.2 k | 108 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds ×3 | 10.20 | 98 | 11.36 | 12.13 | 82 | 24.7 | 0/29, 0/27, 0/27 | 98 | 2580 | 95.0 | 78 | 8.47 | 47.0 k | 24.6 k | 210 | 25.56 | 31.19 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds #1 | 11.08 | 90 | 13.95 | 15.92 | 63 | 31.8 | 5/29 | 84 | 2575 | 80.8 | 76 | 7.10 | 59.4 k | 31.8 k | 286 | 24.82 | 31.61 | 22 | 18.1 k | 108 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds #2 | 11.01 | 91 | 14.10 | 16.27 | 61 | 31.6 | 3/22 | 85 | 2580 | 81.4 | 77 | 7.03 | 59.2 k | 31.6 k | 286 | 25 | 32.24 | 22 | 18.2 k | 107 | 17.3 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds #3 | 10.94 | 91 | 13.88 | 15.92 | 63 | 30.5 | 3/31 | 86 | 2580 | 82.3 | 78 | 7.09 | 59.2 k | 31.6 k | 286 | 25.47 | 31.87 | 22 | 18.3 k | 108 | 17.4 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds #1 | 10.20 | 98 | 11.36 | 12.13 | 82 | 24.7 | 0/29 | 98 | 2580 | 95.0 | 79 | 8.41 | 47.0 k | 24.6 k | 210 | 26.44 | 33.02 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds #2 | 10.20 | 98 | 11.33 | 11.95 | 84 | 24.4 | 0/27 | 98 | 2578 | 95.0 | 78 | 8.47 | 47.0 k | 24.6 k | 210 | 22.01 | 27.24 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds #3 | 10.33 | 97 | 11.60 | 12.28 | 81 | 22.4 | 0/27 | 97 | 2580 | 93.0 | 77 | 8.52 | 47.0 k | 24.6 k | 210 | 25.56 | 31.19 | 4 | 14.6 k | 89 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
