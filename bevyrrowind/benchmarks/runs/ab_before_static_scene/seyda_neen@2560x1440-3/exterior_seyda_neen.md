# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1491 frames over 15.0 s · **median 9.92 ms (101 fps)** · mean 10.06 · p95 11.62 · p99 13.01 (1% low 77 fps) · worst 20.74 ms · 7 spikes over 1.5× median · 24.0 stutters/min (work 20.0/min)
- warm-up: 249 frames over 3.6 s, worst 574.4 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.52 ms of a 10.06 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.92 ms, limit 11 ms
- pass worst frame 20.74 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.143 | 2.582 | 0.188 |
| main_opaque_pass_3d | 2.081 | 2.582 | 0.042 |
| early prepass | 0.685 | 1.028 | 0.079 |
| main_transparent_pass_3d | 0.483 | 0.652 | 0.119 |
| lightprobe_irradiance_map | 0.433 | 0.602 | 0.001 |
| atmosphere_luts | 0.366 | 0.703 | 0.003 |
| bloom | 0.328 | 0.330 | 0.071 |
| lightprobe_radiance_map | 0.225 | 0.236 | 0.005 |
| ssao | 0.224 | 0.357 | 0.007 |
| clouds_trace | 0.200 | 0.261 | 0.001 |
| uniform_allocation | 0.184 | 0.186 | 0.017 |
| water_fft | 0.135 | 0.129 | 0.003 |
| upscaling | 0.128 | 0.133 | 0.004 |
| bin_unpacking | 0.125 | 0.129 | 0.012 |
| water_velocity | 0.098 | 0.096 | 0.002 |
| atmos_froxel_inject | 0.092 | 0.096 | 0.002 |
| tonemapping | 0.089 | 0.091 | 0.004 |
| clustering | 0.073 | 0.079 | 0.022 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| auxiliary_views | 0.027 | 0.137 | 0.031 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.007 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.015 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.008 | 0.009 | 0.002 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.866 | 10.381 | 14.45 | 13219.4 |
| render/busy/ms | 8.032 | 9.383 | 13.35 | 11975.1 |
| main/busy/ms | 7.046 | 8.102 | 13.34 | 10505.3 |
| render/graph/ms | 4.029 | 5.017 | 6.94 | 6006.8 |
| render/present/ms | 0.512 | 1.264 | 2.13 | 763.1 |
| bigspace/total/ms | 0.366 | 0.657 | 1.85 | 545.2 |
| render/acquire/ms | 0.323 | 1.219 | 2.74 | 481.2 |
| terrain/update/ms | 0.253 | 0.321 | 0.82 | 377.2 |
| terrain/select/ms | 0.222 | 0.282 | 0.73 | 331.7 |
| stream/drive/ms | 0.201 | 0.348 | 0.67 | 300.2 |
| stream/tick/ms | 0.174 | 0.309 | 0.64 | 259.2 |
| controller/feed/ms | 0.170 | 0.246 | 0.61 | 253.2 |
| far/select/ms | 0.149 | 0.249 | 0.61 | 222.3 |
| anim/evaluate_cpu/ms | 0.131 | 0.184 | 0.50 | 195.1 |
| bigspace/low_precision/ms | 0.125 | 0.215 | 1.25 | 187.0 |
| anim/evaluate/ms | 0.103 | 0.253 | 1.46 | 153.5 |
| bigspace/high_precision/ms | 0.097 | 0.151 | 1.18 | 145.2 |
| controller/move/ms | 0.095 | 0.170 | 0.48 | 141.7 |
| anim/apply/ms | 0.065 | 0.114 | 1.09 | 97.4 |
| physics/step/ms | 0.047 | 0.092 | 1.00 | 70.2 |
| physics/insert/ms | 0.026 | 0.178 | 1.01 | 38.5 |
| weather/map/ms | 0.026 | 0.033 | 0.24 | 38.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 17.1 |
| controller/avoid/ms | 0.011 | 0.018 | 0.21 | 16.5 |
| script/update/ms | 0.007 | 0.010 | 0.20 | 10.8 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.4 |
| controller/write/ms | 0.003 | 0.006 | 0.04 | 5.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.08 | 1.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.5 |
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
| anim/a2/count | count | 2 | 2 | 2 |
| anim/a3/count | count | 380 | 382 | 380 |
| anim/characters/count | count | 382 | 382 | 382 |
| anim/evaluated/count | count | 20 | 55 | 20 |
| controller/movers/count | count | 22 | 22 | 22 |
| ecs/entities | count | 42.5 k | 42.5 k | 42.5 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.59 | 13.35 | 8.61 |
| gpu/timed/ms | ms | 8.49 | 10.72 | 8.61 |
| mesh_allocator_allocations |  meshes | 11.6 k | 11.6 k | 11.6 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 378.52 M | 378.52 M | 378.52 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 2143 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 46 | 0 |
| process/cpu_usage | % | 31.25 | 40.62 | 32.69 |
| process/mem_usage | GiB | 2.15 | 2.18 | 2.18 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2484 | 2485 | 2485 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 192 | 192 | 192 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 16.10 M | 16.10 M | 16.10 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 792 | 792 | 792 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.73 | 53.14 | 37.79 |
| system/mem_usage | % | 65.48 | 65.59 | 65.59 |
| terrain/patches | count | 441 | 441 | 441 |
| world/meshes | count | 23.0 k | 23.0 k | 23.0 k |
| world/point_lights | count | 394 | 394 | 394 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 10.52 | 34 | 15.7 k |
| anim/events/count | count | 0.05 | 2 | 80 |
| assets/Image/added | count | 0.10 | 30 | 148 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.31 | 112 | 469 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.10 | 30 | 150 |
| assets/StandardMaterial/modified | count | 1.51 | 5 | 2250 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.91 | 1713 | 5835 |
| physics/inserts | count | 0.81 | 136 | 1203 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 3 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 7 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 574.4 | main/busy/ms 554, surface/advance_cpu_ms 67.91, surface/cell_upload_ms 1.02, stream/drive/ms 0.52, stream/tick/ms 0.13, weather/map/ms 0.10, bigspace/total/ms 0.05, weather/eval/ms 0.04, bigspace/high_precision/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.92 | 348.2 | main/busy/ms 591, render/thread/ms 308, render/busy/ms 261, surface/advance_cpu_ms 67.91, render/acquire/ms 45.39, render/graph/ms 7.04, render/present/ms 1.93, surface/cell_upload_ms 1.02, stream/drive/ms 0.97, terrain/update/ms 0.45, stream/tick/ms 0.42, terrain/select/ms 0.29 |
| warm-up | 0.97 | 49.4 | render/thread/ms 327, render/busy/ms 278, main/busy/ms 53.16, render/acquire/ms 45.54, terrain/update/ms 23.26, render/graph/ms 11.73, render/present/ms 3.21, stream/drive/ms 0.77, terrain/select/ms 0.75, stream/tick/ms 0.54, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.12 |
| warm-up | 1.22 | 91.0 | main/busy/ms 78.35, render/thread/ms 59.55, render/busy/ms 57.75, terrain/update/ms 5.03, render/graph/ms 3.88, stream/drive/ms 1.85, render/present/ms 1.56, stream/tick/ms 1.28, terrain/select/ms 0.60, bigspace/total/ms 0.24, render/acquire/ms 0.24, bigspace/high_precision/ms 0.11 |
| warm-up | 1.97 | 31.4 | main/busy/ms 33.71, render/thread/ms 29.18, render/busy/ms 27.98, render/graph/ms 4.82, stream/drive/ms 3.50, stream/spawn/ms 2.08, render/present/ms 1.01, stream/tick/ms 0.80, bigspace/total/ms 0.52, terrain/update/ms 0.49, terrain/select/ms 0.41, bigspace/high_precision/ms 0.21 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_before\seyda_neen@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 87%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.4 W · 71 °C (max 74) · VRAM 4192 MiB · throttled 100% of samples
