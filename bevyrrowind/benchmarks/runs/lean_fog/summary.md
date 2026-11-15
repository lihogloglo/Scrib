# Perf suite `lean_fog`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds,fog,froxels · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels ×3 | 10.91 | 92 | 14.42 | 16.69 | 60 | 32.2 | 5/32, 4/25, 1/33 | 80 | 2578 | 77.9 | 76 | 6.85 | 59.2 k | 31.6 k | 286 | 25.40 | 31.40 | 22 | 18.3 k | 112 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels ×3 | 9.66 | 104 | 11.09 | 12.08 | 83 | 27.3 | 0/32, 1/36, 1/30 | 98 | 2580 | 93.3 | 77 | 8.00 | 47.0 k | 24.6 k | 210 | 24.52 | 39.20 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #1 | 10.91 | 92 | 13.94 | 15.95 | 63 | 31.7 | 5/32 | 82 | 2578 | 78.7 | 76 | 6.74 | 59.2 k | 31.6 k | 286 | 25.71 | 31.99 | 22 | 18.3 k | 112 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #2 | 11.09 | 90 | 14.79 | 17.04 | 59 | 32.2 | 4/25 | 80 | 2580 | 77.3 | 76 | 6.90 | 59.2 k | 31.6 k | 286 | 25.40 | 31.34 | 22 | 17.8 k | 115 | 17.2 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #3 | 10.88 | 92 | 14.42 | 16.69 | 60 | 28.2 | 1/33 | 80 | 2575 | 77.9 | 76 | 6.85 | 59.2 k | 31.6 k | 286 | 25 | 31.40 | 22 | 18.3 k | 111 | 17.4 k | 11 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #1 | 9.66 | 104 | 10.86 | 11.75 | 85 | 24.0 | 0/32 | 98 | 2580 | 95.5 | 79 | 8.00 | 47.0 k | 24.6 k | 210 | 24.52 | 30.23 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #2 | 9.66 | 104 | 11.09 | 12.08 | 83 | 25.0 | 1/36 | 95 | 2580 | 92.8 | 76 | 7.75 | 47.0 k | 24.6 k | 210 | 26.44 | 39.20 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels #3 | 9.83 | 102 | 11.34 | 12.14 | 82 | 27.3 | 1/30 | 98 | 2580 | 93.3 | 77 | 8.16 | 47.0 k | 24.6 k | 210 | 24.04 | 40.04 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
