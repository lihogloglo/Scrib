# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1648 frames over 15.0 s · **median 8.94 ms (112 fps)** · mean 9.11 · p95 10.67 · p99 11.97 (1% low 84 fps) · worst 14.96 ms · 3 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 335 frames over 4.3 s, worst 567.7 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.75 ms of a 9.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.94 ms, limit 11 ms
- pass worst frame 14.96 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.091 | 2.432 | 0.179 |
| main_opaque_pass_3d | 1.801 | 2.180 | 0.041 |
| early prepass | 0.727 | 0.965 | 0.075 |
| lightprobe_irradiance_map | 0.436 | 0.666 | 0.001 |
| atmosphere_luts | 0.371 | 0.588 | 0.003 |
| bloom | 0.318 | 0.317 | 0.065 |
| clouds_trace | 0.272 | 0.361 | 0.001 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.212 | 0.209 | 0.007 |
| uniform_allocation | 0.159 | 0.370 | 0.015 |
| water_fft | 0.128 | 0.127 | 0.003 |
| upscaling | 0.116 | 0.124 | 0.004 |
| bin_unpacking | 0.102 | 0.111 | 0.010 |
| tonemapping | 0.089 | 0.090 | 0.003 |
| atmos_froxel_inject | 0.087 | 0.087 | 0.001 |
| main_transparent_pass_3d | 0.082 | 0.090 | 0.067 |
| water_velocity | 0.075 | 0.072 | 0.001 |
| clustering | 0.066 | 0.073 | 0.021 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.051 | 0.051 | 0.002 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.005 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.001 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.007 |
| lightprobe_copy | 0.016 | 0.015 | 0.004 |
| night_sky | 0.014 | 0.014 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.001 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 7.792 | 9.243 | 12.33 | 12841.4 |
| render/busy/ms | 7.182 | 8.535 | 11.64 | 11836.2 |
| main/busy/ms | 6.554 | 7.508 | 11.55 | 10801.1 |
| render/graph/ms | 3.500 | 4.406 | 6.33 | 5767.2 |
| render/present/ms | 0.497 | 1.229 | 1.96 | 819.4 |
| bigspace/total/ms | 0.336 | 0.544 | 1.26 | 553.1 |
| far/select/ms | 0.242 | 0.405 | 0.79 | 399.4 |
| terrain/update/ms | 0.230 | 0.291 | 0.64 | 378.3 |
| stream/drive/ms | 0.212 | 0.371 | 0.77 | 350.1 |
| terrain/select/ms | 0.200 | 0.254 | 0.60 | 328.9 |
| stream/tick/ms | 0.184 | 0.328 | 0.74 | 303.4 |
| controller/feed/ms | 0.143 | 0.207 | 0.53 | 235.8 |
| render/acquire/ms | 0.113 | 0.201 | 1.44 | 185.8 |
| controller/move/ms | 0.105 | 0.187 | 0.42 | 173.7 |
| bigspace/low_precision/ms | 0.099 | 0.170 | 0.75 | 163.5 |
| bigspace/high_precision/ms | 0.092 | 0.142 | 0.99 | 151.7 |
| anim/evaluate/ms | 0.092 | 0.172 | 0.77 | 151.1 |
| anim/evaluate_cpu/ms | 0.083 | 0.105 | 0.27 | 137.4 |
| anim/apply/ms | 0.078 | 0.102 | 0.82 | 128.2 |
| physics/step/ms | 0.038 | 0.080 | 0.92 | 63.1 |
| weather/map/ms | 0.025 | 0.033 | 0.18 | 41.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 18.8 |
| controller/avoid/ms | 0.010 | 0.016 | 0.33 | 17.2 |
| script/update/ms | 0.007 | 0.010 | 0.05 | 11.2 |
| script/input/ms | 0.005 | 0.006 | 0.24 | 8.6 |
| script/ui/ms | 0.005 | 0.007 | 0.34 | 7.9 |
| controller/write/ms | 0.003 | 0.005 | 0.04 | 4.8 |
| physics/insert/ms | 0.002 | 0.000 | 0.67 | 3.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 1.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.6 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 1 | 0 |
| anim/a2/count | count | 0 | 1 | 0 |
| anim/a3/count | count | 502 | 503 | 503 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 40 | 15 |
| controller/movers/count | count | 15 | 15 | 15 |
| ecs/entities | count | 51.4 k | 51.4 k | 51.4 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.77 | 11.64 | 7.50 |
| gpu/timed/ms | ms | 7.72 | 8.95 | 7.50 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.19 M | 433.19 M | 433.19 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 423 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 157 | 157 | 157 |
| pipelines/waiting | count | 0 | 21 | 0 |
| process/cpu_usage | % | 24.76 | 32.69 | 25.48 |
| process/mem_usage | GiB | 1.95 | 1.98 | 1.98 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2653 | 2654 | 2654 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 214 | 214 | 214 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 23.03 M | 23.03 M | 23.03 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 1124 | 1124 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.23 | 53.79 | 35.08 |
| system/mem_usage | % | 44.18 | 44.34 | 44.28 |
| terrain/patches | count | 391 | 391 | 391 |
| world/meshes | count | 23.9 k | 23.9 k | 23.9 k |
| world/point_lights | count | 200 | 200 | 200 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 8.94 | 34 | 14.7 k |
| anim/events/count | count | 0.15 | 2 | 242 |
| assets/Image/added | count | 0.03 | 24 | 46 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.07 | 53 | 109 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 25 | 48 |
| assets/StandardMaterial/modified | count | 3.01 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.67 | 998 | 2758 |
| physics/inserts | count | 0.26 | 101 | 435 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 1 | 901 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 3 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 567.7 | main/busy/ms 547, surface/advance_cpu_ms 67.11, surface/cell_upload_ms 1.16, stream/drive/ms 0.26, weather/map/ms 0.12, bigspace/total/ms 0.08, stream/tick/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.02, script/ui/ms 0.01 |
| warm-up | 0.91 | 346.8 | main/busy/ms 580, render/thread/ms 309, render/busy/ms 241, surface/advance_cpu_ms 67.11, render/acquire/ms 65.34, render/graph/ms 8.94, render/present/ms 2.01, stream/drive/ms 1.43, surface/cell_upload_ms 1.16, stream/tick/ms 0.96, terrain/update/ms 0.58, terrain/select/ms 0.42 |
| warm-up | 0.95 | 36.5 | render/thread/ms 327, render/busy/ms 258, render/acquire/ms 65.59, main/busy/ms 49.14, terrain/update/ms 20.04, render/graph/ms 14.05, render/present/ms 3.20, stream/drive/ms 1.57, stream/tick/ms 1.23, terrain/select/ms 0.71, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.12 |
| warm-up | 1.19 | 88.0 | main/busy/ms 60.13, render/thread/ms 45.63, render/busy/ms 43.98, render/graph/ms 3.70, stream/drive/ms 1.55, render/present/ms 1.42, stream/tick/ms 1.18, terrain/update/ms 0.68, terrain/select/ms 0.55, bigspace/total/ms 0.32, render/acquire/ms 0.23, bigspace/low_precision/ms 0.12 |
| warm-up | 1.88 | 32.4 | main/busy/ms 35.22, render/thread/ms 29.70, render/busy/ms 28.40, render/graph/ms 4.39, stream/drive/ms 2.42, render/present/ms 1.10, stream/tick/ms 0.72, stream/spawn/ms 0.68, terrain/update/ms 0.57, bigspace/total/ms 0.49, terrain/select/ms 0.47, bigspace/high_precision/ms 0.23 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_before\red_mountain@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 18%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **94%** (min 85%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.4 W · 71 °C (max 74) · VRAM 3317 MiB · throttled 100% of samples
