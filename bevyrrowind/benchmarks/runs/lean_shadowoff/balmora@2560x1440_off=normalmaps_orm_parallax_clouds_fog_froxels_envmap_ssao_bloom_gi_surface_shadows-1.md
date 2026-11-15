# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1908 frames over 15.0 s · **median 7.66 ms (131 fps)** · mean 7.87 · p95 9.64 · p99 10.96 (1% low 91 fps) · worst 31.50 ms · 13 spikes over 1.5× median · 44.0 stutters/min (work 40.0/min)
- warm-up: 296 frames over 7.5 s, worst 2807.2 ms
- hitches (> 25.0 ms): **1** measured, 31 in warm-up
- timed GPU passes: 5.95 ms of a 7.87 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.66 ms, limit 11 ms
- pass worst frame 31.50 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.927 | 2.081 | 2.335 | 0.93 | 0.188 |
| main_opaque_pass_3d | 1.723 | 1.860 | 2.228 | 0.93 | 0.065 |
| early prepass | 0.511 | 0.552 | 0.793 | 0.93 | 0.106 |
| atmosphere_luts | 0.357 | 0.385 | 0.603 | 0.93 | 0.004 |
| water_velocity | 0.226 | 0.245 | 0.265 | 0.93 | 0.002 |
| main_transmissive_pass_3d | 0.215 | 0.232 | 0.241 | 0.93 | 0.009 |
| water_sspr | 0.205 | 0.221 | 0.222 | 0.93 | 0.003 |
| water_fft | 0.136 | 0.147 | 0.283 | 0.93 | 0.003 |
| upscaling | 0.122 | 0.132 | 0.164 | 0.93 | 0.005 |
| uniform_allocation | 0.114 | 0.123 | 0.119 | 0.93 | 0.026 |
| tonemapping | 0.111 | 0.120 | 0.128 | 0.93 | 0.005 |
| bin_unpacking | 0.084 | 0.091 | 0.098 | 0.93 | 0.017 |
| clustering | 0.060 | 0.065 | 0.070 | 0.93 | 0.023 |
| render_sky | 0.050 | 0.054 | 0.055 | 0.93 | 0.002 |
| water_snapshot | 0.032 | 0.035 | 0.034 | 0.93 | 0.002 |
| early_mesh_preprocessing | 0.025 | 0.027 | 0.030 | 0.93 | 0.005 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.019 | 0.020 | 0.93 | 0.060 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.93 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.93 | 0.002 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.93 | 0.004 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.93 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.93 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.007 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 40.379 | 0.000 | 10652.52 | 77043.9 |
| render/thread/ms | 6.883 | 8.475 | 16.32 | 13132.6 |
| main/busy/ms | 6.443 | 7.899 | 21.63 | 12292.7 |
| render/busy/ms | 6.213 | 7.792 | 15.78 | 11853.7 |
| render/graph/ms | 3.505 | 4.594 | 6.00 | 6686.7 |
| pipelines/compile_ms | 0.817 | 0.000 | 133.31 | 1558.1 |
| pipelines/front_ms | 0.789 | 0.000 | 128.64 | 1505.3 |
| render/present/ms | 0.520 | 1.023 | 2.86 | 992.3 |
| bigspace/total/ms | 0.428 | 1.068 | 2.09 | 816.4 |
| terrain/update/ms | 0.239 | 0.310 | 0.53 | 456.4 |
| stream/drive/ms | 0.225 | 0.326 | 0.54 | 429.9 |
| terrain/select/ms | 0.208 | 0.267 | 0.49 | 396.0 |
| stream/tick/ms | 0.194 | 0.287 | 0.50 | 369.5 |
| far/select/ms | 0.179 | 0.271 | 0.46 | 341.9 |
| bigspace/high_precision/ms | 0.173 | 0.785 | 1.73 | 330.0 |
| controller/feed/ms | 0.165 | 0.234 | 0.52 | 314.0 |
| anim/evaluate/ms | 0.157 | 0.746 | 1.50 | 299.0 |
| render/acquire/ms | 0.150 | 0.577 | 1.68 | 286.5 |
| controller/move/ms | 0.117 | 0.206 | 0.47 | 223.8 |
| bigspace/low_precision/ms | 0.082 | 0.134 | 1.54 | 157.3 |
| anim/evaluate_cpu/ms | 0.060 | 0.089 | 0.34 | 114.5 |
| pipelines/driver_ms | 0.028 | 0.000 | 4.86 | 52.8 |
| physics/step/ms | 0.027 | 0.066 | 1.19 | 51.4 |
| weather/eval/ms | 0.014 | 0.020 | 0.22 | 25.9 |
| script/update/ms | 0.008 | 0.014 | 0.18 | 16.1 |
| physics/insert/ms | 0.007 | 0.000 | 0.97 | 13.3 |
| controller/avoid/ms | 0.005 | 0.007 | 0.03 | 10.2 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 10.1 |
| script/ui/ms | 0.005 | 0.008 | 0.02 | 9.3 |
| anim/apply/ms | 0.005 | 0.007 | 0.04 | 8.8 |
| controller/write/ms | 0.002 | 0.003 | 0.01 | 3.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.8 |
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
| anim/a2/count | count | 0 | 22 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 60 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 28 | 8 |
| crowd/c0/joints/count | count | 272 | 952 | 272 |
| crowd/c0/meshes/count | count | 48 | 168 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1112 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 5281 | 2697 |
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
| frame/work/ms | ms | 6.59 | 21.63 | 7.25 |
| gpu/timed/ms | ms | 6.45 | 7.66 | 6.26 |
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
| physics/pending | count | 0 | 880 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 146 | 146 | 146 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 0 | 130 | 0 |
| preprocess/bind_groups_created | count | 0 | 130 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 28.71 | 41.83 | 30.77 |
| process/mem_usage | GiB | 4.02 | 4.05 | 4.05 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 44.22 | 70.50 | 70.50 |
| system/mem_usage | % | 72.39 | 72.75 | 72.73 |
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
| anim/bones_written/count | count | 0.59 | 495 | 1122 |
| anim/events/count | count | 0.07 | 2 | 137 |
| assets/Image/added | count | 0.06 | 53 | 121 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.18 | 146 | 347 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.07 | 64 | 140 |
| assets/StandardMaterial/modified | count | 2.59 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 6 | 3463 | 11.4 k |
| physics/inserts | count | 0.48 | 131 | 917 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.47 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.81 | 2807.2 | main/busy/ms 2779, surface/advance_cpu_ms 92.54, surface/cell_upload_ms 0.80, stream/drive/ms 0.27, bigspace/total/ms 0.08, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.49 | 678.4 | main/busy/ms 2816, render/thread/ms 638, render/busy/ms 535, render/acquire/ms 101, surface/advance_cpu_ms 92.54, render/graph/ms 5.07, render/present/ms 2.07, stream/drive/ms 0.92, surface/cell_upload_ms 0.80, terrain/update/ms 0.62, stream/tick/ms 0.49, terrain/select/ms 0.43 |
| warm-up | 3.62 | 133.2 | pipelines/wait_ms 1581, render/thread/ms 647, render/busy/ms 543, pipelines/compile_ms 136, pipelines/front_ms 128, render/acquire/ms 101, main/busy/ms 51.50, terrain/update/ms 21.34, pipelines/driver_ms 8.18, render/graph/ms 7.94, render/present/ms 2.97, stream/drive/ms 1.06 |
| warm-up | 3.87 | 89.6 | pipelines/compile_ms 242, pipelines/front_ms 223, main/busy/ms 93.27, render/thread/ms 56.59, render/busy/ms 54.84, pipelines/driver_ms 18.69, render/graph/ms 3.75, render/present/ms 1.49, stream/drive/ms 1.24, stream/tick/ms 0.91, terrain/update/ms 0.71, terrain/select/ms 0.55 |
| warm-up | 4.02 | 58.6 | render/thread/ms 40.73, render/busy/ms 39.44, main/busy/ms 38.62, stream/drive/ms 29.64, render/graph/ms 3.40, render/present/ms 1.11, stream/tick/ms 0.58, terrain/update/ms 0.52, terrain/select/ms 0.41, stream/spawn/ms 0.39, bigspace/total/ms 0.20, render/acquire/ms 0.18 |
| warm-up | 4.08 | 48.5 | pipelines/wait_ms 2323, pipelines/compile_ms 215, pipelines/front_ms 194, render/thread/ms 34.49, render/busy/ms 32.59, main/busy/ms 29.59, pipelines/driver_ms 21.22, stream/drive/ms 15.92, render/graph/ms 3.90, render/present/ms 1.63, stream/tick/ms 1.38, stream/spawn/ms 1.24 |
| warm-up | 4.11 | 28.7 | pipelines/wait_ms 3669, pipelines/compile_ms 235, pipelines/front_ms 213, main/busy/ms 58.04, render/thread/ms 49.10, render/busy/ms 46.98, stream/drive/ms 40.22, pipelines/driver_ms 22.86, render/graph/ms 5.20, stream/spawn/ms 2.35, stream/tick/ms 2.13, render/present/ms 1.78 |
| warm-up | 4.24 | 133.5 | pipelines/wait_ms 5636, pipelines/compile_ms 121, pipelines/front_ms 113, render/thread/ms 92.52, render/busy/ms 64.48, main/busy/ms 59.07, stream/drive/ms 38.60, render/present/ms 27.75, pipelines/driver_ms 7.61, render/graph/ms 3.59, stream/tick/ms 1.54, stream/spawn/ms 1.29 |
| warm-up | 4.28 | 41.6 | pipelines/wait_ms 6372, pipelines/compile_ms 196, pipelines/front_ms 154, render/thread/ms 99.31, render/busy/ms 71.55, main/busy/ms 47.97, pipelines/driver_ms 41.60, render/present/ms 27.46, stream/drive/ms 25.92, render/graph/ms 4.89, stream/tick/ms 0.84, stream/spawn/ms 0.81 |
| warm-up | 4.32 | 39.0 | pipelines/wait_ms 2082, pipelines/compile_ms 95.79, pipelines/front_ms 60.17, main/busy/ms 51.66, render/thread/ms 43.29, render/busy/ms 41.84, pipelines/driver_ms 35.62, stream/drive/ms 32.17, render/graph/ms 5.26, terrain/update/ms 3.02, render/present/ms 1.19, stream/spawn/ms 1.09 |
| warm-up | 4.45 | 76.4 | pipelines/wait_ms 1851, pipelines/compile_ms 135, pipelines/front_ms 130, main/busy/ms 54.72, render/thread/ms 35.15, stream/drive/ms 35.11, render/busy/ms 33.31, pipelines/driver_ms 5.00, render/graph/ms 3.32, stream/tick/ms 1.62, stream/spawn/ms 1.57, render/present/ms 1.54 |
| warm-up | 4.57 | 82.0 | pipelines/wait_ms 6651, pipelines/compile_ms 196, pipelines/front_ms 185, main/busy/ms 89.04, stream/drive/ms 65.07, render/thread/ms 28.67, render/busy/ms 27.25, pipelines/driver_ms 11.44, terrain/update/ms 4.16, render/graph/ms 3.43, render/present/ms 1.13, stream/tick/ms 0.91 |
| warm-up | 4.66 | 73.1 | main/busy/ms 70.70, stream/drive/ms 36.30, render/thread/ms 27.30, render/busy/ms 25.90, terrain/update/ms 3.46, render/graph/ms 2.92, stream/spawn/ms 2.05, render/present/ms 1.21, physics/insert/ms 1.10, stream/tick/ms 0.68, terrain/select/ms 0.47, physics/step/ms 0.41 |
| warm-up | 4.84 | 35.6 | pipelines/wait_ms 358, pipelines/compile_ms 138, pipelines/front_ms 131, render/thread/ms 31.04, main/busy/ms 30.38, render/busy/ms 29.54, stream/drive/ms 11.93, pipelines/driver_ms 6.70, render/graph/ms 4.09, stream/spawn/ms 3.28, stream/activate/ms 1.39, render/present/ms 1.29 |
| warm-up | 4.86 | 25.6 | pipelines/wait_ms 358, pipelines/compile_ms 138, pipelines/front_ms 131, render/thread/ms 34.66, render/busy/ms 33.63, main/busy/ms 24.40, pipelines/driver_ms 6.70, stream/drive/ms 5.97, render/graph/ms 3.29, stream/spawn/ms 2.45, stream/activate/ms 1.58, stream/tick/ms 0.87 |
| warm-up | 5.05 | 37.2 | pipelines/wait_ms 4536, pipelines/compile_ms 159, pipelines/front_ms 144, render/thread/ms 46.44, render/busy/ms 44.75, main/busy/ms 25.26, pipelines/driver_ms 14.85, render/graph/ms 5.25, stream/drive/ms 3.77, stream/spawn/ms 2.10, render/present/ms 1.40, stream/tick/ms 0.88 |
| warm-up | 5.08 | 30.3 | pipelines/wait_ms 11.1 k, pipelines/compile_ms 191, pipelines/front_ms 174, render/thread/ms 55.70, render/busy/ms 54.28, main/busy/ms 31.00, pipelines/driver_ms 17.85, render/graph/ms 5.08, stream/drive/ms 4.28, stream/spawn/ms 2.36, render/present/ms 1.14, stream/tick/ms 0.80 |
| warm-up | 5.13 | 48.4 | pipelines/wait_ms 16.4 k, pipelines/compile_ms 89.02, pipelines/front_ms 76.84, render/thread/ms 60.70, render/busy/ms 59.00, main/busy/ms 44.03, pipelines/driver_ms 12.18, render/graph/ms 6.75, stream/drive/ms 5.17, stream/spawn/ms 2.36, render/present/ms 1.41, physics/insert/ms 1.12 |
| warm-up | 5.15 | 25.4 | pipelines/wait_ms 12.8 k, pipelines/compile_ms 66.33, render/thread/ms 58.82, render/busy/ms 57.11, pipelines/front_ms 56.23, main/busy/ms 42.87, pipelines/driver_ms 10.10, render/graph/ms 6.25, terrain/update/ms 5.12, stream/drive/ms 4.48, stream/spawn/ms 2.04, render/present/ms 1.43 |
| warm-up | 5.19 | 40.6 | pipelines/wait_ms 2982, render/thread/ms 52.09, render/busy/ms 50.15, main/busy/ms 35.73, pipelines/compile_ms 9.81, pipelines/front_ms 8.88, stream/drive/ms 6.64, render/graph/ms 5.55, terrain/update/ms 5.14, stream/spawn/ms 2.13, render/present/ms 1.67, physics/insert/ms 1.10 |
| warm-up | 5.23 | 36.9 | render/thread/ms 61.97, render/busy/ms 59.58, main/busy/ms 46.77, render/graph/ms 7.87, stream/drive/ms 7.27, stream/spawn/ms 2.45, render/present/ms 2.04, stream/tick/ms 0.94, physics/insert/ms 0.88, physics/step/ms 0.83, bigspace/total/ms 0.70, terrain/update/ms 0.66 |
| warm-up | 5.28 | 45.9 | render/thread/ms 63.02, render/busy/ms 60.84, main/busy/ms 41.59, render/graph/ms 7.76, stream/drive/ms 4.57, terrain/update/ms 4.01, stream/spawn/ms 2.88, render/present/ms 1.88, stream/tick/ms 0.94, bigspace/total/ms 0.74, physics/step/ms 0.60, terrain/select/ms 0.60 |
| warm-up | 5.31 | 33.7 | pipelines/wait_ms 3852, pipelines/compile_ms 163, pipelines/front_ms 159, render/thread/ms 51.16, render/busy/ms 49.27, main/busy/ms 34.29, stream/drive/ms 14.63, render/graph/ms 6.76, terrain/update/ms 4.07, pipelines/driver_ms 4.01, stream/spawn/ms 2.71, render/present/ms 1.66 |
| warm-up | 5.35 | 39.8 | pipelines/wait_ms 9017, pipelines/compile_ms 212, pipelines/front_ms 206, render/thread/ms 47.54, render/busy/ms 46.15, main/busy/ms 35.84, stream/drive/ms 14.63, pipelines/driver_ms 6.43, render/graph/ms 5.23, stream/spawn/ms 2.58, render/present/ms 1.19, bigspace/total/ms 1.07 |
| warm-up | 5.47 | 29.0 | pipelines/wait_ms 1128, pipelines/compile_ms 135, pipelines/front_ms 130, main/busy/ms 33.05, render/thread/ms 22.25, render/busy/ms 20.69, stream/drive/ms 17.51, render/graph/ms 5.19, pipelines/driver_ms 5.02, stream/spawn/ms 2.04, render/present/ms 1.28, stream/tick/ms 0.91 |
| warm-up | 5.72 | 31.7 | main/busy/ms 34.53, render/thread/ms 23.71, render/busy/ms 21.86, stream/drive/ms 12.40, render/graph/ms 6.79, stream/spawn/ms 2.18, render/present/ms 1.54, stream/tick/ms 1.14, bigspace/total/ms 1.02, terrain/update/ms 0.74, terrain/select/ms 0.63, bigspace/high_precision/ms 0.45 |
| warm-up | 5.75 | 30.7 | pipelines/wait_ms 3461, pipelines/compile_ms 184, pipelines/front_ms 177, render/thread/ms 34.11, render/busy/ms 32.21, main/busy/ms 30.54, pipelines/driver_ms 7.42, render/graph/ms 6.36, stream/drive/ms 5.68, stream/spawn/ms 3.23, render/present/ms 1.55, stream/tick/ms 1.31 |
| warm-up | 5.78 | 28.8 | pipelines/wait_ms 5563, pipelines/compile_ms 192, pipelines/front_ms 184, render/thread/ms 36.70, render/busy/ms 35.22, main/busy/ms 32.12, pipelines/driver_ms 7.96, stream/drive/ms 4.74, render/graph/ms 4.68, stream/spawn/ms 3.13, render/present/ms 1.25, bigspace/total/ms 0.96 |
| warm-up | 5.97 | 29.8 | pipelines/wait_ms 8574, pipelines/compile_ms 47.53, main/busy/ms 45.30, pipelines/front_ms 44.67, render/thread/ms 36.40, render/busy/ms 34.34, render/graph/ms 7.75, pipelines/driver_ms 2.86, stream/drive/ms 1.81, render/present/ms 1.77, physics/insert/ms 1.19, physics/step/ms 0.91 |
| warm-up | 6.02 | 46.9 | pipelines/wait_ms 6382, main/busy/ms 70.92, render/thread/ms 51.78, render/busy/ms 49.56, pipelines/compile_ms 41.30, pipelines/front_ms 39.54, render/graph/ms 10.07, render/present/ms 1.85, pipelines/driver_ms 1.76, stream/drive/ms 1.50, physics/insert/ms 1.33, physics/step/ms 0.92 |
| warm-up | 6.07 | 50.1 | pipelines/wait_ms 4079, render/thread/ms 56.02, main/busy/ms 54.67, render/busy/ms 54.03, pipelines/compile_ms 25.06, pipelines/front_ms 24.02, render/graph/ms 8.66, render/present/ms 1.67, stream/drive/ms 1.47, pipelines/driver_ms 1.05, physics/insert/ms 0.90, physics/step/ms 0.72 |
| measure | 0.83 | 31.5 | pipelines/wait_ms 6249, main/busy/ms 29.60, render/thread/ms 29.39, render/busy/ms 28.24, pipelines/compile_ms 19.94, pipelines/front_ms 18.73, render/graph/ms 7.40, pipelines/driver_ms 1.22, render/present/ms 0.94, bigspace/total/ms 0.78, terrain/update/ms 0.51, terrain/select/ms 0.44 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **92%** (min 67%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.6 W · 76 °C (max 80) · VRAM 5201 MiB · throttled 100% of samples
