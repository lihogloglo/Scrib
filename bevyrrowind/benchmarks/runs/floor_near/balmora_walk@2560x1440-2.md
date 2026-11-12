# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 8471 frames over 65.6 s · **median 7.71 ms (130 fps)** · mean 7.75 · p95 9.00 · p99 9.94 (1% low 101 fps) · worst 31.37 ms · 13 spikes over 1.5× median · 15.5 stutters/min (work 3.7/min)
- warm-up: 340 frames over 3.5 s, worst 585.0 ms
- hitches (> 25.0 ms): **2** measured, 4 in warm-up
- timed GPU passes: 7.24 ms of a 7.75 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.71 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- pass worst frame 31.37 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.180 | 2.603 | 0.185 |
| main_opaque_pass_3d | 1.346 | 1.900 | 0.020 |
| atmosphere_luts | 0.840 | 1.138 | 0.003 |
| lightprobe_irradiance_map | 0.433 | 0.580 | 0.001 |
| bloom | 0.333 | 0.389 | 0.061 |
| early prepass | 0.330 | 0.581 | 0.035 |
| lightprobe_radiance_map | 0.231 | 0.237 | 0.004 |
| ssao | 0.229 | 0.531 | 0.007 |
| clouds_trace | 0.152 | 0.247 | 0.001 |
| water_fft | 0.140 | 0.141 | 0.002 |
| atmos_froxel_inject | 0.105 | 0.110 | 0.001 |
| tonemapping | 0.088 | 0.090 | 0.003 |
| water_velocity | 0.087 | 0.085 | 0.001 |
| main_transparent_pass_3d | 0.080 | 0.260 | 0.014 |
| upscaling | 0.080 | 0.116 | 0.004 |
| clustering | 0.065 | 0.075 | 0.020 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| uniform_allocation | 0.041 | 0.049 | 0.011 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| clouds_shadow | 0.032 | 0.037 | 0.003 |
| water_mips | 0.029 | 0.027 | 0.002 |
| bin_unpacking | 0.028 | 0.030 | 0.009 |
| per_view_shadow_pass | 0.025 | 0.034 | 0.007 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| early_mesh_preprocessing | 0.023 | 0.024 | 0.005 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.012 | 0.015 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.002 | 0.002 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.998 | 8.215 | 30.69 | 59279.9 |
| render/busy/ms | 4.971 | 6.232 | 29.29 | 42110.3 |
| main/busy/ms | 4.889 | 5.910 | 12.15 | 41418.5 |
| render/graph/ms | 2.972 | 3.982 | 27.16 | 25177.8 |
| render/acquire/ms | 1.567 | 2.687 | 4.07 | 13278.1 |
| render/present/ms | 0.459 | 0.601 | 2.92 | 3891.5 |
| terrain/update/ms | 0.305 | 0.392 | 1.52 | 2587.5 |
| terrain/select/ms | 0.272 | 0.349 | 0.88 | 2303.8 |
| bigspace/total/ms | 0.242 | 0.510 | 1.58 | 2049.5 |
| stream/drive/ms | 0.215 | 0.319 | 2.19 | 1818.9 |
| stream/tick/ms | 0.181 | 0.266 | 0.88 | 1535.9 |
| far/select/ms | 0.172 | 0.256 | 1.04 | 1457.0 |
| bigspace/high_precision/ms | 0.125 | 0.273 | 1.26 | 1060.4 |
| bigspace/low_precision/ms | 0.051 | 0.160 | 1.41 | 429.2 |
| weather/map/ms | 0.026 | 0.035 | 0.51 | 220.5 |
| physics/insert/ms | 0.021 | 0.216 | 0.88 | 174.5 |
| weather/eval/ms | 0.010 | 0.015 | 0.19 | 88.6 |
| physics/step/ms | 0.009 | 0.024 | 0.59 | 76.3 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 55.4 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 41.7 |
| script/ui/ms | 0.004 | 0.007 | 0.30 | 33.3 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.04 | 17.4 |
| controller/move/ms | 0.002 | 0.003 | 0.10 | 17.1 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.67 | 12.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 8.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.09 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.0 |
| anim/apply/ms | 0.001 | 0.001 | 0.07 | 4.5 |
| stream/spawn/ms | 0.000 | 0.000 | 0.22 | 3.4 |
| controller/feed/ms | 0.000 | 0.001 | 0.06 | 2.6 |
| stream/despawn/ms | 0.000 | 0.000 | 0.18 | 2.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.14 | 2.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
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
| ecs/entities | count | 8061 | 8414 | 7946 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.22 | 29.29 | 8.30 |
| gpu/timed/ms | ms | 7.22 | 9.98 | 8.30 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 146 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 326 | 617 | 604 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 146 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 326 | 617 | 604 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 5 | 7 | 6 |
| phases/shadow/bins | count | 85 | 108 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 575 | 615 | 572 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 5 | 7 | 6 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2723 | 3248 | 2329 |
| physics/items | count | 248 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 106 | 106 | 106 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 3 | 38 | 2 |
| preprocess/bind_groups_created | count | 3 | 38 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 51 | 58 | 54 |
| process/cpu_usage | % | 23.67 | 58.65 | 25 |
| process/mem_usage | GiB | 2.14 | 2.18 | 2.15 |
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
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.13 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.88 | 80.55 | 44.55 |
| system/mem_usage | % | 54.35 | 54.61 | 54.02 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 628 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1495 | 2536 | 2502 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1595 | 2745 | 2703 |
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
| views/main/items | count | 902 | 1960 | 1930 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1002 | 2171 | 2131 |
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
| assets/Mesh/added | count | 0.25 | 213 | 2087 |
| assets/Mesh/removed | count | 0.52 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.02 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.28 | 111 | 2376 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.28 | 161 | 2335 |
| physics/steps | count | 0.46 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3996 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 876 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 585.0 | main/busy/ms 563, surface/advance_cpu_ms 65.52, surface/cell_upload_ms 0.70, stream/drive/ms 0.37, weather/map/ms 0.11, bigspace/total/ms 0.08, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.02, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 0.92 | 336.2 | main/busy/ms 596, render/thread/ms 302, render/busy/ms 229, render/acquire/ms 71.36, surface/advance_cpu_ms 65.52, render/graph/ms 4.86, render/present/ms 2.05, stream/drive/ms 1.15, surface/cell_upload_ms 0.70, stream/tick/ms 0.53, terrain/update/ms 0.40, bigspace/total/ms 0.32 |
| warm-up | 0.99 | 68.2 | pipelines/wait_ms 792, render/thread/ms 312, render/busy/ms 238, render/acquire/ms 71.60, pipelines/compile_ms 70.31, pipelines/front_ms 62.69, main/busy/ms 51.15, terrain/update/ms 17.13, render/graph/ms 8.33, pipelines/driver_ms 7.61, render/present/ms 2.89, stream/drive/ms 1.28 |
| warm-up | 1.30 | 86.8 | pipelines/wait_ms 188, pipelines/compile_ms 92.02, pipelines/front_ms 84.93, main/busy/ms 59.68, render/thread/ms 45.64, render/busy/ms 43.95, pipelines/driver_ms 7.10, render/graph/ms 3.12, render/present/ms 1.45, stream/drive/ms 1.07, stream/tick/ms 0.81, terrain/update/ms 0.62 |
| measure | 4.55 | 31.4 | render/thread/ms 37.73, render/busy/ms 35.27, render/graph/ms 31.05, main/busy/ms 10.57, render/acquire/ms 1.45, render/present/ms 1.01, terrain/update/ms 0.61, terrain/select/ms 0.56, stream/drive/ms 0.53, stream/tick/ms 0.47, far/select/ms 0.41, bigspace/total/ms 0.36 |
| measure | 30.49 | 25.2 | main/busy/ms 12.59, render/thread/ms 8.95, render/busy/ms 8.21, render/graph/ms 5.92, far/select/ms 0.72, terrain/update/ms 0.65, render/present/ms 0.64, terrain/select/ms 0.58, stream/drive/ms 0.56, stream/tick/ms 0.50, bigspace/total/ms 0.42, bigspace/high_precision/ms 0.19 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near
- before the run: GPU 1%, CPU 4%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **98%** (min 88%) · 2580 MHz (min 2580) · memory 8001 MHz · 97.5 W · 78 °C (max 81) · VRAM 3935 MiB · throttled 100% of samples
