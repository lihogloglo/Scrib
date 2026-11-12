# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9097 frames over 65.6 s · **median 7.21 ms (139 fps)** · mean 7.22 · p95 8.28 · p99 8.94 (1% low 112 fps) · worst 23.68 ms · 15 spikes over 1.5× median · 19.2 stutters/min (work 4.6/min)
- warm-up: 334 frames over 3.3 s, worst 588.6 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 6.74 ms of a 7.22 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.21 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.20/min, limit 1/min
- pass worst frame 23.68 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.217 | 2.717 | 0.179 |
| main_opaque_pass_3d | 1.308 | 1.934 | 0.020 |
| lightprobe_irradiance_map | 0.451 | 0.766 | 0.001 |
| bloom | 0.338 | 0.497 | 0.058 |
| early prepass | 0.337 | 0.590 | 0.034 |
| atmosphere_luts | 0.292 | 0.409 | 0.003 |
| lightprobe_radiance_map | 0.246 | 0.390 | 0.003 |
| ssao | 0.219 | 0.213 | 0.007 |
| clouds_trace | 0.154 | 0.247 | 0.001 |
| water_fft | 0.134 | 0.128 | 0.002 |
| upscaling | 0.107 | 0.137 | 0.004 |
| atmos_froxel_inject | 0.098 | 0.104 | 0.001 |
| tonemapping | 0.093 | 0.091 | 0.003 |
| water_velocity | 0.088 | 0.085 | 0.001 |
| main_transparent_pass_3d | 0.083 | 0.251 | 0.013 |
| clustering | 0.066 | 0.066 | 0.019 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| surface_trail | 0.060 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| uniform_allocation | 0.042 | 0.050 | 0.008 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| bin_unpacking | 0.029 | 0.030 | 0.006 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| early_mesh_preprocessing | 0.024 | 0.024 | 0.005 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.006 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_copy | 0.014 | 0.016 | 0.003 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.002 | 0.003 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.502 | 7.563 | 22.70 | 59151.0 |
| render/busy/ms | 4.874 | 5.475 | 12.34 | 44336.4 |
| main/busy/ms | 4.700 | 5.338 | 11.66 | 42758.1 |
| render/graph/ms | 2.778 | 3.268 | 10.20 | 25275.6 |
| render/acquire/ms | 1.190 | 2.260 | 3.89 | 10825.5 |
| render/present/ms | 0.439 | 0.540 | 14.66 | 3989.1 |
| terrain/update/ms | 0.296 | 0.378 | 2.23 | 2690.4 |
| terrain/select/ms | 0.261 | 0.334 | 0.73 | 2376.7 |
| bigspace/total/ms | 0.239 | 0.533 | 1.40 | 2176.8 |
| stream/drive/ms | 0.215 | 0.304 | 2.04 | 1959.8 |
| stream/tick/ms | 0.182 | 0.250 | 0.69 | 1651.8 |
| far/select/ms | 0.169 | 0.231 | 0.59 | 1536.1 |
| bigspace/high_precision/ms | 0.125 | 0.304 | 1.32 | 1133.2 |
| bigspace/low_precision/ms | 0.051 | 0.170 | 1.17 | 460.3 |
| weather/map/ms | 0.025 | 0.033 | 0.28 | 226.3 |
| physics/insert/ms | 0.020 | 0.184 | 0.91 | 185.2 |
| physics/step/ms | 0.010 | 0.026 | 0.48 | 93.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.29 | 92.0 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 59.6 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 43.2 |
| script/ui/ms | 0.003 | 0.005 | 0.19 | 31.0 |
| controller/move/ms | 0.002 | 0.003 | 0.06 | 18.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.05 | 16.0 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.59 | 12.3 |
| anim/apply/ms | 0.001 | 0.002 | 0.08 | 12.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.11 | 8.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.2 |
| stream/spawn/ms | 0.000 | 0.000 | 0.16 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.7 |
| controller/feed/ms | 0.000 | 0.001 | 0.03 | 2.6 |
| stream/despawn/ms | 0.000 | 0.000 | 0.17 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.05 | 0.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 8059 | 8412 | 7944 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 6.79 | 12.34 | 7.79 |
| gpu/timed/ms | ms | 6.79 | 8.41 | 7.79 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 349.99 M | 351.04 M | 349.99 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 328 | 617 | 603 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 328 | 617 | 603 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/shadow/batch_sets | count | 5 | 7 | 6 |
| phases/shadow/bins | count | 85 | 108 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 575 | 615 | 572 |
| phases/shadow/pipelines | count | 5 | 7 | 6 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2727 | 3248 | 2329 |
| physics/items | count | 239 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 106 | 106 | 106 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 54 | 58 | 56 |
| preprocess/bind_groups_created | count | 54 | 58 | 56 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 22.12 | 34.13 | 17.14 |
| process/mem_usage | GiB | 2.05 | 2.08 | 2.08 |
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
| started_load_count |  | 3012 | 3046 | 3046 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.19 | 45.01 | 25.74 |
| system/mem_usage | % | 48.10 | 48.41 | 48.28 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 633 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1504 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1605 | 2746 | 2701 |
| views/cascade/batch_sets | count | 5 | 7 | 6 |
| views/cascade/bins | count | 85 | 108 | 75 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 575 | 615 | 572 |
| views/cascade/pipelines | count | 5 | 7 | 6 |
| views/cascade/work_items | count | 575 | 615 | 572 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 544 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 908 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1010 | 2172 | 2129 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 3200 | 3212 | 3203 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 125 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.23 | 195 | 2087 |
| assets/Mesh/removed | count | 0.49 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.26 | 111 | 2404 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.26 | 161 | 2335 |
| physics/steps | count | 0.43 | 1 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3721 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 816 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 588.6 | main/busy/ms 566, surface/advance_cpu_ms 69.47, surface/cell_upload_ms 0.73, stream/drive/ms 0.43, weather/map/ms 0.11, bigspace/total/ms 0.10, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.91 | 324.9 | main/busy/ms 599, render/thread/ms 291, render/busy/ms 213, render/acquire/ms 74.92, surface/advance_cpu_ms 69.47, render/graph/ms 5.49, render/present/ms 2.33, stream/drive/ms 1.13, surface/cell_upload_ms 0.73, terrain/update/ms 0.52, stream/tick/ms 0.52, terrain/select/ms 0.35 |
| warm-up | 1.00 | 89.9 | pipelines/wait_ms 313, render/thread/ms 309, render/busy/ms 230, pipelines/compile_ms 77.00, render/acquire/ms 75.37, pipelines/front_ms 72.31, main/busy/ms 50.31, terrain/update/ms 17.99, render/graph/ms 10.47, pipelines/driver_ms 4.69, render/present/ms 3.96, stream/drive/ms 1.11 |
| warm-up | 1.24 | 88.9 | pipelines/compile_ms 182, pipelines/wait_ms 173, pipelines/front_ms 169, main/busy/ms 61.36, render/thread/ms 51.08, render/busy/ms 49.17, pipelines/driver_ms 13.59, render/graph/ms 3.76, stream/drive/ms 1.46, render/present/ms 1.44, stream/tick/ms 1.17, terrain/update/ms 1.00 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora_walk@2560x1440_off=statics-1 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 5%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **97%** (min 82%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.7 W · 78 °C (max 80) · VRAM 3731 MiB · throttled 100% of samples
