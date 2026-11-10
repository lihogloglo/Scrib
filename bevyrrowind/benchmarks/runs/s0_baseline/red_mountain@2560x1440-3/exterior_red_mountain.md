# `exterior_red_mountain`

- bench_ground: false · build: release · camera: [-120.0, 82.1, 80.0] -> [293.0, 250.0, -878.0] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1673 frames over 15.0 s · **median 8.86 ms (113 fps)** · mean 8.97 · p95 10.16 · p99 11.17 (1% low 89 fps) · worst 24.19 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 459 frames over 5.2 s, worst 574.7 ms
- hitches (> 25.0 ms): **0** measured, 6 in warm-up
- timed GPU passes: 7.80 ms of a 8.97 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.86 ms, limit 11 ms
- pass worst frame 24.19 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.156 | 2.524 | 0.175 |
| main_opaque_pass_3d | 1.790 | 2.244 | 0.041 |
| early prepass | 0.674 | 1.015 | 0.076 |
| lightprobe_irradiance_map | 0.436 | 0.751 | 0.001 |
| atmosphere_luts | 0.376 | 0.663 | 0.003 |
| bloom | 0.323 | 0.324 | 0.062 |
| clouds_trace | 0.267 | 0.304 | 0.001 |
| lightprobe_radiance_map | 0.239 | 0.239 | 0.004 |
| ssao | 0.215 | 0.210 | 0.007 |
| uniform_allocation | 0.157 | 0.154 | 0.014 |
| water_fft | 0.129 | 0.127 | 0.002 |
| upscaling | 0.125 | 0.157 | 0.004 |
| bin_unpacking | 0.107 | 0.112 | 0.010 |
| tonemapping | 0.095 | 0.093 | 0.003 |
| atmos_froxel_inject | 0.090 | 0.091 | 0.001 |
| main_transparent_pass_3d | 0.077 | 0.084 | 0.070 |
| water_velocity | 0.077 | 0.073 | 0.001 |
| clouds_composite | 0.066 | 0.065 | 0.002 |
| clustering | 0.065 | 0.074 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.037 | 0.005 |
| clouds_resolve | 0.035 | 0.046 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.025 | 0.006 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_copy | 0.015 | 0.017 | 0.004 |
| night_sky | 0.014 | 0.014 | 0.002 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 7.666 | 8.808 | 22.57 | 12824.8 |
| render/busy/ms | 6.979 | 7.907 | 12.46 | 11676.2 |
| main/busy/ms | 6.470 | 7.193 | 8.80 | 10823.6 |
| render/graph/ms | 3.443 | 4.098 | 6.45 | 5760.0 |
| render/present/ms | 0.488 | 1.191 | 14.18 | 815.8 |
| bigspace/total/ms | 0.329 | 0.521 | 1.44 | 549.7 |
| stream/drive/ms | 0.236 | 0.404 | 0.61 | 395.0 |
| terrain/update/ms | 0.234 | 0.294 | 0.79 | 391.8 |
| far/select/ms | 0.211 | 0.362 | 0.79 | 352.5 |
| stream/tick/ms | 0.205 | 0.363 | 0.59 | 343.3 |
| terrain/select/ms | 0.203 | 0.256 | 0.74 | 338.8 |
| render/acquire/ms | 0.199 | 0.734 | 1.70 | 332.7 |
| controller/feed/ms | 0.138 | 0.193 | 0.52 | 230.6 |
| controller/move/ms | 0.128 | 0.243 | 0.46 | 213.9 |
| bigspace/low_precision/ms | 0.102 | 0.173 | 0.80 | 170.0 |
| bigspace/high_precision/ms | 0.096 | 0.169 | 0.92 | 161.2 |
| anim/evaluate/ms | 0.092 | 0.136 | 0.98 | 153.8 |
| anim/evaluate_cpu/ms | 0.082 | 0.102 | 0.44 | 137.5 |
| anim/apply/ms | 0.079 | 0.109 | 0.92 | 132.3 |
| physics/step/ms | 0.037 | 0.080 | 1.39 | 62.2 |
| weather/map/ms | 0.024 | 0.033 | 0.09 | 40.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 18.2 |
| controller/avoid/ms | 0.009 | 0.014 | 0.17 | 15.5 |
| script/update/ms | 0.007 | 0.013 | 0.27 | 11.3 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.8 |
| script/ui/ms | 0.004 | 0.006 | 0.26 | 7.1 |
| controller/write/ms | 0.002 | 0.004 | 0.04 | 4.0 |
| physics/insert/ms | 0.002 | 0.000 | 0.84 | 3.5 |
| shapes/activate/ms | 0.001 | 0.003 | 0.04 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 0.9 |
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
| anim/a3/count | count | 502 | 503 | 502 |
| anim/characters/count | count | 503 | 503 | 503 |
| anim/evaluated/count | count | 15 | 15 | 15 |
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
| frame/work/ms | ms | 7.79 | 12.46 | 7.71 |
| gpu/timed/ms | ms | 7.78 | 8.81 | 7.71 |
| mesh_allocator_allocations |  meshes | 11.7 k | 11.7 k | 11.7 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 484.21 M | 484.21 M | 484.21 M |
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
| phases/shadow/bins | count | 1713 | 1913 | 1713 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4045 | 4307 | 4044 |
| phases/shadow/pipelines | count | 25 | 31 | 25 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2229 | 2229 | 2229 |
| physics/items | count | 15 | 15 | 15 |
| physics/pending | count | 0 | 255 | 0 |
| physics/terrain/chunks | count | 64 | 64 | 64 |
| pipelines/total | count | 157 | 157 | 157 |
| pipelines/waiting | count | 0 | 13 | 0 |
| preprocess/bind_groups | count | 154 | 167 | 154 |
| preprocess/bind_groups_created | count | 154 | 167 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25.96 | 36.71 | 24.55 |
| process/mem_usage | GiB | 2.01 | 2.06 | 2.06 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 214 | 214 | 214 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 23.03 M | 23.03 M | 23.03 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 1124 | 1124 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.44 | 48.12 | 29.97 |
| system/mem_usage | % | 62.17 | 62.41 | 62.41 |
| terrain/patches | count | 391 | 391 | 391 |
| views/all/batch_sets | count | 67 | 73 | 67 |
| views/all/bins | count | 4411 | 4611 | 4411 |
| views/all/count | count | 4 | 5 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.8 k | 11.1 k | 10.8 k |
| views/all/pipelines | count | 63 | 69 | 63 |
| views/all/pipelines_distinct | count | 53 | 55 | 53 |
| views/all/work_items | count | 11.1 k | 11.3 k | 11.1 k |
| views/cascade/batch_sets | count | 25 | 31 | 25 |
| views/cascade/bins | count | 1713 | 1913 | 1713 |
| views/cascade/count | count | 3 | 4 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4045 | 4307 | 4044 |
| views/cascade/pipelines | count | 25 | 31 | 25 |
| views/cascade/work_items | count | 4045 | 4307 | 4044 |
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
| anim/bones_written/count | count | 8.62 | 34 | 14.4 k |
| anim/events/count | count | 0.15 | 2 | 252 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.01 | 15 | 15 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.96 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 820 | 1287 |
| physics/inserts | count | 0.19 | 70 | 315 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.54 | 1 | 900 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 574.7 | main/busy/ms 553, surface/advance_cpu_ms 66.66, surface/cell_upload_ms 0.70, stream/drive/ms 0.37, weather/map/ms 0.16, stream/tick/ms 0.08, bigspace/total/ms 0.06, shapes/activate/ms 0.05, bigspace/high_precision/ms 0.03, weather/eval/ms 0.03, script/input/ms 0.03, script/update/ms 0.02 |
| warm-up | 0.88 | 307.1 | main/busy/ms 586, render/thread/ms 274, render/busy/ms 206, surface/advance_cpu_ms 66.66, render/acquire/ms 66.05, render/graph/ms 5.69, render/present/ms 1.95, stream/drive/ms 0.99, surface/cell_upload_ms 0.70, terrain/update/ms 0.51, stream/tick/ms 0.49, terrain/select/ms 0.37 |
| warm-up | 0.96 | 77.1 | render/thread/ms 294, render/busy/ms 224, render/acquire/ms 66.31, main/busy/ms 46.98, terrain/update/ms 18.27, render/graph/ms 10.66, render/present/ms 3.61, stream/drive/ms 1.00, stream/tick/ms 0.71, terrain/select/ms 0.67, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.11 |
| warm-up | 1.18 | 84.0 | main/busy/ms 59.09, render/thread/ms 51.30, render/busy/ms 49.73, render/graph/ms 4.16, stream/drive/ms 1.89, stream/tick/ms 1.52, render/present/ms 1.10, terrain/update/ms 0.63, terrain/select/ms 0.47, render/acquire/ms 0.46, bigspace/total/ms 0.28, bigspace/high_precision/ms 0.16 |
| warm-up | 1.35 | 26.4 | render/thread/ms 38.10, render/busy/ms 36.99, main/busy/ms 15.07, stream/drive/ms 4.12, render/graph/ms 3.04, stream/spawn/ms 2.05, render/present/ms 0.91, stream/tick/ms 0.68, terrain/update/ms 0.60, terrain/select/ms 0.45, bigspace/total/ms 0.30, stream/activate/ms 0.29 |
| warm-up | 1.52 | 25.6 | main/busy/ms 31.77, render/thread/ms 18.94, render/busy/ms 17.79, stream/drive/ms 3.79, render/graph/ms 2.98, stream/spawn/ms 1.96, physics/insert/ms 1.07, render/present/ms 0.97, stream/tick/ms 0.84, terrain/update/ms 0.55, stream/activate/ms 0.53, terrain/select/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --camera -120,+70,80,293,250,-878 --bench benchmarks/exterior_red_mountain.json --resolution 2560x1440 --bench-out target/perf\ab_main\red_mountain@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 79%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.1 W · 72 °C (max 76) · VRAM 4776 MiB · throttled 100% of samples
