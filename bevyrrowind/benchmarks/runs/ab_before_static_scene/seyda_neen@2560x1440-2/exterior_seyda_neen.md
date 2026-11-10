# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1500 frames over 15.0 s · **median 9.89 ms (101 fps)** · mean 10.00 · p95 11.46 · p99 12.71 (1% low 79 fps) · worst 17.46 ms · 3 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 475 frames over 5.7 s, worst 576.2 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 8.49 ms of a 10.00 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.89 ms, limit 11 ms
- pass worst frame 17.46 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.128 | 2.471 | 0.186 |
| main_opaque_pass_3d | 2.081 | 2.529 | 0.045 |
| early prepass | 0.696 | 1.023 | 0.084 |
| main_transparent_pass_3d | 0.476 | 0.640 | 0.125 |
| lightprobe_irradiance_map | 0.433 | 0.616 | 0.001 |
| atmosphere_luts | 0.367 | 0.633 | 0.003 |
| bloom | 0.328 | 0.330 | 0.068 |
| lightprobe_radiance_map | 0.225 | 0.233 | 0.004 |
| ssao | 0.219 | 0.354 | 0.007 |
| clouds_trace | 0.201 | 0.280 | 0.001 |
| uniform_allocation | 0.182 | 0.185 | 0.017 |
| water_fft | 0.132 | 0.131 | 0.003 |
| upscaling | 0.127 | 0.131 | 0.004 |
| bin_unpacking | 0.122 | 0.129 | 0.012 |
| water_velocity | 0.094 | 0.096 | 0.002 |
| atmos_froxel_inject | 0.092 | 0.098 | 0.002 |
| tonemapping | 0.089 | 0.091 | 0.003 |
| clustering | 0.073 | 0.078 | 0.021 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| auxiliary_views | 0.027 | 0.197 | 0.030 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.025 | 0.007 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.008 | 0.009 | 0.002 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.829 | 10.346 | 14.88 | 13244.1 |
| render/busy/ms | 8.059 | 9.245 | 14.33 | 12088.3 |
| main/busy/ms | 7.081 | 8.031 | 11.94 | 10621.2 |
| render/graph/ms | 4.089 | 5.003 | 6.77 | 6133.9 |
| render/present/ms | 0.514 | 1.307 | 4.87 | 770.3 |
| bigspace/total/ms | 0.357 | 0.601 | 1.83 | 536.2 |
| render/acquire/ms | 0.257 | 1.003 | 2.39 | 385.5 |
| terrain/update/ms | 0.251 | 0.318 | 0.69 | 376.4 |
| terrain/select/ms | 0.220 | 0.280 | 0.65 | 330.7 |
| stream/drive/ms | 0.202 | 0.346 | 0.95 | 302.8 |
| controller/feed/ms | 0.176 | 0.241 | 0.62 | 264.5 |
| stream/tick/ms | 0.175 | 0.308 | 0.91 | 262.2 |
| far/select/ms | 0.157 | 0.272 | 0.59 | 236.1 |
| anim/evaluate_cpu/ms | 0.140 | 0.178 | 0.53 | 209.5 |
| controller/move/ms | 0.130 | 0.218 | 0.48 | 195.4 |
| bigspace/low_precision/ms | 0.123 | 0.198 | 1.07 | 185.0 |
| anim/evaluate/ms | 0.105 | 0.238 | 0.92 | 157.1 |
| bigspace/high_precision/ms | 0.097 | 0.152 | 1.57 | 145.8 |
| anim/apply/ms | 0.066 | 0.105 | 1.02 | 98.7 |
| physics/step/ms | 0.045 | 0.085 | 0.82 | 67.6 |
| weather/map/ms | 0.025 | 0.032 | 0.37 | 37.6 |
| controller/avoid/ms | 0.013 | 0.018 | 0.31 | 18.8 |
| weather/eval/ms | 0.012 | 0.016 | 0.07 | 17.7 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 10.3 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 8.0 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 6.9 |
| controller/write/ms | 0.004 | 0.006 | 0.39 | 6.1 |
| physics/insert/ms | 0.002 | 0.000 | 0.45 | 3.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.4 |
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
| anim/a3/count | count | 380 | 380 | 380 |
| anim/characters/count | count | 382 | 382 | 382 |
| anim/evaluated/count | count | 21 | 28 | 20 |
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
| frame/work/ms | ms | 8.54 | 14.33 | 10.72 |
| gpu/timed/ms | ms | 8.47 | 10.06 | 8.21 |
| mesh_allocator_allocations |  meshes | 11.6 k | 11.6 k | 11.6 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 378.52 M | 378.52 M | 378.52 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 260 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 164 | 164 | 164 |
| pipelines/waiting | count | 0 | 5 | 0 |
| process/cpu_usage | % | 27.66 | 45.67 | 30.77 |
| process/mem_usage | GiB | 2.13 | 2.18 | 2.18 |
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
| system/cpu_usage | % | 37.12 | 58.19 | 38.96 |
| system/mem_usage | % | 64.79 | 65.03 | 65.03 |
| terrain/patches | count | 441 | 441 | 441 |
| world/meshes | count | 23.0 k | 23.0 k | 23.0 k |
| world/point_lights | count | 394 | 394 | 394 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 12.51 | 34 | 18.8 k |
| anim/events/count | count | 0.05 | 1 | 69 |
| assets/Image/added | count | 0.01 | 14 | 18 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.05 | 60 | 74 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 14 | 18 |
| assets/StandardMaterial/modified | count | 1.50 | 5 | 2250 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.60 | 799 | 894 |
| physics/inserts | count | 0.20 | 116 | 305 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 576.2 | main/busy/ms 555, surface/advance_cpu_ms 68.28, surface/cell_upload_ms 0.70, stream/drive/ms 0.32, weather/map/ms 0.12, stream/tick/ms 0.08, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, script/update/ms 0.02, weather/eval/ms 0.02, script/input/ms 0.02, controller/feed/ms 0.01 |
| warm-up | 0.94 | 360.9 | main/busy/ms 591, render/thread/ms 329, render/busy/ms 263, surface/advance_cpu_ms 68.28, render/acquire/ms 64.05, render/graph/ms 5.33, render/present/ms 2.08, stream/drive/ms 0.75, surface/cell_upload_ms 0.70, terrain/update/ms 0.59, terrain/select/ms 0.43, stream/tick/ms 0.36 |
| warm-up | 1.19 | 86.0 | main/busy/ms 62.41, render/thread/ms 48.49, render/busy/ms 46.90, render/graph/ms 4.36, stream/drive/ms 1.71, stream/tick/ms 1.46, render/present/ms 1.36, terrain/update/ms 0.93, terrain/select/ms 0.77, render/acquire/ms 0.24, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.10 |
| warm-up | 1.99 | 25.5 | render/thread/ms 37.62, render/busy/ms 36.48, main/busy/ms 18.47, render/graph/ms 3.70, stream/drive/ms 2.51, stream/spawn/ms 1.30, render/present/ms 0.95, stream/tick/ms 0.63, terrain/update/ms 0.59, terrain/select/ms 0.49, bigspace/total/ms 0.48, bigspace/high_precision/ms 0.21 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_before\seyda_neen@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 14%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 78%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.3 W · 74 °C (max 77) · VRAM 4126 MiB · throttled 100% of samples
