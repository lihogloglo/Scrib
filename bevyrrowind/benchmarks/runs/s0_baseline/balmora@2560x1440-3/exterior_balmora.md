# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1537 frames over 15.0 s · **median 9.63 ms (104 fps)** · mean 9.76 · p95 11.03 · p99 12.15 (1% low 82 fps) · worst 61.35 ms · 3 spikes over 1.5× median · 16.0 stutters/min (work 8.0/min)
- warm-up: 438 frames over 4.7 s, worst 582.5 ms
- hitches (> 25.0 ms): **1** measured, 5 in warm-up
- timed GPU passes: 8.64 ms of a 9.76 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.63 ms, limit 11 ms
- **FAIL** worst frame 61.35 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.561 | 3.103 | 0.045 |
| dlss_super_resolution | 2.232 | 2.756 | 0.190 |
| early prepass | 0.633 | 0.966 | 0.083 |
| lightprobe_irradiance_map | 0.449 | 0.761 | 0.001 |
| atmosphere_luts | 0.361 | 0.641 | 0.003 |
| bloom | 0.342 | 0.488 | 0.066 |
| lightprobe_radiance_map | 0.231 | 0.236 | 0.004 |
| ssao | 0.217 | 0.216 | 0.007 |
| main_transparent_pass_3d | 0.207 | 0.201 | 0.070 |
| uniform_allocation | 0.161 | 0.155 | 0.016 |
| water_fft | 0.137 | 0.129 | 0.003 |
| clouds_trace | 0.137 | 0.163 | 0.001 |
| bin_unpacking | 0.113 | 0.114 | 0.011 |
| upscaling | 0.113 | 0.154 | 0.004 |
| atmos_froxel_inject | 0.095 | 0.095 | 0.002 |
| tonemapping | 0.093 | 0.098 | 0.003 |
| water_velocity | 0.079 | 0.077 | 0.002 |
| clustering | 0.069 | 0.081 | 0.022 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.005 |
| clouds_resolve | 0.036 | 0.043 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.032 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.030 | 0.007 |
| water_snapshot | 0.024 | 0.025 | 0.001 |
| water_mips | 0.022 | 0.024 | 0.002 |
| lightprobe_copy | 0.018 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.018 | 0.015 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| night_sky | 0.003 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.525 | 9.789 | 60.01 | 13102.9 |
| render/busy/ms | 7.430 | 8.724 | 59.41 | 11419.2 |
| main/busy/ms | 6.725 | 7.721 | 13.07 | 10336.4 |
| render/graph/ms | 3.921 | 4.987 | 55.87 | 6026.8 |
| render/acquire/ms | 0.620 | 1.882 | 3.31 | 952.5 |
| render/present/ms | 0.476 | 0.601 | 3.45 | 731.2 |
| bigspace/total/ms | 0.315 | 0.574 | 1.78 | 484.2 |
| terrain/update/ms | 0.233 | 0.293 | 0.98 | 357.7 |
| stream/drive/ms | 0.216 | 0.384 | 0.75 | 332.3 |
| terrain/select/ms | 0.200 | 0.250 | 0.94 | 307.1 |
| far/select/ms | 0.189 | 0.332 | 0.68 | 291.0 |
| stream/tick/ms | 0.187 | 0.341 | 0.71 | 287.8 |
| controller/move/ms | 0.142 | 0.242 | 0.87 | 218.2 |
| controller/feed/ms | 0.139 | 0.202 | 0.38 | 214.1 |
| bigspace/high_precision/ms | 0.106 | 0.225 | 1.57 | 162.5 |
| anim/evaluate/ms | 0.079 | 0.195 | 0.85 | 121.5 |
| anim/apply/ms | 0.078 | 0.176 | 0.81 | 119.2 |
| bigspace/low_precision/ms | 0.074 | 0.139 | 0.99 | 113.0 |
| anim/evaluate_cpu/ms | 0.063 | 0.081 | 0.35 | 97.3 |
| physics/step/ms | 0.031 | 0.063 | 0.73 | 48.2 |
| weather/map/ms | 0.026 | 0.036 | 0.12 | 40.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.31 | 17.5 |
| script/update/ms | 0.007 | 0.014 | 0.09 | 10.6 |
| controller/avoid/ms | 0.006 | 0.009 | 0.02 | 9.6 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 7.6 |
| script/ui/ms | 0.004 | 0.006 | 0.07 | 6.5 |
| physics/insert/ms | 0.003 | 0.000 | 0.56 | 4.3 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.5 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 24 | 7 |
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
| frame/work/ms | ms | 8.62 | 59.41 | 10.35 |
| gpu/timed/ms | ms | 8.59 | 10.51 | 10.35 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 484.73 M | 484.73 M | 484.73 M |
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
| phases/opaque3d/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d_prepass/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 13 |
| phases/shadow/batch_sets | count | 23 | 26 | 23 |
| phases/shadow/bins | count | 2036 | 2343 | 2036 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5353 | 5679 | 5353 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 208 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 154 | 162 | 154 |
| preprocess/bind_groups_created | count | 154 | 162 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 30.29 | 48.79 | 35.43 |
| process/mem_usage | GiB | 2.12 | 2.21 | 2.21 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2591 | 2593 | 2593 |
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
| system/cpu_usage | % | 37.62 | 49.67 | 39.87 |
| system/mem_usage | % | 62.48 | 62.75 | 62.75 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 71 | 67 |
| views/all/bins | count | 5218 | 5525 | 5218 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 63 | 67 | 63 |
| views/all/pipelines_distinct | count | 55 | 59 | 55 |
| views/all/work_items | count | 14.8 k | 15.2 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2036 | 2343 | 2036 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5353 | 5679 | 5353 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5353 | 5679 | 5353 |
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
| views/main/bins | count | 3182 | 3182 | 3182 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9294 | 9294 | 9294 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 9472 | 9472 | 9472 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 134 |
| assets/Image/added | count | 0.04 | 25 | 69 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.16 | 82 | 250 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.05 | 26 | 71 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.45 | 838 | 2222 |
| physics/inserts | count | 0.12 | 71 | 185 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 3 | 899 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 5 | 7 |
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

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 582.5 | main/busy/ms 562, surface/advance_cpu_ms 68.27, surface/cell_upload_ms 0.68, stream/drive/ms 0.31, weather/map/ms 0.15, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.89 | 306.4 | main/busy/ms 595, render/thread/ms 270, render/busy/ms 202, surface/advance_cpu_ms 68.27, render/acquire/ms 66.32, render/graph/ms 5.22, render/present/ms 2.12, stream/drive/ms 0.87, surface/cell_upload_ms 0.68, terrain/update/ms 0.53, stream/tick/ms 0.44, terrain/select/ms 0.35 |
| warm-up | 0.98 | 88.4 | render/thread/ms 286, render/busy/ms 216, render/acquire/ms 66.60, main/busy/ms 48.23, terrain/update/ms 19.72, render/graph/ms 9.79, render/present/ms 3.23, stream/drive/ms 0.90, terrain/select/ms 0.65, stream/tick/ms 0.64, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.12 |
| warm-up | 1.20 | 84.5 | main/busy/ms 63.86, render/thread/ms 54.98, render/busy/ms 53.09, render/graph/ms 4.88, stream/drive/ms 2.78, stream/tick/ms 1.84, render/present/ms 1.67, terrain/update/ms 0.64, stream/spawn/ms 0.55, terrain/select/ms 0.50, render/acquire/ms 0.23, bigspace/total/ms 0.22 |
| warm-up | 4.50 | 26.4 | main/busy/ms 36.87, render/thread/ms 17.10, render/busy/ms 15.99, render/graph/ms 5.22, render/present/ms 0.92, bigspace/total/ms 0.76, physics/insert/ms 0.69, terrain/update/ms 0.50, far/select/ms 0.46, terrain/select/ms 0.40, anim/evaluate/ms 0.38, anim/evaluate_cpu/ms 0.36 |
| measure | 4.22 | 61.3 | render/thread/ms 68.79, render/busy/ms 67.68, render/graph/ms 60.65, main/busy/ms 14.18, render/present/ms 0.95, stream/drive/ms 0.61, stream/tick/ms 0.52, bigspace/total/ms 0.49, terrain/update/ms 0.44, far/select/ms 0.40, terrain/select/ms 0.38, controller/feed/ms 0.23 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 11%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **94%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.3 W · 73 °C (max 75) · VRAM 4794 MiB · throttled 100% of samples
