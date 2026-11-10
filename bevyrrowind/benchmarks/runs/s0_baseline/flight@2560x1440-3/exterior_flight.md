# `exterior_flight`

- bench_ground: true · build: release · camera: [-85.6, 64.5, 1173.4] -> [-175.6, 2.4, 1053.4] · cell: Bevyrrowind - Seyda Neen (2 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6443 frames over 71.5 s · **median 10.90 ms (92 fps)** · mean 11.10 · p95 13.95 · p99 16.58 (1% low 60 fps) · worst 32.38 ms · 70 spikes over 1.5× median · 47.8 stutters/min (work 59.6/min)
- warm-up: 582 frames over 5.8 s, worst 577.0 ms
- hitches (> 27.3 ms): **1** measured, 4 in warm-up
- timed GPU passes: 8.50 ms of a 11.10 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.90 ms, limit 11 ms
- **FAIL** frame_ms: 57 stutters (> 1.5x moving median of 60) in 1.19 min = 47.82/min, limit 1/min
- pass worst frame 32.38 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.290 | 3.270 | 0.042 |
| dlss_super_resolution | 2.242 | 2.901 | 0.181 |
| early prepass | 0.573 | 0.938 | 0.079 |
| lightprobe_irradiance_map | 0.447 | 0.757 | 0.001 |
| atmosphere_luts | 0.387 | 0.657 | 0.003 |
| bloom | 0.338 | 0.487 | 0.065 |
| lightprobe_radiance_map | 0.227 | 0.236 | 0.004 |
| clouds_trace | 0.226 | 0.266 | 0.001 |
| ssao | 0.218 | 0.212 | 0.007 |
| main_transparent_pass_3d | 0.215 | 0.501 | 0.087 |
| uniform_allocation | 0.155 | 0.292 | 0.014 |
| water_fft | 0.135 | 0.130 | 0.003 |
| upscaling | 0.127 | 0.142 | 0.004 |
| tonemapping | 0.094 | 0.093 | 0.003 |
| atmos_froxel_inject | 0.094 | 0.093 | 0.001 |
| bin_unpacking | 0.092 | 0.106 | 0.010 |
| water_velocity | 0.079 | 0.095 | 0.001 |
| clustering | 0.072 | 0.074 | 0.021 |
| clouds_composite | 0.067 | 0.068 | 0.002 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| clouds_resolve | 0.037 | 0.046 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.041 | 0.005 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.031 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.031 | 0.007 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.016 | 0.017 | 0.004 |
| auxiliary_views | 0.014 | 0.003 | 0.013 |
| weather_map | 0.014 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.016 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.200 | 11.835 | 30.04 | 59275.4 |
| render/busy/ms | 8.522 | 11.187 | 22.99 | 54905.6 |
| main/busy/ms | 7.769 | 9.244 | 16.95 | 50054.3 |
| render/graph/ms | 3.918 | 4.824 | 11.71 | 25246.6 |
| render/present/ms | 0.471 | 0.619 | 20.39 | 3035.5 |
| bigspace/total/ms | 0.440 | 0.883 | 3.49 | 2835.2 |
| stream/drive/ms | 0.319 | 0.878 | 10.24 | 2053.3 |
| terrain/update/ms | 0.257 | 0.477 | 2.12 | 1657.2 |
| controller/feed/ms | 0.228 | 0.348 | 1.02 | 1467.6 |
| far/select/ms | 0.222 | 0.411 | 1.14 | 1429.5 |
| render/acquire/ms | 0.207 | 1.176 | 3.38 | 1334.3 |
| terrain/select/ms | 0.193 | 0.249 | 0.96 | 1244.1 |
| stream/tick/ms | 0.187 | 0.331 | 1.08 | 1207.0 |
| bigspace/low_precision/ms | 0.159 | 0.400 | 2.83 | 1021.4 |
| controller/move/ms | 0.158 | 0.406 | 1.27 | 1015.2 |
| anim/evaluate_cpu/ms | 0.130 | 0.213 | 0.54 | 836.5 |
| bigspace/high_precision/ms | 0.119 | 0.268 | 1.93 | 766.4 |
| anim/apply/ms | 0.105 | 0.194 | 1.44 | 674.1 |
| anim/evaluate/ms | 0.100 | 0.221 | 1.37 | 642.5 |
| physics/step/ms | 0.072 | 0.190 | 1.64 | 466.6 |
| stream/despawn/ms | 0.044 | 0.421 | 1.71 | 284.2 |
| stream/spawn/ms | 0.033 | 0.257 | 1.47 | 212.0 |
| weather/map/ms | 0.027 | 0.035 | 0.20 | 170.8 |
| physics/insert/ms | 0.017 | 0.086 | 0.82 | 111.7 |
| controller/avoid/ms | 0.011 | 0.018 | 0.44 | 72.9 |
| weather/eval/ms | 0.010 | 0.015 | 0.34 | 64.5 |
| script/update/ms | 0.007 | 0.010 | 0.37 | 43.3 |
| stream/deactivate/ms | 0.006 | 0.000 | 0.87 | 39.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 33.1 |
| script/ui/ms | 0.005 | 0.007 | 0.14 | 32.1 |
| stream/activate/ms | 0.005 | 0.000 | 0.52 | 30.5 |
| controller/write/ms | 0.003 | 0.006 | 0.46 | 22.4 |
| stream/upload/ms | 0.001 | 0.013 | 0.09 | 9.3 |
| shapes/activate/ms | 0.001 | 0.002 | 0.05 | 6.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.3 |
| surface/cell_upload_ms | 0.001 | 0.000 | 0.13 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.8 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 608 | 778 | 471 |
| anim/characters/count | count | 609 | 782 | 471 |
| anim/evaluated/count | count | 19 | 80 | 14 |
| controller/movers/count | count | 19 | 31 | 14 |
| ecs/entities | count | 60.2 k | 72.5 k | 53.0 k |
| far/entities | count | 2681 | 3461 | 2126 |
| far/forced | count | 0 | 8 | 0 |
| far/impostors | count | 61.0 k | 65.7 k | 55.8 k |
| far/lights | count | 7115 | 7757 | 6575 |
| far/nodes/l1 | count | 1100 | 1439 | 857 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 227.79 M | 173.66 M |
| far/triangles | count | 5.04 M | 5.72 M | 4.26 M |
| frame/work/ms | ms | 8.83 | 22.99 | 7.76 |
| gpu/timed/ms | ms | 8.43 | 12.42 | 7.76 |
| mesh_allocator_allocations |  meshes | 15.8 k | 19.2 k | 16.9 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 537.07 M | 645.72 M | 645.72 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d/bins | count | 321 | 410 | 39 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 484 | 907 | 170 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/alpha_mask3d_prepass/bins | count | 321 | 410 | 39 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 484 | 907 | 170 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 5 |
| phases/opaque3d/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d/bins | count | 1143 | 1966 | 536 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3004 | 4179 | 1239 |
| phases/opaque3d/pipelines | count | 13 | 17 | 10 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 18 | 10 |
| phases/opaque3d_prepass/bins | count | 1143 | 1966 | 536 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3004 | 4179 | 1239 |
| phases/opaque3d_prepass/pipelines | count | 13 | 17 | 10 |
| phases/shadow/batch_sets | count | 19 | 164 | 16 |
| phases/shadow/bins | count | 1572 | 2774 | 1788 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4186 | 6836 | 4573 |
| phases/shadow/pipelines | count | 19 | 165 | 16 |
| physics/awake | count | 0 | 4 | 0 |
| physics/bodies | count | 1892 | 2692 | 1955 |
| physics/items | count | 12 | 106 | 106 |
| physics/pending | count | 0 | 317 | 0 |
| physics/terrain/chunks | count | 68 | 80 | 68 |
| pipelines/total | count | 171 | 180 | 180 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 142 | 445 | 112 |
| preprocess/bind_groups_created | count | 142 | 445 | 112 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.52 | 41.90 | 23.77 |
| process/mem_usage | GiB | 2.69 | 2.91 | 2.90 |
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
| stream/activate/queue | count | 0 | 2 | 0 |
| stream/assets/queue | count | 0 | 16 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 53 | 0 |
| stream/fetch/queue | count | 0 | 170 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.02 |
| stream/model_queue | count | 0 | 17 | 0 |
| stream/models | count | 201 | 261 | 201 |
| stream/oldest_request/s | s | 0 | 0.51 | 0 |
| stream/page_cache/bytes | bytes | 26.32 M | 28.52 M | 28.52 M |
| stream/requests | count | 0 | 171 | 0 |
| stream/resident_cells | count | 1124 | 1481 | 872 |
| stream/spawn/queue | count | 0 | 7 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.16 | 49.10 | 31.83 |
| system/mem_usage | % | 64.13 | 64.93 | 64.91 |
| terrain/patches | count | 349 | 496 | 319 |
| views/all/batch_sets | count | 61 | 194 | 46 |
| views/all/bins | count | 4510 | 6728 | 2938 |
| views/all/count | count | 4 | 42 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.4 k | 15.9 k | 7391 |
| views/all/pipelines | count | 57 | 191 | 46 |
| views/all/pipelines_distinct | count | 52 | 61 | 44 |
| views/all/work_items | count | 11.6 k | 16.2 k | 7420 |
| views/cascade/batch_sets | count | 19 | 28 | 16 |
| views/cascade/bins | count | 1563 | 2705 | 1788 |
| views/cascade/count | count | 3 | 6 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4150 | 6836 | 4573 |
| views/cascade/pipelines | count | 19 | 28 | 16 |
| views/cascade/work_items | count | 4150 | 6836 | 4573 |
| views/lamp_face/batch_sets | count | 0 | 139 | 0 |
| views/lamp_face/bins | count | 0 | 1532 | 0 |
| views/lamp_face/count | count | 0 | 36 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2256 | 0 |
| views/lamp_face/pipelines | count | 0 | 139 | 0 |
| views/lamp_face/work_items | count | 0 | 2229 | 0 |
| views/main/batch_sets | count | 42 | 50 | 30 |
| views/main/bins | count | 2890 | 4442 | 1150 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7166 | 10.2 k | 2818 |
| views/main/pipelines | count | 38 | 46 | 30 |
| views/main/work_items | count | 7360 | 10.4 k | 2847 |
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
| anim/bones_written/count | count | 9.51 | 211 | 61.3 k |
| anim/events/count | count | 0.19 | 6 | 1225 |
| assets/Image/added | count | 0.14 | 57 | 900 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.02 | 30 | 122 |
| assets/Mesh/added | count | 6.28 | 461 | 40.4 k |
| assets/Mesh/removed | count | 10.27 | 922 | 66.2 k |
| assets/StandardMaterial/added | count | 0.18 | 75 | 1172 |
| assets/StandardMaterial/modified | count | 5.29 | 33 | 34.1 k |
| assets/StandardMaterial/removed | count | 0.07 | 36 | 470 |
| assets/WorldAsset/added | count | 0.01 | 9 | 83 |
| assets/WorldAsset/removed | count | 0.02 | 10 | 124 |
| ecs/entities_delta | count | 3.13 | 2043 | 20.2 k |
| physics/inserts | count | 2.27 | 137 | 14.6 k |
| physics/rebases | count | 0.00 | 1 | 7 |
| physics/removals | count | 2.20 | 336 | 14.2 k |
| physics/steps | count | 0.67 | 2 | 4291 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 5 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.21 | 23 | 1345 |
| stream/deactivate/items | count | 0.21 | 21 | 1345 |
| stream/despawn/items | count | 1.08 | 315 | 6982 |
| stream/fetch/items | count | 0.23 | 8 | 1465 |
| stream/shift_guard/frames | count | 0.01 | 1 | 40 |
| stream/spawn/items | count | 0.71 | 29 | 4586 |
| stream/upload/bytes | bytes | 76.9 k | 3.54 M | 495.17 M |
| stream/upload/items | count | 0.20 | 8 | 1288 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.04 | 1 | 268 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 17.8 k | 1.21 M | 114.84 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0.00 | 1 | 20 |

## Hitches (> 27.3 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.0 | main/busy/ms 557, surface/advance_cpu_ms 68.67, surface/cell_upload_ms 0.86, stream/drive/ms 0.33, weather/map/ms 0.12, stream/tick/ms 0.09, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.03, weather/eval/ms 0.02, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.90 | 320.0 | main/busy/ms 590, render/thread/ms 286, render/busy/ms 219, surface/advance_cpu_ms 68.67, render/acquire/ms 65.44, render/graph/ms 4.84, render/present/ms 1.93, stream/drive/ms 1.06, surface/cell_upload_ms 0.86, stream/tick/ms 0.62, terrain/update/ms 0.37, terrain/select/ms 0.23 |
| warm-up | 0.95 | 57.9 | render/thread/ms 303, render/busy/ms 234, render/acquire/ms 65.57, main/busy/ms 48.22, terrain/update/ms 18.68, render/graph/ms 9.74, render/present/ms 3.37, stream/drive/ms 1.04, stream/tick/ms 0.80, terrain/select/ms 0.46, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.12 |
| warm-up | 1.19 | 97.2 | main/busy/ms 60.00, render/thread/ms 55.82, render/busy/ms 53.67, render/graph/ms 4.06, render/present/ms 1.79, stream/drive/ms 1.15, stream/tick/ms 0.88, terrain/update/ms 0.54, terrain/select/ms 0.41, render/acquire/ms 0.37, bigspace/total/ms 0.24, weather/map/ms 0.10 |
| measure | 43.51 | 32.4 | render/thread/ms 41.65, render/present/ms 20.99, render/busy/ms 20.45, main/busy/ms 16.30, render/graph/ms 9.63, bigspace/total/ms 0.87, controller/feed/ms 0.66, far/select/ms 0.60, stream/drive/ms 0.50, terrain/update/ms 0.45, stream/tick/ms 0.42, terrain/select/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground --resolution 2560x1440 --bench-out target/perf\ab_main\flight@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (279 samples): GPU load **86%** (min 59%) · 2580 MHz (min 2580) · memory 8001 MHz · 84.5 W · 76 °C (max 78) · VRAM 4874 MiB · throttled 100% of samples
