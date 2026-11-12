# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4423 frames over 65.6 s · **median 14.43 ms (69 fps)** · mean 14.84 · p95 18.04 · p99 21.39 (1% low 47 fps) · worst 30.57 ms · 39 spikes over 1.5× median · 20.1 stutters/min (work 25.6/min)
- warm-up: 369 frames over 6.6 s, worst 637.7 ms
- hitches (> 36.1 ms): **0** measured, 4 in warm-up
- timed GPU passes: 7.78 ms of a 14.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.43 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- pass worst frame 30.57 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.258 | 2.940 | 0.198 |
| main_opaque_pass_3d | 1.488 | 2.265 | 0.055 |
| early prepass | 0.526 | 1.032 | 0.091 |
| lightprobe_irradiance_map | 0.459 | 0.791 | 0.001 |
| bloom | 0.350 | 0.626 | 0.077 |
| atmosphere_luts | 0.338 | 0.588 | 0.003 |
| lightprobe_radiance_map | 0.232 | 0.240 | 0.005 |
| ssao | 0.226 | 0.370 | 0.008 |
| auxiliary_views | 0.201 | 0.697 | 0.201 |
| uniform_allocation | 0.171 | 0.173 | 0.020 |
| water_fft | 0.133 | 0.130 | 0.003 |
| bin_unpacking | 0.132 | 0.135 | 0.016 |
| upscaling | 0.131 | 0.146 | 0.005 |
| clouds_trace | 0.127 | 0.211 | 0.001 |
| atmos_froxel_inject | 0.100 | 0.108 | 0.002 |
| tonemapping | 0.093 | 0.092 | 0.004 |
| water_velocity | 0.089 | 0.086 | 0.002 |
| surface_trail | 0.073 | 0.065 | 0.002 |
| early_mesh_preprocessing | 0.070 | 0.075 | 0.004 |
| clustering | 0.069 | 0.069 | 0.021 |
| clouds_composite | 0.066 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.061 | 0.109 | 0.115 |
| render_sky | 0.055 | 0.054 | 0.002 |
| gpu_instance_materialization | 0.050 | 0.049 | 0.004 |
| per_view_shadow_pass | 0.040 | 0.039 | 0.009 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.031 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| range_unpacking | 0.019 | 0.019 | 0.004 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.964 | 14.792 | 27.60 | 52917.4 |
| render/busy/ms | 11.387 | 14.113 | 26.70 | 50362.6 |
| main/busy/ms | 9.945 | 12.390 | 17.75 | 43985.5 |
| render/graph/ms | 5.191 | 6.792 | 15.51 | 22960.3 |
| pipelines/wait_ms | 4.574 | 0.000 | 5545.13 | 20229.9 |
| bigspace/total/ms | 0.521 | 0.950 | 3.22 | 2303.8 |
| render/present/ms | 0.482 | 0.613 | 3.92 | 2129.9 |
| controller/feed/ms | 0.317 | 0.469 | 1.21 | 1400.1 |
| terrain/update/ms | 0.291 | 0.369 | 1.85 | 1288.8 |
| terrain/select/ms | 0.255 | 0.321 | 0.69 | 1126.0 |
| stream/drive/ms | 0.231 | 0.361 | 1.75 | 1023.2 |
| bigspace/low_precision/ms | 0.184 | 0.316 | 2.87 | 814.9 |
| far/select/ms | 0.184 | 0.261 | 0.58 | 814.7 |
| anim/evaluate_cpu/ms | 0.183 | 0.251 | 0.64 | 810.8 |
| stream/tick/ms | 0.180 | 0.235 | 0.80 | 794.6 |
| anim/evaluate/ms | 0.177 | 0.409 | 2.04 | 782.2 |
| controller/move/ms | 0.133 | 0.255 | 0.64 | 588.1 |
| bigspace/high_precision/ms | 0.122 | 0.268 | 1.49 | 538.7 |
| anim/apply/ms | 0.118 | 0.281 | 1.51 | 521.4 |
| pipelines/compile_ms | 0.104 | 0.000 | 153.12 | 461.5 |
| pipelines/front_ms | 0.101 | 0.000 | 148.00 | 447.3 |
| render/acquire/ms | 0.096 | 0.128 | 0.56 | 424.9 |
| physics/step/ms | 0.065 | 0.095 | 0.60 | 286.9 |
| physics/insert/ms | 0.045 | 0.393 | 0.80 | 198.5 |
| weather/map/ms | 0.028 | 0.037 | 0.37 | 123.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 49.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.30 | 47.1 |
| script/update/ms | 0.007 | 0.011 | 0.12 | 32.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.01 | 26.8 |
| script/ui/ms | 0.006 | 0.009 | 0.04 | 25.6 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 23.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 21.6 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 15.5 |
| pipelines/driver_ms | 0.003 | 0.000 | 5.12 | 14.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 12.6 |
| stream/upload/ms | 0.001 | 0.000 | 0.42 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.05 | 1.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| ecs/entities | count | 70.6 k | 79.8 k | 71.1 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 11.08 | 26.70 | 10.41 |
| gpu/timed/ms | ms | 7.70 | 10.84 | 8.57 |
| mesh_allocator_allocations |  meshes | 13.4 k | 14.7 k | 13.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 323.10 M | 323.10 M | 323.10 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 185 | 401 | 393 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 765 | 1201 | 597 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 185 | 401 | 393 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 765 | 1201 | 597 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 1027 | 1163 | 1096 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 2014 | 2089 | 2089 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 3958 | 5666 | 3549 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1027 | 1163 | 1096 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 2014 | 2089 | 2089 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 3958 | 5666 | 3549 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 51 | 218 | 40 |
| phases/shadow/bins | count | 1974 | 2756 | 1261 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 8040 | 8415 | 8415 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 8211 | 10.6 k | 5236 |
| phases/shadow/pipelines | count | 103 | 229 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 172 | 172 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 192 | 565 | 167 |
| preprocess/bind_groups_created | count | 185 | 558 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 21.67 | 39.09 | 19.71 |
| process/mem_usage | GiB | 2.61 | 2.66 | 2.66 |
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 12.1 k | 12.7 k | 12.7 k |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/batch_capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/slots/batch_capacity_bytes | bytes | 1.05 M | 1.05 M | 1.05 M |
| scene/slots/batch_live | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5348 | 5446 | 5446 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 387 | 481 | 347 |
| statics/materials_bound | count | 387 | 481 | 347 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.05 | 0.10 | 0.07 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 618 | 712 | 541 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.77 | 63.99 | 36.59 |
| system/mem_usage | % | 60.83 | 62.49 | 60.69 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 102 | 264 | 90 |
| views/all/bins | count | 4333 | 5248 | 4239 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 13.4 k | 14.0 k | 14.0 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 18.2 k | 23.7 k | 13.5 k |
| views/all/pipelines | count | 143 | 267 | 68 |
| views/all/pipelines_distinct | count | 61 | 66 | 53 |
| views/all/work_items | count | 99.8 k | 105.1 k | 95.7 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1756 | 2164 | 1261 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 8040 | 8415 | 8415 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 7796 | 9624 | 5236 |
| views/cascade/pipelines | count | 38 | 48 | 30 |
| views/cascade/work_items | count | 56.9 k | 58.8 k | 54.4 k |
| views/lamp_face/batch_sets | count | 4 | 163 | 0 |
| views/lamp_face/bins | count | 243 | 724 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 399 | 1063 | 0 |
| views/lamp_face/pipelines | count | 64 | 181 | 0 |
| views/lamp_face/work_items | count | 40 | 894 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2464 | 2990 | 2978 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 5360 | 5610 | 5610 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 9462 | 13.6 k | 8292 |
| views/main/pipelines | count | 40 | 42 | 38 |
| views/main/work_items | count | 42.4 k | 46.6 k | 41.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 42.7 k | 49.4 k | 42.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 59.92 | 253 | 265.0 k |
| anim/events/count | count | 0.37 | 4 | 1618 |
| assets/Image/added | count | 0.07 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.81 | 261 | 3573 |
| assets/Mesh/removed | count | 1.10 | 1220 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 488 |
| assets/StandardMaterial/modified | count | 4.90 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.64 | 1562 | 7239 |
| physics/inserts | count | 0.54 | 111 | 2370 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.53 | 161 | 2335 |
| physics/steps | count | 0.89 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 5 |
| scene/overflow/cells | count | 0.01 | 1 | 27 |
| scene/slots/overflow | count | 0.67 | 520 | 2950 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 16 | 351 |
| stream/upload/bytes | bytes | 7654 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1678 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 36.1 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.64 | 637.7 | main/busy/ms 613, surface/advance_cpu_ms 87.41, surface/cell_upload_ms 0.73, stream/drive/ms 0.32, weather/map/ms 0.16, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.94 | 302.6 | main/busy/ms 653, render/thread/ms 256, render/busy/ms 184, surface/advance_cpu_ms 87.41, render/acquire/ms 69.75, render/graph/ms 5.94, render/present/ms 2.30, stream/drive/ms 1.31, terrain/update/ms 1.11, terrain/select/ms 0.91, surface/cell_upload_ms 0.73, stream/tick/ms 0.73 |
| warm-up | 1.10 | 163.0 | pipelines/wait_ms 2038, render/thread/ms 280, render/busy/ms 204, pipelines/compile_ms 156, pipelines/front_ms 143, render/acquire/ms 70.14, main/busy/ms 60.66, terrain/update/ms 22.45, pipelines/driver_ms 12.73, render/graph/ms 12.44, render/present/ms 5.18, stream/drive/ms 1.42 |
| warm-up | 1.43 | 90.3 | pipelines/wait_ms 412, main/busy/ms 64.74, pipelines/compile_ms 46.04, render/thread/ms 46.00, render/busy/ms 44.05, pipelines/front_ms 43.72, render/graph/ms 3.36, pipelines/driver_ms 2.32, render/present/ms 1.67, stream/drive/ms 1.50, stream/tick/ms 1.12, terrain/update/ms 0.91 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora_walk@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 19%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **65%** (min 42%) · 2580 MHz (min 2580) · memory 8001 MHz · 63.5 W · 66 °C (max 72) · VRAM 4369 MiB · throttled 100% of samples
