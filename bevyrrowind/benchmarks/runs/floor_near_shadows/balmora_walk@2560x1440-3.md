# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,shadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9428 frames over 65.6 s · **median 6.98 ms (143 fps)** · mean 6.96 · p95 7.98 · p99 8.42 (1% low 119 fps) · worst 9.84 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.9/min)
- warm-up: 325 frames over 3.5 s, worst 579.7 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 6.95 ms of a 6.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 6.98 ms, limit 11 ms
- pass frame_ms: 0 stutters (> 1.5x moving median of 60) in 1.09 min = 0.00/min, limit 1/min
- pass worst frame 9.84 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.172 | 2.470 | 0.179 |
| main_opaque_pass_3d | 1.249 | 1.813 | 0.020 |
| atmosphere_luts | 0.753 | 1.084 | 0.003 |
| lightprobe_irradiance_map | 0.429 | 0.558 | 0.001 |
| early prepass | 0.332 | 0.582 | 0.034 |
| bloom | 0.329 | 0.337 | 0.057 |
| lightprobe_radiance_map | 0.232 | 0.240 | 0.003 |
| ssao | 0.209 | 0.217 | 0.006 |
| clouds_trace | 0.150 | 0.246 | 0.001 |
| water_fft | 0.134 | 0.129 | 0.002 |
| upscaling | 0.103 | 0.119 | 0.004 |
| tonemapping | 0.090 | 0.093 | 0.003 |
| water_velocity | 0.087 | 0.086 | 0.001 |
| main_transparent_pass_3d | 0.075 | 0.224 | 0.013 |
| atmos_froxel_inject | 0.074 | 0.073 | 0.001 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| clustering | 0.063 | 0.073 | 0.019 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| clouds_resolve | 0.036 | 0.042 | 0.001 |
| clouds_shadow | 0.031 | 0.035 | 0.003 |
| uniform_allocation | 0.031 | 0.041 | 0.007 |
| atmos_froxel_integrate | 0.026 | 0.031 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| bin_unpacking | 0.023 | 0.024 | 0.005 |
| lightprobe_copy | 0.015 | 0.016 | 0.003 |
| early_mesh_preprocessing | 0.015 | 0.015 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.012 | 0.011 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.002 | 0.001 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.261 | 7.282 | 14.49 | 59024.7 |
| main/busy/ms | 4.435 | 5.184 | 11.79 | 41808.9 |
| render/busy/ms | 4.308 | 5.232 | 8.94 | 40612.8 |
| render/graph/ms | 2.544 | 3.374 | 6.18 | 23986.5 |
| render/acquire/ms | 1.515 | 2.649 | 4.58 | 14280.2 |
| render/present/ms | 0.438 | 0.543 | 2.66 | 4131.7 |
| terrain/update/ms | 0.297 | 0.377 | 1.71 | 2802.0 |
| terrain/select/ms | 0.265 | 0.336 | 0.93 | 2501.2 |
| bigspace/total/ms | 0.221 | 0.469 | 1.42 | 2087.4 |
| stream/drive/ms | 0.211 | 0.303 | 2.12 | 1990.1 |
| stream/tick/ms | 0.178 | 0.249 | 0.54 | 1682.8 |
| far/select/ms | 0.164 | 0.220 | 0.57 | 1542.7 |
| bigspace/high_precision/ms | 0.118 | 0.267 | 1.33 | 1112.4 |
| bigspace/low_precision/ms | 0.047 | 0.148 | 0.96 | 443.7 |
| weather/map/ms | 0.025 | 0.034 | 0.27 | 235.3 |
| physics/insert/ms | 0.020 | 0.166 | 0.88 | 184.2 |
| weather/eval/ms | 0.010 | 0.015 | 0.14 | 97.7 |
| physics/step/ms | 0.010 | 0.026 | 0.59 | 92.6 |
| script/update/ms | 0.006 | 0.008 | 0.32 | 56.7 |
| script/input/ms | 0.005 | 0.006 | 0.24 | 44.4 |
| script/ui/ms | 0.003 | 0.006 | 0.27 | 32.3 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.19 | 19.2 |
| controller/move/ms | 0.002 | 0.003 | 0.12 | 18.3 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.61 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 8.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.4 |
| anim/apply/ms | 0.001 | 0.001 | 0.04 | 5.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.32 | 3.7 |
| weather/apply/ms | 0.000 | 0.000 | 0.03 | 2.6 |
| stream/despawn/ms | 0.000 | 0.000 | 0.19 | 2.5 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 2.4 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.01 | 0.4 |
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
| frame/work/ms | ms | 7.00 | 11.79 | 7.44 |
| gpu/timed/ms | ms | 7.00 | 8.57 | 7.44 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 307.48 M | 308.53 M | 307.48 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 123 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 123 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 123 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 123 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 145 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 325 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 145 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 325 | 617 | 603 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 0 | 0 |
| phases/shadow/bins | count | 0 | 0 | 0 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 0 | 0 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 0 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2727 | 3248 | 2329 |
| physics/items | count | 255 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 100 | 100 | 100 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 35 | 2 |
| preprocess/bind_groups_created | count | 2 | 35 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 37 | 41 | 37 |
| process/cpu_usage | % | 21.15 | 39.82 | 15.38 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.43 | 49.90 | 24.76 |
| system/mem_usage | % | 50.77 | 50.99 | 50.78 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 12 | 12 | 12 |
| views/all/bins | count | 538 | 1586 | 1560 |
| views/all/count | count | 1 | 3 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 894 | 1960 | 1928 |
| views/all/pipelines | count | 8 | 8 | 8 |
| views/all/pipelines_distinct | count | 8 | 8 | 8 |
| views/all/work_items | count | 995 | 2171 | 2129 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 2 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
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
| views/main/bins | count | 538 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 894 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 995 | 2171 | 2129 |
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
| assets/Image/modified | count | 0 | 0 | 0 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.22 | 195 | 2087 |
| assets/Mesh/removed | count | 0.47 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.25 | 111 | 2392 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 145 | 2335 |
| physics/steps | count | 0.42 | 1 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3591 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 787 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.7 | main/busy/ms 558, surface/advance_cpu_ms 65.27, surface/cell_upload_ms 0.76, stream/drive/ms 0.28, weather/map/ms 0.09, bigspace/total/ms 0.08, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.93 | 347.6 | main/busy/ms 596, render/thread/ms 313, render/busy/ms 223, render/acquire/ms 87.63, surface/advance_cpu_ms 65.27, render/graph/ms 18.47, render/present/ms 2.21, stream/drive/ms 0.91, surface/cell_upload_ms 0.76, terrain/update/ms 0.49, stream/tick/ms 0.44, terrain/select/ms 0.36 |
| warm-up | 1.13 | 201.2 | pipelines/wait_ms 3497, render/thread/ms 326, render/busy/ms 235, pipelines/compile_ms 226, pipelines/front_ms 208, render/acquire/ms 87.89, main/busy/ms 55.92, terrain/update/ms 22.66, render/graph/ms 21.99, pipelines/driver_ms 18.08, render/present/ms 3.15, stream/drive/ms 1.46 |
| warm-up | 1.24 | 27.1 | pipelines/compile_ms 89.86, pipelines/front_ms 82.51, pipelines/wait_ms 81.27, render/thread/ms 45.34, render/acquire/ms 21.52, render/busy/ms 20.91, main/busy/ms 18.68, pipelines/driver_ms 7.35, render/graph/ms 4.39, stream/drive/ms 3.35, render/present/ms 2.91, stream/tick/ms 2.10 |
| warm-up | 1.37 | 89.1 | main/busy/ms 70.81, render/thread/ms 69.25, render/busy/ms 67.02, terrain/update/ms 6.69, render/graph/ms 4.19, stream/drive/ms 3.90, stream/tick/ms 2.73, render/present/ms 1.86, terrain/select/ms 0.73, bigspace/total/ms 0.44, render/acquire/ms 0.37, bigspace/high_precision/ms 0.21 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_shadows\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,shadows
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **98%** (min 84%) · 2580 MHz (min 2580) · memory 8001 MHz · 99.3 W · 80 °C (max 83) · VRAM 3671 MiB · throttled 100% of samples
