# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1743 frames over 15.0 s · **median 8.53 ms (117 fps)** · mean 8.61 · p95 10.22 · p99 11.24 (1% low 89 fps) · worst 20.12 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 334 frames over 8.9 s, worst 2709.1 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 5.63 ms of a 8.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.53 ms, limit 11 ms
- pass worst frame 20.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.856 | 2.071 | 2.248 | 0.90 | 0.186 |
| main_opaque_pass_3d | 1.460 | 1.629 | 1.817 | 0.90 | 0.060 |
| early prepass | 0.479 | 0.534 | 0.770 | 0.90 | 0.100 |
| atmosphere_luts | 0.315 | 0.351 | 0.479 | 0.90 | 0.003 |
| water_velocity | 0.217 | 0.242 | 0.265 | 0.90 | 0.002 |
| water_sspr | 0.199 | 0.222 | 0.222 | 0.90 | 0.003 |
| main_transmissive_pass_3d | 0.193 | 0.215 | 0.229 | 0.90 | 0.009 |
| uniform_allocation | 0.160 | 0.178 | 0.190 | 0.90 | 0.035 |
| water_fft | 0.124 | 0.139 | 0.138 | 0.90 | 0.003 |
| upscaling | 0.120 | 0.134 | 0.166 | 0.90 | 0.005 |
| bin_unpacking | 0.118 | 0.132 | 0.145 | 0.90 | 0.025 |
| tonemapping | 0.108 | 0.120 | 0.130 | 0.90 | 0.005 |
| clustering | 0.097 | 0.108 | 0.332 | 0.90 | 0.023 |
| render_sky | 0.048 | 0.054 | 0.054 | 0.90 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.040 | 0.042 | 0.90 | 0.006 |
| water_snapshot | 0.029 | 0.032 | 0.034 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.023 | 0.024 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.020 | 0.022 | 0.90 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.018 | 0.90 | 0.057 |
| sparse buffer updates | 0.008 | 0.009 | 0.008 | 0.90 | 0.003 |
| auxiliary_views | 0.007 | 0.008 | 0.004 | 0.90 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.90 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 0.90 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.362 | 0.000 | 6281.84 | 23289.8 |
| render/thread/ms | 7.635 | 9.126 | 13.40 | 13307.3 |
| render/busy/ms | 7.004 | 8.469 | 12.84 | 12208.1 |
| main/busy/ms | 6.883 | 8.302 | 13.87 | 11997.5 |
| render/graph/ms | 3.883 | 4.975 | 6.76 | 6768.6 |
| render/present/ms | 0.538 | 1.076 | 1.77 | 938.3 |
| bigspace/total/ms | 0.490 | 1.385 | 2.37 | 853.9 |
| pipelines/compile_ms | 0.336 | 0.000 | 116.47 | 585.5 |
| pipelines/front_ms | 0.325 | 0.000 | 113.50 | 566.0 |
| terrain/update/ms | 0.249 | 0.322 | 0.54 | 434.5 |
| stream/drive/ms | 0.236 | 0.342 | 0.70 | 411.8 |
| bigspace/high_precision/ms | 0.217 | 0.971 | 2.08 | 377.5 |
| terrain/select/ms | 0.215 | 0.276 | 0.50 | 375.3 |
| stream/tick/ms | 0.205 | 0.302 | 0.67 | 356.5 |
| far/select/ms | 0.177 | 0.260 | 0.46 | 307.8 |
| controller/feed/ms | 0.164 | 0.229 | 0.43 | 286.0 |
| controller/move/ms | 0.135 | 0.211 | 0.35 | 236.0 |
| bigspace/low_precision/ms | 0.131 | 0.473 | 1.70 | 229.0 |
| anim/evaluate/ms | 0.094 | 0.230 | 1.20 | 164.7 |
| render/acquire/ms | 0.092 | 0.131 | 0.47 | 160.8 |
| anim/evaluate_cpu/ms | 0.064 | 0.085 | 0.27 | 111.6 |
| physics/step/ms | 0.027 | 0.064 | 0.40 | 46.5 |
| weather/eval/ms | 0.013 | 0.019 | 0.17 | 23.1 |
| pipelines/driver_ms | 0.011 | 0.000 | 4.49 | 19.5 |
| script/update/ms | 0.009 | 0.014 | 0.21 | 15.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 10.4 |
| script/input/ms | 0.005 | 0.007 | 0.12 | 9.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.01 | 7.9 |
| script/ui/ms | 0.004 | 0.007 | 0.02 | 7.7 |
| physics/insert/ms | 0.003 | 0.000 | 0.55 | 4.9 |
| controller/write/ms | 0.002 | 0.003 | 0.03 | 3.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| anim/a2/count | count | 0 | 6 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 32 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 12 | 8 |
| crowd/c0/joints/count | count | 272 | 408 | 272 |
| crowd/c0/meshes/count | count | 48 | 72 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 8258 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
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
| frame/work/ms | ms | 7.17 | 13.87 | 7.67 |
| gpu/timed/ms | ms | 6.28 | 7.29 | 6.39 |
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
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2039 | 2040 | 2040 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5403 | 5404 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 57 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 10 | 0 |
| preprocess/bind_groups | count | 0 | 106 | 0 |
| preprocess/bind_groups_created | count | 0 | 106 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 25.48 | 37.98 | 23.08 |
| process/mem_usage | GiB | 3.94 | 3.95 | 3.95 |
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
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 44.58 | 65.15 | 51.22 |
| system/mem_usage | % | 72.77 | 73.01 | 72.78 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 91 | 91 |
| views/all/bins | count | 5209 | 5210 | 5210 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.7 k | 14.7 k |
| views/all/pipelines | count | 87 | 87 | 87 |
| views/all/pipelines_distinct | count | 77 | 77 | 77 |
| views/all/work_items | count | 14.9 k | 14.9 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2039 | 2040 | 2040 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5403 | 5404 | 5404 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5403 | 5404 | 5404 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 54 | 54 |
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
| anim/bones_written/count | count | 0.09 | 165 | 165 |
| anim/events/count | count | 0.08 | 2 | 145 |
| assets/Image/added | count | 0.01 | 22 | 22 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.05 | 79 | 79 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 26 | 26 |
| assets/StandardMaterial/modified | count | 2.84 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.70 | 1076 | 1216 |
| physics/inserts | count | 0.03 | 12 | 58 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.52 | 1 | 900 |
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
| warm-up | 2.71 | 2709.1 | main/busy/ms 2682, surface/advance_cpu_ms 74.54, surface/cell_upload_ms 0.74, stream/drive/ms 0.35, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.79 | 1076.7 | main/busy/ms 2731, render/thread/ms 1045, render/busy/ms 889, render/acquire/ms 154, surface/advance_cpu_ms 74.54, render/graph/ms 5.94, render/present/ms 2.00, stream/drive/ms 1.06, surface/cell_upload_ms 0.74, terrain/update/ms 0.58, stream/tick/ms 0.53, terrain/select/ms 0.29 |
| warm-up | 4.06 | 95.5 | pipelines/compile_ms 195, pipelines/front_ms 176, main/busy/ms 60.09, render/thread/ms 56.92, render/busy/ms 54.60, pipelines/driver_ms 18.43, render/graph/ms 6.71, render/present/ms 1.99, pipelines/wait_ms 1.44, stream/drive/ms 1.11, stream/tick/ms 0.87, terrain/update/ms 0.67 |
| warm-up | 4.09 | 25.7 | render/thread/ms 32.92, render/busy/ms 31.16, main/busy/ms 18.39, render/graph/ms 3.97, stream/drive/ms 2.42, render/present/ms 1.51, stream/tick/ms 1.19, terrain/update/ms 0.83, terrain/select/ms 0.65, stream/spawn/ms 0.43, bigspace/total/ms 0.35, render/acquire/ms 0.25 |
| warm-up | 4.24 | 79.5 | pipelines/wait_ms 2871, pipelines/compile_ms 273, pipelines/front_ms 257, main/busy/ms 47.87, render/thread/ms 46.56, render/busy/ms 45.30, stream/drive/ms 35.92, pipelines/driver_ms 16.29, render/graph/ms 2.55, render/present/ms 1.08, stream/tick/ms 0.80, terrain/update/ms 0.69 |
| warm-up | 4.30 | 51.5 | pipelines/wait_ms 1461, pipelines/compile_ms 45.93, pipelines/front_ms 42.50, main/busy/ms 37.82, render/thread/ms 32.50, render/busy/ms 31.09, stream/drive/ms 26.89, render/graph/ms 3.72, pipelines/driver_ms 3.43, render/present/ms 1.17, stream/tick/ms 0.70, terrain/update/ms 0.55 |
| warm-up | 4.39 | 84.3 | pipelines/wait_ms 1653, main/busy/ms 51.24, pipelines/compile_ms 49.63, pipelines/front_ms 46.14, render/thread/ms 46.07, render/busy/ms 44.49, stream/drive/ms 35.27, pipelines/driver_ms 3.49, render/graph/ms 2.92, render/present/ms 1.39, stream/spawn/ms 0.96, terrain/update/ms 0.87 |
| warm-up | 4.45 | 43.9 | render/thread/ms 36.47, main/busy/ms 35.86, render/busy/ms 35.12, stream/drive/ms 19.10, stream/spawn/ms 3.66, render/graph/ms 3.06, render/present/ms 1.00, terrain/update/ms 0.91, terrain/select/ms 0.70, stream/tick/ms 0.68, stream/activate/ms 0.52, render/acquire/ms 0.36 |
| warm-up | 4.51 | 39.3 | pipelines/wait_ms 272, pipelines/compile_ms 130, pipelines/front_ms 121, render/thread/ms 41.44, render/busy/ms 39.23, main/busy/ms 21.94, pipelines/driver_ms 9.74, render/graph/ms 6.78, stream/drive/ms 5.05, stream/spawn/ms 2.56, render/present/ms 1.94, stream/tick/ms 1.28 |
| warm-up | 4.55 | 46.2 | pipelines/wait_ms 2042, pipelines/compile_ms 106, pipelines/front_ms 96.97, render/thread/ms 59.12, render/busy/ms 56.10, main/busy/ms 54.81, stream/drive/ms 35.46, pipelines/driver_ms 9.29, render/graph/ms 7.19, render/present/ms 2.53, stream/tick/ms 1.67, terrain/update/ms 0.79 |
| warm-up | 4.66 | 111.7 | pipelines/wait_ms 4965, pipelines/compile_ms 216, pipelines/front_ms 201, render/thread/ms 96.78, render/busy/ms 94.43, main/busy/ms 50.50, stream/drive/ms 34.93, pipelines/driver_ms 15.36, render/graph/ms 5.16, render/present/ms 1.91, stream/tick/ms 1.21, stream/activate/ms 0.79 |
| warm-up | 4.74 | 31.1 | pipelines/wait_ms 1451, pipelines/compile_ms 67.46, pipelines/front_ms 63.84, render/thread/ms 37.97, render/busy/ms 36.07, main/busy/ms 25.92, render/graph/ms 8.67, pipelines/driver_ms 3.63, stream/drive/ms 2.88, render/present/ms 1.62, stream/tick/ms 1.17, stream/spawn/ms 1.02 |
| warm-up | 4.89 | 112.1 | pipelines/wait_ms 2857, pipelines/compile_ms 179, pipelines/front_ms 173, main/busy/ms 105, stream/drive/ms 84.16, render/thread/ms 29.06, render/busy/ms 27.75, pipelines/driver_ms 5.85, render/graph/ms 3.44, stream/spawn/ms 2.64, render/present/ms 1.09, terrain/update/ms 0.85 |
| warm-up | 5.04 | 27.0 | main/busy/ms 26.75, render/thread/ms 26.70, render/busy/ms 24.60, stream/drive/ms 11.68, render/graph/ms 4.22, render/present/ms 1.73, stream/spawn/ms 1.12, stream/tick/ms 0.74, stream/activate/ms 0.59, terrain/update/ms 0.59, terrain/select/ms 0.48, physics/insert/ms 0.44 |
| warm-up | 5.08 | 27.4 | pipelines/wait_ms 3334, pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 31.51, render/busy/ms 30.27, main/busy/ms 23.16, stream/drive/ms 4.29, pipelines/driver_ms 3.81, render/graph/ms 2.73, stream/spawn/ms 2.19, render/present/ms 0.96, stream/tick/ms 0.88 |
| warm-up | 5.13 | 33.2 | main/busy/ms 35.17, render/thread/ms 32.69, render/busy/ms 30.51, stream/drive/ms 13.71, render/graph/ms 6.59, stream/spawn/ms 4.23, render/present/ms 1.90, stream/tick/ms 1.60, terrain/update/ms 0.91, terrain/select/ms 0.69, stream/activate/ms 0.52, bigspace/total/ms 0.39 |
| warm-up | 5.32 | 28.7 | pipelines/wait_ms 4102, render/thread/ms 40.79, render/busy/ms 39.56, pipelines/compile_ms 35.20, pipelines/front_ms 32.80, main/busy/ms 24.89, stream/drive/ms 3.81, render/graph/ms 3.58, pipelines/driver_ms 2.40, stream/spawn/ms 0.97, render/present/ms 0.95, stream/tick/ms 0.90 |
| warm-up | 5.38 | 26.8 | render/thread/ms 36.95, render/busy/ms 35.27, main/busy/ms 27.12, stream/drive/ms 5.26, render/graph/ms 4.35, stream/spawn/ms 2.99, render/present/ms 1.38, stream/tick/ms 1.04, terrain/update/ms 0.69, bigspace/total/ms 0.65, terrain/select/ms 0.57, bigspace/high_precision/ms 0.35 |
| warm-up | 5.41 | 30.7 | render/thread/ms 45.98, render/busy/ms 44.33, main/busy/ms 28.25, stream/drive/ms 4.68, render/graph/ms 4.06, stream/spawn/ms 2.14, render/present/ms 1.37, stream/tick/ms 1.23, bigspace/total/ms 0.77, terrain/update/ms 0.72, terrain/select/ms 0.59, bigspace/high_precision/ms 0.39 |
| warm-up | 5.44 | 33.1 | pipelines/wait_ms 3236, render/thread/ms 47.21, render/busy/ms 45.67, main/busy/ms 26.20, stream/drive/ms 4.80, render/graph/ms 3.72, pipelines/compile_ms 2.64, pipelines/front_ms 2.31, stream/spawn/ms 2.19, render/present/ms 1.33, stream/tick/ms 0.88, bigspace/total/ms 0.83 |
| warm-up | 5.53 | 31.0 | render/thread/ms 43.68, render/busy/ms 42.13, main/busy/ms 26.81, stream/drive/ms 5.61, render/graph/ms 5.09, stream/spawn/ms 2.06, render/present/ms 1.16, stream/tick/ms 1.01, bigspace/total/ms 0.85, terrain/update/ms 0.70, terrain/select/ms 0.53, render/acquire/ms 0.39 |
| warm-up | 5.56 | 27.9 | render/thread/ms 46.40, render/busy/ms 44.70, main/busy/ms 25.55, render/graph/ms 4.41, stream/drive/ms 3.25, stream/spawn/ms 2.05, render/present/ms 1.29, bigspace/total/ms 0.89, terrain/update/ms 0.84, stream/tick/ms 0.69, terrain/select/ms 0.63, bigspace/high_precision/ms 0.43 |
| warm-up | 5.59 | 28.3 | pipelines/wait_ms 940, pipelines/compile_ms 142, pipelines/front_ms 139, render/thread/ms 38.93, render/busy/ms 36.94, main/busy/ms 34.46, stream/drive/ms 14.21, render/graph/ms 5.62, pipelines/driver_ms 3.61, stream/spawn/ms 2.04, render/present/ms 1.73, bigspace/total/ms 0.97 |
| warm-up | 5.61 | 26.6 | pipelines/wait_ms 7056, pipelines/compile_ms 176, pipelines/front_ms 170, render/thread/ms 34.43, main/busy/ms 33.89, render/busy/ms 32.77, stream/drive/ms 14.45, pipelines/driver_ms 5.85, render/graph/ms 4.51, stream/spawn/ms 2.10, render/present/ms 1.45, bigspace/total/ms 0.99 |
| warm-up | 5.74 | 32.5 | main/busy/ms 40.69, render/thread/ms 26.52, render/busy/ms 24.98, stream/drive/ms 18.04, render/graph/ms 4.41, terrain/update/ms 3.20, stream/spawn/ms 2.12, render/present/ms 1.13, bigspace/total/ms 0.64, stream/tick/ms 0.61, terrain/select/ms 0.52, render/acquire/ms 0.42 |
| warm-up | 6.04 | 31.2 | main/busy/ms 45.68, render/thread/ms 35.78, render/busy/ms 33.39, stream/drive/ms 13.78, render/graph/ms 10.53, stream/spawn/ms 2.23, render/present/ms 2.15, stream/tick/ms 1.16, bigspace/total/ms 1.07, physics/insert/ms 0.96, terrain/update/ms 0.94, terrain/select/ms 0.77 |
| warm-up | 6.10 | 58.6 | main/busy/ms 67.79, render/thread/ms 55.52, render/busy/ms 53.39, stream/drive/ms 14.75, render/graph/ms 7.36, stream/spawn/ms 3.32, render/present/ms 1.81, physics/insert/ms 1.16, stream/tick/ms 1.05, terrain/update/ms 0.96, terrain/select/ms 0.80, bigspace/total/ms 0.78 |
| warm-up | 6.14 | 40.1 | main/busy/ms 64.92, render/thread/ms 59.11, render/busy/ms 57.43, render/graph/ms 5.72, stream/drive/ms 4.74, stream/spawn/ms 3.38, render/present/ms 1.35, physics/insert/ms 1.09, bigspace/total/ms 0.76, stream/tick/ms 0.72, terrain/update/ms 0.69, terrain/select/ms 0.58 |
| warm-up | 6.22 | 57.3 | pipelines/wait_ms 9073, pipelines/compile_ms 196, pipelines/front_ms 188, main/busy/ms 53.91, render/thread/ms 48.43, render/busy/ms 46.48, pipelines/driver_ms 8.32, render/graph/ms 8.13, stream/drive/ms 2.61, render/present/ms 1.68, stream/spawn/ms 1.21, bigspace/total/ms 1.19 |
| warm-up | 6.38 | 32.7 | pipelines/wait_ms 7062, main/busy/ms 49.84, render/thread/ms 36.48, render/busy/ms 34.65, pipelines/compile_ms 15.60, pipelines/front_ms 12.83, render/graph/ms 9.31, pipelines/driver_ms 2.77, stream/drive/ms 1.75, render/present/ms 1.56, bigspace/total/ms 1.02, anim/evaluate_cpu/ms 0.81 |
| warm-up | 6.43 | 51.3 | render/thread/ms 52.44, main/busy/ms 51.12, render/busy/ms 50.80, render/graph/ms 8.75, stream/drive/ms 1.88, render/present/ms 1.36, bigspace/total/ms 1.10, stream/tick/ms 0.91, anim/evaluate_cpu/ms 0.79, terrain/update/ms 0.67, terrain/select/ms 0.55, physics/step/ms 0.55 |
| warm-up | 6.50 | 33.2 | pipelines/wait_ms 4457, pipelines/compile_ms 155, pipelines/front_ms 150, main/busy/ms 39.97, render/thread/ms 21.93, render/busy/ms 20.45, render/graph/ms 6.94, pipelines/driver_ms 5.14, stream/drive/ms 1.65, render/present/ms 1.22, bigspace/total/ms 0.82, stream/tick/ms 0.72 |
| warm-up | 6.72 | 27.1 | main/busy/ms 40.77, render/thread/ms 35.66, render/busy/ms 33.96, render/graph/ms 11.69, stream/drive/ms 1.65, render/present/ms 1.37, bigspace/total/ms 0.82, stream/tick/ms 0.78, anim/evaluate_cpu/ms 0.77, terrain/update/ms 0.58, terrain/select/ms 0.49, anim/evaluate/ms 0.45 |
| warm-up | 6.75 | 31.9 | main/busy/ms 43.46, render/thread/ms 41.89, render/busy/ms 40.27, render/graph/ms 10.58, stream/drive/ms 1.33, render/present/ms 1.23, bigspace/total/ms 0.89, anim/evaluate_cpu/ms 0.83, physics/insert/ms 0.59, stream/tick/ms 0.58, terrain/update/ms 0.54, anim/evaluate/ms 0.49 |
| warm-up | 8.27 | 29.2 | main/busy/ms 38.37, render/thread/ms 33.18, render/busy/ms 29.38, render/graph/ms 12.13, render/present/ms 3.51, stream/drive/ms 1.40, bigspace/total/ms 1.38, anim/evaluate/ms 1.24, bigspace/high_precision/ms 0.76, terrain/update/ms 0.69, stream/tick/ms 0.66, terrain/select/ms 0.57 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 19% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **90%** (min 77%) · 2580 MHz (min 2580) · memory 8001 MHz · 88.5 W · 75 °C (max 79) · VRAM 5413 MiB · throttled 100% of samples
