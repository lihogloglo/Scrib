# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,sky · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 12636 frames over 65.6 s · **median 5.18 ms (193 fps)** · mean 5.19 · p95 6.08 · p99 6.59 (1% low 152 fps) · worst 12.11 ms · 10 spikes over 1.5× median · 9.1 stutters/min (work 3.7/min)
- warm-up: 334 frames over 3.1 s, worst 577.5 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 4.50 ms of a 5.19 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 5.18 ms, limit 11 ms
- **FAIL** frame_ms: 10 stutters (> 1.5x moving median of 60) in 1.09 min = 9.14/min, limit 1/min
- pass worst frame 12.11 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 1.996 | 2.620 | 0.023 |
| ssao | 0.747 | 1.072 | 0.007 |
| early prepass | 0.451 | 0.730 | 0.034 |
| bloom | 0.340 | 0.484 | 0.057 |
| upscaling | 0.174 | 0.211 | 0.004 |
| water_snapshot | 0.136 | 0.163 | 0.002 |
| water_fft | 0.130 | 0.127 | 0.003 |
| tonemapping | 0.105 | 0.124 | 0.003 |
| water_velocity | 0.098 | 0.105 | 0.002 |
| main_transparent_pass_3d | 0.098 | 0.329 | 0.012 |
| clustering | 0.064 | 0.072 | 0.020 |
| surface_trail | 0.058 | 0.057 | 0.003 |
| uniform_allocation | 0.032 | 0.043 | 0.008 |
| water_mips | 0.023 | 0.023 | 0.002 |
| bin_unpacking | 0.023 | 0.025 | 0.006 |
| early_mesh_preprocessing | 0.015 | 0.015 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.004 |
| auxiliary_views | 0.001 | 0.002 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 4.418 | 5.336 | 11.47 | 55824.4 |
| main/busy/ms | 4.190 | 4.722 | 9.72 | 52939.9 |
| render/busy/ms | 3.576 | 4.396 | 9.75 | 45183.7 |
| render/graph/ms | 1.949 | 2.675 | 8.29 | 24632.9 |
| render/present/ms | 0.432 | 0.603 | 1.85 | 5462.7 |
| render/acquire/ms | 0.410 | 1.208 | 3.14 | 5178.0 |
| terrain/update/ms | 0.290 | 0.365 | 1.76 | 3662.6 |
| terrain/select/ms | 0.258 | 0.327 | 0.74 | 3266.1 |
| stream/drive/ms | 0.229 | 0.315 | 2.18 | 2888.5 |
| bigspace/total/ms | 0.214 | 0.427 | 1.33 | 2706.6 |
| stream/tick/ms | 0.195 | 0.270 | 0.68 | 2458.1 |
| far/select/ms | 0.169 | 0.234 | 0.51 | 2132.3 |
| bigspace/high_precision/ms | 0.107 | 0.240 | 0.95 | 1357.9 |
| bigspace/low_precision/ms | 0.039 | 0.064 | 1.17 | 495.5 |
| physics/insert/ms | 0.014 | 0.000 | 0.95 | 177.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.25 | 134.4 |
| physics/step/ms | 0.008 | 0.030 | 0.60 | 104.0 |
| script/update/ms | 0.007 | 0.010 | 0.18 | 91.3 |
| script/input/ms | 0.004 | 0.006 | 0.12 | 54.3 |
| script/ui/ms | 0.004 | 0.005 | 0.21 | 48.0 |
| controller/move/ms | 0.002 | 0.003 | 0.05 | 27.2 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.17 | 25.4 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.59 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 11.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 8.0 |
| anim/apply/ms | 0.001 | 0.001 | 0.02 | 7.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.51 | 5.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 4.1 |
| stream/spawn/ms | 0.000 | 0.000 | 0.19 | 3.8 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 3.2 |
| stream/despawn/ms | 0.000 | 0.000 | 0.18 | 2.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.03 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
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
| frame/work/ms | ms | 4.65 | 9.75 | 5.24 |
| gpu/timed/ms | ms | 4.62 | 7.57 | 5.24 |
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
| phases/opaque3d/items | count | 328 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 328 | 617 | 603 |
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
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 93 | 93 | 93 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 35 | 2 |
| preprocess/bind_groups_created | count | 2 | 35 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 38 | 41 | 39 |
| process/cpu_usage | % | 25.97 | 45.67 | 29.61 |
| process/mem_usage | GiB | 1.90 | 1.99 | 1.99 |
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
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.01 | 0.03 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.10 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.07 | 46.10 | 29.13 |
| system/mem_usage | % | 51.17 | 51.43 | 51.42 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 12 | 12 | 12 |
| views/all/bins | count | 544 | 1586 | 1560 |
| views/all/count | count | 1 | 3 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 908 | 1960 | 1928 |
| views/all/pipelines | count | 8 | 8 | 8 |
| views/all/pipelines_distinct | count | 8 | 8 | 8 |
| views/all/work_items | count | 1010 | 2172 | 2129 |
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
| views/main/bins | count | 544 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 908 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1010 | 2172 | 2129 |
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
| assets/Mesh/added | count | 0.17 | 213 | 2087 |
| assets/Mesh/removed | count | 0.35 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.19 | 101 | 2369 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.18 | 161 | 2335 |
| physics/steps | count | 0.31 | 1 | 3938 |
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
| stream/upload/bytes | bytes | 2679 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.00 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 15 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 587 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.5 | main/busy/ms 557, surface/advance_cpu_ms 65.39, surface/cell_upload_ms 0.70, stream/drive/ms 0.43, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.95 | 369.6 | main/busy/ms 590, render/thread/ms 331, render/busy/ms 301, surface/advance_cpu_ms 65.39, render/acquire/ms 17.42, render/present/ms 12.57, render/graph/ms 4.10, stream/drive/ms 1.15, surface/cell_upload_ms 0.70, terrain/update/ms 0.54, stream/tick/ms 0.50, terrain/select/ms 0.38 |
| warm-up | 0.99 | 32.4 | pipelines/wait_ms 152, render/thread/ms 38.11, render/busy/ms 35.07, main/busy/ms 31.54, terrain/update/ms 23.70, pipelines/compile_ms 22.59, render/graph/ms 21.70, pipelines/front_ms 13.66, pipelines/driver_ms 8.93, stream/drive/ms 2.83, render/present/ms 2.61, stream/tick/ms 2.42 |
| warm-up | 1.22 | 86.4 | pipelines/compile_ms 206, pipelines/front_ms 194, main/busy/ms 65.86, render/thread/ms 54.55, render/busy/ms 53.14, pipelines/driver_ms 12.10, stream/drive/ms 3.17, stream/tick/ms 2.56, render/graph/ms 2.27, render/present/ms 1.19, terrain/update/ms 1.09, terrain/select/ms 0.91 |
| warm-up | 1.25 | 26.1 | pipelines/wait_ms 194, pipelines/compile_ms 99.39, pipelines/front_ms 92.97, render/thread/ms 30.46, render/busy/ms 28.99, main/busy/ms 18.54, pipelines/driver_ms 6.42, stream/drive/ms 2.87, render/graph/ms 2.28, stream/tick/ms 1.60, render/present/ms 1.26, terrain/update/ms 0.86 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_sky\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,sky
- before the run: GPU 1%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **95%** (min 64%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.2 W · 78 °C (max 81) · VRAM 3273 MiB · throttled 100% of samples
