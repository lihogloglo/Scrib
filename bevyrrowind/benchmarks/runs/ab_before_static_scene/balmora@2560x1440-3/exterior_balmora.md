# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1587 frames over 15.0 s · **median 9.35 ms (107 fps)** · mean 9.45 · p95 10.84 · p99 11.80 (1% low 85 fps) · worst 18.51 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 299 frames over 4.1 s, worst 570.1 ms
- hitches (> 25.0 ms): **0** measured, 8 in warm-up
- timed GPU passes: 8.28 ms of a 9.45 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.35 ms, limit 11 ms
- pass worst frame 18.51 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.422 | 2.710 | 0.044 |
| dlss_super_resolution | 2.078 | 2.221 | 0.185 |
| early prepass | 0.639 | 0.882 | 0.081 |
| lightprobe_irradiance_map | 0.434 | 0.563 | 0.001 |
| atmosphere_luts | 0.395 | 0.595 | 0.003 |
| bloom | 0.322 | 0.328 | 0.068 |
| lightprobe_radiance_map | 0.225 | 0.235 | 0.004 |
| ssao | 0.219 | 0.353 | 0.008 |
| main_transparent_pass_3d | 0.197 | 0.204 | 0.072 |
| uniform_allocation | 0.151 | 0.152 | 0.015 |
| clouds_trace | 0.136 | 0.161 | 0.001 |
| water_fft | 0.129 | 0.127 | 0.003 |
| upscaling | 0.114 | 0.122 | 0.004 |
| bin_unpacking | 0.108 | 0.113 | 0.011 |
| atmos_froxel_inject | 0.089 | 0.090 | 0.002 |
| tonemapping | 0.087 | 0.091 | 0.004 |
| water_velocity | 0.075 | 0.076 | 0.001 |
| clustering | 0.065 | 0.074 | 0.022 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.033 | 0.036 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.001 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.007 |
| lightprobe_copy | 0.021 | 0.018 | 0.005 |
| lightprobe_downsampling_first_pass | 0.014 | 0.015 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.240 | 9.568 | 17.46 | 13077.0 |
| render/busy/ms | 7.434 | 8.597 | 10.61 | 11798.0 |
| main/busy/ms | 6.682 | 7.570 | 9.76 | 10603.8 |
| render/graph/ms | 3.745 | 4.619 | 6.30 | 5943.5 |
| render/present/ms | 0.507 | 1.223 | 9.90 | 804.0 |
| bigspace/total/ms | 0.311 | 0.485 | 1.95 | 493.9 |
| render/acquire/ms | 0.299 | 1.101 | 2.18 | 475.0 |
| terrain/update/ms | 0.220 | 0.277 | 0.59 | 349.0 |
| stream/drive/ms | 0.220 | 0.383 | 0.86 | 349.0 |
| far/select/ms | 0.211 | 0.369 | 0.75 | 334.5 |
| terrain/select/ms | 0.190 | 0.240 | 0.56 | 301.6 |
| stream/tick/ms | 0.189 | 0.336 | 0.83 | 300.3 |
| controller/feed/ms | 0.142 | 0.203 | 0.53 | 225.1 |
| controller/move/ms | 0.111 | 0.188 | 0.44 | 176.9 |
| bigspace/high_precision/ms | 0.096 | 0.176 | 0.81 | 152.2 |
| anim/evaluate/ms | 0.083 | 0.204 | 1.11 | 131.3 |
| anim/apply/ms | 0.079 | 0.157 | 0.89 | 125.2 |
| bigspace/low_precision/ms | 0.073 | 0.118 | 1.10 | 116.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.073 | 0.39 | 96.0 |
| physics/step/ms | 0.032 | 0.064 | 0.50 | 50.1 |
| weather/map/ms | 0.026 | 0.033 | 0.30 | 41.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 18.1 |
| script/update/ms | 0.007 | 0.011 | 0.18 | 11.8 |
| controller/avoid/ms | 0.007 | 0.011 | 0.04 | 11.1 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 8.6 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.7 |
| physics/insert/ms | 0.005 | 0.000 | 1.03 | 7.3 |
| controller/write/ms | 0.002 | 0.003 | 0.04 | 3.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| anim/a2/count | count | 0 | 7 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 29 | 7 |
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
| frame/work/ms | ms | 8.31 | 10.61 | 8.21 |
| gpu/timed/ms | ms | 8.29 | 9.33 | 8.21 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 199 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 26 | 0 |
| process/cpu_usage | % | 29.47 | 45.05 | 24.20 |
| process/mem_usage | GiB | 2.03 | 2.09 | 2.09 |
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
| system/cpu_usage | % | 37.98 | 51.66 | 28.81 |
| system/mem_usage | % | 64.56 | 64.67 | 64.63 |
| terrain/patches | count | 370 | 370 | 370 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.19 | 165 | 297 |
| anim/events/count | count | 0.09 | 2 | 149 |
| assets/Image/added | count | 0.01 | 8 | 20 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.03 | 21 | 43 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 8 | 22 |
| assets/StandardMaterial/modified | count | 3.12 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.40 | 728 | 2222 |
| physics/inserts | count | 0.13 | 104 | 199 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 570.1 | main/busy/ms 550, surface/advance_cpu_ms 66.61, surface/cell_upload_ms 0.84, stream/drive/ms 0.26, weather/map/ms 0.09, bigspace/total/ms 0.06, stream/tick/ms 0.06, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.01, controller/feed/ms 0.01 |
| warm-up | 0.94 | 366.0 | main/busy/ms 580, render/thread/ms 330, render/busy/ms 262, surface/advance_cpu_ms 66.61, render/acquire/ms 65.67, render/graph/ms 5.76, render/present/ms 2.00, surface/cell_upload_ms 0.84, stream/drive/ms 0.76, stream/tick/ms 0.39, terrain/update/ms 0.35, terrain/select/ms 0.20 |
| warm-up | 0.99 | 51.2 | render/thread/ms 349, render/busy/ms 280, render/acquire/ms 66.10, main/busy/ms 45.06, terrain/update/ms 17.62, render/graph/ms 10.01, render/present/ms 3.15, stream/drive/ms 0.85, stream/tick/ms 0.62, terrain/select/ms 0.57, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.15 |
| warm-up | 1.27 | 133.0 | main/busy/ms 90.02, render/thread/ms 65.79, render/busy/ms 63.15, render/graph/ms 4.41, render/present/ms 1.44, stream/drive/ms 1.32, render/acquire/ms 1.21, stream/tick/ms 1.06, terrain/update/ms 0.67, terrain/select/ms 0.51, bigspace/total/ms 0.49, bigspace/high_precision/ms 0.25 |
| warm-up | 2.84 | 41.7 | main/busy/ms 64.62, render/thread/ms 30.94, render/busy/ms 29.00, render/graph/ms 11.01, render/present/ms 1.69, stream/drive/ms 1.26, physics/insert/ms 1.09, bigspace/total/ms 0.92, physics/step/ms 0.68, anim/evaluate_cpu/ms 0.62, terrain/update/ms 0.56, stream/tick/ms 0.49 |
| warm-up | 2.87 | 35.7 | main/busy/ms 47.25, render/thread/ms 38.12, render/busy/ms 36.45, render/graph/ms 9.38, stream/drive/ms 1.33, render/present/ms 1.29, physics/insert/ms 1.08, bigspace/total/ms 0.96, physics/step/ms 0.69, stream/tick/ms 0.59, terrain/update/ms 0.50, anim/evaluate_cpu/ms 0.44 |
| warm-up | 2.92 | 25.5 | render/thread/ms 41.88, render/busy/ms 39.71, main/busy/ms 29.97, render/graph/ms 17.38, render/present/ms 1.92, bigspace/total/ms 1.86, physics/insert/ms 1.46, stream/drive/ms 1.35, terrain/update/ms 0.65, physics/step/ms 0.65, stream/tick/ms 0.63, terrain/select/ms 0.51 |
| warm-up | 3.57 | 29.6 | main/busy/ms 43.83, render/thread/ms 25.86, render/busy/ms 24.23, render/graph/ms 9.49, render/present/ms 1.37, physics/insert/ms 1.07, stream/drive/ms 0.91, bigspace/total/ms 0.68, terrain/update/ms 0.52, anim/evaluate_cpu/ms 0.47, anim/evaluate/ms 0.45, physics/step/ms 0.44 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **96%** (min 69%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.5 W · 72 °C (max 75) · VRAM 4131 MiB · throttled 100% of samples
