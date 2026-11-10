# `exterior_run`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3606 frames over 39.8 s · **median 10.35 ms (97 fps)** · mean 11.03 · p95 16.20 · p99 19.74 (1% low 51 fps) · worst 30.13 ms · 242 spikes over 1.5× median · 34.7 stutters/min (work 37.7/min)
- warm-up: 355 frames over 4.5 s, worst 587.2 ms
- hitches (> 25.9 ms): **3** measured, 6 in warm-up
- timed GPU passes: 8.26 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.35 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 0.66 min = 34.70/min, limit 1/min
- pass worst frame 30.13 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.258 | 2.807 | 0.184 |
| main_opaque_pass_3d | 1.863 | 2.726 | 0.050 |
| early prepass | 0.778 | 1.133 | 0.083 |
| lightprobe_irradiance_map | 0.437 | 0.557 | 0.001 |
| atmosphere_luts | 0.392 | 0.642 | 0.003 |
| bloom | 0.339 | 0.485 | 0.067 |
| lightprobe_radiance_map | 0.231 | 0.236 | 0.004 |
| ssao | 0.222 | 0.352 | 0.007 |
| uniform_allocation | 0.187 | 0.201 | 0.019 |
| clouds_trace | 0.157 | 0.202 | 0.001 |
| auxiliary_views | 0.147 | 1.142 | 0.130 |
| water_fft | 0.131 | 0.128 | 0.003 |
| upscaling | 0.129 | 0.135 | 0.004 |
| bin_unpacking | 0.118 | 0.139 | 0.014 |
| atmos_froxel_inject | 0.099 | 0.103 | 0.001 |
| tonemapping | 0.093 | 0.092 | 0.004 |
| water_velocity | 0.084 | 0.082 | 0.002 |
| clustering | 0.068 | 0.071 | 0.020 |
| main_transparent_pass_3d | 0.066 | 0.095 | 0.104 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.055 | 0.056 | 0.002 |
| early_mesh_preprocessing | 0.048 | 0.047 | 0.005 |
| clouds_resolve | 0.035 | 0.040 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| per_view_shadow_pass | 0.025 | 0.026 | 0.007 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.007 | 0.011 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.407 | 14.235 | 29.30 | 33921.3 |
| render/busy/ms | 8.796 | 13.647 | 26.43 | 31717.3 |
| main/busy/ms | 7.586 | 9.800 | 19.86 | 27354.8 |
| render/graph/ms | 4.316 | 7.318 | 17.65 | 15564.1 |
| render/present/ms | 0.508 | 1.231 | 8.39 | 1830.6 |
| bigspace/total/ms | 0.357 | 0.543 | 1.88 | 1287.4 |
| terrain/update/ms | 0.287 | 0.360 | 2.36 | 1035.0 |
| stream/drive/ms | 0.261 | 0.453 | 2.04 | 942.7 |
| terrain/select/ms | 0.249 | 0.306 | 0.77 | 898.1 |
| stream/tick/ms | 0.208 | 0.365 | 0.96 | 750.7 |
| controller/feed/ms | 0.186 | 0.254 | 0.60 | 670.4 |
| far/select/ms | 0.168 | 0.289 | 0.91 | 604.9 |
| anim/evaluate_cpu/ms | 0.124 | 0.172 | 0.52 | 448.6 |
| bigspace/low_precision/ms | 0.123 | 0.198 | 1.67 | 443.2 |
| anim/evaluate/ms | 0.109 | 0.223 | 2.43 | 394.3 |
| render/acquire/ms | 0.104 | 0.135 | 1.77 | 373.4 |
| controller/move/ms | 0.098 | 0.204 | 0.54 | 352.2 |
| bigspace/high_precision/ms | 0.096 | 0.152 | 1.40 | 346.6 |
| anim/apply/ms | 0.069 | 0.105 | 1.05 | 248.5 |
| physics/step/ms | 0.043 | 0.083 | 0.99 | 155.3 |
| physics/insert/ms | 0.039 | 0.364 | 1.01 | 138.9 |
| weather/map/ms | 0.026 | 0.034 | 0.18 | 92.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.16 | 41.6 |
| controller/avoid/ms | 0.009 | 0.015 | 0.07 | 33.3 |
| stream/spawn/ms | 0.007 | 0.000 | 1.30 | 26.2 |
| script/update/ms | 0.007 | 0.014 | 0.03 | 26.1 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 18.5 |
| script/ui/ms | 0.004 | 0.006 | 0.14 | 14.6 |
| controller/write/ms | 0.003 | 0.005 | 0.15 | 10.4 |
| stream/despawn/ms | 0.003 | 0.000 | 1.32 | 10.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.38 | 5.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.16 | 4.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 7 | 2 |
| anim/a2/count | count | 1 | 4 | 0 |
| anim/a3/count | count | 426 | 490 | 489 |
| anim/characters/count | count | 426 | 492 | 491 |
| anim/evaluated/count | count | 15 | 73 | 15 |
| controller/movers/count | count | 16 | 18 | 15 |
| ecs/entities | count | 47.2 k | 60.8 k | 59.9 k |
| far/entities | count | 1954 | 1954 | 1954 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.2 k | 55.2 k | 55.2 k |
| far/lights | count | 6515 | 6515 | 6515 |
| far/nodes/l1 | count | 778 | 778 | 778 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 167.30 M | 167.30 M | 167.30 M |
| far/triangles | count | 4.09 M | 4.09 M | 4.09 M |
| frame/work/ms | ms | 8.57 | 26.43 | 11.95 |
| gpu/timed/ms | ms | 8.28 | 12.40 | 8.76 |
| mesh_allocator_allocations |  meshes | 12.8 k | 16.2 k | 16.1 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 446.61 M | 446.61 M | 446.61 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 393 | 417 | 350 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 393 | 417 | 350 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 1028 | 1072 | 1016 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/opaque3d/batch_sets | count | 14 | 16 | 16 |
| phases/opaque3d/bins | count | 1976 | 2109 | 1789 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 4267 | 4505 | 4063 |
| phases/opaque3d/pipelines | count | 13 | 15 | 15 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1976 | 2109 | 1789 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 4267 | 4505 | 4063 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 15 |
| phases/shadow/batch_sets | count | 30 | 273 | 65 |
| phases/shadow/bins | count | 2853 | 8193 | 6399 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8199 | 13.9 k | 11.8 k |
| phases/shadow/pipelines | count | 31 | 277 | 193 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2404 | 3289 | 3236 |
| physics/items | count | 20 | 368 | 305 |
| physics/pending | count | 0 | 828 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 80 |
| pipelines/total | count | 163 | 175 | 175 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 166 | 710 | 250 |
| preprocess/bind_groups_created | count | 166 | 710 | 250 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 25.48 | 43.75 | 13.46 |
| process/mem_usage | GiB | 2.33 | 2.75 | 2.75 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 2886 | 3316 | 3316 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 377 | 540 | 534 |
| statics/materials_bound | count | 377 | 540 | 534 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 1 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.12 | 0.05 |
| stream/model_queue | count | 0 | 72 | 1 |
| stream/models | count | 228 | 438 | 429 |
| stream/oldest_request/s | s | 0 | 0.28 | 0.08 |
| stream/page_cache/bytes | bytes | 16.28 M | 16.52 M | 16.52 M |
| stream/requests | count | 0 | 2 | 1 |
| stream/resident_cells | count | 792 | 797 | 797 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.08 | 48.75 | 30.35 |
| system/mem_usage | % | 63.23 | 64.42 | 64.42 |
| terrain/patches | count | 498 | 520 | 490 |
| views/all/batch_sets | count | 73 | 321 | 113 |
| views/all/bins | count | 7711 | 12.5 k | 10.7 k |
| views/all/count | count | 4 | 54 | 40 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.8 k | 24.2 k | 22.0 k |
| views/all/pipelines | count | 70 | 321 | 237 |
| views/all/pipelines_distinct | count | 55 | 70 | 69 |
| views/all/work_items | count | 19.1 k | 24.0 k | 19.0 k |
| views/cascade/batch_sets | count | 28 | 44 | 37 |
| views/cascade/bins | count | 2808 | 3288 | 2815 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8058 | 8976 | 8021 |
| views/cascade/pipelines | count | 28 | 44 | 37 |
| views/cascade/work_items | count | 8058 | 8976 | 8021 |
| views/lamp_face/batch_sets | count | 0 | 235 | 28 |
| views/lamp_face/bins | count | 0 | 5051 | 3584 |
| views/lamp_face/count | count | 0 | 48 | 36 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 5295 | 3798 |
| views/lamp_face/pipelines | count | 0 | 235 | 156 |
| views/lamp_face/work_items | count | 0 | 5294 | 557 |
| views/main/batch_sets | count | 44 | 48 | 48 |
| views/main/bins | count | 4744 | 5004 | 4278 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 10.6 k | 11.1 k | 10.2 k |
| views/main/pipelines | count | 40 | 44 | 44 |
| views/main/work_items | count | 10.9 k | 11.3 k | 10.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 26.5 k | 35.1 k | 34.4 k |
| world/point_lights | count | 394 | 460 | 455 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 23.21 | 320 | 83.7 k |
| anim/events/count | count | 0.11 | 4 | 411 |
| assets/Image/added | count | 0.10 | 51 | 351 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0.00 | 4 | 8 |
| assets/Mesh/added | count | 1.70 | 300 | 6146 |
| assets/Mesh/removed | count | 0.67 | 718 | 2434 |
| assets/StandardMaterial/added | count | 0.15 | 52 | 539 |
| assets/StandardMaterial/modified | count | 2.18 | 10 | 7874 |
| assets/StandardMaterial/removed | count | 0.04 | 26 | 128 |
| assets/WorldAsset/added | count | 0.01 | 7 | 36 |
| assets/WorldAsset/removed | count | 0.01 | 6 | 30 |
| ecs/entities_delta | count | 5.80 | 2353 | 20.9 k |
| physics/inserts | count | 0.85 | 133 | 3064 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.34 | 162 | 1231 |
| physics/steps | count | 0.66 | 2 | 2385 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 7 | 14 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 18 | 214 |
| stream/deactivate/items | count | 0.04 | 16 | 147 |
| stream/despawn/items | count | 0.03 | 11 | 95 |
| stream/fetch/items | count | 0.02 | 2 | 77 |
| stream/shift_guard/frames | count | 0.00 | 1 | 4 |
| stream/spawn/items | count | 0.11 | 18 | 394 |
| stream/upload/bytes | bytes | 10.9 k | 2.07 M | 39.18 M |
| stream/upload/items | count | 0.01 | 1 | 50 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2534 | 1.02 M | 9.14 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 2 |

## Hitches (> 25.9 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 587.2 | main/busy/ms 564, surface/advance_cpu_ms 76.20, surface/cell_upload_ms 0.67, stream/drive/ms 0.30, weather/map/ms 0.11, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, script/update/ms 0.02, weather/eval/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.89 | 307.6 | main/busy/ms 597, render/thread/ms 272, render/busy/ms 204, surface/advance_cpu_ms 76.20, render/acquire/ms 65.46, render/graph/ms 4.65, render/present/ms 1.99, stream/drive/ms 0.95, surface/cell_upload_ms 0.67, terrain/update/ms 0.54, stream/tick/ms 0.45, terrain/select/ms 0.37 |
| warm-up | 0.97 | 71.6 | render/thread/ms 288, render/busy/ms 219, render/acquire/ms 65.83, main/busy/ms 50.07, terrain/update/ms 18.42, render/graph/ms 9.36, render/present/ms 3.32, stream/drive/ms 1.12, stream/tick/ms 0.77, terrain/select/ms 0.74, bigspace/total/ms 0.53, bigspace/high_precision/ms 0.12 |
| warm-up | 1.20 | 86.2 | main/busy/ms 57.57, render/thread/ms 52.31, render/busy/ms 50.62, render/graph/ms 3.77, render/present/ms 1.35, stream/drive/ms 1.22, stream/tick/ms 1.01, terrain/update/ms 0.93, terrain/select/ms 0.79, render/acquire/ms 0.35, bigspace/total/ms 0.29, bigspace/high_precision/ms 0.14 |
| warm-up | 2.68 | 28.1 | render/thread/ms 32.72, main/busy/ms 32.01, render/busy/ms 31.40, render/graph/ms 6.55, stream/drive/ms 1.24, render/present/ms 1.11, terrain/update/ms 0.64, bigspace/total/ms 0.61, terrain/select/ms 0.55, stream/tick/ms 0.54, stream/spawn/ms 0.37, physics/insert/ms 0.32 |
| warm-up | 2.74 | 31.7 | main/busy/ms 40.42, render/thread/ms 23.48, render/busy/ms 22.35, render/graph/ms 7.18, bigspace/total/ms 1.07, stream/drive/ms 1.03, render/present/ms 0.97, bigspace/low_precision/ms 0.63, terrain/update/ms 0.56, terrain/select/ms 0.47, stream/tick/ms 0.45, stream/spawn/ms 0.32 |
| measure | 35.37 | 26.2 | render/thread/ms 41.76, render/busy/ms 40.08, render/graph/ms 22.59, main/busy/ms 21.81, render/present/ms 1.51, bigspace/total/ms 0.86, stream/drive/ms 0.76, terrain/update/ms 0.71, physics/insert/ms 0.66, terrain/select/ms 0.49, anim/evaluate_cpu/ms 0.42, controller/feed/ms 0.40 |
| measure | 37.37 | 27.4 | render/thread/ms 39.43, render/busy/ms 38.27, main/busy/ms 20.65, render/graph/ms 20.36, physics/insert/ms 1.03, render/present/ms 0.90, bigspace/total/ms 0.81, terrain/update/ms 0.56, stream/drive/ms 0.52, terrain/select/ms 0.50, stream/tick/ms 0.46, anim/evaluate_cpu/ms 0.45 |
| measure | 37.40 | 30.1 | render/thread/ms 52.75, render/busy/ms 51.34, render/graph/ms 31.10, main/busy/ms 18.47, render/present/ms 1.24, terrain/update/ms 1.16, bigspace/total/ms 0.87, physics/insert/ms 0.59, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.45, controller/feed/ms 0.41, stream/drive/ms 0.40 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_run.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\run@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (154 samples): GPU load **87%** (min 59%) · 2580 MHz (min 2580) · memory 8001 MHz · 84.1 W · 74 °C (max 77) · VRAM 4345 MiB · throttled 100% of samples
