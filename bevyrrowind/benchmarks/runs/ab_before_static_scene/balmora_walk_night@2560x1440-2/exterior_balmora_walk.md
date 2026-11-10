# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5624 frames over 65.6 s · **median 10.89 ms (92 fps)** · mean 11.67 · p95 16.14 · p99 18.66 (1% low 54 fps) · worst 37.61 ms · 259 spikes over 1.5× median · 22.8 stutters/min (work 25.6/min)
- warm-up: 338 frames over 4.2 s, worst 506.2 ms
- hitches (> 27.2 ms): **1** measured, 5 in warm-up
- timed GPU passes: 7.36 ms of a 11.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.89 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 37.61 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.246 | 2.800 | 0.192 |
| main_opaque_pass_3d | 1.300 | 1.938 | 0.047 |
| early prepass | 0.504 | 0.883 | 0.085 |
| lightprobe_irradiance_map | 0.445 | 0.706 | 0.001 |
| bloom | 0.343 | 0.546 | 0.072 |
| atmosphere_luts | 0.339 | 0.525 | 0.003 |
| auxiliary_views | 0.291 | 1.287 | 0.269 |
| lightprobe_radiance_map | 0.231 | 0.237 | 0.005 |
| ssao | 0.226 | 0.360 | 0.008 |
| uniform_allocation | 0.173 | 0.406 | 0.018 |
| upscaling | 0.134 | 0.148 | 0.005 |
| water_fft | 0.133 | 0.132 | 0.003 |
| bin_unpacking | 0.111 | 0.126 | 0.014 |
| tonemapping | 0.092 | 0.091 | 0.004 |
| water_velocity | 0.086 | 0.085 | 0.002 |
| atmos_froxel_inject | 0.082 | 0.084 | 0.002 |
| clustering | 0.072 | 0.069 | 0.021 |
| main_transparent_pass_3d | 0.068 | 0.119 | 0.112 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| clouds_trace | 0.053 | 0.071 | 0.001 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.029 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.002 |
| water_mips | 0.024 | 0.024 | 0.002 |
| lightprobe_copy | 0.018 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.016 | 0.014 | 0.001 |
| night_sky | 0.015 | 0.030 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.004 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.145 | 14.592 | 35.78 | 57054.9 |
| render/busy/ms | 9.584 | 13.953 | 34.85 | 53897.6 |
| main/busy/ms | 8.481 | 11.093 | 24.78 | 47698.9 |
| render/graph/ms | 4.732 | 7.692 | 19.42 | 26612.5 |
| bigspace/total/ms | 0.458 | 0.769 | 2.18 | 2577.8 |
| render/present/ms | 0.458 | 0.598 | 2.02 | 2573.6 |
| terrain/update/ms | 0.292 | 0.379 | 1.74 | 1642.9 |
| controller/feed/ms | 0.262 | 0.378 | 0.89 | 1470.8 |
| terrain/select/ms | 0.258 | 0.333 | 0.71 | 1450.3 |
| stream/drive/ms | 0.242 | 0.409 | 2.71 | 1359.4 |
| far/select/ms | 0.223 | 0.371 | 0.84 | 1252.1 |
| stream/tick/ms | 0.195 | 0.312 | 0.87 | 1094.8 |
| bigspace/low_precision/ms | 0.177 | 0.303 | 1.84 | 998.0 |
| anim/evaluate_cpu/ms | 0.173 | 0.231 | 0.71 | 974.2 |
| controller/move/ms | 0.169 | 0.382 | 1.03 | 950.3 |
| anim/evaluate/ms | 0.167 | 0.313 | 1.52 | 939.0 |
| bigspace/high_precision/ms | 0.108 | 0.208 | 1.48 | 605.7 |
| render/acquire/ms | 0.104 | 0.139 | 1.38 | 583.6 |
| anim/apply/ms | 0.094 | 0.162 | 1.41 | 526.0 |
| physics/step/ms | 0.053 | 0.094 | 0.91 | 299.9 |
| physics/insert/ms | 0.051 | 0.427 | 1.06 | 285.3 |
| weather/map/ms | 0.017 | 0.022 | 0.17 | 95.2 |
| controller/avoid/ms | 0.012 | 0.018 | 0.33 | 67.1 |
| weather/eval/ms | 0.010 | 0.013 | 0.15 | 57.3 |
| script/update/ms | 0.007 | 0.011 | 0.07 | 40.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.18 | 31.9 |
| script/input/ms | 0.006 | 0.008 | 0.18 | 31.7 |
| script/ui/ms | 0.005 | 0.008 | 0.35 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.25 | 27.3 |
| controller/write/ms | 0.004 | 0.006 | 0.06 | 20.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 5.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 47 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 131 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.91 | 34.85 | 10.66 |
| gpu/timed/ms | ms | 7.30 | 11.86 | 7.81 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 2315 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 173 | 173 |
| pipelines/waiting | count | 0 | 36 | 0 |
| process/cpu_usage | % | 24.52 | 36.61 | 17.31 |
| process/mem_usage | GiB | 2.52 | 2.56 | 2.56 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3478 | 3530 | 3530 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.77 | 84.67 | 45.25 |
| system/mem_usage | % | 47.08 | 47.44 | 47.44 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 301 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.01 | 1683 | 270.0 k |
| anim/events/count | count | 0.21 | 4 | 1169 |
| assets/Image/added | count | 0.08 | 36 | 474 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 160 | 4462 |
| assets/Mesh/removed | count | 0.87 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.11 | 37 | 636 |
| assets/StandardMaterial/modified | count | 3.85 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| ecs/entities_delta | count | 3.57 | 6074 | 20.1 k |
| physics/inserts | count | 0.76 | 183 | 4267 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6019 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1320 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 506.2 | main/busy/ms 483, surface/advance_cpu_ms 0.74, surface/cell_upload_ms 0.40, stream/drive/ms 0.27, bigspace/total/ms 0.08, stream/tick/ms 0.07, weather/map/ms 0.05, bigspace/high_precision/ms 0.03, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.74 | 237.7 | main/busy/ms 521, render/thread/ms 203, render/busy/ms 156, render/acquire/ms 45.15, render/graph/ms 6.99, render/present/ms 1.81, stream/drive/ms 1.25, surface/advance_cpu_ms 0.74, stream/tick/ms 0.67, terrain/update/ms 0.45, surface/cell_upload_ms 0.40, terrain/select/ms 0.32 |
| warm-up | 0.89 | 144.5 | render/thread/ms 219, render/busy/ms 171, main/busy/ms 55.11, render/acquire/ms 45.51, terrain/update/ms 23.56, render/graph/ms 11.42, render/present/ms 3.11, terrain/select/ms 1.44, stream/drive/ms 1.43, stream/tick/ms 0.97, bigspace/total/ms 0.79, bigspace/high_precision/ms 0.74 |
| warm-up | 1.13 | 82.9 | main/busy/ms 59.53, render/thread/ms 46.49, render/busy/ms 43.66, render/graph/ms 4.75, render/present/ms 2.21, stream/drive/ms 1.74, stream/tick/ms 1.50, terrain/update/ms 0.72, render/acquire/ms 0.62, terrain/select/ms 0.59, bigspace/total/ms 0.27, bigspace/high_precision/ms 0.13 |
| warm-up | 2.30 | 32.3 | main/busy/ms 37.19, render/thread/ms 23.64, render/busy/ms 22.45, render/graph/ms 5.21, stream/drive/ms 3.65, stream/spawn/ms 1.97, render/present/ms 1.02, stream/tick/ms 0.75, bigspace/total/ms 0.68, terrain/update/ms 0.54, terrain/select/ms 0.47, bigspace/high_precision/ms 0.33 |
| measure | 2.73 | 37.6 | render/thread/ms 54.46, render/busy/ms 52.64, main/busy/ms 29.99, render/graph/ms 29.49, render/present/ms 1.55, bigspace/total/ms 1.25, physics/insert/ms 0.92, controller/feed/ms 0.86, terrain/update/ms 0.72, stream/drive/ms 0.69, terrain/select/ms 0.64, stream/tick/ms 0.62 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **72%** (min 43%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.5 W · 71 °C (max 76) · VRAM 3759 MiB · throttled 100% of samples
