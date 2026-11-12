# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5154 frames over 65.6 s · **median 12.26 ms (82 fps)** · mean 12.74 · p95 16.20 · p99 19.11 (1% low 52 fps) · worst 28.26 ms · 77 spikes over 1.5× median · 22.8 stutters/min (work 28.3/min)
- warm-up: 444 frames over 5.9 s, worst 673.3 ms
- hitches (> 30.6 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.09 ms of a 12.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.26 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 28.26 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.422 | 3.052 | 0.199 |
| main_opaque_pass_3d | 1.604 | 2.312 | 0.054 |
| early prepass | 0.567 | 1.096 | 0.087 |
| lightprobe_irradiance_map | 0.470 | 0.801 | 0.001 |
| atmosphere_luts | 0.363 | 0.673 | 0.004 |
| bloom | 0.362 | 0.652 | 0.073 |
| ssao | 0.241 | 0.495 | 0.008 |
| lightprobe_radiance_map | 0.233 | 0.237 | 0.005 |
| uniform_allocation | 0.199 | 0.456 | 0.022 |
| auxiliary_views | 0.175 | 0.669 | 0.172 |
| upscaling | 0.139 | 0.299 | 0.005 |
| bin_unpacking | 0.138 | 0.147 | 0.017 |
| water_fft | 0.138 | 0.137 | 0.003 |
| clouds_trace | 0.128 | 0.217 | 0.001 |
| atmos_froxel_inject | 0.105 | 0.112 | 0.002 |
| tonemapping | 0.098 | 0.110 | 0.004 |
| water_velocity | 0.098 | 0.253 | 0.002 |
| clustering | 0.072 | 0.071 | 0.022 |
| clouds_composite | 0.066 | 0.065 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.112 | 0.117 |
| surface_trail | 0.060 | 0.058 | 0.002 |
| render_sky | 0.057 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.050 | 0.050 | 0.006 |
| clouds_resolve | 0.036 | 0.042 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.032 | 0.001 |
| per_view_shadow_pass | 0.025 | 0.024 | 0.008 |
| water_mips | 0.024 | 0.024 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.002 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.966 | 14.226 | 26.48 | 56518.6 |
| render/busy/ms | 10.355 | 13.541 | 25.78 | 53369.4 |
| main/busy/ms | 9.274 | 11.774 | 21.99 | 47796.8 |
| pipelines/wait_ms | 6.072 | 0.000 | 4130.66 | 31295.7 |
| render/graph/ms | 5.229 | 7.076 | 14.80 | 26947.7 |
| render/present/ms | 0.511 | 0.707 | 5.86 | 2631.8 |
| bigspace/total/ms | 0.485 | 0.770 | 3.08 | 2501.3 |
| terrain/update/ms | 0.306 | 0.395 | 1.69 | 1577.7 |
| controller/feed/ms | 0.299 | 0.418 | 0.94 | 1540.8 |
| stream/drive/ms | 0.282 | 0.490 | 7.54 | 1453.1 |
| terrain/select/ms | 0.268 | 0.345 | 1.08 | 1382.4 |
| far/select/ms | 0.239 | 0.406 | 1.18 | 1232.8 |
| stream/tick/ms | 0.226 | 0.395 | 0.72 | 1165.3 |
| pipelines/compile_ms | 0.199 | 0.000 | 179.69 | 1027.0 |
| pipelines/front_ms | 0.192 | 0.000 | 171.64 | 990.6 |
| bigspace/low_precision/ms | 0.186 | 0.314 | 1.98 | 961.2 |
| anim/evaluate_cpu/ms | 0.177 | 0.235 | 0.66 | 912.5 |
| anim/evaluate/ms | 0.155 | 0.252 | 1.22 | 796.6 |
| controller/move/ms | 0.151 | 0.283 | 0.82 | 780.5 |
| bigspace/high_precision/ms | 0.112 | 0.216 | 1.48 | 576.6 |
| render/acquire/ms | 0.100 | 0.140 | 0.79 | 517.4 |
| anim/apply/ms | 0.091 | 0.142 | 1.92 | 469.0 |
| physics/insert/ms | 0.065 | 0.487 | 1.17 | 333.8 |
| physics/step/ms | 0.061 | 0.101 | 0.60 | 313.9 |
| weather/map/ms | 0.028 | 0.036 | 0.30 | 145.7 |
| controller/avoid/ms | 0.012 | 0.018 | 0.50 | 62.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.15 | 60.2 |
| script/update/ms | 0.008 | 0.012 | 0.12 | 41.9 |
| pipelines/driver_ms | 0.007 | 0.000 | 8.05 | 36.5 |
| stream/despawn/ms | 0.007 | 0.000 | 1.40 | 34.9 |
| stream/spawn/ms | 0.006 | 0.000 | 1.14 | 28.6 |
| script/ui/ms | 0.005 | 0.008 | 0.05 | 28.1 |
| script/input/ms | 0.005 | 0.007 | 0.09 | 28.0 |
| controller/write/ms | 0.004 | 0.006 | 0.07 | 19.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.78 | 17.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.08 | 5.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.15 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
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
| frame/work/ms | ms | 9.99 | 25.78 | 9.46 |
| gpu/timed/ms | ms | 8.21 | 13.59 | 9.46 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 808 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 808 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3396 | 5252 | 3171 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3396 | 5252 | 3171 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 40 | 222 | 28 |
| phases/shadow/bins | count | 3737 | 5386 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9518 | 12.7 k | 5673 |
| phases/shadow/pipelines | count | 107 | 248 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 497 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 11 | 0 |
| preprocess/bind_groups | count | 190 | 592 | 164 |
| preprocess/bind_groups_created | count | 190 | 592 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.17 | 35.71 | 26.92 |
| process/mem_usage | GiB | 2.57 | 2.63 | 2.63 |
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
| started_load_count |  | 3492 | 3554 | 3554 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.80 | 74.01 | 33.99 |
| system/mem_usage | % | 67.64 | 68.51 | 68.25 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 262 | 72 |
| views/all/bins | count | 6472 | 8215 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.2 k | 13.2 k |
| views/all/pipelines | count | 149 | 284 | 68 |
| views/all/pipelines_distinct | count | 61 | 69 | 55 |
| views/all/work_items | count | 17.2 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2606 | 3189 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8152 | 10.2 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8152 | 10.2 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 183 | 0 |
| views/lamp_face/bins | count | 1072 | 2442 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1196 | 2810 | 0 |
| views/lamp_face/pipelines | count | 71 | 202 | 0 |
| views/lamp_face/work_items | count | 0 | 2478 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2891 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8402 | 13.3 k | 7576 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8568 | 13.5 k | 7884 |
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
| anim/bones_written/count | count | 52.59 | 231 | 271.1 k |
| anim/events/count | count | 0.31 | 4 | 1616 |
| assets/Image/added | count | 0.06 | 35 | 310 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.80 | 160 | 4100 |
| assets/Mesh/removed | count | 0.95 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 467 |
| assets/StandardMaterial/modified | count | 4.20 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.93 | 1312 | 4814 |
| physics/inserts | count | 0.53 | 84 | 2741 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.76 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 25 | 378 |
| stream/upload/bytes | bytes | 6568 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1440 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.6 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.67 | 673.3 | main/busy/ms 647, surface/advance_cpu_ms 96.40, surface/cell_upload_ms 0.75, stream/drive/ms 0.46, weather/map/ms 0.16, stream/tick/ms 0.16, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 1.01 | 338.1 | main/busy/ms 685, render/thread/ms 299, render/busy/ms 226, surface/advance_cpu_ms 96.40, render/acquire/ms 71.25, render/graph/ms 5.05, render/present/ms 2.03, stream/drive/ms 1.21, surface/cell_upload_ms 0.75, stream/tick/ms 0.63, terrain/update/ms 0.48, terrain/select/ms 0.35 |
| warm-up | 1.09 | 79.7 | pipelines/wait_ms 1047, render/thread/ms 316, render/busy/ms 242, render/acquire/ms 71.52, pipelines/compile_ms 71.52, pipelines/front_ms 65.62, main/busy/ms 54.65, terrain/update/ms 22.82, render/graph/ms 10.21, pipelines/driver_ms 5.89, render/present/ms 3.15, stream/drive/ms 1.17 |
| warm-up | 1.31 | 83.3 | pipelines/compile_ms 193, pipelines/front_ms 171, main/busy/ms 59.06, render/thread/ms 50.11, render/busy/ms 48.62, pipelines/driver_ms 22.16, pipelines/wait_ms 15.94, render/graph/ms 2.83, stream/drive/ms 1.97, stream/tick/ms 1.61, render/present/ms 1.28, terrain/update/ms 0.73 |
| warm-up | 1.52 | 31.8 | pipelines/wait_ms 1917, pipelines/compile_ms 102, pipelines/front_ms 93.85, render/thread/ms 29.74, render/busy/ms 28.56, main/busy/ms 16.01, pipelines/driver_ms 7.86, stream/drive/ms 3.64, render/graph/ms 3.61, stream/spawn/ms 1.51, render/present/ms 1.02, stream/tick/ms 0.74 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\scene_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **77%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.0 W · 68 °C (max 73) · VRAM 4785 MiB · throttled 100% of samples
