# Perf suite `lean_post`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface ×3 | 10.92 | 92 | 14.36 | 17.04 | 59 | 37.1 | 5/35, 4/24, 5/29 | 69 | 2580 | 69.5 | 74 | 5.89 | 59.2 k | 31.6 k | 286 | 25.48 | 35.79 | 22 | 18.1 k | 112 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface ×3 | 8.61 | 116 | 10.20 | 11.06 | 90 | 16.7 | 0/35, 0/29, 0/32 | 94 | 2580 | 90.8 | 77 | 6.24 | 47.0 k | 24.6 k | 210 | 26.92 | 43.46 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #1 | 10.51 | 95 | 13.62 | 16.39 | 61 | 36.9 | 5/35 | 71 | 2580 | 71.2 | 75 | 5.81 | 59.2 k | 31.6 k | 286 | 25.48 | 34.12 | 22 | 17.8 k | 110 | 16.6 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #2 | 11.15 | 90 | 14.65 | 17.14 | 58 | 33.2 | 4/24 | 68 | 2580 | 69.5 | 74 | 5.93 | 59.2 k | 31.6 k | 286 | 25.26 | 35.79 | 22 | 18.2 k | 115 | 17.5 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #3 | 10.92 | 92 | 14.36 | 17.04 | 59 | 37.1 | 5/29 | 69 | 2580 | 69.3 | 73 | 5.89 | 59.2 k | 31.6 k | 286 | 25.48 | 38.96 | 22 | 18.1 k | 112 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #1 | 8.73 | 115 | 10.88 | 11.80 | 85 | 16.7 | 0/35 | 90 | 2580 | 89.1 | 77 | 6.24 | 47.0 k | 24.6 k | 210 | 28.65 | 46.67 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #2 | 8.61 | 116 | 10.20 | 11.06 | 90 | 12.3 | 0/29 | 94 | 2580 | 90.8 | 77 | 6.17 | 47.0 k | 24.6 k | 210 | 26.92 | 43.46 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface #3 | 8.42 | 119 | 9.99 | 10.86 | 92 | 12.0 | 0/32 | 95 | 2580 | 92.3 | 77 | 6.31 | 47.0 k | 24.6 k | 210 | 25.79 | 40.55 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
