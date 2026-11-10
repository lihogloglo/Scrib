# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1520 frames over 15.0 s · **median 9.61 ms (104 fps)** · mean 9.87 · p95 11.83 · p99 13.47 (1% low 74 fps) · worst 33.77 ms · 8 spikes over 1.5× median · 20.0 stutters/min (work 20.0/min)
- warm-up: 316 frames over 4.7 s, worst 617.5 ms
- hitches (> 25.0 ms): **2** measured, 8 in warm-up
- timed GPU passes: 8.59 ms of a 9.87 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.61 ms, limit 11 ms
- **FAIL** worst frame 33.77 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.579 | 3.164 | 0.046 |
| dlss_super_resolution | 2.145 | 2.673 | 0.197 |
| early prepass | 0.609 | 0.939 | 0.085 |
| lightprobe_irradiance_map | 0.431 | 0.559 | 0.001 |
| atmosphere_luts | 0.411 | 0.684 | 0.004 |
| bloom | 0.333 | 0.346 | 0.071 |
| ssao | 0.248 | 0.549 | 0.008 |
| lightprobe_radiance_map | 0.236 | 0.239 | 0.004 |
| main_transparent_pass_3d | 0.196 | 0.196 | 0.072 |
| uniform_allocation | 0.154 | 0.153 | 0.017 |
| water_fft | 0.142 | 0.279 | 0.003 |
| clouds_trace | 0.136 | 0.162 | 0.001 |
| upscaling | 0.121 | 0.130 | 0.005 |
| bin_unpacking | 0.107 | 0.114 | 0.012 |
| atmos_froxel_inject | 0.093 | 0.093 | 0.002 |
| tonemapping | 0.090 | 0.091 | 0.004 |
| water_velocity | 0.078 | 0.076 | 0.002 |
| clustering | 0.069 | 0.081 | 0.024 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.006 |
| clouds_resolve | 0.036 | 0.038 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| per_view_shadow_pass | 0.028 | 0.030 | 0.007 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| lightprobe_copy | 0.017 | 0.019 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.013 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.646 | 10.492 | 31.95 | 13141.2 |
| render/busy/ms | 7.658 | 9.527 | 27.41 | 11640.3 |
| main/busy/ms | 6.974 | 8.337 | 41.48 | 10601.0 |
| render/graph/ms | 3.910 | 5.227 | 15.83 | 5943.3 |
| render/present/ms | 0.523 | 1.122 | 3.43 | 795.5 |
| render/acquire/ms | 0.464 | 1.485 | 2.56 | 705.4 |
| bigspace/total/ms | 0.333 | 0.599 | 1.54 | 505.5 |
| terrain/update/ms | 0.232 | 0.301 | 0.59 | 353.2 |
| stream/drive/ms | 0.231 | 0.413 | 0.81 | 351.1 |
| far/select/ms | 0.203 | 0.342 | 0.73 | 309.0 |
| terrain/select/ms | 0.201 | 0.260 | 0.52 | 305.3 |
| stream/tick/ms | 0.199 | 0.363 | 0.75 | 301.8 |
| controller/feed/ms | 0.149 | 0.233 | 0.73 | 227.1 |
| controller/move/ms | 0.136 | 0.262 | 0.70 | 207.1 |
| bigspace/high_precision/ms | 0.106 | 0.219 | 0.97 | 161.7 |
| anim/evaluate/ms | 0.098 | 0.201 | 0.95 | 148.8 |
| anim/apply/ms | 0.081 | 0.160 | 1.17 | 122.5 |
| bigspace/low_precision/ms | 0.076 | 0.156 | 0.78 | 115.9 |
| anim/evaluate_cpu/ms | 0.059 | 0.079 | 0.32 | 89.1 |
| physics/step/ms | 0.033 | 0.070 | 0.58 | 50.8 |
| weather/map/ms | 0.026 | 0.035 | 0.16 | 39.7 |
| weather/eval/ms | 0.012 | 0.015 | 0.20 | 17.7 |
| script/update/ms | 0.008 | 0.013 | 0.08 | 11.8 |
| controller/avoid/ms | 0.007 | 0.010 | 0.03 | 10.1 |
| physics/insert/ms | 0.006 | 0.031 | 0.59 | 8.7 |
| script/ui/ms | 0.006 | 0.008 | 0.05 | 8.4 |
| script/input/ms | 0.005 | 0.007 | 0.01 | 8.3 |
| controller/write/ms | 0.002 | 0.004 | 0.02 | 3.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 11 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 46.9 k | 46.9 k | 46.9 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.55 | 41.48 | 8.42 |
| gpu/timed/ms | ms | 8.51 | 12.04 | 8.42 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 113 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 18 | 0 |
| process/cpu_usage | % | 26.92 | 41.83 | 24.04 |
| process/mem_usage | GiB | 2.17 | 2.20 | 2.20 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2587 | 2589 | 2589 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 41.84 | 84.95 | 55.24 |
| system/mem_usage | % | 64.50 | 64.68 | 60.13 |
| terrain/patches | count | 370 | 370 | 370 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.13 | 198 | 198 |
| anim/events/count | count | 0.09 | 3 | 138 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.26 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.26 | 310 | 391 |
| physics/inserts | count | 0.08 | 24 | 129 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 900 |
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
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 10, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.62 | 617.5 | main/busy/ms 592, surface/advance_cpu_ms 68.32, surface/cell_upload_ms 0.84, stream/drive/ms 0.41, weather/map/ms 0.21, stream/tick/ms 0.10, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.96 | 341.4 | main/busy/ms 627, render/thread/ms 298, render/busy/ms 212, render/acquire/ms 84.09, surface/advance_cpu_ms 68.32, render/graph/ms 6.47, render/present/ms 2.24, stream/drive/ms 0.99, surface/cell_upload_ms 0.84, stream/tick/ms 0.48, terrain/update/ms 0.44, weather/map/ms 0.29 |
| warm-up | 1.33 | 368.2 | render/thread/ms 319, render/busy/ms 230, render/acquire/ms 84.39, main/busy/ms 53.64, terrain/update/ms 20.48, render/graph/ms 11.76, render/present/ms 3.95, stream/drive/ms 0.97, stream/tick/ms 0.69, terrain/select/ms 0.63, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.13 |
| warm-up | 1.57 | 93.1 | main/busy/ms 63.94, render/thread/ms 57.69, render/busy/ms 55.48, render/graph/ms 4.47, stream/drive/ms 2.07, stream/tick/ms 1.77, render/present/ms 1.73, terrain/update/ms 0.73, terrain/select/ms 0.60, render/acquire/ms 0.49, bigspace/total/ms 0.25, bigspace/high_precision/ms 0.13 |
| warm-up | 2.15 | 27.6 | render/thread/ms 36.25, render/busy/ms 35.10, main/busy/ms 16.94, render/graph/ms 3.92, stream/drive/ms 3.83, stream/spawn/ms 2, render/present/ms 0.93, stream/tick/ms 0.72, bigspace/total/ms 0.70, terrain/update/ms 0.59, terrain/select/ms 0.48, bigspace/high_precision/ms 0.39 |
| warm-up | 2.21 | 28.8 | render/thread/ms 49.07, render/busy/ms 47.78, main/busy/ms 17.64, stream/drive/ms 4.27, render/graph/ms 4.21, stream/spawn/ms 2.40, render/present/ms 0.98, stream/tick/ms 0.84, terrain/update/ms 0.58, bigspace/total/ms 0.57, terrain/select/ms 0.46, bigspace/high_precision/ms 0.35 |
| warm-up | 2.41 | 31.9 | main/busy/ms 36.09, render/thread/ms 19.86, render/busy/ms 18.62, render/graph/ms 4.84, stream/drive/ms 3.69, stream/spawn/ms 2.19, render/present/ms 1.05, bigspace/total/ms 0.83, stream/tick/ms 0.72, terrain/update/ms 0.54, bigspace/high_precision/ms 0.49, terrain/select/ms 0.44 |
| warm-up | 2.45 | 25.5 | main/busy/ms 33.12, render/thread/ms 19.45, render/busy/ms 18.32, render/graph/ms 5.05, stream/drive/ms 2.91, render/present/ms 0.96, stream/tick/ms 0.53, terrain/update/ms 0.53, bigspace/total/ms 0.53, terrain/select/ms 0.43, bigspace/high_precision/ms 0.25, stream/spawn/ms 0.18 |
| measure | 13.71 | 33.8 | main/busy/ms 49.82, render/thread/ms 42.81, render/busy/ms 37.22, render/graph/ms 20.49, render/present/ms 4.33, render/acquire/ms 1.26, bigspace/total/ms 1.18, controller/move/ms 0.84, stream/drive/ms 0.63, terrain/update/ms 0.57, stream/tick/ms 0.56, far/select/ms 0.51 |
| measure | 13.74 | 29.4 | main/busy/ms 56.52, render/thread/ms 48.78, render/busy/ms 43.25, render/graph/ms 21.66, render/present/ms 4.29, bigspace/total/ms 1.33, render/acquire/ms 1.23, stream/drive/ms 1.17, stream/tick/ms 1.03, controller/move/ms 0.85, far/select/ms 0.66, terrain/update/ms 0.65 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.2 W · 74 °C (max 76) · VRAM 4653 MiB · throttled 100% of samples
