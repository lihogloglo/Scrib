# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1471 frames over 15.0 s · **median 10.04 ms (100 fps)** · mean 10.20 · p95 11.98 · p99 13.65 (1% low 73 fps) · worst 28.93 ms · 6 spikes over 1.5× median · 16.0 stutters/min (work 12.0/min)
- warm-up: 396 frames over 5.3 s, worst 571.4 ms
- hitches (> 25.1 ms): **1** measured, 4 in warm-up
- timed GPU passes: 8.54 ms of a 10.20 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.04 ms, limit 11 ms
- pass worst frame 28.93 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.149 | 2.527 | 0.194 |
| main_opaque_pass_3d | 2.101 | 2.501 | 0.044 |
| early prepass | 0.708 | 0.963 | 0.083 |
| main_transparent_pass_3d | 0.479 | 0.643 | 0.124 |
| lightprobe_irradiance_map | 0.436 | 0.657 | 0.001 |
| atmosphere_luts | 0.341 | 0.516 | 0.003 |
| bloom | 0.333 | 0.333 | 0.071 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.214 | 0.212 | 0.007 |
| clouds_trace | 0.199 | 0.257 | 0.001 |
| uniform_allocation | 0.191 | 0.334 | 0.017 |
| water_fft | 0.130 | 0.129 | 0.003 |
| upscaling | 0.127 | 0.132 | 0.005 |
| bin_unpacking | 0.124 | 0.129 | 0.012 |
| water_velocity | 0.095 | 0.096 | 0.002 |
| atmos_froxel_inject | 0.093 | 0.094 | 0.001 |
| tonemapping | 0.089 | 0.092 | 0.004 |
| clustering | 0.072 | 0.077 | 0.022 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.005 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| auxiliary_views | 0.033 | 0.218 | 0.035 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| night_sky | 0.008 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.975 | 10.714 | 27.80 | 13202.3 |
| render/busy/ms | 8.236 | 9.719 | 27.17 | 12114.8 |
| main/busy/ms | 7.138 | 8.363 | 25.43 | 10500.7 |
| render/graph/ms | 4.199 | 5.222 | 23.62 | 6176.1 |
| render/present/ms | 0.545 | 1.352 | 2.22 | 801.3 |
| bigspace/total/ms | 0.356 | 0.545 | 1.73 | 523.0 |
| terrain/update/ms | 0.251 | 0.318 | 0.63 | 369.2 |
| terrain/select/ms | 0.221 | 0.278 | 0.60 | 324.5 |
| stream/drive/ms | 0.206 | 0.352 | 0.70 | 302.8 |
| render/acquire/ms | 0.195 | 0.837 | 2.32 | 286.1 |
| controller/feed/ms | 0.181 | 0.251 | 0.38 | 266.9 |
| stream/tick/ms | 0.178 | 0.316 | 0.67 | 261.7 |
| far/select/ms | 0.160 | 0.274 | 0.56 | 235.3 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.57 | 226.8 |
| bigspace/low_precision/ms | 0.124 | 0.203 | 1.18 | 182.9 |
| controller/move/ms | 0.119 | 0.188 | 0.63 | 175.8 |
| bigspace/high_precision/ms | 0.094 | 0.131 | 1.08 | 138.6 |
| anim/evaluate/ms | 0.094 | 0.166 | 1.18 | 137.7 |
| anim/apply/ms | 0.069 | 0.117 | 1.20 | 102.1 |
| physics/step/ms | 0.049 | 0.090 | 0.68 | 72.2 |
| weather/map/ms | 0.025 | 0.033 | 0.06 | 36.8 |
| weather/eval/ms | 0.012 | 0.015 | 0.28 | 17.4 |
| controller/avoid/ms | 0.012 | 0.018 | 0.12 | 17.2 |
| physics/insert/ms | 0.009 | 0.039 | 0.57 | 13.2 |
| script/update/ms | 0.007 | 0.011 | 0.13 | 10.9 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 7.9 |
| script/ui/ms | 0.005 | 0.007 | 0.14 | 7.4 |
| controller/write/ms | 0.003 | 0.005 | 0.14 | 5.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.05 | 1.6 |
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
| anim/a2/count | count | 2 | 2 | 2 |
| anim/a3/count | count | 380 | 380 | 380 |
| anim/characters/count | count | 382 | 382 | 382 |
| anim/evaluated/count | count | 20 | 30 | 20 |
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
| frame/work/ms | ms | 8.59 | 27.17 | 8.38 |
| gpu/timed/ms | ms | 8.46 | 11.02 | 8.38 |
| mesh_allocator_allocations |  meshes | 11.6 k | 11.6 k | 11.6 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 378.52 M | 378.52 M | 378.52 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 1759 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 18 | 0 |
| process/cpu_usage | % | 29.01 | 47.17 | 29.81 |
| process/mem_usage | GiB | 2.19 | 2.25 | 2.25 |
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
| system/cpu_usage | % | 37.91 | 73.64 | 39.04 |
| system/mem_usage | % | 64.95 | 65.11 | 65.10 |
| terrain/patches | count | 441 | 441 | 441 |
| world/meshes | count | 23.0 k | 23.0 k | 23.0 k |
| world/point_lights | count | 394 | 394 | 394 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 12.01 | 42 | 17.7 k |
| anim/events/count | count | 0.07 | 2 | 101 |
| assets/Image/added | count | 0.01 | 12 | 16 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.08 | 108 | 122 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 14 | 18 |
| assets/StandardMaterial/modified | count | 1.53 | 5 | 2250 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.22 | 1090 | 1794 |
| physics/inserts | count | 0.79 | 156 | 1158 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.61 | 2 | 901 |
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

## Hitches (> 25.1 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 571.4 | main/busy/ms 551, surface/advance_cpu_ms 67.31, surface/cell_upload_ms 1.13, stream/drive/ms 0.30, weather/map/ms 0.12, stream/tick/ms 0.08, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.02, weather/apply/ms 0.01 |
| warm-up | 0.90 | 331.7 | main/busy/ms 580, render/thread/ms 297, render/busy/ms 246, surface/advance_cpu_ms 67.31, render/acquire/ms 49.22, render/graph/ms 7.47, render/present/ms 2.21, surface/cell_upload_ms 1.13, stream/drive/ms 0.77, terrain/update/ms 0.50, stream/tick/ms 0.38, terrain/select/ms 0.37 |
| warm-up | 0.94 | 33.3 | render/thread/ms 314, render/busy/ms 262, render/acquire/ms 49.47, main/busy/ms 42.78, terrain/update/ms 17.12, render/graph/ms 11.57, render/present/ms 3.37, stream/drive/ms 0.78, terrain/select/ms 0.71, stream/tick/ms 0.54, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.14 |
| warm-up | 1.18 | 97.5 | render/thread/ms 65.79, render/busy/ms 64.26, main/busy/ms 60.23, render/graph/ms 3.15, stream/drive/ms 1.55, render/present/ms 1.30, stream/tick/ms 1.23, terrain/update/ms 0.68, terrain/select/ms 0.56, render/acquire/ms 0.24, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.11 |
| measure | 2.55 | 28.9 | render/thread/ms 37.57, render/busy/ms 35.36, main/busy/ms 31.80, render/graph/ms 28.03, render/present/ms 1.98, bigspace/total/ms 0.57, terrain/update/ms 0.47, stream/drive/ms 0.45, terrain/select/ms 0.42, controller/feed/ms 0.40, stream/tick/ms 0.39, anim/evaluate_cpu/ms 0.29 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_before\seyda_neen@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **94%** (min 77%) · 2580 MHz (min 2580) · memory 8001 MHz · 87.9 W · 73 °C (max 76) · VRAM 4128 MiB · throttled 100% of samples
