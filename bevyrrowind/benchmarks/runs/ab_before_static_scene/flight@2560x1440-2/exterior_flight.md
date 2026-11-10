# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5500 frames over 71.5 s · **median 12.65 ms (79 fps)** · mean 13.00 · p95 16.85 · p99 19.90 (1% low 50 fps) · worst 53.09 ms · 80 spikes over 1.5× median · 55.4 stutters/min (work 79.7/min)
- warm-up: 520 frames over 5.7 s, worst 576.2 ms
- hitches (> 31.6 ms): **4** measured, 4 in warm-up
- timed GPU passes: 8.28 ms of a 13.00 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.65 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.19 min = 55.37/min, limit 1/min
- **FAIL** worst frame 53.09 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.224 | 2.888 | 0.065 |
| dlss_super_resolution | 2.095 | 2.423 | 0.192 |
| early prepass | 0.592 | 1.039 | 0.101 |
| lightprobe_irradiance_map | 0.436 | 0.648 | 0.001 |
| atmosphere_luts | 0.382 | 0.555 | 0.004 |
| bloom | 0.317 | 0.319 | 0.074 |
| lightprobe_radiance_map | 0.226 | 0.233 | 0.004 |
| clouds_trace | 0.224 | 0.262 | 0.001 |
| ssao | 0.214 | 0.211 | 0.008 |
| main_transparent_pass_3d | 0.211 | 0.472 | 0.091 |
| uniform_allocation | 0.176 | 0.207 | 0.024 |
| water_fft | 0.130 | 0.130 | 0.003 |
| bin_unpacking | 0.129 | 0.156 | 0.018 |
| upscaling | 0.114 | 0.122 | 0.005 |
| atmos_froxel_inject | 0.090 | 0.093 | 0.002 |
| tonemapping | 0.088 | 0.090 | 0.004 |
| water_velocity | 0.077 | 0.095 | 0.002 |
| clouds_composite | 0.066 | 0.068 | 0.002 |
| clustering | 0.065 | 0.068 | 0.023 |
| surface_trail | 0.064 | 0.059 | 0.003 |
| render_sky | 0.052 | 0.052 | 0.003 |
| early_mesh_preprocessing | 0.038 | 0.041 | 0.006 |
| clouds_resolve | 0.036 | 0.044 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.029 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.031 | 0.008 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| weather_map | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.002 |
| auxiliary_views | 0.012 | 0.002 | 0.014 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.004 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.117 | 14.557 | 40.18 | 61143.5 |
| render/busy/ms | 10.547 | 13.967 | 30.00 | 58008.8 |
| main/busy/ms | 8.609 | 10.984 | 61.72 | 47347.6 |
| render/graph/ms | 4.995 | 6.321 | 20.76 | 27472.8 |
| render/present/ms | 0.468 | 0.608 | 10.88 | 2574.9 |
| bigspace/total/ms | 0.449 | 0.822 | 3.79 | 2470.9 |
| stream/drive/ms | 0.338 | 0.907 | 2.84 | 1856.4 |
| controller/feed/ms | 0.268 | 0.423 | 0.98 | 1476.0 |
| terrain/update/ms | 0.265 | 0.507 | 2.65 | 1454.8 |
| far/select/ms | 0.244 | 0.443 | 0.96 | 1339.4 |
| terrain/select/ms | 0.195 | 0.258 | 1.74 | 1072.6 |
| stream/tick/ms | 0.187 | 0.309 | 0.75 | 1029.0 |
| controller/move/ms | 0.161 | 0.413 | 1.00 | 887.3 |
| bigspace/low_precision/ms | 0.154 | 0.325 | 3.49 | 847.5 |
| anim/evaluate_cpu/ms | 0.141 | 0.237 | 0.93 | 774.4 |
| anim/apply/ms | 0.118 | 0.219 | 4.96 | 647.0 |
| bigspace/high_precision/ms | 0.110 | 0.205 | 1.80 | 607.5 |
| anim/evaluate/ms | 0.109 | 0.227 | 2.04 | 599.1 |
| render/acquire/ms | 0.102 | 0.143 | 0.78 | 559.8 |
| physics/step/ms | 0.094 | 0.240 | 1.87 | 516.7 |
| stream/despawn/ms | 0.052 | 0.508 | 2.29 | 284.9 |
| stream/spawn/ms | 0.041 | 0.334 | 1.59 | 227.5 |
| weather/map/ms | 0.028 | 0.038 | 0.28 | 155.8 |
| physics/insert/ms | 0.024 | 0.174 | 0.91 | 132.2 |
| controller/avoid/ms | 0.011 | 0.018 | 0.17 | 61.2 |
| weather/eval/ms | 0.011 | 0.016 | 0.20 | 59.1 |
| script/update/ms | 0.008 | 0.012 | 0.26 | 45.5 |
| stream/deactivate/ms | 0.007 | 0.000 | 0.63 | 36.5 |
| script/input/ms | 0.006 | 0.009 | 0.13 | 34.2 |
| stream/activate/ms | 0.006 | 0.000 | 0.59 | 32.2 |
| script/ui/ms | 0.005 | 0.008 | 0.11 | 28.4 |
| controller/write/ms | 0.004 | 0.006 | 0.16 | 19.5 |
| stream/upload/ms | 0.002 | 0.015 | 0.21 | 10.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.21 | 6.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.08 | 5.7 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.05 | 3.6 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 3.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 604 | 778 | 471 |
| anim/characters/count | count | 604 | 782 | 471 |
| anim/evaluated/count | count | 19 | 60 | 14 |
| controller/movers/count | count | 19 | 32 | 14 |
| ecs/entities | count | 60.0 k | 72.5 k | 52.9 k |
| far/entities | count | 2681 | 3461 | 2126 |
| far/forced | count | 0 | 14 | 0 |
| far/impostors | count | 61.0 k | 65.7 k | 55.8 k |
| far/lights | count | 7115 | 7757 | 6575 |
| far/nodes/l1 | count | 1100 | 1439 | 857 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 227.79 M | 173.66 M |
| far/triangles | count | 5.04 M | 5.72 M | 4.26 M |
| frame/work/ms | ms | 10.19 | 61.72 | 8.65 |
| gpu/timed/ms | ms | 8.28 | 12.27 | 7.21 |
| mesh_allocator_allocations |  meshes | 15.8 k | 19.2 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 537.07 M | 645.72 M | 645.72 M |
| physics/awake | count | 0 | 4 | 0 |
| physics/bodies | count | 1897 | 2692 | 1955 |
| physics/items | count | 12 | 106 | 106 |
| physics/pending | count | 0 | 447 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 5 | 0 |
| process/cpu_usage | % | 22.60 | 41.15 | 21.76 |
| process/mem_usage | GiB | 2.39 | 2.59 | 2.59 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3879 | 4858 | 4858 |
| stream/activate/queue | count | 0 | 3 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 68 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 201 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.63 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 171 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 9 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.07 | 92.18 | 47.84 |
| system/mem_usage | % | 48.97 | 50.13 | 48.72 |
| terrain/patches | count | 349 | 496 | 319 |
| world/meshes | count | 26.5 k | 33.5 k | 28.1 k |
| world/point_lights | count | 270 | 549 | 524 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 9.74 | 228 | 53.6 k |
| anim/events/count | count | 0.24 | 5 | 1310 |
| assets/Image/added | count | 0.15 | 38 | 830 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 7.30 | 494 | 40.2 k |
| assets/Mesh/removed | count | 12.03 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.20 | 39 | 1092 |
| assets/StandardMaterial/modified | count | 6.20 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.09 | 36 | 470 |
| assets/WorldAsset/added | count | 0.02 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 10 | 124 |
| ecs/entities_delta | count | 3.37 | 2243 | 18.5 k |
| physics/inserts | count | 2.70 | 145 | 14.9 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.58 | 275 | 14.2 k |
| physics/steps | count | 0.78 | 3 | 4291 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.24 | 23 | 1345 |
| stream/deactivate/items | count | 0.24 | 22 | 1345 |
| stream/despawn/items | count | 1.27 | 290 | 6982 |
| stream/fetch/items | count | 0.27 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.83 | 34 | 4586 |
| stream/upload/bytes | bytes | 90.0 k | 3.77 M | 495.17 M |
| stream/upload/items | count | 0.23 | 15 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.05 | 1 | 269 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 20.9 k | 1.21 M | 114.82 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 31.6 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 576.2 | main/busy/ms 555, surface/advance_cpu_ms 67.71, surface/cell_upload_ms 1.02, stream/drive/ms 0.31, weather/map/ms 0.13, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, weather/apply/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.91 | 329.9 | main/busy/ms 585, render/thread/ms 295, render/busy/ms 249, surface/advance_cpu_ms 67.71, render/acquire/ms 44.33, render/graph/ms 7.42, render/present/ms 1.62, surface/cell_upload_ms 1.02, stream/drive/ms 0.80, stream/tick/ms 0.40, terrain/update/ms 0.35, weather/map/ms 0.24 |
| warm-up | 0.94 | 32.1 | render/thread/ms 311, render/busy/ms 263, main/busy/ms 45.28, render/acquire/ms 44.46, terrain/update/ms 16.91, render/graph/ms 12.05, render/present/ms 3.05, stream/drive/ms 0.87, stream/tick/ms 0.62, terrain/select/ms 0.39, bigspace/total/ms 0.20, weather/map/ms 0.13 |
| warm-up | 1.17 | 86.8 | main/busy/ms 66.11, render/thread/ms 47.66, render/busy/ms 45.99, render/graph/ms 4.32, stream/drive/ms 1.54, render/present/ms 1.40, stream/tick/ms 1.24, terrain/update/ms 0.66, terrain/select/ms 0.51, bigspace/total/ms 0.36, render/acquire/ms 0.28, bigspace/high_precision/ms 0.21 |
| measure | 35.10 | 34.3 | render/thread/ms 46.94, render/busy/ms 44.81, main/busy/ms 26.32, render/graph/ms 21.11, bigspace/total/ms 2.02, render/present/ms 1.84, stream/drive/ms 1.74, controller/move/ms 1.37, stream/despawn/ms 1.12, bigspace/high_precision/ms 0.73, controller/feed/ms 0.71, anim/apply/ms 0.58 |
| measure | 35.15 | 33.4 | render/thread/ms 50.24, render/busy/ms 48.33, render/graph/ms 27.68, main/busy/ms 27.54, render/present/ms 1.59, bigspace/total/ms 1.02, stream/drive/ms 1.02, terrain/update/ms 0.85, controller/move/ms 0.84, terrain/select/ms 0.60, far/select/ms 0.59, controller/feed/ms 0.58 |
| measure | 70.00 | 43.1 | main/busy/ms 69.71, render/thread/ms 53.54, render/busy/ms 41.85, render/graph/ms 26.36, render/present/ms 11.50, bigspace/total/ms 1.59, controller/feed/ms 0.80, bigspace/low_precision/ms 0.76, far/select/ms 0.56, stream/drive/ms 0.50, stream/tick/ms 0.44, terrain/update/ms 0.43 |
| measure | 70.05 | 53.1 | main/busy/ms 79.42, render/thread/ms 58.49, render/busy/ms 46.22, render/graph/ms 25.67, render/present/ms 11.38, bigspace/total/ms 1.60, render/acquire/ms 0.89, bigspace/low_precision/ms 0.84, controller/feed/ms 0.78, far/select/ms 0.64, physics/step/ms 0.59, terrain/update/ms 0.49 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\flight@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (278 samples): GPU load **72%** (min 42%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.6 W · 72 °C (max 76) · VRAM 3966 MiB · throttled 100% of samples
