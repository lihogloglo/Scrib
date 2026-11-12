# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5502 frames over 65.6 s · **median 11.36 ms (88 fps)** · mean 11.93 · p95 15.64 · p99 18.63 (1% low 54 fps) · worst 108.41 ms · 124 spikes over 1.5× median · 26.5 stutters/min (work 31.1/min)
- warm-up: 423 frames over 5.9 s, worst 600.3 ms
- hitches (> 28.4 ms): **1** measured, 6 in warm-up
- timed GPU passes: 7.85 ms of a 11.93 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.36 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- **FAIL** worst frame 108.41 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.306 | 2.991 | 0.187 |
| main_opaque_pass_3d | 1.532 | 2.073 | 0.047 |
| early prepass | 0.560 | 0.994 | 0.081 |
| lightprobe_irradiance_map | 0.468 | 0.705 | 0.001 |
| bloom | 0.350 | 0.564 | 0.069 |
| atmosphere_luts | 0.346 | 0.612 | 0.003 |
| lightprobe_radiance_map | 0.237 | 0.372 | 0.005 |
| ssao | 0.224 | 0.380 | 0.008 |
| uniform_allocation | 0.205 | 0.453 | 0.019 |
| auxiliary_views | 0.188 | 0.727 | 0.170 |
| bin_unpacking | 0.146 | 0.309 | 0.014 |
| water_fft | 0.140 | 0.138 | 0.003 |
| upscaling | 0.129 | 0.144 | 0.005 |
| clouds_trace | 0.127 | 0.215 | 0.001 |
| atmos_froxel_inject | 0.104 | 0.110 | 0.002 |
| tonemapping | 0.095 | 0.094 | 0.004 |
| water_velocity | 0.091 | 0.085 | 0.002 |
| clustering | 0.069 | 0.069 | 0.020 |
| clouds_composite | 0.066 | 0.063 | 0.002 |
| surface_trail | 0.061 | 0.058 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.112 | 0.112 |
| render_sky | 0.055 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.046 | 0.048 | 0.005 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.032 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.002 |
| water_mips | 0.025 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.012 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.004 | 0.000 | 9872.88 | 71545.5 |
| render/thread/ms | 10.244 | 13.777 | 107.24 | 56361.5 |
| render/busy/ms | 9.662 | 13.099 | 106.63 | 53161.3 |
| main/busy/ms | 8.743 | 11.272 | 18.38 | 48104.9 |
| render/graph/ms | 4.863 | 6.845 | 102.44 | 26756.5 |
| render/present/ms | 0.485 | 0.623 | 4.49 | 2667.4 |
| bigspace/total/ms | 0.451 | 0.732 | 2.30 | 2479.1 |
| terrain/update/ms | 0.294 | 0.374 | 2.14 | 1615.0 |
| controller/feed/ms | 0.271 | 0.379 | 0.91 | 1489.3 |
| stream/drive/ms | 0.265 | 0.461 | 2.18 | 1460.5 |
| terrain/select/ms | 0.257 | 0.328 | 0.76 | 1416.0 |
| pipelines/compile_ms | 0.239 | 0.000 | 164.62 | 1314.3 |
| pipelines/front_ms | 0.230 | 0.000 | 158.91 | 1266.7 |
| far/select/ms | 0.229 | 0.384 | 0.93 | 1259.8 |
| stream/tick/ms | 0.213 | 0.367 | 0.92 | 1169.5 |
| anim/evaluate_cpu/ms | 0.171 | 0.230 | 0.68 | 940.4 |
| bigspace/low_precision/ms | 0.166 | 0.272 | 1.54 | 914.8 |
| anim/evaluate/ms | 0.145 | 0.244 | 2.18 | 799.7 |
| controller/move/ms | 0.145 | 0.275 | 0.64 | 796.7 |
| bigspace/high_precision/ms | 0.107 | 0.192 | 1.76 | 589.3 |
| render/acquire/ms | 0.097 | 0.132 | 2.60 | 532.7 |
| anim/apply/ms | 0.089 | 0.142 | 1.52 | 489.0 |
| physics/insert/ms | 0.063 | 0.479 | 1.14 | 348.6 |
| physics/step/ms | 0.054 | 0.093 | 0.76 | 298.1 |
| weather/map/ms | 0.028 | 0.036 | 0.33 | 153.3 |
| controller/avoid/ms | 0.012 | 0.017 | 0.22 | 63.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 61.1 |
| pipelines/driver_ms | 0.009 | 0.000 | 8.40 | 47.5 |
| script/update/ms | 0.008 | 0.014 | 0.22 | 43.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 35.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 30.1 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 28.1 |
| script/input/ms | 0.005 | 0.007 | 0.31 | 28.1 |
| controller/write/ms | 0.004 | 0.006 | 0.05 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.77 | 17.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.54 | 5.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.17 | 5.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 21 | 17 |
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
| frame/work/ms | ms | 9.31 | 107 | 8.17 |
| gpu/timed/ms | ms | 7.69 | 13.23 | 7.73 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 200 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 796 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 200 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 796 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1301 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3353 | 5132 | 3179 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1301 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3353 | 5132 | 3179 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 38 | 232 | 28 |
| phases/shadow/bins | count | 3691 | 5292 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9283 | 12.7 k | 5673 |
| phases/shadow/pipelines | count | 102 | 246 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 142 | 476 | 19 |
| physics/pending | count | 0 | 1030 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 187 | 612 | 164 |
| preprocess/bind_groups_created | count | 187 | 612 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 23.56 | 40.10 | 19.23 |
| process/mem_usage | GiB | 2.63 | 2.69 | 2.69 |
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3495 | 3554 | 3554 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 530 | 398 |
| statics/materials_bound | count | 433 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.41 | 64.50 | 29.31 |
| system/mem_usage | % | 67.49 | 68.69 | 67.01 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 83 | 272 | 72 |
| views/all/bins | count | 6232 | 8150 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.2 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 144 | 286 | 68 |
| views/all/pipelines_distinct | count | 60 | 69 | 55 |
| views/all/work_items | count | 17.0 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2604 | 3155 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8137 | 10.1 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8137 | 10.1 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 1042 | 2408 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1165 | 2748 | 0 |
| views/lamp_face/pipelines | count | 66 | 201 | 0 |
| views/lamp_face/work_items | count | 0 | 2572 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2910 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8312 | 13.0 k | 7592 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 8469 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.25 | 328 | 309.5 k |
| anim/events/count | count | 0.27 | 4 | 1509 |
| assets/Image/added | count | 0.06 | 35 | 315 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 160 | 4114 |
| assets/Mesh/removed | count | 0.89 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 473 |
| assets/StandardMaterial/modified | count | 3.93 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.95 | 1379 | 5224 |
| physics/inserts | count | 0.59 | 134 | 3261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 129 | 2335 |
| physics/steps | count | 0.72 | 3 | 3936 |
| physics/steps_dropped | count | 0.00 | 4 | 4 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 18 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6153 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1349 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.4 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.60 | 600.3 | main/busy/ms 577, surface/advance_cpu_ms 75.49, surface/cell_upload_ms 0.71, stream/drive/ms 0.27, weather/map/ms 0.17, bigspace/total/ms 0.08, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.97 | 367.8 | main/busy/ms 616, render/thread/ms 324, render/busy/ms 251, surface/advance_cpu_ms 75.49, render/acquire/ms 70.81, render/graph/ms 6.14, render/present/ms 2.55, stream/drive/ms 1.03, surface/cell_upload_ms 0.71, stream/tick/ms 0.52, terrain/update/ms 0.42, weather/map/ms 0.28 |
| warm-up | 1.05 | 83.8 | pipelines/wait_ms 347, render/thread/ms 344, render/busy/ms 268, pipelines/compile_ms 72.32, render/acquire/ms 71.05, pipelines/front_ms 67.56, main/busy/ms 59.59, terrain/update/ms 23.41, render/graph/ms 11.42, pipelines/driver_ms 4.76, render/present/ms 4.66, stream/drive/ms 1.39 |
| warm-up | 1.30 | 92.4 | pipelines/compile_ms 231, pipelines/front_ms 206, main/busy/ms 75.30, render/thread/ms 57.38, render/busy/ms 55.60, pipelines/driver_ms 24.06, pipelines/wait_ms 9.10, render/graph/ms 3.50, stream/drive/ms 1.81, render/present/ms 1.51, stream/tick/ms 1.40, terrain/update/ms 1.16 |
| warm-up | 3.83 | 32.6 | pipelines/wait_ms 2191, pipelines/compile_ms 155, pipelines/front_ms 151, main/busy/ms 54.62, render/thread/ms 20.91, render/busy/ms 19.70, render/graph/ms 4.69, pipelines/driver_ms 4.25, bigspace/total/ms 1.41, stream/drive/ms 1.17, anim/evaluate_cpu/ms 1.14, render/present/ms 1.00 |
| warm-up | 3.86 | 31.4 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 200, pipelines/front_ms 193, main/busy/ms 41.65, render/thread/ms 27.46, render/busy/ms 26.14, pipelines/driver_ms 6.74, render/graph/ms 5.00, bigspace/total/ms 1.63, stream/drive/ms 1.17, render/present/ms 1.09, anim/evaluate_cpu/ms 1.06 |
| measure | 0.19 | 108.4 | render/thread/ms 117, render/busy/ms 115, render/graph/ms 107, main/busy/ms 16.95, render/present/ms 1.00, physics/insert/ms 0.76, bigspace/total/ms 0.61, terrain/update/ms 0.54, stream/drive/ms 0.50, terrain/select/ms 0.48, stream/tick/ms 0.44, far/select/ms 0.36 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **79%** (min 56%) · 2581 MHz (min 2580) · memory 8001 MHz · 74.6 W · 69 °C (max 74) · VRAM 4935 MiB · throttled 100% of samples
