# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5768 frames over 65.6 s · **median 10.99 ms (91 fps)** · mean 11.38 · p95 14.46 · p99 19.03 (1% low 53 fps) · worst 33.32 ms · 141 spikes over 1.5× median · 43.9 stutters/min (work 40.2/min)
- warm-up: 329 frames over 8.5 s, worst 2732.4 ms
- hitches (> 27.5 ms): **5** measured, 27 in warm-up
- timed GPU passes: 5.51 ms of a 11.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.99 ms, limit 11 ms
- **FAIL** frame_ms: 48 stutters (> 1.5x moving median of 60) in 1.09 min = 43.87/min, limit 1/min
- **FAIL** worst frame 33.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.056 | 2.057 | 2.086 | 1.00 | 0.188 |
| main_opaque_pass_3d | 0.965 | 0.965 | 1.369 | 1.00 | 0.067 |
| early prepass | 0.418 | 0.418 | 0.641 | 1.00 | 0.099 |
| atmosphere_luts | 0.313 | 0.313 | 0.373 | 1.00 | 0.004 |
| water_velocity | 0.280 | 0.281 | 0.350 | 1.00 | 0.002 |
| uniform_allocation | 0.218 | 0.218 | 0.237 | 1.00 | 0.045 |
| water_sspr | 0.209 | 0.210 | 0.211 | 1.00 | 0.003 |
| bin_unpacking | 0.169 | 0.169 | 0.188 | 1.00 | 0.031 |
| upscaling | 0.145 | 0.145 | 0.174 | 1.00 | 0.005 |
| auxiliary_views | 0.145 | 0.145 | 0.571 | 1.00 | 0.155 |
| water_fft | 0.133 | 0.133 | 0.137 | 1.00 | 0.003 |
| tonemapping | 0.124 | 0.124 | 0.134 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.123 | 1.00 | 0.009 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.020 | 1.00 | 0.080 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.190 | 13.056 | 32.02 | 58777.8 |
| render/busy/ms | 9.633 | 12.423 | 31.20 | 55562.9 |
| main/busy/ms | 8.882 | 11.396 | 25.08 | 51233.9 |
| render/graph/ms | 5.362 | 7.176 | 18.76 | 30930.7 |
| pipelines/wait_ms | 3.877 | 0.000 | 8355.11 | 22364.4 |
| bigspace/total/ms | 0.574 | 1.508 | 3.84 | 3309.8 |
| render/present/ms | 0.463 | 0.589 | 2.27 | 2669.8 |
| terrain/update/ms | 0.313 | 0.416 | 2.02 | 1802.9 |
| controller/feed/ms | 0.304 | 0.431 | 0.84 | 1751.2 |
| terrain/select/ms | 0.277 | 0.370 | 0.88 | 1599.7 |
| stream/drive/ms | 0.263 | 0.389 | 17.29 | 1514.2 |
| bigspace/low_precision/ms | 0.231 | 0.601 | 2.76 | 1331.1 |
| far/select/ms | 0.207 | 0.314 | 0.64 | 1191.3 |
| stream/tick/ms | 0.205 | 0.298 | 0.85 | 1183.5 |
| anim/evaluate_cpu/ms | 0.176 | 0.237 | 0.50 | 1016.2 |
| pipelines/compile_ms | 0.174 | 0.000 | 143.34 | 1004.1 |
| bigspace/high_precision/ms | 0.170 | 0.557 | 3.10 | 977.8 |
| pipelines/front_ms | 0.167 | 0.000 | 137.43 | 963.6 |
| anim/evaluate/ms | 0.146 | 0.289 | 2.62 | 843.2 |
| controller/move/ms | 0.144 | 0.333 | 0.95 | 831.9 |
| render/acquire/ms | 0.095 | 0.130 | 0.37 | 545.1 |
| physics/step/ms | 0.052 | 0.099 | 0.46 | 300.9 |
| physics/insert/ms | 0.043 | 0.409 | 0.91 | 247.2 |
| anim/apply/ms | 0.012 | 0.023 | 0.29 | 70.1 |
| weather/eval/ms | 0.012 | 0.017 | 0.08 | 67.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.27 | 60.5 |
| script/update/ms | 0.008 | 0.011 | 0.14 | 44.5 |
| pipelines/driver_ms | 0.007 | 0.000 | 5.91 | 40.5 |
| script/input/ms | 0.005 | 0.008 | 0.21 | 31.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.27 | 30.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.27 | 28.6 |
| script/ui/ms | 0.004 | 0.007 | 0.04 | 25.3 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.05 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1296 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5603 |
| crowd/c1/meshes/count | count | 1178 | 1272 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4261 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.41 | 31.20 | 9.08 |
| gpu/timed/ms | ms | 5.57 | 8.83 | 5.75 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3386 | 5148 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3386 | 5148 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 287 | 42 |
| phases/shadow/bins | count | 3796 | 5463 | 2234 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9614 | 12.6 k | 5731 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 301 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 208 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 193 | 198 | 198 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 10 | 743 | 6 |
| preprocess/bind_groups_created | count | 10 | 743 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 231 | 350 | 221 |
| process/cpu_usage | % | 25.36 | 37.44 | 25.25 |
| process/mem_usage | GiB | 4.70 | 4.84 | 4.84 |
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
| started_load_count |  | 4751 | 4846 | 4846 |
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
| stream/latency/s | s | 0.03 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.11 | 91.84 | 33.11 |
| system/mem_usage | % | 74.12 | 75.24 | 74.47 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2805 | 2921 | 2921 |
| textures/images | count | 2580 | 2640 | 2640 |
| views/all/batch_sets | count | 113 | 339 | 102 |
| views/all/bins | count | 6451 | 8113 | 5714 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 191 | 355 | 98 |
| views/all/pipelines_distinct | count | 85 | 95 | 80 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 62 | 42 |
| views/cascade/bins | count | 2610 | 2985 | 2234 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8234 | 9752 | 5731 |
| views/cascade/pipelines | count | 52 | 62 | 42 |
| views/cascade/work_items | count | 8234 | 9752 | 5731 |
| views/lamp_face/batch_sets | count | 0 | 230 | 0 |
| views/lamp_face/bins | count | 1059 | 2478 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1194 | 2854 | 0 |
| views/lamp_face/pipelines | count | 84 | 241 | 0 |
| views/lamp_face/work_items | count | 0 | 2504 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2870 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8384 | 13.0 k | 7594 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8548 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 53.30 | 254 | 307.4 k |
| anim/events/count | count | 0.09 | 3 | 523 |
| assets/Image/added | count | 0.11 | 43 | 608 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.71 | 152 | 4117 |
| assets/Mesh/removed | count | 0.85 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 483 |
| assets/StandardMaterial/modified | count | 3.78 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.95 | 1492 | 5457 |
| physics/inserts | count | 0.42 | 111 | 2406 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 5869 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1287 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.5 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.73 | 2732.4 | main/busy/ms 2707, surface/advance_cpu_ms 70.99, surface/cell_upload_ms 0.74, stream/drive/ms 0.32, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.53 | 797.5 | main/busy/ms 2741, render/thread/ms 763, render/busy/ms 600, render/acquire/ms 160, surface/advance_cpu_ms 70.99, render/graph/ms 4.65, render/present/ms 2.59, stream/drive/ms 1.15, surface/cell_upload_ms 0.74, stream/tick/ms 0.57, terrain/update/ms 0.48, terrain/select/ms 0.35 |
| warm-up | 3.61 | 84.5 | pipelines/wait_ms 1308, render/thread/ms 777, render/busy/ms 613, render/acquire/ms 161, pipelines/compile_ms 79.37, pipelines/front_ms 72.03, main/busy/ms 51.46, terrain/update/ms 19.32, render/graph/ms 8.25, pipelines/driver_ms 7.33, render/present/ms 3.60, stream/drive/ms 1.60 |
| warm-up | 3.92 | 136.4 | pipelines/compile_ms 175, pipelines/front_ms 162, render/thread/ms 115, render/busy/ms 109, main/busy/ms 83.99, pipelines/driver_ms 12.48, render/acquire/ms 4.93, render/graph/ms 4.23, stream/drive/ms 2.84, stream/tick/ms 2.30, bigspace/total/ms 1.29, render/present/ms 1.28 |
| warm-up | 4.05 | 59.7 | pipelines/wait_ms 2493, pipelines/compile_ms 288, pipelines/front_ms 275, render/thread/ms 45.53, main/busy/ms 45.01, render/busy/ms 43.97, stream/drive/ms 28.19, pipelines/driver_ms 13.32, render/graph/ms 3.99, stream/spawn/ms 2.07, stream/tick/ms 1.49, render/present/ms 1.24 |
| warm-up | 4.08 | 34.0 | pipelines/wait_ms 2493, pipelines/compile_ms 288, pipelines/front_ms 275, render/thread/ms 58.49, render/busy/ms 56.84, main/busy/ms 33.18, stream/drive/ms 15.19, pipelines/driver_ms 13.32, render/graph/ms 4.55, stream/tick/ms 1.72, render/present/ms 1.32, terrain/update/ms 0.96 |
| warm-up | 4.11 | 30.6 | render/thread/ms 46.75, render/busy/ms 45.20, main/busy/ms 35.94, stream/drive/ms 15.62, render/graph/ms 4.33, stream/tick/ms 1.85, render/present/ms 1.36, terrain/update/ms 0.98, terrain/select/ms 0.77, stream/spawn/ms 0.73, bigspace/total/ms 0.46, stream/activate/ms 0.20 |
| warm-up | 4.15 | 35.9 | pipelines/wait_ms 1432, pipelines/compile_ms 94.65, pipelines/front_ms 89.35, main/busy/ms 49.55, render/thread/ms 48.52, render/busy/ms 47.02, stream/drive/ms 24.94, pipelines/driver_ms 5.30, render/graph/ms 5.17, terrain/update/ms 4.78, stream/spawn/ms 1.95, stream/tick/ms 1.87 |
| warm-up | 4.23 | 76.2 | pipelines/wait_ms 1432, pipelines/compile_ms 94.65, pipelines/front_ms 89.35, render/thread/ms 80.83, render/busy/ms 78.05, main/busy/ms 50.82, stream/drive/ms 25.38, pipelines/driver_ms 5.30, terrain/update/ms 4.70, render/graph/ms 4.61, render/present/ms 2.55, stream/spawn/ms 2.38 |
| warm-up | 4.26 | 31.9 | render/thread/ms 76.63, render/busy/ms 74.03, main/busy/ms 51.49, stream/drive/ms 29.87, render/graph/ms 3.12, render/present/ms 2.40, stream/tick/ms 1.67, stream/spawn/ms 1.32, terrain/update/ms 1.02, terrain/select/ms 0.82, bigspace/total/ms 0.44, render/acquire/ms 0.20 |
| warm-up | 4.37 | 106.7 | pipelines/wait_ms 4609, pipelines/compile_ms 202, pipelines/front_ms 191, render/thread/ms 101, render/busy/ms 65.60, main/busy/ms 53.83, render/present/ms 35.09, stream/drive/ms 29.10, pipelines/driver_ms 10.97, render/graph/ms 4.30, stream/tick/ms 1.33, terrain/update/ms 0.96 |
| warm-up | 4.40 | 33.1 | pipelines/wait_ms 8347, pipelines/compile_ms 335, pipelines/front_ms 229, pipelines/driver_ms 106, render/thread/ms 106, render/busy/ms 69.80, render/present/ms 35.58, main/busy/ms 34.13, render/graph/ms 11.33, stream/drive/ms 4.80, stream/tick/ms 1.52, stream/activate/ms 1.15 |
| warm-up | 4.43 | 30.2 | pipelines/wait_ms 3738, pipelines/compile_ms 133, pipelines/driver_ms 94.76, render/thread/ms 54.48, render/busy/ms 52.30, pipelines/front_ms 38.66, main/busy/ms 23.88, render/graph/ms 12.97, stream/drive/ms 4.27, render/present/ms 1.79, stream/tick/ms 1.64, stream/activate/ms 1.13 |
| warm-up | 4.58 | 88.0 | pipelines/wait_ms 3237, pipelines/compile_ms 117, pipelines/front_ms 113, main/busy/ms 72.76, stream/drive/ms 50.95, render/thread/ms 36.38, render/busy/ms 34.51, render/graph/ms 6.08, pipelines/driver_ms 4.79, stream/spawn/ms 3.82, stream/tick/ms 1.67, render/present/ms 1.39 |
| warm-up | 4.61 | 29.5 | pipelines/wait_ms 3237, pipelines/compile_ms 117, pipelines/front_ms 113, main/busy/ms 64.37, stream/drive/ms 47.10, render/thread/ms 46.00, render/busy/ms 43.96, render/graph/ms 7.76, pipelines/driver_ms 4.79, stream/spawn/ms 2.21, stream/tick/ms 1.68, render/present/ms 1.49 |
| warm-up | 4.69 | 78.5 | pipelines/wait_ms 626, pipelines/compile_ms 106, pipelines/front_ms 101, main/busy/ms 64.84, stream/drive/ms 45.91, render/thread/ms 45.15, render/busy/ms 43.96, pipelines/driver_ms 5.21, render/graph/ms 4.66, stream/spawn/ms 1.44, stream/activate/ms 1.04, render/present/ms 0.95 |
| warm-up | 4.75 | 44.6 | pipelines/wait_ms 2104, pipelines/compile_ms 92.46, pipelines/front_ms 86.03, main/busy/ms 46.99, stream/drive/ms 34.04, render/thread/ms 29.20, render/busy/ms 26.47, pipelines/driver_ms 6.43, render/graph/ms 3.72, render/present/ms 2.52, stream/activate/ms 1.09, stream/tick/ms 0.84 |
| warm-up | 4.79 | 28.2 | main/busy/ms 31.47, stream/drive/ms 22.08, render/thread/ms 14.73, render/busy/ms 13.69, render/graph/ms 3.75, render/present/ms 0.89, stream/activate/ms 0.70, stream/tick/ms 0.65, terrain/update/ms 0.57, stream/spawn/ms 0.54, terrain/select/ms 0.48, physics/insert/ms 0.37 |
| warm-up | 5.04 | 32.8 | pipelines/wait_ms 11.9 k, pipelines/compile_ms 48.90, pipelines/front_ms 44.41, main/busy/ms 25.97, render/thread/ms 25.65, render/busy/ms 24.54, stream/drive/ms 8.86, pipelines/driver_ms 4.49, render/graph/ms 3.52, terrain/update/ms 0.91, render/present/ms 0.90, stream/tick/ms 0.82 |
| warm-up | 5.16 | 28.0 | pipelines/wait_ms 3769, pipelines/compile_ms 108, pipelines/front_ms 104, render/thread/ms 34.51, render/busy/ms 33.36, main/busy/ms 14.65, render/graph/ms 4.65, pipelines/driver_ms 3.60, stream/drive/ms 1.88, render/present/ms 0.97, stream/tick/ms 0.90, terrain/update/ms 0.82 |
| warm-up | 5.57 | 36.4 | render/thread/ms 46.04, render/busy/ms 44.41, main/busy/ms 30.70, render/graph/ms 9.66, stream/drive/ms 3.59, stream/spawn/ms 2.18, render/present/ms 1.32, physics/insert/ms 1.08, stream/tick/ms 0.75, terrain/update/ms 0.72, terrain/select/ms 0.64, physics/step/ms 0.58 |
| warm-up | 5.62 | 28.2 | pipelines/wait_ms 2191, pipelines/compile_ms 140, pipelines/front_ms 136, render/thread/ms 33.40, render/busy/ms 32.20, main/busy/ms 21.77, render/graph/ms 5.20, pipelines/driver_ms 4.26, stream/drive/ms 3.35, stream/spawn/ms 1.95, physics/step/ms 1.10, physics/insert/ms 1.06 |
| warm-up | 5.80 | 42.2 | pipelines/wait_ms 1477, main/busy/ms 47.57, render/thread/ms 46.34, render/busy/ms 44.28, pipelines/compile_ms 12.23, pipelines/front_ms 11.82, render/graph/ms 10.60, stream/drive/ms 6.17, stream/spawn/ms 2.34, render/present/ms 1.75, bigspace/total/ms 0.94, anim/evaluate_cpu/ms 0.94 |
| warm-up | 5.83 | 29.1 | render/thread/ms 50.00, render/busy/ms 48.45, main/busy/ms 37.80, render/graph/ms 7.10, stream/drive/ms 4.21, stream/spawn/ms 2.44, render/present/ms 1.25, terrain/update/ms 0.86, bigspace/total/ms 0.86, terrain/select/ms 0.74, stream/tick/ms 0.71, anim/evaluate/ms 0.64 |
| warm-up | 5.90 | 30.5 | pipelines/wait_ms 1357, pipelines/compile_ms 114, pipelines/front_ms 111, main/busy/ms 35.89, render/thread/ms 27.72, render/busy/ms 26.32, stream/drive/ms 14.60, render/graph/ms 9.51, pipelines/driver_ms 2.84, stream/spawn/ms 2.48, stream/tick/ms 1.45, render/present/ms 1.11 |
| warm-up | 6.45 | 27.9 | main/busy/ms 33.87, render/thread/ms 31.23, render/busy/ms 30.06, render/graph/ms 8.20, terrain/update/ms 4.88, stream/drive/ms 2.08, render/present/ms 0.96, stream/tick/ms 0.88, bigspace/total/ms 0.70, stream/spawn/ms 0.64, terrain/select/ms 0.57, anim/evaluate/ms 0.44 |
| warm-up | 8.46 | 35.5 | pipelines/wait_ms 4499, main/busy/ms 41.89, render/thread/ms 30.95, render/busy/ms 29.59, pipelines/compile_ms 21.57, pipelines/front_ms 20.18, render/graph/ms 12.06, pipelines/driver_ms 1.39, render/present/ms 1.18, stream/drive/ms 1.00, anim/evaluate_cpu/ms 0.94, bigspace/total/ms 0.87 |
| measure | 2.72 | 28.2 | render/thread/ms 39.20, render/busy/ms 37.83, main/busy/ms 22.24, render/graph/ms 21.43, render/present/ms 1.17, bigspace/total/ms 1.04, physics/insert/ms 0.68, controller/feed/ms 0.60, terrain/update/ms 0.57, terrain/select/ms 0.51, stream/drive/ms 0.49, stream/tick/ms 0.44 |
| measure | 19.28 | 33.3 | render/thread/ms 45.80, render/busy/ms 44.33, main/busy/ms 26.64, render/graph/ms 26.43, render/present/ms 1.27, bigspace/total/ms 0.89, terrain/update/ms 0.67, controller/feed/ms 0.60, terrain/select/ms 0.60, stream/drive/ms 0.53, stream/tick/ms 0.45, far/select/ms 0.42 |
| measure | 23.09 | 28.5 | render/thread/ms 37.88, render/busy/ms 36.61, main/busy/ms 30.80, render/graph/ms 12.22, anim/evaluate/ms 2.82, controller/feed/ms 1.18, render/present/ms 1.04, stream/drive/ms 1.00, bigspace/total/ms 0.85, anim/evaluate_cpu/ms 0.73, terrain/update/ms 0.65, terrain/select/ms 0.58 |
| measure | 59.01 | 31.2 | render/thread/ms 33.62, main/busy/ms 32.81, render/busy/ms 32.20, render/graph/ms 12.41, bigspace/total/ms 4.31, stream/drive/ms 1.66, render/present/ms 1.20, controller/feed/ms 1.09, anim/evaluate_cpu/ms 0.92, stream/spawn/ms 0.83, terrain/update/ms 0.77, bigspace/high_precision/ms 0.77 |
| measure | 59.04 | 28.7 | render/thread/ms 39.36, render/busy/ms 38.07, main/busy/ms 26.31, render/graph/ms 9.18, bigspace/total/ms 1.59, terrain/update/ms 1.06, render/present/ms 1.06, terrain/select/ms 0.92, controller/feed/ms 0.87, stream/drive/ms 0.86, stream/tick/ms 0.76, bigspace/high_precision/ms 0.70 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **65%** (min 38%) · 2580 MHz (min 2580) · memory 8001 MHz · 67.8 W · 73 °C (max 79) · VRAM 5984 MiB · throttled 100% of samples
