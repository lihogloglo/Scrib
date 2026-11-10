# Perf suite `au_night_noshadowfx`

- exe: `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` (release)
- GPU: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42)
- A/B off: softshadows,contactshadows · hitch threshold: 2.5 × median, ≥ 25 ms
- scenarios: `balmora_night` Balmora standing at 22:00, clear: torch-bearing walkers, far light sprites, moon shadows
- machine: quiet (before each run: no cargo/rustc/bevyrrowind/ovid process, GPU ≤ 15% at ≥ 600 MHz, CPU ≤ 20%; during: other processes ≤ 25% CPU)

## Across repeats (median of runs; worst = max)

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 off=softshadows,contactshadows ×3 | 8.68 | 115 | 10.07 | 11.28 | 89 | 18.3 | 0/6, 0/5, 0/4 | 95 | 2580 | 92.7 | 74 | 8.01 | 47.0 k | 24.7 k | 222 | 29.13 | 39.85 | 3 | 11.3 k | 60 | 11.5 k | 139 |

## Runs

| run | median ms | fps | p95 | p99 | 1% low fps | worst ms | hitches (meas./warm-up) | GPU % | GPU MHz | W | °C | timed GPU ms | entities | meshes | point lights | process CPU % | system CPU % | views | phase items | batch sets | work items | preprocess bind groups |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| balmora_night 2560x1440 off=softshadows,contactshadows #1 | 9.14 | 109 | 11.53 | 13.21 | 76 | 14.9 | 0/6 | 95 | 2580 | 90.9 | 75 | 8.33 | 47.0 k | 24.7 k | 222 | 29.13 | 45.22 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_night 2560x1440 off=softshadows,contactshadows #2 | 8.68 | 115 | 10.07 | 11.02 | 91 | 18.3 | 0/5 | 96 | 2580 | 92.7 | 72 | 8.01 | 47.0 k | 24.7 k | 222 | 28.83 | 39.85 | 3 | 11.3 k | 60 | 11.5 k | 139 |
| balmora_night 2560x1440 off=softshadows,contactshadows #3 | 8.61 | 116 | 10.05 | 11.28 | 89 | 13.4 | 0/4 | 95 | 2580 | 93.6 | 74 | 7.97 | 47.0 k | 24.7 k | 222 | 29.33 | 37.83 | 3 | 11.3 k | 60 | 11.5 k | 139 |
