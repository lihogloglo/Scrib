# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1630 frames over 15.0 s · **median 9.11 ms (110 fps)** · mean 9.21 · p95 10.46 · p99 11.15 (1% low 90 fps) · worst 18.37 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 290 frames over 3.7 s, worst 515.5 ms
- hitches (> 25.0 ms): **0** measured, 9 in warm-up
- timed GPU passes: 8.48 ms of a 9.21 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.11 ms, limit 11 ms
- pass worst frame 18.37 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.340 | 2.996 | 0.043 |
| dlss_super_resolution | 2.310 | 2.932 | 0.186 |
| early prepass | 0.635 | 1.307 | 0.077 |
| lightprobe_irradiance_map | 0.459 | 1.106 | 0.001 |
| atmosphere_luts | 0.385 | 1.009 | 0.003 |
| bloom | 0.358 | 0.493 | 0.065 |
| lightprobe_radiance_map | 0.237 | 0.236 | 0.004 |
| ssao | 0.237 | 0.350 | 0.007 |
| main_transparent_pass_3d | 0.197 | 0.207 | 0.073 |
| uniform_allocation | 0.149 | 0.135 | 0.014 |
| water_fft | 0.135 | 0.128 | 0.002 |
| upscaling | 0.133 | 0.151 | 0.004 |
| bin_unpacking | 0.108 | 0.104 | 0.009 |
| tonemapping | 0.102 | 0.099 | 0.003 |
| atmos_froxel_inject | 0.083 | 0.082 | 0.001 |
| water_velocity | 0.081 | 0.077 | 0.002 |
| clustering | 0.073 | 0.079 | 0.021 |
| clouds_trace | 0.068 | 0.088 | 0.001 |
| clouds_composite | 0.068 | 0.066 | 0.002 |
| render_sky | 0.056 | 0.054 | 0.002 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.005 |
| water_mips | 0.026 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.031 | 0.001 |
| water_snapshot | 0.026 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.005 |
| lightprobe_copy | 0.018 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.018 | 0.015 | 0.001 |
| lightprobe_downsampling_second_pass | 0.015 | 0.012 | 0.001 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.007 | 0.011 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.078 | 9.345 | 17.37 | 13167.8 |
| render/busy/ms | 6.824 | 8.103 | 16.66 | 11123.4 |
| main/busy/ms | 6.618 | 7.537 | 9.39 | 10786.9 |
| render/graph/ms | 3.613 | 4.638 | 13.51 | 5889.8 |
| render/acquire/ms | 0.792 | 1.983 | 3.07 | 1291.5 |
| render/present/ms | 0.462 | 0.584 | 3.01 | 752.9 |
| bigspace/total/ms | 0.340 | 0.676 | 2.07 | 553.7 |
| terrain/update/ms | 0.233 | 0.302 | 0.72 | 380.5 |
| stream/drive/ms | 0.205 | 0.366 | 0.61 | 333.5 |
| terrain/select/ms | 0.201 | 0.256 | 0.68 | 327.1 |
| far/select/ms | 0.179 | 0.299 | 0.57 | 292.5 |
| stream/tick/ms | 0.178 | 0.331 | 0.58 | 289.6 |
| controller/move/ms | 0.160 | 0.349 | 0.73 | 260.6 |
| controller/feed/ms | 0.134 | 0.197 | 0.49 | 218.6 |
| bigspace/high_precision/ms | 0.115 | 0.268 | 1.39 | 186.8 |
| bigspace/low_precision/ms | 0.090 | 0.211 | 1.87 | 147.4 |
| anim/apply/ms | 0.081 | 0.196 | 0.85 | 131.7 |
| anim/evaluate/ms | 0.079 | 0.195 | 1.03 | 128.0 |
| anim/evaluate_cpu/ms | 0.055 | 0.077 | 0.38 | 90.1 |
| physics/step/ms | 0.031 | 0.062 | 0.87 | 50.9 |
| physics/insert/ms | 0.022 | 0.162 | 0.59 | 36.7 |
| weather/map/ms | 0.017 | 0.022 | 0.34 | 27.1 |
| weather/eval/ms | 0.010 | 0.013 | 0.26 | 16.3 |
| script/update/ms | 0.007 | 0.014 | 0.37 | 11.7 |
| controller/avoid/ms | 0.006 | 0.011 | 0.10 | 10.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.0 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 7.0 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.3 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.41 | 16.66 | 7.83 |
| gpu/timed/ms | ms | 8.41 | 10.08 | 7.54 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
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
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 988 | 988 | 988 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2009 | 2009 | 2009 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1124 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 139 | 143 | 139 |
| preprocess/bind_groups_created | count | 139 | 143 | 139 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.13 | 40.38 | 24.48 |
| process/mem_usage | GiB | 2.12 | 2.19 | 2.19 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2589 | 2591 | 2591 |
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
| system/cpu_usage | % | 35.20 | 46.23 | 36.30 |
| system/mem_usage | % | 62.40 | 62.64 | 62.63 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 62 | 60 |
| views/all/bins | count | 4166 | 4168 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 58 | 56 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 988 | 988 | 988 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2009 | 2009 | 2009 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 2009 | 2009 | 2009 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3178 | 3180 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 42 | 40 |
| views/main/work_items | count | 9490 | 9490 | 9490 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 148 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.04 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.11 | 168 | 174 |
| physics/inserts | count | 0.69 | 124 | 1125 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 1 | 901 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.52 | 515.5 | main/busy/ms 493, surface/advance_cpu_ms 0.74, stream/drive/ms 0.42, surface/cell_upload_ms 0.28, stream/tick/ms 0.10, bigspace/total/ms 0.07, weather/map/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.93 | 413.2 | main/busy/ms 524, render/thread/ms 379, render/busy/ms 293, render/acquire/ms 83.83, render/graph/ms 4.99, render/present/ms 2.16, stream/drive/ms 0.99, surface/advance_cpu_ms 0.74, terrain/update/ms 0.68, stream/tick/ms 0.45, terrain/select/ms 0.41, surface/cell_upload_ms 0.28 |
| warm-up | 0.99 | 58.9 | render/thread/ms 395, render/busy/ms 308, render/acquire/ms 84.07, main/busy/ms 47.34, terrain/update/ms 17.84, render/graph/ms 9.79, render/present/ms 3.43, stream/drive/ms 0.92, terrain/select/ms 0.71, stream/tick/ms 0.64, bigspace/total/ms 0.27, bigspace/high_precision/ms 0.17 |
| warm-up | 1.20 | 87.0 | main/busy/ms 58.86, render/thread/ms 49.00, render/busy/ms 47.27, render/graph/ms 5.50, stream/drive/ms 1.78, render/present/ms 1.47, stream/tick/ms 1.46, terrain/update/ms 0.51, terrain/select/ms 0.41, render/acquire/ms 0.26, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.09 |
| warm-up | 1.33 | 25.2 | render/thread/ms 34.48, render/busy/ms 33.47, main/busy/ms 13.60, render/graph/ms 3.15, stream/drive/ms 2.70, render/present/ms 0.82, terrain/update/ms 0.69, stream/tick/ms 0.63, terrain/select/ms 0.53, bigspace/total/ms 0.22, stream/spawn/ms 0.22, render/acquire/ms 0.19 |
| warm-up | 1.68 | 25.4 | render/thread/ms 32.09, render/busy/ms 30.91, main/busy/ms 16.16, render/graph/ms 3.22, stream/drive/ms 3.22, stream/spawn/ms 1.44, render/present/ms 0.94, terrain/update/ms 0.65, stream/tick/ms 0.60, terrain/select/ms 0.53, bigspace/total/ms 0.50, bigspace/high_precision/ms 0.27 |
| warm-up | 2.53 | 27.3 | main/busy/ms 51.84, render/thread/ms 19.28, render/busy/ms 17.85, render/graph/ms 4.89, stream/drive/ms 1.48, render/present/ms 1.20, anim/evaluate_cpu/ms 0.85, stream/tick/ms 0.71, bigspace/total/ms 0.71, terrain/update/ms 0.61, terrain/select/ms 0.49, physics/insert/ms 0.38 |
| warm-up | 2.57 | 39.9 | main/busy/ms 47.94, render/thread/ms 30.13, render/busy/ms 28.74, render/graph/ms 4.99, stream/drive/ms 1.43, render/present/ms 1.15, bigspace/total/ms 1.07, anim/evaluate_cpu/ms 0.92, stream/tick/ms 0.70, terrain/update/ms 0.62, bigspace/low_precision/ms 0.61, terrain/select/ms 0.50 |
| warm-up | 2.64 | 27.6 | main/busy/ms 38.77, render/thread/ms 23.49, render/busy/ms 22.26, render/graph/ms 4.87, stream/drive/ms 1.03, render/present/ms 0.98, bigspace/total/ms 0.76, stream/tick/ms 0.47, terrain/update/ms 0.45, far/select/ms 0.41, terrain/select/ms 0.38, bigspace/low_precision/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 60%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.4 W · 75 °C (max 77) · VRAM 4527 MiB · throttled 100% of samples
