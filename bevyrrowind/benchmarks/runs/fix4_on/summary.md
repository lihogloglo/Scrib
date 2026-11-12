# Perf suite `fix4_on`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 --scene ×3 | 9.60 | 104 | 10.75 | 11.66 | 86 | 26.3 | 0/7, 1/4, 0/8 | 97 | 2580 | 92.8 | 74 | 8.53 | 37.3 k | 15.0 k | 210 | 24.52 | 31.90 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora_walk 2560x1440 --scene ×3 | 11.07 | 90 | 13.42 | 14.92 | 67 | 33.6 | 2/5, 1/6, 1/5 | 80 | 2580 | 77.3 | 72 | 7.72 | 48.6 k | 20.5 k | 286 | 23.08 | 29.74 | 22 | 10.9 k | 99 | 256.4 k | 186 | 117 | 4 | 21.7 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 --scene #1 | 9.89 | 101 | 11.03 | 11.75 | 85 | 20.4 | 0/7 | 97 | 2580 | 92.3 | 76 | 8.74 | 37.3 k | 15.0 k | 210 | 24.52 | 32.92 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #2 | 9.60 | 104 | 10.75 | 11.66 | 86 | 26.3 | 1/4 | 97 | 2580 | 92.8 | 74 | 8.53 | 37.3 k | 15.0 k | 210 | 23.08 | 30.29 | 4 | 9107 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #3 | 9.58 | 104 | 10.74 | 11.36 | 88 | 21.2 | 0/8 | 97 | 2580 | 93.2 | 74 | 8.53 | 37.3 k | 15.0 k | 210 | 25.77 | 31.90 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora_walk 2560x1440 --scene #1 | 11.26 | 89 | 13.38 | 15.26 | 66 | 33.6 | 2/5 | 79 | 2580 | 77.3 | 72 | 7.68 | 48.6 k | 20.7 k | 286 | 22.60 | 29.97 | 22 | 10.9 k | 99 | 256.4 k | 186 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #2 | 11.07 | 90 | 13.42 | 14.83 | 67 | 30.3 | 1/6 | 81 | 2580 | 78.3 | 72 | 7.79 | 48.6 k | 20.5 k | 286 | 23.16 | 29.60 | 22 | 10.9 k | 99 | 256.3 k | 186 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #3 | 11.06 | 90 | 13.59 | 14.92 | 67 | 28.9 | 1/5 | 80 | 2580 | 76.9 | 71 | 7.72 | 48.6 k | 20.5 k | 286 | 23.08 | 29.74 | 22 | 10.8 k | 101 | 256.4 k | 189 | 117 | 4 | 21.7 k | 3.15 M |
