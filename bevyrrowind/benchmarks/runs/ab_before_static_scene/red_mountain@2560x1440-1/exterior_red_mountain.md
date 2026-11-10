# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1553 frames over 15.0 s · **median 9.34 ms (107 fps)** · mean 9.66 · p95 12.10 · p99 13.95 (1% low 72 fps) · worst 23.31 ms · 16 spikes over 1.5× median · 32.0 stutters/min (work 16.0/min)
- warm-up: 426 frames over 5.1 s, worst 590.4 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.97 ms of a 9.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.34 ms, limit 11 ms
- pass worst frame 23.31 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.144 | 2.683 | 0.186 |
| main_opaque_pass_3d | 1.866 | 2.468 | 0.044 |
| early prepass | 0.721 | 1.167 | 0.081 |
| lightprobe_irradiance_map | 0.451 | 0.756 | 0.001 |
| atmosphere_luts | 0.407 | 0.792 | 0.003 |
| bloom | 0.324 | 0.475 | 0.070 |
| clouds_trace | 0.269 | 0.353 | 0.001 |
| ssao | 0.241 | 0.549 | 0.008 |
| lightprobe_radiance_map | 0.228 | 0.233 | 0.004 |
| uniform_allocation | 0.165 | 0.310 | 0.018 |
| water_fft | 0.139 | 0.132 | 0.003 |
| upscaling | 0.115 | 0.119 | 0.005 |
| bin_unpacking | 0.104 | 0.112 | 0.012 |
| tonemapping | 0.091 | 0.091 | 0.004 |
| atmos_froxel_inject | 0.088 | 0.089 | 0.001 |
| main_transparent_pass_3d | 0.083 | 0.088 | 0.072 |
| water_velocity | 0.076 | 0.072 | 0.001 |
| clustering | 0.065 | 0.068 | 0.023 |
| clouds_composite | 0.064 | 0.065 | 0.002 |
| render_sky | 0.051 | 0.051 | 0.002 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.036 | 0.006 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.024 | 0.007 |
| atmos_froxel_integrate | 0.024 | 0.026 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| night_sky | 0.014 | 0.014 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| auxiliary_views | 0.001 | 0.002 | 0.004 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.334 | 10.604 | 21.99 | 12942.0 |
| render/busy/ms | 7.637 | 9.814 | 15.90 | 11860.1 |
| main/busy/ms | 6.907 | 8.474 | 16.04 | 10727.0 |
| render/graph/ms | 3.741 | 5.032 | 7.14 | 5809.0 |
| render/present/ms | 0.526 | 1.175 | 14.03 | 816.4 |
| bigspace/total/ms | 0.367 | 0.643 | 2.46 | 570.4 |
| far/select/ms | 0.244 | 0.416 | 0.67 | 379.4 |
| terrain/update/ms | 0.242 | 0.316 | 0.74 | 376.6 |
| stream/drive/ms | 0.222 | 0.399 | 0.79 | 344.6 |
| terrain/select/ms | 0.211 | 0.272 | 0.70 | 327.0 |
| stream/tick/ms | 0.193 | 0.355 | 0.77 | 299.6 |
| render/acquire/ms | 0.171 | 0.629 | 11.01 | 265.4 |
| controller/feed/ms | 0.168 | 0.258 | 0.68 | 261.4 |
| bigspace/low_precision/ms | 0.108 | 0.204 | 1.48 | 167.0 |
| bigspace/high_precision/ms | 0.107 | 0.206 | 1.46 | 166.1 |
| controller/move/ms | 0.101 | 0.168 | 0.53 | 156.2 |
| anim/evaluate/ms | 0.096 | 0.188 | 0.90 | 148.4 |
| anim/evaluate_cpu/ms | 0.086 | 0.117 | 0.42 | 134.0 |
| anim/apply/ms | 0.085 | 0.128 | 0.75 | 131.4 |
| physics/step/ms | 0.043 | 0.089 | 0.73 | 66.4 |
| weather/map/ms | 0.025 | 0.034 | 0.13 | 39.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.05 | 18.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.15 | 14.9 |
| script/update/ms | 0.008 | 0.012 | 0.24 | 12.1 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.4 |
| script/ui/ms | 0.005 | 0.008 | 0.08 | 8.3 |
| physics/insert/ms | 0.003 | 0.000 | 0.82 | 4.5 |
| controller/write/ms | 0.003 | 0.004 | 0.07 | 4.2 |
| shapes/activate/ms | 0.001 | 0.002 | 0.07 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.1 |
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
| anim/a2/count | count | 1 | 1 | 0 |
| anim/a3/count | count | 502 | 503 | 503 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 15 | 15 |
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
| frame/work/ms | ms | 8.11 | 20.50 | 8.52 |
| gpu/timed/ms | ms | 7.98 | 20.50 | 8.52 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.19 M | 433.19 M | 433.19 M |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 160 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 157 | 157 | 157 |
| pipelines/waiting | count | 0 | 15 | 0 |
| process/cpu_usage | % | 27.88 | 37.98 | 30.77 |
| process/mem_usage | GiB | 2.06 | 2.09 | 2.09 |
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
| system/cpu_usage | % | 39.96 | 82.04 | 49.00 |
| system/mem_usage | % | 65.28 | 66.19 | 65.55 |
| terrain/patches | count | 391 | 391 | 391 |
| world/meshes | count | 23.9 k | 23.9 k | 23.9 k |
| world/point_lights | count | 200 | 200 | 200 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 6.56 | 34 | 10.2 k |
| anim/events/count | count | 0.16 | 2 | 245 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| physics/inserts | count | 0.14 | 45 | 220 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 590.4 | main/busy/ms 569, surface/advance_cpu_ms 67.57, surface/cell_upload_ms 1.18, stream/drive/ms 0.29, weather/map/ms 0.13, stream/tick/ms 0.06, bigspace/total/ms 0.05, bigspace/high_precision/ms 0.03, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 0.90 | 312.2 | main/busy/ms 602, render/thread/ms 280, render/busy/ms 234, surface/advance_cpu_ms 67.57, render/acquire/ms 44.84, render/graph/ms 7.22, render/present/ms 1.80, surface/cell_upload_ms 1.18, stream/drive/ms 0.85, terrain/update/ms 0.58, stream/tick/ms 0.44, terrain/select/ms 0.41 |
| warm-up | 0.97 | 69.2 | render/thread/ms 298, render/busy/ms 250, main/busy/ms 48.31, render/acquire/ms 45.21, terrain/update/ms 19.70, render/graph/ms 11.37, render/present/ms 3.17, stream/drive/ms 1.01, terrain/select/ms 0.86, stream/tick/ms 0.74, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.14 |
| warm-up | 1.21 | 88.3 | main/busy/ms 65.97, render/thread/ms 53.98, render/busy/ms 52.35, render/graph/ms 3.85, stream/drive/ms 2.18, stream/tick/ms 1.46, render/present/ms 1.35, terrain/update/ms 0.73, terrain/select/ms 0.57, render/acquire/ms 0.27, stream/spawn/ms 0.25, bigspace/total/ms 0.22 |
| warm-up | 5.09 | 27.6 | main/busy/ms 36.09, render/thread/ms 30.54, render/busy/ms 29.29, render/graph/ms 13.73, bigspace/total/ms 1.61, render/present/ms 1.01, bigspace/high_precision/ms 0.94, stream/drive/ms 0.60, anim/evaluate_cpu/ms 0.58, anim/evaluate/ms 0.53, stream/tick/ms 0.52, terrain/update/ms 0.47 |

## Run

- `C:\cargo-target\scrib-ab\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_before\red_mountain@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **90%** (min 67%) · 2580 MHz (min 2580) · memory 8001 MHz · 86.4 W · 73 °C (max 76) · VRAM 4213 MiB · throttled 100% of samples
