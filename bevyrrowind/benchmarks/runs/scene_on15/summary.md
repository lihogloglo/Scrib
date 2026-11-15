# Perf suite `scene_on15`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b4.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene ×3 | 11.75 | 85 | 14.18 | 16.06 | 62 | 34.5 | 2/21, 3/16, 4/20 | 80 | 2580 | 78.5 | 73 | 7.42 | 48.6 k | 20.6 k | 286 | 25 | 30.65 | 46 | 11.2 k | 103 | 256.3 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora 2560x1440 --scene ×3 | 10.54 | 95 | 11.80 | 12.73 | 79 | 21.4 | 0/19, 0/21, 0/23 | 97 | 2580 | 93.1 | 75 | 8.61 | 37.3 k | 15.1 k | 210 | 28.33 | 35.30 | 4 | 9108 | 92 | 255.0 k | 7 | 101 | 4 | 19.2 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene #1 | 11.43 | 87 | 14.18 | 16.06 | 62 | 34.0 | 2/21 | 82 | 2580 | 79.6 | 73 | 7.35 | 48.5 k | 20.5 k | 286 | 25 | 30.65 | 46 | 11.2 k | 103 | 256.4 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #2 | 11.75 | 85 | 13.93 | 15.72 | 64 | 34.5 | 3/16 | 80 | 2580 | 78.5 | 73 | 7.42 | 48.7 k | 20.6 k | 286 | 25 | 30.55 | 46 | 11.2 k | 103 | 256.3 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #3 | 11.84 | 84 | 14.50 | 17.13 | 58 | 34.0 | 4/20 | 79 | 2580 | 77.9 | 73 | 7.45 | 48.6 k | 20.6 k | 286 | 24.29 | 30.75 | 46 | 11.2 k | 103 | 256.3 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora 2560x1440 --scene #1 | 10.52 | 95 | 11.89 | 12.73 | 79 | 17.2 | 0/19 | 97 | 2580 | 93.0 | 76 | 8.52 | 37.3 k | 15.1 k | 210 | 28.33 | 36.47 | 4 | 9108 | 92 | 255.0 k | 7 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #2 | 10.54 | 95 | 11.80 | 12.63 | 79 | 18.0 | 0/21 | 97 | 2580 | 93.1 | 75 | 8.64 | 37.3 k | 15.1 k | 210 | 30.92 | 35.30 | 4 | 9108 | 92 | 255.0 k | 7 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #3 | 10.54 | 95 | 11.76 | 12.80 | 78 | 21.4 | 0/23 | 97 | 2580 | 93.3 | 75 | 8.61 | 37.3 k | 15.1 k | 210 | 22.84 | 27.95 | 4 | 9108 | 92 | 255.0 k | 7 | 101 | 4 | 19.2 k | 3.15 M |
