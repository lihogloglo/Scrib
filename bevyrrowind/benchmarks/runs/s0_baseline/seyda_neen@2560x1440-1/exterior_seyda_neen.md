# `exterior_seyda_neen`

- bench_ground: false · build: release · camera: [-105.0, 8.0, 1100.0] -> [-140.0, 4.5, 1035.0] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1463 frames over 15.0 s · **median 10.08 ms (99 fps)** · mean 10.26 · p95 11.88 · p99 13.08 (1% low 76 fps) · worst 41.44 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 445 frames over 5.6 s, worst 573.2 ms
- hitches (> 25.2 ms): **2** measured, 6 in warm-up
- timed GPU passes: 8.66 ms of a 10.26 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.08 ms, limit 11 ms
- **FAIL** worst frame 41.44 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.219 | 2.589 | 0.193 |
| main_opaque_pass_3d | 2.158 | 2.575 | 0.046 |
| early prepass | 0.685 | 1.013 | 0.083 |
| main_transparent_pass_3d | 0.493 | 0.839 | 0.124 |
| lightprobe_irradiance_map | 0.429 | 0.423 | 0.001 |
| bloom | 0.339 | 0.489 | 0.070 |
| atmosphere_luts | 0.336 | 0.511 | 0.003 |
| lightprobe_radiance_map | 0.225 | 0.233 | 0.004 |
| ssao | 0.216 | 0.212 | 0.008 |
| clouds_trace | 0.198 | 0.251 | 0.001 |
| uniform_allocation | 0.183 | 0.181 | 0.017 |
| water_fft | 0.133 | 0.129 | 0.003 |
| upscaling | 0.130 | 0.162 | 0.005 |
| bin_unpacking | 0.121 | 0.127 | 0.012 |
| water_velocity | 0.097 | 0.098 | 0.002 |
| atmos_froxel_inject | 0.095 | 0.104 | 0.002 |
| tonemapping | 0.093 | 0.094 | 0.004 |
| clustering | 0.074 | 0.075 | 0.022 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.056 | 0.057 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.044 | 0.005 |
| clouds_resolve | 0.036 | 0.044 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| auxiliary_views | 0.029 | 0.206 | 0.032 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.007 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| night_sky | 0.008 | 0.009 | 0.002 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.984 | 10.514 | 40.19 | 13144.1 |
| render/busy/ms | 8.221 | 9.620 | 39.57 | 12027.6 |
| main/busy/ms | 7.282 | 8.340 | 35.87 | 10653.0 |
| render/graph/ms | 4.179 | 5.099 | 35.09 | 6113.4 |
| render/present/ms | 0.542 | 1.352 | 13.70 | 792.8 |
| bigspace/total/ms | 0.356 | 0.537 | 1.40 | 520.9 |
| terrain/update/ms | 0.257 | 0.327 | 0.70 | 376.4 |
| terrain/select/ms | 0.224 | 0.283 | 0.67 | 327.8 |
| render/acquire/ms | 0.221 | 0.889 | 2.17 | 323.7 |
| stream/drive/ms | 0.215 | 0.363 | 0.50 | 314.1 |
| stream/tick/ms | 0.186 | 0.329 | 0.48 | 271.9 |
| controller/feed/ms | 0.172 | 0.236 | 0.57 | 251.7 |
| far/select/ms | 0.161 | 0.280 | 0.66 | 235.2 |
| anim/evaluate_cpu/ms | 0.140 | 0.178 | 0.50 | 205.4 |
| bigspace/low_precision/ms | 0.131 | 0.207 | 0.87 | 191.1 |
| controller/move/ms | 0.129 | 0.213 | 0.58 | 188.4 |
| anim/evaluate/ms | 0.097 | 0.190 | 1.07 | 142.0 |
| bigspace/high_precision/ms | 0.096 | 0.147 | 1.15 | 141.1 |
| anim/apply/ms | 0.061 | 0.087 | 1.04 | 89.6 |
| physics/step/ms | 0.046 | 0.089 | 0.61 | 66.9 |
| weather/map/ms | 0.027 | 0.036 | 0.17 | 39.5 |
| weather/eval/ms | 0.012 | 0.017 | 0.04 | 17.5 |
| controller/avoid/ms | 0.011 | 0.016 | 0.26 | 16.7 |
| script/update/ms | 0.007 | 0.014 | 0.03 | 10.3 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 7.4 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.5 |
| physics/insert/ms | 0.004 | 0.000 | 0.47 | 6.1 |
| controller/write/ms | 0.003 | 0.005 | 0.01 | 5.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 2 | 2 | 2 |
| anim/a3/count | count | 380 | 381 | 380 |
| anim/characters/count | count | 382 | 382 | 382 |
| anim/evaluated/count | count | 20 | 24 | 20 |
| controller/movers/count | count | 22 | 22 | 22 |
| ecs/entities | count | 42.6 k | 42.6 k | 42.6 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.71 | 39.57 | 8.83 |
| gpu/timed/ms | ms | 8.64 | 9.70 | 8.83 |
| mesh_allocator_allocations |  meshes | 11.6 k | 11.6 k | 11.6 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 378.52 M | 378.52 M | 378.52 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 957 | 957 | 957 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 957 | 957 | 957 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1509 | 1509 | 1509 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1509 | 1509 | 1509 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3765 | 3765 | 3765 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 32 | 59 | 32 |
| phases/shadow/bins | count | 3605 | 3950 | 3605 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8894 | 9284 | 8892 |
| phases/shadow/pipelines | count | 97 | 104 | 97 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 1601 | 1601 | 1601 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 292 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 11 | 0 |
| preprocess/bind_groups | count | 172 | 235 | 172 |
| preprocess/bind_groups_created | count | 172 | 235 | 172 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.63 | 48.58 | 24.04 |
| process/mem_usage | GiB | 2.20 | 2.24 | 2.24 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2488 | 2489 | 2489 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 372 | 372 | 372 |
| statics/materials_bound | count | 372 | 372 | 372 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.07 | 0.07 | 0.07 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 192 | 192 | 192 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 16.10 M | 16.10 M | 16.10 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 792 | 792 | 792 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.54 | 62.67 | 38.39 |
| system/mem_usage | % | 62.59 | 62.84 | 62.84 |
| terrain/patches | count | 441 | 441 | 441 |
| views/all/batch_sets | count | 76 | 105 | 76 |
| views/all/bins | count | 7283 | 7628 | 7283 |
| views/all/count | count | 16 | 18 | 16 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 18.7 k | 18.3 k |
| views/all/pipelines | count | 137 | 146 | 137 |
| views/all/pipelines_distinct | count | 63 | 65 | 63 |
| views/all/work_items | count | 17.4 k | 18.1 k | 17.4 k |
| views/cascade/batch_sets | count | 32 | 39 | 32 |
| views/cascade/bins | count | 2642 | 2987 | 2642 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7688 | 8082 | 7688 |
| views/cascade/pipelines | count | 32 | 39 | 32 |
| views/cascade/work_items | count | 7688 | 8082 | 7688 |
| views/lamp_face/batch_sets | count | 0 | 20 | 0 |
| views/lamp_face/bins | count | 963 | 968 | 963 |
| views/lamp_face/count | count | 12 | 12 | 12 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1206 | 1213 | 1204 |
| views/lamp_face/pipelines | count | 65 | 67 | 65 |
| views/lamp_face/work_items | count | 0 | 318 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3678 | 3678 | 3678 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9444 | 9444 | 9444 |
| views/main/pipelines | count | 40 | 42 | 40 |
| views/main/work_items | count | 9727 | 9727 | 9727 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 23.0 k | 23.0 k | 23.0 k |
| world/point_lights | count | 394 | 394 | 394 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 11.92 | 34 | 17.4 k |
| anim/events/count | count | 0.05 | 3 | 77 |
| assets/Image/added | count | 0.00 | 1 | 2 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.01 | 18 | 18 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 2 | 2 |
| assets/StandardMaterial/modified | count | 1.53 | 5 | 2245 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.34 | 236 | 495 |
| physics/inserts | count | 0.18 | 91 | 268 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 3 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 4 | 6 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.2 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 573.2 | main/busy/ms 551, surface/advance_cpu_ms 64.94, surface/cell_upload_ms 0.65, stream/drive/ms 0.28, weather/map/ms 0.12, bigspace/total/ms 0.07, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.88 | 302.9 | main/busy/ms 590, render/thread/ms 264, render/busy/ms 197, render/acquire/ms 65.00, surface/advance_cpu_ms 64.94, render/graph/ms 4.74, render/present/ms 1.92, stream/drive/ms 0.83, surface/cell_upload_ms 0.65, terrain/update/ms 0.45, stream/tick/ms 0.43, terrain/select/ms 0.28 |
| warm-up | 0.96 | 83.7 | render/thread/ms 283, render/busy/ms 215, render/acquire/ms 65.17, main/busy/ms 56.66, terrain/update/ms 23.98, render/graph/ms 11.32, render/present/ms 3.07, stream/drive/ms 0.87, terrain/select/ms 0.71, stream/tick/ms 0.62, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.13 |
| warm-up | 1.14 | 86.4 | main/busy/ms 61.33, render/thread/ms 45.79, render/busy/ms 44.35, render/graph/ms 4.24, stream/drive/ms 1.58, stream/tick/ms 1.23, render/present/ms 1.21, terrain/update/ms 0.80, terrain/select/ms 0.65, render/acquire/ms 0.24, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.09 |
| warm-up | 1.18 | 27.3 | render/thread/ms 34.48, render/busy/ms 32.15, main/busy/ms 17.36, render/graph/ms 4.54, terrain/update/ms 1.97, stream/drive/ms 1.96, terrain/select/ms 1.79, render/present/ms 1.78, stream/spawn/ms 0.87, stream/tick/ms 0.72, render/acquire/ms 0.54, bigspace/total/ms 0.35 |
| warm-up | 4.82 | 29.1 | main/busy/ms 43.55, render/thread/ms 26.49, render/busy/ms 25.04, render/graph/ms 9.33, render/present/ms 1.23, bigspace/total/ms 0.96, physics/insert/ms 0.64, terrain/update/ms 0.61, anim/evaluate_cpu/ms 0.58, terrain/select/ms 0.53, stream/drive/ms 0.51, anim/evaluate/ms 0.49 |
| measure | 1.82 | 25.9 | render/thread/ms 35.82, render/busy/ms 21.35, main/busy/ms 17.93, render/present/ms 14.28, render/graph/ms 11.02, bigspace/total/ms 0.71, terrain/update/ms 0.53, terrain/select/ms 0.46, controller/feed/ms 0.41, stream/drive/ms 0.40, far/select/ms 0.36, stream/tick/ms 0.34 |
| measure | 3.27 | 41.4 | render/thread/ms 48.46, render/busy/ms 47.33, main/busy/ms 42.65, render/graph/ms 39.17, render/present/ms 0.96, bigspace/total/ms 0.68, terrain/update/ms 0.63, terrain/select/ms 0.54, stream/drive/ms 0.37, controller/feed/ms 0.34, stream/tick/ms 0.32, anim/evaluate_cpu/ms 0.29 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --camera -105,+8,1100,-140,+3,1035 --bench benchmarks/exterior_seyda_neen.json --resolution 2560x1440 --bench-out target/perf\ab_main\seyda_neen@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **94%** (min 80%) · 2580 MHz (min 2580) · memory 8001 MHz · 89.1 W · 72 °C (max 74) · VRAM 4292 MiB · throttled 100% of samples
