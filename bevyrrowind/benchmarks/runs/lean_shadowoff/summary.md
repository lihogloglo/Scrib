# Perf suite `lean_shadowoff`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view); `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows ×2 | 9.37 | 107 | 12.46 | 14.56 | 69 | 46.6 | 6/36, 7/30 | 69 | 2580 | 71.2 | 73 | 5.59 | 59.2 k | 31.6 k | 286 | 25.25 | 41.37 | 19 | 9462 | 62 | 8610 | 6 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows ×3 | 7.77 | 129 | 9.76 | 10.96 | 91 | 31.5 | 1/31, 0/35, 0/33 | 91 | 2580 | 91.0 | 76 | 5.95 | 47.0 k | 24.6 k | 210 | 25.93 | 44.22 | 1 | 9282 | 58 | 9462 | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows ×3 | 7.52 | 133 | 9.21 | 10.35 | 97 | 24.5 | 0/27, 0/30, 0/31 | 93 | 2580 | 91.3 | 77 | 5.75 | 47.0 k | 24.7 k | 222 | 26.85 | 43.35 | 1 | 9300 | 58 | 9480 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #1 | 9.37 | 107 | 12.56 | 14.80 | 68 | 34.7 | 6/36 | 68 | 2580 | 71.2 | 73 | 5.58 | 59.2 k | 31.6 k | 286 | 25.51 | 41.46 | 19 | 9557 | 62 | 8588 | 6 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #2 | 9.37 | 107 | 12.37 | 14.33 | 70 | 46.6 | 7/30 | 69 | 2580 | 71.3 | 73 | 5.61 | 59.2 k | 31.6 k | 286 | 25 | 41.28 | 19 | 9367 | 62 | 8633 | 6 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #1 | 7.66 | 131 | 9.64 | 10.96 | 91 | 31.5 | 1/31 | 92 | 2580 | 91.6 | 76 | 5.95 | 47.0 k | 24.6 k | 210 | 28.71 | 44.22 | 1 | 9282 | 58 | 9462 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #2 | 7.77 | 129 | 9.76 | 10.79 | 93 | 21.5 | 0/35 | 90 | 2580 | 89.6 | 76 | 5.88 | 47.0 k | 24.6 k | 210 | 25.71 | 40.70 | 1 | 9282 | 58 | 9462 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #3 | 7.79 | 128 | 10.00 | 11.52 | 87 | 16.0 | 0/33 | 91 | 2580 | 91.0 | 77 | 6.01 | 47.0 k | 24.6 k | 210 | 25.93 | 44.43 | 1 | 9282 | 58 | 9462 | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #1 | 7.58 | 132 | 9.47 | 10.72 | 93 | 23.6 | 0/27 | 91 | 2580 | 89.9 | 76 | 5.75 | 47.0 k | 24.7 k | 222 | 26.85 | 43.76 | 1 | 9300 | 58 | 9480 | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #2 | 7.48 | 134 | 9.21 | 10.35 | 97 | 24.5 | 0/30 | 93 | 2580 | 91.3 | 77 | 5.77 | 47.0 k | 24.7 k | 222 | 26.92 | 43.35 | 1 | 9300 | 58 | 9480 | 0 | 0 | 0 | 0 | 0 |
| balmora_night 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows #3 | 7.52 | 133 | 9.21 | 10.24 | 98 | 17.5 | 0/31 | 93 | 2580 | 91.8 | 77 | 5.75 | 47.0 k | 24.7 k | 222 | 25.40 | 42.36 | 1 | 9300 | 58 | 9480 | 0 | 0 | 0 | 0 | 0 |
