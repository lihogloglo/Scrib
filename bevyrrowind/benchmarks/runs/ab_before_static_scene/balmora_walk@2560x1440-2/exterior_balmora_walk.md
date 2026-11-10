# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5658 frames over 65.6 s · **median 11.32 ms (88 fps)** · mean 11.60 · p95 14.21 · p99 16.24 (1% low 62 fps) · worst 24.57 ms · 39 spikes over 1.5× median · 27.4 stutters/min (work 35.6/min)
- warm-up: 389 frames over 4.9 s, worst 568.7 ms
- hitches (> 28.3 ms): **0** measured, 4 in warm-up
- timed GPU passes: 7.23 ms of a 11.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.32 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 24.57 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.114 | 2.434 | 0.190 |
| main_opaque_pass_3d | 1.422 | 1.857 | 0.047 |
| early prepass | 0.487 | 0.799 | 0.081 |
| lightprobe_irradiance_map | 0.428 | 0.427 | 0.001 |
| bloom | 0.323 | 0.329 | 0.072 |
| atmosphere_luts | 0.316 | 0.431 | 0.003 |
| lightprobe_radiance_map | 0.227 | 0.234 | 0.004 |
| ssao | 0.209 | 0.208 | 0.008 |
| uniform_allocation | 0.188 | 0.327 | 0.019 |
| auxiliary_views | 0.170 | 0.610 | 0.158 |
| bin_unpacking | 0.131 | 0.154 | 0.014 |
| clouds_trace | 0.127 | 0.213 | 0.001 |
| water_fft | 0.127 | 0.128 | 0.003 |
| upscaling | 0.122 | 0.154 | 0.005 |
| atmos_froxel_inject | 0.094 | 0.100 | 0.002 |
| tonemapping | 0.088 | 0.089 | 0.004 |
| water_velocity | 0.083 | 0.085 | 0.002 |
| clustering | 0.065 | 0.068 | 0.020 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.063 | 0.105 | 0.110 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.048 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.006 | 0.012 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.088 | 12.601 | 23.07 | 57075.6 |
| render/busy/ms | 9.548 | 12.044 | 22.39 | 54022.4 |
| main/busy/ms | 8.379 | 10.120 | 17.68 | 47409.6 |
| render/graph/ms | 4.735 | 6.177 | 13.45 | 26791.5 |
| render/present/ms | 0.440 | 0.541 | 3.13 | 2491.0 |
| bigspace/total/ms | 0.427 | 0.657 | 2.17 | 2413.2 |
| terrain/update/ms | 0.283 | 0.361 | 1.86 | 1603.0 |
| controller/feed/ms | 0.261 | 0.368 | 0.73 | 1475.9 |
| terrain/select/ms | 0.250 | 0.318 | 0.90 | 1416.1 |
| stream/drive/ms | 0.235 | 0.394 | 4.19 | 1331.3 |
| far/select/ms | 0.219 | 0.379 | 0.76 | 1237.2 |
| stream/tick/ms | 0.190 | 0.294 | 0.82 | 1075.9 |
| anim/evaluate_cpu/ms | 0.166 | 0.216 | 0.65 | 938.7 |
| bigspace/low_precision/ms | 0.159 | 0.238 | 1.42 | 900.7 |
| controller/move/ms | 0.158 | 0.309 | 0.77 | 892.0 |
| anim/evaluate/ms | 0.152 | 0.309 | 1.73 | 859.4 |
| render/acquire/ms | 0.099 | 0.132 | 0.42 | 562.2 |
| bigspace/high_precision/ms | 0.097 | 0.148 | 1.72 | 549.6 |
| anim/apply/ms | 0.095 | 0.169 | 1.97 | 537.7 |
| physics/step/ms | 0.054 | 0.094 | 0.75 | 306.2 |
| physics/insert/ms | 0.036 | 0.370 | 1.01 | 204.7 |
| weather/map/ms | 0.025 | 0.034 | 0.17 | 143.9 |
| controller/avoid/ms | 0.012 | 0.018 | 0.15 | 67.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.19 | 59.8 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 39.8 |
| script/input/ms | 0.006 | 0.008 | 0.11 | 31.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.19 | 30.3 |
| script/ui/ms | 0.005 | 0.007 | 0.11 | 28.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 25.2 |
| controller/write/ms | 0.004 | 0.007 | 0.05 | 23.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 15.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.09 | 5.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.65 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
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
| frame/work/ms | ms | 9.30 | 22.39 | 8.08 |
| gpu/timed/ms | ms | 7.25 | 10.91 | 8.01 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 529 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 17 | 0 |
| process/cpu_usage | % | 24.04 | 35.71 | 15.38 |
| process/mem_usage | GiB | 2.61 | 2.67 | 2.67 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3484 | 3555 | 3555 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.26 | 72.46 | 30.99 |
| system/mem_usage | % | 46.13 | 48.36 | 46.25 |
| terrain/patches | count | 511 | 523 | 514 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.92 | 264 | 293.8 k |
| anim/events/count | count | 0.27 | 4 | 1546 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.72 | 160 | 4086 |
| assets/Mesh/removed | count | 0.86 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.83 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| ecs/entities_delta | count | 0.88 | 1328 | 4978 |
| physics/inserts | count | 0.49 | 116 | 2752 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 5983 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1312 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.3 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 568.7 | main/busy/ms 548, surface/advance_cpu_ms 67.64, surface/cell_upload_ms 0.87, stream/drive/ms 0.26, weather/map/ms 0.13, stream/tick/ms 0.07, bigspace/total/ms 0.05, bigspace/high_precision/ms 0.03, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.89 | 323.4 | main/busy/ms 584, render/thread/ms 281, render/busy/ms 234, surface/advance_cpu_ms 67.64, render/acquire/ms 45.40, render/graph/ms 7.67, render/present/ms 1.80, stream/drive/ms 1.20, surface/cell_upload_ms 0.87, stream/tick/ms 0.54, terrain/update/ms 0.44, terrain/select/ms 0.32 |
| warm-up | 0.93 | 41.1 | render/thread/ms 297, render/busy/ms 248, main/busy/ms 51.16, render/acquire/ms 45.53, terrain/update/ms 21.10, render/graph/ms 11.97, render/present/ms 2.89, stream/drive/ms 1.37, stream/tick/ms 0.82, terrain/select/ms 0.66, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.16 |
| warm-up | 1.17 | 83.4 | main/busy/ms 59.62, render/thread/ms 47.34, render/busy/ms 45.87, render/graph/ms 3.55, stream/drive/ms 2.14, stream/tick/ms 1.40, render/present/ms 1.18, terrain/update/ms 0.79, terrain/select/ms 0.65, render/acquire/ms 0.29, bigspace/total/ms 0.19, stream/spawn/ms 0.16 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 2%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **73%** (min 41%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.3 W · 73 °C (max 77) · VRAM 3901 MiB · throttled 100% of samples
