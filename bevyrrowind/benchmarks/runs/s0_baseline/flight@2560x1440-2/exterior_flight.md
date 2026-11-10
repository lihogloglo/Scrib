# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6429 frames over 71.5 s · **median 10.89 ms (92 fps)** · mean 11.12 · p95 14.00 · p99 16.69 (1% low 60 fps) · worst 38.48 ms · 80 spikes over 1.5× median · 62.9 stutters/min (work 62.1/min)
- warm-up: 602 frames over 5.9 s, worst 579.6 ms
- hitches (> 27.2 ms): **4** measured, 3 in warm-up
- timed GPU passes: 8.49 ms of a 11.12 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.89 ms, limit 11 ms
- **FAIL** frame_ms: 75 stutters (> 1.5x moving median of 60) in 1.19 min = 62.92/min, limit 1/min
- **FAIL** worst frame 38.48 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.282 | 3.207 | 0.045 |
| dlss_super_resolution | 2.223 | 2.793 | 0.181 |
| early prepass | 0.588 | 0.960 | 0.079 |
| lightprobe_irradiance_map | 0.444 | 0.756 | 0.001 |
| atmosphere_luts | 0.386 | 0.647 | 0.003 |
| bloom | 0.337 | 0.489 | 0.064 |
| lightprobe_radiance_map | 0.230 | 0.236 | 0.004 |
| clouds_trace | 0.227 | 0.274 | 0.001 |
| main_transparent_pass_3d | 0.221 | 0.512 | 0.088 |
| ssao | 0.217 | 0.211 | 0.007 |
| uniform_allocation | 0.151 | 0.272 | 0.016 |
| water_fft | 0.132 | 0.129 | 0.003 |
| upscaling | 0.123 | 0.136 | 0.004 |
| bin_unpacking | 0.094 | 0.109 | 0.011 |
| atmos_froxel_inject | 0.093 | 0.093 | 0.002 |
| tonemapping | 0.093 | 0.093 | 0.003 |
| water_velocity | 0.081 | 0.095 | 0.002 |
| clustering | 0.071 | 0.072 | 0.020 |
| clouds_composite | 0.067 | 0.068 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| clouds_resolve | 0.038 | 0.047 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.041 | 0.005 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.031 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.030 | 0.007 |
| lightprobe_copy | 0.018 | 0.018 | 0.004 |
| lightprobe_downsampling_first_pass | 0.016 | 0.016 | 0.001 |
| weather_map | 0.014 | 0.014 | 0.002 |
| auxiliary_views | 0.014 | 0.003 | 0.013 |
| night_sky | 0.012 | 0.013 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.012 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.238 | 11.787 | 35.66 | 59392.3 |
| render/busy/ms | 8.571 | 11.231 | 26.54 | 55105.4 |
| main/busy/ms | 7.830 | 9.373 | 18.80 | 50338.1 |
| render/graph/ms | 3.915 | 4.828 | 12.74 | 25167.6 |
| render/present/ms | 0.469 | 0.599 | 25.58 | 3017.3 |
| bigspace/total/ms | 0.434 | 0.840 | 4.26 | 2789.8 |
| stream/drive/ms | 0.322 | 0.851 | 4.10 | 2067.7 |
| terrain/update/ms | 0.259 | 0.482 | 2.40 | 1662.5 |
| controller/feed/ms | 0.236 | 0.357 | 0.85 | 1520.1 |
| far/select/ms | 0.230 | 0.427 | 1.17 | 1475.9 |
| render/acquire/ms | 0.197 | 1.117 | 3.27 | 1269.6 |
| terrain/select/ms | 0.194 | 0.250 | 0.76 | 1248.8 |
| stream/tick/ms | 0.188 | 0.330 | 1.06 | 1211.3 |
| controller/move/ms | 0.182 | 0.474 | 2.17 | 1170.3 |
| bigspace/low_precision/ms | 0.153 | 0.342 | 3.56 | 984.8 |
| anim/evaluate_cpu/ms | 0.134 | 0.221 | 0.83 | 861.3 |
| bigspace/high_precision/ms | 0.117 | 0.262 | 1.51 | 750.1 |
| anim/apply/ms | 0.107 | 0.212 | 1.48 | 685.2 |
| anim/evaluate/ms | 0.101 | 0.227 | 1.21 | 646.7 |
| physics/step/ms | 0.074 | 0.193 | 1.85 | 472.6 |
| stream/despawn/ms | 0.046 | 0.463 | 2.51 | 292.8 |
| stream/spawn/ms | 0.034 | 0.270 | 1.76 | 219.9 |
| weather/map/ms | 0.028 | 0.037 | 0.41 | 178.4 |
| physics/insert/ms | 0.018 | 0.094 | 0.67 | 114.4 |
| controller/avoid/ms | 0.011 | 0.019 | 0.14 | 73.0 |
| weather/eval/ms | 0.010 | 0.015 | 0.12 | 65.1 |
| script/update/ms | 0.007 | 0.010 | 0.33 | 43.7 |
| stream/deactivate/ms | 0.006 | 0.000 | 0.58 | 38.6 |
| script/input/ms | 0.005 | 0.006 | 0.40 | 33.5 |
| stream/activate/ms | 0.005 | 0.000 | 0.59 | 30.9 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 29.0 |
| controller/write/ms | 0.003 | 0.005 | 0.27 | 20.8 |
| stream/upload/ms | 0.001 | 0.013 | 0.12 | 9.3 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 6.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.3 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.06 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 607 | 778 | 471 |
| anim/characters/count | count | 608 | 782 | 471 |
| anim/evaluated/count | count | 19 | 61 | 14 |
| controller/movers/count | count | 19 | 32 | 14 |
| ecs/entities | count | 60.1 k | 72.5 k | 53.0 k |
| far/entities | count | 2681 | 3461 | 2126 |
| far/forced | count | 0 | 15 | 0 |
| far/impostors | count | 61.0 k | 65.7 k | 55.8 k |
| far/lights | count | 7115 | 7757 | 6575 |
| far/nodes/l1 | count | 1100 | 1439 | 857 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 227.79 M | 173.66 M |
| far/triangles | count | 5.04 M | 5.72 M | 4.26 M |
| frame/work/ms | ms | 8.87 | 26.54 | 7.42 |
| gpu/timed/ms | ms | 8.44 | 12.65 | 7.16 |
| mesh_allocator_allocations |  meshes | 15.8 k | 19.3 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 537.07 M | 645.72 M | 645.72 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d/bins | count | 321 | 409 | 39 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 487 | 907 | 170 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d_prepass/bins | count | 321 | 409 | 39 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 487 | 907 | 170 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 5 |
| phases/opaque3d/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d/bins | count | 1143 | 1966 | 536 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3004 | 4158 | 1240 |
| phases/opaque3d/pipelines | count | 13 | 17 | 10 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d_prepass/bins | count | 1143 | 1966 | 536 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3004 | 4158 | 1240 |
| phases/opaque3d_prepass/pipelines | count | 13 | 17 | 10 |
| phases/shadow/batch_sets | count | 19 | 162 | 16 |
| phases/shadow/bins | count | 1574 | 2748 | 1788 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4187 | 6824 | 4574 |
| phases/shadow/pipelines | count | 19 | 162 | 16 |
| physics/awake | count | 0 | 4 | 0 |
| physics/bodies | count | 1892 | 2692 | 1955 |
| physics/items | count | 12 | 106 | 106 |
| physics/pending | count | 0 | 314 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 11 | 0 |
| preprocess/bind_groups | count | 142 | 441 | 112 |
| preprocess/bind_groups_created | count | 142 | 441 | 112 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25 | 39.62 | 26.44 |
| process/mem_usage | GiB | 2.64 | 2.81 | 2.81 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3883 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 373 | 446 | 406 |
| statics/materials_bound | count | 373 | 446 | 406 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 70 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.04 | 0.06 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 201 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.50 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 171 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 8 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.99 | 51.70 | 34.99 |
| system/mem_usage | % | 64.13 | 64.51 | 64.46 |
| terrain/patches | count | 349 | 493 | 319 |
| views/all/batch_sets | count | 61 | 192 | 46 |
| views/all/bins | count | 4518 | 6730 | 2938 |
| views/all/count | count | 4 | 42 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.4 k | 15.9 k | 7394 |
| views/all/pipelines | count | 59 | 188 | 46 |
| views/all/pipelines_distinct | count | 54 | 61 | 44 |
| views/all/work_items | count | 11.6 k | 16.1 k | 7423 |
| views/cascade/batch_sets | count | 19 | 30 | 16 |
| views/cascade/bins | count | 1564 | 2691 | 1788 |
| views/cascade/count | count | 3 | 6 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4156 | 6824 | 4574 |
| views/cascade/pipelines | count | 19 | 30 | 16 |
| views/cascade/work_items | count | 4156 | 6824 | 4574 |
| views/lamp_face/batch_sets | count | 0 | 136 | 0 |
| views/lamp_face/bins | count | 0 | 1502 | 0 |
| views/lamp_face/count | count | 0 | 36 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2178 | 0 |
| views/lamp_face/pipelines | count | 0 | 137 | 0 |
| views/lamp_face/work_items | count | 0 | 2132 | 0 |
| views/main/batch_sets | count | 42 | 50 | 30 |
| views/main/bins | count | 2890 | 4442 | 1150 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7168 | 10.1 k | 2820 |
| views/main/pipelines | count | 40 | 46 | 30 |
| views/main/work_items | count | 7366 | 10.3 k | 2849 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 26.5 k | 33.5 k | 28.1 k |
| world/point_lights | count | 269 | 550 | 524 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 9.28 | 224 | 59.7 k |
| anim/events/count | count | 0.21 | 4 | 1326 |
| assets/Image/added | count | 0.13 | 75 | 849 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 6.26 | 461 | 40.3 k |
| assets/Mesh/removed | count | 10.29 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.17 | 78 | 1107 |
| assets/StandardMaterial/modified | count | 5.30 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.07 | 36 | 470 |
| assets/WorldAsset/added | count | 0.01 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 12 | 124 |
| ecs/entities_delta | count | 2.90 | 2243 | 18.6 k |
| physics/inserts | count | 2.30 | 136 | 14.8 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.20 | 227 | 14.2 k |
| physics/steps | count | 0.67 | 2 | 4291 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.21 | 25 | 1345 |
| stream/deactivate/items | count | 0.21 | 21 | 1345 |
| stream/despawn/items | count | 1.09 | 295 | 6982 |
| stream/fetch/items | count | 0.23 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.71 | 31 | 4586 |
| stream/upload/bytes | bytes | 77.0 k | 3.51 M | 495.17 M |
| stream/upload/items | count | 0.20 | 15 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.04 | 1 | 269 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 17.9 k | 1.21 M | 114.83 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 27.2 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.6 | main/busy/ms 557, surface/advance_cpu_ms 69.59, surface/cell_upload_ms 0.65, stream/drive/ms 0.33, weather/map/ms 0.12, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.95 | 371.5 | main/busy/ms 589, render/thread/ms 337, render/busy/ms 270, surface/advance_cpu_ms 69.59, render/acquire/ms 65.53, render/graph/ms 5.39, render/present/ms 2.03, stream/drive/ms 0.94, surface/cell_upload_ms 0.65, terrain/update/ms 0.48, stream/tick/ms 0.48, terrain/select/ms 0.30 |
| warm-up | 1.20 | 80.5 | main/busy/ms 58.28, render/thread/ms 49.68, render/busy/ms 47.69, render/graph/ms 3.49, render/present/ms 1.62, stream/drive/ms 1.22, stream/tick/ms 0.97, terrain/update/ms 0.78, terrain/select/ms 0.58, render/acquire/ms 0.37, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.10 |
| measure | 35.26 | 37.2 | render/thread/ms 45.22, render/busy/ms 26.00, main/busy/ms 19.71, render/present/ms 19.00, render/graph/ms 9.80, bigspace/total/ms 0.86, controller/feed/ms 0.71, controller/move/ms 0.65, far/select/ms 0.65, terrain/update/ms 0.52, anim/evaluate_cpu/ms 0.50, terrain/select/ms 0.45 |
| measure | 35.51 | 29.2 | render/thread/ms 40.25, render/busy/ms 39.03, render/graph/ms 19.21, main/busy/ms 17.72, render/present/ms 1.02, controller/move/ms 0.97, bigspace/total/ms 0.92, terrain/update/ms 0.62, controller/feed/ms 0.48, terrain/select/ms 0.43, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.39 |
| measure | 50.24 | 30.5 | render/thread/ms 37.96, render/busy/ms 20.18, main/busy/ms 18.81, render/present/ms 17.58, render/graph/ms 8.89, bigspace/total/ms 0.83, stream/drive/ms 0.74, controller/feed/ms 0.49, terrain/update/ms 0.47, far/select/ms 0.47, terrain/select/ms 0.40, stream/tick/ms 0.37 |
| measure | 65.26 | 38.5 | render/thread/ms 46.37, render/present/ms 26.00, main/busy/ms 20.36, render/busy/ms 20.16, render/graph/ms 8.71, far/select/ms 1.02, bigspace/total/ms 0.96, terrain/update/ms 0.82, controller/feed/ms 0.64, terrain/select/ms 0.62, anim/apply/ms 0.60, stream/drive/ms 0.42 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\flight@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (277 samples): GPU load **85%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.7 W · 76 °C (max 79) · VRAM 4859 MiB · throttled 100% of samples
