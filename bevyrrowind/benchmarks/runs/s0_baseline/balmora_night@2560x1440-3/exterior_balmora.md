# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1709 frames over 15.0 s · **median 8.68 ms (115 fps)** · mean 8.78 · p95 10.07 · p99 11.13 (1% low 90 fps) · worst 23.55 ms · 7 spikes over 1.5× median · 28.0 stutters/min (work 12.0/min)
- warm-up: 297 frames over 3.9 s, worst 505.0 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.04 ms of a 8.78 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.68 ms, limit 11 ms
- pass worst frame 23.55 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.201 | 2.631 | 0.043 |
| dlss_super_resolution | 2.184 | 2.556 | 0.185 |
| early prepass | 0.618 | 0.961 | 0.077 |
| lightprobe_irradiance_map | 0.454 | 0.762 | 0.001 |
| atmosphere_luts | 0.363 | 0.658 | 0.003 |
| bloom | 0.343 | 0.503 | 0.065 |
| lightprobe_radiance_map | 0.232 | 0.238 | 0.004 |
| ssao | 0.222 | 0.350 | 0.007 |
| main_transparent_pass_3d | 0.186 | 0.210 | 0.068 |
| uniform_allocation | 0.135 | 0.135 | 0.014 |
| water_fft | 0.131 | 0.129 | 0.002 |
| upscaling | 0.119 | 0.139 | 0.004 |
| bin_unpacking | 0.101 | 0.103 | 0.008 |
| tonemapping | 0.092 | 0.097 | 0.003 |
| atmos_froxel_inject | 0.081 | 0.083 | 0.001 |
| water_velocity | 0.079 | 0.077 | 0.001 |
| clustering | 0.069 | 0.077 | 0.022 |
| clouds_trace | 0.068 | 0.088 | 0.001 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.005 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.032 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.016 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.016 | 0.015 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.019 | 0.005 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.014 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 7.674 | 8.869 | 22.52 | 13114.5 |
| render/busy/ms | 6.779 | 7.913 | 21.96 | 11584.6 |
| main/busy/ms | 6.501 | 7.310 | 20.05 | 11109.6 |
| render/graph/ms | 3.533 | 4.496 | 18.90 | 6038.0 |
| render/present/ms | 0.479 | 0.650 | 8.23 | 818.1 |
| render/acquire/ms | 0.416 | 1.343 | 2.44 | 711.8 |
| bigspace/total/ms | 0.315 | 0.579 | 1.68 | 538.2 |
| terrain/update/ms | 0.234 | 0.295 | 0.80 | 399.3 |
| stream/drive/ms | 0.220 | 0.376 | 0.85 | 375.7 |
| terrain/select/ms | 0.200 | 0.253 | 0.76 | 342.6 |
| stream/tick/ms | 0.191 | 0.342 | 0.83 | 327.2 |
| far/select/ms | 0.188 | 0.306 | 0.78 | 321.7 |
| controller/move/ms | 0.145 | 0.271 | 0.62 | 248.0 |
| controller/feed/ms | 0.124 | 0.180 | 0.47 | 212.4 |
| bigspace/high_precision/ms | 0.104 | 0.218 | 1.42 | 177.3 |
| bigspace/low_precision/ms | 0.080 | 0.149 | 1.16 | 137.0 |
| anim/apply/ms | 0.071 | 0.108 | 0.85 | 120.8 |
| anim/evaluate/ms | 0.069 | 0.139 | 1.15 | 118.2 |
| anim/evaluate_cpu/ms | 0.064 | 0.080 | 0.42 | 109.4 |
| physics/step/ms | 0.029 | 0.057 | 1.05 | 49.8 |
| physics/insert/ms | 0.017 | 0.128 | 0.63 | 29.3 |
| weather/map/ms | 0.016 | 0.022 | 0.05 | 27.8 |
| weather/eval/ms | 0.010 | 0.013 | 0.22 | 17.0 |
| script/update/ms | 0.007 | 0.015 | 0.05 | 12.1 |
| controller/avoid/ms | 0.006 | 0.009 | 0.05 | 10.1 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 8.4 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 7.5 |
| controller/write/ms | 0.002 | 0.003 | 0.03 | 3.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 32 | 7 |
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
| frame/work/ms | ms | 8.05 | 21.96 | 7.86 |
| gpu/timed/ms | ms | 8.03 | 8.95 | 7.86 |
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
| phases/opaque3d/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 13 |
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
| physics/pending | count | 0 | 1729 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 139 | 147 | 139 |
| preprocess/bind_groups_created | count | 139 | 147 | 139 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.19 | 39.58 | 25.96 |
| process/mem_usage | GiB | 2.13 | 2.22 | 2.22 |
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
| system/cpu_usage | % | 37.27 | 49.77 | 32.44 |
| system/mem_usage | % | 62.41 | 62.68 | 62.68 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 64 | 60 |
| views/all/bins | count | 4166 | 4166 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 60 | 56 |
| views/all/pipelines_distinct | count | 55 | 59 | 55 |
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
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 3178 | 3178 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 44 | 40 |
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
| anim/bones_written/count | count | 0.19 | 297 | 330 |
| anim/events/count | count | 0.08 | 3 | 138 |
| assets/Image/added | count | 0.06 | 29 | 106 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.18 | 76 | 315 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.07 | 34 | 112 |
| assets/StandardMaterial/modified | count | 2.90 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.01 | 1403 | 5136 |
| physics/inserts | count | 0.84 | 156 | 1432 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.53 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 8 |
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

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 505.0 | main/busy/ms 485, surface/advance_cpu_ms 0.74, stream/drive/ms 0.28, surface/cell_upload_ms 0.27, bigspace/total/ms 0.07, stream/tick/ms 0.06, weather/map/ms 0.05, bigspace/high_precision/ms 0.04, script/update/ms 0.02, weather/eval/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.81 | 307.1 | main/busy/ms 516, render/thread/ms 271, render/busy/ms 188, render/acquire/ms 80.44, render/graph/ms 4.45, render/present/ms 2.14, stream/drive/ms 1.03, surface/advance_cpu_ms 0.74, terrain/update/ms 0.55, stream/tick/ms 0.54, terrain/select/ms 0.36, surface/cell_upload_ms 0.27 |
| warm-up | 1.07 | 262.3 | render/thread/ms 288, render/busy/ms 203, render/acquire/ms 80.76, main/busy/ms 47.72, terrain/update/ms 18.16, render/graph/ms 9.59, render/present/ms 4.67, stream/drive/ms 1.10, stream/tick/ms 0.76, terrain/select/ms 0.66, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.16 |
| warm-up | 1.36 | 78.2 | main/busy/ms 63.91, render/thread/ms 42.29, render/busy/ms 40.93, render/graph/ms 3.61, stream/drive/ms 1.75, stream/tick/ms 1.15, render/present/ms 1.13, terrain/update/ms 0.78, terrain/select/ms 0.55, bigspace/total/ms 0.25, stream/spawn/ms 0.24, render/acquire/ms 0.23 |
| warm-up | 2.53 | 29.7 | main/busy/ms 41.32, render/thread/ms 29.50, render/busy/ms 28.39, render/graph/ms 6.65, stream/drive/ms 1.33, bigspace/total/ms 0.92, render/present/ms 0.90, stream/tick/ms 0.57, anim/evaluate_cpu/ms 0.51, terrain/update/ms 0.48, bigspace/low_precision/ms 0.45, stream/spawn/ms 0.41 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **96%** (min 83%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.9 W · 73 °C (max 76) · VRAM 4527 MiB · throttled 100% of samples
