# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1531 frames over 15.0 s · **median 9.65 ms (104 fps)** · mean 9.80 · p95 11.32 · p99 12.05 (1% low 83 fps) · worst 17.68 ms · 3 spikes over 1.5× median · 4.0 stutters/min (work 8.0/min)
- warm-up: 1217 frames over 15.8 s, worst 2471.9 ms
- hitches (> 25.0 ms): **0** measured, 13 in warm-up
- timed GPU passes: 8.58 ms of a 9.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.65 ms, limit 11 ms
- pass worst frame 17.68 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.542 | 2.944 | 0.049 |
| dlss_super_resolution | 2.214 | 2.586 | 0.188 |
| early prepass | 0.625 | 0.966 | 0.084 |
| lightprobe_irradiance_map | 0.450 | 0.762 | 0.001 |
| atmosphere_luts | 0.362 | 0.641 | 0.003 |
| bloom | 0.342 | 0.497 | 0.068 |
| lightprobe_radiance_map | 0.238 | 0.240 | 0.004 |
| ssao | 0.216 | 0.217 | 0.008 |
| main_transparent_pass_3d | 0.205 | 0.359 | 0.071 |
| uniform_allocation | 0.155 | 0.154 | 0.020 |
| clouds_trace | 0.138 | 0.163 | 0.001 |
| water_fft | 0.132 | 0.128 | 0.003 |
| upscaling | 0.119 | 0.148 | 0.004 |
| bin_unpacking | 0.110 | 0.114 | 0.014 |
| atmos_froxel_inject | 0.094 | 0.095 | 0.002 |
| tonemapping | 0.094 | 0.099 | 0.004 |
| water_velocity | 0.079 | 0.078 | 0.002 |
| clustering | 0.067 | 0.080 | 0.022 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.006 |
| clouds_resolve | 0.036 | 0.043 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.032 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.022 | 0.029 | 0.007 |
| lightprobe_copy | 0.017 | 0.018 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| sparse buffer updates | 0.006 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.535 | 9.996 | 16.16 | 13067.6 |
| render/busy/ms | 7.727 | 9.200 | 13.68 | 11829.8 |
| main/busy/ms | 6.827 | 7.942 | 8.98 | 10452.8 |
| render/graph/ms | 3.975 | 5.030 | 6.97 | 6085.8 |
| render/present/ms | 0.495 | 0.692 | 6.90 | 757.9 |
| render/acquire/ms | 0.313 | 1.169 | 2.27 | 479.9 |
| bigspace/total/ms | 0.309 | 0.494 | 1.39 | 472.8 |
| terrain/update/ms | 0.234 | 0.296 | 0.85 | 358.4 |
| stream/drive/ms | 0.230 | 0.401 | 1.05 | 351.9 |
| far/select/ms | 0.209 | 0.349 | 0.71 | 319.8 |
| controller/move/ms | 0.208 | 0.589 | 1.04 | 318.4 |
| terrain/select/ms | 0.201 | 0.253 | 0.82 | 307.5 |
| stream/tick/ms | 0.197 | 0.358 | 0.93 | 301.8 |
| controller/feed/ms | 0.146 | 0.210 | 0.63 | 223.9 |
| bigspace/high_precision/ms | 0.100 | 0.166 | 1.20 | 152.8 |
| anim/apply/ms | 0.076 | 0.126 | 1.15 | 115.9 |
| bigspace/low_precision/ms | 0.075 | 0.125 | 0.66 | 114.6 |
| anim/evaluate/ms | 0.071 | 0.129 | 1.30 | 109.1 |
| anim/evaluate_cpu/ms | 0.065 | 0.081 | 0.22 | 99.8 |
| physics/step/ms | 0.031 | 0.063 | 0.41 | 47.5 |
| weather/map/ms | 0.028 | 0.037 | 0.15 | 42.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.06 | 18.0 |
| script/update/ms | 0.008 | 0.016 | 0.04 | 11.9 |
| controller/avoid/ms | 0.006 | 0.009 | 0.03 | 9.5 |
| script/input/ms | 0.005 | 0.007 | 0.12 | 8.0 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.3 |
| physics/insert/ms | 0.003 | 0.017 | 0.76 | 5.3 |
| controller/write/ms | 0.002 | 0.003 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.09 | 1.9 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 46.9 k | 46.9 k | 46.9 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.59 | 13.68 | 8.29 |
| gpu/timed/ms | ms | 8.55 | 9.67 | 8.29 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 406.00 M | 406.00 M | 406.00 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/opaque3d/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/shadow/batch_sets | count | 23 | 23 | 23 |
| phases/shadow/bins | count | 2035 | 2036 | 2035 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5352 | 5353 | 5352 |
| phases/shadow/pipelines | count | 23 | 23 | 23 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 120 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 148 | 148 | 148 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 154 | 154 | 154 |
| preprocess/bind_groups_created | count | 154 | 154 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.81 | 51.75 | 35.92 |
| process/mem_usage | GiB | 2.33 | 2.40 | 2.40 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2594 | 2597 | 2597 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.13 | 60.31 | 37.82 |
| system/mem_usage | % | 63.01 | 63.15 | 63.10 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 67 | 67 |
| views/all/bins | count | 5217 | 5218 | 5217 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 63 | 63 | 63 |
| views/all/pipelines_distinct | count | 55 | 55 | 55 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 23 | 23 |
| views/cascade/bins | count | 2035 | 2036 | 2035 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5352 | 5353 | 5352 |
| views/cascade/pipelines | count | 23 | 23 | 23 |
| views/cascade/work_items | count | 5352 | 5353 | 5352 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 44 | 44 |
| views/main/bins | count | 3182 | 3182 | 3182 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9294 | 9294 | 9294 |
| views/main/pipelines | count | 40 | 40 | 40 |
| views/main/work_items | count | 9472 | 9472 | 9472 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 4 | 159 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.23 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0.09 | 25 | 134 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 1 | 900 |
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
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 13, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.61 | 608.3 | main/busy/ms 586, surface/advance_cpu_ms 66.22, surface/cell_upload_ms 0.75, stream/drive/ms 0.40, weather/map/ms 0.16, stream/tick/ms 0.12, bigspace/total/ms 0.07, weather/eval/ms 0.05, bigspace/high_precision/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.02, script/input/ms 0.01 |
| warm-up | 1.12 | 516.5 | main/busy/ms 623, render/thread/ms 484, render/busy/ms 363, render/acquire/ms 84.57, surface/advance_cpu_ms 66.22, render/present/ms 36.42, render/graph/ms 8.20, stream/drive/ms 1.43, surface/cell_upload_ms 0.75, stream/tick/ms 0.65, terrain/update/ms 0.49, terrain/select/ms 0.29 |
| warm-up | 1.21 | 82.5 | render/thread/ms 504, render/busy/ms 382, render/acquire/ms 84.73, main/busy/ms 56.73, render/present/ms 37.57, terrain/update/ms 22.68, render/graph/ms 14.09, stream/drive/ms 1.53, stream/tick/ms 0.85, terrain/select/ms 0.68, bigspace/total/ms 0.17, weather/map/ms 0.15 |
| warm-up | 1.27 | 27.8 | render/thread/ms 45.97, render/acquire/ms 28.71, render/busy/ms 15.14, main/busy/ms 11.44, terrain/update/ms 5.57, render/graph/ms 5.49, render/present/ms 2.11, stream/drive/ms 0.94, stream/tick/ms 0.75, terrain/select/ms 0.48, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.14 |
| warm-up | 1.32 | 28.7 | render/thread/ms 50.51, render/acquire/ms 38.73, main/busy/ms 10.61, render/busy/ms 10.31, render/graph/ms 4.06, terrain/update/ms 1.54, render/present/ms 1.47, stream/drive/ms 1.02, stream/tick/ms 0.85, terrain/select/ms 0.75, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.12 |
| warm-up | 1.35 | 33.2 | render/thread/ms 59.21, render/acquire/ms 31.78, render/busy/ms 25.66, main/busy/ms 22.49, render/graph/ms 6.56, render/present/ms 1.77, stream/drive/ms 1.21, stream/tick/ms 1.03, terrain/update/ms 0.55, terrain/select/ms 0.41, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.16 |
| warm-up | 1.99 | 76.6 | main/busy/ms 2545, stream/drive/ms 2487, stream/activate/ms 2485, render/thread/ms 39.07, render/busy/ms 37.48, render/graph/ms 4.96, render/present/ms 1.34, stream/tick/ms 0.97, terrain/update/ms 0.60, terrain/select/ms 0.47, render/acquire/ms 0.25, bigspace/total/ms 0.16 |
| warm-up | 4.46 | 2471.9 | main/busy/ms 2506, stream/drive/ms 2488, stream/activate/ms 2485, render/thread/ms 40.05, render/busy/ms 38.45, terrain/update/ms 6.44, render/graph/ms 5.97, render/present/ms 1.39, stream/spawn/ms 0.96, stream/tick/ms 0.74, terrain/select/ms 0.53, render/acquire/ms 0.21 |
| warm-up | 6.74 | 26.2 | render/thread/ms 37.97, render/busy/ms 35.48, main/busy/ms 29.41, render/graph/ms 7.32, stream/drive/ms 2.87, bigspace/total/ms 2.25, render/present/ms 2.24, stream/spawn/ms 1.05, stream/tick/ms 0.84, bigspace/high_precision/ms 0.78, terrain/update/ms 0.71, terrain/select/ms 0.55 |
| warm-up | 6.77 | 28.3 | render/thread/ms 50.10, main/busy/ms 46.81, render/busy/ms 46.43, render/graph/ms 6.46, bigspace/total/ms 4.49, stream/drive/ms 3.39, bigspace/high_precision/ms 3.00, render/present/ms 2.78, stream/spawn/ms 1.61, render/acquire/ms 0.89, stream/tick/ms 0.87, terrain/update/ms 0.65 |
| warm-up | 6.84 | 39.6 | main/busy/ms 56.34, render/thread/ms 29.01, render/busy/ms 27.28, render/graph/ms 4.82, stream/drive/ms 3.37, stream/spawn/ms 2.05, render/present/ms 1.21, stream/tick/ms 0.74, bigspace/total/ms 0.68, terrain/update/ms 0.61, render/acquire/ms 0.52, terrain/select/ms 0.51 |
| warm-up | 11.65 | 28.4 | main/busy/ms 36.42, render/thread/ms 19.43, render/busy/ms 18.17, render/graph/ms 8.47, render/present/ms 1.09, bigspace/total/ms 0.66, terrain/update/ms 0.54, terrain/select/ms 0.46, stream/drive/ms 0.46, stream/tick/ms 0.40, far/select/ms 0.34, controller/feed/ms 0.21 |
| warm-up | 13.78 | 28.7 | render/thread/ms 36.26, render/busy/ms 34.00, main/busy/ms 10.33, render/graph/ms 9.22, render/present/ms 1.23, render/acquire/ms 1.03, bigspace/total/ms 0.51, terrain/update/ms 0.44, terrain/select/ms 0.38, stream/drive/ms 0.35, far/select/ms 0.32, stream/tick/ms 0.30 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **95%** (min 85%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.3 W · 71 °C (max 74) · VRAM 4281 MiB · throttled 100% of samples
