# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1546 frames over 15.0 s · **median 9.58 ms (104 fps)** · mean 9.71 · p95 11.01 · p99 11.84 (1% low 84 fps) · worst 33.49 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 12.0/min)
- warm-up: 498 frames over 5.5 s, worst 577.7 ms
- hitches (> 25.0 ms): **1** measured, 5 in warm-up
- timed GPU passes: 8.56 ms of a 9.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.58 ms, limit 11 ms
- **FAIL** worst frame 33.49 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.558 | 2.966 | 0.046 |
| dlss_super_resolution | 2.205 | 2.584 | 0.189 |
| early prepass | 0.624 | 0.961 | 0.082 |
| lightprobe_irradiance_map | 0.429 | 0.426 | 0.001 |
| atmosphere_luts | 0.361 | 0.648 | 0.003 |
| bloom | 0.340 | 0.490 | 0.064 |
| lightprobe_radiance_map | 0.235 | 0.245 | 0.004 |
| ssao | 0.219 | 0.224 | 0.007 |
| main_transparent_pass_3d | 0.206 | 0.202 | 0.070 |
| uniform_allocation | 0.151 | 0.152 | 0.015 |
| clouds_trace | 0.138 | 0.164 | 0.001 |
| water_fft | 0.133 | 0.129 | 0.002 |
| bin_unpacking | 0.113 | 0.114 | 0.011 |
| upscaling | 0.108 | 0.147 | 0.004 |
| atmos_froxel_inject | 0.097 | 0.096 | 0.002 |
| tonemapping | 0.095 | 0.097 | 0.003 |
| water_velocity | 0.081 | 0.077 | 0.002 |
| clustering | 0.068 | 0.081 | 0.021 |
| clouds_composite | 0.068 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.041 | 0.005 |
| clouds_resolve | 0.036 | 0.040 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.031 | 0.007 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.473 | 9.761 | 32.28 | 13098.8 |
| render/busy/ms | 7.431 | 8.828 | 30.22 | 11488.4 |
| main/busy/ms | 6.792 | 7.732 | 28.35 | 10500.2 |
| render/graph/ms | 3.867 | 5.008 | 26.76 | 5978.9 |
| render/acquire/ms | 0.553 | 1.592 | 2.74 | 854.6 |
| render/present/ms | 0.489 | 0.642 | 13.23 | 755.8 |
| bigspace/total/ms | 0.322 | 0.599 | 1.62 | 497.7 |
| terrain/update/ms | 0.230 | 0.295 | 0.83 | 355.6 |
| controller/move/ms | 0.227 | 0.593 | 1.15 | 350.4 |
| stream/drive/ms | 0.209 | 0.370 | 0.85 | 322.3 |
| terrain/select/ms | 0.198 | 0.251 | 0.79 | 305.5 |
| far/select/ms | 0.194 | 0.346 | 0.61 | 300.7 |
| stream/tick/ms | 0.180 | 0.325 | 0.75 | 277.9 |
| controller/feed/ms | 0.136 | 0.199 | 0.47 | 210.6 |
| bigspace/high_precision/ms | 0.109 | 0.240 | 1.31 | 168.6 |
| anim/evaluate/ms | 0.095 | 0.192 | 1.53 | 147.0 |
| bigspace/low_precision/ms | 0.080 | 0.142 | 1.38 | 123.8 |
| anim/apply/ms | 0.076 | 0.158 | 1.09 | 117.5 |
| anim/evaluate_cpu/ms | 0.056 | 0.072 | 0.47 | 87.0 |
| physics/step/ms | 0.027 | 0.057 | 0.20 | 42.4 |
| weather/map/ms | 0.027 | 0.035 | 0.23 | 41.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.29 | 17.4 |
| script/update/ms | 0.007 | 0.011 | 0.04 | 10.3 |
| controller/avoid/ms | 0.006 | 0.009 | 0.18 | 9.7 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 7.3 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 6.4 |
| controller/write/ms | 0.002 | 0.004 | 0.06 | 3.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| physics/insert/ms | 0.000 | 0.000 | 0.03 | 0.0 |
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
| frame/work/ms | ms | 8.56 | 30.22 | 8.09 |
| gpu/timed/ms | ms | 8.54 | 9.51 | 8.08 |
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
| phases/shadow/bins | count | 2036 | 2036 | 2036 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5353 | 5353 | 5353 |
| phases/shadow/pipelines | count | 23 | 23 | 23 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 154 | 154 | 154 |
| preprocess/bind_groups_created | count | 154 | 154 | 154 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 29.81 | 39.29 | 35.58 |
| process/mem_usage | GiB | 2.20 | 2.28 | 2.28 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2591 | 2593 | 2593 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.96 | 55.40 | 42.20 |
| system/mem_usage | % | 62.66 | 62.91 | 62.91 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 67 | 67 |
| views/all/bins | count | 5218 | 5218 | 5218 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 63 | 63 | 63 |
| views/all/pipelines_distinct | count | 55 | 55 | 55 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 23 | 23 |
| views/cascade/bins | count | 2036 | 2036 | 2036 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5353 | 5353 | 5353 |
| views/cascade/pipelines | count | 23 | 23 | 23 |
| views/cascade/work_items | count | 5353 | 5353 | 5353 |
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
| anim/events/count | count | 0.11 | 3 | 166 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| physics/inserts | count | 0.00 | 3 | 3 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.7 | main/busy/ms 556, surface/advance_cpu_ms 66.05, surface/cell_upload_ms 0.79, stream/drive/ms 0.30, weather/map/ms 0.16, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.07, stream/tick/ms 0.06, weather/eval/ms 0.02, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 0.89 | 315.2 | main/busy/ms 590, render/thread/ms 276, render/busy/ms 208, surface/advance_cpu_ms 66.05, render/acquire/ms 65.41, render/graph/ms 4.76, render/present/ms 2.27, stream/drive/ms 0.95, surface/cell_upload_ms 0.79, stream/tick/ms 0.50, terrain/update/ms 0.48, terrain/select/ms 0.32 |
| warm-up | 0.98 | 90.0 | render/thread/ms 292, render/busy/ms 223, render/acquire/ms 65.72, main/busy/ms 49.05, terrain/update/ms 20.83, render/graph/ms 9.23, render/present/ms 3.73, stream/drive/ms 1.02, stream/tick/ms 0.73, terrain/select/ms 0.64, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.12 |
| warm-up | 1.22 | 82.4 | main/busy/ms 62.50, render/thread/ms 54.98, render/busy/ms 52.71, render/graph/ms 3.94, stream/drive/ms 2.11, render/present/ms 2.04, stream/tick/ms 1.25, terrain/update/ms 0.68, terrain/select/ms 0.54, bigspace/total/ms 0.34, stream/spawn/ms 0.33, render/acquire/ms 0.23 |
| warm-up | 3.48 | 26.1 | main/busy/ms 33.97, render/thread/ms 17.77, render/busy/ms 16.60, render/graph/ms 4.97, stream/drive/ms 0.97, render/present/ms 0.96, bigspace/total/ms 0.59, anim/evaluate_cpu/ms 0.48, far/select/ms 0.48, terrain/update/ms 0.46, anim/evaluate/ms 0.42, stream/tick/ms 0.38 |
| measure | 0.48 | 33.5 | render/thread/ms 40.12, render/busy/ms 36.79, main/busy/ms 35.42, render/graph/ms 29.98, render/acquire/ms 2.49, render/present/ms 0.85, bigspace/total/ms 0.57, terrain/update/ms 0.50, anim/evaluate/ms 0.47, far/select/ms 0.45, terrain/select/ms 0.43, stream/drive/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 94%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.8 W · 71 °C (max 74) · VRAM 4520 MiB · throttled 100% of samples
