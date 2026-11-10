# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1525 frames over 15.0 s · **median 9.61 ms (104 fps)** · mean 9.84 · p95 11.79 · p99 13.81 (1% low 72 fps) · worst 20.99 ms · 9 spikes over 1.5× median · 28.0 stutters/min (work 16.0/min)
- warm-up: 1116 frames over 11.3 s, worst 607.2 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.53 ms of a 9.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.61 ms, limit 11 ms
- pass worst frame 20.99 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.502 | 2.985 | 0.045 |
| dlss_super_resolution | 2.133 | 2.605 | 0.193 |
| early prepass | 0.618 | 0.961 | 0.084 |
| atmosphere_luts | 0.447 | 0.681 | 0.004 |
| lightprobe_irradiance_map | 0.437 | 0.481 | 0.001 |
| bloom | 0.331 | 0.342 | 0.072 |
| ssao | 0.247 | 0.534 | 0.008 |
| lightprobe_radiance_map | 0.224 | 0.234 | 0.004 |
| main_transparent_pass_3d | 0.199 | 0.205 | 0.074 |
| uniform_allocation | 0.153 | 0.152 | 0.017 |
| water_fft | 0.151 | 0.452 | 0.003 |
| clouds_trace | 0.137 | 0.162 | 0.001 |
| upscaling | 0.120 | 0.124 | 0.005 |
| bin_unpacking | 0.106 | 0.113 | 0.012 |
| atmos_froxel_inject | 0.091 | 0.092 | 0.002 |
| tonemapping | 0.089 | 0.093 | 0.004 |
| water_velocity | 0.077 | 0.076 | 0.002 |
| clustering | 0.067 | 0.074 | 0.024 |
| clouds_composite | 0.065 | 0.066 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.041 | 0.006 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.028 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.028 | 0.008 |
| water_snapshot | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.021 | 0.019 | 0.004 |
| lightprobe_downsampling_first_pass | 0.015 | 0.015 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.012 | 0.013 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.597 | 10.379 | 18.30 | 13109.9 |
| render/busy/ms | 7.705 | 9.445 | 17.59 | 11750.4 |
| main/busy/ms | 6.986 | 8.238 | 17.98 | 10653.8 |
| render/graph/ms | 3.870 | 5.041 | 12.35 | 5901.5 |
| render/present/ms | 0.526 | 1.158 | 3.03 | 802.1 |
| render/acquire/ms | 0.366 | 1.218 | 2.33 | 557.5 |
| bigspace/total/ms | 0.332 | 0.595 | 2.38 | 505.5 |
| stream/drive/ms | 0.238 | 0.414 | 0.76 | 362.3 |
| terrain/update/ms | 0.234 | 0.303 | 0.64 | 356.6 |
| far/select/ms | 0.207 | 0.356 | 0.75 | 315.4 |
| stream/tick/ms | 0.205 | 0.365 | 0.71 | 313.2 |
| terrain/select/ms | 0.202 | 0.261 | 0.61 | 308.6 |
| controller/feed/ms | 0.151 | 0.215 | 0.52 | 230.4 |
| controller/move/ms | 0.149 | 0.275 | 0.53 | 228.0 |
| bigspace/high_precision/ms | 0.107 | 0.210 | 1.21 | 163.9 |
| anim/evaluate/ms | 0.086 | 0.174 | 1.38 | 131.3 |
| anim/apply/ms | 0.080 | 0.142 | 0.93 | 122.6 |
| bigspace/low_precision/ms | 0.079 | 0.166 | 1.81 | 120.9 |
| anim/evaluate_cpu/ms | 0.050 | 0.075 | 0.30 | 76.1 |
| physics/step/ms | 0.033 | 0.069 | 0.74 | 50.5 |
| weather/map/ms | 0.028 | 0.037 | 0.14 | 43.0 |
| physics/insert/ms | 0.012 | 0.062 | 0.79 | 18.7 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 18.3 |
| script/update/ms | 0.008 | 0.014 | 0.04 | 11.8 |
| script/input/ms | 0.006 | 0.008 | 0.13 | 9.1 |
| controller/avoid/ms | 0.006 | 0.010 | 0.07 | 8.9 |
| script/ui/ms | 0.006 | 0.008 | 0.07 | 8.5 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 1.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
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
| frame/work/ms | ms | 8.51 | 17.98 | 8.65 |
| gpu/timed/ms | ms | 8.46 | 10.51 | 8.65 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 629 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 25 | 0 |
| process/cpu_usage | % | 31.42 | 46.92 | 30.05 |
| process/mem_usage | GiB | 2.22 | 2.36 | 2.36 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2589 | 2591 | 2591 |
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
| system/cpu_usage | % | 41.52 | 72.36 | 47.32 |
| system/mem_usage | % | 64.92 | 65.23 | 65.23 |
| terrain/patches | count | 370 | 370 | 370 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 141 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.25 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| physics/inserts | count | 0.42 | 101 | 645 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 901 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.61 | 607.2 | main/busy/ms 582, surface/advance_cpu_ms 68.01, surface/cell_upload_ms 1.28, stream/drive/ms 0.36, weather/map/ms 0.13, stream/tick/ms 0.09, bigspace/total/ms 0.07, weather/eval/ms 0.04, bigspace/high_precision/ms 0.04, script/update/ms 0.03, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 1.10 | 495.3 | main/busy/ms 620, render/thread/ms 459, render/busy/ms 409, surface/advance_cpu_ms 68.01, render/acquire/ms 47.45, render/graph/ms 8.80, render/present/ms 2.39, surface/cell_upload_ms 1.28, stream/drive/ms 0.94, terrain/update/ms 0.56, stream/tick/ms 0.49, terrain/select/ms 0.37 |
| warm-up | 1.42 | 130.1 | main/busy/ms 99.71, render/thread/ms 57.34, render/busy/ms 55.28, render/graph/ms 5.50, stream/drive/ms 2.41, stream/tick/ms 2.03, render/present/ms 1.80, terrain/update/ms 0.73, terrain/select/ms 0.59, render/acquire/ms 0.26, bigspace/total/ms 0.21, weather/map/ms 0.10 |
| warm-up | 1.54 | 25.7 | render/thread/ms 40.90, render/busy/ms 39.49, main/busy/ms 17.27, stream/drive/ms 4.28, render/graph/ms 3.78, render/present/ms 1.17, stream/tick/ms 1.12, terrain/update/ms 0.77, terrain/select/ms 0.56, bigspace/total/ms 0.31, render/acquire/ms 0.23, stream/spawn/ms 0.21 |
| warm-up | 11.07 | 26.7 | main/busy/ms 17.00, render/thread/ms 10.48, render/busy/ms 9.78, render/graph/ms 5.18, bigspace/total/ms 0.74, terrain/update/ms 0.69, terrain/select/ms 0.61, render/present/ms 0.58, stream/drive/ms 0.53, stream/tick/ms 0.46, far/select/ms 0.41, anim/evaluate/ms 0.39 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_before\balmora@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 83%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.1 W · 72 °C (max 75) · VRAM 4704 MiB · throttled 100% of samples
