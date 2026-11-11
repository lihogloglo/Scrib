# Perf suite `fix_off`

- exe: `../target/profiling/bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 ×3 | 9.31 | 107 | 10.42 | 11.38 | 88 | 53.4 | 2/21, 0/12, 1/7 | 97 | 2580 | 93.0 | 69 | 8.36 | 47.0 k | 24.6 k | 210 | 25.90 | 33.49 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 ×3 | 10.71 | 93 | 13.31 | 14.80 | 68 | 26.5 | 0/4, 0/5, 0/3 | 76 | 2580 | 76.2 | 69 | 7.17 | 59.2 k | 31.7 k | 286 | 23.94 | 29.58 | 22 | 18.3 k | 84 | 17.4 k | 188 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 #1 | 9.40 | 106 | 10.85 | 12.22 | 82 | 53.4 | 2/21 | 97 | 2586 | 90.6 | 65 | 8.38 | 47.0 k | 24.6 k | 210 | 27.40 | 34.51 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 9.30 | 108 | 10.39 | 11.38 | 88 | 15.1 | 0/12 | 98 | 2580 | 93.4 | 69 | 8.36 | 47.0 k | 24.6 k | 210 | 25.90 | 33.49 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 9.31 | 107 | 10.42 | 11.03 | 91 | 39.4 | 1/7 | 97 | 2580 | 93.0 | 70 | 8.28 | 47.0 k | 24.6 k | 210 | 24.04 | 31.58 | 4 | 14.6 k | 67 | 14.8 k | 154 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #1 | 10.84 | 92 | 13.31 | 14.95 | 67 | 24.4 | 0/4 | 76 | 2583 | 75.7 | 68 | 7.17 | 59.2 k | 31.7 k | 286 | 23.94 | 29.27 | 22 | 18.2 k | 85 | 17.4 k | 191 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 10.69 | 94 | 13.39 | 14.80 | 68 | 26.5 | 0/5 | 76 | 2580 | 76.2 | 69 | 7.13 | 59.2 k | 31.8 k | 286 | 23.92 | 29.63 | 22 | 18.4 k | 81 | 17.5 k | 182 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 10.71 | 93 | 13.17 | 14.65 | 68 | 22.9 | 0/3 | 77 | 2580 | 76.7 | 69 | 7.18 | 59.2 k | 31.6 k | 286 | 24.52 | 29.58 | 22 | 18.3 k | 84 | 17.4 k | 188 | 0 | 0 | 0 | 0 |
