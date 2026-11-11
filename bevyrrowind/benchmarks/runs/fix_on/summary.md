# Perf suite `fix_on`

- exe: `../target/profiling/bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 --scene ×3 | 9.44 | 106 | 10.60 | 11.54 | 87 | 16.7 | 0/8, 0/5, 0/5 | 97 | 2580 | 92.9 | 72 | 8.30 | 37.3 k | 15.0 k | 210 | 25.96 | 32.27 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora_walk 2560x1440 --scene ×3 | 11.10 | 90 | 13.58 | 15.11 | 66 | 26.7 | 0/5, 0/5, 0/5 | 76 | 2580 | 75.2 | 69 | 7.31 | 48.8 k | 20.5 k | 286 | 23.56 | 28.41 | 22 | 10.8 k | 99 | 256.4 k | 186 | 117 | 4 | 21.7 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 --scene #1 | 9.43 | 106 | 10.60 | 11.54 | 87 | 15.7 | 0/8 | 96 | 2580 | 93.2 | 73 | 8.31 | 37.3 k | 15.0 k | 210 | 23.85 | 29.79 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #2 | 9.45 | 106 | 10.64 | 11.68 | 86 | 14.8 | 0/5 | 97 | 2580 | 92.5 | 72 | 8.28 | 37.3 k | 15.0 k | 210 | 26.32 | 32.67 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #3 | 9.44 | 106 | 10.52 | 11.43 | 87 | 16.7 | 0/5 | 97 | 2580 | 92.9 | 72 | 8.30 | 37.3 k | 15.0 k | 210 | 25.96 | 32.27 | 4 | 9108 | 84 | 255.0 k | 155 | 101 | 4 | 19.2 k | 3.15 M |
| balmora_walk 2560x1440 --scene #1 | 11.09 | 90 | 13.58 | 15.11 | 66 | 25.0 | 0/5 | 76 | 2580 | 75.2 | 69 | 7.29 | 48.8 k | 20.5 k | 286 | 23.56 | 28.84 | 22 | 10.8 k | 99 | 256.3 k | 186 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #2 | 11.10 | 90 | 13.57 | 14.80 | 68 | 26.7 | 0/5 | 76 | 2580 | 75.8 | 69 | 7.31 | 48.8 k | 20.5 k | 286 | 23.56 | 28.41 | 22 | 10.8 k | 99 | 256.4 k | 185 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #3 | 11.24 | 89 | 14.33 | 15.50 | 65 | 23.4 | 0/5 | 75 | 2580 | 73.9 | 69 | 7.41 | 48.4 k | 20.5 k | 286 | 22.60 | 27.89 | 22 | 10.8 k | 101 | 256.4 k | 190 | 117 | 4 | 21.7 k | 3.15 M |
