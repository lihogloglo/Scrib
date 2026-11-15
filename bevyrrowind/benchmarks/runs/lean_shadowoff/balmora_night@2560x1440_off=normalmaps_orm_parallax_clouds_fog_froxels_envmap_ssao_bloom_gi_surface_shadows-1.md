# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1931 frames over 15.0 s · **median 7.58 ms (132 fps)** · mean 7.77 · p95 9.47 · p99 10.72 (1% low 93 fps) · worst 23.59 ms · 12 spikes over 1.5× median · 40.0 stutters/min (work 36.0/min)
- warm-up: 286 frames over 7.4 s, worst 2648.0 ms
- hitches (> 25.0 ms): **0** measured, 27 in warm-up
- timed GPU passes: 5.75 ms of a 7.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.58 ms, limit 11 ms
- pass worst frame 23.59 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.898 | 2.072 | 2.320 | 0.92 | 0.186 |
| main_opaque_pass_3d | 1.533 | 1.673 | 1.928 | 0.92 | 0.057 |
| early prepass | 0.503 | 0.549 | 0.789 | 0.92 | 0.105 |
| atmosphere_luts | 0.378 | 0.412 | 0.625 | 0.92 | 0.004 |
| water_velocity | 0.242 | 0.265 | 0.265 | 0.92 | 0.002 |
| water_sspr | 0.205 | 0.224 | 0.222 | 0.92 | 0.002 |
| main_transmissive_pass_3d | 0.203 | 0.222 | 0.224 | 0.92 | 0.009 |
| water_fft | 0.135 | 0.147 | 0.147 | 0.92 | 0.003 |
| upscaling | 0.125 | 0.136 | 0.161 | 0.92 | 0.005 |
| uniform_allocation | 0.113 | 0.123 | 0.122 | 0.92 | 0.024 |
| tonemapping | 0.113 | 0.123 | 0.133 | 0.92 | 0.005 |
| bin_unpacking | 0.082 | 0.089 | 0.099 | 0.92 | 0.017 |
| clustering | 0.059 | 0.065 | 0.068 | 0.92 | 0.023 |
| render_sky | 0.050 | 0.055 | 0.054 | 0.92 | 0.002 |
| water_snapshot | 0.030 | 0.032 | 0.035 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.024 | 0.027 | 0.029 | 0.92 | 0.004 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.020 | 0.020 | 0.92 | 0.058 |
| night_sky | 0.006 | 0.007 | 0.007 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.92 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.92 | 0.004 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 36.749 | 0.000 | 9471.65 | 70962.1 |
| render/thread/ms | 6.783 | 8.229 | 15.16 | 13098.6 |
| main/busy/ms | 6.444 | 7.933 | 17.85 | 12443.5 |
| render/busy/ms | 6.125 | 7.573 | 14.50 | 11826.7 |
| render/graph/ms | 3.442 | 4.537 | 6.61 | 6646.8 |
| pipelines/compile_ms | 0.870 | 0.000 | 138.64 | 1679.5 |
| pipelines/front_ms | 0.841 | 0.000 | 135.51 | 1623.7 |
| render/present/ms | 0.514 | 0.966 | 2.52 | 993.4 |
| bigspace/total/ms | 0.463 | 1.197 | 2.50 | 894.0 |
| terrain/update/ms | 0.242 | 0.317 | 0.48 | 467.1 |
| stream/drive/ms | 0.232 | 0.332 | 0.65 | 447.1 |
| terrain/select/ms | 0.210 | 0.274 | 0.45 | 405.7 |
| stream/tick/ms | 0.201 | 0.293 | 0.58 | 388.0 |
| bigspace/high_precision/ms | 0.183 | 0.851 | 1.67 | 352.4 |
| far/select/ms | 0.178 | 0.261 | 0.56 | 344.6 |
| controller/feed/ms | 0.158 | 0.223 | 0.58 | 306.0 |
| render/acquire/ms | 0.144 | 0.515 | 1.04 | 278.5 |
| anim/evaluate/ms | 0.140 | 0.783 | 1.62 | 271.2 |
| controller/move/ms | 0.137 | 0.214 | 0.60 | 265.1 |
| bigspace/low_precision/ms | 0.098 | 0.191 | 1.43 | 188.6 |
| anim/evaluate_cpu/ms | 0.062 | 0.086 | 0.29 | 119.0 |
| pipelines/driver_ms | 0.029 | 0.000 | 5.88 | 55.8 |
| physics/step/ms | 0.026 | 0.068 | 0.87 | 50.8 |
| weather/eval/ms | 0.012 | 0.018 | 0.27 | 23.9 |
| script/update/ms | 0.009 | 0.015 | 0.06 | 16.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 11.6 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 9.8 |
| script/ui/ms | 0.005 | 0.008 | 0.02 | 9.6 |
| anim/apply/ms | 0.005 | 0.007 | 0.17 | 8.7 |
| physics/insert/ms | 0.003 | 0.017 | 0.97 | 5.8 |
| controller/write/ms | 0.002 | 0.003 | 0.04 | 3.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 1.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.7 |
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
| anim/a2/count | count | 0 | 17 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 50 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 25 | 8 |
| crowd/c0/joints/count | count | 272 | 850 | 272 |
| crowd/c0/meshes/count | count | 48 | 150 | 48 |
| crowd/c1/count | count | 122 | 129 | 123 |
| crowd/c1/joints/count | count | 4848 | 5219 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1234 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 4669 | 2697 |
| crowd/joints_uploaded/count | count | 2697 | 2697 | 2697 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 6.49 | 17.85 | 6.37 |
| gpu/timed/ms | ms | 6.30 | 7.10 | 6.37 |
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
| phases/opaque3d/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 307 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 145 | 145 | 145 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 0 | 130 | 0 |
| preprocess/bind_groups_created | count | 0 | 130 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 26.85 | 43.75 | 23.19 |
| process/mem_usage | GiB | 4.00 | 4.04 | 4.00 |
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
| started_load_count |  | 3530 | 3532 | 3532 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 43.76 | 70.47 | 32.03 |
| system/mem_usage | % | 71.97 | 72.35 | 72.10 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2461 | 2461 | 2461 |
| textures/images | count | 1908 | 1908 | 1908 |
| views/all/batch_sets | count | 58 | 62 | 58 |
| views/all/bins | count | 3166 | 3168 | 3166 |
| views/all/count | count | 1 | 1 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 9300 | 9300 | 9300 |
| views/all/pipelines | count | 54 | 58 | 54 |
| views/all/pipelines_distinct | count | 54 | 58 | 54 |
| views/all/work_items | count | 9480 | 9480 | 9480 |
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
| views/main/bins | count | 3166 | 3168 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 54 | 58 | 54 |
| views/main/work_items | count | 9480 | 9480 | 9480 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.34 | 132 | 660 |
| anim/events/count | count | 0.08 | 3 | 163 |
| assets/Image/added | count | 0.07 | 39 | 135 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.18 | 123 | 339 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.08 | 43 | 150 |
| assets/StandardMaterial/modified | count | 2.56 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 5.56 | 2421 | 10.7 k |
| physics/inserts | count | 0.17 | 117 | 323 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.47 | 2 | 901 |
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

## Hitches (> 25.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2648.0 | main/busy/ms 2618, surface/advance_cpu_ms 0.87, stream/drive/ms 0.34, surface/cell_upload_ms 0.34, bigspace/total/ms 0.12, stream/tick/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.02, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.61 | 957.1 | main/busy/ms 2655, render/thread/ms 913, render/busy/ms 748, render/acquire/ms 163, render/graph/ms 4.73, render/present/ms 2.42, stream/drive/ms 1.31, surface/advance_cpu_ms 0.87, stream/tick/ms 0.78, terrain/update/ms 0.44, surface/cell_upload_ms 0.34, terrain/select/ms 0.30 |
| warm-up | 3.90 | 99.5 | main/busy/ms 78.53, render/thread/ms 75.12, render/busy/ms 68.60, render/acquire/ms 5.25, render/graph/ms 4.30, stream/drive/ms 3.02, stream/tick/ms 2.31, render/present/ms 1.26, terrain/update/ms 0.68, terrain/select/ms 0.53, bigspace/total/ms 0.39, stream/spawn/ms 0.15 |
| warm-up | 3.94 | 25.0 | pipelines/wait_ms 316, pipelines/compile_ms 256, pipelines/front_ms 247, render/thread/ms 35.69, render/busy/ms 34.57, main/busy/ms 16.38, pipelines/driver_ms 8.85, render/graph/ms 2.08, stream/drive/ms 1.87, render/present/ms 0.94, stream/tick/ms 0.80, terrain/update/ms 0.75 |
| warm-up | 4.05 | 67.0 | main/busy/ms 48.99, render/thread/ms 48.86, render/busy/ms 47.30, stream/drive/ms 36.58, render/graph/ms 3.30, render/present/ms 1.35, stream/tick/ms 1.03, terrain/update/ms 0.58, terrain/select/ms 0.45, render/acquire/ms 0.21, bigspace/total/ms 0.21, stream/spawn/ms 0.19 |
| warm-up | 4.12 | 58.6 | pipelines/wait_ms 2982, pipelines/compile_ms 215, pipelines/front_ms 194, render/thread/ms 41.47, render/busy/ms 39.68, main/busy/ms 34.05, pipelines/driver_ms 21.10, stream/drive/ms 17.76, render/graph/ms 4.87, render/present/ms 1.55, stream/spawn/ms 0.87, terrain/update/ms 0.79 |
| warm-up | 4.24 | 103.5 | render/thread/ms 65.95, render/busy/ms 64.42, main/busy/ms 45.37, stream/drive/ms 20.45, render/graph/ms 2.96, stream/spawn/ms 2.24, stream/tick/ms 1.58, render/present/ms 1.25, terrain/update/ms 0.83, terrain/select/ms 0.64, bigspace/total/ms 0.29, render/acquire/ms 0.28 |
| warm-up | 4.32 | 77.4 | pipelines/wait_ms 2793, pipelines/compile_ms 190, pipelines/front_ms 184, render/thread/ms 117, render/busy/ms 84.35, main/busy/ms 77.28, stream/drive/ms 53.54, render/present/ms 31.95, pipelines/driver_ms 5.90, stream/spawn/ms 3.24, render/graph/ms 1.98, stream/tick/ms 1.05 |
| warm-up | 4.39 | 68.6 | pipelines/wait_ms 6002, pipelines/compile_ms 350, pipelines/front_ms 282, render/thread/ms 101, main/busy/ms 78.40, render/busy/ms 69.39, pipelines/driver_ms 67.98, stream/drive/ms 54.18, render/present/ms 31.83, stream/spawn/ms 2.84, render/graph/ms 2.61, stream/tick/ms 1.72 |
| warm-up | 4.43 | 43.8 | pipelines/wait_ms 5186, pipelines/compile_ms 176, pipelines/front_ms 110, pipelines/driver_ms 66.27, render/thread/ms 66.25, render/busy/ms 65.09, main/busy/ms 33.26, stream/drive/ms 12.33, render/graph/ms 3.06, stream/spawn/ms 2.70, stream/tick/ms 1.78, render/present/ms 0.97 |
| warm-up | 4.49 | 33.3 | pipelines/wait_ms 2749, pipelines/compile_ms 98.26, pipelines/front_ms 92.21, main/busy/ms 37.79, stream/drive/ms 23.58, render/thread/ms 17.14, render/busy/ms 16.12, pipelines/driver_ms 6.05, terrain/update/ms 5.92, render/graph/ms 2.85, render/present/ms 0.83, stream/tick/ms 0.67 |
| warm-up | 4.61 | 63.0 | main/busy/ms 59.64, stream/drive/ms 50.32, render/thread/ms 19.12, render/busy/ms 18.01, render/graph/ms 2.69, render/present/ms 0.91, stream/tick/ms 0.91, terrain/update/ms 0.77, terrain/select/ms 0.62, stream/activate/ms 0.23, render/acquire/ms 0.20, bigspace/total/ms 0.16 |
| warm-up | 4.67 | 55.5 | pipelines/wait_ms 2596, pipelines/compile_ms 142, pipelines/front_ms 138, main/busy/ms 50.66, stream/drive/ms 37.53, render/thread/ms 16.88, render/busy/ms 15.75, pipelines/driver_ms 4.05, render/graph/ms 2.96, stream/spawn/ms 2.06, render/present/ms 0.96, terrain/update/ms 0.77 |
| warm-up | 4.71 | 25.1 | main/busy/ms 25.30, render/thread/ms 18.06, render/busy/ms 17.05, stream/drive/ms 9.02, render/graph/ms 2.89, stream/spawn/ms 2.05, render/present/ms 0.82, terrain/update/ms 0.64, stream/tick/ms 0.62, terrain/select/ms 0.51, bigspace/total/ms 0.37, physics/step/ms 0.21 |
| warm-up | 4.87 | 31.7 | main/busy/ms 26.00, render/thread/ms 23.91, render/busy/ms 22.82, stream/drive/ms 10.97, render/graph/ms 2.61, stream/spawn/ms 2.57, render/present/ms 0.91, terrain/update/ms 0.84, stream/tick/ms 0.76, terrain/select/ms 0.66, bigspace/total/ms 0.50, render/acquire/ms 0.18 |
| warm-up | 5.06 | 44.0 | main/busy/ms 61.33, render/thread/ms 57.09, render/busy/ms 55.25, render/graph/ms 6.84, stream/drive/ms 6.55, stream/spawn/ms 3.37, stream/tick/ms 1.65, render/present/ms 1.41, physics/insert/ms 1.27, terrain/update/ms 0.72, bigspace/total/ms 0.69, terrain/select/ms 0.60 |
| warm-up | 5.12 | 54.2 | render/thread/ms 67.78, render/busy/ms 66.17, main/busy/ms 54.13, render/graph/ms 5.91, stream/drive/ms 5.64, stream/spawn/ms 2.39, physics/insert/ms 1.35, stream/tick/ms 1.31, render/present/ms 1.26, terrain/update/ms 0.67, bigspace/total/ms 0.65, terrain/select/ms 0.51 |
| warm-up | 5.15 | 26.6 | pipelines/wait_ms 2387, pipelines/compile_ms 183, pipelines/front_ms 176, render/thread/ms 54.70, render/busy/ms 53.25, main/busy/ms 28.69, pipelines/driver_ms 7.66, render/graph/ms 5.16, stream/drive/ms 4.48, stream/spawn/ms 2.13, physics/insert/ms 1.25, render/present/ms 1.23 |
| warm-up | 5.17 | 26.1 | pipelines/wait_ms 2387, pipelines/compile_ms 183, pipelines/front_ms 176, render/thread/ms 46.18, render/busy/ms 44.43, main/busy/ms 28.51, pipelines/driver_ms 7.66, render/graph/ms 6.55, stream/drive/ms 4.71, stream/spawn/ms 2.06, render/present/ms 1.43, physics/insert/ms 1.23 |
| warm-up | 5.25 | 29.3 | render/thread/ms 47.40, render/busy/ms 45.76, main/busy/ms 33.80, stream/drive/ms 6.98, render/graph/ms 4.58, stream/spawn/ms 2.59, render/present/ms 1.26, stream/tick/ms 0.75, bigspace/total/ms 0.74, terrain/update/ms 0.72, terrain/select/ms 0.60, physics/step/ms 0.44 |
| warm-up | 5.30 | 29.0 | main/busy/ms 42.82, render/thread/ms 33.29, render/busy/ms 31.10, stream/drive/ms 16.72, render/graph/ms 6.73, stream/spawn/ms 3.18, render/present/ms 1.89, bigspace/total/ms 1.08, terrain/update/ms 0.92, stream/tick/ms 0.86, terrain/select/ms 0.76, bigspace/high_precision/ms 0.58 |
| warm-up | 5.34 | 43.8 | pipelines/wait_ms 5234, pipelines/compile_ms 221, pipelines/front_ms 212, main/busy/ms 37.87, render/thread/ms 32.72, render/busy/ms 30.71, stream/drive/ms 16.05, pipelines/driver_ms 8.68, render/graph/ms 3.56, stream/spawn/ms 2.67, render/present/ms 1.82, terrain/update/ms 0.92 |
| warm-up | 5.44 | 35.0 | main/busy/ms 33.81, render/thread/ms 25.87, render/busy/ms 24.41, render/graph/ms 5.49, stream/drive/ms 3.58, stream/spawn/ms 2.16, render/present/ms 1.28, physics/insert/ms 0.82, stream/tick/ms 0.71, anim/evaluate_cpu/ms 0.71, anim/evaluate/ms 0.60, terrain/update/ms 0.59 |
| warm-up | 5.50 | 28.0 | pipelines/wait_ms 4922, pipelines/compile_ms 145, pipelines/front_ms 138, main/busy/ms 50.48, render/thread/ms 31.01, render/busy/ms 29.41, stream/drive/ms 18.90, pipelines/driver_ms 6.57, render/graph/ms 5.53, stream/spawn/ms 2.60, render/present/ms 1.34, bigspace/total/ms 0.80 |
| warm-up | 5.54 | 41.6 | pipelines/wait_ms 8666, pipelines/compile_ms 176, pipelines/front_ms 167, main/busy/ms 59.23, render/thread/ms 36.66, render/busy/ms 35.13, stream/drive/ms 18.79, pipelines/driver_ms 8.12, render/graph/ms 4.10, stream/spawn/ms 2.50, render/present/ms 1.23, stream/tick/ms 0.82 |
| warm-up | 5.57 | 27.1 | pipelines/wait_ms 3744, main/busy/ms 36.09, render/thread/ms 32.76, render/busy/ms 31.47, pipelines/compile_ms 30.98, pipelines/front_ms 29.43, render/graph/ms 3.71, stream/drive/ms 3.38, stream/spawn/ms 2.07, pipelines/driver_ms 1.55, render/present/ms 1.06, stream/tick/ms 0.79 |
| warm-up | 5.87 | 43.6 | render/thread/ms 39.62, render/busy/ms 37.91, main/busy/ms 37.62, stream/drive/ms 10.85, render/graph/ms 5.41, stream/spawn/ms 1.69, render/present/ms 1.28, bigspace/total/ms 1.21, stream/tick/ms 1.13, terrain/update/ms 0.63, terrain/select/ms 0.52, render/acquire/ms 0.44 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora_night@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **91%** (min 75%) · 2580 MHz (min 2580) · memory 8001 MHz · 89.9 W · 76 °C (max 79) · VRAM 5189 MiB · throttled 100% of samples
