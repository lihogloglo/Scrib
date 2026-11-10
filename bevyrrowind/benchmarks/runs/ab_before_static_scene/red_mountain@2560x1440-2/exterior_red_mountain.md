# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1597 frames over 15.0 s · **median 9.23 ms (108 fps)** · mean 9.40 · p95 11.19 · p99 12.42 (1% low 80 fps) · worst 14.40 ms · 4 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 446 frames over 5.2 s, worst 583.3 ms
- hitches (> 25.0 ms): **0** measured, 6 in warm-up
- timed GPU passes: 7.94 ms of a 9.40 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.23 ms, limit 11 ms
- pass worst frame 14.40 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.181 | 2.667 | 0.182 |
| main_opaque_pass_3d | 1.857 | 2.342 | 0.042 |
| early prepass | 0.721 | 1.047 | 0.078 |
| lightprobe_irradiance_map | 0.442 | 0.685 | 0.001 |
| atmosphere_luts | 0.379 | 0.647 | 0.003 |
| bloom | 0.328 | 0.508 | 0.067 |
| clouds_trace | 0.266 | 0.300 | 0.001 |
| lightprobe_radiance_map | 0.227 | 0.234 | 0.004 |
| ssao | 0.222 | 0.414 | 0.008 |
| uniform_allocation | 0.162 | 0.375 | 0.016 |
| water_fft | 0.132 | 0.130 | 0.003 |
| upscaling | 0.119 | 0.128 | 0.005 |
| bin_unpacking | 0.102 | 0.111 | 0.011 |
| tonemapping | 0.093 | 0.091 | 0.004 |
| atmos_froxel_inject | 0.088 | 0.089 | 0.001 |
| water_velocity | 0.080 | 0.073 | 0.001 |
| main_transparent_pass_3d | 0.078 | 0.084 | 0.071 |
| clustering | 0.068 | 0.066 | 0.023 |
| clouds_composite | 0.066 | 0.065 | 0.002 |
| render_sky | 0.052 | 0.052 | 0.002 |
| clouds_resolve | 0.035 | 0.040 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.035 | 0.005 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.023 | 0.026 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.007 |
| night_sky | 0.014 | 0.014 | 0.002 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.001 | 0.002 | 0.003 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.110 | 9.794 | 12.82 | 12952.4 |
| render/busy/ms | 7.441 | 8.918 | 11.86 | 11884.0 |
| main/busy/ms | 6.780 | 7.916 | 12.03 | 10827.6 |
| render/graph/ms | 3.593 | 4.604 | 6.24 | 5737.2 |
| render/present/ms | 0.543 | 1.295 | 2.26 | 867.0 |
| bigspace/total/ms | 0.346 | 0.527 | 1.32 | 552.5 |
| far/select/ms | 0.243 | 0.404 | 0.98 | 387.6 |
| terrain/update/ms | 0.234 | 0.297 | 0.56 | 373.0 |
| stream/drive/ms | 0.216 | 0.380 | 0.70 | 345.1 |
| terrain/select/ms | 0.203 | 0.256 | 0.53 | 323.9 |
| stream/tick/ms | 0.188 | 0.333 | 0.67 | 300.2 |
| controller/feed/ms | 0.151 | 0.226 | 0.53 | 241.1 |
| controller/move/ms | 0.133 | 0.246 | 0.42 | 213.1 |
| render/acquire/ms | 0.126 | 0.222 | 2.70 | 201.4 |
| bigspace/low_precision/ms | 0.101 | 0.179 | 1.01 | 162.1 |
| bigspace/high_precision/ms | 0.100 | 0.162 | 1.00 | 159.5 |
| anim/evaluate/ms | 0.100 | 0.204 | 1.16 | 159.0 |
| anim/evaluate_cpu/ms | 0.090 | 0.114 | 0.36 | 143.3 |
| anim/apply/ms | 0.079 | 0.111 | 0.83 | 126.2 |
| physics/step/ms | 0.041 | 0.086 | 0.76 | 66.0 |
| weather/map/ms | 0.025 | 0.034 | 0.34 | 40.2 |
| weather/eval/ms | 0.012 | 0.015 | 0.16 | 18.9 |
| controller/avoid/ms | 0.010 | 0.015 | 0.24 | 16.3 |
| script/update/ms | 0.007 | 0.011 | 0.14 | 11.4 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 8.3 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 8.2 |
| controller/write/ms | 0.003 | 0.004 | 0.10 | 4.4 |
| shapes/activate/ms | 0.001 | 0.003 | 0.24 | 2.1 |
| physics/insert/ms | 0.001 | 0.000 | 0.50 | 1.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| weather/apply/ms | 0.001 | 0.001 | 0.03 | 1.3 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 1 | 0 |
| anim/a2/count | count | 0 | 1 | 0 |
| anim/a3/count | count | 503 | 503 | 503 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 15 | 15 |
| controller/movers/count | count | 15 | 15 | 15 |
| ecs/entities | count | 51.4 k | 51.4 k | 51.4 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.97 | 12.03 | 7.55 |
| gpu/timed/ms | ms | 7.89 | 10.00 | 7.55 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.19 M | 433.19 M | 433.19 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 165 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 157 | 157 | 157 |
| pipelines/waiting | count | 0 | 14 | 0 |
| process/cpu_usage | % | 26.92 | 37.02 | 29.81 |
| process/mem_usage | GiB | 2.03 | 2.06 | 2.06 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2653 | 2654 | 2654 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 214 | 214 | 214 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 23.03 M | 23.03 M | 23.03 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 1124 | 1124 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.02 | 52.97 | 36.43 |
| system/mem_usage | % | 46.44 | 46.58 | 46.54 |
| terrain/patches | count | 391 | 391 | 391 |
| world/meshes | count | 23.9 k | 23.9 k | 23.9 k |
| world/point_lights | count | 200 | 200 | 200 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 8.46 | 34 | 13.5 k |
| anim/events/count | count | 0.16 | 4 | 256 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.11 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.25 | 398 | 397 |
| physics/inserts | count | 0.10 | 61 | 166 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 583.3 | main/busy/ms 558, surface/advance_cpu_ms 70.75, surface/cell_upload_ms 0.87, stream/drive/ms 0.25, weather/map/ms 0.12, bigspace/total/ms 0.07, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.91 | 327.8 | main/busy/ms 595, render/thread/ms 288, render/busy/ms 241, surface/advance_cpu_ms 70.75, render/acquire/ms 45.51, render/graph/ms 6.42, render/present/ms 1.94, stream/drive/ms 0.96, surface/cell_upload_ms 0.87, stream/tick/ms 0.58, terrain/update/ms 0.52, terrain/select/ms 0.36 |
| warm-up | 0.96 | 46.1 | render/thread/ms 309, render/busy/ms 259, main/busy/ms 56.56, render/acquire/ms 45.90, terrain/update/ms 23.10, render/graph/ms 11.70, render/present/ms 3.39, terrain/select/ms 1.18, stream/drive/ms 1.08, stream/tick/ms 0.82, bigspace/total/ms 0.23, bigspace/high_precision/ms 0.16 |
| warm-up | 1.15 | 87.8 | main/busy/ms 67.86, render/thread/ms 50.86, render/busy/ms 48.62, render/graph/ms 5.13, stream/drive/ms 2.45, render/present/ms 1.85, stream/tick/ms 1.48, terrain/update/ms 0.79, terrain/select/ms 0.66, stream/spawn/ms 0.42, render/acquire/ms 0.39, bigspace/total/ms 0.29 |
| warm-up | 2.60 | 25.7 | main/busy/ms 36.58, render/thread/ms 19.19, render/busy/ms 17.99, render/graph/ms 4.93, stream/drive/ms 1.10, render/present/ms 0.91, stream/tick/ms 0.59, physics/insert/ms 0.56, bigspace/total/ms 0.51, terrain/update/ms 0.50, terrain/select/ms 0.40, render/acquire/ms 0.28 |
| warm-up | 2.78 | 26.2 | main/busy/ms 30.79, render/thread/ms 19.19, render/busy/ms 17.98, render/graph/ms 5.40, stream/drive/ms 1.36, render/present/ms 1.04, bigspace/total/ms 0.85, stream/tick/ms 0.73, terrain/update/ms 0.60, terrain/select/ms 0.49, far/select/ms 0.48, stream/spawn/ms 0.31 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_before\red_mountain@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **92%** (min 71%) · 2581 MHz (min 2580) · memory 8001 MHz · 87.6 W · 70 °C (max 73) · VRAM 3804 MiB · throttled 100% of samples
