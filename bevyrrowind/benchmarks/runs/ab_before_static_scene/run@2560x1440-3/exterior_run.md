# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3556 frames over 39.8 s · **median 10.58 ms (94 fps)** · mean 11.18 · p95 15.64 · p99 19.50 (1% low 51 fps) · worst 34.71 ms · 166 spikes over 1.5× median · 46.8 stutters/min (work 54.3/min)
- warm-up: 466 frames over 5.6 s, worst 577.0 ms
- hitches (> 26.5 ms): **3** measured, 5 in warm-up
- timed GPU passes: 7.98 ms of a 11.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.58 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 0.66 min = 46.77/min, limit 1/min
- **FAIL** worst frame 34.71 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.117 | 2.385 | 0.183 |
| main_opaque_pass_3d | 1.798 | 2.415 | 0.049 |
| early prepass | 0.761 | 1.014 | 0.089 |
| lightprobe_irradiance_map | 0.436 | 0.668 | 0.001 |
| atmosphere_luts | 0.381 | 0.492 | 0.003 |
| bloom | 0.325 | 0.332 | 0.068 |
| lightprobe_radiance_map | 0.225 | 0.235 | 0.004 |
| ssao | 0.206 | 0.207 | 0.007 |
| uniform_allocation | 0.203 | 0.233 | 0.018 |
| clouds_trace | 0.156 | 0.201 | 0.001 |
| bin_unpacking | 0.143 | 0.170 | 0.013 |
| water_fft | 0.128 | 0.127 | 0.003 |
| upscaling | 0.125 | 0.160 | 0.004 |
| auxiliary_views | 0.106 | 0.784 | 0.096 |
| atmos_froxel_inject | 0.092 | 0.096 | 0.001 |
| tonemapping | 0.088 | 0.091 | 0.003 |
| water_velocity | 0.077 | 0.081 | 0.001 |
| surface_trail | 0.076 | 0.311 | 0.002 |
| clustering | 0.065 | 0.069 | 0.019 |
| clouds_composite | 0.064 | 0.064 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.094 | 0.101 |
| render_sky | 0.053 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.046 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.026 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| weather_map | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.007 | 0.010 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.652 | 13.839 | 32.44 | 34323.5 |
| render/busy/ms | 9.094 | 13.259 | 31.64 | 32338.2 |
| main/busy/ms | 7.487 | 9.329 | 16.29 | 26623.5 |
| render/graph/ms | 4.459 | 6.638 | 19.20 | 15855.7 |
| render/present/ms | 0.464 | 0.690 | 2.45 | 1649.5 |
| bigspace/total/ms | 0.359 | 0.609 | 2.41 | 1277.0 |
| terrain/update/ms | 0.271 | 0.341 | 1.99 | 964.9 |
| terrain/select/ms | 0.238 | 0.288 | 0.86 | 845.0 |
| stream/drive/ms | 0.227 | 0.392 | 2.10 | 806.7 |
| stream/tick/ms | 0.179 | 0.299 | 0.69 | 637.1 |
| controller/feed/ms | 0.176 | 0.250 | 0.86 | 627.3 |
| far/select/ms | 0.165 | 0.284 | 0.70 | 587.2 |
| anim/evaluate/ms | 0.127 | 0.319 | 1.49 | 452.4 |
| bigspace/low_precision/ms | 0.122 | 0.187 | 1.38 | 432.3 |
| anim/evaluate_cpu/ms | 0.116 | 0.164 | 0.50 | 411.4 |
| bigspace/high_precision/ms | 0.099 | 0.151 | 2.09 | 350.5 |
| render/acquire/ms | 0.094 | 0.126 | 0.31 | 335.7 |
| controller/move/ms | 0.078 | 0.133 | 0.43 | 276.1 |
| anim/apply/ms | 0.074 | 0.136 | 1.27 | 263.3 |
| physics/step/ms | 0.043 | 0.081 | 0.67 | 153.8 |
| physics/insert/ms | 0.029 | 0.305 | 1.03 | 101.8 |
| weather/map/ms | 0.025 | 0.033 | 0.18 | 89.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.04 | 38.0 |
| controller/avoid/ms | 0.009 | 0.015 | 0.09 | 33.5 |
| stream/spawn/ms | 0.007 | 0.000 | 1.17 | 24.4 |
| script/update/ms | 0.007 | 0.009 | 0.12 | 23.7 |
| script/input/ms | 0.005 | 0.007 | 0.13 | 18.5 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 14.6 |
| controller/write/ms | 0.003 | 0.006 | 0.02 | 11.6 |
| stream/despawn/ms | 0.003 | 0.000 | 1.26 | 9.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.36 | 4.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 3.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 1 |
| anim/a2/count | count | 1 | 4 | 1 |
| anim/a3/count | count | 438 | 490 | 489 |
| anim/characters/count | count | 438 | 492 | 491 |
| anim/evaluated/count | count | 14 | 25 | 14 |
| controller/movers/count | count | 16 | 18 | 15 |
| ecs/entities | count | 47.9 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.64 | 31.64 | 10.99 |
| gpu/timed/ms | ms | 7.96 | 11.24 | 7.64 |
| mesh_allocator_allocations |  meshes | 13.2 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 446.61 M | 446.61 M | 446.61 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2405 | 3288 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 214 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 167 | 179 | 179 |
| pipelines/waiting | count | 0 | 8 | 0 |
| process/cpu_usage | % | 23.75 | 37.50 | 18.92 |
| process/mem_usage | GiB | 2.22 | 2.67 | 2.67 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2920 | 3314 | 3314 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 1 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.12 | 0.04 |
| stream/model_queue | count | 0 | 72 | 0 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.28 | 0.09 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 2 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.39 | 62.19 | 31.32 |
| system/mem_usage | % | 44.87 | 46.10 | 46.10 |
| terrain/patches | count | 498 | 520 | 490 |
| world/meshes | count | 26.6 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 18.88 | 231 | 67.1 k |
| anim/events/count | count | 0.12 | 2 | 425 |
| assets/Image/added | count | 0.08 | 31 | 283 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.66 | 291 | 5898 |
| assets/Mesh/removed | count | 0.68 | 728 | 2434 |
| assets/StandardMaterial/added | count | 0.13 | 33 | 470 |
| assets/StandardMaterial/modified | count | 2.22 | 10 | 7882 |
| assets/StandardMaterial/removed | count | 0.04 | 26 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 5.00 | 1723 | 17.8 k |
| physics/inserts | count | 0.74 | 130 | 2624 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 162 | 1315 |
| physics/steps | count | 0.67 | 2 | 2386 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 7 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 18 | 214 |
| stream/deactivate/items | count | 0.04 | 16 | 147 |
| stream/despawn/items | count | 0.03 | 9 | 95 |
| stream/fetch/items | count | 0.02 | 2 | 77 |
| stream/shift_guard/frames | count | 0.00 | 1 | 4 |
| stream/spawn/items | count | 0.11 | 18 | 394 |
| stream/upload/bytes | bytes | 11.0 k | 2.07 M | 39.18 M |
| stream/upload/items | count | 0.01 | 2 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 63 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2569 | 1.02 M | 9.13 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 26.5 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.0 | main/busy/ms 555, surface/advance_cpu_ms 68.53, surface/cell_upload_ms 1.09, stream/drive/ms 0.32, weather/map/ms 0.11, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.93 | 348.8 | main/busy/ms 587, render/thread/ms 312, render/busy/ms 243, surface/advance_cpu_ms 68.53, render/acquire/ms 67.22, render/graph/ms 4.97, render/present/ms 2.40, surface/cell_upload_ms 1.09, stream/drive/ms 0.87, terrain/update/ms 0.47, stream/tick/ms 0.40, terrain/select/ms 0.34 |
| warm-up | 0.98 | 59.0 | render/thread/ms 332, render/busy/ms 261, render/acquire/ms 67.58, main/busy/ms 50.67, terrain/update/ms 17.34, render/graph/ms 10.51, render/present/ms 3.81, stream/drive/ms 0.92, terrain/select/ms 0.64, stream/tick/ms 0.61, bigspace/total/ms 0.29, bigspace/high_precision/ms 0.18 |
| warm-up | 1.22 | 90.4 | main/busy/ms 67.97, render/thread/ms 52.65, render/busy/ms 50.80, render/graph/ms 4.97, stream/drive/ms 1.56, render/present/ms 1.56, stream/tick/ms 1.30, terrain/update/ms 0.75, terrain/select/ms 0.62, bigspace/total/ms 0.55, bigspace/high_precision/ms 0.30, render/acquire/ms 0.29 |
| warm-up | 2.77 | 28.4 | main/busy/ms 35.63, render/thread/ms 20.66, render/busy/ms 19.11, render/graph/ms 5.85, render/present/ms 1.31, physics/insert/ms 1.08, stream/drive/ms 1.01, bigspace/total/ms 0.60, terrain/update/ms 0.59, terrain/select/ms 0.52, stream/tick/ms 0.42, stream/spawn/ms 0.30 |
| measure | 35.36 | 34.7 | render/thread/ms 54.71, render/busy/ms 53.14, render/graph/ms 30.84, main/busy/ms 20.72, anim/evaluate/ms 1.56, render/present/ms 1.42, stream/drive/ms 1.23, stream/spawn/ms 0.84, bigspace/total/ms 0.83, terrain/update/ms 0.50, terrain/select/ms 0.44, bigspace/low_precision/ms 0.40 |
| measure | 35.79 | 26.5 | render/thread/ms 42.03, render/busy/ms 40.87, render/graph/ms 22.59, main/busy/ms 17.47, render/present/ms 0.96, physics/insert/ms 0.78, bigspace/total/ms 0.74, terrain/update/ms 0.63, terrain/select/ms 0.46, stream/drive/ms 0.36, controller/feed/ms 0.34, stream/tick/ms 0.32 |
| measure | 37.39 | 29.1 | render/thread/ms 44.54, render/busy/ms 42.99, render/graph/ms 23.96, main/busy/ms 17.45, render/present/ms 1.35, terrain/update/ms 1.13, bigspace/total/ms 0.93, physics/insert/ms 0.55, terrain/select/ms 0.44, stream/drive/ms 0.41, stream/tick/ms 0.36, controller/feed/ms 0.35 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\run@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (153 samples): GPU load **83%** (min 57%) · 2580 MHz (min 2580) · memory 8001 MHz · 81.1 W · 74 °C (max 77) · VRAM 3389 MiB · throttled 100% of samples
