# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6144 frames over 65.6 s · **median 10.26 ms (97 fps)** · mean 10.68 · p95 13.49 · p99 15.59 (1% low 64 fps) · worst 29.48 ms · 67 spikes over 1.5× median · 14.6 stutters/min (work 13.7/min)
- warm-up: 407 frames over 5.4 s, worst 509.3 ms
- hitches (> 25.7 ms): **3** measured, 5 in warm-up
- timed GPU passes: 7.14 ms of a 10.68 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.26 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- pass worst frame 29.48 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.224 | 2.640 | 0.177 |
| main_opaque_pass_3d | 1.160 | 1.710 | 0.045 |
| early prepass | 0.516 | 0.835 | 0.079 |
| lightprobe_irradiance_map | 0.435 | 0.722 | 0.001 |
| bloom | 0.342 | 0.497 | 0.065 |
| atmosphere_luts | 0.334 | 0.492 | 0.003 |
| auxiliary_views | 0.284 | 1.127 | 0.223 |
| lightprobe_radiance_map | 0.223 | 0.234 | 0.004 |
| ssao | 0.222 | 0.219 | 0.007 |
| uniform_allocation | 0.156 | 0.157 | 0.015 |
| upscaling | 0.133 | 0.157 | 0.004 |
| water_fft | 0.131 | 0.129 | 0.002 |
| bin_unpacking | 0.107 | 0.118 | 0.011 |
| tonemapping | 0.096 | 0.099 | 0.003 |
| water_velocity | 0.086 | 0.085 | 0.002 |
| atmos_froxel_inject | 0.082 | 0.085 | 0.001 |
| clustering | 0.070 | 0.070 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.096 | 0.108 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| clouds_trace | 0.053 | 0.072 | 0.001 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| lightprobe_copy | 0.017 | 0.018 | 0.004 |
| lightprobe_downsampling_first_pass | 0.017 | 0.018 | 0.001 |
| night_sky | 0.016 | 0.035 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.199 | 11.987 | 27.98 | 56517.5 |
| render/busy/ms | 8.664 | 11.413 | 22.11 | 53232.0 |
| main/busy/ms | 7.992 | 9.791 | 14.83 | 49102.4 |
| render/graph/ms | 4.416 | 6.364 | 14.34 | 27133.5 |
| render/present/ms | 0.444 | 0.539 | 17.33 | 2729.8 |
| bigspace/total/ms | 0.442 | 0.739 | 2.32 | 2718.4 |
| terrain/update/ms | 0.293 | 0.375 | 1.75 | 1801.0 |
| terrain/select/ms | 0.258 | 0.328 | 0.74 | 1584.7 |
| stream/drive/ms | 0.249 | 0.419 | 2.00 | 1530.8 |
| controller/feed/ms | 0.234 | 0.305 | 0.76 | 1436.2 |
| far/select/ms | 0.216 | 0.365 | 0.97 | 1324.6 |
| stream/tick/ms | 0.204 | 0.355 | 0.83 | 1252.4 |
| bigspace/low_precision/ms | 0.178 | 0.319 | 1.45 | 1095.4 |
| anim/evaluate/ms | 0.169 | 0.294 | 1.82 | 1040.8 |
| anim/evaluate_cpu/ms | 0.163 | 0.203 | 0.65 | 1002.9 |
| controller/move/ms | 0.134 | 0.267 | 0.87 | 823.3 |
| bigspace/high_precision/ms | 0.109 | 0.211 | 1.97 | 668.0 |
| render/acquire/ms | 0.090 | 0.115 | 0.49 | 555.7 |
| anim/apply/ms | 0.081 | 0.116 | 1.45 | 498.1 |
| physics/step/ms | 0.046 | 0.083 | 0.78 | 280.7 |
| physics/insert/ms | 0.039 | 0.376 | 1.12 | 238.4 |
| weather/map/ms | 0.017 | 0.022 | 0.08 | 102.5 |
| controller/avoid/ms | 0.012 | 0.018 | 0.14 | 72.5 |
| weather/eval/ms | 0.009 | 0.013 | 0.16 | 58.3 |
| script/update/ms | 0.006 | 0.008 | 0.39 | 39.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 31.9 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 27.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 27.5 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 25.6 |
| controller/write/ms | 0.004 | 0.007 | 0.16 | 24.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.77 | 16.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.37 | 6.4 |
| shapes/activate/ms | 0.001 | 0.002 | 0.08 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| ecs/entities | count | 59.5 k | 68.3 k | 59.3 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.39 | 22.11 | 7.56 |
| gpu/timed/ms | ms | 7.15 | 10.80 | 7.29 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 799 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d/bins | count | 1291 | 1400 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3365 | 5146 | 3181 |
| phases/opaque3d/pipelines | count | 14 | 15 | 15 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1400 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3365 | 5146 | 3181 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 15 |
| phases/shadow/batch_sets | count | 24 | 205 | 18 |
| phases/shadow/bins | count | 2271 | 4023 | 1122 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3630 | 6900 | 2427 |
| phases/shadow/pipelines | count | 100 | 220 | 18 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 198 | 476 | 19 |
| physics/pending | count | 0 | 244 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 166 | 168 | 168 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 157 | 561 | 147 |
| preprocess/bind_groups_created | count | 157 | 561 | 147 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.25 | 38.46 | 17.31 |
| process/mem_usage | GiB | 2.54 | 2.59 | 2.59 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3459 | 3519 | 3519 |
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
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.22 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.22 | 47.52 | 30.17 |
| system/mem_usage | % | 63.27 | 63.45 | 63.45 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 69 | 247 | 64 |
| views/all/bins | count | 4975 | 6948 | 4614 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.5 k | 19.2 k | 10.0 k |
| views/all/pipelines | count | 142 | 262 | 60 |
| views/all/pipelines_distinct | count | 60 | 69 | 56 |
| views/all/work_items | count | 11.0 k | 17.4 k | 10.3 k |
| views/cascade/batch_sets | count | 22 | 28 | 18 |
| views/cascade/bins | count | 1183 | 1502 | 1122 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2736 | 4295 | 2427 |
| views/cascade/pipelines | count | 22 | 28 | 18 |
| views/cascade/work_items | count | 2736 | 4295 | 2427 |
| views/lamp_face/batch_sets | count | 0 | 182 | 0 |
| views/lamp_face/bins | count | 1120 | 2822 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1294 | 3099 | 0 |
| views/lamp_face/pipelines | count | 75 | 196 | 0 |
| views/lamp_face/work_items | count | 0 | 2764 | 0 |
| views/main/batch_sets | count | 44 | 48 | 46 |
| views/main/bins | count | 2885 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8332 | 13.1 k | 7596 |
| views/main/pipelines | count | 42 | 44 | 42 |
| views/main/work_items | count | 8500 | 13.3 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 45.84 | 266 | 281.6 k |
| anim/events/count | count | 0.18 | 4 | 1104 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.67 | 160 | 4086 |
| assets/Mesh/removed | count | 0.80 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 459 |
| assets/StandardMaterial/modified | count | 3.53 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.78 | 1299 | 4787 |
| physics/inserts | count | 0.40 | 84 | 2435 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 129 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 6 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5510 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1208 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.7 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 509.3 | main/busy/ms 489, surface/advance_cpu_ms 0.75, stream/drive/ms 0.32, surface/cell_upload_ms 0.31, stream/tick/ms 0.09, bigspace/total/ms 0.06, weather/map/ms 0.05, weather/eval/ms 0.03, bigspace/high_precision/ms 0.03, script/update/ms 0.02, script/input/ms 0.02, script/ui/ms 0.01 |
| warm-up | 0.75 | 238.3 | main/busy/ms 520, render/thread/ms 200, render/busy/ms 134, render/acquire/ms 63.98, render/graph/ms 4.76, render/present/ms 2.06, stream/drive/ms 1.28, surface/advance_cpu_ms 0.75, stream/tick/ms 0.70, terrain/update/ms 0.38, surface/cell_upload_ms 0.31, terrain/select/ms 0.25 |
| warm-up | 0.88 | 134.2 | render/thread/ms 216, render/busy/ms 149, render/acquire/ms 64.23, main/busy/ms 47.53, terrain/update/ms 17.87, render/graph/ms 9.67, render/present/ms 3.25, stream/drive/ms 1.46, stream/tick/ms 1.01, terrain/select/ms 0.56, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.18 |
| warm-up | 1.13 | 89.5 | main/busy/ms 71.59, render/thread/ms 56.28, render/busy/ms 54.58, render/graph/ms 4.00, stream/drive/ms 2.16, stream/tick/ms 1.62, render/present/ms 1.41, terrain/update/ms 0.75, terrain/select/ms 0.61, bigspace/total/ms 0.30, render/acquire/ms 0.29, bigspace/high_precision/ms 0.14 |
| warm-up | 5.40 | 27.8 | main/busy/ms 42.85, render/thread/ms 30.94, render/busy/ms 29.78, render/graph/ms 9.52, bigspace/total/ms 1.28, anim/evaluate_cpu/ms 1.06, render/present/ms 0.95, anim/evaluate/ms 0.82, bigspace/low_precision/ms 0.73, terrain/update/ms 0.54, controller/feed/ms 0.52, physics/insert/ms 0.51 |
| measure | 10.97 | 26.6 | render/thread/ms 37.73, render/busy/ms 23.18, main/busy/ms 19.38, render/present/ms 14.39, render/graph/ms 12.69, bigspace/total/ms 0.87, controller/feed/ms 0.68, far/select/ms 0.59, terrain/update/ms 0.57, terrain/select/ms 0.51, stream/drive/ms 0.45, anim/evaluate_cpu/ms 0.41 |
| measure | 25.93 | 26.3 | render/thread/ms 34.36, render/busy/ms 19.05, main/busy/ms 18.20, render/present/ms 15.08, render/graph/ms 9.12, bigspace/total/ms 0.91, far/select/ms 0.85, terrain/update/ms 0.62, stream/drive/ms 0.60, terrain/select/ms 0.54, stream/tick/ms 0.53, controller/feed/ms 0.51 |
| measure | 55.93 | 29.5 | render/thread/ms 37.13, render/busy/ms 19.16, render/present/ms 17.76, main/busy/ms 17.42, render/graph/ms 8.97, bigspace/total/ms 0.83, far/select/ms 0.66, controller/feed/ms 0.65, terrain/update/ms 0.57, controller/move/ms 0.50, terrain/select/ms 0.50, stream/drive/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 4%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **76%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.0 W · 72 °C (max 77) · VRAM 4856 MiB · throttled 100% of samples
