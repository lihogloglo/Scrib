# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3748 frames over 39.8 s · **median 10.17 ms (98 fps)** · mean 10.61 · p95 13.95 · p99 17.45 (1% low 57 fps) · worst 35.38 ms · 84 spikes over 1.5× median · 49.8 stutters/min (work 54.3/min)
- warm-up: 427 frames over 5.3 s, worst 570.6 ms
- hitches (> 25.4 ms): **5** measured, 5 in warm-up
- timed GPU passes: 8.47 ms of a 10.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.17 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 0.66 min = 49.79/min, limit 1/min
- **FAIL** worst frame 35.38 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.218 | 2.588 | 0.188 |
| main_opaque_pass_3d | 1.839 | 2.591 | 0.044 |
| early prepass | 0.787 | 1.128 | 0.081 |
| lightprobe_irradiance_map | 0.439 | 0.751 | 0.001 |
| atmosphere_luts | 0.389 | 0.629 | 0.003 |
| weather_map | 0.368 | 0.368 | 0.002 |
| bloom | 0.339 | 0.493 | 0.066 |
| lightprobe_radiance_map | 0.229 | 0.236 | 0.004 |
| ssao | 0.216 | 0.212 | 0.007 |
| uniform_allocation | 0.187 | 0.203 | 0.016 |
| clouds_trace | 0.157 | 0.200 | 0.001 |
| water_fft | 0.131 | 0.128 | 0.003 |
| upscaling | 0.129 | 0.153 | 0.004 |
| bin_unpacking | 0.115 | 0.138 | 0.011 |
| atmos_froxel_inject | 0.097 | 0.102 | 0.001 |
| tonemapping | 0.090 | 0.092 | 0.003 |
| water_velocity | 0.081 | 0.081 | 0.001 |
| main_transparent_pass_3d | 0.068 | 0.099 | 0.105 |
| clustering | 0.067 | 0.070 | 0.020 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| auxiliary_views | 0.064 | 0.351 | 0.059 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.048 | 0.048 | 0.005 |
| clouds_resolve | 0.036 | 0.040 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.026 | 0.007 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.965 | 11.966 | 33.56 | 33601.4 |
| render/busy/ms | 8.357 | 11.430 | 33.10 | 31323.9 |
| main/busy/ms | 7.518 | 9.753 | 33.27 | 28176.5 |
| render/graph/ms | 4.022 | 5.596 | 28.08 | 15073.3 |
| render/present/ms | 0.502 | 1.193 | 16.79 | 1880.9 |
| bigspace/total/ms | 0.356 | 0.560 | 1.92 | 1334.7 |
| terrain/update/ms | 0.285 | 0.352 | 1.91 | 1067.5 |
| stream/drive/ms | 0.255 | 0.453 | 4.10 | 955.0 |
| terrain/select/ms | 0.248 | 0.295 | 0.82 | 928.3 |
| stream/tick/ms | 0.203 | 0.357 | 0.75 | 761.4 |
| controller/feed/ms | 0.185 | 0.260 | 0.62 | 691.8 |
| far/select/ms | 0.170 | 0.293 | 0.82 | 636.2 |
| anim/evaluate_cpu/ms | 0.120 | 0.160 | 0.54 | 449.1 |
| bigspace/low_precision/ms | 0.120 | 0.192 | 1.21 | 448.9 |
| anim/evaluate/ms | 0.120 | 0.233 | 1.14 | 448.4 |
| render/acquire/ms | 0.106 | 0.149 | 1.58 | 396.7 |
| bigspace/high_precision/ms | 0.096 | 0.144 | 1.63 | 360.1 |
| controller/move/ms | 0.089 | 0.190 | 0.63 | 333.0 |
| anim/apply/ms | 0.066 | 0.103 | 0.99 | 247.9 |
| physics/step/ms | 0.040 | 0.078 | 0.69 | 150.3 |
| physics/insert/ms | 0.031 | 0.298 | 1.10 | 116.2 |
| weather/map/ms | 0.026 | 0.034 | 0.40 | 97.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 41.3 |
| controller/avoid/ms | 0.009 | 0.014 | 0.24 | 34.4 |
| script/update/ms | 0.007 | 0.012 | 0.41 | 26.3 |
| stream/spawn/ms | 0.007 | 0.000 | 1.34 | 24.4 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 17.8 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 15.4 |
| controller/write/ms | 0.003 | 0.005 | 0.40 | 11.3 |
| stream/despawn/ms | 0.002 | 0.000 | 0.80 | 9.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 6.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.5 |
| shapes/activate/ms | 0.001 | 0.003 | 0.06 | 4.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 1 |
| anim/a2/count | count | 1 | 5 | 1 |
| anim/a3/count | count | 441 | 490 | 489 |
| anim/characters/count | count | 441 | 492 | 491 |
| anim/evaluated/count | count | 14 | 36 | 14 |
| controller/movers/count | count | 16 | 18 | 15 |
| ecs/entities | count | 48.4 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.48 | 33.27 | 10.34 |
| gpu/timed/ms | ms | 8.28 | 12.08 | 8.55 |
| mesh_allocator_allocations |  meshes | 13.3 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 446.61 M | 446.61 M | 446.61 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 392 | 417 | 350 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 392 | 417 | 350 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/opaque3d/batch_sets | count | 14 | 16 | 15 |
| phases/opaque3d/bins | count | 1970 | 2115 | 1788 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 4266 | 4504 | 4063 |
| phases/opaque3d/pipelines | count | 13 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1970 | 2115 | 1788 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 4266 | 4504 | 4063 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 14 |
| phases/shadow/batch_sets | count | 30 | 275 | 43 |
| phases/shadow/bins | count | 2857 | 8248 | 6343 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8205 | 13.9 k | 11.7 k |
| phases/shadow/pipelines | count | 32 | 283 | 191 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2405 | 3288 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 973 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 166 | 178 | 178 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 168 | 714 | 199 |
| preprocess/bind_groups_created | count | 168 | 714 | 199 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.52 | 37.50 | 24.04 |
| process/mem_usage | GiB | 2.33 | 2.70 | 2.70 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2924 | 3317 | 3317 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 377 | 540 | 534 |
| statics/materials_bound | count | 377 | 540 | 534 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 1 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 72 | 0 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.24 | 0.08 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 3 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.37 | 48.07 | 33.79 |
| system/mem_usage | % | 63.23 | 64.26 | 64.26 |
| terrain/patches | count | 498 | 520 | 490 |
| views/all/batch_sets | count | 74 | 323 | 89 |
| views/all/bins | count | 7715 | 12.6 k | 10.6 k |
| views/all/count | count | 4 | 54 | 40 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.8 k | 24.2 k | 21.9 k |
| views/all/pipelines | count | 70 | 327 | 233 |
| views/all/pipelines_distinct | count | 55 | 70 | 67 |
| views/all/work_items | count | 19.0 k | 23.9 k | 18.5 k |
| views/cascade/batch_sets | count | 30 | 44 | 37 |
| views/cascade/bins | count | 2822 | 3380 | 2830 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8077 | 9025 | 8014 |
| views/cascade/pipelines | count | 30 | 44 | 37 |
| views/cascade/work_items | count | 8077 | 9025 | 8014 |
| views/lamp_face/batch_sets | count | 0 | 237 | 6 |
| views/lamp_face/bins | count | 0 | 5062 | 3513 |
| views/lamp_face/count | count | 0 | 48 | 36 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 5308 | 3727 |
| views/lamp_face/pipelines | count | 0 | 239 | 154 |
| views/lamp_face/work_items | count | 0 | 5262 | 117 |
| views/main/batch_sets | count | 44 | 48 | 46 |
| views/main/bins | count | 4720 | 5016 | 4276 |
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
| world/meshes | count | 26.9 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 21.94 | 322 | 82.2 k |
| anim/events/count | count | 0.11 | 2 | 416 |
| assets/Image/added | count | 0.08 | 31 | 306 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.60 | 279 | 6007 |
| assets/Mesh/removed | count | 0.65 | 718 | 2434 |
| assets/StandardMaterial/added | count | 0.13 | 33 | 492 |
| assets/StandardMaterial/modified | count | 2.10 | 10 | 7882 |
| assets/StandardMaterial/removed | count | 0.03 | 26 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 5.06 | 1637 | 19.0 k |
| physics/inserts | count | 0.80 | 158 | 3010 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 1315 |
| physics/steps | count | 0.64 | 2 | 2385 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 7 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 18 | 214 |
| stream/deactivate/items | count | 0.04 | 16 | 147 |
| stream/despawn/items | count | 0.03 | 13 | 95 |
| stream/fetch/items | count | 0.02 | 2 | 77 |
| stream/shift_guard/frames | count | 0.00 | 1 | 4 |
| stream/spawn/items | count | 0.11 | 18 | 394 |
| stream/upload/bytes | bytes | 10.5 k | 2.07 M | 39.18 M |
| stream/upload/items | count | 0.01 | 2 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2438 | 1.02 M | 9.14 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 25.4 ms; slowest 10, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 570.6 | main/busy/ms 551, surface/advance_cpu_ms 66.30, surface/cell_upload_ms 0.64, stream/drive/ms 0.33, stream/tick/ms 0.10, weather/map/ms 0.10, bigspace/total/ms 0.06, weather/eval/ms 0.04, bigspace/high_precision/ms 0.03, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.92 | 344.5 | main/busy/ms 582, render/thread/ms 309, render/busy/ms 235, render/acquire/ms 71.12, surface/advance_cpu_ms 66.30, render/graph/ms 5.07, render/present/ms 2.59, stream/drive/ms 0.92, surface/cell_upload_ms 0.64, terrain/update/ms 0.56, stream/tick/ms 0.43, terrain/select/ms 0.38 |
| warm-up | 0.98 | 68.0 | render/thread/ms 328, render/busy/ms 252, render/acquire/ms 71.44, main/busy/ms 44.84, terrain/update/ms 16.05, render/graph/ms 9.19, render/present/ms 3.70, stream/drive/ms 0.93, terrain/select/ms 0.68, stream/tick/ms 0.61, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.15 |
| warm-up | 1.19 | 90.1 | render/thread/ms 60.26, render/busy/ms 58.70, main/busy/ms 57.79, render/graph/ms 3.48, stream/drive/ms 1.77, render/present/ms 1.27, stream/tick/ms 1.26, terrain/update/ms 0.83, terrain/select/ms 0.66, bigspace/total/ms 0.33, render/acquire/ms 0.28, bigspace/high_precision/ms 0.21 |
| warm-up | 1.23 | 28.5 | render/thread/ms 36.55, render/busy/ms 35.23, main/busy/ms 18.99, stream/drive/ms 3.53, render/graph/ms 2.41, stream/tick/ms 1.66, stream/spawn/ms 1.22, render/present/ms 1.13, terrain/update/ms 0.96, terrain/select/ms 0.77, bigspace/total/ms 0.29, render/acquire/ms 0.20 |
| measure | 14.63 | 27.1 | render/thread/ms 34.84, render/busy/ms 19.04, main/busy/ms 15.85, render/present/ms 15.61, render/graph/ms 9.02, bigspace/total/ms 0.80, terrain/update/ms 0.53, far/select/ms 0.52, stream/drive/ms 0.50, terrain/select/ms 0.47, controller/feed/ms 0.42, stream/tick/ms 0.41 |
| measure | 29.58 | 28.1 | render/thread/ms 36.62, render/busy/ms 19.14, render/present/ms 17.30, main/busy/ms 17.04, render/graph/ms 10.13, anim/apply/ms 1.08, bigspace/total/ms 0.81, stream/drive/ms 0.72, stream/tick/ms 0.64, controller/feed/ms 0.56, terrain/update/ms 0.54, terrain/select/ms 0.48 |
| measure | 34.43 | 35.4 | render/thread/ms 44.36, render/busy/ms 43.36, main/busy/ms 41.99, render/graph/ms 33.93, render/present/ms 0.84, bigspace/total/ms 0.70, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.37, controller/feed/ms 0.35, stream/tick/ms 0.32, bigspace/low_precision/ms 0.31 |
| measure | 36.72 | 31.2 | render/thread/ms 40.40, render/busy/ms 39.39, main/busy/ms 37.35, render/graph/ms 29.16, render/present/ms 0.82, bigspace/total/ms 0.68, physics/insert/ms 0.63, terrain/update/ms 0.61, far/select/ms 0.57, terrain/select/ms 0.53, stream/drive/ms 0.36, controller/feed/ms 0.34 |
| measure | 37.38 | 33.2 | render/thread/ms 43.12, render/busy/ms 41.99, render/graph/ms 18.50, main/busy/ms 18.11, terrain/update/ms 1.18, render/present/ms 0.95, bigspace/total/ms 0.72, physics/insert/ms 0.61, terrain/select/ms 0.47, controller/feed/ms 0.43, stream/drive/ms 0.42, stream/tick/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\run@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 1%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (154 samples): GPU load **89%** (min 63%) · 2580 MHz (min 2580) · memory 8001 MHz · 84.3 W · 74 °C (max 78) · VRAM 4594 MiB · throttled 100% of samples
