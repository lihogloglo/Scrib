# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5699 frames over 65.6 s · **median 11.19 ms (89 fps)** · mean 11.52 · p95 14.61 · p99 16.88 (1% low 59 fps) · worst 34.65 ms · 62 spikes over 1.5× median · 30.2 stutters/min (work 26.5/min)
- warm-up: 351 frames over 8.5 s, worst 2615.7 ms
- hitches (> 28.0 ms): **3** measured, 28 in warm-up
- timed GPU passes: 6.51 ms of a 11.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.19 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- **FAIL** worst frame 34.65 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.060 | 2.070 | 2.264 | 1.00 | 0.184 |
| main_opaque_pass_3d | 1.346 | 1.353 | 1.743 | 1.00 | 0.062 |
| early prepass | 0.491 | 0.494 | 0.711 | 1.00 | 0.094 |
| bloom | 0.318 | 0.319 | 0.326 | 1.00 | 0.066 |
| atmosphere_luts | 0.313 | 0.314 | 0.374 | 1.00 | 0.004 |
| water_velocity | 0.280 | 0.281 | 0.350 | 1.00 | 0.002 |
| uniform_allocation | 0.218 | 0.219 | 0.237 | 1.00 | 0.042 |
| water_sspr | 0.214 | 0.215 | 0.212 | 1.00 | 0.002 |
| ssao | 0.193 | 0.193 | 0.195 | 1.00 | 0.008 |
| bin_unpacking | 0.167 | 0.167 | 0.191 | 1.00 | 0.029 |
| auxiliary_views | 0.163 | 0.164 | 0.631 | 1.00 | 0.164 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.134 | 0.134 | 0.167 | 1.00 | 0.005 |
| tonemapping | 0.090 | 0.090 | 0.099 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.146 | 1.00 | 0.008 |
| clustering | 0.072 | 0.073 | 0.070 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.006 |
| surface_trail | 0.042 | 0.057 | 0.057 | 0.74 | 0.002 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.023 | 1.00 | 0.077 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 26.671 | 0.000 | 13960.76 | 151995.9 |
| render/thread/ms | 10.330 | 13.314 | 27.26 | 58873.4 |
| render/busy/ms | 9.771 | 12.692 | 26.63 | 55685.8 |
| main/busy/ms | 8.858 | 11.214 | 31.33 | 50483.0 |
| render/graph/ms | 5.614 | 7.482 | 16.22 | 31995.2 |
| bigspace/total/ms | 0.559 | 1.557 | 3.91 | 3183.9 |
| render/present/ms | 0.467 | 0.587 | 1.73 | 2658.7 |
| pipelines/compile_ms | 0.360 | 0.000 | 146.90 | 2053.2 |
| pipelines/front_ms | 0.348 | 0.000 | 141.15 | 1984.2 |
| terrain/update/ms | 0.307 | 0.406 | 1.95 | 1749.7 |
| controller/feed/ms | 0.282 | 0.390 | 0.86 | 1606.1 |
| terrain/select/ms | 0.273 | 0.361 | 1.01 | 1557.2 |
| stream/drive/ms | 0.256 | 0.379 | 22.85 | 1459.1 |
| bigspace/low_precision/ms | 0.227 | 0.635 | 3.53 | 1296.4 |
| far/select/ms | 0.201 | 0.291 | 0.65 | 1142.8 |
| stream/tick/ms | 0.198 | 0.292 | 0.60 | 1129.7 |
| bigspace/high_precision/ms | 0.172 | 0.628 | 2.65 | 979.0 |
| anim/evaluate_cpu/ms | 0.168 | 0.224 | 0.49 | 956.6 |
| anim/evaluate/ms | 0.138 | 0.256 | 2.08 | 785.4 |
| controller/move/ms | 0.137 | 0.264 | 0.52 | 781.4 |
| render/acquire/ms | 0.093 | 0.124 | 0.56 | 528.8 |
| physics/step/ms | 0.052 | 0.092 | 0.93 | 294.7 |
| physics/insert/ms | 0.039 | 0.395 | 0.81 | 220.8 |
| anim/apply/ms | 0.012 | 0.024 | 0.16 | 69.5 |
| pipelines/driver_ms | 0.012 | 0.000 | 5.75 | 69.1 |
| weather/eval/ms | 0.011 | 0.016 | 0.14 | 64.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.22 | 59.9 |
| script/update/ms | 0.008 | 0.012 | 0.28 | 44.2 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 30.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 29.9 |
| script/ui/ms | 0.005 | 0.007 | 0.17 | 26.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 26.6 |
| controller/write/ms | 0.004 | 0.005 | 0.08 | 20.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 11 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 31 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1281 | 2204 | 583 |
| crowd/c0/meshes/count | count | 353 | 634 | 162 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5542 | 6046 | 5613 |
| crowd/c1/meshes/count | count | 1181 | 1324 | 1179 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4282 | 7677 | 6179 |
| crowd/joints_uploaded/count | count | 4253 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.59 | 31.33 | 8.84 |
| gpu/timed/ms | ms | 6.58 | 9.94 | 6.74 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 198 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 795 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 198 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 795 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3356 | 5153 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3356 | 5153 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 56 | 297 | 42 |
| phases/shadow/bins | count | 3751 | 5360 | 2250 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9462 | 12.5 k | 5747 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 134 | 305 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 216 | 476 | 19 |
| physics/pending | count | 0 | 1295 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 198 | 203 | 203 |
| pipelines/waiting | count | 0 | 36 | 0 |
| preprocess/bind_groups | count | 10 | 763 | 6 |
| preprocess/bind_groups_created | count | 10 | 763 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 231 | 333 | 221 |
| process/cpu_usage | % | 24.88 | 38.94 | 25.96 |
| process/mem_usage | GiB | 5.04 | 5.21 | 5.21 |
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
| started_load_count |  | 4753 | 4844 | 4844 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.42 | 67.60 | 43.41 |
| system/mem_usage | % | 64.57 | 65.07 | 65.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2917 | 2917 |
| textures/images | count | 2582 | 2635 | 2635 |
| views/all/batch_sets | count | 115 | 349 | 102 |
| views/all/bins | count | 6339 | 8202 | 5730 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.8 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 184 | 355 | 98 |
| views/all/pipelines_distinct | count | 83 | 96 | 80 |
| views/all/work_items | count | 17.0 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 69 | 42 |
| views/cascade/bins | count | 2612 | 3171 | 2250 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8212 | 10.1 k | 5747 |
| views/cascade/pipelines | count | 52 | 69 | 42 |
| views/cascade/work_items | count | 8212 | 10.1 k | 5747 |
| views/lamp_face/batch_sets | count | 0 | 240 | 0 |
| views/lamp_face/bins | count | 1060 | 2463 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1177 | 2803 | 0 |
| views/lamp_face/pipelines | count | 81 | 243 | 0 |
| views/lamp_face/work_items | count | 0 | 2599 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8306 | 13.1 k | 7594 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8469 | 13.3 k | 7902 |
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
| anim/bones_written/count | count | 60.09 | 363 | 342.4 k |
| anim/events/count | count | 0.10 | 3 | 550 |
| assets/Image/added | count | 0.11 | 37 | 632 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.73 | 152 | 4162 |
| assets/Mesh/removed | count | 0.86 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 505 |
| assets/StandardMaterial/modified | count | 3.82 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.45 | 1386 | 8264 |
| physics/inserts | count | 0.62 | 181 | 3507 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 5940 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1302 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2615.7 | main/busy/ms 2591, surface/advance_cpu_ms 68.08, surface/cell_upload_ms 0.73, stream/drive/ms 0.33, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.53 | 918.1 | main/busy/ms 2626, render/thread/ms 879, render/busy/ms 721, render/acquire/ms 156, surface/advance_cpu_ms 68.08, render/graph/ms 6.39, render/present/ms 2.23, stream/drive/ms 1.09, surface/cell_upload_ms 0.73, stream/tick/ms 0.57, terrain/update/ms 0.39, bigspace/total/ms 0.31 |
| warm-up | 3.80 | 91.4 | pipelines/compile_ms 209, pipelines/front_ms 196, main/busy/ms 75.20, render/thread/ms 59.11, render/busy/ms 56.93, pipelines/driver_ms 13.55, render/graph/ms 4.02, stream/drive/ms 2.38, stream/tick/ms 2.05, render/present/ms 1.74, terrain/update/ms 0.83, terrain/select/ms 0.67 |
| warm-up | 3.84 | 30.1 | render/thread/ms 34.42, render/busy/ms 32.39, main/busy/ms 19.66, render/graph/ms 5.23, stream/drive/ms 2.73, render/present/ms 1.84, stream/tick/ms 1.34, terrain/update/ms 0.98, stream/spawn/ms 0.87, terrain/select/ms 0.79, bigspace/total/ms 0.23, render/acquire/ms 0.19 |
| warm-up | 3.99 | 91.5 | render/thread/ms 66.41, render/busy/ms 65.00, main/busy/ms 63.72, stream/drive/ms 45.94, render/graph/ms 4.50, stream/tick/ms 1.93, render/present/ms 1.12, stream/spawn/ms 1.07, terrain/update/ms 0.68, terrain/select/ms 0.55, stream/upload/ms 0.35, bigspace/total/ms 0.34 |
| warm-up | 4.07 | 59.1 | pipelines/wait_ms 1578, pipelines/compile_ms 329, pipelines/front_ms 314, render/thread/ms 44.11, render/busy/ms 42.25, main/busy/ms 26.02, pipelines/driver_ms 15.84, stream/drive/ms 9.39, render/graph/ms 3.85, render/present/ms 1.58, stream/spawn/ms 1.17, stream/tick/ms 1.01 |
| warm-up | 4.10 | 31.4 | pipelines/wait_ms 2410, pipelines/compile_ms 120, pipelines/front_ms 112, render/thread/ms 53.76, render/busy/ms 52.05, main/busy/ms 34.83, stream/drive/ms 17.15, pipelines/driver_ms 8.38, render/graph/ms 4.26, stream/spawn/ms 2.20, render/present/ms 1.46, stream/tick/ms 0.89 |
| warm-up | 4.15 | 53.8 | pipelines/wait_ms 2095, render/thread/ms 58.26, render/busy/ms 56.94, pipelines/compile_ms 49.09, pipelines/front_ms 47.29, main/busy/ms 40.24, stream/drive/ms 17.60, render/graph/ms 3.46, stream/spawn/ms 2.38, pipelines/driver_ms 1.80, render/present/ms 1.11, stream/tick/ms 1.02 |
| warm-up | 4.19 | 36.0 | pipelines/wait_ms 1593, pipelines/compile_ms 90.51, pipelines/front_ms 86.26, render/thread/ms 67.11, render/busy/ms 65.77, main/busy/ms 30.32, stream/drive/ms 5.20, pipelines/driver_ms 4.25, render/graph/ms 3.78, stream/spawn/ms 2.29, stream/tick/ms 1.29, render/present/ms 1.16 |
| warm-up | 4.22 | 33.1 | pipelines/wait_ms 1769, pipelines/compile_ms 96.07, pipelines/front_ms 91.72, main/busy/ms 55.87, render/thread/ms 52.72, render/busy/ms 50.99, stream/drive/ms 31.09, render/graph/ms 6.29, pipelines/driver_ms 4.35, stream/spawn/ms 1.69, render/present/ms 1.51, stream/tick/ms 1.19 |
| warm-up | 4.27 | 52.8 | pipelines/wait_ms 1244, main/busy/ms 54.77, render/thread/ms 49.33, render/busy/ms 47.48, stream/drive/ms 30.61, pipelines/compile_ms 12.39, pipelines/front_ms 11.56, render/graph/ms 7.37, render/present/ms 1.53, terrain/update/ms 1.31, stream/activate/ms 1.08, terrain/select/ms 1.08 |
| warm-up | 4.31 | 35.7 | render/thread/ms 58.07, render/busy/ms 56.39, main/busy/ms 20.58, render/graph/ms 11.36, stream/drive/ms 3.22, render/present/ms 1.39, terrain/update/ms 1.36, terrain/select/ms 1.15, stream/activate/ms 1.04, stream/tick/ms 0.79, bigspace/total/ms 0.56, bigspace/high_precision/ms 0.34 |
| warm-up | 4.34 | 30.1 | pipelines/wait_ms 2631, pipelines/compile_ms 134, pipelines/front_ms 130, render/thread/ms 54.60, render/busy/ms 53.08, main/busy/ms 18.28, render/graph/ms 13.01, pipelines/driver_ms 4.55, stream/drive/ms 2.91, render/present/ms 1.34, stream/tick/ms 0.95, terrain/update/ms 0.88 |
| warm-up | 4.41 | 32.4 | main/busy/ms 66.59, stream/drive/ms 46.31, render/thread/ms 36.61, render/busy/ms 34.67, render/graph/ms 5.64, stream/spawn/ms 2.45, stream/tick/ms 2.26, render/present/ms 1.60, terrain/update/ms 0.89, bigspace/total/ms 0.83, terrain/select/ms 0.70, bigspace/low_precision/ms 0.35 |
| warm-up | 4.50 | 93.6 | pipelines/wait_ms 8237, pipelines/compile_ms 153, pipelines/front_ms 144, main/busy/ms 80.96, render/thread/ms 55.89, stream/drive/ms 55.51, render/busy/ms 54.26, pipelines/driver_ms 8.76, render/graph/ms 4.55, stream/spawn/ms 1.86, stream/tick/ms 1.51, render/present/ms 1.32 |
| warm-up | 4.55 | 50.8 | pipelines/wait_ms 8237, pipelines/compile_ms 153, pipelines/front_ms 144, render/thread/ms 81.64, main/busy/ms 71.12, render/busy/ms 48.76, stream/drive/ms 45.94, render/present/ms 32.66, pipelines/driver_ms 8.76, render/graph/ms 4.13, stream/spawn/ms 2.08, stream/tick/ms 0.93 |
| warm-up | 4.63 | 33.8 | pipelines/wait_ms 6930, pipelines/compile_ms 143, pipelines/front_ms 129, main/busy/ms 53.65, stream/drive/ms 37.11, render/thread/ms 32.59, render/busy/ms 31.03, pipelines/driver_ms 14.46, render/graph/ms 5.01, stream/spawn/ms 2.03, render/present/ms 1.20, stream/activate/ms 1.07 |
| warm-up | 5.20 | 28.7 | main/busy/ms 30.67, render/thread/ms 19.16, render/busy/ms 17.90, stream/drive/ms 14.34, render/graph/ms 4.06, render/present/ms 0.95, stream/tick/ms 0.68, terrain/update/ms 0.67, stream/spawn/ms 0.60, terrain/select/ms 0.54, stream/activate/ms 0.36, render/acquire/ms 0.31 |
| warm-up | 5.28 | 28.4 | pipelines/wait_ms 14.7 k, pipelines/compile_ms 48.42, pipelines/front_ms 43.76, main/busy/ms 35.30, render/thread/ms 31.90, render/busy/ms 30.22, render/graph/ms 5.63, pipelines/driver_ms 4.66, stream/drive/ms 2.78, render/present/ms 1.39, stream/spawn/ms 1.12, stream/tick/ms 0.86 |
| warm-up | 5.31 | 33.4 | pipelines/wait_ms 14.8 k, pipelines/compile_ms 56.84, pipelines/front_ms 51.75, render/thread/ms 46.63, render/busy/ms 44.83, main/busy/ms 38.99, render/graph/ms 7.90, pipelines/driver_ms 5.09, stream/drive/ms 4.06, stream/spawn/ms 1.94, render/present/ms 1.44, terrain/update/ms 1.01 |
| warm-up | 5.37 | 30.3 | render/thread/ms 44.15, render/busy/ms 42.88, main/busy/ms 20.04, render/graph/ms 5.49, stream/drive/ms 3.03, stream/spawn/ms 1.24, render/present/ms 1.08, stream/tick/ms 0.79, bigspace/total/ms 0.70, terrain/update/ms 0.68, terrain/select/ms 0.59, bigspace/high_precision/ms 0.38 |
| warm-up | 5.42 | 28.3 | render/thread/ms 44.02, render/busy/ms 42.80, main/busy/ms 23.61, stream/drive/ms 4.50, render/graph/ms 4.35, stream/spawn/ms 1.85, render/present/ms 0.85, terrain/update/ms 0.67, stream/tick/ms 0.65, bigspace/total/ms 0.63, terrain/select/ms 0.57, render/acquire/ms 0.37 |
| warm-up | 5.44 | 28.2 | render/thread/ms 53.56, render/busy/ms 51.91, main/busy/ms 28.12, render/graph/ms 6.82, stream/drive/ms 4.61, stream/spawn/ms 1.97, render/present/ms 1.19, terrain/update/ms 0.77, bigspace/total/ms 0.71, terrain/select/ms 0.66, stream/tick/ms 0.65, render/acquire/ms 0.46 |
| warm-up | 5.51 | 36.2 | pipelines/wait_ms 7397, pipelines/compile_ms 171, pipelines/front_ms 165, render/thread/ms 46.15, render/busy/ms 44.94, main/busy/ms 23.27, pipelines/driver_ms 6.22, render/graph/ms 5.61, stream/drive/ms 4.77, stream/spawn/ms 2.06, stream/tick/ms 1.37, render/present/ms 1.04 |
| warm-up | 5.77 | 29.6 | pipelines/wait_ms 5073, pipelines/compile_ms 169, pipelines/front_ms 159, render/thread/ms 35.70, render/busy/ms 34.43, main/busy/ms 29.42, pipelines/driver_ms 10.48, render/graph/ms 4.65, stream/drive/ms 4.44, stream/spawn/ms 2.07, stream/tick/ms 1.37, render/present/ms 1.05 |
| warm-up | 5.87 | 36.1 | main/busy/ms 49.59, render/thread/ms 27.50, render/busy/ms 26.43, stream/drive/ms 19.57, render/graph/ms 4.80, stream/spawn/ms 2.08, physics/insert/ms 1.51, render/present/ms 0.87, stream/tick/ms 0.84, terrain/update/ms 0.56, bigspace/total/ms 0.51, terrain/select/ms 0.49 |
| warm-up | 6.62 | 31.5 | pipelines/wait_ms 4255, main/busy/ms 43.76, render/thread/ms 36.65, render/busy/ms 35.02, pipelines/compile_ms 32.85, pipelines/front_ms 31.70, render/graph/ms 14.01, physics/insert/ms 1.63, stream/drive/ms 1.46, render/present/ms 1.38, pipelines/driver_ms 1.15, bigspace/total/ms 0.82 |
| warm-up | 6.65 | 35.7 | pipelines/wait_ms 4255, main/busy/ms 48.79, render/thread/ms 44.27, render/busy/ms 42.96, pipelines/compile_ms 32.85, pipelines/front_ms 31.70, render/graph/ms 9.29, physics/insert/ms 1.72, stream/drive/ms 1.44, pipelines/driver_ms 1.15, render/present/ms 1.03, anim/evaluate_cpu/ms 0.86 |
| measure | 18.19 | 34.7 | main/busy/ms 42.06, render/thread/ms 24.77, render/busy/ms 23.59, stream/drive/ms 23.15, render/graph/ms 12.71, render/present/ms 0.97, bigspace/total/ms 0.83, controller/feed/ms 0.62, terrain/update/ms 0.57, stream/tick/ms 0.54, terrain/select/ms 0.51, far/select/ms 0.42 |
| measure | 19.26 | 28.6 | render/thread/ms 38.65, render/busy/ms 37.48, render/graph/ms 22.68, main/busy/ms 18.30, render/present/ms 1.00, physics/insert/ms 0.87, bigspace/total/ms 0.78, terrain/update/ms 0.57, controller/feed/ms 0.56, terrain/select/ms 0.52, stream/drive/ms 0.43, far/select/ms 0.38 |
| measure | 59.02 | 28.1 | render/thread/ms 39.47, render/busy/ms 38.29, main/busy/ms 25.14, render/graph/ms 12.13, bigspace/total/ms 1.42, controller/feed/ms 1.01, terrain/update/ms 0.95, anim/evaluate/ms 0.94, render/present/ms 0.89, terrain/select/ms 0.84, stream/drive/ms 0.66, anim/evaluate_cpu/ms 0.63 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **74%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.9 W · 74 °C (max 81) · VRAM 5981 MiB · throttled 100% of samples
