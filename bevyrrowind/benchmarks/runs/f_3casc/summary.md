# Perf suite `f_3casc`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 11.44 | 87 | 14.42 | 16.88 | 59 | 32.3 | 2/35, 2/42, 2/47 | 75 | 2580 | 76.2 | 72 | 7.37 | 59.3 k | 31.7 k | 286 | 26.44 | 33.82 | 46 | 20.1 k | 61 | 10.6 k | 14 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 10.05 | 100 | 11.52 | 12.10 | 83 | 20.4 | 0/55, 0/44, 0/59 | 98 | 2580 | 95.7 | 74 | 8.95 | 47.1 k | 24.7 k | 210 | 26.52 | 32.44 | 4 | 15.2 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 11.66 | 86 | 14.42 | 16.88 | 59 | 32.3 | 2/35 | 75 | 2580 | 76.1 | 71 | 7.37 | 59.3 k | 31.7 k | 286 | 26.44 | 33.97 | 46 | 20.1 k | 62 | 10.8 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 11.43 | 88 | 14.26 | 16.86 | 59 | 32.0 | 2/42 | 76 | 2580 | 76.4 | 72 | 7.43 | 59.3 k | 31.8 k | 286 | 26.89 | 33.82 | 46 | 20.1 k | 61 | 10.6 k | 14 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 11.44 | 87 | 14.82 | 17.45 | 57 | 28.8 | 2/47 | 75 | 2580 | 76.2 | 73 | 7.20 | 59.3 k | 31.7 k | 286 | 26.44 | 33.37 | 46 | 20.1 k | 61 | 10.6 k | 14 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 9.96 | 100 | 11.52 | 12.07 | 83 | 17.4 | 0/55 | 98 | 2580 | 96.4 | 75 | 8.95 | 47.1 k | 24.7 k | 210 | 25.23 | 28.91 | 4 | 15.2 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 10.06 | 99 | 11.57 | 12.52 | 80 | 20.4 | 0/44 | 98 | 2580 | 94.9 | 74 | 8.84 | 47.1 k | 24.7 k | 210 | 26.52 | 33.16 | 4 | 15.2 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 10.05 | 100 | 11.51 | 12.10 | 83 | 16.8 | 0/59 | 98 | 2580 | 95.7 | 74 | 9.00 | 47.1 k | 24.7 k | 210 | 28.12 | 32.44 | 4 | 15.2 k | 53 | 9496 | 0 | 0 | 0 | 0 | 0 |
