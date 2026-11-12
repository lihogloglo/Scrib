# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,sky · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 12592 frames over 65.6 s · **median 5.21 ms (192 fps)** · mean 5.21 · p95 6.05 · p99 6.49 (1% low 154 fps) · worst 14.34 ms · 13 spikes over 1.5× median · 13.7 stutters/min (work 9.1/min)
- warm-up: 334 frames over 3.1 s, worst 579.4 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 4.54 ms of a 5.21 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 5.21 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- pass worst frame 14.34 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.013 | 2.642 | 0.020 |
| ssao | 0.747 | 1.080 | 0.006 |
| early prepass | 0.460 | 0.731 | 0.034 |
| bloom | 0.341 | 0.514 | 0.058 |
| upscaling | 0.179 | 0.219 | 0.003 |
| water_fft | 0.133 | 0.131 | 0.002 |
| water_snapshot | 0.132 | 0.165 | 0.002 |
| tonemapping | 0.107 | 0.123 | 0.003 |
| water_velocity | 0.100 | 0.112 | 0.002 |
| main_transparent_pass_3d | 0.100 | 0.323 | 0.012 |
| clustering | 0.064 | 0.072 | 0.020 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| uniform_allocation | 0.032 | 0.043 | 0.008 |
| water_mips | 0.024 | 0.024 | 0.002 |
| bin_unpacking | 0.023 | 0.025 | 0.005 |
| early_mesh_preprocessing | 0.014 | 0.015 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| auxiliary_views | 0.001 | 0.002 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 4.406 | 5.328 | 13.21 | 55478.8 |
| main/busy/ms | 4.189 | 4.704 | 9.56 | 52750.5 |
| render/busy/ms | 3.502 | 4.225 | 12.74 | 44095.2 |
| render/graph/ms | 1.906 | 2.547 | 10.98 | 24001.1 |
| render/acquire/ms | 0.477 | 1.187 | 2.92 | 6006.7 |
| render/present/ms | 0.427 | 0.553 | 2.06 | 5376.9 |
| terrain/update/ms | 0.293 | 0.367 | 1.44 | 3694.3 |
| terrain/select/ms | 0.262 | 0.328 | 0.76 | 3298.5 |
| stream/drive/ms | 0.224 | 0.315 | 2.10 | 2817.9 |
| bigspace/total/ms | 0.213 | 0.424 | 1.43 | 2676.4 |
| stream/tick/ms | 0.191 | 0.269 | 0.65 | 2405.2 |
| far/select/ms | 0.173 | 0.236 | 0.58 | 2174.6 |
| bigspace/high_precision/ms | 0.106 | 0.246 | 0.84 | 1338.0 |
| bigspace/low_precision/ms | 0.043 | 0.084 | 0.68 | 536.3 |
| physics/insert/ms | 0.014 | 0.000 | 1.06 | 172.2 |
| weather/eval/ms | 0.010 | 0.015 | 0.35 | 129.7 |
| physics/step/ms | 0.008 | 0.028 | 0.66 | 96.8 |
| script/update/ms | 0.007 | 0.009 | 0.33 | 82.3 |
| script/input/ms | 0.004 | 0.005 | 0.16 | 53.8 |
| script/ui/ms | 0.004 | 0.006 | 0.26 | 46.7 |
| controller/move/ms | 0.002 | 0.003 | 0.16 | 27.8 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.24 | 25.4 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.61 | 11.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 11.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 8.3 |
| anim/apply/ms | 0.001 | 0.001 | 0.11 | 6.6 |
| stream/activate/ms | 0.000 | 0.000 | 0.39 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 4.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.23 | 3.8 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 3.1 |
| stream/despawn/ms | 0.000 | 0.000 | 0.21 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.02 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 8042 | 8395 | 7927 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 4.69 | 12.74 | 5.16 |
| gpu/timed/ms | ms | 4.65 | 6.83 | 5.16 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 327 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 327 | 617 | 603 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 0 | 0 |
| phases/shadow/bins | count | 0 | 0 | 0 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 0 | 0 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 0 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2727 | 3248 | 2329 |
| physics/items | count | 255 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 93 | 93 | 93 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 35 | 2 |
| preprocess/bind_groups_created | count | 2 | 35 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 38 | 41 | 39 |
| process/cpu_usage | % | 25.96 | 43.54 | 23 |
| process/mem_usage | GiB | 1.93 | 1.98 | 1.94 |
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
| started_load_count |  | 2998 | 3032 | 3032 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.01 | 0.03 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.09 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.82 | 52.79 | 28.24 |
| system/mem_usage | % | 51.44 | 51.75 | 51.36 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 12 | 12 | 12 |
| views/all/bins | count | 542 | 1586 | 1560 |
| views/all/count | count | 1 | 3 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 906 | 1960 | 1928 |
| views/all/pipelines | count | 8 | 8 | 8 |
| views/all/pipelines_distinct | count | 8 | 8 | 8 |
| views/all/work_items | count | 1008 | 2172 | 2129 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 2 | 0 |
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
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 542 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 906 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1008 | 2172 | 2129 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 3200 | 3212 | 3203 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 125 |
| assets/Image/modified | count | 0 | 0 | 0 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.17 | 195 | 2087 |
| assets/Mesh/removed | count | 0.35 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.19 | 106 | 2356 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.19 | 145 | 2335 |
| physics/steps | count | 0.31 | 1 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.03 | 27 | 351 |
| stream/upload/bytes | bytes | 2688 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.00 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 589 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.4 | main/busy/ms 558, surface/advance_cpu_ms 64.58, surface/cell_upload_ms 0.69, stream/drive/ms 0.33, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.97 | 392.8 | main/busy/ms 591, render/thread/ms 358, render/busy/ms 330, surface/advance_cpu_ms 64.58, render/acquire/ms 19.84, render/present/ms 7.90, render/graph/ms 3.29, stream/drive/ms 1.03, surface/cell_upload_ms 0.69, stream/tick/ms 0.47, terrain/update/ms 0.42, bigspace/total/ms 0.28 |
| warm-up | 1.00 | 25.1 | render/thread/ms 382, render/busy/ms 353, pipelines/wait_ms 162, main/busy/ms 51.87, render/acquire/ms 20.08, terrain/update/ms 19.26, pipelines/compile_ms 15.95, render/graph/ms 15.63, pipelines/front_ms 12.85, render/present/ms 9.12, pipelines/driver_ms 3.10, stream/drive/ms 1.26 |
| warm-up | 1.23 | 99.4 | pipelines/compile_ms 184, pipelines/front_ms 170, render/thread/ms 59.77, render/busy/ms 58.12, main/busy/ms 56.51, pipelines/driver_ms 13.96, render/graph/ms 2.56, stream/drive/ms 1.92, stream/tick/ms 1.62, render/present/ms 1.42, terrain/update/ms 0.68, terrain/select/ms 0.55 |
| warm-up | 1.27 | 28.4 | pipelines/wait_ms 170, pipelines/compile_ms 97.48, pipelines/front_ms 88.25, render/thread/ms 33.00, render/busy/ms 30.97, main/busy/ms 16.79, pipelines/driver_ms 9.23, stream/drive/ms 3.34, render/graph/ms 2.72, render/present/ms 1.86, stream/tick/ms 1.40, stream/spawn/ms 1.05 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_sky\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,sky
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **95%** (min 61%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.0 W · 78 °C (max 80) · VRAM 3482 MiB · throttled 100% of samples
