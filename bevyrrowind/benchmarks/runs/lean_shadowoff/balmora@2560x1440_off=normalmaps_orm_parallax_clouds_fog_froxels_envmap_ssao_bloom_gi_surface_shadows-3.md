# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1874 frames over 15.0 s · **median 7.79 ms (128 fps)** · mean 8.01 · p95 10.00 · p99 11.52 (1% low 87 fps) · worst 16.01 ms · 15 spikes over 1.5× median · 36.0 stutters/min (work 16.0/min)
- warm-up: 283 frames over 8.2 s, worst 2856.6 ms
- hitches (> 25.0 ms): **0** measured, 33 in warm-up
- timed GPU passes: 6.01 ms of a 8.01 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.79 ms, limit 11 ms
- pass worst frame 16.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.966 | 2.131 | 2.495 | 0.92 | 0.189 |
| main_opaque_pass_3d | 1.738 | 1.884 | 2.250 | 0.92 | 0.072 |
| early prepass | 0.507 | 0.550 | 0.797 | 0.92 | 0.108 |
| atmosphere_luts | 0.339 | 0.367 | 0.596 | 0.92 | 0.004 |
| water_velocity | 0.235 | 0.254 | 0.388 | 0.92 | 0.002 |
| main_transmissive_pass_3d | 0.220 | 0.239 | 0.246 | 0.92 | 0.009 |
| water_sspr | 0.208 | 0.225 | 0.223 | 0.92 | 0.003 |
| water_fft | 0.130 | 0.141 | 0.142 | 0.92 | 0.003 |
| upscaling | 0.125 | 0.136 | 0.162 | 0.92 | 0.005 |
| uniform_allocation | 0.122 | 0.132 | 0.365 | 0.92 | 0.026 |
| tonemapping | 0.111 | 0.121 | 0.126 | 0.92 | 0.005 |
| bin_unpacking | 0.086 | 0.093 | 0.099 | 0.92 | 0.018 |
| clustering | 0.061 | 0.066 | 0.071 | 0.92 | 0.024 |
| render_sky | 0.051 | 0.055 | 0.055 | 0.92 | 0.002 |
| water_snapshot | 0.035 | 0.038 | 0.037 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.025 | 0.027 | 0.030 | 0.92 | 0.005 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.019 | 0.019 | 0.92 | 0.062 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.92 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.005 | 0.92 | 0.002 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.004 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.008 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 37.466 | 0.000 | 10663.92 | 70211.1 |
| render/thread/ms | 7.046 | 8.763 | 14.23 | 13203.3 |
| main/busy/ms | 6.614 | 8.228 | 15.45 | 12394.7 |
| render/busy/ms | 6.356 | 8.075 | 13.06 | 11910.5 |
| render/graph/ms | 3.596 | 4.788 | 7.49 | 6739.6 |
| pipelines/compile_ms | 0.835 | 0.000 | 188.69 | 1565.5 |
| pipelines/front_ms | 0.809 | 0.000 | 183.39 | 1516.0 |
| render/present/ms | 0.536 | 1.033 | 2.75 | 1005.0 |
| bigspace/total/ms | 0.460 | 1.172 | 2.10 | 861.6 |
| terrain/update/ms | 0.240 | 0.309 | 0.80 | 449.5 |
| stream/drive/ms | 0.230 | 0.333 | 0.56 | 431.9 |
| terrain/select/ms | 0.207 | 0.266 | 0.75 | 388.7 |
| stream/tick/ms | 0.198 | 0.293 | 0.52 | 371.7 |
| bigspace/high_precision/ms | 0.196 | 0.811 | 1.81 | 366.6 |
| far/select/ms | 0.180 | 0.264 | 0.79 | 337.4 |
| controller/feed/ms | 0.173 | 0.242 | 0.53 | 323.6 |
| render/acquire/ms | 0.154 | 0.584 | 1.35 | 287.8 |
| anim/evaluate/ms | 0.150 | 0.646 | 1.79 | 280.4 |
| controller/move/ms | 0.118 | 0.186 | 0.39 | 220.4 |
| bigspace/low_precision/ms | 0.090 | 0.262 | 1.31 | 169.5 |
| anim/evaluate_cpu/ms | 0.063 | 0.091 | 0.29 | 117.4 |
| pipelines/driver_ms | 0.026 | 0.000 | 5.29 | 49.5 |
| physics/step/ms | 0.026 | 0.068 | 0.77 | 49.1 |
| weather/eval/ms | 0.013 | 0.019 | 0.06 | 24.7 |
| script/update/ms | 0.009 | 0.015 | 0.47 | 17.7 |
| physics/insert/ms | 0.007 | 0.042 | 0.76 | 12.4 |
| script/input/ms | 0.006 | 0.008 | 0.22 | 11.1 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 10.6 |
| script/ui/ms | 0.005 | 0.008 | 0.02 | 9.9 |
| anim/apply/ms | 0.005 | 0.007 | 0.05 | 9.0 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.13 | 0.8 |
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
| anim/evaluated/count | count | 7 | 11 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 14 | 8 |
| crowd/c0/joints/count | count | 272 | 476 | 272 |
| crowd/c0/meshes/count | count | 48 | 84 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4856 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1109 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 4125 | 2697 |
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
| frame/work/ms | ms | 6.71 | 15.45 | 6.48 |
| gpu/timed/ms | ms | 6.51 | 7.95 | 6.48 |
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
| physics/pending | count | 0 | 291 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 146 | 146 | 146 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 0 | 100 | 0 |
| preprocess/bind_groups_created | count | 0 | 100 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 25.93 | 42.79 | 26.92 |
| process/mem_usage | GiB | 4.02 | 4.03 | 4.00 |
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
| system/cpu_usage | % | 44.43 | 75.99 | 40.14 |
| system/mem_usage | % | 72.53 | 72.75 | 72.39 |
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
| anim/bones_written/count | count | 0.12 | 165 | 231 |
| anim/events/count | count | 0.06 | 2 | 109 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.65 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.54 | 714 | 1012 |
| physics/inserts | count | 0.17 | 82 | 323 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.48 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.86 | 2856.6 | main/busy/ms 2832, surface/advance_cpu_ms 71.38, surface/cell_upload_ms 0.84, stream/drive/ms 0.34, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, weather/apply/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.78 | 921.3 | main/busy/ms 2868, render/thread/ms 884, render/busy/ms 713, render/acquire/ms 168, surface/advance_cpu_ms 71.38, render/graph/ms 6.19, render/present/ms 2.68, stream/drive/ms 0.94, surface/cell_upload_ms 0.84, terrain/update/ms 0.59, stream/tick/ms 0.47, terrain/select/ms 0.41 |
| warm-up | 4.05 | 111.2 | pipelines/compile_ms 247, pipelines/front_ms 229, render/thread/ms 97.09, render/busy/ms 82.05, main/busy/ms 77.45, pipelines/driver_ms 18.00, render/acquire/ms 13.75, render/graph/ms 5.45, stream/drive/ms 2.43, stream/tick/ms 1.97, render/present/ms 1.29, terrain/update/ms 0.60 |
| warm-up | 4.08 | 27.6 | pipelines/compile_ms 294, pipelines/front_ms 272, pipelines/wait_ms 229, render/thread/ms 103, render/busy/ms 101, main/busy/ms 40.67, pipelines/driver_ms 21.98, render/graph/ms 4.84, stream/drive/ms 3.05, render/present/ms 1.34, stream/tick/ms 1.29, stream/spawn/ms 1.05 |
| warm-up | 4.25 | 80.8 | pipelines/wait_ms 441, render/thread/ms 58.80, render/busy/ms 56.96, main/busy/ms 52.24, stream/drive/ms 37.72, pipelines/compile_ms 6.43, render/graph/ms 6.04, pipelines/front_ms 5.73, stream/tick/ms 1.61, render/present/ms 1.56, pipelines/driver_ms 0.70, terrain/update/ms 0.63 |
| warm-up | 4.33 | 68.2 | pipelines/wait_ms 4415, pipelines/compile_ms 274, pipelines/front_ms 254, render/thread/ms 44.92, render/busy/ms 42.62, main/busy/ms 41.91, stream/drive/ms 21.61, pipelines/driver_ms 20.11, render/graph/ms 6.06, render/present/ms 2.02, stream/tick/ms 1.06, stream/spawn/ms 1.02 |
| warm-up | 4.36 | 29.0 | pipelines/wait_ms 3251, pipelines/compile_ms 98.04, pipelines/front_ms 92.78, main/busy/ms 64.01, render/thread/ms 57.42, render/busy/ms 55.41, stream/drive/ms 41.15, render/graph/ms 5.68, pipelines/driver_ms 5.26, render/present/ms 1.70, stream/spawn/ms 1.36, stream/tick/ms 1.30 |
| warm-up | 4.49 | 129.3 | pipelines/wait_ms 2115, render/thread/ms 80.24, render/busy/ms 78.74, main/busy/ms 67.91, pipelines/compile_ms 58.47, pipelines/front_ms 54.73, stream/drive/ms 41.53, pipelines/driver_ms 3.74, render/graph/ms 2.79, stream/spawn/ms 1.58, render/present/ms 1.22, stream/tick/ms 1.21 |
| warm-up | 4.57 | 88.6 | pipelines/wait_ms 6515, pipelines/compile_ms 234, pipelines/front_ms 221, render/thread/ms 133, render/busy/ms 131, main/busy/ms 64.05, stream/drive/ms 38.19, pipelines/driver_ms 13.17, render/graph/ms 5.29, stream/spawn/ms 2.53, render/present/ms 1.24, stream/tick/ms 0.95 |
| warm-up | 4.60 | 28.9 | pipelines/wait_ms 5616, pipelines/compile_ms 236, pipelines/front_ms 224, render/thread/ms 95.15, render/busy/ms 93.76, main/busy/ms 62.20, stream/drive/ms 37.35, pipelines/driver_ms 12.15, render/graph/ms 6.14, stream/spawn/ms 2.09, render/present/ms 1.19, terrain/update/ms 0.94 |
| warm-up | 4.63 | 28.7 | pipelines/wait_ms 771, render/thread/ms 44.44, render/busy/ms 43.02, pipelines/compile_ms 28.65, main/busy/ms 27.49, pipelines/front_ms 27.08, render/graph/ms 4.64, stream/drive/ms 3.98, pipelines/driver_ms 1.57, render/present/ms 1.18, stream/tick/ms 0.91, terrain/update/ms 0.87 |
| warm-up | 4.67 | 41.5 | render/thread/ms 61.96, render/busy/ms 59.76, main/busy/ms 48.03, render/graph/ms 6.97, stream/drive/ms 6.60, stream/spawn/ms 2.62, render/present/ms 1.86, stream/tick/ms 1.09, stream/activate/ms 1.06, terrain/update/ms 0.96, terrain/select/ms 0.80, bigspace/total/ms 0.55 |
| warm-up | 4.71 | 31.8 | pipelines/wait_ms 3328, pipelines/compile_ms 109, pipelines/front_ms 105, main/busy/ms 72.85, render/thread/ms 59.92, render/busy/ms 57.73, stream/drive/ms 30.12, render/graph/ms 6.05, pipelines/driver_ms 4.97, stream/spawn/ms 3.64, render/present/ms 1.81, stream/tick/ms 1.44 |
| warm-up | 4.77 | 62.2 | pipelines/wait_ms 5795, pipelines/compile_ms 124, pipelines/front_ms 117, main/busy/ms 60.59, render/thread/ms 45.09, render/busy/ms 43.66, stream/drive/ms 27.55, pipelines/driver_ms 6.39, render/graph/ms 3.73, stream/spawn/ms 1.73, stream/tick/ms 1.30, render/present/ms 1.13 |
| warm-up | 4.89 | 69.7 | pipelines/wait_ms 1921, pipelines/compile_ms 111, pipelines/front_ms 104, main/busy/ms 76.87, stream/drive/ms 59.16, render/thread/ms 31.91, render/busy/ms 29.90, pipelines/driver_ms 7.01, render/graph/ms 4.49, render/present/ms 1.35, stream/tick/ms 0.82, terrain/update/ms 0.66 |
| warm-up | 4.97 | 69.2 | main/busy/ms 60.47, stream/drive/ms 42.14, render/thread/ms 29.22, render/busy/ms 27.36, render/graph/ms 5.07, render/present/ms 1.54, stream/tick/ms 0.91, stream/spawn/ms 0.87, terrain/update/ms 0.65, terrain/select/ms 0.52, stream/activate/ms 0.47, bigspace/total/ms 0.37 |
| warm-up | 5.18 | 41.5 | pipelines/wait_ms 934, pipelines/compile_ms 152, pipelines/front_ms 146, main/busy/ms 31.21, render/thread/ms 28.95, render/busy/ms 27.40, stream/drive/ms 12.52, pipelines/driver_ms 5.89, stream/spawn/ms 4.59, render/graph/ms 2.82, render/present/ms 1.29, physics/insert/ms 0.92 |
| warm-up | 5.21 | 28.3 | pipelines/wait_ms 3859, pipelines/compile_ms 87.64, pipelines/front_ms 83.78, render/thread/ms 36.69, render/busy/ms 35.50, main/busy/ms 22.89, stream/drive/ms 5.17, pipelines/driver_ms 3.86, stream/spawn/ms 2.94, render/graph/ms 2.70, physics/insert/ms 1.18, render/present/ms 0.95 |
| warm-up | 5.45 | 31.4 | pipelines/wait_ms 6994, pipelines/compile_ms 58.25, pipelines/front_ms 53.73, render/thread/ms 46.67, render/busy/ms 44.23, main/busy/ms 40.99, render/graph/ms 7.89, stream/drive/ms 6.08, pipelines/driver_ms 4.53, stream/spawn/ms 2.20, render/present/ms 2.11, stream/tick/ms 1.89 |
| warm-up | 5.50 | 46.6 | pipelines/wait_ms 7924, render/thread/ms 59.44, render/busy/ms 57.45, pipelines/compile_ms 44.15, main/busy/ms 40.89, pipelines/front_ms 39.61, render/graph/ms 6.95, stream/drive/ms 5.47, pipelines/driver_ms 4.54, stream/spawn/ms 2.09, render/present/ms 1.63, stream/tick/ms 1.43 |
| warm-up | 5.52 | 26.0 | pipelines/wait_ms 3293, render/thread/ms 50.08, render/busy/ms 48.15, main/busy/ms 46.81, pipelines/compile_ms 15.55, pipelines/front_ms 14.15, render/graph/ms 6.46, stream/drive/ms 4.54, stream/spawn/ms 2.20, render/present/ms 1.60, pipelines/driver_ms 1.40, physics/insert/ms 1.17 |
| warm-up | 5.59 | 64.1 | render/thread/ms 57.72, render/busy/ms 55.26, main/busy/ms 52.32, render/graph/ms 8.69, stream/drive/ms 7.06, stream/spawn/ms 2.23, render/present/ms 2.15, physics/insert/ms 1.15, stream/tick/ms 0.87, terrain/update/ms 0.64, bigspace/total/ms 0.63, physics/step/ms 0.54 |
| warm-up | 5.66 | 60.6 | pipelines/wait_ms 4981, pipelines/compile_ms 157, pipelines/front_ms 144, render/thread/ms 51.72, render/busy/ms 49.77, main/busy/ms 30.37, pipelines/driver_ms 12.58, render/graph/ms 6.68, stream/drive/ms 5.11, stream/spawn/ms 3.13, render/present/ms 1.60, physics/insert/ms 1.24 |
| warm-up | 5.72 | 37.1 | render/thread/ms 45.43, render/busy/ms 43.93, main/busy/ms 35.18, stream/drive/ms 14.73, render/graph/ms 4.83, stream/spawn/ms 2.04, render/present/ms 1.26, physics/insert/ms 1.01, terrain/update/ms 0.91, terrain/select/ms 0.74, bigspace/total/ms 0.65, stream/tick/ms 0.63 |
| warm-up | 5.77 | 29.5 | pipelines/wait_ms 2571, pipelines/compile_ms 160, pipelines/front_ms 153, render/thread/ms 40.61, render/busy/ms 38.76, main/busy/ms 31.23, stream/drive/ms 8.23, pipelines/driver_ms 7.29, render/graph/ms 5.17, stream/spawn/ms 2.12, render/present/ms 1.59, physics/insert/ms 1.14 |
| warm-up | 5.89 | 31.0 | main/busy/ms 33.74, render/thread/ms 22.64, render/busy/ms 21.06, stream/drive/ms 18.63, render/graph/ms 5.58, stream/spawn/ms 1.72, render/present/ms 1.26, stream/tick/ms 0.79, terrain/update/ms 0.50, bigspace/total/ms 0.48, terrain/select/ms 0.42, render/acquire/ms 0.32 |
| warm-up | 6.09 | 34.4 | pipelines/wait_ms 11.9 k, pipelines/compile_ms 70.79, pipelines/front_ms 65.72, render/thread/ms 34.38, render/busy/ms 32.86, main/busy/ms 32.70, pipelines/driver_ms 5.07, render/graph/ms 4.79, stream/drive/ms 4.54, terrain/update/ms 3.18, stream/spawn/ms 2.07, stream/tick/ms 1.29 |
| warm-up | 6.12 | 29.3 | pipelines/wait_ms 19.9 k, pipelines/compile_ms 104, pipelines/front_ms 96.18, render/thread/ms 38.86, render/busy/ms 37.39, main/busy/ms 27.14, pipelines/driver_ms 7.40, render/graph/ms 3.97, stream/drive/ms 3.61, terrain/update/ms 3.12, stream/spawn/ms 2.08, render/present/ms 1.23 |
| warm-up | 6.16 | 27.5 | pipelines/wait_ms 1133, main/busy/ms 30.57, render/thread/ms 30.46, render/busy/ms 28.93, render/graph/ms 6.39, pipelines/compile_ms 4.64, pipelines/driver_ms 4.62, stream/drive/ms 2.03, render/present/ms 1.31, stream/spawn/ms 0.97, terrain/update/ms 0.83, terrain/select/ms 0.72 |
| warm-up | 6.18 | 29.5 | render/thread/ms 41.52, render/busy/ms 40.31, main/busy/ms 27.73, render/graph/ms 4.53, stream/drive/ms 2.25, stream/spawn/ms 1.20, render/present/ms 0.98, bigspace/total/ms 0.62, stream/tick/ms 0.62, terrain/update/ms 0.60, terrain/select/ms 0.50, render/acquire/ms 0.24 |
| warm-up | 6.25 | 44.1 | pipelines/wait_ms 1631, pipelines/compile_ms 133, pipelines/front_ms 128, main/busy/ms 49.04, render/thread/ms 37.03, render/busy/ms 35.25, stream/drive/ms 8.10, render/graph/ms 6.16, pipelines/driver_ms 4.57, render/present/ms 1.48, stream/spawn/ms 1.02, stream/tick/ms 0.66 |
| warm-up | 6.70 | 58.5 | pipelines/wait_ms 5502, main/busy/ms 67.57, render/thread/ms 45.57, render/busy/ms 43.68, pipelines/compile_ms 40.34, pipelines/front_ms 38.36, render/graph/ms 10.32, pipelines/driver_ms 1.98, render/present/ms 1.66, stream/drive/ms 1.43, anim/evaluate_cpu/ms 1.08, bigspace/total/ms 0.95 |
| warm-up | 6.75 | 43.6 | pipelines/wait_ms 527, main/busy/ms 63.90, render/thread/ms 57.95, render/busy/ms 56.53, pipelines/compile_ms 21.61, pipelines/front_ms 20.80, render/graph/ms 9.04, stream/drive/ms 1.60, render/present/ms 1.15, anim/evaluate_cpu/ms 1.08, bigspace/total/ms 1.07, physics/insert/ms 0.96 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 18% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **91%** (min 70%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.0 W · 77 °C (max 80) · VRAM 5440 MiB · throttled 100% of samples
