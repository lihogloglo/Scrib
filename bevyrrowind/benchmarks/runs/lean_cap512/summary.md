# Perf suite `lean_cap512`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --texture-cap 512 ×3 | 13.85 | 72 | 17.87 | 19.98 | 50 | 39.9 | 1/16, 1/17, 2/13 | 73 | 2580 | 71.2 | 73 | 7.99 | 59.2 k | 31.6 k | 286 | 22.71 | 28.82 | 22 | 18.1 k | 158 | 17.2 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 --texture-cap 512 ×3 | 11.14 | 90 | 12.63 | 13.67 | 73 | 25.0 | 0/27, 0/24, 0/26 | 97 | 2580 | 92.4 | 77 | 8.99 | 47.0 k | 24.6 k | 210 | 24.77 | 32.40 | 4 | 14.6 k | 124 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --texture-cap 512 #1 | 13.85 | 72 | 17.87 | 19.98 | 50 | 39.9 | 1/16 | 72 | 2580 | 70.3 | 73 | 7.98 | 59.2 k | 31.6 k | 286 | 23.08 | 28.62 | 22 | 18.1 k | 157 | 17.4 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 --texture-cap 512 #2 | 13.87 | 72 | 17.58 | 19.86 | 50 | 36.4 | 1/17 | 73 | 2580 | 71.2 | 73 | 7.99 | 59.2 k | 31.6 k | 286 | 22.71 | 28.82 | 22 | 17.9 k | 164 | 17.0 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 --texture-cap 512 #3 | 13.55 | 74 | 18.05 | 20.33 | 49 | 39.8 | 2/13 | 73 | 2580 | 71.5 | 73 | 7.99 | 59.2 k | 31.6 k | 286 | 22.60 | 28.95 | 22 | 18.1 k | 158 | 17.2 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 --texture-cap 512 #1 | 11.74 | 85 | 13.23 | 14.29 | 70 | 18.3 | 0/27 | 98 | 2580 | 91.6 | 77 | 9.58 | 47.0 k | 24.6 k | 210 | 30.20 | 38.00 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 --texture-cap 512 #2 | 11.10 | 90 | 12.63 | 13.52 | 74 | 17.8 | 0/24 | 97 | 2580 | 93.0 | 77 | 8.96 | 47.0 k | 24.6 k | 210 | 24.71 | 32.37 | 4 | 14.6 k | 123 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 --texture-cap 512 #3 | 11.14 | 90 | 12.59 | 13.67 | 73 | 25.0 | 0/26 | 97 | 2580 | 92.4 | 77 | 8.99 | 47.0 k | 24.6 k | 210 | 24.77 | 32.40 | 4 | 14.6 k | 124 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
