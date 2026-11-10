# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3600 frames over 39.8 s · **median 10.49 ms (95 fps)** · mean 11.05 · p95 15.34 · p99 18.85 (1% low 53 fps) · worst 37.27 ms · 160 spikes over 1.5× median · 46.8 stutters/min (work 63.4/min)
- warm-up: 253 frames over 3.9 s, worst 580.4 ms
- hitches (> 26.2 ms): **6** measured, 6 in warm-up
- timed GPU passes: 7.93 ms of a 11.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.49 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 0.66 min = 46.77/min, limit 1/min
- **FAIL** worst frame 37.27 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.127 | 2.476 | 0.188 |
| main_opaque_pass_3d | 1.768 | 2.451 | 0.048 |
| early prepass | 0.749 | 1.021 | 0.088 |
| lightprobe_irradiance_map | 0.435 | 0.668 | 0.001 |
| atmosphere_luts | 0.379 | 0.493 | 0.003 |
| bloom | 0.324 | 0.331 | 0.069 |
| lightprobe_radiance_map | 0.224 | 0.234 | 0.004 |
| ssao | 0.208 | 0.208 | 0.007 |
| uniform_allocation | 0.198 | 0.225 | 0.018 |
| clouds_trace | 0.158 | 0.201 | 0.001 |
| bin_unpacking | 0.132 | 0.161 | 0.013 |
| water_fft | 0.127 | 0.127 | 0.003 |
| upscaling | 0.125 | 0.158 | 0.004 |
| auxiliary_views | 0.107 | 0.863 | 0.096 |
| atmos_froxel_inject | 0.093 | 0.096 | 0.002 |
| tonemapping | 0.088 | 0.090 | 0.004 |
| water_velocity | 0.078 | 0.081 | 0.001 |
| clustering | 0.067 | 0.070 | 0.020 |
| clouds_composite | 0.064 | 0.064 | 0.002 |
| surface_trail | 0.061 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.061 | 0.095 | 0.102 |
| early_mesh_preprocessing | 0.055 | 0.051 | 0.005 |
| render_sky | 0.053 | 0.055 | 0.002 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| weather_map | 0.014 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.008 | 0.007 | 0.003 |
| night_sky | 0.007 | 0.011 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.501 | 13.580 | 28.37 | 34204.5 |
| render/busy/ms | 8.930 | 13.042 | 27.54 | 32146.8 |
| main/busy/ms | 7.573 | 9.835 | 22.79 | 27263.5 |
| render/graph/ms | 4.361 | 6.598 | 16.97 | 15698.5 |
| render/present/ms | 0.474 | 0.842 | 2.84 | 1705.0 |
| bigspace/total/ms | 0.367 | 0.581 | 2.39 | 1320.5 |
| terrain/update/ms | 0.278 | 0.351 | 2.61 | 1001.3 |
| terrain/select/ms | 0.243 | 0.306 | 0.63 | 876.5 |
| stream/drive/ms | 0.230 | 0.412 | 2.02 | 828.5 |
| stream/tick/ms | 0.182 | 0.319 | 0.82 | 655.5 |
| controller/feed/ms | 0.181 | 0.263 | 0.54 | 652.2 |
| far/select/ms | 0.167 | 0.284 | 0.63 | 603.0 |
| bigspace/low_precision/ms | 0.125 | 0.196 | 1.53 | 450.3 |
| anim/evaluate_cpu/ms | 0.121 | 0.167 | 0.46 | 436.7 |
| anim/evaluate/ms | 0.119 | 0.312 | 2.38 | 428.8 |
| bigspace/high_precision/ms | 0.098 | 0.143 | 2.05 | 353.7 |
| render/acquire/ms | 0.098 | 0.132 | 1.37 | 352.8 |
| controller/move/ms | 0.080 | 0.154 | 0.46 | 288.3 |
| anim/apply/ms | 0.070 | 0.118 | 1.27 | 252.9 |
| physics/insert/ms | 0.044 | 0.396 | 0.96 | 159.5 |
| physics/step/ms | 0.044 | 0.084 | 1.06 | 159.0 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 89.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 40.0 |
| controller/avoid/ms | 0.009 | 0.015 | 0.08 | 33.9 |
| stream/spawn/ms | 0.007 | 0.000 | 1.21 | 25.5 |
| script/update/ms | 0.007 | 0.010 | 0.08 | 25.2 |
| script/input/ms | 0.005 | 0.007 | 0.21 | 18.9 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 16.5 |
| controller/write/ms | 0.003 | 0.006 | 0.05 | 11.8 |
| stream/despawn/ms | 0.003 | 0.000 | 1.22 | 9.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.69 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.38 | 4.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 3.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 1 |
| anim/a2/count | count | 1 | 4 | 1 |
| anim/a3/count | count | 434 | 490 | 489 |
| anim/characters/count | count | 434 | 492 | 491 |
| anim/evaluated/count | count | 14 | 34 | 14 |
| controller/movers/count | count | 16 | 18 | 15 |
| ecs/entities | count | 47.6 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.57 | 27.54 | 11.67 |
| gpu/timed/ms | ms | 7.95 | 11.67 | 7.95 |
| mesh_allocator_allocations |  meshes | 13.0 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 446.61 M | 446.61 M | 446.61 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2405 | 3288 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 1264 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 168 | 180 | 180 |
| pipelines/waiting | count | 0 | 41 | 0 |
| process/cpu_usage | % | 25.23 | 44.20 | 21.88 |
| process/mem_usage | GiB | 2.26 | 2.78 | 2.78 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2920 | 3312 | 3312 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 1 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.12 | 0.05 |
| stream/model_queue | count | 0 | 72 | 0 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.28 | 0.08 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 3 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.56 | 79.15 | 42.51 |
| system/mem_usage | % | 44.98 | 46.74 | 46.74 |
| terrain/patches | count | 498 | 520 | 490 |
| world/meshes | count | 26.6 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 19.03 | 251 | 68.5 k |
| anim/events/count | count | 0.12 | 3 | 437 |
| assets/Image/added | count | 0.18 | 61 | 654 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.88 | 300 | 6784 |
| assets/Mesh/removed | count | 0.68 | 728 | 2434 |
| assets/StandardMaterial/added | count | 0.24 | 72 | 868 |
| assets/StandardMaterial/modified | count | 2.19 | 10 | 7879 |
| assets/StandardMaterial/removed | count | 0.04 | 32 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 7.65 | 1536 | 27.5 k |
| physics/inserts | count | 0.95 | 127 | 3424 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 146 | 1247 |
| physics/steps | count | 0.66 | 2 | 2386 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 7 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 18 | 214 |
| stream/deactivate/items | count | 0.04 | 16 | 147 |
| stream/despawn/items | count | 0.03 | 11 | 95 |
| stream/fetch/items | count | 0.02 | 2 | 77 |
| stream/shift_guard/frames | count | 0.00 | 1 | 4 |
| stream/spawn/items | count | 0.11 | 18 | 394 |
| stream/upload/bytes | bytes | 10.9 k | 2.07 M | 39.18 M |
| stream/upload/items | count | 0.01 | 2 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 63 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2537 | 1.02 M | 9.13 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 26.2 ms; slowest 12, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.4 | main/busy/ms 558, surface/advance_cpu_ms 67.71, surface/cell_upload_ms 1.09, stream/drive/ms 0.35, weather/map/ms 0.16, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.80 | 222.3 | main/busy/ms 592, render/thread/ms 191, render/busy/ms 121, surface/advance_cpu_ms 67.71, render/acquire/ms 66.84, render/graph/ms 5.39, render/present/ms 2.49, stream/drive/ms 1.31, surface/cell_upload_ms 1.09, stream/tick/ms 0.70, terrain/update/ms 0.56, terrain/select/ms 0.39 |
| warm-up | 1.18 | 378.1 | render/thread/ms 210, render/busy/ms 138, render/acquire/ms 67.22, main/busy/ms 50.08, terrain/update/ms 19.43, render/graph/ms 10.89, render/present/ms 4.11, stream/drive/ms 1.32, stream/tick/ms 0.88, terrain/select/ms 0.69, bigspace/total/ms 0.27, weather/map/ms 0.15 |
| warm-up | 1.41 | 87.8 | main/busy/ms 66.58, render/thread/ms 47.72, render/busy/ms 46.38, render/graph/ms 3.93, stream/drive/ms 2.02, stream/tick/ms 1.70, render/present/ms 1.10, terrain/update/ms 0.74, terrain/select/ms 0.63, render/acquire/ms 0.24, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.10 |
| warm-up | 1.80 | 27.2 | render/thread/ms 28.69, render/busy/ms 27.41, main/busy/ms 22.85, render/graph/ms 5.83, stream/drive/ms 3.60, stream/spawn/ms 2.03, render/present/ms 1.08, terrain/update/ms 0.84, stream/tick/ms 0.71, terrain/select/ms 0.70, bigspace/total/ms 0.61, bigspace/high_precision/ms 0.26 |
| warm-up | 2.04 | 26.5 | main/busy/ms 37.58, render/thread/ms 26.01, render/busy/ms 24.68, render/graph/ms 7.39, terrain/update/ms 2.15, stream/drive/ms 1.49, render/present/ms 1.10, physics/insert/ms 1.10, stream/tick/ms 0.72, bigspace/total/ms 0.55, terrain/select/ms 0.48, physics/step/ms 0.40 |
| measure | 1.19 | 29.8 | render/thread/ms 37.95, render/busy/ms 36.49, render/graph/ms 21.99, main/busy/ms 13.88, render/present/ms 1.26, physics/insert/ms 0.96, bigspace/total/ms 0.56, stream/drive/ms 0.55, terrain/update/ms 0.53, stream/tick/ms 0.48, terrain/select/ms 0.47, far/select/ms 0.26 |
| measure | 1.64 | 37.3 | main/busy/ms 38.56, render/thread/ms 38.13, render/busy/ms 36.68, render/graph/ms 11.07, physics/insert/ms 1.28, render/present/ms 1.20, bigspace/total/ms 0.96, terrain/update/ms 0.95, terrain/select/ms 0.68, stream/drive/ms 0.64, stream/tick/ms 0.48, far/select/ms 0.42 |
| measure | 35.32 | 26.7 | render/thread/ms 38.94, render/busy/ms 37.65, render/graph/ms 20.62, main/busy/ms 19.78, stream/drive/ms 1.25, render/present/ms 1.13, stream/spawn/ms 0.85, bigspace/total/ms 0.65, terrain/update/ms 0.51, terrain/select/ms 0.45, controller/feed/ms 0.36, stream/tick/ms 0.31 |
| measure | 35.78 | 28.2 | render/thread/ms 39.25, render/busy/ms 37.95, render/graph/ms 20.96, main/busy/ms 20.82, render/present/ms 1.11, bigspace/total/ms 0.87, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.36, controller/feed/ms 0.36, bigspace/low_precision/ms 0.33, stream/tick/ms 0.32 |
| measure | 35.81 | 29.6 | render/thread/ms 53.34, render/busy/ms 51.61, render/graph/ms 31.26, main/busy/ms 20.45, render/present/ms 1.56, terrain/update/ms 1.05, terrain/select/ms 0.88, bigspace/total/ms 0.79, anim/evaluate_cpu/ms 0.40, controller/feed/ms 0.38, stream/drive/ms 0.38, stream/tick/ms 0.31 |
| measure | 37.39 | 27.3 | render/thread/ms 40.53, render/busy/ms 39.41, main/busy/ms 21.92, render/graph/ms 21.46, terrain/update/ms 1.23, render/present/ms 0.96, bigspace/total/ms 0.74, physics/insert/ms 0.69, anim/evaluate/ms 0.61, terrain/select/ms 0.53, stream/drive/ms 0.44, anim/evaluate_cpu/ms 0.42 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\run@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (154 samples): GPU load **83%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 80.4 W · 73 °C (max 76) · VRAM 3393 MiB · throttled 100% of samples
