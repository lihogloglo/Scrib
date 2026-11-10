# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5909 frames over 65.6 s · **median 10.30 ms (97 fps)** · mean 11.11 · p95 15.16 · p99 16.66 (1% low 60 fps) · worst 32.33 ms · 229 spikes over 1.5× median · 12.8 stutters/min (work 11.9/min)
- warm-up: 341 frames over 4.7 s, worst 513.4 ms
- hitches (> 25.7 ms): **1** measured, 9 in warm-up
- timed GPU passes: 7.25 ms of a 11.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.30 ms, limit 11 ms
- **FAIL** frame_ms: 14 stutters (> 1.5x moving median of 60) in 1.09 min = 12.80/min, limit 1/min
- pass worst frame 32.33 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.208 | 2.573 | 0.182 |
| main_opaque_pass_3d | 1.165 | 1.684 | 0.045 |
| early prepass | 0.522 | 0.779 | 0.077 |
| lightprobe_irradiance_map | 0.440 | 0.753 | 0.001 |
| auxiliary_views | 0.399 | 1.582 | 0.315 |
| bloom | 0.341 | 0.500 | 0.065 |
| atmosphere_luts | 0.331 | 0.478 | 0.003 |
| lightprobe_radiance_map | 0.228 | 0.237 | 0.004 |
| ssao | 0.221 | 0.216 | 0.007 |
| uniform_allocation | 0.157 | 0.288 | 0.016 |
| upscaling | 0.133 | 0.156 | 0.004 |
| water_fft | 0.129 | 0.129 | 0.002 |
| bin_unpacking | 0.107 | 0.119 | 0.011 |
| tonemapping | 0.093 | 0.093 | 0.003 |
| water_velocity | 0.086 | 0.085 | 0.001 |
| atmos_froxel_inject | 0.081 | 0.084 | 0.001 |
| clustering | 0.071 | 0.070 | 0.019 |
| main_transparent_pass_3d | 0.064 | 0.095 | 0.108 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| clouds_trace | 0.053 | 0.071 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.002 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.029 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.018 | 0.018 | 0.004 |
| lightprobe_downsampling_first_pass | 0.016 | 0.017 | 0.001 |
| night_sky | 0.016 | 0.031 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.611 | 13.635 | 30.39 | 56793.7 |
| render/busy/ms | 9.072 | 13.038 | 23.03 | 53607.3 |
| main/busy/ms | 8.071 | 10.226 | 14.76 | 47691.9 |
| render/graph/ms | 4.689 | 7.445 | 14.48 | 27709.9 |
| render/present/ms | 0.449 | 0.553 | 20.92 | 2651.0 |
| bigspace/total/ms | 0.437 | 0.733 | 2.95 | 2582.9 |
| terrain/update/ms | 0.291 | 0.372 | 1.75 | 1717.7 |
| terrain/select/ms | 0.256 | 0.326 | 0.88 | 1511.2 |
| stream/drive/ms | 0.246 | 0.420 | 2.04 | 1454.3 |
| controller/feed/ms | 0.236 | 0.304 | 0.67 | 1392.2 |
| far/select/ms | 0.221 | 0.376 | 0.84 | 1308.0 |
| stream/tick/ms | 0.201 | 0.345 | 0.79 | 1186.0 |
| bigspace/low_precision/ms | 0.178 | 0.292 | 2.52 | 1051.3 |
| anim/evaluate_cpu/ms | 0.168 | 0.213 | 0.56 | 993.7 |
| anim/evaluate/ms | 0.167 | 0.283 | 1.87 | 986.2 |
| controller/move/ms | 0.145 | 0.282 | 0.69 | 855.1 |
| bigspace/high_precision/ms | 0.103 | 0.193 | 1.21 | 611.2 |
| render/acquire/ms | 0.091 | 0.115 | 0.59 | 535.5 |
| anim/apply/ms | 0.085 | 0.130 | 1.37 | 504.0 |
| physics/step/ms | 0.049 | 0.086 | 0.85 | 289.6 |
| physics/insert/ms | 0.038 | 0.376 | 1.35 | 223.2 |
| weather/map/ms | 0.017 | 0.022 | 0.20 | 99.2 |
| controller/avoid/ms | 0.012 | 0.019 | 0.45 | 70.1 |
| weather/eval/ms | 0.009 | 0.013 | 0.36 | 55.5 |
| script/update/ms | 0.006 | 0.009 | 0.37 | 38.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 31.6 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 28.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.19 | 27.5 |
| script/ui/ms | 0.004 | 0.006 | 0.24 | 24.3 |
| controller/write/ms | 0.004 | 0.006 | 0.12 | 21.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.69 | 15.8 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 6.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.34 | 23.03 | 7.69 |
| gpu/timed/ms | ms | 7.24 | 10.99 | 7.54 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 202 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 777 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 202 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 777 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d/bins | count | 1290 | 1400 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3336 | 5109 | 3182 |
| phases/opaque3d/pipelines | count | 14 | 15 | 15 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1400 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3336 | 5109 | 3182 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 15 |
| phases/shadow/batch_sets | count | 24 | 213 | 19 |
| phases/shadow/bins | count | 2238 | 4300 | 1137 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3800 | 7301 | 2434 |
| phases/shadow/pipelines | count | 93 | 225 | 19 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 380 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 166 | 168 | 168 |
| pipelines/waiting | count | 0 | 25 | 0 |
| preprocess/bind_groups | count | 159 | 577 | 149 |
| preprocess/bind_groups_created | count | 159 | 577 | 149 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.36 | 43.75 | 23.27 |
| process/mem_usage | GiB | 2.59 | 2.64 | 2.64 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3480 | 3540 | 3540 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 431 | 530 | 398 |
| statics/materials_bound | count | 431 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.36 | 50.15 | 29.24 |
| system/mem_usage | % | 63.35 | 63.54 | 63.47 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 70 | 255 | 65 |
| views/all/bins | count | 4822 | 7164 | 4629 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.5 k | 19.1 k | 10.0 k |
| views/all/pipelines | count | 135 | 265 | 61 |
| views/all/pipelines_distinct | count | 58 | 69 | 57 |
| views/all/work_items | count | 11.0 k | 17.5 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 27 | 19 |
| views/cascade/bins | count | 1174 | 1517 | 1137 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2738 | 4309 | 2434 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 2738 | 4309 | 2434 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 1055 | 3074 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1200 | 3348 | 0 |
| views/lamp_face/pipelines | count | 70 | 199 | 0 |
| views/lamp_face/work_items | count | 0 | 2885 | 0 |
| views/main/batch_sets | count | 46 | 48 | 46 |
| views/main/bins | count | 2896 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8226 | 13.0 k | 7598 |
| views/main/pipelines | count | 42 | 44 | 42 |
| views/main/work_items | count | 8389 | 13.2 k | 7906 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.98 | 221 | 313.0 k |
| anim/events/count | count | 0.19 | 3 | 1119 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.69 | 160 | 4086 |
| assets/Mesh/removed | count | 0.83 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 459 |
| assets/StandardMaterial/modified | count | 3.67 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 0.80 | 1328 | 4738 |
| physics/inserts | count | 0.45 | 147 | 2678 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.67 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 6 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 23 | 378 |
| stream/upload/bytes | bytes | 5729 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1256 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.7 ms; slowest 10, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 513.4 | main/busy/ms 492, surface/advance_cpu_ms 0.73, stream/drive/ms 0.31, surface/cell_upload_ms 0.29, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.83 | 313.1 | main/busy/ms 526, render/thread/ms 277, render/busy/ms 194, render/acquire/ms 80.09, render/graph/ms 5.44, render/present/ms 2.95, stream/drive/ms 1.35, stream/tick/ms 0.76, surface/advance_cpu_ms 0.73, terrain/update/ms 0.60, terrain/select/ms 0.40, bigspace/total/ms 0.34 |
| warm-up | 0.97 | 148.2 | render/thread/ms 297, render/busy/ms 213, render/acquire/ms 80.26, main/busy/ms 52.64, terrain/update/ms 20.38, render/graph/ms 10.28, render/present/ms 4.18, stream/drive/ms 1.44, stream/tick/ms 1.00, terrain/select/ms 0.70, bigspace/total/ms 0.38, bigspace/high_precision/ms 0.30 |
| warm-up | 1.04 | 27.1 | render/thread/ms 44.17, render/acquire/ms 28.88, main/busy/ms 18.98, render/busy/ms 13.52, terrain/update/ms 9.84, render/graph/ms 4.43, render/present/ms 1.77, stream/drive/ms 0.96, stream/tick/ms 0.82, terrain/select/ms 0.58, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.10 |
| warm-up | 1.07 | 29.5 | render/thread/ms 55.15, render/acquire/ms 41.97, main/busy/ms 15.89, render/busy/ms 12.05, terrain/update/ms 9.04, render/graph/ms 3.32, stream/drive/ms 1.55, stream/tick/ms 1.37, render/present/ms 1.13, terrain/select/ms 0.54, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.09 |
| warm-up | 1.09 | 27.0 | render/thread/ms 53.88, render/busy/ms 30.80, render/acquire/ms 21.75, main/busy/ms 19.94, terrain/update/ms 4.78, render/graph/ms 3.10, stream/drive/ms 1.68, stream/tick/ms 1.47, render/present/ms 1.33, terrain/select/ms 0.60, bigspace/total/ms 0.33, bigspace/high_precision/ms 0.20 |
| warm-up | 1.15 | 30.7 | main/busy/ms 80.98, render/thread/ms 49.41, render/busy/ms 38.46, render/acquire/ms 9.41, render/graph/ms 5.30, stream/drive/ms 3.49, stream/tick/ms 3.01, render/present/ms 1.54, terrain/update/ms 1.04, terrain/select/ms 0.83, script/input/ms 0.23, bigspace/total/ms 0.23 |
| warm-up | 1.25 | 105.4 | render/thread/ms 99.07, render/busy/ms 88.32, main/busy/ms 81.98, render/acquire/ms 9.26, render/graph/ms 8.52, stream/drive/ms 4.43, stream/tick/ms 3.40, render/present/ms 1.48, terrain/update/ms 0.88, terrain/select/ms 0.72, bigspace/total/ms 0.27, bigspace/high_precision/ms 0.14 |
| warm-up | 1.44 | 26.4 | render/thread/ms 39.77, render/busy/ms 38.44, main/busy/ms 22.15, terrain/update/ms 5.23, render/graph/ms 4.83, stream/drive/ms 3.26, stream/spawn/ms 1.24, render/present/ms 1.14, stream/tick/ms 0.72, terrain/select/ms 0.49, stream/activate/ms 0.33, bigspace/total/ms 0.25 |
| measure | 61.08 | 32.3 | render/thread/ms 39.81, render/present/ms 21.36, render/busy/ms 18.20, main/busy/ms 16.22, render/graph/ms 8.78, bigspace/total/ms 0.80, terrain/update/ms 0.76, terrain/select/ms 0.69, controller/feed/ms 0.67, stream/drive/ms 0.58, controller/move/ms 0.53, stream/tick/ms 0.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **73%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.5 W · 72 °C (max 77) · VRAM 4863 MiB · throttled 100% of samples
