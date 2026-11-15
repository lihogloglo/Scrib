# Perf suite `lean_shadowlite`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows ×3 | 10.99 | 91 | 14.46 | 18.37 | 54 | 70.1 | 5/32, 5/27, 4/23 | 65 | 2580 | 67.8 | 73 | 5.51 | 59.2 k | 31.7 k | 286 | 25.76 | 39.13 | 22 | 18.3 k | 113 | 17.5 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows ×3 | 8.53 | 117 | 10.29 | 11.31 | 88 | 20.1 | 0/31, 0/41, 0/35 | 90 | 2580 | 88.4 | 75 | 5.77 | 47.0 k | 24.6 k | 210 | 25.84 | 42.77 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #1 | 10.85 | 92 | 14.12 | 16.48 | 61 | 38.0 | 5/32 | 66 | 2580 | 68.6 | 73 | 5.48 | 59.2 k | 31.8 k | 286 | 25.89 | 39.13 | 22 | 18.3 k | 113 | 17.5 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #2 | 10.99 | 91 | 14.46 | 19.03 | 53 | 33.3 | 5/27 | 65 | 2580 | 67.8 | 73 | 5.51 | 59.2 k | 31.6 k | 286 | 25.36 | 38.11 | 22 | 18.2 k | 113 | 17.2 k | 10 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #3 | 12.02 | 83 | 15.89 | 18.37 | 54 | 70.1 | 4/23 | 61 | 2580 | 64.5 | 73 | 5.56 | 59.2 k | 31.7 k | 286 | 25.76 | 43.85 | 22 | 18.3 k | 115 | 17.5 k | 10 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #1 | 8.54 | 117 | 10.29 | 11.40 | 88 | 14.6 | 0/31 | 90 | 2580 | 88.4 | 75 | 5.77 | 47.0 k | 24.6 k | 210 | 25.84 | 42.77 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #2 | 8.49 | 118 | 10.37 | 11.31 | 88 | 14.2 | 0/41 | 89 | 2580 | 87.3 | 73 | 5.77 | 47.0 k | 24.6 k | 210 | 27.40 | 38.56 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 off=normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows #3 | 8.53 | 117 | 10.22 | 11.24 | 89 | 20.1 | 0/35 | 90 | 2580 | 88.5 | 75 | 5.63 | 47.0 k | 24.6 k | 210 | 25.48 | 44.58 | 4 | 14.7 k | 91 | 14.9 k | 0 | 0 | 0 | 0 | 0 |
