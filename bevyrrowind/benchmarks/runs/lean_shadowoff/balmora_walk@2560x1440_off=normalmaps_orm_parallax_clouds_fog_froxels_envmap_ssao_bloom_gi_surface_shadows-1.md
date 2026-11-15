# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6771 frames over 65.6 s · **median 9.37 ms (107 fps)** · mean 9.69 · p95 12.56 · p99 14.80 (1% low 68 fps) · worst 34.73 ms · 112 spikes over 1.5× median · 44.8 stutters/min (work 29.2/min)
- warm-up: 337 frames over 8.8 s, worst 2784.8 ms
- hitches (> 25.0 ms): **6** measured, 36 in warm-up
- timed GPU passes: 5.58 ms of a 9.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.37 ms, limit 11 ms
- **FAIL** frame_ms: 49 stutters (> 1.5x moving median of 60) in 1.09 min = 44.79/min, limit 1/min
- **FAIL** worst frame 34.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.058 | 2.060 | 2.096 | 1.00 | 0.189 |
| main_opaque_pass_3d | 1.200 | 1.201 | 1.617 | 1.00 | 0.064 |
| early prepass | 0.455 | 0.455 | 0.707 | 1.00 | 0.104 |
| atmosphere_luts | 0.313 | 0.314 | 0.373 | 1.00 | 0.004 |
| water_velocity | 0.280 | 0.280 | 0.350 | 1.00 | 0.002 |
| water_sspr | 0.209 | 0.210 | 0.212 | 1.00 | 0.002 |
| upscaling | 0.141 | 0.141 | 0.175 | 1.00 | 0.005 |
| uniform_allocation | 0.136 | 0.137 | 0.375 | 1.00 | 0.025 |
| water_fft | 0.133 | 0.133 | 0.140 | 1.00 | 0.003 |
| auxiliary_views | 0.130 | 0.130 | 0.539 | 1.00 | 0.130 |
| tonemapping | 0.121 | 0.121 | 0.131 | 1.00 | 0.005 |
| bin_unpacking | 0.093 | 0.093 | 0.102 | 1.00 | 0.019 |
| main_transmissive_pass_3d | 0.082 | 0.082 | 0.138 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.070 | 1.00 | 0.022 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.026 | 0.026 | 0.031 | 1.00 | 0.004 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.022 | 1.00 | 0.081 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.409 | 11.052 | 22.48 | 56934.0 |
| main/busy/ms | 8.191 | 10.472 | 27.82 | 55458.0 |
| render/busy/ms | 7.849 | 10.429 | 21.97 | 53144.5 |
| pipelines/wait_ms | 7.169 | 0.000 | 10916.51 | 48538.0 |
| render/graph/ms | 4.515 | 6.216 | 13.10 | 30569.3 |
| bigspace/total/ms | 0.631 | 1.557 | 3.60 | 4275.1 |
| render/present/ms | 0.467 | 0.608 | 2.37 | 3159.0 |
| terrain/update/ms | 0.296 | 0.390 | 1.61 | 2001.8 |
| controller/feed/ms | 0.288 | 0.408 | 0.93 | 1953.0 |
| anim/evaluate/ms | 0.264 | 1.012 | 2.44 | 1789.8 |
| stream/drive/ms | 0.262 | 0.395 | 16.96 | 1774.9 |
| terrain/select/ms | 0.261 | 0.344 | 0.73 | 1767.9 |
| pipelines/compile_ms | 0.226 | 0.000 | 149.67 | 1531.6 |
| pipelines/front_ms | 0.217 | 0.000 | 145.95 | 1469.0 |
| bigspace/low_precision/ms | 0.213 | 0.567 | 2.55 | 1442.3 |
| stream/tick/ms | 0.209 | 0.312 | 0.79 | 1416.3 |
| far/select/ms | 0.198 | 0.291 | 0.63 | 1338.0 |
| bigspace/high_precision/ms | 0.182 | 0.762 | 3.09 | 1230.2 |
| anim/evaluate_cpu/ms | 0.167 | 0.236 | 0.71 | 1131.3 |
| controller/move/ms | 0.124 | 0.283 | 0.49 | 837.1 |
| render/acquire/ms | 0.093 | 0.132 | 0.38 | 630.5 |
| physics/step/ms | 0.044 | 0.096 | 0.61 | 300.6 |
| physics/insert/ms | 0.037 | 0.375 | 1.14 | 249.5 |
| anim/apply/ms | 0.013 | 0.024 | 0.23 | 89.3 |
| weather/eval/ms | 0.012 | 0.017 | 0.23 | 81.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.15 | 69.3 |
| pipelines/driver_ms | 0.009 | 0.000 | 7.25 | 62.6 |
| script/update/ms | 0.008 | 0.013 | 0.08 | 52.8 |
| script/input/ms | 0.005 | 0.008 | 0.08 | 36.0 |
| script/ui/ms | 0.005 | 0.008 | 0.24 | 34.9 |
| stream/despawn/ms | 0.004 | 0.000 | 0.98 | 29.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.08 | 26.9 |
| controller/write/ms | 0.004 | 0.004 | 0.19 | 23.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 16 |
| controller/movers/count | count | 20 | 21 | 16 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1269 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5549 | 5995 | 5613 |
| crowd/c1/meshes/count | count | 1176 | 1297 | 1179 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 136 | 183 | 173 |
| crowd/joints_extracted/count | count | 2821 | 8821 | 4553 |
| crowd/joints_uploaded/count | count | 2792 | 4934 | 4553 |
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
| frame/work/ms | ms | 8.19 | 27.82 | 6.80 |
| gpu/timed/ms | ms | 5.62 | 8.91 | 6.14 |
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
| phases/alpha_mask3d/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 24 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3376 | 5147 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 24 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3376 | 5147 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 234 | 0 |
| phases/shadow/bins | count | 1710 | 2435 | 362 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 1862 | 2808 | 416 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 238 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2345 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 289 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 166 | 169 | 169 |
| pipelines/waiting | count | 0 | 13 | 0 |
| preprocess/bind_groups | count | 6 | 634 | 2 |
| preprocess/bind_groups_created | count | 6 | 634 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 229 | 142 |
| process/cpu_usage | % | 25.51 | 37.02 | 25.96 |
| process/mem_usage | GiB | 4.67 | 4.81 | 4.81 |
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
| started_load_count |  | 4753 | 4844 | 4844 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 41.46 | 67.02 | 47.19 |
| system/mem_usage | % | 73.43 | 74.12 | 74.08 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2921 | 2921 |
| textures/images | count | 2582 | 2640 | 2640 |
| views/all/batch_sets | count | 62 | 286 | 62 |
| views/all/bins | count | 3762 | 5310 | 3480 |
| views/all/count | count | 19 | 49 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 9557 | 15.7 k | 7592 |
| views/all/pipelines | count | 133 | 291 | 58 |
| views/all/pipelines_distinct | count | 60 | 74 | 58 |
| views/all/work_items | count | 8588 | 15.2 k | 7900 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 234 | 0 |
| views/lamp_face/bins | count | 1058 | 2435 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2808 | 0 |
| views/lamp_face/pipelines | count | 78 | 238 | 0 |
| views/lamp_face/work_items | count | 0 | 2532 | 0 |
| views/main/batch_sets | count | 58 | 64 | 62 |
| views/main/bins | count | 2886 | 3560 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8352 | 13.1 k | 7592 |
| views/main/pipelines | count | 54 | 60 | 58 |
| views/main/work_items | count | 8522 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 55.59 | 267 | 376.4 k |
| anim/events/count | count | 0.08 | 3 | 562 |
| assets/Image/added | count | 0.09 | 35 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 7 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.60 | 152 | 4075 |
| assets/Mesh/removed | count | 0.72 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.22 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.70 | 1485 | 4719 |
| physics/inserts | count | 0.37 | 91 | 2525 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.34 | 161 | 2335 |
| physics/steps | count | 0.58 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5000 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1096 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.78 | 2784.8 | main/busy/ms 2755, surface/advance_cpu_ms 68.31, surface/cell_upload_ms 0.89, stream/drive/ms 0.38, stream/tick/ms 0.12, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.54 | 750.7 | main/busy/ms 2791, render/thread/ms 714, render/busy/ms 615, render/acquire/ms 96.77, surface/advance_cpu_ms 68.31, render/graph/ms 6.28, render/present/ms 2.24, stream/drive/ms 1.21, surface/cell_upload_ms 0.89, stream/tick/ms 0.64, terrain/update/ms 0.41, bigspace/total/ms 0.32 |
| warm-up | 3.68 | 148.2 | pipelines/wait_ms 1785, render/thread/ms 724, render/busy/ms 624, pipelines/compile_ms 167, pipelines/front_ms 157, render/acquire/ms 96.98, main/busy/ms 53.37, terrain/update/ms 18.95, pipelines/driver_ms 10.43, render/graph/ms 9.28, render/present/ms 3.36, stream/drive/ms 1.47 |
| warm-up | 3.94 | 92.6 | main/busy/ms 63.37, render/thread/ms 52.27, render/busy/ms 50.39, render/graph/ms 4.40, stream/drive/ms 2.03, stream/tick/ms 1.67, render/present/ms 1.59, terrain/update/ms 0.85, terrain/select/ms 0.71, bigspace/total/ms 0.31, render/acquire/ms 0.29, bigspace/high_precision/ms 0.10 |
| warm-up | 3.97 | 27.8 | pipelines/compile_ms 269, pipelines/front_ms 255, render/thread/ms 33.28, render/busy/ms 31.61, main/busy/ms 22.37, pipelines/driver_ms 14.43, stream/drive/ms 3.98, render/graph/ms 3.37, stream/tick/ms 2.92, terrain/update/ms 1.41, render/present/ms 1.39, terrain/select/ms 1.13 |
| warm-up | 4.13 | 89.5 | main/busy/ms 69.92, render/thread/ms 69.57, render/busy/ms 67.34, stream/drive/ms 53.22, render/graph/ms 6.94, render/present/ms 1.84, stream/tick/ms 1.38, terrain/update/ms 0.97, terrain/select/ms 0.78, stream/spawn/ms 0.70, render/acquire/ms 0.39, bigspace/total/ms 0.36 |
| warm-up | 4.21 | 63.4 | pipelines/wait_ms 1724, pipelines/compile_ms 238, pipelines/front_ms 215, render/thread/ms 46.74, render/busy/ms 44.60, main/busy/ms 31.08, pipelines/driver_ms 22.85, stream/drive/ms 12.59, render/graph/ms 3.95, render/present/ms 1.90, stream/tick/ms 1.43, stream/spawn/ms 1.22 |
| warm-up | 4.30 | 66.6 | pipelines/wait_ms 3091, pipelines/compile_ms 77.64, pipelines/front_ms 71.71, render/thread/ms 63.89, render/busy/ms 61.26, main/busy/ms 43.29, stream/drive/ms 13.78, stream/spawn/ms 9.10, render/graph/ms 6.94, pipelines/driver_ms 5.93, stream/tick/ms 2.40, render/present/ms 2.31 |
| warm-up | 4.35 | 51.2 | pipelines/wait_ms 2217, pipelines/compile_ms 100, pipelines/front_ms 94.91, render/thread/ms 77.27, render/busy/ms 75.14, main/busy/ms 41.27, stream/drive/ms 13.06, stream/spawn/ms 8.16, render/graph/ms 7.01, pipelines/driver_ms 5.22, stream/tick/ms 2.64, render/present/ms 1.83 |
| warm-up | 4.44 | 93.6 | pipelines/wait_ms 1171, render/thread/ms 102, render/busy/ms 63.92, pipelines/compile_ms 48.38, pipelines/front_ms 46.03, render/present/ms 37.38, main/busy/ms 32.66, stream/drive/ms 5.88, render/graph/ms 3.53, pipelines/driver_ms 2.35, stream/spawn/ms 2.10, stream/tick/ms 1.93 |
| warm-up | 4.51 | 46.5 | pipelines/wait_ms 4037, pipelines/compile_ms 173, pipelines/front_ms 146, main/busy/ms 49.27, render/thread/ms 34.18, render/busy/ms 33.03, stream/drive/ms 26.93, pipelines/driver_ms 26.13, render/graph/ms 2.79, stream/spawn/ms 1.03, terrain/update/ms 1.03, render/present/ms 0.97 |
| warm-up | 4.57 | 35.3 | render/thread/ms 37.61, main/busy/ms 35.45, render/busy/ms 34.77, render/graph/ms 10.73, stream/drive/ms 9.68, terrain/update/ms 7.17, render/present/ms 2.52, stream/tick/ms 1.27, bigspace/total/ms 0.99, stream/activate/ms 0.96, terrain/select/ms 0.93, stream/spawn/ms 0.78 |
| warm-up | 4.62 | 31.7 | pipelines/wait_ms 520, pipelines/compile_ms 139, pipelines/front_ms 130, main/busy/ms 64.21, stream/drive/ms 49.64, render/thread/ms 39.15, render/busy/ms 37.46, pipelines/driver_ms 9.80, render/graph/ms 9.01, stream/tick/ms 2.20, terrain/update/ms 2.18, render/present/ms 1.45 |
| warm-up | 4.67 | 50.4 | pipelines/wait_ms 1418, pipelines/compile_ms 189, pipelines/front_ms 178, main/busy/ms 64.67, stream/drive/ms 48.72, render/thread/ms 39.52, render/busy/ms 37.63, pipelines/driver_ms 11.57, render/graph/ms 10.59, render/present/ms 1.66, stream/spawn/ms 1.23, stream/tick/ms 0.88 |
| warm-up | 4.76 | 75.2 | pipelines/wait_ms 3320, pipelines/compile_ms 119, pipelines/front_ms 113, main/busy/ms 69.48, stream/drive/ms 51.00, render/thread/ms 27.47, render/busy/ms 26.28, pipelines/driver_ms 6.46, render/graph/ms 3.37, stream/spawn/ms 2.14, render/present/ms 1.03, terrain/update/ms 0.93 |
| warm-up | 4.82 | 44.9 | pipelines/wait_ms 1040, main/busy/ms 47.73, stream/drive/ms 34.98, render/thread/ms 32.57, render/busy/ms 30.68, pipelines/compile_ms 15.98, pipelines/front_ms 14.92, render/graph/ms 5.52, stream/spawn/ms 1.96, render/present/ms 1.56, pipelines/driver_ms 1.06, terrain/update/ms 0.87 |
| warm-up | 4.86 | 31.9 | main/busy/ms 37.03, stream/drive/ms 25.10, render/thread/ms 20.27, render/busy/ms 18.94, render/graph/ms 4.47, render/present/ms 1.15, stream/activate/ms 1.04, stream/tick/ms 0.85, terrain/update/ms 0.67, stream/spawn/ms 0.58, terrain/select/ms 0.57, physics/insert/ms 0.43 |
| warm-up | 4.96 | 33.4 | pipelines/wait_ms 4339, pipelines/compile_ms 46.92, pipelines/front_ms 43.03, main/busy/ms 35.48, render/thread/ms 31.73, render/busy/ms 29.68, stream/drive/ms 6.14, render/graph/ms 5.42, stream/spawn/ms 4.20, pipelines/driver_ms 3.88, render/present/ms 1.72, terrain/update/ms 1.46 |
| warm-up | 4.98 | 28.4 | pipelines/wait_ms 972, main/busy/ms 38.92, render/thread/ms 35.25, render/busy/ms 33.69, pipelines/compile_ms 24.88, pipelines/front_ms 23.89, stream/drive/ms 4.82, render/graph/ms 3.98, stream/spawn/ms 3.05, terrain/update/ms 1.38, render/present/ms 1.32, physics/insert/ms 1.15 |
| warm-up | 5.02 | 33.2 | pipelines/wait_ms 3579, pipelines/compile_ms 78.60, pipelines/front_ms 71.86, main/busy/ms 36.97, render/thread/ms 35.98, render/busy/ms 34.72, pipelines/driver_ms 6.75, stream/drive/ms 4.00, render/graph/ms 3.11, stream/spawn/ms 2.06, stream/tick/ms 1.14, render/present/ms 1.02 |
| warm-up | 5.14 | 29.9 | render/thread/ms 37.73, render/busy/ms 36.06, main/busy/ms 28.67, stream/drive/ms 12.14, render/graph/ms 4.05, stream/tick/ms 1.73, render/present/ms 1.44, stream/spawn/ms 1.04, bigspace/total/ms 0.75, terrain/update/ms 0.72, terrain/select/ms 0.60, bigspace/high_precision/ms 0.31 |
| warm-up | 5.28 | 42.6 | pipelines/wait_ms 2979, pipelines/compile_ms 64.21, pipelines/front_ms 61.96, render/thread/ms 45.44, render/busy/ms 43.90, main/busy/ms 35.08, render/graph/ms 5.17, stream/drive/ms 2.45, pipelines/driver_ms 2.25, stream/tick/ms 1.37, render/present/ms 1.23, terrain/update/ms 0.96 |
| warm-up | 5.32 | 40.7 | render/thread/ms 62.10, render/busy/ms 60.62, main/busy/ms 49.14, stream/drive/ms 7.52, render/graph/ms 4.96, stream/tick/ms 1.30, render/present/ms 1.24, terrain/update/ms 1.03, terrain/select/ms 0.81, bigspace/total/ms 0.76, bigspace/high_precision/ms 0.29, render/acquire/ms 0.24 |
| warm-up | 5.37 | 45.3 | render/thread/ms 60.61, render/busy/ms 59.14, main/busy/ms 41.16, stream/drive/ms 6.94, render/graph/ms 4.91, render/present/ms 1.24, stream/tick/ms 0.92, terrain/update/ms 0.88, terrain/select/ms 0.70, bigspace/total/ms 0.56, bigspace/high_precision/ms 0.30, render/acquire/ms 0.23 |
| warm-up | 5.61 | 35.4 | pipelines/wait_ms 8761, pipelines/compile_ms 40.14, main/busy/ms 39.19, pipelines/front_ms 37.96, render/thread/ms 27.11, render/busy/ms 25.58, stream/drive/ms 7.65, render/graph/ms 5.50, pipelines/driver_ms 2.18, render/present/ms 1.33, stream/tick/ms 1.30, stream/spawn/ms 1.10 |
| warm-up | 5.65 | 26.1 | main/busy/ms 32.70, render/thread/ms 16.78, render/busy/ms 15.44, stream/drive/ms 15.11, render/graph/ms 4.56, physics/insert/ms 1.19, render/present/ms 1.16, stream/tick/ms 1.07, stream/spawn/ms 0.90, terrain/update/ms 0.66, bigspace/total/ms 0.62, terrain/select/ms 0.57 |
| warm-up | 5.77 | 29.4 | pipelines/wait_ms 8230, pipelines/compile_ms 164, pipelines/front_ms 153, render/thread/ms 35.50, render/busy/ms 34.04, main/busy/ms 26.52, pipelines/driver_ms 10.71, stream/drive/ms 5.49, render/graph/ms 5.07, stream/spawn/ms 2.38, stream/tick/ms 1.73, render/present/ms 1.21 |
| warm-up | 5.86 | 25.8 | main/busy/ms 38.62, render/thread/ms 33.75, render/busy/ms 31.95, stream/drive/ms 8.62, render/graph/ms 7.36, render/present/ms 1.54, bigspace/total/ms 1.25, stream/spawn/ms 1.12, physics/insert/ms 1.10, terrain/update/ms 0.90, stream/tick/ms 0.80, terrain/select/ms 0.73 |
| warm-up | 5.98 | 30.1 | pipelines/wait_ms 5349, pipelines/compile_ms 145, pipelines/front_ms 140, render/thread/ms 35.77, main/busy/ms 34.20, render/busy/ms 33.56, render/graph/ms 7.48, pipelines/driver_ms 5.76, stream/drive/ms 4.16, bigspace/total/ms 2.36, stream/spawn/ms 2.29, render/present/ms 1.86 |
| warm-up | 6.08 | 25.8 | pipelines/wait_ms 1415, pipelines/compile_ms 156, pipelines/front_ms 151, render/thread/ms 31.68, render/busy/ms 30.31, main/busy/ms 28.50, stream/drive/ms 8.88, render/graph/ms 5.30, pipelines/driver_ms 4.27, stream/spawn/ms 2.15, stream/tick/ms 1.38, render/present/ms 1.11 |
| warm-up | 6.26 | 30.3 | pipelines/wait_ms 7987, pipelines/compile_ms 169, pipelines/front_ms 161, render/thread/ms 45.86, render/busy/ms 44.45, main/busy/ms 26.42, pipelines/driver_ms 8.21, render/graph/ms 5.51, stream/drive/ms 3.94, stream/spawn/ms 2.12, render/present/ms 1.14, terrain/update/ms 0.77 |
| warm-up | 6.45 | 37.7 | pipelines/wait_ms 1504, main/busy/ms 37.47, render/thread/ms 35.22, render/busy/ms 33.78, pipelines/compile_ms 17.70, pipelines/front_ms 16.71, render/graph/ms 5.91, stream/drive/ms 5.50, stream/spawn/ms 2.99, stream/tick/ms 1.56, render/present/ms 1.18, pipelines/driver_ms 0.99 |
| warm-up | 7.06 | 25.0 | pipelines/wait_ms 3300, main/busy/ms 39.28, render/thread/ms 27.74, render/busy/ms 25.87, render/graph/ms 10.69, pipelines/compile_ms 4.92, pipelines/front_ms 4.61, stream/drive/ms 1.94, render/present/ms 1.64, stream/tick/ms 1.01, terrain/update/ms 0.78, bigspace/total/ms 0.73 |
| warm-up | 7.09 | 31.1 | main/busy/ms 55.80, render/thread/ms 36.47, render/busy/ms 34.56, render/graph/ms 10.79, stream/drive/ms 1.86, render/present/ms 1.63, bigspace/total/ms 1.37, anim/evaluate_cpu/ms 1.22, stream/tick/ms 0.98, bigspace/low_precision/ms 0.78, terrain/update/ms 0.72, terrain/select/ms 0.63 |
| warm-up | 7.13 | 39.6 | main/busy/ms 50.08, render/thread/ms 38.59, render/busy/ms 36.96, render/graph/ms 9.85, stream/drive/ms 1.67, render/present/ms 1.37, bigspace/total/ms 1.32, stream/tick/ms 0.80, anim/evaluate_cpu/ms 0.76, bigspace/low_precision/ms 0.74, terrain/update/ms 0.72, terrain/select/ms 0.62 |
| warm-up | 8.35 | 30.2 | main/busy/ms 34.76, render/thread/ms 25.25, render/busy/ms 23.93, render/graph/ms 9.89, stream/drive/ms 1.20, render/present/ms 1.08, bigspace/total/ms 0.96, physics/insert/ms 0.70, anim/evaluate_cpu/ms 0.69, controller/feed/ms 0.58, stream/tick/ms 0.58, terrain/update/ms 0.54 |
| measure | 0.79 | 34.7 | main/busy/ms 41.88, render/thread/ms 30.86, render/busy/ms 29.38, stream/drive/ms 16.71, render/graph/ms 11.89, physics/insert/ms 1.50, render/present/ms 1.22, bigspace/total/ms 1.03, terrain/update/ms 0.85, far/select/ms 0.77, terrain/select/ms 0.76, controller/feed/ms 0.75 |
| measure | 0.87 | 26.0 | pipelines/wait_ms 4102, pipelines/compile_ms 150, pipelines/front_ms 146, main/busy/ms 30.87, render/thread/ms 28.55, render/busy/ms 27.30, render/graph/ms 10.38, pipelines/driver_ms 3.72, stream/drive/ms 1.25, render/present/ms 1.04, stream/tick/ms 0.88, bigspace/total/ms 0.77 |
| measure | 18.17 | 28.4 | main/busy/ms 34.34, render/thread/ms 19.24, render/busy/ms 18.11, stream/drive/ms 17.18, render/graph/ms 9.92, render/present/ms 0.98, bigspace/total/ms 0.74, controller/feed/ms 0.62, far/select/ms 0.60, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/tick/ms 0.38 |
| measure | 19.27 | 25.4 | render/thread/ms 30.31, render/busy/ms 29.29, main/busy/ms 18.54, render/graph/ms 17.14, anim/evaluate/ms 1.77, render/present/ms 0.85, bigspace/total/ms 0.77, physics/insert/ms 0.67, terrain/update/ms 0.57, controller/feed/ms 0.54, terrain/select/ms 0.51, stream/drive/ms 0.50 |
| measure | 23.02 | 25.2 | render/thread/ms 31.90, render/busy/ms 30.73, main/busy/ms 28.90, render/graph/ms 10.98, stream/drive/ms 1.03, render/present/ms 0.96, controller/feed/ms 0.94, bigspace/total/ms 0.94, anim/evaluate_cpu/ms 0.59, anim/evaluate/ms 0.58, far/select/ms 0.54, terrain/update/ms 0.54 |
| measure | 59.05 | 25.5 | render/thread/ms 31.60, render/busy/ms 30.46, main/busy/ms 18.02, render/graph/ms 7.27, bigspace/total/ms 1.13, render/present/ms 0.96, terrain/update/ms 0.71, far/select/ms 0.71, terrain/select/ms 0.62, controller/feed/ms 0.56, stream/drive/ms 0.49, bigspace/low_precision/ms 0.45 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **68%** (min 46%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.2 W · 73 °C (max 80) · VRAM 5971 MiB · throttled 100% of samples
