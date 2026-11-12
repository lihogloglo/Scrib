# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5311 frames over 65.6 s · **median 11.72 ms (85 fps)** · mean 12.36 · p95 16.57 · p99 19.92 (1% low 50 fps) · worst 51.32 ms · 170 spikes over 1.5× median · 43.0 stutters/min (work 48.4/min)
- warm-up: 415 frames over 6.1 s, worst 641.1 ms
- hitches (> 29.3 ms): **1** measured, 5 in warm-up
- timed GPU passes: 7.22 ms of a 12.36 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.72 ms, limit 11 ms
- **FAIL** frame_ms: 47 stutters (> 1.5x moving median of 60) in 1.09 min = 42.96/min, limit 1/min
- **FAIL** worst frame 51.32 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.124 | 2.531 | 0.192 |
| main_opaque_pass_3d | 1.427 | 1.855 | 0.056 |
| early prepass | 0.495 | 0.715 | 0.084 |
| lightprobe_irradiance_map | 0.427 | 0.423 | 0.001 |
| bloom | 0.326 | 0.327 | 0.071 |
| atmosphere_luts | 0.318 | 0.424 | 0.003 |
| lightprobe_radiance_map | 0.225 | 0.234 | 0.005 |
| ssao | 0.210 | 0.208 | 0.008 |
| uniform_allocation | 0.184 | 0.329 | 0.023 |
| auxiliary_views | 0.145 | 0.615 | 0.152 |
| clouds_trace | 0.127 | 0.215 | 0.001 |
| water_fft | 0.126 | 0.128 | 0.003 |
| bin_unpacking | 0.126 | 0.139 | 0.017 |
| upscaling | 0.123 | 0.133 | 0.005 |
| atmos_froxel_inject | 0.095 | 0.100 | 0.002 |
| tonemapping | 0.088 | 0.089 | 0.004 |
| water_velocity | 0.083 | 0.085 | 0.002 |
| clustering | 0.065 | 0.068 | 0.021 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.057 | 0.101 | 0.114 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.048 | 0.006 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.002 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 11.175 | 0.000 | 7802.80 | 59349.9 |
| render/thread/ms | 10.610 | 14.523 | 48.98 | 56351.2 |
| render/busy/ms | 10.030 | 13.833 | 48.17 | 53269.2 |
| main/busy/ms | 8.995 | 11.866 | 44.69 | 47770.8 |
| render/graph/ms | 5.039 | 7.108 | 42.85 | 26759.9 |
| render/present/ms | 0.485 | 0.619 | 4.73 | 2576.2 |
| bigspace/total/ms | 0.463 | 0.719 | 2.63 | 2460.9 |
| terrain/update/ms | 0.298 | 0.388 | 2.01 | 1583.9 |
| controller/feed/ms | 0.292 | 0.414 | 0.71 | 1550.7 |
| stream/drive/ms | 0.280 | 0.478 | 7.32 | 1486.4 |
| terrain/select/ms | 0.261 | 0.338 | 0.82 | 1385.6 |
| far/select/ms | 0.236 | 0.397 | 0.87 | 1255.8 |
| stream/tick/ms | 0.224 | 0.389 | 0.90 | 1189.2 |
| pipelines/compile_ms | 0.212 | 0.000 | 175.16 | 1128.0 |
| pipelines/front_ms | 0.200 | 0.000 | 165.51 | 1063.5 |
| bigspace/low_precision/ms | 0.173 | 0.273 | 1.54 | 918.4 |
| anim/evaluate_cpu/ms | 0.171 | 0.231 | 0.99 | 909.7 |
| anim/evaluate/ms | 0.165 | 0.285 | 2.20 | 876.6 |
| controller/move/ms | 0.141 | 0.260 | 1.64 | 749.9 |
| bigspace/high_precision/ms | 0.105 | 0.187 | 1.71 | 559.4 |
| render/acquire/ms | 0.095 | 0.130 | 0.44 | 505.8 |
| anim/apply/ms | 0.088 | 0.137 | 1.41 | 468.9 |
| physics/insert/ms | 0.063 | 0.491 | 1.30 | 332.7 |
| physics/step/ms | 0.057 | 0.097 | 1.60 | 302.5 |
| weather/map/ms | 0.029 | 0.037 | 0.25 | 153.5 |
| pipelines/driver_ms | 0.012 | 0.000 | 30.28 | 64.5 |
| controller/avoid/ms | 0.012 | 0.018 | 0.07 | 62.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.25 | 62.3 |
| script/update/ms | 0.008 | 0.012 | 0.08 | 41.3 |
| stream/despawn/ms | 0.007 | 0.000 | 2.09 | 37.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 29.2 |
| script/input/ms | 0.005 | 0.007 | 0.12 | 28.5 |
| script/ui/ms | 0.005 | 0.007 | 0.13 | 25.3 |
| stream/deactivate/ms | 0.004 | 0.000 | 1.94 | 21.1 |
| controller/write/ms | 0.004 | 0.006 | 0.25 | 20.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.24 | 6.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.06 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.12 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 10 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
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
| frame/work/ms | ms | 9.49 | 48.17 | 8.54 |
| gpu/timed/ms | ms | 7.23 | 11.75 | 7.69 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 815 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 815 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1301 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5133 | 3179 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1301 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5133 | 3179 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 37 | 232 | 28 |
| phases/shadow/bins | count | 3778 | 5454 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9612 | 12.8 k | 5673 |
| phases/shadow/pipelines | count | 109 | 254 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 186 | 476 | 19 |
| physics/pending | count | 0 | 790 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 184 | 612 | 164 |
| preprocess/bind_groups_created | count | 184 | 612 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.04 | 39.90 | 15.94 |
| process/mem_usage | GiB | 2.56 | 2.61 | 2.61 |
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
| started_load_count |  | 3480 | 3548 | 3548 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.05 | 78.69 | 54.47 |
| system/mem_usage | % | 67.11 | 67.58 | 67.08 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 82 | 272 | 72 |
| views/all/bins | count | 6480 | 8246 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 151 | 295 | 68 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.3 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2627 | 3164 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8180 | 10.2 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8180 | 10.2 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 1064 | 2493 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1187 | 2834 | 0 |
| views/lamp_face/pipelines | count | 72 | 210 | 0 |
| views/lamp_face/work_items | count | 0 | 2542 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2902 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8438 | 13.1 k | 7592 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8600 | 13.3 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.20 | 351 | 261.3 k |
| anim/events/count | count | 0.31 | 4 | 1630 |
| assets/Image/added | count | 0.06 | 28 | 313 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.77 | 160 | 4107 |
| assets/Mesh/removed | count | 0.92 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 472 |
| assets/StandardMaterial/modified | count | 4.08 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.16 | 1272 | 6184 |
| physics/inserts | count | 0.57 | 89 | 3031 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 161 | 2335 |
| physics/steps | count | 0.74 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6374 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1397 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.3 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.64 | 641.1 | main/busy/ms 603, surface/advance_cpu_ms 70.78, stream/drive/ms 1.57, surface/cell_upload_ms 0.75, stream/tick/ms 0.60, bigspace/total/ms 0.36, anim/evaluate/ms 0.28, weather/map/ms 0.25, bigspace/high_precision/ms 0.17, far/select/ms 0.15, weather/eval/ms 0.04, script/update/ms 0.04 |
| warm-up | 1.10 | 454.4 | main/busy/ms 643, render/thread/ms 416, render/busy/ms 327, render/acquire/ms 83.31, surface/advance_cpu_ms 70.78, render/graph/ms 8.13, render/present/ms 5.76, stream/drive/ms 2.96, stream/tick/ms 1.40, terrain/update/ms 0.86, surface/cell_upload_ms 0.75, bigspace/total/ms 0.55 |
| warm-up | 1.16 | 66.9 | render/thread/ms 439, pipelines/wait_ms 354, render/busy/ms 348, render/acquire/ms 83.56, main/busy/ms 57.40, pipelines/compile_ms 57.22, pipelines/front_ms 51.48, terrain/update/ms 18.23, render/graph/ms 13.45, render/present/ms 6.93, pipelines/driver_ms 5.74, stream/drive/ms 1.86 |
| warm-up | 1.32 | 50.8 | render/thread/ms 70.27, render/busy/ms 58.20, render/graph/ms 34.07, main/busy/ms 22.72, render/acquire/ms 10.36, stream/drive/ms 3.02, stream/tick/ms 2.66, render/present/ms 1.70, terrain/update/ms 0.95, terrain/select/ms 0.74, bigspace/total/ms 0.30, weather/map/ms 0.25 |
| warm-up | 1.49 | 157.5 | pipelines/compile_ms 112, render/thread/ms 111, pipelines/front_ms 103, main/busy/ms 70.48, render/busy/ms 65.07, render/present/ms 45.90, pipelines/wait_ms 45.87, pipelines/driver_ms 9.01, render/graph/ms 3.02, stream/drive/ms 2.95, stream/tick/ms 1.42, terrain/update/ms 0.77 |
| measure | 0.44 | 51.3 | pipelines/wait_ms 6579, pipelines/compile_ms 175, pipelines/front_ms 145, render/thread/ms 57.60, render/busy/ms 55.73, main/busy/ms 54.31, render/graph/ms 46.08, pipelines/driver_ms 30.28, render/present/ms 1.70, bigspace/total/ms 1.45, physics/insert/ms 1.02, bigspace/high_precision/ms 0.80 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **70%** (min 44%) · 2584 MHz (min 2580) · memory 8001 MHz · 70.4 W · 65 °C (max 72) · VRAM 4821 MiB · throttled 100% of samples
