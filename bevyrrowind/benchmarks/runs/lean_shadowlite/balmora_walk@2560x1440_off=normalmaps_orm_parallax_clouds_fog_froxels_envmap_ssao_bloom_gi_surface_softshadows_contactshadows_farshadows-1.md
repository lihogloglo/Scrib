# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5881 frames over 65.6 s · **median 10.85 ms (92 fps)** · mean 11.16 · p95 14.12 · p99 16.48 (1% low 61 fps) · worst 38.01 ms · 64 spikes over 1.5× median · 25.6 stutters/min (work 26.5/min)
- warm-up: 1059 frames over 16.9 s, worst 2819.5 ms
- hitches (> 27.1 ms): **5** measured, 32 in warm-up
- timed GPU passes: 5.48 ms of a 11.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.85 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- **FAIL** worst frame 38.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.055 | 2.057 | 2.083 | 1.00 | 0.185 |
| main_opaque_pass_3d | 0.961 | 0.962 | 1.374 | 1.00 | 0.066 |
| early prepass | 0.415 | 0.415 | 0.627 | 1.00 | 0.099 |
| atmosphere_luts | 0.311 | 0.311 | 0.370 | 1.00 | 0.003 |
| water_velocity | 0.263 | 0.263 | 0.349 | 1.00 | 0.002 |
| uniform_allocation | 0.217 | 0.217 | 0.234 | 1.00 | 0.045 |
| water_sspr | 0.209 | 0.209 | 0.210 | 1.00 | 0.002 |
| bin_unpacking | 0.167 | 0.167 | 0.190 | 1.00 | 0.031 |
| auxiliary_views | 0.155 | 0.155 | 0.607 | 1.00 | 0.161 |
| upscaling | 0.146 | 0.147 | 0.177 | 1.00 | 0.005 |
| water_fft | 0.130 | 0.130 | 0.136 | 1.00 | 0.003 |
| tonemapping | 0.120 | 0.121 | 0.130 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.076 | 0.076 | 0.124 | 1.00 | 0.009 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.019 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.020 | 1.00 | 0.079 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.970 | 12.750 | 27.11 | 58634.8 |
| render/busy/ms | 9.414 | 12.135 | 26.07 | 55364.9 |
| main/busy/ms | 8.783 | 11.031 | 30.49 | 51654.6 |
| pipelines/wait_ms | 7.275 | 0.000 | 17520.37 | 42782.7 |
| render/graph/ms | 5.253 | 6.939 | 14.94 | 30894.7 |
| pipelines/compile_ms | 1.604 | 0.000 | 1746.51 | 9434.6 |
| pipelines/driver_ms | 1.403 | 0.000 | 1630.43 | 8253.4 |
| bigspace/total/ms | 0.585 | 1.666 | 3.03 | 3437.7 |
| render/present/ms | 0.462 | 0.585 | 2.35 | 2718.0 |
| terrain/update/ms | 0.312 | 0.411 | 2.40 | 1835.1 |
| controller/feed/ms | 0.298 | 0.414 | 0.88 | 1753.2 |
| terrain/select/ms | 0.277 | 0.363 | 0.97 | 1627.4 |
| stream/drive/ms | 0.263 | 0.366 | 22.05 | 1546.9 |
| bigspace/low_precision/ms | 0.235 | 0.654 | 2.81 | 1382.3 |
| stream/tick/ms | 0.205 | 0.285 | 0.69 | 1202.9 |
| far/select/ms | 0.202 | 0.297 | 0.69 | 1187.9 |
| pipelines/front_ms | 0.201 | 0.000 | 265.64 | 1181.2 |
| bigspace/high_precision/ms | 0.181 | 0.674 | 2.69 | 1063.2 |
| anim/evaluate_cpu/ms | 0.162 | 0.215 | 0.48 | 951.2 |
| anim/evaluate/ms | 0.160 | 0.306 | 2.04 | 938.0 |
| controller/move/ms | 0.135 | 0.305 | 0.56 | 795.9 |
| render/acquire/ms | 0.094 | 0.128 | 0.43 | 551.9 |
| physics/step/ms | 0.050 | 0.094 | 0.65 | 294.6 |
| physics/insert/ms | 0.034 | 0.362 | 1.10 | 201.1 |
| anim/apply/ms | 0.012 | 0.023 | 0.11 | 70.7 |
| weather/eval/ms | 0.011 | 0.016 | 0.07 | 67.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.28 | 64.1 |
| script/update/ms | 0.008 | 0.012 | 0.17 | 45.9 |
| script/input/ms | 0.005 | 0.008 | 0.07 | 31.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 29.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 27.5 |
| script/ui/ms | 0.005 | 0.007 | 0.16 | 27.3 |
| controller/write/ms | 0.004 | 0.005 | 0.12 | 20.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.17 | 2.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 149 |
| crowd/c1/joints/count | count | 5535 | 5889 | 5661 |
| crowd/c1/meshes/count | count | 1166 | 1254 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4254 | 9951 | 6179 |
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
| frame/work/ms | ms | 9.29 | 30.49 | 7.65 |
| gpu/timed/ms | ms | 5.52 | 8.68 | 5.77 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5134 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5134 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 301 | 44 |
| phases/shadow/bins | count | 3804 | 5485 | 2264 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9686 | 12.6 k | 5765 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 325 | 44 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 202 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 193 | 198 | 198 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 10 | 771 | 10 |
| preprocess/bind_groups_created | count | 10 | 771 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 232 | 350 | 221 |
| process/cpu_usage | % | 25.89 | 50.37 | 23.08 |
| process/mem_usage | GiB | 4.82 | 4.99 | 4.98 |
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
| started_load_count |  | 4751 | 4845 | 4845 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.13 | 77.44 | 30.10 |
| system/mem_usage | % | 68.47 | 73.91 | 73.59 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2805 | 2921 | 2921 |
| textures/images | count | 2580 | 2640 | 2640 |
| views/all/batch_sets | count | 113 | 353 | 104 |
| views/all/bins | count | 6519 | 8359 | 5744 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.3 k | 13.4 k |
| views/all/pipelines | count | 194 | 372 | 100 |
| views/all/pipelines_distinct | count | 85 | 95 | 80 |
| views/all/work_items | count | 17.5 k | 24.0 k | 13.7 k |
| views/cascade/batch_sets | count | 52 | 62 | 44 |
| views/cascade/bins | count | 2622 | 3000 | 2264 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8246 | 9740 | 5765 |
| views/cascade/pipelines | count | 52 | 62 | 44 |
| views/cascade/work_items | count | 8246 | 9740 | 5765 |
| views/lamp_face/batch_sets | count | 0 | 245 | 0 |
| views/lamp_face/bins | count | 1074 | 2621 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1203 | 2950 | 0 |
| views/lamp_face/pipelines | count | 86 | 265 | 0 |
| views/lamp_face/work_items | count | 0 | 2599 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2880 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8428 | 13.0 k | 7600 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8589 | 13.3 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 61.26 | 309 | 360.3 k |
| anim/events/count | count | 0.09 | 3 | 552 |
| assets/Image/added | count | 0.10 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 9 | 22 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.69 | 152 | 4075 |
| assets/Mesh/removed | count | 0.83 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.70 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.39 | 74 | 2305 |
| ecs/entities_delta | count | 0.86 | 1495 | 5053 |
| physics/inserts | count | 0.38 | 106 | 2250 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 3 | 3939 |
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
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5756 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1262 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.82 | 2819.5 | main/busy/ms 2794, surface/advance_cpu_ms 71.33, surface/cell_upload_ms 1.01, stream/drive/ms 0.32, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.59 | 767.0 | main/busy/ms 2830, render/thread/ms 734, render/busy/ms 574, render/acquire/ms 157, surface/advance_cpu_ms 71.33, render/graph/ms 6.03, render/present/ms 3.12, stream/drive/ms 1.05, surface/cell_upload_ms 1.01, terrain/update/ms 0.80, terrain/select/ms 0.58, stream/tick/ms 0.52 |
| warm-up | 3.79 | 205.3 | pipelines/wait_ms 1396, render/thread/ms 749, render/busy/ms 587, render/acquire/ms 157, main/busy/ms 56.13, pipelines/compile_ms 25.23, pipelines/front_ms 22.43, terrain/update/ms 20.85, render/graph/ms 9.25, render/present/ms 4.27, pipelines/driver_ms 2.80, stream/drive/ms 1.69 |
| warm-up | 4.05 | 146.7 | render/thread/ms 116, render/busy/ms 111, main/busy/ms 92.36, terrain/update/ms 7.27, render/graph/ms 6.30, stream/drive/ms 4.00, render/acquire/ms 3.18, stream/tick/ms 2.27, render/present/ms 1.93, bigspace/total/ms 1.03, terrain/select/ms 0.84, stream/spawn/ms 0.67 |
| warm-up | 4.10 | 43.8 | pipelines/wait_ms 862, render/thread/ms 43.09, render/busy/ms 41.31, main/busy/ms 24.81, pipelines/compile_ms 17.82, pipelines/front_ms 16.32, render/graph/ms 4.35, stream/drive/ms 3.01, stream/tick/ms 1.98, render/present/ms 1.56, pipelines/driver_ms 1.50, terrain/update/ms 1.26 |
| warm-up | 4.24 | 67.6 | pipelines/wait_ms 1502, pipelines/compile_ms 106, pipelines/front_ms 103, render/thread/ms 53.26, render/busy/ms 51.64, main/busy/ms 45.47, stream/drive/ms 28.99, render/graph/ms 3.56, pipelines/driver_ms 3.42, render/present/ms 1.18, stream/tick/ms 1.17, terrain/update/ms 1.09 |
| warm-up | 4.26 | 28.7 | pipelines/wait_ms 3397, pipelines/compile_ms 110, pipelines/front_ms 104, render/thread/ms 58.70, render/busy/ms 57.13, main/busy/ms 52.00, stream/drive/ms 34.99, pipelines/driver_ms 5.88, render/graph/ms 3.15, render/present/ms 1.35, stream/tick/ms 1.33, terrain/update/ms 0.91 |
| warm-up | 4.31 | 49.6 | pipelines/wait_ms 6956, pipelines/compile_ms 76.75, pipelines/front_ms 69.33, main/busy/ms 57.49, render/thread/ms 46.06, render/busy/ms 44.12, stream/drive/ms 42.02, pipelines/driver_ms 7.43, render/graph/ms 4.22, render/present/ms 1.70, stream/spawn/ms 1.09, stream/tick/ms 0.87 |
| warm-up | 4.34 | 28.8 | pipelines/wait_ms 4745, render/thread/ms 49.00, render/busy/ms 46.55, pipelines/compile_ms 40.53, pipelines/front_ms 36.93, main/busy/ms 33.25, stream/drive/ms 10.95, render/graph/ms 5.85, pipelines/driver_ms 3.60, render/present/ms 2.12, stream/spawn/ms 1.32, stream/tick/ms 0.94 |
| warm-up | 4.41 | 64.0 | render/thread/ms 76.44, render/busy/ms 73.65, main/busy/ms 32.71, render/graph/ms 8.12, stream/drive/ms 5.04, render/present/ms 2.36, stream/tick/ms 1.47, stream/spawn/ms 1.27, terrain/update/ms 1.00, terrain/select/ms 0.80, stream/activate/ms 0.61, render/acquire/ms 0.43 |
| warm-up | 4.45 | 41.1 | pipelines/wait_ms 1108, pipelines/compile_ms 111, pipelines/front_ms 102, render/thread/ms 76.98, render/busy/ms 74.65, main/busy/ms 30.92, pipelines/driver_ms 8.67, render/graph/ms 7.64, stream/drive/ms 7.22, stream/spawn/ms 3.11, render/present/ms 1.95, stream/tick/ms 1.57 |
| warm-up | 4.57 | 122.9 | pipelines/wait_ms 7452, pipelines/compile_ms 226, pipelines/front_ms 211, render/thread/ms 107, render/busy/ms 75.40, main/busy/ms 34.09, render/present/ms 31.50, pipelines/driver_ms 14.97, stream/drive/ms 6.59, render/graph/ms 4.45, stream/spawn/ms 3.11, stream/tick/ms 1.33 |
| warm-up | 4.63 | 61.9 | pipelines/wait_ms 12.5 k, pipelines/compile_ms 201, pipelines/front_ms 170, render/thread/ms 127, render/busy/ms 94.84, main/busy/ms 63.96, stream/drive/ms 35.63, render/present/ms 31.68, pipelines/driver_ms 30.85, render/graph/ms 6.22, stream/spawn/ms 2.12, terrain/update/ms 1.18 |
| warm-up | 4.66 | 29.5 | pipelines/wait_ms 11.0 k, pipelines/compile_ms 115, pipelines/front_ms 84.71, render/thread/ms 63.73, render/busy/ms 61.63, main/busy/ms 55.15, stream/drive/ms 34.87, pipelines/driver_ms 29.87, render/graph/ms 10.57, render/present/ms 1.90, stream/spawn/ms 1.39, stream/tick/ms 1.30 |
| warm-up | 4.72 | 40.2 | main/busy/ms 43.46, render/thread/ms 34.10, render/busy/ms 32.57, stream/drive/ms 26.29, render/graph/ms 8.05, terrain/update/ms 4.01, render/present/ms 1.31, stream/tick/ms 0.92, terrain/select/ms 0.85, stream/activate/ms 0.54, physics/insert/ms 0.35, bigspace/total/ms 0.28 |
| warm-up | 4.78 | 40.5 | pipelines/wait_ms 1258, pipelines/compile_ms 71.02, main/busy/ms 43.17, pipelines/front_ms 39.42, render/thread/ms 38.82, render/busy/ms 37.19, pipelines/driver_ms 31.60, stream/drive/ms 29.52, render/graph/ms 13.96, render/present/ms 1.35, stream/spawn/ms 1.19, stream/tick/ms 1.01 |
| warm-up | 4.81 | 27.9 | pipelines/wait_ms 1931, pipelines/compile_ms 82.73, main/busy/ms 67.69, stream/drive/ms 52.14, pipelines/front_ms 50.56, render/thread/ms 46.17, render/busy/ms 44.66, pipelines/driver_ms 32.17, render/graph/ms 18.59, stream/spawn/ms 2.41, render/present/ms 1.29, terrain/update/ms 0.79 |
| warm-up | 4.87 | 61.8 | pipelines/wait_ms 674, main/busy/ms 78.99, stream/drive/ms 64.45, render/thread/ms 45.55, render/busy/ms 43.20, render/graph/ms 16.21, pipelines/compile_ms 11.71, pipelines/front_ms 11.15, stream/spawn/ms 2.24, render/present/ms 2.11, terrain/update/ms 1.08, terrain/select/ms 0.91 |
| warm-up | 4.93 | 40.9 | main/busy/ms 58.70, stream/drive/ms 40.05, render/thread/ms 23.02, render/busy/ms 21.27, render/graph/ms 5.04, stream/spawn/ms 2.08, render/present/ms 1.55, stream/tick/ms 0.85, terrain/update/ms 0.59, terrain/select/ms 0.49, bigspace/total/ms 0.28, render/acquire/ms 0.20 |
| warm-up | 4.97 | 37.9 | main/busy/ms 56.50, stream/drive/ms 34.62, render/thread/ms 33.48, render/busy/ms 31.15, render/graph/ms 6.34, stream/spawn/ms 2.09, render/present/ms 2.08, stream/tick/ms 0.87, terrain/update/ms 0.69, terrain/select/ms 0.56, bigspace/total/ms 0.31, render/acquire/ms 0.24 |
| warm-up | 5.00 | 30.7 | main/busy/ms 43.76, render/thread/ms 39.20, render/busy/ms 37.23, stream/drive/ms 21.87, render/graph/ms 5.81, stream/spawn/ms 1.74, render/present/ms 1.55, stream/tick/ms 0.84, terrain/update/ms 0.82, terrain/select/ms 0.67, stream/activate/ms 0.54, bigspace/total/ms 0.47 |
| warm-up | 5.57 | 29.8 | main/busy/ms 48.18, render/thread/ms 24.28, render/busy/ms 22.66, render/graph/ms 7.40, stream/drive/ms 5.19, render/present/ms 1.40, physics/insert/ms 1.09, stream/tick/ms 0.89, physics/step/ms 0.70, stream/spawn/ms 0.69, terrain/update/ms 0.60, terrain/select/ms 0.50 |
| warm-up | 5.60 | 31.0 | main/busy/ms 47.08, render/thread/ms 28.20, render/busy/ms 26.58, stream/drive/ms 14.25, render/graph/ms 5.69, render/present/ms 1.42, stream/tick/ms 1.15, stream/spawn/ms 0.74, physics/step/ms 0.73, physics/insert/ms 0.69, terrain/update/ms 0.58, bigspace/total/ms 0.54 |
| warm-up | 5.79 | 32.6 | render/thread/ms 43.93, render/busy/ms 42.15, main/busy/ms 23.54, render/graph/ms 6.09, stream/drive/ms 4.77, stream/spawn/ms 2.81, render/present/ms 1.44, stream/tick/ms 0.83, bigspace/total/ms 0.70, terrain/update/ms 0.66, terrain/select/ms 0.56, render/acquire/ms 0.33 |
| warm-up | 5.85 | 38.3 | pipelines/wait_ms 3441, render/thread/ms 49.13, render/busy/ms 47.84, main/busy/ms 27.90, pipelines/compile_ms 26.48, pipelines/front_ms 25.14, render/graph/ms 6.24, stream/drive/ms 4.60, stream/spawn/ms 2.06, stream/tick/ms 1.51, pipelines/driver_ms 1.34, render/present/ms 1.06 |
| warm-up | 6.20 | 37.9 | main/busy/ms 44.20, render/thread/ms 32.02, render/busy/ms 30.26, stream/drive/ms 18.88, render/graph/ms 9.42, stream/spawn/ms 2.86, stream/tick/ms 1.54, render/present/ms 1.44, bigspace/total/ms 0.84, terrain/update/ms 0.72, terrain/select/ms 0.59, bigspace/high_precision/ms 0.40 |
| warm-up | 6.43 | 28.9 | render/thread/ms 47.65, render/busy/ms 45.76, main/busy/ms 29.59, render/graph/ms 8.83, stream/drive/ms 3.89, stream/spawn/ms 2.05, render/present/ms 1.59, stream/tick/ms 0.96, terrain/update/ms 0.87, bigspace/total/ms 0.87, terrain/select/ms 0.70, bigspace/high_precision/ms 0.38 |
| warm-up | 9.88 | 27.7 | main/busy/ms 38.33, render/thread/ms 31.76, render/busy/ms 29.92, render/graph/ms 11.59, render/present/ms 1.57, terrain/update/ms 1.40, physics/insert/ms 0.86, bigspace/total/ms 0.85, stream/drive/ms 0.83, stream/tick/ms 0.74, terrain/select/ms 0.73, far/select/ms 0.50 |
| warm-up | 9.91 | 33.8 | main/busy/ms 43.60, render/thread/ms 40.28, render/busy/ms 38.64, render/graph/ms 9.12, render/present/ms 1.32, physics/insert/ms 0.98, bigspace/total/ms 0.93, stream/drive/ms 0.83, terrain/update/ms 0.83, stream/tick/ms 0.74, terrain/select/ms 0.68, far/select/ms 0.60 |
| warm-up | 9.99 | 58.8 | main/busy/ms 68.25, render/thread/ms 40.49, render/busy/ms 38.83, render/graph/ms 7.72, render/present/ms 1.33, bigspace/total/ms 1.11, physics/insert/ms 0.99, anim/evaluate_cpu/ms 0.95, stream/drive/ms 0.90, terrain/update/ms 0.84, stream/tick/ms 0.81, terrain/select/ms 0.67 |
| warm-up | 12.86 | 35.9 | pipelines/wait_ms 15.7 k, render/thread/ms 40.95, render/busy/ms 39.51, main/busy/ms 37.12, pipelines/compile_ms 11.86, pipelines/front_ms 11.30, render/graph/ms 9.61, render/present/ms 1.18, physics/insert/ms 1.03, bigspace/total/ms 0.95, terrain/update/ms 0.68, physics/step/ms 0.68 |
| warm-up | 15.43 | 27.8 | pipelines/wait_ms 9882, main/busy/ms 34.49, render/thread/ms 32.03, render/busy/ms 30.60, pipelines/compile_ms 13.38, render/graph/ms 13.32, pipelines/front_ms 12.94, render/present/ms 1.21, bigspace/total/ms 0.86, anim/evaluate_cpu/ms 0.81, physics/insert/ms 0.67, terrain/update/ms 0.62 |
| measure | 0.77 | 27.2 | main/busy/ms 34.29, render/thread/ms 26.16, render/busy/ms 24.91, stream/drive/ms 15.40, render/graph/ms 9.82, render/present/ms 0.92, bigspace/total/ms 0.88, controller/feed/ms 0.66, far/select/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.50, stream/tick/ms 0.47 |
| measure | 18.19 | 32.0 | main/busy/ms 39.86, stream/drive/ms 22.29, render/thread/ms 22.00, render/busy/ms 20.84, render/graph/ms 11.16, render/present/ms 0.99, bigspace/total/ms 0.84, controller/feed/ms 0.80, terrain/update/ms 0.67, terrain/select/ms 0.60, anim/evaluate_cpu/ms 0.49, far/select/ms 0.48 |
| measure | 19.26 | 28.4 | render/thread/ms 37.54, render/busy/ms 35.92, render/graph/ms 20.55, main/busy/ms 18.59, render/present/ms 1.43, bigspace/total/ms 1.09, controller/feed/ms 0.65, physics/insert/ms 0.63, terrain/update/ms 0.57, far/select/ms 0.54, terrain/select/ms 0.52, stream/drive/ms 0.46 |
| measure | 59.04 | 38.0 | render/thread/ms 37.97, render/busy/ms 36.23, main/busy/ms 26.01, render/graph/ms 13.17, bigspace/total/ms 1.95, render/present/ms 1.42, stream/drive/ms 1.03, controller/feed/ms 0.99, stream/tick/ms 0.92, terrain/update/ms 0.80, bigspace/low_precision/ms 0.73, terrain/select/ms 0.70 |
| measure | 59.08 | 36.8 | render/thread/ms 41.16, render/busy/ms 39.84, main/busy/ms 22.32, render/graph/ms 8.22, bigspace/total/ms 1.08, render/present/ms 1.04, stream/drive/ms 0.95, stream/tick/ms 0.85, controller/feed/ms 0.83, terrain/update/ms 0.82, terrain/select/ms 0.72, far/select/ms 0.47 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **66%** (min 41%) · 2580 MHz (min 2580) · memory 8001 MHz · 68.6 W · 73 °C (max 79) · VRAM 6025 MiB · throttled 100% of samples
