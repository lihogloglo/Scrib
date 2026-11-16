# Perf suite `f_scene`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)
- focus: clean, all 6 runs (window in front for every measured frame)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene ×3 | 11.01 | 91 | 13.34 | 15.24 | 66 | 34.2 | 2/40, 3/32, 2/37 | 80 | 2580 | 78.8 | 73 | 7.33 | 48.7 k | 20.6 k | 286 | 25.96 | 31.24 | 45 | 10.7 k | 76 | 154.0 k | 18 | 117 | 4 | 21.7 k | 3.15 M |
| balmora 2560x1440 --scene ×3 | 9.97 | 100 | 11.44 | 12.26 | 82 | 24.2 | 0/35, 0/41, 0/40 | 98 | 2580 | 93.4 | 73 | 8.73 | 37.3 k | 15.1 k | 210 | 26.08 | 31.60 | 3 | 9381 | 65 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 --scene #1 | 11.01 | 91 | 13.34 | 15.09 | 66 | 34.0 | 2/40 | 80 | 2580 | 78.9 | 72 | 7.24 | 48.7 k | 20.6 k | 286 | 25.52 | 31.16 | 45 | 10.7 k | 74 | 154.0 k | 17 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #2 | 11.01 | 91 | 13.36 | 15.24 | 66 | 34.2 | 3/32 | 80 | 2580 | 78.8 | 73 | 7.33 | 48.7 k | 20.6 k | 286 | 25.96 | 31.30 | 45 | 10.8 k | 76 | 154.1 k | 18 | 117 | 4 | 21.7 k | 3.15 M |
| balmora_walk 2560x1440 --scene #3 | 11.08 | 90 | 13.34 | 15.45 | 65 | 32.9 | 2/37 | 80 | 2580 | 78.5 | 73 | 7.36 | 48.7 k | 20.6 k | 286 | 25.96 | 31.24 | 45 | 10.7 k | 77 | 154.0 k | 18 | 117 | 4 | 21.7 k | 3.15 M |
| balmora 2560x1440 --scene #1 | 9.96 | 100 | 11.38 | 12.03 | 83 | 16.6 | 0/35 | 98 | 2580 | 93.9 | 74 | 8.73 | 37.3 k | 15.1 k | 210 | 26.08 | 31.60 | 3 | 9381 | 65 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #2 | 10.01 | 100 | 11.44 | 12.26 | 82 | 24.2 | 0/41 | 98 | 2580 | 93.2 | 73 | 8.75 | 37.3 k | 15.1 k | 210 | 26.39 | 32.30 | 3 | 9381 | 65 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
| balmora 2560x1440 --scene #3 | 9.97 | 100 | 11.47 | 12.31 | 81 | 15.9 | 0/40 | 97 | 2580 | 93.4 | 73 | 8.70 | 37.3 k | 15.1 k | 210 | 23.08 | 29.00 | 3 | 9381 | 63 | 153.8 k | 5 | 101 | 4 | 19.2 k | 3.15 M |
