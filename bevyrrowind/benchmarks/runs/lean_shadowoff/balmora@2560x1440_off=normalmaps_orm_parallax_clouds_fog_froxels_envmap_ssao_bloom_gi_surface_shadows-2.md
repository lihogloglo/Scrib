# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1885 frames over 15.0 s · **median 7.77 ms (129 fps)** · mean 7.96 · p95 9.76 · p99 10.79 (1% low 93 fps) · worst 21.46 ms · 7 spikes over 1.5× median · 20.0 stutters/min (work 20.0/min)
- warm-up: 304 frames over 7.7 s, worst 2781.9 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 5.88 ms of a 7.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.77 ms, limit 11 ms
- pass worst frame 21.46 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.934 | 2.092 | 2.433 | 0.92 | 0.186 |
| main_opaque_pass_3d | 1.668 | 1.805 | 2.184 | 0.92 | 0.085 |
| early prepass | 0.492 | 0.532 | 0.788 | 0.92 | 0.108 |
| atmosphere_luts | 0.348 | 0.377 | 0.599 | 0.92 | 0.004 |
| water_velocity | 0.230 | 0.249 | 0.266 | 0.92 | 0.002 |
| main_transmissive_pass_3d | 0.207 | 0.224 | 0.241 | 0.92 | 0.010 |
| water_sspr | 0.204 | 0.221 | 0.222 | 0.92 | 0.003 |
| water_fft | 0.129 | 0.139 | 0.141 | 0.92 | 0.003 |
| upscaling | 0.124 | 0.134 | 0.166 | 0.92 | 0.005 |
| uniform_allocation | 0.123 | 0.133 | 0.372 | 0.92 | 0.027 |
| tonemapping | 0.111 | 0.120 | 0.127 | 0.92 | 0.005 |
| bin_unpacking | 0.090 | 0.097 | 0.101 | 0.92 | 0.019 |
| clustering | 0.060 | 0.065 | 0.071 | 0.92 | 0.024 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.92 | 0.002 |
| water_snapshot | 0.034 | 0.037 | 0.035 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.025 | 0.027 | 0.030 | 0.92 | 0.005 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.017 | 0.019 | 0.020 | 0.92 | 0.061 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.92 | 0.004 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.92 | 0.002 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.004 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 32.904 | 0.000 | 11701.39 | 62024.0 |
| render/thread/ms | 7.007 | 8.498 | 13.67 | 13208.5 |
| main/busy/ms | 6.582 | 8.065 | 15.83 | 12407.5 |
| render/busy/ms | 6.353 | 7.779 | 13.05 | 11975.9 |
| render/graph/ms | 3.563 | 4.571 | 5.96 | 6715.9 |
| pipelines/compile_ms | 0.818 | 0.000 | 161.86 | 1542.3 |
| pipelines/front_ms | 0.792 | 0.000 | 155.88 | 1492.2 |
| render/present/ms | 0.533 | 0.998 | 3.68 | 1005.2 |
| bigspace/total/ms | 0.463 | 1.174 | 1.86 | 872.5 |
| terrain/update/ms | 0.239 | 0.311 | 0.58 | 450.0 |
| stream/drive/ms | 0.234 | 0.325 | 0.59 | 441.2 |
| terrain/select/ms | 0.207 | 0.267 | 0.54 | 389.7 |
| stream/tick/ms | 0.202 | 0.288 | 0.55 | 381.0 |
| controller/move/ms | 0.193 | 0.457 | 0.67 | 363.1 |
| far/select/ms | 0.188 | 0.284 | 0.51 | 355.0 |
| bigspace/high_precision/ms | 0.184 | 0.823 | 1.66 | 347.2 |
| controller/feed/ms | 0.163 | 0.226 | 0.42 | 306.6 |
| anim/evaluate/ms | 0.131 | 0.432 | 1.54 | 247.7 |
| render/acquire/ms | 0.121 | 0.356 | 0.99 | 227.5 |
| bigspace/low_precision/ms | 0.097 | 0.240 | 1.55 | 183.1 |
| anim/evaluate_cpu/ms | 0.070 | 0.099 | 0.59 | 131.2 |
| physics/step/ms | 0.031 | 0.070 | 1.18 | 58.3 |
| pipelines/driver_ms | 0.027 | 0.000 | 5.99 | 50.1 |
| weather/eval/ms | 0.013 | 0.018 | 0.16 | 24.3 |
| physics/insert/ms | 0.013 | 0.021 | 0.72 | 24.0 |
| script/update/ms | 0.009 | 0.015 | 0.26 | 17.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 11.0 |
| script/input/ms | 0.005 | 0.007 | 0.12 | 10.3 |
| anim/apply/ms | 0.005 | 0.007 | 0.05 | 9.4 |
| script/ui/ms | 0.005 | 0.007 | 0.10 | 8.7 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 3.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.7 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 53 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 13 | 8 |
| crowd/c0/joints/count | count | 272 | 442 | 272 |
| crowd/c0/meshes/count | count | 48 | 78 | 48 |
| crowd/c1/count | count | 122 | 126 | 123 |
| crowd/c1/joints/count | count | 4848 | 4890 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1093 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 4666 | 2697 |
| crowd/joints_uploaded/count | count | 2697 | 2697 | 2697 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 6.67 | 15.83 | 6.50 |
| gpu/timed/ms | ms | 6.47 | 7.86 | 6.50 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1439 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 144 | 144 | 144 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 0 | 130 | 0 |
| preprocess/bind_groups_created | count | 0 | 130 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 25.71 | 36.84 | 30.21 |
| process/mem_usage | GiB | 3.94 | 4.03 | 4.03 |
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
| started_load_count |  | 3532 | 3534 | 3534 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 392 | 392 | 392 |
| statics/materials_bound | count | 392 | 392 | 392 |
| statics/materials_unclassed | count | 6 | 6 | 6 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.70 | 75.10 | 45.35 |
| system/mem_usage | % | 71.73 | 72.67 | 72.46 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 58 | 62 | 58 |
| views/all/bins | count | 3170 | 3172 | 3170 |
| views/all/count | count | 1 | 1 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 9282 | 9282 | 9282 |
| views/all/pipelines | count | 54 | 58 | 54 |
| views/all/pipelines_distinct | count | 54 | 58 | 54 |
| views/all/work_items | count | 9462 | 9462 | 9462 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
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
| views/main/batch_sets | count | 58 | 62 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 58 | 54 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.40 | 264 | 759 |
| anim/events/count | count | 0.08 | 3 | 154 |
| assets/Image/added | count | 0.03 | 20 | 56 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.07 | 58 | 138 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 22 | 60 |
| assets/StandardMaterial/modified | count | 2.63 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.70 | 1839 | 6975 |
| physics/inserts | count | 0.77 | 143 | 1456 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.48 | 3 | 902 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.78 | 2781.9 | main/busy/ms 2757, surface/advance_cpu_ms 94.22, surface/cell_upload_ms 0.74, stream/drive/ms 0.26, bigspace/total/ms 0.08, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, script/ui/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.65 | 869.3 | main/busy/ms 2794, render/thread/ms 832, render/busy/ms 735, render/acquire/ms 95.36, surface/advance_cpu_ms 94.22, render/graph/ms 5.19, render/present/ms 2.04, stream/drive/ms 0.89, surface/cell_upload_ms 0.74, stream/tick/ms 0.48, terrain/update/ms 0.39, terrain/select/ms 0.25 |
| warm-up | 3.93 | 96.6 | main/busy/ms 60.16, render/thread/ms 54.06, render/busy/ms 52.38, render/graph/ms 3.72, render/present/ms 1.47, stream/drive/ms 1.21, stream/tick/ms 0.97, terrain/update/ms 0.81, terrain/select/ms 0.62, bigspace/total/ms 0.24, render/acquire/ms 0.21, bigspace/high_precision/ms 0.12 |
| warm-up | 4.08 | 56.5 | render/thread/ms 46.85, render/busy/ms 45.14, main/busy/ms 44.10, stream/drive/ms 32.78, render/graph/ms 3.75, render/present/ms 1.50, stream/tick/ms 0.94, terrain/update/ms 0.73, terrain/select/ms 0.57, stream/spawn/ms 0.41, render/acquire/ms 0.20, bigspace/total/ms 0.20 |
| warm-up | 4.14 | 51.7 | render/thread/ms 35.99, render/busy/ms 34.00, main/busy/ms 30.56, stream/drive/ms 17.56, render/graph/ms 6.03, render/present/ms 1.67, stream/spawn/ms 1.35, stream/tick/ms 0.99, terrain/update/ms 0.72, terrain/select/ms 0.58, render/acquire/ms 0.32, bigspace/total/ms 0.30 |
| warm-up | 4.16 | 25.8 | render/thread/ms 47.00, render/busy/ms 44.50, main/busy/ms 35.29, stream/drive/ms 17.76, render/graph/ms 6.70, stream/spawn/ms 2.38, render/present/ms 2.18, stream/tick/ms 1.11, terrain/update/ms 0.66, terrain/select/ms 0.55, render/acquire/ms 0.33, bigspace/total/ms 0.28 |
| warm-up | 4.24 | 79.8 | pipelines/wait_ms 816, pipelines/compile_ms 193, pipelines/front_ms 173, render/thread/ms 54.09, render/busy/ms 52.15, main/busy/ms 43.63, pipelines/driver_ms 20.35, stream/drive/ms 18.51, render/graph/ms 5.31, stream/spawn/ms 2.90, render/present/ms 1.74, stream/tick/ms 1.45 |
| warm-up | 4.30 | 52.5 | pipelines/wait_ms 2491, pipelines/compile_ms 325, pipelines/front_ms 297, render/thread/ms 70.67, render/busy/ms 69.26, main/busy/ms 66.61, stream/drive/ms 42.21, pipelines/driver_ms 27.95, stream/spawn/ms 3.63, render/graph/ms 3.18, stream/tick/ms 1.35, render/present/ms 1.21 |
| warm-up | 4.39 | 95.7 | pipelines/wait_ms 2667, pipelines/compile_ms 158, pipelines/front_ms 149, render/thread/ms 90.80, render/busy/ms 63.56, main/busy/ms 62.98, stream/drive/ms 41.02, render/present/ms 27.04, pipelines/driver_ms 8.91, render/graph/ms 3.04, stream/spawn/ms 2.51, stream/tick/ms 1.06 |
| warm-up | 4.44 | 44.6 | pipelines/wait_ms 993, render/thread/ms 86.38, render/busy/ms 59.02, main/busy/ms 43.37, render/present/ms 27.00, pipelines/compile_ms 26.41, pipelines/front_ms 25.10, stream/drive/ms 21.65, render/graph/ms 3.06, stream/spawn/ms 1.91, pipelines/driver_ms 1.31, stream/tick/ms 1.11 |
| warm-up | 4.51 | 44.0 | pipelines/wait_ms 2919, main/busy/ms 51.89, pipelines/compile_ms 36.73, stream/drive/ms 34.01, pipelines/front_ms 33.98, render/thread/ms 16.73, render/busy/ms 15.47, terrain/update/ms 4.52, pipelines/driver_ms 2.74, render/graph/ms 2.52, render/present/ms 1.03, stream/tick/ms 0.70 |
| warm-up | 4.63 | 71.9 | pipelines/wait_ms 4008, pipelines/compile_ms 136, pipelines/front_ms 127, main/busy/ms 66.77, stream/drive/ms 56.35, render/thread/ms 22.71, render/busy/ms 21.49, pipelines/driver_ms 8.77, render/graph/ms 3.35, render/present/ms 1.01, stream/tick/ms 0.74, terrain/update/ms 0.72 |
| warm-up | 4.69 | 56.5 | pipelines/wait_ms 5091, pipelines/compile_ms 79.47, pipelines/front_ms 70.40, main/busy/ms 52.83, stream/drive/ms 38.13, render/thread/ms 17.81, render/busy/ms 16.54, pipelines/driver_ms 9.07, render/graph/ms 2.96, stream/spawn/ms 2.18, render/present/ms 1.09, terrain/update/ms 0.71 |
| warm-up | 4.73 | 25.8 | main/busy/ms 36.99, render/thread/ms 26.51, render/busy/ms 24.64, stream/drive/ms 11.12, render/graph/ms 5.15, stream/spawn/ms 2.01, render/present/ms 1.66, stream/tick/ms 1.02, terrain/update/ms 0.80, terrain/select/ms 0.60, bigspace/total/ms 0.42, render/acquire/ms 0.21 |
| warm-up | 4.76 | 25.7 | render/thread/ms 30.53, render/busy/ms 28.79, main/busy/ms 27.69, stream/drive/ms 10.81, render/graph/ms 4.22, stream/spawn/ms 1.93, render/present/ms 1.53, stream/tick/ms 0.88, terrain/update/ms 0.66, terrain/select/ms 0.50, bigspace/total/ms 0.27, render/acquire/ms 0.21 |
| warm-up | 4.80 | 31.1 | main/busy/ms 41.26, render/thread/ms 27.85, render/busy/ms 26.17, render/graph/ms 5.14, stream/drive/ms 4.30, stream/spawn/ms 1.41, render/present/ms 1.39, stream/activate/ms 1.07, stream/tick/ms 0.81, physics/insert/ms 0.76, terrain/update/ms 0.57, bigspace/total/ms 0.50 |
| warm-up | 4.86 | 36.8 | pipelines/wait_ms 5395, pipelines/compile_ms 195, pipelines/front_ms 179, main/busy/ms 37.81, render/thread/ms 30.93, render/busy/ms 29.30, pipelines/driver_ms 16.25, stream/drive/ms 15.85, render/graph/ms 3.64, stream/spawn/ms 1.67, render/present/ms 1.40, stream/activate/ms 1.30 |
| warm-up | 5.03 | 29.9 | render/thread/ms 30.46, render/busy/ms 28.37, main/busy/ms 20.75, render/graph/ms 7.00, stream/drive/ms 5.82, stream/spawn/ms 2.76, render/present/ms 1.88, stream/tick/ms 1.43, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.52, bigspace/high_precision/ms 0.32 |
| warm-up | 5.09 | 37.3 | pipelines/wait_ms 986, pipelines/compile_ms 143, pipelines/front_ms 140, render/thread/ms 45.37, render/busy/ms 43.85, main/busy/ms 24.64, stream/drive/ms 5.19, pipelines/driver_ms 3.98, render/graph/ms 3.75, stream/spawn/ms 2.49, stream/tick/ms 1.38, render/present/ms 1.31 |
| warm-up | 5.15 | 47.8 | render/thread/ms 41.40, render/busy/ms 39.86, main/busy/ms 27.12, render/graph/ms 4.99, stream/drive/ms 4.82, stream/spawn/ms 2.03, render/present/ms 1.33, stream/tick/ms 1.09, terrain/update/ms 0.81, bigspace/total/ms 0.69, terrain/select/ms 0.66, bigspace/high_precision/ms 0.44 |
| warm-up | 5.27 | 30.6 | render/thread/ms 48.73, render/busy/ms 46.75, main/busy/ms 23.08, render/graph/ms 5.92, stream/drive/ms 5.36, stream/spawn/ms 3.06, render/present/ms 1.67, stream/tick/ms 0.97, bigspace/total/ms 0.68, terrain/update/ms 0.61, terrain/select/ms 0.52, render/acquire/ms 0.32 |
| warm-up | 5.29 | 28.4 | render/thread/ms 49.26, render/busy/ms 47.61, main/busy/ms 22.84, render/graph/ms 4.68, stream/drive/ms 4.11, stream/spawn/ms 2.04, render/present/ms 1.38, stream/tick/ms 0.99, bigspace/total/ms 0.67, terrain/update/ms 0.56, terrain/select/ms 0.47, bigspace/high_precision/ms 0.30 |
| warm-up | 5.32 | 30.3 | render/thread/ms 44.83, render/busy/ms 43.58, main/busy/ms 21.63, stream/drive/ms 4.42, render/graph/ms 3.72, stream/spawn/ms 2.04, render/present/ms 1.03, stream/tick/ms 0.80, bigspace/total/ms 0.60, terrain/update/ms 0.55, terrain/select/ms 0.46, bigspace/high_precision/ms 0.30 |
| warm-up | 5.41 | 30.0 | pipelines/wait_ms 5091, main/busy/ms 42.08, render/thread/ms 40.96, pipelines/compile_ms 38.61, render/busy/ms 38.41, pipelines/front_ms 20.50, pipelines/driver_ms 18.11, render/graph/ms 7.46, stream/drive/ms 6.46, stream/spawn/ms 2.54, render/present/ms 2.22, terrain/update/ms 0.93 |
| warm-up | 5.44 | 32.1 | render/thread/ms 43.85, render/busy/ms 41.59, main/busy/ms 35.42, render/graph/ms 7.10, stream/drive/ms 4.69, stream/spawn/ms 2.81, render/present/ms 1.94, bigspace/total/ms 0.92, stream/tick/ms 0.89, terrain/update/ms 0.84, terrain/select/ms 0.68, bigspace/high_precision/ms 0.40 |
| warm-up | 5.50 | 35.5 | pipelines/wait_ms 4676, pipelines/compile_ms 121, pipelines/front_ms 116, render/thread/ms 32.67, main/busy/ms 32.62, render/busy/ms 31.14, stream/drive/ms 13.85, pipelines/driver_ms 5.22, render/graph/ms 4.11, stream/spawn/ms 2.19, render/present/ms 1.33, bigspace/total/ms 0.86 |
| warm-up | 5.61 | 30.6 | main/busy/ms 36.37, render/thread/ms 22.13, render/busy/ms 20.53, stream/drive/ms 17.21, render/graph/ms 5.20, stream/spawn/ms 2.24, render/present/ms 1.23, stream/tick/ms 0.65, bigspace/total/ms 0.59, terrain/update/ms 0.53, terrain/select/ms 0.43, render/acquire/ms 0.37 |
| warm-up | 5.92 | 41.9 | pipelines/wait_ms 1620, main/busy/ms 35.94, render/thread/ms 33.43, render/busy/ms 31.94, pipelines/compile_ms 30.70, pipelines/front_ms 28.99, stream/drive/ms 12.29, render/graph/ms 5.26, stream/spawn/ms 2.74, pipelines/driver_ms 1.71, render/present/ms 1.19, stream/tick/ms 1.13 |
| warm-up | 7.42 | 42.8 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 55.04, pipelines/front_ms 52.35, main/busy/ms 50.38, render/thread/ms 39.62, render/busy/ms 37.49, render/graph/ms 7.35, pipelines/driver_ms 2.69, render/present/ms 1.83, stream/drive/ms 1.22, bigspace/total/ms 0.90, far/select/ms 0.70 |
| warm-up | 7.46 | 36.1 | pipelines/wait_ms 9317, main/busy/ms 51.06, render/thread/ms 51.01, render/busy/ms 49.06, pipelines/compile_ms 40.67, pipelines/front_ms 38.44, render/graph/ms 6.74, pipelines/driver_ms 2.23, render/present/ms 1.54, stream/drive/ms 1.35, bigspace/total/ms 1.12, far/select/ms 0.72 |
| warm-up | 7.49 | 31.0 | render/thread/ms 47.99, render/busy/ms 46.34, main/busy/ms 44.03, render/graph/ms 6.43, stream/drive/ms 1.34, render/present/ms 1.31, bigspace/total/ms 1.18, physics/insert/ms 0.67, terrain/update/ms 0.61, far/select/ms 0.52, terrain/select/ms 0.51, stream/tick/ms 0.51 |
| warm-up | 7.64 | 35.6 | main/busy/ms 48.74, render/thread/ms 38.77, render/busy/ms 37.18, render/graph/ms 8.69, bigspace/total/ms 1.71, render/present/ms 1.34, physics/insert/ms 1.14, stream/drive/ms 0.71, anim/evaluate_cpu/ms 0.68, far/select/ms 0.64, stream/tick/ms 0.60, bigspace/high_precision/ms 0.57 |
| warm-up | 7.67 | 26.6 | main/busy/ms 46.69, render/thread/ms 41.04, render/busy/ms 39.45, terrain/update/ms 6.92, render/graph/ms 6.57, bigspace/total/ms 1.53, physics/insert/ms 1.32, render/present/ms 1.30, stream/drive/ms 0.80, stream/tick/ms 0.69, far/select/ms 0.66, bigspace/high_precision/ms 0.56 |
| warm-up | 7.70 | 25.3 | main/busy/ms 57.05, render/thread/ms 36.99, render/busy/ms 35.35, terrain/update/ms 9.14, render/graph/ms 7.34, render/present/ms 1.32, physics/insert/ms 1.32, bigspace/total/ms 0.77, stream/drive/ms 0.73, far/select/ms 0.71, stream/tick/ms 0.64, physics/step/ms 0.57 |
| warm-up | 7.74 | 39.7 | pipelines/wait_ms 2707, pipelines/compile_ms 167, pipelines/front_ms 162, main/busy/ms 50.11, render/thread/ms 35.92, render/busy/ms 34.38, render/graph/ms 6.98, pipelines/driver_ms 4.96, terrain/update/ms 2.79, render/present/ms 1.23, physics/insert/ms 1.11, bigspace/total/ms 0.78 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **90%** (min 39%) · 2580 MHz (min 2580) · memory 8001 MHz · 89.6 W · 76 °C (max 79) · VRAM 5206 MiB · throttled 100% of samples
