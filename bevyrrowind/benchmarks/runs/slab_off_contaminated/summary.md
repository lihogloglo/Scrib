# Perf suite `slab_off`

- exe: `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: none · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up; `balmora_walk` walk at 4 m/s, eye height (path grid + 1.7 m), 254 m of Balmora streets and terraces (the gameplay view)
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 ×3 | 13.64 | 73 | 79.52 | 86.41 | 12 | 288.4 | 666/372, 3/20, 0/3 | 95 | 2580 | 57.2 | 70 | 18.60 | 59.2 k | 31.6 k | 286 | 17.23 | 28.93 | 22 | 18.0 k | 164 | 17.0 k | 13 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 ×3 | 49.88 | 20 | 54.17 | 56.57 | 18 | 78.4 | 0/4, 0/3, 0/27 | 98 | 2580 | 55.6 | 70 | 37.16 | 47.0 k | 24.6 k | 210 | 17.97 | 34.01 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups | scene cells | scene batches | scene live slots | scene slot bytes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_walk 2560x1440 #1 | 12.86 | 78 | 79.52 | 86.41 | 12 | 288.4 | 666/372 | 95 | 2580 | 57.2 | 70 | 18.60 | 58.7 k | 30.4 k | 286 | 17.23 | 28.84 | 4 | 13.5 k | 137 | 13.8 k | 11 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #2 | 13.64 | 73 | 17.59 | 20.14 | 50 | 46.3 | 3/20 | 74 | 2580 | 72.6 | 74 | 8.15 | 59.2 k | 31.6 k | 286 | 22.60 | 28.93 | 22 | 18.0 k | 164 | 17.0 k | 13 | 0 | 0 | 0 | 0 |
| balmora_walk 2560x1440 #3 | 79.73 | 13 | 111.99 | 119.37 | 8 | 158.2 | 0/3 | 98 | 2580 | 46.7 | 67 | 59.47 | 59.3 k | 31.8 k | 286 | 10.94 | 28.99 | 28 | 19.4 k | 181 | 18.0 k | 53 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #1 | 49.88 | 20 | 54.17 | 56.57 | 18 | 60.4 | 0/4 | 98 | 2580 | 55.6 | 68 | 37.16 | 47.0 k | 24.6 k | 210 | 17.97 | 37.59 | 4 | 14.6 k | 127 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #2 | 70.91 | 14 | 75.65 | 76.76 | 13 | 78.4 | 0/3 | 99 | 2580 | 50.2 | 70 | 56.05 | 47.0 k | 24.6 k | 210 | 9.79 | 29.64 | 4 | 14.6 k | 128 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
| balmora 2560x1440 #3 | 11.20 | 89 | 12.97 | 14.20 | 70 | 21.6 | 0/27 | 97 | 2580 | 91.4 | 78 | 9.02 | 47.0 k | 24.6 k | 210 | 28.37 | 34.01 | 4 | 14.6 k | 124 | 14.8 k | 0 | 0 | 0 | 0 | 0 |
