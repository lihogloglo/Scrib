# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3604 frames over 39.8 s · **median 10.54 ms (95 fps)** · mean 11.03 · p95 14.68 · p99 17.09 (1% low 59 fps) · worst 34.29 ms · 86 spikes over 1.5× median · 33.2 stutters/min (work 42.2/min)
- warm-up: 262 frames over 3.7 s, worst 586.6 ms
- hitches (> 26.4 ms): **2** measured, 6 in warm-up
- timed GPU passes: 7.95 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.54 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 0.66 min = 33.19/min, limit 1/min
- **FAIL** worst frame 34.29 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.116 | 2.439 | 0.185 |
| main_opaque_pass_3d | 1.800 | 2.463 | 0.052 |
| early prepass | 0.764 | 1.019 | 0.092 |
| lightprobe_irradiance_map | 0.436 | 0.662 | 0.001 |
| atmosphere_luts | 0.377 | 0.492 | 0.003 |
| bloom | 0.325 | 0.332 | 0.068 |
| lightprobe_radiance_map | 0.225 | 0.234 | 0.004 |
| uniform_allocation | 0.208 | 0.236 | 0.019 |
| ssao | 0.207 | 0.207 | 0.007 |
| clouds_trace | 0.156 | 0.200 | 0.001 |
| bin_unpacking | 0.155 | 0.177 | 0.014 |
| water_fft | 0.130 | 0.128 | 0.002 |
| upscaling | 0.125 | 0.157 | 0.004 |
| atmos_froxel_inject | 0.092 | 0.096 | 0.001 |
| tonemapping | 0.087 | 0.090 | 0.004 |
| water_velocity | 0.078 | 0.081 | 0.002 |
| surface_trail | 0.069 | 0.061 | 0.002 |
| clustering | 0.066 | 0.070 | 0.019 |
| clouds_composite | 0.064 | 0.064 | 0.002 |
| auxiliary_views | 0.063 | 0.417 | 0.065 |
| main_transparent_pass_3d | 0.062 | 0.094 | 0.100 |
| render_sky | 0.053 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.045 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| weather_map | 0.012 | 0.012 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.010 | 0.011 | 0.001 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.488 | 12.933 | 32.70 | 34194.6 |
| render/busy/ms | 8.916 | 12.371 | 31.80 | 32133.3 |
| main/busy/ms | 7.562 | 9.819 | 17.07 | 27253.7 |
| render/graph/ms | 4.391 | 6.212 | 21.63 | 15823.5 |
| render/present/ms | 0.476 | 1.175 | 2.33 | 1715.5 |
| bigspace/total/ms | 0.356 | 0.555 | 2.65 | 1283.9 |
| terrain/update/ms | 0.278 | 0.349 | 2.11 | 1000.9 |
| terrain/select/ms | 0.244 | 0.301 | 0.68 | 878.5 |
| stream/drive/ms | 0.235 | 0.429 | 1.84 | 848.3 |
| stream/tick/ms | 0.186 | 0.327 | 1.01 | 670.6 |
| controller/feed/ms | 0.178 | 0.266 | 0.55 | 642.4 |
| far/select/ms | 0.168 | 0.292 | 0.74 | 605.3 |
| anim/evaluate_cpu/ms | 0.124 | 0.172 | 0.45 | 445.1 |
| anim/evaluate/ms | 0.121 | 0.272 | 1.61 | 436.0 |
| bigspace/low_precision/ms | 0.121 | 0.178 | 1.54 | 435.8 |
| render/acquire/ms | 0.096 | 0.127 | 1.17 | 345.9 |
| bigspace/high_precision/ms | 0.093 | 0.125 | 2.02 | 336.3 |
| controller/move/ms | 0.088 | 0.168 | 0.60 | 318.1 |
| anim/apply/ms | 0.074 | 0.130 | 2.23 | 265.4 |
| physics/insert/ms | 0.047 | 0.414 | 1.34 | 169.4 |
| physics/step/ms | 0.045 | 0.084 | 0.88 | 162.0 |
| weather/map/ms | 0.025 | 0.032 | 0.16 | 90.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 39.5 |
| controller/avoid/ms | 0.009 | 0.015 | 0.20 | 33.9 |
| script/update/ms | 0.007 | 0.011 | 0.14 | 26.0 |
| stream/spawn/ms | 0.007 | 0.000 | 1.40 | 25.5 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 19.0 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 15.3 |
| controller/write/ms | 0.003 | 0.005 | 0.03 | 10.7 |
| stream/despawn/ms | 0.003 | 0.000 | 1.08 | 9.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 1.18 | 7.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.35 | 4.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.28 | 4.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 2 |
| anim/a2/count | count | 1 | 5 | 0 |
| anim/a3/count | count | 441 | 490 | 489 |
| anim/characters/count | count | 441 | 492 | 491 |
| anim/evaluated/count | count | 14 | 68 | 15 |
| controller/movers/count | count | 16 | 18 | 15 |
| ecs/entities | count | 48.3 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.63 | 31.80 | 11.95 |
| gpu/timed/ms | ms | 7.95 | 11.32 | 8.11 |
| mesh_allocator_allocations |  meshes | 13.3 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 497.62 M | 497.62 M | 497.62 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2405 | 3288 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 1679 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 167 | 178 | 178 |
| pipelines/waiting | count | 0 | 42 | 0 |
| process/cpu_usage | % | 24.52 | 37.50 | 20.98 |
| process/mem_usage | GiB | 2.23 | 2.69 | 2.69 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 72 | 1 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.25 | 0.07 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 3 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.95 | 53.82 | 45.69 |
| system/mem_usage | % | 44.76 | 46.22 | 46.22 |
| terrain/patches | count | 498 | 520 | 490 |
| world/meshes | count | 26.9 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 19.88 | 263 | 71.7 k |
| anim/events/count | count | 0.13 | 3 | 469 |
| assets/Image/added | count | 0.09 | 31 | 311 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.67 | 300 | 6008 |
| assets/Mesh/removed | count | 0.68 | 718 | 2434 |
| assets/StandardMaterial/added | count | 0.14 | 33 | 500 |
| assets/StandardMaterial/modified | count | 2.19 | 10 | 7884 |
| assets/StandardMaterial/removed | count | 0.04 | 32 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 5.76 | 1720 | 20.8 k |
| physics/inserts | count | 0.98 | 127 | 3530 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 162 | 1313 |
| physics/steps | count | 0.66 | 2 | 2385 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 14 |
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
| stream/upload/items | count | 0.01 | 1 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 63 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2535 | 1.02 M | 9.13 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 26.4 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 586.6 | main/busy/ms 565, surface/advance_cpu_ms 77.60, surface/cell_upload_ms 0.79, stream/drive/ms 0.32, weather/map/ms 0.12, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.98 | 389.0 | main/busy/ms 597, render/thread/ms 354, render/busy/ms 305, surface/advance_cpu_ms 77.60, render/acquire/ms 46.29, render/graph/ms 8.08, render/present/ms 2.04, stream/drive/ms 0.95, surface/cell_upload_ms 0.79, stream/tick/ms 0.48, terrain/update/ms 0.46, terrain/select/ms 0.32 |
| warm-up | 1.02 | 45.0 | render/thread/ms 373, render/busy/ms 323, main/busy/ms 49.41, render/acquire/ms 46.62, terrain/update/ms 17.53, render/graph/ms 13.42, render/present/ms 3.37, stream/drive/ms 1.05, stream/tick/ms 0.71, terrain/select/ms 0.67, bigspace/total/ms 0.23, bigspace/high_precision/ms 0.17 |
| warm-up | 1.26 | 99.7 | main/busy/ms 63.17, render/thread/ms 54.57, render/busy/ms 52.82, render/graph/ms 4.01, render/present/ms 1.48, stream/drive/ms 1.23, stream/tick/ms 1.04, terrain/update/ms 0.79, terrain/select/ms 0.66, bigspace/total/ms 0.31, render/acquire/ms 0.27, bigspace/high_precision/ms 0.13 |
| warm-up | 3.49 | 101.6 | render/thread/ms 96.04, render/busy/ms 94.87, render/graph/ms 84.64, main/busy/ms 32.80, render/present/ms 1.00, physics/insert/ms 0.95, terrain/update/ms 0.81, stream/drive/ms 0.80, bigspace/total/ms 0.75, terrain/select/ms 0.71, stream/tick/ms 0.35, far/select/ms 0.33 |
| warm-up | 3.58 | 68.6 | main/busy/ms 71.26, render/thread/ms 44.20, render/busy/ms 42.83, render/graph/ms 9.13, physics/insert/ms 1.18, render/present/ms 1.13, anim/evaluate_cpu/ms 0.96, bigspace/total/ms 0.91, terrain/update/ms 0.51, stream/drive/ms 0.51, bigspace/low_precision/ms 0.48, terrain/select/ms 0.45 |
| measure | 35.36 | 34.3 | render/thread/ms 41.64, render/busy/ms 40.11, render/graph/ms 25.07, main/busy/ms 19.91, render/present/ms 1.36, bigspace/total/ms 0.94, physics/insert/ms 0.78, anim/evaluate/ms 0.58, anim/evaluate_cpu/ms 0.53, terrain/update/ms 0.52, terrain/select/ms 0.46, stream/drive/ms 0.37 |
| measure | 37.38 | 27.6 | render/thread/ms 38.15, render/busy/ms 37.12, render/graph/ms 20.16, main/busy/ms 19.16, bigspace/total/ms 1.85, bigspace/low_precision/ms 1.20, terrain/update/ms 1.12, render/present/ms 0.84, physics/insert/ms 0.71, anim/evaluate/ms 0.62, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.46 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\run@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (154 samples): GPU load **84%** (min 61%) · 2580 MHz (min 2580) · memory 8001 MHz · 81.7 W · 73 °C (max 76) · VRAM 3645 MiB · throttled 100% of samples
