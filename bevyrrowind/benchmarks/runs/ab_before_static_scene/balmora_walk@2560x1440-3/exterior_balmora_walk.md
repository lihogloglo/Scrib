# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5219 frames over 65.6 s · **median 12.28 ms (81 fps)** · mean 12.58 · p95 15.97 · p99 19.45 (1% low 51 fps) · worst 144.67 ms · 93 spikes over 1.5× median · 53.9 stutters/min (work 65.8/min)
- warm-up: 396 frames over 5.2 s, worst 576.8 ms
- hitches (> 30.7 ms): **6** measured, 6 in warm-up
- timed GPU passes: 7.30 ms of a 12.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.28 ms, limit 11 ms
- **FAIL** frame_ms: 59 stutters (> 1.5x moving median of 60) in 1.09 min = 53.93/min, limit 1/min
- **FAIL** worst frame 144.67 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.136 | 2.542 | 0.202 |
| main_opaque_pass_3d | 1.443 | 1.971 | 0.052 |
| early prepass | 0.485 | 0.886 | 0.088 |
| lightprobe_irradiance_map | 0.435 | 0.629 | 0.001 |
| bloom | 0.325 | 0.329 | 0.078 |
| atmosphere_luts | 0.323 | 0.460 | 0.004 |
| lightprobe_radiance_map | 0.226 | 0.236 | 0.005 |
| ssao | 0.212 | 0.210 | 0.008 |
| uniform_allocation | 0.189 | 0.322 | 0.021 |
| auxiliary_views | 0.178 | 0.624 | 0.178 |
| water_fft | 0.131 | 0.131 | 0.003 |
| bin_unpacking | 0.129 | 0.148 | 0.016 |
| clouds_trace | 0.126 | 0.204 | 0.001 |
| upscaling | 0.122 | 0.140 | 0.005 |
| atmos_froxel_inject | 0.096 | 0.101 | 0.002 |
| tonemapping | 0.089 | 0.090 | 0.004 |
| water_velocity | 0.083 | 0.085 | 0.002 |
| clustering | 0.066 | 0.068 | 0.021 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.104 | 0.117 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.048 | 0.006 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.022 | 0.024 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.004 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.011 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.932 | 14.157 | 34.53 | 57052.5 |
| render/busy/ms | 10.366 | 13.511 | 32.78 | 54097.9 |
| main/busy/ms | 8.959 | 11.082 | 66.39 | 46756.3 |
| render/graph/ms | 5.008 | 6.563 | 18.62 | 26136.9 |
| render/present/ms | 0.462 | 0.602 | 3.16 | 2412.1 |
| bigspace/total/ms | 0.455 | 0.670 | 6.56 | 2373.4 |
| terrain/update/ms | 0.293 | 0.382 | 1.90 | 1526.8 |
| controller/feed/ms | 0.276 | 0.402 | 1.15 | 1441.3 |
| terrain/select/ms | 0.258 | 0.336 | 0.81 | 1344.2 |
| stream/drive/ms | 0.245 | 0.407 | 3.75 | 1276.5 |
| far/select/ms | 0.236 | 0.405 | 1.34 | 1231.9 |
| stream/tick/ms | 0.197 | 0.306 | 1.18 | 1026.7 |
| anim/evaluate_cpu/ms | 0.183 | 0.244 | 0.48 | 957.4 |
| bigspace/low_precision/ms | 0.172 | 0.264 | 1.66 | 895.9 |
| controller/move/ms | 0.161 | 0.317 | 0.82 | 839.6 |
| anim/evaluate/ms | 0.160 | 0.362 | 1.82 | 833.1 |
| render/acquire/ms | 0.104 | 0.144 | 0.80 | 542.5 |
| bigspace/high_precision/ms | 0.103 | 0.157 | 1.67 | 536.7 |
| anim/apply/ms | 0.101 | 0.181 | 1.35 | 526.6 |
| physics/step/ms | 0.061 | 0.109 | 0.68 | 316.0 |
| physics/insert/ms | 0.056 | 0.441 | 1.08 | 293.9 |
| weather/map/ms | 0.027 | 0.035 | 0.27 | 139.1 |
| controller/avoid/ms | 0.012 | 0.018 | 0.21 | 63.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.16 | 58.8 |
| script/update/ms | 0.008 | 0.011 | 0.09 | 39.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 30.5 |
| script/ui/ms | 0.006 | 0.008 | 0.23 | 30.5 |
| script/input/ms | 0.006 | 0.008 | 0.06 | 29.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.29 | 25.2 |
| controller/write/ms | 0.004 | 0.007 | 0.16 | 22.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.73 | 16.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| ecs/entities | count | 59.1 k | 68.1 k | 59.1 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 10.16 | 66.39 | 7.68 |
| gpu/timed/ms | ms | 7.30 | 10.47 | 7.68 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 584 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 18 | 0 |
| process/cpu_usage | % | 23.53 | 38.39 | 37.33 |
| process/mem_usage | GiB | 2.60 | 2.66 | 2.66 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3482 | 3540 | 3540 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 6 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 6 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 6 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 3 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 43.24 | 93.17 | 35.48 |
| system/mem_usage | % | 47.48 | 50.74 | 50.54 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.55 | 219 | 269.1 k |
| anim/events/count | count | 0.30 | 7 | 1588 |
| assets/Image/added | count | 0.06 | 35 | 319 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 183 | 4111 |
| assets/Mesh/removed | count | 0.94 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 477 |
| assets/StandardMaterial/modified | count | 4.14 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| ecs/entities_delta | count | 1.18 | 1494 | 6173 |
| physics/inserts | count | 0.56 | 109 | 2912 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 113 | 2335 |
| physics/steps | count | 0.75 | 3 | 3932 |
| physics/steps_dropped | count | 0.00 | 6 | 6 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 6 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6486 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 4 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1422 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.7 ms; slowest 12, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 576.8 | main/busy/ms 551, surface/advance_cpu_ms 67.34, surface/cell_upload_ms 0.75, stream/drive/ms 0.34, weather/map/ms 0.11, stream/tick/ms 0.11, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 0.83 | 252.5 | main/busy/ms 590, render/thread/ms 214, render/busy/ms 166, surface/advance_cpu_ms 67.34, render/acquire/ms 45.67, render/graph/ms 6.80, render/present/ms 2.09, stream/drive/ms 1.45, stream/tick/ms 0.84, surface/cell_upload_ms 0.75, terrain/update/ms 0.35, bigspace/total/ms 0.31 |
| warm-up | 0.99 | 164.8 | render/thread/ms 233, render/busy/ms 183, main/busy/ms 55.31, render/acquire/ms 45.83, terrain/update/ms 22.85, render/graph/ms 11.40, render/present/ms 3.42, stream/drive/ms 1.50, stream/tick/ms 1.06, terrain/select/ms 0.67, bigspace/total/ms 0.50, bigspace/high_precision/ms 0.16 |
| warm-up | 1.22 | 82.5 | main/busy/ms 59.62, render/thread/ms 50.76, render/busy/ms 49.26, render/graph/ms 3.76, stream/drive/ms 3.36, render/present/ms 1.23, stream/tick/ms 1.11, terrain/update/ms 0.71, terrain/select/ms 0.61, bigspace/total/ms 0.29, render/acquire/ms 0.27, bigspace/high_precision/ms 0.18 |
| warm-up | 1.38 | 30.8 | render/thread/ms 33.20, render/busy/ms 31.83, main/busy/ms 16.13, stream/drive/ms 4.98, render/graph/ms 4.61, stream/spawn/ms 1.99, render/present/ms 1.17, stream/tick/ms 1.03, terrain/update/ms 0.80, terrain/select/ms 0.69, render/acquire/ms 0.20, bigspace/total/ms 0.17 |
| warm-up | 3.15 | 37.1 | main/busy/ms 54.46, render/thread/ms 37.03, render/busy/ms 35.63, render/graph/ms 7.29, terrain/update/ms 4.06, stream/drive/ms 1.63, render/present/ms 1.19, anim/evaluate_cpu/ms 0.89, bigspace/total/ms 0.86, stream/tick/ms 0.81, anim/evaluate/ms 0.51, terrain/select/ms 0.49 |
| measure | 7.68 | 144.7 | main/busy/ms 18.11, render/thread/ms 11.21, render/busy/ms 10.68, render/graph/ms 5.47, bigspace/total/ms 0.94, terrain/update/ms 0.68, stream/drive/ms 0.64, terrain/select/ms 0.59, controller/feed/ms 0.56, stream/tick/ms 0.55, render/present/ms 0.45, far/select/ms 0.44 |
| measure | 34.23 | 35.2 | render/thread/ms 47.50, render/busy/ms 45.58, main/busy/ms 29.28, render/graph/ms 17.09, bigspace/total/ms 3.28, bigspace/low_precision/ms 2.12, render/present/ms 1.61, controller/feed/ms 0.82, stream/drive/ms 0.57, far/select/ms 0.57, terrain/update/ms 0.55, stream/tick/ms 0.50 |
| measure | 34.29 | 36.8 | render/thread/ms 54.61, render/busy/ms 51.86, main/busy/ms 44.94, render/graph/ms 23.44, render/present/ms 1.79, bigspace/total/ms 1.65, render/acquire/ms 0.95, terrain/update/ms 0.82, controller/feed/ms 0.81, stream/drive/ms 0.81, stream/tick/ms 0.74, anim/evaluate_cpu/ms 0.73 |
| measure | 46.59 | 32.7 | render/thread/ms 43.09, main/busy/ms 41.40, render/busy/ms 41.21, render/graph/ms 15.06, bigspace/total/ms 1.88, render/present/ms 1.58, bigspace/high_precision/ms 0.85, terrain/update/ms 0.83, controller/feed/ms 0.80, terrain/select/ms 0.74, far/select/ms 0.61, stream/drive/ms 0.59 |
| measure | 62.64 | 37.7 | main/busy/ms 74.75, render/thread/ms 42.75, render/busy/ms 38.85, render/graph/ms 24.11, bigspace/total/ms 7.03, render/present/ms 3.65, far/select/ms 0.81, anim/evaluate/ms 0.66, controller/feed/ms 0.66, terrain/update/ms 0.56, stream/drive/ms 0.53, terrain/select/ms 0.50 |
| measure | 62.68 | 45.7 | main/busy/ms 78.26, render/thread/ms 45.22, render/busy/ms 41.26, render/graph/ms 24.54, bigspace/total/ms 7.13, render/present/ms 3.71, controller/feed/ms 0.86, stream/drive/ms 0.78, terrain/update/ms 0.71, stream/tick/ms 0.68, terrain/select/ms 0.63, far/select/ms 0.51 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 5%
- during the run: other processes 20% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **68%** (min 14%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.4 W · 72 °C (max 77) · VRAM 4000 MiB · throttled 100% of samples
