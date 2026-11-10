# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5829 frames over 71.5 s · **median 12.17 ms (82 fps)** · mean 12.27 · p95 15.58 · p99 18.33 (1% low 55 fps) · worst 31.95 ms · 65 spikes over 1.5× median · 31.9 stutters/min (work 52.8/min)
- warm-up: 484 frames over 5.6 s, worst 592.8 ms
- hitches (> 30.4 ms): **1** measured, 3 in warm-up
- timed GPU passes: 8.29 ms of a 12.27 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.17 ms, limit 11 ms
- **FAIL** frame_ms: 38 stutters (> 1.5x moving median of 60) in 1.19 min = 31.88/min, limit 1/min
- pass worst frame 31.95 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.233 | 2.927 | 0.051 |
| dlss_super_resolution | 2.094 | 2.426 | 0.189 |
| early prepass | 0.599 | 1.049 | 0.098 |
| lightprobe_irradiance_map | 0.437 | 0.640 | 0.001 |
| atmosphere_luts | 0.384 | 0.593 | 0.004 |
| bloom | 0.318 | 0.319 | 0.072 |
| clouds_trace | 0.226 | 0.265 | 0.001 |
| lightprobe_radiance_map | 0.225 | 0.234 | 0.004 |
| ssao | 0.214 | 0.210 | 0.008 |
| main_transparent_pass_3d | 0.207 | 0.473 | 0.089 |
| uniform_allocation | 0.174 | 0.207 | 0.017 |
| water_fft | 0.132 | 0.130 | 0.003 |
| bin_unpacking | 0.130 | 0.155 | 0.014 |
| upscaling | 0.113 | 0.120 | 0.005 |
| atmos_froxel_inject | 0.091 | 0.092 | 0.002 |
| tonemapping | 0.088 | 0.090 | 0.004 |
| water_velocity | 0.076 | 0.095 | 0.002 |
| clouds_composite | 0.066 | 0.068 | 0.002 |
| surface_trail | 0.064 | 0.059 | 0.003 |
| clustering | 0.064 | 0.068 | 0.022 |
| render_sky | 0.052 | 0.052 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.006 |
| clouds_resolve | 0.036 | 0.044 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.029 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.031 | 0.007 |
| water_mips | 0.023 | 0.024 | 0.002 |
| auxiliary_views | 0.014 | 0.002 | 0.015 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| weather_map | 0.013 | 0.013 | 0.001 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.459 | 13.583 | 29.73 | 60966.6 |
| render/busy/ms | 9.870 | 13.006 | 28.74 | 57529.6 |
| main/busy/ms | 8.163 | 10.231 | 15.76 | 47580.1 |
| render/graph/ms | 4.668 | 5.845 | 13.06 | 27212.3 |
| render/present/ms | 0.461 | 0.597 | 2.35 | 2685.7 |
| bigspace/total/ms | 0.435 | 0.789 | 3.55 | 2536.0 |
| stream/drive/ms | 0.320 | 0.867 | 9.67 | 1866.6 |
| terrain/update/ms | 0.258 | 0.486 | 2.58 | 1505.1 |
| controller/feed/ms | 0.239 | 0.376 | 1.14 | 1390.9 |
| far/select/ms | 0.227 | 0.420 | 1.29 | 1325.9 |
| terrain/select/ms | 0.192 | 0.252 | 0.61 | 1121.3 |
| stream/tick/ms | 0.179 | 0.298 | 0.82 | 1040.7 |
| controller/move/ms | 0.151 | 0.354 | 1.14 | 879.9 |
| bigspace/low_precision/ms | 0.145 | 0.261 | 3.20 | 844.6 |
| anim/evaluate_cpu/ms | 0.135 | 0.223 | 0.64 | 784.1 |
| render/acquire/ms | 0.129 | 0.157 | 13.70 | 751.2 |
| bigspace/high_precision/ms | 0.111 | 0.222 | 2.04 | 647.7 |
| anim/apply/ms | 0.110 | 0.207 | 1.85 | 642.6 |
| anim/evaluate/ms | 0.105 | 0.239 | 2.48 | 614.2 |
| physics/step/ms | 0.083 | 0.227 | 1.58 | 484.8 |
| stream/despawn/ms | 0.048 | 0.461 | 1.97 | 281.9 |
| stream/spawn/ms | 0.037 | 0.290 | 1.51 | 216.3 |
| weather/map/ms | 0.027 | 0.037 | 0.23 | 159.2 |
| physics/insert/ms | 0.021 | 0.141 | 0.81 | 121.7 |
| controller/avoid/ms | 0.011 | 0.018 | 0.51 | 65.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.20 | 61.7 |
| script/update/ms | 0.007 | 0.010 | 0.05 | 43.3 |
| stream/deactivate/ms | 0.006 | 0.000 | 0.55 | 36.6 |
| script/input/ms | 0.006 | 0.008 | 0.22 | 33.6 |
| script/ui/ms | 0.006 | 0.008 | 0.18 | 33.3 |
| stream/activate/ms | 0.005 | 0.000 | 0.63 | 31.3 |
| controller/write/ms | 0.003 | 0.006 | 0.11 | 19.8 |
| stream/upload/ms | 0.002 | 0.014 | 0.10 | 9.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.27 | 6.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.7 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.08 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.9 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 604 | 777 | 471 |
| anim/characters/count | count | 604 | 782 | 471 |
| anim/evaluated/count | count | 19 | 46 | 14 |
| controller/movers/count | count | 19 | 32 | 14 |
| ecs/entities | count | 59.9 k | 72.5 k | 52.9 k |
| far/entities | count | 2681 | 3461 | 2126 |
| far/forced | count | 0 | 8 | 0 |
| far/impostors | count | 61.0 k | 65.7 k | 55.8 k |
| far/lights | count | 7115 | 7757 | 6575 |
| far/nodes/l1 | count | 1100 | 1439 | 857 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 227.79 M | 173.66 M |
| far/triangles | count | 5.04 M | 5.72 M | 4.26 M |
| frame/work/ms | ms | 9.76 | 30.48 | 8.67 |
| gpu/timed/ms | ms | 8.27 | 30.48 | 7.41 |
| mesh_allocator_allocations |  meshes | 15.7 k | 19.3 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 588.08 M | 594.70 M | 594.70 M |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 1898 | 2692 | 1957 |
| physics/items | count | 13 | 108 | 108 |
| physics/pending | count | 0 | 358 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 5 | 0 |
| process/cpu_usage | % | 22.69 | 48.08 | 24.04 |
| process/mem_usage | GiB | 2.47 | 2.61 | 2.61 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3859 | 4858 | 4858 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 55 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 205 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.56 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 171 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 7 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.22 | 69.39 | 42.47 |
| system/mem_usage | % | 48.13 | 48.72 | 48.63 |
| terrain/patches | count | 349 | 493 | 319 |
| world/meshes | count | 26.6 k | 33.5 k | 28.1 k |
| world/point_lights | count | 271 | 550 | 524 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 8.97 | 201 | 52.3 k |
| anim/events/count | count | 0.21 | 4 | 1239 |
| assets/Image/added | count | 0.13 | 38 | 732 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 6.85 | 461 | 39.9 k |
| assets/Mesh/removed | count | 11.35 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.17 | 39 | 985 |
| assets/StandardMaterial/modified | count | 5.85 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.08 | 36 | 470 |
| assets/WorldAsset/added | count | 0.01 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 10 | 124 |
| ecs/entities_delta | count | 2.76 | 2005 | 16.1 k |
| physics/inserts | count | 2.50 | 143 | 14.6 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.43 | 287 | 14.2 k |
| physics/steps | count | 0.74 | 2 | 4291 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.23 | 27 | 1345 |
| stream/deactivate/items | count | 0.23 | 27 | 1345 |
| stream/despawn/items | count | 1.20 | 298 | 6982 |
| stream/fetch/items | count | 0.25 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.79 | 29 | 4586 |
| stream/upload/bytes | bytes | 84.9 k | 3.51 M | 495.17 M |
| stream/upload/items | count | 0.22 | 9 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.05 | 1 | 269 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 19.7 k | 1.21 M | 114.84 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 30.4 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 592.8 | main/busy/ms 569, surface/advance_cpu_ms 69.35, surface/cell_upload_ms 1.00, weather/map/ms 0.41, stream/drive/ms 0.30, stream/tick/ms 0.07, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.95 | 356.3 | main/busy/ms 604, render/thread/ms 315, render/busy/ms 267, surface/advance_cpu_ms 69.35, render/acquire/ms 46.32, render/graph/ms 8.06, render/present/ms 1.80, surface/cell_upload_ms 1.00, stream/drive/ms 0.77, terrain/update/ms 0.53, weather/map/ms 0.49, stream/tick/ms 0.38 |
| warm-up | 1.21 | 99.4 | render/thread/ms 61.40, render/busy/ms 59.52, main/busy/ms 59.16, render/graph/ms 4.82, render/present/ms 1.64, stream/drive/ms 1.61, stream/tick/ms 1.33, terrain/update/ms 0.50, terrain/select/ms 0.37, render/acquire/ms 0.24, bigspace/total/ms 0.23, weather/map/ms 0.12 |
| measure | 35.52 | 32.0 | render/thread/ms 41.95, render/busy/ms 40.35, render/graph/ms 19.24, main/busy/ms 18.08, render/present/ms 1.16, controller/move/ms 1.16, bigspace/total/ms 0.74, terrain/update/ms 0.67, anim/apply/ms 0.63, terrain/select/ms 0.46, controller/feed/ms 0.45, render/acquire/ms 0.43 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\flight@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (278 samples): GPU load **77%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.0 W · 73 °C (max 76) · VRAM 3876 MiB · throttled 100% of samples
