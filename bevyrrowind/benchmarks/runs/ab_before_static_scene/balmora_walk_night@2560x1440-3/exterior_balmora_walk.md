# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6006 frames over 65.6 s · **median 10.45 ms (96 fps)** · mean 10.93 · p95 14.25 · p99 16.07 (1% low 62 fps) · worst 28.16 ms · 79 spikes over 1.5× median · 13.7 stutters/min (work 16.5/min)
- warm-up: 405 frames over 5.0 s, worst 511.5 ms
- hitches (> 26.1 ms): **1** measured, 5 in warm-up
- timed GPU passes: 7.05 ms of a 10.93 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.45 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- pass worst frame 28.16 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.146 | 2.660 | 0.188 |
| main_opaque_pass_3d | 1.245 | 1.655 | 0.045 |
| early prepass | 0.483 | 0.733 | 0.079 |
| lightprobe_irradiance_map | 0.436 | 0.588 | 0.001 |
| bloom | 0.329 | 0.329 | 0.069 |
| atmosphere_luts | 0.329 | 0.438 | 0.003 |
| auxiliary_views | 0.247 | 0.954 | 0.220 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.215 | 0.212 | 0.007 |
| uniform_allocation | 0.171 | 0.402 | 0.017 |
| water_fft | 0.128 | 0.129 | 0.002 |
| upscaling | 0.127 | 0.134 | 0.005 |
| bin_unpacking | 0.109 | 0.129 | 0.012 |
| tonemapping | 0.089 | 0.089 | 0.004 |
| water_velocity | 0.083 | 0.085 | 0.002 |
| atmos_froxel_inject | 0.080 | 0.082 | 0.002 |
| clustering | 0.071 | 0.069 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.106 | 0.107 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| clouds_trace | 0.053 | 0.073 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.002 |
| clouds_resolve | 0.035 | 0.040 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.018 | 0.016 | 0.004 |
| night_sky | 0.016 | 0.038 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.478 | 12.723 | 26.95 | 56926.9 |
| render/busy/ms | 8.944 | 12.125 | 26.10 | 53715.2 |
| main/busy/ms | 8.041 | 10.073 | 19.91 | 48295.9 |
| render/graph/ms | 4.438 | 6.461 | 15.94 | 26653.3 |
| render/present/ms | 0.438 | 0.553 | 4.99 | 2631.1 |
| bigspace/total/ms | 0.438 | 0.718 | 2.35 | 2629.2 |
| terrain/update/ms | 0.284 | 0.365 | 1.53 | 1703.9 |
| terrain/select/ms | 0.251 | 0.324 | 0.73 | 1507.4 |
| controller/feed/ms | 0.235 | 0.330 | 0.64 | 1414.1 |
| stream/drive/ms | 0.228 | 0.376 | 2.81 | 1368.3 |
| far/select/ms | 0.215 | 0.365 | 0.81 | 1293.6 |
| stream/tick/ms | 0.184 | 0.289 | 0.78 | 1105.7 |
| controller/move/ms | 0.176 | 0.407 | 1.09 | 1056.4 |
| bigspace/low_precision/ms | 0.170 | 0.274 | 1.50 | 1021.9 |
| anim/evaluate_cpu/ms | 0.169 | 0.227 | 0.62 | 1015.5 |
| anim/evaluate/ms | 0.155 | 0.303 | 2.10 | 928.9 |
| bigspace/high_precision/ms | 0.102 | 0.175 | 1.94 | 612.7 |
| render/acquire/ms | 0.097 | 0.127 | 4.73 | 580.7 |
| anim/apply/ms | 0.088 | 0.142 | 1.95 | 528.4 |
| physics/step/ms | 0.050 | 0.089 | 0.76 | 298.3 |
| physics/insert/ms | 0.039 | 0.382 | 1.03 | 235.7 |
| weather/map/ms | 0.016 | 0.020 | 0.07 | 95.3 |
| controller/avoid/ms | 0.012 | 0.018 | 0.16 | 69.7 |
| weather/eval/ms | 0.009 | 0.012 | 0.18 | 56.9 |
| script/update/ms | 0.007 | 0.009 | 0.30 | 40.3 |
| script/input/ms | 0.005 | 0.008 | 0.15 | 32.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.1 |
| script/ui/ms | 0.005 | 0.007 | 0.13 | 27.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 25.9 |
| controller/write/ms | 0.004 | 0.007 | 0.12 | 22.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.75 | 14.3 |
| shapes/activate/ms | 0.001 | 0.002 | 0.21 | 6.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 14 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 39 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
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
| frame/work/ms | ms | 8.52 | 26.10 | 9.90 |
| gpu/timed/ms | ms | 6.99 | 10.93 | 7.72 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 716 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 173 | 173 |
| pipelines/waiting | count | 0 | 23 | 0 |
| process/cpu_usage | % | 23.88 | 35.71 | 12.61 |
| process/mem_usage | GiB | 2.54 | 2.58 | 2.58 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3481 | 3543 | 3543 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.81 | 54.38 | 54.38 |
| system/mem_usage | % | 47.32 | 48.38 | 48.38 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.95 | 510 | 282.0 k |
| anim/events/count | count | 0.20 | 4 | 1216 |
| assets/Image/added | count | 0.06 | 35 | 347 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 175 | 4184 |
| assets/Mesh/removed | count | 0.81 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 498 |
| assets/StandardMaterial/modified | count | 3.61 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 1.55 | 2127 | 9292 |
| physics/inserts | count | 0.49 | 142 | 2966 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 129 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 6 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 23 | 378 |
| stream/upload/bytes | bytes | 5636 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1236 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.1 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 511.5 | main/busy/ms 488, surface/advance_cpu_ms 0.75, surface/cell_upload_ms 0.36, stream/drive/ms 0.31, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.03, weather/map/ms 0.03, script/update/ms 0.02, weather/eval/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.78 | 270.8 | main/busy/ms 521, render/thread/ms 237, render/busy/ms 191, render/acquire/ms 44.66, render/graph/ms 9.87, render/present/ms 1.71, stream/drive/ms 1.07, surface/advance_cpu_ms 0.75, terrain/update/ms 0.67, stream/tick/ms 0.53, terrain/select/ms 0.49, surface/cell_upload_ms 0.36 |
| warm-up | 0.90 | 118.7 | render/thread/ms 253, render/busy/ms 206, main/busy/ms 47.65, render/acquire/ms 44.85, terrain/update/ms 18.67, render/graph/ms 14.50, render/present/ms 2.67, stream/drive/ms 1.17, terrain/select/ms 0.92, stream/tick/ms 0.78, bigspace/total/ms 0.23, bigspace/high_precision/ms 0.18 |
| warm-up | 1.13 | 87.0 | main/busy/ms 58.29, render/thread/ms 56.33, render/busy/ms 54.67, render/graph/ms 4.42, render/present/ms 1.46, stream/drive/ms 1.43, stream/tick/ms 0.91, terrain/update/ms 0.87, terrain/select/ms 0.72, bigspace/total/ms 0.49, bigspace/high_precision/ms 0.34, render/acquire/ms 0.20 |
| warm-up | 3.98 | 30.9 | main/busy/ms 46.38, render/thread/ms 29.96, render/busy/ms 28.65, render/graph/ms 6.43, terrain/update/ms 5.65, physics/insert/ms 1.13, anim/evaluate_cpu/ms 1.12, render/present/ms 1.02, bigspace/total/ms 0.94, stream/drive/ms 0.92, terrain/select/ms 0.79, anim/evaluate/ms 0.54 |
| measure | 2.18 | 28.2 | render/thread/ms 39.38, render/busy/ms 37.80, render/graph/ms 21.55, main/busy/ms 18.54, render/present/ms 1.33, bigspace/total/ms 1.09, terrain/update/ms 0.53, terrain/select/ms 0.48, far/select/ms 0.48, stream/drive/ms 0.47, anim/evaluate_cpu/ms 0.40, stream/tick/ms 0.40 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 13%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (256 samples): GPU load **73%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.6 W · 72 °C (max 78) · VRAM 3695 MiB · throttled 100% of samples
