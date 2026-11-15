# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5287 frames over 65.6 s · **median 12.02 ms (83 fps)** · mean 12.42 · p95 15.89 · p99 18.37 (1% low 54 fps) · worst 70.12 ms · 63 spikes over 1.5× median · 22.9 stutters/min (work 26.5/min)
- warm-up: 330 frames over 9.0 s, worst 2836.5 ms
- hitches (> 30.0 ms): **4** measured, 23 in warm-up
- timed GPU passes: 5.56 ms of a 12.42 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.02 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 70.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.057 | 2.058 | 2.086 | 1.00 | 0.206 |
| main_opaque_pass_3d | 0.959 | 0.960 | 1.370 | 1.00 | 0.075 |
| early prepass | 0.419 | 0.419 | 0.642 | 1.00 | 0.107 |
| atmosphere_luts | 0.312 | 0.312 | 0.373 | 1.00 | 0.004 |
| water_velocity | 0.280 | 0.280 | 0.350 | 1.00 | 0.002 |
| uniform_allocation | 0.220 | 0.220 | 0.237 | 1.00 | 0.049 |
| water_sspr | 0.210 | 0.210 | 0.211 | 1.00 | 0.003 |
| auxiliary_views | 0.193 | 0.193 | 0.639 | 1.00 | 0.212 |
| bin_unpacking | 0.169 | 0.169 | 0.190 | 1.00 | 0.034 |
| upscaling | 0.144 | 0.144 | 0.176 | 1.00 | 0.005 |
| water_fft | 0.134 | 0.134 | 0.140 | 1.00 | 0.003 |
| tonemapping | 0.122 | 0.122 | 0.130 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.079 | 0.079 | 0.124 | 1.00 | 0.010 |
| clustering | 0.066 | 0.066 | 0.070 | 1.00 | 0.023 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.007 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.008 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.019 | 1.00 | 0.084 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.290 | 0.000 | 14427.31 | 70263.7 |
| render/thread/ms | 11.115 | 14.379 | 69.11 | 58767.0 |
| render/busy/ms | 10.519 | 13.673 | 68.47 | 55613.1 |
| main/busy/ms | 9.682 | 12.336 | 24.93 | 51191.1 |
| render/graph/ms | 5.901 | 7.938 | 58.79 | 31198.3 |
| bigspace/total/ms | 0.611 | 1.598 | 3.36 | 3229.4 |
| render/present/ms | 0.495 | 0.642 | 3.11 | 2618.1 |
| controller/feed/ms | 0.342 | 0.480 | 1.08 | 1809.1 |
| terrain/update/ms | 0.327 | 0.435 | 2.34 | 1729.4 |
| terrain/select/ms | 0.289 | 0.384 | 1.01 | 1527.2 |
| stream/drive/ms | 0.287 | 0.434 | 16.93 | 1514.8 |
| bigspace/low_precision/ms | 0.249 | 0.621 | 2.69 | 1316.2 |
| stream/tick/ms | 0.224 | 0.341 | 0.66 | 1184.6 |
| far/select/ms | 0.221 | 0.343 | 0.78 | 1167.2 |
| pipelines/compile_ms | 0.206 | 0.000 | 129.20 | 1091.5 |
| pipelines/front_ms | 0.198 | 0.000 | 124.71 | 1046.6 |
| anim/evaluate_cpu/ms | 0.194 | 0.274 | 0.84 | 1023.3 |
| bigspace/high_precision/ms | 0.175 | 0.576 | 2.85 | 922.9 |
| controller/move/ms | 0.159 | 0.362 | 1.19 | 839.9 |
| anim/evaluate/ms | 0.158 | 0.310 | 2.15 | 834.0 |
| render/acquire/ms | 0.101 | 0.142 | 0.42 | 535.8 |
| physics/step/ms | 0.061 | 0.111 | 0.84 | 321.8 |
| physics/insert/ms | 0.053 | 0.443 | 0.93 | 279.9 |
| anim/apply/ms | 0.014 | 0.025 | 0.42 | 74.5 |
| weather/eval/ms | 0.013 | 0.019 | 0.19 | 70.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.21 | 58.9 |
| pipelines/driver_ms | 0.008 | 0.000 | 6.94 | 44.9 |
| script/update/ms | 0.008 | 0.012 | 0.19 | 44.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.26 | 33.1 |
| script/input/ms | 0.006 | 0.008 | 0.16 | 31.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 27.5 |
| script/ui/ms | 0.005 | 0.008 | 0.06 | 27.3 |
| controller/write/ms | 0.004 | 0.005 | 0.16 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.16 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.06 | 4.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5507 | 5937 | 5581 |
| crowd/c1/meshes/count | count | 1166 | 1281 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4273 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4245 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.31 | 68.47 | 9.13 |
| gpu/timed/ms | ms | 5.60 | 8.74 | 5.73 |
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
| phases/alpha_mask3d/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5112 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5112 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 59 | 286 | 42 |
| phases/shadow/bins | count | 3775 | 5454 | 2226 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9636 | 12.5 k | 5723 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 147 | 303 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 961 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 192 | 197 | 197 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 10 | 741 | 6 |
| preprocess/bind_groups_created | count | 10 | 741 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 236 | 356 | 221 |
| process/cpu_usage | % | 25.76 | 40.79 | 19.23 |
| process/mem_usage | GiB | 4.71 | 4.86 | 4.86 |
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
| started_load_count |  | 4752 | 4846 | 4846 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 43.85 | 79.75 | 42.94 |
| system/mem_usage | % | 74.53 | 75.02 | 74.84 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2805 | 2921 | 2921 |
| textures/images | count | 2580 | 2640 | 2640 |
| views/all/batch_sets | count | 115 | 338 | 102 |
| views/all/bins | count | 6514 | 8163 | 5706 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 200 | 353 | 98 |
| views/all/pipelines_distinct | count | 84 | 95 | 80 |
| views/all/work_items | count | 17.5 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 53 | 61 | 42 |
| views/cascade/bins | count | 2623 | 2968 | 2226 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8237 | 9672 | 5723 |
| views/cascade/pipelines | count | 53 | 61 | 42 |
| views/cascade/work_items | count | 8237 | 9672 | 5723 |
| views/lamp_face/batch_sets | count | 6 | 230 | 0 |
| views/lamp_face/bins | count | 1085 | 2532 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1211 | 2868 | 0 |
| views/lamp_face/pipelines | count | 88 | 244 | 0 |
| views/lamp_face/work_items | count | 81 | 2606 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2878 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8436 | 13.0 k | 7594 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8601 | 13.2 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 59.58 | 253 | 315.0 k |
| anim/events/count | count | 0.11 | 4 | 606 |
| assets/Image/added | count | 0.11 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.77 | 152 | 4075 |
| assets/Mesh/removed | count | 0.92 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.12 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.89 | 1485 | 4718 |
| physics/inserts | count | 0.60 | 143 | 3172 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.74 | 3 | 3936 |
| physics/steps_dropped | count | 0.00 | 2 | 2 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 6403 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1404 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.84 | 2836.5 | main/busy/ms 2811, surface/advance_cpu_ms 71.54, surface/cell_upload_ms 0.76, stream/drive/ms 0.31, stream/tick/ms 0.07, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.62 | 783.5 | main/busy/ms 2842, render/thread/ms 747, render/busy/ms 577, render/acquire/ms 168, surface/advance_cpu_ms 71.54, render/graph/ms 5.82, render/present/ms 2.07, stream/drive/ms 1.32, stream/tick/ms 0.80, surface/cell_upload_ms 0.76, terrain/update/ms 0.65, terrain/select/ms 0.47 |
| warm-up | 3.75 | 126.6 | render/thread/ms 768, render/busy/ms 596, pipelines/wait_ms 577, render/acquire/ms 168, pipelines/compile_ms 115, pipelines/front_ms 108, main/busy/ms 53.32, terrain/update/ms 15.85, render/graph/ms 14.12, pipelines/driver_ms 6.87, render/present/ms 3.53, stream/drive/ms 1.93 |
| warm-up | 4.01 | 106.8 | pipelines/compile_ms 213, pipelines/front_ms 192, render/thread/ms 87.81, render/busy/ms 76.74, main/busy/ms 70.62, pipelines/driver_ms 21.30, render/acquire/ms 9.01, render/graph/ms 6.24, render/present/ms 2.05, stream/drive/ms 1.72, stream/tick/ms 1.37, pipelines/wait_ms 1.35 |
| warm-up | 4.16 | 55.5 | pipelines/compile_ms 227, pipelines/front_ms 222, pipelines/wait_ms 193, main/busy/ms 45.16, render/thread/ms 44.72, render/busy/ms 43.40, stream/drive/ms 29.50, pipelines/driver_ms 5.69, render/graph/ms 3.59, stream/tick/ms 1.34, render/present/ms 1.09, terrain/update/ms 1.09 |
| warm-up | 4.26 | 41.0 | pipelines/wait_ms 2265, pipelines/compile_ms 111, pipelines/front_ms 102, render/thread/ms 48.41, render/busy/ms 46.34, main/busy/ms 32.98, pipelines/driver_ms 9.44, terrain/update/ms 6.80, render/graph/ms 5.66, stream/drive/ms 4.12, stream/tick/ms 2.11, render/present/ms 1.62 |
| warm-up | 4.29 | 32.4 | pipelines/wait_ms 4384, pipelines/compile_ms 97.84, pipelines/front_ms 89.90, render/thread/ms 50.49, render/busy/ms 48.80, main/busy/ms 48.44, stream/drive/ms 25.20, pipelines/driver_ms 7.94, terrain/update/ms 7.77, render/graph/ms 4.17, stream/tick/ms 1.89, render/present/ms 1.44 |
| warm-up | 4.37 | 82.3 | pipelines/wait_ms 3061, render/thread/ms 75.60, render/busy/ms 74.08, main/busy/ms 52.67, pipelines/compile_ms 45.67, pipelines/front_ms 40.71, stream/drive/ms 26.25, pipelines/driver_ms 4.96, render/graph/ms 3.98, stream/tick/ms 1.80, terrain/update/ms 1.80, render/present/ms 1.30 |
| warm-up | 4.53 | 131.5 | pipelines/wait_ms 2846, pipelines/compile_ms 200, pipelines/front_ms 192, render/thread/ms 102, render/busy/ms 63.98, main/busy/ms 57.19, render/present/ms 38.12, stream/drive/ms 30.34, pipelines/driver_ms 8.24, render/graph/ms 3.44, stream/spawn/ms 2.84, stream/tick/ms 1.25 |
| warm-up | 4.58 | 32.1 | pipelines/wait_ms 1533, pipelines/compile_ms 92.61, pipelines/front_ms 88.62, render/thread/ms 48.43, render/busy/ms 43.50, main/busy/ms 27.87, render/graph/ms 6.81, stream/drive/ms 4.97, render/present/ms 4.54, pipelines/driver_ms 3.99, stream/spawn/ms 1.23, stream/tick/ms 1.17 |
| warm-up | 4.65 | 43.5 | pipelines/wait_ms 820, pipelines/compile_ms 75.62, pipelines/front_ms 72.73, render/thread/ms 58.70, render/busy/ms 56.09, main/busy/ms 23.86, render/graph/ms 10.91, stream/drive/ms 7.16, stream/spawn/ms 3.00, pipelines/driver_ms 2.89, render/present/ms 2.31, stream/tick/ms 1.71 |
| warm-up | 4.69 | 34.2 | pipelines/wait_ms 4204, pipelines/compile_ms 110, pipelines/front_ms 105, render/thread/ms 65.58, render/busy/ms 62.90, main/busy/ms 27.30, render/graph/ms 13.33, stream/drive/ms 7.48, pipelines/driver_ms 5.22, stream/spawn/ms 4.28, render/present/ms 2.37, stream/tick/ms 2.05 |
| warm-up | 4.73 | 46.6 | pipelines/wait_ms 6856, main/busy/ms 76.21, pipelines/compile_ms 62.29, pipelines/front_ms 56.76, render/thread/ms 55.16, stream/drive/ms 54.99, render/busy/ms 53.47, render/graph/ms 6.68, pipelines/driver_ms 5.53, stream/spawn/ms 3.26, stream/tick/ms 2.15, render/present/ms 1.45 |
| warm-up | 4.83 | 95.2 | pipelines/wait_ms 7312, pipelines/compile_ms 136, pipelines/front_ms 129, render/thread/ms 82.18, render/busy/ms 80.77, main/busy/ms 77.19, stream/drive/ms 53.67, pipelines/driver_ms 6.62, render/graph/ms 4.35, stream/spawn/ms 3.08, render/present/ms 1.18, stream/tick/ms 0.99 |
| warm-up | 4.92 | 65.4 | pipelines/wait_ms 3037, pipelines/compile_ms 109, pipelines/front_ms 105, main/busy/ms 71.19, stream/drive/ms 50.39, render/thread/ms 38.63, render/busy/ms 37.24, pipelines/driver_ms 4.24, render/graph/ms 3.10, stream/spawn/ms 1.35, stream/activate/ms 1.13, render/present/ms 1.00 |
| warm-up | 4.98 | 48.7 | pipelines/wait_ms 9914, main/busy/ms 45.52, stream/drive/ms 33.86, pipelines/compile_ms 31.09, render/thread/ms 28.29, render/busy/ms 26.99, pipelines/front_ms 25.68, pipelines/driver_ms 5.42, render/graph/ms 4.87, stream/activate/ms 1.27, render/present/ms 1.06, stream/tick/ms 0.81 |
| warm-up | 5.35 | 40.0 | pipelines/wait_ms 11.8 k, pipelines/compile_ms 187, pipelines/front_ms 170, main/busy/ms 47.15, render/thread/ms 45.92, render/busy/ms 43.60, pipelines/driver_ms 16.93, render/graph/ms 6.98, stream/drive/ms 2.34, render/present/ms 2.00, stream/tick/ms 1.11, terrain/update/ms 0.73 |
| warm-up | 5.40 | 50.9 | pipelines/wait_ms 12.2 k, pipelines/compile_ms 74.91, pipelines/front_ms 68.73, render/thread/ms 63.00, render/busy/ms 61.23, main/busy/ms 48.94, render/graph/ms 8.31, pipelines/driver_ms 6.18, stream/drive/ms 2.33, render/present/ms 1.44, stream/tick/ms 1.15, terrain/update/ms 0.72 |
| warm-up | 5.44 | 36.5 | pipelines/wait_ms 13.7 k, pipelines/compile_ms 85.76, pipelines/front_ms 79.13, render/thread/ms 70.21, render/busy/ms 68.55, main/busy/ms 59.81, render/graph/ms 9.59, pipelines/driver_ms 6.63, stream/drive/ms 1.96, render/present/ms 1.37, physics/insert/ms 0.92, bigspace/total/ms 0.83 |
| warm-up | 5.49 | 45.0 | pipelines/wait_ms 6306, pipelines/compile_ms 65.88, render/thread/ms 64.89, render/busy/ms 63.56, pipelines/front_ms 62.33, main/busy/ms 36.39, render/graph/ms 7.66, pipelines/driver_ms 3.56, stream/drive/ms 2.26, physics/insert/ms 1.20, render/present/ms 1.12, terrain/update/ms 0.90 |
| warm-up | 6.24 | 31.2 | pipelines/wait_ms 1954, main/busy/ms 41.93, render/thread/ms 38.66, render/busy/ms 36.42, pipelines/compile_ms 18.57, pipelines/front_ms 17.78, render/graph/ms 11.76, stream/drive/ms 4.12, bigspace/total/ms 2.83, stream/spawn/ms 2.52, bigspace/low_precision/ms 2.10, render/present/ms 1.99 |
| warm-up | 6.27 | 31.6 | render/thread/ms 42.25, main/busy/ms 41.38, render/busy/ms 40.71, render/graph/ms 7.98, stream/drive/ms 3.90, bigspace/total/ms 2.66, stream/spawn/ms 2.12, bigspace/low_precision/ms 2.03, physics/insert/ms 1.32, render/present/ms 1.23, terrain/update/ms 1.22, terrain/select/ms 0.98 |
| warm-up | 6.48 | 45.6 | pipelines/wait_ms 3116, pipelines/compile_ms 127, pipelines/front_ms 119, render/thread/ms 54.78, render/busy/ms 53.29, main/busy/ms 38.33, stream/drive/ms 19.51, render/graph/ms 8.58, pipelines/driver_ms 8.46, render/present/ms 1.29, stream/spawn/ms 1.15, stream/tick/ms 0.71 |
| measure | 2.76 | 70.1 | render/thread/ms 79.98, render/busy/ms 78.83, render/graph/ms 65.09, main/busy/ms 20.57, render/present/ms 0.95, bigspace/total/ms 0.72, terrain/update/ms 0.58, stream/drive/ms 0.54, controller/feed/ms 0.53, terrain/select/ms 0.53, stream/tick/ms 0.48, physics/insert/ms 0.46 |
| measure | 7.13 | 37.7 | render/thread/ms 46.23, render/busy/ms 45.24, main/busy/ms 16.66, render/graph/ms 9.98, render/present/ms 0.82, bigspace/total/ms 0.81, stream/drive/ms 0.79, terrain/update/ms 0.63, stream/tick/ms 0.60, terrain/select/ms 0.57, controller/feed/ms 0.54, anim/evaluate_cpu/ms 0.35 |
| measure | 59.05 | 35.0 | render/thread/ms 34.41, render/busy/ms 32.87, main/busy/ms 23.51, render/graph/ms 10.44, render/present/ms 1.29, controller/feed/ms 1.19, bigspace/total/ms 1.18, terrain/update/ms 0.84, terrain/select/ms 0.75, far/select/ms 0.66, stream/drive/ms 0.62, anim/evaluate_cpu/ms 0.56 |
| measure | 59.08 | 35.5 | render/thread/ms 41.26, render/busy/ms 40.13, main/busy/ms 20.93, render/graph/ms 8.61, controller/feed/ms 0.98, bigspace/total/ms 0.96, render/present/ms 0.91, terrain/update/ms 0.77, terrain/select/ms 0.68, stream/drive/ms 0.54, controller/move/ms 0.50, far/select/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 18% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **61%** (min 41%) · 2580 MHz (min 2580) · memory 8001 MHz · 64.5 W · 73 °C (max 78) · VRAM 5985 MiB · throttled 100% of samples
