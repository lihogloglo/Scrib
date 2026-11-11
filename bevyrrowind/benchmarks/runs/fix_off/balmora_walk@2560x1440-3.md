# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5956 frames over 65.6 s · **median 10.71 ms (93 fps)** · mean 11.02 · p95 13.17 · p99 14.65 (1% low 68 fps) · worst 22.89 ms · 23 spikes over 1.5× median · 15.5 stutters/min (work 21.9/min)
- warm-up: 413 frames over 5.3 s, worst 582.0 ms
- hitches (> 26.8 ms): **0** measured, 3 in warm-up
- timed GPU passes: 7.18 ms of a 11.02 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.71 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- pass worst frame 22.89 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.075 | 2.099 | 0.176 |
| main_opaque_pass_3d | 1.393 | 1.745 | 0.044 |
| early prepass | 0.554 | 1.102 | 0.074 |
| lightprobe_irradiance_map | 0.459 | 0.757 | 0.001 |
| bloom | 0.318 | 0.323 | 0.062 |
| atmosphere_luts | 0.308 | 0.368 | 0.003 |
| lightprobe_radiance_map | 0.224 | 0.236 | 0.004 |
| ssao | 0.204 | 0.208 | 0.006 |
| uniform_allocation | 0.169 | 0.182 | 0.015 |
| auxiliary_views | 0.144 | 0.517 | 0.131 |
| clouds_trace | 0.126 | 0.212 | 0.001 |
| water_fft | 0.126 | 0.127 | 0.002 |
| bin_unpacking | 0.124 | 0.142 | 0.012 |
| upscaling | 0.120 | 0.131 | 0.004 |
| atmos_froxel_inject | 0.093 | 0.099 | 0.001 |
| tonemapping | 0.087 | 0.088 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.068 | 0.067 | 0.018 |
| clouds_composite | 0.062 | 0.063 | 0.002 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.103 | 0.103 |
| render_sky | 0.052 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.048 | 0.004 |
| clouds_resolve | 0.034 | 0.036 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.006 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.459 | 11.537 | 21.57 | 56340.3 |
| render/busy/ms | 8.938 | 10.992 | 20.63 | 53236.4 |
| main/busy/ms | 8.228 | 9.924 | 16.42 | 49008.7 |
| render/graph/ms | 4.564 | 5.837 | 11.95 | 27185.1 |
| pipelines/wait_ms | 2.105 | 0.000 | 3906.72 | 12538.3 |
| bigspace/total/ms | 0.443 | 0.776 | 2.77 | 2640.4 |
| render/present/ms | 0.434 | 0.530 | 2.49 | 2584.0 |
| terrain/update/ms | 0.286 | 0.365 | 1.74 | 1705.7 |
| stream/drive/ms | 0.258 | 0.434 | 2.29 | 1534.0 |
| terrain/select/ms | 0.252 | 0.321 | 0.73 | 1498.0 |
| controller/feed/ms | 0.245 | 0.299 | 0.53 | 1457.6 |
| far/select/ms | 0.214 | 0.363 | 0.66 | 1273.8 |
| stream/tick/ms | 0.208 | 0.360 | 0.60 | 1237.6 |
| bigspace/low_precision/ms | 0.175 | 0.309 | 1.82 | 1042.3 |
| anim/evaluate_cpu/ms | 0.169 | 0.211 | 0.50 | 1004.2 |
| pipelines/compile_ms | 0.142 | 0.000 | 170.76 | 842.8 |
| controller/move/ms | 0.139 | 0.288 | 0.60 | 827.2 |
| pipelines/front_ms | 0.136 | 0.000 | 163.57 | 812.6 |
| anim/evaluate/ms | 0.132 | 0.204 | 1.50 | 788.4 |
| bigspace/high_precision/ms | 0.106 | 0.198 | 1.76 | 631.8 |
| render/acquire/ms | 0.087 | 0.114 | 0.36 | 519.9 |
| anim/apply/ms | 0.079 | 0.110 | 1.02 | 467.6 |
| physics/step/ms | 0.048 | 0.085 | 0.69 | 285.8 |
| physics/insert/ms | 0.047 | 0.410 | 0.99 | 279.6 |
| weather/map/ms | 0.026 | 0.034 | 0.28 | 156.9 |
| controller/avoid/ms | 0.011 | 0.017 | 0.11 | 66.5 |
| weather/eval/ms | 0.011 | 0.014 | 0.21 | 62.6 |
| script/update/ms | 0.007 | 0.015 | 0.11 | 42.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.07 | 33.6 |
| pipelines/driver_ms | 0.005 | 0.000 | 7.18 | 30.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 27.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.8 |
| script/ui/ms | 0.004 | 0.005 | 0.05 | 24.1 |
| controller/write/ms | 0.004 | 0.006 | 0.06 | 21.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.83 | 16.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.15 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
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
| frame/work/ms | ms | 8.78 | 20.63 | 8.13 |
| gpu/timed/ms | ms | 7.16 | 9.46 | 8.13 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 818 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 818 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3418 | 5172 | 3180 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3418 | 5172 | 3180 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 39 | 238 | 29 |
| phases/shadow/bins | count | 3791 | 5477 | 2239 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9578 | 12.8 k | 5690 |
| phases/shadow/pipelines | count | 121 | 249 | 29 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 189 | 476 | 19 |
| physics/pending | count | 0 | 540 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 188 | 624 | 166 |
| preprocess/bind_groups_created | count | 188 | 624 | 166 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.52 | 49.76 | 31.25 |
| process/mem_usage | GiB | 2.96 | 3.03 | 3.03 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
| scene/batch/masked/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
| scene/batch/opaque/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/overflow | count | 0 | 0 | 0 |
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
| started_load_count |  | 3479 | 3547 | 3547 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.58 | 45.56 | 29.42 |
| system/mem_usage | % | 43.46 | 43.69 | 43.69 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 278 | 73 |
| views/all/bins | count | 6524 | 8211 | 5719 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 163 | 286 | 69 |
| views/all/pipelines_distinct | count | 60 | 69 | 55 |
| views/all/work_items | count | 17.4 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 29 |
| views/cascade/bins | count | 2612 | 3217 | 2239 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8170 | 10.3 k | 5690 |
| views/cascade/pipelines | count | 36 | 50 | 29 |
| views/cascade/work_items | count | 8170 | 10.3 k | 5690 |
| views/lamp_face/batch_sets | count | 0 | 199 | 0 |
| views/lamp_face/bins | count | 1102 | 2488 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1238 | 2857 | 0 |
| views/lamp_face/pipelines | count | 79 | 202 | 0 |
| views/lamp_face/work_items | count | 0 | 2537 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2894 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8461 | 13.1 k | 7594 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8622 | 13.3 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 60.30 | 307 | 359.1 k |
| anim/events/count | count | 0.27 | 4 | 1589 |
| assets/Image/added | count | 0.05 | 35 | 310 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.69 | 176 | 4100 |
| assets/Mesh/removed | count | 0.82 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 467 |
| assets/StandardMaterial/modified | count | 3.64 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1562 | 4900 |
| physics/inserts | count | 0.47 | 118 | 2791 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5684 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1246 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 582.0 | main/busy/ms 560, surface/advance_cpu_ms 67.39, surface/cell_upload_ms 1.07, stream/drive/ms 0.33, weather/map/ms 0.23, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.99 | 412.1 | main/busy/ms 594, render/thread/ms 377, render/busy/ms 299, render/acquire/ms 75.09, surface/advance_cpu_ms 67.39, render/graph/ms 6.30, render/present/ms 2.97, stream/drive/ms 1.13, surface/cell_upload_ms 1.07, stream/tick/ms 0.60, terrain/update/ms 0.60, terrain/select/ms 0.42 |
| warm-up | 1.24 | 84.9 | pipelines/compile_ms 222, pipelines/front_ms 206, pipelines/wait_ms 179, main/busy/ms 62.25, render/thread/ms 49.42, render/busy/ms 47.66, pipelines/driver_ms 15.88, render/graph/ms 3.67, render/present/ms 1.46, stream/drive/ms 1.37, stream/tick/ms 1.11, terrain/update/ms 0.77 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\fix_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **77%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.7 W · 69 °C (max 74) · VRAM 3977 MiB · throttled 100% of samples
