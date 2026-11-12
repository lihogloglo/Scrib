# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 8423 frames over 65.6 s · **median 7.75 ms (129 fps)** · mean 7.79 · p95 8.95 · p99 9.85 (1% low 102 fps) · worst 23.75 ms · 7 spikes over 1.5× median · 9.1 stutters/min (work 5.5/min)
- warm-up: 333 frames over 3.4 s, worst 589.4 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.27 ms of a 7.79 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.75 ms, limit 11 ms
- **FAIL** frame_ms: 10 stutters (> 1.5x moving median of 60) in 1.09 min = 9.14/min, limit 1/min
- pass worst frame 23.75 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.167 | 2.682 | 0.184 |
| main_opaque_pass_3d | 1.366 | 1.960 | 0.019 |
| atmosphere_luts | 0.849 | 1.212 | 0.003 |
| lightprobe_irradiance_map | 0.429 | 0.425 | 0.001 |
| bloom | 0.329 | 0.336 | 0.061 |
| early prepass | 0.326 | 0.561 | 0.035 |
| ssao | 0.247 | 0.562 | 0.007 |
| lightprobe_radiance_map | 0.231 | 0.236 | 0.003 |
| clouds_trace | 0.151 | 0.245 | 0.001 |
| water_fft | 0.149 | 0.474 | 0.002 |
| atmos_froxel_inject | 0.103 | 0.107 | 0.001 |
| upscaling | 0.089 | 0.119 | 0.004 |
| tonemapping | 0.087 | 0.089 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.001 |
| main_transparent_pass_3d | 0.079 | 0.257 | 0.014 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| clustering | 0.064 | 0.066 | 0.020 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| uniform_allocation | 0.041 | 0.050 | 0.011 |
| clouds_resolve | 0.034 | 0.039 | 0.001 |
| clouds_shadow | 0.033 | 0.038 | 0.003 |
| water_mips | 0.031 | 0.028 | 0.002 |
| bin_unpacking | 0.027 | 0.030 | 0.009 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.025 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| early_mesh_preprocessing | 0.023 | 0.024 | 0.005 |
| lightprobe_copy | 0.015 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.013 | 0.013 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 7.040 | 8.172 | 15.69 | 59299.7 |
| render/busy/ms | 4.764 | 5.881 | 12.84 | 40128.1 |
| main/busy/ms | 4.762 | 5.856 | 13.66 | 40107.9 |
| render/graph/ms | 2.779 | 3.597 | 9.73 | 23404.6 |
| render/acquire/ms | 1.824 | 2.909 | 4.75 | 15366.0 |
| render/present/ms | 0.452 | 0.601 | 2.26 | 3805.7 |
| terrain/update/ms | 0.305 | 0.387 | 1.61 | 2567.9 |
| terrain/select/ms | 0.271 | 0.346 | 0.84 | 2286.7 |
| bigspace/total/ms | 0.227 | 0.449 | 1.38 | 1909.9 |
| stream/drive/ms | 0.215 | 0.319 | 2.66 | 1808.9 |
| stream/tick/ms | 0.181 | 0.263 | 0.82 | 1523.8 |
| far/select/ms | 0.169 | 0.242 | 0.64 | 1425.5 |
| bigspace/high_precision/ms | 0.120 | 0.264 | 1.08 | 1009.1 |
| bigspace/low_precision/ms | 0.048 | 0.152 | 0.91 | 405.2 |
| weather/map/ms | 0.025 | 0.034 | 0.35 | 212.1 |
| physics/insert/ms | 0.022 | 0.226 | 0.91 | 184.5 |
| physics/step/ms | 0.012 | 0.029 | 0.53 | 104.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 88.9 |
| script/update/ms | 0.006 | 0.009 | 0.28 | 51.6 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 41.3 |
| script/ui/ms | 0.004 | 0.007 | 0.31 | 30.3 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.11 | 17.3 |
| controller/move/ms | 0.002 | 0.003 | 0.15 | 16.9 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.62 | 12.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.08 | 8.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.39 | 5.0 |
| anim/apply/ms | 0.001 | 0.001 | 0.11 | 4.6 |
| stream/spawn/ms | 0.000 | 0.000 | 0.16 | 3.4 |
| stream/despawn/ms | 0.000 | 0.000 | 0.25 | 2.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.5 |
| controller/feed/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.04 | 0.4 |
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
| frame/work/ms | ms | 7.28 | 13.66 | 7.71 |
| gpu/timed/ms | ms | 7.27 | 9.99 | 7.71 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 328 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 328 | 617 | 603 |
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
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 225 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 106 | 106 | 106 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 3 | 38 | 2 |
| preprocess/bind_groups_created | count | 3 | 38 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 51 | 58 | 54 |
| process/cpu_usage | % | 21.74 | 43.27 | 12.98 |
| process/mem_usage | GiB | 2.11 | 2.15 | 2.11 |
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
| stream/oldest_request/s | s | 0 | 0.13 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.70 | 66.84 | 23.81 |
| system/mem_usage | % | 51.69 | 53.30 | 51.68 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 635 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1505 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1605 | 2745 | 2701 |
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
| views/main/bins | count | 546 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 908 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1011 | 2171 | 2129 |
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
| assets/Mesh/added | count | 0.25 | 195 | 2087 |
| assets/Mesh/removed | count | 0.53 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.02 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.28 | 111 | 2379 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.28 | 161 | 2335 |
| physics/steps | count | 0.47 | 2 | 3938 |
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
| stream/upload/bytes | bytes | 4019 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 15 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 881 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 589.4 | main/busy/ms 567, surface/advance_cpu_ms 68.50, surface/cell_upload_ms 0.70, stream/drive/ms 0.28, weather/map/ms 0.12, bigspace/total/ms 0.11, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.93 | 339.4 | main/busy/ms 603, render/thread/ms 297, render/busy/ms 221, render/acquire/ms 74.06, surface/advance_cpu_ms 68.50, render/graph/ms 4.81, render/present/ms 2.05, stream/drive/ms 0.95, surface/cell_upload_ms 0.70, terrain/update/ms 0.59, stream/tick/ms 0.47, terrain/select/ms 0.46 |
| warm-up | 1.03 | 102.2 | pipelines/wait_ms 2533, render/thread/ms 310, render/busy/ms 232, pipelines/compile_ms 104, pipelines/front_ms 94.08, render/acquire/ms 74.34, main/busy/ms 54.83, terrain/update/ms 21.44, pipelines/driver_ms 9.55, render/graph/ms 8.39, render/present/ms 3.04, stream/drive/ms 1.45 |
| warm-up | 1.11 | 30.2 | render/thread/ms 33.67, render/busy/ms 31.79, render/graph/ms 22.13, main/busy/ms 14.19, terrain/update/ms 6.41, stream/drive/ms 2.38, stream/tick/ms 2.12, render/present/ms 1.52, terrain/select/ms 0.71, render/acquire/ms 0.36, bigspace/total/ms 0.27, weather/map/ms 0.17 |
| warm-up | 1.29 | 92.9 | pipelines/compile_ms 203, pipelines/front_ms 191, main/busy/ms 61.08, render/thread/ms 60.46, render/busy/ms 59.00, pipelines/driver_ms 11.83, render/graph/ms 3.60, terrain/update/ms 1.48, terrain/select/ms 1.33, stream/drive/ms 1.22, render/present/ms 1.11, stream/tick/ms 0.94 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near
- before the run: GPU 0%, CPU 12%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **97%** (min 79%) · 2580 MHz (min 2580) · memory 8001 MHz · 96.3 W · 77 °C (max 80) · VRAM 3791 MiB · throttled 100% of samples
