# Perf suite `au_noshadowfx`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: softshadows,contactshadows · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora` Balmora standing: default view, 150 m south-east, 60 m up
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=softshadows,contactshadows ×3 | 9.93 | 101 | 12.37 | 13.92 | 72 | 71.0 | 0/6, 2/5, 0/6 | 92 | 2580 | 85.4 | 71 | 8.60 | 46.9 k | 24.6 k | 210 | 27.88 | 49.31 | 4 | 14.6 k | 67 | 14.8 k | 154 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora 2560x1440 off=softshadows,contactshadows #1 | 10.16 | 98 | 12.95 | 15.35 | 65 | 21.0 | 0/6 | 92 | 2580 | 85.4 | 71 | 8.59 | 46.9 k | 24.6 k | 210 | 29.33 | 49.31 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora 2560x1440 off=softshadows,contactshadows #2 | 9.93 | 101 | 12.37 | 13.76 | 73 | 71.0 | 2/5 | 91 | 2581 | 85.0 | 71 | 8.66 | 46.9 k | 24.6 k | 210 | 27.88 | 51.50 | 4 | 14.6 k | 67 | 14.8 k | 154 |
| balmora 2560x1440 off=softshadows,contactshadows #3 | 9.58 | 104 | 11.81 | 13.92 | 72 | 18.2 | 0/6 | 96 | 2580 | 89.5 | 71 | 8.60 | 46.9 k | 24.6 k | 210 | 26.92 | 45.18 | 4 | 14.6 k | 67 | 14.8 k | 154 |
