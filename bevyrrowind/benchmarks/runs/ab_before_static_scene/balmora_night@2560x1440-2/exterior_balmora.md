# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1553 frames over 15.0 s · **median 9.49 ms (105 fps)** · mean 9.67 · p95 11.86 · p99 13.48 (1% low 74 fps) · worst 26.12 ms · 7 spikes over 1.5× median · 28.0 stutters/min (work 16.0/min)
- warm-up: 361 frames over 4.4 s, worst 526.3 ms
- hitches (> 25.0 ms): **1** measured, 6 in warm-up
- timed GPU passes: 8.49 ms of a 9.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.49 ms, limit 11 ms
- pass worst frame 26.12 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.511 | 3.084 | 0.052 |
| dlss_super_resolution | 2.208 | 2.752 | 0.202 |
| early prepass | 0.645 | 0.954 | 0.087 |
| lightprobe_irradiance_map | 0.435 | 0.605 | 0.001 |
| atmosphere_luts | 0.384 | 0.667 | 0.004 |
| bloom | 0.343 | 0.513 | 0.077 |
| ssao | 0.256 | 0.537 | 0.009 |
| lightprobe_radiance_map | 0.227 | 0.241 | 0.005 |
| main_transparent_pass_3d | 0.212 | 0.383 | 0.079 |
| uniform_allocation | 0.141 | 0.276 | 0.018 |
| water_fft | 0.137 | 0.137 | 0.003 |
| upscaling | 0.135 | 0.151 | 0.005 |
| bin_unpacking | 0.095 | 0.101 | 0.012 |
| tonemapping | 0.093 | 0.094 | 0.004 |
| atmos_froxel_inject | 0.082 | 0.083 | 0.002 |
| water_velocity | 0.078 | 0.077 | 0.002 |
| clouds_trace | 0.067 | 0.073 | 0.001 |
| clustering | 0.066 | 0.069 | 0.025 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.003 |
| clouds_resolve | 0.035 | 0.042 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.034 | 0.006 |
| atmos_froxel_integrate | 0.027 | 0.032 | 0.001 |
| water_mips | 0.026 | 0.024 | 0.002 |
| water_snapshot | 0.026 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.018 | 0.016 | 0.001 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.006 |
| lightprobe_copy | 0.015 | 0.016 | 0.005 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.004 |
| night_sky | 0.007 | 0.007 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| clouds_shadow | 0.004 | 0.004 | 0.004 |
| auxiliary_views | 0.001 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.449 | 10.468 | 21.61 | 13121.2 |
| render/busy/ms | 7.636 | 9.650 | 20.84 | 11858.5 |
| main/busy/ms | 7.188 | 8.795 | 20.31 | 11162.3 |
| render/graph/ms | 3.802 | 5.084 | 7.69 | 5904.1 |
| render/present/ms | 0.599 | 1.370 | 9.45 | 930.6 |
| bigspace/total/ms | 0.378 | 0.785 | 2.01 | 587.0 |
| stream/drive/ms | 0.256 | 0.418 | 0.86 | 398.1 |
| terrain/update/ms | 0.248 | 0.325 | 0.99 | 385.2 |
| stream/tick/ms | 0.221 | 0.373 | 0.68 | 342.5 |
| far/select/ms | 0.214 | 0.338 | 0.65 | 332.0 |
| render/acquire/ms | 0.214 | 0.914 | 4.08 | 332.0 |
| terrain/select/ms | 0.214 | 0.278 | 0.95 | 332.0 |
| controller/feed/ms | 0.163 | 0.238 | 0.84 | 253.1 |
| bigspace/high_precision/ms | 0.131 | 0.311 | 1.27 | 203.2 |
| controller/move/ms | 0.129 | 0.226 | 0.55 | 200.1 |
| bigspace/low_precision/ms | 0.093 | 0.201 | 1.65 | 144.4 |
| anim/evaluate/ms | 0.080 | 0.151 | 1.03 | 123.6 |
| anim/apply/ms | 0.079 | 0.137 | 0.73 | 122.0 |
| anim/evaluate_cpu/ms | 0.064 | 0.093 | 0.46 | 99.9 |
| physics/step/ms | 0.040 | 0.086 | 0.94 | 62.2 |
| physics/insert/ms | 0.025 | 0.217 | 0.91 | 39.0 |
| weather/map/ms | 0.019 | 0.026 | 0.31 | 30.0 |
| weather/eval/ms | 0.012 | 0.016 | 0.29 | 18.4 |
| script/update/ms | 0.009 | 0.017 | 0.26 | 14.5 |
| controller/avoid/ms | 0.006 | 0.009 | 0.04 | 9.6 |
| script/input/ms | 0.006 | 0.008 | 0.10 | 9.4 |
| script/ui/ms | 0.006 | 0.008 | 0.15 | 9.2 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 2.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.04 | 1.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.7 |
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
| anim/a2/count | count | 0 | 17 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 66 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.45 | 20.84 | 9.47 |
| gpu/timed/ms | ms | 8.36 | 11.33 | 8.40 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1297 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 25 | 0 |
| process/cpu_usage | % | 27.78 | 40.87 | 32.69 |
| process/mem_usage | GiB | 1.98 | 2.06 | 2.06 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2585 | 2587 | 2587 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 52.00 | 77.95 | 51.46 |
| system/mem_usage | % | 47.04 | 47.91 | 47.45 |
| terrain/patches | count | 370 | 370 | 370 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.68 | 396 | 1056 |
| anim/events/count | count | 0.11 | 2 | 164 |
| assets/Image/added | count | 0.04 | 26 | 69 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.16 | 65 | 256 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.05 | 27 | 83 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 5.38 | 3085 | 8355 |
| physics/inserts | count | 0.84 | 160 | 1302 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 901 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.53 | 526.3 | main/busy/ms 505, surface/advance_cpu_ms 0.76, surface/cell_upload_ms 0.38, stream/drive/ms 0.30, stream/tick/ms 0.07, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, weather/eval/ms 0.02, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 0.91 | 385.2 | main/busy/ms 538, render/thread/ms 345, render/busy/ms 270, render/acquire/ms 73.20, render/graph/ms 5.73, render/present/ms 2.33, stream/drive/ms 0.84, surface/advance_cpu_ms 0.76, terrain/update/ms 0.45, stream/tick/ms 0.42, surface/cell_upload_ms 0.38, terrain/select/ms 0.32 |
| warm-up | 0.96 | 51.7 | render/thread/ms 363, render/busy/ms 286, render/acquire/ms 73.48, main/busy/ms 48.97, terrain/update/ms 19.00, render/graph/ms 10.81, render/present/ms 3.82, stream/drive/ms 0.90, stream/tick/ms 0.64, terrain/select/ms 0.63, bigspace/total/ms 0.27, bigspace/high_precision/ms 0.14 |
| warm-up | 1.22 | 109.3 | main/busy/ms 85.55, render/thread/ms 52.52, render/busy/ms 50.86, terrain/update/ms 5.36, render/graph/ms 4.44, stream/drive/ms 1.65, render/present/ms 1.32, stream/tick/ms 1.31, terrain/select/ms 0.49, bigspace/total/ms 0.33, render/acquire/ms 0.33, bigspace/high_precision/ms 0.20 |
| warm-up | 1.35 | 27.7 | render/thread/ms 36.38, render/busy/ms 35.17, main/busy/ms 14.77, render/graph/ms 3.26, stream/drive/ms 3.14, render/present/ms 0.99, stream/spawn/ms 0.79, stream/tick/ms 0.72, terrain/update/ms 0.53, terrain/select/ms 0.42, bigspace/total/ms 0.26, render/acquire/ms 0.21 |
| warm-up | 2.71 | 25.4 | main/busy/ms 33.39, render/thread/ms 20.11, render/busy/ms 18.69, render/graph/ms 5.26, stream/drive/ms 1.19, render/present/ms 1.16, bigspace/total/ms 0.75, terrain/update/ms 0.52, stream/tick/ms 0.50, terrain/select/ms 0.44, stream/spawn/ms 0.38, bigspace/low_precision/ms 0.31 |
| measure | 3.09 | 26.1 | main/busy/ms 34.45, render/thread/ms 33.15, render/busy/ms 31.58, render/graph/ms 13.34, bigspace/total/ms 2.39, bigspace/low_precision/ms 1.73, render/present/ms 1.20, terrain/update/ms 1.15, controller/feed/ms 1.07, stream/drive/ms 0.96, stream/tick/ms 0.88, far/select/ms 0.84 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 24% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **94%** (min 71%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.1 W · 74 °C (max 78) · VRAM 3757 MiB · throttled 100% of samples
