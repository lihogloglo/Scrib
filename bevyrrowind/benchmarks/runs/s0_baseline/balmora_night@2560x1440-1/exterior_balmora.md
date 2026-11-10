# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1716 frames over 15.0 s · **median 8.67 ms (115 fps)** · mean 8.74 · p95 9.88 · p99 10.60 (1% low 94 fps) · worst 24.87 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 475 frames over 5.2 s, worst 505.4 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 8.03 ms of a 8.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.67 ms, limit 11 ms
- pass worst frame 24.87 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.207 | 2.626 | 0.044 |
| dlss_super_resolution | 2.187 | 2.564 | 0.183 |
| early prepass | 0.600 | 0.938 | 0.080 |
| lightprobe_irradiance_map | 0.434 | 0.753 | 0.001 |
| atmosphere_luts | 0.364 | 0.660 | 0.003 |
| bloom | 0.341 | 0.495 | 0.066 |
| lightprobe_radiance_map | 0.237 | 0.243 | 0.004 |
| ssao | 0.222 | 0.353 | 0.007 |
| main_transparent_pass_3d | 0.182 | 0.178 | 0.071 |
| uniform_allocation | 0.132 | 0.134 | 0.015 |
| water_fft | 0.130 | 0.128 | 0.003 |
| upscaling | 0.130 | 0.151 | 0.004 |
| bin_unpacking | 0.098 | 0.102 | 0.009 |
| tonemapping | 0.098 | 0.104 | 0.004 |
| atmos_froxel_inject | 0.082 | 0.083 | 0.001 |
| water_velocity | 0.079 | 0.078 | 0.002 |
| clouds_trace | 0.069 | 0.090 | 0.001 |
| clustering | 0.068 | 0.074 | 0.021 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.005 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.032 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.016 | 0.016 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| clouds_fog_light | 0.013 | 0.014 | 0.002 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 7.635 | 8.702 | 23.59 | 13102.1 |
| render/busy/ms | 6.760 | 7.712 | 10.56 | 11599.6 |
| main/busy/ms | 6.437 | 7.157 | 8.54 | 11045.5 |
| render/graph/ms | 3.473 | 4.199 | 5.50 | 5959.4 |
| render/present/ms | 0.489 | 1.145 | 14.46 | 838.3 |
| render/acquire/ms | 0.387 | 1.271 | 2.35 | 664.1 |
| bigspace/total/ms | 0.311 | 0.579 | 1.70 | 533.2 |
| terrain/update/ms | 0.236 | 0.295 | 0.91 | 404.5 |
| stream/drive/ms | 0.217 | 0.364 | 0.73 | 372.1 |
| terrain/select/ms | 0.203 | 0.253 | 0.75 | 347.6 |
| far/select/ms | 0.191 | 0.321 | 0.74 | 327.7 |
| stream/tick/ms | 0.187 | 0.325 | 0.70 | 321.4 |
| controller/move/ms | 0.186 | 0.412 | 0.83 | 319.4 |
| controller/feed/ms | 0.138 | 0.191 | 0.53 | 236.2 |
| bigspace/high_precision/ms | 0.109 | 0.252 | 1.47 | 187.7 |
| bigspace/low_precision/ms | 0.075 | 0.141 | 0.95 | 128.1 |
| anim/apply/ms | 0.070 | 0.108 | 0.95 | 120.6 |
| anim/evaluate_cpu/ms | 0.067 | 0.081 | 0.41 | 114.2 |
| anim/evaluate/ms | 0.066 | 0.117 | 0.97 | 113.1 |
| physics/step/ms | 0.026 | 0.056 | 1.14 | 44.9 |
| weather/map/ms | 0.017 | 0.022 | 0.21 | 28.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.11 | 17.4 |
| script/update/ms | 0.008 | 0.015 | 0.34 | 12.9 |
| controller/avoid/ms | 0.006 | 0.009 | 0.18 | 10.7 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 8.3 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 7.9 |
| physics/insert/ms | 0.003 | 0.000 | 0.73 | 5.3 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 1.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| frame/work/ms | ms | 8.01 | 10.56 | 7.85 |
| gpu/timed/ms | ms | 8.01 | 9.01 | 7.85 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/opaque3d/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 988 | 988 | 988 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2009 | 2009 | 2009 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 265 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 139 | 139 | 139 |
| preprocess/bind_groups_created | count | 139 | 139 | 139 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.74 | 42.72 | 32.21 |
| process/mem_usage | GiB | 2.07 | 2.11 | 2.11 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2589 | 2591 | 2591 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 37.03 | 48.58 | 37.26 |
| system/mem_usage | % | 62.18 | 62.32 | 62.32 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 60 | 60 |
| views/all/bins | count | 4166 | 4166 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 56 | 56 |
| views/all/pipelines_distinct | count | 55 | 55 | 55 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 988 | 988 | 988 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2009 | 2009 | 2009 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 2009 | 2009 | 2009 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 44 | 44 |
| views/main/bins | count | 3178 | 3178 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 40 | 40 |
| views/main/work_items | count | 9490 | 9490 | 9490 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.08 | 3 | 134 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.88 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| physics/inserts | count | 0.14 | 79 | 245 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.52 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 505.4 | main/busy/ms 485, surface/advance_cpu_ms 0.71, stream/drive/ms 0.29, surface/cell_upload_ms 0.28, bigspace/total/ms 0.09, stream/tick/ms 0.08, weather/map/ms 0.05, bigspace/high_precision/ms 0.05, weather/eval/ms 0.02, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.02 |
| warm-up | 0.75 | 249.0 | main/busy/ms 522, render/thread/ms 213, render/busy/ms 148, render/acquire/ms 63.21, render/graph/ms 5.97, render/present/ms 2.12, stream/drive/ms 0.90, surface/advance_cpu_ms 0.71, terrain/update/ms 0.54, stream/tick/ms 0.45, terrain/select/ms 0.34, surface/cell_upload_ms 0.28 |
| warm-up | 0.89 | 132.7 | render/thread/ms 230, render/busy/ms 164, render/acquire/ms 63.36, main/busy/ms 53.78, terrain/update/ms 23.70, render/graph/ms 10.52, render/present/ms 3.34, stream/drive/ms 0.96, terrain/select/ms 0.73, stream/tick/ms 0.65, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.11 |
| warm-up | 1.11 | 92.3 | render/thread/ms 54.59, main/busy/ms 53.26, render/busy/ms 53.17, render/graph/ms 3.70, stream/drive/ms 1.29, render/present/ms 1.13, stream/tick/ms 1.04, terrain/update/ms 0.73, terrain/select/ms 0.57, render/acquire/ms 0.30, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.09 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 91%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.4 W · 74 °C (max 77) · VRAM 4525 MiB · throttled 100% of samples
