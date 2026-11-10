# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6483 frames over 71.5 s · **median 10.90 ms (92 fps)** · mean 11.03 · p95 14.13 · p99 16.93 (1% low 59 fps) · worst 29.42 ms · 81 spikes over 1.5× median · 73.0 stutters/min (work 88.1/min)
- warm-up: 278 frames over 3.4 s, worst 577.6 ms
- hitches (> 27.2 ms): **1** measured, 5 in warm-up
- timed GPU passes: 8.31 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.90 ms, limit 11 ms
- **FAIL** frame_ms: 87 stutters (> 1.5x moving median of 60) in 1.19 min = 72.99/min, limit 1/min
- pass worst frame 29.42 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.214 | 2.683 | 0.178 |
| main_opaque_pass_3d | 2.175 | 2.944 | 0.042 |
| early prepass | 0.550 | 0.953 | 0.079 |
| lightprobe_irradiance_map | 0.445 | 0.759 | 0.001 |
| atmosphere_luts | 0.386 | 0.657 | 0.003 |
| bloom | 0.332 | 0.481 | 0.064 |
| clouds_trace | 0.226 | 0.269 | 0.001 |
| lightprobe_radiance_map | 0.226 | 0.234 | 0.004 |
| ssao | 0.215 | 0.210 | 0.007 |
| main_transparent_pass_3d | 0.208 | 0.502 | 0.088 |
| uniform_allocation | 0.152 | 0.282 | 0.015 |
| water_fft | 0.133 | 0.130 | 0.003 |
| upscaling | 0.128 | 0.207 | 0.004 |
| atmos_froxel_inject | 0.094 | 0.094 | 0.001 |
| tonemapping | 0.093 | 0.106 | 0.003 |
| bin_unpacking | 0.089 | 0.106 | 0.010 |
| water_velocity | 0.080 | 0.095 | 0.002 |
| clustering | 0.073 | 0.071 | 0.021 |
| clouds_composite | 0.067 | 0.068 | 0.002 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| clouds_resolve | 0.037 | 0.046 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.041 | 0.005 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.028 | 0.007 |
| lightprobe_copy | 0.018 | 0.018 | 0.004 |
| lightprobe_downsampling_first_pass | 0.015 | 0.017 | 0.001 |
| auxiliary_views | 0.014 | 0.002 | 0.013 |
| weather_map | 0.013 | 0.013 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.002 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.012 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.124 | 11.943 | 28.15 | 59152.9 |
| render/busy/ms | 8.522 | 11.353 | 22.19 | 55247.4 |
| main/busy/ms | 7.753 | 9.448 | 20.30 | 50264.4 |
| render/graph/ms | 3.885 | 4.867 | 12.98 | 25186.0 |
| render/present/ms | 0.468 | 0.627 | 11.65 | 3034.0 |
| bigspace/total/ms | 0.426 | 0.824 | 3.59 | 2760.6 |
| stream/drive/ms | 0.324 | 0.848 | 10.58 | 2099.3 |
| terrain/update/ms | 0.262 | 0.489 | 2.21 | 1696.1 |
| controller/feed/ms | 0.226 | 0.355 | 0.94 | 1462.0 |
| far/select/ms | 0.224 | 0.407 | 1.29 | 1455.2 |
| terrain/select/ms | 0.197 | 0.257 | 0.69 | 1278.3 |
| stream/tick/ms | 0.191 | 0.335 | 0.70 | 1235.3 |
| controller/move/ms | 0.155 | 0.389 | 1.35 | 1006.5 |
| bigspace/low_precision/ms | 0.149 | 0.316 | 3.09 | 968.5 |
| render/acquire/ms | 0.134 | 0.246 | 2.83 | 871.5 |
| anim/evaluate_cpu/ms | 0.128 | 0.211 | 0.69 | 830.9 |
| bigspace/high_precision/ms | 0.114 | 0.245 | 1.29 | 739.6 |
| anim/apply/ms | 0.104 | 0.198 | 1.20 | 674.0 |
| anim/evaluate/ms | 0.096 | 0.214 | 1.35 | 624.8 |
| physics/step/ms | 0.080 | 0.231 | 1.87 | 515.7 |
| stream/despawn/ms | 0.044 | 0.429 | 1.78 | 284.6 |
| stream/spawn/ms | 0.036 | 0.240 | 10.17 | 231.5 |
| weather/map/ms | 0.027 | 0.036 | 0.47 | 174.1 |
| physics/insert/ms | 0.021 | 0.110 | 0.76 | 133.0 |
| controller/avoid/ms | 0.011 | 0.018 | 0.34 | 70.2 |
| weather/eval/ms | 0.010 | 0.015 | 0.15 | 64.3 |
| script/update/ms | 0.007 | 0.010 | 0.27 | 43.6 |
| stream/deactivate/ms | 0.006 | 0.000 | 0.77 | 37.8 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 32.9 |
| script/ui/ms | 0.005 | 0.007 | 0.34 | 32.0 |
| stream/activate/ms | 0.005 | 0.000 | 0.60 | 31.7 |
| controller/write/ms | 0.003 | 0.006 | 0.13 | 21.4 |
| stream/upload/ms | 0.001 | 0.013 | 0.09 | 9.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 6.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.5 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.06 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 3.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 604 | 778 | 471 |
| anim/characters/count | count | 604 | 782 | 471 |
| anim/evaluated/count | count | 19 | 56 | 14 |
| controller/movers/count | count | 18 | 32 | 14 |
| ecs/entities | count | 59.9 k | 72.5 k | 53.0 k |
| far/entities | count | 2681 | 3461 | 2126 |
| far/forced | count | 0 | 8 | 0 |
| far/impostors | count | 61.0 k | 65.7 k | 55.8 k |
| far/lights | count | 7115 | 7757 | 6575 |
| far/nodes/l1 | count | 1100 | 1439 | 857 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 227.79 M | 173.66 M |
| far/triangles | count | 5.04 M | 5.72 M | 4.26 M |
| frame/work/ms | ms | 8.79 | 22.19 | 8.27 |
| gpu/timed/ms | ms | 8.39 | 12.55 | 7.39 |
| mesh_allocator_allocations |  meshes | 15.8 k | 19.2 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 537.07 M | 645.72 M | 645.72 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d/bins | count | 321 | 410 | 39 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 508 | 907 | 170 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d_prepass/bins | count | 321 | 410 | 39 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 508 | 907 | 170 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 5 |
| phases/opaque3d/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d/bins | count | 1141 | 1967 | 536 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3002 | 4157 | 1240 |
| phases/opaque3d/pipelines | count | 13 | 17 | 10 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d_prepass/bins | count | 1141 | 1967 | 536 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3002 | 4157 | 1240 |
| phases/opaque3d_prepass/pipelines | count | 13 | 17 | 10 |
| phases/shadow/batch_sets | count | 19 | 157 | 16 |
| phases/shadow/bins | count | 1564 | 2790 | 1788 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4218 | 6831 | 4574 |
| phases/shadow/pipelines | count | 19 | 163 | 16 |
| physics/awake | count | 0 | 2 | 0 |
| physics/bodies | count | 1894 | 2692 | 1958 |
| physics/items | count | 10 | 109 | 109 |
| physics/pending | count | 0 | 2434 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 41 | 0 |
| preprocess/bind_groups | count | 142 | 430 | 112 |
| preprocess/bind_groups_created | count | 142 | 430 | 112 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.52 | 41.40 | 19.71 |
| process/mem_usage | GiB | 2.55 | 2.68 | 2.68 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3883 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 373 | 446 | 406 |
| statics/materials_bound | count | 373 | 446 | 406 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 63 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 201 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.61 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 170 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 7 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.75 | 52.97 | 36.94 |
| system/mem_usage | % | 63.72 | 64.18 | 64.17 |
| terrain/patches | count | 349 | 493 | 319 |
| views/all/batch_sets | count | 61 | 187 | 46 |
| views/all/bins | count | 4492 | 6726 | 2938 |
| views/all/count | count | 4 | 43 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 16.0 k | 7394 |
| views/all/pipelines | count | 58 | 189 | 46 |
| views/all/pipelines_distinct | count | 53 | 61 | 44 |
| views/all/work_items | count | 11.5 k | 16.2 k | 7423 |
| views/cascade/batch_sets | count | 19 | 30 | 16 |
| views/cascade/bins | count | 1550 | 2691 | 1788 |
| views/cascade/count | count | 3 | 6 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4170 | 6831 | 4574 |
| views/cascade/pipelines | count | 19 | 30 | 16 |
| views/cascade/work_items | count | 4170 | 6831 | 4574 |
| views/lamp_face/batch_sets | count | 0 | 136 | 0 |
| views/lamp_face/bins | count | 0 | 1498 | 0 |
| views/lamp_face/count | count | 0 | 36 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2177 | 0 |
| views/lamp_face/pipelines | count | 0 | 138 | 0 |
| views/lamp_face/work_items | count | 0 | 2144 | 0 |
| views/main/batch_sets | count | 40 | 50 | 30 |
| views/main/bins | count | 2851 | 4444 | 1150 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7166 | 10.1 k | 2820 |
| views/main/pipelines | count | 38 | 46 | 30 |
| views/main/work_items | count | 7360 | 10.3 k | 2849 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 26.5 k | 33.5 k | 28.1 k |
| world/point_lights | count | 270 | 549 | 524 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 9.09 | 225 | 58.9 k |
| anim/events/count | count | 0.20 | 5 | 1299 |
| assets/Image/added | count | 0.15 | 38 | 981 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 6.27 | 494 | 40.6 k |
| assets/Mesh/removed | count | 10.21 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.19 | 39 | 1250 |
| assets/StandardMaterial/modified | count | 5.26 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.07 | 36 | 470 |
| assets/WorldAsset/added | count | 0.01 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 124 |
| ecs/entities_delta | count | 3.61 | 2172 | 23.4 k |
| physics/inserts | count | 2.45 | 131 | 15.9 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.18 | 274 | 14.2 k |
| physics/steps | count | 0.66 | 2 | 4290 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 32 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.21 | 22 | 1345 |
| stream/deactivate/items | count | 0.21 | 23 | 1345 |
| stream/despawn/items | count | 1.08 | 252 | 6982 |
| stream/fetch/items | count | 0.23 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.71 | 30 | 4586 |
| stream/upload/bytes | bytes | 76.4 k | 3.51 M | 495.17 M |
| stream/upload/items | count | 0.20 | 9 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.04 | 1 | 274 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 17.7 k | 1.21 M | 114.84 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 27.2 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.6 | main/busy/ms 557, surface/advance_cpu_ms 67.81, surface/cell_upload_ms 0.67, stream/drive/ms 0.30, weather/map/ms 0.11, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.04, script/update/ms 0.02, weather/eval/ms 0.02, far/select/ms 0.02, script/input/ms 0.02 |
| warm-up | 0.92 | 338.3 | main/busy/ms 590, render/thread/ms 303, render/busy/ms 229, render/acquire/ms 71.41, surface/advance_cpu_ms 67.81, render/graph/ms 6.13, render/present/ms 2.66, stream/drive/ms 0.82, surface/cell_upload_ms 0.67, terrain/update/ms 0.45, stream/tick/ms 0.43, terrain/select/ms 0.27 |
| warm-up | 1.01 | 91.3 | render/thread/ms 323, render/busy/ms 247, render/acquire/ms 71.70, main/busy/ms 51.07, terrain/update/ms 18.13, render/graph/ms 11.03, render/present/ms 4.18, stream/drive/ms 0.86, stream/tick/ms 0.62, terrain/select/ms 0.47, bigspace/total/ms 0.32, bigspace/high_precision/ms 0.26 |
| warm-up | 1.07 | 28.7 | render/thread/ms 44.56, render/acquire/ms 27.58, main/busy/ms 22.15, render/busy/ms 15.09, terrain/update/ms 12.10, render/graph/ms 4.25, render/present/ms 1.89, stream/drive/ms 1.19, stream/tick/ms 0.98, terrain/select/ms 0.75, bigspace/total/ms 0.15, weather/map/ms 0.09 |
| warm-up | 1.21 | 91.0 | render/thread/ms 82.34, main/busy/ms 74.28, render/busy/ms 71.46, render/acquire/ms 8.89, render/graph/ms 5.57, render/present/ms 1.98, terrain/update/ms 1.97, stream/drive/ms 1.49, stream/tick/ms 1.01, bigspace/total/ms 0.76, bigspace/high_precision/ms 0.41, terrain/select/ms 0.39 |
| measure | 0.07 | 29.4 | render/thread/ms 29.00, main/busy/ms 28.39, render/busy/ms 27.82, render/graph/ms 4.04, stream/drive/ms 0.97, render/present/ms 0.85, bigspace/total/ms 0.59, terrain/update/ms 0.41, far/select/ms 0.40, stream/tick/ms 0.34, terrain/select/ms 0.34, render/acquire/ms 0.33 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\flight@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 7%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (278 samples): GPU load **84%** (min 44%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.8 W · 75 °C (max 78) · VRAM 4872 MiB · throttled 100% of samples
