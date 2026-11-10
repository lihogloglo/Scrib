# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5645 frames over 71.5 s · **median 12.28 ms (81 fps)** · mean 12.67 · p95 16.69 · p99 20.64 (1% low 48 fps) · worst 72.65 ms · 132 spikes over 1.5× median · 78.0 stutters/min (work 99.0/min)
- warm-up: 245 frames over 3.5 s, worst 630.1 ms
- hitches (> 30.7 ms): **7** measured, 5 in warm-up
- timed GPU passes: 8.23 ms of a 12.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.28 ms, limit 11 ms
- **FAIL** frame_ms: 93 stutters (> 1.5x moving median of 60) in 1.19 min = 78.02/min, limit 1/min
- **FAIL** worst frame 72.65 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.205 | 2.879 | 0.054 |
| dlss_super_resolution | 2.079 | 2.403 | 0.192 |
| early prepass | 0.583 | 1.057 | 0.101 |
| lightprobe_irradiance_map | 0.444 | 0.756 | 0.001 |
| atmosphere_luts | 0.385 | 0.667 | 0.004 |
| bloom | 0.317 | 0.318 | 0.073 |
| lightprobe_radiance_map | 0.226 | 0.234 | 0.004 |
| clouds_trace | 0.225 | 0.262 | 0.001 |
| main_transparent_pass_3d | 0.210 | 0.467 | 0.089 |
| ssao | 0.208 | 0.209 | 0.008 |
| uniform_allocation | 0.176 | 0.210 | 0.019 |
| water_fft | 0.128 | 0.129 | 0.003 |
| bin_unpacking | 0.128 | 0.154 | 0.015 |
| upscaling | 0.116 | 0.123 | 0.005 |
| atmos_froxel_inject | 0.090 | 0.092 | 0.002 |
| tonemapping | 0.087 | 0.090 | 0.004 |
| water_velocity | 0.076 | 0.095 | 0.002 |
| clustering | 0.071 | 0.069 | 0.022 |
| clouds_composite | 0.065 | 0.067 | 0.002 |
| surface_trail | 0.060 | 0.057 | 0.003 |
| render_sky | 0.052 | 0.052 | 0.002 |
| clouds_resolve | 0.037 | 0.044 | 0.001 |
| early_mesh_preprocessing | 0.036 | 0.040 | 0.006 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.029 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.030 | 0.007 |
| lightprobe_copy | 0.014 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| weather_map | 0.013 | 0.013 | 0.001 |
| auxiliary_views | 0.012 | 0.002 | 0.014 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.802 | 14.447 | 69.87 | 60979.6 |
| render/busy/ms | 10.225 | 13.856 | 68.35 | 57721.9 |
| main/busy/ms | 8.398 | 10.944 | 97.86 | 47406.8 |
| render/graph/ms | 4.819 | 6.264 | 24.02 | 27200.7 |
| render/present/ms | 0.466 | 0.638 | 3.25 | 2627.8 |
| bigspace/total/ms | 0.443 | 0.802 | 4.08 | 2498.7 |
| stream/drive/ms | 0.329 | 0.910 | 10.04 | 1858.2 |
| terrain/update/ms | 0.261 | 0.493 | 2.38 | 1471.4 |
| controller/feed/ms | 0.246 | 0.414 | 0.85 | 1387.6 |
| far/select/ms | 0.230 | 0.416 | 0.93 | 1300.1 |
| terrain/select/ms | 0.193 | 0.259 | 0.76 | 1090.8 |
| stream/tick/ms | 0.183 | 0.301 | 0.78 | 1032.6 |
| bigspace/low_precision/ms | 0.147 | 0.267 | 3.75 | 831.7 |
| controller/move/ms | 0.147 | 0.340 | 0.97 | 827.2 |
| anim/evaluate_cpu/ms | 0.135 | 0.229 | 1.44 | 759.7 |
| bigspace/high_precision/ms | 0.113 | 0.210 | 2.93 | 635.5 |
| render/acquire/ms | 0.112 | 0.151 | 3.00 | 629.9 |
| anim/apply/ms | 0.111 | 0.207 | 2.41 | 628.9 |
| anim/evaluate/ms | 0.105 | 0.238 | 3.57 | 592.9 |
| physics/step/ms | 0.090 | 0.233 | 1.75 | 507.4 |
| stream/despawn/ms | 0.050 | 0.494 | 1.85 | 280.4 |
| stream/spawn/ms | 0.038 | 0.305 | 1.55 | 216.2 |
| weather/map/ms | 0.027 | 0.037 | 0.27 | 154.6 |
| physics/insert/ms | 0.024 | 0.180 | 1.00 | 137.2 |
| controller/avoid/ms | 0.011 | 0.018 | 0.36 | 61.1 |
| weather/eval/ms | 0.011 | 0.016 | 0.17 | 60.2 |
| script/update/ms | 0.008 | 0.012 | 0.16 | 43.7 |
| stream/deactivate/ms | 0.007 | 0.000 | 0.53 | 36.9 |
| script/input/ms | 0.006 | 0.009 | 0.23 | 33.9 |
| script/ui/ms | 0.006 | 0.009 | 0.29 | 32.7 |
| stream/activate/ms | 0.006 | 0.000 | 0.52 | 31.4 |
| controller/write/ms | 0.003 | 0.006 | 0.43 | 19.7 |
| stream/upload/ms | 0.002 | 0.015 | 0.12 | 9.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 5.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.13 | 3.9 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 3.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 597 | 778 | 471 |
| anim/characters/count | count | 597 | 782 | 471 |
| anim/evaluated/count | count | 18 | 47 | 14 |
| controller/movers/count | count | 18 | 32 | 14 |
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
| frame/work/ms | ms | 9.89 | 97.86 | 8.19 |
| gpu/timed/ms | ms | 8.22 | 11.73 | 7.04 |
| mesh_allocator_allocations |  meshes | 15.8 k | 19.3 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 537.07 M | 645.72 M | 645.72 M |
| physics/awake | count | 0 | 3 | 0 |
| physics/bodies | count | 1892 | 2692 | 1956 |
| physics/items | count | 10 | 107 | 107 |
| physics/pending | count | 0 | 992 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 30 | 0 |
| process/cpu_usage | % | 22.62 | 37.50 | 21.90 |
| process/mem_usage | GiB | 2.51 | 2.58 | 2.58 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3879 | 4858 | 4858 |
| stream/activate/queue | count | 0 | 2 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 76 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 205 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.58 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 171 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 8 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.35 | 85.40 | 30.64 |
| system/mem_usage | % | 49.41 | 50.60 | 48.53 |
| terrain/patches | count | 349 | 496 | 319 |
| world/meshes | count | 26.7 k | 33.5 k | 28.1 k |
| world/point_lights | count | 271 | 549 | 524 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 9.21 | 200 | 52.0 k |
| anim/events/count | count | 0.21 | 5 | 1173 |
| assets/Image/added | count | 0.16 | 41 | 909 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 7.15 | 494 | 40.4 k |
| assets/Mesh/removed | count | 11.72 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.21 | 46 | 1170 |
| assets/StandardMaterial/modified | count | 6.04 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.08 | 36 | 470 |
| assets/WorldAsset/added | count | 0.01 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 12 | 124 |
| ecs/entities_delta | count | 4.08 | 2007 | 23.0 k |
| physics/inserts | count | 2.75 | 148 | 15.5 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.51 | 288 | 14.2 k |
| physics/steps | count | 0.76 | 3 | 4291 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 5 | 32 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.24 | 24 | 1345 |
| stream/deactivate/items | count | 0.24 | 26 | 1345 |
| stream/despawn/items | count | 1.24 | 299 | 6982 |
| stream/fetch/items | count | 0.26 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.81 | 29 | 4586 |
| stream/upload/bytes | bytes | 87.7 k | 3.54 M | 495.17 M |
| stream/upload/items | count | 0.23 | 8 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.05 | 1 | 269 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 20.3 k | 1.21 M | 114.83 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 30.7 ms; slowest 12, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.63 | 630.1 | main/busy/ms 608, surface/advance_cpu_ms 84.83, surface/cell_upload_ms 0.71, stream/drive/ms 0.31, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.13, weather/map/ms 0.12, stream/tick/ms 0.08, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 1.01 | 379.9 | main/busy/ms 643, render/thread/ms 345, render/busy/ms 275, surface/advance_cpu_ms 84.83, render/acquire/ms 68.65, render/graph/ms 4.91, render/present/ms 1.94, stream/drive/ms 1.03, surface/cell_upload_ms 0.71, stream/tick/ms 0.55, terrain/update/ms 0.44, bigspace/total/ms 0.36 |
| warm-up | 1.28 | 102.5 | main/busy/ms 68.15, render/thread/ms 59.24, render/busy/ms 57.15, render/graph/ms 5.09, render/present/ms 1.86, stream/drive/ms 1.85, stream/tick/ms 1.56, terrain/update/ms 0.88, terrain/select/ms 0.70, bigspace/total/ms 0.47, render/acquire/ms 0.23, bigspace/low_precision/ms 0.22 |
| warm-up | 2.80 | 32.5 | main/busy/ms 60.84, render/thread/ms 22.80, render/busy/ms 21.33, render/graph/ms 6.35, terrain/update/ms 4.85, render/present/ms 1.25, stream/drive/ms 1.19, bigspace/total/ms 0.80, anim/evaluate_cpu/ms 0.71, stream/tick/ms 0.47, terrain/select/ms 0.43, stream/spawn/ms 0.39 |
| warm-up | 2.85 | 46.2 | main/busy/ms 47.62, render/thread/ms 36.39, render/busy/ms 35.01, terrain/update/ms 7.32, render/graph/ms 6.28, render/present/ms 1.16, stream/drive/ms 1.06, bigspace/total/ms 0.85, anim/evaluate_cpu/ms 0.58, stream/tick/ms 0.43, terrain/select/ms 0.43, stream/spawn/ms 0.33 |
| measure | 0.55 | 58.9 | main/busy/ms 88.08, render/thread/ms 68.83, render/busy/ms 66.10, render/graph/ms 20.07, render/present/ms 2.33, stream/drive/ms 2.10, stream/spawn/ms 0.95, bigspace/total/ms 0.93, stream/tick/ms 0.52, terrain/update/ms 0.48, far/select/ms 0.44, terrain/select/ms 0.40 |
| measure | 0.59 | 33.1 | main/busy/ms 88.42, render/thread/ms 68.34, render/busy/ms 65.92, render/graph/ms 19.91, terrain/update/ms 2.28, render/present/ms 2.09, stream/drive/ms 1.56, stream/spawn/ms 0.95, bigspace/total/ms 0.94, terrain/select/ms 0.51, stream/tick/ms 0.51, far/select/ms 0.40 |
| measure | 0.81 | 72.6 | main/busy/ms 107, render/thread/ms 83.22, render/busy/ms 80.86, render/graph/ms 15.21, render/present/ms 1.82, bigspace/total/ms 0.91, stream/drive/ms 0.66, render/acquire/ms 0.54, stream/tick/ms 0.48, terrain/update/ms 0.45, far/select/ms 0.41, terrain/select/ms 0.38 |
| measure | 0.85 | 46.5 | main/busy/ms 111, render/thread/ms 85.91, render/busy/ms 83.73, render/graph/ms 11.15, render/present/ms 1.61, bigspace/total/ms 0.92, stream/drive/ms 0.71, render/acquire/ms 0.57, terrain/update/ms 0.55, stream/tick/ms 0.53, controller/feed/ms 0.47, terrain/select/ms 0.46 |
| measure | 0.88 | 32.1 | render/thread/ms 46.87, render/busy/ms 45.56, main/busy/ms 41.17, render/graph/ms 27.22, render/present/ms 1.07, terrain/update/ms 1.05, bigspace/total/ms 0.64, terrain/select/ms 0.51, stream/drive/ms 0.48, controller/feed/ms 0.46, stream/tick/ms 0.41, far/select/ms 0.40 |
| measure | 35.51 | 33.3 | render/thread/ms 45.09, render/busy/ms 43.35, render/graph/ms 22.62, main/busy/ms 21.00, render/present/ms 1.51, bigspace/total/ms 1.08, controller/feed/ms 0.69, terrain/update/ms 0.63, controller/move/ms 0.51, far/select/ms 0.50, stream/drive/ms 0.49, terrain/select/ms 0.44 |
| measure | 42.65 | 32.1 | render/thread/ms 40.44, render/busy/ms 39.05, main/busy/ms 21.33, render/graph/ms 11.86, bigspace/total/ms 4.27, bigspace/low_precision/ms 3.48, render/present/ms 1.15, controller/feed/ms 0.73, far/select/ms 0.69, stream/drive/ms 0.68, terrain/update/ms 0.57, stream/tick/ms 0.53 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_before\flight@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (276 samples): GPU load **74%** (min 46%) · 2581 MHz (min 2580) · memory 8001 MHz · 76.4 W · 71 °C (max 76) · VRAM 3965 MiB · throttled 100% of samples
