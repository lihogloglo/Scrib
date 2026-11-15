# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6047 frames over 65.6 s · **median 10.51 ms (95 fps)** · mean 10.86 · p95 13.62 · p99 16.39 (1% low 61 fps) · worst 36.87 ms · 71 spikes over 1.5× median · 35.6 stutters/min (work 32.0/min)
- warm-up: 672 frames over 12.0 s, worst 2617.9 ms
- hitches (> 26.3 ms): **5** measured, 35 in warm-up
- timed GPU passes: 5.81 ms of a 10.86 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.51 ms, limit 11 ms
- **FAIL** frame_ms: 39 stutters (> 1.5x moving median of 60) in 1.09 min = 35.65/min, limit 1/min
- **FAIL** worst frame 36.87 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.060 | 2.061 | 2.189 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.272 | 1.273 | 1.721 | 1.00 | 0.054 |
| early prepass | 0.412 | 0.412 | 0.626 | 1.00 | 0.095 |
| atmosphere_luts | 0.313 | 0.313 | 0.372 | 1.00 | 0.004 |
| water_velocity | 0.272 | 0.273 | 0.349 | 1.00 | 0.002 |
| uniform_allocation | 0.216 | 0.216 | 0.231 | 1.00 | 0.041 |
| water_sspr | 0.210 | 0.210 | 0.212 | 1.00 | 0.002 |
| bin_unpacking | 0.170 | 0.170 | 0.187 | 1.00 | 0.029 |
| upscaling | 0.146 | 0.146 | 0.186 | 1.00 | 0.004 |
| auxiliary_views | 0.141 | 0.141 | 0.539 | 1.00 | 0.140 |
| water_fft | 0.132 | 0.132 | 0.137 | 1.00 | 0.003 |
| tonemapping | 0.122 | 0.122 | 0.132 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.082 | 0.082 | 0.145 | 1.00 | 0.008 |
| clustering | 0.066 | 0.066 | 0.070 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.049 | 1.00 | 0.005 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.022 | 1.00 | 0.074 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 77.390 | 0.000 | 32589.53 | 467977.8 |
| render/thread/ms | 9.690 | 12.326 | 29.10 | 58592.9 |
| render/busy/ms | 9.142 | 11.731 | 28.31 | 55280.4 |
| main/busy/ms | 8.536 | 10.786 | 29.53 | 51617.4 |
| pipelines/compile_ms | 6.734 | 0.000 | 2746.16 | 40718.1 |
| pipelines/driver_ms | 6.112 | 0.000 | 2615.61 | 36957.3 |
| render/graph/ms | 5.086 | 6.706 | 16.09 | 30756.3 |
| pipelines/front_ms | 0.622 | 0.000 | 181.94 | 3760.8 |
| bigspace/total/ms | 0.545 | 1.452 | 3.61 | 3296.3 |
| render/present/ms | 0.453 | 0.578 | 1.83 | 2738.7 |
| terrain/update/ms | 0.306 | 0.395 | 1.63 | 1848.7 |
| terrain/select/ms | 0.272 | 0.350 | 0.88 | 1642.1 |
| controller/feed/ms | 0.266 | 0.371 | 0.75 | 1607.8 |
| stream/drive/ms | 0.261 | 0.376 | 22.00 | 1576.2 |
| bigspace/low_precision/ms | 0.211 | 0.607 | 2.55 | 1273.6 |
| stream/tick/ms | 0.205 | 0.300 | 0.80 | 1241.0 |
| far/select/ms | 0.195 | 0.278 | 0.54 | 1176.8 |
| bigspace/high_precision/ms | 0.177 | 0.651 | 3.22 | 1067.3 |
| anim/evaluate_cpu/ms | 0.154 | 0.207 | 0.59 | 934.2 |
| anim/evaluate/ms | 0.132 | 0.271 | 2.03 | 795.2 |
| controller/move/ms | 0.116 | 0.206 | 0.55 | 699.7 |
| render/acquire/ms | 0.095 | 0.128 | 0.44 | 573.7 |
| physics/insert/ms | 0.080 | 0.465 | 1.34 | 483.1 |
| physics/step/ms | 0.046 | 0.087 | 0.78 | 278.3 |
| weather/eval/ms | 0.011 | 0.016 | 0.20 | 68.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.21 | 64.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.28 | 57.6 |
| script/update/ms | 0.007 | 0.012 | 0.20 | 45.2 |
| script/input/ms | 0.005 | 0.008 | 0.26 | 32.3 |
| script/ui/ms | 0.005 | 0.007 | 0.19 | 29.2 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 29.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 26.7 |
| controller/write/ms | 0.003 | 0.005 | 0.30 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.71 | 14.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 24 | 0 |
| anim/a2/count | count | 1 | 19 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 80 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 25 | 43 | 11 |
| crowd/c0/joints/count | count | 1247 | 2262 | 480 |
| crowd/c0/meshes/count | count | 343 | 646 | 117 |
| crowd/c1/count | count | 146 | 157 | 150 |
| crowd/c1/joints/count | count | 5475 | 6046 | 5716 |
| crowd/c1/meshes/count | count | 1160 | 1322 | 1224 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4253 | 17.5 k | 6179 |
| crowd/joints_uploaded/count | count | 4234 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.03 | 29.53 | 7.22 |
| gpu/timed/ms | ms | 5.86 | 8.67 | 6.57 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1277 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3375 | 5148 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1277 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3375 | 5148 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 53 | 275 | 42 |
| phases/shadow/bins | count | 3716 | 5478 | 2242 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9101 | 12.6 k | 5739 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 137 | 303 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2629 | 3268 | 2348 |
| physics/items | count | 91 | 476 | 19 |
| physics/pending | count | 0 | 2697 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 193 | 198 | 198 |
| pipelines/waiting | count | 0 | 45 | 0 |
| preprocess/bind_groups | count | 10 | 719 | 6 |
| preprocess/bind_groups_created | count | 10 | 719 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 228 | 341 | 221 |
| process/cpu_usage | % | 25.48 | 50.60 | 22.42 |
| process/mem_usage | GiB | 4.86 | 4.98 | 4.98 |
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
| started_load_count |  | 4754 | 4845 | 4845 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.12 | 67.82 | 31.90 |
| system/mem_usage | % | 63.86 | 64.45 | 64.38 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2921 | 2921 |
| textures/images | count | 2582 | 2640 | 2640 |
| views/all/batch_sets | count | 110 | 327 | 102 |
| views/all/bins | count | 5953 | 8056 | 5722 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.8 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 191 | 357 | 98 |
| views/all/pipelines_distinct | count | 83 | 95 | 80 |
| views/all/work_items | count | 16.6 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 61 | 42 |
| views/cascade/bins | count | 2589 | 2969 | 2242 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8099 | 9701 | 5739 |
| views/cascade/pipelines | count | 52 | 61 | 42 |
| views/cascade/work_items | count | 8099 | 9701 | 5739 |
| views/lamp_face/batch_sets | count | 0 | 219 | 0 |
| views/lamp_face/bins | count | 1058 | 2531 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1194 | 2901 | 0 |
| views/lamp_face/pipelines | count | 83 | 243 | 0 |
| views/lamp_face/work_items | count | 0 | 2451 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2856 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8370 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8540 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 46.28 | 854 | 279.8 k |
| anim/events/count | count | 0.08 | 3 | 475 |
| assets/Image/added | count | 0.12 | 47 | 752 |
| assets/Image/modified | count | 0.00 | 9 | 23 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.75 | 157 | 4545 |
| assets/Mesh/removed | count | 0.81 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 49 | 640 |
| assets/StandardMaterial/modified | count | 3.60 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 57.80 | 1304 | 349.5 k |
| ecs/entities_delta | count | 2.82 | 3294 | 17.0 k |
| physics/inserts | count | 0.74 | 176 | 4502 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2270 |
| physics/steps | count | 0.65 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 9 | 19 |
| pipelines/created | count | 0.01 | 9 | 34 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5598 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1227 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2617.9 | main/busy/ms 2589, surface/advance_cpu_ms 71.72, surface/cell_upload_ms 0.81, stream/drive/ms 0.35, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.39 | 776.0 | main/busy/ms 2632, render/thread/ms 730, render/busy/ms 562, render/acquire/ms 166, surface/advance_cpu_ms 71.72, render/graph/ms 6.41, render/present/ms 2.43, stream/drive/ms 1.36, surface/cell_upload_ms 0.81, stream/tick/ms 0.63, terrain/update/ms 0.42, bigspace/total/ms 0.30 |
| warm-up | 3.61 | 213.2 | render/thread/ms 742, pipelines/wait_ms 640, render/busy/ms 572, pipelines/compile_ms 203, pipelines/driver_ms 178, render/acquire/ms 166, main/busy/ms 61.25, terrain/update/ms 25.34, pipelines/front_ms 25.31, render/graph/ms 9.83, render/present/ms 3.39, stream/drive/ms 1.69 |
| warm-up | 3.83 | 86.0 | main/busy/ms 63.36, render/thread/ms 46.74, render/busy/ms 45.06, render/graph/ms 4.59, stream/drive/ms 2.81, stream/tick/ms 1.45, render/present/ms 1.45, terrain/update/ms 0.85, terrain/select/ms 0.71, stream/spawn/ms 0.39, render/acquire/ms 0.24, bigspace/total/ms 0.23 |
| warm-up | 4.04 | 84.4 | pipelines/wait_ms 3412, pipelines/compile_ms 89.31, pipelines/front_ms 81.16, main/busy/ms 59.58, render/thread/ms 46.95, render/busy/ms 45.58, stream/drive/ms 43.61, pipelines/driver_ms 8.15, render/graph/ms 3.01, stream/tick/ms 1.33, render/present/ms 1.14, terrain/update/ms 0.96 |
| warm-up | 4.09 | 36.6 | main/busy/ms 32.12, render/thread/ms 28.91, render/busy/ms 26.99, stream/drive/ms 17.30, render/graph/ms 4.58, render/present/ms 1.65, stream/tick/ms 0.94, terrain/update/ms 0.92, terrain/select/ms 0.78, render/acquire/ms 0.26, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.11 |
| warm-up | 4.19 | 76.7 | pipelines/wait_ms 5429, pipelines/compile_ms 122, pipelines/front_ms 115, render/thread/ms 71.08, render/busy/ms 68.71, main/busy/ms 46.28, stream/drive/ms 19.08, render/graph/ms 9.25, pipelines/driver_ms 7.05, stream/spawn/ms 2.49, render/present/ms 2.08, stream/tick/ms 1.34 |
| warm-up | 4.23 | 38.6 | pipelines/wait_ms 3089, render/thread/ms 77.94, render/busy/ms 75.78, main/busy/ms 54.38, pipelines/compile_ms 44.41, pipelines/front_ms 40.59, stream/drive/ms 29.17, render/graph/ms 8.70, pipelines/driver_ms 3.82, stream/spawn/ms 2.92, render/present/ms 1.76, stream/tick/ms 1.61 |
| warm-up | 4.34 | 114.6 | pipelines/wait_ms 8924, pipelines/compile_ms 143, pipelines/front_ms 136, main/busy/ms 106, render/thread/ms 94.15, render/busy/ms 92.51, stream/drive/ms 28.42, pipelines/driver_ms 7.87, render/graph/ms 6.26, stream/spawn/ms 2.41, render/present/ms 1.31, stream/tick/ms 1.28 |
| warm-up | 4.39 | 44.0 | pipelines/wait_ms 8924, pipelines/compile_ms 143, pipelines/front_ms 136, render/thread/ms 107, render/busy/ms 106, main/busy/ms 81.80, render/graph/ms 15.29, pipelines/driver_ms 7.87, stream/drive/ms 4.92, render/present/ms 1.59, stream/tick/ms 1.02, stream/activate/ms 0.95 |
| warm-up | 4.45 | 43.6 | pipelines/wait_ms 1446, pipelines/compile_ms 88.41, pipelines/front_ms 82.53, render/thread/ms 55.61, render/busy/ms 54.01, main/busy/ms 16.31, render/graph/ms 14.00, pipelines/driver_ms 5.88, stream/drive/ms 3.16, render/present/ms 1.35, stream/tick/ms 0.97, terrain/update/ms 0.88 |
| warm-up | 4.50 | 32.3 | render/thread/ms 40.65, render/busy/ms 38.30, main/busy/ms 32.05, render/graph/ms 12.55, stream/drive/ms 9.50, stream/spawn/ms 5.20, stream/tick/ms 2.57, render/present/ms 1.93, terrain/update/ms 1.21, terrain/select/ms 1.04, render/acquire/ms 0.42, bigspace/total/ms 0.41 |
| warm-up | 4.54 | 39.4 | main/busy/ms 71.83, stream/drive/ms 51.95, render/thread/ms 42.48, render/busy/ms 40.49, render/graph/ms 5.44, stream/spawn/ms 3.17, stream/tick/ms 2.37, render/present/ms 1.50, terrain/update/ms 1.30, terrain/select/ms 1.11, render/acquire/ms 0.49, stream/activate/ms 0.44 |
| warm-up | 4.61 | 65.7 | main/busy/ms 66.18, stream/drive/ms 49.19, render/thread/ms 40.16, render/busy/ms 38.59, render/graph/ms 3.72, stream/spawn/ms 2.26, render/present/ms 1.21, stream/tick/ms 1.02, terrain/update/ms 0.96, terrain/select/ms 0.77, stream/activate/ms 0.67, bigspace/total/ms 0.37 |
| warm-up | 4.71 | 86.7 | main/busy/ms 77.35, stream/drive/ms 49.31, render/thread/ms 39.88, render/busy/ms 38.26, render/graph/ms 6.73, stream/spawn/ms 2.33, render/present/ms 1.38, stream/tick/ms 1.12, terrain/update/ms 0.89, terrain/select/ms 0.74, bigspace/total/ms 0.60, stream/upload/ms 0.31 |
| warm-up | 4.79 | 57.4 | main/busy/ms 55.03, stream/drive/ms 40.26, render/thread/ms 35.40, render/busy/ms 34.11, render/graph/ms 6.55, stream/spawn/ms 1.71, stream/activate/ms 1.07, stream/tick/ms 1.05, render/present/ms 1.03, terrain/update/ms 0.84, terrain/select/ms 0.67, physics/insert/ms 0.32 |
| warm-up | 4.83 | 31.5 | main/busy/ms 38.72, stream/drive/ms 26.67, render/thread/ms 20.76, render/busy/ms 18.83, render/graph/ms 5.62, render/present/ms 1.63, stream/activate/ms 1.10, stream/spawn/ms 0.90, stream/tick/ms 0.87, terrain/update/ms 0.78, terrain/select/ms 0.62, physics/insert/ms 0.36 |
| warm-up | 4.96 | 29.3 | render/thread/ms 36.93, render/busy/ms 35.46, main/busy/ms 26.12, stream/drive/ms 5.65, render/graph/ms 4.66, stream/spawn/ms 3.02, terrain/update/ms 1.12, render/present/ms 1.09, stream/tick/ms 0.98, terrain/select/ms 0.88, bigspace/total/ms 0.50, render/acquire/ms 0.38 |
| warm-up | 5.24 | 30.8 | main/busy/ms 37.95, render/thread/ms 25.63, render/busy/ms 23.92, stream/drive/ms 17.44, render/graph/ms 6.54, stream/tick/ms 1.33, render/present/ms 1.27, terrain/update/ms 0.85, stream/spawn/ms 0.76, terrain/select/ms 0.70, bigspace/total/ms 0.54, render/acquire/ms 0.43 |
| warm-up | 5.46 | 27.5 | main/busy/ms 35.01, render/thread/ms 24.83, render/busy/ms 22.68, stream/drive/ms 16.90, render/graph/ms 5.86, render/present/ms 1.76, stream/spawn/ms 1.04, stream/tick/ms 0.97, terrain/update/ms 0.62, terrain/select/ms 0.50, render/acquire/ms 0.39, stream/activate/ms 0.36 |
| warm-up | 5.65 | 31.8 | pipelines/compile_ms 2094, pipelines/driver_ms 2024, pipelines/wait_ms 219, pipelines/front_ms 69.46, render/thread/ms 45.24, render/busy/ms 43.22, main/busy/ms 31.25, stream/drive/ms 6.83, render/graph/ms 5.97, stream/spawn/ms 2.93, render/present/ms 1.69, terrain/update/ms 1.57 |
| warm-up | 5.68 | 31.7 | pipelines/compile_ms 2094, pipelines/driver_ms 2024, pipelines/wait_ms 219, pipelines/front_ms 69.46, render/thread/ms 56.23, render/busy/ms 53.88, main/busy/ms 36.32, render/graph/ms 8.51, stream/drive/ms 5.96, stream/spawn/ms 3.00, render/present/ms 2.01, stream/tick/ms 1.39 |
| warm-up | 5.73 | 52.6 | render/thread/ms 71.09, render/busy/ms 69.01, main/busy/ms 59.50, render/graph/ms 8.66, stream/drive/ms 5.35, stream/spawn/ms 2.26, render/present/ms 1.72, stream/tick/ms 1.55, terrain/update/ms 1.07, terrain/select/ms 0.76, bigspace/total/ms 0.61, render/acquire/ms 0.37 |
| warm-up | 5.78 | 48.0 | render/thread/ms 79.32, render/busy/ms 76.89, main/busy/ms 64.06, render/graph/ms 7.72, stream/drive/ms 4.99, render/present/ms 2.16, stream/spawn/ms 2.12, stream/tick/ms 1.15, terrain/update/ms 0.90, terrain/select/ms 0.74, bigspace/total/ms 0.63, render/acquire/ms 0.28 |
| warm-up | 5.83 | 53.9 | pipelines/wait_ms 1533, render/thread/ms 83.65, render/busy/ms 80.35, pipelines/compile_ms 67.12, pipelines/driver_ms 45.80, main/busy/ms 45.51, pipelines/front_ms 21.31, render/graph/ms 9.08, stream/drive/ms 5.09, render/present/ms 3.02, stream/spawn/ms 2.01, stream/tick/ms 1.13 |
| warm-up | 5.86 | 29.8 | pipelines/wait_ms 3716, pipelines/compile_ms 1523, pipelines/driver_ms 1323, pipelines/front_ms 200, render/thread/ms 74.40, render/busy/ms 70.52, main/busy/ms 43.60, render/graph/ms 12.12, stream/drive/ms 6.09, render/present/ms 3.53, stream/spawn/ms 2.33, stream/tick/ms 1.98 |
| warm-up | 5.91 | 43.6 | pipelines/wait_ms 2183, pipelines/compile_ms 1456, pipelines/driver_ms 1278, pipelines/front_ms 179, render/thread/ms 67.16, render/busy/ms 61.82, main/busy/ms 53.09, render/graph/ms 10.71, stream/drive/ms 5.81, render/present/ms 2.76, render/acquire/ms 2.57, stream/spawn/ms 2.39 |
| warm-up | 5.96 | 34.2 | pipelines/wait_ms 5761, pipelines/compile_ms 175, pipelines/driver_ms 113, pipelines/front_ms 61.41, render/thread/ms 50.97, render/busy/ms 48.89, main/busy/ms 31.16, render/graph/ms 8.04, stream/drive/ms 4.93, stream/spawn/ms 2.20, render/present/ms 1.67, stream/tick/ms 1.08 |
| warm-up | 6.02 | 44.6 | render/thread/ms 49.79, render/busy/ms 48.17, main/busy/ms 34.48, stream/drive/ms 10.00, render/graph/ms 6.54, stream/spawn/ms 2.08, render/present/ms 1.38, terrain/update/ms 1.02, terrain/select/ms 0.83, stream/tick/ms 0.74, bigspace/total/ms 0.74, bigspace/high_precision/ms 0.35 |
| warm-up | 6.16 | 54.7 | pipelines/wait_ms 402, main/busy/ms 61.81, render/thread/ms 53.46, render/busy/ms 51.47, pipelines/compile_ms 9.35, pipelines/front_ms 8.59, render/graph/ms 7.15, stream/drive/ms 4.75, stream/spawn/ms 2.66, physics/insert/ms 2.15, render/present/ms 1.68, stream/tick/ms 0.89 |
| warm-up | 6.19 | 30.9 | pipelines/wait_ms 402, render/thread/ms 52.86, render/busy/ms 51.37, main/busy/ms 44.24, pipelines/compile_ms 9.35, pipelines/front_ms 8.59, render/graph/ms 5.11, stream/drive/ms 4.47, stream/spawn/ms 2.10, physics/insert/ms 1.63, stream/tick/ms 1.25, render/present/ms 1.24 |
| warm-up | 6.24 | 29.5 | pipelines/wait_ms 2675, pipelines/compile_ms 146, pipelines/driver_ms 114, main/busy/ms 41.49, pipelines/front_ms 32.37, render/thread/ms 31.35, render/busy/ms 29.52, stream/drive/ms 17.43, render/graph/ms 7.80, stream/spawn/ms 2.22, render/present/ms 1.55, physics/insert/ms 1.20 |
| warm-up | 6.29 | 26.8 | render/thread/ms 32.15, render/busy/ms 30.84, main/busy/ms 25.86, stream/drive/ms 7.90, render/graph/ms 4.27, stream/spawn/ms 2.19, stream/tick/ms 1.16, render/present/ms 1.10, terrain/update/ms 1.02, physics/insert/ms 1.00, terrain/select/ms 0.85, bigspace/total/ms 0.48 |
| warm-up | 11.67 | 30.0 | main/busy/ms 50.29, render/thread/ms 35.07, render/busy/ms 33.65, render/graph/ms 8.31, render/present/ms 1.16, bigspace/total/ms 0.85, physics/insert/ms 0.74, terrain/update/ms 0.59, stream/drive/ms 0.58, stream/tick/ms 0.51, terrain/select/ms 0.49, anim/evaluate_cpu/ms 0.49 |
| warm-up | 11.71 | 41.0 | main/busy/ms 54.63, render/thread/ms 40.72, render/busy/ms 39.08, render/graph/ms 6.82, render/present/ms 1.32, bigspace/total/ms 0.81, physics/insert/ms 0.75, anim/evaluate_cpu/ms 0.63, terrain/update/ms 0.62, stream/drive/ms 0.62, anim/evaluate/ms 0.58, stream/tick/ms 0.55 |
| measure | 0.79 | 30.2 | main/busy/ms 36.88, render/thread/ms 27.76, render/busy/ms 26.44, stream/drive/ms 14.78, render/graph/ms 10.06, render/present/ms 1.04, physics/insert/ms 0.91, terrain/update/ms 0.85, terrain/select/ms 0.76, bigspace/total/ms 0.64, far/select/ms 0.63, stream/tick/ms 0.50 |
| measure | 2.57 | 36.9 | pipelines/wait_ms 19.6 k, render/thread/ms 39.87, render/busy/ms 38.29, main/busy/ms 34.94, pipelines/compile_ms 13.02, pipelines/front_ms 12.51, render/graph/ms 8.23, render/present/ms 1.19, physics/insert/ms 0.98, bigspace/total/ms 0.82, stream/drive/ms 0.63, terrain/update/ms 0.56 |
| measure | 18.18 | 32.7 | main/busy/ms 38.71, stream/drive/ms 22.23, render/thread/ms 21.42, render/busy/ms 20.31, render/graph/ms 10.86, render/present/ms 0.93, bigspace/total/ms 0.71, terrain/update/ms 0.57, controller/feed/ms 0.55, terrain/select/ms 0.51, far/select/ms 0.40, anim/evaluate_cpu/ms 0.36 |
| measure | 19.27 | 30.3 | render/thread/ms 41.28, render/busy/ms 39.90, render/graph/ms 23.48, main/busy/ms 19.08, render/present/ms 1.21, terrain/update/ms 1.04, physics/insert/ms 0.98, terrain/select/ms 0.98, controller/feed/ms 0.82, bigspace/total/ms 0.68, stream/drive/ms 0.46, stream/tick/ms 0.40 |
| measure | 23.11 | 29.4 | render/thread/ms 46.55, render/busy/ms 45.04, main/busy/ms 36.49, render/graph/ms 16.26, bigspace/total/ms 1.37, stream/drive/ms 1.22, render/present/ms 1.21, controller/feed/ms 1.17, bigspace/low_precision/ms 0.67, terrain/update/ms 0.66, anim/evaluate_cpu/ms 0.66, stream/tick/ms 0.61 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **71%** (min 40%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.2 W · 75 °C (max 80) · VRAM 5995 MiB · throttled 100% of samples
