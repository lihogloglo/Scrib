# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5711 frames over 65.6 s · **median 11.15 ms (90 fps)** · mean 11.49 · p95 14.65 · p99 17.14 (1% low 58 fps) · worst 33.23 ms · 70 spikes over 1.5× median · 29.2 stutters/min (work 33.8/min)
- warm-up: 329 frames over 8.4 s, worst 2617.3 ms
- hitches (> 27.9 ms): **4** measured, 24 in warm-up
- timed GPU passes: 5.93 ms of a 11.49 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.15 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- **FAIL** worst frame 33.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.075 | 2.076 | 2.320 | 1.00 | 0.189 |
| main_opaque_pass_3d | 1.291 | 1.291 | 1.729 | 1.00 | 0.065 |
| early prepass | 0.421 | 0.421 | 0.630 | 1.00 | 0.097 |
| atmosphere_luts | 0.316 | 0.316 | 0.376 | 1.00 | 0.004 |
| water_velocity | 0.283 | 0.283 | 0.351 | 1.00 | 0.002 |
| uniform_allocation | 0.222 | 0.222 | 0.237 | 1.00 | 0.044 |
| water_sspr | 0.211 | 0.211 | 0.211 | 1.00 | 0.002 |
| auxiliary_views | 0.191 | 0.191 | 0.652 | 1.00 | 0.192 |
| bin_unpacking | 0.171 | 0.171 | 0.190 | 1.00 | 0.030 |
| upscaling | 0.147 | 0.147 | 0.187 | 1.00 | 0.005 |
| water_fft | 0.136 | 0.136 | 0.139 | 1.00 | 0.003 |
| tonemapping | 0.123 | 0.123 | 0.132 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.084 | 0.084 | 0.147 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.023 | 1.00 | 0.079 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 10.493 | 0.000 | 12773.80 | 59923.6 |
| render/thread/ms | 10.303 | 13.287 | 31.10 | 58843.3 |
| render/busy/ms | 9.744 | 12.675 | 30.48 | 55648.3 |
| main/busy/ms | 8.875 | 11.288 | 29.28 | 50686.3 |
| render/graph/ms | 5.480 | 7.250 | 26.39 | 31297.6 |
| bigspace/total/ms | 0.550 | 1.506 | 3.36 | 3140.5 |
| render/present/ms | 0.463 | 0.579 | 4.48 | 2646.1 |
| terrain/update/ms | 0.313 | 0.408 | 2.33 | 1787.9 |
| controller/feed/ms | 0.295 | 0.401 | 0.86 | 1686.4 |
| terrain/select/ms | 0.278 | 0.362 | 0.89 | 1588.9 |
| pipelines/compile_ms | 0.256 | 0.000 | 160.56 | 1463.7 |
| stream/drive/ms | 0.253 | 0.379 | 20.99 | 1442.3 |
| pipelines/front_ms | 0.246 | 0.000 | 154.21 | 1407.3 |
| bigspace/low_precision/ms | 0.220 | 0.545 | 2.89 | 1255.9 |
| far/select/ms | 0.201 | 0.290 | 0.75 | 1147.9 |
| stream/tick/ms | 0.195 | 0.288 | 0.76 | 1115.2 |
| bigspace/high_precision/ms | 0.168 | 0.568 | 3.03 | 959.3 |
| anim/evaluate_cpu/ms | 0.163 | 0.217 | 0.51 | 929.9 |
| anim/evaluate/ms | 0.141 | 0.271 | 2.38 | 804.6 |
| controller/move/ms | 0.140 | 0.277 | 0.73 | 799.0 |
| render/acquire/ms | 0.096 | 0.130 | 2.09 | 548.9 |
| physics/step/ms | 0.051 | 0.093 | 0.67 | 293.1 |
| physics/insert/ms | 0.049 | 0.434 | 0.86 | 277.4 |
| anim/apply/ms | 0.012 | 0.023 | 0.26 | 70.3 |
| weather/eval/ms | 0.012 | 0.016 | 0.16 | 66.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.13 | 59.2 |
| pipelines/driver_ms | 0.010 | 0.000 | 6.35 | 56.3 |
| script/update/ms | 0.008 | 0.011 | 0.06 | 43.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.03 | 30.8 |
| script/input/ms | 0.005 | 0.008 | 0.15 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.16 | 27.1 |
| script/ui/ms | 0.005 | 0.007 | 0.17 | 26.2 |
| controller/write/ms | 0.004 | 0.005 | 0.16 | 21.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.24 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.18 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 17 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 48 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1269 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5613 |
| crowd/c1/meshes/count | count | 1165 | 1268 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4251 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.53 | 30.48 | 7.88 |
| gpu/timed/ms | ms | 5.94 | 9.16 | 6.14 |
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
| phases/alpha_mask3d/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3385 | 5114 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3385 | 5114 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58 | 290 | 42 |
| phases/shadow/bins | count | 3804 | 5496 | 2250 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9677 | 12.6 k | 5747 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 138 | 304 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 592 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 193 | 198 | 198 |
| pipelines/waiting | count | 0 | 17 | 0 |
| preprocess/bind_groups | count | 11 | 749 | 6 |
| preprocess/bind_groups_created | count | 11 | 749 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 232 | 364 | 221 |
| process/cpu_usage | % | 25.26 | 38.46 | 19.23 |
| process/mem_usage | GiB | 4.72 | 4.86 | 4.86 |
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
| started_load_count |  | 4750 | 4844 | 4844 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.79 | 66.46 | 30.55 |
| system/mem_usage | % | 63.22 | 63.76 | 63.02 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2921 | 2921 |
| textures/images | count | 2582 | 2640 | 2640 |
| views/all/batch_sets | count | 115 | 342 | 102 |
| views/all/bins | count | 6501 | 8242 | 5730 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 191 | 360 | 98 |
| views/all/pipelines_distinct | count | 84 | 95 | 80 |
| views/all/work_items | count | 17.5 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 62 | 42 |
| views/cascade/bins | count | 2630 | 2973 | 2250 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8226 | 9723 | 5747 |
| views/cascade/pipelines | count | 52 | 62 | 42 |
| views/cascade/work_items | count | 8226 | 9723 | 5747 |
| views/lamp_face/batch_sets | count | 6 | 233 | 0 |
| views/lamp_face/bins | count | 1092 | 2529 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1243 | 2903 | 0 |
| views/lamp_face/pipelines | count | 85 | 247 | 0 |
| views/lamp_face/work_items | count | 79 | 2547 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2880 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8370 | 13.0 k | 7592 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8543 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 61.00 | 590 | 348.4 k |
| anim/events/count | count | 0.09 | 3 | 533 |
| assets/Image/added | count | 0.11 | 36 | 627 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.73 | 152 | 4178 |
| assets/Mesh/removed | count | 0.85 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 504 |
| assets/StandardMaterial/modified | count | 3.81 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.42 | 1644 | 8112 |
| physics/inserts | count | 0.50 | 91 | 2864 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.69 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 5928 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1300 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.9 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2617.3 | main/busy/ms 2586, surface/advance_cpu_ms 70.07, surface/cell_upload_ms 0.84, stream/drive/ms 0.32, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.48 | 865.1 | main/busy/ms 2625, render/thread/ms 821, render/busy/ms 652, render/acquire/ms 166, surface/advance_cpu_ms 70.07, render/graph/ms 6.85, render/present/ms 3.36, stream/drive/ms 1.09, surface/cell_upload_ms 0.84, terrain/update/ms 0.57, stream/tick/ms 0.56, terrain/select/ms 0.43 |
| warm-up | 3.57 | 90.4 | render/thread/ms 836, render/busy/ms 666, pipelines/wait_ms 473, render/acquire/ms 166, pipelines/compile_ms 87.40, pipelines/front_ms 80.57, main/busy/ms 58.43, terrain/update/ms 22.93, render/graph/ms 9.99, pipelines/driver_ms 6.83, render/present/ms 4.14, stream/drive/ms 1.54 |
| warm-up | 3.81 | 87.8 | pipelines/compile_ms 217, pipelines/front_ms 195, pipelines/wait_ms 167, render/thread/ms 71.54, main/busy/ms 70.88, render/busy/ms 68.48, pipelines/driver_ms 21.33, render/graph/ms 5.74, stream/drive/ms 4.72, stream/tick/ms 2.68, render/present/ms 1.94, render/acquire/ms 1.12 |
| warm-up | 3.99 | 55.3 | render/thread/ms 53.08, render/busy/ms 51.56, main/busy/ms 40.42, stream/drive/ms 26.06, render/graph/ms 2.91, stream/tick/ms 1.76, render/present/ms 1.32, terrain/update/ms 0.77, terrain/select/ms 0.64, bigspace/total/ms 0.23, render/acquire/ms 0.20, stream/activate/ms 0.19 |
| warm-up | 4.09 | 37.3 | pipelines/wait_ms 2525, pipelines/compile_ms 261, pipelines/front_ms 243, render/thread/ms 44.75, render/busy/ms 42.80, main/busy/ms 37.41, stream/drive/ms 21.33, pipelines/driver_ms 17.71, render/graph/ms 5.31, render/present/ms 1.70, stream/tick/ms 1.67, terrain/update/ms 0.95 |
| warm-up | 4.15 | 64.6 | pipelines/wait_ms 3354, pipelines/compile_ms 83.53, render/thread/ms 79.68, pipelines/front_ms 77.14, render/busy/ms 77.07, main/busy/ms 41.46, stream/drive/ms 22.31, render/graph/ms 9.40, pipelines/driver_ms 6.39, render/present/ms 2.13, stream/spawn/ms 1.55, stream/tick/ms 0.97 |
| warm-up | 4.30 | 127.8 | pipelines/wait_ms 2409, pipelines/compile_ms 158, pipelines/front_ms 151, render/thread/ms 93.71, render/busy/ms 60.83, main/busy/ms 55.99, render/present/ms 32.70, stream/drive/ms 29.16, pipelines/driver_ms 6.90, render/graph/ms 3.40, stream/spawn/ms 2.40, stream/tick/ms 1.26 |
| warm-up | 4.35 | 44.1 | pipelines/wait_ms 5026, pipelines/compile_ms 289, pipelines/front_ms 198, render/thread/ms 111, pipelines/driver_ms 90.53, render/busy/ms 77.78, main/busy/ms 34.21, render/present/ms 32.77, stream/drive/ms 4.93, render/graph/ms 4.31, stream/tick/ms 1.27, stream/spawn/ms 1.22 |
| warm-up | 4.43 | 31.4 | pipelines/wait_ms 6076, pipelines/compile_ms 114, pipelines/front_ms 106, render/thread/ms 48.41, render/busy/ms 46.22, main/busy/ms 27.64, render/graph/ms 24.14, stream/drive/ms 12.93, pipelines/driver_ms 7.81, render/present/ms 2.01, stream/spawn/ms 1.15, stream/tick/ms 0.74 |
| warm-up | 4.53 | 82.4 | pipelines/wait_ms 1750, pipelines/compile_ms 77.67, pipelines/front_ms 74.49, main/busy/ms 63.66, render/thread/ms 46.97, render/busy/ms 45.30, stream/drive/ms 44.23, render/graph/ms 5.39, terrain/update/ms 4.37, pipelines/driver_ms 3.18, render/present/ms 1.34, stream/tick/ms 1.15 |
| warm-up | 4.63 | 85.3 | pipelines/wait_ms 2266, pipelines/compile_ms 136, pipelines/front_ms 110, main/busy/ms 56.19, render/thread/ms 45.47, stream/drive/ms 40.70, pipelines/driver_ms 26.11, render/busy/ms 23.37, render/present/ms 21.93, render/graph/ms 5.63, terrain/update/ms 2.11, stream/spawn/ms 2.04 |
| warm-up | 4.71 | 61.6 | pipelines/wait_ms 7138, pipelines/compile_ms 113, pipelines/front_ms 89.63, main/busy/ms 63.07, render/thread/ms 41.69, render/busy/ms 40.15, stream/drive/ms 38.40, pipelines/driver_ms 23.48, render/graph/ms 10.16, stream/spawn/ms 2.22, render/present/ms 1.30, stream/tick/ms 0.82 |
| warm-up | 4.77 | 38.1 | main/busy/ms 54.61, render/thread/ms 28.17, render/busy/ms 26.57, stream/drive/ms 24.68, render/graph/ms 5.42, stream/spawn/ms 1.52, render/present/ms 1.27, stream/activate/ms 1.02, physics/insert/ms 1.02, stream/tick/ms 0.95, terrain/update/ms 0.71, physics/step/ms 0.60 |
| warm-up | 4.80 | 31.1 | render/thread/ms 35.70, main/busy/ms 34.98, render/busy/ms 34.03, render/graph/ms 4.18, stream/drive/ms 3.85, render/present/ms 1.26, stream/activate/ms 1.05, physics/insert/ms 1.05, stream/tick/ms 0.89, physics/step/ms 0.83, stream/spawn/ms 0.72, terrain/update/ms 0.56 |
| warm-up | 4.86 | 40.1 | pipelines/wait_ms 5156, pipelines/compile_ms 143, pipelines/front_ms 138, render/thread/ms 41.41, render/busy/ms 39.61, main/busy/ms 32.84, render/graph/ms 7.01, pipelines/driver_ms 5.68, stream/drive/ms 3.91, render/present/ms 1.61, stream/spawn/ms 1.38, physics/insert/ms 1.11 |
| warm-up | 5.53 | 34.3 | pipelines/wait_ms 10.2 k, pipelines/compile_ms 185, pipelines/front_ms 175, render/thread/ms 48.98, render/busy/ms 47.12, main/busy/ms 30.96, pipelines/driver_ms 10.10, render/graph/ms 6.68, stream/drive/ms 5.43, stream/spawn/ms 2.51, stream/tick/ms 1.69, render/present/ms 1.54 |
| warm-up | 5.56 | 30.0 | pipelines/wait_ms 5078, pipelines/compile_ms 63.77, pipelines/front_ms 61.10, render/thread/ms 48.32, render/busy/ms 47.06, main/busy/ms 29.00, stream/drive/ms 5.14, render/graph/ms 4.94, terrain/update/ms 2.76, pipelines/driver_ms 2.66, stream/spawn/ms 2.33, stream/tick/ms 1.71 |
| warm-up | 5.73 | 28.2 | pipelines/wait_ms 3514, main/busy/ms 35.32, render/thread/ms 33.22, render/busy/ms 31.60, pipelines/compile_ms 24.94, pipelines/front_ms 23.13, stream/drive/ms 9.29, render/graph/ms 7.01, stream/spawn/ms 2.16, pipelines/driver_ms 1.80, render/present/ms 1.42, physics/insert/ms 1.01 |
| warm-up | 5.83 | 29.9 | render/thread/ms 36.52, render/busy/ms 35.29, main/busy/ms 26.50, render/graph/ms 4.59, stream/drive/ms 4.30, stream/spawn/ms 2.12, stream/tick/ms 1.34, bigspace/total/ms 1.04, render/present/ms 0.96, terrain/update/ms 0.89, terrain/select/ms 0.76, bigspace/high_precision/ms 0.52 |
| warm-up | 5.94 | 29.5 | pipelines/wait_ms 5681, pipelines/compile_ms 85.00, pipelines/front_ms 81.89, main/busy/ms 39.52, render/thread/ms 31.65, render/busy/ms 29.67, stream/drive/ms 7.77, render/graph/ms 7.64, pipelines/driver_ms 3.11, stream/spawn/ms 2.10, render/present/ms 1.34, stream/tick/ms 0.79 |
| warm-up | 5.97 | 29.1 | pipelines/wait_ms 1139, main/busy/ms 42.83, render/thread/ms 39.61, render/busy/ms 38.16, pipelines/compile_ms 9.01, pipelines/front_ms 8.70, render/graph/ms 5.40, stream/drive/ms 3.56, stream/spawn/ms 2.05, bigspace/total/ms 1.10, render/present/ms 1.10, stream/tick/ms 0.68 |
| warm-up | 6.54 | 28.4 | main/busy/ms 33.22, render/thread/ms 25.69, render/busy/ms 24.14, render/graph/ms 7.69, stream/drive/ms 1.56, render/present/ms 1.36, bigspace/total/ms 0.70, stream/tick/ms 0.62, anim/evaluate_cpu/ms 0.60, terrain/update/ms 0.56, stream/spawn/ms 0.54, terrain/select/ms 0.49 |
| warm-up | 8.20 | 38.2 | pipelines/wait_ms 5245, main/busy/ms 42.27, render/thread/ms 29.51, render/busy/ms 28.26, pipelines/compile_ms 12.73, pipelines/front_ms 11.91, render/graph/ms 7.21, render/present/ms 1.08, stream/drive/ms 1.07, physics/insert/ms 1.03, pipelines/driver_ms 0.81, bigspace/total/ms 0.77 |
| measure | 0.76 | 29.1 | main/busy/ms 32.93, render/thread/ms 26.77, render/busy/ms 25.64, stream/drive/ms 13.41, render/graph/ms 10.62, physics/insert/ms 1.51, render/present/ms 0.92, bigspace/total/ms 0.78, far/select/ms 0.74, controller/feed/ms 0.59, terrain/update/ms 0.57, terrain/select/ms 0.51 |
| measure | 18.18 | 32.8 | main/busy/ms 40.21, render/thread/ms 26.80, render/busy/ms 25.48, stream/drive/ms 21.23, render/graph/ms 14.43, render/present/ms 1.10, bigspace/total/ms 0.87, controller/feed/ms 0.78, terrain/update/ms 0.63, terrain/select/ms 0.57, far/select/ms 0.51, stream/tick/ms 0.39 |
| measure | 19.34 | 32.2 | render/thread/ms 42.27, render/busy/ms 41.19, render/graph/ms 32.63, main/busy/ms 18.33, render/present/ms 0.92, physics/insert/ms 0.87, bigspace/total/ms 0.75, terrain/update/ms 0.65, terrain/select/ms 0.59, controller/feed/ms 0.53, stream/drive/ms 0.38, far/select/ms 0.33 |
| measure | 59.05 | 33.2 | render/thread/ms 46.42, render/busy/ms 45.23, main/busy/ms 27.58, render/graph/ms 12.09, bigspace/total/ms 1.42, controller/feed/ms 1.06, terrain/update/ms 1.02, render/present/ms 0.95, stream/drive/ms 0.90, terrain/select/ms 0.90, stream/tick/ms 0.81, controller/move/ms 0.62 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **68%** (min 46%) · 2580 MHz (min 2580) · memory 8001 MHz · 69.5 W · 74 °C (max 80) · VRAM 6025 MiB · throttled 100% of samples
