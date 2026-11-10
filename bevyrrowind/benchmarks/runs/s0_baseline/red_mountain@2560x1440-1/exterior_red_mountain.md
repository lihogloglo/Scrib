# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1603 frames over 15.0 s · **median 9.27 ms (108 fps)** · mean 9.36 · p95 10.67 · p99 11.80 (1% low 85 fps) · worst 27.77 ms · 4 spikes over 1.5× median · 20.0 stutters/min (work 8.0/min)
- warm-up: 331 frames over 4.5 s, worst 575.2 ms
- hitches (> 25.0 ms): **1** measured, 7 in warm-up
- timed GPU passes: 7.87 ms of a 9.36 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.27 ms, limit 11 ms
- pass worst frame 27.77 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.169 | 2.538 | 0.178 |
| main_opaque_pass_3d | 1.820 | 2.244 | 0.055 |
| early prepass | 0.706 | 1.037 | 0.080 |
| lightprobe_irradiance_map | 0.440 | 0.757 | 0.001 |
| atmosphere_luts | 0.366 | 0.652 | 0.003 |
| bloom | 0.324 | 0.476 | 0.066 |
| clouds_trace | 0.264 | 0.296 | 0.001 |
| lightprobe_radiance_map | 0.229 | 0.237 | 0.004 |
| ssao | 0.217 | 0.352 | 0.007 |
| uniform_allocation | 0.162 | 0.326 | 0.022 |
| water_fft | 0.129 | 0.129 | 0.003 |
| upscaling | 0.120 | 0.142 | 0.004 |
| bin_unpacking | 0.103 | 0.109 | 0.015 |
| tonemapping | 0.096 | 0.104 | 0.004 |
| main_transparent_pass_3d | 0.090 | 0.089 | 0.074 |
| atmos_froxel_inject | 0.088 | 0.091 | 0.001 |
| water_velocity | 0.080 | 0.075 | 0.002 |
| clustering | 0.067 | 0.068 | 0.022 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| render_sky | 0.051 | 0.053 | 0.002 |
| clouds_resolve | 0.035 | 0.046 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.006 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.007 |
| water_mips | 0.023 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.023 | 0.030 | 0.001 |
| lightprobe_copy | 0.014 | 0.017 | 0.004 |
| night_sky | 0.014 | 0.014 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.011 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| auxiliary_views | 0.001 | 0.002 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.044 | 9.322 | 26.05 | 12895.0 |
| render/busy/ms | 7.396 | 8.528 | 13.12 | 11856.2 |
| main/busy/ms | 6.758 | 7.593 | 12.12 | 10832.5 |
| render/graph/ms | 3.552 | 4.382 | 6.67 | 5694.5 |
| render/present/ms | 0.555 | 1.297 | 16.87 | 889.4 |
| bigspace/total/ms | 0.331 | 0.507 | 1.72 | 531.2 |
| stream/drive/ms | 0.244 | 0.403 | 1.28 | 391.2 |
| terrain/update/ms | 0.240 | 0.299 | 0.74 | 384.5 |
| far/select/ms | 0.239 | 0.402 | 0.86 | 383.1 |
| stream/tick/ms | 0.212 | 0.360 | 1.25 | 340.0 |
| terrain/select/ms | 0.206 | 0.257 | 0.70 | 330.7 |
| controller/feed/ms | 0.161 | 0.221 | 0.53 | 257.4 |
| controller/move/ms | 0.128 | 0.228 | 0.54 | 205.3 |
| bigspace/low_precision/ms | 0.098 | 0.167 | 1.09 | 157.3 |
| bigspace/high_precision/ms | 0.096 | 0.159 | 1.47 | 153.5 |
| render/acquire/ms | 0.093 | 0.122 | 0.64 | 149.3 |
| anim/evaluate_cpu/ms | 0.086 | 0.111 | 0.43 | 137.7 |
| anim/apply/ms | 0.081 | 0.099 | 1.08 | 129.8 |
| anim/evaluate/ms | 0.068 | 0.096 | 1.15 | 108.5 |
| physics/step/ms | 0.041 | 0.083 | 0.95 | 66.5 |
| weather/map/ms | 0.026 | 0.036 | 0.43 | 42.1 |
| physics/insert/ms | 0.014 | 0.069 | 0.91 | 23.2 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 18.9 |
| controller/avoid/ms | 0.010 | 0.015 | 0.42 | 15.6 |
| script/update/ms | 0.007 | 0.012 | 0.13 | 12.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.0 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 6.0 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.7 |
| shapes/activate/ms | 0.001 | 0.003 | 0.01 | 2.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 1 | 1 |
| anim/a2/count | count | 0 | 1 | 0 |
| anim/a3/count | count | 503 | 503 | 502 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 61 | 15 |
| controller/movers/count | count | 15 | 15 | 15 |
| ecs/entities | count | 51.4 k | 51.4 k | 51.4 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.88 | 13.12 | 7.72 |
| gpu/timed/ms | ms | 7.83 | 9.23 | 7.72 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.19 M | 433.19 M | 433.19 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 331 | 331 | 331 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 462 | 462 | 462 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 331 | 331 | 331 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 462 | 462 | 462 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 1018 | 1019 | 1018 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2936 | 2942 | 2936 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1018 | 1019 | 1018 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2936 | 2942 | 2936 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 25 | 31 | 25 |
| phases/shadow/bins | count | 1728 | 1919 | 1713 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4060 | 4319 | 4045 |
| phases/shadow/pipelines | count | 25 | 31 | 25 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 1419 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 152 | 152 | 152 |
| pipelines/waiting | count | 0 | 23 | 0 |
| preprocess/bind_groups | count | 154 | 167 | 154 |
| preprocess/bind_groups_created | count | 154 | 167 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 26.92 | 44.23 | 22.01 |
| process/mem_usage | GiB | 2.03 | 2.09 | 2.09 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2657 | 2658 | 2658 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 355 | 355 | 355 |
| statics/materials_bound | count | 355 | 355 | 355 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 214 | 214 | 214 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 23.03 M | 23.03 M | 23.03 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 1124 | 1124 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.93 | 51.60 | 31.11 |
| system/mem_usage | % | 62.17 | 62.28 | 62.28 |
| terrain/patches | count | 391 | 391 | 391 |
| views/all/batch_sets | count | 67 | 73 | 67 |
| views/all/bins | count | 4426 | 4617 | 4411 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.9 k | 11.1 k | 10.8 k |
| views/all/pipelines | count | 63 | 69 | 63 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 11.1 k | 11.3 k | 11.1 k |
| views/cascade/batch_sets | count | 25 | 31 | 25 |
| views/cascade/bins | count | 1728 | 1919 | 1713 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4060 | 4319 | 4045 |
| views/cascade/pipelines | count | 25 | 31 | 25 |
| views/cascade/work_items | count | 4060 | 4319 | 4045 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 42 | 44 | 42 |
| views/main/bins | count | 2698 | 2700 | 2698 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 6796 | 6808 | 6796 |
| views/main/pipelines | count | 38 | 40 | 38 |
| views/main/work_items | count | 7014 | 7026 | 7014 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 23.9 k | 23.9 k | 23.9 k |
| world/point_lights | count | 200 | 200 | 200 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 5.49 | 34 | 8803 |
| anim/events/count | count | 0.15 | 3 | 239 |
| assets/Image/added | count | 0.04 | 18 | 60 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.10 | 46 | 164 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.04 | 18 | 65 |
| assets/StandardMaterial/modified | count | 3.09 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.49 | 2085 | 7198 |
| physics/inserts | count | 0.88 | 182 | 1412 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 3 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 575.2 | main/busy/ms 554, surface/advance_cpu_ms 68.47, surface/cell_upload_ms 0.70, stream/drive/ms 0.30, weather/map/ms 0.14, stream/tick/ms 0.08, bigspace/total/ms 0.05, bigspace/high_precision/ms 0.03, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.88 | 300.5 | main/busy/ms 585, render/thread/ms 265, render/busy/ms 199, surface/advance_cpu_ms 68.47, render/acquire/ms 63.68, render/graph/ms 5.55, render/present/ms 2.01, stream/drive/ms 0.89, surface/cell_upload_ms 0.70, terrain/update/ms 0.51, stream/tick/ms 0.46, terrain/select/ms 0.34 |
| warm-up | 0.96 | 81.6 | render/thread/ms 283, render/busy/ms 215, render/acquire/ms 64.05, main/busy/ms 46.36, terrain/update/ms 16.86, render/graph/ms 10.19, render/present/ms 3.34, stream/drive/ms 0.97, stream/tick/ms 0.69, terrain/select/ms 0.64, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.12 |
| warm-up | 1.17 | 92.9 | main/busy/ms 69.01, render/thread/ms 50.47, render/busy/ms 48.78, render/graph/ms 4.55, stream/drive/ms 1.74, render/present/ms 1.44, stream/tick/ms 1.11, terrain/update/ms 0.57, terrain/select/ms 0.43, bigspace/total/ms 0.40, render/acquire/ms 0.26, bigspace/high_precision/ms 0.25 |
| warm-up | 1.60 | 27.8 | render/thread/ms 32.25, render/busy/ms 31.01, main/busy/ms 19.65, stream/drive/ms 5.28, render/graph/ms 3.48, stream/spawn/ms 1.78, render/present/ms 1.03, stream/tick/ms 1.00, bigspace/total/ms 0.68, terrain/update/ms 0.60, terrain/select/ms 0.48, bigspace/high_precision/ms 0.42 |
| warm-up | 2.74 | 25.9 | main/busy/ms 48.21, render/thread/ms 24.34, render/busy/ms 22.57, render/graph/ms 8.66, render/present/ms 1.59, stream/drive/ms 1.51, physics/insert/ms 0.77, stream/tick/ms 0.67, bigspace/total/ms 0.57, physics/step/ms 0.55, stream/spawn/ms 0.51, terrain/update/ms 0.48 |
| warm-up | 2.77 | 30.4 | main/busy/ms 40.79, render/thread/ms 29.81, render/busy/ms 28.32, render/graph/ms 8.22, render/present/ms 1.30, stream/drive/ms 1.18, bigspace/total/ms 0.68, physics/step/ms 0.55, terrain/update/ms 0.54, stream/tick/ms 0.51, terrain/select/ms 0.46, anim/evaluate_cpu/ms 0.42 |
| measure | 0.95 | 27.8 | render/thread/ms 35.37, render/busy/ms 17.60, render/present/ms 17.57, main/busy/ms 16.51, render/graph/ms 8.41, anim/evaluate/ms 1.19, far/select/ms 0.71, bigspace/total/ms 0.64, stream/drive/ms 0.54, terrain/update/ms 0.50, stream/tick/ms 0.47, controller/avoid/ms 0.43 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_main\red_mountain@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 6%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **93%** (min 73%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.2 W · 74 °C (max 75) · VRAM 4522 MiB · throttled 100% of samples
