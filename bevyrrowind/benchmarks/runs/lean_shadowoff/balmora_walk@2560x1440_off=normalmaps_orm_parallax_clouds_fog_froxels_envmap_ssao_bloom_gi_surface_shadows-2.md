# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6813 frames over 65.6 s · **median 9.37 ms (107 fps)** · mean 9.64 · p95 12.37 · p99 14.33 (1% low 70 fps) · worst 46.61 ms · 83 spikes over 1.5× median · 40.2 stutters/min (work 36.6/min)
- warm-up: 350 frames over 8.5 s, worst 2745.0 ms
- hitches (> 25.0 ms): **7** measured, 30 in warm-up
- timed GPU passes: 5.61 ms of a 9.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.37 ms, limit 11 ms
- **FAIL** frame_ms: 44 stutters (> 1.5x moving median of 60) in 1.09 min = 40.22/min, limit 1/min
- **FAIL** worst frame 46.61 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.058 | 2.060 | 2.090 | 1.00 | 0.187 |
| main_opaque_pass_3d | 1.202 | 1.203 | 1.616 | 1.00 | 0.060 |
| early prepass | 0.458 | 0.458 | 0.835 | 1.00 | 0.104 |
| atmosphere_luts | 0.316 | 0.316 | 0.376 | 1.00 | 0.004 |
| water_velocity | 0.281 | 0.281 | 0.350 | 1.00 | 0.002 |
| water_sspr | 0.210 | 0.210 | 0.211 | 1.00 | 0.003 |
| auxiliary_views | 0.149 | 0.149 | 0.551 | 1.00 | 0.145 |
| upscaling | 0.142 | 0.142 | 0.179 | 1.00 | 0.005 |
| uniform_allocation | 0.140 | 0.140 | 0.381 | 1.00 | 0.024 |
| water_fft | 0.133 | 0.133 | 0.140 | 1.00 | 0.003 |
| tonemapping | 0.121 | 0.121 | 0.131 | 1.00 | 0.005 |
| bin_unpacking | 0.097 | 0.097 | 0.104 | 1.00 | 0.019 |
| main_transmissive_pass_3d | 0.081 | 0.081 | 0.137 | 1.00 | 0.009 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| water_snapshot | 0.030 | 0.030 | 0.033 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.026 | 0.026 | 0.031 | 1.00 | 0.004 |
| water_mips | 0.022 | 0.023 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.022 | 1.00 | 0.081 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.391 | 10.972 | 44.91 | 57167.6 |
| main/busy/ms | 8.035 | 10.271 | 29.55 | 54741.4 |
| render/busy/ms | 7.826 | 10.361 | 27.02 | 53315.3 |
| pipelines/wait_ms | 6.106 | 0.000 | 12315.93 | 41599.7 |
| render/graph/ms | 4.506 | 6.281 | 17.94 | 30697.3 |
| bigspace/total/ms | 0.650 | 1.571 | 3.20 | 4429.7 |
| render/present/ms | 0.472 | 0.605 | 37.51 | 3217.5 |
| terrain/update/ms | 0.297 | 0.389 | 2.55 | 2022.1 |
| controller/feed/ms | 0.286 | 0.407 | 0.91 | 1945.7 |
| terrain/select/ms | 0.262 | 0.343 | 0.72 | 1787.0 |
| stream/drive/ms | 0.261 | 0.384 | 21.30 | 1775.1 |
| bigspace/low_precision/ms | 0.232 | 0.768 | 2.90 | 1581.9 |
| pipelines/compile_ms | 0.221 | 0.000 | 148.70 | 1503.2 |
| pipelines/front_ms | 0.212 | 0.000 | 142.19 | 1446.3 |
| stream/tick/ms | 0.208 | 0.304 | 0.94 | 1416.0 |
| far/select/ms | 0.197 | 0.289 | 0.52 | 1343.1 |
| bigspace/high_precision/ms | 0.193 | 0.876 | 2.58 | 1312.6 |
| anim/evaluate/ms | 0.181 | 0.546 | 2.02 | 1234.4 |
| anim/evaluate_cpu/ms | 0.180 | 0.244 | 0.65 | 1225.8 |
| controller/move/ms | 0.124 | 0.222 | 0.70 | 841.4 |
| render/acquire/ms | 0.093 | 0.131 | 0.36 | 634.8 |
| physics/step/ms | 0.044 | 0.095 | 0.56 | 300.3 |
| physics/insert/ms | 0.037 | 0.383 | 1.03 | 248.8 |
| anim/apply/ms | 0.013 | 0.023 | 0.30 | 86.7 |
| weather/eval/ms | 0.012 | 0.018 | 0.19 | 81.4 |
| controller/avoid/ms | 0.010 | 0.013 | 0.07 | 68.5 |
| pipelines/driver_ms | 0.008 | 0.000 | 6.65 | 56.9 |
| script/update/ms | 0.008 | 0.012 | 0.25 | 51.5 |
| script/ui/ms | 0.005 | 0.008 | 0.31 | 36.0 |
| script/input/ms | 0.005 | 0.008 | 0.17 | 35.7 |
| stream/despawn/ms | 0.004 | 0.000 | 1.37 | 30.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 27.9 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 24.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.15 | 2.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
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
| anim/evaluated/count | count | 19 | 28 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6037 | 5613 |
| crowd/c1/meshes/count | count | 1165 | 1311 | 1179 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 136 | 184 | 173 |
| crowd/joints_extracted/count | count | 2802 | 8821 | 4553 |
| crowd/joints_uploaded/count | count | 2754 | 4934 | 4553 |
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
| frame/work/ms | ms | 8.09 | 29.55 | 7.07 |
| gpu/timed/ms | ms | 5.64 | 8.88 | 5.84 |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 24 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3363 | 5115 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 24 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3363 | 5115 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 245 | 8 |
| phases/shadow/bins | count | 1698 | 2413 | 377 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 1909 | 2781 | 431 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 245 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 459 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 170 | 173 | 173 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 6 | 656 | 6 |
| preprocess/bind_groups_created | count | 6 | 656 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 136 | 237 | 138 |
| process/cpu_usage | % | 25 | 40 | 22.77 |
| process/mem_usage | GiB | 4.65 | 4.79 | 4.79 |
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
| started_load_count |  | 4752 | 4843 | 4843 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 41.28 | 70.58 | 43.28 |
| system/mem_usage | % | 74.07 | 74.67 | 74.45 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2921 | 2921 |
| textures/images | count | 2582 | 2640 | 2640 |
| views/all/batch_sets | count | 62 | 297 | 62 |
| views/all/bins | count | 3730 | 5268 | 3480 |
| views/all/count | count | 19 | 49 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 9367 | 15.5 k | 7592 |
| views/all/pipelines | count | 137 | 293 | 58 |
| views/all/pipelines_distinct | count | 60 | 72 | 58 |
| views/all/work_items | count | 8633 | 15.2 k | 7900 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 245 | 0 |
| views/lamp_face/bins | count | 1101 | 2413 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1224 | 2781 | 0 |
| views/lamp_face/pipelines | count | 83 | 245 | 0 |
| views/lamp_face/work_items | count | 0 | 2606 | 0 |
| views/main/batch_sets | count | 58 | 64 | 62 |
| views/main/bins | count | 2870 | 3560 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8328 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 60 | 58 |
| views/main/work_items | count | 8497 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 57.50 | 271 | 391.7 k |
| anim/events/count | count | 0.08 | 3 | 574 |
| assets/Image/added | count | 0.09 | 32 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 7 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.60 | 152 | 4075 |
| assets/Mesh/removed | count | 0.72 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.20 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.69 | 1485 | 4719 |
| physics/inserts | count | 0.40 | 109 | 2694 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.34 | 129 | 2335 |
| physics/steps | count | 0.58 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 4969 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1089 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.74 | 2745.0 | main/busy/ms 2719, surface/advance_cpu_ms 77.21, surface/cell_upload_ms 0.78, stream/drive/ms 0.27, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, far/select/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.68 | 933.1 | main/busy/ms 2753, render/thread/ms 898, render/busy/ms 740, render/acquire/ms 156, surface/advance_cpu_ms 77.21, render/graph/ms 5.73, render/present/ms 2.57, stream/drive/ms 1.01, surface/cell_upload_ms 0.78, terrain/update/ms 0.55, stream/tick/ms 0.53, terrain/select/ms 0.41 |
| warm-up | 3.97 | 93.5 | pipelines/compile_ms 234, pipelines/front_ms 225, main/busy/ms 78.64, render/thread/ms 74.94, render/busy/ms 63.03, render/acquire/ms 10.63, pipelines/driver_ms 8.69, render/graph/ms 3.62, stream/drive/ms 2.44, stream/tick/ms 2.00, render/present/ms 1.28, terrain/update/ms 0.86 |
| warm-up | 4.10 | 73.5 | pipelines/wait_ms 994, pipelines/compile_ms 188, pipelines/front_ms 175, main/busy/ms 53.27, stream/drive/ms 37.67, render/thread/ms 33.69, render/busy/ms 32.22, pipelines/driver_ms 12.68, render/graph/ms 4.11, render/present/ms 1.29, stream/tick/ms 0.96, terrain/update/ms 0.71 |
| warm-up | 4.13 | 26.3 | pipelines/wait_ms 1394, pipelines/compile_ms 228, pipelines/front_ms 210, render/thread/ms 48.54, render/busy/ms 46.53, main/busy/ms 46.37, stream/drive/ms 28.73, pipelines/driver_ms 18.23, render/graph/ms 5.33, render/present/ms 1.79, stream/tick/ms 1.12, terrain/update/ms 1.08 |
| warm-up | 4.20 | 72.3 | pipelines/wait_ms 3412, pipelines/compile_ms 116, pipelines/front_ms 103, render/thread/ms 66.72, render/busy/ms 64.35, main/busy/ms 49.62, stream/drive/ms 30.86, pipelines/driver_ms 13.07, render/graph/ms 7.13, stream/tick/ms 2.04, render/present/ms 2.00, stream/spawn/ms 1.47 |
| warm-up | 4.30 | 73.2 | pipelines/wait_ms 2333, pipelines/compile_ms 65.78, pipelines/front_ms 60.11, render/thread/ms 59.73, render/busy/ms 57.70, main/busy/ms 38.24, pipelines/driver_ms 5.67, stream/drive/ms 5.13, render/graph/ms 4.44, stream/spawn/ms 2.25, render/present/ms 1.78, stream/tick/ms 1.40 |
| warm-up | 4.34 | 45.4 | pipelines/wait_ms 1848, render/thread/ms 77.01, render/busy/ms 75.22, pipelines/compile_ms 37.92, main/busy/ms 37.67, pipelines/front_ms 34.73, terrain/update/ms 8.66, render/graph/ms 4.60, stream/drive/ms 4.44, pipelines/driver_ms 3.19, render/present/ms 1.60, stream/tick/ms 1.48 |
| warm-up | 4.38 | 36.7 | render/thread/ms 62.87, render/busy/ms 61.80, main/busy/ms 26.98, terrain/update/ms 8.40, stream/drive/ms 4.55, render/graph/ms 3.55, stream/tick/ms 1.39, stream/spawn/ms 1.32, terrain/select/ms 0.91, render/present/ms 0.90, stream/activate/ms 0.80, physics/insert/ms 0.30 |
| warm-up | 4.46 | 65.9 | pipelines/wait_ms 2055, pipelines/compile_ms 152, pipelines/front_ms 147, main/busy/ms 53.13, render/thread/ms 33.20, render/busy/ms 31.12, stream/drive/ms 30.70, pipelines/driver_ms 5.49, render/graph/ms 3.49, stream/spawn/ms 3.01, stream/tick/ms 1.58, render/present/ms 1.54 |
| warm-up | 4.50 | 44.7 | pipelines/wait_ms 1773, render/thread/ms 55.67, render/busy/ms 52.86, main/busy/ms 44.41, stream/drive/ms 21.53, pipelines/compile_ms 20.88, pipelines/front_ms 19.92, render/graph/ms 7.02, stream/tick/ms 2.22, render/present/ms 2.06, terrain/update/ms 1.28, stream/spawn/ms 1.18 |
| warm-up | 4.56 | 57.6 | pipelines/wait_ms 988, pipelines/compile_ms 166, pipelines/front_ms 157, render/thread/ms 61.30, render/busy/ms 59.36, main/busy/ms 35.68, stream/drive/ms 19.42, pipelines/driver_ms 9.00, render/graph/ms 7.99, render/present/ms 1.53, stream/tick/ms 1.40, terrain/update/ms 1.33 |
| warm-up | 4.59 | 32.1 | pipelines/wait_ms 988, pipelines/compile_ms 166, pipelines/front_ms 157, render/thread/ms 57.16, render/busy/ms 55.52, main/busy/ms 51.72, stream/drive/ms 39.87, render/graph/ms 17.04, pipelines/driver_ms 9.00, render/present/ms 1.47, stream/spawn/ms 1.16, terrain/update/ms 0.89 |
| warm-up | 4.63 | 42.9 | pipelines/wait_ms 850, pipelines/compile_ms 69.35, pipelines/front_ms 63.02, main/busy/ms 54.75, render/thread/ms 47.37, render/busy/ms 45.47, stream/drive/ms 40.70, render/graph/ms 19.63, pipelines/driver_ms 6.34, stream/spawn/ms 2.26, render/present/ms 1.67, stream/tick/ms 0.85 |
| warm-up | 4.71 | 69.4 | pipelines/wait_ms 1607, main/busy/ms 70.98, pipelines/compile_ms 58.77, pipelines/front_ms 56.06, stream/drive/ms 49.24, render/thread/ms 26.88, render/busy/ms 25.84, render/graph/ms 3.08, pipelines/driver_ms 2.71, stream/spawn/ms 2.30, terrain/update/ms 0.94, render/present/ms 0.87 |
| warm-up | 4.78 | 53.9 | main/busy/ms 49.30, stream/drive/ms 34.33, render/thread/ms 26.81, render/busy/ms 25.36, render/graph/ms 4.81, stream/spawn/ms 2.12, render/present/ms 1.24, stream/tick/ms 0.91, terrain/update/ms 0.73, terrain/select/ms 0.58, stream/activate/ms 0.51, bigspace/total/ms 0.35 |
| warm-up | 4.82 | 31.4 | pipelines/wait_ms 690, pipelines/compile_ms 119, pipelines/front_ms 111, main/busy/ms 38.18, stream/drive/ms 24.70, render/thread/ms 19.19, render/busy/ms 17.62, pipelines/driver_ms 8.11, render/graph/ms 4.75, render/present/ms 1.32, stream/activate/ms 1.09, stream/spawn/ms 1.06 |
| warm-up | 4.93 | 28.0 | render/thread/ms 31.65, render/busy/ms 30.16, main/busy/ms 25.57, stream/drive/ms 5.69, render/graph/ms 4.14, stream/spawn/ms 3.04, stream/tick/ms 1.57, render/present/ms 1.20, terrain/update/ms 1.02, terrain/select/ms 0.84, bigspace/total/ms 0.41, render/acquire/ms 0.30 |
| warm-up | 5.10 | 36.8 | pipelines/wait_ms 9521, pipelines/compile_ms 171, pipelines/front_ms 162, render/thread/ms 36.16, render/busy/ms 34.49, main/busy/ms 28.64, stream/drive/ms 11.81, pipelines/driver_ms 9.49, render/graph/ms 4.69, render/present/ms 1.37, stream/spawn/ms 1.01, stream/tick/ms 0.71 |
| warm-up | 5.24 | 27.6 | render/thread/ms 38.48, main/busy/ms 38.33, render/busy/ms 37.43, stream/drive/ms 6.56, render/graph/ms 3.71, physics/insert/ms 1.06, render/present/ms 0.84, stream/tick/ms 0.81, terrain/update/ms 0.77, terrain/select/ms 0.66, bigspace/total/ms 0.51, physics/step/ms 0.32 |
| warm-up | 5.35 | 26.7 | main/busy/ms 33.74, render/thread/ms 18.38, stream/drive/ms 17.44, render/busy/ms 17.19, render/graph/ms 4.28, render/present/ms 0.96, stream/tick/ms 0.90, terrain/update/ms 0.78, stream/spawn/ms 0.68, terrain/select/ms 0.67, bigspace/total/ms 0.48, bigspace/high_precision/ms 0.27 |
| warm-up | 5.42 | 26.4 | pipelines/wait_ms 910, pipelines/compile_ms 154, pipelines/front_ms 148, main/busy/ms 36.30, render/thread/ms 34.70, render/busy/ms 31.58, render/graph/ms 7.01, pipelines/driver_ms 5.92, stream/drive/ms 2.18, render/acquire/ms 1.81, render/present/ms 1.30, bigspace/total/ms 1.17 |
| warm-up | 5.56 | 28.0 | pipelines/wait_ms 4700, pipelines/compile_ms 157, pipelines/front_ms 153, main/busy/ms 33.54, stream/drive/ms 19.39, render/thread/ms 16.62, render/busy/ms 15.45, pipelines/driver_ms 4.89, render/graph/ms 3.82, stream/spawn/ms 1.21, render/present/ms 0.99, stream/tick/ms 0.90 |
| warm-up | 5.86 | 29.7 | render/thread/ms 36.58, render/busy/ms 34.58, main/busy/ms 29.52, render/graph/ms 5.65, stream/drive/ms 4.49, stream/spawn/ms 2.69, render/present/ms 1.69, bigspace/total/ms 1.13, stream/tick/ms 0.98, terrain/update/ms 0.76, terrain/select/ms 0.66, bigspace/high_precision/ms 0.54 |
| warm-up | 5.91 | 34.1 | main/busy/ms 31.24, render/thread/ms 23.92, render/busy/ms 22.78, stream/drive/ms 13.43, render/graph/ms 3.26, stream/spawn/ms 2.06, render/present/ms 0.97, terrain/update/ms 0.92, terrain/select/ms 0.77, stream/tick/ms 0.70, bigspace/total/ms 0.59, bigspace/high_precision/ms 0.26 |
| warm-up | 6.76 | 35.7 | pipelines/wait_ms 8272, pipelines/compile_ms 55.40, main/busy/ms 51.85, pipelines/front_ms 50.33, render/thread/ms 24.98, render/busy/ms 21.01, render/graph/ms 6.89, pipelines/driver_ms 5.08, render/present/ms 3.66, physics/insert/ms 1.87, stream/drive/ms 1.28, terrain/update/ms 0.66 |
| warm-up | 6.80 | 37.1 | pipelines/wait_ms 11.4 k, pipelines/compile_ms 44.30, pipelines/front_ms 41.12, main/busy/ms 36.65, render/thread/ms 33.99, render/busy/ms 32.33, render/graph/ms 5.53, pipelines/driver_ms 3.18, physics/insert/ms 1.72, render/present/ms 1.46, stream/drive/ms 1.18, bigspace/total/ms 0.66 |
| warm-up | 7.14 | 27.9 | pipelines/wait_ms 7149, pipelines/compile_ms 48.42, pipelines/front_ms 45.88, main/busy/ms 45.82, render/thread/ms 30.96, render/busy/ms 28.75, render/graph/ms 12.33, stream/drive/ms 2.69, pipelines/driver_ms 2.54, render/present/ms 1.94, physics/insert/ms 1.61, bigspace/total/ms 1.18 |
| warm-up | 7.19 | 46.6 | pipelines/wait_ms 8678, main/busy/ms 74.68, pipelines/compile_ms 68.19, pipelines/front_ms 60.31, render/thread/ms 44.24, render/busy/ms 42.45, render/graph/ms 9.12, pipelines/driver_ms 7.88, terrain/update/ms 2.92, stream/drive/ms 2.26, anim/evaluate_cpu/ms 1.69, physics/insert/ms 1.55 |
| warm-up | 7.25 | 59.4 | pipelines/wait_ms 4066, main/busy/ms 62.03, render/thread/ms 53.23, render/busy/ms 51.73, pipelines/compile_ms 33.47, pipelines/front_ms 27.58, render/graph/ms 6.65, pipelines/driver_ms 5.89, terrain/update/ms 5.10, stream/drive/ms 1.53, anim/evaluate_cpu/ms 1.39, physics/insert/ms 1.23 |
| measure | 0.38 | 28.1 | render/thread/ms 32.97, render/busy/ms 31.69, render/graph/ms 15.48, main/busy/ms 15.44, render/present/ms 1.11, bigspace/total/ms 0.81, physics/insert/ms 0.77, terrain/update/ms 0.53, terrain/select/ms 0.48, controller/feed/ms 0.47, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.36 |
| measure | 0.78 | 26.3 | main/busy/ms 33.03, render/thread/ms 24.57, render/busy/ms 23.43, stream/drive/ms 13.18, render/graph/ms 10.56, render/present/ms 0.96, bigspace/total/ms 0.90, physics/insert/ms 0.86, far/select/ms 0.75, terrain/update/ms 0.57, controller/feed/ms 0.54, terrain/select/ms 0.50 |
| measure | 2.75 | 46.6 | render/thread/ms 64.89, render/present/ms 38.16, render/busy/ms 26.55, main/busy/ms 16.64, render/graph/ms 14.69, bigspace/total/ms 0.73, physics/insert/ms 0.58, stream/drive/ms 0.57, terrain/update/ms 0.54, stream/tick/ms 0.51, terrain/select/ms 0.49, controller/feed/ms 0.41 |
| measure | 18.17 | 31.9 | main/busy/ms 39.83, stream/drive/ms 21.59, render/thread/ms 19.68, render/busy/ms 18.58, render/graph/ms 10.41, render/present/ms 0.92, bigspace/total/ms 0.78, controller/feed/ms 0.66, terrain/update/ms 0.60, terrain/select/ms 0.55, far/select/ms 0.51, stream/tick/ms 0.37 |
| measure | 19.27 | 29.2 | render/thread/ms 39.21, render/busy/ms 37.73, render/graph/ms 22.24, main/busy/ms 21.07, render/present/ms 1.19, bigspace/total/ms 0.95, controller/feed/ms 0.70, stream/drive/ms 0.69, terrain/update/ms 0.69, physics/insert/ms 0.68, stream/tick/ms 0.61, terrain/select/ms 0.61 |
| measure | 23.01 | 26.3 | render/thread/ms 32.81, render/busy/ms 31.44, main/busy/ms 31.03, render/graph/ms 11.39, bigspace/total/ms 1.80, controller/feed/ms 1.17, stream/drive/ms 0.98, render/present/ms 0.97, anim/evaluate_cpu/ms 0.68, terrain/update/ms 0.61, bigspace/low_precision/ms 0.54, terrain/select/ms 0.54 |
| measure | 59.01 | 27.7 | render/thread/ms 32.22, render/busy/ms 30.06, main/busy/ms 29.65, render/graph/ms 16.73, stream/drive/ms 2.08, render/present/ms 1.84, bigspace/total/ms 1.51, controller/feed/ms 1.22, stream/tick/ms 1.11, stream/spawn/ms 0.85, terrain/update/ms 0.80, terrain/select/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **69%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.3 W · 73 °C (max 80) · VRAM 5964 MiB · throttled 100% of samples
