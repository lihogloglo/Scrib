# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6051 frames over 65.7 s · **median 10.17 ms (98 fps)** · mean 10.85 · p95 14.81 · p99 16.39 (1% low 61 fps) · worst 30.94 ms · 211 spikes over 1.5× median · 18.3 stutters/min (work 23.8/min)
- warm-up: 392 frames over 4.7 s, worst 511.0 ms
- hitches (> 25.4 ms): **1** measured, 6 in warm-up
- timed GPU passes: 7.15 ms of a 10.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.17 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- pass worst frame 30.94 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.208 | 2.577 | 0.179 |
| main_opaque_pass_3d | 1.152 | 1.682 | 0.044 |
| early prepass | 0.518 | 0.806 | 0.077 |
| lightprobe_irradiance_map | 0.438 | 0.751 | 0.001 |
| bloom | 0.341 | 0.643 | 0.065 |
| atmosphere_luts | 0.331 | 0.483 | 0.003 |
| auxiliary_views | 0.327 | 1.260 | 0.257 |
| lightprobe_radiance_map | 0.226 | 0.239 | 0.004 |
| ssao | 0.221 | 0.216 | 0.007 |
| uniform_allocation | 0.154 | 0.154 | 0.015 |
| upscaling | 0.135 | 0.167 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.003 |
| bin_unpacking | 0.108 | 0.118 | 0.011 |
| tonemapping | 0.094 | 0.094 | 0.003 |
| water_velocity | 0.085 | 0.085 | 0.001 |
| atmos_froxel_inject | 0.080 | 0.085 | 0.001 |
| clustering | 0.069 | 0.070 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.059 | 0.095 | 0.109 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.053 | 0.002 |
| clouds_trace | 0.053 | 0.073 | 0.001 |
| clouds_resolve | 0.036 | 0.040 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.038 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.029 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.016 | 0.017 | 0.001 |
| night_sky | 0.016 | 0.037 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.005 |
| lightprobe_copy | 0.015 | 0.018 | 0.004 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.003 |
| blas_build | 0.001 | 0.001 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.345 | 13.198 | 29.20 | 56547.6 |
| render/busy/ms | 8.808 | 12.591 | 23.41 | 53299.2 |
| main/busy/ms | 7.956 | 9.999 | 15.10 | 48143.2 |
| render/graph/ms | 4.534 | 7.019 | 14.59 | 27435.9 |
| render/present/ms | 0.446 | 0.548 | 17.84 | 2698.5 |
| bigspace/total/ms | 0.440 | 0.791 | 1.79 | 2660.7 |
| terrain/update/ms | 0.291 | 0.369 | 1.65 | 1761.5 |
| terrain/select/ms | 0.256 | 0.325 | 0.85 | 1549.4 |
| stream/drive/ms | 0.248 | 0.420 | 2.58 | 1503.4 |
| far/select/ms | 0.221 | 0.371 | 0.99 | 1334.6 |
| controller/feed/ms | 0.220 | 0.286 | 0.64 | 1329.6 |
| stream/tick/ms | 0.202 | 0.350 | 0.83 | 1222.3 |
| bigspace/low_precision/ms | 0.174 | 0.335 | 1.32 | 1052.8 |
| anim/evaluate_cpu/ms | 0.171 | 0.217 | 0.72 | 1036.5 |
| controller/move/ms | 0.154 | 0.427 | 0.94 | 931.0 |
| anim/evaluate/ms | 0.142 | 0.242 | 1.57 | 862.0 |
| bigspace/high_precision/ms | 0.110 | 0.232 | 1.49 | 667.4 |
| render/acquire/ms | 0.091 | 0.117 | 0.54 | 549.8 |
| anim/apply/ms | 0.083 | 0.132 | 1.45 | 500.3 |
| physics/step/ms | 0.045 | 0.081 | 0.59 | 272.3 |
| physics/insert/ms | 0.041 | 0.387 | 0.95 | 245.5 |
| weather/map/ms | 0.016 | 0.021 | 0.34 | 99.4 |
| controller/avoid/ms | 0.011 | 0.017 | 0.41 | 65.9 |
| weather/eval/ms | 0.010 | 0.013 | 0.19 | 58.7 |
| script/update/ms | 0.006 | 0.009 | 0.15 | 39.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.13 | 31.2 |
| script/input/ms | 0.005 | 0.006 | 0.41 | 29.0 |
| stream/spawn/ms | 0.004 | 0.000 | 1.14 | 27.1 |
| script/ui/ms | 0.004 | 0.006 | 0.35 | 25.2 |
| controller/write/ms | 0.004 | 0.006 | 0.07 | 21.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.76 | 16.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.10 | 6.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 13 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
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
| frame/work/ms | ms | 8.24 | 23.41 | 7.76 |
| gpu/timed/ms | ms | 7.17 | 10.49 | 7.41 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 200 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 795 | 1420 | 618 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 200 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 795 | 1420 | 618 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d/bins | count | 1294 | 1400 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3351 | 5154 | 3184 |
| phases/opaque3d/pipelines | count | 14 | 15 | 15 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1294 | 1400 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3351 | 5154 | 3184 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 15 |
| phases/shadow/batch_sets | count | 23 | 209 | 19 |
| phases/shadow/bins | count | 2194 | 4190 | 1137 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3504 | 7061 | 2434 |
| phases/shadow/pipelines | count | 101 | 238 | 19 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1032 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 171 | 173 | 173 |
| pipelines/waiting | count | 0 | 27 | 0 |
| preprocess/bind_groups | count | 155 | 569 | 149 |
| preprocess/bind_groups_created | count | 155 | 569 | 149 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.07 | 36.92 | 17.79 |
| process/mem_usage | GiB | 2.63 | 2.68 | 2.68 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 3473 | 3525 | 3525 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 434 | 530 | 398 |
| statics/materials_bound | count | 434 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.55 | 47.65 | 28.63 |
| system/mem_usage | % | 63.52 | 63.73 | 63.73 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 68 | 251 | 65 |
| views/all/bins | count | 4789 | 7036 | 4629 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.5 k | 19.4 k | 10.0 k |
| views/all/pipelines | count | 143 | 278 | 61 |
| views/all/pipelines_distinct | count | 59 | 69 | 57 |
| views/all/work_items | count | 11.0 k | 18.0 k | 10.3 k |
| views/cascade/batch_sets | count | 22 | 28 | 19 |
| views/cascade/bins | count | 1173 | 1524 | 1137 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2734 | 4307 | 2434 |
| views/cascade/pipelines | count | 22 | 28 | 19 |
| views/cascade/work_items | count | 2734 | 4307 | 2434 |
| views/lamp_face/batch_sets | count | 0 | 186 | 0 |
| views/lamp_face/bins | count | 1115 | 2840 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1267 | 3133 | 0 |
| views/lamp_face/pipelines | count | 76 | 212 | 0 |
| views/lamp_face/work_items | count | 0 | 2791 | 0 |
| views/main/batch_sets | count | 44 | 48 | 46 |
| views/main/bins | count | 2890 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8302 | 13.1 k | 7604 |
| views/main/pipelines | count | 42 | 44 | 42 |
| views/main/work_items | count | 8462 | 13.3 k | 7912 |
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
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.21 | 229 | 279.6 k |
| anim/events/count | count | 0.19 | 3 | 1121 |
| assets/Image/added | count | 0.06 | 35 | 357 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.71 | 160 | 4285 |
| assets/Mesh/removed | count | 0.81 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 512 |
| assets/StandardMaterial/modified | count | 3.58 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| ecs/entities_delta | count | 1.20 | 1327 | 7271 |
| physics/inserts | count | 0.53 | 152 | 3226 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 4 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 18 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 25 | 378 |
| stream/upload/bytes | bytes | 5594 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1227 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.4 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.51 | 511.0 | main/busy/ms 490, surface/advance_cpu_ms 0.72, stream/drive/ms 0.33, surface/cell_upload_ms 0.27, stream/tick/ms 0.11, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.82 | 307.0 | main/busy/ms 523, render/thread/ms 272, render/busy/ms 206, render/acquire/ms 63.46, render/graph/ms 4.89, render/present/ms 2.08, stream/drive/ms 0.95, surface/advance_cpu_ms 0.72, stream/tick/ms 0.49, bigspace/total/ms 0.49, terrain/update/ms 0.45, bigspace/high_precision/ms 0.37 |
| warm-up | 0.89 | 70.7 | render/thread/ms 289, render/busy/ms 222, render/acquire/ms 63.81, main/busy/ms 50.56, terrain/update/ms 19.24, render/graph/ms 9.84, render/present/ms 3.31, stream/drive/ms 1.02, stream/tick/ms 0.71, terrain/select/ms 0.63, bigspace/total/ms 0.50, bigspace/high_precision/ms 0.38 |
| warm-up | 1.15 | 107.0 | render/thread/ms 70.08, main/busy/ms 69.91, render/busy/ms 50.82, render/present/ms 18.97, render/graph/ms 3.66, stream/drive/ms 2.46, stream/tick/ms 1.85, terrain/update/ms 0.87, terrain/select/ms 0.73, render/acquire/ms 0.28, bigspace/total/ms 0.26, bigspace/high_precision/ms 0.10 |
| warm-up | 1.19 | 26.4 | render/thread/ms 33.87, render/busy/ms 32.68, main/busy/ms 18.51, stream/drive/ms 3.04, render/graph/ms 2.15, stream/tick/ms 1.64, render/present/ms 1.01, stream/spawn/ms 0.91, terrain/update/ms 0.72, terrain/select/ms 0.58, bigspace/total/ms 0.25, render/acquire/ms 0.18 |
| warm-up | 2.78 | 28.3 | main/busy/ms 41.08, render/thread/ms 23.53, render/busy/ms 22.20, render/graph/ms 5.88, stream/drive/ms 1.20, render/present/ms 1.12, physics/insert/ms 1.10, bigspace/total/ms 0.68, stream/tick/ms 0.60, terrain/update/ms 0.55, terrain/select/ms 0.46, anim/evaluate_cpu/ms 0.43 |
| measure | 11.44 | 30.9 | render/thread/ms 40.10, render/busy/ms 21.67, main/busy/ms 19.60, render/present/ms 18.25, render/graph/ms 12.25, bigspace/total/ms 0.83, far/select/ms 0.62, stream/drive/ms 0.57, terrain/update/ms 0.55, stream/tick/ms 0.49, terrain/select/ms 0.49, controller/feed/ms 0.48 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ab_main\balmora_walk_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 16%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **74%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.5 W · 72 °C (max 77) · VRAM 4867 MiB · throttled 100% of samples
