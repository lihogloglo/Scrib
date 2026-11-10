# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5978 frames over 65.6 s · **median 10.75 ms (93 fps)** · mean 10.98 · p95 13.07 · p99 14.72 (1% low 68 fps) · worst 26.10 ms · 24 spikes over 1.5× median · 21.9 stutters/min (work 22.9/min)
- warm-up: 468 frames over 5.5 s, worst 570.4 ms
- hitches (> 26.9 ms): **0** measured, 7 in warm-up
- timed GPU passes: 7.50 ms of a 10.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.75 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 26.10 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.246 | 2.800 | 0.182 |
| main_opaque_pass_3d | 1.472 | 2.176 | 0.044 |
| early prepass | 0.507 | 0.778 | 0.079 |
| lightprobe_irradiance_map | 0.439 | 0.754 | 0.001 |
| bloom | 0.340 | 0.492 | 0.066 |
| atmosphere_luts | 0.332 | 0.500 | 0.003 |
| lightprobe_radiance_map | 0.226 | 0.234 | 0.004 |
| ssao | 0.219 | 0.212 | 0.007 |
| uniform_allocation | 0.184 | 0.183 | 0.016 |
| auxiliary_views | 0.153 | 0.591 | 0.136 |
| water_fft | 0.131 | 0.128 | 0.002 |
| bin_unpacking | 0.129 | 0.141 | 0.012 |
| upscaling | 0.127 | 0.138 | 0.004 |
| clouds_trace | 0.126 | 0.214 | 0.001 |
| atmos_froxel_inject | 0.101 | 0.108 | 0.001 |
| tonemapping | 0.093 | 0.092 | 0.003 |
| water_velocity | 0.089 | 0.085 | 0.001 |
| clustering | 0.067 | 0.069 | 0.019 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.061 | 0.102 | 0.106 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.048 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.432 | 11.439 | 30.19 | 56385.1 |
| render/busy/ms | 8.882 | 10.925 | 22.05 | 53098.4 |
| main/busy/ms | 8.024 | 9.636 | 15.74 | 47965.0 |
| render/graph/ms | 4.380 | 5.598 | 12.45 | 26180.7 |
| render/present/ms | 0.460 | 0.594 | 11.79 | 2746.9 |
| bigspace/total/ms | 0.409 | 0.585 | 2.00 | 2443.6 |
| terrain/update/ms | 0.288 | 0.363 | 1.43 | 1723.1 |
| terrain/select/ms | 0.253 | 0.318 | 0.93 | 1515.0 |
| stream/drive/ms | 0.250 | 0.435 | 2.16 | 1493.3 |
| controller/feed/ms | 0.239 | 0.311 | 0.69 | 1426.6 |
| far/select/ms | 0.223 | 0.381 | 1.07 | 1335.7 |
| stream/tick/ms | 0.204 | 0.357 | 1.01 | 1217.3 |
| anim/evaluate_cpu/ms | 0.159 | 0.201 | 0.65 | 952.2 |
| bigspace/low_precision/ms | 0.159 | 0.243 | 1.63 | 949.8 |
| controller/move/ms | 0.153 | 0.304 | 0.82 | 915.4 |
| anim/evaluate/ms | 0.146 | 0.267 | 1.44 | 874.1 |
| bigspace/high_precision/ms | 0.094 | 0.139 | 1.38 | 559.6 |
| render/acquire/ms | 0.090 | 0.114 | 0.51 | 539.8 |
| anim/apply/ms | 0.085 | 0.130 | 1.56 | 508.0 |
| physics/step/ms | 0.048 | 0.084 | 0.55 | 285.1 |
| physics/insert/ms | 0.036 | 0.362 | 0.98 | 213.9 |
| weather/map/ms | 0.026 | 0.034 | 0.36 | 154.5 |
| controller/avoid/ms | 0.012 | 0.018 | 0.37 | 69.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 58.5 |
| script/update/ms | 0.006 | 0.008 | 0.36 | 38.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.30 | 32.6 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 29.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 26.7 |
| script/ui/ms | 0.004 | 0.006 | 0.35 | 26.1 |
| controller/write/ms | 0.004 | 0.006 | 0.16 | 22.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 16.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 6.4 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.55 | 5.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.77 | 22.05 | 8.20 |
| gpu/timed/ms | ms | 7.46 | 10.28 | 8.20 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 544.17 M | 544.17 M | 544.17 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5135 | 3180 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5135 | 3180 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 40 | 235 | 29 |
| phases/shadow/bins | count | 3863 | 5481 | 2244 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9693 | 12.9 k | 5695 |
| phases/shadow/pipelines | count | 114 | 251 | 29 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2662 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 185 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 189 | 618 | 166 |
| preprocess/bind_groups_created | count | 189 | 618 | 166 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25 | 40.69 | 20.57 |
| process/mem_usage | GiB | 2.64 | 2.83 | 2.83 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3444 | 3506 | 3506 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.64 | 48.09 | 33.62 |
| system/mem_usage | % | 64.08 | 64.72 | 64.72 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 275 | 73 |
| views/all/bins | count | 6542 | 8363 | 5724 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 156 | 287 | 69 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.4 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 29 |
| views/cascade/bins | count | 2613 | 3185 | 2244 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8176 | 10.2 k | 5695 |
| views/cascade/pipelines | count | 36 | 50 | 29 |
| views/cascade/work_items | count | 8176 | 10.2 k | 5695 |
| views/lamp_face/batch_sets | count | 0 | 196 | 0 |
| views/lamp_face/bins | count | 1134 | 2474 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1293 | 2833 | 0 |
| views/lamp_face/pipelines | count | 76 | 205 | 0 |
| views/lamp_face/work_items | count | 0 | 2681 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8494 | 13.0 k | 7594 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8655 | 13.3 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 57.64 | 230 | 344.6 k |
| anim/events/count | count | 0.26 | 4 | 1566 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.68 | 160 | 4086 |
| assets/Mesh/removed | count | 0.82 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.62 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.79 | 1494 | 4737 |
| physics/inserts | count | 0.39 | 88 | 2353 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5663 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1242 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 570.4 | main/busy/ms 550, surface/advance_cpu_ms 67.60, surface/cell_upload_ms 0.70, stream/drive/ms 0.32, weather/map/ms 0.14, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.80 | 226.7 | main/busy/ms 586, render/thread/ms 190, render/busy/ms 124, surface/advance_cpu_ms 67.60, render/acquire/ms 63.60, render/graph/ms 4.56, render/present/ms 2.22, stream/drive/ms 1.23, surface/cell_upload_ms 0.70, stream/tick/ms 0.66, terrain/update/ms 0.63, terrain/select/ms 0.46 |
| warm-up | 0.95 | 149.0 | render/thread/ms 207, render/busy/ms 140, render/acquire/ms 63.86, main/busy/ms 51.07, terrain/update/ms 20.84, render/graph/ms 9.69, render/present/ms 3.69, stream/drive/ms 1.32, stream/tick/ms 0.91, terrain/select/ms 0.79, bigspace/total/ms 0.23, bigspace/high_precision/ms 0.18 |
| warm-up | 1.17 | 92.8 | main/busy/ms 64.88, render/thread/ms 49.40, render/busy/ms 48.00, render/graph/ms 3.71, stream/drive/ms 2.36, stream/tick/ms 1.66, render/present/ms 1.14, terrain/update/ms 0.87, terrain/select/ms 0.72, bigspace/total/ms 0.41, stream/spawn/ms 0.30, render/acquire/ms 0.26 |
| warm-up | 1.34 | 27.2 | render/thread/ms 36.38, render/busy/ms 35.24, main/busy/ms 16.96, render/graph/ms 3.89, stream/drive/ms 3.55, stream/spawn/ms 1.84, render/present/ms 0.97, stream/tick/ms 0.86, terrain/update/ms 0.69, terrain/select/ms 0.56, bigspace/total/ms 0.40, bigspace/low_precision/ms 0.23 |
| warm-up | 2.90 | 27.5 | main/busy/ms 33.37, render/thread/ms 19.47, render/busy/ms 17.94, render/graph/ms 4.30, render/present/ms 1.34, stream/drive/ms 1.06, terrain/update/ms 0.68, bigspace/total/ms 0.59, terrain/select/ms 0.57, stream/tick/ms 0.55, anim/evaluate_cpu/ms 0.31, far/select/ms 0.26 |
| warm-up | 3.76 | 29.1 | main/busy/ms 37.29, render/thread/ms 27.09, render/busy/ms 26.05, render/graph/ms 5.32, terrain/update/ms 2.61, physics/insert/ms 1.09, bigspace/total/ms 1.07, stream/drive/ms 1.03, render/present/ms 0.85, anim/evaluate_cpu/ms 0.71, bigspace/low_precision/ms 0.52, terrain/select/ms 0.51 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **81%** (min 64%) · 2580 MHz (min 2580) · memory 8001 MHz · 79.2 W · 72 °C (max 78) · VRAM 5126 MiB · throttled 100% of samples
