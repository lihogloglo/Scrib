# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3672 frames over 39.8 s · **median 10.20 ms (98 fps)** · mean 10.83 · p95 16.05 · p99 19.21 (1% low 52 fps) · worst 31.73 ms · 223 spikes over 1.5× median · 42.2 stutters/min (work 45.3/min)
- warm-up: 422 frames over 4.9 s, worst 584.9 ms
- hitches (> 25.5 ms): **6** measured, 4 in warm-up
- timed GPU passes: 8.17 ms of a 10.83 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.20 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 0.66 min = 42.24/min, limit 1/min
- pass worst frame 31.73 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.216 | 2.584 | 0.180 |
| main_opaque_pass_3d | 1.841 | 2.632 | 0.044 |
| early prepass | 0.773 | 1.113 | 0.083 |
| lightprobe_irradiance_map | 0.438 | 0.754 | 0.001 |
| atmosphere_luts | 0.385 | 0.615 | 0.003 |
| bloom | 0.337 | 0.489 | 0.067 |
| lightprobe_radiance_map | 0.228 | 0.236 | 0.004 |
| ssao | 0.219 | 0.353 | 0.007 |
| uniform_allocation | 0.185 | 0.199 | 0.016 |
| clouds_trace | 0.157 | 0.204 | 0.001 |
| auxiliary_views | 0.135 | 1.003 | 0.116 |
| upscaling | 0.130 | 0.140 | 0.004 |
| water_fft | 0.127 | 0.128 | 0.002 |
| bin_unpacking | 0.116 | 0.137 | 0.012 |
| atmos_froxel_inject | 0.096 | 0.103 | 0.001 |
| tonemapping | 0.091 | 0.091 | 0.004 |
| water_velocity | 0.082 | 0.082 | 0.002 |
| clustering | 0.068 | 0.071 | 0.020 |
| main_transparent_pass_3d | 0.065 | 0.096 | 0.103 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.055 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.048 | 0.048 | 0.005 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.026 | 0.006 |
| water_mips | 0.023 | 0.024 | 0.002 |
| weather_map | 0.015 | 0.015 | 0.001 |
| lightprobe_copy | 0.014 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.223 | 13.993 | 30.07 | 33868.2 |
| render/busy/ms | 8.614 | 13.413 | 27.56 | 31632.0 |
| main/busy/ms | 7.455 | 9.537 | 19.96 | 27375.6 |
| render/graph/ms | 4.206 | 7.008 | 16.09 | 15445.6 |
| render/present/ms | 0.504 | 1.197 | 16.25 | 1850.6 |
| bigspace/total/ms | 0.361 | 0.560 | 2.96 | 1324.6 |
| terrain/update/ms | 0.281 | 0.351 | 1.87 | 1033.5 |
| stream/drive/ms | 0.254 | 0.449 | 2.21 | 933.0 |
| terrain/select/ms | 0.245 | 0.299 | 0.67 | 898.8 |
| stream/tick/ms | 0.203 | 0.352 | 0.91 | 744.0 |
| controller/feed/ms | 0.181 | 0.247 | 0.60 | 663.6 |
| far/select/ms | 0.165 | 0.282 | 0.65 | 604.2 |
| bigspace/low_precision/ms | 0.127 | 0.205 | 1.80 | 466.5 |
| anim/evaluate_cpu/ms | 0.127 | 0.171 | 0.58 | 465.6 |
| anim/evaluate/ms | 0.117 | 0.264 | 2.53 | 429.8 |
| render/acquire/ms | 0.105 | 0.140 | 1.64 | 385.7 |
| bigspace/high_precision/ms | 0.097 | 0.148 | 1.81 | 357.6 |
| controller/move/ms | 0.077 | 0.131 | 0.56 | 281.7 |
| anim/apply/ms | 0.068 | 0.098 | 3.37 | 248.4 |
| physics/step/ms | 0.044 | 0.084 | 0.82 | 160.2 |
| physics/insert/ms | 0.039 | 0.383 | 0.88 | 143.2 |
| weather/map/ms | 0.026 | 0.034 | 0.15 | 94.1 |
| weather/eval/ms | 0.011 | 0.016 | 0.15 | 41.0 |
| controller/avoid/ms | 0.009 | 0.015 | 0.07 | 34.6 |
| stream/spawn/ms | 0.007 | 0.000 | 1.19 | 25.8 |
| script/update/ms | 0.007 | 0.014 | 0.11 | 25.7 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 18.3 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 15.7 |
| controller/write/ms | 0.003 | 0.005 | 0.18 | 11.4 |
| stream/despawn/ms | 0.002 | 0.000 | 1.13 | 9.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 6.8 |
| stream/activate/ms | 0.002 | 0.000 | 0.51 | 6.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.21 | 4.2 |
| shapes/activate/ms | 0.001 | 0.003 | 0.02 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 1 |
| anim/a2/count | count | 1 | 5 | 1 |
| anim/a3/count | count | 426 | 490 | 489 |
| anim/characters/count | count | 426 | 492 | 491 |
| anim/evaluated/count | count | 15 | 30 | 14 |
| controller/movers/count | count | 16 | 19 | 15 |
| ecs/entities | count | 47.2 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.43 | 27.56 | 10.64 |
| gpu/timed/ms | ms | 8.13 | 12.46 | 8.45 |
| mesh_allocator_allocations |  meshes | 12.8 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 446.61 M | 446.61 M | 446.61 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 393 | 417 | 350 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 393 | 417 | 350 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/opaque3d/batch_sets | count | 14 | 16 | 15 |
| phases/opaque3d/bins | count | 1973 | 2109 | 1788 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 4268 | 4505 | 4063 |
| phases/opaque3d/pipelines | count | 13 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1973 | 2109 | 1788 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 4268 | 4505 | 4063 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 14 |
| phases/shadow/batch_sets | count | 30 | 277 | 55 |
| phases/shadow/bins | count | 2854 | 8261 | 6328 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8205 | 14.0 k | 11.7 k |
| phases/shadow/pipelines | count | 32 | 283 | 191 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2405 | 3288 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 858 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 168 | 180 | 180 |
| pipelines/waiting | count | 0 | 18 | 0 |
| preprocess/bind_groups | count | 166 | 718 | 225 |
| preprocess/bind_groups_created | count | 166 | 718 | 225 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.31 | 39.23 | 31.90 |
| process/mem_usage | GiB | 2.32 | 2.73 | 2.73 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2885 | 3316 | 3316 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 377 | 540 | 534 |
| statics/materials_bound | count | 377 | 540 | 534 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 1 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.12 | 0.05 |
| stream/model_queue | count | 0 | 72 | 0 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.29 | 0.08 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 2 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.86 | 50.98 | 30.23 |
| system/mem_usage | % | 63.25 | 64.54 | 64.54 |
| terrain/patches | count | 498 | 520 | 490 |
| views/all/batch_sets | count | 73 | 325 | 101 |
| views/all/bins | count | 7713 | 12.6 k | 10.6 k |
| views/all/count | count | 4 | 54 | 40 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.8 k | 24.3 k | 21.9 k |
| views/all/pipelines | count | 70 | 327 | 233 |
| views/all/pipelines_distinct | count | 55 | 70 | 67 |
| views/all/work_items | count | 19.0 k | 23.9 k | 18.8 k |
| views/cascade/batch_sets | count | 29 | 44 | 37 |
| views/cascade/bins | count | 2814 | 3379 | 2815 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8068 | 9049 | 8021 |
| views/cascade/pipelines | count | 29 | 44 | 37 |
| views/cascade/work_items | count | 8068 | 9049 | 8021 |
| views/lamp_face/batch_sets | count | 0 | 239 | 18 |
| views/lamp_face/bins | count | 0 | 5085 | 3513 |
| views/lamp_face/count | count | 0 | 48 | 36 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 5330 | 3727 |
| views/lamp_face/pipelines | count | 0 | 239 | 154 |
| views/lamp_face/work_items | count | 0 | 5294 | 372 |
| views/main/batch_sets | count | 44 | 48 | 46 |
| views/main/bins | count | 4732 | 5004 | 4276 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 10.6 k | 11.1 k | 10.2 k |
| views/main/pipelines | count | 40 | 44 | 42 |
| views/main/work_items | count | 10.9 k | 11.3 k | 10.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 26.5 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 24.64 | 247 | 90.5 k |
| anim/events/count | count | 0.14 | 3 | 531 |
| assets/Image/added | count | 0.09 | 31 | 322 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.63 | 300 | 6000 |
| assets/Mesh/removed | count | 0.66 | 718 | 2434 |
| assets/StandardMaterial/added | count | 0.14 | 33 | 504 |
| assets/StandardMaterial/modified | count | 2.15 | 10 | 7881 |
| assets/StandardMaterial/removed | count | 0.03 | 32 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 5.43 | 1416 | 19.9 k |
| physics/inserts | count | 0.79 | 122 | 2917 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 162 | 1315 |
| physics/steps | count | 0.65 | 2 | 2386 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 18 | 214 |
| stream/deactivate/items | count | 0.04 | 16 | 147 |
| stream/despawn/items | count | 0.03 | 11 | 95 |
| stream/fetch/items | count | 0.02 | 1 | 77 |
| stream/shift_guard/frames | count | 0.00 | 1 | 4 |
| stream/spawn/items | count | 0.11 | 18 | 394 |
| stream/upload/bytes | bytes | 10.7 k | 2.07 M | 39.18 M |
| stream/upload/items | count | 0.01 | 1 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 63 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2488 | 1.02 M | 9.14 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 25.5 ms; slowest 10, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 584.9 | main/busy/ms 563, surface/advance_cpu_ms 68.06, surface/cell_upload_ms 0.67, stream/drive/ms 0.31, weather/map/ms 0.13, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.02, far/select/ms 0.02 |
| warm-up | 0.93 | 347.5 | main/busy/ms 594, render/thread/ms 313, render/busy/ms 241, render/acquire/ms 70.02, surface/advance_cpu_ms 68.06, render/graph/ms 4.85, render/present/ms 2.19, stream/drive/ms 0.91, surface/cell_upload_ms 0.67, terrain/update/ms 0.58, stream/tick/ms 0.41, terrain/select/ms 0.40 |
| warm-up | 0.99 | 62.3 | render/thread/ms 333, render/busy/ms 258, render/acquire/ms 70.32, main/busy/ms 48.92, terrain/update/ms 17.92, render/graph/ms 9.97, render/present/ms 4.63, stream/drive/ms 1.05, terrain/select/ms 0.90, stream/tick/ms 0.68, bigspace/total/ms 0.29, bigspace/high_precision/ms 0.23 |
| warm-up | 1.23 | 100.1 | main/busy/ms 59.82, render/thread/ms 57.99, render/busy/ms 56.39, render/graph/ms 3.44, render/present/ms 1.26, stream/drive/ms 1.10, stream/tick/ms 0.90, terrain/update/ms 0.65, terrain/select/ms 0.54, render/acquire/ms 0.34, bigspace/total/ms 0.25, bigspace/high_precision/ms 0.09 |
| measure | 2.00 | 31.7 | render/thread/ms 38.97, render/busy/ms 22.11, render/present/ms 16.69, main/busy/ms 16.24, render/graph/ms 10.98, stream/drive/ms 1.06, stream/tick/ms 0.79, bigspace/total/ms 0.76, terrain/update/ms 0.58, controller/feed/ms 0.54, terrain/select/ms 0.52, far/select/ms 0.39 |
| measure | 35.34 | 28.9 | render/thread/ms 42.70, render/busy/ms 41.24, main/busy/ms 25.34, render/graph/ms 23.14, stream/drive/ms 2.53, stream/spawn/ms 1.92, render/present/ms 1.27, bigspace/total/ms 0.81, terrain/update/ms 0.54, stream/tick/ms 0.54, terrain/select/ms 0.48, controller/feed/ms 0.40 |
| measure | 35.37 | 30.6 | render/thread/ms 55.23, render/busy/ms 53.70, render/graph/ms 30.24, main/busy/ms 25.89, anim/apply/ms 3.43, stream/drive/ms 1.57, render/present/ms 1.33, stream/spawn/ms 0.88, bigspace/total/ms 0.83, terrain/update/ms 0.64, stream/tick/ms 0.61, terrain/select/ms 0.57 |
| measure | 35.78 | 25.8 | render/thread/ms 39.20, render/busy/ms 38.11, render/graph/ms 21.64, main/busy/ms 20.61, render/present/ms 0.92, physics/insert/ms 0.90, bigspace/total/ms 0.87, terrain/update/ms 0.54, terrain/select/ms 0.48, controller/feed/ms 0.44, stream/drive/ms 0.42, stream/tick/ms 0.37 |
| measure | 36.91 | 27.2 | render/thread/ms 39.35, render/busy/ms 38.23, main/busy/ms 18.95, render/graph/ms 17.00, physics/insert/ms 0.97, render/present/ms 0.94, bigspace/total/ms 0.76, terrain/update/ms 0.56, terrain/select/ms 0.49, far/select/ms 0.47, stream/drive/ms 0.41, controller/feed/ms 0.35 |
| measure | 37.39 | 28.0 | render/thread/ms 40.97, render/busy/ms 39.76, render/graph/ms 21.97, main/busy/ms 19.63, terrain/update/ms 1.16, render/present/ms 1.04, bigspace/total/ms 0.69, stream/drive/ms 0.63, physics/insert/ms 0.63, stream/tick/ms 0.50, terrain/select/ms 0.47, far/select/ms 0.43 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\run@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 11%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (155 samples): GPU load **86%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 84.1 W · 75 °C (max 78) · VRAM 4342 MiB · throttled 100% of samples
