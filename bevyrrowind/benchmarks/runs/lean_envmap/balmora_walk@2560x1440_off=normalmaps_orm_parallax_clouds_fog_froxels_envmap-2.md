# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5489 frames over 65.6 s · **median 11.41 ms (88 fps)** · mean 11.96 · p95 15.50 · p99 20.13 (1% low 50 fps) · worst 35.59 ms · 124 spikes over 1.5× median · 44.8 stutters/min (work 45.7/min)
- warm-up: 432 frames over 9.1 s, worst 2513.9 ms
- hitches (> 28.5 ms): **4** measured, 23 in warm-up
- timed GPU passes: 6.77 ms of a 11.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.41 ms, limit 11 ms
- **FAIL** frame_ms: 49 stutters (> 1.5x moving median of 60) in 1.09 min = 44.78/min, limit 1/min
- **FAIL** worst frame 35.59 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.131 | 2.135 | 2.478 | 1.00 | 0.182 |
| main_opaque_pass_3d | 1.388 | 1.390 | 1.820 | 1.00 | 0.067 |
| early prepass | 0.495 | 0.496 | 0.715 | 1.00 | 0.095 |
| bloom | 0.329 | 0.330 | 0.332 | 1.00 | 0.067 |
| atmosphere_luts | 0.320 | 0.321 | 0.384 | 1.00 | 0.004 |
| water_velocity | 0.287 | 0.288 | 0.352 | 1.00 | 0.002 |
| auxiliary_views | 0.226 | 0.226 | 0.868 | 1.00 | 0.211 |
| uniform_allocation | 0.224 | 0.225 | 0.236 | 1.00 | 0.046 |
| water_sspr | 0.217 | 0.218 | 0.215 | 1.00 | 0.003 |
| ssao | 0.201 | 0.201 | 0.204 | 1.00 | 0.008 |
| bin_unpacking | 0.182 | 0.182 | 0.373 | 1.00 | 0.031 |
| upscaling | 0.140 | 0.140 | 0.177 | 1.00 | 0.005 |
| water_fft | 0.136 | 0.137 | 0.138 | 1.00 | 0.003 |
| tonemapping | 0.093 | 0.093 | 0.100 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.087 | 0.151 | 1.00 | 0.009 |
| clustering | 0.069 | 0.069 | 0.070 | 1.00 | 0.020 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| surface_trail | 0.045 | 0.060 | 0.057 | 0.75 | 0.003 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.033 | 0.033 | 0.035 | 1.00 | 0.002 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.023 | 1.00 | 0.079 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.762 | 14.133 | 34.27 | 59072.4 |
| render/busy/ms | 10.195 | 13.513 | 27.50 | 55962.7 |
| main/busy/ms | 9.041 | 11.435 | 25.00 | 49626.1 |
| pipelines/wait_ms | 6.077 | 0.000 | 9671.56 | 33355.7 |
| render/graph/ms | 5.820 | 8.010 | 15.55 | 31943.9 |
| bigspace/total/ms | 0.539 | 1.331 | 3.47 | 2960.2 |
| render/present/ms | 0.473 | 0.602 | 19.20 | 2593.6 |
| terrain/update/ms | 0.306 | 0.409 | 1.92 | 1678.0 |
| controller/feed/ms | 0.291 | 0.408 | 0.88 | 1599.7 |
| terrain/select/ms | 0.271 | 0.361 | 0.92 | 1490.2 |
| pipelines/compile_ms | 0.265 | 0.000 | 139.55 | 1456.9 |
| pipelines/front_ms | 0.256 | 0.000 | 134.04 | 1405.5 |
| stream/drive/ms | 0.254 | 0.371 | 16.89 | 1394.7 |
| bigspace/low_precision/ms | 0.222 | 0.600 | 3.17 | 1216.4 |
| far/select/ms | 0.200 | 0.302 | 0.65 | 1097.2 |
| stream/tick/ms | 0.197 | 0.289 | 0.91 | 1081.9 |
| anim/evaluate_cpu/ms | 0.178 | 0.231 | 0.54 | 977.1 |
| controller/move/ms | 0.169 | 0.421 | 0.88 | 925.1 |
| bigspace/high_precision/ms | 0.155 | 0.570 | 2.48 | 852.4 |
| anim/evaluate/ms | 0.128 | 0.263 | 2.08 | 704.2 |
| render/acquire/ms | 0.094 | 0.131 | 0.42 | 516.2 |
| physics/step/ms | 0.055 | 0.099 | 0.54 | 299.9 |
| physics/insert/ms | 0.040 | 0.405 | 0.94 | 222.0 |
| anim/apply/ms | 0.012 | 0.023 | 0.17 | 66.6 |
| weather/eval/ms | 0.011 | 0.016 | 0.19 | 62.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.09 | 57.9 |
| pipelines/driver_ms | 0.009 | 0.000 | 5.51 | 51.4 |
| script/update/ms | 0.008 | 0.012 | 0.39 | 42.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.70 | 31.6 |
| script/input/ms | 0.005 | 0.008 | 0.16 | 29.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.19 | 26.9 |
| script/ui/ms | 0.004 | 0.007 | 0.05 | 24.1 |
| controller/write/ms | 0.004 | 0.005 | 0.15 | 19.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 16 |
| controller/movers/count | count | 20 | 22 | 16 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1283 | 2262 | 482 |
| crowd/c0/meshes/count | count | 353 | 646 | 119 |
| crowd/c1/count | count | 146 | 156 | 150 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5714 |
| crowd/c1/meshes/count | count | 1203 | 1295 | 1222 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4264 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4243 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.77 | 27.50 | 11.82 |
| gpu/timed/ms | ms | 6.77 | 10.54 | 6.78 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3373 | 5128 | 3178 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3373 | 5128 | 3178 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 316 | 42 |
| phases/shadow/bins | count | 3744 | 5653 | 2234 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9465 | 13.1 k | 5731 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 131 | 337 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2345 |
| physics/items | count | 192 | 476 | 19 |
| physics/pending | count | 0 | 195 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 198 | 203 | 203 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 12 | 801 | 32 |
| preprocess/bind_groups_created | count | 12 | 801 | 32 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 232 | 389 | 195 |
| process/cpu_usage | % | 25 | 39.90 | 22.49 |
| process/mem_usage | GiB | 5.09 | 5.26 | 5.26 |
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
| started_load_count |  | 4752 | 4845 | 4845 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.67 | 99.45 | 30.29 |
| system/mem_usage | % | 64.99 | 65.48 | 65.25 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2921 | 2921 |
| textures/images | count | 2582 | 2640 | 2640 |
| views/all/batch_sets | count | 112 | 368 | 102 |
| views/all/bins | count | 6508 | 8425 | 5714 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 184 | 385 | 98 |
| views/all/pipelines_distinct | count | 84 | 95 | 80 |
| views/all/work_items | count | 17.4 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 71 | 42 |
| views/cascade/bins | count | 2621 | 3153 | 2234 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8227 | 10.4 k | 5731 |
| views/cascade/pipelines | count | 52 | 71 | 42 |
| views/cascade/work_items | count | 8227 | 10.4 k | 5731 |
| views/lamp_face/batch_sets | count | 0 | 259 | 0 |
| views/lamp_face/bins | count | 1055 | 2676 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 3052 | 0 |
| views/lamp_face/pipelines | count | 78 | 268 | 0 |
| views/lamp_face/work_items | count | 0 | 2712 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8344 | 13.0 k | 7590 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8512 | 13.2 k | 7898 |
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
| anim/bones_written/count | count | 56.96 | 299 | 312.7 k |
| anim/events/count | count | 0.10 | 3 | 572 |
| assets/Image/added | count | 0.11 | 41 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.74 | 152 | 4075 |
| assets/Mesh/removed | count | 0.89 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.97 | 20 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.90 | 1379 | 4958 |
| physics/inserts | count | 0.44 | 90 | 2437 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 161 | 2335 |
| physics/steps | count | 0.72 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6167 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1352 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.5 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2513.9 | main/busy/ms 2490, surface/advance_cpu_ms 69.41, surface/cell_upload_ms 0.72, stream/drive/ms 0.36, stream/tick/ms 0.11, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.43 | 918.5 | main/busy/ms 2521, render/thread/ms 885, render/busy/ms 775, render/acquire/ms 108, surface/advance_cpu_ms 69.41, render/graph/ms 5.42, render/present/ms 2.70, stream/drive/ms 1.13, surface/cell_upload_ms 0.72, terrain/update/ms 0.70, stream/tick/ms 0.60, terrain/select/ms 0.48 |
| warm-up | 3.70 | 83.3 | pipelines/compile_ms 224, pipelines/front_ms 203, main/busy/ms 65.75, render/thread/ms 51.65, render/busy/ms 49.41, pipelines/driver_ms 20.63, render/graph/ms 6.62, render/present/ms 1.97, stream/drive/ms 1.22, stream/tick/ms 1.01, terrain/update/ms 0.86, terrain/select/ms 0.71 |
| warm-up | 3.82 | 49.6 | main/busy/ms 49.13, stream/drive/ms 37.29, render/thread/ms 31.94, render/busy/ms 30.34, render/graph/ms 4.28, render/present/ms 1.18, stream/tick/ms 1.05, terrain/update/ms 0.69, terrain/select/ms 0.57, render/acquire/ms 0.42, stream/spawn/ms 0.33, bigspace/total/ms 0.32 |
| warm-up | 3.88 | 31.5 | pipelines/wait_ms 426, render/thread/ms 46.43, render/busy/ms 44.30, main/busy/ms 21.14, render/graph/ms 5.28, stream/drive/ms 3.47, pipelines/compile_ms 2.58, pipelines/front_ms 2.03, stream/tick/ms 1.91, render/present/ms 1.76, terrain/update/ms 0.65, pipelines/driver_ms 0.55 |
| warm-up | 3.92 | 41.3 | pipelines/wait_ms 1039, pipelines/compile_ms 256, pipelines/front_ms 244, render/thread/ms 49.86, render/busy/ms 48.40, main/busy/ms 27.04, pipelines/driver_ms 12.33, stream/drive/ms 8.89, render/graph/ms 3.55, stream/tick/ms 1.86, render/present/ms 1.25, terrain/update/ms 0.85 |
| warm-up | 3.95 | 31.4 | pipelines/wait_ms 2462, pipelines/compile_ms 311, pipelines/front_ms 296, render/thread/ms 49.55, render/busy/ms 47.98, main/busy/ms 26.31, pipelines/driver_ms 15.82, stream/drive/ms 9.87, render/graph/ms 3.53, stream/spawn/ms 1.80, render/present/ms 1.23, stream/tick/ms 0.99 |
| warm-up | 4.04 | 69.2 | render/thread/ms 57.48, render/busy/ms 55.96, main/busy/ms 47.02, stream/drive/ms 22.65, render/graph/ms 3.26, stream/tick/ms 2.35, stream/spawn/ms 2.20, render/present/ms 1.24, terrain/update/ms 1.18, terrain/select/ms 1.01, bigspace/total/ms 0.37, stream/activate/ms 0.30 |
| warm-up | 4.08 | 37.9 | pipelines/wait_ms 1453, pipelines/compile_ms 141, pipelines/front_ms 137, render/thread/ms 64.86, render/busy/ms 63.54, main/busy/ms 54.95, stream/drive/ms 32.37, pipelines/driver_ms 4.32, render/graph/ms 3.33, stream/spawn/ms 2.09, stream/tick/ms 1.77, terrain/update/ms 1.50 |
| warm-up | 4.12 | 39.1 | pipelines/wait_ms 3721, pipelines/compile_ms 182, pipelines/front_ms 175, main/busy/ms 48.21, render/thread/ms 44.21, render/busy/ms 42.65, stream/drive/ms 30.03, pipelines/driver_ms 6.47, render/graph/ms 5.84, render/present/ms 1.33, terrain/update/ms 1.28, stream/spawn/ms 1.21 |
| warm-up | 4.15 | 32.6 | pipelines/wait_ms 2927, pipelines/compile_ms 74.77, pipelines/front_ms 70.62, render/thread/ms 46.16, render/busy/ms 44.06, main/busy/ms 24.78, render/graph/ms 8.88, pipelines/driver_ms 4.15, stream/drive/ms 3.56, render/present/ms 1.72, terrain/update/ms 0.87, stream/tick/ms 0.83 |
| warm-up | 4.20 | 47.6 | pipelines/wait_ms 1801, pipelines/compile_ms 76.91, render/thread/ms 71.08, render/busy/ms 68.30, pipelines/front_ms 62.42, main/busy/ms 32.84, render/graph/ms 20.31, pipelines/driver_ms 14.50, stream/drive/ms 5.19, render/present/ms 2.15, stream/tick/ms 1.43, stream/spawn/ms 1.03 |
| warm-up | 4.30 | 48.1 | pipelines/wait_ms 1298, pipelines/compile_ms 79.37, pipelines/front_ms 74.81, main/busy/ms 64.93, stream/drive/ms 50.03, render/thread/ms 27.82, render/busy/ms 25.49, pipelines/driver_ms 4.55, render/graph/ms 3.16, terrain/update/ms 2.19, render/present/ms 2.17, stream/spawn/ms 2.10 |
| warm-up | 4.36 | 37.7 | pipelines/wait_ms 816, pipelines/compile_ms 82.73, main/busy/ms 81.37, pipelines/front_ms 79.62, stream/drive/ms 59.63, render/thread/ms 48.51, render/busy/ms 45.57, render/graph/ms 8.46, stream/spawn/ms 3.12, pipelines/driver_ms 3.10, render/present/ms 2.56, stream/tick/ms 1.65 |
| warm-up | 4.45 | 89.7 | pipelines/wait_ms 816, main/busy/ms 113, stream/drive/ms 91.87, pipelines/compile_ms 82.73, pipelines/front_ms 79.62, render/thread/ms 68.05, render/busy/ms 65.59, render/graph/ms 7.13, pipelines/driver_ms 3.10, stream/spawn/ms 2.40, render/present/ms 2.14, stream/tick/ms 1.62 |
| warm-up | 4.53 | 79.9 | pipelines/wait_ms 1647, pipelines/compile_ms 201, pipelines/front_ms 136, render/thread/ms 88.48, render/busy/ms 87.30, pipelines/driver_ms 65.17, main/busy/ms 59.82, stream/drive/ms 37.53, render/graph/ms 4.24, stream/spawn/ms 2.20, stream/activate/ms 1.18, terrain/update/ms 1.04 |
| warm-up | 5.03 | 40.3 | pipelines/wait_ms 5856, main/busy/ms 45.53, pipelines/compile_ms 41.68, pipelines/front_ms 38.78, render/thread/ms 21.71, render/busy/ms 20.33, stream/drive/ms 12.68, render/graph/ms 3.94, pipelines/driver_ms 2.90, render/present/ms 1.04, stream/tick/ms 0.79, stream/spawn/ms 0.68 |
| warm-up | 5.72 | 33.0 | pipelines/wait_ms 2960, pipelines/compile_ms 193, pipelines/front_ms 184, main/busy/ms 45.71, render/thread/ms 44.89, render/busy/ms 43.30, stream/drive/ms 22.66, pipelines/driver_ms 9.89, render/graph/ms 6.59, stream/spawn/ms 2.25, stream/tick/ms 1.75, render/present/ms 1.34 |
| warm-up | 5.77 | 43.0 | pipelines/wait_ms 5204, pipelines/compile_ms 249, pipelines/front_ms 237, render/thread/ms 58.14, render/busy/ms 56.70, main/busy/ms 47.03, stream/drive/ms 26.57, pipelines/driver_ms 11.55, render/graph/ms 5.80, stream/spawn/ms 2.04, stream/tick/ms 1.41, render/present/ms 1.23 |
| warm-up | 7.24 | 40.5 | main/busy/ms 52.13, render/thread/ms 33.73, render/busy/ms 32.19, render/graph/ms 11.11, physics/insert/ms 1.63, stream/drive/ms 1.37, render/present/ms 1.23, bigspace/total/ms 0.71, terrain/update/ms 0.60, stream/tick/ms 0.54, terrain/select/ms 0.50, stream/spawn/ms 0.47 |
| warm-up | 7.82 | 35.6 | pipelines/wait_ms 16.0 k, pipelines/compile_ms 62.49, pipelines/front_ms 60.15, main/busy/ms 58.65, render/thread/ms 40.29, render/busy/ms 38.42, render/graph/ms 15.31, pipelines/driver_ms 2.34, render/present/ms 1.62, anim/evaluate_cpu/ms 1.49, bigspace/total/ms 1.22, stream/drive/ms 1.11 |
| warm-up | 7.87 | 55.6 | pipelines/wait_ms 16.5 k, main/busy/ms 77.51, pipelines/compile_ms 55.61, render/thread/ms 54.85, pipelines/front_ms 53.79, render/busy/ms 53.46, render/graph/ms 14.05, pipelines/driver_ms 1.82, anim/evaluate_cpu/ms 1.68, physics/insert/ms 1.21, bigspace/total/ms 1.18, render/present/ms 1.12 |
| warm-up | 7.91 | 31.7 | pipelines/wait_ms 3702, main/busy/ms 50.14, render/thread/ms 46.64, render/busy/ms 45.57, render/graph/ms 10.69, pipelines/compile_ms 7.75, pipelines/front_ms 7.57, physics/insert/ms 1.53, stream/drive/ms 1.08, render/present/ms 0.90, bigspace/total/ms 0.87, physics/step/ms 0.72 |
| measure | 10.31 | 35.6 | render/thread/ms 46.46, render/busy/ms 26.52, main/busy/ms 24.87, render/present/ms 19.75, render/graph/ms 15.15, controller/feed/ms 0.90, bigspace/total/ms 0.88, terrain/update/ms 0.70, terrain/select/ms 0.62, stream/drive/ms 0.57, controller/move/ms 0.51, anim/evaluate_cpu/ms 0.49 |
| measure | 19.27 | 29.4 | render/thread/ms 41.30, render/busy/ms 40.12, render/graph/ms 23.28, main/busy/ms 18.68, render/present/ms 0.97, anim/evaluate/ms 0.91, bigspace/total/ms 0.80, terrain/update/ms 0.62, terrain/select/ms 0.56, controller/feed/ms 0.52, far/select/ms 0.47, anim/evaluate_cpu/ms 0.44 |
| measure | 34.32 | 28.9 | render/thread/ms 48.52, render/busy/ms 44.55, main/busy/ms 34.80, render/graph/ms 27.84, render/present/ms 3.72, bigspace/total/ms 1.51, stream/drive/ms 1.26, stream/tick/ms 1.17, controller/feed/ms 0.92, bigspace/low_precision/ms 0.83, terrain/update/ms 0.81, terrain/select/ms 0.72 |
| measure | 34.36 | 35.6 | render/thread/ms 52.40, render/busy/ms 50.18, main/busy/ms 30.33, render/graph/ms 28.81, render/present/ms 1.90, bigspace/total/ms 1.60, controller/feed/ms 0.96, terrain/update/ms 0.90, bigspace/low_precision/ms 0.86, terrain/select/ms 0.81, stream/drive/ms 0.74, stream/tick/ms 0.65 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **74%** (min 47%) · 2577 MHz (min 2565) · memory 8001 MHz · 74.0 W · 75 °C (max 82) · VRAM 6197 MiB · throttled 100% of samples
