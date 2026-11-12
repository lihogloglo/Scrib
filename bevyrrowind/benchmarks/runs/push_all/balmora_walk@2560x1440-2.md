# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5693 frames over 65.6 s · **median 10.93 ms (92 fps)** · mean 11.53 · p95 14.16 · p99 21.98 (1% low 45 fps) · worst 83.70 ms · 124 spikes over 1.5× median · 96.9 stutters/min (work 105.1/min)
- warm-up: 354 frames over 5.8 s, worst 583.8 ms
- hitches (> 27.3 ms): **36** measured, 25 in warm-up
- timed GPU passes: 8.04 ms of a 11.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.93 ms, limit 11 ms
- **FAIL** frame_ms: 106 stutters (> 1.5x moving median of 60) in 1.09 min = 96.89/min, limit 1/min
- **FAIL** worst frame 83.70 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.134 | 2.584 | 0.180 |
| main_opaque_pass_3d | 1.433 | 2.077 | 0.045 |
| atmosphere_luts | 0.863 | 1.207 | 0.003 |
| early prepass | 0.535 | 1.051 | 0.078 |
| lightprobe_irradiance_map | 0.450 | 0.767 | 0.001 |
| bloom | 0.328 | 0.330 | 0.066 |
| auxiliary_views | 0.268 | 1.094 | 0.154 |
| lightprobe_radiance_map | 0.228 | 0.233 | 0.004 |
| ssao | 0.209 | 0.210 | 0.007 |
| uniform_allocation | 0.207 | 0.214 | 0.031 |
| bin_unpacking | 0.131 | 0.143 | 0.023 |
| water_fft | 0.127 | 0.128 | 0.003 |
| clouds_trace | 0.127 | 0.216 | 0.001 |
| upscaling | 0.123 | 0.138 | 0.004 |
| atmos_froxel_inject | 0.096 | 0.101 | 0.001 |
| tonemapping | 0.089 | 0.090 | 0.004 |
| water_velocity | 0.084 | 0.085 | 0.002 |
| clustering | 0.067 | 0.070 | 0.019 |
| main_transparent_pass_3d | 0.065 | 0.109 | 0.146 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| surface_trail | 0.063 | 0.057 | 0.002 |
| early_mesh_preprocessing | 0.062 | 0.059 | 0.005 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.034 | 0.036 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.025 | 0.007 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.220 | 12.650 | 82.31 | 58183.9 |
| render/busy/ms | 9.611 | 12.081 | 81.65 | 54714.8 |
| main/busy/ms | 7.953 | 9.470 | 21.75 | 45279.0 |
| render/graph/ms | 4.596 | 5.887 | 15.39 | 26166.5 |
| pipelines/wait_ms | 1.549 | 0.000 | 4468.27 | 8820.7 |
| render/present/ms | 0.505 | 1.166 | 5.15 | 2876.5 |
| bigspace/total/ms | 0.468 | 0.821 | 2.99 | 2665.6 |
| terrain/update/ms | 0.295 | 0.374 | 8.62 | 1678.0 |
| controller/feed/ms | 0.269 | 0.359 | 0.93 | 1533.2 |
| terrain/select/ms | 0.261 | 0.332 | 0.93 | 1483.5 |
| stream/drive/ms | 0.227 | 0.329 | 7.21 | 1294.6 |
| far/select/ms | 0.197 | 0.283 | 0.83 | 1119.4 |
| stream/tick/ms | 0.183 | 0.246 | 1.11 | 1044.0 |
| bigspace/low_precision/ms | 0.178 | 0.310 | 2.00 | 1012.1 |
| anim/evaluate_cpu/ms | 0.168 | 0.212 | 0.54 | 956.8 |
| anim/evaluate/ms | 0.151 | 0.284 | 1.78 | 857.2 |
| controller/move/ms | 0.130 | 0.237 | 1.16 | 741.1 |
| bigspace/high_precision/ms | 0.117 | 0.240 | 1.74 | 665.9 |
| render/acquire/ms | 0.104 | 0.139 | 1.61 | 592.7 |
| pipelines/compile_ms | 0.069 | 0.000 | 139.96 | 391.1 |
| pipelines/front_ms | 0.067 | 0.000 | 134.41 | 378.7 |
| physics/step/ms | 0.048 | 0.091 | 0.55 | 274.6 |
| physics/insert/ms | 0.034 | 0.368 | 0.85 | 191.1 |
| weather/map/ms | 0.027 | 0.035 | 0.33 | 153.8 |
| anim/apply/ms | 0.012 | 0.020 | 0.17 | 65.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.23 | 63.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 62.8 |
| script/update/ms | 0.007 | 0.011 | 0.18 | 41.0 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 28.6 |
| script/input/ms | 0.005 | 0.007 | 0.16 | 28.5 |
| stream/despawn/ms | 0.005 | 0.000 | 0.87 | 27.0 |
| stream/spawn/ms | 0.004 | 0.000 | 1.05 | 23.1 |
| controller/write/ms | 0.003 | 0.005 | 0.09 | 19.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 13.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 5.55 | 12.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 9.07 | 81.65 | 9.04 |
| gpu/timed/ms | ms | 8.03 | 11.29 | 8.38 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 256 | 486 | 478 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3935 | 4350 | 4314 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 256 | 486 | 478 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3935 | 4350 | 4314 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 4107 | 4334 | 4333 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 4107 | 4334 | 4333 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 39 | 340 | 31 |
| phases/shadow/bins | count | 67.7 k | 174.1 k | 12.0 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 220.2 k | 559.8 k | 39.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 148 | 358 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 174 | 174 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 10 | 823 | 6 |
| preprocess/bind_groups_created | count | 10 | 823 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 171 | 306 | 164 |
| process/cpu_usage | % | 24.28 | 39.32 | 24.52 |
| process/mem_usage | GiB | 2.88 | 2.98 | 2.97 |
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
| started_load_count |  | 3481 | 3541 | 3541 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.84 | 62.73 | 42.03 |
| system/mem_usage | % | 50.10 | 52.07 | 51.93 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 85 | 380 | 75 |
| views/all/bins | count | 76.0 k | 182.4 k | 21.6 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 246.2 k | 588.1 k | 69.0 k |
| views/all/pipelines | count | 190 | 394 | 71 |
| views/all/pipelines_distinct | count | 62 | 70 | 55 |
| views/all/work_items | count | 69.2 k | 575.5 k | 69.7 k |
| views/cascade/batch_sets | count | 37 | 50 | 31 |
| views/cascade/bins | count | 11.9 k | 12.5 k | 12.0 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.4 k | 40.0 k | 39.4 k |
| views/cascade/pipelines | count | 37 | 50 | 31 |
| views/cascade/work_items | count | 38.4 k | 40.0 k | 39.4 k |
| views/lamp_face/batch_sets | count | 0 | 299 | 0 |
| views/lamp_face/bins | count | 56.9 k | 162.2 k | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 184.6 k | 521.7 k | 0 |
| views/lamp_face/pipelines | count | 110 | 311 | 0 |
| views/lamp_face/work_items | count | 0 | 510.1 k | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 8718 | 9622 | 9622 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.8 k | 29.6 k |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 28.5 k | 30.5 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.63 | 208 | 271.2 k |
| anim/events/count | count | 0.27 | 4 | 1564 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.72 | 160 | 4086 |
| assets/Mesh/removed | count | 0.86 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 21.2 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1497 | 4739 |
| physics/inserts | count | 0.42 | 107 | 2372 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.68 | 3 | 3897 |
| physics/steps_dropped | count | 0.01 | 2 | 42 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 23 | 378 |
| stream/upload/bytes | bytes | 5946 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1304 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.3 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 583.8 | main/busy/ms 560, surface/advance_cpu_ms 65.21, surface/cell_upload_ms 0.70, stream/drive/ms 0.34, weather/map/ms 0.11, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.92 | 336.9 | main/busy/ms 591, render/thread/ms 306, render/busy/ms 221, render/acquire/ms 82.89, surface/advance_cpu_ms 65.21, render/graph/ms 5.28, render/present/ms 2.04, stream/drive/ms 0.98, surface/cell_upload_ms 0.70, terrain/update/ms 0.61, stream/tick/ms 0.48, terrain/select/ms 0.45 |
| warm-up | 0.99 | 72.5 | pipelines/wait_ms 1212, render/thread/ms 318, render/busy/ms 232, render/acquire/ms 83.15, pipelines/compile_ms 73.52, pipelines/front_ms 66.07, main/busy/ms 47.26, terrain/update/ms 17.43, render/graph/ms 9.19, pipelines/driver_ms 7.44, render/present/ms 3.17, stream/drive/ms 1.19 |
| warm-up | 1.24 | 105.5 | pipelines/compile_ms 199, pipelines/front_ms 182, pipelines/wait_ms 181, main/busy/ms 69.09, render/thread/ms 62.78, render/busy/ms 59.62, pipelines/driver_ms 17.35, render/graph/ms 4.77, render/present/ms 1.95, stream/drive/ms 1.67, stream/tick/ms 1.39, render/acquire/ms 1.21 |
| warm-up | 1.76 | 31.0 | pipelines/wait_ms 6744, pipelines/compile_ms 46.23, pipelines/front_ms 42.07, render/thread/ms 40.52, render/busy/ms 39.31, main/busy/ms 15.55, render/graph/ms 4.19, pipelines/driver_ms 4.16, stream/drive/ms 2.62, stream/spawn/ms 1.24, render/present/ms 0.94, stream/tick/ms 0.81 |
| warm-up | 1.84 | 32.7 | render/thread/ms 51.47, render/busy/ms 50.16, main/busy/ms 13.65, render/graph/ms 5.21, stream/drive/ms 2.27, render/present/ms 1.05, stream/tick/ms 0.84, terrain/update/ms 0.60, terrain/select/ms 0.50, bigspace/total/ms 0.43, render/acquire/ms 0.25, bigspace/high_precision/ms 0.25 |
| warm-up | 1.99 | 29.0 | render/thread/ms 27.53, main/busy/ms 26.79, render/busy/ms 26.14, stream/drive/ms 4.99, render/graph/ms 4.17, terrain/update/ms 3.47, render/present/ms 1.22, physics/insert/ms 1.08, stream/tick/ms 0.80, terrain/select/ms 0.56, bigspace/total/ms 0.52, physics/step/ms 0.44 |
| warm-up | 2.23 | 31.4 | pipelines/wait_ms 4893, pipelines/compile_ms 157, pipelines/front_ms 150, render/thread/ms 40.79, render/busy/ms 39.49, main/busy/ms 14.86, pipelines/driver_ms 6.94, render/graph/ms 6.44, stream/drive/ms 2.70, render/present/ms 1.07, stream/spawn/ms 1.02, stream/tick/ms 0.70 |
| warm-up | 2.26 | 27.5 | pipelines/wait_ms 7300, pipelines/compile_ms 66.92, pipelines/front_ms 61.96, render/thread/ms 56.66, render/busy/ms 55.47, main/busy/ms 14.66, render/graph/ms 6.13, pipelines/driver_ms 4.96, stream/drive/ms 2.89, stream/spawn/ms 1.10, render/present/ms 0.95, stream/tick/ms 0.69 |
| warm-up | 2.30 | 35.5 | pipelines/wait_ms 4001, render/thread/ms 60.38, render/busy/ms 59.26, pipelines/compile_ms 33.63, pipelines/front_ms 31.03, main/busy/ms 20.61, render/graph/ms 5.91, stream/drive/ms 3.33, pipelines/driver_ms 2.61, stream/spawn/ms 2.07, render/present/ms 0.87, terrain/update/ms 0.69 |
| warm-up | 2.33 | 35.2 | render/thread/ms 67.44, render/busy/ms 66.28, main/busy/ms 34.47, render/graph/ms 4.75, stream/drive/ms 3.40, stream/spawn/ms 2.21, render/present/ms 0.89, stream/tick/ms 0.67, terrain/update/ms 0.65, physics/insert/ms 0.57, terrain/select/ms 0.55, bigspace/total/ms 0.45 |
| warm-up | 2.38 | 46.2 | pipelines/wait_ms 977, pipelines/compile_ms 111, pipelines/front_ms 108, render/thread/ms 78.03, render/busy/ms 76.90, main/busy/ms 30.75, render/graph/ms 4.92, pipelines/driver_ms 3.03, stream/drive/ms 2.89, stream/spawn/ms 1.47, render/present/ms 0.88, stream/tick/ms 0.79 |
| warm-up | 2.41 | 30.0 | pipelines/wait_ms 977, pipelines/compile_ms 111, pipelines/front_ms 108, render/thread/ms 73.34, render/busy/ms 72.19, main/busy/ms 15.45, render/graph/ms 5.31, pipelines/driver_ms 3.03, stream/drive/ms 1.91, render/present/ms 0.93, stream/tick/ms 0.80, terrain/update/ms 0.60 |
| warm-up | 2.57 | 41.5 | pipelines/wait_ms 13.5 k, pipelines/compile_ms 141, pipelines/front_ms 131, render/thread/ms 58.07, render/busy/ms 56.82, main/busy/ms 20.55, pipelines/driver_ms 9.98, render/graph/ms 6.69, stream/drive/ms 3.17, stream/spawn/ms 1.99, render/present/ms 1.02, stream/tick/ms 0.66 |
| warm-up | 2.61 | 38.4 | pipelines/wait_ms 4298, render/thread/ms 76.15, render/busy/ms 74.95, pipelines/compile_ms 35.72, pipelines/front_ms 33.04, main/busy/ms 20.83, render/graph/ms 6.30, stream/drive/ms 3.31, pipelines/driver_ms 2.68, stream/spawn/ms 2.20, render/present/ms 0.95, stream/tick/ms 0.66 |
| warm-up | 2.66 | 44.0 | render/thread/ms 78.80, render/busy/ms 77.64, main/busy/ms 18.63, render/graph/ms 6.80, stream/drive/ms 3.64, stream/spawn/ms 2.19, render/present/ms 0.92, stream/tick/ms 0.72, terrain/update/ms 0.55, physics/insert/ms 0.50, terrain/select/ms 0.47, bigspace/total/ms 0.44 |
| warm-up | 2.69 | 32.9 | render/thread/ms 73.39, render/busy/ms 72.16, main/busy/ms 17.48, render/graph/ms 6.75, stream/drive/ms 3.04, stream/spawn/ms 1.66, render/present/ms 0.99, stream/tick/ms 0.70, terrain/update/ms 0.56, physics/insert/ms 0.50, terrain/select/ms 0.48, bigspace/total/ms 0.45 |
| warm-up | 2.75 | 35.0 | pipelines/wait_ms 5446, pipelines/compile_ms 147, pipelines/front_ms 140, render/thread/ms 59.22, render/busy/ms 58.12, main/busy/ms 15.80, pipelines/driver_ms 6.44, render/graph/ms 6.11, stream/drive/ms 3.19, stream/spawn/ms 2.11, render/present/ms 0.89, stream/tick/ms 0.64 |
| warm-up | 2.81 | 54.1 | pipelines/wait_ms 1362, pipelines/compile_ms 101, pipelines/front_ms 97.89, render/thread/ms 85.97, render/busy/ms 84.72, main/busy/ms 15.76, render/graph/ms 6.28, pipelines/driver_ms 3.32, stream/drive/ms 2.89, stream/spawn/ms 1.79, render/present/ms 0.99, stream/tick/ms 0.64 |
| warm-up | 2.85 | 43.4 | pipelines/wait_ms 1362, pipelines/compile_ms 101, pipelines/front_ms 97.89, render/thread/ms 94.02, render/busy/ms 92.76, main/busy/ms 16.15, render/graph/ms 6.10, pipelines/driver_ms 3.32, stream/drive/ms 2.53, stream/spawn/ms 1.31, render/present/ms 1.00, stream/tick/ms 0.65 |
| warm-up | 3.00 | 52.3 | pipelines/wait_ms 6476, pipelines/compile_ms 208, pipelines/front_ms 199, render/thread/ms 67.77, render/busy/ms 66.66, main/busy/ms 21.26, pipelines/driver_ms 8.92, render/graph/ms 4.99, stream/drive/ms 3.71, stream/spawn/ms 1.93, render/present/ms 0.83, stream/tick/ms 0.75 |
| warm-up | 3.03 | 29.5 | pipelines/wait_ms 6476, pipelines/compile_ms 208, pipelines/front_ms 199, render/thread/ms 77.74, render/busy/ms 76.66, main/busy/ms 22.16, pipelines/driver_ms 8.92, render/graph/ms 5.76, stream/drive/ms 3.85, stream/spawn/ms 2.13, physics/insert/ms 0.87, render/present/ms 0.86 |
| warm-up | 3.09 | 33.5 | render/thread/ms 50.53, render/busy/ms 49.39, main/busy/ms 18.64, render/graph/ms 5.37, stream/drive/ms 2.39, stream/spawn/ms 1.07, render/present/ms 0.88, stream/tick/ms 0.83, terrain/update/ms 0.74, terrain/select/ms 0.64, physics/insert/ms 0.57, bigspace/total/ms 0.52 |
| warm-up | 3.28 | 28.1 | pipelines/wait_ms 12.1 k, pipelines/compile_ms 48.44, render/thread/ms 46.17, pipelines/front_ms 45.86, render/busy/ms 44.98, main/busy/ms 22.94, render/graph/ms 7.59, stream/drive/ms 3.31, pipelines/driver_ms 2.58, stream/spawn/ms 2.10, render/present/ms 0.97, bigspace/total/ms 0.77 |
| warm-up | 3.48 | 29.5 | main/busy/ms 42.60, render/thread/ms 19.94, render/busy/ms 18.74, render/graph/ms 6.13, stream/drive/ms 1.15, bigspace/total/ms 0.98, render/present/ms 0.97, physics/insert/ms 0.73, anim/evaluate_cpu/ms 0.66, stream/tick/ms 0.59, terrain/update/ms 0.56, terrain/select/ms 0.48 |
| measure | 0.43 | 74.5 | render/thread/ms 80.81, render/busy/ms 79.29, render/graph/ms 16.30, main/busy/ms 15.62, render/present/ms 1.18, physics/insert/ms 1.17, bigspace/total/ms 0.93, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.42, stream/drive/ms 0.41, bigspace/low_precision/ms 0.37 |
| measure | 0.84 | 53.4 | render/thread/ms 67.04, render/busy/ms 65.80, main/busy/ms 23.67, render/graph/ms 13.00, render/present/ms 1.02, bigspace/total/ms 1.00, stream/drive/ms 0.66, terrain/update/ms 0.63, terrain/select/ms 0.56, controller/feed/ms 0.52, anim/evaluate_cpu/ms 0.44, stream/tick/ms 0.34 |
| measure | 2.22 | 74.0 | render/thread/ms 82.59, render/busy/ms 81.42, render/graph/ms 16.25, main/busy/ms 15.19, pipelines/compile_ms 10.62, pipelines/front_ms 10.41, bigspace/total/ms 1.29, render/present/ms 0.97, bigspace/high_precision/ms 0.67, terrain/update/ms 0.54, terrain/select/ms 0.49, anim/evaluate_cpu/ms 0.42 |
| measure | 2.77 | 75.1 | render/thread/ms 84.63, render/busy/ms 83.45, render/graph/ms 16.81, main/busy/ms 15.39, render/present/ms 0.96, bigspace/total/ms 0.92, physics/insert/ms 0.78, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.48, stream/drive/ms 0.39, bigspace/low_precision/ms 0.35 |
| measure | 6.05 | 61.6 | render/thread/ms 68.88, render/busy/ms 67.84, main/busy/ms 13.10, render/graph/ms 8.62, render/present/ms 0.81, bigspace/total/ms 0.80, terrain/update/ms 0.52, stream/drive/ms 0.47, terrain/select/ms 0.46, controller/feed/ms 0.45, stream/tick/ms 0.41, bigspace/low_precision/ms 0.37 |
| measure | 8.16 | 62.0 | render/thread/ms 69.38, render/busy/ms 68.24, main/busy/ms 15.03, render/graph/ms 8.67, bigspace/total/ms 0.93, render/present/ms 0.90, anim/evaluate/ms 0.58, terrain/update/ms 0.54, terrain/select/ms 0.48, controller/feed/ms 0.48, bigspace/low_precision/ms 0.42, stream/drive/ms 0.41 |
| measure | 9.33 | 66.4 | render/thread/ms 74.87, render/busy/ms 73.84, main/busy/ms 15.00, render/graph/ms 9.58, bigspace/total/ms 0.86, render/present/ms 0.84, terrain/update/ms 0.56, terrain/select/ms 0.51, controller/feed/ms 0.44, bigspace/low_precision/ms 0.42, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.35 |
| measure | 9.41 | 67.1 | render/thread/ms 76.46, render/busy/ms 75.39, main/busy/ms 15.72, render/graph/ms 9.66, bigspace/total/ms 0.97, render/present/ms 0.82, controller/feed/ms 0.56, terrain/update/ms 0.50, stream/drive/ms 0.45, terrain/select/ms 0.45, bigspace/low_precision/ms 0.40, anim/evaluate_cpu/ms 0.38 |
| measure | 11.56 | 66.6 | render/thread/ms 74.30, render/busy/ms 73.18, main/busy/ms 14.95, render/graph/ms 8.46, render/present/ms 0.88, bigspace/total/ms 0.73, controller/feed/ms 0.53, terrain/update/ms 0.53, terrain/select/ms 0.47, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.38, stream/tick/ms 0.34 |
| measure | 12.03 | 69.3 | render/thread/ms 77.10, render/busy/ms 75.97, main/busy/ms 14.04, render/graph/ms 9.44, render/present/ms 0.87, bigspace/total/ms 0.76, terrain/update/ms 0.51, controller/feed/ms 0.46, terrain/select/ms 0.46, anim/evaluate_cpu/ms 0.39, stream/drive/ms 0.38, stream/tick/ms 0.34 |
| measure | 16.18 | 65.8 | render/thread/ms 73.30, render/busy/ms 72.16, main/busy/ms 14.30, render/graph/ms 9.07, render/present/ms 0.91, bigspace/total/ms 0.77, terrain/update/ms 0.54, controller/feed/ms 0.52, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.39, stream/drive/ms 0.37, far/select/ms 0.34 |
| measure | 16.41 | 66.0 | render/thread/ms 73.93, render/busy/ms 71.99, main/busy/ms 14.53, render/graph/ms 9.42, render/present/ms 1.71, bigspace/total/ms 0.83, terrain/update/ms 0.56, terrain/select/ms 0.50, controller/feed/ms 0.46, anim/evaluate_cpu/ms 0.39, stream/drive/ms 0.37, far/select/ms 0.35 |
| measure | 16.99 | 66.5 | render/thread/ms 75.07, render/busy/ms 73.95, main/busy/ms 14.46, render/graph/ms 10.55, render/present/ms 0.92, bigspace/total/ms 0.81, terrain/update/ms 0.59, terrain/select/ms 0.51, controller/feed/ms 0.48, stream/drive/ms 0.40, far/select/ms 0.37, stream/tick/ms 0.35 |
| measure | 18.12 | 67.0 | render/thread/ms 74.89, render/busy/ms 73.80, main/busy/ms 14.20, render/graph/ms 9.69, bigspace/total/ms 0.88, render/present/ms 0.88, controller/feed/ms 0.56, terrain/update/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.38, anim/evaluate_cpu/ms 0.34, stream/tick/ms 0.34 |
| measure | 18.25 | 28.0 | render/thread/ms 37.68, render/busy/ms 36.61, main/busy/ms 17.99, render/graph/ms 10.37, bigspace/total/ms 1.02, stream/drive/ms 0.99, render/present/ms 0.87, stream/spawn/ms 0.60, terrain/update/ms 0.56, controller/feed/ms 0.55, terrain/select/ms 0.50, anim/evaluate_cpu/ms 0.44 |
| measure | 18.82 | 76.0 | render/thread/ms 88.41, render/busy/ms 87.32, main/busy/ms 16.05, render/graph/ms 12.06, bigspace/total/ms 1.12, render/present/ms 0.85, terrain/update/ms 0.59, bigspace/low_precision/ms 0.56, terrain/select/ms 0.53, controller/feed/ms 0.51, far/select/ms 0.39, stream/drive/ms 0.39 |
| measure | 19.28 | 67.6 | render/thread/ms 75.15, render/busy/ms 74.09, main/busy/ms 15.03, render/graph/ms 9.40, render/present/ms 0.84, physics/insert/ms 0.77, bigspace/total/ms 0.74, terrain/update/ms 0.55, terrain/select/ms 0.50, controller/feed/ms 0.47, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.36 |
| measure | 19.37 | 83.7 | render/thread/ms 92.85, render/busy/ms 91.73, render/graph/ms 17.70, main/busy/ms 16.82, render/present/ms 0.91, physics/insert/ms 0.75, bigspace/total/ms 0.70, terrain/update/ms 0.56, terrain/select/ms 0.49, controller/feed/ms 0.47, stream/drive/ms 0.43, anim/evaluate_cpu/ms 0.35 |
| measure | 20.17 | 71.9 | render/thread/ms 81.89, render/busy/ms 80.68, main/busy/ms 17.11, render/graph/ms 10.98, render/present/ms 1.00, physics/insert/ms 0.94, bigspace/total/ms 0.79, controller/feed/ms 0.64, terrain/update/ms 0.55, terrain/select/ms 0.50, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.38 |
| measure | 21.00 | 70.9 | render/thread/ms 80.07, render/busy/ms 78.88, main/busy/ms 15.02, render/graph/ms 10.93, render/present/ms 0.96, bigspace/total/ms 0.85, terrain/update/ms 0.57, controller/feed/ms 0.53, terrain/select/ms 0.52, anim/evaluate_cpu/ms 0.42, stream/drive/ms 0.38, stream/tick/ms 0.34 |
| measure | 22.28 | 68.6 | render/thread/ms 77.30, render/busy/ms 76.12, main/busy/ms 15.90, render/graph/ms 10.24, render/present/ms 0.94, bigspace/total/ms 0.81, terrain/update/ms 0.55, anim/evaluate/ms 0.55, controller/feed/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.46, anim/evaluate_cpu/ms 0.41 |
| measure | 22.47 | 70.1 | render/thread/ms 79.34, render/busy/ms 78.27, main/busy/ms 14.52, render/graph/ms 10.68, render/present/ms 0.88, bigspace/total/ms 0.71, controller/feed/ms 0.56, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.38, anim/evaluate_cpu/ms 0.34, stream/tick/ms 0.33 |
| measure | 22.77 | 69.4 | render/thread/ms 80.20, render/busy/ms 79.00, main/busy/ms 17.16, render/graph/ms 12.15, anim/evaluate/ms 1.91, render/present/ms 0.99, bigspace/total/ms 0.79, controller/feed/ms 0.62, terrain/update/ms 0.57, stream/drive/ms 0.52, terrain/select/ms 0.51, stream/tick/ms 0.43 |
| measure | 23.22 | 68.1 | render/thread/ms 76.23, render/busy/ms 75.17, main/busy/ms 15.48, render/graph/ms 9.57, render/present/ms 0.85, bigspace/total/ms 0.75, stream/drive/ms 0.66, physics/insert/ms 0.66, controller/feed/ms 0.57, terrain/update/ms 0.53, terrain/select/ms 0.47, anim/evaluate/ms 0.40 |
| measure | 23.59 | 28.8 | render/thread/ms 37.91, render/busy/ms 36.83, main/busy/ms 16.76, render/graph/ms 10.05, physics/insert/ms 0.99, render/present/ms 0.90, stream/drive/ms 0.89, bigspace/total/ms 0.75, terrain/update/ms 0.63, terrain/select/ms 0.56, controller/feed/ms 0.53, stream/spawn/ms 0.48 |
| measure | 25.75 | 68.5 | render/thread/ms 76.12, render/busy/ms 74.95, main/busy/ms 14.22, render/graph/ms 9.11, bigspace/total/ms 1.20, render/present/ms 0.97, terrain/update/ms 0.56, terrain/select/ms 0.50, controller/feed/ms 0.47, stream/drive/ms 0.41, stream/tick/ms 0.36, far/select/ms 0.33 |
| measure | 26.14 | 70.3 | render/thread/ms 78.76, render/busy/ms 77.70, main/busy/ms 15.49, render/graph/ms 9.39, bigspace/total/ms 1.10, render/present/ms 0.86, terrain/update/ms 0.69, terrain/select/ms 0.62, controller/feed/ms 0.48, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.39, stream/tick/ms 0.34 |
| measure | 26.22 | 69.3 | render/thread/ms 77.69, render/busy/ms 76.53, main/busy/ms 14.79, render/graph/ms 10.09, render/present/ms 0.93, bigspace/total/ms 0.86, terrain/update/ms 0.60, stream/drive/ms 0.58, terrain/select/ms 0.53, stream/tick/ms 0.53, controller/feed/ms 0.48, far/select/ms 0.37 |
| measure | 26.32 | 72.7 | render/thread/ms 84.04, render/busy/ms 82.81, main/busy/ms 14.62, render/graph/ms 12.38, render/present/ms 1.00, bigspace/total/ms 0.79, terrain/update/ms 0.61, terrain/select/ms 0.53, controller/feed/ms 0.51, stream/drive/ms 0.41, anim/evaluate_cpu/ms 0.36, stream/tick/ms 0.36 |
| measure | 26.39 | 75.6 | render/thread/ms 145, render/busy/ms 144, main/busy/ms 14.86, render/graph/ms 12.51, render/present/ms 0.98, bigspace/total/ms 0.73, terrain/update/ms 0.57, terrain/select/ms 0.51, controller/feed/ms 0.46, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.37, stream/tick/ms 0.36 |
| measure | 26.48 | 74.7 | render/thread/ms 86.19, render/busy/ms 84.96, main/busy/ms 16.63, render/graph/ms 11.67, render/present/ms 1.01, bigspace/total/ms 0.88, terrain/update/ms 0.55, controller/feed/ms 0.55, terrain/select/ms 0.48, stream/drive/ms 0.39, far/select/ms 0.37, anim/evaluate_cpu/ms 0.35 |
| measure | 29.53 | 70.8 | render/thread/ms 79.28, render/busy/ms 78.20, main/busy/ms 16.13, render/graph/ms 9.40, bigspace/total/ms 0.95, render/present/ms 0.86, terrain/update/ms 0.53, controller/feed/ms 0.51, terrain/select/ms 0.48, stream/drive/ms 0.39, far/select/ms 0.36, anim/evaluate_cpu/ms 0.36 |
| measure | 30.01 | 70.2 | render/thread/ms 79.30, render/busy/ms 78.24, main/busy/ms 15.99, render/graph/ms 10.40, render/present/ms 0.87, bigspace/total/ms 0.80, terrain/update/ms 0.55, controller/feed/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.42, stream/tick/ms 0.37, anim/evaluate_cpu/ms 0.36 |
| measure | 33.01 | 70.5 | render/thread/ms 78.74, render/busy/ms 77.69, main/busy/ms 14.49, render/graph/ms 10.14, terrain/update/ms 1.07, render/present/ms 0.82, bigspace/total/ms 0.80, terrain/select/ms 0.57, controller/feed/ms 0.55, anim/evaluate_cpu/ms 0.43, stream/drive/ms 0.41, stream/tick/ms 0.36 |
| measure | 33.15 | 70.6 | render/thread/ms 78.64, render/busy/ms 77.43, main/busy/ms 16.35, render/graph/ms 9.40, render/present/ms 0.96, bigspace/total/ms 0.89, controller/feed/ms 0.55, terrain/update/ms 0.55, terrain/select/ms 0.50, anim/evaluate/ms 0.42, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.39 |
| measure | 34.35 | 71.3 | render/thread/ms 79.27, render/busy/ms 78.15, main/busy/ms 15.60, render/graph/ms 10.26, render/present/ms 0.91, bigspace/total/ms 0.88, terrain/update/ms 0.55, controller/feed/ms 0.53, terrain/select/ms 0.50, bigspace/low_precision/ms 0.41, far/select/ms 0.40, stream/drive/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **83%** (min 38%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.0 W · 72 °C (max 78) · VRAM 4146 MiB · throttled 100% of samples
