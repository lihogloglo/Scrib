# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1736 frames over 15.0 s · **median 8.54 ms (117 fps)** · mean 8.64 · p95 10.29 · p99 11.40 (1% low 88 fps) · worst 14.62 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 295 frames over 7.8 s, worst 2812.1 ms
- hitches (> 25.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 5.77 ms of a 8.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.54 ms, limit 11 ms
- pass worst frame 14.62 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.898 | 2.067 | 2.226 | 0.92 | 0.184 |
| main_opaque_pass_3d | 1.499 | 1.632 | 1.806 | 0.92 | 0.067 |
| early prepass | 0.494 | 0.538 | 0.779 | 0.92 | 0.098 |
| atmosphere_luts | 0.319 | 0.347 | 0.467 | 0.92 | 0.004 |
| water_velocity | 0.223 | 0.242 | 0.265 | 0.92 | 0.002 |
| water_sspr | 0.203 | 0.221 | 0.220 | 0.92 | 0.003 |
| main_transmissive_pass_3d | 0.198 | 0.215 | 0.228 | 0.92 | 0.009 |
| uniform_allocation | 0.165 | 0.180 | 0.191 | 0.92 | 0.037 |
| water_fft | 0.126 | 0.137 | 0.137 | 0.92 | 0.003 |
| bin_unpacking | 0.125 | 0.136 | 0.146 | 0.92 | 0.025 |
| upscaling | 0.120 | 0.131 | 0.134 | 0.92 | 0.005 |
| tonemapping | 0.109 | 0.118 | 0.121 | 0.92 | 0.005 |
| clustering | 0.104 | 0.113 | 0.335 | 0.92 | 0.023 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.039 | 0.041 | 0.92 | 0.007 |
| water_snapshot | 0.030 | 0.033 | 0.035 | 0.92 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.022 | 0.92 | 0.008 |
| main_transparent_pass_3d | 0.017 | 0.018 | 0.019 | 0.92 | 0.060 |
| sparse buffer updates | 0.009 | 0.010 | 0.008 | 0.92 | 0.003 |
| auxiliary_views | 0.008 | 0.009 | 0.004 | 0.92 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.92 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 37.665 | 0.000 | 13668.90 | 65387.2 |
| render/thread/ms | 7.674 | 9.097 | 11.54 | 13322.6 |
| render/busy/ms | 7.050 | 8.417 | 10.67 | 12239.1 |
| main/busy/ms | 6.943 | 8.401 | 11.61 | 12053.6 |
| render/graph/ms | 3.874 | 4.908 | 6.73 | 6725.1 |
| pipelines/compile_ms | 0.551 | 0.000 | 131.88 | 956.0 |
| pipelines/front_ms | 0.530 | 0.000 | 126.90 | 920.2 |
| render/present/ms | 0.528 | 1.044 | 2.96 | 917.3 |
| bigspace/total/ms | 0.486 | 1.397 | 2.14 | 843.7 |
| terrain/update/ms | 0.252 | 0.325 | 0.63 | 436.9 |
| stream/drive/ms | 0.235 | 0.333 | 0.58 | 408.0 |
| terrain/select/ms | 0.218 | 0.279 | 0.58 | 378.8 |
| bigspace/high_precision/ms | 0.211 | 1.098 | 1.81 | 366.4 |
| stream/tick/ms | 0.202 | 0.295 | 0.54 | 351.5 |
| far/select/ms | 0.182 | 0.273 | 0.50 | 315.3 |
| controller/feed/ms | 0.167 | 0.226 | 0.48 | 289.9 |
| controller/move/ms | 0.143 | 0.275 | 0.42 | 248.9 |
| bigspace/low_precision/ms | 0.132 | 0.498 | 1.67 | 229.3 |
| render/acquire/ms | 0.096 | 0.136 | 0.93 | 166.2 |
| anim/evaluate/ms | 0.084 | 0.194 | 1.66 | 146.4 |
| anim/evaluate_cpu/ms | 0.061 | 0.080 | 0.17 | 106.1 |
| physics/step/ms | 0.029 | 0.067 | 0.64 | 50.9 |
| pipelines/driver_ms | 0.021 | 0.000 | 5.24 | 35.7 |
| weather/eval/ms | 0.013 | 0.018 | 0.04 | 21.8 |
| script/update/ms | 0.009 | 0.014 | 0.15 | 15.2 |
| physics/insert/ms | 0.006 | 0.034 | 0.77 | 10.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 10.5 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 9.5 |
| anim/apply/ms | 0.005 | 0.006 | 0.06 | 8.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 7.1 |
| controller/write/ms | 0.002 | 0.003 | 0.04 | 3.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.4 |
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
| anim/a2/count | count | 0 | 2 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 12 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 9 | 8 |
| crowd/c0/joints/count | count | 272 | 306 | 272 |
| crowd/c0/meshes/count | count | 48 | 54 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 5668 | 4212 |
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
| frame/work/ms | ms | 7.18 | 11.61 | 6.83 |
| gpu/timed/ms | ms | 6.28 | 7.13 | 6.16 |
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
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2040 | 2040 | 2040 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5404 | 5404 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 558 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 0 | 158 | 0 |
| preprocess/bind_groups_created | count | 0 | 158 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 25.84 | 41.83 | 27.40 |
| process/mem_usage | GiB | 4.01 | 4.03 | 4.00 |
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
| system/cpu_usage | % | 42.77 | 61.44 | 41.78 |
| system/mem_usage | % | 71.42 | 71.67 | 71.31 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 93 | 91 |
| views/all/bins | count | 5210 | 5210 | 5210 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.7 k | 14.7 k |
| views/all/pipelines | count | 87 | 89 | 87 |
| views/all/pipelines_distinct | count | 77 | 79 | 77 |
| views/all/work_items | count | 14.9 k | 14.9 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2040 | 2040 | 2040 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5404 | 5404 | 5404 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5404 | 5404 | 5404 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 60 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 56 | 54 |
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
| anim/bones_written/count | count | 0.06 | 66 | 99 |
| anim/events/count | count | 0.08 | 3 | 146 |
| assets/Image/added | count | 0.01 | 16 | 25 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.05 | 58 | 86 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 18 | 27 |
| assets/StandardMaterial/modified | count | 2.85 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.71 | 458 | 1237 |
| physics/inserts | count | 0.22 | 139 | 385 |
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

## Hitches (> 25.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.81 | 2812.1 | main/busy/ms 2788, surface/advance_cpu_ms 67.80, surface/cell_upload_ms 0.99, stream/drive/ms 0.39, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01, weather/apply/ms 0.01 |
| warm-up | 3.61 | 793.4 | main/busy/ms 2821, render/thread/ms 758, render/busy/ms 659, render/acquire/ms 96.55, surface/advance_cpu_ms 67.80, render/graph/ms 4.62, render/present/ms 1.98, stream/drive/ms 1.04, surface/cell_upload_ms 0.99, stream/tick/ms 0.49, terrain/update/ms 0.36, bigspace/total/ms 0.23 |
| warm-up | 3.86 | 82.9 | pipelines/compile_ms 222, pipelines/front_ms 202, pipelines/wait_ms 160, main/busy/ms 59.74, render/thread/ms 52.65, render/busy/ms 51.25, pipelines/driver_ms 20.84, render/graph/ms 3.30, stream/drive/ms 2.17, stream/tick/ms 1.63, render/present/ms 1.20, terrain/update/ms 0.63 |
| warm-up | 4.03 | 64.0 | pipelines/wait_ms 951, pipelines/compile_ms 202, pipelines/front_ms 194, render/thread/ms 49.08, render/busy/ms 47.80, main/busy/ms 43.87, stream/drive/ms 32.04, pipelines/driver_ms 7.56, render/graph/ms 3.45, render/present/ms 1.01, stream/tick/ms 0.87, terrain/update/ms 0.68 |
| warm-up | 4.13 | 93.8 | pipelines/wait_ms 4238, pipelines/compile_ms 113, pipelines/front_ms 105, render/thread/ms 54.15, render/busy/ms 52.56, main/busy/ms 42.09, stream/drive/ms 21.46, pipelines/driver_ms 7.61, render/graph/ms 5.22, terrain/update/ms 4.99, stream/spawn/ms 1.52, render/present/ms 1.24 |
| warm-up | 4.16 | 29.9 | pipelines/wait_ms 4497, pipelines/compile_ms 126, pipelines/front_ms 118, render/thread/ms 68.10, render/busy/ms 66.24, main/busy/ms 47.49, stream/drive/ms 31.03, pipelines/driver_ms 8.37, render/graph/ms 6.29, terrain/update/ms 5.31, stream/spawn/ms 1.73, render/present/ms 1.45 |
| warm-up | 4.27 | 112.2 | pipelines/wait_ms 3922, pipelines/compile_ms 110, pipelines/front_ms 104, render/thread/ms 89.16, render/busy/ms 87.53, main/busy/ms 47.86, stream/drive/ms 29.11, pipelines/driver_ms 6.16, render/graph/ms 3.51, render/present/ms 1.39, terrain/update/ms 0.99, stream/spawn/ms 0.82 |
| warm-up | 4.32 | 40.9 | pipelines/wait_ms 1035, pipelines/compile_ms 91.79, pipelines/front_ms 82.84, main/busy/ms 44.58, stream/drive/ms 31.07, render/thread/ms 24.71, render/busy/ms 23.39, pipelines/driver_ms 8.95, render/graph/ms 3.44, stream/spawn/ms 1.31, render/present/ms 1.15, terrain/update/ms 0.70 |
| warm-up | 4.42 | 52.4 | pipelines/wait_ms 673, pipelines/compile_ms 73.16, pipelines/front_ms 70.19, main/busy/ms 47.08, stream/drive/ms 29.59, render/thread/ms 27.54, render/busy/ms 26.23, render/graph/ms 3.81, pipelines/driver_ms 2.97, stream/spawn/ms 2.49, stream/tick/ms 1.24, render/present/ms 1.12 |
| warm-up | 4.46 | 35.1 | pipelines/wait_ms 673, pipelines/compile_ms 73.16, pipelines/front_ms 70.19, render/thread/ms 42.73, render/busy/ms 40.90, main/busy/ms 31.77, stream/drive/ms 5.78, render/graph/ms 5.01, pipelines/driver_ms 2.97, stream/spawn/ms 2.19, render/present/ms 1.57, stream/tick/ms 1.25 |
| warm-up | 4.49 | 30.9 | render/thread/ms 43.95, render/busy/ms 42.16, main/busy/ms 28.07, stream/drive/ms 5.22, render/graph/ms 4.57, stream/spawn/ms 2.10, render/present/ms 1.52, stream/tick/ms 1.01, terrain/update/ms 0.75, terrain/select/ms 0.55, bigspace/total/ms 0.46, stream/activate/ms 0.29 |
| warm-up | 4.52 | 31.8 | pipelines/wait_ms 1589, pipelines/compile_ms 131, pipelines/front_ms 127, main/busy/ms 95.86, stream/drive/ms 82.75, render/thread/ms 39.33, render/busy/ms 37.89, pipelines/driver_ms 3.73, render/graph/ms 3.61, stream/spawn/ms 1.19, render/present/ms 1.17, stream/tick/ms 1.17 |
| warm-up | 4.61 | 92.8 | pipelines/wait_ms 5081, pipelines/compile_ms 240, pipelines/front_ms 229, main/busy/ms 92.45, stream/drive/ms 81.65, render/thread/ms 38.51, render/busy/ms 37.32, pipelines/driver_ms 11.46, render/graph/ms 2.96, stream/tick/ms 0.97, render/present/ms 0.94, stream/activate/ms 0.94 |
| warm-up | 4.65 | 30.6 | main/busy/ms 34.61, render/thread/ms 22.46, render/busy/ms 20.52, stream/drive/ms 11.14, render/graph/ms 5.97, terrain/update/ms 3.21, stream/spawn/ms 2.38, render/present/ms 1.67, stream/tick/ms 0.80, bigspace/total/ms 0.53, stream/activate/ms 0.48, terrain/select/ms 0.47 |
| warm-up | 4.76 | 34.1 | pipelines/wait_ms 4963, pipelines/compile_ms 52.49, pipelines/front_ms 47.68, main/busy/ms 45.27, render/thread/ms 28.25, render/busy/ms 26.33, stream/drive/ms 13.05, render/graph/ms 5.79, pipelines/driver_ms 4.81, stream/activate/ms 1.71, render/present/ms 1.70, stream/spawn/ms 1.00 |
| warm-up | 4.85 | 32.8 | render/thread/ms 38.86, render/busy/ms 37.53, main/busy/ms 24.56, stream/drive/ms 4.79, render/graph/ms 4.59, stream/spawn/ms 1.96, stream/tick/ms 1.39, render/present/ms 1.03, terrain/update/ms 0.59, terrain/select/ms 0.49, bigspace/total/ms 0.33, render/acquire/ms 0.30 |
| warm-up | 5.00 | 25.2 | render/thread/ms 32.38, render/busy/ms 31.24, main/busy/ms 15.35, render/graph/ms 3.48, stream/drive/ms 2.69, stream/spawn/ms 0.97, render/present/ms 0.97, bigspace/total/ms 0.94, stream/tick/ms 0.58, terrain/update/ms 0.48, terrain/select/ms 0.40, bigspace/high_precision/ms 0.37 |
| warm-up | 5.07 | 27.3 | pipelines/wait_ms 1832, pipelines/compile_ms 128, pipelines/front_ms 121, render/thread/ms 39.37, render/busy/ms 38.30, main/busy/ms 18.54, pipelines/driver_ms 7.00, render/graph/ms 3.49, stream/drive/ms 3.14, render/present/ms 0.89, stream/tick/ms 0.66, bigspace/total/ms 0.59 |
| warm-up | 5.13 | 26.9 | pipelines/wait_ms 4115, render/thread/ms 40.64, render/busy/ms 38.91, main/busy/ms 23.60, pipelines/compile_ms 14.62, pipelines/front_ms 11.65, render/graph/ms 6.04, stream/drive/ms 4.38, pipelines/driver_ms 2.97, stream/spawn/ms 2.17, render/present/ms 1.50, stream/tick/ms 1.07 |
| warm-up | 5.17 | 38.1 | render/thread/ms 52.58, render/busy/ms 51.29, main/busy/ms 21.98, render/graph/ms 4.79, stream/drive/ms 4.09, stream/spawn/ms 2.05, render/present/ms 1.06, stream/tick/ms 1.02, terrain/update/ms 0.66, bigspace/total/ms 0.66, terrain/select/ms 0.54, bigspace/high_precision/ms 0.35 |
| warm-up | 5.20 | 25.5 | render/thread/ms 48.51, render/busy/ms 47.39, main/busy/ms 21.00, stream/drive/ms 3.96, render/graph/ms 3.82, stream/spawn/ms 2.04, render/present/ms 0.90, terrain/update/ms 0.74, bigspace/total/ms 0.74, stream/tick/ms 0.74, terrain/select/ms 0.61, bigspace/high_precision/ms 0.33 |
| warm-up | 5.29 | 30.2 | pipelines/wait_ms 5500, pipelines/compile_ms 44.15, pipelines/front_ms 41.69, render/thread/ms 36.99, render/busy/ms 34.97, main/busy/ms 31.38, stream/drive/ms 8.52, render/graph/ms 6.54, stream/spawn/ms 3.50, pipelines/driver_ms 2.47, render/present/ms 1.77, stream/tick/ms 1.42 |
| warm-up | 5.33 | 36.0 | pipelines/wait_ms 5500, pipelines/compile_ms 44.15, pipelines/front_ms 41.69, render/thread/ms 38.63, render/busy/ms 37.10, main/busy/ms 26.06, render/graph/ms 5.31, stream/drive/ms 5.08, stream/spawn/ms 2.76, pipelines/driver_ms 2.47, stream/tick/ms 1.52, render/present/ms 1.27 |
| warm-up | 5.49 | 26.6 | main/busy/ms 30.65, render/thread/ms 23.04, render/busy/ms 21.61, stream/drive/ms 17.43, render/graph/ms 5.44, stream/spawn/ms 1.44, render/present/ms 1.19, stream/tick/ms 0.87, bigspace/total/ms 0.63, terrain/update/ms 0.46, terrain/select/ms 0.39, bigspace/high_precision/ms 0.34 |
| warm-up | 5.66 | 43.7 | pipelines/wait_ms 7205, render/thread/ms 50.94, render/busy/ms 49.26, main/busy/ms 45.38, pipelines/compile_ms 18.72, pipelines/front_ms 16.86, render/graph/ms 7.29, terrain/update/ms 3.85, stream/drive/ms 3.39, pipelines/driver_ms 1.86, stream/spawn/ms 1.75, render/present/ms 1.41 |
| warm-up | 5.71 | 30.0 | render/thread/ms 40.75, render/busy/ms 39.59, main/busy/ms 31.38, render/graph/ms 4.89, stream/drive/ms 3.93, stream/spawn/ms 2.41, render/present/ms 0.87, bigspace/total/ms 0.69, stream/tick/ms 0.59, terrain/update/ms 0.49, terrain/select/ms 0.41, render/acquire/ms 0.29 |
| warm-up | 5.82 | 43.3 | render/thread/ms 37.67, render/busy/ms 36.23, main/busy/ms 35.88, stream/drive/ms 12.25, render/graph/ms 5.94, stream/spawn/ms 2.24, render/present/ms 1.22, stream/tick/ms 1.21, terrain/update/ms 0.80, bigspace/total/ms 0.79, terrain/select/ms 0.67, physics/insert/ms 0.35 |
| warm-up | 5.93 | 36.0 | pipelines/wait_ms 1587, main/busy/ms 40.59, render/thread/ms 35.29, render/busy/ms 33.50, pipelines/compile_ms 15.50, pipelines/front_ms 14.64, render/graph/ms 8.94, render/present/ms 1.54, stream/drive/ms 1.35, bigspace/total/ms 1.03, pipelines/driver_ms 0.87, stream/tick/ms 0.62 |
| warm-up | 5.96 | 26.2 | render/thread/ms 40.07, render/busy/ms 38.76, main/busy/ms 37.82, render/graph/ms 5.51, stream/drive/ms 1.26, render/present/ms 1.02, bigspace/total/ms 0.70, terrain/update/ms 0.63, stream/tick/ms 0.58, terrain/select/ms 0.53, anim/evaluate_cpu/ms 0.41, stream/spawn/ms 0.30 |
| warm-up | 6.04 | 50.0 | pipelines/wait_ms 4047, main/busy/ms 59.22, render/thread/ms 37.71, render/busy/ms 35.59, pipelines/compile_ms 30.30, pipelines/front_ms 29.05, render/graph/ms 12.14, render/present/ms 1.92, stream/drive/ms 1.44, pipelines/driver_ms 1.25, anim/evaluate_cpu/ms 0.97, bigspace/total/ms 0.83 |
| warm-up | 6.07 | 33.8 | pipelines/wait_ms 4047, render/thread/ms 45.27, render/busy/ms 44.05, main/busy/ms 36.36, pipelines/compile_ms 30.30, pipelines/front_ms 29.05, render/graph/ms 8.70, pipelines/driver_ms 1.25, stream/drive/ms 1.23, render/present/ms 1.04, bigspace/total/ms 0.80, anim/evaluate_cpu/ms 0.75 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **90%** (min 74%) · 2580 MHz (min 2580) · memory 8001 MHz · 88.4 W · 75 °C (max 79) · VRAM 5452 MiB · throttled 100% of samples
