# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5829 frames over 65.6 s · **median 10.96 ms (91 fps)** · mean 11.26 · p95 14.10 · p99 16.58 (1% low 60 fps) · worst 67.87 ms · 64 spikes over 1.5× median · 30.2 stutters/min (work 24.7/min)
- warm-up: 691 frames over 12.2 s, worst 2556.8 ms
- hitches (> 27.4 ms): **9** measured, 32 in warm-up
- timed GPU passes: 6.74 ms of a 11.26 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.96 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- **FAIL** worst frame 67.87 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.162 | 2.189 | 2.551 | 0.99 | 0.183 |
| main_opaque_pass_3d | 1.404 | 1.421 | 2.055 | 0.99 | 0.053 |
| early prepass | 0.492 | 0.498 | 0.724 | 0.99 | 0.094 |
| bloom | 0.331 | 0.335 | 0.493 | 0.99 | 0.067 |
| atmosphere_luts | 0.321 | 0.324 | 0.449 | 0.99 | 0.003 |
| water_velocity | 0.283 | 0.286 | 0.355 | 0.99 | 0.002 |
| uniform_allocation | 0.223 | 0.225 | 0.234 | 0.99 | 0.040 |
| water_sspr | 0.218 | 0.221 | 0.221 | 0.99 | 0.002 |
| ssao | 0.204 | 0.206 | 0.217 | 0.99 | 0.008 |
| bin_unpacking | 0.180 | 0.182 | 0.194 | 0.99 | 0.028 |
| auxiliary_views | 0.159 | 0.161 | 0.665 | 0.99 | 0.146 |
| upscaling | 0.142 | 0.144 | 0.184 | 0.99 | 0.004 |
| water_fft | 0.138 | 0.139 | 0.140 | 0.99 | 0.003 |
| tonemapping | 0.092 | 0.093 | 0.101 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.085 | 0.086 | 0.151 | 0.99 | 0.009 |
| clustering | 0.067 | 0.068 | 0.070 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.049 | 0.99 | 0.005 |
| surface_trail | 0.042 | 0.059 | 0.057 | 0.72 | 0.002 |
| water_snapshot | 0.035 | 0.036 | 0.039 | 0.99 | 0.002 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.006 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.023 | 0.99 | 0.076 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.99 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.008 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 65.229 | 0.000 | 25892.66 | 380219.7 |
| render/thread/ms | 10.095 | 12.819 | 64.96 | 58844.0 |
| render/busy/ms | 9.519 | 12.210 | 64.03 | 55485.6 |
| main/busy/ms | 8.616 | 11.124 | 30.41 | 50223.7 |
| pipelines/compile_ms | 5.522 | 0.000 | 2617.79 | 32185.7 |
| render/graph/ms | 5.497 | 7.099 | 53.60 | 32040.1 |
| pipelines/driver_ms | 4.888 | 0.000 | 2479.65 | 28490.9 |
| pipelines/front_ms | 0.634 | 0.000 | 162.28 | 3694.8 |
| bigspace/total/ms | 0.565 | 1.557 | 3.38 | 3294.4 |
| render/present/ms | 0.482 | 0.601 | 15.56 | 2811.8 |
| terrain/update/ms | 0.302 | 0.389 | 2.21 | 1761.7 |
| controller/feed/ms | 0.276 | 0.380 | 0.72 | 1609.5 |
| terrain/select/ms | 0.269 | 0.345 | 0.71 | 1567.9 |
| stream/drive/ms | 0.251 | 0.380 | 22.48 | 1465.5 |
| bigspace/low_precision/ms | 0.223 | 0.647 | 2.63 | 1300.5 |
| stream/tick/ms | 0.194 | 0.291 | 0.67 | 1132.6 |
| far/select/ms | 0.194 | 0.273 | 0.62 | 1132.0 |
| bigspace/high_precision/ms | 0.184 | 0.743 | 3.02 | 1071.4 |
| anim/evaluate_cpu/ms | 0.157 | 0.205 | 0.53 | 916.5 |
| anim/evaluate/ms | 0.141 | 0.272 | 2.80 | 821.2 |
| controller/move/ms | 0.132 | 0.235 | 0.50 | 770.9 |
| render/acquire/ms | 0.094 | 0.128 | 0.52 | 546.5 |
| physics/insert/ms | 0.058 | 0.444 | 0.95 | 336.2 |
| physics/step/ms | 0.049 | 0.089 | 0.99 | 287.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 65.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.21 | 64.8 |
| controller/avoid/ms | 0.010 | 0.013 | 0.09 | 57.9 |
| script/update/ms | 0.007 | 0.011 | 0.07 | 43.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.39 | 31.3 |
| script/input/ms | 0.005 | 0.007 | 0.17 | 29.9 |
| script/ui/ms | 0.005 | 0.007 | 0.11 | 28.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 26.2 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 20.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.59 | 14.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.29 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.23 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 9 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 41 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1277 | 2262 | 707 |
| crowd/c0/meshes/count | count | 352 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5481 | 5991 | 5489 |
| crowd/c1/meshes/count | count | 1158 | 1295 | 1152 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4258 | 10.2 k | 6179 |
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
| frame/work/ms | ms | 9.36 | 64.03 | 8.14 |
| gpu/timed/ms | ms | 6.90 | 10.17 | 7.17 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1281 | 1395 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3361 | 5122 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1281 | 1395 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3361 | 5122 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 299 | 44 |
| phases/shadow/bins | count | 3746 | 5382 | 2256 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9379 | 12.5 k | 5753 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 134 | 309 | 44 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 1454 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 198 | 203 | 203 |
| pipelines/waiting | count | 0 | 44 | 0 |
| preprocess/bind_groups | count | 10 | 767 | 6 |
| preprocess/bind_groups_created | count | 10 | 767 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 230 | 342 | 225 |
| process/cpu_usage | % | 25.60 | 51.83 | 23.56 |
| process/mem_usage | GiB | 5.20 | 5.33 | 5.33 |
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
| started_load_count |  | 4755 | 4847 | 4847 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.79 | 70.49 | 32.03 |
| system/mem_usage | % | 65.07 | 65.59 | 65.54 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2805 | 2921 | 2921 |
| textures/images | count | 2580 | 2640 | 2640 |
| views/all/batch_sets | count | 112 | 351 | 104 |
| views/all/bins | count | 6270 | 8291 | 5736 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 184 | 362 | 100 |
| views/all/pipelines_distinct | count | 83 | 95 | 80 |
| views/all/work_items | count | 17.0 k | 24.1 k | 13.7 k |
| views/cascade/batch_sets | count | 52 | 67 | 44 |
| views/cascade/bins | count | 2607 | 3135 | 2256 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8097 | 10.1 k | 5753 |
| views/cascade/pipelines | count | 52 | 67 | 44 |
| views/cascade/work_items | count | 8097 | 10.1 k | 5753 |
| views/lamp_face/batch_sets | count | 0 | 243 | 0 |
| views/lamp_face/bins | count | 1037 | 2463 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2822 | 0 |
| views/lamp_face/pipelines | count | 81 | 252 | 0 |
| views/lamp_face/work_items | count | 0 | 2586 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2858 | 3588 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8312 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8482 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 50.69 | 297 | 295.5 k |
| anim/events/count | count | 0.09 | 3 | 543 |
| assets/Image/added | count | 0.11 | 40 | 661 |
| assets/Image/modified | count | 0.00 | 6 | 21 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.74 | 152 | 4291 |
| assets/Mesh/removed | count | 0.84 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 545 |
| assets/StandardMaterial/modified | count | 3.73 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 78.96 | 2167 | 460.2 k |
| ecs/entities_delta | count | 1.49 | 1301 | 8678 |
| physics/inserts | count | 0.61 | 120 | 3570 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0.00 | 3 | 7 |
| pipelines/created | count | 0.00 | 3 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5808 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1273 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2556.8 | main/busy/ms 2527, surface/advance_cpu_ms 75.21, surface/cell_upload_ms 0.73, stream/drive/ms 0.33, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.04, anim/evaluate/ms 0.01, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.26 | 704.5 | main/busy/ms 2564, render/thread/ms 665, render/busy/ms 538, render/acquire/ms 125, surface/advance_cpu_ms 75.21, render/graph/ms 4.78, render/present/ms 2.39, stream/drive/ms 1.10, terrain/update/ms 0.78, surface/cell_upload_ms 0.73, terrain/select/ms 0.58, stream/tick/ms 0.54 |
| warm-up | 3.52 | 263.3 | pipelines/wait_ms 3737, render/thread/ms 678, render/busy/ms 549, pipelines/compile_ms 281, pipelines/front_ms 261, render/acquire/ms 125, main/busy/ms 56.77, terrain/update/ms 20.11, pipelines/driver_ms 19.94, render/graph/ms 9.03, render/present/ms 3.36, bigspace/total/ms 1.40 |
| warm-up | 3.56 | 31.1 | pipelines/wait_ms 3737, pipelines/compile_ms 281, pipelines/front_ms 261, main/busy/ms 35.21, render/thread/ms 35.05, render/busy/ms 32.45, terrain/update/ms 25.82, render/graph/ms 20.27, pipelines/driver_ms 19.94, render/present/ms 2.19, bigspace/total/ms 1.32, stream/drive/ms 1.23 |
| warm-up | 3.78 | 95.5 | pipelines/compile_ms 194, pipelines/front_ms 177, pipelines/wait_ms 176, main/busy/ms 72.29, render/thread/ms 57.13, render/busy/ms 55.48, pipelines/driver_ms 16.72, terrain/update/ms 4.29, render/graph/ms 3.41, render/present/ms 1.45, stream/drive/ms 1.29, stream/tick/ms 1.07 |
| warm-up | 3.93 | 63.4 | pipelines/wait_ms 890, pipelines/compile_ms 237, pipelines/front_ms 228, main/busy/ms 46.87, render/thread/ms 44.11, render/busy/ms 42.87, stream/drive/ms 33.72, pipelines/driver_ms 9.20, render/graph/ms 2.98, render/present/ms 1.01, terrain/update/ms 0.99, stream/tick/ms 0.93 |
| warm-up | 3.97 | 29.9 | pipelines/wait_ms 406, main/busy/ms 35.11, render/thread/ms 28.33, render/busy/ms 26.34, stream/drive/ms 17.80, render/graph/ms 4.65, pipelines/compile_ms 2.63, pipelines/front_ms 2.15, render/present/ms 1.73, stream/tick/ms 1.69, stream/spawn/ms 1.33, terrain/update/ms 0.62 |
| warm-up | 4.01 | 35.0 | pipelines/wait_ms 1243, pipelines/compile_ms 64.13, pipelines/front_ms 57.18, render/thread/ms 43.76, render/busy/ms 41.68, main/busy/ms 28.53, pipelines/driver_ms 6.96, render/graph/ms 5.45, stream/drive/ms 5.06, stream/tick/ms 2.42, render/present/ms 1.86, stream/spawn/ms 1.56 |
| warm-up | 4.04 | 31.8 | pipelines/wait_ms 1702, pipelines/compile_ms 92.12, pipelines/front_ms 83.54, render/thread/ms 46.48, render/busy/ms 44.61, main/busy/ms 38.69, stream/drive/ms 16.89, pipelines/driver_ms 8.58, render/graph/ms 4.00, stream/tick/ms 1.72, render/present/ms 1.59, terrain/update/ms 1.07 |
| warm-up | 4.10 | 57.7 | pipelines/wait_ms 865, render/thread/ms 57.97, render/busy/ms 56.44, main/busy/ms 38.19, pipelines/compile_ms 30.62, pipelines/front_ms 28.52, stream/drive/ms 17.28, render/graph/ms 2.87, pipelines/driver_ms 2.10, stream/tick/ms 1.35, render/present/ms 1.26, stream/spawn/ms 1.12 |
| warm-up | 4.18 | 62.4 | main/busy/ms 50.67, render/thread/ms 33.07, render/busy/ms 31.83, stream/drive/ms 30.94, stream/spawn/ms 3.55, render/graph/ms 2.73, render/present/ms 1.08, stream/tick/ms 1.08, terrain/update/ms 0.96, terrain/select/ms 0.74, bigspace/total/ms 0.46, bigspace/low_precision/ms 0.19 |
| warm-up | 4.21 | 34.4 | pipelines/wait_ms 672, render/thread/ms 46.83, render/busy/ms 45.20, main/busy/ms 25.96, stream/drive/ms 5.96, render/graph/ms 4.36, pipelines/compile_ms 3.36, pipelines/front_ms 3.10, stream/spawn/ms 2.98, render/present/ms 1.35, stream/tick/ms 1.22, terrain/update/ms 0.95 |
| warm-up | 4.27 | 35.9 | pipelines/wait_ms 651, render/thread/ms 46.78, render/busy/ms 45.29, main/busy/ms 14.30, render/graph/ms 10.71, stream/drive/ms 2.14, render/present/ms 1.20, terrain/update/ms 0.93, stream/tick/ms 0.80, terrain/select/ms 0.75, pipelines/compile_ms 0.68, pipelines/front_ms 0.58 |
| warm-up | 4.31 | 42.4 | pipelines/wait_ms 3577, pipelines/compile_ms 93.96, pipelines/front_ms 84.05, render/thread/ms 70.41, render/busy/ms 68.23, render/graph/ms 28.93, main/busy/ms 20.36, pipelines/driver_ms 9.91, stream/drive/ms 4.60, stream/spawn/ms 2.40, render/present/ms 1.84, stream/tick/ms 0.96 |
| warm-up | 4.39 | 53.2 | pipelines/wait_ms 14.4 k, pipelines/compile_ms 93.16, pipelines/front_ms 85.67, main/busy/ms 83.31, stream/drive/ms 59.38, render/thread/ms 46.79, render/busy/ms 45.51, pipelines/driver_ms 7.48, render/graph/ms 3.65, stream/spawn/ms 2.99, stream/tick/ms 1.60, render/present/ms 1.08 |
| warm-up | 4.44 | 52.5 | pipelines/wait_ms 14.6 k, main/busy/ms 79.91, pipelines/compile_ms 78.34, pipelines/front_ms 71.71, stream/drive/ms 57.92, render/thread/ms 42.05, render/busy/ms 40.83, pipelines/driver_ms 6.63, terrain/update/ms 5.41, render/graph/ms 4.85, stream/spawn/ms 2.30, render/present/ms 1.02 |
| warm-up | 4.55 | 92.6 | pipelines/wait_ms 6050, pipelines/compile_ms 107, pipelines/front_ms 100, main/busy/ms 82.35, stream/drive/ms 50.09, render/thread/ms 39.29, render/busy/ms 38.05, pipelines/driver_ms 6.97, render/graph/ms 4.69, terrain/update/ms 2.19, stream/spawn/ms 1.69, render/present/ms 1.02 |
| warm-up | 4.62 | 53.8 | main/busy/ms 53.18, stream/drive/ms 37.52, render/thread/ms 35.80, render/busy/ms 34.41, render/graph/ms 7.50, render/present/ms 1.17, stream/activate/ms 1.03, terrain/update/ms 0.72, stream/tick/ms 0.70, stream/spawn/ms 0.69, terrain/select/ms 0.61, physics/step/ms 0.48 |
| warm-up | 4.67 | 40.3 | pipelines/wait_ms 5555, main/busy/ms 42.07, pipelines/compile_ms 29.94, stream/drive/ms 28.56, pipelines/front_ms 27.04, render/thread/ms 19.09, render/busy/ms 17.96, render/graph/ms 5.11, pipelines/driver_ms 2.90, stream/activate/ms 1.48, stream/spawn/ms 1.25, render/present/ms 0.93 |
| warm-up | 4.83 | 40.1 | pipelines/wait_ms 4829, pipelines/compile_ms 111, pipelines/front_ms 104, render/thread/ms 52.26, render/busy/ms 50.94, main/busy/ms 38.79, stream/drive/ms 14.20, pipelines/driver_ms 7.67, render/graph/ms 5.77, stream/tick/ms 1.89, render/present/ms 1.03, terrain/update/ms 0.90 |
| warm-up | 4.86 | 30.1 | pipelines/wait_ms 1223, render/thread/ms 55.71, render/busy/ms 54.28, main/busy/ms 34.52, stream/drive/ms 12.79, pipelines/compile_ms 8.35, pipelines/front_ms 7.94, render/graph/ms 5.74, stream/tick/ms 1.17, render/present/ms 1.14, terrain/update/ms 0.86, bigspace/total/ms 0.73 |
| warm-up | 5.11 | 28.3 | render/thread/ms 34.49, render/busy/ms 33.00, main/busy/ms 26.21, render/graph/ms 8.97, stream/drive/ms 7.71, physics/insert/ms 1.36, stream/spawn/ms 1.27, render/present/ms 1.17, stream/tick/ms 0.85, terrain/update/ms 0.78, terrain/select/ms 0.65, bigspace/total/ms 0.62 |
| warm-up | 5.17 | 29.7 | main/busy/ms 32.48, stream/drive/ms 17.10, render/thread/ms 16.50, render/busy/ms 15.22, render/graph/ms 4.96, render/present/ms 1.12, stream/tick/ms 0.86, terrain/update/ms 0.82, terrain/select/ms 0.68, bigspace/total/ms 0.65, physics/step/ms 0.47, physics/insert/ms 0.39 |
| warm-up | 5.42 | 53.2 | render/thread/ms 47.34, render/busy/ms 46.00, main/busy/ms 31.86, stream/drive/ms 19.58, render/graph/ms 5.92, stream/spawn/ms 1.15, render/present/ms 1.10, stream/tick/ms 0.79, terrain/update/ms 0.63, terrain/select/ms 0.53, bigspace/total/ms 0.50, stream/activate/ms 0.41 |
| warm-up | 5.49 | 29.4 | render/thread/ms 39.25, render/busy/ms 37.16, main/busy/ms 27.48, render/graph/ms 12.57, stream/drive/ms 5.55, render/present/ms 1.80, stream/spawn/ms 1.71, stream/tick/ms 0.81, terrain/update/ms 0.74, bigspace/total/ms 0.65, terrain/select/ms 0.63, stream/activate/ms 0.38 |
| warm-up | 5.55 | 33.5 | render/thread/ms 36.14, render/busy/ms 34.83, main/busy/ms 23.91, render/graph/ms 6.20, stream/drive/ms 3.77, stream/spawn/ms 1.97, bigspace/total/ms 1.65, bigspace/high_precision/ms 1.36, stream/tick/ms 1.16, render/present/ms 1.06, terrain/update/ms 0.87, terrain/select/ms 0.74 |
| warm-up | 5.76 | 27.6 | render/thread/ms 36.86, render/busy/ms 34.92, main/busy/ms 32.14, render/graph/ms 9.82, stream/drive/ms 3.68, stream/spawn/ms 2.07, bigspace/total/ms 1.83, render/present/ms 1.57, terrain/update/ms 1.08, terrain/select/ms 0.89, stream/tick/ms 0.75, bigspace/low_precision/ms 0.64 |
| warm-up | 6.02 | 31.0 | render/thread/ms 43.30, render/busy/ms 41.75, main/busy/ms 37.64, stream/drive/ms 19.58, render/graph/ms 7.86, render/present/ms 1.33, stream/spawn/ms 1.27, terrain/update/ms 0.78, stream/tick/ms 0.77, bigspace/total/ms 0.67, terrain/select/ms 0.65, bigspace/high_precision/ms 0.27 |
| warm-up | 7.38 | 45.0 | main/busy/ms 54.56, render/thread/ms 41.20, render/busy/ms 39.58, render/graph/ms 11.13, terrain/update/ms 2.22, stream/drive/ms 1.37, render/present/ms 1.30, bigspace/total/ms 0.70, stream/tick/ms 0.59, far/select/ms 0.58, terrain/select/ms 0.52, physics/insert/ms 0.42 |
| warm-up | 7.41 | 32.8 | render/thread/ms 49.79, render/busy/ms 48.26, main/busy/ms 47.69, render/graph/ms 8.13, terrain/update/ms 2.20, stream/drive/ms 1.56, render/present/ms 1.20, bigspace/total/ms 0.79, stream/tick/ms 0.64, physics/step/ms 0.58, far/select/ms 0.57, anim/evaluate_cpu/ms 0.55 |
| warm-up | 11.02 | 31.2 | main/busy/ms 53.91, render/thread/ms 36.35, render/busy/ms 34.72, render/graph/ms 11.84, anim/evaluate_cpu/ms 1.65, render/present/ms 1.36, bigspace/total/ms 1.33, bigspace/low_precision/ms 0.80, anim/evaluate/ms 0.62, terrain/update/ms 0.56, terrain/select/ms 0.50, stream/drive/ms 0.49 |
| warm-up | 11.06 | 37.9 | pipelines/wait_ms 8952, main/busy/ms 49.14, render/thread/ms 38.46, render/busy/ms 37.16, pipelines/compile_ms 21.10, pipelines/front_ms 20.26, render/graph/ms 7.94, anim/evaluate_cpu/ms 1.30, bigspace/total/ms 1.23, render/present/ms 1.01, pipelines/driver_ms 0.84, stream/drive/ms 0.73 |
| measure | 0.79 | 28.5 | main/busy/ms 35.97, render/thread/ms 27.58, render/busy/ms 26.21, stream/drive/ms 15.10, render/graph/ms 10.95, render/present/ms 1.17, controller/feed/ms 0.66, terrain/update/ms 0.61, bigspace/total/ms 0.60, stream/tick/ms 0.59, terrain/select/ms 0.54, far/select/ms 0.44 |
| measure | 2.18 | 27.9 | render/thread/ms 37.65, render/busy/ms 35.96, render/graph/ms 21.26, main/busy/ms 20.81, render/present/ms 1.50, physics/insert/ms 1.23, bigspace/total/ms 0.81, terrain/update/ms 0.62, stream/drive/ms 0.57, terrain/select/ms 0.55, stream/tick/ms 0.49, controller/feed/ms 0.45 |
| measure | 2.46 | 28.3 | render/thread/ms 39.64, main/busy/ms 25.18, render/busy/ms 23.05, render/present/ms 16.35, render/graph/ms 13.18, physics/insert/ms 1.19, bigspace/total/ms 0.84, controller/feed/ms 0.72, far/select/ms 0.66, terrain/update/ms 0.61, stream/drive/ms 0.59, terrain/select/ms 0.55 |
| measure | 2.76 | 67.9 | render/thread/ms 75.40, render/busy/ms 73.84, render/graph/ms 58.96, main/busy/ms 22.69, bigspace/total/ms 2.55, bigspace/low_precision/ms 1.81, render/present/ms 1.34, stream/drive/ms 1.07, stream/tick/ms 0.99, terrain/update/ms 0.63, terrain/select/ms 0.56, physics/insert/ms 0.54 |
| measure | 4.90 | 39.1 | pipelines/wait_ms 18.8 k, pipelines/compile_ms 658, pipelines/driver_ms 541, pipelines/front_ms 118, main/busy/ms 39.50, render/thread/ms 28.05, render/busy/ms 26.79, stream/drive/ms 13.48, render/graph/ms 10.48, physics/insert/ms 1.48, render/present/ms 1.05, controller/feed/ms 0.94 |
| measure | 17.50 | 28.3 | render/thread/ms 38.73, render/busy/ms 23.70, main/busy/ms 21.16, render/present/ms 14.81, render/graph/ms 14.18, bigspace/total/ms 0.82, controller/feed/ms 0.78, terrain/update/ms 0.56, stream/drive/ms 0.56, far/select/ms 0.53, terrain/select/ms 0.51, stream/tick/ms 0.50 |
| measure | 18.18 | 32.7 | main/busy/ms 39.63, render/thread/ms 23.49, stream/drive/ms 22.70, render/busy/ms 22.31, render/graph/ms 12.48, render/present/ms 1.01, bigspace/total/ms 0.79, controller/feed/ms 0.61, terrain/update/ms 0.59, terrain/select/ms 0.54, stream/tick/ms 0.36, far/select/ms 0.35 |
| measure | 19.27 | 32.0 | render/thread/ms 43.63, render/busy/ms 42.16, render/graph/ms 26.02, main/busy/ms 23.56, render/present/ms 1.27, physics/insert/ms 0.90, bigspace/total/ms 0.81, controller/feed/ms 0.62, terrain/update/ms 0.58, stream/drive/ms 0.55, terrain/select/ms 0.53, stream/tick/ms 0.48 |
| measure | 59.04 | 28.0 | render/thread/ms 40.27, render/busy/ms 38.80, main/busy/ms 27.60, render/graph/ms 13.74, render/present/ms 1.16, bigspace/total/ms 1.16, terrain/update/ms 0.93, terrain/select/ms 0.83, controller/feed/ms 0.82, stream/drive/ms 0.73, anim/evaluate_cpu/ms 0.68, far/select/ms 0.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 12%, CPU 10%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **78%** (min 59%) · 2579 MHz (min 2565) · memory 8001 MHz · 77.2 W · 76 °C (max 81) · VRAM 6260 MiB · throttled 100% of samples
