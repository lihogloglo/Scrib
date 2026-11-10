# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5874 frames over 65.7 s · **median 10.77 ms (93 fps)** · mean 11.18 · p95 13.70 · p99 15.20 (1% low 66 fps) · worst 28.22 ms · 30 spikes over 1.5× median · 14.6 stutters/min (work 17.4/min)
- warm-up: 457 frames over 5.7 s, worst 567.4 ms
- hitches (> 26.9 ms): **1** measured, 6 in warm-up
- timed GPU passes: 7.45 ms of a 11.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.77 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- pass worst frame 28.22 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.219 | 2.583 | 0.181 |
| main_opaque_pass_3d | 1.454 | 2.059 | 0.044 |
| early prepass | 0.504 | 0.753 | 0.077 |
| lightprobe_irradiance_map | 0.431 | 0.425 | 0.001 |
| bloom | 0.336 | 0.489 | 0.066 |
| atmosphere_luts | 0.328 | 0.520 | 0.003 |
| lightprobe_radiance_map | 0.228 | 0.236 | 0.004 |
| ssao | 0.217 | 0.212 | 0.007 |
| uniform_allocation | 0.182 | 0.186 | 0.017 |
| auxiliary_views | 0.180 | 0.716 | 0.156 |
| water_fft | 0.130 | 0.128 | 0.003 |
| bin_unpacking | 0.127 | 0.140 | 0.012 |
| upscaling | 0.127 | 0.142 | 0.004 |
| clouds_trace | 0.127 | 0.218 | 0.001 |
| atmos_froxel_inject | 0.099 | 0.108 | 0.002 |
| tonemapping | 0.092 | 0.091 | 0.003 |
| water_velocity | 0.087 | 0.085 | 0.002 |
| clustering | 0.066 | 0.069 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.061 | 0.105 | 0.107 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.049 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.025 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.636 | 12.046 | 26.53 | 56599.9 |
| render/busy/ms | 9.094 | 11.481 | 22.61 | 53419.4 |
| main/busy/ms | 8.095 | 9.890 | 17.22 | 47547.3 |
| render/graph/ms | 4.547 | 5.986 | 13.49 | 26711.1 |
| render/present/ms | 0.451 | 0.558 | 17.77 | 2651.4 |
| bigspace/total/ms | 0.414 | 0.626 | 2.06 | 2429.7 |
| terrain/update/ms | 0.290 | 0.368 | 1.52 | 1705.6 |
| stream/drive/ms | 0.256 | 0.443 | 2.17 | 1503.4 |
| terrain/select/ms | 0.255 | 0.323 | 0.87 | 1500.4 |
| controller/feed/ms | 0.230 | 0.303 | 0.67 | 1350.1 |
| far/select/ms | 0.222 | 0.378 | 0.87 | 1305.1 |
| stream/tick/ms | 0.209 | 0.368 | 1.05 | 1226.7 |
| anim/evaluate_cpu/ms | 0.177 | 0.223 | 0.56 | 1042.3 |
| bigspace/low_precision/ms | 0.162 | 0.246 | 1.70 | 951.8 |
| controller/move/ms | 0.143 | 0.267 | 0.85 | 837.9 |
| anim/evaluate/ms | 0.127 | 0.239 | 1.42 | 745.0 |
| bigspace/high_precision/ms | 0.096 | 0.144 | 1.45 | 565.2 |
| render/acquire/ms | 0.090 | 0.113 | 0.46 | 529.0 |
| anim/apply/ms | 0.087 | 0.131 | 1.55 | 509.2 |
| physics/step/ms | 0.049 | 0.087 | 0.95 | 289.2 |
| physics/insert/ms | 0.045 | 0.407 | 1.15 | 265.1 |
| weather/map/ms | 0.026 | 0.036 | 0.36 | 155.1 |
| controller/avoid/ms | 0.012 | 0.018 | 0.59 | 68.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.40 | 60.3 |
| script/update/ms | 0.006 | 0.008 | 0.36 | 37.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.27 | 33.3 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 28.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 26.3 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 24.4 |
| controller/write/ms | 0.004 | 0.006 | 0.37 | 22.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.83 | 16.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.18 | 6.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.15 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
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
| frame/work/ms | ms | 8.75 | 22.61 | 8.06 |
| gpu/timed/ms | ms | 7.51 | 11.10 | 8.06 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 806 | 1420 | 618 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 806 | 1420 | 618 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3394 | 5154 | 3182 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3394 | 5154 | 3182 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 37 | 231 | 28 |
| phases/shadow/bins | count | 3811 | 5503 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9622 | 13.0 k | 5677 |
| phases/shadow/pipelines | count | 113 | 258 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 200 | 476 | 19 |
| physics/pending | count | 0 | 442 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 173 | 173 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 186 | 610 | 164 |
| preprocess/bind_groups_created | count | 186 | 610 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.11 | 36.06 | 29.38 |
| process/mem_usage | GiB | 2.60 | 2.66 | 2.66 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3480 | 3535 | 3535 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.17 | 46.50 | 36.31 |
| system/mem_usage | % | 63.94 | 64.18 | 64.08 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 83 | 271 | 72 |
| views/all/bins | count | 6501 | 8336 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 155 | 296 | 68 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.2 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2617 | 3185 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8164 | 10.2 k | 5677 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8164 | 10.2 k | 5677 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 1090 | 2552 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1222 | 2928 | 0 |
| views/lamp_face/pipelines | count | 74 | 211 | 0 |
| views/lamp_face/work_items | count | 0 | 2625 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2876 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8394 | 13.0 k | 7600 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8560 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 54.49 | 249 | 320.1 k |
| anim/events/count | count | 0.28 | 4 | 1627 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 160 | 4086 |
| assets/Mesh/removed | count | 0.83 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.69 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.81 | 1494 | 4737 |
| physics/inserts | count | 0.48 | 115 | 2798 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5763 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1264 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 567.4 | main/busy/ms 547, surface/advance_cpu_ms 65.61, surface/cell_upload_ms 0.69, stream/drive/ms 0.32, weather/map/ms 0.15, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, script/update/ms 0.03, weather/eval/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.86 | 291.2 | main/busy/ms 580, render/thread/ms 257, render/busy/ms 192, surface/advance_cpu_ms 65.61, render/acquire/ms 62.74, render/graph/ms 4.92, render/present/ms 2.05, stream/drive/ms 1.08, surface/cell_upload_ms 0.69, stream/tick/ms 0.59, terrain/update/ms 0.42, bigspace/total/ms 0.36 |
| warm-up | 0.95 | 87.3 | render/thread/ms 273, render/busy/ms 207, render/acquire/ms 62.97, main/busy/ms 48.59, terrain/update/ms 18.07, render/graph/ms 9.34, render/present/ms 3.48, stream/drive/ms 1.22, stream/tick/ms 0.87, terrain/select/ms 0.57, bigspace/total/ms 0.39, bigspace/high_precision/ms 0.31 |
| warm-up | 1.18 | 94.3 | main/busy/ms 60.34, render/thread/ms 50.57, render/busy/ms 48.65, render/graph/ms 4.31, render/present/ms 1.59, stream/drive/ms 1.55, stream/tick/ms 1.26, terrain/update/ms 0.71, terrain/select/ms 0.59, render/acquire/ms 0.33, bigspace/total/ms 0.32, bigspace/high_precision/ms 0.16 |
| warm-up | 2.74 | 40.8 | main/busy/ms 59.24, render/thread/ms 26.97, render/busy/ms 25.57, render/graph/ms 7.04, stream/drive/ms 1.36, anim/evaluate_cpu/ms 1.35, render/present/ms 1.16, bigspace/total/ms 0.93, physics/insert/ms 0.74, terrain/update/ms 0.70, stream/tick/ms 0.65, terrain/select/ms 0.60 |
| warm-up | 2.78 | 34.3 | main/busy/ms 45.50, render/thread/ms 38.18, render/busy/ms 36.88, render/graph/ms 7.61, bigspace/total/ms 2.60, stream/drive/ms 1.14, render/present/ms 1.04, anim/evaluate_cpu/ms 0.74, stream/tick/ms 0.61, terrain/update/ms 0.59, terrain/select/ms 0.50, bigspace/low_precision/ms 0.47 |
| measure | 55.91 | 28.2 | render/thread/ms 36.71, render/present/ms 18.55, render/busy/ms 17.97, main/busy/ms 16.46, render/graph/ms 9.27, bigspace/total/ms 0.80, stream/drive/ms 0.62, controller/feed/ms 0.54, terrain/update/ms 0.53, stream/tick/ms 0.52, terrain/select/ms 0.47, far/select/ms 0.42 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **79%** (min 60%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.4 W · 73 °C (max 78) · VRAM 4862 MiB · throttled 100% of samples
