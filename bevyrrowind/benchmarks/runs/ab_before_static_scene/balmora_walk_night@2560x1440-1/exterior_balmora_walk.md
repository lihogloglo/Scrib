# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6128 frames over 65.7 s · **median 10.33 ms (97 fps)** · mean 10.71 · p95 13.52 · p99 15.14 (1% low 66 fps) · worst 101.28 ms · 50 spikes over 1.5× median · 17.4 stutters/min (work 21.9/min)
- warm-up: 375 frames over 4.8 s, worst 516.1 ms
- hitches (> 25.8 ms): **3** measured, 8 in warm-up
- timed GPU passes: 7.06 ms of a 10.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.33 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.36/min, limit 1/min
- **FAIL** worst frame 101.28 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.098 | 2.434 | 0.180 |
| main_opaque_pass_3d | 1.226 | 1.594 | 0.045 |
| early prepass | 0.503 | 1.054 | 0.075 |
| lightprobe_irradiance_map | 0.440 | 0.736 | 0.001 |
| bloom | 0.324 | 0.327 | 0.067 |
| atmosphere_luts | 0.323 | 0.389 | 0.003 |
| auxiliary_views | 0.298 | 1.094 | 0.252 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.213 | 0.211 | 0.007 |
| uniform_allocation | 0.178 | 0.477 | 0.016 |
| water_fft | 0.127 | 0.129 | 0.002 |
| upscaling | 0.126 | 0.132 | 0.004 |
| bin_unpacking | 0.110 | 0.125 | 0.012 |
| tonemapping | 0.088 | 0.089 | 0.003 |
| water_velocity | 0.082 | 0.085 | 0.001 |
| atmos_froxel_inject | 0.078 | 0.081 | 0.001 |
| clustering | 0.076 | 0.070 | 0.019 |
| main_transparent_pass_3d | 0.064 | 0.106 | 0.107 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| clouds_trace | 0.053 | 0.072 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.002 |
| clouds_resolve | 0.036 | 0.040 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.022 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.019 | 0.014 | 0.001 |
| night_sky | 0.015 | 0.035 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.271 | 12.033 | 99.45 | 56809.7 |
| render/busy/ms | 8.741 | 11.494 | 98.74 | 53563.7 |
| main/busy/ms | 7.833 | 9.642 | 34.32 | 48002.9 |
| render/graph/ms | 4.469 | 6.375 | 89.39 | 27388.9 |
| render/present/ms | 0.435 | 0.536 | 1.70 | 2665.6 |
| bigspace/total/ms | 0.423 | 0.671 | 2.00 | 2594.8 |
| terrain/update/ms | 0.280 | 0.360 | 1.89 | 1716.3 |
| terrain/select/ms | 0.248 | 0.317 | 0.86 | 1516.9 |
| controller/feed/ms | 0.232 | 0.311 | 1.19 | 1418.9 |
| stream/drive/ms | 0.224 | 0.374 | 2.47 | 1375.5 |
| far/select/ms | 0.214 | 0.360 | 0.67 | 1314.0 |
| stream/tick/ms | 0.182 | 0.297 | 0.63 | 1115.2 |
| anim/evaluate_cpu/ms | 0.172 | 0.217 | 0.74 | 1055.5 |
| bigspace/low_precision/ms | 0.163 | 0.266 | 1.41 | 997.3 |
| controller/move/ms | 0.150 | 0.296 | 0.64 | 922.1 |
| anim/evaluate/ms | 0.144 | 0.261 | 1.80 | 880.3 |
| bigspace/high_precision/ms | 0.101 | 0.181 | 1.61 | 620.6 |
| render/acquire/ms | 0.095 | 0.122 | 6.48 | 580.4 |
| anim/apply/ms | 0.084 | 0.128 | 1.21 | 514.6 |
| physics/step/ms | 0.045 | 0.083 | 0.60 | 278.7 |
| physics/insert/ms | 0.040 | 0.377 | 0.88 | 245.2 |
| weather/map/ms | 0.016 | 0.021 | 0.07 | 98.0 |
| controller/avoid/ms | 0.012 | 0.018 | 0.51 | 71.3 |
| weather/eval/ms | 0.009 | 0.012 | 0.10 | 56.0 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 40.7 |
| script/input/ms | 0.005 | 0.007 | 0.17 | 31.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.16 | 30.5 |
| script/ui/ms | 0.004 | 0.006 | 0.16 | 26.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 24.9 |
| controller/write/ms | 0.004 | 0.007 | 0.05 | 24.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
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
| frame/work/ms | ms | 8.45 | 98.74 | 7.97 |
| gpu/timed/ms | ms | 7.06 | 10.50 | 7.14 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 418 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 173 | 173 |
| pipelines/waiting | count | 0 | 22 | 0 |
| process/cpu_usage | % | 24.64 | 37.02 | 25 |
| process/mem_usage | GiB | 2.57 | 2.61 | 2.61 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3467 | 3520 | 3520 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.36 | 53.47 | 33.40 |
| system/mem_usage | % | 45.10 | 45.21 | 45.10 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 40.32 | 194 | 247.1 k |
| anim/events/count | count | 0.18 | 3 | 1109 |
| assets/Image/added | count | 0.05 | 35 | 319 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.67 | 160 | 4133 |
| assets/Mesh/removed | count | 0.80 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 470 |
| assets/StandardMaterial/modified | count | 3.53 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.95 | 1731 | 5844 |
| physics/inserts | count | 0.43 | 105 | 2660 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 3 | 3936 |
| physics/steps_dropped | count | 0.00 | 3 | 3 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 21 | 378 |
| stream/upload/bytes | bytes | 5524 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1211 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 11, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.52 | 516.1 | main/busy/ms 493, surface/advance_cpu_ms 0.74, surface/cell_upload_ms 0.45, stream/drive/ms 0.27, stream/tick/ms 0.07, bigspace/total/ms 0.05, weather/map/ms 0.04, bigspace/high_precision/ms 0.03, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, controller/feed/ms 0.01 |
| warm-up | 0.83 | 312.2 | main/busy/ms 527, render/thread/ms 277, render/busy/ms 205, render/acquire/ms 70.04, render/graph/ms 4.57, render/present/ms 2.09, stream/drive/ms 1.05, surface/advance_cpu_ms 0.74, terrain/update/ms 0.70, stream/tick/ms 0.57, terrain/select/ms 0.45, surface/cell_upload_ms 0.45 |
| warm-up | 1.00 | 170.1 | render/thread/ms 298, render/busy/ms 224, render/acquire/ms 70.22, main/busy/ms 54.49, terrain/update/ms 19.93, render/graph/ms 9.49, render/present/ms 3.63, stream/drive/ms 1.17, stream/tick/ms 0.82, terrain/select/ms 0.77, bigspace/total/ms 0.53, bigspace/high_precision/ms 0.24 |
| warm-up | 1.10 | 30.5 | render/thread/ms 47.31, render/busy/ms 24.14, render/acquire/ms 21.34, main/busy/ms 19.18, terrain/update/ms 4.22, render/graph/ms 4.16, stream/drive/ms 3.41, stream/tick/ms 2.72, render/present/ms 1.83, terrain/select/ms 0.52, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.11 |
| warm-up | 1.19 | 32.8 | main/busy/ms 63.02, render/thread/ms 50.03, render/busy/ms 25.38, render/acquire/ms 23.29, render/graph/ms 4.92, stream/drive/ms 3.17, stream/tick/ms 2.81, render/present/ms 1.35, terrain/update/ms 0.82, terrain/select/ms 0.66, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.07 |
| warm-up | 1.30 | 111.0 | render/thread/ms 115, render/busy/ms 102, main/busy/ms 86.04, render/acquire/ms 11.79, render/graph/ms 4.49, stream/drive/ms 3.05, stream/tick/ms 2.65, bigspace/total/ms 1.57, bigspace/high_precision/ms 1.36, render/present/ms 1.29, terrain/update/ms 0.74, terrain/select/ms 0.61 |
| warm-up | 1.83 | 30.0 | render/thread/ms 27.13, render/busy/ms 25.81, main/busy/ms 19.95, stream/drive/ms 7.51, stream/spawn/ms 5.97, render/graph/ms 4.38, render/present/ms 1.06, stream/tick/ms 0.72, terrain/update/ms 0.63, terrain/select/ms 0.50, bigspace/total/ms 0.45, render/acquire/ms 0.26 |
| warm-up | 2.33 | 41.3 | render/thread/ms 41.33, render/busy/ms 40.01, main/busy/ms 32.97, render/graph/ms 4.72, stream/drive/ms 3.22, stream/spawn/ms 1.20, physics/insert/ms 1.13, render/present/ms 1.10, stream/tick/ms 0.70, terrain/update/ms 0.57, bigspace/total/ms 0.49, terrain/select/ms 0.48 |
| measure | 0.48 | 101.3 | render/thread/ms 119, render/busy/ms 118, render/graph/ms 99.28, main/busy/ms 20.40, render/present/ms 1.56, physics/insert/ms 1.27, bigspace/total/ms 0.59, stream/drive/ms 0.56, far/select/ms 0.53, terrain/update/ms 0.51, stream/tick/ms 0.50, terrain/select/ms 0.45 |
| measure | 1.08 | 26.0 | render/thread/ms 32.16, render/busy/ms 31.21, render/graph/ms 23.03, main/busy/ms 16.65, bigspace/total/ms 1.02, render/present/ms 0.77, terrain/update/ms 0.53, physics/insert/ms 0.50, terrain/select/ms 0.47, stream/drive/ms 0.41, far/select/ms 0.39, stream/tick/ms 0.36 |
| measure | 5.12 | 37.3 | render/thread/ms 46.08, render/busy/ms 45.13, main/busy/ms 42.45, render/graph/ms 36.42, bigspace/total/ms 0.85, render/present/ms 0.79, terrain/update/ms 0.52, terrain/select/ms 0.46, stream/drive/ms 0.45, bigspace/low_precision/ms 0.42, controller/feed/ms 0.41, stream/tick/ms 0.40 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **74%** (min 49%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.6 W · 71 °C (max 78) · VRAM 4060 MiB · throttled 100% of samples
