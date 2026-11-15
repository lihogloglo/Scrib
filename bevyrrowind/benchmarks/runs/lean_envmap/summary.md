# Perf suite `lean_envmap`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds,fog,froxels,envmap · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap ×3 | 11.19 | 89 | 14.61 | 16.88 | 59 | 67.9 | 9/32, 4/23, 3/28 | 74 | 2579 | 74.9 | 75 | 6.74 | 59.2 k | 31.6 k | 286 | 25 | 33.67 | 22 | 18.0 k | 112 | 17.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap ×3 | 9.18 | 109 | 10.57 | 11.72 | 85 | 20.2 | 0/31, 0/36, 0/27 | 96 | 2580 | 94.7 | 78 | 7.28 | 47.0 k | 24.6 k | 210 | 31.33 | 39.25 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #1 | 10.96 | 91 | 14.10 | 16.58 | 60 | 67.9 | 9/32 | 78 | 2579 | 77.2 | 76 | 6.74 | 59.2 k | 31.6 k | 286 | 25.60 | 32.79 | 22 | 18.0 k | 112 | 17.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #2 | 11.41 | 88 | 15.50 | 20.13 | 50 | 35.6 | 4/23 | 74 | 2577 | 74.0 | 75 | 6.77 | 59.2 k | 31.5 k | 286 | 25 | 33.67 | 22 | 18.1 k | 112 | 17.4 k | 12 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #3 | 11.19 | 89 | 14.61 | 16.88 | 59 | 34.7 | 3/28 | 74 | 2580 | 74.9 | 74 | 6.51 | 59.2 k | 31.6 k | 286 | 24.88 | 35.42 | 22 | 17.8 k | 115 | 17.0 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #1 | 9.18 | 109 | 10.68 | 11.72 | 85 | 16.3 | 0/31 | 96 | 2580 | 94.7 | 78 | 7.28 | 47.0 k | 24.6 k | 210 | 31.49 | 41.22 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #2 | 9.17 | 109 | 10.57 | 11.33 | 88 | 12.9 | 0/36 | 97 | 2580 | 95.8 | 78 | 7.17 | 47.0 k | 24.6 k | 210 | 31.33 | 39.25 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap #3 | 9.25 | 108 | 10.56 | 12.00 | 83 | 20.2 | 0/27 | 96 | 2574 | 94.5 | 78 | 7.39 | 47.0 k | 24.6 k | 210 | 30.62 | 38.91 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
