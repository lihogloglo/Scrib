# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9150 frames over 65.6 s · **median 7.16 ms (140 fps)** · mean 7.17 · p95 8.25 · p99 8.99 (1% low 111 fps) · worst 21.83 ms · 15 spikes over 1.5× median · 16.5 stutters/min (work 2.7/min)
- warm-up: 709 frames over 10.1 s, worst 3405.3 ms
- hitches (> 25.0 ms): **0** measured, 17 in warm-up
- timed GPU passes: 6.68 ms of a 7.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.16 ms, limit 11 ms
- **FAIL** frame_ms: 18 stutters (> 1.5x moving median of 60) in 1.09 min = 16.45/min, limit 1/min
- pass worst frame 21.83 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.195 | 2.568 | 0.183 |
| main_opaque_pass_3d | 1.302 | 1.876 | 0.018 |
| lightprobe_irradiance_map | 0.436 | 0.588 | 0.001 |
| bloom | 0.337 | 0.504 | 0.057 |
| early prepass | 0.327 | 0.560 | 0.032 |
| atmosphere_luts | 0.317 | 0.474 | 0.003 |
| lightprobe_radiance_map | 0.236 | 0.241 | 0.003 |
| ssao | 0.219 | 0.218 | 0.006 |
| clouds_trace | 0.157 | 0.256 | 0.001 |
| water_fft | 0.135 | 0.131 | 0.002 |
| atmos_froxel_inject | 0.102 | 0.109 | 0.001 |
| tonemapping | 0.089 | 0.090 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.001 |
| upscaling | 0.084 | 0.117 | 0.004 |
| main_transparent_pass_3d | 0.079 | 0.275 | 0.013 |
| clustering | 0.066 | 0.074 | 0.019 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| uniform_allocation | 0.041 | 0.048 | 0.007 |
| clouds_resolve | 0.035 | 0.042 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| bin_unpacking | 0.029 | 0.030 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| early_mesh_preprocessing | 0.024 | 0.025 | 0.004 |
| per_view_shadow_pass | 0.023 | 0.031 | 0.006 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.015 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.011 | 0.001 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.453 | 7.514 | 20.67 | 59048.4 |
| render/busy/ms | 4.788 | 5.656 | 10.50 | 43814.0 |
| main/busy/ms | 4.665 | 5.352 | 11.30 | 42688.8 |
| render/graph/ms | 2.744 | 3.520 | 5.77 | 25109.1 |
| render/acquire/ms | 1.228 | 2.283 | 3.51 | 11231.9 |
| render/present/ms | 0.437 | 0.533 | 14.72 | 4002.5 |
| terrain/update/ms | 0.296 | 0.378 | 1.62 | 2711.3 |
| terrain/select/ms | 0.261 | 0.333 | 0.73 | 2390.7 |
| bigspace/total/ms | 0.239 | 0.512 | 1.43 | 2188.1 |
| stream/drive/ms | 0.213 | 0.314 | 2.52 | 1952.3 |
| stream/tick/ms | 0.179 | 0.259 | 0.64 | 1640.9 |
| far/select/ms | 0.166 | 0.226 | 0.54 | 1517.5 |
| bigspace/high_precision/ms | 0.125 | 0.299 | 1.35 | 1147.5 |
| bigspace/low_precision/ms | 0.051 | 0.181 | 1.13 | 471.2 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 229.7 |
| physics/insert/ms | 0.016 | 0.000 | 0.92 | 150.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.34 | 95.1 |
| physics/step/ms | 0.010 | 0.026 | 0.60 | 91.9 |
| script/update/ms | 0.006 | 0.008 | 0.18 | 54.3 |
| script/input/ms | 0.005 | 0.006 | 0.29 | 43.4 |
| script/ui/ms | 0.003 | 0.005 | 0.21 | 30.2 |
| controller/move/ms | 0.002 | 0.003 | 0.10 | 18.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.07 | 16.1 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.75 | 12.2 |
| anim/apply/ms | 0.001 | 0.002 | 0.17 | 12.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.17 | 9.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| stream/spawn/ms | 0.000 | 0.000 | 0.27 | 3.5 |
| stream/despawn/ms | 0.000 | 0.000 | 0.25 | 2.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.5 |
| controller/feed/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| ecs/entities | count | 8060 | 8412 | 7945 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 6.70 | 11.30 | 7.49 |
| gpu/timed/ms | ms | 6.69 | 8.04 | 7.49 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 327 | 617 | 603 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 327 | 617 | 603 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/shadow/batch_sets | count | 5 | 7 | 6 |
| phases/shadow/bins | count | 85 | 108 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 575 | 615 | 572 |
| phases/shadow/pipelines | count | 5 | 7 | 6 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2724 | 3248 | 2329 |
| physics/items | count | 255 | 476 | 19 |
| physics/pending | count | 0 | 313 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 101 | 101 | 101 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 54 | 58 | 56 |
| preprocess/bind_groups_created | count | 54 | 58 | 56 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 23.08 | 45.67 | 16.36 |
| process/mem_usage | GiB | 2.14 | 2.18 | 2.16 |
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
| started_load_count |  | 3012 | 3048 | 3048 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 2.33 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.90 | 62.15 | 25.41 |
| system/mem_usage | % | 48.63 | 48.95 | 48.78 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 629 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1497 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1598 | 2745 | 2701 |
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
| views/main/bins | count | 540 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 904 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1004 | 2171 | 2129 |
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
| assets/Image/added | count | 0.01 | 11 | 125 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.23 | 84 | 2087 |
| assets/Mesh/removed | count | 0.48 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 16 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.24 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.26 | 161 | 2335 |
| physics/steps | count | 0.43 | 1 | 3939 |
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
| stream/upload/bytes | bytes | 3700 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 811 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.60 | 599.9 | main/busy/ms 577, surface/advance_cpu_ms 66.49, surface/cell_upload_ms 0.75, stream/drive/ms 0.27, weather/map/ms 0.13, bigspace/total/ms 0.08, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 1.11 | 505.2 | main/busy/ms 616, render/thread/ms 467, render/busy/ms 374, render/acquire/ms 90.14, surface/advance_cpu_ms 66.49, render/graph/ms 6.61, render/present/ms 2.18, stream/drive/ms 1.10, surface/cell_upload_ms 0.75, stream/tick/ms 0.58, terrain/update/ms 0.55, terrain/select/ms 0.37 |
| warm-up | 1.17 | 63.8 | pipelines/wait_ms 1024, render/thread/ms 486, render/busy/ms 393, render/acquire/ms 90.37, pipelines/compile_ms 82.84, pipelines/front_ms 76.66, main/busy/ms 54.76, terrain/update/ms 24.31, render/graph/ms 10.49, pipelines/driver_ms 6.18, render/present/ms 3.09, stream/drive/ms 1.25 |
| warm-up | 1.23 | 25.3 | render/thread/ms 41.15, render/acquire/ms 23.24, main/busy/ms 17.76, render/busy/ms 16.09, terrain/update/ms 12.65, render/graph/ms 5.84, render/present/ms 1.82, stream/drive/ms 1.47, stream/tick/ms 1.24, terrain/select/ms 0.69, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.10 |
| warm-up | 1.26 | 26.7 | render/thread/ms 49.70, render/acquire/ms 37.82, main/busy/ms 11.85, render/busy/ms 10.71, terrain/update/ms 5.81, render/graph/ms 3.58, stream/drive/ms 1.20, render/present/ms 1.17, stream/tick/ms 1.00, terrain/select/ms 0.62, bigspace/total/ms 0.14, weather/map/ms 0.09 |
| warm-up | 1.28 | 29.0 | render/thread/ms 53.60, render/acquire/ms 41.98, main/busy/ms 12.84, render/busy/ms 10.38, render/graph/ms 3.68, render/present/ms 1.24, terrain/update/ms 1.15, stream/drive/ms 1.07, stream/tick/ms 0.86, terrain/select/ms 0.52, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.12 |
| warm-up | 1.31 | 26.4 | render/thread/ms 53.63, render/acquire/ms 39.41, main/busy/ms 16.55, render/busy/ms 12.79, render/graph/ms 3.60, stream/drive/ms 1.65, render/present/ms 1.44, stream/tick/ms 1.25, terrain/update/ms 1.18, terrain/select/ms 0.55, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.12 |
| warm-up | 1.34 | 28.3 | render/thread/ms 52.51, render/busy/ms 27.58, main/busy/ms 26.23, render/acquire/ms 23.32, terrain/update/ms 4.43, render/graph/ms 3.97, stream/drive/ms 3.01, stream/tick/ms 2.13, render/present/ms 1.61, bigspace/total/ms 0.68, terrain/select/ms 0.58, stream/spawn/ms 0.38 |
| warm-up | 5.02 | 3405.3 | pipelines/wait_ms 54.0 k, main/busy/ms 3434, stream/drive/ms 3371, stream/activate/ms 3368, pipelines/compile_ms 802, pipelines/front_ms 750, pipelines/driver_ms 52.65, render/thread/ms 35.99, render/busy/ms 34.50, render/graph/ms 6.31, terrain/update/ms 3.22, render/present/ms 1.26 |
| warm-up | 5.09 | 65.6 | pipelines/wait_ms 54.0 k, pipelines/compile_ms 803, pipelines/front_ms 750, main/busy/ms 70.90, render/thread/ms 69.14, render/busy/ms 67.63, pipelines/driver_ms 53.14, render/graph/ms 7.50, terrain/update/ms 7.03, stream/drive/ms 2.11, render/present/ms 1.28, stream/tick/ms 1.03 |
| warm-up | 5.13 | 39.7 | render/thread/ms 40.42, pipelines/compile_ms 32.23, pipelines/wait_ms 30.57, pipelines/front_ms 27.02, render/acquire/ms 23.30, main/busy/ms 18.95, render/busy/ms 15.47, render/graph/ms 7.19, terrain/update/ms 6.87, pipelines/driver_ms 5.20, stream/drive/ms 3.39, render/present/ms 1.65 |
| warm-up | 5.16 | 26.8 | render/thread/ms 59.60, render/acquire/ms 40.63, pipelines/compile_ms 32.23, pipelines/wait_ms 30.57, pipelines/front_ms 27.02, render/busy/ms 17.76, main/busy/ms 12.42, render/graph/ms 7.54, terrain/update/ms 5.74, pipelines/driver_ms 5.20, stream/drive/ms 1.71, render/present/ms 1.21 |
| warm-up | 5.19 | 34.5 | render/thread/ms 59.68, render/acquire/ms 44.30, render/busy/ms 14.33, main/busy/ms 10.74, render/graph/ms 5.86, terrain/update/ms 1.83, stream/drive/ms 1.49, render/present/ms 1.05, stream/tick/ms 0.74, terrain/select/ms 0.47, physics/insert/ms 0.34, bigspace/total/ms 0.16 |
| warm-up | 5.23 | 37.5 | pipelines/compile_ms 118, pipelines/front_ms 111, pipelines/wait_ms 111, render/thread/ms 70.50, render/acquire/ms 57.71, render/busy/ms 11.91, main/busy/ms 10.20, pipelines/driver_ms 7.06, render/graph/ms 4.87, stream/drive/ms 1.25, render/present/ms 0.87, stream/tick/ms 0.70 |
| warm-up | 5.27 | 34.1 | pipelines/compile_ms 118, pipelines/front_ms 111, pipelines/wait_ms 111, render/thread/ms 70.07, render/acquire/ms 58.69, render/busy/ms 10.59, main/busy/ms 9.74, pipelines/driver_ms 7.06, render/graph/ms 4.59, stream/drive/ms 1.18, render/present/ms 0.79, stream/tick/ms 0.68 |
| warm-up | 5.30 | 35.4 | render/thread/ms 67.80, render/acquire/ms 56.73, render/busy/ms 10.32, main/busy/ms 9.53, render/graph/ms 4.62, stream/drive/ms 1.56, render/present/ms 0.75, stream/tick/ms 0.74, terrain/update/ms 0.56, terrain/select/ms 0.49, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.07 |
| warm-up | 5.33 | 33.1 | render/thread/ms 66.80, render/acquire/ms 56.14, render/busy/ms 9.90, main/busy/ms 8.84, render/graph/ms 4.31, stream/drive/ms 1.55, render/present/ms 0.77, stream/tick/ms 0.72, terrain/update/ms 0.54, terrain/select/ms 0.47, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.08 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora_walk@2560x1440_off=statics-3 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **97%** (min 85%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.8 W · 78 °C (max 81) · VRAM 3468 MiB · throttled 100% of samples
