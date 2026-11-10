# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5837 frames over 65.6 s · **median 10.90 ms (92 fps)** · mean 11.25 · p95 13.60 · p99 15.48 (1% low 65 fps) · worst 26.89 ms · 43 spikes over 1.5× median · 20.1 stutters/min (work 24.7/min)
- warm-up: 352 frames over 4.6 s, worst 579.5 ms
- hitches (> 27.2 ms): **0** measured, 4 in warm-up
- timed GPU passes: 7.53 ms of a 11.25 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.90 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- pass worst frame 26.89 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.257 | 2.798 | 0.183 |
| main_opaque_pass_3d | 1.468 | 2.108 | 0.047 |
| early prepass | 0.511 | 0.873 | 0.077 |
| lightprobe_irradiance_map | 0.441 | 0.752 | 0.001 |
| bloom | 0.343 | 0.508 | 0.067 |
| atmosphere_luts | 0.332 | 0.506 | 0.003 |
| lightprobe_radiance_map | 0.226 | 0.236 | 0.004 |
| ssao | 0.221 | 0.215 | 0.008 |
| uniform_allocation | 0.182 | 0.184 | 0.018 |
| auxiliary_views | 0.164 | 0.607 | 0.147 |
| water_fft | 0.134 | 0.130 | 0.003 |
| upscaling | 0.130 | 0.148 | 0.004 |
| bin_unpacking | 0.128 | 0.142 | 0.013 |
| clouds_trace | 0.127 | 0.215 | 0.001 |
| atmos_froxel_inject | 0.101 | 0.108 | 0.002 |
| tonemapping | 0.091 | 0.091 | 0.003 |
| water_velocity | 0.087 | 0.085 | 0.002 |
| clustering | 0.068 | 0.070 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.109 | 0.108 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.049 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.675 | 11.943 | 29.12 | 56471.1 |
| render/busy/ms | 9.127 | 11.388 | 21.86 | 53274.4 |
| main/busy/ms | 8.217 | 9.961 | 16.17 | 47964.5 |
| render/graph/ms | 4.501 | 5.798 | 13.42 | 26269.7 |
| render/present/ms | 0.455 | 0.571 | 13.48 | 2658.3 |
| bigspace/total/ms | 0.414 | 0.617 | 2.06 | 2418.9 |
| terrain/update/ms | 0.291 | 0.371 | 1.55 | 1701.4 |
| terrain/select/ms | 0.256 | 0.324 | 0.81 | 1496.0 |
| stream/drive/ms | 0.254 | 0.440 | 6.05 | 1481.2 |
| controller/feed/ms | 0.235 | 0.320 | 0.78 | 1372.7 |
| far/select/ms | 0.224 | 0.388 | 0.96 | 1309.1 |
| stream/tick/ms | 0.205 | 0.361 | 1.11 | 1193.7 |
| anim/evaluate_cpu/ms | 0.166 | 0.216 | 0.65 | 966.4 |
| controller/move/ms | 0.164 | 0.371 | 0.87 | 958.4 |
| anim/evaluate/ms | 0.162 | 0.286 | 1.53 | 943.8 |
| bigspace/low_precision/ms | 0.159 | 0.236 | 1.52 | 928.5 |
| bigspace/high_precision/ms | 0.100 | 0.158 | 1.41 | 585.1 |
| render/acquire/ms | 0.092 | 0.119 | 0.81 | 538.4 |
| anim/apply/ms | 0.088 | 0.141 | 1.33 | 514.0 |
| physics/step/ms | 0.047 | 0.083 | 0.66 | 277.2 |
| physics/insert/ms | 0.044 | 0.394 | 1.23 | 256.7 |
| weather/map/ms | 0.026 | 0.036 | 0.39 | 153.9 |
| controller/avoid/ms | 0.011 | 0.018 | 0.41 | 66.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.27 | 59.6 |
| script/update/ms | 0.007 | 0.008 | 0.32 | 38.2 |
| stream/despawn/ms | 0.006 | 0.000 | 0.98 | 32.7 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 28.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.17 | 26.9 |
| script/ui/ms | 0.004 | 0.006 | 0.18 | 25.7 |
| controller/write/ms | 0.004 | 0.006 | 0.19 | 22.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.76 | 18.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.04 | 6.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 9 | 0 |
| anim/a2/count | count | 1 | 12 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 39 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
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
| frame/work/ms | ms | 8.89 | 21.86 | 8.80 |
| gpu/timed/ms | ms | 7.56 | 11.41 | 8.80 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 815 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 815 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5142 | 3179 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5142 | 3179 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 37 | 235 | 29 |
| phases/shadow/bins | count | 3750 | 5516 | 2244 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9460 | 13.0 k | 5695 |
| phases/shadow/pipelines | count | 115 | 250 | 29 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 833 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 32 | 0 |
| preprocess/bind_groups | count | 184 | 618 | 166 |
| preprocess/bind_groups_created | count | 184 | 618 | 166 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 23.45 | 37.98 | 22.12 |
| process/mem_usage | GiB | 2.63 | 2.71 | 2.71 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3482 | 3548 | 3548 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.58 | 52.41 | 28.83 |
| system/mem_usage | % | 64.15 | 64.46 | 64.40 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 82 | 275 | 73 |
| views/all/bins | count | 6430 | 8394 | 5724 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 155 | 287 | 69 |
| views/all/pipelines_distinct | count | 60 | 69 | 55 |
| views/all/work_items | count | 17.2 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 29 |
| views/cascade/bins | count | 2618 | 3197 | 2244 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8167 | 10.3 k | 5695 |
| views/cascade/pipelines | count | 36 | 50 | 29 |
| views/cascade/work_items | count | 8167 | 10.3 k | 5695 |
| views/lamp_face/batch_sets | count | 0 | 196 | 0 |
| views/lamp_face/bins | count | 1058 | 2486 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2847 | 0 |
| views/lamp_face/pipelines | count | 76 | 206 | 0 |
| views/lamp_face/work_items | count | 0 | 2659 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8443 | 13.0 k | 7592 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 8604 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 54.40 | 334 | 317.5 k |
| anim/events/count | count | 0.26 | 4 | 1535 |
| assets/Image/added | count | 0.07 | 31 | 408 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.76 | 160 | 4422 |
| assets/Mesh/removed | count | 0.84 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.10 | 31 | 571 |
| assets/StandardMaterial/modified | count | 3.71 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 2.16 | 1599 | 12.6 k |
| physics/inserts | count | 0.52 | 121 | 3051 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 25 | 378 |
| stream/upload/bytes | bytes | 5800 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1272 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.5 | main/busy/ms 558, surface/advance_cpu_ms 67.55, surface/cell_upload_ms 0.85, stream/drive/ms 0.32, weather/map/ms 0.12, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.99 | 413.4 | main/busy/ms 591, render/thread/ms 376, render/busy/ms 305, render/acquire/ms 68.58, surface/advance_cpu_ms 67.55, render/graph/ms 5.42, render/present/ms 2.54, stream/drive/ms 1.05, surface/cell_upload_ms 0.85, terrain/update/ms 0.66, stream/tick/ms 0.54, terrain/select/ms 0.45 |
| warm-up | 1.25 | 96.9 | main/busy/ms 63.73, render/thread/ms 63.43, render/busy/ms 61.76, render/graph/ms 4.00, stream/drive/ms 2.27, stream/tick/ms 1.88, render/present/ms 1.37, terrain/update/ms 0.78, terrain/select/ms 0.67, bigspace/total/ms 0.36, render/acquire/ms 0.30, bigspace/high_precision/ms 0.18 |
| warm-up | 2.46 | 58.4 | render/thread/ms 48.02, render/busy/ms 46.64, main/busy/ms 43.80, render/graph/ms 6.28, stream/drive/ms 4.12, stream/spawn/ms 2.13, render/present/ms 1.07, terrain/update/ms 0.72, stream/tick/ms 0.70, bigspace/total/ms 0.65, terrain/select/ms 0.60, physics/insert/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 8%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **78%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.9 W · 73 °C (max 78) · VRAM 4868 MiB · throttled 100% of samples
