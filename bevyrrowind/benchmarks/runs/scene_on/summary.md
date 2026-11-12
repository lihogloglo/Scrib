# Perf suite `scene_on`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene ×3 | 14.50 | 69 | 18.04 | 20.93 | 48 | 62.4 | 0/4, 0/5, 1/5 | 65 | 2580 | 63.5 | 66 | 7.78 | 70.6 k | 42.7 k | 286 | 21.63 | 35.85 | 22 | 18.2 k | 101 | 99.8 k | 189 | 117 | 4 | 9654 | 1.05 M |
| balmora 2560x1440 --scene ×3 | 11.23 | 89 | 12.92 | 14.57 | 69 | 70.3 | 0/5, 0/5, 6/7 | 88 | 2581 | 81.2 | 69 | 8.73 | 56.8 k | 34.5 k | 210 | 22.49 | 33.12 | 4 | 13.5 k | 84 | 95.6 k | 155 | 101 | 4 | 8782 | 1.05 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene #1 | 14.43 | 69 | 18.04 | 21.39 | 47 | 30.6 | 0/4 | 65 | 2580 | 63.5 | 66 | 7.78 | 70.6 k | 42.7 k | 286 | 21.67 | 34.77 | 22 | 18.2 k | 102 | 99.8 k | 192 | 117 | 4 | 9654 | 1.05 M |
| balmora_walk 2560x1440 --scene #2 | 14.50 | 69 | 17.92 | 20.71 | 48 | 30.4 | 0/5 | 63 | 2580 | 63.5 | 66 | 7.66 | 70.6 k | 42.7 k | 286 | 21.63 | 35.85 | 22 | 18.5 k | 99 | 100.2 k | 187 | 117 | 4 | 9654 | 1.05 M |
| balmora_walk 2560x1440 --scene #3 | 14.90 | 67 | 18.55 | 20.93 | 48 | 62.4 | 1/5 | 67 | 2589 | 64.2 | 65 | 8.18 | 70.7 k | 42.8 k | 286 | 21.15 | 36.27 | 22 | 18.0 k | 101 | 99.7 k | 189 | 117 | 4 | 9654 | 1.05 M |
| balmora 2560x1440 --scene #1 | 11.81 | 85 | 13.14 | 14.57 | 69 | 20.1 | 0/5 | 84 | 2589 | 79.0 | 66 | 8.73 | 56.8 k | 34.5 k | 210 | 21.63 | 33.12 | 4 | 13.5 k | 84 | 95.6 k | 155 | 101 | 4 | 8782 | 1.05 M |
| balmora 2560x1440 --scene #2 | 11.23 | 89 | 12.69 | 13.81 | 72 | 17.1 | 0/5 | 88 | 2580 | 82.7 | 70 | 8.73 | 56.8 k | 34.5 k | 210 | 22.60 | 32.92 | 4 | 13.9 k | 84 | 96.0 k | 155 | 101 | 4 | 8782 | 1.05 M |
| balmora 2560x1440 --scene #3 | 11.23 | 89 | 12.92 | 18.85 | 53 | 70.3 | 6/7 | 88 | 2581 | 81.2 | 69 | 9.01 | 56.8 k | 34.5 k | 210 | 22.49 | 33.55 | 4 | 13.5 k | 84 | 95.5 k | 155 | 101 | 4 | 8782 | 1.05 M |
